# Zap v2.1.11 Release Notes

**Release date:** 2026-08-21

## Release summary

Zap v2.1.11 adds the first explicit per-run `RuntimeState` and `ExecutionContext` boundary to the native runtime. The release keeps existing language behavior while preventing module-cache, import-cycle, and execution-depth state from leaking through process-global ownership.

## Highlights

The native entrypoint creates an `ExecutionContext` for each source run and passes it through the AST evaluator, legacy evaluator, expression parser, function calls, method calls, object-field initialization, and module loading. `RuntimeState` owns the current module cache, active import-cycle stack, and bounded execution-depth counter. Context reset behavior is explicit and independently testable.

The repository also adds a bilingual runtime-state contract, navigation links, README architecture/status updates, roadmap acceptance evidence, and documentation-consistency coverage for the new English/Burmese pair.

## Compatibility and deferred scope

This is an internal runtime-boundary improvement. It does not introduce broad async syntax, executor-backed language scheduling, weak references, tracing collection, cumulative per-run byte accounting, or a new `Send`/`Sync` guarantee. Workspace confinement and the existing memory contract remain separate boundaries in this slice. Existing AST/legacy compatibility behavior remains covered by the native suite.

## Verification

The release was validated with Rust 1.75.0 using `cargo fmt --check`, strict `cargo clippy --all-targets --all-features -- -D warnings`, the full native all-target/all-feature test suite, documentation consistency validation, documentation regression harnesses, benchmark regression checks, and `git diff --check`. The native integration suite reports 254 passing tests, with additional runtime-state isolation and reset regressions.

## Upgrade guidance

Users can upgrade by downloading the archive for their operating system and architecture from the [v2.1.11 GitHub Release](https://github.com/hidecard/zap/releases/tag/v2.1.11). Verify the published checksum and signature before installation. No source-language migration is required for this release.

## Documentation

Read the [English runtime-state contract](RUNTIME_STATE_EN.md), [Burmese runtime-state contract](RUNTIME_STATE_MM.md), [English documentation navigation](DOCUMENTATION_NAVIGATION_EN.md), and [Burmese documentation navigation](DOCUMENTATION_NAVIGATION_MM.md). The remaining memory, async, conformance, specification, tooling, and traits work is tracked in the bilingual TODO registers and next-step plans.
