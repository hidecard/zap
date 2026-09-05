#!/usr/bin/env python3
"""Minimal non-Rust Zap lexer host.

This module re-implements the canonical B1 lexer from
`bootstrap/b1/lexer.zp` in Python so that the lexer can be executed
without the Rust reference binary. It is a seed host, not the final
runtime; its only job is to prove that the lexer logic is Rust-free
and that its output matches the Rust reference for the owned corpus.
"""

from __future__ import annotations

import json
import sys
from pathlib import Path


ALPHA = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_éλ"
DIGITS = "0123456789"
ALPHA_DIGITS = ALPHA + DIGITS


def _quoted(value: str) -> str:
    return json.dumps(value, ensure_ascii=False)


def _token(kind: str, value, line: int, column: int, length: int) -> str:
    if isinstance(value, str):
        value_json = json.dumps(value, ensure_ascii=False)
    else:
        value_json = json.dumps(value)
    return (
        '{"kind":'
        + _quoted(kind)
        + ',"span":{"column":'
        + str(column)
        + ',"length":'
        + str(length)
        + ',"line":'
        + str(line)
        + '},"value":'
        + value_json
        + "}"
    )


def _lexer_error(source_name: str, code: str, message: str, line: int, column: int) -> str:
    return (
        '{"diagnostics":[{"code":'
        + _quoted(code)
        + ',"column":'
        + str(column)
        + ',"help":"check the source token and its spelling","line":'
        + str(line)
        + ',"message":'
        + _quoted(message)
        + ',"severity":"error","source_name":'
        + _quoted(source_name)
        + '}],"kind":"zap.diagnostics","schema_version":1,"source_name":'
        + _quoted(source_name)
        + "}"
    )


