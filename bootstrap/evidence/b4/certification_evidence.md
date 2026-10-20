# B4 Rust-Free Full-Language Certification Evidence

## Evidence Snapshot Date
2026-10-20

## Contract Status
- **Current:** `not-certified`
- **Reason:** Current-master CI passes and bounded Zap-owned backend/runtime slices have expanded substantially, but the available seed is still explicitly `certification_ready=false`; the full Zap-owned B1..B4 production path, the data-structure driver overflow fix, and clean multi-platform seed provenance are not complete.

## Acceptance Manifest Declarations (19 rows marked pass; not certification evidence)

The manifest's `pass` labels record declared component-level results. They do not establish that all 19 acceptance behaviors pass end-to-end through a complete Zap-owned compiler path.

| ID | Area | Fixture | Owner | Artifact | Manifest status |
|----|------|---------|-------|----------|-----------------|
| B4-FULL-001 | lexer-parser | `bootstrap/fixtures/b4/full_language_surface.zp` | `bootstrap/b1/parser.zp` | canonical_ast | pass |
| B4-FULL-002 | expressions-control-flow | `bootstrap/fixtures/typecheck/basic_type_matrix.zp` | `bootstrap/b2/typecheck.zp` | typed_ir | pass |
| B4-FULL-003 | functions-closures | `bootstrap/fixtures/typecheck/function.zp` | `bootstrap/b2/typed_ir.zp` | typed_ir | pass |
| B4-FULL-004 | classes-methods | `bootstrap/fixtures/typecheck/generic_class.zp` | `bootstrap/b3/lower.zp` | bytecode | pass |
| B4-FULL-005 | collections-maps | `bootstrap/fixtures/typecheck/collection_expression_map.zp` | `bootstrap/b3/lower.zp` | bytecode | pass |
| B4-FULL-006 | aliases-generics | `bootstrap/fixtures/typecheck/generic_compound_bounds.zp` | `bootstrap/b2/typecheck.zp` | diagnostics | pass |
| B4-FULL-007 | result-option | `bootstrap/fixtures/typecheck/expression_result_constructor.zp` | `bootstrap/b3/vm.zp` | vm_result | pass |
| B4-FULL-008 | modules-imports | `bootstrap/fixtures/typecheck/generic_cross_module.zp` | `bootstrap/b3/package.zp` | module_graph | pass |
| B4-FULL-009 | async-runtime | `bootstrap/fixtures/typecheck/flow_engine.zp` | `bootstrap/b3/vm.zp` | vm_result | pass |
| B4-FULL-010 | diagnostics | `bootstrap/fixtures/typecheck/function_incompatible.zp` | `bootstrap/b2/typecheck.zp` | stable_diagnostic | pass |
| B4-FULL-011 | package-build | `bootstrap/b3/package.zp` | `bootstrap/b3/package.zp` | build_artifact | pass |
| B4-FULL-012 | test-runner | `bootstrap/b4/runner.zp` | `bootstrap/b4/runner.zp` | test_result | pass |
| B4-FULL-013 | cli-entrypoint | `bootstrap/fixtures/b4/c_backend_cli.zp` | `host/zap-bootstrap/c_backend.py` | cli_result | pass |
| B4-FULL-014 | self-rebuild | `bootstrap/fixtures/b4/c_backend_self_rebuild.zp` | `host/zap-bootstrap/c_backend.py` | self_rebuild_bytes | pass |
| B4-FULL-015 | cross-platform-determinism | `bootstrap/fixtures/b4/c_backend_full_surface.zp` | `host/zap-bootstrap/c_backend.py` | platform_rebuild | pass |
| B4-FULL-016 | byte-determinism | `bootstrap/fixtures/b4/c_backend_seed.zp` | `host/zap-bootstrap/c_backend.py` | artifact_bytes | pass |
| B4-FULL-017 | second-stage-rebuild | `bootstrap/fixtures/b4/c_backend_self_rebuild.zp` | `host/zap-bootstrap/c_backend.py` | stage2_artifact | pass |
| B4-FULL-018 | clean-environment | `bootstrap/fixtures/b4/c_backend_full_surface.zp` | `host/zap-bootstrap/c_backend.py` | clean_run | pass |
| B4-FULL-019 | seed-provenance | `host/zap-bootstrap/c_backend.py` | `host/zap-bootstrap/c_backend.py` | c_backend_artifact | pass |

## C Backend Acceptance Evidence

`scripts/bootstrap/verify_b4_c_backend_acceptance.sh` still compiles the acceptance fixtures through the reference `host/zap-bootstrap/compile.py` and `host/zap-bootstrap/c_backend.py`, links them with the system C compiler, executes the native binaries, and records SHA-256 digests. The newer Zap-owned backend/runtime slices are covered by their separate ownership and migration gates; they do not yet replace this full acceptance harness.

| ID | Verified behavior | Local result |
|----|-------------------|--------------|
| B4-FULL-013 | CLI `check`, `build`, `run`, `test`, unsupported command, and usage dispatch | pass on Windows x86_64 |
| B4-FULL-014 | Two fresh source-to-C-to-native builds produce byte-identical C and PE binaries with identical stdout | pass on Windows x86_64 |
| B4-FULL-015 | Two fresh full-surface builds produce byte-identical emitted C and identical stdout; native hashes remain platform-specific | pass on Windows x86_64 |
| B4-FULL-016 | Two fresh seed builds produce byte-identical C and PE binaries with identical stdout | pass on Windows x86_64 |
| B4-FULL-017 | Two independent second-stage rebuilds produce identical C, PE, and execution artifacts | pass on Windows x86_64 |
| B4-FULL-018 | Full-surface execution with Rust/Cargo environment variables removed matches normal execution | pass on Windows x86_64 |

