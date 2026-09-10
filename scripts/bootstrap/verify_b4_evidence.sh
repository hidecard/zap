#!/usr/bin/env bash
# B4 evidence independent verification.
#
# Verifies the B4 Rust-free full-language evidence package
# from a clean checkout. This script does not require a Rust toolchain;
# it validates evidence files, contract integrity, and acceptance rows.
#
# Usage:
#   bash scripts/bootstrap/verify_b4_evidence.sh [--run-gates]
#
# --run-gates: also execute B4 verifier scripts (requires native binary)

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

CONTRACT="bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml"
ACCEPTANCE="bootstrap/contracts/B4_ACCEPTANCE.tsv"
EVIDENCE="bootstrap/evidence/b4/certification_evidence.md"
REPORT="${B4_EVIDENCE_REPORT:-target/b4-evidence-report.tsv}"

run_gates=false
if [[ "${1:-}" == "--run-gates" ]]; then
  run_gates=true
fi

mkdir -p "$(dirname "$REPORT")"

fail() {
  echo "B4 evidence verification failed: $*" >&2
  exit 1
}

pass() {
  echo "PASS: $*"
}

# 1. Verify evidence package files exist
[[ -f "$CONTRACT" ]] || fail "missing contract: $CONTRACT"
[[ -f "$ACCEPTANCE" ]] || fail "missing acceptance manifest: $ACCEPTANCE"
[[ -f "$EVIDENCE" ]] || fail "missing evidence document: $EVIDENCE"

# 2. Verify contract integrity
grep -q '^schema_version = 1$' "$CONTRACT" || fail "contract schema is not version 1"
grep -q '^contract_id = "B4-RUST-FREE-FULL-LANGUAGE"$' "$CONTRACT" || fail "wrong contract id"
contract_status=$(grep '^status = ' "$CONTRACT" | cut -d'"' -f2)
[[ "$contract_status" == "not-certified" || "$contract_status" == "certified" ]] || fail "invalid contract status: $contract_status"
for required in \
  'full_language_surface = true' \
  'rust_or_cargo_in_compiler_path = false' \
  'user_facing_cli_owned_by_zap = true' \
  'build_path_owned_by_zap = true' \
  'test_path_owned_by_zap = true'; do
  grep -q "^${required}$" "$CONTRACT" || fail "missing contract requirement: $required"
done
pass "contract integrity"

# 3. Verify acceptance manifest structure
[[ "$(awk -F '\t' 'NR == 1 { print $1 }' "$ACCEPTANCE")" == "schema_version" ]] || fail "acceptance manifest missing schema row"
[[ "$(awk -F '\t' 'NR == 2 { print $2 }' "$ACCEPTANCE")" == "B4-RUST-FREE-FULL-LANGUAGE" ]] || fail "acceptance manifest has wrong contract id"
header="$(awk -F '\t' 'NR == 3 { print $0 }' "$ACCEPTANCE")"
[[ "$header" == $'id\tarea\tfixture\towner\tartifact\tstatus' ]] || fail "acceptance manifest header is invalid"
pass "acceptance manifest structure"

# 4. Verify acceptance rows
rows=0
passing=0
failing=0
missing_evidence=0
while IFS=$'\t' read -r id area fixture owner artifact status; do
  [[ "$id" == "schema_version" ]] && continue
  [[ "$id" == "id" ]] && continue
  [[ -z "$id" ]] && continue
  [[ "$id" != B4-* ]] && continue
  rows=$((rows + 1))
  if [[ "$status" != "pass" ]]; then
    failing=$((failing + 1))
    echo "FAIL: $id ($area) status=$status"
  else
    passing=$((passing + 1))
  fi
  if ! grep -q "$id" "$EVIDENCE"; then
    missing_evidence=$((missing_evidence + 1))
    echo "WARN: $id not mentioned in evidence document"
  fi
done < "$ACCEPTANCE"

echo "Acceptance rows: $rows (pass=$passing fail=$failing missing_evidence=$missing_evidence)"
[[ "$passing" -eq 20 ]] || fail "expected 20 passing rows, got $passing"
pass "acceptance rows verified"

# 5. Verify evidence document references key artifacts
for ref in \
  "bootstrap/b1/parser.zp" \
  "bootstrap/b2/typecheck.zp" \
  "bootstrap/b2/typed_ir.zp" \
  "bootstrap/b3/lower.zp" \
  "bootstrap/b3/vm.zp" \
  "bootstrap/b4/compiler_driver.zp" \
  "scripts/bootstrap/verify_b4_rust_free_contract.sh" \
  "scripts/bootstrap/verify_b4_byte_determinism.sh" \
  "scripts/bootstrap/verify_b4_second_stage_rebuild.sh" \
  "scripts/bootstrap/verify_b4_clean_environment.sh" \
  "scripts/bootstrap/verify_b4_driver_owned_pipeline.sh" \
  "scripts/bootstrap/verify_b4_driver_source_to_vm.sh"; do
  if ! grep -q "$ref" "$EVIDENCE"; then
    echo "WARN: evidence document does not reference $ref"
  fi
done
pass "evidence document references"

# 6. Optional: run B4 gates
if [[ "$run_gates" == "true" ]]; then
  if [[ -x "native/target/release/zap" || -x "native/target/release/zap.exe" ]]; then
    echo "Running B4 gates..."
    bash scripts/bootstrap/verify_b4_rust_free_contract.sh
    bash scripts/bootstrap/verify_b4_byte_determinism.sh
    bash scripts/bootstrap/verify_b4_second_stage_rebuild.sh
    bash scripts/bootstrap/verify_b4_clean_environment.sh
    pass "B4 gates executed"
  else
    fail "--run-gates requires a prebuilt native binary; build it before requesting executable B4 evidence"
  fi
fi

# 7. Write report
cat > "$REPORT" <<EOF
schema_version\t1
contract_id\tB4-RUST-FREE-FULL-LANGUAGE
contract_status\t$contract_status
acceptance_rows\t$rows
passing\t$passing
failing\t$failing
missing_evidence\t$missing_evidence
verified_at\t$(date -u +%Y-%m-%dT%H:%M:%SZ)
git_commit\t$(git rev-parse HEAD)
EOF

echo "B4 evidence package verified (contract status: $contract_status). Report: $REPORT"
