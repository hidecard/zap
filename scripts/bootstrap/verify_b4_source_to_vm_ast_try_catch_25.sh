#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b4-ast-try.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b3/vm.zp"
let raised = seed_compile_ast_source("try:\n    raise \"boom\"\ncatch err:\n    say err", "ast-try-error.zp")
let raised_state = vm_run(raised["instructions"])
say raised["status"]
say raised_state["error"]
say raised_state["output"][0]
let normal = seed_compile_ast_source("try:\n    say 7\ncatch err:\n    say 8", "ast-try-normal.zp")
let normal_state = vm_run(normal["instructions"])
say normal_state["error"]
say normal_state["output"][0]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["compiled_ast_slice", "none", "boom", "none", "7"]:
    raise SystemExit(f"unexpected AST try/catch output: {lines!r}")
PY
printf 'B4 canonical AST try/catch gate passed: binding, raise recovery, and normal path\n'
