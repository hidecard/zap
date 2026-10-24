#!/usr/bin/env bash
# Verify that the Zap-owned C backend emits native programs that compile and run.
#
# The reference C backend owns the full-language acceptance rows. The Zap-owned
# backend in bootstrap/b4/c_backend.zp must be able to emit equivalent native
# programs, so this gate emits C with the Zap backend, compiles it with the
# system C compiler, runs the produced executable, and compares the output.
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

REPORT="${B4_ZAP_C_BACKEND_REPORT:-target/b4-zap-c-backend-emitted-c.tsv}"
mkdir -p "$(dirname "$REPORT")"

PYTHON_BIN="${PYTHON_BIN:-python3}"
command -v "$PYTHON_BIN" >/dev/null 2>&1 || PYTHON_BIN=python

"$PYTHON_BIN" host/zap-bootstrap/verify_b4_zap_c_backend_emitted_c.py
printf 'B4 Zap-owned C backend gate passed: emitted programs compiled and produced the expected output\n'
