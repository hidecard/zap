#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "B4 frontend ownership slice failed: $*" >&2; exit 1; }

SEED="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN:-$ROOT_DIR/bin/zap}}"
[[ -x "$SEED" ]] || fail "verified Zap seed required; set ZAP_BOOTSTRAP_BIN"
REPORT="${B4_FRONTEND_OWNERSHIP_REPORT:-target/b4-frontend-ownership.tsv}"
mkdir -p "$(dirname "$REPORT")"
runner=$(mktemp "$ROOT_DIR/.zap-frontend-ownership.XXXXXX.zp")
output=$(mktemp)
trap 'rm -f "$runner" "$output"' EXIT

cat > "$runner" <<'EOF'
import "bootstrap/b4/compiler_driver.zp"
let ownership = driver_frontend_ownership()
let verified = driver_verified_frontend_acceptance()
let source = read_text("bootstrap/fixtures/driver/module_ambiguous_app.zp")
let parsed = driver_parse_source(source, "app.zp")
let graph = driver_resolve_modules([source, read_text("bootstrap/fixtures/driver/module_graph_base.zp"), read_text("bootstrap/fixtures/driver/module_graph_base.zp")], ["app.zp", "a/shared.zp", "b/shared.zp"])
say ownership["status"]
say ownership["complete_language"]
say ownership["components"][0]["owner"]
say parsed["status"]
say graph["diagnostics"][0]["code"]
say ownership["acceptance"]["lexer_corpus"]["fixture_count"]
say ownership["acceptance"]["parser_corpus"]["fixture_count"]
say ownership["acceptance"]["module_resolution"]["diagnostic_count"]
say verified["status"]
say verified["complete_language"]
say verified["components"][0]["status"]
EOF

"$SEED" "$(basename "$runner")" > "$output"
cat > "${output}.expected" <<'EOF'
frontend_ownership_slice
false
zap
ok
ZAP-MODULE-004
192
62
5
verified_frontend_components_candidate_language
false
zap_owned
EOF
cmp "$output" "${output}.expected" || fail "frontend ownership/resolution output changed"
rm -f "${output}.expected"

printf 'schema_version\t1\ncontract_id\tB4-FRONTEND-OWNERSHIP-SLICE\nstatus\tpassed\nverified_acceptance_status\tverified_frontend_components_candidate_language\nlexer_owner\tzap\nparser_owner\tzap\nmodule_resolution_owner\tzap\nlexer_fixture_count\t192\nparser_fixture_count\t62\nmodule_diagnostic_count\t5\ncomplete_language\tfalse\nreference_owner\trust\n' > "$REPORT"
printf 'B4 frontend ownership slice passed: Zap lexer/parser/module-resolution path and explicit non-certification boundary verified\n'
