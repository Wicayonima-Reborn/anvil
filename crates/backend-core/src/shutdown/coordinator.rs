use std::sync::Mutex;
use std::future::Future;
use std::pin::Pin;

type ShutdownHook =
    Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = ()> + Send>> + Send>;

pub struct ShutdownCoordinator {
    hooks: Mutex<Vec<ShutdownHook>>,
}

impl ShutdownCoordinator {
    pub fn new() -> Self {
        Self {
            hooks: Mutex::new(Vec::new()),
        }
    }

    pub fn register<F, Fut>(&self, hook: F)
    where
        F: FnOnce() -> Fut + Send + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let boxed: ShutdownHook = Box::new(move || Box::pin(hook()));
        self.hooks.lock().unwrap().push(boxed);
    }

    pub async fn shutdown(self) {
        let hooks = self.hooks.into_inner().unwrap();

        for hook in hooks {
            hook().await;
        }
    }
}