#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-package-build.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
let leaf = {"checksum": "leaf-checksum", "dependencies": [], "name": "leaf", "source": "registry", "version": "1.2.0"}
let mid = {"checksum": "mid-checksum", "dependencies": [{"checksum": "leaf-checksum", "name": "leaf", "version": "^1.0.0"}], "name": "mid", "source": "registry", "version": "1.0.0"}
let ok = seed_build_package_owned("app", "0.1.0", "main.zp", [{"checksum": "mid-checksum", "name": "mid", "version": "^1.0.0"}], [mid, leaf], "let value: number = 7\nsay value", "main.zp")
let bad = seed_build_package_owned("app", "0.1.0", "main.zp", [{"checksum": "missing-checksum", "name": "missing", "version": "^1.0.0"}], [mid, leaf], "say 7", "main.zp")
say ok["status"]
say len(ok["dependency_graph"])
say len(ok["lockfile"]["dependencies"])
say ok["execution"]["output"][0]
say len(ok["diagnostics"])
say ok["artifact_digest"] != ""
say bad["status"]
say bad["diagnostics"][0]["code"]
say bad["native_independent"]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner_rel"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner_rel"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "candidate_package_build_executed 2 2 7 0 true package_dependency_error ZAP-PKG-MISSING-001 false" ]]; then
  echo "unexpected owned package build output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 owned-package-build gate passed: transitive lock graph, Zap pipeline execution, artifact digest, VM output, and dependency failure boundary\n'
