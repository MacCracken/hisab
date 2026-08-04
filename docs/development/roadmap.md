# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.5.6**. Stdlib `ganita` (6.2.x math umbrella) provides dense decompositions + transcendentals.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.6.15

- **34 math modules in `src/`, ~16,900 lines** (`lib/` is vendored-only)
- **1127 test assertions** (foundation 307 + hisab 205 + edge_cases 199 + modules 416), 28 benchmarks (incl. amplified SIMD batches), fuzz harness (1/1), and `scripts/check-constants.sh` verifying **110/110** hand-encoded f64 constants against their own comments. `cyrius coverage` **348/587 (59%)**, files 34/35 — **all 34 modules now covered by a suite**
- **CLI smoke binary** ~207 KB static ELF
- **`dist/hisab.cyr` distlib bundle** ~17,286 lines / 578 KB (all **34 modules**) — fits cycc 6.5.6's 1 MB input_buf with ample headroom
- Toolchain **6.5.6**, sakshi **2.4.7**. CI fmt/lint/vet/security green; supply chain SHA-locked
  (`cyrius.lock` 30 deps, verify 30/30). Pin/vendoring detail lives in
  [`dependency-watch.md`](dependency-watch.md); the 6.5.6 bump story is CHANGELOG 2.6.11
- **Tracked toolchain issues: 3 open, all worked around** — interval-ident-lex and
  for-empty-clauses re-confirmed live on 6.5.6; cli-arg-clobber not re-tested (destructive).
  See [`issues/`](issues/)
- **Arc history** — 2.3.x (optimization/modernization), 2.4.x (collision-correctness + security),
  2.5.x (CGA depth), 2.6.0–2.6.5 (differential-geometry depth), 2.6.6–2.6.11 (toolchain/dependency
  maintenance), **2.6.12–2.6.15 (audit & repair)**. Per-version detail is in the Release History
  table + CHANGELOG; equation material in [`../architecture/math.md`](../architecture/math.md).
  Suite grew 825 → 1127 across them
- **The 2.6.x line is closed** — features ended at 2.6.5, and the repair arc discharged the
  2026-08-03 audit in full. Nothing carries forward; 2.7.0 is next
- ⚠️ **Standing caution for future closeouts.** The "posture audited solid" verdicts of the 2.5.4
  and 2.6.5 closeouts did **not** hold: both were scoped to the functions their own arc had just
  added and never reached the modules no suite included. The 2026-08-03 sweep then found 70
  defects, 2 critical, almost all in exactly those modules. **An audit's scope is part of its
  result** — say what was not looked at

---

## 2.6.12–2.6.15 -- Audit & repair arc  **CLOSED**

Retired from active work 2026-08-03. The full record lives in
[`../audit/2026-08-03.md`](../audit/2026-08-03.md) (all 70 findings), the CHANGELOG entries for
2.6.12–2.6.15, and the Release History table below. Kept here only as the one-paragraph summary
a future reader needs:

> A P(-1) sweep of the 2.6.11 tree — 8 parallel dimensions over all 34 modules, every finding
> adversarially re-verified — produced **70 confirmed findings (2 critical, 23 high, 23 medium,
> 22 low)** across 64 sites. All four tiers closed across four releases. The headline was
> systemic, not scattered: **seven hand-encoded IEEE-754 constant tables did not encode the
> values their own comments stated**, including four published tableaux each falsified by its own
> invariant — DOPRI45's Σb was 0.636, so it was **not a consistent integrator at any order**.
> Also closed: an SVD returning U transposed, an eigensolver that never converged for n ≥ 3,
> `einsum` mis-parsing its own documented examples then segfaulting, a `kdtree_build` that
> SIGSEGV'd at 60k coincident points, and two O(n²) hot paths (190 ms → 1.2 ms, 22 ms → 2.1 ms).
> **Every defect sat in a module with zero or near-zero coverage** — eight modules were not even
> compiled by `cyrius test`. Suite 957 → 1127; coverage → 59%; all 34 modules now under test.
> `scripts/check-constants.sh` was added as a CI gate so the constant class cannot recur.

**What the arc leaves behind as standing practice** (not work items — they are already in place):
- `scripts/check-constants.sh` runs in CI between the vet and distlib gates.
- Vendoring is byte-checked against the **pin's own snapshot**, not previous-pin vs new-pin.
- Test harnesses clamp `assert_summary()` before exiting, and use `sys_exit_group`.
- Fixes land with the assertion that would have caught them; perf claims need before/after rows
  in `bench-history.csv`.

