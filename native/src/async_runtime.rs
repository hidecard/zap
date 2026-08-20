use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

/// A small deterministic executor foundation for future Zap async syntax.
///
/// This deliberately runs one task at a time and does not create worker threads.
/// It provides runtime semantics without changing the synchronous language surface.
pub struct AsyncRuntime {
    tasks: Vec<Pin<Box<dyn Future<Output = ()>>>>,
    limits: RuntimeLimits,
}

#[allow(dead_code)]
impl AsyncRuntime {
    pub fn new() -> Self {
        Self {
            tasks: Vec::new(),
            limits: RuntimeLimits::default(),
        }
    }

    pub fn with_limits(limits: RuntimeLimits) -> Self {
        Self {
            tasks: Vec::new(),
            limits,
        }
    }

    pub fn limits(&self) -> RuntimeLimits {
        self.limits
    }

    pub fn set_limits(&mut self, limits: RuntimeLimits) {
        self.limits = limits;
    }

    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + 'static,
    {
        let _ = self.spawn_limited(future);
    }

    pub fn spawn_limited<F>(&mut self, future: F) -> Result<(), SpawnError>
    where
        F: Future<Output = ()> + 'static,
    {
        if self.tasks.len() >= self.limits.max_tasks {
            return Err(SpawnError::TaskLimitReached {
                limit: self.limits.max_tasks,
            });
        }
        self.tasks.push(Box::pin(future));
        Ok(())
    }

