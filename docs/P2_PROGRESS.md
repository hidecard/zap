# Zap P2 Progress — Ecosystem Foundation

## Current status

Zap P1 Language Core was released as `v1.0.0`. P2 has now started with a local package-manager foundation that remains deterministic and does not require a remote registry.

| Milestone | Status | Notes |
|---|---|---|
| Manifest dependency declarations | Implemented | `[dependencies]` entries are parsed and validated. |
| Canonical lockfile | Implemented | `zap.lock` is generated in stable package/dependency order. |
| `zap add` command | Implemented | Adds a string-valued dependency, sorts the dependency section, rejects duplicates, and invalidates the old lockfile. |
| Remote registry resolution | Planned | Requires package metadata, network policy, caching, and integrity checks. |
| `zap install` / `zap update` | Planned | To be implemented after registry and dependency graph contracts are finalized. |
| Async runtime | Planned | Separate P2 track. |
| LSP/editor integration | Planned | Separate tooling track. |

## `zap add` contract

```bash
zap add <name> <version> [project-dir]
```

The command updates `zap.toml` deterministically. It rejects empty or whitespace-containing names, duplicate dependency names, and invalid single-line requirements. If `zap.lock` exists, it is removed because the manifest no longer matches the lockfile. Running `zap lock` regenerates the canonical lockfile.

## Verification

The native test suite covers successful additions, lexicographic ordering, duplicate rejection, lockfile invalidation, and CLI help exposure. The next package-manager milestone is a dependency source model for local path packages and registry-ready metadata without compromising reproducibility.

See the [English package guide](PACKAGE_EN.md), [Burmese package guide](PACKAGE.md), and [ecosystem roadmap](ECOSYSTEM.md).