---

## 2.7.0 -- Re-audit & refactor  (IN PROGRESS)

The re-audit has **run** — see [`../audit/2026-08-04.md`](../audit/2026-08-04.md): **42 findings
confirmed, 8 refuted** (4 high, 21 medium, 17 low; no criticals) across six dimensions.

> **Regression verification came back CLEAN** — every 2.6.12–2.6.15 repair holds under execution,
> verified against mathematical invariants rather than comments, with the pre-fix files re-run as
> controls. The rewrites are correct as well as fast.
>
> ⚠️ **But it also corrected the record: the 2026-08-03 audit was NOT fully discharged.** Six of
> the original 70 findings were never fixed, because the repair work was scheduled from this
> roadmap's P0–P4 *summary bullets* — a deduplicated digest — rather than from the audit's finding
> list. "All P-tiers closed" was then read as "all findings closed". **An audit's finding list is
> the unit of disposition.** Every numbered finding now needs an explicit disposition before 2.7.0
> closes.

### Already fixed during the audit (gate infrastructure, not scheduled work)
- [x] **`check-constants.sh` had a 25% blind spot** — its regex rejected `_` digit separators, so
      it printed "110/110 verified" while skipping 35 of 145 declarations. Coverage now
      **110 → 145**, ±Infinity verified rather than skipped, and the skip list is printed
- [x] **`symbolic_ext.cyr:213`** — `limit` encoded 9999999978962944.0 against a documented `1e15`;
      exposed by the fixed gate

### Carried over — never fixed from the 2026-08-03 audit  ✅ **CLOSED (2.7.0-A)**

All six fixed and **mutation-proven**: each source file was reverted and the new assertions
confirmed to fail. Suite 1127 → 1154.

- [x] `geo_ray_capsule` returned NaN instead of 0 on a parallel miss (`geo.cyr:456`) — `a == 0`
      made `disc` exactly `+0.0`, which does not satisfy `disc < 0`, so the cap-sphere fallback
      was skipped and `0.5/0` was evaluated. The parallel case now joins the cap path
- [x] `cmat_exp` scaling loop never terminated on an infinite Frobenius norm (`complex.cyr:390`) —
      `+Inf * 0.5` is still `+Inf`. Proven: 1 s with the fix, >110 s without. Bounded at 64
- [x] `geo_ray_plane` now returns 0 on a miss, matching the contract stated twice in its own
      module. **Behaviour change** — the old `-1` was a negative NaN as an f64
- [x] `f64_fmod` used floor where IEEE-754 `fmod` truncates — the result carried the sign of the
      divisor, not the dividend (`fmod(-7, 3)` gave `+2`, not `-1`)
- [x] `f64_copysign` branched on an *ordered* comparison, so it was blind to `-0.0` and could
      never produce it; replaced with the specified sign-bit splice
- [x] `lorentz_is_valid` used an absolute `1e-12` against terms of size cosh²(η), rejecting valid
      boosts from rapidity ~5 up; now scaled by matrix magnitude, with rejection still asserted
- [x] *(found while fixing)* `lorentz_boost_x/y/z` took **rapidity** but named the parameter
      `beta` (velocity), contradicting their own doc comments — renamed `eta`

### High  ✅ **CLOSED (2.7.0-B)**

`geo_advanced.cyr` — the largest module at 1,261 lines — had **zero** coverage on GJK/EPA/BVH/TOI
before this. Suite 1154 → 1181.

- [x] **`cmat_mul` never checked `cmat_cols(a) == cmat_rows(b)`** and read past the end of b's
      buffer (`complex.cyr:224`) — a 2×3 times a 2×2 read two complex numbers of adjacent heap
- [x] **`_bvh_build_rec` degenerated to a 1-vs-(n−1) split** on coincident AABBs
      (`geo_advanced.cyr:532,594`). Not O(n) depth alone — because each level re-merges its whole
      range for `bounds`, build cost was **O(n²)**. Median split restores O(n log n):
      `bvh_degenerate_4k` **2.722 s → 15.5 ms (−99.4%)**, row in `bench-history.csv`.
      ⚠️ Proven by benchmark, **not** by assertion — both forms return the same answer, so the
      defect is a cost and `assert_eq` cannot state it
