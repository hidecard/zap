#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-ast-loop-try-closures.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let loop_source = "fn make_last():\n    for item in [4, 5]:\n        fn current():\n            return item\n    return current\nlet last = make_last()\nsay last()"
let loop_compiled = seed_compile_ast_source(loop_source, "ast-loop-closures.zp")
let loop_state = vm_run(loop_compiled["instructions"])
say loop_compiled["status"]
say loop_state["error"]
say loop_state["output"][0]
let try_source = "fn make_reader(value):\n    fn read():\n        try:\n            if value:\n                raise \"raised\"\n            return value\n        catch err:\n            return err\n    return read\nlet raised = make_reader(true)\nlet normal = make_reader(false)\nsay raised()\nsay normal()"
let try_compiled = seed_compile_ast_source(try_source, "ast-try-closures.zp")
let try_state = vm_run(try_compiled["instructions"])
say try_compiled["status"]
say try_state["error"]
say try_state["output"][0]
say try_state["output"][1]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["compiled_ast_slice", "none", "5", "compiled_ast_slice", "none", "raised", "false"]:
    raise SystemExit(f"unexpected loop/try closure output: {lines!r}")
PY
printf 'B4 canonical AST loop/try closure gate passed: loop capture and handler frames\n'
