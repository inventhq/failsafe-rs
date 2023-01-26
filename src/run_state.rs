pub enum RunState {
    Success,
    TimeoutError,
    CircuitBreakerError,
}

#[derive(Debug, Clone)]
pub enum PolicyActionState {
    Success,
    Retry,
    UsingFallback,
    TimeoutError,
    CircuitBreakerError,
}
