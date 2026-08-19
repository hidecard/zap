use std::{
    fs,
    path::{Path, PathBuf},
};

use super::{manifest_value, run, validate_function_calls, validate_function_signatures};

pub(crate) fn validate_project(dir: &Path) -> Result<String, String> {
    let manifest = dir.join("zap.toml");
    let text = fs::read_to_string(&manifest)
        .map_err(|e| format!("cannot read {}: {e}", manifest.display()))?;
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
    let source = fs::read_to_string(&main_path)
        .map_err(|e| format!("cannot read {}: {e}", main_path.display()))?;
    validate_function_signatures(&source, &main_path)?;
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
pub(crate) fn run_zap_tests(dir: &Path) -> Result<usize, String> {
    let mut files = Vec::new();
    collect_test_files(dir, &mut files)?;
    files.sort();
    if files.is_empty() {
        return Err(format!("no *_test.zp files found in {}", dir.display()));
    }
    let mut passed = 0;
    for path in files {
        let source = fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        match run(&source, path.parent().unwrap_or(Path::new("."))) {
            Ok(()) => {
                println!("ok   {}", path.display());
                passed += 1;
            }
            Err(error) => {
                eprintln!("FAIL {}: {}", path.display(), error);
                return Err(format!("{} test file(s) failed", path.display()));
            }
        }
    }
    println!("{} Zap test file(s) passed", passed);
    Ok(passed)
}
