//! Calculus: differentiation, integration, interpolation, curves.
//!
//! Provides numerical differentiation, integration (trapezoidal and Simpson's),
//! linear interpolation, and Bezier curve evaluation.

use glam::{Vec2, Vec3};

/// Numerical derivative using the central difference method.
///
/// `f`: the function to differentiate.
/// `x`: the point at which to evaluate the derivative.
/// `h`: the step size (smaller = more accurate but risk of cancellation).
#[inline]
pub fn derivative(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Numerical integration using the trapezoidal rule.
///
/// Divides [a, b] into `n` sub-intervals.
#[inline]
pub fn integral_trapezoidal(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    assert!(n > 0, "n must be positive");
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));
    for i in 1..n {
        sum += f(a + i as f64 * h);
    }
    sum * h
}

/// Numerical integration using Simpson's rule.
///
/// `n` must be even. If odd, it is rounded up.
#[inline]
pub fn integral_simpson(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    let n = if n % 2 == 1 { n + 1 } else { n };
    assert!(n > 0, "n must be positive");
    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);

    // Process pairs: odd indices get coefficient 4, even get 2.
    // Unrolled to avoid branch per iteration.
    let mut i = 1;
    while i < n {
        sum += 4.0 * f(a + i as f64 * h);
        sum += 2.0 * f(a + (i + 1) as f64 * h);
        i += 2;
    }
    // Correct the last even-index term (we added 2*f(b) but f(b) is already counted)
    sum -= 2.0 * f(b);

    sum * h / 3.0
}

/// Linear interpolation between two f64 values.
#[inline]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Evaluate a quadratic Bezier curve at parameter `t` in [0, 1].
///
/// B(t) = (1-t)^2 * p0 + 2(1-t)t * p1 + t^2 * p2
#[inline]
pub fn bezier_quadratic(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    p0 * (u * u) + p1 * (2.0 * u * t) + p2 * (t * t)
}

/// Evaluate a cubic Bezier curve at parameter `t` in [0, 1].
///
/// B(t) = (1-t)^3 * p0 + 3(1-t)^2*t * p1 + 3(1-t)*t^2 * p2 + t^3 * p3
#[inline]
pub fn bezier_cubic(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    let u2 = u * u;
    let t2 = t * t;
    p0 * (u2 * u) + p1 * (3.0 * u2 * t) + p2 * (3.0 * u * t2) + p3 * (t2 * t)
}

// ---------------------------------------------------------------------------
// 3D Bezier curves
// ---------------------------------------------------------------------------

/// Evaluate a quadratic Bezier curve in 3D at parameter `t` in [0, 1].
#[inline]
pub fn bezier_quadratic_3d(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
    let u = 1.0 - t;
    p0 * (u * u) + p1 * (2.0 * u * t) + p2 * (t * t)
}

/// Evaluate a cubic Bezier curve in 3D at parameter `t` in [0, 1].
#[inline]
pub fn bezier_cubic_3d(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let u = 1.0 - t;
    let u2 = u * u;
    let t2 = t * t;
    p0 * (u2 * u) + p1 * (3.0 * u2 * t) + p2 * (3.0 * u * t2) + p3 * (t2 * t)
}

// ---------------------------------------------------------------------------
// De Casteljau subdivision
// ---------------------------------------------------------------------------

/// Evaluate a cubic Bezier at `t` using the de Casteljau algorithm (2D).
///
/// Also returns the subdivision — the two sets of control points for the
/// left `[0, t]` and right `[t, 1]` sub-curves.
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

    // De Boor's algorithm
    let mut d: Vec<Vec3> = (0..=degree)
        .map(|j| control_points[k - degree + j])
        .collect();

    for r in 1..=degree {
        for j in (r..=degree).rev() {
            let i = k - degree + j;
            let denom = knots[i + degree + 1 - r] - knots[i];
            if denom.abs() < 1e-12 {
                continue;
            }
            let alpha = ((t - knots[i]) / denom) as f32;
            d[j] = d[j - 1] * (1.0 - alpha) + d[j] * alpha;
        }
    }

    Some(d[degree])
}

