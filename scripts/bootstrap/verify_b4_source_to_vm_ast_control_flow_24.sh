#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-ast-control.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let compiled = seed_compile_ast_source("let i = 0\nwhile i < 4:\n    if i == 1:\n        let i = i + 1\n        continue\n    say i\n    let i = i + 1\n    if i == 3:\n        break\nsay 9", "ast-control.zp")
let state = vm_run(compiled["instructions"])
say compiled["status"]
say state["error"]
say state["output"][0]
say state["output"][1]
say state["output"][2]
let nested = seed_compile_ast_source("let outer = 0\nlet hits = 0\nwhile outer < 2:\n    let inner = 0\n    while inner < 3:\n        let inner = inner + 1\n        if inner == 2:\n            break\n        let hits = hits + 1\n    let outer = outer + 1\nsay hits", "ast-nested-control.zp")
let nested_state = vm_run(nested["instructions"])
say nested["status"]
say nested_state["error"]
say nested_state["output"][0]
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
if lines != ["compiled_ast_slice", "none", "0", "2", "9", "compiled_ast_slice", "none", "2"]:
    raise SystemExit(f"unexpected AST control-flow output: {lines!r}")
PY
printf 'B4 canonical AST control-flow gate passed: nested if/while, nearest-loop break, and continue\n'
