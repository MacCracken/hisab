//! Symbolic algebra primitives.
//!
//! Provides an expression tree ([`Expr`]) supporting evaluation, symbolic
//! differentiation, basic algebraic simplification, advanced simplification
//! rules, symbolic integration, LaTeX rendering, and pattern matching.

mod bridge;
mod integrate;
mod latex;
mod pattern;
mod simplify_rules;

pub use bridge::{
    ExprValue, SolveOptions, eval_verified, expr_to_value, solve_expr, value_to_expr,
};
pub use integrate::symbolic_integrate;
pub use latex::to_latex;
pub use pattern::{
    Pattern, RewriteRule, apply_rule, instantiate, match_expr, rewrite, rewrite_fixpoint,
};
pub use simplify_rules::simplify_advanced;

use std::collections::HashMap;
use std::fmt;

/// Check if a float is effectively zero.
fn is_zero(x: f64) -> bool {
    x.abs() < 1e-15
}

/// Check if a float is effectively one.
fn is_one(x: f64) -> bool {
    (x - 1.0).abs() < 1e-15
}

/// A symbolic mathematical expression.
///
/// # Examples
///
/// ```
/// use hisab::symbolic::Expr;
/// use std::collections::HashMap;
///
/// // Build x² + 1, differentiate, evaluate
/// let x = Expr::Var("x".into());
/// let expr = Expr::Add(
///     Box::new(Expr::Pow(Box::new(x.clone()), Box::new(Expr::Const(2.0)))),
///     Box::new(Expr::Const(1.0)),
/// );
/// let d = expr.differentiate("x").simplify();
/// let mut vars = HashMap::new();
/// vars.insert("x".into(), 3.0);
/// assert!((d.evaluate(&vars).unwrap() - 6.0).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Expr {
    /// A constant value.
    Const(f64),
    /// A named variable.
    Var(String),
    /// Addition: lhs + rhs.
    Add(Box<Expr>, Box<Expr>),
    /// Multiplication: lhs * rhs.
    Mul(Box<Expr>, Box<Expr>),
    /// Power: base ^ exponent.
    Pow(Box<Expr>, Box<Expr>),
    /// Negation: -expr.
    Neg(Box<Expr>),
    /// Sine.
    Sin(Box<Expr>),
    /// Cosine.
    Cos(Box<Expr>),
    /// Exponential (e^x).
    Exp(Box<Expr>),
    /// Natural logarithm.
    Ln(Box<Expr>),
}

impl Expr {
    /// Evaluate the expression with the given variable bindings.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HisabError::InvalidInput`] if a variable is not in `vars`.
    #[must_use = "returns the evaluated result or an error"]
    pub fn evaluate(&self, vars: &HashMap<String, f64>) -> Result<f64, crate::HisabError> {
        match self {
            Expr::Const(c) => Ok(*c),
            Expr::Var(name) => vars.get(name).copied().ok_or_else(|| {
                crate::HisabError::InvalidInput(format!("undefined variable: {name}"))
            }),
            Expr::Add(a, b) => Ok(a.evaluate(vars)? + b.evaluate(vars)?),
            Expr::Mul(a, b) => Ok(a.evaluate(vars)? * b.evaluate(vars)?),
            Expr::Pow(base, exp) => Ok(base.evaluate(vars)?.powf(exp.evaluate(vars)?)),
            Expr::Neg(a) => Ok(-a.evaluate(vars)?),
            Expr::Sin(a) => Ok(a.evaluate(vars)?.sin()),
            Expr::Cos(a) => Ok(a.evaluate(vars)?.cos()),
            Expr::Exp(a) => Ok(a.evaluate(vars)?.exp()),
            Expr::Ln(a) => Ok(a.evaluate(vars)?.ln()),
        }
    }

