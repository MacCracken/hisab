//! Numerical methods: root finding and linear solvers.
//!
//! Provides Newton-Raphson, bisection, and Gaussian elimination with partial pivoting.

use crate::GanitError;

/// Newton-Raphson root finding.
///
/// Finds `x` such that `f(x) ≈ 0`.
///
/// - `f`: the function whose root we seek.
/// - `df`: the derivative of `f`.
/// - `x0`: initial guess.
/// - `tol`: convergence tolerance (stops when `|f(x)| < tol`).
/// - `max_iter`: maximum iterations.
pub fn newton_raphson(
    f: impl Fn(f64) -> f64,
    df: impl Fn(f64) -> f64,
    x0: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, GanitError> {
    let mut x = x0;
    for _ in 0..max_iter {
        let fx = f(x);
        if fx.abs() < tol {
            return Ok(x);
        }
        let dfx = df(x);
        if dfx.abs() < 1e-15 {
            return Err(GanitError::InvalidInput(
                "derivative is zero".to_string(),
            ));
        }
        x -= fx / dfx;
    }
    Err(GanitError::NoConvergence(max_iter))
}

/// Bisection root finding.
///
/// Finds `x` in `[a, b]` such that `f(x) ≈ 0`. Requires `f(a)` and `f(b)`
/// to have opposite signs (intermediate value theorem).
pub fn bisection(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
    max_iter: usize,
) -> Result<f64, GanitError> {
    let mut lo = a;
    let mut hi = b;
    let f_lo = f(lo);
    let f_hi = f(hi);

    if f_lo * f_hi > 0.0 {
        return Err(GanitError::InvalidInput(
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
pub fn gaussian_elimination(matrix: &mut [Vec<f64>]) -> Result<Vec<f64>, GanitError> {
    let n = matrix.len();
    if n == 0 {
        return Err(GanitError::InvalidInput("empty matrix".to_string()));
    }
    for row in matrix.iter() {
        if row.len() != n + 1 {
            return Err(GanitError::InvalidInput(format!(
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

        if max_val < 1e-12 {
            return Err(GanitError::SingularPivot);
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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    #[test]
    fn newton_sqrt2() {
        let root = newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, 1.5, 1e-10, 100).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-9);
    }

    #[test]
    fn newton_cube_root_27() {
        let root =
            newton_raphson(|x| x * x * x - 27.0, |x| 3.0 * x * x, 2.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, 3.0));
    }

    #[test]
    fn newton_no_convergence() {
        let result = newton_raphson(|x| x * x + 1.0, |x| 2.0 * x, 1.0, 1e-15, 5);
        assert!(result.is_err());
    }

    #[test]
    fn bisection_sqrt2() {
        let root = bisection(|x| x * x - 2.0, 1.0, 2.0, 1e-10, 100).unwrap();
        assert!((root - std::f64::consts::SQRT_2).abs() < 1e-9);
    }

    #[test]
    fn bisection_cubic_root() {
        let root = bisection(|x| x * x * x - 8.0, 1.0, 3.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, 2.0));
    }

    #[test]
    fn bisection_same_sign_error() {
        let result = bisection(|x| x * x + 1.0, 1.0, 2.0, 1e-10, 100);
        assert!(result.is_err());
    }

    #[test]
    fn bisection_sin_root() {
        let root = bisection(f64::sin, 3.0, 4.0, 1e-10, 100).unwrap();
        assert!((root - std::f64::consts::PI).abs() < 1e-9);
    }

    #[test]
    fn gaussian_2x2() {
        let mut matrix = vec![vec![2.0, 1.0, 5.0], vec![1.0, 3.0, 10.0]];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 3.0));
    }

    #[test]
    fn gaussian_3x3() {
        let mut matrix = vec![
            vec![1.0, 1.0, 1.0, 6.0],
            vec![2.0, 1.0, -1.0, 1.0],
            vec![1.0, -1.0, 1.0, 2.0],
        ];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 2.0));
        assert!(approx_eq(x[2], 3.0));
    }

    #[test]
    fn gaussian_singular_matrix() {
        let mut matrix = vec![vec![1.0, 2.0, 3.0], vec![2.0, 4.0, 6.0]];
        let result = gaussian_elimination(&mut matrix);
        assert!(result.is_err());
    }

    #[test]
    fn gaussian_empty_matrix() {
        let mut matrix: Vec<Vec<f64>> = vec![];
        let result = gaussian_elimination(&mut matrix);
        assert!(result.is_err());
    }

    #[test]
    fn newton_zero_derivative_error() {
        let result = newton_raphson(|x| x * x + 1.0, |_| 0.0, 2.0, 1e-10, 100);
        assert!(result.is_err());
    }

    #[test]
    fn error_display() {
        let e = GanitError::NoConvergence(50);
        assert_eq!(e.to_string(), "no convergence after 50 iterations");
        let e = GanitError::SingularPivot;
        assert!(e.to_string().contains("singular"));
    }

    #[test]
    fn error_display_invalid_input() {
        let e = GanitError::InvalidInput("bad data".to_string());
        assert_eq!(e.to_string(), "invalid input: bad data");
    }

    #[test]
    fn newton_linear_root() {
        let root = newton_raphson(|x| 2.0 * x - 6.0, |_| 2.0, 0.0, 1e-10, 10).unwrap();
        assert!(approx_eq(root, 3.0));
    }

    #[test]
    fn newton_sin_root_near_zero() {
        let root = newton_raphson(f64::sin, f64::cos, 3.0, 1e-12, 50).unwrap();
        assert!((root - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn bisection_linear() {
        let root = bisection(|x| x - 5.0, 0.0, 10.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, 5.0));
    }

    #[test]
    fn bisection_negative_interval() {
        let root = bisection(|x| x + 3.0, -5.0, 0.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, -3.0));
    }

    #[test]
    fn bisection_swaps_when_f_lo_positive() {
        let root = bisection(|x| x - 1.0, 2.0, 0.0, 1e-10, 100).unwrap();
        assert!(approx_eq(root, 1.0));
    }

    #[test]
    fn gaussian_1x1() {
        let mut matrix = vec![vec![3.0, 9.0]];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 3.0));
    }

    #[test]
    fn gaussian_needs_pivoting() {
        let mut matrix = vec![vec![0.0, 1.0, 3.0], vec![1.0, 1.0, 5.0]];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 2.0));
        assert!(approx_eq(x[1], 3.0));
    }

    #[test]
    fn gaussian_wrong_column_count() {
        let mut matrix = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
        let result = gaussian_elimination(&mut matrix);
        assert!(result.is_err());
    }

    #[test]
    fn gaussian_4x4() {
        let mut matrix = vec![
            vec![1.0, 1.0, 1.0, 1.0, 4.0],
            vec![2.0, 1.0, 1.0, 1.0, 5.0],
            vec![1.0, 3.0, 1.0, 1.0, 6.0],
            vec![1.0, 1.0, 1.0, 4.0, 7.0],
        ];
        let x = gaussian_elimination(&mut matrix).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 1.0));
        assert!(approx_eq(x[2], 1.0));
        assert!(approx_eq(x[3], 1.0));
    }

    #[test]
    fn newton_converges_exactly_at_root() {
        let root = newton_raphson(|x| x - 5.0, |_| 1.0, 5.0, 1e-10, 1).unwrap();
        assert!(approx_eq(root, 5.0));
    }

    #[test]
    fn bisection_exact_root_at_midpoint() {
        let root = bisection(|x| x, -1.0, 1.0, 1e-10, 1).unwrap();
        assert!(approx_eq(root, 0.0));
    }
}
