//! Carrier-only design counterexample. No application/authority/I/O qualification.
use raft::eraftpb::{Message, MessageType, Snapshot};
use raft::storage::MemStorage;
use raft::{Config, RawNode};
fn checkpoint() -> Snapshot {
    let mut snapshot = Snapshot::default();
    snapshot.data = b"carrier-only".to_vec().into();
    let metadata = snapshot.mut_metadata();
    metadata.index = 5;
    metadata.term = 1;
    metadata.mut_conf_state().voters = vec![1, 2, 3];
    metadata.mut_conf_state().learners = vec![4];
    snapshot
}
fn receiver(seeded: bool) -> RawNode<MemStorage> {
    let storage = MemStorage::new_with_conf_state((vec![1, 2, 3], Vec::<u64>::new()));
    if seeded {
        storage.wl().apply_snapshot(checkpoint()).unwrap();
    }
    let config = Config {
        id: 4,
        election_tick: 10,
        heartbeat_tick: 1,
        applied: if seeded { 5 } else { 0 },
        ..Config::default()
    };
    RawNode::new(
        &config,
        storage,
        &slog::Logger::root(slog::Discard, slog::o!()),
    )
    .unwrap()
}
fn incoming(receiver: &mut RawNode<MemStorage>) -> bool {
    let mut message = Message::default();
    message.set_msg_type(MessageType::MsgSnapshot);
    message.from = 1;
    message.to = 4;
    message.term = 2;
    message.set_snapshot(checkpoint());
    receiver.step(message).unwrap();
    assert!(receiver.has_ready());
    receiver.ready().snapshot().get_metadata().index == 5
}
#[test]
fn seeded_join_cut_cannot_receive_same_cut_as_actual_snapshot_ready() {
    assert!(
        !incoming(&mut receiver(true)),
        "seeded commit S rejects same-cut snapshot before restoration"
    );
    assert!(
        incoming(&mut receiver(false)),
        "original-genesis nonserving receiver obtains actual snapshot Ready at S"
    );
}
