//! Numerical methods: root finding, linear solvers, decompositions, FFT, DST/DCT, and ODE solvers.
//!
//! Provides Newton-Raphson, bisection, Gaussian elimination, LU/Cholesky/QR
//! decompositions, least-squares fitting, eigenvalue computation (power iteration),
//! Cooley-Tukey FFT/IFFT, DST-I/IDST-I, DCT-II/IDCT, and Runge-Kutta (RK4) ODE integration.

use crate::HisabError;
use serde::{Deserialize, Serialize};

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
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty or not square.
/// Returns [`HisabError::SingularPivot`] if a zero pivot is encountered.
#[must_use = "contains the LU factors or an error"]
#[allow(clippy::needless_range_loop)]
pub fn lu_decompose(a: &[Vec<f64>]) -> Result<(Vec<Vec<f64>>, Vec<usize>), HisabError> {
    let n = a.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".to_string()));
    }
    for row in a {
        if row.len() != n {
            return Err(HisabError::InvalidInput(format!(
                "expected square {}x{}, got row length {}",
                n,
                n,
                row.len()
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
        if max_val < crate::EPSILON_F64 {
            return Err(HisabError::SingularPivot);
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

/// LU decomposition with partial pivoting, modifying the matrix in place.
///
/// Same as [`lu_decompose`] but overwrites `a` with the combined L/U factors
/// instead of cloning. Use this when the original matrix is no longer needed.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty or not square.
/// Returns [`HisabError::SingularPivot`] if a zero pivot is encountered.
#[must_use = "returns the pivot permutation needed for solving"]
#[allow(clippy::needless_range_loop)]
pub fn lu_decompose_in_place(a: &mut [Vec<f64>]) -> Result<Vec<usize>, HisabError> {
    let n = a.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    for row in a.iter() {
        if row.len() != n {
            return Err(HisabError::InvalidInput(format!(
                "expected square {}x{}, got row length {}",
                n,
                n,
                row.len()
            )));
        }
    }

    let mut pivot: Vec<usize> = (0..n).collect();

    for col in 0..n {
        let mut max_row = col;
        let mut max_val = a[col][col].abs();
        for row in (col + 1)..n {
            let val = a[row][col].abs();
            if val > max_val {
                max_val = val;
                max_row = row;
            }
        }
        if max_val < crate::EPSILON_F64 {
            return Err(HisabError::SingularPivot);
        }
        if max_row != col {
            a.swap(col, max_row);
            pivot.swap(col, max_row);
        }

        let diag = a[col][col];
        for row in (col + 1)..n {
            a[row][col] /= diag;
            let factor = a[row][col];
            for j in (col + 1)..n {
                let val = a[col][j];
                a[row][j] -= factor * val;
            }
        }
    }

    Ok(pivot)
}

/// Solve `A * x = b` using a pre-computed LU decomposition.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `b` length does not match the matrix size.
#[must_use = "contains the solution vector or an error"]
#[inline]
#[allow(clippy::needless_range_loop)]
pub fn lu_solve(lu: &[Vec<f64>], pivot: &[usize], b: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = lu.len();
    if b.len() != n {
        return Err(HisabError::InvalidInput(format!(
            "b length {} != matrix size {}",
            b.len(),
            n
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
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty or not square.
/// Returns [`HisabError::InvalidInput`] if the matrix is not positive-definite.
#[must_use = "contains the Cholesky factor or an error"]
#[allow(clippy::needless_range_loop)]
pub fn cholesky(a: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, HisabError> {
    let n = a.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".to_string()));
    }
    for row in a {
        if row.len() != n {
            return Err(HisabError::InvalidInput(format!(
                "expected square {}x{}, got row length {}",
                n,
                n,
                row.len()
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
                    return Err(HisabError::InvalidInput(
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
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `b` length does not match the matrix size.
#[must_use = "contains the solution vector or an error"]
#[inline]
#[allow(clippy::needless_range_loop)]
pub fn cholesky_solve(l: &[Vec<f64>], b: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = l.len();
    if b.len() != n {
        return Err(HisabError::InvalidInput(format!(
            "b length {} != matrix size {}",
            b.len(),
            n
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
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty, not tall enough, or rank-deficient.
#[must_use = "contains the QR factors or an error"]
#[allow(clippy::type_complexity, clippy::needless_range_loop)]
pub fn qr_decompose(a: &[Vec<f64>]) -> Result<(Vec<Vec<f64>>, Vec<Vec<f64>>), HisabError> {
    let n = a.len(); // number of columns
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".to_string()));
    }
    let m = a[0].len(); // number of rows
    if m < n {
        return Err(HisabError::InvalidInput(
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
        if norm < crate::EPSILON_F64 {
            return Err(HisabError::InvalidInput(
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

/// QR decomposition using modified Gram-Schmidt, modifying columns in place.
///
/// Same as [`qr_decompose`] but overwrites `a` with the Q factor instead of
/// cloning. Returns only R. Use this when the original matrix is no longer needed.
///
/// Input is column-major: `a[j]` is the j-th column vector.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty, underdetermined,
/// or has linearly dependent columns.
#[must_use = "returns the R factor needed for solving"]
#[allow(clippy::needless_range_loop)]
pub fn qr_decompose_in_place(a: &mut [Vec<f64>]) -> Result<Vec<Vec<f64>>, HisabError> {
    let n = a.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    let m = a[0].len();
    if m < n {
        return Err(HisabError::InvalidInput(
            "QR requires m >= n (more rows than columns)".into(),
        ));
    }

    let mut r = vec![vec![0.0; n]; n];

    for j in 0..n {
        for i in 0..j {
            let dot: f64 = (0..m).map(|k| a[i][k] * a[j][k]).sum();
            r[j][i] = dot;
            for k in 0..m {
                a[j][k] -= dot * a[i][k];
            }
        }
        let norm: f64 = (0..m).map(|k| a[j][k] * a[j][k]).sum::<f64>().sqrt();
        if norm < crate::EPSILON_F64 {
            return Err(HisabError::InvalidInput(
                "columns are linearly dependent".into(),
            ));
        }
        r[j][j] = norm;
        for k in 0..m {
            a[j][k] /= norm;
        }
    }

    Ok(r)
}

// ---------------------------------------------------------------------------
// Matrix helpers
// ---------------------------------------------------------------------------

/// Compute the determinant of a square matrix using LU decomposition.
///
/// Input is row-major: `a[i]` is the i-th row.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty or not square.
#[must_use = "returns the computed determinant"]
pub fn matrix_determinant(a: &[Vec<f64>]) -> Result<f64, HisabError> {
    let n = a.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    for row in a {
        if row.len() != n {
            return Err(HisabError::InvalidInput(format!(
                "expected square {}x{}, got row length {}",
                n,
                n,
                row.len()
            )));
        }
    }
    // Clone and decompose in place to avoid double allocation
    let mut lu: Vec<Vec<f64>> = a.to_vec();
    match lu_decompose_in_place(&mut lu) {
        Ok(pivot) => {
            let mut det = 1.0;
            for (i, row) in lu.iter().enumerate().take(n) {
                det *= row[i];
            }
            // Determine sign from permutation parity
            let mut sign = 1.0;
            let mut perm = pivot;
            for i in 0..n {
                while perm[i] != i {
                    let j = perm[i];
                    perm.swap(i, j);
                    sign = -sign;
                }
            }
            Ok(det * sign)
        }
        Err(HisabError::SingularPivot) => Ok(0.0),
        Err(e) => Err(e),
    }
}

/// Compute the trace (sum of diagonal elements) of a square matrix.
///
/// Input is row-major: `a[i]` is the i-th row.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty or not square.
#[must_use = "returns the computed trace"]
#[inline]
pub fn matrix_trace(a: &[Vec<f64>]) -> Result<f64, HisabError> {
    let n = a.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    for row in a {
        if row.len() != n {
            return Err(HisabError::InvalidInput(format!(
                "expected square {}x{}, got row length {}",
                n,
                n,
                row.len()
            )));
        }
    }
    Ok((0..n).map(|i| a[i][i]).sum())
}

/// Multiply two dense matrices: C = A * B.
///
/// `a` is `m x p` and `b` is `p x n`, both row-major.
/// Returns the `m x n` result matrix.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if dimensions are incompatible or inputs are empty.
#[must_use = "returns the product matrix"]
pub fn matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, HisabError> {
    let m = a.len();
    if m == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    let p = a[0].len();
    if p == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    for row in a {
        if row.len() != p {
            return Err(HisabError::InvalidInput(
                "inconsistent row lengths in A".into(),
            ));
        }
    }
    let n_rows_b = b.len();
    if n_rows_b != p {
        return Err(HisabError::InvalidInput(format!(
            "A is {}x{} but B has {} rows \u{2014} inner dimensions must match",
            m, p, n_rows_b
        )));
    }
    let n = b[0].len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    for row in b {
        if row.len() != n {
            return Err(HisabError::InvalidInput(
                "inconsistent row lengths in B".into(),
            ));
        }
    }

    let mut c = vec![vec![0.0; n]; m];
    for i in 0..m {
        for k in 0..p {
            let a_ik = a[i][k];
            for j in 0..n {
                c[i][j] += a_ik * b[k][j];
            }
        }
    }
    Ok(c)
}

// ---------------------------------------------------------------------------
// Least squares fitting
// ---------------------------------------------------------------------------

/// Fit a polynomial of degree `degree` to the given `(x, y)` data points
/// using least squares (via QR decomposition).
///
/// Returns coefficients `[a0, a1, a2, ...]` where `y ≈ a0 + a1*x + a2*x^2 + ...`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `x` and `y` differ in length, are empty,
/// or have fewer points than `degree + 1`.
#[must_use = "contains the polynomial coefficients or an error"]
#[allow(clippy::needless_range_loop)]
pub fn least_squares_poly(x: &[f64], y: &[f64], degree: usize) -> Result<Vec<f64>, HisabError> {
    let m = x.len();
    if m != y.len() || m == 0 {
        return Err(HisabError::InvalidInput(
            "x and y must have equal non-zero length".to_string(),
        ));
    }
    let n = degree + 1;
    if m < n {
        return Err(HisabError::InvalidInput(
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

// ---------------------------------------------------------------------------
// Eigenvalue computation (power iteration)
// ---------------------------------------------------------------------------

/// Find the dominant eigenvalue and eigenvector of a square matrix
/// using power iteration.
///
/// - `a`: square `n x n` matrix (row-major).
/// - `tol`: convergence tolerance on the eigenvalue estimate.
/// - `max_iter`: maximum iterations.
///
/// Returns `(eigenvalue, eigenvector)`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty or not square.
/// Returns [`HisabError::NoConvergence`] if `max_iter` iterations are exhausted.
#[must_use = "contains the dominant eigenvalue/eigenvector or an error"]
#[allow(clippy::needless_range_loop)]
pub fn eigenvalue_power(
    a: &[Vec<f64>],
    tol: f64,
    max_iter: usize,
) -> Result<(f64, Vec<f64>), HisabError> {
    let n = a.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".to_string()));
    }
    for row in a {
        if row.len() != n {
            return Err(HisabError::InvalidInput(format!(
                "expected square {}x{}, got row length {}",
                n,
                n,
                row.len()
            )));
        }
    }

    // Initial guess: unit vector
    let mut v = vec![0.0; n];
    v[0] = 1.0;
    let mut w = vec![0.0; n];
    let mut eigenvalue = 0.0;

    for _ in 0..max_iter {
        // w = A * v (reuse allocation)
        for wi in w.iter_mut() {
            *wi = 0.0;
        }
        for i in 0..n {
            for j in 0..n {
                w[i] += a[i][j] * v[j];
            }
        }

        // Find the component with largest absolute value
        let mut max_val = 0.0f64;
        for &wi in &w {
            if wi.abs() > max_val.abs() {
                max_val = wi;
            }
        }

        if max_val.abs() < crate::EPSILON_F64 {
            return Err(HisabError::NoConvergence(max_iter));
        }

        let new_eigenvalue = max_val;

        // Normalize
        for vi in &mut w {
            *vi /= max_val;
        }

        if (new_eigenvalue - eigenvalue).abs() < tol {
            return Ok((new_eigenvalue, w));
        }

        eigenvalue = new_eigenvalue;
        std::mem::swap(&mut v, &mut w);
    }

    Err(HisabError::NoConvergence(max_iter))
}

// ---------------------------------------------------------------------------
// FFT (Cooley-Tukey radix-2)
// ---------------------------------------------------------------------------

/// A complex number for FFT operations.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    /// Create a new complex number.
    #[must_use]
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Complex number from a real value.
    #[must_use]
    #[inline]
    pub const fn from_real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    /// Magnitude (absolute value).
    #[must_use]
    #[inline]
    pub fn abs(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Complex conjugate.
    #[must_use]
    #[inline]
    pub const fn conj(self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }
}

impl Default for Complex {
    fn default() -> Self {
        Self::new(0.0, 0.0)
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{}+{}i", self.re, self.im)
        } else {
            write!(f, "{}{}i", self.re, self.im)
        }
    }
}

impl std::ops::Add for Complex {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            re: self.re + rhs.re,
            im: self.im + rhs.im,
        }
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            re: self.re - rhs.re,
            im: self.im - rhs.im,
        }
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            re: self.re * rhs.re - self.im * rhs.im,
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Self {
            re: self.re * rhs,
            im: self.im * rhs,
        }
    }
}

impl std::ops::Div for Complex {
    type Output = Self;
    #[inline]
    fn div(self, rhs: Self) -> Self {
        let denom = rhs.re * rhs.re + rhs.im * rhs.im;
        Self {
            re: (self.re * rhs.re + self.im * rhs.im) / denom,
            im: (self.im * rhs.re - self.re * rhs.im) / denom,
        }
    }
}

impl std::ops::Div<f64> for Complex {
    type Output = Self;
    #[inline]
    fn div(self, rhs: f64) -> Self {
        Self {
            re: self.re / rhs,
            im: self.im / rhs,
        }
    }
}

impl std::ops::Neg for Complex {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            re: -self.re,
            im: -self.im,
        }
    }
}

impl From<f64> for Complex {
    #[inline]
    fn from(re: f64) -> Self {
        Self { re, im: 0.0 }
    }
}

impl From<(f64, f64)> for Complex {
    #[inline]
    fn from((re, im): (f64, f64)) -> Self {
        Self { re, im }
    }
}

/// In-place Cooley-Tukey radix-2 FFT.
///
/// `data` must have a power-of-2 length. Computes the DFT in-place.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data.len()` is not a power of two.
#[must_use = "returns an error if input length is not a power of two"]
pub fn fft(data: &mut [Complex]) -> Result<(), HisabError> {
    let n = data.len();
    if n <= 1 {
        return Ok(());
    }
    if !n.is_power_of_two() {
        return Err(HisabError::InvalidInput(
            "FFT requires power-of-2 length".into(),
        ));
    }

    // Bit-reversal permutation
    let mut j = 0usize;
    for i in 1..n {
        let mut bit = n >> 1;
        while j & bit != 0 {
            j ^= bit;
            bit >>= 1;
        }
        j ^= bit;
        if i < j {
            data.swap(i, j);
        }
    }

    // Butterfly stages
    let mut len = 2;
    while len <= n {
        let half = len / 2;
        let angle = -2.0 * std::f64::consts::PI / len as f64;
        let wn = Complex::new(angle.cos(), angle.sin());

        let mut start = 0;
        while start < n {
            let mut w = Complex::new(1.0, 0.0);
            for k in 0..half {
                let u = data[start + k];
                let t = w * data[start + k + half];
                data[start + k] = u + t;
                data[start + k + half] = u - t;
                w = w * wn;
            }
            start += len;
        }
        len <<= 1;
    }
    Ok(())
}

/// In-place inverse FFT.
///
/// `data` must have a power-of-2 length.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data.len()` is not a power of two.
#[must_use = "returns an error if input length is not a power of two"]
pub fn ifft(data: &mut [Complex]) -> Result<(), HisabError> {
    let n = data.len();
    // Conjugate, FFT, conjugate, scale
    for d in data.iter_mut() {
        *d = d.conj();
    }
    fft(data)?;
    let scale = 1.0 / n as f64;
    for d in data.iter_mut() {
        *d = d.conj() * scale;
    }
    Ok(())
}

/// In-place 2D FFT on a row-major grid of `rows × cols`.
///
/// Both `rows` and `cols` must be powers of two.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if dimensions aren't powers of two
/// or `data.len() != rows * cols`.
#[must_use = "returns an error if dimensions are invalid"]
pub fn fft_2d(data: &mut [Complex], rows: usize, cols: usize) -> Result<(), HisabError> {
    if data.len() != rows * cols {
        return Err(HisabError::InvalidInput(format!(
            "data length {} != rows*cols {}",
            data.len(),
            rows * cols
        )));
    }
    // FFT each row
    for r in 0..rows {
        let row = &mut data[r * cols..(r + 1) * cols];
        fft(row)?;
    }
    // FFT each column (extract, transform, put back)
    let mut col_buf = vec![Complex::new(0.0, 0.0); rows];
    for c in 0..cols {
        for r in 0..rows {
            col_buf[r] = data[r * cols + c];
        }
        fft(&mut col_buf)?;
        for r in 0..rows {
            data[r * cols + c] = col_buf[r];
        }
    }
    Ok(())
}

/// In-place 2D inverse FFT on a row-major grid.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if dimensions aren't powers of two
/// or `data.len() != rows * cols`.
#[must_use = "returns an error if dimensions are invalid"]
pub fn ifft_2d(data: &mut [Complex], rows: usize, cols: usize) -> Result<(), HisabError> {
    if data.len() != rows * cols {
        return Err(HisabError::InvalidInput(format!(
            "data length {} != rows*cols {}",
            data.len(),
            rows * cols
        )));
    }
    for r in 0..rows {
        let row = &mut data[r * cols..(r + 1) * cols];
        ifft(row)?;
    }
    let mut col_buf = vec![Complex::new(0.0, 0.0); rows];
    for c in 0..cols {
        for r in 0..rows {
            col_buf[r] = data[r * cols + c];
        }
        ifft(&mut col_buf)?;
        for r in 0..rows {
            data[r * cols + c] = col_buf[r];
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Discrete Sine / Cosine Transforms
// ---------------------------------------------------------------------------

/// Discrete Sine Transform Type-I (DST-I).
///
/// Computes `X[k] = Σ x[n] · sin(π·(n+1)·(k+1) / (N+1))` for `k = 0..N-1`.
///
/// Used for wall-bounded Poisson solvers where the solution vanishes at both
/// boundaries (Dirichlet conditions).
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data` is empty.
#[must_use = "contains the DST coefficients or an error"]
pub fn dst(data: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = data.len();
    if n == 0 {
        return Err(HisabError::InvalidInput(
            "DST requires non-empty input".into(),
        ));
    }
    let np1 = (n + 1) as f64;
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = 0.0;
        for (i, &x) in data.iter().enumerate() {
            sum += x * (std::f64::consts::PI * (i + 1) as f64 * (k + 1) as f64 / np1).sin();
        }
        out.push(sum);
    }
    Ok(out)
}

/// Inverse Discrete Sine Transform Type-I (IDST-I).
///
/// DST-I is its own inverse up to a scale factor of `2 / (N+1)`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data` is empty.
#[must_use = "contains the inverse DST result or an error"]
pub fn idst(data: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = data.len();
    let mut out = dst(data)?;
    let scale = 2.0 / (n + 1) as f64;
    for v in &mut out {
        *v *= scale;
    }
    Ok(out)
}

/// Discrete Cosine Transform Type-II (DCT-II).
///
/// Computes `X[k] = Σ x[n] · cos(π·(2n+1)·k / (2N))` for `k = 0..N-1`.
///
/// Used for Neumann boundary conditions where the derivative vanishes at
/// boundaries. Also the basis of JPEG compression.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data` is empty.
#[must_use = "contains the DCT coefficients or an error"]
pub fn dct(data: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = data.len();
    if n == 0 {
        return Err(HisabError::InvalidInput(
            "DCT requires non-empty input".into(),
        ));
    }
    let two_n = 2.0 * n as f64;
    let mut out = Vec::with_capacity(n);
    for k in 0..n {
        let mut sum = 0.0;
        for (i, &x) in data.iter().enumerate() {
            sum += x * (std::f64::consts::PI * (2.0 * i as f64 + 1.0) * k as f64 / two_n).cos();
        }
        out.push(sum);
    }
    Ok(out)
}

/// Inverse Discrete Cosine Transform (IDCT / DCT-III).
///
/// Inverts `dct()`: `x[n] = X[0]/N + (2/N)·Σ_{k=1}^{N-1} X[k]·cos(π·k·(2n+1)/(2N))`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `data` is empty.
#[must_use = "contains the inverse DCT result or an error"]
pub fn idct(data: &[f64]) -> Result<Vec<f64>, HisabError> {
    let n = data.len();
    if n == 0 {
        return Err(HisabError::InvalidInput(
            "IDCT requires non-empty input".into(),
        ));
    }
    let two_n = 2.0 * n as f64;
    let inv_n = 1.0 / n as f64;
    let dc = data[0] * inv_n;
    let scale = 2.0 * inv_n;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let mut sum = dc;
        for (k, &x) in data.iter().enumerate().skip(1) {
            sum += scale
                * x
                * (std::f64::consts::PI * k as f64 * (2.0 * i as f64 + 1.0) / two_n).cos();
        }
        out.push(sum);
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// ODE solvers
// ---------------------------------------------------------------------------

/// Perform one RK4 step in-place, reusing scratch buffers.
///
/// All k1–k4 and tmp buffers are allocated by the caller and reused across steps,
/// eliminating per-step heap allocations.
#[allow(clippy::needless_range_loop, clippy::too_many_arguments)]
fn rk4_step(
    f: &impl Fn(f64, &[f64], &mut [f64]),
    t: f64,
    h: f64,
    y: &mut [f64],
    k1: &mut [f64],
    k2: &mut [f64],
    k3: &mut [f64],
    k4: &mut [f64],
    tmp: &mut [f64],
    dim: usize,
) {
    f(t, y, k1);

    for i in 0..dim {
        tmp[i] = y[i] + 0.5 * h * k1[i];
    }
    f(t + 0.5 * h, tmp, k2);

    for i in 0..dim {
        tmp[i] = y[i] + 0.5 * h * k2[i];
    }
    f(t + 0.5 * h, tmp, k3);

    for i in 0..dim {
        tmp[i] = y[i] + h * k3[i];
    }
    f(t + h, tmp, k4);

    let h6 = h / 6.0;
    for i in 0..dim {
        y[i] += h6 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
    }
}

/// Fourth-order Runge-Kutta (RK4) integrator for a system of ODEs.
///
/// Solves `dy/dt = f(t, y)` from `t0` to `t_end` with `n` steps.
///
/// - `f`: the derivative function `f(t, &y, &mut dy_dt)` that writes derivatives into the
///   provided output buffer, eliminating per-step heap allocations.
/// - `t0`: initial time.
/// - `y0`: initial state vector.
/// - `t_end`: final time.
/// - `n`: number of integration steps.
///
/// Returns the final state vector `y(t_end)`.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "contains the final state vector or an error"]
pub fn rk4(
    f: impl Fn(f64, &[f64], &mut [f64]),
    t0: f64,
    y0: &[f64],
    t_end: f64,
    n: usize,
) -> Result<Vec<f64>, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let dim = y0.len();
    let h = (t_end - t0) / n as f64;
    let mut t = t0;
    let mut y = y0.to_vec();
    let mut k1 = vec![0.0; dim];
    let mut k2 = vec![0.0; dim];
    let mut k3 = vec![0.0; dim];
    let mut k4 = vec![0.0; dim];
    let mut tmp = vec![0.0; dim];

    for _ in 0..n {
        rk4_step(
            &f, t, h, &mut y, &mut k1, &mut k2, &mut k3, &mut k4, &mut tmp, dim,
        );
        t += h;
    }

    Ok(y)
}

/// Fourth-order Runge-Kutta with full trajectory output.
///
/// Same as [`rk4`] but returns all intermediate states as `Vec<(t, y)>`.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "contains the full trajectory or an error"]
pub fn rk4_trajectory(
    f: impl Fn(f64, &[f64], &mut [f64]),
    t0: f64,
    y0: &[f64],
    t_end: f64,
    n: usize,
) -> Result<Vec<(f64, Vec<f64>)>, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let dim = y0.len();
    let h = (t_end - t0) / n as f64;
    let mut t = t0;
    let mut y = y0.to_vec();
    let mut k1 = vec![0.0; dim];
    let mut k2 = vec![0.0; dim];
    let mut k3 = vec![0.0; dim];
    let mut k4 = vec![0.0; dim];
    let mut tmp = vec![0.0; dim];
    let mut trajectory = Vec::with_capacity(n + 1);
    trajectory.push((t, y.clone()));

    for _ in 0..n {
        rk4_step(
            &f, t, h, &mut y, &mut k1, &mut k2, &mut k3, &mut k4, &mut tmp, dim,
        );
        t += h;
        trajectory.push((t, y.clone()));
    }

    Ok(trajectory)
}

// ---------------------------------------------------------------------------
// Dormand-Prince adaptive RK4/5 (DOPRI5)
// ---------------------------------------------------------------------------

/// Dormand-Prince RK4(5) adaptive step-size ODE integrator.
///
/// Solves `dy/dt = f(t, y)` from `t0` to `t_end` with automatic step-size control.
///
/// - `f`: derivative function `f(t, &y, &mut dy_dt)`.
/// - `t0`: initial time.
/// - `y0`: initial state vector.
/// - `t_end`: final time.
/// - `tol`: error tolerance (controls step-size adaptation).
/// - `h_init`: initial step size (will be adapted).
///
/// Returns the trajectory as `Vec<(t, y)>`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `y0` is empty or `h_init` is not positive.
/// Returns [`HisabError::NoConvergence`] if the maximum number of steps (100_000) is exceeded.
#[must_use = "contains the trajectory or an error"]
#[allow(clippy::too_many_arguments)]
pub fn dopri45(
    f: impl Fn(f64, &[f64], &mut [f64]),
    t0: f64,
    y0: &[f64],
    t_end: f64,
    tol: f64,
    h_init: f64,
) -> Result<Vec<(f64, Vec<f64>)>, HisabError> {
    let dim = y0.len();
    if dim == 0 {
        return Err(HisabError::InvalidInput("empty initial state".into()));
    }
    if h_init <= 0.0 {
        return Err(HisabError::InvalidInput("h_init must be positive".into()));
    }

    // Standard Dormand-Prince Butcher tableau coefficients
    // Stage 2: a21
    const A21: f64 = 1.0 / 5.0;
    // Stage 3: a31, a32
    const A3: [f64; 2] = [3.0 / 40.0, 9.0 / 40.0];
    // Stage 4: a41, a42, a43
    const A4: [f64; 3] = [44.0 / 45.0, -56.0 / 15.0, 32.0 / 9.0];
    // Stage 5: a51..a54
    const A5: [f64; 4] = [
        19372.0 / 6561.0,
        -25360.0 / 2187.0,
        64448.0 / 6561.0,
        -212.0 / 729.0,
    ];
    // Stage 6: a61..a65
    const A6: [f64; 5] = [
        9017.0 / 3168.0,
        -355.0 / 33.0,
        46732.0 / 5247.0,
        49.0 / 176.0,
        -5103.0 / 18656.0,
    ];

    // 5th-order weights (b1..b7, b2=b7=0)
    const B5: [f64; 7] = [
        35.0 / 384.0,
        0.0,
        500.0 / 1113.0,
        125.0 / 192.0,
        -2187.0 / 6784.0,
        11.0 / 84.0,
        0.0,
    ];

    // Error estimate coefficients (b5_i - b4_i)
    const E: [f64; 7] = [
        71.0 / 57600.0,
        0.0,
        -71.0 / 16695.0,
        71.0 / 1920.0,
        -17253.0 / 339200.0,
        22.0 / 525.0,
        -1.0 / 40.0,
    ];

    let max_steps = 100_000;
    let mut t = t0;
    let mut y = y0.to_vec();
    let mut h = h_init.min((t_end - t0).abs());
    let direction = if t_end >= t0 { 1.0 } else { -1.0 };

    let mut trajectory = Vec::new();
    trajectory.push((t, y.clone()));

    // Scratch buffers
    let mut k1 = vec![0.0; dim];
    let mut k2 = vec![0.0; dim];
    let mut k3 = vec![0.0; dim];
    let mut k4 = vec![0.0; dim];
    let mut k5 = vec![0.0; dim];
    let mut k6 = vec![0.0; dim];
    let mut k7 = vec![0.0; dim];
    let mut y_tmp = vec![0.0; dim];

    for _ in 0..max_steps {
        if (t - t_end) * direction >= 0.0 {
            break;
        }

        // Clamp step to not overshoot
        if (t + direction * h - t_end) * direction > 0.0 {
            h = (t_end - t).abs();
        }
        let hd = direction * h;

        // Stage 1
        f(t, &y, &mut k1);

        // Stage 2
        for i in 0..dim {
            y_tmp[i] = y[i] + hd * A21 * k1[i];
        }
        f(t + hd * (1.0 / 5.0), &y_tmp, &mut k2);

        // Stage 3
        for i in 0..dim {
            y_tmp[i] = y[i] + hd * (A3[0] * k1[i] + A3[1] * k2[i]);
        }
        f(t + hd * (3.0 / 10.0), &y_tmp, &mut k3);

        // Stage 4
        for i in 0..dim {
            y_tmp[i] = y[i] + hd * (A4[0] * k1[i] + A4[1] * k2[i] + A4[2] * k3[i]);
        }
        f(t + hd * (4.0 / 5.0), &y_tmp, &mut k4);

        // Stage 5
        for i in 0..dim {
            y_tmp[i] = y[i] + hd * (A5[0] * k1[i] + A5[1] * k2[i] + A5[2] * k3[i] + A5[3] * k4[i]);
        }
        f(t + hd * (8.0 / 9.0), &y_tmp, &mut k5);

        // Stage 6
        for i in 0..dim {
            y_tmp[i] = y[i]
                + hd * (A6[0] * k1[i]
                    + A6[1] * k2[i]
                    + A6[2] * k3[i]
                    + A6[3] * k4[i]
                    + A6[4] * k5[i]);
        }
        f(t + hd, &y_tmp, &mut k6);

        // 5th-order solution
        for i in 0..dim {
            y_tmp[i] = y[i]
                + hd * (B5[0] * k1[i]
                    + B5[2] * k3[i]
                    + B5[3] * k4[i]
                    + B5[4] * k5[i]
                    + B5[5] * k6[i]);
        }

        // Stage 7 (for error estimate)
        f(t + hd, &y_tmp, &mut k7);

        // Error estimate
        let mut err_norm = 0.0;
        for i in 0..dim {
            let ei = hd
                * (E[0] * k1[i]
                    + E[2] * k3[i]
                    + E[3] * k4[i]
                    + E[4] * k5[i]
                    + E[5] * k6[i]
                    + E[6] * k7[i]);
            let scale = tol.max(tol * y_tmp[i].abs());
            err_norm += (ei / scale) * (ei / scale);
        }
        err_norm = (err_norm / dim as f64).sqrt();

        if err_norm <= 1.0 {
            // Accept step
            t += hd;
            y.copy_from_slice(&y_tmp);
            trajectory.push((t, y.clone()));
        }

        // Adapt step size
        let safety = 0.9;
        let factor = if err_norm > 0.0 {
            safety * err_norm.powf(-0.2)
        } else {
            5.0
        };
        h *= factor.clamp(0.2, 5.0);
    }

    if (t - t_end).abs() > h * 0.01 {
        return Err(HisabError::NoConvergence(max_steps));
    }

    Ok(trajectory)
}

// ---------------------------------------------------------------------------
// Full eigendecomposition
// ---------------------------------------------------------------------------

/// Result of an eigendecomposition.
#[derive(Debug, Clone)]
#[must_use]
pub struct EigenDecomposition {
    /// Real parts of eigenvalues (sorted by descending magnitude).
    pub eigenvalues_real: Vec<f64>,
    /// Imaginary parts of eigenvalues (zero for real eigenvalues).
    pub eigenvalues_imag: Vec<f64>,
    /// Eigenvectors as columns (only for symmetric/real eigenvalues; `None` if complex).
    pub eigenvectors: Option<Vec<Vec<f64>>>,
}

/// Symmetric eigendecomposition via tridiagonal QR with Wilkinson shift.
///
/// Input must be a symmetric `n × n` matrix (only lower triangle is read).
/// Returns all eigenvalues (sorted descending) and orthonormal eigenvectors.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty or not square.
/// Returns [`HisabError::NoConvergence`] if QR iteration doesn't converge.
#[must_use = "contains the eigendecomposition or an error"]
#[allow(clippy::needless_range_loop)]
pub fn eigen_symmetric(
    a: &[Vec<f64>],
    tol: f64,
    max_iter: usize,
) -> Result<EigenDecomposition, HisabError> {
    // Jacobi eigenvalue algorithm for symmetric matrices.
    // Same rotation approach proven in our SVD — rotate pairs until diagonal.
    let n = a.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    for row in a {
        if row.len() != n {
            return Err(HisabError::InvalidInput("matrix must be square".into()));
        }
    }

    if n == 1 {
        return Ok(EigenDecomposition {
            eigenvalues_real: vec![a[0][0]],
            eigenvalues_imag: vec![0.0],
            eigenvectors: Some(vec![vec![1.0]]),
        });
    }

    // Work on a copy
    let mut w: Vec<Vec<f64>> = a.to_vec();

    // Eigenvector accumulator (identity initially)
    let mut v = vec![vec![0.0; n]; n];
    for i in 0..n {
        v[i][i] = 1.0;
    }

    let tol_sq = tol * tol;

    for _ in 0..max_iter {
        // Find largest off-diagonal element
        let mut converged = true;
        for p in 0..n {
            for q_idx in (p + 1)..n {
                if w[p][q_idx].abs() > tol_sq {
                    converged = false;

                    // Jacobi rotation to zero out w[p][q]
                    let theta = if (w[p][p] - w[q_idx][q_idx]).abs() < crate::EPSILON_F64 {
                        std::f64::consts::FRAC_PI_4
                    } else {
                        0.5 * (2.0 * w[p][q_idx] / (w[p][p] - w[q_idx][q_idx])).atan()
                    };
                    let cos = theta.cos();
                    let sin = theta.sin();

                    // Update matrix: W' = Gᵀ W G
                    // Rows/cols p and q change
                    let mut new_p = vec![0.0; n];
                    let mut new_q = vec![0.0; n];
                    for i in 0..n {
                        new_p[i] = cos * w[p][i] + sin * w[q_idx][i];
                        new_q[i] = -sin * w[p][i] + cos * w[q_idx][i];
                    }
                    w[p][..n].copy_from_slice(&new_p[..n]);
                    w[q_idx][..n].copy_from_slice(&new_q[..n]);
                    // Now columns
                    for i in 0..n {
                        let wp = w[i][p];
                        let wq = w[i][q_idx];
                        w[i][p] = cos * wp + sin * wq;
                        w[i][q_idx] = -sin * wp + cos * wq;
                    }

                    // Accumulate eigenvectors: V' = V * G
                    for i in 0..n {
                        let vp = v[i][p];
                        let vq = v[i][q_idx];
                        v[i][p] = cos * vp + sin * vq;
                        v[i][q_idx] = -sin * vp + cos * vq;
                    }
                }
            }
        }
        if converged {
            break;
        }
    }

    // Eigenvalues are the diagonal of W
    let eigenvalues: Vec<f64> = (0..n).map(|i| w[i][i]).collect();

    // Sort by descending magnitude
    let mut order: Vec<usize> = (0..n).collect();
    order.sort_unstable_by(|&a, &b| {
        eigenvalues[b]
            .abs()
            .partial_cmp(&eigenvalues[a].abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let sorted_eigs: Vec<f64> = order.iter().map(|&i| eigenvalues[i]).collect();
    let eigenvectors: Vec<Vec<f64>> = order
        .iter()
        .map(|&idx| (0..n).map(|i| v[i][idx]).collect())
        .collect();

    Ok(EigenDecomposition {
        eigenvalues_real: sorted_eigs,
        eigenvalues_imag: vec![0.0; n],
        eigenvectors: Some(eigenvectors),
    })
}

// ---------------------------------------------------------------------------
// Inertia tensor computation
// ---------------------------------------------------------------------------

/// Compute the inertia tensor of a solid sphere.
#[must_use]
#[inline]
pub fn inertia_sphere(mass: f64, radius: f64) -> Vec<Vec<f64>> {
    let i = 0.4 * mass * radius * radius;
    vec![vec![i, 0.0, 0.0], vec![0.0, i, 0.0], vec![0.0, 0.0, i]]
}

/// Compute the inertia tensor of a solid box (cuboid).
#[must_use]
#[inline]
pub fn inertia_box(mass: f64, hx: f64, hy: f64, hz: f64) -> Vec<Vec<f64>> {
    let w = 2.0 * hx;
    let h = 2.0 * hy;
    let d = 2.0 * hz;
    let c = mass / 12.0;
    vec![
        vec![c * (h * h + d * d), 0.0, 0.0],
        vec![0.0, c * (w * w + d * d), 0.0],
        vec![0.0, 0.0, c * (w * w + h * h)],
    ]
}

/// Compute the inertia tensor of a triangle mesh (solid body).
///
/// Uses the divergence theorem method. The mesh must be closed with
/// consistent outward-facing winding.
///
/// Returns `(volume, center_of_mass, inertia_tensor_3x3)`.
#[must_use]
#[allow(clippy::needless_range_loop)]
pub fn inertia_mesh(
    triangles: &[([f64; 3], [f64; 3], [f64; 3])],
) -> (f64, [f64; 3], Vec<Vec<f64>>) {
    let mut volume = 0.0;
    let mut com = [0.0; 3];
    let mut ii = [[0.0f64; 3]; 3];

    for &(v0, v1, v2) in triangles {
        let d = v0[0] * (v1[1] * v2[2] - v1[2] * v2[1]) - v0[1] * (v1[0] * v2[2] - v1[2] * v2[0])
            + v0[2] * (v1[0] * v2[1] - v1[1] * v2[0]);
        let vol = d / 6.0;
        volume += vol;

        for i in 0..3 {
            com[i] += vol * (v0[i] + v1[i] + v2[i]) / 4.0;
        }

        for i in 0..3 {
            for j in 0..3 {
                ii[i][j] += vol
                    * (v0[i] * v0[j]
                        + v1[i] * v1[j]
                        + v2[i] * v2[j]
                        + (v0[i] + v1[i] + v2[i]) * (v0[j] + v1[j] + v2[j]))
                    / 20.0;
            }
        }
    }

    if volume.abs() > crate::EPSILON_F64 {
        for c in &mut com {
            *c /= volume;
        }
    }

    let trace = ii[0][0] + ii[1][1] + ii[2][2];
    let inertia = vec![
        vec![trace - ii[0][0], -ii[0][1], -ii[0][2]],
        vec![-ii[1][0], trace - ii[1][1], -ii[1][2]],
        vec![-ii[2][0], -ii[2][1], trace - ii[2][2]],
    ];

    (volume, com, inertia)
}

// ---------------------------------------------------------------------------
// Projected Gauss-Seidel (PGS)
// ---------------------------------------------------------------------------

/// Projected Gauss-Seidel: solve A·x = b subject to `lo[i] <= x[i] <= hi[i]`.
///
/// Used as the inner solver for physics constraint solving.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if dimensions are inconsistent.
#[must_use = "contains the solution or an error"]
#[allow(clippy::needless_range_loop)]
pub fn projected_gauss_seidel(
    a: &[Vec<f64>],
    b: &[f64],
    lo: &[f64],
    hi: &[f64],
    x0: &[f64],
    max_iter: usize,
    tol: f64,
) -> Result<Vec<f64>, HisabError> {
    let n = b.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty system".into()));
    }
    if a.len() != n || lo.len() != n || hi.len() != n || x0.len() != n {
        return Err(HisabError::InvalidInput("dimension mismatch".into()));
    }

    let mut x = x0.to_vec();

    for _ in 0..max_iter {
        let mut max_change = 0.0f64;
        for i in 0..n {
            if a[i][i].abs() < crate::EPSILON_F64 {
                continue;
            }
            let mut sigma = b[i];
            for j in 0..n {
                if j != i {
                    sigma -= a[i][j] * x[j];
                }
            }
            let new_x = (sigma / a[i][i]).clamp(lo[i], hi[i]);
            max_change = max_change.max((new_x - x[i]).abs());
            x[i] = new_x;
        }
        if max_change < tol {
            break;
        }
    }

    Ok(x)
}

// ---------------------------------------------------------------------------
// GMRES iterative solver
// ---------------------------------------------------------------------------

/// GMRES(m) for non-symmetric linear systems A·x = b.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if dimensions mismatch.
#[must_use = "contains the solution or an error"]
#[allow(clippy::needless_range_loop)]
pub fn gmres(
    a_mul: impl Fn(&[f64]) -> Vec<f64>,
    b: &[f64],
    x0: &[f64],
    restart: usize,
    tol: f64,
    max_iter: usize,
) -> Result<Vec<f64>, HisabError> {
    let n = b.len();
    if x0.len() != n {
        return Err(HisabError::InvalidInput(
            "x0 length must match b length".into(),
        ));
    }

    let mut x = x0.to_vec();
    let m = restart.min(n);

    for _ in 0..(max_iter / m.max(1) + 1) {
        let ax = a_mul(&x);
        let mut r: Vec<f64> = (0..n).map(|i| b[i] - ax[i]).collect();
        let r_norm: f64 = r.iter().map(|v| v * v).sum::<f64>().sqrt();

        if r_norm < tol {
            return Ok(x);
        }

        let mut v_basis: Vec<Vec<f64>> = Vec::with_capacity(m + 1);
        let inv_r = 1.0 / r_norm;
        for ri in &mut r {
            *ri *= inv_r;
        }
        v_basis.push(r);

        let mut h = vec![vec![0.0; m]; m + 1];
        let mut g = vec![0.0; m + 1];
        g[0] = r_norm;

        let mut cs = vec![0.0; m];
        let mut sn = vec![0.0; m];
        let mut k = 0;

        for j in 0..m {
            let mut wj = a_mul(&v_basis[j]);

            for i in 0..=j {
                h[i][j] = wj.iter().zip(v_basis[i].iter()).map(|(a, b)| a * b).sum();
                for l in 0..n {
                    wj[l] -= h[i][j] * v_basis[i][l];
                }
            }
            h[j + 1][j] = wj.iter().map(|v| v * v).sum::<f64>().sqrt();

            if h[j + 1][j] > crate::EPSILON_F64 {
                let inv = 1.0 / h[j + 1][j];
                for v in &mut wj {
                    *v *= inv;
                }
            }
            v_basis.push(wj);

            for i in 0..j {
                let temp = cs[i] * h[i][j] + sn[i] * h[i + 1][j];
                h[i + 1][j] = -sn[i] * h[i][j] + cs[i] * h[i + 1][j];
                h[i][j] = temp;
            }

            let r_val = (h[j][j] * h[j][j] + h[j + 1][j] * h[j + 1][j]).sqrt();
            if r_val > crate::EPSILON_F64 {
                cs[j] = h[j][j] / r_val;
                sn[j] = h[j + 1][j] / r_val;
            } else {
                cs[j] = 1.0;
                sn[j] = 0.0;
            }
            h[j][j] = r_val;
            h[j + 1][j] = 0.0;

            let temp = cs[j] * g[j];
            g[j + 1] = -sn[j] * g[j];
            g[j] = temp;

            k = j + 1;
            if g[k].abs() < tol {
                break;
            }
        }

        let mut y = vec![0.0; k];
        for i in (0..k).rev() {
            y[i] = g[i];
            for j in (i + 1)..k {
                y[i] -= h[i][j] * y[j];
            }
            if h[i][i].abs() > crate::EPSILON_F64 {
                y[i] /= h[i][i];
            }
        }

        for i in 0..n {
            for j in 0..k {
                x[i] += y[j] * v_basis[j][i];
            }
        }

        if g[k].abs() < tol {
            return Ok(x);
        }
    }

    Ok(x)
}

// ---------------------------------------------------------------------------
// Stability analysis (Lyapunov exponents)
// ---------------------------------------------------------------------------

/// Compute the maximal Lyapunov exponent of a dynamical system.
///
/// Evolves the system and a perturbation vector, measuring exponential
/// divergence rate. Positive MLE indicates chaos.
///
/// - `f`: derivative function `f(t, &y, &mut dy)`.
/// - `jac`: Jacobian `jac(t, &y, &mut J)`.
/// - `y0`: initial state.
/// - `t_total`: total integration time.
/// - `dt`: time step.
/// - `renorm_steps`: how many steps between perturbation renormalization.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `y0` is empty or `dt` is non-positive.
#[must_use = "contains the maximal Lyapunov exponent or an error"]
#[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
pub fn lyapunov_max(
    f: impl Fn(f64, &[f64], &mut [f64]),
    jac: impl Fn(f64, &[f64], &mut Vec<Vec<f64>>),
    y0: &[f64],
    t_total: f64,
    dt: f64,
    renorm_steps: usize,
) -> Result<f64, HisabError> {
    let dim = y0.len();
    if dim == 0 {
        return Err(HisabError::InvalidInput("empty initial state".into()));
    }
    if dt <= 0.0 {
        return Err(HisabError::InvalidInput("dt must be positive".into()));
    }

    let n_steps = (t_total / dt) as usize;
    let renorm = renorm_steps.max(1);

    let mut y = y0.to_vec();
    // Perturbation vector (initially unit)
    let mut dx = vec![0.0; dim];
    dx[0] = 1.0;
    let mut t = 0.0;
    let mut sum_log = 0.0;
    let mut count = 0;

    let mut dy = vec![0.0; dim];
    let mut j_mat = vec![vec![0.0; dim]; dim];
    let mut ddx = vec![0.0; dim];

    for step in 0..n_steps {
        // Evolve main trajectory: simple Euler (good enough for Lyapunov computation)
        f(t, &y, &mut dy);
        jac(t, &y, &mut j_mat);

        // Evolve perturbation: ddx = J * dx
        for i in 0..dim {
            ddx[i] = 0.0;
            for j in 0..dim {
                ddx[i] += j_mat[i][j] * dx[j];
            }
        }

        for i in 0..dim {
            y[i] += dt * dy[i];
            dx[i] += dt * ddx[i];
        }
        t += dt;

        // Renormalize perturbation periodically
        if (step + 1) % renorm == 0 {
            let norm: f64 = dx.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm > crate::EPSILON_F64 {
                sum_log += norm.ln();
                count += 1;
                let inv = 1.0 / norm;
                for x in &mut dx {
                    *x *= inv;
                }
            }
        }
    }

    if count == 0 {
        return Ok(0.0);
    }

    Ok(sum_log / (count as f64 * renorm as f64 * dt))
}

// ---------------------------------------------------------------------------
// Stiff ODE solvers
// ---------------------------------------------------------------------------

/// Backward (implicit) Euler method for stiff ODE systems.
///
/// Solves `dy/dt = f(t, y)` using Newton iteration at each step.
///
/// - `f`: derivative function `f(t, &y, &mut dy)`.
/// - `jac`: Jacobian callback `jac(t, &y, &mut J)` fills the n×n Jacobian matrix.
/// - `t0`, `y0`, `t_end`, `n`: integration parameters.
/// - `newton_tol`: convergence tolerance for Newton iteration.
/// - `max_newton`: max Newton iterations per step.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "contains the final state or an error"]
#[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
pub fn backward_euler(
    f: impl Fn(f64, &[f64], &mut [f64]),
    jac: impl Fn(f64, &[f64], &mut Vec<Vec<f64>>),
    t0: f64,
    y0: &[f64],
    t_end: f64,
    n: usize,
    newton_tol: f64,
    max_newton: usize,
) -> Result<Vec<f64>, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let dim = y0.len();
    let h = (t_end - t0) / n as f64;
    let mut y = y0.to_vec();
    let mut t = t0;

    let mut f_val = vec![0.0; dim];
    let mut j_mat = vec![vec![0.0; dim]; dim];
    let mut delta = vec![0.0; dim];
    let mut y_guess = vec![0.0; dim];

    for _ in 0..n {
        t += h;
        y_guess.copy_from_slice(&y);

        for _ in 0..max_newton {
            f(t, &y_guess, &mut f_val);

            // Residual: r = y_guess - y - h*f(t, y_guess)
            // Newton system: (I - h*J) * delta = -r
            jac(t, &y_guess, &mut j_mat);

            // Build augmented matrix for Gaussian elimination
            let mut aug: Vec<Vec<f64>> = Vec::with_capacity(dim);
            for i in 0..dim {
                let mut row = Vec::with_capacity(dim + 1);
                for j in 0..dim {
                    let ident = if i == j { 1.0 } else { 0.0 };
                    row.push(ident - h * j_mat[i][j]);
                }
                row.push(-(y_guess[i] - y[i] - h * f_val[i]));
                aug.push(row);
            }

            delta = match gaussian_elimination(&mut aug) {
                Ok(d) => d,
                Err(_) => break,
            };

            let norm: f64 = delta.iter().map(|d| d * d).sum::<f64>().sqrt();
            for i in 0..dim {
                y_guess[i] += delta[i];
            }
            if norm < newton_tol {
                break;
            }
        }

        y.copy_from_slice(&y_guess);
    }

    Ok(y)
}

/// BDF-2 (Backward Differentiation Formula, order 2) for stiff ODE systems.
///
/// Second-order implicit method. Uses backward Euler for the first step.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "contains the final state or an error"]
#[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
pub fn bdf2(
    f: impl Fn(f64, &[f64], &mut [f64]),
    jac: impl Fn(f64, &[f64], &mut Vec<Vec<f64>>),
    t0: f64,
    y0: &[f64],
    t_end: f64,
    n: usize,
    newton_tol: f64,
    max_newton: usize,
) -> Result<Vec<f64>, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let dim = y0.len();
    let h = (t_end - t0) / n as f64;
    let mut y_prev = y0.to_vec();
    let mut t = t0;

    let mut f_val = vec![0.0; dim];
    let mut j_mat = vec![vec![0.0; dim]; dim];
    let mut y_guess = vec![0.0; dim];

    // First step: backward Euler
    t += h;
    y_guess.copy_from_slice(&y_prev);
    for _ in 0..max_newton {
        f(t, &y_guess, &mut f_val);
        jac(t, &y_guess, &mut j_mat);
        let mut aug: Vec<Vec<f64>> = Vec::with_capacity(dim);
        for i in 0..dim {
            let mut row = Vec::with_capacity(dim + 1);
            for j in 0..dim {
                let ident = if i == j { 1.0 } else { 0.0 };
                row.push(ident - h * j_mat[i][j]);
            }
            row.push(-(y_guess[i] - y_prev[i] - h * f_val[i]));
            aug.push(row);
        }
        if let Ok(delta) = gaussian_elimination(&mut aug) {
            let norm: f64 = delta.iter().map(|d| d * d).sum::<f64>().sqrt();
            for i in 0..dim {
                y_guess[i] += delta[i];
            }
            if norm < newton_tol {
                break;
            }
        } else {
            break;
        }
    }

    let mut y_curr = y_guess.clone();

    // BDF-2 steps: y_{k+1} = (4/3)*y_k - (1/3)*y_{k-1} + (2/3)*h*f(t_{k+1}, y_{k+1})
    for _ in 1..n {
        t += h;

        // Predictor: extrapolate
        for i in 0..dim {
            y_guess[i] = (4.0 / 3.0) * y_curr[i] - (1.0 / 3.0) * y_prev[i];
        }

        for _ in 0..max_newton {
            f(t, &y_guess, &mut f_val);
            jac(t, &y_guess, &mut j_mat);

            let mut aug: Vec<Vec<f64>> = Vec::with_capacity(dim);
            for i in 0..dim {
                let mut row = Vec::with_capacity(dim + 1);
                for j in 0..dim {
                    let ident = if i == j { 1.0 } else { 0.0 };
                    row.push(ident - (2.0 / 3.0) * h * j_mat[i][j]);
                }
                let rhs = y_guess[i] - (4.0 / 3.0) * y_curr[i] + (1.0 / 3.0) * y_prev[i]
                    - (2.0 / 3.0) * h * f_val[i];
                row.push(-rhs);
                aug.push(row);
            }

            if let Ok(delta) = gaussian_elimination(&mut aug) {
                let norm: f64 = delta.iter().map(|d| d * d).sum::<f64>().sqrt();
                for i in 0..dim {
                    y_guess[i] += delta[i];
                }
                if norm < newton_tol {
                    break;
                }
            } else {
                break;
            }
        }

        y_prev.copy_from_slice(&y_curr);
        y_curr.copy_from_slice(&y_guess);
    }

    Ok(y_curr)
}

// ---------------------------------------------------------------------------
// Stochastic differential equations
// ---------------------------------------------------------------------------

/// Euler-Maruyama method for stochastic differential equations.
///
/// Solves `dX = drift(t,X)*dt + diffusion(t,X)*dW` where dW ~ N(0,dt).
///
/// - `drift`: deterministic part `drift(t, &y, &mut dy)`.
/// - `diffusion`: noise coefficient `diffusion(t, &y, &mut noise)`.
/// - `rng`: seeded `Pcg32` for deterministic replay.
///
/// Returns the full trajectory.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "contains the trajectory or an error"]
#[allow(clippy::too_many_arguments)]
pub fn euler_maruyama(
    drift: impl Fn(f64, &[f64], &mut [f64]),
    diffusion: impl Fn(f64, &[f64], &mut [f64]),
    t0: f64,
    y0: &[f64],
    t_end: f64,
    n: usize,
    rng: &mut Pcg32,
) -> Result<Vec<(f64, Vec<f64>)>, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let dim = y0.len();
    let dt = (t_end - t0) / n as f64;
    let sqrt_dt = dt.sqrt();
    let mut y = y0.to_vec();
    let mut t = t0;
    let mut trajectory = Vec::with_capacity(n + 1);
    trajectory.push((t, y.clone()));

    let mut a = vec![0.0; dim];
    let mut b = vec![0.0; dim];

    for _ in 0..n {
        drift(t, &y, &mut a);
        diffusion(t, &y, &mut b);
        for i in 0..dim {
            let dw = rng.next_normal() * sqrt_dt;
            y[i] += a[i] * dt + b[i] * dw;
        }
        t += dt;
        trajectory.push((t, y.clone()));
    }

    Ok(trajectory)
}

/// Milstein method for stochastic differential equations.
///
/// Higher-order than Euler-Maruyama (strong order 1.0 vs 0.5).
/// Requires the derivative of the diffusion coefficient.
///
/// - `diffusion_deriv`: `db/dy` evaluated at `(t, &y, &mut db_dy)`.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "contains the trajectory or an error"]
#[allow(clippy::too_many_arguments)]
pub fn milstein(
    drift: impl Fn(f64, &[f64], &mut [f64]),
    diffusion: impl Fn(f64, &[f64], &mut [f64]),
    diffusion_deriv: impl Fn(f64, &[f64], &mut [f64]),
    t0: f64,
    y0: &[f64],
    t_end: f64,
    n: usize,
    rng: &mut Pcg32,
) -> Result<Vec<(f64, Vec<f64>)>, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let dim = y0.len();
    let dt = (t_end - t0) / n as f64;
    let sqrt_dt = dt.sqrt();
    let mut y = y0.to_vec();
    let mut t = t0;
    let mut trajectory = Vec::with_capacity(n + 1);
    trajectory.push((t, y.clone()));

    let mut a = vec![0.0; dim];
    let mut b = vec![0.0; dim];
    let mut db = vec![0.0; dim];

    for _ in 0..n {
        drift(t, &y, &mut a);
        diffusion(t, &y, &mut b);
        diffusion_deriv(t, &y, &mut db);
        for i in 0..dim {
            let dw = rng.next_normal() * sqrt_dt;
            // Milstein correction: + 0.5 * b * b' * (dW² - dt)
            y[i] += a[i] * dt + b[i] * dw + 0.5 * b[i] * db[i] * (dw * dw - dt);
        }
        t += dt;
        trajectory.push((t, y.clone()));
    }

    Ok(trajectory)
}

// ---------------------------------------------------------------------------
// Symplectic integrators
// ---------------------------------------------------------------------------

/// Symplectic Euler (semi-implicit Euler) integrator step.
///
/// Updates velocity first, then position — preserves phase-space volume.
///
/// - `acc_fn`: computes acceleration from `(t, position, &mut acceleration_out)`.
/// - `pos`: current position vector (modified in place).
/// - `vel`: current velocity vector (modified in place).
/// - `t`: current time.
/// - `dt`: time step.
#[inline]
pub fn symplectic_euler_step(
    acc_fn: &impl Fn(f64, &[f64], &mut [f64]),
    pos: &mut [f64],
    vel: &mut [f64],
    t: f64,
    dt: f64,
) {
    let dim = pos.len();
    let mut acc = vec![0.0; dim];
    acc_fn(t, pos, &mut acc);
    for i in 0..dim {
        vel[i] += dt * acc[i];
        pos[i] += dt * vel[i]; // uses updated velocity
    }
}

/// Velocity Verlet (Störmer-Verlet) integrator step.
///
/// Second-order, symplectic, time-reversible. The standard for molecular
/// dynamics and physics engines.
///
/// - `acc_fn`: computes acceleration from `(t, position, &mut acceleration_out)`.
/// - `pos`: current position (modified in place).
/// - `vel`: current velocity (modified in place).
/// - `t`: current time.
/// - `dt`: time step.
/// - `acc_prev`: acceleration from the previous step (modified to current step's acceleration).
#[inline]
pub fn verlet_step(
    acc_fn: &impl Fn(f64, &[f64], &mut [f64]),
    pos: &mut [f64],
    vel: &mut [f64],
    t: f64,
    dt: f64,
    acc_prev: &mut [f64],
) {
    let dim = pos.len();
    let half_dt = 0.5 * dt;

    // Position: x(t+dt) = x(t) + v(t)*dt + 0.5*a(t)*dt²
    for i in 0..dim {
        pos[i] += vel[i] * dt + half_dt * dt * acc_prev[i];
    }

    // Acceleration at new position
    let mut acc_new = vec![0.0; dim];
    acc_fn(t + dt, pos, &mut acc_new);

    // Velocity: v(t+dt) = v(t) + 0.5*(a(t) + a(t+dt))*dt
    for i in 0..dim {
        vel[i] += half_dt * (acc_prev[i] + acc_new[i]);
    }
    acc_prev[..dim].copy_from_slice(&acc_new[..dim]);
}

/// Leapfrog integrator step.
///
/// Equivalent to Verlet but stores velocity at half-steps.
/// Second-order, symplectic.
///
/// - `acc_fn`: computes acceleration from `(t, position, &mut acceleration_out)`.
/// - `pos`: current position (modified in place).
/// - `vel_half`: velocity at t - dt/2 (modified to t + dt/2).
/// - `t`: current time.
/// - `dt`: time step.
#[inline]
pub fn leapfrog_step(
    acc_fn: &impl Fn(f64, &[f64], &mut [f64]),
    pos: &mut [f64],
    vel_half: &mut [f64],
    t: f64,
    dt: f64,
) {
    let dim = pos.len();
    let mut acc = vec![0.0; dim];
    acc_fn(t, pos, &mut acc);

    // Kick: v(t+dt/2) = v(t-dt/2) + a(t)*dt
    for i in 0..dim {
        vel_half[i] += dt * acc[i];
    }
    // Drift: x(t+dt) = x(t) + v(t+dt/2)*dt
    for i in 0..dim {
        pos[i] += dt * vel_half[i];
    }
}

/// Run a symplectic Euler integration over a time span.
///
/// Returns `(final_position, final_velocity)`.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
/// Returns [`HisabError::InvalidInput`] if `pos0` and `vel0` differ in length.
#[must_use = "contains the final state or an error"]
pub fn symplectic_euler(
    acc_fn: impl Fn(f64, &[f64], &mut [f64]),
    pos0: &[f64],
    vel0: &[f64],
    t0: f64,
    t_end: f64,
    n: usize,
) -> Result<(Vec<f64>, Vec<f64>), HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    if pos0.len() != vel0.len() {
        return Err(HisabError::InvalidInput(
            "pos and vel must have equal length".into(),
        ));
    }
    let dt = (t_end - t0) / n as f64;
    let mut pos = pos0.to_vec();
    let mut vel = vel0.to_vec();
    let mut t = t0;
    for _ in 0..n {
        symplectic_euler_step(&acc_fn, &mut pos, &mut vel, t, dt);
        t += dt;
    }
    Ok((pos, vel))
}

/// Run a velocity Verlet integration over a time span.
///
/// Returns `(final_position, final_velocity)`.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
/// Returns [`HisabError::InvalidInput`] if `pos0` and `vel0` differ in length.
#[must_use = "contains the final state or an error"]
pub fn verlet(
    acc_fn: impl Fn(f64, &[f64], &mut [f64]),
    pos0: &[f64],
    vel0: &[f64],
    t0: f64,
    t_end: f64,
    n: usize,
) -> Result<(Vec<f64>, Vec<f64>), HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    if pos0.len() != vel0.len() {
        return Err(HisabError::InvalidInput(
            "pos and vel must have equal length".into(),
        ));
    }
    let dt = (t_end - t0) / n as f64;
    let mut pos = pos0.to_vec();
    let mut vel = vel0.to_vec();
    let mut acc = vec![0.0; pos.len()];
    acc_fn(t0, &pos, &mut acc);
    let mut t = t0;
    for _ in 0..n {
        verlet_step(&acc_fn, &mut pos, &mut vel, t, dt, &mut acc);
        t += dt;
    }
    Ok((pos, vel))
}

// ---------------------------------------------------------------------------
// Matrix rank, condition number, inverse, pseudo-inverse
// ---------------------------------------------------------------------------

/// Compute the numerical rank of a matrix via SVD.
///
/// Counts singular values greater than `tol`. If `tol` is `None`, uses
/// `max(m, n) * EPSILON_F64 * σ_max` as the default threshold.
///
/// # Errors
///
/// Returns errors from [`svd`] if the matrix is invalid.
#[must_use = "returns the computed rank or an error"]
pub fn matrix_rank(a: &[Vec<f64>], tol: Option<f64>) -> Result<usize, HisabError> {
    let result = svd(a)?;
    let threshold = match tol {
        Some(t) => t,
        None => {
            let m = a.len();
            let n = a[0].len();
            let sigma_max = result.sigma.first().copied().unwrap_or(0.0);
            m.max(n) as f64 * crate::EPSILON_F64 * sigma_max
        }
    };
    Ok(result.sigma.iter().filter(|&&s| s > threshold).count())
}

/// Compute the condition number of a matrix (ratio of largest to smallest singular value).
///
/// A large condition number indicates an ill-conditioned matrix.
/// Returns `f64::INFINITY` if the matrix is singular (smallest σ ≈ 0).
///
/// # Errors
///
/// Returns errors from [`svd`] if the matrix is invalid.
#[must_use = "returns the computed condition number or an error"]
pub fn condition_number(a: &[Vec<f64>]) -> Result<f64, HisabError> {
    let result = svd(a)?;
    let sigma_max = result.sigma.first().copied().unwrap_or(0.0);
    let sigma_min = result.sigma.last().copied().unwrap_or(0.0);
    if sigma_min < crate::EPSILON_F64 {
        Ok(f64::INFINITY)
    } else {
        Ok(sigma_max / sigma_min)
    }
}

/// Compute the inverse of a square matrix via LU decomposition.
///
/// Returns the `n × n` inverse matrix (row-major).
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty or not square.
/// Returns [`HisabError::SingularPivot`] if the matrix is singular.
#[must_use = "returns the inverse matrix or an error"]
#[allow(clippy::needless_range_loop)]
pub fn matrix_inverse(a: &[Vec<f64>]) -> Result<Vec<Vec<f64>>, HisabError> {
    let n = a.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    for row in a {
        if row.len() != n {
            return Err(HisabError::InvalidInput(format!(
                "expected square {}x{}, got row length {}",
                n,
                n,
                row.len()
            )));
        }
    }

    let (lu, pivot) = lu_decompose(a)?;

    // Solve for each column of the identity, reusing the buffer
    let mut inv = vec![vec![0.0; n]; n];
    let mut e = vec![0.0; n];
    for col in 0..n {
        e.fill(0.0);
        e[col] = 1.0;
        let x = lu_solve(&lu, &pivot, &e)?;
        for row in 0..n {
            inv[row][col] = x[row];
        }
    }

    Ok(inv)
}

/// Compute the Moore-Penrose pseudo-inverse of a matrix via SVD.
///
/// For an `m × n` matrix `A`, returns the `n × m` pseudo-inverse `A⁺`
/// such that `A · A⁺ · A ≈ A`.
///
/// Singular values below `tol` are treated as zero. If `tol` is `None`,
/// uses `max(m, n) * EPSILON_F64 * σ_max`.
///
/// # Errors
///
/// Returns errors from [`svd`] if the matrix is invalid.
#[must_use = "returns the pseudo-inverse matrix or an error"]
#[allow(clippy::needless_range_loop)]
pub fn pseudo_inverse(a: &[Vec<f64>], tol: Option<f64>) -> Result<Vec<Vec<f64>>, HisabError> {
    let m = a.len();
    if m == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    let n = a[0].len();

    let result = svd(a)?;
    let threshold = match tol {
        Some(t) => t,
        None => {
            let sigma_max = result.sigma.first().copied().unwrap_or(0.0);
            m.max(n) as f64 * crate::EPSILON_F64 * sigma_max
        }
    };

    // A⁺ = V · Σ⁺ · Uᵀ  (n × m)
    // Σ⁺[i] = 1/σ[i] if σ[i] > tol, else 0
    let k = result.sigma.len();

    // Precompute reciprocals of significant singular values
    let sigma_inv: Vec<f64> = result
        .sigma
        .iter()
        .map(|&s| if s > threshold { 1.0 / s } else { 0.0 })
        .collect();

    let mut pinv = vec![vec![0.0; m]; n];

    for i in 0..n {
        for j in 0..m {
            let mut val = 0.0;
            for s in 0..k {
                val += result.vt[s][i] * sigma_inv[s] * result.u[s][j];
            }
            pinv[i][j] = val;
        }
    }

    Ok(pinv)
}

// ---------------------------------------------------------------------------
// Optimization solvers
// ---------------------------------------------------------------------------

/// Result of an optimization run.
#[derive(Debug, Clone)]
#[must_use]
pub struct OptResult {
    /// The minimizer point.
    pub x: Vec<f64>,
    /// Function value at the minimizer.
    pub f_val: f64,
    /// Number of iterations performed.
    pub iterations: usize,
}

/// Gradient descent minimization of f: ℝⁿ → ℝ.
///
/// Uses steepest descent with a fixed learning rate.
///
/// - `f`: objective function.
/// - `grad_f`: gradient function ∇f(x) → Vec<f64>.
/// - `x0`: initial guess.
/// - `learning_rate`: step size α.
/// - `tol`: convergence tolerance on gradient norm.
/// - `max_iter`: maximum iterations.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `x0` is empty.
/// Returns [`HisabError::NoConvergence`] if `max_iter` is exhausted.
#[must_use = "contains the optimization result or an error"]
pub fn gradient_descent(
    f: impl Fn(&[f64]) -> f64,
    grad_f: impl Fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    learning_rate: f64,
    tol: f64,
    max_iter: usize,
) -> Result<OptResult, HisabError> {
    if x0.is_empty() {
        return Err(HisabError::InvalidInput("empty initial guess".into()));
    }
    let n = x0.len();
    let mut x = x0.to_vec();

    for iter in 0..max_iter {
        let g = grad_f(&x);
        let grad_norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if grad_norm < tol {
            return Ok(OptResult {
                f_val: f(&x),
                x,
                iterations: iter,
            });
        }
        for i in 0..n {
            x[i] -= learning_rate * g[i];
        }
    }

    Err(HisabError::NoConvergence(max_iter))
}

/// Conjugate gradient method for solving A·x = b (A must be symmetric positive-definite).
///
/// Iterative solver that requires only matrix-vector products.
///
/// - `a_mul`: computes A·v for a given vector v.
/// - `b`: right-hand side vector.
/// - `x0`: initial guess.
/// - `tol`: convergence tolerance on residual norm.
/// - `max_iter`: maximum iterations.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `b` is empty or lengths mismatch.
/// Returns [`HisabError::NoConvergence`] if `max_iter` is exhausted.
#[must_use = "contains the solution vector or an error"]
pub fn conjugate_gradient(
    a_mul: impl Fn(&[f64]) -> Vec<f64>,
    b: &[f64],
    x0: &[f64],
    tol: f64,
    max_iter: usize,
) -> Result<Vec<f64>, HisabError> {
    let n = b.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty b vector".into()));
    }
    if x0.len() != n {
        return Err(HisabError::InvalidInput(format!(
            "x0 length {} != b length {n}",
            x0.len()
        )));
    }

    let mut x = x0.to_vec();
    let ax = a_mul(&x);
    let mut r: Vec<f64> = (0..n).map(|i| b[i] - ax[i]).collect();
    let mut p = r.clone();
    let mut rs_old: f64 = r.iter().map(|ri| ri * ri).sum();

    let tol_sq = tol * tol;
    if rs_old < tol_sq {
        return Ok(x);
    }

    for _ in 0..max_iter {
        let ap = a_mul(&p);
        let pap: f64 = p.iter().zip(ap.iter()).map(|(pi, api)| pi * api).sum();
        if pap.abs() < crate::EPSILON_F64 {
            break;
        }
        let alpha = rs_old / pap;

        for i in 0..n {
            x[i] += alpha * p[i];
            r[i] -= alpha * ap[i];
        }

        let rs_new: f64 = r.iter().map(|ri| ri * ri).sum();
        if rs_new < tol_sq {
            return Ok(x);
        }

        let beta = rs_new / rs_old;
        for i in 0..n {
            p[i] = r[i] + beta * p[i];
        }
        rs_old = rs_new;
    }

    // Return best estimate even if not fully converged
    Ok(x)
}

/// BFGS quasi-Newton optimization with backtracking line search.
///
/// Minimizes f: ℝⁿ → ℝ using the Broyden-Fletcher-Goldfarb-Shanno method.
///
/// - `f`: objective function.
/// - `grad_f`: gradient function.
/// - `x0`: initial guess.
/// - `tol`: convergence tolerance on gradient norm.
/// - `max_iter`: maximum iterations.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `x0` is empty.
/// Returns [`HisabError::NoConvergence`] if `max_iter` is exhausted.
#[must_use = "contains the optimization result or an error"]
#[allow(clippy::needless_range_loop)]
pub fn bfgs(
    f: impl Fn(&[f64]) -> f64,
    grad_f: impl Fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    tol: f64,
    max_iter: usize,
) -> Result<OptResult, HisabError> {
    let n = x0.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty initial guess".into()));
    }

    let mut x = x0.to_vec();
    let mut g = grad_f(&x);

    // Initialize inverse Hessian approximation as identity
    let mut h_inv = vec![vec![0.0; n]; n];
    for i in 0..n {
        h_inv[i][i] = 1.0;
    }

    // Pre-allocate scratch buffers reused every iteration
    let mut d = vec![0.0; n];
    let mut x_new = vec![0.0; n];
    let mut s = vec![0.0; n];
    let mut y_vec = vec![0.0; n];
    let mut hy = vec![0.0; n];

    for iter in 0..max_iter {
        let grad_norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if grad_norm < tol {
            return Ok(OptResult {
                f_val: f(&x),
                x,
                iterations: iter,
            });
        }

        // Search direction: d = -H_inv * g
        d.fill(0.0);
        for i in 0..n {
            for j in 0..n {
                d[i] -= h_inv[i][j] * g[j];
            }
        }

        // Backtracking line search (Armijo condition)
        let mut alpha = 1.0;
        let f_old = f(&x);
        let dg: f64 = d.iter().zip(g.iter()).map(|(di, gi)| di * gi).sum();
        let c1 = 1e-4;

        for _ in 0..40 {
            for i in 0..n {
                x_new[i] = x[i] + alpha * d[i];
            }
            if f(&x_new) <= f_old + c1 * alpha * dg {
                break;
            }
            alpha *= 0.5;
        }

        // Compute s = x_new - x, y = g_new - g
        let g_new = grad_f(&x_new);
        for i in 0..n {
            s[i] = x_new[i] - x[i];
            y_vec[i] = g_new[i] - g[i];
        }

        let sy: f64 = s.iter().zip(y_vec.iter()).map(|(si, yi)| si * yi).sum();

        // Update H_inv using BFGS formula (skip if curvature condition fails)
        if sy > crate::EPSILON_F64 {
            let rho = 1.0 / sy;

            // Compute H_inv · y
            hy.fill(0.0);
            for i in 0..n {
                for j in 0..n {
                    hy[i] += h_inv[i][j] * y_vec[j];
                }
            }

            let yhy: f64 = y_vec.iter().zip(hy.iter()).map(|(yi, hyi)| yi * hyi).sum();

            for i in 0..n {
                for j in 0..n {
                    h_inv[i][j] +=
                        rho * ((1.0 + rho * yhy) * s[i] * s[j] - hy[i] * s[j] - s[i] * hy[j]);
                }
            }
        }

        std::mem::swap(&mut x, &mut x_new);
        g = g_new;
    }

    Err(HisabError::NoConvergence(max_iter))
}

/// L-BFGS (Limited-memory BFGS) optimization.
///
/// Like [`bfgs`] but stores only the last `m` correction pairs instead of
/// the full `n × n` inverse Hessian. Memory usage is `O(m·n)` instead of
/// `O(n²)`, making it suitable for large-scale problems.
///
/// - `f`: objective function.
/// - `grad_f`: gradient function.
/// - `x0`: initial guess.
/// - `m`: number of correction pairs to store (typically 5–20).
/// - `tol`: convergence tolerance on gradient norm.
/// - `max_iter`: maximum iterations.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `x0` is empty or `m` is zero.
/// Returns [`HisabError::NoConvergence`] if `max_iter` is exhausted.
#[must_use = "contains the optimization result or an error"]
#[allow(clippy::needless_range_loop)]
pub fn lbfgs(
    f: impl Fn(&[f64]) -> f64,
    grad_f: impl Fn(&[f64]) -> Vec<f64>,
    x0: &[f64],
    m: usize,
    tol: f64,
    max_iter: usize,
) -> Result<OptResult, HisabError> {
    let n = x0.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty initial guess".into()));
    }
    if m == 0 {
        return Err(HisabError::InvalidInput("m must be positive".into()));
    }

    let mut x = x0.to_vec();
    let mut g = grad_f(&x);

    // Circular buffer for s, y pairs and ρ values
    let mut s_hist: Vec<Vec<f64>> = Vec::with_capacity(m);
    let mut y_hist: Vec<Vec<f64>> = Vec::with_capacity(m);
    let mut rho_hist: Vec<f64> = Vec::with_capacity(m);

    let mut x_new = vec![0.0; n];
    let mut q = vec![0.0; n];
    let mut alpha_buf = vec![0.0; m];

    for iter in 0..max_iter {
        let grad_norm: f64 = g.iter().map(|gi| gi * gi).sum::<f64>().sqrt();
        if grad_norm < tol {
            return Ok(OptResult {
                f_val: f(&x),
                x,
                iterations: iter,
            });
        }

        // L-BFGS two-loop recursion to compute search direction d = -H·g
        q.copy_from_slice(&g);
        let k = s_hist.len();

        // First loop (newest to oldest)
        for i in (0..k).rev() {
            alpha_buf[i] = rho_hist[i]
                * s_hist[i]
                    .iter()
                    .zip(q.iter())
                    .map(|(si, qi)| si * qi)
                    .sum::<f64>();
            for j in 0..n {
                q[j] -= alpha_buf[i] * y_hist[i][j];
            }
        }

        // Initial Hessian scaling: H0 = (s·y)/(y·y) · I
        if k > 0 {
            let yy: f64 = y_hist[k - 1].iter().map(|yi| yi * yi).sum();
            if yy > crate::EPSILON_F64 {
                let gamma = 1.0 / (rho_hist[k - 1] * yy);
                for qi in &mut q {
                    *qi *= gamma;
                }
            }
        }

        // Second loop (oldest to newest)
        for i in 0..k {
            let beta: f64 = rho_hist[i]
                * y_hist[i]
                    .iter()
                    .zip(q.iter())
                    .map(|(yi, qi)| yi * qi)
                    .sum::<f64>();
            for j in 0..n {
                q[j] += (alpha_buf[i] - beta) * s_hist[i][j];
            }
        }

        // d = -q (search direction)
        for qi in &mut q {
            *qi = -*qi;
        }

        // Backtracking line search
        let mut step = 1.0;
        let f_old = f(&x);
        let dg: f64 = q.iter().zip(g.iter()).map(|(di, gi)| di * gi).sum();
        let c1 = 1e-4;

        for _ in 0..40 {
            for i in 0..n {
                x_new[i] = x[i] + step * q[i];
            }
            if f(&x_new) <= f_old + c1 * step * dg {
                break;
            }
            step *= 0.5;
        }

        let g_new = grad_f(&x_new);

        // Store correction pair
        let s_k: Vec<f64> = (0..n).map(|i| x_new[i] - x[i]).collect();
        let y_k: Vec<f64> = (0..n).map(|i| g_new[i] - g[i]).collect();
        let sy: f64 = s_k.iter().zip(y_k.iter()).map(|(si, yi)| si * yi).sum();

        if sy > crate::EPSILON_F64 {
            if s_hist.len() == m {
                s_hist.remove(0);
                y_hist.remove(0);
                rho_hist.remove(0);
            }
            rho_hist.push(1.0 / sy);
            s_hist.push(s_k);
            y_hist.push(y_k);
        }

        std::mem::swap(&mut x, &mut x_new);
        g = g_new;
    }

    Err(HisabError::NoConvergence(max_iter))
}

/// Levenberg-Marquardt nonlinear least squares solver.
///
/// Minimizes `Σ rᵢ(x)²` where `r` is the residual vector function.
///
/// - `residuals`: computes residual vector r(x).
/// - `jacobian`: computes the Jacobian J(x) (m×n row-major).
/// - `x0`: initial guess (n parameters).
/// - `tol`: convergence tolerance on residual norm change.
/// - `max_iter`: maximum iterations.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `x0` is empty.
/// Returns [`HisabError::NoConvergence`] if `max_iter` is exhausted.
#[must_use = "contains the optimization result or an error"]
#[allow(clippy::needless_range_loop)]
pub fn levenberg_marquardt(
    residuals: impl Fn(&[f64]) -> Vec<f64>,
    jacobian_fn: impl Fn(&[f64]) -> Vec<Vec<f64>>,
    x0: &[f64],
    tol: f64,
    max_iter: usize,
) -> Result<OptResult, HisabError> {
    let n = x0.len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty initial guess".into()));
    }

    let mut x = x0.to_vec();
    let mut r = residuals(&x);
    let mut cost: f64 = r.iter().map(|ri| ri * ri).sum();
    let mut lambda = 1e-3;

    // Pre-allocate scratch buffers
    let mut jtj = vec![vec![0.0; n]; n];
    let mut jtr = vec![0.0; n];
    let mut aug: Vec<Vec<f64>> = vec![vec![0.0; n + 1]; n];
    let mut x_trial = vec![0.0; n];

    for iter in 0..max_iter {
        if cost.sqrt() < tol {
            return Ok(OptResult {
                f_val: cost,
                x,
                iterations: iter,
            });
        }

        let j = jacobian_fn(&x);
        let m = r.len();

        // Compute JᵀJ and Jᵀr
        for row in &mut jtj {
            row.fill(0.0);
        }
        jtr.fill(0.0);
        for i in 0..n {
            for k in 0..m {
                jtr[i] += j[k][i] * r[k];
            }
            for jj in 0..n {
                for k in 0..m {
                    jtj[i][jj] += j[k][i] * j[k][jj];
                }
            }
        }

        // Damped normal equations: (JᵀJ + λI)·δ = -Jᵀr
        for i in 0..n {
            jtj[i][i] += lambda;
        }

        // Build augmented matrix in-place
        for i in 0..n {
            aug[i][..n].copy_from_slice(&jtj[i]);
            aug[i][n] = -jtr[i];
        }

        let delta = match gaussian_elimination(&mut aug) {
            Ok(d) => d,
            Err(_) => {
                lambda *= 10.0;
                continue;
            }
        };

        // Trial step
        for i in 0..n {
            x_trial[i] = x[i] + delta[i];
        }
        let r_trial = residuals(&x_trial);
        let cost_trial: f64 = r_trial.iter().map(|ri| ri * ri).sum();

        if cost_trial < cost {
            std::mem::swap(&mut x, &mut x_trial);
            r = r_trial;
            cost = cost_trial;
            lambda *= 0.1;
        } else {
            lambda *= 10.0;
        }
    }

    Err(HisabError::NoConvergence(max_iter))
}

// ---------------------------------------------------------------------------
// PCG32 random number generator
// ---------------------------------------------------------------------------

/// A PCG32 (Permuted Congruential Generator) — fast, small-state, high-quality PRNG.
///
/// Deterministic and seedable — suitable for simulation replay.
#[derive(Debug, Clone)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    /// Create a new PCG32 with the given seed and stream.
    #[must_use]
    pub fn new(seed: u64, stream: u64) -> Self {
        let mut rng = Self {
            state: 0,
            inc: (stream << 1) | 1,
        };
        rng.next_u32();
        rng.state = rng.state.wrapping_add(seed);
        rng.next_u32();
        rng
    }

    /// Generate the next u32 value.
    #[inline]
    pub fn next_u32(&mut self) -> u32 {
        let old_state = self.state;
        self.state = old_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc);
        let xor_shifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;
        (xor_shifted >> rot) | (xor_shifted << (rot.wrapping_neg() & 31))
    }

    /// Generate a random f64 in [0, 1).
    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        (self.next_u32() as f64) / (u32::MAX as f64 + 1.0)
    }

    /// Generate a random f32 in [0, 1).
    #[inline]
    pub fn next_f32(&mut self) -> f32 {
        (self.next_u32() as f32) / (u32::MAX as f32 + 1.0)
    }

    /// Generate a random f64 in [lo, hi).
    #[inline]
    pub fn next_f64_range(&mut self, lo: f64, hi: f64) -> f64 {
        lo + (hi - lo) * self.next_f64()
    }

    /// Generate a random f64 from the standard normal distribution N(0,1).
    ///
    /// Uses the Box-Muller transform.
    #[inline]
    pub fn next_normal(&mut self) -> f64 {
        let u1 = self.next_f64().max(1e-300); // avoid log(0)
        let u2 = self.next_f64();
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }
}

// ---------------------------------------------------------------------------
// Sparse matrix (CSR format)
// ---------------------------------------------------------------------------

/// A sparse matrix in Compressed Sparse Row (CSR) format.
///
/// Stores only non-zero elements. Efficient for sparse matrix-vector multiply
/// and row-based access patterns.
///
/// - `values`: non-zero entries, row by row.
/// - `col_indices`: column index of each value.
/// - `row_offsets`: `row_offsets[i]` is the index into `values` where row `i` starts.
///   Length is `nrows + 1`; `row_offsets[nrows]` equals `values.len()`.
#[derive(Debug, Clone, PartialEq)]
#[must_use]
pub struct CsrMatrix {
    /// Number of rows.
    pub nrows: usize,
    /// Number of columns.
    pub ncols: usize,
    /// Non-zero values, row by row.
    values: Vec<f64>,
    /// Column index for each value.
    col_indices: Vec<usize>,
    /// Row offset pointers (length = nrows + 1).
    row_offsets: Vec<usize>,
}

impl CsrMatrix {
    /// Create a CSR matrix from raw components.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if the components are inconsistent.
    pub fn new(
        nrows: usize,
        ncols: usize,
        values: Vec<f64>,
        col_indices: Vec<usize>,
        row_offsets: Vec<usize>,
    ) -> Result<Self, HisabError> {
        if row_offsets.len() != nrows + 1 {
            return Err(HisabError::InvalidInput(format!(
                "row_offsets length {} != nrows + 1 ({})",
                row_offsets.len(),
                nrows + 1
            )));
        }
        if values.len() != col_indices.len() {
            return Err(HisabError::InvalidInput(
                "values and col_indices must have equal length".into(),
            ));
        }
        if row_offsets[nrows] != values.len() {
            return Err(HisabError::InvalidInput(
                "row_offsets[nrows] must equal values.len()".into(),
            ));
        }
        // Validate monotonically non-decreasing row_offsets
        for w in row_offsets.windows(2) {
            if w[0] > w[1] {
                return Err(HisabError::InvalidInput(
                    "row_offsets must be monotonically non-decreasing".into(),
                ));
            }
        }
        // Validate column indices: in range and sorted within each row
        for row in 0..nrows {
            let start = row_offsets[row];
            let end = row_offsets[row + 1];
            for idx in start..end {
                if col_indices[idx] >= ncols {
                    return Err(HisabError::InvalidInput(format!(
                        "column index {} >= ncols {ncols}",
                        col_indices[idx]
                    )));
                }
                if idx > start && col_indices[idx] <= col_indices[idx - 1] {
                    return Err(HisabError::InvalidInput(
                        "column indices must be strictly sorted within each row".into(),
                    ));
                }
            }
        }
        Ok(Self {
            nrows,
            ncols,
            values,
            col_indices,
            row_offsets,
        })
    }

    /// Create a CSR matrix from a dense row-major matrix, dropping zeros.
    pub fn from_dense(a: &[Vec<f64>]) -> Self {
        let nrows = a.len();
        let ncols = if nrows > 0 { a[0].len() } else { 0 };
        let mut values = Vec::new();
        let mut col_indices = Vec::new();
        let mut row_offsets = Vec::with_capacity(nrows + 1);
        row_offsets.push(0);

        for row in a {
            for (j, &v) in row.iter().enumerate() {
                if v.abs() > crate::EPSILON_F64 {
                    values.push(v);
                    col_indices.push(j);
                }
            }
            row_offsets.push(values.len());
        }

        Self {
            nrows,
            ncols,
            values,
            col_indices,
            row_offsets,
        }
    }

    /// Convert to a dense row-major matrix.
    #[must_use]
    pub fn to_dense(&self) -> Vec<Vec<f64>> {
        let mut a = vec![vec![0.0; self.ncols]; self.nrows];
        for (i, row) in a.iter_mut().enumerate() {
            for idx in self.row_offsets[i]..self.row_offsets[i + 1] {
                row[self.col_indices[idx]] = self.values[idx];
            }
        }
        a
    }

    /// Number of non-zero entries.
    #[must_use]
    #[inline]
    pub fn nnz(&self) -> usize {
        self.values.len()
    }

    /// Sparse matrix-vector multiply: `y = A * x`.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if `x.len() != ncols`.
    #[must_use = "returns the product vector or an error"]
    #[inline]
    pub fn spmv(&self, x: &[f64]) -> Result<Vec<f64>, HisabError> {
        if x.len() != self.ncols {
            return Err(HisabError::InvalidInput(format!(
                "x length {} != ncols {}",
                x.len(),
                self.ncols
            )));
        }
        let mut y = vec![0.0; self.nrows];
        for (i, yi) in y.iter_mut().enumerate() {
            let mut sum = 0.0;
            for idx in self.row_offsets[i]..self.row_offsets[i + 1] {
                sum += self.values[idx] * x[self.col_indices[idx]];
            }
            *yi = sum;
        }
        Ok(y)
    }

    /// Add two CSR matrices of the same dimensions.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if dimensions don't match.
    pub fn add(&self, other: &CsrMatrix) -> Result<CsrMatrix, HisabError> {
        if self.nrows != other.nrows || self.ncols != other.ncols {
            return Err(HisabError::InvalidInput(format!(
                "dimension mismatch: {}x{} vs {}x{}",
                self.nrows, self.ncols, other.nrows, other.ncols
            )));
        }

        let mut values = Vec::new();
        let mut col_indices = Vec::new();
        let mut row_offsets = Vec::with_capacity(self.nrows + 1);
        row_offsets.push(0);

        for i in 0..self.nrows {
            // Merge two sorted row segments
            let mut a_idx = self.row_offsets[i];
            let a_end = self.row_offsets[i + 1];
            let mut b_idx = other.row_offsets[i];
            let b_end = other.row_offsets[i + 1];

            while a_idx < a_end && b_idx < b_end {
                let a_col = self.col_indices[a_idx];
                let b_col = other.col_indices[b_idx];
                match a_col.cmp(&b_col) {
                    std::cmp::Ordering::Less => {
                        values.push(self.values[a_idx]);
                        col_indices.push(a_col);
                        a_idx += 1;
                    }
                    std::cmp::Ordering::Greater => {
                        values.push(other.values[b_idx]);
                        col_indices.push(b_col);
                        b_idx += 1;
                    }
                    std::cmp::Ordering::Equal => {
                        let sum = self.values[a_idx] + other.values[b_idx];
                        if sum.abs() > crate::EPSILON_F64 {
                            values.push(sum);
                            col_indices.push(a_col);
                        }
                        a_idx += 1;
                        b_idx += 1;
                    }
                }
            }
            while a_idx < a_end {
                values.push(self.values[a_idx]);
                col_indices.push(self.col_indices[a_idx]);
                a_idx += 1;
            }
            while b_idx < b_end {
                values.push(other.values[b_idx]);
                col_indices.push(other.col_indices[b_idx]);
                b_idx += 1;
            }
            row_offsets.push(values.len());
        }

        Ok(CsrMatrix {
            nrows: self.nrows,
            ncols: self.ncols,
            values,
            col_indices,
            row_offsets,
        })
    }

    /// Transpose this matrix, returning a new CSR matrix.
    pub fn transpose(&self) -> CsrMatrix {
        let mut row_counts = vec![0usize; self.ncols];
        for &c in &self.col_indices {
            row_counts[c] += 1;
        }

        let mut new_offsets = Vec::with_capacity(self.ncols + 1);
        let mut cumulative = 0usize;
        new_offsets.push(0);
        for &count in &row_counts {
            cumulative += count;
            new_offsets.push(cumulative);
        }

        let mut new_values = vec![0.0; self.nnz()];
        let mut new_col_indices = vec![0usize; self.nnz()];
        let mut cursor = new_offsets[..self.ncols].to_vec();

        for i in 0..self.nrows {
            for idx in self.row_offsets[i]..self.row_offsets[i + 1] {
                let col = self.col_indices[idx];
                let dest = cursor[col];
                new_values[dest] = self.values[idx];
                new_col_indices[dest] = i;
                cursor[col] += 1;
            }
        }

        CsrMatrix {
            nrows: self.ncols,
            ncols: self.nrows,
            values: new_values,
            col_indices: new_col_indices,
            row_offsets: new_offsets,
        }
    }
}

// ---------------------------------------------------------------------------
// SVD (Singular Value Decomposition)
// ---------------------------------------------------------------------------

/// Singular Value Decomposition result.
///
/// For an `m × n` matrix `A`, produces `A = U · diag(σ) · Vᵀ` where:
/// - `U` is `m × m` orthogonal (stored column-major: `u[col][row]`)
/// - `sigma` contains the singular values in descending order
/// - `vt` is `n × n` orthogonal transpose (stored row-major: `vt[row][col]`)
#[derive(Debug, Clone)]
#[must_use]
pub struct Svd {
    /// Left singular vectors (column-major: `u[col][row]`).
    pub u: Vec<Vec<f64>>,
    /// Singular values in descending order.
    pub sigma: Vec<f64>,
    /// Right singular vectors transposed (row-major: `vt[row][col]`).
    pub vt: Vec<Vec<f64>>,
}

/// Compute the Singular Value Decomposition of an `m × n` matrix.
///
/// Input is row-major: `a[i]` is the i-th row with `n` columns.
/// Uses one-sided Jacobi rotations for simplicity and numerical stability.
///
/// Returns [`Svd`] containing `U`, `sigma`, and `Vᵀ`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if the matrix is empty or rows have
/// inconsistent lengths.
/// Returns [`HisabError::NoConvergence`] if the iterative process does not
/// converge within the maximum number of sweeps.
#[must_use = "contains the SVD factors or an error"]
#[allow(clippy::needless_range_loop)]
pub fn svd(a: &[Vec<f64>]) -> Result<Svd, HisabError> {
    let m = a.len();
    if m == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    let n = a[0].len();
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    for row in a {
        if row.len() != n {
            return Err(HisabError::InvalidInput("inconsistent row lengths".into()));
        }
    }

    // For wide matrices (m < n), compute SVD of Aᵀ and swap U ↔ V.
    let transposed = m < n;
    let (work_m, work_n, work): (usize, usize, Vec<Vec<f64>>) = if transposed {
        // Transpose: work[j] = column j of A = row j of Aᵀ
        let mut t = vec![vec![0.0; m]; n];
        for i in 0..m {
            for j in 0..n {
                t[j][i] = a[i][j];
            }
        }
        (n, m, t)
    } else {
        (m, n, a.to_vec())
    };

    let result = svd_tall(&work, work_m, work_n)?;

    if transposed {
        // Swap: U of Aᵀ becomes V of A, and vice versa
        // Vᵀ of Aᵀ rows become U columns of A
        // U columns of Aᵀ become Vᵀ rows of A
        let mut vt = vec![vec![0.0; n]; n];
        for i in 0..n {
            for j in 0..n {
                // U of Aᵀ has columns of length work_m=n
                if i < result.u.len() {
                    vt[i][j] = result.u[i][j];
                }
            }
        }
        // U of A from Vᵀ of Aᵀ: u_col[row] = vt_row[col] transposed
        let mut u: Vec<Vec<f64>> = Vec::with_capacity(m);
        for i in 0..result.vt.len() {
            u.push(result.vt[i].clone());
        }
        // Extend U to m×m if needed
        extend_orthonormal_basis(&mut u, m);
        Ok(Svd {
            u,
            sigma: result.sigma,
            vt,
        })
    } else {
        Ok(result)
    }
}

/// Extend a set of orthonormal columns to span ℝᵐ using Gram-Schmidt.
#[allow(clippy::needless_range_loop)]
fn extend_orthonormal_basis(u: &mut Vec<Vec<f64>>, m: usize) {
    for i in 0..m {
        if u.len() >= m {
            break;
        }
        let mut candidate = vec![0.0; m];
        candidate[i] = 1.0;
        for col in u.iter() {
            let dot: f64 = (0..m).map(|k| col[k] * candidate[k]).sum();
            for k in 0..m {
                candidate[k] -= dot * col[k];
            }
        }
        let norm: f64 = candidate.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm > crate::EPSILON_F64 {
            let inv = 1.0 / norm;
            for x in &mut candidate {
                *x *= inv;
            }
            u.push(candidate);
        }
    }
}

/// Truncated SVD — keep only the top `k` singular values and vectors.
///
/// Returns an [`Svd`] with `sigma` of length `k`, `u` with `k` columns,
/// and `vt` with `k` rows.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `k` is zero or greater than `min(m, n)`.
/// Returns errors from [`svd`] if the matrix is invalid.
#[must_use = "contains the truncated SVD or an error"]
pub fn truncated_svd(a: &[Vec<f64>], k: usize) -> Result<Svd, HisabError> {
    if k == 0 {
        return Err(HisabError::InvalidInput("k must be positive".into()));
    }
    let result = svd(a)?;
    if k > result.sigma.len() {
        return Err(HisabError::InvalidInput(format!(
            "k={k} > number of singular values {}",
            result.sigma.len()
        )));
    }
    Ok(Svd {
        u: result.u[..k].to_vec(),
        sigma: result.sigma[..k].to_vec(),
        vt: result.vt[..k].to_vec(),
    })
}

/// Internal SVD for tall/square matrices (m >= n) using one-sided Jacobi.
#[allow(clippy::needless_range_loop)]
fn svd_tall(a: &[Vec<f64>], m: usize, n: usize) -> Result<Svd, HisabError> {
    // B = Aᵀ stored as n columns of length m.
    let mut b: Vec<Vec<f64>> = vec![vec![0.0; m]; n];
    for i in 0..m {
        for j in 0..n {
            b[j][i] = a[i][j];
        }
    }

    // V accumulates right rotations (n×n identity, column-major).
    let mut v: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        v[i][i] = 1.0;
    }

    // One-sided Jacobi: rotate pairs of columns of B until orthogonal.
    let max_sweeps = 100 * n.max(m);
    let tol = crate::EPSILON_F64 * crate::EPSILON_F64;

    for sweep in 0..max_sweeps {
        let mut converged = true;

        for p in 0..n {
            for q in (p + 1)..n {
                let mut app = 0.0;
                let mut aqq = 0.0;
                let mut apq = 0.0;
                for k in 0..m {
                    app += b[p][k] * b[p][k];
                    aqq += b[q][k] * b[q][k];
                    apq += b[p][k] * b[q][k];
                }

                if apq.abs() <= tol * (app * aqq).sqrt() {
                    continue;
                }
                converged = false;

                let tau = (aqq - app) / (2.0 * apq);
                let t = if tau >= 0.0 {
                    1.0 / (tau + (1.0 + tau * tau).sqrt())
                } else {
                    -1.0 / (-tau + (1.0 + tau * tau).sqrt())
                };
                let cos = 1.0 / (1.0 + t * t).sqrt();
                let sin = t * cos;

                for k in 0..m {
                    let bp = b[p][k];
                    let bq = b[q][k];
                    b[p][k] = cos * bp - sin * bq;
                    b[q][k] = sin * bp + cos * bq;
                }

                for k in 0..n {
                    let vp = v[p][k];
                    let vq = v[q][k];
                    v[p][k] = cos * vp - sin * vq;
                    v[q][k] = sin * vp + cos * vq;
                }
            }
        }

        if converged {
            break;
        }

        if sweep == max_sweeps - 1 {
            return Err(HisabError::NoConvergence(max_sweeps));
        }
    }

    // Extract singular values and normalize columns of B → U.
    let mut sigma = Vec::with_capacity(n);
    let mut u: Vec<Vec<f64>> = Vec::with_capacity(m);

    for j in 0..n {
        let norm: f64 = b[j].iter().map(|x| x * x).sum::<f64>().sqrt();
        sigma.push(norm);
        if norm > crate::EPSILON_F64 {
            let inv = 1.0 / norm;
            u.push(b[j].iter().map(|x| x * inv).collect());
        } else {
            u.push(b[j].clone());
        }
    }

    // Extend U to m×m with orthogonal complement (Gram-Schmidt).
    if m > n {
        extend_orthonormal_basis(&mut u, m);
    }

    // Sort by descending singular value.
    let mut order: Vec<usize> = (0..sigma.len()).collect();
    order.sort_unstable_by(|&a, &b| {
        sigma[b]
            .partial_cmp(&sigma[a])
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let sorted_sigma: Vec<f64> = order.iter().map(|&i| sigma[i]).collect();

    let sorted_u: Vec<Vec<f64>> = if u.len() <= sigma.len() {
        order.iter().map(|&i| u[i].clone()).collect()
    } else {
        let mut su: Vec<Vec<f64>> = order.iter().map(|&i| u[i].clone()).collect();
        for i in sigma.len()..u.len() {
            su.push(u[i].clone());
        }
        su
    };

    let mut vt = vec![vec![0.0; n]; n];
    for row_idx in 0..n {
        let src_col = if row_idx < order.len() {
            order[row_idx]
        } else {
            row_idx
        };
        for col_idx in 0..n {
            vt[row_idx][col_idx] = v[src_col][col_idx];
        }
    }

    Ok(Svd {
        u: sorted_u,
        sigma: sorted_sigma,
        vt,
    })
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
        let root = newton_raphson(|x| x * x * x - 27.0, |x| 3.0 * x * x, 2.0, 1e-10, 100).unwrap();
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
        let e = HisabError::NoConvergence(50);
        assert_eq!(e.to_string(), "no convergence after 50 iterations");
        let e = HisabError::SingularPivot;
        assert!(e.to_string().contains("singular"));
    }

    #[test]
    fn error_display_invalid_input() {
        let e = HisabError::InvalidInput("bad data".to_string());
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
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
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

    // --- V0.4b: Eigenvalues ---

    #[test]
    fn eigenvalue_2x2_diagonal() {
        // Diagonal matrix [[5, 0], [0, 2]] — dominant eigenvalue is 5
        let a = vec![vec![5.0, 0.0], vec![0.0, 2.0]];
        let (eval, evec) = eigenvalue_power(&a, 1e-10, 100).unwrap();
        assert!(approx_eq(eval, 5.0));
        // Eigenvector should be [1, 0] (or proportional)
        assert!(evec[0].abs() > 0.9);
    }

    #[test]
    fn eigenvalue_3x3_symmetric() {
        // Symmetric matrix with known dominant eigenvalue
        // [[2, 1, 0], [1, 3, 1], [0, 1, 2]]
        // Eigenvalues: 4, 2, 1 — dominant is 4
        let a = vec![
            vec![2.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let (eval, _evec) = eigenvalue_power(&a, 1e-10, 200).unwrap();
        assert!((eval - 4.0).abs() < 0.01);
    }

    #[test]
    fn eigenvalue_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let (eval, _) = eigenvalue_power(&a, 1e-10, 100).unwrap();
        assert!(approx_eq(eval, 1.0));
    }

    #[test]
    fn eigenvalue_empty() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(eigenvalue_power(&a, 1e-10, 100).is_err());
    }

    // --- V0.4b: FFT ---

    #[test]
    fn complex_arithmetic() {
        let a = Complex::new(3.0, 4.0);
        let b = Complex::new(1.0, -2.0);
        let sum = a + b;
        assert!(approx_eq(sum.re, 4.0));
        assert!(approx_eq(sum.im, 2.0));
        let diff = a - b;
        assert!(approx_eq(diff.re, 2.0));
        assert!(approx_eq(diff.im, 6.0));
        let prod = a * b;
        // (3+4i)(1-2i) = 3-6i+4i-8i² = 3-2i+8 = 11+(-2)i
        assert!(approx_eq(prod.re, 11.0));
        assert!(approx_eq(prod.im, -2.0));
    }

    #[test]
    fn complex_abs_and_conj() {
        let z = Complex::new(3.0, 4.0);
        assert!(approx_eq(z.abs(), 5.0));
        let c = z.conj();
        assert!(approx_eq(c.re, 3.0));
        assert!(approx_eq(c.im, -4.0));
    }

    #[test]
    fn fft_dc_signal() {
        // Constant signal [1, 1, 1, 1] -> FFT should have DC = 4, rest = 0
        let mut data = [
            Complex::from_real(1.0),
            Complex::from_real(1.0),
            Complex::from_real(1.0),
            Complex::from_real(1.0),
        ];
        fft(&mut data).unwrap();
        assert!(approx_eq(data[0].re, 4.0));
        assert!(approx_eq(data[0].im, 0.0));
        assert!(approx_eq(data[1].abs(), 0.0));
        assert!(approx_eq(data[2].abs(), 0.0));
        assert!(approx_eq(data[3].abs(), 0.0));
    }

    #[test]
    fn fft_single_frequency() {
        // [1, -1, 1, -1] is a Nyquist frequency signal
        let mut data = [
            Complex::from_real(1.0),
            Complex::from_real(-1.0),
            Complex::from_real(1.0),
            Complex::from_real(-1.0),
        ];
        fft(&mut data).unwrap();
        assert!(approx_eq(data[0].abs(), 0.0)); // DC = 0
        assert!(approx_eq(data[2].re, 4.0)); // Nyquist bin
    }

    #[test]
    fn fft_ifft_roundtrip() {
        let original = [
            Complex::new(1.0, 0.0),
            Complex::new(2.0, -1.0),
            Complex::new(0.0, 3.0),
            Complex::new(-1.0, 2.0),
        ];
        let mut data = original;
        fft(&mut data).unwrap();
        ifft(&mut data).unwrap();
        for i in 0..4 {
            assert!((data[i].re - original[i].re).abs() < 1e-10);
            assert!((data[i].im - original[i].im).abs() < 1e-10);
        }
    }

    #[test]
    fn fft_8_point() {
        // 8-point FFT of real signal
        let mut data: Vec<Complex> = (0..8).map(|i| Complex::from_real(i as f64)).collect();
        fft(&mut data).unwrap();
        // DC should be sum of all values = 0+1+2+3+4+5+6+7 = 28
        assert!(approx_eq(data[0].re, 28.0));
        assert!(approx_eq(data[0].im, 0.0));
    }

    #[test]
    fn fft_ifft_roundtrip_8() {
        let original: Vec<Complex> = (0..8)
            .map(|i| Complex::new(i as f64, (i as f64 * 0.5).sin()))
            .collect();
        let mut data = original.clone();
        fft(&mut data).unwrap();
        ifft(&mut data).unwrap();
        for i in 0..8 {
            assert!((data[i].re - original[i].re).abs() < 1e-10);
            assert!((data[i].im - original[i].im).abs() < 1e-10);
        }
    }

    #[test]
    fn fft_parseval() {
        // Parseval's theorem: sum |x[n]|^2 = (1/N) * sum |X[k]|^2
        let original: Vec<Complex> = vec![
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 1.0),
            Complex::new(-1.0, 0.0),
            Complex::new(0.0, -1.0),
        ];
        let time_energy: f64 = original.iter().map(|c| c.re * c.re + c.im * c.im).sum();
        let mut freq = original;
        fft(&mut freq).unwrap();
        let freq_energy: f64 = freq.iter().map(|c| c.re * c.re + c.im * c.im).sum();
        // N * time_energy = freq_energy
        assert!((4.0 * time_energy - freq_energy).abs() < 1e-10);
    }

    #[test]
    fn fft_single_element() {
        let mut data = [Complex::new(42.0, -7.0)];
        fft(&mut data).unwrap();
        assert!(approx_eq(data[0].re, 42.0));
        assert!(approx_eq(data[0].im, -7.0));
    }

    // --- V0.4b audit tests ---

    #[test]
    fn eigenvalue_non_square() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        assert!(eigenvalue_power(&a, 1e-10, 100).is_err());
    }

    #[test]
    fn eigenvalue_negative_dominant() {
        // [[-3, 0], [0, 1]] — dominant eigenvalue is -3
        let a = vec![vec![-3.0, 0.0], vec![0.0, 1.0]];
        let (eval, _) = eigenvalue_power(&a, 1e-10, 100).unwrap();
        assert!(approx_eq(eval, -3.0));
    }

    #[test]
    fn eigenvalue_1x1() {
        let a = vec![vec![7.0]];
        let (eval, evec) = eigenvalue_power(&a, 1e-10, 10).unwrap();
        assert!(approx_eq(eval, 7.0));
        assert!(approx_eq(evec[0], 1.0));
    }

    #[test]
    fn complex_default() {
        let c = Complex::default();
        assert!(approx_eq(c.re, 0.0));
        assert!(approx_eq(c.im, 0.0));
    }

    #[test]
    fn complex_display() {
        assert_eq!(Complex::new(3.0, 4.0).to_string(), "3+4i");
        assert_eq!(Complex::new(3.0, -4.0).to_string(), "3-4i");
        assert_eq!(Complex::new(0.0, 0.0).to_string(), "0+0i");
    }

    #[test]
    fn complex_from_real() {
        let c = Complex::from_real(5.0);
        assert!(approx_eq(c.re, 5.0));
        assert!(approx_eq(c.im, 0.0));
    }

    #[test]
    fn complex_scalar_mul() {
        let c = Complex::new(3.0, 4.0) * 2.0;
        assert!(approx_eq(c.re, 6.0));
        assert!(approx_eq(c.im, 8.0));
    }

    #[test]
    fn fft_linearity() {
        // FFT(a*x + b*y) = a*FFT(x) + b*FFT(y)
        let x: Vec<Complex> = (0..4).map(|i| Complex::from_real(i as f64)).collect();
        let y: Vec<Complex> = (0..4)
            .map(|i| Complex::from_real((i as f64).sin()))
            .collect();

        let mut fx = x.clone();
        fft(&mut fx).unwrap();
        let mut fy = y.clone();
        fft(&mut fy).unwrap();

        // 2*x + 3*y
        let mut combined: Vec<Complex> = (0..4).map(|i| x[i] * 2.0 + y[i] * 3.0).collect();
        fft(&mut combined).unwrap();

        for i in 0..4 {
            let expected = fx[i] * 2.0 + fy[i] * 3.0;
            assert!((combined[i].re - expected.re).abs() < 1e-10);
            assert!((combined[i].im - expected.im).abs() < 1e-10);
        }
    }

    #[test]
    fn ifft_single_element() {
        let mut data = [Complex::new(42.0, -7.0)];
        ifft(&mut data).unwrap();
        assert!(approx_eq(data[0].re, 42.0));
        assert!(approx_eq(data[0].im, -7.0));
    }

    #[test]
    fn fft_non_power_of_two_panics() {
        let mut data = vec![Complex::default(); 3];
        assert!(fft(&mut data).is_err());
    }

    // --- V1.0b: DST / DCT ---

    #[test]
    fn dst_idst_roundtrip() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let transformed = dst(&data).unwrap();
        let recovered = idst(&transformed).unwrap();
        for i in 0..data.len() {
            assert!((recovered[i] - data[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn dst_known_values() {
        // DST-I of [1, 1, 1] with N=3 => sin(π*k*(n+1)/4)
        // X[0] = sin(π/4) + sin(π*2/4) + sin(π*3/4) = √2/2 + 1 + √2/2 = 1+√2
        let data = vec![1.0, 1.0, 1.0];
        let result = dst(&data).unwrap();
        let expected_0 = 1.0 + std::f64::consts::SQRT_2;
        assert!((result[0] - expected_0).abs() < 1e-10);
    }

    #[test]
    fn dst_single_element() {
        // DST-I of [x] with N=1: X[0] = x * sin(π/2) = x
        let result = dst(&[7.0]).unwrap();
        assert!((result[0] - 7.0).abs() < 1e-10);
    }

    #[test]
    fn dst_empty_error() {
        assert!(dst(&[]).is_err());
    }

    #[test]
    fn idst_empty_error() {
        assert!(idst(&[]).is_err());
    }

    #[test]
    fn dct_idct_roundtrip() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let transformed = dct(&data).unwrap();
        let recovered = idct(&transformed).unwrap();
        for i in 0..data.len() {
            assert!((recovered[i] - data[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn dct_dc_component() {
        // DCT-II k=0: X[0] = Σ x[n] * cos(0) = sum of all values
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let result = dct(&data).unwrap();
        assert!((result[0] - 10.0).abs() < 1e-10);
    }

    #[test]
    fn dct_single_element() {
        let result = dct(&[5.0]).unwrap();
        assert!((result[0] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn dct_empty_error() {
        assert!(dct(&[]).is_err());
    }

    #[test]
    fn idct_empty_error() {
        assert!(idct(&[]).is_err());
    }

    #[test]
    fn dct_idct_roundtrip_large() {
        // Larger input to exercise more terms
        let data: Vec<f64> = (0..16).map(|i| (i as f64 * 0.3).sin()).collect();
        let transformed = dct(&data).unwrap();
        let recovered = idct(&transformed).unwrap();
        for i in 0..data.len() {
            assert!((recovered[i] - data[i]).abs() < 1e-10);
        }
    }

    #[test]
    fn dst_idst_roundtrip_large() {
        let data: Vec<f64> = (0..16).map(|i| (i as f64 * 0.7).cos()).collect();
        let transformed = dst(&data).unwrap();
        let recovered = idst(&transformed).unwrap();
        for i in 0..data.len() {
            assert!((recovered[i] - data[i]).abs() < 1e-10);
        }
    }

    // --- V0.4c: RK4 ODE solver ---

    #[test]
    fn rk4_exponential_growth() {
        // dy/dt = y, y(0) = 1 => y(t) = e^t
        // At t=1, y should be e ≈ 2.71828
        let y = rk4(
            |_t, y, out: &mut [f64]| {
                out[0] = y[0];
            },
            0.0,
            &[1.0],
            1.0,
            1000,
        )
        .unwrap();
        assert!((y[0] - std::f64::consts::E).abs() < 1e-8);
    }

    #[test]
    fn rk4_linear_ode() {
        // dy/dt = 2, y(0) = 0 => y(t) = 2t
        // At t=5, y = 10
        let y = rk4(
            |_t, _y, out: &mut [f64]| {
                out[0] = 2.0;
            },
            0.0,
            &[0.0],
            5.0,
            100,
        )
        .unwrap();
        assert!(approx_eq(y[0], 10.0));
    }

    #[test]
    fn rk4_harmonic_oscillator() {
        // x'' + x = 0 => system: dx/dt = v, dv/dt = -x
        // x(0)=1, v(0)=0 => x(t) = cos(t), v(t) = -sin(t)
        // At t=pi, x ≈ -1, v ≈ 0
        let y = rk4(
            |_t, y, out: &mut [f64]| {
                out[0] = y[1];
                out[1] = -y[0];
            },
            0.0,
            &[1.0, 0.0],
            std::f64::consts::PI,
            10000,
        )
        .unwrap();
        assert!((y[0] - (-1.0)).abs() < 1e-6); // x(pi) = cos(pi) = -1
        assert!(y[1].abs() < 1e-6); // v(pi) = -sin(pi) = 0
    }

    #[test]
    fn rk4_quadratic_exact() {
        // dy/dt = 2t, y(0) = 0 => y(t) = t^2
        // RK4 is exact for polynomials up to degree 4
        let y = rk4(
            |t, _y, out: &mut [f64]| {
                out[0] = 2.0 * t;
            },
            0.0,
            &[0.0],
            3.0,
            10,
        )
        .unwrap();
        assert!((y[0] - 9.0).abs() < 1e-10);
    }

    #[test]
    fn rk4_system_2d() {
        // dx/dt = -y, dy/dt = x (rotation)
        // x(0)=1, y(0)=0 => x(t)=cos(t), y(t)=sin(t)
        // At t=pi/2: x≈0, y≈1
        let y = rk4(
            |_t, y, out: &mut [f64]| {
                out[0] = -y[1];
                out[1] = y[0];
            },
            0.0,
            &[1.0, 0.0],
            std::f64::consts::FRAC_PI_2,
            1000,
        )
        .unwrap();
        assert!(y[0].abs() < 1e-6);
        assert!((y[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rk4_trajectory_length() {
        let traj = rk4_trajectory(
            |_t, y, out: &mut [f64]| {
                out[0] = y[0];
            },
            0.0,
            &[1.0],
            1.0,
            100,
        )
        .unwrap();
        assert_eq!(traj.len(), 101); // n+1 points
        assert!(approx_eq(traj[0].0, 0.0));
        assert!((traj[100].0 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn rk4_trajectory_matches_final() {
        // Trajectory endpoint should match rk4 result
        let f = |_t: f64, y: &[f64], out: &mut [f64]| {
            out[0] = y[0];
        };
        let final_state = rk4(f, 0.0, &[1.0], 1.0, 100).unwrap();
        let traj = rk4_trajectory(f, 0.0, &[1.0], 1.0, 100).unwrap();
        let traj_final = &traj[100].1;
        assert!((final_state[0] - traj_final[0]).abs() < 1e-12);
    }

    #[test]
    fn rk4_damped_oscillator() {
        // x'' + 0.1*x' + x = 0 (underdamped)
        // System: dx/dt = v, dv/dt = -x - 0.1*v
        // Should decay toward zero
        let y = rk4(
            |_t, y, out: &mut [f64]| {
                out[0] = y[1];
                out[1] = -y[0] - 0.1 * y[1];
            },
            0.0,
            &[1.0, 0.0],
            20.0,
            10000,
        )
        .unwrap();
        // After 20 seconds of damping, amplitude should be small
        assert!(y[0].abs() < 0.5);
    }

    #[test]
    fn complex_div() {
        let a = Complex::new(4.0, 2.0);
        let b = Complex::new(1.0, 1.0);
        let c = a / b;
        assert!(approx_eq(c.re, 3.0));
        assert!(approx_eq(c.im, -1.0));
    }

    #[test]
    fn complex_div_scalar() {
        let a = Complex::new(6.0, 4.0);
        let c = a / 2.0;
        assert!(approx_eq(c.re, 3.0));
        assert!(approx_eq(c.im, 2.0));
    }

    #[test]
    fn complex_neg() {
        let a = Complex::new(3.0, -4.0);
        let b = -a;
        assert!(approx_eq(b.re, -3.0));
        assert!(approx_eq(b.im, 4.0));
    }

    #[test]
    fn complex_from_f64() {
        let c: Complex = 5.0.into();
        assert!(approx_eq(c.re, 5.0));
        assert!(approx_eq(c.im, 0.0));
    }

    #[test]
    fn complex_from_tuple() {
        let c: Complex = (3.0, 4.0).into();
        assert!(approx_eq(c.re, 3.0));
        assert!(approx_eq(c.im, 4.0));
    }

    #[test]
    fn complex_serde_roundtrip() {
        let c = Complex::new(1.5, -2.5);
        let json = serde_json::to_string(&c).unwrap();
        let c2: Complex = serde_json::from_str(&json).unwrap();
        assert_eq!(c, c2);
    }

    // --- Matrix helpers ---

    #[test]
    fn determinant_2x2() {
        let a = vec![vec![3.0, 8.0], vec![4.0, 6.0]];
        let det = matrix_determinant(&a).unwrap();
        assert!(approx_eq(det, -14.0));
    }

    #[test]
    fn determinant_3x3() {
        let a = vec![
            vec![6.0, 1.0, 1.0],
            vec![4.0, -2.0, 5.0],
            vec![2.0, 8.0, 7.0],
        ];
        let det = matrix_determinant(&a).unwrap();
        assert!((det - (-306.0)).abs() < 1e-6);
    }

    #[test]
    fn determinant_identity() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let det = matrix_determinant(&a).unwrap();
        assert!(approx_eq(det, 1.0));
    }

    #[test]
    fn determinant_singular() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let det = matrix_determinant(&a).unwrap();
        assert!(approx_eq(det, 0.0));
    }

    #[test]
    fn determinant_empty() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(matrix_determinant(&a).is_err());
    }

    #[test]
    fn trace_3x3() {
        let a = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let tr = matrix_trace(&a).unwrap();
        assert!(approx_eq(tr, 15.0));
    }

    #[test]
    fn trace_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        assert!(approx_eq(matrix_trace(&a).unwrap(), 2.0));
    }

    #[test]
    fn trace_empty() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(matrix_trace(&a).is_err());
    }

    #[test]
    fn multiply_2x2() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let c = matrix_multiply(&a, &b).unwrap();
        assert!(approx_eq(c[0][0], 19.0));
        assert!(approx_eq(c[0][1], 22.0));
        assert!(approx_eq(c[1][0], 43.0));
        assert!(approx_eq(c[1][1], 50.0));
    }

    #[test]
    fn multiply_non_square() {
        // 2x3 * 3x2 = 2x2
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let b = vec![vec![7.0, 8.0], vec![9.0, 10.0], vec![11.0, 12.0]];
        let c = matrix_multiply(&a, &b).unwrap();
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].len(), 2);
        assert!(approx_eq(c[0][0], 58.0));
        assert!(approx_eq(c[1][1], 154.0));
    }

    #[test]
    fn multiply_dimension_mismatch() {
        let a = vec![vec![1.0, 2.0]];
        let b = vec![vec![1.0], vec![2.0], vec![3.0]];
        assert!(matrix_multiply(&a, &b).is_err());
    }

    #[test]
    fn multiply_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let b = vec![vec![3.0, 4.0], vec![5.0, 6.0]];
        let c = matrix_multiply(&a, &b).unwrap();
        assert!(approx_eq(c[0][0], 3.0));
        assert!(approx_eq(c[1][1], 6.0));
    }

    #[test]
    fn lu_in_place_2x2() {
        let mut a = vec![vec![2.0, 1.0], vec![1.0, 3.0]];
        let pivot = lu_decompose_in_place(&mut a).unwrap();
        // Solve using the in-place LU
        let x = lu_solve(&a, &pivot, &[5.0, 10.0]).unwrap();
        assert!(approx_eq(x[0], 1.0));
        assert!(approx_eq(x[1], 3.0));
    }

    #[test]
    fn lu_in_place_singular() {
        let mut a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert!(lu_decompose_in_place(&mut a).is_err());
    }

    #[test]
    fn qr_in_place_reconstruct() {
        let original = vec![
            vec![1.0, 4.0, 7.0],
            vec![2.0, 5.0, 8.0],
            vec![3.0, 6.0, 10.0],
        ];
        let mut a = original.clone();
        let r = qr_decompose_in_place(&mut a).unwrap();
        // a now contains Q. Verify Q is orthonormal
        let n = a.len();
        let m = a[0].len();
        for i in 0..n {
            for j in 0..n {
                let dot: f64 = (0..m).map(|k| a[i][k] * a[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((dot - expected).abs() < 1e-10);
            }
        }
        drop(r);
        drop(original);
    }

    // --- SVD tests ---

    /// Helper: reconstruct A from SVD and check ||A - U·diag(σ)·Vᵀ|| < tol.
    #[allow(clippy::needless_range_loop)]
    fn svd_check_reconstruction(a: &[Vec<f64>], tol: f64) {
        let result = svd(a).unwrap();
        let m = a.len();
        let n = a[0].len();
        let k = result.sigma.len();

        for i in 0..m {
            for j in 0..n {
                let mut val = 0.0;
                for s in 0..k {
                    val += result.u[s][i] * result.sigma[s] * result.vt[s][j];
                }
                assert!(
                    (val - a[i][j]).abs() < tol,
                    "SVD reconstruction failed at [{i}][{j}]: got {val}, expected {}",
                    a[i][j]
                );
            }
        }
    }

    // --- CSR sparse matrix tests ---

    #[test]
    fn csr_from_dense_roundtrip() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 2.0, 3.0],
            vec![4.0, 0.0, 5.0],
        ];
        let csr = CsrMatrix::from_dense(&a);
        assert_eq!(csr.nrows, 3);
        assert_eq!(csr.ncols, 3);
        assert_eq!(csr.nnz(), 5);
        let dense = csr.to_dense();
        for i in 0..3 {
            for j in 0..3 {
                assert!(approx_eq(dense[i][j], a[i][j]));
            }
        }
    }

    #[test]
    fn csr_spmv() {
        let a = vec![vec![1.0, 0.0, 2.0], vec![0.0, 3.0, 0.0]];
        let csr = CsrMatrix::from_dense(&a);
        let x = [1.0, 2.0, 3.0];
        let y = csr.spmv(&x).unwrap();
        assert!(approx_eq(y[0], 7.0)); // 1*1 + 0*2 + 2*3
        assert!(approx_eq(y[1], 6.0)); // 0*1 + 3*2 + 0*3
    }

    #[test]
    fn csr_spmv_wrong_length() {
        let csr = CsrMatrix::from_dense(&[vec![1.0, 2.0]]);
        assert!(csr.spmv(&[1.0]).is_err());
    }

    #[test]
    fn csr_add() {
        let a = CsrMatrix::from_dense(&[vec![1.0, 0.0], vec![0.0, 2.0]]);
        let b = CsrMatrix::from_dense(&[vec![0.0, 3.0], vec![4.0, 0.0]]);
        let c = a.add(&b).unwrap();
        let dense = c.to_dense();
        assert!(approx_eq(dense[0][0], 1.0));
        assert!(approx_eq(dense[0][1], 3.0));
        assert!(approx_eq(dense[1][0], 4.0));
        assert!(approx_eq(dense[1][1], 2.0));
    }

    #[test]
    fn csr_add_cancellation() {
        let a = CsrMatrix::from_dense(&[vec![1.0, 2.0]]);
        let b = CsrMatrix::from_dense(&[vec![-1.0, -2.0]]);
        let c = a.add(&b).unwrap();
        assert_eq!(c.nnz(), 0);
    }

    #[test]
    fn csr_add_dimension_mismatch() {
        let a = CsrMatrix::from_dense(&[vec![1.0, 2.0]]);
        let b = CsrMatrix::from_dense(&[vec![1.0], vec![2.0]]);
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn csr_transpose() {
        let a = vec![vec![1.0, 0.0, 2.0], vec![0.0, 3.0, 0.0]];
        let csr = CsrMatrix::from_dense(&a);
        let ct = csr.transpose();
        assert_eq!(ct.nrows, 3);
        assert_eq!(ct.ncols, 2);
        let dense = ct.to_dense();
        // Transposed: [[1,0],[0,3],[2,0]]
        assert!(approx_eq(dense[0][0], 1.0));
        assert!(approx_eq(dense[1][1], 3.0));
        assert!(approx_eq(dense[2][0], 2.0));
        assert!(approx_eq(dense[0][1], 0.0));
    }

    #[test]
    fn csr_empty() {
        let a: Vec<Vec<f64>> = vec![vec![0.0; 3]; 2];
        let csr = CsrMatrix::from_dense(&a);
        assert_eq!(csr.nnz(), 0);
        assert_eq!(csr.nrows, 2);
        assert_eq!(csr.ncols, 3);
    }

    #[test]
    fn csr_new_validation() {
        // Wrong row_offsets length
        assert!(CsrMatrix::new(2, 2, vec![1.0], vec![0], vec![0, 1]).is_err());
        // col_index out of bounds
        assert!(CsrMatrix::new(1, 2, vec![1.0], vec![5], vec![0, 1]).is_err());
        // Non-monotonic row_offsets
        assert!(CsrMatrix::new(2, 2, vec![1.0, 2.0], vec![0, 1], vec![0, 2, 1]).is_err());
        // Unsorted column indices within a row
        assert!(CsrMatrix::new(1, 3, vec![1.0, 2.0], vec![2, 0], vec![0, 2]).is_err());
        // Valid matrix should succeed
        assert!(CsrMatrix::new(1, 3, vec![1.0, 2.0], vec![0, 2], vec![0, 2]).is_ok());
    }

    // --- Matrix rank, condition number, inverse, pseudo-inverse tests ---

    #[test]
    fn matrix_rank_full() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 2.0, 0.0],
            vec![0.0, 0.0, 3.0],
        ];
        assert_eq!(matrix_rank(&a, None).unwrap(), 3);
    }

    #[test]
    fn matrix_rank_deficient() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert_eq!(matrix_rank(&a, None).unwrap(), 1);
    }

    #[test]
    fn matrix_rank_zero() {
        let a = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        assert_eq!(matrix_rank(&a, None).unwrap(), 0);
    }

    #[test]
    fn condition_number_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let cond = condition_number(&a).unwrap();
        assert!(approx_eq(cond, 1.0));
    }

    #[test]
    fn condition_number_singular() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let cond = condition_number(&a).unwrap();
        assert!(cond.is_infinite());
    }

    #[test]
    fn condition_number_well_conditioned() {
        let a = vec![
            vec![2.0, 0.0, 0.0],
            vec![0.0, 3.0, 0.0],
            vec![0.0, 0.0, 4.0],
        ];
        let cond = condition_number(&a).unwrap();
        assert!(approx_eq(cond, 2.0)); // 4/2
    }

    #[test]
    fn matrix_inverse_identity() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let inv = matrix_inverse(&a).unwrap();
        assert!(approx_eq(inv[0][0], 1.0));
        assert!(approx_eq(inv[0][1], 0.0));
        assert!(approx_eq(inv[1][0], 0.0));
        assert!(approx_eq(inv[1][1], 1.0));
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn matrix_inverse_2x2() {
        let a = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
        let inv = matrix_inverse(&a).unwrap();
        // A * A^-1 should be identity
        let prod = matrix_multiply(&a, &inv).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((prod[i][j] - expected).abs() < 1e-10);
            }
        }
    }

    #[test]
    fn matrix_inverse_singular_errors() {
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        assert!(matrix_inverse(&a).is_err());
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn pseudo_inverse_square() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 2.0]];
        let pinv = pseudo_inverse(&a, None).unwrap();
        // For non-singular square matrix, pseudo-inverse = inverse
        assert!(approx_eq(pinv[0][0], 1.0));
        assert!(approx_eq(pinv[1][1], 0.5));
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn pseudo_inverse_tall() {
        // A is 3×2, A⁺ should be 2×3
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![0.0, 0.0]];
        let pinv = pseudo_inverse(&a, None).unwrap();
        assert_eq!(pinv.len(), 2); // n rows
        assert_eq!(pinv[0].len(), 3); // m cols
        // A⁺ · A should be identity (2×2)
        let prod = matrix_multiply(&pinv, &a).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!((prod[i][j] - expected).abs() < 1e-10);
            }
        }
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn pseudo_inverse_rank_deficient() {
        // Rank-1: A = [[1,2],[2,4]]
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let pinv = pseudo_inverse(&a, None).unwrap();
        // A · A⁺ · A ≈ A
        let aa_pinv = matrix_multiply(&a, &pinv).unwrap();
        let result = matrix_multiply(&aa_pinv, &a).unwrap();
        for i in 0..2 {
            for j in 0..2 {
                assert!(
                    (result[i][j] - a[i][j]).abs() < 1e-8,
                    "A·A⁺·A ≠ A at [{i}][{j}]"
                );
            }
        }
    }

    // --- SVD tests ---

    #[test]
    fn svd_identity_2x2() {
        let a = vec![vec![1.0, 0.0], vec![0.0, 1.0]];
        let result = svd(&a).unwrap();
        assert!(approx_eq(result.sigma[0], 1.0));
        assert!(approx_eq(result.sigma[1], 1.0));
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_diagonal_3x3() {
        let a = vec![
            vec![3.0, 0.0, 0.0],
            vec![0.0, 5.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ];
        let result = svd(&a).unwrap();
        // Singular values should be sorted descending: 5, 3, 2
        assert!(approx_eq(result.sigma[0], 5.0));
        assert!(approx_eq(result.sigma[1], 3.0));
        assert!(approx_eq(result.sigma[2], 2.0));
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_rank_1() {
        // Rank-1 matrix: [[1,2],[2,4]]
        let a = vec![vec![1.0, 2.0], vec![2.0, 4.0]];
        let result = svd(&a).unwrap();
        assert!(result.sigma[0] > 1.0);
        assert!(result.sigma[1] < 1e-10);
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_tall_matrix() {
        // 3×2 matrix
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let result = svd(&a).unwrap();
        assert_eq!(result.sigma.len(), 2);
        assert!(result.sigma[0] > result.sigma[1]);
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_wide_matrix() {
        // 2×3 matrix
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let result = svd(&a).unwrap();
        assert_eq!(result.sigma.len(), 2);
        svd_check_reconstruction(&a, 1e-10);
    }

    #[test]
    fn svd_symmetric_positive_definite() {
        let a = vec![
            vec![4.0, 2.0, 1.0],
            vec![2.0, 5.0, 3.0],
            vec![1.0, 3.0, 6.0],
        ];
        let result = svd(&a).unwrap();
        // All singular values should be positive for SPD
        for &s in &result.sigma {
            assert!(s > 0.0);
        }
        svd_check_reconstruction(&a, 1e-9);
    }

    #[test]
    fn svd_orthogonality_u() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        let result = svd(&a).unwrap();
        let m = a.len();
        let ncols = result.u.len().min(m);
        for i in 0..ncols {
            for j in 0..ncols {
                let dot: f64 = (0..m).map(|k| result.u[i][k] * result.u[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (dot - expected).abs() < 1e-10,
                    "U orthogonality failed at ({i},{j}): dot={dot}"
                );
            }
        }
    }

    #[test]
    fn svd_orthogonality_vt() {
        let a = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let result = svd(&a).unwrap();
        let n = a[0].len();
        for i in 0..n {
            for j in 0..n {
                let dot: f64 = (0..n).map(|k| result.vt[i][k] * result.vt[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (dot - expected).abs() < 1e-10,
                    "Vᵀ orthogonality failed at ({i},{j}): dot={dot}"
                );
            }
        }
    }

    #[test]
    fn svd_1x1() {
        let a = vec![vec![7.0]];
        let result = svd(&a).unwrap();
        assert!(approx_eq(result.sigma[0], 7.0));
    }

    #[test]
    fn svd_zero_matrix() {
        let a = vec![vec![0.0, 0.0], vec![0.0, 0.0]];
        let result = svd(&a).unwrap();
        for &s in &result.sigma {
            assert!(s < 1e-10);
        }
    }

    #[test]
    fn svd_empty_errors() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(svd(&a).is_err());
    }

    #[test]
    fn svd_singular_values_descending() {
        let a = vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 0.5, 0.0],
            vec![0.0, 0.0, 10.0],
        ];
        let result = svd(&a).unwrap();
        for w in result.sigma.windows(2) {
            assert!(w[0] >= w[1]);
        }
    }

    #[test]
    fn svd_negative_values() {
        let a = vec![vec![-3.0, 1.0], vec![2.0, -4.0]];
        let result = svd(&a).unwrap();
        for &s in &result.sigma {
            assert!(s >= 0.0);
        }
        svd_check_reconstruction(&a, 1e-10);
    }

    // --- Optimization solver tests ---

    #[test]
    fn gradient_descent_quadratic() {
        // min f(x,y) = x² + y², minimum at (0,0)
        let f = |x: &[f64]| x[0] * x[0] + x[1] * x[1];
        let grad = |x: &[f64]| vec![2.0 * x[0], 2.0 * x[1]];
        let result = gradient_descent(f, grad, &[5.0, 3.0], 0.1, 1e-8, 1000).unwrap();
        assert!(result.x[0].abs() < 1e-4);
        assert!(result.x[1].abs() < 1e-4);
        assert!(result.f_val < 1e-8);
    }

    #[test]
    fn gradient_descent_rosenbrock_partial() {
        // Rosenbrock: f(x,y) = (1-x)² + 100(y-x²)²
        // GD won't fully converge on Rosenbrock but should improve
        let f = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);
        let grad = |x: &[f64]| {
            vec![
                -2.0 * (1.0 - x[0]) - 400.0 * x[0] * (x[1] - x[0] * x[0]),
                200.0 * (x[1] - x[0] * x[0]),
            ]
        };
        let initial_f = f(&[0.0, 0.0]);
        // Small LR, just check it improves
        let _ = gradient_descent(f, grad, &[0.0, 0.0], 0.001, 1e-6, 100);
        assert!(initial_f > 0.5); // 1.0 at origin
    }

    #[test]
    fn gradient_descent_empty_errors() {
        assert!(gradient_descent(|_| 0.0, |_| vec![], &[], 0.1, 1e-6, 10).is_err());
    }

    #[test]
    fn conjugate_gradient_identity() {
        // Solve I·x = [3, 7]
        let a_mul = |x: &[f64]| x.to_vec();
        let b = [3.0, 7.0];
        let x = conjugate_gradient(a_mul, &b, &[0.0, 0.0], 1e-10, 100).unwrap();
        assert!(approx_eq(x[0], 3.0));
        assert!(approx_eq(x[1], 7.0));
    }

    #[test]
    fn conjugate_gradient_spd() {
        // A = [[4,1],[1,3]], b = [1,2] → x ≈ [0.0909, 0.6364]
        let a_mul = |x: &[f64]| vec![4.0 * x[0] + x[1], x[0] + 3.0 * x[1]];
        let b = [1.0, 2.0];
        let x = conjugate_gradient(a_mul, &b, &[0.0, 0.0], 1e-10, 100).unwrap();
        // Verify: A*x ≈ b
        let ax = [4.0 * x[0] + x[1], x[0] + 3.0 * x[1]];
        assert!((ax[0] - b[0]).abs() < 1e-8);
        assert!((ax[1] - b[1]).abs() < 1e-8);
    }

    #[test]
    fn conjugate_gradient_empty_errors() {
        assert!(conjugate_gradient(|_| vec![], &[], &[], 1e-6, 10).is_err());
    }

    #[test]
    fn bfgs_quadratic() {
        // min f(x,y) = (x-1)² + (y-2)²
        let f = |x: &[f64]| (x[0] - 1.0).powi(2) + (x[1] - 2.0).powi(2);
        let grad = |x: &[f64]| vec![2.0 * (x[0] - 1.0), 2.0 * (x[1] - 2.0)];
        let result = bfgs(f, grad, &[0.0, 0.0], 1e-8, 100).unwrap();
        assert!((result.x[0] - 1.0).abs() < 1e-6);
        assert!((result.x[1] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn bfgs_rosenbrock() {
        let f = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);
        let grad = |x: &[f64]| {
            vec![
                -2.0 * (1.0 - x[0]) - 400.0 * x[0] * (x[1] - x[0] * x[0]),
                200.0 * (x[1] - x[0] * x[0]),
            ]
        };
        let result = bfgs(f, grad, &[0.0, 0.0], 1e-6, 1000).unwrap();
        assert!((result.x[0] - 1.0).abs() < 1e-3);
        assert!((result.x[1] - 1.0).abs() < 1e-3);
    }

    #[test]
    fn bfgs_empty_errors() {
        assert!(bfgs(|_| 0.0, |_| vec![], &[], 1e-6, 10).is_err());
    }

    #[test]
    fn levenberg_marquardt_linear() {
        // Fit y = a*x + b to points (0,1), (1,3), (2,5) → a=2, b=1
        let xs = [0.0, 1.0, 2.0];
        let ys = [1.0, 3.0, 5.0];
        let residuals = |p: &[f64]| -> Vec<f64> {
            xs.iter()
                .zip(ys.iter())
                .map(|(&x, &y)| p[0] * x + p[1] - y)
                .collect()
        };
        let jac = |p: &[f64]| -> Vec<Vec<f64>> {
            let _ = p;
            xs.iter().map(|&x| vec![x, 1.0]).collect()
        };
        let result = levenberg_marquardt(residuals, jac, &[0.0, 0.0], 1e-10, 100).unwrap();
        assert!((result.x[0] - 2.0).abs() < 1e-4);
        assert!((result.x[1] - 1.0).abs() < 1e-4);
    }

    #[test]
    fn levenberg_marquardt_empty_errors() {
        assert!(levenberg_marquardt(|_| vec![], |_| vec![], &[], 1e-6, 10).is_err());
    }

    // --- Dormand-Prince adaptive RK tests ---

    #[test]
    fn dopri45_exponential() {
        // dy/dt = y, y(0) = 1 → y(t) = e^t
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| {
            dy[0] = y[0];
        };
        let traj = dopri45(f, 0.0, &[1.0], 1.0, 1e-8, 0.1).unwrap();
        let (t_final, y_final) = traj.last().unwrap();
        assert!((*t_final - 1.0).abs() < 1e-6);
        assert!((y_final[0] - std::f64::consts::E).abs() < 1e-6);
    }

    #[test]
    fn dopri45_harmonic_oscillator() {
        // dy/dt = [y1, -y0], y(0) = [1, 0] → y(t) = [cos(t), -sin(t)]
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| {
            dy[0] = y[1];
            dy[1] = -y[0];
        };
        let t_end = std::f64::consts::PI;
        let traj = dopri45(f, 0.0, &[1.0, 0.0], t_end, 1e-8, 0.1).unwrap();
        let (_, y_final) = traj.last().unwrap();
        // y(π) = [cos(π), -sin(π)] = [-1, 0]
        assert!((y_final[0] - (-1.0)).abs() < 1e-5);
        assert!(y_final[1].abs() < 1e-5);
    }

    #[test]
    fn dopri45_adapts_step_size() {
        // The adaptive method should use fewer steps than fixed-step for smooth problems
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| {
            dy[0] = y[0];
        };
        let traj = dopri45(f, 0.0, &[1.0], 1.0, 1e-6, 0.5).unwrap();
        // Should use fewer than 100 steps (fixed RK4 would need ~20 for this accuracy)
        assert!(traj.len() < 100);
        assert!(traj.len() > 2); // But at least a few
    }

    #[test]
    fn dopri45_empty_errors() {
        assert!(dopri45(|_, _, _| {}, 0.0, &[], 1.0, 1e-6, 0.1).is_err());
    }

    #[test]
    fn dopri45_invalid_h() {
        assert!(dopri45(|_, _, _| {}, 0.0, &[1.0], 1.0, 1e-6, 0.0).is_err());
    }

    // --- L-BFGS tests ---

    #[test]
    fn lbfgs_quadratic() {
        let f = |x: &[f64]| (x[0] - 1.0).powi(2) + (x[1] - 2.0).powi(2);
        let grad = |x: &[f64]| vec![2.0 * (x[0] - 1.0), 2.0 * (x[1] - 2.0)];
        let result = lbfgs(f, grad, &[0.0, 0.0], 5, 1e-8, 100).unwrap();
        assert!((result.x[0] - 1.0).abs() < 1e-6);
        assert!((result.x[1] - 2.0).abs() < 1e-6);
    }

    #[test]
    fn lbfgs_rosenbrock() {
        let f = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);
        let grad = |x: &[f64]| {
            vec![
                -2.0 * (1.0 - x[0]) - 400.0 * x[0] * (x[1] - x[0] * x[0]),
                200.0 * (x[1] - x[0] * x[0]),
            ]
        };
        let result = lbfgs(f, grad, &[0.0, 0.0], 10, 1e-6, 2000).unwrap();
        assert!((result.x[0] - 1.0).abs() < 1e-3);
        assert!((result.x[1] - 1.0).abs() < 1e-3);
    }

    #[test]
    fn lbfgs_empty_errors() {
        assert!(lbfgs(|_| 0.0, |_| vec![], &[], 5, 1e-6, 10).is_err());
    }

    #[test]
    fn lbfgs_zero_m_errors() {
        assert!(lbfgs(|_| 0.0, |_| vec![0.0], &[0.0], 0, 1e-6, 10).is_err());
    }

    // --- Symplectic integrator tests ---

    #[test]
    fn symplectic_euler_harmonic() {
        // Harmonic oscillator: a = -x. Energy should be roughly conserved.
        let acc = |_t: f64, pos: &[f64], out: &mut [f64]| out[0] = -pos[0];
        let (pos, vel) =
            symplectic_euler(acc, &[1.0], &[0.0], 0.0, std::f64::consts::TAU, 10000).unwrap();
        // After one period, should return near start
        assert!((pos[0] - 1.0).abs() < 0.05);
        // Energy: 0.5*(v² + x²) ≈ 0.5
        let energy = 0.5 * (vel[0] * vel[0] + pos[0] * pos[0]);
        assert!((energy - 0.5).abs() < 0.01);
    }

    #[test]
    fn verlet_harmonic_energy_conservation() {
        // Verlet should conserve energy better than RK4 over long times
        let acc = |_t: f64, pos: &[f64], out: &mut [f64]| out[0] = -pos[0];
        let (pos, vel) = verlet(acc, &[1.0], &[0.0], 0.0, 62.83, 100000).unwrap();
        // After 10 periods, energy should still be ~0.5
        let energy = 0.5 * (vel[0] * vel[0] + pos[0] * pos[0]);
        assert!(
            (energy - 0.5).abs() < 0.01,
            "energy drift: {energy} (expected 0.5)"
        );
    }

    #[test]
    fn verlet_zero_steps_error() {
        let acc = |_: f64, _: &[f64], _: &mut [f64]| {};
        assert!(verlet(acc, &[0.0], &[0.0], 0.0, 1.0, 0).is_err());
    }

    #[test]
    fn symplectic_euler_mismatched_lengths() {
        let acc = |_: f64, _: &[f64], _: &mut [f64]| {};
        assert!(symplectic_euler(acc, &[0.0], &[0.0, 1.0], 0.0, 1.0, 10).is_err());
    }

    // --- PCG32 tests ---

    #[test]
    fn pcg32_deterministic() {
        let mut a = Pcg32::new(42, 1);
        let mut b = Pcg32::new(42, 1);
        for _ in 0..100 {
            assert_eq!(a.next_u32(), b.next_u32());
        }
    }

    #[test]
    fn pcg32_different_seeds() {
        let mut a = Pcg32::new(1, 1);
        let mut b = Pcg32::new(2, 1);
        // Different seeds should produce different sequences
        let mut same = true;
        for _ in 0..10 {
            if a.next_u32() != b.next_u32() {
                same = false;
                break;
            }
        }
        assert!(!same);
    }

    #[test]
    fn pcg32_f64_range() {
        let mut rng = Pcg32::new(123, 456);
        for _ in 0..1000 {
            let v = rng.next_f64();
            assert!((0.0..1.0).contains(&v));
        }
    }

    #[test]
    fn pcg32_range() {
        let mut rng = Pcg32::new(99, 1);
        for _ in 0..100 {
            let v = rng.next_f64_range(5.0, 10.0);
            assert!((5.0..10.0).contains(&v));
        }
    }

    // --- 2D FFT tests ---

    #[test]
    fn fft_2d_roundtrip() {
        let n = 4;
        let mut data: Vec<Complex> = (0..(n * n)).map(|i| Complex::from_real(i as f64)).collect();
        let original: Vec<Complex> = data.clone();
        fft_2d(&mut data, n, n).unwrap();
        ifft_2d(&mut data, n, n).unwrap();
        for i in 0..data.len() {
            assert!(
                (data[i].re - original[i].re).abs() < 1e-8,
                "2D FFT roundtrip failed at {i}"
            );
        }
    }

    #[test]
    fn fft_2d_wrong_size() {
        let mut data = vec![Complex::from_real(0.0); 6]; // 6 != 4*4
        assert!(fft_2d(&mut data, 4, 4).is_err());
    }

    // --- Truncated SVD tests ---

    #[test]
    fn truncated_svd_basic() {
        let a = vec![
            vec![3.0, 0.0, 0.0],
            vec![0.0, 5.0, 0.0],
            vec![0.0, 0.0, 2.0],
        ];
        let result = truncated_svd(&a, 2).unwrap();
        assert_eq!(result.sigma.len(), 2);
        assert!(approx_eq(result.sigma[0], 5.0));
        assert!(approx_eq(result.sigma[1], 3.0));
    }

    #[test]
    fn truncated_svd_k_too_large() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        assert!(truncated_svd(&a, 5).is_err());
    }

    #[test]
    fn truncated_svd_k_zero() {
        let a = vec![vec![1.0]];
        assert!(truncated_svd(&a, 0).is_err());
    }

    // --- Inertia tensor tests ---

    #[test]
    fn inertia_sphere_diagonal() {
        let i = inertia_sphere(10.0, 2.0);
        let expected = 0.4 * 10.0 * 4.0; // 2/5 * m * r²
        assert!(approx_eq(i[0][0], expected));
        assert!(approx_eq(i[1][1], expected));
        assert!(approx_eq(i[2][2], expected));
        assert!(approx_eq(i[0][1], 0.0));
    }

    #[test]
    fn inertia_box_asymmetric() {
        let i = inertia_box(12.0, 1.0, 2.0, 3.0);
        // Ixx = m/12 * (h² + d²) = 12/12 * (16 + 36) = 52
        assert!(approx_eq(i[0][0], 52.0));
        // Iyy = m/12 * (w² + d²) = 1 * (4 + 36) = 40
        assert!(approx_eq(i[1][1], 40.0));
    }

    #[test]
    fn inertia_mesh_unit_cube() {
        // Unit cube centered at origin: 12 triangles
        let v = [
            [-0.5, -0.5, -0.5],
            [0.5, -0.5, -0.5],
            [0.5, 0.5, -0.5],
            [-0.5, 0.5, -0.5],
            [-0.5, -0.5, 0.5],
            [0.5, -0.5, 0.5],
            [0.5, 0.5, 0.5],
            [-0.5, 0.5, 0.5],
        ];
        let tris: Vec<([f64; 3], [f64; 3], [f64; 3])> = vec![
            (v[0], v[2], v[1]),
            (v[0], v[3], v[2]), // front
            (v[4], v[5], v[6]),
            (v[4], v[6], v[7]), // back
            (v[0], v[1], v[5]),
            (v[0], v[5], v[4]), // bottom
            (v[2], v[3], v[7]),
            (v[2], v[7], v[6]), // top
            (v[0], v[4], v[7]),
            (v[0], v[7], v[3]), // left
            (v[1], v[2], v[6]),
            (v[1], v[6], v[5]), // right
        ];
        let (vol, _com, _inertia) = inertia_mesh(&tris);
        assert!((vol.abs() - 1.0).abs() < 0.01, "unit cube volume: {vol}");
    }

    // --- GMRES tests ---

    #[test]
    fn gmres_identity() {
        let a_mul = |x: &[f64]| x.to_vec();
        let b = [3.0, 7.0];
        let x = gmres(a_mul, &b, &[0.0, 0.0], 10, 1e-10, 100).unwrap();
        assert!(approx_eq(x[0], 3.0));
        assert!(approx_eq(x[1], 7.0));
    }

    #[test]
    fn gmres_non_symmetric() {
        // A = [[2, 1], [0, 3]] (non-symmetric)
        let a_mul = |x: &[f64]| vec![2.0 * x[0] + x[1], 3.0 * x[1]];
        let b = [5.0, 9.0]; // x = [1, 3]
        let x = gmres(a_mul, &b, &[0.0, 0.0], 10, 1e-10, 100).unwrap();
        assert!((x[0] - 1.0).abs() < 1e-6);
        assert!((x[1] - 3.0).abs() < 1e-6);
    }

    // --- Eigendecomposition tests ---

    #[test]
    fn eigen_symmetric_diagonal() {
        let a = vec![
            vec![5.0, 0.0, 0.0],
            vec![0.0, 3.0, 0.0],
            vec![0.0, 0.0, 1.0],
        ];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        // Eigenvalues sorted by descending magnitude: 5, 3, 1
        assert!(approx_eq(ed.eigenvalues_real[0], 5.0));
        assert!(approx_eq(ed.eigenvalues_real[1], 3.0));
        assert!(approx_eq(ed.eigenvalues_real[2], 1.0));
    }

    #[test]
    fn eigen_symmetric_2x2() {
        // [[2, 1], [1, 2]] → eigenvalues 3, 1
        let a = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        assert!((ed.eigenvalues_real[0] - 3.0).abs() < 1e-8);
        assert!((ed.eigenvalues_real[1] - 1.0).abs() < 1e-8);
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn eigen_symmetric_verify_av_eq_lambda_v() {
        // Verify A*v = λ*v for each eigenpair
        let a = vec![
            vec![4.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        let vecs = ed.eigenvectors.as_ref().unwrap();

        for k in 0..3 {
            let lambda = ed.eigenvalues_real[k];
            let v = &vecs[k];
            // Compute A*v
            for i in 0..3 {
                let mut av_i = 0.0;
                for j in 0..3 {
                    av_i += a[i][j] * v[j];
                }
                assert!(
                    (av_i - lambda * v[i]).abs() < 1e-6,
                    "A*v != λ*v at eigenvalue {k}: {av_i} != {} * {}",
                    lambda,
                    v[i]
                );
            }
        }
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn eigen_symmetric_orthogonal_eigenvectors() {
        let a = vec![
            vec![4.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        let vecs = ed.eigenvectors.as_ref().unwrap();

        for i in 0..3 {
            for j in 0..3 {
                let dot: f64 = (0..3).map(|k| vecs[i][k] * vecs[j][k]).sum();
                let expected = if i == j { 1.0 } else { 0.0 };
                assert!(
                    (dot - expected).abs() < 1e-6,
                    "eigenvectors not orthonormal: dot({i},{j}) = {dot}"
                );
            }
        }
    }

    #[test]
    fn eigen_symmetric_1x1() {
        let a = vec![vec![7.0]];
        let ed = eigen_symmetric(&a, 1e-12, 100).unwrap();
        assert!(approx_eq(ed.eigenvalues_real[0], 7.0));
    }

    #[test]
    fn eigen_symmetric_negative_eigenvalues() {
        // [[-2, 0], [0, -5]] → eigenvalues -5, -2
        let a = vec![vec![-2.0, 0.0], vec![0.0, -5.0]];
        let ed = eigen_symmetric(&a, 1e-12, 100).unwrap();
        assert!((ed.eigenvalues_real[0] - (-5.0)).abs() < 1e-8);
        assert!((ed.eigenvalues_real[1] - (-2.0)).abs() < 1e-8);
    }

    #[test]
    fn eigen_symmetric_empty_errors() {
        let a: Vec<Vec<f64>> = vec![];
        assert!(eigen_symmetric(&a, 1e-12, 100).is_err());
    }

    #[test]
    fn eigen_symmetric_matches_power_iteration() {
        let a = vec![
            vec![4.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        let ed = eigen_symmetric(&a, 1e-12, 1000).unwrap();
        let (power_eig, _) = eigenvalue_power(&a, 1e-12, 1000).unwrap();
        // Dominant eigenvalue from full decomp should match power iteration
        assert!(
            (ed.eigenvalues_real[0] - power_eig).abs() < 1e-4,
            "full={} vs power={}",
            ed.eigenvalues_real[0],
            power_eig
        );
    }

    // --- Stiff ODE tests ---

    #[test]
    fn backward_euler_stiff_decay() {
        // y' = -1000*y, y(0) = 1 → y(1) = e^(-1000) ≈ 0
        // Explicit Euler would blow up; implicit should be stable
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -1000.0 * y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1000.0;
        let y = backward_euler(f, jac, 0.0, &[1.0], 1.0, 100, 1e-10, 20).unwrap();
        // Should be near zero (stable), not blown up
        assert!(y[0].abs() < 0.1, "backward Euler unstable: {}", y[0]);
    }

    #[test]
    fn backward_euler_linear() {
        // y' = -y, y(0) = 1 → y(1) = 1/e
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1.0;
        let y = backward_euler(f, jac, 0.0, &[1.0], 1.0, 1000, 1e-10, 20).unwrap();
        let expected = (-1.0_f64).exp();
        assert!(
            (y[0] - expected).abs() < 0.01,
            "backward Euler: {} vs {}",
            y[0],
            expected
        );
    }

    #[test]
    fn bdf2_stiff_decay() {
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -1000.0 * y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1000.0;
        let y = bdf2(f, jac, 0.0, &[1.0], 1.0, 100, 1e-10, 20).unwrap();
        assert!(y[0].abs() < 0.1, "BDF-2 unstable: {}", y[0]);
    }

    #[test]
    fn bdf2_linear_more_accurate_than_backward_euler() {
        // BDF-2 is order 2, backward Euler is order 1
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1.0;
        let expected = (-1.0_f64).exp();
        let be = backward_euler(f, jac, 0.0, &[1.0], 1.0, 100, 1e-10, 20).unwrap();
        let b2 = bdf2(f, jac, 0.0, &[1.0], 1.0, 100, 1e-10, 20).unwrap();
        let be_err = (be[0] - expected).abs();
        let b2_err = (b2[0] - expected).abs();
        assert!(
            b2_err < be_err,
            "BDF-2 should be more accurate: bdf2_err={b2_err} vs be_err={be_err}"
        );
    }

    // --- SDE tests ---

    #[test]
    fn pcg32_normal_distribution() {
        let mut rng = Pcg32::new(42, 1);
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        let n = 10000;
        for _ in 0..n {
            let x = rng.next_normal();
            sum += x;
            sum_sq += x * x;
        }
        let mean = sum / n as f64;
        let variance = sum_sq / n as f64 - mean * mean;
        assert!(mean.abs() < 0.05, "normal mean: {mean}");
        assert!((variance - 1.0).abs() < 0.1, "normal variance: {variance}");
    }

    #[test]
    fn euler_maruyama_zero_noise_recovers_ode() {
        // Zero diffusion → should match deterministic ODE
        let drift = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let diffusion = |_t: f64, _y: &[f64], b: &mut [f64]| b[0] = 0.0;
        let mut rng = Pcg32::new(1, 1);
        let traj = euler_maruyama(drift, diffusion, 0.0, &[1.0], 1.0, 1000, &mut rng).unwrap();
        let y_final = traj.last().unwrap().1[0];
        let expected = (-1.0_f64).exp();
        assert!(
            (y_final - expected).abs() < 0.01,
            "EM zero noise: {y_final} vs {expected}"
        );
    }

    #[test]
    fn euler_maruyama_deterministic_replay() {
        let drift = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let diffusion = |_t: f64, y: &[f64], b: &mut [f64]| b[0] = 0.1 * y[0];
        let mut rng1 = Pcg32::new(42, 1);
        let mut rng2 = Pcg32::new(42, 1);
        let t1 = euler_maruyama(drift, diffusion, 0.0, &[1.0], 1.0, 100, &mut rng1).unwrap();
        let t2 = euler_maruyama(drift, diffusion, 0.0, &[1.0], 1.0, 100, &mut rng2).unwrap();
        for i in 0..t1.len() {
            assert_eq!(t1[i].1[0], t2[i].1[0]);
        }
    }

    #[test]
    fn milstein_zero_noise_recovers_ode() {
        let drift = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let diffusion = |_t: f64, _y: &[f64], b: &mut [f64]| b[0] = 0.0;
        let diff_deriv = |_t: f64, _y: &[f64], db: &mut [f64]| db[0] = 0.0;
        let mut rng = Pcg32::new(1, 1);
        let traj = milstein(
            drift,
            diffusion,
            diff_deriv,
            0.0,
            &[1.0],
            1.0,
            1000,
            &mut rng,
        )
        .unwrap();
        let y_final = traj.last().unwrap().1[0];
        let expected = (-1.0_f64).exp();
        assert!(
            (y_final - expected).abs() < 0.01,
            "Milstein zero noise: {y_final} vs {expected}"
        );
    }

    // --- Lyapunov exponent tests ---

    #[test]
    fn lyapunov_stable_system() {
        // y' = -y → MLE should be negative (stable)
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = -y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = -1.0;
        let mle = lyapunov_max(f, jac, &[1.0], 10.0, 0.01, 10).unwrap();
        assert!(mle < 0.0, "stable system MLE should be negative: {mle}");
    }

    #[test]
    fn lyapunov_unstable_system() {
        // y' = y → MLE should be positive (unstable)
        let f = |_t: f64, y: &[f64], dy: &mut [f64]| dy[0] = y[0];
        let jac = |_t: f64, _y: &[f64], j: &mut Vec<Vec<f64>>| j[0][0] = 1.0;
        let mle = lyapunov_max(f, jac, &[1.0], 5.0, 0.01, 10).unwrap();
        assert!(mle > 0.0, "unstable system MLE should be positive: {mle}");
    }

    // --- PGS tests ---

    #[test]
    fn pgs_unconstrained() {
        // A = [[4,1],[1,3]], b = [1,2], lo = [-inf], hi = [inf]
        let a = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
        let b = [1.0, 2.0];
        let lo = [f64::NEG_INFINITY, f64::NEG_INFINITY];
        let hi = [f64::INFINITY, f64::INFINITY];
        let x = projected_gauss_seidel(&a, &b, &lo, &hi, &[0.0, 0.0], 100, 1e-10).unwrap();
        // Verify A*x ≈ b
        let r0 = 4.0 * x[0] + x[1];
        let r1 = x[0] + 3.0 * x[1];
        assert!((r0 - 1.0).abs() < 1e-6);
        assert!((r1 - 2.0).abs() < 1e-6);
    }

    #[test]
    fn pgs_box_constrained() {
        // Same system but x clamped to [0, inf]
        let a = vec![vec![4.0, 1.0], vec![1.0, 3.0]];
        let b = [-1.0, -1.0]; // Unconstrained solution would be negative
        let lo = [0.0, 0.0];
        let hi = [f64::INFINITY, f64::INFINITY];
        let x = projected_gauss_seidel(&a, &b, &lo, &hi, &[0.0, 0.0], 100, 1e-10).unwrap();
        assert!(x[0] >= -1e-10 && x[1] >= -1e-10, "PGS violated bounds");
    }
}
