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

shell_quote() {
  local value="$1"
  value=${value//\'/\'\\\'\'}
  printf "'%s'" "$value"
}

remove_path() {
  local file="$1"
  [ -f "$file" ] || return 0
  local marker="# Zap CLI: $USER_BIN"
  local path_line="export PATH=$(shell_quote "$USER_BIN"):\"\$PATH\""
  local temp="${file}.zap-tmp.$$"
  awk -v marker="$marker" -v path_line="$path_line" '
    $0 == marker {
      if (getline next_line > 0) {
        if (next_line != path_line) {
          print $0
          print next_line
        }
      } else {
        print $0
      }
      next
    }
    { print }
  ' "$file" > "$temp"
  chmod --reference="$file" "$temp"
  mv "$temp" "$file"
}

case "${SHELL##*/}" in
  zsh) remove_path "$HOME/.zshrc" ;;
  *) remove_path "$HOME/.bashrc" ;;
esac

echo "Zap uninstall completed."
