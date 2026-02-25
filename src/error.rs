use std::fmt;

pub enum RcalError {
    Lexer(String, usize),
    Parser(String, usize),
    Math(String, usize),
}

impl RcalError {
    pub fn pos(&self) -> usize {
        match self {
            RcalError::Lexer(_, p) | RcalError::Parser(_, p) | RcalError::Math(_, p) => *p,
        }
    }
}

impl fmt::Display for RcalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RcalError::Lexer(msg, _) => write!(f, "Lexer Error: {}", msg),
            RcalError::Parser(msg, _) => write!(f, "Parser Error: {}", msg),
            RcalError::Math(msg, _) => write!(f, "Math Error: {}", msg),
        }
    }
}
