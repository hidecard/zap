#!/usr/bin/env bash
# Run the modern RustSec audit against Zap's existing locked dependency graph.
# This script only audits the existing project lockfile and never mutates dependencies.

set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel 2>/dev/null || true)"
if [[ -z "$ROOT_DIR" ]]; then
  echo "rustsec audit: must run inside the Zap Git repository" >&2
  exit 1
fi
cd "$ROOT_DIR"

AUDIT_BIN="${CARGO_AUDIT_BIN:-cargo-audit}"
if ! command -v "$AUDIT_BIN" >/dev/null 2>&1; then
  echo "rustsec audit: required executable not found: $AUDIT_BIN" >&2
  echo "install cargo-audit 0.22.x with a current stable Rust toolchain before running this gate" >&2
  exit 1
fi

"$AUDIT_BIN" --version
cd native
exec "$AUDIT_BIN" audit
