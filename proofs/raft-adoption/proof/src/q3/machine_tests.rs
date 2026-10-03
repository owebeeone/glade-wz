#[test]
fn checkpoint_history_terms_must_not_regress() {
    use crate::q3::machine::Machine;
    use glade_raft_q3_api::{Error, StoredEntry, conformance};
    use protobuf::Message as ProtobufMessage;
    use raft::eraftpb::Entry;
    fn fixture() -> Machine {
        let mut machine = Machine::new();
        let noop = Entry {
            index: 1,
            term: 1,
            ..Entry::default()
        };
        machine
            .apply(StoredEntry {
                index: 1,
                term: 1,
                bytes: noop.write_to_bytes().unwrap(),
            })
            .unwrap();
        let create = Entry {
            index: 2,
            term: 1,
            data: crate::codec::encode(conformance::create(), None).into(),
            ..Entry::default()
        };
        machine
            .apply(StoredEntry {
                index: 2,
                term: 1,
                bytes: create.write_to_bytes().unwrap(),
            })
            .unwrap();
        machine
    }

    let machine = fixture();
    let mut checkpoint = machine.checkpoint().unwrap();
    // Complete canonical versioned frame, original noop is still a valid Entry,
    // result and all materialized maps remain plausible. Its term is now 2,
    // followed by the original term-1 creation at checkpoint term 1.
    checkpoint.application[56..64].copy_from_slice(&2_u64.to_le_bytes());
    let count = u64::from_le_bytes(checkpoint.application[64..72].try_into().unwrap()) as usize;
    let mut entry = Entry::parse_from_bytes(&checkpoint.application[72..72 + count]).unwrap();
    entry.term = 2;
    let replacement = entry.write_to_bytes().unwrap();
    assert_eq!(replacement.len(), count);
    checkpoint.application[72..72 + count].copy_from_slice(&replacement);
    assert!(matches!(
        Machine::restore(&checkpoint),
        Err(Error::Quarantined)
    ));
}
#[test]
fn valid_format_materialized_maps_must_equal_full_original_replay() {
    use crate::q3::machine::{Machine, parse};
    use glade_raft_q3_api::{Error, StoredEntry, conformance};
    use protobuf::Message as ProtobufMessage;
    use raft::eraftpb::Entry;
    fn fixture() -> Machine {
        let mut machine = Machine::new();
        let noop = Entry {
            index: 1,
            term: 1,
            ..Entry::default()
        };
        machine
            .apply(StoredEntry {
                index: 1,
                term: 1,
                bytes: noop.write_to_bytes().unwrap(),
            })
            .unwrap();
        let create = Entry {
            index: 2,
            term: 1,
            data: crate::codec::encode(conformance::create(), None).into(),
            ..Entry::default()
        };
        machine
            .apply(StoredEntry {
                index: 2,
                term: 1,
                bytes: create.write_to_bytes().unwrap(),
            })
            .unwrap();
        machine
    }

    let machine = fixture();
    let checkpoint = machine.checkpoint().unwrap();
    let materialized = machine.application.complete_evidence();
    let start = checkpoint.application.len() - materialized.len();
    // Full canonical scalar values in the materialized application envelope:
    // applied, policy frontier, voter, resource generation/home/payload/tombstone,
    // reserved name/id. Framing/history remain intact for each adversary.
    for offset in [8, 16, 32, 88, 96, 104, 112, 128, 136] {
        let mut bad = checkpoint.clone();
        let pos = start + offset;
        let previous = u64::from_le_bytes(bad.application[pos..pos + 8].try_into().unwrap());
        bad.application[pos..pos + 8].copy_from_slice(&(previous ^ 1).to_le_bytes());
        assert!(
            matches!(Machine::restore(&bad), Err(Error::Quarantined)),
            "materialized offset {offset}"
        );
    }
    assert!(Machine::restore(&checkpoint).is_ok());
    for (stored, _) in machine.history.values() {
        parse(stored).unwrap();
    }
}

#[test]
fn private_uncommitted_configuration_entries_reject_unsupported_carrier_grammar() {
    use super::{
        encoding,
        machine::{Machine, change, parse},
    };
    use glade_raft_q3_api::{Change, Error, StoredEntry, conformance};
    use protobuf::Message as ProtobufMessage;
    use raft::eraftpb::{ConfChangeSingle, ConfChangeType, Entry, EntryType};
    let machine = Machine::new();
    let intent = conformance::intent(1, 0, Change::AddLearner { node: 4 });
    let context = encoding::context(&intent, &[]);
    let original = change(&intent.change, &machine.configuration);
    let mut variants = Vec::new();
    let mut unknown = original.clone();
    unknown.mut_changes()[0]
        .mut_unknown_fields()
        .add_varint(100, 1);
    variants.push(unknown);
    let mut demote = original.clone();
    demote.mut_changes()[0].node_id = 1;
    variants.push(demote);
    let mut duplicate = original.clone();
    duplicate.mut_changes().push(ConfChangeSingle {
        node_id: 4,
        change_type: ConfChangeType::AddLearnerNode,
        ..ConfChangeSingle::default()
    });
    variants.push(duplicate);
    for carrier in variants {
        let entry = Entry {
            index: 1,
            term: 1,
            entry_type: EntryType::EntryConfChangeV2,
            data: carrier.write_to_bytes().unwrap().into(),
            context: context.clone().into(),
            ..Entry::default()
        };
        assert_eq!(
            parse(&StoredEntry {
                index: 1,
                term: 1,
                bytes: entry.write_to_bytes().unwrap()
            }),
            Err(Error::Quarantined)
        );
    }
}

