# Zap P2 Progress — Ecosystem Foundation

## Current status

Zap P1 Language Core was released as `v1.0.0`. P2 has now started with a local package-manager foundation that remains deterministic and does not require a remote registry.

| Milestone | Status | Notes |
|---|---|---|
| Manifest dependency declarations | Implemented | `[dependencies]` entries are parsed and validated, including local path specifications. |
| Canonical lockfile | Implemented | `zap.lock` is generated in stable package/dependency order and renders local paths canonically. |
| `zap add` command | Implemented | Adds a string-valued dependency, sorts the dependency section, rejects duplicates, and invalidates the old lockfile. |
| Registry-ready package metadata | Implemented | Optional `description`, `authors`, `license`, `repository`, and 64-character hexadecimal SHA-256 `checksum` fields are validated deterministically. |
| Registry index and file-backed cache foundation | Implemented | JSON index validation, exact version selection, deterministic file-backed cache, SHA-256 enforcement, and offline reuse are supported through explicit environment configuration. HTTP transport and publishing remain planned. |
| `zap install` | Implemented | Validates the current manifest and canonical lockfile; when configured, it resolves registry entries and verifies or populates the checksum-checked cache. |
| `zap update` | Implemented | Regenerates the canonical lockfile deterministically, validates the complete local graph, and performs configured registry/cache integrity checks. |
| Async runtime foundation | Implemented | Deterministic single-thread executor, `spawn`, `run_until_idle`, and `block_on` are available internally; async language syntax and timers remain planned. |
| LSP/editor foundation | Implemented foundation | `zap lsp` provides stdio JSON-RPC framing, initialize/shutdown, text synchronization, lint diagnostics with source ranges, and deterministic starter keyword completion; parser-aware completion, hover, formatting, and workspace indexing remain planned. |

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

The native test suite covers successful additions, lexicographic ordering, duplicate rejection, lockfile invalidation, install validation, update regeneration, idempotence, stale-lock rejection, CLI help exposure, valid local packages, missing local package manifests, nested local packages, and deterministic cycle diagnostics. A dependency may use `name = { path = "../local-lib" }`; the path is resolved relative to the consuming project, must contain a valid `zap.toml` with package name and version metadata, and is represented canonically in `zap.lock`. Local path dependencies are traversed depth-first in sorted dependency order. A repeated canonical path on the active traversal stack produces an error such as `dependency cycle detected: left -> right -> left`. The manifest metadata contract and local registry/cache foundation are now registry-ready; HTTP transport, remote publishing, range solving, signed indexes, and cache garbage collection remain later P2 work. Metadata validation is applied to root and nested local packages before lockfile generation or update. The async foundation is intentionally single-threaded and deterministic so it does not change existing synchronous execution. The LSP foundation uses standard Content-Length JSON-RPC over stdio, reuses Zap lint diagnostics, maps reported lines to source ranges, and exposes deterministic keyword completion; it does not yet claim a complete language server.

See the [English package guide](PACKAGE_EN.md), [Burmese package guide](PACKAGE.md), and [ecosystem roadmap](ECOSYSTEM.md).
