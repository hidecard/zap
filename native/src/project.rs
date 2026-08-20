use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use crate::registry::{
    cache_package, find_package_requirement, package_cache_path, read_index, verify_cached_package,
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
    validate_package_metadata(&text, "zap.toml")?;
    let main = manifest_value(&text, "main").unwrap_or_else(|| "main.zp".into());
    let main_path = dir.join(&main);
    if !main_path.is_file() {
        return Err(format!(
            "zap.toml: main file not found: {}",
            main_path.display()
        ));
    }
    validate_lockfile(dir, &text)?;
    let source = read_limited_text(&main_path, "source read")?;
    validate_function_signatures(&source, &main_path)?;
    validate_function_returns(&source, &main_path)?;
    validate_function_calls(&source, &main_path)?;
    Ok(format!("{name} {version} (main: {main})"))
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum DependencySpec {
    Requirement(String),
    LocalPath(String),
}

impl DependencySpec {
    fn lock_value(&self) -> String {
        match self {
            Self::Requirement(value) => format!("\"{value}\""),
            Self::LocalPath(path) => format!("{{ path = \"{path}\" }}"),
        }
    }
}

fn parse_dependencies(manifest: &str) -> Result<BTreeMap<String, DependencySpec>, String> {
    let mut dependencies = BTreeMap::new();
    let mut in_dependencies = false;
    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_dependencies = line == "[dependencies]";
            continue;
        }
        if !in_dependencies {
            continue;
        }
        let (name, value) = line
            .split_once('=')
            .ok_or_else(|| format!("zap.toml: invalid dependency entry `{line}`"))?;
        let name = name.trim();
        let raw_value = value.trim();
        if name.is_empty() || raw_value.is_empty() || name.contains(char::is_whitespace) {
            return Err(format!("zap.toml: invalid dependency entry `{line}`"));
        }
        let spec = if raw_value.starts_with('{') {
            let path = raw_value
                .strip_prefix('{')
                .and_then(|value| value.strip_suffix('}'))
                .and_then(|value| value.split_once("path"))
                .and_then(|(_, value)| value.split_once('='))
                .map(|(_, value)| value.trim().trim_matches('"').to_string())
                .filter(|path| !path.is_empty())
                .ok_or_else(|| format!("zap.toml: invalid local path dependency `{line}`"))?;
            DependencySpec::LocalPath(path)
        } else {
            let requirement = raw_value.trim_matches('"').to_string();
            if requirement.contains('\n') || requirement.is_empty() {
                return Err(format!("zap.toml: invalid dependency entry `{line}`"));
            }
            DependencySpec::Requirement(requirement)
        };
        if dependencies.insert(name.to_string(), spec).is_some() {
            return Err(format!("zap.toml: duplicate dependency `{name}`"));
        }
    }
    Ok(dependencies)
}

