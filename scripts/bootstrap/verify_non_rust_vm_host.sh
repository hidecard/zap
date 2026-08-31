#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

# Rust-free verification of the Zap execution layer: the Python VM host
# (host/zap-vm-host/run.py) interprets the bytecode produced by the Zap
# compiler without the native Rust reference interpreter.
if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 not found; cannot run the non-Rust VM host verifier" >&2
  exit 2
fi

python3 host/zap-vm-host/verify.py
