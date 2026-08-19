use std::process::Command;

fn binary() -> String {
    env!("CARGO_BIN_EXE_zap").to_string()
}

#[test]
fn runs_arithmetic_and_lists() {
    let file = std::env::temp_dir().join("zap_core_test.zp");
    std::fs::write(&file, "let items = [10, 20, 30]\nsay items[1]\nsay len(items)\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "20\n3\n");
}

#[test]
fn runs_functions_and_returns() {
    let file = std::env::temp_dir().join("zap_functions_test.zp");
    std::fs::write(&file, "fn add(a, b):\n    return a + b\nlet result = add(7, 8)\nsay result\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "15\n");
}

#[test]
fn formats_zp_source_in_place() {
    let file = std::env::temp_dir().join("zap_formatter_test.zp");
    std::fs::write(&file, "say 1\t\n").unwrap();
    let output = Command::new(binary()).args(["fmt", file.to_str().unwrap()]).output().unwrap();
    assert!(output.status.success());
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "say 1\n");
    let _ = std::fs::remove_file(&file);
}

#[test]
fn captures_nested_closure_variables() {
    let file = std::env::temp_dir().join("zap_closure_test.zp");
    std::fs::write(&file, "fn make_adder(x):\n    fn add(y):\n        return x + y\n    return add(5)\nsay make_adder(7)\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "12\n");
}

#[test]
fn encodes_and_decodes_json() {
    let file = std::env::temp_dir().join("zap_json_test.zp");
    std::fs::write(&file, "let data = {\"name\": \"Zap\", \"items\": [1, 2]}\nsay json(data)\nsay from_json(\"{\\\"ok\\\": true}\")[\"ok\"]\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"name\":\"Zap\""));
    assert!(stdout.ends_with("true\n"));
}

#[test]
fn reads_and_writes_text_files() {
    let file = std::env::temp_dir().join("zap_file_test.txt");
    let program = std::env::temp_dir().join("zap_file_builtins_test.zp");
    let path = file.to_string_lossy().replace('\\', "\\\\");
    let source = format!("write_text(\"{}\", \"hello\")\nsay read_text(\"{}\")\n", path, path);
    std::fs::write(&program, source).unwrap();
    let output = Command::new(binary()).arg(&program).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hello\n");
}

#[test]
fn validates_zap_manifest_and_module_directory() {
    let root = std::env::temp_dir().join("zap_manifest_project");
    let modules = root.join("modules");
    std::fs::create_dir_all(&modules).unwrap();
    std::fs::write(root.join("zap.toml"), "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n").unwrap();
    std::fs::write(root.join("main.zp"), "use \"math\"\nsay triple(4)\n").unwrap();
    std::fs::write(modules.join("math.zp"), "fn triple(x):\n    return x * 3\n").unwrap();
    let check = Command::new(binary()).args(["check", root.to_str().unwrap()]).output().unwrap();
    assert!(check.status.success());
    let run = Command::new(binary()).arg(root.join("main.zp")).output().unwrap();
    assert!(run.status.success());
    assert_eq!(String::from_utf8_lossy(&run.stdout), "12\n");
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn imports_local_module() {
    let main = std::env::temp_dir().join("zap_import_test.zp");
    let module = main.with_file_name("zap_test_module.zp");
    std::fs::write(&module, "fn triple(x):\n    return x * 3\n").unwrap();
    std::fs::write(&main, "use \"zap_test_module.zp\"\nsay triple(4)\n").unwrap();
    let output = Command::new(binary()).arg(&main).output().unwrap();
    let _ = std::fs::remove_file(&module);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "12\n");
}

#[test]
fn runs_standard_builtins() {
    let file = std::env::temp_dir().join("zap_builtins_test.zp");
    std::fs::write(&file, "let items = range(3)\nsay items[2]\nsay str(42)\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n42\n");
}

#[test]
fn runs_boolean_logic() {
    let file = std::env::temp_dir().join("zap_boolean_test.zp");
    std::fs::write(&file, "let ready = true\nlet valid = false\nsay ready and not valid\nsay ready or valid\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\ntrue\n");
}

#[test]
fn runs_break_and_continue() {
    let file = std::env::temp_dir().join("zap_loop_control_test.zp");
    std::fs::write(&file, "let values = range(5)\nfor item in values:\n    if item == 1:\n        continue\n    if item == 3:\n        break\n    say item\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "0\n2\n");
}

#[test]
fn runs_conditionals() {
    let file = std::env::temp_dir().join("zap_condition_test.zp");
    std::fs::write(&file, "let x = 14\nif x > 10:\n    say \"ok\"\nelse:\n    say \"bad\"\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "ok\n");
}

#[test]
fn runs_collection_and_runtime_helpers() {
    let file = std::env::temp_dir().join("zap_collection_helpers_test.zp");
    std::fs::write(&file, "let user = {\"name\": \"Zap\", \"version\": 3}\nsay type(user)\nsay contains(user, \"name\")\nsay join(keys(user), \",\")\nsay contains(\"native runtime\", \"runtime\")\nassert(type(user) == \"map\", \"map type expected\")\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("map\n"));
    assert!(stdout.contains("true\n"));
    assert!(stdout.contains("version,name\n") || stdout.contains("name,version\n"));
}

#[test]
fn reports_assertion_failures() {
    let file = std::env::temp_dir().join("zap_assert_failure_test.zp");
    std::fs::write(&file, "assert(false, \"expected failure\")\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("expected failure"));
}

#[test]
fn runs_text_and_numeric_helpers() {
    let file = std::env::temp_dir().join("zap_text_helpers_test.zp");
    std::fs::write(&file, "say abs(-7)\nsay min(4, 9)\nsay max(4, 9)\nsay upper(\"zap\")\nsay lower(\"ZAP\")\nsay trim(\"  core  \")\nsay split(\"a,b,c\", \",\")[1]\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "7\n4\n9\nZAP\nzap\ncore\nb\n");
}

#[test]
fn runs_v060_standard_library_helpers() {
    let file = std::env::temp_dir().join("zap_v060_stdlib_test.zp");
    std::fs::write(&file, "say basename(path_join(\"tmp\", \"zap\", \"main.zp\"))\nsay dirname(\"tmp/zap/main.zp\")\nsay pow(2, 4)\nsay sqrt(16)\nsay has_env(\"PATH\")\nsay type(now())\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("main.zp\n"));
    assert!(stdout.contains("tmp/zap\n") || stdout.contains("tmp\\\\zap\n"));
    assert!(stdout.contains("16\n"));
    assert!(stdout.contains("true\n"));
    assert!(stdout.contains("number\n"));
}

#[test]
fn build_command_validates_project() {
    let root = std::env::temp_dir().join("zap_v060_build_project");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("zap.toml"), "[package]\nname = \"build-demo\"\nversion = \"0.6.0\"\nmain = \"main.zp\"\n").unwrap();
    std::fs::write(root.join("main.zp"), "say \"build ok\"\n").unwrap();
    let output = Command::new(binary()).args(["build", root.to_str().unwrap()]).output().unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("built Zap project"));
}