fn validate_package_metadata(manifest: &str, context: &str) -> Result<(), String> {
    let known = [
        "description",
        "authors",
        "license",
        "repository",
        "checksum",
    ];
    let mut in_package = false;
    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if !in_package {
            continue;
        }
        let Some((key, raw_value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        if !known.contains(&key) {
            continue;
        }
        let value = raw_value.trim();
        if value.is_empty() || value.contains(['\n', '\r']) {
            return Err(format!(
                "{context}: metadata field `{key}` must not be empty"
            ));
        }
        if key == "authors" {
            let valid_array = value.starts_with('[') && value.ends_with(']') && value.len() > 2;
            let valid_string = value.starts_with('"') && value.ends_with('"') && value.len() > 2;
            if !valid_array && !valid_string {
                return Err(format!(
                    "{context}: metadata field `authors` must be a non-empty string or array"
                ));
            }
        } else {
            let quoted = value.starts_with('"') && value.ends_with('"') && value.len() > 2;
            if !quoted {
                return Err(format!(
                    "{context}: metadata field `{key}` must be a non-empty string"
                ));
            }
        }
        if key == "checksum" {
            let checksum = value.trim_matches('"');
            if checksum.len() != 64 || !checksum.chars().all(|ch| ch.is_ascii_hexdigit()) {
                return Err(format!("{context}: metadata field `checksum` must be a 64-character hexadecimal SHA-256 value"));
            }
        }
    }
    Ok(())
}

fn canonical_lockfile(
    name: &str,
    version: &str,
    dependencies: &BTreeMap<String, DependencySpec>,
) -> String {
    let mut output = String::from("# This file is generated by Zap. Do not edit manually.\nlockfile_version = 1\n\n[package]\n");
    output.push_str(&format!(
        "name = \"{name}\"\nversion = \"{version}\"\n\n[dependencies]\n"
    ));
    for (dependency, requirement) in dependencies {
        output.push_str(&format!("{dependency} = {}\n", requirement.lock_value()));
    }
    output
}

fn validate_lockfile(dir: &Path, manifest: &str) -> Result<(), String> {
    let dependencies = parse_dependencies(manifest)?;
    validate_dependency_graph(dir, &dependencies)?;
    let lock_path = dir.join("zap.lock");
    if dependencies.is_empty() && !lock_path.exists() {
        return Ok(());
    }
    let name = manifest_value(manifest, "name").unwrap_or_default();
    let version = manifest_value(manifest, "version").unwrap_or_default();
    let expected = canonical_lockfile(&name, &version, &dependencies);
    let actual = fs::read_to_string(&lock_path)
        .map_err(|_| "zap.lock: missing lockfile; run `zap lock` to generate it".to_string())?;
    if actual != expected {
        return Err(
            "zap.lock: out of date or non-canonical; run `zap lock` to regenerate it".into(),
        );
    }
    Ok(())
}

pub(crate) fn add_dependency(dir: &Path, name: &str, requirement: &str) -> Result<String, String> {
    if name.is_empty() || name.contains(char::is_whitespace) || name.contains('=') {
        return Err(format!("invalid dependency name `{name}`"));
    }
    if requirement.is_empty() || requirement.contains('"') || requirement.contains('\n') {
        return Err("dependency requirement must be a non-empty single-line value".to_string());
    }
    let manifest_path = dir.join("zap.toml");
    let manifest = read_limited_text(&manifest_path, "manifest read")?;
    let mut lines: Vec<String> = manifest.lines().map(str::to_string).collect();
    let mut section_start = None;
    let mut section_end = lines.len();
    for (index, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed == "[dependencies]" {
            section_start = Some(index);
            continue;
        }
        if section_start.is_some() && trimmed.starts_with('[') {
            section_end = index;
            break;
        }
    }
    let entry = format!("{name} = \"{requirement}\"");
    match section_start {
        Some(start) => {
            let end = section_end;
            let mut dependencies = parse_dependencies(&manifest)?;
            if dependencies.contains_key(name) {
                return Err(format!("dependency already exists: `{name}`"));
            }
            dependencies.insert(
                name.to_string(),
                DependencySpec::Requirement(requirement.to_string()),
            );
            let mut replacement = vec!["[dependencies]".to_string()];
            for (dependency, value) in dependencies {
                replacement.push(format!("{dependency} = {}", value.lock_value()));
            }
            lines.splice(start..end, replacement);
        }
        None => {
            if !lines.is_empty() && lines.last().is_some_and(|line| !line.is_empty()) {
                lines.push(String::new());
            }
            lines.push("[dependencies]".to_string());
            lines.push(entry);
        }
    }
    let mut updated = lines.join("\n");
    updated.push('\n');
    fs::write(&manifest_path, updated)
        .map_err(|e| format!("zap.toml: cannot write manifest: {e}"))?;
    let lock_path = dir.join("zap.lock");
    if lock_path.exists() {
        fs::remove_file(lock_path)
            .map_err(|e| format!("zap.lock: cannot invalidate lockfile: {e}"))?;
    }
    Ok(format!("added dependency `{name}` = \"{requirement}\""))
}

fn validate_dependency_graph(
    root: &Path,
    dependencies: &BTreeMap<String, DependencySpec>,
) -> Result<(), String> {
    let mut active = Vec::new();
    let mut completed = HashSet::new();
    for (name, spec) in dependencies {
        visit_dependency(root, name, spec, &mut active, &mut completed)?;
    }
    Ok(())
}

fn visit_dependency(
    parent: &Path,
    name: &str,
    spec: &DependencySpec,
    active: &mut Vec<(String, PathBuf)>,
    completed: &mut HashSet<PathBuf>,
) -> Result<(), String> {
    let DependencySpec::LocalPath(raw_path) = spec else {
        return Ok(());
    };
    let dependency_dir = parent.join(raw_path);
    if !dependency_dir.is_dir() {
        return Err(format!(
            "dependency `{name}`: local path does not exist: {}",
            dependency_dir.display()
        ));
    }
    let identity = fs::canonicalize(&dependency_dir).map_err(|e| {
        format!(
            "dependency `{name}`: cannot canonicalize local path {}: {e}",
            dependency_dir.display()
        )
    })?;
    if let Some(position) = active.iter().position(|(_, path)| path == &identity) {
        let mut cycle = active[position..]
            .iter()
            .map(|(package, _)| package.clone())
            .collect::<Vec<_>>();
        cycle.push(name.to_string());
        return Err(format!("dependency cycle detected: {}", cycle.join(" -> ")));
    }
    if completed.contains(&identity) {
        return Ok(());
    }
    let manifest_path = dependency_dir.join("zap.toml");
    if !manifest_path.is_file() {
        return Err(format!(
            "dependency `{name}`: missing zap.toml at {}",
            dependency_dir.display()
        ));
    }
    let text = read_limited_text(&manifest_path, "local dependency manifest read")?;
    validate_package_metadata(&text, &format!("dependency `{name}`"))?;
    let package_name = manifest_value(&text, "name")
        .ok_or_else(|| format!("dependency `{name}`: local zap.toml is missing package name"))?;
    manifest_value(&text, "version")
        .ok_or_else(|| format!("dependency `{name}`: local zap.toml is missing package version"))?;
    let nested = parse_dependencies(&text)?;
    active.push((package_name, identity.clone()));
    for (nested_name, nested_spec) in &nested {
        visit_dependency(&dependency_dir, nested_name, nested_spec, active, completed)?;
    }
    active.pop();
    completed.insert(identity);
    Ok(())
}

pub(crate) fn write_lockfile(dir: &Path) -> Result<String, String> {
    let manifest_path = dir.join("zap.toml");
    let manifest = read_limited_text(&manifest_path, "manifest read")?;
    validate_package_metadata(&manifest, "zap.toml")?;
    let name =
        manifest_value(&manifest, "name").ok_or("zap.toml: missing package name".to_string())?;
    let version = manifest_value(&manifest, "version")
        .ok_or("zap.toml: missing package version".to_string())?;
    let dependencies = parse_dependencies(&manifest)?;
    validate_dependency_graph(dir, &dependencies)?;
    let content = canonical_lockfile(&name, &version, &dependencies);
    fs::write(dir.join("zap.lock"), content)
        .map_err(|e| format!("zap.lock: cannot write lockfile: {e}"))?;
    Ok(format!(
        "wrote zap.lock with {} dependencies",
        dependencies.len()
    ))
}

pub(crate) fn install_dependencies(dir: &Path) -> Result<String, String> {
    let manifest_path = dir.join("zap.toml");
    let manifest = read_limited_text(&manifest_path, "manifest read")?;
    manifest_value(&manifest, "name").ok_or("zap.toml: missing package name".to_string())?;
    manifest_value(&manifest, "version").ok_or("zap.toml: missing package version".to_string())?;
    let dependencies = parse_dependencies(&manifest)?;
    validate_lockfile(dir, &manifest)?;
    resolve_registry_dependencies(dir, &dependencies, false)?;
    Ok(format!(
        "installed {} locked dependencies",
        dependencies.len()
    ))
}

pub(crate) fn update_dependencies(dir: &Path) -> Result<String, String> {
    let manifest_path = dir.join("zap.toml");
    let manifest = read_limited_text(&manifest_path, "manifest read")?;
    validate_package_metadata(&manifest, "zap.toml")?;
    let name =
        manifest_value(&manifest, "name").ok_or("zap.toml: missing package name".to_string())?;
    let version = manifest_value(&manifest, "version")
        .ok_or("zap.toml: missing package version".to_string())?;
    let dependencies = parse_dependencies(&manifest)?;
    validate_dependency_graph(dir, &dependencies)?;
    resolve_registry_dependencies(dir, &dependencies, true)?;
    let content = canonical_lockfile(&name, &version, &dependencies);
    fs::write(dir.join("zap.lock"), content)
        .map_err(|e| format!("zap.lock: cannot write lockfile: {e}"))?;
    Ok(format!(
        "updated zap.lock with {} dependencies",
        dependencies.len()
    ))
}

fn resolve_registry_dependencies(
    project_dir: &Path,
    dependencies: &BTreeMap<String, DependencySpec>,
    update: bool,
) -> Result<(), String> {
    let Some(index_path) = std::env::var_os("ZAP_REGISTRY_INDEX") else {
        return Ok(());
    };
    let index_path = PathBuf::from(index_path);
    let index = read_index(&index_path)?;
    let cache_root = std::env::var_os("ZAP_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| project_dir.join(".zap/cache"));
    let offline = std::env::var_os("ZAP_OFFLINE").is_some();
    for (name, spec) in dependencies {
        let DependencySpec::Requirement(version) = spec else {
            continue;
        };
        let package = find_package_requirement(&index, name, version)?;
        let cached = package_cache_path(&cache_root, &package);
        if cached.is_file() {
            verify_cached_package(&cached, &package)?;
            continue;
        }
        if offline {
            return Err(format!(
                "registry package is not cached in offline mode: {name} {version}"
            ));
        }
        let source = package
            .source
            .strip_prefix("file://")
            .unwrap_or(&package.source);
        let source_path = Path::new(source);
        let source_path = if source_path.is_absolute() {
            source_path.to_path_buf()
        } else {
            index_path
                .parent()
                .unwrap_or(Path::new("."))
                .join(source_path)
        };
        cache_package(&source_path, &cache_root, &package)?;
        if update && !cached.is_file() {
            verify_cached_package(&cached, &package)?;
        }
    }
    Ok(())
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
