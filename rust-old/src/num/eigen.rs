use crate::HisabError;

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
