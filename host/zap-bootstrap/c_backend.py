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
    lines.append("#include <stdint.h>")
    lines.append("#include <stdbool.h>")
    lines.append("")
    lines.append("typedef struct frame frame;")
    lines.append("typedef struct {")
    lines.append("  char **locals;")
    lines.append("  int local_count;")
    lines.append("  char **stack;")
    lines.append("  int stack_count;")
    lines.append("  int ip;")
    lines.append("  int halted;")
    lines.append("  char **output;")
    lines.append("  int output_count;")
    lines.append("  int32_t *int_stack;")
    lines.append("  int int_stack_count;")
    lines.append("  frame *frames;")
    lines.append("  int frame_count;")
    lines.append("} state;")
    lines.append("")
    lines.append("struct frame {")
    lines.append("  int return_ip;")
    lines.append("  char **saved_locals;")
    lines.append("  int saved_local_count;")
    lines.append("  int saved_stack_count;")
    lines.append("};")
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
    lines.append("static void push_int(state *st, int32_t value) {")
    lines.append("  st->int_stack = realloc(st->int_stack, sizeof(int32_t) * (st->int_stack_count + 1));")
    lines.append("  st->int_stack[st->int_stack_count++] = value;")
    lines.append("}")
    lines.append("")
    lines.append("static int32_t pop_int(state *st) {")
    lines.append("  int32_t value = st->int_stack[--st->int_stack_count];")
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
    lines.append("static int32_t to_int(const char *value) {")
    lines.append("  if (value == NULL) return 0;")
    lines.append("  size_t len = strlen(value);")
    lines.append("  int negative = 0;")
    lines.append("  size_t i = 0;")
    lines.append("  if (len > 0 && value[0] == '-') { negative = 1; i = 1; }")
    lines.append("  int32_t result = 0;")
    lines.append("  while (i < len) {")
    lines.append("    char c = value[i++];")
    lines.append("    if (c < '0' || c > '9') break;")
    lines.append("    result = result * 10 + (c - '0');")
    lines.append("  }")
    lines.append("  return negative ? -result : result;")
    lines.append("}")
    lines.append("")
    lines.append("static char *from_int(int32_t value) {")
    lines.append("  char buf[32];")
    lines.append("  snprintf(buf, sizeof(buf), \"%d\", value);")
    lines.append("  return strdup(buf);")
    lines.append("}")
    lines.append("")
    lines.append("static bool to_bool(const char *value) {")
    lines.append("  if (value == NULL) return false;")
    lines.append("  if (value[0] == 't' && value[1] == 'r') return true;")
    lines.append("  if (value[0] == '1') return true;")
    lines.append("  size_t len = strlen(value);")
    lines.append("  if (len > 0 && value[0] != '0') return true;")
    lines.append("  return false;")
    lines.append("}")
    lines.append("")
    lines.append("static void call_push(state *st, int return_ip) {")
    lines.append("  st->frames = realloc(st->frames, sizeof(frame) * (st->frame_count + 1));")
    lines.append("  frame *f = &st->frames[st->frame_count++];")
    lines.append("  f->return_ip = return_ip;")
    lines.append("  f->saved_locals = st->locals;")
    lines.append("  f->saved_local_count = st->local_count;")
    lines.append("  f->saved_stack_count = st->stack_count;")
    lines.append("}")
    lines.append("")
    lines.append("static int call_pop(state *st) {")
    lines.append("  if (st->frame_count == 0) { fprintf(stderr, \"stack underflow\\n\"); exit(1); }")
    lines.append("  frame *f = &st->frames[--st->frame_count];")
    lines.append("  for (int i = 0; i < st->local_count; ++i) free(st->locals[i]);")
    lines.append("  free(st->locals);")
    lines.append("  st->locals = f->saved_locals;")
    lines.append("  st->local_count = f->saved_local_count;")
    lines.append("  while (st->stack_count > f->saved_stack_count) {")
    lines.append("    free(pop_str(st));")
    lines.append("  }")
    lines.append("  return f->return_ip;")
    lines.append("}")
    lines.append("")

    # Precompute function entry points.
    functions = {}
    for idx, instr in enumerate(program):
        if instr.get("op") == "function_def":
            functions[instr["name"]] = {
                "entry": instr["entry"],
                "params": instr.get("params", []),
            }

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
    lines.append("  int32_t ia;")
    lines.append("  int32_t ib;")
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
        elif op == "multiply":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  ia = to_int(a); ib = to_int(b);")
            lines.append("  push_str(&st, from_int(ia * ib)); free(a); free(b);")
        elif op == "divide":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  ia = to_int(a); ib = to_int(b);")
            lines.append("  push_str(&st, from_int(ib == 0 ? 0 : ia / ib)); free(a); free(b);")
        elif op == "remainder":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  ia = to_int(a); ib = to_int(b);")
            lines.append("  push_str(&st, from_int(ib == 0 ? 0 : ia % ib)); free(a); free(b);")
        elif op == "less":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  ia = to_int(a); ib = to_int(b);")
            lines.append("  push_str(&st, ia < ib ? strdup(\"true\") : strdup(\"false\")); free(a); free(b);")
        elif op == "greater":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  ia = to_int(a); ib = to_int(b);")
            lines.append("  push_str(&st, ia > ib ? strdup(\"true\") : strdup(\"false\")); free(a); free(b);")
        elif op == "equal":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  bool eq = (a == NULL && b == NULL) || (a != NULL && b != NULL && strcmp(a, b) == 0);")
            lines.append("  push_str(&st, eq ? strdup(\"true\") : strdup(\"false\")); free(a); free(b);")
        elif op == "not":
            lines.append("  b = pop_str(&st); a = b;")
            lines.append("  bool v = to_bool(a);")
            lines.append("  push_str(&st, v ? strdup(\"false\") : strdup(\"true\")); free(a); free(b);")
        elif op == "print":
            lines.append("  print_value(&st, pop_str(&st));")
        elif op == "halt":
            lines.append("  st.halted = 1;")
        elif op == "jump":
            lines.append(f"  goto label_{instr['target']};")
        elif op == "jump_if_false":
            lines.append("  b = pop_str(&st); a = b;")
            lines.append("  if (!to_bool(a)) goto label_%d;" % instr["target"])
            lines.append("  free(a); free(b);")
        elif op == "jump_if_true":
            lines.append("  b = pop_str(&st); a = b;")
            lines.append("  if (to_bool(a)) goto label_%d;" % instr["target"])
            lines.append("  free(a); free(b);")
        elif op == "function_def":
            lines.append(f"  /* function {instr.get('name')} */")
        elif op == "call":
            fn = functions.get(instr["name"])
            if fn:
                # Store current IP for return
                lines.append(f"  call_push(&st, {len(lines) + 3});")
                lines.append(f"  goto label_{fn['entry']};")
                lines.append(f"label_call_{instr['name']}_{len(lines)}:")
            else:
                lines.append(f"  /* unknown call {instr['name']} */")
                lines.append("  push_str(&st, strdup(\"\"));")
        elif op == "return_value":
            lines.append("  /* return with value on stack */")
            lines.append("  target = call_pop(&st);")
            lines.append("  if (target >= 0) goto label_target_return;")
            lines.append("  goto label_999;")
        elif op == "return_none":
            lines.append("  /* return without value */")
            lines.append("  push_str(&st, strdup(\"\"));")
            lines.append("  target = call_pop(&st);")
            lines.append("  if (target >= 0) goto label_target_return;")
            lines.append("  goto label_999;")
        elif op == "make_list":
            count = instr.get("count", 0)
            lines.append(f"  /* make_list count={count} */")
            # Create a list structure - for now use a simple string representation
            # In a full implementation, this would create a proper list data structure
            if count == 0:
                lines.append("  push_str(&st, strdup(\"[]\"));")
            else:
                lines.append("  push_str(&st, strdup(\"[list]\"));")
        elif op == "list_get":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  /* list_get: get element at index */")
            lines.append("  /* For now, return the first element as placeholder */")
            lines.append("  push_str(&st, a); free(a); free(b);")
        elif op == "list_len":
            lines.append("  b = pop_str(&st); a = b;")
            lines.append("  /* list_len: return list length */")
            lines.append("  /* For now, return dummy length - full implementation would parse list string */")
            lines.append("  push_str(&st, from_int(3)); free(a);")
        elif op == "list_set":
            lines.append("  c = pop_str(&st); b = pop_str(&st); a = pop_str(&st);")
            lines.append("  /* list_set: set element at index */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a); free(b); free(c);")
        elif op == "list_append":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  /* list_append: add element to end of list */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a); free(b);")
        elif op == "make_map":
            count = instr.get("count", 0)
            lines.append(f"  /* make_map count={count} */")
            # Create a map structure - for now use a simple string representation
            if count == 0:
                lines.append("  push_str(&st, strdup(\"{}\"));")
            else:
                lines.append("  push_str(&st, strdup(\"{map}\"));")
        elif op == "map_get":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  /* map_get: get value by key */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"\")); free(a); free(b);")
        elif op == "map_set":
            lines.append("  c = pop_str(&st); b = pop_str(&st); a = pop_str(&st);")
            lines.append("  /* map_set: set key-value pair */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a); free(b); free(c);")
        elif op == "map_has_key":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  /* map_has_key: check if key exists */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"false\")); free(a); free(b);")
        elif op == "map_keys":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* map_keys: get all keys as list */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"[]\")); free(a);")
        elif op == "map_values":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* map_values: get all values as list */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"[]\")); free(a);")
        elif op == "struct_new":
            lines.append("  /* struct_new: create new struct instance */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"{struct}\"));")
        elif op == "struct_get":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  /* struct_get: get field value */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"\")); free(a); free(b);")
        elif op == "struct_set":
            lines.append("  c = pop_str(&st); b = pop_str(&st); a = pop_str(&st);")
            lines.append("  /* struct_set: set field value */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a); free(b); free(c);")
        elif op == "error_new":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* error_new: create error value */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a);")
        elif op == "error_is_error":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* error_is_error: check if value is error */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"false\")); free(a);")
        elif op == "error_unwrap":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* error_unwrap: unwrap error value */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a);")
        elif op == "option_some":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* option_some: wrap value in Some */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a);")
        elif op == "option_none":
            lines.append("  /* option_none: create None value */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"none\"));")
        elif op == "option_is_some":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* option_is_some: check if Some */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"false\")); free(a);")
        elif op == "option_is_none":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* option_is_none: check if None */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"true\")); free(a);")
        elif op == "option_unwrap":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* option_unwrap: unwrap Some value */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a);")
        elif op == "option_unwrap_or":
            lines.append("  b = pop_str(&st); a = pop_str(&st);")
            lines.append("  /* option_unwrap_or: unwrap or default */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a); free(b);")
        elif op == "await":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* await: await async value */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, a); free(a);")
        elif op == "async_new":
            lines.append("  /* async_new: create async task */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"{async}\"));")
        elif op == "import_module":
            name = instr.get("name", "")
            lines.append(f"  /* import_module: {name} */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  push_str(&st, strdup(\"{module}\"));")
        elif op == "export_value":
            lines.append("  a = pop_str(&st);")
            lines.append("  /* export_value: export from module */")
            lines.append("  /* Placeholder implementation */")
            lines.append("  free(a);")
        else:
            lines.append(f"  /* unhandled op {op} */")
    lines.append("")

    # Emit labels for jumps and call return sites.
    label_targets = set()
    for idx, instr in enumerate(program):
        if instr.get("op") in ("jump", "jump_if_false", "jump_if_true"):
            label_targets.add(instr.get("target", idx))
    # Add function entry points
    for fn_info in functions.values():
        label_targets.add(fn_info["entry"])
    for target in sorted(label_targets):
        if target < len(program):
            lines.append(f"label_{target}:")
            lines.append("  { (void)0; }")

    lines.append("label_target_return:")
    lines.append("  /* function return landing */")
    lines.append("  { (void)0; }")
    lines.append("")
    lines.append("label_999:")
    lines.append("  { (void)0; }")
    lines.append("")
    lines.append("  for (int i = 0; i < st.output_count; ++i) {")
    lines.append('    printf("%s\\n", st.output[i]);')
    lines.append("    free(st.output[i]);")
    lines.append("  }")
    lines.append("  free(st.output);")
    lines.append("  for (int i = 0; i < st.stack_count; ++i) free(st.stack[i]);")
    lines.append("  free(st.stack);")
    lines.append("  for (int i = 0; i < st.int_stack_count; ++i) { /* int stack has no heap data */ }")
    lines.append("  free(st.int_stack);")
    lines.append("  for (int i = 0; i < st.local_count; ++i) free(st.locals[i]);")
    lines.append("  free(st.locals);")
    lines.append("  for (int i = 0; i < st.frame_count; ++i) {")
    lines.append("    frame *f = &st.frames[i];")
    lines.append("    for (int j = 0; j < f->saved_local_count; ++j) free(f->saved_locals[j]);")
    lines.append("    free(f->saved_locals);")
    lines.append("  }")
    lines.append("  free(st.frames);")
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
