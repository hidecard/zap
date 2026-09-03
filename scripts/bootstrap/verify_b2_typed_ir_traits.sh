#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
runner=$(mktemp "$ROOT_DIR/.zap-b2-trait-ir.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'EOF'
import "bootstrap/b2/typed_ir.zp"
let source = "trait Printable:\n    fn format(self) -> text\ninterface Identifiable:\n    fn id(self) -> text\nclass Report with Printable implements Identifiable:\n    fn format(self) -> text:\n        return \"report\"\n    fn id(self) -> text:\n        return \"id\"\nfn show<T: Printable>(value: T) -> text:\n    return \"ok\""
say emit(source, "traits-ir.zp")
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
grep -q '"schema_version":2' "$out"
grep -q '"kind":"trait_declaration"' "$out"
grep -q '"trait_kind":"interface"' "$out"
grep -q '"kind":"trait_conformance"' "$out"
grep -q '"trait":"Printable"' "$out"
grep -q '"kind":"generic_bound"' "$out"
grep -q '"parameter":"T"' "$out"
printf 'B2 trait typed-IR gate passed: declarations, conformance, and generic bounds\n'
