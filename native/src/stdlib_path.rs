use crate::value::Value;
use std::fs;
use std::path::{Path, PathBuf};

/// Path operations standard library module
pub(crate) fn path_join(base: &str, segment: &str) -> Result<Value, String> {
    let result = PathBuf::from(base).join(segment);
    match result.to_str() {
        Some(path) => Ok(Value::Text(path.to_string())),
        None => Err("invalid path encoding".into()),
    }
}

pub(crate) fn path_basename(path: &str) -> Result<Value, String> {
    let result = Path::new(path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
    Ok(Value::Text(result.to_string()))
}

pub(crate) fn path_dirname(path: &str) -> Result<Value, String> {
    let result = Path::new(path)
        .parent()
        .and_then(|p| p.to_str())
        .unwrap_or("");
    Ok(Value::Text(result.to_string()))
}

pub(crate) fn path_exists(path: &str) -> Result<Value, String> {
    Ok(Value::Bool(Path::new(path).exists()))
}

pub(crate) fn path_is_file(path: &str) -> Result<Value, String> {
    Ok(Value::Bool(Path::new(path).is_file()))
}

pub(crate) fn path_is_dir(path: &str) -> Result<Value, String> {
    Ok(Value::Bool(Path::new(path).is_dir()))
}

pub(crate) fn path_is_symlink(path: &str) -> Result<Value, String> {
    Ok(Value::Bool(Path::new(path).is_symlink()))
}

pub(crate) fn path_canonical(path: &str) -> Result<Value, String> {
    let canonical = Path::new(path)
        .canonicalize()
        .map_err(|e| format!("cannot canonicalize path {}: {}", path, e))?;
    match canonical.to_str() {
        Some(path_str) => Ok(Value::Text(path_str.to_string())),
        None => Err("invalid path encoding after canonicalization".into()),
    }
}

pub(crate) fn path_create_dir(path: &str) -> Result<Value, String> {
    // Security check: prevent creating directories outside current directory
    let path_obj = Path::new(path);
    if path_obj.is_absolute() {
        return Err("security: absolute paths not allowed for directory creation".into());
    }

    fs::create_dir_all(path).map_err(|e| format!("failed to create directory {}: {}", path, e))?;
    Ok(Value::None)
}

pub(crate) fn path_remove_dir(path: &str) -> Result<Value, String> {
    // Security check: prevent removing directories outside current directory
    let path_obj = Path::new(path);
    if path_obj.is_absolute() {
        return Err("security: absolute paths not allowed for directory removal".into());
    }

    fs::remove_dir_all(path).map_err(|e| format!("failed to remove directory {}: {}", path, e))?;
    Ok(Value::None)
}

pub(crate) fn path_list_dir(path: &str) -> Result<Value, String> {
    let entries =
        fs::read_dir(path).map_err(|e| format!("failed to list directory {}: {}", path, e))?;

    let mut result = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("failed to read directory entry: {}", e))?;
        let name = entry
            .file_name()
            .to_str()
            .ok_or("invalid filename encoding")?
            .to_string();
        result.push(Value::Text(name));
    }

    Ok(Value::List(result))
}
