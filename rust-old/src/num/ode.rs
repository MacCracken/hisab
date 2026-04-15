use super::rng::Pcg32;
use super::roots::gaussian_elimination;
use crate::HisabError;

// ---------------------------------------------------------------------------
// ODE solvers
// ---------------------------------------------------------------------------

/// Perform one RK4 step in-place, reusing scratch buffers.
///
/// All k1–k4 and tmp buffers are allocated by the caller and reused across steps,
/// eliminating per-step heap allocations. The `comp` buffer carries Neumaier
/// compensation terms across steps, reducing floating-point accumulation error.
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
    comp: &mut [f64],
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

    // Neumaier-compensated accumulation: tracks low-order bits across steps.
    let h6 = h / 6.0;
    for i in 0..dim {
        let delta = h6 * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
        let new_sum = y[i] + delta;
        // Capture the rounding error and accumulate into the compensation term.
        if y[i].abs() >= delta.abs() {
            comp[i] += (y[i] - new_sum) + delta;
        } else {
            comp[i] += (delta - new_sum) + y[i];
        }
        y[i] = new_sum + comp[i];
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
    // Neumaier compensation buffer — persists across steps.
    let mut comp = vec![0.0f64; dim];

    for _ in 0..n {
        rk4_step(
            &f, t, h, &mut y, &mut k1, &mut k2, &mut k3, &mut k4, &mut tmp, &mut comp, dim,
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
    // Neumaier compensation buffer — persists across steps.
    let mut comp = vec![0.0f64; dim];
    let mut trajectory = Vec::with_capacity(n + 1);
    trajectory.push((t, y.clone()));

    for _ in 0..n {
        rk4_step(
            &f, t, h, &mut y, &mut k1, &mut k2, &mut k3, &mut k4, &mut tmp, &mut comp, dim,
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
                Err(_) => {
                    tracing::warn!("backward_euler: Newton iteration failed (singular Jacobian)");
                    break;
                }
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
            tracing::warn!("bdf2: Newton iteration failed in bootstrap step (singular Jacobian)");
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
                tracing::warn!("bdf2: Newton iteration failed (singular Jacobian)");
                break;
            }
        }

        y_prev.copy_from_slice(&y_curr);
        y_curr.copy_from_slice(&y_guess);
    }

    Ok(y_curr)
}

/// BDF-k (Backward Differentiation Formula, orders 3–5) for stiff ODE systems.
///
/// Higher-order implicit methods. Uses backward Euler and BDF-2 for bootstrap steps.
///
/// - `order`: BDF order (3, 4, or 5).
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
/// Returns [`HisabError::InvalidInput`] if `order` is not 3, 4, or 5.
#[must_use = "contains the final state or an error"]
#[allow(clippy::too_many_arguments, clippy::needless_range_loop)]
pub fn bdf(
    f: impl Fn(f64, &[f64], &mut [f64]),
    jac: impl Fn(f64, &[f64], &mut Vec<Vec<f64>>),
    t0: f64,
    y0: &[f64],
    t_end: f64,
    n: usize,
    newton_tol: f64,
    max_newton: usize,
    order: usize,
) -> Result<Vec<f64>, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    if !(3..=5).contains(&order) {
        return Err(HisabError::InvalidInput(
            "bdf order must be 3, 4, or 5".into(),
        ));
    }
    let dim = y0.len();
    let h = (t_end - t0) / n as f64;

    // BDF coefficients: alpha[0..order] * y_history + beta * h * f(t_{k+1}, y_{k+1}) = 0
    // Stored as: predictor coefficients for y_{k+1} and the f-coefficient
    let (alphas, beta): (&[f64], f64) = match order {
        3 => (&[18.0 / 11.0, -9.0 / 11.0, 2.0 / 11.0], 6.0 / 11.0),
        4 => (
            &[48.0 / 25.0, -36.0 / 25.0, 16.0 / 25.0, -3.0 / 25.0],
            12.0 / 25.0,
        ),
        5 => (
            &[
                300.0 / 137.0,
                -300.0 / 137.0,
                200.0 / 137.0,
                -75.0 / 137.0,
                12.0 / 137.0,
            ],
            60.0 / 137.0,
        ),
        _ => return Err(HisabError::InvalidInput("unreachable".into())),
    };

    // Bootstrap: use backward Euler for initial steps
    let bootstrap_steps = order - 1;
    let mut history: Vec<Vec<f64>> = Vec::with_capacity(order);
    history.push(y0.to_vec());

    let mut t = t0;
    let mut f_val = vec![0.0; dim];
    let mut j_mat = vec![vec![0.0; dim]; dim];
    let mut y_guess = vec![0.0; dim];

    // Bootstrap with backward Euler
    for _ in 0..bootstrap_steps.min(n) {
        t += h;
        y_guess.copy_from_slice(history.last().map_or(y0, |v| v.as_slice()));

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
                let prev = history.last().map_or(y0[i], |v| v[i]);
                row.push(-(y_guess[i] - prev - h * f_val[i]));
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
                tracing::warn!(
                    "bdf: Newton iteration failed in bootstrap step (singular Jacobian)"
                );
                break;
            }
        }
        history.push(y_guess.clone());
    }

    if n <= bootstrap_steps {
        return Ok(history.pop().unwrap_or_else(|| y0.to_vec()));
    }

    // Main BDF-k loop
    for _ in bootstrap_steps..n {
        t += h;

        // Predictor: weighted sum of history
        for i in 0..dim {
            y_guess[i] = 0.0;
            for (k, alpha) in alphas.iter().enumerate() {
                let hist_idx = history.len() - 1 - k;
                y_guess[i] += alpha * history[hist_idx][i];
            }
        }

        // Newton corrector
        for _ in 0..max_newton {
            f(t, &y_guess, &mut f_val);
            jac(t, &y_guess, &mut j_mat);

            let mut aug: Vec<Vec<f64>> = Vec::with_capacity(dim);
            for i in 0..dim {
                let mut row = Vec::with_capacity(dim + 1);
                for j in 0..dim {
                    let ident = if i == j { 1.0 } else { 0.0 };
                    row.push(ident - beta * h * j_mat[i][j]);
                }
                // Residual: y_guess - sum(alpha_k * y_{n-k}) - beta*h*f
                let mut pred = 0.0;
                for (k, alpha) in alphas.iter().enumerate() {
                    let hist_idx = history.len() - 1 - k;
                    pred += alpha * history[hist_idx][i];
                }
                let rhs = y_guess[i] - pred - beta * h * f_val[i];
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
                tracing::warn!("bdf: Newton iteration failed (singular Jacobian)");
                break;
            }
        }

        // Slide history window
        if history.len() >= order {
            history.remove(0);
        }
        history.push(y_guess.clone());
    }

    Ok(history.pop().unwrap_or_else(|| y0.to_vec()))
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

