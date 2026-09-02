#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b2-p0-flow.XXXXXX.zp")
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
let positive = check("fn positive(maybe: option<number>) -> number:\n    if is_some(maybe):\n        return maybe + 1\n    return 0", "positive.zp")
let compound = check("fn compound(maybe: option<number>, ready: bool) -> number:\n    if is_some(maybe) and ready:\n        return maybe + 1\n    return 0", "compound.zp")
let loop = check("fn consume(maybe: option<number>) -> number:\n    while is_some(maybe):\n        maybe = none\n    return 0", "loop.zp")
let invalidated = check("fn invalidated(maybe: option<number>) -> number:\n    if is_some(maybe):\n        maybe = none\n        return maybe + 1\n    return 0", "invalidated-flow.zp")
say result_ok(positive)
say result_ok(compound)
say result_ok(loop)
say result_ok(invalidated)
say diagnostic_count(invalidated)
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
python3 - "$out" <<'PY'
import pathlib, sys
lines = [line.strip() for line in pathlib.Path(sys.argv[1]).read_text().splitlines() if line.strip()]
if lines != ["true", "true", "true", "false", "1"]:
    raise SystemExit(f"unexpected flow-sensitive output: {lines!r}")
PY
printf 'B2 P0 flow-sensitive gate passed: branch, compound, loop fixpoint, invalidation\n'
