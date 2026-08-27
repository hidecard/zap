#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-guards.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typecheck.zp"
fn result_ok(result):
    let parsed = from_json(result)
    return parsed["ok"]
fn diagnostic_count(result):
    let parsed = from_json(result)
    return len(parsed["diagnostics"])
fn first_code(result):
    let parsed = from_json(result)
    if len(parsed["diagnostics"]) == 0:
        return "none"
    return parsed["diagnostics"][0]["code"]
let nullable = check("let maybe: option<number> = none", "nullable.zp")
let unary_not = check("fn accept(maybe: option<number>) -> number:\n    if not is_some(maybe):\n        return 0\n    return maybe + 1", "unary-not.zp")
let reversed_none = check("fn accept(maybe: option<number>) -> number:\n    if none != maybe:\n        return maybe + 1\n    return 0", "reversed-none.zp")
let compound = check("fn accept(maybe: option<number>) -> number:\n    if is_some(maybe) and not is_option_none(maybe):\n        return maybe + 1\n    return 0", "compound.zp")
let compound_or = check("fn accept(maybe: option<number>) -> number:\n    if is_option_none(maybe) or maybe == none:\n        return 0\n    return maybe + 1", "compound-or.zp")
let invalidated = check("fn reject(maybe: option<number>) -> number:\n    if is_some(maybe):\n        maybe = none\n        return maybe + 1\n    return 0", "invalidated.zp")
say result_ok(nullable)
say result_ok(unary_not)
say result_ok(reversed_none)
say result_ok(compound)
say result_ok(compound_or)
say result_ok(invalidated)
say diagnostic_count(invalidated)
say first_code(invalidated)
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["true", "true", "true", "true", "true", "false", "1", "ZAP-TYPE-002"]:
    raise SystemExit(f"unexpected P0 inference/guard output: {lines!r}")
PY
printf 'B2 P0 inference/guard gate passed: nullable, unary-not, reversed, compound, invalidation\n'
