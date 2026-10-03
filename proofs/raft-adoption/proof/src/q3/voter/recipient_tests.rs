#[test]
fn actual_snapshot_recipient_rejection_never_installs_or_claims_join_readiness() {
    use super::*;
    use crate::q3::{recovery, storage::snapshot};
    use glade_raft_disk::v2::V2DiskStore;
    use glade_raft_q3_api::{Instance, Outcome, conformance};
    use raft::{Config, eraftpb::MessageType};
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../target")
        .join(format!("q3-recipient-{}", std::process::id()));
    std::fs::create_dir(&root).unwrap();
    let instance = Instance {
        scope: 7,
        group: 70,
        node: 4,
        application_profile: 2,
    };
    let mut disk =
        V2DiskStore::create_new(&root.join("node-4"), instance, recovery::initial()).unwrap();
    let state = disk.load().unwrap();
    let mut source = Machine::new();
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
        source
            .apply(StoredEntry {
                index: entry.index,
                term: entry.term,
                bytes: entry.write_to_bytes().unwrap(),
            })
            .unwrap();
    }
    assert_eq!(
        source
            .application
            .reply(conformance::create())
            .unwrap()
            .outcome,
        Outcome::Accepted(conformance::created_resource(1))
    );
    let cp = source.checkpoint().unwrap();
    assert!(!cp.configuration.voters.contains(&4) && !cp.configuration.learners.contains(&4));
    let config = Config {
        id: 4,
        election_tick: 10,
        heartbeat_tick: 1,
        ..Config::default()
    };
    let raft = RawNode::new(
        &config,
        Q3Storage::new(state.image.clone()),
        &slog::Logger::root(slog::Discard, slog::o!()),
    )
    .unwrap();
    let mut target = Voter {
        raft,
        machine: Machine::new(),
        store: Box::new(disk),
        state,
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
    assert_eq!(ready.snapshot().get_metadata().index, 0);
    target.finish(ready).unwrap();
    assert_eq!(target.machine.applied(), 0);
    assert_eq!(target.state.image.applied, 0);
    assert!(target.state.image.checkpoint.is_none());
    assert!(target.machine.application.resource(100).is_none());
    drop(target);
    std::fs::remove_dir_all(root).unwrap();
}
