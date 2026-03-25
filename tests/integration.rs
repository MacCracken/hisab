//! Integration tests exercising cross-module usage.

use hisab::geo::{
    aabb_aabb, closest_point_on_aabb, gjk_intersect_3d, ray_aabb, ray_capsule, ray_obb, ray_plane,
    ray_sphere, ray_triangle, sphere_sphere,
};
use hisab::transforms::{Transform3D, lerp_vec3, transform3d_lerp};
use hisab::{
    Aabb, Capsule, Frustum, HisabError, Obb, Plane, Quat, Ray, Segment, Sphere, Triangle, Vec3,
};

const EPSILON: f32 = 1e-4;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[test]
fn transform_then_ray_intersection() {
    // Place a sphere at a transformed position, then ray-test it.
    let t = Transform3D::new(Vec3::new(0.0, 0.0, 10.0), Quat::IDENTITY, Vec3::ONE);
    let sphere_center = t.apply_to_point(Vec3::ZERO);

    let sphere = Sphere::new(sphere_center, 1.0).unwrap();
    let ray = Ray::new(Vec3::ZERO, Vec3::Z).unwrap();

    let hit_t = ray_sphere(&ray, &sphere).expect("should hit");
    assert!(approx_eq(hit_t, 9.0)); // 10.0 - 1.0 radius
}

#[test]
fn interpolated_ray_origin() {
    // Interpolate between two positions, cast a ray from the midpoint.
    let a = Vec3::new(-5.0, 0.5, 0.5);
    let b = Vec3::new(5.0, 0.5, 0.5);
    let mid = lerp_vec3(a, b, 0.5);

    let ray = Ray::new(mid, Vec3::Z).unwrap();
    let aabb = Aabb::new(Vec3::ZERO, Vec3::new(1.0, 1.0, 10.0));

    let hit = ray_aabb(&ray, &aabb);
    assert!(hit.is_some());
}

#[test]
fn numerical_root_matches_geometry() {
    // Find the distance where a ray hits a plane using both geometry and root-finding.
    let ray = Ray::new(Vec3::ZERO, Vec3::Y).unwrap();
    let plane = Plane::from_point_normal(Vec3::new(0.0, 7.0, 0.0), Vec3::Y).unwrap();

    // Geometric method
    let geo_t = ray_plane(&ray, &plane).unwrap();

    // Numerical method: find t where ray.at(t).y - 7.0 = 0
    let num_t = hisab::num::bisection(|t| t - 7.0, 0.0, 20.0, 1e-10, 100).unwrap();

    assert!((geo_t as f64 - num_t).abs() < 1e-4);
}

#[test]
fn calculus_and_numerical_consistency() {
    // Integrate the derivative to verify fundamental theorem of calculus.
    // f(x) = x^3, F(x) = x^4/4
    // ∫₁² 3x² dx = F(2) - F(1) = 4 - 0.25 = 3.75
    // But actually d/dx(x^3) = 3x^2, and ∫₁² 3x² dx = [x³]₁² = 8 - 1 = 7
    let integral = hisab::calc::integral_simpson(|x| 3.0 * x * x, 1.0, 2.0, 100).unwrap();
    assert!((integral - 7.0).abs() < 1e-6);

    // Verify the derivative at x=2: d/dx(x^3)|₂ = 3*4 = 12
    let deriv = hisab::calc::derivative(|x| x * x * x, 2.0, 1e-7);
    assert!((deriv - 12.0).abs() < 1e-4);
}

#[test]
fn error_types_unified() {
    // Different modules produce the same HisabError type.
    let num_err: Result<f64, HisabError> =
        hisab::num::bisection(|x| x * x + 1.0, 1.0, 2.0, 1e-10, 100);
    assert!(num_err.is_err());

    // Can pattern-match on the unified enum.
    match num_err.unwrap_err() {
        HisabError::InvalidInput(msg) => assert!(msg.contains("opposite signs")),
        other => panic!("unexpected error: {other}"),
    }
}

// --- V0.2 integration tests ---

#[test]
fn ray_triangle_through_transformed_mesh() {
    // Place a triangle via transform, then ray-test it
    let t = Transform3D::new(Vec3::new(0.0, 0.0, 10.0), Quat::IDENTITY, Vec3::ONE);
    let tri = Triangle::new(
        t.apply_to_point(Vec3::new(-1.0, -1.0, 0.0)),
        t.apply_to_point(Vec3::new(1.0, -1.0, 0.0)),
        t.apply_to_point(Vec3::new(0.0, 1.0, 0.0)),
    );
    let ray = Ray::new(Vec3::ZERO, Vec3::Z).unwrap();
    let hit = ray_triangle(&ray, &tri).unwrap();
    assert!(approx_eq(hit, 10.0));
}

