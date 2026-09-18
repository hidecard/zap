#!/usr/bin/env python3
"""C backend for the Zap bootstrap compiler.

Emits a self-contained C file from Zap bytecode and compiles it with the
system C compiler (gcc/clang/cc, or MSVC cl.exe on Windows).  The resulting
executable is a native Zap-produced binary that does not invoke `cargo`,
`rustc`, or `rustup`.

The emitted runtime uses proper data structures:

- a tagged value system (scalar / list / map / object / error / option / task)
- dynamic lists (make_list, list_get, list_len, list_set, list_append)
- key/value maps (make_map, map_get, map_set, map_has_key, map_keys,
  map_values)
- struct objects with named fields (struct_new, struct_get, struct_set)
- Result/Option tagged values (error_new, error_is_error, error_unwrap,
  option_some, option_none, option_is_some, option_is_none, option_unwrap,
  option_unwrap_or)
- a task registry for async values (async_new, await)
- a module registry for imports/exports (import_module, export_value)
- numeric add/subtract/multiply/divide/remainder and comparisons
- call frames with parameter binding and return-site dispatch
"""
import json
import os
import platform
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from compile import compile_program  # noqa: E402


CANONICAL_BYTECODE_KIND = "zap.bytecode"
CANONICAL_BYTECODE_SCHEMA_VERSION = 1
_DRIVER_BYTECODE_ARTIFACT_KIND = "bytecode"
_BYTECODE_TARGET_OPS = frozenset(("jump", "jump_if_false", "jump_if_true"))
_BYTECODE_SUPPORTED_OPS = frozenset((
    "add", "and", "async_new", "await", "call", "const", "divide", "dup",
    "equal", "error_is_error", "error_message", "error_new", "error_unwrap",
    "export_value", "function_def", "greater", "greater_equal", "halt",
    "import_module", "import_symbol", "in", "index", "jump",
    "jump_if_false", "jump_if_true", "less", "less_equal", "list_append",
    "list_contains", "list_get", "list_len", "list_reverse", "list_set",
    "make_list", "make_map", "map_get", "map_has_key", "map_keys", "map_set",
    "map_set_pair", "map_values", "multiply", "not", "not_equal", "option_is_none",
    "option_is_some", "option_none", "option_some", "option_unwrap",
    "option_unwrap_or", "or", "pop", "print", "remainder", "return_none",
    "return_value", "str_concat", "struct_field_names", "struct_get",
    "struct_has_field", "struct_new", "struct_set", "store", "subtract", "load",
))


def _c_escape(value):
    if isinstance(value, str):
        return json.dumps(value)
    if isinstance(value, bool):
        return json.dumps(value)
    return repr(value)


# Builtins the C backend implements natively. Each entry maps a Zap-level
# builtin name to (arity, C expression) where `bi_args[i]` is the i-th argument
# as a `value *`. These are the standard-library primitives the bootstrap
# language surface relies on; anything else must be a Zap-defined function.
_BUILTIN_SPECS = {
    "str": (1, "vstr(fmt(bi_args[0]))"),
    "int": (1, "vstr(from_int(to_int(bi_args[0]->str)))"),
    "json": (1, "json_serialize(bi_args[0])"),
    "len_str": (1, "string_length(bi_args[0])"),
    "keys": (1, "map_keys(bi_args[0])"),
    "values": (1, "map_values(bi_args[0])"),
    "has_key": (2, "map_has_key(bi_args[0], bi_args[1]->str ? bi_args[1]->str : \"\")"),
    "contains": (2, "contains_value(bi_args[0], bi_args[1])"),
    "push": (2, "list_append_value(bi_args[0], bi_args[1])"),
    "reverse": (1, "list_reverse_value(bi_args[0])"),
    "argv": (0, "builtin_argv()"),
    "argc": (0, "vstr(from_int((int32_t)(g_argc > 0 ? g_argc - 1 : 0)))"),
    "read_file": (1, "read_file_text(bi_args[0]->str ? bi_args[0]->str : \"\")"),
    "write_file": (2, "write_file_text(bi_args[0]->str ? bi_args[0]->str : \"\", bi_args[1]->str ? bi_args[1]->str : \"\")"),
    "exists": (1, "file_exists(bi_args[0]->str ? bi_args[0]->str : \"\")"),
    "is_error": (1, "vstr(is_error(bi_args[0]) ? \"true\" : \"false\")"),
    "error": (1, "verr(bi_args[0]->str ? bi_args[0]->str : \"\")"),
    "error_message": (1, "vstr(is_error(bi_args[0]) && bi_args[0]->str ? bi_args[0]->str : \"\")"),
    "unwrap": (1, "unwrap_value(bi_args[0])"),
    "some": (1, "vsome(bi_args[0])"),
    "none": (0, "vnone()"),
    "option_is_some": (1, "vstr(bi_args[0]->kind == VK_SOME ? \"true\" : \"false\")"),
    "is_some": (1, "vstr(bi_args[0]->kind == VK_SOME ? \"true\" : \"false\")"),
    "option_is_none": (1, "vstr(bi_args[0]->kind == VK_NONE ? \"true\" : \"false\")"),
    "is_none": (1, "vstr(bi_args[0]->kind == VK_NONE ? \"true\" : \"false\")"),
    "option_unwrap_or": (2, "option_unwrap_or_value(bi_args[0], bi_args[1])"),
    "async": (1, "async_new_value(bi_args[0])"),
    "await": (1, "task_run(bi_args[0]->task_id)"),
    "join": (2, "list_join_value(bi_args[0], bi_args[1]->str ? bi_args[1]->str : \"\")"),
    "diagnostic": (2, "vstr(diagnostic_format(bi_args[0]->str ? bi_args[0]->str : \"\", bi_args[1]->str ? bi_args[1]->str : \"\"))"),
    "equal": (2, "vstr(string_equals(bi_args[0], bi_args[1]) ? \"true\" : \"false\")"),
}


def _is_windows_host():
    system = platform.system().lower()
    return os.name == "nt" or system == "windows" or system.startswith(("msys", "mingw"))


class BytecodeValidationError(ValueError):
    """Raised when a bytecode artifact is not in the supported canonical form."""


def _load_bytecode_artifact(bytecode):
    if isinstance(bytecode, os.PathLike):
        with open(os.fspath(bytecode), "r", encoding="utf-8-sig") as handle:
            return json.load(handle)
    if isinstance(bytecode, str):
        stripped = bytecode.lstrip()
        if stripped.startswith("{") or stripped.startswith("["):
            return json.loads(bytecode)
        with open(bytecode, "r", encoding="utf-8-sig") as handle:
            return json.load(handle)
    return bytecode


def _require_int(value, label, minimum=None, maximum=None):
    if isinstance(value, bool) or not isinstance(value, int):
        raise BytecodeValidationError("%s must be an integer" % label)
    if minimum is not None and value < minimum:
        raise BytecodeValidationError("%s must be >= %d" % (label, minimum))
    if maximum is not None and value > maximum:
        raise BytecodeValidationError("%s must be <= %d" % (label, maximum))
    return value


def _validate_instruction(instruction, index, instruction_count):
    if not isinstance(instruction, dict):
        raise BytecodeValidationError("instruction %d must be an object" % index)
    op = instruction.get("op")
    if not isinstance(op, str) or not op:
        raise BytecodeValidationError("instruction %d has no string opcode" % index)
    if op not in _BYTECODE_SUPPORTED_OPS:
        raise BytecodeValidationError(
            "instruction %d uses unsupported opcode %r" % (index, op))

    if op in _BYTECODE_TARGET_OPS:
        _require_int(
            instruction.get("target"), "instruction %d target" % index,
            minimum=0, maximum=instruction_count)

    if op == "const" and not isinstance(
            instruction.get("value"), (type(None), bool, int, float, str)):
        raise BytecodeValidationError(
            "instruction %d const value is not a scalar" % index)

    if op == "function_def":
        if not isinstance(instruction.get("name"), str):
            raise BytecodeValidationError(
                "instruction %d function name must be a string" % index)
        params = instruction.get("params")
        if not isinstance(params, list) or not all(isinstance(p, str) for p in params):
            raise BytecodeValidationError(
                "instruction %d function params must be strings" % index)
        entry = _require_int(
            instruction.get("entry"), "instruction %d entry" % index,
            minimum=0, maximum=instruction_count)
        end = _require_int(
            instruction.get("end"), "instruction %d end" % index,
            minimum=entry, maximum=instruction_count)
        if "binding" in instruction and instruction["binding"] is not None \
                and not isinstance(instruction["binding"], str):
            raise BytecodeValidationError(
                "instruction %d binding must be a string or null" % index)
        captures = instruction.get("captures", [])
        if not isinstance(captures, list) or not all(isinstance(c, str) for c in captures):
            raise BytecodeValidationError(
                "instruction %d captures must be strings" % index)

    if op == "call":
        if not isinstance(instruction.get("name"), str):
            raise BytecodeValidationError(
                "instruction %d call name must be a string" % index)
        _require_int(
            instruction.get("argc", 0), "instruction %d call argc" % index,
            minimum=0)

    if op == "make_list":
        _require_int(
            instruction.get("count", 0), "instruction %d make_list count" % index,
            minimum=0, maximum=64)


