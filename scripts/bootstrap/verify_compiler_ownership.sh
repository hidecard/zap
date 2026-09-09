#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
MANIFEST="bootstrap/contracts/COMPILER_OWNERSHIP.tsv"
REPORT="${COMPILER_OWNERSHIP_REPORT:-target/compiler-ownership.tsv}"
mkdir -p "$(dirname "$REPORT")"

failures=0
zap_owned=0
reference_only=0
printf 'component\towner\tsource\tstatus\tresult\n' > "$REPORT"

if [[ ! -f "$MANIFEST" ]]; then
  echo "missing compiler ownership manifest: $MANIFEST" >&2
  exit 1
fi

while IFS=$'\t' read -r component owner source status evidence; do
  [[ -z "${component:-}" || "$component" == \#* || "$component" == schema_version || "$component" == contract_id ]] && continue
  if [[ -z "${owner:-}" || -z "${source:-}" || -z "${status:-}" ]]; then
    echo "invalid ownership row: component=$component" >&2
    failures=$((failures + 1))
    printf '%s\t%s\t%s\t%s\tFAIL-invalid-row\n' "$component" "$owner" "$source" "$status" >> "$REPORT"
    continue
  fi
  if [[ ! -e "$source" ]]; then
    echo "missing ownership source: $component -> $source" >&2
    failures=$((failures + 1))
    printf '%s\t%s\t%s\t%s\tFAIL-missing-source\n' "$component" "$owner" "$source" "$status" >> "$REPORT"
    continue
  fi
  if [[ "$status" == "zap-owned" && "$owner" != "zap" ]]; then
    echo "invalid Zap-owned row: $component owner=$owner" >&2
    failures=$((failures + 1))
    printf '%s\t%s\t%s\t%s\tFAIL-owner-mismatch\n' "$component" "$owner" "$source" "$status" >> "$REPORT"
    continue
  fi
  if [[ "$status" == "reference-only" ]]; then
    reference_only=$((reference_only + 1))
  elif [[ "$status" == "zap-owned" ]]; then
    zap_owned=$((zap_owned + 1))
  fi
  printf '%s\t%s\t%s\t%s\tPASS\n' "$component" "$owner" "$source" "$status" >> "$REPORT"
done < "$MANIFEST"

if (( zap_owned == 0 )); then
  echo "ownership manifest contains no Zap-owned compiler component" >&2
  failures=$((failures + 1))
fi

printf 'Zap compiler ownership gate: zap_owned=%d reference_only=%d failures=%d report=%s\n' "$zap_owned" "$reference_only" "$failures" "$REPORT"
if (( failures > 0 )); then exit 1; fi
