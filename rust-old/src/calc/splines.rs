use super::*;

// ---------------------------------------------------------------------------

/// Evaluate a cubic Bezier at `t` using the de Casteljau algorithm (2D).
///
/// Also returns the subdivision — the two sets of control points for the
/// left `[0, t]` and right `[t, 1]` sub-curves.
#[must_use]
#[inline]
pub fn de_casteljau_split(
    p0: Vec2,
    p1: Vec2,
    p2: Vec2,
    p3: Vec2,
    t: f32,
) -> (Vec2, [Vec2; 4], [Vec2; 4]) {
    let u = 1.0 - t;
    // Level 1
    let q0 = p0 * u + p1 * t;
    let q1 = p1 * u + p2 * t;
    let q2 = p2 * u + p3 * t;
    // Level 2
    let r0 = q0 * u + q1 * t;
    let r1 = q1 * u + q2 * t;
    // Level 3 (the point)
    let s = r0 * u + r1 * t;

    let left = [p0, q0, r0, s];
    let right = [s, r1, q2, p3];
    (s, left, right)
}

// ---------------------------------------------------------------------------
// Catmull-Rom splines
// ---------------------------------------------------------------------------

/// Evaluate a Catmull-Rom spline segment at parameter `t` in [0, 1].
///
/// Takes four control points: `p0` and `p3` are the tangent-influencing
/// neighbors, `p1` and `p2` are the interpolated segment endpoints.
/// The curve passes through `p1` at `t=0` and `p2` at `t=1`.
#[must_use]
#[inline]
pub fn catmull_rom(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let t2 = t * t;
    let t3 = t2 * t;
    // Standard Catmull-Rom matrix form (alpha = 0.5)
    0.5 * ((2.0 * p1)
        + (-p0 + p2) * t
        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t2
        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t3)
}

// ---------------------------------------------------------------------------
// B-spline evaluation
// ---------------------------------------------------------------------------

/// Evaluate a uniform B-spline of arbitrary degree using de Boor's algorithm.
///
/// - `degree`: spline degree (1 = linear, 2 = quadratic, 3 = cubic).
/// - `control_points`: the control polygon.
/// - `knots`: the knot vector (length = control_points.len() + degree + 1).
/// - `t`: parameter value (must be within the valid knot range).
///
/// Returns `None` if inputs are invalid or `t` is out of range.
#[must_use = "returns the evaluated spline point"]
#[inline]
pub fn bspline_eval(degree: usize, control_points: &[Vec3], knots: &[f64], t: f64) -> Option<Vec3> {
    let n = control_points.len();
    if n == 0 || knots.len() != n + degree + 1 {
        return None;
    }

    // Find the knot span: largest k such that knots[k] <= t, clamped to valid range
    if t < knots[degree] || t > knots[n] {
        return None;
    }
    let mut k = degree;
    while k < n - 1 && knots[k + 1] <= t {
        k += 1;
    }

    // De Boor's algorithm — stack buffer for degree <= 4, heap otherwise
    let mut buf = [Vec3::ZERO; 5];
    let d: &mut [Vec3] = if degree < 5 {
        for j in 0..=degree {
            buf[j] = control_points[k - degree + j];
        }
        &mut buf[..=degree]
    } else {
        // Fallback for high degree (rare)
        let mut v: Vec<Vec3> = (0..=degree)
            .map(|j| control_points[k - degree + j])
            .collect();
        // High-degree fallback: use heap allocation and return early
        return {
            for r in 1..=degree {
                for j in (r..=degree).rev() {
                    let i = k - degree + j;
                    let denom = knots[i + degree + 1 - r] - knots[i];
                    if denom.abs() < crate::EPSILON_F64 {
                        continue;
                    }
                    let alpha = ((t - knots[i]) / denom) as f32;
                    v[j] = v[j - 1] * (1.0 - alpha) + v[j] * alpha;
                }
            }
            Some(v[degree])
        };
    };

    for r in 1..=degree {
        for j in (r..=degree).rev() {
            let i = k - degree + j;
            let denom = knots[i + degree + 1 - r] - knots[i];
            if denom.abs() < crate::EPSILON_F64 {
                continue;
            }
            let alpha = ((t - knots[i]) / denom) as f32;
            d[j] = d[j - 1] * (1.0 - alpha) + d[j] * alpha;
        }
    }

    Some(d[degree])
}

// ---------------------------------------------------------------------------
// NURBS (Non-Uniform Rational B-Splines)
// ---------------------------------------------------------------------------

