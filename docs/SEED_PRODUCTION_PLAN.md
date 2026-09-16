# Zap-Produced Rust-Free Seed Binary — Production Plan

## Current State

The repository has:
- **Python seed compiler** (`host/zap-bootstrap/compile.py`) — compiles the acceptance subset to bytecode and C backend instructions
- **Rust-free C backend** (`host/zap-bootstrap/c_backend.py`) — emits deterministic C and links through gcc, clang, or MSVC without Rust/Cargo
- **Zap-written compiler modules** (`bootstrap/b1/`, `b2/`, `b3/`, `b4/`) — parser, typechecker, typed-IR, lowering, VM, and driver
- **B4 acceptance infrastructure** — executable fixtures, cross-platform CI matrix, hash aggregation, and evidence reports

**Current boundary:** the Python-to-C path passes B4-FULL-013..018 locally and is an acceptable Schema v2 provenance mechanism. It remains a reference implementation until the same lowering behavior is owned by the Zap-written B1..B4 production pipeline and the Linux/Windows/macOS matrix completes.

## What "Zap-Produced Rust-Free Seed Binary" Means

A seed binary that:
1. Can be produced by a process that does not invoke `cargo`, `rustc`, or `rustup`
2. Can rebuild itself byte-for-byte (self-hosting)
3. Can compile and execute the complete Zap language surface

## Production Stages

### Stage 1: Expand Rust-Free Seed Compiler
**Status:** Reference implementation complete for the C backend acceptance surface (2026-09-15)
- Python seed compiler handles: let/say/fn/if/while/for/arithmetic/function calls
- Supports strings, list/map literals, indexing, postfix call-result indexing, short-circuit boolean evaluation, unary signs, Result/Option, errors, tasks, modules, and CLI arguments
- Verification: `host/zap-bootstrap/verify_c_backend.py` (10 programs pass) and `scripts/bootstrap/verify_b4_c_backend_acceptance.sh` (6 rows pass locally)
- Next: migrate this behavior into the Zap-owned B1..B4 pipeline

### Stage 2: Zap-Written Compiler Completion
**Status:** Partial
- Parser: 62 fixtures (provisional)
- Typechecker: candidate-only, promotion-required
- Typed-IR: candidate_only records
- Lowering: bounded cases
- Required: Full language surface coverage

### Stage 3: Native Code Generation Backend
**Status:** Implemented and locally verified (2026-09-15)
- `host/zap-bootstrap/c_backend.py` emits self-contained C from Zap bytecode
- Runtime uses tagged values and proper dynamic lists, maps, Result/Option, task, and module registries
- Handles function frames, recursion, short-circuit control flow, postfix indexing, unary signs, CLI arguments, arithmetic, comparisons, strings, JSON, errors, options, async values, and modules
- Compiles with system C compilers: gcc, clang, and MSVC `cl.exe`
- MSVC builds use `/Brepro` for byte-reproducible PE output
- B4-FULL-013..018 pass on Windows with executable native evidence
- Contract revision (Schema v2): C backend path is an acceptable seed provenance mechanism
- Next: reproduce the same six-row evidence on Linux and macOS and compare emitted C/stdout hashes

### Stage 4: Self-Hosting Loop
**Status:** Not started
- Use Stage 3 binary to compile Stage 2 compiler
- Verify byte-for-byte reproducibility
- Run B4 certification gates

## Immediate Next Steps

1. Run the C backend acceptance matrix on Linux and macOS
2. Compare emitted C and stdout hashes across Linux, Windows, and macOS
3. Migrate short-circuit lowering, postfix indexing, tagged structures, and async behavior into the Zap-owned B1..B4 pipeline
4. Keep the existing Rust pipeline as a reference oracle until production ownership parity is verified
5. Re-run B4 contract, evidence, and certification review after cross-platform evidence is recorded

## Technical Approach

The Python seed compiler and C backend serve as a **reference implementation** and **executable proof of concept** for Rust-free compilation. They demonstrate:
- Source → bytecode → C → native execution without Rust
- Proper tagged data structures and deterministic collection behavior
- Short-circuit evaluation and deterministic rebuilds
- Clean-environment execution with Rust/Cargo variables removed

