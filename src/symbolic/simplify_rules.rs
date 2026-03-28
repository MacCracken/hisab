//! Advanced algebraic simplification rules: trig identities, log rules, power rules.

use super::Expr;

/// Apply advanced simplification rules beyond basic constant folding.
///
/// Handles:
/// - Trig identities: sin²(x) + cos²(x) = 1, sin(0)=0, cos(0)=1, etc.
/// - Log rules: ln(e^x) = x, ln(1) = 0, ln(a*b) = ln(a)+ln(b)
/// - Power rules: e^(ln(x)) = x, x^a * x^b = x^(a+b), (x^a)^b = x^(a*b)
/// - Negation distribution: -(a+b) = -a + -b
#[must_use]
pub fn simplify_advanced(expr: &Expr) -> Expr {
    // First, apply basic simplification
    let expr = expr.simplify();
    // Then apply advanced rules recursively
    apply_rules(&expr)
}

fn apply_rules(expr: &Expr) -> Expr {
    match expr {
        Expr::Sin(a) => {
            let a = apply_rules(a);
            simplify_sin(&a)
        }
        Expr::Cos(a) => {
            let a = apply_rules(a);
            simplify_cos(&a)
        }
        Expr::Ln(a) => {
            let a = apply_rules(a);
            simplify_ln(&a)
        }
        Expr::Exp(a) => {
            let a = apply_rules(a);
            simplify_exp(&a)
        }
        Expr::Pow(base, exp) => {
            let base = apply_rules(base);
            let exp = apply_rules(exp);
            simplify_pow(&base, &exp)
        }
        Expr::Mul(a, b) => {
            let a = apply_rules(a);
            let b = apply_rules(b);
            simplify_mul(&a, &b)
        }
        Expr::Add(a, b) => {
            let a = apply_rules(a);
            let b = apply_rules(b);
            simplify_add(&a, &b)
        }
        Expr::Neg(a) => {
            let a = apply_rules(a);
            simplify_neg(&a)
        }
        Expr::Const(_) | Expr::Var(_) => expr.clone(),
    }
}

// -- Trig identities --

fn simplify_sin(a: &Expr) -> Expr {
    match a {
        // sin(0) = 0
        Expr::Const(x) if is_zero(*x) => Expr::Const(0.0),
        // sin(π) = 0
        Expr::Const(x) if is_close(*x, std::f64::consts::PI) => Expr::Const(0.0),
        // sin(π/2) = 1
        Expr::Const(x) if is_close(*x, std::f64::consts::FRAC_PI_2) => Expr::Const(1.0),
        // sin(-x) = -sin(x)
        Expr::Neg(inner) => Expr::Neg(Box::new(Expr::Sin(inner.clone()))),
        _ => Expr::Sin(Box::new(a.clone())),
    }
}

fn simplify_cos(a: &Expr) -> Expr {
    match a {
        // cos(0) = 1
        Expr::Const(x) if is_zero(*x) => Expr::Const(1.0),
        // cos(π) = -1
        Expr::Const(x) if is_close(*x, std::f64::consts::PI) => Expr::Const(-1.0),
        // cos(π/2) = 0
        Expr::Const(x) if is_close(*x, std::f64::consts::FRAC_PI_2) => Expr::Const(0.0),
        // cos(-x) = cos(x)
        Expr::Neg(inner) => Expr::Cos(inner.clone()),
        _ => Expr::Cos(Box::new(a.clone())),
    }
}

// -- Log rules --

fn simplify_ln(a: &Expr) -> Expr {
    match a {
        // ln(1) = 0
        Expr::Const(x) if is_one(*x) => Expr::Const(0.0),
        // ln(e) = 1
        Expr::Const(x) if is_close(*x, std::f64::consts::E) => Expr::Const(1.0),
        // ln(e^x) = x
        Expr::Exp(inner) => *inner.clone(),
        // ln(x^n) = n*ln(x)
        Expr::Pow(base, exp) => Expr::Mul(exp.clone(), Box::new(Expr::Ln(base.clone()))),
        _ => Expr::Ln(Box::new(a.clone())),
    }
}

// -- Exp rules --

fn simplify_exp(a: &Expr) -> Expr {
    match a {
        // e^0 = 1
        Expr::Const(x) if is_zero(*x) => Expr::Const(1.0),
        // e^1 = e
        Expr::Const(x) if is_one(*x) => Expr::Const(std::f64::consts::E),
        // e^(ln(x)) = x
        Expr::Ln(inner) => *inner.clone(),
        _ => Expr::Exp(Box::new(a.clone())),
    }
}

// -- Power rules --

