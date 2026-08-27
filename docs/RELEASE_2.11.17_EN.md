# Zap v2.11.17

**Release status:** Published after complete validation and public artifact/signature verification. Zap remains at bootstrap stage B0; the B4 work in this release is bounded and provisional.

## Summary

Zap v2.11.17 extends the canonical parser-AST → B3 lowerer → B4 VM path with bounded closure execution. Nested AST functions can capture outer values, return closure values, and execute with independent captured environments. The release also retains the earlier canonical AST control-flow, exception, class, inheritance, and C3 `super()` slices.

## Changes

| Area | Change | Boundary |
|---|---|---|
| Canonical AST closures | Nested functions capture referenced outer values and can be returned and invoked. | Bounded lexical capture; no full heap-level shared-cell or cycle collector model |
| B4 module boundary | AST control-flow lowering is isolated in `bootstrap/b4/ast_control.zp`. | Rust/native runtime remains the complete semantics owner |
| Typed-IR handoff | `seed_compile_typed_ir` exposes the existing typed-IR payload-to-VM slice. | Not complete typed-IR production or compiler ownership |
| Verification | Added closure, control-flow, try/catch, literal-list `for`, and typed-IR-to-VM gates. | Literal-list `for` only; general iterators remain deferred |

## Verification contract

The release source passed the bootstrap verifier matrix, native Rust tests, formatting, whitespace, release preflight, cross-platform build jobs, artifact manifest/checksum/provenance validation, and detached-signature verification. The authoritative release version is sourced from `native/Cargo.toml` and checked against the lockfile, CLI, documentation, and release surfaces.

## Deferred scope

This release is not full B4 self-hosting. Arbitrary-program parser coverage, complete type inference, complete typed-IR production, package/build ownership, platform-seed acceptance, Rust-independent self-rebuild, production garbage collection, production asynchronous I/O, general runtime iterators, full trait runtime, and complete object/type semantics remain deferred. `native_independent:false` remains intentional.
