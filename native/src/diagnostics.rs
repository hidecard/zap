use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ZapError {
    Syntax {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    Name {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    Type {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    Value {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    Io {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    FileNotFound {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    Key {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    Permission {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    Overflow {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    Runtime {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
    Project {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
}

impl ZapError {
    pub(crate) fn from_message(message: impl Into<String>) -> Self {
        let message = redact_sensitive(&message.into());
        let (file, line, column) = diagnostic_location(&message);
        let kind = if message.starts_with("SyntaxError")
            || message.contains("unexpected character")
            || message.contains("unexpected token")
        {
            "SyntaxError"
        } else if message.starts_with("NameError")
            || message.contains("unknown name")
            || message.contains("undefined")
        {
            "NameError"
        } else if message.starts_with("TypeError")
            || message.contains("expects")
            || message.contains("expected ")
        {
            "TypeError"
        } else if message.contains("key not found") || message.contains("property not found") {
            "KeyError"
        } else if message.contains("not found") || message.contains("No such file") {
            "FileNotFound"
        } else if message.contains("permission denied") || message.contains("Permission denied") {
            "PermissionError"
        } else if message.contains("overflow") || message.contains("exceeded") {
            "OverflowError"
        } else if message.starts_with("Error:")
            || message.starts_with("uncaught error")
            || message.starts_with("raised error")
        {
            "Error"
        } else if message.contains("cannot read")
            || message.contains("cannot write")
            || message.contains("I/O")
        {
            "IOError"
        } else if message.starts_with("ValueError") || message.contains("cannot ") {
            "ValueError"
        } else {
            "ProjectError"
        };
        Self::with_kind(kind, message, file, line, column)
    }

    fn with_kind(kind: &str, message: String, file: String, line: usize, column: usize) -> Self {
        match kind {
            "SyntaxError" => Self::Syntax {
                message,
                file,
                line,
                column,
            },
            "NameError" => Self::Name {
                message,
                file,
                line,
                column,
            },
            "TypeError" => Self::Type {
                message,
                file,
                line,
                column,
            },
            "ValueError" => Self::Value {
                message,
                file,
                line,
                column,
            },
            "IOError" => Self::Io {
                message,
                file,
                line,
                column,
            },
            "FileNotFound" => Self::FileNotFound {
                message,
                file,
                line,
                column,
            },
            "KeyError" => Self::Key {
                message,
                file,
                line,
                column,
            },
            "PermissionError" => Self::Permission {
                message,
                file,
                line,
                column,
            },
            "OverflowError" => Self::Overflow {
                message,
                file,
                line,
                column,
            },
            "Error" => Self::Runtime {
                message,
                file,
                line,
                column,
            },
            _ => Self::Project {
                message,
                file,
                line,
                column,
            },
        }
    }

    pub(crate) fn severity(&self) -> &'static str {
        "error"
    }

    /// Stable machine-readable identifier for the diagnostic contract.
    ///
    /// The identifier is intentionally distinct from `kind`: `kind` remains the
    /// user-facing category while `code` is the compatibility key consumed by
    /// editors and CI integrations.
    pub(crate) fn code(&self) -> &'static str {
        match self {
            Self::Syntax { .. } => "ZAP-SYNTAX-001",
            Self::Name { .. } => "ZAP-NAME-001",
            Self::Type { .. } => "ZAP-TYPE-001",
            Self::Value { .. } => "ZAP-VALUE-001",
            Self::Io { .. } => "ZAP-IO-001",
            Self::FileNotFound { .. } => "ZAP-FILE-001",
            Self::Key { .. } => "ZAP-KEY-001",
            Self::Permission { .. } => "ZAP-PERM-001",
            Self::Overflow { .. } => "ZAP-OVERFLOW-001",
            Self::Runtime { .. } => "ZAP-RUNTIME-001",
            Self::Project { .. } => "ZAP-PROJECT-001",
        }
    }

    /// Canonical field ordering for machine-readable CLI diagnostics.
    pub(crate) fn json_fields(&self) -> String {
        let (_, file, line, column) = self.parts();
        let notes = self
            .notes()
            .into_iter()
            .map(|note| format!("\"{}\"", json_escape(&note)))
            .collect::<Vec<_>>();
        let help = self.help().map_or_else(
            || "null".to_string(),
            |value| format!("\"{}\"", json_escape(value)),
        );
        format!(
            "\"code\":\"{}\",\"kind\":\"{}\",\"severity\":\"{}\",\"file\":\"{}\",\"line\":{},\"column\":{},\"message\":\"{}\",\"notes\":[{}],\"help\":{},\"error\":\"{}\"",
            self.code(),
            self.kind(),
            self.severity(),
            json_escape(file),
            line,
            column,
            json_escape(self.message()),
            notes.join(","),
            help,
            json_escape(&self.to_string())
        )
    }

    pub(crate) fn notes(&self) -> Vec<String> {
        match self {
            Self::Type { .. } => {
                vec!["Check the expression type and the expected annotation.".to_string()]
            }
            Self::Syntax { .. } => vec!["Check the surrounding syntax and delimiters.".to_string()],
            Self::Name { .. } => {
                vec!["Check that the name is declared in the current scope.".to_string()]
            }
            _ => Vec::new(),
        }
    }

    pub(crate) fn help(&self) -> Option<&'static str> {
        match self {
            Self::Type { .. } => Some("Use a compatible value or update the type annotation."),
            Self::Syntax { .. } => Some("Review the Zap syntax guide for the expected form."),
            Self::Name { .. } => Some("Declare the name before using it, or correct its spelling."),
            _ => None,
        }
    }

    pub(crate) fn kind(&self) -> &'static str {
        match self {
            Self::Syntax { .. } => "SyntaxError",
            Self::Name { .. } => "NameError",
            Self::Type { .. } => "TypeError",
            Self::Value { .. } => "ValueError",
            Self::Io { .. } => "IOError",
            Self::FileNotFound { .. } => "FileNotFound",
            Self::Key { .. } => "KeyError",
            Self::Permission { .. } => "PermissionError",
            Self::Overflow { .. } => "OverflowError",
            Self::Runtime { .. } => "Error",
            Self::Project { .. } => "ProjectError",
        }
    }

    pub(crate) fn message(&self) -> &str {
        let (message, _, _, _) = self.parts();
        if message.starts_with(self.kind()) {
            message
                .split_once(": ")
                .map(|(_, rest)| rest)
                .unwrap_or(message)
        } else {
            message
        }
    }

    pub(crate) fn parts(&self) -> (&str, &str, usize, usize) {
        match self {
            Self::Syntax {
                message,
                file,
                line,
                column,
            }
            | Self::Name {
                message,
                file,
                line,
                column,
            }
            | Self::Type {
                message,
                file,
                line,
                column,
            }
            | Self::Value {
                message,
                file,
                line,
                column,
            }
            | Self::Io {
                message,
                file,
                line,
                column,
            }
            | Self::FileNotFound {
                message,
                file,
                line,
                column,
            }
            | Self::Key {
                message,
                file,
                line,
                column,
            }
            | Self::Permission {
                message,
                file,
                line,
                column,
            }
            | Self::Overflow {
                message,
                file,
                line,
                column,
            }
            | Self::Runtime {
                message,
                file,
                line,
                column,
            }
            | Self::Project {
                message,
                file,
                line,
                column,
            } => (message, file, *line, *column),
        }
    }
}

impl fmt::Display for ZapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (_, file, line, column) = self.parts();
        let message = self.message();
        if file.is_empty() {
            write!(f, "{}: {}", self.kind(), message)
        } else {
            write!(
                f,
                "{} at {}:{}:{}: {}",
                self.kind(),
                file,
                line,
                column,
                message
            )
        }
    }
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0c}' => escaped.push_str("\\f"),
            character if character.is_control() => {
                escaped.push_str(&format!("\\u{:04x}", character as u32));
            }
            character => escaped.push(character),
        }
    }
    escaped
}

fn redact_sensitive(message: &str) -> String {
    let mut redacted = message.to_string();
    for key in ["password", "passwd", "secret", "token", "api_key", "apikey"] {
        let mut start = 0;
        while let Some(relative) = redacted[start..].to_ascii_lowercase().find(key) {
            let index = start + relative;
            let after_key = index + key.len();
            let bytes = redacted.as_bytes();
            let mut cursor = after_key;
            while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                cursor += 1;
            }
            if cursor < bytes.len() && (bytes[cursor] == b'=' || bytes[cursor] == b':') {
                cursor += 1;
                while cursor < bytes.len() && bytes[cursor].is_ascii_whitespace() {
                    cursor += 1;
                }
                let end =
                    if cursor < bytes.len() && (bytes[cursor] == b'"' || bytes[cursor] == b'\'') {
                        let quote = bytes[cursor];
                        cursor += 1;
                        while cursor < bytes.len() && bytes[cursor] != quote {
                            cursor += 1;
                        }
                        if cursor < bytes.len() {
                            cursor + 1
                        } else {
                            cursor
                        }
                    } else {
                        redacted[cursor..]
                            .find(|character: char| {
                                character.is_whitespace() || character == ',' || character == ';'
                            })
                            .map(|offset| cursor + offset)
                            .unwrap_or(redacted.len())
                    };
                redacted.replace_range(index..end, &format!("{key}=<redacted>"));
                start = index + key.len() + 1;
            } else {
                start = after_key;
            }
        }
    }
    redacted
}

fn diagnostic_location(error: &str) -> (String, usize, usize) {
    if let Some((_, rest)) = error.split_once(" at ") {
        if let Some((location, _)) = rest.split_once(": ") {
            let mut parts = location.rsplitn(3, ':');
            let first = parts.next();
            let second = parts.next();
            let third = parts.next();
            if let (Some(column), Some(line), Some(file)) = (
                first.and_then(|x| x.parse::<usize>().ok()),
                second.and_then(|x| x.parse::<usize>().ok()),
                third,
            ) {
                return (file.to_string(), line, column);
            }
            if let (Some(line), Some(file)) = (first.and_then(|x| x.parse::<usize>().ok()), second)
            {
                return (file.to_string(), line, 1);
            }
        }
    }
    (String::new(), 0, 0)
}

#[cfg(test)]
mod tests {
    use super::{json_escape, ZapError};

    #[test]
    fn classifies_runtime_error_messages_stably() {
        let error = ZapError::from_message("uncaught error: Err(invalid input)");
        assert_eq!(error.kind(), "Error");
        assert_eq!(error.message(), "uncaught error: Err(invalid input)");
        assert_eq!(
            error.to_string(),
            "Error: uncaught error: Err(invalid input)"
        );
    }

    #[test]
    fn structured_metadata_is_stable_for_type_errors() {
        let error = ZapError::from_message("TypeError at main.zp:4:12: expected number, got text");
        assert_eq!(error.severity(), "error");
        assert_eq!(error.code(), "ZAP-TYPE-001");
        assert_eq!(error.kind(), "TypeError");
        assert_eq!(
            error.notes(),
            vec!["Check the expression type and the expected annotation."]
        );
        assert_eq!(
            error.help(),
            Some("Use a compatible value or update the type annotation.")
        );
    }

    #[test]
    fn structured_json_snapshot_is_stable_for_type_errors() {
        let error = ZapError::from_message("TypeError at main.zp:4:12: expected number, got text");
        assert_eq!(
            error.json_fields(),
            "\"code\":\"ZAP-TYPE-001\",\"kind\":\"TypeError\",\"severity\":\"error\",\"file\":\"main.zp\",\"line\":4,\"column\":12,\"message\":\"expected number, got text\",\"notes\":[\"Check the expression type and the expected annotation.\"],\"help\":\"Use a compatible value or update the type annotation.\",\"error\":\"TypeError at main.zp:4:12: expected number, got text\""
        );
    }

    #[test]
    fn json_snapshot_escapes_control_characters() {
        assert_eq!(json_escape("line\tvalue\nnext"), "line\\tvalue\\nnext");
        assert_eq!(json_escape("nul\u{0000}"), "nul\\u0000");
    }

    #[test]
    fn redacts_sensitive_key_value_messages() {
        let error = ZapError::from_message(
            "request failed password=hunter2 token='abc123' api_key: secret-value",
        );
        let rendered = error.to_string();
        assert!(!rendered.contains("hunter2"));
        assert!(!rendered.contains("abc123"));
        assert!(!rendered.contains("secret-value"));
        assert!(rendered.contains("password=<redacted>"));
        assert!(rendered.contains("token=<redacted>"));
        assert!(rendered.contains("api_key=<redacted>"));
    }
}