// ---------------------------------------------------------------------------
// Arc-length parameterization
// ---------------------------------------------------------------------------

/// Approximate the arc length of a cubic Bezier curve in 3D.
///
/// Uses `n` linear segments to approximate. Higher `n` = more accurate.
#[inline]
pub fn bezier_cubic_3d_arc_length(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, n: usize) -> f32 {
    let mut length = 0.0f32;
    let mut prev = p0;
    for i in 1..=n {
        let t = i as f32 / n as f32;
        let curr = bezier_cubic_3d(p0, p1, p2, p3, t);
        length += (curr - prev).length();
        prev = curr;
    }
    length
}

/// Re-parameterize a cubic Bezier by arc length.
///
/// Given a normalized distance `s` in [0, 1] (where 0 = start, 1 = end),
/// returns the corresponding `t` parameter via binary search.
/// `n` controls the accuracy of the arc-length estimation.
pub fn bezier_cubic_3d_param_at_length(
    p0: Vec3,
    p1: Vec3,
    p2: Vec3,
    p3: Vec3,
    s: f32,
    n: usize,
) -> f32 {
    if s <= 0.0 {
        return 0.0;
    }
    if s >= 1.0 {
        return 1.0;
    }
    let total = bezier_cubic_3d_arc_length(p0, p1, p2, p3, n);
    let target = s * total;

    // Binary search for the t that gives the target arc length
    let mut lo = 0.0f32;
    let mut hi = 1.0f32;
    for _ in 0..32 {
        let mid = (lo + hi) * 0.5;
        let len = bezier_cubic_3d_arc_length(p0, p1, p2, p3, (mid * n as f32) as usize);
        if len < target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) * 0.5
}

// ---------------------------------------------------------------------------
// Gauss-Legendre quadrature
// ---------------------------------------------------------------------------

/// Numerical integration using 5-point Gauss-Legendre quadrature.
///
/// More accurate than Simpson's for smooth functions with fewer evaluations.
/// Integrates `f` over `[a, b]`.
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
    let mut sum = 0.0;
    for i in 0..5 {
        sum += WEIGHTS[i] * f(mid + half * NODES[i]);
    }
    sum * half
}

/// Composite Gauss-Legendre quadrature (5-point) over `n` sub-intervals.
///
/// Divides `[a, b]` into `n` panels and applies 5-point GL to each.
#[inline]
pub fn integral_gauss_legendre(f: impl Fn(f64) -> f64, a: f64, b: f64, n: usize) -> f64 {
    assert!(n > 0, "n must be positive");
    let h = (b - a) / n as f64;
    let mut total = 0.0;
    for i in 0..n {
        let lo = a + i as f64 * h;
        let hi = lo + h;
        total += integral_gauss_legendre_5(&f, lo, hi);
    }
    total
}

// ---------------------------------------------------------------------------
// Easing functions
// ---------------------------------------------------------------------------

/// Ease-in (quadratic): slow start, fast end.
#[inline]
pub fn ease_in(t: f32) -> f32 {
    t * t
}

/// Ease-out (quadratic): fast start, slow end.
#[inline]
pub fn ease_out(t: f32) -> f32 {
    t * (2.0 - t)
}

/// Ease-in-out (cubic smoothstep): slow start, fast middle, slow end.
#[inline]
pub fn ease_in_out(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Ease-in (cubic): slower start than quadratic.
#[inline]
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Ease-out (cubic): slower end than quadratic.
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    let u = 1.0 - t;
    1.0 - u * u * u
}

