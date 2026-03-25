use super::*;

// ---------------------------------------------------------------------------

/// Numerical integration using 5-point Gauss-Legendre quadrature.
///
/// More accurate than Simpson's for smooth functions with fewer evaluations.
/// Integrates `f` over `[a, b]`.
#[must_use]
#[inline]
pub fn integral_gauss_legendre_5(f: impl Fn(f64) -> f64, a: f64, b: f64) -> f64 {
    // 5-point GL nodes and weights on [-1, 1]
    const NODES: [f64; 5] = [
        -0.906179845938664,
        -0.538469310105683,
        0.0,
        0.538469310105683,
        0.906179845938664,
    ];
    const WEIGHTS: [f64; 5] = [
        0.236926885056189,
        0.478628670499366,
        0.568888888888889,
        0.478628670499366,
        0.236926885056189,
    ];

    let half = (b - a) * 0.5;
    let mid = (a + b) * 0.5;
    let sum = WEIGHTS[0] * f(mid + half * NODES[0])
        + WEIGHTS[1] * f(mid + half * NODES[1])
        + WEIGHTS[2] * f(mid + half * NODES[2])
        + WEIGHTS[3] * f(mid + half * NODES[3])
        + WEIGHTS[4] * f(mid + half * NODES[4]);
    sum * half
}

/// Composite Gauss-Legendre quadrature (5-point) over `n` sub-intervals.
///
/// Divides `[a, b]` into `n` panels and applies 5-point GL to each.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "returns the computed integral"]
#[inline]
pub fn integral_gauss_legendre(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let h = (b - a) / n as f64;
    let mut total = 0.0;
    for i in 0..n {
        let lo = a + i as f64 * h;
        let hi = lo + h;
        total += integral_gauss_legendre_5(&f, lo, hi);
    }
    Ok(total)
}

// ---------------------------------------------------------------------------
// Easing functions

// ---------------------------------------------------------------------------

/// Adaptive Simpson's rule for numerical integration.
///
/// Recursively subdivides `[a, b]` until the error estimate is below `tol`.
/// Uses Richardson extrapolation to estimate the error.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInterval`] if `a >= b`.
/// Returns [`HisabError::NoConvergence`] if maximum recursion depth (50) is exceeded.
#[must_use = "returns the computed integral or an error"]
pub fn integral_adaptive_simpson(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    tol: f64,
) -> Result<f64, HisabError> {
    if a >= b {
        return Err(HisabError::InvalidInterval);
    }
    let fa = f(a);
    let fb = f(b);
    let mid = (a + b) * 0.5;
    let fmid = f(mid);
    let whole = (b - a) / 6.0 * (fa + 4.0 * fmid + fb);
    adaptive_simpson_recursive(&f, a, b, fa, fb, fmid, whole, tol, 50)
}

#[allow(clippy::too_many_arguments)]
fn adaptive_simpson_recursive(
    f: &impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    fa: f64,
    fb: f64,
    fmid: f64,
    whole: f64,
    tol: f64,
    depth: usize,
) -> Result<f64, HisabError> {
    if depth == 0 {
        return Err(HisabError::NoConvergence(50));
    }

    let mid = (a + b) * 0.5;
    let left_mid = (a + mid) * 0.5;
    let right_mid = (mid + b) * 0.5;
    let flm = f(left_mid);
    let frm = f(right_mid);

    let h = (b - a) * 0.5;
    let left = h / 6.0 * (fa + 4.0 * flm + fmid);
    let right = h / 6.0 * (fmid + 4.0 * frm + fb);
    let refined = left + right;

    // Richardson extrapolation error estimate
    let error = (refined - whole) / 15.0;

    if error.abs() < tol {
        return Ok(refined + error);
    }

    let left_result =
        adaptive_simpson_recursive(f, a, mid, fa, fmid, flm, left, tol * 0.5, depth - 1)?;
    let right_result =
        adaptive_simpson_recursive(f, mid, b, fmid, fb, frm, right, tol * 0.5, depth - 1)?;
    Ok(left_result + right_result)
}

/// Monte Carlo integration over an N-dimensional hyperrectangle.
///
/// Estimates `∫∫...∫ f(x) dx` over the region defined by `bounds`,
/// where `bounds[i] = (lower_i, upper_i)`.
///
/// Uses a simple pseudo-random LCG for deterministic reproducibility
/// (seeded from dimension count and sample count).
///
/// Returns `(estimate, standard_error)`.
///
/// # Errors
///
/// Returns [`HisabError::InvalidInput`] if `bounds` is empty or `n_samples` is zero.
/// Returns [`HisabError::InvalidInterval`] if any bound has `lower >= upper`.
#[must_use = "returns the estimated integral and standard error or an error"]
pub fn integral_monte_carlo(
    f: impl Fn(&[f64]) -> f64,
    bounds: &[(f64, f64)],
    n_samples: usize,
) -> Result<(f64, f64), HisabError> {
    let dim = bounds.len();
    if dim == 0 {
        return Err(HisabError::InvalidInput("empty bounds".into()));
    }
    if n_samples == 0 {
        return Err(HisabError::ZeroSteps);
    }

    // Compute volume of the hyperrectangle
    let mut volume = 1.0;
    for &(lo, hi) in bounds {
        if lo >= hi {
            return Err(HisabError::InvalidInterval);
        }
        volume *= hi - lo;
    }

    // Simple LCG for deterministic pseudo-random numbers
    let mut rng_state: u64 = 6364136223846793005_u64
        .wrapping_mul(dim as u64)
        .wrapping_add(n_samples as u64);
    let lcg_next = |state: &mut u64| -> f64 {
        *state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        // Map to [0, 1)
        (*state >> 11) as f64 / (1u64 << 53) as f64
    };

    let mut x = vec![0.0; dim];
    let mut sum = 0.0;
    let mut sum_sq = 0.0;

    for _ in 0..n_samples {
        for (d, &(lo, hi)) in bounds.iter().enumerate() {
            x[d] = lo + (hi - lo) * lcg_next(&mut rng_state);
        }
        let val = f(&x);
        sum += val;
        sum_sq += val * val;
    }

    let n = n_samples as f64;
    let mean = sum / n;
    let variance = (sum_sq / n - mean * mean).max(0.0);
    let std_error = (variance / n).sqrt() * volume;
    let estimate = mean * volume;

    Ok((estimate, std_error))
}

// ---------------------------------------------------------------------------
// Multivariable calculus

