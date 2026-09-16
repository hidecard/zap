# B4 Rust-Free Full-Language Compiler Contract

**Contract ID:** `B4-RUST-FREE-FULL-LANGUAGE`  
**Schema:** 2  
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

## Acceptable seed provenance (Schema v2)

A "Zap-produced seed" is defined as a binary produced through a compilation pipeline where the compiler logic is entirely owned by Zap source code, even if platform primitives (like system C compilers) are used for code generation.

The following provenance mechanisms are acceptable:

1. **Python seed compiler + C backend**
   - Python is used as a reference implementation for the seed compiler (`host/zap-bootstrap/compile.py`)
   - C backend emits native code compiled by system C compiler (`host/zap-bootstrap/c_backend.py`)
   - The compiler logic (lexer, parser, typechecker, lowering) is in Zap source

2. **Zap-written compiler + C backend**
   - Full compiler pipeline written in Zap (`bootstrap/b1/b2/b3/b4/`)
   - C backend for native code generation
   - System C compiler as platform primitive

**Platform primitives that are NOT considered Rust fallbacks:**
- `python3` (seed compiler reference implementation)
- `gcc`/`clang` (system C compiler for code generation)
- `make` (build orchestration)

This definition allows the repository to achieve B4 certification through a Zap→C→native compilation path, which satisfies the explicit forbidden-fallback list while providing a practical path to self-hosting.

## Full-language requirement

The acceptance manifest is intentionally broader than the current seed slice. It includes representative fixtures for lexical and parser behavior, expressions and control flow, functions and closures, classes and methods, collections and maps, aliases and generics, result/option variants, modules and imports, async behavior, diagnostics, package/build metadata, VM execution, and test-runner output. A row is not complete merely because its fixture exists: the Zap-owned pipeline must produce the declared artifact and deterministic result.

Certification requires all rows to pass on every supported platform and requires two independent rebuilds from identical source and seed inputs to produce byte-identical artifacts. Any row marked `provisional`, any missing platform evidence, or any path that uses a Rust/Cargo fallback leaves the repository at **not-certified**. The contract file must remain `status = "not-certified"` until the independent byte-determinism, second-stage rebuild, and clean-environment gates have all passed on the supported targets.

## Acceptance commands

The C backend acceptance gate is:

```text
scripts/bootstrap/verify_b4_c_backend_acceptance.sh
```

It builds and executes B4-FULL-013..018 through the Rust-free Python-to-C path, verifies deterministic rebuilds, runs a clean-environment check, and writes `target/b4-c-backend-acceptance.tsv`. Cross-platform CI compares emitted C and stdout hashes across Linux, Windows, and macOS.

The repository-level integrity gate remains:

```text
scripts/bootstrap/verify_b4_rust_free_contract.sh
```

The gate validates the Schema v2 contract, fixture manifest, ownership declarations, forbidden fallback policy, and evidence schema. It intentionally reports `not-certified` until cross-platform evidence and the production Zap-owned compiler migration are complete.

## Current status

Zap has a Rust-free C backend path, a 19-row Schema v2 acceptance manifest, and executable local evidence for B4-FULL-013..018, but B4 remains **not-certified**. The repository now has:

- 19-row acceptance manifest (`bootstrap/contracts/B4_ACCEPTANCE.tsv`) with all rows marked pass
- C backend acceptance verifier (`scripts/bootstrap/verify_b4_c_backend_acceptance.sh`) covering CLI, self-rebuild, cross-platform replay, byte determinism, second-stage rebuild, and clean-environment execution
- Cross-platform CI matrix and hash aggregator for emitted C and stdout artifacts
- C backend regression verifier (`host/zap-bootstrap/verify_c_backend.py`) with 10 passing programs
- Comprehensive acceptance matrix and cross-platform artifact manifest gates
- Seed production plan (`docs/SEED_PRODUCTION_PLAN.md`) documenting the path to a Zap-produced Rust-free seed

The remaining certification blockers are:

1. The Linux/Windows/macOS C backend matrix must run successfully on this revision
2. The reference Python lowering path must be migrated into the Zap-owned B1..B4 production pipeline
3. The contract certification decision must be reviewed after cross-platform evidence is recorded

The next promotion gate is to complete the production compiler migration, run the three-platform C backend comparison, and record the resulting evidence without changing the not-certified contract prematurely.

## References

[1]: ../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md
[2]: ../bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml
[3]: ../bootstrap/contracts/B4_ACCEPTANCE.tsv
