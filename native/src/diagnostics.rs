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
    Project {
        message: String,
        file: String,
        line: usize,
        column: usize,
    },
}

impl ZapError {
    pub(crate) fn from_message(message: impl Into<String>) -> Self {
        let message = message.into();
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
        } else if message.contains("not found") || message.contains("No such file") {
            "FileNotFound"
        } else if message.contains("permission denied") || message.contains("Permission denied") {
            "PermissionError"
        } else if message.contains("overflow") || message.contains("exceeded") {
            "OverflowError"
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
            _ => Self::Project {
                message,
                file,
                line,
                column,
            },
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
            Self::Permission { .. } => "PermissionError",
            Self::Overflow { .. } => "OverflowError",
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
