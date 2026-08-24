# Zap Usage Guide

**Verified baseline:** Zap v2.8.0 development line

**Purpose:** This guide is a compact command and operations reference. For the complete installation-to-advanced learning path, use the [English Language Guide](LEARN_ZAP_EN.md). Normative behavior belongs to the [language specification](LANGUAGE_SPEC_EN.md).

## Install the native runtime

Zap is distributed as a standalone native executable. Download the archive that matches the operating system and CPU architecture, verify its checksum and signature, extract it, and place the executable on `PATH`. The v2.8.0 release will publish only the targets listed on its release page; do not infer support for an unlisted target.

| Platform | Archive pattern | Installation |
|---|---|---|
| Linux x86_64 | `zap-<version>-linux-x86_64.tar.gz` | Extract and run `bash install.sh`. |
| macOS ARM64 | `zap-<version>-macos-arm64.tar.gz` | Extract, run `chmod +x install.sh`, then `./install.sh`. |
| Windows x86_64 | `zap-<version>-windows-x86_64.zip` | Extract and run `install_windows.bat` from Command Prompt. |

A release archive containing `bin/zap` does not require Rust, Cargo, Python, Node.js, Java, or another language runtime on the application host. The installer uses a user-writable location by default; set `ZAP_INSTALL_DIR` when a different destination is required.

## Create a project

Use the one-command, user-managed Web scaffold. Zap deliberately has no Django-style `startapp` command and no hidden application registry.

```bash
zap new my_app
cd my_app
zap check
zap build --locked
zap test tests
zap dev
```

The generated directories are ordinary user-owned files: `models/`, `functions/`, `ui/`, `routes/`, `middleware/`, `migrations/`, `admin/`, `public/`, and `tests/`. Add, remove, rename, and organize modules directly. The scaffold is a development/reference Web slice; it is not by itself a production server, ORM, admin UI, authentication system, or deployment supervisor.

For a small non-Web project, create a directory with `zap.toml` and `main.zp`, or use the compatibility `zap init <directory>` command. New documentation and new projects should prefer `zap new` for the complete user-managed workflow.

## CLI workflow

| Command | Purpose |
|---|---|
| `zap <file.zp>` | Run a source file. |
| `zap run <file.zp>` | Explicit source-file execution. |
| `zap new <dir>` | Create the complete user-managed Web scaffold. |
| `zap check [dir]` | Validate a Zap project directory, manifest, modules, and known types. |
| `zap check --json [dir]` | Emit structured project diagnostics. |
| `zap build [dir]` | Validate and prepare a project. |
| `zap build --locked [dir]` | Require the existing lockfile and reject graph changes. |
| `zap test [dir]` | Run `*_test.zp` files in deterministic order. |
| `zap test --filter <value> [dir]` | Run matching tests. |
| `zap test --fail-fast [dir]` | Stop after the first test failure. |
| `zap test --json [dir]` | Emit machine-readable test results where supported. |
| `zap fmt <file.zp>` | Format Zap source. |
| `zap lint <file.zp>` | Report formatting and style issues. |
| `zap lock [dir]` | Generate canonical `zap.lock`. |
| `zap add <name> <version> [dir]` | Add a dependency and invalidate the old lockfile. |
| `zap install [dir]` | Validate and install from the lockfile/cache. |
| `zap install --locked [dir]` | Require a valid existing lockfile. |
| `zap update [dir]` | Regenerate lock data after manifest changes. |
| `zap web check [dir]` | Validate Web configuration and project structure. |
| `zap db check [dir]` | Validate migration layout and database plan. |
| `zap db plan [dir] --json` | Show a read-only SQLite migration plan. |
| `zap db migrate [dir] --dry-run` | Preview migrations without applying them. |
| `zap db migrate [dir] --check` | Fail when pending migrations exist. |
| `zap dev [dir]` | Run the bounded native development server. |
| `zap lsp` | Run the stdio language server. |

A normal development loop is:

```bash
zap fmt main.zp
zap lint main.zp
zap check .
zap test tests
zap build --locked .
zap db check .
```

## Packages and lockfiles

Declare package identity and dependencies in `zap.toml`, commit the canonical `zap.lock`, and use `zap install --locked` in reproducible environments. Local path dependencies are validated recursively and dependency cycles are rejected. Registry operations enforce configured transport, checksums, signatures, cache bounds, and credential policy, but the registry foundation is not yet equivalent to the package volume and governance of npm, PyPI, crates.io, or Go modules.

## Web and frontend boundary

HTML, CSS, and JavaScript are ordinary files under `public/`. React, Vue, Svelte, Alpine, or another browser framework may be used as a separate build-time toolchain. Copy the generated output into the declared asset directory; the deployed Zap executable serves the files and does not execute npm or Node.js at runtime. See the [frontend integration guide](FRONTEND_INTEGRATION_EN.md) and [Zap Web guide](ZAP_WEB_NATIVE_EN.md).

The current Web runtime provides bounded request/response and static/SPA development behavior. TLS termination, production concurrency, WebSocket, streaming uploads, session persistence, provider-neutral database drivers, ORM behavior, SSR/template compilation, cache invalidation, observability, and process supervision remain explicit host/deployment work.

## Safety boundary

Runtime limits for source size, execution depth, loops, values, collections, files, process output, HTTP requests, and tasks are reliability controls. They are not an OS sandbox. Untrusted Zap code must run inside an operating-system isolation profile with an explicit filesystem, process, network, identity, and secret policy.

## Source development

The repository pins its Rust toolchain in `rust-toolchain.toml`. Source contributors should use the locked dependency graph and run the complete native tests, formatting, strict Clippy, documentation consistency, release-version, VS Code asset, LSP parity, framework, and release-preflight gates described in the [documentation navigation](DOCUMENTATION_NAVIGATION_EN.md).
