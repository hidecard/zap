#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-ast-nested-frames.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let source = "fn make_chain(a):\n    fn middle(b):\n        fn inner(c):\n            return a + c\n        return inner\n    return middle\nfn choose(flag):\n    if flag:\n        fn yes():\n            return 7\n        return yes\n    fn no():\n        return 9\n    return no\nlet middle = make_chain(1)\nlet inner = middle(2)\nlet yes = choose(true)\nlet no = choose(false)\nsay inner(3)\nsay yes()\nsay no()"
let compiled = seed_compile_ast_source(source, "ast-nested-frames.zp")
let state = vm_run(compiled["instructions"])
say compiled["status"]
say state["error"]
say state["output"][0]
say state["output"][1]
say state["output"][2]
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
if lines != ["compiled_ast_slice", "none", "4", "7", "9"]:
    raise SystemExit(f"unexpected nested frame output: {lines!r}")
PY
printf 'B4 canonical AST nested-frame gate passed: transitive captures and conditional definitions\n'
