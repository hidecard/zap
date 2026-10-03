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
let test_bytecode = [{"op": "const", "value": 42}]
let result = c_backend_generate(test_bytecode)
say "Generated C lines: " + str(len(result))
say "First line: " + result[0]
EOF

ZAP_TEST_FILE="$TEST_FILE"

if output=$("$ZAP_BIN" "$ZAP_TEST_FILE" 2>&1); then
    echo "PASS: Zap C backend module compiles successfully"
else
    printf '%s\n' "$output" >&2
    echo "FAIL: Zap C backend module compilation failed"
    exit 1
fi

echo "Zap-owned C backend structure verification passed"
