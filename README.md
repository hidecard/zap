# Zap Programming Language

![Zap Programming Language banner](assets/branding/zap-banner.png)

[![Zap CI](https://github.com/hidecard/zap/actions/workflows/ci.yml/badge.svg)](https://github.com/hidecard/zap/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/hidecard/zap?display_name=tag&sort=semver&color=2ea44f)](https://github.com/hidecard/zap/releases/latest)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey.svg)](https://github.com/hidecard/zap/actions)
[![Source: .zp](https://img.shields.io/badge/source-.zp-8A2BE2.svg)](README.md)

**Documentation:** [English](README.md) · [မြန်မာ](README_MM.md) · [Documentation hub](docs/DOCUMENTATION_NAVIGATION_EN.md) · [Language Guide](docs/LEARN_ZAP_EN.md)

> **Zap is a readable, native-first general-purpose programming language for developers who want a small, explicit project workflow without giving up structured types, diagnostics, and cross-platform distribution.**

Zap is implemented in Rust and distributed as a standalone native executable. It currently provides a `.zp` language core, project manifests and lockfiles, CLI tooling for checking/building/testing/formatting/linting, a user-managed Web scaffold, and a bounded native development server. The current release line is **`v2.9.0`**. Advanced production features listed below are not yet claimed complete.

## Why Zap?

Zap is designed for developers who value readable syntax, explicit project ownership, and a compact native workflow. It is a good fit for learning, experiments, focused command-line tools, small services, and teams that want to explore a language with a simple project model.

Zap is not intended to replace every established ecosystem. Rust remains the stronger choice for systems work that needs its mature safety and library ecosystem; Go is the safer default for large cloud-service teams; Python remains the broadest choice for automation and data work; and TypeScript remains the natural choice for browser-first products. Zap’s opportunity is to make a focused, beginner-friendly, native-first workflow compelling enough to earn adoption beyond the repository.

| Zap differentiator | What it means |
|---|---|
| **Native-first distribution** | Zap ships as a standalone executable, so an application does not need a Zap, Python, Node.js, Java, or Rust runtime installed separately at deployment time. |
| **Readable syntax** | `.zp` files use indentation-based blocks, optional type annotations, explicit modules, and structured diagnostics. |
| **Explicit project structure** | Generated directories such as `routes/`, `models/`, `functions/`, `tests/`, and `public/` are ordinary user-managed modules rather than a hidden application registry. |
| **One CLI workflow** | Project validation, locked builds, tests, formatting, linting, package locking, and Web checks are exposed through the `zap` command. |

## Current status

| Item | Status |
|---|---|
| Current release line | `v2.9.0` |
| Source files | `.zp`, commonly `main.zp` |
| Project manifest | `zap.toml` |
| Lockfile | `zap.lock` |
| Runtime | Standalone native executable |
| Platforms | Linux x86_64, Windows x86_64, macOS ARM64 |
| License | MIT |
| Repository | [github.com/hidecard/zap](https://github.com/hidecard/zap) |
| Releases | [GitHub Releases](https://github.com/hidecard/zap/releases) |

> **Security boundary:** `ZAP_UNTRUSTED=1` is a defensive runtime mode for capability denial and request policy; it is not a kernel-enforced sandbox. Do not run untrusted source, downloaded plugins, or multi-tenant workloads without OS-level isolation, least-privilege filesystem permissions, network egress controls, resource quotas, process-group cleanup, and audit logging.

## Quickstart

### 1. Install Zap

Download the archive for your operating system and CPU architecture from the [`v2.9.0` release page](https://github.com/hidecard/zap/releases/tag/v2.9.0). Verify its checksum and signature before extracting it, then place the `zap` executable on your `PATH`.

#### Linux x86_64

```bash
tar -xzf zap-2.9.0-linux-x86_64.tar.gz
cd zap
bash install.sh
zap --version
```

#### macOS ARM64

```bash
tar -xzf zap-2.9.0-macos-arm64.tar.gz
cd zap
chmod +x install.sh
./install.sh
zap --version
```

#### Windows x86_64

The expected archive is `zap-2.9.0-windows-x86_64.zip`.

```bat
cd C:\Zap
install_windows.bat
zap.exe --version
```

If you do not have administrator access, keep the executable in a user-owned directory and add that directory to `PATH`. For platform-specific verification and troubleshooting, see the [Language Guide](docs/LEARN_ZAP_EN.md).

### 2. Create, check, build, test, and run a project

```bash
zap new hello_zap
cd hello_zap
zap check .
zap build --locked .
zap test tests
zap run main.zp
```

`zap dev` starts a bounded development server for local development. It is not a production hosting platform.

The generated project is intentionally ordinary and user-managed:

```text
hello_zap/
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

Use `models/` for data shape and validation, `functions/` for business logic, `ui/` for browser UI metadata, `routes/` for HTTP routes, `middleware/` for request/response policy, `migrations/` for schema changes, `admin/` for optional administration registrations, `public/` for browser assets, and `tests/` for executable checks. Add, remove, rename, and organize these modules directly as the project grows.

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

Expected output:

```text
Hello, Zap
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

## CLI reference

| Command | Purpose | Typical use |
|---|---|---|
| `zap new my_app` | Create a complete user-managed project scaffold | Start a project |
| `zap check .` | Validate a Zap project directory | Before a commit |
| `zap check --json .` | Emit structured diagnostics | CI and editor tooling |
| `zap build --locked .` | Build using reproducible lockfile inputs | Before a release |
| `zap test tests` | Run the project tests | After code changes |
| `zap fmt main.zp` | Format source code | Before review |
| `zap lint main.zp` | Report style issues | Before a commit |
| `zap lock` | Generate canonical lock data | After manifest changes |
| `zap install` | Validate locked dependencies | On a clean checkout |
| `zap update` | Regenerate lock data after manifest changes | Intentional dependency updates |
| `zap web check` | Validate Web configuration and route conflicts | Before serving a Web project |
| `zap web routes --json` | Inspect routes in machine-readable form | CI and integrations |
| `zap dev` | Start the bounded development server | Local development only |
| `zap --help` | Show all available commands | Discover the CLI |

## Web development model

Zap can serve plain HTML, CSS, JavaScript, or the output of a separately built React, Vue, Svelte, or other frontend project from `public/`:

```html
<script type="module" src="/assets/app.js"></script>
```

The boundary is intentional:

1. Frontend source can be built with its own toolchain before deployment.
2. The resulting browser assets can be copied into `public/`.
3. Zap serves those assets and handles the declared server-side routes.
4. Node.js or npm is not required as an application runtime dependency when serving the built output.
5. `routes/` remains user-managed; `zap web check` and `zap web routes` validate route configuration before a listener accepts traffic.

This Web foundation does not currently claim a complete ORM, built-in authentication, provider-neutral database abstraction, production async I/O reactor, WebSocket support, streaming uploads, SSR/template compilation, or built-in admin UI. See the status table below and the [Zap Web guide](docs/ZAP_WEB_NATIVE_EN.md).

## Feature status

The table distinguishes the current stable direction from areas that need more implementation and evidence. Do not interpret “implemented direction” as a guarantee that every production use case is complete.

| Area | Current status | Boundary |
|---|---|---|
| `.zp` language core | Implemented direction | Core values, functions, classes, modules, JSON, diagnostics, and related language behavior are documented and tested. |
| Native runtime | Implemented direction | Standalone executable and supported release platforms. |
| CLI tooling | Implemented direction | Check, build, test, format, lint, package lock, Web checks, and development serving. |
| Web scaffold | Implemented foundation | User-managed directories, route validation, and static frontend output. |
| Package and lock workflow | Available | Lockfile-based reproducibility and package commands. |
| LSP and editor support | Foundation | Coverage and parity should be checked against the editor documentation and fixtures. |
| ORM | Not claimed complete | Do not assume a production-ready database abstraction. |
| Production migrations | Not claimed complete | SQLite-first contracts do not equal a provider-neutral migration platform. |
| Advanced async I/O | Not claimed complete | Bounded asynchronous tasks do not equal a production I/O reactor. |
| Debugger and profiler integration | Not claimed complete | Treat as a tooling roadmap item. |
| WebSockets and streaming uploads | Not claimed complete | Treat as Web framework limitations for now. |
| Cross-file semantic rename | Not claimed complete | Use ordinary editor tooling until a language-aware refactor is available. |
| Hidden app registry | Intentionally not planned | Projects remain explicit and user-managed. |

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
| Release history | [English changelog](CHANGELOG_EN.md) · [မြန်မာ changelog](CHANGELOG_MM.md) |

## Development from source

Zap itself is implemented in Rust. Install the pinned toolchain described by [`rust-toolchain.toml`](rust-toolchain.toml), then run:

```bash
cargo test --manifest-path native/Cargo.toml --all-targets
cargo build --release --manifest-path native/Cargo.toml
```

Before opening a pull request, also run the documentation, Web scaffold, release-version, VS Code asset, and LSP parity validators described in the [documentation hub](docs/DOCUMENTATION_NAVIGATION_EN.md).

## Contributing

Contributions are welcome. Before changing the language or runtime, read [CONTRIBUTING.md](CONTRIBUTING.md), the relevant specification and contract documents, and the current release notes. Keep changes focused, add regression coverage, update both language versions of documentation when behavior changes, and run the native, documentation, Web scaffold, and release validation checks locally.

For a small first contribution, documentation corrections, executable examples, conformance fixtures, and focused regression tests are good places to start. Open an issue first for larger syntax, runtime, package, or Web framework changes so the proposed contract can be discussed before implementation.

## Bilingual documentation policy

`README.md` is the canonical release-facing README. `README_MM.md` is the Burmese companion. When a release changes commands, feature status, security boundaries, or supported platforms, update both files in the same change and verify that their claims remain equivalent. Language-specific explanations may differ, but release facts and limitations must not.

## Security and responsible use

Zap includes capability restrictions and bounded request/process policies for restricted operation. Those controls reduce risk but do not replace a kernel-enforced sandbox. Hosts that execute untrusted Zap code should isolate the worker, restrict filesystem and environment access, filter network egress, enforce CPU/memory/process quotas, clean up process groups, and record audit events.

For security reports, follow the process described in [SECURITY.md](SECURITY.md) rather than publishing a sensitive issue publicly.

## Release provenance

The current source baseline is `v2.9.0`. Release artifacts are published only after version consistency, native tests, cross-platform builds, security checks, documentation checks, and installer verification pass. Historical records remain available in [GitHub Releases](https://github.com/hidecard/zap/releases) and the bilingual changelog files.

## License

Zap is released under the [MIT License](LICENSE).
