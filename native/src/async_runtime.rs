use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

/// A small deterministic executor foundation for future Zap async syntax.
///
/// This deliberately runs one task at a time and does not create worker threads.
/// It provides runtime semantics without changing the synchronous language surface.
pub struct AsyncRuntime {
    tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,
}

#[allow(dead_code)]
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

    /// Spawn a task that completes with cancellation instead of polling its
    /// inner future after the returned token is cancelled.
    pub fn spawn_cancellable<F>(&mut self, future: F) -> CancellationToken
    where
        F: Future<Output = ()> + 'static,
    {
        let token = CancellationToken::new();
        let task_token = token.clone();
        self.spawn(async move {
            let _ = Cancellable::new(future, task_token).await;
        });
        token
    }

    pub fn run_until_idle(&mut self) {
        let waker = no_op_waker();
        let mut context = Context::from_waker(&waker);
        let mut index = 0;
        while index < self.tasks.len() {
            let ready = matches!(
                self.tasks[index].as_mut().poll(&mut context),
                Poll::Ready(())
            );
            if ready {
                std::mem::drop(self.tasks.remove(index));
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

#[allow(dead_code)]
pub fn block_on<F>(future: F) -> F::Output
where
    F: Future,
{
    let waker = no_op_waker();
    let mut context = Context::from_waker(&waker);
    let mut future = Box::pin(future);
    loop {
        if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
            return value;
        }
    }
}

/// A deterministic delay measured in executor polls rather than wall-clock time.
/// A zero-tick delay is immediately ready; positive values require that many
/// pending polls before becoming ready.
#[allow(dead_code)]
pub fn delay_ticks(ticks: u64) -> Delay {
    Delay { remaining: ticks }
}

#[allow(dead_code)]
pub struct Delay {
    remaining: u64,
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            Poll::Ready(())
        } else {
            self.remaining -= 1;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

#[allow(dead_code)]
impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Cancelled;

#[allow(dead_code)]
pub struct Cancellable<F> {
    future: Pin<Box<F>>,
    token: CancellationToken,
}

#[allow(dead_code)]
impl<F> Cancellable<F> {
    pub fn new(future: F, token: CancellationToken) -> Self {
        Self {
            future: Box::pin(future),
            token,
        }
    }
}

#[allow(dead_code)]
impl<F> Future for Cancellable<F>
where
    F: Future,
{
    type Output = Result<F::Output, Cancelled>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.token.is_cancelled() {
            return Poll::Ready(Err(Cancelled));
        }
        this.future.as_mut().poll(context).map(Ok)
    }
}

fn no_op_waker() -> Waker {
    Waker::from(Arc::new(NoopWaker))
}

struct NoopWaker;

#[allow(clippy::manual_noop_waker)]
impl Wake for NoopWaker {
    fn wake(self: Arc<Self>) {}
}

#[cfg(test)]
mod tests {
    use super::{block_on, delay_ticks, AsyncRuntime, Cancellable, CancellationToken};
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

    #[test]
    fn delay_ticks_is_deterministic_and_non_blocking() {
        assert_eq!(
            block_on(async {
                delay_ticks(0).await;
                7_u8
            }),
            7
        );
        let mut runtime = AsyncRuntime::new();
        runtime.spawn(async { delay_ticks(2).await });
        runtime.run_until_idle();
        assert_eq!(runtime.pending_tasks(), 1);
        runtime.run_until_idle();
        assert_eq!(runtime.pending_tasks(), 1);
        runtime.run_until_idle();
        assert_eq!(runtime.pending_tasks(), 0);
    }

    #[test]
    fn cancellation_stops_inner_future_before_polling() {
        let token = CancellationToken::new();
        let polled = std::sync::Arc::new(std::sync::Mutex::new(false));
        let marker = polled.clone();
        let mut future = Box::pin(Cancellable::new(
            async move { *marker.lock().unwrap() = true },
            token.clone(),
        ));
        token.cancel();
        assert_eq!(
            block_on(async move { future.as_mut().await }),
            Err(super::Cancelled)
        );
        assert!(!*polled.lock().unwrap());
    }

    #[test]
    fn cancellable_spawn_is_removed_after_cancel() {
        let mut runtime = AsyncRuntime::new();
        let token = runtime.spawn_cancellable(async { panic!("cancelled task was polled") });
        token.cancel();
        runtime.run_until_idle();
        assert_eq!(runtime.pending_tasks(), 0);
    }
}
