// ---------------------------------------------------------------------------

/// Ease-in (quadratic): slow start, fast end.
#[must_use]
#[inline]
pub fn ease_in(t: f32) -> f32 {
    t * t
}

/// Ease-out (quadratic): fast start, slow end.
#[must_use]
#[inline]
pub fn ease_out(t: f32) -> f32 {
    t * (2.0 - t)
}

/// Ease-in-out (cubic smoothstep): slow start, fast middle, slow end.
#[must_use]
#[inline]
pub fn ease_in_out(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

/// Ease-in (cubic): slower start than quadratic.
#[must_use]
#[inline]
pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
}

/// Ease-out (cubic): slower end than quadratic.
#[must_use]
#[inline]
pub fn ease_out_cubic(t: f32) -> f32 {
    let u = 1.0 - t;
    1.0 - u * u * u
}

/// Ease-in-out (quintic smootherstep): C2 continuous, zero first and second derivatives at endpoints.
#[must_use]
#[inline]
pub fn ease_in_out_smooth(t: f32) -> f32 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

// ---------------------------------------------------------------------------
// Noise functions

// ---------------------------------------------------------------------------

/// Critically damped spring step (analytical solution).
///
/// Moves `current` toward `target` with smooth deceleration.
/// Returns `(new_position, new_velocity)`.
///
/// - `current`: current value.
/// - `target`: target value.
/// - `velocity`: current velocity (modified each step).
/// - `stiffness`: spring stiffness (ω² where ω is natural frequency). Typical: 100–500.
/// - `damping`: damping ratio. Use `2.0 * stiffness.sqrt()` for critical damping.
/// - `dt`: time step.
#[must_use]
#[inline]
pub fn spring_step(
    current: f64,
    target: f64,
    velocity: f64,
    stiffness: f64,
    damping: f64,
    dt: f64,
) -> (f64, f64) {
    let omega = stiffness.sqrt();
    let zeta = damping / (2.0 * omega);
    let x = current - target;

    if (zeta - 1.0).abs() < 1e-6 {
        // Critically damped: (c1 + c2*t) * e^(-ω*t)
        let exp = (-omega * dt).exp();
        let c1 = x;
        let c2 = velocity + omega * x;
        let new_x = (c1 + c2 * dt) * exp;
        let new_v = (c2 - omega * (c1 + c2 * dt)) * exp;
        (target + new_x, new_v)
    } else if zeta < 1.0 {
        // Underdamped
        let omega_d = omega * (1.0 - zeta * zeta).sqrt();
        let exp = (-zeta * omega * dt).exp();
        let cos = (omega_d * dt).cos();
        let sin = (omega_d * dt).sin();
        let new_x = exp * (x * cos + ((velocity + zeta * omega * x) / omega_d) * sin);
        let new_v = exp * ((velocity + zeta * omega * x) * cos - x * omega_d * sin)
            - zeta * omega * exp * (x * cos + ((velocity + zeta * omega * x) / omega_d) * sin);
        (target + new_x, new_v)
    } else {
        // Overdamped
        let s1 = -omega * (zeta - (zeta * zeta - 1.0).sqrt());
        let s2 = -omega * (zeta + (zeta * zeta - 1.0).sqrt());
        let c2 = (velocity - s1 * x) / (s2 - s1);
        let c1 = x - c2;
        let e1 = (s1 * dt).exp();
        let e2 = (s2 * dt).exp();
        let new_x = c1 * e1 + c2 * e2;
        let new_v = c1 * s1 * e1 + c2 * s2 * e2;
        (target + new_x, new_v)
    }
}

// ---------------------------------------------------------------------------
// CSS cubic-bezier easing
// ---------------------------------------------------------------------------

/// CSS `cubic-bezier(x1, y1, x2, y2)` timing function.
///
/// Given control points `(0,0)-(x1,y1)-(x2,y2)-(1,1)`, returns the eased
/// value `y` for input progress `t` in [0, 1].
///
/// Uses Newton-Raphson to solve the x(t) curve, then evaluates y(t).
#[must_use]
#[inline]
pub fn cubic_bezier_ease(x1: f32, y1: f32, x2: f32, y2: f32, t: f32) -> f32 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    // Find parameter u such that bezier_x(u) = t using Newton-Raphson
    let bezier_x = |u: f32| -> f32 {
        let iu = 1.0 - u;
        3.0 * iu * iu * u * x1 + 3.0 * iu * u * u * x2 + u * u * u
    };
    let bezier_dx = |u: f32| -> f32 {
        let iu = 1.0 - u;
        3.0 * iu * iu * x1 + 6.0 * iu * u * (x2 - x1) + 3.0 * u * u * (1.0 - x2)
    };

    let mut u = t; // Initial guess
    for _ in 0..8 {
        let dx = bezier_dx(u);
        if dx.abs() < 1e-7 {
            break;
        }
        u -= (bezier_x(u) - t) / dx;
        u = u.clamp(0.0, 1.0);
    }

    // Evaluate y at the found parameter
    let iu = 1.0 - u;
    3.0 * iu * iu * u * y1 + 3.0 * iu * u * u * y2 + u * u * u
}

// ---------------------------------------------------------------------------
// Advanced integration
