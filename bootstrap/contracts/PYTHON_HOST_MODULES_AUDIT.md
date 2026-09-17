# Python Host Modules Audit

## Purpose

This document audits Python host modules to distinguish production logic from proof-only logic for B4 certification ownership migration.

## Module Classification

### Production Logic (Requires Migration to Zap-Owned)

These modules contain production logic that is currently used in the C backend seed generation pipeline and must be migrated to Zap-owned bootstrap modules:

#### 1. `host/zap-bootstrap/compile.py` - Seed Compiler
- **Purpose:** Minimal non-Rust Zap bootstrap compiler
- **Production Role:** Compiles Zap source to bytecode for C backend
- **Status:** CURRENTLY USED IN PRODUCTION (C backend path)
- **Migration Target:** `bootstrap/b1/parser.zp` + `bootstrap/b2/typecheck.zp` + `bootstrap/b3/lower.zp`
- **Key Functions:**
  - Tokenizer (`_tokenize_expr`)
  - Expression parser (`_ExprParser`)
  - Statement parser (`_parse_stmt`)
  - Bytecode lowering (`_lower`, `_compile_expr`)
  - Short-circuit evaluation (and/or operators)
  - List/map/collection operations
  - Function calls and control flow

#### 2. `host/zap-bootstrap/c_backend.py` - C Backend
- **Purpose:** Emits self-contained C from Zap bytecode
- **Production Role:** Generates native binaries without Rust
- **Status:** CURRENTLY USED IN PRODUCTION (seed generation)
- **Migration Target:** New Zap-owned C backend module (`bootstrap/b4/c_backend.zp`)
- **Key Functions:**
  - C code emission (`emit_c`)
  - Runtime data structures (tagged values, lists, maps)
  - Builtin implementations (`_BUILTIN_SPECS`)
  - Native compilation (gcc/clang/MSVC integration)
  - Bytecode-to-C instruction mapping

### Proof-Only Logic (Reference Implementation)

These modules are reference implementations used for verification and differential testing. They do not need to be migrated to Zap-owned modules:

#### 3. `host/zap-vm-host/run.py` - VM Host
- **Purpose:** Non-Rust Zap VM host (reference implementation)
- **Role:** Executes bytecode for proof-of-concept and differential testing
- **Status:** PROOF-ONLY (reference oracle)
- **Migration:** NOT REQUIRED (remains as reference)
- **Key Functions:**
  - Bytecode interpretation (`_step`, `run`)
  - Object model and MRO
  - Method dispatch and trait resolution
  - Exception handling

#### 4. `host/zap-lexer-host/lexer.py` - Lexer Host
- **Purpose:** Python lexer for differential testing
- **Role:** Reference lexer for verifying Zap-owned lexer
- **Status:** PROOF-ONLY (reference oracle)
- **Migration:** NOT REQUIRED (remains as reference)
- **Key Functions:**
  - Tokenization for verification
  - Golden file generation

#### 5. `host/zap-parser-host/parser.py` - Parser Host
- **Purpose:** Python parser for differential testing
- **Role:** Reference parser for verifying Zap-owned parser
- **Status:** PROOF-ONLY (reference oracle)
- **Migration:** NOT REQUIRED (remains as reference)
- **Key Functions:**
  - AST generation for verification
  - Diagnostic parity testing

### Verification Scripts (Not Production Logic)

These scripts are test infrastructure and do not contain production logic:

- `host/zap-bootstrap/verify.py` - C backend verification
- `host/zap-bootstrap/verify_b4_c_backend_acceptance.py` - Acceptance testing
- `host/zap-bootstrap/verify_b4_c_backend_cross_platform.py` - Cross-platform comparison
- `host/zap-bootstrap/verify_c_backend.py` - C backend regression testing
- `host/zap-vm-host/verify.py` - VM host verification
- `host/zap-bootstrap/test_strings.py` - Test utilities

## Migration Strategy

### Phase 1: Audit Complete ✅
- [x] Identify production vs. proof-only modules
- [x] Document current ownership boundaries
- [x] Define migration targets

### Phase 2: Create Zap-Owned Equivalents
- [ ] Migrate `compile.py` logic to `bootstrap/b1..b3` modules
- [ ] Create Zap-owned C backend in `bootstrap/b4/c_backend.zp`
- [ ] Integrate C backend through `compiler_driver.zp`

### Phase 3: Verification
- [ ] Pass all B4-FULL rows through Zap-owned path
- [ ] Verify byte-for-byte determinism
- [ ] Cross-platform clean-environment testing

### Phase 4: Ownership Transition
- [ ] Update `bootstrap/contracts/OWNERS.tsv`
- [ ] Update B4 contract schema
- [ ] Mark production path as Zap-owned

## Current Ownership Boundaries

| Component | Current Owner | Reference Owner | Migration Target |
|-----------|---------------|-----------------|-----------------|
| Lexer | Zap (`bootstrap/b1/lexer.zp`) | Python (`host/zap-lexer-host/`) | Complete |
| Parser | Zap (`bootstrap/b1/parser.zp`) | Python (`host/zap-parser-host/`) | Complete |
| Type Checker | Zap (`bootstrap/b2/typecheck.zp`) | Rust (`native/src`) | Complete |
| Typed IR | Zap (`bootstrap/b2/typed_ir.zp`) | Rust (`native/src`) | Complete |
| Lowering | Zap (`bootstrap/b3/lower.zp`) | Rust (`native/src`) | Complete |
| VM | Zap (`bootstrap/b3/vm.zp`) | Python (`host/zap-vm-host/`) | Reference-only |
| Seed Compiler | Python (`host/zap-bootstrap/compile.py`) | Rust (`native/src`) | **PENDING** |
| C Backend | Python (`host/zap-bootstrap/c_backend.py`) | Rust (`native/src`) | **PENDING** |
| Driver | Zap (`bootstrap/b4/compiler_driver.zp`) | Rust (`native/src`) | Complete |

## Safety Considerations

### Preserved Components
- Reference implementations (VM host, lexer host, parser host) remain for differential testing
- Golden file generation continues to use Python hosts for verification
- CI gates maintain both Zap-owned and reference paths

### Migration Guarantees
- No functionality loss during migration
- Byte-for-byte determinism preserved
- Cross-platform parity maintained
- Clean-environment execution verified

## References

- Seed Generation Pipeline: `bootstrap/contracts/SEED_GENERATION_PIPELINE.md`
- B4 Contract: `bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml`
- B4 Action Plan: `docs/B4_CERTIFICATION_ACTION_PLAN.md`
