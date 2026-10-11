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

### **[3.3.6]** — instruments: benchmarks and measurement tooling

**Carried from [3.3.5] (rows not finished there).**
- **Box-corner contact normals**: the "regressed in 3.3.4" premise did not reproduce. On 5,000
  quaternion-box corners 3.3.4 sits at or below 3.3.3 (max 2.27e-5 against 0.192; p99 4.84e-7
  against 5.44e-7), the 1.79e-6 → 1.01e-5 figure was one 1,000-pair sample's maximum, and an
  independent 6,000-sweep review found the maximum swings with the seed in both directions. The
  outlier (2.27e-5 at a gap of 2.9e-10) comes from 3.3.4's face projection of a tiny triangle; the
  barycentric face point fixes corners (2.27e-5 → 2.8e-6) but breaks edges and capsule sides on the
  21,000-pair oracle sweep (wrong 34 → 174, edge-normal p99 4.7e-8 → 1.9e-5). Needs a contract for
  what a corner normal should be, or a better reference sweep.
- **EPA/MPR depth on a box inside a ball, second local minima**: after 3.3.5's crease walk, 4 (EPA)
  and 8 (MPR) of 1,000 concentric pairs still come back at the second-nearest face's depth (inside a
  sphere 48–520× the box's thinnest half-extent every face normal is a local minimum). 1–3 extra
  polish starts from the closest EPA faces left 3 / 1 above 1e-6 but cost +148% to +348% support
  calls on every polished hit.

**Found during 3.3.5, not repaired (not on the [3.3.5] rows).** Each was found by a 3.3.5 lane or
its reviewers and reproduced by an independent check on the 3.3.5 tree and on the 3.3.4 tag (all of
them answer the same on 3.3.4 unless noted), under cycc 6.6.14. Review and repair each here.

*Memory safety (high)*
- **`ad_tape_new(2^61 + 1)` wraps its sizes**: cap·40 and cap·8 wrap to 40 and 8 bytes, so it
  returns a live tape and `ad_var` pushes overwrite the next heap block (a canary after the tape had
  8 of 8 words overwritten). Guard the capacity against a derived bound before allocating, as
  CLAUDE.md's caller-sized-allocation rule asks.
- **`eigen_symmetric` never checks n against A**: diag(1, 2) with n = 1 and an 8-byte out_vals
  returns Ok and overwrites the word after out_vals (still on cyrius 6.6.16). Related:
  `eigen_symmetric([[1, NaN], [NaN, 2]])` returns Ok (2, 1) with the NaN erased (6.6.16 / ganita
  1.2.13: Err(−8), still not INVALID_INPUT), and a 2×3 input gives −8 (NO_CONVERGENCE) where
  `eigen_qr` gives −10: it maps every ganita error to NO_CONVERGENCE.

*Crashes and hangs on hostile sizes*
- `calc_nurbs` / `calc_bspline` with degree = INT64_MAX, n_pts = 1, n_knots = 0x8000000000000001:
  degree + 1 wraps and the call exits 139.