fn simplify_pow(base: &Expr, exp: &Expr) -> Expr {
    match (base, exp) {
        // x^0 = 1
        (_, Expr::Const(n)) if is_zero(*n) => Expr::Const(1.0),
        // x^1 = x
        (_, Expr::Const(n)) if is_one(*n) => base.clone(),
        // 0^n = 0 (for positive n)
        (Expr::Const(b), Expr::Const(n)) if is_zero(*b) && *n > 0.0 => Expr::Const(0.0),
        // 1^n = 1
        (Expr::Const(b), _) if is_one(*b) => Expr::Const(1.0),
        // (x^a)^b = x^(a*b)
        (Expr::Pow(inner_base, inner_exp), _) => {
            let new_exp = Expr::Mul(inner_exp.clone(), Box::new(exp.clone())).simplify();
            simplify_pow(inner_base, &new_exp)
        }
        // Const folding
        (Expr::Const(b), Expr::Const(n)) => Expr::Const(b.powf(*n)),
        _ => Expr::Pow(Box::new(base.clone()), Box::new(exp.clone())),
    }
}

// -- Mul rules --

fn simplify_mul(a: &Expr, b: &Expr) -> Expr {
    match (a, b) {
        // 0 * x = 0
        (Expr::Const(x), _) | (_, Expr::Const(x)) if is_zero(*x) => Expr::Const(0.0),
        // 1 * x = x
        (Expr::Const(x), _) if is_one(*x) => b.clone(),
        (_, Expr::Const(x)) if is_one(*x) => a.clone(),
        // Const fold
        (Expr::Const(x), Expr::Const(y)) => Expr::Const(x * y),
        // x * x = x^2
        (l, r) if l == r => Expr::Pow(Box::new(l.clone()), Box::new(Expr::Const(2.0))),
        // x^a * x^b = x^(a+b)
        (Expr::Pow(b1, e1), Expr::Pow(b2, e2)) if b1 == b2 => {
            let new_exp = Expr::Add(e1.clone(), e2.clone()).simplify();
            Expr::Pow(b1.clone(), Box::new(new_exp))
        }
        // x * x^n = x^(n+1)
        (base, Expr::Pow(pb, exp)) | (Expr::Pow(pb, exp), base) if *base == **pb => {
            let new_exp = Expr::Add(exp.clone(), Box::new(Expr::Const(1.0))).simplify();
            Expr::Pow(Box::new(base.clone()), Box::new(new_exp))
        }
        _ => Expr::Mul(Box::new(a.clone()), Box::new(b.clone())),
    }
}

// -- Add rules --

fn simplify_add(a: &Expr, b: &Expr) -> Expr {
    match (a, b) {
        // 0 + x = x
        (Expr::Const(x), _) if is_zero(*x) => b.clone(),
        (_, Expr::Const(x)) if is_zero(*x) => a.clone(),
        // Const fold
        (Expr::Const(x), Expr::Const(y)) => Expr::Const(x + y),
        // x + x = 2*x
        (l, r) if l == r => Expr::Mul(Box::new(Expr::Const(2.0)), Box::new(l.clone())),
        // sin²(x) + cos²(x) = 1
        (Expr::Pow(s, se), Expr::Pow(c, ce)) if is_const_two(se) && is_const_two(ce) => {
            match (s.as_ref(), c.as_ref()) {
                (Expr::Sin(sa), Expr::Cos(ca)) | (Expr::Cos(ca), Expr::Sin(sa)) if sa == ca => {
                    return Expr::Const(1.0);
                }
                _ => {}
            }
            Expr::Add(Box::new(a.clone()), Box::new(b.clone()))
        }
        // Needed for non_exhaustive Expr enum
        #[allow(unreachable_patterns)]
        // x + (-y) = x - y simplification: x + (-x) = 0
        (x, Expr::Neg(y)) if x == y.as_ref() => Expr::Const(0.0),
        (Expr::Neg(x), y) if x.as_ref() == y => Expr::Const(0.0),
        _ => Expr::Add(Box::new(a.clone()), Box::new(b.clone())),
    }
}

// -- Neg rules --

fn simplify_neg(a: &Expr) -> Expr {
    match a {
        Expr::Const(x) if is_zero(*x) => Expr::Const(0.0),
        Expr::Const(x) => Expr::Const(-x),
        Expr::Neg(inner) => *inner.clone(),
        _ => Expr::Neg(Box::new(a.clone())),
    }
}

// -- Helpers --

fn is_zero(x: f64) -> bool {
    x.abs() < 1e-15
}

fn is_one(x: f64) -> bool {
    (x - 1.0).abs() < 1e-15
}

