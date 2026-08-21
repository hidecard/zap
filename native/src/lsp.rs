use serde_json::{json, Value};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs,
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

thread_local! {
    static DOCUMENTS: RefCell<BTreeMap<String, String>> = const { RefCell::new(BTreeMap::new()) };
}

pub fn run_stdio() -> Result<(), String> {
    let stdin = io::stdin();
    let mut input = BufReader::new(stdin.lock());
    let mut output = io::BufWriter::new(io::stdout());
    while let Some(message) = read_message(&mut input)? {
        if let Some(response) = handle_message(&message) {
            encode_message(&mut output, &response)?;
            output
                .flush()
                .map_err(|e| format!("lsp write failed: {e}"))?;
        }
    }
    Ok(())
}

fn read_message<R: BufRead>(reader: &mut R) -> Result<Option<Value>, String> {
    let mut content_length = None;
    let mut line = String::new();
    loop {
        line.clear();
        let bytes = reader
            .read_line(&mut line)
            .map_err(|e| format!("lsp header read failed: {e}"))?;
        if bytes == 0 {
            return if content_length.is_none() {
                Ok(None)
            } else {
                Err("lsp message ended before the header terminator".to_string())
            };
        }
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }
        if let Some(value) = trimmed.strip_prefix("Content-Length:") {
            content_length = Some(
                value
                    .trim()
                    .parse::<usize>()
                    .map_err(|_| "lsp Content-Length is invalid".to_string())?,
            );
        }
    }
    let length =
        content_length.ok_or_else(|| "lsp message is missing Content-Length".to_string())?;
    let mut body = vec![0_u8; length];
    reader
        .read_exact(&mut body)
        .map_err(|e| format!("lsp body read failed: {e}"))?;
    serde_json::from_slice(&body)
        .map(Some)
        .map_err(|e| format!("lsp JSON is invalid: {e}"))
}

#[cfg(test)]
pub fn decode_messages(input: &[u8]) -> Result<Vec<Value>, String> {
    let mut messages = Vec::new();
    let mut cursor = 0;
    while cursor < input.len() {
        let header_end = input[cursor..]
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .ok_or_else(|| "lsp message is missing a header terminator".to_string())?
            + cursor;
        let header = std::str::from_utf8(&input[cursor..header_end])
            .map_err(|_| "lsp headers must be utf-8".to_string())?;
        let length = header
            .lines()
            .find_map(|line| line.strip_prefix("Content-Length:"))
            .ok_or_else(|| "lsp message is missing Content-Length".to_string())?
            .trim()
            .parse::<usize>()
            .map_err(|_| "lsp Content-Length is invalid".to_string())?;
        let body_start = header_end + 4;
        let body_end = body_start
            .checked_add(length)
            .ok_or_else(|| "lsp message length overflow".to_string())?;
        if body_end > input.len() {
            return Err("lsp message body is truncated".to_string());
        }
        let value: Value = serde_json::from_slice(&input[body_start..body_end])
            .map_err(|e| format!("lsp JSON is invalid: {e}"))?;
        messages.push(value);
        cursor = body_end;
    }
    Ok(messages)
}

pub fn encode_message<W: Write>(writer: &mut W, value: &Value) -> Result<(), String> {
    let body = serde_json::to_vec(value).map_err(|e| format!("lsp JSON encode failed: {e}"))?;
    write!(writer, "Content-Length: {}\r\n\r\n", body.len())
        .map_err(|e| format!("lsp header write failed: {e}"))?;
    writer
        .write_all(&body)
        .map_err(|e| format!("lsp body write failed: {e}"))
}

