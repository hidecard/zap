# Zap-Produced Rust-Free Seed Binary — Production Plan

## Current State

The repository has:
- **Python seed compiler** (`host/zap-bootstrap/compile.py`) — compiles a bounded subset of Zap to JSON bytecode, executed by `host/zap-vm-host/run.py`
- **Zap-written compiler modules** (`bootstrap/b1/`, `b2/`, `b3/`, `b4/`) — parser, typechecker, typed-IR, lowering, VM, driver
- **B4 verification infrastructure** — 46+ scripts, CI jobs, evidence collection

**Current blocker:** The native binary `native/target/release/zap` is produced by `cargo build` from `native/src/*.rs`. There is no mechanism to produce this binary without Rust/Cargo in the compiler path.

## What "Zap-Produced Rust-Free Seed Binary" Means

A seed binary that:
1. Can be produced by a process that does not invoke `cargo`, `rustc`, or `rustup`
2. Can rebuild itself byte-for-byte (self-hosting)
3. Can compile and execute the complete Zap language surface

## Production Stages

### Stage 1: Expand Rust-Free Seed Compiler (In Progress)
**Status:** Extended with list and for-loop support (2026-09-13)
- Python seed compiler handles: let/say/fn/if/while/for/arithmetic/function calls
- **NEW:** list literals `[1, 2, 3]`, list indexing `xs[0]`, `len(xs)`
- **NEW:** `for x in xs:` loops with index-based lowering
- Verification: `verify_non_rust_bootstrap_compiler.sh` (10 programs pass)
- Next: strings, basic data structures

### Stage 2: Zap-Written Compiler Completion
**Status:** Partial
- Parser: 62 fixtures (provisional)
- Typechecker: candidate-only, promotion-required
- Typed-IR: candidate_only records
- Lowering: bounded cases
- Required: Full language surface coverage

### Stage 3: Native Code Generation Backend
**Status:** Not started
- Implement a code generator that emits C or LLVM-IR from Zap bytecode/IR
- Use system C compiler as platform seed boundary (allowed by B4 contract)
- Output: native executable (ELF/PE/Mach-O)

### Stage 4: Self-Hosting Loop
**Status:** Not started
- Use Stage 3 binary to compile Stage 2 compiler
- Verify byte-for-byte reproducibility
- Run B4 certification gates

## Immediate Next Steps

1. **Extend Python seed compiler** with additional language features (for loops, string operations)
2. **Document exact bytecode format** for new operations
3. **Add C emission backend** to Python seed compiler (proof of concept for native binary)
4. **Update verification** to test new features
5. **Create roadmap** for Stage 2 (Zap compiler completion)

## Technical Approach

The Python seed compiler serves as a **reference implementation** and **proof of concept** for Rust-free compilation. It demonstrates:
- Source → AST → bytecode → execution without Rust
- Deterministic output
- Self-contained execution

To produce a native binary without Rust/Cargo:
1. Complete the Zap-written compiler
2. Add a C emission backend to the lowering phase
3. Use system C compiler (gcc/clang) as the platform primitive
4. The resulting binary is "Zap-produced" because the compiler logic is entirely in Zap

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
