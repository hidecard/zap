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

The runtime remains deterministic: tasks are stored in submission order and are polled by the existing budgeted executor. `spawn_joinable` propagates task-admission errors instead of silently discarding them. The implementation uses Rust 1.75-compatible standard-library primitives and does not create worker threads.

## Example

```rust
let mut runtime = AsyncRuntime::new();
let handle = runtime.spawn_joinable(async { 42 }).unwrap();
runtime.run_until_idle();
let value = block_on(handle).unwrap();
```

The runtime must be driven before joining. If a task is still pending, the handle remains pending and the configured poll budget continues to apply.

## Safety and remaining scope

Runtime limits remain explicit through `RuntimeLimits::max_tasks` and `RuntimeLimits::max_polls_per_run`. The existing cancellation token and deterministic delay primitives remain available to the runtime foundation. Timeout propagation, task error values, language-level task builtins, and formatter/LSP/VS Code synchronization are later v2.1-D slices and are not implied by this first joinable-handle implementation.

Regression coverage verifies successful output joining, deterministic readiness, and propagation of task-limit errors. Cross-platform behavior is limited to standard-library executor semantics and must still be covered by the release verification matrix.
