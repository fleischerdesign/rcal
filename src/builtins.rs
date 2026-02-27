//! Built-in functions, constants, and formatting logic.

use crate::unit::{
    ACCELERATION, ACTION, ANGLE, AVOGADRO_CONSTANT, BOLTZMANN_CONSTANT, GRAVITATIONAL_CONSTANT,
    Quantity, UNITS, VELOCITY, Dimensions,
};

/// Arity of a function (number of expected arguments).
pub enum Arity {
    /// Fixed number of arguments.
    Fixed(usize),
    /// Variable number of arguments.
    Variadic,
}

/// Definition of a built-in function.
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
        dims: ANGLE,
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

macro_rules! builtin {
    ($name:expr, $arity:expr, $func:expr) => {
        Builtin {
            name: $name,
            arity: $arity,
            func: $func,
        }
    };
}

/// List of all built-in mathematical and bitwise functions.
pub const BUILTINS: &[Builtin] = &[
    builtin!("sin", Arity::Fixed(1), |a| trig(a, f64::sin)),
    builtin!("cos", Arity::Fixed(1), |a| trig(a, f64::cos)),
    builtin!("tan", Arity::Fixed(1), |a| trig(a, f64::tan)),
    builtin!("asin", Arity::Fixed(1), |a| inv_trig(a, f64::asin)),
    builtin!("acos", Arity::Fixed(1), |a| inv_trig(a, f64::acos)),
    builtin!("atan", Arity::Fixed(1), |a| inv_trig(a, f64::atan)),
    builtin!("abs", Arity::Fixed(1), |a| {
        Ok(Quantity {
            value: a[0].value.abs(),
            dims: a[0].dims,
        })
    }),
    builtin!("sqrt", Arity::Fixed(1), |args| {
        if args[0].value < 0.0 {
            return Err("Sqrt of negative".into());
        }
        let d = args[0].dims;
        if d.length % 2 != 0 || d.mass % 2 != 0 || d.time % 2 != 0 || d.current % 2 != 0 
           || d.temperature % 2 != 0 || d.amount % 2 != 0 || d.intensity % 2 != 0 || d.angle % 2 != 0 {
            return Err("Cannot take sqrt of this unit".into());
        }
        let dims = Dimensions {
            length: d.length / 2,
            mass: d.mass / 2,
            time: d.time / 2,
            current: d.current / 2,
            temperature: d.temperature / 2,
            amount: d.amount / 2,
            intensity: d.intensity / 2,
            angle: d.angle / 2,
        };
        Ok(Quantity {
            value: args[0].value.sqrt(),
            dims,
        })
    }),
    builtin!("ln", Arity::Fixed(1), |a| scalar_op(a, f64::ln)),
    builtin!("log", Arity::Fixed(1), |a| scalar_op(a, f64::log10)),
    builtin!("not", Arity::Fixed(1), |a| scalar_op(a, |v| !(v as u64) as f64)),
    builtin!("and", Arity::Fixed(2), |a| {
        if !a[0].is_scalar() || !a[1].is_scalar() {
            return Err("Expected scalars".into());
        }
        Ok(Quantity::scalar(((a[0].value as u64) & (a[1].value as u64)) as f64))
    }),
    builtin!("or", Arity::Fixed(2), |a| {
        if !a[0].is_scalar() || !a[1].is_scalar() {
            return Err("Expected scalars".into());
        }
        Ok(Quantity::scalar(((a[0].value as u64) | (a[1].value as u64)) as f64))
    }),
    builtin!("xor", Arity::Fixed(2), |a| {
        if !a[0].is_scalar() || !a[1].is_scalar() {
            return Err("Expected scalars".into());
        }
        Ok(Quantity::scalar(((a[0].value as u64) ^ (a[1].value as u64)) as f64))
    }),
    builtin!("lshift", Arity::Fixed(2), |a| {
        if !a[0].is_scalar() || !a[1].is_scalar() {
            return Err("Expected scalars".into());
        }
        Ok(Quantity::scalar(((a[0].value as u64) << (a[1].value as u64)) as f64))
    }),
    builtin!("rshift", Arity::Fixed(2), |a| {
        if !a[0].is_scalar() || !a[1].is_scalar() {
            return Err("Expected scalars".into());
        }
        Ok(Quantity::scalar(((a[0].value as u64) >> (a[1].value as u64)) as f64))
    }),
    builtin!("round", Arity::Fixed(2), |a| {
        if !a[1].is_scalar() {
            return Err("Precision must be scalar".into());
        }
        let m = 10.0f64.powf(a[1].value.round());
        Ok(Quantity {
            value: (a[0].value * m).round() / m,
            dims: a[0].dims,
        })
    }),
    builtin!("floor", Arity::Fixed(1), |a| {
        Ok(Quantity {
            value: a[0].value.floor(),
            dims: a[0].dims,
        })
    }),
    builtin!("ceil", Arity::Fixed(1), |a| {
        Ok(Quantity {
            value: a[0].value.ceil(),
            dims: a[0].dims,
        })
    }),
    builtin!("exp", Arity::Fixed(1), |a| scalar_op(a, f64::exp)),
    builtin!("clamp", Arity::Fixed(3), |a| {
        if a[0].dims != a[1].dims || a[0].dims != a[2].dims {
            return Err("Dimension mismatch".into());
        }
        Ok(Quantity {
            value: a[0].value.clamp(a[1].value, a[2].value),
            dims: a[0].dims,
        })
    }),
    builtin!("max", Arity::Variadic, |a| aggregate(a, |v| v.iter().cloned().fold(f64::NEG_INFINITY, f64::max))),
    builtin!("min", Arity::Variadic, |a| aggregate(a, |v| v.iter().cloned().fold(f64::INFINITY, f64::min))),
    builtin!("sum", Arity::Variadic, |a| aggregate(a, |v| v.iter().sum())),
    builtin!("avg", Arity::Variadic, |a| aggregate(a, |v| v.iter().sum::<f64>() / v.len() as f64)),
];

