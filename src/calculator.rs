use crate::ast::{Expr, Node};
use crate::error::RcalError;
use crate::evaluator::evaluate;
use crate::lexer::{TokenKind, tokenize};
use crate::parser::Parser;
use crate::unit::Quantity;
use std::collections::HashMap;

pub struct UserFunction {
    pub params: Vec<String>,
    pub body: Box<Node>,
}

pub struct Calculator {
    vars: HashMap<String, Quantity>,
    funcs: HashMap<String, UserFunction>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    pub fn eval(&mut self, input: &str) -> Result<(Quantity, Expr), RcalError> {
        let toks = tokenize(input)?;
        let mut p = Parser::new(toks);
        let ast = p.parse_expr()?;

        if p.cur().kind != TokenKind::EOF {
            return Err(RcalError::Parser(
                "Unexpected character".to_string(),
                p.cur().pos,
            ));
        }

        let v = evaluate(&ast, &mut self.vars, &mut self.funcs)?;
        self.vars.insert("ans".to_string(), v);
        Ok((v, ast.expr))
    }

    pub fn vars(&self) -> &HashMap<String, Quantity> {
        &self.vars
    }

    pub fn funcs(&self) -> &HashMap<String, UserFunction> {
        &self.funcs
    }
}
