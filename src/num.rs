//! Numerical methods: root finding, linear solvers, and matrix decompositions.
//!
//! Provides Newton-Raphson, bisection, Gaussian elimination, LU/Cholesky/QR
//! decompositions, and least-squares fitting.

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

// ---------------------------------------------------------------------------
// LU decomposition
// ---------------------------------------------------------------------------

/// LU decomposition with partial pivoting (Doolittle form).
///
/// Decomposes an `n x n` matrix `A` into `P * A = L * U` where:
/// - `L` is lower-triangular with unit diagonal
/// - `U` is upper-triangular
/// - `P` is a permutation (returned as a pivot index vector)
///
/// Returns `(lu, pivot)` where `lu` stores both L (below diagonal) and U
/// (on and above diagonal) in a single matrix.
#[allow(clippy::needless_range_loop)]
pub fn lu_decompose(a: &[Vec<f64>]) -> Result<(Vec<Vec<f64>>, Vec<usize>), GanitError> {
    let n = a.len();
    if n == 0 {
        return Err(GanitError::InvalidInput("empty matrix".to_string()));
    }
    for row in a {
        if row.len() != n {
            return Err(GanitError::InvalidInput(format!(
                "expected square {}x{}, got row length {}",
                n, n, row.len()
            )));
        }
    }

    let mut lu: Vec<Vec<f64>> = a.to_vec();
    let mut pivot: Vec<usize> = (0..n).collect();

    for col in 0..n {
        // Find pivot
        let mut max_row = col;
        let mut max_val = lu[col][col].abs();
        for row in (col + 1)..n {
            let val = lu[row][col].abs();
            if val > max_val {
                max_val = val;
                max_row = row;
            }
        }
        if max_val < 1e-12 {
            return Err(GanitError::SingularPivot);
        }
        if max_row != col {
            lu.swap(col, max_row);
            pivot.swap(col, max_row);
        }

        let diag = lu[col][col];
        for row in (col + 1)..n {
            lu[row][col] /= diag;
            let factor = lu[row][col];
            for j in (col + 1)..n {
                let val = lu[col][j];
                lu[row][j] -= factor * val;
            }
        }
    }

    Ok((lu, pivot))
}

/// Solve `A * x = b` using a pre-computed LU decomposition.
#[inline]
#[allow(clippy::needless_range_loop)]
pub fn lu_solve(lu: &[Vec<f64>], pivot: &[usize], b: &[f64]) -> Result<Vec<f64>, GanitError> {
    let n = lu.len();
    if b.len() != n {
        return Err(GanitError::InvalidInput(format!(
            "b length {} != matrix size {}",
            b.len(), n
        )));
    }

    // Apply permutation
    let mut x: Vec<f64> = pivot.iter().map(|&i| b[i]).collect();

    // Forward substitution (L * y = Pb)
    for i in 1..n {
        let mut sum = x[i];
        for j in 0..i {
            sum -= lu[i][j] * x[j];
        }
        x[i] = sum;
    }

    // Back substitution (U * x = y)
    for i in (0..n).rev() {
        let mut sum = x[i];
        for j in (i + 1)..n {
            sum -= lu[i][j] * x[j];
        }
        x[i] = sum / lu[i][i];
    }

    Ok(x)
}

// ---------------------------------------------------------------------------
// Cholesky decomposition
// ---------------------------------------------------------------------------

/// Cholesky decomposition for symmetric positive-definite matrices.
///
/// Decomposes `A = L * L^T` where `L` is lower-triangular.
/// Returns `L`. Fails if `A` is not positive-definite.
///
/// Only the lower triangle of `A` is read. The caller must ensure `A` is symmetric.
#[allow(clippy::needless_range_loop)]
pub fn cholesky(a: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, GanitError> {
    let n = a.len();
    if n == 0 {
        return Err(GanitError::InvalidInput("empty matrix".to_string()));
    }
    for row in a {
        if row.len() != n {
            return Err(GanitError::InvalidInput(format!(
                "expected square {}x{}, got row length {}",
                n, n, row.len()
            )));
        }
    }

    let mut l = vec![vec![0.0; n]; n];

    for i in 0..n {
        for j in 0..=i {
            let mut sum = a[i][j];
            for k in 0..j {
                sum -= l[i][k] * l[j][k];
            }
            if i == j {
                if sum <= 0.0 {
                    return Err(GanitError::InvalidInput(
                        "matrix is not positive-definite".to_string(),
                    ));
                }
                l[i][j] = sum.sqrt();
            } else {
                l[i][j] = sum / l[j][j];
            }
        }
    }

    Ok(l)
}

/// Solve `A * x = b` using a pre-computed Cholesky factor `L` (where `A = L * L^T`).
#[inline]
#[allow(clippy::needless_range_loop)]
pub fn cholesky_solve(l: &[Vec<f64>], b: &[f64]) -> Result<Vec<f64>, GanitError> {
    let n = l.len();
    if b.len() != n {
        return Err(GanitError::InvalidInput(format!(
            "b length {} != matrix size {}",
            b.len(), n
        )));
    }

    // Forward substitution: L * y = b
    let mut y = vec![0.0; n];
    for i in 0..n {
        let mut sum = b[i];
        for j in 0..i {
            sum -= l[i][j] * y[j];
        }
        y[i] = sum / l[i][i];
    }

    // Back substitution: L^T * x = y
    let mut x = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = y[i];
        for j in (i + 1)..n {
            sum -= l[j][i] * x[j]; // L^T[i][j] = L[j][i]
        }
        x[i] = sum / l[i][i];
    }

    Ok(x)
}

