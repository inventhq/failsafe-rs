use crate::failsafe_error::FailsafeError;
use crate::policies::{Policy, PolicyData};
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::any::Any;
use std::thread::sleep;
use std::time::Duration;

/// Retry policy, that retries given amount time with a delay before failing
///
/// This policy will retry execution pipeline with given delay between attempts, if execution fails
/// after retries have been exceeded, it will return `FailsafeError::Runnable<Box<Any>`
///
/// ## Features
///
/// - [x] Retries
/// - [x] Delay between retries
/// - [ ] Back off [Link](https://failsafe.dev/javadoc/core/dev/failsafe/RetryPolicyBuilder.html#withBackoff-long-long-java.time.temporal.ChronoUnit-)
/// - [ ] Random delay
/// - [ ] Jitter [Check](https://failsafe.dev/javadoc/core/dev/failsafe/RetryPolicyBuilder.html#withJitter-double-)
/// - [ ] No limit
///
pub struct RetryPolicy {
    policy_data: PolicyData,
    retries: i32,
    delay: Duration,
    tries: i32,
}

impl RetryPolicy {
    pub(crate) fn new(retries: i32, delay: Duration) -> Self {
        let policy = RetryPolicy {
            policy_data: Default::default(),
            retries,
            delay,
            tries: 0,
        };
        policy
    }
}

impl Policy for RetryPolicy {
    fn policy_data(&self) -> &PolicyData {
        &self.policy_data
    }

    fn policy_data_mut(&mut self) -> &mut PolicyData {
        &mut self.policy_data
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
            sleep(self.delay);
            Ok(PolicyActionState::Retry)
        };
    }

    fn reset(&mut self) {
        self.tries = 0;
        self.inner_mut()
            .as_mut()
            .and_then(|mut inner| Some(inner.reset()));
    }
}
