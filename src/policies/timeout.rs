use crate::failsafe_error::FailsafeError;
use crate::policies::Policy;
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::any::Any;
use std::borrow::Borrow;
use std::time::{Duration, Instant};

pub struct TimeoutPolicy {
    timeout: Duration,
    inner: Option<Box<dyn Policy>>,
    state: PolicyActionState,
    runnable_error: Box<dyn Any>,
    time_taken: Option<Duration>,
}

impl TimeoutPolicy {
    pub(crate) fn new(timeout: Duration) -> Self {
        TimeoutPolicy {
            timeout,
            inner: None,
            state: PolicyActionState::Success,
            runnable_error: Box::new(()),
            time_taken: None,
        }
    }
}

impl Policy for TimeoutPolicy {
    fn inner(&self) -> &Option<Box<dyn Policy>> {
        &self.inner
    }

    fn inner_mut(&mut self) -> &mut Option<Box<dyn Policy>> {
        &mut self.inner
    }

    fn set_inner(&mut self, inner: Box<dyn Policy>) {
        self.inner = Some(inner);
    }

    fn name(&self) -> String {
        "TimeoutPolicy".to_string()
    }

    fn run_guarded(&mut self, runnable: &mut Box<&mut dyn Runnable>) -> Result<(), FailsafeError> {
        let start = Instant::now();
        let r = runnable.run();
        self.time_taken = Some(start.elapsed());
        if self.time_taken > Some(self.timeout) {
            self.state = PolicyActionState::TimeoutError;
            return Err(FailsafeError::TimeoutError);
        }
        match r {
            Ok(_) => {}
            Err(e) => return Err(FailsafeError::RunnableError(e)),
        }
        Ok(())
    }

    fn policy_action(
        &mut self,
        _: &mut Box<&mut dyn Runnable>,
    ) -> Result<PolicyActionState, FailsafeError> {
        match self.state {
            PolicyActionState::TimeoutError => Err(FailsafeError::TimeoutError),
            _ => Ok(PolicyActionState::Success),
        }
    }

    fn state(&self) -> PolicyActionState {
        self.state.clone()
    }

    fn set_state(&mut self, state: PolicyActionState) {
        self.state = state;
    }

    fn on_error(&mut self) {
        todo!()
    }

    fn runnable_error(&self) -> &Box<dyn Any> {
        &self.runnable_error
    }

    fn set_runnable_error(&mut self, err: Box<dyn Any>) {
        self.runnable_error = err;
    }

    fn reset(&mut self) {
        self.time_taken = None;
        self.inner_mut()
            .as_mut()
            .and_then(|inner| Some(inner.reset()));
    }
}

pub trait Interruptable {
    fn as_any(&self) -> &dyn Any;
}