- `num_dct` / `num_idct(buf, 2^61 + 1)` and `num_dst(buf, 2^60)` hang (`_numx_next_pow2` overflows
  to 0) instead of returning `Err(HSB_ERR_ALLOC)`. Separately, DST/DCT take the O(n²) direct path
  for 524,288 ≤ n ≤ 2^28 (an estimate from num_ext's cost table, not measured).
- `EPA_MAX_ITER` = −1, −2 or −3 segfaults (the buffer guard checks the allocation result, which
  these values do not reach); folds into [3.5.0] `K003`.
- einsum stops counting operands at 8: `"i,i,i,i,i,i,i,i,i->"` (or a 9th operand `Q`) with
  n_tensors = 8 returns the 8-operand result 257; with n_tensors = 9 it returns 0.

*Wrong answers returned as Ok*
- `num_bisection` with an exact root at the left endpoint and f(b) < 0 returns a non-root: 1 − x on
  [1, 2] gives Ok(`0x3FFFFFFFFFFFF800`), −x on [0, 1] gives Ok(`0x3FEFFFFFFFFFF000`) (f_lo = ±0 is
  not > 0, so no swap). And a NaN f(mid) is read as ≥ 0: f = x − 1.5 but NaN on (1.2, 1.8) over
  [1, 2] returns Ok(`0x3FF3333333333800`), where f = −0.3.
- `eigen_power` loses a dominant direction with no check tripping: upper-triangular row-major
  (`BC6484B2B4CC260A E3464FFBBC3B1503 F7C5C0C568914462 0 10F3BEB8C474F8B2 7382BFC15C2B84DD 0 0
  44E0057FCD2D629A`) returns λ = a00 = −8.9e-18 where λ1 = a22 = 6.05e23 (its eigenvector's third
  component is below 2^-1074); |trace|/n = 2.0e23 ≤ ρ(A) would catch it. Also 46 Ok answers with a
  complex or tied dominant, non-normal accuracy (one row 4.5e-6 off), and 3.3.4's larger-only rule
  accepting a weakly converged wide answer from above (814 of 4,000 of one family, both trees).
- `eigen_qr([[2^1023, 2^1023, 0], [2^1023, 2^1023, 0], [0, 0, 2^-1074]])` returns Ok with
  `0x7FEFFFFFFFFFFFFE` though the dominant eigenvalue 2^1024 overflows; `svd_golub_kahan` refuses the
  same matrix (NO_CONVERGENCE). And `eigen_qr` refuses a matrix whose eigenvalues fit — [[`0x7FC9EDB8D873A8E9`,
  `0xFFBD5FE19F6C5892`, 0], [same, 0, `0x2E`], [0, `0x2E`, 0]], λ ≈ 4.57e307: the balance multiplies
  by 4, an attempt "converges" with +Inf on the diagonal and the rescale refuses without a retry (22
  of 22,000 tridiagonals; the SVD retries and answers).
- `ivl_sin([NaN, 1])` drops the NaN and returns `ivl_sin([1, 1])` ([1, NaN] gives [NaN, NaN]); the
  module's rule is that a NaN operand propagates.
- Quadrature on a width that underflows: f ≡ 2^52 on [0, 2^-1074] (truth DBL_MIN) gives 0 from
  gauss5, simpson, adaptive and trapez with steps ≥ 2; gauss5 on [0, 3·2^-1074] is 33% high; and
  **`calc_adaptive_simpson` on [0, 6·2^-1074] returns `0x8006666666666666`, the wrong sign** (the
  children's w/6 underflow to 0 and the correction (0 − whole)/15 is accepted). Also
  `calc_integral_gauss5(f ≡ 0, −DBL_MAX, DBL_MAX)` is NaN (sum·half = 0·Inf), and mid = (a+b)/2
  overflows for same-sign endpoints near DBL_MAX (f = x/DBL_MAX on [`0x7FE0000000000000`, DBL_MAX]
  gives +Inf; truth `0x7FD7FFFFFFFFFFFF`; simpson and trapez answer), in adaptive too.
- `opt_bfgs` / `opt_lbfgs` below a variable scale of ~2^-496 / 2^-500: s·y is subnormal, the cosine
  test passes it (‖s‖‖y‖ underflows) and rho = 1/(s·y) is +Inf; f = (x0−k)² + 1e5(x1−2k)² from
  (−k/2, 5k/2) returns Err(NO_CONVERGENCE) with NaN out_x at k = 2^-520. The scale sweep stops at
  2^-199.
- `world_to_screen` returns (0, 0, 0) for 0 < |w| ≤ 2^-1024 (MVP = 2^-1074·I, (3, 5, 7): right
  answer (1600, −1200, 4)) as well as for the NaN point [3.4.0] `D069` names; and
  `m4_transform_point(2^-1074·I, (3, 5, 7))` returns the undivided (0x3, 0x5, 0x7) — `D088` covers
  only w = 0.

*Geometry and collision*
- **`geo_ray_aabb[_face]` (and the AABB jet) report a false hit when b − o overflows on finite
  input**: o = (`0xFFE0000000000000`, 0, 0), d = (4, 1, 0), box min (−DBL_MAX, 1.5·2^1022, −1), max
  (2^1023, DBL_MAX, 1) gives t = `0x7FD8000000000000` on face 2 (truth: a miss). 3.3.5's BVH
  predicate orders the same class through its extended-real far test; the face routine does not.
- **An exit that underflows to −0 reads as t = 0**: o = 0, d = (2^1000, 1, 0), box x [−1, −2^-1074],
  y and z [−1, 1]: the face routine returns t = −0 on face 1 and the BVH predicate, query and jet a
  hit (truth: a miss; the exit is t = −2^-2074). A repair must tell an underflowed −0 from a true one
  (b = o with d < 0 also gives −0).
- `geo_ray_obb[_face]` has row 2's slab shape in its own code (a DBL_MIN parallel test and a
  reciprocal): OBB centre (1.5, 0, 0), half-extents (0.5, 2^1008, 0.5), identity; ray from
  (`0x3FEFFFFFFFFFE000`, 0, 0) along (2^-1030, 1, 0) returns 0 (truth 2^990).
- Ray/sphere and ray/capsule at large scales: a raw direction above ~2^511 (unit sphere at
  (0, 0, 5), d = (0, 0, 2^600): NaN, truth 2^-598; capsule (0, 0, 4)–(0, 0, 6), r = 1: NaN, truth
  3·2^-600 at the start cap; already at 2^511, where (5·2^511)² overflows; AABB, triangle and OBB
  answer), |o − c| ≥ 2^512 along the line of centres (origin (0, 0, −2^512) along +z: NaN; a
  perpendicular miss answers 0 correctly), a sphere centred at (0, 0, 2^600) (NaN, truth
  RN(2^600 − 1)), and a capsule (0, 0, 0)–(2^600, 0, 0) of radius 1 from (5, 3, 0) along −y (0,
  truth 2: |ab|² overflows; 2^511 answers). A ray within 2^-511 of parallel to a capsule at least
  2^458 long (where the barrel crossing sits) gets a spurious far-cap root from cancellation:
  capsule (0,0,0)–(1.5·2^511, 0, 0), r = 1, o = (1, 0.5, 0), d = (1, 2^-512, 0) gives t =
  1.5·2^511 on the end cap (truth: the barrel exit 2^511).
- `geo_ray_sphere`'s near root reaches 8.06 ulp on ordinary unit-size rays (o =
  (`0x4010893F256B9DCF`, `0x4010C561CABBB478`, `0x40013AD4D2FEA674`), stored d = (`0xbfe5a84d58140398`,
  `0xbfdd5a91b00775f6`, `0xbfe26d3e75975063`), c = (`0x3FF02DE3BD29868E`, `0x3FF04858D7DFAC0C`,
  `0xBFFA04F8FCBBBA54`), r = `0x3FF49817A5ACCB15` gives `0x40140055239EE7D6`). The loss is in
  disc = h² − a·c, not the root formula (the citardauq form gives the same): r² − |oc − h·d|² brings
  the maximum to 3.46 ulp and the median from 1.49 to 0.59 on a 1,500-ray sample.
