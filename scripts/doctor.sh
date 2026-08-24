#!/usr/bin/env bash
set -u

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

strict=0
if [[ "${1:-}" == "--strict" ]]; then
  strict=1
elif [[ "${1:-}" != "" ]]; then
  printf 'usage: %s [--strict]\n' "$0" >&2
  exit 2
fi

missing=0
printf 'Zap developer environment doctor\n'
printf 'Repository: %s\n' "$ROOT_DIR"
printf '%-18s %s\n' 'CHECK' 'RESULT'
printf '%-18s %s\n' '-----' '------'

check_command() {
  local label="$1"
  local command_name="$2"
  if command -v "$command_name" >/dev/null 2>&1; then
    printf '%-18s available (%s)\n' "$label" "$("$command_name" --version 2>/dev/null | head -n 1)"
  else
    printf '%-18s missing\n' "$label"
    missing=$((missing + 1))
  fi
}

check_command cargo cargo
check_command rustc rustc
check_command rustup rustup
check_command python3 python3

if cargo audit --version >/tmp/zap-doctor-cargo-audit.$$ 2>/dev/null; then
  printf '%-18s available (%s)\n' 'cargo-audit' "$(head -n 1 /tmp/zap-doctor-cargo-audit.$$)"
else
  printf '%-18s missing (required for RustSec audit)\n' 'cargo-audit'
  missing=$((missing + 1))
fi
rm -f /tmp/zap-doctor-cargo-audit.$$

if [[ -f rust-toolchain.toml ]]; then
  pinned="$(sed -n 's/^channel[[:space:]]*=[[:space:]]*"\([^"]*\)"/\1/p' rust-toolchain.toml | head -n 1)"
  printf '%-18s %s\n' 'pinned-toolchain' "${pinned:-declared without channel}"
else
  printf '%-18s missing (rust-toolchain.toml)\n' 'pinned-toolchain'
  missing=$((missing + 1))
fi

if command -v rustc >/dev/null 2>&1; then
  host="$(rustc -vV | sed -n 's/^host: //p')"
  printf '%-18s %s\n' 'host-target' "${host:-unknown}"
else
  printf '%-18s unavailable (rustc missing)\n' 'host-target'
fi

if command -v rustup >/dev/null 2>&1; then
  active="$(rustup show active-toolchain 2>/dev/null | head -n 1 || true)"
  printf '%-18s %s\n' 'active-toolchain' "${active:-unknown}"
else
  printf '%-18s unavailable (rustup missing)\n' 'active-toolchain'
fi

runtime=""
if [[ -n "${ZAP_BIN:-}" ]]; then
  runtime="$ZAP_BIN"
elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then
  runtime="$ROOT_DIR/native/target/release/zap"
elif command -v zap >/dev/null 2>&1; then
  runtime="$(command -v zap)"
fi
if [[ -n "$runtime" && -x "$runtime" ]]; then
  printf '%-18s available (%s)\n' 'runtime-binary' "$runtime"
  version="$($runtime --version 2>/dev/null | head -n 1 || true)"
  printf '%-18s %s\n' 'runtime-version' "${version:-version command failed}"
else
  printf '%-18s missing (set ZAP_BIN or build native/target/release/zap)\n' 'runtime-binary'
  missing=$((missing + 1))
fi

if (( strict && missing > 0 )); then
  printf '\nEnvironment incomplete: %d prerequisite check(s) missing. No tests were run.\n' "$missing" >&2
  exit 1
fi
if (( missing > 0 )); then
  printf '\nEnvironment incomplete: %d prerequisite check(s) missing. No tests were run.\n' "$missing"
else
  printf '\nEnvironment ready: all doctor checks passed.\n'
fi
