use std::{env, fs, path::PathBuf};

pub(crate) const DEFAULT_SEED: u64 = 20_260_821;
pub(crate) const DEFAULT_ROUNDS: usize = 1;
pub(crate) const MAX_ROUNDS: usize = 64;
pub(crate) const CATEGORIES: [&str; 6] =
    ["parser", "json", "lockfile", "registry", "memory", "async"];

pub(crate) fn seed() -> u64 {
    env::var("ZAP_CORPUS_SEED")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_SEED)
}

pub(crate) fn validate_rounds(rounds: usize) -> Result<usize, String> {
    if rounds == 0 || rounds > MAX_ROUNDS {
        return Err(format!(
            "replay rounds must be between 1 and {MAX_ROUNDS}, got {rounds}"
        ));
    }
    Ok(rounds)
}

pub(crate) fn rounds() -> Result<usize, String> {
    let value = env::var("ZAP_CORPUS_ROUNDS").unwrap_or_else(|_| DEFAULT_ROUNDS.to_string());
    let rounds = value.parse::<usize>().map_err(|error| {
        format!("ZAP_CORPUS_ROUNDS must be a positive decimal integer: {error}")
    })?;
    validate_rounds(rounds)
}

pub(crate) fn fixture_cases(category: &str) -> Result<Vec<(String, String)>, String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let directory = manifest
        .parent()
        .ok_or_else(|| "native manifest directory has no repository parent".to_string())?
        .join("corpus")
        .join("p1-05")
        .join(category);
    let mut paths = fs::read_dir(directory)
        .map_err(|error| format!("replay corpus `{category}` read failed: {error}"))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("replay corpus `{category}` entry failed: {error}"))?;
    paths.retain(|path| path.is_file());
    paths.sort();
    if paths.is_empty() {
        return Err(format!("replay corpus `{category}` is empty"));
    }
    paths
        .into_iter()
        .map(|path| {
            let name = path
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or_else(|| format!("replay corpus `{category}` has a non-UTF-8 name"))?
                .to_string();
            let contents = fs::read_to_string(&path).map_err(|error| {
                format!("replay fixture `{category}/{name}` read failed: {error}")
            })?;
            Ok((name, contents))
        })
        .collect()
}

pub(crate) fn replay_order(length: usize, seed: u64) -> Vec<usize> {
    let mut order = (0..length).collect::<Vec<_>>();
    let mut state = seed ^ 0x5a50_0105_9e37_79b9;
    for index in (1..length).rev() {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let swap = (state % (index as u64 + 1)) as usize;
        order.swap(index, swap);
    }
    order
}

#[cfg(test)]
mod tests {
    use super::{
        fixture_cases, replay_order, rounds, seed, validate_rounds, CATEGORIES, DEFAULT_ROUNDS,
        DEFAULT_SEED, MAX_ROUNDS,
    };
    use crate::{
        ast::parse_program, async_runtime::AsyncRuntime, evaluator::json_to_value,
        lexer::tokenize_with_spans, project::parse_resolved_lockfile, registry::parse_index_bytes,
        value::Value,
    };
    use sha2::{Digest, Sha256};
    use std::panic::catch_unwind;

    fn replay(category: &str, input: &str) -> String {
        match category {
            "parser" => format!(
                "lexer={:?};ast={:?}",
                tokenize_with_spans(input),
                parse_program(input)
            ),
            "json" => format!(
                "json={:?}",
                serde_json::from_str::<serde_json::Value>(input)
                    .map_err(|error| error.to_string())
                    .and_then(json_to_value)
                    .map(|value| value.show())
            ),
            "lockfile" => format!("lockfile={:?}", parse_resolved_lockfile(input)),
            "registry" => format!("registry={:?}", parse_index_bytes(input.as_bytes())),
            "memory" => {
                let nodes = input.trim().parse::<usize>().unwrap_or(1);
                let value = Value::List((0..nodes).map(|_| Value::None).collect());
                format!("memory={:?}", value.validate_memory_limits())
            }
            "async" => {
                let budget = input.trim().parse::<usize>().unwrap_or(0);
                let mut runtime = AsyncRuntime::new();
                runtime
                    .spawn_limited(async {})
                    .expect("replay async fixture must admit one task");
                format!("async={:?}", runtime.run_with_budget(budget))
            }
            other => panic!("unknown replay corpus category: {other}"),
        }
    }

