use crate::HisabError;
use glam::{Vec2, Vec3};

mod core;
pub use core::*;

mod splines;
pub use splines::*;
mod easing;
pub use easing::*;
mod noise;
pub use noise::*;
mod integration;
pub use integration::*;
mod multivar;
pub use multivar::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HisabError;

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
        let result = integral_trapezoidal(|_| 5.0, 0.0, 2.0, 100).unwrap();
        assert!(approx_eq_f64(result, 10.0));
    }

    #[test]
    fn integral_trapezoidal_linear() {
        let result = integral_trapezoidal(|x| x, 0.0, 4.0, 1000).unwrap();
        assert!((result - 8.0).abs() < 1e-4);
    }

    #[test]
    fn integral_trapezoidal_quadratic() {
        let result = integral_trapezoidal(|x| x * x, 0.0, 3.0, 10000).unwrap();
        assert!((result - 9.0).abs() < 1e-3);
    }

    #[test]
    fn integral_simpson_quadratic() {
        let result = integral_simpson(|x| x * x, 0.0, 3.0, 4).unwrap();
        assert!(approx_eq_f64(result, 9.0));
    }

    #[test]
    fn integral_simpson_cubic() {
        let result = integral_simpson(|x| x * x * x, 0.0, 2.0, 4).unwrap();
        assert!(approx_eq_f64(result, 4.0));
    }

    #[test]
    fn integral_simpson_sin() {
        let result = integral_simpson(f64::sin, 0.0, std::f64::consts::PI, 100).unwrap();
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
        let result = integral_simpson(|x| x * x, 0.0, 3.0, 3).unwrap();
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
        let result = integral_trapezoidal(f64::sin, 0.0, std::f64::consts::PI, 10000).unwrap();
        assert!((result - 2.0).abs() < 1e-4);
    }

    #[test]
    fn integral_simpson_constant() {
        let result = integral_simpson(|_| 7.0, 1.0, 4.0, 4).unwrap();
        assert!(approx_eq_f64(result, 21.0));
    }

    #[test]
    fn integral_simpson_linear() {
        let result = integral_simpson(|x| 2.0 * x, 0.0, 3.0, 2).unwrap();
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
        let result = integral_trapezoidal(|x| x, 0.0, 2.0, 1).unwrap();
        assert!(approx_eq_f64(result, 2.0));
    }

    #[test]
    fn integral_simpson_exp() {
        let expected = std::f64::consts::E - 1.0;
        let result = integral_simpson(f64::exp, 0.0, 1.0, 100).unwrap();
        assert!((result - expected).abs() < 1e-6);
    }

    #[test]
    fn calc_error_display() {
        assert_eq!(
            HisabError::InvalidInterval.to_string(),
            "invalid interval: a must be less than b"
        );
        assert_eq!(
            HisabError::ZeroSteps.to_string(),
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
        let len = bezier_cubic_3d_arc_length(p0, p1, p2, p3, 100).unwrap();
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
        let result = integral_gauss_legendre(f64::sin, 0.0, std::f64::consts::PI, 10).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn gauss_legendre_vs_simpson() {
        // GL with 2 panels should beat Simpson with 100 panels for smooth functions
        let gl = integral_gauss_legendre(f64::exp, 0.0, 1.0, 2).unwrap();
        let simp = integral_simpson(f64::exp, 0.0, 1.0, 100).unwrap();
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

    // --- Audit tests ---

    #[test]
    fn param_at_length_endpoints() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(10.0 / 3.0, 0.0, 0.0);
        let p2 = Vec3::new(20.0 / 3.0, 0.0, 0.0);
        let p3 = Vec3::new(10.0, 0.0, 0.0);
        assert!(approx_eq_f32(
            bezier_cubic_3d_param_at_length(p0, p1, p2, p3, 0.0, 100).unwrap(),
            0.0
        ));
        assert!(approx_eq_f32(
            bezier_cubic_3d_param_at_length(p0, p1, p2, p3, 1.0, 100).unwrap(),
            1.0
        ));
    }

    #[test]
    fn param_at_length_midpoint_straight_line() {
        // Straight line: s=0.5 should give t≈0.5
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(10.0 / 3.0, 0.0, 0.0);
        let p2 = Vec3::new(20.0 / 3.0, 0.0, 0.0);
        let p3 = Vec3::new(10.0, 0.0, 0.0);
        let t = bezier_cubic_3d_param_at_length(p0, p1, p2, p3, 0.5, 100).unwrap();
        assert!((t - 0.5).abs() < 0.02);
    }

    #[test]
    fn gauss_legendre_5_high_degree_poly() {
        // GL5 is exact for degree <= 9: test x^8
        // ∫₀¹ x^8 dx = 1/9
        let result = integral_gauss_legendre_5(|x| x.powi(8), 0.0, 1.0);
        assert!((result - 1.0 / 9.0).abs() < 1e-10);
    }

    #[test]
    fn bspline_quadratic() {
        // Degree 2, 3 control points
        let pts = [
            Vec3::ZERO,
            Vec3::new(1.0, 2.0, 0.0),
            Vec3::new(2.0, 0.0, 0.0),
        ];
        let knots = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0];
        let start = bspline_eval(2, &pts, &knots, 0.0).unwrap();
        let end = bspline_eval(2, &pts, &knots, 1.0).unwrap();
        assert!(vec3_approx_eq(start, pts[0]));
        assert!(vec3_approx_eq(end, pts[2]));
    }

    #[test]
    fn catmull_rom_quarter() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(1.0, 0.0, 0.0);
        let p2 = Vec3::new(2.0, 0.0, 0.0);
        let p3 = Vec3::new(3.0, 0.0, 0.0);
        let q = catmull_rom(p0, p1, p2, p3, 0.25);
        assert!(approx_eq_f32(q.x, 1.25));
    }

    #[test]
    fn bezier_quadratic_3d_midpoint() {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(0.5, 0.5, 0.5);
        let p2 = Vec3::ONE;
        let mid = bezier_quadratic_3d(p0, p1, p2, 0.5);
        assert!(vec3_approx_eq(mid, Vec3::splat(0.5)));
    }

    #[test]
    fn ease_in_out_smooth_c2_symmetry() {
        // Smootherstep is symmetric: f(t) + f(1-t) = 1
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            let sum = ease_in_out_smooth(t) + ease_in_out_smooth(1.0 - t);
            assert!(approx_eq_f32(sum, 1.0));
        }
    }

    // --- Adaptive Simpson tests ---

    #[test]
    fn adaptive_simpson_quadratic() {
        // ∫₀¹ x² dx = 1/3
        let result = integral_adaptive_simpson(|x| x * x, 0.0, 1.0, 1e-10).unwrap();
        assert!((result - 1.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn adaptive_simpson_sin() {
        // ∫₀^π sin(x) dx = 2
        let result = integral_adaptive_simpson(f64::sin, 0.0, std::f64::consts::PI, 1e-10).unwrap();
        assert!((result - 2.0).abs() < 1e-10);
    }

    #[test]
    fn adaptive_simpson_high_accuracy() {
        // ∫₀¹ e^x dx = e - 1
        let result = integral_adaptive_simpson(f64::exp, 0.0, 1.0, 1e-12).unwrap();
        assert!((result - (std::f64::consts::E - 1.0)).abs() < 1e-11);
    }

    #[test]
    fn adaptive_simpson_invalid_interval() {
        assert!(integral_adaptive_simpson(|x| x, 1.0, 0.0, 1e-6).is_err());
    }

    // --- Monte Carlo integration tests ---

    #[test]
    fn monte_carlo_constant() {
        // ∫₀¹ 1 dx = 1
        let (est, _err) = integral_monte_carlo(|_| 1.0, &[(0.0, 1.0)], 10000).unwrap();
        assert!((est - 1.0).abs() < 0.05);
    }

    #[test]
    fn monte_carlo_linear() {
        // ∫₀¹ x dx = 0.5
        let (est, _err) = integral_monte_carlo(|x| x[0], &[(0.0, 1.0)], 50000).unwrap();
        assert!((est - 0.5).abs() < 0.05);
    }

    #[test]
    fn monte_carlo_2d() {
        // ∫₀¹ ∫₀¹ (x+y) dx dy = 1
        let (est, _err) =
            integral_monte_carlo(|x| x[0] + x[1], &[(0.0, 1.0), (0.0, 1.0)], 50000).unwrap();
        assert!((est - 1.0).abs() < 0.1);
    }

    #[test]
    fn monte_carlo_empty_bounds() {
        assert!(integral_monte_carlo(|_| 1.0, &[], 100).is_err());
    }

    #[test]
    fn monte_carlo_zero_samples() {
        assert!(integral_monte_carlo(|_| 1.0, &[(0.0, 1.0)], 0).is_err());
    }

    #[test]
    fn monte_carlo_invalid_bounds() {
        assert!(integral_monte_carlo(|_| 1.0, &[(1.0, 0.0)], 100).is_err());
    }

    // --- Multivariable calculus tests ---

    #[test]
    fn partial_derivative_linear() {
        let f = |x: &[f64]| 3.0 * x[0] + 2.0 * x[1];
        let x = [1.0, 1.0];
        let dfx = partial_derivative(f, &x, 0, 1e-7).unwrap();
        let dfy = partial_derivative(f, &x, 1, 1e-7).unwrap();
        assert!(approx_eq_f64(dfx, 3.0));
        assert!(approx_eq_f64(dfy, 2.0));
    }

    #[test]
    fn partial_derivative_quadratic() {
        let f = |x: &[f64]| x[0] * x[0] * x[1];
        let x = [3.0, 2.0];
        let dfx = partial_derivative(f, &x, 0, 1e-5).unwrap();
        let dfy = partial_derivative(f, &x, 1, 1e-5).unwrap();
        assert!((dfx - 12.0).abs() < 1e-4);
        assert!((dfy - 9.0).abs() < 1e-4);
    }

    #[test]
    fn partial_derivative_out_of_range() {
        let f = |x: &[f64]| x[0];
        assert!(partial_derivative(f, &[1.0], 5, 1e-7).is_err());
    }

    #[test]
    fn gradient_quadratic() {
        let f = |x: &[f64]| x[0] * x[0] + 2.0 * x[1] * x[1] + 3.0 * x[2] * x[2];
        let x = [1.0, 2.0, 3.0];
        let g = gradient(f, &x, 1e-5).unwrap();
        assert!((g[0] - 2.0).abs() < 1e-4);
        assert!((g[1] - 8.0).abs() < 1e-4);
        assert!((g[2] - 18.0).abs() < 1e-4);
    }

    #[test]
    fn gradient_empty_errors() {
        let f = |_: &[f64]| 0.0;
        assert!(gradient(f, &[], 1e-7).is_err());
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn jacobian_linear_map() {
        let f1: &dyn Fn(&[f64]) -> f64 = &|x: &[f64]| 2.0 * x[0] + x[1];
        let f2: &dyn Fn(&[f64]) -> f64 = &|x: &[f64]| x[0] - 3.0 * x[1];
        let fs: Vec<&dyn Fn(&[f64]) -> f64> = vec![f1, f2];
        let x = [1.0, 1.0];
        let j = jacobian(&fs, &x, 1e-7).unwrap();
        assert!((j[0][0] - 2.0).abs() < 1e-4);
        assert!((j[0][1] - 1.0).abs() < 1e-4);
        assert!((j[1][0] - 1.0).abs() < 1e-4);
        assert!((j[1][1] - (-3.0)).abs() < 1e-4);
    }

    #[test]
    #[allow(clippy::type_complexity)]
    fn jacobian_empty_errors() {
        let fs: Vec<&dyn Fn(&[f64]) -> f64> = vec![];
        assert!(jacobian(&fs, &[1.0], 1e-7).is_err());
    }

    #[test]
    fn hessian_quadratic() {
        let f = |x: &[f64]| x[0] * x[0] + 3.0 * x[0] * x[1] + 2.0 * x[1] * x[1];
        let x = [1.0, 1.0];
        let h = hessian(f, &x, 1e-4).unwrap();
        assert!((h[0][0] - 2.0).abs() < 1e-3);
        assert!((h[0][1] - 3.0).abs() < 1e-3);
        assert!((h[1][0] - 3.0).abs() < 1e-3);
        assert!((h[1][1] - 4.0).abs() < 1e-3);
    }

    #[test]
    #[allow(clippy::needless_range_loop)]
    fn hessian_symmetric() {
        let f = |x: &[f64]| x[0] * x[0] * x[1] + x[1] * x[1] * x[2] + x[0] * x[2];
        let x = [2.0, 3.0, 4.0];
        let h = hessian(f, &x, 1e-4).unwrap();
        for i in 0..3 {
            for j in 0..3 {
                assert!(
                    (h[i][j] - h[j][i]).abs() < 1e-3,
                    "Hessian not symmetric at [{i}][{j}]"
                );
            }
        }
    }

    #[test]
    fn hessian_empty_errors() {
        let f = |_: &[f64]| 0.0;
        assert!(hessian(f, &[], 1e-7).is_err());
    }

    // --- Noise tests ---

    #[test]
    fn perlin_2d_deterministic() {
        let a = perlin_2d(1.5, 2.3);
        let b = perlin_2d(1.5, 2.3);
        assert!((a - b).abs() < 1e-15);
    }

    #[test]
    fn perlin_2d_range() {
        for i in 0..100 {
            let x = i as f64 * 0.37;
            let y = i as f64 * 0.53;
            let v = perlin_2d(x, y);
            assert!((-1.5..=1.5).contains(&v), "perlin_2d out of range: {v}");
        }
    }

    #[test]
    fn perlin_3d_deterministic() {
        let a = perlin_3d(0.5, 1.5, 2.5);
        let b = perlin_3d(0.5, 1.5, 2.5);
        assert!((a - b).abs() < 1e-15);
    }

    #[test]
    fn fbm_2d_range() {
        let v = fbm_2d(perlin_2d, 1.0, 2.0, 4, 2.0, 0.5);
        assert!(v > -2.0 && v < 2.0);
    }

    #[test]
    fn fbm_2d_more_octaves_more_detail() {
        // fBm with 1 octave = raw noise; more octaves adds detail (different value)
        let v1 = fbm_2d(perlin_2d, 1.23, 4.56, 1, 2.0, 0.5);
        let v4 = fbm_2d(perlin_2d, 1.23, 4.56, 4, 2.0, 0.5);
        // They should differ (more octaves changes the result)
        assert!((v1 - v4).abs() > 1e-6 || v1.abs() < 1e-10);
    }

    // --- Spring dynamics tests ---

    #[test]
    fn spring_critically_damped_converges() {
        let stiffness: f64 = 100.0;
        let damping = 2.0 * stiffness.sqrt(); // critical damping
        let mut pos = 0.0;
        let mut vel = 0.0;
        let target = 1.0;
        for _ in 0..1000 {
            let (p, v) = spring_step(pos, target, vel, stiffness, damping, 0.01);
            pos = p;
            vel = v;
        }
        assert!((pos - target).abs() < 1e-4, "pos={pos}");
        assert!(vel.abs() < 1e-4, "vel={vel}");
    }

    #[test]
    fn spring_underdamped_oscillates() {
        let stiffness = 100.0;
        let damping = 2.0; // underdamped (zeta < 1)
        let mut pos = 1.0;
        let mut vel = 0.0;
        let target = 0.0;
        let mut crossed_zero = false;
        for _ in 0..500 {
            let (p, v) = spring_step(pos, target, vel, stiffness, damping, 0.01);
            if pos > 0.0 && p < 0.0 {
                crossed_zero = true;
            }
            pos = p;
            vel = v;
        }
        assert!(
            crossed_zero,
            "underdamped spring should oscillate past target"
        );
    }

    // --- Cubic bezier easing tests ---

    #[test]
    fn cubic_bezier_ease_endpoints() {
        // Any curve should map 0→0 and 1→1
        assert!(approx_eq_f32(
            cubic_bezier_ease(0.25, 0.1, 0.25, 1.0, 0.0),
            0.0
        ));
        assert!(approx_eq_f32(
            cubic_bezier_ease(0.25, 0.1, 0.25, 1.0, 1.0),
            1.0
        ));
    }

    #[test]
    fn cubic_bezier_ease_linear() {
        // Linear: (0.0, 0.0, 1.0, 1.0) should give y ≈ t
        for i in 0..=10 {
            let t = i as f32 / 10.0;
            let y = cubic_bezier_ease(0.0, 0.0, 1.0, 1.0, t);
            assert!((y - t).abs() < 0.02, "linear ease at t={t}: got {y}");
        }
    }

    #[test]
    fn cubic_bezier_ease_monotonic() {
        // CSS ease: (0.25, 0.1, 0.25, 1.0) should be monotonically increasing
        let mut prev = 0.0;
        for i in 1..=20 {
            let t = i as f32 / 20.0;
            let y = cubic_bezier_ease(0.25, 0.1, 0.25, 1.0, t);
            assert!(y >= prev - 1e-4, "non-monotonic at t={t}: {prev} → {y}");
            prev = y;
        }
    }
}
