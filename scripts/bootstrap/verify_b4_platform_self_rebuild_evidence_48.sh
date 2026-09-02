#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-a12-a13-evidence.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/seed_pipeline.zp"
let targets = ["linux-x86_64", "macos-arm64", "windows-x86_64"]
let valid = [seed_platform_record_evidence("linux-x86_64", "artifact", "digest", "executed", "source-digest", "toolchain-digest", "clean", "bootstrap-artifact"), seed_platform_record_evidence("macos-arm64", "artifact", "digest", "executed", "source-digest", "toolchain-digest", "clean", "bootstrap-artifact"), seed_platform_record_evidence("windows-x86_64", "artifact", "digest", "executed", "source-digest", "toolchain-digest", "clean", "bootstrap-artifact")]
let dirty = [seed_platform_record_evidence("linux-x86_64", "artifact", "digest", "executed", "source-digest", "toolchain-digest", "dirty", "bootstrap-artifact"), seed_platform_record_evidence("macos-arm64", "artifact", "digest", "executed", "source-digest", "toolchain-digest", "clean", "bootstrap-artifact"), seed_platform_record_evidence("windows-x86_64", "artifact", "digest", "executed", "source-digest", "toolchain-digest", "clean", "bootstrap-artifact")]
let first = {"artifact": "bytecode-v1", "digest": "digest-v1"}
let second = {"artifact": "bytecode-v1", "digest": "digest-v1"}
let mismatch = {"artifact": "bytecode-v2", "digest": "digest-v2"}
let ok = seed_self_rebuild_evidence(first, second, valid, targets)
let blocked = seed_self_rebuild_evidence(first, mismatch, dirty, targets)
say seed_platform_evidence_matrix_valid(valid, targets)
say seed_platform_evidence_matrix_valid(dirty, targets)
say ok["status"]
say ok["supported"]
say ok["native_independent"]
say blocked["status"]
say blocked["supported"]
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true false candidate_self_rebuild_evidence true false candidate_self_rebuild_blocked false" ]]; then
  echo "unexpected A12/A13 evidence output: ${lines[*]}" >&2
  exit 1
fi
printf 'A12/A13 candidate evidence gate passed: clean-platform provenance fields, cross-target matrix rejection, deterministic two-pass byte equality, and blocked mismatch status\n'