def _canonical_instructions(bytecode):
    artifact = _load_bytecode_artifact(bytecode)
    if not isinstance(artifact, dict):
        raise BytecodeValidationError("canonical bytecode must be a JSON object")

    if artifact.get("kind") == CANONICAL_BYTECODE_KIND:
        if artifact.get("schema_version") != CANONICAL_BYTECODE_SCHEMA_VERSION:
            raise BytecodeValidationError(
                "canonical bytecode schema_version must be %d" %
                CANONICAL_BYTECODE_SCHEMA_VERSION)
    elif artifact.get("artifact_kind") == _DRIVER_BYTECODE_ARTIFACT_KIND:
        if "schema_version" in artifact and artifact["schema_version"] != \
                CANONICAL_BYTECODE_SCHEMA_VERSION:
            raise BytecodeValidationError(
                "driver bytecode schema_version must be %d" %
                CANONICAL_BYTECODE_SCHEMA_VERSION)
    else:
        raise BytecodeValidationError(
            "canonical bytecode must have kind %r or artifact_kind %r" %
            (CANONICAL_BYTECODE_KIND, _DRIVER_BYTECODE_ARTIFACT_KIND))

    instructions = artifact.get("instructions")
    if not isinstance(instructions, list):
        raise BytecodeValidationError("canonical bytecode instructions must be a list")
    instruction_count = len(instructions)
    for index, instruction in enumerate(instructions):
        _validate_instruction(instruction, index, instruction_count)
    return list(instructions)


def validate_canonical_bytecode(bytecode):
    """Validate and return instructions from a canonical B3/B4 bytecode artifact."""
    return _canonical_instructions(bytecode)


def parse_canonical_bytecode(bytecode):
    """Compatibility alias for validate_canonical_bytecode."""
    return validate_canonical_bytecode(bytecode)


def emit_c_from_bytecode(bytecode, out_path):
    """Emit C from a canonical B3/B4 bytecode artifact."""
    return emit_c(validate_canonical_bytecode(bytecode), out_path)


def emit_c_from_bytecode_file(bytecode_path, out_path):
    """Load a canonical bytecode JSON file and emit C."""
    with open(bytecode_path, "r", encoding="utf-8-sig") as handle:
        return emit_c_from_bytecode(json.load(handle), out_path)


def build_native_binary_from_bytecode(bytecode, out_prefix, compiler=None, work_dir=None):
    """Emit and compile a native executable from canonical bytecode."""
    instructions = validate_canonical_bytecode(bytecode)
    prefix = os.fspath(out_prefix)
    exe_path = prefix
    if _is_windows_host() and not prefix.lower().endswith(".exe"):
        exe_path += ".exe"
    work_dir = work_dir or os.path.dirname(os.path.abspath(prefix)) or "."
    os.makedirs(work_dir, exist_ok=True)
    c_path = os.path.join(
        work_dir, os.path.splitext(os.path.basename(exe_path))[0] + ".c")
    emit_c(instructions, c_path)
    compile_c(c_path, exe_path, compiler=compiler)
    return {
        "c_path": c_path,
        "exe_path": exe_path,
        "compiler": compiler or find_c_compiler(),
    }


def build_native_binary_from_bytecode_file(bytecode_path, out_prefix, compiler=None):
    """Load a canonical bytecode JSON file and compile a native executable."""
    with open(bytecode_path, "r", encoding="utf-8-sig") as handle:
        return build_native_binary_from_bytecode(
            json.load(handle), out_prefix, compiler=compiler)


