#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Initializing,
    Starting,
    Ready,
    ShuttingDown,
    Terminated,
}