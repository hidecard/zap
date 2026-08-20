use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{
    manifest_value, read_limited_text, run, validate_function_calls, validate_function_returns,
    validate_function_signatures,
};

pub(crate) fn validate_project(dir: &Path) -> Result<String, String> {
    let manifest = dir.join("zap.toml");
    let text = read_limited_text(&manifest, "manifest read")?;
    let name = manifest_value(&text, "name").ok_or("zap.toml: missing package name".to_string())?;
    let version =
        manifest_value(&text, "version").ok_or("zap.toml: missing package version".to_string())?;
    let main = manifest_value(&text, "main").unwrap_or_else(|| "main.zp".into());
    let main_path = dir.join(&main);
    if !main_path.is_file() {
        return Err(format!(
            "zap.toml: main file not found: {}",
            main_path.display()
        ));
    }
    let source = read_limited_text(&main_path, "source read")?;
    validate_function_signatures(&source, &main_path)?;
    validate_function_returns(&source, &main_path)?;
    validate_function_calls(&source, &main_path)?;
    Ok(format!("{name} {version} (main: {main})"))
}

pub(crate) fn resolve_module(base: &Path, raw: &str) -> Option<PathBuf> {
    let candidate = if Path::new(raw).extension().is_some() {
        raw.to_string()
    } else {
        format!("{raw}.zp")
    };
    let candidates = [
        base.join(&candidate),
        base.join("modules").join(&candidate),
        base.join("lib").join(&candidate),
    ];
    candidates.into_iter().find(|p| p.is_file())
}

pub(crate) fn collect_test_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(dir)
        .map_err(|e| format!("cannot read test directory {}: {e}", dir.display()))?
    {
        let path = entry
            .map_err(|e| format!("cannot inspect test directory {}: {e}", dir.display()))?
            .path();
        if path.is_dir() {
            collect_test_files(&path, files)?;
        } else if path.extension().and_then(|x| x.to_str()) == Some("zp")
            && path
                .file_stem()
                .and_then(|x| x.to_str())
                .map(|x| x.ends_with("_test"))
                .unwrap_or(false)
        {
            files.push(path);
        }
    }
    Ok(())
}

#[derive(Clone, Debug, Default)]
pub(crate) struct TestOptions {
    pub(crate) filter: Option<String>,
    pub(crate) fail_fast: bool,
    pub(crate) json: bool,
}

struct TestResult {
    path: PathBuf,
    passed: bool,
    error: Option<String>,
}

fn json_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn print_test_report(results: &[TestResult], skipped: usize, json: bool) {
    let passed = results.iter().filter(|result| result.passed).count();
    let failed = results.len() - passed;
    if json {
        let cases = results
            .iter()
            .map(|result| {
                let error = result
                    .error
                    .as_deref()
                    .map(|value| format!("\"{}\"", json_escape(value)))
                    .unwrap_or_else(|| "null".to_string());
                format!(
                    "{{\"path\":\"{}\",\"passed\":{},\"error\":{}}}",
                    json_escape(&result.path.display().to_string()),
                    result.passed,
                    error
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        println!(
            "{{\"passed\":{},\"failed\":{},\"skipped\":{},\"tests\":[{}]}}",
            passed, failed, skipped, cases
        );
    } else {
        println!(
            "test result: {} passed; {} failed; {} skipped",
            passed, failed, skipped
        );
    }
}

pub(crate) fn run_zap_tests(dir: &Path, options: &TestOptions) -> Result<usize, String> {
    let mut files = Vec::new();
    collect_test_files(dir, &mut files)?;
    files.sort();
    let selected = files
        .into_iter()
        .filter(|path| {
            options
                .filter
                .as_deref()
                .map(|filter| path.display().to_string().contains(filter))
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();
    if selected.is_empty() {
        return Err(match &options.filter {
            Some(filter) => format!(
                "no test files matched filter `{filter}` in {}",
                dir.display()
            ),
            None => format!("no *_test.zp files found in {}", dir.display()),
        });
    }

    let selected_count = selected.len();
    let mut results = Vec::new();
    for path in selected {
        let outcome = read_limited_text(&path, "test read")
            .and_then(|source| run(&source, path.parent().unwrap_or(Path::new("."))));
        match outcome {
            Ok(()) => {
                if !options.json {
                    println!("ok   {}", path.display());
                }
                results.push(TestResult {
                    path,
                    passed: true,
                    error: None,
                });
            }
            Err(error) => {
                if !options.json {
                    eprintln!("FAIL {}: {}", path.display(), error);
                }
                results.push(TestResult {
                    path,
                    passed: false,
                    error: Some(error),
                });
                if options.fail_fast {
                    break;
                }
            }
        }
    }

    let skipped = selected_count.saturating_sub(results.len());
    print_test_report(&results, skipped, options.json);
    let passed = results.iter().filter(|result| result.passed).count();
    if results.iter().any(|result| !result.passed) {
        Err(format!("{} test file(s) failed", results.len() - passed))
    } else {
        Ok(passed)
    }
}
