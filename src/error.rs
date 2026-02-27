//! Error types and reporting.

use std::fmt;

/// Errors that occur during lexical analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    /// An unknown character was encountered.
    UnexpectedCharacter(char),
    /// A numeric literal was malformed.
    InvalidNumber(String),
    /// A multi-line comment was not closed.
    UnclosedComment,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter(c) => write!(f, "Unexpected character '{}'", c),
            LexerError::InvalidNumber(n) => write!(f, "Invalid number literal '{}'", n),
            LexerError::UnclosedComment => write!(f, "Unclosed comment"),
        }
    }
}

/// Errors that occur during parsing.
#[derive(Debug, Clone, PartialEq)]
pub enum ParserError {
    /// A specific token was expected but another was found.
    UnexpectedToken { expected: String, actual: String },
    /// A specific token was expected.
    ExpectedToken(String),
    /// The input ended prematurely.
    UnexpectedEof,
    /// The expression was empty.
    EmptyExpression,
    /// An assignment was made to an invalid target.
    InvalidAssignment,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::UnexpectedToken { expected, actual } => {
                write!(f, "Expected {}, found {}", expected, actual)
            }
            ParserError::ExpectedToken(tok) => write!(f, "Expected {}", tok),
            ParserError::UnexpectedEof => write!(f, "Unexpected end of expression"),
            ParserError::EmptyExpression => write!(f, "Empty expression"),
            ParserError::InvalidAssignment => write!(f, "Invalid assignment target"),
        }
    }
}

/// Errors that occur during evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum MathError {
    /// Operands had incompatible physical dimensions.
    DimensionMismatch { expected: String, actual: String },
    /// An unknown variable name was used.
    UnknownVariable(String),
    /// An unknown function name was used.
    UnknownFunction(String),
    /// A function was called with the wrong number of arguments.
    ArityMismatch { expected: usize, actual: usize },
    /// Division by zero.
    DivisionByZero,
    /// Modulo by zero.
    ModuloByZero,
    /// An operation required a scalar but received a quantity with units.
    NonScalarOperation(String),
    /// Attempted to overwrite a protected name.
    ProtectedName(String),
    /// A result overflowed numeric limits.
    Overflow(String),
    /// A generic mathematical error.
    Generic(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MathError::DimensionMismatch { expected, actual } => {
                write!(
                    f,
                    "Dimension mismatch: expected {}, got {}",
                    expected, actual
                )
            }
            MathError::UnknownVariable(name) => write!(f, "Unknown variable '{}'", name),
            MathError::UnknownFunction(name) => write!(f, "Unknown function '{}'", name),
            MathError::ArityMismatch { expected, actual } => {
                write!(f, "Expected {} arguments, got {}", expected, actual)
            }
            MathError::DivisionByZero => write!(f, "Division by zero"),
            MathError::ModuloByZero => write!(f, "Modulo by zero"),
            MathError::NonScalarOperation(op) => write!(f, "{} requires scalars", op),
            MathError::ProtectedName(name) => write!(f, "'{}' is a protected name", name),
            MathError::Overflow(msg) => write!(f, "Overflow: {}", msg),
            MathError::Generic(msg) => write!(f, "{}", msg),
        }
    }
}

/// The root error type for rcal.
#[derive(Debug)]
pub enum Error {
    /// Lexer error with position.
    Lexer(LexerError, usize),
    /// Parser error with position.
    Parser(ParserError, usize),
    /// Math error with position.
    Math(MathError, usize),
    /// Non-engine errors (CLI, IO).
    Cli(String),
}

impl Error {
    /// Returns the source position of the error, if available.
    pub fn pos(&self) -> Option<usize> {
        match self {
            Error::Lexer(_, p) | Error::Parser(_, p) | Error::Math(_, p) => Some(*p),
            Error::Cli(_) => None,
        }
    }

    /// Reports the error to stdout with ANSI colors.
    pub fn report(&self) {
        let red = "\x1b[31m";
        let reset = "\x1b[0m";
        match self {
            Error::Lexer(e, _) => println!("{}Lexer Error: {}{}", red, e, reset),
            Error::Parser(e, _) => println!("{}Parser Error: {}{}", red, e, reset),
            Error::Math(e, _) => println!("{}Math Error: {}{}", red, e, reset),
            Error::Cli(m) => println!("{}CLI Error: {}{}", red, m, reset),
        }
    }

    /// Reports the error at a specific position in the input string.
    pub fn report_at(&self, input: &str, line_num: Option<usize>) {
        let red = "\x1b[31m";
        let reset = "\x1b[0m";

        if let Some(n) = line_num {
            println!("{}Error in line {}:{}", red, n, reset);
        }

        if let Some(pos) = self.pos() {
            println!("{}", input);
            let msg = match self {
                Error::Lexer(e, _) => format!("Lexer Error: {}", e),
                Error::Parser(e, _) => format!("Parser Error: {}", e),
                Error::Math(e, _) => format!("Math Error: {}", e),
                Error::Cli(m) => format!("CLI Error: {}", m),
            };
            println!("{}{}^-- {}{}", red, " ".repeat(pos), msg, reset);
        } else {
            println!("{}Error: {}{}", red, self, reset);
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Lexer(e, _) => write!(f, "Lexer Error: {}", e),
            Error::Parser(e, _) => write!(f, "Parser Error: {}", e),
            Error::Math(e, _) => write!(f, "Math Error: {}", e),
            Error::Cli(m) => write!(f, "CLI Error: {}", m),
        }
    }
}
