#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b3-semver.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b3/package.zp"
let d120 = registry_package("demo", "1.2.0", "d120", [])
let d129 = registry_package("demo", "1.2.9", "d129", [])
let d142 = registry_package("demo", "1.4.2", "d142", [])
let d130 = registry_package("demo", "1.3.0", "d130", [])
let d200 = registry_package("demo", "2.0.0", "d200", [])
let caret = resolve_dependency_graph([dependency("demo", "^1.2.0", "")], [d120, d142, d200])
let tilde = resolve_dependency_graph([dependency("demo", "~1.2.0", "")], [d120, d129, d130])
let intersection = resolve_dependency_graph([dependency("demo", ">=1.2.0,<1.3.0", "")], [d120, d129, d130])
let invalid = resolve_dependency_graph([dependency("demo", "^broken", "")], [d120])
let exact_short = semver_satisfies("1.2.0", "1.2")
let prerelease_core = semver_satisfies("1.2.9-beta.1", "~1.2.0")
say semver_parse("1.2")["patch"]
say exact_short
say semver_satisfies("1.4.2", "^1.2.0")
say semver_satisfies("2.0.0", "^1.2.0")
say semver_satisfies("1.2.9", "~1.2.0")
say semver_satisfies("1.3.0", ">=1.2.0,<1.3.0")
say prerelease_core
say caret["resolved"][0]["version"]
say tilde["resolved"][0]["version"]
say intersection["resolved"][0]["version"]
say invalid["ok"]
say invalid["errors"][0]["code"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "0 true true false true false true 1.4.2 1.2.9 1.2.9 false ZAP-PKG-VERSION-INVALID-001" ]]; then
  echo "unexpected semver output: ${lines[*]}" >&2
  exit 1
fi
printf 'B3 semver-range gate passed: padded exact, caret, tilde, comparator intersection, highest selection, prerelease core, and invalid-range rejection\n'