- [x] **`time_of_impact` missed every collision beyond 0.64 units** of relative travel
      (`geo_advanced.cyr:744`) — the loop was bounded by `GJK_MAX_ITER`, a cap on simplex
      refinement, not on time sampling, so `max_t` was ignored entirely. A false negative from a
      collision query. Step budget is now its own constant spanning the full horizon

### Medium — correctness & safety
- [x] `cmat_commutator`/`add`/`sub`/`adjoint`/`scale`/`trace` dereferenced `cmat_mul`'s designed 0
      — the 2.6.14 null-propagation fix stopped one call-link short. **Both mutants SIGSEGV**;
      guarded, plus shape checks on `add`/`sub` *(closed with 2.7.0-B)*
- [x] `detect_islands` validated only the upper bound of contact body indices; a negative index
      subscripted before the union-find arrays (`collision_mesh.cyr:538`). **SIGSEGV** at large
      magnitude; at small magnitude a *silent* wrong partition (3 islands where 4 is correct)
      *(2.7.0-C)*
- [x] diffgeo's downstream tensor functions applied neither the dim cap nor a null check, so both
      the cap path and the alloc-failure path dereferenced NULL (`diffgeo.cyr:336`). Shared
      `_DG_MAX_DIM`, operand guards on 9 functions, checks at **all ten** alloc sites — plus
      `wedge_1_1`, a live SIGSEGV the finding never mentioned *(2.7.0-C)*
- [ ] `svd_golub_kahan` returns NO_CONVERGENCE plus all-zero singular values for a **rank-deficient**
      matrix (`linalg_precision.cyr:340`). ✅ verified + challenged, fix specified, **not applied**.
      Re-checked after the 2.7.0-D balancing landed in case it was fixed incidentally — it is
      **not**: a rank-1 3×3 (true sv 14, 0, 0) still returns NO_CONVERGENCE with S all zero
- [x] `expr_to_str`/`sym_to_latex` dropped the rounding carry — 0.9999999 rendered as "0.1000000"
      and 2.9999999 as "2.1000000". stdlib `fmt_float_buf` skips its zero-pad when the fraction
      rounds up to 10^decimals. hisab now carries `_sym_render_f64` *(2.7.0-D)*
- [x] `levi_civita_3`/`_4` returned raw i64 while their siblings return f64 bit patterns; the −1
      case was a **negative NaN**, poisoning even the exactly-zero components of a contraction
      (`tensor.cyr:227`). 8 existing assertions migrated with it *(2.7.0-C)*
- [x] `num_ifft` conjugated the caller's buffer before validating, so its error return left the
      input corrupted (`num.cyr:236`) — a failure code *and* silently mutated data *(2.7.0-C)*
- [x] **Householder reflector construction was unscaled** (`linalg_precision.cyr:31`) — usable band
      only ~1 .. 1e23; **40/40 failures at scale 1e-10** (SI metres) across 440 random matrices.
      Overflow gave NaN `U` + NO_CONVERGENCE; underflow silently returned the raw diagonal of A,
      42.6% off, with `HSB_ERR_NONE`. `svd_golub_kahan`/`eigen_qr`/`cqr_decompose` now balance to
      unit max magnitude with a **power-of-two divisor**, so unit-scale results are bit-identical.
      9 of 11 new assertions fail on revert *(2.7.0-D)*

### Test quality  (12 findings — assertions that cannot fail for the right reason)
- [ ] `num_rk4`'s **only** assertion in the whole tree tolerates a 90% error
- [ ] `ode_verlet` / `ode_symplectic_euler` asserted only `!= 0` — unconditional
- [ ] **21 assertions use `f64_from(1)` — a literal tolerance of 1.0** — on quantities whose exact
      value is 0 or 1
- [ ] "gell-mann hermitian" is a tautology that passes for every complex matrix
- [ ] `calc_integral_simpson` has no tolerance assertion anywhere; the existing ones admit ±49%
- [ ] `eigen_power`, `cga_point`, `sym_to_latex`, `opt_gradient_descent`, `hvec4_length`,
      `num_dst` — existence asserted, value never checked
- [ ] Raise `cyrius coverage` from **59%** toward the 80% target

### Performance  (each needs before/after rows in `bench-history.csv`)
- [ ] `delaunay_2d` rescans every triangle per inserted point — O(n²), 232 ms at n = 1,600
- [ ] `triangulate_polygon` tests every remaining vertex for containment instead of only reflex
      ones — 29 ms for a 1,000-gon that should be O(n)
