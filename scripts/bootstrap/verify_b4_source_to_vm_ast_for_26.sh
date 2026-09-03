#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-ast-for.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let compiled = seed_compile_ast_source("let total = 0\nfor item in [1, 2, 3]:\n    let total = total + item\nsay total", "ast-for.zp")
let state = vm_run(compiled["instructions"])
say compiled["status"]
say state["error"]
say state["output"][0]
let unsupported = seed_compile_ast_source("for item in source:\n    say item", "ast-for-unsupported.zp")
say unsupported["error"]
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
if lines != ["compiled_ast_slice", "none", "6", "unsupported_ast_for_iterable"]:
    raise SystemExit(f"unexpected AST for output: {lines!r}")
PY
printf 'B4 canonical AST for gate passed: literal unrolling and bounded rejection\n'
