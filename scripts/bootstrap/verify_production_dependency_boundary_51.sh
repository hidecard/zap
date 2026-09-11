#!/usr/bin/env bash
# Verify that the Zap-owned compiler/runtime source path has no Rust/native
# fallback references. Reference implementations and test harnesses are
# intentionally outside this production boundary.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
REPORT="${PRODUCTION_DEPENDENCY_REPORT:-target/production-dependency-boundary.tsv}"
mkdir -p "$(dirname "$REPORT")"
fail() { echo "production dependency boundary failed: $*" >&2; exit 1; }

paths=(bootstrap/b1 bootstrap/b2 bootstrap/b3 bootstrap/b4)
for path in "${paths[@]}"; do
  [[ -d "$path" ]] || fail "missing production source path: $path"
done

# These tokens are forbidden in Zap production source. Rust reference code,
# host adapters, CI scripts, and documentation are not scanned here.
if grep -RInE --include='*.zp' --exclude='*.generated.zp' \
  -e '(^|[^[:alnum:]_])(cargo|rustc|rustup)([^[:alnum:]_]|$)' \
  -e 'native/src' -e 'host/zap-host' \
  bootstrap/b1 bootstrap/b2 bootstrap/b3 bootstrap/b4; then
  fail "forbidden Rust/native dependency found in Zap production source"
fi

# Ensure the compiler driver uses explicit Zap-owned stages, not the removed
# composite seed wrapper or a host fallback.
DRIVER=bootstrap/b4/compiler_driver.zp
[[ -f "$DRIVER" ]] || fail "missing compiler driver: $DRIVER"
for required in \
  'import "bootstrap/b2/typed_ir.zp"' \
  'import "bootstrap/b3/lower.zp"' \
  'import "bootstrap/b3/vm.zp"' \
  'import "bootstrap/b3/package.zp"' \
  'export fn driver_compile_backend(' \
  'export fn driver_command(' \
  'export fn driver_rebuild('; do
  grep -Fq "$required" "$DRIVER" || fail "missing Zap-owned production stage: $required"
done
if grep -Fq 'bootstrap/b4/native_independent.zp' "$DRIVER"; then
  fail "compiler driver still imports composite seed wrapper"
fi

printf 'schema_version\t1\ncontract_id\tPRODUCTION-DEPENDENCY-BOUNDARY\nstatus\tpassed\nproduction_paths\t%s\nforbidden_fallbacks\t0\n' "$(IFS=,; echo "${paths[*]}")" > "$REPORT"
printf 'production dependency boundary gate passed: Zap source path contains no Rust/Cargo/native fallback\n'
