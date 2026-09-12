#!/usr/bin/env bash
# Verify the Zap-owned compiler driver exposes the user-facing command contract.
# Each command runs in a fresh process so bounded runtime memory is respected.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "B4 user-command integration failed: $*" >&2; exit 1; }
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

runner=$(mktemp "$ROOT_DIR/.zap-user-command.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT

run_command() {
  local command="$1" expression="$2" expected="$3"
  cat > "$runner" <<EOF
import "bootstrap/b4/compiler_driver.zp"
let source = "let value: number = 7\\nsay value\\n"
let result = ${expression}
say result["status"]
say result["native_independent"]
EOF
  run_zap "$(basename "$runner")" > "$out"
  printf '%s\n%s\n' "$expected" "false" > "${out}.expected"
  cmp "$out" "${out}.expected" || fail "$command returned unexpected result: $(tr '\n' ' ' < "$out")"
  rm -f "${out}.expected"
}

run_command check 'driver_check_source(source, "main.zp")' ok
run_command build 'driver_build_source(source, "main.zp")' ok
run_command run 'driver_run_source(source, "main.zp")' ok
run_command test 'driver_test_source(source, "main.zp")' ok

cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
let registry = []
let result = driver_build_package("app", "0.1.0", "main.zp", [], registry, "let value: number = 7\nsay value\n", "main.zp")
say result["status"]
say result["native_independent"]
say result["artifact_digest"] != ""
EOF
run_zap "$(basename "$runner")" > "$out"
printf 'driver_package_build_executed\nfalse\ntrue\n' > "${out}.expected"
cmp "$out" "${out}.expected" || fail "package returned unexpected result: $(tr '\n' ' ' < "$out")"
rm -f "${out}.expected"

cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
let source = "let value: number = 7\nsay value\n"
let result = driver_rebuild(source, "main.zp")
say result["status"]
say result["native_independent"]
say result["byte_equal"]
EOF
run_zap "$(basename "$runner")" > "$out"
printf 'candidate_driver_rebuild\nfalse\ntrue\n' > "${out}.expected"
cmp "$out" "${out}.expected" || fail "rebuild returned unexpected result: $(tr '\n' ' ' < "$out")"
rm -f "${out}.expected"

printf 'schema_version\t1\ncontract_id\tB4-USER-COMMANDS\nstatus\tpassed\ncheck\tpassed\nbuild\tpassed\nrun\tpassed\ntest\tpassed\npackage\tpassed\ncompiler_rebuild\tpassed\n' > "${B4_USER_COMMAND_REPORT:-target/b4-user-commands.tsv}"
printf 'B4 user-command integration gate passed: check/build/run/test/package/rebuild contracts verified in fresh processes\n'
