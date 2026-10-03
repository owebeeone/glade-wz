//! Carrier-free actual-future scheduling chain; no engine/role/leader surrogate.
macro_rules! chain_case {
    ($name:ident, $early:expr, $inline:expr) => {
        #[test]
        fn $name() {
            use crate::fixture::{Fixture, QUANTUM_NS};
            use glade_carrier_api::*;
            use std::{
                cell::{Cell, RefCell},
                rc::Rc,
                task::{Poll, Waker},
            };
            let fixture = Fixture::new(Carrier::OpenRaft, 120);
            let inputs = fixture.inputs(1, 1);
            let mut source = fixture.source(1, 1);
            let mut work = fixture.work(1, 1);
            let current = Rc::new(Cell::new(0));
            let action = Rc::new(Cell::new(0));
            let records = Rc::new(RefCell::new(Vec::<(&str, u64, usize)>::new()));
            let ready = Rc::new(Cell::new(false));
            let wake = Rc::new(RefCell::new(None::<Waker>));
            let mut core_work = work.clone();
            let core_ready = ready.clone();
            let core_wake = wake.clone();
            let core_records = records.clone();
            let core_time = current.clone();
            let core_action = action.clone();
            let core = work
                .spawn(Box::pin(std::future::poll_fn(move |cx| {
                    if !core_ready.get() {
                        *core_wake.borrow_mut() = Some(cx.waker().clone());
                        core_records.borrow_mut().push((
                            "startup",
                            core_time.get(),
                            core_action.get(),
                        ));
                        return Poll::Pending;
                    }
                    core_records.borrow_mut().push((
                        "eligible",
                        core_time.get(),
                        core_action.get(),
                    ));
                    let send_records = core_records.clone();
                    let send_time = core_time.clone();
                    let send_action = core_action.clone();
                    let mut send = Box::pin(std::future::poll_fn(move |_| {
                        send_records.borrow_mut().push((
                            "send",
                            send_time.get(),
                            send_action.get(),
                        ));
                        Poll::Ready(())
                    }));
                    if $inline {
                        // Deliberately invalid test mutant: polls newly owned work
                        // inside the parent's selected poll, before registration.
                        let _result = std::future::Future::poll(send.as_mut(), cx);
                    }
                    core_work.spawn(send).unwrap();
                    Poll::Ready(())
                })))
                .unwrap();
            let tick_records = records.clone();
            let tick_time = current.clone();
            let tick_action = action.clone();
            let ticker = work
                .register(WorkSpec::At {
                    deadline: LogicalInstant {
                        domain: inputs.sources.scope.domain,
                        nanos: if $early { QUANTUM_NS } else { 2 * QUANTUM_NS },
                    },
                    task: Box::pin(async move {
                        tick_records.borrow_mut().push((
                            "tick",
                            tick_time.get(),
                            tick_action.get(),
                        ));
                        ready.set(true);
                        wake.borrow().as_ref().unwrap().wake_by_ref();
                    }),
                })
                .unwrap();
            assert!(records.borrow().is_empty(), "registration is inert");
            let select = |work: &mut crate::fixture::WorkProbe| {
                super::drain_point(|selected| {
                    if let Some(id) = selected {
                        action.set(action.get() + 1);
                        work.poll(id)?;
                    }
                    assert_eq!(
                        fixture.source(1, 1).clock().nanos,
                        current.get(),
                        "fixed-instant drain cannot advance time"
                    );
                    assert!(
                        fixture.messages().is_empty(),
                        "chain never delivers protocol messages"
                    );
                    Ok(work.inventory())
                })
                .unwrap()
            };
            let startup = select(&mut work);
            assert_eq!(startup, vec![core]);
            let invalid = || {
                let records = records.borrow();
                let eligible = records.iter().find(|record| record.0 == "eligible");
                eligible.is_some_and(|eligible| {
                    eligible.1 < 2 * QUANTUM_NS
                        || records
                            .iter()
                            .any(|send| send.0 == "send" && send.2 <= eligible.2)
                })
            };
            let mut at = Vec::new();
            for nanos in [QUANTUM_NS, 2 * QUANTUM_NS] {
                let instant = LogicalInstant {
                    domain: inputs.sources.scope.domain,
                    nanos,
                };
                source.advance(instant).unwrap();
                work.advance(instant).unwrap();
                current.set(nanos);
                let inert = records.borrow().clone();
                assert_eq!(*records.borrow(), inert, "advance/wake does not poll");
                at = select(&mut work);
                if nanos == QUANTUM_NS {
                    if $early {
                        assert!(invalid(), "early eligibility mutant must be rejected");
                        return;
                    }
                    assert!(
                        !records
                            .borrow()
                            .iter()
                            .any(|record| matches!(record.0, "eligible" | "send"))
                    );
                }
            }
            if $inline {
                assert!(invalid(), "inline-send mutant must be rejected");
            } else {
                assert!(!invalid());
                assert_eq!(
                    records
                        .borrow()
                        .iter()
                        .map(|record| record.0)
                        .collect::<Vec<_>>(),
                    vec!["startup", "tick", "eligible", "send"],
                    "refresh must select ticker, awakened core, newly spawned send"
                );
                assert_eq!(at.len(), 3);
                assert_eq!(at[0..2], [ticker, core]);
                assert_ne!(at[2], core);
                assert_ne!(at[2], ticker);
                assert_eq!(action.get(), 4, "each selected action polls exactly once");
            }
        }
    };
}
chain_case!(
    current_inventory_reaches_ticker_core_and_new_send_future,
    false,
    false
);
chain_case!(early_eligibility_mutant_is_rejected, true, false);
chain_case!(inline_send_poll_mutant_is_rejected, false, true);

