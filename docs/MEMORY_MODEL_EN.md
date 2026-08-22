# Zap Memory Model

## Scope

Zap values use explicit ownership primitives. Function closures and object fields are reference-counted, while mutable state is guarded by `RefCell` on the single-threaded interpreter path.

## Ownership contract

`Value::Object` owns a reference-counted field map. Cloning an object value clones the handle; it does not copy the object fields. This makes object aliasing observable and keeps field mutation localized to the shared field map.

Cyclic object graphs are not reclaimed by reference counting alone. Embedders and runtime cleanup paths must explicitly clear cyclic fields before releasing the final external object handle. The runtime exposes `clear_object_fields()` for this boundary and `object_field_count()` for diagnostics and regression tests.

Function closures use parent-linked `EnvFrame` nodes. A frame owns its local bindings and points to the lexical parent; lookup walks from the current frame toward the parent, while assignment updates the nearest existing binding and creates a local binding when no parent binding exists. Callable values retain the frame chain through reference-counted handles, so a returned closure keeps its captured state alive after the outer function returns. Recursive calls continue to resolve through the function table, and closure mutation is synchronized back to the captured frame without exposing raw addresses.

The interpreter remains single-threaded for mutable object access. Object field reads and writes use checked `try_borrow`/`try_borrow_mut` accessors. If a read/write conflict occurs, the runtime returns a typed `BorrowError` with stable code `ZAP-BORROW-001` instead of panicking. `clear_object_fields()` and `object_field_count()` therefore return a fallible result at this boundary. This API does not claim thread-safe ownership or provide a tracing garbage collector. Any future multi-threaded runtime must introduce an explicit synchronization or tracing-collector design rather than sharing these handles across threads.

The runtime exposes the zero-argument `memory_stats()` builtin as a bounded diagnostic record. Its stable map fields include `live_objects`, `object_allocations`, `object_deallocations`, `max_text_bytes`, `max_collection_items`, and `max_value_nodes`. Context-backed calls additionally expose `cleanup_attempts`, `cleanup_successes`, `cleanup_failures`, `validation_runs`, `max_bytes`, `used_bytes`, `max_tasks`, `admitted_tasks`, `max_output_bytes`, and `used_output_bytes`. The record also states that public weak references are `unsupported_public_api` and tracing collection is `not_implemented`; these values are explicit capability information, not promises of automatic cycle collection. Logical object admissions use a fixed base charge plus a fixed per-field charge; these are deterministic accounting units, not Rust heap-layout measurements.

At public builtin boundaries, runtime values are checked without recursively looping through cyclic object graphs. One text value is limited to 8 MiB, one list or map is limited to 100,000 entries, and one traversed value graph is limited to 100,000 nodes. Violations return deterministic memory-limit errors. These checks are bounded validation, not a claim that every internal allocation is globally accounted for.

## Regression guarantee

The native test `parent_linked_closures_preserve_mutation_after_outer_return` verifies that a returned callable retains and updates its captured binding across calls. The native test `cyclic_object_graph_can_be_explicitly_broken` creates a self-referential object, verifies that the cycle is observable, clears the fields, and verifies that the field allocation can be released. The `conflicting_object_borrows_return_typed_failures` regression holds one mutable field borrow and verifies deterministic `BorrowError` results for competing reads and writes without a panic. M2-MEM-02 regressions verify per-context validation and cleanup counters, failed-cleanup accounting, output/task admission, and reset detachment from objects retained by a prior run. These are memory-contract tests, not a claim that arbitrary cycles are automatically collected.

## Future work

Weak-reference support and a tracing collector remain future milestones. Process-wide heap telemetry and automatic reclamation of arbitrary cycles are also intentionally outside the current contract. EnvFrame lifetime is reference-counted and may participate in cycles when captured values retain callables; cycle breaking remains an explicit cleanup responsibility. They must be designed separately from the current single-threaded `Rc<RefCell>` boundary.
