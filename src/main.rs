use std::collections::HashMap;
use std::io::{self, Write};

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Number(f64),
    Identifier(String),
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Power,
    Factorial,
    Assign,
    Semicolon,
    Comma,
    LParen,
    RParen,
    EOF,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    pos: usize,
}

fn tokenize(input: &str) -> Result<Vec<Token>, (String, usize)> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();
    while let Some(&(i, c)) = chars.peek() {
        let kind = match c {
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
                continue;
            }
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Multiply,
            '/' => TokenKind::Divide,
            '%' => TokenKind::Modulo,
            '^' => TokenKind::Power,
            '!' => TokenKind::Factorial,
            '=' => TokenKind::Assign,
            ';' => TokenKind::Semicolon,
            ',' => TokenKind::Comma,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            'a'..='z' | 'A'..='Z' | '_' => {
                let start = i;
                let mut s = String::new();
                while let Some(&(_, ch)) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        s.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    kind: TokenKind::Identifier(s),
                    pos: start,
                });
                continue;
            }
            '0'..='9' | '.' => {
                let start = i;
                let mut s = String::new();
                if c == '0' {
                    chars.next();
                    if let Some(&(_, next)) = chars.peek() {
                        let r = match next {
                            'x' | 'X' => 16,
                            'b' | 'B' => 2,
                            _ => 0,
                        };
                        if r > 0 {
                            chars.next();
                            let mut vs = String::new();
                            while let Some(&(_, ch)) = chars.peek() {
                                if ch.is_digit(r) {
                                    vs.push(ch);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            if let Ok(n) = u64::from_str_radix(&vs, r) {
                                tokens.push(Token {
                                    kind: TokenKind::Number(n as f64),
                                    pos: start,
                                });
                                continue;
                            }
                            return Err((format!("LexerError: Invalid radix-{} format", r), start));
                        }
                        s.push('0');
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::Number(0.0),
                            pos: start,
                        });
                        continue;
                    }
                }
                while let Some(&(_, ch)) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' {
                        s.push(ch);
                        chars.next();
                        if (ch == 'e' || ch == 'E')
                            && chars.peek().map_or(false, |&(_, n)| n == '-' || n == '+')
                        {
                            s.push(chars.next().unwrap().1);
                        }
                    } else {
                        break;
                    }
                }
                let n = s
                    .parse::<f64>()
                    .map_err(|_| (format!("LexerError: Invalid number '{}'", s), start))?;
                if n.is_infinite() {
                    return Err((format!("LexerError: Overflow in '{}'", s), start));
                }
                tokens.push(Token {
                    kind: TokenKind::Number(n),
                    pos: start,
                });
                continue;
            }
            _ => return Err((format!("LexerError: Unexpected character '{}'", c), i)),
        };
        tokens.push(Token { kind, pos: i });
        chars.next();
    }
    tokens.push(Token {
        kind: TokenKind::EOF,
        pos: input.len(),
    });
    Ok(tokens)
}

