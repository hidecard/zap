# B4 Certification Action Plan

**Created:** 2026-09-17
**Status:** B4 remains `not-certified`
**Goal:** B4-RUST-FREE-FULL-LANGUAGE contract → `certified`

---

## Current Status (2026-09-15)

### B4-FULL Rows: 19/19 Local Pass

| ID | Area | Status | Evidence |
|---|---|---|---|
| B4-FULL-001 | lexer-parser | pass | bootstrap/fixtures/b4/full_language_surface.zp → canonical_ast |
| B4-FULL-002 | expressions-control-flow | pass | typed_ir via bootstrap/b2/typecheck.zp |
| B4-FULL-003 | functions-closures | pass | typed_ir via bootstrap/b2/typed_ir.zp |
| B4-FULL-004 | classes-methods | pass | bytecode via bootstrap/b3/lower.zp |
| B4-FULL-005 | collections-maps | pass | bytecode via bootstrap/b3/lower.zp |
| B4-FULL-006 | aliases-generics | pass | diagnostics via bootstrap/b2/typecheck.zp |
| B4-FULL-007 | result-option | pass | vm_result via bootstrap/b3/vm.zp |
| B4-FULL-008 | modules-imports | pass | module_graph via bootstrap/b3/package.zp |
| B4-FULL-009 | async-runtime | pass | vm_result via bootstrap/b3/vm.zp |
| B4-FULL-010 | diagnostics | pass | stable_diagnostic via bootstrap/b2/typecheck.zp |
| B4-FULL-011 | package-build | pass | build_artifact via bootstrap/b3/package.zp |
| B4-FULL-012 | test-runner | pass | test_result via bootstrap/b4/runner.zp |
| B4-FULL-013 | cli-entrypoint | pass | cli_result via host/zap-bootstrap/c_backend.py (Windows/MSVC) |
| B4-FULL-014 | self-rebuild | pass | self_rebuild_bytes via c_backend.py |
| B4-FULL-015 | cross-platform-determinism | pass | platform_rebuild via c_backend.py |
| B4-FULL-016 | byte-determinism | pass | artifact_bytes via c_backend.py |
| B4-FULL-017 | second-stage-rebuild | pass | stage2_artifact via c_backend.py |
| B4-FULL-018 | clean-environment | pass | clean_run via c_backend.py (Windows/MSVC, Rust/Cargo vars removed) |
| B4-FULL-019 | seed-provenance | pass | c_backend_artifact (Python+C provenance accepted per Schema v2) |

### What Passes Locally
- C backend (`host/zap-bootstrap/c_backend.py`) emits self-contained C from Zap bytecode
- Python seed compiler + C compiler → native binary is accepted as "Zap-produced seed" per Schema v2
- All 18 B4-FULL rows pass via the Rust-free Python+C backend path on Windows/MSVC
- Three-stage self-hosting passes on Linux x86_64 with recorded seed SHA-256

### What Is NOT Yet Certified
- **Cross-platform clean-environment evidence:** Linux/Windows/macOS clean-environment runs with Zap-produced Rust-free seed are not all confirmed from a single shared source state
- **Zap-produced seed provenance:** The seed binary used in self-hosting tests must be demonstrably produced by a Rust-free compiler chain (not by `native/` Rust source)
- **Migration of reference Python lowering:** The Python reference lowering path must be migrated into the Zap-owned B1..B4 production pipeline (`bootstrap/b1/parser.zp`, `bootstrap/b2/typed_ir.zp`, `bootstrap/b3/lower.zp`, `bootstrap/b4/compiler_driver.zp`)

---

## Certification Blockers (in order)

### Blocker 1: Cross-Platform Clean-Environment Evidence

**What's needed:** Linux x86_64 + Windows x86_64 + macOS ARM64 clean-environment execution from a Zap-produced Rust-free seed.

**Why:** CI can run on all three platforms, but the seed used must be proven Zap-produced (not Cargo-built).

**Action items:**
- [ ] Generate Zap-produced seed binary using C backend on each target platform
- [ ] Run `verify_b4_clean_environment.sh` on each platform with the Zap-produced seed
- [ ] Record hash comparison across all three platforms
- [ ] Wire into CI matrix (`b4-platform-evidence` job)

**Blockers to investigation:**
- Seed generation pipeline needs `ZAP_BOOTSTRAP_BIN` to point to the C-backend-produced seed
- macOS ARM64 seed production requires macOS runner
- Windows x86_64 clean-environment execution requires Windows runner

### Blocker 2: Zap-Produced Seed Provenance

**What's needed:** A seed binary that was produced entirely through Zap-owned compiler logic + platform primitives (python3, gcc/clang), with no Rust/Cargo involvement in the compilation chain.

**Why:** B4 certification requires proving that the compiler can bootstrap itself without the Rust reference implementation.

