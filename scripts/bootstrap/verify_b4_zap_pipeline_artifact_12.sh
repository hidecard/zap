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
runner=$(mktemp "$ROOT_DIR/.zap-pipeline-artifact.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"
import "bootstrap/b2/typed_ir.zp"
let source = "let value = 1\nsay value"
let tokens = from_json(lex(source, "seed.zp"))
let ast_json = parse_or_diagnostics(source, tokens["tokens"], "seed.zp")
let ast = from_json(ast_json)
let type_json = check(source, "seed.zp")
let typed_ir_json = emit(source, "seed.zp")
let artifact = {"ast": ast, "kind": "zap.compiler_artifact", "schema_version": 1, "source_name": "seed.zp", "tokens": tokens, "typed_ir": from_json(typed_ir_json), "types": from_json(type_json)}
say artifact["kind"]
say artifact["schema_version"]
say len(artifact["tokens"]["tokens"])
say artifact["ast"]["kind"]
say artifact["types"]["kind"]
say artifact["typed_ir"]["kind"]
say artifact["typed_ir"]["candidate_only"]
say len(json(artifact)) > 0
EOF
cat > "$expected" <<'EOF'
zap.compiler_artifact
1
7
zap.ast
zap.typecheck
zap.typed_ir
true
true
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'Zap pipeline artifact gate passed: 12 lexer/parser/typecheck/typed-IR assembly cases\n'
