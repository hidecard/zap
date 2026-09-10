# B4 Rust-Free Full-Language Certification Evidence

## Certification Date
2026-09-10 (draft — not certified)

## Contract Status
- **Current:** `not-certified`
- **Target:** `certified`

## Acceptance Rows (20/20 manifest entries; 14 driver-owned pass, 6 seed-dependent pending)

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
| B4-FULL-013 | cli-entrypoint | `bootstrap/b4/compiler_driver.zp` | `bootstrap/b4/compiler_driver.zp` | cli_result | ⏳ pending seed |
| B4-FULL-014 | self-rebuild | `bootstrap/fixtures/b4/full_language_surface.zp` | `bootstrap/b4/compiler_driver.zp` | self_rebuild_bytes | ⏳ pending seed |
| B4-FULL-015 | cross-platform-determinism | `bootstrap/fixtures/b4/full_language_surface.zp` | `bootstrap/b4/compiler_driver.zp` | platform_rebuild | ⏳ pending seed |
| B4-FULL-016 | byte-determinism | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_byte_determinism.sh` | artifact_bytes | ⏳ blocked (no seed) |
| B4-FULL-017 | second-stage-rebuild | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_second_stage_rebuild.sh` | stage2_artifact | ⏳ blocked (no seed) |
| B4-FULL-018 | clean-environment | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_clean_environment.sh` | clean_run | ⏳ blocked (no seed) |
| B4-FULL-019 | driver-source-to-vm | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_driver_source_to_vm.sh` | driver_source_vm | ✅ pass |
| B4-FULL-020 | driver-owned-pipeline | `bootstrap/fixtures/b4/full_language_surface.zp` | `scripts/bootstrap/verify_b4_driver_owned_pipeline.sh` | driver_pipeline | ✅ pass |

## Current Evidence State

### Driver-owned pipeline (no seed required)
- `verify_compiler_driver_contract.sh`: ✅ pass — contract status `owned`, no `native_independent.zp` dependency in driver
- `verify_full_language_backend_ownership.sh`: ✅ pass — driver owns all pipeline stages directly
- `verify_b4_driver_module_resolution_errors.sh`: ✅ pass — module error coverage (ZAP-MODULE-002..005)
- `verify_b4_driver_owned_pipeline.sh`: ✅ pass — 6 driver-executable verification cases
- `verify_b4_driver_source_to_vm.sh`: ✅ pass — driver-owned source-to-VM execution
- `verify_b4_typed_ir_source_rebuild_37.sh`: ✅ pass — driver-owned typed-IR source rebuild
- `verify_b4_supported_subset_rebuild_43.sh`: ✅ pass — driver-owned subset rebuild determinism
- `verify_b4_seed_preflight.sh`: ✅ pass — seed preflight validator (when ZAP_BOOTSTRAP_BIN is set)

### Seed-dependent gates (blocked pending verified `ZAP_BOOTSTRAP_BIN`)
- `verify_b4_byte_determinism.sh`: ⏳ blocked — requires prebuilt seed
- `verify_b4_second_stage_rebuild.sh`: ⏳ blocked — requires prebuilt seed
- `verify_b4_clean_environment.sh`: ⏳ blocked — requires prebuilt seed

## Seed Preflight Requirements

A verified `ZAP_BOOTSTRAP_BIN` must satisfy:
1. Executable Zap binary that responds to `--version`
2. Reports `driver_contract_status() = "owned"`
3. Executes `driver_execute_owned_pipeline` successfully
4. Produces deterministic output across fresh processes
5. Resolves modules via `driver_resolve_modules`

Use `bash scripts/bootstrap/verify_b4_seed_preflight.sh` to validate a candidate seed.

## Certification Decision

**Not certified.** The driver contract is promoted to `owned` and the driver-owned pipeline is verified passing. However, B4 certification requires a verified prebuilt `ZAP_BOOTSTRAP_BIN` produced without Rust/Cargo, plus two-stage/three-stage rebuild evidence and Linux/macOS/Windows clean-environment runs. These remain blocked until the seed is produced and validated.
