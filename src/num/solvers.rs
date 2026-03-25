use crate::HisabError;

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
