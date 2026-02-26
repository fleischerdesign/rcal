mod ast;
mod calculator;
mod cli;
mod error;
mod evaluator;
mod lexer;
mod parser;

use cli::Cli;

fn main() {
    let mut cli = Cli::new();
    cli.run();
}
