# Zap P2 Progress — Ecosystem Foundation

## Current status

Zap P1 Language Core was released as `v1.0.0`. P2 has now started with a local package-manager foundation that remains deterministic and does not require a remote registry.

| Milestone | Status | Notes |
|---|---|---|
| Manifest dependency declarations | Implemented | `[dependencies]` entries are parsed and validated, including local path specifications. |
| Canonical lockfile | Implemented | `zap.lock` is generated in stable package/dependency order and renders local paths canonically. |
| `zap add` command | Implemented | Adds a string-valued dependency, sorts the dependency section, rejects duplicates, and invalidates the old lockfile. |
| Registry-ready package metadata | Implemented | Optional `description`, `authors`, `license`, `repository`, and 64-character hexadecimal SHA-256 `checksum` fields are validated deterministically. |
| Registry index, HTTPS transport, and cache foundation | Implemented foundation | JSON index validation, deterministic exact and version-range selection, local/HTTPS index and artifact transport, deterministic content-addressed cache, SHA-256 enforcement, and offline reuse are supported through explicit configuration. |
| `zap install` | Implemented | Validates the current manifest and canonical lockfile; when configured, it resolves registry entries and verifies or populates the checksum-checked cache. |
| `zap update` | Implemented | Regenerates the canonical lockfile deterministically, validates the complete local graph, and performs configured registry/cache integrity checks. |
| Async runtime and language syntax | Implemented foundation | Deterministic single-thread executor, `spawn`, `run_until_idle`, and `block_on` are available internally. `async fn`, `await`, deterministic `Future` values, and evaluator integration are implemented; timers, cancellation, and multi-thread scheduling remain planned. |
| LSP/editor integration | Implemented foundation | `zap lsp` provides stdio JSON-RPC framing, initialize/shutdown, text synchronization, parser-backed diagnostics, parser-span hover for top-level declarations, and source-prefix/context-aware completion. Formatting, go-to-definition, and workspace indexing remain planned. |

## Local install/update contract

```bash
zap install [project-dir]
zap update [project-dir]
```

`zap install` is validation-only for the project manifest and lockfile. When `ZAP_REGISTRY_INDEX` is configured, it also validates exact registry entries and checksum-verified cache state; `ZAP_OFFLINE=1` permits only already-cached packages and never downloads new sources. `zap update` regenerates `zap.lock` using canonical ordering and performs the same configured registry/cache checks. For local path dependencies, it recursively validates nested manifests in lexicographic order and rejects cycles before writing or accepting the lockfile.

## `zap add` contract

```bash
zap add <name> <version> [project-dir]
```

The command updates `zap.toml` deterministically. It rejects empty or whitespace-containing names, duplicate dependency names, and invalid single-line requirements. If `zap.lock` exists, it is removed because the manifest no longer matches the lockfile. Running `zap lock` regenerates the canonical lockfile.

## Verification

The native test suite covers successful additions, lexicographic ordering, duplicate rejection, lockfile invalidation, install validation, update regeneration, idempotence, stale-lock rejection, CLI help exposure, valid local packages, missing local package manifests, nested local packages, deterministic cycle diagnostics, registry index validation, local and HTTPS transport policy, cache population, checksum mismatch rejection, offline reuse, publish validation before network requests, signed-index verification and tamper rejection, deterministic cache pruning, authenticated local registry persistence, version-range selection, async parsing and evaluation, Future unwrapping, timer/cancellation behavior, resource limits, poll budgets, one-poll suspension, LSP capability negotiation, context-filtered completion, parser-backed hover, definitions, workspace symbols, and deterministic formatting. A dependency may use `name = { path = "../local-lib" }`; the path is resolved relative to the consuming project, must contain a valid `zap.toml` with package name and version metadata, and is represented canonically in `zap.lock`. Local path dependencies are traversed depth-first in sorted dependency order. A repeated canonical path on the active traversal stack produces an error such as `dependency cycle detected: left -> right -> left`. The registry foundation now supports local and HTTPS index/artifact transport, exact and compatible version selection, signed indexes, content-addressed caching, deterministic garbage collection, SHA-256 enforcement, offline reuse, checksum-verified archive publishing, and authenticated local persistence. Metadata validation is applied to root and nested local packages before lockfile generation or update. The async language layer represents completed async calls as deterministic `Future` values and resolves them with `await`; the runtime adds deterministic delay, cooperative cancellation, task limits, poll budgets, and explicit suspension. The LSP uses standard Content-Length JSON-RPC over stdio, reuses Zap lint diagnostics, preserves parser source spans for hover and definitions, derives completion candidates from the active document, indexes all tracked documents for workspace symbols, and provides deterministic formatting. Remaining boundaries are external registry-service deployment, external asynchronous I/O, multi-thread scheduling, and richer nested/module-aware indexing.

See the [English package guide](PACKAGE_EN.md), [Burmese package guide](PACKAGE.md), and [ecosystem roadmap](ECOSYSTEM.md).
