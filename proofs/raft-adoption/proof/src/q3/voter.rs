//! Ready/LightReady barrier: coherent image sync before apply/messages/receipts.
use super::{
    machine::{Machine, parse},
    storage::Q3Storage,
};
use glade_raft_q3_api::{
    Checkpoint, CheckpointStore, Error, GroupIdentity, Image, ReplayResult, State, StoredEntry,
};
use protobuf::Message as ProtobufMessage;
use raft::eraftpb::{Entry, HardState, Message};
use raft::{RawNode, Ready};

pub(super) struct Voter {
    pub(super) raft: RawNode<Q3Storage>,
    pub(super) machine: Machine,
    pub(super) store: Box<dyn CheckpointStore>,
    pub(super) state: State,
    pub(super) failure: Option<Error>,
}
impl Voter {
    pub(super) fn publish(&mut self, image: Image) -> Result<(), Error> {
        if image.term == u64::MAX {
            return Err(Error::CapacityExhausted);
        }
        if image == self.state.image {
            return Ok(());
        }
        let published = self.store.publish(self.state.revision, image.clone());
        let validated = published.and_then(|next| {
            if next.instance != self.state.instance
                || next.revision
                    != self
                        .state
                        .revision
                        .checked_add(1)
                        .ok_or(Error::CapacityExhausted)?
                || next.image != image
            {
                return Err(Error::Quarantined);
            }
            Ok(next)
        });
        let next = match validated {
            Ok(next) => next,
            Err(error) => {
                // Publication errors and malformed success both leave this
                // instance stopped until physical reopen validates actual bytes.
                self.failure = Some(error.clone());
                return Err(error);
            }
        };
        self.state = next;
        Ok(())
    }
    pub(super) fn persist_ready(
        &mut self,
        entries: &[Entry],
        hard: Option<&HardState>,
        snapshot: &raft::eraftpb::Snapshot,
    ) -> Result<(), Error> {
        let mut image = self.state.image.clone();
        let candidate = if snapshot.get_metadata().index > 0 {
            let metadata = snapshot.get_metadata();
            let cp = Checkpoint {
                binding: GroupIdentity {
                    scope: 7,
                    group: 70,
                    application_profile: 2,
                },
                index: metadata.index,
                term: metadata.term,
                configuration: super::recovery::configuration(metadata.get_conf_state(), 0),
                application: snapshot.data.to_vec(),
            };
            // The persisted configuration version is recovered from full original history.
            let cp = super::recovery::checkpoint(cp)?;
            let candidate = Machine::restore(&cp)?;
            super::recovery::snapshot_agrees(&self.state, &self.machine, &candidate)?;
            image.applied = cp.index;
            image.commit = image.commit.max(cp.index);
            image.configuration = cp.configuration.clone();
            image.checkpoint = Some(cp);
            image.suffix.clear();
            Some(candidate)
        } else {
            None
        };
        if let Some(first) = entries.first() {
            let cut = image.checkpoint.as_ref().map_or(0, |cp| cp.index);
            let offset = usize::try_from(
                first
                    .index
                    .checked_sub(cut)
                    .and_then(|n| n.checked_sub(1))
                    .ok_or(Error::Quarantined)?,
            )
            .map_err(|_| Error::CapacityExhausted)?;
            if offset > image.suffix.len() {
                return Err(Error::Quarantined);
            }
            image.suffix.truncate(offset);
            for entry in entries {
                let stored = StoredEntry {
                    index: entry.index,
                    term: entry.term,
                    bytes: entry.write_to_bytes().map_err(|_| Error::Quarantined)?,
                };
                parse(&stored)?;
                image.suffix.push(stored);
            }
        }
        if let Some(hard) = hard {
            image.term = hard.term;
            image.vote = hard.vote;
            image.commit = hard.commit;
        }
        self.publish(image)?;
        if let Some(candidate) = candidate {
            self.machine = candidate;
        }
        *self.raft.mut_store() = Q3Storage::new(self.state.image.clone());
        Ok(())
    }
    fn apply(&mut self, entries: Vec<Entry>) -> Result<(), Error> {
        for entry in entries {
            if entry.index <= self.machine.applied() {
                continue;
            }
            let stored = StoredEntry {
                index: entry.index,
                term: entry.term,
                bytes: entry.write_to_bytes().map_err(|_| Error::Quarantined)?,
            };
            let (_, change) = self.machine.apply(stored)?;
            if let Some(change) = change {
                let mut state = self
                    .raft
                    .apply_conf_change(&change)
                    .map_err(|_| Error::Quarantined)?;
                // raft-rs tracker output is unsorted; normalize only carrier
                // output, never hostile incoming ConfState declarations.
                for nodes in [
                    &mut state.voters,
                    &mut state.learners,
                    &mut state.voters_outgoing,
                    &mut state.learners_next,
                ] {
                    nodes.sort_unstable();
                }
                if state != super::storage::conf(&self.machine.configuration) {
                    return Err(Error::Quarantined);
                }
            }
            let mut image = self.state.image.clone();
            image.applied = self.machine.applied();
            image.configuration = self.machine.configuration.clone();
            self.publish(image)?;
            *self.raft.mut_store() = Q3Storage::new(self.state.image.clone());
        }
        Ok(())
    }
    pub(super) fn finish(&mut self, mut ready: Ready) -> Result<Vec<Message>, Error> {
        if !ready.read_states().is_empty() {
            return Err(Error::Quarantined);
        }
        let entries = ready.take_entries();
        self.persist_ready(&entries, ready.hs(), ready.snapshot())?;
        let mut messages = ready.take_messages();
        messages.extend(ready.take_persisted_messages());
        let committed = ready.take_committed_entries();
        let mut light = self.raft.advance_append(ready);
        if let Some(commit) = light.commit_index() {
            let mut image = self.state.image.clone();
            image.commit = commit;
            self.publish(image)?;
            *self.raft.mut_store() = Q3Storage::new(self.state.image.clone());
        }
        self.apply(committed)?;
        self.apply(light.take_committed_entries())?;
        messages.extend(light.take_messages());
        self.raft.advance_apply_to(self.machine.applied());
        Ok(messages)
    }
    pub(super) fn ready(&mut self) -> Result<Vec<Message>, Error> {
        let mut messages = Vec::new();
        while self.raft.has_ready() {
            let ready = self.raft.ready();
            messages.extend(self.finish(ready)?);
        }
        Ok(messages)
    }
    pub(super) fn replay(&self, entry: StoredEntry) -> Result<ReplayResult, Error> {
        let (original, result) = self
            .machine
            .history
            .get(&entry.index)
            .ok_or(Error::Missing)?;
        if *original != entry {
            return Err(Error::ConflictingReplay { index: entry.index });
        }
        Ok(result.clone())
    }
}

mod snapshot_tests;

mod recipient_tests;
