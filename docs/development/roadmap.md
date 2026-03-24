# Roadmap

> **Hisab** (Sanskrit: गणित) — higher mathematics library for the AGNOS ecosystem.
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

---

## Completed Milestones

### V0.1 — Foundation (2026-03-22)
- transforms: Transform2D/3D, projections, lerp, glam re-exports (36 tests)
- geo: Ray, Plane, Aabb, Sphere, ray intersection tests (33 tests)
- calc: derivative, trapezoidal/Simpson integration, 2D Bezier (29 tests)
- num: Newton-Raphson, bisection, Gaussian elimination (25 tests)
- ai: daimon/hoosh client (7 tests, feature-gated)
- Infrastructure: flat src/ modules, feature flags, unified HisabError, CI, docs

### V0.2 — Geometry Expansion (2026-03-22)
- Triangle, Line, Segment, Frustum primitives
- Ray-triangle (Möller–Trumbore), plane-plane intersection
- AABB/sphere overlap, closest-point functions
- slerp, transform3d_lerp, inverse_matrix, flip_handedness_z

### V0.3 — Curves & Splines (2026-03-22)
- 3D Bezier, de Casteljau subdivision, Catmull-Rom, B-spline (de Boor)
- Arc-length parameterization, Gauss-Legendre quadrature
- Easing functions (quadratic, cubic, quintic smootherstep)

### V0.4 — Numerical Methods (2026-03-23)
- LU, Cholesky, QR decompositions + least-squares fitting
- Power iteration eigenvalues, Complex type, FFT/IFFT
- RK4 ODE solver with trajectory output

