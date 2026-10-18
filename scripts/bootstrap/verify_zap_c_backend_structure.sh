#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

echo "Verifying Zap-owned C backend structure..."

# Check if the Zap C backend module exists
if [ ! -f "bootstrap/b4/c_backend.zp" ]; then
    echo "FAIL: Zap C backend module not found"
    exit 1
fi

echo "PASS: Zap C backend module exists"

for required in \
    'export fn c_backend_validate_bytecode(' \
    'export fn c_backend_emit(' \
    'export fn c_backend_ownership(' \
    'zap_owned_scalar_backend'; do
    if ! grep -Fq "$required" bootstrap/b4/c_backend.zp; then
        echo "FAIL: Zap C backend missing required ownership slice: $required"
        exit 1
    fi
done

echo "PASS: Zap C backend validation and ownership slice present"

# Try to compile the Zap C backend module with the current Zap binary
ZAP_BIN="${ZAP_BIN:-${ZAP_BOOTSTRAP_BIN:-$ROOT_DIR/bin/zap}}"
if [ ! -x "$ZAP_BIN" ] && [ -x "$ZAP_BIN.exe" ]; then
    ZAP_BIN="$ZAP_BIN.exe"
fi
if [ ! -x "$ZAP_BIN" ]; then
    echo "SKIP: No Zap binary available for compilation test"
    exit 0
fi

# Create a simple test that imports the C backend module
TEST_FILE=$(mktemp ".zap-c-backend.XXXXXX.zp")
trap 'rm -f "$TEST_FILE"' EXIT

cat > "$TEST_FILE" <<'EOF'
import "bootstrap/b4/c_backend.zp"
let test_bytecode = [{"op": "const", "value": true}, {"op": "dup"}, {"op": "const", "value": false}, {"op": "and"}, {"op": "const", "value": false}, {"op": "not"}, {"op": "or"}, {"op": "halt"}]
let result = c_backend_generate(test_bytecode)
let unknown = c_backend_validate_bytecode([{"op": "future_opcode"}])
let malformed = c_backend_validate_bytecode([{"op": "const"}])
let string_validation = c_backend_validate_bytecode([{"op": "const", "value": "hello"}])
let string_runtime = c_backend_emit([{"op": "const", "value": "hello"}, {"op": "const", "value": " world"}, {"op": "str_concat"}, {"op": "print"}, {"op": "halt"}])
let mixed_runtime = c_backend_emit([{"op": "const", "value": "hello"}, {"op": "const", "value": " world"}, {"op": "add"}, {"op": "pop"}, {"op": "const", "value": 1}, {"op": "const", "value": 2}, {"op": "make_list", "count": 2}, {"op": "dup"}, {"op": "const", "value": 2}, {"op": "list_contains"}, {"op": "pop"}, {"op": "list_reverse"}, {"op": "halt"}])
let control_flow = c_backend_emit([{"op": "const", "value": true}, {"op": "jump_if_false", "target": 4}, {"op": "jump", "target": 4}, {"op": "halt"}])
let list_runtime = c_backend_emit([{"op": "const", "value": 10}, {"op": "const", "value": 20}, {"op": "make_list", "count": 2}, {"op": "dup"}, {"op": "list_len"}, {"op": "pop"}, {"op": "const", "value": 0}, {"op": "list_get"}, {"op": "pop"}, {"op": "const", "value": 1}, {"op": "const", "value": 30}, {"op": "list_set"}, {"op": "const", "value": 40}, {"op": "list_append"}, {"op": "halt"}])
let map_runtime = c_backend_emit([{"op": "make_map"}, {"op": "const", "value": 1}, {"op": "const", "value": 10}, {"op": "map_set_pair"}, {"op": "dup"}, {"op": "const", "value": 1}, {"op": "map_get"}, {"op": "pop"}, {"op": "dup"}, {"op": "map_keys"}, {"op": "pop"}, {"op": "dup"}, {"op": "map_values"}, {"op": "pop"}, {"op": "const", "value": 2}, {"op": "const", "value": 20}, {"op": "map_set"}, {"op": "struct_new"}, {"op": "const", "value": 3}, {"op": "const", "value": 30}, {"op": "struct_set"}, {"op": "halt"}])
say "Generated C lines: " + str(len(result))
say "First line: " + result[0]
say "Has halt: " + str(contains(result, "  return 0;"))
say "Has dup: " + str(contains(result, "  { long value = zap_pop(); zap_push(value); zap_push(value); }"))
say "Has and: " + str(contains(result, "  { long rhs = zap_pop(); long lhs = zap_pop(); zap_push(lhs && rhs); }"))
say "Has not: " + str(contains(result, "  { long value = zap_pop(); zap_push(!value); }"))
say "Has or: " + str(contains(result, "  { long rhs = zap_pop(); long lhs = zap_pop(); zap_push(lhs || rhs); }"))
say "Rejects unknown: " + str(unknown["status"] == "rejected")
say "Rejects malformed const: " + str(malformed["status"] == "rejected")
say "Accepts string validation: " + str(string_validation["status"] == "validated")
say "Emits string runtime: " + str(string_runtime["status"] != "rejected" and contains(json(string_runtime["lines"]), "static long zap_string_new") and contains(json(string_runtime["lines"]), "zap_string_concat(lhs, rhs)") and contains(json(string_runtime["lines"]), "zap_is_string(value)"))
say "Emits mixed runtime: " + str(mixed_runtime["status"] != "rejected" and contains(json(mixed_runtime["lines"]), "zap_add(lhs, rhs)") and contains(json(mixed_runtime["lines"]), "zap_value_equal(zap_lists[list][index], needle)") and contains(json(mixed_runtime["lines"]), "zap_list_append(reversed"))
say "Emits jump_if_false: " + str(contains(control_flow["lines"], "  if (!zap_pop()) goto zap_label_4;"))
say "Emits jump: " + str(contains(control_flow["lines"], "  goto zap_label_4;"))
say "Emits list runtime: " + str(contains(json(list_runtime["lines"]), "static long zap_list_get(long handle, long index)") and contains(json(list_runtime["lines"]), "zap_list_len(zap_pop())") and contains(json(list_runtime["lines"]), "zap_list_append(handle, value)"))
say "Emits map/object runtime: " + str(contains(json(map_runtime["lines"]), "static void zap_map_set(long handle, long key, long value)") and contains(json(map_runtime["lines"]), "zap_map_keys_list(zap_pop())") and contains(json(map_runtime["lines"]), "zap_map_values_list(zap_pop())"))
EOF

ZAP_TEST_FILE="$TEST_FILE"

if output=$("$ZAP_BIN" "$TEST_FILE" 2>&1); then
    printf '%s\n' "$output"
    for expected in 'Has halt: true' 'Has dup: true' 'Has and: true' 'Has not: true' 'Has or: true' 'Rejects unknown: true' 'Rejects malformed const: true' 'Accepts string validation: true' 'Emits string runtime: true' 'Emits mixed runtime: true' 'Emits jump_if_false: true' 'Emits jump: true' 'Emits list runtime: true' 'Emits map/object runtime: true'; do
        if ! grep -Fq "$expected" <<<"$output"; then
            echo "FAIL: C backend boolean emission check missing: $expected"
            exit 1
        fi
    done
    echo "PASS: Zap C backend module compiles successfully"
else
    printf '%s\n' "$output" >&2
    echo "FAIL: Zap C backend module compilation failed"
    exit 1
fi

echo "Zap-owned C backend structure verification passed"
