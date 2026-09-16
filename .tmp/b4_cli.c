#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>

typedef struct value value;
typedef struct frame frame;

static char *fmt(value *v);

enum { VK_STR = 0, VK_LIST = 1, VK_MAP = 2, VK_ERROR = 3, VK_SOME = 4, VK_NONE = 5, VK_TASK = 6 };

struct value {
  int kind;
  char *str;
  value **items;
  value **vals;
  int count;
  int cap;
  int vcount;
  int vcap;
  int task_id;
};

struct frame {
  int return_site;
  value **saved_locals;
  int saved_local_count;
  int saved_stack_count;
  int argc;
};

#define ZAP_HEAP_MAX 262144
static value *g_heap[ZAP_HEAP_MAX];
static int g_heap_count = 0;

static value *track(value *v) {
  if (g_heap_count >= ZAP_HEAP_MAX) { fprintf(stderr, "heap exhausted\n"); exit(1); }
  g_heap[g_heap_count++] = v;
  return v;
}

static value *vmake(int kind) {
  value *v = (value *)calloc(1, sizeof(value));
  if (v == NULL) { fprintf(stderr, "out of memory\n"); exit(1); }
  v->kind = kind;
  return track(v);
}

static value *vstr(const char *s) {
  value *v = vmake(VK_STR);
  v->str = strdup(s ? s : "");
  return v;
}

static value *vlist(void) {
  return vmake(VK_LIST);
}

static value *vmap(void) {
  return vmake(VK_MAP);
}

static value *verr(const char *msg) {
  value *v = vmake(VK_ERROR);
  v->str = strdup(msg ? msg : "");
  return v;
}

static value *vsome(value *payload) {
  value *v = vmake(VK_SOME);
  v->items = (value **)malloc(sizeof(value *));
  v->items[0] = payload;
  v->count = 1;
  v->cap = 1;
  return v;
}

static value *vnone(void) {
  return vmake(VK_NONE);
}

static value *vtask(int id) {
  value *v = vmake(VK_TASK);
  v->task_id = id;
  return v;
}

typedef struct {
  value **stack;
  int stack_count;
  int stack_cap;
  value **locals;
  int local_count;
  char **output;
  int output_count;
  int output_cap;
  frame *frames;
  int frame_count;
} state;

static void push(state *st, value *v) {
  if (st->stack_count >= st->stack_cap) {
    st->stack_cap = st->stack_cap ? st->stack_cap * 2 : 16;
    st->stack = (value **)realloc(st->stack, sizeof(value *) * st->stack_cap);
  }
  st->stack[st->stack_count++] = v;
}

static value *pop(state *st) {
  if (st->stack_count <= 0) { fprintf(stderr, "stack underflow\n"); exit(1); }
  return st->stack[--st->stack_count];
}

static void store_local(state *st, int index, value *v) {
  if (index >= st->local_count) {
    int new_count = index + 1;
    st->locals = (value **)realloc(st->locals, sizeof(value *) * new_count);
    for (int i = st->local_count; i < new_count; ++i) st->locals[i] = NULL;
    st->local_count = new_count;
  }
  st->locals[index] = v;
}

static value *load_local(state *st, int index, const char *name) {
  if (index < 0 || index >= st->local_count || st->locals[index] == NULL) {
    fprintf(stderr, "undefined local %d (%s)\n", index, name ? name : "?");
    exit(1);
  }
  return st->locals[index];
}

#define ZAP_GLOBALS_MAX 4096
static value *g_globals[ZAP_GLOBALS_MAX];
static int g_globals_count = 0;

/* Top-level bindings live in a process-wide table so function frames
   can read and mutate them without owning a copy. */
static void store_global(state *st, int index, value *v) {
  (void)st;
  if (index < 0 || index >= ZAP_GLOBALS_MAX) { fprintf(stderr, "global index out of range\n"); exit(1); }
  if (index + 1 > g_globals_count) g_globals_count = index + 1;
  g_globals[index] = v;
}

