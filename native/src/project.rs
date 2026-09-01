use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use crate::ast::{parse_program, Stmt};
use crate::database::validate_database_manifest;
use crate::registry::{
    cache_package, cache_package_source_with_credentials, load_registry_credentials,
    package_cache_path, read_index, read_index_source_with_credentials, resolve_dependency_graph,
    verify_cached_package, version_satisfies_requirement, RegistryPackage,
};
use crate::value::{Param, StaticSignature};

use super::{
    manifest_value, read_limited_text, run, validate_function_calls, validate_function_returns,
    validate_function_signatures,
};

/// Security validation for path operations to prevent directory traversal attacks
pub(crate) fn validate_path_security(path: &Path, project_root: &Path) -> Result<(), String> {
    // Reject absolute paths
    if path.is_absolute() {
        return Err(format!(
            "security: absolute path not allowed: {}",
            path.display()
        ));
    }

    // Normalize the path and check for traversal components
    let normalized = path.canonicalize().map_err(|e| {
        format!(
            "security: cannot canonicalize path {}: {}",
            path.display(),
            e
        )
    })?;

    // Check if the normalized path is within project root
    let canonical_root = project_root.canonicalize().map_err(|e| {
        format!(
            "security: cannot canonicalize project root {}: {}",
            project_root.display(),
            e
        )
    })?;

    if !normalized.starts_with(&canonical_root) {
        return Err(format!(
            "security: path {} escapes project root {}",
            normalized.display(),
            canonical_root.display()
        ));
    }

    // Check for symlink components that could escape project boundary
    check_symlink_security(&normalized, &canonical_root)?;

    Ok(())
}

/// Check that symlinks don't escape the project boundary
fn check_symlink_security(path: &Path, project_root: &Path) -> Result<(), String> {
    let mut current = path;

    while let Some(parent) = current.parent() {
        if parent == project_root {
            break;
        }

        if let Ok(metadata) = fs::symlink_metadata(current) {
            if metadata.is_symlink() {
                let target = fs::read_link(current).map_err(|e| {
                    format!("security: cannot read symlink {}: {}", current.display(), e)
                })?;

                // Check if symlink target is absolute and outside project
                if Path::new(&target).is_absolute() {
                    let canonical_target = Path::new(&target).canonicalize().map_err(|e| {
                        format!(
                            "security: cannot canonicalize symlink target {}: {}",
                            target.display(),
                            e
                        )
                    })?;

                    if !canonical_target.starts_with(project_root) {
                        return Err(format!(
                            "security: symlink {} points outside project root to {}",
                            current.display(),
                            canonical_target.display()
                        ));
                    }
                }
            }
        }

        current = parent;
    }

    Ok(())
}

/// Validate module path security to prevent escaping module boundaries
pub(crate) fn validate_module_path_security(
    module_path: &Path,
    project_root: &Path,
) -> Result<(), String> {
    validate_path_security(module_path, project_root)?;

    // Additional module-specific checks
    if module_path.components().any(|c| c.as_os_str() == "..") {
        return Err(format!(
            "security: module path contains parent directory component: {}",
            module_path.display()
        ));
    }

    Ok(())
}

/// Check for directory traversal attempts in path strings
pub(crate) fn detect_path_traversal(path: &str) -> bool {
    path.contains("..") || path.starts_with('/') || path.starts_with('\\')
}

/// Sanitize a path string to prevent traversal attacks
pub(crate) fn sanitize_path(path: &str) -> Result<String, String> {
    if detect_path_traversal(path) {
        return Err(format!("security: path traversal detected in: {}", path));
    }

    // Remove any suspicious patterns
    let normalized = path.replace("\\", "/").replace("//", "/");
    let sanitized = normalized.trim_matches('/');

    Ok(sanitized.to_string())
}

pub(crate) fn validate_project(dir: &Path) -> Result<String, String> {
    validate_project_with_lock_mode(dir, false)
}

pub(crate) fn validate_project_locked(dir: &Path) -> Result<String, String> {
    validate_project_with_lock_mode(dir, true)
}

fn validate_project_with_lock_mode(dir: &Path, require_lockfile: bool) -> Result<String, String> {
    let manifest = dir.join("zap.toml");
    let text = read_limited_text(&manifest, "manifest read")?;
    validate_manifest_syntax(&text)?;
    validate_database_manifest(&text)?;
    let name = manifest_value(&text, "name").ok_or("zap.toml: missing package name".to_string())?;
    let version =
        manifest_value(&text, "version").ok_or("zap.toml: missing package version".to_string())?;
    validate_package_metadata(&text, "zap.toml")?;
    validate_web_manifest(dir, &text)?;
    validate_frontend_manifest(dir, &text)?;
    validate_module_manifest(dir, &text)?;
    let main = manifest_value(&text, "main").unwrap_or_else(|| "main.zp".into());
    let main_path = dir.join(&main);
    if !main_path.is_file() {
        return Err(format!(
            "zap.toml: main file not found: {}",
            main_path.display()
        ));
    }
    validate_lockfile(dir, &text, require_lockfile)?;
    let source = read_limited_text(&main_path, "source read")?;
    validate_explicit_imports(dir, &text, &source, &main_path)?;
    validate_function_signatures(&source, &main_path)?;
    validate_function_returns(&source, &main_path)?;
    let imported_signatures = collect_imported_signatures(dir, &text, &main_path, &source)?;
    validate_function_calls(&source, &main_path, &imported_signatures)?;
    Ok(format!("{name} {version} (main: {main})"))
}

fn collect_imported_signatures(
    dir: &Path,
    manifest: &str,
    source_path: &Path,
    source: &str,
) -> Result<HashMap<String, StaticSignature>, String> {
    let has_import = source.lines().any(|line| {
        let line = line.trim_start();
        line == "import" || line.starts_with("import ")
    });
    if !has_import {
        return Ok(HashMap::new());
    }
    let root_dir = dir.join(module_root(manifest));
    let mut visited = HashSet::new();
    let mut signatures = HashMap::new();
    collect_module_signatures(
        &root_dir,
        source_path,
        source,
        &mut visited,
        &mut signatures,
    )?;
    Ok(signatures)
}

