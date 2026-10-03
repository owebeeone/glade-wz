use super::*;
impl glade_raft_q3_api::QualificationSession for crate::q3::Q3Session {
    fn control(&mut self, input: Control) -> Result<(), Error> {
        match input {
            Control::Drain => self.drain(),
            Control::Disconnect { nodes } => {
                if nodes.iter().any(|node| !(1..=4).contains(node)) {
                    return Err(Error::WrongBinding);
                }
                self.disconnected.extend(nodes);
                Ok(())
            }
            Control::Reconnect => {
                self.disconnected.clear();
                if let Ok(leader) = self.leader() {
                    self.nodes.get_mut(&leader).unwrap().raft.ping();
                }
                self.drain()
            }
            Control::CatchUp { node } => self.catch_up(node),
            Control::Restart => self.restart(),
            Control::LoseNextConfigurationReply => {
                self.lose_reply = true;
                Ok(())
            }
            Control::QueueApplicationBeforeNextConfiguration { command } => {
                if self.queued.replace(command).is_some() {
                    return Err(Error::InvalidImage);
                }
                Ok(())
            }
        }
    }
    fn view(&self) -> Result<View, Error> {
        let source = self.serving()?;
        Ok(View {
            configuration: source.machine.configuration.clone(),
            committed: source.state.image.commit,
            nodes: self
                .nodes
                .iter()
                .map(|(id, node)| NodeCut {
                    node: *id,
                    snapshot_index: node.state.image.checkpoint.as_ref().map(|cp| cp.index),
                    durable: node.state.image.commit,
                    applied: node.machine.applied(),
                })
                .collect(),
        })
    }
    fn configure(&mut self, intent: ConfigIntent) -> Result<Option<ConfigReceipt>, Error> {
        if intent.key.scope != 7 || intent.key.group != 70 {
            return Err(Error::WrongBinding);
        }
        if !self.authorities.contains(&intent.key.principal) {
            return Err(Error::Unauthorized);
        }
        let leader = match self.leader() {
            Ok(node) => node,
            Err(Error::NoQuorum) => {
                return Ok(None);
            }
            Err(error) => {
                return Err(error);
            }
        };
        if let Some(original) = self.nodes[&leader].machine.configurations.get(&intent.key) {
            if original.intent != intent {
                return Err(Error::RetryConflict);
            }
            return Ok(Some(original.clone()));
        }
        let node = &self.nodes[&leader];
        for stored in node
            .state
            .image
            .suffix
            .iter()
            .filter(|entry| entry.index > node.machine.applied())
        {
            let entry = machine::parse(stored)?;
            if entry.get_entry_type() == raft::eraftpb::EntryType::EntryConfChangeV2 {
                let (pending, _) = encoding::parse_context(&entry.context)?;
                if pending.key == intent.key {
                    if pending != intent {
                        return Err(Error::RetryConflict);
                    }
                    // This exact intent is already admitted. Local eligibility
                    // changes describe a new attempt, not terminal noncommit.
                    return Ok(None);
                }
            }
        }
        node.machine.admission(&intent)?;
        if node.raft.raft.pending_conf_index > node.machine.applied() {
            return Ok(None);
        }
        let mut proofs = Vec::new();
        if let Change::EnterJoint { voters } = &intent.change {
            for id in voters
                .iter()
                .filter(|id| !node.machine.configuration.voters.contains(id))
            {
                let target = self.nodes.get(id).ok_or(Error::IncompleteLearner)?;
                if !self.observed.contains(id)
                    || self.disconnected.contains(id)
                    || target.failure.is_some()
                    || target.machine.applied() != node.machine.applied()
                    || target.state.image.applied != node.machine.applied()
                    || target.machine.snapshot_bytes()? != node.machine.snapshot_bytes()?
                {
                    return Err(Error::IncompleteLearner);
                }
                proofs.push(PromotionProof {
                    node: *id,
                    index: target.machine.applied(),
                    term: target
                        .machine
                        .history
                        .last_key_value()
                        .map_or(0, |(_, (entry, _))| entry.term),
                    state: target.machine.snapshot_bytes()?,
                });
            }
        }
        let change = machine::change(&intent.change, &node.machine.configuration);
        let context = encoding::context(&intent, &proofs);
        if context.len() > encoding::LIMIT {
            return Err(Error::CapacityExhausted);
        }
        if let Some(command) = self.queued.take() {
            self.propose(command)?;
        }
        let _unknown = self
            .nodes
            .get_mut(&leader)
            .unwrap()
            .raft
            .propose_conf_change(context, change);
        self.drain()?;
        let receipt = self.nodes[&leader]
            .machine
            .configurations
            .get(&intent.key)
            .cloned();
        if self.lose_reply {
            self.lose_reply = false;
            Ok(None)
        } else {
            Ok(receipt)
        }
    }
    fn configuration_outcome(&self, key: ConfigKey) -> Result<Option<ConfigReceipt>, Error> {
        if key.scope != 7 || key.group != 70 {
            return Err(Error::WrongBinding);
        }
        if !self.authorities.contains(&key.principal) {
            return Err(Error::Unauthorized);
        }
        Ok(self.serving()?.machine.configurations.get(&key).cloned())
    }
    fn submit(&mut self, command: Command) -> Result<Option<Receipt>, Error> {
        if let Ok(source) = self.serving() {
            if !source.machine.application.may_disclose(command.request) {
                return Err(Error::Unauthorized);
            }
            if let Some(original) = source.machine.application.original_command(command.request) {
                if original != command {
                    return Err(Error::RetryConflict);
                }
                return Ok(source.machine.application.lookup(command.request));
            }
        }
        self.propose(command)?;
        self.drain()?;
        self.outcome(command.request)
    }
    fn outcome(&self, request: RequestId) -> Result<Option<Receipt>, Error> {
        let source = match self.serving() {
            Ok(node) => node,
            Err(Error::NoQuorum) => {
                return Ok(None);
            }
            Err(error) => {
                return Err(error);
            }
        };
        if !source.machine.application.may_disclose(request) {
            return Err(Error::Unauthorized);
        }
        Ok(source.machine.application.lookup(request))
    }
    fn resource(&self, id: u64) -> Result<Option<Resource>, Error> {
        Ok(self.serving()?.machine.application.resource(id))
    }
    fn checkpoint(&self) -> Result<Checkpoint, Error> {
        self.serving()?.machine.checkpoint()
    }
    fn install(&mut self, checkpoint: Checkpoint) -> Result<(), Error> {
        let candidate = Machine::restore(&checkpoint)?;
        // Self-consistency does not prove agreement with acknowledged history.
        // Validate every common committed prefix before filtering recipients.
        for node in self.nodes.values() {
            recovery::snapshot_agrees(&node.state, &node.machine, &candidate)?;
        }
        let targets: Vec<_> = self
            .nodes
            .iter()
            .filter(|(_, node)| {
                node.failure.is_none() && node.machine.applied() >= checkpoint.index
            })
            .map(|(id, _)| *id)
            .collect();
        if targets.is_empty() {
            // This port only compacts a locally applied prefix; an ahead cut
            // needs actual protocol restoration, not a successful no-op.
            return Err(Error::InvalidImage);
        }
        for id in targets {
            self.compact_node(id, checkpoint.clone())?;
        }
        Ok(())
    }
    fn applied_entry(&self, index: u64) -> Result<StoredEntry, Error> {
        self.serving()?
            .machine
            .history
            .get(&index)
            .map(|(entry, _)| entry.clone())
            .ok_or(Error::Missing)
    }
    fn replay(&mut self, entry: StoredEntry) -> Result<ReplayResult, Error> {
        self.serving()?.replay(entry)
    }
}