    /// Spawn a task and return a handle that resolves to its output after the
    /// task has completed. The handle is deterministic and does not create
    /// worker threads; it is driven by this runtime's polling loop.
    pub fn spawn_joinable<F, T>(&mut self, future: F) -> Result<JoinHandle<T>, SpawnError>
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        let output = Rc::new(RefCell::new(None));
        let task_output = output.clone();
        self.spawn_limited(async move {
            let value = future.await;
            *task_output.borrow_mut() = Some(Ok(value));
        })?;
        Ok(JoinHandle {
            output,
            consumed: false,
        })
    }

    /// Spawn a joinable task controlled by a cancellation token.
    pub fn spawn_joinable_cancellable<F, T>(
        &mut self,
        future: F,
    ) -> Result<(JoinHandle<T>, CancellationToken), SpawnError>
    where
        F: Future<Output = T> + 'static,
        T: 'static,
    {
        let output = Rc::new(RefCell::new(None));
        let task_output = output.clone();
        let token = CancellationToken::new();
        let task_token = token.clone();
        self.spawn_limited(async move {
            match Cancellable::new(future, task_token).await {
                Ok(value) => *task_output.borrow_mut() = Some(Ok(value)),
                Err(_) => *task_output.borrow_mut() = Some(Err(JoinError::Cancelled)),
            }
        })?;
        Ok((
            JoinHandle {
                output,
                consumed: false,
            },
            token,
        ))
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
        let _ = self.run_with_budget(self.limits.max_polls_per_run);
    }

    pub fn run_with_budget(&mut self, budget: usize) -> RunReport {
        let waker = no_op_waker();
        let mut context = Context::from_waker(&waker);
        let mut index = 0;
        let mut polls = 0;
        while index < self.tasks.len() && polls < budget {
            polls += 1;
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
        RunReport {
            polls,
            pending_tasks: self.tasks.len(),
            budget_exhausted: polls >= budget && !self.tasks.is_empty(),
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeLimits {
    pub max_tasks: usize,
    pub max_polls_per_run: usize,
}

impl Default for RuntimeLimits {
    fn default() -> Self {
        Self {
            max_tasks: usize::MAX,
            max_polls_per_run: usize::MAX,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpawnError {
    TaskLimitReached { limit: usize },
}

#[allow(dead_code)]
pub struct JoinHandle<T> {
    output: Rc<RefCell<Option<Result<T, JoinError>>>>,
    consumed: bool,
}

#[allow(dead_code)]
impl<T> JoinHandle<T> {
    pub fn is_ready(&self) -> bool {
        self.output.borrow().is_some()
    }
}

impl<T> Future for JoinHandle<T> {
    type Output = Result<T, JoinError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.consumed {
            return Poll::Ready(Err(JoinError::AlreadyJoined));
        }
        if let Some(value) = this.output.borrow_mut().take() {
            this.consumed = true;
            Poll::Ready(value)
        } else {
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JoinError {
    AlreadyJoined,
    Cancelled,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RunReport {
    pub polls: usize,
    pub pending_tasks: usize,
    pub budget_exhausted: bool,
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

/// Wrap a future with a deterministic poll-based timeout.
///
/// The inner future is polled first. A pending poll consumes one tick; when
/// no ticks remain, the wrapper completes with `TimeoutError`.
#[allow(dead_code)]
pub fn timeout_ticks<F>(future: F, ticks: u64) -> Timeout<F>
where
    F: Future,
{
    Timeout {
        future: Box::pin(future),
        remaining: ticks,
    }
}

#[allow(dead_code)]
pub struct Timeout<F> {
    future: Pin<Box<F>>,
    remaining: u64,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TimeoutError;

impl<F> Future for Timeout<F>
where
    F: Future,
{
    type Output = Result<F::Output, TimeoutError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        match this.future.as_mut().poll(context) {
            Poll::Ready(value) => Poll::Ready(Ok(value)),
            Poll::Pending if this.remaining == 0 => Poll::Ready(Err(TimeoutError)),
            Poll::Pending => {
                this.remaining -= 1;
                context.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

/// Suspend exactly once, then become ready on the next poll.
#[allow(dead_code)]
pub fn yield_now() -> YieldNow {
    YieldNow { yielded: false }
}

#[allow(dead_code)]
pub struct YieldNow {
    yielded: bool,
}

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yielded {
            Poll::Ready(())
        } else {
            self.yielded = true;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    }
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
    use super::{
        block_on, delay_ticks, timeout_ticks, yield_now, AsyncRuntime, Cancellable,
        CancellationToken, JoinError, RunReport, RuntimeLimits, SpawnError, TimeoutError,
    };
    use std::future::ready;

    #[test]
    fn block_on_returns_ready_value() {
        assert_eq!(block_on(ready(42_u8)), 42);
    }

    #[test]
    fn joinable_task_returns_output_after_runtime_polling() {
        let mut runtime = AsyncRuntime::new();
        let handle = runtime
            .spawn_joinable(async { 42_u8 })
            .expect("task should fit within runtime limits");
        assert!(!handle.is_ready());
        runtime.run_until_idle();
        assert!(handle.is_ready());
        assert_eq!(block_on(handle), Ok(42_u8));
    }

    #[test]
    fn joinable_task_propagates_cancellation() {
        let mut runtime = AsyncRuntime::new();
        let (handle, token) = runtime
            .spawn_joinable_cancellable(async {
                panic!("cancelled task was polled");
            })
            .expect("task should fit within runtime limits");
        token.cancel();
        runtime.run_until_idle();
        assert_eq!(block_on(handle), Err(JoinError::Cancelled));
    }

    #[test]
    fn timeout_ticks_propagates_timeout_and_allows_completion() {
        assert_eq!(
            block_on(timeout_ticks(
                async {
                    delay_ticks(2).await;
                    7_u8
                },
                1
            )),
            Err(TimeoutError)
        );
        assert_eq!(
            block_on(timeout_ticks(
                async {
                    delay_ticks(1).await;
                    9_u8
                },
                2
            )),
            Ok(9_u8)
        );
    }

    #[test]
    fn joinable_task_propagates_spawn_limit_error() {
        let mut runtime = AsyncRuntime::with_limits(RuntimeLimits {
            max_tasks: 0,
            max_polls_per_run: 1,
        });
        assert!(matches!(
            runtime.spawn_joinable(async { 1_u8 }),
            Err(SpawnError::TaskLimitReached { limit: 0 })
        ));
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
    fn resource_limits_reject_excess_tasks_and_report_budget() {
        let mut runtime = AsyncRuntime::with_limits(RuntimeLimits {
            max_tasks: 1,
            max_polls_per_run: 1,
        });
        runtime.spawn(async { yield_now().await });
        assert_eq!(
            runtime.spawn_limited(async {}),
            Err(SpawnError::TaskLimitReached { limit: 1 })
        );
        assert_eq!(
            runtime.run_with_budget(1),
            RunReport {
                polls: 1,
                pending_tasks: 1,
                budget_exhausted: true,
            }
        );
        runtime.set_limits(RuntimeLimits {
            max_tasks: 1,
            max_polls_per_run: 2,
        });
        assert_eq!(runtime.run_with_budget(2).pending_tasks, 0);
    }

    #[test]
    fn yield_now_suspends_once_without_wall_clock_time() {
        let mut runtime = AsyncRuntime::new();
        runtime.spawn(async { yield_now().await });
        assert_eq!(runtime.run_with_budget(1).pending_tasks, 1);
        assert_eq!(runtime.run_with_budget(1).pending_tasks, 0);
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
