//! Injected reopen, whole-group validation and explicit manual campaign.
use super::*;
impl Q3Session {
    pub fn recover(
        mut stores: BTreeMap<u64, Box<dyn CheckpointStore>>,
        lifecycle: Box<dyn StoreLifecycle>,
        authorities: Vec<u64>,
    ) -> Result<Self, Error> {
        if authorities != [1, 2]
            || stores.is_empty()
            || stores.keys().any(|node| !(1..=4).contains(node))
        {
            return Err(Error::WrongBinding);
        }
        let mut loaded = Vec::new();
        for (id, mut store) in std::mem::take(&mut stores) {
            let state = store.load()?;
            let machine = recovery::restore(&state, &instance(id))?;
            loaded.push((id, store, state, machine));
        }
        if loaded.iter().any(|(id, _, _, _)| *id == 4)
            && !loaded.iter().any(|(id, _, _, machine)| {
                *id != 4
                    && machine.configurations.values().any(|receipt| {
                        receipt.outcome == glade_raft_q3_api::ConfigOutcome::Accepted
                            && receipt.intent.change == Change::AddLearner { node: 4 }
                    })
            })
        {
            return Err(Error::Unauthorized);
        }
        for left in &loaded {
            for right in &loaded {
                for (index, (entry, result)) in &left.3.history {
                    if let Some((other, outcome)) = right.3.history.get(index)
                        && (entry != other || result != outcome)
                    {
                        return Err(Error::Quarantined);
                    }
                }
            }
        }
        let newest = loaded
            .iter()
            .max_by_key(|(_, _, _, machine)| machine.configuration.index)
            .ok_or(Error::Missing)?;
        for required in newest
            .3
            .configuration
            .voters
            .iter()
            .chain(&newest.3.configuration.voters_outgoing)
        {
            if !loaded.iter().any(|(id, _, _, _)| id == required) {
                return Err(Error::Missing);
            }
        }
        let mut nodes = BTreeMap::new();
        for (id, store, state, _) in loaded {
            nodes.insert(id, voter(store, state)?);
        }
        let mut session = Self {
            nodes,
            lifecycle,
            messages: VecDeque::new(),
            disconnected: BTreeSet::new(),
            observed: BTreeSet::new(),
            authorities,
            lose_reply: false,
            queued: None,
        };
        session.campaign()?;
        session.drain()?;
        Ok(session)
    }
    pub(super) fn campaign(&mut self) -> Result<(), Error> {
        let newest = self
            .nodes
            .values()
            .max_by_key(|node| node.machine.configuration.index)
            .ok_or(Error::Missing)?;
        let candidate = newest
            .machine
            .configuration
            .voters
            .iter()
            .copied()
            .find(|id| {
                self.nodes.get(id).is_some_and(|node| {
                    node.failure.is_none()
                        && (node.machine.configuration.voters.contains(id)
                            || node.machine.configuration.voters_outgoing.contains(id))
                        && node.state.image.applied == node.machine.applied()
                })
            })
            .ok_or(Error::NoQuorum)?;
        let node = self.nodes.get_mut(&candidate).ok_or(Error::Missing)?;
        if node.state.image.term >= u64::MAX - 1 {
            return Err(Error::CapacityExhausted);
        }
        node.raft.campaign().map_err(|_| Error::Quarantined)
    }
    pub(super) fn restart(&mut self) -> Result<(), Error> {
        let saved: Vec<_> = self
            .nodes
            .values()
            .map(|node| (node.state.instance.clone(), node.state.revision))
            .collect();
        self.nodes.clear();
        self.messages.clear();
        let mut stores = BTreeMap::new();
        for (instance, floor) in saved {
            let store = self.lifecycle.open(instance.clone(), Some(floor))?;
            stores.insert(instance.node, store);
        }
        let mut loaded = Vec::new();
        for (id, mut store) in stores {
            let state = store.load()?;
            let machine = recovery::restore(&state, &instance(id))?;
            loaded.push((id, store, state, machine));
        }
        if loaded.iter().any(|(id, _, _, _)| *id == 4)
            && !loaded.iter().any(|(id, _, _, machine)| {
                *id != 4
                    && machine.configurations.values().any(|receipt| {
                        receipt.outcome == glade_raft_q3_api::ConfigOutcome::Accepted
                            && receipt.intent.change == Change::AddLearner { node: 4 }
                    })
            })
        {
            return Err(Error::Unauthorized);
        }
        for left in &loaded {
            for right in &loaded {
                for (index, value) in &left.3.history {
                    if let Some(other) = right.3.history.get(index)
                        && value != other
                    {
                        return Err(Error::Quarantined);
                    }
                }
            }
        }
        for (id, store, state, _) in loaded {
            self.nodes.insert(id, voter(store, state)?);
        }
        self.observed.clear();
        self.campaign()?;
        self.drain()
    }
}
