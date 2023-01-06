#[derive(Debug, Clone)]
pub enum PolicyActionState {
    Success,
    Retry,
    UsingFallback,
    TimeoutError,
}
