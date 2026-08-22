# MemoryBudget and ObjectStore Contract

**Design status:** M2-MEM-01 foundation design

**Verified baseline:** Zap v2.1.14.

## Purpose

Zap needs bounded, observable memory behavior without claiming that the current `Rc<RefCell>` runtime is a tracing garbage collector. This contract defines the first run-owned accounting boundary. It separates logical budget admission from Rust allocator measurements and keeps object identity, raw addresses, and process-global counters out of public results.

## Ownership

Each `ExecutionContext` owns one `MemoryBudget` and one `ObjectStore` through `RuntimeState`. A context reset clears both stores. Independent contexts must never observe one another's logical bytes, task admissions, output usage, or object counters. The LSP `LspState` remains a separate per-session state and is not charged to a language execution budget.

| Component | Responsibility | Public guarantee |
|---|---|---|
| `MemoryBudget` | Account logical bytes, admitted tasks, and bounded output | Deterministic admission and typed limit errors; no allocator-size claim |
| `ObjectStore` | Track run-owned object allocations and live/deallocated counts | Counters are per run, monotonic where specified, and contain no raw addresses |
| `Value` validation | Check text, collection, graph, and node limits | Existing value-limit errors remain fail-closed |
| Rust allocator | Actual process memory | Not measured or exposed by this contract |

## Logical accounting units

The budget uses logical byte units: UTF-8 payload length for text, a fixed per-entry charge for list/map slots, and a fixed object-field charge for object entries. The charge constants are deterministic implementation constants, not measurements of Rust heap layout. A request that would overflow an accounting counter or exceed a configured limit is rejected before admission with a stable error. Accounting is saturating internally and never wraps into an apparently available budget.

The first foundation exposes explicit methods for reserving and releasing logical bytes, admitting and completing tasks, and reserving output. Later value constructors may add precise charges at their public boundaries; they must not bypass the budget by silently converting overflow into success.

## Default limits

The initial defaults remain conservative and compatible with existing limits. They are configurable only through the explicit runtime-state API in this slice; no new environment variable or user-facing syntax is introduced until a separate configuration contract exists.

| Limit | Meaning | Failure boundary |
|---|---|---|
| `max_bytes` | Total logical bytes reserved by one execution | `memory budget exceeded` |
| `max_tasks` | Concurrent/admitted logical task count | `task budget exceeded` |
| `max_output_bytes` | Output bytes admitted by one execution | `output budget exceeded` |

## Object lifecycle

Production object construction receives the current context-owned object store. Allocation increments `object_allocations` and `live_objects`; dropping the tracked field storage decrements `live_objects` and increments `object_deallocations`. Test-only or compatibility constructors may create an untracked standalone object, but they must not reintroduce process-global production statistics. `memory_stats()` reports the current execution store when called through an execution context and reports stable zero counters for a context that has not allocated objects.

Object counters are diagnostic evidence, not a reclamation guarantee. Cycles remain explicitly breakable through the existing checked field APIs. Public weak references and automatic tracing collection remain deferred and must continue to be reported as unsupported/not implemented.

## Errors and determinism

Budget failures use stable, operation-specific text and must propagate through CLI and LSP diagnostic boundaries without panic. The same sequence of admissions must produce the same counters and failure point on repeated runs. A failed reservation does not consume budget. A release cannot underflow usage. Reset returns all counters and usage to their initial state.

## Compatibility boundary

This slice does not add first-class callable values, parent-linked `EnvFrame` bindings, executor-backed language scheduling, forced interruption of foreign blocking calls, or tracing garbage collection. Existing `read_lines`/`write_lines` compatibility behavior and the canonical AST boundary remain unchanged.

## Acceptance evidence

The milestone is complete when independent contexts have isolated budgets and object stores; reset clears every counter; object allocation/deallocation diagnostics are deterministic; byte/task/output over-limit cases fail closed; nested AST/module execution charges the same context; JSON/LSP/CLI error propagation remains panic-free; and the full native suite plus cross-platform CI remain green.

## References

* [Runtime-state contract](RUNTIME_STATE_EN.md)
* [Memory model](MEMORY_MODEL_EN.md)
* [Remaining TODO register](PDF_REMAINING_TODO_EN.md)
