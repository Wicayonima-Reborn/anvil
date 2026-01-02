pub mod config;
pub mod lifecycle;
pub mod shutdown;

use lifecycle::{Lifecycle, LifecycleState};
use shutdown::ShutdownCoordinator;

/// Execute the coordinated shutdown flow.
///
/// This function transitions lifecycle state and executes all
/// registered shutdown hooks.
///
/// Higher-level startup orchestration may move this function
/// into a dedicated module in the future.
pub async fn shutdown_flow(
    lifecycle: &Lifecycle,
    coordinator: ShutdownCoordinator,
) {
    lifecycle.transition(LifecycleState::ShuttingDown);
    coordinator.shutdown().await;
    lifecycle.transition(LifecycleState::Terminated);
}