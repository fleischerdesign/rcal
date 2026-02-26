use std::fmt;

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

#[derive(Debug, Clone)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Assign(String, Box<Node>),
    FnDefine(String, Vec<String>, Box<Node>),
    Function(String, Vec<Node>),
    Binary(BinOp, Box<Node>, Box<Node>),
    Factorial(Box<Node>),
    Unary(UnOp, Box<Node>),
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
        }
    }
}

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
