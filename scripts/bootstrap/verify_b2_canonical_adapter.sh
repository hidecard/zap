#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "${BASH_SOURCE[0]%/*}/../.." && pwd)"
cd "$ROOT_DIR"

runner=$(mktemp "$ROOT_DIR/.zap-b2-canonical-runner.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT

cat > "$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"

let valid = from_json(parse("let value: option<number> = some(1)\nif is_some(value):\n    let inside: number = value\n", "canonical-valid.zp"))
let invalid = from_json(parse("let value: number = \"bad\"\n", "canonical-invalid.zp"))
say json(check_ast_complete(valid["ast"], "canonical-valid.zp"))
say json(check_ast_complete(invalid["ast"], "canonical-invalid.zp"))
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
sed -n '1p' "$out" | jq -e '(.kind == "zap.typecheck") and (.ok == true) and (.schema_version == 1) and ((.diagnostics | length) == 0)' >/dev/null
sed -n '2p' "$out" | jq -e '(.kind == "zap.typecheck") and (.ok == false) and (.schema_version == 1) and (.diagnostics[0].code == "ZAP-TYPE-001") and (.diagnostics[0].kind == "TypeError") and (.diagnostics[0].message | contains("expects number, got text"))' >/dev/null
printf 'B2 canonical adapter passed: parser AST -> check_ast_complete -> diagnostics envelope\n'
