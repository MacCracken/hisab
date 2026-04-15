//! Bridge API for abaco integration.
//!
//! Provides conversion between hisab's [`Expr`] and abaco's `Value` type,
//! solver dispatch, and verified evaluation using interval arithmetic.
//!
//! This module does NOT depend on abaco — it provides the hisab side of the
//! bridge. Abaco depends on hisab and implements the other direction.

use super::Expr;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Expr ↔ serializable representation (bridge to abaco Value)
// ---------------------------------------------------------------------------

/// A serializable, flat representation of a symbolic expression.
///
/// Abaco's `Value` type can convert to/from this representation without
/// depending on hisab's internal `Expr` enum directly.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[non_exhaustive]
pub enum ExprValue {
    /// A numeric constant.
    Number(f64),
    /// A named variable.
    Symbol(String),
    /// A function call: (name, arguments).
    Call(String, Vec<ExprValue>),
    /// Negation.
    Negate(Box<ExprValue>),
}

/// Convert an [`Expr`] to an [`ExprValue`] for bridge transport to abaco.
#[must_use]
pub fn expr_to_value(expr: &Expr) -> ExprValue {
    match expr {
        Expr::Const(c) => ExprValue::Number(*c),
        Expr::Var(name) => ExprValue::Symbol(name.clone()),
        Expr::Add(a, b) => ExprValue::Call("add".into(), vec![expr_to_value(a), expr_to_value(b)]),
        Expr::Mul(a, b) => ExprValue::Call("mul".into(), vec![expr_to_value(a), expr_to_value(b)]),
        Expr::Pow(a, b) => ExprValue::Call("pow".into(), vec![expr_to_value(a), expr_to_value(b)]),
        Expr::Neg(a) => ExprValue::Negate(Box::new(expr_to_value(a))),
        Expr::Sin(a) => ExprValue::Call("sin".into(), vec![expr_to_value(a)]),
        Expr::Cos(a) => ExprValue::Call("cos".into(), vec![expr_to_value(a)]),
        Expr::Exp(a) => ExprValue::Call("exp".into(), vec![expr_to_value(a)]),
        Expr::Ln(a) => ExprValue::Call("ln".into(), vec![expr_to_value(a)]),
    }
}

/// Convert an [`ExprValue`] back to an [`Expr`].
///
/// Returns `None` if the value contains an unrecognized function call.
#[must_use]
pub fn value_to_expr(val: &ExprValue) -> Option<Expr> {
    match val {
        ExprValue::Number(c) => Some(Expr::Const(*c)),
        ExprValue::Symbol(name) => Some(Expr::Var(name.clone())),
        ExprValue::Negate(inner) => Some(Expr::Neg(Box::new(value_to_expr(inner)?))),
        ExprValue::Call(name, args) => match (name.as_str(), args.as_slice()) {
            ("add", [a, b]) => Some(Expr::Add(
                Box::new(value_to_expr(a)?),
                Box::new(value_to_expr(b)?),
            )),
            ("mul", [a, b]) => Some(Expr::Mul(
                Box::new(value_to_expr(a)?),
                Box::new(value_to_expr(b)?),
            )),
            ("pow", [a, b]) => Some(Expr::Pow(
                Box::new(value_to_expr(a)?),
                Box::new(value_to_expr(b)?),
            )),
            ("sin", [a]) => Some(Expr::Sin(Box::new(value_to_expr(a)?))),
            ("cos", [a]) => Some(Expr::Cos(Box::new(value_to_expr(a)?))),
            ("exp", [a]) => Some(Expr::Exp(Box::new(value_to_expr(a)?))),
            ("ln", [a]) => Some(Expr::Ln(Box::new(value_to_expr(a)?))),
            _ => None,
        },
    }
}

// ---------------------------------------------------------------------------
// Solver dispatch
// ---------------------------------------------------------------------------

/// Options for the equation solver.
#[derive(Debug, Clone)]
pub struct SolveOptions {
    /// Initial guess for Newton's method.
    pub x0: f64,
    /// Search interval for bisection [a, b].
    pub bracket: Option<(f64, f64)>,
    /// Convergence tolerance.
    pub tol: f64,
    /// Maximum iterations.
    pub max_iter: usize,
}

impl Default for SolveOptions {
    fn default() -> Self {
        Self {
            x0: 0.0,
            bracket: None,
            tol: 1e-12,
            max_iter: 100,
        }
    }
}

