# Zap Programming Language

![Zap Programming Language banner](assets/branding/zap-banner.png)

[![Zap CI](https://github.com/hidecard/zap/actions/workflows/ci.yml/badge.svg)](https://github.com/hidecard/zap/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/hidecard/zap?display_name=tag&sort=semver&color=2ea44f)](https://github.com/hidecard/zap/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Runtime: Rust](https://img.shields.io/badge/runtime-Rust-orange.svg)](native/)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey.svg)](https://github.com/hidecard/zap/actions)
[![Source: .zp](https://img.shields.io/badge/source-.zp-8A2BE2.svg)](README.md)
[![Documentation](https://img.shields.io/badge/docs-English%20%7C%20မြန်မာ-0969da.svg)](README_MM.md) [![Discord](https://img.shields.io/badge/Discord-Join%20community-5865F2.svg?logo=discord&logoColor=white)](https://discord.gg/j9DHdCtJE) [![Telegram](https://img.shields.io/badge/Telegram-Join%20group-26A5E4.svg?logo=telegram&logoColor=white)](https://t.me/+fySFCXwMt8U3Y2Y1)

**Documentation:** [English README](README.md) · [မြန်မာ README](README_MM.md) · [Discord Community](https://discord.gg/j9DHdCtJE) · [Telegram Group](https://t.me/+fySFCXwMt8U3Y2Y1)

> **Zap** is a simple, readable, general-purpose programming language with `.zp` source files and a standalone native runtime.

Zap is designed to make programming approachable while providing a clear path from small scripts to structured applications. The language uses indentation-based blocks, readable keywords, explicit modules, optional type annotations, structured Result/Option values, and a practical command-line workflow. Each native source run now receives an explicit `ExecutionContext` for module-cache, import-cycle, execution-depth, workspace-confinement, logical budget with recursive value charging/rollback, object-store isolation, and parent-linked lexical closure frames backed by live binding cells. Object/capture cycles remain subject to the explicit `clear_object_fields()` policy, and checked object/frame accesses return typed borrow failures rather than panicking. The LSP server owns open documents in an explicit per-session `LspState`. Normal source programs and local modules execute through the canonical AST boundary; native object construction, default expressions, and direct built-in dispatch are covered there, while unsupported named built-in calls fail explicitly. The line interpreter remains only as a compatibility boundary for older line-bodied function records. Post-v2.2.2 master hardening now makes canonical equality cycle-safe and bounded, propagates checked object/frame borrow errors through logical accounting and AST member reads, hardens task/frame invariant fallbacks, and removes an LSP rename scope-stack panic path. These post-v2.2.2 hardening changes are included in v2.2.3; the active-baseline documentation synchronization is included in v2.2.4, and the HTTP URL invariant hardening is included in v2.2.5. v2.2.7 adds workspace-confined line I/O, bounded sleep and exponentiation, strict locked-build validation, malformed-port rejection, cycle-safe test discovery, process-tree cleanup, registry-test isolation, and cross-platform grammar parity. No public weak-reference API, automatic collector, traits implementation, parser syntax, or runtime syntax is added.

## Project Status

Zap is actively evolving toward a production-ready language ecosystem. The stable P1 language core includes a native Rust runtime, direct AST execution, static checks for current type annotations, structured JSON diagnostics, a dedicated `ZapError` diagnostic boundary including stable memory-limit code `ZAP-MEMORY-001`, Result/Option foundations, complex control-flow narrowing, module-aware visibility, OOP field and method visibility, constructor delegation rules, module caching, circular-import detection, deterministic dependency lockfiles, and Result error propagation with `?`. P2 now provides deterministic registry resolution with exact and compatible version ranges, HTTPS transport, signed-index verification, content-addressed caching with integrity enforcement and deterministic pruning, authenticated local registry persistence, checksum-verified publishing, a deterministic single-thread async runtime with executor-backed language scheduling, context-owned `ScheduledFuture` handles, `async fn`, `Future`, `await`, timers, cooperative `task_cancel`, poll-budget `task_join_timeout`, task budgets with explicit terminal-state and one-time join-release semantics; async function bodies use the documented eager scheduled-value contract, and suspension controls, plus a stdio LSP/editor integration with diagnostics, hover, completion, formatting, definitions, workspace symbols, parser/lexer-backed rename, didClose cleanup, nested/module-aware indexing, and async builtin metadata. M4-RFC-01 records the reviewed design direction for traits and composition without enabling the proposed syntax. Post-v2.2.0 LSP hardening now consumes standard full-sync `didChange` content from `params.contentChanges`, tracks document versions, publishes diagnostics from the accepted buffer, and safely rejects stale or unsupported range edits. Scope-aware semantic rename now resolves file-local bindings, including shadowing, closures, parameters, and import aliases; cross-file rename remains unsupported. These LSP/editor corrections landed on `master` after the immutable v2.2.0 tag and are included in the v2.2.1 corrective release; the subsequent runtime-safety, helper, grammar, and documentation corrections are included in v2.2.2. The post-v2.2.2 hardening described above is included in v2.2.3; v2.2.4 contains the active-baseline documentation synchronization, v2.2.5 contains the HTTP URL invariant hardening, and v2.2.7 contains the bounded core-reliability maintenance fixes described above.

| Item | Current status |
|---|---|
| Current release line | `v2.2.7` |
| Runtime | Native Rust runtime |
| Source files | `.zp`, commonly `main.zp` |
| Project manifest | `zap.toml` |
| CLI | `zap` |
| Platforms | Linux, Windows, and macOS ARM64 release workflows |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |
| Documentation hub | [English navigation](docs/DOCUMENTATION_NAVIGATION_EN.md) · [မြန်မာ navigation](docs/DOCUMENTATION_NAVIGATION_MM.md) |
| Runtime-state contract | [English](docs/RUNTIME_STATE_EN.md) · [မြန်မာ](docs/RUNTIME_STATE_MM.md) |
| Memory budget/object store contract | [English](docs/MEMORY_BUDGET_OBJECT_STORE_EN.md) · [မြန်မာ](docs/MEMORY_BUDGET_OBJECT_STORE_MM.md) |
| AST foundation status | [English](docs/P0_FOUNDATION_STATUS_EN.md) · [မြန်မာ](docs/P0_FOUNDATION_STATUS_MM.md) |
| Documentation source | [Zap documentation directory](https://github.com/hidecard/zap/tree/master/docs) |
| Test status | Native test suite verified by GitHub Actions |
| Verification status | M2-VERIFY-01 bounded replay, M2-VERIFY-02 native matrix, M2-BENCH-01 provenance/variance, M2-REG-01 transport, M3-STDLIB-01 policy evidence, M3-LSP-01 semantic-parity/editor validation, and post-release LSP protocol synchronization evidence |
| Language design | [Traits/composition RFC](docs/TRAITS_RFC_EN.md) — design-only; deferred for v2.2.7 |
| Release version policy | [Single-source-of-truth policy](docs/RELEASE_VERSION_POLICY_EN.md) |
| Post-v2.2.0 remediation provenance | [Corrective-release record](docs/POST_V2.2.0_REMEDIATION_EN.md) — v2.2.0, v2.2.1, and v2.2.2 remain immutable; v2.2.1 contains the LSP/editor corrections, v2.2.2 contains the subsequent runtime-safety/helper corrections, and v2.2.3 contains the post-v2.2.2 runtime, equality, borrow, and LSP hardening; v2.2.4 contains the active-baseline documentation synchronization; v2.2.5 contains the HTTP URL invariant hardening |

## Release provenance

The installation links and archive names in this README describe the published [v2.2.7 release](https://github.com/hidecard/zap/releases/tag/v2.2.7), sourced from tagged commit [`d1d6816`](https://github.com/hidecard/zap/commit/d1d6816d7d39198b4a9778d531e29cd7b4e1f38a). The published v2.2.7 release and its signed assets are now the latest official distribution. The published [v2.2.5 release](https://github.com/hidecard/zap/releases/tag/v2.2.5) remains immutable. The earlier [v2.2.0 release](https://github.com/hidecard/zap/releases/tag/v2.2.0), [v2.2.1 release](https://github.com/hidecard/zap/releases/tag/v2.2.1), [v2.2.2 release](https://github.com/hidecard/zap/releases/tag/v2.2.2), [v2.2.3 release](https://github.com/hidecard/zap/releases/tag/v2.2.3), and [v2.2.4 release](https://github.com/hidecard/zap/releases/tag/v2.2.4), together with their tags and signed assets, remain immutable. The post-v2.2.0 remediation history and the runtime-safety/helper corrections through v2.2.3 are documented in the [remediation/provenance record](docs/POST_V2.2.0_REMEDIATION_EN.md) and the v2.2.3 release notes. The post-v2.2.2 hardening is included in v2.2.3.

## v2.2.7 Dependency Remediation Status

The approved dependency graph was developed on the isolated `chore/dependency-remediation-v2.2.7` branch, reviewed in [PR #2](https://github.com/hidecard/zap/pull/2), merged into `master`, and released as v2.2.7 after the final CI and release validations passed.

| Area | Verified remediation state |
|---|---|
| URL/TLS runtime graph | `ureq 2.12.1`, `url 2.5.8`, `idna 1.1.0`, `rustls-webpki 0.103.15`, and `rustls 0.23.40` with the `ring` provider selected for the TLS fixture dependency |
| TLS test fixture | `rcgen 0.13.2`, using its current `CertifiedKey` API; this is test-only compatibility maintenance |
| Development-time time dependency | `time 0.3.47`, which requires Rust 1.88.0 |
| Security evidence | `cargo-audit 0.22.2` reports zero unresolved advisories across the 87-crate locked graph |

Because `time 0.3.47` declares Rust 1.88.0 as its minimum supported toolchain, the released source pins Rust 1.88.0 in `rust-toolchain.toml` and in the CI quality job. This is a build/toolchain compatibility change only; the Zap language surface, runtime contract, and explicitly deferred Framework/Web/App/IoT scope are unchanged. The v2.2.7 release was published only after the clean committed source, GitHub CI, strict release preflight, and published artifact verification passed.

## Learning Guide

Start with the [English documentation navigation](docs/DOCUMENTATION_NAVIGATION_EN.md) to choose a learner, reference, runtime, tooling, deployment, or release path. Then use the [English learning guide](docs/LEARN_ZAP_EN.md) and [English syntax guide](docs/SYNTAX_GUIDE_EN.md) for language reference. Burmese lessons are available in the [မြန်မာ learning guide](docs/LEARN_ZAP_MM.md), [မြန်မာ syntax guide](docs/SYNTAX_GUIDE.md), and [မြန်မာ documentation navigation](docs/DOCUMENTATION_NAVIGATION_MM.md).

### LSP editor limitation during post-release hardening

The current server advertises full document synchronization and correctly consumes standard `contentChanges` for accepted newer document versions. Range-based incremental changes are rejected until position-aware application is implemented. The server negotiates UTF-8, UTF-16, or UTF-32 position columns, rejects malformed/host/traversal file URIs, and bounds the session index to 256 documents, 32 import levels, and 32 MiB of source text. Rename now resolves file-local lexical bindings, including shadowing, closures, parameters, and import aliases; cross-file rename remains unsupported and results should be reviewed before automated refactoring. The protocol regression can be reproduced with `scripts/test_lsp_protocol_sync.sh`.

## Native Runtime Architecture

The native runtime is maintained as focused Rust modules rather than a single implementation file.

| Module | Responsibility | Status |
|---|---|---|
| `lexer.rs` | Tokenization | Implemented |
| `parser.rs` | Expression, signature, and static parsing helpers | Implemented |
| `ast.rs` | Source-span AST, canonical source dispatch, exports, and native module execution | Implemented |
| `value.rs` | Runtime values, first-class callables, parent-linked EnvFrame closures, classes, and object model | Implemented M2-FN-02 |
| `evaluator.rs` | Evaluation, functions, methods, modules, and control flow | Implemented |
| `runtime_state.rs` | Per-run `RuntimeState`, `MemoryBudget`, `ObjectStore`, workspace-root ownership, module-cache isolation, import-cycle tracking, execution-depth accounting, reset-detached lifecycle statistics, and closure-state boundaries | Implemented M2-FN-02 foundation |
| `stdlib.rs` | Text, math, collection, filesystem, JSON, environment, path, and time built-in operations | Stabilized initial API surface |
| `diagnostics.rs` | `ZapError`, source-aware diagnostics, and secret redaction | Implemented |
| `project.rs` | Project, manifest, lockfile, dependency graph, metadata, and module validation | Implemented |
| `cli.rs` | CLI command orchestration, async-check, LSP entry point, and exit codes | Implemented |
| `async_runtime.rs` | Stable-Rust-compatible deterministic single-thread future executor foundation | Implemented foundation |
| `lsp.rs` | Content-Length JSON-RPC server, per-session versioned `LspState`, full-sync document changes, diagnostics, hover, and context-aware completion | Implemented foundation; standards-compliance hardening continues |

The standard-library public surface is organized into deterministic `text`, `math`, `collections`, `filesystem`, `json`, `system`, `time`, `logging`, `runtime`, `async`, `network`, and `process` domains. M3-STDLIB-01 records stability, deprecation, semver, platform, limit, timeout/error, and determinism metadata in a machine-readable catalog and bilingual policy pair. The native runtime includes async foundations, stdio LSP/editor integration, explicit runtime-state ownership for per-run workspace/module/execution state, run-owned logical budget/object counters with stable `memory_stats()` lifecycle fields, deterministic AST value/callable/default/object charging with failed-operation rollback, explicit `cycle_policy=explicit_clear_object_fields` reporting with no public weak-reference API or automatic collector, checked object-field and canonical-AST EnvFrame borrow boundaries, explicit `ScheduledFuture` terminal states and one-time admitted-task release, eager async scheduled-value semantics, first-class callable values with parent-linked live-cell `EnvFrame` closures, per-session LSP document state, and a canonical AST-only path for parser-owned programs, including native `new(...)` construction and AST-evaluated default expressions.
Current project status and usage guidance are maintained in the [English README](README.md), [မြန်မာ README](README_MM.md), the [runtime-state contract](docs/RUNTIME_STATE_EN.md), and the [AST foundation status](docs/P0_FOUNDATION_STATUS_EN.md).
Package projects use `zap.toml` and canonical `zap.lock` files. Local path dependencies are recursively validated in deterministic order, cycles are rejected, and registry artifacts are checksum-verified with offline reuse through `ZAP_OFFLINE=1`. The modular architecture preserves existing language behavior. Runtime execution applies source-size, loop, and execution-depth limits. Token diagnostics retain one-based source locations, sensitive diagnostic values are redacted, and malformed input is handled through typed diagnostics instead of uncontrolled panics.

## Why Zap?

Zap focuses on a small and readable language core. Programs use familiar constructs such as variables, functions, collections, conditions, loops, classes, modules, and tests without requiring a large amount of ceremony. The runtime is distributed as a native executable, and `.zp` files can be run directly from the command line.

The project is intended as a foundation for future web, AI, mobile, and IoT libraries. Those domain libraries will be built on top of the language core rather than being mixed into the syntax prematurely.

## Installation

Zap is distributed as a standalone native executable. No separate language runtime is required. Download the archive that matches your operating system and CPU architecture from the [published v2.2.7 release](https://github.com/hidecard/zap/releases/tag/v2.2.7) or the [GitHub Releases page](https://github.com/hidecard/zap/releases), verify its checksum and signature, extract it, and make the `zap` executable available on your `PATH`. The v2.2.7 release is the latest published release.

### Supported Release Targets

| Platform | Architecture | Archive format | Installation command or action |
|---|---|---|---|
| Linux | x86_64 | `.tar.gz` | Extract and run `bash install.sh` |
| Windows | x86_64 | `.zip` | Extract and run `install_windows.bat` from Command Prompt |
| macOS | ARM64 | `.tar.gz` | Extract, make the installer executable, and run `./install.sh` |

For the planned v2.2.7 release, the platform assets are expected to be `zap-2.2.7-linux-x86_64.tar.gz`, `zap-2.2.7-macos-arm64.tar.gz`, and `zap-2.2.7-windows-x86_64.zip`; verify the published asset names on the GitHub Releases page before installing. The exact archive filename may change with each release. Select the asset whose platform and architecture match your computer; do not install a Linux archive on Windows or a macOS archive on Linux.

### Linux Installation

1. Open the [Zap releases page](https://github.com/hidecard/zap/releases) and download the Linux x86_64 `.tar.gz` archive and its checksum file.
2. Extract the archive in a directory you control.
3. Enter the extracted directory and run the installer:

```bash
tar -xzf zap-2.2.7-linux-x86_64.tar.gz
cd zap
bash install.sh
```

4. Open a new terminal, or reload your shell configuration, and verify the installation:

```bash
zap --version
zap --help
```

If you prefer a local installation, keep the extracted `zap` executable in a project directory and run it with `./zap main.zp` without changing the system `PATH`.

### macOS Installation

1. Download the macOS ARM64 `.tar.gz` archive from the [published v2.2.7 release](https://github.com/hidecard/zap/releases/tag/v2.2.7).
2. Extract it and enter the extracted directory:

```bash
tar -xzf zap-2.2.7-macos-arm64.tar.gz
cd zap
```

3. Run the installer. If macOS reports that the installer is not executable, grant execute permission first:

```bash
chmod +x install.sh
./install.sh
```

4. Verify the installation:

```bash
zap --version
zap --help
```

On Intel-based Macs, use a compatible release asset if one is published. Do not use the ARM64 archive unless your Mac supports ARM64 execution.

### Windows Installation

1. Download the Windows x86_64 `.zip` archive from the [published v2.2.7 release](https://github.com/hidecard/zap/releases/tag/v2.2.7).
2. Extract the archive to a folder such as `C:\Zap`.
3. Open **Command Prompt** as a normal user and run the installer batch file from the extracted directory:

```bat
cd C:\Zap
install_windows.bat
```

4. Close and reopen Command Prompt so the updated `PATH` is loaded, then verify:

```bat
zap.exe --version
zap.exe --help
```

If you do not want a global installation, run Zap directly from the extracted folder:

```bat
C:\Zap\bin\zap.exe main.zp
```

### Running a First `.zp` File

After installation, create or open a Zap source file with the `.zp` extension and run it from any directory where the `zap` command is available:

```bash
zap main.zp
```

On Windows Command Prompt, the equivalent command is:

```bat
zap.exe main.zp
```

## VS Code Extension

Zap has an official VS Code extension, **Zap Language Support v0.5.0**, published on the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=ArkarYan.zap-language-support). It provides syntax highlighting, snippets, diagnostics, autocomplete, signature help, hover, go-to-definition, formatting, workspace symbols, rename support, and run support through the native Zap CLI/LSP. The repository also checks in a catalog-aligned TextMate grammar and language configuration under `editors/vscode/`; validate them with `scripts/validate_vscode_assets.py`.

Install it from the command line:

```bash
code --install-extension ArkarYan.zap-language-support
```

Alternatively, open the [Zap Language Support Marketplace page](https://marketplace.visualstudio.com/items?itemName=ArkarYan.zap-language-support) in VS Code and choose **Install**. After installation, ensure the `zap` executable is available on `PATH`, or set `zap.executable` in VS Code settings. The extension supports `.zp` files and uses `zap lsp` for editor integration.

## Language Overview

| Area | Current support |
|---|---|
| Values | text, integer number, boolean, list, map, and none |
| Variables | `let` declarations and reassignment |
| Type annotations | `text`, `number`, `bool`, `list`, `map`, `none`, and `any` |
| Operators | arithmetic, comparison, `and`, `or`, and `not` |
| Control flow | `if`, `else`, `for`, `while`, `break`, and `continue` |
| Functions and async tasks | parameters, return values, local scope, nested functions, closures, `async fn`, context-owned `ScheduledFuture`, `await`, `spawn`, `task_join`, `task_is_ready`, `task_cancel`, and `task_join_timeout` |
| Classes | classes, constructors, methods, properties, inheritance, `self`, public/private/protected visibility, and `super` delegation |
| Collections | indexing, keys, contains, join, get, sum, reverse, sort, and emptiness checks |
| Text | upper, lower, trim, split, string conversion, and length |
| Data | JSON encoding and decoding |
| Files | text and line-based file I/O |
| System helpers | paths, time, sleep, environment variables, and math helpers |
| Modules | explicit `import`/`export`, local search paths, cache, cycle detection, module-aware private access, deterministic package lockfiles, nested local dependency graphs, and cycle validation |
| Error values | `ok`, `err`, `some`, `option_none`, `unwrap`, `unwrap_or`, typed `result<T>`/`option<T>`, and `?` |
| Diagnostics and verification | human-readable errors, source locations, secret redaction, structured JSON diagnostics, static type-narrowing errors, fixed-seed replay, bounded repeated outcome digests, and durable CI evidence |
