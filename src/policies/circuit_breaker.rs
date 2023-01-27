use crate::failsafe_error::FailsafeError;
use crate::policies::{Policy, PolicyData};
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::any::Any;
use std::time::{Duration, Instant};

#[derive(PartialEq, Debug)]
pub enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}

pub struct CircuitBreakerPolicy {
    policy_data: PolicyData,
    circuit_breaker_state: CircuitBreakerState,
    failure_threshold: i32,
    success_threshold: i32,
    delay: Duration,
    last_attempt: Option<Instant>,
    failure_count: i32,
    success_count: i32,
}

impl CircuitBreakerPolicy {
    pub(crate) fn new(failure_threshold: i32, delay: Duration, success_threshold: i32) -> Self {
        CircuitBreakerPolicy {
            policy_data: PolicyData {
                state: PolicyActionState::Success,
                runnable_error: Box::new(()),
                inner: None,
            },
            circuit_breaker_state: CircuitBreakerState::Closed,
            failure_threshold,
            success_threshold,
            delay,
            last_attempt: None,
            failure_count: 0,
            success_count: 0,
        }
    }

    pub fn circuit_breaker_state(&self) -> &CircuitBreakerState {
        &self.circuit_breaker_state
    }
    pub fn failure_threshold(&self) -> i32 {
        self.failure_threshold
    }
    pub fn success_threshold(&self) -> i32 {
        self.success_threshold
    }
    pub fn delay(&self) -> Duration {
        self.delay
    }
    pub fn last_attempt(&self) -> Option<Instant> {
        self.last_attempt
    }
    pub fn failure_count(&self) -> i32 {
        self.failure_count
    }
    pub fn success_count(&self) -> i32 {
        self.success_count
    }
}

impl Policy for CircuitBreakerPolicy {
    fn policy_data(&self) -> &PolicyData {
        &self.policy_data
    }

    fn policy_data_mut(&mut self) -> &mut PolicyData {
        &mut self.policy_data
    }

    fn name(&self) -> String {
        "CircuitBreakerPolicy".to_string()
    }

    fn run_guarded(&mut self, runnable: &mut Box<&mut dyn Runnable>) -> Result<(), FailsafeError> {
        if self.circuit_breaker_state == CircuitBreakerState::Open {
            let now = Instant::now();
            if let Some(last_attempt) = self.last_attempt {
                if now - last_attempt > self.delay {
                    self.circuit_breaker_state = CircuitBreakerState::HalfOpen;
                } else {
                    return Err(FailsafeError::CircuitBreakerOpen);
                }
            } else {
                return Err(FailsafeError::CircuitBreakerOpen);
            }
        }
        self.last_attempt = Some(Instant::now());
        match runnable.run() {
            Ok(_) => match self.circuit_breaker_state {
                CircuitBreakerState::Closed => {
                    self.reset();
                    Ok(())
                }
                CircuitBreakerState::HalfOpen => {
                    self.success_count += 1;
                    if self.success_count >= self.success_threshold {
                        self.reset();
                    }
                    Ok(())
                }
                CircuitBreakerState::Open => Err(FailsafeError::CircuitBreakerOpen),
            },
            Err(e) => {
                self.policy_data_mut().state = PolicyActionState::CircuitBreakerError;
                Err(FailsafeError::RunnableError(e))
            }
        }
    }

    fn policy_action(
        &mut self,
        _: &mut Box<&mut dyn Runnable>,
    ) -> Result<PolicyActionState, FailsafeError> {
        match self.policy_data().state {
            PolicyActionState::CircuitBreakerError => {
                self.failure_count += 1;
                if self.failure_count >= self.failure_threshold {
                    self.circuit_breaker_state = CircuitBreakerState::Open
                }
                Err(FailsafeError::CircuitBreakerOpen)
            }
            _ => Ok(PolicyActionState::Success),
        }
    }

    fn reset(&mut self) {
        self.last_attempt = None;
        self.failure_count = 0;
        self.success_count = 0;
        self.circuit_breaker_state = CircuitBreakerState::Closed;
        self.inner_mut()
            .as_mut()
            .and_then(|inner| Some(inner.reset()));
    }
}
