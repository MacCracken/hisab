//! Pattern matching and substitution engine for symbolic expressions.
//!
//! Allows defining expression patterns with wildcards and applying
//! rewrite rules to transform expressions.

use super::Expr;
use std::collections::HashMap;

/// A pattern that can match against an [`Expr`].
///
/// Wildcards capture sub-expressions into named bindings.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Pattern {
    /// Matches any expression and binds it to the given name.
    Wildcard(String),
    /// Matches a specific constant.
    Const(f64),
    /// Matches a specific variable.
    Var(String),
    /// Matches Add(lhs, rhs).
    Add(Box<Pattern>, Box<Pattern>),
    /// Matches Mul(lhs, rhs).
    Mul(Box<Pattern>, Box<Pattern>),
    /// Matches Pow(base, exp).
    Pow(Box<Pattern>, Box<Pattern>),
    /// Matches Neg(inner).
    Neg(Box<Pattern>),
    /// Matches Sin(inner).
    Sin(Box<Pattern>),
    /// Matches Cos(inner).
    Cos(Box<Pattern>),
    /// Matches Exp(inner).
    Exp(Box<Pattern>),
    /// Matches Ln(inner).
    Ln(Box<Pattern>),
    /// Matches any constant (binds the value).
    AnyConst(String),
}

/// A rewrite rule: pattern → template.
#[derive(Debug, Clone)]
pub struct RewriteRule {
    /// Pattern to match.
    pub pattern: Pattern,
    /// Template for the replacement.
    pub template: Pattern,
}

/// Match a pattern against an expression, returning variable bindings on success.
#[must_use]
pub fn match_expr(pattern: &Pattern, expr: &Expr) -> Option<HashMap<String, Expr>> {
    let mut bindings = HashMap::new();
    if match_recursive(pattern, expr, &mut bindings) {
        Some(bindings)
    } else {
        None
    }
}

fn match_recursive(pattern: &Pattern, expr: &Expr, bindings: &mut HashMap<String, Expr>) -> bool {
    match (pattern, expr) {
        (Pattern::Wildcard(name), _) => {
            if let Some(existing) = bindings.get(name) {
                existing == expr
            } else {
                bindings.insert(name.clone(), expr.clone());
                true
            }
        }
        (Pattern::AnyConst(name), Expr::Const(c)) => {
            if let Some(existing) = bindings.get(name) {
                existing == expr
            } else {
                bindings.insert(name.clone(), Expr::Const(*c));
                true
            }
        }
        (Pattern::Const(pc), Expr::Const(ec)) => (pc - ec).abs() < 1e-15,
        (Pattern::Var(pv), Expr::Var(ev)) => pv == ev,
        (Pattern::Add(pa, pb), Expr::Add(ea, eb)) => {
            match_recursive(pa, ea, bindings) && match_recursive(pb, eb, bindings)
        }
        (Pattern::Mul(pa, pb), Expr::Mul(ea, eb)) => {
            match_recursive(pa, ea, bindings) && match_recursive(pb, eb, bindings)
        }
        (Pattern::Pow(pa, pb), Expr::Pow(ea, eb)) => {
            match_recursive(pa, ea, bindings) && match_recursive(pb, eb, bindings)
        }
        (Pattern::Neg(pa), Expr::Neg(ea)) => match_recursive(pa, ea, bindings),
        (Pattern::Sin(pa), Expr::Sin(ea)) => match_recursive(pa, ea, bindings),
        (Pattern::Cos(pa), Expr::Cos(ea)) => match_recursive(pa, ea, bindings),
        (Pattern::Exp(pa), Expr::Exp(ea)) => match_recursive(pa, ea, bindings),
        (Pattern::Ln(pa), Expr::Ln(ea)) => match_recursive(pa, ea, bindings),
        _ => false,
    }
}

