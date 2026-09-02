#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-a13-supported-evidence.XXXXXX.zp")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/native_independent.zp"
import "bootstrap/b4/seed_pipeline.zp"
let sources = ["let value: number = 7\nsay value", "let value: number = 0\nif value == 0:\n    say 1\nelse:\n    say 2", "fn add(a, b):\n    return a + b\nsay add(2, 3)"]
let names = ["literal.zp", "branch.zp", "function.zp"]
let targets = ["linux-x86_64", "macos-arm64", "windows-x86_64"]
let records = [seed_platform_record_evidence("linux-x86_64", "artifact", "digest", "executed", "source", "toolchain", "clean", "bootstrap-artifact"), seed_platform_record_evidence("macos-arm64", "artifact", "digest", "executed", "source", "toolchain", "clean", "bootstrap-artifact"), seed_platform_record_evidence("windows-x86_64", "artifact", "digest", "executed", "source", "toolchain", "clean", "bootstrap-artifact")]
let evidence = seed_a13_supported_rebuild_evidence(sources, names, records, targets)
say evidence["status"]
say evidence["native_independent"]
say evidence["source_count"]
say evidence["byte_equal"]
say evidence["platform_evidence"]
say evidence["supported"]
say evidence["vm_outputs"][0]["execution"]["output"][0]
say evidence["vm_outputs"][1]["execution"]["output"][0]
say evidence["vm_outputs"][2]["execution"]["output"][0]
ZP
ZAP_BIN="${ZAP_BIN:-native/target/release/zap}"
if [ -x "$ZAP_BIN" ]; then
  "$ZAP_BIN" "$runner"
else
  cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$runner"
fi >"$out"
mapfile -t lines < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "candidate_a13_supported_rebuild false 3 true true true 7 1 5" ]]; then
  echo "unexpected A13 supported rebuild output: ${lines[*]}" >&2
  exit 1
fi
printf 'A13 candidate supported-rebuild gate passed: three source forms, deterministic two-pass rebuild, VM output parity, and platform provenance linkage\n'
