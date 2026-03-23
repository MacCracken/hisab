//! Numerical methods: root finding, linear solvers, decompositions, FFT, and ODE solvers.
//!
//! Provides Newton-Raphson, bisection, Gaussian elimination, LU/Cholesky/QR
//! decompositions, least-squares fitting, eigenvalue computation (power iteration),
//! Cooley-Tukey FFT/IFFT, and Runge-Kutta (RK4) ODE integration.

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
            return Err(GanitError::InvalidInput("derivative is zero".to_string()));
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
        return Err(GanitError::InvalidInput(
            "x and y must have equal non-zero length".to_string(),
        ));
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
#[allow(clippy::needless_range_loop)]
pub fn eigenvalue_power(
    a: &[Vec<f64>],
    tol: f64,
    max_iter: usize,
) -> Result<(f64, Vec<f64>), GanitError> {
    let n = a.len();
    if n == 0 {
        return Err(GanitError::InvalidInput("empty matrix".to_string()));
    }
    for row in a {
        if row.len() != n {
            return Err(GanitError::InvalidInput(format!(
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

        if max_val.abs() < 1e-15 {
            return Err(GanitError::NoConvergence(max_iter));
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

    Err(GanitError::NoConvergence(max_iter))
}

// ---------------------------------------------------------------------------
// FFT (Cooley-Tukey radix-2)
// ---------------------------------------------------------------------------

/// A complex number for FFT operations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub re: f64,
    pub im: f64,
}

impl Complex {
    /// Create a new complex number.
    #[inline]
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Complex number from a real value.
    #[inline]
    pub const fn from_real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    /// Magnitude (absolute value).
    #[inline]
    pub fn abs(self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Complex conjugate.
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

/// In-place Cooley-Tukey radix-2 FFT.
///
/// `data` must have a power-of-2 length. Computes the DFT in-place.
///
/// # Panics
///
/// Panics if `data.len()` is not a power of two.
pub fn fft(data: &mut [Complex]) {
    let n = data.len();
    if n <= 1 {
        return;
    }
    assert!(n.is_power_of_two(), "FFT requires power-of-2 length");

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
}

/// In-place inverse FFT.
///
/// `data` must have a power-of-2 length.
pub fn ifft(data: &mut [Complex]) {
    let n = data.len();
    // Conjugate, FFT, conjugate, scale
    for d in data.iter_mut() {
        *d = d.conj();
    }
    fft(data);
    let scale = 1.0 / n as f64;
    for d in data.iter_mut() {
        *d = d.conj() * scale;
    }
}

// ---------------------------------------------------------------------------
// ODE solvers
// ---------------------------------------------------------------------------

/// Perform one RK4 step in-place, reusing scratch buffers.
#[allow(clippy::needless_range_loop)]
fn rk4_step(
    f: &impl Fn(f64, &[f64]) -> Vec<f64>,
    t: f64,
    h: f64,
    y: &mut [f64],
    tmp: &mut [f64],
    dim: usize,
) {
    let k1 = f(t, y);

    for i in 0..dim {
        tmp[i] = y[i] + 0.5 * h * k1[i];
    }
    let k2 = f(t + 0.5 * h, tmp);

    for i in 0..dim {
        tmp[i] = y[i] + 0.5 * h * k2[i];
    }
    let k3 = f(t + 0.5 * h, tmp);

    for i in 0..dim {
        tmp[i] = y[i] + h * k3[i];
    }
    let k4 = f(t + h, tmp);

    let h6 = h / 6.0;
    for i in 0..dim {
        y[i] += h6 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
    }
}

/// Fourth-order Runge-Kutta (RK4) integrator for a system of ODEs.
///
/// Solves `dy/dt = f(t, y)` from `t0` to `t_end` with `n` steps.
///
/// - `f`: the derivative function `f(t, &y) -> dy/dt` (returns a Vec of same length as `y`).
/// - `t0`: initial time.
/// - `y0`: initial state vector.
/// - `t_end`: final time.
/// - `n`: number of integration steps.
///
/// Returns the final state vector `y(t_end)`.
pub fn rk4(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t0: f64,
    y0: &[f64],
    t_end: f64,
    n: usize,
) -> Vec<f64> {
    assert!(n > 0, "n must be positive");
    let dim = y0.len();
    let h = (t_end - t0) / n as f64;
    let mut t = t0;
    let mut y = y0.to_vec();
    let mut tmp = vec![0.0; dim];

    for _ in 0..n {
        rk4_step(&f, t, h, &mut y, &mut tmp, dim);
        t += h;
    }

    y
}

/// Fourth-order Runge-Kutta with full trajectory output.
///
/// Same as [`rk4`] but returns all intermediate states as `Vec<(t, y)>`.
pub fn rk4_trajectory(
    f: impl Fn(f64, &[f64]) -> Vec<f64>,
    t0: f64,
    y0: &[f64],
    t_end: f64,
    n: usize,
) -> Vec<(f64, Vec<f64>)> {
    assert!(n > 0, "n must be positive");
    let dim = y0.len();
    let h = (t_end - t0) / n as f64;
    let mut t = t0;
    let mut y = y0.to_vec();
    let mut tmp = vec![0.0; dim];
    let mut trajectory = Vec::with_capacity(n + 1);
    trajectory.push((t, y.clone()));

    for _ in 0..n {
        rk4_step(&f, t, h, &mut y, &mut tmp, dim);
        t += h;
        trajectory.push((t, y.clone()));
    }

    trajectory
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
        fft(&mut data);
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
        fft(&mut data);
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
        fft(&mut data);
        ifft(&mut data);
        for i in 0..4 {
            assert!((data[i].re - original[i].re).abs() < 1e-10);
            assert!((data[i].im - original[i].im).abs() < 1e-10);
        }
    }

    #[test]
    fn fft_8_point() {
        // 8-point FFT of real signal
        let mut data: Vec<Complex> = (0..8).map(|i| Complex::from_real(i as f64)).collect();
        fft(&mut data);
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
        fft(&mut data);
        ifft(&mut data);
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
        fft(&mut freq);
        let freq_energy: f64 = freq.iter().map(|c| c.re * c.re + c.im * c.im).sum();
        // N * time_energy = freq_energy
        assert!((4.0 * time_energy - freq_energy).abs() < 1e-10);
    }

    #[test]
    fn fft_single_element() {
        let mut data = [Complex::new(42.0, -7.0)];
        fft(&mut data);
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
        fft(&mut fx);
        let mut fy = y.clone();
        fft(&mut fy);

        // 2*x + 3*y
        let mut combined: Vec<Complex> = (0..4).map(|i| x[i] * 2.0 + y[i] * 3.0).collect();
        fft(&mut combined);

        for i in 0..4 {
            let expected = fx[i] * 2.0 + fy[i] * 3.0;
            assert!((combined[i].re - expected.re).abs() < 1e-10);
            assert!((combined[i].im - expected.im).abs() < 1e-10);
        }
    }

    #[test]
    fn ifft_single_element() {
        let mut data = [Complex::new(42.0, -7.0)];
        ifft(&mut data);
        assert!(approx_eq(data[0].re, 42.0));
        assert!(approx_eq(data[0].im, -7.0));
    }

    #[test]
    #[should_panic(expected = "power-of-2")]
    fn fft_non_power_of_two_panics() {
        let mut data = vec![Complex::default(); 3];
        fft(&mut data);
    }

    // --- V0.4c: RK4 ODE solver ---

    #[test]
    fn rk4_exponential_growth() {
        // dy/dt = y, y(0) = 1 => y(t) = e^t
        // At t=1, y should be e ≈ 2.71828
        let y = rk4(|_t, y| vec![y[0]], 0.0, &[1.0], 1.0, 1000);
        assert!((y[0] - std::f64::consts::E).abs() < 1e-8);
    }

    #[test]
    fn rk4_linear_ode() {
        // dy/dt = 2, y(0) = 0 => y(t) = 2t
        // At t=5, y = 10
        let y = rk4(|_t, _y| vec![2.0], 0.0, &[0.0], 5.0, 100);
        assert!(approx_eq(y[0], 10.0));
    }

    #[test]
    fn rk4_harmonic_oscillator() {
        // x'' + x = 0 => system: dx/dt = v, dv/dt = -x
        // x(0)=1, v(0)=0 => x(t) = cos(t), v(t) = -sin(t)
        // At t=pi, x ≈ -1, v ≈ 0
        let y = rk4(
            |_t, y| vec![y[1], -y[0]],
            0.0,
            &[1.0, 0.0],
            std::f64::consts::PI,
            10000,
        );
        assert!((y[0] - (-1.0)).abs() < 1e-6); // x(pi) = cos(pi) = -1
        assert!(y[1].abs() < 1e-6); // v(pi) = -sin(pi) = 0
    }

    #[test]
    fn rk4_quadratic_exact() {
        // dy/dt = 2t, y(0) = 0 => y(t) = t^2
        // RK4 is exact for polynomials up to degree 4
        let y = rk4(|t, _y| vec![2.0 * t], 0.0, &[0.0], 3.0, 10);
        assert!((y[0] - 9.0).abs() < 1e-10);
    }

    #[test]
    fn rk4_system_2d() {
        // dx/dt = -y, dy/dt = x (rotation)
        // x(0)=1, y(0)=0 => x(t)=cos(t), y(t)=sin(t)
        // At t=pi/2: x≈0, y≈1
        let y = rk4(
            |_t, y| vec![-y[1], y[0]],
            0.0,
            &[1.0, 0.0],
            std::f64::consts::FRAC_PI_2,
            1000,
        );
        assert!(y[0].abs() < 1e-6);
        assert!((y[1] - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rk4_trajectory_length() {
        let traj = rk4_trajectory(|_t, y| vec![y[0]], 0.0, &[1.0], 1.0, 100);
        assert_eq!(traj.len(), 101); // n+1 points
        assert!(approx_eq(traj[0].0, 0.0));
        assert!((traj[100].0 - 1.0).abs() < 1e-10);
    }

    #[test]
    fn rk4_trajectory_matches_final() {
        // Trajectory endpoint should match rk4 result
        let final_state = rk4(|_t, y| vec![y[0]], 0.0, &[1.0], 1.0, 100);
        let traj = rk4_trajectory(|_t, y| vec![y[0]], 0.0, &[1.0], 1.0, 100);
        let traj_final = &traj[100].1;
        assert!((final_state[0] - traj_final[0]).abs() < 1e-12);
    }

    #[test]
    fn rk4_damped_oscillator() {
        // x'' + 0.1*x' + x = 0 (underdamped)
        // System: dx/dt = v, dv/dt = -x - 0.1*v
        // Should decay toward zero
        let y = rk4(
            |_t, y| vec![y[1], -y[0] - 0.1 * y[1]],
            0.0,
            &[1.0, 0.0],
            20.0,
            10000,
        );
        // After 20 seconds of damping, amplitude should be small
        assert!(y[0].abs() < 0.5);
    }
}