pub fn handle_message(message: &Value) -> Option<Value> {
    let method = message.get("method")?.as_str()?;
    match method {
        "initialize" => Some(json!({
            "jsonrpc": "2.0",
            "id": message.get("id").cloned().unwrap_or(Value::Null),
            "result": {
                "capabilities": {
                    "textDocumentSync": 1,
                    "diagnosticProvider": {"interFileDependencies": false, "workspaceDiagnostics": false},
                    "completionProvider": {"resolveProvider": false, "triggerCharacters": ["."]},
                    "signatureHelpProvider": {"triggerCharacters": ["(", ","]},
                    "hoverProvider": true,
                    "definitionProvider": true,
                    "workspaceSymbolProvider": true,
                    "documentSymbolProvider": true,
                    "documentFormattingProvider": true
                },
                "serverInfo": {"name": "zap", "version": env!("CARGO_PKG_VERSION")}
            }
        })),
        "shutdown" => Some(json!({
            "jsonrpc": "2.0",
            "id": message.get("id").cloned().unwrap_or(Value::Null),
            "result": null
        })),
        "textDocument/completion" => Some(completion_response(message)),
        "textDocument/signatureHelp" => Some(signature_help_response(message)),
        "textDocument/hover" => Some(hover_response(message)),
        "textDocument/definition" => Some(definition_response(message)),
        "textDocument/formatting" => Some(formatting_response(message)),
        "workspace/symbol" => Some(workspace_symbol_response(message)),
        "textDocument/documentSymbol" => Some(document_symbol_response(message)),
        "textDocument/didOpen" | "textDocument/didChange" => {
            let params = message.get("params")?;
            let document = params.get("textDocument")?;
            let uri = document.get("uri")?.as_str()?;
            let text = document.get("text").and_then(Value::as_str).unwrap_or("");
            DOCUMENTS.with(|documents| {
                documents
                    .borrow_mut()
                    .insert(uri.to_string(), text.to_string());
            });
            Some(publish_diagnostics(uri, text))
        }
        _ => message.get("id").map(|id| {
            json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {
                    "code": -32601,
                    "message": "Method not found"
                }
            })
        }),
    }
}

fn completion_response(message: &Value) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = DOCUMENTS
        .with(|documents| documents.borrow().get(uri).cloned())
        .unwrap_or_default();
    let position = &message["params"]["position"];
    let prefix = source_prefix(
        &source,
        position["line"].as_u64().unwrap_or(0) as usize,
        position["character"].as_u64().unwrap_or(0) as usize,
    );
    let mut candidates = vec![
        ("let", "Declare a local binding"),
        ("fn", "Declare a function"),
        ("if", "Start a conditional expression"),
        ("else", "Start the alternative branch"),
        ("for", "Start a loop"),
        ("while", "Start a loop"),
        ("class", "Declare a class"),
        ("module", "Declare a module"),
        ("import", "Import a module"),
        ("return", "Return a value from a function"),
        ("async", "Declare an asynchronous function"),
        ("await", "Await a Future value"),
        ("spawn", "Create a task from a Future"),
        ("task_join", "Join a spawned task"),
        ("task_is_ready", "Check task readiness"),
    ];
    for line in source.lines() {
        let declaration = line.trim();
        if let Some(name) = declaration
            .strip_prefix("let ")
            .and_then(|value| value.split([':', '=']).next())
        {
            candidates.push((name.trim(), "Local binding"));
        } else if let Some(name) = declaration
            .strip_prefix("fn ")
            .or_else(|| declaration.strip_prefix("async fn "))
        {
            if let Some(name) = name.split('(').next() {
                candidates.push((name.trim(), "Function"));
            }
        }
    }
    candidates.dedup_by(|left, right| left.0 == right.0);
    let items = candidates
        .into_iter()
        .filter(|(label, _)| prefix.is_empty() || label.starts_with(&prefix))
        .map(|(label, detail)| json!({"label": label, "kind": 14, "detail": detail}))
        .collect::<Vec<_>>();
    json!({"jsonrpc": "2.0", "id": message.get("id").cloned().unwrap_or(Value::Null), "result": {"isIncomplete": false, "items": items}})
}

fn source_prefix(source: &str, line: usize, character: usize) -> String {
    source
        .lines()
        .nth(line)
        .unwrap_or("")
        .chars()
        .take(character)
        .collect::<String>()
        .rsplit(|value: char| !value.is_ascii_alphanumeric() && value != '_')
        .next()
        .unwrap_or("")
        .to_string()
}

