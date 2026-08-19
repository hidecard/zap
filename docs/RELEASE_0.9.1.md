# Zap v0.9.1 — Module, Result, and Documentation Hardening

**Release date:** 2026-08-19

Zap v0.9.1 is a post-v0.9.0 hardening release. It strengthens the module system, adds automatic Result error propagation, improves static assignment checking, and makes the project documentation available in English and Burmese.

## Highlights

### Explicit and safer modules

Zap now uses explicit `import` and `export` visibility. Symbols that are not exported remain private to their defining module. Canonical-path module caching prevents repeated top-level execution during a single runtime. The loader detects circular imports and rejects absolute module paths at the module boundary.

### Static assignment checking

The checker validates annotated variable assignments against literal and inferred expression types. Mismatches are reported as structured, line-aware diagnostics instead of being deferred until an unrelated runtime operation.

### Result error propagation

The `?` operator can unwrap a successful Result or return an error Result from the current function:

```zap
fn load_user() -> Result:
    return err("user not found")

fn profile() -> Result:
    let user = load_user()?
    return ok(user)
```

Using `?` on a non-Result value is rejected. Success, error, and invalid-operand behavior are covered by integration tests.

### Bilingual documentation

The main README is now written in English and includes a language chooser. The repository provides English and Burmese beginner courses and syntax references:

- [`docs/LEARN_ZAP_EN.md`](LEARN_ZAP_EN.md)
- [`docs/LEARN_ZAP_MM.md`](LEARN_ZAP_MM.md)
- [`docs/SYNTAX_GUIDE_EN.md`](SYNTAX_GUIDE_EN.md)
- [`docs/SYNTAX_GUIDE.md`](SYNTAX_GUIDE.md)

## Verification

- Native integration tests: **34 passing**
- `cargo test --manifest-path native/Cargo.toml`: passed
- `git diff --check`: passed
- Release workflow remains configured for Linux, Windows, and macOS ARM64 artifacts.

## Known limitations

The dedicated `ZapError` diagnostic boundary is now included. The remaining roadmap items are advanced branch/loop type narrowing, Result/Option payload checking, HTTP/URL/Regex standard-library modules, package lockfiles, and LSP/editor tooling. Some evaluator internals still use legacy Rust `String` error paths and are planned for a later architecture refactor.

## Upgrade

Download the archive for your operating system and CPU architecture from the [GitHub Releases page](https://github.com/hidecard/zap/releases). Existing `.zp` source files remain compatible with the v0.9.x release line, subject to the current language roadmap and documented behavior.