def lex(source: str, source_name: str) -> str:
    i = 0
    line = 1
    column = 1
    first = True
    tokens = ""
    source_length = len(source)

    while True:
        if i >= source_length:
            break

        ch = source[i]
        if ch == " " or ch == "\t" or ch == "\n":
            if ch == "\n":
                line += 1
                column = 1
            else:
                column += 1
            i += 1
        else:
            if ch == "#":
                while i < source_length and source[i] != "\n":
                    i += 1
                    column += 1
            else:
                start_line = line
                start_column = column
                if ch.isdigit():
                    start = i
                    while i < source_length and source[i].isdigit():
                        i += 1
                        column += 1
                    raw = source[start:i]
                    if len(raw) > 19:
                        return _lexer_error(
                            source_name,
                            "ZAP-LEX-INT-001",
                            f"invalid integer literal at {start_line}:{start_column}: {raw}",
                            start_line,
                            start_column,
                        )
                    token_str = _token("number", int(raw), start_line, start_column, i - start)
                    if first:
                        tokens = token_str
                        first = False
                    else:
                        tokens = tokens + "," + token_str
                elif ch.isalpha() or ch == "_" or ch in ALPHA:
                    start = i
                    while i < source_length and (source[i].isalnum() or source[i] == "_" or source[i] in ALPHA):
                        i += 1
                        column += 1
                    word = source[start:i]
                    kind = "name"
                    value = word
                    if word == "and":
                        kind = "and"
                        value = None
                    elif word == "or":
                        kind = "or"
                        value = None
                    token_str = _token(kind, value, start_line, start_column, i - start)
                    if first:
                        tokens = token_str
                        first = False
                    else:
                        tokens = tokens + "," + token_str
                elif ch == '"':
                    start = i
                    i += 1
                    column += 1
                    value = ""
                    closed = False
                    while i < source_length:
                        current = source[i]
                        if current == '"':
                            i += 1
                            column += 1
                            closed = True
                            break
                        if current == "\\":
                            if i + 1 >= source_length:
                                return _lexer_error(
                                    source_name,
                                    "ZAP-LEX-STR-001",
                                    f"unterminated string at {start_line}:{start_column}",
                                    start_line,
                                    start_column,
                                )
                            i += 1
                            column += 1
                            escaped = source[i]
                            if escaped == "n":
                                value += "\n"
                            elif escaped == "t":
                                value += "\t"
                            elif escaped == '"':
                                value += '"'
                            elif escaped == "\\":
                                value += "\\"
                            elif escaped == "r":
                                value += "\r"
                            else:
                                value += escaped
                            i += 1
                            column += 1
                        else:
                            value += current
                            if current == "\n":
                                line += 1
                                column = 1
                            else:
                                column += 1
                            i += 1
                    if not closed:
                        return _lexer_error(
                            source_name,
                            "ZAP-LEX-STR-001",
                            f"unterminated string at {start_line}:{start_column}",
                            start_line,
                            start_column,
                        )
                    token_str = _token("text", value, start_line, start_column, i - start)
                    if first:
                        tokens = token_str
                        first = False
                    else:
                        tokens = tokens + "," + token_str
                else:
                    two = source[i:i + 2] if i + 1 < source_length else ""
                    kind = ""
                    width = 1
                    if two == "==":
                        kind = "equal_equal"
                        width = 2
                    elif two == "!=":
                        kind = "not_equal"
                        width = 2
                    elif two == "<=":
                        kind = "less_equal"
                        width = 2
                    elif two == ">=":
                        kind = "greater_equal"
                        width = 2
                    elif ch == "+":
                        kind = "plus"
                    elif ch == "-":
                        kind = "minus"
                    elif ch == "*":
                        kind = "star"
                    elif ch == "/":
                        kind = "slash"
                    elif ch == "%":
                        kind = "percent"
                    elif ch == "=":
                        kind = "equal"
                    elif ch == "(":
                        kind = "left_paren"
                    elif ch == ")":
                        kind = "right_paren"
                    elif ch == "[":
                        kind = "left_bracket"
                    elif ch == "]":
                        kind = "right_bracket"
                    elif ch == "{":
                        kind = "left_brace"
                    elif ch == "}":
                        kind = "right_brace"
                    elif ch == ":":
                        kind = "colon"
                    elif ch == ",":
                        kind = "comma"
                    elif ch == ".":
                        kind = "dot"
                    elif ch == "?":
                        kind = "question"
                    elif ch == "<":
                        kind = "less"
                    elif ch == ">":
                        kind = "greater"
                    else:
                        return _lexer_error(
                            source_name,
                            "ZAP-LEX-CHAR-001",
                            f"unexpected character at {line}:{column}: {ch}",
                            line,
                            column,
                        )
                    token_str = _token(kind, None, start_line, start_column, width)
                    if first:
                        tokens = token_str
                        first = False
                    else:
                        tokens = tokens + "," + token_str
                    i += width
                    column += width

    end_line = line
    end_column = column
    end_token = _token("end", None, end_line, end_column, 1)
    if not tokens:
        tokens = end_token
    else:
        tokens = tokens + "," + end_token
    return '{"kind":"zap.token_stream","schema_version":1,"source_name":' + _quoted(source_name) + ',"tokens":[' + tokens + "]}"


def main() -> int:
    if len(sys.argv) < 2:
        print("usage: python3 lexer.py <source.zp> [source_name]", file=sys.stderr)
        return 2

    source_path = Path(sys.argv[1]).resolve()
    if len(sys.argv) > 2:
        source_name = sys.argv[2]
    else:
        try:
            source_name = str(source_path.relative_to(Path.cwd().resolve()))
        except ValueError:
            source_name = str(source_path)
    source_name = source_name.replace("\\", "/")

    source = source_path.read_text(encoding="utf-8")
    try:
        print(lex(source, source_name))
    except Exception as exc:  # noqa: BLE001
        print(
            _lexer_error(source_name, "ZAP-LEX-HOST-001", f"lexer host failed: {exc}", 1, 1),
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
