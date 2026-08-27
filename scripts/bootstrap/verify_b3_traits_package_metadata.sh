#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b3-trait-package.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b3/package.zp"
let value = manifest_with_traits("demo", "0.1.0", "main.zp", [])
say value["features"]["traits"]["canonical_parser"]
say value["features"]["traits"]["checker_registry"]
say value["features"]["traits"]["runtime_dispatch"]
say value["features"]["traits"]["release_supported"]
say manifest_traits_compatible(value)
say manifest_traits_compatible(manifest("demo", "0.1.0", "main.zp", []))
EOF
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["true", "true", "false", "false", "true", "true"]:
    raise SystemExit(f"unexpected package metadata output: {lines!r}")
PY
printf 'B3 traits package metadata gate passed: bounded feature flags and compatibility\n'
