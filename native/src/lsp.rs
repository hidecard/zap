use serde_json::{json, Value};
use std::io::{self, Read, Write};

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
                    "hoverProvider": false
                },
                "serverInfo": {"name": "zap", "version": "1.0.0"}
            }
        })),
        "shutdown" => Some(json!({
            "jsonrpc": "2.0",
            "id": message.get("id").cloned().unwrap_or(Value::Null),
            "result": null
        })),
        "textDocument/didOpen" | "textDocument/didChange" => {
            let params = message.get("params")?;
            let document = params.get("textDocument")?;
            let uri = document.get("uri")?.as_str()?;
            let text = document.get("text").and_then(Value::as_str).unwrap_or("");
            Some(publish_diagnostics(uri, text))
        }
        _ => None,
    }
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
    fn initialize_returns_deterministic_capabilities() {
        let response =
            handle_message(&json!({"jsonrpc":"2.0","id":7,"method":"initialize"})).unwrap();
        assert_eq!(response["id"], 7);
        assert_eq!(response["result"]["serverInfo"]["name"], "zap");
        assert_eq!(response["result"]["capabilities"]["textDocumentSync"], 1);
    }
}
