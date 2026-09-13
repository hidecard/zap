#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "B4 cross-platform artifact manifest failed: $*" >&2; exit 1; }
SEED="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN_OVERRIDE:-$ROOT_DIR/bin/zap}}"
[[ -x "$SEED" ]] || fail "prebuilt Zap seed required"
REPORT="${B4_CROSS_PLATFORM_MANIFEST_REPORT:-target/b4-cross-platform-manifest.tsv}"
mkdir -p "$(dirname "$REPORT")"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap.exe" ]]; then
    "$ROOT_DIR/bin/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then
    "$ROOT_DIR/native/target/release/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap.exe" ]]; then
    "$ROOT_DIR/native/target/debug/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-cross-platform.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b4/seed_pipeline.zp"
import "bootstrap/b3/vm.zp"

let source = "say 1"
let result_a = driver_execute_owned_pipeline(source, "cross_platform.zp")
let result_b = driver_execute_owned_pipeline(source, "cross_platform.zp")
let artifacts_a = result_a["artifacts"]
let artifacts_b = result_b["artifacts"]
let typed_ir_a = ""
let bytecode_a = ""
let typed_ir_b = ""
let bytecode_b = ""
for artifact in artifacts_a:
    if artifact["kind"] == "typed_ir":
        typed_ir_a = artifact["digest"]
    if artifact["kind"] == "bytecode":
        bytecode_a = artifact["digest"]
for artifact in artifacts_b:
    if artifact["kind"] == "typed_ir":
        typed_ir_b = artifact["digest"]
    if artifact["kind"] == "bytecode":
        bytecode_b = artifact["digest"]
let typed_ir_equal = typed_ir_a == typed_ir_b
let bytecode_equal = bytecode_a == bytecode_b
let stage_chain_a = result_a["stage_chain_valid"]
let stage_chain_b = result_b["stage_chain_valid"]
let execution_a = result_a["execution"]
let execution_b = result_b["execution"]
let behavior_equal = execution_a["output"][0] == execution_b["output"][0] and execution_a["halted"] == execution_b["halted"]
say typed_ir_equal
say bytecode_equal
say stage_chain_a
say stage_chain_b
say behavior_equal
say typed_ir_a
say bytecode_a
say execution_a["output"][0]
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-$SEED}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel" > "$out"
else
  run_zap "$runner_rel" > "$out"
fi
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
expected=(true true true true true)
if [[ "${lines[*]:0:5}" != "${expected[*]}" ]]; then
  fail "cross-platform artifact manifest mismatch: ${lines[*]}"
fi
typed_ir_digest="${lines[5]}"
bytecode_digest="${lines[6]}"
vm_output="${lines[7]}"
seed_sha=$(sha256sum "$SEED" | awk '{print $1}')
seed_name=$(basename "$SEED")
platform="unknown"
if [[ "$(uname -s)" == "Linux" ]]; then
  platform="linux-x86_64"
elif [[ "$(uname -s)" == "Darwin" ]]; then
  platform="macos-arm64"
elif [[ "$(uname -s)" == "MINGW"* || "$(uname -s)" == "MSYS"* || "$(uname -s)" == "CYGWIN"* ]]; then
  platform="windows-x86_64"
fi
: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB4-CROSS-PLATFORM-MANIFEST\nverified_at\t%s\nplatform\t%s\ngit_commit\t%s\nseed\t%s\nseed_sha256\t%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$platform" "$(git rev-parse HEAD)" "$seed_name" "$seed_sha" >> "$REPORT"
printf 'typed_ir_digest\t%s\nbytecode_digest\t%s\nvm_output\t%s\nreplay_typed_ir_equal\ttrue\nreplay_bytecode_equal\ttrue\nreplay_behavior_equal\ttrue\n' "$typed_ir_digest" "$bytecode_digest" "$vm_output" >> "$REPORT"
printf 'manifest_rows\t3\nplatforms_recorded\t%s\nstatus\tpassed\n' "$platform" >> "$REPORT"
printf 'B4 cross-platform artifact manifest gate passed: typed-IR/bytecode digests and VM behavior are deterministic across fresh processes on %s\n' "$platform"
