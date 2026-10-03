//! Pure scripted source scaffolding. Not an adapted carrier source provider.
use glade_carrier_api::*;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
#[derive(Clone, Copy, Debug)]
pub enum Fault {
    Entropy,
    Exhausted,
    OutOfRange,
    Clock,
    Register,
    Task,
}
#[derive(Clone)]
pub struct SourceProbe(pub(super) Rc<RefCell<SourceState>>);
pub(super) struct SourceState {
    pub(super) scope: NodeScope,
    pub(super) clock: LogicalInstant,
    pub(super) samples: VecDeque<Result<u64, SourceError>>,
    pub(super) fault: Option<Fault>,
    pub(super) trace: Rc<RefCell<Vec<Event>>>,
}
impl SourceProbe {
    pub fn script(&self, values: Vec<u64>) {
        self.0.borrow_mut().samples = values.into_iter().map(Ok).collect();
    }
    pub fn clock(&self) -> LogicalInstant {
        self.0.borrow().clock
    }
    pub fn fail(&self, fault: Fault) {
        self.0.borrow_mut().fault = Some(fault);
    }
    pub fn draws(&self) -> Vec<Event> {
        self.0.borrow().trace.borrow().iter().filter(|event| matches!(event, Event::SourceDraw { scope, .. } if *scope == self.0.borrow().scope)).cloned().collect()
    }
}
impl ElectionSources for SourceProbe {
    fn now(&self) -> Result<LogicalInstant, SourceError> {
        let state = self.0.borrow();
        if matches!(state.fault, Some(Fault::Clock)) {
            return Err(SourceError::ClockFailed);
        }
        let now = state.clock;
        state.trace.borrow_mut().push(Event::SourceRead {
            scope: state.scope,
            now,
        });
        Ok(now)
    }
    fn sample(
        &mut self,
        purpose: DrawPurpose,
        lower: u64,
        upper_exclusive: u64,
    ) -> Result<u64, SourceError> {
        if lower >= upper_exclusive {
            return Err(SourceError::InvalidRange);
        }
        let range = SampleRange {
            lower,
            upper_exclusive,
        };
        let mut state = self.0.borrow_mut();
        let value = match state.fault {
            Some(Fault::Entropy) => return Err(SourceError::EntropyFailed),
            Some(Fault::Exhausted) => return Err(SourceError::ScriptExhausted),
            Some(Fault::OutOfRange) => upper_exclusive,
            _ => state
                .samples
                .pop_front()
                .ok_or(SourceError::ScriptExhausted)??,
        };
        // Deliberately do not repair injected out-of-range results: adapter MUST check.
        state.trace.borrow_mut().push(Event::SourceDraw {
            scope: state.scope,
            purpose,
            range,
            value,
        });
        Ok(value)
    }
    fn advance(&mut self, to: LogicalInstant) -> Result<(), SourceError> {
        let mut state = self.0.borrow_mut();
        if state.clock.domain != to.domain {
            return Err(SourceError::WrongDomain);
        }
        if to.nanos < state.clock.nanos {
            return Err(SourceError::BackwardTime);
        }
        if matches!(state.fault, Some(Fault::Clock)) {
            return Err(SourceError::ClockFailed);
        }
        state.clock = to;
        Ok(())
    }
}
