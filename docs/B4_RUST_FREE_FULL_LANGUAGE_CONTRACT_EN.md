# B4 Rust-Free Full-Language Compiler Contract

**Contract ID:** `B4-RUST-FREE-FULL-LANGUAGE`  
**Schema:** 1  
**Status:** Not certified

## Purpose

This is the official acceptance boundary for B4. B4 is not a supported-subset demonstration. It is certified only when the complete language surface, compiler pipeline, user-facing CLI, package/build path, and test path are owned by Zap source and can execute without Rust, Cargo, or the Rust host compiler in the compiler path.

> A Rust-free seed pipeline is evidence of independence for a bounded slice. It is not evidence of full self-hosting. The B4 contract therefore separates contract integrity from B4 certification.

## Normative ownership requirements

| Area | B4 requirement | Required evidence |
|---|---|---|
| Language surface | Every syntax, expression, statement, type, generic, module, error, async, package, and runtime feature in the language specification has an acceptance fixture. | `bootstrap/contracts/B4_ACCEPTANCE.tsv` |
| Front end | Zap-owned lexer/parser produces the canonical AST for the full acceptance surface. | Source-to-AST fixture results |
| Static pipeline | Zap-owned type checker and typed-IR producer cover the same surface, including rejection behavior. | Typed-IR and diagnostic fixtures |
| Execution | Zap-owned lowering, bytecode/VM, and runtime execute accepted fixtures deterministically. | Source-to-VM fixture results |
| CLI | `check`, `build`, `run`, `test`, package, and diagnostic output paths dispatch through Zap-owned compiler code. | CLI ownership and no-fallback checks |
| Package/build | Manifest, lockfile, dependency, artifact, and rebuild operations are Zap-owned. | Package/build fixture results |
| Test runner | Test discovery, execution, result encoding, and failure reporting are Zap-owned. | Test-runner fixture results |
| Host boundary | Operating-system loading and explicitly documented platform primitives are the only seed boundary. | Platform-seed evidence |

## Forbidden compiler-path fallbacks

The compiler path MUST NOT invoke or depend on `cargo`, `rustc`, `rustup`, the Rust native implementation, or the Rust host wrapper. Those components may remain as a reference oracle and development tool until the B4 migration is complete, but they cannot be reached by a certified Zap CLI/build/test invocation.

The contract does not prohibit a separately invoked reference-oracle job. It prohibits silently using that oracle to compile, build, run, or test a user project on the B4 path.

## Full-language requirement

The acceptance manifest is intentionally broader than the current seed slice. It includes representative fixtures for lexical and parser behavior, expressions and control flow, functions and closures, classes and methods, collections and maps, aliases and generics, result/option variants, modules and imports, async behavior, diagnostics, package/build metadata, VM execution, and test-runner output. A row is not complete merely because its fixture exists: the Zap-owned pipeline must produce the declared artifact and deterministic result.

Certification requires all rows to pass on every supported platform and requires two independent rebuilds from identical source and seed inputs to produce byte-identical artifacts. Any row marked `provisional` or any path that uses a Rust/Cargo fallback leaves the repository at **not-certified**.

## Acceptance commands

The repository-level integrity gate is:

```text
scripts/bootstrap/verify_b4_rust_free_contract.sh
```

The gate validates the contract, fixture manifest, ownership declarations, forbidden fallback policy, and evidence schema. It intentionally reports `not-certified` until the full source-to-VM and self-rebuild acceptance implementation exists; this prevents a subset implementation from being advertised as B4.

## Current status

Zap has a Rust-free seed pipeline and several Zap-owned compiler candidates, but the full-language self-hosting path is not yet certified. The next promotion gate is to replace the current candidate seed entrypoint with a complete Zap compiler driver and make every acceptance row executable through that driver.

## References

[1]: ../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md
[2]: ../bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml
[3]: ../bootstrap/contracts/B4_ACCEPTANCE.tsv
