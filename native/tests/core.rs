use std::process::Command;

fn binary() -> String {
    env!("CARGO_BIN_EXE_zap").to_string()
}

#[test]
fn runs_arithmetic_and_lists() {
    let file = std::env::temp_dir().join("zap_core_test.zp");
    std::fs::write(
        &file,
        "let items = [10, 20, 30]\nsay items[1]\nsay len(items)\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "20\n3\n");
}

#[test]
fn runs_functions_and_returns() {
    let file = std::env::temp_dir().join("zap_functions_test.zp");
    std::fs::write(
        &file,
        "fn add(a, b):\n    return a + b\nlet result = add(7, 8)\nsay result\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "15\n");
}

#[test]
fn enforces_function_parameter_and_return_annotations() {
    let ok_file = std::env::temp_dir().join("zap_typed_function_test.zp");
    std::fs::write(
        &ok_file,
        "fn add(a: number, b: number) -> number:\n    return a + b\nsay add(2, 3)\n",
    )
    .unwrap();
    let ok = Command::new(binary()).arg(&ok_file).output().unwrap();
    let _ = std::fs::remove_file(&ok_file);
    assert!(
        ok.status.success(),
        "{}",
        String::from_utf8_lossy(&ok.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&ok.stdout), "5\n");

    let bad_file = std::env::temp_dir().join("zap_typed_function_error_test.zp");
    std::fs::write(
        &bad_file,
        "fn add(a: number, b: number) -> number:\n    return a + b\nsay add(\"wrong\", 3)\n",
    )
    .unwrap();
    let bad = Command::new(binary()).arg(&bad_file).output().unwrap();
    let _ = std::fs::remove_file(&bad_file);
    assert!(!bad.status.success());
    assert!(String::from_utf8_lossy(&bad.stderr).contains("type mismatch"));
}

#[test]
fn formats_zp_source_in_place() {
    let file = std::env::temp_dir().join("zap_formatter_test.zp");
    std::fs::write(&file, "say 1\t\n").unwrap();
    let output = Command::new(binary())
        .args(["fmt", file.to_str().unwrap()])
        .output()
        .unwrap();
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
    let source = format!(
        "write_text(\"{}\", \"hello\")\nsay read_text(\"{}\")\n",
        path, path
    );
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
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "use \"math\"\nsay triple(4)\n").unwrap();
    std::fs::write(modules.join("math.zp"), "fn triple(x):\n    return x * 3\n").unwrap();
    let check = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(check.status.success());
    let run = Command::new(binary())
        .arg(root.join("main.zp"))
        .output()
        .unwrap();
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
    std::fs::write(
        &file,
        "let ready = true\nlet valid = false\nsay ready and not valid\nsay ready or valid\n",
    )
    .unwrap();
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
    std::fs::write(
        &file,
        "let x = 14\nif x > 10:\n    say \"ok\"\nelse:\n    say \"bad\"\n",
    )
    .unwrap();
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
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "7\n4\n9\nZAP\nzap\ncore\nb\n"
    );
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
fn check_json_reports_structured_type_diagnostics() {
    let root = std::env::temp_dir().join("zap_check_json_type_project");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"diagnostic-demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        root.join("main.zp"),
        "fn bad(value: unknown_type) -> number:\n    return 1\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["check", "--json", root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"ok\":false"));
    assert!(stdout.contains("\"kind\":\"TypeError\""));
    assert!(stdout.contains("unknown type annotation"));
}

#[test]
fn build_command_validates_project() {
    let root = std::env::temp_dir().join("zap_v060_build_project");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"build-demo\"\nversion = \"0.6.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"build ok\"\n").unwrap();
    let output = Command::new(binary())
        .args(["build", root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("built Zap project"));
}

