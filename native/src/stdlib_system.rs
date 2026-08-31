use std::env;
use crate::value::Value;

/// System operations standard library module
pub(crate) fn system_env_var(name: &str) -> Result<Value, String> {
    match env::var(name) {
        Ok(value) => Ok(Value::Text(value)),
        Err(_) => Ok(Value::None),
    }
}

pub(crate) fn system_args() -> Result<Value, String> {
    let args: Vec<Value> = env::args()
        .skip(1) // Skip program name
        .map(|arg| Value::Text(arg))
        .collect();
    Ok(Value::List(args))
}

pub(crate) fn system_exit(code: i64) -> Result<Value, String> {
    std::process::exit(code as i32);
}

pub(crate) fn system_exit_with_message(code: i64, message: &str) -> Result<Value, String> {
    eprintln!("{}", message);
    std::process::exit(code as i32);
}