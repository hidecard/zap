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

pub(crate) fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let chars: Vec<char> = source.chars().collect();
    let mut i = 0;
    let mut out = Vec::new();
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c == '#' {
            while i < chars.len() && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }
        if c.is_ascii_digit() {
            let s = i;
            while i < chars.len() && chars[i].is_ascii_digit() {
                i += 1;
            }
            out.push(Token::Number(
                chars[s..i].iter().collect::<String>().parse().unwrap(),
            ));
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let s = i;
            while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[s..i].iter().collect();
            out.push(match word.as_str() {
                "and" => Token::And,
                "or" => Token::Or,
                _ => Token::Name(word),
            });
            continue;
        }
        if c == '"' {
            i += 1;
            let mut s = String::new();
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 1;
                    s.push(match chars[i] {
                        'n' => '\n',
                        't' => '\t',
                        '"' => '"',
                        '\\' => '\\',
                        x => x,
                    });
                } else {
                    s.push(chars[i]);
                }
                i += 1;
            }
            if i == chars.len() {
                return Err("unterminated string".into());
            }
            i += 1;
            out.push(Token::Text(s));
            continue;
        }
        let t = match c {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '*' => Token::Star,
            '/' => Token::Slash,
            '%' => Token::Percent,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '[' => Token::LBracket,
            ']' => Token::RBracket,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            ':' => Token::Colon,
            ',' => Token::Comma,
            '=' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                i += 1;
                Token::EqEq
            }
            '!' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                i += 1;
                Token::NotEq
            }
            '<' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                i += 1;
                Token::LessEq
            }
            '>' if i + 1 < chars.len() && chars[i + 1] == '=' => {
                i += 1;
                Token::GreaterEq
            }
            '<' => Token::Less,
            '>' => Token::Greater,
            '.' => Token::Dot,
            _ => return Err(format!("unexpected character: {c}")),
        };
        out.push(t);
        i += 1;
    }
    out.push(Token::End);
    Ok(out)
}
