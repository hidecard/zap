# Zap Programming Language

![Zap Programming Language banner](assets/branding/zap-banner.png)

[![Zap CI](https://github.com/hidecard/zap/actions/workflows/ci.yml/badge.svg)](https://github.com/hidecard/zap/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/hidecard/zap?display_name=tag&sort=semver&color=2ea44f)](https://github.com/hidecard/zap/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey.svg)](https://github.com/hidecard/zap/actions)
[![Source: .zp](https://img.shields.io/badge/source-.zp-8A2BE2.svg)](README.md)

**Documentation:** [English](README.md) · [မြန်မာ](README_MM.md) · [Documentation hub](docs/DOCUMENTATION_NAVIGATION_EN.md) · [Language Guide](docs/LEARN_ZAP_EN.md)

> **Zap** is a readable, general-purpose programming language with `.zp` source files, indentation-based blocks, optional type checking, explicit modules, structured errors, and a standalone native runtime.

Zap is distributed as a native executable. After Zap is installed, a project can be created, checked, built, tested, and served without installing Python, Node.js, Java, or Rust as application runtime dependencies. HTML, CSS, plain JavaScript, or the built output of React, Vue, Svelte, and other frontend tools can be placed under the project’s `public/` directory and served by Zap.

## Current release

| Item | Status |
|---|---|
| Current release line | `v2.10.0` |
| Source files | `.zp`, commonly `main.zp` |
| Project manifest | `zap.toml` |
| Lockfile | `zap.lock` |
| Runtime | Standalone native executable |
| Platforms | Linux x86_64, Windows x86_64, macOS ARM64 |
| License | MIT |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |

## Install

Download the archive matching your operating system and CPU architecture from the [v2.10.0 release page](https://github.com/hidecard/zap/releases/tag/v2.10.0), verify its checksum and signature, extract it, and place the `zap` executable on `PATH`.

### Linux

```bash
tar -xzf zap-2.10.0-linux-x86_64.tar.gz
cd zap
bash install.sh
zap --version
```

### macOS ARM64

```bash
tar -xzf zap-2.10.0-macos-arm64.tar.gz
cd zap
chmod +x install.sh
./install.sh
zap --version
```

### Windows

The expected archive is `zap-2.10.0-windows-x86_64.zip`.

```bat
cd C:\Zap
install_windows.bat
zap.exe --version
```

If you do not have administrator access, keep the executable in a user-owned directory and add that directory to `PATH`. The [complete Language Guide](docs/LEARN_ZAP_EN.md) contains platform-specific installation and verification details.

## Create a project with one command

Zap intentionally uses a simple, user-managed project workflow. There is no Django-style `startapp` command and no hidden app registry.

```bash
zap new my_app
cd my_app
zap check
zap build --locked
zap test tests
zap dev
```

The single generator creates the following structure:

```text
my_app/
├── zap.toml
├── zap.lock
├── main.zp
├── web.zp
├── server.zp
├── models/
├── functions/
├── ui/
├── routes/
├── middleware/
├── migrations/
├── admin/
├── public/
└── tests/
```

These are ordinary user-owned directories. Add, remove, rename, and organize modules directly inside the project as it grows. Use `models/` for data shape and validation, `functions/` for business logic, `ui/` for browser UI metadata, `routes/` for HTTP routes, `middleware/` for request/response policy, `migrations/` for schema changes, `admin/` for optional administration registrations, `public/` for browser assets, and `tests/` for executable checks.

## First Zap program

Create `hello.zp`:

```zap
fn greet(name: text) -> text:
    return "Hello, " + name

say greet("Zap")
```

Run it with:

```bash
zap hello.zp
# or
zap run hello.zp
```

## Language at a glance

```zap
let scores: list<number> = [80, 45, 90]

fn passed(score: number) -> bool:
    return score >= 50

for score in scores:
    if passed(score):
        say "passed: " + str(score)
```

The language includes text, numbers, booleans, lists, maps, objects, functions, classes, inheritance, optional annotations, closures, explicit modules, JSON, `Result`/`Option`, default and named arguments, bounded asynchronous tasks, and deterministic diagnostics.

## Documentation

| Need | Start here |
|---|---|
| Install and learn Zap from beginner to advanced | [English Language Guide](docs/LEARN_ZAP_EN.md) · [မြန်မာ Language Guide](docs/LEARN_ZAP_MM.md) |
| Searchable syntax reference | [English syntax](docs/SYNTAX_GUIDE_EN.md) · [မြန်မာ syntax](docs/SYNTAX_GUIDE.md) |
| Normative language behavior | [English specification](docs/LANGUAGE_SPEC_EN.md) · [မြန်မာ specification](docs/LANGUAGE_SPEC_MM.md) |
| Standard library | [English index](docs/STDLIB_INDEX_EN.md) · [မြန်မာ index](docs/STDLIB_INDEX_MM.md) |
| Package and lockfile workflow | [English package guide](docs/PACKAGE_EN.md) · [မြန်မာ package guide](docs/PACKAGE.md) |
| Web framework and frontend integration | [Zap Web guide](docs/ZAP_WEB_NATIVE_EN.md) · [Frontend integration](docs/FRONTEND_INTEGRATION_EN.md) |
| Runtime, memory, and async boundaries | [Runtime state](docs/RUNTIME_STATE_EN.md) · [Memory contract](docs/MEMORY_BUDGET_OBJECT_STORE_EN.md) · [Async boundaries](docs/ASYNC_BOUNDARIES_EN.md) |
| Host adapter and deployment | [Host guide](docs/ZAP_HOST_EN.md) · [Deployment guide](docs/DEPLOYMENT_EN.md) |
| Burmese documentation navigation | [မြန်မာ documentation hub](docs/DOCUMENTATION_NAVIGATION_MM.md) |

## CLI essentials

```bash
zap file.zp                 # run a source file
zap new my_app               # create a complete user-managed Web project
zap check .                  # validate a Zap project directory
zap check --json .           # emit structured diagnostics
zap build --locked .         # validate reproducible build inputs
zap test tests               # run Zap tests
zap fmt main.zp              # format source
zap lint main.zp             # report style issues
zap lock                    # generate canonical lock data
zap install                 # validate locked dependencies
zap update                  # regenerate lock data after manifest changes
zap web check               # validate Web configuration
zap dev                     # start the bounded development server
zap --help                  # show all commands
```

## Frontend integration

Plain HTML, CSS, and JavaScript work without a JavaScript runtime in production:

```html
<script type="module" src="/assets/app.js"></script>
```

A React, Vue, Svelte, or other frontend project may be built separately and its output copied into `public/`. Zap serves the resulting files; it does not require npm or Node.js at deployment time. Keep API route declarations in `routes/` and browser assets in `public/` as described in the [frontend integration guide](docs/FRONTEND_INTEGRATION_EN.md).

## What is implemented and what is deferred

The current stable direction covers the `.zp` language core, native CLI, project manifests and lockfiles, typed checks, modules, classes, Result/Option, JSON, tests, formatter/linter, structured diagnostics, LSP foundations, a user-managed Web scaffold, bounded native Web serving, and SQLite-first migration contracts.

A complete ORM, provider-neutral production migration platform, user-defined trait syntax, production asynchronous I/O reactor, cross-file semantic rename, template compiler, and hidden app registry are not claimed as complete. Their status is tracked in the [language specification](docs/LANGUAGE_SPEC_EN.md), contracts, tests, and release notes.

## Development

Zap itself is implemented in Rust. To build the runtime from source, install the pinned toolchain described by `rust-toolchain.toml`, then run:

```bash
cargo test --manifest-path native/Cargo.toml --all-targets
cargo build --release --manifest-path native/Cargo.toml
```

Before contributing, also run the documentation, Web scaffold, release-version, VS Code asset, and LSP parity validators described in the [documentation hub](docs/DOCUMENTATION_NAVIGATION_EN.md).

## Release provenance

The current source baseline is v2.10.0. The preceding v2.3.0, v2.2.7, and earlier release records remain available in [GitHub Releases](https://github.com/hidecard/zap/releases) and the bilingual `CHANGELOG` files. Release artifacts are published only after version consistency, native tests, cross-platform builds, security checks, documentation checks, and installer verification pass.

## License

Zap is released under the [MIT License](LICENSE).
