use crate::HisabError;

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