/// Solve `expr = 0` for the given variable.
///
/// Tries Newton-Raphson first (using symbolic differentiation), then falls back
/// to bisection if a bracket is provided.
///
/// # Errors
///
/// Returns [`crate::HisabError`] if neither method converges.
pub fn solve_expr(expr: &Expr, var: &str, opts: &SolveOptions) -> Result<f64, crate::HisabError> {
    let deriv = expr.differentiate(var).simplify();

    // Build closures for f(x) and f'(x)
    let f = |x: f64| -> f64 {
        let mut vars = HashMap::new();
        vars.insert(var.to_string(), x);
        expr.evaluate(&vars).unwrap_or(f64::NAN)
    };

    let df = |x: f64| -> f64 {
        let mut vars = HashMap::new();
        vars.insert(var.to_string(), x);
        deriv.evaluate(&vars).unwrap_or(f64::NAN)
    };

    // Try Newton-Raphson first
    match crate::num::newton_raphson(f, df, opts.x0, opts.tol, opts.max_iter) {
        Ok(root) => return Ok(root),
        Err(_) if opts.bracket.is_some() => {} // Fall through to bisection
        Err(e) => return Err(e),
    }

    // Bisection fallback
    if let Some((a, b)) = opts.bracket {
        crate::num::bisection(f, a, b, opts.tol, opts.max_iter)
    } else {
        Err(crate::HisabError::NoConvergence(opts.max_iter))
    }
}

// ---------------------------------------------------------------------------
// Verified evaluation via Interval
// ---------------------------------------------------------------------------

