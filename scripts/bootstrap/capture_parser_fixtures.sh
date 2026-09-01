#!/usr/bin/env bash
# Capture parser-corpus golden fixtures from the Rust reference.
#
# This script regenerates the `*.ast.json` and `*.diagnostics.json` companion
# files for every parser-corpus `.zp` source under `bootstrap/fixtures/parser/`.
# It must be run on a host where the Rust reference is buildable — either with
# `cargo` and a populated `~/.rustup/toolchains` directory, or with a prebuilt
# `native/target/release/zap` binary.
#
# Invocation:
#   bash scripts/bootstrap/capture_parser_fixtures.sh [--check]
#
# `--check` (default): regenerate every missing expected-output JSON, leave
#                     already-present files untouched.
# `--overwrite`:       regenerate every JSON, replacing existing files.
# `--only BOOT-051,...`: only regenerate the named BOOT rules
#                        (BOOT-051 through BOOT-057 cover the current gaps).
#
# The sandbox used for plan authoring cannot run this script: see
# `bootstrap/BOOTSTRAP_ADVANCEMENT_EVIDENCE.md` "Blocker: golden fixtures
# require Rust reference runner".

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FIXTURE_DIR="${ROOT_DIR}/bootstrap/fixtures/parser"
MANIFEST_PATH="${ROOT_DIR}/native/Cargo.toml"
BINARY_PATH="${ROOT_DIR}/target/release/zap"

mode="check"
only_rules=()
while [[ $# -gt 0 ]]; do
    case "$1" in
        --check)
            mode="check"
            ;;
        --overwrite)
            mode="overwrite"
            ;;
        --only)
            shift
            while [[ $# -gt 0 && "$1" != --* ]]; do
                only_rules+=("$1")
                shift
            done
            continue
            ;;
        *)
            printf 'unknown argument: %s\n' "$1" >&2
            exit 2
            ;;
    esac
    shift
done

if [[ ! -d "$FIXTURE_DIR" ]]; then
    printf 'missing fixture directory: %s\n' "$FIXTURE_DIR" >&2
    exit 2
fi

# Locate the runner.
runner=""
if [[ -x "$BINARY_PATH" ]]; then
    runner="$BINARY_PATH"
elif command -v cargo >/dev/null 2>&1 && [[ -f "$MANIFEST_PATH" ]]; then
    runner="cargo"
fi

if [[ -z "$runner" ]]; then
    printf 'cannot locate a runner: build native first or set --binary\n' >&2
    printf '  expected prebuilt: %s\n' "$BINARY_PATH" >&2
    printf '  or cargo with manifest: %s\n' "$MANIFEST_PATH" >&2
    exit 3
fi

# Helper: invoke the Rust reference and write JSON to $2.
capture_ast() {
    local source_path="$1"
    local target_path="$2"
    if [[ "$runner" == "cargo" ]]; then
        cargo run --quiet --release --locked --manifest-path "$MANIFEST_PATH" -- bootstrap ast "$source_path"
    else
        "$runner" bootstrap ast "$source_path"
    fi > "$target_path"
}

capture_diagnostics() {
    local source_path="$1"
    local target_path="$2"
    if [[ "$runner" == "cargo" ]]; then
        cargo run --quiet --release --locked --manifest-path "$MANIFEST_PATH" -- bootstrap diagnostics "$source_path"
    else
        "$runner" bootstrap diagnostics "$source_path"
    fi > "$target_path"
}

# Maps a BOOT rule ID to the expected-output path under bootstrap/fixtures/parser/.
declare -A BOOT_TO_OUTPUT=(
    ["BOOT-051"]="arbitrary_complex_call.ast.json"
    ["BOOT-052"]="arbitrary_deep_nesting.ast.json"
    ["BOOT-053"]="arbitrary_nested_expressions.ast.json"
    ["BOOT-054"]="malformed_recovery.diagnostics.json"
    ["BOOT-055"]="multi_diagnostic.diagnostics.json"
    ["BOOT-056"]="numeric_literals.ast.json"
    ["BOOT-057"]="span_coverage.ast.json"
)

# If --only was supplied, restrict work to the named rules.
if [[ ${#only_rules[@]} -gt 0 ]]; then
    selected_files=()
    for rule in "${only_rules[@]}"; do
        if [[ -z "${BOOT_TO_OUTPUT[$rule]:-}" ]]; then
            printf 'unknown BOOT rule: %s\n' "$rule" >&2
            exit 2
        fi
        selected_files+=("${BOOT_TO_OUTPUT[$rule]}")
    done
else
    selected_files=("${BOOT_TO_OUTPUT[@]}")
fi

written=0
skipped=0
failed=0

for output in "${selected_files[@]}"; do
    target_path="${FIXTURE_DIR}/${output}"
    base="${output%.*}"
    # Determine the kind from the suffix.
    case "$output" in
        *.ast.json)
            kind="ast"
            source_extension=".zp"
            ;;
        *.diagnostics.json)
            kind="diagnostics"
            source_extension=".zp"
            ;;
        *)
            printf 'unrecognised output kind: %s\n' "$output" >&2
            failed=$((failed + 1))
            continue
            ;;
    esac
    source_path="${FIXTURE_DIR}/${base}${source_extension}"
    if [[ ! -f "$source_path" ]]; then
        printf 'missing source for %s: %s\n' "$output" "$source_path" >&2
        failed=$((failed + 1))
        continue
    fi
    if [[ "$mode" == "check" && -f "$target_path" ]]; then
        skipped=$((skipped + 1))
        continue
    fi
    if ! capture_"$kind" "$source_path" "$target_path"; then
        printf 'capture failed for %s\n' "$output" >&2
        failed=$((failed + 1))
        continue
    fi
    written=$((written + 1))
    printf 'captured %s -> %s\n' "$source_path" "$target_path"
done

printf 'capture_parser_fixtures: %d written, %d skipped, %d failed\n' "$written" "$skipped" "$failed"

if [[ "$failed" -gt 0 ]]; then
    exit 1
fi