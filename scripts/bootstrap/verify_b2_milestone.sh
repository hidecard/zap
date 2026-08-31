#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
REPORT="${ZAP_B2_MILESTONE_REPORT:-$ROOT_DIR/target/b2-milestone-report.tsv}"
mkdir -p "$(dirname "$REPORT")"
printf 'gate\tcommand\tstatus\tresult\n' > "$REPORT"

# These gates cover the B2 acceptance contract without claiming the broader B3/B4 work.
gates=(
  "typecheck|scripts/bootstrap/verify_b2_typecheck.sh"
  "typecheck-candidate|scripts/bootstrap/verify_b2_typecheck_candidate.sh"
  "typed-ir-candidate|scripts/bootstrap/verify_b2_typed_ir_candidate.sh"
  "typed-ir-owned-program|scripts/bootstrap/verify_b2_typed_ir_owned_program_38.sh"
  "flow-sensitive|scripts/bootstrap/verify_b2_flow_sensitive_10.sh"
  "recursive-cfg|scripts/bootstrap/verify_b2_recursive_cfg_loop_convergence_12.sh"
  "generic-end-to-end|scripts/bootstrap/verify_b2_generic_end_to_end_10.sh"
  "generic-bounds|scripts/bootstrap/verify_b2_generic_bounds_10.sh"
)

failures=0
for entry in "${gates[@]}"; do
  gate="${entry%%|*}"
  command="${entry#*|}"
  log="$(mktemp "${TMPDIR:-/tmp}/zap-b2-${gate}.XXXXXX.log")"
  set +e
  bash "$command" >"$log" 2>&1
  status=$?
  set -e
  if (( status == 0 )); then
    result=PASS
  else
    result=FAIL
    failures=$((failures + 1))
  fi
  printf '%s\t%s\t%s\t%s\n' "$gate" "$command" "$status" "$result" >> "$REPORT"
  cat "$log"
  rm -f "$log"
done

if (( failures > 0 )); then
  printf 'B2 milestone gate failed: %s gate(s); report=%s\n' "$failures" "$REPORT" >&2
  exit 1
fi
printf 'B2 milestone gate passed: %s gates; report=%s\n' "${#gates[@]}" "$REPORT"
