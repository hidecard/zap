#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PYTHON_BIN="${PYTHON_BIN:-python3}"
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  PYTHON_BIN=python
fi
if ! command -v "$PYTHON_BIN" >/dev/null 2>&1; then
  echo "B3 bytecode C bridge verification failed: Python 3 is required" >&2
  exit 1
fi

exec "$PYTHON_BIN" "$ROOT_DIR/host/zap-bootstrap/verify_b3_bytecode_c_bridge.py"
