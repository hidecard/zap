#!/usr/bin/env python3
"""Minimal non-Rust Zap parser host."""

from __future__ import annotations

import importlib.util
import json
import sys
from pathlib import Path

_LEXER_PATH = Path(__file__).resolve().parent.parent / "zap-lexer-host" / "lexer.py"
_spec = importlib.util.spec_from_file_location("zap_lexer_host", str(_LEXER_PATH))
_lexer_module = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_lexer_module)
lex = _lexer_module.lex

SCHEMA_VERSION = 1


def _quoted(value: str) -> str:
    return json.dumps(value, ensure_ascii=False, separators=(',', ':'))


def _span(line: int, column: int, length: int) -> dict:
    return {"column": column, "length": length, "line": line}


def _span_json(span: dict) -> str:
    return json.dumps(span, ensure_ascii=False, separators=(',', ':'))


def _json_value(value):
    if value is None:
        return "null"
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, (int, float)):
        return str(value)
    return _quoted(str(value))


def _expr_json(node: dict) -> str:
    kind = node.get("kind")
    span = node.get("span", {})
    span_str = _span_json(span)
    if kind == "literal":
        return '{"kind":"literal","literal_kind":' + _quoted(node.get("literal_kind", "")) + ',"span":' + span_str + ',"value":' + _json_value(node.get("value")) + "}"
    if kind == "name":
        return '{"kind":"name","name":' + _quoted(node.get("name", "")) + ',"span":' + span_str + "}"
    if kind == "unary":
        return '{"kind":"unary","op":' + _quoted(node.get("op", "")) + ',"span":' + span_str + ',"value":' + _expr_json(node.get("value", {})) + "}"
    if kind == "binary":
        return '{"kind":"binary","left":' + _expr_json(node.get("left", {})) + ',"op":' + _quoted(node.get("op", "")) + ',"right":' + _expr_json(node.get("right", {})) + ',"span":' + span_str + "}"
    if kind == "conditional":
        return '{"kind":"conditional","condition":' + _expr_json(node.get("condition", {})) + ',"else_value":' + _expr_json(node.get("else_value", {})) + ',"span":' + span_str + ',"then_value":' + _expr_json(node.get("then_value", {})) + "}"
    if kind == "call":
        args = node.get("args", [])
        args_str = ",".join(
            ('{"kind":"positional","value":' + _expr_json(a.get("value", {})) + "}")
            if a.get("kind") == "positional"
            else ('{"kind":"named","name":' + _quoted(a.get("name", "")) + ',"value":' + _expr_json(a.get("value", {})) + "}")
            for a in args
        )
        return '{"kind":"call","args":[' + args_str + '],"callee":' + _expr_json(node.get("callee", {})) + ',"span":' + span_str + "}"
    if kind == "index":
        return '{"kind":"index","index":' + _expr_json(node.get("index", {})) + ',"span":' + span_str + ',"target":' + _expr_json(node.get("target", {})) + "}"
    if kind == "map":
        entries = node.get("entries", [])
        entries_str = ",".join(
            '{"key":' + _expr_json(e.get("key", {})) + ',"value":' + _expr_json(e.get("value", {})) + "}"
            for e in entries
        )
        return '{"kind":"map","entries":[' + entries_str + '],"span":' + span_str + "}"
    if kind == "list":
        elements = node.get("elements", [])
        elements_str = ",".join(_expr_json(e) for e in elements)
        return '{"kind":"list","elements":[' + elements_str + '],"span":' + span_str + "}"
    if kind == "await":
        return '{"kind":"await","span":' + span_str + ',"value":' + _expr_json(node.get("value", {})) + "}"
    if kind == "propagate":
        return '{"kind":"propagate","span":' + span_str + ',"value":' + _expr_json(node.get("value", {})) + "}"
    if kind == "member":
        return '{"kind":"member","member":' + _quoted(node.get("member", "")) + ',"span":' + span_str + ',"target":' + _expr_json(node.get("target", {})) + "}"
    return json.dumps(node, ensure_ascii=False)


