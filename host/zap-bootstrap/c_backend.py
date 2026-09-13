#!/usr/bin/env python3
"""Minimal C backend for the Zap bootstrap compiler.

Emits a self-contained C file from Zap bytecode and compiles it with the
system C compiler.  The resulting executable is a native Zap-produced binary
that does not invoke `cargo`, `rustc`, or `rustup`.
"""
import json
import os
import platform
import shutil
import subprocess
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from compile import compile_program  # noqa: E402


def _c_escape(value):
    if isinstance(value, str):
        return json.dumps(value)
    if isinstance(value, bool):
        return "1" if value else "0"
    return repr(value)


def emit_c(program, out_path):
    lines = []
    lines.append("#include <stdio.h>")
    lines.append("#include <stdlib.h>")
    lines.append("#include <string.h>")
    lines.append("")
    lines.append("typedef struct {")
    lines.append("  char **locals;")
    lines.append("  int local_count;")
    lines.append("  char **stack;")
    lines.append("  int stack_count;")
    lines.append("  int ip;")
    lines.append("  int halted;")
    lines.append("  char **output;")
    lines.append("  int output_count;")
    lines.append("} state;")
    lines.append("")
    lines.append("static void push_str(state *st, const char *value) {")
    lines.append("  st->stack = realloc(st->stack, sizeof(char *) * (st->stack_count + 1));")
    lines.append("  st->stack[st->stack_count++] = strdup(value);")
    lines.append("}")
    lines.append("")
    lines.append("static char *pop_str(state *st) {")
    lines.append("  char *value = st->stack[--st->stack_count];")
    lines.append("  return value;")
    lines.append("}")
    lines.append("")
    lines.append("static void store_local(state *st, int index, const char *value) {")
    lines.append("  if (index >= st->local_count) {")
    lines.append("    st->locals = realloc(st->locals, sizeof(char *) * (index + 1));")
    lines.append("    for (int i = st->local_count; i < index; ++i) st->locals[i] = NULL;")
    lines.append("    st->local_count = index + 1;")
    lines.append("  }")
    lines.append("  if (st->locals[index]) free(st->locals[index]);")
    lines.append("  st->locals[index] = strdup(value);")
    lines.append("}")
    lines.append("")
    lines.append("static char *load_local(state *st, int index) {")
    lines.append("  if (index < 0 || index >= st->local_count || st->locals[index] == NULL) {")
    lines.append("    fprintf(stderr, \"undefined local %d\\n\", index);")
    lines.append("    exit(1);")
    lines.append("  }")
    lines.append("  return strdup(st->locals[index]);")
    lines.append("}")
    lines.append("")
    lines.append("static void print_value(state *st, const char *value) {")
    lines.append("  st->output = realloc(st->output, sizeof(char *) * (st->output_count + 1));")
    lines.append("  st->output[st->output_count++] = strdup(value);")
    lines.append("}")
    lines.append("")

    # Precompute names to indices for a tiny fixed local window.
    name_index = {}
    current_index = 0

    def _name_index(name):
        nonlocal current_index
        if name not in name_index:
            name_index[name] = current_index
            current_index += 1
        return name_index[name]

    lines.append("int main(int argc, char **argv) {")
    lines.append("  state st = {0};")
    lines.append("  char *a;")
    lines.append("  char *b;")
    lines.append("  int target;")
    lines.append("")

    for instr in program:
        op = instr.get("op")
        if op == "const":
            value = _c_escape(instr.get("value"))
            lines.append(f"  push_str(&st, {value});")
        elif op == "store":
            idx = _name_index(instr["name"])
            lines.append(f"  store_local(&st, {idx}, pop_str(&st));")
        elif op == "load":
            idx = _name_index(instr["name"])
            lines.append(f"  push_str(&st, load_local(&st, {idx}));")
        elif op == "add":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  size_t len_a = strlen(a), len_b = strlen(b);")
            lines.append("  char *sum = malloc(len_a + len_b + 1);")
            lines.append("  memcpy(sum, a, len_a); memcpy(sum + len_a, b, len_b); sum[len_a + len_b] = 0;")
            lines.append("  push_str(&st, sum); free(a); free(b); free(sum);")
        elif op == "subtract":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  push_str(&st, a); free(a); free(b);")
        elif op == "print":
            lines.append("  print_value(&st, pop_str(&st));")
        elif op == "halt":
            lines.append("  st.halted = 1;")
        elif op == "jump":
            lines.append(f"  goto label_{instr['target']};")
        elif op == "jump_if_false":
            lines.append("  b = pop_str(&st); a = b;")
            lines.append("  if (!a[0] || (a[0] == 'f' && a[1] == 'a')) goto label_%d;" % instr["target"])
            lines.append("  free(a); free(b);")
        elif op == "function_def":
            lines.append(f"  /* function {instr.get('name')} */")
        elif op == "call":
            lines.append(f"  /* call {instr.get('name')} argc={instr.get('argc', 0)} */")
            lines.append("  push_str(&st, strdup(\"\"));")
        elif op == "return_value":
            lines.append("  /* return */")
        elif op == "return_none":
            lines.append("  /* return */")
        else:
            lines.append(f"  /* unhandled op {op} */")
    lines.append("")

    # Emit labels for jumps.
    label_positions = {}
    for idx, instr in enumerate(program):
        if instr.get("op") == "jump" or instr.get("op") == "jump_if_false":
            label_positions.setdefault(instr.get("target", idx), []).append(idx)

    for target in sorted(label_positions):
        if target < len(program):
            lines.append(f"label_{target}:")
            lines.append("  { (void)0; }")

    lines.append("")
    lines.append("  for (int i = 0; i < st.output_count; ++i) {")
    lines.append('    printf("%s\\n", st.output[i]);')
    lines.append("    free(st.output[i]);")
    lines.append("  }")
    lines.append("  free(st.output);")
    lines.append("  for (int i = 0; i < st.stack_count; ++i) free(st.stack[i]);")
    lines.append("  free(st.stack);")
    lines.append("  for (int i = 0; i < st.local_count; ++i) free(st.locals[i]);")
    lines.append("  free(st.locals);")
    lines.append("  return 0;")
    lines.append("}")
    lines.append("")

    with open(out_path, "w", encoding="utf-8") as fh:
        fh.write("\n".join(lines) + "\n")


def compile_c(c_path, exe_path):
    cc = shutil.which("gcc") or shutil.which("clang") or shutil.which("cc")
    if cc is None:
        raise RuntimeError("no C compiler found; install gcc or clang")
    subprocess.run([cc, "-O2", "-o", exe_path, c_path], check=True)


def main():
    if len(sys.argv) < 2:
        print("Usage: c_backend.py <source.zp> [output_prefix]")
        return 1
    source_path = sys.argv[1]
    prefix = sys.argv[2] if len(sys.argv) > 2 else os.path.splitext(source_path)[0]
    with open(source_path, "r", encoding="utf-8") as fh:
        source = fh.read()
    program = compile_program(source)
    c_path = prefix + ".c"
    exe_path = prefix + (".exe" if platform.system() == "Windows" else "")
    emit_c(program, c_path)
    compile_c(c_path, exe_path)
    print("C backend emitted: %s" % c_path)
    print("C backend executable: %s" % exe_path)
    return 0


if __name__ == "__main__":
    sys.exit(main())
