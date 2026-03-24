# Changelog

## 0.24.3 (2026-03-24)

### num — Discrete Sine/Cosine Transforms
- `dst()` — Discrete Sine Transform Type-I (Dirichlet boundary conditions)
- `idst()` — Inverse DST-I (self-inverse with `2/(N+1)` scaling)
- `dct()` — Discrete Cosine Transform Type-II (Neumann boundary conditions)
- `idct()` — Inverse DCT (DCT-III)
- All return `Result` with error on empty input
- 12 tests: roundtrips, known values, single-element, empty-input errors, large-N
- 4 benchmarks: `dst_64`, `dct_64`, `dst_idst_256`, `dct_idct_256`

### geo — Display impls
- `Display` for `Ray`, `Plane`, `Aabb`, `Sphere`, `Triangle`

### geo — Rect parity
- `Rect::merge()` — merge two rectangles into enclosing rect
- `Rect::area()` — rectangle area

### transforms
- `Transform2D::inverse_matrix()` — inverse 3x3 matrix (parity with Transform3D)

## 0.22.3 (2026-03-23)

Initial release — 360 tests, 82 benchmarks.

### transforms
- Vec2/Vec3/Vec4/Mat3/Mat4/Quat re-exports from glam
- `Transform2D`, `Transform3D` with `to_matrix()`, `apply_to_point()` (optimized: inline math, no matrix construction)
- Orthographic and perspective projection matrices
- `lerp_f32`, `lerp_vec3`, `slerp`, `transform3d_lerp`
- `Transform3D::inverse_matrix()`, `flip_handedness_z()`

### geo — Primitives & Intersections
- `Ray`, `Plane`, `Aabb`, `Sphere`, `Triangle`, `Line`, `Segment`
- `Frustum` (6 planes from VP matrix) with `contains_point()`, `contains_aabb()`
- Ray-plane, ray-sphere (half-b optimized), ray-AABB (slab method), ray-triangle (Möller–Trumbore)
- Plane-plane intersection, AABB-AABB overlap, sphere-sphere overlap
- Closest point on ray, plane, sphere, AABB
- `Rect` (2D AABB) with `contains_point()`, `overlaps()`

### geo — Spatial Structures
- `Bvh` — Bounding Volume Hierarchy (midpoint split, O(n) partition) with `query_ray()`, `query_aabb()`
- `KdTree` — 3D k-d tree (`select_nth_unstable` median) with `nearest()`, `within_radius()`
- `Quadtree` — 2D spatial index with configurable split/depth
- `Octree` — 3D spatial index
- `SpatialHash` — uniform hash grid with O(1) cell lookup

### geo — Collision
- `convex_hull_2d()` — Andrew's monotone chain, O(n log n)
- `ConvexSupport` trait, `ConvexPolygon`
- `gjk_intersect()` — GJK 2D collision detection
- `epa_penetration()` — EPA penetration depth/normal
- `gjk_epa()` — combined GJK + EPA

### calc — Differentiation & Integration
- Central-difference `derivative`
- `integral_trapezoidal`, `integral_simpson` (unrolled pair processing)
- `integral_gauss_legendre_5`, `integral_gauss_legendre` (composite 5-point)
- `lerp` (f64)

### calc — Curves & Splines
- `bezier_quadratic`, `bezier_cubic` (2D), `bezier_quadratic_3d`, `bezier_cubic_3d` (3D)
- `de_casteljau_split` — curve subdivision
- `catmull_rom` — spline segment evaluation
- `bspline_eval` — arbitrary-degree via de Boor's algorithm (stack-optimized for degree ≤ 4)
- `bezier_cubic_3d_arc_length`, `bezier_cubic_3d_param_at_length` (cumulative table + binary search)

### calc — Easing
- `ease_in`, `ease_out`, `ease_in_out` (quadratic)
- `ease_in_cubic`, `ease_out_cubic`
- `ease_in_out_smooth` (C2 quintic smootherstep)

### num — Root Finding & Linear Solvers
- `newton_raphson`, `bisection`
- `gaussian_elimination` with partial pivoting
- `lu_decompose`, `lu_solve` — LU with partial pivoting
- `cholesky`, `cholesky_solve` — for SPD matrices
- `qr_decompose` — modified Gram-Schmidt
- `least_squares_poly` — polynomial fitting via QR

### num — Spectral & Dynamics
- `eigenvalue_power` — dominant eigenvalue via power iteration
- `Complex` type with `+`, `-`, `*`, `abs`, `conj`, `Display`, `Default`
- `fft` — in-place Cooley-Tukey radix-2
- `ifft` — inverse FFT

### num — ODE Solvers
- `rk4` — classic 4th-order Runge-Kutta (allocation-optimized with reused scratch buffer)
- `rk4_trajectory` — full trajectory output

### Infrastructure
- Flat `src/` module structure with feature flags
- Unified `HisabError` with `#[non_exhaustive]`, `DaimonError` (AI feature-gated)
- 82 criterion benchmarks with CSV history tracking and 3-point trend markdown
- CI: fmt, clippy, test, security audit, supply chain, MSRV 1.89, coverage
- Release: multi-platform build, version verification, crates.io publish, GitHub Release
- Documentation: README, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, codecov
