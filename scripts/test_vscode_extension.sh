#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(git rev-parse --show-toplevel)"
EXTENSION_DIR="$ROOT_DIR/vscode-extension"
DIST_DIR="$EXTENSION_DIR/dist"
EXPECTED_VERSION="$(sed -n 's/^version[[:space:]]*=[[:space:]]*"\([^"]*\)"[[:space:]]*$/\1/p' "$ROOT_DIR/native/Cargo.toml" | head -n 1)"

cd "$ROOT_DIR"
trap 'rm -rf "$DIST_DIR"' EXIT

node "$EXTENSION_DIR/scripts/test-extension.js"
node "$EXTENSION_DIR/scripts/package-extension.js"

archive="$DIST_DIR/zap-language-support-${EXPECTED_VERSION}.vsix"
test -s "$archive"
unzip -t "$archive" >/dev/null

for entry in \
  package.json \
  extension.js \
  lsp-client.js \
  language-configuration.json \
  syntaxes/zap.tmLanguage.json \
  snippets/zap.json \
  README.md \
  README_MM.md; do
  unzip -Z1 "$archive" | grep -Fxq "$entry" || {
    echo "missing packaged extension entry: $entry" >&2
    exit 1
  }
done

if unzip -Z1 "$archive" | grep -Eq '(^|/)\.git(/|$)|(^|/)dist(/|$)'; then
  echo "packaged extension contains generated or VCS files" >&2
  exit 1
fi

printf 'VS Code canonical extension package contract passed: %s\n' "$archive"
