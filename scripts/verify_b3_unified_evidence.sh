#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT_DIR"
REPORT=${B3_EVIDENCE_REPORT:-"$ROOT_DIR/target/b3-unified-evidence.tsv"}
mkdir -p "$(dirname "$REPORT")"
tmp=$(mktemp)
trap 'rm -f "$tmp"' EXIT
printf 'gate	category	status	duration_seconds	log\n' > "$tmp"

run_gate() {
  local gate="$1" category="$2" command="$3" start end duration status log
  log="${REPORT%.tsv}-${gate}.log"
  start=$(date +%s)
  if bash "$ROOT_DIR/$command" >"$log" 2>&1; then status=passed; else status=failed; fi
  end=$(date +%s)
  duration=$((end - start))
  printf '%s\t%s\t%s\t%s\t%s\n' "$gate" "$category" "$status" "$duration" "$log" >> "$tmp"
  [[ "$status" == "passed" ]]
}

set +e
run_gate canonical_ast_schema schema scripts/bootstrap/verify_b3_canonical_ast_schema.sh
s1=$?
run_gate ast_schema_compatibility schema scripts/test_b3_ast_schema_compatibility.sh
s2=$?
run_gate b3_foundations foundation scripts/bootstrap/verify_b3_foundations.sh
s3=$?
run_gate dependency_graph foundation scripts/bootstrap/verify_b3_dependency_graph_11.sh
s4=$?
run_gate cli_lsp_semantic_parity cli-lsp scripts/test_lsp_semantic_parity.sh
s5=$?
run_gate lsp_protocol_sync cli-lsp scripts/test_lsp_protocol_sync.sh
s6=$?
run_gate rust_free_seed seed scripts/bootstrap/verify_non_rust_seed_pipeline.sh
s7=$?
set -e
mv "$tmp" "$REPORT"
if (( s1 || s2 || s3 || s4 || s5 || s6 || s7 )); then
  printf 'B3 unified evidence failed: report=%s\n' "$REPORT" >&2
  exit 1
fi
printf 'B3 unified evidence passed: schema, foundation, CLI/LSP, and Rust-free seed gates; report=%s\n' "$REPORT"
