pub mod lifecycle;
pub mod config;
pub mod shutdown;

use crate::lifecycle::{Lifecycle, LifecycleState};
use crate::shutdown::ShutdownCoordinator;

pub async fn shutdown_flow(
    lifecycle: &Lifecycle,
    coordinator: ShutdownCoordinator,
) {
    lifecycle.transition(LifecycleState::ShuttingDown);
    coordinator.shutdown().await;
    lifecycle.transition(LifecycleState::Terminated);
}