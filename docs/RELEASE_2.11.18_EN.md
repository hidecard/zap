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
