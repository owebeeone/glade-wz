//! Shared executable B0 consumer scenarios, still deliberately RED.
use super::*;
// External access/state oracle: poisoning never authorizes an extra source use.
fn runtime_cut(
    carrier: Carrier,
    fault: Fault,
    before: usize,
    after: usize,
    failed: bool,
) -> Result<(), &'static str> {
    let valid = match (carrier, fault) {
        (Carrier::OpenRaft, Fault::Entropy) => after == before && !failed,
        (Carrier::RaftRs, Fault::Entropy) => after > before && failed,
        (_, Fault::Register) => (after == before && !failed) || (after > before && failed),
        (_, Fault::Clock | Fault::Task) => failed,
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err("runtime access/state footprint mismatch")
    }
}

#[test]
fn once_only_openraft_entropy_footprint_accepts_post_constructor_poison() {
    let fixture = Fixture::new(Carrier::OpenRaft, 90);
    let _inputs = fixture.inputs(1, 1);
    let mut source = fixture.source(1, 1);
    source.sample(DrawPurpose::Constructor, 10, 20).unwrap();
    let before = source.attempts();
    source.fail(Fault::Entropy);
    // EngineConfig's once-only source footprint: subsequent work uses time only.
    let mut work = fixture.work(1, 1);
    let task = work
        .spawn(Box::pin(std::future::poll_fn(|cx| {
            cx.waker().wake_by_ref();
            std::task::Poll::Pending
        })))
        .unwrap();
    for nanos in [QUANTUM_NS, 2 * QUANTUM_NS] {
        source
            .advance(LogicalInstant {
                nanos,
                ..source.clock()
            })
            .unwrap();
        source.now().unwrap();
        assert_eq!(work.poll(task), Ok(WorkPoll::Pending));
    }
    assert_eq!(
        runtime_cut(
            Carrier::OpenRaft,
            Fault::Entropy,
            before,
            source.attempts(),
            false
        ),
        Ok(())
    );
}
#[test]
fn extra_runtime_entropy_sampling_mutant_is_rejected() {
    let fixture = Fixture::new(Carrier::OpenRaft, 91);
    let _inputs = fixture.inputs(1, 1);
    let mut source = fixture.source(1, 1);
    source.sample(DrawPurpose::Constructor, 10, 20).unwrap();
    let before = source.attempts();
    source.fail(Fault::Entropy);
    let failed =
        source.sample(DrawPurpose::TimeoutReset, 10, 20) == Err(SourceError::EntropyFailed);
    assert!(
        runtime_cut(
            Carrier::OpenRaft,
            Fault::Entropy,
            before,
            source.attempts(),
            failed
        )
        .is_err()
    );
}
#[test]
fn persistent_work_does_not_require_artificial_runtime_registration() {
    for carrier in [Carrier::RaftRs, Carrier::OpenRaft] {
        let fixture = Fixture::new(carrier, 92);
        let _inputs = fixture.inputs(1, 1);
        let mut work = fixture.work(1, 1);
        let task = work
            .spawn(Box::pin(std::future::poll_fn(|cx| {
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            })))
            .unwrap();
        let before = work.registration_attempts();
        work.fail(Fault::Register);
        assert_eq!(work.poll(task), Ok(WorkPoll::Pending));
        assert_eq!(work.poll(task), Ok(WorkPoll::Pending));
        assert_eq!(
            runtime_cut(
                carrier,
                Fault::Register,
                before,
                work.registration_attempts(),
                false
            ),
            Ok(())
        );
        assert!(runtime_cut(carrier, Fault::Register, before, before + 1, false).is_err());
        assert_eq!(
            work.spawn(Box::pin(std::future::pending())),
            Err(SourceError::RegistrationFailed)
        );
        assert_eq!(
            runtime_cut(
                carrier,
                Fault::Register,
                before,
                work.registration_attempts(),
                true
            ),
            Ok(())
        );
    }
}
pub fn constructor_and_reset_use_supplied_streams(provider: &impl Provider) {
    let mut cluster = Cluster::new(provider, false);
    // This reaches the source-use invariant on refusing constructors, not a tool error.
    for node in 1..=3 {
        oracle::draws(
            &cluster.fixture.trace(),
            cluster.fixture.scope(node, 1),
            cluster.fixture.range(),
        )
        .unwrap_or_else(|error| {
            panic!(
                "B0-02 constructor supplied-stream assertion: {error}; provider={:?}",
                cluster.nodes[node as usize - 1].observe()
            )
        });
    }
    // Change only node 2's explicitly supplied constructor stream. No delivery
    // has occurred, so another node's constructor source footprint must agree.
    let startup = cluster.fixture.trace();
    let constructor_attempts: Vec<_> = (1..=3)
        .map(|node| cluster.fixture.source(node, 1).attempts())
        .collect();
    let changed = Fixture::new(provider.carrier(), 1);
    let _changed_nodes: Vec<_> = (1..=3)
        .map(|node| {
            let inputs = changed.inputs(node, 1);
            if node == 2 {
                changed
                    .source(node, 1)
                    .script(vec![changed.range().lower + 5; 30_000]);
            }
            provider
                .construct(inputs)
                .expect("separately supplied changed source constructor")
        })
        .collect();
    for node in [1, 3] {
        assert_eq!(
            oracle::draws(
                &startup,
                cluster.fixture.scope(node, 1),
                cluster.fixture.range()
            )
            .unwrap(),
            oracle::draws(&changed.trace(), changed.scope(node, 1), changed.range()).unwrap(),
            "B0-02 unchanged constructor stream footprint"
        );
    }
    assert_ne!(
        oracle::draws(
            &startup,
            cluster.fixture.scope(2, 1),
            cluster.fixture.range()
        )
        .unwrap(),
        oracle::draws(&changed.trace(), changed.scope(2, 1), changed.range()).unwrap(),
        "B0-02 own supplied script must distinguish construction"
    );
    let leader = cluster.fair_leader("B0-02 automatic transitions");
    cluster.isolated[leader.node as usize - 1] = true;
    for _ in 0..100 {
        cluster.round("B0-02 automatic reset transitions");
    }
    for node in 1..=3 {
        oracle::draws(
            &cluster.fixture.trace(),
            cluster.fixture.scope(node, 1),
            cluster.fixture.range(),
        )
        .unwrap();
    }
    match provider.carrier() {
        Carrier::OpenRaft => {
            for node in 1..=3 {
                runtime_cut(
                    Carrier::OpenRaft,
                    Fault::Entropy,
                    constructor_attempts[node as usize - 1],
                    cluster.fixture.source(node, 1).attempts(),
                    false,
                )
                .expect("B0-02 OpenRaft constructor-only entropy footprint");
            }
        }
        Carrier::RaftRs => {
            assert!(
                cluster.fixture.source(leader.node, 1).attempts()
                    > constructor_attempts[leader.node as usize - 1],
                "B0-02 isolated check-quorum leader must reach documented reset draw"
            );
        }
    }
    // Separately supplied equal scripts remain legal, never selected statically.
    let equal = Cluster::new(provider, true);
    let first = oracle::draws(
        &equal.fixture.trace(),
        equal.fixture.scope(1, 1),
        equal.fixture.range(),
    )
    .unwrap();
    let second = oracle::draws(
        &equal.fixture.trace(),
        equal.fixture.scope(2, 1),
        equal.fixture.range(),
    )
    .unwrap();
    assert_eq!(first, second);
}

