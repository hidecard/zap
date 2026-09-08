#!/usr/bin/env bash
# Verify the candidate compiler-driver contract and its stable Zap exports.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
CONTRACT="bootstrap/contracts/COMPILER_DRIVER_CONTRACT.toml"
DRIVER="bootstrap/b4/compiler_driver.zp"
FIXTURE="bootstrap/fixtures/driver/driver_smoke.zp"
fail() { echo "compiler-driver contract failed: $*" >&2; exit 1; }
[[ -f "$CONTRACT" ]] || fail "missing $CONTRACT"
[[ -f "$DRIVER" ]] || fail "missing $DRIVER"
[[ -f "$FIXTURE" ]] || fail "missing $FIXTURE"
python3 - "$CONTRACT" <<'PY'
import sys, tomllib
with open(sys.argv[1], "rb") as handle:
    data = tomllib.load(handle)
assert data["schema_version"] == 1
assert data["contract_id"] == "ZAP-COMPILER-DRIVER"
assert data["status"] == "candidate"
assert data["pipeline"]["stages"] == ["source", "typed_ir", "bytecode", "execution"]
assert data["pipeline"]["owner"] == "bootstrap/b4/compiler_driver.zp"
assert data["pipeline"]["seed"] == "bootstrap/b4/compiler_driver.zp"
assert data["determinism"]["seed_epoch"] == 0
assert data["artifact"]["newline"] == "LF"
PY
for export in driver_parse_source driver_typecheck_source driver_normalize_module_name driver_module_name_matches driver_module_match_count driver_collect_imports driver_resolve_modules driver_source_index driver_check_modules driver_build_modules driver_module_build_manifest driver_modules_rebuild driver_typed_ir_semantics driver_compile_backend driver_execute_owned_pipeline driver_check_source driver_build_source driver_run_source driver_test_source driver_build_package driver_artifact_record driver_canonical_artifacts driver_artifact_manifest driver_rebuild driver_subset_run driver_subset_rebuild; do
  grep -q "^export fn ${export}(" "$DRIVER" || fail "missing export: $export"
done
grep -q '"typed_ir"' "$DRIVER" || fail "canonical artifact order must include typed_ir"
grep -q '"bytecode"' "$DRIVER" || fail "canonical artifact order must include bytecode"
if grep -q 'native_independent.zp' "$CONTRACT" "$DRIVER"; then
  fail "driver contract must not depend on the removed composite seed"
fi
grep -q 'ZAP_BOOTSTRAP_BIN' scripts/bootstrap/verify_b4_byte_determinism.sh || fail "byte-determinism gate must require a prebuilt seed"
grep -q 'ZAP_BOOTSTRAP_BIN' scripts/bootstrap/verify_b4_second_stage_rebuild.sh || fail "second-stage gate must require a prebuilt seed"
grep -q 'ZAP_BOOTSTRAP_BIN' scripts/bootstrap/verify_b4_clean_environment.sh || fail "clean-environment gate must require a prebuilt seed"
if grep -n -E 'timestamp|hostname|username|absolute_path|temp_path|pointer_address' "$DRIVER"; then
  fail "driver source contains forbidden non-deterministic metadata"
fi
python3 - "$FIXTURE" <<'PY'
from pathlib import Path
source = Path(__import__("sys").argv[1]).read_text(encoding="utf-8")
assert "let answer = 40 + 2" in source
assert source.endswith("\n")
PY
printf 'compiler-driver contract gate passed: candidate driver exports and deterministic policy verified\n'
