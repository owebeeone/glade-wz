//! Validate whole retained semantic prefix before any serving carrier.
use super::encoding::Reader;
use super::machine::{Machine, parse};
use glade_raft_q3_api::{Checkpoint, Configuration, Error, Image, Instance, State};
use raft::eraftpb::ConfState;

pub(super) fn configuration(cs: &ConfState, index: u64) -> Configuration {
    Configuration {
        voters: cs.voters.clone(),
        learners: cs.learners.clone(),
        voters_outgoing: cs.voters_outgoing.clone(),
        learners_next: cs.learners_next.clone(),
        auto_leave: cs.auto_leave,
        index,
    }
}
fn canonical(nodes: &[u64]) -> bool {
    nodes.len() <= 4
        && nodes.iter().all(|n| (1..=4).contains(n))
        && nodes.windows(2).all(|p| p[0] < p[1])
}
pub(super) fn configuration_valid(cs: &Configuration) -> bool {
    !cs.voters.is_empty()
        && [
            &cs.voters,
            &cs.learners,
            &cs.voters_outgoing,
            &cs.learners_next,
        ]
        .into_iter()
        .all(|n| canonical(n))
        && cs.learners_next.is_empty()
        && !cs.auto_leave
        && cs
            .learners
            .iter()
            .all(|n| !cs.voters.contains(n) && !cs.voters_outgoing.contains(n))
}
// Snapshot metadata lacks the private successful configuration index. Recover it
// from complete canonical history, then validate exact carrier lists against it.
pub(super) fn checkpoint(mut cp: Checkpoint) -> Result<Checkpoint, Error> {
    if !configuration_valid(&cp.configuration) {
        return Err(Error::Quarantined);
    }
    let mut input = Reader(&cp.application);
    if input.bytes()? != b"GQ3APP02" || [input.word()?, input.word()?, input.word()?] != [7, 70, 2]
    {
        return Err(Error::Quarantined);
    }
    let count = input.word()?;
    if count != cp.index || count > input.0.len() as u64 / 24 {
        return Err(Error::Quarantined);
    }
    let mut machine = Machine::new();
    for _ in 0..count {
        let stored = glade_raft_q3_api::StoredEntry {
            index: input.word()?,
            term: input.word()?,
            bytes: input.bytes()?,
        };
        let (result, _) = machine.apply(stored)?;
        let mut expected = Vec::new();
        super::machine::result_bytes(&mut expected, &result);
        if input.take(expected.len())? != expected {
            return Err(Error::Quarantined);
        }
    }
    let mut carrier = cp.configuration.clone();
    carrier.index = machine.configuration.index;
    if carrier != machine.configuration {
        return Err(Error::Quarantined);
    }
    cp.configuration = carrier;
    Machine::restore(&cp)?;
    Ok(cp)
}
pub(super) fn restore(state: &State, expected: &Instance) -> Result<Machine, Error> {
    if state.instance != *expected
        || expected.scope != 7
        || expected.group != 70
        || expected.application_profile != 2
        || !(1..=4).contains(&expected.node)
    {
        return Err(Error::WrongBinding);
    }
    let image = &state.image;
    if image.term == u64::MAX {
        return Err(Error::CapacityExhausted);
    }
    if !configuration_valid(&image.configuration)
        || image.vote > 4
        || image.applied > image.commit
        || image.configuration.index > image.applied
    {
        return Err(Error::Quarantined);
    }
    let mut machine = if let Some(cp) = &image.checkpoint {
        Machine::restore(cp)?
    } else {
        Machine::new()
    };
    let cut = machine.applied();
    if cut > image.applied
        || image.commit
            > cut
                .checked_add(image.suffix.len() as u64)
                .ok_or(Error::CapacityExhausted)?
    {
        return Err(Error::Quarantined);
    }
    if image.applied == cut && machine.configuration != image.configuration {
        return Err(Error::Quarantined);
    }
    let mut previous_term = image.checkpoint.as_ref().map_or(0, |cp| cp.term);
    for (offset, entry) in image.suffix.iter().enumerate() {
        if entry.index
            != cut
                .checked_add(offset as u64 + 1)
                .ok_or(Error::CapacityExhausted)?
            || entry.term < previous_term
            || entry.term > image.term
        {
            return Err(Error::Quarantined);
        }
        parse(entry)?;
        previous_term = entry.term;
        if entry.index <= image.commit {
            machine.apply(entry.clone())?;
            if entry.index == image.applied && machine.configuration != image.configuration {
                return Err(Error::Quarantined);
            }
        }
    }
    if image.term == 0 && (image.vote != 0 || machine.applied() != 0 || !image.suffix.is_empty()) {
        return Err(Error::Quarantined);
    }
    Ok(machine)
}

pub(super) fn snapshot_agrees(
    state: &State,
    live: &Machine,
    candidate: &Machine,
) -> Result<(), Error> {
    for (index, original) in &live.history {
        if let Some(incoming) = candidate.history.get(index)
            && original != incoming
        {
            return Err(Error::Quarantined);
        }
    }
    for entry in state
        .image
        .suffix
        .iter()
        .filter(|entry| entry.index <= state.image.commit && entry.index <= candidate.applied())
    {
        if candidate.history.get(&entry.index).map(|(entry, _)| entry) != Some(entry) {
            return Err(Error::Quarantined);
        }
    }
    Ok(())
}
pub(super) fn initial() -> Image {
    glade_raft_q3_api::Image {
        term: 0,
        vote: 0,
        commit: 0,
        applied: 0,
        configuration: glade_raft_q3_api::conformance::stable(),
        checkpoint: None,
        suffix: Vec::new(),
    }
}
