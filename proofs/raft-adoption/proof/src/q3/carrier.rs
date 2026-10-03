//! Exact private ConfChangeV2 translation and original Entry grammar.
use super::encoding;
use glade_raft_q3_api::{Change, Configuration, Error, StoredEntry};
use protobuf::Message as ProtobufMessage;
use raft::eraftpb::{
    ConfChangeSingle, ConfChangeTransition, ConfChangeType, ConfChangeV2, Entry, EntryType,
};

pub(super) fn change(value: &Change, current: &Configuration) -> ConfChangeV2 {
    let mut change = ConfChangeV2::default();
    let mut add = |node_id, change_type| {
        change.mut_changes().push(ConfChangeSingle {
            node_id,
            change_type,
            ..ConfChangeSingle::default()
        });
    };
    match value {
        Change::AddLearner { node } => {
            add(*node, ConfChangeType::AddLearnerNode);
        }
        Change::EnterJoint { voters } => {
            for node in current.voters.iter().filter(|node| !voters.contains(node)) {
                add(*node, ConfChangeType::RemoveNode);
            }
            for node in voters.iter().filter(|node| !current.voters.contains(node)) {
                add(*node, ConfChangeType::AddNode);
            }
            // An empty ConfChangeV2 means leave_joint in raft-rs even with
            // Explicit transition. An existing voter add is idempotent and
            // preserves the requested equal incoming/outgoing voter sets.
            if voters == &current.voters {
                add(voters[0], ConfChangeType::AddNode);
            }
            change.set_transition(ConfChangeTransition::Explicit);
        }
        Change::LeaveJoint => {}
    }
    change
}
pub(super) fn parse(stored: &StoredEntry) -> Result<Entry, Error> {
    let entry = Entry::parse_from_bytes(&stored.bytes).map_err(|_| Error::Quarantined)?;
    if entry.index != stored.index
        || entry.term != stored.term
        || entry.term == 0
        || entry.term == u64::MAX
        || entry.sync_log
        || entry.get_unknown_fields().iter().next().is_some()
    {
        return Err(Error::Quarantined);
    }
    match entry.get_entry_type() {
        EntryType::EntryNormal => {
            if !entry.context.is_empty()
                || (!entry.data.is_empty() && crate::codec::decode(&entry.data).is_none())
            {
                return Err(Error::Quarantined);
            }
        }
        EntryType::EntryConfChangeV2 => {
            let (intent, _) = encoding::parse_context(&entry.context)?;
            let value =
                ConfChangeV2::parse_from_bytes(&entry.data).map_err(|_| Error::Quarantined)?;
            let changes = value.get_changes();
            let mut nodes = std::collections::BTreeSet::new();
            let valid = changes.iter().all(|change| {
                (1..=4).contains(&change.node_id)
                    && change.get_unknown_fields().iter().next().is_none()
                    && nodes.insert(change.node_id)
            }) && match intent.change {
                Change::AddLearner { node } => {
                    changes.len() == 1
                        && node == 4
                        && changes[0].node_id == node
                        && changes[0].get_change_type() == ConfChangeType::AddLearnerNode
                        && value.get_transition() == ConfChangeTransition::Auto
                }
                Change::EnterJoint { .. } => {
                    !changes.is_empty()
                        && value.get_transition() == ConfChangeTransition::Explicit
                        && changes.iter().all(|change| {
                            matches!(
                                change.get_change_type(),
                                ConfChangeType::AddNode | ConfChangeType::RemoveNode
                            )
                        })
                }
                Change::LeaveJoint => {
                    changes.is_empty() && value.get_transition() == ConfChangeTransition::Auto
                }
            };
            if !valid
                || value.get_unknown_fields().iter().next().is_some()
                || !value.context.is_empty()
            {
                return Err(Error::Quarantined);
            }
        }
        _ => {
            return Err(Error::Quarantined);
        }
    }
    Ok(entry)
}
