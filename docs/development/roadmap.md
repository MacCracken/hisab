# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.0.14**. Stdlib `linalg.cyr` provides dense decompositions.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.4.4

- **34 math modules in `src/`, ~16,460 lines** (`lib/` is vendored-only)
- **888 test assertions**, 28 benchmarks (incl. amplified SIMD batches), fuzz harness
- **CLI smoke binary** ~152 KB static ELF
- **`dist/hisab.cyr` distlib bundle** ~16,426 lines (all **34 modules**) — fits cycc 6.0.14's 1 MB input_buf with ample headroom
- Toolchain **6.0.14**; CI fmt/lint/vet/security all green
- P(-1) audit: 26/31 fixed
- **2.3.x optimization/modernization arc complete** (2.3.0 toolchain → 2.3.1 SIMD → 2.3.2 einsum scratch → 2.3.3 safety audit → 2.3.4 layout/idiom). Per-version detail in the Release History table + CHANGELOG.
- **2.4.x collision-correctness arc in progress** — 2.4.0 (`convex_hull_2d`) + 2.4.1 (`triangulate_polygon`) + 2.4.2 (`delaunay_2d`) + 2.4.3 (half-edge mesh) + 2.4.4 (MPR narrowphase) shipped; only 2.4.5 (PGS solver) pending.

---

## 2.4.x -- Collision correctness arc

`collision_core.cyr` and `collision_mesh.cyr` compile and link, but the heavy
algorithms carry pre-existing bugs from the 2.2.0 Rust port — they were added
then but never exercised (they sat outside the build chain via the old
orphan-include-after-syscall trick). 2.2.x only smoke-tests the API surface
(`contact_new` + `ColContact_*` accessors, `detect_islands`); the algorithms
below need a real correctness pass.

Unlike the 2.3.x arc (internal / codegen-neutral), **these patches change
behavior** — so the discipline inverts: a **red fixture comes first** (a test
that reproduces the bug or pins the expected result and currently fails), then
the fix, then robustness + coverage. Each algorithm is **its own patch**;
within a patch the work splits into **commit-sized bites** — each bite is one
self-contained, individually-verified commit (gates green before the next).
Public function signatures are unchanged (the functions already exist), so
every fix ships as a **patch**.

> Order is dependency- and risk-aware: 2D primitives first (smallest, one has a
> known trap), mesh connectivity next, then 3D narrowphase, then the solver that
> consumes contacts. Reorder by leverage as fixtures reveal coupling.

### 2.4.0 — `convex_hull_2d` (monotone chain) ✅ shipped
Two pre-existing port bugs made the function trap on any non-trivial input
(it had never run). Both fixed; 13 assertions added (833 → 846).
- [x] **Sort fix:** the hand-rolled insertion sort's `done`-exit path overwrote the insertion index with `-1` on mid-array inserts → `vec: index < 0` trap. Rewritten as a standard insertion sort (shift-greater, drop `key` at `sj + 1`).
- [x] **Missing primitives:** the chain's pop test calls `f64_le`/`f64_ge`, which were never defined (only strict `f64_lt`/`f64_gt` are intrinsics) → SIGILL once the sort was fixed. Defined both in `f64_util.cyr`; also clears the same latent references at 6 `spatial.cyr` sites.
- [x] **Coverage:** square + interior (4-vertex CCW hull, shoelace 2×area = +8); degeneracies — empty/single, triangle (count + CCW area), collinear (→ 2 endpoints), duplicate corner (→ 4). None trap.

### 2.4.1 — `triangulate_polygon` (ear clipping) ✅ shipped (no bug found)
Audited the suspected-buggy port — it was **already correct**. Deliverable: 13
assertions (846 → 859), no source change.
- [x] **Audit:** ear test (reflex classification + point-in-triangle), CCW/CW winding normalization, and shrink-as-clipped bookkeeping all correct. The `n*n` cap + bail guard terminate degenerate inputs without trapping. Shares none of the hull's broken sort and never referenced the undefined `f64_le`/`f64_ge`.
- [x] **Coverage:** count `== 3*(n−2)` + tiling check (|Σ 2×area| == polygon |2×area|) over convex quad, concave 5-gon, hexagon, U-shape (2 reflex), CW quad, collinear edge vertex, and `n < 3`.

