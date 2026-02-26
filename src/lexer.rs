use crate::error::RcalError;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Factorial,
    Assign,
    Semicolon,
    Comma,
    LParen,
    RParen,
    EOF,
}

impl TokenKind {
    pub fn color(&self) -> Option<&'static str> {
        match self {
            TokenKind::Number(_) => Some("\x1b[35m"),
            TokenKind::Plus
            | TokenKind::Minus
            | TokenKind::Multiply
            | TokenKind::Divide
            | TokenKind::Modulo
            | TokenKind::Power
            | TokenKind::Assign => Some("\x1b[32m"),
            TokenKind::Semicolon | TokenKind::Comma | TokenKind::LParen | TokenKind::RParen => {
                Some("\x1b[90m")
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
    pub len: usize,
}

pub fn is_comment_or_empty(input: &str) -> bool {
    let trimmed = input.trim();
    trimmed.is_empty() || trimmed.starts_with('#')
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, RcalError> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some(&(i, c)) = chars.peek() {
        let kind = match c {
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
                continue;
            }
            '#' => {
                while let Some(&(_, ch)) = chars.peek() {
                    if ch == '\n' {
                        break;
                    }
                    chars.next();
                }
                continue;
            }
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divide,
            '%' => TokenKind::Modulo,
            '^' => TokenKind::Power,
            '!' => TokenKind::Factorial,
            '=' => TokenKind::Assign,
            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                let mut s = String::new();
                while let Some(&(_, ch)) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        s.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let len = s.len();
                tokens.push(Token {
                    kind: TokenKind::Identifier(s),
                    pos: start,
                    len,
                });
                continue;
            }
            '0'..='9' | '.' => {
                let start = i;
                let mut s = String::new();
                if c == '0' {
                    chars.next();
                    if let Some(&(_, next)) = chars.peek() {
                        let r = match next {
                            'x' | 'X' => 16,
                            'b' | 'B' => 2,
                            _ => 0,
                        };
                        if r > 0 {
                            chars.next();
                            let mut vs = String::new();
                            while let Some(&(_, ch)) = chars.peek() {
                                if ch.is_digit(r) {
                                    vs.push(ch);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            if let Ok(n) = u64::from_str_radix(&vs, r) {
                                tokens.push(Token {
                                    kind: TokenKind::Number(n as f64),
                                    pos: start,
                                    len: i - start + vs.len() + 2,
                                });
                                continue;
                            }
                            return Err(RcalError::Lexer(
                                format!("Invalid radix-{} format", r),
                                start,
                            ));
                        }
                        s.push('0');
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::Number(0.0),
                            pos: start,
                            len: 1,
                        });
                        continue;
                    }
                }
                while let Some(&(_, ch)) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' {
                        s.push(ch);
                        chars.next();
                        if (ch == 'e' || ch == 'E')
                            && chars.peek().map_or(false, |&(_, n)| n == '-' || n == '+')
                        {
                            s.push(chars.next().unwrap().1);
                        }
                    } else {
                        break;
                    }
                }
                let len = s.len();
                let n = s.parse::<f64>().map_err(|_| {
                    RcalError::Lexer(format!("LexerError: Invalid number '{}'", s), start)
                })?;
                if n.is_infinite() {
                    return Err(RcalError::Lexer(format!("Overflow in '{}'", s), start));
                }
                tokens.push(Token {
                    kind: TokenKind::Number(n),
                    pos: start,
                    len,
                });
                continue;
            }
            _ => return Err(RcalError::Lexer(format!("Unexpected character '{}'", c), i)),
        };
        tokens.push(Token {
            kind,
            pos: i,
            len: 1,
        });
        chars.next();
    }
    tokens.push(Token {
        kind: TokenKind::EOF,
        pos: input.len(),
        len: 0,
    });
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let input = "1 + 2 * (3 / 4)";
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens.len(), 10);
        assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 1.0));
        assert!(matches!(tokens[1].kind, TokenKind::Plus));
        assert!(matches!(tokens[2].kind, TokenKind::Number(n) if n == 2.0));
        assert!(matches!(tokens[3].kind, TokenKind::Multiply));
        assert!(matches!(tokens[4].kind, TokenKind::LParen));
        assert!(matches!(tokens[9].kind, TokenKind::EOF));
    }

    #[test]
    fn test_radix_numbers() {
        let hex = "0xff";
        let tokens = tokenize(hex).unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 255.0));

        let bin = "0b1010";
        let tokens = tokenize(bin).unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Number(n) if n == 10.0));
    }

    #[test]
    fn test_scientific_notation() {
        let input = "1.2e-3";
        let tokens = tokenize(input).unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Number(n) if (n - 0.0012).abs() < 1e-10));
    }

    #[test]
    fn test_identifiers() {
        let input = "var_name_123 sin pi";
        let tokens = tokenize(input).unwrap();
        assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "var_name_123"));
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "sin"));
        assert!(matches!(tokens[2].kind, TokenKind::Identifier(ref s) if s == "pi"));
    }

    #[test]
    fn test_errors() {
        assert!(tokenize("1.2.3").is_err());
        assert!(tokenize("0xz").is_err());
        assert!(tokenize("@").is_err());
    }
}
