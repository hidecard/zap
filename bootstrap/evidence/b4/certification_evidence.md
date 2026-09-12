# B4 Rust-Free Full-Language Certification Evidence

## Evidence Snapshot Date
2026-09-12

## Contract Status
- **Current:** `not-certified`
- **Reason:** candidate driver contract, deterministic gates, and verifier infrastructure are wired and passing on the current prebuilt seed; certification remains blocked pending a Zap-produced Rust-free seed, cross-platform clean-environment execution (Linux/Windows/macOS), and executable full-language self-rebuild evidence.

## Acceptance Rows (12/18 PASS; 6 provisional)

| ID | Area | Fixture | Owner | Artifact | Status |
|----|------|---------|-------|----------|--------|
| B4-FULL-001 | lexer-parser | `bootstrap/fixtures/b4/full_language_surface.zp` | `bootstrap/b1/parser.zp` | canonical_ast | ✅ pass |
| B4-FULL-002 | expressions-control-flow | `bootstrap/fixtures/typecheck/basic_type_matrix.zp` | `bootstrap/b2/typecheck.zp` | typed_ir | ✅ pass |
| B4-FULL-003 | functions-closures | `bootstrap/fixtures/typecheck/function.zp` | `bootstrap/b2/typed_ir.zp` | typed_ir | ✅ pass |
| B4-FULL-004 | classes-methods | `bootstrap/fixtures/typecheck/generic_class.zp` | `bootstrap/b3/lower.zp` | bytecode | ✅ pass |
| B4-FULL-005 | collections-maps | `bootstrap/fixtures/typecheck/collection_expression_map.zp` | `bootstrap/b3/lower.zp` | bytecode | ✅ pass |
| B4-FULL-006 | aliases-generics | `bootstrap/fixtures/typecheck/generic_compound_bounds.zp` | `bootstrap/b2/typecheck.zp` | diagnostics | ✅ pass |
| B4-FULL-007 | result-option | `bootstrap/fixtures/typecheck/expression_result_constructor.zp` | `bootstrap/b3/vm.zp` | vm_result | ✅ pass |
| B4-FULL-008 | modules-imports | `bootstrap/fixtures/typecheck/generic_cross_module.zp` | `bootstrap/b3/package.zp` | module_graph | ✅ pass |
| B4-FULL-009 | async-runtime | `bootstrap/fixtures/typecheck/flow_engine.zp` | `bootstrap/b3/vm.zp` | vm_result | ✅ pass |
| B4-FULL-010 | diagnostics | `bootstrap/fixtures/typecheck/function_incompatible.zp` | `bootstrap/b2/typecheck.zp` | stable_diagnostic | ✅ pass |
| B4-FULL-011 | package-build | `bootstrap/b3/package.zp` | `bootstrap/b3/package.zp` | build_artifact | ✅ pass |
| B4-FULL-012 | test-runner | `bootstrap/b4/runner.zp` | `bootstrap/b4/runner.zp` | test_result | ✅ pass |
| B4-FULL-013 | cli-entrypoint | `bootstrap/b4/compiler_driver.zp` | `bootstrap/b4/compiler_driver.zp` | cli_result | provisional |
| B4-FULL-014 | self-rebuild | `bootstrap/fixtures/b4/full_language_surface.zp` | `bootstrap/b4/compiler_driver.zp` | self_rebuild_bytes | provisional |
| B4-FULL-015 | cross-platform-determinism | `bootstrap/fixtures/b4/full_language_surface.zp` | `bootstrap/b4/compiler_driver.zp` | platform_rebuild | provisional |
| B4-FULL-016 | byte-determinism | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_byte_determinism.sh` | artifact_bytes | provisional |
| B4-FULL-017 | second-stage-rebuild | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | stage2_artifact | provisional |
| B4-FULL-018 | clean-environment | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_clean_environment.sh` | clean_run | provisional |

## Verified Gates (2026-09-12)

| Gate | Result | Notes |
|------|--------|-------|
| `scripts/bootstrap/verify_b4_rust_free_contract.sh` | ✅ passed | 18 acceptance rows validated; contract status: not-certified |
| `scripts/bootstrap/verify_b4_evidence.sh --run-gates` | ✅ passed | Contract integrity, acceptance manifest, evidence document references, and all delegated B4 gates passed |
| `scripts/bootstrap/verify_b4_byte_determinism.sh` | ✅ passed | Frontend, typed-IR, backend, and pipeline replay verified in bounded fresh processes |
| `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | ✅ passed | 6 deterministic second-stage cases passed |
| `scripts/bootstrap/verify_b4_clean_environment.sh` | ✅ passed | 5 clean-environment cases passed, including no-state-leakage checks |
| `scripts/bootstrap/verify_b4_typed_ir_source_rebuild_37.sh` | ✅ passed | Zap source → typed-IR → bytecode/VM handoff and reproducible rebuild passed |
| `scripts/bootstrap/verify_b4_source_to_vm_10.sh` | ✅ passed | 10 bounded source-to-VM acceptance cases passed |

## Provisional Row Evidence

| ID | Blocker | Required Evidence |
|----|---------|-------------------|
| B4-FULL-013 | CLI entrypoint | Executable seed evidence for `driver_command()` across all supported commands |
| B4-FULL-014 | Self-rebuild | Zap-produced seed that can rebuild itself byte-for-byte |
| B4-FULL-015 | Cross-platform determinism | Linux/Windows/macOS clean-environment evidence with platform seed artifacts |
| B4-FULL-016 | Byte-determinism | Verified prebuilt Zap seed provenance (current seed is native/Cargo-built) |
| B4-FULL-017 | Second-stage rebuild | Zap-produced seed for second-stage rebuild evidence |
| B4-FULL-018 | Clean-environment | Clean VM execution without Rust/Cargo on all supported platforms |

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
  "notes": "Local prebuilt Windows x86_64 seed. Deterministic gates (byte-determinism, second-stage-rebuild, clean-environment) verified passing with this binary. B4 certification remains blocked pending a Zap-produced Rust-free seed and cross-platform clean-environment evidence."
}
```

## Evidence Artifacts

- B4 milestone report: `target/b4-evidence-report.tsv`
- Rebuild artifacts: `target/b4-rebuild-*`
- Platform provenance: `target/b4-platform-*`
- Byte-determinism records: `target/b4-byte-*`
- Clean-environment records: `target/b4-clean-environment.tsv`

## Certification Decision

The repository remains **not-certified**. Candidate contract and ownership wiring are verified, and deterministic gates pass on the current prebuilt seed, but certification is intentionally blocked until:
1. A Zap-produced Rust-free seed is available
2. Cross-platform clean-environment evidence exists for Linux, Windows, and macOS
3. Executable full-language self-rebuild evidence is recorded
