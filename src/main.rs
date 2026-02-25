mod ast;
mod error;
mod evaluator;
mod lexer;
mod parser;

use std::collections::HashMap;
use std::io::{self, Write};

use ast::Expr;
use error::RcalError;
use evaluator::evaluate;
use lexer::{TokenKind, tokenize};
use parser::Parser;

const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

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
            println!(
                "  {}Trigonometric:{} sin, cos, tan, asin, acos, atan",
                BOLD, RESET
            );
            println!(
                "  {}Math:{}          sqrt, abs, ln, log, round(val, places)",
                BOLD, RESET
            );
            println!("  {}Aggregates:{}    sum, avg, min, max", BOLD, RESET);
            println!(
                "  {}Bitwise:{}       and, or, xor, not, lshift, rshift",
                BOLD, RESET
            );
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
        let result = tokenize(t).and_then(|toks| {
            let mut p = Parser::new(toks);
            let ast = p.parse_expr()?;
            if p.cur().kind != TokenKind::EOF {
                return Err(RcalError::Parser(
                    "Unexpected character".to_string(),
                    p.cur().pos,
                ));
            }
            let v = evaluate(&ast, vars)?;
            Ok((v, ast))
        });

        match result {
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
            Err(e) => e.report(t),
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
