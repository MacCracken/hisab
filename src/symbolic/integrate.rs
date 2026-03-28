//! Symbolic integration for basic expression classes.
//!
//! Supports polynomial, trigonometric, and exponential integrands.
//! Returns `None` for expressions it cannot integrate symbolically.

use super::Expr;

/// Attempt symbolic integration of `expr` with respect to `var`.
///
/// Returns `Some(antiderivative)` for supported forms, `None` otherwise.
/// The constant of integration is omitted (indefinite integral).
///
/// # Supported forms
///
/// - Constants: `∫ c dx = c*x`
/// - Variables: `∫ x dx = x²/2`
/// - Powers: `∫ x^n dx = x^(n+1)/(n+1)` for n ≠ -1
/// - Reciprocal: `∫ x^(-1) dx = ln(|x|)` (returned as `ln(x)`)
/// - Exponential: `∫ e^x dx = e^x`
/// - Trig: `∫ sin(x) dx = -cos(x)`, `∫ cos(x) dx = sin(x)`
/// - Sums: `∫ (f+g) dx = ∫f dx + ∫g dx`
/// - Constant multiples: `∫ c*f dx = c * ∫f dx`
/// - Negation: `∫ -f dx = -∫f dx`
#[must_use]
pub fn symbolic_integrate(expr: &Expr, var: &str) -> Option<Expr> {
    match expr {
        // ∫ c dx = c*x
        Expr::Const(c) => Some(Expr::Mul(
            Box::new(Expr::Const(*c)),
            Box::new(Expr::Var(var.into())),
        )),

        // ∫ x dx = x²/2
        Expr::Var(name) if name == var => Some(Expr::Mul(
            Box::new(Expr::Const(0.5)),
            Box::new(Expr::Pow(
                Box::new(Expr::Var(var.into())),
                Box::new(Expr::Const(2.0)),
            )),
        )),

        // ∫ y dx = y*x (y is a different variable, treated as constant)
        Expr::Var(name) if name != var => Some(Expr::Mul(
            Box::new(expr.clone()),
            Box::new(Expr::Var(var.into())),
        )),

        // ∫ (f + g) dx = ∫f dx + ∫g dx
        Expr::Add(a, b) => {
            let ia = symbolic_integrate(a, var)?;
            let ib = symbolic_integrate(b, var)?;
            Some(Expr::Add(Box::new(ia), Box::new(ib)))
        }

        // ∫ -f dx = -∫f dx
        Expr::Neg(a) => {
            let ia = symbolic_integrate(a, var)?;
            Some(Expr::Neg(Box::new(ia)))
        }

        // ∫ c*f dx = c * ∫f dx  (c is constant w.r.t. var)
        Expr::Mul(a, b) => {
            if !contains_var(a, var) {
                let ib = symbolic_integrate(b, var)?;
                Some(Expr::Mul(a.clone(), Box::new(ib)))
            } else if !contains_var(b, var) {
                let ia = symbolic_integrate(a, var)?;
                Some(Expr::Mul(Box::new(ia), b.clone()))
            } else {
                None // Cannot integrate product of two var-dependent exprs symbolically
            }
        }

        // ∫ x^n dx
        Expr::Pow(base, exp) => {
            // Only handle x^n where base=var and exp is constant
            if is_var(base, var) && !contains_var(exp, var) {
                if let Expr::Const(n) = exp.as_ref() {
                    if (*n + 1.0).abs() < 1e-15 {
                        // ∫ x^(-1) dx = ln(x)
                        Some(Expr::Ln(Box::new(Expr::Var(var.into()))))
                    } else {
                        // ∫ x^n dx = x^(n+1) / (n+1)
                        let np1 = n + 1.0;
                        Some(Expr::Mul(
                            Box::new(Expr::Const(1.0 / np1)),
                            Box::new(Expr::Pow(
                                Box::new(Expr::Var(var.into())),
                                Box::new(Expr::Const(np1)),
                            )),
                        ))
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }

        // ∫ sin(x) dx = -cos(x)
        Expr::Sin(a) if is_var(a, var) => Some(Expr::Neg(Box::new(Expr::Cos(Box::new(
            Expr::Var(var.into()),
        ))))),

        // ∫ cos(x) dx = sin(x)
        Expr::Cos(a) if is_var(a, var) => Some(Expr::Sin(Box::new(Expr::Var(var.into())))),

        // ∫ e^x dx = e^x
        Expr::Exp(a) if is_var(a, var) => Some(Expr::Exp(Box::new(Expr::Var(var.into())))),

        // ∫ 1/x dx = ln(x)  (represented as x^(-1))
        Expr::Ln(_) => None, // ln(x) integral is x*ln(x) - x, but complex

        _ => None,
    }
}

/// Check if an expression is exactly the given variable.
fn is_var(expr: &Expr, var: &str) -> bool {
    matches!(expr, Expr::Var(name) if name == var)
}

/// Check if an expression contains the given variable.
fn contains_var(expr: &Expr, var: &str) -> bool {
    match expr {
        Expr::Const(_) => false,
        Expr::Var(name) => name == var,
        Expr::Add(a, b) | Expr::Mul(a, b) | Expr::Pow(a, b) => {
            contains_var(a, var) || contains_var(b, var)
        }
        Expr::Neg(a) | Expr::Sin(a) | Expr::Cos(a) | Expr::Exp(a) | Expr::Ln(a) => {
            contains_var(a, var)
        }
        #[allow(unreachable_patterns)]
        _ => true, // Conservative: assume unknown variants contain var
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn var(name: &str) -> Expr {
        Expr::Var(name.into())
    }
    fn c(v: f64) -> Expr {
        Expr::Const(v)
    }
    fn eval(e: &Expr, x: f64) -> f64 {
        let mut vars = HashMap::new();
        vars.insert("x".to_string(), x);
        e.evaluate(&vars).unwrap()
    }

    #[test]
    fn integrate_constant() {
        // ∫ 3 dx = 3x
        let i = symbolic_integrate(&c(3.0), "x").unwrap();
        let v = eval(&i.simplify(), 2.0);
        assert!((v - 6.0).abs() < 1e-10);
    }

    #[test]
    fn integrate_x() {
        // ∫ x dx = x²/2
        let i = symbolic_integrate(&var("x"), "x").unwrap();
        let v = eval(&i.simplify(), 4.0);
        assert!((v - 8.0).abs() < 1e-10); // 16/2
    }

    #[test]
    fn integrate_x_squared() {
        // ∫ x² dx = x³/3
        let e = Expr::Pow(Box::new(var("x")), Box::new(c(2.0)));
        let i = symbolic_integrate(&e, "x").unwrap();
        let v = eval(&i.simplify(), 3.0);
        assert!((v - 9.0).abs() < 1e-10); // 27/3
    }

    #[test]
    fn integrate_reciprocal() {
        // ∫ x^(-1) dx = ln(x)
        let e = Expr::Pow(Box::new(var("x")), Box::new(c(-1.0)));
        let i = symbolic_integrate(&e, "x").unwrap();
        let v = eval(&i, std::f64::consts::E);
        assert!((v - 1.0).abs() < 1e-10);
    }

    #[test]
    fn integrate_sin() {
        // ∫ sin(x) dx = -cos(x)
        let i = symbolic_integrate(&Expr::Sin(Box::new(var("x"))), "x").unwrap();
        let v = eval(&i.simplify(), 0.0);
        assert!((v - (-1.0)).abs() < 1e-10); // -cos(0) = -1
    }

    #[test]
    fn integrate_cos() {
        // ∫ cos(x) dx = sin(x)
        let i = symbolic_integrate(&Expr::Cos(Box::new(var("x"))), "x").unwrap();
        let v = eval(&i.simplify(), std::f64::consts::FRAC_PI_2);
        assert!((v - 1.0).abs() < 1e-10); // sin(π/2) = 1
    }

    #[test]
    fn integrate_exp() {
        // ∫ e^x dx = e^x
        let i = symbolic_integrate(&Expr::Exp(Box::new(var("x"))), "x").unwrap();
        let v = eval(&i, 0.0);
        assert!((v - 1.0).abs() < 1e-10); // e^0 = 1
    }

    #[test]
    fn integrate_sum() {
        // ∫ (x + 1) dx = x²/2 + x
        let e = Expr::Add(Box::new(var("x")), Box::new(c(1.0)));
        let i = symbolic_integrate(&e, "x").unwrap();
        let v = eval(&i.simplify(), 2.0);
        assert!((v - 4.0).abs() < 1e-10); // 4/2 + 2 = 4
    }

    #[test]
    fn integrate_constant_multiple() {
        // ∫ 3x² dx = x³
        let e = Expr::Mul(
            Box::new(c(3.0)),
            Box::new(Expr::Pow(Box::new(var("x")), Box::new(c(2.0)))),
        );
        let i = symbolic_integrate(&e, "x").unwrap();
        let v = eval(&i.simplify(), 2.0);
        assert!((v - 8.0).abs() < 1e-10); // 3 * 8/3
    }

    #[test]
    fn integrate_negation() {
        // ∫ -x dx = -x²/2
        let e = Expr::Neg(Box::new(var("x")));
        let i = symbolic_integrate(&e, "x").unwrap();
        let v = eval(&i.simplify(), 4.0);
        assert!((v - (-8.0)).abs() < 1e-10);
    }

    #[test]
    fn integrate_unsupported_returns_none() {
        // x * x cannot be integrated by our engine (product of two var-dependent)
        let e = Expr::Mul(Box::new(var("x")), Box::new(var("x")));
        assert!(symbolic_integrate(&e, "x").is_none());
    }

    #[test]
    fn integrate_other_var_as_constant() {
        // ∫ y dx = y*x
        let i = symbolic_integrate(&var("y"), "x").unwrap();
        let mut vars = HashMap::new();
        vars.insert("x".into(), 3.0);
        vars.insert("y".into(), 5.0);
        let v = i.simplify().evaluate(&vars).unwrap();
        assert!((v - 15.0).abs() < 1e-10);
    }
}
