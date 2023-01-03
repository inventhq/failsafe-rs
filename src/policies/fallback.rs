use std::any::Any;
use std::borrow::BorrowMut;
use std::fmt::{Display};
use crate::failsafe_error::FailsafeError;
use crate::policies::Policy;
use crate::run_state::PolicyActionState;
use crate::Runnable;


pub struct FallbackPolicy {
    fallback: Box<dyn FnMut() -> Box<dyn FallbackAble>>,
    inner: Option<Box<dyn Policy>>,
    state: PolicyActionState,
    runnable: Option<Box<dyn Runnable>>,
    runnable_error: Box<dyn Any>,
}

impl FallbackPolicy {
    pub(crate) fn new(fallback: Box<dyn FnMut() -> Box<dyn FallbackAble>>) -> Self {
        FallbackPolicy { fallback, inner: None, state: PolicyActionState::Success, runnable: None, runnable_error: Box::new(()) }
    }
}

impl Policy for FallbackPolicy {

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
        "FallbackPolicy".to_string()
    }

    fn policy_action(&mut self, runnable: &mut Box<&mut dyn Runnable>) -> Result<PolicyActionState, FailsafeError> {
        runnable.update(&(self.fallback)());
        Ok(PolicyActionState::UsingFallback)
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
}

pub trait FallbackAble {
    fn as_any(&self) -> &dyn Any;
}