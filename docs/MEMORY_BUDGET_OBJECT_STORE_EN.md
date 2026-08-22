# MemoryBudget and ObjectStore Contract

**Design status:** M2-MEM-01 foundation design

**Verified baseline:** Zap v2.2.1.

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

The budget uses logical byte units: UTF-8 payload length for text, a fixed per-entry charge for list/map slots, a fixed object base charge, and a fixed object-field charge for object entries. The charge constants are deterministic implementation constants, not measurements of Rust heap layout. A request that would overflow an accounting counter or exceed a configured limit is rejected before admission with a stable error. Accounting never wraps into an apparently available budget, and failed reservations do not consume the resource whose reservation failed.

The runtime exposes explicit methods for reserving logical bytes and object charges, admitting and completing logical tasks, and reserving output. Canonical AST object construction charges the current context before allocation, while text-producing builtin results charge the output budget after successful evaluation. These charges are logical accounting units and must not be interpreted as allocator-size measurements.

## Default limits

The initial defaults remain conservative and compatible with existing limits. They are configurable only through the explicit runtime-state API in this slice; no new environment variable or user-facing syntax is introduced until a separate configuration contract exists.

| Limit | Meaning | Failure boundary |
|---|---|---|
| `max_bytes` | Total logical bytes reserved by one execution | `memory budget exceeded` |
| `max_tasks` | Concurrent/admitted logical task count | `task budget exceeded` |
| `max_output_bytes` | Output bytes admitted by one execution | `output budget exceeded` |

## Object lifecycle

Production object construction receives the current context-owned object store. Allocation increments `object_allocations` and `live_objects`; dropping the tracked field storage decrements `live_objects` and increments `object_deallocations`. Explicit cleanup records attempts, successes, and borrow failures; bounded validation records validation runs. Reset replaces the active store, so objects retained from a prior run cannot mutate the next run's counters. Test-only or compatibility constructors may create an untracked standalone object, but they must not reintroduce process-global production statistics. `memory_stats()` reports the current execution store and budget when called through an execution context and reports stable zero counters and default budget fields for a context-free compatibility call.

Object counters are diagnostic evidence, not a reclamation guarantee. Cycles remain explicitly breakable through the existing checked field APIs. Public weak references and automatic tracing collection remain deferred and must continue to be reported as unsupported/not implemented.

## Errors and determinism

Budget failures use stable, operation-specific text and map to `ZAP-MEMORY-001` through the structured diagnostic boundary. The same sequence of admissions must produce the same counters and failure point on repeated runs. A failed reservation does not consume the resource whose reservation failed. A release cannot underflow usage. Reset returns all active counters and usage to their initial state while detaching the old object store.

## Compatibility boundary

This slice does not add first-class callable values, parent-linked `EnvFrame` bindings, executor-backed language scheduling, forced interruption of foreign blocking calls, or tracing garbage collection. Existing `read_lines`/`write_lines` compatibility behavior and the canonical AST boundary remain unchanged.

## Acceptance evidence

M2-MEM-02 is complete when independent contexts have isolated budgets and object stores; reset detaches old stores and clears active counters; object allocation/deallocation, validation, and cleanup diagnostics are deterministic; byte/object/task/output over-limit cases fail closed; repeated module execution reuses one context cache; JSON/LSP/CLI error propagation remains panic-free; and the full native suite plus cross-platform CI remain green.

## References

* [Runtime-state contract](RUNTIME_STATE_EN.md)
* [Memory model](MEMORY_MODEL_EN.md)
* [Remaining TODO register](PDF_REMAINING_TODO_EN.md)
