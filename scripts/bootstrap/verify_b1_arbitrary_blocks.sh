#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

for fixture in \
  bootstrap/fixtures/parser/while_else_syntax.zp \
  bootstrap/fixtures/parser/invalid_indentation_jump.zp \
  bootstrap/fixtures/parser/while_without_else.zp \
  bootstrap/fixtures/parser/mixed_top_level_statements.zp \
  bootstrap/fixtures/parser/arbitrary_deep_indentation.zp \
  bootstrap/fixtures/parser/arbitrary_nested_blocks_complex.zp; do
  [[ -f "$fixture" ]] || { printf 'missing fixture: %s\n' "$fixture" >&2; exit 2; }
done

runner=$(mktemp "$ROOT_DIR/.zap-arbitrary-blocks-runner.XXXXXX.zp")
output=$(mktemp "${TMPDIR:-/tmp}/zap-arbitrary-blocks-output.XXXXXX")
trap 'rm -f "$runner" "$output"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"

export fn report(source_name):
    let src = read_text("bootstrap/fixtures/parser/" + source_name)
    let toks = from_json(lex(src, source_name))
    let result = parse_or_diagnostics(src, toks["tokens"], source_name)
    if contains(result, "zap.diagnostics"):
        say source_name + " => DIAGNOSTIC:" + result
    else:
        say source_name + " => AST"
    return none

report("while_else_syntax.zp")
report("invalid_indentation_jump.zp")
report("while_without_else.zp")
report("mixed_top_level_statements.zp")
report("arbitrary_deep_indentation.zp")
report("arbitrary_nested_blocks_complex.zp")
EOF

ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi > "$output"

# while ... else is explicitly unsupported -> diagnostic with the dedicated message.
grep -q "while_else_syntax.zp => DIAGNOSTIC" "$output" || { printf 'FAIL: while_else_syntax did not produce a diagnostic\n' >&2; exit 1; }
grep -q "unsupported 'while ... else' syntax" "$output" || { printf 'FAIL: while_else_syntax missing unsupported-syntax message\n' >&2; exit 1; }

# invalid indentation jump -> diagnostic reporting unexpected indentation.
grep -q "invalid_indentation_jump.zp => DIAGNOSTIC" "$output" || { printf 'FAIL: invalid_indentation_jump did not produce a diagnostic\n' >&2; exit 1; }
grep -q "unexpected indentation" "$output" || { printf 'FAIL: invalid_indentation_jump missing unexpected-indentation message\n' >&2; exit 1; }

# Valid arbitrary programs must parse to an AST (no diagnostic).
for valid in while_without_else mixed_top_level_statements arbitrary_deep_indentation arbitrary_nested_blocks_complex; do
  grep -q "${valid}.zp => AST" "$output" || { printf 'FAIL: %s expected to parse to AST\n' "$valid" >&2; exit 1; }
  grep -q "${valid}.zp => DIAGNOSTIC" "$output" && { printf 'FAIL: %s unexpectedly produced a diagnostic\n' "$valid" >&2; exit 1; }
done

printf 'B1 arbitrary-block candidate gate passed: while-else unsupported, invalid-indentation jump rejected, and arbitrary-depth/mixed-top-level programs parse\n'
