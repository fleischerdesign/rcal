use crate::ast::Expr;
use crate::error::RcalError;
use crate::evaluator::evaluate;
use crate::lexer::{TokenKind, tokenize};
use crate::parser::Parser;
use std::collections::HashMap;

pub struct Calculator {
    vars: HashMap<String, f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn eval(&mut self, input: &str) -> Result<(f64, Expr), RcalError> {
        let toks = tokenize(input)?;
        let mut p = Parser::new(toks);
        let ast = p.parse_expr()?;

        if p.cur().kind != TokenKind::EOF {
            return Err(RcalError::Parser(
                "Unexpected character".to_string(),
                p.cur().pos,
            ));
        }

        let v = evaluate(&ast, &mut self.vars)?;
        self.vars.insert("ans".to_string(), v);
        Ok((v, ast.expr))
    }

    pub fn vars(&self) -> &HashMap<String, f64> {
        &self.vars
    }
}
