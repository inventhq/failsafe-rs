use crate::failsafe_error::FailsafeError;
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::any::Any;
use std::error::Error;

pub mod circuit_breaker;
pub mod fallback;
pub mod retry;
pub mod timeout;

pub struct PolicyData {
    state: PolicyActionState,
    runnable_error: Box<dyn Any>,
    inner: Option<Box<dyn Policy>>,
}

impl Default for PolicyData {
    fn default() -> Self {
        PolicyData {
            state: PolicyActionState::Success,
            runnable_error: Box::new(()),
            inner: None,
        }
    }
}

pub trait Policy {
    fn policy_data(&self) -> &PolicyData;
    fn policy_data_mut(&mut self) -> &mut PolicyData;

    fn inner(&self) -> &Option<Box<dyn Policy>> {
        &self.policy_data().inner
    }

    fn inner_mut(&mut self) -> &mut Option<Box<dyn Policy>> {
        &mut self.policy_data_mut().inner
    }

    fn set_inner(&mut self, inner: Box<dyn Policy>) {
        self.policy_data_mut().inner = Some(inner);
    }

    fn state(&self) -> &PolicyActionState {
        &self.policy_data().state
    }

    fn set_state(&mut self, state: PolicyActionState) {
        self.policy_data_mut().state = state;
    }

    fn runnable_error(&self) -> &Box<dyn Any> {
        &self.policy_data().runnable_error
    }

    fn set_runnable_error(&mut self, err: Box<dyn Any>) {
        self.policy_data_mut().runnable_error = err;
    }

    fn name(&self) -> String;

    fn run(
        &mut self,
        mut runnable: &mut Box<&mut dyn Runnable>,
        policy_errors: &mut Vec<FailsafeError>,
    ) -> Result<(), FailsafeError> {
        loop {
            let e = if self.inner_mut().is_some() {
                let result = self
                    .inner_mut()
                    .as_mut()
                    .and_then(|inner| Some(inner.run(runnable, policy_errors)))
                    .unwrap();
                match result {
                    Ok(_) => {
                        self.reset();
                        return Ok(());
                    }
                    Err(e) => e,
                }
            } else {
                match self.run_guarded(runnable) {
                    Ok(_) => return Ok(()),
                    Err(e) => e,
                }
            };
            policy_errors.push(e);
            let result = self.policy_action(&mut runnable);
            if result.is_err() {
                return Err(result.err().unwrap());
            }
            return match result {
                Ok(PolicyActionState::Success) => {
                    self.reset();
                    Ok(())
                }
                Ok(PolicyActionState::Retry) => continue,
                Ok(PolicyActionState::UsingFallback) => Err(FailsafeError::UsedFallback),
                _ => Ok(()),
            };
        }
    }

    fn run_guarded(&mut self, runnable: &mut Box<&mut dyn Runnable>) -> Result<(), FailsafeError> {
        let result = runnable.run();
        match result {
            Ok(_) => {
                self.reset();
                return Ok(());
            }
            Err(e) => Err(FailsafeError::RunnableError(e)),
        }
    }

    fn policy_action(
        &mut self,
        runnable: &mut Box<&mut dyn Runnable>,
    ) -> Result<PolicyActionState, FailsafeError>;

    fn reset(&mut self) {
        self.inner_mut()
            .as_mut()
            .and_then(|inner| Some(inner.reset()));
    }
}
