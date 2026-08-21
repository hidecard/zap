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
| `spawn(future)` | Language-level facade that accepts a `Future` and returns a task future. |
| `task_is_ready(task)` | Returns `true` for the current eager language-level future representation; rejects non-future values. |
| `task_join(task)` | Consumes a language-level task future and returns its completed value; rejects non-future values. |
| `async_capabilities()` | Returns a stable map describing deterministic executor, worker, network, process, cancellation, limit, and deferred language-level boundaries. |

The runtime remains deterministic: tasks are stored in submission order and are polled by the existing budgeted executor. `spawn_joinable` propagates task-admission errors instead of silently discarding them. The implementation uses Rust 1.75-compatible standard-library primitives and does not create worker threads.

## Examples

Runtime-level joining remains available:

```rust
let mut runtime = AsyncRuntime::new();
let handle = runtime.spawn_joinable(async { 42 }).unwrap();
runtime.run_until_idle();
let value = block_on(handle).unwrap();
```

The first language-level facade is available in `.zp` programs:

```zap
async fn load() -> number:
    return 42

let task = spawn(load())
let ready: bool = task_is_ready(task)
let value: number = task_join(task)
```

The current evaluator computes async function bodies eagerly and stores their result in the existing `Future` value. Therefore `spawn` establishes the language-level task contract and `task_join`/`task_is_ready` provide a stable surface, while executor-backed scheduling remains a later integration slice. Runtime-level handles must still be driven before joining; if a runtime task is pending, its handle remains pending and the configured poll budget continues to apply.

## Structured cancellation

`spawn_joinable_cancellable(future)` returns a join handle and a cloneable `CancellationToken`. Calling `cancel()` marks the token atomically. The task wrapper checks the token before polling the inner future, so a cancelled task is not polled and its handle resolves to `Err(JoinError::Cancelled)`. This preserves structured ownership: the caller retains a handle for completion and an explicit token for cancellation.

## Timeout propagation

`timeout_ticks(future, ticks)` measures deadlines in executor polls rather than wall-clock time. The inner future is polled first; each pending poll consumes one tick. If the inner future remains pending when no ticks remain, the wrapper resolves to `Err(TimeoutError)`. A completed inner future propagates as `Ok(value)`, and no threads or sleeping system calls are introduced.

## Task error propagation

`spawn_joinable_result(future)` accepts a future whose output is `Result<T, E>`. The runtime stores either the successful value or the exact typed error, and the caller receives `TaskJoinError::Failed(E)` without string conversion or panic-based propagation. The cancellable variant checks its token before polling the inner future; when cancellation is already requested, the handle resolves to `TaskJoinError::Cancelled` and the task error is not produced.

## Boundary capability report

The zero-argument `async_capabilities()` builtin makes the runtime boundary observable without claiming that every adapter is part of the language-level scheduler. Its stable fields distinguish the single-threaded poll-budget executor, fixed worker adapter, bounded non-blocking TCP adapter, bounded process adapter, terminate-then-drain process cancellation, eager language-level futures, deferred language-level scheduling/cancellation/timeout, and unsupported interruption of arbitrary foreign blocking calls. It also reports the current default worker, read, socket, process-output, and timeout limits, and states that resource-limit preflight is `enforced` with `typed_deterministic` invalid-limit errors.

The report is intentionally descriptive and deterministic. It does not start workers, open sockets, spawn processes, or change task scheduling. Applications must still choose the appropriate adapter and enforce their own deployment policy at the operating-system boundary.

## Cross-platform matrix

P0-05-C runs the same focused async matrix on Linux x86_64, Windows x86_64, and macOS ARM64 through the build job in `.github/workflows/ci.yml`. The checked-in `scripts/test_p005c_async_matrix.sh` records the target triple, runner OS, Rust/Cargo versions, and exact test filters in a target-named artifact. The matrix covers worker concurrency, invalid-limit preflight, loopback TCP round trips and response/request bounds, platform-native process output and cancellation, and bounded regular-file reads. A runner/toolchain limitation must be recorded as a versioned limitation artifact rather than silently skipped.

## Safety and remaining scope

Runtime limits remain explicit through `RuntimeLimits::max_tasks` and `RuntimeLimits::max_polls_per_run`. Structured cancellation, poll-based timeout propagation, typed task error propagation, the first language-level task facade (`spawn`, `task_join`, and `task_is_ready`), the descriptive `async_capabilities()` report, typed limit validation, TCP request-size preflight, and the reproducible three-target focused matrix are included in this slice. Executor-backed language-level scheduling, language-level cancellation/timeout controls, and formatter/LSP/VS Code synchronization remain later v2.1-D work.

Regression coverage verifies successful output joining, deterministic readiness, propagation of task-limit errors and typed task failures, cancellation precedence before the inner future is polled, repeated joins, timeout and completion paths, zero/oversized resource-limit rejection, TCP request-size rejection before queue admission, and target-native process/file/socket behavior through the P0-05-C matrix script. Cross-platform evidence is retained as target-named CI artifacts.
