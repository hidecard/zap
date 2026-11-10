#!/usr/bin/env bash
set -euo pipefail
ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"
run_zap() {
  if [[ -x "$ROOT_DIR/bin/zap.exe" ]]; then "$ROOT_DIR/bin/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap.exe" ]]; then "$ROOT_DIR/native/target/release/zap.exe" "$@"
  elif [[ -x "$ROOT_DIR/bin/zap" ]]; then "$ROOT_DIR/bin/zap" "$@"
  elif [[ -x "$ROOT_DIR/native/target/release/zap" ]]; then "$ROOT_DIR/native/target/release/zap" "$@"
  else cargo run --quiet --release --locked --manifest-path native/Cargo.toml -- "$@"
  fi
}
[ -f "$HOME/.cargo/env" ] && source "$HOME/.cargo/env" || true
runner=$(mktemp "$ROOT_DIR/.zap-b4-offline-artifact.XXXXXX.zp")
runner_rel=$(basename "$runner")
out=$(mktemp)
trap 'rm -f "$runner" "$out"' EXIT
cat >"$runner" <<'ZP'
import "bootstrap/b4/compiler_driver.zp"
let source = "let value: number = 7\nsay value"
let package = driver_build_package("offline_app", "1.0.0", "main.zp", [], [], source, "main.zp")
let valid = driver_validate_package_artifact(package["artifact_bytes"], package["artifact_digest"])
let envelope = driver_package_artifact_envelope(package["artifact_bytes"])
say package["offline"]
say package["status"]
say valid["status"]
say valid["valid"]
say envelope["schema_version"]
say envelope["artifact_manifest"]["status"]
say len(envelope["artifact_manifest"]["artifacts"])
say package["execution"]["output"][0]
ZP
ZAP_BIN="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-native/target/release/zap}}"
if [ -x "$ZAP_BIN" ]; then "$ZAP_BIN" "$runner_rel"; else run_zap "$runner_rel"; fi >"$out"
lines=()
while IFS= read -r line; do lines+=("$line"); done < <(sed '/^[[:space:]]*$/d' "$out")
if [[ "${lines[*]}" != "true driver_package_build_executed candidate_offline_package_artifact true 2 offline_package_artifact 2 7" ]]; then
  echo "unexpected offline package artifact output: ${lines[*]}" >&2
  exit 1
fi
printf 'B4 offline package-artifact gate passed: canonical manifest, lockfile/dependency envelope, digest validation, and replayable typed-IR/bytecode artifacts\n'
