use crate::failsafe_error::FailsafeError;
use crate::policies::{Policy, PolicyData};
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::any::Any;
use std::fmt::Display;

#[macro_export]
macro_rules! on_fallback {
    ($f: expr) => {
        Box::new(move || -> Box<dyn FallbackAble> { Box::new($f) })
    };
}

pub struct FallbackPolicy {
    fallback: Box<dyn FnMut() -> Box<dyn FallbackAble>>,
    policy_data: PolicyData,
}

impl FallbackPolicy {
    pub(crate) fn new(fallback: Box<dyn FnMut() -> Box<dyn FallbackAble>>) -> Self {
        FallbackPolicy {
            fallback,
            policy_data: PolicyData::default(),
        }
    }
}

impl Policy for FallbackPolicy {
    fn policy_data(&self) -> &PolicyData {
        &self.policy_data
    }

    fn policy_data_mut(&mut self) -> &mut PolicyData {
        &mut self.policy_data
    }

    fn name(&self) -> String {
        "FallbackPolicy".to_string()
    }

    fn policy_action(
        &mut self,
        runnable: &mut Box<&mut dyn Runnable>,
    ) -> Result<PolicyActionState, FailsafeError> {
        runnable.update(&(self.fallback)());
        Ok(PolicyActionState::UsingFallback)
    }
}

pub trait FallbackAble {
    fn as_any(&self) -> &dyn Any;
}
