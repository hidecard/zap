#!/usr/bin/env python3
"""Minimal non-Rust Zap bootstrap compiler.

A *seed* compiler (Python, no Rust) that compiles a small but real subset of
Zap source into the bytecode consumed by `host/zap-vm-host/run.py` (the
non-Rust VM host). It demonstrates that the full source -> bytecode ->
execution loop can run with **zero Rust dependency**:

    zap_source --(this compiler)--> bytecode --(run.py)--> output

Supported subset:
  - integer/string literals, true/false
  - `let name = expr`, `name = expr` (reassign)
  - `say expr`
  - `fn name(a, b): ... return expr`  (top-level function definitions)
  - `if expr: ... else: ...` / `while expr: ...`  (indented blocks)
  - arithmetic `+ - * /`, comparisons `< > ==`, parenthesised expressions
  - function calls `name(arg, arg)`

It is a bootstrap seed, not a replacement for the full Zap compiler in
`bootstrap/b1..b4`; it proves the single-language loop is possible without Rust
and is verified by `verify_non_rust_bootstrap_compiler.sh`.
"""
import os
import sys

sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "zap-vm-host"))
from run import run  # noqa: E402


# ---------------------------------------------------------------------------
# Tokenizer (per-expression)
# ---------------------------------------------------------------------------

def _tokenize_expr(text):
    tokens = []
    i = 0
    n = len(text)
    while i < n:
        c = text[i]
        if c in " \t\r":
            i += 1
        elif c == '"':
            j = i + 1
            s = ""
            while j < n and text[j] != '"':
                s += text[j]
                j += 1
            tokens.append(("STRING", s))
            i = j + 1
        elif c.isdigit():
            j = i
            while j < n and text[j].isdigit():
                j += 1
            tokens.append(("NUMBER", int(text[i:j])))
            i = j
        elif c.isalpha() or c == "_":
            j = i
            while j < n and (text[j].isalnum() or text[j] == "_"):
                j += 1
            tokens.append(("ID", text[i:j]))
            i = j
        else:
            two = text[i:i + 2]
            if two == "==":
                tokens.append(("OP", two))
                i += 2
            elif c in "+-*/<>()=,":
                tokens.append(("OP", c))
                i += 1
            else:
                i += 1
    return tokens


# ---------------------------------------------------------------------------
# Expression parser (recursive descent)
# ---------------------------------------------------------------------------

class _ExprParser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.pos = 0

    def peek(self):
        return self.tokens[self.pos] if self.pos < len(self.tokens) else (None,)

    def take(self):
        tok = self.tokens[self.pos]
        self.pos += 1
        return tok

    def parse(self):
        return self._comparison()

    def _comparison(self):
        left = self._additive()
        while self.peek()[0] == "OP" and self.peek()[1] in ("<", ">", "=="):
            op = self.take()[1]
            right = self._additive()
            left = {"kind": "binop", "op": op, "left": left, "right": right}
        return left

    def _additive(self):
        left = self._multiplicative()
        while self.peek()[0] == "OP" and self.peek()[1] in ("+", "-"):
            op = self.take()[1]
            right = self._multiplicative()
            left = {"kind": "binop", "op": op, "left": left, "right": right}
        return left

    def _multiplicative(self):
        left = self._primary()
        while self.peek()[0] == "OP" and self.peek()[1] in ("*", "/"):
            op = self.take()[1]
            right = self._primary()
            left = {"kind": "binop", "op": op, "left": left, "right": right}
        return left

    def _primary(self):
        tok = self.peek()
        if tok[0] == "NUMBER":
            self.take()
            return {"kind": "num", "value": tok[1]}
        if tok[0] == "STRING":
            self.take()
            return {"kind": "str", "value": tok[1]}
        if tok[0] == "ID":
            name = self.take()[1]
            if self.peek()[0] == "OP" and self.peek()[1] == "(":
                self.take()
                args = []
                if self.peek()[0] == "OP" and self.peek()[1] == ")":
                    self.take()
                else:
                    args.append(self._comparison())
                    while self.peek()[0] == "OP" and self.peek()[1] == ",":
                        self.take()
                        args.append(self._comparison())
                    self.take()  # ')'
                return {"kind": "call", "name": name, "args": args}
            if name == "true":
                return {"kind": "bool", "value": True}
            if name == "false":
                return {"kind": "bool", "value": False}
            return {"kind": "var", "name": name}
        if tok[0] == "OP" and tok[1] == "(":
            self.take()
            node = self._comparison()
            self.take()  # ')'
            return node
        self.take()
        return {"kind": "num", "value": 0}