- Top-binade siblings of 3.3.5's `geo_segment_direction` repair, triangle (2^1023, 0, 0),
  (2^1023, 2^-1074, 0), (2^1023, 0, 2^-1074): `geo_triangle_unit_normal` gives (0, 1, 0) (truth
  (1, 0, 0)); `geo_barycentric_coords` at (2^1023, 2^-1074, 2^-1074) gives the centroid (truth
  (−1, 1, 1)); `geo_closest_point_on_triangle` at (2^1023, 2·2^-1074, 0) gives (2^1023, 0, 0) (truth
  (2^1023, 2^-1074, 0)). Their 2^1022 copies are right.
- The jets read a −0.0 contact as a hit: the primals' miss sentinel is +0.0 and they test
  `f64_lt(t, 0)`, so −0.0 passes as a real t, and the jets test integer `t == 0`. Triangle
  (0,0,0), (1,0,0), (0,1,0) from (0.25, 0.25, 0): along +z the primal returns −0.0 and the jet a hit,
  along −z +0 and a miss; the plane, AABB and OBB jets too. A one-token repair (`f64_eq(t, 0)`) keeps
  the suites clean.
- `geo_jet_triangle` refuses a hit its primal accepts in a band one subnormal ulp wide near
  |a| ≈ 2^-1024 (s = `0x1FF1F5FAB7CD0488`, raw direction (0, 0, −`0x3FE964EF4831B102`)).
