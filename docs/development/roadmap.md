# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.5.8**. Stdlib `ganita` (6.2.x math umbrella) provides dense decompositions + transcendentals.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current — v2.8.4

Suite **2271** across four harnesses, 40 benchmarks, constant gate **153/153** plus a
duplicate-global check. All gates green (`lint` 0 warnings, `fmt --check` clean across
`.cyr`/`.tcyr`/`.bcyr`/`.fcyr`, `vet` 2 deps / 0 untrusted, `fuzz` 1/0).

Shipped since 2.8.2: **2.8.3**, the eight-track audit-repair merge — the narrowphase now returns
the exact MTV (862/862 against a reference computed in Python `Fractions`, worst relative error
1.6e-9, against `mpr_penetration`'s prior 66/455 with 216 wrong signs), `time_of_impact` replaced
its fixed-step sampler with real conservative advancement, and `gjk_epa_3d` gained an out-param
contract on every return path. **2.8.4** corrected the DCT/DST dispatch heuristic.

**This retired most of what 2.9.0 was scoped to deliver** — see that section, which has been
rewritten to what actually remains rather than left describing a tree that no longer exists.

**Open against the tree:** of the 2026-08-04 audit's **42 findings**, the bulk were discharged by
2.8.3. The remaining gap is not a finding count, it is that
[`../audit/2026-08-04-v2.8.0-full.md`](../audit/2026-08-04-v2.8.0-full.md) has **no disposition
column** — its table is `| Sev | Dimension | Finding | Site |`. The doc asserts the standing rule
that "the finding list is the unit of disposition" and then provides nowhere to record one, which
is the same shape as the 2.7.0 failure it was written to prevent. Closing that is 2.9.0 exit
criterion 5, and it is a mechanism gap, not a roadmap item.

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

## Release train

What each release **delivers to a consumer**. Defects are not roadmap items — they are tracked in
`../audit/2026-08-04-v2.8.0-full.md` and discharged as a **precondition** of the release they gate,
not as line items here. If a finding does not change what a consumer can rely on, it does not
appear on this page at all.

| Release | Deliverable | Gated on |
|---|---|---|
| **2.9.0** | A narrowphase a physics engine can build on | sibling agreement + abuse harness + finding disposition (exact-MTV and arena bound shipped in 2.8.3) |
| **2.10.0** | Differentiable geometry (autodiff through intersections) | 2.9.0 |
| **2.11.0** | Reverse-mode autodiff (tape-based) | 2.10.0 |
| **3.0.0** | `Result<T,E>` API — breaking | 2.11.0 feature-complete |

---

## 2.9.0 — a narrowphase a physics engine can build on

**Most of this shipped early, inside 2.8.3.** The capability statement this section carried —
"a consumer cannot trust a contact" — is no longer true: the wrong-sign penetration depth, the
swept query that missed half its collisions and published a NULL normal, and the 1.23 MB of
never-freed arena per call are all fixed and measured. So is every item this section had
"deliberately deferred to 2.9.1" (spatial-hash bucket growth, BVH centroid recomputation, the
O(n²) DCT/DST — the last in 2.8.4). Rewritten below to what actually remains, rather than left
describing a tree that no longer exists.

**What a consumer can already rely on (2.8.3):** a penetration depth agreeing with the exact MTV
in sign and magnitude, worst relative error **1.6e-9** over 862 configurations in 35 shape
classes; a swept query using real conservative advancement; and `gjk_epa_3d` writing both
out-params on **every** return path, so a `1` return always carries a readable normal.

**Exit criteria — ALL FIVE MET.** 2.9.0 cuts.

1. ~~Narrowphase matches an independent exact reference with zero wrong-sign results and a stated
   worst-case error.~~ **MET (2.8.3)** — 862/862, worst 1.6e-9, reference computed in exact
   rationals and never from a previous hisab output.
2. ~~An abuse harness runs clean: no exit 139, no canary overwrite.~~ **HARNESS MET, ITS FINDINGS
   OPEN.** `tests/abuse.tcyr` — 577 assertions, 0 failed, exit 0, mutation-proven. It immediately
   surfaced **11 real defects on public entry points**, held in a known-defect register rather than
   deleted. The headline is precisely what this criterion names: `cqr_decompose`
   (`src/linalg_precision.cyr:1278`) has no squareness check, walks `i<m, j<n` storing into an m×m
   `out_R`, and **overruns the buffer while returning `HSB_ERR_NONE`** — a success code on top of
   the overrun (2 of 8 canary slots clobbered; the square control clobbers 0). **All 11 are now
   dispositioned:** 7 fixed with guards and assertions, 4 reclassified. `cmat_get`/`cmat_set` were
   deliberately left unguarded and marked `[BY DESIGN]` — the register's own reproducer did not
   reproduce, there is no error channel (the success value is a handle, and `cx_zero()` is
   bit-identical to a legitimate element), the identical unchecked accessor ships in
   `lib/ganita.cyr`, the validated surface one level up is complete, and guarding costs a measured
   +6.5%. Harness now 732 assertions.
3. ~~Arena per narrowphase call bounded **and asserted**.~~ **MET** — `assert_lt(alloc_used() -
   before, 262144)` per narrowphase call, plus a separate EPA arena bound.
4. ~~Every fix mutation-proven, with the mutation set including the shape family that broke the
   prior attempt.~~ **MET.** 54 mutations on the INTEGRATED tree — a different claim from each of
   the eight tracks proving its own in isolation, since the merge dropped one track's
   `geo_advanced.cyr` as subsumed and re-implemented another's fix against a different structure.
   45 were caught; several reproduced the CHANGELOG's failure counts exactly. The 11 that broke
   nothing were the point: 5 were real coverage gaps and are now closed, 6 were referred to 2.9.1
   as a design question rather than papered over with invented assertions. The sharpest: the
   delaunay ghost-direction invariant — the property the 2.8.2 critical was rebuilt around — had
   **zero** assertions, and the documented failing set passed every suite. Its replacement pins the
   in-circle determinant's leading coefficients, re-derived and validated in exact rationals over
   3,584 trials with zero sign mismatches. The EPA arena bounds went from 25x headroom (blind to an
   80 B/face/iter leak) to 12-35%.
5. Every one of the 42 audit findings has an explicit disposition. **MECHANISM IN PLACE, SIX
   FINDINGS OPEN.** The audit table now carries a Disposition column: **36 FIXED / 6 OPEN /
   0 REFUTED** of the original 42, plus one addendum row. It earned its keep on the first pass —
   an adversarial check caught `sequential_impulse` marked **FIXED when only half of it was**: the
   restitution repair landed in 2.8.3, but the friction block its own Site cell names is
   byte-identical to 2.8.1, and the friction impulse is **identically zero at every mu and every
   iteration count** (0 of 320 configs nonzero). No assertion mentions friction; every solver
   fixture uses friction = 0, so the branch is never entered. Corrected in place, and the
   mis-marking recorded rather than quietly amended.

   Worth stating on its own: **the entire low tier — 5 of 5 — was never scheduled and never
   worked.** Not deferred with a reason; simply never picked up, and invisible until there was a
   column to record it in. That is the 2.7.0 failure recurring, and it is the argument for the
   column existing at all.

   **CLOSED 2026-08-05.** The low tier was worked as one pass; all five dispositions in the audit
   table are now `FIXED`, each with a measured before/after and a mutation-proved guard. Two of the
   five were **understated by their own tier**: `quadtree_new`/`octree_new`'s uncapped `max_depth`
   was a measured **SIGSEGV** (exit 139 at `max_depth` 23,906, 8 MiB stack, 64 coincident points),
   now capped at `_SP_MAX_TREE_DEPTH = 1075` — a value taken from measurement (1 ULP apart at 0.5
   separates at depth 53; 0 against the smallest denormal at depth 1074) rather than picked round;
   and the `spatial_hash_insert` return-convention row is a **breaking API change**
   (`quadtree_insert`/`octree_insert` now return `HSB_ERR_NONE` / `HSB_ERR_OUT_OF_RANGE` instead of
   1 / 0), with a migration note in the CHANGELOG. Two sub-claims inside the
   allocation-inside-loop row did **not** survive: `src/tensor.cyr:217` was already hoisted
   (**REFUTED**, 1,272 B per call measured identical before and after), and that row's fourth site
   is truncated to `src/geo_` in the audit's own text in `70e7c8b` as well as HEAD and stays
   **UNRESOLVED** rather than guessed at. A brace-depth scan of `src/` while resolving it found
   **six allocation-in-loop sites the audit never named**, all in `solve_gmres`'s restart loop
   (`src/linalg_ext.cyr:386, 390, 395, 401, 402, 488` — `v_basis`, `h`, `g`, `cs`, `sn`, `y`);
   they are **not fixed** and are carried forward as the one item this pass leaves behind.

**The one narrowphase defect still open**, deferred out of 2.8.3 deliberately rather than missed:
`gjk_epa_3d` and `gjk_intersect_3d` **disagree** on 4 of 90 swept pairs — exact tangency in both
operand orders, and two coincident degenerate-point pairs. `gjk_epa_3d` is the correct one (two
touching convex sets do intersect), so `gjk_intersect_3d` is what needs repair. It is a hot public
entry point that consumers call per pair per frame, and the probe that fixes it measured at roughly
double the no-hit path cost on `gjk_epa_3d`, so it needs its own cost measurement before it lands.
The suite does **not** currently police this: the test group named "agrees with `gjk_intersect_3d`"
exercises that agreement only on overlapping fixtures.

---

## 2.9.1 — the deferred tier: everything 2.9.0 found and consciously did not fix

Every item here was **found, measured, and deliberately left** during the 2.9.0 arc. None is a
regression; all are pre-existing or latent. They are gathered so that "we decided not to fix this
yet" cannot decay into "nobody remembered." Each has an issue file or an audit row — this page
carries the schedule, not the detail.

**Correctness — latent, unreachable on today's paths:**
- [ ] `_col_dl_ic_g2`'s tie branch applies orientation twice (`s2 * cr > 0` where the determinant
      says `s2 > 0`) — `src/collision_mesh.cyr:292`, while the M³ branch at `:274` is correct for
      both signs. Exact-rational check: 0 mismatches on cyclic pairs, **8,737 of 8,737 on
      anti-cyclic**. Unreachable only because callers always pass CCW triangles. The real decision
      is whether to handle `cr == -1` or to **assert** the CCW precondition.
      → [`issues/2026-08-05-two-ghost-tie-branch-sign-rule-inconsistent.md`](issues/2026-08-05-two-ghost-tie-branch-sign-rule-inconsistent.md)