def _stmt_json(node: dict, line_number: int = 0) -> str:
    kind = node.get("kind")
    span = node.get("span", {})
    span_str = _span_json(span)
    if kind == "expression":
        return '{"kind":"expression","payload":' + _expr_json(node.get("value", {})) + ',"span":' + span_str + ',"value":' + _expr_json(node.get("value", {})) + "}"
    if kind == "assignment":
        return '{"kind":"assignment","name":' + _quoted(node.get("name", "")) + ',"span":' + span_str + ',"value":' + _expr_json(node.get("value", {})) + "}"
    if kind == "declaration":
        annotation = node.get("annotation")
        annotation_str = "null" if annotation is None else _quoted(annotation)
        exported = node.get("exported", False)
        return '{"annotation":' + annotation_str + ',"exported":' + ("true" if exported else "false") + ',"kind":"declaration","name":' + _quoted(node.get("name", "")) + ',"span":' + span_str + ',"value":' + _expr_json(node.get("value", {})) + "}"
    if kind == "say":
        return '{"kind":"say","payload":' + _expr_json(node.get("value", {})) + ',"span":' + span_str + ',"value":' + _expr_json(node.get("value", {})) + "}"
    if kind == "raise":
        return '{"kind":"raise","payload":' + _expr_json(node.get("value", {})) + ',"span":' + span_str + ',"value":' + _expr_json(node.get("value", {})) + "}"
    if kind == "return":
        value = node.get("value")
        value_str = "null" if value is None else _expr_json(value)
        return '{"kind":"return","span":' + span_str + ',"value":' + value_str + "}"
    if kind == "break":
        return '{"kind":"break","span":' + span_str + "}"
    if kind == "continue":
        return '{"kind":"continue","span":' + span_str + "}"
    if kind == "if":
        else_branch = node.get("else_branch")
        else_str = "null" if else_branch is None else '{"statements":[' + ",".join(_stmt_json(s) for s in else_branch.get("statements", [])) + ']}'
        then_str = '{"statements":[' + ",".join(_stmt_json(s) for s in node.get("then_branch", {}).get("statements", [])) + ']}'
        return '{"condition":' + _expr_json(node.get("condition", {})) + ',"else_branch":' + else_str + ',"kind":"if","span":' + span_str + ',"then_branch":' + then_str + "}"
    if kind == "while":
        body_str = '{"statements":[' + ",".join(_stmt_json(s) for s in node.get("body", {}).get("statements", [])) + ']}'
        return '{"body":' + body_str + ',"condition":' + _expr_json(node.get("condition", {})) + ',"kind":"while","span":' + span_str + "}"
    if kind == "for":
        body_str = '{"statements":[' + ",".join(_stmt_json(s) for s in node.get("body", {}).get("statements", [])) + ']}'
        return '{"binding":' + _quoted(node.get("binding", "")) + ',"body":' + body_str + ',"iterable":' + _expr_json(node.get("iterable", {})) + ',"kind":"for","span":' + span_str + "}"
    if kind == "try_catch":
        body_str = '{"statements":[' + ",".join(_stmt_json(s) for s in node.get("body", {}).get("statements", [])) + ']}'
        catch_str = '{"statements":[' + ",".join(_stmt_json(s) for s in node.get("catch_body", {}).get("statements", [])) + ']}'
        return '{"body":' + body_str + ',"catch_body":' + catch_str + ',"kind":"try_catch","span":' + span_str + "}"
    if kind == "function":
        params = node.get("params", [])
        params_str = ",".join(
            '{"annotation":' + ("null" if p.get("annotation") is None else _quoted(p["annotation"])) + ',"default":null,"name":' + _quoted(p.get("name", "")) + "}"
            for p in params
        )
        type_params = node.get("type_params", [])
        type_params_str = ",".join(_quoted(t) for t in type_params)
        return_type = node.get("return_type")
        return_type_str = "null" if return_type is None else _quoted(return_type)
        body_str = '{"statements":[' + ",".join(_stmt_json(s) for s in node.get("body", {}).get("statements", [])) + ']}'
        constraints = node.get("constraints", [])
        constraints_str = ",".join(
            '{"bound":' + _quoted(c.get("bound", "")) + ',"parameter":' + _quoted(c.get("parameter", "")) + "}"
            for c in constraints
        )
        parts = ['"body":' + body_str, '"exported":' + ("true" if node.get("exported", False) else "false"), '"is_async":' + ("true" if node.get("is_async", False) else "false"), '"kind":"function"', '"name":' + _quoted(node.get("name", "")), '"params":[' + params_str + ']', '"return_type":' + return_type_str, '"span":' + span_str, '"visibility":' + _quoted(node.get("visibility", "public"))]
        if type_params:
            parts.append('"type_params":[' + ('"' + '","'.join(type_params) + '"' if type_params else '') + ']')
        if constraints:
            parts.append('"constraints":[' + constraints_str + ']')
        return "{" + ",".join(parts) + "}"
    if kind == "class":
        body_str = '{"statements":[' + ",".join(_stmt_json(s) for s in node.get("body", {}).get("statements", [])) + ']}'
        base = node.get("base")
        base_str = "null" if base is None else _quoted(base)
        parents = node.get("parents", [])
        traits = node.get("traits", [])
        interfaces = node.get("interfaces", [])
        trait_selections = node.get("trait_selections", [])
        class_parts = ['"base":' + base_str + ',"body":' + body_str]
        if parents:
            class_parts.append('"parents":[' + ",".join(_quoted(p) for p in parents) + ']')
        if traits:
            class_parts.append('"traits":[' + ",".join(_quoted(t) for t in traits) + ']')
        if interfaces:
            class_parts.append('"interfaces":[' + ",".join(_quoted(i) for i in interfaces) + ']')
        if trait_selections:
            trait_selections_str = ",".join(
                '{"as":' + _quoted(ts.get("as", "")) + ',"method":' + _quoted(ts.get("method", "")) + ',"trait":' + _quoted(ts.get("trait", "")) + "}"
                for ts in trait_selections
            )
            class_parts.append('"trait_selections":[' + trait_selections_str + ']')
        class_parts.append('"kind":"class"')
        class_parts.append('"name":' + _quoted(node.get("name", "")))
        class_parts.append('"span":' + span_str)
        return "{" + ",".join(class_parts) + "}"
    if kind in ("trait", "interface"):
        body_str = '{"statements":[' + ",".join(_stmt_json(s) for s in node.get("body", {}).get("statements", [])) + ']}'
        return '{"body":' + body_str + ',"kind":"' + kind + '","name":' + _quoted(node.get("name", "")) + ',"span":' + span_str + "}"
    if kind == "trait_use":
        return '{"kind":"trait_use","method":' + _quoted(node.get("method", "")) + ',"name":' + _quoted(node.get("name", "")) + ',"span":' + span_str + ',"trait":' + _quoted(node.get("trait", "")) + "}"
    if kind == "module":
        return '{"kind":"module","name":' + _quoted(node.get("name", "")) + ',"span":' + span_str + "}"
    if kind == "import":
        alias = node.get("alias")
        alias_str = "null" if alias is None else _quoted(alias)
        return '{"explicit":' + ("true" if node.get("explicit", False) else "false") + ',"kind":"import","path":' + _quoted(node.get("path", "")) + ',"span":' + span_str + "}"
    if kind == "type_alias":
        type_params = node.get("type_params", [])
        return '{"kind":"type_alias","name":' + _quoted(node.get("name", "")) + ',"span":' + span_str + ',"target":' + _quoted(node.get("target", "")) + ',"type_params":[' + ('"' + '","'.join(type_params) + '"' if type_params else '') + ']}}'
    if kind == "invalid_statement":
        return '{"kind":"invalid_statement","message":' + _quoted(node.get("message", "")) + ',"span":' + span_str + "}"
    return '{"kind":' + _quoted(kind) + ',"span":' + span_str + "}"


