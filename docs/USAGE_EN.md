# Zap Usage Guide

**Verified baseline:** Zap v2.2.6 maintenance branch

**Purpose:** This guide covers installation, project development, dependency locking, testing, registry use, and the production boundary. The v2.2.6 branch is a release candidate; use the latest published release from [GitHub Releases](https://github.com/hidecard/zap/releases) until the candidate is formally published.

## 1. Install the native runtime

Zap is distributed as a standalone native executable. End users should download the archive for the correct operating system and architecture, verify its checksum, and install the executable. Do not run an archive built for a different operating system or CPU architecture.

| Platform | Release asset | Installation |
|---|---|---|
| Linux x86_64 | `zap-<version>-linux-x86_64.tar.gz` | Extract and run `bash install.sh` |
| macOS ARM64 | `zap-<version>-macos-arm64.tar.gz` | Extract, run `chmod +x install.sh`, then `./install.sh` |
| Windows x86_64 | `zap-<version>-windows-x86_64.zip` | Extract and run `install_windows.bat` from Command Prompt |

On Linux or macOS, verify an archive before installation:

```bash
sha256sum -c zap-<version>-linux-x86_64.tar.gz.sha256
# Replace the archive name above with the macOS asset when applicable.
```

After extraction, install the user-level binary and verify it:

```bash
cd zap
bash install.sh
zap --version
zap --help
```

The installer places the executable in `~/.local/bin` by default. Set `ZAP_INSTALL_DIR` to choose another user-writable directory. The installer does not require root privileges. A release archive containing `bin/zap` does not require Rust or Cargo.

### Build from source

A source build is an explicit developer or operator action. The repository pins Rust 1.75.0 in `rust-toolchain.toml`; use the locked build so the dependency graph cannot change during installation:

```bash
ZAP_BUILD_FROM_SOURCE=1 bash install.sh
```

For a repository checkout, the equivalent reproducible build is:

```bash
cargo build --release --locked --manifest-path native/Cargo.toml
./native/target/release/zap --version
```

## 2. Create and run a project

A minimal project contains a `main.zp` source file and, when dependencies are used, a `zap.toml` manifest and a committed `zap.lock` lockfile:

```bash
mkdir hello-app
cd hello-app
cat > main.zp <<'EOF'
say "Hello from Zap"
EOF
zap check .
zap run main.zp
```

A project manifest can declare package identity and dependencies:

```toml
[package]
name = "hello-app"
version = "0.1.0"
main = "main.zp"
```

Use `zap init <directory>` to create the standard scaffold. Use `zap check` before execution, and use `zap check --json .` when an editor or CI system needs structured diagnostics. Local modules are resolved from the main-file directory and the supported project module directories; module cycles and unsafe paths are rejected.

## 3. CLI workflow

| Command | Use |
|---|---|
| `zap <file.zp>` | Run a source file through the canonical native AST runtime. |
| `zap run <file.zp>` | Explicitly run a source file. |
| `zap init <dir>` | Create a project scaffold. |
| `zap fmt <file.zp>` | Format source code. |
| `zap lint <file.zp>` | Check source formatting and style. |
| `zap check [dir]` | Validate the manifest, modules, types, and project structure. |
| `zap check --json [dir]` | Emit structured diagnostics for CI or editor integrations. |
| `zap test [dir]` | Run `*_test.zp` files in deterministic path order. |
| `zap test --fail-fast [dir]` | Stop after the first user-facing test failure. |
| `zap lock [dir]` | Generate the canonical `zap.lock`. |
| `zap add <name> <version> [dir]` | Add a dependency and invalidate the old lockfile. |
| `zap install [dir]` | Validate the project and install from its lockfile/cache. |
| `zap install --locked [dir]` | Require an existing valid lockfile and refuse graph changes. |
| `zap update [dir]` | Regenerate the lockfile from the manifest. |
| `zap registry gc [--dry-run] [dir]` | Remove unreferenced cache artifacts, or preview the removal. |
| `zap lsp` | Run the stdio language server for editor integration. |
| `zap async-check` | Validate the deterministic async runtime foundation. |

A normal development loop is:

```bash
zap fmt main.zp
zap lint main.zp
zap check .
zap test .
zap build --locked .
zap install --locked .
```

The native runtime applies bounded source, execution-depth, loop, output, memory, task, and collection limits. These are runtime safety limits, not a replacement for OS-level isolation.

## 4. Language examples

Zap uses indentation-based blocks and readable expressions:

```zap
fn greet(name):
    return "Hello, " + name

for item in ["language", "runtime", "tooling"]:
    say greet(item)
```

A typed result can be checked and propagated with `?`:

```zap
fn load_name(value: text) -> result<text>:
    if value == "":
        return err("name is empty")
    return ok(value)
```

The language includes functions, closures, classes, modules, collections, JSON values, `Result`/`Option` values, async task handles, and deterministic tests. Consult the [language specification](LANGUAGE_SPEC_EN.md) for normative behavior rather than relying on an older example.

## 5. Files, JSON, and environment access

The standard library includes bounded text and line-based file helpers, JSON encoding/decoding, path helpers, time helpers, logging, and environment access. Example:

```zap
let lines = ["one", "two"]
write_lines("notes.txt", lines)
let loaded = read_lines("notes.txt")
say json({"lines": loaded})
```

When a run owns an `ExecutionContext`, relative file operations are confined to that run's workspace. Existing symlink and canonicalization checks are defensive controls; they do not make a process a kernel sandbox. For untrusted programs, use an isolated worker with a read-only source tree, a dedicated writable directory, minimal environment variables, quotas, and network egress restrictions.

## 6. Dependency and registry workflow

For a dependency-backed project, generate and commit the lockfile:

```bash
zap add utility 1.2.0 .
zap lock .
zap check .
zap install --locked .
```

`zap install --locked` verifies that the manifest, lockfile, registry metadata, selected versions, yanked policy, and SHA-256 cache artifacts agree. `ZAP_OFFLINE=1 zap install --locked .` permits only already-cached, checksum-verified artifacts and performs no network retrieval.

Configure a remote registry explicitly. A remote origin must be trusted before a request is made, and HTTP is disabled unless it is deliberately enabled for a controlled local fixture:

```bash
zap registry trust add https://registry.example/team
export ZAP_REGISTRY_TOKEN_CI='read-token-from-your-secret-manager'
zap registry credential set https://registry.example/team --token-env ZAP_REGISTRY_TOKEN_CI
zap install --locked .
```

The credential list command prints origins, never token values. Credentials must be kept in a secret manager or protected environment variable; never commit them to `zap.toml`, `zap.lock`, source code, logs, or CI output.

To publish a package, compute its checksum locally and send it through the HTTPS endpoint:

```bash
checksum="$(sha256sum ./demo.pkg | awk '{print $1}')"
export ZAP_REGISTRY_TOKEN='publish-token-from-your-secret-manager'
zap registry publish https://registry.example/team/publish ./demo.pkg demo 1.0.0 "$checksum"
```

The client verifies the package checksum before sending the body. Registry fetch/publish paths disable automatic redirects; in untrusted mode they resolve the registry host once, reject special/private destinations, and pin the connection to the validated address set. TLS certificate validation still uses the normal platform trust configuration.

## 7. Testing and CI

Application tests use `*_test.zp` names under `tests/` or a selected test directory:

```bash
zap test --fail-fast .
```

Runtime contributors should run the complete locked native gates:

```bash
cargo fmt --manifest-path native/Cargo.toml --all -- --check
cargo check --manifest-path native/Cargo.toml --all-targets --all-features --locked
cargo clippy --manifest-path native/Cargo.toml --all-targets --all-features --locked -- -D warnings
cargo test --manifest-path native/Cargo.toml --all-targets --all-features --locked
scripts/validate_registry_deployment.sh
```

The CI and release workflows also run RustSec auditing with `cargo-audit`, deployment-policy validation, deterministic replay, native/legacy parity, archive checks, and release provenance checks. Read [`RUSTSEC_AUDIT_EN.md`](RUSTSEC_AUDIT_EN.md) for the dependency evidence and known audit-tool compatibility boundary.

## 8. Production security boundary

`ZAP_UNTRUSTED=1` denies filesystem, environment, process, outbound network, and local-registry capabilities at the runtime boundary. It must be combined with OS-level controls for production use:

```bash
ZAP_UNTRUSTED=1 zap check --json .
ZAP_UNTRUSTED=1 zap run main.zp
```

Do not expose the native process directly to the Internet. The production registry reference deployment binds the service to loopback, terminates TLS at an ingress proxy, uses a dedicated service identity, limits memory/CPU/tasks/open files, disables backend egress, and keeps credentials outside the repository. Follow the [production operations guide](PRODUCTION_OPERATIONS_EN.md) for the complete systemd/nginx runbook.

The runtime is **not** a universal OS sandbox, does not provide kernel-enforced multi-tenant isolation, and does not provide built-in metrics or a durable backup system. Operators must provide isolation, monitoring, alerting, backups, restore drills, key rotation, firewall policy, certificate renewal, and incident response.

## 9. VS Code and LSP

Install the published extension when available:

```bash
code --install-extension ArkarYan.zap-language-support
```

The extension uses `zap lsp`. Ensure that `zap` is on `PATH` or configure `zap.executable` in VS Code settings. The LSP currently supports full document synchronization, diagnostics, hover, completion, formatting, definitions, workspace symbols, and file-local rename. Cross-file rename remains unsupported and should be reviewed before applying automated refactors.

## 10. Uninstall

The Unix installer uses a user-level directory. Run `uninstall.sh` or remove the installed binary and the Zap PATH line from the relevant shell profile. On Windows, run `uninstall_windows.bat` or remove the user-level `.zap\bin\zap.exe` and PATH entry. Uninstalling the CLI does not delete project files, registry data, or credentials.

## References

The normative references are the [language specification](LANGUAGE_SPEC_EN.md), [package guide](PACKAGE_EN.md), [registry authentication contract](REGISTRY_AUTH_EN.md), [deployment boundaries](DEPLOYMENT_EN.md), [production operations guide](PRODUCTION_OPERATIONS_EN.md), [security policy](../SECURITY.md), and [RustSec audit evidence](RUSTSEC_AUDIT_EN.md).
