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
  'export fn c_backend_validation_opcode_manifest(' \
  'export fn c_backend_opcode_manifest('; do
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
manifest_runner=$(mktemp "$ROOT_DIR/.zap-c-backend-manifest.XXXXXX.zp")
out=$(mktemp)
expected=$(mktemp)
python_manifest=$(mktemp)
zap_manifest=$(mktemp)
trap 'rm -f "$runner" "$manifest_runner" "$out" "$expected" "$python_manifest" "$zap_manifest"' EXIT
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

# The canonical opcode contract is shared, not merely equal in size: compare the
# full opcode sets so a backend cannot silently grow or shrink its contract.
cat > "$manifest_runner" <<'EOF'
import "bootstrap/b4/c_backend.zp"
for op in c_backend_validation_opcode_manifest():
    say op
EOF
run_zap "$(basename "$manifest_runner")" | tr -d '\r' > "$zap_manifest"
python3 - "$python_manifest" <<'PY'
import sys
from pathlib import Path
sys.path.insert(0, str(Path('host/zap-bootstrap').resolve()))
from c_backend import _BYTECODE_SUPPORTED_OPS
Path(sys.argv[1]).write_bytes(("\n".join(sorted(_BYTECODE_SUPPORTED_OPS)) + "\n").encode("utf-8"))
PY
if ! diff -u "$python_manifest" "$zap_manifest"; then
  fail 'Zap and Python canonical opcode manifests differ'
fi

# Every canonical opcode must be lowerable by the Zap-owned backend. Opcodes
# the backend cannot lower would otherwise validate and then emit nothing.
cat > "$manifest_runner" <<'EOF'
import "bootstrap/b4/c_backend.zp"
let structural = ["call", "function_def"]
for op in c_backend_validation_opcode_manifest():
    if contains(structural, op):
        continue
    let instruction = {"op": op}
    if op == "const":
        instruction = {"op": "const", "value": 1}
    if op == "jump" or op == "jump_if_false" or op == "jump_if_true":
        instruction = {"op": op, "target": 0}
    if op == "make_list":
        instruction = {"op": "make_list", "count": 0}
    if op == "store" or op == "load":
        instruction = {"op": op, "name": "x"}
    if len(c_backend_emit_instruction(instruction)) == 0:
        say "missing_emitter:" + op
EOF
run_zap "$(basename "$manifest_runner")" > "$out"
if [[ -s "$out" ]]; then
  echo '--- unlowerable canonical opcodes ---' >&2
  cat "$out" >&2
  fail 'canonical opcodes without a Zap-owned emitter'
fi

printf 'B4 C-backend validation parity passed: canonical artifacts, scalar/string values, collection rejection, opcode rejection, and the full shared opcode contract agree\n'
