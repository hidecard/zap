#!/usr/bin/env bash
# Capture bootstrap-corpus golden fixtures from the Rust reference.
#
# This script regenerates the expected-output JSON companions for every
# `.zp` source under `bootstrap/fixtures/parser/` and `bootstrap/fixtures/lexer/`
# (`.tokens.json`, `.ast.json`, `.diagnostics.json`, `.typed-ir.json`).
# It must be run on a host where the Rust reference is buildable — either with
# `cargo` and a populated `~/.rustup/toolchains` directory, or with a prebuilt
# `native/target/release/zap` binary.
#
# Invocation:
#   bash scripts/bootstrap/capture_parser_fixtures.sh [--check] [--family=...]
#
# `--check` (default): regenerate every missing expected-output JSON, leave
#                     already-present files untouched.
# `--overwrite`:       regenerate every JSON, replacing existing files.
# `--only BOOT-051,...`: only regenerate the named BOOT rules
#                        (BOOT-051 through BOOT-071 cover the current gaps).
# `--family=parser|lexer|typedir|diagnostics|tokens|all` (default: all):
#                        restrict the capture to a single artifact family.
#
# The sandbox used for plan authoring cannot run this script: see
# `bootstrap/BOOTSTRAP_ADVANCEMENT_EVIDENCE.md` "Blocker: golden fixtures
# require Rust reference runner".

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PARSER_DIR="${ROOT_DIR}/bootstrap/fixtures/parser"
LEXER_DIR="${ROOT_DIR}/bootstrap/fixtures/lexer"
MANIFEST_PATH="${ROOT_DIR}/native/Cargo.toml"
BINARY_PATH="${ROOT_DIR}/target/release/zap"

mode="check"
family="all"
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
        --family=*)
            family="${1#*=}"
            if [[ "$family" != "all" && "$family" != "parser" && "$family" != "lexer" && "$family" != "typedir" && "$family" != "diagnostics" && "$family" != "tokens" ]]; then
                printf 'unknown --family value: %s (allowed: all|parser|lexer|typedir|diagnostics|tokens)\n' "$family" >&2
                exit 2
            fi
            ;;
        *)
            printf 'unknown argument: %s\n' "$1" >&2
            exit 2
            ;;
    esac
    shift
done

if [[ ! -d "$PARSER_DIR" ]]; then
    printf 'missing parser fixture directory: %s\n' "$PARSER_DIR" >&2
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

capture_typed_ir() {
    local source_path="$1"
    local target_path="$2"
    if [[ "$runner" == "cargo" ]]; then
        cargo run --quiet --release --locked --manifest-path "$MANIFEST_PATH" -- bootstrap typed-ir "$source_path"
    else
        "$runner" bootstrap typed-ir "$source_path"
    fi > "$target_path"
}

capture_tokens() {
    local source_path="$1"
    local target_path="$2"
    if [[ "$runner" == "cargo" ]]; then
        cargo run --quiet --release --locked --manifest-path "$MANIFEST_PATH" -- bootstrap tokens "$source_path"
    else
        "$runner" bootstrap tokens "$source_path"
    fi > "$target_path"
}

# Maps a BOOT rule ID to (source-dir, source-base, expected-output-name, kind).
declare -A BOOT_TO_OUTPUT=(
    ["BOOT-051"]="${PARSER_DIR}:arbitrary_complex_call:arbitrary_complex_call.ast.json:ast"
    ["BOOT-052"]="${PARSER_DIR}:arbitrary_deep_nesting:arbitrary_deep_nesting.ast.json:ast"
    ["BOOT-053"]="${PARSER_DIR}:arbitrary_nested_expressions:arbitrary_nested_expressions.ast.json:ast"
    ["BOOT-054"]="${PARSER_DIR}:malformed_recovery:malformed_recovery.diagnostics.json:diagnostics"
    ["BOOT-055"]="${PARSER_DIR}:multi_diagnostic:multi_diagnostic.diagnostics.json:diagnostics"
    ["BOOT-056"]="${PARSER_DIR}:numeric_literals:numeric_literals.ast.json:ast"
    ["BOOT-057"]="${PARSER_DIR}:span_coverage:span_coverage.ast.json:ast"
    ["BOOT-058"]="${LEXER_DIR}:delimiters:delimiters.ast.json:ast"
    ["BOOT-059"]="${LEXER_DIR}:delimiters:delimiters.typed-ir.json:typedir"
    ["BOOT-060"]="${LEXER_DIR}:operators:operators.ast.json:ast"
    ["BOOT-061"]="${LEXER_DIR}:operators:operators.typed-ir.json:typedir"
    ["BOOT-062"]="${LEXER_DIR}:unicode:unicode.ast.json:ast"
    ["BOOT-063"]="${LEXER_DIR}:unicode:unicode.typed-ir.json:typedir"
    ["BOOT-064"]="${PARSER_DIR}:generic_nested_option_list:generic_nested_option_list.typed-ir.json:typedir"
    ["BOOT-065"]="${PARSER_DIR}:generic_scope_external_incompatible:generic_scope_external_incompatible.typed-ir.json:typedir"
    ["BOOT-066"]="${PARSER_DIR}:generic_scope_parameter_incompatible:generic_scope_parameter_incompatible.typed-ir.json:typedir"
    ["BOOT-067"]="${PARSER_DIR}:generic_cross_module_body:generic_cross_module_body.typed-ir.json:typedir"
    ["BOOT-068"]="${PARSER_DIR}:generic_compound_bounds:generic_compound_bounds.typed-ir.json:typedir"
    ["BOOT-069"]="${PARSER_DIR}:generic_explicit_call_deferred:generic_explicit_call_deferred.typed-ir.json:typedir"
    ["BOOT-070"]="${PARSER_DIR}:generic_alias_deferred:generic_alias_deferred.typed-ir.json:typedir"
)

# Filter by family if requested.
family_filter() {
    local kind="$1"
    case "$family" in
        all) return 0 ;;
        parser) [[ "$kind" == "ast" || "$kind" == "diagnostics" ]] ;;
        lexer) [[ "$kind" == "tokens" ]] ;;
        typedir) [[ "$kind" == "typedir" ]] ;;
        diagnostics) [[ "$kind" == "diagnostics" ]] ;;
        tokens) [[ "$kind" == "tokens" ]] ;;
    esac
}

# If --only was supplied, restrict work to the named rules.
if [[ ${#only_rules[@]} -gt 0 ]]; then
    selected_keys=()
    for rule in "${only_rules[@]}"; do
        if [[ -z "${BOOT_TO_OUTPUT[$rule]:-}" ]]; then
            printf 'unknown BOOT rule: %s\n' "$rule" >&2
            exit 2
        fi
        selected_keys+=("$rule")
    done
else
    selected_keys=("${!BOOT_TO_OUTPUT[@]}")
fi

written=0
skipped=0
failed=0

for rule in "${selected_keys[@]}"; do
    IFS=':' read -r fixture_dir base output kind <<< "${BOOT_TO_OUTPUT[$rule]}"
    if ! family_filter "$kind"; then
        continue
    fi
    target_path="${fixture_dir}/${output}"
    source_path="${fixture_dir}/${base}.zp"
    if [[ ! -f "$source_path" ]]; then
        printf 'missing source for %s: %s\n' "$rule" "$source_path" >&2
        failed=$((failed + 1))
        continue
    fi
    if [[ "$mode" == "check" && -f "$target_path" ]]; then
        skipped=$((skipped + 1))
        continue
    fi
    if ! capture_"$kind" "$source_path" "$target_path"; then
        printf 'capture failed for %s\n' "$rule" >&2
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