    /// Symbolic differentiation with respect to `var`.
    #[must_use]
    pub fn differentiate(&self, var: &str) -> Expr {
        match self {
            Expr::Const(_) => Expr::Const(0.0),
            Expr::Var(name) => {
                if name == var {
                    Expr::Const(1.0)
                } else {
                    Expr::Const(0.0)
                }
            }
            Expr::Add(a, b) => Expr::Add(
                Box::new(a.differentiate(var)),
                Box::new(b.differentiate(var)),
            ),
            Expr::Mul(a, b) => {
                // Product rule: (a*b)' = a'*b + a*b'
                Expr::Add(
                    Box::new(Expr::Mul(Box::new(a.differentiate(var)), b.clone())),
                    Box::new(Expr::Mul(a.clone(), Box::new(b.differentiate(var)))),
                )
            }
            Expr::Pow(base, exp) => {
                // Power rule for constant exponent: (x^n)' = n*x^(n-1)*x'
                // General: (f^g)' = f^g * (g'*ln(f) + g*f'/f)
                if let Expr::Const(n) = exp.as_ref() {
                    Expr::Mul(
                        Box::new(Expr::Mul(
                            Box::new(Expr::Const(*n)),
                            Box::new(Expr::Pow(base.clone(), Box::new(Expr::Const(n - 1.0)))),
                        )),
                        Box::new(base.differentiate(var)),
                    )
                } else {
                    // General case: f^g * (g'*ln(f) + g*f'/f)
                    let ln_f = Expr::Ln(base.clone());
                    let f_prime = base.differentiate(var);
                    let g_prime = exp.differentiate(var);
                    Expr::Mul(
                        Box::new(Expr::Pow(base.clone(), exp.clone())),
                        Box::new(Expr::Add(
                            Box::new(Expr::Mul(Box::new(g_prime), Box::new(ln_f))),
                            Box::new(Expr::Mul(
                                exp.clone(),
                                Box::new(Expr::Mul(
                                    Box::new(f_prime),
                                    Box::new(Expr::Pow(base.clone(), Box::new(Expr::Const(-1.0)))),
                                )),
                            )),
                        )),
                    )
                }
            }
            Expr::Neg(a) => Expr::Neg(Box::new(a.differentiate(var))),
            Expr::Sin(a) => {
                // (sin(f))' = cos(f)*f'
                Expr::Mul(
                    Box::new(Expr::Cos(a.clone())),
                    Box::new(a.differentiate(var)),
                )
            }
            Expr::Cos(a) => {
                // (cos(f))' = -sin(f)*f'
                Expr::Neg(Box::new(Expr::Mul(
                    Box::new(Expr::Sin(a.clone())),
                    Box::new(a.differentiate(var)),
                )))
            }
            Expr::Exp(a) => {
                // (e^f)' = e^f * f'
                Expr::Mul(
                    Box::new(Expr::Exp(a.clone())),
                    Box::new(a.differentiate(var)),
                )
            }
            Expr::Ln(a) => {
                // (ln(f))' = f'/f
                Expr::Mul(
                    Box::new(a.differentiate(var)),
                    Box::new(Expr::Pow(a.clone(), Box::new(Expr::Const(-1.0)))),
                )
            }
        }
    }

    /// Substitute a variable with another expression.
    #[must_use]
    pub fn substitute(&self, var: &str, replacement: &Expr) -> Expr {
        match self {
            Expr::Const(_) => self.clone(),
            Expr::Var(name) => {
                if name == var {
                    replacement.clone()
                } else {
                    self.clone()
                }
            }
            Expr::Add(a, b) => Expr::Add(
                Box::new(a.substitute(var, replacement)),
                Box::new(b.substitute(var, replacement)),
            ),
            Expr::Mul(a, b) => Expr::Mul(
                Box::new(a.substitute(var, replacement)),
                Box::new(b.substitute(var, replacement)),
            ),
            Expr::Pow(a, b) => Expr::Pow(
                Box::new(a.substitute(var, replacement)),
                Box::new(b.substitute(var, replacement)),
            ),
            Expr::Neg(a) => Expr::Neg(Box::new(a.substitute(var, replacement))),
            Expr::Sin(a) => Expr::Sin(Box::new(a.substitute(var, replacement))),
            Expr::Cos(a) => Expr::Cos(Box::new(a.substitute(var, replacement))),
            Expr::Exp(a) => Expr::Exp(Box::new(a.substitute(var, replacement))),
            Expr::Ln(a) => Expr::Ln(Box::new(a.substitute(var, replacement))),
        }
    }

