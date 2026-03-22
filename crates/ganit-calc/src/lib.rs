//! ganit-calc — Calculus: differentiation, integration, interpolation, curves.
//!
//! Provides numerical differentiation, integration (trapezoidal and Simpson's),
//! linear interpolation, and Bezier curve evaluation.

use glam::Vec2;
use thiserror::Error;

pub use ganit_core;

/// Errors from calculus operations.
#[derive(Error, Debug)]
pub enum CalcError {
    #[error("invalid interval: a must be less than b")]
    InvalidInterval,
    #[error("step count must be positive")]
    ZeroSteps,
}

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

    for i in 1..n {
        let coeff = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += coeff * f(a + i as f64 * h);
    }

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
        // d/dx(x^2) = 2x, at x=3 should be 6
        let d = derivative(|x| x * x, 3.0, 1e-7);
        assert!(approx_eq_f64(d, 6.0));
    }

    #[test]
    fn derivative_of_sin() {
        // d/dx(sin(x)) = cos(x), at x=0 should be 1
        let d = derivative(f64::sin, 0.0, 1e-7);
        assert!(approx_eq_f64(d, 1.0));
    }

    #[test]
    fn derivative_of_exp() {
        // d/dx(e^x) = e^x, at x=1 should be e
        let d = derivative(f64::exp, 1.0, 1e-7);
        assert!((d - std::f64::consts::E).abs() < 1e-5);
    }

    #[test]
    fn integral_trapezoidal_constant() {
        // Integral of f(x) = 5 from 0 to 2 = 10
        let result = integral_trapezoidal(|_| 5.0, 0.0, 2.0, 100);
        assert!(approx_eq_f64(result, 10.0));
    }

    #[test]
    fn integral_trapezoidal_linear() {
        // Integral of f(x) = x from 0 to 4 = 8
        let result = integral_trapezoidal(|x| x, 0.0, 4.0, 1000);
        assert!((result - 8.0).abs() < 1e-4);
    }

    #[test]
    fn integral_trapezoidal_quadratic() {
        // Integral of f(x) = x^2 from 0 to 3 = 9
        let result = integral_trapezoidal(|x| x * x, 0.0, 3.0, 10000);
        assert!((result - 9.0).abs() < 1e-3);
    }

    #[test]
    fn integral_simpson_quadratic() {
        // Simpson's is exact for polynomials up to degree 3.
        // Integral of x^2 from 0 to 3 = 9
        let result = integral_simpson(|x| x * x, 0.0, 3.0, 4);
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn integral_simpson_cubic() {
        // Integral of x^3 from 0 to 2 = 4
        let result = integral_simpson(|x| x * x * x, 0.0, 2.0, 4);
        assert!(approx_eq_f64(result, 4.0));
    }

    #[test]
    fn integral_simpson_sin() {
        // Integral of sin(x) from 0 to pi = 2
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
        // Straight line: p0=(0,0), p1=(0.5,0.5), p2=(1,1) -> midpoint = (0.5, 0.5)
        let mid = bezier_quadratic(Vec2::ZERO, Vec2::new(0.5, 0.5), Vec2::ONE, 0.5);
        assert!(approx_eq_f32(mid.x, 0.5));
        assert!(approx_eq_f32(mid.y, 0.5));
    }

    #[test]
    fn integral_simpson_odd_n_rounds_up() {
        // n=3 should round to n=4, and still give correct result
        let result = integral_simpson(|x| x * x, 0.0, 3.0, 3);
        assert!(approx_eq_f64(result, 9.0));
    }
}
