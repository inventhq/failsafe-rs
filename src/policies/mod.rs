use crate::failsafe_error::FailsafeError;
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::any::Any;
use std::error::Error;

pub mod circuit_breaker;
pub mod fallback;
pub mod retry;
pub mod timeout;

pub struct PolicyState {}

pub trait Policy {
    fn inner(&self) -> &Option<Box<dyn Policy>>;
    fn inner_mut(&mut self) -> &mut Option<Box<dyn Policy>>;
    fn set_inner(&mut self, inner: Box<dyn Policy>);
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

    fn state(&self) -> PolicyActionState;
    fn set_state(&mut self, state: PolicyActionState);
    fn on_error(&mut self);
    fn runnable_error(&self) -> &Box<dyn Any>;
    fn set_runnable_error(&mut self, err: Box<dyn Any>);
    fn reset(&mut self) {
        self.inner_mut()
            .as_mut()
            .and_then(|inner| Some(inner.reset()));
    }
}
