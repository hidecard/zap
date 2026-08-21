# Runtime State and Execution Context

**Status:** Implemented first slice for the next Zap release

This document defines the first explicit runtime-state boundary. It does not claim that every evaluator concern has already moved into one object; it records the state that has been migrated and the boundaries that remain deferred.

## Purpose

Each source execution receives its own `ExecutionContext`. The context owns mutable state that must not leak between independent runs, tests, or future runtime instances. This replaces process-global thread-local ownership for module caching, import-cycle tracking, and execution-depth accounting.

## State ownership

| State | Owner | Contract |
|---|---|---|
| Module cache | `RuntimeState` | Cached module values and functions live only for the current execution context. |
| Import-cycle stack | `RuntimeState` | The active module chain is tracked explicitly and is cleared when a run is reset. |
| Execution depth | `RuntimeState` | Nested AST and legacy execution share one bounded counter for the context. |
| Source workspace confinement | Existing evaluator boundary | Path confinement remains a separate compatibility boundary in this slice. |
| Heap statistics and object ownership | Existing value boundary | Memory accounting remains governed by the existing bounded memory contract. |

## ExecutionContext flow

The native entrypoint creates an `ExecutionContext` at the beginning of a run and resets it before evaluating source. The context is passed through the expression parser, AST evaluator, legacy evaluator, function and method calls, object-field initialization, and module loading. Imported modules therefore use the caller's context rather than a process-global cache.

A context can be created independently of another context. Mutating one context's module stack or execution-depth counter does not mutate another context. Resetting a context clears its module cache, import stack, and depth counter before it is reused.

## Safety boundaries

The migrated state is intentionally single-threaded and owned by an execution instance. The implementation does not add `Send`/`Sync` claims, worker sharing, tracing garbage collection, weak references, cumulative byte accounting, or a language-level task scheduler. The current execution-depth limit remains bounded, and the existing AST/legacy compatibility path remains explicit until canonicalization is complete.

## Regression evidence

The runtime-state module includes isolation and reset regressions. The native suite also exercises AST execution, legacy compatibility, module imports, circular-import diagnostics, function calls, method calls, and bounded execution depth through the context-aware call graph.

The acceptance criterion for this first slice is that module and depth state are instance-owned and do not leak across contexts while existing language behavior remains unchanged. Later slices may move workspace, capability, diagnostics, memory, and cancellation state into the same explicit runtime boundary.

## Deferred roadmap

The following work remains separate: complete `RuntimeState`/`ExecutionContext` migration for all hidden state, AST-only canonicalization, first-class function values and `EnvFrame`, object-store and weak-reference policy, per-run memory budgets, typed source-span propagation, and full language-level async task semantics.

See the [English documentation navigation hub](DOCUMENTATION_NAVIGATION_EN.md), the [next-step plan](NEXT_TODO_PLAN_EN.md), and the [language specification](LANGUAGE_SPEC_EN.md) for the maintained contracts.

## References

[1]: DOCUMENTATION_NAVIGATION_EN.md "Zap English documentation navigation"
[2]: NEXT_TODO_PLAN_EN.md "Zap English next-step plan"
[3]: LANGUAGE_SPEC_EN.md "Zap English language specification"
