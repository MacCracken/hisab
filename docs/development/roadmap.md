# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.5.6**. Stdlib `ganita` (6.2.x math umbrella) provides dense decompositions + transcendentals.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.6.12

- **34 math modules in `src/`, ~16,900 lines** (`lib/` is vendored-only)
- **981 test assertions** (foundation 307 + hisab 186 + edge_cases 173 + modules 315), 26 benchmarks (incl. amplified SIMD batches), fuzz harness (1/1), and `scripts/check-constants.sh` verifying **110/110** hand-encoded f64 constants against their own comments
- **CLI smoke binary** ~207 KB static ELF
- **`dist/hisab.cyr` distlib bundle** ~16,897 lines / 554 KB (all **34 modules**) — fits cycc 6.5.6's 1 MB input_buf with ample headroom
- Toolchain **6.5.6** (bumped from 6.4.69 in **2.6.11** — a **minor** jump across 24 releases: no executable library change, the bundle diff being the version header plus one `mat_new_guarded` doc comment; sakshi 2.4.6 → **2.4.7**). The headline effect is the vendored `ganita` reaching **1.0.4**: the repo's copy had been stale at **1.0.3** since before the 6.4.69 pin, so this bump closes the long-tracked CWE-190 in stdlib `mat_new` — measured, `mat_new(-5, 3)` **segfaulted** on 1.0.3 and returns null on 1.0.4. hisab's `mat_new_guarded` stays as the stricter 16M-element policy cap (upstream's ceiling is 33.5M). Also adopted this release: the 6.5.6 `sys_exit_group` epilogue idiom, and an **exit-code clamp** in all four test harnesses — the unclamped `assert_summary()` count meant exactly 256/512/768 failures exited 0 and scored PASS (proven, then fixed). `cyrius fuzz` now discovers `tests/*.fcyr` (6.5.6 walks `tests/`, not just `fuzz/`), so the harness runs for the first time — and passes. CI fmt/lint/vet/security all green; supply chain SHA-locked (`cyrius.lock` 30 deps, verify 30/30). Tracked-issue re-verify on the new pin (minimal repros): for-empty-clauses **still live**, interval-ident-lex **still live** (both worked around) — no new fixes
- **Arc history** — 2.3.x (optimization/modernization), 2.4.x (collision-correctness + security, fixed three real collision bugs), 2.5.x (CGA depth + matrix guard, CGA 1 → 29 assertions), and 2.6.x (differential-geometry depth — sectional curvature, Weyl, parallel transport, geodesic deviation, higher forms; 28 known-manifold assertions). Per-version detail is in the Release History table + CHANGELOG; equation material in [`../architecture/math.md`](../architecture/math.md). Suite grew 825 → 961 across them.
- **The 2.6.x *feature* line is closed** — no residual feature work carried forward; the diffgeo
  arc completed at 2.6.5 and 2.6.6–2.6.11 were toolchain/dependency maintenance. What the 2.6.x
  line still owes is **repair, not features**: the 2.6.12 sweep below. Items too large for a patch
  release have been moved to 2.7.0.
- ⚠️ **The "posture audited solid" claim from the 2.5.4 / 2.6.5 closeouts did not hold.** Those
  audits were scoped to the arcs' *new* functions and did not reach the modules no test suite
  includes. The 2026-08-03 sweep found 70 verified defects, 2 critical — see 2.6.12.

---

## 2.6.12 -- Audit, hardening & repair sweep (PARTIAL — constants + harness shipped)

> **Status 2026-08-03.** The systemic constant defect is closed and guarded; `num_is_prime` and
> `solve_bicgstab` are fixed. **Five P0 items and all of P1–P4 remain open** — see the unchecked
> boxes below. Suite 961 → **981**, all mutation-proven. Next up, in order: `svd_golub_kahan`,
> `eigen_qr`, the SU(2)/SE(3) convention split, `einsum` validation, `ivl_sin`.

A full P(-1) sweep of the 2.6.11 tree: 8 parallel audit dimensions over all 34 modules, every
finding then adversarially re-verified against the running code. **70 findings survived
verification** (2 critical, 23 high, 23 medium, 22 low) across 64 distinct sites in 27 files;
7 were refuted and dropped. This is a **repair release**, not a feature release.

**The headline is a systemic defect class, not a scattering of unrelated bugs.** Seven separate
hand-encoded IEEE-754 hex constant tables were mis-transcribed — in every case the adjacent
comment states the *correct* value and the bit pattern does not encode it. Four are published
numerical-method tableaux, and each one is falsified by its own mathematical invariant
(independently re-confirmed by decoding the shipped literals):

| table | site | invariant | shipped | required |
|---|---|---|---|---|
| Dormand-Prince RK4(5) | `ode.cyr:55-88` | Σb = 1 | **0.6356** | 1.0 |
| Gauss-Legendre 5-pt | `calc.cyr:96-107` | Σw = 2 | **2.000492** | 2.0 |
| BDF-4 | `ode.cyr:108-110` | Σα = 1 | **1.01** | 1.0 |
| Yoshida-4 symplectic | `ode.cyr:130-136` | 2w₁+w₀ = 1 | **1.4738** | 1.0 |

23 of DOPRI45's 30 tableau constants are wrong and its stage rows fail `Σa_ij = c_i` — it is
**not a consistent integrator at any order**. All four ship in `dist/hisab.cyr`; **joshua**
(simulation) consumes exactly these. The other three mis-transcribed tables are `color.cyr`'s
sRGB breakpoints (0.03275 vs the spec's 0.04045) and spherical-harmonic normalisations, and
`noise_simplex.cyr`'s `_SIMPLEX_G2`.

### P0 -- Correctness (critical / high)

- [x] **Re-derived all four numerical-method tableaux** from exact rationals (DOPRI45, BDF-4,
      Yoshida-4, Gauss-Legendre-5) plus the three other mis-transcribed tables (sRGB breakpoints,
      spherical harmonics, `_SIMPLEX_G2`) and the `quat` slerp threshold — **47 constants**, each
      table now pinned by its own invariant rather than a bounds check. Σb = 1, Σα = 1,
      2w₁+w₀ = 1, Σw = 2 all hold. **Shipped 2.6.12**
- [x] **`num_is_prime` i64 overflow** (`num.cyr`) — Miller-Rabin squared with raw `x * x` instead
      of `_num_mulmod`; returned *composite* for the genuine primes 3037218841 and 4294967357.
      Fixed and pinned across the overflow band. **Shipped 2.6.12**
- [x] **`solve_bicgstab` was a no-op** (`linalg_ext.cyr`) — `f64_from(0x3C32725DD1D243AC)`
      *converted* a bit pattern instead of reinterpreting it, making the breakdown tolerance
      4.34e18 so the solver returned the initial guess verbatim. **Shipped 2.6.12**
- [ ] **`svd_golub_kahan` returns U transposed** (`linalg_precision.cyr:180`) — `A != U·S·Vᵀ`;
      S and Vᵀ are correct, only the left-reflector composition order is reversed
- [ ] **`eigen_qr` never converges for n ≥ 3** (`linalg_precision.cyr:851-859`) — the Givens
      similarity is applied transposed relative to the rotation that zeroes the bulge; at n = 2 it
      pairs each eigenvalue with the wrong eigenvector
- [ ] **SU(2)/SE(3) angle-convention split** (`lie.cyr:139`, `lie_ext.cyr:371`) — `su2_exp`/`se3_exp`
      use a full-angle convention while `so3_exp`, `su2_from_axis_angle` and every SE(3) consumer
      assume half-angle, so `se3_exp` yields an (R, t) pair that is not the exponential of any twist
- [ ] **`einsum` silently returns wrong answers for its own documented examples** (`einsum.cyr:5,65`)
      — labels are accepted only in `a`..`h`, so the `i,j,k` used in the module's header examples are
      *silently dropped* rather than rejected; plus an out-of-bounds shape read when an operand has
      more than `_EINSUM_MAX_RANK` labels
- [ ] **`ivl_sin` violates the interval contract** (`interval.cyr:135`) — returns an enclosure
      *narrower* than the true range when the input straddles an extremum. An interval library that
      under-approximates is worse than none; abaco consumes this

### P1 -- Memory safety & robustness

- [ ] Null-return dereferences after capped constructors: `cmat_mul`/`cmat_kronecker`
      (`complex.cyr:332`), `cmat_inverse` (`linalg_ext.cyr:931`), `einsum` (`einsum.cyr:231`),
      `opt_bfgs`/`opt_lbfgs` (`optimize.cyr:279`) — each turns a designed error signal into SIGSEGV
- [ ] `calc_bspline` / `calc_nurbs` negative control-point index → wild-pointer deref when
      `n_pts <= degree` (`calc_ext.cyr:335`)
- [ ] `kdtree_build` O(n) recursion depth on coincident points → **segfault at 60k points**
      (`spatial.cyr:61`); `_kd_partition` splits on the value midpoint, not a positional median
- [ ] Adaptive Simpson bounds *depth* (50) but not *work* → ~2^50 evaluations on a NaN integrand
      (`calc_ext.cyr:243`); NaN also defeats the Armijo test in `_opt_armijo` (`optimize.cyr:94`),
      poisoning BFGS/L-BFGS/CG
- [ ] `FLOAT_RENDER_BUF = 32` too small for `fmt_float_buf`, which uses the caller buffer as scratch
      out to offset 40 (`symbolic.cyr:19`)
- [ ] `cx_div`/`cx_inv`/`cx_powf` compare a **squared** modulus against the unsquared `EPSILON_F64`,
      so the effective zero-divisor cutoff is 1e-6, not 1e-12 (`complex.cyr:57`)
- [ ] `ivl_div` zero-guard uses raw integer `== 0` on an f64 bit pattern, so **-0.0 slips through**
      (`interval.cyr:80`); `ivl_sqrt` of a wholly-negative interval returns `[0, NaN]`
- [ ] Four fBm entry points return NaN for `octaves <= 0` despite documenting a `[-1,1]` range

### P2 -- Test coverage (the root cause)

`cyrius coverage` reports **297/587 functions (50%)** against CLAUDE.md's 80% target, and the
correlation with the defect map is the finding: **35 of 65 source defects sit in modules with
≤30% function coverage.** Eight modules — `calc_ext`, `einsum`, `lie_ext`, `linalg_precision`,
`mat3`, `noise_simplex`, `num_ext`, `symbolic_ext` — **5,068 lines (30% of `src/`), 104 public
functions** — are included by *no* test suite, so they are not even compiled by `cyrius test`,
yet all eight ship in `dist/hisab.cyr` to 8 consumers.

- [ ] Bring the 8 never-included modules into the suites; then raise `cyrius coverage` past 80%
- [ ] Replace bounds-check assertions with **value** assertions where a broken method still passes:
      the only `ode_dopri45` test asserts `1 < y < 2` for a value whose exact answer is 1.10517,
      and the `gauss5` tests round to the nearest integer, hiding a 246 ppm error floor
- [x] **Constant-verification harness** — `scripts/check-constants.sh` decodes every hex f64
      literal in `src/` and asserts it against the value in its own comment (exact rationals,
      closed-form expressions, truncated decimals; tolerance derived from the precision the
      comment claims). Also cross-checks comments stating both an exact form and a decimal, which
      caught four DOPRI45 comments whose decimals disagreed with their own fractions.
      **110/110 verified; wired into CI.** **Shipped 2.6.12**
- [x] Replaced the loose assertions that let the broken tableaux ship — see the P2 note above;
      `ode_dopri45`, `gauss5`, BDF-4, Yoshida-4, sRGB, `solve_bicgstab` and the `num_is_prime`
      overflow band all now carry value assertions. Suite 961 → **981**. **Shipped 2.6.12**
- [ ] Pin the remaining CWE-190 guard (`num_sieve`'s 10M cap) that the 2026-05-29 audit claims is
      already pinned but is not

### P3 -- Performance (each needs before/after numbers before any claim)

- [ ] `halfedge_from_triangles` twin pairing is O(n_he²) — **3.07 s on an 8,192-triangle mesh**
      (`collision_mesh.cyr:331`); a hash of vertex-pair keys makes it linear
- [ ] `convex_hull_2d`'s insertion-sort pre-pass — **690 ms on 8,000 points** (`collision_core.cyr:339`)
- [ ] Levenberg-Marquardt allocates 4 loop-invariant scratch buffers per iteration (5.35 MB of
      unreclaimed bump scratch over 200 iterations) and computes the symmetric JᵀJ twice
- [ ] L-BFGS allocates 3 n-length buffers per iteration

### P4 -- Documentation drift (10 findings)

- [ ] `overview.md`'s module map credits BVH to `spatial.cyr` (all 12 `bvh_*` live in
      `geo_advanced.cyr`) and lists `f64_le`/`f64_ge` in `f64_util.cyr`, which documents them as removed
- [ ] `opt_conjugate_gradient` documented Fletcher-Reeves, implements Polak-Ribière+
- [ ] `num_dst` documented DST-II, implements DST-I
- [ ] `expr_eval` documents "aborts on undefined variable"; it warns and returns 0.0 — and a test
      was skipped *because of* that false comment
- [ ] `hodge_star_2form_4d` documents its `sign` parameter three mutually contradictory ways
- [ ] `collision_core.cyr`'s header names a file that no longer exists
- [x] README's Building block said 163 edge-case tests — synced to the real counts. **Shipped 2.6.12**

> **Sequencing.** P0 first, each fix landing with the value-assertion that would have caught it.
> P2's constant harness before closing P0, so the tableau re-derivations are verified mechanically.
> P1 and P3 are independent and can batch. P3 claims need `./scripts/bench-history.sh` numbers.

---

## 2.7.0 -- Rendering, GPU, reverse-mode AD

- [ ] Differentiable rendering math (autodiff through ray-surface intersections)
- [ ] Reverse-mode autodiff (Tape-based)
- [ ] GPU compute via soorat (feature-gated)

**Moved here from the 2.6.x line** (too large for a patch release; each needs its own benchmark
and design pass rather than riding the 2.6.12 repair sweep):

- [ ] **Adopt `vec_sort_by` / `vec_select_nth`** (cyrius 6.5.4). Blocked on an API mismatch, not
      effort: the comparator receives element *values* via `fncall2`, while hisab sorts *indices*
      by dereferencing a separate `points` vector, and Cyrius has no closures. Needs either a
      packed key+index encoding or a context-carrying comparator upstream. 2.6.12 fixes the
      *complexity* of the two hot sorts; this item is the *consolidation*
- [ ] **SIMD the flat-array kernels** — `_opt_dot`/`_opt_norm`/`_opt_axpy`, the three L-BFGS sweeps
      and `_lext_dot`/`_lext_norm` are scalar while `f64v_dot`/`f64v_scale` take a runtime `n`
      (measured 5-8× on comparable kernels in 2.3.1). Deferred out of 2.6.12 because a hot-path
      rewrite must not ride a correctness release
- [ ] **Declare the per-module dependency graph.** The README documents an à-la-carte
      `modules = ["src/vec3.cyr", ...]` consumption path, but 5 module pairs reach across
      boundaries into another module's *private* (`_`-prefixed) symbols — `noise_simplex` and
      `calc_ext` need `calc`'s `_perm`/`_perm_init`, `lie_ext` needs `lie`'s `_su2_*`, `num_ext`
      needs `num`'s `_num_mulmod`, `symbolic_ext` needs `symbolic`'s `_sym_is_zero`. The full
      bundle is unaffected (the graph is a DAG and `[lib]` order is a valid topological sort), but
      a consumer picking files individually gets link errors with no manifest to consult

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
aren't silently lost (full rationale in the CHANGELOG). Items that have since **shipped** are
removed from this list rather than struck through; their record lives in the CHANGELOG and the
Release History table below.
- **SIMD `cross` / `lerp`** (from 2.3.1) — need lane shuffles, not a clean `f64v_*` fit. Revisit if Cyrius adds packed shuffles.
- **`#pure` annotations** (from 2.3.4) — unsafe CSE interaction with hisab's allocate-a-fresh-result convention; speculative perf, no driver.
- **Slices (`[T]` / `slice<T>`)** (from 2.3.4) — would regress the proven raw-pointer SIMD hot paths; `slice_unchecked_get_W` discards the safety benefit.
- **`defer`** (from 2.3.4) — N/A under the bump/arena model (no per-resource lifecycle to clean up).

*Retired 2026-08-03:* **stdlib `mat_new` overflow guard** (parked from 2.5.3) — shipped in
**2.6.11** via ganita 1.0.4 at the 6.5.6 pin. `mat_new_guarded` is retained as the stricter 16M
entry point, so nothing further is owed.

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
| 2.6.12 | 2026-08-03 | 16,600 | 34 | **Audit sweep — repair release.** Full P(-1) audit of all 34 modules found **70 verified defects (2 critical)**; see `audit/2026-08-03.md`. Closed the critical tier: **seven hand-encoded constant tables** did not encode their documented values — **47 constants re-derived** from exact rationals. DOPRI45 had 23 of 30 tableau constants wrong (Σb = 0.636, not 1) and was **not a consistent integrator at any order**; BDF-4 drifted 1%/step; Yoshida-4 moved a free particle 85.9% of the correct distance; Gauss-5 carried a 246 ppm error floor; plus sRGB breakpoints, spherical harmonics, `_SIMPLEX_G2`, the slerp threshold. Also fixed `num_is_prime` (i64 overflow reported real primes composite above 3.03e9) and `solve_bicgstab` (`f64_from` on a bit pattern made the tolerance 4.34e18 — it never iterated). Added **`scripts/check-constants.sh`**, a CI gate verifying all 110 hex f64 literals against their comments. Replaced the loose assertions that let it all ship (dopri45 asserted only `1 < y < 2`). 981 |
| 2.6.11 | 2026-08-03 | 16,600 | 34 | Toolchain 6.4.69 → **6.5.6** (a **minor** jump across 24 releases) + sakshi 2.4.6 → **2.4.7**. No executable library change — the bundle diff is the version header plus one `mat_new_guarded` doc comment, zero code lines. **Security:** the vendored `lib/ganita.cyr` was stale at **1.0.3** (the 6.4.69 pin already shipped 1.0.4); re-vendoring closes the tracked CWE-190 in stdlib `mat_new` — `mat_new(-5, 3)` **segfaulted** on 1.0.3, returns null on 1.0.4 (measured, same compiler, only ganita swapped). **Fixed:** all four `.tcyr` harnesses exited with `assert_summary()`'s raw failure count, so exactly 256/512/768 failures truncated to 0 and scored PASS — now clamped. Adopted the 6.5.6 `sys_exit_group` epilogue. `cyrius fuzz` now discovers `tests/*.fcyr` (1 passed — previously never run). Smoke string 2.6.10 → 2.6.11. Tracked issues re-verified still-live (interval-ident-lex, for-empty-clauses). +4 assertions pinning the upstream `mat_new` contract. 961 |
| 2.6.10 | 2026-07-21 | 16,600 | 34 | Toolchain 6.4.66 → **6.4.69** (clean 3-patch bump; sakshi unchanged at 2.4.6, already latest). No library source change — bundle byte-identical bar the header. Vendored stdlib picks up three upstream fixes: `fmt` hex-high-bit + `fmt_float_buf` non-finite guard (linked by `symbolic`; byte-identical for finite values), `math` float-parse DoS hardening, agnos-only `sys_reboot` widening. Smoke string 2.6.9 → 2.6.10. Tracked issues re-verified still-live (interval-ident-lex, for-empty-clauses); no new fixes. 957 |
| 2.6.9 | 2026-07-17 | 16,600 | 34 | Toolchain 6.3.11 → **6.4.66** + sakshi 2.4.2 → **2.4.6**. Infrastructure + test-only fix — no library source change; bundle byte-identical bar the header. Fixed a pre-existing `tests/modules.tcyr` compile failure (`iv_add`/`iv_sub`/`iv_mul` collide with reserved cycc SIMD intrinsic names; renamed `iv_sum`/`iv_diff`/`iv_prod`), restoring the suite to 312/312. Smoke string 2.6.7 → 2.6.9. New interval-ident-lex issue filed; for-empty-clauses still open. 957 |
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
