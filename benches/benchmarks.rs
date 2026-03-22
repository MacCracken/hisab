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

// ---------------------------------------------------------------------------
// Batch operations (realistic workloads)
// ---------------------------------------------------------------------------

fn bench_batch(c: &mut Criterion) {
    let mut group = c.benchmark_group("batch");

    group.bench_function("ray_sphere_100", |b| {
        let ray = ganit::Ray::new(Vec3::new(0.0, 0.0, -20.0), Vec3::Z);
        let spheres: Vec<ganit::Sphere> = (0..100)
            .map(|i| ganit::Sphere::new(Vec3::new(i as f32 * 0.1 - 5.0, 0.0, 0.0), 0.5))
            .collect();
        b.iter(|| {
            let mut count = 0u32;
            for s in &spheres {
                if ray_sphere(black_box(&ray), s).is_some() {
                    count += 1;
                }
            }
            count
        })
    });

    group.bench_function("aabb_contains_100", |b| {
        let bb = ganit::Aabb::new(Vec3::ZERO, Vec3::splat(10.0));
        let points: Vec<Vec3> = (0..100)
            .map(|i| Vec3::new(i as f32 * 0.2 - 5.0, i as f32 * 0.1, 5.0))
            .collect();
        b.iter(|| {
            let mut count = 0u32;
            for p in &points {
                if bb.contains(black_box(*p)) {
                    count += 1;
                }
            }
            count
        })
    });

    group.bench_function("transform3d_batch_100", |b| {
        let t = Transform3D::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::from_rotation_y(0.5),
            Vec3::splat(2.0),
        );
        let points: Vec<Vec3> = (0..100)
            .map(|i| Vec3::new(i as f32, i as f32 * 0.5, i as f32 * 0.1))
            .collect();
        b.iter(|| {
            let mut sum = Vec3::ZERO;
            for p in &points {
                sum += t.apply_to_point(black_box(*p));
            }
            sum
        })
    });

    group.bench_function("simpson_sin_10000", |b| {
        b.iter(|| {
            ganit::calc::integral_simpson(
                f64::sin,
                black_box(0.0),
                black_box(std::f64::consts::PI),
                10000,
            )
        })
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// V0.2 types
// ---------------------------------------------------------------------------

fn bench_v02(c: &mut Criterion) {
    let mut group = c.benchmark_group("v02");

    group.bench_function("ray_triangle", |b| {
        let ray = ganit::Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::Z);
        let tri = ganit::Triangle::new(
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        b.iter(|| ganit::geo::ray_triangle(black_box(&ray), black_box(&tri)))
    });

    group.bench_function("aabb_aabb_overlap", |b| {
        let a = ganit::Aabb::new(Vec3::ZERO, Vec3::ONE);
        let bb = ganit::Aabb::new(Vec3::splat(0.5), Vec3::splat(1.5));
        b.iter(|| ganit::geo::aabb_aabb(black_box(&a), black_box(&bb)))
    });

    group.bench_function("sphere_sphere_overlap", |b| {
        let a = ganit::Sphere::new(Vec3::ZERO, 1.0);
        let bb = ganit::Sphere::new(Vec3::new(1.5, 0.0, 0.0), 1.0);
        b.iter(|| ganit::geo::sphere_sphere(black_box(&a), black_box(&bb)))
    });

    group.bench_function("frustum_contains_point", |b| {
        let proj = ganit::transforms::projection_perspective(
            std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0,
        );
        let frustum = ganit::Frustum::from_view_projection(proj);
        let point = Vec3::new(0.0, 0.0, -10.0);
        b.iter(|| frustum.contains_point(black_box(point)))
    });

    group.bench_function("frustum_contains_aabb", |b| {
        let proj = ganit::transforms::projection_perspective(
            std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0,
        );
        let frustum = ganit::Frustum::from_view_projection(proj);
        let bb = ganit::Aabb::new(Vec3::new(-1.0, -1.0, -5.0), Vec3::new(1.0, 1.0, -3.0));
        b.iter(|| frustum.contains_aabb(black_box(&bb)))
    });

    group.bench_function("slerp", |b| {
        let a = Quat::IDENTITY;
        let bb = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        b.iter(|| ganit::transforms::slerp(black_box(a), black_box(bb), black_box(0.5)))
    });

    group.bench_function("transform3d_lerp", |b| {
        let a = Transform3D::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
        let bb = Transform3D::new(
            Vec3::new(10.0, 0.0, 0.0),
            Quat::from_rotation_y(1.0),
            Vec3::splat(2.0),
        );
        b.iter(|| ganit::transforms::transform3d_lerp(black_box(&a), black_box(&bb), black_box(0.5)))
    });

    group.bench_function("closest_on_aabb", |b| {
        let bb = ganit::Aabb::new(Vec3::ZERO, Vec3::ONE);
        let point = Vec3::new(5.0, 0.5, -3.0);
        b.iter(|| ganit::geo::closest_point_on_aabb(black_box(&bb), black_box(point)))
    });

    group.bench_function("segment_closest_point", |b| {
        let seg = ganit::Segment::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
        let point = Vec3::new(5.0, 3.0, 0.0);
        b.iter(|| seg.closest_point(black_box(point)))
    });

    group.bench_function("plane_plane_intersection", |b| {
        let a = ganit::Plane::from_point_normal(Vec3::ZERO, Vec3::Y);
        let bb = ganit::Plane::from_point_normal(Vec3::ZERO, Vec3::X);
        b.iter(|| ganit::geo::plane_plane(black_box(&a), black_box(&bb)))
    });

    group.bench_function("triangle_unit_normal", |b| {
        let tri = ganit::Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        b.iter(|| black_box(tri).unit_normal())
    });

    group.bench_function("line_closest_point", |b| {
        let l = ganit::Line::new(Vec3::ZERO, Vec3::X);
        let point = Vec3::new(5.0, 3.0, 4.0);
        b.iter(|| l.closest_point(black_box(point)))
    });

    group.bench_function("closest_on_sphere", |b| {
        let s = ganit::Sphere::new(Vec3::ZERO, 5.0);
        let point = Vec3::new(10.0, 0.0, 0.0);
        b.iter(|| ganit::geo::closest_point_on_sphere(black_box(&s), black_box(point)))
    });

    group.bench_function("inverse_matrix", |b| {
        let t = Transform3D::new(
            Vec3::new(1.0, 2.0, 3.0),
            Quat::from_rotation_y(0.5),
            Vec3::splat(2.0),
        );
        b.iter(|| black_box(t).inverse_matrix())
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_transforms,
    bench_geo,
    bench_calc,
    bench_num,
    bench_batch,
    bench_v02,
);
criterion_main!(benches);