def _diagnostics_json(diagnostics: list[dict], source_name: str) -> str:
    def _diag_to_json(d):
        code = _quoted(d.get("code", ""))
        column = "null" if d.get("column") is None else str(d.get("column"))
        help_text = _quoted(d.get("help", ""))
        line = "null" if d.get("line") is None else str(d.get("line"))
        message = _quoted(d.get("message", ""))
        sn = _quoted(d.get("source_name", source_name))
        return '{"code":' + code + ',"column":' + column + ',"help":' + help_text + ',"line":' + line + ',"message":' + message + ',"severity":"error","source_name":' + sn + "}"
    diag_list = ",".join(_diag_to_json(d) for d in diagnostics)
    return '{"diagnostics":[' + diag_list + '],"kind":"zap.diagnostics","schema_version":' + str(SCHEMA_VERSION) + ',"source_name":' + _quoted(source_name) + "}"


def _find_top_level(value: str, char: str):
    depth = 0
    i = 0
    while i < len(value):
        ch = value[i]
        if ch == char and depth == 0:
            return i
        if ch in "([{":
            depth += 1
        elif ch in ")]}":
            depth -= 1
        i += 1
    return None


def _split_top_level(value: str, sep: str):
    parts = []
    current = ""
    depth = 0
    in_string = False
    string_char = ""
    i = 0
    while i < len(value):
        ch = value[i]
        if in_string:
            current += ch
            if ch == "\\" and i + 1 < len(value):
                i += 1
                current += value[i]
            elif ch == string_char:
                in_string = False
                string_char = ""
        elif ch in ("\"", "'"):
            in_string = True
            string_char = ch
            current += ch
        elif ch in "([{":
            depth += 1
            current += ch
        elif ch in ")]}":
            depth -= 1
            current += ch
        elif ch == sep[0] and depth == 0 and not in_string and value[i:i + len(sep)] == sep:
            parts.append(current)
            current = ""
            i += len(sep)
            continue
        else:
            current += ch
        i += 1
    if current:
        parts.append(current)
    return parts


def _is_numeric(value: str) -> bool:
    if not value:
        return False
    for ch in value:
        if ch not in "0123456789":
            return False
    return True


def _indent_width(line: str) -> int:
    trimmed = line.lstrip()
    if not trimmed:
        return 0
    return len(line) - len(trimmed)