/// Instantiate a pattern template with the given bindings to produce an expression.
#[must_use]
pub fn instantiate(template: &Pattern, bindings: &HashMap<String, Expr>) -> Option<Expr> {
    match template {
        Pattern::Wildcard(name) | Pattern::AnyConst(name) => bindings.get(name).cloned(),
        Pattern::Const(c) => Some(Expr::Const(*c)),
        Pattern::Var(v) => Some(Expr::Var(v.clone())),
        Pattern::Add(a, b) => {
            let a = instantiate(a, bindings)?;
            let b = instantiate(b, bindings)?;
            Some(Expr::Add(Box::new(a), Box::new(b)))
        }
        Pattern::Mul(a, b) => {
            let a = instantiate(a, bindings)?;
            let b = instantiate(b, bindings)?;
            Some(Expr::Mul(Box::new(a), Box::new(b)))
        }
        Pattern::Pow(a, b) => {
            let a = instantiate(a, bindings)?;
            let b = instantiate(b, bindings)?;
            Some(Expr::Pow(Box::new(a), Box::new(b)))
        }
        Pattern::Neg(a) => {
            let a = instantiate(a, bindings)?;
            Some(Expr::Neg(Box::new(a)))
        }
        Pattern::Sin(a) => {
            let a = instantiate(a, bindings)?;
            Some(Expr::Sin(Box::new(a)))
        }
        Pattern::Cos(a) => {
            let a = instantiate(a, bindings)?;
            Some(Expr::Cos(Box::new(a)))
        }
        Pattern::Exp(a) => {
            let a = instantiate(a, bindings)?;
            Some(Expr::Exp(Box::new(a)))
        }
        Pattern::Ln(a) => {
            let a = instantiate(a, bindings)?;
            Some(Expr::Ln(Box::new(a)))
        }
    }
}

/// Apply a single rewrite rule to an expression.
///
/// If the pattern matches at the root, returns the rewritten expression.
/// Otherwise returns `None`.
#[must_use]
pub fn apply_rule(rule: &RewriteRule, expr: &Expr) -> Option<Expr> {
    let bindings = match_expr(&rule.pattern, expr)?;
    instantiate(&rule.template, &bindings)
}

/// Apply a rewrite rule recursively to all sub-expressions (bottom-up).
///
/// Applies the rule at each node, starting from the leaves.
#[must_use]
pub fn rewrite(rule: &RewriteRule, expr: &Expr) -> Expr {
    // First rewrite children
    let rewritten = match expr {
        Expr::Const(_) | Expr::Var(_) => expr.clone(),
        Expr::Add(a, b) => Expr::Add(Box::new(rewrite(rule, a)), Box::new(rewrite(rule, b))),
        Expr::Mul(a, b) => Expr::Mul(Box::new(rewrite(rule, a)), Box::new(rewrite(rule, b))),
        Expr::Pow(a, b) => Expr::Pow(Box::new(rewrite(rule, a)), Box::new(rewrite(rule, b))),
        Expr::Neg(a) => Expr::Neg(Box::new(rewrite(rule, a))),
        Expr::Sin(a) => Expr::Sin(Box::new(rewrite(rule, a))),
        Expr::Cos(a) => Expr::Cos(Box::new(rewrite(rule, a))),
        Expr::Exp(a) => Expr::Exp(Box::new(rewrite(rule, a))),
        Expr::Ln(a) => Expr::Ln(Box::new(rewrite(rule, a))),
        #[allow(unreachable_patterns)]
        _ => expr.clone(),
    };

    // Then try to apply the rule at this node
    apply_rule(rule, &rewritten).unwrap_or(rewritten)
}

