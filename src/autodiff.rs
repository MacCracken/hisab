//! Forward-mode automatic differentiation using dual numbers.
//!
//! A [`Dual`] number carries both a value and its derivative. Arithmetic
//! operations propagate derivatives automatically via the chain rule,
//! computing `f(x)` and `f'(x)` in a single forward pass.

use std::ops;

/// A dual number `a + bε` where `ε² = 0`.
///
/// - `val`: the function value.
/// - `deriv`: the derivative with respect to the seeded variable.
///
/// # Examples
///
/// ```
/// use hisab::autodiff::Dual;
///
/// // Compute f(x) = x² + 2x and f'(x) at x=3
/// let x = Dual::var(3.0);
/// let c = Dual::constant(2.0);
/// let result = x * x + c * x;
/// assert!((result.val - 15.0).abs() < 1e-10); // f(3) = 9 + 6
/// assert!((result.deriv - 8.0).abs() < 1e-10); // f'(3) = 6 + 2
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dual {
    /// Function value.
    pub val: f64,
    /// Derivative value.
    pub deriv: f64,
}

impl Dual {
    /// Create a new dual number.
    #[must_use]
    #[inline]
    pub const fn new(val: f64, deriv: f64) -> Self {
        Self { val, deriv }
    }

    /// Create a dual number representing a variable (derivative = 1).
    #[must_use]
    #[inline]
    pub const fn var(val: f64) -> Self {
        Self { val, deriv: 1.0 }
    }

    /// Create a dual number representing a constant (derivative = 0).
    #[must_use]
    #[inline]
    pub const fn constant(val: f64) -> Self {
        Self { val, deriv: 0.0 }
    }

    /// Sine.
    #[must_use]
    #[inline]
    pub fn sin(self) -> Self {
        Self {
            val: self.val.sin(),
            deriv: self.deriv * self.val.cos(),
        }
    }

    /// Cosine.
    #[must_use]
    #[inline]
    pub fn cos(self) -> Self {
        Self {
            val: self.val.cos(),
            deriv: -self.deriv * self.val.sin(),
        }
    }

    /// Exponential.
    #[must_use]
    #[inline]
    pub fn exp(self) -> Self {
        let e = self.val.exp();
        Self {
            val: e,
            deriv: self.deriv * e,
        }
    }

    /// Natural logarithm.
    #[must_use]
    #[inline]
    pub fn ln(self) -> Self {
        Self {
            val: self.val.ln(),
            deriv: self.deriv / self.val,
        }
    }

    /// Square root.
    #[must_use]
    #[inline]
    pub fn sqrt(self) -> Self {
        let s = self.val.sqrt();
        Self {
            val: s,
            deriv: self.deriv / (2.0 * s),
        }
    }

    /// Power (self^n).
    #[must_use]
    #[inline]
    pub fn powf(self, n: f64) -> Self {
        Self {
            val: self.val.powf(n),
            deriv: self.deriv * n * self.val.powf(n - 1.0),
        }
    }

    /// Absolute value.
    #[must_use]
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            val: self.val.abs(),
            deriv: if self.val >= 0.0 {
                self.deriv
            } else {
                -self.deriv
            },
        }
    }

    /// Tangent.
    #[must_use]
    #[inline]
    pub fn tan(self) -> Self {
        let c = self.val.cos();
        Self {
            val: self.val.tan(),
            deriv: self.deriv / (c * c),
        }
    }
}

impl ops::Add for Dual {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            val: self.val + rhs.val,
            deriv: self.deriv + rhs.deriv,
        }
    }
}

impl ops::Sub for Dual {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            val: self.val - rhs.val,
            deriv: self.deriv - rhs.deriv,
        }
    }
}

impl ops::Mul for Dual {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            val: self.val * rhs.val,
            deriv: self.val * rhs.deriv + self.deriv * rhs.val,
        }
    }
}

impl ops::Div for Dual {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        Self {
            val: self.val / rhs.val,
            deriv: (self.deriv * rhs.val - self.val * rhs.deriv) / (rhs.val * rhs.val),
        }
    }
}

impl ops::Neg for Dual {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            val: -self.val,
            deriv: -self.deriv,
        }
    }
}

impl ops::Add<f64> for Dual {
    type Output = Self;
    #[inline]
    fn add(self, rhs: f64) -> Self {
        Self {
            val: self.val + rhs,
            deriv: self.deriv,
        }
    }
}

impl ops::Sub<f64> for Dual {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: f64) -> Self {
        Self {
            val: self.val - rhs,
            deriv: self.deriv,
        }
    }
}

impl ops::Mul<f64> for Dual {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self {
            val: self.val * rhs,
            deriv: self.deriv * rhs,
        }
    }
}

