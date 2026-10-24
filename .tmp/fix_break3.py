path = r"D:\zap\scripts\bootstrap\verify_b4_source_to_vm_break_payload_10.sh"
text = open(path).read()
# fix case9
old = "cat > \"$TMPDIR_GATE/case9.zp\" <<'EOF'\nimport \"bootstrap/b4/compiler_driver.zp\"\nimport \"bootstrap/b3/vm.zp\"\n  let outside_break = driver_seed_compile_source(\"break\", \"outside_break.zp\")\nsay outside_break[\"status\"]\nif outside_break[\"error\"] == \"none\":\n    say outside_break[\"bytecode\"][\"error\"]\nelse:\n    say outside_break[\"error\"]"
new = "cat > \"$TMPDIR_GATE/case9.zp\" <<'EOF'\nimport \"bootstrap/b4/compiler_driver.zp\"\nimport \"bootstrap/b3/vm.zp\"\nlet outside_break = driver_seed_compile_source(\"break\", \"outside_break.zp\")\nsay outside_break[\"status\"]\nsay outside_break[\"error\"]"
if old not in text: print("no9")
else:
    text = text.replace(old, new)
    open(path,"w").write(text)
    print("ok9")
# fix case10
old2 = "cat > \"$TMPDIR_GATE/case10.zp\" <<'EOF'\nimport \"bootstrap/b4/compiler_driver.zp\"\nimport \"bootstrap/b3/vm.zp\"\n  let outside_continue = driver_seed_compile_source(\"continue\", \"outside_continue.zp\")\nsay outside_continue[\"status\"]\nif outside_continue[\"error\"] == \"none\":\n    say outside_continue[\"bytecode\"][\"error\"]\nelse:\n    say outside_continue[\"error\"]"
new2 = "cat > \"$TMPDIR_GATE/case10.zp\" <<'EOF'\nimport \"bootstrap/b4/compiler_driver.zp\"\nimport \"bootstrap/b3/vm.zp\"\nlet outside_continue = driver_seed_compile_source(\"continue\", \"outside_continue.zp\")\nsay outside_continue[\"status\"]\nsay outside_continue[\"error\"]"
if old2 not in text: print("no10")
else:
    text = text.replace(old2, new2)
    open(path,"w").write(text)
    print("ok10")