### V0.5 — Spatial Structures & Collision (2026-03-23)
- BVH, k-d tree, Quadtree, Octree, SpatialHash
- 2D convex hull (Andrew's monotone chain)
- GJK collision detection, EPA penetration depth

---

## V1.0 — Hardening & Stability

The codebase has feature completeness for V1.0. This milestone focuses on safety, API quality, and publishability.

### V1.0a — Safety & Error Handling
**Priority: CRITICAL** — Library code must not panic on user input.

- [ ] Replace `assert!` panics with `Result` returns in:
  - `integral_trapezoidal`, `integral_simpson` (n=0)
  - `integral_gauss_legendre` (n=0)
  - `bezier_cubic_3d_arc_length`, `bezier_cubic_3d_param_at_length` (n=0)
  - `fft`, `ifft` (non-power-of-2)
  - `rk4`, `rk4_trajectory` (n=0)
- [ ] Validate `ConvexPolygon::new()` — reject empty vertex list
- [ ] Validate `Sphere::new()` — reject negative radius
- [ ] Validate `Ray::new()` — reject zero-length direction
- [ ] Define epsilon constants: `EPSILON_F32 = 1e-7`, `EPSILON_F64 = 1e-12`
  - Normalize all tolerance checks to use these instead of ad-hoc `1e-8`, `1e-12`, `1e-15`

### V1.0b — API Completeness
**Priority: HIGH** — Fill trait and conversion gaps.

- [ ] `Complex`: add `Div`, `Div<f64>`, `Neg`, `From<f64>`, `From<(f64, f64)>`
- [ ] `Complex`: add `Serialize`/`Deserialize`
- [ ] Add `matrix_determinant()`, `matrix_trace()` helpers
- [ ] Add `matrix_multiply()` for `Vec<Vec<f64>>` (currently only decomposition-based)

### V1.0c — Performance
**Priority: HIGH** — Eliminate unnecessary allocations on hot paths.

- [ ] `rk4` / `rk4_trajectory`: the closure `f` returns `Vec<f64>` per call — 4 allocations per step. Refactor to `f(t, y, out: &mut [f64])` callback style.
- [ ] EPA: `polytope.insert()` is O(n) — switch to a linked-list or circular buffer
- [ ] GJK: `simplex` uses `Vec` with `.remove(0)` (O(n)) — use fixed `[Vec2; 3]` array
- [ ] `convex_hull_2d`: clones input `points.to_vec()` — take `&mut` or document the clone
- [ ] `lu_decompose` / `qr_decompose`: clone input matrices — offer in-place variants

### V1.0d — Documentation
**Priority: MEDIUM** — Required for crates.io publishing.

- [ ] Add `# Examples` doctest to every public function
- [ ] Add `# Panics` section to any remaining panicking functions
- [ ] Add `# Errors` section to all `Result`-returning functions
- [ ] Update `README.md` with V0.2-V0.5 features, new module table
- [ ] Update `CHANGELOG.md` with all milestones
- [ ] Verify `cargo doc --no-deps --all-features` has zero warnings

### V1.0e — Publish
- [ ] Final API review pass — check naming consistency, argument order
- [ ] Run `cargo semver-checks` for compatibility
- [ ] Bump version to 1.0.0 via `scripts/version-bump.sh`
- [ ] Tag and push — CI handles crates.io publish + GitHub Release

---

## V1.1 — Extended Linear Algebra

### num
- [ ] SVD (Singular Value Decomposition)
- [ ] Matrix determinant, trace, rank, condition number
- [ ] Matrix inverse (via LU)
- [ ] Pseudo-inverse for rectangular matrices
- [ ] Sparse matrix support (CSR format)

---

## V1.2 — Multivariable Calculus

### calc
- [ ] `partial_derivative()` — partial derivative of f(x₁, x₂, ..., xₙ)
- [ ] `gradient()` — gradient vector ∇f
- [ ] `jacobian()` — Jacobian matrix of vector function
- [ ] `hessian()` — Hessian matrix (second partial derivatives)
- [ ] `integral_monte_carlo()` — high-dimensional integration
- [ ] `integral_adaptive_simpson()` — automatic refinement

---

## V1.3 — Optimization Solvers

### num (new submodule or feature-gated)
- [ ] Gradient descent / steepest descent
- [ ] Conjugate gradient (CG) iterative solver
- [ ] BFGS / L-BFGS quasi-Newton optimization
- [ ] Levenberg-Marquardt (non-linear least squares)
- [ ] Adaptive RK4/5 (Dormand-Prince) with step-size control

---

## V1.4 — 3D Collision

### geo
- [ ] 3D convex hull (Quickhull algorithm)
- [ ] 3D GJK collision detection
- [ ] 3D EPA penetration depth
- [ ] OBB (Oriented Bounding Box)
- [ ] Capsule primitive (2D and 3D)
- [ ] Mesh-mesh intersection

---

## V2.0 — Advanced

### New modules (feature-gated)
- [ ] **autodiff**: Forward-mode automatic differentiation (dual numbers)
- [ ] **symbolic**: CAS primitives — simplify, expand, factor, symbolic differentiate
- [ ] **interval**: Interval arithmetic for verified numerics
- [ ] **tensor**: N-dimensional array type for ML interop
- [ ] **gpu**: Compute kernels via wgpu (shared with ranga)
- [ ] **parallel**: Rayon integration for batch spatial queries, large FFTs

---

## Known Technical Debt

| Area | Issue | Severity |
|------|-------|----------|
| calc | Integration/FFT functions panic instead of returning Result | Critical |
| geo | ConvexPolygon accepts empty vertex list | High |
| num | rk4 closure allocates Vec per call (4x per step) | High |
| num | Inconsistent epsilon values (1e-8, 1e-12, 1e-15) | Medium |
| geo | GJK/EPA hardcoded to 64 iterations, not configurable | Medium |
| geo | EPA `polytope.insert()` is O(n) per iteration | Medium |
| calc | `convex_hull_2d` clones input unnecessarily | Low |
| all | Zero doctests across all public functions | Medium |
| geo | 3D collision deferred (only 2D GJK/EPA) | Medium |

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
