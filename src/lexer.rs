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

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize,
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
                tokens.push(Token {
                    kind: TokenKind::Identifier(s),
                    pos: start,
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
                let n = s.parse::<f64>().map_err(|_| {
                    RcalError::Lexer(format!("LexerError: Invalid number '{}'", s), start)
                })?;
                if n.is_infinite() {
                    return Err(RcalError::Lexer(format!("Overflow in '{}'", s), start));
                }
                tokens.push(Token {
                    kind: TokenKind::Number(n),
                    pos: start,
                });
                continue;
            }
            _ => return Err(RcalError::Lexer(format!("Unexpected character '{}'", c), i)),
        };
        tokens.push(Token { kind, pos: i });
        chars.next();
    }
    tokens.push(Token {
        kind: TokenKind::EOF,
        pos: input.len(),
    });
    Ok(tokens)
}
