#!/usr/bin/env python3
import json
import glob

def normalize_source_name(source_name):
    if not isinstance(source_name, str):
        return source_name
    return source_name.replace("\\", "/")

def normalize_paths(obj):
    if isinstance(obj, dict):
        for key, value in list(obj.items()):
            if key == "source_name":
                obj[key] = normalize_source_name(value)
            else:
                normalize_paths(value)
    elif isinstance(obj, list):
        for item in obj:
            normalize_paths(item)

count = 0
for path in glob.glob("bootstrap/fixtures/**/*.typed-ir.json", recursive=True):
    with open(path, "r", encoding="utf-8") as f:
        data = json.load(f)
    old_sn = data.get("source_name", "")
    normalize_paths(data)
    new_sn = data.get("source_name", "")
    if old_sn != new_sn:
        with open(path, "w", encoding="utf-8") as f:
            json.dump(data, f, ensure_ascii=False, separators=(",", ":"))
            f.write("\n")
        count += 1
        print(f"Fixed: {path} ({old_sn!r} -> {new_sn!r})")

print(f"Total fixed: {count}")
