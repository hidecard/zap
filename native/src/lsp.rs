use serde_json::{json, Value};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    io::{self, Read, Write},
};

thread_local! {
    static DOCUMENTS: RefCell<BTreeMap<String, String>> = const { RefCell::new(BTreeMap::new()) };
}

pub fn run_stdio() -> Result<(), String> {
    let mut input = Vec::new();
    io::stdin()
        .read_to_end(&mut input)
        .map_err(|e| format!("lsp read failed: {e}"))?;
    let mut output = io::BufWriter::new(io::stdout());
    for message in decode_messages(&input)? {
        if let Some(response) = handle_message(&message) {
            encode_message(&mut output, &response)?;
        }
    }
    output.flush().map_err(|e| format!("lsp write failed: {e}"))
}

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
                    "hoverProvider": true,
                    "definitionProvider": true,
                    "workspaceSymbolProvider": true
                },
                "serverInfo": {"name": "zap", "version": "1.0.0"}
            }
        })),
        "shutdown" => Some(json!({
            "jsonrpc": "2.0",
            "id": message.get("id").cloned().unwrap_or(Value::Null),
            "result": null
        })),
        "textDocument/completion" => Some(completion_response(message)),
        "textDocument/hover" => Some(hover_response(message)),
        "textDocument/definition" => Some(definition_response(message)),
        "workspace/symbol" => Some(workspace_symbol_response(message)),
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
        _ => None,
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
        ("import", "Import a module"),
        ("return", "Return a value from a function"),
        ("async", "Declare an asynchronous function"),
        ("await", "Await a Future value"),
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

fn workspace_symbol_response(message: &Value) -> Value {
    let query = message["params"]["query"].as_str().unwrap_or("");
    let documents = DOCUMENTS.with(|documents| {
        documents
            .borrow()
            .iter()
            .map(|(uri, source)| (uri.clone(), source.clone()))
            .collect::<Vec<_>>()
    });
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
        assert_eq!(response["result"]["items"].as_array().unwrap().len(), 11);
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
