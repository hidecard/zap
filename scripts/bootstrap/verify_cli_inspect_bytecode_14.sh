#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"
BIN="${ZAP_CLI_BINARY:-$ROOT_DIR/native/target/release/zap}"
[[ -x "$BIN" ]] || { echo "inspect CLI binary missing: $BIN" >&2; exit 1; }
valid=$(mktemp "$ROOT_DIR/.zap-bytecode-valid.XXXXXX.json")
invalid=$(mktemp "$ROOT_DIR/.zap-bytecode-invalid.XXXXXX.json")
wrong=$(mktemp "$ROOT_DIR/.zap-bytecode-wrong.XXXXXX.json")
out=$(mktemp)
trap 'rm -f "$valid" "$invalid" "$wrong" "$out"' EXIT
cat > "$valid" <<'JSON'
{"kind":"zap.bytecode","schema_version":1,"instructions":[{"op":"const","value":2},{"op":"const","value":3},{"op":"add"},{"op":"halt"}]}
JSON
cat > "$invalid" <<'JSON'
{"kind":"not.bytecode","schema_version":1,"instructions":[]}
JSON
cat > "$wrong" <<'JSON'
{"kind":"zap.bytecode","schema_version":99,"instructions":[]}
JSON
"$BIN" inspect --bytecode "$valid" > "$out"
grep -Fxq 'kind=zap.bytecode' "$out"
grep -Fxq 'schema_version=1' "$out"
grep -Fxq 'instruction_count=4' "$out"
grep -Fq '0000 {"op":"const","value":2}' "$out"
if "$BIN" inspect --bytecode "$invalid" >/dev/null 2>&1; then
  echo 'invalid bytecode kind was accepted' >&2
  exit 1
fi
if "$BIN" inspect --bytecode "$wrong" >/dev/null 2>&1; then
  echo 'unsupported bytecode schema was accepted' >&2
  exit 1
fi
printf 'CLI bytecode inspection gate passed: valid rendering and invalid kind/schema rejection\n'
