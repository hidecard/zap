# Zap v2.11.18 Release Notes

## Release summary

- B2 type checker: complete generic constraints, compound bounds, alias checking & verifiers.
- B1 lexer/parser: expanded arbitrary block coverage and parser diagnostic parity.
- Typed-IR generalization: arbitrary expression typed-IR, control-flow typed-IR, and cross-module typed-IR.
- B3 package/build: Zap-side package resolver, dependency graph ownership, and typed-IR to bytecode lowering.
- B4 self-hosting: deterministic rebuild evidence, second-stage compiler rebuild, and clean environment verification.
- Bootstrap validation gates: B0, B1, B3, VM platform, non-Rust seed pipeline, and B4 byte-determinism all passed.

## Platform support

- Linux x86_64: tested
- macOS ARM64: build/test pending CI
- Windows x86_64: build/test pending CI

## Security

- RustSec cargo audit: advisory database CVSS 4.0 compatibility note (cargo-audit 0.17.0; newer advisory DB format pending toolchain update).
- Added filesystem race boundary and process cleanup regression tests.
- Added DNS-to-connection pinning security regression tests.
- Added dependency license check regression tests.

## Implemented scope

This release includes the following implemented features and improvements:
- P0.1: Native CLI release gate with version validation and helpful build instructions
- P0.2: Cross-platform release verification for Linux x86_64, macOS ARM64, and Windows x86_64
- P0.3: Runtime/security regression gates including filesystem race boundary, DNS pinning, and license checks
- B2 type checker enhancements with complete generic constraints and compound bounds
- B1 lexer/parser improvements with expanded arbitrary block coverage
- B3 package/build foundations with Zap-side package resolver
- B4 self-hosting evidence with deterministic rebuild verification

## Deferred scope

The following features remain deferred for future releases:
- Full framework ecosystem expansion (P1+ deferred until P0-P2 acceptance)
- Self-hosting production deployment claims (P3 deferred until P2 ownership transfer)
- Broad async feature expansion beyond current bounded foundation
- Package registry without full B3 implementation
- Performance claims beyond current benchmark baseline
