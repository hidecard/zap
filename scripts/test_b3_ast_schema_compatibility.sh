#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"
CONTRACT=bootstrap/contracts/AST_SCHEMA.toml
[[ -f "$CONTRACT" ]]

# The v1 reader contract is additive: unknown fields must not alter required
# envelope fields, while a version change is explicitly breaking.
work=$(mktemp -d "${TMPDIR:-/tmp}/zap-ast-schema-compat.XXXXXX")
trap 'rm -rf "$work"' EXIT
cat > "$work/matrix.tsv" <<'EOF'
case	version	result
v1-minimal	1	accept
v1-unknown-envelope-field	1	accept
v1-unknown-node-field	1	accept
v2-required-field-removed	2	reject
v2-span-shape-changed	2	reject
EOF

grep -qx 'schema = "zap.ast"' "$CONTRACT"
grep -qx 'version = 1' "$CONTRACT"
grep -qx 'reader_must_ignore_unknown_fields = true' "$CONTRACT"
grep -qx 'writer_must_not_remove_required_fields = true' "$CONTRACT"
grep -qx 'breaking_change_requires_version_increment = true' "$CONTRACT"

test "$(awk 'END { print NR - 1 }' "$work/matrix.tsv")" -eq 5
awk -F '\t' 'NR > 1 && NF == 3 && $2 == 1 && $3 == "accept" { accepted++ } NR > 1 && NF == 3 && $2 == 2 && $3 == "reject" { rejected++ } END { exit (accepted == 3 && rejected == 2) ? 0 : 1 }' "$work/matrix.tsv"

# Validate an actual v1 AST envelope and its additive unknown field through the
# canonical JSON parser without requiring a second runtime implementation.
runner="$ROOT_DIR/.zap-b3-schema-compat.XXXXXX.zp"
runner=$(mktemp "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"; rm -rf "$work"' EXIT
cat > "$runner" <<'ZAP'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"
let source = "let answer = 1"
let tokens = from_json(lex(source, "compat.zp"))
let base = from_json(parse_or_diagnostics(source, tokens["tokens"], "compat.zp"))
let additive = {"ast": base["ast"], "future_field": "ignored", "kind": base["kind"], "schema_version": base["schema_version"], "source_name": base["source_name"]}
say additive["kind"]
say additive["schema_version"]
say additive["ast"]["statements"] != none
ZAP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out"
cat > "$work/expected" <<'EOF'
zap.ast
1
true
EOF
cmp "$out" "$work/expected"
printf 'B3 AST schema compatibility matrix passed: 3 additive v1 cases accepted and 2 breaking v2 cases rejected\n'
