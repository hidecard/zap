use std::cell::RefCell;
use std::future::Future;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::pin::Pin;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Wake, Waker};
use std::thread;
use std::time::{Duration, Instant};

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

    /// Spawn a fallible task and preserve its typed error for the joining caller.
    pub fn spawn_joinable_result<F, T, E>(
        &mut self,
        future: F,
    ) -> Result<TaskJoinHandle<T, E>, SpawnError>
    where
        F: Future<Output = Result<T, E>> + 'static,
        T: 'static,
        E: 'static,
    {
        let output = Rc::new(RefCell::new(None));
        let task_output = output.clone();
        self.spawn_limited(async move {
            *task_output.borrow_mut() = Some(future.await.map_err(TaskJoinError::Failed));
        })?;
        Ok(TaskJoinHandle {
            output,
            consumed: false,
        })
    }

    /// Spawn a fallible joinable task controlled by a cancellation token.
    /// Cancellation wins because it is checked before polling the inner future.
    pub fn spawn_joinable_result_cancellable<F, T, E>(
        &mut self,
        future: F,
    ) -> Result<(TaskJoinHandle<T, E>, CancellationToken), SpawnError>
    where
        F: Future<Output = Result<T, E>> + 'static,
        T: 'static,
        E: 'static,
    {
        let output = Rc::new(RefCell::new(None));
        let task_output = output.clone();
        let token = CancellationToken::new();
        let task_token = token.clone();
        self.spawn_limited(async move {
            match Cancellable::new(future, task_token).await {
                Ok(result) => {
                    *task_output.borrow_mut() = Some(result.map_err(TaskJoinError::Failed));
                }
                Err(_) => *task_output.borrow_mut() = Some(Err(TaskJoinError::Cancelled)),
            }
        })?;
        Ok((
            TaskJoinHandle {
                output,
                consumed: false,
            },
            token,
        ))
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
#[derive(Debug, Eq, PartialEq)]
pub enum TaskJoinError<E> {
    AlreadyJoined,
    Cancelled,
    Failed(E),
}

#[allow(dead_code)]
pub struct TaskJoinHandle<T, E> {
    output: Rc<RefCell<Option<Result<T, TaskJoinError<E>>>>>,
    consumed: bool,
}

#[allow(dead_code)]
impl<T, E> TaskJoinHandle<T, E> {
    pub fn is_ready(&self) -> bool {
        self.output.borrow().is_some()
    }
}

impl<T, E> Future for TaskJoinHandle<T, E> {
    type Output = Result<T, TaskJoinError<E>>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.consumed {
            return Poll::Ready(Err(TaskJoinError::AlreadyJoined));
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

/// Bounded limits for the production-oriented threaded scheduler and I/O facade.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ThreadRuntimeLimits {
    pub max_workers: usize,
    pub max_tasks: usize,
    pub max_read_bytes: u64,
}

impl Default for ThreadRuntimeLimits {
    fn default() -> Self {
        Self {
            max_workers: 4,
            max_tasks: 64,
            max_read_bytes: 8 * 1024 * 1024,
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadSpawnError {
    InvalidWorkerLimit,
    TaskLimitReached { limit: usize },
    QueueClosed,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThreadJoinError {
    AlreadyJoined,
    WorkerPanicked,
}

struct ThreadResult<T> {
    result: Mutex<Option<Result<T, ThreadJoinError>>>,
    waker: Mutex<Option<Waker>>,
}

/// A bounded worker scheduler for blocking system I/O and CPU-bound adapters.
/// Tasks are admitted atomically, run on a fixed worker set, and wake joiners
/// without requiring a polling thread to busy-wait.
#[allow(dead_code)]
pub struct ThreadedRuntime {
    sender: mpsc::SyncSender<Box<dyn FnOnce() + Send + 'static>>,
    active: Arc<std::sync::atomic::AtomicUsize>,
    limits: ThreadRuntimeLimits,
}

#[allow(dead_code)]
impl ThreadedRuntime {
    pub fn new(limits: ThreadRuntimeLimits) -> Result<Self, ThreadSpawnError> {
        if limits.max_workers == 0 || limits.max_tasks == 0 {
            return Err(ThreadSpawnError::InvalidWorkerLimit);
        }
        let (sender, receiver) =
            mpsc::sync_channel::<Box<dyn FnOnce() + Send + 'static>>(limits.max_tasks);
        let receiver = Arc::new(Mutex::new(receiver));
        for _ in 0..limits.max_workers {
            let worker_receiver = receiver.clone();
            thread::spawn(move || loop {
                let job = worker_receiver
                    .lock()
                    .ok()
                    .and_then(|queue| queue.recv().ok());
                match job {
                    Some(job) => job(),
                    None => break,
                }
            });
        }
        Ok(Self {
            sender,
            active: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            limits,
        })
    }

    pub fn limits(&self) -> ThreadRuntimeLimits {
        self.limits
    }

    pub fn spawn_blocking<F, T>(&self, task: F) -> Result<ThreadJoinHandle<T>, ThreadSpawnError>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let mut current = self.active.load(Ordering::Acquire);
        loop {
            if current >= self.limits.max_tasks {
                return Err(ThreadSpawnError::TaskLimitReached {
                    limit: self.limits.max_tasks,
                });
            }
            match self.active.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(observed) => current = observed,
            }
        }
        let shared = Arc::new(ThreadResult {
            result: Mutex::new(None),
            waker: Mutex::new(None),
        });
        let worker_result = shared.clone();
        let active = self.active.clone();
        let job = Box::new(move || {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(task))
                .map_err(|_| ThreadJoinError::WorkerPanicked);
            if let Ok(mut slot) = worker_result.result.lock() {
                *slot = Some(result);
            }
            if let Ok(mut slot) = worker_result.waker.lock() {
                if let Some(waker) = slot.take() {
                    waker.wake();
                }
            }
            active.fetch_sub(1, Ordering::AcqRel);
        });
        if self.sender.send(job).is_err() {
            self.active.fetch_sub(1, Ordering::AcqRel);
            return Err(ThreadSpawnError::QueueClosed);
        }
        Ok(ThreadJoinHandle {
            shared,
            consumed: false,
        })
    }

    pub fn read_file_async<P>(
        &self,
        path: P,
    ) -> Result<ThreadJoinHandle<Result<Vec<u8>, String>>, ThreadSpawnError>
    where
        P: Into<std::path::PathBuf>,
    {
        let path = path.into();
        let max_bytes = self.limits.max_read_bytes;
        self.spawn_blocking(move || {
            let metadata = std::fs::symlink_metadata(&path).map_err(|error| error.to_string())?;
            if !metadata.file_type().is_file() {
                return Err("async read requires a regular file".to_owned());
            }
            if metadata.len() > max_bytes {
                return Err(format!("async read exceeds {} byte limit", max_bytes));
            }
            std::fs::read(&path).map_err(|error| error.to_string())
        })
    }
}

#[allow(dead_code)]
pub struct ThreadJoinHandle<T> {
    shared: Arc<ThreadResult<T>>,
    consumed: bool,
}

#[allow(dead_code)]
impl<T> ThreadJoinHandle<T> {
    pub fn is_ready(&self) -> bool {
        self.shared
            .result
            .lock()
            .map(|result| result.is_some())
            .unwrap_or(false)
    }
}

impl<T> Future for ThreadJoinHandle<T> {
    type Output = Result<T, ThreadJoinError>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        if this.consumed {
            return Poll::Ready(Err(ThreadJoinError::AlreadyJoined));
        }
        if let Ok(mut result) = this.shared.result.lock() {
            if let Some(value) = result.take() {
                this.consumed = true;
                return Poll::Ready(value);
            }
        }
        if let Ok(mut waker) = this.shared.waker.lock() {
            *waker = Some(context.waker().clone());
        }
        Poll::Pending
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdapterLimits {
    pub max_socket_bytes: usize,
    pub socket_timeout: Duration,
    pub max_process_output_bytes: usize,
    pub process_timeout: Duration,
}

impl Default for AdapterLimits {
    fn default() -> Self {
        Self {
            max_socket_bytes: 1024 * 1024,
            socket_timeout: Duration::from_secs(10),
            max_process_output_bytes: 64 * 1024,
            process_timeout: Duration::from_secs(10),
        }
    }
}

#[allow(dead_code)]
impl ThreadedRuntime {
    /// Execute one bounded TCP request/response exchange without blocking the
    /// caller. The socket is switched to non-blocking mode and all waits are
    /// deadline-bounded; response bytes are capped by `max_socket_bytes`.
    pub fn tcp_exchange(
        &self,
        address: String,
        request: Vec<u8>,
    ) -> Result<ThreadJoinHandle<Result<Vec<u8>, String>>, ThreadSpawnError> {
        let limits = AdapterLimits {
            max_socket_bytes: self.limits.max_read_bytes as usize,
            ..AdapterLimits::default()
        };
        self.spawn_blocking(move || nonblocking_tcp_exchange(&address, &request, limits))
    }

    /// Spawn a process with bounded stdout/stderr capture and a hard deadline.
    /// On deadline expiry the child is killed and the join result is an error.
    pub fn process_async(
        &self,
        command: String,
        arguments: Vec<String>,
    ) -> Result<ThreadJoinHandle<Result<ProcessOutput, String>>, ThreadSpawnError> {
        let limits = AdapterLimits {
            max_process_output_bytes: self.limits.max_read_bytes as usize,
            process_timeout: Duration::from_secs(10),
            ..AdapterLimits::default()
        };
        self.spawn_blocking(move || run_process_bounded(&command, &arguments, limits))
    }

    /// Spawn a process that is forcibly terminated when the returned token is
    /// cancelled. Cancellation is checked while waiting and always performs a
    /// best-effort kill and wait before the worker resolves.
    pub fn process_async_cancellable(
        &self,
        command: String,
        arguments: Vec<String>,
    ) -> Result<
        (
            ThreadJoinHandle<Result<ProcessOutput, String>>,
            CancellationToken,
        ),
        ThreadSpawnError,
    > {
        let limits = AdapterLimits {
            max_process_output_bytes: self.limits.max_read_bytes as usize,
            process_timeout: Duration::from_secs(10),
            ..AdapterLimits::default()
        };
        let token = CancellationToken::new();
        let task_token = token.clone();
        let handle = self.spawn_blocking(move || {
            run_process_bounded_with_cancel(&command, &arguments, limits, &task_token)
        })?;
        Ok((handle, token))
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessOutput {
    pub status: i32,
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

fn nonblocking_tcp_exchange(
    address: &str,
    request: &[u8],
    limits: AdapterLimits,
) -> Result<Vec<u8>, String> {
    let endpoint = address
        .to_socket_addrs()
        .map_err(|error| format!("tcp address resolution failed: {error}"))?
        .next()
        .ok_or_else(|| "tcp address resolution returned no endpoints".to_owned())?;
    let deadline = Instant::now() + limits.socket_timeout;
    let stream = TcpStream::connect_timeout(&endpoint, limits.socket_timeout)
        .map_err(|error| format!("tcp connect failed: {error}"))?;
    stream
        .set_nonblocking(true)
        .map_err(|error| format!("tcp non-blocking setup failed: {error}"))?;
    let mut stream = stream;
    let mut sent = 0;
    while sent < request.len() {
        match stream.write(&request[sent..]) {
            Ok(0) => return Err("tcp peer closed during write".to_owned()),
            Ok(count) => sent += count,
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                if Instant::now() >= deadline {
                    return Err("tcp write exceeded deadline".to_owned());
                }
                thread::sleep(Duration::from_millis(1));
            }
            Err(error) => return Err(format!("tcp write failed: {error}")),
        }
    }
    let mut response = Vec::new();
    let mut buffer = [0_u8; 8192];
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => {
                if response.len().saturating_add(count) > limits.max_socket_bytes {
                    return Err(format!(
                        "tcp response exceeds {} byte limit",
                        limits.max_socket_bytes
                    ));
                }
                response.extend_from_slice(&buffer[..count]);
            }
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                if Instant::now() >= deadline {
                    return Err("tcp read exceeded deadline".to_owned());
                }
                thread::sleep(Duration::from_millis(1));
            }
            Err(error) => return Err(format!("tcp read failed: {error}")),
        }
    }
    Ok(response)
}

fn run_process_bounded(
    command: &str,
    arguments: &[String],
    limits: AdapterLimits,
) -> Result<ProcessOutput, String> {
    run_process_bounded_with_cancel(command, arguments, limits, &CancellationToken::new())
}

fn run_process_bounded_with_cancel(
    command: &str,
    arguments: &[String],
    limits: AdapterLimits,
    token: &CancellationToken,
) -> Result<ProcessOutput, String> {
    let mut child = Command::new(command)
        .args(arguments)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("process start failed: {error}"))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "process stdout was not captured".to_owned())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "process stderr was not captured".to_owned())?;
    let stdout_reader =
        thread::spawn(move || read_capped_output(stdout, limits.max_process_output_bytes));
    let stderr_reader =
        thread::spawn(move || read_capped_output(stderr, limits.max_process_output_bytes));
    let deadline = Instant::now() + limits.process_timeout;
    let status = loop {
        match child.try_wait() {
            Ok(Some(status)) => break status,
            Ok(None) if token.is_cancelled() => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
                return Err("process cancelled and child terminated".to_owned());
            }
            Ok(None) if Instant::now() >= deadline => {
                let _ = child.kill();
                let _ = child.wait();
                let _ = stdout_reader.join();
                let _ = stderr_reader.join();
                return Err("process exceeded deadline".to_owned());
            }
            Ok(None) => thread::sleep(Duration::from_millis(2)),
            Err(error) => return Err(format!("process wait failed: {error}")),
        }
    };
    let stdout = stdout_reader
        .join()
        .map_err(|_| "process stdout reader panicked".to_owned())??;
    let stderr = stderr_reader
        .join()
        .map_err(|_| "process stderr reader panicked".to_owned())??;
    let stdout = String::from_utf8(stdout).map_err(|_| "process stdout is not UTF-8".to_owned())?;
    let stderr = String::from_utf8(stderr).map_err(|_| "process stderr is not UTF-8".to_owned())?;
    Ok(ProcessOutput {
        status: status.code().unwrap_or(-1),
        success: status.success(),
        stdout,
        stderr,
    })
}