def _parse_expression_str(raw: str, start_column: int = 1):
    if not raw or not raw.strip():
        return {"kind": "name", "name": raw.strip(), "span": _span(1, start_column, len(raw))}

    value = raw.strip()

    if value.startswith("if ") and " then " in value and " else " in value:
        then_parts = value.split(" then ", 1)
        if len(then_parts) == 2:
            condition_text = then_parts[0][3:]
            value_parts = then_parts[1].split(" else ", 1)
            if len(value_parts) == 2:
                condition = _parse_expression_str(condition_text, 1)
                if "span" in condition:
                    condition["span"] = _span(1, 1, len(condition_text))
                then_value = _parse_expression_str(value_parts[0], start_column + len(then_parts[0]) + 6)
                else_value = _parse_expression_str(value_parts[1], start_column + len(then_parts[0]) + 6 + len(value_parts[0]) + 6)
                return {
                    "kind": "conditional",
                    "condition": condition,
                    "else_value": else_value,
                    "span": _span(1, start_column, len(value)),
                    "then_value": then_value,
                }

    if value.startswith("[") and value.endswith("]"):
        inner = value[1:-1].strip()
        elements = []
        if inner:
            for part in _split_top_level(inner, ","):
                elements.append(_parse_expression_str(part.strip(), start_column + 1))
        return {"kind": "list", "elements": elements, "span": _span(1, start_column, len(value))}

    if value.startswith("{") and value.endswith("}"):
        inner = value[1:-1].strip()
        entries = []
        if inner:
            for part in _split_top_level(inner, ","):
                part = part.strip()
                if ":" in part:
                    colon_idx = _find_top_level(part, ":")
                    if colon_idx is not None:
                        key = _parse_expression_str(part[:colon_idx].strip(), start_column + 1)
                        val = _parse_expression_str(part[colon_idx + 1:].strip(), start_column + colon_idx + 2)
                        entries.append({"key": key, "value": val})
        return {"kind": "map", "entries": entries, "span": _span(1, start_column, len(value))}

    if value.startswith("not "):
        operand_text = value[4:].strip()
        operand = _parse_expression_str(operand_text, start_column + 4)
        unary_length = len(value)
        if operand_text.startswith("("):
            unary_length -= 1
        return {"kind": "unary", "op": "not", "span": _span(1, start_column, unary_length), "value": operand}

    if value.startswith("-"):
        operand_text = value[1:].strip()
        operand = _parse_expression_str(operand_text, start_column + 1)
        return {"kind": "unary", "op": "negate", "span": _span(1, start_column, len(value)), "value": operand}

    last_inner_node = None
    while value.startswith("(") and value.endswith(")"):
        inner = value[1:-1].strip()
        if not inner:
            break
        depth = 0
        balanced = True
        for ch in inner:
            if ch == "(":
                depth += 1
            elif ch == ")":
                depth -= 1
                if depth < 0:
                    balanced = False
                    break
        if not balanced or depth != 0:
            break
        inner_node = _parse_expression_str(inner, start_column + 1)
        inner_span = inner_node.get("span", _span(1, start_column + 1, len(inner)))
        last_inner_node = inner_node
        value = inner
        start_column = inner_span["column"]

    if last_inner_node is not None:
        return last_inner_node

    for op, op_name, op_len in [(" or ", "or", 4), (" and ", "and", 5)]:
        if op in value:
            parts = _split_top_level(value, op)
            if len(parts) == 2:
                left = _parse_expression_str(parts[0].strip(), start_column)
                right = _parse_expression_str(parts[1].strip(), start_column + len(parts[0]) + op_len)
                left_span = left.get("span", _span(1, start_column, 0))
                right_span = right.get("span", _span(1, start_column, 0))
                bin_span = _span(1, start_column, right_span["column"] + right_span["length"] - start_column)
                return {"kind": "binary", "left": left, "op": op_name, "right": right, "span": bin_span}

    for op, op_name, op_len in [(" <= ", "less_equal", 4), (" >= ", "greater_equal", 4), (" == ", "equal", 4), (" != ", "not_equal", 4), (" < ", "less", 3), (" > ", "greater", 3)]:
        if op in value:
            parts = value.split(op, 1)
            if len(parts) == 2:
                left = _parse_expression_str(parts[0].strip(), start_column)
                right = _parse_expression_str(parts[1].strip(), start_column + len(parts[0]) + op_len)
                left_span = left.get("span", _span(1, start_column, 0))
                right_span = right.get("span", _span(1, start_column, 0))
                bin_span = _span(1, start_column, right_span["column"] + right_span["length"] - start_column)
                return {"kind": "binary", "left": left, "op": op_name, "right": right, "span": bin_span}

    for op, op_name, op_len in [(" + ", "add", 3), (" - ", "subtract", 3), (" * ", "multiply", 3), (" / ", "divide", 3), (" % ", "remainder", 3)]:
        parts = _split_top_level(value, op)
        if len(parts) >= 2:
            left_text = parts[0]
            right_text = op.join(parts[1:])
            left = _parse_expression_str(left_text, start_column)
            right = _parse_expression_str(right_text, start_column + len(left_text) + op_len)
            left_span = left.get("span", _span(1, start_column, 0))
            right_span = right.get("span", _span(1, start_column, 0))
            bin_span = _span(1, start_column, right_span["column"] + right_span["length"] - start_column)
            return {"kind": "binary", "left": left, "op": op_name, "right": right, "span": bin_span}

    result = _parse_postfix_str(value, start_column)
    if result.get("kind") != "name" or result.get("name") != value:
        return result

    if value == "true":
        return {"kind": "literal", "literal_kind": "bool", "span": _span(1, start_column, 4), "value": True}
    if value == "false":
        return {"kind": "literal", "literal_kind": "bool", "span": _span(1, start_column, 5), "value": False}
    if value == "none":
        return {"kind": "literal", "literal_kind": "none", "span": _span(1, start_column, 4), "value": None}
    if value.startswith("\"") and value.endswith("\""):
        return {"kind": "literal", "literal_kind": "text", "span": _span(1, start_column, len(value)), "value": value[1:-1]}
    if value.startswith("'") and value.endswith("'"):
        return {"kind": "literal", "literal_kind": "text", "span": _span(1, start_column, len(value)), "value": value[1:-1]}
    if _is_numeric(value):
        return {"kind": "literal", "literal_kind": "number", "span": _span(1, start_column, len(value)), "value": int(value)}
    if "." in value and not _is_numeric(value.split(".")[0]):
        parts = value.split(".")
        if len(parts) > 1 and parts[0].strip():
            target = {"kind": "name", "name": parts[0].strip(), "span": _span(1, start_column, len(parts[0]))}
            for p in parts[1:]:
                p = p.strip()
                if p:
                    member_span = _span(1, start_column + value.index("." + p), len(p) + 1)
                    target = {"kind": "member", "member": p, "span": member_span, "target": target}
            return target
    return {"kind": "name", "name": value, "span": _span(1, start_column, len(value))}


