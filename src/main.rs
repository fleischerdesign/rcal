mod ast;
mod builtins;
mod calculator;
mod cli;
mod completer;
mod error;
mod evaluator;
mod lexer;
mod parser;
mod unit;

use cli::Cli;

fn main() {
    let mut cli = Cli::new();
    cli.run();
}
