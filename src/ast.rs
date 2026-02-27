//! Abstract Syntax Tree definitions.

use std::fmt;

/// A node in the AST, containing an expression and its source position.
#[derive(Debug, Clone)]
pub struct Node {
    pub expr: Expr,
    pub pos: usize,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.expr)
    }
}

/// Types of expressions in the rcal language.
#[derive(Debug, Clone)]
pub enum Expr {
    /// A numeric literal.
    Number(f64),
    /// A variable lookup.
    Variable(String),
    /// A variable assignment.
    Assign(String, Box<Node>),
    /// A function definition.
    FnDefine(String, Vec<String>, Box<Node>),
    /// A function call.
    Function(String, Vec<Node>),
    /// A binary operation.
    Binary(BinOp, Box<Node>, Box<Node>),
    /// A factorial operation.
    Factorial(Box<Node>),
    /// A unary operation.
    Unary(UnOp, Box<Node>),
    /// A unit conversion operation.
    Convert(Box<Node>, Box<Node>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Variable(name) => write!(f, "{}", name),
            Expr::Assign(name, val) => write!(f, "{} = {}", name, val),
            Expr::FnDefine(name, params, body) => {
                write!(f, "{}({}) = {}", name, params.join(", "), body)
            }
            Expr::Function(name, args) => {
                let args_str: Vec<_> = args.iter().map(|a| a.expr.to_string()).collect();
                write!(f, "{}({})", name, args_str.join(", "))
            }
            Expr::Binary(op, l, r) => write!(f, "({} {} {})", l, op, r),
            Expr::Factorial(e) => write!(f, "{}!", e),
            Expr::Unary(op, e) => write!(f, "{}{}", op, e),
            Expr::Convert(e, target) => write!(f, "{} in {}", e, target),
        }
    }
}

/// Binary operators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

impl fmt::Display for BinOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinOp::Add => "+",
            BinOp::Sub => "-",
            BinOp::Mul => "*",
            BinOp::Div => "/",
            BinOp::Mod => "%",
            BinOp::Pow => "^",
        };
        write!(f, "{}", s)
    }
}

/// Unary operators.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnOp {
    Neg,
    Pos,
}

impl fmt::Display for UnOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnOp::Neg => "-",
            UnOp::Pos => "+",
        };
        write!(f, "{}", s)
    }
}
