//! Shared authority and stale-incarnation regressions, not engine qualification.
use glade_carrier_api::*;
use glade_carrier_spec::fixture::Fixture;
fn bytes() -> ProtocolMessage {
    ProtocolMessage {
        carrier: Carrier::OpenRaft,
        bytes: vec![7],
    }
}
#[test]
fn separately_obtained_and_fresh_handles_share_scope_stop() {
    let fixture = Fixture::new(Carrier::OpenRaft, 80);
    let mut first = fixture.endpoint(fixture.scope(1, 1));
    let mut separate = fixture.endpoint(fixture.scope(1, 1));
    first.stop().unwrap();
    let before = fixture.messages();
    assert_eq!(
        separate.emit(fixture.scope(2, 1).node, ProtocolKind::Other, bytes()),
        Err(DriverError::Stopped)
    );
    let mut fresh = fixture.endpoint(fixture.scope(1, 1));
    assert!(matches!(
        fresh.request(fixture.scope(2, 1).node, ProtocolKind::VoteRequest, bytes()),
        Err(DriverError::Stopped)
    ));
    assert_eq!(fixture.messages(), before);
}
#[test]
fn replaced_issuer_handle_cannot_emit_request_or_mutate_old_tokens() {
    let fixture = Fixture::new(Carrier::OpenRaft, 81);
    let mut old = fixture.endpoint(fixture.scope(1, 1));
    let held = old
        .emit(fixture.scope(2, 1).node, ProtocolKind::Other, bytes())
        .unwrap();
    let _replacement = fixture.inputs(1, 2);
    let before = fixture.messages();
    assert_eq!(
        old.emit(fixture.scope(2, 1).node, ProtocolKind::Other, bytes()),
        Err(DriverError::InvalidToken)
    );
    assert!(matches!(
        old.request(fixture.scope(2, 1).node, ProtocolKind::VoteRequest, bytes()),
        Err(DriverError::InvalidToken)
    ));
    for action in [
        TransportAction::Release(held),
        TransportAction::Hold(held),
        TransportAction::Drop(held),
        TransportAction::Duplicate(held),
    ] {
        assert_eq!(old.control(action), Err(DriverError::InvalidToken));
        assert_eq!(fixture.messages(), before);
    }
    let mut current = fixture.endpoint(fixture.scope(1, 2));
    let token = current
        .emit(fixture.scope(2, 1).node, ProtocolKind::Other, bytes())
        .unwrap();
    assert_eq!(
        token.sequence,
        held.sequence + 1,
        "refusal must not consume sequence"
    );
}
#[test]
fn replaced_destination_rejects_held_token_controls_before_mutation() {
    let fixture = Fixture::new(Carrier::OpenRaft, 82);
    let mut caller = fixture.endpoint(fixture.scope(1, 1));
    let held = caller
        .emit(fixture.scope(2, 1).node, ProtocolKind::Other, bytes())
        .unwrap();
    let _replacement = fixture.inputs(2, 2);
    let before = fixture.messages();
    for action in [
        TransportAction::Release(held),
        TransportAction::Hold(held),
        TransportAction::Drop(held),
        TransportAction::Duplicate(held),
    ] {
        assert_eq!(caller.control(action), Err(DriverError::InvalidToken));
        assert_eq!(fixture.messages(), before);
    }
    let token = caller
        .emit(fixture.scope(2, 2).node, ProtocolKind::Other, bytes())
        .unwrap();
    assert_eq!(token.sequence, held.sequence + 1);
}
#[test]
fn take_refuses_replaced_issuer_and_destination_without_consuming() {
    for replace in [1, 2] {
        let fixture = Fixture::new(Carrier::OpenRaft, 83 + replace);
        let mut caller = fixture.endpoint(fixture.scope(1, 1));
        let mut peer = fixture.endpoint(fixture.scope(2, 1));
        let token = caller
            .emit(fixture.scope(2, 1).node, ProtocolKind::Other, bytes())
            .unwrap();
        caller.control(TransportAction::Release(token)).unwrap();
        let _replacement = fixture.inputs(replace, 2);
        let before = fixture.messages();
        assert!(matches!(peer.take(token), Err(DriverError::InvalidToken)));
        assert_eq!(fixture.messages(), before);
    }
}
#[test]
fn timeout_and_cancel_validate_live_rpc_peer_before_removing_ownership() {
    for cancel in [false, true] {
        let fixture = Fixture::new(Carrier::OpenRaft, 86 + u64::from(cancel));
        let mut caller = fixture.endpoint(fixture.scope(1, 1));
        let rpc = caller
            .request(fixture.scope(2, 1).node, ProtocolKind::VoteRequest, bytes())
            .unwrap();
        let _replacement = fixture.inputs(2, 2);
        let before = fixture.messages();
        let action = if cancel {
            TransportAction::Cancel(rpc.id)
        } else {
            TransportAction::Timeout(rpc.id)
        };
        assert_eq!(caller.control(action), Err(DriverError::InvalidToken));
        assert_eq!(caller.pending_rpcs(), vec![rpc.id]);
        assert_eq!(fixture.messages(), before);
    }
}
#[test]
fn stopped_peer_and_caller_reject_all_rpc_and_queue_mutations() {
    for stop_node in [1, 2] {
        let fixture = Fixture::new(Carrier::OpenRaft, 88 + stop_node);
        let mut caller = fixture.endpoint(fixture.scope(1, 1));
        let mut peer = fixture.endpoint(fixture.scope(2, 1));
        let rpc = caller
            .request(fixture.scope(2, 1).node, ProtocolKind::VoteRequest, bytes())
            .unwrap();
        caller
            .control(TransportAction::Release(rpc.message))
            .unwrap();
        peer.take(rpc.message).unwrap();
        let reply = peer
            .respond(rpc.id, ProtocolKind::VoteResponse, bytes())
            .unwrap();
        peer.control(TransportAction::Release(reply)).unwrap();
        fixture
            .endpoint(fixture.scope(stop_node, 1))
            .stop()
            .unwrap();
        let before = fixture.messages();
        let pending = caller.pending_rpcs();
        let caller_error = if stop_node == 1 {
            DriverError::Stopped
        } else {
            DriverError::InvalidToken
        };
        for action in [
            TransportAction::Resolve { rpc: rpc.id, reply },
            TransportAction::Timeout(rpc.id),
            TransportAction::Cancel(rpc.id),
        ] {
            assert_eq!(caller.control(action), Err(caller_error));
            assert_eq!(caller.pending_rpcs(), pending);
            assert_eq!(fixture.messages(), before);
        }
        let peer_error = if stop_node == 2 {
            DriverError::Stopped
        } else {
            DriverError::InvalidToken
        };
        assert_eq!(
            peer.respond(rpc.id, ProtocolKind::VoteResponse, bytes()),
            Err(peer_error)
        );
        assert_eq!(
            peer.control(TransportAction::Duplicate(reply)),
            Err(peer_error)
        );
        assert!(
            matches!(caller.request(fixture.scope(2, 1).node, ProtocolKind::VoteRequest, bytes()), Err(error) if error == caller_error)
        );
        assert_eq!(fixture.messages(), before);
        // Local stop still owns and cancels its RPC even if the peer is terminal.
        caller.stop().unwrap();
        assert!(caller.pending_rpcs().is_empty());
    }
}
#[test]
fn both_replacements_participate_and_old_rpc_controls_refuse_unchanged() {
    let fixture = Fixture::new(Carrier::OpenRaft, 93);
    let mut old_caller = fixture.endpoint(fixture.scope(1, 1));
    let mut old_peer = fixture.endpoint(fixture.scope(2, 1));
    let old = old_caller
        .request(fixture.scope(2, 1).node, ProtocolKind::VoteRequest, bytes())
        .unwrap();
    old_caller
        .control(TransportAction::Release(old.message))
        .unwrap();
    old_peer.take(old.message).unwrap();
    let old_reply = old_peer
        .respond(old.id, ProtocolKind::VoteResponse, bytes())
        .unwrap();
    old_peer
        .control(TransportAction::Release(old_reply))
        .unwrap();
    let _caller = fixture.inputs(1, 2);
    let _peer = fixture.inputs(2, 2);
    let before = fixture.messages();
    for action in [
        TransportAction::Resolve {
            rpc: old.id,
            reply: old_reply,
        },
        TransportAction::Timeout(old.id),
        TransportAction::Cancel(old.id),
    ] {
        assert_eq!(old_caller.control(action), Err(DriverError::InvalidToken));
        assert_eq!(fixture.messages(), before);
        assert_eq!(old_caller.pending_rpcs(), vec![old.id]);
    }
    assert_eq!(
        old_peer.respond(old.id, ProtocolKind::VoteResponse, bytes()),
        Err(DriverError::InvalidToken)
    );
    let mut caller = fixture.endpoint(fixture.scope(1, 2));
    let mut peer = fixture.endpoint(fixture.scope(2, 2));
    let rpc = caller
        .request(fixture.scope(2, 2).node, ProtocolKind::VoteRequest, bytes())
        .unwrap();
    assert_eq!(rpc.message.sequence, old_reply.sequence + 1);
    caller
        .control(TransportAction::Release(rpc.message))
        .unwrap();
    peer.take(rpc.message).unwrap();
    let reply = peer
        .respond(rpc.id, ProtocolKind::VoteResponse, bytes())
        .unwrap();
    peer.control(TransportAction::Release(reply)).unwrap();
    caller
        .control(TransportAction::Resolve { rpc: rpc.id, reply })
        .unwrap();
    assert!(caller.pending_rpcs().is_empty());
}