fn read_capped_output<R: Read>(mut reader: R, limit: usize) -> Result<Vec<u8>, String> {
    let mut output = Vec::new();
    let mut buffer = [0_u8; 8192];
    loop {
        let count = reader
            .read(&mut buffer)
            .map_err(|error| format!("process output read failed: {error}"))?;
        if count == 0 {
            return Ok(output);
        }
        if output.len().saturating_add(count) > limit {
            return Err(format!("process output exceeds {limit} byte limit"));
        }
        output.extend_from_slice(&buffer[..count]);
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
        CancellationToken, JoinError, RunReport, RuntimeLimits, SpawnError, TaskJoinError,
        ThreadJoinError, ThreadRuntimeLimits, ThreadSpawnError, ThreadedRuntime, TimeoutError,
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
    fn fallible_joinable_task_propagates_typed_error() {
        let mut runtime = AsyncRuntime::new();
        let mut handle = runtime
            .spawn_joinable_result(async { Err::<u8, &'static str>("disk failure") })
            .expect("task should fit within runtime limits");
        runtime.run_until_idle();
        assert_eq!(
            block_on(&mut handle),
            Err(TaskJoinError::Failed("disk failure"))
        );
        assert_eq!(block_on(&mut handle), Err(TaskJoinError::AlreadyJoined));
    }

    #[test]
    fn fallible_cancellation_wins_before_task_error() {
        let mut runtime = AsyncRuntime::new();
        let (handle, token) = runtime
            .spawn_joinable_result_cancellable(async { Err::<u8, &'static str>("task failure") })
            .expect("task should fit within runtime limits");
        token.cancel();
        runtime.run_until_idle();
        assert_eq!(block_on(handle), Err(TaskJoinError::Cancelled));
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

    #[test]
    fn threaded_runtime_runs_two_tasks_concurrently() {
        let runtime = ThreadedRuntime::new(ThreadRuntimeLimits {
            max_workers: 2,
            max_tasks: 2,
            max_read_bytes: 1024,
        })
        .unwrap();
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
        let first_barrier = barrier.clone();
        let second_barrier = barrier.clone();
        let first = runtime
            .spawn_blocking(move || {
                first_barrier.wait();
                1_u8
            })
            .unwrap();
        let second = runtime
            .spawn_blocking(move || {
                second_barrier.wait();
                2_u8
            })
            .unwrap();
        barrier.wait();
        assert_eq!(block_on(first), Ok(1));
        assert_eq!(block_on(second), Ok(2));
    }

    #[test]
    fn threaded_runtime_enforces_task_limit_and_reports_panics() {
        let runtime = ThreadedRuntime::new(ThreadRuntimeLimits {
            max_workers: 1,
            max_tasks: 1,
            max_read_bytes: 1024,
        })
        .unwrap();
        let first =
            runtime.spawn_blocking(|| std::thread::sleep(std::time::Duration::from_millis(20)));
        assert!(first.is_ok());
        assert!(matches!(
            runtime.spawn_blocking(|| 7_u8),
            Err(ThreadSpawnError::TaskLimitReached { limit: 1 })
        ));
        let panic_runtime = ThreadedRuntime::new(ThreadRuntimeLimits::default()).unwrap();
        let panic_handle = panic_runtime
            .spawn_blocking(|| panic!("worker failure"))
            .unwrap();
        assert_eq!(block_on(panic_handle), Err(ThreadJoinError::WorkerPanicked));
    }

    #[test]
    fn nonblocking_tcp_exchange_round_trips_with_bounded_response() {
        use std::io::{Read, Write};
        use std::net::TcpListener;
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            let mut request = [0_u8; 4];
            stream.read_exact(&mut request).unwrap();
            assert_eq!(&request, b"ping");
            stream.write_all(b"pong").unwrap();
        });
        let runtime = ThreadedRuntime::new(ThreadRuntimeLimits {
            max_workers: 1,
            max_tasks: 2,
            max_read_bytes: 16,
        })
        .unwrap();
        let response = runtime
            .tcp_exchange(address.to_string(), b"ping".to_vec())
            .unwrap();
        assert_eq!(block_on(response), Ok(Ok(b"pong".to_vec())));
        server.join().unwrap();
    }

    #[test]
    fn nonblocking_tcp_exchange_rejects_oversized_response() {
        use std::io::Write;
        use std::net::TcpListener;
        let listener = TcpListener::bind(("127.0.0.1", 0)).unwrap();
        let address = listener.local_addr().unwrap();
        let server = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream.write_all(b"too-large").unwrap();
        });
        let runtime = ThreadedRuntime::new(ThreadRuntimeLimits {
            max_workers: 1,
            max_tasks: 2,
            max_read_bytes: 4,
        })
        .unwrap();
        let response = runtime
            .tcp_exchange(address.to_string(), Vec::new())
            .unwrap();
        assert_eq!(
            block_on(response),
            Ok(Err("tcp response exceeds 4 byte limit".to_owned()))
        );
        server.join().unwrap();
    }

    #[test]
    fn async_process_adapter_captures_output_cross_platform() {
        let runtime = ThreadedRuntime::new(ThreadRuntimeLimits {
            max_workers: 1,
            max_tasks: 2,
            max_read_bytes: 1024,
        })
        .unwrap();
        #[cfg(windows)]
        let (command, arguments) = (
            "cmd".to_owned(),
            vec!["/C".to_owned(), "echo zap".to_owned()],
        );
        #[cfg(not(windows))]
        let (command, arguments) = (
            "sh".to_owned(),
            vec!["-c".to_owned(), "printf zap".to_owned()],
        );
        let output = runtime.process_async(command, arguments).unwrap();
        let output = block_on(output).unwrap().unwrap();
        assert!(output.success);
        assert_eq!(output.status, 0);
        assert_eq!(output.stdout.trim_end_matches(['\r', '\n']), "zap");
        assert!(output.stderr.is_empty());
    }

    #[test]
    fn async_process_adapter_rejects_capped_output() {
        let runtime = ThreadedRuntime::new(ThreadRuntimeLimits {
            max_workers: 1,
            max_tasks: 2,
            max_read_bytes: 2,
        })
        .unwrap();
        #[cfg(windows)]
        let (command, arguments) = (
            "cmd".to_owned(),
            vec!["/C".to_owned(), "echo zap".to_owned()],
        );
        #[cfg(not(windows))]
        let (command, arguments) = (
            "sh".to_owned(),
            vec!["-c".to_owned(), "printf zap".to_owned()],
        );
        let output = runtime.process_async(command, arguments).unwrap();
        assert_eq!(
            block_on(output),
            Ok(Err("process output exceeds 2 byte limit".to_owned()))
        );
    }

    #[test]
    fn async_process_cancellation_terminates_child() {
        let runtime = ThreadedRuntime::new(ThreadRuntimeLimits {
            max_workers: 1,
            max_tasks: 2,
            max_read_bytes: 1024,
        })
        .unwrap();
        #[cfg(windows)]
        let (command, arguments) = (
            "cmd".to_owned(),
            vec!["/C".to_owned(), "ping 127.0.0.1 -n 8 >NUL".to_owned()],
        );
        #[cfg(not(windows))]
        let (command, arguments) = ("sh".to_owned(), vec!["-c".to_owned(), "sleep 5".to_owned()]);
        let (handle, token) = runtime
            .process_async_cancellable(command, arguments)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(20));
        token.cancel();
        assert_eq!(
            block_on(handle),
            Ok(Err("process cancelled and child terminated".to_owned()))
        );
    }

    #[test]
    fn async_file_read_is_bounded_and_returns_bytes() {
        let suffix = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("zap-async-read-{suffix}.txt"));
        std::fs::write(&path, b"hello").unwrap();
        let runtime = ThreadedRuntime::new(ThreadRuntimeLimits {
            max_workers: 1,
            max_tasks: 2,
            max_read_bytes: 5,
        })
        .unwrap();
        let handle = runtime.read_file_async(&path).unwrap();
        assert_eq!(block_on(handle), Ok(Ok(b"hello".to_vec())));
        std::fs::write(&path, b"oversized").unwrap();
        let oversized = runtime.read_file_async(&path).unwrap();
        assert_eq!(
            block_on(oversized),
            Ok(Err("async read exceeds 5 byte limit".to_owned()))
        );
        std::fs::remove_file(path).unwrap();
    }
}
