use crate::run_state::RunState;

pub mod retry;
pub mod fallback;
pub mod timeout;
pub mod circuit_breaker;

pub trait Policy {
    fn inner(&mut self) -> &mut Option<Box<dyn Policy>>;
    fn set_inner(&mut self, inner: Box<dyn Policy>);
    fn name(&self) -> String;

    fn run(&mut self);

    fn run_inner(&mut self) {
        let n = self.name();
        if self.inner().is_some() {
            let state = self.inner().as_mut().and_then(|mut inner| {
                println!(" > {}", inner.name());
                inner.run();
                Some(inner.state())
            });
            self.set_state(state.unwrap().clone())
        } else {
            println!(".");
        }
    }

    fn state(&self) -> RunState;
    fn set_state(&mut self, state: RunState);
}