/// Evaluate a NURBS (Non-Uniform Rational B-Spline) curve at parameter `t`.
///
/// A NURBS curve is a rational generalization of a B-spline, where each
/// control point has an associated weight. This allows exact representation
/// of conic sections (circles, ellipses) and other curves that B-splines
/// cannot represent exactly.
///
/// - `degree`: spline degree (1 = linear, 2 = quadratic, 3 = cubic).
/// - `control_points`: the control polygon (in 3D).
/// - `weights`: per-control-point weights (must be positive; uniform = all 1.0).
/// - `knots`: the knot vector (length = control_points.len() + degree + 1).
/// - `t`: parameter value (within valid knot range).
///
/// Returns `None` if inputs are invalid.
#[must_use = "returns the evaluated NURBS point"]
#[allow(clippy::needless_range_loop)]
pub fn nurbs_eval(
    degree: usize,
    control_points: &[Vec3],
    weights: &[f64],
    knots: &[f64],
    t: f64,
) -> Option<Vec3> {
    let n = control_points.len();
    if n == 0 || weights.len() != n || knots.len() != n + degree + 1 {
        return None;
    }
    if t < knots[degree] || t > knots[n] {
        return None;
    }

    // Find the knot span
    let mut k = degree;
    while k < n - 1 && knots[k + 1] <= t {
        k += 1;
    }

    // De Boor's algorithm on weighted (homogeneous) control points
    // P_w[i] = (w[i]*x[i], w[i]*y[i], w[i]*z[i], w[i])
    let mut pw: Vec<[f64; 4]> = (0..=degree)
        .map(|j| {
            let idx = k - degree + j;
            let w = weights[idx];
            let p = control_points[idx];
            [p.x as f64 * w, p.y as f64 * w, p.z as f64 * w, w]
        })
        .collect();

    for r in 1..=degree {
        for j in (r..=degree).rev() {
            let i = k - degree + j;
            let denom = knots[i + degree + 1 - r] - knots[i];
            if denom.abs() < crate::EPSILON_F64 {
                continue;
            }
            let alpha = (t - knots[i]) / denom;
            let one_minus = 1.0 - alpha;
            for c in 0..4 {
                pw[j][c] = one_minus * pw[j - 1][c] + alpha * pw[j][c];
            }
        }
    }

    // Perspective divide
    let w = pw[degree][3];
    if w.abs() < crate::EPSILON_F64 {
        return None;
    }
    Some(Vec3::new(
        (pw[degree][0] / w) as f32,
        (pw[degree][1] / w) as f32,
        (pw[degree][2] / w) as f32,
    ))
}

// ---------------------------------------------------------------------------
// Hermite spline with TCB (tension, continuity, bias)
// ---------------------------------------------------------------------------

/// Evaluate a Kochanek-Bartels (TCB) Hermite spline segment at parameter `t` in \[0, 1\].
///
/// Takes four control points `p0..p3` (the curve interpolates `p1` to `p2`)
/// and TCB parameters:
/// - `tension`: 0 = Catmull-Rom, 1 = sharp corners, -1 = loose
/// - `continuity`: 0 = smooth, nonzero = corner at key
/// - `bias`: 0 = symmetric, 1 = overshoots toward, -1 = overshoots away
#[must_use]
#[allow(clippy::many_single_char_names, clippy::too_many_arguments)]
pub fn hermite_tcb(
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    t: f32,
    tension: f32,
    continuity: f32,
    bias: f32,
) -> Vec3 {
    // Outgoing tangent at p1
    let a = (1.0 - tension) * (1.0 + continuity) * (1.0 + bias) * 0.5;
    let b = (1.0 - tension) * (1.0 - continuity) * (1.0 - bias) * 0.5;
    let m0 = (p1 - p0) * a + (p2 - p1) * b;

    // Incoming tangent at p2
    let c = (1.0 - tension) * (1.0 - continuity) * (1.0 + bias) * 0.5;
    let d = (1.0 - tension) * (1.0 + continuity) * (1.0 - bias) * 0.5;
    let m1 = (p2 - p1) * c + (p3 - p2) * d;

    // Cubic Hermite basis
    let t2 = t * t;
    let t3 = t2 * t;
    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = t3 - 2.0 * t2 + t;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = t3 - t2;

    p1 * h00 + m0 * h10 + p2 * h01 + m1 * h11
}

// ---------------------------------------------------------------------------
// Monotone cubic interpolation (Fritsch-Carlson)
// ---------------------------------------------------------------------------

