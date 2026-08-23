# Standard-Library Stability Policy

## Status and scope

This policy defines the compatibility contract for every public standard-library domain and its directly exposed builtins. The machine-readable source is the native [`stdlib_catalog.rs`](../native/src/stdlib_catalog.rs) catalog, while this document explains how users and maintainers interpret the metadata. Runtime dispatch remains centralized in the evaluator; the catalog does not create a second implementation path.

The policy applies to the current release line, **v2.2.7**, and is intended to be reviewed whenever a public builtin is added, changed, deprecated, or removed.

## Stability model

Every public domain and builtin has one stability label, one introduction release, one deprecation-window value, one semantic-versioning rule, one platform-support declaration, explicit input/output limits, a timeout policy, an error contract, and a `determinism_class`. A new public API must not be added to the catalog with any field omitted. Catalog schema version 2 also retains the schema-version-1 `deterministic` boolean as a compatibility view.

| Label | Meaning | Compatibility consequence |
|---|---|---|
| `stable` | Supported public behavior for the release line | Bug fixes and compatible additions may ship in a minor release; breaking semantic changes require a major release or an approved migration plan |
| `experimental` | Opt-in behavior still subject to design change | The documentation must identify the opt-in boundary and migration risk; promotion to `stable` requires a catalog and regression review |
| `platform-specific` | Supported only on named target families | The platform matrix is normative; unsupported targets must fail with a stable diagnostic rather than silently emulating behavior |
| `deprecated` | Existing behavior retained during migration | The catalog must name a deprecation window and replacement; removal is prohibited before that window closes |

The current public catalog marks all released domains and builtins as `stable`, introduced in `2.1.14`, with no active deprecation window. Future entries may use another label only when their documentation and tests satisfy the corresponding requirements.

## Determinism taxonomy

`determinism_class` is more precise than the legacy boolean. `pure` means the result is a function of explicit inputs and has no runtime or external-state dependency. `input-deterministic` means validated inputs determine a repeatable transformation, including parsing, encoding, or duration decomposition. `runtime-dependent` means the result may depend on process state, scheduling, platform configuration, or the current clock. `external-io` means the operation reads, writes, or coordinates with the filesystem, environment, network, process table, or another external system.

The retained `deterministic` field is `true` only for `pure` and `input-deterministic` entries, and `false` for `runtime-dependent` and `external-io` entries. New tooling should consume `determinism_class`; the boolean remains available for consumers that still read schema-version-1 metadata. Domain classifications are conservative defaults, and builtin-level exceptions are explicit: path and structured-log builders are `pure`; URL parsing/encoding/decoding and duration transforms are `input-deterministic`; clock and environment/configuration access are `runtime-dependent` or `external-io` as appropriate.

## Public domain policy

The following table is the normative domain-level summary. Individual builtins inherit the limits and error contract of their domain unless the catalog gives a narrower value.

| Public domain | Stability | Since | Deprecation window | Semver rule | Supported targets | Input limit | Output limit | Timeout policy | Determinism class |
|---|---|---|---|---|---|---|---|---|---|
| `text` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB text argument | 8 KiB text result | not applicable | pure |
| `math` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | bounded integer arguments | bounded integer result | not applicable | pure |
| `collections` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 MiB logical collection graph | 8 MiB logical collection graph | not applicable | pure |
| `filesystem` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 MiB path/content input | 8 MiB text/line output | not applicable | external-io |
| `json` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 MiB JSON input | 8 MiB JSON output | not applicable | pure |
| `system` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB environment/path input | 8 KiB text or structured result | not applicable | runtime-dependent |
| `time` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | checked integer milliseconds | checked duration map | not applicable | runtime-dependent |
| `logging` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB message and 64 fields | 64 KiB encoded record | not applicable | external-io |
| `runtime` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | bounded diagnostic request | bounded statistics map | not applicable | runtime-dependent |
| `async` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | run-owned task and poll budgets | bounded task result | cooperative cancellation or poll-budget timeout | runtime-dependent |
| `network` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | 8 KiB URL and 8 MiB request body | 8 MiB response body | bounded connect/read/write; server wait 10 seconds | external-io |
| `process` | stable | 2.1.14 | none | minor-compatible | Linux, Windows, macOS ARM64 | text command, text arguments, 1 MiB output | 1 MiB captured stdout/stderr | bounded child wait and cleanup | external-io |

All public domains use the stable runtime diagnostic contract. Invalid types, malformed values, path escapes, oversized values, unavailable platform operations, and exceeded logical budgets fail closed. The `determinism_class` field describes the source of repeatability and dependency; it does not claim that external clocks, network peers, process scheduling, or filesystem latency are deterministic. In particular, a `pure` or `input-deterministic` builtin can be listed within a domain whose default is `runtime-dependent` or `external-io` when the builtin-level implementation has no such dependency. The public `sqrt` helper accepts a non-negative integer and returns the rounded integer square root; `sort` returns a cloned number-only or text-only list in ascending order; and `assert` returns `none` on truthy input or a deterministic runtime error containing the supplied message and observed value. The runtime `memory_stats()` record reports `cycle_policy=explicit_clear_object_fields`; public weak references and automatic tracing collection remain unsupported/deferred.

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

For v2.2.7, all cataloged standard-library domains and builtins remain **stable**, have no active deprecation window, follow the minor-compatible default, support the release-target matrix, and expose bounded error behavior. Their schema-2 determinism classes distinguish pure/input-driven transformations from runtime-dependent and external-I/O behavior; the legacy boolean remains only as a compatibility view. Namespace imports and remote standard-library packages remain separate future milestones; traits-based composition is documented by the design-only M4-RFC-01 and remains deferred.
