#!/usr/bin/env bash
# Verify the Zap-owned source -> typed-IR -> bytecode -> C path.
# This is an ownership slice, not full B4 certification.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "source-to-C ownership failed: $*" >&2; exit 1; }
DRIVER="bootstrap/b4/compiler_driver.zp"
SEED="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN:-$ROOT_DIR/bin/zap}}"
[[ -f "$DRIVER" ]] || fail "missing $DRIVER"
[[ -x "$SEED" ]] || fail "verified Zap seed required; set ZAP_BOOTSTRAP_BIN"
grep -Fq 'export fn driver_compile_c_source(' "$DRIVER" || fail "missing source-to-C driver export"
grep -Fq 'driver_compile_c_backend(lowered["instructions"])' "$DRIVER" || fail "source-to-C path does not call Zap C backend"
if grep -n -E '\b(cargo|rustc|rustup)\b|host/zap-host|native/src' "$DRIVER"; then
  fail "driver contains a forbidden Rust/native fallback"
fi

runner=$(mktemp "$ROOT_DIR/.zap-source-to-c.XXXXXX.zp")
output=$(mktemp)
trap 'rm -f "$runner" "$output" "${output}.expected"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
let result = driver_compile_c_source("let answer = 1\nsay answer\n", "source-c.zp")
say result["status"]
say result["ownership"]
say result["reference_owner"]
say result["bytecode"]["status"]
say result["instruction_count"]
EOF
"$SEED" "$(basename "$runner")" > "$output"
cat > "${output}.expected" <<'EOF'
zap_owned_source_to_c
zap_owned
zap
compiled_driver_c_backend
5
EOF
cmp "$output" "${output}.expected" || fail "source-to-C driver result changed"
printf 'source-to-C ownership gate passed: Zap driver produced owned C source from source input\n'
