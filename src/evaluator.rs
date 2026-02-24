use crate::ast::{BinOp, Expr, Node, UnOp};
use std::collections::HashMap;

pub fn evaluate(node: &Node, vars: &mut HashMap<String, f64>) -> Result<f64, (String, usize)> {
    let pos = node.pos;
    let check = |res: f64| {
        if res.is_infinite() {
            Err(("Math Error: Overflow".to_string(), pos))
        } else {
            Ok(res)
        }
    };
    match &node.expr {
        Expr::Number(n) => Ok(*n),
        Expr::Variable(name) => match name.as_str() {
            "pi" => Ok(std::f64::consts::PI),
            "e" => Ok(std::f64::consts::E),
            "deg" => Ok(std::f64::consts::PI / 180.0),
            _ => vars
                .get(name)
                .copied()
                .ok_or_else(|| (format!("Math Error: Unknown variable '{}'", name), pos)),
        },
        Expr::Assign(name, e) => {
            let v = evaluate(e, vars)?;
            vars.insert(name.clone(), v);
            Ok(v)
        }
        Expr::Function(name, args) => {
            let vs = args
                .iter()
                .map(|a| evaluate(a, vars))
                .collect::<Result<Vec<_>, _>>()?;
            match name.as_str() {
                "sin" | "cos" | "tan" | "asin" | "acos" | "atan" | "abs" | "sqrt" | "ln"
                | "log" | "not" | "hex" | "bin"
                    if vs.len() == 1 =>
                {
                    let v = vs[0];
                    match name.as_str() {
                        "sin" => Ok(v.sin()),
                        "cos" => Ok(v.cos()),
                        "tan" => Ok(v.tan()),
                        "asin" => Ok(v.asin()),
                        "acos" => Ok(v.acos()),
                        "atan" => Ok(v.atan()),
                        "abs" => Ok(v.abs()),
                        "sqrt" => {
                            if v < 0.0 {
                                Err(("Math Error: Sqrt of negative".to_string(), pos))
                            } else {
                                Ok(v.sqrt())
                            }
                        }
                        "ln" | "log" => {
                            if v <= 0.0 {
                                Err(("Math Error: Log of non-positive".to_string(), pos))
                            } else {
                                if name == "ln" {
                                    Ok(v.ln())
                                } else {
                                    Ok(v.log10())
                                }
                            }
                        }
                        "not" => Ok(!(v as u64) as f64),
                        "hex" | "bin" => {
                            if v < 0.0 || v > u64::MAX as f64 || v.fract() != 0.0 {
                                Err(("Math Error: Invalid for hex/bin".to_string(), pos))
                            } else {
                                Ok(v)
                            }
                        }
                        _ => unreachable!(),
                    }
                }
                "and" | "or" | "xor" | "lshift" | "rshift" | "round" if vs.len() == 2 => {
                    let (a, b) = (vs[0], vs[1]);
                    match name.as_str() {
                        "and" => Ok(((a as u64) & (b as u64)) as f64),
                        "or" => Ok(((a as u64) | (b as u64)) as f64),
                        "xor" => Ok(((a as u64) ^ (b as u64)) as f64),
                        "lshift" => Ok(((a as u64) << (b as u64)) as f64),
                        "rshift" => Ok(((a as u64) >> (b as u64)) as f64),
                        "round" => {
                            let m = 10.0f64.powf(b.round());
                            Ok((a * m).round() / m)
                        }
                        _ => unreachable!(),
                    }
                }
                "max" | "min" | "sum" | "avg" if !vs.is_empty() => match name.as_str() {
                    "max" => Ok(vs.iter().cloned().fold(f64::NEG_INFINITY, f64::max)),
                    "min" => Ok(vs.iter().cloned().fold(f64::INFINITY, f64::min)),
                    "sum" => Ok(vs.iter().sum()),
                    "avg" => Ok(vs.iter().sum::<f64>() / vs.len() as f64),
                    _ => unreachable!(),
                },
                _ => Err((
                    format!("Math Error: Unknown function or wrong args for '{}'", name),
                    pos,
                )),
            }
        }
        Expr::Binary(op, l, r) => {
            let (lv, rv) = (evaluate(l, vars)?, evaluate(r, vars)?);
            match op {
                BinOp::Add => check(lv + rv),
                BinOp::Sub => check(lv - rv),
                BinOp::Mul => check(lv * rv),
                BinOp::Div => {
                    if rv == 0.0 {
                        Err(("Math Error: Division by zero".to_string(), pos))
                    } else {
                        check(lv / rv)
                    }
                }
                BinOp::Mod => {
                    if rv == 0.0 {
                        Err(("Math Error: Modulo by zero".to_string(), pos))
                    } else {
                        Ok(lv % rv)
                    }
                }
                BinOp::Pow => {
                    let res = lv.powf(rv);
                    if res.is_infinite() {
                        check(res)
                    } else if res.is_nan() {
                        Err(("Math Error: Invalid power".to_string(), pos))
                    } else {
                        Ok(res)
                    }
                }
            }
        }
        Expr::Factorial(e) => {
            let v = evaluate(e, vars)?;
            if v < 0.0 || v.fract() != 0.0 {
                return Err(("Math Error: Needs non-neg int".to_string(), pos));
            }
            if v > 170.0 {
                return Err(("Math Error: Factorial overflow".to_string(), pos));
            }
            let mut r = 1.0;
            for i in 1..=(v as u64) {
                r *= i as f64;
            }
            Ok(r)
        }
        Expr::Unary(op, e) => {
            let v = evaluate(e, vars)?;
            Ok(if let UnOp::Neg = op { -v } else { v })
        }
    }
}
