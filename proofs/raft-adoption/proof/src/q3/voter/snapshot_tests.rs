#[test]
fn actual_snapshot_ready_all_five_faults_preserve_coherent_join_cut_then_real_suffix() {
    use super::*;
    use crate::q3::{encoding, machine::change, recovery, storage::snapshot};
    use glade_raft_disk::{FaultPoint, v2::V2DiskStore};
    use glade_raft_q3_api::{Action, Change, ConfigOutcome, Instance, Outcome, conformance};
    use raft::Config;
    use raft::eraftpb::{EntryType, MessageType};
    for (case, fault) in [
        None,
        Some(FaultPoint::BeforeWrite),
        Some(FaultPoint::PartialWrite),
        Some(FaultPoint::AfterWrite),
        Some(FaultPoint::BeforeSync),
        Some(FaultPoint::AfterSync),
    ]
    .into_iter()
    .enumerate()
    {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../target")
            .join(format!("q3-snapshot-fault-{}-{case}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let instance = Instance {
            scope: 7,
            group: 70,
            node: 4,
            application_profile: 2,
        };
        let mut disk =
            V2DiskStore::create_new(&root.join("node-4"), instance.clone(), recovery::initial())
                .unwrap();
        let prior = disk.load().unwrap();
        let mut source = Machine::new();
        let create = conformance::create();
        for entry in [
            Entry {
                index: 1,
                term: 1,
                ..Entry::default()
            },
            Entry {
                index: 2,
                term: 1,
                data: crate::codec::encode(create, None).into(),
                ..Entry::default()
            },
        ] {
            source
                .apply(StoredEntry {
                    index: entry.index,
                    term: entry.term,
                    bytes: entry.write_to_bytes().unwrap(),
                })
                .unwrap();
        }
        assert_eq!(
            source.application.reply(create).unwrap().outcome,
            Outcome::Accepted(conformance::created_resource(1))
        );
        let intent = conformance::intent(1, 0, Change::AddLearner { node: 4 });
        let entry = Entry {
            index: 3,
            term: 1,
            entry_type: EntryType::EntryConfChangeV2,
            context: encoding::context(&intent, &[]).into(),
            data: change(&intent.change, &source.configuration)
                .write_to_bytes()
                .unwrap()
                .into(),
            ..Entry::default()
        };
        let original_entry = StoredEntry {
            index: 3,
            term: 1,
            bytes: entry.write_to_bytes().unwrap(),
        };
        source.apply(original_entry.clone()).unwrap();
        let original_config = source.configurations[&intent.key].clone();
        assert_eq!(original_config.outcome, ConfigOutcome::Accepted);
        let cp = source.checkpoint().unwrap();
        let mutation = conformance::command(2, Action::Mutate { payload: 23 });
        let suffix = Entry {
            index: 4,
            term: 1,
            data: crate::codec::encode(mutation, None).into(),
            ..Entry::default()
        };
        source
            .apply(StoredEntry {
                index: 4,
                term: 1,
                bytes: suffix.write_to_bytes().unwrap(),
            })
            .unwrap();
        let original_receipt = source.application.reply(mutation).unwrap();
        assert_eq!(
            original_receipt.outcome,
            Outcome::Accepted(glade_raft_q3_api::Resource {
                payload: 23,
                ..conformance::created_resource(1)
            })
        );
        let config = Config {
            id: 4,
            election_tick: 10,
            heartbeat_tick: 1,
            ..Config::default()
        };
        let raft = RawNode::new(
            &config,
            Q3Storage::new(prior.image.clone()),
            &slog::Logger::root(slog::Discard, slog::o!()),
        )
        .unwrap();
        if let Some(fault) = fault {
            disk.inject_once(fault);
        }
        let mut target = Voter {
            raft,
            machine: Machine::new(),
            store: Box::new(disk),
            state: prior.clone(),
            failure: None,
        };
        let mut message = Message::default();
        message.set_msg_type(MessageType::MsgSnapshot);
        message.from = 1;
        message.to = 4;
        message.term = 1;
        message.set_snapshot(snapshot(&cp));
        target.raft.step(message).unwrap();
        let ready = target.raft.ready();
        assert_eq!(
            ready.snapshot().get_metadata().index,
            3,
            "actual incoming Ready snapshot"
        );
        assert!(
            ready.committed_entries().is_empty(),
            "later suffix arrives separately"
        );
        let result = target.finish(ready);
        if fault.is_some() {
            assert_eq!(
                result,
                Err(Error::IoUnknown),
                "no messages returned at failed publication"
            );
            assert_eq!(target.state, prior);
            assert_eq!(target.machine.applied(), 0);
        } else {
            assert!(
                result
                    .unwrap()
                    .iter()
                    .any(|message| message.get_msg_type() == MessageType::MsgAppendResponse)
            );
            assert_eq!(target.machine.applied(), 3);
            assert_eq!(target.machine.configurations[&intent.key], original_config);
            let mut append = Message::default();
            append.set_msg_type(MessageType::MsgAppend);
            append.from = 1;
            append.to = 4;
            append.term = 1;
            append.index = 3;
            append.log_term = 1;
            append.commit = 4;
            append.mut_entries().push(suffix);
            target.raft.step(append).unwrap();
            target.ready().unwrap();
            assert_eq!(
                target.machine.application.reply(mutation),
                Some(original_receipt)
            );
        }
        drop(target);
        match V2DiskStore::open(&root.join("node-4"), instance.clone(), None) {
            Ok(mut reopened) => {
                let state = reopened.load().unwrap();
                let recovered = recovery::restore(&state, &instance).unwrap();
                if fault == Some(FaultPoint::BeforeWrite) {
                    assert_eq!(state, prior);
                    assert_eq!(recovered.applied(), 0);
                } else {
                    assert_eq!(recovered.configurations[&intent.key], original_config);
                    assert_eq!(recovered.history[&3].0, original_entry);
                    assert_eq!(
                        recovered.application.reply(create).unwrap().outcome,
                        Outcome::Accepted(conformance::created_resource(1))
                    );
                    assert_eq!(recovered.applied(), if fault.is_none() { 4 } else { 3 });
                }
            }
            Err(error) => {
                assert_eq!(fault, Some(FaultPoint::PartialWrite));
                assert_eq!(error, Error::Quarantined);
            }
        }
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn actual_matching_snapshot_fast_forward_and_stale_rejection_keep_original_history() {
    use super::*;
    use crate::q3::{recovery, storage::snapshot};
    use glade_raft_disk::v2::V2DiskStore;
    use glade_raft_q3_api::{Instance, Outcome, conformance};
    use raft::{Config, eraftpb::MessageType};
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-fast-forward-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let instance = Instance {
        scope: 7,
        group: 70,
        node: 3,
        application_profile: 2,
    };
    let mut complete = Machine::new();
    for entry in [
        Entry {
            index: 1,
            term: 1,
            ..Entry::default()
        },
        Entry {
            index: 2,
            term: 1,
            data: crate::codec::encode(conformance::create(), None).into(),
            ..Entry::default()
        },
        Entry {
            index: 3,
            term: 1,
            ..Entry::default()
        },
    ] {
        complete
            .apply(StoredEntry {
                index: entry.index,
                term: entry.term,
                bytes: entry.write_to_bytes().unwrap(),
            })
            .unwrap();
    }
    let cp = complete.checkpoint().unwrap();
    let image = Image {
        term: 1,
        vote: 1,
        commit: 2,
        applied: 2,
        configuration: conformance::stable(),
        checkpoint: None,
        suffix: complete
            .history
            .values()
            .map(|(entry, _)| entry.clone())
            .collect(),
    };
    let mut disk =
        V2DiskStore::create_new(&root.join("node-3"), instance.clone(), recovery::initial())
            .unwrap();
    let state = disk.publish(0, image.clone()).unwrap();
    let machine = recovery::restore(&state, &instance).unwrap();
    let config = Config {
        id: 3,
        election_tick: 10,
        heartbeat_tick: 1,
        applied: 2,
        ..Config::default()
    };
    let raft = RawNode::new(
        &config,
        Q3Storage::new(image),
        &slog::Logger::root(slog::Discard, slog::o!()),
    )
    .unwrap();
    let mut target = Voter {
        raft,
        machine,
        store: Box::new(disk),
        state,
        failure: None,
    };
    for _ in 0..2 {
        let mut message = Message::default();
        message.set_msg_type(MessageType::MsgSnapshot);
        message.from = 1;
        message.to = 3;
        message.term = 1;
        message.set_snapshot(snapshot(&cp));
        target.raft.step(message).unwrap();
        let ready = target.raft.ready();
        assert_eq!(
            ready.snapshot().get_metadata().index,
            0,
            "matching fast-forward/stale refusal never installs candidate"
        );
        target.finish(ready).unwrap();
        assert_eq!(target.machine.applied(), 3);
        assert!(target.state.image.checkpoint.is_none());
        assert_eq!(target.machine.history, complete.history);
        assert_eq!(
            target
                .machine
                .application
                .reply(conformance::create())
                .unwrap()
                .outcome,
            Outcome::Accepted(conformance::created_resource(1))
        );
    }
    drop(target);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn actual_append_after_snapshot_reconciles_uncommitted_original_suffix_without_applying_it() {
    use super::*;
    use crate::q3::recovery;
    use glade_raft_disk::v2::V2DiskStore;
    use glade_raft_q3_api::{Action, Instance, Outcome, conformance};
    use raft::{Config, eraftpb::MessageType};
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-reconcile-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let instance = Instance {
        scope: 7,
        group: 70,
        node: 3,
        application_profile: 2,
    };
    let mut machine = Machine::new();
    for entry in [
        Entry {
            index: 1,
            term: 1,
            ..Entry::default()
        },
        Entry {
            index: 2,
            term: 1,
            data: crate::codec::encode(conformance::create(), None).into(),
            ..Entry::default()
        },
    ] {
        machine
            .apply(StoredEntry {
                index: entry.index,
                term: entry.term,
                bytes: entry.write_to_bytes().unwrap(),
            })
            .unwrap();
    }
    let original = machine.history[&2].clone();
    assert_eq!(
        machine
            .application
            .reply(conformance::create())
            .unwrap()
            .outcome,
        Outcome::Accepted(conformance::created_resource(1))
    );
    let uncommitted = conformance::command(2, Action::Mutate { payload: 99 });
    let old = Entry {
        index: 3,
        term: 1,
        data: crate::codec::encode(uncommitted, None).into(),
        ..Entry::default()
    };
    let image = Image {
        term: 1,
        vote: 1,
        commit: 2,
        applied: 2,
        configuration: conformance::stable(),
        checkpoint: Some(machine.checkpoint().unwrap()),
        suffix: vec![StoredEntry {
            index: 3,
            term: 1,
            bytes: old.write_to_bytes().unwrap(),
        }],
    };
    let mut disk =
        V2DiskStore::create_new(&root.join("node-3"), instance.clone(), recovery::initial())
            .unwrap();
    let state = disk.publish(0, image.clone()).unwrap();
    let recovered = recovery::restore(&state, &instance).unwrap();
    assert!(recovered.application.reply(uncommitted).is_none());
    assert_eq!(recovered.history.len(), 2);
    let config = Config {
        id: 3,
        election_tick: 10,
        heartbeat_tick: 1,
        applied: 2,
        ..Config::default()
    };
    let raft = RawNode::new(
        &config,
        Q3Storage::new(image),
        &slog::Logger::root(slog::Discard, slog::o!()),
    )
    .unwrap();
    let mut target = Voter {
        raft,
        machine: recovered,
        store: Box::new(disk),
        state,
        failure: None,
    };
    let actual = conformance::command(2, Action::Mutate { payload: 23 });
    let entry = Entry {
        index: 3,
        term: 2,
        data: crate::codec::encode(actual, None).into(),
        ..Entry::default()
    };
    let expected_entry = StoredEntry {
        index: 3,
        term: 2,
        bytes: entry.write_to_bytes().unwrap(),
    };
    let mut message = Message::default();
    message.set_msg_type(MessageType::MsgAppend);
    message.from = 1;
    message.to = 3;
    message.term = 2;
    message.index = 2;
    message.log_term = 1;
    message.commit = 3;
    message.mut_entries().push(entry);
    target.raft.step(message).unwrap();
    target.ready().unwrap();
    assert_eq!(target.machine.history[&2], original);
    assert_eq!(target.machine.history[&3].0, expected_entry);
    assert_eq!(
        target.machine.application.reply(actual).unwrap().outcome,
        Outcome::Accepted(glade_raft_q3_api::Resource {
            payload: 23,
            ..conformance::created_resource(1)
        })
    );
    assert!(
        target.machine.application.reply(uncommitted).is_none(),
        "uncommitted changed command never gains an original receipt"
    );
    assert_eq!(
        target.machine.application.original_command(actual.request),
        Some(actual)
    );
    drop(target);
    let mut reopened = V2DiskStore::open(&root.join("node-3"), instance.clone(), None).unwrap();
    let recovered = recovery::restore(&reopened.load().unwrap(), &instance).unwrap();
    assert_eq!(recovered.history[&3].0, expected_entry);
    drop(reopened);
    std::fs::remove_dir_all(root).unwrap();
}
