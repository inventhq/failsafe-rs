use std::any::Any;
use std::time::Duration;
use crate::failsafe_error::FailsafeError;
use crate::policies::Policy;
use crate::run_state::PolicyActionState;
use crate::Runnable;

pub struct TimeoutPolicy {
    timeout: Duration,
    inner: Option<Box<dyn Policy>>,
    state: PolicyActionState,
    runnable: Option<Box<dyn Runnable>>,
    runnable_error: Box<dyn Any>,
}

impl TimeoutPolicy {
    pub(crate) fn new(timeout: Duration) -> Self {
        TimeoutPolicy { timeout, inner: None, state: PolicyActionState::Success, runnable: None, runnable_error: Box::new(()) }
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

    fn policy_action(&mut self, runnable: &mut Box<&mut dyn Runnable>) -> Result<PolicyActionState, FailsafeError> {
        todo!()
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
}

pub trait Interruptable {
    fn timeout_reached(&self) {}
}