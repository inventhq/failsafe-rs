use crate::failsafe_error::FailsafeError;
use crate::policies::Policy;
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::any::Any;
use std::thread::sleep;
use std::time::Duration;

pub struct RetryPolicy {
    retries: i32,
    wait_for: Duration,
    inner: Option<Box<dyn Policy>>,
    state: PolicyActionState,
    tries: i32,
    runnable_error: Box<dyn Any>,
}

impl RetryPolicy {
    pub(crate) fn new(retries: i32, wait_for: Duration) -> Self {
        let policy = RetryPolicy {
            retries,
            wait_for,
            inner: None,
            state: PolicyActionState::Success,
            tries: 0,
            runnable_error: Box::new(()),
        };
        policy
    }
}

impl Policy for RetryPolicy {
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
        "RetryPolicy".to_string()
    }

    fn policy_action(
        &mut self,
        _: &mut Box<&mut dyn Runnable>,
    ) -> Result<PolicyActionState, FailsafeError> {
        self.tries += 1;
        return if self.tries >= self.retries {
            self.tries = 0;
            Err(FailsafeError::RetryError)
        } else {
            sleep(self.wait_for);
            Ok(PolicyActionState::Retry)
        };
    }

    fn state(&self) -> PolicyActionState {
        self.state.clone()
    }

    fn set_state(&mut self, state: PolicyActionState) {
        self.state = state;
    }

    fn on_error(&mut self) {}

    fn runnable_error(&self) -> &Box<dyn Any> {
        &self.runnable_error
    }

    fn set_runnable_error(&mut self, err: Box<dyn Any>) {
        self.runnable_error = err;
    }

    fn reset(&mut self) {
        self.tries = 0;
        self.inner_mut()
            .as_mut()
            .and_then(|mut inner| Some(inner.reset()));
    }
}
