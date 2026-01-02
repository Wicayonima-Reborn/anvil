#![cfg(feature = "tokio")]

use std::sync::OnceLock;
use tokio::sync::Notify;

/// Global shutdown notifier
static SHUTDOWN: OnceLock<Notify> = OnceLock::new();

pub fn shutdown_notifier() -> &'static Notify {
    SHUTDOWN.get_or_init(Notify::new)
}

/// Wait for OS shutdown signal (Ctrl+C)
pub async fn wait_for_signal() {
    let notify = shutdown_notifier();

    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            notify.notify_waiters();
        }
    }
}