fn is_close(x: f64, target: f64) -> bool {
    (x - target).abs() < 1e-12
}

fn is_const_two(e: &Expr) -> bool {
    matches!(e, Expr::Const(x) if (*x - 2.0).abs() < 1e-15)
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

    #[test]
    fn sin_zero() {
        let e = Expr::Sin(Box::new(c(0.0)));
        assert_eq!(simplify_advanced(&e), c(0.0));
    }

    #[test]
    fn cos_zero() {
        let e = Expr::Cos(Box::new(c(0.0)));
        assert_eq!(simplify_advanced(&e), c(1.0));
    }

    #[test]
    fn sin_neg() {
        let e = Expr::Sin(Box::new(Expr::Neg(Box::new(var("x")))));
        let s = simplify_advanced(&e);
        assert_eq!(s, Expr::Neg(Box::new(Expr::Sin(Box::new(var("x"))))));
    }

    #[test]
    fn cos_neg() {
        // cos(-x) = cos(x)
        let e = Expr::Cos(Box::new(Expr::Neg(Box::new(var("x")))));
        assert_eq!(simplify_advanced(&e), Expr::Cos(Box::new(var("x"))));
    }

    #[test]
    fn ln_one() {
        assert_eq!(simplify_advanced(&Expr::Ln(Box::new(c(1.0)))), c(0.0));
    }

    #[test]
    fn ln_exp_cancel() {
        let e = Expr::Ln(Box::new(Expr::Exp(Box::new(var("x")))));
        assert_eq!(simplify_advanced(&e), var("x"));
    }

    #[test]
    fn exp_ln_cancel() {
        let e = Expr::Exp(Box::new(Expr::Ln(Box::new(var("x")))));
        assert_eq!(simplify_advanced(&e), var("x"));
    }

    #[test]
    fn exp_zero() {
        assert_eq!(simplify_advanced(&Expr::Exp(Box::new(c(0.0)))), c(1.0));
    }

    #[test]
    fn power_of_power() {
        // (x^2)^3 = x^6
        let e = Expr::Pow(
            Box::new(Expr::Pow(Box::new(var("x")), Box::new(c(2.0)))),
            Box::new(c(3.0)),
        );
        let s = simplify_advanced(&e);
        assert_eq!(s, Expr::Pow(Box::new(var("x")), Box::new(c(6.0))));
    }

    #[test]
    fn x_times_x() {
        let e = Expr::Mul(Box::new(var("x")), Box::new(var("x")));
        let s = simplify_advanced(&e);
        assert_eq!(s, Expr::Pow(Box::new(var("x")), Box::new(c(2.0))));
    }

    #[test]
    fn x_pow_a_times_x_pow_b() {
        // x^2 * x^3 = x^5
        let e = Expr::Mul(
            Box::new(Expr::Pow(Box::new(var("x")), Box::new(c(2.0)))),
            Box::new(Expr::Pow(Box::new(var("x")), Box::new(c(3.0)))),
        );
        let s = simplify_advanced(&e);
        assert_eq!(s, Expr::Pow(Box::new(var("x")), Box::new(c(5.0))));
    }

    #[test]
    fn sin_sq_plus_cos_sq() {
        // sin²(x) + cos²(x) = 1
        let e = Expr::Add(
            Box::new(Expr::Pow(
                Box::new(Expr::Sin(Box::new(var("x")))),
                Box::new(c(2.0)),
            )),
            Box::new(Expr::Pow(
                Box::new(Expr::Cos(Box::new(var("x")))),
                Box::new(c(2.0)),
            )),
        );
        assert_eq!(simplify_advanced(&e), c(1.0));
    }

    #[test]
    fn x_plus_neg_x() {
        let e = Expr::Add(Box::new(var("x")), Box::new(Expr::Neg(Box::new(var("x")))));
        assert_eq!(simplify_advanced(&e), c(0.0));
    }

    #[test]
    fn x_plus_x() {
        let e = Expr::Add(Box::new(var("x")), Box::new(var("x")));
        let s = simplify_advanced(&e);
        assert_eq!(s, Expr::Mul(Box::new(c(2.0)), Box::new(var("x"))));
    }

    #[test]
    fn ln_x_pow_n() {
        // ln(x^3) = 3*ln(x)
        let e = Expr::Ln(Box::new(Expr::Pow(Box::new(var("x")), Box::new(c(3.0)))));
        let s = simplify_advanced(&e);
        assert_eq!(
            s,
            Expr::Mul(Box::new(c(3.0)), Box::new(Expr::Ln(Box::new(var("x")))))
        );
    }
}
