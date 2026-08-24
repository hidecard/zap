#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)
cd "$ROOT_DIR"

if (($# > 0)); then
  printf 'usage: %s\n' "$0" >&2
  exit 2
fi

TMP_DIR=$(mktemp -d "${TMPDIR:-/tmp}/zap-b3-foundations.XXXXXX")
cleanup() {
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

mkdir -p "$TMP_DIR/project/tests"
cat > "$TMP_DIR/project/zap.toml" <<'EOF'
[package]
name = "bootstrap-b3-fixture"
version = "0.0.0"
main = "main.zp"

[dependencies]
EOF
cat > "$TMP_DIR/project/main.zp" <<'EOF'
say "offline build fixture"
EOF
cat > "$TMP_DIR/project/tests/smoke_test.zp" <<'EOF'
assert(1 + 1 == 2, "arithmetic regression")
assert(type([1, 2]) == "list", "list type regression")
say "B3 test runner fixture passed"
EOF

cargo run --quiet --locked --manifest-path native/Cargo.toml -- lock "$TMP_DIR/project" >/tmp/zap-b3-lock-first.out
sha256sum "$TMP_DIR/project/zap.lock" > "$TMP_DIR/lock.sha256"
cargo run --quiet --locked --manifest-path native/Cargo.toml -- lock "$TMP_DIR/project" >/tmp/zap-b3-lock-second.out
sha256sum -c "$TMP_DIR/lock.sha256" >/dev/null
cargo run --quiet --locked --manifest-path native/Cargo.toml -- check "$TMP_DIR/project" >/tmp/zap-b3-check.out
cargo run --quiet --locked --manifest-path native/Cargo.toml -- build --locked "$TMP_DIR/project" >/tmp/zap-b3-build.out
cargo run --quiet --locked --manifest-path native/Cargo.toml -- test "$TMP_DIR/project" >/tmp/zap-b3-test.out

python3 - "$ROOT_DIR" <<'PY'
import json
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
catalog = json.loads((root / "bootstrap/fixtures/stdlib/catalog.json").read_text(encoding="utf-8"))
entries = catalog["entries"]
allowed = {"pure", "input-deterministic", "runtime-dependent", "external-io"}
if not entries:
    raise SystemExit("stdlib catalog must not be empty")
for entry in entries:
    if entry["determinism"] not in allowed:
        raise SystemExit(f"unknown determinism class: {entry['determinism']}")
    if not entry["name"] or not entry["domain"]:
        raise SystemExit("stdlib entries require domain and name")

versions = (root / "bootstrap/contracts/VERSIONS.toml").read_text(encoding="utf-8")
for required in ["[typed_ir_schema]", "version = 1", '[bootstrap]', 'stage = "B0"', "self_hosted = false"]:
    if required not in versions:
        raise SystemExit(f"version contract missing {required!r}")
PY

printf 'B3 foundation verification passed: pure stdlib policy, manifest, lockfile, offline build, and test runner\n'
