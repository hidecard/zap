#!/usr/bin/env bash
set -euo pipefail

USER_BIN="${ZAP_INSTALL_DIR:-$HOME/.local/bin}"
TARGET="$USER_BIN/zap"

if [ -e "$TARGET" ]; then
  rm -f "$TARGET"
  echo "Removed Zap binary: $TARGET"
else
  echo "Zap binary not installed at $TARGET"
fi

remove_path() {
  local file="$1"
  [ -f "$file" ] || return 0
  sed -i.bak '/^# Zap CLI$/N;/^# Zap CLI\nexport PATH="\$HOME\/\.local\/bin:\$PATH"$/d' "$file"
  rm -f "$file.bak"
}

case "${SHELL##*/}" in
  zsh) remove_path "$HOME/.zshrc" ;;
  *) remove_path "$HOME/.bashrc" ;;
esac

echo "Zap uninstall completed."
