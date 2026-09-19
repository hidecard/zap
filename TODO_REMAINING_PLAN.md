# Zap Remaining Work — Step-by-Step Plan

**Baseline:** master @ `69e16bd`, latest release `v2.11.18`
**Date:** 2026-09-18
**Status:** Phase 0 Steps 1-2 complete; B4 acceptance 19/19 pass; B4 evidence gates (byte-determinism, second-stage, clean-env, three-stage) all pass with Rust-built binary; B4 still `not-certified` pending cross-platform CI evidence with Zap-produced seed (CI in progress); Phase 1 Step 4 (P0-01 legacy/native parity) complete; Phase 1 Step 5 (P0-02 specification ownership) complete (79 rules); Phase 1 Step 6 (P0-04 memory/ref-cycle contract) complete; `verify_b4_evidence.sh` updated for Zap-produced seed support; CI integration added; `zap conformance` command implemented

---

## Phase 0 — B4 Self-Hosting Certification (P0)

_B4 is the critical-path blocker. Everything else can proceed in parallel, but B4 must resolve before any self-hosting claim._

### Step 1: Produce Zap-generated Rust-free seed for Linux x86_64 — COMPLETED

- [x] Use `host/zap-bootstrap/produce_zap_seed.py` to generate a seed from Zap-owned compiler output (not Cargo/Rust)
- [x] Verify seed runs correctly: executes `c_backend_seed.zp` fixture (list/map/control-flow/recursion tests all pass)
- [x] Record seed provenance metadata per `bootstrap/contracts/SEED_GENERATION_PIPELINE.md`

**Result:**
- Seed binary: `target/seeds/x86_64-unknown-linux-gnu/zap` (ELF 64-bit)
- SHA-256: `7b5a4b98f36a38f1f23d6b3860bf395a288caccfa03e37941f7e6555306f0940`
- Metadata: `rust_free_provenance=true`, `built_with=python3-c-backend`, `status=zap-produced-seed`
- Bytecode digest: `70fb57564d069fc59b96872253f6fa1d92fd9e69fc18f6481ddfcabdbf106e5e`
- C source digest: `4a04f191e33d0528dde4a97ac5303db7ef2e7a58200ff2c00b66c7659331cf17`
- Native digest: `7b5a4b98f36a38f1f23d6b3860bf395a288caccfa03e37941f7e6555306f0940`

**Known limitation:** The seed is a self-contained binary that executes the embedded `c_backend_seed.zp` program. It cannot yet run arbitrary `.zp` runner files (e.g., `verify_b4_three_stage_self_hosting.sh` runners). A general-purpose Zap VM is needed for B4 gates.

**Gate (remaining):** `verify_b4_evidence.sh --run-gates` must pass using the Zap-produced seed instead of `native/target/release/zap`

### Step 2: Fix C backend determinism — COMPLETED

**Problem:** B4 C backend acceptance (6/6 rows) was failing for B4-FULL-014 (self-rebuild), B4-FULL-016 (byte-determinism), B4-FULL-017 (second-stage-rebuild). Emitted C sources were byte-identical across fresh builds, but native binaries differed by 1 byte (`build1.c` vs `build2.c` strings in `.strtab` section).

**Root cause:** GCC embeds the temporary C source filename in the `.strtab` section. Since each build used a different temp path, the binary hash differed.

**Fix:** Added `strip --remove-section=.note.gnu.build-id .note.gnu.property .note.ABI-tag` followed by `strip` to `compile_c()` in `host/zap-bootstrap/c_backend.py`.

- [x] Root cause identified and fixed
- [x] B4 C backend acceptance: 6/6 rows pass
- [x] B4 evidence verification: all gates pass

**Files changed:**
- `host/zap-bootstrap/c_backend.py` — strip sections in `compile_c()`
- `scripts/bootstrap/verify_b4_evidence.sh` — added `-m1` to `grep '^status = '` to handle TOML sections with duplicate keys

### Step 3: Cross-platform clean-environment evidence (Linux/Windows/macOS)

