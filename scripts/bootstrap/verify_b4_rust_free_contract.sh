#!/usr/bin/env bash
# Validate the official B4 Rust-free full-language contract boundary.
#
# This gate verifies contract integrity and prevents false B4 claims. It does
# not certify the full language until every acceptance row is executable through
# a complete Zap-owned source-to-VM and self-rebuild pipeline.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

CONTRACT="bootstrap/contracts/B4_RUST_FREE_FULL_LANGUAGE_CONTRACT.toml"
ACCEPTANCE="bootstrap/contracts/B4_ACCEPTANCE.tsv"
REPORT="${B4_CONTRACT_REPORT:-target/b4-rust-free-contract.tsv}"
mkdir -p "$(dirname "$REPORT")"

fail() {
  echo "B4 Rust-free contract failed: $*" >&2
  exit 1
}

[[ -f "$CONTRACT" ]] || fail "missing $CONTRACT"
[[ -f "$ACCEPTANCE" ]] || fail "missing $ACCEPTANCE"

schema_version=$(grep '^schema_version = ' "$CONTRACT" | cut -d' ' -f3)
[[ "$schema_version" == "1" || "$schema_version" == "2" ]] || fail "contract schema is not version 1 or 2: $schema_version"

# Validate acceptance manifest schema version matches contract
acceptance_schema=$(awk -F '\t' 'NR == 1 { print $2 }' "$ACCEPTANCE")
[[ "$acceptance_schema" == "$schema_version" ]] || fail "acceptance manifest schema version $acceptance_schema does not match contract schema version $schema_version"
grep -q '^contract_id = "B4-RUST-FREE-FULL-LANGUAGE"$' "$CONTRACT" || fail "wrong contract id"
contract_status=$(grep '^status = ' "$CONTRACT" | cut -d'"' -f2)
[[ "$contract_status" == "not-certified" || "$contract_status" == "certified" ]] || fail "invalid B4 contract status: $contract_status"
for required in \
  'full_language_surface = true' \
  'rust_or_cargo_in_compiler_path = false' \
  'user_facing_cli_owned_by_zap = true' \
  'build_path_owned_by_zap = true' \
  'test_path_owned_by_zap = true'; do
  grep -q "^${required}$" "$CONTRACT" || fail "missing contract requirement: $required"
done

# Schema v2: validate acceptable seed provenance section if present
if [[ "$schema_version" == "2" ]]; then
  grep -q '^\[acceptable_seed_provenance\]' "$CONTRACT" || fail "schema v2 contract missing acceptable_seed_provenance section"
  # Check that the C backend is listed as acceptable provenance
  grep -q 'seed_provenance = "host/zap-bootstrap/c_backend.py"' "$CONTRACT" || fail "schema v2 contract missing seed_provenance field"
fi

[[ "$(awk -F '\t' 'NR == 1 { print $1 }' "$ACCEPTANCE")" == "schema_version" ]] || fail "acceptance manifest missing schema row"
[[ "$(awk -F '\t' 'NR == 2 { print $2 }' "$ACCEPTANCE")" == "B4-RUST-FREE-FULL-LANGUAGE" ]] || fail "acceptance manifest has wrong contract id"
header="$(awk -F '\t' 'NR == 3 { print $0 }' "$ACCEPTANCE")"
[[ "$header" == $'id\tarea\tfixture\towner\tartifact\tstatus' ]] || fail "acceptance manifest header is invalid"

: > "$REPORT"
printf 'schema_version\t%s\ncontract_id\tB4-RUST-FREE-FULL-LANGUAGE\ncontract_status\t%s\n' "$schema_version" "$contract_status" >> "$REPORT"
if [[ "$schema_version" == "2" ]]; then
  printf 'acceptable_provenance\tc_backend\n' >> "$REPORT"
fi
rows=0
while IFS=$'\t' read -r id area fixture owner artifact status; do
  status="${status%%$'\r'}"
  [[ -n "$id" ]] || continue
  [[ "$id" != id ]] || continue
  [[ "$id" == B4-* ]] || fail "invalid acceptance id: $id"
  [[ -n "$area" && -n "$fixture" && -n "$owner" && -n "$artifact" ]] || fail "$id has an empty required field"
  [[ -f "$fixture" ]] || fail "$id fixture is missing: $fixture"
  [[ -f "$owner" ]] || fail "$id owner source is missing: $owner"
  [[ "$status" == "provisional" || "$status" == "pass" || "$status" == "certified" ]] || fail "$id has invalid status: $status"
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$id" "$area" "$fixture" "$owner" "$artifact" "$status" >> "$REPORT"
  rows=$((rows + 1))
done < <(tail -n +4 "$ACCEPTANCE")

# Schema v2 requires 19 rows (original 18 + seed-provenance)
if [[ "$schema_version" == "2" ]]; then
  (( rows >= 19 )) || fail "schema v2 full-language acceptance manifest has only $rows rows (expected at least 19)"
else
  (( rows >= 18 )) || fail "full-language acceptance manifest has only $rows rows (expected at least 18)"
fi

# These paths are allowed to exist as reference or development artifacts, but
# they must not be named by the Zap-owned compiler source as a fallback.
if grep -R -n -E '\b(cargo|rustc|rustup)\b' bootstrap/b1 bootstrap/b2 bootstrap/b3 bootstrap/b4 >/tmp/zap-b4-forbidden-fallbacks 2>/dev/null; then
  cat /tmp/zap-b4-forbidden-fallbacks >&2
  rm -f /tmp/zap-b4-forbidden-fallbacks
  fail "Zap-owned compiler source mentions a Rust/Cargo fallback"
fi
rm -f /tmp/zap-b4-forbidden-fallbacks

if [[ "${B4_RUST_FREE_CERTIFIED:-0}" == 1 ]]; then
  fail "B4 certification must be recorded in the contract file, not via environment override"
fi

# Validate that self-rebuild acceptance scripts exist
for script in \
  "scripts/bootstrap/verify_b4_byte_determinism.sh" \
  "scripts/bootstrap/verify_b4_second_stage_rebuild.sh" \
  "scripts/bootstrap/verify_b4_clean_environment.sh"; do
  [[ -f "$script" ]] || fail "missing self-rebuild acceptance script: $script"
done

# Schema v2: validate C backend exists as acceptable provenance
if [[ "$schema_version" == "2" ]]; then
  [[ -f "host/zap-bootstrap/c_backend.py" ]] || fail "schema v2 requires C backend for seed provenance"
  [[ -f "host/zap-bootstrap/verify_c_backend.py" ]] || fail "schema v2 requires C backend verification script"
fi

if [[ "$contract_status" == "certified" ]]; then
  # Structural validation alone must never turn a candidate/subset into B4.
  [[ -f "target/b4-evidence-report.tsv" ]] || fail "certified contract is missing target/b4-evidence-report.tsv; run verify_b4_evidence.sh"
fi
printf 'B4 Rust-free contract gate passed: %s acceptance rows validated; contract status: %s; schema version: %s\n' "$rows" "$contract_status" "$schema_version"