def _parse_expr(text):
    return _ExprParser(_tokenize_expr(text)).parse()


# ---------------------------------------------------------------------------
# Line-based statement parser (indentation blocks)
# ---------------------------------------------------------------------------

def _split_lines(text):
    out = []
    for raw in text.split("\n"):
        if raw.strip() == "" or raw.strip().startswith("#"):
            continue
        indent = len(raw) - len(raw.lstrip(" "))
        out.append((indent, raw.strip()))
    return out


def _parse_block(lines, idx, indent):
    stmts = []
    while idx < len(lines):
        cur_indent, text = lines[idx]
        if cur_indent < indent:
            break
        if cur_indent > indent:
            idx += 1
            continue
        stmt, idx = _parse_stmt(lines, idx, indent)
        if stmt is not None:
            stmts.append(stmt)
    return stmts, idx


def _parse_stmt(lines, idx, indent):
    _, text = lines[idx]
    if text.startswith("fn "):
        header = text[:-1] if text.endswith(":") else text
        rest = header[3:].strip()
        name = rest[:rest.index("(")].strip()
        params_str = rest[rest.index("(") + 1:rest.rindex(")")]
        params = [p.strip() for p in params_str.split(",") if p.strip()]
        body_indent = lines[idx + 1][0]
        body, end = _parse_block(lines, idx + 1, body_indent)
        return {"kind": "function", "name": name, "params": params, "body": body}, end

    if text.startswith("if "):
        cond = _parse_expr(text[3:].rstrip(":").strip())
        body_indent = lines[idx + 1][0]
        then_body, end = _parse_block(lines, idx + 1, body_indent)
        else_body = None
        if end < len(lines) and lines[end][0] == indent and lines[end][1] == "else:":
            e_indent = lines[end + 1][0]
            else_body, end = _parse_block(lines, end + 1, e_indent)
        return {"kind": "if", "cond": cond, "then": then_body, "else": else_body}, end

    if text.startswith("while "):
        cond = _parse_expr(text[6:].rstrip(":").strip())
        body_indent = lines[idx + 1][0]
        body, end = _parse_block(lines, idx + 1, body_indent)
        return {"kind": "while", "cond": cond, "body": body}, end

    if text.startswith("let "):
        rhs = text[4:]
        name = rhs[:rhs.index("=")].strip()
        expr = rhs[rhs.index("=") + 1:].strip()
        return {"kind": "let", "name": name, "expr": _parse_expr(expr)}, idx + 1
    if text.startswith("say "):
        return {"kind": "say", "expr": _parse_expr(text[4:].strip())}, idx + 1
    if text.startswith("return "):
        return {"kind": "return", "expr": _parse_expr(text[7:].strip())}, idx + 1
    if "=" in text:
        eq = text.find("=")
        if not (eq + 1 < len(text) and text[eq + 1] == "="):
            name = text[:eq].strip()
            expr = text[eq + 1:].strip()
            if name.isidentifier():
                return {"kind": "assign", "name": name, "expr": _parse_expr(expr)}, idx + 1
    return {"kind": "expr", "expr": _parse_expr(text)}, idx + 1


# ---------------------------------------------------------------------------
# Lowering to bytecode (jump targets are absolute indices in the final program)
# ---------------------------------------------------------------------------

_BINOP = {"+": "add", "-": "subtract", "*": "multiply", "/": "divide",
          "<": "less", ">": "greater", "==": "equal"}


