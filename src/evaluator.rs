//! AST evaluation and scope management.

use crate::{
    ast::{BinOp, Expr, Node, UnOp},
    builtins::{Arity, BUILTINS, is_protected, resolve_static_var},
    calculator::UserFunction,
    error::{MathError, Error},
    unit::{Quantity, UNITS},
};
use std::collections::HashMap;

/// A hierarchical execution scope for variables and functions.
pub struct Scope<'a> {
    vars: HashMap<String, Quantity>,
    funcs: HashMap<String, UserFunction>,
    parent: Option<&'a Scope<'a>>,
}

impl<'a> Scope<'a> {
    /// Creates a new root scope.
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            parent: None,
        }
    }

    /// Creates a new child scope with a parent.
    pub fn with_parent(parent: &'a Scope<'a>) -> Self {
        Self {
            vars: HashMap::new(),
            funcs: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Resolves a variable name to its quantity, searching up the hierarchy.
    pub fn get_var(&self, name: &str) -> Option<Quantity> {
        if let Some(v) = resolve_static_var(name) {
            return Some(v);
        }
        self.vars.get(name).copied().or_else(|| self.parent.and_then(|p| p.get_var(name)))
    }

    /// Inserts a variable into the local scope.
    pub fn insert_var(&mut self, name: String, val: Quantity) {
        self.vars.insert(name, val);
    }

    /// Resolves a function name, searching up the hierarchy.
    pub fn get_func(&self, name: &str) -> Option<UserFunction> {
        self.funcs.get(name).cloned().or_else(|| self.parent.and_then(|p| p.get_func(name)))
    }

    /// Inserts a function into the local scope.
    pub fn insert_func(&mut self, name: String, func: UserFunction) {
        self.funcs.insert(name, func);
    }

    /// Returns the local variable map.
    pub fn vars(&self) -> &HashMap<String, Quantity> {
        &self.vars
    }

    /// Returns the local function map.
    pub fn funcs(&self) -> &HashMap<String, UserFunction> {
        &self.funcs
    }
}

impl<'a> Default for Scope<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Evaluates an AST node within a given scope.
pub fn evaluate(
    node: &Node,
    scope: &mut Scope,
) -> Result<Quantity, Error> {
    let pos = node.pos;
    match &node.expr {
        Expr::Number(n) => Ok(Quantity::scalar(*n)),
        Expr::Variable(name) => {
            scope.get_var(name)
                .ok_or_else(|| Error::Math(MathError::UnknownVariable(name.clone()), pos))
        }
        Expr::Assign(name, e) => {
            if is_protected(name) {
                return Err(Error::Math(MathError::ProtectedName(name.clone()), pos));
            }
            let v = evaluate(e, scope)?;
            scope.insert_var(name.clone(), v);
            Ok(v)
        }
        Expr::FnDefine(name, params, body) => {
            if is_protected(name) {
                return Err(Error::Math(MathError::ProtectedName(name.clone()), pos));
            }
            scope.insert_func(
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
                .map(|a| evaluate(a, scope))
                .collect::<Result<Vec<_>, _>>()?;

            if let Some(f) = scope.get_func(name) {
                if vs.len() != f.params.len() {
                    return Err(Error::Math(
                        MathError::ArityMismatch { expected: f.params.len(), actual: vs.len() },
                        pos,
                    ));
                }
                let mut child_scope = Scope::with_parent(scope);
                for (p, v) in f.params.iter().zip(vs) {
                    child_scope.insert_var(p.clone(), v);
                }
                return evaluate(&f.body, &mut child_scope);
            }

            if let Some(b) = BUILTINS.iter().find(|b| b.name == name) {
                match b.arity {
                    Arity::Fixed(n) if vs.len() != n => {
                        return Err(Error::Math(
                            MathError::ArityMismatch { expected: n, actual: vs.len() },
                            pos,
                        ));
                    }
                    _ => {}
                }
                return (b.func)(&vs).map_err(|e| Error::Math(MathError::Generic(e), pos));
            }

            Err(Error::Math(MathError::UnknownFunction(name.clone()), pos))
        }
        Expr::Binary(op, l, r) => {
            let (lv, rv) = (evaluate(l, scope)?, evaluate(r, scope)?);
            match op {
                BinOp::Add => (lv + rv).map_err(|e| Error::Math(MathError::Generic(e), pos)),
                BinOp::Sub => (lv - rv).map_err(|e| Error::Math(MathError::Generic(e), pos)),
                BinOp::Mul => Ok(lv * rv),
                BinOp::Div => {
                    if rv.value == 0.0 {
                        Err(Error::Math(MathError::DivisionByZero, pos))
                    } else {
                        Ok(lv / rv)
                    }
                }
                BinOp::Mod => {
                    if !rv.is_scalar() || !lv.is_scalar() {
                        return Err(Error::Math(MathError::NonScalarOperation("Modulo".into()), pos));
                    }
                    if rv.value == 0.0 {
                        Err(Error::Math(MathError::ModuloByZero, pos))
                    } else {
                        Ok(Quantity::scalar(lv.value % rv.value))
                    }
                }
                BinOp::Pow => {
                    if !rv.is_scalar() {
                        return Err(Error::Math(MathError::NonScalarOperation("Exponentiation".into()), pos));
                    }
                    lv.pow(rv.value).map_err(|e| Error::Math(MathError::Generic(e), pos))
                }
            }
        }
        Expr::Factorial(e) => {
            let v = evaluate(e, scope)?;
            if !v.is_scalar() {
                return Err(Error::Math(MathError::NonScalarOperation("Factorial".into()), pos));
            }
            if v.value < 0.0 || v.value.fract() != 0.0 {
                return Err(Error::Math(MathError::Generic("Factorial needs non-negative integer".to_string()), pos));
            }
            if v.value > 170.0 {
                return Err(Error::Math(MathError::Overflow("Factorial result too large".to_string()), pos));
            }
            let mut r = 1.0;
            for i in 1..=(v.value as u64) {
                r *= i as f64;
            }
            Ok(Quantity::scalar(r))
        }
        Expr::Unary(op, e) => {
            let v = evaluate(e, scope)?;
            Ok(if let UnOp::Neg = op { -v } else { v })
        }
        Expr::Convert(e, target_node) => {
            let v = evaluate(e, scope)?;
            let target = evaluate(target_node, scope)?;

            if v.dims != target.dims {
                return Err(Error::Math(
                    MathError::DimensionMismatch {
                        expected: target.dims.to_string(),
                        actual: v.dims.to_string(),
                    },
                    target_node.pos,
                ));
            }

            let val_si = if let Expr::Variable(name) = &e.expr {
                if let Some(unit) = UNITS.iter().find(|u| u.name == name) {
                    unit.convert_to_si(v.value)
                } else {
                    v.value
                }
            } else if let Expr::Binary(BinOp::Mul, l, r) = &e.expr {
                if let Expr::Variable(name) = &r.expr {
                    if let Some(unit) = UNITS.iter().find(|u| u.name == name) {
                        unit.convert_to_si(evaluate(l, scope)?.value)
                    } else {
                        v.value
                    }
                } else {
                    v.value
                }
            } else {
                v.value
            };

            if let Expr::Variable(name) = &target_node.expr
                && let Some(unit) = UNITS.iter().find(|u| u.name == name)
            {
                return Ok(Quantity::scalar(unit.convert_from_si(val_si)));
            }

            if target.value == 0.0 {
                return Err(Error::Math(
                    MathError::Generic("Cannot convert to zero-value unit".to_string()),
                    target_node.pos,
                ));
            }

            Ok(Quantity::scalar(val_si / target.value))
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
        scope: &mut Scope,
    ) -> Result<Quantity, Error> {
        let toks = tokenize(input).unwrap();
        let mut p = Parser::new(toks);
        let ast = p.parse_expr().unwrap();
        evaluate(&ast, scope)
    }

    #[test]
    fn test_eval_simple() {
        let mut scope = Scope::new();
        let res = eval_str("1 + 2 * 3", &mut scope).unwrap();
        assert_eq!(res.value, 7.0);
    }

    #[test]
    fn test_eval_vars() {
        let mut scope = Scope::new();
        eval_str("a = 5", &mut scope).unwrap();
        let res = eval_str("a * 2", &mut scope).unwrap();
        assert_eq!(res.value, 10.0);
    }

    #[test]
    fn test_eval_funcs() {
        let mut scope = Scope::new();
        eval_str("f(x) = x * x", &mut scope).unwrap();
        let res = eval_str("f(3) + 1", &mut scope).unwrap();
        assert_eq!(res.value, 10.0);
    }

    #[test]
    fn test_units() {
        let mut scope = Scope::new();
        let res = eval_str("10m / 2s", &mut scope).unwrap();
        assert_eq!(res.value, 5.0);
        assert_eq!(res.dims.length, 1);
        assert_eq!(res.dims.time, -1);
    }

    #[test]
    fn test_scope_isolation() {
        let mut scope = Scope::new();
        eval_str("f(x) = y = x + 1", &mut scope).unwrap();
        eval_str("f(10)", &mut scope).unwrap();
        
        let res = eval_str("y", &mut scope);
        assert!(res.is_err(), "Variable 'y' should not be visible in global scope");
    }

    #[test]
    fn test_temperature_conversion() {
        let mut scope = Scope::new();
        let res = eval_str("0degC in K", &mut scope).unwrap();
        assert_eq!(res.value, 273.15);
        
        let res = eval_str("100degC in K", &mut scope).unwrap();
        assert_eq!(res.value, 373.15);

        let res = eval_str("300K in degC", &mut scope).unwrap();
        assert!((res.value - 26.85).abs() < 1e-10);
    }
}
