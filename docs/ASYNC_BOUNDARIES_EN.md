# Zap Async Boundaries

**Status:** Normative runtime-boundary guidance for Zap v2.1.10

## Purpose

Zap provides a deterministic, single-threaded task executor for language-runtime experiments and bounded integration tests, together with explicitly submitted bounded production I/O adapters. This document separates those contracts from a full production asynchronous reactor that Zap may adopt later. A deterministic task runner must not be presented as a complete network reactor, thread scheduler, or interruption mechanism.

## Current deterministic executor

The current executor stores tasks in insertion order and polls them with a no-op waker. `run_until_idle()` uses the configured maximum poll budget, while `run_with_budget()` returns a `RunReport` containing the number of polls, pending-task count, and whether the budget was exhausted. The executor can enforce a maximum task count and a maximum number of polls per run.

| Contract | Current behavior |
|---|---|
| Scheduling | Cooperative, single-threaded polling in deterministic task order. |
| Wake-up | No operating-system reactor; the executor uses a no-op waker. |
| Fairness | Bounded by the poll budget and task order; no latency guarantee is made. |
| Shared state | Runtime task handles use `Rc<RefCell<...>>`; this is not `Send`/`Sync`. |
| Failure | Join handles preserve task failure or cancellation as an explicit result. |
| Cancellation | Cancellation tokens are checked before polling the wrapped future; cancellation is cooperative. |
| Limits | `max_tasks` and `max_polls_per_run` prevent unbounded executor work. |

The executor is suitable for deterministic language semantics, unit tests, conformance fixtures, and small in-process tasks that never block. It is not suitable for claiming production-grade socket readiness, parallel CPU execution, preemptive fairness, or forced interruption of arbitrary code.

## Production boundary

The current production boundary provides bounded file, TCP, and process adapters through explicitly submitted worker operations; it does not provide a general operating-system reactor. A full production asynchronous I/O layer would wait for readiness events, register and remove file descriptors, handle timers, and wake tasks without busy polling. That reactor remains outside the current stable contract and must define its supported platforms, readiness semantics, timer precision, shutdown behavior, and resource limits before it is exposed as a stable Zap API.

Blocking system calls require an explicit adapter boundary. A blocking filesystem operation, process wait, DNS lookup, or foreign-function call must not be executed on the reactor thread. The production design must either use a bounded blocking pool or an OS-specific cancellable operation. A cancellation request may stop waiting for a result, but it cannot be described as killing an arbitrary blocking syscall unless the adapter provides a documented, safe interruption guarantee.

Multi-thread scheduling is also a separate boundary. The current `Rc<RefCell>` task state cannot be moved across worker threads. A production scheduler would require `Send`/`Sync`-safe task state, ownership transfer rules, a defined memory-ordering model, deterministic shutdown, and explicit limits for worker count and queue depth. These changes are semantic and architectural; they must not be inferred from the current executor.

## Cancellation and timeout semantics

Cancellation is cooperative and has a defined precedence: a cancellation-aware wrapper checks its token before polling the inner future. A cancelled task completes with a cancellation result rather than silently disappearing. Timeouts should be implemented as a race between the operation and a timer future, with the timer and operation both participating in shutdown. A timeout must not imply that an underlying blocking operation was forcibly terminated.

Task errors propagate through join handles as typed results. A caller that drops a join handle may stop observing the result, but dropping the handle does not provide a general cancellation guarantee unless the API explicitly documents that behavior. Production APIs must specify whether cancellation is best effort, whether resources are closed before completion, and how errors from reactor shutdown are reported.

## Stability rules

The deterministic executor is the stable baseline for v2.1.x. New APIs must identify whether they are deterministic-only, reactor-backed, or blocking-adapted. Documentation and diagnostics must use those same terms. No release note or benchmark may claim parallel scheduling or production non-blocking I/O until the corresponding reactor and platform gates exist.

A future production implementation must add, at minimum:

1. A reactor abstraction with platform-specific readiness backends and a deterministic test backend.
2. Timer registration, cancellation, and monotonic-clock rules.
3. A bounded blocking adapter with shutdown and cancellation behavior documented per operation.
4. `Send`/`Sync`-safe scheduler state or an explicit single-thread-only API boundary.
5. Integration tests for socket readiness, process and filesystem adapters, timeout propagation, cancellation races, shutdown, and resource limits on every supported platform.

## Verification

The current contract is verified by native tests for bounded polling, task limits, join results, cancellation precedence, timeout behavior, and child-process cancellation. These tests verify deterministic semantics only; they do not certify a production reactor or forced cancellation of arbitrary blocking work.