- `hquat_rotate_vec3((0, 0, 1, 0), (2^1023, 0, 0))` returns (−Inf, NaN, NaN); the answer is
  (−2^1023, 0, 0) (2^1022 is right).
- `gjk_epa_3d` on crossing segments (`_mpr_mkbox(0,0,0,1,0,0)` against `_mpr_mkbox(0,0,0,0,1,0)`)
  returns depth 0 with normal (−√½, −√½, 0), in the plane of the Minkowski difference; only ±z
  separates them (the test's own comment says ±z).
- `geo_closest_point_on_sphere` breaks scale covariance by up to 1.6 ulp: `hvec3_length`'s scaled
  path rounds the length one ulp differently from the plain root, and 1/len is subnormal past 2^1022
  (point (`0xBFF3C601805D43CD`, `0xBFFF6E3BAC9D7052`, `0x3FD5A5E52B0020B5`) against its 2^1021 and
  2^1019 copies). Cosmetic.
- **`gjk_intersect_3d` / `gjk_epa_3d` miss shallow overlaps** that `mpr_*` find: of 300 sphere pairs
  overlapping by 1e-6 / 1e-9 / 1e-12 / 1e-14 of (ra + rb), gi and ge miss 0 / 98 / 274 / 296 (a
  further 54 / 10 / 4 return ge = 1 with depth exactly 0); capsule/sphere 0 / 4 / 101 / 156. At 1e-9
  the overlap is ~5e6 ulp, so rounding does not explain it: GJK never encloses the origin and the
  certificate re-read and touch probe prove neither touch nor separation. Repro: spheres
  (0.7601015027223987, −0.8265634947301563, 0.21170376743975972) r 1.1782736506544615 and
  (1.9374603492476101, −0.7857383867485482, −1.997692684351919) r 1.3255780295896167 (overlap 1e-9
  relative): gi = ge = 0, mi = mp = 1.
- Depth of a small sphere nested in a large one: `mpr_penetration` up to 3.07e-4 relative and
  `gjk_epa_3d` 6.8e-6 (a = (`0xBFE3AEB703D0C3C2`, `0xBFECCEB071DE714A`, `0xBFB32B6419F274C0`) r
  `0x3FDF67A431C80234`, b = (`0x3F93F87F29711340`, `0x3FBC6F30C4A211A0`, `0xBFA7A225B97F2FE0`) r
  `0x4020B82DA608D15E`: exact 7.656167310709249, mp 7.658516096461451). Rare outside that shape.
- `time_of_impact` misses when the impact time is deep-subnormal: spheres of radius 2^-600 at the
  origin and 3·2^-600 apart with |v| = 2^e miss once t < 2^-1074, and oblique approaches miss even at
  representable t (2^-1055, 2^-1060, 2^-1070 miss where 2^-1065 and 2^-1050 hit).
- Collision residuals of the [3.3.5] repairs (measured, accepted as documented): depth at high
  aspect is wrong at every scale from aspect ~2^14 (a sphere 0.18 deep in the side of a box 2^62
  long gives depth 0; 3.3.4 the same at unit scale; from 2^100 about 3% of overlapping pairs that
  3.3.4 answered right at one particular scale are now depth 0 at every scale); verdicts whose gap
  is within 1 ulp of the pair's largest coordinate move both ways (107 right → wrong, 100 wrong →
  right on one 4,000-pair family); a misrepresentative first sample costs its rerun (2^-200-deep box
  pairs: 772 / 820 support calls against 334 / 338); and `mpr_*` pay the shallow-portal distance
  query on genuine near-tangent overlaps (+79% to +178% support calls for `mpr_intersect`, +37% to
  +50% for `mpr_penetration`, by family).

*Constants and tests*
- `_SH_Y20` = `0x3FD42F601A8C679B` (src/color.cyr) is 0.7067 ulp above √(5/π)/4; the correctly
  rounded constant is `…679A`. check-constants passes because the comment carries nine digits.
  Only band 6 uses it, and the band-6 pins record the bits a repaired constant gives.
