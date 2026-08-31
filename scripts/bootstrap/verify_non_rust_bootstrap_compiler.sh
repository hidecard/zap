#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

# Rust-free end-to-end verification: a Python bootstrap compiler turns Zap source
# into bytecode that the Python VM host (host/zap-vm-host/run.py) executes. No
# Rust toolchain is involved at any stage.
if ! command -v python3 >/dev/null 2>&1; then
  echo "python3 not found; cannot run the non-Rust bootstrap compiler verifier" >&2
  exit 2
fi

python3 host/zap-bootstrap/verify.py