// ---------------------------------------------------------------------------
// QR decomposition (modified Gram-Schmidt)
// ---------------------------------------------------------------------------

/// QR decomposition using modified Gram-Schmidt orthogonalization.
///
/// Decomposes an `m x n` matrix `A` (m >= n) into `A = Q * R` where:
/// - `Q` is `m x n` with orthonormal columns
/// - `R` is `n x n` upper-triangular
///
/// Input is column-major: `a[j]` is the j-th column vector.
/// Output `r` uses the same layout: `R[i][j]` is stored as `r[j][i]`.
#[allow(clippy::type_complexity, clippy::needless_range_loop)]
pub fn qr_decompose(a: &[Vec<f64>]) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>), GanitError> {
    let n = a.len(); // number of columns
    if n == 0 {
        return Err(GanitError::InvalidInput("empty matrix".to_string()));
    }
    let m = a[0].len(); // number of rows
    if m < n {
        return Err(GanitError::InvalidInput(
            "QR requires m >= n (more rows than columns)".to_string(),
        ));
    }

    let mut q: Vec<Vec<f64>> = a.to_vec();
    let mut r = vec![vec![0.0; n]; n];

    for j in 0..n {
        // Modified Gram-Schmidt: orthogonalize q[j] against all previous q[i]
        for i in 0..j {
            let dot: f64 = (0..m).map(|k| q[i][k] * q[j][k]).sum();
            r[j][i] = dot; // R[i][j] stored as r[j][i]
            for k in 0..m {
                q[j][k] -= dot * q[i][k];
            }
        }
        // Normalize
        let norm: f64 = (0..m).map(|k| q[j][k] * q[j][k]).sum::<f64>().sqrt();
        if norm < 1e-12 {
            return Err(GanitError::InvalidInput(
                "columns are linearly dependent".to_string(),
            ));
        }
        r[j][j] = norm; // R[j][j]
        for k in 0..m {
            q[j][k] /= norm;
        }
    }

    Ok((q, r))
}

// ---------------------------------------------------------------------------
// Least squares fitting
// ---------------------------------------------------------------------------

