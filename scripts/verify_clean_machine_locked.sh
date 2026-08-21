#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
NATIVE_DIR="$ROOT/native"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

cargo build --manifest-path "$NATIVE_DIR/Cargo.toml" --release --locked >/dev/null
cargo test --manifest-path "$NATIVE_DIR/Cargo.toml" cache_write_and_verification_use_checksum_path --locked >/dev/null
cargo test --manifest-path "$NATIVE_DIR/Cargo.toml" --bin zap lockfile_security_tests --locked >/dev/null
ZAP="$NATIVE_DIR/target/release/zap"
PROJECT="$WORK/project"
CLEAN="$WORK/clean"
mkdir -p "$PROJECT" "$CLEAN"

cat > "$PROJECT/zap.toml" <<'MANIFEST'
[package]
name = "clean-machine-fixture"
version = "1.0.0"
main = "main.zp"

[dependencies]
MANIFEST
cat > "$PROJECT/main.zp" <<'PROGRAM'
say "clean machine locked fixture"
PROGRAM

(
  cd "$PROJECT"
  "$ZAP" lock >/tmp/zap-clean-lock.out
  "$ZAP" install --locked >/tmp/zap-clean-install.out
  "$ZAP" build --locked >/tmp/zap-clean-build.out
)
grep -Fq 'installed 0 locked dependencies' /tmp/zap-clean-install.out
grep -Fq 'built Zap project' /tmp/zap-clean-build.out

cp -a "$PROJECT/." "$CLEAN/"
(
  cd "$CLEAN"
  env -u ZAP_REGISTRY_INDEX -u ZAP_CACHE_DIR -u ZAP_OFFLINE \
    "$ZAP" install --locked >/tmp/zap-clean-copy-install.out
  env -u ZAP_REGISTRY_INDEX -u ZAP_CACHE_DIR -u ZAP_OFFLINE \
    "$ZAP" build --locked >/tmp/zap-clean-copy-build.out
)
grep -Fq 'installed 0 locked dependencies' /tmp/zap-clean-copy-install.out
grep -Fq 'built Zap project' /tmp/zap-clean-copy-build.out

cp "$CLEAN/zap.lock" "$WORK/zap.lock.valid"
printf '%s\n' '# tampered' >> "$CLEAN/zap.lock"
if (cd "$CLEAN" && "$ZAP" install --locked >/tmp/zap-clean-tampered.out 2>&1); then
  echo 'expected tampered lockfile rejection did not occur' >&2
  exit 1
fi
grep -Fq 'zap.lock:' /tmp/zap-clean-tampered.out

cp "$WORK/zap.lock.valid" "$CLEAN/zap.lock"
(cd "$CLEAN" && "$ZAP" install --locked >/tmp/zap-clean-restored.out)
grep -Fq 'installed 0 locked dependencies' /tmp/zap-clean-restored.out

echo 'clean-machine locked verification passed'
