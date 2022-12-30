use std::time::Duration;

pub mod failsafe_error;
pub mod run_state;
pub mod failsafe;
pub mod policies;

// all objects that are being protected should implement Executable trait
pub trait Runnable {
    fn run(&mut self);
}


#[cfg(test)]
mod tests;
