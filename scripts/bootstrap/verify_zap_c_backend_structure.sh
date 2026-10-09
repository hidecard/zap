#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "Verifying Zap-owned C backend structure..."

# Check if the Zap C backend module exists
if [ ! -f "bootstrap/b4/c_backend.zp" ]; then
    echo "FAIL: Zap C backend module not found"
    exit 1
fi

echo "PASS: Zap C backend module exists"

for required in \
    'export fn c_backend_validate_bytecode(' \
    'export fn c_backend_emit(' \
    'export fn c_backend_ownership(' \
    'zap_owned_scalar_backend'; do
    if ! grep -Fq "$required" bootstrap/b4/c_backend.zp; then
        echo "FAIL: Zap C backend missing required ownership slice: $required"
        exit 1
    fi
done

echo "PASS: Zap C backend validation and ownership slice present"

# Try to compile the Zap C backend module with the current Zap binary
ZAP_BIN="${ZAP_BIN:-${ZAP_BOOTSTRAP_BIN:-$ROOT_DIR/bin/zap}}"
if [ ! -x "$ZAP_BIN" ] && [ -x "$ZAP_BIN.exe" ]; then
    ZAP_BIN="$ZAP_BIN.exe"
fi
if [ ! -x "$ZAP_BIN" ]; then
    echo "SKIP: No Zap binary available for compilation test"
    exit 0
fi

# Create a simple test that imports the C backend module
TEST_FILE=$(mktemp ".zap-c-backend.XXXXXX.zp")
trap 'rm -f "$TEST_FILE"' EXIT

cat > "$TEST_FILE" <<'EOF'
import "bootstrap/b4/c_backend.zp"
let test_bytecode = [{"op": "const", "value": true}, {"op": "dup"}, {"op": "const", "value": false}, {"op": "and"}, {"op": "const", "value": false}, {"op": "not"}, {"op": "or"}, {"op": "halt"}]
let result = c_backend_generate(test_bytecode)
let unknown = c_backend_validate_bytecode([{"op": "future_opcode"}])
let malformed = c_backend_validate_bytecode([{"op": "const"}])
say "Generated C lines: " + str(len(result))
say "First line: " + result[0]
say "Has halt: " + str(contains(result, "  return 0;"))
say "Has dup: " + str(contains(result, "  { long value = zap_pop(); zap_push(value); zap_push(value); }"))
say "Has and: " + str(contains(result, "  { long rhs = zap_pop(); long lhs = zap_pop(); zap_push(lhs && rhs); }"))
say "Has not: " + str(contains(result, "  { long value = zap_pop(); zap_push(!value); }"))
say "Has or: " + str(contains(result, "  { long rhs = zap_pop(); long lhs = zap_pop(); zap_push(lhs || rhs); }"))
say "Rejects unknown: " + str(unknown["status"] == "rejected")
say "Rejects malformed const: " + str(malformed["status"] == "rejected")
EOF

ZAP_TEST_FILE="$TEST_FILE"

if output=$("$ZAP_BIN" "$TEST_FILE" 2>&1); then
    printf '%s\n' "$output"
    for expected in 'Has halt: true' 'Has dup: true' 'Has and: true' 'Has not: true' 'Has or: true' 'Rejects unknown: true' 'Rejects malformed const: true'; do
        if ! grep -Fq "$expected" <<<"$output"; then
            echo "FAIL: C backend boolean emission check missing: $expected"
            exit 1
        fi
    done
    echo "PASS: Zap C backend module compiles successfully"
else
    printf '%s\n' "$output" >&2
    echo "FAIL: Zap C backend module compilation failed"
    exit 1
fi

echo "Zap-owned C backend structure verification passed"