def emit_c(program, out_path):
    lines = []
    lines.append("#include <stdio.h>")
    lines.append("#include <stdlib.h>")
    lines.append("#include <string.h>")
    lines.append("#include <stdint.h>")
    lines.append("#include <stdbool.h>")
    lines.append("")
    lines.append("typedef struct value value;")
    lines.append("typedef struct frame frame;")
    lines.append("")
    lines.append("static char *fmt(value *v);")
    lines.append("")
    lines.append("enum { VK_STR = 0, VK_LIST = 1, VK_MAP = 2, VK_ERROR = 3, VK_SOME = 4, VK_NONE = 5, VK_TASK = 6 };")
    lines.append("")
    lines.append("struct value {")
    lines.append("  int kind;")
    lines.append("  char *str;")
    lines.append("  value **items;")
    lines.append("  value **vals;")
    lines.append("  int count;")
    lines.append("  int cap;")
    lines.append("  int vcount;")
    lines.append("  int vcap;")
    lines.append("  int task_id;")
    lines.append("};")
    lines.append("")
    lines.append("struct frame {")
    lines.append("  int return_site;")
    lines.append("  value **saved_locals;")
    lines.append("  int saved_local_count;")
    lines.append("  int saved_stack_count;")
    lines.append("  int argc;")
    lines.append("};")
    lines.append("")
    lines.append("#define ZAP_HEAP_MAX 262144")
    lines.append("static value *g_heap[ZAP_HEAP_MAX];")
    lines.append("static int g_heap_count = 0;")
    lines.append("")
    lines.append("static value *track(value *v) {")
    lines.append("  if (g_heap_count >= ZAP_HEAP_MAX) { fprintf(stderr, \"heap exhausted\\n\"); exit(1); }")
    lines.append("  g_heap[g_heap_count++] = v;")
    lines.append("  return v;")
    lines.append("}")
    lines.append("")
    lines.append("static value *vmake(int kind) {")
    lines.append("  value *v = (value *)calloc(1, sizeof(value));")
    lines.append("  if (v == NULL) { fprintf(stderr, \"out of memory\\n\"); exit(1); }")
    lines.append("  v->kind = kind;")
    lines.append("  return track(v);")
    lines.append("}")
    lines.append("")
    lines.append("static value *vstr(const char *s) {")
    lines.append("  value *v = vmake(VK_STR);")
    lines.append("  v->str = strdup(s ? s : \"\");")
    lines.append("  return v;")
    lines.append("}")
    lines.append("")
    lines.append("static value *vlist(void) {")
    lines.append("  return vmake(VK_LIST);")
    lines.append("}")
    lines.append("")
    lines.append("static value *vmap(void) {")
    lines.append("  return vmake(VK_MAP);")
    lines.append("}")
    lines.append("")
    lines.append("static value *verr(const char *msg) {")
    lines.append("  value *v = vmake(VK_ERROR);")
    lines.append("  v->str = strdup(msg ? msg : \"\");")
    lines.append("  return v;")
    lines.append("}")
    lines.append("")
    lines.append("static value *vsome(value *payload) {")
    lines.append("  value *v = vmake(VK_SOME);")
    lines.append("  v->items = (value **)malloc(sizeof(value *));")
    lines.append("  v->items[0] = payload;")
    lines.append("  v->count = 1;")
    lines.append("  v->cap = 1;")
    lines.append("  return v;")
    lines.append("}")
    lines.append("")
    lines.append("static value *vnone(void) {")
    lines.append("  return vmake(VK_NONE);")
    lines.append("}")
    lines.append("")
    lines.append("static value *vtask(int id) {")
    lines.append("  value *v = vmake(VK_TASK);")
    lines.append("  v->task_id = id;")
    lines.append("  return v;")
    lines.append("}")
    lines.append("")
    lines.append("typedef struct {")
    lines.append("  value **stack;")
    lines.append("  int stack_count;")
    lines.append("  int stack_cap;")
    lines.append("  value **locals;")
    lines.append("  int local_count;")
    lines.append("  char **output;")
    lines.append("  int output_count;")
    lines.append("  int output_cap;")
    lines.append("  frame *frames;")
    lines.append("  int frame_count;")
    lines.append("} state;")
    lines.append("")
    lines.append("static void push(state *st, value *v) {")
    lines.append("  if (st->stack_count >= st->stack_cap) {")
    lines.append("    st->stack_cap = st->stack_cap ? st->stack_cap * 2 : 16;")
    lines.append("    st->stack = (value **)realloc(st->stack, sizeof(value *) * st->stack_cap);")
    lines.append("  }")
    lines.append("  st->stack[st->stack_count++] = v;")
    lines.append("}")
    lines.append("")
    lines.append("static value *pop(state *st) {")
    lines.append("  if (st->stack_count <= 0) { fprintf(stderr, \"stack underflow\\n\"); exit(1); }")
    lines.append("  return st->stack[--st->stack_count];")
    lines.append("}")
    lines.append("")
    lines.append("static void store_local(state *st, int index, value *v) {")
    lines.append("  if (index >= st->local_count) {")
    lines.append("    int new_count = index + 1;")
    lines.append("    st->locals = (value **)realloc(st->locals, sizeof(value *) * new_count);")
    lines.append("    for (int i = st->local_count; i < new_count; ++i) st->locals[i] = NULL;")
    lines.append("    st->local_count = new_count;")
    lines.append("  }")
    lines.append("  st->locals[index] = v;")
    lines.append("}")
    lines.append("")
    lines.append("static value *load_local(state *st, int index, const char *name) {")
    lines.append("  if (index < 0 || index >= st->local_count || st->locals[index] == NULL) {")
    lines.append("    fprintf(stderr, \"undefined local %d (%s)\\n\", index, name ? name : \"?\");")
    lines.append("    exit(1);")
    lines.append("  }")
    lines.append("  return st->locals[index];")
    lines.append("}")
    lines.append("")
    lines.append("#define ZAP_GLOBALS_MAX 4096")
    lines.append("static value *g_globals[ZAP_GLOBALS_MAX];")
    lines.append("static int g_globals_count = 0;")
    lines.append("")
    lines.append("/* Top-level bindings live in a process-wide table so function frames")
    lines.append("   can read and mutate them without owning a copy. */")
    lines.append("static void store_global(state *st, int index, value *v) {")
    lines.append("  (void)st;")
    lines.append("  if (index < 0 || index >= ZAP_GLOBALS_MAX) { fprintf(stderr, \"global index out of range\\n\"); exit(1); }")
    lines.append("  if (index + 1 > g_globals_count) g_globals_count = index + 1;")
    lines.append("  g_globals[index] = v;")
    lines.append("}")
    lines.append("")
    lines.append("static value *load_global(state *st, int index) {")
    lines.append("  (void)st;")
    lines.append("  if (index < 0 || index >= g_globals_count || g_globals[index] == NULL) {")
    lines.append("    fprintf(stderr, \"undefined global %d\\n\", index);")
    lines.append("    exit(1);")
    lines.append("  }")
    lines.append("  return g_globals[index];")
    lines.append("}")
    lines.append("")
    lines.append("static int32_t to_int(const char *s) {")
    lines.append("  if (s == NULL) return 0;")
    lines.append("  size_t len = strlen(s);")
    lines.append("  int negative = 0;")
    lines.append("  size_t i = 0;")
    lines.append("  if (len > 0 && s[0] == '-') { negative = 1; i = 1; }")
    lines.append("  if (i >= len) return 0;")
    lines.append("  int32_t result = 0;")
    lines.append("  while (i < len) {")
    lines.append("    char c = s[i++];")
    lines.append("    if (c < '0' || c > '9') return 0;")
    lines.append("    result = result * 10 + (c - '0');")
    lines.append("  }")
    lines.append("  return negative ? -result : result;")
    lines.append("}")
    lines.append("")
    lines.append("static bool is_int_str(const char *s) {")
    lines.append("  if (s == NULL) return false;")
    lines.append("  size_t len = strlen(s);")
    lines.append("  size_t i = 0;")
    lines.append("  if (len == 0) return false;")
    lines.append("  if (s[0] == '-') { i = 1; }")
    lines.append("  if (i >= len) return false;")
    lines.append("  for (; i < len; ++i) {")
    lines.append("    if (s[i] < '0' || s[i] > '9') return false;")
    lines.append("  }")
    lines.append("  return true;")
    lines.append("}")
    lines.append("")
    lines.append("static char *from_int(int32_t n) {")
    lines.append("  char buf[32];")
    lines.append("  snprintf(buf, sizeof(buf), \"%d\", n);")
    lines.append("  return strdup(buf);")
    lines.append("}")
    lines.append("")
    lines.append("static bool to_bool(value *v) {")
    lines.append("  if (v == NULL) return false;")
    lines.append("  if (v->kind == VK_NONE) return false;")
    lines.append("  if (v->kind == VK_STR) {")
    lines.append("    if (v->str == NULL || v->str[0] == 0) return false;")
    lines.append("    if (strcmp(v->str, \"false\") == 0) return false;")
    lines.append("    if (strcmp(v->str, \"0\") == 0) return false;")
    lines.append("    return true;")
    lines.append("  }")
    lines.append("  return true;")
    lines.append("}")
    lines.append("")
    lines.append("static void list_push(value *list, value *item) {")
    lines.append("  if (list->count >= list->cap) {")
    lines.append("    list->cap = list->cap ? list->cap * 2 : 8;")
    lines.append("    list->items = (value **)realloc(list->items, sizeof(value *) * list->cap);")
    lines.append("  }")
    lines.append("  list->items[list->count++] = item;")
    lines.append("}")
    lines.append("")
    lines.append("static value *list_get(value *list, int32_t index) {")
    lines.append("  if (list->kind != VK_LIST) { fprintf(stderr, \"list_get_non_list\\n\"); exit(1); }")
    lines.append("  if (index < 0 || index >= list->count) { fprintf(stderr, \"list index out of range\\n\"); exit(1); }")
    lines.append("  return list->items[index];")
    lines.append("}")
    lines.append("")
    lines.append("static void list_set(value *list, int32_t index, value *item) {")
    lines.append("  if (list->kind != VK_LIST) { fprintf(stderr, \"list_set_non_list\\n\"); exit(1); }")
    lines.append("  if (index < 0 || index >= list->count) { fprintf(stderr, \"list index out of range\\n\"); exit(1); }")
    lines.append("  list->items[index] = item;")
    lines.append("}")
    lines.append("")
    lines.append("static int map_find(value *map, const char *key) {")
    lines.append("  for (int i = 0; i < map->count; ++i) {")
    lines.append("    if (map->items[i]->str != NULL && strcmp(map->items[i]->str, key) == 0) return i;")
    lines.append("  }")
    lines.append("  return -1;")
    lines.append("}")
    lines.append("")
    lines.append("static value *map_get(value *map, const char *key) {")
    lines.append("  if (map->kind != VK_MAP) { fprintf(stderr, \"map_get_non_map\\n\"); exit(1); }")
    lines.append("  int idx = map_find(map, key);")
    lines.append("  if (idx < 0) return vstr(\"\");")
    lines.append("  return map->vals[idx];")
    lines.append("}")
    lines.append("")
    lines.append("static void map_set(value *map, const char *key, value *val) {")
    lines.append("  if (map->kind != VK_MAP) { fprintf(stderr, \"map_set_non_map\\n\"); exit(1); }")
    lines.append("  int idx = map_find(map, key);")
    lines.append("  if (idx >= 0) { map->vals[idx] = val; return; }")
    lines.append("  if (map->count >= map->cap) {")
    lines.append("    map->cap = map->cap ? map->cap * 2 : 8;")
    lines.append("    map->items = (value **)realloc(map->items, sizeof(value *) * map->cap);")
    lines.append("    map->vals = (value **)realloc(map->vals, sizeof(value *) * map->cap);")
    lines.append("  }")
    lines.append("  map->items[map->count] = vstr(key);")
    lines.append("  map->vals[map->count] = val;")
    lines.append("  map->count++;")
    lines.append("}")
    lines.append("")
    lines.append("static value *map_has_key(value *map, const char *key) {")
    lines.append("  if (map->kind != VK_MAP) { fprintf(stderr, \"map_has_key_non_map\\n\"); exit(1); }")
    lines.append("  return vstr(map_find(map, key) >= 0 ? \"true\" : \"false\");")
    lines.append("}")
    lines.append("")
    lines.append("static value *map_keys(value *map) {")
    lines.append("  value *out = vlist();")
    lines.append("  for (int i = 0; i < map->count; ++i) list_push(out, vstr(map->items[i]->str));")
    lines.append("  return out;")
    lines.append("}")
    lines.append("")
    lines.append("static value *map_values(value *map) {")
    lines.append("  value *out = vlist();")
    lines.append("  for (int i = 0; i < map->count; ++i) list_push(out, map->vals[i]);")
    lines.append("  return out;")
    lines.append("}")
    lines.append("")
    lines.append("static value *list_concat_value(value *a, value *b) {")
    lines.append("  if (a->kind != VK_LIST || b->kind != VK_LIST) {")
    lines.append("    fprintf(stderr, \"list_concat_non_list\\n\");")
    lines.append("    exit(1);")
    lines.append("  }")
    lines.append("  value *out = vlist();")
    lines.append("  for (int i = 0; i < a->count; ++i) list_push(out, a->items[i]);")
    lines.append("  for (int i = 0; i < b->count; ++i) list_push(out, b->items[i]);")
    lines.append("  return out;")
    lines.append("}")
    lines.append("")
    lines.append("static value *vconcat(value *a, value *b) {")
    lines.append("  if (a->kind == VK_LIST && b->kind == VK_LIST) return list_concat_value(a, b);")
    lines.append("  char *left_s = fmt(a);")
    lines.append("  char *right_s = fmt(b);")
    lines.append("  size_t nn = strlen(left_s) + strlen(right_s) + 1;")
    lines.append("  char *buf = (char *)malloc(nn);")
    lines.append("  if (buf == NULL) { fprintf(stderr, \"out of memory\\n\"); exit(1); }")
    lines.append("  strcpy(buf, left_s);")
    lines.append("  strcat(buf, right_s);")
    lines.append("  free(left_s);")
    lines.append("  free(right_s);")
    lines.append("  value *out = vmake(VK_STR);")
    lines.append("  out->str = buf;")
    lines.append("  return out;")
    lines.append("}")
    lines.append("")
    lines.append("static value *index_get(value *container, value *key) {")
    lines.append("  if (container->kind == VK_LIST) return list_get(container, to_int(key->str));")
    lines.append("  if (container->kind == VK_MAP) return map_get(container, key->str ? key->str : \"\");")
    lines.append("  fprintf(stderr, \"index_get_non_container\\n\");")
    lines.append("  exit(1);")
    lines.append("}")
    lines.append("")
    lines.append("static void index_set(value *container, value *key, value *item) {")
    lines.append("  if (container->kind == VK_LIST) { list_set(container, to_int(key->str), item); return; }")
    lines.append("  if (container->kind == VK_MAP) { map_set(container, key->str ? key->str : \"\", item); return; }")
    lines.append("  fprintf(stderr, \"index_set_non_container\\n\");")
    lines.append("  exit(1);")
    lines.append("}")
    lines.append("")
    lines.append("static value *contains_value(value *container, value *needle) {")
    lines.append("  if (container->kind == VK_LIST) {")
    lines.append("    for (int i = 0; i < container->count; ++i) {")
    lines.append("      value *item = container->items[i];")
    lines.append("      if (item == needle) return vstr(\"true\");")
    lines.append("      if (item->str != NULL && needle->str != NULL && strcmp(item->str, needle->str) == 0) return vstr(\"true\");")
    lines.append("    }")
    lines.append("    return vstr(\"false\");")
    lines.append("  }")
    lines.append("  if (container->kind == VK_MAP) {")
    lines.append("    return vstr(map_find(container, needle->str ? needle->str : \"\") >= 0 ? \"true\" : \"false\");")
    lines.append("  }")
    lines.append("  fprintf(stderr, \"contains_non_container\\n\");")
    lines.append("  exit(1);")
    lines.append("}")
    lines.append("")
    lines.append("static bool is_error(value *v) {")
    lines.append("  return v != NULL && v->kind == VK_ERROR;")
    lines.append("}")
    lines.append("")
    lines.append("static value *list_append_value(value *list, value *item) {")
    lines.append("  if (list->kind != VK_LIST) { fprintf(stderr, \"push_non_list\\n\"); exit(1); }")
    lines.append("  list_push(list, item);")
    lines.append("  return list;")
    lines.append("}")
    lines.append("")
    lines.append("static value *list_reverse_value(value *list) {")
    lines.append("  if (list->kind != VK_LIST) { fprintf(stderr, \"reverse_non_list\\n\"); exit(1); }")
    lines.append("  value *out = vlist();")
    lines.append("  for (int i = list->count - 1; i >= 0; --i) list_push(out, list->items[i]);")
    lines.append("  return out;")
    lines.append("}")
    lines.append("")
    lines.append("static value *list_join_value(value *list, const char *separator) {")
    lines.append("  if (list->kind != VK_LIST) { fprintf(stderr, \"join_non_list\\n\"); exit(1); }")
    lines.append("  size_t total = 1;")
    lines.append("  size_t sep_len = strlen(separator ? separator : \"\");")
    lines.append("  char **parts = (char **)malloc(sizeof(char *) * (list->count > 0 ? list->count : 1));")
    lines.append("  for (int i = 0; i < list->count; ++i) {")
    lines.append("    parts[i] = fmt(list->items[i]);")
    lines.append("    total += strlen(parts[i]) + (i > 0 ? sep_len : 0);")
    lines.append("  }")
    lines.append("  char *buf = (char *)malloc(total);")
    lines.append("  if (buf == NULL) { fprintf(stderr, \"out of memory\\n\"); exit(1); }")
    lines.append("  buf[0] = 0;")
    lines.append("  for (int i = 0; i < list->count; ++i) {")
    lines.append("    if (i > 0) strcat(buf, separator ? separator : \"\");")
    lines.append("    strcat(buf, parts[i]);")
    lines.append("    free(parts[i]);")
    lines.append("  }")
    lines.append("  free(parts);")
    lines.append("  value *out = vstr(buf);")
    lines.append("  free(buf);")
    lines.append("  return out;")
    lines.append("}")
    lines.append("")
    lines.append("static value *unwrap_value(value *v) {")
    lines.append("  if (v->kind == VK_SOME) return v->items[0];")
    lines.append("  if (v->kind == VK_ERROR) { fprintf(stderr, \"unwrapped error: %s\\n\", v->str ? v->str : \"\"); exit(1); }")
    lines.append("  if (v->kind == VK_NONE) { fprintf(stderr, \"unwrapped none\\n\"); exit(1); }")
    lines.append("  return v;")
    lines.append("}")
    lines.append("")
    lines.append("static value *option_unwrap_or_value(value *v, value *fallback) {")
    lines.append("  if (v->kind == VK_SOME) return v->items[0];")
    lines.append("  return fallback;")
    lines.append("}")
    lines.append("")
    lines.append("static const char *diagnostic_format(const char *code, const char *message) {")
    lines.append("  static char buf[512];")
    lines.append("  snprintf(buf, sizeof(buf), \"%s %s\", code ? code : \"ZAP-DIAG-000\", message ? message : \"\");")
    lines.append("  return buf;")
    lines.append("}")
    lines.append("")
    lines.append("static value *kind_of(value *v) {")
    lines.append("  if (v == NULL) return vstr(\"unknown\");")
    lines.append("  switch (v->kind) {")
    lines.append("    case VK_LIST: return vstr(\"list\");")
    lines.append("    case VK_MAP: return vstr(\"map\");")
    lines.append("    case VK_ERROR: return vstr(\"error\");")
    lines.append("    case VK_SOME: return vstr(\"some\");")
    lines.append("    case VK_NONE: return vstr(\"none\");")
    lines.append("    case VK_TASK: return vstr(\"task\");")
    lines.append("    default: break;")
    lines.append("  }")
    lines.append("  if (is_int_str(v->str)) return vstr(\"int\");")
    lines.append("  return vstr(\"str\");")
    lines.append("}")
    lines.append("")
    lines.append("#define ZAP_READ_MAX 4194304")
    lines.append("static int g_argc = 0;")
    lines.append("static char **g_argv = NULL;")
    lines.append("")
    lines.append("static value *builtin_argv(void) {")
    lines.append("  value *out = vlist();")
    lines.append("  for (int i = 1; i < g_argc; ++i) list_push(out, vstr(g_argv[i]));")
    lines.append("  return out;")
    lines.append("}")
    lines.append("")
    lines.append("static value *read_file_text(const char *path) {")
    lines.append("  FILE *fh = fopen(path, \"rb\");")
    lines.append("  if (fh == NULL) return verr(\"read_file_failed\");")
    lines.append("  size_t cap = 4096;")
    lines.append("  size_t len = 0;")
    lines.append("  char *buf = (char *)malloc(cap);")
    lines.append("  if (buf == NULL) { fclose(fh); return verr(\"out_of_memory\"); }")
    lines.append("  size_t n;")
    lines.append("  while ((n = fread(buf + len, 1, cap - len - 1, fh)) > 0) {")
    lines.append("    len += n;")
    lines.append("    if (len + 1 >= cap) {")
    lines.append("      if (cap >= ZAP_READ_MAX) break;")
    lines.append("      cap *= 2;")
    lines.append("      buf = (char *)realloc(buf, cap);")
    lines.append("      if (buf == NULL) { fclose(fh); return verr(\"out_of_memory\"); }")
    lines.append("    }")
    lines.append("  }")
    lines.append("  fclose(fh);")
    lines.append("  buf[len] = 0;")
    lines.append("  value *out = vstr(buf);")
    lines.append("  free(buf);")
    lines.append("  return out;")
    lines.append("}")
    lines.append("")
    lines.append("static value *write_file_text(const char *path, const char *content) {")
    lines.append("  FILE *fh = fopen(path, \"wb\");")
    lines.append("  if (fh == NULL) return vstr(\"false\");")
    lines.append("  if (content != NULL && content[0] != 0) fwrite(content, 1, strlen(content), fh);")
    lines.append("  fclose(fh);")
    lines.append("  return vstr(\"true\");")
    lines.append("}")
    lines.append("")
    lines.append("static value *file_exists(const char *path) {")
    lines.append("  FILE *fh = fopen(path, \"rb\");")
    lines.append("  if (fh == NULL) return vstr(\"false\");")
    lines.append("  fclose(fh);")
    lines.append("  return vstr(\"true\");")
    lines.append("}")
    lines.append("")
    lines.append("static value *string_length(value *v) {")
    lines.append("  const char *s = (v != NULL && v->str != NULL) ? v->str : \"\";")
    lines.append("  return vstr(from_int((int32_t)strlen(s)));")
    lines.append("}")
    lines.append("")
    lines.append("static bool string_equals(value *a, value *b) {")
    lines.append("  const char *l = (a != NULL && a->str != NULL) ? a->str : \"\";")
    lines.append("  const char *r = (b != NULL && b->str != NULL) ? b->str : \"\";")
    lines.append("  return strcmp(l, r) == 0;")
    lines.append("}")
    lines.append("")
    lines.append("static bool values_equal(value *a, value *b) {")
    lines.append("  if (a == b) return true;")
    lines.append("  if (a == NULL || b == NULL) return false;")
    lines.append("  if (a->kind != b->kind) return false;")
    lines.append("  switch (a->kind) {")
    lines.append("    case VK_STR: return string_equals(a, b);")
    lines.append("    case VK_NONE: return true;")
    lines.append("    case VK_ERROR: return string_equals(a, b);")
    lines.append("    case VK_SOME: return values_equal(a->items[0], b->items[0]);")
    lines.append("    case VK_LIST: {")
    lines.append("      if (a->count != b->count) return false;")
    lines.append("      for (int i = 0; i < a->count; ++i) {")
    lines.append("        if (!values_equal(a->items[i], b->items[i])) return false;")
    lines.append("      }")
    lines.append("      return true;")
    lines.append("    }")
    lines.append("    case VK_MAP: {")
    lines.append("      if (a->count != b->count) return false;")
    lines.append("      for (int i = 0; i < a->count; ++i) {")
    lines.append("        int idx = map_find(b, a->items[i]->str);")
    lines.append("        if (idx < 0) return false;")
    lines.append("        if (!values_equal(a->vals[i], b->vals[idx])) return false;")
    lines.append("      }")
    lines.append("      return true;")
    lines.append("    }")
    lines.append("    case VK_TASK: return a->task_id == b->task_id;")
    lines.append("  }")
    lines.append("  return false;")
    lines.append("}")
    lines.append("")
    lines.append("static value *json_serialize(value *v);")
    lines.append("")
    lines.append("static value *json_serialize(value *v) {")
    lines.append("  char *s = fmt(v);")
    lines.append("  value *out = vstr(s);")
    lines.append("  free(s);")
    lines.append("  return out;")
    lines.append("}")
    lines.append("")
    lines.append("")
    lines.append("static char *fmt(value *v);")
    lines.append("")
    lines.append("static char *fmt(value *v) {")
    lines.append("  if (v == NULL) return strdup(\"\");")
    lines.append("  if (v->kind == VK_STR) return strdup(v->str ? v->str : \"\");")
    lines.append("  if (v->kind == VK_NONE) return strdup(\"none\");")
    lines.append("  if (v->kind == VK_ERROR) {")
    lines.append("    size_t n = strlen(v->str ? v->str : \"\") + 32;")
    lines.append("    char *buf = (char *)malloc(n);")
    lines.append("    snprintf(buf, n, \"error(\\\"%s\\\")\", v->str ? v->str : \"\");")
    lines.append("    return buf;")
    lines.append("  }")
    lines.append("  if (v->kind == VK_SOME) {")
    lines.append("    char *inner = fmt(v->items[0]);")
    lines.append("    size_t n = strlen(inner) + 16;")
    lines.append("    char *buf = (char *)malloc(n);")
    lines.append("    snprintf(buf, n, \"some(%s)\", inner);")
    lines.append("    free(inner);")
    lines.append("    return buf;")
    lines.append("  }")
    lines.append("  if (v->kind == VK_TASK) {")
    lines.append("    char *buf = (char *)malloc(48);")
    lines.append("    snprintf(buf, 48, \"<task %d>\", v->task_id);")
    lines.append("    return buf;")
    lines.append("  }")
    lines.append("  if (v->kind == VK_LIST) {")
    lines.append("    size_t n = 16;")
    lines.append("    char **parts = (char **)malloc(sizeof(char *) * (v->count > 0 ? v->count : 1));")
    lines.append("    for (int i = 0; i < v->count; ++i) { parts[i] = fmt(v->items[i]); n += strlen(parts[i]) + 2; }")
    lines.append("    char *buf = (char *)malloc(n);")
    lines.append("    strcpy(buf, \"[\");")
    lines.append("    for (int i = 0; i < v->count; ++i) {")
    lines.append("      if (i > 0) strcat(buf, \", \");")
    lines.append("      strcat(buf, parts[i]);")
    lines.append("      free(parts[i]);")
    lines.append("    }")
    lines.append("    strcat(buf, \"]\");")
    lines.append("    free(parts);")
    lines.append("    return buf;")
    lines.append("  }")
    lines.append("  if (v->kind == VK_MAP) {")
    lines.append("    size_t n = 16;")
    lines.append("    char **parts = (char **)malloc(sizeof(char *) * (v->count > 0 ? v->count : 1));")
    lines.append("    for (int i = 0; i < v->count; ++i) {")
    lines.append("      char *val_s = fmt(v->vals[i]);")
    lines.append("      parts[i] = (char *)malloc(strlen(v->items[i]->str) + strlen(val_s) + 4);")
    lines.append("      sprintf(parts[i], \"%s: %s\", v->items[i]->str, val_s);")
    lines.append("      free(val_s);")
    lines.append("      n += strlen(parts[i]) + 2;")
    lines.append("    }")
    lines.append("    char *buf = (char *)malloc(n);")
    lines.append("    strcpy(buf, \"{\");")
    lines.append("    for (int i = 0; i < v->count; ++i) {")
    lines.append("      if (i > 0) strcat(buf, \", \");")
    lines.append("      strcat(buf, parts[i]);")
    lines.append("      free(parts[i]);")
    lines.append("    }")
    lines.append("    strcat(buf, \"}\");")
    lines.append("    free(parts);")
    lines.append("    return buf;")
    lines.append("  }")
    lines.append("  return strdup(\"\");")
    lines.append("}")
    lines.append("")
    lines.append("static void print_value(state *st, value *v) {")
    lines.append("  char *s = fmt(v);")
    lines.append("  if (st->output_count >= st->output_cap) {")
    lines.append("    st->output_cap = st->output_cap ? st->output_cap * 2 : 16;")
    lines.append("    st->output = (char **)realloc(st->output, sizeof(char *) * st->output_cap);")
    lines.append("  }")
    lines.append("  st->output[st->output_count++] = s;")
    lines.append("}")
    lines.append("")
    lines.append("static void call_push(state *st, int return_site, int argc) {")
    lines.append("  st->frames = (frame *)realloc(st->frames, sizeof(frame) * (st->frame_count + 1));")
    lines.append("  frame *f = &st->frames[st->frame_count++];")
    lines.append("  f->return_site = return_site;")
    lines.append("  f->saved_locals = st->locals;")
    lines.append("  f->saved_local_count = st->local_count;")
    lines.append("  f->saved_stack_count = st->stack_count;")
    lines.append("  f->argc = argc;")
    lines.append("  /* The callee gets a fresh local array so parameter binding never")
    lines.append("     reallocates (or clobbers) the caller's locals. */")
    lines.append("  st->locals = NULL;")
    lines.append("  st->local_count = 0;")
    lines.append("}")
    lines.append("")
    lines.append("static int call_pop(state *st) {")
    lines.append("  if (st->frame_count == 0) return -1;")
    lines.append("  frame *f = &st->frames[--st->frame_count];")
    lines.append("  value *ret = NULL;")
    lines.append("  int callee_base = f->saved_stack_count - f->argc;")
    lines.append("  if (st->stack_count > callee_base) {")
    lines.append("    ret = st->stack[st->stack_count - 1];")
    lines.append("  }")
    lines.append("  st->stack_count = callee_base;")
    lines.append("  free(st->locals);")
    lines.append("  st->locals = f->saved_locals;")
    lines.append("  st->local_count = f->saved_local_count;")
    lines.append("  if (ret != NULL) push(st, ret);")
    lines.append("  return f->return_site;")
    lines.append("}")
    lines.append("")
    lines.append("#define ZAP_MODULES_MAX 64")
    lines.append("static value *g_modules[ZAP_MODULES_MAX];")
    lines.append("static char *g_module_names[ZAP_MODULES_MAX];")
    lines.append("static int g_module_count = 0;")
    lines.append("static value *g_exports[ZAP_MODULES_MAX];")
    lines.append("static int g_active_module = -1;")
    lines.append("")
    lines.append("static value *module_registry(const char *name) {")
    lines.append("  for (int i = 0; i < g_module_count; ++i) {")
    lines.append("    if (strcmp(g_module_names[i], name) == 0) { g_active_module = i; return g_modules[i]; }")
    lines.append("  }")
    lines.append("  if (g_module_count >= ZAP_MODULES_MAX) { fprintf(stderr, \"module registry exhausted\\n\"); exit(1); }")
    lines.append("  g_modules[g_module_count] = vmap();")
    lines.append("  g_module_names[g_module_count] = strdup(name);")
    lines.append("  g_exports[g_module_count] = vmap();")
    lines.append("  g_active_module = g_module_count;")
    lines.append("  return g_modules[g_module_count++];")
    lines.append("}")
    lines.append("")
    lines.append("static void export_value(const char *name, value *v) {")
    lines.append("  if (g_module_count == 0) { module_registry(\"__main__\"); g_active_module = 0; }")
    lines.append("  map_set(g_exports[g_active_module], name, v);")
    lines.append("}")
    lines.append("")
    lines.append("#define ZAP_TASKS_MAX 256")
    lines.append("static value *g_task_results[ZAP_TASKS_MAX];")
    lines.append("static int g_task_count = 0;")
    lines.append("")
    lines.append("static value *task_run(int id) {")
    lines.append("  if (id >= 0 && id < g_task_count && g_task_results[id] != NULL) return g_task_results[id];")
    lines.append("  return vstr(\"\");")
    lines.append("}")
    lines.append("")
    lines.append("static value *async_new_value(value *payload) {")
    lines.append("  if (g_task_count >= ZAP_TASKS_MAX) { fprintf(stderr, \"task registry exhausted\\n\"); exit(1); }")
    lines.append("  g_task_results[g_task_count] = payload;")
    lines.append("  value *task = vtask(g_task_count);")
    lines.append("  g_task_count++;")
    lines.append("  return task;")
    lines.append("}")
    lines.append("")
    lines.append("static void free_all(void) {")
    lines.append("  for (int i = 0; i < g_heap_count; ++i) {")
    lines.append("    value *v = g_heap[i];")
    lines.append("    if (v->kind == VK_STR || v->kind == VK_ERROR) free(v->str);")
    lines.append("    free(v->items);")
    lines.append("    free(v->vals);")
    lines.append("    free(v);")
    lines.append("  }")
    lines.append("  g_heap_count = 0;")
    lines.append("  for (int i = 0; i < g_module_count; ++i) free(g_module_names[i]);")
    lines.append("  g_module_count = 0;")
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
    entry_bindings = {}
    for info in functions.values():
        entry_bindings[info["entry"]] = list(reversed(info["params"]))

    name_index = {}
    current_index = [0]

    def _name_index(name):
        if name not in name_index:
            name_index[name] = current_index[0]
            current_index[0] += 1
        return name_index[name]

    # Scope model: top-level bindings live in the process-wide globals table so
    # function frames can read (and mutate) them, while function-internal names
    # (including parameters) live in the per-call local array. A name that is
    # used as a parameter anywhere is treated as function-scoped everywhere so
    # shadowing stays consistent.
    function_ranges = []
    for instr in program:
        if instr.get("op") == "function_def":
            function_ranges.append((instr.get("entry", 0),
                                    instr.get("end", instr.get("entry", 0))))

    def _in_function(instr_index):
        for start, end in function_ranges:
            if start <= instr_index < end:
                return True
        return False

    param_names = set()
    for info in functions.values():
        param_names.update(info["params"])

    top_level_names = set()
    for instr_index, instr in enumerate(program):
        if instr.get("op") not in ("store", "set_index"):
            continue
        if _in_function(instr_index):
            continue
        bound = instr.get("name")
        if bound and bound not in param_names:
            top_level_names.add(bound)

    global_index = {}

    def _global_index(name):
        if name not in global_index:
            global_index[name] = len(global_index)
        return global_index[name]

    def _store(name):
        if name in top_level_names:
            return f"  store_global(&st, {_global_index(name)}, pop(&st));"
        return f"  store_local(&st, {_name_index(name)}, pop(&st));"

    def _load(name):
        if name in top_level_names:
            return f"  push(&st, load_global(&st, {_global_index(name)}));"
        return f"  push(&st, load_local(&st, {_name_index(name)}, {json.dumps(name)}));"

    # Jump destinations and function entries become labels.
    label_targets = set()
    for idx, instr in enumerate(program):
        if instr.get("op") in ("jump", "jump_if_false", "jump_if_true"):
            label_targets.add(instr.get("target", idx))
    for fn_info in functions.values():
        label_targets.add(fn_info["entry"])
    label_targets.add(len(program))
    for idx, instr in enumerate(program):
        if instr.get("op") == "function_def":
            label_targets.add(instr.get("end", 0))
        elif instr.get("op") in ("and", "or"):
            short_label = idx + len(program) + 1
            end_label = idx + len(program) + 2
            label_targets.add(short_label)
            label_targets.add(end_label)

    # Call sites get sequential ids for return dispatch (precomputed so
    # return instructions can reference every return site). Only calls to
    # Zap-defined functions create frames, so only those sites get a return
    # label; builtin calls are dispatched natively without a frame.
    return_sites = []
    site_ids = {}
    for idx, instr in enumerate(program):
        if instr.get("op") == "call":
            site_ids[idx] = idx + 1
            if instr.get("name") in functions or instr.get("name") not in _BUILTIN_SPECS:
                return_sites.append(idx + 1)
    # Return sites need their own labels to avoid conflicting with instruction indices
    for site in return_sites:
        label_targets.add(site)
    call_site_count = len(return_sites)

    def _const_push(value):
        """Emit a stack push for a constant with the correct value kind."""
        if isinstance(value, bool):
            return '  push(&st, vstr("%s"));' % ("true" if value else "false")
        if isinstance(value, (int, float)):
            return "  push(&st, vstr(from_int(%d)));" % int(value)
        return "  push(&st, vstr(%s));" % json.dumps(str(value))

    lines.append("int main(int argc, char **argv) {")
    lines.append("  g_argc = argc;")
    lines.append("  g_argv = argv;")
    lines.append("  state st = {0};")
    lines.append("  value *a;")
    lines.append("  value *b;")
    lines.append("  value *c;")
    lines.append("  int32_t ia;")
    lines.append("  int32_t ib;")
    lines.append("  int target;")
    lines.append("")
    site_counter = 0
    for idx, instr in enumerate(program):
        if idx in label_targets:
            lines.append(f"label_{idx}: ;")
        if idx in entry_bindings:
            for param in entry_bindings[idx]:
                lines.append(f"  store_local(&st, {_name_index(param)}, pop(&st));")
        op = instr.get("op")
        if op == "const":
            lines.append(_const_push(instr["value"]))
        elif op == "store":
            lines.append(_store(instr["name"]))
        elif op == "load":
            lines.append(_load(instr["name"]))
        elif op == "jump":
            lines.append(f"  goto label_{instr['target']};")
        elif op == "jump_if_false":
            lines.append("  a = pop(&st);")
            lines.append(f"  if (!to_bool(a)) goto label_{instr['target']};")
        elif op == "jump_if_true":
            lines.append("  a = pop(&st);")
            lines.append(f"  if (to_bool(a)) goto label_{instr['target']};")
        elif op == "and":
            lines.append("  a = pop(&st);")
            lines.append("  b = pop(&st);")
            short_label = idx + len(program) + 1
            end_label = idx + len(program) + 2
            lines.append("  if (!to_bool(b)) { goto label_%d; }" % short_label)
            lines.append("  push(&st, vstr(to_bool(a) ? \"true\" : \"false\"));")
            lines.append("  goto label_%d;" % end_label)
            lines.append("label_%d:" % short_label)
            lines.append("  push(&st, vstr(\"false\"));")
            lines.append("label_%d:" % end_label)
        elif op == "or":
            lines.append("  a = pop(&st);")
            lines.append("  b = pop(&st);")
            short_label = idx + len(program) + 1
            end_label = idx + len(program) + 2
            lines.append("  if (to_bool(b)) { goto label_%d; }" % short_label)
            lines.append("  push(&st, vstr(to_bool(a) ? \"true\" : \"false\"));")
            lines.append("  goto label_%d;" % end_label)
            lines.append("label_%d:" % short_label)
            lines.append("  push(&st, vstr(\"true\"));")
            lines.append("label_%d:" % end_label)
        elif op in ("add", "subtract", "multiply", "divide", "remainder", "less", "greater",
                    "equal", "not_equal", "less_equal", "greater_equal", "in",
                    "str_concat", "list_concat"):
                lines.append("  b = pop(&st);")
                lines.append("  a = pop(&st);")
                lines.append("  ia = to_int(a->str);")
                lines.append("  ib = to_int(b->str);")
                if op == "add":
                    lines.append("  if (is_int_str(a->str) && is_int_str(b->str)) push(&st, vstr(from_int(ia + ib)));")
                    lines.append("  else push(&st, vconcat(a, b));")
                elif op == "subtract":
                    lines.append("  push(&st, vstr(from_int(ia - ib)));")
                elif op == "multiply":
                    lines.append("  push(&st, vstr(from_int(ia * ib)));")
                elif op == "divide":
                    lines.append("  if (ib == 0) { fprintf(stderr, \"division by zero\\n\"); exit(1); }")
                    lines.append("  push(&st, vstr(from_int(ia / ib)));")
                elif op == "remainder":
                    lines.append("  if (ib == 0) { fprintf(stderr, \"division by zero\\n\"); exit(1); }")
                    lines.append("  push(&st, vstr(from_int(ia % ib)));")
                elif op == "less":
                    lines.append("  push(&st, vstr(ia < ib ? \"true\" : \"false\"));")
                elif op == "less_equal":
                    lines.append("  push(&st, vstr(ia <= ib ? \"true\" : \"false\"));")
                elif op == "greater":
                    lines.append("  push(&st, vstr(ia > ib ? \"true\" : \"false\"));")
                elif op == "greater_equal":
                    lines.append("  push(&st, vstr(ia >= ib ? \"true\" : \"false\"));")
                elif op == "equal":
                    lines.append("  push(&st, vstr(values_equal(a, b) ? \"true\" : \"false\"));")
                elif op == "not_equal":
                    lines.append("  push(&st, vstr(values_equal(a, b) ? \"false\" : \"true\"));")
                elif op == "in":
                    lines.append("  push(&st, contains_value(b, a));")
                elif op == "list_concat":
                    lines.append("  push(&st, vconcat(a, b));")
                elif op == "str_concat":
                    lines.append("  push(&st, vconcat(a, b));")
        elif op == "not":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, vstr(to_bool(a) ? \"false\" : \"true\"));")
        elif op == "dup":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, a);")
            lines.append("  push(&st, a);")
        elif op == "pop":
            lines.append("  (void)pop(&st);")
        elif op == "print":
            lines.append("  print_value(&st, pop(&st));")
        elif op == "return_value":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, a);")
            lines.append("  ia = call_pop(&st);")
            lines.append(f"  if (ia < 0) {{ goto label_{len(program)}; }}")
            for site in return_sites:
                lines.append(f"  if (ia == {site}) {{ goto label_{site}; }}")
            lines.append("  fprintf(stderr, \"invalid return site\\n\");")
            lines.append("  exit(1);")
        elif op == "return_none":
            lines.append("  push(&st, vnone());")
            lines.append("  ia = call_pop(&st);")
            lines.append(f"  if (ia < 0) {{ goto label_{len(program)}; }}")
            for site in return_sites:
                lines.append(f"  if (ia == {site}) {{ goto label_{site}; }}")
            lines.append("  fprintf(stderr, \"invalid return site\\n\");")
            lines.append("  exit(1);")
        elif op == "make_list":
            count = int(instr.get("count", 0))
            lines.append("  {")
            lines.append("    value *elems[64];")
            lines.append(f"    if ({count} > 64) {{ fprintf(stderr, \"make_list too large\\n\"); exit(1); }}")
            lines.append(f"    for (ia = {count} - 1; ia >= 0; --ia) elems[ia] = pop(&st);")
            lines.append("    c = vlist();")
            lines.append(f"    for (ia = 0; ia < {count}; ++ia) list_push(c, elems[ia]);")
            lines.append("    push(&st, c);")
            lines.append("  }")
        elif op == "list_len":
            lines.append("  a = pop(&st);")
            lines.append("  if (a->kind != VK_LIST && a->kind != VK_MAP) { fprintf(stderr, \"list_len_non_list\\n\"); exit(1); }")
            lines.append("  push(&st, vstr(from_int(a->count)));")
        elif op == "list_get":
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, index_get(a, b));")
        elif op == "index":
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, index_get(a, b));")
        elif op == "list_set":
            lines.append("  c = pop(&st);")
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  index_set(a, b, c);")
            lines.append("  push(&st, a);")
        elif op == "list_append":
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  list_push(a, b);")
            lines.append("  push(&st, a);")
        elif op == "list_reverse":
            lines.append("  a = pop(&st);")
            lines.append("  c = vlist();")
            lines.append("  for (ia = a->count - 1; ia >= 0; --ia) list_push(c, a->items[ia]);")
            lines.append("  push(&st, c);")
        elif op == "list_contains":
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, contains_value(a, b));")
        elif op == "make_map":
            lines.append("  push(&st, vmap());")
        elif op == "map_set_pair":
            lines.append("  c = pop(&st);")
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  map_set(a, b->str ? b->str : \"\", c);")
            lines.append("  push(&st, a);")
        elif op == "map_get":
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, map_get(a, b->str ? b->str : \"\"));")
        elif op == "map_set":
            lines.append("  c = pop(&st);")
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  map_set(a, b->str ? b->str : \"\", c);")
            lines.append("  push(&st, a);")
        elif op == "map_has_key":
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, map_has_key(a, b->str ? b->str : \"\"));")
        elif op == "map_keys":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, map_keys(a));")
        elif op == "map_values":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, map_values(a));")
        elif op == "struct_new":
            lines.append("  push(&st, vmap());")
        elif op == "struct_get":
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, map_get(a, b->str ? b->str : \"\"));")
        elif op == "struct_set":
            lines.append("  c = pop(&st);")
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  map_set(a, b->str ? b->str : \"\", c);")
            lines.append("  push(&st, a);")
        elif op == "struct_has_field":
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, map_has_key(a, b->str ? b->str : \"\"));")
        elif op == "struct_field_names":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, map_keys(a));")
        elif op == "error_new":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, verr(a->str ? a->str : \"\"));")
        elif op == "error_message":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, vstr(is_error(a) && a->str ? a->str : \"\"));")
        elif op == "error_is_error":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, vstr(is_error(a) ? \"true\" : \"false\"));")
        elif op == "error_unwrap":
            lines.append("  a = pop(&st);")
            lines.append("  if (is_error(a)) { fprintf(stderr, \"unwrap on error: %s\\n\", a->str ? a->str : \"\"); exit(1); }")
            lines.append("  push(&st, a);")
        elif op == "option_some":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, vsome(a));")
        elif op == "option_none":
            lines.append("  push(&st, vnone());")
        elif op == "option_is_some":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, vstr(a->kind == VK_SOME ? \"true\" : \"false\"));")
        elif op == "option_is_none":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, vstr(a->kind == VK_NONE ? \"true\" : \"false\"));")
        elif op == "option_unwrap":
            lines.append("  a = pop(&st);")
            lines.append("  if (a->kind != VK_SOME) { fprintf(stderr, \"unwrap on none\\n\"); exit(1); }")
            lines.append("  push(&st, a->items[0]);")
        elif op == "option_unwrap_or":
            lines.append("  b = pop(&st);")
            lines.append("  a = pop(&st);")
            lines.append("  if (a->kind == VK_SOME) push(&st, a->items[0]);")
            lines.append("  else push(&st, b);")
        elif op == "async_new":
            lines.append("  a = pop(&st);")
            lines.append("  if (g_task_count >= ZAP_TASKS_MAX) { fprintf(stderr, \"task registry exhausted\\n\"); exit(1); }")
            lines.append("  g_task_results[g_task_count] = a;")
            lines.append("  push(&st, vtask(g_task_count));")
            lines.append("  g_task_count++;")
        elif op == "await":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, task_run(a->task_id));")
        elif op == "import_module":
            lines.append("  a = pop(&st);")
            lines.append("  push(&st, module_registry(a->str ? a->str : \"\"));")
        elif op == "import_symbol":
            lines.append("  c = pop(&st);")
            lines.append("  b = pop(&st);")
            lines.append("  push(&st, map_get(b, c->str ? c->str : \"\"));")
        elif op == "export_value":
            lines.append("  c = pop(&st);")
            lines.append("  b = pop(&st);")
            lines.append("  export_value(b->str ? b->str : \"\", c);")
        elif op == "function_def":
            lines.append(f"  target = {instr.get('end', idx + 1)};")
            lines.append(f"  goto label_{instr.get('end', idx + 1)};")
        elif op == "call":
            name = instr["name"]
            argc = int(instr.get("argc", 0))
            site = site_ids[idx]
            fn_info = functions.get(name)
            builtin = _BUILTIN_SPECS.get(name)
            if fn_info is None and builtin is None:
                lines.append(f"  fprintf(stderr, \"unknown function: {name}\\n\");")
                lines.append("  exit(1);")
            elif builtin is not None:
                arity, body = builtin
                if argc != arity:
                    lines.append(f"  fprintf(stderr, \"arity mismatch calling builtin {name}\\n\");")
                    lines.append("  exit(1);")
                else:
                    lines.append("  {")
                    lines.append(f"    value *bi_args[{max(arity, 1)}];")
                    lines.append(f"    for (ia = {arity} - 1; ia >= 0; --ia) bi_args[ia] = pop(&st);")
                    lines.append(f"    push(&st, {body});")
                    lines.append("  }")
            else:
                if argc != len(fn_info["params"]):
                    lines.append(f"  fprintf(stderr, \"arity mismatch calling {name}\\n\");")
                    lines.append("  exit(1);")
                lines.append(f"  call_push(&st, {site}, {argc});")
                lines.append(f"  target = {fn_info['entry']};")
                lines.append(f"  goto label_{fn_info['entry']};")
        elif op == "halt":
            lines.append(f"  goto label_{len(program)};")
        else:
            lines.append(f"  fprintf(stderr, \"unsupported opcode: {op}\\n\");")
            lines.append("  exit(1);")
    lines.append(f"label_{len(program)}: ;")
    lines.append("  free_all();")
    lines.append("  for (ia = 0; ia < st.output_count; ++ia) {")
    lines.append("    printf(\"%s\\n\", st.output[ia]);")
    lines.append("  }")
    lines.append("  return 0;")
    lines.append("}")

    with open(out_path, "w", newline="\n") as fh:
        fh.write("\n".join(lines) + "\n")
    return "\n".join(lines)


