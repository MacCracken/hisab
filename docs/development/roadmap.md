# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.6.14**. Stdlib `ganita` (6.2.x math umbrella) provides dense
> decompositions + transcendentals.

⭐ **This file is future-facing only.** Nothing below has shipped. The record of what *has* is:

| record | lives in |
|---|---|
| per-release changes, with the numbers | [`CHANGELOG.md`](../../CHANGELOG.md) |
| per-defect filings, in-tree and shipped | [`issues/`](issues/) and [`issues/archived/`](issues/archived/) |
| toolchain defects | `cyrius/docs/development/issues/` — **not this repo** |
| the evidence behind a closed item | [`audit/`](../audit/) |
| the evidence behind every open row below | [`audit/2026-09-30-deferment-audit.md`](../audit/2026-09-30-deferment-audit.md) |
| test counts and suite composition | [`../guides/testing.md`](../guides/testing.md) |

⚠ **Completed items are removed, not struck through.** This changed on 2026-09-11. The file had
grown to 1308 lines, about 95% of them history, and a backlog nobody can see the end of is not a
backlog. To learn why an item was closed, or that it closed differently from how it was scoped, read
the CHANGELOG entry for its release.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current — v3.3.4

**Status**:
- Suite **7073** across six harnesses: hisab 752, foundation 503, modules 1931, modules_b 1562,
  edge_cases 603, abuse 1722. (`modules_b.tcyr` is the second half of the per-module suite, split
  in 3.3.4 to stay under cyrlint/cyrfmt's 1028 KB input cap.)
- Constant gate **177/177**.
- Public surface **703** declarations (846 gate probes; all 616 non-public probes refused),
  enforced since 3.3.0 by the bundle's `private` marker.
- **80** benchmarks.
- **35** math modules in `[lib]`, plus the `src/visibility.cyr` marker.
- Toolchain **6.6.14**, sakshi **2.5.6**, ganita **1.2.11**.
- All gates green. Per-release detail is in [`CHANGELOG.md`](../../CHANGELOG.md).

Two releases broke the API, and each has a consumer guide:
- **3.0.0** moved to `Result<T, E>` with no deprecation window
  ([`../guides/migration-3.0.md`](../guides/migration-3.0.md)). **2.24.0 is the supported 2.x line.**
- **3.3.0** made the bundle private and removed the accidental public names
  ([`../guides/migration-3.3.md`](../guides/migration-3.3.md)).

⚠ **hisab ≥ 3.1.0 requires cyrius ≥ 6.6.3** (`public struct` + `#derive`). Measured again on the
3.3.4 bundle from pinned dirs:
- 6.6.2 refuses it with the known `#derive` error.
- Under 6.6.3, 6.6.6, 6.6.9, 6.6.10, 6.6.12, 6.6.13 and 6.6.14 a cross-module consumer program
  (perspective, `f64_fmod`, `cx_sqrt`, a ray/box hit, `cga_point`, a subnormal normalize, Delaunay
  and the SVD fallback on a singular 3×3) builds and runs; the outputs are identical on every pin
  except the perspective focal term below 6.6.9 (x87 `sin`/`cos`). On 3.3.0, all 511
  non-public probes were refused under 6.6.3, 6.6.4 and 6.6.12.
- ⚠ On 6.6.3 a private fn is still reachable through `&name`. That is upstream, fixed in 6.6.4;
  see *Decisions owed*.

## How to read this file

- ⭐ **Every open item sits under a pinned version heading or in an unversioned section.** A pinned
  version looks like `**[3.4.0]**`. *Demand-gated (3.x.x)* and *Parked* are the unversioned
  sections: they hold work with no driver yet. Nothing moves out of them without a consumer
  asking, and when one does, the item gets a pinned version here first.
- **The maintainer sets every release's number and contents.** The pins below are the plan of
  record, and they move only when the maintainer moves them.
- **Breaking changes are pinned inside 3.x like any other item.** 3.0.0 and 3.3.0 both broke the
  API. A break ships with a **Breaking** CHANGELOG section and a migration guide. A public name
  whose meaning changes is retired, not redefined, because consumers compile the bundle under
  their own pins.
- **`Dnnn` / `Knnn` are register IDs** in
  [`audit/2026-09-30-deferment-audit.md`](../audit/2026-09-30-deferment-audit.md). Each carries
  the sources (file:line), both skeptics' verdicts and the probes behind the numbers quoted here.

⚠ **An item's measurement is part of the item.** Rows here carry the number that scoped them.
Rows without one are how this file kept getting the size of a class wrong: for seven consecutive
releases a row named one or two sites and a tree-wide grep found five, six or fifty-one. **Grep for
the shape before sizing anything below.**

⛔ **An open row is not a verified one.** Every row below was re-verified on 2026-09-30 by two
independent skeptics, one told to prove it already done and one told to prove it is not hisab's
work. Premises still go stale: check the tree before believing a row in either direction,
including a row that says something is blocked.

---

## Pinned — 3.3.x patches (no public API change)

### **[3.3.5]** — the suites: assertions that cannot see

**Found during 3.3.4, not repaired (not on the [3.3.4] rows).** Each was found by the 3.3.4
lanes or their reviewers and measured on the 3.3.4 tree under cycc 6.6.14 unless noted. Review and
repair each here; repros are in the 3.3.4 CHANGELOG's notes or below.

*Geometry and collision*
- **Sphere/capsule quadratic, raw direction below 2^-511**: `a = d·d` is tested against
  `_GEO_F64_TINY`, so a raw (unnormalised, set through `GeoRay_set_direction`) direction below
  2^-511 is a miss for sphere and capsule (true t = 5·2^512 at |d| = 2^-512) while plane, triangle
  and AABB answer to ~2^-1022. Pre-scale d by a power of two in `_geo_sphere_roots` and the capsule
  quadratic (scalars, no allocation), with an A/B on the ray/jet rows.
- **Ray slabs read a direction component below DBL_MIN as parallel**: 241 of 79,291 finite box-ray
  pairs disagree with the exact answer (228 false hits, 13 misses), e.g. a direction normalised from
  (1e-310, 1, 0). Divide (b − o)/d instead of multiplying by 1/d, in `geo_ray_aabb` and
  `_bvh_ray_hits_aabb` together; every ray's crossing bits change, so it needs its own A/B.
- **GJK/EPA/MPR verdicts are scale-covariant only for scales in [2^-268, 2^340]** (24 pairs at every
  4th power of two); above ~2^516 `gjk_intersect_3d` reports separated shapes as overlapping.
  Divide the Minkowski supports by one per-call power of two, gated so unit scale stays
  bit-identical; own A/B.
- **EPA/MPR depth on a sphere concentric with a box**: relative depth error p99 1.8e-3 (EPA) /
  2.8e-3 (MPR), max 9.6e-3, over ~1,000 pairs; `_epa_polish` does not reach its tolerance on ridged
  supports.
- **`time_of_impact` residual**: on ~0.3% of capsule-side sweeps the distance query stalls at
  ~1e-8 × extent and the contact is found 1–2.6 tolerances in. Warm-started GJK or a generator-line
  bound, judged by the 21,000-pair oracle sweep.
- **`cga_blade_inverse` at the norm² extremes**: |nsq| < DBL_MIN returns the zero multivector for
  non-null blades scaled below ~2^-511; an overflowing nsq still returns NaN. Rescale by
  pow2floor(max|b|) as `cga_norm` does, counting each coefficient at least DBL_MIN/s.
- **`cga_sphere` when |c|²/2 and r²/2 both overflow**: ninf = Inf − Inf (3,060 of 25,228 sweep
  spheres, wrong before and after; 45 straddling ones moved NaN → ±Inf). Low value.
- **Delaunay, wide sets**: sets whose nonzero magnitudes span more than 430 binades run unscaled
  and still fail at extreme scales (the square plus (DBL_MAX, 1) comes back empty from 2^-540 down).
  Make each predicate scale-safe per call; own A/B.
- **Delaunay precision class at scale 1** (D055's reopen trigger): sets mixing magnitudes are wrong
  at scale 1 — two100 126/150, span200 107/150, circle 65/150 — including a 3-site non-degenerate
  triangle returned empty and an 18-site set emitting a clockwise triangle. Exact orientation in
  `_col_dl_orient`, an exact ghost term and D055's adaptive in-circle; A/B on the Delaunay rows.

*Vectors and Lie groups*
- **`hvec3_angle` near 0 and π**: the acos line gives 2.1e-8 for identical vectors (truth 0), 45°
  1 ulp high on 66,750 of 96,100 grid pairs. Kahan's 2·atan2(|a|b|−b|a||, |a|b|+b|a||) is the
  candidate; the plain atan2(|a×b|, a·b) variant moved 188 of 38,406 beyond 1 ulp.
- **Subnormal reciprocal bands**: where 1/|v| comes out subnormal (|v| in (2^1022, DBL_MAX], and
  normalize's (2^-1024, 2^-1022)), normalize, `su2_from_*`, `so3_from_axis_angle` and the Lorentz
  maps reach 3.7–15.3 ulp against a 1.8–9.1 ulp normal-range bound. Routing those bands to the
  exact power-of-two copy brings every maximum inside the bound (49–114 of 1,500 per band move from
  under 1 ulp to inside it).
- **`hquat_inverse` rescue precision**: up to 4.9 ulp in the rescued bands and 13.4 ulp for a
  subnormal |q|. Candidate: (q·2^k)^-1·2^k with an exact k; judge by the 2041-binade q·q^-1 sweep.
- **`hvec2_normalize` / `hvec4_normalize`**: the D087 class — 51 of 52 axis inputs 2^-1023..2^-1074
  are not unit, and (DBL_MAX, DBL_MAX) gives x = 0. Mirror 3.3.4's hvec3 repair.

*Linear algebra*
- **`svd_golub_kahan` / `eigen_qr` on the 2^-1074 grid**: a decoupled block of subnormal-grid
  values never deflates (52 probe rows; repros pinned in abuse.tcyr as tripwires). Candidate: a
  normwise deflation floor applied only after an attempt fails.
- **SVD/eigen B-side lift on first attempts**: lifting a sub-DBL_MIN rotation on the bidiagonal too
  fixes 27 targeted wrong rows (up to 1.75e13 eps) and moves 2 rows slightly (3.18→6.61, 1.6→1.72
  ulp). Decide under [3.6.0] D001.
- **`eigen_power`, absolute vector test**: a dominant direction whose share of the iterate is below
  tol and still growing is not seen; repros in the 3.3.4 comment (the larger-modulus class and
  [[7.03e-288, 0, -2.60e188], [0, 5.63e169, 0], [0, 3.82e-22, -2.69e168]], which returns −2.69e168).

*Numerics*
- **`perlin_2d` / `perlin_3d` past 2^63**: `f64_to(xf) & 255` leaves i64, so on cycc 6.6.8+
  (hisab's own pin) 1,650 of 2,000 `perlin_2d` and 1,999 of 2,000 `perlin_3d` calls with a
  coordinate in [2^63, 2^1021) violate the field's exact 256-periodicity (repro: x = 0x43E5EB851EB851EC,
  y = 0.3). Give each module its own copy of 3.3.4's simplex cell reduction.
- **`calc_integral_gauss5` and `calc_adaptive_simpson` overflow before scaling**: f ≡ DBL_MAX on
  [0, 0.5] gives +Inf where the exact value is 0.5·DBL_MAX. Gated rescaled sums, as 3.3.4 did for
  trapez/simpson.
- **`ivl_width` rounds to nearest**: [-2^-60, 1] gives 1.0 below the exact width; RU(hi − lo).
- **`ivl_sin` below cyrius 6.6.9**: x87 fsin (66-bit π) is not an enclosure near multiples of π;
  document the requirement or detect the old fsin once.
- **`cx_powf` non-finite leftovers**: n = ±Inf returns NaN + NaN i even on the positive real axis,
  and arg·n overflowing returns NaN + NaN i where r = 0; on consumer pins 6.6.3/6.6.6
  `ganita_f64_pow(m, ±Inf)` is NaN, so those pins keep NaN regardless; r = +Inf with an undefined
  angle returns NaN + NaN i where C99's cexp analogue is ±Inf + i NaN.
- **`num_bisection` accepts NaN endpoint values**: f ≡ NaN on [1, 2] returns Ok(1.0000000000000004),
  a fabricated root (the `f64_gt(f(a)·f(b), 0)` NaN-true guard). Refuse a non-finite f(a)/f(b);
  the max_iter-exhaustion Ok is a [3.4.0] contract row.

*Gates*
- **`check-measurements.sh` recall**: numberless outcome claims and some value shapes still pass
  unmarked (samples in the 3.3.4 gate notes); widen 'error' to one-decimal mantissas next to an
  observation verb and add 'stored'/'lands' to OBSERVED, measured the same way.

*Added by the 3.3.4 reviews (not repaired in 3.3.4)*
- **Finite shapes near DBL_MAX became collision misses in 3.3.4** (right → wrong): two boxes of
  half-extent (1.7e308, 1, 1) at the origin give 0 from `gjk_intersect_3d`, `mpr_intersect` and
  `time_of_impact` (3.3.3: 1 / 1 / t = 0); spheres of radius 9e307 at 0 and (1e308, 0, 0) too.
  `_gjk_mink_support` convicts a non-finite difference. Fold into the predicate scale-class repair
  above (scale supports before differencing, convict only non-finite support points).
- **Box-corner contact normals regressed in 3.3.4**: sphere onto a rotated-box corner, max angle to
  the closed form 1.79e-6 → 1.01e-5 rad, 147 of 1,000 more than twice the 3.3.3 error. Bisect the
  triangle closest-point repair's parts on that family.
- **`mpr_intersect` / `mpr_penetration` false contact near tangency** (pre-existing): the capsule
  and sphere 1.0003e-12 apart pinned in `tests/modules_b.tcyr` give 1 from MPR while
  `gjk_intersect_3d` gives 0.
- **Inverted-box reading is untimed**: 3.3.4 reads an AABB with min > max as its sorted box (the
  maintainer's choice); time `ray_aabb`, `ray_aabb_diag`, the BVH rows and a `geo_aabb_aabb` /
  `bvh_query_aabb` row in a same-binary A/B.
- **Top-binade cancellation** (wrong in 3.3.3 too): `geo_segment_direction((2^1023, 2^-1074, 0) ->
  (2^1023, 0, 0))` gives unit_x (truth (0, −1, 0)); halve only when a difference overflows.
- **`calc_adaptive_simpson`'s relative floor accepts aliased child panels**: f = 1.1 + 0.5·|sin 8πx|
  on [0, 1] at tol 1e-17 returns 1.1 after 9 evaluations (exact 1.1 + 1/π; 3.3.3 returned 1.259
  after 678,729). Decide what floored acceptance requires, with an A/B on the adaptive sweep.
- **`cga_point` double-rounds a subnormal q/2**: (0x1FF0000000000001, 0x1FF0000000000001, 0) gives
  0x4000000000000 where the correctly rounded value (3.3.3's) is 0x4000000000001; 11–17 of ~3,700
  subnormal results per seed. An error-free sum in `_cga_half_sq3`'s scaled branch.
- **`eigen_power`, flushed-block eigenvalue larger than A's dominant one**: the 5×5 in the 3.3.4
  `eigen_power` comment returns 13680.22 (the leading 3×3's) where A's is 13601.37; and the growth
  class [[7.03e-288, 0, −2.60e188], [0, 5.63e169, 0], [0, 3.82e-22, −2.69e168]] returns −2.69e168.
  Repair the growth test first, then re-measure "take the wide answer whenever a check trips".
- **ganita no-convergence repro lacks the rank-deficient and two-cycle witnesses** (hisab's ganita
  filing): generalize its `row()`/`ortho()` beyond 3×2 and add Z, [[1,2,3],[1,2,3],[4,5,6]] and the
  5×4 witness with their exact σ.


**Sweeps sized to floors that are gone.**
- **SVD and eigen sweeps** still stop at floors 2.18.0 removed:

  | sweep | stops at | measured today |
  |---|---|---|
  | 3×3 SVD | 78 decades | 300/300 correct |
  | 4×4 SVD | 9 decades | correct to 1e-300 |
  | eigen | 9 decades | 299/299 correct |

  The 4×4 reconstruction gate checks only `lp_r < 4`, under a dangling "recorded on the roadmap".
  Widen the sweeps and mutation-prove them against the 2.18.0 U-replay pair. `D005`, `D006`
- **`_lp_pow2_floor`'s +Inf arm** has been load-bearing since 2.22.1 for matrices spanning more
  than ~1533 binades (k = 480 goes wrong without it). Survivor (b) is therefore a coverage gap,
  not an equivalence. Add the fixture. `D004`
- **`so3_exp`'s bit-exact sweep** stops at 2^-529; it can now reach 2^-1022. `D093`

**Mutation survivors with no fixture.**
- The right bulge-chase Givens trigger (`linalg_precision.cyr:1010`); the left one is now killed.
  It needs the rotation made observable, through a 2-parameter helper with an A/B, or a recorded
  equivalence. `D007`
- The BFGS and L-BFGS bare-sign curvature tests and the L-BFGS gamma guard. They need an
  indefinite fixture with a near-orthogonal s/y pair. `D031`
- The Bluestein chirp reduction. It needs n ≥ ~512 and a tolerance between 6.5e-14 and 2.5e-13.
  `D026`
- Raw vs translated operands in `_col_orient2d_sign`. A distinguishing case now exists:
  (0,0), (1,0), (2,1e-30) with p = (0.1, 0.3). `D054`

**Vacuous or loose assertions.**
- The CGA point-nullity bound of 2^-26 sweeps on-axis points that are exactly null: 0 of 1001 are
  non-zero. `D047`
- The `dual_pow` derivative is checked through `LOOSE_TOL_M`, though it is bit-exact. `D083`
- 48 `f64_to(f64_round(…))` sites fell outside 2.13.0's scope. Several are exact, such as
  sqrt(4)′ = 0.25. `D099`
- The two tree-wide sweeps the 2.11.0 and 2.11.4 lessons imply were never run. One looks for
  tolerance-pinned assertions on since-repaired defects. The other interrogates guards directly
  outside the EPSILON and sqrt censuses. `D104`
- A 1-ulp additive error in `hvec2_add` is invisible, because its fixtures sit at magnitudes 4–6.
  A visible error needs sums in [0.5, 2). `D105`

**Never value-asserted.**
- 20 public fns are reached only by `assert_neq(f(…), 0)`. They include `m3_normal_matrix`,
  `m3_from_cols`, `se3_to_mat4`/`se3_from_mat4`, `t3d_to_matrix`, `sym_rewrite`, `sym_rule`,
  `pat_*` and `gamma_0`. 7 of the 9 SH bands (1–5, 7, 8) have no value assertion; band 0 has
  one, and band 6 only a zero-direction sum. `D100`
- `ad_grad_write`'s `n < 0` guard is uncovered (coverage 653/654). `D081`
- The box/box EPA depth is unpinned; it is exactly 1.5. `D067`

**Other.**
- **Fuzz**: `hisab.fcyr` runs 7 fixed buffers, because `cyrius fuzz` supplies no input. Add a
  seeded generator and an einsum-notation target; einsum's is hisab's only string parser. Also,
  `fuzz_quat_rotate` accepts a NaN `rel`. `D118`
- **The one compiler warning in the suites** ("comparison mixes f64 and integer operands",
  `foundation.tcyr:869`, the `m3_frobenius` comparison; still emitted on 6.6.14) has been left
  since 3.1.1. `D121`

### **[3.3.6]** — instruments: benchmarks and measurement tooling

New benchmark rows register LAST (`tests/hisab.bcyr:1398-1408`).
- **Unbenched modules**: 11 of 35 have no row: calc_ext, diffgeo, interval, lie, lie_ext, mat3,
  noise_simplex, ode, optimize, symbolic, symbolic_ext. The bench calls 0 of `linalg_ext`'s 25 fns
  and 0 of `f64_util`'s 4. So the LM/L-BFGS hoists and calc_ext's costs have no number. `D114`
- **Costs measured only in scratch**: `num_is_prime` above 2^62 (its row uses 1000003),
  `num_pollard_rho`/`num_crt`/`num_modinv`/`num_divisor_sigma`, `perlin_3d`, `simplex_*`, and
  `dual_pow` at x³. `D115`
- **Separated pairs**: `gjk_epa_3d` and `mpr_penetration` have no separated-pair row, and
  `time_of_impact` has no row at all. `D044`
- **`bench_batch` windows**: there are 40 call sites, not 39, and 80 labels, not 78. Since 6.6.9,
  a window below 100 × (clock + tick) reports its mean as both min and max. 40 of 80 rows print
  min = avg = max, 14 of them `bench_batch` rows. Decide the windows by same-boot A/B, and fix the
  harness comments (the ~488 ns floor). `D155`
- **`bench-history.sh`** has three gaps:
  - Its regime column cannot see an instrument change that keeps printing the floor line; that
    happened at 6.6.5 and 6.6.9. Write the clock tick to the CSV. `D112`
  - Rows are labelled with `HEAD` before the release commit, so 3.3.0's rows say `1f237a8`, whose
    VERSION is 3.2.2. Add a dirty flag or a VERSION column. `K004`
  - The declined load column's decision survives only in git history. Record it. `K004`
- **Mutation harness**: none is checked in, and the 38.4% survival figure from 2026-08-11 has
  never been re-measured. `D101`
- **Miscompile audit**: `threat-model.md:179-181` says every suite binary "is now audited for
  unwritten destination slots". That was a one-time objdump run in 3.2.0. Script it against the
  bundle compiled under the lowest live pin, or correct the sentence. `D119`
- **Measurement provenance**: 484 claim lines in 25 files are unmarked at 3.3.3 (487 at 3.3.2, 517
  in 27 before 3.3.2's comment sweep). `--ratchet` is red against the 417 baseline, with 8 files up, and CI runs
  `--diff` only on pull requests. The fix needs a
  call from the maintainer; see *Decisions owed*. `D106`

### **[3.3.7]** — performance, with before/after

- **`solve_gmres`** allocates seven work buffers inside the restart loop: the six named in 2.9.0
  plus `r`, for 8·(n + m² + 5m + 2 + k) B per cycle (computed from the alloc sizes, not measured).
  Hoist them behind an `alloc_used()` arena
  guard and a new GMRES row. `D016`
- **`_cga_scalar_of_geo`** is quadratic in occupancy: dense k = 32 costs ~21× k = 5. In the null
  basis only diagonal and (n0, n∞) pairs reach blade 0. A precomputed 32-pair list restores O(k),
  bit-identically up to the Neumaier order; re-verify the fabrication witness. `D050`
- **`_cga_geo_blades`** does `alloc(32)` per term pair in every CGA product: 1056 B per
  point × point and 33,024 B per dense product, 32,768 B of it scratch. `D051`
- **`m3_mul_vec3`**: its −6.5% was measured against the raw store tail, never against the natural
  accessor form, which crashed then. Time that form now on ≥ 6.6.5. The hoist stays for 6.6.3
  consumers either way. `D141`

---

## Pinned — 3.x minors (API and observable behaviour)

### **[3.4.0]** — error channels for the fabricated-answer class (breaking)

Each function below returns a value that is also a legal answer when the operation has failed.
- **How the break ships**: each gets a Result-returning replacement, and the old name is retired,
  not redefined. A **Breaking** CHANGELOG section and `docs/guides/migration-3.4.md` go with it.
- **Prerequisite met**: `check-result-migration.sh` sees forwarders since 3.3.1 (`D122`).
- **Before the API work, two audit tasks**:
  - Run a full multi-dimension sweep. The last one covered v2.11.0 (2026-08-11), and 3.0.0, 3.1.0
    and 3.3.0 have shipped since. Its "did NOT reach" list was never discharged, including the
    SIMD `f64v_*` paths and cross-module interaction. `D102`
  - Reconcile the 2026-08-11 audit's confirmed findings against the censuses, row by row;
    `solve_bicgstab` fell between the buckets. `D103`

**The functions.**
- **`svd_compute`** passes ganita's −1/−2 through as integers that alias hisab codes. It becomes a
  Result. `D012`
- **`solve_bicgstab`** and the Krylov family should report breakdown and non-convergence instead
  of returning the current `x`. Until then the contract is "an unconverged stop returns the last
  iterate" (the maintainer's 3.3.4 decision): 941 of 43,000 runs in the 3.3.4 sweep return a finite
  wrong iterate, and the exact-zero-t stop returns an unreported half step (36 wrong of 549).
  Operand scaling lands with the error channel. `D015`
- **`calc_adaptive_simpson`, `ode_backward_euler`, `ode_bdf2`, `ode_bdf`** should report
  non-convergence. `calc_ext`'s "no Result type" reason was false from 3.0.0, and `ode.cyr`
  documented a return value the code doesn't give; 3.3.2 corrected both comments. 3.3.3 made
  `calc_adaptive_simpson` return NaN for an unreachable tolerance (`D071`); the Result form replaces
  those NaNs. Also: `calc_adaptive_simpson` still walks its depth cap on integrands with zeros or
  evaluation noise above the 3.3.4 relative floor (5x^4 − 3x^2 + x − 1/4 at tol ≤ 1e-20, Gaussian
  tails: over 3,000,000 evaluations), and `num_bisection` returns Ok on max_iter exhaustion. `D070`
- **Plausible-value fallbacks**: `hisab_inverse_lerp` (returns 0), `world_to_screen`
  (`hvec3_zero`), `calc_monotone_cubic` (0), `calc_partial_derivative`, `calc_hermite_tcb`,
  `ode_bdf`. **svara** works around `calc_monotone_cubic`'s 0 with
  `if (r == 0) { return F64_ONE; }`. The dual sentinels stay, because they are a documented miss
  contract. Also: `f64_fmod(x, ±0)` returns +0 for every x, NaN and ±Inf included (C: NaN; kept
  when 3.3.4 made the rest of `f64_fmod` exact); `fbm_2d` / `fbm_3d` / simplex fBm return +0 for
  octaves ≤ 0; `dual_ln` / `dual_sqrt` / `ad_ln` / `ad_sqrt` map NaN to the (0, 0) miss sentinel;
  `num_pollard_rho(n ≤ 1)` returns n. `D069`
- **`expr_eval`** returns +0.0 and writes to fd 2 on an undefined variable or unknown tag. That
  write is the only I/O in library code. Return NaN or a Result, and drop the write. `D074`
- **`m3_inverse` / `m4_inverse`** return the identity for singular or NaN input.
  `m3_normal_matrix` inherits this, and `m4_transform_point` returns undivided coordinates at
  w = 0. `hquat_inverse(0)` and `hquat_normalize(0)` return the identity too (left unchanged in
  3.3.4, which repaired their subnormal inputs only). `D088`
- **`lyapunov_max`** returns `Ok` with exponent 0 when `iterations ≤ 0` or the Jacobian has no
  growth at all. Its doc block does not say so. `D032`

### **[3.5.0]** — constants and conventions (breaking)

- **23 `public var` constants that any consumer can write**: `GJK_MAX_ITER` and `EPA_MAX_ITER`
  (EPA sizes its buffer from the latter), `EPSILON_F64`, `F64_TINY`, `F64_POS_INF`, the
  `GEO_JET_*` / `GEO_CAP_*` / `GEO_FACE_*` tags, `AD_NONE`/`AD_FULL`, `HALFEDGE_NONE` and
  `CGA_NUM_BLADES`/`CGA_MV_SIZE`. Probed: `GJK_MAX_ITER = 0` set from a consumer takes effect.
  Convert them to enums, per CLAUDE.md's "enums for constants". `check-constants.sh` must parse
  enum bodies first, or three verified constants drop out of its population. `K003`
- **`cx_powf` with a zero base**: decide hisab's 0⁰. `cx_powf` gives 0, while `num_modpow` and
  `dual_pow` give 1. `D019`
- **`linearize_depth_reverse_z(0)`** returns 0, which means the eye, for the infinite far plane.
  IEEE and continuity with the subnormal neighbour both say +Inf. Three assertions pin the 0.
  `D096`
- **`t2d_apply`** takes an `HVec2` and returns `hvec3_new(x, y, 1)`, which tests read back through
  HVec2 accessors. None of this is documented (audit 2026-04-15, L4). `D090`
- **`sym_integrate`** returns 0 ("unsupported") for var-free integrands (`sin(y)`, `ln(2)`, `y^2`,
  `x^y`, `2^x`) and for `ln(x)`, though x·ln x − x can be built from existing nodes. The fix is
  additive. `D076`

### **[3.6.0]** — SVD relative accuracy

- **`svd_golub_kahan` on a graded bidiagonal `[[t,t,0],[0,t,1],[0,0,1]]`**: every singular value
  is right to 4.4e-16 relative through t = 2^-39 (1200-digit oracle). At 2^-40 it returns
  `rc = OK` with an exact-zero singular value where the truth is 2^-41.09, the middle one 19% low,
  and ‖A − USVᵀ‖_F = 9.1e-13 (~2900 ε, so not even backward stable). At every t from 2^-41 to
  2^-1022 two exact zeros come back. LAPACK gets all of these to full relative accuracy. (Through
  3.3.1 this row said "through 2^-36" and "from 2^-44"; 3.3.2 re-measured it.)
  - **Candidate causes**: the `EPSILON_F64 = 1e-12` deflation and zero-diagonal snap
    (`_lp_negligible_rel`, documented at `linalg_precision.cyr:687-691` as snapping values "where
    LAPACK's dbdsqr would keep it"), and the lack of dqds or zero-shift QR.
  - **Rules for the fix**: use a private tolerance and never redefine `EPSILON_F64`. Commit a
    graded-bidiagonal oracle sweep first, then A/B against 3.3.0. `D001`, `D009`
- **Since 3.3.4 this residual reaches `svd_compute` / `svd_truncated`**: they answer ganita's −1
  (rank-deficient square input) through `svd_golub_kahan`. On 7,012 such rows the worst
  ‖A − U S Vᵀ‖_F is 4,820 ε‖A‖_F (ganita's own answers reach 12.9), with S within 7.75 ε·σ₁.
  Re-run that probe as part of this row's A/B.
- **Commit the high-precision oracle for the subnormal-boundary rows.** `tests/hisab.tcyr:4209-4220`
  asserts only `rc`. The committed fixtures show 2 rows at 3 ulp, and a wider band reaches 14 ulp.
  `D003`

### **[3.7.0]** — the abaco bridge (additive; the pin is the maintainer's to move)

Scheduled proactively by the maintainer on 2026-10-03, so abaco can build its Rust-era plan
(`solve x^2 - 2 = 0`, interval-wrapped results) on hisab without having to ask. It adds public
names, so it needs a minor. It depends on [3.4.0] `D074` (`expr_eval` reads an unbound name as 0).

- **What Rust 1.3.0 shipped** (`src/symbolic/bridge.rs`, commit `bc09eb9`, unchanged through 1.4.0):
  `solve_expr(expr, var, opts{x0 = 0, bracket, tol = 1e-12, max_iter = 100})` ran Newton on the
  expression and its simplified derivative, with a bisection fallback only when Newton erred and a
  bracket was given (that fallback returned Ok(midpoint) on exhaustion); `eval_verified(expr,
  intervals)` evaluated a tree over intervals. `ExprValue` and its converters were transport only
  and stay out of scope (port-audit).
- **Port onto hisab's expression TREE** (abaco builds the tree; hisab has no tokenizer), each
  `#must_use`, returning a Result and writing `out` only on Ok:
  - `sym_solve(e, var_name, vars, x0, tol, max_iter, out)`: refuses an unbound name, a non-finite
    x0, tol <= 0 or max_iter <= 0 before evaluating; then Newton on `e` and
    `expr_simplify(expr_diff(e, var_name))` under `num_newton`'s contract.
  - `sym_solve_bracket(e, var_name, vars, x0, a, b, tol, max_iter, out)`: Newton, then bisection on
    [a, b] when Newton errs or leaves the bracket; refuses a non-finite f(a)/f(b) or equal signs, and
    returns `Err(NO_CONVERGENCE)` on exhaustion (unlike `num_bisection`).
  - `sym_eval_interval(e, ivars, out)`: interval evaluation through `ivl_*`; needs new public
    `ivl_cos`, `ivl_ln` and `ivl_pow` that are sound for every base sign (integer exponents by
    parity, negative exponents through `ivl_div`'s unbounded rule, non-integer exponents and ln only
    on lo > 0).
- **Do not port Rust's defects**: its Pow corner rule is unsound across 0 ([-2, 1]^4 gave [1, 16];
  the range is [0, 16]), and a free variable read as NaN (hisab's `expr_eval` reads 0, `D074`).
- **Implementation notes, measured 2026-10-03**: routing through `num_newton` with the evaluator as
  capturing closures built inside the hisab fn solves x^2 - 2 from 1.5 to 0x3FF6A09E667F3BCD,
  bit-exact under cycc 6.6.3 and 6.6.14. Keep those closures inside a fn (the filed top-level
  `fncallN` crash) and never `return` a `: stack` value from them (`D082`). abaco 2.4.12's bundle and
  hisab's compile together under abaco's pin 6.6.12 with no duplicate symbol.
- **Tests**: Rust's cases as bit-exact fixtures (√2), a bracket case that really reaches the
  fallback (x^2 - 2 from x0 = 0 on [1, 2]), the unbound-name and NaN-endpoint refusals, exhaustion,
  and [-2, 1]^4 ⊇ [0, 16].

---

## Decisions owed — the maintainer's

Nothing in this section is scheduled until the maintainer decides it.

1. **2.24.x support.** SECURITY.md lists 2.24.x as supported, but only the 2.24.0 tag exists, and
   it carries known defects:
   - the CWE-190 `num_*` overflow above 2^62
   - the non-terminating `num_pollard_rho` and `num_divisor_sigma(2^63−1)`
   - the `_sh_within` wrap
   - the `dual_pow` subnormal 0

   No consumer pins 2.24.x; the 2.x consumers are on 2.22.1 and 2.11.2. Backport, or mark it
   best-effort with an end version. A backport must not take 3.3.4's revert of
   `_cga_build_null_tbl`'s guard form: 2.24.0 pins cyrius 6.6.2. `D148`
2. **Minimum cyrius 6.6.4?** On 6.6.3, `&_private_fn` still reaches private bundle functions
   (`&_num_mulmod` probed), and six direct and three transitive consumers pin 6.6.3. 3.3.0 kept
   the floor at 6.6.3 and documented the hole. `D144`
3. **Measurement debt.** Either mark the claims added since the baseline, or run
   `--update-baseline`, which forgives them. 3.3.3 had 484 unmarked against the 417 baseline; 3.3.4's
   stronger detector (count and sweep claims) reports 525, about 25 of them newly visible older lines. Then decide whether `--ratchet`
   runs on push.
   `D106`
4. **aarch64.** `f64v_dot` is fused (`fmla`) on aarch64 and mul+add on x86. So `hvec4_dot` and
   `hquat_dot`, and through it `hquat_slerp`, give different bits on the two targets. CLAUDE.md's
   Language rule names only `f64v_axpy`/`f64v_fmadd`. Update the rule? An aarch64 test run is
   demand-gated below. `D120`
5. **CI consumer-pin matrix?** The public-surface and consumer-build gates run only under 6.6.14.
   The multi-pin build is a manual release step, which is how 3.3.0's `_SYM_EPS` leak under 6.6.3
   was caught. Automate it? `D110`
6. **Missing local tags.** The CHANGELOG has releases 3.2.0, 2.11.1, 2.9.3, 2.2.2 and 2.2.1 with
   no local git tag. The remote was not checked. `D164`

---

## Toolchain, tracked upstream

⚠ **Nothing is filed upstream without the maintainer's approval.** Once approved:
- cyrius defects go to `cyrius/docs/development/issues/`, and ganita's to
  `ganita/docs/development/issues/`;
- hisab keeps a record of its exposure in `issues/`, and closes it in `issues/archived/` with a
  paired measurement on the bump that fixes it. The upstream agents never edit hisab.

⛔ **TWO hisab-filed cyrius items are OPEN**, filed 2026-10-03 with the maintainer's approval
(written to `cyrius/docs/development/issues/`, not yet committed there):
- `2026-10-03-hisab-toplevel-fncall-capturing-closure-segv.md`: `fncallN` on a capturing closure
  at true top level exits 139 on every pin from 6.6.0 to 6.6.14. hisab is not exposed in-tree; the
  recipe note in `src/autodiff.cyr` tells callers to invoke such closures inside a fn. Record:
  `issues/2026-10-03-cyrius-toplevel-fncall-capturing-closure-segv.md`.
- `2026-10-03-hisab-closure-stack-return-blamed-on-enclosing-fn.md` (`D082`, until 3.3.4
  *Decisions owed* #4): a closure's `return <: stack call>` is booked against the enclosing fn;
  the repro exits 3 on every pin from 6.6.0 to 6.6.14. hisab is not exposed since 3.3.1. Record:
  `issues/2026-10-03-cyrius-closure-stack-return-blamed-on-enclosing-fn.md`.
- **When either closes**: run its repro on the old and new pins as a pair, then archive the
  record. Keep the recipe notes while any consumer pins below the fix.

⛔ **TWO hisab-filed ganita items are OPEN**, filed 2026-10-03 with the maintainer's approval (written
to `ganita/docs/development/issues/`, not committed there): `ganita_mat_svd` returns −1 for
rank-deficient square matrices, tiny-rotation column pairs and a rounding two-cycle
(`issues/2026-10-03-ganita-mat-svd-no-convergence-on-tiny-columns.md`; hisab answers these through
its own `svd_golub_kahan` since 3.3.4), and it accepts non-finite input
(`issues/2026-10-03-ganita-mat-svd-accepts-non-finite-input.md`; hisab refuses first).

✅ **ganita's `atan2` filing closed in 3.3.4** (ganita 1.2.11, cyrius 6.6.13): the tripwires are
flipped, the D132 rows are pinned as C99 values, and the record is in
`issues/archived/2026-09-30-ganita-atan2-signed-zero-and-nan.md` with its paired run. `D140`, `D132`

---

## 3.x.x — demand-gated (unversioned)

Each row names its driver. A driver arriving gets the row a pinned version above.

**Private flip and kernels.**
- **Flip the `src/` modules `private` too, not only the bundle.**
  - 3.3.0 made `dist/hisab.cyr` private through `src/visibility.cyr`, which is every consumer's
    view. The modules themselves stay unflipped, so a caller that `include`s `src/*.cyr` one at a
    time sees no boundary. Today only hisab's own suites do that.
  - ⚠ **Measured cost on 3.3.0, reproduced 2026-09-30**: under a full per-module flip the suites
    reach **68 private names at 390 sites**: `tests/modules.tcyr` 375 / 63, `tests/hisab.tcyr`
    14 / 4, `tests/hisab.bcyr` 1 / 1.
  - These are almost all deliberate white-box tests: Delaunay predicates, spatial node layouts,
    and the instrumentation counters that mutation-proven guards read (`_CGA_NULL_TBL`,
    `_GA_EPA_POLISH_COUNT`, `_CGA_NULL_COEF_BAD`). Those counters would need a public read-only
    getter, e.g. `cga_null_table_built()`.
  - **Driver**: an à-la-carte consumer that includes `src/` directly. None does; all ten use the
    bundle. `D156`, `D157`
- **SIMD the flat-array kernels**: `_opt_dot`/`_opt_norm`/`_opt_axpy`, the L-BFGS sweeps,
  `_lext_dot`/`_lext_norm`.
  - Every test calls these solvers at n = 1–3, where a 2-wide dot cannot win, so **the driver is a
    consumer-sourced `n`**. No live consumer calls `optimize.cyr` or `linalg_ext.cyr`.
  - 2.3.1 measured 1.6–6.5× (median ~2.3×), and much of that came from removing accessor calls,
    which don't exist here.
  - The buffers are exactly `alloc(n * 8)`, and `f64v_*` over-reads on odd `n`, so each kernel
    needs a pair loop plus a scalar tail.
  - A SIMD sum reorders `_opt_norm_ss` and breaks bit-identity with the scalar path. `D018`

**Numerics and algorithms.**
- **Public arbitrary-length FFT.** The Bluestein engine exists (`_numx_dft`, capped at 2^20), but
  prakash pins `num_fft(…, 3)` as `Err`, so exposing it changes a live consumer's assertion.
  **Driver**: a caller that needs a non-power-of-two DFT. `D027`
- **L-BFGS convergence.** It takes 674 iterations on Rosenbrock where BFGS takes 36. Both use the
  same Armijo search, so the gap comes from L-BFGS's γ scaling combined with backtracking that
  never extends α past 1. Measure that before choosing a Wolfe search. **Driver**: any caller of
  `opt_*`. `D030`
- **Complex arithmetic without a heap `HComplex` per temporary.** One 64×64 `cqr_decompose` uses
  32.9 MB of arena. **Driver**: a caller of `cmat_*`/`cqr_*`. `D011`
- **The interval entire/empty tag**, after **[3.3.3]**'s clamp. `D021`
- **`sym_integrate` by parts and linear substitution.** abaco, the planned user, declares no
  `[deps.hisab]`. `D076`
- **A position-dependent connection in `geodesic_rk4` / `parallel_transport`.** `D092`
- **kdtree ninther pivot**, for a consumer feeding attacker-controlled coordinates. `D060`

**Geometry.**
- **Jet tie flag**: name which faces tied, for subgradient callers. `D036`
- **The literal 1024-entry CGA table.** Measured at 2.24.0 on 6.6.2 (re-measure on the current
  pin first):
  - **Gain**: the first call drops from 258 µs to 11.9 µs, and the ~2% steady-state layout cost
    goes away.
  - **Cost**: +748 lines, and +2.93% consumer `code_size`.
  - **Driver**: a live `cga_*` caller. `D049`

**Autodiff.**
- **`ad_grad` reachability sweep.** A shared-tape Jacobian runs 5× to 47× slower than resetting
  the tape per residual, at m = 256 to 2048. `D078`
- **`ad_minimize` convenience.** The source says "Revisit if a consumer asks". `D079`
- **Fixed-size `dual_*` vector layer.** Declined in 2.19.0: forward mode wins only below n = 8,
  and the tape is 1.43× ahead at n = 16. `D080`

**Platform and infrastructure.**
- **aarch64 bit-identity run.** Cross-build plus qemu works on 6.6.12. attn11 gates hisab to
  x86_64 today. See *Decisions owed* #4. `D120`
- **Thread safety of the process-global tables**: Perlin, the einsum arena, the CGA table, simplex
  gradients, Delaunay scratch and the EPA counter. **Driver**: a threaded consumer. `K005`
- **Benchmark row isolation.** All rows share one never-freeing arena, and adding a row once moved
  another by −32% to −44%. Isolation needs an allocator mark/restore that the stdlib lacks. `D113`

**Rust-era features never ported.** The Rust 1.4.0 line had all of these. The trigger for each is
a consumer asking.
- Monte Carlo integration
- Atkin and segmented sieves (`num_sieve` caps at 10M)
- spline arc length, de Casteljau split
- sparse Cholesky/LU, SOR (a new fn: `solve_pgs` takes no ω)
- Voronoi, Mat4 SRT decompose (scale recovery; `se3_from_mat4` already covers a rigid R + t),
  plane–plane intersection `D158`
- dual quaternions, convex decomposition, differentiable rendering
- GPU via mabda (an opt-in include, not the Rust-era soorat) `D159`
- frustum (planned by kiran); spinors and complex Hermitian eigen/SVD (planned by kana) `D161`
- the parallel module (`thread.cyr` exists at the 6.6.12 tag), and serialization. The Rust AI
  module would conflict with SECURITY's "no network I/O in core". `D160`

**Docs.**
- **`docs/guides/usage.md`**, if onboarding needs more than the README. `D137`

---

## Parked / deferred (revisit when a driver appears)

These were evaluated and consciously deferred. Each carries the measurement that parked it,
because once the reason is gone, "deferred with a reason" and "forgotten" look the same.

- **SIMD `cross`** (from 2.3.1) needs lane shuffles.
  - `f64v_shuffle`, `permute`, `blend`, `swap` and `swizzle` are all still undefined on 6.6.14
    (probed 2026-10-03; the intrinsic-name lists are identical at the 6.6.12 and 6.6.14 tags).
  - The best shuffle-free formulation measured **38 ns vs 25 ns scalar (+52%)** on 6.6.2. That
    pair has not been re-run since. `D152`
- **`#pure` annotations** (from 2.3.4). The original premise, "unsafe CSE interaction", is
  refuted: there is no CSE to be unsafe.
  - Re-probed on 6.6.14 (2026-10-03): binaries are byte-identical with and without `#pure`, and
    two allocating calls return distinct pointers.
  - ⛔ **The objection to keep**: `alloc()` carries no `#alloc`. Annotating hisab's **286**
    `alloc()` call sites `#pure` would assert a falsehood with no compiler backstop. `D153`
- **Slices (`[T]` / `slice<T>`)** (from 2.3.4).
  - Re-measured on 6.6.12: checked slice indexing costs **3.6–4.2×** a raw `load64`, and the
    unchecked accessor **3.1–3.5×**.
  - The tag's `simd.cyr` has **zero** slice-taking forms, so slices cannot cover the SIMD hot
    paths.
  - The unchecked typed-array subscript (new in 6.6.12) refuses f64 elements by design: 6.6.13's
    M2 keeps it integer-only, and 6.6.14 still refuses `var a: f64[4]; a[2]`. It is no
    alternative. The cost figures above were measured on 6.6.12. `D154`

---

## Consumers

**Ten repos consume `dist/hisab.cyr` today, SHA-locked**, read from each `cyrius.cyml` on
2026-09-30:

| hisab tag | consumers |
|---|---|
| **3.2.1** | prakash |
| **2.22.1** | svara, naad, goonj |
| **2.11.2** | dhvani, attn11, ghurni, prani, garjan, nidhi |

| cyrius pin | consumers |
|---|---|
| **6.6.2** | goonj, attn11 |
| **6.6.3** | svara, naad, ghurni, prani, garjan, nidhi |
| **6.6.6** | prakash |
| **6.6.10** | dhvani |

prakash moved to cyrius 6.6.6 on 2026-09-23. It was the first consumer past the `Result` break, on
2026-09-15, calling `num_fft` through the `var rt, rv = …` idiom. `svara/src/spectral.cyr:246`
calls `num_fft`.

**Four more repos carry a copy transitively.** `cyrius deps` vendors it through another dependency,
and each compiles it under its own pin (read 2026-09-30 from each `lib/hisab.cyr` header):

| repo | hisab | via | cyrius pin |
|---|---|---|---|
| jalwa | 2.11.2 | dhvani | 6.6.3 |
| ranga | 2.11.2 | prakash | 6.6.9 |
| shabda | 2.22.1 | svara | 6.6.3 |
| shabdakosh | 2.22.1 | svara | 6.6.3 |

`ls ~/Repos/*/lib/hisab.cyr` lists all fourteen; the `[deps.hisab]` grep finds only the ten direct
ones.

⛔ **goonj and attn11 cannot take ANY hisab ≥ 3.1.0 until they move their cyrius pin.** cycc 6.6.2
refuses `public struct` + `#derive` (`#derive(...) applies to a struct or an enum`). Measured from a
dir pinned to 6.6.2: the 3.1.0, 3.2.1, 3.2.2 and 3.3.0 bundles are refused, while the 3.0.1 bundle
compiles. Under 6.6.3, 6.6.6, 6.6.10 and 6.6.12 the 3.3.0 bundle compiles and a consumer-shaped
program runs correctly. `D146`

⚠ **The pin matters to every consumer for four more reasons**, because a consumer compiles the
bundle under its OWN cycc:
- `m3_mul_vec3` keeps its hoisted z tail, because the natural form is silently wrong below 6.6.5.
  (`_cga_build_null_tbl`'s `if`-guard form went in 3.3.4: it guarded against a defect fixed in
  6.6.3, below every pin that can compile a 3.x bundle.)
- A zero produced by negation is −0 from **6.6.8** and +0 below.
- Since 3.3.4 a NaN spatial-hash coordinate goes into cell 0 on every pin (it followed `f64_to`
  before: cell 0 from 6.6.8, i64::MIN below), and a query with a NaN centre or radius returns
  nothing (3.3.3 returned every entry for a NaN radius). `D059`
- `dual_pow` is within 1 ulp from **6.6.10**. Below that it takes the older ganita's `pow`: 137
  ulp at 0.9^1024 on every pin from 6.6.3 to 6.6.9.
- From **6.6.13** (ganita 1.2.11): `cx_arg`, `cx_ln` and `cx_powf` take the
  C99 side of the branch cut for a −0 imaginary part, `atan2` of a NaN is NaN, and both-infinite
  arguments give ±π/4 or ±3π/4 (were NaN). `sinh`/`cosh` moved too, so `cx_sin`, `cx_cos` and the
  Lorentz builders change bits (hundreds of ulp near overflow and below 1; cosh is even bit for
  bit). `f64_tan` exists only from 6.6.13 (ganita's); hisab retired its own in 3.3.4.
- A consumer moving its pin to **6.6.13** or later must re-vendor `lib/math.cyr`: `f64_le`,
  `f64_ge` and `f64_trunc` became compiler builtins and reserved names, and the old file is
  refused ("reserved keyword"), loudly.
- On **6.6.0–6.6.5** a block-bodied closure in a top-level `var` is miscompiled (fixed in 6.6.6),
  so `autodiff.cyr`'s closure recipe must be built inside a fn there.

All of these were measured on the 3.2.2 bundle from pinned dirs and re-measured unchanged on
3.3.0's. The NaN cell was re-probed on the 3.3.1 bundle on 2026-09-30: an origin query returns the
NaN-inserted point under 6.6.12 and does not under 6.6.6.

⛔ **The remaining nine are still on 2.x, and the `Result` break fails silently.** A `Result` in
*argument* position degrades to its tag, and the `Ok` tag is 0, which equals `HSB_ERR_NONE`. So
`assert_eq(f(...), HSB_ERR_NONE)` **builds clean and tests nothing**. `D147`
- Hand a migrating consumer `scripts/check-result-migration.sh`, which exists for exactly that
  class and is mutation-proven.
- The reached surface is small (**34 distinct public fns in 8 modules**, per the Boundary table
  below), so each port is bounded.

⛔ **impetus, kiran, joshua, hisab-mimamsa and kana are NOT consumers.** They have no
`cyrius.cyml`; they are Rust repos needing a *port*, not a scheduling decision.
- aethersafha IS a Cyrius port now (`cyrius = "6.6.11"`), but declares no `[deps.hisab]`.
- abaco's README calls hisab a sibling, not a consumer (verified 2026-09-14).
- The ten live pins were re-read on 2026-09-30. `D162`

The table below is what these repos *would* use. It is kept because it is the planning surface,
not a statement that anything is wired up. Capabilities it names that hisab lacks are demand-gated
above.

| Planned consumer | Domain | Surface it will use |
|----------|--------|---------------------|
| **impetus** | physics | GJK/EPA, MPR, PGS, sequential-impulse, inertia, spatial |
| **kiran** | engine | projections, BVH, k-d tree, frustum (not in hisab) |
| **joshua** | simulation | DOPRI45, BDF, symplectic, optimize |
| **aethersafha** | compositor | projections, compositing, color |
| **abaco** | expression eval | symbolic integrate/LaTeX/patterns, interval |
| **hisab-mimamsa** | physics | tensors, Lie groups, diffgeo, CGA |
| **kana** | quantum | tensors, Lie groups, complex LA (no complex eigen/SVD), spinors (not in hisab) |

**Known caveat, carried forward: the no-hit path costs more since 2.9.x.**
- `gjk_intersect_3d` costs **+62% (box) / +57% (sphere)** on the no-hit path since 2.9.0, the
  price of no longer missing 134 genuine interior overlaps per 4,386 evaluations. Re-derived on
  6.6.2 by same-binary ABBA, and measured again on 6.6.12 at +62% / +58%.
- `mpr_intersect` inherits the same question: **+21% / +20%** on its miss path on 6.6.12 (the
  2.9.1 delegation to `gjk_intersect_3d`). `D041`, `D042`
- The no-hit path is the broadphase-common case. ⛔ But "a cheaper pre-filter" cannot be built
  inside the entry point. It receives only two support-function pointers, so it has **no cheaper
  information to filter with**, and hisab already ships the broadphase (`src/spatial.cyr`).
- **This is caller-side work, not a hisab roadmap item.** No live consumer calls `gjk_*` or `mpr_*`
  today.

---

## Boundary with Abaco

One row per surface, at the 3.3.1 surface: fn counts are `^public fn` in the module, `_` helpers
excluded, re-counted 2026-09-30 (identical at the 3.3.1 tag and in the 3.3.2 tree). The 3.1.1
counts this table carried until then were stale for five modules: symbolic 21 → 23, geo_diff
25 → 29, geo_advanced 41 → 43 (MPR moved in at 3.3.0), lie 30 → 32, lie_ext 26 → 25. `D163`

The last column is **measured, not assumed**: every call or `&` reference of a hisab public fn in
each consumer's own source (`lib/`, `dist/` and `build/` excluded; comments and strings stripped;
a name the consumer defines itself is not counted), re-derived on 2026-09-30 across abaco and the
ten live consumers. The names searched are the union of the current `public fn`s and every
non-`_` fn in the bundle the consumer actually vendors. Against the 2026-09-14 count, three cells
moved: naad's vec3 5/47 → 5/45 and num 5/9 → 4/8 (its only `num_rk4` is in a comment), and
prakash's num 2/7 → 2/9.

| Feature | abaco | hisab | live callers (symbols / call sites) |
|---------|-------|-------|-------------------------------------|
| `eval("sin(pi/4)")` — tokenize + parse a string | parses and evaluates | -- (no tokenizer) | -- |
| `expr_eval(tree, vars)` — evaluate an expression TREE | -- | symbolic.cyr (23 fn); ⚠ domain changed 2.11.2: `(-2)^3` was NaN for hisab's whole history, is -8 since | none |
| `sym_integrate(e, var)`, `sym_to_latex(e)`, patterns | -- | symbolic_ext.cyr (25 fn) | none |
| `ivl_*` interval arithmetic | -- | interval.cyr (15 fn) | none |
| `hvec3_cross(a, b)` and the vec family | -- | vec3.cyr | goonj 17/502, prakash 7/11, naad 5/45, dhvani 1/2 |
| `geo_ray_sphere(ray, sphere)`, BVH | -- | geo.cyr, geo_advanced.cyr | goonj 4/5 (`geo_aabb_new`, `geo_ray_new`, `bvh_build`, `bvh_query_ray`) |
| `calc_integral_simpson(&f, a, b, n, out)` -> `Result` | -- | calc.cyr, calc_ext.cyr | svara 2/11, naad 2/2, prani 1/2 (splines, `ease_in_out_smooth`) |
| `num_newton(&f, &df, x0, tol, max, out)` -> `Result`, `num_fft` | -- | num.cyr, num_ext.cyr | naad 4/8, prakash 2/9, svara 2/2, attn11 1/1 (`num_fft` in all four) |
| `dual_*` forward duals, `ad_*` reverse tape | -- | autodiff.cyr (15 dual + 23 tape fn) | none |
| `geo_jet_{sphere,plane,triangle,aabb,obb,capsule}` + partial readers | -- | geo_diff.cyr (29 fn) | none |
| `cga_*` conformal GA (null basis since 2.21.0) | -- | geo_advanced.cyr (27 of its 43 fn) | none |
| `so3_*`/`se3_*`/`bch_*`, `u1_*`/`su2_*`/`su3`/`lorentz_*` | -- | lie.cyr (32 fn), lie_ext.cyr (25 fn) | none |
| `f64_fmod` scalar helper (and `f64_tan` through 3.3.3; ganita's from cyrius 6.6.13) | -- | f64_util.cyr | `f64_tan`: goonj 1/3, naad 1/1; `f64_fmod`: garjan 1/2 |

**Hisab should never depend on abaco.** Abaco may optionally depend on hisab, and **today it does
not** (measured 2026-09-14, re-read 2026-09-30):
- abaco's `cyrius.cyml` has no `[deps.hisab]`, its 22 source files call 0 hisab symbols, and its
  own README lists hisab as a *sibling* library, "not a consumer". The "abaco" row in the
  planned-consumer table above is therefore a plan on hisab's side only.

**The reached surface across the ten live consumers is 34 distinct public fns in 8 modules**: vec3,
num, num_ext, calc, calc_ext, geo, geo_advanced, f64_util. The 2026-09-14 count said 35, because it
counted naad's `num_rk4`, which appears only in a comment.
- ghurni `include`s the bundle and calls nothing from it, and nidhi names hisab only in a comment.
- **0 of 10 reference `HSB_*` in `src/`.** prakash's `tests/wave_pattern.tcyr` asserts
  `HSB_ERR_INVALID_INPUT` as the Err payload of `num_fft(…, 3)`, which is the correct post-3.0.0
  idiom (re-read 2026-09-30).
- Eight of hisab's 35 modules have a live caller. The other 27, including everything the rows
  above mark "none", are reached only by hisab's own suites.

**No gate is owed here.** The only git dep is sakshi, and `deps --verify` already fails any
unreviewed dep.

⚠ `eval("sin(pi/4)")` being abaco's while hisab exposes `expr_eval` is not a contradiction:
`src/symbolic.cyr` takes a **tree**, not a string. hisab has no tokenizer at all.
