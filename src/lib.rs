//! # rcal
//!
//! `rcal` is a professional-grade dimensional analysis calculator engine.
//! It supports units with offsets (like Celsius), hierarchical scoping for functions,
//! and a robust error handling system.

pub mod ast;
pub mod builtins;
pub mod calculator;
pub mod cli;
pub mod completer;
pub mod error;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod unit;

pub use calculator::Calculator;
pub use error::Error;
pub use unit::Quantity;
