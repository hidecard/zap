#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-ast-mutable-closures.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let source = "fn make_counter(start):\n    fn next():\n        start = start + 1\n        return start\n    return next\nlet left = make_counter(0)\nlet right = make_counter(10)\nsay left()\nsay left()\nsay right()\nsay left()"
let compiled = seed_compile_ast_source(source, "ast-mutable-closures.zp")
let state = vm_run(compiled["instructions"])
say compiled["status"]
say state["error"]
say state["output"][0]
say state["output"][1]
say state["output"][2]
say state["output"][3]
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
if lines != ["compiled_ast_slice", "none", "1", "2", "11", "3"]:
    raise SystemExit(f"unexpected mutable closure output: {lines!r}")
PY
printf 'B4 canonical AST mutable closure gate passed: reassignment and persistent captured state\n'