/// Physical constants.
pub const CONSTANTS: &[(&str, Quantity)] = &[
    ("pi", Quantity { value: std::f64::consts::PI, dims: Dimensions::SCALAR }),
    ("e", Quantity { value: std::f64::consts::E, dims: Dimensions::SCALAR }),
    ("c", Quantity { value: 299_792_458.0, dims: VELOCITY }),
    ("G", Quantity { value: 6.674_30e-11, dims: GRAVITATIONAL_CONSTANT }),
    ("planck", Quantity { value: 6.626_070_15e-34, dims: ACTION }),
    ("k_b", Quantity { value: 1.380_649e-23, dims: BOLTZMANN_CONSTANT }),
    ("Na", Quantity { value: 6.022_140_76e23, dims: AVOGADRO_CONSTANT }),
    ("g0", Quantity { value: 9.806_65, dims: ACCELERATION }),
];

/// Special formatting keywords.
pub const FORMATTERS: &[&str] = &["hex", "bin"];

/// Resolves a name to a static quantity (constant, unit, or formatter).
pub fn resolve_static_var(name: &str) -> Option<Quantity> {
    if FORMATTERS.contains(&name) {
        return Some(Quantity::scalar(1.0));
    }
    if let Some((_, val)) = CONSTANTS.iter().find(|(n, _)| *n == name) {
        return Some(*val);
    }
    if let Some(u) = UNITS.iter().find(|u| u.name == name) {
        return Some(u.quantity);
    }
    None
}

/// Returns true if a name is protected and cannot be assigned to.
pub fn is_protected(name: &str) -> bool {
    resolve_static_var(name).is_some() || BUILTINS.iter().any(|b| b.name == name)
}

/// Formats a numeric value based on a formatter keyword.
pub fn format_as(name: &str, value: f64) -> Option<String> {
    if !FORMATTERS.contains(&name) {
        return None;
    }
    match name {
        "hex" => Some(format!("0x{:x}", value as u64)),
        "bin" => Some(format!("0b{:b}", value as u64)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::{AREA, LENGTH, MASS};

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
            dims: AREA,
        };
        let res = (sqrt_f.func)(&[m2]).unwrap();
        assert_eq!(res.value, 4.0);
        assert_eq!(res.dims, LENGTH);
    }

    #[test]
    fn test_aggregate_dims() {
        let sum_f = BUILTINS.iter().find(|b| b.name == "sum").unwrap();
        let m = Quantity {
            value: 1.0,
            dims: LENGTH,
        };
        let kg = Quantity {
            value: 1.0,
            dims: MASS,
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
