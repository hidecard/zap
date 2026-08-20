use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, Waker};

/// A small deterministic executor foundation for future Zap async syntax.
///
/// This deliberately runs one task at a time and does not create worker threads.
/// It provides runtime semantics without changing the synchronous language surface.
pub struct AsyncRuntime {
    tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,
}

impl AsyncRuntime {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        self.tasks.push(Box::pin(future));
    }

    pub fn run_until_idle(&mut self) {
        let waker = Waker::noop();
        let mut context = Context::from_waker(waker);
        let mut index = 0;
        while index < self.tasks.len() {
            let ready = matches!(
                self.tasks[index].as_mut().poll(&mut context),
                Poll::Ready(())
            );
            if ready {
                let _ = self.tasks.remove(index);
            } else {
                index += 1;
            }
        }
    }

    pub fn pending_tasks(&self) -> usize {
        self.tasks.len()
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new()
    }
}

pub fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    let waker = Waker::from(Arc::new(NoopWaker));
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    loop {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{block_on, AsyncRuntime};
    use std::future::ready;

    #[test]
    fn block_on_returns_ready_value() {
        assert_eq!(block_on(ready(42_u8)), 42);
    }

    #[test]
    fn executor_runs_tasks_in_spawn_order() {
        let mut runtime = AsyncRuntime::new();
        let output = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        for value in [1, 2, 3] {
            let output = output.clone();
            runtime.spawn(async move { output.lock().unwrap().push(value) });
        }
        runtime.run_until_idle();
        assert_eq!(*output.lock().unwrap(), vec![1, 2, 3]);
        assert_eq!(runtime.pending_tasks(), 0);
    }
}
