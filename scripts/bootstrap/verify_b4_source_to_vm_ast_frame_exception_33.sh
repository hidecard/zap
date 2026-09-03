#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-ast-frame-exception.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let source = "fn make_failure():\n    fn fail():\n        raise \"from-callee\"\n    return fail\nfn recover():\n    let fail = make_failure()\n    try:\n        fail()\n    catch err:\n        return err\nsay recover()"
let compiled = seed_compile_ast_source(source, "ast-frame-exception.zp")
let state = vm_run(compiled["instructions"])
say compiled["status"]
say state["error"]
say state["output"][0]
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
if lines != ["compiled_ast_slice", "none", "from-callee"]:
    raise SystemExit(f"unexpected frame exception output: {lines!r}")
PY
printf 'B4 canonical AST frame-exception gate passed: callee raise reaches caller catch\n'
