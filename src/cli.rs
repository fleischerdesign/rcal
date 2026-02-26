use crate::ast::Expr;
use crate::calculator::Calculator;
use crate::completer::RcalHelper;
use rustyline::error::ReadlineError;
use rustyline::{Config, Editor};

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

    fn history_path() -> Option<std::path::PathBuf> {
        #[allow(deprecated)]
        std::env::home_dir().map(|mut p| {
            p.push(".rcal_history");
            p
        })
    }

    pub fn run(&mut self) {
        let args: Vec<_> = std::env::args().collect();

        if args.len() > 1 {
            self.execute(&args[1..].join(" "));
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
            if let Err(e) = rl.load_history(path) {
                if !matches!(e, ReadlineError::Io(ref io_err) if io_err.kind() == std::io::ErrorKind::NotFound)
                {
                    crate::error::RcalError::Cli(format!("Failed to load history: {}", e))
                        .report("");
                }
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
                    self.execute(trimmed);
                }
                Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    crate::error::RcalError::Cli(format!("Readline error: {}", err)).report("");
                    break;
                }
            }
        }

        if let Some(ref path) = h_path {
            let _ = rl.save_history(path);
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

            if t.eq_ignore_ascii_case("list") {
                self.print_list();
                continue;
            }

            match self.calc.eval(t) {
                Ok((v, expr)) => {
                    if !matches!(expr, Expr::Assign(_, _))
                        && !matches!(expr, Expr::FnDefine(_, _, _))
                    {
                        if let Expr::Function(n, _) = expr {
                            if n == "hex" && v.is_scalar() {
                                println!("{}= 0x{:x}{}", GREEN, v.value as u64, RESET);
                            } else if n == "bin" && v.is_scalar() {
                                println!("{}= 0b{:b}{}", GREEN, v.value as u64, RESET);
                            } else {
                                println!("{}= {}{}", GREEN, v, RESET);
                            }
                        } else {
                            println!("{}= {}{}", GREEN, v, RESET);
                        }
                    }
                }
                Err(e) => e.report(t),
            }
        }
    }

    fn print_help(&self) {
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
        println!("  help, list, exit, quit\n");
    }

    fn print_list(&self) {
        let vars = self.calc.vars();
        let funcs = self.calc.funcs();

        if vars.is_empty() && funcs.is_empty() {
            println!("No variables or functions defined.");
            return;
        }

        for (k, v) in vars {
            println!("{}: {}", k, v);
        }
        for (k, f) in funcs {
            println!("{}({}) = {}", k, f.params.join(", "), f.body);
        }
    }
}