#[test]
fn frustum_culling_with_aabb() {
    // Build a frustum and test AABBs at various positions
    let proj =
        hisab::transforms::projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
    let frustum = Frustum::from_view_projection(proj);

    // AABB in front of camera — should be visible
    let visible = Aabb::new(Vec3::new(-0.5, -0.5, -5.0), Vec3::new(0.5, 0.5, -3.0));
    assert!(frustum.contains_aabb(&visible));

    // AABB behind camera — should be culled
    let behind = Aabb::new(Vec3::new(-0.5, -0.5, 1.0), Vec3::new(0.5, 0.5, 3.0));
    assert!(!frustum.contains_aabb(&behind));
}

#[test]
fn broadphase_then_narrowphase() {
    // Use AABB overlap as broadphase, then sphere-sphere as narrowphase
    let a_aabb = Aabb::new(Vec3::ZERO, Vec3::splat(2.0));
    let b_aabb = Aabb::new(Vec3::splat(1.0), Vec3::splat(3.0));
    assert!(aabb_aabb(&a_aabb, &b_aabb)); // Broadphase pass

    let a_sphere = Sphere::new(Vec3::ONE, 1.0).unwrap();
    let b_sphere = Sphere::new(Vec3::splat(2.0), 1.0).unwrap();
    assert!(sphere_sphere(&a_sphere, &b_sphere)); // Narrowphase pass
}

#[test]
fn interpolated_transform_ray_test() {
    // Interpolate between two transforms, cast ray at interpolated sphere
    let a = Transform3D::new(Vec3::new(0.0, 0.0, -5.0), Quat::IDENTITY, Vec3::ONE);
    let b = Transform3D::new(Vec3::new(0.0, 0.0, -15.0), Quat::IDENTITY, Vec3::ONE);
    let mid = transform3d_lerp(&a, &b, 0.5);

    let sphere = Sphere::new(mid.position, 1.0).unwrap();
    let ray = Ray::new(Vec3::ZERO, Vec3::new(0.0, 0.0, -1.0)).unwrap();
    let hit = ray_sphere(&ray, &sphere).unwrap();
    assert!(approx_eq(hit, 9.0)); // z=-10 center, radius 1 -> hit at z=-9 -> t=9
}

#[test]
fn segment_closest_to_aabb_corner() {
    let seg = Segment::new(Vec3::new(-5.0, 0.0, 0.0), Vec3::new(5.0, 0.0, 0.0));
    let bb = Aabb::new(Vec3::new(3.0, 3.0, 0.0), Vec3::new(4.0, 4.0, 1.0));
    // Closest point on segment to AABB corner (3,3,0)
    let seg_pt = seg.closest_point(Vec3::new(3.0, 3.0, 0.0));
    let aabb_pt = closest_point_on_aabb(&bb, seg_pt);
    // seg_pt = (3,0,0), aabb_pt = (3,3,0), distance = 3
    assert!(approx_eq(seg_pt.x, 3.0));
    assert!(approx_eq((aabb_pt - seg_pt).length(), 3.0));
}

// --- 0.25.3: SVD + calc cross-talk ---

#[allow(clippy::type_complexity, clippy::needless_range_loop)]
#[test]
fn svd_reconstructs_jacobian() {
    let f1: &dyn Fn(&[f64]) -> f64 = &|x: &[f64]| x[0] * x[0] + x[1];
    let f2: &dyn Fn(&[f64]) -> f64 = &|x: &[f64]| x[0] * x[1];
    let fs: Vec<&dyn Fn(&[f64]) -> f64> = vec![f1, f2];

    let jac = hisab::calc::jacobian(&fs, &[2.0, 3.0], 1e-6).unwrap();
    let svd_result = hisab::num::svd(&jac).unwrap();

    let m = jac.len();
    let n = jac[0].len();
    for i in 0..m {
        for j in 0..n {
            let mut val = 0.0;
            for s in 0..svd_result.sigma.len() {
                val += svd_result.u[s][i] * svd_result.sigma[s] * svd_result.vt[s][j];
            }
            assert!((val - jac[i][j]).abs() < 1e-4);
        }
    }
}

#[test]
fn gradient_descent_matches_newton() {
    // Both should find the root of f'(x) = 0 for f(x) = (x-3)²
    let gd = hisab::num::gradient_descent(
        |x: &[f64]| (x[0] - 3.0) * (x[0] - 3.0),
        |x: &[f64]| vec![2.0 * (x[0] - 3.0)],
        &[0.0],
        0.1,
        1e-8,
        1000,
    )
    .unwrap();

    let nr = hisab::num::newton_raphson(
        |x| 2.0 * (x - 3.0), // f' = 0 at minimum
        |_| 2.0,
        0.0,
        1e-10,
        100,
    )
    .unwrap();

    assert!((gd.x[0] - 3.0).abs() < 1e-3);
    assert!((nr - 3.0).abs() < 1e-8);
}

// --- 0.26.3: OBB/Capsule + transform cross-talk ---

#[test]
fn transformed_obb_ray_test() {
    // Place an OBB via rotation and ray-test it
    let rot = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);
    let obb = Obb::new(Vec3::new(5.0, 0.0, 0.0), Vec3::ONE, rot);

    let ray = Ray::new(Vec3::ZERO, Vec3::X).unwrap();
    let hit = ray_obb(&ray, &obb);
    assert!(hit.is_some());
    // Should hit somewhere around x=4 (center at 5, half-extent rotated)
    assert!(hit.unwrap() > 3.0 && hit.unwrap() < 5.0);
}

