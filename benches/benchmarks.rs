use criterion::{Criterion, black_box, criterion_group, criterion_main};

use ganit::geo::{ray_aabb, ray_plane, ray_sphere};
use ganit::transforms::{
    Transform2D, Transform3D, lerp_f32, lerp_vec3, projection_orthographic, projection_perspective,
};
use glam::{Quat, Vec2, Vec3};

// ---------------------------------------------------------------------------
// Transforms
// ---------------------------------------------------------------------------

fn bench_transforms(c: &mut Criterion) {
    let mut group = c.benchmark_group("transforms");

    group.bench_function("transform2d_to_matrix", |b| {
        let t = Transform2D::new(Vec2::new(3.0, 4.0), 0.5, Vec2::new(2.0, 2.0));
        b.iter(|| black_box(t).to_matrix())
    });

    group.bench_function("transform2d_apply_point", |b| {
        let t = Transform2D::new(Vec2::new(3.0, 4.0), 0.5, Vec2::new(2.0, 2.0));
        let p = Vec2::new(1.0, 1.0);
        b.iter(|| black_box(t).apply_to_point(black_box(p)))
    });

    group.bench_function("transform3d_to_matrix", |b| {
        let t = Transform3D::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::from_rotation_y(0.5),
            Vec3::splat(2.0),
        );
        b.iter(|| black_box(t).to_matrix())
    });

    group.bench_function("transform3d_apply_point", |b| {
        let t = Transform3D::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::from_rotation_y(0.5),
            Vec3::splat(2.0),
        );
        let p = Vec3::ONE;
        b.iter(|| black_box(t).apply_to_point(black_box(p)))
    });

    group.bench_function("projection_perspective", |b| {
        b.iter(|| {
            projection_perspective(
                black_box(std::f32::consts::FRAC_PI_4),
                black_box(16.0 / 9.0),
                black_box(0.1),
                black_box(100.0),
            )
        })
    });

    group.bench_function("projection_orthographic", |b| {
        b.iter(|| {
            projection_orthographic(
                black_box(-10.0),
                black_box(10.0),
                black_box(-10.0),
                black_box(10.0),
                black_box(0.1),
                black_box(100.0),
            )
        })
    });

    group.bench_function("lerp_f32", |b| {
        b.iter(|| lerp_f32(black_box(0.0), black_box(100.0), black_box(0.5)))
    });

    group.bench_function("lerp_vec3", |b| {
        let a = Vec3::ZERO;
        let e = Vec3::new(10.0, 20.0, 30.0);
        b.iter(|| lerp_vec3(black_box(a), black_box(e), black_box(0.5)))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Geometry
// ---------------------------------------------------------------------------

fn bench_geo(c: &mut Criterion) {
    let mut group = c.benchmark_group("geo");

    let ray = ganit::Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::Z);
    let sphere = ganit::Sphere::new(Vec3::ZERO, 1.0);
    let plane = ganit::Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y);
    let aabb = ganit::Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

    group.bench_function("ray_sphere_hit", |b| {
        b.iter(|| ray_sphere(black_box(&ray), black_box(&sphere)))
    });

    group.bench_function("ray_plane_hit", |b| {
        let r = ganit::Ray::new(Vec3::ZERO, Vec3::Y);
        b.iter(|| ray_plane(black_box(&r), black_box(&plane)))
    });

    group.bench_function("ray_aabb_hit", |b| {
        b.iter(|| ray_aabb(black_box(&ray), black_box(&aabb)))
    });

    group.bench_function("ray_sphere_miss", |b| {
        let r = ganit::Ray::new(Vec3::new(100.0, 100.0, -10.0), Vec3::Z);
        b.iter(|| ray_sphere(black_box(&r), black_box(&sphere)))
    });

    group.bench_function("aabb_contains", |b| {
        let p = Vec3::new(0.5, 0.5, 0.5);
        b.iter(|| black_box(aabb).contains(black_box(p)))
    });

    group.bench_function("sphere_contains", |b| {
        let p = Vec3::new(0.5, 0.0, 0.0);
        b.iter(|| black_box(sphere).contains_point(black_box(p)))
    });

    group.bench_function("aabb_merge", |b| {
        let other = ganit::Aabb::new(Vec3::splat(2.0), Vec3::splat(4.0));
        b.iter(|| black_box(aabb).merge(black_box(&other)))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Calculus
// ---------------------------------------------------------------------------

fn bench_calc(c: &mut Criterion) {
    let mut group = c.benchmark_group("calc");

    group.bench_function("derivative_x_squared", |b| {
        b.iter(|| ganit::calc::derivative(|x| x * x, black_box(3.0), 1e-7))
    });

    group.bench_function("integral_simpson_100", |b| {
        b.iter(|| ganit::calc::integral_simpson(|x| x * x, black_box(0.0), black_box(1.0), 100))
    });

    group.bench_function("integral_simpson_1000", |b| {
        b.iter(|| ganit::calc::integral_simpson(|x| x * x, black_box(0.0), black_box(1.0), 1000))
    });

    group.bench_function("integral_trapezoidal_100", |b| {
        b.iter(|| {
            ganit::calc::integral_trapezoidal(|x| x * x, black_box(0.0), black_box(1.0), 100)
        })
    });

    group.bench_function("integral_trapezoidal_1000", |b| {
        b.iter(|| {
            ganit::calc::integral_trapezoidal(|x| x * x, black_box(0.0), black_box(1.0), 1000)
        })
    });

    group.bench_function("bezier_quadratic", |b| {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 1.0);
        let p2 = Vec2::ONE;
        b.iter(|| ganit::calc::bezier_quadratic(p0, p1, p2, black_box(0.5)))
    });

    group.bench_function("bezier_cubic", |b| {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.25, 1.0);
        let p2 = Vec2::new(0.75, 1.0);
        let p3 = Vec2::ONE;
        b.iter(|| ganit::calc::bezier_cubic(p0, p1, p2, p3, black_box(0.5)))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Numerical methods
// ---------------------------------------------------------------------------

fn bench_num(c: &mut Criterion) {
    let mut group = c.benchmark_group("num");

    group.bench_function("newton_sqrt2", |b| {
        b.iter(|| {
            ganit::num::newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, black_box(1.5), 1e-10, 100)
        })
    });

    group.bench_function("bisection_sqrt2", |b| {
        b.iter(|| {
            ganit::num::bisection(|x| x * x - 2.0, black_box(1.0), black_box(2.0), 1e-10, 100)
        })
    });

    group.bench_function("gaussian_3x3", |b| {
        b.iter(|| {
            let mut m = vec![
                vec![1.0, 1.0, 1.0, 6.0],
                vec![2.0, 1.0, -1.0, 1.0],
                vec![1.0, -1.0, 1.0, 2.0],
            ];
            ganit::num::gaussian_elimination(black_box(&mut m))
        })
    });

    group.bench_function("gaussian_4x4", |b| {
        b.iter(|| {
            let mut m = vec![
                vec![1.0, 1.0, 1.0, 1.0, 4.0],
                vec![2.0, 1.0, 1.0, 1.0, 5.0],
                vec![1.0, 3.0, 1.0, 1.0, 6.0],
                vec![1.0, 1.0, 1.0, 4.0, 7.0],
            ];
            ganit::num::gaussian_elimination(black_box(&mut m))
        })
    });

    group.finish();
}

criterion_group!(benches, bench_transforms, bench_geo, bench_calc, bench_num,);
criterion_main!(benches);
