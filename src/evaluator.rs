use crate::{
    ast::{BinOp, Expr, Node, UnOp},
    builtins::{Arity, BUILTINS, CONSTANTS, UNITS, is_protected},
    calculator::UserFunction,
    error::RcalError,
    unit::Quantity,
};
use std::collections::HashMap;

pub fn evaluate(
    node: &Node,
    vars: &mut HashMap<String, Quantity>,
    funcs: &mut HashMap<String, UserFunction>,
) -> Result<Quantity, RcalError> {
    let pos = node.pos;
    match &node.expr {
        Expr::Number(n) => Ok(Quantity::scalar(*n)),
        Expr::Variable(name) => {
            if let Some((_, val)) = CONSTANTS.iter().find(|(n, _)| *n == name) {
                return Ok(*val);
            }
            if let Some((_, val)) = UNITS.iter().find(|(n, _)| *n == name) {
                return Ok(*val);
            }
            vars.get(name)
                .copied()
                .ok_or_else(|| RcalError::Math(format!("Unknown variable '{}'", name), pos))
        }
        Expr::Assign(name, e) => {
            if is_protected(name) {
                return Err(RcalError::Math(
                    format!("'{}' is a protected name", name),
                    pos,
                ));
            }
            let v = evaluate(e, vars, funcs)?;
            vars.insert(name.clone(), v);
            Ok(v)
        }
        Expr::FnDefine(name, params, body) => {
            if is_protected(name) {
                return Err(RcalError::Math(
                    format!("'{}' is a protected name", name),
                    pos,
                ));
            }
            funcs.insert(
                name.clone(),
                UserFunction {
                    params: params.clone(),
                    body: body.clone(),
                },
            );
            Ok(Quantity::scalar(0.0))
        }
        Expr::Function(name, args) => {
            let vs = args
                .iter()
                .map(|a| evaluate(a, vars, funcs))
                .collect::<Result<Vec<_>, _>>()?;

            if let Some(f) = funcs.get(name) {
                if vs.len() != f.params.len() {
                    return Err(RcalError::Math(
                        format!("Expected {} args, got {}", f.params.len(), vs.len()),
                        pos,
                    ));
                }
                let mut scope_vars = vars.clone();
                for (p, v) in f.params.iter().zip(vs) {
                    scope_vars.insert(p.clone(), v);
                }
                let body = f.body.clone();
                return evaluate(&body, &mut scope_vars, funcs);
            }

            if let Some(b) = BUILTINS.iter().find(|b| b.name == name) {
                match b.arity {
                    Arity::Fixed(n) if vs.len() != n => {
                        return Err(RcalError::Math(
                            format!("Expected {} args, got {}", n, vs.len()),
                            pos,
                        ));
                    }
                    _ => {}
                }
                return (b.func)(&vs).map_err(|e| RcalError::Math(e, pos));
            }

            Err(RcalError::Math(format!("Unknown function '{}'", name), pos))
        }
        Expr::Binary(op, l, r) => {
            let (lv, rv) = (evaluate(l, vars, funcs)?, evaluate(r, vars, funcs)?);
            match op {
                BinOp::Add => (lv + rv).map_err(|e| RcalError::Math(e, pos)),
                BinOp::Sub => (lv - rv).map_err(|e| RcalError::Math(e, pos)),
                BinOp::Mul => Ok(lv * rv),
                BinOp::Div => {
                    if rv.value == 0.0 {
                        Err(RcalError::Math("Division by zero".to_string(), pos))
                    } else {
                        Ok(lv / rv)
                    }
                }
                BinOp::Mod => {
                    if !rv.is_scalar() || !lv.is_scalar() {
                        return Err(RcalError::Math("Modulo requires scalars".into(), pos));
                    }
                    if rv.value == 0.0 {
                        Err(RcalError::Math("Modulo by zero".to_string(), pos))
                    } else {
                        Ok(Quantity::scalar(lv.value % rv.value))
                    }
                }
                BinOp::Pow => {
                    if !rv.is_scalar() {
                        return Err(RcalError::Math("Exponent must be scalar".into(), pos));
                    }
                    lv.pow(rv.value).map_err(|e| RcalError::Math(e, pos))
                }
            }
        }
        Expr::Factorial(e) => {
            let v = evaluate(e, vars, funcs)?;
            if !v.is_scalar() {
                return Err(RcalError::Math("Factorial requires scalar".into(), pos));
            }
            if v.value < 0.0 || v.value.fract() != 0.0 {
                return Err(RcalError::Math("Needs non-neg int".to_string(), pos));
            }
            if v.value > 170.0 {
                return Err(RcalError::Math("Factorial overflow".to_string(), pos));
            }
            let mut r = 1.0;
            for i in 1..=(v.value as u64) {
                r *= i as f64;
            }
            Ok(Quantity::scalar(r))
        }
        Expr::Unary(op, e) => {
            let v = evaluate(e, vars, funcs)?;
            Ok(if let UnOp::Neg = op { -v } else { v })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::Parser;

    fn eval_str(
        input: &str,
        vars: &mut HashMap<String, Quantity>,
        funcs: &mut HashMap<String, UserFunction>,
    ) -> Result<Quantity, RcalError> {
        let toks = tokenize(input).unwrap();
        let mut p = Parser::new(toks);
        let ast = p.parse_expr().unwrap();
        evaluate(&ast, vars, funcs)
    }

    #[test]
    fn test_eval_simple() {
        let mut vars = HashMap::new();
        let mut funcs = HashMap::new();
        let res = eval_str("1 + 2 * 3", &mut vars, &mut funcs).unwrap();
        assert_eq!(res.value, 7.0);
    }

    #[test]
    fn test_eval_vars() {
        let mut vars = HashMap::new();
        let mut funcs = HashMap::new();
        eval_str("a = 5", &mut vars, &mut funcs).unwrap();
        let res = eval_str("a * 2", &mut vars, &mut funcs).unwrap();
        assert_eq!(res.value, 10.0);
    }

    #[test]
    fn test_eval_funcs() {
        let mut vars = HashMap::new();
        let mut funcs = HashMap::new();
        eval_str("f(x) = x * x", &mut vars, &mut funcs).unwrap();
        let res = eval_str("f(3) + 1", &mut vars, &mut funcs).unwrap();
        assert_eq!(res.value, 10.0);
    }

    #[test]
    fn test_units() {
        let mut vars = HashMap::new();
        let mut funcs = HashMap::new();
        let res = eval_str("10m / 2s", &mut vars, &mut funcs).unwrap();
        assert_eq!(res.value, 5.0);
        assert_eq!(res.dims[0], 1); // m
        assert_eq!(res.dims[2], -1); // s^-1
    }
}