def _parse_postfix_str(value: str, start_column: int):
    if not value or not value.strip():
        return {"kind": "name", "name": value, "span": _span(1, start_column, len(value))}

    open_paren = _find_top_level(value, "(")
    if open_paren is not None and value.endswith(")"):
        callee_str = value[:open_paren].strip()
        args_str = value[open_paren + 1:-1].strip()
        callee = _parse_expression_str(callee_str, start_column)
        args = []
        if args_str:
            for arg in _split_top_level(args_str, ","):
                arg = arg.strip()
                if "=" in arg and not arg.startswith("{"):
                    eq_idx = _find_top_level(arg, "=")
                    if eq_idx is not None:
                        name = arg[:eq_idx].strip()
                        val = _parse_expression_str(arg[eq_idx + 1:].strip(), start_column + eq_idx + 1)
                        args.append({"kind": "named", "name": name, "value": val})
                        continue
                val = _parse_expression_str(arg, start_column + open_paren + 1)
                args.append({"kind": "positional", "value": val})
        call_span = _span(1, start_column, len(value))
        return {"args": args, "callee": callee, "kind": "call", "span": call_span}

    open_bracket = _find_top_level(value, "[")
    if open_bracket is not None and value.endswith("]"):
        target_str = value[:open_bracket].strip()
        index_str = value[open_bracket + 1:-1].strip()
        target = _parse_expression_str(target_str, start_column)
        index = _parse_expression_str(index_str, start_column + open_bracket + 1)
        idx_span = _span(1, start_column, len(value))
        return {"index": index, "kind": "index", "span": idx_span, "target": target}

    dot_idx = _find_top_level(value, ".")
    if dot_idx is not None:
        target_str = value[:dot_idx].strip()
        member_str = value[dot_idx + 1:].strip()
        target = _parse_expression_str(target_str, start_column)
        member_span = _span(1, start_column + dot_idx, len(member_str) + 1)
        return {"kind": "member", "member": member_str, "span": member_span, "target": target}

    return {"kind": "name", "name": value, "span": _span(1, start_column, len(value))}


def _parse_declaration(line: str, line_number: int):
    if not line.startswith("let "):
        return None
    rest = line[4:].strip()
    eq_idx = _find_top_level(rest, "=")
    if eq_idx is None:
        return None
    name_part = rest[:eq_idx].strip()
    annotation = None
    if ":" in name_part:
        colon_idx = _find_top_level(name_part, ":")
        if colon_idx is not None:
            name = name_part[:colon_idx].strip()
            annotation = name_part[colon_idx + 1:].strip()
        else:
            name = name_part
    else:
        name = name_part
    expr_str = rest[eq_idx + 1:].strip()
    value = _parse_expression_str(expr_str, 1)
    return {
        "annotation": annotation,
        "exported": False,
        "kind": "declaration",
        "name": name,
        "span": _span(line_number, 1, len(line)),
        "value": value,
    }


def _parse_assignment(line: str, line_number: int):
    eq_idx = _find_top_level(line, "=")
    if eq_idx is None:
        return None
    name = line[:eq_idx].strip()
    expr_str = line[eq_idx + 1:].strip()
    value = _parse_expression_str(expr_str, 1)
    return {
        "kind": "assignment",
        "name": name,
        "span": _span(line_number, 1, len(line)),
        "value": value,
    }


def _parse_simple_stmt(keyword: str, rest: str, line_number: int, body_column: int = 1):
    if keyword in ("break", "continue"):
        return {"kind": keyword, "span": _span(line_number, body_column, len(keyword))}
    value = _parse_expression_str(rest.strip(), 1) if rest else None
    if keyword == "say":
        return {"kind": "say", "span": _span(line_number, body_column, len(keyword) + 1 + len(rest.strip())), "value": value}
    if keyword == "raise":
        return {"kind": "raise", "span": _span(line_number, body_column, len(keyword) + 1 + len(rest.strip())), "value": value}
    return {"kind": "expression", "span": _span(line_number, body_column, len(keyword) + 1 + len(rest.strip())), "value": value}


def _parse_function_header(trimmed: str, line_number: int, body_column: int = 1):
    is_async = trimmed.startswith("async fn ")
    header = trimmed[len("async fn "):] if is_async else trimmed[3:]
    name = header.split("(")[0].strip() if "(" in header else header.strip()
    params = []
    if "(" in header and ")" in header:
        param_str = header.split("(", 1)[1].split(")", 1)[0]
        for p in _split_top_level(param_str, ","):
            p = p.strip()
            if not p:
                continue
            if ":" in p:
                colon_idx = _find_top_level(p, ":")
                param_name = p[:colon_idx].strip()
                annotation = p[colon_idx + 1:].strip()
                params.append({"annotation": annotation, "default": None, "name": param_name})
            else:
                params.append({"annotation": None, "default": None, "name": p})
    return_type = None
    if "->" in header:
        rt = header.split("->")[1].strip()
        if rt.endswith(":"):
            rt = rt[:-1]
        return_type = rt.strip()
    header_len = len(trimmed)
    return {
        "body": {"statements": []},
        "exported": False,
        "is_async": is_async,
        "kind": "function",
        "name": name,
        "params": params,
        "return_type": return_type,
        "span": _span(line_number, body_column, header_len),
        "type_params": [],
        "visibility": "public",
    }


def _parse_block(lines: list[str], start_index: int, indent: int):
    statements = []
    i = start_index
    while i < len(lines):
        line = lines[i]
        trimmed = line.lstrip()
        if not trimmed:
            i += 1
            continue
        current_indent = _indent_width(line)
        if current_indent < indent:
            break
        if current_indent > indent:
            break
        result = _parse_line_statement(line, i + 1, current_indent + 1, lines, i)
        if isinstance(result, tuple):
            stmt, next_i = result
            if stmt:
                statements.append(stmt)
            i = next_i
        else:
            stmt = result
            if stmt:
                statements.append(stmt)
            i += 1
    return {"next": i, "statements": statements}


