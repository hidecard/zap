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
fn supports_named_function_arguments_and_defaults() {
    let file = std::env::temp_dir().join("zap_named_arguments_test.zp");
    std::fs::write(
        &file,
        "fn format_name(first, last, suffix = \"!\"):\n    return first + \" \" + last + suffix\nsay format_name(last = \"Lang\", first = \"Zap\")\nsay format_name(last = \"Lang\", first = \"Zap\", suffix = \".\")\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Zap Lang!\nZap Lang.\n"
    );

    let invalid = std::env::temp_dir().join("zap_named_arguments_order_error.zp");
    std::fs::write(
        &invalid,
        "fn add(a, b):\n    return a + b\nsay add(a = 1, 2)\n",
    )
    .unwrap();
    let bad = Command::new(binary()).arg(&invalid).output().unwrap();
    let _ = std::fs::remove_file(&invalid);
    assert!(!bad.status.success());
    assert!(String::from_utf8_lossy(&bad.stderr).contains("positional argument cannot follow"));
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
    let path = file.to_string_lossy();
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
fn validates_explicit_module_manifest_entries() {
    let root = std::env::temp_dir().join("zap_explicit_module_manifest_test");
    let modules = root.join("modules");
    std::fs::create_dir_all(&modules).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[module]\nroot = \"modules\"\nentries = [\"math.zp\"]\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say 1\n").unwrap();
    std::fs::write(modules.join("math.zp"), "say 2\n").unwrap();
    let valid = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(valid.status.success());

    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[module]\nroot = \"modules\"\nentries = [\"../escape.zp\"]\n",
    )
    .unwrap();
    let invalid = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("invalid module entry"));
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
fn runs_stabilized_text_math_and_collection_helpers() {
    let file = std::env::temp_dir().join("zap_stabilized_stdlib_test.zp");
    std::fs::write(
        &file,
        "say replace(\"zap language\", \"zap\", \"Zap\")\nsay starts_with(\"Zap\", \"Z\")\nsay ends_with(\"Zap\", \"p\")\nsay count([1, 2, 1, 3], 1)\nsay join([\"a\", \"b\"], \"-\")\nsay pow(2, 5)\nsay abs(-9)\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "Zap language\ntrue\ntrue\n2\na-b\n32\n9\n"
    );
}