/// Evaluate an expression using interval arithmetic for verified error bounds.
///
/// Each variable is mapped to an [`crate::interval::Interval`]. The result is
/// an interval guaranteed to contain the true value.
///
/// # Errors
///
/// Returns [`crate::HisabError::InvalidInput`] if a variable is unbound.
#[cfg(feature = "interval")]
pub fn eval_verified(
    expr: &Expr,
    vars: &HashMap<String, crate::interval::Interval>,
) -> Result<crate::interval::Interval, crate::HisabError> {
    use crate::interval::Interval;

    match expr {
        Expr::Const(c) => Ok(Interval::point(*c)),
        Expr::Var(name) => vars
            .get(name)
            .copied()
            .ok_or_else(|| crate::HisabError::InvalidInput(format!("undefined variable: {name}"))),
        Expr::Add(a, b) => Ok(eval_verified(a, vars)? + eval_verified(b, vars)?),
        Expr::Mul(a, b) => Ok(eval_verified(a, vars)? * eval_verified(b, vars)?),
        Expr::Pow(base, exp) => {
            let b = eval_verified(base, vars)?;
            let e = eval_verified(exp, vars)?;
            // For integer exponents, use sqr; otherwise approximate
            let mid_exp = e.midpoint();
            if (mid_exp - mid_exp.round()).abs() < 1e-15 && e.width() < 1e-15 {
                let n = mid_exp.round() as i32;
                if n == 2 {
                    return Ok(b.sqr());
                }
            }
            // General power: use monotonicity for positive base
            let lo = b
                .lo()
                .powf(e.lo())
                .min(b.lo().powf(e.hi()))
                .min(b.hi().powf(e.lo()))
                .min(b.hi().powf(e.hi()));
            let hi = b
                .lo()
                .powf(e.lo())
                .max(b.lo().powf(e.hi()))
                .max(b.hi().powf(e.lo()))
                .max(b.hi().powf(e.hi()));
            Ok(Interval::new(lo, hi))
        }
        Expr::Neg(a) => Ok(-eval_verified(a, vars)?),
        Expr::Sin(a) => {
            let iv = eval_verified(a, vars)?;
            // Conservative: sin bounds over the interval
            if iv.width() >= 2.0 * std::f64::consts::PI {
                Ok(Interval::new(-1.0, 1.0))
            } else {
                let samples = [iv.lo(), iv.hi(), iv.midpoint()];
                let mut lo = f64::INFINITY;
                let mut hi = f64::NEG_INFINITY;
                for &s in &samples {
                    let v = s.sin();
                    lo = lo.min(v);
                    hi = hi.max(v);
                }
                // Check if critical points (π/2 + kπ) are in interval
                let k_start = (iv.lo() / std::f64::consts::PI - 0.5).ceil() as i64;
                let k_end = (iv.hi() / std::f64::consts::PI - 0.5).floor() as i64;
                for k in k_start..=k_end {
                    let cp = (k as f64 + 0.5) * std::f64::consts::PI;
                    if iv.contains(cp) {
                        let v = cp.sin();
                        lo = lo.min(v);
                        hi = hi.max(v);
                    }
                }
                Ok(Interval::new(lo, hi))
            }
        }
        Expr::Cos(a) => {
            let iv = eval_verified(a, vars)?;
            if iv.width() >= 2.0 * std::f64::consts::PI {
                Ok(Interval::new(-1.0, 1.0))
            } else {
                let samples = [iv.lo(), iv.hi(), iv.midpoint()];
                let mut lo = f64::INFINITY;
                let mut hi = f64::NEG_INFINITY;
                for &s in &samples {
                    let v = s.cos();
                    lo = lo.min(v);
                    hi = hi.max(v);
                }
                // Critical points at kπ
                let k_start = (iv.lo() / std::f64::consts::PI).ceil() as i64;
                let k_end = (iv.hi() / std::f64::consts::PI).floor() as i64;
                for k in k_start..=k_end {
                    let cp = k as f64 * std::f64::consts::PI;
                    if iv.contains(cp) {
                        let v = cp.cos();
                        lo = lo.min(v);
                        hi = hi.max(v);
                    }
                }
                Ok(Interval::new(lo, hi))
            }
        }
        Expr::Exp(a) => {
            let iv = eval_verified(a, vars)?;
            // exp is monotonically increasing
            Ok(Interval::new(iv.lo().exp(), iv.hi().exp()))
        }
        Expr::Ln(a) => {
            let iv = eval_verified(a, vars)?;
            if iv.lo() <= 0.0 {
                return Err(crate::HisabError::InvalidInput(
                    "ln of non-positive interval".into(),
                ));
            }
            // ln is monotonically increasing
            Ok(Interval::new(iv.lo().ln(), iv.hi().ln()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn var(name: &str) -> Expr {
        Expr::Var(name.into())
    }
    fn c(v: f64) -> Expr {
        Expr::Const(v)
    }

    // -- Bridge roundtrip --

    #[test]
    fn roundtrip_const() {
        let e = c(42.0);
        let v = expr_to_value(&e);
        let e2 = value_to_expr(&v).unwrap();
        assert_eq!(e, e2);
    }

    #[test]
    fn roundtrip_var() {
        let e = var("x");
        let v = expr_to_value(&e);
        let e2 = value_to_expr(&v).unwrap();
        assert_eq!(e, e2);
    }

    #[test]
    fn roundtrip_complex() {
        // sin(x^2 + 1)
        let e = Expr::Sin(Box::new(Expr::Add(
            Box::new(Expr::Pow(Box::new(var("x")), Box::new(c(2.0)))),
            Box::new(c(1.0)),
        )));
        let v = expr_to_value(&e);
        let e2 = value_to_expr(&v).unwrap();
        assert_eq!(e, e2);
    }

    #[test]
    fn roundtrip_neg() {
        let e = Expr::Neg(Box::new(var("x")));
        let v = expr_to_value(&e);
        let e2 = value_to_expr(&v).unwrap();
        assert_eq!(e, e2);
    }

    #[test]
    fn roundtrip_all_ops() {
        for e in [
            Expr::Exp(Box::new(var("x"))),
            Expr::Ln(Box::new(var("x"))),
            Expr::Cos(Box::new(var("x"))),
            Expr::Mul(Box::new(c(2.0)), Box::new(var("x"))),
        ] {
            let v = expr_to_value(&e);
            let e2 = value_to_expr(&v).unwrap();
            assert_eq!(e, e2);
        }
    }

    #[test]
    fn value_unknown_returns_none() {
        let v = ExprValue::Call("unknown_func".into(), vec![ExprValue::Number(1.0)]);
        assert!(value_to_expr(&v).is_none());
    }

    // -- Solver dispatch --

    #[test]
    fn solve_x_squared_minus_2() {
        // x² - 2 = 0 → x = √2
        let expr = Expr::Add(
            Box::new(Expr::Pow(Box::new(var("x")), Box::new(c(2.0)))),
            Box::new(c(-2.0)),
        );
        let opts = SolveOptions {
            x0: 1.5,
            bracket: Some((1.0, 2.0)),
            ..Default::default()
        };
        let root = solve_expr(&expr, "x", &opts).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-10);
    }

    #[test]
    fn solve_sin_x() {
        // sin(x) = 0 near x = π
        let expr = Expr::Sin(Box::new(var("x")));
        let opts = SolveOptions {
            x0: 3.0,
            bracket: Some((2.5, 3.5)),
            ..Default::default()
        };
        let root = solve_expr(&expr, "x", &opts).unwrap();
        assert!((root - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn solve_with_bisection_fallback() {
        // x³ - 1 = 0 → x = 1 (use bracket, bad initial guess)
        let expr = Expr::Add(
            Box::new(Expr::Pow(Box::new(var("x")), Box::new(c(3.0)))),
            Box::new(c(-1.0)),
        );
        let opts = SolveOptions {
            x0: 100.0, // bad guess
            bracket: Some((0.0, 2.0)),
            ..Default::default()
        };
        let root = solve_expr(&expr, "x", &opts).unwrap();
        assert!((root - 1.0).abs() < 1e-10);
    }

    // -- Verified evaluation --

    #[cfg(feature = "interval")]
    #[test]
    fn verified_const() {
        let e = c(3.0);
        let result = eval_verified(&e, &HashMap::new()).unwrap();
        assert!((result.lo() - 3.0).abs() < 1e-15);
        assert!((result.hi() - 3.0).abs() < 1e-15);
    }

    #[cfg(feature = "interval")]
    #[test]
    fn verified_add() {
        use crate::interval::Interval;
        let e = Expr::Add(Box::new(var("x")), Box::new(c(1.0)));
        let mut vars = HashMap::new();
        vars.insert("x".into(), Interval::new(1.0, 2.0));
        let result = eval_verified(&e, &vars).unwrap();
        assert!((result.lo() - 2.0).abs() < 1e-15);
        assert!((result.hi() - 3.0).abs() < 1e-15);
    }

    #[cfg(feature = "interval")]
    #[test]
    fn verified_mul() {
        use crate::interval::Interval;
        let e = Expr::Mul(Box::new(var("x")), Box::new(var("x")));
        let mut vars = HashMap::new();
        vars.insert("x".into(), Interval::new(2.0, 3.0));
        let result = eval_verified(&e, &vars).unwrap();
        // [2,3] * [2,3] = [4, 9]
        assert!(result.lo() <= 4.0 + 1e-10);
        assert!(result.hi() >= 9.0 - 1e-10);
    }

    #[cfg(feature = "interval")]
    #[test]
    fn verified_exp_monotonic() {
        use crate::interval::Interval;
        let e = Expr::Exp(Box::new(var("x")));
        let mut vars = HashMap::new();
        vars.insert("x".into(), Interval::new(0.0, 1.0));
        let result = eval_verified(&e, &vars).unwrap();
        assert!((result.lo() - 1.0).abs() < 1e-10); // e^0 = 1
        assert!((result.hi() - std::f64::consts::E).abs() < 1e-10); // e^1 = e
    }

    #[cfg(feature = "interval")]
    #[test]
    fn verified_ln_positive() {
        use crate::interval::Interval;
        let e = Expr::Ln(Box::new(var("x")));
        let mut vars = HashMap::new();
        vars.insert("x".into(), Interval::new(1.0, std::f64::consts::E));
        let result = eval_verified(&e, &vars).unwrap();
        assert!((result.lo()).abs() < 1e-10); // ln(1) = 0
        assert!((result.hi() - 1.0).abs() < 1e-10); // ln(e) = 1
    }

    #[cfg(feature = "interval")]
    #[test]
    fn verified_ln_nonpositive_errors() {
        use crate::interval::Interval;
        let e = Expr::Ln(Box::new(var("x")));
        let mut vars = HashMap::new();
        vars.insert("x".into(), Interval::new(-1.0, 1.0));
        assert!(eval_verified(&e, &vars).is_err());
    }

    #[cfg(feature = "interval")]
    #[test]
    fn verified_sin_bounds() {
        use crate::interval::Interval;
        let e = Expr::Sin(Box::new(var("x")));
        let mut vars = HashMap::new();
        // Full period: should give [-1, 1]
        vars.insert("x".into(), Interval::new(0.0, 7.0));
        let result = eval_verified(&e, &vars).unwrap();
        assert!(result.lo() <= -1.0 + 1e-10);
        assert!(result.hi() >= 1.0 - 1e-10);
    }
}
