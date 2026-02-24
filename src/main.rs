use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Token {
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
    LParen,
    RParen,
    EOF,
}

enum Expr {
    Number(f64),
    Variable(String),
    Assign(String, Box<Expr>),
    Function(String, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Modulo(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
    Factorial(Box<Expr>),
    UnaryMinus(Box<Expr>),
    UnaryPlus(Box<Expr>),
}

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\n' | '\r' | '\t' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                tokens.push(Token::Multiply);
                chars.next();
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '%' => {
                tokens.push(Token::Modulo);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Power);
                chars.next();
            }
            '!' => {
                tokens.push(Token::Factorial);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Assign);
                chars.next();
            }
            ';' => {
                tokens.push(Token::Semicolon);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut ident = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() || ch == '_' {
                        ident.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Identifier(ident));
            }
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' || ch == 'e' || ch == 'E' {
                        let current = chars.next().unwrap();
                        num_str.push(current);
                        if (current == 'e' || current == 'E') && (chars.peek() == Some(&'-') || chars.peek() == Some(&'+')) {
                            num_str.push(chars.next().unwrap());
                        }
                    } else {
                        break;
                    }
                }
                if let Ok(num) = num_str.parse::<f64>() {
                    tokens.push(Token::Number(num));
                } else {
                    return Err(format!("LexerError: Invalid number format '{}'", num_str));
                }
            }
            _ => return Err(format!("LexerError: Unexpected character '{}'", c)),
        }
    }
    tokens.push(Token::EOF);
    Ok(tokens)
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
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos + 1).unwrap_or(&Token::EOF)
    }

    fn consume(&mut self) {
        self.pos += 1;
    }

    fn parse_expr(&mut self) -> Result<Box<Expr>, String> {
        if let Token::Identifier(name) = self.current().clone() {
            if self.peek() == &Token::Assign {
                self.consume();
                self.consume();
                let right = self.parse_expr()?;
                return Ok(Box::new(Expr::Assign(name.to_lowercase(), right)));
            }
        }
        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Result<Box<Expr>, String> {
        let mut left = self.parse_term()?;

        while self.current() == &Token::Plus || self.current() == &Token::Minus {
            let op = self.current().clone();
            self.consume();
            let right = self.parse_term()?;

            if op == Token::Plus {
                left = Box::new(Expr::Add(left, right));
            } else {
                left = Box::new(Expr::Subtract(left, right));
            }
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Box<Expr>, String> {
        let mut left = self.parse_power()?;

        loop {
            match self.current() {
                Token::Multiply | Token::Divide | Token::Modulo => {
                    let op = self.current().clone();
                    self.consume();
                    let right = self.parse_power()?;

                    match op {
                        Token::Multiply => {
                            left = Box::new(Expr::Multiply(left, right));
                        }
                        Token::Divide => {
                            left = Box::new(Expr::Divide(left, right));
                        }
                        Token::Modulo => {
                            left = Box::new(Expr::Modulo(left, right));
                        }
                        _ => unreachable!(),
                    }
                }
                Token::LParen | Token::Identifier(_) | Token::Number(_) => {
                    let right = self.parse_power()?;
                    left = Box::new(Expr::Multiply(left, right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Box<Expr>, String> {
        let left = self.parse_postfix()?;

        if self.current() == &Token::Power {
            self.consume();
            let right = self.parse_power()?;
            Ok(Box::new(Expr::Power(left, right)))
        } else {
            Ok(left)
        }
    }

    fn parse_postfix(&mut self) -> Result<Box<Expr>, String> {
        let mut left = self.parse_unary()?;

        while self.current() == &Token::Factorial {
            self.consume();
            left = Box::new(Expr::Factorial(left));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Box<Expr>, String> {
        match self.current() {
            Token::Minus => {
                self.consume();
                let expr = self.parse_unary()?;
                Ok(Box::new(Expr::UnaryMinus(expr)))
            }
            Token::Plus => {
                self.consume();
                let expr = self.parse_unary()?;
                Ok(Box::new(Expr::UnaryPlus(expr)))
            }
            _ => self.parse_factor(),
        }
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, String> {
        match self.current().clone() {
            Token::Number(n) => {
                self.consume();
                Ok(Box::new(Expr::Number(n)))
            }
            Token::Identifier(name) => {
                self.consume();
                let name = name.to_lowercase();
                if self.current() == &Token::LParen {
                    self.consume();
                    let arg = self.parse_expr()?;
                    if self.current() == &Token::RParen {
                        self.consume();
                        Ok(Box::new(Expr::Function(name, arg)))
                    } else {
                        Err(
                            "SyntaxError: Lacking closing parenthesis after function argument"
                                .to_string(),
                        )
                    }
                } else {
                    Ok(Box::new(Expr::Variable(name)))
                }
            }
            Token::LParen => {
                self.consume();
                let expr = self.parse_expr()?;
                if self.current() == &Token::RParen {
                    self.consume();
                    Ok(expr)
                } else {
                    Err("SyntaxError: Lacking closing parenthesis".to_string())
                }
            }
            Token::EOF => Err("Unexpected ending of input".to_string()),
            _ => Err(format!(
                "SyntaxError: Unexpected token {:?}",
                self.current()
            )),
        }
    }
}

fn factorial(n: f64) -> Result<f64, String> {
    if n < 0.0 || n.fract() != 0.0 {
        return Err("Math Error: Factorial only defined for non-negative integers".to_string());
    }
    if n > 170.0 {
        return Err("Math Error: Factorial overflow (too large)".to_string());
    }
    let mut res = 1.0;
    for i in 1..=(n as u64) {
        res *= i as f64;
    }
    Ok(res)
}

fn evaluate(expr: &Expr, vars: &mut HashMap<String, f64>) -> Result<f64, String> {
    match expr {
        Expr::Number(n) => Ok(*n),
        Expr::Variable(name) => {
            if name == "pi" { return Ok(std::f64::consts::PI); }
            if name == "e" { return Ok(std::f64::consts::E); }
            if name == "deg" { return Ok(std::f64::consts::PI / 180.0); }
            vars.get(name).copied().ok_or_else(|| format!("Math Error: Unknown variable '{}'", name))
        }
        Expr::Assign(name, val_expr) => {
            let val = evaluate(val_expr, vars)?;
            vars.insert(name.clone(), val);
            Ok(val)
        }
        Expr::Function(name, arg) => {
            let val = evaluate(arg, vars)?;
            match name.as_str() {
                "sin" => Ok(val.sin()),
                "cos" => Ok(val.cos()),
                "tan" => Ok(val.tan()),
                "asin" => Ok(val.asin()),
                "acos" => Ok(val.acos()),
                "atan" => Ok(val.atan()),
                "sqrt" => {
                    if val < 0.0 {
                        return Err("Math Error: Square root of negative number".to_string());
                    }
                    Ok(val.sqrt())
                }
                "abs" => Ok(val.abs()),
                "ln" => {
                    if val <= 0.0 {
                        return Err(
                            "Math Error: Natural logarithm of zero or negative number".to_string()
                        );
                    }
                    Ok(val.ln())
                }
                "log" => {
                    if val <= 0.0 {
                        return Err("Math Error: Logarithm of zero or negative number".to_string());
                    }
                    Ok(val.log10())
                }
                _ => Err(format!("Math Error: Unknown function '{}'", name)),
            }
        }
        Expr::Add(l, r) => Ok(evaluate(l, vars)? + evaluate(r, vars)?),
        Expr::Subtract(l, r) => Ok(evaluate(l, vars)? - evaluate(r, vars)?),
        Expr::Multiply(l, r) => Ok(evaluate(l, vars)? * evaluate(r, vars)?),
        Expr::Divide(l, r) => {
            let left = evaluate(l, vars)?;
            let right = evaluate(r, vars)?;
            if right == 0.0 {
                return Err("Math Error: Division by zero".to_string());
            }
            let res = left / right;
            if res.is_infinite() {
                return Err("Math Error: Overflow".to_string());
            }
            Ok(res)
        }
        Expr::Modulo(l, r) => {
            let left = evaluate(l, vars)?;
            let right = evaluate(r, vars)?;
            if right == 0.0 {
                return Err("Math Error: Modulo by zero".to_string());
            }
            Ok(left % right)
        }
        Expr::Power(l, r) => {
            let left = evaluate(l, vars)?;
            let right = evaluate(r, vars)?;
            let res = left.powf(right);
            if res.is_infinite() {
                return Err("Math Error: Overflow (Result too large)".to_string());
            }
            if res.is_nan() {
                return Err(
                    "Math Error: Invalid operation (e.g. negative base with fractional exponent)"
                        .to_string(),
                );
            }
            Ok(res)
        }
        Expr::Factorial(e) => factorial(evaluate(e, vars)?),
        Expr::UnaryMinus(e) => Ok(-evaluate(e, vars)?),
        Expr::UnaryPlus(e) => evaluate(e, vars),
    }
}

fn process_input(input: &str, vars: &mut HashMap<String, f64>) {
    let parts: Vec<&str> = input.split(';').collect();

    for part in parts {
        let trimmed = part.trim();
        if trimmed.is_empty() {
            continue;
        }

        let tokens = match tokenize(trimmed) {
            Ok(t) => t,
            Err(e) => {
                println!("{}\n", e);
                return;
            }
        };

        let mut parser = Parser::new(tokens);

        match parser.parse_expr() {
            Ok(ast) => {
                if parser.current() != &Token::EOF {
                    println!("SyntaxError: Unexpected character on end of expression\n");
                    return;
                }

                match evaluate(&ast, vars) {
                    Ok(result) => {
                        let normalized = if result == 0.0 { 0.0 } else { result };
                        vars.insert("ans".to_string(), normalized);

                        if !matches!(*ast, Expr::Assign(_, _)) {
                            println!("= {}\n", normalized);
                        }
                    }
                    Err(e) => println!("{}\n", e),
                }
            }
            Err(e) => {
                println!("{}\n", e);
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
