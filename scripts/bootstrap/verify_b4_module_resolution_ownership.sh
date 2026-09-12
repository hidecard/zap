#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "B4 module-resolution ownership failed: $*" >&2; exit 1; }
SEED="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN_OVERRIDE:-$ROOT_DIR/bin/zap}}"
[[ -x "$SEED" ]] || fail "Zap seed required: $SEED"
REPORT="${B4_MODULE_RESOLUTION_REPORT:-target/b4-module-resolution-ownership.tsv}"
mkdir -p "$(dirname "$REPORT")"
runner=$(mktemp "$ROOT_DIR/.zap-b4-module-owner.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
let owner = driver_module_resolution_ownership()
let normalized = driver_normalize_module_name("pkg\\\\sub///feature.zp")
let sources = ["import \"shared\"\nsay 1\n", "say 2\n"]
let graph = driver_resolve_modules(sources, ["app.zp", "shared.zp"])
let replay = driver_modules_graph_replay(sources, ["app.zp", "shared.zp"])
say owner["owner"]
say owner["status"]
say normalized
say graph["status"]
say graph["modules"][0]["module"]
say graph["modules"][1]["module"]
say replay["byte_equal"]
EOF
"$SEED" "$(basename "$runner")" > "$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
expected=(zap zap_owned_slice pkg/sub/feature modules_resolved shared app true)
[[ "${lines[*]}" == "${expected[*]}" ]] || fail "unexpected ownership/graph output: ${lines[*]}"
{
  printf 'schema_version\t1\ncontract_id\tB4-MODULE-RESOLUTION-OWNERSHIP\nstatus\tpassed\nowner\tzap\ncanonical_graph\ttrue\ndeterministic_replay\ttrue\n' 
} > "$REPORT"
printf 'B4 module-resolution ownership passed: canonical names, dependency graph, and deterministic replay verified\n'
