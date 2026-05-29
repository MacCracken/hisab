# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.0.14**. Stdlib `linalg.cyr` provides dense decompositions.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.5.3

- **34 math modules in `src/`, ~16,500 lines** (`lib/` is vendored-only)
- **929 test assertions**, 26 benchmarks (incl. amplified SIMD batches), fuzz harness
- **CLI smoke binary** ~152 KB static ELF
- **`dist/hisab.cyr` distlib bundle** ~16,446 lines (all **34 modules**) — fits cycc 6.0.14's 1 MB input_buf with ample headroom
- Toolchain **6.0.14**; CI fmt/lint/vet/security all green; supply chain SHA-locked (`deps --verify` 60/60, 0 untrusted)
- **Arc history** — the 2.3.x (optimization/modernization) and 2.4.x (collision-correctness + security) arcs are **complete**; per-version detail is in the Release History table and CHANGELOG. The 2.4.x arc fixed three real collision bugs (hull sort, MPR, contact solver), verified the rest, and audited the security posture (`docs/audit/2026-05-29.md`).
- **2.5.x arc COMPLETE** — CGA depth + matrix guard. 2.5.0 (contraction) → 2.5.1 (dual) → 2.5.2 (projection/rejection) → 2.5.3 (`mat_new` guard). CGA grew from 1 smoke assertion to 29; `mat_new_guarded` added as the CWE-190-safe constructor. The upstream stdlib `mat_new` fix remains tracked for when the cyrius pin moves.

---

## 2.5.x -- CGA depth + matrix guard

Conformal geometric algebra already ships in `geo_advanced.cyr` — a 32-blade
(2⁵) conformal multivector with geometric product, outer (wedge) product,
reverse, sandwich, norms, and the conformal constructors (`cga_point`,
`cga_sphere`, `cga_plane`, `cga_translator`, `cga_rotor`). What's **missing** is
the interior-product family — contraction, dual, projection/rejection — and CGA
currently has a single alloc smoke assertion. This arc adds those operators and
gives CGA real coverage, then closes out the carried-over `mat_new` guard.

Unlike the 2.4.x arc (bug fixes, red-fixture-first), these are **additive feature
patches** (new public `cga_*` functions, no signature changes), so each ships as
a patch. The discipline: each operator lands with the **GA identities that define
it** as its test oracle (e.g. contraction grade rules, dual involution,
projection idempotence) — write the identity assertion, implement against the
existing blade machinery (`_cga_geo_blades` / `_cga_blade_grade`), verify green,
no regression on the 901-assertion suite. Commit-bites per patch below.

> Order is dependency-aware: contraction is the interior-product primitive; dual
> needs the pseudoscalar inverse; projection/rejection compose contraction + a
> blade inverse. The `mat_new` guard is independent and lands last as the
> hardening closeout.

### 2.5.0 — CGA contraction operators (`cga_left_contraction` / `cga_right_contraction`) ✅ shipped
Added the interior products (⌋ left, ⌊ right) — the outer-product loop with a
grade-difference selector instead of grade-sum. 8 assertions (901 → 909).
- [x] **Implement:** `cga_left_contraction` keeps `grade(b)−grade(a)`, `cga_right_contraction` keeps `grade(a)−grade(b)`; negative targets never match a non-negative blade grade, so out-of-range terms drop to zero.
- [x] **Coverage (GA identities):** `e1 ⌋ e12 = e2`, `e1 ⌋ e1 = 1`, `(2e1+3e2) ⌋ self = 13` (vector norm²), `e1 ⌋ e23 = 0` (orthogonal), scalar contraction as scaling, `e12 ⌊ e1 = −e2` with grade drop.

### 2.5.1 — CGA dual + pseudoscalar inverse (`cga_pseudoscalar`, `cga_dual`) ✅ shipped
Added `cga_pseudoscalar` / `cga_pseudoscalar_inv` / `cga_dual`. 6 assertions (909 → 915).
- [x] **Implement:** `I` = grade-5 unit blade; `I⁻¹ = reverse(I)/(I·reverse(I))_scalar` (= −I, em²=−1, derived not hard-coded); `dual(x) = x · I⁻¹`.
- [x] **Coverage:** `I·I⁻¹ = 1`; grade flips `dual(1) = −I` (0→5), `dual(I) = 1` (5→0), `dual(e1) = −e23pm` (1→4); involution `dual(dual(e1)) = −e1` (sign pinned for this metric).

### 2.5.2 — CGA blade projection / rejection (`cga_project`, `cga_reject`) ✅ shipped
Added `cga_blade_inverse` / `cga_project` / `cga_reject`. 10 assertions (915 → 925).
- [x] **Implement:** `cga_blade_inverse(B) = reverse(B)/norm_sq(B)` (zero-norm guard for null blades); `cga_project(X,B) = (X ⌋ B) ⌋ B⁻¹` (preserves grade(X)); `cga_reject = X − project`.
- [x] **Coverage:** `project(e1,e12)=e1`, `project(e12,e12)=e12` (self), `project(e3,e12)=0` / `reject(e3,e12)=e3` (orthogonal), `reject(e1,e12)=0`, idempotence, `project+reject=X`, null-blade guard (no trap).

### 2.5.3 — `mat_new` overflow guard (arc closeout) ✅ shipped
Added `mat_new_guarded` as the hisab-side mitigation. 4 assertions (925 → 929).
- [x] **Re-check upstream:** stdlib `mat_new` (`16 + rows*cols*8`) still unguarded on the pinned 6.0.14 — the cyrius fix is deferred until the toolchain pin moves (tracked, not a hisab edit since `lib/` is vendored).
- [x] **Guarded wrapper:** `mat_new_guarded(rows, cols)` in `linalg_ext.cyr` caps dims (`_MAT_MAX_ELEMS = 16M`) and returns null on non-positive / overflow-prone sizes, else delegates to `mat_new`. Parity with `cmat_new`; the safe entry point for untrusted dims. Internal callers stay mitigated.
- [x] **Pin it:** 4 CWE-190 regression assertions in `tests/hisab.tcyr` (huge / zero / over-cap / valid).

> **Still open (deferred):** verify/land the upstream stdlib `mat_new` guard when the cyrius pin advances, then a regression test can target stdlib `mat_new` directly.

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
| **impetus** (physics) | Usable -- GJK/EPA, MPR, PGS, sequential-impulse, inertia, spatial |
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
| 2.5.3 | 2026-05-29 | 16,500 | 34 | CGA arc COMPLETE — `mat_new_guarded` (CWE-190 real-matrix guard); 4 assertions. 929 |
| 2.5.2 | 2026-05-29 | 16,490 | 34 | CGA arc — blade projection/rejection (`cga_project`/`cga_reject` + blade inverse); 10 assertions. 925 |
| 2.5.1 | 2026-05-29 | 16,480 | 34 | CGA arc — dual + pseudoscalar inverse (`cga_pseudoscalar`/`cga_dual`); 6 GA-identity assertions. 915 |
| 2.5.0 | 2026-05-29 | 16,470 | 34 | CGA arc — contraction operators (`cga_left_contraction`/`cga_right_contraction`); 8 GA-identity assertions. 909 |
| 2.4.6 | 2026-05-29 | 16,460 | 34 | Security/hardening audit — posture solid, no new vuln; 6 alloc-guard tests + threat-model refresh. 901 |
| 2.4.5 | 2026-05-29 | 16,460 | 34 | Collision arc COMPLETE — contact solver fixed (impulse was always 0); solve_pgs verified; 7 assertions. 895 |
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
