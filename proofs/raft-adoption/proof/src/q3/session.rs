use super::{
    encoding::{self, PromotionProof},
    machine::{self, Machine},
    recovery,
    storage::Q3Storage,
    voter::Voter,
};
use glade_raft_adoption_api::CommittedMachine;
use glade_raft_q3_api::{
    Action, Change, Checkpoint, CheckpointStore, Command, ConfigIntent, ConfigKey, ConfigReceipt,
    Control, Error, Instance, NodeCut, Receipt, ReplayResult, RequestId, Resource, State,
    StoreLifecycle, StoredEntry, View,
};
use raft::eraftpb::MessageType;
use raft::{Config, RawNode, SnapshotStatus, StateRole};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::Q3Session;

fn instance(node: u64) -> Instance {
    Instance {
        scope: 7,
        group: 70,
        node,
        application_profile: 2,
    }
}
fn voter(mut store: Box<dyn CheckpointStore>, mut state: State) -> Result<Voter, Error> {
    let machine = recovery::restore(&state, &instance(state.instance.node))?;
    if machine.applied() != state.image.applied
        || machine.configuration != state.image.configuration
    {
        let mut image = state.image.clone();
        image.applied = machine.applied();
        image.configuration = machine.configuration.clone();
        let next = store.publish(state.revision, image.clone())?;
        if next.instance != state.instance
            || next.revision
                != state
                    .revision
                    .checked_add(1)
                    .ok_or(Error::CapacityExhausted)?
            || next.image != image
        {
            return Err(Error::Quarantined);
        }
        state = next;
    }
    let config = Config {
        id: state.instance.node,
        election_tick: 10,
        heartbeat_tick: 1,
        applied: machine.applied(),
        ..Config::default()
    };
    let logger = slog::Logger::root(slog::Discard, slog::o!());
    let raft = RawNode::new(&config, Q3Storage::new(state.image.clone()), &logger)
        .map_err(|_| Error::Quarantined)?;
    Ok(Voter {
        raft,
        machine,
        store,
        state,
        failure: None,
    })
}
impl Q3Session {
    fn leader(&self) -> Result<u64, Error> {
        self.nodes
            .iter()
            .filter(|(_, node)| {
                node.failure.is_none()
                    && node.raft.raft.state == StateRole::Leader
                    && (node
                        .machine
                        .configuration
                        .voters
                        .contains(&node.state.instance.node)
                        || node
                            .machine
                            .configuration
                            .voters_outgoing
                            .contains(&node.state.instance.node))
            })
            .max_by_key(|(_, node)| node.raft.raft.term)
            .map(|(id, _)| *id)
            .ok_or(Error::NoQuorum)
    }
    fn serving(&self) -> Result<&Voter, Error> {
        let leader = self.leader()?;
        Ok(&self.nodes[&leader])
    }
    fn drain(&mut self) -> Result<(), Error> {
        for _ in 0..100_000 {
            let mut progress = false;
            for node in self.nodes.values_mut() {
                if node.failure.is_none() && node.raft.has_ready() {
                    match node.ready() {
                        Ok(messages) => {
                            self.messages.extend(messages);
                        }
                        Err(error) => {
                            node.failure = Some(error.clone());
                            return Err(error);
                        }
                    }
                    progress = true;
                }
            }
            if let Some(message) = self.messages.pop_front() {
                progress = true;
                let from = message.from;
                let to = message.to;
                let snapshot = message.get_msg_type() == MessageType::MsgSnapshot;
                let delivered = !self.disconnected.contains(&from)
                    && !self.disconnected.contains(&to)
                    && self.nodes.contains_key(&to);
                if delivered {
                    if snapshot {
                        let metadata = message.get_snapshot().get_metadata();
                        let cp = Checkpoint {
                            binding: glade_raft_q3_api::GroupIdentity {
                                scope: 7,
                                group: 70,
                                application_profile: 2,
                            },
                            index: metadata.index,
                            term: metadata.term,
                            configuration: recovery::configuration(metadata.get_conf_state(), 0),
                            application: message.get_snapshot().data.to_vec(),
                        };
                        let cp = recovery::checkpoint(cp)?;
                        let candidate = Machine::restore(&cp)?;
                        let receiver = &self.nodes[&to];
                        recovery::snapshot_agrees(&receiver.state, &receiver.machine, &candidate)?;
                    }
                    if matches!(
                        message.get_msg_type(),
                        MessageType::MsgRequestVote | MessageType::MsgRequestPreVote
                    ) {
                        let receiver = &self.nodes[&to];
                        let configuration = &receiver.machine.configuration;
                        let authorized = |id| {
                            configuration.voters.contains(&id)
                                || configuration.voters_outgoing.contains(&id)
                        };
                        // An authorized join alone does not turn an incomplete
                        // original-genesis receiver into a voter. This derives
                        // solely from validated local committed history and is
                        // reconstructed after reopen; routing cannot grant it.
                        if !authorized(to) || !authorized(from) {
                            continue;
                        }
                    }
                    let node = self.nodes.get_mut(&to).unwrap();
                    if node.failure.is_none() {
                        match node.raft.step(message) {
                            Ok(()) | Err(raft::Error::StepPeerNotFound) => {}
                            Err(_) => {
                                return Err(Error::Quarantined);
                            }
                        }
                    }
                }
                if snapshot && let Some(source) = self.nodes.get_mut(&from) {
                    source.raft.report_snapshot(
                        to,
                        if delivered {
                            SnapshotStatus::Finish
                        } else {
                            SnapshotStatus::Failure
                        },
                    );
                }
            }
            if !progress {
                return Ok(());
            }
        }
        Err(Error::Quarantined)
    }
    fn propose(&mut self, command: Command) -> Result<(), Error> {
        let leader = match self.leader() {
            Ok(node) => node,
            Err(Error::NoQuorum) => {
                return Ok(());
            }
            Err(error) => {
                return Err(error);
            }
        };
        let readiness = if let Action::Move {
            home,
            successor_applied,
        } = command.action
        {
            let source = &self.nodes[&leader];
            self.nodes
                .get(&home)
                .filter(|node| {
                    node.failure.is_none()
                        && !self.disconnected.contains(&home)
                        && node.machine.applied() == source.machine.applied()
                        && node.state.image.applied == source.machine.applied()
                        && node.machine.snapshot_bytes().ok()
                            == source.machine.snapshot_bytes().ok()
                        && successor_applied == Some(source.machine.applied())
                })
                .map(|_| crate::application::Readiness {
                    home,
                    frontier: source.machine.applied(),
                })
        } else {
            None
        };
        let data = crate::codec::encode(command, readiness);
        let _unknown = self
            .nodes
            .get_mut(&leader)
            .unwrap()
            .raft
            .propose(Vec::new(), data);
        Ok(())
    }
    fn catch_up(&mut self, node: u64) -> Result<(), Error> {
        if self.disconnected.contains(&node) {
            return Err(Error::IncompleteLearner);
        }
        let leader = self.leader()?;
        let config = self.nodes[&leader].machine.configuration.clone();
        if !config.voters.contains(&node)
            && !config.voters_outgoing.contains(&node)
            && !config.learners.contains(&node)
        {
            return Err(Error::Unauthorized);
        }
        if !self.nodes.contains_key(&node) {
            // Existing-group authorization is proven by the retained accepted join,
            // never by a missing path. Actual original log/snapshot transfer follows.
            if !self.nodes[&leader]
                .machine
                .configurations
                .values()
                .any(|receipt| {
                    receipt.outcome == glade_raft_q3_api::ConfigOutcome::Accepted
                        && receipt.intent.change == Change::AddLearner { node }
                })
            {
                return Err(Error::Unauthorized);
            }
            let initial = recovery::initial();
            let mut store = self.lifecycle.create(instance(node), initial)?;
            let state = store.load()?;
            self.nodes.insert(node, voter(store, state)?);
        }
        let source_compacted = self.nodes[&leader].state.image.checkpoint.is_some();
        if source_compacted
            && self.nodes[&node].machine.applied() < self.nodes[&leader].machine.applied()
        {
            let admission = self.nodes[&leader]
                .machine
                .configurations
                .values()
                .filter(|receipt| {
                    receipt.intent.change == Change::AddLearner { node }
                        && receipt.outcome == glade_raft_q3_api::ConfigOutcome::Accepted
                })
                .map(|receipt| receipt.index)
                .max()
                .ok_or(Error::IncompleteLearner)?;
            let cut = admission.max(
                self.nodes[&leader]
                    .state
                    .image
                    .checkpoint
                    .as_ref()
                    .map_or(0, |cp| cp.index),
            );
            let mut prefix = Machine::new();
            for (_, (entry, _)) in self.nodes[&leader].machine.history.range(..=cut) {
                prefix.apply(entry.clone())?;
            }
            let cp = prefix.checkpoint()?;
            self.compact_node(leader, cp)?;
        }
        self.nodes.get_mut(&leader).unwrap().raft.ping();
        self.drain()?;
        let source = &self.nodes[&leader];
        let target = &self.nodes[&node];
        if target.failure.is_some()
            || target.machine.applied() != source.machine.applied()
            || target.state.image.applied != source.machine.applied()
            || target.machine.snapshot_bytes()? != source.machine.snapshot_bytes()?
        {
            return Err(Error::IncompleteLearner);
        }
        self.observed.insert(node);
        Ok(())
    }
    fn compact_node(&mut self, id: u64, checkpoint: Checkpoint) -> Result<(), Error> {
        let node = self.nodes.get_mut(&id).ok_or(Error::Missing)?;
        let restored = Machine::restore(&checkpoint)?;
        for (index, value) in &restored.history {
            if node.machine.history.get(index) != Some(value) {
                return Err(Error::Quarantined);
            }
        }
        if checkpoint.index > node.machine.applied() {
            return Err(Error::Quarantined);
        }
        let mut image = node.state.image.clone();
        image.suffix = node
            .machine
            .history
            .range((checkpoint.index + 1)..)
            .map(|(_, (entry, _))| entry.clone())
            .chain(
                image
                    .suffix
                    .iter()
                    .filter(|entry| entry.index > node.machine.applied())
                    .cloned(),
            )
            .collect();
        image.checkpoint = Some(checkpoint);
        node.publish(image)?;
        *node.raft.mut_store() = Q3Storage::new(node.state.image.clone());
        Ok(())
    }
}

mod port;

mod admission_tests;

mod light_ready_tests;

mod recovery_tests;

mod lifecycle;
