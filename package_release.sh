#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET="${1:-x86_64-unknown-linux-gnu}"
if [[ -n "${ZAP_VERSION:-}" ]]; then
  VERSION="$ZAP_VERSION"
else
  VERSION="$(awk -F'"' '/^version = "/ { print $2; exit }' "$ROOT/native/Cargo.toml")"
fi
if [[ -z "$VERSION" ]]; then
  echo "Could not determine the package version from native/Cargo.toml" >&2
  exit 1
fi
case "$TARGET" in
  x86_64-unknown-linux-gnu) ARCHIVE="zap-$VERSION-linux-x86_64.tar.gz"; BINARY="zap" ;;
  aarch64-apple-darwin) ARCHIVE="zap-$VERSION-macos-arm64.tar.gz"; BINARY="zap" ;;
  x86_64-apple-darwin) ARCHIVE="zap-$VERSION-macos-x86_64.tar.gz"; BINARY="zap" ;;
  x86_64-pc-windows-msvc) ARCHIVE="zap-$VERSION-windows-x86_64.zip"; BINARY="zap.exe" ;;
  *) echo "Unsupported target: $TARGET" >&2; exit 2 ;;
esac
SOURCE="$ROOT/native/target/$TARGET/release/$BINARY"
if [ ! -f "$SOURCE" ]; then echo "Missing prebuilt binary: $SOURCE" >&2; echo "Build it before packaging or use the CI release workflow." >&2; exit 1; fi
DIST="$ROOT/dist/$TARGET"
rm -rf "$DIST"
mkdir -p "$DIST/zap-$VERSION/bin"
cp "$SOURCE" "$DIST/zap-$VERSION/$BINARY"
if [ "$TARGET" != "x86_64-pc-windows-msvc" ]; then mv "$DIST/zap-$VERSION/$BINARY" "$DIST/zap-$VERSION/bin/zap"; else mv "$DIST/zap-$VERSION/$BINARY" "$DIST/zap-$VERSION/bin/zap.exe"; fi
if [ "$TARGET" = "x86_64-pc-windows-msvc" ]; then
  cp "$ROOT/install_windows.bat" "$ROOT/README.md" "$ROOT/README_MM.md" "$ROOT/LICENSE" "$ROOT/CHANGELOG.md" "$ROOT/CHANGELOG_EN.md" "$ROOT/CHANGELOG_MM.md" "$DIST/zap-$VERSION/"
  cp -R "$ROOT/assets" "$DIST/zap-$VERSION/"
  cp -R "$ROOT/docs" "$ROOT/examples" "$DIST/zap-$VERSION/"
else
  cp "$ROOT/install.sh" "$ROOT/README.md" "$ROOT/README_MM.md" "$ROOT/LICENSE" "$ROOT/CHANGELOG.md" "$ROOT/CHANGELOG_EN.md" "$ROOT/CHANGELOG_MM.md" "$DIST/zap-$VERSION/"
  cp -R "$ROOT/assets" "$DIST/zap-$VERSION/"
  cp -R "$ROOT/docs" "$ROOT/examples" "$DIST/zap-$VERSION/"
fi
chmod 0755 "$DIST/zap-$VERSION/bin/$([ "$TARGET" = "x86_64-pc-windows-msvc" ] && echo zap.exe || echo zap)"
if [[ "$ARCHIVE" == *.zip ]]; then
  (cd "$DIST" && zip -qr "$ARCHIVE" "zap-$VERSION")
else
  (cd "$DIST" && tar -czf "$ARCHIVE" "zap-$VERSION")
fi
sha256sum "$DIST/$ARCHIVE" > "$DIST/$ARCHIVE.sha256"
python3 "$ROOT/scripts/validate_release_archive.py" "$DIST/$ARCHIVE"
echo "Created $DIST/$ARCHIVE"
cat "$DIST/$ARCHIVE.sha256"