/// Apply multiple rewrite rules repeatedly until no more changes occur (fixpoint).
///
/// `max_iterations` prevents infinite loops from cyclic rules.
#[must_use]
pub fn rewrite_fixpoint(rules: &[RewriteRule], expr: &Expr, max_iterations: usize) -> Expr {
    let mut current = expr.clone();
    for _ in 0..max_iterations {
        let mut changed = false;
        for rule in rules {
            let next = rewrite(rule, &current);
            if next != current {
                current = next;
                changed = true;
                break; // restart from first rule
            }
        }
        if !changed {
            break;
        }
    }
    current
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
    fn wild(name: &str) -> Pattern {
        Pattern::Wildcard(name.into())
    }

    #[test]
    fn match_wildcard() {
        let bindings = match_expr(&wild("a"), &var("x")).unwrap();
        assert_eq!(bindings["a"], var("x"));
    }

    #[test]
    fn match_const() {
        assert!(match_expr(&Pattern::Const(3.0), &c(3.0)).is_some());
        assert!(match_expr(&Pattern::Const(3.0), &c(4.0)).is_none());
    }

    #[test]
    fn match_var() {
        assert!(match_expr(&Pattern::Var("x".into()), &var("x")).is_some());
        assert!(match_expr(&Pattern::Var("x".into()), &var("y")).is_none());
    }

    #[test]
    fn match_add() {
        let pat = Pattern::Add(Box::new(wild("a")), Box::new(wild("b")));
        let expr = Expr::Add(Box::new(var("x")), Box::new(c(1.0)));
        let bindings = match_expr(&pat, &expr).unwrap();
        assert_eq!(bindings["a"], var("x"));
        assert_eq!(bindings["b"], c(1.0));
    }

    #[test]
    fn match_repeated_wildcard() {
        // Pattern: a + a (both must be the same)
        let pat = Pattern::Add(Box::new(wild("a")), Box::new(wild("a")));
        let good = Expr::Add(Box::new(var("x")), Box::new(var("x")));
        let bad = Expr::Add(Box::new(var("x")), Box::new(var("y")));
        assert!(match_expr(&pat, &good).is_some());
        assert!(match_expr(&pat, &bad).is_none());
    }

    #[test]
    fn match_any_const() {
        let pat = Pattern::AnyConst("c".into());
        assert!(match_expr(&pat, &c(42.0)).is_some());
        assert!(match_expr(&pat, &var("x")).is_none());
    }

    #[test]
    fn instantiate_template() {
        let template = Pattern::Mul(Box::new(Pattern::Const(2.0)), Box::new(wild("a")));
        let mut bindings = HashMap::new();
        bindings.insert("a".into(), var("x"));
        let result = instantiate(&template, &bindings).unwrap();
        assert_eq!(result, Expr::Mul(Box::new(c(2.0)), Box::new(var("x"))));
    }

    #[test]
    fn apply_rule_basic() {
        // Rule: a + a → 2 * a
        let rule = RewriteRule {
            pattern: Pattern::Add(Box::new(wild("a")), Box::new(wild("a"))),
            template: Pattern::Mul(Box::new(Pattern::Const(2.0)), Box::new(wild("a"))),
        };
        let expr = Expr::Add(Box::new(var("x")), Box::new(var("x")));
        let result = apply_rule(&rule, &expr).unwrap();
        assert_eq!(result, Expr::Mul(Box::new(c(2.0)), Box::new(var("x"))));
    }

    #[test]
    fn apply_rule_no_match() {
        let rule = RewriteRule {
            pattern: Pattern::Add(Box::new(wild("a")), Box::new(wild("a"))),
            template: Pattern::Mul(Box::new(Pattern::Const(2.0)), Box::new(wild("a"))),
        };
        let expr = Expr::Add(Box::new(var("x")), Box::new(var("y")));
        assert!(apply_rule(&rule, &expr).is_none());
    }

    #[test]
    fn rewrite_recursive() {
        // Rule: a + a → 2 * a, applied to nested expression
        let rule = RewriteRule {
            pattern: Pattern::Add(Box::new(wild("a")), Box::new(wild("a"))),
            template: Pattern::Mul(Box::new(Pattern::Const(2.0)), Box::new(wild("a"))),
        };
        // (x + x) + 1
        let expr = Expr::Add(
            Box::new(Expr::Add(Box::new(var("x")), Box::new(var("x")))),
            Box::new(c(1.0)),
        );
        let result = rewrite(&rule, &expr);
        // Inner (x + x) → 2*x, outer unchanged
        assert_eq!(
            result,
            Expr::Add(
                Box::new(Expr::Mul(Box::new(c(2.0)), Box::new(var("x")))),
                Box::new(c(1.0)),
            )
        );
    }

    #[test]
    fn rewrite_fixpoint_convergence() {
        // Two rules: a+0 → a, 0+a → a
        let rules = vec![
            RewriteRule {
                pattern: Pattern::Add(Box::new(wild("a")), Box::new(Pattern::Const(0.0))),
                template: wild("a"),
            },
            RewriteRule {
                pattern: Pattern::Add(Box::new(Pattern::Const(0.0)), Box::new(wild("a"))),
                template: wild("a"),
            },
        ];
        let expr = Expr::Add(
            Box::new(Expr::Add(Box::new(c(0.0)), Box::new(var("x")))),
            Box::new(c(0.0)),
        );
        let result = rewrite_fixpoint(&rules, &expr, 10);
        assert_eq!(result, var("x"));
    }

    #[test]
    fn match_sin_pattern() {
        let pat = Pattern::Sin(Box::new(wild("a")));
        let expr = Expr::Sin(Box::new(var("x")));
        let bindings = match_expr(&pat, &expr).unwrap();
        assert_eq!(bindings["a"], var("x"));
    }

    #[test]
    fn match_nested_deep() {
        // Pattern: sin(a * b)
        let pat = Pattern::Sin(Box::new(Pattern::Mul(
            Box::new(wild("a")),
            Box::new(wild("b")),
        )));
        let expr = Expr::Sin(Box::new(Expr::Mul(Box::new(c(2.0)), Box::new(var("x")))));
        let bindings = match_expr(&pat, &expr).unwrap();
        assert_eq!(bindings["a"], c(2.0));
        assert_eq!(bindings["b"], var("x"));
    }
}
