# Zap Memory Model

## Scope

Zap values use explicit ownership primitives. Function closures and object fields are reference-counted, while mutable state is guarded by `RefCell` on the single-threaded interpreter path.

## Ownership contract

`Value::Object` owns a reference-counted field map. Cloning an object value clones the handle; it does not copy the object fields. This makes object aliasing observable and keeps field mutation localized to the shared field map.

Cyclic object graphs are not reclaimed by reference counting alone. Embedders and runtime cleanup paths must explicitly clear cyclic fields before releasing the final external object handle. The runtime exposes `clear_object_fields()` for this boundary and `object_field_count()` for diagnostics and regression tests.

The interpreter remains single-threaded for mutable object access. This API does not claim thread-safe ownership or provide a tracing garbage collector. Any future multi-threaded runtime must introduce an explicit synchronization or tracing-collector design rather than sharing these handles across threads.

## Regression guarantee

The native test `cyclic_object_graph_can_be_explicitly_broken` creates a self-referential object, verifies that the cycle is observable, clears the fields, and verifies that the field allocation can be released. This is a memory-contract test, not a claim that arbitrary cycles are automatically collected.

## Future work

Heap statistics, allocation counters, cycle diagnostics, weak references, and a tracing collector remain future milestones. They must be designed separately from the current `Rc<RefCell>` contract.
