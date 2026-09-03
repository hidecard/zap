#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b3-trait-package.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b3/package.zp"
let value = manifest_with_traits("demo", "0.1.0", "main.zp", [])
let required = trait_dependency("ui", "1.0.0", "abc", ["Printable"])
let provider = registry_trait_entry("ui", "1.0.0", "abc", ["Printable", "Identifiable"])
say dependency_traits_match(required, provider)
say registry_lock_satisfies_trait_dependencies({"dependencies": [required]}, [provider])
say value["features"]["traits"]["canonical_parser"]
say value["features"]["traits"]["checker_registry"]
say value["features"]["traits"]["runtime_dispatch"]
say value["features"]["traits"]["release_supported"]
say manifest_traits_compatible(value)
say manifest_traits_compatible(manifest("demo", "0.1.0", "main.zp", []))
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["true", "true", "true", "true", "false", "false", "true", "true"]:
    raise SystemExit(f"unexpected package metadata output: {lines!r}")
PY
printf 'B3 traits package metadata gate passed: bounded feature flags and compatibility\n'
