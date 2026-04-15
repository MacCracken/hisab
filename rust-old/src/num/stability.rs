use crate::HisabError;

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
