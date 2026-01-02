use std::future::Future;
use std::pin::Pin;
use std::sync::Mutex;

type ShutdownHook =
    Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

/// Coordinates graceful shutdown hooks.
///
/// This type is framework-agnostic and does not handle OS signals.
/// Signal handling is provided separately (e.g. via Tokio integration).
pub struct ShutdownCoordinator {
    hooks: Mutex<Vec<ShutdownHook>>,
}

impl ShutdownCoordinator {
    /// Create a new shutdown coordinator.
    pub fn new() -> Self {
        Self {
            hooks: Mutex::new(Vec::new()),
        }
    }

    /// Register an async shutdown hook.
    ///
    /// Hooks are executed in registration order.
    pub fn register<F, Fut>(&self, hook: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let boxed: ShutdownHook = Box::new(move || Box::pin(hook()));
        self.hooks.lock().unwrap().push(boxed);
    }

    /// Execute all registered shutdown hooks.
    ///
    /// This method consumes the coordinator and guarantees hooks
    /// are executed at most once.
    pub async fn shutdown(self) {
        let hooks = self.hooks.into_inner().unwrap();

        for hook in hooks {
            hook().await;
        }
    }
}