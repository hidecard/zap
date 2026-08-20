# Zap Async Runtime

## v2.1-D first slice

Zap's async runtime currently provides a deterministic, single-threaded executor foundation. The first v2.1-D slice adds **joinable task handles** without creating worker threads or changing the synchronous language surface.

> A joinable task is a future submitted to the runtime together with a handle that resolves to the task's output after the runtime polls it to completion.

## API contract

| API | Contract |
|---|---|
| `AsyncRuntime::spawn_joinable(future)` | Submits a future and returns `Result<JoinHandle<T>, SpawnError>`. |
| `JoinHandle<T>::is_ready()` | Reports whether the task has produced its output. |
| `JoinHandle<T>` as a future | Resolves to `Result<T, JoinError>`. Polling before completion remains pending; polling after consumption returns `AlreadyJoined`. |
| `SpawnError::TaskLimitReached` | Returned when `max_tasks` would be exceeded. |
| `AsyncRuntime::spawn_joinable_cancellable(future)` | Returns `(JoinHandle<T>, CancellationToken)` and resolves cancellation as `JoinError::Cancelled`. |
| `timeout_ticks(future, ticks)` | Returns `Ok(output)` when the inner future completes before the deterministic poll deadline, otherwise `Err(TimeoutError)`. |
| `AsyncRuntime::spawn_joinable_result(future)` | Submits a `Future<Output = Result<T, E>>` and returns `TaskJoinHandle<T, E>`, preserving the task's typed failure. |
| `TaskJoinHandle<T, E>` as a future | Resolves to `Ok(T)`, `Err(TaskJoinError::Failed(E))`, or `Err(TaskJoinError::AlreadyJoined)`. |
| `AsyncRuntime::spawn_joinable_result_cancellable(future)` | Returns `(TaskJoinHandle<T, E>, CancellationToken)`; cancellation is checked before polling and therefore wins over a task result. |

The runtime remains deterministic: tasks are stored in submission order and are polled by the existing budgeted executor. `spawn_joinable` propagates task-admission errors instead of silently discarding them. The implementation uses Rust 1.75-compatible standard-library primitives and does not create worker threads.

## Example

```rust
let mut runtime = AsyncRuntime::new();
let handle = runtime.spawn_joinable(async { 42 }).unwrap();
runtime.run_until_idle();
let value = block_on(handle).unwrap();
```

The runtime must be driven before joining. If a task is still pending, the handle remains pending and the configured poll budget continues to apply.

## Structured cancellation

`spawn_joinable_cancellable(future)` returns a join handle and a cloneable `CancellationToken`. Calling `cancel()` marks the token atomically. The task wrapper checks the token before polling the inner future, so a cancelled task is not polled and its handle resolves to `Err(JoinError::Cancelled)`. This preserves structured ownership: the caller retains a handle for completion and an explicit token for cancellation.

## Timeout propagation

`timeout_ticks(future, ticks)` measures deadlines in executor polls rather than wall-clock time. The inner future is polled first; each pending poll consumes one tick. If the inner future remains pending when no ticks remain, the wrapper resolves to `Err(TimeoutError)`. A completed inner future propagates as `Ok(value)`, and no threads or sleeping system calls are introduced.

## Task error propagation

`spawn_joinable_result(future)` accepts a future whose output is `Result<T, E>`. The runtime stores either the successful value or the exact typed error, and the caller receives `TaskJoinError::Failed(E)` without string conversion or panic-based propagation. The cancellable variant checks its token before polling the inner future; when cancellation is already requested, the handle resolves to `TaskJoinError::Cancelled` and the task error is not produced.

## Safety and remaining scope

Runtime limits remain explicit through `RuntimeLimits::max_tasks` and `RuntimeLimits::max_polls_per_run`. Structured cancellation, poll-based timeout propagation, and typed task error propagation are included in this slice; language-level task builtins and formatter/LSP/VS Code synchronization remain later v2.1-D work.

Regression coverage verifies successful output joining, deterministic readiness, propagation of task-limit errors and typed task failures, cancellation precedence before the inner future is polled, repeated joins, and both timeout and completion paths. Cross-platform behavior is limited to standard-library executor semantics and must still be covered by the release verification matrix.
