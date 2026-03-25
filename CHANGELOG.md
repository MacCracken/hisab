# Changelog

## 1.0.0 (2026-03-31)

Stable release. All pre-1.0 milestones complete.

### Final changes
- GJK/EPA iteration limits now configurable via `GJK_MAX_ITERATIONS` and `EPA_MAX_ITERATIONS` constants
- Resolved all known technical debt
- 617 tests (574 unit + 34 integration + 9 doc), zero clippy warnings

## 0.28.3 (2026-03-28)

### parallel — Rayon batch operations (new module)
- `par_transform_points()` — batch 3D transform application
- `par_ray_aabb_batch()`, `par_ray_sphere_batch()` — parallel intersection tests
- `par_matrix_vector_multiply()` — parallel dense matvec
- `par_map()` — parallel element-wise operation
- Feature-gated: `parallel` (requires rayon)

### Doctests
- Added doc examples on key entry points: `Transform2D`, `Ray`, `derivative`, `newton_raphson`, `Dual`, `Interval`, `Expr`, `Tensor`
- 9 doctests passing

### API review
- Added missing `#[must_use]` on `Expr::evaluate()`
- Verified naming consistency, argument order, `#[non_exhaustive]` coverage across all modules

### Audit hardening (0.26.3 code)
- autodiff: Added `Sub<f64>`, `Div<f64>`, `f64 + Dual`, `f64 * Dual` ops
- interval: Made fields private, added `lo()`/`hi()` accessors (invariant protection)
- symbolic: Epsilon-based simplification (handles `-0.0`, near-zero/one correctly)

### Consumer smoke tests (integration)
- impetus: broadphase→narrowphase pipeline, raycast scene query
- kiran: camera frustum culling, transform hierarchy composition
- joshua: ODE deterministic replay, multibody conservation laws
- aethersafha: compositor projection chain, keyframe animation interpolation
- abaco: symbolic differentiation→evaluation pipeline

### Stats
- 617 tests (574 unit + 34 integration + 9 doc), zero clippy warnings

## 0.27.3 (2026-03-27)

### autodiff — Forward-mode automatic differentiation (new module)
- `Dual` type with val/deriv, `var()`, `constant()`
- Arithmetic: Add, Sub, Mul, Div, Neg, scalar ops
- Transcendentals: sin, cos, tan, exp, ln, sqrt, powf, abs
- Feature-gated: `autodiff`

### interval — Interval arithmetic (new module)
- `Interval` type with lo/hi bounds
- Arithmetic: Add, Sub, Mul, Div, Neg
- Operations: contains, overlaps, intersect, hull, width, midpoint, abs, sqr, sqrt
- Feature-gated: `interval`

### symbolic — Symbolic algebra (new module)
- `Expr` enum: Const, Var, Add, Mul, Pow, Neg, Sin, Cos, Exp, Ln
- `evaluate(vars)`, `differentiate(var)`, `simplify()`
- Simplification: 0+x, 1*x, x^0, x^1, double-neg, constant folding
- Feature-gated: `symbolic`

### tensor — N-dimensional tensor (new module)
- `Tensor` type with shape/data, zeros, ones
- get/set, reshape, add, sub, scale, matmul (2D), transpose (2D)
- Feature-gated: `tensor`

### Stats
- 578 tests, zero clippy warnings

## 0.26.3 (2026-03-26)

### num — Optimization solvers
- `gradient_descent()` — steepest descent with fixed learning rate
- `conjugate_gradient()` — iterative SPD linear solver (Ax=b)
- `bfgs()` — quasi-Newton optimizer with backtracking line search
- `levenberg_marquardt()` — nonlinear least squares (damped Gauss-Newton)
- `OptResult` struct for optimization output

### num — Adaptive ODE solver
- `dopri45()` — Dormand-Prince RK4(5) with automatic step-size control

### geo — 3D collision detection
- `ConvexSupport3D` trait, `ConvexHull3D` type
- `gjk_intersect_3d()`, `gjk_epa_3d()`, `Penetration3D`
- `Sphere`, `Obb`, `Capsule` implement `ConvexSupport3D`

### geo — New primitives
- `Obb` — oriented bounding box (center, half_extents, rotation)
  - `contains_point()`, `closest_point()`, `axes()`
  - `ray_obb()` intersection
- `Capsule` — line segment + radius (Minkowski sum)
  - `contains_point()`, `axis_length()`
  - `ray_capsule()` intersection

### Stats
- 504 tests, zero clippy warnings

## 0.25.3 (2026-03-25)

### num — Singular Value Decomposition
- `svd()` — one-sided Jacobi SVD for m×n matrices, returns U, σ, Vᵀ
- `Svd` struct with `u`, `sigma`, `vt` fields

### num — Matrix utilities (built on SVD + LU)
- `matrix_rank()` — numerical rank via singular value thresholding
- `condition_number()` — ratio σ_max/σ_min
- `matrix_inverse()` — full inverse via LU decomposition
- `pseudo_inverse()` — Moore-Penrose pseudo-inverse via SVD

### num — Sparse matrices (CSR)
- `CsrMatrix` — Compressed Sparse Row format
- `from_dense()`, `to_dense()`, `spmv()`, `add()`, `transpose()`
- `nnz()`, `new()` with full validation

### calc — Multivariable calculus
- `partial_derivative()` — central difference on single variable
- `gradient()` — full gradient vector ∇f
- `jacobian()` — m×n Jacobian matrix of vector-valued function
- `hessian()` — n×n Hessian matrix of scalar function