def compile_program_to_c(source, out_path):
    """Compile Zap source all the way to an emitted C file."""
    program = compile_program(source)
    return emit_c(program, out_path)


def find_c_compiler():
    """Locate a system C compiler without ever consulting Rust/Cargo."""
    preferred = [
        os.environ.get("ZAP_CC"),
        os.environ.get("CC"),
        shutil.which("gcc"),
        shutil.which("clang"),
        shutil.which("cc"),
    ]
    for candidate in preferred:
        if candidate and os.path.isfile(candidate) and os.access(candidate, os.X_OK):
            return candidate
    if _is_windows_host():
        cl_candidates = [
            os.environ.get("ZAP_MSVC_CL"),
        ]
        for base in (
            r"C:\Program Files\Microsoft Visual Studio",
            r"C:\Program Files (x86)\Microsoft Visual Studio",
        ):
            if not os.path.isdir(base):
                continue
            for root, dirs, files in os.walk(base):
                if "cl.exe" in files and "Hostx64" in root:
                    cl_candidates.append(os.path.join(root, "cl.exe"))
        for candidate in cl_candidates:
            if candidate and os.path.isfile(candidate):
                return candidate
        for candidate in (
            r"C:\msys64\mingw64\bin\gcc.exe",
            r"C:\msys64\ucrt64\bin\gcc.exe",
            r"C:\TDM-GCC-64\bin\gcc.exe",
        ):
            if os.path.isfile(candidate):
                return candidate
    return None