- [ ] `solve_bicgstab` allocates `s` inside its iteration loop — the defect 2.6.15 fixed in LM/L-BFGS
- [ ] `_kd_partition` value-midpoint split costs O(n·log(range/min_gap)) on geometrically-spaced
      coordinates — 574 ms vs 38 ms at n = 50,000
- [ ] Benchmark the hot public functions that still have none, so regressions are visible

### Found during 2.7.0-C (new — not in the 2026-08-04 audit)
- [ ] **`eigen_qr` returns `HSB_ERR_NO_CONVERGENCE` at unit scale** for a well-conditioned
      symmetric 4×4 (numpy cond 6.85, eigenvalues −1.877, −0.278, 0.274, 1.387), even at
      `max_iter = 1,000,000`. Reproduced on **unpatched** source, so it is independent of the
      Householder work. Repro matrix bit patterns are in the 2.7.0-C verification record

### Found during 2.7.0-B (new — not in the 2026-08-04 audit)
- [ ] **`lib/hisab.cyr` is a tracked 591 KB copy of hisab's own 2.6.15 distlib bundle**, sitting in
      the directory CLAUDE.md reserves for "vendored stdlib + first-party deps ONLY — no project
      source here". Nothing references it: not `cyrius.cyml`, not a source `include`, not CI, not a
      script, not the docs. Nothing regenerates it either — the distlib gate writes
      `dist/hisab.cyr`. So it is stale by construction and, in a flat last-definition-wins
      namespace, a standing shadowing hazard: any harness that pulls it in silently gets a frozen
      2.6.15 definition of all 777 functions. **Decide:** delete it, or document why lib/ carries
      a self-copy. *(Cost real time this session — it was the first suspect for a benchmark that
      appeared to ignore a source fix.)*

### Refactor  (CLAUDE.md: third instance only, never speculative, same gates as new code)
- [ ] **`calc.cyr` and `calc_ext.cyr` both define the unprefixed public globals `F64_THREE`,
      `F64_FOUR`, `F64_SIX`, `F64_FIFTEEN`** — both are in the same bundle, so last-definition
      silently wins. This is the flat-namespace hazard 2.6.8 addressed for `ERR_*`, recurring
- [ ] Resolve the cross-module private-symbol coupling — five pairs reach into another module's
      `_`-prefixed internals, breaking the README's à-la-carte consumption path. **Decide:** ship a
      dependency manifest, promote the symbols, or drop the à-la-carte claim
- [ ] README's à-la-carte example does not compile — `vec2.cyr` and `vec4.cyr` are missing
- [ ] BVH node-layout header documents a 40-byte node against a 32-byte allocation, plus three
      other false statements about a 12-function, zero-coverage block

### Close-out
- [ ] Give **every** numbered finding in `audit/2026-08-04.md` an explicit disposition
- [ ] Fold the 2.6.14 memory-safety closures into `SECURITY.md` and `threat-model.md` (both owe an
      entry — tracked in doc-health as read-through outstanding)
- [ ] Supersede with a new dated audit once the above lands

---

## 2.7.x -- Feature line  (after 2.7.0 lands)

Carried forward from the original 2.7.0 plan, plus the items moved out of the 2.6.x line because
each needs its own design and benchmark pass rather than riding a repair release.

### Features
- [ ] Differentiable rendering math (autodiff through ray-surface intersections)
- [ ] Reverse-mode autodiff (Tape-based)
- [ ] GPU compute via soorat (feature-gated)

### Moved out of the 2.6.x line
- [ ] **Adopt `vec_sort_by` / `vec_select_nth`** (cyrius 6.5.4). Blocked on an API mismatch, not
      effort: the comparator receives element *values* via `fncall2`, while hisab sorts *indices*
      by dereferencing a separate `points` vector, and Cyrius has no closures. Needs a packed
      key+index encoding or a context-carrying comparator upstream. **2.6.15 already fixed the
      complexity** of both hot sorts (heapsort / hashed pairing); this item is only the
      *consolidation onto stdlib*, so it is now optional rather than load-bearing
- [ ] **SIMD the flat-array kernels** — `_opt_dot`/`_opt_norm`/`_opt_axpy`, the three L-BFGS
      sweeps and `_lext_dot`/`_lext_norm` are scalar while `f64v_dot`/`f64v_scale` take a runtime
      `n` (5–8× on comparable kernels in 2.3.1). Needs a benchmark for each kernel first — none
      of them is currently benchmarked, so there is nothing to prove a win against
