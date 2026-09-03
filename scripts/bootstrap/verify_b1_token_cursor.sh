#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-token-cursor-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
expected=$(mktemp)
trap 'rm -f "$runner" "$out" "$expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/token_cursor.zp"
let c0 = cursor_new([{"kind": "name", "span": {"line": 2, "column": 5}}])
say cursor_done(c0)
say cursor_peek_kind(c0)
let c1 = cursor_advance(c0)
say cursor_done(c1)
say indentation_relation(0, 4)
say indentation_relation(4, 0)
say indentation_relation(4, 8)
say indentation_relation(8, 2)
EOF
cat > "$expected" <<'EOF'
false
name
true
nested
dedent
nested
dedent
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$out"
cmp "$out" "$expected"
printf 'B1 token cursor foundation gate passed: immutable cursor and indentation relation matrix\n'