fn signature_help_response(message: &Value) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = DOCUMENTS
        .with(|documents| documents.borrow().get(uri).cloned())
        .unwrap_or_default();
    let line = message["params"]["position"]["line"].as_u64().unwrap_or(0) as usize;
    let character = message["params"]["position"]["character"]
        .as_u64()
        .unwrap_or(0) as usize;
    let line_prefix = source
        .lines()
        .nth(line)
        .unwrap_or("")
        .chars()
        .take(character)
        .collect::<String>();
    let Some(open) = line_prefix.rfind('(') else {
        return json!({"jsonrpc": "2.0", "id": message.get("id").cloned().unwrap_or(Value::Null), "result": null});
    };
    let callee = line_prefix[..open]
        .rsplit(|value: char| !value.is_ascii_alphanumeric() && value != '_')
        .next()
        .unwrap_or("");
    if callee.is_empty() {
        return json!({"jsonrpc": "2.0", "id": message.get("id").cloned().unwrap_or(Value::Null), "result": null});
    }
    let active_parameter = line_prefix[open + 1..]
        .chars()
        .filter(|value| *value == ',')
        .count();
    let signature = source.lines().find_map(|line| {
        let trimmed = line.trim();
        let declaration = trimmed
            .strip_prefix("fn ")
            .or_else(|| trimmed.strip_prefix("async fn "))?;
        let name_end = declaration.find('(')?;
        if declaration[..name_end].trim() != callee {
            return None;
        }
        let close = declaration[name_end + 1..].find(')')? + name_end + 1;
        let parameters = declaration[name_end + 1..close]
            .split(',')
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| json!({"label": value}))
            .collect::<Vec<_>>();
        let label = trimmed.trim_end_matches(':').to_string();
        Some(json!({
            "label": label,
            "documentation": format!("Zap function `{callee}`"),
            "parameters": parameters
        }))
    });
    let result = signature.map(|signature| {
        let parameter_count = signature["parameters"].as_array().map_or(0, Vec::len);
        json!({
            "signatures": [signature],
            "activeSignature": 0,
            "activeParameter": active_parameter.min(parameter_count.saturating_sub(1))
        })
    });
    json!({"jsonrpc": "2.0", "id": message.get("id").cloned().unwrap_or(Value::Null), "result": result.unwrap_or(Value::Null)})
}

fn hover_response(message: &Value) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = DOCUMENTS
        .with(|documents| documents.borrow().get(uri).cloned())
        .unwrap_or_default();
    let line = message["params"]["position"]["line"].as_u64().unwrap_or(0) as usize;
    let character = message["params"]["position"]["character"]
        .as_u64()
        .unwrap_or(0) as usize;
    let word = source_prefix(&source, line, character);
    let program = crate::ast::parse_program(&source).ok();
    let description = program.as_ref().and_then(|program| {
        program
            .statements
            .iter()
            .find_map(|statement| match &statement.node {
                crate::ast::Stmt::Module { name } if name == &word => {
                    Some(format!("module `{name}`"))
                }
                crate::ast::Stmt::Import {
                    path,
                    explicit: true,
                    alias,
                } if alias.as_deref() == Some(word.as_str()) || path == &word => Some(format!(
                    "import `{path}`{}",
                    alias
                        .as_ref()
                        .map_or(String::new(), |value| format!(" as `{value}`"))
                )),
                crate::ast::Stmt::Function {
                    name,
                    return_type,
                    is_async,
                    ..
                } if name == &word => Some(format!(
                    "{}function `{name}` -> `{}`",
                    if *is_async { "async " } else { "" },
                    return_type.as_deref().unwrap_or("none")
                )),
                crate::ast::Stmt::Class { name, .. } if name == &word => {
                    Some(format!("class `{name}`"))
                }
                crate::ast::Stmt::Declaration {
                    name, annotation, ..
                } if name == &word => Some(format!(
                    "binding `{name}`: `{}`",
                    annotation.as_deref().unwrap_or("inferred")
                )),
                _ => None,
            })
    });
    let result = description
        .map(|value| json!({"contents": {"kind": "markdown", "value": value}}))
        .unwrap_or(Value::Null);
    json!({"jsonrpc": "2.0", "id": message.get("id").cloned().unwrap_or(Value::Null), "result": result})
}

