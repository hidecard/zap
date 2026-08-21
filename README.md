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

Zap is designed to make programming approachable while providing a clear path from small scripts to structured applications. The language uses indentation-based blocks, readable keywords, explicit modules, optional type annotations, structured Result/Option values, and a practical command-line workflow. Each native source run now receives an explicit `ExecutionContext` for module-cache, import-cycle, and execution-depth isolation. Normal source programs and local modules execute through the canonical AST boundary; the line interpreter remains only as a compatibility boundary for older line-bodied function records.

## Project Status

Zap is actively evolving toward a production-ready language ecosystem. The stable P1 language core includes a native Rust runtime, direct AST execution, static checks for current type annotations, structured JSON diagnostics, a dedicated `ZapError` diagnostic boundary, Result/Option foundations, complex control-flow narrowing, module-aware visibility, OOP field and method visibility, constructor delegation rules, module caching, circular-import detection, deterministic dependency lockfiles, and Result error propagation with `?`. P2 now provides deterministic registry resolution with exact and compatible version ranges, HTTPS transport, signed-index verification, content-addressed caching with integrity enforcement and deterministic pruning, authenticated local registry persistence, checksum-verified publishing, a deterministic single-thread async runtime with `async fn`, `Future`, `await`, timers, cancellation, task budgets, and suspension controls, plus a stdio LSP/editor integration with diagnostics, hover, completion, formatting, definitions, and workspace symbols.

