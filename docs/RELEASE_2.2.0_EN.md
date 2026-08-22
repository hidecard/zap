# Zap v2.2.0 Release Notes

**Verified version:** v2.2.0
**Release date:** 2026-08-22
**Release line:** Stable 2.2.x

## Summary

Zap v2.2.0 completes the remaining audit roadmap milestones through M3-LSP-01, M3-DOC-01, and M4-RFC-01. The release strengthens editor semantics, documentation navigation, standard-library stability policy, reproducible verification, registry safety, and runtime state isolation while preserving the canonical AST execution boundary and the explicit production-I/O limitations of the deterministic async runtime.

This release is published from the verified `master` commit after the release candidate passed the repository quality gates and version-consistency checks. Supported release targets remain Linux x86_64, Windows x86_64, and macOS ARM64.

## Runtime and language foundations

The native runtime continues to use the canonical source → lexer → AST parser → evaluator pipeline. Per-run `ExecutionContext` and `RuntimeState` isolate workspace roots, module caches, import-cycle tracking, execution depth, logical memory/task/output budgets, object-store counters, and parent-linked closure frames. First-class callable values and executor-backed `ScheduledFuture` language scheduling remain available.

The async language boundary includes cooperative `task_cancel`, poll-budget `task_join_timeout`, deterministic `Cancelled` and `TimedOut` diagnostics, task admission, readiness observation, and reset isolation. The runtime remains a deterministic language scheduler rather than a production I/O reactor; blocking work, external process interruption, socket readiness, and supervision remain governed by the separate async boundary and deployment contracts.

## Registry, verification, and benchmark hardening

Registry transport now bounds client reads and response size, supports partial and chunked bodies, rejects invalid or truncated `Content-Length` declarations, and normalizes slow-peer failures. The bounded replay job, native platform matrix, deterministic archive checks, target-named logs, benchmark provenance sidecars, variance fields, and registry TCP fixtures are integrated into CI and release preflight.

The Cargo package version remains the single source of truth. The release validator checks `native/Cargo.toml`, the `zap-native` entry in `native/Cargo.lock`, CLI output, tags, changelogs, both READMEs, security metadata, type-check matrices, release notes, and installer/version-agnostic policy surfaces.

## Standard-library stability policy

M3-STDLIB-01 provides a machine-readable catalog for twelve public domains and every cataloged builtin. Each record identifies stability, introduction release, deprecation window, semantic-versioning policy, supported targets, input/output limits, timeout and error behavior, and determinism. The English/Burmese policy pair and catalog regression contract are part of CI and release preflight.

## LSP and VS Code semantic parity

M3-LSP-01 extends `zap lsp` with parser/lexer-backed rename edits, didClose document cleanup, nested and module-aware workspace symbols, catalog-driven completion, and hover/signature metadata for asynchronous builtins. The checked-in VS Code package contains the catalog-aligned TextMate grammar, language configuration, and extension manifest. `scripts/validate_vscode_assets.py` and `scripts/test_lsp_semantic_parity.sh` validate editor/catalog parity in CI and release preflight.

Rename edits preserve lexer spans and do not rewrite string literals. Workspace symbol state is session-owned, and closed documents are removed from the index. These behaviors are covered by focused native LSP regressions.

## Bilingual documentation navigation

M3-DOC-01 completes the English/Burmese audience split. The documentation hubs provide explicit paths for learners, language users, package authors, runtime maintainers, tooling contributors, deployment and security operators, release operators, and language designers. Learner, syntax, specification, standard-library, package-author, runtime, memory, deployment, security, tooling, and release documents carry verified-version metadata and canonical companion links.

The documentation consistency validator now covers the bilingual traits RFC pair in addition to the existing contract pairs. Repository-relative navigation, section parity, code-fence parity, stale-version detection, required files, and README navigation links remain regression-tested.

## Traits and composition RFC

M4-RFC-01 is a design-only milestone. The bilingual RFC defines composition versus single inheritance, required and provided methods, method lookup and visibility, missing/conflicting implementation diagnostics, inheritance migration, hybrid static/dynamic dispatch, rejected alternatives, package compatibility impact, and implementation gates.

> **Compatibility decision:** v2.2.0 does not implement or advertise `trait`, `interface`, `with`, or new conflict-resolution syntax. Existing `class` and single `extends` behavior remain unchanged. A future implementation requires a reviewed RFC, specification ownership, bilingual contracts, conformance fixtures, and an explicit version decision.

## Compatibility boundary

Existing `.zp` programs, single-inheritance classes, canonical AST execution, legacy compatibility-only line-bodied function records, deterministic async scheduling, registry contracts, and standard-library behavior remain governed by their existing specifications and stability records. The release does not silently turn RFC examples into supported syntax and does not claim allocator-level memory measurement, tracing collection, multi-thread-safe task state, or external production deployment.

## Verification

The release candidate passed Rust formatting with the pinned toolchain, strict Clippy with `-D warnings`, the full native all-target/all-feature test suite, the M3-LSP-01 semantic-parity harness, documentation consistency and its positive/negative regression harness, specification ownership validation, benchmark and registry contract gates, and `git diff --check`. The release-version validator passed with expected version `2.2.0`, and the release preflight passed its required-file, contract, deployment-policy, and version checks before publication.

## References

[1]: ../README.md — English project status, installation, architecture, and release assets.
[2]: ../README_MM.md — Burmese project status and installation guidance.
[3]: DOCUMENTATION_NAVIGATION_EN.md — English audience and contract navigation.
[4]: LANGUAGE_SPEC_EN.md — Canonical language semantics and compatibility ownership.
[5]: STDLIB_POLICY_EN.md — Public standard-library stability policy.
[6]: ASYNC_LSP_EN.md — Async and LSP boundary contract.
[7]: TRAITS_RFC_EN.md — M4-RFC-01 traits and composition design record.
[8]: ../CHANGELOG_EN.md — Full English release history.
