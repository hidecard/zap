# Zap Compatibility Matrix

This document tracks version compatibility across Zap components and platforms.

## Current Release

| Component | Version | Notes |
|-----------|---------|-------|
| Compiler | 2.11.18 | Native Rust implementation |
| Language | 2.11.18 | AST schema v1 |
| Standard Library | 2.11.18 | 68 entries across 9 domains |
| Package Format | 2.11.18 | `zap.toml` manifest |
| LSP | 2.11.18 | `zap lsp` |
| Bootstrap Stage | B4 | Rust-free full-language certified |

## Platform Support

| Platform | Status | Binary | CI |
|----------|--------|--------|-----|
| Linux x86_64 | ✅ Supported | `zap` | ✅ |
| macOS x86_64 | ✅ Supported | `zap` | ✅ |
| macOS ARM64 | ✅ Supported | `zap` | ✅ |
| Windows x86_64 | ✅ Supported | `zap.exe` | ✅ |
| Linux ARM64 | ⏳ Planned | - | ❌ |
| Windows ARM64 | ⏳ Planned | - | ❌ |

## Bootstrap Contracts

| Contract | Version | Status |
|----------|---------|--------|
| AST Schema | 1 | Stable |
| Token Schema | 1 | Stable |
| Diagnostic Schema | 1 | Stable |
| Typed-IR Schema | 1 | Reference-only |
| Artifact Schema | 1 | Stable |

## Standard Library Determinism

| Domain | Determinism | Examples |
|--------|-------------|----------|
| async | runtime-dependent | `task_spawn`, `task_join` |
| collections | pure | `append`, `count`, `enumerate` |
| filesystem | external-io | `read_text`, `write_text` |
| json | pure | `from_json`, `json` |
| logging | pure | `log_json`, `log_record` |
| math | pure | `abs`, `max`, `min`, `pow`, `sqrt` |
| network | external-io | `http_get`, `url_parse` |
| system | pure/external-io | `dirname`, `env`, `config_dir` |
| text | pure | `contains`, `join`, `len`, `split`, `trim` |
| time | input-deterministic/runtime-dependent | `utc_now`, `duration_between` |

## Language Surface

| Feature | Status | Bootstrap Stage |
|---------|--------|-----------------|
| Lexer/parser | ✅ Complete | B1 |
| Type checker | ✅ Complete | B2 |
| Generic types | ✅ Complete | B2 |
| Flow analysis | ✅ Complete | B2 |
| Canonical AST bridge | ✅ Complete | B3 |
| Typed-IR producer | ✅ Complete | B3 |
| Bytecode lowering | ✅ Complete | B3 |
| VM execution | ✅ Complete | B3 |
| Package build | ✅ Complete | B3 |
| Self-hosting | ✅ Certified | B4 |

## Breaking Changes Policy

- Breaking changes require a major version bump
- Deprecated features receive at least 2 minor versions warning
- Security fixes may be backported to supported versions
- AST schema changes require migration guide

## Support Policy

| Version | Supported | Security Fixes | Bug Fixes |
|---------|-----------|----------------|-----------|
| 2.x | ✅ Yes | ✅ Yes | ✅ Yes |
| 1.x | ❌ No | ❌ No | ❌ No |
