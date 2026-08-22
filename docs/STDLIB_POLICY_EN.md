# Standard-Library Stability Policy

## Status and scope

This policy defines the compatibility contract for every public standard-library domain and its directly exposed builtins. The machine-readable source is the native [`stdlib_catalog.rs`](../native/src/stdlib_catalog.rs) catalog, while this document explains how users and maintainers interpret the metadata. Runtime dispatch remains centralized in the evaluator; the catalog does not create a second implementation path.

The policy applies to the current release line, **v2.1.14**, and is intended to be reviewed whenever a public builtin is added, changed, deprecated, or removed.

## Stability model

Every public domain and builtin has one stability label, one introduction release, one deprecation-window value, one semantic-versioning rule, one platform-support declaration, explicit input/output limits, a timeout policy, an error contract, and a determinism flag. A new public API must not be added to the catalog with any field omitted.

| Label | Meaning | Compatibility consequence |
|---|---|---|
| `stable` | Supported public behavior for the release line | Bug fixes and compatible additions may ship in a minor release; breaking semantic changes require a major release or an approved migration plan |
| `experimental` | Opt-in behavior still subject to design change | The documentation must identify the opt-in boundary and migration risk; promotion to `stable` requires a catalog and regression review |
| `platform-specific` | Supported only on named target families | The platform matrix is normative; unsupported targets must fail with a stable diagnostic rather than silently emulating behavior |
| `deprecated` | Existing behavior retained during migration | The catalog must name a deprecation window and replacement; removal is prohibited before that window closes |

The current public catalog marks all released domains and builtins as `stable`, introduced in `2.1.14`, with no active deprecation window. Future entries may use another label only when their documentation and tests satisfy the corresponding requirements.

## Public domain policy

The following table is the normative domain-level summary. Individual builtins inherit the limits and error contract of their domain unless the catalog gives a narrower value.

| Public domain | Stability | Since | Deprecation window | Semver rule | Supported targets | Input limit | Output limit | Timeout policy | Deterministic |
|---|---|---|---|---|---|---|---|---|---|
| `text` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB text argument | 8 KiB text result | not applicable | yes |
| `math` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | bounded integer arguments | bounded integer result | not applicable | yes |
| `collections` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 MiB logical collection graph | 8 MiB logical collection graph | not applicable | yes |
| `filesystem` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 MiB path/content input | 8 MiB text/line output | not applicable | yes |
| `json` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 MiB JSON input | 8 MiB JSON output | not applicable | yes |
| `system` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB environment/path input | 8 KiB text or structured result | not applicable | yes |
| `time` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | checked integer milliseconds | checked duration map | not applicable | yes |
| `logging` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB message and 64 fields | 64 KiB encoded record | not applicable | yes |
| `runtime` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | bounded diagnostic request | bounded statistics map | not applicable | yes |
| `async` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | run-owned task and poll budgets | bounded task result | cooperative cancellation or poll-budget timeout | yes |
| `network` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB URL and 8 MiB request body | 8 MiB response body | bounded connect/read/write; server wait 10 seconds | yes |
| `process` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | text command, text arguments, 1 MiB output | 1 MiB captured stdout/stderr | bounded child wait and cleanup | yes |

All public domains use the stable runtime diagnostic contract. Invalid types, malformed values, path escapes, oversized values, unavailable platform operations, and exceeded logical budgets fail closed. The `deterministic` field describes repeatable Zap-level behavior and does not claim that external clocks, network peers, process scheduling, or filesystem latency are deterministic.

## API evolution and semver rules

A **minor-compatible** change may add a new builtin, add a new optional field to a returned record, clarify a diagnostic without changing its stable code, or fix a bug while preserving accepted valid programs. Such changes require catalog metadata, English/Burmese documentation, a regression test, and an updated compatibility record.

A **major-breaking** change is required when an existing valid program changes meaning, an accepted input becomes invalid, a stable result field is removed or changes type, a documented diagnostic contract is removed, or a platform guarantee is narrowed. The change must first be described in the bilingual compatibility template and approved before implementation.

The catalog's `since` value is the first release in which the public behavior is supported. It must not be changed to the current release merely because documentation was edited. The `deprecation_window` value is `none` for active stable APIs and must contain a concrete migration period for deprecated APIs.

## Deprecation and removal

Deprecation is a documentation and tooling event, not a silent runtime change. A deprecated entry must retain its old dispatch behavior, name its replacement, specify the first release of deprecation, and state the minimum release in which removal may occur. Removal requires a major-version decision or an explicitly approved compatibility exception, plus a migration example in both language trees.

No current public standard-library domain or builtin is deprecated. The catalog tests reject missing metadata, duplicate names, unknown domains, non-stable release entries, and absent limits or error contracts.

## Platform support and limits

The release-target platform value `linux,windows,macos-arm64` covers the supported CI and release targets. A future Unix-only or Windows-only API must use the corresponding catalog value and provide a target-native regression. Unsupported behavior must not be represented as portable merely because the source compiles on another target.

The limits in this policy are admission and safety boundaries. They are not performance guarantees. Filesystem and JSON operations retain their documented 8 MiB safety boundary; network responses retain their documented 8 MiB boundary; registry transport separately enforces its 16 MiB response bound; process output remains bounded at 1 MiB; and run-owned memory/task/output budgets remain logical accounting rather than allocator or tracing-collector measurements.

## Verification and change checklist

The M3-STDLIB-01 acceptance gate requires the catalog metadata tests, the standard-library security corpus, the full native test suite, strict Clippy, documentation consistency, specification ownership, and `git diff --check`. A public API change must also update the relevant English and Burmese index entries, this policy pair, the compatibility record, and the release roadmap.

The public surface remains discoverable through the [English standard-library index](STDLIB_INDEX_EN.md) and [Burmese standard-library index](STDLIB_INDEX_MM.md). The catalog is intentionally deterministic: each public builtin appears once and belongs to one declared domain.

## Current release decision

For v2.1.14, all cataloged standard-library domains and builtins are **stable**, have no active deprecation window, follow the minor-compatible default, support the release-target matrix, and expose bounded deterministic error behavior. Namespace imports, remote standard-library packages, and traits-based composition remain separate future milestones.
