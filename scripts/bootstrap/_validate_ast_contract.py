#!/usr/bin/env python3
"""Validate AST golden files against AST_SCHEMA.toml contract."""
import json
import os
import sys

PARSER_DIR = "bootstrap/fixtures/parser"

def read_json_file(path):
    """Read JSON file with auto-detected encoding."""
    encodings = ["utf-8", "utf-16", "utf-16-le", "utf-16-be", "latin-1"]
    for encoding in encodings:
        try:
            with open(path, "r", encoding=encoding) as f:
                return json.load(f)
        except (UnicodeDecodeError, UnicodeError):
            continue
    raise ValueError(f"Could not decode {path} with any supported encoding")

def validate_ast_file(path):
    errors = []
    try:
        data = read_json_file(path)
    except Exception as e:
        return [f"JSON parse error: {e}"]
    
    # Check envelope fields
    for field in ["ast", "kind", "schema_version", "source_name"]:
        if field not in data:
            errors.append(f"missing required envelope field: {field}")
    
    if data.get("kind") != "zap.ast":
        errors.append(f"kind should be 'zap.ast', got {data.get('kind')}")
    
    if data.get("schema_version") != 1:
        errors.append(f"schema_version should be 1, got {data.get('schema_version')}")
    
    # Check AST structure
    ast = data.get("ast")
    if ast is not None:
        if not isinstance(ast, dict):
            errors.append("ast should be an object")
        elif "statements" not in ast:
            errors.append("ast missing required 'statements' field")
        elif not isinstance(ast["statements"], list):
            errors.append("ast.statements should be an array")
    
    # Check that source_name doesn't contain backslashes
    source_name = data.get("source_name", "")
    if "\\" in source_name:
        errors.append(f"source_name contains backslashes: {source_name!r}")
    
    return errors

def main():
    exit_code = 0
    for filename in sorted(os.listdir(PARSER_DIR)):
        if not filename.endswith(".ast.json"):
            continue
        
        path = os.path.join(PARSER_DIR, filename)
        errors = validate_ast_file(path)
        
        if errors:
            print(f"FAIL: {filename}")
            for error in errors:
                print(f"  - {error}")
            exit_code = 1
        else:
            print(f"OK: {filename}")
    
    # Also check diagnostics files
    for filename in sorted(os.listdir(PARSER_DIR)):
        if not filename.endswith(".diagnostics.json"):
            continue
        
        path = os.path.join(PARSER_DIR, filename)
        try:
            data = read_json_file(path)
        except Exception as e:
            print(f"FAIL: {filename} - JSON parse error: {e}")
            exit_code = 1
            continue
        
        errors = []
        for field in ["diagnostics", "kind", "schema_version"]:
            if field not in data:
                errors.append(f"missing required field: {field}")
        
        if data.get("kind") != "zap.diagnostics":
            errors.append(f"kind should be 'zap.diagnostics', got {data.get('kind')}")
        
        if data.get("schema_version") != 1:
            errors.append(f"schema_version should be 1, got {data.get('schema_version')}")
        
        if errors:
            print(f"FAIL: {filename}")
            for error in errors:
                print(f"  - {error}")
            exit_code = 1
        else:
            print(f"OK: {filename}")
    
    sys.exit(exit_code)

if __name__ == "__main__":
    main()
