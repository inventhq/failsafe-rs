use crate::failsafe_error::FailsafeError;
use crate::policies::Policy;
use crate::run_state::PolicyActionState;
use crate::Runnable;
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;

/// Failsafe is a simple library for handling failures. It tries to resemble Failsafe for Java closely.

type FailsafeRunnableResult = Result<Result<(), Box<dyn Any>>, FailsafeError>;

#[macro_export]
macro_rules! failsafe {
    (
        $x:tt;
        $( [ $( $y:expr ),* ]);*
    ) => {
        $x::new($($( $y ),*),*)
    };

    ([
        $(
            $x:tt; $( [ $( $y:expr ),* ])*
         ),*
     ]) => {
        Failsafe::builder()
        $(.push(failsafe!($x; [$($( $y ),*),*])))*
        .build()
    }
}

pub struct Failsafe {
    policy: Box<dyn Policy>,
    state: PolicyActionState,
}

impl Failsafe {
    pub fn run<'a, T: Runnable>(&'a mut self, protected: &'a mut T) -> Result<(), FailsafeError> {
        let mut errors = vec![];
        let k = self.policy.run(&mut Box::new(protected), &mut errors);
        println!("{:?}", errors);
        k
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

#[cfg(test)]
mod tests {
    use super::*;
}