static value *load_global(state *st, int index) {
  (void)st;
  if (index < 0 || index >= g_globals_count || g_globals[index] == NULL) {
    fprintf(stderr, "undefined global %d\n", index);
    exit(1);
  }
  return g_globals[index];
}

static int32_t to_int(const char *s) {
  if (s == NULL) return 0;
  size_t len = strlen(s);
  int negative = 0;
  size_t i = 0;
  if (len > 0 && s[0] == '-') { negative = 1; i = 1; }
  if (i >= len) return 0;
  int32_t result = 0;
  while (i < len) {
    char c = s[i++];
    if (c < '0' || c > '9') return 0;
    result = result * 10 + (c - '0');
  }
  return negative ? -result : result;
}

static bool is_int_str(const char *s) {
  if (s == NULL) return false;
  size_t len = strlen(s);
  size_t i = 0;
  if (len == 0) return false;
  if (s[0] == '-') { i = 1; }
  if (i >= len) return false;
  for (; i < len; ++i) {
    if (s[i] < '0' || s[i] > '9') return false;
  }
  return true;
}

static char *from_int(int32_t n) {
  char buf[32];
  snprintf(buf, sizeof(buf), "%d", n);
  return strdup(buf);
}

static bool to_bool(value *v) {
  if (v == NULL) return false;
  if (v->kind == VK_NONE) return false;
  if (v->kind == VK_STR) {
    if (v->str == NULL || v->str[0] == 0) return false;
    if (strcmp(v->str, "false") == 0) return false;
    if (strcmp(v->str, "0") == 0) return false;
    return true;
  }
  return true;
}

static void list_push(value *list, value *item) {
  if (list->count >= list->cap) {
    list->cap = list->cap ? list->cap * 2 : 8;
    list->items = (value **)realloc(list->items, sizeof(value *) * list->cap);
  }
  list->items[list->count++] = item;
}

static value *list_get(value *list, int32_t index) {
  if (list->kind != VK_LIST) { fprintf(stderr, "list_get_non_list\n"); exit(1); }
  if (index < 0 || index >= list->count) { fprintf(stderr, "list index out of range\n"); exit(1); }
  return list->items[index];
}

static void list_set(value *list, int32_t index, value *item) {
  if (list->kind != VK_LIST) { fprintf(stderr, "list_set_non_list\n"); exit(1); }
  if (index < 0 || index >= list->count) { fprintf(stderr, "list index out of range\n"); exit(1); }
  list->items[index] = item;
}

static int map_find(value *map, const char *key) {
  for (int i = 0; i < map->count; ++i) {
    if (map->items[i]->str != NULL && strcmp(map->items[i]->str, key) == 0) return i;
  }
  return -1;
}

static value *map_get(value *map, const char *key) {
  if (map->kind != VK_MAP) { fprintf(stderr, "map_get_non_map\n"); exit(1); }
  int idx = map_find(map, key);
  if (idx < 0) return vstr("");
  return map->vals[idx];
}

static void map_set(value *map, const char *key, value *val) {
  if (map->kind != VK_MAP) { fprintf(stderr, "map_set_non_map\n"); exit(1); }
  int idx = map_find(map, key);
  if (idx >= 0) { map->vals[idx] = val; return; }
  if (map->count >= map->cap) {
    map->cap = map->cap ? map->cap * 2 : 8;
    map->items = (value **)realloc(map->items, sizeof(value *) * map->cap);
    map->vals = (value **)realloc(map->vals, sizeof(value *) * map->cap);
  }
  map->items[map->count] = vstr(key);
  map->vals[map->count] = val;
  map->count++;
}

static value *map_has_key(value *map, const char *key) {
  if (map->kind != VK_MAP) { fprintf(stderr, "map_has_key_non_map\n"); exit(1); }
  return vstr(map_find(map, key) >= 0 ? "true" : "false");
}

static value *map_keys(value *map) {
  value *out = vlist();
  for (int i = 0; i < map->count; ++i) list_push(out, vstr(map->items[i]->str));
  return out;
}

static value *map_values(value *map) {
  value *out = vlist();
  for (int i = 0; i < map->count; ++i) list_push(out, map->vals[i]);
  return out;
}