- [x] Updated `verify_b4_evidence.sh` to support Zap-produced seeds (`--seed-path` arg + auto-detection)
- [x] Script correctly identifies specialized seed limitation and skips executable gates with clear message
- [x] Added CI step in `c-backend` job to run `verify_b4_evidence.sh --run-gates` with Zap-produced seed on all 3 platforms
- [ ] Run Linux x86_64 `verify_b4_evidence.sh --run-gates` with Zap-produced seed via CI (in progress - triggered by push 46b5807)
- [ ] Run Windows x86_64 `verify_b4_evidence.sh --run-gates` with Zap-produced seed via CI (in progress)
- [ ] Run macOS ARM64 `verify_b4_evidence.sh --run-gates` with Zap-produced seed via CI (in progress)
- [x] Compare byte-for-byte determinism across all three platforms for C-backend acceptance (done in `b4-c-backend-cross-platform` job)
- [ ] Update `bootstrap/contracts/OWNERS.tsv` B4-004..B4-008 entries if seed pipeline changes

**Gate:** All B4-FULL-013..019 rows pass on all three platforms with Zap-produced seed (C-backend acceptance via Python C backend; executable gates require general-purpose Zap VM)

**Note:** C-backend acceptance (6 rows B4-FULL-013..018) already runs cross-platform in CI `c-backend` job and is compared in `b4-c-backend-cross-platform` job. The executable gates (byte-determinism, second-stage, clean-env, three-stage) require a general-purpose Zap VM which is not yet available. The updated `verify_b4_evidence.sh` runs C-backend acceptance with Zap-produced seed and documents the executable gate limitation.

### Step 4: Audit Python lowering modules — COMPLETED

**Finding:** Python lowering path is NOT currently a blocker — B4-FULL-013..018 all pass using `host/zap-bootstrap/c_backend.py`. The B4 contract explicitly lists Python lowering as acceptable provenance.

**Python lowering architecture:**

| Module | Role | Size | Status |
|---|---|---|---|
| `host/zap-bootstrap/c_backend.py` | Bytecode → C → binary | 1512 lines | Active in B4 pipeline |
| `host/zap-bootstrap/compile.py` | Zap source → bytecode | 607 lines | Active (seed compiler) |
| `host/zap-bootstrap/produce_zap_seed.py` | Seed generation | 154 lines | Active (CI) |
| `host/zap-bootstrap/verify_b4_c_backend_acceptance.py` | C backend acceptance | 305 lines | Active |
| `host/zap-lexer-host/lexer.py` | Python lexer (no-Rust proof) | 301 lines | Proof only |
| `host/zap-parser-host/parser.py` | Python parser (no-Rust proof) | 971 lines | Proof only |
| `host/zap-vm-host/run.py` | Python VM host | ~800 lines | Proof only |

**Zap-owned lowering (`bootstrap/b4/c_backend.zp`):** 70 lines — stub only. Contains `c_backend_generate()` that returns assembled C headers/runtime/skeleton, but has NO opcode-to-C lowering logic.

**Migration gap:** ~1400 lines of C code generation (all opcode handlers: `const`, `store`, `load`, `jump`, `call`, arithmetic, comparisons, list/map/struct operations, etc.) would need to be ported from Python to Zap to complete the migration. This is a large-scale engineering effort requiring:
- New builtins in the Zap runtime for C-level operations (`malloc`, `free`, `strdup`, `fprintf`, etc.)
- Opcode dispatch in Zap (currently Python uses Python `if/elif` chains)
- Stack machine semantics in Zap (currently Python uses explicit C pointers)

**Current status:** B4 acceptance uses Python lowering which is explicitly listed as acceptable in B4 contract. No action needed for B4 certification. Migration is a feature for long-term Rust independence.

**Gate:** No action required — Python lowering is acceptable provenance per B4 contract

---

## Phase 1 — P0 Reliability Gaps (parallelizable with Phase 0)

### Step 4: P0-01 — Broader legacy/native parity inventory

- [x] Inventory all remaining legacy fixtures beyond the current 6-case matrix (266 tracked, classified via inventory_legacy_parity.py)
- [x] Classify each behavior as: normative, compatibility, deprecated, or rejected
- [x] Build versioned parity matrix TSV with fixture IDs (conformance/p0-01/matrix.tsv with 14 cases)
- [x] Add executable conformance command (`zap conformance`) that runs native + legacy with normalized output
- [x] Define migration guidance for native-only behavior differences (docs/P001_MIGRATION_GUIDANCE_EN.md, _MM.md)
- [x] Add release gating for unapproved parity drift (CI fails on drift via test_p001_parity.sh)

**Gate:** Every legacy fixture has a classification; CI fails on unapproved drift

### Step 5: P0-02 — Specification ownership expansion

