#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b2-flow-edge.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b2/typecheck.zp"
let base_environment = [b2c_binding("value", "option<number>", "option<number>", true)]
let narrowed_environment = [b2c_binding("value", "option<number>", "number", true)]
let base = b2c_state(base_environment, [], true)
let break_edge = b2c_state_with_edges(narrowed_environment, [], true, [], [])
let then_state = b2c_state_with_edges(narrowed_environment, [], false, [break_edge], [])
let else_state = b2c_state_with_edges(base_environment, [], true, [], [])
let merged = b2c_merge_path_states(base, then_state, else_state)
let continue_edge = b2c_state_with_edges(narrowed_environment, [], true, [], [])
let loop_body = b2c_state_with_edges(base_environment, [], true, [], [continue_edge])
let back = b2c_merge_loop_back_environment(base_environment, base_environment, loop_body)
let complex_guard = {"args": [], "callee": {"kind": "member", "member": "ready", "target": {"kind": "name", "name": "service"}}, "kind": "call", "span": {"column": 1, "length": 14, "line": 1}}
let guard_result = b2c_condition_fact(base_environment, complex_guard, true)
say len(merged["breaks"])
say merged["reachable"]
say b2c_lookup(merged["environment"], "value")["current"]
say b2c_lookup(back, "value")["current"]
say json(guard_result) == json(base_environment)
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "1 true option<number> option<number> true" ]]; then
  echo "unexpected flow-edge output: ${lines[*]}" >&2
  exit 1
fi
printf 'B2 flow-edge safety gate passed: branch edges preserved, continue back-edge merged, complex guard safe\n'
