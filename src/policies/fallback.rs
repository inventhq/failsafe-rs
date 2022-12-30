use std::fmt::{Display};
use crate::policies::Policy;
use crate::run_state::RunState;

pub struct Fallback<T: FallbackAble> {
    fallback: T,
    inner: Option<Box<dyn Policy>>,
    state: RunState,
}

impl<T: FallbackAble> Fallback<T> {
    pub(crate) fn new(fallback: T) -> Self {
        Fallback { fallback, inner: None, state: RunState::Stable }
    }

}

impl<T: FallbackAble> Policy for Fallback<T> {
    fn inner(&mut self) -> &mut Option<Box<dyn Policy>> {
        &mut self.inner
    }

    fn set_inner(&mut self, inner: Box<dyn Policy>) {
        self.inner = Some(inner);
    }

    fn name(&self) -> String {
        "Fallback".to_string()
    }

    fn run(&mut self) {
        print!("Running {}", self.name());
        Policy::run_inner(self);
        println!();
    }

    fn state(&self) -> RunState {
        self.state.clone()
    }

    fn set_state(&mut self, state: RunState) {
        self.state = state;
    }

}

pub trait FallbackAble {
    fn default(&mut self);
}