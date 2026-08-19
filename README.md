# Zap Programming Language

![Zap Programming Language banner](assets/branding/zap-banner.png)

> **Zap** is a simple, readable, general-purpose programming language with `.zp` source files and a standalone native runtime.

Zap is designed to make programming approachable while providing a clear path from small scripts to structured applications. The language uses indentation-based blocks, readable keywords, explicit modules, optional type annotations, structured Result/Option values, and a practical command-line workflow.

## Project Status

Zap is actively evolving toward a production-ready language ecosystem. The `v0.9.3` release line includes a native Rust runtime, source-span-aware AST execution, static checks for current type annotations, structured JSON diagnostics, a dedicated `ZapError` diagnostic boundary, Result/Option foundations, explicit module visibility, module caching, circular-import detection, and Result error propagation with `?`.

| Item | Current status |
|---|---|
| Current release line | `v0.9.3` |
| Runtime | Native Rust runtime |
| Source files | `.zp`, commonly `main.zp` |
| Project manifest | `zap.toml` |
| CLI | `zap` |
| Platforms | Linux, Windows, and macOS ARM64 release workflows |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |
| Test status | 25 native unit tests and 47 integration tests passing |

## Native Runtime Architecture

The native runtime is maintained as focused Rust modules rather than a single implementation file.

| Module | Responsibility | Status |
|---|---|---|
| `lexer.rs` | Tokenization | Implemented |
| `parser.rs` | Expression, signature, and static parsing helpers | Implemented |
| `ast.rs` | Source-span AST and native AST execution architecture | Implemented |
| `value.rs` | Runtime values, functions, classes, and object model | Implemented |
| `evaluator.rs` | Evaluation, functions, methods, modules, and control flow | Implemented |
| `stdlib.rs` | Pure math and text built-in operations | First extraction implemented |
| `diagnostics.rs` | `ZapError`, source-aware diagnostics, and secret redaction | Implemented |
| `project.rs` | Project, manifest, and module validation | Implemented |
| `cli.rs` | CLI command orchestration and exit codes | Implemented |

The modular architecture preserves existing language behavior. Runtime execution applies source-size, loop, and execution-depth limits. Token diagnostics retain one-based source locations, sensitive diagnostic values are redacted, and malformed input is handled through typed diagnostics instead of uncontrolled panics.

## Why Zap?

Zap focuses on a small and readable language core. Programs use familiar constructs such as variables, functions, collections, conditions, loops, classes, modules, and tests without requiring a large amount of ceremony. The runtime is distributed as a native executable, and `.zp` files can be run directly from the command line.

The project is intended as a foundation for future web, AI, mobile, and IoT libraries. Those domain libraries will be built on top of the language core rather than being mixed into the syntax prematurely.

## Installation

Download the archive for your operating system and CPU architecture from [GitHub Releases](https://github.com/hidecard/zap/releases), extract it, and place the `zap` executable on your `PATH`.

The `v0.9.3` release provides native archives and checksums for the supported Linux x86_64, Windows x86_64, and macOS ARM64 targets. The exact archive name depends on the selected release version and platform.

## Language Overview

| Area | Current support |
|---|---|
| Values | text, integer number, boolean, list, map, and none |
| Variables | `let` declarations and reassignment |
| Type annotations | `text`, `number`, `bool`, `list`, `map`, `none`, and `any` |
| Operators | arithmetic, comparison, `and`, `or`, and `not` |
| Control flow | `if`, `else`, `for`, `while`, `break`, and `continue` |
| Functions | parameters, return values, local scope, nested functions, and closures |
| Classes | classes, constructors, methods, properties, inheritance, and `self` |
| Collections | indexing, keys, contains, join, get, sum, reverse, sort, and emptiness checks |
| Text | upper, lower, trim, split, string conversion, and length |
| Data | JSON encoding and decoding |
| Files | text and line-based file I/O |
| System helpers | paths, time, sleep, environment variables, and math helpers |
| Modules | explicit `import`/`export`, local search paths, cache, and cycle detection |
| Error values | `ok`, `err`, `some`, `option_none`, `unwrap`, `unwrap_or`, typed `result<T>`/`option<T>`, and `?` |
| Diagnostics | human-readable errors, source locations, secret redaction, and structured JSON diagnostics |
