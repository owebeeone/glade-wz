//! Clock identity regressions; no process allocator or new public API.
#[test]
fn node_one_incarnation_eleven_and_node_two_incarnation_one_are_distinct() {
    use glade_carrier_api::*;
    for carrier in [Carrier::RaftRs, Carrier::OpenRaft] {
        let fixture = super::Fixture::new(carrier, 1);
        let first = fixture.inputs(1, 11);
        let second = fixture.inputs(2, 1);
        assert_ne!(first.sources.scope.domain, second.sources.scope.domain);
    }
}
#[test]
fn colliding_counterexample_instants_are_foreign_to_both_owned_sources() {
    use glade_carrier_api::*;
    let fixture = super::Fixture::new(Carrier::OpenRaft, 1);
    let _first = fixture.inputs(1, 11);
    let _second = fixture.inputs(2, 1);
    for (node, incarnation, other, other_incarnation) in [(1, 11, 2, 1), (2, 1, 1, 11)] {
        let mut source = fixture.source(node, incarnation);
        let before = source.clock();
        let trace = fixture.trace();
        let foreign = LogicalInstant {
            nanos: 1,
            ..fixture.source(other, other_incarnation).clock()
        };
        assert_eq!(source.advance(foreign), Err(SourceError::WrongDomain));
        assert_eq!(source.clock(), before);
        assert_eq!(fixture.trace(), trace);
    }
}
#[test]
fn colliding_counterexample_instants_cannot_enable_either_owned_scheduler() {
    use glade_carrier_api::*;
    let fixture = super::Fixture::new(Carrier::OpenRaft, 1);
    let _first = fixture.inputs(1, 11);
    let _second = fixture.inputs(2, 1);
    for (node, incarnation, other, other_incarnation) in [(1, 11, 2, 1), (2, 1, 1, 11)] {
        let mut work = fixture.work(node, incarnation);
        let own = fixture.source(node, incarnation).clock();
        work.register(WorkSpec::At {
            deadline: LogicalInstant { nanos: 1, ..own },
            task: Box::pin(std::future::pending()),
        })
        .unwrap();
        let before = work.inventory();
        let foreign = LogicalInstant {
            nanos: 1,
            ..fixture.source(other, other_incarnation).clock()
        };
        assert_eq!(work.advance(foreign), Err(SourceError::WrongDomain));
        assert_eq!(work.inventory(), before);
        work.advance(own).unwrap();
    }
}
#[test]
fn colliding_counterexample_deadlines_refuse_without_registration() {
    use glade_carrier_api::*;
    let fixture = super::Fixture::new(Carrier::OpenRaft, 1);
    let _first = fixture.inputs(1, 11);
    let _second = fixture.inputs(2, 1);
    for (node, incarnation, other, other_incarnation) in [(1, 11, 2, 1), (2, 1, 1, 11)] {
        let mut work = fixture.work(node, incarnation);
        let before = work.inventory();
        let deadline = LogicalInstant {
            nanos: 1,
            ..fixture.source(other, other_incarnation).clock()
        };
        assert_eq!(
            work.register(WorkSpec::At {
                deadline,
                task: Box::pin(std::future::pending())
            }),
            Err(SourceError::WrongDomain)
        );
        assert_eq!(work.inventory(), before);
    }
}
#[test]
fn domain_encoding_neighbors_limits_carriers_sessions_and_replay_are_distinct() {
    use glade_carrier_api::*;
    use std::collections::BTreeSet;
    let mut issued = BTreeSet::new();
    for carrier in [Carrier::RaftRs, Carrier::OpenRaft] {
        for session in [0, 1, (1 << 31) - 1] {
            let fixture = super::Fixture::new(carrier, session);
            let replay = super::Fixture::new(carrier, session);
            for node in 1..=3 {
                for incarnation in [0, 1, 10, 11, 12, (1 << 30) - 1] {
                    let domain = fixture.scope(node, incarnation).domain;
                    assert!(issued.insert(domain), "bounded coordinates must not alias");
                    assert_eq!(domain, replay.scope(node, incarnation).domain);
                }
            }
        }
    }
}
#[test]
fn invalid_encoding_coordinates_refuse_before_source_or_work_allocation() {
    use glade_carrier_api::*;
    assert!(std::panic::catch_unwind(|| super::Fixture::new(Carrier::OpenRaft, 1 << 31)).is_err());
    for (node, incarnation) in [(0, 1), (4, 1), (1, 0), (1, 1 << 30)] {
        let fixture = super::Fixture::new(Carrier::OpenRaft, 1);
        let before = fixture.messages();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            fixture.inputs(node, incarnation)
        }));
        assert!(
            result.is_err(),
            "invalid fixture coordinates must refuse before assembly"
        );
        assert!(fixture.probes.borrow().is_empty());
        assert_eq!(fixture.messages(), before);
        assert!(fixture.trace().is_empty());
    }
}

#[test]
fn maximum_issued_coordinates_construct_and_drive_owned_sources_and_work() {
    use glade_carrier_api::*;
    for carrier in [Carrier::RaftRs, Carrier::OpenRaft] {
        let fixture = super::Fixture::new(carrier, (1 << 31) - 1);
        let inputs = fixture.inputs(3, (1 << 30) - 1);
        assert_eq!(
            inputs.sources.scope.domain,
            ClockDomain(if carrier == Carrier::OpenRaft {
                u64::MAX
            } else {
                (1 << 63) - 1
            })
        );
        let mut source = fixture.source(3, (1 << 30) - 1);
        let mut work = fixture.work(3, (1 << 30) - 1);
        let to = LogicalInstant {
            domain: inputs.sources.scope.domain,
            nanos: 1,
        };
        let task = work
            .register(WorkSpec::At {
                deadline: to,
                task: Box::pin(async {}),
            })
            .unwrap();
        assert_eq!(work.poll(task), Err(SourceError::NotDue));
        source.advance(to).unwrap();
        work.advance(to).unwrap();
        assert_eq!(work.poll(task), Ok(WorkPoll::Complete));
        assert_eq!(source.clock(), to);
    }
}