impl ops::Div<f64> for Dual {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self {
            val: self.val / rhs,
            deriv: self.deriv / rhs,
        }
    }
}

impl ops::Add<Dual> for f64 {
    type Output = Dual;
    #[inline]
    fn add(self, rhs: Dual) -> Dual {
        Dual {
            val: self + rhs.val,
            deriv: rhs.deriv,
        }
    }
}

impl ops::Mul<Dual> for f64 {
    type Output = Dual;
    #[inline]
    fn mul(self, rhs: Dual) -> Dual {
        Dual {
            val: self * rhs.val,
            deriv: self * rhs.deriv,
        }
    }
}

impl From<f64> for Dual {
    #[inline]
    fn from(val: f64) -> Self {
        Self::constant(val)
    }
}

impl std::fmt::Display for Dual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}ε", self.val, self.deriv)
    }
}

// ---------------------------------------------------------------------------
// Reverse-mode automatic differentiation (tape-based)
// ---------------------------------------------------------------------------

/// Operation recorded on the computation tape.
#[derive(Debug, Clone, Copy)]
enum TapeOp {
    Const,
    Add(usize, usize),
    Mul(usize, usize),
    Sub(usize, usize),
    Div(usize, usize),
    Neg(usize),
    Sin(usize),
    Cos(usize),
    Exp(usize),
    Ln(usize),
    Pow(usize, f64),
}

/// A computation tape for reverse-mode automatic differentiation.
///
/// Records operations during a forward pass, then computes gradients
/// via backpropagation in a single reverse pass.
#[derive(Debug)]
pub struct Tape {
    ops: Vec<TapeOp>,
    values: Vec<f64>,
}

/// A variable on a tape — tracks its index for gradient computation.
#[derive(Debug, Clone, Copy)]
pub struct Var {
    index: usize,
    val: f64,
}

impl Tape {
    /// Create a new empty tape.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Create a variable (input) on the tape.
    pub fn var(&mut self, val: f64) -> Var {
        let index = self.ops.len();
        self.ops.push(TapeOp::Const);
        self.values.push(val);
        Var { index, val }
    }

    /// Create a constant (non-differentiated) on the tape.
    pub fn constant(&mut self, val: f64) -> Var {
        let index = self.ops.len();
        self.ops.push(TapeOp::Const);
        self.values.push(val);
        Var { index, val }
    }

    fn push(&mut self, op: TapeOp, val: f64) -> Var {
        let index = self.ops.len();
        self.ops.push(op);
        self.values.push(val);
        Var { index, val }
    }

    /// Add two variables.
    pub fn add(&mut self, a: Var, b: Var) -> Var {
        self.push(TapeOp::Add(a.index, b.index), a.val + b.val)
    }

    /// Subtract two variables.
    pub fn sub(&mut self, a: Var, b: Var) -> Var {
        self.push(TapeOp::Sub(a.index, b.index), a.val - b.val)
    }

    /// Multiply two variables.
    pub fn mul(&mut self, a: Var, b: Var) -> Var {
        self.push(TapeOp::Mul(a.index, b.index), a.val * b.val)
    }

    /// Divide two variables.
    pub fn div(&mut self, a: Var, b: Var) -> Var {
        self.push(TapeOp::Div(a.index, b.index), a.val / b.val)
    }

    /// Negate a variable.
    pub fn neg(&mut self, a: Var) -> Var {
        self.push(TapeOp::Neg(a.index), -a.val)
    }

    /// Sine.
    pub fn sin(&mut self, a: Var) -> Var {
        self.push(TapeOp::Sin(a.index), a.val.sin())
    }

    /// Cosine.
    pub fn cos(&mut self, a: Var) -> Var {
        self.push(TapeOp::Cos(a.index), a.val.cos())
    }

    /// Exponential.
    pub fn exp(&mut self, a: Var) -> Var {
        self.push(TapeOp::Exp(a.index), a.val.exp())
    }

    /// Natural logarithm.
    pub fn ln(&mut self, a: Var) -> Var {
        self.push(TapeOp::Ln(a.index), a.val.ln())
    }

    /// Power with constant exponent.
    pub fn powf(&mut self, a: Var, n: f64) -> Var {
        self.push(TapeOp::Pow(a.index, n), a.val.powf(n))
    }

