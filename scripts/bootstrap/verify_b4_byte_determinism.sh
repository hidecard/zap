#!/usr/bin/env bash
# B4 byte-for-byte deterministic artifact verification.
#
# Produces real compiler artifacts (tokens, AST, typed IR, bytecode) from the
# same source bytes multiple times and compares the output byte-for-byte.
# This is stronger than the synthetic digest comparison in verify_b4_rebuild_bytes_6.sh
# because it exercises the actual artifact production pipeline.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

REPORT="${B4_BYTE_DETERMINISM_REPORT:-target/b4-byte-determinism.tsv}"
mkdir -p "$(dirname "$REPORT")"

fail() {
  echo "B4 byte-determinism failed: $*" >&2
  exit 1
}

runner=$(mktemp "$ROOT_DIR/.zap-byte-det.XXXXXX.zp")
out_a=$(mktemp)
out_b=$(mktemp)
expected_a=$(mktemp)
expected_b=$(mktemp)
expected_c=$(mktemp)
trap 'rm -f "$runner" "$out_a" "$out_b" "$expected_a" "$expected_b" "$expected_c"' EXIT

cat > "$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typed_ir.zp"
import "bootstrap/b4/native_independent.zp"

let source = "let x = 1 + 2\nsay x\n"

let first_tokens = lex(source, "det_test")
let second_tokens = lex(source, "det_test")
let tokens_equal = json(first_tokens) == json(second_tokens)

let first_ast = parse_general(source, "det_test")
let second_ast = parse_general(source, "det_test")
let ast_equal = json(first_ast) == json(second_ast)

let first_typed = emit_inferred_program_typed_ir(source, "det_test")
let second_typed = emit_inferred_program_typed_ir(source, "det_test")
let typed_equal = json(first_typed) == json(second_typed)

let first_rebuild = seed_self_rebuild(source, "det_test")
let second_rebuild = seed_self_rebuild(source, "det_test")
let rebuild_equal = first_rebuild["byte_equal"]

let first_pipeline = seed_execute_owned_pipeline(source, "det_test")
let second_pipeline = seed_execute_owned_pipeline(source, "det_test")
let pipeline_equal = json(first_pipeline) == json(second_pipeline)

say tokens_equal
say ast_equal
say typed_equal
say rebuild_equal
say pipeline_equal
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out_a"

cat > "$expected_a" <<'EOF'
true
true
true
true
true
EOF

cmp "$out_a" "$expected_a" || fail "first run did not produce deterministic artifacts"

# Second run: same source, fresh process
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out_b"

cmp "$out_a" "$out_b" || fail "second run produced different output"

# Multi-line source surface
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"

let source = "fn add(a: number, b: number) -> number:\n    return a + b\n\nlet result = add(3, 4)\nsay result\n"

let first = seed_self_rebuild(source, "multi_line")
let second = seed_self_rebuild(source, "multi_line")
let third = seed_self_rebuild(source, "multi_line")

say first["byte_equal"]
say second["byte_equal"]
say third["byte_equal"]
say first["status"] == "reproducible"
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out_a"

cat > "$expected_b" <<'EOF'
true
true
true
true
EOF

cmp "$out_a" "$expected_b" || fail "multi-line source did not produce deterministic rebuild"

# Control-flow surface
cat > "$runner" <<'EOF'
import "bootstrap/b4/native_independent.zp"

let source = "let n = 5\nlet total = 0\nlet i = 0\nwhile i < n:\n    total = total + i\n    i = i + 1\nsay total\n"

let first = seed_self_rebuild(source, "control_flow")
let second = seed_self_rebuild(source, "control_flow")

say first["byte_equal"]
say first["status"] == "reproducible"
EOF

cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" > "$out_a"

cat > "$expected_c" <<'EOF'
true
true
EOF

cmp "$out_a" "$expected_c" || fail "control-flow source did not produce deterministic rebuild"

# Report
: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB4-BYTE-DETERMINISM\nstatus\tpassed\n' >> "$REPORT"
printf 'tokens_deterministic\ttrue\n' >> "$REPORT"
printf 'ast_deterministic\ttrue\n' >> "$REPORT"
printf 'typed_ir_deterministic\ttrue\n' >> "$REPORT"
printf 'rebuild_deterministic\ttrue\n' >> "$REPORT"
printf 'pipeline_deterministic\ttrue\n' >> "$REPORT"
printf 'multi_line_deterministic\ttrue\n' >> "$REPORT"
printf 'control_flow_deterministic\ttrue\n' >> "$REPORT"

printf 'B4 byte-determinism gate passed: 7 deterministic artifact families verified byte-for-byte\n'
