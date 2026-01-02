#[cfg(feature = "tokio")]
mod signal;

mod coordinator;

#[cfg(feature = "tokio")]
pub use signal::{shutdown_notifier, wait_for_signal};

pub use coordinator::ShutdownCoordinator;