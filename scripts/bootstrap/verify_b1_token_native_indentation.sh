#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

# Detect platform and set appropriate binary name
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then
    "$ROOT_DIR/native/target/release/zap.exe" "$@"
  else
    printf 'missing zap binary\n' >&2
    exit 2
  fi
}

for fixture in \
  bootstrap/fixtures/parser/arbitrary_deep_indentation.zp \
  bootstrap/fixtures/parser/arbitrary_nested_blocks_complex.zp \
  bootstrap/fixtures/parser/mixed_top_level_statements.zp \
  bootstrap/fixtures/parser/invalid_indentation_jump.zp \
  bootstrap/fixtures/parser/while_without_else.zp \
  bootstrap/fixtures/parser/while_else_syntax.zp; do
  [[ -f "$fixture" ]] || { printf 'missing fixture: %s\n' "$fixture" >&2; exit 2; }
done

runner=$(mktemp "$ROOT_DIR/.zap-token-native-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
output=$(mktemp "${TMPDIR:-/tmp}/zap-token-native-output.XXXXXX")
trap 'rm -f "$runner" "$output"' EXIT

cat > "$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"

export fn report(source_name):
    let src = read_text("bootstrap/fixtures/parser/" + source_name)
    let toks = from_json(lex(src, source_name))
    let table = token_indentation_table(toks["tokens"], 0, [])
    let lines = split(trim(src), "\n")
    let statements = parse_general_program_tokens(lines, table)
    let result = parse_general(src, source_name)
    if contains(result, "zap.diagnostics"):
        say source_name + " => DIAGNOSTIC:" + result
    else:
        say source_name + " => AST:" + str(len(statements))
    return none

report("arbitrary_deep_indentation.zp")
report("arbitrary_nested_blocks_complex.zp")
report("mixed_top_level_statements.zp")
report("invalid_indentation_jump.zp")
report("while_without_else.zp")
report("while_else_syntax.zp")
EOF

run_zap "$runner_rel" > "$output"

# Valid arbitrary programs must parse to an AST through the token-native path.
for valid in arbitrary_deep_indentation arbitrary_nested_blocks_complex mixed_top_level_statements while_without_else; do
  grep -q "${valid}.zp => AST:" "$output" || { printf 'FAIL: %s expected to parse to AST via token-native path\n' "$valid" >&2; exit 1; }
  grep -q "${valid}.zp => DIAGNOSTIC" "$output" && { printf 'FAIL: %s unexpectedly produced a diagnostic\n' "$valid" >&2; exit 1; }
done

# Invalid indentation jump -> diagnostic reporting unexpected indentation.
grep -q "invalid_indentation_jump.zp => DIAGNOSTIC" "$output" || { printf 'FAIL: invalid_indentation_jump did not produce a diagnostic\n' >&2; exit 1; }
grep -q "unexpected indentation" "$output" || { printf 'FAIL: invalid_indentation_jump missing unexpected-indentation message\n' >&2; exit 1; }

# while ... else is explicitly unsupported -> diagnostic with the dedicated message.
grep -q "while_else_syntax.zp => DIAGNOSTIC" "$output" || { printf 'FAIL: while_else_syntax did not produce a diagnostic\n' >&2; exit 1; }
grep -q "unsupported 'while ... else' syntax" "$output" || { printf 'FAIL: while_else_syntax missing unsupported-syntax message\n' >&2; exit 1; }

printf 'B1 token-native indentation gate passed: parse_general_program_tokens assembles arbitrary-depth blocks from token spans and rejects invalid indentation\n'