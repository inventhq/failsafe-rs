use crate::failsafe_error::FailsafeError;
use crate::policies::{Policy, PolicyData};
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::any::Any;
use std::borrow::Borrow;
use std::time::{Duration, Instant};

pub struct TimeoutPolicy {
    timeout: Duration,
    policy_data: PolicyData,
    time_taken: Option<Duration>,
}

impl TimeoutPolicy {
    pub(crate) fn new(timeout: Duration) -> Self {
        TimeoutPolicy {
            timeout,
            policy_data: Default::default(),
            time_taken: None,
        }
    }
}

impl Policy for TimeoutPolicy {
    fn policy_data(&self) -> &PolicyData {
        &self.policy_data
    }

    fn policy_data_mut(&mut self) -> &mut PolicyData {
        &mut self.policy_data
    }

    fn name(&self) -> String {
        "TimeoutPolicy".to_string()
    }

    fn run_guarded(&mut self, runnable: &mut Box<&mut dyn Runnable>) -> Result<(), FailsafeError> {
        let start = Instant::now();
        let r = runnable.run();
        self.time_taken = Some(start.elapsed());
        if self.time_taken > Some(self.timeout) {
            self.policy_data.state = PolicyActionState::TimeoutError;
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
        match self.policy_data().state {
            PolicyActionState::TimeoutError => Err(FailsafeError::TimeoutError),
            _ => Ok(PolicyActionState::Success),
        }
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