### calc — Advanced integration
- `integral_adaptive_simpson()` — recursive adaptive Simpson with Richardson extrapolation
- `integral_monte_carlo()` — N-dimensional Monte Carlo integration with deterministic LCG

### geo — Edge-case hardening (from P(-1) audit)
- `Plane::from_point_normal()` now returns `Result` (rejects zero-length normals)
- `Segment::direction()` returns fallback instead of NaN on zero-length segments
- `Triangle::unit_normal()` returns fallback instead of NaN on degenerate triangles

### Performance
- `matrix_determinant()` uses `lu_decompose_in_place` (avoids double allocation)
- GJK: deduplicated `gjk_intersect`/`gjk_epa` via shared `gjk_core()` (~60 lines removed)

### Stats
- 466 tests, 89 benchmarks, zero clippy warnings

## 0.24.3 (2026-03-24)

### num — Discrete Sine/Cosine Transforms
- `dst()` — Discrete Sine Transform Type-I (Dirichlet boundary conditions)
- `idst()` — Inverse DST-I (self-inverse with `2/(N+1)` scaling)
- `dct()` — Discrete Cosine Transform Type-II (Neumann boundary conditions)
- `idct()` — Inverse DCT (DCT-III)

### num — Complex API completeness
- `Div`, `Div<f64>`, `Neg`, `From<f64>`, `From<(f64, f64)>` operators
- `Serialize`/`Deserialize` derives

### num — Matrix helpers
- `matrix_determinant()` — via LU decomposition with permutation parity
- `matrix_trace()` — sum of diagonal elements
- `matrix_multiply()` — dense A*B with dimension validation

### geo — Display + Rect parity
- `Display` for `Ray`, `Plane`, `Aabb`, `Sphere`, `Triangle`
- `Rect::merge()`, `Rect::area()`

### transforms
- `Transform2D::inverse_matrix()`

### Safety — Panic elimination
- All `assert!`/`unwrap`/`panic!` in library code replaced with `Result` returns
  - calc: `integral_trapezoidal`, `integral_simpson`, `integral_gauss_legendre`, `bezier_cubic_3d_arc_length`, `bezier_cubic_3d_param_at_length`
  - num: `fft`, `ifft`, `rk4`, `rk4_trajectory`
  - geo: `SpatialHash::new`, `ConvexPolygon::new`, `Sphere::new`, `Ray::new`, `Line::new`

### Quality
- `#[must_use]` on all ~90 pure public functions/methods
- `#[inline]` on 14 hot-path functions
- `EPSILON_F32` (1e-7), `EPSILON_F64` (1e-12) constants; all tolerance checks normalized
- `# Errors` doc sections on all Result-returning public functions
- `cargo doc --all-features` zero warnings
- Removed duplicate `Result<T>` type alias
- License identifier: `GPL-3.0` → `GPL-3.0-only`

### Performance
- `rk4`/`rk4_trajectory`: closure refactored to `f(t, y, out: &mut [f64])` — 4 allocs/step → 0
- GJK: `Vec` simplex → fixed `[Vec2; 3]` array (zero heap allocation)
- EPA: pre-allocated polytope (`Vec::with_capacity(32)`)
- `lu_decompose_in_place()` — zero-clone LU decomposition
- `qr_decompose_in_place()` — zero-clone QR decomposition

### Stats
- 408 tests, 82 benchmarks, zero clippy warnings

## 0.22.3 (2026-03-22)

Initial release — 360 tests, 82 benchmarks.

### transforms
- Vec2/Vec3/Vec4/Mat3/Mat4/Quat re-exports from glam
- `Transform2D`, `Transform3D` with `to_matrix()`, `apply_to_point()`
- Orthographic and perspective projection matrices
- `lerp_f32`, `lerp_vec3`, `slerp`, `transform3d_lerp`
- `Transform3D::inverse_matrix()`, `flip_handedness_z()`

### geo — Primitives & Intersections
- `Ray`, `Plane`, `Aabb`, `Sphere`, `Triangle`, `Line`, `Segment`
- `Frustum` with `contains_point()`, `contains_aabb()`
- Ray-plane, ray-sphere, ray-AABB, ray-triangle intersection
- Plane-plane, AABB-AABB, sphere-sphere overlap
- Closest point on ray, plane, sphere, AABB
- `Rect` with `contains_point()`, `overlaps()`

### geo — Spatial Structures
- `Bvh`, `KdTree`, `Quadtree`, `Octree`, `SpatialHash`

### geo — Collision
- `convex_hull_2d()`, `ConvexSupport` trait, `ConvexPolygon`
- `gjk_intersect()`, `epa_penetration()`, `gjk_epa()`

### calc
- `derivative`, `integral_trapezoidal`, `integral_simpson`
- `integral_gauss_legendre_5`, `integral_gauss_legendre`
- `bezier_quadratic/cubic` (2D/3D), `de_casteljau_split`
- `catmull_rom`, `bspline_eval`, arc-length parameterization
- Easing: `ease_in/out/in_out`, cubic, quintic smootherstep

### num
- `newton_raphson`, `bisection`, `gaussian_elimination`
- `lu_decompose/solve`, `cholesky/solve`, `qr_decompose`
- `least_squares_poly`, `eigenvalue_power`
- `Complex` with arithmetic, `fft`/`ifft`
- `rk4`, `rk4_trajectory`

### Infrastructure
- Feature flags, unified `HisabError`, `DaimonError`
- CI, benchmarks with CSV history, docs
