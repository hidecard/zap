#!/usr/bin/env bash
set -euo pipefail

# Standalone Zap native installer.
# Release archive ထဲတွင် bin/zap binary ပါလျှင် တိုက်ရိုက် install လုပ်သည်။
# Binary package မှ install လုပ်သောအခါ အပို development toolchain မလိုပါ။ Source build သည် explicit opt-in ဖြစ်သည်။
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
USER_BIN="${ZAP_INSTALL_DIR:-$HOME/.local/bin}"
BINARY="${SCRIPT_DIR}/bin/zap"

if [ ! -x "$BINARY" ]; then
  if [ "${ZAP_BUILD_FROM_SOURCE:-0}" = "1" ] && command -v cargo >/dev/null 2>&1; then
    echo "Building Zap native runtime from source..."
    cargo build --release --locked --manifest-path "$SCRIPT_DIR/native/Cargo.toml"
    BINARY="$SCRIPT_DIR/native/target/release/zap"
  else
    echo "Prebuilt Zap binary မတွေ့ပါ။ Official binary release archive ကို download လုပ်ပါ။" >&2
    echo "Source build လုပ်လိုပါက ZAP_BUILD_FROM_SOURCE=1 သတ်မှတ်ပြီး Rust toolchain လိုအပ်ပါသည်။" >&2
    exit 1
  fi
fi

mkdir -p "$USER_BIN"
install -m 0755 "$BINARY" "$USER_BIN/zap"

add_path() {
  local file="$1"
  touch "$file"
  if ! grep -Fq 'export PATH="$HOME/.local/bin:$PATH"' "$file"; then
    printf '\n# Zap CLI\nexport PATH="$HOME/.local/bin:$PATH"\n' >> "$file"
  fi
}

case "${SHELL##*/}" in
  zsh) [ -f "$HOME/.zshrc" ] && add_path "$HOME/.zshrc" ;;
  *) [ -f "$HOME/.bashrc" ] && add_path "$HOME/.bashrc" ;;
esac

export PATH="$USER_BIN:$PATH"
echo "Zap native installed globally: $("$USER_BIN/zap" --version)"
echo "Standalone Zap ကို install လုပ်ပြီးပါပြီ။ Terminal အသစ်ဖွင့်ပြီး မည်သည့် folder မှာမဆို 'zap file.zp' ဟု run လုပ်နိုင်ပါသည်။"
if ! command -v zap >/dev/null 2>&1; then
  echo "Current shell အတွက်: export PATH=\"$USER_BIN:\$PATH\""
fi
