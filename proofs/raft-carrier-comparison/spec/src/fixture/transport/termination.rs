//! Local termination regressions, with read-only inspection of original bytes.
// Unconditional module; generated ordinary tests need no conditional attributes.
macro_rules! termination_case {
    ($name:ident, $consumed:expr, $replace:expr, $cancel:expr) => {
        #[test]
        fn $name() {
            use crate::fixture::Fixture;
            use glade_carrier_api::*;
            use std::{
                cell::{Cell, RefCell},
                rc::Rc,
            };
            let fixture = Fixture::new(Carrier::OpenRaft, 110);
            let _caller_inputs = fixture.inputs(1, 1);
            let _peer_inputs = fixture.inputs(2, 1);
            let mut caller = fixture.endpoint(fixture.scope(1, 1));
            let mut peer = fixture.endpoint(fixture.scope(2, 1));
            let mut foreign = fixture.endpoint(fixture.scope(3, 1));
            let mut work = fixture.work(1, 1);
            let mut peer_work = fixture.work(2, 1);
            peer_work.spawn(Box::pin(std::future::pending())).unwrap();
            let original = ProtocolMessage {
                carrier: fixture.carrier,
                bytes: vec![0, 255, 3, 7],
            };
            let pending = caller
                .request(
                    fixture.scope(2, 1).node,
                    ProtocolKind::VoteRequest,
                    original.clone(),
                )
                .unwrap();
            let rpc = pending.id;
            let request = pending.message;
            let unrelated = caller
                .request(
                    fixture.scope(3, 1).node,
                    ProtocolKind::VoteRequest,
                    original.clone(),
                )
                .unwrap();
            let peer_owned = peer
                .request(
                    fixture.scope(3, 1).node,
                    ProtocolKind::VoteRequest,
                    original.clone(),
                )
                .unwrap();
            let mut late_reply = None;
            if $consumed {
                caller.control(TransportAction::Release(request)).unwrap();
                assert_eq!(peer.take(request).unwrap().message, original);
                let reply = peer
                    .respond(rpc, ProtocolKind::VoteResponse, original.clone())
                    .unwrap();
                peer.control(TransportAction::Release(reply)).unwrap();
                late_reply = Some(reply);
            }
            let completed = Rc::new(Cell::new(0));
            let received = Rc::new(RefCell::new(None));
            let completion = completed.clone();
            let result = received.clone();
            let task = work
                .spawn(Box::pin(async move {
                    *result.borrow_mut() = Some(pending.response.await);
                    completion.set(completion.get() + 1);
                }))
                .unwrap();
            assert_eq!(work.poll(task), Ok(WorkPoll::Pending));
            assert_eq!(completed.get(), 0);
            let bytes_snapshot = || {
                fixture
                    .network
                    .borrow()
                    .messages
                    .iter()
                    .map(|(token, queued)| (*token, queued.message.clone()))
                    .collect::<Vec<_>>()
            };
            let original_bytes = bytes_snapshot();
            if $replace {
                let _replacement = fixture.inputs(2, 2);
            } else {
                fixture.endpoint(fixture.scope(2, 1)).stop().unwrap();
            }
            assert_eq!(
                bytes_snapshot(),
                original_bytes,
                "peer lifecycle preserves issued bytes"
            );
            let before = fixture.messages();
            let peer_messages = peer.messages();
            let peer_rpcs = peer.pending_rpcs();
            let peer_tasks = peer_work.inventory();
            let replacement = fixture.endpoint(fixture.scope(2, if $replace { 2 } else { 1 }));
            let replacement_messages = replacement.messages();
            let replacement_rpcs = replacement.pending_rpcs();
            let replacement_tasks = if $replace {
                Some(fixture.work(2, 2).inventory())
            } else {
                None
            };
            let selected = if $cancel {
                TransportAction::Cancel(rpc)
            } else {
                TransportAction::Timeout(rpc)
            };
            assert_eq!(foreign.control(selected), Err(DriverError::InvalidToken));
            let forged = RpcId {
                sequence: rpc.sequence + 999,
                ..rpc
            };
            for action in [
                TransportAction::Timeout(forged),
                TransportAction::Cancel(forged),
            ] {
                assert_eq!(caller.control(action), Err(DriverError::InvalidToken));
                assert_eq!(caller.pending_rpcs(), vec![rpc, unrelated.id]);
                assert_eq!(fixture.messages(), before);
            }
            let reply_error = if $replace {
                DriverError::InvalidToken
            } else {
                DriverError::Stopped
            };
            assert_eq!(
                peer.respond(rpc, ProtocolKind::VoteResponse, original.clone()),
                Err(reply_error)
            );
            if let Some(reply) = late_reply {
                assert_eq!(
                    caller.control(TransportAction::Resolve { rpc, reply }),
                    Err(DriverError::InvalidToken)
                );
            }
            assert_eq!(fixture.messages(), before);
            assert_eq!(
                caller.control(selected),
                Ok(vec![Event::RpcResolved { rpc }]),
                "a live caller must terminate its own RPC without a live peer"
            );
            assert_eq!(
                caller.pending_rpcs(),
                vec![unrelated.id],
                "only selected local ownership is removed"
            );
            assert_eq!(
                completed.get(),
                0,
                "terminal control wakes eligibility without inline execution"
            );
            assert!(received.borrow().is_none());
            assert_eq!(
                work.inventory()
                    .iter()
                    .find(|view| view.id == task)
                    .unwrap()
                    .state,
                WorkState::Runnable
            );
            assert_eq!(fixture.messages(), before);
            assert_eq!(bytes_snapshot(), original_bytes);
            assert_eq!(peer.messages(), peer_messages);
            assert_eq!(peer.pending_rpcs(), peer_rpcs);
            assert_eq!(peer_work.inventory(), peer_tasks);
            assert_eq!(replacement.messages(), replacement_messages);
            assert_eq!(replacement.pending_rpcs(), replacement_rpcs);
            if let Some(tasks) = replacement_tasks {
                assert_eq!(fixture.work(2, 2).inventory(), tasks);
            }
            assert_eq!(work.poll(task), Ok(WorkPoll::Complete));
            let expected = if $cancel {
                RpcError::Cancelled
            } else {
                RpcError::TimedOut
            };
            assert_eq!(*received.borrow(), Some(Err(expected)));
            assert_eq!(completed.get(), 1);
            for action in [TransportAction::Timeout(rpc), TransportAction::Cancel(rpc)] {
                assert_eq!(caller.control(action), Err(DriverError::InvalidToken));
                assert_eq!(caller.pending_rpcs(), vec![unrelated.id]);
                assert_eq!(fixture.messages(), before);
            }
            if let Some(reply) = late_reply {
                assert_eq!(
                    caller.control(TransportAction::Resolve { rpc, reply }),
                    Err(DriverError::InvalidToken)
                );
                assert_eq!(fixture.messages(), before);
            }
            assert_eq!(
                peer.respond(rpc, ProtocolKind::VoteResponse, original.clone()),
                Err(reply_error)
            );
            assert_eq!(completed.get(), 1);
            let continued = caller
                .emit(
                    fixture.scope(3, 1).node,
                    ProtocolKind::Other,
                    original.clone(),
                )
                .unwrap();
            caller.control(TransportAction::Release(continued)).unwrap();
            assert_eq!(foreign.take(continued).unwrap().message, original);
            assert_eq!(peer.messages(), peer_messages);
            assert_eq!(peer.pending_rpcs(), peer_rpcs);
            caller
                .control(TransportAction::Cancel(unrelated.id))
                .unwrap();
            assert!(caller.pending_rpcs().is_empty());
            let _pending_peer_response = peer_owned.response;
        }
    };
}
termination_case!(held_stopped_peer_timeout, false, false, false);
termination_case!(held_stopped_peer_cancel, false, false, true);
termination_case!(held_replaced_peer_timeout, false, true, false);
termination_case!(held_replaced_peer_cancel, false, true, true);
termination_case!(consumed_stopped_peer_timeout, true, false, false);
termination_case!(consumed_stopped_peer_cancel, true, false, true);
termination_case!(consumed_replaced_peer_timeout, true, true, false);
termination_case!(consumed_replaced_peer_cancel, true, true, true);

