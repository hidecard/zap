#!/usr/bin/env bash
# Verify the currently supported Rust-free bootstrap slice.
#
# This is deliberately separate from the Rust reference and B0--B4 gates.
# It proves that the seed compiler and VM host can compile and execute their
# supported Zap subset without invoking Cargo, rustc, or rustup.  It is not a
# claim that the complete language is self-hosted.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 not found; cannot verify the Rust-free seed pipeline" >&2
  exit 2
fi

# Remove the conventional Rust toolchain variables so accidental subprocess
# usage cannot borrow the native toolchain configuration from the caller.
env -u CARGO -u CARGO_HOME -u RUSTC -u RUSTUP_HOME \
  python3 host/zap-bootstrap/verify.py
env -u CARGO -u CARGO_HOME -u RUSTC -u RUSTUP_HOME \
  python3 host/zap-vm-host/verify.py

printf 'Rust-free seed-pipeline gate passed: compiler and VM host ran without a Rust toolchain\n'
