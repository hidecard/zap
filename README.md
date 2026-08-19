# Zap Programming Language

![Zap Programming Language banner](assets/branding/zap-banner.png)

[![Zap CI](https://github.com/hidecard/zap/actions/workflows/ci.yml/badge.svg)](https://github.com/hidecard/zap/actions/workflows/ci.yml)

> **Zap** is a simple, readable, general-purpose programming language with `.zp` source files and a standalone native runtime.

Zap is designed to make programming approachable while providing a clear path from small scripts to structured applications. The language uses indentation-based blocks, readable keywords, explicit modules, optional type annotations, structured Result/Option values, and a practical command-line workflow.

## Choose Your Documentation Language

| Language | Beginner course | Syntax and usage reference |
|---|---|---|
| English | [`docs/LEARN_ZAP_EN.md`](docs/LEARN_ZAP_EN.md) | [`docs/SYNTAX_GUIDE_EN.md`](docs/SYNTAX_GUIDE_EN.md) |
| မြန်မာ | [`docs/LEARN_ZAP_MM.md`](docs/LEARN_ZAP_MM.md) | [`docs/SYNTAX_GUIDE.md`](docs/SYNTAX_GUIDE.md) |

Start with the **English beginner course** or the **မြန်မာ beginner course**, then use the reference guides when you need a complete syntax or built-in-function lookup.

## Continuous Integration and Release Automation

Every push to `master`/`main` and every pull request runs the CI workflow. It checks Rust formatting, Clippy warnings, compilation, the native integration suite, repository whitespace, and release builds for Linux x86_64, Windows x86_64, and macOS ARM64.

Tagged releases matching `v*` use the release workflow to build native archives, generate SHA-256 checksum files, upload artifacts, and publish them to the corresponding GitHub Release. The workflow can also be started manually from the Actions tab.

## Project Status

Zap is actively evolving toward a production-ready language ecosystem. The `v0.9.1` release line includes a native runtime, static checks for current type annotations, structured JSON diagnostics, a dedicated `ZapError` diagnostic boundary, Result/Option foundations, explicit module visibility, module caching, circular-import detection, and Result error propagation with `?`.

| Item | Current status |
|---|---|
| Current release line | `v0.9.1` |
| Runtime | Native Rust runtime |
| Source files | `.zp`, commonly `main.zp` |
| Project manifest | `zap.toml` |
| CLI | `zap` |
| Platforms | Linux, Windows, and macOS ARM64 release workflows |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |

### Native Runtime Architecture

The native runtime is being maintained as focused Rust modules rather than a single implementation file.

| Module | Responsibility | Status |
|---|---|---|
| `lexer.rs` | Tokenization | Implemented |
| `parser.rs` | Expression, signature, and static parsing helpers | Implemented |
| `value.rs` | Runtime values and object model | Implemented |
| `evaluator.rs` | Evaluation, functions, methods, modules, and control flow | Implemented |
| `stdlib.rs` | Pure math and text built-in operations | First extraction implemented |
| `diagnostics.rs` | `ZapError` and structured diagnostics | Implemented |
| `project.rs` | Project, manifest, and module validation | Implemented |
| `cli.rs` | CLI command orchestration and exit codes | Implemented |
| `ast.rs` | AST-based execution architecture | Planned |

The modularization refactor preserves the existing language behavior. CLI command failures use exit code `1`, invalid command usage uses exit code `2`, and successful commands return normally with exit code `0`. The native suite currently covers **3 unit tests and 35 integration tests**.

## Why Zap?

Zap focuses on a small and readable language core. Programs use familiar constructs such as variables, functions, collections, conditions, loops, classes, modules, and tests without requiring a large amount of ceremony. The runtime is distributed as a native executable, and `.zp` files can be run directly from the command line.

The project is intended as a foundation for future web, AI, mobile, and IoT libraries. Those domain libraries will be built on top of the language core rather than being mixed into the syntax prematurely.

## Installation

Download the archive for your operating system and CPU architecture from [Releases](https://github.com/hidecard/zap/releases), extract it, and place the `zap` executable on your `PATH`.

On Linux or macOS:

```bash
tar -xzf zap-0.9.1-linux-x86_64.tar.gz
cd zap-0.9.1
bash install.sh
zap --version
```

On Windows, extract the release archive and run the installer batch file from Command Prompt:

```bat
install_windows.bat
zap --version
```

The executable can also be run without installing it globally:

```bat
bin\zap.exe main.zp
```

The exact archive name may differ by release version and platform. Always select the archive that matches your system.

## Quick Start

Create `hello.zp`:

```zap
say "Hello from Zap"
```

Run it:

```bash
zap hello.zp
```

Create a project scaffold:

```bash
zap init hello-project
cd hello-project
zap check .
zap build .
zap test .
zap main.zp
```

`zap init` creates a project manifest, an entry file, and a starter test structure.

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
| Diagnostics | human-readable errors and `zap check --json` structured diagnostics |

## Syntax Examples

### Variables and Types

```zap
let name: text = "Zap"
let version: number = 9
let ready: bool = true
let features: list = ["web", "ai", "iot"]

say name
say version
say type(features)
```

Annotations are optional. When an annotation is present, the checker and runtime validate the assigned value.

### Conditions and Loops

```zap
let score = 85

if score >= 80:
    say "Excellent"
else:
    say "Keep practising"

for number in range(3):
    say number
```

Zap blocks begin after a colon and are defined by indentation.

### Functions and Return Types

```zap
fn add(a: number, b: number) -> number:
    return a + b

say add(4, 6)
```

The checker validates known function signatures, argument counts, and compatible literal or inferred values.

### Classes

```zap
class User:
    fn init(self, name):
        self.name = name

    fn greet(self):
        return "Hello, " + self.name

let user = new("User", "Zap")
say user.greet()
```

### Modules

`modules/greeting.zp`:

```zap
export fn greet(name):
    return "Hello, " + name

fn private_helper():
    return "internal"
```

`main.zp`:

```zap
import "greeting.zp"
say greet("Zap")
```

Only explicitly exported symbols are visible to importers. Modules are cached during one runtime execution, circular imports are rejected, and absolute module paths are not accepted.

### Result and Option

```zap
let success = ok(42)
let failure = err("not found")
let present = some("Zap")
let missing = option_none()

say unwrap_or(success, 0)
say unwrap_or(failure, 0)
say unwrap_or(missing, "unknown")
```

### Result Propagation

```zap
fn load_user() -> Result:
    return err("user not found")

fn profile() -> Result:
    let user = load_user()?
    return ok(user)
```

The `?` operator unwraps a successful Result and returns an error Result immediately when the expression contains an error. Result and Option payloads can also be checked statically with annotations such as `result<number>` and `option<text>`.

```zap
let answer: result<number> = ok(42)
let username: option<text> = some("Zap")
```

The checker reports a `TypeError` when the constructor payload does not match the annotated payload type.

## Built-in Functions

| Group | Functions |
|---|---|
| Output and values | `say`, `type`, `str`, `len`, `range` |
| Collections | `keys`, `contains`, `join`, `get`, `is_empty`, `sum`, `reverse`, `sort` |
| Text | `upper`, `lower`, `trim`, `split` |
| JSON | `json`, `from_json` |
| Files | `read_text`, `write_text`, `read_lines`, `write_lines` |
| Paths | `path_join`, `basename`, `dirname`, `exists` |
| Time and environment | `now`, `sleep`, `env`, `has_env` |
| Math | `abs`, `min`, `max`, `pow`, `sqrt` |
| Testing | `assert` |
| Result and Option | `ok`, `err`, `some`, `option_none`, `is_ok`, `is_err`, `is_some`, `is_option_none`, `unwrap`, `unwrap_or` |

## CLI Reference

```text
zap <file.zp>          Run a Zap source file
zap init <directory>    Create a project scaffold
zap check [directory]   Check a project or source tree
zap check --json [dir]  Emit structured JSON diagnostics
zap build [directory]   Validate build readiness
zap test [directory]    Run *_test.zp files
zap fmt <file.zp>       Format a source file
zap lint <file.zp>      Report style and whitespace issues
zap run <file.zp>       Run a source file explicitly
zap --version           Print the runtime version
zap --help              Print command help
```

A structured check diagnostic can contain `kind`, `message`, `file`, `line`, and `column` fields. This makes the checker suitable for editor and automation integration.

## Project Layout

```text
my-zap-project/
├── zap.toml
├── main.zp
├── modules/
│   └── greeting.zp
├── lib/
│   └── text_helpers.zp
└── tests/
    └── smoke_test.zp
```

Example `zap.toml`:

```toml
[package]
name = "my-zap-project"
version = "0.1.0"
main = "main.zp"
```

## Testing and Development

Zap test files conventionally end with `_test.zp`:

```zap
fn add(a, b):
    return a + b

assert(add(2, 3) == 5, "addition failed")
assert(type(add(2, 3)) == "number", "result type failed")
say "test passed"
```

Run project tests with:

```bash
zap test
zap test tests
```

Run the native Rust integration suite from the repository root:

```bash
cargo test --manifest-path native/Cargo.toml
```

Format and check code before submitting a change:

```bash
zap fmt main.zp
zap lint main.zp
zap check .
zap check --json .
git diff --check
```

## Learning Path

The recommended order is to begin with installation and Hello World, then study values, variables, operators, conditions, collections, loops, functions, closures, file I/O, modules, classes, Result/Option values, error propagation, testing, and a small project.

| Stage | English | မြန်မာ |
|---|---|---|
| Complete beginner course | [`LEARN_ZAP_EN.md`](docs/LEARN_ZAP_EN.md) | [`LEARN_ZAP_MM.md`](docs/LEARN_ZAP_MM.md) |
| Syntax reference | [`SYNTAX_GUIDE_EN.md`](docs/SYNTAX_GUIDE_EN.md) | [`SYNTAX_GUIDE.md`](docs/SYNTAX_GUIDE.md) |
| Core specification | [`CORE_SPEC.md`](docs/CORE_SPEC.md) | Use the Burmese course notes alongside the specification |
| Project roadmap | [`TODO_ZAP_MM.md`](docs/TODO_ZAP_MM.md) | Burmese roadmap |
| Language comparison | [`AUDIT_LANGUAGE_COMPARISON_2026-08.md`](docs/AUDIT_LANGUAGE_COMPARISON_2026-08.md) | Comparative audit |

## Current Roadmap

The next development areas are deeper control-flow type narrowing, HTTP/URL/Regex standard-library modules, package metadata and lockfiles, asynchronous programming, and editor tooling. The project will continue to prioritize a stable language core, clear diagnostics, cross-platform behavior, and synchronized English/Burmese documentation.

## Contributing

Before opening a pull request, run the native test suite and whitespace check, update the relevant English and Burmese documentation, and add a regression test for behavior changes. Keep examples runnable against the current release line and describe any compatibility impact in the changelog.

## License

Zap is distributed under the MIT License. See [`LICENSE`](LICENSE) for the full license text.

## Links

- [English beginner guide](docs/LEARN_ZAP_EN.md)
- [မြန်မာ beginner guide](docs/LEARN_ZAP_MM.md)
- [English syntax reference](docs/SYNTAX_GUIDE_EN.md)
- [မြန်မာ syntax reference](docs/SYNTAX_GUIDE.md)
- [Releases](https://github.com/hidecard/zap/releases)
- [Issue tracker](https://github.com/hidecard/zap/issues)