    /// Run backpropagation from the given output variable.
    ///
    /// Returns gradients with respect to all tape variables.
    #[must_use]
    pub fn backward(&self, output: Var) -> Vec<f64> {
        let n = self.ops.len();
        let mut grads = vec![0.0; n];
        grads[output.index] = 1.0;

        for i in (0..n).rev() {
            let g = grads[i];
            if g == 0.0 {
                continue;
            }
            match self.ops[i] {
                TapeOp::Const => {}
                TapeOp::Add(a, b) => {
                    grads[a] += g;
                    grads[b] += g;
                }
                TapeOp::Sub(a, b) => {
                    grads[a] += g;
                    grads[b] -= g;
                }
                TapeOp::Mul(a, b) => {
                    grads[a] += g * self.values[b];
                    grads[b] += g * self.values[a];
                }
                TapeOp::Div(a, b) => {
                    grads[a] += g / self.values[b];
                    grads[b] -= g * self.values[a] / (self.values[b] * self.values[b]);
                }
                TapeOp::Neg(a) => {
                    grads[a] -= g;
                }
                TapeOp::Sin(a) => {
                    grads[a] += g * self.values[a].cos();
                }
                TapeOp::Cos(a) => {
                    grads[a] -= g * self.values[a].sin();
                }
                TapeOp::Exp(a) => {
                    grads[a] += g * self.values[i]; // e^x * grad
                }
                TapeOp::Ln(a) => {
                    grads[a] += g / self.values[a];
                }
                TapeOp::Pow(a, n) => {
                    grads[a] += g * n * self.values[a].powf(n - 1.0);
                }
            }
        }

        grads
    }
}

impl Default for Tape {
    fn default() -> Self {
        Self::new()
    }
}

