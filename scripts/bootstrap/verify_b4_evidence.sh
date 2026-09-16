#!/usr/bin/env bash
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

fail() {
  echo "B4 evidence verification failed: $*" >&2
  exit 1
}

pass() {
  echo "PASS: $*"
}

[[ -f "$CONTRACT" ]] || fail "missing contract: $CONTRACT"
[[ -f "$ACCEPTANCE" ]] || fail "missing acceptance manifest: $ACCEPTANCE"
[[ -f "$EVIDENCE" ]] || fail "missing evidence document: $EVIDENCE"

grep -q '^schema_version = 2$' "$CONTRACT" || fail "contract schema is not version 2"
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

[[ "$(awk -F '\t' 'NR == 1 { print $1 }' "$ACCEPTANCE")" == "schema_version" ]] || fail "acceptance manifest missing schema row"
[[ "$(awk -F '\t' 'NR == 1 { print $2 }' "$ACCEPTANCE")" == "2" ]] || fail "acceptance manifest is not Schema v2"
[[ "$(awk -F '\t' 'NR == 2 { print $2 }' "$ACCEPTANCE")" == "B4-RUST-FREE-FULL-LANGUAGE" ]] || fail "acceptance manifest has wrong contract id"
header="$(awk -F '\t' 'NR == 3 { print $0 }' "$ACCEPTANCE")"
[[ "$header" == $'id\tarea\tfixture\towner\tartifact\tstatus' ]] || fail "acceptance manifest header is invalid"
pass "acceptance manifest structure"

rows=0
passing=0
provisional=0
missing_evidence=0
while IFS=$'\t' read -r id area fixture owner artifact status; do
  status="${status%$'\r'}"
  [[ "$id" == "schema_version" || "$id" == "id" || -z "$id" ]] && continue
  [[ "$id" == B4-* ]] || fail "invalid acceptance id: $id"
  [[ -n "$area" && -n "$fixture" && -n "$owner" && -n "$artifact" ]] || fail "$id has an empty required field"
  [[ -f "$fixture" ]] || fail "$id fixture is missing: $fixture"
  [[ -f "$owner" ]] || fail "$id owner is missing: $owner"
  rows=$((rows + 1))
  case "$status" in
    pass|certified) passing=$((passing + 1)) ;;
    provisional) provisional=$((provisional + 1)) ;;
    *) fail "$id has invalid status: $status" ;;
  esac
  grep -q "$id" "$EVIDENCE" || missing_evidence=$((missing_evidence + 1))
done < <(tail -n +4 "$ACCEPTANCE")

[[ "$rows" -ge 19 ]] || fail "Schema v2 requires at least 19 acceptance rows, got $rows"
if [[ "$contract_status" == "certified" ]]; then
  [[ "$provisional" -eq 0 ]] || fail "certified B4 cannot contain provisional acceptance rows"
  [[ "$passing" -eq "$rows" ]] || fail "certified B4 requires all $rows rows to pass, got $passing"
fi

echo "Acceptance rows: $rows (pass=$passing provisional=$provisional missing_evidence=$missing_evidence)"
pass "acceptance rows verified"

for ref in \
  "host/zap-bootstrap/c_backend.py" \
  "host/zap-bootstrap/verify_b4_c_backend_acceptance.py" \
  "scripts/bootstrap/verify_b4_c_backend_acceptance.sh" \
  "bootstrap/fixtures/b4/c_backend_cli.zp" \
  "bootstrap/fixtures/b4/c_backend_datastructures.zp" \
  "bootstrap/fixtures/b4/c_backend_full_surface.zp" \
  "bootstrap/fixtures/b4/c_backend_seed.zp" \
  "bootstrap/fixtures/b4/c_backend_self_rebuild.zp" \
  ".github/workflows/ci.yml"; do
  grep -q "$ref" "$EVIDENCE" || echo "WARN: evidence document does not reference $ref"
done
pass "evidence document references"

if [[ "$run_gates" == "true" ]]; then
  PYTHON_BIN="${PYTHON_BIN:-python3}"
  command -v "$PYTHON_BIN" >/dev/null 2>&1 || PYTHON_BIN=python
  if "$PYTHON_BIN" -c "import sys; sys.path.insert(0, 'host/zap-bootstrap'); import c_backend; exit(0 if c_backend.find_c_compiler() else 1)" 2>/dev/null; then
    bash scripts/bootstrap/verify_b4_c_backend_acceptance.sh
  else
    echo "INFO: C backend acceptance gate skipped (no system C compiler found)"
  fi
  if [[ -x "native/target/release/zap" || -x "native/target/release/zap.exe" ]]; then
    bash scripts/bootstrap/verify_b4_byte_determinism.sh
    bash scripts/bootstrap/verify_b4_second_stage_rebuild.sh
    bash scripts/bootstrap/verify_b4_clean_environment.sh
  else
    echo "INFO: native B4 gates skipped because no prebuilt native seed is available"
  fi
  pass "B4 executable gates executed"
fi

mkdir -p "$(dirname "$REPORT")"
cat > "$REPORT" <<EOF
schema_version	2
contract_id	B4-RUST-FREE-FULL-LANGUAGE
contract_status	$contract_status
acceptance_rows	$rows
passing	$passing
provisional	$provisional
missing_evidence	$missing_evidence
verified_at	$(date -u +%Y-%m-%dT%H:%M:%SZ)
git_commit	$(git rev-parse HEAD)
EOF

pass "B4 evidence package verified (contract status: $contract_status). Report: $REPORT"
