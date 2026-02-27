use crate::ast::{Expr, Node};
use crate::error::{Error, ParserError};
use crate::evaluator::{Scope, evaluate};
use crate::lexer::{TokenKind, tokenize};
use crate::parser::Parser;
use crate::unit::Quantity;

/// Represents a user-defined function.
#[derive(Clone)]
pub struct UserFunction {
    /// The names of the parameters.
    pub params: Vec<String>,
    /// The body of the function.
    pub body: Box<Node>,
}

/// The main calculator engine.
///
/// Holds the variable and function scopes and provides the entry point for evaluation.
pub struct Calculator {
    /// The root execution scope.
    pub scope: Scope<'static>,
}

impl Calculator {
    /// Creates a new calculator with an empty scope.
    pub fn new() -> Self {
        Self {
            scope: Scope::new(),
        }
    }

    /// Evaluates a string input.
    ///
    /// Returns the resulting `Quantity` and the parsed `Expr` on success.
    pub fn eval(&mut self, input: &str) -> Result<(Quantity, Expr), Error> {
        let toks = tokenize(input)?;
        let mut p = Parser::new(toks);
        let ast = p.parse_expr()?;

        if p.cur().kind != TokenKind::Eof {
            return Err(Error::Parser(
                ParserError::UnexpectedToken {
                    expected: "end of expression".to_string(),
                    actual: format!("{:?}", p.cur().kind),
                },
                p.cur().pos,
            ));
        }

        let v = evaluate(&ast, &mut self.scope)?;
        self.scope.insert_var("ans".to_string(), v);
        Ok((v, ast.expr))
    }
}

impl Default for Calculator {
    fn default() -> Self {
        Self::new()
    }
}
