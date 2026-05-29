# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.0.14**. Stdlib `linalg.cyr` provides dense decompositions.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.6.2

- **34 math modules in `src/`, ~16,560 lines** (`lib/` is vendored-only)
- **943 test assertions**, 26 benchmarks (incl. amplified SIMD batches), fuzz harness
- **CLI smoke binary** ~152 KB static ELF
- **`dist/hisab.cyr` distlib bundle** ~16,575 lines (all **34 modules**) — fits cycc 6.0.14's 1 MB input_buf with ample headroom
- Toolchain **6.0.14**; CI fmt/lint/vet/security all green; supply chain SHA-locked (`deps --verify` 60/60, 0 untrusted)
- **Arc history** — the 2.3.x (optimization/modernization), 2.4.x (collision-correctness + security), and 2.5.x (CGA depth + matrix guard) arcs are all **complete**. Per-version detail is in the Release History table + CHANGELOG; equation material in [`../architecture/math.md`](../architecture/math.md). Suite grew 825 → 929 across them; the 2.4.x arc fixed three real collision bugs, the 2.5.x arc grew CGA from 1 → 29 assertions.
- **2.6.x arc in progress** — differential-geometry depth. 2.6.0 (sectional curvature) + 2.6.1 (Weyl) + 2.6.2 (parallel transport) shipped; 2.6.3 (geodesic deviation) → 2.6.4 (higher forms) → 2.6.5 (closeout) pending.

---

## 2.6.x -- Differential geometry depth

`diffgeo.cyr` already provides the curvature machinery — `christoffel_symbols`,
`riemann_tensor`, `ricci_tensor` / `ricci_scalar`, `einstein_tensor`,
`geodesic_rk4`, `killing_residual`, and a starter exterior algebra (`wedge_1_1`,
`hodge_star_2form_4d`). This arc deepens it with the curvature / transport
operators that build on those tensors, and extends the exterior algebra to
higher grades.

Additive feature patches (new functions, no signature changes), so each ships as
a patch. Discipline: each operator lands with a **closed-form / known-manifold
oracle** as its test — constant-curvature sphere `K = 1/r²`, Weyl vanishing for
`n ≤ 3` and conformally-flat metrics, metric-compatible transport preserving
length, the Jacobi equation `J'' = −K·J` on a sphere, form identities
(antisymmetry, `d² = 0`). Write the identity, implement against the existing
tensors, verify green, no regression. Commit-bites per patch.

> Order is dependency-aware: sectional curvature + Weyl are algebraic on the
> existing Riemann/Ricci (lowest-risk first); parallel transport adds connection
> integration; geodesic deviation builds on Riemann + transport; higher forms
> extend the exterior algebra independently. A P(-1)/security/docs closeout lands
> last (and grows the diffgeo section of `math.md`).

### 2.6.0 — Sectional curvature (`sectional_curvature`) ✅ shipped
`K(u,v) = R(u,v,v,u) / (⟨u,u⟩⟨v,v⟩ − ⟨u,v⟩²)` with the first Riemann index lowered.
5 assertions (929 → 934).
- [x] **Implement:** lower `R_{ασμν} = g_{αρ} R^ρ_{σμν}`, contract over the plane, degenerate-plane guard; `_dg_inner` metric inner product. Sign verified positive for a sphere (hisab convention: `R^θ_{φθφ} = +1` at the equator).
- [x] **Coverage:** constant-curvature space form → `K = 1` for any plane (axis + skew); radius-2 sphere (`metric = diag(4,4)`) → `1/4` (index-lowering); flat → 0; degenerate plane → 0.

### 2.6.1 — Weyl conformal-curvature tensor (`weyl_tensor`) ✅ shipped
Trace-free part of Riemann (`1/(n−2)` and `R/((n−1)(n−2))` trace terms). 5 assertions (934 → 939).
- [x] **Implement:** `weyl_tensor` / `weyl_get` from lowered Riemann + Ricci + scalar; all-zero for `n ≤ 2` (undefined there).
- [x] **Coverage:** `C = 0` for 3D and 4D space forms (Riemann ≠ 0 — conformally-flat oracle); non-space-form 4D → `C ≠ 0`; trace-free `g^{ρμ} C_{ρσμν} = 0`.

### 2.6.2 — Parallel transport along a curve (`parallel_transport`) ✅ shipped
RK4 integration of `dV^a/dt = −Γ^a_{μν} V^μ ẋ^ν` (constant-Γ per step, like
`geodesic_rk4`). 4 assertions (939 → 943).
- [x] **Implement:** `parallel_transport` + `_pt_deriv` RHS; the transport ODE is linear in V (`dV/dt = M·V` for fixed `ẋ`).
- [x] **Coverage:** flat (`Γ=0`) leaves V unchanged; unit-sphere latitude circle (θ=π/4) preserves `⟨V,V⟩` (metric compatibility) and rotates the vector.

### 2.6.3 — Geodesic deviation / Jacobi equation (`geodesic_deviation`)
The tidal acceleration `D²J^a/dτ² = −R^a_{μνρ} u^μ J^ν u^ρ` for a separation
field `J` along a geodesic with tangent `u`.
- [ ] **Bite 1 (oracle):** unit sphere → nearby geodesics satisfy `J'' = −J` (`K = 1`); flat space → `J'' = 0`. Failing baseline.
- [ ] **Bite 2 (implement):** evaluate the tidal term from `riemann_get` along a geodesic state.
- [ ] **Bite 3 (coverage):** sphere convergence vs flat zero-deviation; linearity of the tidal operator in `J`.

### 2.6.4 — Higher-order differential forms (`wedge_2_1`, `wedge_3_1`, …)
Extend the exterior algebra past 2-forms to 3- and 4-forms (4D), with grade
bookkeeping and antisymmetry.
- [ ] **Bite 1 (oracle):** graded antisymmetry `α∧β = (−1)^{pq} β∧α`, `α∧α = 0` for odd degree, associativity. Failing baseline.
- [ ] **Bite 2 (implement):** general graded wedge for 1/2/3-forms → up to 4-forms, reusing the antisymmetrization pattern in `wedge_1_1`.
- [ ] **Bite 3 (coverage):** grade/sign bookkeeping; `d(dα) = 0` on a sampled form if an exterior derivative is in scope.

### 2.6.5 — P(-1) / security / docs closeout
- [ ] Audit the new tensor allocations + contraction loops (dim caps, index bounds, degenerate-plane / `n<3` guards); cleanliness + full gate.
- [ ] Grow the **differential-geometry section of `docs/architecture/math.md`** (curvature conventions, sectional / Weyl / Jacobi formulas, the transport ODE, form grading) with references; dated audit report; doc-health refresh.

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