#[test]
fn perpetually_rewoken_work_hits_the_explicit_point_action_bound() {
    use crate::fixture::Fixture;
    use glade_carrier_api::*;
    let fixture = Fixture::new(Carrier::OpenRaft, 121);
    let _inputs = fixture.inputs(1, 1);
    let mut work = fixture.work(1, 1);
    work.spawn(Box::pin(std::future::poll_fn(|cx| {
        cx.waker().wake_by_ref();
        std::task::Poll::Pending
    })))
    .unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        super::drain_point(|selected| {
            if let Some(id) = selected {
                work.poll(id)?;
            }
            Ok(work.inventory())
        })
    }));
    assert!(
        result.is_err(),
        "nonquiescent point must fail at an explicit action bound"
    );
}

#[test]
fn selected_host_ticks_require_actual_progress_between_logical_points() {
    use crate::fixture::{Fixture, QUANTUM_NS, WorkProbe};
    use glade_carrier_api::*;
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };
    fn tick(
        mut work: WorkProbe,
        now: Rc<Cell<u64>>,
        times: Rc<RefCell<Vec<u64>>>,
        domain: ClockDomain,
    ) -> OwnedTask {
        Box::pin(async move {
            times.borrow_mut().push(now.get());
            let next = tick(work.clone(), now.clone(), times, domain);
            work.register(WorkSpec::At {
                deadline: LogicalInstant {
                    domain,
                    nanos: now.get() + QUANTUM_NS,
                },
                task: next,
            })
            .unwrap();
        })
    }
    let fixture = Fixture::new(Carrier::RaftRs, 122);
    let inputs = fixture.inputs(1, 1);
    let mut source = fixture.source(1, 1);
    let mut work = fixture.work(1, 1);
    let now = Rc::new(Cell::new(0));
    let times = Rc::new(RefCell::new(Vec::new()));
    work.register(WorkSpec::At {
        deadline: LogicalInstant {
            domain: inputs.sources.scope.domain,
            nanos: QUANTUM_NS,
        },
        task: tick(
            work.clone(),
            now.clone(),
            times.clone(),
            inputs.sources.scope.domain,
        ),
    })
    .unwrap();
    for point in super::time_points(
        source.clock(),
        LogicalInstant {
            domain: inputs.sources.scope.domain,
            nanos: 4 * QUANTUM_NS,
        },
    ) {
        source.advance(point).unwrap();
        work.advance(point).unwrap();
        now.set(point.nanos);
        super::drain_point(|selected| {
            if let Some(id) = selected {
                work.poll(id)?;
            }
            Ok(work.inventory())
        })
        .unwrap();
    }
    assert_eq!(
        *times.borrow(),
        vec![QUANTUM_NS, 2 * QUANTUM_NS, 3 * QUANTUM_NS, 4 * QUANTUM_NS],
        "a jump grants one resumed tick, never four elapsed host ticks"
    );
}

#[test]
fn partial_boundary_points_are_exact_bounded_and_domain_checked() {
    use crate::fixture::QUANTUM_NS;
    use glade_carrier_api::*;
    let from = LogicalInstant {
        domain: ClockDomain(7),
        nanos: QUANTUM_NS + 1,
    };
    let to = LogicalInstant {
        nanos: 3 * QUANTUM_NS + 5,
        ..from
    };
    assert_eq!(
        super::time_points(from, to)
            .iter()
            .map(|point| point.nanos)
            .collect::<Vec<_>>(),
        vec![2 * QUANTUM_NS, 3 * QUANTUM_NS, 3 * QUANTUM_NS + 5]
    );
    assert!(super::time_points(from, from).is_empty());
    for invalid in [
        LogicalInstant {
            domain: ClockDomain(8),
            ..to
        },
        LogicalInstant { nanos: 0, ..from },
        LogicalInstant {
            nanos: 5000 * QUANTUM_NS,
            ..from
        },
    ] {
        assert!(std::panic::catch_unwind(|| super::time_points(from, invalid)).is_err());
    }
}
