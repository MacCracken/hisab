use super::*;

// ---------------------------------------------------------------------------

/// Partial derivative of a multivariate function with respect to variable `var`.
///
/// Uses central difference: `∂f/∂x_var ≈ (f(x + h·e_var) - f(x - h·e_var)) / (2h)`.
///
/// - `f`: function from ℝⁿ → ℝ.
/// - `x`: point at which to evaluate.
/// - `var`: index of the variable to differentiate with respect to.
/// - `h`: step size.
///
/// # Errors
///
/// Returns [`HisabError::OutOfRange`] if `var >= x.len()`.
#[must_use = "returns the computed partial derivative or an error"]
#[inline]
pub fn partial_derivative(
    f: impl Fn(&[f64]) -> f64,
    x: &[f64],
    var: usize,
    h: f64,
) -> Result<f64, HisabError> {
    if var >= x.len() {
        return Err(HisabError::OutOfRange(format!(
            "var index {var} >= dimension {}",
            x.len()
        )));
    }
    let mut x_plus = x.to_vec();
    let mut x_minus = x.to_vec();
    x_plus[var] += h;
    x_minus[var] -= h;
    Ok((f(&x_plus) - f(&x_minus)) / (2.0 * h))
}

/// Gradient of a scalar function f: ℝⁿ → ℝ.
///
/// Returns the vector of partial derivatives `[∂f/∂x₀, ∂f/∂x₁, ...]`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `x` is empty.
#[must_use = "returns the gradient vector or an error"]
pub fn gradient(f: impl Fn(&[f64]) -> f64, x: &[f64], h: f64) -> Result<Vec<f64>, HisabError> {
    let n = x.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty input".into()));
    }
    let mut grad = Vec::with_capacity(n);
    let mut x_buf = x.to_vec();
    for i in 0..n {
        x_buf[i] = x[i] + h;
        let f_plus = f(&x_buf);
        x_buf[i] = x[i] - h;
        let f_minus = f(&x_buf);
        x_buf[i] = x[i]; // restore
        grad.push((f_plus - f_minus) / (2.0 * h));
    }
    Ok(grad)
}

/// Jacobian matrix of a vector function f: ℝⁿ → ℝᵐ.
///
/// Returns an `m × n` matrix (row-major) where `J[i][j] = ∂fᵢ/∂xⱼ`.
///
/// - `fs`: vector of scalar functions, each ℝⁿ → ℝ.
/// - `x`: point at which to evaluate.
/// - `h`: step size for finite differences.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `fs` or `x` is empty.
#[must_use = "returns the Jacobian matrix or an error"]
#[allow(clippy::type_complexity)]
pub fn jacobian(
    fs: &[&dyn Fn(&[f64]) -> f64],
    x: &[f64],
    h: f64,
) -> Result<Vec<Vec<f64>>, HisabError> {
    let m = fs.len();
    let n = x.len();
    if m == 0 || n == 0 {
        return Err(HisabError::InvalidInput("empty input".into()));
    }

    let mut jac = vec![vec![0.0; n]; m];
    let mut x_buf = x.to_vec();

    for j in 0..n {
        x_buf[j] = x[j] + h;
        let f_plus: Vec<f64> = fs.iter().map(|fi| fi(&x_buf)).collect();
        x_buf[j] = x[j] - h;
        let f_minus: Vec<f64> = fs.iter().map(|fi| fi(&x_buf)).collect();
        x_buf[j] = x[j]; // restore
        let inv_2h = 1.0 / (2.0 * h);
        for i in 0..m {
            jac[i][j] = (f_plus[i] - f_minus[i]) * inv_2h;
        }
    }

    Ok(jac)
}

/// Hessian matrix of a scalar function f: ℝⁿ → ℝ.
///
/// Returns an `n × n` symmetric matrix where `H[i][j] = ∂²f/∂xᵢ∂xⱼ`.
///
/// Uses second-order central differences.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `x` is empty.
#[must_use = "returns the Hessian matrix or an error"]
pub fn hessian(f: impl Fn(&[f64]) -> f64, x: &[f64], h: f64) -> Result<Vec<Vec<f64>>, HisabError> {
    let n = x.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty input".into()));
    }

    let mut hess = vec![vec![0.0; n]; n];
    let mut x_buf = x.to_vec();
    let f0 = f(x);
    let h2 = h * h;

    // Diagonal: ∂²f/∂xᵢ² ≈ (f(x+heᵢ) - 2f(x) + f(x-heᵢ)) / h²
    for i in 0..n {
        x_buf[i] = x[i] + h;
        let fp = f(&x_buf);
        x_buf[i] = x[i] - h;
        let fm = f(&x_buf);
        x_buf[i] = x[i];
        hess[i][i] = (fp - 2.0 * f0 + fm) / h2;
    }

    // Off-diagonal: ∂²f/∂xᵢ∂xⱼ ≈ (f(x+heᵢ+heⱼ) - f(x+heᵢ-heⱼ) - f(x-heᵢ+heⱼ) + f(x-heᵢ-heⱼ)) / (4h²)
    let inv_4h2 = 1.0 / (4.0 * h2);
    for i in 0..n {
        for j in (i + 1)..n {
            x_buf[i] = x[i] + h;
            x_buf[j] = x[j] + h;
            let fpp = f(&x_buf);
            x_buf[j] = x[j] - h;
            let fpm = f(&x_buf);
            x_buf[i] = x[i] - h;
            let fmm = f(&x_buf);
            x_buf[j] = x[j] + h;
            let fmp = f(&x_buf);

            // Restore
            x_buf[i] = x[i];
            x_buf[j] = x[j];

            let val = (fpp - fpm - fmp + fmm) * inv_4h2;
            hess[i][j] = val;
            hess[j][i] = val;
        }
    }

    Ok(hess)
}