/// Ease-in-out (quintic smootherstep): C2 continuous, zero first and second derivatives at endpoints.
#[inline]
pub fn ease_in_out_smooth(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GanitError;

    const EPSILON_F64: f64 = 1e-6;
    const EPSILON_F32: f32 = 1e-4;

    fn approx_eq_f64(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON_F64
    }

    fn approx_eq_f32(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON_F32
    }

    #[test]
    fn derivative_of_x_squared() {
        let d = derivative(|x| x * x, 3.0, 1e-7);
        assert!(approx_eq_f64(d, 6.0));
    }

    #[test]
    fn derivative_of_sin() {
        let d = derivative(f64::sin, 0.0, 1e-7);
        assert!(approx_eq_f64(d, 1.0));
    }

    #[test]
    fn derivative_of_exp() {
        let d = derivative(f64::exp, 1.0, 1e-7);
        assert!((d - std::f64::consts::E).abs() < 1e-5);
    }

    #[test]
    fn integral_trapezoidal_constant() {
        let result = integral_trapezoidal(|_| 5.0, 0.0, 2.0, 100);
        assert!(approx_eq_f64(result, 10.0));
    }

    #[test]
    fn integral_trapezoidal_linear() {
        let result = integral_trapezoidal(|x| x, 0.0, 4.0, 1000);
        assert!((result - 8.0).abs() < 1e-4);
    }

    #[test]
    fn integral_trapezoidal_quadratic() {
        let result = integral_trapezoidal(|x| x * x, 0.0, 3.0, 10000);
        assert!((result - 9.0).abs() < 1e-3);
    }

    #[test]
    fn integral_simpson_quadratic() {
        let result = integral_simpson(|x| x * x, 0.0, 3.0, 4);
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn integral_simpson_cubic() {
        let result = integral_simpson(|x| x * x * x, 0.0, 2.0, 4);
        assert!(approx_eq_f64(result, 4.0));
    }

    #[test]
    fn integral_simpson_sin() {
        let result = integral_simpson(f64::sin, 0.0, std::f64::consts::PI, 100);
        assert!((result - 2.0).abs() < 1e-6);
    }

    #[test]
    fn lerp_endpoints() {
        assert!(approx_eq_f64(lerp(0.0, 10.0, 0.0), 0.0));
        assert!(approx_eq_f64(lerp(0.0, 10.0, 1.0), 10.0));
        assert!(approx_eq_f64(lerp(0.0, 10.0, 0.5), 5.0));
    }

    #[test]
    fn bezier_quadratic_endpoints() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 1.0);
        let p2 = Vec2::new(1.0, 0.0);

        let start = bezier_quadratic(p0, p1, p2, 0.0);
        let end = bezier_quadratic(p0, p1, p2, 1.0);

        assert!(approx_eq_f32(start.x, 0.0) && approx_eq_f32(start.y, 0.0));
        assert!(approx_eq_f32(end.x, 1.0) && approx_eq_f32(end.y, 0.0));
    }

    #[test]
    fn bezier_cubic_endpoints() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.25, 1.0);
        let p2 = Vec2::new(0.75, 1.0);
        let p3 = Vec2::new(1.0, 0.0);

        let start = bezier_cubic(p0, p1, p2, p3, 0.0);
        let end = bezier_cubic(p0, p1, p2, p3, 1.0);

        assert!(approx_eq_f32(start.x, 0.0) && approx_eq_f32(start.y, 0.0));
        assert!(approx_eq_f32(end.x, 1.0) && approx_eq_f32(end.y, 0.0));
    }

    #[test]
    fn bezier_quadratic_midpoint() {
        let mid = bezier_quadratic(Vec2::ZERO, Vec2::new(0.5, 0.5), Vec2::ONE, 0.5);
        assert!(approx_eq_f32(mid.x, 0.5));
        assert!(approx_eq_f32(mid.y, 0.5));
    }

    #[test]
    fn integral_simpson_odd_n_rounds_up() {
        let result = integral_simpson(|x| x * x, 0.0, 3.0, 3);
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn derivative_of_constant() {
        let d = derivative(|_| 5.0, 3.0, 1e-7);
        assert!(approx_eq_f64(d, 0.0));
    }

    #[test]
    fn derivative_of_cubic() {
        let d = derivative(|x| x * x * x, 2.0, 1e-7);
        assert!((d - 12.0).abs() < 1e-4);
    }

    #[test]
    fn derivative_of_cos() {
        let d = derivative(f64::cos, std::f64::consts::FRAC_PI_2, 1e-7);
        assert!((d - (-1.0)).abs() < 1e-5);
    }

    #[test]
    fn integral_trapezoidal_sin() {
        let result = integral_trapezoidal(f64::sin, 0.0, std::f64::consts::PI, 10000);
        assert!((result - 2.0).abs() < 1e-4);
    }

    #[test]
    fn integral_simpson_constant() {
        let result = integral_simpson(|_| 7.0, 1.0, 4.0, 4);
        assert!(approx_eq_f64(result, 21.0));
    }

    #[test]
    fn integral_simpson_linear() {
        let result = integral_simpson(|x| 2.0 * x, 0.0, 3.0, 2);
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn lerp_at_quarter() {
        assert!(approx_eq_f64(lerp(0.0, 100.0, 0.25), 25.0));
        assert!(approx_eq_f64(lerp(0.0, 100.0, 0.75), 75.0));
    }

    #[test]
    fn lerp_negative_range() {
        assert!(approx_eq_f64(lerp(-10.0, -20.0, 0.5), -15.0));
    }

    #[test]
    fn lerp_extrapolation() {
        assert!(approx_eq_f64(lerp(0.0, 10.0, 2.0), 20.0));
        assert!(approx_eq_f64(lerp(0.0, 10.0, -1.0), -10.0));
    }

    #[test]
    fn bezier_quadratic_straight_line() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 0.5);
        let p2 = Vec2::ONE;
        let quarter = bezier_quadratic(p0, p1, p2, 0.25);
        assert!(approx_eq_f32(quarter.x, 0.25));
        assert!(approx_eq_f32(quarter.y, 0.25));
    }

    #[test]
    fn bezier_cubic_midpoint() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.0, 1.0);
        let p2 = Vec2::new(1.0, 0.0);
        let p3 = Vec2::ONE;
        let mid = bezier_cubic(p0, p1, p2, p3, 0.5);
        assert!(approx_eq_f32(mid.x, 0.5));
        assert!(approx_eq_f32(mid.y, 0.5));
    }

    #[test]
    fn bezier_cubic_straight_line() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(1.0, 1.0);
        let p2 = Vec2::new(2.0, 2.0);
        let p3 = Vec2::new(3.0, 3.0);
        let mid = bezier_cubic(p0, p1, p2, p3, 0.5);
        assert!(approx_eq_f32(mid.x, 1.5));
        assert!(approx_eq_f32(mid.y, 1.5));
    }

    #[test]
    fn integral_trapezoidal_single_step() {
        let result = integral_trapezoidal(|x| x, 0.0, 2.0, 1);
        assert!(approx_eq_f64(result, 2.0));
    }

    #[test]
    fn integral_simpson_exp() {
        let expected = std::f64::consts::E - 1.0;
        let result = integral_simpson(f64::exp, 0.0, 1.0, 100);
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn calc_error_display() {
        assert_eq!(
            GanitError::InvalidInterval.to_string(),
            "invalid interval: a must be less than b"
        );
        assert_eq!(
            GanitError::ZeroSteps.to_string(),
            "step count must be positive"
        );
    }

    // --- V0.3 tests ---

    fn vec3_approx_eq(a: Vec3, b: Vec3) -> bool {
        approx_eq_f32(a.x, b.x) && approx_eq_f32(a.y, b.y) && approx_eq_f32(a.z, b.z)
    }

    // 3D Bezier
    #[test]
    fn bezier_quadratic_3d_endpoints() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(0.5, 1.0, 0.5);
        let p2 = Vec3::ONE;
        assert!(vec3_approx_eq(bezier_quadratic_3d(p0, p1, p2, 0.0), p0));
        assert!(vec3_approx_eq(bezier_quadratic_3d(p0, p1, p2, 1.0), p2));
    }

    #[test]
    fn bezier_cubic_3d_endpoints() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(1.0, 2.0, 0.0);
        let p2 = Vec3::new(2.0, 2.0, 1.0);
        let p3 = Vec3::new(3.0, 0.0, 1.0);
        assert!(vec3_approx_eq(bezier_cubic_3d(p0, p1, p2, p3, 0.0), p0));
        assert!(vec3_approx_eq(bezier_cubic_3d(p0, p1, p2, p3, 1.0), p3));
    }

    #[test]
    fn bezier_cubic_3d_straight_line() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::ONE;
        let p2 = Vec3::splat(2.0);
        let p3 = Vec3::splat(3.0);
        let mid = bezier_cubic_3d(p0, p1, p2, p3, 0.5);
        assert!(vec3_approx_eq(mid, Vec3::splat(1.5)));
    }

    // De Casteljau
    #[test]
    fn de_casteljau_matches_direct() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.25, 1.0);
        let p2 = Vec2::new(0.75, 1.0);
        let p3 = Vec2::ONE;
        let (pt, _left, _right) = de_casteljau_split(p0, p1, p2, p3, 0.5);
        let direct = bezier_cubic(p0, p1, p2, p3, 0.5);
        assert!(approx_eq_f32(pt.x, direct.x));
        assert!(approx_eq_f32(pt.y, direct.y));
    }

    #[test]
    fn de_casteljau_endpoints() {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(1.0, 2.0);
        let p2 = Vec2::new(3.0, 2.0);
        let p3 = Vec2::new(4.0, 0.0);
        let (pt0, _, _) = de_casteljau_split(p0, p1, p2, p3, 0.0);
        let (pt1, _, _) = de_casteljau_split(p0, p1, p2, p3, 1.0);
        assert!(approx_eq_f32(pt0.x, p0.x) && approx_eq_f32(pt0.y, p0.y));
        assert!(approx_eq_f32(pt1.x, p3.x) && approx_eq_f32(pt1.y, p3.y));
    }

    #[test]
    fn de_casteljau_left_right_rejoin() {
        // Evaluating the left sub-curve at t=1 and right at t=0 should give the split point
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(1.0, 2.0);
        let p2 = Vec2::new(3.0, 2.0);
        let p3 = Vec2::new(4.0, 0.0);
        let (split_pt, left, right) = de_casteljau_split(p0, p1, p2, p3, 0.3);
        let left_end = bezier_cubic(left[0], left[1], left[2], left[3], 1.0);
        let right_start = bezier_cubic(right[0], right[1], right[2], right[3], 0.0);
        assert!(approx_eq_f32(left_end.x, split_pt.x));
        assert!(approx_eq_f32(right_start.x, split_pt.x));
    }

    // Catmull-Rom
    #[test]
    fn catmull_rom_passes_through_endpoints() {
        let p0 = Vec3::new(-1.0, 0.0, 0.0);
        let p1 = Vec3::ZERO;
        let p2 = Vec3::new(1.0, 1.0, 0.0);
        let p3 = Vec3::new(2.0, 1.0, 0.0);
        let at_0 = catmull_rom(p0, p1, p2, p3, 0.0);
        let at_1 = catmull_rom(p0, p1, p2, p3, 1.0);
        assert!(vec3_approx_eq(at_0, p1));
        assert!(vec3_approx_eq(at_1, p2));
    }

    #[test]
    fn catmull_rom_straight_line() {
        // Equally spaced collinear points -> straight line
        let p0 = Vec3::new(0.0, 0.0, 0.0);
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(2.0, 0.0, 0.0);
        let p3 = Vec3::new(3.0, 0.0, 0.0);
        let mid = catmull_rom(p0, p1, p2, p3, 0.5);
        assert!(vec3_approx_eq(mid, Vec3::new(1.5, 0.0, 0.0)));
    }

    // B-spline
    #[test]
    fn bspline_linear_interpolation() {
        // Degree 1 (linear), 2 control points
        let pts = [Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0)];
        let knots = [0.0, 0.0, 1.0, 1.0]; // degree+1 repeated at each end
        let mid = bspline_eval(1, &pts, &knots, 0.5).unwrap();
        assert!(vec3_approx_eq(mid, Vec3::new(5.0, 0.0, 0.0)));
    }

    #[test]
    fn bspline_cubic_endpoints() {
        // Degree 3, 4 control points, clamped knot vector
        let pts = [
            Vec3::ZERO,
            Vec3::new(1.0, 2.0, 0.0),
            Vec3::new(3.0, 2.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ];
        let knots = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        let start = bspline_eval(3, &pts, &knots, 0.0).unwrap();
        let end = bspline_eval(3, &pts, &knots, 1.0).unwrap();
        assert!(vec3_approx_eq(start, pts[0]));
        assert!(vec3_approx_eq(end, pts[3]));
    }

    #[test]
    fn bspline_invalid_knots() {
        let pts = [Vec3::ZERO, Vec3::X];
        let bad_knots = [0.0, 1.0]; // Wrong length
        assert!(bspline_eval(1, &pts, &bad_knots, 0.5).is_none());
    }

    // Arc length
    #[test]
    fn bezier_arc_length_straight_line() {
        // Straight line from origin to (10,0,0) -> length = 10
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(10.0 / 3.0, 0.0, 0.0);
        let p2 = Vec3::new(20.0 / 3.0, 0.0, 0.0);
        let p3 = Vec3::new(10.0, 0.0, 0.0);
        let len = bezier_cubic_3d_arc_length(p0, p1, p2, p3, 100);
        assert!((len - 10.0).abs() < 0.01);
    }

    // Gauss-Legendre
    #[test]
    fn gauss_legendre_5_constant() {
        let result = integral_gauss_legendre_5(|_| 3.0, 0.0, 5.0);
        assert!(approx_eq_f64(result, 15.0));
    }

    #[test]
    fn gauss_legendre_5_quadratic() {
        // GL5 is exact for polynomials up to degree 9
        let result = integral_gauss_legendre_5(|x| x * x, 0.0, 3.0);
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn gauss_legendre_composite_sin() {
        let result = integral_gauss_legendre(f64::sin, 0.0, std::f64::consts::PI, 10);
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn gauss_legendre_vs_simpson() {
        // GL with 2 panels should beat Simpson with 100 panels for smooth functions
        let gl = integral_gauss_legendre(f64::exp, 0.0, 1.0, 2);
        let simp = integral_simpson(f64::exp, 0.0, 1.0, 100);
        let exact = std::f64::consts::E - 1.0;
        assert!((gl - exact).abs() <= (simp - exact).abs());
    }

    // Easing functions
    #[test]
    fn ease_in_endpoints() {
        assert!(approx_eq_f32(ease_in(0.0), 0.0));
        assert!(approx_eq_f32(ease_in(1.0), 1.0));
    }

    #[test]
    fn ease_out_endpoints() {
        assert!(approx_eq_f32(ease_out(0.0), 0.0));
        assert!(approx_eq_f32(ease_out(1.0), 1.0));
    }

    #[test]
    fn ease_in_out_endpoints_and_midpoint() {
        assert!(approx_eq_f32(ease_in_out(0.0), 0.0));
        assert!(approx_eq_f32(ease_in_out(1.0), 1.0));
        assert!(approx_eq_f32(ease_in_out(0.5), 0.5));
    }

    #[test]
    fn ease_in_cubic_slower_than_quadratic() {
        // At t=0.5, cubic ease should be slower (lower value) than quadratic
        assert!(ease_in_cubic(0.5) < ease_in(0.5));
    }

    #[test]
    fn ease_out_cubic_endpoints() {
        assert!(approx_eq_f32(ease_out_cubic(0.0), 0.0));
        assert!(approx_eq_f32(ease_out_cubic(1.0), 1.0));
    }

    #[test]
    fn ease_in_out_smooth_endpoints() {
        assert!(approx_eq_f32(ease_in_out_smooth(0.0), 0.0));
        assert!(approx_eq_f32(ease_in_out_smooth(1.0), 1.0));
        assert!(approx_eq_f32(ease_in_out_smooth(0.5), 0.5));
    }

    #[test]
    fn ease_in_monotonic() {
        let mut prev = 0.0f32;
        for i in 1..=10 {
            let t = i as f32 / 10.0;
            let v = ease_in(t);
            assert!(v >= prev);
            prev = v;
        }
    }

    #[test]
    fn ease_out_monotonic() {
        let mut prev = 0.0f32;
        for i in 1..=10 {
            let t = i as f32 / 10.0;
            let v = ease_out(t);
            assert!(v >= prev);
            prev = v;
        }
    }
}