#[test]
fn replaced_caller_cannot_terminate_old_pending_ownership_or_wake_its_future() {
    use crate::fixture::Fixture;
    use glade_carrier_api::*;
    use std::{cell::Cell, rc::Rc};
    let fixture = Fixture::new(Carrier::OpenRaft, 111);
    let _old_inputs = fixture.inputs(1, 1);
    let mut old = fixture.endpoint(fixture.scope(1, 1));
    let pending = old
        .request(
            fixture.scope(2, 1).node,
            ProtocolKind::VoteRequest,
            ProtocolMessage {
                carrier: fixture.carrier,
                bytes: vec![4, 5],
            },
        )
        .unwrap();
    let rpc = pending.id;
    let completed = Rc::new(Cell::new(false));
    let done = completed.clone();
    let mut work = fixture.work(1, 1);
    let task = work
        .spawn(Box::pin(async move {
            let _result = pending.response.await;
            done.set(true);
        }))
        .unwrap();
    assert_eq!(work.poll(task), Ok(WorkPoll::Pending));
    let _replacement = fixture.inputs(1, 2);
    let mut current = fixture.endpoint(fixture.scope(1, 2));
    let before = fixture.messages();
    for endpoint in [&mut old, &mut current] {
        for action in [TransportAction::Timeout(rpc), TransportAction::Cancel(rpc)] {
            assert_eq!(endpoint.control(action), Err(DriverError::InvalidToken));
            assert_eq!(fixture.messages(), before);
            assert_eq!(
                fixture.endpoint(fixture.scope(1, 1)).pending_rpcs(),
                vec![rpc]
            );
        }
    }
    assert!(!completed.get());
    assert_eq!(work.poll(task), Err(SourceError::NotDue));
    assert!(!completed.get());
}