/// Compute the gradient of a scalar function using reverse-mode AD.
///
/// - `f`: function that takes a tape and input variables, returns output variable.
/// - `x`: input point.
///
/// Returns the gradient vector `[df/dx0, df/dx1, ...]`.
#[must_use]
pub fn reverse_gradient(f: impl Fn(&mut Tape, &[Var]) -> Var, x: &[f64]) -> Vec<f64> {
    let mut tape = Tape::new();
    let vars: Vec<Var> = x.iter().map(|&v| tape.var(v)).collect();
    let output = f(&mut tape, &vars);
    let grads = tape.backward(output);
    vars.iter().map(|v| grads[v.index]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn dual_arithmetic() {
        let x = Dual::var(3.0);
        let c = Dual::constant(2.0);
        let r = x * x + c * x;
        // f(x) = x² + 2x, f(3) = 15, f'(3) = 2*3+2 = 8
        assert!(approx(r.val, 15.0));
        assert!(approx(r.deriv, 8.0));
    }

    #[test]
    fn dual_division() {
        let x = Dual::var(4.0);
        let one = Dual::constant(1.0);
        let r = one / x; // 1/x, d/dx = -1/x²
        assert!(approx(r.val, 0.25));
        assert!(approx(r.deriv, -1.0 / 16.0));
    }

    #[test]
    fn dual_sin_cos() {
        let x = Dual::var(0.0);
        let s = x.sin();
        assert!(approx(s.val, 0.0));
        assert!(approx(s.deriv, 1.0)); // cos(0) = 1

        let c = x.cos();
        assert!(approx(c.val, 1.0));
        assert!(approx(c.deriv, 0.0)); // -sin(0) = 0
    }

    #[test]
    fn dual_exp_ln() {
        let x = Dual::var(1.0);
        let e = x.exp();
        assert!(approx(e.val, std::f64::consts::E));
        assert!(approx(e.deriv, std::f64::consts::E)); // d/dx e^x = e^x

        let l = x.ln();
        assert!(approx(l.val, 0.0));
        assert!(approx(l.deriv, 1.0)); // d/dx ln(x) = 1/x
    }

    #[test]
    fn dual_sqrt() {
        let x = Dual::var(4.0);
        let s = x.sqrt();
        assert!(approx(s.val, 2.0));
        assert!(approx(s.deriv, 0.25)); // 1/(2√4)
    }

    #[test]
    fn dual_powf() {
        let x = Dual::var(2.0);
        let p = x.powf(3.0); // x³
        assert!(approx(p.val, 8.0));
        assert!(approx(p.deriv, 12.0)); // 3x² = 12
    }

    #[test]
    fn dual_chain_rule() {
        // f(x) = sin(x²), f'(x) = 2x·cos(x²)
        let x = Dual::var(1.0);
        let r = (x * x).sin();
        assert!(approx(r.val, 1.0_f64.sin()));
        assert!(approx(r.deriv, 2.0 * 1.0_f64.cos()));
    }

    #[test]
    fn dual_neg() {
        let x = Dual::var(3.0);
        let r = -x;
        assert!(approx(r.val, -3.0));
        assert!(approx(r.deriv, -1.0));
    }

    #[test]
    fn dual_abs() {
        let x = Dual::var(-3.0);
        let r = x.abs();
        assert!(approx(r.val, 3.0));
        assert!(approx(r.deriv, -1.0));
    }

    #[test]
    fn dual_tan() {
        let x = Dual::var(0.0);
        let r = x.tan();
        assert!(approx(r.val, 0.0));
        assert!(approx(r.deriv, 1.0)); // sec²(0) = 1
    }

    #[test]
    fn dual_display() {
        let d = Dual::new(1.0, 2.0);
        assert_eq!(format!("{d}"), "1+2ε");
    }

    #[test]
    fn dual_from_f64() {
        let d: Dual = 5.0.into();
        assert!(approx(d.val, 5.0));
        assert!(approx(d.deriv, 0.0));
    }

    #[test]
    fn dual_scalar_ops() {
        let x = Dual::var(3.0);
        let r = x + 1.0;
        assert!(approx(r.val, 4.0));
        assert!(approx(r.deriv, 1.0));

        let r2 = x * 2.0;
        assert!(approx(r2.val, 6.0));
        assert!(approx(r2.deriv, 2.0));
    }

    #[test]
    fn dual_sub_scalar() {
        let x = Dual::var(5.0);
        let r = x - 3.0;
        assert!(approx(r.val, 2.0));
        assert!(approx(r.deriv, 1.0));
    }

    #[test]
    fn dual_div_scalar() {
        let x = Dual::var(6.0);
        let r = x / 3.0;
        assert!(approx(r.val, 2.0));
        assert!(approx(r.deriv, 1.0 / 3.0));
    }

    #[test]
    fn dual_reverse_scalar_ops() {
        let x = Dual::var(3.0);
        let r = 2.0 * x;
        assert!(approx(r.val, 6.0));
        assert!(approx(r.deriv, 2.0));

        let r2 = 10.0 + x;
        assert!(approx(r2.val, 13.0));
        assert!(approx(r2.deriv, 1.0));
    }

    // --- Reverse-mode AD tests ---

    #[test]
    fn reverse_simple_product() {
        // f(x, y) = x * y → df/dx = y, df/dy = x
        let grad = reverse_gradient(|tape, vars| tape.mul(vars[0], vars[1]), &[3.0, 5.0]);
        assert!(approx(grad[0], 5.0)); // df/dx = y
        assert!(approx(grad[1], 3.0)); // df/dy = x
    }

    #[test]
    fn reverse_sum() {
        // f(x, y) = x + y → df/dx = 1, df/dy = 1
        let grad = reverse_gradient(|tape, vars| tape.add(vars[0], vars[1]), &[3.0, 5.0]);
        assert!(approx(grad[0], 1.0));
        assert!(approx(grad[1], 1.0));
    }

    #[test]
    fn reverse_chain_rule() {
        // f(x) = sin(x²), df/dx = 2x*cos(x²)
        let grad = reverse_gradient(
            |tape, vars| {
                let x2 = tape.mul(vars[0], vars[0]);
                tape.sin(x2)
            },
            &[1.0],
        );
        let expected = 2.0 * 1.0_f64.cos(); // 2*1*cos(1)
        assert!(approx(grad[0], expected));
    }

    #[test]
    fn reverse_matches_forward() {
        // f(x) = x³ + 2x → df/dx = 3x² + 2
        let x_val = 2.0;

        // Forward mode
        let x = Dual::var(x_val);
        let fwd = x * x * x + Dual::constant(2.0) * x;

        // Reverse mode
        let grad = reverse_gradient(
            |tape, vars| {
                let x2 = tape.mul(vars[0], vars[0]);
                let x3 = tape.mul(x2, vars[0]);
                let two = tape.constant(2.0);
                let two_x = tape.mul(two, vars[0]);
                tape.add(x3, two_x)
            },
            &[x_val],
        );

        assert!(
            (fwd.deriv - grad[0]).abs() < 1e-8,
            "forward={} vs reverse={}",
            fwd.deriv,
            grad[0]
        );
    }

    #[test]
    fn reverse_exp_ln() {
        // f(x) = exp(ln(x)) = x → df/dx = 1
        let grad = reverse_gradient(
            |tape, vars| {
                let ln_x = tape.ln(vars[0]);
                tape.exp(ln_x)
            },
            &[3.0],
        );
        assert!(approx(grad[0], 1.0));
    }

    #[test]
    fn reverse_power() {
        // f(x) = x^3 → df/dx = 3x²
        let grad = reverse_gradient(|tape, vars| tape.powf(vars[0], 3.0), &[2.0]);
        assert!(approx(grad[0], 12.0)); // 3*4
    }
}
