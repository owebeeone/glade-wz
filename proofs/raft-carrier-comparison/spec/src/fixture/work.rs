//! Pure explicit one-poll scheduler scaffolding for actual owned futures.
use super::Fault;
use glade_carrier_api::*;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    task::{Context, Poll, Wake, Waker},
};
struct ReadyFlag(AtomicBool);
impl Wake for ReadyFlag {
    fn wake(self: Arc<Self>) {
        self.0.store(true, Ordering::SeqCst);
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.store(true, Ordering::SeqCst);
    }
}
struct Task {
    view: WorkView,
    future: Option<OwnedTask>,
    ready: Arc<ReadyFlag>,
}
struct WorkScope {
    scope: NodeScope,
    clock: LogicalInstant,
    next: u64,
    tasks: BTreeMap<WorkId, Task>,
    stopped: bool,
    failure: Option<Fault>,
    registration_attempts: usize,
}
#[derive(Clone)]
pub struct WorkProbe(Rc<RefCell<WorkScope>>);
impl WorkProbe {
    pub(crate) fn registration_attempts(&self) -> usize {
        self.0.borrow().registration_attempts
    }
    pub fn fail(&self, fault: Fault) {
        self.0.borrow_mut().failure = Some(fault);
    }
    pub(super) fn new(scope: NodeScope) -> Self {
        Self(Rc::new(RefCell::new(WorkScope {
            scope,
            clock: LogicalInstant {
                domain: scope.domain,
                nanos: 0,
            },
            next: 0,
            tasks: BTreeMap::new(),
            stopped: false,
            failure: None,
            registration_attempts: 0,
        })))
    }
}
impl ScheduledWork for WorkProbe {
    fn register(&mut self, work: WorkSpec) -> Result<WorkId, SourceError> {
        // Bind the owned future before the RefCell guard: every rejection drops
        // the guard first, including wrong-domain and sequence overflow.
        let (deadline, future) = match work {
            WorkSpec::Runnable(task) => (None, task),
            WorkSpec::At { deadline, task } => (Some(deadline), task),
        };
        let mut state = self.0.borrow_mut();
        state.registration_attempts += 1;
        if state.stopped {
            return Err(SourceError::Stopped);
        }
        if matches!(state.failure, Some(Fault::Register)) {
            return Err(SourceError::RegistrationFailed);
        }
        if deadline.is_some_and(|deadline| deadline.domain != state.scope.domain) {
            return Err(SourceError::WrongDomain);
        }
        let next = state.next.checked_add(1).ok_or(SourceError::Overflow)?;
        let runnable = deadline.is_none_or(|deadline| deadline.nanos <= state.clock.nanos);
        let id = WorkId {
            scope: state.scope,
            sequence: next,
        };
        state.next = next;
        state.tasks.insert(
            id,
            Task {
                view: WorkView {
                    id,
                    deadline,
                    state: if runnable {
                        WorkState::Runnable
                    } else {
                        WorkState::WaitingDeadline
                    },
                },
                future: Some(future),
                ready: Arc::new(ReadyFlag(AtomicBool::new(runnable))),
            },
        );
        Ok(id)
    }
    fn spawn(&mut self, task: OwnedTask) -> Result<WorkId, SourceError> {
        self.register(WorkSpec::Runnable(task))
    }
    fn advance(&mut self, to: LogicalInstant) -> Result<(), SourceError> {
        let mut state = self.0.borrow_mut();
        if state.stopped {
            return Err(SourceError::Stopped);
        }
        if state.clock.domain != to.domain {
            return Err(SourceError::WrongDomain);
        }
        if to.nanos < state.clock.nanos {
            return Err(SourceError::BackwardTime);
        }
        state.clock = to;
        for task in state.tasks.values_mut() {
            if task.view.state == WorkState::WaitingDeadline
                && task
                    .view
                    .deadline
                    .is_some_and(|deadline| deadline.nanos <= to.nanos)
            {
                task.view.state = WorkState::Runnable;
                task.ready.0.store(true, Ordering::SeqCst);
            }
        }
        Ok(())
    }
    fn poll(&mut self, work: WorkId) -> Result<WorkPoll, SourceError> {
        let mut task = {
            let mut state = self.0.borrow_mut();
            if state.stopped {
                return Err(SourceError::Stopped);
            }
            if work.scope != state.scope {
                return Err(SourceError::InvalidToken);
            }
            if matches!(state.failure, Some(Fault::Task)) {
                return Err(SourceError::TaskFailed);
            }
            let task = state.tasks.get(&work).ok_or(SourceError::InvalidToken)?;
            if matches!(task.view.state, WorkState::Complete | WorkState::Cancelled) {
                return Err(SourceError::InvalidToken);
            }
            if task.future.is_none() || !task.ready.0.load(Ordering::SeqCst) {
                return Err(SourceError::NotDue);
            }
            let task = state.tasks.remove(&work).ok_or(SourceError::InvalidToken)?;
            // Keep scheduler-visible ownership while the future is selected.
            state.tasks.insert(
                work,
                Task {
                    view: task.view,
                    future: None,
                    ready: task.ready.clone(),
                },
            );
            task
        };
        task.ready.0.store(false, Ordering::SeqCst);
        let waker = Waker::from(task.ready.clone());
        let result = task
            .future
            .as_mut()
            .ok_or(SourceError::InvalidToken)?
            .as_mut()
            .poll(&mut Context::from_waker(&waker));
        let (outcome, detached) = {
            let mut state = self.0.borrow_mut();
            let cancelled = state.stopped
                || state
                    .tasks
                    .get(&work)
                    .is_some_and(|task| task.view.state == WorkState::Cancelled);
            if cancelled {
                task.view.state = WorkState::Cancelled;
                task.view.deadline = None;
                let detached = task.future.take();
                state.tasks.insert(work, task);
                (
                    Err(if state.stopped {
                        SourceError::Stopped
                    } else {
                        SourceError::InvalidToken
                    }),
                    detached,
                )
            } else {
                task.view.deadline = None;
                let (outcome, detached) = match result {
                    Poll::Pending => {
                        task.view.state = WorkState::Pending;
                        (WorkPoll::Pending, None)
                    }
                    Poll::Ready(()) => {
                        task.view.state = WorkState::Complete;
                        (WorkPoll::Complete, task.future.take())
                    }
                };
                state.tasks.insert(work, task);
                (Ok(outcome), detached)
            }
        };
        drop(detached);
        outcome
    }
    fn cancel(&mut self, work: WorkId) -> Result<(), SourceError> {
        let detached = {
            let mut state = self.0.borrow_mut();
            if state.stopped {
                return Err(SourceError::Stopped);
            }
            if work.scope != state.scope {
                return Err(SourceError::InvalidToken);
            }
            let task = state
                .tasks
                .get_mut(&work)
                .ok_or(SourceError::InvalidToken)?;
            if matches!(task.view.state, WorkState::Complete | WorkState::Cancelled) {
                return Err(SourceError::InvalidToken);
            }
            task.view.state = WorkState::Cancelled;
            task.view.deadline = None;
            task.future.take()
        };
        drop(detached);
        Ok(())
    }
    fn inventory(&self) -> Vec<WorkView> {
        self.0
            .borrow()
            .tasks
            .values()
            .map(|task| {
                let mut view = task.view;
                if view.state == WorkState::Pending && task.ready.0.load(Ordering::SeqCst) {
                    view.state = WorkState::Runnable;
                }
                view
            })
            .collect()
    }
    fn stop(&mut self) -> Result<(), SourceError> {
        let detached: Vec<_> = {
            let mut state = self.0.borrow_mut();
            state.stopped = true;
            state
                .tasks
                .values_mut()
                .filter_map(|task| {
                    if !matches!(task.view.state, WorkState::Complete | WorkState::Cancelled) {
                        task.view.state = WorkState::Cancelled;
                        task.view.deadline = None;
                    }
                    task.future.take()
                })
                .collect()
        };
        drop(detached);
        Ok(())
    }
}

#[test]
fn overflow_rejection_releases_borrow_before_owned_future_drop() {
    use std::{cell::Cell, future::Future, pin::Pin};
    struct InspectOnDrop(WorkProbe, Rc<Cell<bool>>);
    impl Future for InspectOnDrop {
        type Output = ();
        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<()> {
            Poll::Pending
        }
    }
    impl Drop for InspectOnDrop {
        fn drop(&mut self) {
            let _inventory = self.0.inventory();
            self.1.set(true);
        }
    }
    let fixture = super::Fixture::new(Carrier::OpenRaft, 73);
    let _inputs = fixture.inputs(1, 1);
    let mut work = fixture.work(1, 1);
    work.0.borrow_mut().next = u64::MAX;
    let inspected = Rc::new(Cell::new(false));
    let task = Box::pin(InspectOnDrop(work.clone(), inspected.clone()));
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| work.spawn(task)));
    assert!(
        result.is_ok(),
        "overflow rejection must release borrow before Drop"
    );
    assert_eq!(result.unwrap(), Err(SourceError::Overflow));
    assert!(inspected.get());
    assert!(work.inventory().is_empty());
}
