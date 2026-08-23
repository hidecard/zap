# Runtime State and Execution Context

**Navigation:** [Documentation hub](DOCUMENTATION_NAVIGATION_EN.md) · [Learning guide](LEARN_ZAP_EN.md) · [Language specification](LANGUAGE_SPEC_EN.md) · [Memory model](MEMORY_MODEL_EN.md) · [Memory budget/ObjectStore](MEMORY_BUDGET_OBJECT_STORE_EN.md) · [Async/LSP guide](ASYNC_LSP_EN.md) · [Deployment boundaries](DEPLOYMENT_EN.md)

**Status:** Implemented migration foundation for Zap v2.2.7

This document defines the first explicit runtime-state boundary. It does not claim that every evaluator concern has already moved into one object; it records the state that has been migrated and the boundaries that remain deferred.

## Purpose

Each source execution receives its own `ExecutionContext`. The context owns mutable state that must not leak between independent runs, tests, or future runtime instances. This replaces process-global thread-local ownership for module caching, import-cycle tracking, and execution-depth accounting.

## State ownership

| State | Owner | Contract |
|---|---|---|
| Module cache | `RuntimeState` | Cached module values and functions live only for the current execution context. |
| Import-cycle stack | `RuntimeState` | The active module chain is tracked explicitly and is cleared when a run is reset. |
| Execution depth | `RuntimeState` | Nested AST and legacy execution share one bounded counter for the context. |
| Source workspace confinement | `RuntimeState` | The canonical workspace root is fixed for the context, inherited by nested module/function calls, and cleared on run reset. |
| LSP open documents | `LspState` | Each LSP server session owns its document map; independent server states do not share open-document contents. |
| Heap statistics and object ownership | `ObjectStore` in `RuntimeState` | Production allocation/deallocation, validation, and cleanup counters are per-run; no raw addresses or tracing-collector guarantee is exposed. |
| Logical memory/task/output budget | `MemoryBudget` in `RuntimeState` | Deterministic byte/object/task/output admission and fail-closed reserve/release APIs are available to context-aware runtime boundaries. |

## ExecutionContext flow

The native entrypoint creates an `ExecutionContext` at the beginning of a run and resets it before evaluating source. The context is passed through the expression parser, AST evaluator, legacy evaluator, function and method calls, object-field initialization, and module loading. Function values retain parent-linked `EnvFrame` capture chains backed by live binding cells, so nested functions can outlive their defining call while preserving deterministic lexical lookup and mutation. Each AST function and method frame also inherits the defining module’s base directory; a relative import executed inside that callable resolves from that directory rather than from the process working directory. Imported modules therefore use the caller's context rather than a process-global cache. The first AST execution that establishes a workspace records its canonical root in `RuntimeState`; nested execution retains that root instead of replacing it with the process working directory. Filesystem built-ins receive the same context-aware boundary.

A context can be created independently of another context. Mutating one context's module stack or execution-depth counter does not mutate another context. Resetting a context clears its module cache, import stack, depth counter, budget, and active object-store counters before it is reused. The active object store is replaced on reset, so objects retained from the previous run cannot mutate the new run's statistics.

## Safety boundaries

The migrated state is intentionally single-threaded and owned by an execution instance or LSP server session. The implementation does not add `Send`/`Sync` claims, worker sharing, tracing garbage collection, or weak references. `MemoryBudget` provides logical byte/task/output accounting; it is not an allocator measurement. The context-owned language scheduler now provides bounded task admission, explicit terminal states, cancellation, timeout, and one-time join release under the documented eager scheduled-value contract. Object/frame borrows, logical accounting, and canonical AST equality use checked bounded paths; LSP rename scope analysis also fails closed on an empty stack. The current execution-depth limit remains bounded. Parser-owned source uses canonical AST execution; the line interpreter remains explicit and compatibility-only for older line-bodied function records.

## Regression evidence

The runtime-state module includes workspace, budget, object-store isolation, stable snapshot, and reset-detachment regressions. The evaluator verifies context-aware `memory_stats()` fields, output/task admission, validation and cleanup lifecycle counters, current-run object-store reads, non-panicking task-join fallback behavior, checked AST object-member reads, and cycle-safe equality. The LSP module includes independent-server document isolation coverage and checked rename scope-stack handling. The native suite also exercises AST execution, legacy compatibility, module imports, nested relative imports from called functions, circular-import diagnostics, function calls, method calls, filesystem confinement, bounded execution depth, explicit task terminal transitions, and one-time task-budget release through the context-aware call graph.

The acceptance criterion for this migration slice is that module, depth, and workspace state are instance-owned and do not leak across execution contexts, while LSP document maps are session-owned and do not leak across server states. Existing language and editor behavior must remain unchanged. Later slices may move capability, diagnostics, memory, and cancellation state into additional explicit boundaries.

## Deferred roadmap

The following work remains separate: allocator-level measurement, public weak references, automatic tracing collection, typed source-span propagation, interruption of foreign blocking calls beyond the supported adapter boundary, and broader production async semantics.

See the [English documentation navigation hub](DOCUMENTATION_NAVIGATION_EN.md), the [next-step plan](NEXT_TODO_PLAN_EN.md), and the [language specification](LANGUAGE_SPEC_EN.md) for the maintained contracts.

## References

[1]: DOCUMENTATION_NAVIGATION_EN.md "Zap English documentation navigation"
[2]: NEXT_TODO_PLAN_EN.md "Zap English next-step plan"
[3]: LANGUAGE_SPEC_EN.md "Zap English language specification"
