#!/usr/bin/env bash
set -euo pipefail

ARCHIVE="${1:?archive path is required}"
EXPECTED_VERSION="${2:?expected version is required}"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

HOME_DIR="$WORK/home"
INSTALL_DIR="$HOME_DIR/.local/bin"
mkdir -p "$HOME_DIR"

if [[ "$ARCHIVE" != /* ]]; then
  ARCHIVE="$(pwd)/$ARCHIVE"
fi

tar -xzf "$ARCHIVE" -C "$WORK"
PACKAGE="$WORK/zap"
[ -x "$PACKAGE/bin/zap" ] || { echo "missing executable in archive" >&2; exit 1; }

HOME="$HOME_DIR" SHELL=/bin/bash ZAP_INSTALL_DIR="$INSTALL_DIR" bash "$PACKAGE/install.sh"
[ -x "$INSTALL_DIR/zap" ] || { echo "installer did not create zap binary" >&2; exit 1; }
VERSION="$($INSTALL_DIR/zap --version)"
grep -F "$EXPECTED_VERSION" <<<"$VERSION" >/dev/null || { echo "installed version mismatch: $VERSION" >&2; exit 1; }
HOME="$HOME_DIR" PATH="$INSTALL_DIR:$PATH" "$INSTALL_DIR/zap" "$PACKAGE/examples/hello.zp" >/dev/null

# Reinstalling the same package is the upgrade contract: it must replace the binary cleanly.
HOME="$HOME_DIR" SHELL=/bin/bash ZAP_INSTALL_DIR="$INSTALL_DIR" bash "$PACKAGE/install.sh" >/dev/null
[ -x "$INSTALL_DIR/zap" ] || { echo "upgrade removed the installed binary" >&2; exit 1; }

HOME="$HOME_DIR" SHELL=/bin/bash ZAP_INSTALL_DIR="$INSTALL_DIR" bash "$PACKAGE/uninstall.sh" >/dev/null
[ ! -e "$INSTALL_DIR/zap" ] || { echo "uninstaller left the Zap binary behind" >&2; exit 1; }
! grep -F 'export PATH="$HOME/.local/bin:$PATH"' "$HOME_DIR/.bashrc" >/dev/null 2>&1 || { echo "uninstaller left Zap PATH entry behind" >&2; exit 1; }

echo "Unix installer verification passed: install, version, execution, upgrade, uninstall"
