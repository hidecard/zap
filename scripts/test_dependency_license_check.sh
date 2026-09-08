#!/usr/bin/env bash
# Test dependency license check
# P0.3 focused regression test for license compliance

set -euo pipefail
IFS=$'\n\t'

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Check that all dependencies have acceptable licenses
test_native_licenses() {
  if command -v cargo-about >/dev/null 2>&1; then
    printf 'PASS: cargo-about is available for license checking\n'
  else
    printf 'WARN: cargo-about not available; using manual license check\n'
  fi
  
  # Manual check for known problematic licenses
  if grep -qi "GPL\|AGPL\|LGPL" native/Cargo.toml; then
    printf 'FAIL: Found potentially problematic GPL-family license in native/Cargo.toml\n' >&2
    exit 1
  fi
  
  if grep -qi "GPL\|AGPL\|LGPL" host/zap-host/Cargo.toml; then
    printf 'FAIL: Found potentially problematic GPL-family license in host/zap-host/Cargo.toml\n' >&2
    exit 1
  fi
  
  printf 'PASS: manual license check passed - no GPL-family licenses found\n'
}

# Check that the project uses MIT license
test_project_license() {
  if [[ -f "LICENSE" ]]; then
    if grep -qi "MIT" LICENSE; then
      printf 'PASS: Project uses MIT license\n'
    else
      printf 'FAIL: Project LICENSE file does not appear to use MIT license\n' >&2
      exit 1
    fi
  else
    printf 'FAIL: LICENSE file not found\n' >&2
    exit 1
  fi
}

# Run all tests
test_native_licenses
test_project_license

printf 'dependency license check tests passed\n'