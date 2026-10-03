//! Shared executable B0 consumer scenarios, still deliberately RED.
use super::*;
pub fn paused_leader_automatic_failover_and_resume(provider: &impl Provider) {
    let mut cluster = Cluster::new(provider, false);
    let old = cluster.fair_leader("B0-08 first automatic leader");
    cluster.paused[old.node as usize - 1] = true;
    let successor = cluster.fair_leader("B0-08 paused leader successor");
    assert_ne!(old, successor);
    let before = cluster.nodes[old.node as usize - 1].inventory().unwrap();
    assert!(before.clock.nanos > 0);
    assert_eq!(
        before.timing.resume,
        match provider.carrier() {
            Carrier::RaftRs => ResumePolicy::HostSingleTick,
            Carrier::OpenRaft => ResumePolicy::PinnedTickerSingleWake,
        }
    );
    cluster.paused[old.node as usize - 1] = false;
    for _ in 0..100 {
        cluster.round("B0-08 resume without reset");
    }
    let leader = cluster.fair_leader("B0-08 converged leader");
    assert_eq!(
        cluster.nodes[old.node as usize - 1]
            .observe()
            .unwrap()
            .leader,
        Some(leader)
    );
}

pub fn independent_clock_schedules(provider: &impl Provider) {
    fn run(provider: &impl Provider) -> Vec<Event> {
        let mut cluster = Cluster::new(provider, false);
        let baseline = cluster.views("B0-09 independent initial state");
        for index in 0..3 {
            let unchanged: Vec<_> = (0..3)
                .filter(|other| *other != index)
                .map(|other| {
                    (
                        other,
                        cluster.fixture.source(other as u64 + 1, 1).clock(),
                        cluster.nodes[other].inventory().unwrap(),
                    )
                })
                .collect();
            let now = cluster.fixture.source(index as u64 + 1, 1).clock();
            cluster.nodes[index]
                .advance(LogicalInstant {
                    nanos: now.nanos + 25 * QUANTUM_NS,
                    ..now
                })
                .unwrap();
            for work in cluster.nodes[index].inventory().unwrap().work {
                if work.state == WorkState::Runnable {
                    cluster.nodes[index].drive(work.id).unwrap();
                }
            }
            for (other, clock, inventory) in unchanged {
                assert_eq!(cluster.fixture.source(other as u64 + 1, 1).clock(), clock);
                assert_eq!(
                    cluster.nodes[other].inventory().unwrap().work,
                    inventory.work
                );
            }
        }
        assert_eq!(baseline.len(), 3);
        cluster.fixture.trace()
    }
    assert_eq!(
        run(provider),
        run(provider),
        "B0-09 exact explicit schedule trace replay within one carrier"
    );
}

pub fn invalid_controls_and_stop_lifecycle(provider: &impl Provider) {
    let mut cluster = Cluster::new(provider, false);
    let scope = cluster.fixture.scope(1, 1);
    let now = LogicalInstant {
        domain: scope.domain,
        nanos: 10 * QUANTUM_NS,
    };
    assert_eq!(
        cluster.nodes[0].advance(now),
        Ok(()),
        "B0-10 valid time before invalid controls"
    );
    let before = cluster.nodes[0].inventory().unwrap();
    let view = cluster.nodes[0].observe().unwrap();
    for (to, error) in [
        (
            LogicalInstant {
                nanos: now.nanos - 1,
                ..now
            },
            SourceError::BackwardTime,
        ),
        (
            LogicalInstant {
                domain: ClockDomain(999),
                ..now
            },
            SourceError::WrongDomain,
        ),
    ] {
        assert_eq!(
            cluster.nodes[0].advance(to),
            Err(DriverError::Source(error))
        );
        assert_eq!(cluster.nodes[0].inventory().unwrap(), before);
        assert_eq!(cluster.nodes[0].observe().unwrap(), view);
    }
    // Guaranteed checked-add overflow, with no carrier-specific MAX policy.
    assert_eq!(
        cluster.nodes[0].advance_by(u64::MAX),
        Err(DriverError::Source(SourceError::Overflow))
    );
    assert_eq!(cluster.nodes[0].inventory().unwrap(), before);
    let foreign = WorkId {
        scope: cluster.fixture.scope(2, 1),
        sequence: 1,
    };
    let stale = WorkId {
        scope: NodeScope {
            incarnation: 0,
            ..scope
        },
        sequence: 1,
    };
    for work in [foreign, stale] {
        assert_eq!(cluster.nodes[0].drive(work), Err(DriverError::InvalidToken));
        assert_eq!(cluster.nodes[0].inventory().unwrap(), before);
    }
    let message = MessageToken {
        issuer: scope,
        destination: cluster.fixture.scope(2, 1),
        sequence: 999,
    };
    assert_eq!(
        cluster.nodes[0].receive(message),
        Err(DriverError::InvalidToken)
    );
    assert_eq!(cluster.nodes[0].inventory().unwrap(), before);
    // Consume actual engine work and message tokens, then prove replay refusal.
    cluster.fair_leader("B0-10 actual consumed-token setup");
    let completed = cluster
        .fixture
        .work(1, 1)
        .spawn(Box::pin(async {}))
        .unwrap();
    cluster.nodes[0].drive(completed).unwrap();
    let consumed_before = cluster.nodes[0].inventory().unwrap();
    assert_eq!(
        cluster.nodes[0].drive(completed),
        Err(DriverError::InvalidToken)
    );
    assert_eq!(cluster.nodes[0].inventory().unwrap(), consumed_before);
    let consumed = cluster
        .fixture
        .messages()
        .into_iter()
        .find(|message| {
            message.state == MessageState::Consumed && message.token.destination == scope
        })
        .expect("actual delivered engine message")
        .token;
    assert_eq!(
        cluster.nodes[0].receive(consumed),
        Err(DriverError::InvalidToken)
    );
    assert_eq!(cluster.nodes[0].inventory().unwrap(), consumed_before);
    // Inject real pending fixture RPC future into this explicitly supplied scope.
    let mut endpoint = cluster.fixture.endpoint(scope);
    let pending = endpoint
        .request(
            cluster.fixture.scope(2, 1).node,
            ProtocolKind::Other,
            ProtocolMessage {
                carrier: provider.carrier(),
                bytes: vec![0],
            },
        )
        .unwrap();
    let waiting = cluster
        .fixture
        .work(1, 1)
        .spawn(Box::pin(async move {
            let _result = pending.response.await;
        }))
        .unwrap();
    cluster.nodes[0].drive(waiting).unwrap();
    assert!(
        !cluster.nodes[0]
            .inventory()
            .unwrap()
            .pending_rpcs
            .is_empty()
    );
    cluster.nodes[0].stop().unwrap();
    let stopped = cluster.nodes[0].inventory().unwrap();
    assert!(stopped.pending_rpcs.is_empty());
    assert!(
        stopped
            .work
            .iter()
            .all(|work| matches!(work.state, WorkState::Cancelled | WorkState::Complete))
    );
    assert_eq!(cluster.nodes[0].drive(stale), Err(DriverError::Stopped));
    assert_eq!(cluster.nodes[0].receive(message), Err(DriverError::Stopped));
    let mut replacement = provider.construct(cluster.fixture.inputs(1, 2)).unwrap();
    let replacement_before = replacement.inventory().unwrap();
    assert_eq!(
        replacement.drive(WorkId { scope, sequence: 1 }),
        Err(DriverError::InvalidToken)
    );
    assert_eq!(replacement.inventory().unwrap(), replacement_before);
}