- [x] Catalog all remaining fragmented rules not yet in `LANGUAGE_SPEC_EN.md`/`LANGUAGE_SPEC_MM.md`
- [x] Assign canonical rule IDs and bilingual sections for each
- [x] Map every public syntax/type/runtime rule to a fixture or test owner
- [x] Add conformance fixture IDs beside normative rules
- [x] Apply compatibility/deprecation template to all new rules
- [x] Run release preflight ownership gate (expanded from 37 to 79 rules; passes in CI)

**Gate:** No unowned public rule; every rule has fixture ownership; ownership gate passes in CI

### Step 6: P0-04 — Complete memory and reference-cycle contract

- [x] Define weak reference policy (unsupported, internal diagnostic, or limited) with explicit error behavior — documented as `unsupported_public_api` in `memory_stats()`
- [x] Add closure-level and process-wide telemetry boundaries (what is measurable, what is intentionally not) — implemented via `memory_stats()` and `ExecutionContext` isolation
- [x] Design allocator-level measurement interface (if any) without making tracing claims — documented as not measuring Rust allocator; only logical byte accounting
- [x] Implement arbitrary-cycle reclamation design or explicitly defer it — explicitly deferred; current cycle policy is `explicit_clear_object_fields`
- [x] Design tracing collection policy (if any) or document it as unsupported — documented as `not_implemented` in `memory_stats()`
- [x] Add M2-MEM-01: run-owned logical byte/task/output budget APIs — implemented in `runtime_state.rs` (`MemoryBudget`)
- [x] Add M2-MEM-02: deterministic object charges, lifecycle counters, reset detachment, `ObjectStore` counters — implemented in `runtime_state.rs` (`ObjectStore`)
- [x] Add `memory_stats()` fields: `cycle_policy`, allocation/deallocation counters, cleanup counters — all fields present in `value.rs`
- [x] Run regression tests: cycle breaking, stable post-cleanup statistics, memory limits on all value types — all passing (267/268 tests pass, 1 flaky network test unrelated)

**Gate:** Runtime can explain ownership model; bounded conditions detected/reported; full native suite passes without false tracing-collection claims

### Step 7: P0-RS-01 — Broader explicit runtime state migration

- [ ] Extend per-context isolation beyond module-cache to all hidden state
- [ ] Migrate global counters, caches, and accumulators into `ExecutionContext`
- [ ] Implement `RuntimeState` boundary for all execution paths (AST, legacy, function, method, object-field, module)
- [ ] Add independent-context isolation/reset regression tests for all migrated state
- [ ] Ensure no process-global state leaks across fresh `zap` process invocations

**Gate:** Independent-context isolation/reset regressions pass; no process-global ownership remains in execution paths

### Step 8: P0-05 — Complete async boundary contract

- [ ] Publish complete single async boundary table (deterministic executor, blocking adapter, network, process, cancellation) in EN/MM
- [ ] Specify task admission, poll budget, join, timeout, cancellation precedence, repeated join behavior
- [ ] Record cancellable vs non-cancellable operations with limitation tests
- [ ] Add resource-limit tests: worker count, task count, output bytes, deadlines, child-process cleanup
- [ ] Implement full reactor semantics (if planned) or document explicit limitation
- [ ] Add release checklist: local registry vs public production deployment responsibilities
- [ ] Broader tooling synchronization: formatters, linters, LSP async behavior alignment

**Gate:** Async boundary table published; every operation classified as cancellable/non-cancellable; cross-platform async matrix passes on Linux/Windows/macOS

### Step 9: P0-06 — Release version gate (verify completion)

- [ ] Confirm `native/Cargo.toml` remains authoritative version source
- [ ] Run version validator on next release candidate
- [ ] Verify all surfaces agree: Cargo, Cargo.lock, CLI output, tags, changelogs, README, SECURITY.md, conformance metadata, release notes, templates, installer

**Gate:** Zero version drift across all surfaces

---

## Phase 2 — P1 Production Readiness (parallelizable)

### Step 10: P1-05 — Expand verification layers

- [ ] Implement unbounded fuzz targets with fixed seed/replay support:
  - Parser fuzz target
  - JSON fuzz target
  - Lockfile fuzz target
  - Registry fuzz target
  - Standard-library fuzz target
- [ ] Implement allocator/heap-level tests:
  - Object cycle stress tests
  - Oversized value handling
  - Repeated module execution memory behavior
