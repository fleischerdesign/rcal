//! Command-line interface and interactive shell.

use crate::ast::Expr;
use crate::builtins::format_as;
use crate::calculator::Calculator;
use crate::completer::RcalHelper;
use rustyline::error::ReadlineError;
use rustyline::{Config, Editor};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GREEN: &str = "\x1b[32m";

/// The rcal command-line interface.
pub struct Cli {
    calc: Calculator,
}

impl Cli {
    /// Creates a new CLI instance.
    pub fn new() -> Self {
        Self {
            calc: Calculator::new(),
        }
    }

    /// Returns the path to the history file.
    fn history_path() -> Option<std::path::PathBuf> {
        #[allow(deprecated)]
        std::env::home_dir().map(|mut p| {
            p.push(".rcal_history");
            p
        })
    }

    /// Runs the CLI in interactive or batch mode.
    pub fn run(&mut self) {
        let args: Vec<_> = std::env::args().collect();

        if args.len() > 1 {
            let path = std::path::Path::new(&args[1]);
            if path.exists() && path.is_file() {
                match std::fs::read_to_string(path) {
                    Ok(content) => {
                        for (i, line) in content.lines().enumerate() {
                            self.execute(line, Some(i + 1));
                        }
                    }
                    Err(e) => {
                        crate::error::Error::Cli(format!("Failed to read file: {}", e)).report();
                    }
                }
            } else {
                self.execute(&args[1..].join(" "), None);
            }
            return;
        }

        println!(
            "{}rcal v{}{}\nType 'help' for info or 'exit' to quit\n",
            BOLD,
            env!("CARGO_PKG_VERSION"),
            RESET
        );

        let config = Config::builder().build();
        let mut rl = Editor::<RcalHelper, _>::with_config(config).expect("Failed to create editor");
        rl.set_helper(Some(RcalHelper));

        let h_path = Self::history_path();
        if let Some(ref path) = h_path {
            match rl.load_history(path) {
                Err(ReadlineError::Io(ref io_err))
                    if io_err.kind() == std::io::ErrorKind::NotFound => {}
                Err(e) => {
                    crate::error::Error::Cli(format!("Failed to load history: {}", e)).report();
                }
                _ => {}
            }
        }

        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() {
                        continue;
                    }

                    if trimmed.eq_ignore_ascii_case("exit") || trimmed.eq_ignore_ascii_case("quit")
                    {
                        break;
                    }

                    let _ = rl.add_history_entry(trimmed);
                    self.execute(trimmed, None);
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    crate::error::Error::Cli(format!("Readline error: {}", err)).report();
                    break;
                }
            }
        }

        if let Some(ref path) = h_path {
            let _ = rl.save_history(path);
        }
    }

    /// Executes a single line of input (can contain multiple semicolon-separated statements).
    fn execute(&mut self, input: &str, line_num: Option<usize>) {
        for part in input.split(';') {
            let t = part.trim();
            if crate::lexer::is_comment_or_empty(t) {
                continue;
            }

            if t.eq_ignore_ascii_case("help") {
                self.print_help();
                continue;
            }

            if t.eq_ignore_ascii_case("list") {
                self.print_list();
                continue;
            }

            match self.calc.eval(t) {
                Ok((v, expr)) => {
                    self.handle_result(v, expr);
                }
                Err(e) => {
                    e.report_at(t, line_num);
                }
            }
        }
    }

    /// Handles the result of an evaluation, including special formatting for `Convert`.
    fn handle_result(&self, v: crate::unit::Quantity, expr: crate::ast::Expr) {
        if matches!(expr, Expr::Assign(_, _)) || matches!(expr, Expr::FnDefine(_, _, _)) {
            return;
        }

        if let Expr::Convert(_, ref target_node) = expr {
            if let Expr::Variable(name) = &target_node.expr
                && let Some(formatted) = format_as(name, v.value)
            {
                println!("{}= {}{}", GREEN, formatted, RESET);
                return;
            }
            println!("{}= {} {}{}", GREEN, v.value, target_node, RESET);
        } else {
            println!("{}= {}{}", GREEN, v, RESET);
        }
    }

    /// Prints the help message.
    fn print_help(&self) {
        println!("{}rcal v{}{}", BOLD, env!("CARGO_PKG_VERSION"), RESET);
        println!("\n{}Available Operations:{}", BOLD, RESET);
        println!("  +, -, *, /, %, ^, ! (factorial)");
        println!("  = (assignment), in (conversion)");
        println!("  ; (separator), , (arguments)");

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
        println!("  help, list, exit, quit\n");
    }

    /// Prints all user-defined variables and functions.
    fn print_list(&self) {
        let vars = self.calc.scope.vars();
        let funcs = self.calc.scope.funcs();

        if vars.is_empty() && funcs.is_empty() {
            println!("No variables or functions defined.");
            return;
        }

        let mut v_keys: Vec<_> = vars.keys().collect();
        v_keys.sort();
        for k in v_keys {
            if let Some(v) = vars.get(k) {
                println!("{}: {}", k, v);
            }
        }

        let mut f_keys: Vec<_> = funcs.keys().collect();
        f_keys.sort();
        for k in f_keys {
            if let Some(f) = funcs.get(k) {
                println!("{}({}) = {}", k, f.params.join(", "), f.body);
            }
        }
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
