use std::io::{self, Write};
use crate::calculator::Calculator;
use crate::ast::Expr;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";

pub struct Cli {
    calc: Calculator,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            calc: Calculator::new(),
        }
    }

    pub fn run(&mut self) {
        let args: Vec<_> = std::env::args().collect();
        
        if args.len() > 1 {
            self.execute(&args[1..].join(" "));
            return;
        }

        println!(
            "{}rcal v{}{}
Type 'help' for info or 'exit' to quit
",
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
            
            self.execute(trimmed);
        }
    }

    fn execute(&mut self, input: &str) {
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

            match self.calc.eval(t) {
                Ok((v, expr)) => {
                    if !matches!(expr, Expr::Assign(_, _)) {
                        let norm = if v == 0.0 { 0.0 } else { v };
                        if let Expr::Function(n, _) = expr {
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
        let vars = self.calc.vars();
        if vars.is_empty() {
            println!("No variables defined.");
            return;
        }
        for (k, v) in vars {
            println!("{}: {}", k, v);
        }
    }
}
