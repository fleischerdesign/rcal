pub enum Arity {
    Fixed(usize),
    Variadic,
}

pub struct Builtin {
    pub name: &'static str,
    pub arity: Arity,
    pub func: fn(&[f64]) -> Result<f64, String>,
}

pub const BUILTINS: &[Builtin] = &[
    Builtin {
        name: "sin",
        arity: Arity::Fixed(1),
        func: |args| Ok(args[0].sin()),
    },
    Builtin {
        name: "cos",
        arity: Arity::Fixed(1),
        func: |args| Ok(args[0].cos()),
    },
    Builtin {
        name: "tan",
        arity: Arity::Fixed(1),
        func: |args| Ok(args[0].tan()),
    },
    Builtin {
        name: "asin",
        arity: Arity::Fixed(1),
        func: |args| Ok(args[0].asin()),
    },
    Builtin {
        name: "acos",
        arity: Arity::Fixed(1),
        func: |args| Ok(args[0].acos()),
    },
    Builtin {
        name: "atan",
        arity: Arity::Fixed(1),
        func: |args| Ok(args[0].atan()),
    },
    Builtin {
        name: "abs",
        arity: Arity::Fixed(1),
        func: |args| Ok(args[0].abs()),
    },
    Builtin {
        name: "sqrt",
        arity: Arity::Fixed(1),
        func: |args| {
            if args[0] < 0.0 {
                Err("Sqrt of negative".into())
            } else {
                Ok(args[0].sqrt())
            }
        },
    },
    Builtin {
        name: "ln",
        arity: Arity::Fixed(1),
        func: |args| {
            if args[0] <= 0.0 {
                Err("Log of non-positive".into())
            } else {
                Ok(args[0].ln())
            }
        },
    },
    Builtin {
        name: "log",
        arity: Arity::Fixed(1),
        func: |args| {
            if args[0] <= 0.0 {
                Err("Log of non-positive".into())
            } else {
                Ok(args[0].log10())
            }
        },
    },
    Builtin {
        name: "not",
        arity: Arity::Fixed(1),
        func: |args| Ok(!(args[0] as u64) as f64),
    },
    Builtin {
        name: "hex",
        arity: Arity::Fixed(1),
        func: |args| {
            let v = args[0];
            if v < 0.0 || v > u64::MAX as f64 || v.fract() != 0.0 {
                Err("Invalid for hex".into())
            } else {
                Ok(v)
            }
        },
    },
    Builtin {
        name: "bin",
        arity: Arity::Fixed(1),
        func: |args| {
            let v = args[0];
            if v < 0.0 || v > u64::MAX as f64 || v.fract() != 0.0 {
                Err("Invalid for bin".into())
            } else {
                Ok(v)
            }
        },
    },
    Builtin {
        name: "and",
        arity: Arity::Fixed(2),
        func: |args| Ok(((args[0] as u64) & (args[1] as u64)) as f64),
    },
    Builtin {
        name: "or",
        arity: Arity::Fixed(2),
        func: |args| Ok(((args[0] as u64) | (args[1] as u64)) as f64),
    },
    Builtin {
        name: "xor",
        arity: Arity::Fixed(2),
        func: |args| Ok(((args[0] as u64) ^ (args[1] as u64)) as f64),
    },
    Builtin {
        name: "lshift",
        arity: Arity::Fixed(2),
        func: |args| Ok(((args[0] as u64) << (args[1] as u64)) as f64),
    },
    Builtin {
        name: "rshift",
        arity: Arity::Fixed(2),
        func: |args| Ok(((args[0] as u64) >> (args[1] as u64)) as f64),
    },
    Builtin {
        name: "round",
        arity: Arity::Fixed(2),
        func: |args| {
            let m = 10.0f64.powf(args[1].round());
            Ok((args[0] * m).round() / m)
        },
    },
    Builtin {
        name: "max",
        arity: Arity::Variadic,
        func: |args| Ok(args.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
    },
    Builtin {
        name: "min",
        arity: Arity::Variadic,
        func: |args| Ok(args.iter().cloned().fold(f64::INFINITY, f64::min)),
    },
    Builtin {
        name: "sum",
        arity: Arity::Variadic,
        func: |args| Ok(args.iter().sum()),
    },
    Builtin {
        name: "avg",
        arity: Arity::Variadic,
        func: |args| {
            if args.is_empty() {
                Ok(0.0)
            } else {
                Ok(args.iter().sum::<f64>() / args.len() as f64)
            }
        },
    },
];

pub const CONSTANTS: &[(&str, f64)] = &[
    ("pi", std::f64::consts::PI),
    ("e", std::f64::consts::E),
    ("deg", std::f64::consts::PI / 180.0),
];
