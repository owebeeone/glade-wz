//! Shared executable B0 consumer scenarios, still deliberately RED.
use super::*;
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
        cluster.fair_leader("B0-03 establish runtime before injected failure");
        match fault {
            Fault::Register | Fault::Task => cluster.fixture.work(1, 1).fail(fault),
            _ => cluster.fixture.source(1, 1).fail(fault),
        }
        // Faults are observed at their next actual source/task use, not eagerly.
        let mut failed = false;
        for _ in 0..100 {
            let now = cluster.fixture.source(1, 1).clock();
            let result = cluster.nodes[0].advance(LogicalInstant {
                nanos: now.nanos + QUANTUM_NS,
                ..now
            });
            if result == Err(DriverError::Source(expected)) {
                failed = true;
                break;
            }
            if let Ok(inventory) = cluster.nodes[0].inventory() {
                for work in inventory.work {
                    if work.state == WorkState::Runnable
                        && cluster.nodes[0].drive(work.id) == Err(DriverError::Source(expected))
                    {
                        failed = true;
                        break;
                    }
                }
            }
            if failed {
                break;
            }
        }
        assert!(failed, "B0-03 exact runtime source failure must surface");
        assert_eq!(
            cluster.nodes[0].observe().unwrap().state,
            NodeState::Failed(expected)
        );
        assert!(
            cluster.nodes[0]
                .inventory()
                .unwrap()
                .pending_rpcs
                .is_empty()
        );
    }
}