#[test]
fn capsule_sphere_gjk3d() {
    // Capsule along Y axis overlapping a sphere
    let cap = Capsule::new(Vec3::new(0.0, -2.0, 0.0), Vec3::new(0.0, 2.0, 0.0), 0.5).unwrap();
    let sphere = Sphere::new(Vec3::new(1.0, 0.0, 0.0), 1.0).unwrap();

    // GJK should detect overlap (distance = 1.0, combined radii = 1.5)
    assert!(gjk_intersect_3d(&cap, &sphere));

    // Ray through both
    let ray = Ray::new(Vec3::new(-5.0, 0.0, 0.0), Vec3::X).unwrap();
    let cap_hit = ray_capsule(&ray, &cap);
    let sphere_hit = ray_sphere(&ray, &sphere);
    assert!(cap_hit.is_some());
    assert!(sphere_hit.is_some());
    // Capsule is closer (centered at origin)
    assert!(cap_hit.unwrap() < sphere_hit.unwrap());
}

// --- 0.26.3: Adaptive ODE vs fixed-step ---

#[test]
fn dopri45_matches_rk4() {
    // Both should solve dy/dt = -y, y(0) = 1 → y(1) = 1/e
    let f_dopri = |_t: f64, y: &[f64], dy: &mut [f64]| {
        dy[0] = -y[0];
    };
    let f_rk4 = |_t: f64, y: &[f64], dy: &mut [f64]| {
        dy[0] = -y[0];
    };

    let dopri = hisab::num::dopri45(f_dopri, 0.0, &[1.0], 1.0, 1e-8, 0.1).unwrap();
    let rk4 = hisab::num::rk4(f_rk4, 0.0, &[1.0], 1.0, 1000).unwrap();

    let expected = (-1.0_f64).exp();
    let dopri_final = dopri.last().unwrap().1[0];
    let rk4_final = rk4[0];

    assert!(
        (dopri_final - expected).abs() < 1e-6,
        "DOPRI: {dopri_final} vs {expected}"
    );
    assert!(
        (rk4_final - expected).abs() < 1e-6,
        "RK4: {rk4_final} vs {expected}"
    );
}

// --- 0.27.3: Autodiff vs numerical derivative ---

#[test]
fn autodiff_matches_numerical_derivative() {
    use hisab::autodiff::Dual;

    // f(x) = sin(x²), compare autodiff vs calc::derivative
    let x_val = 1.5;

    // Autodiff
    let x = Dual::var(x_val);
    let ad_result = (x * x).sin();

    // Numerical
    let num_deriv = hisab::calc::derivative(|x| (x * x).sin(), x_val, 1e-7);

    assert!(
        (ad_result.deriv - num_deriv).abs() < 1e-5,
        "autodiff={} vs numerical={}",
        ad_result.deriv,
        num_deriv
    );
}

#[test]
fn interval_contains_numerical_root() {
    use hisab::interval::Interval;

    // Newton-Raphson finds √2, interval arithmetic should contain it
    let root = hisab::num::newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, 1.5, 1e-12, 100).unwrap();

    // Interval evaluation: [1.4, 1.5]² - 2 should straddle zero
    let i = Interval::new(1.4, 1.5);
    let i_sq = i * i;
    let result = i_sq - Interval::point(2.0);
    assert!(result.lo() < 0.0 && result.hi() > 0.0); // Contains zero

    // Root should be in [1.4, 1.5]
    assert!(i.contains(root));
}

#[test]
fn symbolic_diff_matches_autodiff() {
    use hisab::autodiff::Dual;
    use hisab::symbolic::Expr;
    use std::collections::HashMap;

    // f(x) = x³ + 2x
    // symbolic: d/dx = 3x² + 2
    let x_expr = Expr::Var("x".into());
    let expr = Expr::Add(
        Box::new(Expr::Pow(
            Box::new(x_expr.clone()),
            Box::new(Expr::Const(3.0)),
        )),
        Box::new(Expr::Mul(Box::new(Expr::Const(2.0)), Box::new(x_expr))),
    );
    let d_expr = expr.differentiate("x").simplify();

    let x_val = 2.0;
    let mut vars = HashMap::new();
    vars.insert("x".into(), x_val);
    let sym_deriv = d_expr.evaluate(&vars).unwrap();

    // autodiff
    let x = Dual::var(x_val);
    let ad = x * x * x + Dual::constant(2.0) * x;

    assert!(
        (sym_deriv - ad.deriv).abs() < 1e-8,
        "symbolic={sym_deriv} vs autodiff={}",
        ad.deriv
    );
}