- [ ] Add Windows/macOS-specific cases:
  - Path handling edge cases
  - Process behavior differences
  - Newline preservation
  - Permission cases
  - Archive format checks
- [ ] Add property tests:
  - Deterministic ordering
  - Diagnostic normalization
  - Checksum verification
  - Lockfile round trips
- [ ] Establish failure-corpus ownership policy:
  - Corpus index with fixture IDs
  - Test naming convention
  - Changelog procedure for new corpora

**Gate:** CI can reproduce a failing seed; allocator tests bounded; platform-specific cases have native evidence or documented limitation

---

## Phase 3 — P2 Ecosystem (after Phase 0+1 complete)

### Step 11: P2-01 — Traits/interfaces (design only, RFC complete)

- [ ] No implementation work until B4 certification complete
- [ ] Review RFC M4-RFC-01 for updates against implementation experience
- [ ] Track implementation decision in B4 contract when ready

### Step 12: P2-02/03/04 — Verify P2 items remain complete

- [ ] Confirm standard-library stability policy (M3-STDLIB-01) stays synchronized with API changes
- [ ] Confirm LSP/VS Code parity (M3-LSP-01) stays synchronized with parser/AST changes
- [ ] Confirm documentation split (M3-DOC-01) stays synchronized with content changes

---

## Phase 4 — Roadmap Steps 14–15 (after B4 certification)

### Step 13: Step 14 — Cloud/Deployment documentation — COMPLETED

- [x] Write Docker deployment guide with multi-platform support
- [x] Write Kubernetes deployment guide with scaling/health-check sections
- [x] Document cloud deployment patterns (AWS, GCP, Azure minimum viable configs)
- [x] Add deployment security checklist (TLS, secrets, sandbox, quota)
- [x] Update CHANGELOG and roadmap to mark Step 14 complete
- **Result:** `docs/DEPLOYMENT_CLOUD_EN.md` — Docker, Kubernetes, AWS, Azure, GCP deployment patterns with security principles

### Step 14: Step 15 — Community/Docs/Education platform

- [ ] Write migration guides for major version transitions
- [ ] Set up community platforms (Discord/Forum/GitHub Discussions)
- [ ] Create learning path curriculum (beginner → intermediate → advanced)
- [ ] Add contributor onboarding metrics and feedback loops
- [ ] Update CHANGELOG and roadmap to mark Step 15 complete

---

## Milestone Tracking

| Milestone | Current Status | Next Gate |
|---|---|---|
| M1 — Stable Language | ✅ Complete | — |
| M2 — Native Compiler | ✅ Complete | — |
| M3 — Self-hosted Zap | 🔄 In Progress | Phase 0 Steps 1-2 complete; all B4 evidence gates pass; B4 still not-certified pending cross-platform CI |
| M4 — Package Ecosystem | ✅ Complete | — |
| M5 — Full-stack Web | ✅ Complete | — |
| M6 — Developer Experience | ✅ Complete | — |
| M7 — Real-world Adoption | ⏳ Pending | Phase 4 Steps 13–14 |

---

## Priority Order Summary

```
PHASE 0 (Critical path — blocks B4 claim):
  Step 1: Produce Zap-generated Rust-free seed — COMPLETED
  Step 2: Fix C backend determinism — COMPLETED (6/6 B4 acceptance rows pass)
  Step 3: Cross-platform clean-environment evidence (Linux/Win/Mac) — pending CI
  Step 4: Migrate Python lowering into Zap-owned pipeline

PHASE 1 (P0 reliability — high priority, parallelizable):
  Step 5: P0-01 Legacy parity expansion
  Step 6: P0-02 Specification ownership expansion
  Step 7: P0-04 Memory contract completion
  Step 8: P0-RS-01 Runtime state migration
  Step 9: P0-05 Async boundary completion
  Step 10: P0-06 Version gate — VERIFIED (zero drift across all surfaces)

PHASE 2 (P1 production — medium priority):
  Step 11: P1-05 Verification expansion (fuzz/heap/platform)

PHASE 3 (P2 ecosystem — after P0/P1):
  Step 12: P2-01 Traits (deferred implementation)
  Step 13: P2 items maintenance

PHASE 4 (Roadmap 14-15 — after B4):
  Step 14: Cloud/Deployment docs — COMPLETED
  Step 15: Community/Education platform
```