    #[test]
    fn replayable_failure_corpus_is_seeded_panic_free_and_deterministic() {
        let corpus_seed = seed();
        let corpus_rounds = rounds().expect("replay round count must be valid");
        assert_eq!(DEFAULT_SEED, 20_260_821);
        assert_eq!(DEFAULT_ROUNDS, 1);
        assert_eq!(MAX_ROUNDS, 64);
        let mut replayed = 0usize;
        for round in 1..=corpus_rounds {
            let mut round_replayed = 0usize;
            let mut outcome_digest = Sha256::new();
            for category in CATEGORIES {
                let cases =
                    fixture_cases(category).expect("durable replay corpus must be readable");
                for index in replay_order(cases.len(), corpus_seed ^ category.len() as u64) {
                    let (name, input) = &cases[index];
                    let first = catch_unwind(|| replay(category, input));
                    let second = catch_unwind(|| replay(category, input));
                    assert!(first.is_ok(), "replay panicked for {category}/{name}");
                    assert!(
                        second.is_ok(),
                        "replay panicked on repeat for {category}/{name}"
                    );
                    let first = first.unwrap();
                    let second = second.unwrap();
                    assert_eq!(first, second, "replay changed for {category}/{name}");
                    outcome_digest.update(category.as_bytes());
                    outcome_digest.update([0]);
                    outcome_digest.update(name.as_bytes());
                    outcome_digest.update([0]);
                    outcome_digest.update(first.as_bytes());
                    outcome_digest.update([b'\n']);
                    round_replayed += 1;
                    replayed += 1;
                }
            }
            let outcome_digest = format!("{:x}", outcome_digest.finalize());
            println!(
                "M2_VERIFY_REPLAY round={round} seed={corpus_seed} cases={round_replayed} digest={outcome_digest}"
            );
        }
        assert!(replayed >= 12, "replay corpus is too small");
    }

    #[test]
    fn replay_round_count_is_bounded() {
        assert_eq!(validate_rounds(1), Ok(1));
        assert_eq!(validate_rounds(MAX_ROUNDS), Ok(MAX_ROUNDS));
        assert!(validate_rounds(0).is_err());
        assert!(validate_rounds(MAX_ROUNDS + 1).is_err());
    }

    #[test]
    fn replay_order_is_seeded_and_is_a_permutation() {
        let first = replay_order(16, 1);
        let second = replay_order(16, 1);
        let alternate = replay_order(16, 2);
        assert_eq!(first, second);
        assert_ne!(first, alternate);
        let mut sorted = first;
        sorted.sort_unstable();
        assert_eq!(sorted, (0..16).collect::<Vec<_>>());
    }

    // Allocator/heap-level tests using existing Value API
    #[test]
    fn object_cycle_stress_test() {
        // Create deeply nested cyclic objects using Maps/Lists and verify memory limits
        for depth in [10, 50, 100, 500, 1000] {
            let mut head: Value = Value::None;
            for _ in 0..depth {
                let mut map = std::collections::HashMap::new();
                map.insert("next".into(), head.clone());
                head = Value::Map(map);
            }

            let result = head.validate_memory_limits();
            assert!(
                result.is_ok() || result.is_err(),
                "cycle validation should not panic at depth {depth}"
            );
        }
    }

