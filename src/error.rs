use std::fmt;

#[derive(Debug)]
pub enum RcalError {
    Lexer(String, usize),
    Parser(String, usize),
    Math(String, usize),
    Cli(String),
}

impl RcalError {
    pub fn pos(&self) -> Option<usize> {
        match self {
            RcalError::Lexer(_, p) | RcalError::Parser(_, p) | RcalError::Math(_, p) => Some(*p),
            RcalError::Cli(_) => None,
        }
    }

    pub fn report(&self) {
        let red = "\x1b[31m";
        let reset = "\x1b[0m";
        println!("{}Error: {}{}", red, self, reset);
    }

    pub fn report_at(&self, input: &str, line_num: Option<usize>) {
        let red = "\x1b[31m";
        let reset = "\x1b[0m";

        if let Some(n) = line_num {
            println!("{}Error in line {}:{}", red, n, reset);
        }

        if let Some(pos) = self.pos() {
            println!("{}", input);
            println!("{}{}^-- {}{}", red, " ".repeat(pos), self, reset);
        } else {
            println!("{}Error: {}{}", red, self, reset);
        }
    }
}

impl fmt::Display for RcalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RcalError::Lexer(msg, _) => write!(f, "Lexer Error: {}", msg),
            RcalError::Parser(msg, _) => write!(f, "Parser Error: {}", msg),
            RcalError::Math(msg, _) => write!(f, "Math Error: {}", msg),
            RcalError::Cli(msg) => write!(f, "CLI Error: {}", msg),
        }
    }
}