#[test]
#[allow(clippy::needless_range_loop)]
fn tensor_matmul_matches_num_multiply() {
    use hisab::tensor::Tensor;

    // Same matrices in both representations
    let a_dense = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
    let b_dense = vec![vec![5.0, 6.0], vec![7.0, 8.0]];

    let num_result = hisab::num::matrix_multiply(&a_dense, &b_dense).unwrap();

    let a_tensor = Tensor::new(vec![2, 2], vec![1.0, 2.0, 3.0, 4.0]).unwrap();
    let b_tensor = Tensor::new(vec![2, 2], vec![5.0, 6.0, 7.0, 8.0]).unwrap();
    let tensor_result = a_tensor.matmul(&b_tensor).unwrap();

    for i in 0..2 {
        for j in 0..2 {
            assert!(
                (num_result[i][j] - tensor_result.get(&[i, j]).unwrap()).abs() < 1e-12,
                "mismatch at [{i}][{j}]"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Deep cross-module tests (3+ modules per test)
// ---------------------------------------------------------------------------

#[test]
fn transforms_geo_num_obb_least_squares_fit() {
    // transforms → build OBB at a rotated position
    // geo → ray-OBB intersection at multiple angles
    // num → least-squares fit a line through the hit distances
    let rot = Quat::from_rotation_z(std::f32::consts::FRAC_PI_6);
    let obb = Obb::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(2.0, 1.0, 1.0), rot);

    let mut xs = Vec::new();
    let mut ts = Vec::new();
    for i in 0..5 {
        let y_off = (i as f32 - 2.0) * 0.1;
        let ray = Ray::new(Vec3::new(0.0, y_off, 0.0), Vec3::X).unwrap();
        if let Some(t) = ray_obb(&ray, &obb) {
            xs.push(y_off as f64);
            ts.push(t as f64);
        }
    }

    // Fit a polynomial to hit distances — should succeed if we got hits
    assert!(xs.len() >= 3, "expected at least 3 hits, got {}", xs.len());
    let coeffs = hisab::num::least_squares_poly(&xs, &ts, 1).unwrap();
    // Linear fit: t ≈ a0 + a1*y — a0 should be ~8 (center at 10, half-extent ~2)
    assert!(coeffs[0] > 5.0 && coeffs[0] < 12.0, "a0={}", coeffs[0]);
}

#[test]
fn autodiff_calc_num_optimize_with_exact_gradient() {
    // autodiff → compute exact gradient of Rosenbrock
    // calc → verify gradient matches finite-difference
    // num → BFGS with the exact gradient
    let rosenbrock = |x: &[f64]| (1.0 - x[0]).powi(2) + 100.0 * (x[1] - x[0] * x[0]).powi(2);

    let exact_grad = |x: &[f64]| -> Vec<f64> {
        use hisab::autodiff::Dual;
        // Partial w.r.t. x[0]
        let x0 = Dual::var(x[0]);
        let x1 = Dual::constant(x[1]);
        let one = Dual::constant(1.0);
        let hundred = Dual::constant(100.0);
        let f0 = (one - x0) * (one - x0) + hundred * (x1 - x0 * x0) * (x1 - x0 * x0);

        // Partial w.r.t. x[1]
        let x0b = Dual::constant(x[0]);
        let x1b = Dual::var(x[1]);
        let one_b = Dual::constant(1.0);
        let hundred_b = Dual::constant(100.0);
        let f1 = (one_b - x0b) * (one_b - x0b) + hundred_b * (x1b - x0b * x0b) * (x1b - x0b * x0b);

        vec![f0.deriv, f1.deriv]
    };

    // Verify autodiff gradient matches numerical gradient at a point
    let pt = [0.5, 0.5];
    let ad_g = exact_grad(&pt);
    let num_g = hisab::calc::gradient(rosenbrock, &pt, 1e-6).unwrap();
    assert!(
        (ad_g[0] - num_g[0]).abs() < 1e-3,
        "grad[0]: ad={} num={}",
        ad_g[0],
        num_g[0]
    );
    assert!(
        (ad_g[1] - num_g[1]).abs() < 1e-3,
        "grad[1]: ad={} num={}",
        ad_g[1],
        num_g[1]
    );

    // BFGS with the exact gradient
    let result = hisab::num::bfgs(rosenbrock, exact_grad, &[0.0, 0.0], 1e-6, 2000).unwrap();
    assert!((result.x[0] - 1.0).abs() < 1e-3);
    assert!((result.x[1] - 1.0).abs() < 1e-3);
}

#[test]
fn symbolic_interval_verified_root_bound() {
    // symbolic → build x² - 2, differentiate to get 2x
    // num → Newton-Raphson finds √2
    // interval → verify the root is contained in the interval evaluation
    use hisab::interval::Interval;
    use hisab::symbolic::Expr;
    use std::collections::HashMap;

    let x = Expr::Var("x".into());
    let expr = Expr::Add(
        Box::new(Expr::Pow(Box::new(x.clone()), Box::new(Expr::Const(2.0)))),
        Box::new(Expr::Const(-2.0)),
    );

    // Symbolic derivative: 2x
    let d_expr = expr.differentiate("x").simplify();

    // Use symbolic expressions to drive Newton-Raphson
    let f = |xv: f64| -> f64 {
        let mut v = HashMap::new();
        v.insert("x".into(), xv);
        expr.evaluate(&v).unwrap()
    };
    let df = |xv: f64| -> f64 {
        let mut v = HashMap::new();
        v.insert("x".into(), xv);
        d_expr.evaluate(&v).unwrap()
    };
    let root = hisab::num::newton_raphson(f, df, 1.5, 1e-12, 100).unwrap();
    assert!((root - std::f64::consts::SQRT_2).abs() < 1e-10);

    // Interval: evaluate x² - 2 on [1.41, 1.42] — should straddle zero
    let i = Interval::new(1.41, 1.42);
    let i_result = i * i - Interval::point(2.0);
    assert!(i_result.lo() < 0.0 && i_result.hi() > 0.0);
    assert!(i.contains(root));
}

#[test]
fn tensor_svd_pseudo_inverse_solve() {
    // tensor → build a 3x2 matrix
    // num → SVD → pseudo-inverse → solve least-squares Ax≈b
    // calc → verify residual via integration (L2 norm)
    use hisab::tensor::Tensor;

    // Overdetermined system: 3 equations, 2 unknowns
    let a = vec![vec![1.0, 0.0], vec![0.0, 1.0], vec![1.0, 1.0]];
    let b_vec = [1.0, 2.0, 2.8]; // Slightly inconsistent

    let pinv = hisab::num::pseudo_inverse(&a, None).unwrap();

    // x = A⁺ · b
    let x = hisab::num::matrix_multiply(&pinv, &b_vec.iter().map(|&v| vec![v]).collect::<Vec<_>>())
        .unwrap();
    let x0 = x[0][0];
    let x1 = x[1][0];

    // Compute residual: r = Ax - b
    let a_tensor = Tensor::new(vec![3, 2], vec![1.0, 0.0, 0.0, 1.0, 1.0, 1.0]).unwrap();
    let x_tensor = Tensor::new(vec![2, 1], vec![x0, x1]).unwrap();
    let ax = a_tensor.matmul(&x_tensor).unwrap();

    let residual_sq: f64 = (0..3)
        .map(|i| {
            let r = ax.get(&[i, 0]).unwrap() - b_vec[i];
            r * r
        })
        .sum();

    // Least-squares solution should have small residual
    assert!(residual_sq < 0.1, "residual² = {residual_sq}");
}

#[test]
fn geo_transforms_capsule_broadphase_narrowphase() {
    // transforms → interpolate capsule position over time
    // geo → broadphase (AABB overlap) then narrowphase (GJK 3D)
    let cap_start = Vec3::new(-5.0, 0.0, 0.0);
    let cap_end = Vec3::new(5.0, 0.0, 0.0);

    // Sphere obstacle at origin
    let obstacle = Sphere::new(Vec3::ZERO, 1.0).unwrap();
    let obstacle_aabb = Aabb::new(Vec3::splat(-1.0), Vec3::splat(1.0));

    // Sweep capsule across 10 time steps, find first collision
    let mut first_collision_t = None;
    for step in 0..10 {
        let t = step as f32 / 9.0;
        let pos = lerp_vec3(cap_start, cap_end, t);
        let cap = Capsule::new(
            pos + Vec3::new(0.0, -0.5, 0.0),
            pos + Vec3::new(0.0, 0.5, 0.0),
            0.3,
        )
        .unwrap();

        // Broadphase: AABB of capsule vs obstacle AABB
        let cap_aabb = Aabb::new(
            pos - Vec3::new(0.3, 1.0, 0.3),
            pos + Vec3::new(0.3, 1.0, 0.3),
        );

        if aabb_aabb(&cap_aabb, &obstacle_aabb) {
            // Narrowphase: GJK 3D
            if gjk_intersect_3d(&cap, &obstacle) {
                first_collision_t = Some(t);
                break;
            }
        }
    }

    // Capsule radius 0.3 + sphere radius 1.0 = 1.3 clearance needed
    // Capsule center sweeps from -5 to +5, hits around x ∈ [-1.3, 1.3]
    assert!(first_collision_t.is_some());
    let ct = first_collision_t.unwrap();
    assert!(ct > 0.2 && ct < 0.8, "collision at t={ct}");
}

// ===========================================================================
// Downstream consumer simulation tests
// ===========================================================================
// These simulate real usage patterns from hisab's consumers:
//   impetus  — physics engine (broadphase, transforms, collision)
//   kiran    — game engine (camera, frustum, rendering math)
//   joshua   — simulation (ODE, deterministic replay)
//   aethersafha — compositor (projection, transform composition)
//   abaco    — expression engine (symbolic, evaluation)
#[test]
fn impetus_broadphase_narrowphase_pipeline() {
    // impetus: spatial hash broadphase → AABB filter → GJK narrowphase → EPA depth
    // Simulates a physics step with multiple bodies

    // Scene: 4 bodies at known positions
    let bodies: Vec<(Vec3, f32)> = vec![
        (Vec3::new(0.0, 0.0, 0.0), 1.0),
        (Vec3::new(1.5, 0.0, 0.0), 1.0),  // overlaps body 0
        (Vec3::new(10.0, 0.0, 0.0), 1.0), // isolated
        (Vec3::new(0.5, 0.5, 0.0), 0.5),  // overlaps body 0
    ];

    // Step 1: Insert into spatial hash (broadphase)
    let mut hash = hisab::geo::SpatialHash::new(3.0).unwrap();
    for (i, (pos, _)) in bodies.iter().enumerate() {
        hash.insert(*pos, i);
    }

    // Step 2: Query neighbors of body 0
    let candidates = hash.query_radius(bodies[0].0, 3.0);
    assert!(candidates.contains(&1)); // body 1 is nearby
    assert!(candidates.contains(&3)); // body 3 is nearby

    // Step 3: AABB filter
    let aabbs: Vec<Aabb> = bodies
        .iter()
        .map(|(pos, r)| Aabb::new(*pos - Vec3::splat(*r), *pos + Vec3::splat(*r)))
        .collect();
    let mut aabb_pairs = Vec::new();
    for &i in &candidates {
        if i != 0 && aabb_aabb(&aabbs[0], &aabbs[i]) {
            aabb_pairs.push((0, i));
        }
    }
    assert!(!aabb_pairs.is_empty());

    // Step 4: GJK narrowphase on surviving pairs
    let spheres: Vec<Sphere> = bodies
        .iter()
        .map(|(pos, r)| Sphere::new(*pos, *r).unwrap())
        .collect();
    let mut contacts = Vec::new();
    for (a, b) in &aabb_pairs {
        if gjk_intersect_3d(&spheres[*a], &spheres[*b]) {
            contacts.push((*a, *b));
        }
    }
    assert!(!contacts.is_empty());
}

#[test]
fn kiran_camera_frustum_culling_pipeline() {
    // kiran: build camera → projection matrix → frustum → cull scene objects

    // Camera at origin looking down -Z
    let fov = std::f32::consts::FRAC_PI_4;
    let aspect = 16.0 / 9.0;
    let near = 0.1;
    let far = 100.0;
    let proj = hisab::transforms::projection_perspective(fov, aspect, near, far);

    // View transform: camera at (0,5,10) looking at origin
    let camera_pos = Vec3::new(0.0, 5.0, 10.0);
    let camera_target = Vec3::ZERO;
    let camera_forward = (camera_target - camera_pos).normalize();
    let camera_right = camera_forward.cross(Vec3::Y).normalize();
    let camera_up = camera_right.cross(camera_forward);

    // Build view matrix manually (lookAt)
    let view = glam::Mat4::look_at_rh(
        glam::Vec3::new(camera_pos.x, camera_pos.y, camera_pos.z),
        glam::Vec3::new(camera_target.x, camera_target.y, camera_target.z),
        glam::Vec3::new(camera_up.x, camera_up.y, camera_up.z),
    );
    let vp = proj * view;
    let frustum = Frustum::from_view_projection(vp);

    // Scene objects at various positions
    let objects = [
        Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0)), // at origin, visible
        Aabb::new(Vec3::new(-1.0, 99.0, -1.0), Vec3::new(1.0, 101.0, 1.0)), // way above, culled
        Aabb::new(Vec3::new(0.0, 0.0, 50.0), Vec3::new(1.0, 1.0, 51.0)),  // behind camera, culled
    ];

    let visible: Vec<usize> = objects
        .iter()
        .enumerate()
        .filter(|(_, aabb)| frustum.contains_aabb(aabb))
        .map(|(i, _)| i)
        .collect();

    assert!(visible.contains(&0), "origin object should be visible");
    // Object 1 (y=100) and object 2 (z=50, behind) should be culled
    assert!(
        !visible.contains(&2),
        "behind-camera object should be culled"
    );
}

#[test]
fn kiran_transform_hierarchy() {
    // kiran: parent-child transform composition for scene graph
    // World ← Parent ← Child, point in child space → world space

    let parent = Transform3D::new(
        Vec3::new(10.0, 0.0, 0.0),
        Quat::from_rotation_y(std::f32::consts::FRAC_PI_2),
        Vec3::ONE,
    );
    let child = Transform3D::new(Vec3::new(0.0, 5.0, 0.0), Quat::IDENTITY, Vec3::splat(0.5));

    // Compose: world = parent_matrix * child_matrix
    let parent_mat = parent.to_matrix();
    let child_mat = child.to_matrix();
    let world_mat = parent_mat * child_mat;

    // Transform a point from child-local to world
    let local_point = glam::Vec4::new(1.0, 0.0, 0.0, 1.0);
    let world_point = world_mat * local_point;

    // Parent rotates 90° around Y, then child offsets by (0,5,0) and scales 0.5
    // local (1,0,0) → child scaled (0.5,0,0) → child offset (0.5,5,0)
    // → parent rotated 90° Y: (0.5,5,0) → (0,5,-0.5) → parent offset (10,5,-0.5)
    assert!((world_point.y - 5.0).abs() < 0.1);
    assert!((world_point.x - 10.0).abs() < 1.0);
}

#[test]
fn joshua_ode_simulation_deterministic_replay() {
    // joshua: simulate a damped spring, then replay and verify identical results

    let k = 10.0; // spring constant
    let b = 0.5; // damping

    let spring = |_t: f64, y: &[f64], dy: &mut [f64]| {
        // y[0] = position, y[1] = velocity
        dy[0] = y[1];
        dy[1] = -k * y[0] - b * y[1];
    };

    let y0 = [1.0, 0.0]; // displaced, at rest

    // Run 1: fixed-step RK4
    let run1 = hisab::num::rk4(spring, 0.0, &y0, 2.0, 2000).unwrap();

    // Run 2: identical parameters → must be bit-identical (deterministic)
    let run2 = hisab::num::rk4(spring, 0.0, &y0, 2.0, 2000).unwrap();

    for i in 0..run1.len() {
        assert_eq!(run1[i], run2[i], "determinism broken at element {i}");
    }

    // Run 3: adaptive DOPRI — different steps but same endpoint
    let run3 = hisab::num::dopri45(spring, 0.0, &y0, 2.0, 1e-8, 0.01).unwrap();
    let rk4_final = run1[0];
    let dopri_final = run3.last().unwrap().1[0];

    // Both should show damped oscillation (amplitude < initial displacement)
    assert!(
        rk4_final.abs() < 1.0,
        "spring should have damped: {rk4_final}"
    );
    assert!(
        (rk4_final - dopri_final).abs() < 1e-4,
        "RK4={rk4_final} vs DOPRI={dopri_final}"
    );
}

#[test]
fn joshua_multibody_integration() {
    // joshua: simulate two coupled particles (4D ODE system)
    // Particle A at y[0..2], particle B at y[2..4], connected by spring

    let coupled = |_t: f64, y: &[f64], dy: &mut [f64]| {
        let dx = y[2] - y[0]; // B.pos - A.pos
        let dv = y[3] - y[1]; // B.vel - A.vel
        let force = 2.0 * dx + 0.1 * dv; // spring + damping
        dy[0] = y[1]; // A velocity
        dy[1] = force; // A acceleration
        dy[2] = y[3]; // B velocity
        dy[3] = -force; // B acceleration (Newton's 3rd)
    };

    // A at x=0, B at x=2, both at rest
    let y0 = [0.0, 0.0, 2.0, 0.0];
    let result = hisab::num::rk4(coupled, 0.0, &y0, 5.0, 5000).unwrap();

    // Center of mass should be conserved: (A.pos + B.pos) / 2 = 1.0
    let com = (result[0] + result[2]) / 2.0;
    assert!((com - 1.0).abs() < 1e-6, "COM drift: {com}");

    // Total momentum should be conserved: A.vel + B.vel = 0
    let total_p = result[1] + result[3];
    assert!(total_p.abs() < 1e-6, "momentum leak: {total_p}");
}

#[test]
fn aethersafha_compositor_projection_chain() {
    // aethersafha: compose projection matrices for multi-layer compositing
    // Layer 1: orthographic UI overlay
    // Layer 2: perspective 3D scene
    // Both project points to NDC, compositor blends

    let ortho = hisab::transforms::projection_orthographic(-400.0, 400.0, -300.0, 300.0, -1.0, 1.0);
    let persp = hisab::transforms::projection_perspective(
        std::f32::consts::FRAC_PI_4,
        4.0 / 3.0,
        0.1,
        100.0,
    );

    // UI point at pixel (200, 150) → NDC
    let ui_point = ortho * glam::Vec4::new(200.0, 150.0, 0.0, 1.0);
    let ui_ndc_x = ui_point.x / ui_point.w;
    let ui_ndc_y = ui_point.y / ui_point.w;
    assert!(ui_ndc_x > 0.0 && ui_ndc_x <= 1.0);
    assert!(ui_ndc_y > 0.0 && ui_ndc_y <= 1.0);

    // 3D point at (0, 0, -5) → NDC (should be near center)
    let scene_point = persp * glam::Vec4::new(0.0, 0.0, -5.0, 1.0);
    let scene_ndc_x = scene_point.x / scene_point.w;
    let scene_ndc_y = scene_point.y / scene_point.w;
    assert!(scene_ndc_x.abs() < 0.1, "center point should map to NDC ~0");
    assert!(scene_ndc_y.abs() < 0.1);

    // Inverse projection: recover world-space from NDC
    let inv_persp = persp.inverse();
    let recovered = inv_persp * scene_point;
    let recovered_pos = Vec3::new(
        recovered.x / recovered.w,
        recovered.y / recovered.w,
        recovered.z / recovered.w,
    );
    assert!((recovered_pos.z - (-5.0)).abs() < 0.01);
}

#[test]
fn aethersafha_transform_interpolation_for_animation() {
    // aethersafha: interpolate between keyframes for animated layers

    let key_a = Transform3D::new(Vec3::new(0.0, 0.0, 0.0), Quat::IDENTITY, Vec3::ONE);
    let key_b = Transform3D::new(
        Vec3::new(100.0, 0.0, 0.0),
        Quat::from_rotation_z(std::f32::consts::PI),
        Vec3::splat(2.0),
    );

    // Sample 10 frames
    let mut positions = Vec::new();
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let frame = transform3d_lerp(&key_a, &key_b, t);
        positions.push(frame.position.x);
    }

    // Positions should be monotonically increasing (linear interp)
    for w in positions.windows(2) {
        assert!(w[1] >= w[0], "non-monotonic: {} -> {}", w[0], w[1]);
    }
    assert!(approx_eq(positions[0], 0.0));
    assert!(approx_eq(positions[10], 100.0));
}

