# Changelog

## [Unreleased]

## 2.2.0 (2026-04-15) -- Geometry & group extensions

### Added
- **lie_ext.cyr** (523 lines) -- SE(3) rigid body motions (exp/log, compose, transform), SO(3) explicit (Rodrigues, exp/log), adjoint representations (SU(2), Lorentz, SE(3)), Baker-Campbell-Hausdorff 2nd/3rd order
- **spatial.cyr** (864 lines) -- k-d tree (build, nearest, within_radius), quadtree (insert, query), octree (insert, query), spatial hash (insert, query_cell, query_radius, clear)
- **collision_core.cyr** (574 lines) -- MPR/XenoCollide (intersect + penetration), sequential impulse solver with friction, convex hull 2D (Andrew's monotone chain), polygon triangulation (ear clipping)
- **collision_mesh.cyr** (522 lines, written but deferred) -- Delaunay triangulation (Bowyer-Watson), half-edge mesh, island detection (union-find). Exceeds cc3 1MB preprocess buffer. Ships with Cyrius 5.0.

### Changed
- All .cyr files cleaned of Unicode in comments (em-dashes, Greek letters -> ASCII)

### Known issue
- Cyrius cc3 1MB preprocess buffer limit reached at ~16K lines. collision_mesh.cyr deferred to Cyrius 5.0.

## 2.1.0 (2026-04-15) -- Precision + depth

### Added
- **linalg_precision.cyr** (1,124 lines) -- Golub-Kahan SVD (full precision, replaces A^T*A), QR eigendecomposition O(n^3) (replaces Jacobi O(n^5)), complex Householder QR
- **noise_simplex.cyr** (343 lines) -- OpenSimplex2 2D+3D with fBm layering
- **einsum.cyr** (305 lines) -- Einstein summation notation parser (`"ij,jk->ik"`)

### Changed
- Audit M8 (SVD precision) resolved

### Performance
- `eigen_qr`: O(n^3) for symmetric eigenproblems (was O(n^5) with Jacobi for large n)

## 2.0.0 (2026-04-15) -- Cyrius port

**Breaking: complete rewrite from Rust to Cyrius.** New language, new API, new binary format. Rust source available via pre-2.0 git tags.

### Changed
- **Language**: Rust -> Cyrius (self-hosting systems language, static ELF binaries)
- **Types**: glam f32 SIMD types -> native f64 heap-allocated types (HVec2/3/4, HQuat, Mat3, Mat4)
- **Errors**: `Result<T, HisabError>` -> integer error codes (ERR_NONE, ERR_SINGULAR_MATRIX, etc.)
- **API**: method syntax (v.dot(w)) -> free functions (hvec3_dot(v, w))
- **Precision**: f32 (1e-7) -> f64 (1e-12) everywhere
- **Dependencies**: 9 Rust crates -> 1 Cyrius dep (sakshi)
- **Binary**: ~800KB dynamic -> 420KB static ELF

### Added -- 27 library files, 11,943 lines

**Foundation (8 files):** error, f64_util, vec2, vec3, vec4, quat, mat3, mat4
**Transforms (2 files):** transforms (T2D/T3D, Euler, screen), color (sRGB, Porter-Duff, tone mapping, SH, EV)
**Geometry (2 files):** geo (9 primitives, 6 ray tests), geo_advanced (GJK/EPA, BVH, SDF, CGA 5D)
**Calculus (2 files):** calc (integration, Bezier, easing, Perlin), calc_ext (gradient/Jacobian/Hessian, B-spline, NURBS, Hermite, monotone cubic, 3D Perlin)
**Numerical (5 files):** num (roots, FFT, RK4, PCG32, primes), ode (DOPRI45, BDF, symplectic), optimize (GD, CG, BFGS, L-BFGS, LM), linalg_ext (CSR, GMRES, BiCGSTAB, PGS, SVD, eigen, inertia), num_ext (extended GCD, totient, Mobius, factorize, CRT, DST/DCT, 2D-FFT, Halton/Sobol, tridiagonal)
**Physics (3 files):** complex (numbers + matrices, Pauli, Dirac, matrix exp), lie (U(1), SU(2), SU(3), SO(3,1)), diffgeo (Christoffel -> Einstein, geodesics, exterior algebra)
**Symbolic (2 files):** symbolic (expr tree, eval, diff, simplify), symbolic_ext (integration, LaTeX, pattern matching)
**Other (3 files):** autodiff (dual numbers), interval (arithmetic), tensor (N-D dense, contraction, physics tensors)

### Security -- P(-1) audit (31 issues found, 25 fixed)
- Allocation overflow guards (tensor, complex matrix, diffgeo dim cap, sieve cap)
- Division-by-zero guards throughout (complex, autodiff, transforms, depth)
- m4_determinant rewritten with correct cofactor formula
- tensor_contract implemented (was returning zeros)
- BDF-5 coefficients recomputed exact (IEEE 754 verified)
- Bisection midpoint overflow fix, CG upgraded to Polak-Ribiere+
- modpow overflow-safe via Russian peasant multiplication
- expr_eval warns instead of aborting process

### Testing
- 821 assertions (4 suites), 22 benchmarks, 5 fuzz targets

### Documentation
- README, CONTRIBUTING, SECURITY updated for Cyrius
- Architecture overview, testing guide, threat model, dependency watch
- P(-1) audit report, Rust vs Cyrius benchmark comparison
- Working example (examples/basic_math.cyr)

---

## Rust era (archived in pre-2.0 git tags)

### 1.4.0 (2026-03-30) -- Theoretical physics foundation
- Complex linear algebra (ComplexMatrix, Hermitian eigen, complex SVD, Pauli/Dirac, spinors, matrix exp)
- Indexed tensor algebra (Einstein summation, contraction, raising/lowering)
- Symmetric/antisymmetric/sparse tensors
- Lie groups (U(1), SU(2), SU(3), SO(3,1), exponential maps, Casimir)
- Differential geometry (Christoffel, Riemann, Ricci, Einstein, geodesics, Killing, exterior algebra)
- Conformal geometric algebra (5D CGA multivectors)

### 1.3.0 (2026-03-27) -- Number theory + symbolic
- Prime sieves, primality, factorization, modular arithmetic
- Symbolic integration, LaTeX, pattern matching, abaco bridge

### 1.2.0 (2026-03-27) -- Interpolation + color
- Inverse lerp, remap, reverse-Z, HSV/HSL/Oklab, Porter-Duff, compensated summation

### 1.1.0 (2026-03-25) -- Feature completion
- Symplectic integrators, SDFs, DualQuat, SH, stiff ODE, SDE, eigen, reverse-mode AD

### 1.0.0 -- Stable release
- Core: transforms, geo, calc, num, autodiff, interval, symbolic, tensor, parallel, ai, logging
