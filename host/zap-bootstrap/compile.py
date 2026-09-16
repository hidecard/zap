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
  - list literals `[1, 2, 3]`, list indexing `xs[0]`, `len(xs)`
  - `for x in xs:` loops
  - Basic map literals `{}` (placeholder)
  - Basic class syntax (placeholder)

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
        elif c == '[':
            tokens.append(("LBRACKET", c))
            i += 1
        elif c == ']':
            tokens.append(("RBRACKET", c))
            i += 1
        elif c == '{':
            tokens.append(("LBRACE", c))
            i += 1
        elif c == '}':
            tokens.append(("RBRACE", c))
            i += 1
        elif c == ':':
            tokens.append(("OP", ":"))
            i += 1
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
            elif two == "!=":
                tokens.append(("OP", two))
                i += 2
            elif two == "<=":
                tokens.append(("OP", two))
                i += 2
            elif two == ">=":
                tokens.append(("OP", two))
                i += 2
            elif c in "+-*/%<>()[]=,:":
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
        return self._or()

    def _or(self):
        left = self._and()
        while self.peek()[0] == "ID" and self.peek()[1] == "or":
            self.take()
            right = self._and()
            left = {"kind": "binop", "op": "or", "left": left, "right": right}
        return left

    def _and(self):
        left = self._not()
        while self.peek()[0] == "ID" and self.peek()[1] == "and":
            self.take()
            right = self._not()
            left = {"kind": "binop", "op": "and", "left": left, "right": right}
        return left

    def _not(self):
        if self.peek()[0] == "ID" and self.peek()[1] == "not":
            self.take()
            return {"kind": "not", "operand": self._not()}
        return self._comparison()

    def _comparison(self):
        left = self._additive()
        while True:
            tok = self.peek()
            if tok[0] == "OP" and tok[1] in ("<", ">", "==", "!=", "<=", ">="):
                op = self.take()[1]
                right = self._additive()
                left = {"kind": "binop", "op": op, "left": left, "right": right}
            elif tok[0] == "ID" and tok[1] == "in":
                self.take()
                right = self._additive()
                left = {"kind": "binop", "op": "in", "left": left, "right": right}
            else:
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
        while self.peek()[0] == "OP" and self.peek()[1] in ("*", "/", "%"):
            op = self.take()[1]
            right = self._primary()
            left = {"kind": "binop", "op": op, "left": left, "right": right}
        return left

    def _postfix(self, node):
        while self.peek()[0] == "LBRACKET":
            self.take()
            index = self._comparison()
            self.take()  # ']'
            node = {"kind": "index_expr", "base": node, "index": index}
        return node

    def _primary(self):
        tok = self.peek()
        if tok[0] == "OP" and tok[1] in ("+", "-"):
            self.take()
            sign = -1 if tok[1] == "-" else 1
            if self.peek()[0] == "NUMBER":
                value = self.take()[1]
                return {"kind": "num", "value": sign * value}
            node = self._primary()
            return {"kind": "unary", "op": tok[1], "operand": node}
        if tok[0] == "NUMBER":
            self.take()
            return {"kind": "num", "value": tok[1]}
        if tok[0] == "STRING":
            self.take()
            return {"kind": "str", "value": tok[1]}
        if tok[0] == "ID":
            name = self.take()[1]
            if name == "len" and self.peek()[0] == "OP" and self.peek()[1] == "(":
                self.take()
                arg = self._comparison()
                self.take()  # ')'
                node = {"kind": "len", "arg": arg}
                return self._postfix(node)
            if name in ("keys", "values", "has_key", "contains", "append", "map_get", "map_set") \
                    and self.peek()[0] == "OP" and self.peek()[1] == "(":
                self.take()
                args = [self._comparison()]
                while self.peek()[0] == "OP" and self.peek()[1] == ",":
                    self.take()
                    args.append(self._comparison())
                self.take()  # ')'
                node = {"kind": name, "args": args}
                return self._postfix(node)
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
                node = {"kind": "call", "name": name, "args": args}
                return self._postfix(node)
            if name == "true":
                return {"kind": "bool", "value": True}
            if name == "false":
                return {"kind": "bool", "value": False}
            node = {"kind": "var", "name": name}
            return self._postfix(node)
        if tok[0] == "LBRACKET":
            self.take()
            elements = []
            if self.peek()[0] != "RBRACKET":
                elements.append(self._comparison())
                while self.peek()[0] == "OP" and self.peek()[1] == ",":
                    self.take()
                    elements.append(self._comparison())
            self.take()  # ']'
            node = {"kind": "list", "elements": elements}
            return self._postfix(node)
        if tok[0] == "LBRACE":
            self.take()
            entries = []
            if self.peek()[0] != "RBRACE":
                entries.append(self._map_entry())
                while self.peek()[0] == "OP" and self.peek()[1] == ",":
                    self.take()
                    entries.append(self._map_entry())
            self.take()  # '}'
            node = {"kind": "map", "entries": entries}
            return self._postfix(node)
        if tok[0] == "OP" and tok[1] == "(":
            self.take()
            node = self._comparison()
            self.take()  # ')'
            return self._postfix(node)
        self.take()
        return {"kind": "num", "value": 0}

    def _map_entry(self):
        key = self._comparison()
        if self.peek()[0] == "OP" and self.peek()[1] == ":":
            self.take()
        value = self._comparison()
        return (key, value)


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

    if text.startswith("for "):
        rest = text[4:].rstrip(":").strip()
        var = rest[:rest.index(" in ")].strip()
        iterable = _parse_expr(rest[rest.index(" in ") + 4:].strip())
        body_indent = lines[idx + 1][0]
        body, end = _parse_block(lines, idx + 1, body_indent)
        return {"kind": "for", "var": var, "iterable": iterable, "body": body}, end

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
            if name.endswith("]") and "[" in name:
                base = name[:name.index("[")].strip()
                index_src = name[name.index("[") + 1:-1].strip()
                if base.isidentifier() and index_src:
                    return {"kind": "set_index", "name": base,
                            "index": _parse_expr(index_src),
                            "expr": _parse_expr(expr)}, idx + 1
    return {"kind": "expr", "expr": _parse_expr(text)}, idx + 1