/// Fit a polynomial of degree `degree` to the given `(x, y)` data points
/// using least squares (via QR decomposition).
///
/// Returns coefficients `[a0, a1, a2, ...]` where `y ≈ a0 + a1*x + a2*x^2 + ...`.
#[allow(clippy::needless_range_loop)]
pub fn least_squares_poly(x: &[f64], y: &[f64], degree: usize) -> Result<Vec<f64>, GanitError> {
    let m = x.len();
    if m != y.len() || m == 0 {
        return Err(GanitError::InvalidInput("x and y must have equal non-zero length".to_string()));
    }
    let n = degree + 1;
    if m < n {
        return Err(GanitError::InvalidInput(
            "need at least degree+1 data points".to_string(),
        ));
    }

    // Build Vandermonde matrix (column-major for QR)
    let mut cols: Vec<Vec<f64>> = Vec::with_capacity(n);
    for j in 0..n {
        let col: Vec<f64> = x.iter().map(|&xi| xi.powi(j as i32)).collect();
        cols.push(col);
    }

    // QR decompose the Vandermonde matrix
    let (q, r) = qr_decompose(&cols)?;

    // Compute Q^T * y
    let mut qty = vec![0.0; n];
    for j in 0..n {
        qty[j] = (0..m).map(|k| q[j][k] * y[k]).sum();
    }

    // Back substitution: R * coeffs = Q^T * y
    // r is stored as r[col][row], so R[i][j] = r[j][i]
    let mut coeffs = vec![0.0; n];
    for i in (0..n).rev() {
        let mut sum = qty[i];
        for j in (i + 1)..n {
            sum -= r[j][i] * coeffs[j];
        }
        coeffs[i] = sum / r[i][i];
    }

    Ok(coeffs)
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

    // --- V0.4a: LU decomposition ---

    #[test]
    fn lu_decompose_2x2() {
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let (lu, _pivot) = lu_decompose(&a).unwrap();
        // U[0][0] should be 2, U[0][1] should be 1
        assert!(approx_eq(lu[0][0], 2.0));
        assert!(approx_eq(lu[0][1], 1.0));
    }

    #[test]
    fn lu_solve_2x2() {
        // 2x + y = 5, x + 3y = 10 => x=1, y=3
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let b = [5.0, 10.0];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        let x = lu_solve(&lu, &pivot, &b).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 3.0));
    }

    #[test]
    fn lu_solve_3x3() {
        // x+y+z=6, 2x+y-z=1, x-y+z=2 => x=1, y=2, z=3
        let a = vec![
            vec![1.0, 1.0, 1.0],
            vec![2.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0],
        ];
        let b = [6.0, 1.0, 2.0];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        let x = lu_solve(&lu, &pivot, &b).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 2.0));
        assert!(approx_eq(x[2], 3.0));
    }

    #[test]
    fn lu_singular() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert!(lu_decompose(&a).is_err());
    }

    #[test]
    fn lu_needs_pivoting() {
        // First pivot is zero
        let a = vec![vec![0.0, 1.0], vec![1.0, 1.0]];
        let b = [3.0, 5.0];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        let x = lu_solve(&lu, &pivot, &b).unwrap();
        assert!(approx_eq(x[0], 2.0));
        assert!(approx_eq(x[1], 3.0));
    }

    // --- V0.4a: Cholesky ---

    #[test]
    fn cholesky_2x2() {
        // A = [[4, 2], [2, 3]] is SPD
        let a = vec![vec![4.0, 2.0], vec![2.0, 3.0]];
        let l = cholesky(&a).unwrap();
        assert!(approx_eq(l[0][0], 2.0));
        assert!(approx_eq(l[1][0], 1.0));
        assert!(approx_eq(l[1][1], (2.0f64).sqrt()));
    }

    #[test]
    fn cholesky_solve_2x2() {
        let a = vec![vec![4.0, 2.0], vec![2.0, 3.0]];
        let b = [8.0, 7.0];
        let l = cholesky(&a).unwrap();
        let x = cholesky_solve(&l, &b).unwrap();
        // Verify A*x = b
        let r0 = 4.0 * x[0] + 2.0 * x[1];
        let r1 = 2.0 * x[0] + 3.0 * x[1];
        assert!(approx_eq(r0, 8.0));
        assert!(approx_eq(r1, 7.0));
    }

    #[test]
    fn cholesky_3x3_identity() {
        let a = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]];
        let l = cholesky(&a).unwrap();
        // L of identity is identity
        assert!(approx_eq(l[0][0], 1.0));
        assert!(approx_eq(l[1][1], 1.0));
        assert!(approx_eq(l[2][2], 1.0));
        assert!(approx_eq(l[1][0], 0.0));
        assert!(approx_eq(l[2][0], 0.0));
        assert!(approx_eq(l[2][1], 0.0));
    }

    #[test]
    fn cholesky_not_positive_definite() {
        // Negative diagonal -> not PD
        let a = vec![vec![-1.0, 0.0], vec![0.0, 1.0]];
        assert!(cholesky(&a).is_err());
    }

    // --- V0.4a: QR decomposition ---

    #[test]
    fn qr_orthogonality() {
        // Verify Q columns are orthonormal
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]]; // 2 cols, 3 rows
        let (q, _r) = qr_decompose(&a).unwrap();
        // q[0] dot q[1] should be ~0
        let dot: f64 = (0..3).map(|k| q[0][k] * q[1][k]).sum();
        assert!(dot.abs() < 1e-10);
        // Each column should have unit norm
        for col in &q {
            let norm: f64 = col.iter().map(|x| x * x).sum::<f64>().sqrt();
            assert!(approx_eq(norm, 1.0));
        }
    }

    #[test]
    fn qr_reconstruct() {
        // Verify A = Q * R
        let a = vec![vec![1.0, 0.0, 1.0], vec![1.0, 1.0, 0.0]]; // 2 cols, 3 rows
        let (q, r) = qr_decompose(&a).unwrap();
        // Reconstruct A[col][row] = sum_k Q[k][row] * R[col][k]
        for j in 0..2 {
            for i in 0..3 {
                let mut sum = 0.0;
                for k in 0..2 {
                    sum += q[k][i] * r[j][k];
                }
                assert!((sum - a[j][i]).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn qr_dependent_columns() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]]; // Linearly dependent
        assert!(qr_decompose(&a).is_err());
    }

    // --- V0.4a: Least squares ---

    #[test]
    fn least_squares_linear_exact() {
        // Points exactly on y = 2x + 1
        let x = [0.0, 1.0, 2.0, 3.0];
        let y = [1.0, 3.0, 5.0, 7.0];
        let coeffs = least_squares_poly(&x, &y, 1).unwrap();
        assert!(approx_eq(coeffs[0], 1.0)); // intercept
        assert!(approx_eq(coeffs[1], 2.0)); // slope
    }

    #[test]
    fn least_squares_quadratic() {
        // Points on y = x^2
        let x = [0.0, 1.0, 2.0, 3.0, 4.0];
        let y: Vec<f64> = x.iter().map(|&xi| xi * xi).collect();
        let coeffs = least_squares_poly(&x, &y, 2).unwrap();
        assert!(approx_eq(coeffs[0], 0.0)); // a0
        assert!(approx_eq(coeffs[1], 0.0)); // a1
        assert!(approx_eq(coeffs[2], 1.0)); // a2
    }

    #[test]
    fn least_squares_constant() {
        // All y=5 => constant fit
        let x = [1.0, 2.0, 3.0];
        let y = [5.0, 5.0, 5.0];
        let coeffs = least_squares_poly(&x, &y, 0).unwrap();
        assert!(approx_eq(coeffs[0], 5.0));
    }

    #[test]
    fn least_squares_noisy_linear() {
        // Noisy data around y = 3x + 2
        let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.1, 4.9, 8.1, 10.9, 14.1, 16.9];
        let coeffs = least_squares_poly(&x, &y, 1).unwrap();
        // Should be close to a0≈2, a1≈3
        assert!((coeffs[0] - 2.0).abs() < 0.5);
        assert!((coeffs[1] - 3.0).abs() < 0.2);
    }

    #[test]
    fn least_squares_insufficient_points() {
        let x = [1.0];
        let y = [5.0];
        assert!(least_squares_poly(&x, &y, 2).is_err());
    }

    // --- Audit tests ---

    #[test]
    fn lu_solve_wrong_b_length() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        assert!(lu_solve(&lu, &pivot, &[1.0]).is_err());
    }

    #[test]
    fn lu_non_square() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert!(lu_decompose(&a).is_err());
    }

    #[test]
    fn cholesky_solve_wrong_b_length() {
        let a = vec![vec![4.0, 2.0], vec![2.0, 3.0]];
        let l = cholesky(&a).unwrap();
        assert!(cholesky_solve(&l, &[1.0, 2.0, 3.0]).is_err());
    }

    #[test]
    fn cholesky_non_square() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert!(cholesky(&a).is_err());
    }

    #[test]
    fn cholesky_solve_3x3_verify() {
        // 3x3 SPD system: solve and verify A*x = b
        let a = vec![
            vec![4.0, 2.0, 1.0],
            vec![2.0, 5.0, 2.0],
            vec![1.0, 2.0, 6.0],
        ];
        let b = [1.0, 2.0, 3.0];
        let l = cholesky(&a).unwrap();
        let x = cholesky_solve(&l, &b).unwrap();
        // Verify: A * x ≈ b
        for i in 0..3 {
            let mut row_sum = 0.0;
            for j in 0..3 {
                row_sum += a[i][j] * x[j];
            }
            assert!(approx_eq(row_sum, b[i]));
        }
    }

    #[test]
    fn qr_square_matrix() {
        // 3x3 square matrix
        let a = vec![
            vec![1.0, 4.0, 7.0],
            vec![2.0, 5.0, 8.0],
            vec![3.0, 6.0, 10.0], // not 9, to avoid singular
        ];
        let (q, _r) = qr_decompose(&a).unwrap();
        // Verify Q^T * Q = I (columns orthonormal)
        for i in 0..3 {
            for j in 0..3 {
                let dot: f64 = (0..3).map(|k| q[i][k] * q[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((dot - expected).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn least_squares_mismatched_lengths() {
        let x = [1.0, 2.0, 3.0];
        let y = [1.0, 2.0];
        assert!(least_squares_poly(&x, &y, 1).is_err());
    }

    #[test]
    fn least_squares_empty() {
        let x: [f64; 0] = [];
        let y: [f64; 0] = [];
        assert!(least_squares_poly(&x, &y, 0).is_err());
    }

    #[test]
    fn lu_multiple_rhs() {
        // Solve same system with two different RHS vectors
        let a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let (lu, pivot) = lu_decompose(&a).unwrap();
        let x1 = lu_solve(&lu, &pivot, &[5.0, 10.0]).unwrap();
        let x2 = lu_solve(&lu, &pivot, &[3.0, 7.0]).unwrap();
        assert!(approx_eq(x1[0], 1.0));
        assert!(approx_eq(x1[1], 3.0));
        // 2x+y=3, x+3y=7 => x=0.4, y=2.2
        assert!(approx_eq(x2[0], 0.4));
        assert!(approx_eq(x2[1], 2.2));
    }
}
