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
}
#[derive(Clone)]
pub struct WorkProbe(Rc<RefCell<WorkScope>>);
impl WorkProbe {
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
        })))
    }
}
impl ScheduledWork for WorkProbe {
    fn register(&mut self, work: WorkSpec) -> Result<WorkId, SourceError> {
        let mut state = self.0.borrow_mut();
        if state.stopped {
            return Err(SourceError::Stopped);
        }
        if matches!(state.failure, Some(Fault::Register)) {
            return Err(SourceError::RegistrationFailed);
        }
        let (deadline, future) = match work {
            WorkSpec::Runnable(task) => (None, task),
            WorkSpec::At { deadline, task } => {
                if deadline.domain != state.scope.domain {
                    return Err(SourceError::WrongDomain);
                }
                (Some(deadline), task)
            }
        };
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
            if !task.ready.0.load(Ordering::SeqCst) {
                return Err(SourceError::NotDue);
            }
            state.tasks.remove(&work).ok_or(SourceError::InvalidToken)?
        };
        task.ready.0.store(false, Ordering::SeqCst);
        let waker = Waker::from(task.ready.clone());
        let result = task
            .future
            .as_mut()
            .ok_or(SourceError::InvalidToken)?
            .as_mut()
            .poll(&mut Context::from_waker(&waker));
        task.view.deadline = None;
        let outcome = match result {
            Poll::Pending => {
                task.view.state = WorkState::Pending;
                WorkPoll::Pending
            }
            Poll::Ready(()) => {
                task.view.state = WorkState::Complete;
                task.future = None;
                WorkPoll::Complete
            }
        };
        self.0.borrow_mut().tasks.insert(work, task);
        Ok(outcome)
    }
    fn cancel(&mut self, work: WorkId) -> Result<(), SourceError> {
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
        task.future = None;
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
        let mut state = self.0.borrow_mut();
        for task in state.tasks.values_mut() {
            if !matches!(task.view.state, WorkState::Complete | WorkState::Cancelled) {
                task.view.state = WorkState::Cancelled;
                task.future = None;
            }
        }
        state.stopped = true;
        Ok(())
    }
}
