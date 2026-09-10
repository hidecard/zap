# B4 Rust-Free Full-Language Certification Evidence

## Certification Date
2026-09-10 (draft — not certified)

## Contract Status
- **Current:** `not-certified`
- **Target:** `certified`

## Acceptance Rows (20/20 pass; verified with current native binary as seed)

| ID | Area | Fixture | Owner | Artifact | Status |
|---|---|---|---|---|---|
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
| B4-FULL-013 | cli-entrypoint | `bootstrap/b4/compiler_driver.zp` | `bootstrap/b4/compiler_driver.zp` | driver_status | ✅ pass |
| B4-FULL-014 | self-rebuild | `bootstrap/fixtures/b4/full_language_surface.zp` | `bootstrap/b4/compiler_driver.zp` | driver_rebuild | ✅ pass |
| B4-FULL-015 | cross-platform-determinism | `bootstrap/fixtures/b4/full_language_surface.zp` | `bootstrap/b4/compiler_driver.zp` | driver_modules_rebuild | ✅ pass |
| B4-FULL-016 | byte-determinism | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_byte_determinism.sh` | artifact_bytes | ✅ pass |
| B4-FULL-017 | second-stage-rebuild | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | stage2_artifact | ✅ pass |
| B4-FULL-018 | clean-environment | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_clean_environment.sh` | clean_run | ✅ pass |
| B4-FULL-019 | driver-source-to-vm | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_driver_source_to_vm.sh` | driver_source_vm | ✅ pass |
| B4-FULL-020 | driver-owned-pipeline | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_driver_owned_pipeline.sh` | driver_pipeline | ✅ pass |

## Current Evidence State

All 20 B4 acceptance gates pass with the current native binary as the verified `ZAP_BOOTSTRAP_BIN` seed.

### Driver-owned pipeline (no seed required)
- `verify_compiler_driver_contract.sh`: ✅ pass
- `verify_full_language_backend_ownership.sh`: ✅ pass
- `verify_b4_driver_module_resolution_errors.sh`: ✅ pass
- `verify_b4_driver_owned_pipeline.sh`: ✅ pass
- `verify_b4_driver_source_to_vm.sh`: ✅ pass
- `verify_b4_typed_ir_source_rebuild_37.sh`: ✅ pass
- `verify_b4_supported_subset_rebuild_43.sh`: ✅ pass
- `verify_b4_seed_preflight.sh`: ✅ pass

### Seed-dependent gates (passing with current native binary)
- `verify_b4_byte_determinism.sh`: ✅ pass
- `verify_b4_second_stage_rebuild.sh`: ✅ pass
- `verify_b4_clean_environment.sh`: ✅ pass

## Seed Preflight Requirements

A verified `ZAP_BOOTSTRAP_BIN` must satisfy:
1. Executable Zap binary that responds to `--version`
2. Reports `driver_contract_status() = "owned"`
3. Executes `driver_execute_owned_pipeline` successfully
4. Produces deterministic output across fresh processes
5. Resolves modules via `driver_resolve_modules`

Use `bash scripts/bootstrap/verify_b4_seed_preflight.sh` to validate a candidate seed.

## Certification Decision

**Candidate evidence passing.** The driver contract is promoted to `owned` and the driver-owned pipeline is verified passing. Seed-dependent B4 gates (byte-determinism, second-stage rebuild, clean-environment) pass with the current native binary as a verified seed. Full B4 certification requires the same evidence reproduced on Linux x86_64, macOS ARM64, and Windows x86_64 clean environments with platform-native seeds.
