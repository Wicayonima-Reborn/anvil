use std::sync::atomic::{AtomicBool, Ordering};

/// Represents service health state.
///
/// - Liveness: is the process alive?
/// - Readiness: is the service ready to receive traffic?
pub struct HealthState {
    live: AtomicBool,
    ready: AtomicBool,
}

impl HealthState {
    /// Create a new health state.
    ///
    /// Services start as:
    /// - live = true
    /// - ready = false
    pub fn new() -> Self {
        Self {
            live: AtomicBool::new(true),
            ready: AtomicBool::new(false),
        }
    }

    /// Mark service as ready.
    pub fn mark_ready(&self) {
        self.ready.store(true, Ordering::SeqCst);
        
    }

    /// Mark service as not ready.
    pub fn mark_not_ready(&self) {
        self.ready.store(false, Ordering::SeqCst);
    }

    /// Mark service as not live.
    ///
    /// Typically used during shutdown.
    pub fn mark_dead(&self) {
        self.live.store(false, Ordering::SeqCst);
    }

    /// Is the service alive?
    pub fn is_live(&self) -> bool {
        self.live.load(Ordering::SeqCst)
    }

    /// Is the service ready?
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::SeqCst)
    }
}