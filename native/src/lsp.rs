use serde_json::{json, Value};
#[cfg(test)]
use std::cell::RefCell;
use std::{
    collections::BTreeMap,
    fs,
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

#[derive(Debug, Clone)]
struct DocumentState {
    version: Option<i64>,
    text: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum PositionEncoding {
    Utf8,
    #[default]
    Utf16,
    Utf32,
}

impl PositionEncoding {
    fn as_str(self) -> &'static str {
        match self {
            Self::Utf8 => "utf-8",
            Self::Utf16 => "utf-16",
            Self::Utf32 => "utf-32",
        }
    }

    fn character_width(self, character: char) -> usize {
        match self {
            Self::Utf8 => character.len_utf8(),
            Self::Utf16 => character.len_utf16(),
            Self::Utf32 => 1,
        }
    }

    fn encoded_column(self, line: &str, char_index: usize) -> usize {
        line.chars()
            .take(char_index)
            .map(|character| self.character_width(character))
            .sum()
    }

    fn char_index_for_column(self, line: &str, column: usize) -> usize {
        let mut encoded: usize = 0;
        for (index, character) in line.chars().enumerate() {
            let width = self.character_width(character);
            if encoded.saturating_add(width) > column {
                return index;
            }
            encoded += width;
        }
        line.chars().count()
    }
}

const MAX_WORKSPACE_DOCUMENTS: usize = 256;
const MAX_IMPORT_DEPTH: usize = 32;
const MAX_WORKSPACE_BYTES: usize = 32 * 1024 * 1024;
const MAX_CONTENT_CHANGES: usize = 128;

#[derive(Debug, Default)]
pub struct LspState {
    documents: BTreeMap<String, DocumentState>,
    position_encoding: PositionEncoding,
}

impl LspState {
    pub fn new() -> Self {
        Self::default()
    }
}

fn accepts_document_version(current: Option<i64>, incoming: Option<i64>) -> bool {
    match (current, incoming) {
        (Some(current), Some(incoming)) => incoming > current,
        (Some(_), None) => false,
        (None, _) => true,
    }
}

fn negotiate_position_encoding(params: Option<&Value>) -> PositionEncoding {
    params
        .and_then(|params| params["capabilities"]["general"]["positionEncodings"].as_array())
        .and_then(|encodings| {
            encodings
                .iter()
                .find_map(|encoding| match encoding.as_str()? {
                    "utf-8" => Some(PositionEncoding::Utf8),
                    "utf-16" => Some(PositionEncoding::Utf16),
                    "utf-32" => Some(PositionEncoding::Utf32),
                    _ => None,
                })
        })
        .unwrap_or_default()
}

fn workspace_bytes(state: &LspState) -> usize {
    state
        .documents
        .values()
        .map(|document| document.text.len())
        .sum()
}

fn can_store_document(state: &LspState, uri: &str, text: &str) -> bool {
    let existing_bytes = state
        .documents
        .get(uri)
        .map_or(0, |document| document.text.len());
    (state.documents.contains_key(uri) || state.documents.len() < MAX_WORKSPACE_DOCUMENTS)
        && workspace_bytes(state)
            .saturating_sub(existing_bytes)
            .saturating_add(text.len())
            <= MAX_WORKSPACE_BYTES
}

fn line_start(text: &str, target_line: usize) -> Option<usize> {
    let mut line = 0;
    let mut offset = 0;
    for (index, byte) in text.bytes().enumerate() {
        if line == target_line {
            return Some(offset);
        }
        if byte == b'\n' {
            line += 1;
            offset = index + 1;
        }
    }
    (line == target_line).then_some(offset)
}

fn position_to_byte_offset(
    text: &str,
    position: &Value,
    encoding: PositionEncoding,
) -> Option<usize> {
    let line = usize::try_from(position.get("line")?.as_u64()?).ok()?;
    let character = usize::try_from(position.get("character")?.as_u64()?).ok()?;
    let start = line_start(text, line)?;
    let remaining = &text[start..];
    let line_length = remaining.find('\n').unwrap_or(remaining.len());
    let line_text = &remaining[..line_length];
    let char_index = encoding.char_index_for_column(line_text, character);
    if encoding.encoded_column(line_text, char_index) != character {
        return None;
    }
    let byte_offset = line_text
        .char_indices()
        .nth(char_index)
        .map_or(line_text.len(), |(offset, _)| offset);
    Some(start + byte_offset)
}

fn apply_content_changes(
    original: &str,
    changes: &[Value],
    encoding: PositionEncoding,
) -> Option<String> {
    if changes.is_empty() || changes.len() > MAX_CONTENT_CHANGES {
        return None;
    }
    let mut text = original.to_owned();
    for change in changes {
        let replacement = change.get("text")?.as_str()?;
        match change.get("range") {
            None => {
                if change.get("rangeLength").is_some() {
                    return None;
                }
                text.clear();
                text.push_str(replacement);
            }
            Some(range) => {
                let start = position_to_byte_offset(&text, range.get("start")?, encoding)?;
                let end = position_to_byte_offset(&text, range.get("end")?, encoding)?;
                if start > end {
                    return None;
                }
                text.replace_range(start..end, replacement);
            }
        }
        if text.len() > MAX_WORKSPACE_BYTES {
            return None;
        }
    }
    Some(text)
}

pub fn run_stdio() -> Result<(), String> {
    let stdin = io::stdin();
    let mut input = BufReader::new(stdin.lock());
    let mut output = io::BufWriter::new(io::stdout());
    let mut state = LspState::new();
    while let Some(message) = read_message(&mut input)? {
        if let Some(response) = handle_message_with_state(&message, &mut state) {
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

pub fn handle_message_with_state(message: &Value, state: &mut LspState) -> Option<Value> {
    let method = message.get("method")?.as_str()?;
    match method {
        "initialize" => {
            state.position_encoding = negotiate_position_encoding(message.get("params"));
            Some(json!({
                "jsonrpc": "2.0",
                "id": message.get("id").cloned().unwrap_or(Value::Null),
                "result": {
                    "capabilities": {
                        "textDocumentSync": {"openClose": true, "change": 2},
                        "diagnosticProvider": {"interFileDependencies": false, "workspaceDiagnostics": false},
                        "codeActionProvider": {"codeActionKinds": ["quickfix", "source", "source.organizeImports"], "resolveProvider": false},
                        "completionProvider": {"resolveProvider": false, "triggerCharacters": ["."]},
                        "signatureHelpProvider": {"triggerCharacters": ["(", ","]},
                        "hoverProvider": true,
                        "definitionProvider": true,
                        "renameProvider": true,
                        "workspaceSymbolProvider": true,
                        "documentSymbolProvider": true,
                        "documentFormattingProvider": true
                    },
                    "serverInfo": {"name": "zap", "version": env!("CARGO_PKG_VERSION")},
                    "positionEncoding": state.position_encoding.as_str()
                }
            }))
        }
        "shutdown" => Some(json!({
            "jsonrpc": "2.0",
            "id": message.get("id").cloned().unwrap_or(Value::Null),
            "result": null
        })),
        "textDocument/completion" => Some(completion_response(message, state)),
        "textDocument/signatureHelp" => Some(signature_help_response(message, state)),
        "textDocument/hover" => Some(hover_response(message, state)),
        "textDocument/definition" => Some(definition_response(message, state)),
        "textDocument/rename" => Some(rename_response(message, state)),
        "textDocument/codeAction" => Some(code_action_response(message, state)),
        "textDocument/formatting" => Some(formatting_response(message, state)),
        "workspace/symbol" => Some(workspace_symbol_response(message, state)),
        "textDocument/documentSymbol" => Some(document_symbol_response(message, state)),
        "textDocument/didOpen" => {
            let params = message.get("params")?;
            let document = params.get("textDocument")?;
            let uri = document.get("uri")?.as_str()?;
            let text = document.get("text").and_then(Value::as_str).unwrap_or("");
            let version = document.get("version").and_then(Value::as_i64);
            if !can_store_document(state, uri, text) {
                return None;
            }
            state.documents.insert(
                uri.to_string(),
                DocumentState {
                    version,
                    text: text.to_string(),
                },
            );
            Some(publish_diagnostics(uri, text, state.position_encoding))
        }
        "textDocument/didChange" => {
            let params = message.get("params")?;
            let document = params.get("textDocument")?;
            let uri = document.get("uri")?.as_str()?;
            let version = document.get("version").and_then(Value::as_i64);
            let changes = params.get("contentChanges")?.as_array()?;
            let current_version = state
                .documents
                .get(uri)
                .and_then(|document| document.version);
            if !accepts_document_version(current_version, version) {
                return None;
            }
            let current_text = state
                .documents
                .get(uri)
                .map(|document| document.text.as_str());
            if current_text.is_none() && changes.iter().any(|change| change.get("range").is_some())
            {
                return None;
            }
            let text = apply_content_changes(
                current_text.unwrap_or_default(),
                changes,
                state.position_encoding,
            )?;
            if !can_store_document(state, uri, &text) {
                return None;
            }
            state.documents.insert(
                uri.to_string(),
                DocumentState {
                    version,
                    text: text.clone(),
                },
            );
            Some(publish_diagnostics(uri, &text, state.position_encoding))
        }
        "textDocument/didClose" => {
            let uri = message["params"]["textDocument"]["uri"].as_str()?;
            state.documents.remove(uri);
            None
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

#[cfg(test)]
thread_local! {
    static TEST_STATE: RefCell<LspState> = RefCell::new(LspState::new());
}

#[cfg(test)]
pub fn handle_message(message: &Value) -> Option<Value> {
    TEST_STATE.with(|state| handle_message_with_state(message, &mut state.borrow_mut()))
}

fn completion_response(message: &Value, state: &LspState) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = state
        .documents
        .get(uri)
        .map(|document| document.text.clone())
        .unwrap_or_default();
    let position = &message["params"]["position"];
    let prefix = source_prefix(
        &source,
        position["line"].as_u64().unwrap_or(0) as usize,
        position["character"].as_u64().unwrap_or(0) as usize,
        state.position_encoding,
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
    ];
    candidates.extend(
        crate::stdlib_catalog::PUBLIC_BUILTINS
            .iter()
            .map(|builtin| (builtin.name, builtin.domain)),
    );
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

fn source_prefix(
    source: &str,
    line: usize,
    character: usize,
    encoding: PositionEncoding,
) -> String {
    let line_text = source.lines().nth(line).unwrap_or("");
    line_text
        .chars()
        .take(encoding.char_index_for_column(line_text, character))
        .collect::<String>()
        .rsplit(|value: char| !value.is_ascii_alphanumeric() && value != '_')
        .next()
        .unwrap_or("")
        .to_string()
}

fn encoded_span_range(
    source: &str,
    span: &crate::lexer::SourceSpan,
    encoding: PositionEncoding,
) -> Value {
    let line_index = span.line.saturating_sub(1);
    let line = source.lines().nth(line_index).unwrap_or("");
    let start_character = encoding.encoded_column(line, span.column.saturating_sub(1));
    let end_character = encoding.encoded_column(
        line,
        span.column.saturating_sub(1).saturating_add(span.length),
    );
    json!({
        "start": {"line": line_index as u64, "character": start_character as u64},
        "end": {"line": line_index as u64, "character": end_character as u64}
    })
}

fn signature_help_response(message: &Value, state: &LspState) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = state
        .documents
        .get(uri)
        .map(|document| document.text.clone())
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
    let signature = source
        .lines()
        .find_map(|line| {
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
        })
        .or_else(|| async_builtin_signature(callee));
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

fn async_builtin_signature(callee: &str) -> Option<Value> {
    let (label, documentation, parameters) = match callee {
        "spawn" => (
            "spawn(future)",
            "Create an executor-backed ScheduledFuture task.",
            vec![json!({"label": "future"})],
        ),
        "task_join" => (
            "task_join(future)",
            "Poll a ScheduledFuture until it completes and consume its result.",
            vec![json!({"label": "future"})],
        ),
        "task_is_ready" => (
            "task_is_ready(future)",
            "Inspect readiness without polling the task.",
            vec![json!({"label": "future"})],
        ),
        "task_cancel" => (
            "task_cancel(future)",
            "Request cooperative cancellation of a language task.",
            vec![json!({"label": "future"})],
        ),
        "task_join_timeout" => (
            "task_join_timeout(future, poll_budget)",
            "Poll a task up to the supplied budget before returning a TimedOut diagnostic.",
            vec![json!({"label": "future"}), json!({"label": "poll_budget"})],
        ),
        "async_capabilities" => (
            "async_capabilities()",
            "Report the deterministic async scheduling and production-I/O boundaries.",
            Vec::new(),
        ),
        _ => return None,
    };
    Some(json!({
        "label": label,
        "documentation": documentation,
        "parameters": parameters
    }))
}

fn async_builtin_hover(name: &str) -> Option<String> {
    async_builtin_signature(name).map(|signature| {
        format!(
            "async builtin `{name}`: {}",
            signature["documentation"]
                .as_str()
                .unwrap_or("async operation")
        )
    })
}

fn is_valid_identifier(name: &str) -> bool {
    let mut characters = name.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

#[derive(Clone, Debug)]
struct RenameBinding {
    name: String,
    line: usize,
}

#[derive(Clone, Debug)]
struct RenameScope {
    parent: Option<usize>,
    indent: usize,
    bindings: Vec<usize>,
}

#[derive(Clone, Debug, Default)]
struct RenameModel {
    bindings: Vec<RenameBinding>,
    scopes: Vec<RenameScope>,
    token_bindings: BTreeMap<usize, usize>,
    token_scopes: BTreeMap<usize, usize>,
}

fn rename_keyword(name: &str) -> bool {
    matches!(
        name,
        "let"
            | "fn"
            | "async"
            | "if"
            | "else"
            | "for"
            | "while"
            | "class"
            | "module"
            | "import"
            | "return"
            | "true"
            | "false"
            | "none"
            | "and"
            | "or"
            | "as"
            | "try"
            | "catch"
            | "raise"
            | "pass"
            | "break"
            | "continue"
    )
}

fn line_name_tokens<'a>(
    token_ids: &'a [usize],
    tokens: &'a [crate::lexer::SpannedToken],
) -> impl Iterator<Item = (usize, &'a str)> + 'a {
    token_ids
        .iter()
        .filter_map(|index| match &tokens[*index].token {
            crate::lexer::Token::Name(name) => Some((*index, name.as_str())),
            _ => None,
        })
}

fn declaration_token_after(
    token_ids: &[usize],
    tokens: &[crate::lexer::SpannedToken],
    keyword: &str,
) -> Option<usize> {
    let keyword_position = token_ids.iter().position(
        |index| matches!(&tokens[*index].token, crate::lexer::Token::Name(name) if name == keyword),
    )?;
    token_ids[keyword_position + 1..]
        .iter()
        .find(|index| {
            matches!(&tokens[**index].token, crate::lexer::Token::Name(name) if !rename_keyword(name))
        })
        .copied()
}

fn function_name_token(
    token_ids: &[usize],
    tokens: &[crate::lexer::SpannedToken],
) -> Option<usize> {
    let mut after_fn = false;
    for index in token_ids {
        match &tokens[*index].token {
            crate::lexer::Token::Name(name) if name == "fn" => after_fn = true,
            crate::lexer::Token::Name(_) if after_fn => return Some(*index),
            _ => {}
        }
    }
    None
}

fn line_opens_scope(token_ids: &[usize], tokens: &[crate::lexer::SpannedToken]) -> bool {
    let words = line_name_tokens(token_ids, tokens)
        .map(|(_, name)| name)
        .collect::<Vec<_>>();
    matches!(
        words.as_slice(),
        ["fn", ..]
            | ["async", "fn", ..]
            | ["class", ..]
            | ["if", ..]
            | ["else", ..]
            | ["for", ..]
            | ["while", ..]
            | ["try", ..]
            | ["catch", ..]
    )
}

fn indentation_width(line: &str) -> usize {
    line.chars()
        .take_while(|character| character.is_whitespace() && *character != '\n')
        .map(|character| if character == '\t' { 4 } else { 1 })
        .sum()
}

fn add_rename_binding(
    model: &mut RenameModel,
    token_index: usize,
    scope: usize,
    line: usize,
    name: &str,
) {
    let binding = model.bindings.len();
    model.bindings.push(RenameBinding {
        name: name.to_string(),
        line,
    });
    model.scopes[scope].bindings.push(binding);
    model.token_bindings.insert(token_index, binding);
    model.token_scopes.insert(token_index, scope);
}

fn rename_model(source: &str, tokens: &[crate::lexer::SpannedToken]) -> RenameModel {
    let line_count = source.lines().count().max(1).max(
        tokens
            .iter()
            .map(|token| token.span.line)
            .max()
            .unwrap_or(1),
    );
    let mut tokens_by_line = vec![Vec::new(); line_count + 1];
    for (index, token) in tokens.iter().enumerate() {
        if token.span.line <= line_count {
            tokens_by_line[token.span.line].push(index);
        }
    }

    let mut model = RenameModel {
        scopes: vec![RenameScope {
            parent: None,
            indent: 0,
            bindings: Vec::new(),
        }],
        ..RenameModel::default()
    };
    let mut scope_for_line = vec![0; line_count + 1];
    let mut block_children = vec![None; line_count + 1];
    let mut scope_stack = vec![0];
    for line in 1..=line_count {
        let line_text = source.lines().nth(line - 1).unwrap_or("");
        if line_text.trim().is_empty() {
            scope_for_line[line] = *scope_stack.last().unwrap_or(&0);
            continue;
        }
        let indent = indentation_width(line_text);
        while scope_stack.len() > 1 {
            let Some(&current_scope) = scope_stack.last() else {
                break;
            };
            if indent > model.scopes[current_scope].indent {
                break;
            }
            scope_stack.pop();
        }
        let current = *scope_stack.last().unwrap_or(&0);
        scope_for_line[line] = current;
        if line_opens_scope(&tokens_by_line[line], tokens) {
            let child = model.scopes.len();
            model.scopes.push(RenameScope {
                parent: Some(current),
                indent,
                bindings: Vec::new(),
            });
            block_children[line] = Some(child);
            scope_stack.push(child);
        }
    }
    for line in 1..=line_count {
        let current = scope_for_line[line];
        let token_ids = &tokens_by_line[line];
        for index in token_ids {
            model.token_scopes.entry(*index).or_insert(current);
        }
        if let Some(function_index) = function_name_token(token_ids, tokens) {
            if let crate::lexer::Token::Name(name) = &tokens[function_index].token {
                add_rename_binding(&mut model, function_index, current, line, name);
            }
            if let Some(child) = block_children[line] {
                let mut in_params = false;
                let mut expect_name = false;
                for index in token_ids
                    .iter()
                    .skip_while(|index| **index != function_index)
                    .skip(1)
                {
                    match &tokens[*index].token {
                        crate::lexer::Token::LParen if !in_params => {
                            in_params = true;
                            expect_name = true;
                        }
                        crate::lexer::Token::RParen if in_params => break,
                        crate::lexer::Token::Comma if in_params => expect_name = true,
                        crate::lexer::Token::Name(name)
                            if in_params && expect_name && !rename_keyword(name) =>
                        {
                            add_rename_binding(&mut model, *index, child, line, name);
                            expect_name = false;
                        }
                        _ => {}
                    }
                }
            }
        } else {
            for keyword in ["let", "class", "module", "import", "catch"] {
                let scope = current;
                if keyword == "import" {
                    let Some(as_position) = token_ids.iter().position(|index| {
                        matches!(&tokens[*index].token, crate::lexer::Token::Name(name) if name == "as")
                    }) else {
                        continue;
                    };
                    if let Some(token_index) = token_ids[as_position + 1..].iter().find(|index| {
                        matches!(&tokens[**index].token, crate::lexer::Token::Name(name) if !rename_keyword(name))
                    }) {
                        if let crate::lexer::Token::Name(name) = &tokens[*token_index].token {
                            add_rename_binding(&mut model, *token_index, scope, line, name);
                        }
                    }
                } else if let Some(token_index) =
                    declaration_token_after(token_ids, tokens, keyword)
                {
                    if let crate::lexer::Token::Name(name) = &tokens[token_index].token {
                        add_rename_binding(&mut model, token_index, scope, line, name);
                    }
                }
            }
            if let Some(for_position) = token_ids.iter().position(|index| {
                matches!(&tokens[*index].token, crate::lexer::Token::Name(name) if name == "for")
            }) {
                if let Some(token_index) = token_ids[for_position + 1..].iter().find(|index| {
                    matches!(&tokens[**index].token, crate::lexer::Token::Name(name) if !rename_keyword(name))
                }) {
                    if let crate::lexer::Token::Name(name) = &tokens[*token_index].token {
                        add_rename_binding(
                            &mut model,
                            *token_index,
                            block_children[line].unwrap_or(current),
                            line,
                            name,
                        );
                    }
                }
            }
        }
    }
    model
}

fn resolved_rename_binding(
    model: &RenameModel,
    name: &str,
    mut scope: usize,
    line: usize,
) -> Option<usize> {
    loop {
        if let Some(binding) = model.scopes[scope].bindings.iter().rev().find(|binding| {
            model.bindings[**binding].name == name && model.bindings[**binding].line <= line
        }) {
            return Some(*binding);
        }
        let parent = model.scopes[scope].parent?;
        scope = parent;
    }
}

fn token_contains_position(
    source: &str,
    token: &crate::lexer::SpannedToken,
    line: usize,
    character: usize,
    encoding: PositionEncoding,
) -> bool {
    let line_index = token.span.line.saturating_sub(1);
    if line_index != line {
        return false;
    }
    let line_text = source.lines().nth(line_index).unwrap_or("");
    let start_character = encoding.encoded_column(line_text, token.span.column.saturating_sub(1));
    let end_character = encoding.encoded_column(
        line_text,
        token
            .span
            .column
            .saturating_sub(1)
            .saturating_add(token.span.length),
    );
    character >= start_character && character <= end_character
}

fn rename_response(message: &Value, state: &LspState) -> Value {
    let id = message.get("id").cloned().unwrap_or(Value::Null);
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let new_name = message["params"]["newName"].as_str().unwrap_or("");
    let source = state
        .documents
        .get(uri)
        .map(|document| document.text.clone())
        .unwrap_or_default();
    if !is_valid_identifier(new_name) {
        return json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {"code": -32602, "message": "rename newName must be an identifier"}
        });
    }
    let line = message["params"]["position"]["line"].as_u64().unwrap_or(0) as usize;
    let character = message["params"]["position"]["character"]
        .as_u64()
        .unwrap_or(0) as usize;
    let tokens = match crate::lexer::tokenize_with_spans(&source) {
        Ok(tokens) => tokens,
        Err(error) => {
            return json!({
                "jsonrpc": "2.0",
                "id": id,
                "error": {"code": -32602, "message": format!("rename requires a valid document: {error}")}
            });
        }
    };
    let target = tokens.iter().enumerate().find_map(|(index, token)| {
        let crate::lexer::Token::Name(name) = &token.token else {
            return None;
        };
        token_contains_position(&source, token, line, character, state.position_encoding)
            .then_some((index, name.clone()))
    });
    let Some((target_index, old_name)) = target else {
        return json!({"jsonrpc": "2.0", "id": id, "result": Value::Null});
    };
    if crate::stdlib_catalog::contains(&old_name) || rename_keyword(&old_name) {
        return json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": {"code": -32602, "message": "built-in names and language keywords cannot be renamed"}
        });
    }
    let model = rename_model(&source, &tokens);
    let target_scope = model.token_scopes.get(&target_index).copied().unwrap_or(0);
    let Some(target_binding) = model
        .token_bindings
        .get(&target_index)
        .copied()
        .or_else(|| resolved_rename_binding(&model, &old_name, target_scope, line + 1))
    else {
        return json!({"jsonrpc": "2.0", "id": id, "result": Value::Null});
    };
    let edits = tokens
        .iter()
        .enumerate()
        .filter_map(|(index, token)| {
            let crate::lexer::Token::Name(name) = &token.token else {
                return None;
            };
            if name != &old_name || rename_keyword(name) {
                return None;
            }
            let previous_is_dot =
                index > 0 && matches!(tokens[index - 1].token, crate::lexer::Token::Dot);
            if previous_is_dot {
                return None;
            }
            let scope = model.token_scopes.get(&index).copied().unwrap_or(0);
            let binding = model
                .token_bindings
                .get(&index)
                .copied()
                .or_else(|| resolved_rename_binding(&model, name, scope, token.span.line));
            (binding == Some(target_binding)).then(|| {
                json!({
                    "range": encoded_span_range(&source, &token.span, state.position_encoding),
                    "newText": new_name
                })
            })
        })
        .collect::<Vec<_>>();
    json!({
        "jsonrpc": "2.0",
        "id": id,
        "result": {"changes": {(uri): edits}}
    })
}

fn hover_response(message: &Value, state: &LspState) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = state
        .documents
        .get(uri)
        .map(|document| document.text.clone())
        .unwrap_or_default();
    let line = message["params"]["position"]["line"].as_u64().unwrap_or(0) as usize;
    let character = message["params"]["position"]["character"]
        .as_u64()
        .unwrap_or(0) as usize;
    let word = source_prefix(&source, line, character, state.position_encoding);
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
                    type_params,
                    ..
                } if name == &word => {
                    let generic_suffix = if type_params.is_empty() {
                        String::new()
                    } else {
                        format!("<{}>", type_params.join(", "))
                    };
                    Some(format!(
                        "{}function `{name}`{generic_suffix} -> `{}`",
                        if *is_async { "async " } else { "" },
                        return_type.as_deref().unwrap_or("none")
                    ))
                }
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
    let description = description.or_else(|| async_builtin_hover(&word));
    let result = description
        .map(|value| json!({"contents": {"kind": "markdown", "value": value}}))
        .unwrap_or(Value::Null);
    json!({"jsonrpc": "2.0", "id": message.get("id").cloned().unwrap_or(Value::Null), "result": result})
}

fn definition_response(message: &Value, state: &LspState) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = state
        .documents
        .get(uri)
        .map(|document| document.text.clone())
        .unwrap_or_default();
    let position = &message["params"]["position"];
    let word = source_prefix(
        &source,
        position["line"].as_u64().unwrap_or(0) as usize,
        position["character"].as_u64().unwrap_or(0) as usize,
        state.position_encoding,
    );
    let locations = declaration_symbols(uri, &source, state.position_encoding)
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

fn code_action_allowed(params: &Value, kind: &str) -> bool {
    params["context"]["only"]
        .as_array()
        .map(|only| only.iter().any(|value| value.as_str() == Some(kind)))
        .unwrap_or(true)
}

fn unmatched_closing_delimiter(source: &str) -> Option<char> {
    let mut stack = Vec::new();
    let mut quote = None;
    let mut escaped = false;
    let mut comment = false;
    for character in source.chars() {
        if comment {
            if character == '\n' {
                comment = false;
            }
            continue;
        }
        if let Some(active_quote) = quote {
            if escaped {
                escaped = false;
            } else if character == '\\' {
                escaped = true;
            } else if character == active_quote {
                quote = None;
            }
            continue;
        }
        match character {
            '#' => comment = true,
            '\'' | '"' => quote = Some(character),
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                if stack.last() == Some(&character) {
                    stack.pop();
                } else if !stack.is_empty() {
                    return None;
                }
            }
            _ => {}
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}

fn insertion_edit_for_function_signature(
    source: &str,
    line_number: usize,
    encoding: PositionEncoding,
) -> Option<Value> {
    let line = source.lines().nth(line_number)?;
    let trimmed = line.trim_start();
    let declaration = trimmed
        .strip_prefix("fn ")
        .or_else(|| trimmed.strip_prefix("def "))?;
    let name_len = declaration
        .chars()
        .take_while(|character| character.is_ascii_alphanumeric() || *character == '_')
        .count();
    if name_len == 0 || declaration[name_len..].trim_start().starts_with('(') {
        return None;
    }
    let name_start = line.len() - trimmed.len();
    let insertion_char = name_start + trimmed.find(declaration)? + name_len;
    let character = encoding.encoded_column(line, insertion_char) as u64;
    Some(json!({
        "range": {
            "start": {"line": line_number as u64, "character": character},
            "end": {"line": line_number as u64, "character": character}
        },
        "newText": "()"
    }))
}

fn code_action_response(message: &Value, state: &LspState) -> Value {
    let id = message.get("id").cloned().unwrap_or(Value::Null);
    let params = &message["params"];
    let uri = params["textDocument"]["uri"].as_str().unwrap_or("");
    let source = state
        .documents
        .get(uri)
        .map(|document| document.text.as_str())
        .unwrap_or("");
    let diagnostics = params["context"]["diagnostics"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut actions = Vec::new();
    for diagnostic in diagnostics {
        let code = diagnostic["code"].as_str().unwrap_or("");
        let range = diagnostic.get("range").cloned().unwrap_or_else(|| {
            json!({
                "start": {"line": 0, "character": 0},
                "end": {"line": 0, "character": 0}
            })
        });
        let edit = |edits: Vec<Value>| {
            let mut changes = serde_json::Map::new();
            changes.insert(uri.to_string(), Value::Array(edits));
            json!({"changes": changes})
        };
        match code {
            "ZAP-STYLE-001" if code_action_allowed(params, "quickfix") => {
                actions.push(json!({
                    "title": "Replace tab with spaces",
                    "kind": "quickfix",
                    "isPreferred": true,
                    "diagnostics": [diagnostic],
                    "edit": edit(vec![json!({"range": range, "newText": "    "})])
                }));
            }
            "ZAP-STYLE-002" if code_action_allowed(params, "quickfix") => {
                actions.push(json!({
                    "title": "Remove trailing whitespace",
                    "kind": "quickfix",
                    "isPreferred": true,
                    "diagnostics": [diagnostic],
                    "edit": edit(vec![json!({"range": range, "newText": ""})])
                }));
            }
            "ZAP-SYNTAX-002" if code_action_allowed(params, "quickfix") => {
                if let Some(line) = range["start"]["line"].as_u64() {
                    if let Some(insert) = insertion_edit_for_function_signature(
                        source,
                        line as usize,
                        state.position_encoding,
                    ) {
                        actions.push(json!({
                            "title": "Add function parentheses",
                            "kind": "quickfix",
                            "isPreferred": true,
                            "diagnostics": [diagnostic],
                            "edit": edit(vec![insert])
                        }));
                    }
                }
            }
            "ZAP-SYNTAX-001" if code_action_allowed(params, "quickfix") => {
                if let Some(closing) = unmatched_closing_delimiter(source) {
                    let line = range["end"]["line"].as_u64().unwrap_or(0);
                    let character = range["end"]["character"].as_u64().unwrap_or(0);
                    actions.push(json!({
                        "title": format!("Insert missing `{closing}`"),
                        "kind": "quickfix",
                        "isPreferred": true,
                        "diagnostics": [diagnostic],
                        "edit": edit(vec![json!({
                            "range": {
                                "start": {"line": line, "character": character},
                                "end": {"line": line, "character": character}
                            },
                            "newText": closing.to_string()
                        })])
                    }));
                }
            }
            "ZAP-SYNTAX-003" if code_action_allowed(params, "quickfix") => {
                let closing = diagnostic["data"]["expectedDelimiter"]
                    .as_str()
                    .unwrap_or("");
                if [")", "]", "}"].contains(&closing) {
                    let line = range["end"]["line"].as_u64().unwrap_or(0);
                    let character = range["end"]["character"].as_u64().unwrap_or(0);
                    actions.push(json!({
                        "title": format!("Insert missing `{closing}`"),
                        "kind": "quickfix",
                        "isPreferred": true,
                        "diagnostics": [diagnostic],
                        "edit": edit(vec![json!({
                            "range": {
                                "start": {"line": line, "character": character},
                                "end": {"line": line, "character": character}
                            },
                            "newText": closing
                        })])
                    }));
                }
            }
            _ => {}
        }
    }
    json!({"jsonrpc": "2.0", "id": id, "result": actions})
}

fn formatting_response(message: &Value, state: &LspState) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = state
        .documents
        .get(uri)
        .map(|document| document.text.clone())
        .unwrap_or_default();
    let formatted = format_source(&source);
    let end_line = source.lines().count().saturating_sub(1) as u64;
    let end_character = source
        .lines()
        .last()
        .map(|line| {
            state
                .position_encoding
                .encoded_column(line, line.chars().count())
        })
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

fn workspace_symbol_response(message: &Value, state: &LspState) -> Value {
    let query = message["params"]["query"].as_str().unwrap_or("");
    let documents = workspace_documents(state);
    let symbols = documents
        .iter()
        .flat_map(|(uri, source)| {
            declaration_symbols(uri, source, state.position_encoding)
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

fn workspace_documents(state: &LspState) -> Vec<(String, String)> {
    const MAX_MODULE_BYTES: u64 = 8 * 1024 * 1024;
    let mut documents = state.documents.clone();
    let mut pending = documents
        .keys()
        .cloned()
        .map(|uri| (uri, 0usize))
        .collect::<Vec<_>>();
    let mut cursor = 0;
    while cursor < pending.len() {
        let (uri, depth) = pending[cursor].clone();
        cursor += 1;
        if depth >= MAX_IMPORT_DEPTH {
            continue;
        }
        let Some(source) = documents.get(&uri).map(|document| document.text.clone()) else {
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
            if documents.contains_key(&module_uri) || documents.len() >= MAX_WORKSPACE_DOCUMENTS {
                continue;
            }
            let indexed_bytes = documents
                .values()
                .map(|document| document.text.len())
                .sum::<usize>();
            if indexed_bytes.saturating_add(module_source.len()) > MAX_WORKSPACE_BYTES {
                continue;
            }
            documents.insert(
                module_uri.clone(),
                DocumentState {
                    version: None,
                    text: module_source,
                },
            );
            pending.push((module_uri, depth + 1));
        }
    }
    documents
        .into_iter()
        .map(|(uri, document)| (uri, document.text))
        .collect()
}

fn percent_decode_uri_path(encoded: &str) -> Option<String> {
    let bytes = encoded.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' {
            if index + 2 >= bytes.len() {
                return None;
            }
            let high = (bytes[index + 1] as char).to_digit(16)? as u8;
            let low = (bytes[index + 2] as char).to_digit(16)? as u8;
            decoded.push(high * 16 + low);
            index += 3;
        } else {
            decoded.push(bytes[index]);
            index += 1;
        }
    }
    String::from_utf8(decoded).ok()
}

fn file_uri_path(uri: &str) -> Option<PathBuf> {
    let path = uri.strip_prefix("file://")?;
    if !path.starts_with('/') || path.starts_with("//") {
        return None;
    }
    let decoded = percent_decode_uri_path(path)?;
    if decoded.contains('\0') || decoded.split('/').any(|component| component == "..") {
        return None;
    }
    #[cfg(windows)]
    if decoded.len() >= 3
        && decoded.as_bytes()[0] == b'/'
        && decoded.as_bytes()[2] == b':'
        && decoded.as_bytes()[1].is_ascii_alphabetic()
    {
        return Some(PathBuf::from(&decoded[1..]));
    }
    Some(PathBuf::from(decoded))
}

fn percent_encode_uri_path(path: &str) -> String {
    path.bytes()
        .map(|byte| match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'/' | b':' => {
                (byte as char).to_string()
            }
            other => format!("%{other:02X}"),
        })
        .collect()
}

fn path_to_file_uri(path: &Path) -> String {
    let rendered = path.to_string_lossy().replace('\\', "/");
    if rendered.starts_with('/') {
        format!("file://{}", percent_encode_uri_path(&rendered))
    } else {
        format!("file:///{}", percent_encode_uri_path(&rendered))
    }
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
fn declaration_symbols(
    uri: &str,
    source: &str,
    encoding: PositionEncoding,
) -> Vec<(String, u32, Value, String)> {
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
            let start_character = encoding.encoded_column(line, column);
            let end_character =
                encoding.encoded_column(line, column.saturating_add(name.chars().count()));
            let range = json!({
                "start": {"line": start_line, "character": start_character as u64},
                "end": {"line": start_line, "character": end_character as u64}
            });
            Some((name, kind, range, format!("{detail} in {uri}")))
        })
        .collect()
}

fn document_symbol_response(message: &Value, state: &LspState) -> Value {
    let uri = message["params"]["textDocument"]["uri"]
        .as_str()
        .unwrap_or("");
    let source = state
        .documents
        .get(uri)
        .map(|document| document.text.clone())
        .unwrap_or_default();
    let symbols = crate::ast::parse_program(&source)
        .map(|program| {
            document_symbols_for_program(uri, &source, &program, state.position_encoding)
        })
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
    encoding: PositionEncoding,
) -> Vec<Value> {
    program
        .statements
        .iter()
        .filter_map(|statement| document_symbol_for_statement(uri, source, statement, encoding))
        .collect()
}

fn document_symbol_for_statement(
    uri: &str,
    source: &str,
    statement: &crate::ast::Spanned<crate::ast::Stmt>,
    encoding: PositionEncoding,
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
    let range = symbol_range(source, &statement.span, &name, encoding);
    let children = match &statement.node {
        crate::ast::Stmt::Function { body, .. } | crate::ast::Stmt::Class { body, .. } => {
            document_symbols_for_program(uri, source, body, encoding)
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

fn symbol_range(
    source: &str,
    span: &crate::lexer::SourceSpan,
    name: &str,
    encoding: PositionEncoding,
) -> Value {
    let line_index = span.line.saturating_sub(1);
    let line = source.lines().nth(line_index).unwrap_or("");
    let column = line.find(name).unwrap_or(span.column.saturating_sub(1));
    let start = json!({
        "line": line_index as u64,
        "character": encoding.encoded_column(line, column) as u64
    });
    let end = json!({
        "line": line_index as u64,
        "character": encoding.encoded_column(line, column + name.chars().count()) as u64
    });
    json!({"start": start, "end": end})
}

fn quoted_token(message: &str) -> Option<&str> {
    let start = message.find('\'')? + 1;
    let rest = &message[start..];
    let end = rest.find('\'')?;
    Some(&rest[..end])
}

fn diagnostic_fix_ids(code: &str) -> Vec<&'static str> {
    match code {
        "ZAP-STYLE-001" => vec!["zap.replace-tabs"],
        "ZAP-STYLE-002" => vec!["zap.remove-trailing-whitespace"],
        "ZAP-SYNTAX-002" => vec!["zap.add-parentheses"],
        "ZAP-SYNTAX-003" => vec!["zap.close-delimiter"],
        _ => Vec::new(),
    }
}

fn diagnostic_range(
    source: &str,
    raw: &str,
    diagnostic: &crate::diagnostics::ZapError,
    line_number: usize,
    column_number: usize,
    encoding: PositionEncoding,
) -> Value {
    let line = line_number.saturating_sub(1);
    let line_text = source.lines().nth(line).unwrap_or("");
    let line_chars = line_text.chars().count();
    let mut start = column_number.saturating_sub(1).min(line_chars);
    let mut length = 1usize;
    if raw.starts_with("line ") && raw.contains("tabs are not allowed") {
        start = line_text
            .chars()
            .position(|character| character == '\t')
            .unwrap_or(start);
    } else if raw.starts_with("line ") && raw.contains("trailing whitespace") {
        start = line_text
            .trim_end_matches(char::is_whitespace)
            .chars()
            .count();
        length = line_chars.saturating_sub(start).max(1);
    } else if raw.starts_with("line ") && raw.contains("line exceeds 120 characters") {
        start = 120.min(line_chars);
        length = line_chars.saturating_sub(start).max(1);
    } else if let Some(token) = quoted_token(diagnostic.message()) {
        if let Some(byte_index) = line_text.find(token) {
            start = line_text[..byte_index].chars().count();
            length = token.chars().count().max(1);
        }
    }
    let character = encoding.encoded_column(line_text, start);
    let end = encoding.encoded_column(line_text, (start + length).min(line_chars));
    json!({
        "start": {"line": line as u64, "character": character as u64},
        "end": {"line": line as u64, "character": end.max(character + 1) as u64}
    })
}

fn publish_diagnostics(uri: &str, source: &str, encoding: PositionEncoding) -> Value {
    let file = file_uri_path(uri).unwrap_or_else(|| PathBuf::from("<lsp>"));
    let diagnostics = crate::source_diagnostics(source, &file)
        .into_iter()
        .map(|raw| {
            let diagnostic = crate::diagnostics::ZapError::from_message(raw.clone());
            let (_, _, parsed_line, parsed_column) = diagnostic.parts();
            let line_number = if parsed_line == 0 {
                diagnostic_line(&raw).unwrap_or(1)
            } else {
                parsed_line
            };
            let column_number = if parsed_column == 0 { 1 } else { parsed_column };
            let (code, severity, severity_name) = if raw.starts_with("line ") && raw.contains("tabs are not allowed") {
                ("ZAP-STYLE-001", 2, "warning")
            } else if raw.starts_with("line ") && raw.contains("trailing whitespace") {
                ("ZAP-STYLE-002", 2, "warning")
            } else if raw.starts_with("line ") && raw.contains("line exceeds 120 characters") {
                ("ZAP-STYLE-003", 2, "warning")
            } else {
                (diagnostic.code(), 1, diagnostic.severity())
            };
            let fix_ids = diagnostic_fix_ids(code);
            json!({
                "range": diagnostic_range(source, &raw, &diagnostic, line_number, column_number, encoding),
                "severity": severity,
                "source": "zap",
                "code": code,
                "message": diagnostic.message(),
                "data": {
                    "kind": diagnostic.kind(),
                    "code": code,
                    "severity": severity_name,
                    "notes": diagnostic.notes(),
                    "help": diagnostic.help(),
                    "fixIds": fix_ids
                }
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
    use super::{
        can_store_document, decode_messages, file_uri_path, handle_message,
        handle_message_with_state, path_to_file_uri, LspState, MAX_WORKSPACE_BYTES,
        MAX_WORKSPACE_DOCUMENTS,
    };
    use serde_json::{json, Value};

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
    fn lsp_diagnostics_match_cli_type_error_contract() {
        let response = handle_message(&json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": "file:///conditional.zp", "text": r#"let value: number = if true then 1 else "bad"
"#}}
        }))
        .unwrap();
        let diagnostic = &response["params"]["diagnostics"][0];
        assert_eq!(diagnostic["code"], "ZAP-TYPE-001");
        assert_eq!(diagnostic["severity"], 1);
        assert_eq!(diagnostic["data"]["code"], "ZAP-TYPE-001");
        assert_eq!(diagnostic["data"]["kind"], "TypeError");
        assert_eq!(diagnostic["data"]["severity"], "error");
        assert_eq!(
            diagnostic["data"]["notes"][0],
            "Check the expression type and the expected annotation."
        );
        assert_eq!(
            diagnostic["data"]["help"],
            "Use a compatible value or update the type annotation."
        );
        assert_eq!(diagnostic["range"]["start"]["line"], 0);
        assert_eq!(diagnostic["range"]["start"]["character"], 0);
        assert!(diagnostic["message"]
            .as_str()
            .unwrap()
            .contains("conditional branches must have compatible types"));
    }

    #[test]
    fn code_actions_offer_safe_style_and_signature_fixes() {
        let style_uri = "file:///code-actions-style.zp";
        let style_response = handle_message(&json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": style_uri, "text": "let value = 1\t  \n"}}
        }))
        .unwrap();
        assert_eq!(style_response["params"]["diagnostics"][0]["severity"], 2);
        assert_eq!(
            style_response["params"]["diagnostics"][0]["code"],
            "ZAP-STYLE-001"
        );
        assert_eq!(
            style_response["params"]["diagnostics"][0]["range"]["start"]["character"],
            13
        );
        let style_actions = handle_message(&json!({
            "jsonrpc": "2.0",
            "id": 61,
            "method": "textDocument/codeAction",
            "params": {
                "textDocument": {"uri": style_uri},
                "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 20}},
                "context": {"diagnostics": style_response["params"]["diagnostics"].clone(), "only": ["quickfix"]}
            }
        }))
        .unwrap();
        assert!(style_actions["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|action| action["title"] == "Replace tab with spaces"));
        assert!(style_actions["result"]
            .as_array()
            .unwrap()
            .iter()
            .any(|action| action["title"] == "Remove trailing whitespace"));

        let signature_uri = "file:///code-actions-signature.zp";
        let signature_response = handle_message(&json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": signature_uri, "text": "fn add:\n  say(1)\n"}}
        }))
        .unwrap();
        assert_eq!(
            signature_response["params"]["diagnostics"][0]["code"],
            "ZAP-SYNTAX-002"
        );
        let signature_actions = handle_message(&json!({
            "jsonrpc": "2.0",
            "id": 62,
            "method": "textDocument/codeAction",
            "params": {
                "textDocument": {"uri": signature_uri},
                "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 8}},
                "context": {"diagnostics": signature_response["params"]["diagnostics"].clone()}
            }
        }))
        .unwrap();
        assert_eq!(
            signature_actions["result"][0]["title"],
            "Add function parentheses"
        );
        assert_eq!(
            signature_actions["result"][0]["edit"]["changes"][signature_uri][0]["newText"],
            "()"
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
        for expected in [
            "spawn",
            "task_join",
            "task_is_ready",
            "task_cancel",
            "task_join_timeout",
        ] {
            assert!(
                labels.contains(&expected),
                "missing completion item: {expected}"
            );
        }
        assert_eq!(
            labels.len(),
            12 + crate::stdlib_catalog::PUBLIC_BUILTINS.len()
        );
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
        let completion_labels = response["result"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|item| item["label"].as_str())
            .collect::<Vec<_>>();
        assert!(completion_labels.contains(&"load"));
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
    fn hover_includes_generic_function_type_parameters() {
        let uri = "file:///generic-hover.zp";
        let _ = handle_message(&json!({
            "jsonrpc": "2.0",
            "method": "textDocument/didOpen",
            "params": {"textDocument": {"uri": uri, "text": "fn identity<T>(value: T) -> T:\n    return value\nidentity(1)\n"}}
        }));
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 101, "method": "textDocument/hover",
            "params": {"textDocument": {"uri": uri}, "position": {"line": 2, "character": 8}}
        }))
        .unwrap();
        assert_eq!(
            response["result"]["contents"]["value"],
            "function `identity`<T> -> `T`"
        );
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
        let uri = path_to_file_uri(&main);
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
    fn independent_lsp_states_do_not_share_documents() {
        let mut first = LspState::new();
        let mut second = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": "file:///first.zp", "text": "fn first():\n    return 1\n"}}
            }),
            &mut first,
        );
        let first_symbols = handle_message_with_state(
            &json!({"jsonrpc": "2.0", "id": 30, "method": "workspace/symbol", "params": {"query": ""}}),
            &mut first,
        )
        .expect("first state should respond");
        let second_symbols = handle_message_with_state(
            &json!({"jsonrpc": "2.0", "id": 31, "method": "workspace/symbol", "params": {"query": ""}}),
            &mut second,
        )
        .expect("second state should respond");
        assert!(!first_symbols["result"].as_array().unwrap().is_empty());
        assert!(second_symbols["result"].as_array().unwrap().is_empty());
    }

    #[test]
    fn rename_returns_span_edits_without_rewriting_string_literals() {
        let uri = "file:///rename.zp";
        let source =
            "fn greet(name):\n    let copy = name\n    return name\nlet label = \"greet\"\n";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "text": source}}
            }),
            &mut state,
        );
        let response = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0",
                "id": 40,
                "method": "textDocument/rename",
                "params": {
                    "textDocument": {"uri": uri},
                    "position": {"line": 1, "character": 8},
                    "newName": "value"
                }
            }),
            &mut state,
        )
        .unwrap();
        let edits = response["result"]["changes"]
            .get(uri)
            .and_then(Value::as_array)
            .unwrap();
        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0]["newText"], "value");
        assert_eq!(edits[0]["range"]["start"]["line"], 1);
        assert_eq!(edits[0]["range"]["start"]["character"], 8);
        assert_eq!(response["id"], 40);
    }

    #[test]
    fn rename_rejects_invalid_names_and_builtin_targets() {
        let uri = "file:///rename-errors.zp";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "text": "let value = task_join(1)\n"}}
            }),
            &mut state,
        );
        let invalid = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 41, "method": "textDocument/rename",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 4}, "newName": "bad-name"}
            }),
            &mut state,
        )
        .unwrap();
        assert_eq!(invalid["error"]["code"], -32602);
        let builtin = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 42, "method": "textDocument/rename",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 16}, "newName": "join"}
            }),
            &mut state,
        )
        .unwrap();
        assert_eq!(builtin["error"]["code"], -32602);
    }

    #[test]
    fn rename_resolves_outer_binding_without_renaming_inner_shadow() {
        let uri = "file:///rename-shadow.zp";
        let source = "let value = 1\nfn demo():\n    let value = 2\n    say value\nsay value\n";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "text": source}}
            }),
            &mut state,
        );
        let response = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 49, "method": "textDocument/rename",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 5}, "newName": "outer_value"}
            }),
            &mut state,
        )
        .unwrap();
        let edits = response["result"]["changes"][uri].as_array().unwrap();
        assert_eq!(edits.len(), 2);
        assert_eq!(edits[0]["range"]["start"]["line"], 0);
        assert_eq!(edits[1]["range"]["start"]["line"], 4);
    }

    #[test]
    fn rename_resolves_parameters_through_nested_closure_scopes() {
        let uri = "file:///rename-closure.zp";
        let source = "fn greet(value):\n    fn nested():\n        return value\n    return value\n";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "text": source}}
            }),
            &mut state,
        );
        let response = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 50, "method": "textDocument/rename",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 10}, "newName": "input"}
            }),
            &mut state,
        )
        .unwrap();
        let edits = response["result"]["changes"][uri].as_array().unwrap();
        assert_eq!(edits.len(), 3);
        assert!(edits.iter().all(|edit| edit["newText"] == "input"));
    }

    #[test]
    fn rename_does_not_edit_comments_or_string_literals() {
        let uri = "file:///rename-literals.zp";
        let source = "let value = 1\n# value\nsay \"value\"\nsay value\n";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "text": source}}
            }),
            &mut state,
        );
        let response = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 51, "method": "textDocument/rename",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 5}, "newName": "renamed"}
            }),
            &mut state,
        )
        .unwrap();
        let edits = response["result"]["changes"][uri].as_array().unwrap();
        assert_eq!(edits.len(), 2);
        assert!(edits.iter().all(|edit| edit["range"]["start"]["line"] != 1));
        assert!(edits.iter().all(|edit| edit["range"]["start"]["line"] != 2));
    }

    #[test]
    fn rename_preserves_import_path_and_updates_alias_references() {
        let uri = "file:///rename-import.zp";
        let source = "import app.util as helper\nsay helper\n";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "text": source}}
            }),
            &mut state,
        );
        let response = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 53, "method": "textDocument/rename",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 19}, "newName": "utils"}
            }),
            &mut state,
        )
        .unwrap();
        let edits = response["result"]["changes"][uri].as_array().unwrap();
        assert_eq!(edits.len(), 2);
        assert_eq!(edits[0]["range"]["start"]["character"], 19);
        assert_eq!(edits[1]["range"]["start"]["line"], 1);
    }

    #[test]
    fn rename_after_full_sync_uses_updated_document_state() {
        let uri = "file:///rename-after-change.zp";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "version": 1, "text": "let old_name = 1\nsay old_name\n"}}
            }),
            &mut state,
        );
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didChange",
                "params": {
                    "textDocument": {"uri": uri, "version": 2},
                    "contentChanges": [{"text": "let new_name = 1\nsay new_name\n"}]
                }
            }),
            &mut state,
        );
        let response = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 52, "method": "textDocument/rename",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 5}, "newName": "renamed"}
            }),
            &mut state,
        )
        .unwrap();
        let edits = response["result"]["changes"][uri].as_array().unwrap();
        assert_eq!(edits.len(), 2);
        assert_eq!(edits[0]["range"]["start"]["line"], 0);
        assert_eq!(edits[1]["range"]["start"]["line"], 1);
    }

    #[test]
    fn async_builtins_have_hover_and_signature_metadata() {
        let uri = "file:///async-lsp.zp";
        let source = "task_join_timeout(handle, 1)\n";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "text": source}}
            }),
            &mut state,
        );
        let hover = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 43, "method": "textDocument/hover",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 17}}
            }),
            &mut state,
        )
        .unwrap();
        assert!(hover["result"]["contents"]["value"]
            .as_str()
            .unwrap()
            .contains("async builtin `task_join_timeout`"));
        let signature = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 44, "method": "textDocument/signatureHelp",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 28}}
            }),
            &mut state,
        )
        .unwrap();
        assert_eq!(
            signature["result"]["signatures"][0]["label"],
            "task_join_timeout(future, poll_budget)"
        );
        assert_eq!(signature["result"]["activeParameter"], 1);
    }

    #[test]
    fn did_close_removes_document_from_workspace_symbols() {
        let uri = "file:///close.zp";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "text": "fn close_me():\n    return 1\n"}}
            }),
            &mut state,
        );
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didClose",
                "params": {"textDocument": {"uri": uri}}
            }),
            &mut state,
        );
        let response = handle_message_with_state(
            &json!({"jsonrpc": "2.0", "id": 45, "method": "workspace/symbol", "params": {"query": "close_me"}}),
            &mut state,
        )
        .unwrap();
        assert!(response["result"].as_array().unwrap().is_empty());
    }

    #[test]
    fn did_change_full_sync_uses_content_changes_and_updates_symbols() {
        let uri = "file:///sync-symbols.zp";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "version": 1, "text": "fn first():\n    return 1\n"}}
            }),
            &mut state,
        );
        let change = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didChange",
                "params": {
                    "textDocument": {"uri": uri, "version": 2},
                    "contentChanges": [{"text": "fn second():\n    return 2\n"}]
                }
            }),
            &mut state,
        )
        .expect("accepted full-sync change should publish diagnostics");
        assert_eq!(change["params"]["uri"], uri);
        let symbols = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 47, "method": "textDocument/documentSymbol",
                "params": {"textDocument": {"uri": uri}}
            }),
            &mut state,
        )
        .unwrap();
        let names = symbols["result"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|symbol| symbol["name"].as_str())
            .collect::<Vec<_>>();
        assert_eq!(names, vec!["second"]);
    }

    #[test]
    fn did_change_full_sync_publishes_diagnostics_from_new_text() {
        let uri = "file:///sync-diagnostics.zp";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "version": 1, "text": "let value: number = 1\n"}}
            }),
            &mut state,
        );
        let change = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didChange",
                "params": {
                    "textDocument": {"uri": uri, "version": 2},
                    "contentChanges": [{"text": "let value: number = \"bad\"\n"}]
                }
            }),
            &mut state,
        )
        .unwrap();
        assert_eq!(change["params"]["diagnostics"][0]["code"], "ZAP-TYPE-001");
    }

    #[test]
    fn did_change_rejects_stale_versions_without_replacing_document() {
        let uri = "file:///sync-version.zp";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "version": 2, "text": "fn current():\n    return 1\n"}}
            }),
            &mut state,
        );
        assert!(handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didChange",
                "params": {
                    "textDocument": {"uri": uri, "version": 1},
                    "contentChanges": [{"text": "fn stale():\n    return 1\n"}]
                }
            }),
            &mut state,
        )
        .is_none());
        let symbols = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 48, "method": "textDocument/documentSymbol",
                "params": {"textDocument": {"uri": uri}}
            }),
            &mut state,
        )
        .unwrap();
        assert_eq!(symbols["result"][0]["name"], "current");
    }

    #[test]
    fn did_change_rejects_unversioned_update_after_versioned_open() {
        let uri = "file:///sync-unversioned.zp";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "version": 1, "text": "fn current():\n    return 1\n"}}
            }),
            &mut state,
        );
        assert!(handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didChange",
                "params": {
                    "textDocument": {"uri": uri},
                    "contentChanges": [{"text": "fn unsafe():\n    return 1\n"}]
                }
            }),
            &mut state,
        )
        .is_none());
    }

    #[test]
    fn did_change_applies_incremental_payload_and_updates_symbols() {
        let uri = "file:///sync-incremental.zp";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "version": 1, "text": "fn current():\n    return 1\n"}}
            }),
            &mut state,
        );
        let change = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didChange",
                "params": {
                    "textDocument": {"uri": uri, "version": 2},
                    "contentChanges": [{
                        "range": {"start": {"line": 0, "character": 3}, "end": {"line": 0, "character": 10}},
                        "rangeLength": 7,
                        "text": "updated"
                    }]
                }
            }),
            &mut state,
        )
        .expect("incremental change should publish diagnostics");
        assert_eq!(change["params"]["uri"], uri);
        let symbols = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 49, "method": "textDocument/documentSymbol",
                "params": {"textDocument": {"uri": uri}}
            }),
            &mut state,
        )
        .unwrap();
        assert_eq!(symbols["result"][0]["name"], "updated");
    }

    #[test]
    fn incremental_changes_respect_utf16_boundaries_and_apply_sequentially() {
        let source = "let value = \"😀\"\n";
        let changes = vec![
            json!({
                "range": {"start": {"line": 0, "character": 13}, "end": {"line": 0, "character": 15}},
                "rangeLength": 2,
                "text": "ok"
            }),
            json!({
                "range": {"start": {"line": 0, "character": 4}, "end": {"line": 0, "character": 9}},
                "rangeLength": 5,
                "text": "result"
            }),
        ];
        let updated =
            super::apply_content_changes(source, &changes, super::PositionEncoding::Utf16)
                .expect("valid UTF-16 changes should apply");
        assert_eq!(updated, "let result = \"ok\"\n");

        let invalid = vec![json!({
            "range": {"start": {"line": 0, "character": 14}, "end": {"line": 0, "character": 15}},
            "text": "x"
        })];
        assert!(
            super::apply_content_changes(source, &invalid, super::PositionEncoding::Utf16)
                .is_none()
        );
    }

    #[test]
    fn initialize_negotiates_supported_position_encoding() {
        let mut state = LspState::new();
        let response = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0",
                "id": 54,
                "method": "initialize",
                "params": {"capabilities": {"general": {"positionEncodings": ["utf-8", "utf-16"]}}}
            }),
            &mut state,
        )
        .unwrap();
        assert_eq!(response["result"]["positionEncoding"], "utf-8");
    }

    #[test]
    fn file_uri_path_decodes_safe_paths_and_rejects_unsafe_forms() {
        assert_eq!(
            file_uri_path("file:///tmp/Zap%20source.zp")
                .unwrap()
                .to_string_lossy(),
            "/tmp/Zap source.zp"
        );
        assert!(file_uri_path("file:///tmp/%2E%2E/secret.zp").is_none());
        assert!(file_uri_path("file://host/tmp/source.zp").is_none());
        assert!(file_uri_path("file:///tmp/%GG").is_none());
    }

    #[test]
    fn utf16_position_encoding_is_used_for_unicode_rename_ranges() {
        let uri = "file:///unicode-rename.zp";
        let source = "let value = 1\nsay \"😀\" + value\n";
        let mut state = LspState::new();
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 55, "method": "initialize",
                "params": {"capabilities": {"general": {"positionEncodings": ["utf-16"]}}}
            }),
            &mut state,
        );
        let _ = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": uri, "text": source}}
            }),
            &mut state,
        );
        let response = handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "id": 56, "method": "textDocument/rename",
                "params": {"textDocument": {"uri": uri}, "position": {"line": 0, "character": 5}, "newName": "renamed"}
            }),
            &mut state,
        )
        .unwrap();
        let edits = response["result"]["changes"][uri].as_array().unwrap();
        assert_eq!(edits.len(), 2);
        assert_eq!(edits[1]["range"]["start"]["character"], 11);
    }

    #[test]
    fn workspace_document_cap_rejects_new_open_documents() {
        let mut state = LspState::new();
        for index in 0..MAX_WORKSPACE_DOCUMENTS {
            let uri = format!("file:///bounded-{index}.zp");
            assert!(handle_message_with_state(
                &json!({
                    "jsonrpc": "2.0", "method": "textDocument/didOpen",
                    "params": {"textDocument": {"uri": uri, "text": "let value = 1\n"}}
                }),
                &mut state,
            )
            .is_some());
        }
        assert!(handle_message_with_state(
            &json!({
                "jsonrpc": "2.0", "method": "textDocument/didOpen",
                "params": {"textDocument": {"uri": "file:///bounded-overflow.zp", "text": "let value = 1\n"}}
            }),
            &mut state,
        )
        .is_none());
        assert_eq!(state.documents.len(), MAX_WORKSPACE_DOCUMENTS);
    }

    #[test]
    fn workspace_byte_cap_rejects_oversized_document() {
        let state = LspState::new();
        let oversized = "x".repeat(MAX_WORKSPACE_BYTES + 1);
        assert!(!can_store_document(
            &state,
            "file:///oversized.zp",
            &oversized
        ));
    }

    #[test]
    fn initialize_advertises_rename_provider() {
        let response = handle_message(&json!({
            "jsonrpc": "2.0", "id": 46, "method": "initialize", "params": {}
        }))
        .unwrap();
        assert_eq!(response["result"]["capabilities"]["renameProvider"], true);
        assert_eq!(
            response["result"]["capabilities"]["codeActionProvider"]["codeActionKinds"],
            json!(["quickfix", "source", "source.organizeImports"])
        );
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
        assert_eq!(
            response["result"]["capabilities"]["textDocumentSync"]["change"],
            2
        );
        assert_eq!(
            response["result"]["capabilities"]["textDocumentSync"]["openClose"],
            true
        );
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
