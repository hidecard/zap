use std::{env, fs, path::PathBuf};

pub(crate) const DEFAULT_SEED: u64 = 20_260_821;
pub(crate) const CATEGORIES: [&str; 6] =
    ["parser", "json", "lockfile", "registry", "memory", "async"];

pub(crate) fn seed() -> u64 {
    env::var("ZAP_CORPUS_SEED")
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(DEFAULT_SEED)
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
    use super::{fixture_cases, replay_order, seed, CATEGORIES, DEFAULT_SEED};
    use crate::{
        ast::parse_program, async_runtime::AsyncRuntime, evaluator::json_to_value,
        lexer::tokenize_with_spans, project::parse_resolved_lockfile, registry::parse_index_bytes,
        value::Value,
    };
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
        assert_eq!(DEFAULT_SEED, 20_260_821);
        let mut replayed = 0usize;
        for category in CATEGORIES {
            let cases = fixture_cases(category).expect("durable replay corpus must be readable");
            for index in replay_order(cases.len(), corpus_seed ^ category.len() as u64) {
                let (name, input) = &cases[index];
                let first = catch_unwind(|| replay(category, input));
                let second = catch_unwind(|| replay(category, input));
                assert!(first.is_ok(), "replay panicked for {category}/{name}");
                assert!(
                    second.is_ok(),
                    "replay panicked on repeat for {category}/{name}"
                );
                assert_eq!(
                    first.unwrap(),
                    second.unwrap(),
                    "replay changed for {category}/{name}"
                );
                replayed += 1;
            }
        }
        assert!(replayed >= 12, "replay corpus is too small");
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
}
