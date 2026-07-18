# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.4.66**. Stdlib `ganita` (6.2.x math umbrella) provides dense decompositions + transcendentals.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.6.9

- **34 math modules in `src/`, ~16,900 lines** (`lib/` is vendored-only)
- **957 test assertions** (foundation 307 + hisab 175 + edge_cases 163 + modules 312), 26 benchmarks (incl. amplified SIMD batches), fuzz harness
- **CLI smoke binary** ~152 KB static ELF
- **`dist/hisab.cyr` distlib bundle** ~16,878 lines / 553 KB (all **34 modules**) — fits cycc 6.4.66's 1 MB input_buf with ample headroom
- Toolchain **6.4.66** (bumped from 6.3.11 this release — infrastructure + a test-only fix: no library source change, bundle byte-identical bar the header; first-party dep sakshi 2.4.2 → 2.4.6). `tests/modules.tcyr` had been silently un-compilable (a cycc identifier-lexer bug: result vars `iv_add`/`iv_sub`/`iv_mul` lex as `unknown` with `interval.cyr` in the unit — pre-existing, reproduces on 6.3.11; renamed to `iv_sum`/`iv_diff`/`iv_prod`, restoring the suite to 312/312). CI fmt/lint/vet/security all green; supply chain SHA-locked. Tracked-issue re-verify on the new pin: for-empty-clauses **still open**; new interval-ident-lex issue filed (`docs/development/issues/2026-07-17-cyrius-interval-ident-lex.md`) and worked around
- **Arc history (all complete)** — 2.3.x (optimization/modernization), 2.4.x (collision-correctness + security, fixed three real collision bugs), 2.5.x (CGA depth + matrix guard, CGA 1 → 29 assertions), and 2.6.x (differential-geometry depth — sectional curvature, Weyl, parallel transport, geodesic deviation, higher forms; 28 known-manifold assertions, posture audited solid). Per-version detail is in the Release History table + CHANGELOG; equation material in [`../architecture/math.md`](../architecture/math.md). Suite grew 825 → 957 across them.

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
- **Stdlib `mat_new` overflow guard** (from 2.5.3) — upstream cyrius fix; re-verify when the toolchain pin moves past it (hisab's `mat_new_guarded` is the local mitigation).

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
| 2.6.9 | 2026-07-17 | 16,600 | 34 | Toolchain 6.3.11 → **6.4.66** + sakshi 2.4.2 → **2.4.6**. Infrastructure + test-only fix — no library source change; bundle byte-identical bar the header. Fixed a pre-existing `tests/modules.tcyr` compile failure (cycc identifier-lexer bug: `iv_add`/`iv_sub`/`iv_mul` → `unknown` with `interval.cyr`; renamed `iv_sum`/`iv_diff`/`iv_prod`), restoring the suite to 312/312. Smoke string 2.6.7 → 2.6.9. New interval-ident-lex issue filed; for-empty-clauses still open. 957 |
| 2.6.8 | 2026-07-06 | 16,600 | 34 | Collision hardening for co-compilation with the sandhi/TLS stack: `symbolic` float-render scratch moved `var buf[N]` → `alloc(N)` (dodges the "array size must be enum constant" path under `tls`/`dynlib` co-compile); bare error constants namespaced `ERR_*` → `HSB_ERR_*` (values unchanged) to stop a last-wins global collision on consumers. 957 |
| 2.6.7 | 2026-06-30 | 16,600 | 34 | Toolchain 6.2.11 → **6.3.11** + sakshi 2.1.0 → **2.4.2**. Infrastructure-only — no library source change; bundle byte-identical bar the header. `lib/result.cyr` `_die` agnos-portability fix; smoke version string 2.3.3 → 2.6.7. for-empty-clauses still open on 6.3.11 (no new fixes). 957 |
| 2.6.6 | 2026-06-15 | 16,600 | 34 | Toolchain 6.0.14 → **6.2.11**. Stdlib math reorg: transcendentals + matrix/linalg → new `ganita` umbrella; `math` gains NaN-correct `f64_le`/`f64_ge` (dropped local copies). `[deps]`: +ganita −matrix −linalg. 3 of 5 tracked toolchain bugs fixed (archived). 957 |
| 2.6.5 | 2026-05-30 | 16,600 | 34 | Diffgeo arc COMPLETE — P(-1)/security audit (posture solid) + `math.md §2` differential-geometry reference. Docs-only, 957 |
| 2.6.4 | 2026-05-29 | 16,600 | 34 | Diffgeo arc — higher-order forms (`wedge_2_1`/`wedge_3_1`); 8 wedge antisymmetry/grading assertions. 957 |
| 2.6.3 | 2026-05-29 | 16,580 | 34 | Diffgeo arc — geodesic deviation / Jacobi (`geodesic_deviation`); 6 sphere/flat/linearity assertions. 949 |
| 2.6.2 | 2026-05-29 | 16,560 | 34 | Diffgeo arc — parallel transport (`parallel_transport`, RK4); 4 flat/sphere length-preservation assertions. 943 |
| 2.6.1 | 2026-05-29 | 16,540 | 34 | Diffgeo arc — Weyl conformal-curvature tensor (`weyl_tensor`); 5 space-form/trace-free assertions. 939 |
| 2.6.0 | 2026-05-29 | 16,520 | 34 | Diffgeo arc — sectional curvature (`sectional_curvature` from Riemann); 5 space-form/sphere assertions. 934 |
| 2.5.4 | 2026-05-29 | 16,500 | 34 | CGA arc closeout — P(-1)/security audit (posture solid) + `architecture/math.md` equation catalogue. Docs-only, 929 |
| 2.5.3 | 2026-05-29 | 16,500 | 34 | CGA arc — `mat_new_guarded` (CWE-190 real-matrix guard); 4 assertions. 929 |
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
