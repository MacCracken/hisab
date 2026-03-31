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
| **impetus** | Vectors, quaternions, transforms, spatial geometry, 3D GJK/EPA broadphase+narrowphase |
| **kiran** | Projections, transforms, frustum culling, camera math, OBB/Capsule ray tests |
| **joshua** | ODE solvers (RK4, DOPRI45), simulation math, deterministic replay |
| **aethersafha** | Projection matrices, transform composition/interpolation for compositor |
| **abaco** | Symbolic algebra (Expr), interval arithmetic for verified evaluation |
| **svara** | Complex, FFT, easing functions (vocal synthesis) |
| **prani** | Easing functions (creature vocal synthesis, via svara) |
| **hisab-mimamsa** | Indexed tensors, Lie groups, differential geometry, complex LA, CGA (theoretical physics) |
| **kana** | Indexed tensors, Lie groups, complex LA, spinors (quantum science) |

## Versioning

Post-1.0: standard semver.

---

## V1.0.0 — Stable Release

### Modules

| Module | Feature | Description |
|--------|---------|-------------|
| **transforms** | `transforms` (default) | Transform2D/3D, projections, lerp, slerp, glam re-exports |
| **geo** | `geo` (default) | Primitives, intersections, BVH, KdTree, Quadtree, Octree, SpatialHash, 2D+3D GJK/EPA, OBB, Capsule |
| **calc** | `calc` (default) | Differentiation, integration (Simpson, Gauss-Legendre, adaptive, Monte Carlo), Bezier, splines, easing, gradient/jacobian/hessian |
| **num** | `num` (default) | Root finding, LU/Cholesky/QR/SVD, FFT/DST/DCT, optimization (GD, CG, BFGS, L-BFGS, LM), ODE (RK4, DOPRI45), sparse matrices |
| **autodiff** | `autodiff` | Forward-mode automatic differentiation (dual numbers) |
| **interval** | `interval` | Interval arithmetic for verified numerics |
| **symbolic** | `symbolic` | Expression tree with evaluation, differentiation, simplification |
| **tensor** | `tensor` | N-dimensional dense tensor type |
| **parallel** | `parallel` | Rayon-powered parallel batch operations |
| **ai** | `ai` | Daimon/hoosh AI client |
| **logging** | `logging` | Structured logging via tracing-subscriber |

### Stats

- 617 tests (574 unit + 34 integration + 9 doc)
- 12 modules, 13 feature flags
- Zero clippy warnings, cargo audit clean, cargo deny clean
- Consumer smoke tests for impetus, kiran, joshua, aethersafha, abaco

### 1.1.0 — All critical + important items shipped (2026-03-25)
- Symplectic integrators, quaternion utils, frustum-sphere, spring dynamics, bezier easing
- Screen projection, sRGB, noise (Perlin/fBm), PCG32, 2D FFT, truncated SVD
- SDFs + CSG, polygon triangulation, ray-quadric, Fresnel, SAP broadphase
- DualQuat, CSS decompose, color matrices, Oklab, spherical harmonics
- Inertia tensors, GMRES, PGS, eigendecomposition, stiff ODE (backward Euler, BDF-2)
- Euler-Maruyama, Milstein SDE, Lyapunov exponents, CCD/TOI, sequential impulse
- Convex decomposition, reverse-mode AD (tape-based)
- Refactored num.rs → 13 submodules, geo.rs → 7 submodules
- 714 tests