def _compile_expr(node):
    if node["kind"] == "num":
        return [{"op": "const", "value": node["value"]}]
    if node["kind"] == "str":
        return [{"op": "const", "value": node["value"]}]
    if node["kind"] == "bool":
        return [{"op": "const", "value": node["value"]}]
    if node["kind"] == "var":
        return [{"op": "load", "name": node["name"]}]
    if node["kind"] == "call":
        instrs = []
        for arg in node["args"]:
            instrs += _compile_expr(arg)
        instrs.append({"op": "call", "name": node["name"], "argc": len(node["args"])})
        return instrs
    if node["kind"] == "binop":
        instrs = _compile_expr(node["left"])
        instrs += _compile_expr(node["right"])
        instrs.append({"op": _BINOP[node["op"]]})
        return instrs
    return []


def _lower(program, stmt):
    """Append `stmt`'s instructions to `program`; returns the appended list."""
    kind = stmt["kind"]
    if kind in ("let", "assign"):
        instrs = _compile_expr(stmt["expr"]) + [{"op": "store", "name": stmt["name"]}]
    elif kind == "say":
        instrs = _compile_expr(stmt["expr"]) + [{"op": "print"}]
    elif kind == "return":
        instrs = (_compile_expr(stmt["expr"]) if stmt["expr"] is not None
                  else []) + [{"op": "return_value" if stmt["expr"] is not None else "return_none"}]
    elif kind == "expr":
        instrs = _compile_expr(stmt["expr"]) + [{"op": "pop"}]
    elif kind == "if":
        instrs = _compile_expr(stmt["cond"])
        jf_local = len(instrs)
        instrs.append({"op": "jump_if_false", "target": 0})
        for s in stmt["then"]:
            instrs += _lower(program, s)
        if stmt["else"]:
            jmp_local = len(instrs)
            instrs.append({"op": "jump", "target": 0})
            for s in stmt["else"]:
                instrs += _lower(program, s)
            instrs[jf_local]["target"] = len(program) + jmp_local + 1
            instrs[jmp_local]["target"] = len(program) + len(instrs)
        else:
            instrs[jf_local]["target"] = len(program) + len(instrs)
    elif kind == "while":
        cond = _compile_expr(stmt["cond"])
        instrs = cond + [{"op": "jump_if_false", "target": 0}]
        for s in stmt["body"]:
            instrs += _lower(program, s)
        instrs.append({"op": "jump", "target": len(program)})  # back to header
        instrs[len(cond)]["target"] = len(program) + len(instrs)  # exit after loop
    else:
        instrs = []
    return instrs


def compile_program(source):
    lines = _split_lines(source)
    functions = []
    main = []
    i = 0
    while i < len(lines):
        cur_indent, text = lines[i]
        if cur_indent != 0:
            i += 1
            continue
        stmt, i = _parse_stmt(lines, i, 0)
        if stmt is not None:
            (functions if stmt["kind"] == "function" else main).append(stmt)

    program = []
    for fn in functions:
        fdef_index = len(program)
        program.append({"op": "function_def", "name": fn["name"],
                        "params": fn["params"], "entry": 0, "end": 0})
        for s in fn["body"]:
            program += _lower(program, s)
        program[fdef_index]["entry"] = fdef_index + 1
        program[fdef_index]["end"] = len(program)
    for s in main:
        program += _lower(program, s)
    program.append({"op": "halt"})
    return program


def compile_and_run(source):
    return run(compile_program(source))["output"]


if __name__ == "__main__":
    if len(sys.argv) > 1:
        with open(sys.argv[1]) as fh:
            src = fh.read()
        for value in compile_and_run(src):
            print(value)
    else:
        sample = (
            "fn add(a, b):\n"
            "    return a + b\n"
            "let x = add(2, 3)\n"
            "say x\n"
            "let i = 0\n"
            "let total = 0\n"
            "while i < 5:\n"
            "    total = total + i\n"
            "    i = i + 1\n"
            "say total\n"
        )
        for value in compile_and_run(sample):
            print(value)
