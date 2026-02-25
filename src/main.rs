mod ast;
mod calculator;
mod error;
mod evaluator;
mod lexer;
mod parser;

use std::io::{self, Write};
use calculator::Calculator;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";

fn main() {
    let mut calc = Calculator::new();
    let args: Vec<_> = std::env::args().collect();
    
    if args.len() > 1 {
        calc.execute(&args[1..].join(" "));
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
        
        calc.execute(trimmed);
    }
}