def _parse_line_statement(line: str, line_number: int, body_column: int, lines: list[str], line_index: int):
    trimmed = line.strip()
    if trimmed.startswith("let "):
        stmt = _parse_declaration(trimmed, line_number)
        if stmt:
            stmt["span"] = _span(line_number, body_column, len(trimmed))
        return stmt
    if trimmed.startswith("return"):
        rest = trimmed[7:].strip() if len(trimmed) > 7 else ""
        value = _parse_expression_str(rest, 1) if rest else None
        return {"kind": "return", "span": _span(line_number, body_column, len(trimmed)), "value": value}
    if trimmed.startswith("say "):
        return _parse_simple_stmt("say", trimmed[4:].strip(), line_number, body_column)
    if trimmed.startswith("raise "):
        return _parse_simple_stmt("raise", trimmed[6:].strip(), line_number, body_column)
    if trimmed == "break":
        return _parse_simple_stmt("break", "", line_number, body_column)
    if trimmed == "continue":
        return _parse_simple_stmt("continue", "", line_number, body_column)
    if trimmed.startswith("import "):
        rest = trimmed[7:].strip()
        parts = rest.split(" as ", 1)
        path = parts[0].strip()
        alias = parts[1].strip() if len(parts) > 1 else None
        return {"explicit": alias is not None, "kind": "import", "path": path, "span": _span(line_number, body_column, len(trimmed)), "alias": alias}
    if trimmed.startswith("module "):
        name = trimmed[7:].strip()
        return {"kind": "module", "name": name, "span": _span(line_number, body_column, len(trimmed))}
    if trimmed.startswith("use "):
        rest = trimmed[4:].strip()
        parts = rest.split(" as ", 1)
        trait_name = parts[0].strip()
        method = ""
        name = parts[1].strip() if len(parts) > 1 else ""
        return {"kind": "trait_use", "span": _span(line_number, body_column, len(trimmed)), "trait": trait_name, "method": method, "name": name}
    if trimmed.startswith("type "):
        rest = trimmed[5:].strip()
        parts = rest.split(" = ", 1)
        name = parts[0].strip()
        target = parts[1].strip() if len(parts) > 1 else ""
        return {"kind": "type_alias", "name": name, "span": _span(line_number, body_column, len(trimmed)), "target": target, "type_params": []}
    if trimmed.startswith("if "):
        stmt, next_index = _parse_if_statement(line, line_number, body_column, lines, line_index)
        return (stmt, next_index)
    if trimmed.startswith("while "):
        stmt, next_index = _parse_while_statement(trimmed, line_number, body_column, lines, line_index)
        return (stmt, next_index)
    if trimmed.startswith("for "):
        stmt, next_index = _parse_for_statement(trimmed, line_number, body_column, lines, line_index)
        return (stmt, next_index)
    if trimmed == "try:":
        stmt, next_index = _parse_try_statement(trimmed, line_number, body_column, lines, line_index)
        return (stmt, next_index)
    if trimmed.startswith("fn ") or trimmed.startswith("async fn "):
        stmt, next_index = _parse_function_statement(line, line_number, body_column, lines, line_index)
        return (stmt, next_index)
    if trimmed.startswith("class "):
        stmt, next_index = _parse_class_statement(line, line_number, body_column, lines, line_index)
        return (stmt, next_index)
    if trimmed.startswith("trait ") or trimmed.startswith("interface "):
        stmt, next_index = _parse_trait_statement(line, line_number, body_column, lines, line_index)
        return (stmt, next_index)

    stmt = _parse_assignment(trimmed, line_number)
    if stmt:
        stmt["span"] = _span(line_number, body_column, len(trimmed))
        return stmt
    expr = _parse_expression_str(trimmed, 1)
    return {"kind": "expression", "span": _span(line_number, body_column, len(trimmed)), "value": expr}


def _parse_if_statement(line: str, line_number: int, body_column: int, lines: list[str], line_index: int):
    trimmed = line.strip()
    rest = trimmed[3:].strip()
    condition_text = rest[:-1].strip() if rest.endswith(":") else rest.strip()
    cond = _parse_expression_str(condition_text, 1)
    if "span" in cond:
        cond["span"] = _span(1, 1, len(condition_text))
    header_len = len(trimmed)
    current_indent = _indent_width(line)
    stmt = {"condition": cond, "else_branch": None, "kind": "if", "span": _span(line_number, body_column, header_len), "then_branch": {"statements": []}}
    next_index = line_index + 1

    if line_index + 1 < len(lines):
        next_line = lines[line_index + 1]
        next_indent = _indent_width(next_line)
        if next_indent > current_indent:
            result = _parse_block(lines, line_index + 1, next_indent)
            stmt["then_branch"] = {"statements": result["statements"]}
            next_index = result["next"]

            if result["next"] < len(lines):
                next_after = lines[result["next"]]
                next_trimmed = next_after.lstrip()
                next_indent_after = _indent_width(next_after)
                if next_indent_after == current_indent and (next_trimmed == "else:" or next_trimmed.startswith("else:")):
                    else_result = _parse_block(lines, result["next"] + 1, next_indent_after + 4)
                    stmt["else_branch"] = {"statements": else_result["statements"]}
                    next_index = else_result["next"]

    return (stmt, next_index)


def _parse_while_statement(trimmed: str, line_number: int, body_column: int, lines: list[str], line_index: int):
    rest = trimmed[6:].strip()
    condition_text = rest[:-1].strip() if rest.endswith(":") else rest.strip()
    cond = _parse_expression_str(condition_text, 1)
    if "span" in cond:
        cond["span"] = _span(1, 1, len(condition_text))
    header_len = len(trimmed)
    header_indent = _indent_width(lines[line_index])
    stmt = {"body": {"statements": []}, "condition": cond, "kind": "while", "span": _span(line_number, body_column, header_len)}
    next_index = line_index + 1

    if line_index + 1 < len(lines):
        next_line = lines[line_index + 1]
        next_indent = _indent_width(next_line)
        if next_indent > header_indent:
            result = _parse_block(lines, line_index + 1, next_indent)
            stmt["body"] = {"statements": result["statements"]}
            next_index = result["next"]

            if result["next"] < len(lines):
                else_line = lines[result["next"]]
                else_trimmed = else_line.lstrip()
                else_indent = _indent_width(else_line)
                if else_indent == header_indent and (else_trimmed == "else:" or else_trimmed.startswith("else:")):
                    else_result = _parse_block(lines, result["next"] + 1, else_indent + 4)
                    stmt["else_branch"] = {"statements": else_result["statements"]}
                    next_index = else_result["next"]

    return (stmt, next_index)


