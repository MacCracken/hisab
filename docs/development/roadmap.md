# Roadmap

> **Hisab** (Arabic: حساب — calculation, reckoning) — higher mathematics library for the AGNOS ecosystem.
> Ported to Cyrius from Rust. Cyrius stdlib linalg.cyr (4.10.3) provides dense decompositions.

## Scope

Hisab owns **typed mathematical operations** — the programmatic math that engines, physics, rendering, and simulation need. It does NOT own:

- **Expression parsing, user-typed math** → abaco (eval engine)
- **Unit conversion** → abaco (unit registry)
- **Physics simulation** → impetus (wraps rapier, uses hisab types)
- **Game engine** → kiran (uses hisab for transforms/projections)

## Consumers

| Consumer | What it uses from hisab | Cyrius port status |
|----------|----------------------|-------------------|
| **impetus** | Vectors, transforms, GJK/EPA, PGS solver, inertia | Usable |
| **kiran** | Projections, transforms, frustum, BVH, ray tests | Usable |
| **joshua** | ODE (DOPRI45, BDF, symplectic), optimization | Usable |
| **aethersafha** | Projections, transform interpolation, color | Usable |
| **abaco** | Symbolic algebra, interval arithmetic | Partial (no LaTeX/patterns) |
| **svara** | Complex, FFT, easing | Usable |
| **hisab-mimamsa** | Tensors, Lie groups, diffgeo, complex LA, CGA | Usable |
| **kana** | Tensors, Lie groups, complex LA, spinors | Usable |

---

## Current — v2.0.0 (2026-04-15)

- **27 lib files, 11,769 lines** (ported from 33,612 lines Rust)
- **821 test assertions** across 4 test suites
- **22 benchmarks**, 5 fuzz targets
- **349KB static binary** (vs ~800KB Rust dynamic)
- Toolchain: Cyrius 4.10.3, linalg.cyr Tier 1+2
- P(-1) audit: 31 issues found, 15 critical/high fixed

---

## Audit Fixes — In Progress

> From docs/audit/2026-04-15.md. Items checked off are fixed and tested.

### Fixed (this session)
- [x] C1: Integer overflow guard in tensor_new
- [x] C2: Integer overflow guard in cmat_new
- [x] C4: Dim cap (16) in christoffel_symbols/riemann_tensor
- [x] C5: Sieve limit cap (10M)
- [x] H2: m4_determinant rewritten with correct s0-s5/c0-c5 formula
- [x] H3: cx_div/cx_inv zero guards
- [x] H5: world_to_screen w==0 guard
- [x] H7: f64_fmod zero guard
- [x] H10: ivl_sin fixed (no longer always [-1,1])
- [x] External: Bisection midpoint overflow fix
- [x] External: CG upgraded from Fletcher-Reeves to Polak-Ribiere+

### Remaining audit items
- [ ] C3: matrix.cyr integer overflow in alloc — **upstream to cyrius 5.0.1** (mat_new rows*cols*8 can wrap)
- [x] H1: tensor_contract — full multi-index contraction implemented
- [x] H4: dual_div/dual_sqrt/dual_ln zero guards added
- [x] H6: linearize_depth_reverse_z zero guard added
- [x] H8: m4_get/m4_set bounds documented (contract comment)
- [x] H9: BDF-5 coefficients recomputed exact (verified via IEEE 754)
- [x] M1: num_modpow overflow-safe via Russian peasant _num_mulmod
- [x] M2: PCG32 signed shift verified safe (masks discard sign extension)
- [x] M3: geo_ray_plane returns -1 for miss (not 0)
- [x] M7: expr_eval returns 0 with warning instead of aborting
- [ ] M8: SVD via Golub-Kahan (future — replaces A^T*A approach)
- [ ] L1-L7: Low-priority quality items

---

## 2.1.0 — Numerical depth + remaining audit

### Audit completion
- [ ] All remaining H/M items from 2026-04-15 audit
- [ ] tensor_contract full implementation
- [ ] modpow mul-mod for large moduli

### Numerical extensions
- [x] Mat3 type (mat3.cyr — 178 lines)
- [x] Tridiagonal solver (Thomas algorithm — num_ext.cyr)
- [x] Extended number theory: extended_gcd, euler_totient, mobius, divisor_sigma, pollard_rho, factorize, CRT, modinv, continued fractions
- [x] DST, DCT, 2D-FFT (num_ext.cyr)
- [x] Halton, Sobol quasi-random sequences (num_ext.cyr)
- [ ] Complex QR decomposition
- [ ] Complex matrix inverse
- [ ] Condition number estimation (||A|| * ||A^-1||)
- [ ] QR iteration for large symmetric eigenproblems (n > 50)
- [ ] Golub-Kahan SVD (replaces A^T*A for precision)

### Calculus extensions
- [x] B-spline eval, NURBS eval (calc_ext.cyr)
- [x] Hermite TCB, monotone cubic (calc_ext.cyr)
- [x] Adaptive Simpson integration (calc_ext.cyr)
- [x] Partial derivative, gradient, Jacobian, Hessian (calc_ext.cyr)
- [x] 3D Perlin noise + fbm_3d (calc_ext.cyr)
- [ ] Simplex noise

### Symbolic extensions
- [x] Symbolic integration (symbolic_ext.cyr)
- [x] LaTeX rendering (symbolic_ext.cyr)
- [x] Pattern matching + rewrite rules with built-in identities (symbolic_ext.cyr)

### Tensor extensions
- [ ] Einsum string notation
- [ ] Full tensor_contract (multi-index expansion)

---

## 2.2.0 — Geometry & group extensions

### Lie group extensions
- [ ] SE(3) — rigid body motions
- [ ] SO(3) explicit
- [ ] Adjoint representation
- [ ] Baker-Campbell-Hausdorff formula

### Geometry extensions
- [ ] k-d tree, quadtree, octree, spatial hash
- [ ] Delaunay/Voronoi triangulation
- [ ] Half-edge mesh
- [ ] Convex hull 2D, polygon triangulation
- [ ] MPR/XenoCollide
- [ ] Sequential impulse solver with friction
- [ ] Island detection

### CGA extensions
- [ ] Left/right contraction operators
- [ ] Dual operation, blade projection/rejection

---

## 2.3.0 — Differential geometry & curvature

- [ ] Parallel transport
- [ ] Sectional curvature
- [ ] Geodesic deviation equation
- [ ] Weyl tensor
- [ ] Higher-order differential forms

---

## 2.4.0 — Rendering & GPU

- [ ] Differentiable rendering math
- [ ] GPU compute via soorat (feature-gated)
- [ ] Reverse-mode autodiff (Tape)

---

## Release History

### 2.0.0 (2026-04-15) — Cyrius port

Complete rewrite from Rust to Cyrius. 27 lib files, 11,769 lines. 821 test assertions,
22 benchmarks, 5 fuzz targets. P(-1) audit: 31 issues found, 25 fixed. 420KB static binary.
See CHANGELOG.md for full details.

### Rust 1.4.0 (2026-03-30) — Final Rust release
Theoretical physics foundation. Archived in rust-old/. See CHANGELOG.md for history.

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