# ---------------------------------------------------------------------------
# Lowering to bytecode (jump targets are absolute indices in the final program)
# ---------------------------------------------------------------------------

_BINOP = {"+": "add", "-": "subtract", "*": "multiply", "/": "divide",
          "%": "remainder", "<": "less", ">": "greater", "==": "equal",
          "!=": "not_equal", "<=": "less_equal", ">=": "greater_equal",
          "and": "and", "or": "or", "in": "in"}


def _compile_expr(node, base=0):
    if node["kind"] == "unary":
        if node["op"] == "+":
            return _compile_expr(node["operand"], base)
        return [{"op": "const", "value": 0}] + _compile_expr(node["operand"], base + 1) + [{"op": "subtract"}]
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
            instrs += _compile_expr(arg, base + len(instrs))
        instrs.append({"op": "call", "name": node["name"], "argc": len(node["args"])})
        return instrs
    if node["kind"] == "binop":
        if node["op"] in ("and", "or"):
            temp_name = "__zap_short_%d" % base
            left = _compile_expr(node["left"], base)
            left_copy = left + [
                {"op": "store", "name": temp_name},
                {"op": "load", "name": temp_name},
            ]
            jump = {
                "op": "jump_if_false" if node["op"] == "and" else "jump_if_true",
                "target": 0,
            }
            right = _compile_expr(node["right"], base + len(left_copy) + 2)
            false_index = base + len(left_copy) + 2 + len(right) + 2
            end_index = false_index + 1
            jump["target"] = false_index
            end_jump = {"op": "jump", "target": end_index}
            result = left_copy + [
                jump,
                {"op": "load", "name": temp_name},
            ] + right + [
                {"op": _BINOP[node["op"]]},
                end_jump,
                {"op": "const", "value": node["op"] == "or"},
            ]
            return result
        left = _compile_expr(node["left"], base)
        right = _compile_expr(node["right"], base + len(left))
        return left + right + [{"op": _BINOP[node["op"]]}]
    if node["kind"] == "list":
        instrs = []
        for element in node["elements"]:
            instrs += _compile_expr(element, base + len(instrs))
        instrs.append({"op": "make_list", "count": len(node["elements"])})
        return instrs
    if node["kind"] == "not":
        return _compile_expr(node["operand"], base) + [{"op": "not"}]
    if node["kind"] == "map":
        instrs = [{"op": "make_map"}]
        for key, value in node["entries"]:
            instrs += _compile_expr(key, base + len(instrs))
            instrs += _compile_expr(value, base + len(instrs))
            instrs.append({"op": "map_set_pair"})
        return instrs
    if node["kind"] == "map_get":
        return (_compile_expr(node["args"][0], base)
                + _compile_expr(node["args"][1], base + 1)
                + [{"op": "map_get"}])
    if node["kind"] == "map_set":
        return (_compile_expr(node["args"][0], base)
                + _compile_expr(node["args"][1], base + 1)
                + _compile_expr(node["args"][2], base + 2)
                + [{"op": "map_set"}])
    if node["kind"] == "append":
        return (_compile_expr(node["args"][0], base)
                + _compile_expr(node["args"][1], base + 1)
                + [{"op": "list_append"}])
    if node["kind"] == "keys":
        return _compile_expr(node["args"][0], base) + [{"op": "map_keys"}]
    if node["kind"] == "values":
        return _compile_expr(node["args"][0], base) + [{"op": "map_values"}]
    if node["kind"] == "has_key":
        return (_compile_expr(node["args"][0], base)
                + _compile_expr(node["args"][1], base + 1)
                + [{"op": "map_has_key"}])
    if node["kind"] == "contains":
        return (_compile_expr(node["args"][0], base)
                + _compile_expr(node["args"][1], base + 1)
                + [{"op": "list_contains"}])
    if node["kind"] in ("index", "index_expr"):
        base_node = node if node["kind"] == "index" else node["base"]
        index_node = node["index"]
        instrs = _compile_expr(base_node, base)
        instrs += _compile_expr(index_node, base + len(instrs))
        instrs.append({"op": "list_get"})
        return instrs
    if node["kind"] == "len":
        instrs = _compile_expr(node["arg"], base)
        instrs.append({"op": "list_len"})
        return instrs
    return []


