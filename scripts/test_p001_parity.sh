#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MANIFEST="$ROOT_DIR/conformance/p0-01/matrix.tsv"
REPORT="${ZAP_PARITY_REPORT:-$ROOT_DIR/target/p001-parity-report.tsv}"
LEGACY_PYTHON="${ZAP_LEGACY_PYTHON:-python3}"
WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT

resolve_native() {
  local override="${ZAP_BIN_OVERRIDE:-${ZAP_BIN:-}}"
  local candidate

  if [[ -n "$override" ]]; then
    if [[ -x "$override" ]]; then
      printf '%s\n' "$override"
      return 0
    fi
    if [[ -x "$override.exe" ]]; then
      printf '%s\n' "$override.exe"
      return 0
    fi
    printf 'parity: native binary is not executable: %s\n' "$override" >&2
    return 1
  fi

  for candidate in \
    "$ROOT_DIR/native/target/release/zap" \
    "$ROOT_DIR/native/target/release/zap.exe" \
    "$ROOT_DIR/native/target/debug/zap" \
    "$ROOT_DIR/native/target/debug/zap.exe" \
    "$ROOT_DIR/bin/zap" \
    "$ROOT_DIR/bin/zap.exe"
  do
    if [[ -x "$candidate" ]]; then
      printf '%s\n' "$candidate"
      return 0
    fi
  done

  if command -v cargo >/dev/null 2>&1; then
    cargo build --locked --manifest-path "$ROOT_DIR/native/Cargo.toml" --bin zap
    for candidate in \
      "$ROOT_DIR/native/target/debug/zap" \
      "$ROOT_DIR/native/target/debug/zap.exe" \
      "$ROOT_DIR/native/target/release/zap" \
      "$ROOT_DIR/native/target/release/zap.exe"
    do
      if [[ -x "$candidate" ]]; then
        printf '%s\n' "$candidate"
        return 0
      fi
    done
  fi

  printf 'parity: native binary missing; set ZAP_BIN or build native/Cargo.toml\n' >&2
  return 1
}

sha256_file() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

normalize_output() {
  sed -e 's/\r$//' -e '/^[[:space:]]*$/d' -e 's#Zap [0-9][^ ]* (native)#Zap <version> (native)#g'
}

run_engine() {
  local output="$1"
  local error="$2"
  shift 2
  set +e
  "$@" >"$output" 2>"$error"
  local status=$?
  set -e
  printf '%s\n' "$status"
}

if ! command -v "$LEGACY_PYTHON" >/dev/null 2>&1; then
  printf 'parity: legacy Python interpreter is not available: %s\n' "$LEGACY_PYTHON" >&2
  exit 2
fi

NATIVE="$(resolve_native)"
mkdir -p "$(dirname "$REPORT")"
printf 'id\tpolicy\tfixture\tnative_status\tlegacy_status\tnative_output_sha256\tlegacy_output_sha256\tdecision\n' > "$REPORT"

failures=0
while IFS=$'\t' read -r case_id policy fixture || [[ -n "$case_id" ]]; do
  [[ -z "$case_id" || "$case_id" == \#* ]] && continue
  source="$ROOT_DIR/conformance/p0-01/$fixture"
  if [[ ! -f "$source" ]]; then
    printf 'parity: missing fixture: %s\n' "$fixture" >&2
    exit 2
  fi

  native_out="$WORK_DIR/${case_id}.native.out"
  native_err="$WORK_DIR/${case_id}.native.err"
  legacy_out="$WORK_DIR/${case_id}.legacy.out"
  legacy_err="$WORK_DIR/${case_id}.legacy.err"
  native_source="$source"
  if [[ "$NATIVE" == *.exe && "$source" == /mnt/* ]]; then
    native_source="${source#/mnt/}"
    native_drive="${native_source%%/*}"
    native_rest="${native_source#*/}"
    native_source="${native_drive^^}:\\${native_rest//\//\\}"
  fi
  native_status="$(run_engine "$native_out" "$native_err" "$NATIVE" run "$native_source")"
  legacy_status="$(run_engine "$legacy_out" "$legacy_err" "$LEGACY_PYTHON" "$ROOT_DIR/legacy/zap.py" run "$source")"
  normalize_output < "$native_out" > "$native_out.normalized"
  normalize_output < "$legacy_out" > "$legacy_out.normalized"
  native_digest="$(sha256_file "$native_out.normalized")"
  legacy_digest="$(sha256_file "$legacy_out.normalized")"
  decision=FAIL

  case "$policy" in
    common)
      if [[ "$native_status" == 0 && "$legacy_status" == 0 && "$native_digest" == "$legacy_digest" ]]; then
        decision=PASS
      fi
      ;;
    native-only)
      if [[ "$native_status" == 0 && "$legacy_status" != 0 ]]; then
        decision=PASS
      fi
      ;;
    rejected)
      if [[ "$native_status" != 0 && "$legacy_status" != 0 ]]; then
        decision=PASS
      fi
      ;;
    *)
      printf 'parity: unknown policy `%s` for %s\n' "$policy" "$case_id" >&2
      exit 2
      ;;
  esac

  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$case_id" "$policy" "$fixture" "$native_status" "$legacy_status" \
    "$native_digest" "$legacy_digest" "$decision" >> "$REPORT"
  printf 'p0-01: %s (%s) native=%s legacy=%s decision=%s\n' \
    "$case_id" "$policy" "$native_status" "$legacy_status" "$decision"

  if [[ "$decision" != PASS ]]; then
    failures=$((failures + 1))
    printf 'parity: output drift or policy violation in %s\n' "$case_id" >&2
  fi
done < "$MANIFEST"

if (( failures > 0 )); then
  printf 'p0-01 parity failed: %s case(s); report=%s\n' "$failures" "$REPORT" >&2
  exit 1
fi
printf 'p0-01 parity passed: report=%s\n' "$REPORT"
