use crate::unit::{
    D_A, D_CD, D_G, D_H, D_HZ, D_J, D_K, D_K_BOLTZ, D_KG, D_M, D_M3, D_MOL, D_MS, D_MS2, D_N, D_NA,
    D_PA, D_RAD, D_S, D_W, Quantity,
};

pub enum Arity {
    Fixed(usize),
    Variadic,
}

pub struct Builtin {
    pub name: &'static str,
    pub arity: Arity,
    pub func: fn(&[Quantity]) -> Result<Quantity, String>,
}

fn trig(args: &[Quantity], f: fn(f64) -> f64) -> Result<Quantity, String> {
    if !args[0].is_angle() && !args[0].is_scalar() {
        return Err("Expected angle or scalar".into());
    }
    Ok(Quantity::scalar(f(args[0].value)))
}

fn inv_trig(args: &[Quantity], f: fn(f64) -> f64) -> Result<Quantity, String> {
    if !args[0].is_scalar() {
        return Err("Expected scalar".into());
    }
    Ok(Quantity {
        value: f(args[0].value),
        dims: D_RAD,
    })
}

fn scalar_op(args: &[Quantity], f: fn(f64) -> f64) -> Result<Quantity, String> {
    if !args[0].is_scalar() {
        return Err("Expected scalar".into());
    }
    Ok(Quantity::scalar(f(args[0].value)))
}

fn aggregate(args: &[Quantity], f: fn(&[f64]) -> f64) -> Result<Quantity, String> {
    if args.is_empty() {
        return Ok(Quantity::scalar(0.0));
    }
    let dims = args[0].dims;
    let mut vals = Vec::with_capacity(args.len());
    for a in args {
        if a.dims != dims {
            return Err("Dimension mismatch".into());
        }
        vals.push(a.value);
    }
    Ok(Quantity {
        value: f(&vals),
        dims,
    })
}

