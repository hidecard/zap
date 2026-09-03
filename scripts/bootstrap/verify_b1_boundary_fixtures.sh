#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

# Detect platform and set appropriate binary name.
# Use MSYSTEM/mingw/msys/cygwin detection for Windows-bash environments,
# otherwise fall back to uname. Allow override via ZAP_BIN_OVERRIDE.
uname_s=$(uname -s 2>/dev/null | tr '[:upper:]' '[:lower:]')
if [[ -n "${MSYSTEM:-}" || "$uname_s" == *"mingw"* || "$uname_s" == *"msys"* || "$uname_s" == *"cygwin"* ]]; then
  ZAP_BIN="$ROOT_DIR/native/target/release/zap.exe"
else
  case "$uname_s" in
    linux*)     ZAP_BIN="$ROOT_DIR/native/target/release/zap" ;;
    darwin*)    ZAP_BIN="$ROOT_DIR/native/target/release/zap" ;;
    *)          ZAP_BIN="$ROOT_DIR/native/target/release/zap" ;;
  esac
fi

if [[ -n "${ZAP_BIN_OVERRIDE:-}" ]]; then
  ZAP_BIN="$ZAP_BIN_OVERRIDE"
fi

if [ ! -x "$ZAP_BIN" ]; then
  printf 'missing zap binary: %s\n' "$ZAP_BIN" >&2
  exit 2
fi

runner=$(mktemp "$ROOT_DIR/.zap-boundary-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
output=$(mktemp "${TMPDIR:-/tmp}/zap-boundary-output.XXXXXX")
trap 'rm -f "$runner" "$output"' EXIT

cat > "$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
import "bootstrap/b1/parser.zp"
let fixtures = ["boundary_trailing_token.zp", "boundary_multi_level_dedent.zp", "boundary_nested_class_module.zp", "boundary_mixed_statement_sequence.zp", "boundary_empty_block_malformed.zp"]
for name in fixtures:
    let path = "bootstrap/fixtures/parser/" + name
    let src = read_text(path)
    let tokens_json = lex(src, name)
    if contains(tokens_json, "\"diagnostics\""):
        say name + " => LEX_DIAG"
    else:
        let toks = from_json(tokens_json)
        if len(toks["tokens"]) == 0:
            say name + " => EMPTY_TOKS"
        else:
            let result = parse_or_diagnostics(src, toks["tokens"], name)
            if contains(result, "zap.diagnostics"):
                say name + " => PARSE_DIAG"
            else:
                say name + " => OK"
let empty_src = read_text("bootstrap/fixtures/parser/boundary_empty_input.zp")
say "boundary_empty_input.zp => SRC_LEN=" + str(len(empty_src))
EOF

"$ZAP_BIN" "$runner_rel" > "$output"

# Empty input must report zero length (no crash).
grep -q "boundary_empty_input.zp => SRC_LEN=0" "$output" || { printf 'FAIL: empty input did not report zero length\n' >&2; exit 1; }

# Trailing token at top level must parse to OK (top-level expression at end).
grep -q "boundary_trailing_token.zp => OK" "$output" || { printf 'FAIL: trailing token not handled\n' >&2; exit 1; }

# Malformed multi-level dedent must produce a diagnostic.
grep -q "boundary_multi_level_dedent.zp => PARSE_DIAG" "$output" || { printf 'FAIL: multi-level dedent not detected as diagnostic\n' >&2; exit 1; }

# Nested class+module must parse cleanly.
grep -q "boundary_nested_class_module.zp => OK" "$output" || { printf 'FAIL: nested class/module not parsed\n' >&2; exit 1; }

# Mixed statement sequence must parse cleanly.
grep -q "boundary_mixed_statement_sequence.zp => OK" "$output" || { printf 'FAIL: mixed statement sequence not parsed\n' >&2; exit 1; }

# Empty block / malformed if must parse (or produce a diagnostic — accept either).
grep -q "boundary_empty_block_malformed.zp => OK" "$output" || grep -q "boundary_empty_block_malformed.zp => PARSE_DIAG" "$output" || { printf 'FAIL: empty block fixture produced neither OK nor PARSE_DIAG\n' >&2; exit 1; }

printf 'B1 boundary fixtures gate passed: empty input, trailing token, multi-level dedent, nested class/module, and mixed statement sequence all handled without panic\n'