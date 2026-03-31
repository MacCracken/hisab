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

## Current — v1.4.0 (2026-03-30)

- 1155 tests (1099 unit + 34 integration + 22 doc)
- 15 modules, 13 feature flags
- Zero clippy warnings, cargo audit clean, cargo deny clean

---

## Projected Work

### 1.5.0 — Numerical & tensor depth

#### Numerical extensions (num)
- [ ] Complex QR decomposition — completes complex LA alongside Hermitian eigen + SVD
- [ ] Complex matrix inverse — needed by mimamsa for propagator calculations
- [ ] Randomized SVD (Halko-Martinsson-Tropp) — large-scale low-rank approximation
- [ ] Low-rank approximations (CUR, Nyström) — matrix compression

#### Tensor algebra depth (tensor)
- [ ] Einsum string notation parser — `"ij,jk->ik"` style contraction specification
- [ ] Tensor trace (contract all paired indices)
- [ ] Symmetric/antisymmetric decomposition of arbitrary tensors
- [ ] Strided views — zero-copy slicing and transposition

### 1.6.0 — Geometry & group extensions

#### Lie group extensions (transforms)
- [ ] SE(3) — rigid body motions (rotation + translation as single group)
- [ ] SO(3) explicit — rotation group without SU(2) double cover overhead
- [ ] Adjoint representation for all implemented groups
- [ ] Baker-Campbell-Hausdorff formula for Lie algebra composition

#### Conformal geometric algebra extensions (geo)
- [ ] Left/right contraction operators
- [ ] Dual operation (pseudoscalar complement)
- [ ] Blade projection and rejection
- [ ] Circle and line pair representations
- [ ] Conformal point pair and flat point extraction
- [ ] Intersection of geometric objects via outer product null space

### 1.7.0 — Differential geometry & curvature

#### Differential geometry extensions (calc)
- [ ] Parallel transport of vector fields along curves
- [ ] Sectional curvature computation
- [ ] Geodesic deviation equation
- [ ] Weyl tensor (conformal curvature)
- [ ] Higher-order differential forms (3-forms, 4-forms, general p-form wedge)

### 1.8.0 — Rendering & GPU

#### Rendering & graphics (geo/autodiff)
- [ ] Differentiable rendering math — autodiff through ray-surface intersections
- [ ] Neural implicit representation primitives — SDF network evaluation helpers

#### GPU compute
- [ ] GPU compute kernels via soorat (feature-gated compute pipeline)

### Ongoing — quality & consumer-driven

- [ ] Complete doctests on all public functions
- Items added here when consumers (mimamsa, kana, impetus, kiran, etc.) request them

---

## Release History

### 1.4.0 (2026-03-30) — Theoretical physics foundation
- Complex linear algebra: ComplexMatrix, Hermitian eigen, complex SVD, Pauli/Dirac matrices, spinor transforms, matrix exponential
- Indexed tensor algebra: covariant/contravariant indices, Einstein summation, contraction, outer product, raising/lowering, Minkowski metric, Levi-Civita symbol
- Symmetric & antisymmetric tensor storage, sparse tensors (COO format)
- Lie groups: U(1), SU(2), SU(3) Gell-Mann, SO(3,1) Lorentz, exponential maps, Casimir operators
- Differential geometry: Christoffel symbols, Riemann/Ricci/Einstein tensors, geodesic RK4, Killing vectors, exterior algebra
- Conformal geometric algebra: 5D multivectors, geometric/outer/inner products, versors (translator, rotor, dilator)

### 1.3.0 (2026-03-27) — Number theory + abaco integration
- Prime sieves, primality tests, factorization, modular arithmetic, number-theoretic functions
- Symbolic integration, LaTeX rendering, pattern matching engine, abaco bridge

### 1.1.0 (2026-03-25) — Feature completion
- Symplectic integrators, SDFs+CSG, DualQuat, color spaces, spherical harmonics
- Stiff ODE (BDF), SDE solvers, eigendecomposition, convex decomposition, reverse-mode AD

### 1.0.0 — Stable release
- Core modules: transforms, geo, calc, num, autodiff, interval, symbolic, tensor, parallel, ai, logging

---

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