def _lower(program, stmt, base=None):
    """Append `stmt`'s instructions to `program`; returns the appended list."""
    if base is None:
        base = len(program)
    kind = stmt["kind"]
    if kind in ("let", "assign"):
        instrs = _compile_expr(stmt["expr"], base) + [{"op": "store", "name": stmt["name"]}]
    elif kind == "set_index":
        instrs = ([{"op": "load", "name": stmt["name"]}]
                  + _compile_expr(stmt["index"], base + 1)
                  + _compile_expr(stmt["expr"], base + 2)
                  + [{"op": "list_set"}, {"op": "pop"}])
    elif kind == "say":
        instrs = _compile_expr(stmt["expr"], base) + [{"op": "print"}]
    elif kind == "return":
        instrs = (_compile_expr(stmt["expr"], base) if stmt["expr"] is not None
                  else []) + [{"op": "return_value" if stmt["expr"] is not None else "return_none"}]
    elif kind == "expr":
        instrs = _compile_expr(stmt["expr"], base) + [{"op": "pop"}]
    elif kind == "if":
        instrs = _compile_expr(stmt["cond"], base)
        jf_local = len(instrs)
        instrs.append({"op": "jump_if_false", "target": 0})
        for s in stmt["then"]:
            instrs += _lower(program, s, base + len(instrs))
        if stmt["else"]:
            jmp_local = len(instrs)
            instrs.append({"op": "jump", "target": 0})
            for s in stmt["else"]:
                instrs += _lower(program, s, base + len(instrs))
            instrs[jf_local]["target"] = base + jmp_local + 1
            instrs[jmp_local]["target"] = base + len(instrs)
        else:
            instrs[jf_local]["target"] = base + len(instrs)
    elif kind == "while":
        cond = _compile_expr(stmt["cond"], base)
        instrs = cond + [{"op": "jump_if_false", "target": 0}]
        for s in stmt["body"]:
            instrs += _lower(program, s, base + len(instrs))
        instrs.append({"op": "jump", "target": base})  # back to header
        instrs[len(cond)]["target"] = base + len(instrs)  # exit after loop
    elif kind == "for":
        loop_var = stmt["var"]
        iterable = stmt["iterable"]
        body = stmt["body"]
        idx_name = "__for_idx_" + loop_var
        len_name = "__for_len_" + loop_var
        iter_name = "__for_iter_" + loop_var
        instrs = _compile_expr(iterable, base) + [{"op": "store", "name": iter_name}]
        instrs += [{"op": "const", "value": 0}, {"op": "store", "name": idx_name}]
        instrs += [{"op": "load", "name": iter_name}, {"op": "list_len"}, {"op": "store", "name": len_name}]
        cond_start = len(instrs)
        instrs += [{"op": "load", "name": idx_name}, {"op": "load", "name": len_name}, {"op": "less"}, {"op": "jump_if_false", "target": 0}]
        instrs += [{"op": "load", "name": iter_name}, {"op": "load", "name": idx_name}, {"op": "list_get"}, {"op": "store", "name": loop_var}]
        for s in body:
            instrs += _lower(program, s, base + len(instrs))
        instrs += [{"op": "load", "name": idx_name}, {"op": "const", "value": 1}, {"op": "add"}, {"op": "store", "name": idx_name}]
        instrs.append({"op": "jump", "target": base + cond_start})
        instrs[cond_start + 3]["target"] = base + len(instrs)
    else:
        instrs = []
    return instrs


def _strip_bom(source):
    """Remove a UTF-8 BOM so Windows-authored sources compile identically."""
    if source and source[0] == "\ufeff":
        return source[1:]
    return source


def compile_program(source):
    lines = _split_lines(_strip_bom(source))
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