fn definition_response(message: &Value) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = DOCUMENTS
        .with(|documents| documents.borrow().get(uri).cloned())
        .unwrap_or_default();
    let position = &message["params"]["position"];
    let word = source_prefix(
        &source,
        position["line"].as_u64().unwrap_or(0) as usize,
        position["character"].as_u64().unwrap_or(0) as usize,
    );
    let locations = declaration_symbols(uri, &source)
        .into_iter()
        .filter(|(name, _, _, _)| name == &word)
        .map(|(_, _, range, _)| json!({"uri": uri, "range": range}))
        .collect::<Vec<_>>();
    let result = if locations.is_empty() {
        Value::Null
    } else {
        Value::Array(locations)
    };
    json!({"jsonrpc": "2.0", "id": message.get("id").cloned().unwrap_or(Value::Null), "result": result})
}

fn formatting_response(message: &Value) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = DOCUMENTS
        .with(|documents| documents.borrow().get(uri).cloned())
        .unwrap_or_default();
    let formatted = format_source(&source);
    let end_line = source.lines().count().saturating_sub(1) as u64;
    let end_character = source
        .lines()
        .last()
        .map(|line| line.chars().count())
        .unwrap_or(0) as u64;
    let edits = if formatted == source {
        Vec::new()
    } else {
        vec![json!({
            "range": {
                "start": {"line": 0, "character": 0},
                "end": {"line": end_line, "character": end_character}
            },
            "newText": formatted
        })]
    };
    json!({
        "jsonrpc": "2.0",
        "id": message.get("id").cloned().unwrap_or(Value::Null),
        "result": edits
    })
}

fn format_source(source: &str) -> String {
    let mut formatted = source
        .replace("\r\n", "\n")
        .replace('\r', "\n")
        .lines()
        .map(|line| line.trim_end().replace('\t', "    "))
        .collect::<Vec<_>>()
        .join("\n");
    if !formatted.is_empty() && !formatted.ends_with('\n') {
        formatted.push('\n');
    }
    formatted
}

fn workspace_symbol_response(message: &Value) -> Value {
    let query = message["params"]["query"].as_str().unwrap_or("");
    let documents = workspace_documents();
    let symbols = documents
        .iter()
        .flat_map(|(uri, source)| {
            declaration_symbols(uri, source)
                .into_iter()
                .map(|(name, kind, range, detail)| (uri.clone(), name, kind, range, detail))
                .collect::<Vec<_>>()
        })
        .filter(|(_, name, _, _, _)| query.is_empty() || name.contains(query))
        .map(|(uri, name, kind, range, detail)| {
            json!({"name": name, "kind": kind, "location": {"uri": uri, "range": range}, "containerName": detail})
        })
        .collect::<Vec<_>>();
    json!({"jsonrpc": "2.0", "id": message.get("id").cloned().unwrap_or(Value::Null), "result": symbols})
}

fn workspace_documents() -> Vec<(String, String)> {
    const MAX_MODULE_BYTES: u64 = 8 * 1024 * 1024;
    let mut documents = DOCUMENTS.with(|documents| {
        documents
            .borrow()
            .iter()
            .map(|(uri, source)| (uri.clone(), source.clone()))
            .collect::<BTreeMap<_, _>>()
    });
    let mut pending = documents.keys().cloned().collect::<Vec<_>>();
    let mut cursor = 0;
    while cursor < pending.len() {
        let uri = pending[cursor].clone();
        cursor += 1;
        let Some(source) = documents.get(&uri).cloned() else {
            continue;
        };
        let Some(source_path) = file_uri_path(&uri) else {
            continue;
        };
        let Some(parent) = source_path.parent() else {
            continue;
        };
        let Ok(program) = crate::ast::parse_program(&source) else {
            continue;
        };
        for statement in program.statements {
            let crate::ast::Stmt::Import {
                path,
                explicit: true,
                ..
            } = statement.node
            else {
                continue;
            };
            let Ok(relative) = module_import_path(&path) else {
                continue;
            };
            let candidate = parent.join(relative);
            let Ok(canonical) = candidate.canonicalize() else {
                continue;
            };
            let Ok(canonical_parent) = parent.canonicalize() else {
                continue;
            };
            if !canonical.starts_with(&canonical_parent)
                || !canonical.is_file()
                || fs::metadata(&canonical)
                    .map(|metadata| metadata.len() > MAX_MODULE_BYTES)
                    .unwrap_or(true)
            {
                continue;
            }
            let Ok(module_source) = fs::read_to_string(&canonical) else {
                continue;
            };
            let module_uri = path_to_file_uri(&canonical);
            if documents
                .insert(module_uri.clone(), module_source)
                .is_none()
            {
                pending.push(module_uri);
            }
        }
    }
    documents.into_iter().collect()
}

