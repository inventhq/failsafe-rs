use std::time::Duration;
use crate::policies::Policy;
use crate::run_state::RunState;

pub struct RetryPolicy {
    retries: i32,
    wait_for: Duration,
    inner: Option<Box<dyn Policy>>,
    state: RunState,
}

impl RetryPolicy {
    pub(crate) fn new(retries: i32, wait_for: Duration) -> Self {
        RetryPolicy {
            retries,
            wait_for,
            inner: None,
            state: RunState::Stable,
        }
    }
}

impl Policy for RetryPolicy {
    fn inner(&mut self) -> &mut Option<Box<dyn Policy>> {
        &mut self.inner
    }

    fn set_inner(&mut self, inner: Box<dyn Policy>) {
        self.inner = Some(inner);
    }

    fn name(&self) -> String {
        "RetryPolicy".to_string()
    }

    fn run(&mut self) {
        print!("Running {}", self.name());
        Policy::run_inner(self);
    }

    fn state(&self) -> RunState {
        self.state.clone()
    }

    fn set_state(&mut self, state: RunState) {
        self.state = state;
    }
}
