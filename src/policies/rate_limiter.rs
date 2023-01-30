use crate::failsafe_error::FailsafeError;
use crate::policies::{Policy, PolicyData};
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::time::Duration;

pub enum LimiterType {
    Smooth,
    Burst,
}

pub struct RateLimiter {
    policy_data: PolicyData,
    limiter_type: LimiterType,
    max_execution: i32,
    duration: Duration,
    can_run: bool,
}

impl RateLimiter {
    pub fn new(limiter_type: LimiterType, max_execution: i32, duration: Duration) -> Self {
        RateLimiter {
            policy_data: Default::default(),
            limiter_type,
            max_execution,
            duration,
            can_run: false,
        }
    }

    pub fn limiter_type(&self) -> &LimiterType {
        &self.limiter_type
    }
    pub fn max_execution(&self) -> i32 {
        self.max_execution
    }
    pub fn duration(&self) -> Duration {
        self.duration
    }
}

impl Policy for RateLimiter {
    fn policy_data(&self) -> &PolicyData {
        &self.policy_data
    }

    fn policy_data_mut(&mut self) -> &mut PolicyData {
        &mut self.policy_data
    }

    fn name(&self) -> String {
        "RateLimiter".to_string()
    }

    fn policy_action(
        &mut self,
        runnable: &mut Box<&mut dyn Runnable>,
    ) -> Result<PolicyActionState, FailsafeError> {
        todo!()
    }
}