/// Yoshida 4th-order symplectic integrator step.
///
/// Fourth-order composition method using the Yoshida triple-jump coefficients.
/// Significantly more accurate than Verlet while remaining symplectic and
/// time-reversible.
///
/// - `acc_fn`: computes acceleration from `(t, position, &mut acceleration_out)`.
/// - `pos`: current position vector (modified in place).
/// - `vel`: current velocity vector (modified in place).
/// - `t`: current time.
/// - `dt`: time step.
#[inline]
pub fn yoshida4_step(
    acc_fn: &impl Fn(f64, &[f64], &mut [f64]),
    pos: &mut [f64],
    vel: &mut [f64],
    t: f64,
    dt: f64,
) {
    // Yoshida triple-jump coefficients
    const CBRT2: f64 = 1.259_921_049_894_873_2; // 2^(1/3)
    const W1: f64 = 1.0 / (2.0 - CBRT2);
    const W0: f64 = -CBRT2 / (2.0 - CBRT2);

    // Position and velocity sub-step weights
    let c = [W1 * 0.5, (W0 + W1) * 0.5, (W0 + W1) * 0.5, W1 * 0.5];
    let d = [W1, W0, W1];

    let dim = pos.len();
    let mut acc = vec![0.0; dim];

    // Initial drift
    for i in 0..dim {
        pos[i] += c[0] * dt * vel[i];
    }

    for step in 0..3 {
        // Kick
        acc_fn(t, pos, &mut acc);
        for i in 0..dim {
            vel[i] += d[step] * dt * acc[i];
        }
        // Drift
        for i in 0..dim {
            pos[i] += c[step + 1] * dt * vel[i];
        }
    }
}

/// Run a Yoshida 4th-order symplectic integration over a time span.
///
/// Returns `(final_position, final_velocity)`.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
/// Returns [`HisabError::InvalidInput`] if `pos0` and `vel0` differ in length.
#[must_use = "contains the final state or an error"]
pub fn yoshida4(
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
        yoshida4_step(&acc_fn, &mut pos, &mut vel, t, dt);
        t += dt;
    }
    Ok((pos, vel))
}
