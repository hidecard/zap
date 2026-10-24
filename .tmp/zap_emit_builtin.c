#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <string.h>

#define ZAP_LIST_MAX 32
#define ZAP_LIST_ITEMS_MAX 64
#define ZAP_MAP_MAX 32
#define ZAP_MAP_ENTRIES_MAX 64
#define ZAP_STRING_MAX 64
static long zap_stack[256];
static int zap_sp = 0;
static char zap_strings[ZAP_STRING_MAX][256];
static int zap_string_count = 0;
static long zap_lists[ZAP_LIST_MAX][ZAP_LIST_ITEMS_MAX];
static int zap_list_lengths[ZAP_LIST_MAX];
static int zap_list_count = 0;
static long zap_map_keys[ZAP_MAP_MAX][ZAP_MAP_ENTRIES_MAX];
static long zap_map_values[ZAP_MAP_MAX][ZAP_MAP_ENTRIES_MAX];
static int zap_map_lengths[ZAP_MAP_MAX];
static int zap_map_count = 0;
static long zap_pop(void) { return zap_stack[--zap_sp]; }
static void zap_push(long value) { zap_stack[zap_sp++] = value; }
static long zap_string_new(const char *text) { if (zap_string_count >= ZAP_STRING_MAX) { fprintf(stderr, "string limit exceeded\n"); exit(1); } strncpy(zap_strings[zap_string_count], text, 255); zap_strings[zap_string_count][255] = 0; zap_string_count += 1; return -zap_string_count; }
static int zap_string_index(long handle) { if (handle >= 0 || handle < -zap_string_count) { fprintf(stderr, "string handle invalid\n"); exit(1); } return (int)(-handle - 1); }
static const char *zap_string_get(long handle) { return zap_strings[zap_string_index(handle)]; }
static int zap_is_string(long value) { return value < 0 && -value <= zap_string_count; }
static int zap_value_equal(long left, long right) { if (zap_is_string(left) && zap_is_string(right)) return strcmp(zap_string_get(left), zap_string_get(right)) == 0; return left == right; }
static long zap_list_new(void) { if (zap_list_count >= ZAP_LIST_MAX) { fprintf(stderr, "list limit exceeded\n"); exit(1); } zap_list_count += 1; zap_list_lengths[zap_list_count - 1] = 0; return zap_list_count; }
static int zap_list_index(long handle) { if (handle <= 0 || handle > zap_list_count) { fprintf(stderr, "list handle invalid\n"); exit(1); } return (int)(handle - 1); }
static long zap_list_len(long handle) { return zap_list_lengths[zap_list_index(handle)]; }
static long zap_list_get(long handle, long index) { int list = zap_list_index(handle); if (index < 0 || index >= zap_list_lengths[list]) { fprintf(stderr, "list index out of range\n"); exit(1); } return zap_lists[list][index]; }
static void zap_list_set(long handle, long index, long value) { int list = zap_list_index(handle); if (index < 0 || index >= ZAP_LIST_ITEMS_MAX) { fprintf(stderr, "list index out of range\n"); exit(1); } if (index >= zap_list_lengths[list]) zap_list_lengths[list] = (int)(index + 1); zap_lists[list][index] = value; }
static void zap_list_append(long handle, long value) { int list = zap_list_index(handle); if (zap_list_lengths[list] >= ZAP_LIST_ITEMS_MAX) { fprintf(stderr, "list too large\n"); exit(1); } zap_lists[list][zap_list_lengths[list]++] = value; }
static long zap_map_new(void) { if (zap_map_count >= ZAP_MAP_MAX) { fprintf(stderr, "map limit exceeded\n"); exit(1); } zap_map_count += 1; zap_map_lengths[zap_map_count - 1] = 0; return 1000 + zap_map_count; }
static int zap_is_map(long handle) { return handle >= 1001 && handle <= 1000 + zap_map_count; }
static int zap_map_index(long handle) { if (!zap_is_map(handle)) { fprintf(stderr, "map handle invalid\n"); exit(1); } return (int)(handle - 1001); }
static int zap_map_find(long handle, long key) { int map = zap_map_index(handle); int i; for (i = 0; i < zap_map_lengths[map]; ++i) if (zap_value_equal(zap_map_keys[map][i], key)) return i; return -1; }
static void zap_map_set(long handle, long key, long value) { int map = zap_map_index(handle); int found = zap_map_find(handle, key); if (found >= 0) { zap_map_values[map][found] = value; return; } if (zap_map_lengths[map] >= ZAP_MAP_ENTRIES_MAX) { fprintf(stderr, "map too large\n"); exit(1); } found = zap_map_lengths[map]++; zap_map_keys[map][found] = key; zap_map_values[map][found] = value; }
static long zap_map_get(long handle, long key) { int map = zap_map_index(handle); int found = zap_map_find(handle, key); return found < 0 ? 0 : zap_map_values[map][found]; }
static long zap_map_has_key(long handle, long key) { return zap_map_find(handle, key) >= 0; }
static long zap_map_keys_list(long handle) { int map = zap_map_index(handle); int i; long list = zap_list_new(); for (i = 0; i < zap_map_lengths[map]; ++i) zap_list_append(list, zap_map_keys[map][i]); return list; }
static long zap_map_values_list(long handle) { int map = zap_map_index(handle); int i; long list = zap_list_new(); for (i = 0; i < zap_map_lengths[map]; ++i) zap_list_append(list, zap_map_values[map][i]); return list; }