pub const BUILTINS: &[Builtin] = &[
    Builtin {
        name: "sin",
        arity: Arity::Fixed(1),
        func: |a| trig(a, f64::sin),
    },
    Builtin {
        name: "cos",
        arity: Arity::Fixed(1),
        func: |a| trig(a, f64::cos),
    },
    Builtin {
        name: "tan",
        arity: Arity::Fixed(1),
        func: |a| trig(a, f64::tan),
    },
    Builtin {
        name: "asin",
        arity: Arity::Fixed(1),
        func: |a| inv_trig(a, f64::asin),
    },
    Builtin {
        name: "acos",
        arity: Arity::Fixed(1),
        func: |a| inv_trig(a, f64::acos),
    },
    Builtin {
        name: "atan",
        arity: Arity::Fixed(1),
        func: |a| inv_trig(a, f64::atan),
    },
    Builtin {
        name: "abs",
        arity: Arity::Fixed(1),
        func: |a| {
            Ok(Quantity {
                value: a[0].value.abs(),
                dims: a[0].dims,
            })
        },
    },
    Builtin {
        name: "sqrt",
        arity: Arity::Fixed(1),
        func: |args| {
            if args[0].value < 0.0 {
                return Err("Sqrt of negative".into());
            }
            let mut dims = [0i8; 8];
            for (i, dim) in dims.iter_mut().enumerate() {
                if args[0].dims[i] % 2 != 0 {
                    return Err("Cannot take sqrt of this unit".into());
                }
                *dim = args[0].dims[i] / 2;
            }
            Ok(Quantity {
                value: args[0].value.sqrt(),
                dims,
            })
        },
    },
    Builtin {
        name: "ln",
        arity: Arity::Fixed(1),
        func: |a| scalar_op(a, f64::ln),
    },
    Builtin {
        name: "log",
        arity: Arity::Fixed(1),
        func: |a| scalar_op(a, f64::log10),
    },
    Builtin {
        name: "not",
        arity: Arity::Fixed(1),
        func: |a| scalar_op(a, |v| !(v as u64) as f64),
    },
    Builtin {
        name: "hex",
        arity: Arity::Fixed(1),
        func: |a| {
            if !a[0].is_scalar() || a[0].value < 0.0 || a[0].value.fract() != 0.0 {
                return Err("Invalid for hex".into());
            }
            Ok(a[0])
        },
    },
    Builtin {
        name: "bin",
        arity: Arity::Fixed(1),
        func: |a| {
            if !a[0].is_scalar() || a[0].value < 0.0 || a[0].value.fract() != 0.0 {
                return Err("Invalid for bin".into());
            }
            Ok(a[0])
        },
    },
    Builtin {
        name: "and",
        arity: Arity::Fixed(2),
        func: |a| {
            if !a[0].is_scalar() || !a[1].is_scalar() {
                return Err("Expected scalars".into());
            }
            Ok(Quantity::scalar(
                ((a[0].value as u64) & (a[1].value as u64)) as f64,
            ))
        },
    },
    Builtin {
        name: "or",
        arity: Arity::Fixed(2),
        func: |a| {
            if !a[0].is_scalar() || !a[1].is_scalar() {
                return Err("Expected scalars".into());
            }
            Ok(Quantity::scalar(
                ((a[0].value as u64) | (a[1].value as u64)) as f64,
            ))
        },
    },
    Builtin {
        name: "xor",
        arity: Arity::Fixed(2),
        func: |a| {
            if !a[0].is_scalar() || !a[1].is_scalar() {
                return Err("Expected scalars".into());
            }
            Ok(Quantity::scalar(
                ((a[0].value as u64) ^ (a[1].value as u64)) as f64,
            ))
        },
    },
    Builtin {
        name: "lshift",
        arity: Arity::Fixed(2),
        func: |a| {
            if !a[0].is_scalar() || !a[1].is_scalar() {
                return Err("Expected scalars".into());
            }
            Ok(Quantity::scalar(
                ((a[0].value as u64) << (a[1].value as u64)) as f64,
            ))
        },
    },
    Builtin {
        name: "rshift",
        arity: Arity::Fixed(2),
        func: |a| {
            if !a[0].is_scalar() || !a[1].is_scalar() {
                return Err("Expected scalars".into());
            }
            Ok(Quantity::scalar(
                ((a[0].value as u64) >> (a[1].value as u64)) as f64,
            ))
        },
    },
    Builtin {
        name: "round",
        arity: Arity::Fixed(2),
        func: |a| {
            if !a[1].is_scalar() {
                return Err("Precision must be scalar".into());
            }
            let m = 10.0f64.powf(a[1].value.round());
            Ok(Quantity {
                value: (a[0].value * m).round() / m,
                dims: a[0].dims,
            })
        },
    },
    Builtin {
        name: "floor",
        arity: Arity::Fixed(1),
        func: |a| {
            Ok(Quantity {
                value: a[0].value.floor(),
                dims: a[0].dims,
            })
        },
    },
    Builtin {
        name: "ceil",
        arity: Arity::Fixed(1),
        func: |a| {
            Ok(Quantity {
                value: a[0].value.ceil(),
                dims: a[0].dims,
            })
        },
    },
    Builtin {
        name: "exp",
        arity: Arity::Fixed(1),
        func: |a| scalar_op(a, f64::exp),
    },
    Builtin {
        name: "clamp",
        arity: Arity::Fixed(3),
        func: |a| {
            if a[0].dims != a[1].dims || a[0].dims != a[2].dims {
                return Err("Dimension mismatch".into());
            }
            Ok(Quantity {
                value: a[0].value.clamp(a[1].value, a[2].value),
                dims: a[0].dims,
            })
        },
    },
    Builtin {
        name: "max",
        arity: Arity::Variadic,
        func: |a| aggregate(a, |v| v.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
    },
    Builtin {
        name: "min",
        arity: Arity::Variadic,
        func: |a| aggregate(a, |v| v.iter().cloned().fold(f64::INFINITY, f64::min)),
    },
    Builtin {
        name: "sum",
        arity: Arity::Variadic,
        func: |a| aggregate(a, |v| v.iter().sum()),
    },
    Builtin {
        name: "avg",
        arity: Arity::Variadic,
        func: |a| aggregate(a, |v| v.iter().sum::<f64>() / v.len() as f64),
    },
];

pub const CONSTANTS: &[(&str, Quantity)] = &[
    (
        "pi",
        Quantity {
            value: std::f64::consts::PI,
            dims: [0; 8],
        },
    ),
    (
        "e",
        Quantity {
            value: std::f64::consts::E,
            dims: [0; 8],
        },
    ),
    (
        "c",
        Quantity {
            value: 299_792_458.0,
            dims: D_MS,
        },
    ),
    (
        "G",
        Quantity {
            value: 6.674_30e-11,
            dims: D_G,
        },
    ),
    (
        "planck",
        Quantity {
            value: 6.626_070_15e-34,
            dims: D_H,
        },
    ),
    (
        "k_b",
        Quantity {
            value: 1.380_649e-23,
            dims: D_K_BOLTZ,
        },
    ),
    (
        "Na",
        Quantity {
            value: 6.022_140_76e23,
            dims: D_NA,
        },
    ),
    (
        "g0",
        Quantity {
            value: 9.806_65,
            dims: D_MS2,
        },
    ),
];

pub const UNITS: &[(&str, Quantity)] = &[
    (
        "rad",
        Quantity {
            value: 1.0,
            dims: D_RAD,
        },
    ),
    (
        "deg",
        Quantity {
            value: std::f64::consts::PI / 180.0,
            dims: D_RAD,
        },
    ),
    (
        "m",
        Quantity {
            value: 1.0,
            dims: D_M,
        },
    ),
    (
        "cm",
        Quantity {
            value: 0.01,
            dims: D_M,
        },
    ),
    (
        "mm",
        Quantity {
            value: 0.001,
            dims: D_M,
        },
    ),
    (
        "um",
        Quantity {
            value: 1e-6,
            dims: D_M,
        },
    ),
    (
        "nm",
        Quantity {
            value: 1e-9,
            dims: D_M,
        },
    ),
    (
        "km",
        Quantity {
            value: 1000.0,
            dims: D_M,
        },
    ),
    (
        "kg",
        Quantity {
            value: 1.0,
            dims: D_KG,
        },
    ),
    (
        "g",
        Quantity {
            value: 0.001,
            dims: D_KG,
        },
    ),
    (
        "s",
        Quantity {
            value: 1.0,
            dims: D_S,
        },
    ),
    (
        "ms",
        Quantity {
            value: 0.001,
            dims: D_S,
        },
    ),
    (
        "us",
        Quantity {
            value: 1e-6,
            dims: D_S,
        },
    ),
    (
        "ns",
        Quantity {
            value: 1e-9,
            dims: D_S,
        },
    ),
    (
        "min",
        Quantity {
            value: 60.0,
            dims: D_S,
        },
    ),
    (
        "h",
        Quantity {
            value: 3600.0,
            dims: D_S,
        },
    ),
    (
        "Wh",
        Quantity {
            value: 3600.0,
            dims: D_J,
        },
    ),
    (
        "kWh",
        Quantity {
            value: 3_600_000.0,
            dims: D_J,
        },
    ),
    (
        "kmh",
        Quantity {
            value: 1000.0 / 3600.0,
            dims: D_MS,
        },
    ),
    (
        "A",
        Quantity {
            value: 1.0,
            dims: D_A,
        },
    ),
    (
        "K",
        Quantity {
            value: 1.0,
            dims: D_K,
        },
    ),
    (
        "mol",
        Quantity {
            value: 1.0,
            dims: D_MOL,
        },
    ),
    (
        "cd",
        Quantity {
            value: 1.0,
            dims: D_CD,
        },
    ),
    (
        "l",
        Quantity {
            value: 0.001,
            dims: D_M3,
        },
    ),
    (
        "bar",
        Quantity {
            value: 100_000.0,
            dims: D_PA,
        },
    ),
    (
        "atm",
        Quantity {
            value: 101_325.0,
            dims: D_PA,
        },
    ),
    (
        "inch",
        Quantity {
            value: 0.0254,
            dims: D_M,
        },
    ),
    (
        "ft",
        Quantity {
            value: 0.3048,
            dims: D_M,
        },
    ),
    (
        "eV",
        Quantity {
            value: 1.602_176_634e-19,
            dims: D_J,
        },
    ),
    (
        "N",
        Quantity {
            value: 1.0,
            dims: D_N,
        },
    ),
    (
        "J",
        Quantity {
            value: 1.0,
            dims: D_J,
        },
    ),
    (
        "W",
        Quantity {
            value: 1.0,
            dims: D_W,
        },
    ),
    (
        "Pa",
        Quantity {
            value: 1.0,
            dims: D_PA,
        },
    ),
    (
        "Hz",
        Quantity {
            value: 1.0,
            dims: D_HZ,
        },
    ),
];

pub fn is_protected(name: &str) -> bool {
    CONSTANTS.iter().any(|(n, _)| *n == name)
        || UNITS.iter().any(|(n, _)| *n == name)
        || BUILTINS.iter().any(|b| b.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::D_M2;

    #[test]
    fn test_trig_scalar() {
        let sin_f = BUILTINS.iter().find(|b| b.name == "sin").unwrap();
        let res = (sin_f.func)(&[Quantity::scalar(0.0)]).unwrap();
        assert_eq!(res.value, 0.0);
        assert!(res.is_scalar());
    }

    #[test]
    fn test_inv_trig_returns_angle() {
        let asin_f = BUILTINS.iter().find(|b| b.name == "asin").unwrap();
        let res = (asin_f.func)(&[Quantity::scalar(0.0)]).unwrap();
        assert!(res.is_angle());
    }

    #[test]
    fn test_sqrt_dims() {
        let sqrt_f = BUILTINS.iter().find(|b| b.name == "sqrt").unwrap();
        let m2 = Quantity {
            value: 16.0,
            dims: D_M2,
        };
        let res = (sqrt_f.func)(&[m2]).unwrap();
        assert_eq!(res.value, 4.0);
        assert_eq!(res.dims, D_M);
    }

    #[test]
    fn test_aggregate_dims() {
        let sum_f = BUILTINS.iter().find(|b| b.name == "sum").unwrap();
        let m = Quantity {
            value: 1.0,
            dims: D_M,
        };
        let kg = Quantity {
            value: 1.0,
            dims: D_KG,
        };

        assert!((sum_f.func)(&[m, m]).is_ok());
        assert!((sum_f.func)(&[m, kg]).is_err());
    }

    #[test]
    fn test_round() {
        let round_f = BUILTINS.iter().find(|b| b.name == "round").unwrap();
        let val = Quantity::scalar(1.23456);
        let prec = Quantity::scalar(2.0);
        let res = (round_f.func)(&[val, prec]).unwrap();
        assert_eq!(res.value, 1.23);
    }

    #[test]
    fn test_protection() {
        assert!(is_protected("pi"));
        assert!(is_protected("sin"));
        assert!(is_protected("m"));
        assert!(!is_protected("my_var"));
    }
}
