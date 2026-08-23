# MemoryBudget and ObjectStore Contract

**Design status:** M2-MEM-02 logical accounting and rollback slice

**Verified baseline:** Zap v2.2.7.

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

The budget uses deterministic logical byte units rather than Rust heap-layout measurements. A scalar or wrapper has a fixed base charge; text adds its UTF-8 payload length; lists and maps add fixed container/slot charges, with map-key bytes included; objects add a fixed base, class-name bytes, and fixed per-field storage; callable values add function metadata, parameter/default metadata, and the reachable live closure-frame bindings. Nested values are traversed with object, frame, and function identity guards, so cycles are bounded and shared references are not counted repeatedly within one charge calculation. A request that would overflow an accounting counter or exceed a configured limit is rejected before admission with a stable error. Accounting never wraps into an apparently available budget.

The runtime exposes explicit methods for reserving logical bytes and object charges, admitting and completing logical tasks, reserving output, and taking/restoring byte/output checkpoints. Canonical AST literals, containers, cloned access results, builtin results, and registered callable captures are charged at their materialization boundaries. Object construction charges its finalized field shape after defaults, explicit fields, and initializers have run, so default and nested values are covered by their own AST charges and object storage is charged by the final shape. A failed AST expression, builtin dispatch, or constructor restores its byte/output checkpoint; task admission remains governed by the separate task lifecycle contract. These charges are logical accounting units and must not be interpreted as allocator-size measurements.

## Default limits

The initial defaults remain conservative and compatible with existing limits. They are configurable only through the explicit runtime-state API in this slice; no new environment variable or user-facing syntax is introduced until a separate configuration contract exists.

| Limit | Meaning | Failure boundary |
|---|---|---|
| `max_bytes` | Total logical bytes reserved by one execution | `memory budget exceeded` |
| `max_tasks` | Concurrent/admitted logical task count | `task budget exceeded` |
| `max_output_bytes` | Output bytes admitted by one execution | `output budget exceeded` |

## Object lifecycle

Production object construction receives the current context-owned object store. Allocation increments `object_allocations` and `live_objects`; dropping the tracked field storage decrements `live_objects` and increments `object_deallocations`. Explicit cleanup records attempts, successes, and borrow failures; bounded validation records validation runs. Reset replaces the active store, so objects retained from a prior run cannot mutate the next run's counters. Test-only or compatibility constructors may create an untracked standalone object, but they must not reintroduce process-global production statistics. `memory_stats()` reports the current execution store and budget when called through an execution context and reports stable zero counters and default budget fields for a context-free compatibility call.

Object counters are diagnostic evidence, not a reclamation guarantee. The current cycle policy is `explicit_clear_object_fields`: cycles remain explicitly breakable through the existing checked field APIs, while public weak references and automatic tracing collection remain deferred and are reported as unsupported/not implemented. Lexical-frame snapshots, insertion, assignment, and import synchronization use checked operations as well, returning deterministic `BorrowError` results instead of panicking when a frame is already borrowed. Canonical AST equality uses a checked, cycle-safe comparator with visited object-pair short-circuiting and the same `max_value_nodes` bound; object-field borrow conflicts propagate through `==` and `!=`.

## Errors and determinism

Budget failures use stable, operation-specific text and map to `ZAP-MEMORY-001` through the structured diagnostic boundary. The same sequence of admissions must produce the same counters and failure point on repeated runs. A failed reservation does not consume the resource whose reservation failed. A release cannot underflow usage. Reset returns all active counters and usage to their initial state while detaching the old object store.

## Compatibility boundary

This slice does not add executor-backed language scheduling, forced interruption of foreign blocking calls, weak references, or tracing garbage collection. It accounts for the existing first-class callable values and parent-linked `EnvFrame` bindings without changing their semantics. Existing `read_lines`/`write_lines` compatibility behavior and the canonical AST boundary remain unchanged.

## Acceptance evidence

The M2-MEM-02 implementation slice is complete when independent contexts have isolated budgets and object stores; reset detaches old stores and clears active counters; nested values, callable captures/default metadata, finalized object fields, and builtin outputs have deterministic logical charges; failed expression/builtin/constructor reservations roll back byte/output usage; object allocation/deallocation, validation, and cleanup diagnostics are deterministic; byte/object/task/output over-limit cases fail closed; repeated module execution reuses one context cache; JSON/LSP/CLI error propagation remains panic-free; and the full native suite plus cross-platform CI remain green. Focused evaluator and value regressions cover the new accounting paths; the repository-wide gate remains the final acceptance check.

## References

* [Runtime-state contract](RUNTIME_STATE_EN.md)
* [Memory model](MEMORY_MODEL_EN.md)
* [Remaining TODO register](PDF_REMAINING_TODO_EN.md)
