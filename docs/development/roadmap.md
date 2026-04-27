# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **5.7.10**. Stdlib `linalg.cyr` provides dense decompositions.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.2.2

- **33 lib files, 15,676 lines**
- **825 test assertions**, 22 benchmarks, 5 fuzz targets
- **CLI smoke binary** ~140 KB static ELF
- **`dist/hisab.cyr` distlib bundle** ~544 KB / 16,200 lines (all **34 modules**) — fits cc5 5.7.10's 1 MB input_buf with ~480 KB headroom
- P(-1) audit: 26/31 fixed

---

## 2.3.0 -- Collision module audit (algorithmic correctness)

`collision_core.cyr` and `collision_mesh.cyr` now compile and link, but
the algorithms themselves carry pre-existing bugs from the 2.2.0 port —
they were added back then but never actually exercised because they sat
outside the build chain (orphan-include-after-syscall trick in the old
`src/main.cyr`). 2.2.2 only smoke-tests the API surface (`contact_new` +
`ColContact_*` accessors, `detect_islands`); the heavy algorithms need a
correctness pass.

- [ ] **`convex_hull_2d`** — `vec: index < 0` runtime bounds check on a 5-point input (square + interior point). Insertion-sort + monotone-chain logic needs review
- [ ] **`triangulate_polygon`** (ear clipping) — likely similar boundary issues; not exercised yet
- [ ] **`mpr_intersect`** + **`mpr_penetration`** — XenoCollide / Minkowski Portal Refinement in 3D; correctness against known test fixtures (sphere-sphere, OBB-OBB)
- [ ] **`sequential_impulse`** + **`solve_pgs`** — projected Gauss-Seidel solver for contact constraints; verify convergence + restitution behavior
- [ ] **`delaunay_2d`** (Bowyer-Watson) — needs a numerical-stability pass alongside basic correctness fixtures
- [ ] **`halfedge_from_triangles`** + `halfedge_adjacent_faces` + `halfedge_is_boundary` — half-edge mesh accessors; check twin-pointer wiring
- [ ] Add coverage to `tests/modules.tcyr` as each algorithm is fixed

## 2.3.x -- CGA + matrix overflow guards

- [ ] CGA left/right contraction operators
- [ ] CGA dual operation, blade projection/rejection
- [ ] Confirm `lib/matrix.cyr` `mat_new(rows, cols)` overflow is fixed upstream (was C3 in P(-1) audit; expected fixed by cyrius 5.x)

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
| Rust 1.4.0 | 2026-03-30 | 33,612 | 65 | Final Rust release. Available via pre-2.0 git tags. |

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
