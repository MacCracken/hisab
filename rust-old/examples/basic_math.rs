//! Basic usage of hisab math operations.

use hisab::transforms::{Transform3D, lerp_f32, projection_perspective};
use hisab::{Quat, Ray, Sphere, Vec3};

fn main() {
    // 3D transform
    let t = Transform3D::new(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE);
    let world = t.apply_to_point(Vec3::ZERO);
    println!("Transform origin -> world: {world}");

    // Perspective projection
    let proj = projection_perspective(std::f32::consts::FRAC_PI_4, 16.0 / 9.0, 0.1, 100.0);
    println!("Perspective matrix: {proj}");

    // Interpolation
    let val = lerp_f32(0.0, 100.0, 0.75);
    println!("lerp(0, 100, 0.75) = {val}");

    // Ray-sphere intersection
    let ray = Ray::new(Vec3::ZERO, Vec3::Z).unwrap();
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, 10.0), 2.0).unwrap();
    match hisab::geo::ray_sphere(&ray, &sphere) {
        Some(t) => println!("Ray hits sphere at t={t}, point={}", ray.at(t)),
        None => println!("Ray misses sphere"),
    }

    // Numerical integration (area under x^2 from 0 to 1)
    let area = hisab::calc::integral_simpson(|x| x * x, 0.0, 1.0, 100).expect("should converge");
    println!("∫₀¹ x² dx ≈ {area:.6}");

    // Root finding (sqrt of 2)
    let sqrt2 = hisab::num::newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, 1.0, 1e-10, 100)
        .expect("should converge");
    println!("√2 ≈ {sqrt2:.10}");
}
