#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
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
runner=$(mktemp "$ROOT_DIR/.zap-b4-rebuild.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/rebuild.zp"
let seed = "platform-seed-0"
let lexer = rebuild_stage("lexer", "source", "tokens", "zap")
let parser = rebuild_stage("parser", "tokens", "ast", "zap")
let typer = rebuild_stage("typecheck", "ast", "typed_ir", "zap")
let vm = rebuild_stage("vm", "typed_ir", "bytecode_result", "zap")
let plan = rebuild_plan(seed, [lexer, parser, typer, vm])
let a = rebuild_artifact(plan, "lexer", "a1")
let b = rebuild_artifact(plan, "parser", "b1")
let c = rebuild_artifact(plan, "typecheck", "c1")
let d = rebuild_artifact(plan, "vm", "d1")
let manifest = rebuild_manifest(plan, [a, b, c, d])
let acceptance = rebuild_acceptance(plan, seed, "B4")
say plan["compiler_stage"]
say plan["network"]
say plan["version"]
say len(plan["stages"])
say lexer["owner"]
say parser["output_kind"]
say typer["input_kind"]
say vm["name"]
say a["digest"]
say d["stage"]
say len(manifest["artifacts"])
say rebuild_is_reproducible(manifest, manifest)
say acceptance["seed_matches"]
say acceptance["stage_matches"]
say acceptance["native_independent"]
say acceptance["all_zap_owned"]
say acceptance["native_owner_allowed"]
EOF
cat > "$expected" <<'EOF'
B4
false
1
4
zap
ast
ast
vm
a1
vm
4
true
true
true
false
true
false
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 rebuild-plan gate passed: 17 deterministic orchestration and ownership-policy cases; native-independent flag remains false\n'
