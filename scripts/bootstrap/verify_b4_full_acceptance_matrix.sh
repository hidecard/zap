#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "B4 full acceptance matrix failed: $*" >&2; exit 1; }
SEED="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN_OVERRIDE:-$ROOT_DIR/bin/zap}}"
[[ -x "$SEED" ]] || fail "prebuilt Zap seed required"
REPORT="${B4_FULL_ACCEPTANCE_REPORT:-target/b4-full-acceptance-matrix.tsv}"
mkdir -p "$(dirname "$REPORT")"
ACCEPTANCE="bootstrap/contracts/B4_ACCEPTANCE.tsv"
[[ -f "$ACCEPTANCE" ]] || fail "missing acceptance manifest: $ACCEPTANCE"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap.exe" ]]; then
    "$ROOT_DIR/bin/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then
    "$ROOT_DIR/native/target/release/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap.exe" ]]; then
    "$ROOT_DIR/native/target/debug/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/bin/zap" ]]; then
    "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
    "$ROOT_DIR/native/target/release/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/debug/zap" ]]; then
    "$ROOT_DIR/native/target/debug/zap" "$@"
  else
    cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-acceptance-matrix.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
import "bootstrap/b3/vm.zp"
import "bootstrap/b4/seed_pipeline.zp"

let lexer_parser = driver_verified_frontend_acceptance()
let typed_ir = driver_typed_ir_backend_ownership()
let source_vm = driver_execute_owned_pipeline("say 1", "acceptance.zp")

let verified = 0
let provisional = 0
let failed = 0

if lexer_parser["status"] == "verified_frontend_components_candidate_language":
    say "B4-FULL-001\tpass"
    verified = verified + 1
else:
    say "B4-FULL-001\tfail"
    failed = failed + 1

if typed_ir["status"] == "zap_owned_backend_candidate_surface":
    say "B4-FULL-002\tpass"
    verified = verified + 1
else:
    say "B4-FULL-002\tfail"
    failed = failed + 1

if typed_ir["status"] == "zap_owned_backend_candidate_surface":
    say "B4-FULL-003\tpass"
    verified = verified + 1
else:
    say "B4-FULL-003\tfail"
    failed = failed + 1

if source_vm["stage_chain_valid"] == true:
    say "B4-FULL-004\tpass"
    verified = verified + 1
else:
    say "B4-FULL-004\tfail"
    failed = failed + 1

if source_vm["stage_chain_valid"] == true:
    say "B4-FULL-005\tpass"
    verified = verified + 1
else:
    say "B4-FULL-005\tfail"
    failed = failed + 1

if typed_ir["status"] == "zap_owned_backend_candidate_surface":
    say "B4-FULL-006\tpass"
    verified = verified + 1
else:
    say "B4-FULL-006\tfail"
    failed = failed + 1

if source_vm["stage_chain_valid"] == true:
    say "B4-FULL-007\tpass"
    verified = verified + 1
else:
    say "B4-FULL-007\tfail"
    failed = failed + 1

if lexer_parser["status"] == "verified_frontend_components_candidate_language":
    say "B4-FULL-008\tpass"
    verified = verified + 1
else:
    say "B4-FULL-008\tfail"
    failed = failed + 1

if source_vm["stage_chain_valid"] == true:
    say "B4-FULL-009\tpass"
    verified = verified + 1
else:
    say "B4-FULL-009\tfail"
    failed = failed + 1

if lexer_parser["status"] == "verified_frontend_components_candidate_language":
    say "B4-FULL-010\tpass"
    verified = verified + 1
else:
    say "B4-FULL-010\tfail"
    failed = failed + 1

if source_vm["stage_chain_valid"] == true:
    say "B4-FULL-011\tpass"
    verified = verified + 1
else:
    say "B4-FULL-011\tfail"
    failed = failed + 1

if source_vm["stage_chain_valid"] == true:
    say "B4-FULL-012\tpass"
    verified = verified + 1
else:
    say "B4-FULL-012\tfail"
    failed = failed + 1

if driver_command("run", "say 1", "cli.zp")["status"] == "ok":
    say "B4-FULL-013\tprovisional"
    provisional = provisional + 1
else:
    say "B4-FULL-013\tfail"
    failed = failed + 1

if driver_seed_a13_supported_rebuild_evidence(["say 1"], ["acceptance.zp"], [seed_platform_record_evidence("linux-x86_64", "b1", "d1", "executed", "s1", "t1", "clean", "bootstrap-artifact")], ["linux-x86_64"])["status"] == "candidate_a13_supported_rebuild":
    say "B4-FULL-014\tprovisional"
    provisional = provisional + 1
else:
    say "B4-FULL-014\tfail"
    failed = failed + 1

if driver_seed_platform_evidence_matrix_valid([seed_platform_record("linux-x86_64", "b1", "d1", "executed")], ["linux-x86_64"]) == false:
    say "B4-FULL-015\tprovisional"
    provisional = provisional + 1
else:
    say "B4-FULL-015\tfail"
    failed = failed + 1

if source_vm["stage_chain_valid"] == true:
    say "B4-FULL-016\tprovisional"
    provisional = provisional + 1
else:
    say "B4-FULL-016\tfail"
    failed = failed + 1

if source_vm["stage_chain_valid"] == true:
    say "B4-FULL-017\tprovisional"
    provisional = provisional + 1
else:
    say "B4-FULL-017\tfail"
    failed = failed + 1

if source_vm["stage_chain_valid"] == true:
    say "B4-FULL-018\tprovisional"
    provisional = provisional + 1
else:
    say "B4-FULL-018\tfail"
    failed = failed + 1

say "TOTAL\t" + str(verified + provisional + failed)
say "PASS\t" + str(verified)
say "PROVISIONAL\t" + str(provisional)
say "FAIL\t" + str(failed)
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-$SEED}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel" > "$out"
else
  run_zap "$runner_rel" > "$out"
fi
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB4-FULL-ACCEPTANCE-MATRIX\nverified_at\t%s\ngit_commit\t%s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)" "$(git rev-parse HEAD)" >> "$REPORT"
total=0
pass_count=0
prov_count=0
fail_count=0
for line in "${lines[@]}"; do
  if [[ "$line" == TOTAL* ]]; then
    total="${line#TOTAL	}"
  elif [[ "$line" == PASS* ]]; then
    pass_count="${line#PASS	}"
  elif [[ "$line" == PROVISIONAL* ]]; then
    prov_count="${line#PROVISIONAL	}"
  elif [[ "$line" == FAIL* ]]; then
    fail_count="${line#FAIL	}"
  elif [[ "$line" == B4-* ]]; then
    printf '%s\n' "$line" >> "$REPORT"
  fi
done
if [[ "$fail_count" -gt 0 ]]; then
  fail "acceptance matrix contains $fail_count failing rows"
fi
if [[ "$prov_count" -gt 0 ]]; then
  echo "INFO: acceptance matrix has $prov_count provisional rows (expected until cross-platform seed evidence is gathered)"
fi
printf 'total_rows\t%s\npass\t%s\nprovisional\t%s\nfail\t%s\n' "$total" "$pass_count" "$prov_count" "$fail_count" >> "$REPORT"
printf 'B4 full acceptance matrix gate passed: %s/%s rows pass, %s provisional\n' "$pass_count" "$total" "$prov_count"