fn collect_module_signatures(
    root_dir: &Path,
    source_path: &Path,
    source: &str,
    visited: &mut HashSet<PathBuf>,
    signatures: &mut HashMap<String, StaticSignature>,
) -> Result<(), String> {
    if !visited.insert(source_path.to_path_buf()) {
        return Ok(());
    }
    let program = parse_program(source).map_err(|error| {
        format!(
            "{}: imported module syntax error: {error}",
            source_path.display()
        )
    })?;
    for statement in program.statements {
        match statement.node {
            Stmt::Function {
                name,
                type_params,
                params,
                return_type,
                exported: true,
                ..
            } => {
                signatures.insert(
                    name,
                    StaticSignature {
                        params: params
                            .into_iter()
                            .map(|(name, annotation, default)| Param {
                                name,
                                annotation,
                                default,
                            })
                            .collect(),
                        type_params,
                        return_annotation: return_type,
                    },
                );
            }
            Stmt::Import {
                path,
                explicit: true,
                ..
            } => {
                let relative = import_target_path(&path)
                    .map_err(|error| format!("{}: {error}", source_path.display()))?;
                let target = root_dir.join(relative);
                if !target.is_file() {
                    return Err(format!(
                        "{}: imported module not found: {}",
                        source_path.display(),
                        target.display()
                    ));
                }
                let target_source = read_limited_text(&target, "module source read")?;
                collect_module_signatures(root_dir, &target, &target_source, visited, signatures)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_explicit_imports(
    dir: &Path,
    manifest: &str,
    source: &str,
    source_path: &Path,
) -> Result<(), String> {
    let has_explicit_syntax = source.lines().any(|line| {
        let line = line.trim_start();
        line == "module"
            || line.starts_with("module ")
            || line == "import"
            || line.starts_with("import ")
    });
    if !has_explicit_syntax {
        return Ok(());
    }
    let root_dir = dir.join(module_root(manifest));
    let mut states = BTreeMap::new();
    let mut stack = Vec::new();
    validate_module_graph(&root_dir, source_path, source, &mut states, &mut stack)
}

fn validate_module_graph(
    root_dir: &Path,
    source_path: &Path,
    source: &str,
    states: &mut BTreeMap<PathBuf, u8>,
    stack: &mut Vec<PathBuf>,
) -> Result<(), String> {
    let key = source_path.to_path_buf();
    match states.get(&key).copied() {
        Some(2) => return Ok(()),
        Some(1) => {
            let start = stack.iter().position(|path| path == &key).unwrap_or(0);
            let mut cycle = stack[start..]
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>();
            cycle.push(key.display().to_string());
            return Err(format!(
                "circular module dependency: {}",
                cycle.join(" -> ")
            ));
        }
        _ => {}
    }
    states.insert(key.clone(), 1);
    stack.push(key.clone());

    let result = (|| {
        let program = parse_program(source).map_err(|error| {
            format!(
                "{}: explicit module/import syntax error: {error}",
                source_path.display()
            )
        })?;
        let mut module_name = None;
        for statement in &program.statements {
            match &statement.node {
                Stmt::Module { name } => {
                    if module_name.replace(name.clone()).is_some() {
                        return Err(format!(
                            "{}: duplicate module declaration",
                            source_path.display()
                        ));
                    }
                }
                Stmt::Import {
                    path,
                    explicit: true,
                    ..
                } => {
                    let relative = import_target_path(path)
                        .map_err(|error| format!("{}: {error}", source_path.display()))?;
                    let target = root_dir.join(relative);
                    if !target.is_file() {
                        return Err(format!(
                            "{}: imported module not found: {}",
                            source_path.display(),
                            target.display()
                        ));
                    }
                    let target_source = read_limited_text(&target, "module source read")?;
                    let target_has_explicit_syntax = target_source.lines().any(|line| {
                        let line = line.trim_start();
                        line == "module"
                            || line.starts_with("module ")
                            || line == "import"
                            || line.starts_with("import ")
                    });
                    if target_has_explicit_syntax {
                        validate_module_graph(root_dir, &target, &target_source, states, stack)?;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    })();

    stack.pop();
    if result.is_ok() {
        states.insert(key, 2);
    }
    result
}

fn module_root(manifest: &str) -> PathBuf {
    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if let Some(value) = line.strip_prefix("root =") {
            let value = value.trim().trim_matches('"');
            if !value.is_empty() {
                return PathBuf::from(value);
            }
        }
    }
    PathBuf::from(".")
}

fn import_target_path(path: &str) -> Result<PathBuf, String> {
    let normalized = path.trim().trim_matches('"');
    if normalized.is_empty() || normalized.starts_with('/') || normalized.contains('\\') {
        return Err(format!("invalid explicit import path `{path}`"));
    }
    let mut relative = PathBuf::new();
    for component in normalized.split('.') {
        if component.is_empty() || component == ".." || component == "." {
            return Err(format!("invalid explicit import path `{path}`"));
        }
        relative.push(component);
    }
    if relative.extension().is_none() {
        relative.set_extension("zp");
    }
    if relative
        .components()
        .any(|component| component == std::path::Component::ParentDir)
    {
        return Err(format!("invalid explicit import path `{path}`"));
    }
    Ok(relative)
}

pub(crate) fn resolve_relative_path(
    base_path: &Path,
    relative_spec: &str,
) -> Result<PathBuf, String> {
    let normalized = relative_spec.trim().trim_matches('"');
    if normalized.is_empty() {
        return Ok(base_path.to_path_buf());
    }

    let base_parent = base_path.parent().unwrap_or_else(|| Path::new("."));
    let mut result = base_parent.to_path_buf();

    for component in normalized.split("..") {
        if !result.pop() {
            return Err(format!(
                "relative path goes beyond project root: {relative_spec}"
            ));
        }
    }

    let remaining = normalized.split("..").last().unwrap_or("");
    if !remaining.is_empty() {
        for component in remaining.split('.') {
            if component.is_empty() || component == "." {
                continue;
            }
            result.push(component);
        }
    }

    Ok(result)
}

pub(crate) fn validate_package_name(name: &str) -> Result<(), String> {
    if name.is_empty() {
        return Err("package name cannot be empty".to_string());
    }

    if !name
        .chars()
        .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
    {
        return Err(format!(
            "package name can only contain alphanumeric characters, underscores, and hyphens: {name}"
        ));
    }

    if name.chars().next().map(|c| c.is_digit(10)).unwrap_or(false) {
        return Err("package name cannot start with a digit".to_string());
    }

    Ok(())
}

pub(crate) fn standard_module_resolution(
    module_name: &str,
    current_path: &Path,
    root_dir: &Path,
) -> Result<PathBuf, String> {
    validate_package_name(module_name)?;

    // Try standard directories
    let standard_dirs = ["modules", "lib", "src"];

    for dir in &standard_dirs {
        let candidate = root_dir.join(dir).join(format!("{module_name}.zp"));
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    // Try relative to current file
    let current_dir = current_path.parent().unwrap_or_else(|| Path::new("."));
    let relative_candidate = current_dir.join(format!("{module_name}.zp"));
    if relative_candidate.is_file() {
        return Ok(relative_candidate);
    }

    Err(format!("module not found: {module_name}"))
}

fn validate_module_manifest(dir: &Path, manifest: &str) -> Result<(), String> {
    let mut in_module = false;
    let mut root = None;
    let mut entries = None;
    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_module = line == "[module]";
            continue;
        }
        if !in_module {
            continue;
        }
        let Some((key, raw_value)) = line.split_once('=') else {
            return Err(format!("zap.toml: invalid module entry `{line}`"));
        };
        let key = key.trim();
        let value = raw_value.trim();
        match key {
            "root" => {
                let value = value.trim_matches('"');
                if value.is_empty() || value.contains(['\n', '\r']) {
                    return Err("zap.toml: module root must be a non-empty path".into());
                }
                root = Some(value.to_string());
            }
            "entries" => {
                let body = value
                    .strip_prefix('[')
                    .and_then(|value| value.strip_suffix(']'))
                    .ok_or("zap.toml: module entries must be an array".to_string())?;
                let mut parsed = Vec::new();
                for item in body.split(',') {
                    let item = item.trim();
                    if item.is_empty() {
                        continue;
                    }
                    let path = item.trim_matches('"');
                    if path.is_empty() {
                        return Err("zap.toml: module entries cannot contain empty paths".into());
                    }
                    parsed.push(path.to_string());
                }
                entries = Some(parsed);
            }
            _ => return Err(format!("zap.toml: unknown module field `{key}`")),
        }
    }
    let Some(root) = root else {
        return Ok(());
    };
    let root_path = Path::new(&root);
    if root_path.is_absolute()
        || root_path
            .components()
            .any(|component| component == std::path::Component::ParentDir)
    {
        return Err("zap.toml: module root must be a relative path without `..`".into());
    }
    let root_dir = dir.join(root_path);
    if !root_dir.is_dir() {
        return Err(format!(
            "zap.toml: module root not found: {}",
            root_dir.display()
        ));
    }
    let mut seen = HashSet::new();
    for entry in entries.unwrap_or_default() {
        let entry_path = Path::new(&entry);
        if entry_path.is_absolute()
            || entry_path
                .components()
                .any(|component| component == std::path::Component::ParentDir)
            || entry_path.extension().and_then(|value| value.to_str()) != Some("zp")
        {
            return Err(format!("zap.toml: invalid module entry `{entry}`"));
        }
        if !seen.insert(entry.clone()) {
            return Err(format!("zap.toml: duplicate module entry `{entry}`"));
        }
        let path = root_dir.join(entry_path);
        if !path.is_file() {
            return Err(format!(
                "zap.toml: module entry not found: {}",
                path.display()
            ));
        }
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct LockedRegistryPackage {
    name: String,
    version: String,
    source: String,
    checksum: String,
}

impl LockedRegistryPackage {
    fn from_registry(package: &RegistryPackage) -> Self {
        Self {
            name: package.name.clone(),
            version: package.version.clone(),
            source: package.source.clone(),
            checksum: package.checksum.clone(),
        }
    }

    fn into_registry(self) -> RegistryPackage {
        RegistryPackage {
            name: self.name,
            version: self.version,
            source: self.source,
            checksum: self.checksum,
            yanked: false,
            dependencies: BTreeMap::new(),
        }
    }
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
        validate_package_name(name).map_err(|error| format!("zap.toml: {error}"))?;
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

fn validate_web_manifest(dir: &Path, manifest: &str) -> Result<(), String> {
    let known = [
        "routes",
        "models",
        "middleware",
        "migrations",
        "admin",
        "server",
        "assets",
        "serialization",
    ];
    let mut in_web = false;
    let mut saw_web = false;
    let mut values = BTreeMap::new();
    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_web = line == "[web]";
            saw_web |= in_web;
            continue;
        }
        if !in_web {
            continue;
        }
        let Some((key, raw_value)) = line.split_once('=') else {
            return Err(format!("zap.toml: invalid Web entry `{line}`"));
        };
        let key = key.trim();
        let value = raw_value.trim();
        if !known.contains(&key) {
            return Err(format!("zap.toml: unknown Web field `{key}`"));
        }
        if values.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!("zap.toml: duplicate Web field `{key}`"));
        }
        if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
            return Err(format!(
                "zap.toml: Web field `{key}` must be a quoted string"
            ));
        }
        if key != "serialization" {
            let path = value.trim_matches('"');
            let candidate = Path::new(path);
            if path.is_empty()
                || candidate.is_absolute()
                || candidate
                    .components()
                    .any(|component| component == std::path::Component::ParentDir)
            {
                return Err(format!(
                    "zap.toml: Web field `{key}` must be a safe relative path"
                ));
            }
        } else if value.trim_matches('"') != "json-by-default" {
            return Err(
                "zap.toml: Web serialization must be `json-by-default` for the first Web profile"
                    .into(),
            );
        }
    }
    if !saw_web {
        return Ok(());
    }
    for key in [
        "routes",
        "models",
        "middleware",
        "migrations",
        "admin",
        "server",
        "assets",
    ] {
        let Some(raw_value) = values.get(key) else {
            return Err(format!("zap.toml: Web field `{key}` is required"));
        };
        let path = dir.join(raw_value.trim_matches('"'));
        let valid = if key == "models" || key == "migrations" || key == "assets" {
            path.is_dir()
        } else {
            path.is_file()
        };
        if !valid {
            return Err(format!(
                "zap.toml: Web {key} path not found: {}",
                path.display()
            ));
        }
    }
    Ok(())
}

fn validate_frontend_manifest(dir: &Path, manifest: &str) -> Result<(), String> {
    let mut in_frontend = false;
    let mut saw_frontend = false;
    let mut values = BTreeMap::new();
    let allowed = ["framework", "output", "spa_fallback"];
    for raw_line in manifest.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_frontend = line == "[frontend]";
            saw_frontend |= in_frontend;
            continue;
        }
        if !in_frontend {
            continue;
        }
        let Some((key, raw_value)) = line.split_once('=') else {
            return Err(format!("zap.toml: invalid Frontend entry `{line}`"));
        };
        let key = key.trim();
        let value = raw_value.trim();
        if !allowed.contains(&key) {
            return Err(format!("zap.toml: unknown Frontend field `{key}`"));
        }
        if values.insert(key.to_string(), value.to_string()).is_some() {
            return Err(format!("zap.toml: duplicate Frontend field `{key}`"));
        }
        if value.len() < 2 || !value.starts_with('"') || !value.ends_with('"') {
            return Err(format!(
                "zap.toml: Frontend field `{key}` must be a quoted string"
            ));
        }
        if key != "framework" {
            let path = value.trim_matches('"');
            let candidate = Path::new(path);
            if path.is_empty()
                || candidate.is_absolute()
                || candidate
                    .components()
                    .any(|component| component == std::path::Component::ParentDir)
            {
                return Err(format!(
                    "zap.toml: Frontend field `{key}` must be a safe relative path"
                ));
            }
        }
    }
    if !saw_frontend {
        return Ok(());
    }
    if !manifest.lines().any(|line| line.trim() == "[web]") {
        return Err("zap.toml: [frontend] requires a [web] section".into());
    }
    let framework = values
        .get("framework")
        .ok_or("zap.toml: Frontend field `framework` is required".to_string())?
        .trim_matches('"');
    if !matches!(framework, "plain" | "react" | "vue" | "svelte" | "other") {
        return Err(
            "zap.toml: Frontend framework must be plain, react, vue, svelte, or other".into(),
        );
    }
    let output = values
        .get("output")
        .ok_or("zap.toml: Frontend field `output` is required".to_string())?
        .trim_matches('"');
    let output_path = dir.join(output);
    if !output_path.is_dir() {
        return Err(format!(
            "zap.toml: Frontend output directory not found: {}",
            output_path.display()
        ));
    }
    let fallback = values
        .get("spa_fallback")
        .ok_or("zap.toml: Frontend field `spa_fallback` is required".to_string())?
        .trim_matches('"');
    if !output_path.join(fallback).is_file() {
        return Err(format!(
            "zap.toml: Frontend SPA fallback not found: {}",
            output_path.join(fallback).display()
        ));
    }
    Ok(())
}
fn validate_manifest_syntax(manifest: &str) -> Result<(), String> {
    let allowed_sections = [
        "package",
        "dependencies",
        "module",
        "web",
        "frontend",
        "database",
    ];
    let mut section = "package";
    let mut keys = HashSet::new();
    for (index, raw_line) in manifest.lines().enumerate() {
        let line_number = index + 1;
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            if !line.ends_with(']') {
                return Err(format!(
                    "zap.toml: unterminated section at line {line_number}"
                ));
            }
            section = line.trim_start_matches('[').trim_end_matches(']');
            if !allowed_sections.contains(&section) {
                return Err(format!("zap.toml: unknown section `[{section}]`"));
            }
            keys.clear();
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(format!("zap.toml: invalid entry at line {line_number}"));
        };
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() {
            return Err(format!("zap.toml: invalid entry at line {line_number}"));
        }
        if !keys.insert(key.to_string()) {
            return Err(format!("zap.toml: duplicate key `{key}` in [{section}]"));
        }
        let quote_count = value.chars().filter(|character| *character == '"').count();
        if quote_count % 2 != 0 {
            return Err(format!(
                "zap.toml: unterminated quote at line {line_number}"
            ));
        }
    }
    Ok(())
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

fn toml_escape(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
}

fn canonical_lockfile(
    name: &str,
    version: &str,
    dependencies: &BTreeMap<String, DependencySpec>,
    resolved: &[LockedRegistryPackage],
) -> String {
    let lockfile_version = if resolved.is_empty() { 1 } else { 2 };
    let mut output = format!(
        "# This file is generated by Zap. Do not edit manually.\nlockfile_version = {lockfile_version}\n\n[package]\n"
    );
    output.push_str(&format!(
        "name = \"{}\"\nversion = \"{}\"\n\n[dependencies]\n",
        toml_escape(name),
        toml_escape(version)
    ));
    for (dependency, requirement) in dependencies {
        output.push_str(&format!("{dependency} = {}\n", requirement.lock_value()));
    }
    if !resolved.is_empty() {
        output.push_str("\n[resolved]\n");
        for package in resolved {
            output.push_str(&format!(
                "{}.version = \"{}\"\n{}.source = \"{}\"\n{}.checksum = \"{}\"\n",
                package.name,
                toml_escape(&package.version),
                package.name,
                toml_escape(&package.source),
                package.name,
                toml_escape(&package.checksum)
            ));
        }
    }
    output
}

fn parse_lockfile_quoted(value: &str, context: &str) -> Result<String, String> {
    let value = value.trim();
    if !value.starts_with('"') || !value.ends_with('"') || value.len() < 2 {
        return Err(format!("zap.lock: {context} must be a quoted string"));
    }
    let body = &value[1..value.len() - 1];
    let mut output = String::new();
    let mut escaped = false;
    for character in body.chars() {
        if escaped {
            match character {
                'n' => output.push('\n'),
                'r' => output.push('\r'),
                '"' => output.push('"'),
                '\\' => output.push('\\'),
                _ => return Err(format!("zap.lock: invalid escape in {context}")),
            }
            escaped = false;
        } else if character == '\\' {
            escaped = true;
        } else {
            output.push(character);
        }
    }
    if escaped {
        return Err(format!("zap.lock: invalid escape in {context}"));
    }
    Ok(output)
}

pub(crate) fn parse_resolved_lockfile(text: &str) -> Result<Vec<LockedRegistryPackage>, String> {
    let version = text
        .lines()
        .find_map(|line| line.trim().strip_prefix("lockfile_version = "))
        .ok_or("zap.lock: missing lockfile_version".to_string())?
        .parse::<u32>()
        .map_err(|_| "zap.lock: invalid lockfile_version".to_string())?;
    if version == 1 {
        return Ok(Vec::new());
    }
    if version != 2 {
        return Err(format!("zap.lock: unsupported lockfile_version {version}"));
    }

    let mut in_resolved = false;
    let mut entries: BTreeMap<String, [Option<String>; 3]> = BTreeMap::new();
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line.starts_with('[') {
            in_resolved = line == "[resolved]";
            continue;
        }
        if !in_resolved {
            continue;
        }
        let (key, raw_value) = line
            .split_once('=')
            .ok_or_else(|| format!("zap.lock: invalid resolved entry `{line}`"))?;
        let (name, field) = key
            .trim()
            .split_once('.')
            .ok_or_else(|| format!("zap.lock: invalid resolved key `{}`", key.trim()))?;
        validate_package_name(name).map_err(|error| format!("zap.lock: {error}"))?;
        let index = match field.trim() {
            "version" => 0,
            "source" => 1,
            "checksum" => 2,
            other => return Err(format!("zap.lock: unknown resolved field `{other}`")),
        };
        let value = parse_lockfile_quoted(raw_value, &format!("{name}.{field}"))?;
        let entry = entries
            .entry(name.to_string())
            .or_insert([None, None, None]);
        if entry[index].replace(value).is_some() {
            return Err(format!(
                "zap.lock: duplicate resolved field `{name}.{field}`"
            ));
        }
    }

    let mut resolved = Vec::new();
    for (name, fields) in entries {
        let version = fields[0]
            .clone()
            .ok_or_else(|| format!("zap.lock: resolved package `{name}` is missing version"))?;
        let source = fields[1]
            .clone()
            .ok_or_else(|| format!("zap.lock: resolved package `{name}` is missing source"))?;
        let checksum = fields[2]
            .clone()
            .ok_or_else(|| format!("zap.lock: resolved package `{name}` is missing checksum"))?;
        if source.is_empty() {
            return Err(format!(
                "zap.lock: resolved package `{name}` has empty source"
            ));
        }
        if checksum.len() != 64
            || !checksum
                .chars()
                .all(|character| character.is_ascii_hexdigit())
        {
            return Err(format!(
                "zap.lock: resolved package `{name}` has invalid checksum"
            ));
        }
        resolved.push(LockedRegistryPackage {
            name,
            version,
            source,
            checksum,
        });
    }
    if resolved.is_empty() {
        return Err("zap.lock: version 2 requires a resolved package section".into());
    }
    Ok(resolved)
}

fn validate_lockfile(
    dir: &Path,
    manifest: &str,
    require_lockfile: bool,
) -> Result<Vec<LockedRegistryPackage>, String> {
    let dependencies = parse_dependencies(manifest)?;
    validate_dependency_graph(dir, &dependencies)?;
    let lock_path = dir.join("zap.lock");
    if dependencies.is_empty() && !lock_path.exists() {
        if require_lockfile {
            return Err("zap.lock: missing lockfile; run `zap lock` to generate it".into());
        }
        return Ok(Vec::new());
    }
    let name = manifest_value(manifest, "name").unwrap_or_default();
    let version = manifest_value(manifest, "version").unwrap_or_default();
    let actual = fs::read_to_string(&lock_path)
        .map_err(|_| "zap.lock: missing lockfile; run `zap lock` to generate it".to_string())?;
    let resolved = parse_resolved_lockfile(&actual).map_err(|_| {
        "zap.lock: out of date or non-canonical; run `zap lock` to regenerate it".to_string()
    })?;
    let expected = canonical_lockfile(&name, &version, &dependencies, &resolved);
    if actual != expected {
        return Err(
            "zap.lock: out of date or non-canonical; run `zap lock` to regenerate it".into(),
        );
    }
    Ok(resolved)
}

pub(crate) fn registry_packages_from_lockfile(dir: &Path) -> Result<Vec<RegistryPackage>, String> {
    let manifest = fs::read_to_string(dir.join("zap.toml"))
        .map_err(|_| "zap.toml: missing manifest".to_string())?;
    validate_lockfile(dir, &manifest, false).map(|packages| {
        packages
            .into_iter()
            .map(LockedRegistryPackage::into_registry)
            .collect()
    })
}

pub(crate) fn add_dependency(dir: &Path, name: &str, requirement: &str) -> Result<String, String> {
    validate_package_name(name).map_err(|_| format!("invalid dependency name `{name}`"))?;
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
    let resolved = resolve_registry_dependencies(dir, &dependencies, true, None)?;
    let content = canonical_lockfile(&name, &version, &dependencies, &resolved);
    fs::write(dir.join("zap.lock"), content)
        .map_err(|e| format!("zap.lock: cannot write lockfile: {e}"))?;
    Ok(format!(
        "wrote zap.lock with {} dependencies",
        dependencies.len()
    ))
}

pub(crate) fn migrate_lockfile(dir: &Path) -> Result<String, String> {
    let manifest_path = dir.join("zap.toml");
    let manifest = read_limited_text(&manifest_path, "manifest read")?;
    validate_package_metadata(&manifest, "zap.toml")?;
    let name =
        manifest_value(&manifest, "name").ok_or("zap.toml: missing package name".to_string())?;
    let version = manifest_value(&manifest, "version")
        .ok_or("zap.toml: missing package version".to_string())?;
    let dependencies = parse_dependencies(&manifest)?;
    validate_dependency_graph(dir, &dependencies)?;
    let lock_path = dir.join("zap.lock");
    let existing = fs::read_to_string(&lock_path)
        .map_err(|_| "zap.lock: missing lockfile; run `zap lock` first".to_string())?;
    let lockfile_version = existing
        .lines()
        .find_map(|line| line.trim().strip_prefix("lockfile_version = "))
        .and_then(|value| value.trim().parse::<u32>().ok())
        .ok_or_else(|| "zap.lock: missing or invalid lockfile_version".to_string())?;
    if lockfile_version >= 2 {
        return Ok("zap.lock is already at lockfile version 2".to_string());
    }
    if lockfile_version != 1 {
        return Err(format!(
            "zap.lock: unsupported lockfile version {lockfile_version}; upgrade Zap"
        ));
    }
    let has_registry_dependencies = dependencies
        .values()
        .any(|dependency| matches!(dependency, DependencySpec::Requirement(_)));
    if !has_registry_dependencies {
        return Ok("zap.lock does not require migration; it has no registry dependencies".into());
    }
    if std::env::var_os("ZAP_REGISTRY_INDEX").is_none() {
        return Err(
            "zap.lock: legacy registry lockfile requires migration; set ZAP_REGISTRY_INDEX and run `zap lock-migrate`".into(),
        );
    }
    let resolved = resolve_registry_dependencies(dir, &dependencies, true, None)?;
    let content = canonical_lockfile(&name, &version, &dependencies, &resolved);
    fs::write(&lock_path, content)
        .map_err(|e| format!("zap.lock: cannot write migrated lockfile: {e}"))?;
    Ok(format!(
        "migrated zap.lock from version 1 to version 2 with {} resolved registry packages",
        resolved.len()
    ))
}

pub(crate) fn install_dependencies(dir: &Path) -> Result<String, String> {
    let manifest_path = dir.join("zap.toml");
    let manifest = read_limited_text(&manifest_path, "manifest read")?;
    manifest_value(&manifest, "name").ok_or("zap.toml: missing package name".to_string())?;
    manifest_value(&manifest, "version").ok_or("zap.toml: missing package version".to_string())?;
    let dependencies = parse_dependencies(&manifest)?;
    let locked = validate_lockfile(dir, &manifest, false)?;
    let resolved = resolve_registry_dependencies(dir, &dependencies, false, Some(&locked))?;
    Ok(format_install_report(&resolved, dependencies.len()))
}

fn format_install_report(packages: &[LockedRegistryPackage], fallback_count: usize) -> String {
    let labels = packages
        .iter()
        .map(|package| format!("{}@{}", package.name, package.version))
        .collect::<Vec<_>>();
    if labels.is_empty() {
        return format!("installed {fallback_count} locked dependencies");
    }
    format!(
        "installed {} locked dependencies: {}",
        labels.len(),
        labels.join(", ")
    )
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
    let resolved = resolve_registry_dependencies(dir, &dependencies, true, None)?;
    let content = canonical_lockfile(&name, &version, &dependencies, &resolved);
    fs::write(dir.join("zap.lock"), content)
        .map_err(|e| format!("zap.lock: cannot write lockfile: {e}"))?;
    Ok(format!(
        "updated zap.lock with {} dependencies",
        dependencies.len()
    ))
}

fn validate_locked_registry_set(
    dependencies: &BTreeMap<String, DependencySpec>,
    locked: &[LockedRegistryPackage],
) -> Result<(), String> {
    let roots = dependencies
        .iter()
        .filter_map(|(name, spec)| match spec {
            DependencySpec::Requirement(requirement) => Some((name, requirement)),
            DependencySpec::LocalPath(_) => None,
        })
        .collect::<BTreeMap<_, _>>();
    if roots.is_empty() {
        if locked.is_empty() {
            return Ok(());
        }
        return Err(
            "zap.lock: resolved registry packages exist but manifest has no registry dependencies"
                .into(),
        );
    }
    for (name, requirement) in roots {
        let package = locked
            .iter()
            .find(|package| package.name == *name)
            .ok_or_else(|| format!("zap.lock: locked package `{name}` is missing"))?;
        if !version_satisfies_requirement(&package.version, requirement)? {
            return Err(format!(
                "zap.lock: locked package `{name}` version {} does not satisfy `{requirement}`",
                package.version
            ));
        }
    }
    Ok(())
}

fn validate_locked_cache(
    cache_root: &Path,
    locked: &[LockedRegistryPackage],
) -> Result<bool, String> {
    for package in locked {
        let registry_package = package.clone().into_registry();
        let cached = package_cache_path(cache_root, &registry_package);
        if !cached.is_file() {
            return Ok(false);
        }
        verify_cached_package(&cached, &registry_package)?;
    }
    Ok(true)
}

fn resolve_registry_dependencies(
    project_dir: &Path,
    dependencies: &BTreeMap<String, DependencySpec>,
    update: bool,
    locked: Option<&[LockedRegistryPackage]>,
) -> Result<Vec<LockedRegistryPackage>, String> {
    let cache_root = std::env::var_os("ZAP_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| project_dir.join(".zap/cache"));
    let Some(index_path) = std::env::var_os("ZAP_REGISTRY_INDEX") else {
        let locked = locked.unwrap_or_default();
        if !update && !locked.is_empty() {
            validate_locked_registry_set(dependencies, locked)?;
            if !validate_locked_cache(&cache_root, locked)? {
                return Err("registry package is not cached without a registry index".into());
            }
        }
        // A version-1 lockfile records dependency requirements but has no
        // resolved package section. Preserve that legacy offline behavior;
        // version-2 lockfiles are rejected earlier when their resolved section
        // is absent or incomplete.
        return Ok(locked.to_vec());
    };
    if !update {
        let locked = locked.unwrap_or_default();
        if !locked.is_empty() {
            validate_locked_registry_set(dependencies, locked)?;
            if validate_locked_cache(&cache_root, locked)? {
                return Ok(locked.to_vec());
            }
        }
    }
    let index_source = index_path.to_string_lossy().into_owned();
    let credentials = load_registry_credentials()?;
    let index = if Path::new(&index_source).is_file() {
        read_index(Path::new(&index_source))?
    } else {
        read_index_source_with_credentials(&index_source, &credentials)?
    };
    let index_path = PathBuf::from(&index_source);
    let offline = std::env::var_os("ZAP_OFFLINE").is_some();
    let trusted_policy = crate::registry::load_effective_trusted_registry_policy()?;
    let roots = dependencies
        .iter()
        .filter_map(|(name, spec)| match spec {
            DependencySpec::Requirement(requirement) => Some((name.clone(), requirement.clone())),
            DependencySpec::LocalPath(_) => None,
        })
        .collect::<BTreeMap<_, _>>();
    let resolved = resolve_dependency_graph(&index, &roots)?;
    let resolved_locked = resolved
        .iter()
        .map(LockedRegistryPackage::from_registry)
        .collect::<Vec<_>>();
    if !update {
        let locked = locked.unwrap_or_default();
        if roots.is_empty() {
            if !locked.is_empty() {
                return Err("zap.lock: resolved registry packages exist but manifest has no registry dependencies".into());
            }
        } else if !locked.is_empty() && locked != resolved_locked {
            return Err(
                "zap.lock: registry resolution differs from the lockfile; run `zap update`".into(),
            );
        }
    }
    for package in resolved {
        let cached = package_cache_path(&cache_root, &package);
        if cached.is_file() {
            verify_cached_package(&cached, &package)?;
            continue;
        }
        if offline {
            return Err(format!(
                "registry package is not cached in offline mode: {} {}",
                package.name, package.version
            ));
        }
        if package.source.starts_with("http://") || package.source.starts_with("https://") {
            trusted_policy.require_trusted(&package.source)?;
            cache_package_source_with_credentials(
                &package.source,
                &cache_root,
                &package,
                &credentials,
            )?;
        } else {
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
        }
        if update && !cached.is_file() {
            verify_cached_package(&cached, &package)?;
        }
    }
    Ok(resolved_locked)
}
fn qualified_project_root(base: &Path, raw: &str) -> Option<PathBuf> {
    if !raw.starts_with("bootstrap/") {
        return None;
    }
    let mut current = base;
    loop {
        if current.join("bootstrap").is_dir() {
            return Some(current.to_path_buf());
        }
        current = current.parent()?;
    }
}

pub(crate) fn resolve_module(base: &Path, raw: &str) -> Option<PathBuf> {
    let candidate = if Path::new(raw).extension().is_some() {
        raw.to_string()
    } else {
        format!("{raw}.zp")
    };
    let mut candidates = vec![
        base.join(&candidate),
        base.join("modules").join(&candidate),
        base.join("lib").join(&candidate),
    ];
    if let Some(root) = qualified_project_root(base, raw) {
        candidates.push(root.join(&candidate));
        candidates.push(root.join("modules").join(&candidate));
        candidates.push(root.join("lib").join(&candidate));
    }
    candidates.into_iter().find(|p| p.is_file())
}

pub(crate) fn collect_test_files(dir: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    let mut visited = HashSet::new();
    collect_test_files_inner(dir, files, &mut visited)
}

fn collect_test_files_inner(
    dir: &Path,
    files: &mut Vec<PathBuf>,
    visited: &mut HashSet<PathBuf>,
) -> Result<(), String> {
    let canonical_dir = fs::canonicalize(dir)
        .map_err(|e| format!("cannot resolve test directory {}: {e}", dir.display()))?;
    if !visited.insert(canonical_dir) {
        return Ok(());
    }
    for entry in fs::read_dir(dir)
        .map_err(|e| format!("cannot read test directory {}: {e}", dir.display()))?
    {
        let path = entry
            .map_err(|e| format!("cannot inspect test directory {}: {e}", dir.display()))?
            .path();
        let metadata = fs::symlink_metadata(&path)
            .map_err(|e| format!("cannot inspect test path {}: {e}", path.display()))?;
        let file_type = metadata.file_type();
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            collect_test_files_inner(&path, files, visited)?;
        } else if file_type.is_file()
            && path.extension().and_then(|x| x.to_str()) == Some("zp")
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
            "{{\"passed\":{passed},\"failed\":{failed},\"skipped\":{skipped},\"tests\":[{cases}]}}"
        );
    } else {
        println!("test result: {passed} passed; {failed} failed; {skipped} skipped");
    }
}

