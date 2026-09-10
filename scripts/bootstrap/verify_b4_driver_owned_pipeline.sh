#!/usr/bin/env bash
# B4 driver-owned pipeline execution verification.
#
# Verifies that the Zap compiler driver directly owns the full
# source → typed_ir → bytecode → VM execution pipeline without
# depending on native_independent.zp. This is driver-executable
# evidence for the owned compiler path.
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

fail() { echo "B4 driver pipeline failed: $*" >&2; exit 1; }

REPORT="${B4_DRIVER_PIPELINE_REPORT:-target/b4-driver-pipeline.tsv}"
mkdir -p "$(dirname "$REPORT")"

run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  elif [[ -n "${ZAP_BOOTSTRAP_BIN:-}" && -x "$ZAP_BOOTSTRAP_BIN" ]]; then
    "$ZAP_BOOTSTRAP_BIN" "$@"
  elif ! command -v cargo >/dev/null 2>&1; then
    echo "BLOCKED: no Zap runtime found; provide ZAP_BOOTSTRAP_BIN or build native/target/release/zap" >&2
    return 2
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}

[ -f bootstrap/b4/compiler_driver.zp ]

runner=$(mktemp "$ROOT_DIR/.zap-b4-driver-pipeline.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT

# Test 1: driver_execute_owned_pipeline with simple arithmetic
cat > "$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"

let source = "let a = 5\nlet b = 10\nsay a + b\n"
let first = driver_execute_owned_pipeline(source, "test1")
let second = driver_execute_owned_pipeline(source, "test1")

say first["status"]
say first["stage_chain_valid"]
say first["native_independent"]
say len(first["artifacts"])
say len(first["stages"])
say first["execution"]["error"]
say first["execution"]["output"][0]
say json(first) == json(second)
ZP

cat > "$expected" <<'EOF'
pipeline_executed
true
true
2
3
none
15
true
EOF

ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [[ -x "$ZAP_BIN" ]]; then
  "$ZAP_BIN" "$runner_rel" > "$out"
else
  run_zap "$runner_rel" > "$out"
fi
cmp "$out" "$expected" || fail "driver pipeline execution failed"

# Test 2: driver_check_source returns typed_ir artifact
cat > "$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"

let source = "let x: number = 1 + 2\n"
let result = driver_check_source(source, "test2.zp")
say result["status"]
say len(result["artifacts"])
say result["artifacts"][0]["kind"]
say result["native_independent"]
ZP

cat > "$expected" <<'EOF'
ok
1
typed_ir
true
EOF

if [[ -x "$ZAP_BIN" ]]; then
  "$ZAP_BIN" "$runner_rel" > "$out"
else
  run_zap "$runner_rel" > "$out"
fi
cmp "$out" "$expected" || fail "driver check source failed"

# Test 3: driver_build_source returns typed_ir + bytecode artifacts
cat > "$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"

let source = "let y = 42\nsay y\n"
let result = driver_build_source(source, "test3.zp")
say result["status"]
say len(result["artifacts"])
say result["artifacts"][0]["kind"]
say result["artifacts"][1]["kind"]
say result["native_independent"]
ZP

cat > "$expected" <<'EOF'
ok
2
typed_ir
bytecode
true
EOF

if [[ -x "$ZAP_BIN" ]]; then
  "$ZAP_BIN" "$runner_rel" > "$out"
else
  run_zap "$runner_rel" > "$out"
fi
cmp "$out" "$expected" || fail "driver build source failed"

# Test 4: driver_typed_ir_semantics validates owned typed-IR
cat > "$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"

let source = "let z = 3\nsay z\n"
let first = driver_typecheck_source(source, "test4.zp")
let second = driver_typecheck_source(source, "test4.zp")
let semantics = driver_typed_ir_semantics(first["typed_ir"], second["typed_ir"])
say semantics["valid"]
say semantics["deterministic"]
say semantics["ownership"]
say semantics["reference_owner"]
ZP

cat > "$expected" <<'EOF'
true
true
zap
zap
EOF

if [[ -x "$ZAP_BIN" ]]; then
  "$ZAP_BIN" "$runner_rel" > "$out"
else
  run_zap "$runner_rel" > "$out"
fi
cmp "$out" "$expected" || fail "driver typed-IR semantics failed"

# Test 5: driver_contract_status reports owned
cat > "$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"
say driver_contract_status()
ZP

cat > "$expected" <<'EOF'
owned
EOF

if [[ -x "$ZAP_BIN" ]]; then
  "$ZAP_BIN" "$runner_rel" > "$out"
else
  run_zap "$runner_rel" > "$out"
fi
cmp "$out" "$expected" || fail "driver contract status failed"

# Test 6: driver modules_rebuild is deterministic (no seed required)
cat > "$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"

let s1 = "let a = 1\nsay a\n"
let s2 = "let b = 2\nsay b\n"
let first = driver_modules_rebuild([s1, s2], ["a.zp", "b.zp"])
let second = driver_modules_rebuild([s1, s2], ["a.zp", "b.zp"])
say first["byte_equal"]
say second["byte_equal"]
say first["native_independent"]
ZP

cat > "$expected" <<'EOF'
true
true
true
EOF

if [[ -x "$ZAP_BIN" ]]; then
  "$ZAP_BIN" "$runner_rel" > "$out"
else
  run_zap "$runner_rel" > "$out"
fi
cmp "$out" "$expected" || fail "driver modules rebuild failed"

# Report
: > "$REPORT"
printf 'schema_version\t1\n' >> "$REPORT"
printf 'contract_id\tB4-DRIVER-OWNED-PIPELINE\n' >> "$REPORT"
printf 'status\tpassed\n' >> "$REPORT"
printf 'pipeline_execution\ttrue\n' >> "$REPORT"
printf 'check_source\ttrue\n' >> "$REPORT"
printf 'build_source\ttrue\n' >> "$REPORT"
printf 'typed_ir_semantics\ttrue\n' >> "$REPORT"
printf 'contract_status_owned\ttrue\n' >> "$REPORT"
printf 'modules_rebuild_deterministic\ttrue\n' >> "$REPORT"

printf 'B4 driver-owned pipeline gate passed: 6 driver-executable verification cases\n'
