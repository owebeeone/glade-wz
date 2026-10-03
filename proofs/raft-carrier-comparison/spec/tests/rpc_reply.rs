//! Controlled fixture transport only; RED precedes respond mechanics.
use glade_carrier_api::*;
use glade_carrier_spec::fixture::{Endpoint, Fixture};
fn setup(session: u64) -> (Fixture, Endpoint, Endpoint, PendingRpc) {
    let fixture = Fixture::new(Carrier::OpenRaft, session);
    let mut caller = fixture.endpoint(fixture.scope(1, 1));
    let mut peer = fixture.endpoint(fixture.scope(2, 1));
    let pending = caller
        .request(
            fixture.scope(2, 1).node,
            ProtocolKind::VoteRequest,
            ProtocolMessage {
                carrier: Carrier::OpenRaft,
                bytes: vec![1],
            },
        )
        .unwrap();
    caller
        .control(TransportAction::Release(pending.message))
        .unwrap();
    peer.take(pending.message).unwrap();
    (fixture, caller, peer, pending)
}
#[test]
fn issued_correlated_reply_resolves_only_its_actual_pending_rpc() {
    let (fixture, mut caller, mut peer, pending) = setup(30);
    let reply = peer
        .respond(
            pending.id,
            ProtocolKind::VoteResponse,
            ProtocolMessage {
                carrier: fixture.carrier,
                bytes: vec![2],
            },
        )
        .expect("typed correlated reply must issue a token");
    let view = peer
        .messages()
        .into_iter()
        .find(|message| message.token == reply)
        .unwrap();
    assert_eq!(view.reply_to, Some(pending.id));
    assert_eq!(view.rpc, None);
    peer.control(TransportAction::Release(reply)).unwrap();
    assert_eq!(
        caller.control(TransportAction::Resolve {
            rpc: pending.id,
            reply
        }),
        Ok(vec![Event::RpcResolved { rpc: pending.id }])
    );
    assert!(caller.pending_rpcs().is_empty());
    assert_eq!(
        caller.control(TransportAction::Resolve {
            rpc: pending.id,
            reply
        }),
        Err(DriverError::InvalidToken)
    );
}
#[test]
fn wrong_rpc_correlation_cannot_consume_reply_or_settle_either_rpc() {
    let (fixture, mut caller, mut peer, first) = setup(31);
    let second = caller
        .request(
            fixture.scope(2, 1).node,
            ProtocolKind::VoteRequest,
            ProtocolMessage {
                carrier: fixture.carrier,
                bytes: vec![3],
            },
        )
        .unwrap();
    let reply = peer
        .respond(
            first.id,
            ProtocolKind::VoteResponse,
            ProtocolMessage {
                carrier: fixture.carrier,
                bytes: vec![2],
            },
        )
        .unwrap();
    peer.control(TransportAction::Release(reply)).unwrap();
    let before = caller.messages();
    let pending = caller.pending_rpcs();
    assert_eq!(
        caller.control(TransportAction::Resolve {
            rpc: second.id,
            reply
        }),
        Err(DriverError::InvalidToken)
    );
    assert_eq!(caller.messages(), before);
    assert_eq!(caller.pending_rpcs(), pending);
    caller
        .control(TransportAction::Resolve {
            rpc: first.id,
            reply,
        })
        .unwrap();
    assert_eq!(caller.pending_rpcs(), vec![second.id]);
}
#[test]
fn foreign_peer_and_old_incarnation_cannot_issue_or_settle_a_reply() {
    let (fixture, mut caller, mut peer, pending) = setup(32);
    let mut foreign = fixture.endpoint(fixture.scope(3, 1));
    let mut stale = fixture.endpoint(fixture.scope(2, 0));
    let before = fixture.messages();
    for endpoint in [&mut foreign, &mut stale] {
        assert_eq!(
            endpoint.respond(
                pending.id,
                ProtocolKind::VoteResponse,
                ProtocolMessage {
                    carrier: fixture.carrier,
                    bytes: vec![2]
                }
            ),
            Err(DriverError::InvalidToken)
        );
        assert_eq!(fixture.messages(), before);
    }
    let forged = foreign
        .emit(
            fixture.scope(1, 1).node,
            ProtocolKind::VoteResponse,
            ProtocolMessage {
                carrier: fixture.carrier,
                bytes: vec![2],
            },
        )
        .unwrap();
    foreign.control(TransportAction::Release(forged)).unwrap();
    let before = caller.messages();
    assert_eq!(
        caller.control(TransportAction::Resolve {
            rpc: pending.id,
            reply: forged
        }),
        Err(DriverError::InvalidToken)
    );
    assert_eq!(caller.messages(), before);
    assert_eq!(caller.pending_rpcs(), vec![pending.id]);
    // An actually issued old-peer reply cannot settle after peer replacement.
    let old_reply = peer
        .respond(
            pending.id,
            ProtocolKind::VoteResponse,
            ProtocolMessage {
                carrier: fixture.carrier,
                bytes: vec![2],
            },
        )
        .unwrap();
    peer.control(TransportAction::Release(old_reply)).unwrap();
    let _replacement_inputs = fixture.inputs(2, 2);
    let before = caller.messages();
    assert_eq!(
        caller.control(TransportAction::Resolve {
            rpc: pending.id,
            reply: old_reply
        }),
        Err(DriverError::InvalidToken)
    );
    assert_eq!(caller.messages(), before);
    assert_eq!(caller.pending_rpcs(), vec![pending.id]);
}
#[test]
fn wrong_carrier_bytes_or_reply_kind_refuse_before_queue_mutation() {
    let (fixture, caller, mut peer, pending) = setup(33);
    let before = fixture.messages();
    assert_eq!(
        peer.respond(
            pending.id,
            ProtocolKind::VoteResponse,
            ProtocolMessage {
                carrier: Carrier::RaftRs,
                bytes: vec![2]
            }
        ),
        Err(DriverError::Protocol)
    );
    assert_eq!(
        peer.respond(
            pending.id,
            ProtocolKind::AppendResponse,
            ProtocolMessage {
                carrier: fixture.carrier,
                bytes: vec![2]
            }
        ),
        Err(DriverError::Protocol)
    );
    assert_eq!(fixture.messages(), before);
    assert_eq!(caller.pending_rpcs(), vec![pending.id]);
}
