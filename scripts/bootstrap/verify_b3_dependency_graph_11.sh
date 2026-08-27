#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
source "$HOME/.cargo/env"
runner=$(mktemp "$ROOT_DIR/.zap-b3-dependency-graph.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b3/package.zp"
let std = dependency("std", "2.11.16", "std-sum")
let net = dependency("net", "1.0.0", "net-sum")
let manifest_value = manifest("demo", "0.1.0", "main.zp", [std, net])
let lock = lockfile(manifest_value, [lock_entry("std", "2.11.16", "std-sum", "registry"), lock_entry("net", "1.0.0", "net-sum", "registry")])
let duplicate_manifest = manifest("demo", "0.1.0", "main.zp", [std, dependency("std", "2.11.17", "other-sum")])
let duplicate_lock = lockfile(manifest_value, [lock_entry("std", "2.11.16", "std-sum", "registry"), lock_entry("std", "2.11.16", "std-sum", "mirror")])
let mismatch_lock = lockfile(manifest_value, [lock_entry("std", "2.11.16", "std-sum", "registry"), lock_entry("net", "1.0.1", "net-sum", "registry")])
say dependency_names_unique(manifest_value["dependencies"])
say lock_entries_unique(lock["dependencies"])
say package_offline_contract(manifest_value, lock["dependencies"])["lock_matches"]
say package_offline_contract(duplicate_manifest, lock["dependencies"])["lock_matches"]
say package_offline_contract(manifest_value, duplicate_lock["dependencies"])["lock_matches"]
say package_offline_contract(manifest_value, mismatch_lock["dependencies"])["lock_matches"]
ZP
cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner" >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true true true false false false" ]]; then
  echo "unexpected dependency graph output: ${lines[*]}" >&2
  exit 1
fi
printf 'B3 dependency-graph gate passed: unique manifest/lock names and deterministic offline matching\n'