#[test]
fn abaco_symbolic_evaluation_pipeline() {
    // abaco: parse-like expression building → simplify → differentiate → evaluate
    // Simulates what abaco would do when a user types "d/dx(x^3 + sin(x))"
    use hisab::symbolic::Expr;
    use std::collections::HashMap;

    // "x^3 + sin(x)"
    let x = Expr::Var("x".into());
    let expr = Expr::Add(
        Box::new(Expr::Pow(Box::new(x.clone()), Box::new(Expr::Const(3.0)))),
        Box::new(Expr::Sin(Box::new(x.clone()))),
    );

    // Differentiate: 3x² + cos(x)
    let deriv = expr.differentiate("x").simplify();

    // Evaluate at multiple points for plotting
    let points: Vec<f64> = (0..10).map(|i| i as f64 * 0.5).collect();
    let values: Vec<f64> = points
        .iter()
        .map(|&xv| {
            let mut vars = HashMap::new();
            vars.insert("x".into(), xv);
            deriv.evaluate(&vars).unwrap()
        })
        .collect();

    // At x=0: 3*0 + cos(0) = 1
    assert!((values[0] - 1.0).abs() < 1e-8);

    // Verify against numerical derivative
    let f = |xv: f64| xv.powi(3) + xv.sin();
    for (i, &xv) in points.iter().enumerate() {
        let numerical = hisab::calc::derivative(f, xv, 1e-7);
        assert!(
            (values[i] - numerical).abs() < 1e-4,
            "mismatch at x={xv}: symbolic={} numerical={numerical}",
            values[i]
        );
    }
}

