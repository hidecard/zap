#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b4-ast-closure-scope.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let source = "fn choose(limit):\n    fn find():\n        for item in [1, 2, 3]:\n            if item == limit:\n                return item\n        return 0\n    return find\nfn make_reader():\n    fn read():\n        try:\n            raise \"payload\"\n        catch err:\n            return err\n    return read\nlet chosen = choose(2)\nlet reader = make_reader()\nsay chosen()\nsay reader()"
let compiled = seed_compile_ast_source(source, "ast-closure-scope.zp")
let state = vm_run(compiled["instructions"])
say compiled["status"]
say state["error"]
say state["output"][0]
say state["output"][1]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["compiled_ast_slice", "none", "2", "payload"]:
    raise SystemExit(f"unexpected closure scope output: {lines!r}")
PY
printf 'B4 canonical AST closure-scope gate passed: loop binding and catch payload scope\n'
