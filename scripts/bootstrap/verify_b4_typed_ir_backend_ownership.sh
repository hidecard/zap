#!/usr/bin/env bash
# Verify the explicit Zap-owned typed-IR -> lowering -> VM boundary.
# This is an ownership wiring gate, not full-language certification.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "typed-IR/backend ownership failed: $*" >&2; exit 1; }
DRIVER="bootstrap/b4/compiler_driver.zp"
[[ -f "$DRIVER" ]] || fail "missing $DRIVER"
for required in \
  'import "bootstrap/b2/typed_ir.zp"' \
  'import "bootstrap/b3/lower.zp"' \
  'import "bootstrap/b3/vm.zp"' \
  'export fn driver_typed_ir_backend_ownership(' \
  'export fn driver_typed_ir_ownership(' \
  'export fn driver_typed_ir_expression_coverage_valid(' \
  'export fn driver_typed_ir_node_coverage_valid(' \
  'export fn driver_typed_ir_nodes_coverage_valid(' \
  'export fn driver_finalize_typed_ir(' \
  'export fn driver_promote_typed_ir(' \
  'export fn driver_compile_backend(' \
  '"ownership": "zap_owned"' \
  '"reference_owner": "zap"' \
  'typed_ir_promotion_error'; do
  grep -Fq "$required" "$DRIVER" || fail "missing ownership boundary: $required"
done
grep -Fq '"candidate_only": false' "$DRIVER" || fail "promoted typed-IR must clear candidate_only"
grep -Fq 'coverage_valid' "$DRIVER" || fail "typed-IR promotion must expose coverage_valid"
if grep -n -E '\b(cargo|rustc|rustup)\b|host/zap-host|native/src' "$DRIVER"; then
  fail "driver contains a forbidden compiler-path fallback"
fi
if ! grep -Fq '"complete_language": false' "$DRIVER"; then
  fail "ownership contract must retain the incomplete-language boundary"
fi
SEED="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN:-$ROOT_DIR/bin/zap}}"
[[ -x "$SEED" ]] || fail "verified Zap seed required; set ZAP_BOOTSTRAP_BIN"
runner=$(mktemp "$ROOT_DIR/.zap-typed-ir-ownership.XXXXXX.zp")
output=$(mktemp)
trap 'rm -f "$runner" "$output"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
let promoted = driver_promote_typed_ir("let answer = 1\nsay answer\n", "ownership.zp")
let executed = driver_execute_owned_pipeline("let answer = 1\nsay answer\n", "ownership.zp")
say promoted["status"]
say promoted["typed_ir"]["candidate_only"]
say promoted["typed_ir"]["ownership"]
say promoted["typed_ir"]["reference_owner"]
say executed["ownership"]
say executed["reference_owner"]
say executed["stage_chain_valid"]
EOF
"$SEED" "$(basename "$runner")" > "$output"
cat > "${output}.expected" <<'EOF'
typed_ir_promoted
false
zap_owned
zap
zap_owned
zap
true
EOF
cmp "$output" "${output}.expected" || fail "finalized typed-IR/backend propagation changed"
rm -f "${output}.expected"
printf 'typed-IR/backend ownership wiring gate passed: Zap promotion, lowering, VM propagation, and fail-closed boundary verified\n'
chmod +x "$ROOT_DIR/scripts/bootstrap/verify_b4_typed_ir_backend_ownership.sh"
