#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
for path in bootstrap/b2/typecheck.zp bootstrap/fixtures/typecheck/annotated.zp bootstrap/fixtures/typecheck/conditional.zp bootstrap/fixtures/typecheck/incompatible.zp; do
  [[ -f "$path" ]] || { printf 'missing B2 candidate fixture: %s\n' "$path" >&2; exit 2; }
done
runner=$(mktemp "$ROOT_DIR/.zap-b2-typecheck-candidate-runner.XXXXXX.zp")
first=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typecheck-candidate-first.XXXXXX")
second=$(mktemp "${TMPDIR:-/tmp}/zap-b2-typecheck-candidate-second.XXXXXX")
trap 'rm -f "$runner" "$first" "$second"' EXIT
cat > "$runner" <<'EOF_RUNNER'
import "bootstrap/b2/typecheck.zp"
let annotated = read_text("bootstrap/fixtures/typecheck/annotated.zp")
let conditional = read_text("bootstrap/fixtures/typecheck/conditional.zp")
let incompatible = read_text("bootstrap/fixtures/typecheck/incompatible.zp")
say check(annotated, "bootstrap/fixtures/typecheck/annotated.zp")
say check(conditional, "bootstrap/fixtures/typecheck/conditional.zp")
say check(incompatible, "bootstrap/fixtures/typecheck/incompatible.zp")
EOF_RUNNER
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$first"
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$second"
cmp "$first" "$second"
[[ "$(wc -l < "$first")" -eq 3 ]] || { printf 'unexpected B2 candidate output line count\n' >&2; exit 1; }
sed -n '1p' "$first" | jq -e '.kind == "zap.typecheck" and .ok == true and .schema_version == 1 and (.diagnostics | length) == 0' >/dev/null
sed -n '2p' "$first" | jq -e '.kind == "zap.typecheck" and .ok == true and .schema_version == 1 and (.diagnostics | length) == 0' >/dev/null
sed -n '3p' "$first" | jq -e '.kind == "zap.typecheck" and .ok == false and .schema_version == 1 and .diagnostics[0].code == "ZAP-TYPE-001" and .diagnostics[0].kind == "TypeError" and .diagnostics[0].line == 1 and .diagnostics[0].column == 1 and (.diagnostics[0].message | contains("expects number, got text"))' >/dev/null
printf 'B2 Zap type-checker candidate differential semantics passed: annotated, conditional, incompatible\n'
