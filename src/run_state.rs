#[derive(Debug, PartialEq, Clone)]
pub enum RunState {
    Stable,
    TimeoutError,
    RetryError,
    UsingFallback,
}
