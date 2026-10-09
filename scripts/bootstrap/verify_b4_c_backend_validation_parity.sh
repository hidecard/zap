#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"

fail() { echo "FAIL: $*" >&2; exit 1; }

for required in \
  'export fn c_backend_validate_bytecode(' \
  'export fn c_backend_validate_artifact(' \
  'export fn c_backend_validation_contract(' \
  'export fn c_backend_artifact_instructions(' \
  'export fn c_backend_validation_opcode_manifest('; do
  grep -q "$required" bootstrap/b4/c_backend.zp || fail "missing Zap-owned export: $required"
done

python3 - <<'PY'
import sys
from pathlib import Path
sys.path.insert(0, str(Path('host/zap-bootstrap').resolve()))
from c_backend import BytecodeValidationError, validate_canonical_bytecode

cases = [
    ({"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "const", "value": 7}, {"op": "print"}, {"op": "halt"}]}, True),
    ({"artifact_kind": "bytecode", "instructions": [{"op": "const", "value": "zap"}, {"op": "print"}, {"op": "halt"}]}, True),
    ({"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "const", "value": [1, 2]}]}, False),
    ({"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "unknown"}]}, False),
    ({"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "jump", "target": 1}]}, True),
    ({"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "jump"}]}, False),
]
for artifact, expected in cases:
    try:
        validate_canonical_bytecode(artifact)
        actual = True
    except (BytecodeValidationError, ValueError, TypeError, KeyError):
        actual = False
    if actual != expected:
        raise SystemExit(f"Python validation parity fixture mismatch: expected={expected} actual={actual} artifact={artifact}")
print("Python canonical-bytecode validation fixtures passed")
PY

run_zap() {
  if [[ -n "${ZAP_BOOTSTRAP_BIN:-}" && -x "$ZAP_BOOTSTRAP_BIN" ]]; then
    "$ZAP_BOOTSTRAP_BIN" "$@"
  elif [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  else
    fail "a verified prebuilt Zap seed is required; set ZAP_BOOTSTRAP_BIN"
  fi
}

runner=$(mktemp "$ROOT_DIR/.zap-c-backend-parity.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/c_backend.zp"
let valid_scalar = {"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "const", "value": 7}, {"op": "print"}, {"op": "halt"}]}
let valid_string = {"artifact_kind": "bytecode", "instructions": [{"op": "const", "value": "zap"}, {"op": "print"}, {"op": "halt"}]}
let invalid_collection = {"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "const", "value": [1, 2]}]}
let invalid_opcode = {"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "unknown"}]}
let valid_jump = {"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "jump", "target": 1}]}
let invalid_jump = {"kind": "zap.bytecode", "schema_version": 1, "instructions": [{"op": "jump"}]}
say c_backend_validate_artifact(valid_scalar)["status"]
say c_backend_validate_artifact(valid_string)["status"]
say c_backend_validate_artifact(invalid_collection)["status"]
say c_backend_validate_artifact(invalid_opcode)["status"]
say c_backend_validate_artifact(valid_jump)["status"]
say c_backend_validate_artifact(invalid_jump)["status"]
say len(c_backend_validation_opcode_manifest())
say c_backend_validation_contract()["status"]
EOF
cat > "$expected" <<'EOF'
validated
validated
rejected
rejected
validated
rejected
65
zap_owned_validation_slice
EOF
run_zap "$(basename "$runner")" > "$out"
cmp "$out" "$expected" || { echo '--- actual ---'; cat "$out"; echo '--- expected ---'; cat "$expected"; fail 'Zap/Python canonical-bytecode validation parity mismatch'; }
printf 'B4 C-backend validation parity passed: canonical artifacts, scalar/string values, collection rejection, and opcode rejection agree\n'
