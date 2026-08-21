# Zap v2.1.13 Release Notes

**Verified version:** v2.1.13
**Release date:** 2026-08-21

## Summary

Zap v2.1.13 completes the next hidden-state migration slice after canonical AST execution. Workspace confinement is now owned by the per-run `RuntimeState`, and the LSP document map is owned by an explicit per-session `LspState` rather than production process-global thread-local storage.

## Runtime-state changes

The native evaluator records one canonical workspace root in `ExecutionContext` and retains it across nested function, block, and module execution. Filesystem built-ins receive the same context-aware boundary, including metadata, atomic writes, text/line reads and writes, and existence checks. A context reset clears the workspace root together with module cache, import-cycle, and execution-depth state.

The LSP stdio server creates one `LspState` for each server session. Completion, signature help, hover, definition, formatting, document symbols, and workspace symbols read from that state. Independent LSP states cannot observe one another's open documents. The test-only compatibility wrapper is not used by production server execution.

## Compatibility boundary

The canonical AST path remains normative for parser-owned programs and local modules. The legacy line interpreter remains an explicit compatibility-only path for older line-bodied function records. This release does not claim first-class callable values, parent-linked `EnvFrame` binding cells, cumulative memory budgets, broad language async syntax, or traits/interfaces semantics.

## Verification

The release candidate passed Rust formatting checks, strict Clippy with `-D warnings`, the full native all-target/all-feature test suite with 254 integration tests, workspace and LSP isolation regressions, documentation consistency validation, the documentation regression harness, and `git diff --check`.

## References

* [Runtime-state contract](RUNTIME_STATE_EN.md)
* [AST foundation status](P0_FOUNDATION_STATUS_EN.md)
* [Documentation navigation](DOCUMENTATION_NAVIGATION_EN.md)
* [Full changelog](../CHANGELOG_EN.md)
