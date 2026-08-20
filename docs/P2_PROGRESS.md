# Zap P2 Progress — Ecosystem Foundation

## Current status

Zap P1 Language Core was released as `v1.0.0`. P2 has now started with a local package-manager foundation that remains deterministic and does not require a remote registry.

| Milestone | Status | Notes |
|---|---|---|
| Manifest dependency declarations | Implemented | `[dependencies]` entries are parsed and validated, including local path specifications. |
| Canonical lockfile | Implemented | `zap.lock` is generated in stable package/dependency order and renders local paths canonically. |
| `zap add` command | Implemented | Adds a string-valued dependency, sorts the dependency section, rejects duplicates, and invalidates the old lockfile. |
| Registry-ready package metadata | Implemented | Optional `description`, `authors`, `license`, `repository`, and 64-character hexadecimal SHA-256 `checksum` fields are validated deterministically. |
| Remote registry resolution | Planned | Requires network policy, caching, index resolution, downloads, and integrity enforcement. |
| `zap install` | Implemented | Validates the current manifest and canonical lockfile without changing project files or contacting a registry. |
| `zap update` | Implemented | Regenerates the canonical lockfile deterministically from the current manifest and validates the complete local dependency graph. |
| Async runtime foundation | Implemented | Deterministic single-thread executor, `spawn`, `run_until_idle`, and `block_on` are available internally; async language syntax and timers remain planned. |
| LSP/editor foundation | Implemented | `zap lsp` provides stdio JSON-RPC framing, initialize/shutdown, text synchronization, and lint diagnostics; completion, hover, formatting, and workspace indexing remain planned. |

## Local install/update contract

```bash
zap install [project-dir]
zap update [project-dir]
```

`zap install` is validation-only. For dependency-bearing projects it requires a present, current, canonical `zap.lock`; it does not modify the manifest or lockfile and performs no implicit network or registry access. `zap update` intentionally regenerates `zap.lock` from the current manifest using the same canonical ordering as `zap lock`. It is deterministic and local, and does not yet download packages or solve registry dependencies. For local path dependencies, it recursively validates nested manifests in lexicographic order and rejects cycles before writing or accepting the lockfile.

## `zap add` contract

```bash
zap add <name> <version> [project-dir]
```

The command updates `zap.toml` deterministically. It rejects empty or whitespace-containing names, duplicate dependency names, and invalid single-line requirements. If `zap.lock` exists, it is removed because the manifest no longer matches the lockfile. Running `zap lock` regenerates the canonical lockfile.

## Verification

The native test suite covers successful additions, lexicographic ordering, duplicate rejection, lockfile invalidation, install validation, update regeneration, idempotence, stale-lock rejection, CLI help exposure, valid local packages, missing local package manifests, nested local packages, and deterministic cycle diagnostics. A dependency may use `name = { path = "../local-lib" }`; the path is resolved relative to the consuming project, must contain a valid `zap.toml` with package name and version metadata, and is represented canonically in `zap.lock`. Local path dependencies are traversed depth-first in sorted dependency order. A repeated canonical path on the active traversal stack produces an error such as `dependency cycle detected: left -> right -> left`. The manifest metadata contract is now registry-ready, while remote fetching remains a later P2 milestone. Metadata validation is applied to root and nested local packages before lockfile generation or update. The async foundation is intentionally single-threaded and deterministic so it does not change existing synchronous execution. The LSP foundation uses standard Content-Length JSON-RPC over stdio and reuses Zap lint diagnostics; it does not yet claim a complete language server.

See the [English package guide](PACKAGE_EN.md), [Burmese package guide](PACKAGE.md), and [ecosystem roadmap](ECOSYSTEM.md).