static value *list_concat_value(value *a, value *b) {
  if (a->kind != VK_LIST || b->kind != VK_LIST) {
    fprintf(stderr, "list_concat_non_list\n");
    exit(1);
  }
  value *out = vlist();
  for (int i = 0; i < a->count; ++i) list_push(out, a->items[i]);
  for (int i = 0; i < b->count; ++i) list_push(out, b->items[i]);
  return out;
}

static value *vconcat(value *a, value *b) {
  if (a->kind == VK_LIST && b->kind == VK_LIST) return list_concat_value(a, b);
  const char *left = (a != NULL && a->str != NULL) ? a->str : "";
  const char *right = (b != NULL && b->str != NULL) ? b->str : "";
  if (a != NULL && a->str == NULL && a->kind != VK_LIST) left = "";
  if (b != NULL && b->str == NULL && b->kind != VK_LIST) right = "";
  size_t nn = strlen(left) + strlen(right) + 1;
  char *buf = (char *)malloc(nn);
  if (buf == NULL) { fprintf(stderr, "out of memory\n"); exit(1); }
  strcpy(buf, left);
  strcat(buf, right);
  value *out = vmake(VK_STR);
  out->str = buf;
  return out;
}

static value *index_get(value *container, value *key) {
  if (container->kind == VK_LIST) return list_get(container, to_int(key->str));
  if (container->kind == VK_MAP) return map_get(container, key->str ? key->str : "");
  fprintf(stderr, "index_get_non_container\n");
  exit(1);
}

static void index_set(value *container, value *key, value *item) {
  if (container->kind == VK_LIST) { list_set(container, to_int(key->str), item); return; }
  if (container->kind == VK_MAP) { map_set(container, key->str ? key->str : "", item); return; }
  fprintf(stderr, "index_set_non_container\n");
  exit(1);
}

static value *contains_value(value *container, value *needle) {
  if (container->kind == VK_LIST) {
    for (int i = 0; i < container->count; ++i) {
      value *item = container->items[i];
      if (item == needle) return vstr("true");
      if (item->str != NULL && needle->str != NULL && strcmp(item->str, needle->str) == 0) return vstr("true");
    }
    return vstr("false");
  }
  if (container->kind == VK_MAP) {
    return vstr(map_find(container, needle->str ? needle->str : "") >= 0 ? "true" : "false");
  }
  fprintf(stderr, "contains_non_container\n");
  exit(1);
}

static bool is_error(value *v) {
  return v != NULL && v->kind == VK_ERROR;
}

static value *list_append_value(value *list, value *item) {
  if (list->kind != VK_LIST) { fprintf(stderr, "push_non_list\n"); exit(1); }
  list_push(list, item);
  return list;
}

static value *list_reverse_value(value *list) {
  if (list->kind != VK_LIST) { fprintf(stderr, "reverse_non_list\n"); exit(1); }
  value *out = vlist();
  for (int i = list->count - 1; i >= 0; --i) list_push(out, list->items[i]);
  return out;
}

static value *list_join_value(value *list, const char *separator) {
  if (list->kind != VK_LIST) { fprintf(stderr, "join_non_list\n"); exit(1); }
  size_t total = 1;
  size_t sep_len = strlen(separator ? separator : "");
  char **parts = (char **)malloc(sizeof(char *) * (list->count > 0 ? list->count : 1));
  for (int i = 0; i < list->count; ++i) {
    parts[i] = fmt(list->items[i]);
    total += strlen(parts[i]) + (i > 0 ? sep_len : 0);
  }
  char *buf = (char *)malloc(total);
  if (buf == NULL) { fprintf(stderr, "out of memory\n"); exit(1); }
  buf[0] = 0;
  for (int i = 0; i < list->count; ++i) {
    if (i > 0) strcat(buf, separator ? separator : "");
    strcat(buf, parts[i]);
    free(parts[i]);
  }
  free(parts);
  value *out = vstr(buf);
  free(buf);
  return out;
}

