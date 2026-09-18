# B4 Rust-Free Full-Language Certification Evidence

## Evidence Snapshot Date
2026-09-18

## Contract Status
- **Current:** `not-certified`
- **Reason:** the C backend now passes all six executable B4-FULL-013..018 checks on Windows, but the three-platform CI comparison has not yet run for this revision. The contract also remains a candidate boundary while the Python seed compiler is a reference implementation rather than the production Zap-owned compiler.

## Acceptance Rows (19/19 PASS)

| ID | Area | Fixture | Owner | Artifact | Status |
|----|------|---------|-------|----------|--------|
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

`scripts/bootstrap/verify_b4_c_backend_acceptance.sh` compiles Zap fixtures through `host/zap-bootstrap/compile.py` and `host/zap-bootstrap/c_backend.py`, links them with the system C compiler, executes the native binaries, and records SHA-256 digests.

| ID | Verified behavior | Local result |
|----|-------------------|--------------|
| B4-FULL-013 | CLI `check`, `build`, `run`, `test`, unsupported command, and usage dispatch | pass on Linux x86_64 |
| B4-FULL-014 | Two fresh source-to-C-to-native builds produce byte-identical C and PE binaries with identical stdout | pass on Linux x86_64 |
| B4-FULL-015 | Two fresh full-surface builds produce byte-identical emitted C and identical stdout; native hashes remain platform-specific | pass on Linux x86_64 |
| B4-FULL-016 | Two fresh seed builds produce byte-identical C and PE binaries with identical stdout | pass on Linux x86_64 |
| B4-FULL-017 | Two independent second-stage rebuilds produce identical C, PE, and execution artifacts | pass on Linux x86_64 |
| B4-FULL-018 | Full-surface execution with Rust/Cargo environment variables removed matches normal execution | pass on Linux x86_64 |

Additional verified fixtures:

| Fixture | Description |
|---------|-------------|
| `bootstrap/fixtures/b4/c_backend_datastructures.zp` | List/map/control-flow/recursion exercises via C backend |

The verifier report is written to `target/b4-c-backend-acceptance.tsv`. Cross-platform CI compares emitted C and stdout hashes across Linux, Windows, and macOS while allowing native executable hashes to differ by target.

## Verified Gates (2026-09-15)

| Gate | Result | Notes |
|------|--------|-------|
| `python host/zap-bootstrap/verify_c_backend.py` | passed | 10 representative C backend programs passed |
| `scripts/bootstrap/verify_b4_c_backend_acceptance.sh` | passed | B4-FULL-013..018 passed 6/6 on Linux x86_64 |
| `scripts/bootstrap/verify_b4_rust_free_contract.sh` | passed | Schema v2 contract and 19-row manifest validated |
| `scripts/bootstrap/verify_b4_evidence.sh` | passed | Schema v2 evidence package and dynamic row counts validated |
| `scripts/bootstrap/verify_b4_seed_preflight_10.sh` | passed | Rust-free seed preflight checks pass |
| `make bootstrap-non-rust-test` | passed | Compiler and VM host run without Rust toolchain |
| `make bootstrap-three-stage-test` | passed | Three-stage self-hosting gate with `bin/zap` |
| `make bootstrap-self-rebuild-test` | passed | Byte-determinism, second-stage, three-stage, clean-env gates |

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
| `.github/workflows/ci.yml` C backend matrix | added | Runs the six-row C backend acceptance verifier on Linux, Windows, and macOS |
| `b4-c-backend-cross-platform` aggregator | added | Downloads all matrix reports and compares emitted C and stdout hashes |
| `scripts/bootstrap/verify_b4_c_backend_acceptance.sh` | added | Portable entrypoint for B4-FULL-013..018 |
| `host/zap-bootstrap/verify_b4_c_backend_acceptance.py` | added | Builds, executes, hashes, and reports C backend artifacts |
| `Makefile` `bootstrap-b4-c-backend-test` | added | Runs the complete C backend acceptance gate |

## Remaining Certification Blockers

| Blocker | Current Status | Required Action |
|---------|---------------|-----------------|
| Cross-platform C backend evidence | pending CI execution | Run the Linux/Windows/macOS matrix and aggregator on this revision |
| Production compiler ownership | candidate | Migrate the reference Python lowering path into the Zap-owned B1..B4 pipeline while retaining the C backend |
| Seed arbitrary-program execution | documented limitation | The current seed (`target/seeds/x86_64-unknown-linux-gnu/zap`) runs the embedded `c_backend_seed.zp` fixture only; it cannot yet run arbitrary `.zp` runner files. General-purpose VM support is needed for `verify_b4_three_stage_self_hosting.sh` to run with the seed instead of `bin/zap`. |
| Contract certification decision | not-certified | Update the contract only after cross-platform evidence and production ownership review pass |

## Certification Decision

The repository remains **not-certified**. The Rust-free C backend path now has executable local evidence for B4-FULL-013..018, but certification intentionally waits for the three-platform CI comparison, production Zap-owned compiler migration, and final contract review.
