//! Fixture machinery only; no adapted carrier or service is qualified here.
use glade_carrier_api::*;
use glade_carrier_spec::fixture::{Fixture, QUANTUM_NS};
use std::cell::Cell;
use std::rc::Rc;
#[test]
fn register_and_spawn_own_actual_futures_without_inline_polling() {
    let fixture = Fixture::new(Carrier::OpenRaft, 20);
    let inputs = fixture.inputs(1, 1);
    let mut work = fixture.work(1, 1);
    let completed = Rc::new(Cell::new(0));
    let result = completed.clone();
    let deadline = LogicalInstant {
        domain: inputs.sources.scope.domain,
        nanos: QUANTUM_NS,
    };
    let id = work
        .register(WorkSpec::At {
            deadline,
            task: Box::pin(async move {
                result.set(1);
            }),
        })
        .unwrap();
    assert_eq!(completed.get(), 0);
    assert_eq!(work.poll(id), Err(SourceError::NotDue));
    work.advance(deadline).unwrap();
    assert_eq!(completed.get(), 0);
    assert_eq!(work.poll(id), Ok(WorkPoll::Complete));
    assert_eq!(completed.get(), 1);
    assert_eq!(work.poll(id), Err(SourceError::InvalidToken));
    let result = completed.clone();
    let cancelled = work
        .spawn(Box::pin(async move {
            result.set(2);
        }))
        .unwrap();
    work.cancel(cancelled).unwrap();
    assert_eq!(work.poll(cancelled), Err(SourceError::InvalidToken));
    assert_eq!(completed.get(), 1);
}
#[test]
fn pending_rpc_waits_for_explicit_resolution_and_wake_requires_selected_poll() {
    let fixture = Fixture::new(Carrier::OpenRaft, 21);
    let _inputs = fixture.inputs(1, 1);
    let mut endpoint = fixture.endpoint(fixture.scope(1, 1));
    let mut peer = fixture.endpoint(fixture.scope(2, 1));
    let mut work = fixture.work(1, 1);
    let rpc = endpoint
        .request(
            fixture.scope(2, 1).node,
            ProtocolKind::VoteRequest,
            ProtocolMessage {
                carrier: Carrier::OpenRaft,
                bytes: vec![1],
            },
        )
        .unwrap();
    let id = rpc.id;
    let request = rpc.message;
    let completed = Rc::new(Cell::new(false));
    let result = completed.clone();
    let task = work
        .spawn(Box::pin(async move {
            assert_eq!(rpc.response.await.unwrap().bytes, vec![2]);
            result.set(true);
        }))
        .unwrap();
    assert_eq!(work.poll(task), Ok(WorkPoll::Pending));
    assert!(!completed.get());
    endpoint.control(TransportAction::Release(request)).unwrap();
    peer.take(request).unwrap();
    assert_eq!(endpoint.pending_rpcs(), vec![id]);
    let reply = peer
        .respond(
            id,
            ProtocolKind::VoteResponse,
            ProtocolMessage {
                carrier: Carrier::OpenRaft,
                bytes: vec![2],
            },
        )
        .unwrap();
    peer.control(TransportAction::Release(reply)).unwrap();
    endpoint
        .control(TransportAction::Resolve { rpc: id, reply })
        .unwrap();
    assert!(!completed.get());
    assert_eq!(work.poll(task), Ok(WorkPoll::Complete));
    assert!(completed.get());
    assert_eq!(
        endpoint.control(TransportAction::Timeout(id)),
        Err(DriverError::InvalidToken)
    );
}
#[test]
fn dropping_message_keeps_rpc_pending_until_explicit_timeout_then_stop_cancels_work() {
    let fixture = Fixture::new(Carrier::OpenRaft, 22);
    let _inputs = fixture.inputs(1, 1);
    let mut endpoint = fixture.endpoint(fixture.scope(1, 1));
    let mut work = fixture.work(1, 1);
    let rpc = endpoint
        .request(
            fixture.scope(2, 1).node,
            ProtocolKind::AppendRequest,
            ProtocolMessage {
                carrier: Carrier::OpenRaft,
                bytes: vec![7],
            },
        )
        .unwrap();
    let id = rpc.id;
    endpoint
        .control(TransportAction::Drop(rpc.message))
        .unwrap();
    assert_eq!(endpoint.pending_rpcs(), vec![id]);
    let task = work
        .spawn(Box::pin(async move {
            assert_eq!(rpc.response.await, Err(RpcError::TimedOut));
        }))
        .unwrap();
    assert_eq!(work.poll(task), Ok(WorkPoll::Pending));
    endpoint.control(TransportAction::Timeout(id)).unwrap();
    assert_eq!(work.poll(task), Ok(WorkPoll::Complete));
    let pending = work.spawn(Box::pin(std::future::pending())).unwrap();
    work.stop().unwrap();
    assert_eq!(work.poll(pending), Err(SourceError::Stopped));
    assert!(
        work.inventory()
            .iter()
            .all(|work| matches!(work.state, WorkState::Complete | WorkState::Cancelled))
    );
}
