#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
fail() { echo "B1 full-language lexer corpus failed: $*" >&2; exit 1; }
SEED="${ZAP_BOOTSTRAP_BIN:-${ZAP_BIN_OVERRIDE:-$ROOT_DIR/native/target/release/zap}}"
[[ -x "$SEED" ]] || fail "Zap seed required: $SEED"
REPORT="${B1_FULL_LANGUAGE_LEXER_REPORT:-target/b1-full-language-lexer.tsv}"
mkdir -p "$(dirname "$REPORT")"
RUNNER=$(mktemp "$ROOT_DIR/.zap-b1-corpus.XXXXXX.zp")
OUT_A=$(mktemp)
OUT_B=$(mktemp)
CORPUS=$(mktemp)
trap 'rm -f "$RUNNER" "$OUT_A" "$OUT_B" "$CORPUS"' EXIT

# The corpus is the tracked bootstrap fixture language surface. Intentionally
# malformed/diagnostic fixtures remain in the dedicated negative lexer gate.
git ls-files 'bootstrap/fixtures/**/*.zp' \
  | grep -viE 'malformed|invalid|unterminated|overflow|error|missing|duplicate|ambiguous|cyclic' \
  | LC_ALL=C sort > "$CORPUS"
count=$(wc -l < "$CORPUS")
(( count >= 150 )) || fail "corpus unexpectedly small: $count fixtures"

: > "$REPORT"
printf 'schema_version\t1\ncontract_id\tB1-FULL-LANGUAGE-LEXER-CORPUS\nstatus\tpassed\n' >> "$REPORT"
printf 'corpus_files\t%s\n' "$count" >> "$REPORT"
corpus_digest=$(sha256sum "$CORPUS" | awk '{print $1}')
printf 'corpus_manifest_sha256\t%s\n' "$corpus_digest" >> "$REPORT"

json_path() {
  python3 - "$1" <<'PY'
import json, sys
print(json.dumps(sys.argv[1], ensure_ascii=False))
PY
}

while IFS= read -r fixture; do
  path_json=$(json_path "$fixture")
cat > "$RUNNER" <<EOF
import "bootstrap/b1/lexer.zp"
let source = read_text($path_json)
say lex(source, $path_json)
EOF
  "$SEED" "$(basename "$RUNNER")" > "$OUT_A" || fail "lexer process failed: $fixture"
  "$SEED" "$(basename "$RUNNER")" > "$OUT_B" || fail "lexer replay failed: $fixture"
  cmp "$OUT_A" "$OUT_B" || fail "non-deterministic token stream: $fixture"
  python3 - "$OUT_A" "$fixture" <<'PY'
import json, sys
from pathlib import Path
raw = Path(sys.argv[1]).read_text(encoding="utf-8")
try:
    value = json.loads(raw)
except json.JSONDecodeError as exc:
    raise SystemExit(f"invalid lexer JSON for {sys.argv[2]}: {exc}")
if value.get("kind") != "zap.token_stream":
    raise SystemExit(f"unexpected lexer kind for {sys.argv[2]}: {value.get('kind')}")
if value.get("schema_version") != 1:
    raise SystemExit(f"unexpected lexer schema for {sys.argv[2]}")
tokens = value.get("tokens")
if not isinstance(tokens, list) or not tokens or tokens[-1].get("kind") != "end":
    raise SystemExit(f"invalid token stream shape for {sys.argv[2]}")
PY
done < "$CORPUS"

printf 'deterministic_replay\ttrue\n' >> "$REPORT"
printf 'token_stream_schema\t1\n' >> "$REPORT"
printf 'negative_diagnostics\tdelegated_to_verify_b1_lexer.sh\n' >> "$REPORT"
printf 'B1 full-language lexer corpus passed: %s tracked fixture sources, deterministic token streams, schema and end-token checks\n' "$count"
