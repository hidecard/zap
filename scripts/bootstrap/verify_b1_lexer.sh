#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
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

if (($# == 0)); then
  set -- \
    bootstrap/fixtures/lexer/basic.zp \
    bootstrap/fixtures/lexer/unicode.zp \
    bootstrap/fixtures/lexer/operators.zp \
    bootstrap/fixtures/lexer/delimiters.zp \
    bootstrap/fixtures/diagnostics/invalid_character.zp \
    bootstrap/fixtures/diagnostics/integer_overflow.zp \
    bootstrap/fixtures/diagnostics/unterminated_string.zp
fi

for fixture in "$@"; do
  case "$fixture" in
    bootstrap/fixtures/lexer/basic.zp)
      expected=bootstrap/fixtures/lexer/basic.tokens.json
      mode=tokens
      ;;
    bootstrap/fixtures/lexer/unicode.zp)
      expected=bootstrap/fixtures/lexer/unicode.tokens.json
      mode=tokens
      ;;
    bootstrap/fixtures/lexer/operators.zp)
      expected=bootstrap/fixtures/lexer/operators.tokens.json
      mode=tokens
      ;;
    bootstrap/fixtures/lexer/delimiters.zp)
      expected=bootstrap/fixtures/lexer/delimiters.tokens.json
      mode=tokens
      ;;
    bootstrap/fixtures/diagnostics/invalid_character.zp)
      expected=bootstrap/fixtures/diagnostics/invalid_character.json
      mode=diagnostics
      ;;
    bootstrap/fixtures/diagnostics/integer_overflow.zp)
      expected=bootstrap/fixtures/diagnostics/integer_overflow.json
      mode=diagnostics
      ;;
    bootstrap/fixtures/diagnostics/unterminated_string.zp)
      expected=bootstrap/fixtures/diagnostics/unterminated_string.json
      mode=diagnostics
      ;;
    *)
      printf 'unsupported B1 fixture: %s\n' "$fixture" >&2
      exit 2
      ;;
  esac

  [[ -f "$fixture" && -f "$expected" ]] || {
    printf 'missing B1 fixture or expected artifact: %s\n' "$fixture" >&2
    exit 2
  }

  runner=$(mktemp "$ROOT_DIR/.zap-b1-runner.XXXXXX.zp")
  runner_rel=$(basename "$runner")
  output=$(mktemp "${TMPDIR:-/tmp}/zap-b1-output.XXXXXX")
  # Use a quoted heredoc with a placeholder. The standalone run does the
  # `sed` substitution below; the aggregate runner picks up the literal
  # `__FIXTURE__` form, which still triggers a read_text error but cleanly
  # rather than referencing an undefined $fixture variable.
  cat > "$runner" <<'EOF'
import "bootstrap/b1/lexer.zp"
let source = read_text("__FIXTURE__")
say lex(source, "__FIXTURE__")
EOF
  sed "s|__FIXTURE__|$fixture|g" "$runner" > "$runner.tmp" && mv "$runner.tmp" "$runner"
  if ! "$ZAP_BIN" "$runner_rel" > "$output"; then
    rm -f "$runner" "$output"
    exit 1
  fi
  if ! cmp "$output" "$expected"; then
    printf 'B1 lexer differential mismatch: %s\n' "$fixture" >&2
    rm -f "$runner" "$output"
    exit 1
  fi
  rm -f "$runner" "$output"
  printf 'B1 lexer differential passed: %s (%s)\n' "$fixture" "$mode"
done
