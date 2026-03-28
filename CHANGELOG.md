# Changelog

## 1.3.0 (2026-03-27)

### num — Number theory
- `sieve_eratosthenes()`, `sieve_atkin()`, `sieve_segmented()` — prime sieves (Eratosthenes, Atkin, O(√n)-memory segmented)
- `is_prime_u64()` — deterministic Miller-Rabin for all u64 (Jim Sinclair witnesses)
- `is_prime_miller_rabin()` — probabilistic Miller-Rabin with configurable witness count
- `is_prime_baillie_psw()` — Baillie-PSW primality test (no known counterexample)
- `factor_trial_division()`, `pollard_rho()`, `factorize()` — integer factorization (trial division, Pollard's rho, hybrid)
- `modpow()`, `modinv()`, `extended_gcd()` — modular arithmetic with 128-bit intermediates
- `gcd()` — binary GCD algorithm
- `euler_totient()`, `mobius()`, `mertens()`, `divisor_sigma()` — number-theoretic functions
- `continued_fraction_rational()`, `continued_fraction_f64()`, `convergents()` — continued fraction expansion + rational approximants
- `chinese_remainder_theorem()` — CRT solver for pairwise coprime moduli

### symbolic — Advanced simplification
- `simplify_advanced()` — trig identities (sin²+cos²=1, sin(-x)=-sin(x), cos(-x)=cos(x)), log rules (ln(e^x)=x, ln(x^n)=n·ln(x)), power rules ((x^a)^b=x^(a·b), x·x=x²)

### symbolic — Symbolic integration
- `symbolic_integrate()` — indefinite integration for polynomial, trig, exponential, sums, constant multiples, negation, and reciprocal forms

### symbolic — LaTeX rendering
- `to_latex()` — render expressions as LaTeX strings with smart formatting (\\frac, \\sqrt, \\cdot, subtraction, multi-char variable wrapping)

### symbolic — Pattern matching engine
- `Pattern` enum with `Wildcard`, `AnyConst`, structural matching
- `match_expr()`, `instantiate()` — pattern matching and template instantiation
- `apply_rule()`, `rewrite()`, `rewrite_fixpoint()` — single/recursive/fixpoint rewrite rule application
- `RewriteRule` struct for composable expression transformations

### symbolic — abaco bridge API
- `ExprValue` — serializable flat representation for cross-crate transport (serde-enabled)
- `expr_to_value()`, `value_to_expr()` — bidirectional Expr ↔ ExprValue conversion
- `solve_expr()` — equation solver dispatch (Newton-Raphson + bisection fallback with symbolic differentiation)
- `SolveOptions` — configurable initial guess, bracket, tolerance, max iterations
- `eval_verified()` — interval arithmetic evaluation for verified error bounds (requires `interval` feature)

### Stats
- 887 tests (843 unit + 34 integration + 10 doc), zero clippy warnings

## 1.2.0 (2026-03-27)

### transforms — Interpolation utilities
- `inverse_lerp()` — compute parameter `t` from a value in a range
- `remap()` — remap a value from one range to another

### transforms — Reverse-Z projection
- `projection_perspective_reverse_z()` — infinite far-plane reverse-Z projection (modern GPU standard)

### transforms — HSV/HSL color conversion
- `linear_to_hsv()`, `hsv_to_linear()` — HSV color space (hue in radians)
- `linear_to_hsl()`, `hsl_to_linear()` — HSL color space (hue in radians)

### transforms — Premultiplied alpha
- `premultiply_alpha()`, `unpremultiply_alpha()` — straight ↔ premultiplied alpha conversion

### transforms — Transform composition
- `Transform2D::compose()` — chain two 2D transforms (rotation, scale, position composed directly)
- `Transform3D::compose()` — chain two 3D transforms (quaternion multiplication, scale composition)

### geo — Closest point on triangle
- `closest_point_on_triangle()` — 3D Voronoi region test (Ericson algorithm)

### geo — Barycentric coordinates
- `barycentric_coords()` — compute (u, v, w) for a point projected onto a 3D triangle

### geo — Segment-segment distance
- `segment_segment_closest()` — closest points between two 3D line segments + squared distance

### geo — Friction in sequential impulse
- `ImpulseResult` struct with normal + friction impulse vectors
- `sequential_impulse()` now solves tangent-plane Coulomb friction (clamped to friction cone)
- **Breaking**: `sequential_impulse()` returns `ImpulseResult` instead of `Vec<f32>`

### num — Compensated summation
- `kahan_sum()` — Kahan compensated summation (O(1) error vs O(n) naive)
- `neumaier_sum()` — improved Kahan that handles large+small value mixing

### num — SOR for PGS
- `projected_gauss_seidel_sor()` — PGS with configurable relaxation parameter omega
- `projected_gauss_seidel()` now delegates to SOR with omega=1.0

### num — BiCGSTAB iterative solver
- `bicgstab()` — Bi-Conjugate Gradient Stabilized for non-symmetric linear systems

### num — BDF high-order stiff solvers
- `bdf()` — BDF-3, BDF-4, BDF-5 with configurable order, Newton corrector, and bootstrap

### num — Quasi-random sequences
- `halton()`, `halton_2d()` — Halton low-discrepancy sequence (any prime base)
- `sobol()` — Sobol/Van der Corput sequence via Gray code + bit reversal

### num — Sparse spmvt
- `CsrMatrix::spmvt()` — sparse matrix-transpose-vector multiply without forming Aᵀ

### num — Yoshida 4th-order symplectic integrator
- `yoshida4_step()`, `yoshida4()` — triple-jump composition, more accurate than Verlet

### calc — Hermite TCB spline
- `hermite_tcb()` — Kochanek-Bartels spline with tension, continuity, bias parameters

### calc — Monotone cubic interpolation
- `monotone_cubic()` — Fritsch-Carlson method, guarantees no overshoot (ideal for replay)

### transforms — Porter-Duff compositing
- 10 operators: `composite_src_over`, `dst_over`, `src_in`, `dst_in`, `src_out`, `dst_out`, `src_atop`, `dst_atop`, `xor`, `plus` — all premultiplied alpha

### transforms — HDR tone mapping
- `tonemap_reinhard()`, `tonemap_reinhard_extended()` — Reinhard operator with optional white point
- `tonemap_aces()` — ACES filmic curve (Narkowicz approximation)

### transforms — Depth buffer utilities
- `linearize_depth()` — standard NDC to view-space depth
- `linearize_depth_reverse_z()` — reverse-Z NDC to view-space depth

### geo — Tangent space computation
- `compute_tangent()` — per-triangle tangent/bitangent from UV coordinates (Mikktspace-compatible)

### geo — MPR / XenoCollide collision
- `mpr_intersect()` — Minkowski Portal Refinement overlap test (3D)
- `mpr_penetration()` — MPR with penetration normal and depth

### geo — Delaunay triangulation + Voronoi diagrams
- `delaunay_2d()` — Bowyer-Watson incremental Delaunay triangulation
- `voronoi_2d()` — Voronoi diagram as dual of Delaunay (finite edges)
- `DelaunayTriangle`, `Triangulation`, `VoronoiEdge`, `VoronoiDiagram` types

### calc — NURBS evaluation
- `nurbs_eval()` — Non-Uniform Rational B-Spline evaluation via weighted de Boor's algorithm

### num — Sparse factorization
- `sparse_cholesky_solve()` — Cholesky factorization + solve for sparse SPD matrices
- `sparse_lu_solve()` — LU factorization + solve for sparse systems via Gaussian elimination
- `CsrMatrix::get()` — random access to sparse matrix elements via binary search

### Fixed
- EPA 2D winding: enforce CCW polytope orientation before expansion (prevents inverted normals)
- `backward_euler()`, `bdf2()`: emit `tracing::warn!` on Newton non-convergence instead of silent acceptance
- Replaced `unreachable!()` in 2D GJK with safe fallback return
- Rustdoc: escaped `[0,1]` bracket in color.rs, wrapped `Vec<f64>` in backticks in optimize.rs

### Stats
- 786 tests (743 unit + 34 integration + 9 doc), zero clippy warnings

## 1.1.0 (2026-03-25)

### num — Full eigendecomposition
- `eigen_symmetric()` — Jacobi rotation algorithm for all eigenvalues + orthonormal eigenvectors
- `EigenDecomposition` struct

### num — Stiff ODE solvers
- `backward_euler()` — implicit Euler with Newton+LU iteration
- `bdf2()` — second-order backward differentiation formula

### num — Stochastic differential equations
- `Pcg32::next_normal()` — Box-Muller normal distribution
- `euler_maruyama()` — SDE solver (strong order 0.5)
- `milstein()` — SDE solver with Ito correction (strong order 1.0)

### num — Stability analysis
- `lyapunov_max()` — maximal Lyapunov exponent via variational equation

### num — Projected Gauss-Seidel
- `projected_gauss_seidel()` — box-constrained linear solver for physics

### geo — Continuous collision detection
- `swept_aabb()` — expand AABB along velocity
- `time_of_impact()` — conservative advancement TOI for convex shapes

### geo — Constraint solvers
- `ContactConstraint` struct
- `sequential_impulse()` — iterative contact constraint solver

### geo — Convex decomposition
- `TriMesh`, `ConvexDecomposition`, `AcdConfig` types
- `convex_decompose()` — approximate convex decomposition via PCA splitting

### autodiff — Reverse-mode automatic differentiation
- `Tape`, `Var`, `TapeOp` — computation graph with recording
- `tape.backward()` — backpropagation for all gradients in one pass
- `reverse_gradient()` — convenience API for gradient computation
- Operations: add, sub, mul, div, neg, sin, cos, exp, ln, powf

### geo — Signed distance fields

### geo — Signed distance fields
- `sdf_sphere()`, `sdf_box()`, `sdf_capsule()` — analytical SDFs
- `sdf_union()`, `sdf_intersection()`, `sdf_subtraction()`, `sdf_smooth_union()` — CSG operations

### geo — Polygon triangulation
- `triangulate_polygon()` — ear-clipping triangulation for simple polygons

### geo — Ray-quadric + Fresnel
- `ray_quadric()` — general quadric surface intersection (ellipsoid, paraboloid, etc.)
- `refract()` — Snell's law refraction vector
- `fresnel_schlick()`, `fresnel_exact()` — Fresnel reflectance

### geo — Sweep-and-prune broadphase
- `sweep_and_prune()` — SAP broadphase collision detection

### transforms — Dual quaternions
- `DualQuat` — rigid body transform type for blend skinning
- `from_rotation_translation()`, `translation()`, `rotation()`, `to_matrix()`, `transform_point()`, `blend()`

### transforms — CSS transform decomposition
- `decompose_mat4()` — extract translate/rotate/scale from arbitrary 4×4 matrix
- `recompose_mat4()` — reconstruct from components
- `DecomposedTransform` struct

### transforms — Color + Oklab
- `color_matrix_saturation()`, `color_matrix_hue_rotate()` — color matrix operations
- `linear_to_oklab()`, `oklab_to_linear()` — Oklab perceptual color space

### transforms — Spherical harmonics
- `sh_eval_l2()` — evaluate 9 SH basis functions at a direction
- `sh_project_l2()`, `sh_evaluate_l2()` — project and reconstruct from SH coefficients

### num — Inertia tensors
- `inertia_sphere()`, `inertia_box()` — primitive shape inertia tensors
- `inertia_mesh()` — inertia tensor from triangle mesh (divergence theorem)

### num — GMRES iterative solver
- `gmres()` — GMRES(m) for non-symmetric linear systems

### num — Symplectic integrators
- `symplectic_euler()`, `symplectic_euler_step()` — semi-implicit Euler
- `verlet()`, `verlet_step()` — velocity Störmer-Verlet
- `leapfrog_step()` — kick-drift-kick leapfrog

### num — PCG32 random number generator
- `Pcg32` struct — fast, deterministic, seedable PRNG for simulation replay
- `next_u32()`, `next_f64()`, `next_f32()`, `next_f64_range()`

### num — 2D FFT + truncated SVD
- `fft_2d()`, `ifft_2d()` — row-major 2D Fourier transforms
- `truncated_svd()` — top-k singular values/vectors

### transforms — Quaternion utilities
- `quat_from_euler()`, `quat_to_euler()` with `EulerOrder` enum (6 rotation orders)
- `quat_look_at()`, `look_at_rh()` — camera/direction construction

### transforms — Screen-space + color
- `world_to_screen()`, `screen_to_world_ray()` — 3D↔2D projection
- `srgb_to_linear()`, `linear_to_srgb()`, vec3 variants — piecewise sRGB transfer

### geo — Frustum-sphere test
- `Frustum::contains_sphere()` — conservative sphere culling

### calc — Spring dynamics + easing
- `spring_step()` — analytical critically/under/over-damped spring solver
- `cubic_bezier_ease()` — CSS cubic-bezier timing function via Newton-Raphson

### calc — Noise functions
- `perlin_2d()`, `perlin_3d()` — classic Perlin gradient noise
- `fbm_2d()` — fractal Brownian motion with configurable octaves

### symbolic — Substitution
- `Expr::substitute()` — replace variables with sub-expressions

### Refactoring
- Split `num.rs` (6097 lines) into 13 submodules: roots, linalg, eigen, complex, fft, ode, inertia, solvers, stability, optimize, rng, sparse, svd
- Split `geo.rs` (5466 lines) into 7 submodules: primitives, intersection, closest, spatial, collision, sdf, decompose
- Zero API changes — all re-exports preserved

### Stats
- 714 tests (671 unit + 34 integration + 9 doc), zero clippy warnings

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