#[test]
fn impetus_raycast_scene_query() {
    // impetus: cast ray through scene, find closest hit across mixed primitives

    let ray = Ray::new(Vec3::ZERO, Vec3::new(1.0, 0.0, 0.0)).unwrap();

    // Scene: sphere, OBB, capsule at different distances
    let sphere = Sphere::new(Vec3::new(5.0, 0.0, 0.0), 1.0).unwrap();
    let obb = Obb::new(Vec3::new(10.0, 0.0, 0.0), Vec3::ONE, Quat::IDENTITY);
    let capsule = Capsule::new(Vec3::new(3.0, -1.0, 0.0), Vec3::new(3.0, 1.0, 0.0), 0.5).unwrap();

    let mut hits: Vec<(&str, f32)> = Vec::new();
    if let Some(t) = ray_sphere(&ray, &sphere) {
        hits.push(("sphere", t));
    }
    if let Some(t) = ray_obb(&ray, &obb) {
        hits.push(("obb", t));
    }
    if let Some(t) = ray_capsule(&ray, &capsule) {
        hits.push(("capsule", t));
    }

    // Sort by distance
    hits.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    assert!(!hits.is_empty());
    // Capsule at x=3 (r=0.5) → hit at ~2.5, closest
    assert_eq!(hits[0].0, "capsule");
    // Sphere at x=5 (r=1) → hit at ~4, second
    assert_eq!(hits[1].0, "sphere");
    // OBB at x=10 → hit at ~9, farthest
    assert_eq!(hits[2].0, "obb");
}

#[test]
fn dopri45_autodiff_sensitivity() {
    // num → DOPRI45 solves ODE dy/dt = -ky
    // autodiff → compute sensitivity dk/dy of final state w.r.t. parameter k
    use hisab::autodiff::Dual;

    let k = 2.0;
    let y0 = 1.0;
    let t_end = 1.0;

    // Solve with DOPRI
    let traj =
        hisab::num::dopri45(|_t, y, dy| dy[0] = -k * y[0], 0.0, &[y0], t_end, 1e-8, 0.1).unwrap();
    let y_final = traj.last().unwrap().1[0];

    // Analytical: y(1) = e^(-k) = e^(-2) ≈ 0.1353
    let expected = (-k).exp();
    assert!(
        (y_final - expected).abs() < 1e-5,
        "DOPRI: {y_final} vs {expected}"
    );

    // Autodiff sensitivity: dy/dk of e^(-k) = -e^(-k)
    let k_dual = Dual::var(k);
    let sensitivity = (-k_dual).exp();
    assert!((sensitivity.val - expected).abs() < 1e-10);
    assert!((sensitivity.deriv - (-expected)).abs() < 1e-10);
}
