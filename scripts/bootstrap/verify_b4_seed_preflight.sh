#!/usr/bin/env bash
# B4 seed preflight validator.
#
# Verifies that a candidate ZAP_BOOTSTRAP_BIN meets the requirements for
# B4 self-hosting evidence: executable, deterministic, and capable of
# running the driver-owned pipeline.
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

fail() { echo "B4 seed preflight failed: $*" >&2; exit 1; }
pass() { echo "PASS: $*"; }

SEED="${ZAP_BOOTSTRAP_BIN:-}"
if [[ -z "$SEED" ]]; then
  fail "ZAP_BOOTSTRAP_BIN is not set; provide a verified prebuilt Zap seed"
fi
if [[ ! -f "$SEED" ]]; then
  fail "ZAP_BOOTSTRAP_BIN points to missing file: $SEED"
fi
if [[ ! -x "$SEED" ]]; then
  fail "ZAP_BOOTSTRAP_BIN is not executable: $SEED"
fi

REPORT="${B4_SEED_PREFLIGHT_REPORT:-target/b4-seed-preflight.tsv}"
mkdir -p "$(dirname "$REPORT")"

# 1. Version sanity check
version="$("$SEED" --version 2>/dev/null || echo "unknown")"
if [[ "$version" == "unknown" ]]; then
  fail "seed binary does not respond to --version"
fi
pass "seed version: $version"

# 2. Driver contract status check
status_runner=$(mktemp "$ROOT_DIR/.zap-b4-seed-preflight-status.XXXXXX.zp")
trap 'rm -f "$runner" "$out1" "$out2" "$status_runner"' EXIT
cat > "$status_runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"
say driver_contract_status()
ZP

driver_status=$("$SEED" "$(basename "$status_runner")" 2>/dev/null || echo "error")
if [[ "$driver_status" != "owned" ]]; then
  fail "seed does not report driver_contract_status=owned (got: $driver_status)"
fi
pass "driver contract status: $driver_status"

# 3. Simple pipeline execution check
runner=$(mktemp "$ROOT_DIR/.zap-b4-seed-preflight.XXXXXX.zp")
runner_rel=$(basename "$runner")
out1=$(mktemp)
out2=$(mktemp)
cat > "$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"
let result = driver_execute_owned_pipeline("let x = 1\nsay x\n", "preflight")
say result["status"]
say result["native_independent"]
say result["execution"]["output"][0]
ZP

pipeline_out=$("$SEED" "$runner_rel" 2>/dev/null || echo "error")
if [[ "$pipeline_out" != *$'pipeline_executed\ntrue\n1'* ]]; then
  fail "seed pipeline execution failed (output: $pipeline_out)"
fi
pass "driver-owned pipeline execution works"

# 4. Determinism check: run twice, compare
"$SEED" "$runner_rel" > "$out1"
"$SEED" "$runner_rel" > "$out2"
if ! cmp -s "$out1" "$out2"; then
  fail "seed is not deterministic across fresh processes"
fi
pass "seed determinism verified"

# 5. Module resolution check
cat > "$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"
let sources = ["let a = 1\nsay a\n", "import \"a\"\nsay a\n"]
let names = ["a.zp", "b.zp"]
let result = driver_resolve_modules(sources, names)
say result["status"]
say len(result["modules"])
ZP

module_out=$("$SEED" "$runner_rel" 2>/dev/null || echo "error")
if [[ "$module_out" != *$'modules_resolved\n2'* ]]; then
  fail "seed module resolution failed (output: $module_out)"
fi
pass "driver-owned module resolution works"

# Report
cat > "$REPORT" <<EOF
schema_version	1
contract_id	B4-SEED-PREFLIGHT
status	passed
seed_path	$SEED
seed_version	$version
driver_contract_status	$driver_status
pipeline_execution	true
determinism	true
module_resolution	true
verified_at	$(date -u +%Y-%m-%dT%H:%M:%SZ)
EOF

printf 'B4 seed preflight passed: seed meets requirements for B4 self-hosting evidence\n'
