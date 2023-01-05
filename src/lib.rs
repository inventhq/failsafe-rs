use crate::failsafe_error::FailsafeError;
use crate::policies::fallback::FallbackAble;
use std::any::Any;
use std::error::Error;
use std::time::Duration;

pub mod failsafe;
pub mod failsafe_error;
pub mod policies;
pub mod run_state;

// all objects that are being protected should implement Executable trait
pub trait Runnable {
    fn run(&mut self) -> Result<(), Box<dyn Any>>;
    fn update(&mut self, other: &Box<dyn FallbackAble>);
}

#[cfg(test)]
pub mod person;
#[cfg(test)]
mod tests;
