use std::time::Duration;
use crate::policies::Policy;
use crate::run_state::RunState;

pub struct TimeoutPolicy {
    timeout: Duration,
    inner: Option<Box<dyn Policy>>,
    state: RunState,
}

impl TimeoutPolicy {
    pub(crate) fn new(timeout: Duration) -> Self {
        TimeoutPolicy { timeout, inner: None, state: RunState::Stable }
    }
}

impl Policy for TimeoutPolicy {
    fn inner(&mut self) -> &mut Option<Box<dyn Policy>> {
        &mut self.inner
    }

    fn set_inner(&mut self, inner: Box<dyn Policy>) {
        self.inner = Some(inner);
    }

    fn name(&self) -> String {
        "TimeoutPolicy".to_string()
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
