//! Regression witnesses for owned-future cleanup; no carrier implementation.
use glade_carrier_api::*;
use glade_carrier_spec::fixture::{Fixture, WorkProbe};
use std::{
    cell::{Cell, RefCell},
    future::Future,
    pin::Pin,
    rc::Rc,
    task::{Context, Poll, Waker},
};

struct Guard {
    work: WorkProbe,
    child: Option<WorkId>,
    drops: Rc<Cell<usize>>,
    inspected: Rc<Cell<bool>>,
}
impl Future for Guard {
    type Output = ();
    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
        Poll::Pending
    }
}
impl Drop for Guard {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
        if let Some(child) = self.child {
            let _result = self.work.cancel(child);
        }
        self.inspected.set(!self.work.inventory().is_empty());
    }
}
struct StopDuringPoll {
    work: WorkProbe,
    drops: Rc<Cell<usize>>,
    saved: Rc<RefCell<Option<Waker>>>,
}
impl Future for StopDuringPoll {
    type Output = ();
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        *self.saved.borrow_mut() = Some(cx.waker().clone());
        self.work.stop().unwrap();
        Poll::Pending
    }
}
impl Drop for StopDuringPoll {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
#[test]
fn own_scope_stop_during_poll_is_terminal_and_saved_wake_cannot_revive() {
    let fixture = Fixture::new(Carrier::OpenRaft, 70);
    let _inputs = fixture.inputs(1, 1);
    let mut work = fixture.work(1, 1);
    let drops = Rc::new(Cell::new(0));
    let saved = Rc::new(RefCell::new(None));
    let id = work
        .spawn(Box::pin(StopDuringPoll {
            work: work.clone(),
            drops: drops.clone(),
            saved: saved.clone(),
        }))
        .unwrap();
    let _result = work.poll(id);
    assert_eq!(
        drops.get(),
        1,
        "selected future must be dropped by own-scope stop"
    );
    assert_eq!(work.inventory()[0].state, WorkState::Cancelled);
    saved.borrow().as_ref().unwrap().wake_by_ref();
    assert_eq!(work.inventory()[0].state, WorkState::Cancelled);
    assert_eq!(work.poll(id), Err(SourceError::Stopped));
    assert_eq!(drops.get(), 1);
}
fn reentrant_cleanup(stop: bool) {
    let fixture = Fixture::new(Carrier::OpenRaft, 71);
    let _inputs = fixture.inputs(1, 1);
    let mut work = fixture.work(1, 1);
    let child_drops = Rc::new(Cell::new(0));
    let parent_drops = Rc::new(Cell::new(0));
    let inspected = Rc::new(Cell::new(false));
    let child = work
        .spawn(Box::pin(Guard {
            work: work.clone(),
            child: None,
            drops: child_drops.clone(),
            inspected: inspected.clone(),
        }))
        .unwrap();
    let parent = work
        .spawn(Box::pin(Guard {
            work: work.clone(),
            child: Some(child),
            drops: parent_drops.clone(),
            inspected: inspected.clone(),
        }))
        .unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        if stop {
            work.stop()
        } else {
            work.cancel(parent)
        }
    }));
    assert!(
        result.is_ok(),
        "future Drop may reenter scheduler after borrow is released"
    );
    result.unwrap().unwrap();
    assert!(inspected.get());
    assert_eq!((parent_drops.get(), child_drops.get()), (1, 1));
    assert!(
        work.inventory()
            .iter()
            .all(|view| view.state == WorkState::Cancelled)
    );
    assert_eq!(
        work.poll(parent),
        Err(if stop {
            SourceError::Stopped
        } else {
            SourceError::InvalidToken
        })
    );
}
#[test]
fn cancelling_parent_releases_borrow_before_reentrant_child_cleanup() {
    reentrant_cleanup(false);
}
#[test]
fn bulk_stop_releases_borrow_before_reentrant_future_cleanup() {
    reentrant_cleanup(true);
}
#[test]
fn rejected_deadline_drops_owned_future_after_releasing_borrow() {
    let fixture = Fixture::new(Carrier::OpenRaft, 72);
    let inputs = fixture.inputs(1, 1);
    let mut work = fixture.work(1, 1);
    let child_drops = Rc::new(Cell::new(0));
    let parent_drops = Rc::new(Cell::new(0));
    let inspected = Rc::new(Cell::new(false));
    let child = work
        .spawn(Box::pin(Guard {
            work: work.clone(),
            child: None,
            drops: child_drops.clone(),
            inspected: inspected.clone(),
        }))
        .unwrap();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        work.register(WorkSpec::At {
            deadline: LogicalInstant {
                domain: ClockDomain(inputs.sources.scope.domain.0 + 1),
                nanos: 0,
            },
            task: Box::pin(Guard {
                work: work.clone(),
                child: Some(child),
                drops: parent_drops.clone(),
                inspected: inspected.clone(),
            }),
        })
    }));
    assert!(
        result.is_ok(),
        "invalid registration must release borrow before owned-future Drop"
    );
    assert_eq!(result.unwrap(), Err(SourceError::WrongDomain));
    assert_eq!((parent_drops.get(), child_drops.get()), (1, 1));
    assert!(inspected.get());
    assert_eq!(work.inventory()[0].state, WorkState::Cancelled);
}
#[test]
fn completed_future_cleanup_may_inspect_and_cancel_child() {
    struct Ready(Guard);
    impl Future for Ready {
        type Output = ();
        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
            let _guard = &self.0;
            Poll::Ready(())
        }
    }
    let fixture = Fixture::new(Carrier::OpenRaft, 74);
    let _inputs = fixture.inputs(1, 1);
    let mut work = fixture.work(1, 1);
    let child_drops = Rc::new(Cell::new(0));
    let parent_drops = Rc::new(Cell::new(0));
    let inspected = Rc::new(Cell::new(false));
    let child = work
        .spawn(Box::pin(Guard {
            work: work.clone(),
            child: None,
            drops: child_drops.clone(),
            inspected: inspected.clone(),
        }))
        .unwrap();
    let parent = work
        .spawn(Box::pin(Ready(Guard {
            work: work.clone(),
            child: Some(child),
            drops: parent_drops.clone(),
            inspected: inspected.clone(),
        })))
        .unwrap();
    assert_eq!(work.poll(parent), Ok(WorkPoll::Complete));
    assert_eq!((parent_drops.get(), child_drops.get()), (1, 1));
    assert!(inspected.get());
    assert!(
        work.inventory()
            .iter()
            .all(|view| matches!(view.state, WorkState::Complete | WorkState::Cancelled))
    );
}
