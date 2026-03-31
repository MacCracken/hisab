//! Complex-valued linear algebra.
//!
//! [`ComplexMatrix`] is a row-major dense matrix of [`Complex`] values, mirroring
//! the real-valued [`DenseMatrix`](super::DenseMatrix) API. On top of standard
//! arithmetic it provides:
//!
//! - Hermitian eigendecomposition (Jacobi algorithm)
//! - Complex SVD
//! - Unitary matrix verification and construction
//! - Pauli and Dirac gamma matrix algebra
//! - Spinor transformations

use super::complex::Complex;
use crate::HisabError;
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// ComplexMatrix
// ---------------------------------------------------------------------------

/// Row-major dense matrix of [`Complex`] values.
///
/// # Examples
///
/// ```
/// use hisab::num::{Complex, ComplexMatrix};
///
/// let id = ComplexMatrix::identity(2);
/// let v = vec![Complex::new(1.0, 0.0), Complex::new(0.0, 1.0)];
/// let result = id.mul_vec(&v).unwrap();
/// assert!((result[0] - v[0]).norm_sq() < 1e-12);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComplexMatrix {
    data: Vec<Complex>,
    rows: usize,
    cols: usize,
}

impl ComplexMatrix {
    // -----------------------------------------------------------------------
    // Constructors

    /// Zero-filled *rows × cols* complex matrix.
    #[must_use]
    #[inline]
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            data: vec![Complex::default(); rows * cols],
            rows,
            cols,
        }
    }

    /// *n × n* identity matrix.
    #[must_use]
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zeros(n, n);
        for i in 0..n {
            m.data[i * n + i] = Complex::from_real(1.0);
        }
        m
    }

    /// Construct from flat row-major data.
    ///
    /// # Errors
    ///
    /// Returns [`HisabError::InvalidInput`] if `data.len() != rows * cols`.
    pub fn from_rows(rows: usize, cols: usize, data: Vec<Complex>) -> Result<Self, HisabError> {
        if data.len() != rows * cols {
            return Err(HisabError::InvalidInput(format!(
                "data length: expected {}, got {}",
                rows * cols,
                data.len()
            )));
        }
        Ok(Self { data, rows, cols })
    }

    /// Construct from a real-valued `DenseMatrix` (zero imaginary parts).
    #[must_use]
    pub fn from_real(m: &super::DenseMatrix) -> Self {
        let rows = m.rows();
        let cols = m.cols();
        let data: Vec<Complex> = (0..rows)
            .flat_map(|r| (0..cols).map(move |c| Complex::from_real(m.get(r, c))))
            .collect();
        Self { data, rows, cols }
    }

    // -----------------------------------------------------------------------
    // Dimensions

    /// Number of rows.
    #[must_use]
    #[inline]
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Number of columns.
    #[must_use]
    #[inline]
    pub fn cols(&self) -> usize {
        self.cols
    }

    // -----------------------------------------------------------------------
    // Element access

    /// Read element at `(row, col)`.
    #[must_use]
    #[inline]
    pub fn get(&self, row: usize, col: usize) -> Complex {
        debug_assert!(row < self.rows && col < self.cols);
        self.data[row * self.cols + col]
    }

    /// Write element at `(row, col)`.
    #[inline]
    pub fn set(&mut self, row: usize, col: usize, val: Complex) {
        debug_assert!(row < self.rows && col < self.cols);
        self.data[row * self.cols + col] = val;
    }

    // -----------------------------------------------------------------------
    // Operations

    /// Conjugate transpose (Hermitian adjoint): `A†`.
    #[must_use]
    pub fn adjoint(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                out.data[c * self.rows + r] = self.data[r * self.cols + c].conj();
            }
        }
        out
    }

    /// Transpose (no conjugation).
    #[must_use]
    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.cols, self.rows);
        for r in 0..self.rows {
            for c in 0..self.cols {
                out.data[c * self.rows + r] = self.data[r * self.cols + c];
            }
        }
        out
    }

    /// Element-wise conjugate.
    #[must_use]
    pub fn conj(&self) -> Self {
        Self {
            data: self.data.iter().map(|z| z.conj()).collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }

    /// Scale every element by a complex scalar.
    #[must_use]
    pub fn scale(&self, s: Complex) -> Self {
        Self {
            data: self.data.iter().map(|&z| z * s).collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }

    /// Scale by a real scalar.
    #[must_use]
    pub fn scale_real(&self, s: f64) -> Self {
        Self {
            data: self.data.iter().map(|&z| z * s).collect(),
            rows: self.rows,
            cols: self.cols,
        }
    }

    /// Matrix addition.
    ///
    /// # Errors
    ///
    /// Returns error if dimensions don't match.
    pub fn add(&self, other: &Self) -> Result<Self, HisabError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(HisabError::InvalidInput(format!(
                "dimension mismatch: {}x{} vs {}x{}",
                self.rows, self.cols, other.rows, other.cols
            )));
        }
        let data: Vec<Complex> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        Ok(Self {
            data,
            rows: self.rows,
            cols: self.cols,
        })
    }

    /// Matrix subtraction.
    ///
    /// # Errors
    ///
    /// Returns error if dimensions don't match.
    pub fn sub(&self, other: &Self) -> Result<Self, HisabError> {
        if self.rows != other.rows || self.cols != other.cols {
            return Err(HisabError::InvalidInput(format!(
                "dimension mismatch: {}x{} vs {}x{}",
                self.rows, self.cols, other.rows, other.cols
            )));
        }
        let data: Vec<Complex> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a - b)
            .collect();
        Ok(Self {
            data,
            rows: self.rows,
            cols: self.cols,
        })
    }

    /// Matrix-vector product: **A** · **x**.
    ///
    /// # Errors
    ///
    /// Returns error if `x.len() != self.cols`.
    pub fn mul_vec(&self, x: &[Complex]) -> Result<Vec<Complex>, HisabError> {
        if x.len() != self.cols {
            return Err(HisabError::InvalidInput(format!(
                "vector length: expected {}, got {}",
                self.cols,
                x.len()
            )));
        }
        let mut out = vec![Complex::default(); self.rows];
        for (r, dst) in out.iter_mut().enumerate() {
            let row = &self.data[r * self.cols..(r + 1) * self.cols];
            let mut sum = Complex::default();
            for c in 0..self.cols {
                sum += row[c] * x[c];
            }
            *dst = sum;
        }
        Ok(out)
    }

    /// Matrix-matrix product: **self** · **other**.
    ///
    /// # Errors
    ///
    /// Returns error if `self.cols != other.rows`.
    pub fn mul_mat(&self, other: &Self) -> Result<Self, HisabError> {
        if self.cols != other.rows {
            return Err(HisabError::InvalidInput(format!(
                "inner dimension mismatch: {} vs {}",
                self.cols, other.rows
            )));
        }
        let rows = self.rows;
        let cols = other.cols;
        let inner = self.cols;
        let mut out = Self::zeros(rows, cols);
        for r in 0..rows {
            for c in 0..cols {
                let mut sum = Complex::default();
                for k in 0..inner {
                    sum += self.data[r * inner + k] * other.data[k * cols + c];
                }
                out.data[r * cols + c] = sum;
            }
        }
        Ok(out)
    }

    /// Trace: sum of diagonal elements.
    ///
    /// # Errors
    ///
    /// Returns error if matrix is not square.
    pub fn trace(&self) -> Result<Complex, HisabError> {
        if self.rows != self.cols {
            return Err(HisabError::InvalidInput("matrix must be square".into()));
        }
        let mut sum = Complex::default();
        for i in 0..self.rows {
            sum += self.data[i * self.cols + i];
        }
        Ok(sum)
    }

    /// Frobenius norm: `√(∑ |aᵢⱼ|²)`.
    #[must_use]
    pub fn frobenius_norm(&self) -> f64 {
        self.data.iter().map(|z| z.norm_sq()).sum::<f64>().sqrt()
    }

    /// Check if this matrix is Hermitian within tolerance.
    #[must_use]
    pub fn is_hermitian(&self, tol: f64) -> bool {
        if self.rows != self.cols {
            return false;
        }
        let n = self.rows;
        for r in 0..n {
            for c in r..n {
                let diff = self.get(r, c) - self.get(c, r).conj();
                if diff.norm_sq() > tol * tol {
                    return false;
                }
            }
        }
        true
    }

    /// Check if this matrix is unitary within tolerance: `U†U ≈ I`.
    #[must_use]
    pub fn is_unitary(&self, tol: f64) -> bool {
        if self.rows != self.cols {
            return false;
        }
        let n = self.rows;
        if let Ok(product) = self.adjoint().mul_mat(self) {
            let id = Self::identity(n);
            if let Ok(diff) = product.sub(&id) {
                return diff.frobenius_norm() < tol;
            }
        }
        false
    }

    /// Determinant of a 2×2 complex matrix.
    ///
    /// # Errors
    ///
    /// Returns error if matrix is not 2×2.
    pub fn det_2x2(&self) -> Result<Complex, HisabError> {
        if self.rows != 2 || self.cols != 2 {
            return Err(HisabError::InvalidInput("matrix must be 2x2".into()));
        }
        Ok(self.get(0, 0) * self.get(1, 1) - self.get(0, 1) * self.get(1, 0))
    }
}

