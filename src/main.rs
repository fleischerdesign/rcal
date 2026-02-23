use std::io::{self, Write};

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    EOF,
}

enum Expr {
    Number(f64),
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
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
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() || ch == '.' {
                        num_str.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                if let Ok(num) = num_str.parse::<f64>() {
                    tokens.push(Token::Number(num));
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

    fn consume(&mut self) {
        self.pos += 1;
    }

    fn parse_expr(&mut self) -> Result<Box<Expr>, String> {
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
        let mut left = self.parse_factor()?;

        loop {
            match self.current() {
                Token::Multiply | Token::Divide => {
                    let op = self.current().clone();
                    self.consume();
                    let right = self.parse_factor()?;

                    if op == Token::Multiply {
                        left = Box::new(Expr::Multiply(left, right));
                    } else {
                        left = Box::new(Expr::Divide(left, right));
                    }
                }
                Token::LParen => {
                    let right = self.parse_factor()?;
                    left = Box::new(Expr::Multiply(left, right));
                }
                _ => break,
            }
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Box<Expr>, String> {
        match self.current().clone() {
            Token::Number(n) => {
                self.consume();
                Ok(Box::new(Expr::Number(n)))
            }
            Token::LParen => {
                self.consume();
                let expr = self.parse_expr()?;
                if self.current() == &Token::RParen {
                    self.consume();
                    Ok(expr)
                } else {
                    Err("SyntaxtError: Lacking closing parenthesis".to_string())
                }
            }
            Token::EOF => Err("Unexpected ending of input".to_string()),
            _ => Err("SyntaxError: Unexpexted character".to_string()),
        }
    }
}

fn evaluate(expr: &Expr) -> f64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Add(l, r) => evaluate(l) + evaluate(r),
        Expr::Subtract(l, r) => evaluate(l) - evaluate(r),
        Expr::Multiply(l, r) => evaluate(l) * evaluate(r),
        Expr::Divide(l, r) => evaluate(l) / evaluate(r),
    }
}

fn main() {
    println!("rcal v0.1");
    println!("please provide a mathematical input or type 'exit'\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            println!("Bye!");
            break;
        }
        if input.is_empty() {
            continue;
        }

        let tokens = match tokenize(input) {
            Ok(t) => t,
            Err(e) => {
                println!("{}\n", e);
                continue;
            }
        };

        let mut parser = Parser::new(tokens);

        match parser.parse_expr() {
            Ok(ast) => {
                if parser.current() != &Token::EOF {
                    println!("SyntaxError: Unexpected character on end of expression");
                    continue;
                }

                let result = evaluate(&ast);
                if result.is_infinite() || result.is_nan() {
                    println!("Error: Dividing by Null is not allowed!");
                } else {
                    println!("= {}\n", result);
                }
            }
            Err(e) => {
                println!("{}\n", e);
            }
        }
    }
}