fn file_uri_path(uri: &str) -> Option<PathBuf> {
    uri.strip_prefix("file://").map(PathBuf::from)
}

fn path_to_file_uri(path: &Path) -> String {
    format!("file://{}", path.display())
}

fn module_import_path(path: &str) -> Result<PathBuf, String> {
    let normalized = path.trim().trim_matches('"');
    if normalized.is_empty() || normalized.contains(['/', '\\']) {
        return Err(format!("invalid explicit import path `{path}`"));
    }
    let mut relative = PathBuf::new();
    for component in normalized.split('.') {
        if component.is_empty() || component == "." || component == ".." {
            return Err(format!("invalid explicit import path `{path}`"));
        }
        relative.push(component);
    }
    if relative.extension().is_none() {
        relative.set_extension("zp");
    }
    Ok(relative)
}

/// Return `(name, kind, range, detail)` for top-level declarations.
fn declaration_symbols(uri: &str, source: &str) -> Vec<(String, u32, Value, String)> {
    let Ok(program) = crate::ast::parse_program(source) else {
        return Vec::new();
    };
    program
        .statements
        .iter()
        .filter_map(|statement| {
            let (name, kind, detail) = match &statement.node {
                crate::ast::Stmt::Function { name, is_async, .. } => (
                    name.clone(),
                    12,
                    if *is_async {
                        "async function"
                    } else {
                        "function"
                    },
                ),
                crate::ast::Stmt::Class { name, .. } => (name.clone(), 5, "class"),
                crate::ast::Stmt::Declaration { name, .. } => (name.clone(), 13, "binding"),
                crate::ast::Stmt::Module { name } => (name.clone(), 3, "module"),
                crate::ast::Stmt::Import {
                    path,
                    explicit: true,
                    alias,
                } => (alias.clone().unwrap_or_else(|| path.clone()), 2, "import"),
                _ => return None,
            };
            let line_index = statement.span.line.saturating_sub(1);
            let line = source.lines().nth(line_index).unwrap_or("");
            let column = line
                .find(&name)
                .unwrap_or(statement.span.column.saturating_sub(1));
            let start_line = line_index as u64;
            let start_character = column as u64;
            let end_character = start_character + name.chars().count() as u64;
            let range = json!({
                "start": {"line": start_line, "character": start_character},
                "end": {"line": start_line, "character": end_character}
            });
            Some((name, kind, range, format!("{detail} in {uri}")))
        })
        .collect()
}

fn document_symbol_response(message: &Value) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = DOCUMENTS
        .with(|documents| documents.borrow().get(uri).cloned())
        .unwrap_or_default();
    let symbols = crate::ast::parse_program(&source)
        .map(|program| document_symbols_for_program(uri, &source, &program))
        .unwrap_or_default();
    json!({
        "jsonrpc": "2.0",
        "id": message.get("id").cloned().unwrap_or(Value::Null),
        "result": symbols
    })
}

fn document_symbols_for_program(
    uri: &str,
    source: &str,
    program: &crate::ast::Program,
) -> Vec<Value> {
    program
        .statements
        .iter()
        .filter_map(|statement| document_symbol_for_statement(uri, source, statement))
        .collect()
}

