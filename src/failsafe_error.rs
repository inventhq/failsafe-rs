use std::any::Any;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FailsafeError {
    #[error("Just a dummy error")]
    DummyError,
    #[error("Timeout error")]
    TimeoutError,
    #[error("Retry error")]
    RetryError,
    #[error("Runnable Error")]
    RunnableError(Box<dyn Any>),
    #[error("Used Fallback")]
    UsedFallback,
    #[error("Unknown Error")]
    UnknownError,
    #[error("Circuit Breaker Open")]
    CircuitBreakerOpen,
}

impl FailsafeError {
    pub fn as_any(&self) -> &dyn Any {
        self
    }

    pub fn from_any(other: &Box<dyn Any>) -> &Self {
        other.downcast_ref::<FailsafeError>().unwrap()
    }
}
