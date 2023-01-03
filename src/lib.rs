use std::any::Any;
use std::error::Error;
use std::time::Duration;
use crate::failsafe_error::FailsafeError;
use crate::person::Person;
use crate::policies::fallback::FallbackAble;

pub mod failsafe_error;
pub mod run_state;
pub mod failsafe;
pub mod policies;

// all objects that are being protected should implement Executable trait
pub trait Runnable {
    fn run(&mut self) -> Result<(), Box<dyn Any>>;
    fn as_any(&self) -> &dyn Any;
    fn update(&mut self, other: &Box<dyn FallbackAble>);
}

#[cfg(test)]
pub mod person;
#[cfg(test)]
mod tests;