def _find_vcvars(cl_path):
    """Locate the vcvars batch file that belongs to a discovered cl.exe."""
    current = os.path.dirname(os.path.abspath(cl_path))
    for _ in range(8):
        candidate = os.path.join(current, "VC", "Auxiliary", "Build", "vcvars64.bat")
        if os.path.isfile(candidate):
            return candidate
        current = os.path.dirname(current)
    for candidate in (
        r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files\Microsoft Visual Studio\2022\Professional\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Auxiliary\Build\vcvars64.bat",
        r"C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat",
    ):
        if os.path.isfile(candidate):
            return candidate
    return None


def compile_c(c_path, out_path, compiler=None, extra_args=None):
    """Compile the emitted C file into a native executable."""
    cc = compiler or find_c_compiler()
    if cc is None:
        raise RuntimeError("no system C compiler found (tried gcc/clang/cc/cl.exe)")
    if os.path.basename(cc).lower().startswith("cl"):
        vcvars = _find_vcvars(cc)
        fd, bat_path = tempfile.mkstemp(suffix=".bat", prefix="zap_c_backend_build_")
        os.close(fd)
        fd, object_path = tempfile.mkstemp(suffix=".obj", prefix="zap_c_backend_")
        os.close(fd)
        try:
            os.remove(object_path)
            bat_lines = ["@echo off"]
            if vcvars:
                bat_lines.append(f'call "{vcvars}" >nul')
            bat_lines.append(
                f'"{cc}" /Brepro /nologo /O2 /Fo:"{object_path}" /Fe:"{out_path}" "{c_path}"')
            with open(bat_path, "w", newline="\r\n") as fh:
                fh.write("\n".join(bat_lines) + "\n")
            args = ["cmd", "/c", bat_path]
        except Exception:
            try:
                os.remove(object_path)
            except OSError:
                pass
            try:
                os.remove(bat_path)
            except OSError:
                pass
            raise
    else:
        bat_path = None
        object_path = None
        args = [cc, "-O2", "-o", out_path, c_path]
    if extra_args:
        args.extend(extra_args)
    result = subprocess.run(args, capture_output=True, text=True)
    if bat_path:
        try:
            os.remove(bat_path)
        except OSError:
            pass
    if object_path:
        try:
            os.remove(object_path)
        except OSError:
            pass
    if result.returncode != 0:
        raise RuntimeError(f"C compiler failed: {' '.join(args)}\n{result.stderr}")
    strip_path = shutil.which("strip")
    if strip_path and not _is_windows_host():
        for section in (".note.gnu.build-id", ".note.gnu.property", ".note.ABI-tag"):
            subprocess.run(
                [strip_path, "--remove-section=" + section, out_path],
                capture_output=True, text=True)
        subprocess.run(
            [strip_path, out_path], capture_output=True, text=True)
    return {
        "compiler": cc,
        "command": args,
        "returncode": result.returncode,
        "stdout": result.stdout,
        "stderr": result.stderr,
    }


