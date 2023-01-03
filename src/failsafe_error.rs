use std::error::Error;
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
    RunnableError(Box<dyn Error>),
    #[error("Used Fallback")]
    UsedFallback,
    #[error("Unknown Error")]
    UnknownError,
}