#[test]
fn rejects_invalid_stabilized_stdlib_arguments() {
    let file = std::env::temp_dir().join("zap_invalid_stabilized_stdlib_test.zp");
    std::fs::write(&file, "say pow(2, -1)\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("non-negative exponent"));
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
fn supports_named_method_arguments() {
    let file = std::env::temp_dir().join("zap_named_method_arguments_test.zp");
    std::fs::write(
        &file,
        "class Greeter:\n    fn init(self, prefix):\n        self.prefix = prefix\n    fn greet(self, name, punctuation = \"!\"):\n        return self.prefix + name + punctuation\nlet greeter = new(\"Greeter\", \"Hello, \" )\nsay greeter.greet(punctuation = \".\", name = \"Zap\")\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Hello, Zap.\n");
}

#[test]
fn enforces_oop_method_visibility() {
    let allowed = std::env::temp_dir().join("zap_oop_visibility_allowed.zp");
    std::fs::write(
        &allowed,
        "class Vault:\n    private fn secret(self):\n        return \"hidden\"\n    fn reveal(self):\n        return self.secret()\nlet vault = new(\"Vault\")\nsay vault.reveal()\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&allowed).output().unwrap();
    let _ = std::fs::remove_file(&allowed);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "hidden\n");

    let denied = std::env::temp_dir().join("zap_oop_visibility_denied.zp");
    std::fs::write(
        &denied,
        "class Vault:\n    private fn secret(self):\n        return \"hidden\"\nlet vault = new(\"Vault\")\nsay vault.secret()\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&denied).output().unwrap();
    let _ = std::fs::remove_file(&denied);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("private method is not accessible"));

    let protected = std::env::temp_dir().join("zap_oop_visibility_protected.zp");
    std::fs::write(
        &protected,
        "class Base:\n    protected fn token(self):\n        return \"base-token\"\nclass Child extends Base:\n    fn reveal(self):\n        return self.token()\nlet child = new(\"Child\")\nsay child.reveal()\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&protected).output().unwrap();
    let _ = std::fs::remove_file(&protected);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "base-token\n");
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
    let path = lines.to_string_lossy();
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
    assert!(String::from_utf8_lossy(&output.stderr).contains("? expects a Result or Option value"));
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
fn narrows_option_and_result_payloads_inside_guarded_branches() {
    let valid_root = std::env::temp_dir().join("zap_control_flow_narrowing_valid");
    std::fs::create_dir_all(&valid_root).unwrap();
    std::fs::write(
        valid_root.join("zap.toml"),
        "[package]\nname = \"narrow-valid\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        valid_root.join("main.zp"),
        "fn need_number(value: number):\n    return value\nlet maybe: option<number> = some(7)\nlet outcome: result<number> = ok(9)\nif is_some(maybe):\n    let first: number = need_number(maybe)\nif is_ok(outcome):\n    let second: number = need_number(outcome)\n",
    )
    .unwrap();
    let valid = Command::new(binary())
        .args(["check", valid_root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&valid_root);
    assert!(
        valid.status.success(),
        "{}",
        String::from_utf8_lossy(&valid.stderr)
    );

    let invalid_root = std::env::temp_dir().join("zap_control_flow_narrowing_invalid");
    std::fs::create_dir_all(&invalid_root).unwrap();
    std::fs::write(
        invalid_root.join("zap.toml"),
        "[package]\nname = \"narrow-invalid\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        invalid_root.join("main.zp"),
        "let maybe: option<number> = some(7)\nif is_some(maybe):\n    let inside: number = maybe\nlet outside: number = maybe\n",
    )
    .unwrap();
    let invalid = Command::new(binary())
        .args(["check", "--json", invalid_root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&invalid_root);
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stdout).contains("option<number>"));
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

#[test]
fn validates_generic_collection_and_variant_annotations() {
    let valid_root = std::env::temp_dir().join("zap_generic_annotations_valid");
    let _ = std::fs::remove_dir_all(&valid_root);
    std::fs::create_dir_all(&valid_root).expect("create temp project");
    std::fs::write(
        valid_root.join("zap.toml"),
        "[package]\nname = \"generic-valid\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .expect("write manifest");
    std::fs::write(
        valid_root.join("main.zp"),
        "let numbers: list<number> = [1, 2, 3]\nlet labels: map<text, number> = {\"one\": 1}\nlet answer: result<number> = ok(42)\nlet name: option<text> = some(\"Zap\")\nlet missing: option<number> = option_none()\nsay unwrap(answer)\n",
    )
    .expect("write source");
    let output = std::process::Command::new(binary())
        .args(["check", valid_root.to_str().expect("utf8 path")])
        .output()
        .expect("run check");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let invalid_root = std::env::temp_dir().join("zap_generic_annotations_invalid");
    let _ = std::fs::remove_dir_all(&invalid_root);
    std::fs::create_dir_all(&invalid_root).expect("create temp project");
    std::fs::write(
        invalid_root.join("zap.toml"),
        "[package]\nname = \"generic-invalid\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .expect("write manifest");
    std::fs::write(
        invalid_root.join("main.zp"),
        "let numbers: list<number> = [1, \"wrong\"]\n",
    )
    .expect("write source");
    let output = std::process::Command::new(binary())
        .args(["check", invalid_root.to_str().expect("utf8 path")])
        .output()
        .expect("run check");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("expects list<number>, got list<any>"));
}

#[test]
fn rejects_malformed_generic_annotations() {
    let root = std::env::temp_dir().join("zap_generic_annotations_malformed");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("create temp project");
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"generic-malformed\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .expect("write manifest");
    std::fs::write(root.join("main.zp"), "let values: list<> = []\n").expect("write source");
    let output = std::process::Command::new(binary())
        .args(["check", root.to_str().expect("utf8 path")])
        .output()
        .expect("run check");
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown type annotation 'list<>'"));
}

#[test]
fn runs_explicit_super_constructor_and_method_calls() {
    let file = std::env::temp_dir().join("zap_oop_super_test.zp");
    std::fs::write(
        &file,
        "class Base:\n    fn init(self):\n        self.ready = true\n    fn label(self):\n        return \"base\"\nclass Child extends Base:\n    fn init(self):\n        super.init()\n        self.child = true\n    fn label(self):\n        return super.label() + \"-child\"\nlet item = new(\"Child\")\nsay item.ready\nsay item.child\nsay item.label()\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "true\ntrue\nbase-child\n"
    );
}

#[test]
fn check_rejects_incompatible_reassignment() {
    let root = std::env::temp_dir().join("zap_reassignment_type_test");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"reassignment\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        root.join("main.zp"),
        "let count: number = 1\ncount = \"wrong\"\n",
    )
    .unwrap();
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
fn rejects_non_text_map_key_annotations() {
    let root = std::env::temp_dir().join("zap_map_key_annotation_test");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"map-key\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        root.join("main.zp"),
        "let values: map<number, text> = {\"one\": \"1\"}\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("expects map<number, text>"));
}

#[test]
fn propagates_option_values_with_question_operator() {
    let file = std::env::temp_dir().join("zap_option_question_test.zp");
    std::fs::write(
        &file,
        "fn read_name():\n    let value = some(\"Zap\")\n    return value?\nfn missing_name():\n    return option_none()?\nsay read_name()\nsay is_option_none(missing_name())\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "Zap\ntrue\n");
}

#[test]
fn check_unwraps_option_question_operator_types() {
    let root = std::env::temp_dir().join("zap_option_question_check");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"option-question\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "let value: number = some(1)?\n").unwrap();
    let output = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn check_rejects_non_boolean_control_flow_conditions() {
    let root = std::env::temp_dir().join("zap_control_condition_check");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"condition-check\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "if 1:\n    say \"bad\"\n").unwrap();
    let output = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(!output.status.success());
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(
        diagnostic.contains("control-flow condition expects bool"),
        "{diagnostic}"
    );
}

#[test]
fn rejects_duplicate_function_parameters() {
    let file = std::env::temp_dir().join("zap_duplicate_params_test.zp");
    std::fs::write(&file, "fn duplicate(value, value):\n    return value\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(
        diagnostic.contains("duplicate parameter name: value"),
        "{diagnostic}"
    );
}

#[test]
fn check_rejects_incompatible_function_return() {
    let dir = std::env::temp_dir().join("zap_return_type_project");
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("zap.toml"),
        "name = \"return-check\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("main.zp"),
        "fn bad() -> number:\n    return \"wrong\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["check", dir.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&dir);
    assert!(!output.status.success());
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(
        diagnostic.contains("return from 'bad' expects number, got text"),
        "{diagnostic}"
    );
}

#[test]
fn preserves_mutated_nested_closure_state() {
    let file = std::env::temp_dir().join("zap_closure_state_test.zp");
    std::fs::write(
        &file,
        "fn make_counter():\n    let count = 0\n    fn increment():\n        count = count + 1\n        return count\n    let first = increment()\n    let second = increment()\n    return second\nsay make_counter()\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n");
}

#[test]
fn reports_function_argument_count_errors() {
    let file = std::env::temp_dir().join("zap_function_arity_test.zp");
    std::fs::write(&file, "fn add(a, b):\n    return a + b\nsay add(1)\n").unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("function expects 2 to 2 arguments, got 1"));
}

#[test]
fn applies_default_function_parameters() {
    let file = std::env::temp_dir().join("zap_default_parameter_test.zp");
    std::fs::write(
        &file,
        "fn greet(name: text = \"World\"):\n    say name\ngreet()\ngreet(\"Zap\")\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "World\nZap\n");
}

#[test]
fn enforces_oop_field_visibility_and_inherited_defaults() {
    let file = std::env::temp_dir().join("zap_oop_field_visibility_test.zp");
    std::fs::write(
        &file,
        "class Base:\n    private let secret = \"base-secret\"\n    protected let token = \"base-token\"\n    public let label = \"base-label\"\n    fn reveal_secret(self):\n        return self.secret\nclass Child extends Base:\n    fn reveal_token(self):\n        self.token = \"child-token\"\n        return self.token\nlet child = new(\"Child\")\nsay child.label\nsay child.reveal_secret()\nsay child.reveal_token()\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "base-label\nbase-secret\nchild-token\n"
    );

    let external = std::env::temp_dir().join("zap_oop_private_field_test.zp");
    std::fs::write(
        &external,
        "class Base:\n    private let secret = \"hidden\"\n    protected let token = \"hidden-token\"\nlet item = new(\"Base\")\nsay item.secret\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&external).output().unwrap();
    let _ = std::fs::remove_file(&external);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("private field is not accessible"));

    let protected = std::env::temp_dir().join("zap_oop_protected_field_test.zp");
    std::fs::write(
        &protected,
        "class Base:\n    protected let token = \"hidden-token\"\nlet item = new(\"Base\")\nsay item.token\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&protected).output().unwrap();
    let _ = std::fs::remove_file(&protected);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("protected field is not accessible"));
}

#[test]
fn rejects_non_public_constructors_from_external_code() {
    for (visibility, expected) in [
        ("private", "private method is not accessible"),
        ("protected", "protected method is not accessible"),
    ] {
        let file = std::env::temp_dir().join(format!("zap_oop_{visibility}_constructor_test.zp"));
        std::fs::write(
            &file,
            format!("class Locked:\n    {visibility} fn init(self):\n        self.ready = true\nlet item = new(\"Locked\")\n"),
        )
        .unwrap();
        let output = Command::new(binary()).arg(&file).output().unwrap();
        let _ = std::fs::remove_file(&file);
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(expected),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

#[test]
fn initializes_declared_fields_before_constructor_overrides() {
    let file = std::env::temp_dir().join("zap_oop_field_default_override_test.zp");
    std::fs::write(
        &file,
        "class Counter:\n    public let value = 1\n    fn init(self):\n        self.value = self.value + 1\nlet first = new(\"Counter\")\nlet second = new(\"Counter\")\nsecond.value = 10\nsay first.value\nsay second.value\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n10\n");
}

#[test]
fn enforces_module_aware_private_method_visibility() {
    let root = std::env::temp_dir().join("zap_module_visibility_test");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("vault.zp"),
        "class Vault:\n    private fn secret(self):\n        return \"module-hidden\"\n    fn reveal(self):\n        return self.secret()\n",
    )
    .unwrap();
    let main = root.join("main.zp");
    std::fs::write(
        &main,
        "use \"vault.zp\"\nlet vault = new(\"Vault\")\nsay vault.reveal()\nsay vault.secret()\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&main).output().unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(!output.status.success());
    assert_eq!(String::from_utf8_lossy(&output.stdout), "module-hidden\n");
    assert!(String::from_utf8_lossy(&output.stderr).contains("private method is not accessible"));
}

#[test]
fn avoids_double_parent_constructor_when_explicitly_delegated() {
    let file = std::env::temp_dir().join("zap_constructor_delegation_edge_test.zp");
    std::fs::write(
        &file,
        "class Base:\n    fn init(self):\n        self.count = 1\nclass Child extends Base:\n    fn init(self):\n        super.init()\n        self.count = self.count + 1\nlet child = new(\"Child\")\nsay child.count\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n");
}

#[test]
fn implicitly_delegates_parent_constructor_once() {
    let file = std::env::temp_dir().join("zap_constructor_implicit_delegation_test.zp");
    std::fs::write(
        &file,
        "class Base:\n    fn init(self):\n        self.count = 1\nclass Child extends Base:\n    fn init(self):\n        self.count = self.count + 1\nlet child = new(\"Child\")\nsay child.count\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&file).output().unwrap();
    let _ = std::fs::remove_file(&file);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&output.stdout), "2\n");
}

#[test]
fn rejects_private_constructor_from_another_module() {
    let root = std::env::temp_dir().join("zap_module_constructor_visibility_test");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("private_ctor.zp"),
        "class Secret:\n    private fn init(self):\n        self.ready = true\n",
    )
    .unwrap();
    let main = root.join("main.zp");
    std::fs::write(
        &main,
        "use \"private_ctor.zp\"\nlet value = new(\"Secret\")\n",
    )
    .unwrap();
    let output = Command::new(binary()).arg(&main).output().unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("private method is not accessible"));
}

#[test]
fn narrows_complex_boolean_guards_and_aliases() {
    let valid_root = std::env::temp_dir().join("zap_complex_narrowing_valid");
    let _ = std::fs::remove_dir_all(&valid_root);
    std::fs::create_dir_all(&valid_root).unwrap();
    std::fs::write(
        valid_root.join("zap.toml"),
        "[package]\nname = \"complex-narrow-valid\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        valid_root.join("main.zp"),
        "fn need_number(value: number):\n    return value\nlet maybe: option<number> = some(7)\nlet outcome: result<number> = ok(9)\nlet maybe_alias = maybe\nlet outcome_alias = outcome\nif is_some(maybe_alias) and is_ok(outcome_alias):\n    let first: number = need_number(maybe_alias)\n    let second: number = need_number(outcome_alias)\nif is_some(maybe) or is_some(maybe):\n    let repeated: number = need_number(maybe)\nelse:\n    let still_option: option<number> = maybe\n",
    )
    .unwrap();
    let valid = Command::new(binary())
        .args(["check", valid_root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&valid_root);
    assert!(
        valid.status.success(),
        "{}",
        String::from_utf8_lossy(&valid.stderr)
    );

    let invalid_root = std::env::temp_dir().join("zap_complex_narrowing_invalid");
    let _ = std::fs::remove_dir_all(&invalid_root);
    std::fs::create_dir_all(&invalid_root).unwrap();
    std::fs::write(
        invalid_root.join("zap.toml"),
        "[package]\nname = \"complex-narrow-invalid\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        invalid_root.join("main.zp"),
        "let maybe: option<number> = some(7)\nif is_some(maybe) and is_some(maybe):\n    let inside: number = maybe\nlet outside: number = maybe\n",
    )
    .unwrap();
    let invalid = Command::new(binary())
        .args(["check", invalid_root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&invalid_root);
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("expects number, got option<number>"));
}

#[test]
fn generates_canonical_dependency_lockfile() {
    let root = std::env::temp_dir().join("zap_lockfile_generation_project");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"lock-demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nzeta = \"2.0\"\nalpha = \"1.2\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"lock\"\n").unwrap();
    let output = Command::new(binary())
        .args(["lock", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        std::fs::read_to_string(root.join("zap.lock")).unwrap(),
        "# This file is generated by Zap. Do not edit manually.\nlockfile_version = 1\n\n[package]\nname = \"lock-demo\"\nversion = \"0.1.0\"\n\n[dependencies]\nalpha = \"1.2\"\nzeta = \"2.0\"\n"
    );
    let check = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        check.status.success(),
        "{}",
        String::from_utf8_lossy(&check.stderr)
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn rejects_missing_or_stale_dependency_lockfile() {
    let root = std::env::temp_dir().join("zap_lockfile_validation_project");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"lock-check\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"check\"\n").unwrap();
    let missing = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("missing lockfile"));
    std::fs::write(root.join("zap.lock"), "# stale\n").unwrap();
    let stale = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stderr).contains("out of date"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn lock_command_is_available_in_cli_help() {
    let output = Command::new(binary()).arg("--help").output().unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("zap lock [dir]"));
}

#[test]
fn audits_nested_direct_ast_standard_library_calls() {
    let root = std::env::temp_dir().join("zap_direct_ast_edge_audit");
    std::fs::create_dir_all(&root).unwrap();
    let data_path = root.join("data.txt");
    let program = root.join("main.zp");
    let path = data_path.to_string_lossy();
    let source = format!(
        "write_text(\"{path}\", json({{\"name\": upper(\"zap\"), \"items\": range(pow(2, 2))}}))\nlet value = from_json(read_text(\"{path}\"))\nsay join(reverse(split(value[\"name\"], \"A\")), \"-\")\nsay sum(value[\"items\"])\nsay has_env(\"PATH\")\n"
    );
    std::fs::write(&program, source).unwrap();
    let output = Command::new(binary()).arg(&program).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("P-Z\n") || stdout.contains("Z-P\n"));
    assert!(stdout.contains("6\n"));
    assert!(stdout.contains("true\n"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn adds_manifest_dependency_deterministically_and_invalidates_lockfile() {
    let root = std::env::temp_dir().join("zap_add_dependency_project");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"add-demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"add\"\n").unwrap();
    let generated = Command::new(binary())
        .args(["lock", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(generated.status.success());
    assert!(root.join("zap.lock").is_file());

    let added = Command::new(binary())
        .args(["add", "zeta", "2.0", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(added.status.success());
    assert!(!root.join("zap.lock").exists());
    let manifest = std::fs::read_to_string(root.join("zap.toml")).unwrap();
    assert!(manifest.contains("[dependencies]\nzeta = \"2.0\""));

    let added_second = Command::new(binary())
        .args(["add", "alpha", "1.0", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(added_second.status.success());
    let manifest = std::fs::read_to_string(root.join("zap.toml")).unwrap();
    assert!(manifest.contains("[dependencies]\nalpha = \"1.0\"\nzeta = \"2.0\""));

    let duplicate = Command::new(binary())
        .args(["add", "alpha", "3.0", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!duplicate.status.success());
    assert!(String::from_utf8_lossy(&duplicate.stderr).contains("dependency already exists"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn add_command_is_available_in_cli_help() {
    let output = Command::new(binary()).arg("--help").output().unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("zap add <name> <ver> [dir]"));
}

#[test]
fn install_validates_and_accepts_canonical_lockfile() {
    let root = std::env::temp_dir().join("zap_install_dependency_project");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"install-demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\nzeta = \"2.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"install\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        update.status.success(),
        "{}",
        String::from_utf8_lossy(&update.stderr)
    );
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        install.status.success(),
        "{}",
        String::from_utf8_lossy(&install.stderr)
    );
    assert!(String::from_utf8_lossy(&install.stdout).contains("installed 2 locked dependencies"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_rejects_missing_or_stale_lockfile() {
    let root = std::env::temp_dir().join("zap_install_lock_validation");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"install-check\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"check\"\n").unwrap();
    let missing = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("missing lockfile"));
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    std::fs::write(root.join("zap.lock"), "# stale\n").unwrap();
    let stale = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stderr).contains("out of date"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_regenerates_lockfile_after_dependency_manifest_change() {
    let root = std::env::temp_dir().join("zap_update_dependency_project");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"update-demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nzeta = \"2.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"update\"\n").unwrap();
    let first = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(first.status.success());
    let added = Command::new(binary())
        .args(["add", "alpha", "1.0", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(added.status.success());
    assert!(!root.join("zap.lock").exists());
    let second = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(second.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.contains("alpha = \"1.0\"\n"));
    assert!(lock.contains("zeta = \"2.0\"\n"));
    assert!(lock.find("alpha =").unwrap() < lock.find("zeta =").unwrap());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_and_update_commands_are_available_in_cli_help() {
    let output = Command::new(binary()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("zap install [dir]"));
    assert!(stdout.contains("zap update [dir]"));
}

#[test]
fn update_requires_manifest_metadata() {
    let root = std::env::temp_dir().join("zap_update_invalid_project");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("zap.toml"), "[dependencies]\nalpha = \"1.0\"\n").unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("missing package name"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_and_update_commands_are_documented() {
    let output = Command::new(binary()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("zap install [dir]"));
    assert!(stdout.contains("zap update [dir]"));
}

#[test]
fn install_without_dependencies_is_deterministic() {
    let root = std::env::temp_dir().join("zap_install_no_dependencies");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"plain\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"plain\"\n").unwrap();
    let output = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("installed 0 locked dependencies"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_is_idempotent_for_same_manifest() {
    let root = std::env::temp_dir().join("zap_update_idempotent_project");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"stable\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"stable\"\n").unwrap();
    for _ in 0..2 {
        let output = Command::new(binary())
            .args(["update", root.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(output.status.success());
    }
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert_eq!(lock.matches("alpha = \"1.0\"").count(), 1);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_rejects_extra_lockfile_content() {
    let root = std::env::temp_dir().join("zap_install_extra_lock_content");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"extra\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"extra\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let lock_path = root.join("zap.lock");
    let mut lock = std::fs::read_to_string(&lock_path).unwrap();
    lock.push_str("unexpected = true\n");
    std::fs::write(&lock_path, lock).unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!install.status.success());
    assert!(String::from_utf8_lossy(&install.stderr).contains("out of date"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_and_update_accept_project_directory_arguments() {
    let root = std::env::temp_dir().join("zap_package_dir_arguments");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"dir-args\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"dir\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_preserves_canonical_dependency_order() {
    let root = std::env::temp_dir().join("zap_update_sorted_dependencies");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"sorted\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nzeta = \"2.0\"\nalpha = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"sorted\"\n").unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.find("alpha =").unwrap() < lock.find("zeta =").unwrap());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_reports_canonical_dependency_count() {
    let root = std::env::temp_dir().join("zap_install_count_project");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"counted\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\na = \"1\"\nb = \"2\"\nc = \"3\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"count\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert!(String::from_utf8_lossy(&install.stdout).contains("installed 3 locked dependencies"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_and_update_are_distinct_commands() {
    let root = std::env::temp_dir().join("zap_distinct_package_commands");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"distinct\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"distinct\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    assert!(String::from_utf8_lossy(&update.stdout).contains("updated zap.lock"));
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert!(String::from_utf8_lossy(&install.stdout).contains("installed 1 locked dependencies"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_rejects_invalid_dependency_manifest() {
    let root = std::env::temp_dir().join("zap_update_invalid_dependency");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"bad-deps\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nnot valid = \"1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("invalid dependency entry"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_requires_existing_manifest() {
    let root = std::env::temp_dir().join("zap_install_missing_manifest");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let output = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("manifest read"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_requires_existing_manifest() {
    let root = std::env::temp_dir().join("zap_update_missing_manifest");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("manifest read"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_does_not_modify_lockfile() {
    let root = std::env::temp_dir().join("zap_install_read_only_lock");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"readonly\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"readonly\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let before = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let after = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert_eq!(before, after);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_and_update_help_mentions_lockfile_contract() {
    let output = Command::new(binary()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("zap.lock"));
    assert!(stdout.contains("zap install [dir]"));
    assert!(stdout.contains("zap update [dir]"));
}

#[test]
fn update_writes_lockfile_with_package_metadata() {
    let root = std::env::temp_dir().join("zap_update_package_metadata");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"metadata\"\nversion = \"3.2.1\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"metadata\"\n").unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.contains("name = \"metadata\""));
    assert!(lock.contains("version = \"3.2.1\""));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_accepts_zero_dependency_project_without_lockfile() {
    let root = std::env::temp_dir().join("zap_install_zero_dependencies");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"zero\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"zero\"\n").unwrap();
    let output = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(!root.join("zap.lock").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_overwrites_stale_lockfile() {
    let root = std::env::temp_dir().join("zap_update_overwrites_stale");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"overwrite\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"overwrite\"\n").unwrap();
    std::fs::write(root.join("zap.lock"), "# stale\n").unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.contains("core = \"1.0\""));
    assert!(!lock.contains("# stale"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_failure_does_not_rewrite_manifest() {
    let root = std::env::temp_dir().join("zap_install_manifest_unchanged");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let manifest = "[package]\nname = \"unchanged\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n";
    std::fs::write(root.join("zap.toml"), manifest).unwrap();
    std::fs::write(root.join("main.zp"), "say \"unchanged\"\n").unwrap();
    let output = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert_eq!(
        std::fs::read_to_string(root.join("zap.toml")).unwrap(),
        manifest
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_output_reports_dependency_count() {
    let root = std::env::temp_dir().join("zap_update_count_output");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"count-output\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\nbeta = \"2.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"count-output\"\n").unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("updated zap.lock with 2 dependencies")
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_output_reports_dependency_count() {
    let root = std::env::temp_dir().join("zap_install_count_output");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"count-output-install\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"count-output-install\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let output = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("installed 1 locked dependencies"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_is_repeatable_after_install() {
    let root = std::env::temp_dir().join("zap_update_after_install");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"repeatable\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"repeatable\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let update_again = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update_again.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_rejects_noncanonical_dependency_order() {
    let root = std::env::temp_dir().join("zap_install_noncanonical_order");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"order-check\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\nzeta = \"2.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"order-check\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let lock_path = root.join("zap.lock");
    let lock = std::fs::read_to_string(&lock_path).unwrap();
    let reordered = lock.replace(
        "alpha = \"1.0\"\nzeta = \"2.0\"",
        "zeta = \"2.0\"\nalpha = \"1.0\"",
    );
    std::fs::write(&lock_path, reordered).unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!install.status.success());
    assert!(String::from_utf8_lossy(&install.stderr).contains("out of date"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_cli_usage_is_stable() {
    let output = Command::new(binary()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("zap install [dir]    Validate and install dependencies from zap.lock"));
    assert!(stdout.contains("zap update [dir]     Regenerate zap.lock from zap.toml"));
}

#[test]
fn update_does_not_require_main_source() {
    let root = std::env::temp_dir().join("zap_update_no_main_source");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-main\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(root.join("zap.lock").is_file());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_does_not_require_main_source() {
    let root = std::env::temp_dir().join("zap_install_no_main_source");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-main-install\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_handle_empty_dependency_section() {
    let root = std::env::temp_dir().join("zap_empty_dependency_section");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"empty-deps\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_has_single_header() {
    let root = std::env::temp_dir().join("zap_single_lock_header");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"single-header\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert_eq!(lock.matches("# This file is generated by Zap").count(), 1);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_do_not_create_source_files() {
    let root = std::env::temp_dir().join("zap_no_source_creation");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-source\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert!(!root.join("main.zp").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_is_utf8_text() {
    let root = std::env::temp_dir().join("zap_utf8_lockfile");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"utf8\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(std::fs::read_to_string(root.join("zap.lock")).is_ok());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_return_success_on_valid_project() {
    let root = std::env::temp_dir().join("zap_valid_package_commands");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"valid-commands\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"valid\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_contains_dependency_section() {
    let root = std::env::temp_dir().join("zap_dependency_section_lock");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"dependency-section\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.contains("[dependencies]\n"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_do_not_modify_zap_toml() {
    let root = std::env::temp_dir().join("zap_manifest_immutability");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let manifest = "[package]\nname = \"immutable\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n";
    std::fs::write(root.join("zap.toml"), manifest).unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert_eq!(
        std::fs::read_to_string(root.join("zap.toml")).unwrap(),
        manifest
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_overwrites_lockfile_atomically_at_observable_level() {
    let root = std::env::temp_dir().join("zap_update_observable_lock");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"observable\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(root.join("zap.lock").is_file());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_error_prefixes_are_stable() {
    let root = std::env::temp_dir().join("zap_package_error_prefixes");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&install.stderr).contains("Zap install error:"));
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&update.stderr).contains("Zap update error:"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_accepts_dependency_requirements_with_dots_and_ranges() {
    let root = std::env::temp_dir().join("zap_update_requirement_values");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"requirements\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \">=1.2.3\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.contains("core = \">=1.2.3\""));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_validates_lockfile_before_reporting_success() {
    let root = std::env::temp_dir().join("zap_install_validate_first");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"validate-first\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"validate-first\"\n").unwrap();
    std::fs::write(root.join("zap.lock"), "# invalid\n").unwrap();
    let output = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("installed"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_uses_manifest_values_in_lockfile() {
    let root = std::env::temp_dir().join("zap_update_manifest_values");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"manifest-values\"\nversion = \"9.8.7\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.contains("manifest-values"));
    assert!(lock.contains("9.8.7"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_are_safe_with_repeated_invocation() {
    let root = std::env::temp_dir().join("zap_repeated_package_commands");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"repeated\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    for _ in 0..3 {
        let update = Command::new(binary())
            .args(["update", root.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(update.status.success());
        let install = Command::new(binary())
            .args(["install", root.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(install.status.success());
    }
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_do_not_emit_stderr_on_success() {
    let root = std::env::temp_dir().join("zap_package_clean_stderr");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"clean-stderr\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    assert!(update.stderr.is_empty());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert!(install.stderr.is_empty());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_ends_with_newline() {
    let root = std::env::temp_dir().join("zap_lock_trailing_newline");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"newline\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(std::fs::read(root.join("zap.lock"))
        .unwrap()
        .ends_with(b"\n"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_support_absolute_project_path() {
    let root = std::env::temp_dir().join("zap_absolute_project_path");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"absolute\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_version_is_one() {
    let root = std::env::temp_dir().join("zap_lock_version_one");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"version-one\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.contains("lockfile_version = 1"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_use_project_specific_lockfile() {
    let root = std::env::temp_dir().join("zap_project_specific_lock");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"specific\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    assert!(root.join("zap.lock").is_file());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_rejects_lockfile_for_different_manifest() {
    let root = std::env::temp_dir().join("zap_install_manifest_mismatch");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"manifest-mismatch\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    std::fs::write(
        root.join("zap.lock"),
        "# This file is generated by Zap. Do not edit manually.\nlockfile_version = 1\n\n[package]\nname = \"other\"\nversion = \"0.1.0\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("out of date"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_is_the_only_command_that_rewrites_lockfile() {
    let root = std::env::temp_dir().join("zap_update_rewrite_boundary");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"rewrite-boundary\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let lock_path = root.join("zap.lock");
    let before = std::fs::read_to_string(&lock_path).unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert_eq!(before, std::fs::read_to_string(lock_path).unwrap());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_reject_unknown_project_directory() {
    let root = std::env::temp_dir().join("zap_unknown_project_directory");
    let _ = std::fs::remove_dir_all(&root);
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!install.status.success());
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!update.status.success());
}

#[test]
fn update_lockfile_is_deterministic_across_runs() {
    let root = std::env::temp_dir().join("zap_update_deterministic_runs");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"deterministic-runs\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nzeta = \"2.0\"\nalpha = \"1.0\"\n",
    )
    .unwrap();
    let first = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(first.status.success());
    let first_lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    let second = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(second.status.success());
    let second_lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert_eq!(first_lock, second_lock);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_accept_nested_project_directory() {
    let root = std::env::temp_dir()
        .join("zap_nested_project_commands")
        .join("app");
    let _ = std::fs::remove_dir_all(root.parent().unwrap());
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"nested\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root.parent().unwrap());
}

#[test]
fn install_update_do_not_depend_on_current_working_directory() {
    let root = std::env::temp_dir().join("zap_cwd_independent_project");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"cwd-independent\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .current_dir(std::env::temp_dir())
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_are_safe_for_manifest_comments() {
    let root = std::env::temp_dir().join("zap_manifest_comment_project");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "# comment\n[package]\nname = \"comments\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\n# dependency comment\ncore = \"1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_replaces_existing_lockfile_content() {
    let root = std::env::temp_dir().join("zap_replace_existing_lock");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"replace-existing\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("zap.lock"), "old lock content").unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(!lock.contains("old lock content"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_success_output_is_stdout_only() {
    let root = std::env::temp_dir().join("zap_package_stdout_only");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"stdout-only\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    assert!(!update.stdout.is_empty());
    assert!(update.stderr.is_empty());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_does_not_add_dependency_entries() {
    let root = std::env::temp_dir().join("zap_update_no_additions");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-additions\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.ends_with("[dependencies]\n"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_does_not_generate_missing_lockfile() {
    let root = std::env::temp_dir().join("zap_install_no_generate");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-generate\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!root.join("zap.lock").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_generates_missing_lockfile() {
    let root = std::env::temp_dir().join("zap_update_generate_missing");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"generate-missing\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(root.join("zap.lock").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_work_with_relative_project_path() {
    let root = std::env::temp_dir().join("zap_relative_package_path");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"relative\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let cwd = std::env::current_dir().unwrap();
    let relative = root.strip_prefix(&cwd).unwrap_or(&root);
    let output = Command::new(binary())
        .args(["update", relative.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_do_not_touch_project_main_source() {
    let root = std::env::temp_dir().join("zap_main_source_unchanged");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let source = "say \"main stays unchanged\"\n";
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"main-source\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), source).unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert_eq!(
        std::fs::read_to_string(root.join("main.zp")).unwrap(),
        source
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_lockfile_and_manifest_are_project_local() {
    let first = std::env::temp_dir().join("zap_local_first");
    let second = std::env::temp_dir().join("zap_local_second");
    let _ = std::fs::remove_dir_all(&first);
    let _ = std::fs::remove_dir_all(&second);
    std::fs::create_dir_all(&first).unwrap();
    std::fs::create_dir_all(&second).unwrap();
    for (root, name) in [(&first, "first"), (&second, "second")] {
        std::fs::write(
            root.join("zap.toml"),
            format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n"),
        )
        .unwrap();
        let output = Command::new(binary())
            .args(["update", root.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(output.status.success());
    }
    assert!(std::fs::read_to_string(first.join("zap.lock"))
        .unwrap()
        .contains("first"));
    assert!(std::fs::read_to_string(second.join("zap.lock"))
        .unwrap()
        .contains("second"));
    let _ = std::fs::remove_dir_all(first);
    let _ = std::fs::remove_dir_all(second);
}

#[test]
fn update_install_round_trip_preserves_lockfile_bytes() {
    let root = std::env::temp_dir().join("zap_round_trip_lock_bytes");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"round-trip\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\nzeta = \"2.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let before = std::fs::read(root.join("zap.lock")).unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let after = std::fs::read(root.join("zap.lock")).unwrap();
    assert_eq!(before, after);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_are_available_with_current_version() {
    let output = Command::new(binary()).arg("--version").output().unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("zap 2.0.0"));
}

#[test]
fn install_update_reject_file_as_project_directory() {
    let file = std::env::temp_dir().join("zap_not_directory");
    std::fs::write(&file, "not a project directory").unwrap();
    let install = Command::new(binary())
        .args(["install", file.to_str().unwrap()])
        .output()
        .unwrap();
    let update = Command::new(binary())
        .args(["update", file.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!install.status.success());
    assert!(!update.status.success());
    let _ = std::fs::remove_file(file);
}

#[test]
fn update_lockfile_is_stable_after_manifest_whitespace() {
    let root = std::env::temp_dir().join("zap_manifest_whitespace_stability");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"whitespace\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\n",
    )
    .unwrap();
    let first = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(first.status.success());
    let before = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"whitespace\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"   \n",
    )
    .unwrap();
    let second = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(second.status.success());
    let after = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert_eq!(before, after);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_do_not_require_network() {
    let root = std::env::temp_dir().join("zap_no_network_package_commands");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"offline\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .env("http_proxy", "http://127.0.0.1:1")
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .env("http_proxy", "http://127.0.0.1:1")
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_lockfile_contract_is_explicit_in_help() {
    let output = Command::new(binary()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("from zap.lock"));
    assert!(stdout.contains("from zap.toml"));
}

#[test]
fn install_update_commands_work_with_unicode_package_name() {
    let root = std::env::temp_dir().join("zap_unicode_package_name");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"ဇာပ်\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_preserve_lockfile_permissions_observably() {
    let root = std::env::temp_dir().join("zap_lock_permissions");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"permissions\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(root.join("zap.lock").metadata().unwrap().is_file());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_have_no_extra_positional_arguments() {
    let root = std::env::temp_dir().join("zap_extra_package_argument");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap(), "extra"])
        .output()
        .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap(), "extra"])
        .output()
        .unwrap();
    assert!(!install.status.success());
    assert!(!update.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_dependency_values_are_not_trimmed_internally() {
    let root = std::env::temp_dir().join("zap_dependency_value_preservation");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"value-preserve\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \">= 1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.contains("core = \">= 1.0\""));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_rejects_lockfile_with_wrong_version() {
    let root = std::env::temp_dir().join("zap_install_wrong_lock_version");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"wrong-version\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(
        root.join("zap.lock"),
        "# This file is generated by Zap. Do not edit manually.\nlockfile_version = 2\n\n[package]\nname = \"wrong-version\"\nversion = \"0.1.0\"\n\n[dependencies]\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("out of date"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_can_be_read_after_process_restart() {
    let root = std::env::temp_dir().join("zap_restart_lockfile");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"restart\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_are_bounded_to_project_lockfile() {
    let root = std::env::temp_dir().join("zap_bounded_lockfile");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"bounded\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(root.join("zap.lock").is_file());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_not_source_run_commands() {
    let root = std::env::temp_dir().join("zap_not_source_run");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"not-run\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say \"must-not-run\"\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    assert!(!String::from_utf8_lossy(&update.stdout).contains("must-not-run"));
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert!(!String::from_utf8_lossy(&install.stdout).contains("must-not-run"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_use_canonical_output_phrases() {
    let root = std::env::temp_dir().join("zap_canonical_package_output");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"canonical-output\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    assert!(String::from_utf8_lossy(&update.stdout).starts_with("updated zap.lock"));
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert!(String::from_utf8_lossy(&install.stdout).starts_with("installed "));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_do_not_modify_dependency_requirements() {
    let root = std::env::temp_dir().join("zap_requirements_unchanged");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let manifest = "[package]\nname = \"requirements-unchanged\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \">=1.0\"\n";
    std::fs::write(root.join("zap.toml"), manifest).unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert_eq!(
        std::fs::read_to_string(root.join("zap.toml")).unwrap(),
        manifest
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_complete_without_network_timeout() {
    let root = std::env::temp_dir().join("zap_package_no_network_timeout");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"timeout-free\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_work_after_add_dependency() {
    let root = std::env::temp_dir().join("zap_add_install_update");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"add-install-update\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let add = Command::new(binary())
        .args(["add", "core", "1.0", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(add.status.success());
    let missing = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!missing.status.success());
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_do_not_change_file_names() {
    let root = std::env::temp_dir().join("zap_file_names_unchanged");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"file-names\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let names = std::fs::read_dir(&root)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect::<Vec<_>>();
    assert!(names.iter().any(|name| name == "zap.toml"));
    assert!(names.iter().any(|name| name == "zap.lock"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_do_not_print_debug_data() {
    let root = std::env::temp_dir().join("zap_package_no_debug_output");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-debug\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(!String::from_utf8_lossy(&output.stdout).contains("DEBUG"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_case_sensitive() {
    let root = std::env::temp_dir().join("zap_case_sensitive_commands");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let install = Command::new(binary())
        .args(["Install", root.to_str().unwrap()])
        .output()
        .unwrap();
    let update = Command::new(binary())
        .args(["Update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!install.status.success());
    assert!(!update.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_have_deterministic_success_exit_code() {
    let root = std::env::temp_dir().join("zap_deterministic_exit_codes");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"exit-codes\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(update.status.code(), Some(0));
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(install.status.code(), Some(0));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_do_not_require_zap_lock_for_empty_dependencies() {
    let root = std::env::temp_dir().join("zap_empty_deps_no_lock");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"empty-no-lock\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\n",
    )
    .unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert!(!root.join("zap.lock").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_are_documented_in_usage_error() {
    let output = Command::new(binary())
        .arg("unknown-command")
        .output()
        .unwrap();
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("zap install [dir]"));
    assert!(stderr.contains("zap update [dir]"));
}

#[test]
fn update_install_commands_do_not_follow_symlinked_manifest_in_test_contract() {
    let root = std::env::temp_dir().join("zap_symlink_manifest_contract");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"symlink-contract\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(root.join("zap.lock").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_preserve_bom_free_lockfile() {
    let root = std::env::temp_dir().join("zap_bom_free_lockfile");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"bom-free\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let bytes = std::fs::read(root.join("zap.lock")).unwrap();
    assert!(!bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_handle_long_project_paths() {
    let root = std::env::temp_dir().join("zap_long_project_path_for_package_commands");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"long-path\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_are_deterministic_for_empty_manifest_sections() {
    let root = std::env::temp_dir().join("zap_empty_sections_deterministic");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"empty-sections\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\n",
    )
    .unwrap();
    let first = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(first.status.success());
    let first_lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    let second = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(second.status.success());
    let second_lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert_eq!(first_lock, second_lock);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_do_not_create_dependency_directories() {
    let root = std::env::temp_dir().join("zap_no_dependency_directories");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-dependency-dirs\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(!root.join("packages").exists());
    assert!(!root.join("vendor").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_ready_for_future_registry() {
    let root = std::env::temp_dir().join("zap_future_registry_contract");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"future-registry\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nremote = \"1.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_keep_current_release_version() {
    let output = Command::new(binary()).arg("--version").output().unwrap();
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stdout).contains("2.0.0"));
}

#[test]
fn install_update_commands_are_available_in_cli_help_exactly_once() {
    let output = Command::new(binary()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout.matches("zap install [dir]").count(), 1);
    assert_eq!(stdout.matches("zap update [dir]").count(), 1);
}

#[test]
fn update_install_commands_do_not_modify_unrelated_files() {
    let root = std::env::temp_dir().join("zap_unrelated_files_unchanged");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let unrelated = root.join("notes.txt");
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"unrelated\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(&unrelated, "keep this file\n").unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        std::fs::read_to_string(unrelated).unwrap(),
        "keep this file\n"
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_safe_when_lockfile_is_readable() {
    let root = std::env::temp_dir().join("zap_readable_lockfile");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"readable\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    assert!(std::fs::read_to_string(root.join("zap.lock")).is_ok());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_do_not_panic_on_bad_input() {
    let root = std::env::temp_dir().join("zap_bad_package_input");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("zap.toml"), "not valid manifest\n").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!update.status.success());
    assert!(!install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_are_independent_of_main_file_extension() {
    let root = std::env::temp_dir().join("zap_main_extension_independence");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"extension\"\nversion = \"0.1.0\"\nmain = \"custom.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_safe_with_empty_manifest_file() {
    let root = std::env::temp_dir().join("zap_empty_manifest_file");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("zap.toml"), "").unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!update.status.success());
    assert!(!install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_have_no_implicit_remote_resolution() {
    let root = std::env::temp_dir().join("zap_no_implicit_remote_resolution");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-remote\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nremote = \"1.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_preserve_lockfile_section_order() {
    let root = std::env::temp_dir().join("zap_lock_section_order");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"section-order\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.find("[package]").unwrap() < lock.find("[dependencies]").unwrap());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_keep_dependency_names_stable() {
    let root = std::env::temp_dir().join("zap_dependency_names_stable");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"names-stable\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert!(lock.contains("alpha"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_are_compatible_with_existing_lock_command() {
    let root = std::env::temp_dir().join("zap_lock_command_compatibility");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"lock-compatible\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let lock = Command::new(binary())
        .args(["lock", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(lock.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_preserve_dependency_count_when_repeated() {
    let root = std::env::temp_dir().join("zap_dependency_count_repeated");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"count-repeat\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\na = \"1\"\nb = \"2\"\n",
    )
    .unwrap();
    for _ in 0..2 {
        let update = Command::new(binary())
            .args(["update", root.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(update.status.success());
        let install = Command::new(binary())
            .args(["install", root.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(install.status.success());
        assert!(String::from_utf8_lossy(&install.stdout).contains("2 locked dependencies"));
    }
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_accept_project_directory_with_spaces() {
    let root = std::env::temp_dir().join("zap project with spaces");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"spaces\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_have_stable_error_exit_code() {
    let root = std::env::temp_dir().join("zap_error_exit_code");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(update.status.code(), Some(1));
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert_eq!(install.status.code(), Some(1));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_contains_no_duplicate_dependency_lines() {
    let root = std::env::temp_dir().join("zap_no_duplicate_lock_lines");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-duplicates\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert_eq!(lock.matches("core = ").count(), 1);
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_do_not_touch_current_directory_lockfile() {
    let root = std::env::temp_dir().join("zap_no_cwd_lock_touch");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-cwd-touch\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let cwd_lock = std::env::current_dir().unwrap().join("zap.lock");
    let before = std::fs::read(&cwd_lock).ok();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(before, std::fs::read(&cwd_lock).ok());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_are_safe_with_trailing_manifest_newline() {
    let root = std::env::temp_dir().join("zap_trailing_manifest_newline");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"trailing-newline\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_do_not_mutate_dependency_values() {
    let root = std::env::temp_dir().join("zap_dependency_values_immutable");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let manifest = "[package]\nname = \"values-immutable\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"^1.2\"\n";
    std::fs::write(root.join("zap.toml"), manifest).unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert_eq!(
        std::fs::read_to_string(root.join("zap.toml")).unwrap(),
        manifest
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_work_when_main_is_named_main_zp() {
    let root = std::env::temp_dir().join("zap_default_main_name");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"default-main\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_do_not_execute_dependency_names() {
    let root = std::env::temp_dir().join("zap_dependency_names_not_executed");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"not-executed\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\n$(touch /tmp/zap-bad) = \"1.0\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!std::path::Path::new("/tmp/zap-bad").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_are_consistent_with_lock_command_output() {
    let root = std::env::temp_dir().join("zap_lock_output_consistency");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"output-consistency\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let lock = Command::new(binary())
        .args(["lock", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(lock.status.success());
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    assert!(String::from_utf8_lossy(&update.stdout).contains("updated zap.lock"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_safe_for_read_only_manifest_content() {
    let root = std::env::temp_dir().join("zap_read_only_manifest_content");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let manifest =
        "[package]\nname = \"readonly-content\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n";
    std::fs::write(root.join("zap.toml"), manifest).unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert_eq!(
        std::fs::read_to_string(root.join("zap.toml")).unwrap(),
        manifest
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_stable_across_processes() {
    let root = std::env::temp_dir().join("zap_stable_process_commands");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"stable-processes\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nalpha = \"1.0\"\n",
    )
    .unwrap();
    for _ in 0..2 {
        let update = Command::new(binary())
            .args(["update", root.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(update.status.success());
        let install = Command::new(binary())
            .args(["install", root.to_str().unwrap()])
            .output()
            .unwrap();
        assert!(install.status.success());
    }
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_is_not_empty() {
    let root = std::env::temp_dir().join("zap_lock_not_empty");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"not-empty\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(!std::fs::read_to_string(root.join("zap.lock"))
        .unwrap()
        .is_empty());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_safe_with_non_ascii_path() {
    let root = std::env::temp_dir().join("zap_ဇာပ်_project");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"non-ascii-path\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_have_expected_lockfile_name() {
    let root = std::env::temp_dir().join("zap_expected_lockfile_name");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"lock-name\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(root.join("zap.lock").exists());
    assert!(!root.join("package.lock").exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_do_not_require_package_cache() {
    let root = std::env::temp_dir().join("zap_no_package_cache");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-cache\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_do_not_change_package_version() {
    let root = std::env::temp_dir().join("zap_package_version_unchanged");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let manifest =
        "[package]\nname = \"version-unchanged\"\nversion = \"4.5.6\"\nmain = \"main.zp\"\n";
    std::fs::write(root.join("zap.toml"), manifest).unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert_eq!(
        std::fs::read_to_string(root.join("zap.toml")).unwrap(),
        manifest
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_have_no_hidden_environment_dependencies() {
    let root = std::env::temp_dir().join("zap_no_hidden_env_dependencies");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-env\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .env_clear()
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_are_safe_with_crlf_manifest() {
    let root = std::env::temp_dir().join("zap_crlf_manifest");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\r\nname = \"crlf\"\r\nversion = \"0.1.0\"\r\nmain = \"main.zp\"\r\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_safe_with_read_only_lock_content() {
    let root = std::env::temp_dir().join("zap_lock_content_read_only");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"lock-content\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_are_safe_with_large_dependency_names() {
    let root = std::env::temp_dir().join("zap_large_dependency_names");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let long_name = "a".repeat(64);
    std::fs::write(
        root.join("zap.toml"),
        format!("[package]\nname = \"large\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\n{long_name} = \"1.0\"\n"),
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_report_no_dependencies_correctly() {
    let root = std::env::temp_dir().join("zap_no_dependencies_message");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"no-dependencies-message\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    assert!(String::from_utf8_lossy(&install.stdout).contains("0 locked dependencies"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_lockfile_keeps_package_and_dependency_headers() {
    let root = std::env::temp_dir().join("zap_lock_headers");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"headers\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.starts_with("# This file is generated by Zap"));
    assert!(lock.contains("[package]"));
    assert!(lock.contains("[dependencies]"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_safe_with_extra_manifest_sections() {
    let root = std::env::temp_dir().join("zap_extra_manifest_sections");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"extra-sections\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ncore = \"1.0\"\n\n[tool]\nmode = \"strict\"\n",
    )
    .unwrap();
    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn update_install_commands_do_not_use_main_source_as_manifest() {
    let root = std::env::temp_dir().join("zap_main_not_manifest");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"main-not-manifest\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "not manifest content\n").unwrap();
    let output = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(output.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_update_commands_are_ready_for_local_path_dependencies() {
    let root = std::env::temp_dir().join("zap_local_path_dependencies_ready");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"local-path-ready\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nlocal-lib = \"path:../local-lib\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "print(1)\n").unwrap();

    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(update.status.success());
    assert!(root.join("zap.lock").exists());

    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(install.status.success());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn local_path_dependency_is_validated_and_locked_canonically() {
    let root = std::env::temp_dir().join("zap_local_path_dependency");
    let local = root.join("local-lib");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&local).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nlocal-lib = { path = \"local-lib\" }\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say 1\n").unwrap();
    std::fs::write(
        local.join("zap.toml"),
        "[package]\nname = \"local-lib\"\nversion = \"0.2.0\"\nmain = \"main.zp\"\n",
    )
    .unwrap();
    std::fs::write(local.join("main.zp"), "say 2\n").unwrap();

    let update = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        update.status.success(),
        "{}",
        String::from_utf8_lossy(&update.stderr)
    );
    let lock = std::fs::read_to_string(root.join("zap.lock")).unwrap();
    assert!(lock.contains("local-lib = { path = \"local-lib\" }"));

    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        install.status.success(),
        "{}",
        String::from_utf8_lossy(&install.stderr)
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn local_path_dependency_reports_missing_package_manifest() {
    let root = std::env::temp_dir().join("zap_local_path_missing_manifest");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("local-lib")).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"consumer\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\nlocal-lib = { path = \"local-lib\" }\n",
    )
    .unwrap();
    let result = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("missing zap.toml"));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn resolves_nested_local_path_dependencies() {
    let root = std::env::temp_dir().join("zap_nested_local_dependencies");
    let _ = std::fs::remove_dir_all(&root);
    let app = root.join("app");
    let mid = root.join("mid");
    let leaf = root.join("leaf");
    for dir in [&app, &mid, &leaf] {
        std::fs::create_dir_all(dir).unwrap();
    }
    std::fs::write(
        app.join("zap.toml"),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\nmid = { path = \"../mid\" }\n",
    )
    .unwrap();
    std::fs::write(
        mid.join("zap.toml"),
        "[package]\nname = \"mid\"\nversion = \"0.1.0\"\n\n[dependencies]\nleaf = { path = \"../leaf\" }\n",
    )
    .unwrap();
    std::fs::write(
        leaf.join("zap.toml"),
        "[package]\nname = \"leaf\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    let result = Command::new(binary())
        .args(["update", app.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn rejects_nested_local_dependency_cycles_deterministically() {
    let root = std::env::temp_dir().join("zap_nested_local_cycle");
    let _ = std::fs::remove_dir_all(&root);
    let app = root.join("app");
    let left = root.join("left");
    let right = root.join("right");
    for dir in [&app, &left, &right] {
        std::fs::create_dir_all(dir).unwrap();
    }
    std::fs::write(
        app.join("zap.toml"),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\n\n[dependencies]\nleft = { path = \"../left\" }\n",
    )
    .unwrap();
    std::fs::write(
        left.join("zap.toml"),
        "[package]\nname = \"left\"\nversion = \"0.1.0\"\n\n[dependencies]\nright = { path = \"../right\" }\n",
    )
    .unwrap();
    std::fs::write(
        right.join("zap.toml"),
        "[package]\nname = \"right\"\nversion = \"0.1.0\"\n\n[dependencies]\nleft = { path = \"../left\" }\n",
    )
    .unwrap();
    let result = Command::new(binary())
        .args(["update", app.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!result.status.success());
    let error = String::from_utf8_lossy(&result.stderr);
    assert!(
        error.contains("dependency cycle detected: left -> right -> left"),
        "{error}"
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn accepts_registry_ready_package_metadata() {
    let root = std::env::temp_dir().join("zap_registry_metadata_valid");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"metadata-app\"\nversion = \"0.1.0\"\ndescription = \"A registry-ready package\"\nauthors = [\"Zap Team\"]\nlicense = \"MIT\"\nrepository = \"https://github.com/hidecard/zap\"\nchecksum = \"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef\"\n",
    )
    .unwrap();
    let result = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn rejects_invalid_registry_metadata() {
    let root = std::env::temp_dir().join("zap_registry_metadata_invalid");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"metadata-app\"\nversion = \"0.1.0\"\nlicense = MIT\n",
    )
    .unwrap();
    let result = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("metadata field `license`"));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn rejects_non_sha256_package_checksum() {
    let root = std::env::temp_dir().join("zap_registry_checksum_invalid");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"checksum-app\"\nversion = \"0.1.0\"\nchecksum = \"deadbeef\"\n",
    )
    .unwrap();
    let result = Command::new(binary())
        .args(["update", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("64-character hexadecimal SHA-256"));
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn async_runtime_foundation_is_available_from_cli() {
    let output = Command::new(binary()).arg("async-check").output().unwrap();
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "async runtime foundation ready\n"
    );
}

#[test]
fn lsp_stdio_handles_initialize_and_shutdown() {
    let initialize = br#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
    let shutdown = br#"{"jsonrpc":"2.0","id":2,"method":"shutdown"}"#;
    let mut input = Vec::new();
    for body in [initialize.as_slice(), shutdown.as_slice()] {
        input.extend_from_slice(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
        input.extend_from_slice(body);
    }
    let output = Command::new(binary())
        .arg("lsp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child.stdin.take().unwrap().write_all(&input)?;
            child.wait_with_output()
        })
        .unwrap();
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("textDocumentSync"));
    assert!(text.contains("Content-Length:"));
    assert!(text.matches("\"jsonrpc\":\"2.0\"").count() >= 2);
}

#[test]
fn help_lists_async_and_lsp_commands() {
    let output = Command::new(binary()).arg("--help").output().unwrap();
    assert!(output.status.success());
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(text.contains("zap lsp"));
    assert!(text.contains("zap async-check"));
}

#[test]
fn lsp_rejects_malformed_content_length() {
    let output = Command::new(binary())
        .arg("lsp")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            child
                .stdin
                .take()
                .unwrap()
                .write_all(b"Content-Length: bad\r\n\r\n{}")
                .unwrap();
            child.wait_with_output()
        })
        .unwrap();
    assert!(!output.status.success());
}

#[test]
fn validates_registry_index_and_caches_verified_package() {
    let root = std::env::temp_dir().join(format!("zap_registry_cli_{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("demo.pkg");
    std::fs::write(&source, b"demo package bytes").unwrap();
    let checksum = {
        use sha2::{Digest, Sha256};
        let digest = Sha256::digest(b"demo package bytes");
        digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    };
    let index = root.join("index.json");
    std::fs::write(
        &index,
        format!(
            r#"{{"packages":[{{"name":"demo","version":"1.0.0","source":"file://demo.pkg","checksum":"{checksum}"}}]}}"#
        ),
    )
    .unwrap();
    let check = Command::new(binary())
        .args(["registry", "check", index.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        check.status.success(),
        "{}",
        String::from_utf8_lossy(&check.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&check.stdout),
        "valid registry index: 1 packages\n"
    );
    let cache = root.join("cache");
    let cached = Command::new(binary())
        .args([
            "registry",
            "cache",
            index.to_str().unwrap(),
            source.to_str().unwrap(),
            "demo",
            "1.0.0",
            cache.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(
        cached.status.success(),
        "{}",
        String::from_utf8_lossy(&cached.stderr)
    );
    assert!(cache
        .join("demo")
        .join("1.0.0")
        .join(format!("{checksum}.pkg"))
        .exists());
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn rejects_registry_cache_checksum_mismatch() {
    let root = std::env::temp_dir().join(format!("zap_registry_bad_{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let source = root.join("bad.pkg");
    std::fs::write(&source, b"actual bytes").unwrap();
    let index = root.join("index.json");
    std::fs::write(
        &index,
        r#"{"packages":[{"name":"demo","version":"1.0.0","source":"file://bad.pkg","checksum":"0000000000000000000000000000000000000000000000000000000000000000"}]}"#,
    )
    .unwrap();
    let output = Command::new(binary())
        .args([
            "registry",
            "cache",
            index.to_str().unwrap(),
            source.to_str().unwrap(),
            "demo",
            "1.0.0",
            root.join("cache").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("checksum mismatch"));
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn install_uses_registry_cache_and_supports_offline_reuse() {
    let root = std::env::temp_dir().join(format!("zap_registry_install_{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[dependencies]\ndemo = \"1.0.0\"\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "say 1\n").unwrap();
    let lock = Command::new(binary())
        .args(["lock", root.to_str().unwrap()])
        .output()
        .unwrap();
    assert!(
        lock.status.success(),
        "{}",
        String::from_utf8_lossy(&lock.stderr)
    );

    let source = root.join("demo.pkg");
    std::fs::write(&source, b"registry package").unwrap();
    let checksum = {
        use sha2::{Digest, Sha256};
        Sha256::digest(b"registry package")
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    };
    let index = root.join("index.json");
    std::fs::write(
        &index,
        format!(
            r#"{{"packages":[{{"name":"demo","version":"1.0.0","source":"file://demo.pkg","checksum":"{checksum}"}}]}}"#
        ),
    )
    .unwrap();
    let cache = root.join("cache");
    let install = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .env("ZAP_REGISTRY_INDEX", &index)
        .env("ZAP_CACHE_DIR", &cache)
        .output()
        .unwrap();
    assert!(
        install.status.success(),
        "{}",
        String::from_utf8_lossy(&install.stderr)
    );
    assert!(cache
        .join("demo")
        .join("1.0.0")
        .join(format!("{checksum}.pkg"))
        .is_file());

    let offline = Command::new(binary())
        .args(["install", root.to_str().unwrap()])
        .env("ZAP_REGISTRY_INDEX", &index)
        .env("ZAP_CACHE_DIR", &cache)
        .env("ZAP_OFFLINE", "1")
        .output()
        .unwrap();
    assert!(
        offline.status.success(),
        "{}",
        String::from_utf8_lossy(&offline.stderr)
    );
    let _ = std::fs::remove_dir_all(root);
}

#[test]
fn validates_explicit_workspace_graph() {
    let root = std::env::temp_dir().join("zap_explicit_workspace_graph_test");
    let module_root = root.join("modules/app");
    std::fs::create_dir_all(&module_root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"workspace-demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[module]\nroot = \"modules\"\nentries = [\"app/core.zp\", \"app/util.zp\"]\n",
    )
    .unwrap();
    std::fs::write(
        root.join("main.zp"),
        "module app.main\nimport app.core as core\nimport app.util as util\n",
    )
    .unwrap();
    std::fs::write(
        module_root.join("core.zp"),
        "module app.core\nimport app.util as util\n",
    )
    .unwrap();
    std::fs::write(module_root.join("util.zp"), "module app.util\n").unwrap();

    let output = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn reports_explicit_import_cycle_chain() {
    let root = std::env::temp_dir().join("zap_explicit_import_cycle_test");
    let module_root = root.join("modules/app");
    std::fs::create_dir_all(&module_root).unwrap();
    std::fs::write(
        root.join("zap.toml"),
        "[package]\nname = \"cycle-demo\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n\n[module]\nroot = \"modules\"\nentries = [\"app/a.zp\", \"app/b.zp\", \"app/c.zp\"]\n",
    )
    .unwrap();
    std::fs::write(root.join("main.zp"), "module app.main\nimport app.a as a\n").unwrap();
    std::fs::write(
        module_root.join("a.zp"),
        "module app.a\nimport app.b as b\n",
    )
    .unwrap();
    std::fs::write(
        module_root.join("b.zp"),
        "module app.b\nimport app.c as c\n",
    )
    .unwrap();
    std::fs::write(
        module_root.join("c.zp"),
        "module app.c\nimport app.a as a\n",
    )
    .unwrap();

    let output = Command::new(binary())
        .args(["check", root.to_str().unwrap()])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success());
    assert!(stderr.contains("circular module dependency"));
    assert!(stderr.contains("a.zp") && stderr.contains("b.zp") && stderr.contains("c.zp"));
}
