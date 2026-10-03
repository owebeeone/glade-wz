#[test]
fn actual_snapshot_ready_rejects_valid_conflicting_committed_application_history() {
    use super::{
        machine::Machine,
        storage::{Q3Storage, snapshot},
        voter::Voter,
    };
    use glade_raft_disk::v2::V2DiskStore;
    use glade_raft_q3_api::{
        Action, CheckpointStore, Error, Image, Instance, StoredEntry, conformance,
    };
    use protobuf::Message as ProtobufMessage;
    use raft::eraftpb::{Entry, Message, MessageType};
    use raft::{Config, RawNode};
    struct Directory(std::path::PathBuf);
    impl Drop for Directory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-conflicting-snapshot-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let directory = Directory(root);
    let instance = Instance {
        scope: 7,
        group: 70,
        node: 3,
        application_profile: 2,
    };
    let initial = Image {
        term: 0,
        vote: 0,
        commit: 0,
        applied: 0,
        configuration: conformance::stable(),
        checkpoint: None,
        suffix: Vec::new(),
    };
    let mut disk = V2DiskStore::create_new(&directory.0.join("node-3"), instance, initial).unwrap();
    let build = |payload| {
        let mut machine = Machine::new();
        let mut command = conformance::create();
        command.action = Action::Create {
            name: 40,
            home: 1,
            payload,
        };
        for entry in [
            Entry {
                index: 1,
                term: 1,
                ..Entry::default()
            },
            Entry {
                index: 2,
                term: 1,
                data: crate::codec::encode(command, None).into(),
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
        machine
    };
    let original = build(11);
    let image = Image {
        term: 1,
        vote: 1,
        commit: 2,
        applied: 2,
        configuration: original.configuration.clone(),
        checkpoint: None,
        suffix: original
            .history
            .values()
            .map(|(entry, _)| entry.clone())
            .collect(),
    };
    let state = disk.publish(0, image.clone()).unwrap();
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
    let mut voter = Voter {
        raft,
        machine: original,
        store: Box::new(disk),
        state: state.clone(),
        failure: None,
    };
    let mut foreign = build(99);
    let noop = Entry {
        index: 3,
        term: 2,
        ..Entry::default()
    };
    foreign
        .apply(StoredEntry {
            index: 3,
            term: 2,
            bytes: noop.write_to_bytes().unwrap(),
        })
        .unwrap();
    let checkpoint = foreign.checkpoint().unwrap();
    assert!(Machine::restore(&checkpoint).is_ok());
    let mut message = Message::default();
    message.set_msg_type(MessageType::MsgSnapshot);
    message.from = 1;
    message.to = 3;
    message.term = 2;
    message.set_snapshot(snapshot(&checkpoint));
    voter.raft.step(message).unwrap();
    assert!(voter.raft.has_ready());
    assert_eq!(voter.ready(), Err(Error::Quarantined));
    assert_eq!(voter.state, state, "no conflicting checkpoint publication");
    assert_eq!(voter.machine.application.resource(100).unwrap().payload, 11);
}
