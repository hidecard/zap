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

grep -q '^schema_version = 1$' "$CONTRACT" || fail "contract schema is not version 1"
grep -q '^contract_id = "B4-RUST-FREE-FULL-LANGUAGE"$' "$CONTRACT" || fail "wrong contract id"
grep -q '^status = "not-certified"$' "$CONTRACT" || fail "B4 status must remain not-certified until all rows pass"
for required in \
  'full_language_surface = true' \
  'rust_or_cargo_in_compiler_path = false' \
  'user_facing_cli_owned_by_zap = true' \
  'build_path_owned_by_zap = true' \
  'test_path_owned_by_zap = true'; do
  grep -q "^${required}$" "$CONTRACT" || fail "missing contract requirement: $required"
done

[[ "$(awk -F '\t' 'NR == 1 { print $1 }' "$ACCEPTANCE")" == "schema_version" ]] || fail "acceptance manifest missing schema row"
[[ "$(awk -F '\t' 'NR == 2 { print $2 }' "$ACCEPTANCE")" == "B4-RUST-FREE-FULL-LANGUAGE" ]] || fail "acceptance manifest has wrong contract id"
header="$(awk -F '\t' 'NR == 3 { print $0 }' "$ACCEPTANCE")"
[[ "$header" == $'id\tarea\tfixture\towner\tartifact\tstatus' ]] || fail "acceptance manifest header is invalid"

: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB4-RUST-FREE-FULL-LANGUAGE\ncontract_status\tnot-certified\n' >> "$REPORT"
rows=0
while IFS=$'\t' read -r id area fixture owner artifact status; do
  status="${status%%$'\r'}"
  [[ -n "$id" ]] || continue
  [[ "$id" != id ]] || continue
  [[ "$id" == B4-* ]] || fail "invalid acceptance id: $id"
  [[ -n "$area" && -n "$fixture" && -n "$owner" && -n "$artifact" ]] || fail "$id has an empty required field"
  [[ -f "$fixture" ]] || fail "$id fixture is missing: $fixture"
  [[ -f "$owner" ]] || fail "$id owner source is missing: $owner"
  [[ "$status" == "provisional" || "$status" == "pass" ]] || fail "$id has invalid status: $status"
  printf '%s\t%s\t%s\t%s\t%s\t%s\n' "$id" "$area" "$fixture" "$owner" "$artifact" "$status" >> "$REPORT"
  rows=$((rows + 1))
done < <(tail -n +4 "$ACCEPTANCE")

(( rows >= 18 )) || fail "full-language acceptance manifest has only $rows rows"

# These paths are allowed to exist as reference or development artifacts, but
# they must not be named by the Zap-owned compiler source as a fallback.
if grep -R -n -E '\b(cargo|rustc|rustup)\b' bootstrap/b1 bootstrap/b2 bootstrap/b3 bootstrap/b4 >/tmp/zap-b4-forbidden-fallbacks 2>/dev/null; then
  cat /tmp/zap-b4-forbidden-fallbacks >&2
  rm -f /tmp/zap-b4-forbidden-fallbacks
  fail "Zap-owned compiler source mentions a Rust/Cargo fallback"
fi
rm -f /tmp/zap-b4-forbidden-fallbacks

if [[ "${B4_RUST_FREE_CERTIFIED:-0}" == 1 ]]; then
  fail "B4 certification requested, but contract status is not-certified"
fi

# Validate that self-rebuild acceptance scripts exist
for script in \
  "scripts/bootstrap/verify_b4_byte_determinism.sh" \
  "scripts/bootstrap/verify_b4_second_stage_rebuild.sh" \
  "scripts/bootstrap/verify_b4_clean_environment.sh"; do
  [[ -f "$script" ]] || fail "missing self-rebuild acceptance script: $script"
done

printf 'B4 Rust-free contract gate passed: %s acceptance rows validated; full-language certification remains explicitly not-certified\n' "$rows"
