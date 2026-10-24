#!/usr/bin/env bash
# Local sweep: run every B4 verification gate and report pass/fail per gate.
set -uo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"
export ZAP_BOOTSTRAP_BIN="${ZAP_BOOTSTRAP_BIN:-$ROOT_DIR/bin/zap.exe}"
export ZAP_BIN="$ZAP_BOOTSTRAP_BIN"
REPORT="${SWEEP_REPORT:-target/b4-gate-sweep.tsv}"
mkdir -p "$(dirname "$REPORT")"
: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB4-GATE-SWEEP\n' >> "$REPORT"
pass=0
fail=0
for script in $(ls scripts/bootstrap/verify_b4_*.sh | sort); do
  name="$(basename "$script")"
  out="$(mktemp)"
  if timeout "${SWEEP_TIMEOUT:-900}" bash "$script" > "$out" 2>&1; then
    status=pass
    pass=$((pass + 1))
  else
    status=fail
    fail=$((fail + 1))
  fi
  printf '%s\t%s\n' "$name" "$status" >> "$REPORT"
  printf '%-56s %s\n' "$name" "$status"
  if [[ "$status" == "fail" ]]; then
    tail -n 6 "$out" | sed 's/^/    | /'
  fi
  rm -f "$out"
done
printf 'total\t%s\npass\t%s\nfail\t%s\n' "$((pass + fail))" "$pass" "$fail" >> "$REPORT"
echo "SWEEP total=$((pass + fail)) pass=$pass fail=$fail"
[[ "$fail" -eq 0 ]]
