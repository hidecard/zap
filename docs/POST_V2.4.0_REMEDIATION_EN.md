# Post-v2.4.0 Remediation and Provenance Record

**Record baseline:** v2.11.4 current-master follow-up; v2.4.0 remains immutable

**Record status:** Current master follow-up record; v2.4.0 remains immutable

**Scope:** This record identifies corrections and documentation work that may land after the published v2.4.0 tag. It does not rewrite the v2.4.0 release or imply that later commits are present in its archives.

## Provenance boundary

The published [`v2.4.0` release](https://github.com/hidecard/zap/releases/tag/v2.4.0) and annotated tag are immutable. Users who install v2.4.0 receive the exact binaries, checksums, signatures, provenance metadata, and documentation associated with that tag. Later `master` commits require their own version decision and release validation.

| Boundary | Meaning |
|---|---|
| v2.4.0 tag | Published and immutable; no later correction is backported into the tag. |
| Current master | Development surface for follow-up fixes and new features; not automatically a release artifact. |
| Next release | Must include a changelog entry, bilingual notes, version-consistent metadata, tests, and the complete preflight gate. |

## Corrective queue

| Area | Current state | Required evidence before release |
|---|---|---|
| Documentation links and baselines | The bilingual navigation, active usage material, and release metadata are being synchronized with the current release line. | Repository-wide relative-link validation, bilingual parity, and accurate historical labels. |
| Language semantics | Optional annotations, bounded generic forms, modules, classes, Result/Option, and structured diagnostics are implemented. Generic declarations, traits, pattern matching, and a typed intermediate representation remain deferred. | Normative specification changes, parser/evaluator conformance fixtures, stable diagnostics, LSP parity, and migration notes. |
| Async runtime | Deterministic single-thread scheduling, cancellation, timeouts, and poll budgets are implemented. | Stream/channel semantics, structured cancellation, external-I/O lifecycle tests, and an explicit worker/isolate boundary before any production concurrency claim. |
| Web runtime | The user-managed scaffold, bounded development server, static/SPA assets, DTO/auth/rate-limit contracts, and SQLite-first migrations are available. | Production listener behavior, shutdown/backpressure, authentication/session persistence, database adapter tests, deployment evidence, and security review. |
| LSP/editor | Full synchronization, diagnostics, hover, completion, definitions, formatting, file-local rename, and workspace symbols are available. | Incremental synchronization and cross-file semantic refactoring must be implemented and tested before being advertised. |

## Release policy

A follow-up change must not be described as part of v2.4.0 solely because it appears on a branch or pull request. The release version is authoritative only after the annotated tag, successful cross-platform workflow, signed artifacts, checksums, provenance, and published-release verification agree. Historical release notes remain historical; current behavior belongs in the current specification, guide, and release notes.

## References

1. [Zap v2.4.0 release](https://github.com/hidecard/zap/releases/tag/v2.4.0).
2. [Normative language specification](LANGUAGE_SPEC_EN.md).
3. [Framework Foundation boundary](FRAMEWORK_EN.md).
4. [Documentation navigation](DOCUMENTATION_NAVIGATION_EN.md).
