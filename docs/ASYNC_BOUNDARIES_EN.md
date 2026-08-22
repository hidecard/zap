# Zap Async Boundaries

**Status:** Normative runtime-boundary guidance for Zap v2.1.14

## Purpose

Zap provides a deterministic, single-threaded task executor for language-runtime experiments and bounded integration tests, together with explicitly submitted bounded production I/O adapters. This document separates those contracts from a full production asynchronous reactor that Zap may adopt later. A deterministic task runner must not be presented as a complete network reactor, thread scheduler, or interruption mechanism.

## Current deterministic executor

The current executor stores tasks in insertion order and polls them with a no-op waker. `run_until_idle()` uses the configured maximum poll budget, while `run_with_budget()` returns a `RunReport` containing the number of polls, pending-task count, and whether the budget was exhausted. The executor can enforce a maximum task count and a maximum number of polls per run. Language `async fn` calls now schedule their result in the caller's `RuntimeState` and return a `ScheduledFuture` task handle; `await` and `task_join` drive the context-owned executor before consuming the result, while `task_is_ready` observes readiness without polling.

| Contract | Current behavior |
|---|---|
| Scheduling | Cooperative, single-threaded polling in deterministic task order; language task handles are owned by the current `ExecutionContext`. |
| Wake-up | No operating-system reactor; the executor uses a no-op waker. |
| Fairness | Bounded by the poll budget and task order; no latency guarantee is made. |
| Shared state | Runtime task handles use `Rc<RefCell<...>>`; this is not `Send`/`Sync`. |
| Failure | Join handles preserve task failure or cancellation as an explicit result; a language task that cannot be found after reset is not observable in the new run. |
| Cancellation | Cancellation tokens are checked before polling the wrapped future; cancellation is cooperative. |
| Limits | `max_tasks` and `max_polls_per_run` prevent unbounded executor work. |

The executor is suitable for deterministic language semantics, context-owned `ScheduledFuture` handles, unit tests, conformance fixtures, and small in-process tasks that never block. It is not suitable for claiming production-grade socket readiness, parallel CPU execution, preemptive fairness, or forced interruption of arbitrary code.

## Production boundary

The current production boundary provides bounded file, TCP, and process adapters through explicitly submitted worker operations; it does not provide a general operating-system reactor. A full production asynchronous I/O layer would wait for readiness events, register and remove file descriptors, handle timers, and wake tasks without busy polling. That reactor remains outside the current stable contract and must define its supported platforms, readiness semantics, timer precision, shutdown behavior, and resource limits before it is exposed as a stable Zap API.

Blocking system calls require an explicit adapter boundary. A blocking filesystem operation, process wait, DNS lookup, or foreign-function call must not be executed on the reactor thread. The production design must either use a bounded blocking pool or an OS-specific cancellable operation. A cancellation request may stop waiting for a result, but it cannot be described as killing an arbitrary blocking syscall unless the adapter provides a documented, safe interruption guarantee.

Multi-thread scheduling is also a separate boundary. The current `Rc<RefCell>` task state cannot be moved across worker threads. A production scheduler would require `Send`/`Sync`-safe task state, ownership transfer rules, a defined memory-ordering model, deterministic shutdown, and explicit limits for worker count and queue depth. These changes are semantic and architectural; they must not be inferred from the current executor.

## Cancellation and timeout semantics

Cancellation is cooperative and has a defined precedence: a cancellation-aware wrapper checks its token before polling the inner future. The language `task_cancel(future)` API requests cancellation for a pending context-owned task and returns whether the request was accepted; `task_join` then reports the deterministic `Cancelled` failure. `task_join_timeout(future, poll_budget)` drives at most the supplied poll budget and reports deterministic `TimedOut` failure when the task is not ready. A cancelled task completes with a cancellation result rather than silently disappearing. Timeouts should be implemented as a race between the operation and a timer future, with the timer and operation both participating in shutdown. A timeout must not imply that an underlying blocking operation was forcibly terminated.

Task errors propagate through join handles as typed results. A caller that drops a join handle may stop observing the result, but dropping the handle does not provide a general cancellation guarantee unless the API explicitly documents that behavior. Production APIs must specify whether cancellation is best effort, whether resources are closed before completion, and how errors from reactor shutdown are reported.

## M2-VERIFY-02 platform-native matrix

GitHub Actions runs the focused matrix on the native Linux x86_64, Windows x86_64, and macOS ARM64 targets rather than treating a single Linux run as cross-platform evidence. Each target executes the target binary's exact unit regressions for worker admission, bounded TCP exchange, oversized request/response rejection, process output/status/cancellation, bounded file reads, newline-byte preservation, and directory rejection. Linux and macOS additionally execute the Unix symlink rejection case because it exercises the shared Unix filesystem boundary; Windows records the platform limitation instead of claiming equivalent symlink-policy evidence.

The matrix also runs `scripts/test_platform_archive.sh` on every runner. That regression creates a small CRLF-containing tree, builds the deterministic tar.gz archive twice, compares the bytes, verifies sorted member names, and compares the extracted payload bytes. Each runner writes target-named logs containing the target triple, runner operating system, Rust/Cargo versions, exact test names, archive result, and final status. The matrix proves the repository's documented behavior on the supported CI targets; it does not claim that arbitrary third-party binaries or foreign operating-system calls are portable.

## Stability rules

The deterministic executor and the context-owned language scheduling boundary are the stable baseline for v2.1.x. New APIs must identify whether they are deterministic-only, reactor-backed, or blocking-adapted. Documentation and diagnostics must use those same terms. No release note or benchmark may claim parallel scheduling or production non-blocking I/O until the corresponding reactor and platform gates exist.

A future production implementation must add, at minimum:

1. A reactor abstraction with platform-specific readiness backends and a deterministic test backend.
2. Timer registration, cancellation, and monotonic-clock rules.
3. A bounded blocking adapter with shutdown and cancellation behavior documented per operation.
4. `Send`/`Sync`-safe scheduler state or an explicit single-thread-only API boundary.
5. Integration tests for socket readiness, process and filesystem adapters, timeout propagation, cancellation races, shutdown, and resource limits on every supported platform.

## Verification

The current contract is verified by native tests for bounded polling, task limits, join results, context-owned language-task readiness/completion, scheduler reset isolation, language `task_cancel` and `task_join_timeout`, cancellation precedence, timeout behavior, child-process cancellation, and the M2-VERIFY-02 target-native filesystem/process/socket/archive cases. These tests verify deterministic semantics and the documented adapter boundaries only; they do not certify a production reactor or forced cancellation of arbitrary blocking work.