#[derive(Debug)]
struct Node {
    expr: Expr,
    pos: usize,
}
#[derive(Debug)]
enum Expr {
    Number(f64),
    Variable(String),
    Assign(String, Box<Node>),
    Function(String, Vec<Box<Node>>),
    Binary(BinOp, Box<Node>, Box<Node>),
    Factorial(Box<Node>),
    Unary(UnOp, Box<Node>),
}
#[derive(Debug, Clone, Copy)]
enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}
#[derive(Debug, Clone, Copy)]
enum UnOp {
    Neg,
    Pos,
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}
impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    fn cur(&self) -> &Token {
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

    fn parse_expr(&mut self) -> Result<Box<Node>, (String, usize)> {
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

    fn parse_binary<F>(
        &mut self,
        mut next: F,
        kinds: &[TokenKind],
    ) -> Result<Box<Node>, (String, usize)>
    where
        F: FnMut(&mut Self) -> Result<Box<Node>, (String, usize)>,
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

    fn parse_term(&mut self) -> Result<Box<Node>, (String, usize)> {
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

    fn parse_power(&mut self) -> Result<Box<Node>, (String, usize)> {
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

    fn parse_postfix(&mut self) -> Result<Box<Node>, (String, usize)> {
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

    fn parse_unary(&mut self) -> Result<Box<Node>, (String, usize)> {
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

    fn parse_factor(&mut self) -> Result<Box<Node>, (String, usize)> {
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
                        Err((
                            "SyntaxError: Lacking closing parenthesis".to_string(),
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
                    Err((
                        "SyntaxError: Lacking closing parenthesis".to_string(),
                        self.cur().pos,
                    ))
                }
            }
            TokenKind::EOF => Err(("Unexpected end of input".to_string(), pos)),
            _ => Err((
                format!("SyntaxError: Unexpected token {:?}", self.cur().kind),
                pos,
            )),
        }
    }
}

fn evaluate(node: &Node, vars: &mut HashMap<String, f64>) -> Result<f64, (String, usize)> {
    let pos = node.pos;
    let check = |res: f64| {
        if res.is_infinite() {
            Err(("Math Error: Overflow".to_string(), pos))
        } else {
            Ok(res)
        }
    };
    match &node.expr {
        Expr::Number(n) => Ok(*n),
        Expr::Variable(name) => match name.as_str() {
            "pi" => Ok(std::f64::consts::PI),
            "e" => Ok(std::f64::consts::E),
            "deg" => Ok(std::f64::consts::PI / 180.0),
            _ => vars
                .get(name)
                .copied()
                .ok_or_else(|| (format!("Math Error: Unknown variable '{}'", name), pos)),
        },
        Expr::Assign(name, e) => {
            let v = evaluate(e, vars)?;
            vars.insert(name.clone(), v);
            Ok(v)
        }
        Expr::Function(name, args) => {
            let vs = args
                .iter()
                .map(|a| evaluate(a, vars))
                .collect::<Result<Vec<_>, _>>()?;
            match name.as_str() {
                "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "abs" | "sqrt" | "ln"
                | "log" | "not" | "hex" | "bin"
                    if vs.len() == 1 =>
                {
                    let v = vs[0];
                    match name.as_str() {
                        "sin" => Ok(v.sin()),
                        "cos" => Ok(v.cos()),
                        "tan" => Ok(v.tan()),
                        "asin" => Ok(v.asin()),
                        "acos" => Ok(v.acos()),
                        "atan" => Ok(v.atan()),
                        "abs" => Ok(v.abs()),
                        "sqrt" => {
                            if v < 0.0 {
                                Err(("Math Error: Sqrt of negative".to_string(), pos))
                            } else {
                                Ok(v.sqrt())
                            }
                        }
                        "ln" | "log" => {
                            if v <= 0.0 {
                                Err(("Math Error: Log of non-positive".to_string(), pos))
                            } else {
                                if name == "ln" {
                                    Ok(v.ln())
                                } else {
                                    Ok(v.log10())
                                }
                            }
                        }
                        "not" => Ok(!(v as u64) as f64),
                        "hex" | "bin" => {
                            if v < 0.0 || v > u64::MAX as f64 || v.fract() != 0.0 {
                                Err(("Math Error: Invalid for hex/bin".to_string(), pos))
                            } else {
                                Ok(v)
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                "and" | "or" | "xor" | "lshift" | "rshift" | "round" if vs.len() == 2 => {
                    let (a, b) = (vs[0], vs[1]);
                    match name.as_str() {
                        "and" => Ok(((a as u64) & (b as u64)) as f64),
                        "or" => Ok(((a as u64) | (b as u64)) as f64),
                        "xor" => Ok(((a as u64) ^ (b as u64)) as f64),
                        "lshift" => Ok(((a as u64) << (b as u64)) as f64),
                        "rshift" => Ok(((a as u64) >> (b as u64)) as f64),
                        "round" => {
                            let m = 10.0f64.powf(b.round());
                            Ok((a * m).round() / m)
                        }
                        _ => unreachable!(),
                    }
                }
                "max" | "min" | "sum" | "avg" if !vs.is_empty() => match name.as_str() {
                    "max" => Ok(vs.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
                    "min" => Ok(vs.iter().cloned().fold(f64::INFINITY, f64::min)),
                    "sum" => Ok(vs.iter().sum()),
                    "avg" => Ok(vs.iter().sum::<f64>() / vs.len() as f64),
                    _ => unreachable!(),
                },
                _ => Err((
                    format!("Math Error: Unknown function or wrong args for '{}'", name),
                    pos,
                )),
            }
        }
        Expr::Binary(op, l, r) => {
            let (lv, rv) = (evaluate(l, vars)?, evaluate(r, vars)?);
            match op {
                BinOp::Add => check(lv + rv),
                BinOp::Sub => check(lv - rv),
                BinOp::Mul => check(lv * rv),
                BinOp::Div => {
                    if rv == 0.0 {
                        Err(("Math Error: Division by zero".to_string(), pos))
                    } else {
                        check(lv / rv)
                    }
                }
                BinOp::Mod => {
                    if rv == 0.0 {
                        Err(("Math Error: Modulo by zero".to_string(), pos))
                    } else {
                        Ok(lv % rv)
                    }
                }
                BinOp::Pow => {
                    let res = lv.powf(rv);
                    if res.is_infinite() {
                        check(res)
                    } else if res.is_nan() {
                        Err(("Math Error: Invalid power".to_string(), pos))
                    } else {
                        Ok(res)
                    }
                }
            }
        }
        Expr::Factorial(e) => {
            let v = evaluate(e, vars)?;
            if v < 0.0 || v.fract() != 0.0 {
                return Err(("Math Error: Needs non-neg int".to_string(), pos));
            }
            if v > 170.0 {
                return Err(("Math Error: Factorial overflow".to_string(), pos));
            }
            let mut r = 1.0;
            for i in 1..=(v as u64) {
                r *= i as f64;
            }
            Ok(r)
        }
        Expr::Unary(op, e) => {
            let v = evaluate(e, vars)?;
            Ok(if let UnOp::Neg = op { -v } else { v })
        }
    }
}

fn report_err(input: &str, msg: &str, pos: usize) {
    println!("{}\n{}{}^-- {}{}", input, RED, " ".repeat(pos), msg, RESET);
}

fn process_input(input: &str, vars: &mut HashMap<String, f64>) {
    for part in input.split(';') {
        let t = part.trim();
        if t.is_empty() {
            continue;
        }
                if t.eq_ignore_ascii_case("help") {
                    println!("{}rcal v{}{}", BOLD, env!("CARGO_PKG_VERSION"), RESET);
                    println!("\n{}Available Operations:{}", BOLD, RESET);
                    println!("  +, -, *, /, %, ^, ! (factorial)");
                    println!("  = (assignment), ; (separator), , (arguments)");
                    
                    println!("\n{}Available Functions:{}", BOLD, RESET);
                    println!("  {}Trigonometric:{} sin, cos, tan, asin, acos, atan", BOLD, RESET);
                    println!("  {}Math:{}          sqrt, abs, ln, log, round(val, places)", BOLD, RESET);
                    println!("  {}Aggregates:{}    sum, avg, min, max", BOLD, RESET);
                    println!("  {}Bitwise:{}       and, or, xor, not, lshift, rshift", BOLD, RESET);
                    println!("  {}Formatting:{}    hex, bin", BOLD, RESET);
                    
                    println!("\n{}Constants & Special:{}", BOLD, RESET);
                    println!("  pi, e, deg, ans");
                    
                    println!("\n{}Commands:{}", BOLD, RESET);
                    println!("  help, vars, exit, quit\n");
                    continue;
                }
        
        if t.eq_ignore_ascii_case("vars") {
            for (k, v) in &*vars {
                println!("{}: {}", k, v);
            }
            continue;
        }
        match tokenize(t).and_then(|toks| {
            let mut p = Parser::new(toks);
            let ast = p.parse_expr()?;
            if p.cur().kind != TokenKind::EOF {
                return Err(("SyntaxError: Unexpected character".to_string(), p.cur().pos));
            }
            evaluate(&ast, vars).map(|v| (v, ast))
        }) {
            Ok((v, ast)) => {
                vars.insert("ans".to_string(), v);
                if !matches!(ast.expr, Expr::Assign(_, _)) {
                    let norm = if v == 0.0 { 0.0 } else { v };
                    if let Expr::Function(n, _) = &ast.expr {
                        if n == "hex" {
                            println!("{}= 0x{:x}{}", GREEN, norm as u64, RESET);
                        } else if n == "bin" {
                            println!("{}= 0b{:b}{}", GREEN, norm as u64, RESET);
                        } else {
                            println!("{}= {}{}", GREEN, norm, RESET);
                        }
                    } else {
                        println!("{}= {}{}", GREEN, norm, RESET);
                    }
                }
            }
            Err((e, p)) => report_err(t, &e, p),
        }
    }
}

fn main() {
    let mut vars = HashMap::new();
    let args: Vec<_> = std::env::args().collect();
    if args.len() > 1 {
        process_input(&args[1..].join(" "), &mut vars);
        return;
    }
    println!(
        "{}rcal v{}{}\nType 'help' for info or 'exit' to quit\n",
        BOLD,
        env!("CARGO_PKG_VERSION"),
        RESET
    );
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let trimmed = input.trim();
        if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
            break;
        }
        process_input(trimmed, &mut vars);
    }
}
