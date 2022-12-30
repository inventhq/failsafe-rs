use std::error::Error;
use thiserror::Error;
use crate::policies::retry::RetryPolicy;
use crate::policies::timeout::TimeoutPolicy;

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

}
