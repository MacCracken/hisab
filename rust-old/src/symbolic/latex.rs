//! LaTeX rendering for symbolic expressions.

use super::Expr;
use std::fmt::Write;

/// Render an expression as a LaTeX string.
///
/// # Examples
///
/// ```
/// use hisab::symbolic::Expr;
/// use hisab::symbolic::to_latex;
///
/// let e = Expr::Pow(
///     Box::new(Expr::Var("x".into())),
///     Box::new(Expr::Const(2.0)),
/// );
/// assert_eq!(to_latex(&e), "x^{2}");
/// ```
#[must_use]
pub fn to_latex(expr: &Expr) -> String {
    let mut buf = String::new();
    write_latex(expr, &mut buf, false);
    buf
}

/// Write LaTeX for `expr` into `buf`.
/// `paren` hints that the caller wants parentheses around binary ops.
fn write_latex(expr: &Expr, buf: &mut String, paren: bool) {
    match expr {
        Expr::Const(c) => {
            if *c < 0.0 && !paren {
                // Negative constants: wrap in braces for safety
                let _ = write!(buf, "{{{c}}}");
            } else {
                format_const(*c, buf);
            }
        }
        Expr::Var(name) => {
            // Multi-char variables get mathrm
            if name.len() > 1 {
                let _ = write!(buf, "\\mathrm{{{name}}}");
            } else {
                buf.push_str(name);
            }
        }
        Expr::Add(a, b) => {
            if paren {
                buf.push_str("\\left(");
            }
            write_latex(a, buf, false);
            // Check if b is a negation to render as subtraction
            if let Expr::Neg(inner) = b.as_ref() {
                buf.push_str(" - ");
                write_latex(inner, buf, true);
            } else {
                buf.push_str(" + ");
                write_latex(b, buf, false);
            }
            if paren {
                buf.push_str("\\right)");
            }
        }
        Expr::Mul(a, b) => {
            if paren {
                buf.push_str("\\left(");
            }
            let needs_cdot = needs_explicit_multiply(a, b);
            write_latex(a, buf, true);
            if needs_cdot {
                buf.push_str(" \\cdot ");
            } else {
                buf.push(' ');
            }
            write_latex(b, buf, true);
            if paren {
                buf.push_str("\\right)");
            }
        }
        Expr::Pow(base, exp) => {
            // Special case: x^(-1) → \frac{1}{base}
            if let Expr::Const(n) = exp.as_ref() {
                if (*n + 1.0).abs() < 1e-15 {
                    buf.push_str("\\frac{1}{");
                    write_latex(base, buf, false);
                    buf.push('}');
                    return;
                }
                // x^(1/2) → \sqrt{x}
                if (*n - 0.5).abs() < 1e-15 {
                    buf.push_str("\\sqrt{");
                    write_latex(base, buf, false);
                    buf.push('}');
                    return;
                }
            }
            write_latex(base, buf, true);
            buf.push_str("^{");
            write_latex(exp, buf, false);
            buf.push('}');
        }
        Expr::Neg(a) => {
            buf.push('-');
            write_latex(a, buf, true);
        }
        Expr::Sin(a) => {
            buf.push_str("\\sin\\left(");
            write_latex(a, buf, false);
            buf.push_str("\\right)");
        }
        Expr::Cos(a) => {
            buf.push_str("\\cos\\left(");
            write_latex(a, buf, false);
            buf.push_str("\\right)");
        }
        Expr::Exp(a) => {
            buf.push_str("e^{");
            write_latex(a, buf, false);
            buf.push('}');
        }
        Expr::Ln(a) => {
            buf.push_str("\\ln\\left(");
            write_latex(a, buf, false);
            buf.push_str("\\right)");
        }
        // Non-exhaustive fallback
        #[allow(unreachable_patterns)]
        _ => {
            let _ = write!(buf, "\\text{{?}}");
        }
    }
}

/// Format a constant nicely (avoid trailing .0 for integers).
fn format_const(c: f64, buf: &mut String) {
    if c == c.floor() && c.abs() < 1e15 {
        let _ = write!(buf, "{}", c as i64);
    } else {
        let _ = write!(buf, "{c}");
    }
}

/// Decide whether we need `\cdot` between two factors.
fn needs_explicit_multiply(a: &Expr, b: &Expr) -> bool {
    // Number * Number needs cdot
    matches!((a, b), (Expr::Const(_), Expr::Const(_)))
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
    fn latex_const() {
        assert_eq!(to_latex(&c(42.0)), "42");
        assert_eq!(to_latex(&c(3.15)), "3.15");
    }

    #[test]
    fn latex_var() {
        assert_eq!(to_latex(&var("x")), "x");
        assert_eq!(to_latex(&var("theta")), "\\mathrm{theta}");
    }

    #[test]
    fn latex_add() {
        let e = Expr::Add(Box::new(var("x")), Box::new(c(1.0)));
        assert_eq!(to_latex(&e), "x + 1");
    }

    #[test]
    fn latex_subtraction() {
        let e = Expr::Add(Box::new(var("x")), Box::new(Expr::Neg(Box::new(c(1.0)))));
        assert_eq!(to_latex(&e), "x - 1");
    }

    #[test]
    fn latex_mul() {
        let e = Expr::Mul(Box::new(c(2.0)), Box::new(var("x")));
        assert_eq!(to_latex(&e), "2 x");
    }

    #[test]
    fn latex_mul_consts() {
        let e = Expr::Mul(Box::new(c(2.0)), Box::new(c(3.0)));
        assert_eq!(to_latex(&e), "2 \\cdot 3");
    }

    #[test]
    fn latex_pow() {
        let e = Expr::Pow(Box::new(var("x")), Box::new(c(2.0)));
        assert_eq!(to_latex(&e), "x^{2}");
    }

    #[test]
    fn latex_sqrt() {
        let e = Expr::Pow(Box::new(var("x")), Box::new(c(0.5)));
        assert_eq!(to_latex(&e), "\\sqrt{x}");
    }

    #[test]
    fn latex_reciprocal() {
        let e = Expr::Pow(Box::new(var("x")), Box::new(c(-1.0)));
        assert_eq!(to_latex(&e), "\\frac{1}{x}");
    }

    #[test]
    fn latex_sin() {
        let e = Expr::Sin(Box::new(var("x")));
        assert_eq!(to_latex(&e), "\\sin\\left(x\\right)");
    }

    #[test]
    fn latex_cos() {
        let e = Expr::Cos(Box::new(var("x")));
        assert_eq!(to_latex(&e), "\\cos\\left(x\\right)");
    }

    #[test]
    fn latex_exp() {
        let e = Expr::Exp(Box::new(var("x")));
        assert_eq!(to_latex(&e), "e^{x}");
    }

    #[test]
    fn latex_ln() {
        let e = Expr::Ln(Box::new(var("x")));
        assert_eq!(to_latex(&e), "\\ln\\left(x\\right)");
    }

    #[test]
    fn latex_neg() {
        let e = Expr::Neg(Box::new(var("x")));
        assert_eq!(to_latex(&e), "-x");
    }

    #[test]
    fn latex_complex_expr() {
        // 2x^2 + sin(x)
        let e = Expr::Add(
            Box::new(Expr::Mul(
                Box::new(c(2.0)),
                Box::new(Expr::Pow(Box::new(var("x")), Box::new(c(2.0)))),
            )),
            Box::new(Expr::Sin(Box::new(var("x")))),
        );
        let latex = to_latex(&e);
        assert!(latex.contains("x^{2}"));
        assert!(latex.contains("\\sin"));
    }
}
