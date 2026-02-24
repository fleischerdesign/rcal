use std::collections::HashMap;
use std::io::{self, Write};

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
        match c {
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token { kind: TokenKind::Plus, pos: i });
                chars.next();
            }
            '-' => {
                tokens.push(Token { kind: TokenKind::Minus, pos: i });
                chars.next();
            }
            '*' => {
                tokens.push(Token { kind: TokenKind::Multiply, pos: i });
                chars.next();
            }
            '/' => {
                tokens.push(Token { kind: TokenKind::Divide, pos: i });
                chars.next();
            }
            '%' => {
                tokens.push(Token { kind: TokenKind::Modulo, pos: i });
                chars.next();
            }
            '^' => {
                tokens.push(Token { kind: TokenKind::Power, pos: i });
                chars.next();
            }
            '!' => {
                tokens.push(Token { kind: TokenKind::Factorial, pos: i });
                chars.next();
            }
            '=' => {
                tokens.push(Token { kind: TokenKind::Assign, pos: i });
                chars.next();
            }
            ';' => {
                tokens.push(Token { kind: TokenKind::Semicolon, pos: i });
                chars.next();
            }
            ',' => {
                tokens.push(Token { kind: TokenKind::Comma, pos: i });
                chars.next();
            }
            '(' => {
                tokens.push(Token { kind: TokenKind::LParen, pos: i });
                chars.next();
            }
            ')' => {
                tokens.push(Token { kind: TokenKind::RParen, pos: i });
                chars.next();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let start_pos = i;
                let mut ident = String::new();
                while let Some(&(_, ch)) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token { kind: TokenKind::Identifier(ident), pos: start_pos });
            }
            '0'..='9' | '.' => {
                let start_pos = i;
                let mut num_str = String::new();
                if c == '0' {
                    chars.next();
                    if let Some(&(_, next_c)) = chars.peek() {
                        if next_c == 'x' || next_c == 'X' {
                            chars.next();
                            let mut hex_str = String::new();
                            while let Some(&(_, ch)) = chars.peek() {
                                if ch.is_ascii_hexdigit() {
                                    hex_str.push(ch);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            if let Ok(num) = u64::from_str_radix(&hex_str, 16) {
                                tokens.push(Token { kind: TokenKind::Number(num as f64), pos: start_pos });
                                continue;
                            } else {
                                return Err((format!("LexerError: Invalid hex format '0x{}'", hex_str), start_pos));
                            }
                        } else if next_c == 'b' || next_c == 'B' {
                            chars.next();
                            let mut bin_str = String::new();
                            while let Some(&(_, ch)) = chars.peek() {
                                if ch == '0' || ch == '1' {
                                    bin_str.push(ch);
                                    chars.next();
                                } else {
                                    break;
                                }
                            }
                            if let Ok(num) = u64::from_str_radix(&bin_str, 2) {
                                tokens.push(Token { kind: TokenKind::Number(num as f64), pos: start_pos });
                                continue;
                            } else {
                                return Err((format!("LexerError: Invalid binary format '0b{}'", bin_str), start_pos));
                            }
                        }
                        num_str.push('0');
                    } else {
                        tokens.push(Token { kind: TokenKind::Number(0.0), pos: start_pos });
                        continue;
                    }
                }

                while let Some(&(_, ch)) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' {
                        num_str.push(ch);
                        chars.next();
                        if (ch == 'e' || ch == 'E') && chars.peek().map_or(false, |&(_, next_c)| next_c == '-' || next_c == '+') {
                            let (_, sign) = chars.next().unwrap();
                            num_str.push(sign);
                        }
                    } else {
                        break;
                    }
                }
                if let Ok(num) = num_str.parse::<f64>() {
                    if num.is_infinite() {
                        return Err((format!("LexerError: Number '{}' is too large (overflow)", num_str), start_pos));
                    }
                    tokens.push(Token { kind: TokenKind::Number(num), pos: start_pos });
                } else {
                    return Err((format!("LexerError: Invalid number format '{}'", num_str), start_pos));
                }
            }
            _ => return Err((format!("LexerError: Unexpected character '{}'", c), i)),
        }
    }
    tokens.push(Token { kind: TokenKind::EOF, pos: input.len() });
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
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
    Modulo(Box<Node>, Box<Node>),
    Power(Box<Node>, Box<Node>),
    Factorial(Box<Node>),
    UnaryMinus(Box<Node>),
    UnaryPlus(Box<Node>),
}

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&self.tokens[self.tokens.len() - 1])
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos + 1).unwrap_or(&self.tokens[self.tokens.len() - 1])
    }

    fn consume(&mut self) {
        self.pos += 1;
    }

    fn parse_expr(&mut self) -> Result<Box<Node>, (String, usize)> {
        if let TokenKind::Identifier(name) = &self.current().kind {
            if self.peek().kind == TokenKind::Assign {
                let name = name.clone();
                let pos = self.current().pos;
                self.consume(); // name
                self.consume(); // =
                let right = self.parse_expr()?;
                return Ok(Box::new(Node { expr: Expr::Assign(name.to_lowercase(), right), pos }));
            }
        }
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Box<Node>, (String, usize)> {
        let mut left = self.parse_term()?;

        while self.current().kind == TokenKind::Plus || self.current().kind == TokenKind::Minus {
            let op = self.current().kind.clone();
            let pos = self.current().pos;
            self.consume();
            let right = self.parse_term()?;

            if op == TokenKind::Plus {
                left = Box::new(Node { expr: Expr::Add(left, right), pos });
            } else {
                left = Box::new(Node { expr: Expr::Subtract(left, right), pos });
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Node>, (String, usize)> {
        let mut left = self.parse_power()?;

        loop {
            match self.current().kind {
                TokenKind::Multiply | TokenKind::Divide | TokenKind::Modulo => {
                    let op = self.current().kind.clone();
                    let pos = self.current().pos;
                    self.consume();
                    let right = self.parse_power()?;

                    match op {
                        TokenKind::Multiply => {
                            left = Box::new(Node { expr: Expr::Multiply(left, right), pos });
                        }
                        TokenKind::Divide => {
                            left = Box::new(Node { expr: Expr::Divide(left, right), pos });
                        }
                        TokenKind::Modulo => {
                            left = Box::new(Node { expr: Expr::Modulo(left, right), pos });
                        }
                        _ => unreachable!(),
                    }
                }
                TokenKind::LParen | TokenKind::Identifier(_) | TokenKind::Number(_) => {
                    let pos = self.current().pos;
                    let right = self.parse_power()?;
                    left = Box::new(Node { expr: Expr::Multiply(left, right), pos });
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Box<Node>, (String, usize)> {
        let left = self.parse_postfix()?;

        if self.current().kind == TokenKind::Power {
            let pos = self.current().pos;
            self.consume();
            let right = self.parse_power()?;
            Ok(Box::new(Node { expr: Expr::Power(left, right), pos }))
        } else {
            Ok(left)
        }
    }

    fn parse_postfix(&mut self) -> Result<Box<Node>, (String, usize)> {
        let mut left = self.parse_unary()?;

        while self.current().kind == TokenKind::Factorial {
            let pos = self.current().pos;
            self.consume();
            left = Box::new(Node { expr: Expr::Factorial(left), pos });
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Box<Node>, (String, usize)> {
        let pos = self.current().pos;
        match self.current().kind {
            TokenKind::Minus => {
                self.consume();
                let expr = self.parse_unary()?;
                Ok(Box::new(Node { expr: Expr::UnaryMinus(expr), pos }))
            }
            TokenKind::Plus => {
                self.consume();
                let expr = self.parse_unary()?;
                Ok(Box::new(Node { expr: Expr::UnaryPlus(expr), pos }))
            }
            _ => self.parse_factor(),
        }
    }

    fn parse_factor(&mut self) -> Result<Box<Node>, (String, usize)> {
        let pos = self.current().pos;
        match self.current().kind.clone() {
            TokenKind::Number(n) => {
                self.consume();
                Ok(Box::new(Node { expr: Expr::Number(n), pos }))
            }
            TokenKind::Identifier(name) => {
                self.consume();
                let name = name.to_lowercase();
                if self.current().kind == TokenKind::LParen {
                    self.consume();
                    let mut args = Vec::new();
                    if self.current().kind != TokenKind::RParen {
                        loop {
                            args.push(self.parse_expr()?);
                            if self.current().kind == TokenKind::Comma {
                                self.consume();
                            } else {
                                break;
                            }
                        }
                    }
                    if self.current().kind == TokenKind::RParen {
                        self.consume();
                        Ok(Box::new(Node { expr: Expr::Function(name, args), pos }))
                    } else {
                        Err((
                            "SyntaxError: Lacking closing parenthesis after function arguments".to_string(),
                            self.current().pos
                        ))
                    }
                } else {
                    Ok(Box::new(Node { expr: Expr::Variable(name), pos }))
                }
            }
            TokenKind::LParen => {
                self.consume();
                let expr = self.parse_expr()?;
                if self.current().kind == TokenKind::RParen {
                    self.consume();
                    Ok(expr)
                } else {
                    Err(("SyntaxError: Lacking closing parenthesis".to_string(), self.current().pos))
                }
            }
            TokenKind::EOF => Err(("Unexpected ending of input".to_string(), self.current().pos)),
            _ => Err((format!(
                "SyntaxError: Unexpected token {:?}",
                self.current().kind
            ), self.current().pos)),
        }
    }
}

fn factorial(n: f64, pos: usize) -> Result<f64, (String, usize)> {
    if n < 0.0 || n.fract() != 0.0 {
        return Err(("Math Error: Factorial only defined for non-negative integers".to_string(), pos));
    }
    if n > 170.0 {
        return Err(("Math Error: Factorial overflow (too large)".to_string(), pos));
    }
    let mut res = 1.0;
    for i in 1..=(n as u64) {
        res *= i as f64;
    }
    Ok(res)
}

fn evaluate(node: &Node, vars: &mut HashMap<String, f64>) -> Result<f64, (String, usize)> {
    let pos = node.pos;
    match &node.expr {
        Expr::Number(n) => Ok(*n),
        Expr::Variable(name) => {
            if name == "pi" { return Ok(std::f64::consts::PI); }
            if name == "e" { return Ok(std::f64::consts::E); }
            if name == "deg" { return Ok(std::f64::consts::PI / 180.0); }
            vars.get(name).copied().ok_or_else(|| (format!("Math Error: Unknown variable '{}'", name), pos))
        }
        Expr::Assign(name, val_expr) => {
            let val = evaluate(val_expr, vars)?;
            vars.insert(name.clone(), val);
            Ok(val)
        }
        Expr::Function(name, args) => {
            let mut vals = Vec::new();
            for arg in args {
                vals.push(evaluate(arg, vars)?);
            }
            
            match name.as_str() {
                "sin" if vals.len() == 1 => Ok(vals[0].sin()),
                "cos" if vals.len() == 1 => Ok(vals[0].cos()),
                "tan" if vals.len() == 1 => Ok(vals[0].tan()),
                "asin" if vals.len() == 1 => Ok(vals[0].asin()),
                "acos" if vals.len() == 1 => Ok(vals[0].acos()),
                "atan" if vals.len() == 1 => Ok(vals[0].atan()),
                "sqrt" if vals.len() == 1 => {
                    if vals[0] < 0.0 {
                        return Err(("Math Error: Square root of negative number".to_string(), pos));
                    }
                    Ok(vals[0].sqrt())
                }
                "abs" if vals.len() == 1 => Ok(vals[0].abs()),
                "hex" | "bin" if vals.len() == 1 => {
                    if vals[0] < 0.0 || vals[0] > u64::MAX as f64 || vals[0].fract() != 0.0 {
                        return Err(("Math Error: Value out of range for hex/bin (0 to 2^64-1, integers only)".to_string(), pos));
                    }
                    Ok(vals[0])
                }
                "ln" if vals.len() == 1 => {
                    if vals[0] <= 0.0 {
                        return Err(("Math Error: Natural logarithm of zero or negative number".to_string(), pos));
                    }
                    Ok(vals[0].ln())
                }
                "log" if vals.len() == 1 => {
                    if vals[0] <= 0.0 {
                        return Err(("Math Error: Logarithm of zero or negative number".to_string(), pos));
                    }
                    Ok(vals[0].log10())
                }
                "max" if !vals.is_empty() => Ok(vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
                "min" if !vals.is_empty() => Ok(vals.iter().cloned().fold(f64::INFINITY, f64::min)),
                "sum" => Ok(vals.iter().sum()),
                "avg" if !vals.is_empty() => Ok(vals.iter().sum::<f64>() / vals.len() as f64),
                // Programmer features (Bit-functions)
                "and" if vals.len() == 2 => Ok((vals[0] as u64 & vals[1] as u64) as f64),
                "or" if vals.len() == 2 => Ok((vals[0] as u64 | vals[1] as u64) as f64),
                "xor" if vals.len() == 2 => Ok((vals[0] as u64 ^ vals[1] as u64) as f64),
                "not" if vals.len() == 1 => Ok((!(vals[0] as u64)) as f64),
                "lshift" if vals.len() == 2 => Ok(((vals[0] as u64) << vals[1] as u64) as f64),
                "rshift" if vals.len() == 2 => Ok(((vals[0] as u64) >> vals[1] as u64) as f64),
                _ => {
                    if ["sin", "cos", "tan", "asin", "acos", "atan", "sqrt", "abs", "hex", "bin", "ln", "log", "not"].contains(&name.as_str()) && vals.len() != 1 {
                        Err((format!("Math Error: Function '{}' expects 1 argument", name), pos))
                    } else if ["and", "or", "xor", "lshift", "rshift"].contains(&name.as_str()) && vals.len() != 2 {
                        Err((format!("Math Error: Function '{}' expects 2 arguments", name), pos))
                    } else if ["max", "min", "avg"].contains(&name.as_str()) && vals.is_empty() {
                        Err((format!("Math Error: Function '{}' expects at least 1 argument", name), pos))
                    } else {
                        Err((format!("Math Error: Unknown function '{}' or wrong number of arguments", name), pos))
                    }
                }
            }
        }
        Expr::Add(l, r) => {
            let res = evaluate(l, vars)? + evaluate(r, vars)?;
            if res.is_infinite() { return Err(("Math Error: Overflow".to_string(), pos)); }
            Ok(res)
        }
        Expr::Subtract(l, r) => {
            let res = evaluate(l, vars)? - evaluate(r, vars)?;
            if res.is_infinite() { return Err(("Math Error: Overflow".to_string(), pos)); }
            Ok(res)
        }
        Expr::Multiply(l, r) => {
            let res = evaluate(l, vars)? * evaluate(r, vars)?;
            if res.is_infinite() { return Err(("Math Error: Overflow".to_string(), pos)); }
            Ok(res)
        }
        Expr::Divide(l, r) => {
            let left = evaluate(l, vars)?;
            let right = evaluate(r, vars)?;
            if right == 0.0 {
                return Err(("Math Error: Division by zero".to_string(), pos));
            }
            let res = left / right;
            if res.is_infinite() {
                return Err(("Math Error: Overflow".to_string(), pos));
            }
            Ok(res)
        }
        Expr::Modulo(l, r) => {
            let left = evaluate(l, vars)?;
            let right = evaluate(r, vars)?;
            if right == 0.0 {
                return Err(("Math Error: Modulo by zero".to_string(), pos));
            }
            Ok(left % right)
        }
        Expr::Power(l, r) => {
            let left = evaluate(l, vars)?;
            let right = evaluate(r, vars)?;
            let res = left.powf(right);
            if res.is_infinite() {
                return Err(("Math Error: Overflow (Result too large)".to_string(), pos));
            }
            if res.is_nan() {
                return Err(("Math Error: Invalid operation (e.g. negative base with fractional exponent)".to_string(), pos));
            }
            Ok(res)
        }
        Expr::Factorial(e) => factorial(evaluate(e, vars)?, pos),
        Expr::UnaryMinus(e) => Ok(-evaluate(e, vars)?),
        Expr::UnaryPlus(e) => evaluate(e, vars),
    }
}

fn print_error(input: &str, msg: &str, pos: usize) {
    println!("{}", input);
    println!("{}^-- {}", " ".repeat(pos), msg);
    println!();
}

fn process_input(input: &str, vars: &mut HashMap<String, f64>) {
    let parts: Vec<&str> = input.split(';').collect();

    for part in parts {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }

        if trimmed.eq_ignore_ascii_case("help") {
            println!("rcal v{}", env!("CARGO_PKG_VERSION"));
            println!("Available Operations:");
            println!("  +, -, *, /, %, ^ (power), ! (factorial)");
            println!("  ; (separate multiple expressions)");
            println!("  = (assignment, e.g., x = 10)");
            println!("\nAvailable Functions:");
            println!("  sin, cos, tan, asin, acos, atan, sqrt, abs, hex, bin, ln, log");
            println!("  max(...), min(...), sum(...), avg(...)");
            println!("  and(a,b), or(a,b), xor(a,b), not(a), lshift(a,n), rshift(a,n)");
            println!("\nAvailable Constants:");
            println!("  pi, e, deg (use as '90 deg' or '90 * deg')");
            println!("\nSpecial Variables:");
            println!("  ans (stores the result of the last calculation)");
            println!("\nCommands:");
            println!("  help, exit, quit\n");
            continue;
        }

        let tokens = match tokenize(trimmed) {
            Ok(t) => t,
            Err((e, pos)) => {
                print_error(trimmed, &e, pos);
                return;
            }
        };

        let mut parser = Parser::new(tokens);

        match parser.parse_expr() {
            Ok(ast) => {
                if parser.current().kind != TokenKind::EOF {
                    print_error(trimmed, "SyntaxError: Unexpected character on end of expression", parser.current().pos);
                    return;
                }

                match evaluate(&ast, vars) {
                    Ok(result) => {
                        let normalized = if result == 0.0 { 0.0 } else { result };
                        vars.insert("ans".to_string(), normalized);

                        if !matches!(ast.expr, Expr::Assign(_, _)) {
                            match &ast.expr {
                                Expr::Function(name, _) if name == "hex" => {
                                    println!("= 0x{:x}\n", normalized as u64);
                                }
                                Expr::Function(name, _) if name == "bin" => {
                                    println!("= 0b{:b}\n", normalized as u64);
                                }
                                _ => {
                                    println!("= {}\n", normalized);
                                }
                            }
                        }
                    }
                    Err((e, pos)) => {
                        print_error(trimmed, &e, pos);
                    }
                }
            }
            Err((e, pos)) => {
                print_error(trimmed, &e, pos);
            }
        }
    }
}

fn main() {
    let mut vars = HashMap::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        let input = args[1..].join(" ");
        process_input(&input, &mut vars);
        return;
    }

    println!("rcal v{}", env!("CARGO_PKG_VERSION"));
    println!("please provide a mathematical input or type 'exit'\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("Bye!");
                break;
            }
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit") {
                    println!("Bye!");
                    break;
                }
                process_input(trimmed, &mut vars);
            }
            Err(e) => {
                eprintln!("Error reading input: {}", e);
                break;
            }
        }
    }
}