**Action items:**
- [ ] Document seed generation pipeline in `bootstrap/contracts/`
- [ ] Produce initial Zap-produced seed for each platform target
- [ ] Verify seed reproducibility (two independent generations produce identical binaries)
- [ ] Record seed provenance metadata (commit, timestamp, toolchain, hash) in contract schema
- [ ] Update `OWNERS.tsv` to reflect seed ownership transfer from Rust reference to Zap-owned

### Blocker 3: Reference Lowering Migration

**What's needed:** Move Python reference lowering (`host/zap-bootstrap/c_backend.py`, `host/zap-vm-host/`) logic into Zap-owned bootstrap modules (`bootstrap/b4/compiler_driver.zp` and related).

**Why:** Current ownership boundary claims `reference_owner=rust` for backend lowering. B4 requires `reference_owner=zap`.

**Action items:**
- [ ] Audit all Python host modules for production logic vs. proof-only logic
- [ ] Create Zap-owned equivalents of:
  - [ ] `backend lowering` (currently in `host/zap-bootstrap/c_backend.py`)
  - [ ] `VM execution` (currently in `host/zap-vm-host/`)
  - [ ] `package/build` (currently in `host/zap-bootstrap/compile.py`)
- [ ] Wire Zap-owned implementations through `compiler_driver.zp`
- [ ] Pass all B4-FULL rows through Zap-owned backend path
- [ ] Update ownership records in `bootstrap/contracts/OWNERS.tsv`

### Blocker 4: Full-Language Semantic Ownership

**What's needed:** Complete language feature ownership promotion from `candidate` to `zap_owned` for all supported syntax/semantic features.

**Why:** `complete_language=false` remains because some features are still candidate-only.

**Action items:**
- [ ] Audit `ownership=candidate` records in typed-IR corpus
- [ ] Create tests for each candidate-only feature through Zap-owned pipeline
- [ ] Promote each to `ownership=zap_owned` after passing
- [ ] Verify `complete_language=true` at the contract level

### Blocker 5: Deterministic Self-Recompile Chain

**What's needed:** Three-stage self-recompile chain where each stage produces byte-identical output using a completely Rust-free toolchain at each step.

**Why:** Currently the first stage uses a Cargo-built seed; subsequent stages can use the Zap-produced seed from stage 1.

**Action items:**
- [ ] Produce initial Zap-free seed (Blocker 2)
- [ ] Run stage 1: Zap source → Zap compiler (using free seed) → Zap bytecode
- [ ] Run stage 2: Zap bytecode → C backend → native binary → second Zap compiler
- [ ] Compare stage 1 and stage 2 outputs byte-for-byte
- [ ] Verify no Cargo/Rust variables present in any stage

---

## CI/CD Actions Required

### Immediate
- [ ] Add `b4-platform-evidence` CI job (partially done — needs Zap-produced seed, not Cargo-built)
- [ ] Create `verify_b4_full_acceptance_matrix.sh` (implemented — 19 rows, 18 pass, 0 provisional)
- [ ] Create `verify_b4_cross_platform_artifact_manifest.sh` (implemented — per-platform manifests)
- [ ] Add `cargo-audit` to release preflight (done — 0 advisories on 87 crates)

### Future
- [ ] Add Zap-produced seed generation job (depends on Blocker 2)
- [ ] Add cross-platform clean-environment verification job (depends on Blocker 1)
- [ ] Add backend ownership verification job (depends on Blocker 3)
- [ ] Add full-language completeness verification job (depends on Blocker 4)

---

## B4 Certification Criteria (definition of done)

B4 will be `certified` when ALL of the following are true:

1. **All 19 B4-FULL rows** pass on a current `master` commit
2. **Cross-platform evidence** exists for Linux x86_64, Windows x86_64, and macOS ARM64
3. **Zap-produced seed** exists for each target platform (proven through Rust-free chain)
4. **Self-recompile** produces byte-identical output in fresh processes
5. **No Rust/Cargo fallbacks** in any verification gate
6. **Full-language ownership** promoted (`complete_language=true`, `reference_owner=zap`)
7. **Independent verifier** (`verify_b4_evidence.sh --run-gates`) passes on clean checkout
8. **Documentation** updated to reflect certified status in README.md, CHANGELOG.md, TODO.md

---

## Reference Documents

- [B4 Contract (Schema v2)](../../bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml)
- [B4 Acceptance Manifest](../../bootstrap/contracts/B4_ACCEPTANCE.tsv)
- [Bootstrap Contract EN](../../bootstrap/contracts/BOOTSTRAP_CONTRACT_EN.md)
- [Certification Evidence](../../bootstrap/evidence/b4/certification_evidence.md)
- [B4 Contract Revision v2 EN](../../docs/B4_CONTRACT_REVISION_V2_EN.md)
- [Seed Production Plan](../../docs/SEED_PRODUCTION_PLAN.md)
- [TODO.md](../../TODO.md) — B4 Certification Status section