fn document_symbol_for_statement(
    uri: &str,
    source: &str,
    statement: &crate::ast::Spanned<crate::ast::Stmt>,
) -> Option<Value> {
    let (name, kind, detail) = match &statement.node {
        crate::ast::Stmt::Function { name, is_async, .. } => (
            name.clone(),
            12,
            if *is_async {
                "async function"
            } else {
                "function"
            },
        ),
        crate::ast::Stmt::Class { name, .. } => (name.clone(), 5, "class"),
        crate::ast::Stmt::Declaration { name, .. } => (name.clone(), 13, "binding"),
        crate::ast::Stmt::Module { name } => (name.clone(), 2, "module"),
        crate::ast::Stmt::Import {
            path,
            explicit: true,
            alias,
        } => (alias.clone().unwrap_or_else(|| path.clone()), 9, "import"),
        _ => return None,
    };
    let range = symbol_range(source, &statement.span, &name);
    let children = match &statement.node {
        crate::ast::Stmt::Function { body, .. } | crate::ast::Stmt::Class { body, .. } => {
            document_symbols_for_program(uri, source, body)
        }
        _ => Vec::new(),
    };
    let mut symbol = json!({
        "name": name,
        "kind": kind,
        "detail": format!("{detail} in {uri}"),
        "range": range,
        "selectionRange": range
    });
    if !children.is_empty() {
        symbol["children"] = Value::Array(children);
    }
    Some(symbol)
}

fn symbol_range(source: &str, span: &crate::lexer::SourceSpan, name: &str) -> Value {
    let line_index = span.line.saturating_sub(1);
    let line = source.lines().nth(line_index).unwrap_or("");
    let column = line.find(name).unwrap_or(span.column.saturating_sub(1));
    let start = json!({
        "line": line_index as u64,
        "character": column as u64
    });
    let end = json!({
        "line": line_index as u64,
        "character": (column + name.chars().count()) as u64
    });
    json!({"start": start, "end": end})
}

fn publish_diagnostics(uri: &str, source: &str) -> Value {
    let diagnostics = crate::lint_source(source)
        .into_iter()
        .map(|message| {
            let line = diagnostic_line(&message).unwrap_or(1).saturating_sub(1);
            let width = source.lines().nth(line).map(|value| value.chars().count()).unwrap_or(1).max(1);
            json!({
                "range": {"start": {"line": line, "character": 0}, "end": {"line": line, "character": width}},
                "severity": 2,
                "source": "zap",
                "message": message
            })
        })
        .collect::<Vec<_>>();
    json!({
        "jsonrpc": "2.0",
        "method": "textDocument/publishDiagnostics",
        "params": {"uri": uri, "diagnostics": diagnostics}
    })
}

