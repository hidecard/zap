#!/usr/bin/env bash
# Verify that the driver has an explicit Zap-owned backend path for every
# full-language acceptance area. This proves ownership wiring, not B4
# certification; executable clean-environment evidence is a separate gate.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "full-language backend ownership failed: $*" >&2; exit 1; }
DRIVER="bootstrap/b4/compiler_driver.zp"
MANIFEST="bootstrap/contracts/B4_ACCEPTANCE.tsv"
[[ -f "$DRIVER" ]] || fail "missing $DRIVER"
[[ -f "$MANIFEST" ]] || fail "missing $MANIFEST"
for required in \
  'import "bootstrap/b2/typed_ir.zp"' \
  'import "bootstrap/b3/lower.zp"' \
  'import "bootstrap/b3/vm.zp"' \
  'import "bootstrap/b3/package.zp"' \
  'import "bootstrap/b4/runner.zp"' \
  'export fn driver_compile_backend(' \
  'export fn driver_build_package(' \
  'export fn driver_test_source('; do
  grep -Fq "$required" "$DRIVER" || fail "driver is missing explicit ownership path: $required"
done
if grep -Fq 'import "bootstrap/b4/native_independent.zp"' "$DRIVER"; then
  fail "driver still depends on the composite candidate seed wrapper"
fi
if grep -n -E '\b(cargo|rustc|rustup)\b|host/zap-host|native/src' "$DRIVER"; then
  fail "driver source contains a forbidden Rust/native fallback"
fi
rows=$(awk -F '\t' 'NR >= 4 && $1 ~ /^B4-FULL-/ { count += 1 } END { print count + 0 }' "$MANIFEST")
[[ "$rows" -eq 18 ]] || fail "expected 18 full-language acceptance rows, got $rows"
for fixture in $(awk -F '\t' 'NR >= 4 && $1 ~ /^B4-FULL-/ { print $3 }' "$MANIFEST"); do
  [[ -f "$fixture" ]] || fail "missing acceptance fixture: $fixture"
done
printf 'full-language backend ownership wiring gate passed: explicit Zap stages and %s acceptance fixtures verified\n' "$rows"
