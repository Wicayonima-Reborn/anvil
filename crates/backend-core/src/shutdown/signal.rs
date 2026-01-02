#![cfg(feature = "tokio")]

use std::sync::OnceLock;
use tokio::sync::Notify;

/// Global shutdown notifier used to broadcast shutdown events.
static SHUTDOWN: OnceLock<Notify> = OnceLock::new();

/// Get the global shutdown notifier.
pub fn shutdown_notifier() -> &'static Notify {
    SHUTDOWN.get_or_init(Notify::new)
}

/// Wait for an OS shutdown signal (Ctrl+C).
///
/// When received, all waiters will be notified.
pub async fn wait_for_signal() {
    let notify = shutdown_notifier();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            notify.notify_waiters();
        }
    }
}