- The tests helper `_dl_non_ccw` (modules_b) uses a float signed area and misreads thin triangles;
  dead helpers `_dl_tiles_hull` (modules.tcyr) and a se3-equality helper (modules_b); the EPA
  sphere/box overlap depth asserted to 1e-8 where it is 1.4e-12 off; three tests assign the public
  vars `GJK_MAX_ITER`, `EPA_MAX_ITER` and the stdlib's `ALLOC_MAX` (and restore them), which [3.5.0]
  `K003`'s enum conversion will break at compile time.

**The instruments.**
New benchmark rows register LAST (`tests/hisab.bcyr:1398-1408`).
- **Unbenched modules**: 11 of 35 have no row: calc_ext, diffgeo, interval, lie, lie_ext, mat3,
  noise_simplex, ode, optimize, symbolic, symbolic_ext. The bench calls 0 of `linalg_ext`'s 25 fns
  and 0 of `f64_util`'s 4. So the LM/L-BFGS hoists and calc_ext's costs have no number. `D114`
- **Costs measured only in scratch**: `num_is_prime` above 2^62 (its row uses 1000003),
  `num_pollard_rho`/`num_crt`/`num_modinv`/`num_divisor_sigma`, `perlin_3d`, `simplex_*`, and
  `dual_pow` at x³. `D115`. 3.3.5 adds to the list: its repairs were timed only in same-binary
  scratch harnesses (CHANGELOG 3.3.5 *Performance*), so `hvec3_angle`, `ivl_width`, `perlin_2d`,
  the Delaunay circle and mixed-magnitude sets, oblique rays through `bvh_query_ray`,
  `cga_blade_inverse`, `eigen_power` and the collision tangent / near-tangent / 2^600 pairs have no
  bench row; register them.
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
- **SVD/eigen B-side lift on first attempts** (moved from [3.3.5], whose row deferred it here):
  lifting a sub-DBL_MIN rotation on the bidiagonal too fixes 27 targeted wrong rows (up to
  1.75e13 eps) and moves 2 rows slightly (3.18 → 6.61, 1.6 → 1.72 ulp). Decide it with `D001`.
- **3.3.5's last-attempt floor is normwise only.** When no headroom is left, a failed
  `svd_golub_kahan` / `eigen_qr` gets one last attempt that zeroes couplings below DBL_MIN × the
  largest entry; on inputs 3.3.4 refused, a decoupled block below about DBL_MIN·max can lose all
  relative accuracy, sign included (an eigen_qr 5×5 returns its block's diagonal, 6.180e-101 and
  2.718e-101, where the truth is 1.1729e-100 and −2.8306e-101; block relative error median 31% svd,
  54% eigen on 300 / 200 such inputs). A reviewer's prototype — apply the floor only on a stall
  (more than 2n passes without deflation) and only to the active window — answers the same rows
  with block errors of 2e-14 / 1.8e-13, but moves one probe row to ~2^20 ε·max; re-measure it here.
  Also: with the last attempt in place the growth constants decide only the last scale tried, and
  4n returns some subnormal-grid values more accurately than 8n (a 25u eigenvalue exact at 4n, 24u
  at 8n).

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
7. **Three [3.3.5] rows landed differently from their text** (CHANGELOG 3.3.5 has the evidence):
   - `num_bisection` refuses a NaN f(a) / f(b) but still bisects ±Inf ones (the row said
     "non-finite"; refusing ±Inf broke four correct answers, e.g. ln on [0, 2] → exactly 1).
   - `calc_adaptive_simpson`'s floored acceptance is kept: every stricter rule measured either kept
     the aliased repro wrong or walked other integrands past 200M evaluations. A tripwire pins the
     wrong 1.1 the aliased integrand returns.
   - Overflowing quadrature now takes the walk its in-range twin takes: calls that returned +Inf
     after 5 evaluations can run past 2·10⁹ evaluations (> 60 s), the [3.4.0] `D070` class.
   Accept, or schedule a different outcome.
