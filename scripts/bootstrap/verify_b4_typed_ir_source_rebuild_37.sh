#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-typed-ir-source.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let artifact = seed_compile_typed_ir_source("let value: number = 7\n", "typed-source.zp")
let state = vm_run(artifact["instructions"])
let rebuild = seed_self_rebuild_typed_ir("let value: number = 7\n", "typed-source.zp")
say artifact["status"]
say artifact["native_independent"]
say artifact["typed_ir"]["kind"]
say artifact["typed_ir"]["candidate_only"]
say state["error"]
say state["locals"][0]["value"]
say rebuild["status"]
say rebuild["byte_equal"]
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["compiled_typed_ir_slice", "false", "zap.typed_ir", "true", "none", "7", "reproducible_typed_ir_slice", "true"]:
    raise SystemExit(f"unexpected typed-IR source output: {lines!r}")
PY
printf 'B4 typed-IR source gate passed: Zap source to typed-IR to VM handoff and reproducible rebuild\n'
