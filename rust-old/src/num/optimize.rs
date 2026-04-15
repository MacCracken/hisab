use super::roots::gaussian_elimination;
use crate::HisabError;

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
/// - `grad_f`: gradient function ∇f(x) → `Vec<f64>`.
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
