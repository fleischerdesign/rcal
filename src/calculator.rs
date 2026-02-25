use std::collections::HashMap;
use crate::lexer::{tokenize, TokenKind};
use crate::parser::Parser;
use crate::evaluator::evaluate;
use crate::ast::Expr;

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

pub struct Calculator {
    vars: HashMap<String, f64>,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn execute(&mut self, input: &str) {
        for part in input.split(';') {
            let t = part.trim();
            if t.is_empty() {
                continue;
            }

            if t.eq_ignore_ascii_case("help") {
                self.print_help();
                continue;
            }

            if t.eq_ignore_ascii_case("vars") {
                self.print_vars();
                continue;
            }

            match tokenize(t).and_then(|toks| {
                let mut p = Parser::new(toks);
                let ast = p.parse_expr()?;
                if p.cur().kind != TokenKind::EOF {
                    return Err(crate::error::RcalError::Parser(
                        "Unexpected character".to_string(),
                        p.cur().pos,
                    ));
                }
                let v = evaluate(&ast, &mut self.vars)?;
                Ok((v, ast))
            }) {
                Ok((v, ast)) => {
                    self.vars.insert("ans".to_string(), v);
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
                Err(e) => e.report(t),
            }
        }
    }

    fn print_help(&self) {
        println!("{}rcal v{}{}", BOLD, env!("CARGO_PKG_VERSION"), RESET);
        println!("
{}Available Operations:{}", BOLD, RESET);
        println!("  +, -, *, /, %, ^, ! (factorial)");
        println!("  = (assignment), ; (separator), , (arguments)");

        println!("
{}Available Functions:{}", BOLD, RESET);
        println!("  {}Trigonometric:{} sin, cos, tan, asin, acos, atan", BOLD, RESET);
        println!("  {}Math:{}          sqrt, abs, ln, log, round(val, places)", BOLD, RESET);
        println!("  {}Aggregates:{}    sum, avg, min, max", BOLD, RESET);
        println!("  {}Bitwise:{}       and, or, xor, not, lshift, rshift", BOLD, RESET);
        println!("  {}Formatting:{}    hex, bin", BOLD, RESET);

        println!("
{}Constants & Special:{}", BOLD, RESET);
        println!("  pi, e, deg, ans");

        println!("
{}Commands:{}", BOLD, RESET);
        println!("  help, vars, exit, quit
");
    }

    fn print_vars(&self) {
        if self.vars.is_empty() {
            println!("No variables defined.");
            return;
        }
        for (k, v) in &self.vars {
            println!("{}: {}", k, v);
        }
    }
}
