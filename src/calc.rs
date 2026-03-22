//! Calculus: differentiation, integration, interpolation, curves.
//!
//! Provides numerical differentiation, integration (trapezoidal and Simpson's),
//! linear interpolation, and Bezier curve evaluation.

use glam::Vec2;

/// Numerical derivative using the central difference method.
///
/// `f`: the function to differentiate.
/// `x`: the point at which to evaluate the derivative.
/// `h`: the step size (smaller = more accurate but risk of cancellation).
pub fn derivative(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Numerical integration using the trapezoidal rule.
///
/// Divides [a, b] into `n` sub-intervals.
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
pub fn bezier_quadratic(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    p0 * (u * u) + p1 * (2.0 * u * t) + p2 * (t * t)
}

/// Evaluate a cubic Bezier curve at parameter `t` in [0, 1].
///
/// B(t) = (1-t)^3 * p0 + 3(1-t)^2*t * p1 + 3(1-t)*t^2 * p2 + t^3 * p3
pub fn bezier_cubic(p0: Vec2, p1: Vec2, p2: Vec2, p3: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    let u2 = u * u;
    let t2 = t * t;
    p0 * (u2 * u) + p1 * (3.0 * u2 * t) + p2 * (3.0 * u * t2) + p3 * (t2 * t)
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
}