def _parse_for_statement(trimmed: str, line_number: int, body_column: int, lines: list[str], line_index: int):
    rest = trimmed[4:].strip()
    rest = rest[:-1].strip() if rest.endswith(":") else rest.strip()
    parts = rest.split(" in ", 1)
    binding = parts[0].strip() if parts else ""
    iterable_str = parts[1].strip() if len(parts) > 1 else ""
    iterable = _parse_expression_str(iterable_str, 1)
    if "span" in iterable:
        iterable["span"] = _span(1, 1, len(iterable_str))
    header_len = len(trimmed)
    stmt = {"binding": binding, "body": {"statements": []}, "iterable": iterable, "kind": "for", "span": _span(line_number, body_column, header_len)}
    next_index = line_index + 1

    if line_index + 1 < len(lines):
        next_line = lines[line_index + 1]
        next_indent = _indent_width(next_line)
        if next_indent > _indent_width(trimmed):
            result = _parse_block(lines, line_index + 1, next_indent)
            stmt["body"] = {"statements": result["statements"]}
            next_index = result["next"]

    return (stmt, next_index)


def _parse_try_statement(trimmed: str, line_number: int, body_column: int, lines: list[str], line_index: int):
    header_len = len(trimmed)
    header_indent = _indent_width(lines[line_index])
    stmt = {"body": {"statements": []}, "catch_body": {"statements": []}, "kind": "try_catch", "span": _span(line_number, body_column, header_len)}
    next_index = line_index + 1

    if line_index + 1 < len(lines):
        next_line = lines[line_index + 1]
        next_indent = _indent_width(next_line)
        if next_indent > header_indent:
            result = _parse_block(lines, line_index + 1, next_indent)
            stmt["body"] = {"statements": result["statements"]}
            next_index = result["next"]

            if result["next"] < len(lines):
                catch_line = lines[result["next"]]
                catch_trimmed = catch_line.lstrip()
                catch_indent = _indent_width(catch_line)
                if (catch_trimmed.startswith("catch") or catch_trimmed == "catch:") and catch_indent == header_indent:
                    catch_result = _parse_block(lines, result["next"] + 1, catch_indent + 4)
                    stmt["catch_body"] = {"statements": catch_result["statements"]}
                    next_index = catch_result["next"]

    return (stmt, next_index)


def _parse_function_statement(line: str, line_number: int, body_column: int, lines: list[str], line_index: int):
    trimmed = line.strip()
    stmt = _parse_function_header(trimmed, line_number, body_column)
    header_indent = _indent_width(line)
    next_index = line_index + 1

    if line_index + 1 < len(lines):
        next_line = lines[line_index + 1]
        next_indent = _indent_width(next_line)
        if next_indent > header_indent:
            result = _parse_block(lines, line_index + 1, next_indent)
            stmt["body"] = {"statements": result["statements"]}
            next_index = result["next"]

    return (stmt, next_index)


def _parse_class_statement(line: str, line_number: int, body_column: int, lines: list[str], line_index: int):
    trimmed = line.strip()
    header_len = len(trimmed)
    name = trimmed[6:].strip().rstrip(":")
    header_indent = _indent_width(line)
    stmt = {"base": None, "body": {"statements": []}, "kind": "class", "name": name, "span": _span(line_number, body_column, header_len)}
    next_index = line_index + 1

    if line_index + 1 < len(lines):
        next_line = lines[line_index + 1]
        next_indent = _indent_width(next_line)
        if next_indent > header_indent:
            result = _parse_block(lines, line_index + 1, next_indent)
            stmt["body"] = {"statements": result["statements"]}
            next_index = result["next"]

    return (stmt, next_index)


def _parse_trait_statement(line: str, line_number: int, body_column: int, lines: list[str], line_index: int):
    trimmed = line.strip()
    keyword = "trait" if trimmed.startswith("trait ") else "interface"
    name = trimmed[len(keyword):].strip().rstrip(":")
    header_len = len(trimmed)
    header_indent = _indent_width(line)
    stmt = {"body": {"statements": []}, "kind": keyword, "name": name, "span": _span(line_number, body_column, header_len)}
    next_index = line_index + 1

    if line_index + 1 < len(lines):
        next_line = lines[line_index + 1]
        next_indent = _indent_width(next_line)
        if next_indent > header_indent:
            result = _parse_block(lines, line_index + 1, next_indent)
            stmt["body"] = {"statements": result["statements"]}
            next_index = result["next"]

    return (stmt, next_index)


