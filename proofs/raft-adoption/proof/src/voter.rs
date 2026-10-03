use glade_raft_durability_api::{DurableImage, DurableStore, StoreError, StoredEntry, StoredState};
use protobuf::Message as ProtobufMessage;
use raft::eraftpb::{Entry, EntryType, HardState, Message};
use raft::storage::MemStorage;
use raft::{RawNode, Ready};

use crate::application::Application;
use crate::codec;

pub(crate) struct Voter {
    pub(crate) raft: RawNode<MemStorage>,
    pub(crate) application: Application,
    pub(crate) persistence: Option<(Box<dyn DurableStore>, StoredState)>,
    pub(crate) failure: Option<StoreError>,
}

impl Voter {
    fn apply(&mut self, entries: Vec<Entry>) {
        for entry in entries {
            assert_eq!(
                entry.get_entry_type(),
                EntryType::EntryNormal,
                "configuration changes are outside Q1a"
            );
            let (command, readiness) = if entry.data.is_empty() {
                (None, None)
            } else {
                let (command, readiness) =
                    codec::decode(&entry.data).expect("invalid committed fixture encoding");
                (Some(command), readiness)
            };
            self.application
                .apply_committed(entry.index, command, readiness)
                .expect("committed application ordering violation");
        }
    }

    fn persist(&mut self, image: DurableImage) -> Result<(), StoreError> {
        // Reserve the terminal term consistently with startup admission.
        // This also fences higher terms learned from incoming peer messages.
        if image.term == u64::MAX {
            return Err(StoreError::CapacityExhausted);
        }
        if let Some((store, previous)) = &mut self.persistence {
            if image == previous.image {
                return Ok(());
            }
            let next = store.persist(previous.revision, image.clone())?;
            if next.binding != previous.binding
                || next.revision
                    != previous
                        .revision
                        .checked_add(1)
                        .ok_or(StoreError::CapacityExhausted)?
                || next.image != image
            {
                return Err(StoreError::Quarantined);
            }
            *previous = next;
        }
        Ok(())
    }

    fn image(&self) -> DurableImage {
        self.persistence
            .as_ref()
            .map(|(_, state)| state.image.clone())
            .unwrap_or_default()
    }

    pub(crate) fn persist_ready(
        &mut self,
        entries: &[Entry],
        hard_state: Option<&HardState>,
    ) -> Result<(), StoreError> {
        let mut image = self.image();
        if self.persistence.is_some() {
            if let Some(first) = entries.first() {
                let offset =
                    usize::try_from(first.index.checked_sub(1).ok_or(StoreError::InvalidImage)?)
                        .map_err(|_| StoreError::CapacityExhausted)?;
                if offset > image.entries.len() {
                    return Err(StoreError::InvalidImage);
                }
                image.entries.truncate(offset);
                for entry in entries {
                    image.entries.push(StoredEntry {
                        index: entry.index,
                        term: entry.term,
                        bytes: entry
                            .write_to_bytes()
                            .map_err(|_| StoreError::InvalidImage)?,
                    });
                }
            }
            if let Some(hard_state) = hard_state {
                image.term = hard_state.term;
                image.vote = hard_state.vote;
                image.commit = hard_state.commit;
            }
            self.persist(image)?;
        }
        self.raft
            .mut_store()
            .wl()
            .append(entries)
            .expect("memory mirror append failed");
        if let Some(hard_state) = hard_state {
            self.raft.mut_store().wl().set_hardstate(hard_state.clone());
        }
        Ok(())
    }

    pub(crate) fn finish_ready(&mut self, mut ready: Ready) -> Result<Vec<Message>, StoreError> {
        let mut outgoing = Vec::new();
        assert!(ready.snapshot().is_empty(), "snapshots are outside Q2");
        assert!(
            ready.read_states().is_empty(),
            "read barriers are outside Q2"
        );
        let entries = ready.take_entries();
        self.persist_ready(&entries, ready.hs())?;
        outgoing.extend(ready.take_messages());
        outgoing.extend(ready.take_persisted_messages());
        let committed = ready.take_committed_entries();
        // Append advancement confirms persistence only. Postpone application
        // and message release until the resulting LightReady is also durable.
        let mut light = self.raft.advance_append(ready);
        if let Some(commit) = light.commit_index() {
            if self.persistence.is_some() {
                let mut image = self.image();
                image.commit = commit;
                self.persist(image)?;
            }
            // Preserve current term/vote for this commit-only update.
            self.raft.mut_store().wl().mut_hard_state().commit = commit;
        }
        self.apply(committed);
        self.apply(light.take_committed_entries());
        outgoing.extend(light.take_messages());
        self.raft.advance_apply_to(self.application.applied());
        Ok(outgoing)
    }

    pub(crate) fn ready(&mut self) -> Result<Vec<Message>, StoreError> {
        let mut outgoing = Vec::new();
        while self.raft.has_ready() {
            let ready = self.raft.ready();
            outgoing.extend(self.finish_ready(ready)?);
        }
        Ok(outgoing)
    }
}
