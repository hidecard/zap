use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use crate::value::Value;

/// File I/O operations standard library module
pub(crate) fn file_read(path: &str) -> Result<Value, String> {
    let contents = fs::read_to_string(path)
        .map_err(|e| format!("failed to read file: {}", e))?;
    Ok(Value::Text(contents))
}

pub(crate) fn file_write(path: &str, content: &str) -> Result<Value, String> {
    // Security check: prevent writing outside current directory
    let path_obj = Path::new(path);
    if path_obj.is_absolute() {
        return Err("security: absolute file paths not allowed for write operations".into());
    }
    
    fs::write(path, content)
        .map_err(|e| format!("failed to write file: {}", e))?;
    Ok(Value::None)
}

pub(crate) fn file_append(path: &str, content: &str) -> Result<Value, String> {
    // Security check: prevent writing outside current directory
    let path_obj = Path::new(path);
    if path_obj.is_absolute() {
        return Err("security: absolute file paths not allowed for append operations".into());
    }
    
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|e| format!("failed to open file for append: {}", e))?;
    
    file.write_all(content.as_bytes())
        .map_err(|e| format!("failed to append to file: {}", e))?;
    
    Ok(Value::None)
}

pub(crate) fn file_delete(path: &str) -> Result<Value, String> {
    // Security check: prevent deleting files outside current directory
    let path_obj = Path::new(path);
    if path_obj.is_absolute() {
        return Err("security: absolute file paths not allowed for delete operations".into());
    }
    
    fs::remove_file(path)
        .map_err(|e| format!("failed to delete file: {}", e))?;
    Ok(Value::None)
}

pub(crate) fn file_exists(path: &str) -> Result<Value, String> {
    Ok(Value::Bool(Path::new(path).exists()))
}

pub(crate) fn file_size(path: &str) -> Result<Value, String> {
    let metadata = fs::metadata(path)
        .map_err(|e| format!("failed to get file metadata: {}", e))?;
    Ok(Value::Number(metadata.len() as i64))
}

pub(crate) fn file_is_safe_path(path: &str) -> Result<Value, String> {
    let path_obj = Path::new(path);
    
    // Reject absolute paths
    if path_obj.is_absolute() {
        return Ok(Value::Bool(false));
    }
    
    // Check for parent directory traversal
    if path_obj.components().any(|c| c.as_os_str() == "..") {
        return Ok(Value::Bool(false));
    }
    
    Ok(Value::Bool(true))
}

pub(crate) fn file_modified_time(path: &str) -> Result<Value, String> {
    let metadata = fs::metadata(path)
        .map_err(|e| format!("failed to get file metadata: {}", e))?;
    let modified = metadata.modified()
        .map_err(|e| format!("failed to get modified time: {}", e))?;
    let duration = modified.duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| format!("failed to compute duration: {}", e))?;
    Ok(Value::Number(duration.as_secs() as i64))
}

pub(crate) fn file_is_readonly(path: &str) -> Result<Value, String> {
    let metadata = fs::metadata(path)
        .map_err(|e| format!("failed to get file metadata: {}", e))?;
    let readonly = metadata.permissions().readonly();
    Ok(Value::Bool(readonly))
}