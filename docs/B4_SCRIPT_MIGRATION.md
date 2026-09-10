# B4 Verification Script Migration Guide

## Overview

B4 verification scripts use one of two entrypoints:
- `bootstrap/b4/compiler_driver.zp` — the owned Zap compiler driver
- `bootstrap/b4/native_independent.zp` — the seed-compiler fixture (retained for backward compatibility)

## When to Use Which Entrypoint

### Use `compiler_driver.zp` when:
- Testing the owned compiler driver boundary
- Testing driver-owned pipeline execution
- Testing driver check/build/run/test commands
- Testing module resolution via the driver
- Testing typed-IR semantics via the driver
- Any new B4 verification work

### Use `native_independent.zp` when:
- Testing the seed fixture's internal `seed_*` functions
- Testing AST compilation via `seed_compile_ast_source`
- Testing typed-IR compilation via `seed_compile_typed_ir`
- Testing contract/validation functions that have no driver equivalent
- Maintaining existing B4 evidence that predates the driver

## Function Mapping

| `native_independent.zp` | `compiler_driver.zp` | Notes |
|---|---|---|
| `seed_execute_owned_pipeline` | `driver_execute_owned_pipeline` | Status strings differ: `pipeline_executed` vs `candidate_pipeline_executed` |
| `seed_compile_source` | `driver_build_source` / `driver_run_source` | Instructions embedded in artifacts |
| `seed_compile_bytes` | N/A | Use `json(result["artifacts"][N]["bytes"])` |
| `seed_self_rebuild` | `driver_rebuild` | Manifest format differs |
| `seed_build_package_owned` | `driver_build_package` | Driver version lacks `execution` output |
| `seed_compile_ast_source` | **No equivalent** | AST compilation not yet in driver |
| `seed_compile_typed_ir` | **No equivalent** | Typed-IR compilation not yet in driver |
| `seed_self_rebuild_ast` | **No equivalent** | AST rebuild not yet in driver |
| `seed_pipeline_replay` | Manual replay | Call `driver_execute_owned_pipeline` twice |
| `seed_owned_pipeline_contract` | **No equivalent** | Add to driver if needed |
| `seed_package_artifact_contract` | **No equivalent** | Add to driver if needed |
| `vm_run` | `vm_run` (from `bootstrap/b3/vm.zp`) | Same function, import from vm.zp |

## Migration Checklist

When migrating a B4 script from `native_independent.zp` to `compiler_driver.zp`:

1. Change import: `import "bootstrap/b4/native_independent.zp"` → `import "bootstrap/b4/compiler_driver.zp"`
2. Replace `seed_*` function calls with `driver_*` equivalents
3. Update expected status strings:
   - `candidate_pipeline_executed` → `pipeline_executed`
   - `candidate_pipeline_error` → `pipeline_error`
   - `candidate_pipeline_replay` → manual replay
   - `candidate_supported_subset_rebuild` → `driver_subset_rebuild`
   - `candidate_driver_rebuild` → `driver_rebuild`
   - `candidate_driver_subset_rebuild` → `driver_subset_rebuild`
   - `candidate_module_graph_error` → `module_graph_error`
   - `candidate_module_graph_replay` → `module_graph_replay`
   - `candidate_module_rebuild` → `module_rebuild`
   - `candidate_typed_ir_semantics` → `typed_ir_semantics_verified`
   - `compiled_ast_slice` → `compiled_driver_backend` (or keep if using `driver_compile_ast_source`)
4. Update `native_independent` expectations:
   - `native_independent: false` → `native_independent: true`
5. Update artifact extraction:
   - `result["instructions"]` → `result["artifacts"][N]["bytes"]` (parsed JSON)
   - `seed_compile_bytes(result)` → `json(result["artifacts"][0]["bytes"])`

## Current Migration Status

### Migrated to `compiler_driver.zp` (owned)
- `verify_b4_driver_owned_pipeline.sh` ✅
- `verify_b4_driver_source_to_vm.sh` ✅
- `verify_b4_driver_module_resolution_errors.sh` ✅
- `verify_b4_typed_ir_source_rebuild_37.sh` ✅
- `verify_b4_supported_subset_rebuild_43.sh` ✅
- `verify_b4_clean_environment.sh` ✅
- `verify_b4_byte_determinism.sh` ✅
- `verify_b4_second_stage_rebuild.sh` ✅
- `verify_b4_seed_preflight.sh` ✅
- `verify_b4_evidence.sh` ✅

### Still using `native_independent.zp` (seed fixture)
These scripts test the seed fixture's internal functions and are expected to remain on `native_independent.zp` until driver equivalents are added:
- `verify_b4_source_to_vm_*.sh` (12 scripts) — use `seed_compile_source` and `vm_run`
- `verify_b4_source_to_vm_ast_*.sh` (10 scripts) — use `seed_compile_ast_source`
- `verify_b4_typed_ir_to_vm_27.sh` — uses `seed_compile_typed_ir`
- `verify_b4_owned_pipeline_42.sh` — uses `seed_execute_owned_pipeline` and `seed_pipeline_replay`
- `verify_b4_owned_package_build_45.sh` — uses `seed_build_package_owned`
- `verify_b4_pipeline_malformed_safety_46.sh` — uses `seed_execute_owned_pipeline`
- `verify_b4_inferred_typed_ir_self_compile_39.sh` — uses `seed_compile_inferred_typed_ir`
- `verify_b4_method_overloads_26.sh` — uses `seed_compile_source`
- `verify_b4_self_rebuild_ast_36.sh` — uses `seed_self_rebuild_ast`
- `verify_b4_a10_a13_artifact_linkage_56.sh` — uses `seed_owned_pipeline_contract`
- `verify_b4_a13_supported_rebuild_evidence_49.sh` — uses `seed_a13_supported_rebuild_evidence`
- `verify_b4_platform_self_rebuild_evidence_48.sh` — uses seed evidence functions
- `verify_b4_vm_execution_contract_47.sh` — uses VM execution contract
- `verify_b4_zap_pipeline_artifact_12.sh` — uses artifact functions

## Adding New Driver Functions

When a B4 script needs a function that doesn't exist in `compiler_driver.zp`:

1. Add the function to `bootstrap/b4/compiler_driver.zp`
2. Set `native_independent: true` in all returned records
3. Use owned status strings (no `candidate_` prefix)
4. Export the function
5. Update this migration guide
6. Migrate the B4 script to use the new driver function

## Priority Migration List

Scripts that should be migrated next (medium difficulty):
1. `verify_b4_pipeline_malformed_safety_46.sh` — uses `seed_execute_owned_pipeline` (has driver equivalent)
2. `verify_b4_method_overloads_26.sh` — uses `seed_compile_source` (has driver equivalent)
3. `verify_b4_owned_package_build_45.sh` — uses `seed_build_package_owned` (has driver equivalent)
4. `verify_b4_owned_pipeline_42.sh` — uses `seed_execute_owned_pipeline` and `seed_pipeline_replay`
5. `verify_b4_source_to_vm_break_payload_10.sh` — uses `seed_compile_source`
6. `verify_b4_source_to_vm_closures_12.sh` — uses `seed_compile_source`

Scripts that require new driver functions (hard):
- All `verify_b4_source_to_vm_ast_*.sh` — need `driver_compile_ast_source`
- `verify_b4_typed_ir_to_vm_27.sh` — need `driver_compile_typed_ir`
- `verify_b4_self_rebuild_ast_36.sh` — need `driver_self_rebuild_ast`
- `verify_b4_inferred_typed_ir_self_compile_39.sh` — need `driver_compile_inferred_typed_ir`
