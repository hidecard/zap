#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
INDEX="$ROOT_DIR/docs/SPEC_OWNERSHIP_INDEX.tsv"
REPORT="${ZAP_SPEC_OWNERSHIP_REPORT:-$ROOT_DIR/target/spec-ownership-report.tsv}"

mkdir -p "$(dirname "$REPORT")"
printf 'rule_id\tdomain\tcanonical_en\tcanonical_mm\tfixture_owner\tstatus\tcompatibility\tdecision\n' > "$REPORT"

if [[ ! -f "$INDEX" ]]; then
  printf 'spec ownership: missing index: %s\n' "$INDEX" >&2
  exit 2
fi

expected_header=$'rule_id\tdomain\tcanonical_en\tcanonical_mm\tfixture_owner\tstatus\tcompatibility'
IFS= read -r header < "$INDEX"
if [[ "$header" != "$expected_header" ]]; then
  printf 'spec ownership: invalid header\n' >&2
  exit 2
fi

check_reference() {
  local reference="$1"
  local label="$2"
  local file="${reference%%#*}"
  local section=""
  if [[ "$reference" == *#* ]]; then
    section="${reference#*#}"
  fi
  if [[ ! -f "$ROOT_DIR/$file" ]]; then
    printf 'spec ownership: %s file missing: %s\n' "$label" "$file" >&2
    return 1
  fi
  if [[ -n "$section" ]] && ! grep -Fq "$section" "$ROOT_DIR/$file"; then
    printf 'spec ownership: %s section missing: %s#%s\n' "$label" "$file" "$section" >&2
    return 1
  fi
}

check_fixture() {
  local reference="$1"
  local file="${reference%%#*}"
  local fragment=""
  if [[ "$reference" == *#* ]]; then
    fragment="${reference#*#}"
  fi
  if [[ ! -f "$ROOT_DIR/$file" ]]; then
    printf 'spec ownership: fixture file missing: %s\n' "$file" >&2
    return 1
  fi
  if [[ -n "$fragment" ]] && ! grep -Fq "$fragment" "$ROOT_DIR/$file"; then
    printf 'spec ownership: fixture owner fragment missing: %s#%s\n' "$file" "$fragment" >&2
    return 1
  fi
}

failures=0
rows=0
previous_id=""
while IFS=$'\t' read -r rule_id domain canonical_en canonical_mm fixture_owner status compatibility || [[ -n "$rule_id" ]]; do
  [[ "$rule_id" == "rule_id" ]] && continue
  if [[ -z "$rule_id" ]]; then
    continue
  fi
  rows=$((rows + 1))
  decision=PASS
  if [[ "$rule_id" == "$previous_id" ]]; then
    decision=FAIL
    printf 'spec ownership: duplicate rule id: %s\n' "$rule_id" >&2
  fi
  previous_id="$rule_id"
  if [[ -z "$domain" || -z "$canonical_en" || -z "$canonical_mm" || -z "$fixture_owner" ]]; then
    decision=FAIL
  fi
  if ! check_reference "$canonical_en" "English" || ! check_reference "$canonical_mm" "Burmese"; then
    decision=FAIL
  fi
  if ! check_fixture "$fixture_owner"; then
    decision=FAIL
  fi
  case "$status" in
    implemented|deferred) ;;
    *) decision=FAIL; printf 'spec ownership: invalid status `%s` for %s\n' "$status" "$rule_id" >&2 ;;
  esac
  case "$compatibility" in
    normative|compatibility|deprecated|rejected) ;;
    *) decision=FAIL; printf 'spec ownership: invalid compatibility `%s` for %s\n' "$compatibility" "$rule_id" >&2 ;;
  esac
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$rule_id" "$domain" "$canonical_en" "$canonical_mm" "$fixture_owner" \
    "$status" "$compatibility" "$decision" >> "$REPORT"
  if [[ "$decision" != PASS ]]; then
    failures=$((failures + 1))
  fi
done < "$INDEX"

if (( rows < 10 )); then
  printf 'spec ownership: index must contain at least 10 rule rows, found %s\n' "$rows" >&2
  failures=$((failures + 1))
fi
if (( failures > 0 )); then
  printf 'spec ownership validation failed: %s issue(s); report=%s\n' "$failures" "$REPORT" >&2
  exit 1
fi
printf 'spec ownership validation passed: %s rows; report=%s\n' "$rows" "$REPORT"