| Item | Current status |
|---|---|
| Current release line | `v2.1.11` |
| Runtime | Native Rust runtime |
| Source files | `.zp`, commonly `main.zp` |
| Project manifest | `zap.toml` |
| CLI | `zap` |
| Platforms | Linux, Windows, and macOS ARM64 release workflows |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |
| Documentation hub | [English navigation](docs/DOCUMENTATION_NAVIGATION_EN.md) · [မြန်မာ navigation](docs/DOCUMENTATION_NAVIGATION_MM.md) |
| Runtime-state contract | [English](docs/RUNTIME_STATE_EN.md) · [မြန်မာ](docs/RUNTIME_STATE_MM.md) |
| AST foundation status | [English](docs/P0_FOUNDATION_STATUS_EN.md) · [မြန်မာ](docs/P0_FOUNDATION_STATUS_MM.md) |
| Documentation source | [Zap documentation directory](https://github.com/hidecard/zap/tree/master/docs) |
| Test status | Native test suite verified by GitHub Actions |
| P3 status | P3.3 production standard library and cross-platform hardening complete; v2.1 package reliability work in progress |
| Release version policy | [Single-source-of-truth policy](docs/RELEASE_VERSION_POLICY_EN.md) |

## Learning Guide

Start with the [English documentation navigation](docs/DOCUMENTATION_NAVIGATION_EN.md) to choose a learner, reference, runtime, tooling, deployment, or release path. Then use the [English learning guide](docs/LEARN_ZAP_EN.md) and [English syntax guide](docs/SYNTAX_GUIDE_EN.md) for language reference. Burmese lessons are available in the [မြန်မာ learning guide](docs/LEARN_ZAP_MM.md), [မြန်မာ syntax guide](docs/SYNTAX_GUIDE.md), and [မြန်မာ documentation navigation](docs/DOCUMENTATION_NAVIGATION_MM.md).

## Native Runtime Architecture

The native runtime is maintained as focused Rust modules rather than a single implementation file.

| Module | Responsibility | Status |
|---|---|---|
| `lexer.rs` | Tokenization | Implemented |
| `parser.rs` | Expression, signature, and static parsing helpers | Implemented |
| `ast.rs` | Source-span AST, canonical source dispatch, exports, and native module execution | Implemented |
| `value.rs` | Runtime values, functions, classes, and object model | Implemented |
| `evaluator.rs` | Evaluation, functions, methods, modules, and control flow | Implemented |
| `runtime_state.rs` | Per-run `RuntimeState`, module-cache isolation, import-cycle tracking, and execution-depth accounting | Implemented first slice |
| `stdlib.rs` | Text, math, collection, filesystem, JSON, environment, path, and time built-in operations | Stabilized initial API surface |
| `diagnostics.rs` | `ZapError`, source-aware diagnostics, and secret redaction | Implemented |
| `project.rs` | Project, manifest, lockfile, dependency graph, metadata, and module validation | Implemented |
| `cli.rs` | CLI command orchestration, async-check, LSP entry point, and exit codes | Implemented |
| `async_runtime.rs` | Stable-Rust-compatible deterministic single-thread future executor foundation | Implemented foundation |
| `lsp.rs` | Content-Length JSON-RPC server, diagnostics, hover, and context-aware completion | Implemented foundation |

The standard-library public surface is organized into deterministic `text`, `math`, `collections`, `filesystem`, `json`, and `system` domains. The native runtime includes async foundations, stdio LSP/editor integration, an explicit first-slice runtime-state boundary for per-run module and execution state, and a canonical AST-only path for parser-owned programs. Current project status and usage guidance are maintained in the [English README](README.md), [မြန်မာ README](README_MM.md), the [runtime-state contract](docs/RUNTIME_STATE_EN.md), and the [AST foundation status](docs/P0_FOUNDATION_STATUS_EN.md).
Package projects use `zap.toml` and canonical `zap.lock` files. Local path dependencies are recursively validated in deterministic order, cycles are rejected, and registry artifacts are checksum-verified with offline reuse through `ZAP_OFFLINE=1`. The modular architecture preserves existing language behavior. Runtime execution applies source-size, loop, and execution-depth limits. Token diagnostics retain one-based source locations, sensitive diagnostic values are redacted, and malformed input is handled through typed diagnostics instead of uncontrolled panics.

## Why Zap?

Zap focuses on a small and readable language core. Programs use familiar constructs such as variables, functions, collections, conditions, loops, classes, modules, and tests without requiring a large amount of ceremony. The runtime is distributed as a native executable, and `.zp` files can be run directly from the command line.

The project is intended as a foundation for future web, AI, mobile, and IoT libraries. Those domain libraries will be built on top of the language core rather than being mixed into the syntax prematurely.

## Installation

Zap is distributed as a standalone native executable. No separate language runtime is required. Download the archive that matches your operating system and CPU architecture from the [v2.1.11 GitHub Release](https://github.com/hidecard/zap/releases/tag/v2.1.11), verify the checksum when available, extract it, and make the `zap` executable available on your `PATH`.

### Supported Release Targets

| Platform | Architecture | Archive format | Installation command or action |
|---|---|---|---|
| Linux | x86_64 | `.tar.gz` | Extract and run `bash install.sh` |
| Windows | x86_64 | `.zip` | Extract and run `install_windows.bat` from Command Prompt |
| macOS | ARM64 | `.tar.gz` | Extract, make the installer executable, and run `./install.sh` |

For the current v2.1.11 release, the platform assets are `zap-2.1.11-linux-x86_64.tar.gz`, `zap-2.1.11-macos-arm64.tar.gz`, and `zap-2.1.11-windows-x86_64.zip`. The exact archive filename may change with each release. Select the asset whose platform and architecture match your computer; do not install a Linux archive on Windows or a macOS archive on Linux.

### Linux Installation

1. Open the [Zap releases page](https://github.com/hidecard/zap/releases) and download the Linux x86_64 `.tar.gz` archive and its checksum file.
2. Extract the archive in a directory you control.
3. Enter the extracted directory and run the installer:

```bash
tar -xzf zap-2.1.11-linux-x86_64.tar.gz
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

1. Download the macOS ARM64 `.tar.gz` archive from the [v2.1.11 release](https://github.com/hidecard/zap/releases/tag/v2.1.11).
2. Extract it and enter the extracted directory:

```bash
tar -xzf zap-2.1.11-macos-arm64.tar.gz
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

1. Download the Windows x86_64 `.zip` archive from the [v2.1.11 release](https://github.com/hidecard/zap/releases/tag/v2.1.11).
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

Zap has an official VS Code extension, **Zap Language Support v0.5.0**, published on the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=ArkarYan.zap-language-support). It provides syntax highlighting, snippets, diagnostics, autocomplete, signature help, hover, go-to-definition, formatting, workspace symbols, and run support through the native Zap CLI/LSP.

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
| Functions | parameters, return values, local scope, nested functions, closures, `async fn`, `Future`, and `await` |
| Classes | classes, constructors, methods, properties, inheritance, `self`, public/private/protected visibility, and `super` delegation |
| Collections | indexing, keys, contains, join, get, sum, reverse, sort, and emptiness checks |
| Text | upper, lower, trim, split, string conversion, and length |
| Data | JSON encoding and decoding |
| Files | text and line-based file I/O |
| System helpers | paths, time, sleep, environment variables, and math helpers |
| Modules | explicit `import`/`export`, local search paths, cache, cycle detection, module-aware private access, deterministic package lockfiles, nested local dependency graphs, and cycle validation |
| Error values | `ok`, `err`, `some`, `option_none`, `unwrap`, `unwrap_or`, typed `result<T>`/`option<T>`, and `?` |
| Diagnostics | human-readable errors, source locations, secret redaction, structured JSON diagnostics, and static type-narrowing errors |