static value *unwrap_value(value *v) {
  if (v->kind == VK_SOME) return v->items[0];
  if (v->kind == VK_ERROR) { fprintf(stderr, "unwrapped error: %s\n", v->str ? v->str : ""); exit(1); }
  if (v->kind == VK_NONE) { fprintf(stderr, "unwrapped none\n"); exit(1); }
  return v;
}

static value *option_unwrap_or_value(value *v, value *fallback) {
  if (v->kind == VK_SOME) return v->items[0];
  return fallback;
}

static const char *diagnostic_format(const char *code, const char *message) {
  static char buf[512];
  snprintf(buf, sizeof(buf), "%s %s", code ? code : "ZAP-DIAG-000", message ? message : "");
  return buf;
}

static value *kind_of(value *v) {
  if (v == NULL) return vstr("unknown");
  switch (v->kind) {
    case VK_LIST: return vstr("list");
    case VK_MAP: return vstr("map");
    case VK_ERROR: return vstr("error");
    case VK_SOME: return vstr("some");
    case VK_NONE: return vstr("none");
    case VK_TASK: return vstr("task");
    default: break;
  }
  if (is_int_str(v->str)) return vstr("int");
  return vstr("str");
}

#define ZAP_READ_MAX 4194304
static int g_argc = 0;
static char **g_argv = NULL;

static value *builtin_argv(void) {
  value *out = vlist();
  for (int i = 1; i < g_argc; ++i) list_push(out, vstr(g_argv[i]));
  return out;
}

static value *read_file_text(const char *path) {
  FILE *fh = fopen(path, "rb");
  if (fh == NULL) return verr("read_file_failed");
  size_t cap = 4096;
  size_t len = 0;
  char *buf = (char *)malloc(cap);
  if (buf == NULL) { fclose(fh); return verr("out_of_memory"); }
  size_t n;
  while ((n = fread(buf + len, 1, cap - len - 1, fh)) > 0) {
    len += n;
    if (len + 1 >= cap) {
      if (cap >= ZAP_READ_MAX) break;
      cap *= 2;
      buf = (char *)realloc(buf, cap);
      if (buf == NULL) { fclose(fh); return verr("out_of_memory"); }
    }
  }
  fclose(fh);
  buf[len] = 0;
  value *out = vstr(buf);
  free(buf);
  return out;
}

static value *write_file_text(const char *path, const char *content) {
  FILE *fh = fopen(path, "wb");
  if (fh == NULL) return vstr("false");
  if (content != NULL && content[0] != 0) fwrite(content, 1, strlen(content), fh);
  fclose(fh);
  return vstr("true");
}

static value *file_exists(const char *path) {
  FILE *fh = fopen(path, "rb");
  if (fh == NULL) return vstr("false");
  fclose(fh);
  return vstr("true");
}

static value *string_length(value *v) {
  const char *s = (v != NULL && v->str != NULL) ? v->str : "";
  return vstr(from_int((int32_t)strlen(s)));
}

static bool string_equals(value *a, value *b) {
  const char *l = (a != NULL && a->str != NULL) ? a->str : "";
  const char *r = (b != NULL && b->str != NULL) ? b->str : "";
  return strcmp(l, r) == 0;
}

static bool values_equal(value *a, value *b) {
  if (a == b) return true;
  if (a == NULL || b == NULL) return false;
  if (a->kind != b->kind) return false;
  switch (a->kind) {
    case VK_STR: return string_equals(a, b);
    case VK_NONE: return true;
    case VK_ERROR: return string_equals(a, b);
    case VK_SOME: return values_equal(a->items[0], b->items[0]);
    case VK_LIST: {
      if (a->count != b->count) return false;
      for (int i = 0; i < a->count; ++i) {
        if (!values_equal(a->items[i], b->items[i])) return false;
      }
      return true;
    }
    case VK_MAP: {
      if (a->count != b->count) return false;
      for (int i = 0; i < a->count; ++i) {
        int idx = map_find(b, a->items[i]->str);
        if (idx < 0) return false;
        if (!values_equal(a->vals[i], b->vals[idx])) return false;
      }
      return true;
    }
    case VK_TASK: return a->task_id == b->task_id;
  }
  return false;
}

static value *json_serialize(value *v);