pub fn source_failure_has_no_ambient_fallback(provider: &impl Provider) {
    for (fault, expected) in [
        (Fault::Entropy, SourceError::EntropyFailed),
        (Fault::Exhausted, SourceError::ScriptExhausted),
        (Fault::OutOfRange, SourceError::OutOfRange),
        (Fault::Clock, SourceError::ClockFailed),
        (Fault::Register, SourceError::RegistrationFailed),
    ] {
        let fixture = Fixture::new(provider.carrier(), 2);
        let inputs = fixture.inputs(1, 1);
        match fault {
            Fault::Register => fixture.work(1, 1).fail(fault),
            _ => fixture.source(1, 1).fail(fault),
        }
        match provider.construct(inputs) {
            Err(error) => assert_eq!(
                error,
                DriverError::Source(expected),
                "B0-03 precise constructor source failure"
            ),
            Ok(node) => panic!(
                "B0-03 precise constructor source failure {expected:?} required; scaffold observation={:?}",
                node.observe()
            ),
        }
        assert!(
            fixture
                .work(1, 1)
                .inventory()
                .iter()
                .all(|work| matches!(work.state, WorkState::Cancelled | WorkState::Complete)),
            "B0-03 no construction orphan"
        );
        assert!(
            fixture.messages().is_empty(),
            "B0-03 no participation after constructor failure"
        );
    }
    for (fault, expected) in [
        (Fault::Entropy, SourceError::EntropyFailed),
        (Fault::Clock, SourceError::ClockFailed),
        (Fault::Register, SourceError::RegistrationFailed),
        (Fault::Task, SourceError::TaskFailed),
    ] {
        let mut cluster = Cluster::new(provider, false);
        let constructor_attempts: Vec<_> = (1..=3)
            .map(|node| cluster.fixture.source(node, 1).attempts())
            .collect();
        let leader = cluster.fair_leader("B0-03 establish runtime before injected failure");
        let selected = if matches!(fault, Fault::Entropy) && provider.carrier() == Carrier::RaftRs {
            // check_quorum leader loses all inbound quorum activity; its documented
            // MsgCheckQuorum -> become_follower -> reset path samples again. An
            // isolated follower may only pre-campaign, which does not reset.
            leader.node
        } else {
            1
        };
        let index = selected as usize - 1;
        let before = match fault {
            Fault::Register => cluster.fixture.work(selected, 1).registration_attempts(),
            Fault::Entropy if provider.carrier() == Carrier::OpenRaft => {
                constructor_attempts[index]
            }
            _ => cluster.fixture.source(selected, 1).attempts(),
        };
        match fault {
            Fault::Register | Fault::Task => cluster.fixture.work(selected, 1).fail(fault),
            _ => cluster.fixture.source(selected, 1).fail(fault),
        }
        // Only this endpoint's owned clock/work advances; no inbound quorum
        // responses are delivered during the controlled cut. No eager probes,
        // campaigns or artificial registrations are supplied by the consumer.
        let mut failed = false;
        for _ in 0..100 {
            let now = cluster.fixture.source(selected, 1).clock();
            match cluster.nodes[index].advance(LogicalInstant {
                nanos: now.nanos.checked_add(QUANTUM_NS).unwrap(),
                ..now
            }) {
                Ok(_) => {}
                Err(error) => {
                    assert_eq!(
                        error,
                        DriverError::Source(expected),
                        "B0-03 unexpected runtime advance error"
                    );
                    failed = true;
                }
            }
            if failed {
                break;
            }
            let inventory = cluster.nodes[index]
                .inventory()
                .expect("B0-03 runtime inventory");
            for work in inventory.work {
                if work.state == WorkState::Runnable {
                    match cluster.nodes[index].drive(work.id) {
                        Ok(_) => {}
                        Err(error) => {
                            assert_eq!(
                                error,
                                DriverError::Source(expected),
                                "B0-03 unexpected runtime drive error"
                            );
                            failed = true;
                            break;
                        }
                    }
                }
            }
            if failed {
                break;
            }
        }
        let after = match fault {
            Fault::Register => cluster.fixture.work(selected, 1).registration_attempts(),
            _ => cluster.fixture.source(selected, 1).attempts(),
        };
        runtime_cut(provider.carrier(), fault, before, after, failed)
            .expect("B0-03 carrier-specific runtime access/state oracle");
        let observation = cluster.nodes[index].observe().unwrap();
        if failed {
            assert_eq!(observation.state, NodeState::Failed(expected));
            let inventory = cluster.nodes[index].inventory().unwrap();
            assert!(inventory.pending_rpcs.is_empty());
            assert!(
                inventory
                    .work
                    .iter()
                    .all(|work| matches!(work.state, WorkState::Cancelled | WorkState::Complete))
            );
        } else {
            assert_eq!(
                observation.state,
                NodeState::Running,
                "B0-03 untouched source cut remains healthy"
            );
        }
    }
}
