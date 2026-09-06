#!/usr/bin/env python3
"""Run reference differential test matrix using Python instead of jq."""
import json
import os
import subprocess
import sys

ROOT_DIR = os.getcwd()
ZAP_BIN = os.path.join(ROOT_DIR, "native", "target", "release", "zap.exe")

def run_zap(*args):
    result = subprocess.run(
        [ZAP_BIN] + list(args),
        capture_output=True,
        text=True,
    )
    return result.stdout

def check_json(output, checks):
    try:
        data = json.loads(output)
    except json.JSONDecodeError as e:
        print(f"FAIL: JSON parse error: {e}")
        return False
    
    for check in checks:
        if check == "ast":
            if data.get("kind") != "zap.ast":
                print(f"FAIL: kind is {data.get('kind')}, expected zap.ast")
                return False
            if not isinstance(data.get("schema_version"), (int, float)):
                print(f"FAIL: schema_version is not a number")
                return False
            if not isinstance(data.get("ast"), dict):
                print(f"FAIL: ast is not an object")
                return False
        elif check == "diagnostics":
            if data.get("kind") != "zap.diagnostics":
                print(f"FAIL: kind is {data.get('kind')}, expected zap.diagnostics")
                return False
            if not isinstance(data.get("schema_version"), (int, float)):
                print(f"FAIL: schema_version is not a number")
                return False
            if not isinstance(data.get("diagnostics"), list):
                print(f"FAIL: diagnostics is not an array")
                return False
        elif check == "typed_ir":
            if data.get("kind") != "zap.typed_ir":
                print(f"FAIL: kind is {data.get('kind')}, expected zap.typed_ir")
                return False
            if not isinstance(data.get("schema_version"), (int, float)):
                print(f"FAIL: schema_version is not a number")
                return False
            if not isinstance(data.get("ir"), dict):
                print(f"FAIL: ir is not an object")
                return False
    return True

parser_fixtures = [
    "bootstrap/fixtures/parser/arithmetic.zp",
    "bootstrap/fixtures/parser/compound.zp",
    "bootstrap/fixtures/parser/parenthesized_nested.zp",
    "bootstrap/fixtures/parser/nested_calls.zp",
    "bootstrap/fixtures/parser/simple_loop.zp",
    "bootstrap/fixtures/parser/simple_function.zp",
    "bootstrap/fixtures/parser/control_flow.zp",
    "bootstrap/fixtures/parser/assignment_statement.zp",
    "bootstrap/fixtures/parser/bool_literals.zp",
    "bootstrap/fixtures/parser/text_escape.zp",
    "bootstrap/fixtures/parser/module_import.zp",
    "bootstrap/fixtures/parser/try_catch_simple.zp",
    "bootstrap/fixtures/parser/await_expression.zp",
    "bootstrap/fixtures/parser/option_constructors.zp",
    "bootstrap/fixtures/parser/result_constructors.zp",
]

diagnostic_fixtures = [
    "bootstrap/fixtures/diagnostics/missing_closing_bracket.zp",
    "bootstrap/fixtures/diagnostics/invalid_character.zp",
    "bootstrap/fixtures/diagnostics/unterminated_string.zp",
    "bootstrap/fixtures/diagnostics/integer_overflow.zp",
    "bootstrap/fixtures/parser/invalid_indentation.zp",
    "bootstrap/fixtures/parser/invalid_indentation_jump.zp",
    "bootstrap/fixtures/parser/malformed_recovery.zp",
    "bootstrap/fixtures/parser/numeric_literals.zp",
]

typed_fixtures = [
    "bootstrap/fixtures/typecheck/expression_number_add.zp",
    "bootstrap/fixtures/typecheck/expression_text_add.zp",
    "bootstrap/fixtures/typecheck/expression_boolean_logic.zp",
    "bootstrap/fixtures/typecheck/list_annotation.zp",
    "bootstrap/fixtures/typecheck/map_annotation.zp",
    "bootstrap/fixtures/typecheck/generic_identity.zp",
    "bootstrap/fixtures/typecheck/generic_nested_option_list.zp",
    "bootstrap/fixtures/typecheck/conditional.zp",
    "bootstrap/fixtures/typecheck/bool_annotation.zp",
    "bootstrap/fixtures/typecheck/none_annotation.zp",
    "bootstrap/fixtures/typecheck/loop_narrowing.zp",
    "bootstrap/fixtures/typecheck/branch_narrowing.zp",
    "bootstrap/fixtures/typecheck/short_circuit_or.zp",
    "bootstrap/fixtures/typecheck/function.zp",
    "bootstrap/fixtures/typecheck/incompatible.zp",
]

count = 0
failures = 0

for fixture in parser_fixtures:
    base = os.path.basename(fixture)
    out1 = run_zap("bootstrap", "ast", fixture)
    out2 = run_zap("bootstrap", "ast", fixture)
    if out1 != out2:
        print(f"FAIL: {base} not deterministic")
        failures += 1
        continue
    if not check_json(out1, ["ast"]):
        failures += 1
        continue
    count += 1

for fixture in diagnostic_fixtures:
    base = os.path.basename(fixture)
    out1 = run_zap("bootstrap", "diagnostics", fixture)
    out2 = run_zap("bootstrap", "diagnostics", fixture)
    if out1 != out2:
        print(f"FAIL: {base} not deterministic")
        failures += 1
        continue
    if not check_json(out1, ["diagnostics"]):
        failures += 1
        continue
    count += 1

for fixture in typed_fixtures:
    base = os.path.basename(fixture)
    out1 = run_zap("bootstrap", "typed-ir", fixture)
    out2 = run_zap("bootstrap", "typed-ir", fixture)
    if out1 != out2:
        print(f"FAIL: {base} not deterministic")
        failures += 1
        continue
    if not check_json(out1, ["typed_ir"]):
        failures += 1
        continue
    count += 1

if failures > 0:
    print(f"Reference differential matrix FAILED: {failures} issue(s)")
    sys.exit(1)
else:
    print(f"Reference differential matrix passed: {count} deterministic parser/diagnostic/typed-IR cases")