static value *json_serialize(value *v) {
  char *s = fmt(v);
  value *out = vstr(s);
  free(s);
  return out;
}


static char *fmt(value *v);

static char *fmt(value *v) {
  if (v == NULL) return strdup("");
  if (v->kind == VK_STR) return strdup(v->str ? v->str : "");
  if (v->kind == VK_NONE) return strdup("none");
  if (v->kind == VK_ERROR) {
    size_t n = strlen(v->str ? v->str : "") + 32;
    char *buf = (char *)malloc(n);
    snprintf(buf, n, "error(\"%s\")", v->str ? v->str : "");
    return buf;
  }
  if (v->kind == VK_SOME) {
    char *inner = fmt(v->items[0]);
    size_t n = strlen(inner) + 16;
    char *buf = (char *)malloc(n);
    snprintf(buf, n, "some(%s)", inner);
    free(inner);
    return buf;
  }
  if (v->kind == VK_TASK) {
    char *buf = (char *)malloc(48);
    snprintf(buf, 48, "<task %d>", v->task_id);
    return buf;
  }
  if (v->kind == VK_LIST) {
    size_t n = 16;
    char **parts = (char **)malloc(sizeof(char *) * (v->count > 0 ? v->count : 1));
    for (int i = 0; i < v->count; ++i) { parts[i] = fmt(v->items[i]); n += strlen(parts[i]) + 2; }
    char *buf = (char *)malloc(n);
    strcpy(buf, "[");
    for (int i = 0; i < v->count; ++i) {
      if (i > 0) strcat(buf, ", ");
      strcat(buf, parts[i]);
      free(parts[i]);
    }
    strcat(buf, "]");
    free(parts);
    return buf;
  }
  if (v->kind == VK_MAP) {
    size_t n = 16;
    char **parts = (char **)malloc(sizeof(char *) * (v->count > 0 ? v->count : 1));
    for (int i = 0; i < v->count; ++i) {
      char *val_s = fmt(v->vals[i]);
      parts[i] = (char *)malloc(strlen(v->items[i]->str) + strlen(val_s) + 4);
      sprintf(parts[i], "%s: %s", v->items[i]->str, val_s);
      free(val_s);
      n += strlen(parts[i]) + 2;
    }
    char *buf = (char *)malloc(n);
    strcpy(buf, "{");
    for (int i = 0; i < v->count; ++i) {
      if (i > 0) strcat(buf, ", ");
      strcat(buf, parts[i]);
      free(parts[i]);
    }
    strcat(buf, "}");
    free(parts);
    return buf;
  }
  return strdup("");
}

static void print_value(state *st, value *v) {
  char *s = fmt(v);
  if (st->output_count >= st->output_cap) {
    st->output_cap = st->output_cap ? st->output_cap * 2 : 16;
    st->output = (char **)realloc(st->output, sizeof(char *) * st->output_cap);
  }
  st->output[st->output_count++] = s;
}

static void call_push(state *st, int return_site, int argc) {
  st->frames = (frame *)realloc(st->frames, sizeof(frame) * (st->frame_count + 1));
  frame *f = &st->frames[st->frame_count++];
  f->return_site = return_site;
  f->saved_locals = st->locals;
  f->saved_local_count = st->local_count;
  f->saved_stack_count = st->stack_count;
  f->argc = argc;
  /* The callee gets a fresh local array so parameter binding never
     reallocates (or clobbers) the caller's locals. */
  st->locals = NULL;
  st->local_count = 0;
}

static int call_pop(state *st) {
  if (st->frame_count == 0) return -1;
  frame *f = &st->frames[--st->frame_count];
  value *ret = NULL;
  int callee_base = f->saved_stack_count - f->argc;
  if (st->stack_count > callee_base) {
    ret = st->stack[st->stack_count - 1];
  }
  st->stack_count = callee_base;
  free(st->locals);
  st->locals = f->saved_locals;
  st->local_count = f->saved_local_count;
  if (ret != NULL) push(st, ret);
  return f->return_site;
}

