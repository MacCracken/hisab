# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: 4.10.3. Stdlib linalg.cyr provides dense decompositions.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.2.0

- **33 lib files, 15,676 lines**
- **821 test assertions**, 22 benchmarks, 5 fuzz targets
- **511KB static binary**
- P(-1) audit: 26/31 fixed. C3 awaiting cyrius 5.0.1.

---

## 2.3.0 -- Mesh + buffer expansion (blocked on Cyrius 5.0)

Cyrius 5.0 expands the 1MB preprocess buffer. These are written and ready to ship.

- [ ] collision_mesh.cyr -- Delaunay triangulation (Bowyer-Watson), half-edge mesh, island detection (union-find). 522 lines, ready.
- [ ] C3 upstream fix -- matrix.cyr alloc overflow (cyrius 5.0.1)
- [ ] CGA left/right contraction operators
- [ ] CGA dual operation, blade projection/rejection

---

## 2.4.0 -- Differential geometry depth

- [ ] Parallel transport of vector fields along curves
- [ ] Sectional curvature computation
- [ ] Geodesic deviation equation
- [ ] Weyl tensor (conformal curvature)
- [ ] Higher-order differential forms (3-forms, 4-forms)

---

## 2.5.0 -- Rendering, GPU, reverse-mode AD

- [ ] Differentiable rendering math (autodiff through ray-surface intersections)
- [ ] Reverse-mode autodiff (Tape-based)
- [ ] GPU compute via soorat (feature-gated)

---

## Consumers

| Consumer | Status |
|----------|--------|
| **impetus** (physics) | Usable -- GJK/EPA, PGS, inertia, spatial |
| **kiran** (engine) | Usable -- projections, BVH, k-d tree, frustum |
| **joshua** (simulation) | Usable -- DOPRI45, BDF, symplectic, optimize |
| **aethersafha** (compositor) | Usable -- projections, compositing, color |
| **abaco** (expression eval) | Usable -- symbolic integrate/LaTeX/patterns, interval |
| **svara** (vocal synthesis) | Usable -- complex, FFT, easing |
| **hisab-mimamsa** (physics) | Usable -- tensors, Lie groups, diffgeo, CGA |
| **kana** (quantum) | Usable -- tensors, Lie groups, complex LA, spinors |

---

## Release History

| Version | Date | Lines | Files | Highlights |
|---------|------|-------|-------|-----------|
| 2.2.0 | 2026-04-15 | 15,676 | 33 | SE(3), SO(3), adjoint, BCH, spatial structures, MPR, impulse solver, simplex noise, einsum, Golub-Kahan SVD |
| 2.1.0 | 2026-04-15 | 13,715 | 30 | Golub-Kahan SVD, QR eigen, complex QR, simplex noise, einsum |
| 2.0.0 | 2026-04-15 | 11,943 | 27 | Cyrius port from Rust. P(-1) audit. |
| Rust 1.4.0 | 2026-03-30 | 33,612 | 65 | Final Rust release. Archived in rust-old/. |

---

## Boundary with Abaco

| Feature | abaco | hisab |
|---------|-------|-------|
| `eval("sin(pi/4)")` | parses and evaluates | -- |
| `hvec3_cross(a, b)` | -- | vec3.cyr |
| `geo_ray_sphere(ray, sphere)` | -- | geo.cyr |
| `calc_integral_simpson(&f, a, b, n, out)` | -- | calc.cyr |
| `num_newton(&f, &df, x0, tol, max, out)` | -- | num.cyr |
| `sym_integrate(expr, var)` | -- | symbolic_ext.cyr |
| `sym_to_latex(expr)` | -- | symbolic_ext.cyr |

Hisab should never depend on abaco. Abaco may optionally depend on hisab.