Additional verified fixtures:

| Fixture | Description |
|---------|-------------|
| `bootstrap/fixtures/b4/c_backend_datastructures.zp` | List/map/control-flow/recursion exercises via C backend |

The verifier report is written to `target/b4-c-backend-acceptance.tsv`. Cross-platform CI compares emitted C and stdout hashes across Linux, Windows, and macOS while allowing native executable hashes to differ by target.

## Verified Gates (latest hosted evidence)

| Gate | Result | Notes |
|------|--------|-------|
| `python host/zap-bootstrap/verify_c_backend.py` | historical pass, not rerun | 10 representative C backend programs |
| `scripts/bootstrap/verify_b4_c_backend_acceptance.sh` | passed locally | B4-FULL-013..018 passed 6/6 on Windows x86_64 on 2026-09-27 |
| `python host/zap-bootstrap/test_b4_c_backend_cross_platform.py` | passed locally | 7/7 aggregation cases, including rejection of missing platforms, incomplete rows, failed rows, invalid hashes, and conflicting reports |
| `scripts/bootstrap/verify_b4_c_backend_cross_platform.sh` | hosted CI pass | Current-master CI run `39204640666` completed successfully; local sandbox cannot execute every foreign native target |
| `scripts/bootstrap/verify_b4_rust_free_contract.sh` | passed locally | Schema v2 structure and 19-row manifest validation only |
| `scripts/bootstrap/verify_b4_evidence.sh` | historical pass, not rerun | Schema v2 evidence package structure |
| `scripts/bootstrap/verify_b4_seed_preflight_10.sh` | historical pass, not rerun | Rust-free seed preflight |
| `make bootstrap-non-rust-test` | historical pass, not rerun | Compiler and VM host run without Rust toolchain |
| `make bootstrap-three-stage-test` | historical pass, not rerun | Three-stage self-hosting gate with `bin/zap` |
| `make bootstrap-self-rebuild-test` | historical pass, not rerun | Byte-determinism, second-stage, three-stage, clean-env gates |

## Platform Seed Record

A local prebuilt Windows x86_64 seed record exists at:
- `bootstrap/fixtures/metadata/platform_seed_windows_x86_64.json`

```json
{
  "platform": "windows-x86_64",
  "binary": "bin/zap.exe",
  "sha256": "7B8476263F98A419C48D4F11B99658FF2A29EF13CEBCE2D1F3239ED8F7EC4756",
  "size_bytes": 8172032,
  "built_with": "rust/cargo",
  "status": "prebuilt-native-seed",
  "notes": "Legacy Rust seed record retained for release provenance. B4-FULL-013..018 executable evidence now uses the Rust-free Python-to-C backend path."
}
```

## Evidence Artifacts

- C backend regression report: `target/c-backend-verification.tsv`
- B4 C backend acceptance report: `target/b4-c-backend-acceptance.tsv`
- Cross-platform C backend comparison: `target/b4-c-backend-cross-platform.tsv`
- B4 evidence report: `target/b4-evidence-report.tsv`
- Rebuild artifacts: `target/b4-rebuild-*`
- Platform provenance: `target/b4-platform-*`
- Byte-determinism records: `target/b4-byte-*`
- Clean-environment records: `target/b4-clean-environment.tsv`

## CI Infrastructure

| Infrastructure | Status | Purpose |
|----------------|--------|---------|
| `.github/workflows/ci.yml` current-master workflow | passed in run `39204640666` | Latest master commit `5c136b4` passed the hosted validation workflow |
| B4 C-backend/platform evidence | historical hosted pass retained | Cross-platform acceptance evidence remains recorded, but it uses the reference acceptance harness and does not establish full Zap-owned production ownership |
| `scripts/bootstrap/verify_b4_c_backend_acceptance.sh` | present | Portable entrypoint for B4-FULL-013..018 |
| `host/zap-bootstrap/verify_b4_c_backend_acceptance.py` | present | Builds, executes, hashes, and reports C backend artifacts |
| `Makefile` `bootstrap-b4-c-backend-test` | present | Runs the C backend acceptance gate |

## Certification Blockers

| Blocker | Current evidence |
|---------|------------------|
| Cross-platform C backend evidence | Hosted cross-platform evidence remains passing; the latest current-master workflow is run `39204640666`. |
| Production compiler ownership | Bounded Zap-owned runtime/lowering slices now cover numeric/generic collections, strings, options/errors, locals, function frames, and closure captures, but full acceptance still uses the Python reference path and the Zap-owned driver has a known data-structure stack-overflow regression. |
| Clean multi-platform seed provenance | The CI platform seed metadata remains `built_with=cargo-rust`, `rust_free_provenance=false`, and `certification_ready=false`; a complete Zap-produced three-platform seed set is still absent. |
| Contract certification decision | Remains `not-certified` until the above evidence and ownership gates pass. |

## Certification Decision

The repository is **not yet certified** for the B4 Rust-Free Full-Language Compiler Contract. Current-master CI and multiple bounded Zap-owned migration slices pass, but certification still requires the full Zap-owned production path, the data-structure driver overflow fix, complete acceptance through that path, and clean multi-platform seed provenance with `certification_ready=true`.