### 2.4.2 — `delaunay_2d` (Bowyer-Watson) ✅ shipped (no bug found)
Audited the suspected-fragile algorithm — it was **already correct and robust**,
including the cocircular grid. Deliverable: 8 assertions (859 → 867), no source change.
- [x] **Audit:** the in-circle predicate adjusts by winding sign and tests *strict* interiority (cocircular points not flagged), so super-triangle setup/removal, bad-triangle cavity, boundary-edge extraction, swap-remove, and CCW output all hold.
- [x] **Coverage:** empty-circumcircle property verified (0 violations) on square+interior, **3×3 grid** (cocircular stress), and an irregular 6-point set; plus `n = 3`, `n < 3`, and all-collinear (→ empty, no trap).

### 2.4.3 — half-edge mesh (`halfedge_from_triangles` + accessors) ✅ shipped (no bug found)
Audited the twin wiring + boundary/adjacency queries — **already correct** on
closed and open meshes. Deliverable: 11 assertions (867 → 878), no source change.
- [x] **Audit:** reverse-edge twin pairing wires closed meshes fully and leaves open-boundary half-edges at `_COL_SENTINEL`; the `next→next→twin` one-ring walk (1000-step guard) detects interior vs boundary; adjacency collects each shared edge's twin face.
- [x] **Coverage:** tetrahedron (closed → 0 boundary, 3 neighbours/face), single triangle (open → all boundary, 0 neighbours), 2-tri quad (4 boundary corners, 1 shared edge), hexagon fan (interior center + 6-vertex boundary rim), and the empty → null error path.

### 2.4.4 — MPR narrowphase (`mpr_intersect` + `mpr_penetration`) ✅ shipped (real bug fixed)
Found + fixed a false-positive: separated shapes always reported a hit.
Deliverable: 10 assertions (878 → 888).
- [x] **Fix:** added the missing origin-containment early-out `if dot(v1, dir) < 0 → miss`. Without it, separated colinear shapes hit a degenerate `(v1−v0)×(−v0)==0` branch that returned 1 unconditionally, and off-axis pairs slipped through convergence. Applied to both `mpr_intersect` and `mpr_penetration`.
- [x] **Coverage:** sphere-sphere overlap vs separated (on/off-axis), penetration depth (1.0 and 3.0 cases) + ±x normal, separated → miss, box-box overlap/separated.

### 2.4.5 — contact solver (`sequential_impulse` + `solve_pgs`)
Projected Gauss-Seidel for contact constraints; verify convergence + restitution.
Depends on correct contacts (2.4.4), so it lands last.
- [ ] **Bite 1 (red):** one contact, two bodies, known normal + penetration → expected accumulated normal impulse / post-solve relative velocity. Failing baseline.
- [ ] **Bite 2 (fix PGS):** effective mass, accumulated normal impulse with `≥ 0` clamping, Baumgarte / bias position correction.
- [ ] **Bite 3:** restitution (bounce) + friction-cone clamping.
- [ ] **Bite 4 (coverage):** resting contact → relative normal velocity → 0; restitution = 1 → velocity reverses; multi-contact island (via `detect_islands`) converges.

---

## 2.5.0 -- CGA + matrix overflow guards

- [ ] CGA left/right contraction operators
- [ ] CGA dual operation, blade projection/rejection
- [ ] Confirm `matrix.cyr` `mat_new(rows, cols)` overflow is fixed upstream (was C3 in P(-1) audit; expected fixed by cyrius 5.x/6.x — verify with a regression test)

---

## 2.6.0 -- Differential geometry depth