- [ ] `_col_in_circumcircle` adaptive exact predicate — carried since 2.7.0, still latent (0/180
      under real `delaunay_2d` geometry).
      → [`issues/2026-08-04-incircle-precision.md`](issues/2026-08-04-incircle-precision.md)

**Consistency — two siblings answering the same question differently:**
- [x] `mpr_intersect` vs `gjk_intersect_3d` diverge at exact tangency. **Re-measured, then fixed
      in 2.9.1.** The re-measurement came first, and it changed the answer: this was never
      `mpr_intersect` "on the strict reading" — its fallback `_epa_seed_gjk != 0` is `_gjk_core_3d`,
      the *pre-2.9.0 `gjk_intersect_3d` body*, so 2.9.0 simply repaired one call site of that logic
      and left the other two (`mpr_penetration` had it too, one case wider). Independent sweep:
      1,149 fixtures × 5 dyadic scales × both orders = 11,490 evaluations classified exactly in
      `Fraction`; the honest count is **50 of the 7,540 whose support functions are exact**, all
      tangencies, all `mpr = 0`/`gjk = 1`, 0 false positives. Both earlier reports were partly
      right: the 50 reduce to 6 configurations, 4 of them coincident degenerate pairs (the second
      reporter's finding) — but 2 are a point on the **corner of a solid box**, which nobody had,
      and which `mpr_intersect` answered **differently in the two operand orders**. 160 tangency
      evaluations between two shapes that both have volume: 0 disagreements, which is exactly why
      an axis-aligned sweep saw none of it. The asymmetry is what ruled out documenting it as
      strict. Now policed by three aggregates on the existing agreement sweep plus a 6-reproducer
      group with ±1-ulp negative controls.

**Coverage the mutation sweep could not close honestly:**
- [ ] U6–U11: six EPA repairs whose mutants produce **bit-identical** output because `_epa_polish`
      recovers the answer whenever `certified == 0`. There is no observable to assert on until it
      is decided whether that redundancy is intended. **A design question, not a missing test** —
      writing assertions here without answering it first would be padding.
- [ ] The `mpr_penetration` arena bound clears its injection by 32 B (0.125%); the other three
      bounds clear theirs by 65–129%. That fixture runs too few iterations for a per-face leak to
      show, so a differently-shaped partial regression could still pass. Needs a fixture with more
      iterations, not a tighter number.

**Process — the gate this arc argued for but could not finish:**
- [ ] `scripts/check-measurements.sh` is built and **not adoptable**. It flags measurement-shaped
      claims lacking a provenance marker (0 of 417 in-tree claims carry one), but an adversarial
      test planted ten realistically-phrased claims and it **missed eight** — its shapes only know
      `ns|us|ms` and comma-grouped byte counts, so "0.9 seconds before, 0.4 after" and "998400
      bytes" sail through. A gate that flags noise while missing real claims is worse than none.
      Either fix recall or drop it; do not enable it as-is. The motivating evidence stands: two
      false measurements reached committed text in two cycles, both caught by a reader, neither by
      a gate.

**Performance, demand-gated** (measured, filed, no consumer blocked):
- [ ] `delaunay_2d`, `_kd_partition`, `triangulate_polygon`
      → [`issues/2026-08-04-perf-delaunay_2d.md`](issues/2026-08-04-perf-delaunay_2d.md),
      [`issues/2026-08-04-perf-kd_partition.md`](issues/2026-08-04-perf-kd_partition.md),
      [`issues/2026-08-04-perf-triangulate_polygon.md`](issues/2026-08-04-perf-triangulate_polygon.md)

**Toolchain issues** are tracked upstream in the cyrius repo, not here:
`cyrius-cli-arg-clobbers-source`, `cyrius-for-empty-clauses`, `cyrius-interval-ident-lex`.
The 2026-08-05 `alloc_reset` defect filed during this arc was **fixed upstream in 6.5.8** and
archived — the workaround it required has been removed.

---


## 2.10.0 — differentiable geometry

Autodiff through ray–surface intersection, so consumers can optimise *through* geometry rather than
around it. The forward-mode dual infrastructure already ships (`autodiff.cyr`); this is the geometry
integration and the chain-rule coverage for `geo_ray_*`.

Driver: aethersafha (compositor) and the differentiable-rendering use case. Blocked on 2.9.0 only
because it consumes the same intersection routines.

## 2.11.0 — reverse-mode autodiff

Tape-based reverse mode for gradients of many-input scalar objectives, where forward-mode's
per-input cost is the wrong shape. Pairs with `optimize.cyr`'s solvers, which currently require a
caller-supplied gradient.

## Optional, demand-gated

- **GPU compute via soorat** (feature-gated) — no consumer has asked; parked until one does.
- **Adopt `vec_sort_by` / `vec_select_nth`** (cyrius 6.5.4) — consolidation onto stdlib, not a fix.
  Blocked on an API mismatch: the comparator receives element *values*, hisab sorts *indices* by
  dereferencing a separate vector, and Cyrius has no closures. 2.6.15 already fixed the *complexity*
  of both hot sorts, so this is now optional rather than load-bearing.
- **SIMD the flat-array kernels** — `_opt_dot`/`_opt_norm`/`_opt_axpy`, the L-BFGS sweeps,
  `_lext_dot`/`_lext_norm`. 5–8× on comparable kernels in 2.3.1. **Needs a benchmark per kernel
  first** — none is benchmarked, so there is nothing to prove a win against.

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

## 2.7.0 -- Re-audit & refactor  **RELEASED 2026-08-04**

Closed the 2026-08-04 re-audit: **35 of 41 findings fixed**, disposition recorded per-finding in
[`../audit/2026-08-04.md`](../audit/2026-08-04.md), close-out in
[`../audit/2026-08-04-2.7.0-closeout.md`](../audit/2026-08-04-2.7.0-closeout.md). Suite
**1127 → 1605**, coverage **59% → 71%**, benchmarks 28 → 35, constant gate 110 → 141 plus a new
duplicate-global check. Every fix mutation-proven except two that say so.

Four defects were found by *checking that a repair preserved behaviour*, not by any audit — the
biggest being `kdtree_within_radius` returning wrong counts on 274/400 queries after three
releases. Two findings had never been scheduled at all, surfaced only by the per-finding
disposition sweep. Three confirmed performance wins were **withheld** because each regresses on
some input class; their measurements and reviews are filed under `issues/`.

### Carried into 2.8.x
- [x] **`delaunay_2d` rewritten with triangle adjacency** (walk + flood-fill Bowyer-Watson) in
      **2.8.0**, after two failed attempts. The premise behind both — "cocircular is inherently
      expensive" — was **wrong**: the shipped code was not triangulating that class at all (1651
      triangles where 148 is correct, 206 non-manifold edges, 9 points dropped, 244x the hull area).
      Fixing the correctness made it **~1300x faster** there, and −88.6% on uniform, with growth per
      doubling 3.9x → 2.1x. Verified across 298 fixtures by an independent adversarial harness;
      5 new assertions pin count, point-usage and manifoldness
- [ ] `_col_in_circumcircle` adaptive exact predicate — latent (0/180 under real `delaunay_2d`
      geometry); needed only if the function is exposed publicly or the 10x constant changes
- [x] **Coverage 94%** (555/587) and **all 35 files referenced** — target was 80%, then 90%.
      Suite 1127 → **1765** across the 2.6.x/2.7.x arc

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
