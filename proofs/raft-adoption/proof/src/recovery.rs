//! Pre-start validation/replay. No I/O or authority comes from a familiar label.
use glade_raft_durability_api::{Binding, StoreError, StoredState};
use protobuf::Message as ProtobufMessage;
use raft::eraftpb::{Entry, EntryType};

use crate::application::Application;
use crate::codec;

pub(crate) fn entries(state: &StoredState, expected: &Binding) -> Result<Vec<Entry>, StoreError> {
    if &state.binding != expected {
        return Err(StoreError::BindingMismatch);
    }
    let image = &state.image;
    if image.term == u64::MAX {
        return Err(StoreError::CapacityExhausted);
    }
    if image.commit > image.entries.len() as u64
        || (image.vote != 0 && !expected.voters.contains(&image.vote))
        || (image.term == 0 && (image.vote != 0 || !image.entries.is_empty()))
    {
        return Err(StoreError::Quarantined);
    }
    let mut last_term = 0;
    let mut decoded = Vec::new();
    for (offset, stored) in image.entries.iter().enumerate() {
        if stored.index != offset as u64 + 1
            || stored.term == 0
            || stored.term < last_term
            || stored.term > image.term
        {
            return Err(StoreError::Quarantined);
        }
        let entry = Entry::parse_from_bytes(&stored.bytes).map_err(|_| StoreError::Quarantined)?;
        if entry.index != stored.index
            || entry.term != stored.term
            || entry.get_entry_type() != EntryType::EntryNormal
            || !entry.context.is_empty()
            || entry.sync_log
            || entry.get_unknown_fields().iter().next().is_some()
        {
            return Err(StoreError::Quarantined);
        }
        if !entry.data.is_empty() {
            codec::decode(&entry.data).ok_or(StoreError::Quarantined)?;
        }
        last_term = entry.term;
        decoded.push(entry);
    }
    Ok(decoded)
}

pub(crate) fn application(
    voters: &[u64],
    commit: u64,
    entries: &[Entry],
) -> Result<Application, StoreError> {
    let mut application = Application::with_voters(voters);
    for entry in entries.iter().take(commit as usize) {
        let (command, readiness) = if entry.data.is_empty() {
            (None, None)
        } else {
            let (command, readiness) = codec::decode(&entry.data).ok_or(StoreError::Quarantined)?;
            (Some(command), readiness)
        };
        application
            .apply_committed(entry.index, command, readiness)
            .map_err(|_| StoreError::Quarantined)?;
    }
    if application.applied() != commit {
        return Err(StoreError::Quarantined);
    }
    Ok(application)
}
