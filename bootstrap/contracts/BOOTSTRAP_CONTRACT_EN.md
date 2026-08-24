# Zap Bootstrap Contract

**Status:** Normative B0 contract for Zap v2.10.0

This contract defines the staged path from the current Rust reference implementation to a Zap-only self-hosted ecosystem. It does not claim that Zap is already self-hosted. The operating-system loader, executable format, filesystem, and explicitly documented platform seed remain the boundary conditions for every stage.

## Bootstrap stages

| Stage | Required capability | Rust/Cargo status | Release wording |
|---|---|---|---|
| B0 | Rust implementation owns reference behavior and canonical fixtures | Required | Rust reference/native implementation |
| B1 | Zap-owned lexer and parser reproduce the B0 token/AST contracts | B0 seed builds candidate | Zap bootstrap compiler foundation |
| B2 | Zap-owned diagnostics and type checker reproduce B0 acceptance and rejection behavior | B0/B1 bridge may remain | Zap bootstrap compiler foundation |
| B3 | Zap-owned standard library, typed IR, package resolver, and test runner produce deterministic offline artifacts | Small B0 bridge may remain | Zap-owned compiler pipeline in transition |
| B4 | Zap compiler rebuilds its own source from a documented platform seed | Not required in compiler path | Fully Zap-only self-hosted compiler |

Until B4 passes, documentation and release notes MUST NOT describe Zap as fully self-hosted or fully Zap-only.

## Ownership rules

The lexer owns tokenization and source spans. The parser owns syntax and AST construction. The type checker owns static acceptance and rejection. Diagnostics own stable codes, severity, locations, notes, and help. The evaluator or VM owns execution. The package layer owns manifests, lockfiles, resolution, hashes, signatures, and offline policy. The platform seed owns operating-system interaction. No layer may silently redefine another layer's contract.

Each normative rule MUST have an English specification section, a Burmese counterpart, an owner, and at least one deterministic fixture. A behavior without a fixture is provisional and cannot be advertised as stable.

## Artifact contract

Canonical artifacts are UTF-8 JSON objects with lexicographically ordered object keys, deterministic array order, no timestamps, no pointer addresses, no host paths outside explicitly supplied source names, and an explicit schema version. The required artifact families are token streams, AST snapshots, diagnostics, typed IR, manifest/lockfile data, test results, and release manifests.

A producer MUST reject malformed input and invalid artifact schema versions rather than guessing. A consumer MUST reject unknown required fields and MUST preserve source locations when the relevant artifact family carries them.

## Capability contract

Compiler-core operations are pure by default. Source and fixture reads are bounded and explicitly provided. Package resolution is offline by default and may access a network only through an explicit user command and host capability. Process execution, environment reads, arbitrary file writes, clocks, randomness, and sockets are not ambient compiler capabilities.

All path operations MUST reject absolute paths, traversal components, symlink escapes, oversized inputs, and platform-specific ambiguity according to the relevant host boundary contract. Resource limits are part of the observable diagnostic behavior.

## Reproducibility and differential gates

For identical source bytes, contract versions, compiler inputs, and platform-seed version, repeated runs MUST produce identical token, AST, diagnostic, typed-IR, and artifact hashes. B1 and later implementations MUST be compared against B0 on the owned corpus. A mismatch is recorded as either a defect or an explicit compatibility decision; it is never hidden by changing a fixture without a contract record.

## Version policy

Language version, compiler version, standard-library version, and each artifact schema version are independent fields in `VERSIONS.toml`. A language-semantic change requires a specification update, bilingual documentation parity, fixtures, a changelog entry, and an explicit compatibility decision. An artifact-schema change requires migration notes and a separate schema-version decision.

## Current status

Zap v2.10.0 is **B0**. The current native Rust lexer, parser, evaluator, standard library, registry, and host boundaries are the reference owners. The bootstrap directories establish the contract and corpus needed for B1 work; they do not replace the native implementation.