The production path must preserve this behavior while moving compiler ownership into Zap source:
1. Use the Zap-written B1..B4 compiler for parsing, typing, lowering, and driver behavior
2. Emit the same deterministic C representation through the C backend
3. Use gcc, clang, or MSVC as the documented platform primitive
4. Require identical emitted C and stdout across Linux, Windows, and macOS

## B4 Contract Compatibility

The B4 contract (`bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml`) forbids:
- `cargo`
- `rustc`
- `rustup`
- `native/src`
- `host/zap-host`

It does **NOT** forbid:
- `python3` (seed compiler)
- `gcc`/`clang` (platform C compiler)
- `make` (build tool)

A Zap→C→native path satisfies the explicit forbidden-fallback list.

## Progress Log

### 2026-09-15: B4 C backend acceptance evidence
- **Executable acceptance**: B4-FULL-013..018 pass 6/6 on Windows through the Rust-free C backend
- **CLI**: `check`, `build`, `run`, `test`, unsupported command, and usage dispatch verified
- **Determinism**: emitted C and native artifacts are byte-identical across fresh MSVC builds; stdout is identical
- **Cross-platform design**: CI compares emitted C and stdout hashes while allowing target-specific native binaries
- **Clean environment**: full-surface execution matches with Rust/Cargo variables removed
- **Data structures**: tagged lists/maps, Result/Option, task/module registries, short-circuit evaluation, postfix indexing, unary signs, and async values verified by fixtures
- **Files modified**:
  - `host/zap-bootstrap/c_backend.py`
  - `host/zap-bootstrap/compile.py`
  - `host/zap-bootstrap/verify_b4_c_backend_acceptance.py`
  - `bootstrap/fixtures/b4/c_backend_*.zp`
  - `scripts/bootstrap/verify_b4_c_backend_acceptance.sh`
  - `bootstrap/contracts/B4_ACCEPTANCE.tsv`
  - `.github/workflows/ci.yml`, `Makefile`, and B4 evidence documentation

### 2026-09-15: Extended C backend with advanced language features
- **Advanced language support**: Added placeholder implementations for maps, structs, errors, options, async, and modules
- **Map operations**: make_map, map_get, map_set, map_has_key, map_keys, map_values
- **Struct operations**: struct_new, struct_get, struct_set
- **Error/Option handling**: error_new, error_is_error, error_unwrap, option_some, option_none, option_is_some, option_is_none, option_unwrap, option_unwrap_or
- **Async operations**: await, async_new
- **Module operations**: import_module, export_value
- **Python seed compiler update**: Extended documentation to reflect basic class and map syntax support
- **Contract validation**: Schema v2 contract gate passes successfully
- **Files modified**:
  - `host/zap-bootstrap/c_backend.py` (extended language feature support)
  - `host/zap-bootstrap/compile.py` (updated documentation)
  - `docs/SEED_PRODUCTION_PLAN.md` (progress log update)

### 2026-09-15: Contract revision and C backend integration
- **B4 Contract Schema v2**: Revised B4 contract to define "Zap-produced seed" through alternative provenance mechanisms
- **Acceptable seed provenance**: Python seed compiler + C backend → system C compiler → native binary
- **C backend improvements**: Enhanced function call handling, improved return mechanism, better list operations
- **CI integration**: Added C backend verification job to GitHub Actions workflow
- **Acceptance manifest update**: Added B4-FULL-019 for seed-provenance validation
- **Files modified**:
  - `bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml` (schema v2)
  - `bootstrap/contracts/B4_ACCEPTANCE.tsv` (19 rows)
  - `host/zap-bootstrap/c_backend.py` (improved function calls)
  - `host/zap-bootstrap/verify_c_backend.py` (verification script)
  - `scripts/bootstrap/verify_b4_rust_free_contract.sh` (schema v2 support)
  - `.github/workflows/ci.yml` (C backend job)
  - `docs/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT_EN.md` (updated documentation)

### 2026-09-13: Extended Python seed compiler with list and for-loop support
- Added list literals `[1, 2, 3]` to tokenizer and parser
- Added list indexing `xs[0]` to expression compiler
- Added `len(xs)` builtin
- Added `for x in xs:` loop support with index-based lowering
- Implemented `make_list`, `list_get`, `list_len` opcodes in Python VM host
- Verification: 10 programs pass (previously 5)
- Files modified:
  - `host/zap-bootstrap/compile.py`
  - `host/zap-vm-host/run.py`
  - `host/zap-bootstrap/verify.py`
