#!/usr/bin/env python3
"""Zap 0.2: readable reference runtime for Web/AI experiments."""
from __future__ import annotations
import json, sys, os, argparse, urllib.request, urllib.parse
__version__ = "0.2.0"

class ZapError(Exception): pass

class Web:
    @staticmethod
    def get(url):
        with urllib.request.urlopen(url, timeout=10) as r:
            return {"status": r.status, "text": r.read().decode("utf-8"), "headers": dict(r.headers)}
    @staticmethod
    def text(value): return {"status": 200, "text": str(value), "headers": {"content-type": "text/plain"}}
    @staticmethod
    def json(value): return {"status": 200, "text": json.dumps(value), "headers": {"content-type": "application/json"}}
    @staticmethod
    def listen(port, handler=None):
        print(f"[Zap web] server foundation ready on http://localhost:{port}")
        return {"port": port, "handler": handler}

class AI:
    @staticmethod
    def ask(prompt):
        # Provider-neutral placeholder. Configure a provider adapter in the next release.
        return {"text": f"[AI placeholder] {prompt}", "model": "local-placeholder"}

ENV = {
    "say": print, "json": json, "web": Web, "ai": AI,
    "true": True, "false": False, "none": None,
    "length": len, "text": str, "number": float,
}

def convert_expr(s):
    s = s.strip()
    # Convert expressions on the right side of a simple assignment.
    if " = " in s and "==" not in s:
        left, right = s.split(" = ", 1)
        return left.strip() + " = " + convert_expr(right)
    # Module calls can appear on the right-hand side of assignments.
    for prefix, func in (("json.stringify ", "json.dumps("), ("json.parse ", "json.loads("), ("ai.ask ", "ai.ask("), ("web.get ", "web.get(")):
        if s.startswith(prefix):
            s = func + s[len(prefix):] + ")"
            break
    # Zap booleans/null and simple property access are normalized here.
    s = s.replace(" true", " True").replace(" false", " False").replace(" none", " None")
    if s.startswith("true"): s = "True" + s[4:]
    if s.startswith("false"): s = "False" + s[5:]
    if s.startswith("none"): s = "None" + s[4:]
    return s

def translate(src):
    out = ["# generated Zap runtime", "def __zap_return(x): return x"]
    indent_stack = [0]
    lines = src.splitlines()
    for no, raw in enumerate(lines, 1):
        if not raw.strip() or raw.lstrip().startswith("#"): continue
        spaces = len(raw) - len(raw.lstrip(" "))
        if spaces % 4: raise ZapError(f"Line {no}: indentation must use multiples of 4 spaces")
        while indent_stack[-1] > spaces: indent_stack.pop()
        if spaces > indent_stack[-1]:
            if spaces != indent_stack[-1] + 4: raise ZapError(f"Line {no}: unexpected indentation")
            indent_stack.append(spaces)
        if spaces < indent_stack[-1]:
            while indent_stack[-1] > spaces: indent_stack.pop()
        body = raw.strip()
        py = body
        if body == "else:": py = "else:"
        elif body.startswith("say "): py = "say(" + convert_expr(body[4:]) + ")"
        elif body.startswith("use "):
            mod = body[4:].strip(); py = f"say('[Zap] loaded module: {mod}')"
        elif body.startswith("return "): py = "return " + convert_expr(body[7:])
        elif body.startswith("if "): py = "if " + convert_expr(body[3:])
        elif body.startswith("while "): py = "while " + convert_expr(body[6:])
        elif body.startswith("for "): py = "for " + body[4:]
        elif body.startswith("fn "):
            # fn name(a, b): -> def name(a, b):
            py = "def " + body[3:]
        elif body.startswith("map "):
            # map user = { ... } uses the runtime's map representation.
            py = body[4:]
        elif body.startswith("json.parse "): py = "json.loads(" + body[11:] + ")"
        elif body.startswith("json.stringify "): py = "json.dumps(" + body[15:] + ")"
        elif body.startswith("web.get "): py = "web.get(" + body[8:] + ")"
        elif body.startswith("web.listen "): py = "web.listen(" + body[11:] + ")"
        elif body.startswith("web.text "): py = "web.text(" + body[9:] + ")"
        elif body.startswith("web.json "): py = "web.json(" + body[9:] + ")"
        elif body.startswith("ai.ask "): py = "ai.ask(" + body[8:] + ")"
        elif body.endswith(":"): py = convert_expr(body)
        else: py = convert_expr(body)
        out.append(" " * spaces + py)
    return "\n".join(out) + "\n"

def run(source, filename="<string>"):
    try:
        code = compile(translate(source), filename, "exec")
        exec(code, dict(ENV))
    except ZapError: raise
    except Exception as e:
        raise ZapError(f"{filename}: {e}") from e

def main():
    parser = argparse.ArgumentParser(prog="zap", description="Zap Web/AI programming language")
    parser.add_argument("--version", action="version", version=f"Zap {__version__}")
    sub = parser.add_subparsers(dest="command")
    run_cmd = sub.add_parser("run", help="run a .zp file with the optional reference runtime")
    run_cmd.add_argument("file")
    new_cmd = sub.add_parser("new", help="create a starter Zap project")
    new_cmd.add_argument("name")
    args = parser.parse_args()
    if args.command == "new":
        os.makedirs(args.name, exist_ok=False)
        with open(os.path.join(args.name, "main.zp"), "w", encoding="utf-8") as f:
            f.write('say "Hello from Zap"\n')
        with open(os.path.join(args.name, "README.md"), "w", encoding="utf-8") as f:
            f.write(f"# {args.name}\n\nRun with `zap main.zp`.\n")
        print(f"Created Zap project: {args.name}")
        return 0
    filename = getattr(args, "file", None) if args.command == "run" else (sys.argv[1] if len(sys.argv) == 2 else None)
    if not filename:
        parser.print_help(); return 2
    try:
        with open(filename, encoding="utf-8") as f: run(f.read(), filename)
    except (OSError, ZapError, SyntaxError, FileExistsError) as e:
        print(f"Zap error: {e}", file=sys.stderr); return 1
    return 0

if __name__ == "__main__": raise SystemExit(main())
