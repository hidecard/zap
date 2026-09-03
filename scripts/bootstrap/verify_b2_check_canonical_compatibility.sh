#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
runner=$(mktemp "$ROOT_DIR/.zap-b2-check-canonical-compatibility-runner.XXXXXX.zp")
runner_rel=$(basename "$runner")
output=$(mktemp "${TMPDIR:-/tmp}/zap-b2-check-canonical-compatibility-output.XXXXXX")
trap 'rm -f "$runner" "$output"' EXIT
cat > "$runner" <<'EOF'
import "bootstrap/b1/parser.zp"
import "bootstrap/b2/typecheck.zp"

export fn diagnostic_core(diagnostic):
    return {"code": diagnostic["code"], "column": diagnostic["column"], "kind": diagnostic["kind"], "line": diagnostic["line"], "message": diagnostic["message"], "severity": diagnostic["severity"]}

export fn diagnostics_core(diagnostics, index, result):
    if index >= len(diagnostics):
        return result
    return diagnostics_core(diagnostics, index + 1, append(result, diagnostic_core(diagnostics[index])))

export fn result_core(result):
    let parsed = from_json(result)
    return {"diagnostics": diagnostics_core(parsed["diagnostics"], 0, []), "kind": parsed["kind"], "ok": parsed["ok"], "schema_version": parsed["schema_version"], "source_name": parsed["source_name"]}

let valid = "let value: option<number> = some(1)"
let type_error = "let value: number = \"bad\""
let generic = "fn identity<T>(value: T) -> T:\n    return value\n\nlet output: number = identity(1)"
let malformed_generic = "fn empty<>(value: number) -> number:\n    return value"

say result_core(check_legacy(valid, "valid.zp")) == result_core(check(valid, "valid.zp"))
say result_core(check_legacy(type_error, "type_error.zp")) == result_core(check(type_error, "type_error.zp"))
say result_core(check_legacy(generic, "generic.zp")) == result_core(check(generic, "generic.zp"))
say result_core(check_legacy(malformed_generic, "malformed_generic.zp")) == result_core(check(malformed_generic, "malformed_generic.zp"))
EOF
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi > "$output"
expected=$'true\ntrue\ntrue\ntrue'
printf '%s\n' "$expected" | cmp -s - "$output"
printf 'B2 canonical check compatibility passed: stable valid, type-error, generic, and malformed-generic semantics\n'
