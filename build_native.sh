#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cargo build --release --manifest-path "$ROOT/native/Cargo.toml"
mkdir -p "$ROOT/bin"
cp "$ROOT/native/target/release/zap" "$ROOT/bin/zap"
chmod 0755 "$ROOT/bin/zap"
echo "Built standalone binary: $ROOT/bin/zap"
echo "Run: $ROOT/bin/zap <file.zp>"