#define ZAP_MODULES_MAX 64
static value *g_modules[ZAP_MODULES_MAX];
static char *g_module_names[ZAP_MODULES_MAX];
static int g_module_count = 0;
static value *g_exports[ZAP_MODULES_MAX];
static int g_active_module = -1;

static value *module_registry(const char *name) {
  for (int i = 0; i < g_module_count; ++i) {
    if (strcmp(g_module_names[i], name) == 0) { g_active_module = i; return g_modules[i]; }
  }
  if (g_module_count >= ZAP_MODULES_MAX) { fprintf(stderr, "module registry exhausted\n"); exit(1); }
  g_modules[g_module_count] = vmap();
  g_module_names[g_module_count] = strdup(name);
  g_exports[g_module_count] = vmap();
  g_active_module = g_module_count;
  return g_modules[g_module_count++];
}

static void export_value(const char *name, value *v) {
  if (g_module_count == 0) { module_registry("__main__"); g_active_module = 0; }
  map_set(g_exports[g_active_module], name, v);
}

#define ZAP_TASKS_MAX 256
static value *g_task_results[ZAP_TASKS_MAX];
static int g_task_count = 0;

static value *task_run(int id) {
  if (id >= 0 && id < g_task_count && g_task_results[id] != NULL) return g_task_results[id];
  return vstr("");
}

static void free_all(void) {
  for (int i = 0; i < g_heap_count; ++i) {
    value *v = g_heap[i];
    if (v->kind == VK_STR || v->kind == VK_ERROR) free(v->str);
    free(v->items);
    free(v->vals);
    free(v);
  }
  g_heap_count = 0;
  for (int i = 0; i < g_module_count; ++i) free(g_module_names[i]);
  g_module_count = 0;
}

