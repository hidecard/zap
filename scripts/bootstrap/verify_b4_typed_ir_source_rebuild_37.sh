#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
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
runner=$(mktemp "$ROOT_DIR/.zap-b4-typed-ir-source.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
let artifact = driver_compile_backend("let value: number = 7\n", "typed-source.zp")
let repeat = driver_compile_backend("let value: number = 7\n", "typed-source.zp")
let state = vm_run(artifact["bytecode"]["instructions"])
let rebuild = driver_rebuild("let value: number = 7\n", "typed-source.zp")
let semantics = driver_typed_ir_semantics(artifact["typed_ir"], repeat["typed_ir"])
say artifact["status"]
say artifact["bytecode"]["native_independent"]
say artifact["typed_ir"]["kind"]
say artifact["typed_ir"]["candidate_only"]
say state["error"]
say state["locals"][0]["value"]
say rebuild["status"]
say rebuild["byte_equal"]
say semantics["status"]
say semantics["valid"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["ok", "false", "zap.typed_ir", "false", "none", "7", "candidate_driver_rebuild", "true", "zap_owned_typed_ir_semantics", "true"]:
    raise SystemExit(f"unexpected typed-IR source output: {lines!r}")
PY
printf 'B4 typed-IR source gate passed: Zap source to typed-IR to VM handoff and reproducible rebuild\n'
