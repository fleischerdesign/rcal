#[derive(Debug)]
pub struct Node {
    pub expr: Expr,
    pub pos: usize,
}

#[derive(Debug)]
pub enum Expr {
    Number(f64),
    Variable(String),
    Assign(String, Box<Node>),
    Function(String, Vec<Box<Node>>),
    Binary(BinOp, Box<Node>, Box<Node>),
    Factorial(Box<Node>),
    Unary(UnOp, Box<Node>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug, Clone, Copy)]
pub enum UnOp {
    Neg,
    Pos,
}