8. **cycc sizes a global `var X[N]` as 8·N bytes**, a local one as about N (measured on 6.6.14:
   `var A[8000]` adds 63,936 B to the binary over `var A[8]`; two consecutive global `[100]` arrays
   sit 800 B apart, two locals 112 B). CLAUDE.md's "`var buf[N]` = N bytes" holds for locals only.
   hisab declares no global array buffer. File upstream, amend CLAUDE.md, or both?

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

✅ **Both are FIXED UPSTREAM in cyrius 6.6.16** (recorded by cyrius 2026-10-05). ⛔ Do not move the
pin until cyrius 6.6.16 is tagged and out. Neither needs an in-tree change; what hisab may adopt once
its pin is ≥ 6.6.16 is its call:
- **Top-level `fncallN` on a capturing closure** (cyrius bite srca-2, its H1): a top-level `fncallN` /
  `callptr` now opens its own stack frame and takes the same closure-aware lowering as one inside a
  fn. The filed repro exits 0 on x86_64, aarch64, macOS (arm64 and Intel) and Windows. Once pinned
  ≥ 6.6.16, `src/autodiff.cyr`'s 3.3.4 recipe note ("invoke such a closure from inside a fn") can be
  relaxed, and so can the "Keep those closures inside a fn" line in the `sym_eval_interval`
  implementation notes above. Keep the note while any consumer pins below 6.6.16: there the
  top-level `fncall2(grad, x, g)` still exits 139.
- **Closure `: stack` return booked on the enclosing fn** (`D082`; cyrius bite srca-1, its H2): cycc's
  post-parse pair scans now skip a closure body, so `return f(..)` inside `|x| { .. }` belongs to the
  closure. The repro's D1–D3 rows build clean and run 0 (C1–C3 unchanged). Once pinned ≥ 6.6.16 the
  recipe may return `ad_grad_into(...)` from the closure directly again (the pre-3.3.1 form) —
  but return a pair on EVERY path of that closure: 6.6.16 judges a closure as its own fn and warns on
  a closure that returns a pair on one path and a single value on another. Below 6.6.16 that form is
  still a hard error, so the bind-both workaround stays while any consumer pins below.
- **To close**: at the 6.6.16 pin bump, run both repros on the old and new pins as a pair, then
  archive both records (their *Fixed upstream* sections hold the details).

⛔ **TWO hisab-filed ganita items are OPEN**, filed 2026-10-03 with the maintainer's approval (written
to `ganita/docs/development/issues/`, not committed there): `ganita_mat_svd` returns −1 for
rank-deficient square matrices, tiny-rotation column pairs and a rounding two-cycle
(`issues/2026-10-03-ganita-mat-svd-no-convergence-on-tiny-columns.md`; hisab answers these through
its own `svd_golub_kahan` since 3.3.4), and it accepts non-finite input
(`issues/2026-10-03-ganita-mat-svd-accepts-non-finite-input.md`; hisab refuses first).

✅ **Both ganita SVD items are addressed by ganita 1.2.13, folded in cyrius 6.6.16** (recorded by cyrius
2026-10-05, from cyrius CHANGELOG [6.6.16] *Downstream*; not re-measured against hisab's repros). ⛔ Do not
move the pin until cyrius 6.6.16 is tagged and out. `ganita_mat_svd` is a new algorithm (one-sided Jacobi
on a column- and row-pivoted QR — LAPACK dgejsv / dgesvj), so every SVD's bits change; it no longer refuses
finite matrices that have an SVD (zero / repeated rows, tiny rotations, the two-cycling 5×4), and a NaN or
infinite input is refused across ganita's linalg. **Return codes hisab can see change:** SVD
non-convergence is **−3** (was −1), non-finite input **−2**, `rank` passes the SVD's −2 / −3 through, and
`eigen_sym`'s allocation failure is −1 (was −2). At the 6.6.16 pin bump: re-run both repros on the old and
new pins as a pair; re-check `svd_compute` / `svd_truncated`'s −1 → `svd_golub_kahan` fallback (above,
`D001` / `D012`) against the new −3, and every place hisab maps ganita's integer codes; then archive both
records.

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