#[test]
fn rejects_division_modulo_by_zero_and_integer_overflow() {
    for (name, source, expected) in [
        ("zap_div_zero_test.zp", "say 1 / 0\n", "division by zero"),
        ("zap_mod_zero_test.zp", "say 1 % 0\n", "division by zero"),
        (
            "zap_overflow_test.zp",
            "say 9223372036854775807 + 1\n",
            "integer overflow",
        ),
    ] {
        let file = std::env::temp_dir().join(name);
        std::fs::write(&file, source).unwrap();
        let output = Command::new(binary()).arg(&file).output().unwrap();
        let _ = std::fs::remove_file(&file);
        assert!(
            !output.status.success(),
            "program unexpectedly succeeded: {name}"
        );
        assert!(String::from_utf8_lossy(&output.stderr).contains(expected));
    }
}

#[test]
fn runs_oop_classes_methods_and_inheritance() {
    let file = std::env::temp_dir().join("zap_oop_core_test.zp");
    std::fs::write(&file, "class User:\n    fn init(self, name):\n        self.name = name\n    fn greet(self):\n        return \"Hello, \" + self.name\nclass Admin extends User:\n    fn role(self):\n        return \"admin\"\nlet user = new(\"User\", \"Tester\")\nlet admin = new(\"Admin\", \"Root\")\nsay user.greet()\nsay admin.greet()\nsay admin.role()\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Hello, Tester\nHello, Root\nadmin\n"
    );
}

#[test]
fn runs_oop_property_assignment() {
    let file = std::env::temp_dir().join("zap_oop_property_test.zp");
    std::fs::write(&file, "class Counter:\n    fn increment(self):\n        self.value = self.value + 1\n        return self.value\nlet counter = new(\"Counter\", {\"value\": 0})\nsay counter.increment()\nsay counter.value\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "1\n1\n");
}

#[test]
fn runs_v070_collection_and_line_helpers() {
    let file = std::env::temp_dir().join("zap_v070_helpers_test.zp");
    let lines = std::env::temp_dir().join("zap_v070_lines.txt");
    let path = lines.to_string_lossy().replace('\\', "\\\\");
    let source = format!("let values = [4, 1, 8, 2]\nsay is_empty(values)\nsay sum(values)\nsay join(sort(values), \",\")\nwrite_lines(\"{}\", [\"one\", \"two\"])\nsay join(read_lines(\"{}\"), \"|\")\n", path, path);
    std::fs::write(&file, source).unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    let _ = std::fs::remove_file(&lines);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "false\n15\n1,2,4,8\none|two\n"
    );
}

#[test]
fn gets_map_default_value() {
    let file = std::env::temp_dir().join("zap_v070_get_test.zp");
    std::fs::write(&file, "let user = {\"name\": \"Zap\"}\nsay get(user, \"name\", \"unknown\")\nsay get(user, \"email\", \"unknown\")\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Zap\nunknown\n");
}

#[test]
fn validates_oop_class_errors_and_parent_constructor() {
    let file = std::env::temp_dir().join("zap_oop_parent_test.zp");
    std::fs::write(&file, "class Base:\n    fn init(self):\n        self.ready = true\nclass Child extends Base:\n    fn init(self):\n        self.child = true\nlet item = new(\"Child\")\nsay item.ready\nsay item.child\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\ntrue\n");

    let missing = std::env::temp_dir().join("zap_oop_missing_class_test.zp");
    std::fs::write(&missing, "let item = new(\"Missing\")\n").unwrap();
    let output = Command::new(binary()).arg(&missing).output().unwrap();
    let _ = std::fs::remove_file(&missing);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown class: Missing"));
}

#[test]
fn rejects_unknown_parent_class() {
    let file = std::env::temp_dir().join("zap_oop_missing_parent_test.zp");
    std::fs::write(
        &file,
        "class Child extends Missing:\n    fn value(self):\n        return 1\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown parent class: Missing"));
}