- [ ] Any further work the 2.7.0 re-audit turns up

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
| 2.6.15 | 2026-08-03 | 17,100 | 34 | **P3 + P4 closeout — the 2026-08-03 audit is fully discharged.** Both O(n^2) hot paths rewritten, benchmarked before and after (rows in `bench-history.csv`): `halfedge_from_triangles` twin pairing 190.1 ms -> **1.2 ms** (-99.4%) via an open-addressed hash of directed edges, and `convex_hull_2d`'s insertion-sort pre-pass 22.1 ms -> **2.1 ms** (-90.3%) via heapsort (chosen over merge sort because its scratch would leak per call under the bump allocator; the comparator is inlined because Cyrius has no closures). Both verified output-identical to the old code. Levenberg-Marquardt's 4 loop-invariant buffers hoisted and its symmetric J^T J computed once; L-BFGS's 3 per-iteration buffers hoisted. Six documentation drifts corrected (BVH mis-attributed to `spatial.cyr`, Fletcher-Reeves vs Polak-Ribiere+, DST-II vs DST-I, `expr_eval` "aborts", `hodge_star_2form_4d`'s three contradictory `sign` docs, `collision_core`'s header naming a deleted file). **`num_ext` + `symbolic_ext` brought in — all 34 modules are now covered by a suite**; coverage 57% -> **59%**, files 34/35. 1127 |
| 2.6.14 | 2026-08-03 | 17,000 | 34 | **P1 closeout — the memory-safety tier.** Three of the four defect groups **crashed the process** pre-fix: reverting `complex.cyr`, `calc_ext.cyr` or `optimize.cyr` individually makes the suite exit 139. Capped constructors (`cmat_mul`, `cmat_kronecker`, `cmat_identity`, `cmat_inverse`) stored through `cmat_new`'s documented 0-on-failure return — reachable from operands *under* the cap, since `cmat_mul` of 1000×1 by 1×1000 asks for 1e6 and `cmat_inverse` doubles the width. `opt_bfgs`/`opt_lbfgs` sized `n×n` / `m×(2n+1)` buffers from an unbounded dimension (added `_OPT_MAX_DIM`, alloc checks, `HSB_ERR_ALLOC`). `calc_bspline`/`calc_nurbs` indexed control points **negatively** when `n_pts <= degree`. Adaptive Simpson bounded depth but not work (2^50 evaluations on a NaN integrand). `kdtree_build` recursed O(n) deep on coincident points — fine at 40k, **SIGSEGV at 60k**. `_opt_armijo` accepted NaN. `cx_div`/`cx_inv`/`cx_powf` used a 1e-6 cutoff instead of 1e-12 (squared vs unsquared tolerance). `FLOAT_RENDER_BUF` was 32 against a scratch reach of 45. Four fBm entry points returned NaN for `octaves <= 0`. **+`calc_ext`, `noise_simplex` into the suites**; coverage 54% → **57%**. 1093 |
| 2.6.13 | 2026-08-03 | 16,900 | 34 | **P0 closeout** of the 2026-08-03 audit — every remaining critical/high correctness defect, all in modules with **zero test coverage**. `svd_golub_kahan` composed the left Householder reflectors forward, returning U **transposed** so `A != U·S·Vᵀ` (8 of 9 entries wrong); now accumulated backward. `eigen_qr` applied `Gᵀ T G` instead of `G T Gᵀ` — the transpose of the rotation that zeroes the bulge — so it **never converged for n ≥ 3** (NO_CONVERGENCE on a symmetric 3×3 at 100k iters) and paired wrong eigenvectors at n = 2. `su2_exp`/`su2_log` moved to half-angle, restoring `su2_to_rotation_matrix(su2_exp(ω)) == so3_exp(ω)` and fixing `se3_exp`, which had built R and t from angles a factor of 2 apart. `einsum` accepted only labels `a`–`h`, so **every example in its own header** was silently mis-parsed — it returned the trace (5, not 19) then segfaulted; alphabet widened to `a`–`z` with real validation. Three interval enclosure-soundness violations fixed (`ivl_sin` under-approximated across extrema, `ivl_sqrt` returned `[0,NaN]` which `ivl_contains` treated as universal, `ivl_div` missed `-0.0`). **4 of the 8 never-tested modules brought into the suites** (`einsum`, `lie_ext`, `mat3`, `linalg_precision`); coverage 50% → **54%**. 1063 |
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
