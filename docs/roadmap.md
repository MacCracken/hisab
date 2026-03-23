# Roadmap

> **Ganit** is the higher mathematics library. Basic expression evaluation and unit conversion lives in [abaco](https://github.com/MacCracken/abaco).

## Scope

Ganit owns **typed mathematical operations** — the programmatic math that engines, physics, rendering, and simulation need. It does NOT own:

- **Expression parsing, user-typed math** → abaco (eval engine)
- **Unit conversion** → abaco (unit registry)
- **Physics simulation** → impetus (wraps rapier, uses ganit types)
- **Game engine** → kiran (uses ganit for transforms/projections)

## Consumers

| Consumer | What it uses from ganit |
|----------|----------------------|
| **impetus** | Vectors, quaternions, transforms (via glam re-exports). Future: spatial geometry for broadphase |
| **kiran** | Projections, transforms, camera math. Scene entity positioning |
| **joshua** | Simulation math, deterministic replay state serialization |
| **aethersafha** | Projection matrices, transform composition for compositor |
| **abaco** | Future: symbolic algebra backend for advanced expression evaluation |
| **abacus** | Indirect via abaco |

## V0.1 — Foundation (done, 2026-03-22)

### transforms (36 tests)
- glam re-exports: Vec2, Vec3, Vec4, Mat3, Mat4, Quat
- Transform2D (position, rotation, scale) with to_matrix(), apply_to_point()
- Transform3D (position, rotation, scale) with to_matrix(), apply_to_point()
- Orthographic and perspective projection matrices
- Scalar and vector lerp

### geo (33 tests)
- Primitives: Ray, Plane, Aabb, Sphere
- Ray-plane intersection
- Ray-sphere intersection (quadratic formula)
- Ray-AABB intersection (slab method)
- Signed distance, containment queries

### calc (29 tests)
- Central difference numerical derivative
- Trapezoidal rule integration
- Simpson's rule integration
- Linear interpolation
- Quadratic and cubic Bezier curves (2D)

### num (25 tests)
- Newton-Raphson root finding
- Bisection root finding
- Gaussian elimination with partial pivoting

### ai (7 tests, feature-gated)
- Daimon/hoosh client

### Infrastructure
- Flat src/ module structure with feature flags
- Unified GanitError with #[non_exhaustive]
- Criterion benchmarks with CSV history tracking
- CI pipeline: fmt, clippy, test, security audit, supply chain, MSRV 1.89, coverage
- Release pipeline: multi-platform build, crates.io publish, GitHub Release

## V0.2 — Geometry Expansion (done, 2026-03-22)

### geo (new types + ~50 tests)
- Triangle primitive with `normal()`, `area()`, `centroid()`
- Ray-triangle intersection (Möller–Trumbore algorithm)
- Line type with `closest_point()`, `distance_to_point()`
- Segment type with `length()`, `midpoint()`, `closest_point()`, `distance_to_point()`
- Frustum (6 planes) from VP matrix with `contains_point()`, `contains_aabb()`
- Plane-plane intersection → Line
- AABB-AABB overlap test (SIMD via `cmple`)
- Sphere-sphere overlap test
- Closest point on ray, plane, sphere, AABB

### transforms (~10 tests)
- `slerp()` — quaternion spherical interpolation
- `transform3d_lerp()` — interpolate position/scale (lerp) + rotation (slerp)
- `Transform3D::inverse_matrix()` — inverse via Mat4
- `flip_handedness_z()` — LH↔RH coordinate system conversion

### Deferred to V0.2.1
- Oriented Bounding Box (OBB)
- Dual quaternion support

## V0.3 — Curves & Splines (done, 2026-03-22)

### calc (~25 tests)
- `bezier_quadratic_3d()`, `bezier_cubic_3d()` — 3D Bezier evaluation
- `de_casteljau_split()` — curve subdivision with left/right sub-curves
- `catmull_rom()` — Catmull-Rom spline segment evaluation
- `bspline_eval()` — arbitrary-degree B-spline via de Boor's algorithm
- `bezier_cubic_3d_arc_length()` — approximate arc length
- `bezier_cubic_3d_param_at_length()` — arc-length re-parameterization
- `integral_gauss_legendre_5()`, `integral_gauss_legendre()` — 5-point Gauss-Legendre quadrature (single + composite)
- `ease_in()`, `ease_out()`, `ease_in_out()` — quadratic easing
- `ease_in_cubic()`, `ease_out_cubic()` — cubic easing
- `ease_in_out_smooth()` — C2 quintic smootherstep

## V0.4 — Numerical Methods Expansion

### num
- LU decomposition with forward/back substitution
- Cholesky decomposition (symmetric positive-definite)
- QR decomposition (Gram-Schmidt)
- Eigenvalue computation (power iteration)
- Basic FFT (Cooley-Tukey radix-2)
- Inverse FFT
- Least squares fitting (linear, polynomial)
- Runge-Kutta ODE solver (RK4)

## V0.5 — Spatial Structures

### geo
- BVH (Bounding Volume Hierarchy) construction and traversal
- K-d tree for point queries
- Grid-based spatial hashing
- Octree (3D) / Quadtree (2D)
- Convex hull computation (2D and 3D)
- GJK collision detection (convex-convex)
- EPA penetration depth

## V1.0 — Stable API

- API review and stabilization
- Comprehensive benchmarks (criterion)
- SIMD optimizations where glam doesn't already provide them
- Documentation with examples for every public function
- Publish to crates.io
- Feature gates for optional heavy modules (fft, spatial, bvh)

## Post-V1 — Advanced

- Symbolic algebra engine (CAS primitives — simplify, expand, factor, differentiate)
- Interval arithmetic
- Automatic differentiation (forward mode)
- Tensor types (for ML interop)
- GPU compute kernels via wgpu (shared with ranga)

## Boundary with Abaco

| Feature | abaco | ganit |
|---------|-------|-------|
| `eval("sin(pi/4)")` | abaco parses and evaluates | — |
| `Vec3::cross(a, b)` | — | transforms |
| `ray.intersect(sphere)` | — | geo |
| `integral(f, 0, 1)` | — | calc |
| `newton_raphson(f, df, x0)` | — | num |
| `eval("solve x^2 - 2 = 0")` | abaco parses | num solves |

Ganit should never depend on abaco. Abaco may optionally depend on ganit (num) for symbolic/solver features in the future.