def build_native_binary(source, out_path, work_dir=None, compiler=None):
    """Compile Zap source to a native executable via the emitted C file."""
    work_dir = work_dir or os.path.dirname(os.path.abspath(out_path))
    os.makedirs(work_dir, exist_ok=True)
    c_path = os.path.join(work_dir, os.path.splitext(os.path.basename(out_path))[0] + ".c")
    emit_c(compile_program(_strip_bom(source)), c_path)
    compile_c(c_path, out_path, compiler=compiler)
    return out_path


def _strip_bom(source):
    """Remove a UTF-8 BOM so Windows-authored sources compile identically."""
    if source and source[0] == "\ufeff":
        return source[1:]
    return source


def build_native_binary_from_file(source_path, out_prefix, compiler=None):
    """Compile a Zap source file to a native executable and report paths."""
    with open(source_path, "r", encoding="utf-8-sig") as fh:
        source = fh.read()
    exe_path = out_prefix + (".exe" if _is_windows_host() else "")
    build_native_binary(source, exe_path,
                        work_dir=os.path.dirname(os.path.abspath(out_prefix)) or ".",
                        compiler=compiler)
    return {"c_path": out_prefix + ".c", "exe_path": exe_path,
            "compiler": find_c_compiler()}


def main():
    if len(sys.argv) < 2:
        print("Usage: c_backend.py <source.zp> [output_prefix]")
        return 1
    source_path = sys.argv[1]
    prefix = sys.argv[2] if len(sys.argv) > 2 else os.path.splitext(source_path)[0]
    result = build_native_binary_from_file(source_path, prefix)
    print("C backend emitted: %s" % result["c_path"])
    print("C backend executable: %s" % result["exe_path"])
    print("C backend compiler: %s" % result["compiler"])
    return 0


if __name__ == "__main__":
    sys.exit(main())