/// Monotone cubic interpolation using the Fritsch-Carlson method.
///
/// Given sorted knots `xs` and values `ys`, evaluates the interpolant at `x`.
/// Guarantees monotonicity between data points (no overshoot), making it
/// ideal for deterministic replay and animation curves.
///
/// Returns `None` if inputs are invalid (fewer than 2 points, mismatched lengths,
/// unsorted xs, or `x` is outside the data range).
#[must_use]
pub fn monotone_cubic(xs: &[f64], ys: &[f64], x: f64) -> Option<f64> {
    let n = xs.len();
    if n < 2 || ys.len() != n {
        return None;
    }
    if x < xs[0] || x > xs[n - 1] {
        return None;
    }

    // Compute secants
    let mut deltas = Vec::with_capacity(n - 1);
    let mut hs = Vec::with_capacity(n - 1);
    for i in 0..n - 1 {
        let h = xs[i + 1] - xs[i];
        if h <= 0.0 {
            return None; // Not sorted
        }
        hs.push(h);
        deltas.push((ys[i + 1] - ys[i]) / h);
    }

    // Initial tangents (average of adjacent secants)
    let mut ms = vec![0.0; n];
    ms[0] = deltas[0];
    ms[n - 1] = deltas[n - 2];
    for i in 1..n - 1 {
        ms[i] = (deltas[i - 1] + deltas[i]) * 0.5;
    }

    // Fritsch-Carlson monotonicity constraint
    for i in 0..n - 1 {
        if deltas[i].abs() < 1e-30 {
            ms[i] = 0.0;
            ms[i + 1] = 0.0;
        } else {
            let alpha = ms[i] / deltas[i];
            let beta = ms[i + 1] / deltas[i];
            let s = alpha * alpha + beta * beta;
            if s > 9.0 {
                let tau = 3.0 / s.sqrt();
                ms[i] = tau * alpha * deltas[i];
                ms[i + 1] = tau * beta * deltas[i];
            }
        }
    }

    // Find interval and evaluate cubic Hermite
    let mut k = 0;
    while k < n - 2 && xs[k + 1] < x {
        k += 1;
    }

    let h = hs[k];
    let t = (x - xs[k]) / h;
    let t2 = t * t;
    let t3 = t2 * t;

    let h00 = 2.0 * t3 - 3.0 * t2 + 1.0;
    let h10 = (t3 - 2.0 * t2 + t) * h;
    let h01 = -2.0 * t3 + 3.0 * t2;
    let h11 = (t3 - t2) * h;

    Some(h00 * ys[k] + h10 * ms[k] + h01 * ys[k + 1] + h11 * ms[k + 1])
}

// ---------------------------------------------------------------------------
// Arc-length parameterization
// ---------------------------------------------------------------------------

/// Approximate the arc length of a cubic Bezier curve in 3D.
///
/// Uses `n` linear segments to approximate. Higher `n` = more accurate.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "returns the computed arc length"]
#[inline]
pub fn bezier_cubic_3d_arc_length(
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    n: usize,
) -> Result<f32, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let mut length = 0.0f32;
    let mut prev = p0;
    for i in 1..=n {
        let t = i as f32 / n as f32;
        let curr = bezier_cubic_3d(p0, p1, p2, p3, t);
        length += (curr - prev).length();
        prev = curr;
    }
    Ok(length)
}

/// Re-parameterize a cubic Bezier by arc length.
///
/// Given a normalized distance `s` in [0, 1] (where 0 = start, 1 = end),
/// returns the corresponding `t` parameter.
/// `n` controls the accuracy (number of linear segments for the lookup table).
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "returns the parameter at the given arc length"]
#[inline]
pub fn bezier_cubic_3d_param_at_length(
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    s: f32,
    n: usize,
) -> Result<f32, HisabError> {
    if s <= 0.0 {
        return Ok(0.0);
    }
    if s >= 1.0 {
        return Ok(1.0);
    }
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }

    // Build cumulative arc-length table (O(n))
    let mut table = Vec::with_capacity(n + 1);
    table.push(0.0f32);
    let mut prev = p0;
    for i in 1..=n {
        let t = i as f32 / n as f32;
        let curr = bezier_cubic_3d(p0, p1, p2, p3, t);
        let seg = (curr - prev).length();
        table.push(table[i - 1] + seg);
        prev = curr;
    }

    let total = table.last().copied().unwrap_or(0.0);
    let target = s * total;

    // Binary search the table for the segment containing the target length
    let mut lo = 0usize;
    let mut hi = n;
    while lo < hi {
        let mid = (lo + hi) / 2;
        if table[mid] < target {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    // Linearly interpolate within the segment
    if lo == 0 {
        return Ok(0.0);
    }
    let seg_start = table[lo - 1];
    let seg_end = table[lo];
    let seg_len = seg_end - seg_start;
    let frac = if seg_len > crate::EPSILON_F32 {
        (target - seg_start) / seg_len
    } else {
        0.0
    };

    Ok(((lo - 1) as f32 + frac) / n as f32)
}

// ---------------------------------------------------------------------------
// Gauss-Legendre quadrature