fn test_module_base(path: &Path) -> PathBuf {
    let starting = path.parent().unwrap_or(Path::new("."));
    let mut current = if starting.as_os_str().is_empty() {
        Path::new(".")
    } else {
        starting
    };
    loop {
        if current.join("zap.toml").is_file() {
            return current.to_path_buf();
        }
        match current.parent() {
            Some(parent) if parent != current && !parent.as_os_str().is_empty() => current = parent,
            _ => return Path::new(".").to_path_buf(),
        }
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
        let base = test_module_base(&path);
        let outcome = read_limited_text(&path, "test read").and_then(|source| run(&source, &base));
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

#[cfg(test)]
mod lockfile_security_tests {
    use super::{
        collect_test_files, package_cache_path, parse_lockfile_quoted, parse_resolved_lockfile,
        resolve_module, validate_locked_cache, validate_locked_registry_set, validate_project,
        validate_project_locked, DependencySpec, LockedRegistryPackage,
    };
    use crate::registry::sha256_hex;
    use std::{collections::BTreeMap, fs, path::Path};

    #[test]
    fn qualified_bootstrap_imports_resolve_from_nested_modules() {
        let root = std::env::temp_dir().join(format!("zap-resolve-module-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("bootstrap/b1")).expect("create parser directory");
        fs::create_dir_all(root.join("bootstrap/b2")).expect("create checker directory");
        let parser = root.join("bootstrap/b1/parser.zp");
        fs::write(&parser, "export fn parse():\n    return none\n").expect("write parser fixture");
        let nested = root.join("bootstrap/b2");
        assert_eq!(
            resolve_module(&nested, "bootstrap/b1/parser.zp"),
            Some(parser.clone())
        );
        assert_eq!(resolve_module(&nested, "bootstrap/b1/parser"), Some(parser));
        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn malformed_lockfile_corpus_is_deterministic_and_panic_free() {
        let corpus = [
            "",
            "lockfile_version = nope",
            "lockfile_version = 3",
            "lockfile_version = 2\n[resolved]\nfoo.version = \"1.0.0\"",
            "lockfile_version = 2\n[resolved]\nfoo.unknown = \"x\"",
            "lockfile_version = 2\n[resolved]\nfoo.version = \"1.0.0\"\nfoo.version = \"2.0.0\"",
            "lockfile_version = 2\n[resolved]\nfoo.version = \"unterminated",
            "lockfile_version = 2\n[resolved]\n../escape.version = \"1.0.0\"",
            "lockfile_version = 2\n[resolved]\nfoo.version = \"1.0.0\\q\"",
        ];
        for input in corpus {
            let first = std::panic::catch_unwind(|| parse_resolved_lockfile(input));
            let second = std::panic::catch_unwind(|| parse_resolved_lockfile(input));
            assert!(first.is_ok(), "lockfile parser panicked for {input:?}");
            assert_eq!(
                first
                    .as_ref()
                    .ok()
                    .and_then(|result| result.as_ref().ok())
                    .map(Vec::len),
                second
                    .as_ref()
                    .ok()
                    .and_then(|result| result.as_ref().ok())
                    .map(Vec::len)
            );
        }
    }

    #[test]
    fn locked_cache_is_authoritative_for_yanked_release_but_not_tampering() {
        let test_name = std::thread::current()
            .name()
            .unwrap_or("test")
            .chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
                    character
                } else {
                    '_'
                }
            })
            .collect::<String>();
        let root = std::env::temp_dir().join(format!(
            "zap-locked-cache-e2e-{}-{}",
            std::process::id(),
            test_name
        ));
        let _ = fs::remove_dir_all(&root);
        let cache = root.join("cache");
        let bytes = b"cached yanked release";
        let checksum = sha256_hex(bytes);
        let locked = [LockedRegistryPackage {
            name: "demo".into(),
            version: "1.2.3".into(),
            source: "file://demo.pkg".into(),
            checksum: checksum.clone(),
        }];
        let mut dependencies = BTreeMap::new();
        dependencies.insert("demo".into(), DependencySpec::Requirement("1.2.3".into()));
        validate_locked_registry_set(&dependencies, &locked).expect("locked requirement matches");
        let cache_path = package_cache_path(&cache, &locked[0].clone().into_registry());
        fs::create_dir_all(cache_path.parent().expect("cache parent")).unwrap();
        fs::write(&cache_path, bytes).unwrap();
        assert!(validate_locked_cache(&cache, &locked).expect("valid cached release"));
        fs::write(&cache_path, b"tampered release").unwrap();
        assert!(validate_locked_cache(&cache, &locked).is_err());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn lockfile_quoted_values_accept_only_supported_escapes() {
        assert_eq!(
            parse_lockfile_quoted("\"line\\nvalue\\\\ok\"", "test").expect("valid escapes"),
            "line\nvalue\\ok"
        );
        for raw in ["value", "\"unterminated", "\"bad\\q\"", "\"dangling\\\""] {
            assert!(
                parse_lockfile_quoted(raw, "test").is_err(),
                "accepted {raw:?}"
            );
        }
    }

    #[test]
    fn strict_project_validation_requires_a_lockfile() {
        let root = std::env::temp_dir().join(format!("zap-strict-build-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("temporary project directory");
        fs::write(
            root.join("zap.toml"),
            "[package]\nname = \"strict-build\"\nversion = \"0.1.0\"\nmain = \"main.zp\"\n",
        )
        .expect("manifest");
        fs::write(root.join("main.zp"), "say 1\n").expect("main source");
        assert!(validate_project(&root).is_ok());
        let error =
            validate_project_locked(&root).expect_err("locked validation must require zap.lock");
        assert!(error.contains("zap.lock: missing lockfile"));
        let _ = fs::remove_dir_all(root);
    }

    #[cfg(unix)]
    #[test]
    fn test_discovery_skips_symlink_directory_cycles() {
        use std::os::unix::fs::symlink;
        let root = std::env::temp_dir().join(format!("zap-test-discovery-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        let tests = root.join("tests");
        let nested = tests.join("nested");
        fs::create_dir_all(&nested).expect("test directories");
        fs::write(nested.join("smoke_test.zp"), "say 1\n").expect("test fixture");
        symlink(&tests, nested.join("loop")).expect("cycle symlink");
        let mut files = Vec::new();
        collect_test_files(Path::new(&tests), &mut files).expect("test discovery should finish");
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("smoke_test.zp"));
        let _ = fs::remove_dir_all(root);
    }
}
