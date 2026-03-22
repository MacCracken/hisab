//! Integration tests exercising cross-module usage.

use ganit::geo::{ray_aabb, ray_plane, ray_sphere};
use ganit::transforms::{lerp_vec3, Transform3D};
use ganit::{Aabb, GanitError, Plane, Quat, Ray, Sphere, Vec3};

const EPSILON: f32 = 1e-4;

fn approx_eq(a: f32, b: f32) -> bool {
    (a - b).abs() < EPSILON
}

#[test]
fn transform_then_ray_intersection() {
    // Place a sphere at a transformed position, then ray-test it.
    let t = Transform3D::new(Vec3::new(0.0, 0.0, 10.0), Quat::IDENTITY, Vec3::ONE);
    let sphere_center = t.apply_to_point(Vec3::ZERO);

    let sphere = Sphere::new(sphere_center, 1.0);
    let ray = Ray::new(Vec3::ZERO, Vec3::Z);

    let hit_t = ray_sphere(&ray, &sphere).expect("should hit");
    assert!(approx_eq(hit_t, 9.0)); // 10.0 - 1.0 radius
}

#[test]
fn interpolated_ray_origin() {
    // Interpolate between two positions, cast a ray from the midpoint.
    let a = Vec3::new(-5.0, 0.5, 0.5);
    let b = Vec3::new(5.0, 0.5, 0.5);
    let mid = lerp_vec3(a, b, 0.5);

    let ray = Ray::new(mid, Vec3::Z);
    let aabb = Aabb::new(Vec3::ZERO, Vec3::new(1.0, 1.0, 10.0));

    let hit = ray_aabb(&ray, &aabb);
    assert!(hit.is_some());
}

#[test]
fn numerical_root_matches_geometry() {
    // Find the distance where a ray hits a plane using both geometry and root-finding.
    let ray = Ray::new(Vec3::ZERO, Vec3::Y);
    let plane = Plane::from_point_normal(Vec3::new(0.0, 7.0, 0.0), Vec3::Y);

    // Geometric method
    let geo_t = ray_plane(&ray, &plane).unwrap();

    // Numerical method: find t where ray.at(t).y - 7.0 = 0
    let num_t = ganit::num::bisection(|t| t - 7.0, 0.0, 20.0, 1e-10, 100).unwrap();

    assert!((geo_t as f64 - num_t).abs() < 1e-4);
}

#[test]
fn calculus_and_numerical_consistency() {
    // Integrate the derivative to verify fundamental theorem of calculus.
    // f(x) = x^3, F(x) = x^4/4
    // ∫₁² 3x² dx = F(2) - F(1) = 4 - 0.25 = 3.75
    // But actually d/dx(x^3) = 3x^2, and ∫₁² 3x² dx = [x³]₁² = 8 - 1 = 7
    let integral = ganit::calc::integral_simpson(|x| 3.0 * x * x, 1.0, 2.0, 100);
    assert!((integral - 7.0).abs() < 1e-6);

    // Verify the derivative at x=2: d/dx(x^3)|₂ = 3*4 = 12
    let deriv = ganit::calc::derivative(|x| x * x * x, 2.0, 1e-7);
    assert!((deriv - 12.0).abs() < 1e-4);
}

#[test]
fn error_types_unified() {
    // Different modules produce the same GanitError type.
    let num_err: Result<f64, GanitError> =
        ganit::num::bisection(|x| x * x + 1.0, 1.0, 2.0, 1e-10, 100);
    assert!(num_err.is_err());

    // Can pattern-match on the unified enum.
    match num_err.unwrap_err() {
        GanitError::InvalidInput(msg) => assert!(msg.contains("opposite signs")),
        other => panic!("unexpected error: {other}"),
    }
}
