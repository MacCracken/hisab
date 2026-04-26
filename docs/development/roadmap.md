# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **5.7.8**. Stdlib `linalg.cyr` provides dense decompositions.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.2.2

- **33 lib files, 15,676 lines**
- **821 test assertions**, 22 benchmarks, 5 fuzz targets
- **CLI smoke binary** ~140 KB static ELF
- **`dist/hisab.cyr` distlib bundle** ~505 KB (32 modules) — fits cc5 5.7.8's 512 KB input_buf
- P(-1) audit: 26/31 fixed.

---

## 2.3.0 -- Restore collision modules (gated on cc5 5.7.9)

cc5 5.7.9 raises `input_buf` from 512 KB → 1 MB (announced in the cc5
5.7.8 release header). That ~40 % bump unblocks adding back the two
modules currently excluded from `dist/hisab.cyr`. The bundle today has
~7 KB of headroom under the 512 KB cap; on 5.7.9 we get ~520 KB of
headroom — easily enough for both files plus future growth.

When cc5 5.7.9 ships:

- [ ] Bump toolchain pin in `cyrius.cyml` 5.7.8 → 5.7.9
- [ ] Diagnose + fix the pre-existing parse issue in `lib/collision_core.cyr` (compiles standalone but trips inside larger compilation units — likely an upstream parse-state interaction with a 5.7.x-reserved keyword or syntax shift; was masked in 2.2.1 by the orphan-include trick in `src/main.cyr`)
- [ ] Same diagnosis + fix for `lib/collision_mesh.cyr` (Delaunay triangulation, half-edge mesh, island detection — 522 lines, written but never validated against cc5 5.7.x)
- [ ] Restore `"lib/collision_core.cyr"` and `"lib/collision_mesh.cyr"` in `cyrius.cyml [lib] modules` (after the foundation/types section, before calculus — see git history pre-2.2.2 for the prior position)
- [ ] Regenerate + commit `dist/hisab.cyr` (CI distlib drift gate enforces this)
- [ ] Add coverage in `tests/modules.tcyr` for the MPR / impulse-solver / Delaunay paths that 2.2.0 added but never exercised post-port

After this lands, hisab is back to "all 33 modules ship in the bundle."

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