    /// Simplify the expression using basic algebraic rules.
    #[must_use]
    pub fn simplify(&self) -> Expr {
        match self {
            Expr::Add(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (Expr::Const(x), _) if is_zero(*x) => b,
                    (_, Expr::Const(x)) if is_zero(*x) => a,
                    (Expr::Const(x), Expr::Const(y)) => Expr::Const(x + y),
                    _ => Expr::Add(Box::new(a), Box::new(b)),
                }
            }
            Expr::Mul(a, b) => {
                let a = a.simplify();
                let b = b.simplify();
                match (&a, &b) {
                    (Expr::Const(x), _) | (_, Expr::Const(x)) if is_zero(*x) => Expr::Const(0.0),
                    (Expr::Const(x), _) if is_one(*x) => b,
                    (_, Expr::Const(x)) if is_one(*x) => a,
                    (Expr::Const(x), Expr::Const(y)) => Expr::Const(x * y),
                    _ => Expr::Mul(Box::new(a), Box::new(b)),
                }
            }
            Expr::Pow(base, exp) => {
                let base = base.simplify();
                let exp = exp.simplify();
                match (&base, &exp) {
                    (_, Expr::Const(x)) if is_zero(*x) => Expr::Const(1.0),
                    (_, Expr::Const(x)) if is_one(*x) => base,
                    (Expr::Const(x), Expr::Const(y)) => Expr::Const(x.powf(*y)),
                    _ => Expr::Pow(Box::new(base), Box::new(exp)),
                }
            }
            Expr::Neg(a) => {
                let a = a.simplify();
                match &a {
                    Expr::Const(x) if is_zero(*x) => Expr::Const(0.0),
                    Expr::Const(x) => Expr::Const(-x),
                    Expr::Neg(inner) => inner.simplify(),
                    _ => Expr::Neg(Box::new(a)),
                }
            }
            Expr::Sin(a) => Expr::Sin(Box::new(a.simplify())),
            Expr::Cos(a) => Expr::Cos(Box::new(a.simplify())),
            Expr::Exp(a) => {
                let a = a.simplify();
                match &a {
                    Expr::Const(x) => Expr::Const(x.exp()),
                    _ => Expr::Exp(Box::new(a)),
                }
            }
            Expr::Ln(a) => {
                let a = a.simplify();
                match &a {
                    Expr::Const(x) => Expr::Const(x.ln()),
                    _ => Expr::Ln(Box::new(a)),
                }
            }
            Expr::Const(_) | Expr::Var(_) => self.clone(),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Const(c) => write!(f, "{c}"),
            Expr::Var(name) => write!(f, "{name}"),
            Expr::Add(a, b) => write!(f, "({a} + {b})"),
            Expr::Mul(a, b) => write!(f, "({a} * {b})"),
            Expr::Pow(base, exp) => write!(f, "({base}^{exp})"),
            Expr::Neg(a) => write!(f, "-{a}"),
            Expr::Sin(a) => write!(f, "sin({a})"),
            Expr::Cos(a) => write!(f, "cos({a})"),
            Expr::Exp(a) => write!(f, "exp({a})"),
            Expr::Ln(a) => write!(f, "ln({a})"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn var(name: &str) -> Expr {
        Expr::Var(name.to_string())
    }

    fn c(val: f64) -> Expr {
        Expr::Const(val)
    }

    fn vars(pairs: &[(&str, f64)]) -> HashMap<String, f64> {
        pairs.iter().map(|&(k, v)| (k.to_string(), v)).collect()
    }

    const EPSILON: f64 = 1e-10;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn eval_constant() {
        let e = c(42.0);
        assert!(approx(e.evaluate(&HashMap::new()).unwrap(), 42.0));
    }

    #[test]
    fn eval_variable() {
        let e = var("x");
        assert!(approx(e.evaluate(&vars(&[("x", 3.0)])).unwrap(), 3.0));
    }

    #[test]
    fn eval_undefined_variable() {
        let e = var("x");
        assert!(e.evaluate(&HashMap::new()).is_err());
    }

    #[test]
    fn eval_arithmetic() {
        // 2*x + 3
        let e = Expr::Add(
            Box::new(Expr::Mul(Box::new(c(2.0)), Box::new(var("x")))),
            Box::new(c(3.0)),
        );
        assert!(approx(e.evaluate(&vars(&[("x", 5.0)])).unwrap(), 13.0));
    }

    #[test]
    fn eval_trig() {
        let e = Expr::Sin(Box::new(c(0.0)));
        assert!(approx(e.evaluate(&HashMap::new()).unwrap(), 0.0));

        let e = Expr::Cos(Box::new(c(0.0)));
        assert!(approx(e.evaluate(&HashMap::new()).unwrap(), 1.0));
    }

    #[test]
    fn differentiate_constant() {
        let e = c(5.0);
        let d = e.differentiate("x");
        assert_eq!(d, c(0.0));
    }

    #[test]
    fn differentiate_variable() {
        let e = var("x");
        let d = e.differentiate("x").simplify();
        assert_eq!(d, c(1.0));
    }

    #[test]
    fn differentiate_x_squared() {
        // x^2 → 2*x^1*1 → simplifies to 2*x
        let e = Expr::Pow(Box::new(var("x")), Box::new(c(2.0)));
        let d = e.differentiate("x").simplify();
        let v = vars(&[("x", 3.0)]);
        // d/dx(x²) at x=3 should be 6
        assert!(approx(d.evaluate(&v).unwrap(), 6.0));
    }

    #[test]
    fn differentiate_sin() {
        // d/dx sin(x) = cos(x)
        let e = Expr::Sin(Box::new(var("x")));
        let d = e.differentiate("x").simplify();
        let v = vars(&[("x", 0.0)]);
        assert!(approx(d.evaluate(&v).unwrap(), 1.0)); // cos(0) = 1
    }

    #[test]
    fn simplify_add_zero() {
        let e = Expr::Add(Box::new(c(0.0)), Box::new(var("x")));
        assert_eq!(e.simplify(), var("x"));
    }

    #[test]
    fn simplify_mul_zero() {
        let e = Expr::Mul(Box::new(c(0.0)), Box::new(var("x")));
        assert_eq!(e.simplify(), c(0.0));
    }

    #[test]
    fn simplify_mul_one() {
        let e = Expr::Mul(Box::new(c(1.0)), Box::new(var("x")));
        assert_eq!(e.simplify(), var("x"));
    }

    #[test]
    fn simplify_pow_zero() {
        let e = Expr::Pow(Box::new(var("x")), Box::new(c(0.0)));
        assert_eq!(e.simplify(), c(1.0));
    }

    #[test]
    fn simplify_pow_one() {
        let e = Expr::Pow(Box::new(var("x")), Box::new(c(1.0)));
        assert_eq!(e.simplify(), var("x"));
    }

    #[test]
    fn simplify_double_neg() {
        let e = Expr::Neg(Box::new(Expr::Neg(Box::new(var("x")))));
        assert_eq!(e.simplify(), var("x"));
    }

    #[test]
    fn simplify_const_fold() {
        let e = Expr::Add(Box::new(c(2.0)), Box::new(c(3.0)));
        assert_eq!(e.simplify(), c(5.0));
    }

    #[test]
    fn substitute_variable() {
        // x² with x → (y+1) → (y+1)²
        let expr = Expr::Pow(Box::new(var("x")), Box::new(c(2.0)));
        let replacement = Expr::Add(Box::new(var("y")), Box::new(c(1.0)));
        let subst = expr.substitute("x", &replacement);
        let v = vars(&[("y", 2.0)]);
        // (2+1)² = 9
        assert!(approx(subst.evaluate(&v).unwrap(), 9.0));
    }

    #[test]
    fn substitute_no_match() {
        let expr = Expr::Mul(Box::new(c(3.0)), Box::new(var("x")));
        let subst = expr.substitute("z", &c(999.0));
        // No "z" in expression, should be unchanged
        let v = vars(&[("x", 5.0)]);
        assert!(approx(subst.evaluate(&v).unwrap(), 15.0));
    }

    #[test]
    fn display_expr() {
        let e = Expr::Add(
            Box::new(Expr::Mul(Box::new(c(2.0)), Box::new(var("x")))),
            Box::new(c(1.0)),
        );
        let s = format!("{e}");
        assert!(s.contains("2"));
        assert!(s.contains("x"));
    }
}
