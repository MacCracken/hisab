# Roadmap

> **Hisab** (Arabic: حساب — calculation, reckoning) — higher mathematics library for the AGNOS ecosystem.
> Basic expression evaluation and unit conversion lives in [abaco](https://github.com/MacCracken/abaco).

## Scope

Hisab owns **typed mathematical operations** — the programmatic math that engines, physics, rendering, and simulation need. It does NOT own:

- **Expression parsing, user-typed math** → abaco (eval engine)
- **Unit conversion** → abaco (unit registry)
- **Physics simulation** → impetus (wraps rapier, uses hisab types)
- **Game engine** → kiran (uses hisab for transforms/projections)

## Consumers

| Consumer | What it uses from hisab |
|----------|----------------------|
| **impetus** | Vectors, quaternions, transforms, spatial geometry for broadphase |
| **kiran** | Projections, transforms, frustum culling, camera math |
| **joshua** | ODE solvers, simulation math, deterministic replay |
| **aethersafha** | Projection matrices, transform composition for compositor |
| **abaco** | Future: symbolic algebra backend for advanced expression evaluation |

## Versioning

Pre-1.0 releases follow **calver/semver hybrid**: `0.DD.M` (day.month).
Post-1.0 releases follow standard semver.

---

## Completed Milestones

### 0.22.3 — Foundation + Geometry + Curves + Numerical (2026-03-22)
- transforms: Transform2D/3D, projections, lerp, slerp, inverse_matrix, flip_handedness_z
- geo: Ray, Plane, Aabb, Sphere, Triangle, Line, Segment, Frustum, Rect
- geo: Ray-plane/sphere/AABB/triangle intersection, closest-point functions
- geo: BVH, KdTree, Quadtree, Octree, SpatialHash
- geo: 2D convex hull, GJK/EPA collision detection
- calc: derivative, trapezoidal/Simpson/Gauss-Legendre integration, 2D/3D Bezier
- calc: de Casteljau, Catmull-Rom, B-spline, arc-length, easing functions
- num: Newton-Raphson, bisection, Gaussian elimination, LU/Cholesky/QR
- num: least-squares fitting, eigenvalues, Complex, FFT/IFFT, RK4
- ai: daimon/hoosh client (feature-gated)
- Infrastructure: feature flags, unified HisabError, CI, 360 tests, 82 benchmarks

### 0.24.3 — DST/DCT, Display, Rect parity, Hardening, Complex API (2026-03-24)
- num: DST-I/IDST-I, DCT-II/IDCT
- geo: Display for Ray, Plane, Aabb, Sphere, Triangle
- geo: Rect::merge(), Rect::area()
- transforms: Transform2D::inverse_matrix()
- **Audit — Safety**: All `assert!`/`unwrap`/`panic!` replaced with `Result` returns
- **Audit — Quality**: `#[must_use]` on all ~90 pure public fns, `#[inline]` on 14 hot-path fns
- **API**: Complex gains `Div`, `Div<f64>`, `Neg`, `From<f64>`, `From<(f64,f64)>`, Serialize/Deserialize
- **Helpers**: `matrix_determinant()`, `matrix_trace()`, `matrix_multiply()`
- **Epsilon**: `EPSILON_F32`, `EPSILON_F64` constants; all tolerance checks normalized
- **Docs**: `# Errors` sections on all Result-returning fns, zero `cargo doc` warnings
- **Perf**: RK4 closure refactored to `f(t, y, out: &mut [f64])` — 4 allocs/step → 0
- **Perf**: GJK simplex `Vec` → fixed `[Vec2; 3]` array (no heap), EPA pre-allocated polytope
- **Perf**: `lu_decompose_in_place()`, `qr_decompose_in_place()` — zero-clone variants
- **Perf**: `convex_hull_2d` clone documented
- License `GPL-3.0` → `GPL-3.0-only`, removed duplicate Result alias
- 408 tests, 82 benchmarks

### 0.25.3 — Extended Linear Algebra + Multivariable Calculus (2026-03-25)
- SVD (one-sided Jacobi, m×n), Svd struct
- matrix_rank, condition_number, matrix_inverse (LU), pseudo_inverse (SVD)
- CsrMatrix: CSR sparse format with from_dense, to_dense, spmv, add, transpose
- partial_derivative, gradient, jacobian, hessian (finite differences)
- integral_adaptive_simpson (recursive Richardson), integral_monte_carlo (N-D LCG)
- P(-1) audit: Plane::from_point_normal → Result, NaN-safe Segment::direction/Triangle::unit_normal
- GJK deduplication, matrix_determinant in-place optimization
- 466 tests, 89 benchmarks

### 0.26.3 — Optimization Solvers + 3D Collision (2026-03-26)
- gradient_descent, conjugate_gradient, bfgs, levenberg_marquardt, OptResult
- dopri45: Dormand-Prince RK4(5) adaptive step-size ODE integrator
- 3D GJK/EPA: ConvexSupport3D trait, ConvexHull3D, gjk_intersect_3d, gjk_epa_3d, Penetration3D
- Obb: oriented bounding box with contains_point, closest_point, ray_obb
- Capsule: segment+radius with contains_point, ray_capsule
- Sphere/Obb/Capsule implement ConvexSupport3D for 3D collision
- 504 tests

### 0.27.3 — Autodiff, Interval, Symbolic, Tensor (2026-03-27)
- autodiff: Dual type, forward-mode AD, sin/cos/tan/exp/ln/sqrt/powf/abs
- interval: Interval type, verified arithmetic, contains/overlaps/intersect/hull
- symbolic: Expr tree, evaluate, differentiate, simplify (constant folding, identity rules)
- tensor: N-D Tensor, zeros/ones, get/set, reshape, add/sub/scale, matmul, transpose
- 578 tests

### 0.28.3 — Parallelism, Doctests, Pre-publish Polish (2026-03-28)
- parallel: par_transform_points, par_ray_aabb_batch, par_ray_sphere_batch, par_matrix_vector_multiply, par_map (rayon)
- Doctests on all key entry points (9 doc tests)
- API review: naming consistency verified, missing #[must_use] added
- Audit: autodiff scalar ops, interval invariant protection, symbolic epsilon simplification
- 593 tests (574 unit + 10 integration + 9 doc)
- GPU deferred to post-1.0

---

## V1 — Stable Release (2026-03-31)

- [ ] Bump version to 1.0.0 via `scripts/version-bump.sh`
- [ ] Tag and push — CI handles crates.io publish + GitHub Release
- [ ] Announce to AGNOS consumers (impetus, kiran, joshua, aethersafha)

---

## Known Technical Debt

| Area | Issue | Severity | Target |
|------|-------|----------|--------|
| geo | GJK/EPA hardcoded to 64 iterations, not configurable | Low | 0.26.3 |
| geo | 3D collision deferred (only 2D GJK/EPA) | Medium | 0.26.3 |

## Boundary with Abaco

| Feature | abaco | hisab |
|---------|-------|-------|
| `eval("sin(pi/4)")` | abaco parses and evaluates | — |
| `Vec3::cross(a, b)` | — | transforms |
| `ray.intersect(sphere)` | — | geo |
| `integral(f, 0, 1)` | — | calc |
| `newton_raphson(f, df, x0)` | — | num |
| `eval("solve x^2 - 2 = 0")` | abaco parses | num solves |

Hisab should never depend on abaco. Abaco may optionally depend on hisab (num) for symbolic/solver features in the future.