fn diagnostic_line(message: &str) -> Option<usize> {
    let suffix = message.strip_prefix("line ")?;
    let digits = suffix.split(':').next()?;
    digits.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::{decode_messages, handle_message};
    use serde_json::json;

    #[test]
    fn decodes_content_length_framed_json() {
        let body = br#"{"jsonrpc":"2.0","id":1,"method":"shutdown"}"#;
        let mut framed = format!("Content-Length: {}\r\n\r\n", body.len()).into_bytes();
        framed.extend_from_slice(body);
        let messages = decode_messages(&framed).unwrap();
        assert_eq!(messages[0]["method"], "shutdown");
    }

    #[test]
    fn line_diagnostic_uses_reported_source_line() {
        let response = handle_message(&json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": "file:///main.zp", "text": "let x = 1\n\tlet y = 2"}}
        }))
        .unwrap();
        assert_eq!(
            response["params"]["diagnostics"][0]["range"]["start"]["line"],
            1
        );
    }

    #[test]
    fn completion_returns_deterministic_keyword_items() {
        let response = handle_message(&json!({
            "jsonrpc": "2.0",
            "id": 8,
            "method": "textDocument/completion",
            "params": {"textDocument": {"uri": "file:///main.zp"}, "position": {"line": 0, "character": 0}}
        })).unwrap();
        assert_eq!(response["id"], 8);
        assert_eq!(response["result"]["isIncomplete"], false);
        assert_eq!(response["result"]["items"][0]["label"], "let");
        let labels = response["result"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| item["label"].as_str())
            .collect::<Vec<_>>();
        for expected in ["spawn", "task_join", "task_is_ready"] {
            assert!(
                labels.contains(&expected),
                "missing completion item: {expected}"
            );
        }
        assert_eq!(labels.len(), 15);
    }

    #[test]
    fn completion_filters_by_document_prefix() {
        let uri = "file:///completion.zp";
        let _ = handle_message(&json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": uri, "text": "async fn load():\n    return 1\nlo"}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 9, "method": "textDocument/completion",
            "params": {"textDocument": {"uri": uri}, "position": {"line": 2, "character": 2}}
        }))
        .unwrap();
        assert_eq!(response["result"]["items"][0]["label"], "load");
    }

    #[test]
    fn hover_uses_parser_owned_function_metadata() {
        let uri = "file:///hover.zp";
        let _ = handle_message(&json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": uri, "text": "async fn load() -> number:\n    return 1\nload()\n"}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 10, "method": "textDocument/hover",
            "params": {"textDocument": {"uri": uri}, "position": {"line": 2, "character": 4}}
        }))
        .unwrap();
        assert!(response["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("async function `load`"));
    }

    #[test]
    fn definition_returns_parser_span_location() {
        let uri = "file:///definition.zp";
        let _ = handle_message(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": uri, "text": "fn load():\n    return 1\nload()\n"}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 11, "method": "textDocument/definition",
            "params": {"textDocument": {"uri": uri}, "position": {"line": 2, "character": 4}}
        }))
        .unwrap();
        assert_eq!(response["result"][0]["uri"], uri);
        assert_eq!(response["result"][0]["range"]["start"]["line"], 0);
        assert_eq!(response["result"][0]["range"]["start"]["character"], 3);
    }

    #[test]
    fn hover_and_workspace_symbols_include_explicit_modules_and_imports() {
        let uri = "file:///module-symbols.zp";
        let _ = handle_message(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": uri, "text": "module app.core\nimport app.util as util\n"}}
        }));
        let hover = handle_message(&json!({
            "jsonrpc": "2.0", "id": 15, "method": "textDocument/hover",
            "params": {"textDocument": {"uri": uri}, "position": {"line": 1, "character": 25}}
        }))
        .unwrap();
        assert!(hover["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("import `app.util`"));

        let symbols = handle_message(&json!({
            "jsonrpc": "2.0", "id": 16, "method": "workspace/symbol",
            "params": {"query": ""}
        }))
        .unwrap();
        let names = symbols["result"].as_array().unwrap();
        assert!(names.iter().any(|item| item["name"] == "app.core"));
        assert!(names.iter().any(|item| item["name"] == "util"));
    }

    #[test]
    fn workspace_symbols_are_filtered_deterministically() {
        let _ = handle_message(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": "file:///symbols.zp", "text": "class Box:\n    pass\nfn load():\n    return 1\n"}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 12, "method": "workspace/symbol",
            "params": {"query": "load"}
        }))
        .unwrap();
        assert_eq!(response["result"].as_array().unwrap().len(), 1);
        assert_eq!(response["result"][0]["name"], "load");
    }

    #[test]
    fn signature_help_returns_function_parameters_and_active_parameter() {
        let uri = "file:///signature.zp";
        let _ = handle_message(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": uri, "text": "fn greet(name: text, punctuation: text = \"!\"):\n    return name\ngreet(\"Zap\", "}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 17, "method": "textDocument/signatureHelp",
            "params": {"textDocument": {"uri": uri}, "position": {"line": 2, "character": 15}}
        }))
        .unwrap();
        assert_eq!(
            response["result"]["signatures"][0]["label"],
            "fn greet(name: text, punctuation: text = \"!\")"
        );
        assert_eq!(response["result"]["activeParameter"], 1);
    }

    #[test]
    fn initialize_advertises_signature_help_and_formatting() {
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 18, "method": "initialize", "params": {}
        }))
        .unwrap();
        assert_eq!(
            response["result"]["capabilities"]["documentFormattingProvider"],
            true
        );
        assert_eq!(
            response["result"]["capabilities"]["signatureHelpProvider"]["triggerCharacters"][0],
            "("
        );
    }

    #[test]
    fn formatting_normalizes_newlines_tabs_and_trailing_spaces() {
        let uri = "file:///format.zp";
        let _ = handle_message(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": uri, "text": "fn main():  \r\n\treturn 1"}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 13, "method": "textDocument/formatting",
            "params": {"textDocument": {"uri": uri}, "options": {}}
        }))
        .unwrap();
        assert_eq!(
            response["result"][0]["newText"],
            "fn main():\n    return 1\n"
        );
    }

    #[test]
    fn workspace_symbols_include_multiple_documents() {
        let _ = handle_message(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": "file:///one.zp", "text": "fn first():\n    return 1\n"}}
        }));
        let _ = handle_message(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": "file:///two.zp", "text": "fn second():\n    return 2\n"}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 14, "method": "workspace/symbol",
            "params": {"query": ""}
        }))
        .unwrap();
        let symbols = response["result"].as_array().unwrap();
        assert!(symbols
            .iter()
            .any(|item| item["name"] == "first" && item["location"]["uri"] == "file:///one.zp"));
        assert!(symbols
            .iter()
            .any(|item| item["name"] == "second" && item["location"]["uri"] == "file:///two.zp"));
    }

    #[test]
    fn workspace_symbols_index_imported_modules_without_opening_them() {
        use std::{
            fs,
            time::{SystemTime, UNIX_EPOCH},
        };
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!("zap-lsp-index-{suffix}"));
        fs::create_dir_all(root.join("app")).unwrap();
        let main = root.join("main.zp");
        fs::write(
            &main,
            "import app.util as util\nfn main():\n    return util\n",
        )
        .unwrap();
        fs::write(root.join("app/util.zp"), "fn loaded():\n    return 1\n").unwrap();
        let uri = format!("file://{}", main.display());
        let _ = handle_message(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": uri, "text": "import app.util as util\nfn main():\n    return util\n"}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 20, "method": "workspace/symbol",
            "params": {"query": "loaded"}
        }))
        .unwrap();
        let symbols = response["result"].as_array().unwrap();
        assert_eq!(symbols.len(), 1);
        assert_eq!(symbols[0]["name"], "loaded");
        assert!(symbols[0]["location"]["uri"]
            .as_str()
            .unwrap()
            .ends_with("/app/util.zp"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn document_symbols_include_nested_declarations() {
        let uri = "file:///nested-symbols.zp";
        let _ = handle_message(&json!({
            "jsonrpc": "2.0", "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": uri, "text": "class Box:\n    fn build():\n        let value = 1\n        return value\nfn outer():\n    fn inner():\n        return 2\n    return inner()\n"}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 19, "method": "textDocument/documentSymbol",
            "params": {"textDocument": {"uri": uri}}
        }))
        .unwrap();
        let symbols = response["result"].as_array().unwrap();
        assert_eq!(
            symbols
                .iter()
                .filter(|symbol| symbol["name"] == "Box")
                .count(),
            1
        );
        assert_eq!(
            symbols
                .iter()
                .filter(|symbol| symbol["name"] == "outer")
                .count(),
            1
        );
        let class = symbols
            .iter()
            .find(|symbol| symbol["name"] == "Box")
            .unwrap();
        assert_eq!(class["children"][0]["name"], "build");
        let function = symbols
            .iter()
            .find(|symbol| symbol["name"] == "outer")
            .unwrap();
        assert_eq!(function["children"][0]["name"], "inner");
        assert_eq!(function["children"][0]["range"]["start"]["line"], 5);
    }

    #[test]
    fn unknown_request_method_returns_json_rpc_error() {
        let response = handle_message(&json!({
            "jsonrpc": "2.0",
            "id": 99,
            "method": "zap/unknown",
            "params": {}
        }))
        .expect("request errors must produce a response");
        assert_eq!(response["error"]["code"], -32601);
        assert_eq!(response["error"]["message"], "Method not found");
        assert!(handle_message(&json!({
            "jsonrpc": "2.0",
            "method": "zap/unknown"
        }))
        .is_none());
    }

    #[test]
    fn initialize_returns_deterministic_capabilities() {
        let response =
            handle_message(&json!({"jsonrpc":"2.0","id":7,"method":"initialize"})).unwrap();
        assert_eq!(response["id"], 7);
        assert_eq!(response["result"]["serverInfo"]["name"], "zap");
        assert_eq!(response["result"]["capabilities"]["textDocumentSync"], 1);
        assert_eq!(
            response["result"]["capabilities"]["definitionProvider"],
            true
        );
        assert_eq!(
            response["result"]["capabilities"]["workspaceSymbolProvider"],
            true
        );
    }
}