static long zap_string_concat(long left, long right) { char buffer[256]; if (!zap_is_string(left) || !zap_is_string(right)) { fprintf(stderr, "string concat requires strings\n"); exit(1); } snprintf(buffer, sizeof(buffer), "%s%s", zap_string_get(left), zap_string_get(right)); return zap_string_new(buffer); }
static long zap_add(long left, long right) { if (zap_is_string(left) || zap_is_string(right)) return zap_string_concat(left, right); return left + right; }
static long zap_less(long left, long right) { if (zap_is_string(left) && zap_is_string(right)) return strcmp(zap_string_get(left), zap_string_get(right)) < 0; if (zap_is_string(left) || zap_is_string(right)) { fprintf(stderr, "comparison requires matching values\n"); exit(1); } return left < right; }
static long zap_less_equal(long left, long right) { if (zap_is_string(left) && zap_is_string(right)) return strcmp(zap_string_get(left), zap_string_get(right)) <= 0; if (zap_is_string(left) || zap_is_string(right)) { fprintf(stderr, "comparison requires matching values\n"); exit(1); } return left <= right; }
static long zap_greater(long left, long right) { if (zap_is_string(left) && zap_is_string(right)) return strcmp(zap_string_get(left), zap_string_get(right)) > 0; if (zap_is_string(left) || zap_is_string(right)) { fprintf(stderr, "comparison requires matching values\n"); exit(1); } return left > right; }
static long zap_greater_equal(long left, long right) { if (zap_is_string(left) && zap_is_string(right)) return strcmp(zap_string_get(left), zap_string_get(right)) >= 0; if (zap_is_string(left) || zap_is_string(right)) { fprintf(stderr, "comparison requires matching values\n"); exit(1); } return left >= right; }
static long zap_index(long handle, long index) { if (zap_is_map(handle)) return zap_map_get(handle, index); return zap_list_get(handle, index); }
static long zap_contains(long handle, long needle) { if (zap_is_map(handle)) return zap_map_has_key(handle, needle); { int list = zap_list_index(handle); int index; for (index = 0; index < zap_list_lengths[list]; ++index) if (zap_value_equal(zap_lists[list][index], needle)) return 1; return 0; } }
static long zap_options[64]; static int zap_option_some[64]; static int zap_option_count = 0;
static int zap_is_option(long handle) { return handle >= 2001 && handle <= 2000 + zap_option_count; }
static int zap_option_index(long handle) { if (!zap_is_option(handle)) { fprintf(stderr, "option handle invalid\n"); exit(1); } return (int)(handle - 2001); }
static long zap_option_new(long value, int some) { if (zap_option_count >= 64) { fprintf(stderr, "option limit exceeded\n"); exit(1); } zap_options[zap_option_count] = value; zap_option_some[zap_option_count] = some; zap_option_count += 1; return 2000 + zap_option_count; }
static long zap_errors[64]; static int zap_error_count = 0;
static int zap_is_error(long handle) { return handle >= 3001 && handle <= 3000 + zap_error_count; }
static int zap_error_index(long handle) { if (!zap_is_error(handle)) { fprintf(stderr, "error handle invalid\n"); exit(1); } return (int)(handle - 3001); }
static long zap_error_new(long message) { if (zap_error_count >= 64) { fprintf(stderr, "error limit exceeded\n"); exit(1); } zap_errors[zap_error_count] = message; zap_error_count += 1; return 3000 + zap_error_count; }
static int zap_frame_depth = 0; static int zap_return_labels[32]; static char zap_local_names[32][64][64]; static long zap_local_values[32][64]; static int zap_local_counts[32];
static void zap_store(const char *name, long value) { int i; for (i = 0; i < zap_local_counts[zap_frame_depth]; ++i) if (strcmp(zap_local_names[zap_frame_depth][i], name) == 0) { zap_local_values[zap_frame_depth][i] = value; return; } if (zap_local_counts[zap_frame_depth] >= 64) { fprintf(stderr, "local limit exceeded\n"); exit(1); } strncpy(zap_local_names[zap_frame_depth][zap_local_counts[zap_frame_depth]], name, 63); zap_local_names[zap_frame_depth][zap_local_counts[zap_frame_depth]][63] = 0; zap_local_values[zap_frame_depth][zap_local_counts[zap_frame_depth]++] = value; }
static long zap_load(const char *name) { int i; for (i = zap_local_counts[zap_frame_depth] - 1; i >= 0; --i) if (strcmp(zap_local_names[zap_frame_depth][i], name) == 0) return zap_local_values[zap_frame_depth][i]; fprintf(stderr, "unknown local: %s\n", name); exit(1); }
static long zap_closure_values[64][16]; static int zap_closure_lengths[64]; static int zap_closure_count = 0;
static long zap_closure_new(int count, long *values) { int i; if (zap_closure_count >= 64 || count > 16) { fprintf(stderr, "closure limit exceeded\n"); exit(1); } for (i = 0; i < count; ++i) zap_closure_values[zap_closure_count][i] = values[i]; zap_closure_lengths[zap_closure_count] = count; zap_closure_count += 1; return 4000 + zap_closure_count; }
static int zap_closure_index(long handle) { if (handle < 4001 || handle > 4000 + zap_closure_count) { fprintf(stderr, "closure handle invalid\n"); exit(1); } return (int)(handle - 4001); }
static long zap_closure_get(long handle, int index) { int closure = zap_closure_index(handle); if (index < 0 || index >= zap_closure_lengths[closure]) { fprintf(stderr, "closure capture invalid\n"); exit(1); } return zap_closure_values[closure][index]; }
static long zap_tasks[ZAP_TASKS_MAX]; static int zap_task_count = 0;
static int zap_is_task(long handle) { return handle >= 5001 && handle <= 5000 + zap_task_count; }
static int zap_task_index(long handle) { if (!zap_is_task(handle)) { fprintf(stderr, "task handle invalid\n"); exit(1); } return (int)(handle - 5001); }
static long zap_task_new(long value) { if (zap_task_count >= ZAP_TASKS_MAX) { fprintf(stderr, "task limit exceeded\n"); exit(1); } zap_tasks[zap_task_count] = value; zap_task_count += 1; return 5000 + zap_task_count; }
static long zap_task_run(long handle) { int index = zap_task_index(handle); return zap_tasks[index]; }
static long zap_modules[64]; static int zap_module_count = 0;
static int zap_is_module(long handle) { return handle >= 6001 && handle <= 6000 + zap_module_count; }
static int zap_module_index(long handle) { if (!zap_is_module(handle)) { fprintf(stderr, "module handle invalid\n"); exit(1); } return (int)(handle - 6001); }
static long zap_module_new(long value) { if (zap_module_count >= 64) { fprintf(stderr, "module limit exceeded\n"); exit(1); } zap_modules[zap_module_count] = value; zap_module_count += 1; return 6000 + zap_module_count; }
static long zap_module_get(long handle) { int index = zap_module_index(handle); return zap_modules[index]; }
static int g_argc = 0;
static long g_argv_list = 0;
static void init_argv(int argc, char **argv) {
  g_argc = argc;
  g_argv_list = zap_list_new();
  int i;
  for (i = 0; i < argc; ++i) {
    zap_list_append(g_argv_list, zap_string_new(argv[i]));
  }
}
int main(int argc, char **argv) {
  init_argv(argc, argv);
  zap_label_0:;
  zap_push(g_argc - 1);
  zap_label_1:;
  { long value = zap_pop(); if (zap_is_string(value)) printf("%s\n", zap_string_get(value)); else printf("%ld\n", value); }
  zap_label_2:;
  zap_push(g_argv_list);
  zap_label_3:;
  { long handle = zap_pop(); if (zap_is_list(handle)) { int list = zap_list_index(handle); zap_push(zap_list_lengths[list]); } else if (zap_is_map(handle)) { int map = zap_map_index(handle); zap_push(zap_map_lengths[map]); } else { zap_push(0); } }
  zap_label_4:;
  return 0;
  zap_label_5:;
  return 0;
}