# Zap-Only Dependency Policy

**Status:** Proposed self-hosting policy for Zap v2.9.0

## Goal

Zap must eventually build and run its compiler, type checker, standard library, package manager, and bootstrap test runner from Zap-owned source without requiring Rust, Cargo, Go, Python, Node.js, JavaScript frameworks, or third-party language runtimes.

This goal does not claim that Zap can operate without an operating system, CPU architecture, executable loader, linker, filesystem, or minimal platform seed. Those are platform boundaries, not language/framework dependencies.

## Dependency classes

| Class | Examples | Long-term policy |
|---|---|---|
| Zap-owned | `.zp` compiler sources, Zap stdlib, typed IR, bootstrap fixtures | Required and versioned in this repository |
| Temporary bootstrap | Existing Rust reference runtime and Cargo build | Allowed only for B0/B1 transition; never required by a clean B4 build |
| Platform seed | OS loader, syscalls, libc/ABI or a tiny platform-specific seed binary | Minimize, document, and keep outside language semantics |
| Forbidden build dependency | Python/Node/Go/Cargo scripts needed to compile the Zap compiler | Must be removed from the B4 compiler path |
| Optional integration | React/Vue/Svelte, external databases, reverse proxies | May support applications but cannot be required by Zap core |

## Required architecture

The Zap compiler path must be split into pure Zap-owned layers: lexer, parser/AST, type checker, diagnostic model, typed IR, pure standard library, package resolver, and bootstrap test runner. Filesystem, process, network, Web, and database operations must be capability-backed host adapters rather than implicit compiler dependencies.

The first self-hosted compiler should emit a stable typed IR or canonical artifact format. It should not immediately emit native machine code. Native code generation can be added after B4 reproducibility is proven, using a Zap-owned backend or a documented minimal platform seed.

## B0–B4 acceptance

- **B0:** the current Rust implementation is the reference implementation and owns the initial behavior.
- **B1:** Zap lexer/parser output matches B0 token, AST, and diagnostic fixtures.
- **B2:** Zap type checker matches B0 accept/reject decisions and diagnostic JSON.
- **B3:** Zap stdlib, package resolver, and typed IR bridge run without network or ambient environment.
- **B4:** a clean machine with no Rust/Cargo, Go, Python, Node.js, or JavaScript runtime can rebuild the compiler from Zap source using only the documented platform seed.

Every stage must record source hashes, compiler hashes, schema versions, and artifact hashes. Two clean consecutive builds must produce identical canonical artifacts.

## Forbidden shortcuts

Removing `Cargo.toml` before a Zap compiler exists is not self-hosting; it only removes the current build path. Replacing Rust with another host language is also not Zap-only. A third-party parser, VM, package manager, or Web framework cannot be part of the core bootstrap path unless it is reimplemented or vendored as Zap-owned source with a clear license and deterministic contract.

## Release gate

A release may claim “Zap-only self-hosted” only when B4 passes on a clean machine, all compiler and stdlib sources are Zap-owned, the bootstrap test runner is Zap-owned, the platform seed is documented, and no forbidden runtime is required by the build command. Until then, release language must say “Rust-bootstrapped” or “self-hosting foundation,” not “fully self-hosted.”
