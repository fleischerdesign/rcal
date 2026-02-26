use crate::ast::{BinOp, Expr, Node, UnOp};
use crate::error::RcalError;
use crate::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub fn cur(&self) -> &Token {
        self.tokens
            .get(self.pos)
            .unwrap_or(&self.tokens[self.tokens.len() - 1])
    }

    fn peek(&self) -> &Token {
        self.tokens
            .get(self.pos + 1)
            .unwrap_or(&self.tokens[self.tokens.len() - 1])
    }

    fn consume(&mut self) {
        self.pos += 1;
    }

    pub fn parse_expr(&mut self) -> Result<Box<Node>, RcalError> {
        if let TokenKind::Identifier(name) = &self.cur().kind {
            if self.peek().kind == TokenKind::Assign {
                let (name, pos) = (name.clone(), self.cur().pos);
                self.consume();
                self.consume();
                return Ok(Box::new(Node {
                    expr: Expr::Assign(name, self.parse_expr()?),
                    pos,
                }));
            }

            if self.peek().kind == TokenKind::LParen {
                let mut lookahead = self.pos + 2;
                let mut params = Vec::new();
                let mut is_def = false;

                while let Some(tok) = self.tokens.get(lookahead) {
                    match &tok.kind {
                        TokenKind::Identifier(p) => {
                            params.push(p.clone());
                            lookahead += 1;
                        }
                        TokenKind::Comma => lookahead += 1,
                        TokenKind::RParen => {
                            if let Some(next_tok) = self.tokens.get(lookahead + 1) {
                                if next_tok.kind == TokenKind::Assign {
                                    is_def = true;
                                }
                            }
                            break;
                        }
                        _ => break,
                    }
                }

                if is_def {
                    let (name, pos) = (name.clone(), self.cur().pos);
                    self.consume();
                    self.consume();
                    while let TokenKind::Identifier(_) = self.cur().kind {
                        self.consume();
                        if self.cur().kind == TokenKind::Comma {
                            self.consume();
                        }
                    }
                    self.consume();
                    self.consume();
                    return Ok(Box::new(Node {
                        expr: Expr::FnDefine(name, params, self.parse_expr()?),
                        pos,
                    }));
                }
            }
        }
        self.parse_binary(Self::parse_term, &[TokenKind::Plus, TokenKind::Minus])
    }

    fn parse_binary<F>(&mut self, mut next: F, kinds: &[TokenKind]) -> Result<Box<Node>, RcalError>
    where
        F: FnMut(&mut Self) -> Result<Box<Node>, RcalError>,
    {
        let mut left = next(self)?;
        while kinds.contains(&self.cur().kind) {
            let (kind, pos) = (self.cur().kind.clone(), self.cur().pos);
            self.consume();
            let right = next(self)?;
            let op = match kind {
                TokenKind::Plus => BinOp::Add,
                TokenKind::Minus => BinOp::Sub,
                TokenKind::Multiply => BinOp::Mul,
                TokenKind::Divide => BinOp::Div,
                TokenKind::Modulo => BinOp::Mod,
                TokenKind::Power => BinOp::Pow,
                _ => unreachable!(),
            };
            left = Box::new(Node {
                expr: Expr::Binary(op, left, right),
                pos,
            });
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Node>, RcalError> {
        let mut left = self.parse_implicit()?;
        loop {
            let (kind, pos) = (self.cur().kind.clone(), self.cur().pos);
            match kind {
                TokenKind::Multiply | TokenKind::Divide | TokenKind::Modulo => {
                    self.consume();
                    let right = self.parse_implicit()?;
                    let op = match kind {
                        TokenKind::Multiply => BinOp::Mul,
                        TokenKind::Divide => BinOp::Div,
                        _ => BinOp::Mod,
                    };
                    left = Box::new(Node {
                        expr: Expr::Binary(op, left, right),
                        pos,
                    });
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_implicit(&mut self) -> Result<Box<Node>, RcalError> {
        let mut left = self.parse_power()?;
        loop {
            let pos = self.cur().pos;
            match &self.cur().kind {
                TokenKind::LParen | TokenKind::Identifier(_) | TokenKind::Number(_) => {
                    let right = self.parse_power()?;
                    left = Box::new(Node {
                        expr: Expr::Binary(BinOp::Mul, left, right),
                        pos,
                    });
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Box<Node>, RcalError> {
        let left = self.parse_postfix()?;
        if self.cur().kind == TokenKind::Power {
            let pos = self.cur().pos;
            self.consume();
            Ok(Box::new(Node {
                expr: Expr::Binary(BinOp::Pow, left, self.parse_power()?),
                pos,
            }))
        } else {
            Ok(left)
        }
    }

    fn parse_postfix(&mut self) -> Result<Box<Node>, RcalError> {
        let mut left = self.parse_unary()?;
        while self.cur().kind == TokenKind::Factorial {
            let pos = self.cur().pos;
            self.consume();
            left = Box::new(Node {
                expr: Expr::Factorial(left),
                pos,
            });
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Box<Node>, RcalError> {
        let (kind, pos) = (self.cur().kind.clone(), self.cur().pos);
        match kind {
            TokenKind::Plus | TokenKind::Minus => {
                self.consume();
                let op = if kind == TokenKind::Plus {
                    UnOp::Pos
                } else {
                    UnOp::Neg
                };
                Ok(Box::new(Node {
                    expr: Expr::Unary(op, self.parse_unary()?),
                    pos,
                }))
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Box<Node>, RcalError> {
        let (kind, pos) = (self.cur().kind.clone(), self.cur().pos);
        match kind {
            TokenKind::Number(n) => {
                self.consume();
                Ok(Box::new(Node {
                    expr: Expr::Number(n),
                    pos,
                }))
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.consume();
                if self.cur().kind == TokenKind::LParen {
                    self.consume();
                    let mut args = Vec::new();
                    if self.cur().kind != TokenKind::RParen {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.cur().kind == TokenKind::Comma {
                                self.consume();
                            } else {
                                break;
                            }
                        }
                    }
                    if self.cur().kind != TokenKind::RParen {
                        return Err(RcalError::Parser(
                            "Expected ')'".to_string(),
                            self.cur().pos,
                        ));
                    }
                    self.consume();
                    Ok(Box::new(Node {
                        expr: Expr::Function(name, args),
                        pos,
                    }))
                } else {
                    Ok(Box::new(Node {
                        expr: Expr::Variable(name),
                        pos,
                    }))
                }
            }
            TokenKind::LParen => {
                self.consume();
                let node = self.parse_expr()?;
                if self.cur().kind != TokenKind::RParen {
                    return Err(RcalError::Parser(
                        format!("Expected ')', found {:?}", self.cur().kind),
                        self.cur().pos,
                    ));
                }
                self.consume();
                Ok(node)
            }
            _ => Err(RcalError::Parser(
                format!("Unexpected token {:?}", kind),
                pos,
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::BinOp;
    use crate::lexer::tokenize;

    #[test]
    fn test_precedence() {
        let toks = tokenize("1 + 2 * 3").unwrap();
        let mut p = Parser::new(toks);
        let ast = p.parse_expr().unwrap();

        if let Expr::Binary(op, _, right) = ast.expr {
            assert_eq!(op, BinOp::Add);
            if let Expr::Binary(op2, _, _) = right.expr {
                assert_eq!(op2, BinOp::Mul);
            } else {
                panic!("Expected binary multiplication on the right");
            }
        } else {
            panic!("Expected binary addition at top level");
        }
    }

    #[test]
    fn test_implicit_mul() {
        let toks = tokenize("2pi (1+2)").unwrap();
        let mut p = Parser::new(toks);
        let ast = p.parse_expr().unwrap();

        if let Expr::Binary(BinOp::Mul, _, _) = ast.expr {
        } else {
            panic!("Expected implicit multiplication, got {:?}", ast.expr);
        }
    }

    #[test]
    fn test_fn_define() {
        let toks = tokenize("f(x, y) = x + y").unwrap();
        let mut p = Parser::new(toks);
        let ast = p.parse_expr().unwrap();

        if let Expr::FnDefine(name, params, _) = ast.expr {
            assert_eq!(name, "f");
            assert_eq!(params, vec!["x", "y"]);
        } else {
            panic!("Expected function definition");
        }
    }
}
