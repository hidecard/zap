#!/usr/bin/env bash
set -euo pipefail
RUN_ID="${1:?usage: verify_release_artifacts.sh RUN_ID}"
OUT="/tmp/zap-gh-artifacts-${RUN_ID}"
rm -rf "$OUT"
mkdir -p "$OUT"
gh run download "$RUN_ID" --dir "$OUT"
find "$OUT" -maxdepth 3 -type f -printf '%P\n' | sort
while IFS= read -r checksum; do
  echo "--- $checksum"
  cat "$checksum"
done < <(find "$OUT" -type f -name '*.sha256' | sort)
while IFS= read -r archive; do
  echo "--- ARCHIVE $archive"
  case "$archive" in
    *.tar.gz) tar -tzf "$archive" | grep -E 'bin/zap|CHANGELOG.md|README.md' | head -20 ;;
    *.zip) unzip -l "$archive" | grep -E 'bin/zap|CHANGELOG.md|README.md' | head -20 ;;
  esac
done < <(find "$OUT" -type f \( -name '*.tar.gz' -o -name '*.zip' \) | sort)