#[test]
fn runs_oop_override_and_empty_class() {
    let file = std::env::temp_dir().join("zap_oop_override_test.zp");
    std::fs::write(&file, "class Base:\n    fn label(self):\n        return \"base\"\nclass Child extends Base:\n    fn label(self):\n        return \"child\"\nclass Empty:\nlet child = new(\"Child\")\nlet empty = new(\"Empty\")\nsay child.label()\nsay type(empty)\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "child\nobject\n");
}

#[test]
fn caches_explicit_module_execution_and_exports_only_public_symbols() {
    let root = std::env::temp_dir().join("zap_module_cache_test");
    std::fs::create_dir_all(&root).unwrap();
    let module = root.join("counter.zp");
    let main = root.join("main.zp");
    std::fs::write(&module, "say \"loaded\"\nlet secret = 99\nexport let answer = 42\nexport fn value():\n    return answer\n").unwrap();
    std::fs::write(
        &main,
        "import \"counter\"\nimport \"counter\"\nsay value()\nsay answer\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&main).output().unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "loaded\n42\n42\n");
}

#[test]
fn rejects_circular_explicit_imports() {
    let root = std::env::temp_dir().join("zap_module_cycle_test");
    std::fs::create_dir_all(&root).unwrap();
    let main = root.join("main.zp");
    std::fs::write(root.join("a.zp"), "import \"b\"\nexport let a = 1\n").unwrap();
    std::fs::write(root.join("b.zp"), "import \"a\"\nexport let b = 2\n").unwrap();
    std::fs::write(&main, "import \"a\"\n").unwrap();
    let output = Command::new(binary()).arg(&main).output().unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("circular import detected"));
}

#[test]
fn rejects_absolute_module_paths() {
    let file = std::env::temp_dir().join("zap_absolute_import_test.zp");
    let module = std::env::temp_dir().join("zap_absolute_target.zp");
    std::fs::write(&module, "export let value = 1\n").unwrap();
    std::fs::write(&file, format!("import \"{}\"\n", module.display())).unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    let _ = std::fs::remove_file(&module);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("absolute module paths are not allowed")
    );
}

#[test]
fn check_rejects_annotated_variable_mismatch() {
    let root = std::env::temp_dir().join("zap_static_assignment_test");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"static-assignment\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "let count: number = \"wrong\"\n").unwrap();
    let output = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("variable 'count' expects number, got text"));
}

#[test]
fn propagates_result_errors_with_question_operator() {
    let file = std::env::temp_dir().join("zap_result_propagation_test.zp");
    let source = "fn load():\n    return err(\"missing\")\n\nfn wrapper():\n    let value = load()?\n    return ok(value)\n\nlet result = wrapper()\nsay is_err(result)\n";
    std::fs::write(&file, source).unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "true\n");
}

#[test]
fn unwraps_ok_result_with_question_operator() {
    let file = std::env::temp_dir().join("zap_result_success_propagation_test.zp");
    let source = "fn load():\n    return ok(42)\n\nfn wrapper():\n    let value = load()?\n    return ok(value + 1)\n\nlet result = wrapper()\nsay unwrap(result)\n";
    std::fs::write(&file, source).unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "43\n");
}

