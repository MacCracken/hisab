//! Calculus: differentiation, integration, interpolation, curves.
//!
//! Provides numerical differentiation, integration (trapezoidal and Simpson's),
//! linear interpolation, and Bezier curve evaluation.

use crate::HisabError;
use glam::{Vec2, Vec3};

/// Numerical derivative using the central difference method.
///
/// `f`: the function to differentiate.
/// `x`: the point at which to evaluate the derivative.
/// `h`: the step size (smaller = more accurate but risk of cancellation).
///
/// # Examples
///
/// ```
/// use hisab::calc::derivative;
///
/// // d/dx(x²) at x=3 ≈ 6
/// let d = derivative(|x| x * x, 3.0, 1e-7);
/// assert!((d - 6.0).abs() < 1e-5);
/// ```
#[must_use]
#[inline]
pub fn derivative(f: impl Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x - h)) / (2.0 * h)
}

/// Numerical integration using the trapezoidal rule.
///
/// Divides [a, b] into `n` sub-intervals.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero.
#[must_use = "returns the computed integral"]
#[inline]
pub fn integral_trapezoidal(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, HisabError> {
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let h = (b - a) / n as f64;
    let mut sum = 0.5 * (f(a) + f(b));
    for i in 1..n {
        sum += f(a + i as f64 * h);
    }
    Ok(sum * h)
}

/// Neumaier-compensated addition helper: adds `v` into `(sum, comp)`.
#[inline(always)]
fn neumaier_add(sum: &mut f64, comp: &mut f64, v: f64) {
    let t = *sum + v;
    if sum.abs() >= v.abs() {
        *comp += (*sum - t) + v;
    } else {
        *comp += (v - t) + *sum;
    }
    *sum = t;
}

/// Numerical integration using Simpson's rule.
///
/// `n` must be even. If odd, it is rounded up.
///
/// Uses Neumaier compensated summation internally to reduce floating-point
/// accumulation error when integrating over many sub-intervals.
///
/// # Errors
///
/// Returns [`HisabError::ZeroSteps`] if `n` is zero (after rounding).
#[must_use = "returns the computed integral"]
#[inline]
pub fn integral_simpson(
    f: impl Fn(f64) -> f64,
    a: f64,
    b: f64,
    n: usize,
) -> Result<f64, HisabError> {
    let n = if n % 2 == 1 { n + 1 } else { n };
    if n == 0 {
        return Err(HisabError::ZeroSteps);
    }
    let h = (b - a) / n as f64;

    // Neumaier-compensated accumulator.
    let mut sum = f(a) + f(b);
    let mut comp = 0.0_f64;

    // Process pairs: odd indices get coefficient 4, even (interior) get 2.
    let mut i = 1;
    while i < n {
        neumaier_add(&mut sum, &mut comp, 4.0 * f(a + i as f64 * h));
        // The last pair adds 2*f(b), but f(b) is already in sum — corrected below.
        neumaier_add(&mut sum, &mut comp, 2.0 * f(a + (i + 1) as f64 * h));
        i += 2;
    }
    // The loop added 2*f(b) for the last even index, which equals f(b).
    // We need only 1*f(b), so subtract the extra copy.
    neumaier_add(&mut sum, &mut comp, -2.0 * f(b));

    Ok((sum + comp) * h / 3.0)
}

/// Linear interpolation between two f64 values.
#[must_use]
#[inline]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

/// Evaluate a quadratic Bezier curve at parameter `t` in [0, 1].
///
/// B(t) = (1-t)^2 * p0 + 2(1-t)t * p1 + t^2 * p2
#[must_use]
#[inline]
pub fn bezier_quadratic(p0: Vec2, p1: Vec2, p2: Vec2, t: f32) -> Vec2 {
    let u = 1.0 - t;
    p0 * (u * u) + p1 * (2.0 * u * t) + p2 * (t * t)
}

/// Evaluate a cubic Bezier curve at parameter `t` in [0, 1].
///
/// B(t) = (1-t)^3 * p0 + 3(1-t)^2*t * p1 + 3(1-t)*t^2 * p2 + t^3 * p3
#[must_use]
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
#[must_use]
#[inline]
pub fn bezier_quadratic_3d(p0: Vec3, p1: Vec3, p2: Vec3, t: f32) -> Vec3 {
    let u = 1.0 - t;
    p0 * (u * u) + p1 * (2.0 * u * t) + p2 * (t * t)
}

/// Evaluate a cubic Bezier curve in 3D at parameter `t` in [0, 1].
#[must_use]
#[inline]
pub fn bezier_cubic_3d(p0: Vec3, p1: Vec3, p2: Vec3, p3: Vec3, t: f32) -> Vec3 {
    let u = 1.0 - t;
    let u2 = u * u;
    let t2 = t * t;
    p0 * (u2 * u) + p1 * (3.0 * u2 * t) + p2 * (3.0 * u * t2) + p3 * (t2 * t)
}

// ---------------------------------------------------------------------------
// De Casteljau subdivision
