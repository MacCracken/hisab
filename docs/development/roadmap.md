# Roadmap

> **Hisab** (Arabic: حساب — calculation, reckoning) — higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius (ported from Rust). Cyrius stdlib linalg.cyr (4.10.3) provides dense decompositions.

## Scope

Hisab owns **typed mathematical operations** — the programmatic math that engines, physics, rendering, and simulation need. It does NOT own:

- **Expression parsing, user-typed math** → abaco (eval engine)
- **Unit conversion** → abaco (unit registry)
- **Physics simulation** → impetus (uses hisab types)
- **Game engine** → kiran (uses hisab for transforms/projections)

## Consumers

| Consumer | What it uses from hisab | Status |
|----------|----------------------|--------|
| **impetus** | Vectors, transforms, GJK/EPA, PGS solver, inertia | Usable |
| **kiran** | Projections, transforms, frustum, BVH, ray tests | Usable |
| **joshua** | ODE (DOPRI45, BDF, symplectic), optimization | Usable |
| **aethersafha** | Projections, transform interpolation, color | Usable |
| **abaco** | Symbolic algebra (integrate, LaTeX, patterns), interval arithmetic | Usable |
| **svara** | Complex, FFT, easing | Usable |
| **hisab-mimamsa** | Tensors, Lie groups, diffgeo, complex LA, CGA | Usable |
| **kana** | Tensors, Lie groups, complex LA, spinors | Usable |

---

## Current — v2.0.0 (2026-04-15)

- **27 lib files, 11,769 lines** (ported from 33,612 lines Rust)
- **821 test assertions** across 4 test suites
- **22 benchmarks**, 5 fuzz targets
- **420KB static binary** (vs ~800KB Rust dynamic)
- Toolchain: Cyrius 4.10.3
- P(-1) audit: 31 issues found, 25 fixed

### Shipped
27 lib files — see CHANGELOG.md for full inventory. P(-1) audit: 25 of 31 issues fixed.
Remaining: C3 (upstream cyrius 5.0.1), M8 (Golub-Kahan SVD), L1-L7 (cosmetic).

---

## 2.1.0 — Precision + depth

### Audit carry-forward
- [ ] M8: SVD via Golub-Kahan bidiagonalization (replaces A^T*A approach)
- [ ] C3: upstream matrix.cyr alloc overflow → cyrius 5.0.1

### Numerical
- [ ] Complex QR decomposition
- [ ] Complex matrix inverse
- [ ] Condition number estimation
- [ ] QR iteration for large symmetric eigenproblems (n > 50)
- [ ] Simplex noise (OpenSimplex2)
- [ ] Einsum string notation parser (`"ij,jk->ik"`)

---

## 2.2.0 — Geometry & group extensions

### Lie group extensions
- [ ] SE(3) — rigid body motions (rotation + translation as single group)
- [ ] SO(3) explicit — rotation group without SU(2) double cover
- [ ] Adjoint representation for all groups
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

- [ ] Parallel transport of vector fields
- [ ] Sectional curvature
- [ ] Geodesic deviation equation
- [ ] Weyl tensor (conformal curvature)
- [ ] Higher-order differential forms (3-forms, 4-forms)

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
Theoretical physics foundation. Archived in `rust-old/`. See CHANGELOG.md for history.

---

## Boundary with Abaco

| Feature | abaco | hisab |
|---------|-------|-------|
| `eval("sin(pi/4)")` | parses and evaluates | — |
| `hvec3_cross(a, b)` | — | vec3.cyr |
| `geo_ray_sphere(ray, sphere)` | — | geo.cyr |
| `calc_integral_simpson(&f, a, b, n, out)` | — | calc.cyr |
| `num_newton(&f, &df, x0, tol, max, out)` | — | num.cyr |
| `sym_integrate(expr, var)` | — | symbolic_ext.cyr |
| `sym_to_latex(expr)` | — | symbolic_ext.cyr |

Hisab should never depend on abaco. Abaco may optionally depend on hisab (num, symbolic) for solver/algebra features.