#[test]
fn rejects_question_operator_for_non_result_values() {
    let file = std::env::temp_dir().join("zap_invalid_result_propagation_test.zp");
    std::fs::write(
        &file,
        "fn wrapper():\n    let value = 42?\n    return value\n\nwrapper()\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("? expects a Result value"));
}

#[test]
fn check_validates_result_and_option_payload_types() {
    let valid_root = std::env::temp_dir().join("zap_result_option_payload_valid");
    std::fs::create_dir_all(&valid_root).unwrap();
    std::fs::write(
        valid_root.join("zap.toml"),
        "[package]\nname = \"payload-valid\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        valid_root.join("main.zp"),
        "let answer: result<number> = ok(42)\nlet name: option<text> = some(\"Zap\")\n",
    )
    .unwrap();
    let valid = Command::new(binary())
        .args(["check", valid_root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&valid_root);
    assert!(valid.status.success());

    let invalid_root = std::env::temp_dir().join("zap_result_option_payload_invalid");
    std::fs::create_dir_all(&invalid_root).unwrap();
    std::fs::write(
        invalid_root.join("zap.toml"),
        "[package]\nname = \"payload-invalid\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        invalid_root.join("main.zp"),
        "let answer: result<number> = ok(\"wrong\")\nlet name: option<number> = some(\"wrong\")\n",
    )
    .unwrap();
    let invalid = Command::new(binary())
        .args(["check", "--json", invalid_root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&invalid_root);
    assert!(!invalid.status.success());
    let stdout = String::from_utf8_lossy(&invalid.stdout);
    assert!(stdout.contains("\"kind\":\"TypeError\""));
    assert!(stdout.contains("result<number>"));
}

#[test]
fn rejects_unused_tokens_after_expression() {
    let file = std::env::temp_dir().join("zap_expression_end_test.zp");
    std::fs::write(&file, "say 1 2\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unexpected token after expression"));
}

#[test]
fn reports_malformed_source_with_line_and_column() {
    let file = std::env::temp_dir().join("zap_malformed_span_test.zp");
    std::fs::write(&file, "say 1\nsay @\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("1:1"),
        "expected source location in stderr: {stderr}"
    );
    assert!(
        stderr.contains("unexpected character"),
        "expected lexer diagnostic: {stderr}"
    );
}

#[test]
fn test_command_accepts_filter_fail_fast_and_json_options() {
    let root = std::env::temp_dir().join("zap_test_options_project");
    let tests = root.join("tests");
    std::fs::create_dir_all(&tests).unwrap();
    std::fs::write(tests.join("first_test.zp"), "assert(1 == 1, \"first\")\n").unwrap();
    std::fs::write(tests.join("second_test.zp"), "assert(2 == 2, \"second\")\n").unwrap();
    let output = Command::new(binary())
        .args([
            "test",
            "--filter",
            "first",
            "--fail-fast",
            "--json",
            root.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("\"passed\":1"));
    assert!(stdout.contains("first_test.zp"));
    assert!(!stdout.contains("second_test.zp"));
}

#[test]
fn test_command_rejects_unknown_options_with_usage_exit_code() {
    let output = Command::new(binary())
        .args(["test", "--unknown"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown test option"));
}

#[test]
fn rejects_mixed_indentation() {
    let file = std::env::temp_dir().join("zap_mixed_indentation_test.zp");
    std::fs::write(&file, "if true:\n    say 1\n\tsay 2\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("mixed indentation"));
}

#[test]
fn accepts_blank_lines_and_comments_inside_nested_blocks() {
    let file = std::env::temp_dir().join("zap_nested_comments_test.zp");
    std::fs::write(
        &file,
        "fn greet():\n    # comment before body\n\n    if true:\n        # nested comment\n        return \"ok\"\n\nsay greet()\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "ok\n");
}

#[test]
fn reports_unicode_runtime_values_without_corruption() {
    let file = std::env::temp_dir().join("zap_unicode_test.zp");
    std::fs::write(&file, "say \"မင်္ဂလာပါ\"\nsay len(\"မြန်မာ\")\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("မင်္ဂလာပါ"));
}

#[test]
fn rejects_non_four_space_indentation() {
    let file = std::env::temp_dir().join("zap_bad_indentation_test.zp");
    std::fs::write(&file, "if true:\n  say 1\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid indentation"));
}

#[test]
fn reports_assertion_expected_and_actual_values() {
    let file = std::env::temp_dir().join("zap_assertion_values_test.zp");
    std::fs::write(&file, "assert(1 == 2)\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("expected") || stderr.contains("assertion"));
}

#[test]
fn handles_windows_style_paths_as_data() {
    let file = std::env::temp_dir().join("zap_windows_path_test.zp");
    std::fs::write(
        &file,
        "say path_join(\"C:\\\\Users\", \"Zap\", \"main.zp\")\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("C:"));
}

#[test]
fn rejects_permission_failures_without_panic() {
    let file = std::env::temp_dir().join("zap_permission_failure_test.zp");
    std::fs::write(
        &file,
        "say read_text(\"/definitely/missing/zap-file.txt\")\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("read_text failed"));
}

#[test]
fn rejects_module_parent_directory_traversal() {
    let root = std::env::temp_dir().join("zap_module_traversal_test");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("modules")).unwrap();
    let main = root.join("main.zp");
    std::fs::write(&main, "import ../outside\n").unwrap();
    let output = Command::new(binary()).arg(&main).output().unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("may not traverse parent directories"));
}
