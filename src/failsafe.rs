use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use crate::policies::Policy;
use crate::run_state::RunState;
use crate::Runnable;


pub struct Failsafe<T: Runnable> {
    error: Option<Box<dyn Error>>,
    policy: Box<dyn Policy>,
    runnable: Option<Result<T, Box<dyn Error>>>
}

impl<T: Runnable> Failsafe<T> {

    pub(crate) fn run(&mut self) {
        self.policy.run();
    }

    pub(crate) fn state(&self) -> RunState {
        RunState::Stable
    }

    pub fn builder() -> FailsafeBuilder<T> {
        FailsafeBuilder::new()
    }

    pub fn error(&self) -> &Option<Box<dyn Error>> {
        &self.error
    }

    pub fn policy(&self) -> &Box<dyn Policy> {
        &self.policy
    }
}

pub struct FailsafeBuilder<R: Runnable> {
    policies: Vec<Box<dyn Policy>>,
    runnable: Option<R>,
}


impl<R: Runnable> FailsafeBuilder<R> {
    fn new() -> FailsafeBuilder<R> {
        FailsafeBuilder { policies: vec![], runnable: None }
    }
}

impl<R: Runnable> FailsafeBuilder<R> {
    pub fn push<T: Policy + 'static>(&mut self, policy: T) -> &mut Self {
        self.policies.push(Box::new(policy));
        self
    }

    pub fn runnable(&mut self, runnable: R) {
        self.runnable = Some(runnable);
    }

    pub(crate) fn build(&mut self) -> Failsafe<R> {
        if self.policies.is_empty() {
            panic!("No policy provided.")
        }
        let mut last = self.policies.pop().unwrap();
        while !self.policies.is_empty() {
            let mut current = self.policies.pop().unwrap();
            current.set_inner(last);
            last = current;
        }
        Failsafe {
            error: None,
            policy: last,
            runnable: None,
        }
    }
}