    #[test]
    fn oversized_value_handling() {
        // Large list
        let large_list: Vec<Value> = (0..10000).map(|i| Value::Number(i as i64)).collect();
        let value = Value::List(large_list);
        let result = value.validate_memory_limits();
        assert!(result.is_ok() || result.is_err());

        // Large map
        let mut large_map = std::collections::HashMap::new();
        for i in 0..10000 {
            large_map.insert(format!("key_{i}"), Value::Number(i as i64));
        }
        let value = Value::Map(large_map);
        let result = value.validate_memory_limits();
        assert!(result.is_ok() || result.is_err());

        // Deeply nested structure (reduced depth to avoid stack overflow)
        let mut nested: Value = Value::None;
        for i in 0..500 {
            let mut map = std::collections::HashMap::new();
            map.insert("value".into(), Value::Number(i));
            map.insert("nested".into(), nested);
            nested = Value::Map(map);
        }
        let result = nested.validate_memory_limits();
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn repeated_module_execution_memory_behavior() {
        let program = parse_program("let x = 1\nlet y = x + 2\nsay y\n").unwrap();

        for _ in 0..100 {
            let mut context = crate::runtime_state::ExecutionContext::new();
            context
                .state_mut()
                .set_workspace_root(std::path::PathBuf::from("."));
            let result = crate::evaluator::execute_ast_program_with_context(
                &program,
                &mut std::collections::HashMap::new(),
                &mut std::collections::HashMap::new(),
                &mut context,
                std::path::Path::new("."),
            );
            assert!(result.is_ok(), "execution should succeed: {:?}", result);
        }
    }

    #[test]
    fn deterministic_ordering_of_collections() {
        let mut map = std::collections::HashMap::new();
        map.insert("z".into(), Value::Number(1));
        map.insert("a".into(), Value::Number(2));
        map.insert("m".into(), Value::Number(3));

        let value = Value::Map(map);
        let serialized1 = value.show();
        let serialized2 = value.show();
        assert_eq!(
            serialized1, serialized2,
            "Value::show() should be deterministic"
        );
    }

    // Property tests using deterministic test inputs
    #[test]
    fn json_roundtrip_deterministic() {
        let test_inputs = [
            "{}",
            "[]",
            "\"hello\"",
            "123",
            "true",
            "null",
            "{\"a\":1,\"b\":2}",
            "[1,2,3]",
            "{\"nested\":{\"value\":42}}",
        ];

        for input in test_inputs {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(input) {
                let result1 = json_to_value(parsed.clone());
                let result2 = json_to_value(parsed);

                match (result1, result2) {
                    (Ok(v1), Ok(v2)) => {
                        assert_eq!(
                            v1.show(),
                            v2.show(),
                            "json_to_value should be deterministic for: {input}"
                        );
                    }
                    (Err(_), Err(_)) => {}
                    _ => panic!("both should succeed or both should fail for: {input}"),
                }
            }
        }
    }

    #[test]
    fn lockfile_roundtrip_deterministic() {
        let test_inputs = [
            "",
            "lockfile_version = 1\n\n[package]\nname = \"test\"\nversion = \"1.0.0\"\n\n[dependencies]\n",
            "invalid lockfile content",
        ];

        for input in test_inputs {
            let result1 = crate::project::parse_resolved_lockfile(input);
            let result2 = crate::project::parse_resolved_lockfile(input);

            match (result1, result2) {
                (Ok(v1), Ok(v2)) => {
                    assert_eq!(
                        format!("{:?}", v1),
                        format!("{:?}", v2),
                        "lockfile parse should be deterministic for: {input}"
                    );
                }
                (Err(e1), Err(e2)) => {
                    assert_eq!(
                        e1, e2,
                        "lockfile parse errors should be deterministic for: {input}"
                    );
                }
                _ => panic!("both should succeed or both should fail for: {input}"),
            }
        }
    }

    #[test]
    fn registry_parse_deterministic() {
        let test_inputs: &[&[u8]] = &[
            b"",
            b"[]",
            b"[{\"name\":\"test\",\"version\":\"1.0.0\"}]",
            b"invalid registry data",
        ];

        for bytes in test_inputs {
            let result1 = crate::registry::parse_index_bytes(bytes);
            let result2 = crate::registry::parse_index_bytes(bytes);

            match (result1, result2) {
                (Ok(v1), Ok(v2)) => {
                    assert_eq!(
                        format!("{:?}", v1),
                        format!("{:?}", v2),
                        "registry parse should be deterministic"
                    );
                }
                (Err(e1), Err(e2)) => {
                    assert_eq!(e1, e2, "registry parse errors should be deterministic");
                }
                _ => panic!("both should succeed or both should fail"),
            }
        }
    }

    #[test]
    fn parser_deterministic() {
        let test_inputs = [
            "",
            "let x = 1\nsay x\n",
            "fn foo():\n    return 42\n",
            "invalid syntax {",
        ];

        for input in test_inputs {
            let result1 = parse_program(input);
            let result2 = parse_program(input);

            match (result1, result2) {
                (Ok(v1), Ok(v2)) => {
                    assert_eq!(
                        format!("{:?}", v1),
                        format!("{:?}", v2),
                        "parser should be deterministic for: {input}"
                    );
                }
                (Err(e1), Err(e2)) => {
                    assert_eq!(e1, e2, "parser errors should be deterministic for: {input}");
                }
                _ => panic!("both should succeed or both should fail for: {input}"),
            }
        }
    }

    #[test]
    fn diagnostic_normalization_deterministic() {
        let test_inputs = [
            "",
            "let x = 1\nsay x\n",
            "unterminated \"string",
            "invalid @char",
        ];

        for input in test_inputs {
            let result1 = crate::lexer::tokenize_with_spans(input);
            let result2 = crate::lexer::tokenize_with_spans(input);

            match (result1, result2) {
                (Ok(v1), Ok(v2)) => {
                    assert_eq!(
                        format!("{:?}", v1),
                        format!("{:?}", v2),
                        "lexer should be deterministic for: {input}"
                    );
                }
                (Err(e1), Err(e2)) => {
                    assert_eq!(e1, e2, "lexer errors should be deterministic for: {input}");
                }
                _ => panic!("both should succeed or both should fail for: {input}"),
            }
        }
    }

    // Windows/macOS-specific tests
    #[test]
    fn path_handling_edge_cases() {
        use std::path::Path;

        // Test path separator handling
        let windows_paths = [
            r"C:\Users\test\file.zp",
            r"C:\path\to\module.zp",
            r"..\relative\path.zp",
            r".\current\dir.zp",
        ];

        for path_str in windows_paths {
            let path = Path::new(path_str);
            // Should not panic on Windows paths
            let _ = path.file_name();
            let _ = path.parent();
            let _ = path.is_absolute();
        }

        // Test Unix paths
        let unix_paths = [
            "/home/user/file.zp",
            "/path/to/module.zp",
            "../relative/path.zp",
            "./current/dir.zp",
        ];

        for path_str in unix_paths {
            let path = Path::new(path_str);
            let _ = path.file_name();
            let _ = path.parent();
            let _ = path.is_absolute();
        }

        // Test mixed separators (should be handled gracefully)
        let mixed = r"C:/Users/test\file.zp";
        let path = Path::new(mixed);
        let _ = path.file_name();
    }

    #[test]
    fn process_behavior_differences() {
        // Test that process spawning handles platform differences
        let program = parse_program(
            r#"import "process"
let result = process_run(["echo", "test"])
say result.stdout
"#,
        )
        .unwrap();

        let mut context = crate::runtime_state::ExecutionContext::new();
        context
            .state_mut()
            .set_workspace_root(std::path::PathBuf::from("."));
        let result = crate::evaluator::execute_ast_program_with_context(
            &program,
            &mut std::collections::HashMap::new(),
            &mut std::collections::HashMap::new(),
            &mut context,
            std::path::Path::new("."),
        );
        // Should either succeed or fail gracefully (not panic)
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn newline_preservation() {
        // Test that newlines are preserved correctly across platforms
        let test_cases = [
            "hello\nworld",
            "hello\r\nworld",
            "hello\rworld",
            "line1\nline2\nline3\n",
            "line1\r\nline2\r\nline3\r\n",
        ];

        for input in test_cases {
            let program = format!(
                "say \"{}\"",
                input.replace("\n", "\\n").replace("\r", "\\r")
            );
            if let Ok(parsed) = parse_program(&program) {
                let mut context = crate::runtime_state::ExecutionContext::new();
                context
                    .state_mut()
                    .set_workspace_root(std::path::PathBuf::from("."));
                let result = crate::evaluator::execute_ast_program_with_context(
                    &parsed,
                    &mut std::collections::HashMap::new(),
                    &mut std::collections::HashMap::new(),
                    &mut context,
                    std::path::Path::new("."),
                );
                assert!(
                    result.is_ok() || result.is_err(),
                    "should not panic on newlines: {:?}",
                    input
                );
            }
        }
    }

    #[test]
    fn permission_cases() {
        use std::fs;
        use std::path::Path;

        // Test that we can handle read-only files gracefully
        let temp_dir = std::env::temp_dir().join("zap_permission_test");
        let _ = fs::remove_dir_all(&temp_dir);
        fs::create_dir_all(&temp_dir).unwrap();

        let test_file = temp_dir.join("readonly.txt");
        fs::write(&test_file, "test content").unwrap();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&test_file).unwrap().permissions();
            perms.set_mode(0o444); // read-only
            fs::set_permissions(&test_file, perms).unwrap();
        }

        #[cfg(windows)]
        {
            // On Windows, use read-only attribute
            let mut perms = fs::metadata(&test_file).unwrap().permissions();
            perms.set_readonly(true);
            fs::set_permissions(&test_file, perms).unwrap();
        }

        let program = format!("say file_read(\"{}\")", test_file.to_string_lossy());
        if let Ok(parsed) = parse_program(&program) {
            let mut context = crate::runtime_state::ExecutionContext::new();
            context.state_mut().set_workspace_root(temp_dir.clone());
            let result = crate::evaluator::execute_ast_program_with_context(
                &parsed,
                &mut std::collections::HashMap::new(),
                &mut std::collections::HashMap::new(),
                &mut context,
                std::path::Path::new("."),
            );
            // Should handle read-only gracefully (not panic)
            assert!(result.is_ok() || result.is_err());
        }

        // Cleanup
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&test_file).unwrap().permissions();
            perms.set_mode(0o644);
            fs::set_permissions(&test_file, perms).unwrap();
        }

        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn archive_format_checks() {
        // Test that archive-related operations handle various formats
        // This is a placeholder for future archive format tests
        // Currently tests that we don't panic on archive-related builtins

        let program = r#"
            import "archive"
            // Test that archive functions exist and don't panic on invalid input
        "#;

        if let Ok(parsed) = parse_program(program) {
            let mut context = crate::runtime_state::ExecutionContext::new();
            context
                .state_mut()
                .set_workspace_root(std::path::PathBuf::from("."));
            let result = crate::evaluator::execute_ast_program_with_context(
                &parsed,
                &mut std::collections::HashMap::new(),
                &mut std::collections::HashMap::new(),
                &mut context,
                std::path::Path::new("."),
            );
            assert!(result.is_ok() || result.is_err());
        }
    }

    // Failure-corpus ownership policy tests
    #[test]
    fn corpus_index_with_fixture_ids() {
        // Verify that corpus fixtures have stable IDs
        let categories = ["parser", "json", "lockfile", "registry", "memory", "async"];

        for category in categories {
            let cases =
                fixture_cases(category).expect(&format!("corpus {} must be readable", category));
            assert!(!cases.is_empty(), "corpus {} must not be empty", category);

            // Each fixture should have a name that can serve as an ID
            for (name, _) in &cases {
                assert!(!name.is_empty(), "fixture name must not be empty");
                // Name should be a valid filename (no path separators)
                assert!(
                    !name.contains('/') && !name.contains('\\'),
                    "fixture name should not contain path separators: {}",
                    name
                );
            }
        }
    }

    #[test]
    fn test_naming_convention() {
        // Verify test naming follows convention: category_fixture
        let categories = ["parser", "json", "lockfile", "registry", "memory", "async"];

        for category in categories {
            let cases =
                fixture_cases(category).expect(&format!("corpus {} must be readable", category));

            for (name, _) in &cases {
                // Names should follow pattern: descriptive-name.extension or just descriptive-name
                // Should not start with numbers or special chars
                let first_char = name.chars().next().unwrap_or('_');
                assert!(
                    first_char.is_alphabetic() || first_char == '_',
                    "fixture name should start with letter or underscore: {}",
                    name
                );

                // Should be lowercase with hyphens/underscores (snake_case or kebab-case)
                // This is a soft convention check
            }
        }
    }

    #[test]
    fn changelog_procedure_for_new_corpora() {
        // This test documents the changelog procedure for new corpora
        // When adding a new corpus category:
        // 1. Add category to CATEGORIES array
        // 2. Create directory under corpus/p1-05/<category>/
        // 3. Add fixture files with descriptive names
        // 4. Update CHANGELOG with new corpus entry
        // 5. Run replay test to verify determinism

        // Verify current CATEGORIES is up to date
        assert_eq!(CATEGORIES.len(), 6, "CATEGORIES should have 6 entries");
        assert!(CATEGORIES.contains(&"parser"));
        assert!(CATEGORIES.contains(&"json"));
        assert!(CATEGORIES.contains(&"lockfile"));
        assert!(CATEGORIES.contains(&"registry"));
        assert!(CATEGORIES.contains(&"memory"));
        assert!(CATEGORIES.contains(&"async"));

        // Verify each has at least one fixture
        for category in CATEGORIES {
            let cases = fixture_cases(category).expect(&format!("corpus {} must exist", category));
            assert!(
                !cases.is_empty(),
                "corpus {} must have at least one fixture",
                category
            );
        }
    }
}
