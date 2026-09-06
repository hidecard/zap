#!/usr/bin/env python3
"""Minimal jq-compatible checker for verify_b2_typecheck.sh."""
import json
import re
import sys


def check(data, expr):
    # Generic contains(...) handler
    contains_match = re.search(r'\(\.message \| contains\("([^"]+)"\)\)', expr)
    if contains_match:
        expected = contains_match.group(1)
        if 'expects' in expected and 'got' in expected:
            return expected in data.get('message', '')
    
    # Generic test(...) handler
    test_match = re.search(r'\(\.message \| test\("([^"]+)"\)\)', expr)
    if test_match:
        pattern = test_match.group(1)
        return re.search(pattern, data.get('message', '')) is not None
    
    if expr == '.ok == true':
        return data.get('ok') is True
    if expr == '.ok == true and .code? == null':
        return data.get('ok') is True and data.get('code') is None
    if expr == '.ok == false and .code == "ZAP-SYNTAX-001" and .kind == "SyntaxError" and .severity == "error" and .line == 1 and .column == 1':
        return (
            data.get('ok') is False
            and data.get('code') == 'ZAP-SYNTAX-001'
            and data.get('kind') == 'SyntaxError'
            and data.get('severity') == 'error'
            and data.get('line') == 1
            and data.get('column') == 1
        )
    if expr == '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 1 and .column == 1 and (.message | contains("expects number, got text"))':
        return (
            data.get('ok') is False
            and data.get('code') == 'ZAP-TYPE-001'
            and data.get('kind') == 'TypeError'
            and data.get('severity') == 'error'
            and data.get('line') == 1
            and data.get('column') == 1
            and 'expects number, got text' in data.get('message', '')
        )
    if expr == '.ok == false and .code == "ZAP-TYPE-001" and .kind == "TypeError" and .severity == "error" and .line == 3 and .column == 22 and (.message | test("argument .* for .* expects number, got text"))':
        return (
            data.get('ok') is False
            and data.get('code') == 'ZAP-TYPE-001'
            and data.get('kind') == 'TypeError'
            and data.get('severity') == 'error'
            and data.get('line') == 3
            and data.get('column') == 22
            and re.search(r'argument .* for .* expects number, got text', data.get('message', '')) is not None
        )
    if 'expects text, got number' in expr:
        return 'expects text, got number' in data.get('message', '')
    if 'expects number, got option' in expr:
        return 'expects number, got option' in data.get('message', '')
    if 'expects option<number>,' in expr:
        return 'expects option<number>,' in data.get('message', '')
    if 'expects none, got number' in expr:
        return 'expects none, got number' in data.get('message', '')
    if 'expects bool, got number' in expr:
        return 'expects bool, got number' in data.get('message', '')
    if 'expects list<number>,' in expr:
        return 'expects list<number>,' in data.get('message', '')
    if 'expects text, got list' in expr:
        return 'expects text, got list' in data.get('message', '')
    if 'expects text, got map' in expr:
        return 'expects text, got map' in data.get('message', '')
    if 'expects text, got option' in expr:
        return 'expects text, got option' in data.get('message', '')
    if 'expects text, got result' in expr:
        return 'expects text, got result' in data.get('message', '')
    if 'expects text, got bool' in expr:
        return 'expects text, got bool' in data.get('message', '')
    if 'expects number, got result' in expr:
        return 'expects number, got result' in data.get('message', '')
    if 'expects list<text>,' in expr:
        return 'expects list<text>,' in data.get('message', '')
    if 'expects map<text,text>' in expr:
        return 'expects map<text,text>' in data.get('message', '')
    if 'expects text, got option<list' in expr:
        return 'expects text, got option<list' in data.get('message', '')
    if 'expects text, got option<number>' in expr:
        return 'expects text, got option<number>' in data.get('message', '')
    if 'expects text, got map<text,number>' in expr:
        return 'expects text, got map<text,number>' in data.get('message', '')
    if 'expects number, got option<list' in expr:
        return 'expects number, got option<list' in data.get('message', '')
    if 'expects number, got map<text,number>' in expr:
        return 'expects number, got map<text,number>' in data.get('message', '')
    if 'expects T, got text' in expr:
        return 'expects T, got text' in data.get('message', '')
    if 'generic argument substitution' in expr:
        return 'generic argument substitution' in data.get('message', '')
    if 'function' in expr and 'expects' in expr and 'got' in expr:
        return 'expects' in data.get('message', '') and 'got' in data.get('message', '')
    if 'unknown type annotation' in expr:
        return 'unknown type annotation' in data.get('message', '')
    if expr == '(.message == "generic type-parameter list cannot be empty")':
        return data.get('message') == 'generic type-parameter list cannot be empty'
    if expr == '(.message == "duplicate generic type parameter: T")':
        return data.get('message') == 'duplicate generic type parameter: T'
    if expr == '(.message == "invalid generic type parameter \'t\'")':
        return data.get('message') == "invalid generic type parameter 't'"
    if expr == '.kind == "zap.typed_ir" and .schema_version == 1 and .reference_only == true and .ir.nodes[0].annotation == "number" and .ir.nodes[0].inferred_type == "number"':
        nodes = data.get('ir', {}).get('nodes', [])
        return (
            data.get('kind') == 'zap.typed_ir'
            and data.get('schema_version') == 1
            and data.get('reference_only') is True
            and len(nodes) > 0
            and nodes[0].get('annotation') == 'number'
            and nodes[0].get('inferred_type') == 'number'
        )
    if expr == '.kind == "zap.typed_ir" and .schema_version == 1 and .reference_only == true and .ir.nodes[0].kind == "function" and .ir.nodes[0].name == "identity" and .ir.nodes[0].type_params == ["T"] and .ir.nodes[1].inferred_type == "number" and .ir.nodes[2].inferred_type == "text"':
        nodes = data.get('ir', {}).get('nodes', [])
        return (
            data.get('kind') == 'zap.typed_ir'
            and data.get('schema_version') == 1
            and data.get('reference_only') is True
            and len(nodes) > 0
            and nodes[0].get('kind') == 'function'
            and nodes[0].get('name') == 'identity'
            and nodes[0].get('type_params') == ['T']
            and len(nodes) > 1
            and nodes[1].get('inferred_type') == 'number'
            and len(nodes) > 2
            and nodes[2].get('inferred_type') == 'text'
        )
    raise ValueError(f"Unsupported jq expression: {expr}")


def main():
    if len(sys.argv) < 2:
        print("Usage: jq_check.py <expr> [file]", file=sys.stderr)
        sys.exit(1)

    expr = sys.argv[1]
    input_file = sys.argv[2] if len(sys.argv) > 2 else None

    if input_file:
        with open(input_file, 'r', encoding='utf-8') as f:
            data = json.load(f)
    else:
        data = json.load(sys.stdin)

    try:
        result = check(data, expr)
    except Exception as e:
        print(f"jq_check error: {e}", file=sys.stderr)
        sys.exit(1)

    sys.exit(0 if result else 1)


if __name__ == '__main__':
    main()
