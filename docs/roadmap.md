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

## V0.2 — Geometry Expansion

### geo
- Triangle primitive with ray-triangle intersection (Moller-Trumbore)
- Line and Segment types
- Plane-plane intersection (line)
- AABB-AABB overlap test
- Sphere-sphere overlap test
- Closest point on ray/line/segment/plane/sphere
- Frustum struct (6 planes) with contains_point(), contains_aabb()
- Oriented Bounding Box (OBB)

### transforms
- Dual quaternion support (for rigid body transforms)
- Transform interpolation (slerp for rotation, lerp for position)
- Coordinate system conversions (left-hand/right-hand)

## V0.3 — Curves & Splines

### calc
- Cubic Bezier curves in 3D
- B-spline evaluation (arbitrary degree)
- Catmull-Rom splines
- Arc-length parameterization
- Curve subdivision (de Casteljau)
- Numerical integration: Gauss-Legendre quadrature

### transforms
- Easing functions (ease-in, ease-out, ease-in-out for common curves)

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
