#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-self-rebuild-ast.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/rebuild.zp"
import "bootstrap/b4/runner.zp"
import "bootstrap/b4/native_independent.zp"
let lexer = rebuild_stage("lexer", "source", "tokens", "zap")
let parser = rebuild_stage("parser", "tokens", "ast", "zap")
let lower = rebuild_stage("lower", "ast", "bytecode", "zap")
let plan = rebuild_plan("platform-seed-0", [lexer, parser, lower])
let acceptance = runner_acceptance(plan)
let execution = runner_execute_plan(plan, "source", "source-digest", ["tokens-digest", "ast-digest", "bytecode-digest"])
let rebuild = seed_rebuild_acceptance_ast("fn add(a, b):\n    return a + b\nsay add(2, 3)", "self-rebuild.zp")
say acceptance["executable"]
say acceptance["native_independent"]
say execution["status"]
say execution["final_kind"]
say rebuild["status"]
say rebuild["byte_equal"]
say rebuild["supported"]
say rebuild["native_independent"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["true", "false", "executed", "bytecode", "reproducible_ast_slice", "true", "true", "false"]:
    raise SystemExit(f"unexpected self-rebuild output: {lines!r}")
PY
printf 'B4 self-rebuild gate passed: executable Zap-owned plan and reproducible canonical AST rebuild\n'