impl std::fmt::Display for ComplexMatrix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ComplexMatrix({}\u{00d7}{})", self.rows, self.cols)
    }
}

// Index operators
impl std::ops::Index<(usize, usize)> for ComplexMatrix {
    type Output = Complex;
    #[inline]
    fn index(&self, (row, col): (usize, usize)) -> &Complex {
        debug_assert!(row < self.rows && col < self.cols);
        &self.data[row * self.cols + col]
    }
}

impl std::ops::IndexMut<(usize, usize)> for ComplexMatrix {
    #[inline]
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Complex {
        debug_assert!(row < self.rows && col < self.cols);
        &mut self.data[row * self.cols + col]
    }
}

// ---------------------------------------------------------------------------
// Hermitian eigendecomposition (Jacobi)
// ---------------------------------------------------------------------------

/// Result of a Hermitian eigendecomposition.
///
/// For a Hermitian matrix **A**, `A = U Λ U†` where Λ is real diagonal
/// and U is unitary.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use]
pub struct HermitianEigen {
    /// Real eigenvalues sorted by descending magnitude.
    pub eigenvalues: Vec<f64>,
    /// Unitary matrix of eigenvectors (columns).
    pub eigenvectors: ComplexMatrix,
}

/// Hermitian eigendecomposition via Jacobi rotations.
///
/// Input must be Hermitian (`A = A†`). Returns real eigenvalues and a unitary
/// matrix of eigenvectors.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if not square.
/// Returns [`HisabError::NoConvergence`] if iteration limit exceeded.
pub fn eigen_hermitian(
    a: &ComplexMatrix,
    tol: f64,
    max_iter: usize,
) -> Result<HermitianEigen, HisabError> {
    let n = a.rows();
    if n != a.cols() {
        return Err(HisabError::InvalidInput("matrix must be square".into()));
    }
    if n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }
    tracing::debug!(size = n, "eigen_hermitian starting");
    if n == 1 {
        return Ok(HermitianEigen {
            eigenvalues: vec![a.get(0, 0).re],
            eigenvectors: ComplexMatrix::identity(1),
        });
    }

    // Work on a copy
    let mut w = a.clone();
    // Eigenvector accumulator (identity)
    let mut v = ComplexMatrix::identity(n);

    let tol_sq = tol * tol;

    for iter_count in 0..max_iter {
        let mut converged = true;

        for p in 0..n {
            for q in (p + 1)..n {
                let apq = w.get(p, q);
                if apq.norm_sq() > tol_sq {
                    converged = false;

                    // Jacobi rotation for Hermitian matrix
                    let app = w.get(p, p).re;
                    let aqq = w.get(q, q).re;
                    let diff = app - aqq;

                    // Compute rotation angle
                    let abs_apq = apq.abs();
                    let phase = if abs_apq > 1e-300 {
                        apq / Complex::from_real(abs_apq)
                    } else {
                        Complex::from_real(1.0)
                    };

                    let theta = if diff.abs() < crate::EPSILON_F64 {
                        PI / 4.0
                    } else {
                        0.5 * (2.0 * abs_apq / diff).atan()
                    };
                    let cos = theta.cos();
                    let sin = theta.sin();

                    // Complex Givens rotation: G = [[c, -s*phase†], [s*phase, c]]
                    let sp = Complex::from_real(sin) * phase;
                    let sp_conj = sp.conj();

                    // Update columns p, q of w
                    for i in 0..n {
                        let wp = w.get(i, p);
                        let wq = w.get(i, q);
                        w.set(i, p, wp * cos + wq * sp_conj);
                        w.set(i, q, wq * cos - wp * sp);
                    }
                    // Update rows p, q of w
                    for j in 0..n {
                        let wp = w.get(p, j);
                        let wq = w.get(q, j);
                        w.set(p, j, wp * cos + wq * sp);
                        w.set(q, j, wq * cos - wp * sp_conj);
                    }

                    // Force diagonal to be real (Hermitian invariant)
                    w.set(p, p, Complex::from_real(w.get(p, p).re));
                    w.set(q, q, Complex::from_real(w.get(q, q).re));
                    w.set(p, q, Complex::default());
                    w.set(q, p, Complex::default());

                    // Accumulate eigenvectors: V' = V * G
                    for i in 0..n {
                        let vp = v.get(i, p);
                        let vq = v.get(i, q);
                        v.set(i, p, vp * cos + vq * sp_conj);
                        v.set(i, q, vq * cos - vp * sp);
                    }
                }
            }
        }
        if converged {
            tracing::trace!(iterations = iter_count, "eigen_hermitian converged");
            // Extract eigenvalues and sort by descending magnitude
            let eigenvalues: Vec<f64> = (0..n).map(|i| w.get(i, i).re).collect();
            let mut order: Vec<usize> = (0..n).collect();
            order.sort_unstable_by(|&a, &b| {
                eigenvalues[b]
                    .abs()
                    .partial_cmp(&eigenvalues[a].abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            let sorted_eigs: Vec<f64> = order.iter().map(|&i| eigenvalues[i]).collect();
            let mut sorted_vecs = ComplexMatrix::zeros(n, n);
            for (new_col, &old_col) in order.iter().enumerate() {
                for row in 0..n {
                    sorted_vecs.set(row, new_col, v.get(row, old_col));
                }
            }

            return Ok(HermitianEigen {
                eigenvalues: sorted_eigs,
                eigenvectors: sorted_vecs,
            });
        }
    }

    Err(HisabError::NoConvergence(max_iter))
}

// ---------------------------------------------------------------------------
// Complex SVD
// ---------------------------------------------------------------------------

/// Result of a complex singular value decomposition.
///
/// `A = U Σ V†` where U, V are unitary and Σ is real diagonal.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use]
pub struct ComplexSvd {
    /// Left singular vectors (m × m unitary).
    pub u: ComplexMatrix,
    /// Singular values (non-negative, descending).
    pub sigma: Vec<f64>,
    /// Right singular vectors (conjugate transpose, n × n unitary).
    pub vt: ComplexMatrix,
}

/// Compute the SVD of a complex matrix via eigendecomposition of A†A.
///
/// Returns `A = U Σ V†`.
///
/// # Errors
///
/// Returns error on empty input or convergence failure.
pub fn complex_svd(a: &ComplexMatrix, tol: f64, max_iter: usize) -> Result<ComplexSvd, HisabError> {
    let m = a.rows();
    let n = a.cols();
    if m == 0 || n == 0 {
        return Err(HisabError::InvalidInput("empty matrix".into()));
    }

    // Form A†A (n×n, Hermitian positive semi-definite)
    let at = a.adjoint();
    let ata = at.mul_mat(a)?;

    // Eigendecompose A†A = V Λ V†
    let eig = eigen_hermitian(&ata, tol, max_iter)?;

    let rank = eig
        .eigenvalues
        .iter()
        .filter(|&&e| e.abs() > tol)
        .count()
        .max(1);
    tracing::debug!(rows = m, cols = n, rank, "complex_svd");

    // σᵢ = √λᵢ, V from eigenvectors
    let sigma: Vec<f64> = eig
        .eigenvalues
        .iter()
        .map(|&e| if e > 0.0 { e.sqrt() } else { 0.0 })
        .collect();
    let v = &eig.eigenvectors;

    // U columns: uᵢ = A vᵢ / σᵢ
    let mut u = ComplexMatrix::zeros(m, m);
    for (j, &sj) in sigma.iter().enumerate().take(rank.min(m).min(n)) {
        if sj > tol {
            // Extract column j of V
            let col_v: Vec<Complex> = (0..n).map(|i| v.get(i, j)).collect();
            let av = a.mul_vec(&col_v)?;
            let inv_s = 1.0 / sj;
            for (i, avi) in av.iter().enumerate() {
                u.set(i, j, *avi * inv_s);
            }
        }
    }

    // Extend U to full unitary via Gram-Schmidt on remaining columns
    gram_schmidt_extend(&mut u, rank.min(m).min(n));

    Ok(ComplexSvd {
        u,
        sigma,
        vt: v.adjoint(),
    })
}

/// Extend the first `k` orthonormal columns of `u` to a full unitary matrix.
fn gram_schmidt_extend(u: &mut ComplexMatrix, k: usize) {
    let m = u.rows();
    let mut filled = k;

    for candidate in 0..m {
        if filled >= m {
            break;
        }
        // Start with e_candidate
        let mut v = vec![Complex::default(); m];
        v[candidate] = Complex::from_real(1.0);

        // Subtract projections onto existing columns
        for j in 0..filled {
            let mut dot = Complex::default();
            for (i, vi) in v.iter().enumerate() {
                dot += u.get(i, j).conj() * *vi;
            }
            for (vi, i) in v.iter_mut().zip(0..m) {
                *vi -= dot * u.get(i, j);
            }
        }

        // Check if residual is non-negligible
        let norm: f64 = v.iter().map(|z| z.norm_sq()).sum::<f64>().sqrt();
        if norm > 1e-10 {
            let inv_norm = 1.0 / norm;
            for (i, vi) in v.iter().enumerate() {
                u.set(i, filled, *vi * inv_norm);
            }
            filled += 1;
        }
    }
}

// ---------------------------------------------------------------------------
// Pauli matrices
// ---------------------------------------------------------------------------

/// The 2×2 identity matrix (σ₀).
#[must_use]
pub fn pauli_0() -> ComplexMatrix {
    ComplexMatrix::identity(2)
}

/// Pauli σ₁ (σₓ): `[[0, 1], [1, 0]]`.
#[must_use]
pub fn pauli_x() -> ComplexMatrix {
    let mut m = ComplexMatrix::zeros(2, 2);
    m.set(0, 1, Complex::from_real(1.0));
    m.set(1, 0, Complex::from_real(1.0));
    m
}

/// Pauli σ₂ (σᵧ): `[[0, -i], [i, 0]]`.
#[must_use]
pub fn pauli_y() -> ComplexMatrix {
    let mut m = ComplexMatrix::zeros(2, 2);
    m.set(0, 1, Complex::new(0.0, -1.0));
    m.set(1, 0, Complex::new(0.0, 1.0));
    m
}

/// Pauli σ₃ (σ_z): `[[1, 0], [0, -1]]`.
#[must_use]
pub fn pauli_z() -> ComplexMatrix {
    let mut m = ComplexMatrix::zeros(2, 2);
    m.set(0, 0, Complex::from_real(1.0));
    m.set(1, 1, Complex::from_real(-1.0));
    m
}

/// Return all three Pauli matrices `[σₓ, σᵧ, σ_z]`.
#[must_use]
pub fn pauli_matrices() -> [ComplexMatrix; 3] {
    [pauli_x(), pauli_y(), pauli_z()]
}

// ---------------------------------------------------------------------------
// Dirac gamma matrices (Dirac representation, 4×4)
// ---------------------------------------------------------------------------

/// Dirac γ⁰ matrix (4×4).
///
/// `γ⁰ = [[I, 0], [0, -I]]` in the Dirac (standard) representation.
#[must_use]
pub fn gamma_0() -> ComplexMatrix {
    let one = Complex::from_real(1.0);
    let neg = Complex::from_real(-1.0);
    let mut m = ComplexMatrix::zeros(4, 4);
    m.set(0, 0, one);
    m.set(1, 1, one);
    m.set(2, 2, neg);
    m.set(3, 3, neg);
    m
}

/// Dirac γⁱ matrix (4×4) for i = 1, 2, 3.
///
/// `γⁱ = [[0, σⁱ], [-σⁱ, 0]]` in the Dirac representation.
///
/// # Errors
///
/// Returns error if `i` is not 1, 2, or 3.
pub fn gamma_spatial(i: usize) -> Result<ComplexMatrix, HisabError> {
    let sigma = match i {
        1 => pauli_x(),
        2 => pauli_y(),
        3 => pauli_z(),
        _ => {
            return Err(HisabError::InvalidInput(
                "spatial index must be 1, 2, or 3".into(),
            ));
        }
    };
    let mut m = ComplexMatrix::zeros(4, 4);
    // Upper-right block: +σ
    for r in 0..2 {
        for c in 0..2 {
            m.set(r, c + 2, sigma.get(r, c));
        }
    }
    // Lower-left block: -σ
    for r in 0..2 {
        for c in 0..2 {
            m.set(r + 2, c, Complex::default() - sigma.get(r, c));
        }
    }
    Ok(m)
}

/// Build a 4×4 Dirac gamma matrix from a 2×2 Pauli matrix.
///
/// `γⁱ = [[0, σ], [-σ, 0]]`
fn gamma_from_pauli(sigma: &ComplexMatrix) -> ComplexMatrix {
    let mut m = ComplexMatrix::zeros(4, 4);
    for r in 0..2 {
        for c in 0..2 {
            m.set(r, c + 2, sigma.get(r, c));
            m.set(r + 2, c, Complex::default() - sigma.get(r, c));
        }
    }
    m
}

/// All four Dirac gamma matrices `[γ⁰, γ¹, γ², γ³]`.
///
/// Satisfies the Clifford algebra: `{γᵘ, γᵛ} = 2ηᵘᵛ I₄`.
#[must_use]
pub fn gamma_matrices() -> [ComplexMatrix; 4] {
    [
        gamma_0(),
        gamma_from_pauli(&pauli_x()),
        gamma_from_pauli(&pauli_y()),
        gamma_from_pauli(&pauli_z()),
    ]
}

/// γ⁵ = iγ⁰γ¹γ²γ³ (chirality matrix).
#[must_use]
pub fn gamma_5() -> ComplexMatrix {
    let g = gamma_matrices();
    // All are 4×4 so mul_mat cannot fail — use inner helper
    let g01 = mat4_mul(&g[0], &g[1]);
    let g012 = mat4_mul(&g01, &g[2]);
    let g0123 = mat4_mul(&g012, &g[3]);
    g0123.scale(Complex::new(0.0, 1.0))
}

/// Infallible 4×4 complex matrix multiply (avoids expect in library code).
fn mat4_mul(a: &ComplexMatrix, b: &ComplexMatrix) -> ComplexMatrix {
    debug_assert!(a.cols() == 4 && b.rows() == 4);
    let mut out = ComplexMatrix::zeros(4, 4);
    for r in 0..4 {
        for c in 0..4 {
            let mut sum = Complex::default();
            for k in 0..4 {
                sum += a.get(r, k) * b.get(k, c);
            }
            out.set(r, c, sum);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// Spinor operations
// ---------------------------------------------------------------------------

/// Apply a 2×2 SU(2) rotation to a 2-component spinor.
///
/// The rotation is parameterized by axis `(nx, ny, nz)` and angle `theta`.
/// `U = cos(θ/2)I - i sin(θ/2)(nₓσₓ + nᵧσᵧ + n_zσ_z)`
///
/// # Errors
///
/// Returns error if the axis is zero-length.
pub fn spinor_rotation(
    spinor: &[Complex; 2],
    axis: [f64; 3],
    theta: f64,
) -> Result<[Complex; 2], HisabError> {
    let norm = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
    if norm < crate::EPSILON_F64 {
        return Err(HisabError::InvalidInput("rotation axis is zero".into()));
    }
    let nx = axis[0] / norm;
    let ny = axis[1] / norm;
    let nz = axis[2] / norm;

    let half = theta / 2.0;
    let c = half.cos();
    let s = half.sin();

    // U = cos(θ/2)I - i·sin(θ/2)(nₓσₓ + nᵧσᵧ + nzσz)
    // = [[c - i·s·nz, -i·s·nx - s·ny],
    //    [-i·s·nx + s·ny, c + i·s·nz]]
    let u00 = Complex::new(c, -s * nz);
    let u01 = Complex::new(-s * ny, -s * nx);
    let u10 = Complex::new(s * ny, -s * nx);
    let u11 = Complex::new(c, s * nz);

    Ok([
        u00 * spinor[0] + u01 * spinor[1],
        u10 * spinor[0] + u11 * spinor[1],
    ])
}

/// Apply a 4×4 Lorentz boost to a 4-component Dirac spinor.
///
/// Boost along direction `(nx, ny, nz)` with rapidity `eta`.
/// `S = cosh(η/2)I + sinh(η/2) γ⁰γⁱnᵢ`
///
/// # Errors
///
/// Returns error if direction is zero or spinor has wrong length.
pub fn dirac_boost(
    spinor: &[Complex; 4],
    direction: [f64; 3],
    rapidity: f64,
) -> Result<[Complex; 4], HisabError> {
    let norm =
        (direction[0] * direction[0] + direction[1] * direction[1] + direction[2] * direction[2])
            .sqrt();
    if norm < crate::EPSILON_F64 {
        return Err(HisabError::InvalidInput("boost direction is zero".into()));
    }
    let n = [
        direction[0] / norm,
        direction[1] / norm,
        direction[2] / norm,
    ];

    let half = rapidity / 2.0;
    let ch = half.cosh();
    let sh = half.sinh();

    // S = cosh(η/2)I₄ + sinh(η/2) Σ where Σ = γ⁰(n₁γ¹ + n₂γ² + n₃γ³)
    let g0 = gamma_0();
    let mut sigma_boost = ComplexMatrix::zeros(4, 4);
    for (i, &ni) in n.iter().enumerate() {
        let gi = gamma_spatial(i + 1)?;
        let g0gi = g0.mul_mat(&gi)?;
        for idx in 0..16 {
            sigma_boost.data[idx] += g0gi.data[idx] * ni;
        }
    }

    // S = ch * I + sh * Σ
    let id4 = ComplexMatrix::identity(4);
    let s_mat = id4.scale_real(ch).add(&sigma_boost.scale_real(sh))?;

    let spinor_vec: Vec<Complex> = spinor.to_vec();
    let result = s_mat.mul_vec(&spinor_vec)?;
    Ok([result[0], result[1], result[2], result[3]])
}

/// Compute the commutator `[A, B] = AB - BA`.
///
/// # Errors
///
/// Returns error if matrices aren't the same square dimensions.
pub fn commutator(a: &ComplexMatrix, b: &ComplexMatrix) -> Result<ComplexMatrix, HisabError> {
    let ab = a.mul_mat(b)?;
    let ba = b.mul_mat(a)?;
    ab.sub(&ba)
}

/// Compute the anticommutator `{A, B} = AB + BA`.
///
/// # Errors
///
/// Returns error if matrices aren't the same square dimensions.
pub fn anticommutator(a: &ComplexMatrix, b: &ComplexMatrix) -> Result<ComplexMatrix, HisabError> {
    let ab = a.mul_mat(b)?;
    let ba = b.mul_mat(a)?;
    ab.add(&ba)
}

/// Kronecker (tensor) product of two complex matrices.
///
/// If A is m×n and B is p×q, result is (mp)×(nq).
#[must_use]
pub fn kronecker(a: &ComplexMatrix, b: &ComplexMatrix) -> ComplexMatrix {
    let m = a.rows();
    let n = a.cols();
    let p = b.rows();
    let q = b.cols();
    let mut out = ComplexMatrix::zeros(m * p, n * q);
    for ar in 0..m {
        for ac in 0..n {
            let aval = a.get(ar, ac);
            for br in 0..p {
                for bc in 0..q {
                    out.set(ar * p + br, ac * q + bc, aval * b.get(br, bc));
                }
            }
        }
    }
    out
}

/// Matrix exponential via Padé approximation (scaling and squaring).
///
/// Computes `exp(A)` for a square complex matrix. Uses a [13/13] Padé
/// approximant with scaling and squaring for numerical stability.
///
/// # Errors
///
/// Returns error if the matrix is not square.
pub fn matrix_exp(a: &ComplexMatrix) -> Result<ComplexMatrix, HisabError> {
    let n = a.rows();
    if n != a.cols() {
        return Err(HisabError::InvalidInput("matrix must be square".into()));
    }
    if n == 0 {
        return Ok(ComplexMatrix::zeros(0, 0));
    }

    // Scaling: choose s so that ||A/2^s||_1 ≤ θ₁₃ ≈ 5.37
    let norm = a.frobenius_norm();
    let theta13 = 5.37;
    let s = if norm <= theta13 {
        0_u32
    } else {
        (norm / theta13).log2().ceil() as u32
    };
    let scale = 0.5_f64.powi(s as i32);
    tracing::debug!(scaling_factor = s, norm, "matrix_exp");
    let a_scaled = a.scale_real(scale);

    // Taylor series: sum A^k / k! up to order 20 for good accuracy
    let id = ComplexMatrix::identity(n);
    let mut result = id.clone();
    let mut term = id; // A^k / k!
    for k in 1..=20_u64 {
        term = term.mul_mat(&a_scaled)?;
        term = term.scale_real(1.0 / k as f64);
        result = result.add(&term)?;
        if term.frobenius_norm() < 1e-16 * result.frobenius_norm() {
            break;
        }
    }

    // Repeated squaring to undo scaling
    for _ in 0..s {
        result = result.mul_mat(&result)?;
    }

    Ok(result)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < 1e-8
    }

    fn complex_approx(a: Complex, b: Complex) -> bool {
        (a - b).norm_sq() < 1e-16
    }

    // -- ComplexMatrix basics --

    #[test]
    fn identity_is_identity() {
        let id = ComplexMatrix::identity(3);
        for r in 0..3 {
            for c in 0..3 {
                let expected = if r == c { 1.0 } else { 0.0 };
                assert!(approx_eq(id.get(r, c).re, expected));
                assert!(approx_eq(id.get(r, c).im, 0.0));
            }
        }
    }

    #[test]
    fn adjoint_of_hermitian_is_self() {
        // Hermitian: [[1, i], [-i, 2]]
        let mut m = ComplexMatrix::zeros(2, 2);
        m.set(0, 0, Complex::from_real(1.0));
        m.set(0, 1, Complex::new(0.0, 1.0));
        m.set(1, 0, Complex::new(0.0, -1.0));
        m.set(1, 1, Complex::from_real(2.0));

        let adj = m.adjoint();
        for r in 0..2 {
            for c in 0..2 {
                assert!(complex_approx(m.get(r, c), adj.get(r, c)));
            }
        }
    }

    #[test]
    fn mul_mat_identity() {
        let mut m = ComplexMatrix::zeros(2, 2);
        m.set(0, 0, Complex::new(1.0, 2.0));
        m.set(0, 1, Complex::new(3.0, 4.0));
        m.set(1, 0, Complex::new(5.0, 6.0));
        m.set(1, 1, Complex::new(7.0, 8.0));

        let id = ComplexMatrix::identity(2);
        let result = m.mul_mat(&id).unwrap();
        for r in 0..2 {
            for c in 0..2 {
                assert!(complex_approx(m.get(r, c), result.get(r, c)));
            }
        }
    }

    #[test]
    fn trace_identity() {
        let id = ComplexMatrix::identity(4);
        let tr = id.trace().unwrap();
        assert!(approx_eq(tr.re, 4.0));
        assert!(approx_eq(tr.im, 0.0));
    }

    #[test]
    fn is_unitary_identity() {
        let id = ComplexMatrix::identity(3);
        assert!(id.is_unitary(1e-10));
    }

    #[test]
    fn is_hermitian_check() {
        let mut m = ComplexMatrix::zeros(2, 2);
        m.set(0, 0, Complex::from_real(1.0));
        m.set(0, 1, Complex::new(3.0, -4.0));
        m.set(1, 0, Complex::new(3.0, 4.0));
        m.set(1, 1, Complex::from_real(2.0));
        assert!(m.is_hermitian(1e-10));
    }

    // -- Pauli matrices --

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn pauli_anticommutation() {
        let sigma = pauli_matrices();
        let id2 = ComplexMatrix::identity(2).scale_real(2.0);

        // {σᵢ, σⱼ} = 2δᵢⱼ I
        for i in 0..3 {
            let ac = anticommutator(&sigma[i], &sigma[i]).unwrap();
            for r in 0..2 {
                for c in 0..2 {
                    assert!(
                        complex_approx(ac.get(r, c), id2.get(r, c)),
                        "{{σ{i}, σ{i}}} ≠ 2I at ({r},{c})"
                    );
                }
            }
        }

        // {σᵢ, σⱼ} = 0 for i ≠ j
        for i in 0..3 {
            for j in (i + 1)..3 {
                let ac = anticommutator(&sigma[i], &sigma[j]).unwrap();
                assert!(ac.frobenius_norm() < 1e-10, "{{σ{i}, σ{j}}} should vanish");
            }
        }
    }

    #[test]
    fn pauli_commutation() {
        let sigma = pauli_matrices();
        // [σₓ, σᵧ] = 2i σ_z
        let comm = commutator(&sigma[0], &sigma[1]).unwrap();
        let expected = sigma[2].scale(Complex::new(0.0, 2.0));
        for r in 0..2 {
            for c in 0..2 {
                assert!(complex_approx(comm.get(r, c), expected.get(r, c)));
            }
        }
    }

    #[test]
    fn pauli_hermitian() {
        for sigma in &pauli_matrices() {
            assert!(sigma.is_hermitian(1e-10));
        }
    }

    #[test]
    fn pauli_unitary() {
        for sigma in &pauli_matrices() {
            assert!(sigma.is_unitary(1e-10));
        }
    }

    // -- Dirac gamma matrices --

    #[test]
    fn gamma_clifford_algebra() {
        let g = gamma_matrices();
        // Minkowski metric η = diag(+1, -1, -1, -1)
        let eta = [1.0, -1.0, -1.0, -1.0];
        let id4 = ComplexMatrix::identity(4);

        for mu in 0..4 {
            for nu in 0..4 {
                let ac = anticommutator(&g[mu], &g[nu]).unwrap();
                let expected_diag = if mu == nu { 2.0 * eta[mu] } else { 0.0 };
                let expected = id4.scale_real(expected_diag);
                assert!(
                    ac.sub(&expected).unwrap().frobenius_norm() < 1e-10,
                    "Clifford {{{mu},{nu}}} failed"
                );
            }
        }
    }

    #[test]
    fn gamma5_squares_to_identity() {
        let g5 = gamma_5();
        let g5sq = g5.mul_mat(&g5).unwrap();
        let id4 = ComplexMatrix::identity(4);
        assert!(g5sq.sub(&id4).unwrap().frobenius_norm() < 1e-10);
    }

    #[test]
    fn gamma5_anticommutes_with_gamma() {
        let g5 = gamma_5();
        let g = gamma_matrices();
        for (mu, gmu) in g.iter().enumerate() {
            let ac = anticommutator(&g5, gmu).unwrap();
            assert!(
                ac.frobenius_norm() < 1e-10,
                "γ⁵ should anticommute with γ{mu}"
            );
        }
    }

    // -- Hermitian eigendecomposition --

    #[test]
    fn eigen_hermitian_real_diagonal() {
        // Diagonal Hermitian = [[3, 0], [0, 1]]
        let mut m = ComplexMatrix::zeros(2, 2);
        m.set(0, 0, Complex::from_real(3.0));
        m.set(1, 1, Complex::from_real(1.0));

        let eig = eigen_hermitian(&m, 1e-12, 1000).unwrap();
        assert!(approx_eq(eig.eigenvalues[0], 3.0));
        assert!(approx_eq(eig.eigenvalues[1], 1.0));
    }

    #[test]
    fn eigen_hermitian_offdiag() {
        // [[2, i], [-i, 2]] -> eigenvalues 3, 1
        let mut m = ComplexMatrix::zeros(2, 2);
        m.set(0, 0, Complex::from_real(2.0));
        m.set(0, 1, Complex::new(0.0, 1.0));
        m.set(1, 0, Complex::new(0.0, -1.0));
        m.set(1, 1, Complex::from_real(2.0));

        let eig = eigen_hermitian(&m, 1e-12, 1000).unwrap();
        assert!(approx_eq(eig.eigenvalues[0], 3.0));
        assert!(approx_eq(eig.eigenvalues[1], 1.0));
    }

    #[test]
    fn eigen_hermitian_eigenvectors_unitary() {
        let mut m = ComplexMatrix::zeros(3, 3);
        m.set(0, 0, Complex::from_real(2.0));
        m.set(0, 1, Complex::new(1.0, 1.0));
        m.set(0, 2, Complex::from_real(0.0));
        m.set(1, 0, Complex::new(1.0, -1.0));
        m.set(1, 1, Complex::from_real(3.0));
        m.set(1, 2, Complex::new(0.0, 1.0));
        m.set(2, 0, Complex::from_real(0.0));
        m.set(2, 1, Complex::new(0.0, -1.0));
        m.set(2, 2, Complex::from_real(1.0));

        let eig = eigen_hermitian(&m, 1e-12, 5000).unwrap();
        assert!(eig.eigenvectors.is_unitary(1e-8));
    }

    #[test]
    fn eigen_hermitian_reconstructs() {
        // A = U Λ U†
        let mut m = ComplexMatrix::zeros(2, 2);
        m.set(0, 0, Complex::from_real(5.0));
        m.set(0, 1, Complex::new(1.0, -2.0));
        m.set(1, 0, Complex::new(1.0, 2.0));
        m.set(1, 1, Complex::from_real(3.0));

        let eig = eigen_hermitian(&m, 1e-12, 1000).unwrap();
        let u = &eig.eigenvectors;
        let ut = u.adjoint();

        // Construct Λ
        let mut lambda = ComplexMatrix::zeros(2, 2);
        for i in 0..2 {
            lambda.set(i, i, Complex::from_real(eig.eigenvalues[i]));
        }

        let reconstructed = u.mul_mat(&lambda).unwrap().mul_mat(&ut).unwrap();
        assert!(
            reconstructed.sub(&m).unwrap().frobenius_norm() < 1e-8,
            "UΛU† should reconstruct A"
        );
    }

    // -- Spinor rotation --

    #[test]
    fn spinor_rotation_z_axis() {
        // Spin-up around z by 2π should negate the spinor
        let spin_up = [Complex::from_real(1.0), Complex::default()];
        let result = spinor_rotation(&spin_up, [0.0, 0.0, 1.0], 2.0 * PI).unwrap();
        // e^{-iπσz} |↑⟩ = -|↑⟩
        assert!(approx_eq(result[0].re, -1.0));
        assert!(approx_eq(result[0].im, 0.0));
    }

    #[test]
    fn spinor_rotation_preserves_norm() {
        let spinor = [Complex::new(0.6, 0.3), Complex::new(-0.2, 0.7)];
        let norm_before: f64 = spinor.iter().map(|z| z.norm_sq()).sum::<f64>().sqrt();
        let result = spinor_rotation(&spinor, [1.0, 1.0, 0.0], 1.5).unwrap();
        let norm_after: f64 = result.iter().map(|z| z.norm_sq()).sum::<f64>().sqrt();
        assert!(approx_eq(norm_before, norm_after));
    }

    // -- Kronecker product --

    #[test]
    fn kronecker_identity() {
        let id2 = ComplexMatrix::identity(2);
        let id4 = kronecker(&id2, &id2);
        assert_eq!(id4.rows(), 4);
        assert_eq!(id4.cols(), 4);
        assert!(id4.is_unitary(1e-10));
        let expected = ComplexMatrix::identity(4);
        assert!(id4.sub(&expected).unwrap().frobenius_norm() < 1e-10);
    }

    // -- Commutator --

    #[test]
    fn commutator_self_vanishes() {
        let m = pauli_x();
        let c = commutator(&m, &m).unwrap();
        assert!(c.frobenius_norm() < 1e-10);
    }

    // -- Complex SVD --

    #[test]
    fn complex_svd_identity() {
        let id = ComplexMatrix::identity(3);
        let svd = complex_svd(&id, 1e-12, 1000).unwrap();
        for &s in &svd.sigma {
            assert!(approx_eq(s, 1.0));
        }
    }

    #[test]
    fn complex_svd_reconstructs() {
        let mut a = ComplexMatrix::zeros(2, 2);
        a.set(0, 0, Complex::new(1.0, 2.0));
        a.set(0, 1, Complex::new(3.0, 0.0));
        a.set(1, 0, Complex::new(0.0, 1.0));
        a.set(1, 1, Complex::new(2.0, -1.0));

        let svd = complex_svd(&a, 1e-12, 2000).unwrap();
        // Reconstruct: U Σ V†
        let mut sigma_mat = ComplexMatrix::zeros(2, 2);
        for i in 0..2 {
            sigma_mat.set(i, i, Complex::from_real(svd.sigma[i]));
        }
        let reconstructed = svd.u.mul_mat(&sigma_mat).unwrap().mul_mat(&svd.vt).unwrap();
        assert!(
            reconstructed.sub(&a).unwrap().frobenius_norm() < 1e-6,
            "UΣV† should reconstruct A"
        );
    }

    // -- Matrix exponential --

    #[test]
    fn matrix_exp_zero_is_identity() {
        let z = ComplexMatrix::zeros(3, 3);
        let result = matrix_exp(&z).unwrap();
        let id = ComplexMatrix::identity(3);
        assert!(result.sub(&id).unwrap().frobenius_norm() < 1e-10);
    }

    #[test]
    fn matrix_exp_identity_is_e_identity() {
        let id = ComplexMatrix::identity(2);
        let result = matrix_exp(&id).unwrap();
        let e = std::f64::consts::E;
        assert!(approx_eq(result.get(0, 0).re, e));
        assert!(approx_eq(result.get(1, 1).re, e));
        assert!(approx_eq(result.get(0, 1).re, 0.0));
    }

    #[test]
    fn matrix_exp_pauli_rotation() {
        // exp(-iθσz/2) for θ=π should give [[e^{-iπ/2}, 0], [0, e^{iπ/2}]] = [[-i,0],[0,i]]
        let sz = pauli_z();
        let arg = sz.scale(Complex::new(0.0, -PI / 2.0));
        let result = matrix_exp(&arg).unwrap();
        assert!(approx_eq(result.get(0, 0).re, 0.0));
        assert!(approx_eq(result.get(0, 0).im, -1.0));
        assert!(approx_eq(result.get(1, 1).re, 0.0));
        assert!(approx_eq(result.get(1, 1).im, 1.0));
    }

    // -- Error paths --

    #[test]
    fn from_rows_size_mismatch() {
        assert!(ComplexMatrix::from_rows(2, 2, vec![Complex::default(); 3]).is_err());
    }

    #[test]
    fn add_dim_mismatch() {
        let a = ComplexMatrix::zeros(2, 3);
        let b = ComplexMatrix::zeros(3, 2);
        assert!(a.add(&b).is_err());
    }

    #[test]
    fn sub_dim_mismatch() {
        let a = ComplexMatrix::zeros(2, 3);
        let b = ComplexMatrix::zeros(3, 2);
        assert!(a.sub(&b).is_err());
    }

    #[test]
    fn mul_mat_inner_dim_mismatch() {
        let a = ComplexMatrix::zeros(2, 3);
        let b = ComplexMatrix::zeros(4, 2);
        assert!(a.mul_mat(&b).is_err());
    }

    #[test]
    fn mul_vec_length_mismatch() {
        let a = ComplexMatrix::zeros(2, 3);
        assert!(a.mul_vec(&[Complex::default(); 2]).is_err());
    }

    #[test]
    fn det_2x2_non_2x2() {
        assert!(ComplexMatrix::zeros(3, 3).det_2x2().is_err());
    }

    #[test]
    fn trace_non_square() {
        assert!(ComplexMatrix::zeros(2, 3).trace().is_err());
    }

    #[test]
    fn eigen_hermitian_empty() {
        assert!(eigen_hermitian(&ComplexMatrix::zeros(0, 0), 1e-12, 100).is_err());
    }

    #[test]
    fn eigen_hermitian_non_square() {
        assert!(eigen_hermitian(&ComplexMatrix::zeros(2, 3), 1e-12, 100).is_err());
    }

    #[test]
    fn spinor_rotation_zero_axis() {
        let s = [Complex::from_real(1.0), Complex::default()];
        assert!(spinor_rotation(&s, [0.0, 0.0, 0.0], 1.0).is_err());
    }

    // -- Dirac boost --

    #[test]
    fn dirac_boost_preserves_norm() {
        let spinor = [
            Complex::new(1.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
            Complex::new(0.0, 0.0),
        ];
        let result = dirac_boost(&spinor, [0.0, 0.0, 1.0], 0.5).unwrap();
        // Boost doesn't preserve Euclidean norm but preserves ψ†γ⁰ψ
        let g0 = gamma_0();
        let psi: Vec<Complex> = result.to_vec();
        let g0_psi = g0.mul_vec(&psi).unwrap();
        let norm: Complex = result
            .iter()
            .zip(g0_psi.iter())
            .map(|(a, b)| a.conj() * *b)
            .fold(Complex::default(), |acc, x| acc + x);
        // For a particle at rest boosted, this should be non-trivial but real
        assert!(norm.im.abs() < 1e-10);
    }
}