### 1.3.0 — Number theory + abaco integration (2026-03-27)
- Prime sieves (Eratosthenes, Atkin, segmented), primality (Miller-Rabin, Baillie-PSW, deterministic u64)
- Integer factorization (trial division, Pollard's rho, hybrid), modular arithmetic (modpow, modinv, extended GCD)
- Number-theoretic functions (totient, Möbius, Mertens, divisor sigma), continued fractions, CRT
- Advanced symbolic simplification (trig, log, power identities)
- Symbolic integration (polynomial, trig, exponential)
- LaTeX rendering, pattern matching/substitution engine
- abaco bridge: ExprValue transport, solver dispatch, verified interval evaluation
- 887 tests

---

## 1.4.0 — Structural depth + quality (shipped in 1.3.0)

### Physics solver completeness
- [x] Constraint warm-starting for sequential impulse — cache impulses across frames (geo, impetus)
- [x] Island detection / contact graph connectivity — union-find for sleeping + parallel solving (geo, impetus)

### Numerical robustness
- [x] Apply compensated summation to ODE solvers and integration routines internally
- [x] Flat `Vec<f64>` matrix layout option for dense linear algebra (cache-friendly alternative to `Vec<Vec<f64>>`)

### Geometry extensions
- [x] Frustum-OBB culling test (geo, kiran)
- [x] Point-in-convex-polygon 2D (geo, kiran)
- [x] AABB-from-transformed-AABB fast path (geo, kiran)
- [x] Triangle mesh adjacency / half-edge structure (geo, kiran)

### Compositor / rendering
- [x] Gamma-aware interpolation — decode→lerp→encode combined (transforms, aethersafha)
- [x] Exposure / EV ↔ luminance conversion for HDR pipelines (transforms, kiran, aethersafha)

### Quality
- [ ] Complete doctests on all public functions
- [ ] GPU compute kernels via soorat (feature-gated compute pipeline)

## 1.5.0 — Theoretical physics foundation (P0 — mimamsa + kana)

### Indexed tensor algebra (tensor)
- [x] Covariant / contravariant index tracking (upper/lower indices)
- [x] Einstein summation convention (automatic contraction)
- [x] Tensor contraction, outer product, index raising/lowering
- [x] Symmetric & antisymmetric tensor storage (exploit symmetries — Riemann 256→20)
- [x] Sparse tensor support for high-rank objects

### Lie groups & algebras (transforms)
- [x] Lorentz group SO(3,1) — boosts, rotations, Lorentz transformations
- [x] SU(2) — spinor representations, Pauli matrices
- [x] SU(3) — Gell-Mann matrices, color charge algebra
- [x] U(1) — phase transformations
- [x] Lie algebra exponential map, commutators, structure constants
- [x] Casimir operators

### Differential geometry (calc/geo)
- [x] Covariant derivative (connection-aware differentiation)
- [x] Christoffel symbols (from metric tensor)
- [x] Levi-Civita connection
- [x] Riemann curvature tensor, Ricci tensor, Ricci scalar
- [x] Geodesic equation solver (integrates with existing ODE solvers)
- [x] Killing vectors, isometry detection
- [x] Exterior algebra — wedge product, Hodge star, differential forms

### Complex linear algebra (num)
- [x] Hermitian matrix eigendecomposition
- [x] Unitary matrix operations
- [x] Pauli & Dirac gamma matrix algebra
- [x] Spinor transformations
- [x] Complex SVD

### Conformal geometric algebra (geo)
- [x] Conformal model (point, sphere, plane representations)
- [x] Conformal transformations (Möbius maps)
- [x] Versors, rotors, translators in conformal space
- 1155 tests (1099 unit + 34 integration + 22 doc)

## Watch List

| Item | Area |
|------|------|
| Randomized SVD (Halko-Martinsson-Tropp) | num |
| Differentiable rendering math | geo/autodiff |
| Neural implicit representation primitives | tensor |
| Low-rank approximations (CUR, Nystrom) | num |

## Boundary with Abaco

| Feature | abaco | hisab |
|---------|-------|-------|
| `eval("sin(pi/4)")` | abaco parses and evaluates | — |
| `Vec3::cross(a, b)` | — | transforms |
| `ray.intersect(sphere)` | — | geo |
| `integral(f, 0, 1)` | — | calc |
| `newton_raphson(f, df, x0)` | — | num |
| `eval("solve x^2 - 2 = 0")` | abaco parses | num solves |

Hisab should never depend on abaco. Abaco may optionally depend on hisab (num, symbolic) for solver/algebra features.
