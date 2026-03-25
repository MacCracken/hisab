use super::svd::svd;
use crate::HisabError;

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
