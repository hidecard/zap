# Section A — Compiler and Self-Hosting Acceptance Contract

**Status:** Design and acceptance contract; it does not claim that any deferred section-A item is complete. Zap remains B0 and Rust remains the reference/compiler/runtime owner.

## Purpose

Section A covers the work required to move from corpus-limited bootstrap evidence toward a self-hosted compiler. The work must proceed in ordered gates. A bounded fixture or a single syntax example cannot promote a candidate to complete compiler ownership.

> A section-A item is complete only when its syntax, semantics, negative behavior, diagnostics, deterministic artifacts, bilingual documentation, ownership record, and cross-platform regression evidence are all accepted at the same gate.

## Ownership rule

Rust is authoritative for semantics, diagnostics, typed IR, package/build behavior, VM execution, and supported release targets until the corresponding acceptance gate passes. Bootstrap implementations may be exercised as candidates, but candidate output must be compared with the Rust reference across the owned corpus and must not silently replace the reference pipeline.

## Ordered section-A gates

| Gate | Work package | Minimum acceptance evidence | Ownership result |
|---|---|---|---|
| A1 | Complete type-inference contract | Type lattice, expression coverage, positive/negative fixtures, inference determinism, and Rust/candidate differential checks | Candidate remains provisional until full expression coverage passes |
| A2 | Broader basic-type inference | Cross-product matrix for `text`, `number`, `bool`, `list`, `map`, and `none`, including direct values, expressions, calls, branches, and invalid combinations | No broad claim from literal-only slices |
| A3 | Generic declarations | Grammar, AST, scope, constraints, arity, substitution, recursion limits, diagnostics, and runtime boundary tests | User-defined generic syntax remains deferred until all are implemented |
| A4 | Collection inference | Homogeneous and heterogeneous list/map rules, nested values, empty collections, key/value constraints, mutation effects, and aliases | Existing exact-literal and element slices do not complete this gate |
| A5 | Nested and compound inference | Recursive maps/lists, deeper expressions, compound guards, branch joins, and short-circuit behavior | Each recursive/compound rule needs paired acceptance/rejection evidence |
| A6 | Flow and mutation analysis | Loop joins, mutation, reassignment invalidation, alias facts, closure capture, and post-branch restoration | Narrowing facts must never survive an invalidating write |
| A7 | Parser coverage | Arbitrary valid programs plus Unicode, malformed, overflow, indentation, delimiter, and determinism corpora | Owned-corpus parser evidence is not arbitrary-program ownership |
| A8 | Diagnostic parity | Error kind/code, message normalization, source line/column, JSON shape, LSP range conversion, and failure exit behavior | Rust and bootstrap must describe the same failure |
| A9 | General typed IR | Stable schema, all supported AST forms, inferred types, spans, determinism, and byte-for-byte/reference semantic comparison | The current annotated-declaration artifact remains reference-only |
| A10 | Package/build ownership | Manifest, lockfile, resolver, dependency validation, offline build, test runner, and reproducible package artifacts implemented by bootstrap | Foundation checks alone do not transfer ownership |
| A11 | VM execution ownership | Bootstrap-produced IR executes with behavior, limits, errors, and security boundaries equivalent to Rust | Native VM remains authoritative until equivalence is accepted |
| A12 | Platform-seed acceptance | Linux x86_64, macOS ARM64, and Windows x86_64 bootstrap builds/runs are reproducible and artifact-verified | Platform evidence must cover the same source and toolchain contract |
| A13 | B4 self-rebuild | Bootstrap compiler builds the documented seed, rebuilt compiler rebuilds itself, outputs are deterministic, and the result passes all gates | Only this gate permits B4/self-hosted wording |

## Required evidence for every gate

Each gate requires a Rust reference fixture set, a bootstrap candidate fixture set, positive and negative cases, deterministic repeated runs, stable diagnostics, malformed-input safety where applicable, a machine-readable ownership record, and synchronized English/Burmese documentation. The gate must state the exact scope it proves and the scope it intentionally leaves deferred.

## Non-claims

No section-A checkpoint may claim a Zap-only compiler, complete inference, complete parser coverage, full diagnostic parity, typed-IR ownership, package/build ownership, VM ownership, platform-seed ownership, or B4 self-hosting unless the corresponding gate has passed. A release may publish bounded evidence while these larger gates remain open, but its notes must preserve the B0 boundary.

## Release rule

The section-A program may produce intermediate releases only for independently verified bounded increments. The release for the completed A program must not be tagged until A1 through A13 have passed, the bilingual contract and ownership ledger are synchronized, the exact committed preflight is clean, all cross-platform release jobs pass, and public checksums, manifests, provenance, and signatures are independently verified.

## Next implementation decision

The A2 design gate now has a small Rust-reference-backed exact-expression matrix covering arithmetic, text addition, boolean logic, comparison, and result construction. A1 complete inference and A2 broader cross-product coverage remain open. Evidence must continue to expand incrementally rather than treating the existing literal, constructor, or exact-expression slices as complete inference.