#[test]
fn valid_format_original_command_result_index_policy_permissions_and_cut_adversaries_quarantine() {
    use super::{
        encoding::word,
        machine::{Machine, result_bytes},
    };
    use glade_raft_q3_api::{Action, Error, Outcome, StoredEntry, conformance};
    use protobuf::Message as ProtobufMessage;
    use raft::eraftpb::Entry;
    let mut machine = Machine::new();
    let mut retire = conformance::command(3, Action::Retire);
    retire.policy_frontier = 3;
    let commands = [
        None,
        Some(conformance::create()),
        Some(conformance::command(
            2,
            Action::SetPermission {
                principal: 10,
                write: false,
                disclose: false,
            },
        )),
        Some(retire),
    ];
    for (offset, command) in commands.into_iter().enumerate() {
        let index = offset as u64 + 1;
        let entry = Entry {
            index,
            term: 1,
            data: command
                .map_or_else(Vec::new, |command| crate::codec::encode(command, None))
                .into(),
            ..Entry::default()
        };
        machine
            .apply(StoredEntry {
                index,
                term: 1,
                bytes: entry.write_to_bytes().unwrap(),
            })
            .unwrap();
    }
    assert_eq!(
        machine
            .application
            .reply(conformance::create())
            .unwrap()
            .outcome,
        Outcome::Accepted(conformance::created_resource(1))
    );
    assert_eq!(
        machine.application.reply(retire).unwrap().outcome,
        Outcome::Accepted(glade_raft_q3_api::Resource {
            retired: true,
            ..conformance::created_resource(1)
        })
    );
    let cp = machine.checkpoint().unwrap();
    assert!(Machine::restore(&cp).is_ok());
    let original_entry = &machine.history[&2].0;
    let mut changed = Entry::parse_from_bytes(&original_entry.bytes).unwrap();
    let mut command = conformance::create();
    command.action = Action::Create {
        name: 40,
        home: 1,
        payload: 99,
    };
    changed.data = crate::codec::encode(command, None).into();
    let replacement = changed.write_to_bytes().unwrap();
    assert_eq!(replacement.len(), original_entry.bytes.len());
    let pos = cp
        .application
        .windows(original_entry.bytes.len())
        .position(|bytes| bytes == original_entry.bytes)
        .unwrap();
    let mut bad = cp.clone();
    bad.application[pos..pos + replacement.len()].copy_from_slice(&replacement);
    assert!(
        matches!(Machine::restore(&bad), Err(Error::Quarantined)),
        "canonical changed original command with original retained result"
    );
    let mut encoded_result = Vec::new();
    result_bytes(&mut encoded_result, &machine.history[&2].1);
    let pos = cp
        .application
        .windows(encoded_result.len())
        .position(|bytes| bytes == encoded_result)
        .unwrap();
    for offset in [8, 48, 88, 96, 104] {
        let mut bad = cp.clone();
        let start = pos + offset;
        let original = u64::from_le_bytes(bad.application[start..start + 8].try_into().unwrap());
        bad.application[start..start + 8].copy_from_slice(&(original ^ 1).to_le_bytes());
        assert!(
            matches!(Machine::restore(&bad), Err(Error::Quarantined)),
            "canonical typed original result offset {offset}"
        );
    }
    let materialized = machine.application.complete_evidence();
    let start = cp.application.len() - materialized.len();
    let mut permission = Vec::new();
    for value in [100, 1, 10, 0, 0] {
        word(&mut permission, value);
    }
    let pos = materialized
        .windows(permission.len())
        .position(|bytes| bytes == permission)
        .unwrap();
    for offset in [0, 8, 16, 24, 32] {
        let mut bad = cp.clone();
        let at = start + pos + offset;
        let old = u64::from_le_bytes(bad.application[at..at + 8].try_into().unwrap());
        bad.application[at..at + 8].copy_from_slice(&(old ^ 1).to_le_bytes());
        assert!(
            matches!(Machine::restore(&bad), Err(Error::Quarantined)),
            "full permission key/value {offset}"
        );
    }
    for case in 0..4 {
        let mut bad = cp.clone();
        match case {
            0 => {
                bad.index += 1;
            }
            1 => {
                bad.term += 1;
            }
            2 => {
                bad.configuration.voters = vec![1, 2, 4];
            }
            _ => {
                bad.configuration.index = 2;
            }
        }
        assert!(
            matches!(Machine::restore(&bad), Err(Error::Quarantined)),
            "cut/ConfState {case}"
        );
    }
    let mut bad = cp.clone();
    bad.application[48..56].copy_from_slice(&2_u64.to_le_bytes());
    assert!(
        matches!(Machine::restore(&bad), Err(Error::Quarantined)),
        "changed original index with valid word framing"
    );
}

#[test]
fn empty_explicit_carrier_entry_cannot_be_reopened_as_private_enter_joint() {
    use super::{encoding, machine::parse};
    use glade_raft_q3_api::{Change, Error, StoredEntry, conformance};
    use protobuf::Message as ProtobufMessage;
    use raft::eraftpb::{ConfChangeTransition, ConfChangeV2, Entry, EntryType};
    let intent = conformance::intent(
        1,
        0,
        Change::EnterJoint {
            voters: vec![1, 2, 3],
        },
    );
    let carrier = ConfChangeV2 {
        transition: ConfChangeTransition::Explicit,
        ..ConfChangeV2::default()
    };
    let entry = Entry {
        index: 2,
        term: 1,
        entry_type: EntryType::EntryConfChangeV2,
        context: encoding::context(&intent, &[]).into(),
        data: carrier.write_to_bytes().unwrap().into(),
        ..Entry::default()
    };
    assert_eq!(
        parse(&StoredEntry {
            index: 2,
            term: 1,
            bytes: entry.write_to_bytes().unwrap()
        }),
        Err(Error::Quarantined)
    );
}
