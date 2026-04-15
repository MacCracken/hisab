use crate::HisabError;

/// Newton-Raphson root finding.
///
/// Finds `x` such that `f(x) ≈ 0`.
///
/// - `f`: the function whose root we seek.
/// - `df`: the derivative of `f`.
/// - `x0`: initial guess.
/// - `tol`: convergence tolerance (stops when `|f(x)| < tol`).
///
/// # Examples
///
/// ```
/// use hisab::num::newton_raphson;
///
/// // Find √2: solve x² - 2 = 0
/// let root = newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, 1.5, 1e-10, 100).unwrap();
/// assert!((root - std::f64::consts::SQRT_2).abs() < 1e-9);
/// ```
/// - `max_iter`: maximum iterations.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the derivative is zero at any iterate.
/// Returns [`HisabError::NoConvergence`] if `max_iter` iterations are exhausted.
#[must_use = "contains the computed root or an error"]
pub fn newton_raphson(
    f: impl Fn(f64) -> f64,
    df: impl Fn(f64) -> f64,
    x0: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, HisabError> {
    let mut x = x0;
    for _ in 0..max_iter {
        let fx = f(x);
        if fx.abs() < tol {
            return Ok(x);
        }
        let dfx = df(x);
        if dfx.abs() < crate::EPSILON_F64 {
            return Err(HisabError::InvalidInput("derivative is zero".to_string()));
        }
        x -= fx / dfx;
    }
    Err(HisabError::NoConvergence(max_iter))
}

/// Bisection root finding.
///
/// Finds `x` in `[a, b]` such that `f(x) ≈ 0`. Requires `f(a)` and `f(b)`
/// to have opposite signs (intermediate value theorem).
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `f(a)` and `f(b)` have the same sign.
/// Returns [`HisabError::NoConvergence`] if `max_iter` iterations are exhausted.
#[must_use = "contains the computed root or an error"]
pub fn bisection(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, HisabError> {
    let mut lo = a;
    let mut hi = b;
    let f_lo = f(lo);
    let f_hi = f(hi);

    if f_lo * f_hi > 0.0 {
        return Err(HisabError::InvalidInput(
            "f(a) and f(b) must have opposite signs".to_string(),
        ));
    }

    // Ensure f(lo) < 0
    if f_lo > 0.0 {
        std::mem::swap(&mut lo, &mut hi);
    }

    for _ in 0..max_iter {
        let mid = (lo + hi) * 0.5;
        let f_mid = f(mid);

        if f_mid.abs() < tol || (hi - lo).abs() < tol {
            return Ok(mid);
        }

        if f_mid < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    Ok((lo + hi) * 0.5)
}

/// Gaussian elimination with partial pivoting.
///
/// Solves `A * x = b` where `matrix` is the augmented matrix `[A | b]`
/// with dimensions `n x (n+1)`.
///
/// The matrix is modified in place. Returns the solution vector `x`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty.
/// Returns [`HisabError::SingularPivot`] if a zero pivot is encountered.
#[must_use = "contains the solution vector or an error"]
pub fn gaussian_elimination(matrix: &mut [Vec<f64>]) -> Result<Vec<f64>, HisabError> {
    let n = matrix.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".to_string()));
    }
    for row in matrix.iter() {
        if row.len() != n + 1 {
            return Err(HisabError::InvalidInput(format!(
                "expected {} columns, got {}",
                n + 1,
                row.len()
            )));
        }
    }

    // Forward elimination with partial pivoting
    for col in 0..n {
        // Find pivot
        let mut max_row = col;
        let mut max_val = matrix[col][col].abs();
        for (row, matrix_row) in matrix.iter().enumerate().skip(col + 1) {
            let val = matrix_row[col].abs();
            if val > max_val {
                max_val = val;
                max_row = row;
            }
        }

        if max_val < crate::EPSILON_F64 {
            return Err(HisabError::SingularPivot);
        }

        // Swap rows
        if max_row != col {
            matrix.swap(col, max_row);
        }

        // Eliminate below pivot row
        let pivot = matrix[col][col];
        #[allow(clippy::needless_range_loop)]
        for row in (col + 1)..n {
            let factor = matrix[row][col] / pivot;
            for j in col..=n {
                let val = matrix[col][j];
                matrix[row][j] -= factor * val;
            }
        }
    }

    // Back substitution
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = matrix[i][n];
        for j in (i + 1)..n {
            sum -= matrix[i][j] * x[j];
        }
        x[i] = sum / matrix[i][i];
    }

    Ok(x)
}