int main(int argc, char **argv) {
  g_argc = argc;
  g_argv = argv;
  state st = {0};
  value *a;
  value *b;
  value *c;
  int32_t ia;
  int32_t ib;
  int target;

  target = 5;
  goto label_5;
label_1: ;
  store_local(&st, 0, pop(&st));
  push(&st, load_global(&st, 0));
  push(&st, load_local(&st, 0, "command"));
  b = pop(&st);
  a = pop(&st);
  push(&st, contains_value(a, b));
  a = pop(&st);
  push(&st, a);
  ia = call_pop(&st);
  if (ia < 0) { goto label_84; }
  if (ia == 8) { goto label_8; }
  if (ia == 21) { goto label_21; }
  if (ia == 32) { goto label_32; }
  if (ia == 68) { goto label_68; }
  if (ia == 76) { goto label_76; }
  if (ia == 80) { goto label_80; }
  fprintf(stderr, "invalid return site\n");
  exit(1);
label_5: ;
  target = 13;
  goto label_13;
label_6: ;
  store_local(&st, 0, pop(&st));
  push(&st, load_local(&st, 0, "command"));
  call_push(&st, 8, 1);
  target = 1;
  goto label_1;
label_8: ;
  a = pop(&st);
  if (!to_bool(a)) goto label_11;
  push(&st, vstr("ok"));
  a = pop(&st);
  push(&st, a);
  ia = call_pop(&st);
  if (ia < 0) { goto label_84; }
  if (ia == 8) { goto label_8; }
  if (ia == 21) { goto label_21; }
  if (ia == 32) { goto label_32; }
  if (ia == 68) { goto label_68; }
  if (ia == 76) { goto label_76; }
  if (ia == 80) { goto label_80; }
  fprintf(stderr, "invalid return site\n");
  exit(1);
label_11: ;
  push(&st, vstr("unsupported"));
  a = pop(&st);
  push(&st, a);
  ia = call_pop(&st);
  if (ia < 0) { goto label_84; }
  if (ia == 8) { goto label_8; }
  if (ia == 21) { goto label_21; }
  if (ia == 32) { goto label_32; }
  if (ia == 68) { goto label_68; }
  if (ia == 76) { goto label_76; }
  if (ia == 80) { goto label_80; }
  fprintf(stderr, "invalid return site\n");
  exit(1);
label_13: ;
  target = 18;
  goto label_18;
label_14: ;
  store_local(&st, 0, pop(&st));
  push(&st, vstr("ZAP-DRIVER-001 unsupported driver command: "));
  push(&st, load_local(&st, 0, "command"));
  b = pop(&st);
  a = pop(&st);
  ia = to_int(a->str);
  ib = to_int(b->str);
  if (is_int_str(a->str) && is_int_str(b->str)) push(&st, vstr(from_int(ia + ib)));
  else push(&st, vconcat(a, b));
  a = pop(&st);
  push(&st, a);
  ia = call_pop(&st);
  if (ia < 0) { goto label_84; }
  if (ia == 8) { goto label_8; }
  if (ia == 21) { goto label_21; }
  if (ia == 32) { goto label_32; }
  if (ia == 68) { goto label_68; }
  if (ia == 76) { goto label_76; }
  if (ia == 80) { goto label_80; }
  fprintf(stderr, "invalid return site\n");
  exit(1);
label_18: ;
  target = 33;
  goto label_33;
label_19: ;
  store_local(&st, 0, pop(&st));
  push(&st, load_local(&st, 0, "command"));
  call_push(&st, 21, 1);
  target = 6;
  goto label_6;
label_21: ;
  store_local(&st, 1, pop(&st));
  push(&st, load_local(&st, 1, "status"));
  push(&st, vstr("ok"));
  b = pop(&st);
  a = pop(&st);
  ia = to_int(a->str);
  ib = to_int(b->str);
  push(&st, vstr(values_equal(a, b) ? "true" : "false"));
  a = pop(&st);
  if (!to_bool(a)) goto label_30;
  push(&st, vstr("command: "));
  push(&st, load_local(&st, 0, "command"));
  b = pop(&st);
  a = pop(&st);
  ia = to_int(a->str);
  ib = to_int(b->str);
  if (is_int_str(a->str) && is_int_str(b->str)) push(&st, vstr(from_int(ia + ib)));
  else push(&st, vconcat(a, b));
  a = pop(&st);
  push(&st, a);
  ia = call_pop(&st);
  if (ia < 0) { goto label_84; }
  if (ia == 8) { goto label_8; }
  if (ia == 21) { goto label_21; }
  if (ia == 32) { goto label_32; }
  if (ia == 68) { goto label_68; }
  if (ia == 76) { goto label_76; }
  if (ia == 80) { goto label_80; }
  fprintf(stderr, "invalid return site\n");
  exit(1);
label_30: ;
  push(&st, load_local(&st, 0, "command"));
  call_push(&st, 32, 1);
  target = 14;
  goto label_14;
label_32: ;
  a = pop(&st);
  push(&st, a);
  ia = call_pop(&st);
  if (ia < 0) { goto label_84; }
  if (ia == 8) { goto label_8; }
  if (ia == 21) { goto label_21; }
  if (ia == 32) { goto label_32; }
  if (ia == 68) { goto label_68; }
  if (ia == 76) { goto label_76; }
  if (ia == 80) { goto label_80; }
  fprintf(stderr, "invalid return site\n");
  exit(1);
label_33: ;
  target = 40;
  goto label_40;
label_34: ;
  push(&st, vstr("usage: zap <command> [source] commands: "));
  push(&st, load_global(&st, 0));
  a = pop(&st);
  if (a->kind != VK_LIST && a->kind != VK_MAP) { fprintf(stderr, "list_len_non_list\n"); exit(1); }
  push(&st, vstr(from_int(a->count)));
  {
    value *bi_args[1];
    for (ia = 1 - 1; ia >= 0; --ia) bi_args[ia] = pop(&st);
    push(&st, vstr(fmt(bi_args[0])));
  }
  b = pop(&st);
  a = pop(&st);
  ia = to_int(a->str);
  ib = to_int(b->str);
  if (is_int_str(a->str) && is_int_str(b->str)) push(&st, vstr(from_int(ia + ib)));
  else push(&st, vconcat(a, b));
  a = pop(&st);
  push(&st, a);
  ia = call_pop(&st);
  if (ia < 0) { goto label_84; }
  if (ia == 8) { goto label_8; }
  if (ia == 21) { goto label_21; }
  if (ia == 32) { goto label_32; }
  if (ia == 68) { goto label_68; }
  if (ia == 76) { goto label_76; }
  if (ia == 80) { goto label_80; }
  fprintf(stderr, "invalid return site\n");
  exit(1);
label_40: ;
  push(&st, vstr("check"));
  store_global(&st, 1, pop(&st));
  push(&st, vstr("build"));
  store_global(&st, 2, pop(&st));
  push(&st, vstr("run"));
  store_global(&st, 3, pop(&st));
  push(&st, vstr("test"));
  store_global(&st, 4, pop(&st));
  push(&st, load_global(&st, 1));
  push(&st, load_global(&st, 2));
  push(&st, load_global(&st, 3));
  push(&st, load_global(&st, 4));
  {
    value *elems[64];
    if (4 > 64) { fprintf(stderr, "make_list too large\n"); exit(1); }
    for (ia = 4 - 1; ia >= 0; --ia) elems[ia] = pop(&st);
    c = vlist();
    for (ia = 0; ia < 4; ++ia) list_push(c, elems[ia]);
    push(&st, c);
  }
  store_global(&st, 0, pop(&st));
  {
    value *bi_args[1];
    for (ia = 0 - 1; ia >= 0; --ia) bi_args[ia] = pop(&st);
    push(&st, builtin_argv());
  }
  store_global(&st, 5, pop(&st));
  {
    value *bi_args[1];
    for (ia = 0 - 1; ia >= 0; --ia) bi_args[ia] = pop(&st);
    push(&st, vstr(from_int((int32_t)(g_argc > 0 ? g_argc - 1 : 0))));
  }
  store_global(&st, 6, pop(&st));
  push(&st, vstr("argc: "));
  push(&st, load_global(&st, 6));
  {
    value *bi_args[1];
    for (ia = 1 - 1; ia >= 0; --ia) bi_args[ia] = pop(&st);
    push(&st, vstr(fmt(bi_args[0])));
  }
  b = pop(&st);
  a = pop(&st);
  ia = to_int(a->str);
  ib = to_int(b->str);
  if (is_int_str(a->str) && is_int_str(b->str)) push(&st, vstr(from_int(ia + ib)));
  else push(&st, vconcat(a, b));
  print_value(&st, pop(&st));
  push(&st, load_global(&st, 6));
  push(&st, vstr(from_int(0)));
  b = pop(&st);
  a = pop(&st);
  ia = to_int(a->str);
  ib = to_int(b->str);
  push(&st, vstr(values_equal(a, b) ? "true" : "false"));
  a = pop(&st);
  if (!to_bool(a)) goto label_70;
  call_push(&st, 68, 0);
  target = 34;
  goto label_34;
label_68: ;
  print_value(&st, pop(&st));
  goto label_83;
label_70: ;
  push(&st, load_global(&st, 5));
  push(&st, vstr(from_int(0)));
  b = pop(&st);
  a = pop(&st);
  push(&st, index_get(a, b));
  store_local(&st, 0, pop(&st));
  push(&st, load_local(&st, 0, "command"));
  call_push(&st, 76, 1);
  target = 19;
  goto label_19;
label_76: ;
  print_value(&st, pop(&st));
  push(&st, vstr("supported: "));
  push(&st, load_local(&st, 0, "command"));
  call_push(&st, 80, 1);
  target = 1;
  goto label_1;
label_80: ;
  {
    value *bi_args[1];
    for (ia = 1 - 1; ia >= 0; --ia) bi_args[ia] = pop(&st);
    push(&st, vstr(fmt(bi_args[0])));
  }
  b = pop(&st);
  a = pop(&st);
  ia = to_int(a->str);
  ib = to_int(b->str);
  if (is_int_str(a->str) && is_int_str(b->str)) push(&st, vstr(from_int(ia + ib)));
  else push(&st, vconcat(a, b));
  print_value(&st, pop(&st));
label_83: ;
  goto label_84;
label_84: ;
  free_all();
  for (ia = 0; ia < st.output_count; ++ia) {
    printf("%s\n", st.output[ia]);
  }
  return 0;
}
