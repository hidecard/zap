#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b4-ast-recursive-frames.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let source = "fn fact(n):\n    if n == 0:\n        return 1\n    let next = n - 1\n    return n * fact(next)\nfn make_fact():\n    fn inner(n):\n        if n == 0:\n            return 1\n        let next = n - 1\n        return n * inner(next)\n    return inner\nlet recursive = make_fact()\nsay fact(5)\nsay recursive(5)"
let compiled = seed_compile_ast_source(source, "ast-recursive-frames.zp")
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
if lines != ["compiled_ast_slice", "none", "120", "120"]:
    raise SystemExit(f"unexpected recursive frame output: {lines!r}")
PY
printf 'B4 canonical AST recursive-frame gate passed: function fallback and self-binding\n'
