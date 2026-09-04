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
runner=$(mktemp "$ROOT_DIR/.zap-rebuild-bytes.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/rebuild.zp"
let lexer = rebuild_stage("lexer", "source", "tokens", "zap")
let parser = rebuild_stage("parser", "tokens", "ast", "zap")
let plan = rebuild_plan("platform-seed-0", [lexer, parser])
let first = rebuild_manifest(plan, [rebuild_artifact(plan, "lexer", "a1"), rebuild_artifact(plan, "parser", "b1")])
let second = rebuild_manifest(plan, [rebuild_artifact(plan, "lexer", "a1"), rebuild_artifact(plan, "parser", "b1")])
let changed = rebuild_manifest(plan, [rebuild_artifact(plan, "lexer", "a2"), rebuild_artifact(plan, "parser", "b1")])
say rebuild_is_reproducible(first, second)
say rebuild_bytes_reproducible(first, second)
say rebuild_is_reproducible(first, changed)
say rebuild_bytes_reproducible(first, changed)
say len(first["artifacts"])
say first["seed"]
EOF
cat > "$expected" <<'EOF'
true
true
false
false
2
platform-seed-0
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  run_zap "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B4 rebuild-byte gate passed: 6 deterministic manifest and byte-comparison cases\n'