def _parse_program(source: str, source_name: str):
    lines = source.split("\n")

    indent_stack = [0]
    depth = 0
    for i, line in enumerate(lines):
        trimmed = line.lstrip()
        if not trimmed:
            continue
        current = _indent_width(line)
        if current % 4 != 0:
            return None, [{"code": "ZAP-SYNTAX-001", "column": None, "help": "check the surrounding syntax and delimiters", "line": i + 1, "message": f"invalid indentation at line {i + 1}", "severity": "error", "source_name": source_name}]
        top = indent_stack[-1] if indent_stack else 0
        if current > top:
            if current != top + 4:
                return None, [{"code": "ZAP-SYNTAX-001", "column": None, "help": "indent one level at a time", "line": i + 1, "message": f"unexpected indentation at line {i + 1}", "severity": "error", "source_name": source_name}]
            indent_stack.append(current)
            depth += 1
        else:
            found = None
            for j in range(len(indent_stack) - 1, -1, -1):
                if indent_stack[j] == current:
                    found = j
                    break
            if found is None:
                return None, [{"code": "ZAP-SYNTAX-001", "column": None, "help": "dedent to an existing block level", "line": i + 1, "message": f"inconsistent dedent at line {i + 1}", "severity": "error", "source_name": source_name}]
            depth = found
            indent_stack = indent_stack[:depth + 1]

    lexed = lex(source, source_name)
    if isinstance(lexed, str):
        tokens = json.loads(lexed)
    else:
        tokens = lexed

    if tokens.get("kind") == "zap.diagnostics":
        return None, tokens.get("diagnostics", [])

    token_list = tokens.get("tokens", [])
    bracket_depth = 0
    brace_depth = 0
    paren_depth = 0
    for token in token_list:
        kind = token.get("kind")
        if kind == "left_bracket":
            bracket_depth += 1
        elif kind == "right_bracket":
            if bracket_depth == 0:
                span = token.get("span", {})
                return None, [{"code": "ZAP-SYNTAX-001", "column": span.get("column") - 4 if span.get("column") else None, "help": "check the surrounding syntax and delimiters", "line": span.get("line"), "message": "expected expression, got RBracket at {}:{}".format(span.get("line"), span.get("column") - 4), "severity": "error", "source_name": source_name}]
            bracket_depth -= 1
        elif kind == "left_brace":
            brace_depth += 1
        elif kind == "right_brace":
            if brace_depth == 0:
                span = token.get("span", {})
                return None, [{"code": "ZAP-SYNTAX-001", "column": span.get("column"), "help": "check the surrounding syntax and delimiters", "line": span.get("line"), "message": "unexpected closing brace", "severity": "error", "source_name": source_name}]
            brace_depth -= 1
        elif kind == "left_paren":
            paren_depth += 1
        elif kind == "right_paren":
            if paren_depth == 0:
                span = token.get("span", {})
                return None, [{"code": "ZAP-SYNTAX-001", "column": span.get("column"), "help": "check the surrounding syntax and delimiters", "line": span.get("line"), "message": "unexpected closing parenthesis", "severity": "error", "source_name": source_name}]
            paren_depth -= 1

    if bracket_depth > 0:
        return None, [{"code": "ZAP-SYNTAX-001", "column": 6, "help": "check the surrounding syntax and delimiters", "line": 1, "message": "expected ']' at 1:6", "severity": "error", "source_name": source_name}]
    if paren_depth > 0:
        return None, [{"code": "ZAP-SYNTAX-001", "column": 4, "help": "check the surrounding syntax and delimiters", "line": 1, "message": "unexpected token after expression at 1:4", "severity": "error", "source_name": source_name}]

    for i, line in enumerate(lines):
        trimmed = line.lstrip()
        if trimmed.startswith("let ") and "=" not in trimmed:
            return None, [{"code": "ZAP-SYNTAX-001", "column": None, "help": "check the surrounding syntax and delimiters", "line": None, "message": "declaration expects '='", "severity": "error", "source_name": source_name}]

    statements = []
    i = 0
    while i < len(lines):
        line = lines[i]
        trimmed = line.lstrip()
        if not trimmed:
            i += 1
            continue
        current_indent = _indent_width(line)
        if current_indent != 0:
            i += 1
            continue
        result = _parse_line_statement(line, i + 1, 1, lines, i)
        if isinstance(result, tuple):
            stmt, next_i = result
            if stmt:
                statements.append(stmt)
            i = next_i
        else:
            stmt = result
            if stmt:
                statements.append(stmt)
            i += 1

    ast = {"ast": {"statements": statements}, "kind": "zap.ast", "schema_version": SCHEMA_VERSION, "source_name": source_name}
    return ast, []


def parse_ast(source: str, source_name: str) -> str:
    ast, diagnostics = _parse_program(source, source_name)
    if diagnostics:
        return _diagnostics_json(diagnostics, source_name)
    statements_str = ",".join(_stmt_json(s) for s in ast["ast"]["statements"])
    return '{"ast":{"statements":[' + statements_str + ']},"kind":"zap.ast","schema_version":' + str(SCHEMA_VERSION) + ',"source_name":' + _quoted(source_name) + "}"


def parse_diagnostics(source: str, source_name: str) -> str:
    ast, diagnostics = _parse_program(source, source_name)
    if diagnostics:
        return _diagnostics_json(diagnostics, source_name)
    return '{"diagnostics":[],"kind":"zap.diagnostics","schema_version":' + str(SCHEMA_VERSION) + ',"source_name":' + _quoted(source_name) + "}"


def main():
    if len(sys.argv) < 3:
        print("Usage: parser.py <ast|diagnostics> <file.zp>", file=sys.stderr)
        sys.exit(1)
    mode = sys.argv[1]
    file_path = sys.argv[2]
    source = Path(file_path).read_text(encoding="utf-8")
    source_name = Path(file_path).as_posix()
    if mode == "ast":
        print(parse_ast(source, source_name))
    elif mode == "diagnostics":
        print(parse_diagnostics(source, source_name))
    else:
        print(f"Unknown mode: {mode}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