#[test]
fn live_peer_late_reply_cannot_reopen_locally_terminated_rpc() {
    use crate::fixture::Fixture;
    use glade_carrier_api::*;
    use std::{cell::RefCell, rc::Rc};
    for cancel in [false, true] {
        let fixture = Fixture::new(Carrier::OpenRaft, 112);
        let _inputs = fixture.inputs(1, 1);
        let mut caller = fixture.endpoint(fixture.scope(1, 1));
        let mut peer = fixture.endpoint(fixture.scope(2, 1));
        let message = ProtocolMessage {
            carrier: fixture.carrier,
            bytes: vec![8, 9],
        };
        let pending = caller
            .request(
                fixture.scope(2, 1).node,
                ProtocolKind::VoteRequest,
                message.clone(),
            )
            .unwrap();
        let rpc = pending.id;
        caller
            .control(TransportAction::Release(pending.message))
            .unwrap();
        peer.take(pending.message).unwrap();
        let reply = peer
            .respond(rpc, ProtocolKind::VoteResponse, message.clone())
            .unwrap();
        let received = Rc::new(RefCell::new(None));
        let result = received.clone();
        let mut work = fixture.work(1, 1);
        let task = work
            .spawn(Box::pin(async move {
                *result.borrow_mut() = Some(pending.response.await);
            }))
            .unwrap();
        assert_eq!(work.poll(task), Ok(WorkPoll::Pending));
        let before = fixture.messages();
        let action = if cancel {
            TransportAction::Cancel(rpc)
        } else {
            TransportAction::Timeout(rpc)
        };
        caller.control(action).unwrap();
        assert!(caller.pending_rpcs().is_empty());
        assert!(received.borrow().is_none());
        assert_eq!(fixture.messages(), before);
        peer.control(TransportAction::Release(reply)).unwrap();
        let before = fixture.messages();
        assert_eq!(
            caller.control(TransportAction::Resolve { rpc, reply }),
            Err(DriverError::InvalidToken)
        );
        assert_eq!(
            peer.respond(rpc, ProtocolKind::VoteResponse, message),
            Err(DriverError::InvalidToken)
        );
        assert_eq!(fixture.messages(), before);
        assert!(caller.pending_rpcs().is_empty());
        assert_eq!(work.poll(task), Ok(WorkPoll::Complete));
        assert_eq!(
            *received.borrow(),
            Some(Err(if cancel {
                RpcError::Cancelled
            } else {
                RpcError::TimedOut
            }))
        );
    }
}
