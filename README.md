# Zap Programming Language

![Zap Programming Language banner](assets/branding/zap-banner.png)

[![Zap CI](https://github.com/hidecard/zap/actions/workflows/ci.yml/badge.svg)](https://github.com/hidecard/zap/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/hidecard/zap?display_name=tag&sort=semver&color=2ea44f)](https://github.com/hidecard/zap/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Runtime: Rust](https://img.shields.io/badge/runtime-Rust-orange.svg)](native/)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey.svg)](https://github.com/hidecard/zap/actions)
[![Source: .zp](https://img.shields.io/badge/source-.zp-8A2BE2.svg)](docs/SYNTAX_GUIDE_EN.md)
[![Documentation](https://img.shields.io/badge/docs-GitHub%20Docs-0969da.svg)](https://github.com/hidecard/zap/tree/master/docs)

**Documentation:** [Zap Documentation Web](https://github.com/hidecard/zap/tree/master/docs) · [English Guide](https://github.com/hidecard/zap/blob/master/docs/LEARN_ZAP_EN.md) · [မြန်မာ Guide](https://github.com/hidecard/zap/blob/master/docs/LEARN_ZAP_MM.md) · [Syntax Guide](docs/SYNTAX_GUIDE_EN.md) · [Type Narrowing EN](docs/TYPE_NARROWING_EN.md) · [Type Narrowing MM](docs/TYPE_NARROWING_MM.md) · [Default Parameters EN](docs/DEFAULT_PARAMETERS_EN.md) · [Default Parameters MM](docs/DEFAULT_PARAMETERS_MM.md) · [Package EN](docs/PACKAGE_EN.md) · [Package MM](docs/PACKAGE.md) · [Changelog EN](CHANGELOG_EN.md) · [Changelog MM](CHANGELOG_MM.md) · [Stdlib Index EN](docs/STDLIB_INDEX_EN.md) · [Stdlib Index MM](docs/STDLIB_INDEX_MM.md) · [Async/LSP EN](docs/ASYNC_LSP_EN.md) · [Async/LSP MM](docs/ASYNC_LSP_MM.md)

> **Zap** is a simple, readable, general-purpose programming language with `.zp` source files and a standalone native runtime.

Zap is designed to make programming approachable while providing a clear path from small scripts to structured applications. The language uses indentation-based blocks, readable keywords, explicit modules, optional type annotations, structured Result/Option values, and a practical command-line workflow.

## Project Status

Zap is actively evolving toward a production-ready language ecosystem. The stable P1 language core includes a native Rust runtime, direct AST execution, static checks for current type annotations, structured JSON diagnostics, a dedicated `ZapError` diagnostic boundary, Result/Option foundations, complex control-flow narrowing, module-aware visibility, OOP field and method visibility, constructor delegation rules, module caching, circular-import detection, deterministic dependency lockfiles, and Result error propagation with `?`. P2 now provides deterministic local package graph validation, nested dependency traversal, cycle diagnostics, registry-ready package metadata validation, deterministic JSON registry indexes, local and HTTPS package transport, content-addressed cache with SHA-256 enforcement and offline reuse, checksum-verified archive publishing, a deterministic single-thread async runtime foundation with `async fn`, `Future`, and `await`, and a stdio LSP/editor integration with parser-backed hover and context-aware completion.

| Item | Current status |
|---|---|
| Current release line | `v1.0.0` |
| Runtime | Native Rust runtime |
| Source files | `.zp`, commonly `main.zp` |
| Project manifest | `zap.toml` |
| CLI | `zap` |
| Platforms | Linux, Windows, and macOS ARM64 release workflows |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |
| Documentation | [Zap Documentation Web](https://github.com/hidecard/zap/tree/master/docs) |
| Test status | 223 native tests passing |

## Native Runtime Architecture

The native runtime is maintained as focused Rust modules rather than a single implementation file.

| Module | Responsibility | Status |
|---|---|---|
| `lexer.rs` | Tokenization | Implemented |
| `parser.rs` | Expression, signature, and static parsing helpers | Implemented |
| `ast.rs` | Source-span AST and native AST execution architecture | Implemented |
| `value.rs` | Runtime values, functions, classes, and object model | Implemented |
| `evaluator.rs` | Evaluation, functions, methods, modules, and control flow | Implemented |
| `stdlib.rs` | Text, math, collection, filesystem, JSON, environment, path, and time built-in operations | Stabilized initial API surface |
| `diagnostics.rs` | `ZapError`, source-aware diagnostics, and secret redaction | Implemented |
| `project.rs` | Project, manifest, lockfile, dependency graph, metadata, and module validation | Implemented |
| `cli.rs` | CLI command orchestration, async-check, LSP entry point, and exit codes | Implemented |
| `async_runtime.rs` | Stable-Rust-compatible deterministic single-thread future executor foundation | Implemented foundation |
| `lsp.rs` | Content-Length JSON-RPC server, diagnostics, hover, and context-aware completion | Implemented foundation |

The standard-library public surface is organized into deterministic `text`, `math`, `collections`, `filesystem`, `json`, and `system` domains; `zap async-check` verifies the internal async runtime foundation and `zap lsp` starts the editor protocol server over stdio. The current LSP surface supports initialize, shutdown, text synchronization, lint diagnostics with deterministic source-line ranges, parser-backed hover, and context-aware completion. Formatting, go-to-definition, and workspace indexing remain future work.
 See the [async/LSP English guide](docs/ASYNC_LSP_EN.md) or [Burmese guide](docs/ASYNC_LSP_MM.md), and the [English stdlib index](docs/STDLIB_INDEX_EN.md) or [Burmese stdlib index](docs/STDLIB_INDEX_MM.md). Package projects use `zap.toml` and canonical `zap.lock` files. Local path dependencies are recursively validated in deterministic order, cycles are rejected, and optional registry metadata includes description, authors, license, repository, and a 64-character hexadecimal SHA-256 checksum. The registry foundation validates local and HTTPS JSON indexes, selects exact versions, fetches and caches package artifacts, enforces SHA-256 integrity, supports offline reuse through `ZAP_OFFLINE=1`, and publishes checksum-verified archives over HTTPS; see the [English package guide](docs/PACKAGE_EN.md), [Burmese package guide](docs/PACKAGE.md), and [P2 progress](docs/P2_PROGRESS.md). The modular architecture preserves existing language behavior. Runtime execution applies source-size, loop, and execution-depth limits. Token diagnostics retain one-based source locations, sensitive diagnostic values are redacted, and malformed input is handled through typed diagnostics instead of uncontrolled panics.

## Why Zap?

Zap focuses on a small and readable language core. Programs use familiar constructs such as variables, functions, collections, conditions, loops, classes, modules, and tests without requiring a large amount of ceremony. The runtime is distributed as a native executable, and `.zp` files can be run directly from the command line.

The project is intended as a foundation for future web, AI, mobile, and IoT libraries. Those domain libraries will be built on top of the language core rather than being mixed into the syntax prematurely.

## Installation

Zap is distributed as a standalone native executable. No separate language runtime is required. Download the archive that matches your operating system and CPU architecture from the [v1.0.0 GitHub Release](https://github.com/hidecard/zap/releases/tag/v1.0.0), verify the checksum when available, extract it, and make the `zap` executable available on your `PATH`.

### Supported Release Targets

| Platform | Architecture | Archive format | Installation command or action |
|---|---|---|---|
| Linux | x86_64 | `.tar.gz` | Extract and run `bash install.sh` |
| Windows | x86_64 | `.zip` | Extract and run `install_windows.bat` from Command Prompt |
| macOS | ARM64 | `.tar.gz` | Extract, make the installer executable, and run `./install.sh` |

The exact archive filename may change with each release. Select the asset whose platform and architecture match your computer; do not install a Linux archive on Windows or a macOS archive on Linux.

### Linux Installation

1. Open the [Zap releases page](https://github.com/hidecard/zap/releases) and download the Linux x86_64 `.tar.gz` archive and its checksum file.
2. Extract the archive in a directory you control.
3. Enter the extracted directory and run the installer:

```bash
tar -xzf zap-1.0.0-linux-x86_64.tar.gz
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

1. Download the macOS ARM64 `.tar.gz` archive from the [v1.0.0 release](https://github.com/hidecard/zap/releases/tag/v1.0.0).
2. Extract it and enter the extracted directory:

```bash
tar -xzf zap-1.0.0-macos-arm64.tar.gz
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

1. Download the Windows x86_64 `.zip` archive from the [v1.0.0 release](https://github.com/hidecard/zap/releases/tag/v1.0.0).
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
