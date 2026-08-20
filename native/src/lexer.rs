#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    Name(String),
    Number(i64),
    Text(String),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqEq,
    NotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    And,
    Or,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Colon,
    Comma,
    Dot,
    End,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SourceSpan {
    pub(crate) line: usize,
    pub(crate) column: usize,
    pub(crate) length: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SpannedToken {
    pub(crate) token: Token,
    pub(crate) span: SourceSpan,
}

fn location(line: usize, column: usize, length: usize) -> SourceSpan {
    SourceSpan {
        line,
        column,
        length: length.max(1),
    }
}

/// Tokenizes source while retaining the one-based line and column of every token.
pub(crate) fn tokenize_with_spans(source: &str) -> Result<Vec<SpannedToken>, String> {
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    let mut line = 1;
    let mut column = 1;
    let mut out = Vec::new();

    let advance = |ch: char, line: &mut usize, column: &mut usize| {
        if ch == '\n' {
            *line += 1;
            *column = 1;
        } else {
            *column += 1;
        }
    };

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            advance(c, &mut line, &mut column);
            i += 1;
            continue;
        }
        if c == '#' {
            while i < chars.len() && chars[i] != '\n' {
                advance(chars[i], &mut line, &mut column);
                i += 1;
            }
            continue;
        }

        let start_line = line;
        let start_column = column;
        if c.is_ascii_digit() {
            let s = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                advance(chars[i], &mut line, &mut column);
                i += 1;
            }
            let raw: String = chars[s..i].iter().collect();
            let value = raw.parse::<i64>().map_err(|_| {
                format!("invalid integer literal at {start_line}:{start_column}: {raw}")
            })?;
            out.push(SpannedToken {
                token: Token::Number(value),
                span: location(start_line, start_column, i - s),
            });
            continue;
        }

        if c.is_ascii_alphabetic() || c == '_' {
            let s = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                advance(chars[i], &mut line, &mut column);
                i += 1;
            }
            let word: String = chars[s..i].iter().collect();
            let token = match word.as_str() {
                "and" => Token::And,
                "or" => Token::Or,
                _ => Token::Name(word),
            };
            out.push(SpannedToken {
                token,
                span: location(start_line, start_column, i - s),
            });
            continue;
        }

        if c == '"' {
            advance(c, &mut line, &mut column);
            i += 1;
            let mut value = String::new();
            let mut closed = false;
            while i < chars.len() {
                if chars[i] == '"' {
                    advance(chars[i], &mut line, &mut column);
                    i += 1;
                    closed = true;
                    break;
                }
                if chars[i] == '\\' && i + 1 < chars.len() {
                    advance(chars[i], &mut line, &mut column);
                    i += 1;
                    let escaped = chars[i];
                    value.push(match escaped {
                        'n' => '\n',
                        't' => '\t',
                        '"' => '"',
                        '\\' => '\\',
                        other => other,
                    });
                    advance(escaped, &mut line, &mut column);
                    i += 1;
                } else {
                    value.push(chars[i]);
                    advance(chars[i], &mut line, &mut column);
                    i += 1;
                }
            }
            if !closed {
                return Err(format!(
                    "unterminated string at {start_line}:{start_column}"
                ));
            }
            out.push(SpannedToken {
                token: Token::Text(value),
                span: location(
                    start_line,
                    start_column,
                    column.saturating_sub(start_column),
                ),
            });
            continue;
        }

        let (token, width) = match c {
            '+' => (Token::Plus, 1),
            '-' => (Token::Minus, 1),
            '*' => (Token::Star, 1),
            '/' => (Token::Slash, 1),
            '%' => (Token::Percent, 1),
            '=' if i + 1 < chars.len() && chars[i + 1] == '=' => (Token::EqEq, 2),
            '=' => (Token::Equal, 1),
            '(' => (Token::LParen, 1),
            ')' => (Token::RParen, 1),
            '[' => (Token::LBracket, 1),
            ']' => (Token::RBracket, 1),
            '{' => (Token::LBrace, 1),
            '}' => (Token::RBrace, 1),
            ':' => (Token::Colon, 1),
            ',' => (Token::Comma, 1),
            '!' if i + 1 < chars.len() && chars[i + 1] == '=' => (Token::NotEq, 2),
            '<' if i + 1 < chars.len() && chars[i + 1] == '=' => (Token::LessEq, 2),
            '>' if i + 1 < chars.len() && chars[i + 1] == '=' => (Token::GreaterEq, 2),
            '<' => (Token::Less, 1),
            '>' => (Token::Greater, 1),
            '.' => (Token::Dot, 1),
            _ => {
                return Err(format!(
                    "unexpected character at {start_line}:{start_column}: {c}"
                ));
            }
        };
        for ch in chars[i..i + width].iter().copied() {
            advance(ch, &mut line, &mut column);
        }
        i += width;
        out.push(SpannedToken {
            token,
            span: location(start_line, start_column, width),
        });
    }

    out.push(SpannedToken {
        token: Token::End,
        span: location(line, column, 0),
    });
    Ok(out)
}

/// Compatibility wrapper for the existing parser and evaluator.
pub(crate) fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    Ok(tokenize_with_spans(source)?
        .into_iter()
        .map(|spanned| spanned.token)
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{tokenize_with_spans, Token};

    #[test]
    fn records_one_based_token_locations() {
        let tokens = tokenize_with_spans("let x == 1\n  say x").expect("tokenize");
        assert_eq!(tokens[0].span.line, 1);
        assert_eq!(tokens[0].span.column, 1);
        assert_eq!(tokens[4].span.line, 2);
        assert_eq!(tokens[4].span.column, 3);
        assert_eq!(tokens[4].token, Token::Name("say".into()));
    }

    #[test]
    fn reports_invalid_literals_and_characters_with_location() {
        let overflow = tokenize_with_spans("        999999999999999999999999").unwrap_err();
        assert!(overflow.contains("1:9"));
        let unexpected = tokenize_with_spans("        @").unwrap_err();
        assert!(unexpected.contains("1:9"));
    }

    #[test]
    fn reports_unterminated_strings_with_location() {
        let error = tokenize_with_spans("say \"hello").unwrap_err();
        assert!(error.contains("1:5"));
    }
}
