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
                    expr: Expr::Assign(name.to_lowercase(), self.parse_expr()?),
                    pos,
                }));
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
        let mut left = self.parse_power()?;
        loop {
            let (kind, pos) = (self.cur().kind.clone(), self.cur().pos);
            match kind {
                TokenKind::Multiply | TokenKind::Divide | TokenKind::Modulo => {
                    self.consume();
                    let right = self.parse_power()?;
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
        let pos = self.cur().pos;
        match self.cur().kind {
            TokenKind::Minus | TokenKind::Plus => {
                let kind = self.cur().kind.clone();
                self.consume();
                let op = if kind == TokenKind::Minus {
                    UnOp::Neg
                } else {
                    UnOp::Pos
                };
                Ok(Box::new(Node {
                    expr: Expr::Unary(op, self.parse_unary()?),
                    pos,
                }))
            }
            _ => self.parse_factor(),
        }
    }

    fn parse_factor(&mut self) -> Result<Box<Node>, RcalError> {
        let pos = self.cur().pos;
        match self.cur().kind.clone() {
            TokenKind::Number(n) => {
                self.consume();
                Ok(Box::new(Node {
                    expr: Expr::Number(n),
                    pos,
                }))
            }
            TokenKind::Identifier(name) => {
                self.consume();
                let name = name.to_lowercase();
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
                    if self.cur().kind == TokenKind::RParen {
                        self.consume();
                        Ok(Box::new(Node {
                            expr: Expr::Function(name, args),
                            pos,
                        }))
                    } else {
                        Err(RcalError::Parser(
                            "Lacking closing parenthesis".to_string(),
                            self.cur().pos,
                        ))
                    }
                } else {
                    Ok(Box::new(Node {
                        expr: Expr::Variable(name),
                        pos,
                    }))
                }
            }
            TokenKind::LParen => {
                self.consume();
                let e = self.parse_expr()?;
                if self.cur().kind == TokenKind::RParen {
                    self.consume();
                    Ok(e)
                } else {
                    Err(RcalError::Parser(
                        "Lacking closing parenthesis".to_string(),
                        self.cur().pos,
                    ))
                }
            }
            TokenKind::EOF => Err(RcalError::Parser(
                "Unexpected end of input".to_string(),
                pos,
            )),
            _ => Err(RcalError::Parser(
                format!("Unexpected token {:?}", self.cur().kind),
                pos,
            )),
        }
    }
}
