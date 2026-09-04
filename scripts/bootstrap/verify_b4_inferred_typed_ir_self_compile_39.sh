#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
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
runner=$(mktemp "$ROOT_DIR/.zap-b4-inferred-self-compile.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let source = "let value: number = 7\nsay value"
let typed = from_json(seed_emit_inferred_program_typed_ir(source, "inferred.zp"))
let artifact = seed_compile_inferred_typed_ir(typed, "inferred.zp")
let state = vm_run(artifact["instructions"])
let acceptance = seed_self_compile_acceptance(source, "inferred.zp")
say typed["schema_version"]
say typed["candidate_only"]
say typed["coverage"]
say typed["ir"]["nodes"][0]["value"]["inferred_type"]
say artifact["status"]
say state["error"]
say state["output"][0]
say acceptance["status"]
say acceptance["byte_equal"]
say acceptance["native_independent"]
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
if lines != ["4", "true", "owned_ast_with_checker_inferred_types", "number", "compiled_inferred_typed_ir_slice", "none", "7", "reproducible_inferred_typed_ir_slice", "true", "false"]:
    raise SystemExit(f"unexpected inferred self-compile output: {lines!r}")
PY
printf 'B4 inferred typed-IR self-compile gate passed: checker metadata, AST rehydration, VM handoff, deterministic rebuild\n'
