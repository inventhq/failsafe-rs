use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use crate::failsafe_error::FailsafeError;
use crate::policies::Policy;
use crate::run_state::PolicyActionState;
use crate::Runnable;


pub struct Failsafe {
    policy: Box<dyn Policy>,
    state: PolicyActionState,
}

impl Failsafe {
    pub fn run<'a, T: Runnable>(&'a mut self, protected: &'a mut T) -> Result<(), FailsafeError> {
        self.policy.run(&mut Box::new(protected))
    }

    pub(crate) fn state(&self) -> PolicyActionState {
        PolicyActionState::Success
    }

    pub fn builder() -> FailsafeBuilder {
        FailsafeBuilder::new()
    }

    pub fn policy(&self) -> &Box<dyn Policy> {
        &self.policy
    }
}

pub struct FailsafeBuilder {
    policies: Vec<Box<dyn Policy>>,
}


impl FailsafeBuilder {
    fn new() -> FailsafeBuilder {
        FailsafeBuilder { policies: vec![] }
    }
}

impl FailsafeBuilder {
    pub fn push<T: Policy + 'static>(&mut self, policy: T) -> &mut Self {
        self.policies.push(Box::new(policy));
        self
    }

    pub(crate) fn build(&mut self) -> Failsafe {
        if self.policies.is_empty() {
            panic!("No policy or runnable provided.")
        }
        let mut first = self.policies.pop().unwrap();
        while !self.policies.is_empty() {
            let mut current = self.policies.pop().unwrap();
            current.set_inner(first);
            first = current;
        }
        Failsafe {
            policy: first,
            state: PolicyActionState::Success,
        }
    }
}