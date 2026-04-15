use criterion::{Criterion, black_box, criterion_group, criterion_main};

use glam::{Quat, Vec2, Vec3};
use hisab::geo::{ray_aabb, ray_plane, ray_sphere};
use hisab::transforms::{
    Transform2D, Transform3D, lerp_f32, lerp_vec3, projection_orthographic, projection_perspective,
};

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

    let ray = hisab::Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::Z).unwrap();
    let sphere = hisab::Sphere::new(Vec3::ZERO, 1.0).unwrap();
    let plane = hisab::Plane::from_point_normal(Vec3::new(0.0, 5.0, 0.0), Vec3::Y).unwrap();
    let aabb = hisab::Aabb::new(Vec3::new(-1.0, -1.0, -1.0), Vec3::new(1.0, 1.0, 1.0));

    group.bench_function("ray_sphere_hit", |b| {
        b.iter(|| ray_sphere(black_box(&ray), black_box(&sphere)))
    });

    group.bench_function("ray_plane_hit", |b| {
        let r = hisab::Ray::new(Vec3::ZERO, Vec3::Y).unwrap();
        b.iter(|| ray_plane(black_box(&r), black_box(&plane)))
    });

    group.bench_function("ray_aabb_hit", |b| {
        b.iter(|| ray_aabb(black_box(&ray), black_box(&aabb)))
    });

    group.bench_function("ray_sphere_miss", |b| {
        let r = hisab::Ray::new(Vec3::new(100.0, 100.0, -10.0), Vec3::Z).unwrap();
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
        let other = hisab::Aabb::new(Vec3::splat(2.0), Vec3::splat(4.0));
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
        b.iter(|| hisab::calc::derivative(|x| x * x, black_box(3.0), 1e-7))
    });

    group.bench_function("integral_simpson_100", |b| {
        b.iter(|| hisab::calc::integral_simpson(|x| x * x, black_box(0.0), black_box(1.0), 100))
    });

    group.bench_function("integral_simpson_1000", |b| {
        b.iter(|| hisab::calc::integral_simpson(|x| x * x, black_box(0.0), black_box(1.0), 1000))
    });

    group.bench_function("integral_trapezoidal_100", |b| {
        b.iter(|| hisab::calc::integral_trapezoidal(|x| x * x, black_box(0.0), black_box(1.0), 100))
    });

    group.bench_function("integral_trapezoidal_1000", |b| {
        b.iter(|| {
            hisab::calc::integral_trapezoidal(|x| x * x, black_box(0.0), black_box(1.0), 1000)
        })
    });

    group.bench_function("bezier_quadratic", |b| {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.5, 1.0);
        let p2 = Vec2::ONE;
        b.iter(|| hisab::calc::bezier_quadratic(p0, p1, p2, black_box(0.5)))
    });

    group.bench_function("bezier_cubic", |b| {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(0.25, 1.0);
        let p2 = Vec2::new(0.75, 1.0);
        let p3 = Vec2::ONE;
        b.iter(|| hisab::calc::bezier_cubic(p0, p1, p2, p3, black_box(0.5)))
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
            hisab::num::newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, black_box(1.5), 1e-10, 100)
        })
    });

    group.bench_function("bisection_sqrt2", |b| {
        b.iter(|| {
            hisab::num::bisection(|x| x * x - 2.0, black_box(1.0), black_box(2.0), 1e-10, 100)
        })
    });

    group.bench_function("gaussian_3x3", |b| {
        b.iter(|| {
            let mut m = vec![
                vec![1.0, 1.0, 1.0, 6.0],
                vec![2.0, 1.0, -1.0, 1.0],
                vec![1.0, -1.0, 1.0, 2.0],
            ];
            hisab::num::gaussian_elimination(black_box(&mut m))
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
            hisab::num::gaussian_elimination(black_box(&mut m))
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
        let ray = hisab::Ray::new(Vec3::new(0.0, 0.0, -20.0), Vec3::Z).unwrap();
        let spheres: Vec<hisab::Sphere> = (0..100)
            .map(|i| hisab::Sphere::new(Vec3::new(i as f32 * 0.1 - 5.0, 0.0, 0.0), 0.5).unwrap())
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
        let bb = hisab::Aabb::new(Vec3::ZERO, Vec3::splat(10.0));
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
            hisab::calc::integral_simpson(
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
        let ray = hisab::Ray::new(Vec3::new(0.0, 0.0, -10.0), Vec3::Z).unwrap();
        let tri = hisab::Triangle::new(
            Vec3::new(-1.0, -1.0, 0.0),
            Vec3::new(1.0, -1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
        );
        b.iter(|| hisab::geo::ray_triangle(black_box(&ray), black_box(&tri)))
    });

    group.bench_function("aabb_aabb_overlap", |b| {
        let a = hisab::Aabb::new(Vec3::ZERO, Vec3::ONE);
        let bb = hisab::Aabb::new(Vec3::splat(0.5), Vec3::splat(1.5));
        b.iter(|| hisab::geo::aabb_aabb(black_box(&a), black_box(&bb)))
    });

    group.bench_function("sphere_sphere_overlap", |b| {
        let a = hisab::Sphere::new(Vec3::ZERO, 1.0).unwrap();
        let bb = hisab::Sphere::new(Vec3::new(1.5, 0.0, 0.0), 1.0).unwrap();
        b.iter(|| hisab::geo::sphere_sphere(black_box(&a), black_box(&bb)))
    });

    group.bench_function("frustum_contains_point", |b| {
        let proj =
            hisab::transforms::projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = hisab::Frustum::from_view_projection(proj);
        let point = Vec3::new(0.0, 0.0, -10.0);
        b.iter(|| frustum.contains_point(black_box(point)))
    });

    group.bench_function("frustum_contains_aabb", |b| {
        let proj =
            hisab::transforms::projection_perspective(std::f32::consts::FRAC_PI_4, 1.0, 0.1, 100.0);
        let frustum = hisab::Frustum::from_view_projection(proj);
        let bb = hisab::Aabb::new(Vec3::new(-1.0, -1.0, -5.0), Vec3::new(1.0, 1.0, -3.0));
        b.iter(|| frustum.contains_aabb(black_box(&bb)))
    });

    group.bench_function("slerp", |b| {
        let a = Quat::IDENTITY;
        let bb = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        b.iter(|| hisab::transforms::slerp(black_box(a), black_box(bb), black_box(0.5)))
    });

    group.bench_function("transform3d_lerp", |b| {
        let a = Transform3D::new(Vec3::ZERO, Quat::IDENTITY, Vec3::ONE);
        let bb = Transform3D::new(
            Vec3::new(10.0, 0.0, 0.0),
            Quat::from_rotation_y(1.0),
            Vec3::splat(2.0),
        );
        b.iter(|| {
            hisab::transforms::transform3d_lerp(black_box(&a), black_box(&bb), black_box(0.5))
        })
    });

    group.bench_function("closest_on_aabb", |b| {
        let bb = hisab::Aabb::new(Vec3::ZERO, Vec3::ONE);
        let point = Vec3::new(5.0, 0.5, -3.0);
        b.iter(|| hisab::geo::closest_point_on_aabb(black_box(&bb), black_box(point)))
    });

    group.bench_function("segment_closest_point", |b| {
        let seg = hisab::Segment::new(Vec3::ZERO, Vec3::new(10.0, 0.0, 0.0));
        let point = Vec3::new(5.0, 3.0, 0.0);
        b.iter(|| seg.closest_point(black_box(point)))
    });

    group.bench_function("plane_plane_intersection", |b| {
        let a = hisab::Plane::from_point_normal(Vec3::ZERO, Vec3::Y).unwrap();
        let bb = hisab::Plane::from_point_normal(Vec3::ZERO, Vec3::X).unwrap();
        b.iter(|| hisab::geo::plane_plane(black_box(&a), black_box(&bb)))
    });

    group.bench_function("triangle_unit_normal", |b| {
        let tri = hisab::Triangle::new(Vec3::ZERO, Vec3::X, Vec3::Y);
        b.iter(|| black_box(tri).unit_normal())
    });

    group.bench_function("line_closest_point", |b| {
        let l = hisab::Line::new(Vec3::ZERO, Vec3::X).unwrap();
        let point = Vec3::new(5.0, 3.0, 4.0);
        b.iter(|| l.closest_point(black_box(point)))
    });

    group.bench_function("closest_on_sphere", |b| {
        let s = hisab::Sphere::new(Vec3::ZERO, 5.0).unwrap();
        let point = Vec3::new(10.0, 0.0, 0.0);
        b.iter(|| hisab::geo::closest_point_on_sphere(black_box(&s), black_box(point)))
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

// ---------------------------------------------------------------------------
// V0.3 curves & splines
// ---------------------------------------------------------------------------

fn bench_v03(c: &mut Criterion) {
    let mut group = c.benchmark_group("v03");

    group.bench_function("bezier_cubic_3d", |b| {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(1.0, 2.0, 0.0);
        let p2 = Vec3::new(3.0, 2.0, 1.0);
        let p3 = Vec3::new(4.0, 0.0, 1.0);
        b.iter(|| hisab::calc::bezier_cubic_3d(p0, p1, p2, p3, black_box(0.5)))
    });

    group.bench_function("de_casteljau_split", |b| {
        let p0 = Vec2::ZERO;
        let p1 = Vec2::new(1.0, 2.0);
        let p2 = Vec2::new(3.0, 2.0);
        let p3 = Vec2::new(4.0, 0.0);
        b.iter(|| hisab::calc::de_casteljau_split(p0, p1, p2, p3, black_box(0.5)))
    });

    group.bench_function("catmull_rom", |b| {
        let p0 = Vec3::new(-1.0, 0.0, 0.0);
        let p1 = Vec3::ZERO;
        let p2 = Vec3::new(1.0, 1.0, 0.0);
        let p3 = Vec3::new(2.0, 1.0, 0.0);
        b.iter(|| hisab::calc::catmull_rom(p0, p1, p2, p3, black_box(0.5)))
    });

    group.bench_function("bspline_cubic", |b| {
        let pts = [
            Vec3::ZERO,
            Vec3::new(1.0, 2.0, 0.0),
            Vec3::new(3.0, 2.0, 0.0),
            Vec3::new(4.0, 0.0, 0.0),
        ];
        let knots = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0];
        b.iter(|| hisab::calc::bspline_eval(3, &pts, &knots, black_box(0.5)))
    });

    group.bench_function("gauss_legendre_5", |b| {
        b.iter(|| hisab::calc::integral_gauss_legendre_5(|x| x * x, black_box(0.0), black_box(3.0)))
    });

    group.bench_function("gauss_legendre_10_panels", |b| {
        b.iter(|| {
            hisab::calc::integral_gauss_legendre(
                f64::sin,
                black_box(0.0),
                black_box(std::f64::consts::PI),
                10,
            )
        })
    });

    group.bench_function("arc_length_100", |b| {
        let p0 = Vec3::ZERO;
        let p1 = Vec3::new(1.0, 2.0, 0.0);
        let p2 = Vec3::new(3.0, 2.0, 1.0);
        let p3 = Vec3::new(4.0, 0.0, 1.0);
        b.iter(|| hisab::calc::bezier_cubic_3d_arc_length(p0, p1, p2, p3, 100))
    });

    group.bench_function("ease_in_out", |b| {
        b.iter(|| hisab::calc::ease_in_out(black_box(0.5)))
    });

    group.bench_function("ease_in_out_smooth", |b| {
        b.iter(|| hisab::calc::ease_in_out_smooth(black_box(0.5)))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// V0.4a linear algebra
// ---------------------------------------------------------------------------

fn bench_v04a(c: &mut Criterion) {
    let mut group = c.benchmark_group("v04a");

    group.bench_function("lu_decompose_3x3", |b| {
        let a = vec![
            vec![1.0, 1.0, 1.0],
            vec![2.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0],
        ];
        b.iter(|| hisab::num::lu_decompose(black_box(&a)))
    });

    group.bench_function("lu_solve_3x3", |b| {
        let a = vec![
            vec![1.0, 1.0, 1.0],
            vec![2.0, 1.0, -1.0],
            vec![1.0, -1.0, 1.0],
        ];
        let (lu, pivot) = hisab::num::lu_decompose(&a).unwrap();
        let rhs = [6.0, 1.0, 2.0];
        b.iter(|| hisab::num::lu_solve(black_box(&lu), black_box(&pivot), black_box(&rhs)))
    });

    group.bench_function("cholesky_3x3", |b| {
        let a = vec![
            vec![4.0, 2.0, 1.0],
            vec![2.0, 5.0, 2.0],
            vec![1.0, 2.0, 6.0],
        ];
        b.iter(|| hisab::num::cholesky(black_box(&a)))
    });

    group.bench_function("cholesky_solve_3x3", |b| {
        let a = vec![
            vec![4.0, 2.0, 1.0],
            vec![2.0, 5.0, 2.0],
            vec![1.0, 2.0, 6.0],
        ];
        let l = hisab::num::cholesky(&a).unwrap();
        let rhs = [1.0, 2.0, 3.0];
        b.iter(|| hisab::num::cholesky_solve(black_box(&l), black_box(&rhs)))
    });

    group.bench_function("qr_decompose_3col", |b| {
        let a = vec![
            vec![1.0, 0.0, 1.0],
            vec![1.0, 1.0, 0.0],
            vec![0.0, 1.0, 1.0],
        ];
        b.iter(|| hisab::num::qr_decompose(black_box(&a)))
    });

    group.bench_function("least_squares_linear_6pt", |b| {
        let x = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];
        let y = [2.1, 4.9, 8.1, 10.9, 14.1, 16.9];
        b.iter(|| hisab::num::least_squares_poly(black_box(&x), black_box(&y), 1))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// V0.4b spectral & dynamics
// ---------------------------------------------------------------------------

fn bench_v04b(c: &mut Criterion) {
    let mut group = c.benchmark_group("v04b");

    group.bench_function("eigenvalue_3x3", |b| {
        let a = vec![
            vec![2.0, 1.0, 0.0],
            vec![1.0, 3.0, 1.0],
            vec![0.0, 1.0, 2.0],
        ];
        b.iter(|| hisab::num::eigenvalue_power(black_box(&a), 1e-10, 100))
    });

    group.bench_function("fft_64", |b| {
        let data: Vec<hisab::num::Complex> = (0..64)
            .map(|i| hisab::num::Complex::from_real(i as f64))
            .collect();
        b.iter(|| {
            let mut d = data.clone();
            hisab::num::fft(black_box(&mut d)).unwrap();
            d
        })
    });

    group.bench_function("fft_1024", |b| {
        let data: Vec<hisab::num::Complex> = (0..1024)
            .map(|i| hisab::num::Complex::from_real((i as f64 * 0.1).sin()))
            .collect();
        b.iter(|| {
            let mut d = data.clone();
            hisab::num::fft(black_box(&mut d)).unwrap();
            d
        })
    });

    group.bench_function("fft_ifft_256", |b| {
        let data: Vec<hisab::num::Complex> = (0..256)
            .map(|i| hisab::num::Complex::new(i as f64, (i as f64 * 0.3).cos()))
            .collect();
        b.iter(|| {
            let mut d = data.clone();
            hisab::num::fft(&mut d).unwrap();
            hisab::num::ifft(&mut d).unwrap();
            d
        })
    });

    group.bench_function("dst_64", |b| {
        let data: Vec<f64> = (0..64).map(|i| (i as f64 * 0.1).sin()).collect();
        b.iter(|| hisab::num::dst(black_box(&data)))
    });

    group.bench_function("dct_64", |b| {
        let data: Vec<f64> = (0..64).map(|i| (i as f64 * 0.1).sin()).collect();
        b.iter(|| hisab::num::dct(black_box(&data)))
    });

    group.bench_function("dst_idst_256", |b| {
        let data: Vec<f64> = (0..256).map(|i| (i as f64 * 0.05).cos()).collect();
        b.iter(|| {
            let t = hisab::num::dst(black_box(&data)).unwrap();
            hisab::num::idst(black_box(&t))
        })
    });

    group.bench_function("dct_idct_256", |b| {
        let data: Vec<f64> = (0..256).map(|i| (i as f64 * 0.05).cos()).collect();
        b.iter(|| {
            let t = hisab::num::dct(black_box(&data)).unwrap();
            hisab::num::idct(black_box(&t))
        })
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// V0.4c ODE solvers
// ---------------------------------------------------------------------------

fn bench_v04c(c: &mut Criterion) {
    let mut group = c.benchmark_group("v04c");

    group.bench_function("rk4_exp_100_steps", |b| {
        b.iter(|| {
            hisab::num::rk4(
                |_t, y, out: &mut [f64]| {
                    out[0] = y[0];
                },
                0.0,
                black_box(&[1.0]),
                1.0,
                100,
            )
        })
    });

    group.bench_function("rk4_exp_1000_steps", |b| {
        b.iter(|| {
            hisab::num::rk4(
                |_t, y, out: &mut [f64]| {
                    out[0] = y[0];
                },
                0.0,
                black_box(&[1.0]),
                1.0,
                1000,
            )
        })
    });

    group.bench_function("rk4_oscillator_1000", |b| {
        b.iter(|| {
            hisab::num::rk4(
                |_t, y, out: &mut [f64]| {
                    out[0] = y[1];
                    out[1] = -y[0];
                },
                0.0,
                black_box(&[1.0, 0.0]),
                std::f64::consts::PI,
                1000,
            )
        })
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// V0.5a spatial indexes
// ---------------------------------------------------------------------------

fn bench_v05a(c: &mut Criterion) {
    let mut group = c.benchmark_group("v05a");

    group.bench_function("bvh_build_100", |b| {
        b.iter(|| {
            let mut items: Vec<(hisab::Aabb, usize)> = (0..100)
                .map(|i| {
                    let x = (i % 10) as f32;
                    let y = (i / 10) as f32;
                    (
                        hisab::Aabb::new(Vec3::new(x, y, 0.0), Vec3::new(x + 0.5, y + 0.5, 0.5)),
                        i,
                    )
                })
                .collect();
            hisab::Bvh::build(black_box(&mut items))
        })
    });

    group.bench_function("bvh_ray_query_100", |b| {
        let mut items: Vec<(hisab::Aabb, usize)> = (0..100)
            .map(|i| {
                let x = (i % 10) as f32;
                let y = (i / 10) as f32;
                (
                    hisab::Aabb::new(Vec3::new(x, y, 0.0), Vec3::new(x + 0.5, y + 0.5, 0.5)),
                    i,
                )
            })
            .collect();
        let bvh = hisab::Bvh::build(&mut items);
        let ray = hisab::Ray::new(Vec3::new(0.25, 0.25, -10.0), Vec3::Z).unwrap();
        b.iter(|| bvh.query_ray(black_box(&ray)))
    });

    group.bench_function("bvh_build_1000", |b| {
        b.iter(|| {
            let mut items: Vec<(hisab::Aabb, usize)> = (0..1000)
                .map(|i| {
                    let x = (i % 10) as f32 * 2.0;
                    let y = ((i / 10) % 10) as f32 * 2.0;
                    let z = (i / 100) as f32 * 2.0;
                    (
                        hisab::Aabb::new(Vec3::new(x, y, z), Vec3::new(x + 1.0, y + 1.0, z + 1.0)),
                        i,
                    )
                })
                .collect();
            hisab::Bvh::build(black_box(&mut items))
        })
    });

    group.bench_function("kdtree_build_1000", |b| {
        b.iter(|| {
            let mut items: Vec<(Vec3, usize)> = (0..1000)
                .map(|i| {
                    let x = (i % 10) as f32;
                    let y = ((i / 10) % 10) as f32;
                    let z = (i / 100) as f32;
                    (Vec3::new(x, y, z), i)
                })
                .collect();
            hisab::KdTree::build(black_box(&mut items))
        })
    });

    group.bench_function("kdtree_nearest_1000", |b| {
        let mut items: Vec<(Vec3, usize)> = (0..1000)
            .map(|i| {
                let x = (i % 10) as f32;
                let y = ((i / 10) % 10) as f32;
                let z = (i / 100) as f32;
                (Vec3::new(x, y, z), i)
            })
            .collect();
        let tree = hisab::KdTree::build(&mut items);
        b.iter(|| tree.nearest(black_box(Vec3::new(4.5, 4.5, 4.5))))
    });

    group.bench_function("kdtree_radius_1000", |b| {
        let mut items: Vec<(Vec3, usize)> = (0..1000)
            .map(|i| {
                let x = (i % 10) as f32;
                let y = ((i / 10) % 10) as f32;
                let z = (i / 100) as f32;
                (Vec3::new(x, y, z), i)
            })
            .collect();
        let tree = hisab::KdTree::build(&mut items);
        b.iter(|| tree.within_radius(black_box(Vec3::new(5.0, 5.0, 5.0)), black_box(2.0)))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// V0.5b spatial grids
// ---------------------------------------------------------------------------

fn bench_v05b(c: &mut Criterion) {
    let mut group = c.benchmark_group("v05b");

    group.bench_function("quadtree_insert_1000", |b| {
        let bounds = hisab::Rect::new(glam::Vec2::ZERO, glam::Vec2::splat(100.0));
        b.iter(|| {
            let mut qt = hisab::Quadtree::new(bounds, 8, 8);
            for i in 0..1000 {
                let x = (i % 100) as f32;
                let y = (i / 100) as f32 * 10.0;
                qt.insert(glam::Vec2::new(x, y), i);
            }
            qt
        })
    });

    group.bench_function("quadtree_query_1000", |b| {
        let bounds = hisab::Rect::new(glam::Vec2::ZERO, glam::Vec2::splat(100.0));
        let mut qt = hisab::Quadtree::new(bounds, 8, 8);
        for i in 0..1000 {
            qt.insert(
                glam::Vec2::new((i % 100) as f32, (i / 100) as f32 * 10.0),
                i,
            );
        }
        let query = hisab::Rect::new(glam::Vec2::new(40.0, 40.0), glam::Vec2::new(60.0, 60.0));
        b.iter(|| qt.query_rect(black_box(&query)))
    });

    group.bench_function("octree_insert_1000", |b| {
        let bounds = hisab::Aabb::new(Vec3::ZERO, Vec3::splat(100.0));
        b.iter(|| {
            let mut ot = hisab::Octree::new(bounds, 8, 8);
            for i in 0..1000 {
                let x = (i % 10) as f32 * 10.0;
                let y = ((i / 10) % 10) as f32 * 10.0;
                let z = (i / 100) as f32 * 10.0;
                ot.insert(Vec3::new(x + 1.0, y + 1.0, z + 1.0), i);
            }
            ot
        })
    });

    group.bench_function("octree_query_1000", |b| {
        let bounds = hisab::Aabb::new(Vec3::ZERO, Vec3::splat(100.0));
        let mut ot = hisab::Octree::new(bounds, 8, 8);
        for i in 0..1000 {
            let x = (i % 10) as f32 * 10.0;
            let y = ((i / 10) % 10) as f32 * 10.0;
            let z = (i / 100) as f32 * 10.0;
            ot.insert(Vec3::new(x + 1.0, y + 1.0, z + 1.0), i);
        }
        let query = hisab::Aabb::new(Vec3::new(40.0, 40.0, 40.0), Vec3::new(60.0, 60.0, 60.0));
        b.iter(|| ot.query_aabb(black_box(&query)))
    });

    group.bench_function("spatial_hash_insert_1000", |b| {
        b.iter(|| {
            let mut sh = hisab::SpatialHash::new(10.0).unwrap();
            for i in 0..1000 {
                sh.insert(Vec3::new(i as f32, 0.0, 0.0), i);
            }
            sh
        })
    });

    group.bench_function("spatial_hash_query_cell", |b| {
        let mut sh = hisab::SpatialHash::new(10.0).unwrap();
        for i in 0..1000 {
            sh.insert(Vec3::new(i as f32, 0.0, 0.0), i);
        }
        b.iter(|| sh.query_cell(black_box(Vec3::new(50.0, 0.0, 0.0))))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// V0.5c collision
// ---------------------------------------------------------------------------

fn bench_v05c(c: &mut Criterion) {
    let mut group = c.benchmark_group("v05c");

    group.bench_function("convex_hull_100", |b| {
        let pts: Vec<glam::Vec2> = (0..100)
            .map(|i| {
                let angle = i as f32 * std::f32::consts::TAU / 100.0;
                glam::Vec2::new(angle.cos() * 10.0, angle.sin() * 10.0)
            })
            .collect();
        b.iter(|| hisab::geo::convex_hull_2d(black_box(&pts)))
    });

    group.bench_function("gjk_intersect", |b| {
        let a = hisab::ConvexPolygon::new(vec![
            glam::Vec2::new(-1.0, -1.0),
            glam::Vec2::new(1.0, -1.0),
            glam::Vec2::new(1.0, 1.0),
            glam::Vec2::new(-1.0, 1.0),
        ])
        .unwrap();
        let bb = hisab::ConvexPolygon::new(vec![
            glam::Vec2::new(0.5, -1.0),
            glam::Vec2::new(2.5, -1.0),
            glam::Vec2::new(2.5, 1.0),
            glam::Vec2::new(0.5, 1.0),
        ])
        .unwrap();
        b.iter(|| hisab::geo::gjk_intersect(black_box(&a), black_box(&bb)))
    });

    group.bench_function("gjk_no_intersect", |b| {
        let a = hisab::ConvexPolygon::new(vec![
            glam::Vec2::new(-1.0, -1.0),
            glam::Vec2::new(1.0, -1.0),
            glam::Vec2::new(1.0, 1.0),
            glam::Vec2::new(-1.0, 1.0),
        ])
        .unwrap();
        let bb = hisab::ConvexPolygon::new(vec![
            glam::Vec2::new(5.0, -1.0),
            glam::Vec2::new(7.0, -1.0),
            glam::Vec2::new(7.0, 1.0),
            glam::Vec2::new(5.0, 1.0),
        ])
        .unwrap();
        b.iter(|| hisab::geo::gjk_intersect(black_box(&a), black_box(&bb)))
    });

    group.bench_function("gjk_epa_penetration", |b| {
        let a = hisab::ConvexPolygon::new(vec![
            glam::Vec2::new(-1.0, -1.0),
            glam::Vec2::new(1.0, -1.0),
            glam::Vec2::new(1.0, 1.0),
            glam::Vec2::new(-1.0, 1.0),
        ])
        .unwrap();
        let bb = hisab::ConvexPolygon::new(vec![
            glam::Vec2::new(0.5, -1.0),
            glam::Vec2::new(2.5, -1.0),
            glam::Vec2::new(2.5, 1.0),
            glam::Vec2::new(0.5, 1.0),
        ])
        .unwrap();
        b.iter(|| hisab::geo::gjk_epa(black_box(&a), black_box(&bb)))
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// v06: SVD + Extended Linear Algebra
// ---------------------------------------------------------------------------

fn bench_v06(c: &mut Criterion) {
    let mut group = c.benchmark_group("v06");

    group.bench_function("svd_3x3", |b| {
        let a = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 10.0],
        ];
        b.iter(|| hisab::num::svd(black_box(&a)))
    });

    group.bench_function("svd_5x5", |b| {
        let a = vec![
            vec![2.0, 1.0, 0.0, 3.0, 1.0],
            vec![1.0, 4.0, 2.0, 0.0, 1.0],
            vec![0.0, 2.0, 3.0, 1.0, 2.0],
            vec![3.0, 0.0, 1.0, 5.0, 0.0],
            vec![1.0, 1.0, 2.0, 0.0, 4.0],
        ];
        b.iter(|| hisab::num::svd(black_box(&a)))
    });

    group.bench_function("matrix_inverse_3x3", |b| {
        let a = vec![
            vec![1.0, 2.0, 3.0],
            vec![0.0, 4.0, 5.0],
            vec![1.0, 0.0, 6.0],
        ];
        b.iter(|| hisab::num::matrix_inverse(black_box(&a)))
    });

    group.bench_function("pseudo_inverse_3x2", |b| {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        b.iter(|| hisab::num::pseudo_inverse(black_box(&a), None))
    });

    group.bench_function("csr_spmv_100x100", |b| {
        // Sparse identity-like: ~10% fill
        let mut dense = vec![vec![0.0; 100]; 100];
        for i in 0..100 {
            dense[i][i] = 1.0;
            if i + 1 < 100 {
                dense[i][i + 1] = 0.5;
            }
        }
        let csr = hisab::CsrMatrix::from_dense(&dense);
        let x: Vec<f64> = (0..100).map(|i| i as f64).collect();
        b.iter(|| csr.spmv(black_box(&x)))
    });

    group.bench_function("svd_4x2_tall", |b| {
        let a = vec![
            vec![1.0, 2.0],
            vec![3.0, 4.0],
            vec![5.0, 6.0],
            vec![7.0, 8.0],
        ];
        b.iter(|| hisab::num::svd(black_box(&a)))
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
    bench_v03,
    bench_v04a,
    bench_v04b,
    bench_v04c,
    bench_v05a,
    bench_v05b,
    bench_v05c,
    bench_v06,
);
criterion_main!(benches);
