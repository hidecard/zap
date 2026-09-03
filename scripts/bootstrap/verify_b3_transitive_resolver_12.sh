#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b3-transitive.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b3/package.zp"
let c = registry_package("c", "1.0.0", "c-sum", [])
let b = registry_package("b", "1.0.0", "b-sum", [dependency("c", "1.0.0", "c-sum")])
let a = registry_package("a", "1.0.0", "a-sum", [dependency("b", "1.0.0", "b-sum")])
let shared = registry_package("shared", "1.0.0", "shared-sum", [])
let d = registry_package("d", "1.0.0", "d-sum", [dependency("shared", "1.0.0", "shared-sum")])
let e = registry_package("e", "1.0.0", "e-sum", [dependency("shared", "1.0.0", "shared-sum")])
let positive = resolve_dependency_graph([dependency("a", "1.0.0", "a-sum")], [a, b, c])
let shared_result = resolve_dependency_graph([dependency("d", "1.0.0", "d-sum"), dependency("e", "1.0.0", "e-sum")], [shared, d, e])
let cycle_a = registry_package("cycle-a", "1.0.0", "cycle-a-sum", [dependency("cycle-b", "1.0.0", "cycle-b-sum")])
let cycle_b = registry_package("cycle-b", "1.0.0", "cycle-b-sum", [dependency("cycle-a", "1.0.0", "cycle-a-sum")])
let cycle = resolve_dependency_graph([dependency("cycle-a", "1.0.0", "cycle-a-sum")], [cycle_a, cycle_b])
let version_a = registry_package("root-a", "1.0.0", "root-a-sum", [dependency("shared", "1.0.0", "shared-sum")])
let version_b = registry_package("root-b", "1.0.0", "root-b-sum", [dependency("shared", "2.0.0", "shared-v2-sum")])
let version = resolve_dependency_graph([dependency("root-a", "1.0.0", "root-a-sum"), dependency("root-b", "1.0.0", "root-b-sum")], [version_a, version_b, shared])
let checksum = resolve_dependency_graph([dependency("a", "1.0.0", "wrong-sum")], [a, b, c])
let missing = resolve_dependency_graph([dependency("missing", "1.0.0", "missing-sum")], [])
let repeat_one = resolve_dependency_graph([dependency("a", "1.0.0", "a-sum")], [a, b, c])
let repeat_two = resolve_dependency_graph([dependency("a", "1.0.0", "a-sum")], [c, b, a])
let manifest_value = manifest("demo", "0.1.0", "main.zp", [dependency("a", "1.0.0", "a-sum")])
let generated_lock = resolved_graph_lockfile(manifest_value, positive)
say positive["ok"]
say positive["status"]
say len(positive["resolved"])
say positive["resolved"][0]["name"]
say positive["resolved"][2]["name"]
say shared_result["ok"]
say len(shared_result["resolved"])
say shared_result["resolved"][0]["name"]
say shared_result["resolved"][1]["name"]
say shared_result["resolved"][2]["name"]
say cycle["ok"]
say cycle["errors"][0]["code"]
say version["ok"]
say version["errors"][0]["code"]
say checksum["ok"]
say checksum["errors"][0]["code"]
say missing["ok"]
say missing["errors"][0]["code"]
say repeat_one == repeat_two
say lock_satisfies_manifest(generated_lock["manifest"], generated_lock["dependencies"])
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true resolved 3 a c true 3 d e shared false ZAP-PKG-CYCLE-001 false ZAP-PKG-VERSION-001 false ZAP-PKG-CHECKSUM-001 false ZAP-PKG-MISSING-001 true true" ]]; then
  echo "unexpected transitive resolver output: ${lines[*]}" >&2
  exit 1
fi
printf 'B3 transitive-resolver gate passed: recursive graph, shared dependency, cycle/version/checksum/missing diagnostics, deterministic lockfile\n'