- [ ] Parallel transport of vector fields along curves
- [ ] Sectional curvature computation
- [ ] Geodesic deviation equation
- [ ] Weyl tensor (conformal curvature)
- [ ] Higher-order differential forms (3-forms, 4-forms)

---

## 2.7.0 -- Rendering, GPU, reverse-mode AD

- [ ] Differentiable rendering math (autodiff through ray-surface intersections)
- [ ] Reverse-mode autodiff (Tape-based)
- [ ] GPU compute via soorat (feature-gated)

---

## 3.0.0 -- Error-handling migration (breaking)

The integer-error-code convention (`lib/error.cyr`: functions return 0 / a
negative `ERR_*` code) predates the stdlib `Result<T,E>` (`lib/result.cyr`,
v5.8.28) and `?` propagation (v5.8.29). Migrating is a library-wide signature
change — breaking for consumers (impetus, kiran, joshua, …) — so it lands as
a major, with a migration guide, not a 2.x patch.

- [ ] Wrap fallible returns in `Result<T,E>` (keep `ERR_*` codes as the `E` payload)
- [ ] Adopt `?` to replace manual `-1`-return + check chains
- [ ] Migration guide + deprecation window for the old integer-code API

---

## Parked / deferred (revisit when a driver appears)

Evaluated during earlier arcs and consciously deferred — recorded so they
aren't silently lost (full rationale in the CHANGELOG):
- **SIMD `cross` / `lerp`** (from 2.3.1) — need lane shuffles, not a clean `f64v_*` fit. Revisit if Cyrius adds packed shuffles.
- **`#pure` annotations** (from 2.3.4) — unsafe CSE interaction with hisab's allocate-a-fresh-result convention; speculative perf, no driver.
- **Slices (`[T]` / `slice<T>`)** (from 2.3.4) — would regress the proven raw-pointer SIMD hot paths; `slice_unchecked_get_W` discards the safety benefit.
- **`defer`** (from 2.3.4) — N/A under the bump/arena model (no per-resource lifecycle to clean up).

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
| 2.4.4 | 2026-05-28 | 16,460 | 34 | Collision arc — MPR narrowphase fixed (separated pairs were false +ve); 10 assertions. 888 |
| 2.4.3 | 2026-05-28 | 16,450 | 34 | Collision arc — half-edge mesh audited (no bug; twin/boundary wiring correct); 11 assertions. 878 |
| 2.4.2 | 2026-05-28 | 16,450 | 34 | Collision arc — `delaunay_2d` audited (no bug; cocircular-robust); 8 empty-circumcircle assertions. 867 |
| 2.4.1 | 2026-05-28 | 16,450 | 34 | Collision arc — `triangulate_polygon` audited (no bug); 13 tiling/count assertions added. 859 |
| 2.4.0 | 2026-05-28 | 16,450 | 34 | Collision arc — `convex_hull_2d` fixed (broken insertion sort + undefined `f64_le`/`f64_ge`); 13 assertions added. 846 |
| 2.3.4 | 2026-05-28 | 16,424 | 34 | Layout/idiom modernization — `alloc(sizeof(T))`+derived setters (13 modules), enum-const grid/buffer sizes, `#must_use` on core API. Codegen-identical, 833/833 |
| 2.3.3 | 2026-05-28 | 16,195 | 34 | Safety/numerical audit — no bugs; fixed wrong `>>` comment + 8 invariant tests. 833/833 |
| 2.3.2 | 2026-05-28 | 16,195 | 34 | Bounded einsum scratch via reused arena — 3960 → 176 B/call (~22×). Memory-only, 825/825 |
| 2.3.1 | 2026-05-28 | 16,195 | 34 | SIMD hot paths (`f64v_*`) for vec/mat/quat — vec4 dot 6.5×, m4_mul 4.5×, m3_mul 3.2×. Bit-identical, 825/825 |
| 2.3.0 | 2026-05-28 | 16,195 | 34 | Cyrius 6.0.14 toolchain; library source moved to `src/`; sakshi resolution repaired; CI aligned to abaco (fmt/security/version gates). No behavioral change |
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
