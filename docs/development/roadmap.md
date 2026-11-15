# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.7.18**. Stdlib `ganita` (6.2.x math umbrella) provides dense
> decompositions + transcendentals.

⭐ **This file is future-facing only.** Nothing below has shipped. The record of what *has* is:

| record | lives in |
|---|---|
| per-release changes, with the numbers | [`CHANGELOG.md`](../../CHANGELOG.md) |
| per-defect filings, in-tree and shipped | [`issues/`](issues/) and [`issues/archived/`](issues/archived/) |
| toolchain defects | `cyrius/docs/development/issues/` — **not this repo** |
| the evidence behind a closed item | [`audit/`](../audit/) |
| the evidence behind the open rows | [`audit/2026-09-30-deferment-audit.md`](../audit/2026-09-30-deferment-audit.md) (`Dnnn` / `Knnn`) and [`audit/2026-10-31-v3.4.0-full.md`](../audit/2026-10-31-v3.4.0-full.md) (the 3.4.0 audits' IDs) |
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

## Current — v3.5.0

**Status**:
- Suite **16483** across eight harnesses: hisab 1350, foundation 1095, modules 2996, modules_b 1842,
  modules_c 2209, modules_d 2188, edge_cases 2044, abuse 2759.
- Constant gate **572/572** (41 skipped), read exactly: hex enum members, function-local constants and
  one-value expression comments included (floors: population 613, verified 572, enum members 156,
  globals 309).
- Public surface **729** declarations (887 gate probes; all 1692 non-public probes refused),
  enforced since 3.3.0 by the bundle's `private` marker.
- **505** benchmarks.
- **36** modules in `[lib]` (35 math modules and `heap.cyr`, the allocation scope), plus the
  `src/visibility.cyr` marker. Bundle `dist/hisab.cyr` 80,595 lines, 4,236,092 B; its sidecar names 9
  leaves.
- Measurement provenance: **8** unmarked claim lines (detector false positives), the baseline; CI runs `--ratchet` on every push since 3.3.6.
- Every public function has a comment block (692 of 692); `check-doc-blocks.sh --ratchet` fails on one without.
- Toolchain **6.7.18**, ganita **1.2.15**, sakshi **2.5.10** (a stdlib leaf since 3.4.2).
- All gates green, `check-miscompile.sh`, `check-test-names.sh` and `check-overcommit.sh` among them
  since 3.5.0. Per-release detail is in [`CHANGELOG.md`](../../CHANGELOG.md).

Four releases broke the API, and each has a consumer guide:
- **3.0.0** moved to `Result<T, E>` with no deprecation window
  ([`../guides/migration-3.0.md`](../guides/migration-3.0.md)). **2.24.0 is the supported 2.x line.**
- **3.3.0** made the bundle private and removed the accidental public names
  ([`../guides/migration-3.3.md`](../guides/migration-3.3.md)).
- **3.4.0** retired 27 names for Result-returning replacements, the error channels for
  the fabricated-answer class ([`../guides/migration-3.4.md`](../guides/migration-3.4.md)).
- **3.5.0** retired 16 more, gave `world_to_screen_checked` a depth-range parameter and made 23
  public constants enum members ([`../guides/migration-3.5.md`](../guides/migration-3.5.md)).

⚠ **hisab ≥ 3.1.0 requires cyrius ≥ 6.6.3** (`public struct` + `#derive`). Measured again on the
3.5.0 bundle from pinned dirs (2026-11-14):
- 6.6.2 refuses it with the known `#derive` error.
- Under every installed pin from 6.6.3 to 6.7.18, 37 toolchains, a cross-module program of 32 readings
  over the release's new and changed entry points builds with no warning, with the manifest's 17 leaves
  and with the sidecar's 9 plus `syscalls` and `io`, and runs; its output is byte-identical on all 74
  builds. `check-miscompile.sh` passes on the bundle under 6.6.3 and 6.7.18. The suites under 6.6.3
  fail 114 assertions and under 6.6.10 59, each the pin's own transcendentals, ganita's SVD, the INS5
  instrument or the carried `_geo_cross_exact` +0 (CHANGELOG 3.5.0).
  [measured: scratch pins/probe_main.cyr, both leaf sets per pin; scratch oldpin/run_oldpin.sh, 2026-11-14]
- 3.4.2's run: under 6.6.3, 6.6.4, 6.6.5, 6.6.10, 6.6.13, 6.6.20, 6.7.9 and 6.7.18 its cross-module
  program built with no warning and ran, bit-identical but for three DST values on 6.6.3–6.6.5
  (`num_dst`'s and `num_dst_into`'s alike), which take those pins' sin / cos.
- 3.4.1's run: under 6.6.3, 6.6.6, 6.6.9, 6.6.10, 6.6.12, 6.6.13, 6.6.14, 6.6.20, 6.7.9 and 6.7.18 a cross-module
  consumer program of 34 readings (a cross product, adaptive Simpson, bisection, `f64_fmod(1, 0)`,
  Pollard rho, `eigen_power`, SVD, `cx_powf`, FFT, an octree, `gjk_epa_3d`, a ray, `sdf_capsule`,
  `expr_diff`, `sym_rewrite`, BDF-2, BFGS, a dual) builds two ways, with the manifest's 16 stdlib
  leaves and with the sidecar's 7 plus `syscalls`, `string` and `io`, with no warning, and runs; under
  6.6.4 and 6.6.15–6.6.19 with the 16. The output is bit-identical on every pin except SVD's two
  singular values, which move by at most 2 ulp at 6.6.16 with the pin's own ganita.
- Earlier runs found `cx_powf(·, ±Inf)` NaN on 6.6.3 / 6.6.6 (their ganita `pow`), and `ivl_sin`
  ([−1, 1]) and the perspective focal term (x87 `sin`/`cos`) different below 6.6.9.
- A 3.4.0 merge build was refused under 6.6.3 alone (`duplicate variable` in `_sym_rewrite_single`,
  repaired before release): the class *Decisions owed* #4 is about.
- ⚠ On 6.6.3 a private fn is still reachable through `&name`. That is upstream, fixed in 6.6.4;
  see *Decisions owed*.
- ⚠ **The sidecar is compile-verified since 3.3.7** (`cyrius distlib` ≥ 6.6.18 writes the `# Requires`
  block and `dist/hisab.deps`): 7 leaves through 3.4.1 (math, alloc, vec, ganita, result, str, fmt),
  9 from 3.4.2 (`fnptr` and `string` added). A consumer that received `assert`, `bench`, `args`, `io`,
  `tagged`, `callback` or `syscalls` through hisab's sidecar must declare it.

## How to read this file

- ⭐ **Every open item sits under a pinned version heading or in an unversioned section.** A pinned
  version looks like `**[3.6.0]**`. *Demand-gated (3.x.x)* and *Parked* are the unversioned
  sections: they hold work with no driver yet. Nothing moves out of them without a consumer
  asking, and when one does, the item gets a pinned version here first.
- **The maintainer sets every release's number and contents.** The pins below are the plan of
  record, and they move only when the maintainer moves them.
- **Breaking changes are pinned inside 3.x like any other item.** 3.0.0, 3.3.0, 3.4.0 and 3.5.0
  broke the API. A break ships with a **Breaking** CHANGELOG section and a migration guide. A public name
  whose meaning changes is retired, not redefined, because consumers compile the bundle under
  their own pins.
- **`Dnnn` / `Knnn` are register IDs** in
  [`audit/2026-09-30-deferment-audit.md`](../audit/2026-09-30-deferment-audit.md). Each carries
  the sources (file:line), both skeptics' verdicts and the probes behind the numbers quoted here.
  **`AU1`/`AU2`/`AU3` IDs** (`AU2-G<n>-F<n>`, `SYM-`, `LIE-`, `SPC-`, `F-…`, `AU3-F<n>`) are
  findings in [`audit/2026-10-31-v3.4.0-full.md`](../audit/2026-10-31-v3.4.0-full.md), with their
  full repros, oracles and verification.

⚠ **An item's measurement is part of the item.** Rows here carry the number that scoped them.
Rows without one are how this file kept getting the size of a class wrong: for seven consecutive
releases a row named one or two sites and a tree-wide grep found five, six or fifty-one. **Grep for
the shape before sizing anything below.**

⛔ **An open row is not a verified one.** The rows carried from 3.3.x were re-verified on 2026-09-30
by two independent skeptics, one told to prove it already done and one told to prove it is not
hisab's work; the rows carried from 3.4.1 state their own verification. Premises still go stale: check the tree
before believing a row in either direction, including a row that says something is blocked.

---

## Pinned — 3.x minors (API and observable behaviour)

### **[3.6.0]** — SVD relative accuracy

- **`eigen_qr`'s 1e-12 deflation** (`_lp_negligible`) is what puts 251 eps·|λ|max into its answer
  on the 4×4 near the overflow line (3.3.6's row; the same matrix at 2^-100 to 2^-300 gives the same
  bits scaled), and an in-range family of its shape reaches 711 eps (median 0.97, p99 53). A private
  2^-52·(|d_k| + |d_k+1|) threshold takes both to ≤ 5 eps but moves the bits of 1,909 of 6,000
  in-range rows and costs +11.7% QR sweeps; LAPACK's form moves 3,752 and makes 5 sparse rows
  NO_CONVERGENCE. Decide it with `D001`; `eigen_qr` carries a ⚠ paragraph saying so.
- **`svd_golub_kahan` refuses some fitting matrices near the overflow line**: 425 of 3,030 sweep
  rows whose σ₁ fits within 9 units of 2^-53 of the line return NO_CONVERGENCE at the 3.3.3 rescale
  (the computed σ rounds past), as on 3.3.6.

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
- **Since 3.3.4 this residual reaches `svd_compute_checked` (`svd_compute` through 3.3.7) /
  `svd_truncated`**: they answer ganita's −1 (rank-deficient square input) through
  `svd_golub_kahan`. On 7,012 such rows the worst ‖A − U S Vᵀ‖_F is 4,820 ε‖A‖_F (ganita's own
  answers reach 12.9), with S within 7.75 ε·σ₁. Re-run that probe as part of this row's A/B.
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

#### Carried from the 3.5.0 close

Every row here is a [3.5.0] row that 3.5.0 left open, or the part of one it left, followed by what its lanes
found outside their rows. 3.5.0 worked its rows in two waves, each lane a repair, an independent review and a
second pass: L1–L8, then W1–W8 on what L1–L8 left. A lane ID names the report that measured the figure beside
it; the audit IDs are the rows' own. The lanes' scratch is deleted, so the repros are inline. *(no API)* and
(API) mean what they mean above.

**Accepted at 3.4.1 with evidence, and still not repaired.** Neither acceptance has been made: *Decisions
owed* #16, with 3.5.0's addition.

- **Collision costs against 3.3.7, not bought back** *(no API)*: 3.4.1's figures, `gjk_intersect_box_hit`
  1.69 → 1.93 µs, `gjk_intersect_tangent` 1.81 → 2.40, `gjk_epa_face_1e9` 17.4 → 28.3,
  `gjk_intersect_small_edge_1e11` 14.8 → 41.4, `mpr_intersect_band_1e6` 4.7 → 9.7, `time_of_impact_sph_hit`
  5.3 → 8.1, were not re-measured against 3.3.7 in 3.5.0, and 3.5.0 moved the rows again: L2's `_checked`
  layer A/B'd at −0.5% to +2.2% against 3.4.2; W3's rim ring took the roadmap's rim scene from 1,294 to 525
  µs (`gjk_intersect_3d`) and 1,289 to 495 µs (`gjk_epa_3d`); W3's deep snap added 3.7% to 5.9% support calls
  on deep families, its dearest parallelepiped 165 → 190 µs. Re-measure against 3.3.7 before sizing any
  repair. [cost; CO3 item 5]
- **`f64v_dot` returns +0 where the IEEE sum is −0** *(no API)*: unchanged.
  `hvec3_dot((0, 0, 0), (BFF0000000000000 ×3))` → 0, exact 8000000000000000; also `hvec4_dot`, `hquat_dot`.
  3.4.1 measured restoring −0 at +5.4–8% (vec3) and +10.6% (vec4) per call and reverted it. Since 3.5.0 the
  same three functions carry W8's 0-handle guards at +7% to +21% per call (W8's reviewer: `hquat_dot` +22.5%
  and +23.4%), a cost 3.5.0 shipped: *Decisions owed*, 3.5.0's addition. [low; AU2-G7-F10 = F-S1]

**Rays**

- **`geo_ray_triangle`: certify t to a few ulps** *(no API)*. t is certified to 2^-33 relative, and 3.5.0
  measured how far inside that it sits (W1): on 20,000 random ordinary hits (unit triangle, origin 0.5–10
  away, uniform directions) 10.2% of t are past 4 ulps and 5.0% past 8, at most 1,854; the far family reaches
  4,646, the grazing family 30% past 4 and at most 12,635, and at 2^1000 at most 2,610. By angle,
  |cos(d, N)| ≥ 0.5: 115 of 9,608 past 4 ulps (max 58.8); 0.1–0.5: 672 of 7,819 (max 507); 0.01–0.1: 997 of
  1,787; < 0.01: 200 of 209. W1's reviewer, on its own 20,000: 9.9% past 4, 4.7% past 8, max 3,126.
  `geo_ray_plane` took the same 2^-33 contract for its certified quotient in 3.5.0 (L1): rays whose d0 − n·o
  cancels are within 2^-33 but up to about 3.4e4 ulps off (2^-37.5). An a-priori bound on plain sums cannot
  certify a few ulps (each sum's bound is at least 3u relative). The forms W1 built in scratch, against base
  hits at 102–104 ns (interleaved, pinned, load about 9):
  - A, compensated t on every certified fast-path hit (two_sum differences, Dekker products; N, S·N and d·N
    as double-doubles): every refined hit within 1.99 ulps on ord, far and graze (20,000 each); ×5.5 with
    helper calls (568–576 ns);
  - A′, the same arithmetic written straight-line: ×3.8–3.9 (393–402 ns);
  - B, a running-error gate: certifies 0% of ordinary hits at 4 ulps, 16–41% at 8, 57–71% at 16;
  - C, the a-priori bound tightened to 2^-45 with refinement where it fails: refines 13,511 of 19,423
    ordinary hits, ×3.85–3.94 on random ordinary hits (×1.04 on the bench fixture);
  - D, the plain normal form −(S·N)/(d·N), no guarantee: ×1.12–1.17, head-on hits past 4 ulps 115 → 6 of
    9,608 (max 17.8), grazing hits still to 889 ulps;
  - not built: a running certificate of about 128 ulps with A′ refinement where it fails, estimated near
    1.3×.
  The slow path (`_geo_ray_tri_slow`) needs the same refinement on its scaled copy, and the plane's certified
  quotient goes with it. The row's far-hit repro (3927B14BB6B53ECC against …EC5) has no recorded input.
  [low; G2-F2]
- **Infinite-end capsules: the near-tangent class is not reproduced** *(no API)*: the row's 62 of 1,600 (3.4.0
  166) needs R2-GEO S7's own family. W2 rebuilt a half-infinite limit oracle (F's ball plus the half-cylinder
  along sg e_k, exact Fractions); inftan, seeds 1–3, 18,000 rays with one end infinite along a coordinate
  axis, tangent to the barrel or F's cap within 2^-20..2^-52 at 2^±10 scales (4,319 hits, 13,681
  near-tangent misses): 3.4.2, the wave-1 tree and 3.5.0 identical, verdict and piece exact on every ray, t
  within 4.56 ulp, 1 past 4. W2's first pass, six families, 9,600 rays: 0 wrong, at most 3.9 ulp.
  `_geo_capsule_ray_inf` answers through `_geo_sphere_roots`, whose near-tangent repairs may be why. To do:
  rebuild that family and locate the class. [low; R2-GEO S7]
- **Past the limit, origins at the surface: GEO7 F4 not reproduced, and a c-underflow class beside it**
  *(no API)*: W1 searched about 130,000 rays on base and 3.4.2 (surface origins, random directions 6,000;
  grazing from the surface 6,000; hb aimed at 2^511 ± 4, where hb² overflows the near-surface gates, 6,000;
  chords of near-tangent lines 6,000; near-tangent with the origin at the tangent point 4,000; origins
  exactly on the surface 20,000; a past-limit mix, r 2^-200..2^700, |d| to 2^1000, two seeds, 60,000, which
  covers the row's |oc| 2^32.2, r 2^32.2, |d| 2^500.7): t at most 4.12 ulps, both roots at most 4.76. The
  GEO7 driver's hex row for the 311-ulp case is needed. W1's reviewer found a class inside the limits: a
  tangent ray from an origin under 2^-537 short of its tangent point has c = along² underflow, so the near
  root c/q reads 0 while q/a is right.
  - o (225E000000004000, 9B48000000000000, 2249000000000000), d (0, 50B8000000000000, 0),
    c (1FF0000000000000, 0, 0), r 2260400000000000: exact double root 4.16e-258; t = +0, roots
    (0, 0A90000000000000);
  - o (9950000000000000, 5C89000000000000, 5C9E000000000000), d (4C68000000000000, 0, 0), c 0,
    r 5CA0400000000000: exact 7.63e-247; t = +0, roots (0, 0CE5555555555555).
  The same on 3.4.2; the unit-size copy does not mend the second (its c is 2^-2154 there). Stated in
  `geo_ray_sphere`'s caveat. Repair candidates, not built (W1): the near root as q/a where c is subnormal on a
  tangent ray, or c/q from c's exact expansion at a scale. [low; GEO7 F4]
- **Exact capsule solve on spread exponents** *(no API)*: 3.4.1's 213 µs median, 1.73 ms worst directly, the
  public capsule 1.57 ms, not re-measured. 3.5.0 sends more rays there: W2's routes took spread2 from 134 to
  161 µs a ray and spread4 from 7.55 to 10.15 µs (first pass; the second pass's gate moved 1 of 5,000 spread2
  rays off the route). `_geo_xc_sgn2` already holds a float filter (1 + 2^-32 ratio) before the exact
  a²c1 − b²c2. To do: profile `_geo_bn_sop`'s O(n²) limb products on spread input, reduce t3, or a cheaper
  sign filter. Stack 64–72 KB (`_geo_cap_xsolve` holds 59,712 B). [cost; R2-GEO S1, S9]
- **`barycentric_thin` above the first 3.4.0 form** *(no API)*: not attempted (L1, W2). 3.9–4.2 µs against
  2.6 (3.4.0 itself 12.8). A filter must correctly round degree-four sums: nine compensated cross components
  at about 61 ns each and a tie certificate. **Declined slivers**, 22×, want `lo` exact. [cost; GEO5]
- **Capsule simplifications, the two left** *(no API)*: 3.5.0 deleted `_geo_barrel_entry_ok` (bin = 1) and
  dropped `f64_eq(tc, 0)` from `_geo_capsule_far` (W2). The deleted test is load-bearing below about 2^-52 of
  the axis, where the 2^-48 near-axis route now covers it: lowering the route requires reinstating it (with
  the route at 2^-60 and no test, 13 of 3,000 lro rays wrong, to 52 ulp). Left:
  - `_geo_barrel_cert` is not redundant: without it 4 of 12,000 go right → wrong (farhit 2, farinside 2, max
    4.97 ulp). Its cost on ordinary rays past 2^20 radii (the row's 10×) stands;
  - `_geo_dperp_fine` uses six exact products where three would do: three exact products then a plain second
    projection are accurate only for |d_perp| ≥ about 2^-50 |d|, which the 2^-48 route now guarantees for the
    rays that reach it. Build it and prove it on near-axis families down to 2^-48.
  [low; GEO6, R2-GEO S3]
- **Survivors with no killing row in `geo.cyr`** *(no API)*: `_geo_barrel_entry_ok`'s survivor went with the
  function (W2). S1 (the |f| term of ed dropped in `_geo_barrel_cert`) and S2 (the copy's piece never refused
  against the certificate's in `_geo_capsule_far`) survive with arguments: 0 rays moved on farhit, farinside,
  farcap and farband (2 seeds each, 48,000 rays) and on fartan (12,000 rays from 2^20..2^46 radii, tangent
  within 2^-20..2^-50). Not located: the sign test at gx < 20; n's exact form from 2^-14 rather than 2^-6; the
  near root's target at 2^-48 (the GEO6 S3 / R2-GEO N12 report is not in the tree). Kill or prove each;
  generators must solve for r from the exact line distance. [low; GEO6 S3, R2-GEO N12]

**Collision**

- **Cone depths, apex to rim** *(no API)* (rescoped from "a ball by a cone's rim" by W3): W3's rim ring cut
  the cone's verdict misses (W3's reviewer: 1,442 → 0 at any axis, 742 → 14 on an axis, 805 → 7 near one),
  but depths stay wrong well beyond the rim. With `_co8_sup_cone` and an exact meridian-plane oracle (W3):
  shallow 528 of 3,000 past 4 eps (apex 276, slant 208, base 39, rim 7), worst 5,416 eps; deep 1,237 of
  2,000, worst 5.0e9 eps; a local search finds support values below the code's depth, so the code reads too
  high. W3's reviewer: by the base rim 39 of 6,000 past 4 eps, on the slant 750 of 3,000. The snap and the
  round fit need to know a cone's corner and slant. 3.4.1's figures for the rim: centre inside, margins
  1e-10..1e-2 of the size, 512 of 2,047, worst 1.4e9 eps; the no-ball test on the absolute least curvature
  mends 175 more and lifts the 9-point cylinder/box fixture to 66,512 B, past its 65,536 B pin. ⚠ A
  single-projection cone support reads false contacts up to 198 eps apart (W3's reviewer): a test support needs
  the double projection. [medium; F-CO8-1]
- **Depth on a rim's torus near a flat feature** *(no API)* (covers "asking the core's section wherever the
  core ran" and "two sides of near-axial cylinders"): balls whose closest point is the rim with the normal
  within about 1e-6 to 3e-4 rad of the cap or the side: 10–13 of 692 mid-rim scenes past 4 eps, worst 538
  eps, at any orientation; past the rim over a cap 72 of 3,000, worst 194 eps; the same on 3.4.2 (W3).
  Near-axial families (seed 13, 3,000 scenes, axis within 1e-15..1e-2 rad of x, y or z): every depth error
  past 4 eps is on the torus; 24 → 8 wrong, the 8 depths, worst 70.8 eps; at any orientation the class appears
  just past the side's end (π/2 − θ 2.6e-6..5e-5, 8–98 eps). CO8's one scene of 66,000 (cap and side within
  16 eps, 15.8 eps) has no recorded input and is plausibly a member. The candidate CO8 named, asking the core's
  section wherever the core ran, costs every polytope core 19–24% (861 → 1,027 and 553 → 685 support calls);
  a repair of the torus class makes it unnecessary. [medium; CO8, F-CO8-3]

**Linear algebra**

- **`lyapunov_max` past what an f64 QR can hold** (code read: stated in its doc; not attempted by L3 or W4):
  a direction Q cannot hold, [[1.7·2^-60, 0], [1.7e308, 1.3·2^-80]] at k = 20 gives −16.94, exact finite-k
  −3.519 (q₀ = (a/c, 1) with a/c = 8.6e-327 flushes); 3 of 12 dense 2×2 / 3×3 with entries 10^±150 up to 9×
  off; a dense exactly nilpotent A gives a finite rounding-level value where the finite-k answer is −Inf. Repair:
  Q and the QR in a wide m·2^e form where every column keeps its own exponent, and an exact rank analysis of
  A^t on the flag (rational arithmetic, or `_lext_det_zero` on the flag's leading blocks); a new QR with its
  own oracle. [low; R2A n3, n4, LA row 9, RV-LA N7]
- **`eigen_power`, a component through a block's root** *(no API)*: not attempted (L3, W4). 5.23 and 10.02
  ulps on components, within 5.4e-17 normwise. Exact needs λ as (P + √Q)/2 and each det as U + V√Q, with
  integers past 2^12,000, beyond the 302-word scratch: a Q(√Q) engine with scratch sized from a derived bound,
  and its oracle. [low; LA3 F5]
- **`eigen_symmetric` / `eigen_qr` disagree on ±λ order** (API, a tie-break): not implemented. Both share
  `linalg_sort_desc`, so an exact tie in the computed |λ| already orders the same way; the disagreement is a
  near-tie, an exact pair λ, −λ whose two computed moduli each routine's rounding orders (640 of 796 matrices
  with exact ±λ pairs, identically in 3.4.0). A tolerance tie-break needs a threshold with a chosen scale (the
  defect class), must cover `eigen_qr`'s 1e-12 deflation, and reorders tiny eigenvalues `eigen_symmetric`
  resolves. The rounding-free design (L3, W4): per-pair enclosures from residuals (|λ̂ − λ| ≤ ‖Av − λv‖ plus
  its rounding bound), single-linkage clusters by enclosure overlap, positive first within a cluster,
  eigenvector columns carried; one n³ residual per call, cost not measured. The returned |vals| are then not
  monotone inside a cluster: *Decisions owed*, 3.5.0's addition. Measure agreement on exact ±λ families
  (bipartite adjacency, paths) and the cost. [low; LA2 F3, RV-LA S1]
- **Two `_lext_det_zero` cost rows** *(no API)*: not attempted (L3, W4). The R2/R3-LA generators are gone:
  rebuild [[B, C], [0, D]] families (C at 2^60 / 2^1000, B and D at 2^-1000, chains) and A/B each against
  3.4.2.
  - **A block-triangular split before the elimination**: a singular block under wide entries stays 11–56×
    3.4.0, because one elimination alone is 16× at n = 100; Dulmage–Mendelsohn would decide the block alone,
    n/2 × n/2, about 8× cheaper on these, with no new bound. [cost; F-LA4-1]
  - **The exact test's ceiling at small n**: `_lext_dz_divp` and `_lext_dz_norm` walk all wa limbs of every
    residual vector each digit, wa sized by the widest entry (code read, W4), so entries past 2^1000 pay at
    small n (96 µs at n = 10 against 27 at 2^60): a count of the limbs in use per vector closes it. Rank n − 1
    with 24- to 50-bit entries has medians touching 10× at n = 10 (10.7 at worst); m unlucky primes cost m
    eliminations, up to 7.7× (8.43 at most). [cost; F-LA4-2, F-LA4-3, R3-LA N6]

**Numerics**

- **Quasi-Newton and conjugate gradients: an extreme dynamic range** *(no API)*: not attempted (L5, W5).
  `_opt_qn_first`'s first trial a0 scales as a²/c, so past 2^1024 it is not a double and the trial falls back
  to 1: 60 of 60 Err at x ×2^±600, 30 of 60 moved at f ×2^±600. Since 3.5.0 `opt_conjugate_gradient` shares
  `_opt_qn_first` (L5). To do: parametrise `_opt_wolfe_search` by d/|d| with the step a length that carries
  its own exponent (a0 held as mantissa and exponent), re-derive every `opt_bfgs`, `opt_lbfgs` and
  `opt_conjugate_gradient` pin, and run the 60-run sweeps at x ×2^±600 and f ×2^±600 against 3.4.2. In the
  same work: the approximate Wolfe allowance 2^-20|f0| mid-run, where Armijo is unmeasurable (the Ok gate
  covers the exit only), and CG's f-noise wall (below). One Beale start of 12 fails in every version.
  [low; NUM3 F2, F3, NUM4 F2, R3-NUM N1]
- **`calc_nurbs` past the documented span range** *(no API)*: not attempted (L5, W5). With weights 1e±300 and
  |P| to 1e300, 10 of 800 calls return the designed 0 for a finite point (span 2^2108..2^2619), 15 more are
  finite and wrong; 6 excursions below that range on mixed-size coordinates reach 72 units (degree 4). Code
  read (L5): `_calcx_nurbs_gauge` picks one power of two per span, so homogeneous values ranging past about
  2^2046 share no gauge, and a per-coordinate scale on P does not help once w alone and P_c alone each span
  about 2^2000. To do: an extended-exponent de Boor (each homogeneous value m·2^E) on those spans, against a
  Fraction Cox–de Boor oracle, with a cost A/B. [low; R3-NUM N4]
- **`calc_adaptive_simpson_checked`, the parts #14 does not decide**: not attempted (L5, W5). (a) Noise in
  cancelling terms has a jump's convergence signature and is told from one only by magnitude, which the
  samples understate near the polynomial's roots: a new entry point that takes the term magnitudes or a
  per-sample error bound (API), with its oracle and A/B. (b) 1/√x needs an open or extrapolating endpoint rule
  (a method change); with the 50-level cap even an open rule reaches only about 6e-8. An overflowing sample is
  right as `Err`. The backstop's size is *Decisions owed* #14. [medium; NUM]
- **`cubic_bezier_ease` near a double root of B_y** *(no API)*: not attempted (L5, W5).
  (0.5435, 2.6677, 0.0223, −1.8860) at 0.49503 → −1.0044821649632793e-17, exact −1.0044821649632942e-17,
  20.35 units; 6 of 16,761 over 8. Near a double root the error is t's rounding times B_y′(t); the
  double-double y in `_calc_cbez_y` is exact at the double t (W5). Crafted curves whose y1 cancels y2 cost up
  to 16.6× (24.5 → 406 ns at x = 2^-1074), and on 6 of those 36 inputs 3.4.0 was right. To do: carry t as a
  double-double from the solve (one double-double Newton step on B_x) and evaluate y = B_y(t_hi + t_lo) to
  first order, against a high-precision oracle on the 16,761 inputs, and a cheaper filter for the crafted
  curves, each with an A/B. [low; R3-NUM N2, N3]
- **The ODE certificate's slope: what it still cannot see** *(no API)* (the rest of "the ODE certificate's
  close pairs"): since W5 every answer also needs g to rise into it along the step's path, read as secants
  toward y over 2^-26 max(|z|, |move|) and the other way, and where those disagree toward y again over 2^-26
  of the move; an answer equal to the step's start is read along the branch's first move c·f(t, s).
  Against an exact oracle on 20,000 triangle-wave steps at y offsets 0 and 1e6, wrong Oks 142 → 6 and
  140 → 6, all six at tol 0.1. Left:
  - a pair narrower than 2^-26 of the move: W5's reviewer's constructed pairs at r = 1e4 (1.04e-4 wide) and
    1e6 are Ok on the second root in both orientations, as on the wave-1 tree;
  - the band class at tol 0.1: 11 right answers per 20,000-step set are refused, the same 11 steps at both
    offsets, with exact g′ < 0 on both sides of z, 1.0–6.7 tol/g′(root) from the root. A refinement that keeps
    correcting from the continuation's σ = 1 point when only the slope fails might recover them (W5, not
    built);
  - 29 right answers on the near-equilibrium set where g′ at the answer is below 1.1e-19, under the margin
    (y′ = sin(y − C) at dt 1, where c f′ = 1 at the equilibrium);
  - an f whose rounding exceeds an ulp of c·f defeats every slope reading: exp(y) − 1 near 0 at dt 2.5, where
    g′ < 0 everywhere and every Ok is wrong; Oks of 1,600 steps: 970 on the wave-1 tree, 358 on 3.5.0, decided
    by rounding. No black-box scale reveals f's internal magnitude; candidate: re-read where the difference is
    within a few ulps of f (W5's second fix pass);
  - at an answer equal to the step's start: a fold of g beyond the first reach or in the last half of the way
    to 0, and a dip past tol that Newton's step crosses whole, still pass (1 of 60,000 sin steps at tol 0.1,
    Y0 = 1e6; 2 of 20,000 on the z = s family).
  [low; NUM5]

**Lie groups**

- **`so3_log`: tighten the all-orders bound, and bound aarch64 away from π** *(no API)*: W7 derived the
  bound to all orders, 26.5 u (1 + 2^-44), 6.6 ulps of π (comment, "derived, not a run"): `ganita_f64_atan2`
  is atan(y/x) + fl(π) for x < 0, fl(π) is 1.10 u below π, which takes the along-axis term from 3.4.1's
  17.8 u to 18.3 u, and the second-order remainders bound under 400 u². Left: tighten toward the sampled
  2.35 ulps (0.36 of the bound) by correlating the terms' worst cases instead of summing maxima; the per-angle
  bound on aarch64 away from π, where its polyfill may add up to half an ulp of atan(y/x), which needs a run on
  that target (*Decisions owed* #3). `se3_exp` for θ ≫ 1 is closed: t within 2.77 eps|v| with no trend in θ,
  R within 1.14 u·θ (L7, __float128 oracle, 15,600 twists). [low; LIE4 F3]
- **`minkowski_interval`'s ordinary-vector floor** *(no API)*: about 1.62–1.71× the bare formula (about
  10 ns) in this codegen (W7, cpu 11, 15 interleaved rounds, load about 7). W7 derived a sound max-form filter,
  fl(a + 2^-980) < fl(r·2.5(1 − 2^-40)) for r > 0 or fl(a + 2^-980) < fl(r·−(1 − 2^-40)) for r < 0, with a
  finite-r guard, margins 1.25ε ≥ 2^-50 + 4.91u + 2^-1023/r (r > 0) and 0.5ε ≥ 1.43·2^-50 + 2^-1023/|r|
  (r < 0), ε = 2^-40: sound for |r| ≥ 2^-982, its pass set differing from the shipped test only on a
  2^-40-thin sliver at cancellation factor 7. Neither branch order is a net win: base 16.24 / 16.94 ns
  (timelike / spacelike), timelike-first 14.52 / 18.67, spacelike-first 17.10 / 16.63. The filter needs about
  5–6 f64 ops at about 1 ns each, and an int guard and a branch cost about as much as the op they replace, so
  the 1–1.3× target is unreachable with a sound filter in this codegen: *Decisions owed*, 3.5.0's addition.
  [cost; LIE3 F1]

**Autodiff**

- **Extended-exponent tangents and adjoints** *(no API)*: not attempted (L4, W6; a design read, no prototype,
  no cost claimed). `Dual` is a public 16-byte struct {val, deriv} with derived accessors, so an extended
  tangent needs hidden fields beside deriv, or `Dual_deriv` changes meaning and is retired; every op's mark
  logic (+0 constant, −0 moving zero, `_AD_NAN` miss, keep_mark) moves to (mantissa, exponent) arithmetic. On
  the tape, partials such as ln's 1/u = 2^1074 leave the doubles, so a node grows from 40 to 56 bytes and the
  adjoints need an exponent array, which changes the public `AdTape` and the hand-built-tape validation.
  Compensation cannot reach the related items. Steps: prototype the extended product-rule tangent (`dual_mul`,
  `dual_add`) and one extended adjoint accumulation with a same-boot interleaved A/B, then decide how `Dual`
  and `AdTape` carry the exponent. Repros: ln(5e-324·cos eˣ) at −1 (−0 forward, −Inf on the tape, for
  −0.1323); 3x − x/(3x) at 1e-300 (−3.7e283 for 3); x → (1e308, −1e308) with adjoints 1e300 + 10 − 1e300
  each (NaN for 0); and, moved here from "autodiff answers with no real function behind them" (W6), the
  forward-finite, tape-NaN rows, 2 per 30,000 programs, all range, such as ln(5e-324·x/x) at 1.57.
  [low; SYM3 item 5, F1, R2-SYM N3, SYM4]
- **A moving miss's NaN on the tape: what the cut pass leaves** *(no API)*: since W6 a variable whose adjoint
  was NaN only because a marked or constant zero's partner edge met an infinite slope reads forward mode's
  answer, from ad_grad's own compensated sweep with those edges cut (three seeds of 200,000 programs: 25, 28
  and 32 answers NaN → forward's finite answer, no other answer moved). Still NaN where forward answers:
  - every sweep from a root at 4,096 or above (uncompensated, no frame for the labels);
  - a variable under one cuttable edge to cut and one not, and a cut beneath another cut's zero:
    cos(w)·sqrt(w) with w = sqrt(y)(x + sqrt(y)) at 0 is 0 along x forward and NaN here (pinned as the limit);
    0 + 1 + 7 nested inputs over the three seeds' 2,399,922 inputs, plus 1 mixed one where forward is NaN too;
  - past 63 cuttable edges two can share a label, which can only leave a NaN;
  - an unmarked zero, an `ad_var` at 0 (item 3): the same label pass closes it if `_ad_cand` accepts an
    unmarked Z and the trigger fires on unmarked two-edge nodes with a zero partial. That puts the trigger
    test on every two-edge node: 8% on marked ones (95.5 → 103.3 µs, a 3,000-node all-marked tape),
    unmeasured on all.
  [low; SYM5]

**Instruments and tests**

- **Strict overcommit, the parts no machine here runs**: L8's gate and emulation are in (`check-overcommit.sh`;
  every private writable mmap past a budget refused through ptrace: `hisab.tcyr` passes at 610,000 KB and
  fails at 605,000, the verdicts `ulimit -v` gives). The kernel's own mode 2 has never run (it needs root).
  At the first CI run: read the mode the step prints; in mode 2 the gate compares CommitLimit − Committed_AS
  with each step's measured peak plus 16,384 KB (Test 2,621,960 KB, the counts step 2,623,204, Fuzz
  2,114,492, Bench 13,369,756 since 3.5.0 (10,748,308 on 3.4.2), on the 6.7.18 pin). The same run is the first of the floor-pin installer path
  `check-miscompile.sh` needs (6.6.3, 6.6.4, 6.6.5 through the upstream installer with `CYRIUS_NO_ACTIVATE=1`,
  Ed25519 over SHA256SUMS; L8). The Test peak was re-measured after `modules_d` began forking a child per W8 case (mode 2 charges each fork
  the parent's private writable mappings again, W8's reviewer): `modules_d`'s tree peaks at 2,114,056 KB, its
  compile, below `modules_b`'s 2,621,960 [measured: scratch vmpeak.py over `cyrius test`, 2026-11-14]. Under 6.6.3–6.6.7
  `alloc.cyr` has no MAP_NORESERVE and no small-grain retry, so every 256 MB chunk is charged in full and the
  figures do not hold there (W8). The other steps' minimum passing budgets were not swept. [low; INS]
- **Zero signs on consumer pins below cycc 6.6.8: `geo.cyr`** *(no API)*: L6 replaced every listed `f64_neg`
  of a possible ±0 with a sign-bit flip (the vec2/3/4 negations, `hquat_conjugate`, `hquat_inverse_checked`,
  slerp's b, `m4_flip_handedness_z`, `m4_ortho_rh`, look-at); bits identical on 6.7.18. Left (W8, reproduced
  by its reviewer): `_geo_cross_exact`'s `x = f64_neg(x)` and `y = f64_neg(y)` (`src/geo.cyr:668`, `:673`)
  negate a share that can flush to +0, so `tests/modules.tcyr`'s "an edge component of 2^-700: z = −0 (exact
  path)" fails on 6.6.3–6.6.7. W8 counts about 55 other `f64_neg` sites in `geo.cyr`, not audited. [low; MAT F1]
- **A 0 handle passed in, beyond vec and quat** *(no API)*: 3.5.0 guarded the row's three
  (`geo_closest_point_on_plane` L1, `ode_bdf_checked` L5, `hvec3_add` L6) and 50 vec2/vec3/vec4/quat functions
  (W8; wave 1 had listed 34): a handle result is the designed 0 before any allocation, a double NaN
  0x7FF8000000000000, a Result `Err(HSB_ERR_INVALID_INPUT)`, an `_into` form NaN in out and a return of 0. Not
  audited: the other modules' functions that take vec or quat handles (mat3, mat4, transforms, geo and so on);
  before W8, L6's heuristic count was 75 of 91 handle-taking public functions in vec2/3/4, quat, mat4,
  transforms, color, tensor and einsum with no 0 test on every handle. The `#derive(accessors)` getters and
  setters (`HVec3_x`, `HQuat_set_w`, …) read or write address 0 and cannot carry a guard. A guard costs a test
  per handle (W8: dot +7% to +21%), so the hot ones need an A/B. [low; INS5 F3]

**New findings in 3.5.0 (not on its roadmap)**

Found by the lanes outside their rows and not repaired. Where a figure is "re-run", it was checked on the
merged tree for this list.

*Rays and jets*

- **Half-infinite capsules: `_geo_capsule_ray_inf`'s acceptance window** *(no API)*, unchanged since 3.4.2
  and exposed by 3.5.0's sphere repairs (W1's second review). It accepts a barrel root within 2^-48
  (|o_k − F_k| + |t d_k|) of the finite end F, a window far wider than r, and keeps the barrel's root on a tie
  with the end cap's. While the sphere answered these circles wrong, the window was not reached; now:
  - a miss read as a hit: o = (4D935B99FC2839A7, CC80ACCA82F7875C, 4CC35109022B6A1C), d = (CD0D500000000000,
    4BF9400000000000, CC3D400000000000), start = (0000000000000001, 8000000000000002, FFF0000000000000), end =
    (0000000000000001, 8000000000000002, 8000000000000002), r = 0000000000000002 → t 0x407521F3F99F15FF,
    `GEO_CAP_CYLINDER`; 3.4.2 0, `GEO_FACE_NONE`, the exact answer (the line passes F's axis at 0.9999967 r,
    1.00006 r on the cap's side of F);
  - the barrel for the end cap: o = (0, 60BC7C654432D600, 60C0C10DE3F84568), d = (0, E171358DADA80000,
    E1743E4A7A4CC000), start = (8087012FE4F5B8B6, FFF0000000000000, 00A34F64C833C0A8), end = (8087012FE4F5B8B6,
    80888CDC5D666F51, 00A34F64C833C0A8), r = 00A21EC8B253B879 → t 0x3F3A7C0000000000, `GEO_CAP_CYLINDER`;
    3.4.2 …7BFFFFFFFFFF, `GEO_CAP_END_CAP`; exact: the end cap, t rounding to …7C0000000000;
  - flushed caps (r 2^-485..2^-1200, origins up to 2^60 radii away), two seeds of 3,000: wrong 1,444 / 1,415
    on 3.4.2 → 324 / 308, of which 21 / 15 were right on 3.4.2; the piece, 63 / 153 right → wrong, and past-limit
    caps (2^515..2^980) 90 / 189;
  - at ordinary scale, rays within 8 ulps of the end cap's tangency from 2^10..2^60 radii away: 521 and 544 of
    3,000 wrong on 3.4.2 and on 3.5.0 alike, about 270 misses read as hits and 240 wrong pieces; ray o =
    (−2^60, 0.9, 2^60 + 512), d = (1, 0, −1) at start (0, 0, −Inf), end 0, r 1 reads 0x43B0000000000000, CYL,
    for an exact miss.
  Candidate, prototyped in W1's scratch and not shipped: replace the infinite end by a finite twin at
  F + sg L e_k, L a power of two past 4(|o_k − F_k| + |d_k| (|o_perp − F_perp| + r)/|d_perp|) + 4r, and answer
  through `_geo_ray_capsule_branch0`: ordinary 0 wrong (was about 530), flushed 18 / 19 (all right → wrong
  against 3.5.0), past-limit 1 per seed (4.37, 4.69 ulps), and the tie still labelled CYL. It needs its own
  derivation, oracle and A/B. [high; W1's second review]
- **`_geo_sphere_roots_full` forms f from a scaled direction that lost bits** *(no API)*: `_geo_perp_part` runs
  on d2, whose subnormal component lost bits when d was scaled, so `_geo_sph_fine_near` can read an exact miss
  as a deep hit and skip the certificates: 1 of 3,000 rays in W1's MIfam family, on 3.4.2 and 3.5.0 alike.
  `_geo_sphere_dd` scales d the same way. [low; W1's second review]
- **`geo_jet_triangle` divides by the plain (e1 × e2)·d** *(no API)* while `geo_ray_triangle` has certified
  its verdict since 3.4.1 (code read on the merged tree: `den = hvec3_dot(n, d)`). On 30,000 rays nearly
  parallel to random triangles (n·d cancelling to 2^-30..2^-58 of its terms, hits 0.5–3 away): 29,078 hit
  jets, 4 with a wrong-sign dt/do and 18,513 with dt/do past 2^-20 relative, identical on 3.4.2 (L1). The
  likely repair is `geo_jet_plane`'s 3.5.0 form: den from the primal's certified or exact sum, with a unit-size
  form for extreme ranges. [medium; L1]
- **`geo_ray_obb` / `geo_ray_obb_face` from finite input whose c − o overflows** *(no API)*: a hit past DBL_MAX
  still reads 0 or NaN, 82 of 15,000 identity-rotation rays (40 → 0, 42 → FFF8000000000000), as on 3.4.2,
  where the AABB answers +Inf with its face since 3.5.0. Stated in `geo_ray_obb_face`'s doc. [low; L1's
  reviewer]
- **`geo_jet_plane`'s dt/dn carries p = o + t d's rounding where p cancels** *(no API)*: 329 of 57,977 past
  2^-20, all in the mixed-scale family, the same count as 3.4.2; the class 3.3.7 repaired for the triangle
  jet's w (L1). [low; L1]
- **`geo_ray_capsule` with a zero direction against a large capsule answers NaN where its contract says the
  miss** *(no API)*: origin (5FC99021243725D2, 0, 5F644E03D0DA9540), direction (0, 0, 0), start (0, 0, 0),
  end (5FC0000000000000, 0, 0), r 5E8B4AF8AAFEF33D → FFF8000000000000 on 3.4.2 and L1's tree (L1); re-run on
  the merged tree, the same. A unit capsule with d = 0 answers 0. [low; L1]
- **The capsule barrel's one-bit c band at the scaled copy** *(no API)*: W2's band repair does not reach
  `_geo_sph_solve`, where rays whose o_perp, r or d_perp lie outside 2^±100 go: ordsc (12,000 ordinary rays at
  scales 2^±110..2^±800, d at 2^±600) keeps 9 past 4 ulp (max 6.19; 3.4.2 159), hugefam 3 of 6,000 (max 5.7,
  first pass). Candidate (W2): a barrel-only band in that routine (bx ≠ 0) sharing `_geo_cap_band_c`'s
  arithmetic at the copy's scale. Beside it, about 1 ordinary ray in 1,300–2,000 stays past 4 ulp (max 4.9),
  other classes, and the band moved 4 rays across 4 ulp the other way (at most 5.93; W2's reviewer read such
  moves as lost lucky cancellations, the new c nearer the exact c). Subnormal-d rays with band origins go
  there too: 37 of 24,000 past 4 ulp, at most 8.8 (3.4.2: 52; W2's second fix pass). [low; W2]

*Collision*

- **"Lower at all" outside the deep snap and the band** *(no API)*: 3.5.0 gave ties within h's rounding
  (2^-49 of `_ga_sup_scale`) to the closed form in `_epa_snap_deep`, `_ga_band_depth` and the rim ring's
  callers, after its first form had moved normals 3.4.2 had to 1e-16..1e-8 rad by up to 1.33e-4 rad (W3's
  second review). The same class stands at two more sites: `_ga_order_pen` takes the other order's depth where
  lower at all (in one family scene it displaced a closed form the band had taken at a tie, normal 2e-16 →
  8.3e-8 rad), and `_epa_round_core` picks between the section's two ends by value (what answered the
  reviewer's cone scene right in 3.4.2 was the base form's reading rounding 0.135 eps high). Apply the tie rule
  there, with an A/B of normals against 3.4.2. [low; W3's second fix pass]
- **`_ga_rim_ring`'s g0 reading carries about 9 eps of noise** *(no API)*: kr is read from heights at tilts b
  and b/2, whose difference is only 3 kr b²/8, so the branch past the circle is taken for origins up to about
  1e-13 inside (W3). On the traced scene (fi1 1590) g0 read −1.3e-13 where the truth is +9.1e-15. [low; W3]
- **Small balls by a cylinder's or a cone's rim still miss** *(no API)* (the rest of `F-CO7-2`): the rim ring
  makes these contacts for balls of r about 1e-3 and up (831 → 0 of 6,000). Misses past 4 eps, 3.4.2 → 3.5.0,
  of 6,000 per family: r 1e-6 to 1e-3, 903 → 75; r 1e-8 to 1e-6, 1,197 → 948; cones by the base rim, r 1e-8
  to 1e-5, 1,086 → 381 (W3's second fix pass). W3's second reviewer also reports r 1e-10 to 1e-9 with depths
  up to 191.5 eps off and normals up to 0.106 rad off, not reproduced by the fixer. [medium; F-CO7-2]
- **Deep balls just outside a cylinder's rim answer far too deep** *(no API)*: balls 1e-10 to 1e-2 deep outside
  the rim; the reviewer's scene answers 538,765.6 eps too deep with the normal 2.85e-2 rad off, the same on
  3.4.2, the wave-1 tree and 3.5.0 (W3's second review, reproduced by its fixer). [medium; W3's second review]
- **Depths and verdicts by a rim, a side's end and a box edge** *(no API)*, all measured on 3.4.2 too (W3):
  - past a cylinder's rim, 4 of 12,000 scenes still miss past 4 eps (4.1 to 18 eps; 1 − ρ/R up to 5.7e-6;
    small r), and depth answers past 4 eps stay about 200 of 9,000 per family, worst 2,504 eps;
  - balls on a cylinder's side 1e-14 to 1e-6 from the cap plane: 12 of 6,000 too shallow by more than 4 eps,
    worst 248 eps answered as depth 0 by the touch path (side family seed 5, scene 1335; W3's reviewer);
  - balls by an axis-aligned box's edge with the normal near a face: 7 of about 1,000 edge scenes past 4 eps,
    worst 2,672 eps (W3's first pass);
  - verdict misses on W3's first-pass tree: a ball beside a cylinder's side just past its end (centre 2.8e-12
    above the cap plane, 28 eps deep; fam2 seed 7 scene 763), 2 cone misses of 3,000, one 4.16-eps borderline.
  [low; W3]

*Linear algebra*

- **`matrix_condition_number` past cond 2^53 when no pivot is lost** *(no API)*: the f64 tier gives the
  magnitude only: 113 of 685 such answers off by more than 2×, worst 125× (Hilbert 13 about 20% off) (L3).
  The certified tier (`_lext_inv_hp`) could serve it behind a residual check, at an n³ cost on every such call
  that needs its own A/B (W4). [low; L3, W4]
- **`matrix_condition_number` of I₂ is 2 + 2^-51** *(no API)*: 0x4000000000000001, two rounded norms
  multiplied on the f64 path (L3, W4); re-run on the merged tree, the same. [low; L3]
- **`cmat_inverse`'s lost-pivot cost at large n** *(no API)*: a coupled complex block at n = 16 / 32 / 64
  takes 37 ms / 354 ms / 2.9 s (single noisy runs), where the wave-1 tree answered 0 in 0.19 / 0.88 / 6.3 ms;
  about a minute extrapolated at the n = 181 cap (W4). Speedups not built: a complex-native hf Gauss–Jordan in
  place of the 2n real form (about 2× on that part), a fused hf multiply-subtract (the update costs about
  370 ns), a better first precision (the failed 6-limb attempt is about a third of the complex cost). Whether
  the cost ships is in *Decisions owed*. [low; W4]
- **`eigen_power` refuses right answers its bound cannot hold** *(no API)*: 20 of the 897 right answers 3.4.2
  certified on W4's reviewer's gap families (A = Q diag(1, r, …) Q^-1, r 0.9–0.995, n 6–20) stay
  NO_CONVERGENCE, strongly non-normal (eigenvector conditions 5,857, 9,211, 7,786 and 6,627 on the four
  examined), and 40 of the 52 right answers in the cancellation family (e ≥ 4: powers bounded only through
  1 + ac − ac with ac past 2^8). Candidates (W4): a double-double `_lext_deflate` inside the double-double
  squarings (marginal on the one traced); an exact small-n certificate, the characteristic polynomial's
  Schur–Cohn test of ρ(A) < 1.001|λ| in the existing bigints. The wide path costs 18–71× 3.4.2 on positive
  matrices scaled by 2^±300 from n = 16 up (5.6 → 393 ms at n = 64; 2.2–3.5× the wave-1 tree, whose wide loop
  is about a third of it; W4's second fix pass): folding the four term types into two (g|M| + R and |M| + R
  per entry) is cheaper at the price of an up-rounding to re-derive; the wide path also has no T-alone gate (no
  coverage loss seen, not proven). Whether the cost ships is in *Decisions owed*. [medium; W4]
- **`eigen_power` certifies answers above ρ on block chains** *(no API)*: 6 of 600 chains (2–5 blocks,
  coupling 2^100..2^1020), up to 0.66% above: chain 5, l = (1.0581, 1.4434, 1.8758) at coupling 2^532,
  answers 1.8881134144379197 (L3). Gelfand's test bounds ρ from above and cannot see them. W4's oracle A/B
  (1,800 matrices with 198 block chains) reports every wrong Ok of the wave-1 tree refused; whether it
  included these six is not stated. [medium; L3]

*Numerics*

- **The ODE continuation's guard slopes read only above each point** *(no API)* (`_ode_fy` over
  +`_ode_dh(zc, mv)`): W5's case A mirrored at 1e6 ends NO_CONVERGENCE after 153 evaluations where the
  unmirrored step is Ok(1e6) in 133; the certificate refuses Newton's wrong root in both orientations. Reading
  the slopes toward s mends the mirror (Ok(−1e6) in 133) but moves 2,116 answers on the Y0 = 0 triangle set
  and turns 117 Err → Ok and 25 Ok → Err at Y0 = 1e6: a change with its own before/after. [low; W5]
- **`ode.cyr`'s measured evaluation bounds predate the certificate** *(no API)*: 3.4.1's 11,755-review-step
  counts (at most 22 more at max_newton 1, 37 at 3, 349 at 50) and `ode_bdf2_checked`'s "at most 565 times
  more than 3.4.0's 101" were measured before Newton's answer was certified; the source labels them "not
  re-derived since", and their input (py/ode_in.txt) is gone (W5). Rebuild the set and re-derive. [low; W5]
- **`opt_conjugate_gradient`'s f-noise wall** *(no API)*: on ln(1 + a²)-type objectives at tol ≤ 1e-9, where
  f is a few 1e-16 the rounding of 1 + a² is as large as f, no trial meets the measurable Armijo test and the
  search falls back to a step that rounds to x: of 500 4-D problems 3.4.2 fails 72 and 3.5.0 70 (12 in
  common); `opt_bfgs` fails 0 of 10,000 (L5). Repro: L5's reviewer's second 4-D run, Σ w ln(1 + (x − c)²) at
  tol 1e-12, pinned NO_CONVERGENCE in `tests/modules_d.tcyr` (it flips on repair). Candidate: a noise-aware
  Armijo (Hager–Zhang's C_k allowance), with its own A/B; it goes with the quasi-Newton row above.
  [low; L5]

*Symbolic and autodiff*

- **Memo walks on trees that share nothing** *(no API)*: `sym_integrate`'s memo walk in `symbolic_ext.cyr` is
  about 3.6× 3.4.2 on a tree past 2^20 operators that shares nothing (97 → about 350 ms, scratch probe), the
  eval/simplify pattern again: `_sx_stk_*` calls, `_sx_memo_get` per operator, no filter (W6). Evaluate,
  simplify and differentiate stay about 2× 3.4.2 there after W6: the plain walk's first 2^20 operators are
  walked again (about 28 ms of evaluate's 108) and the walk is iterative where 3.4.2's was recursive. A
  recursive memo form might take another 30–40 ms off each walk (W6's estimate, unmeasured). [low; W6]
- **The plain rewrite walk gives up past max(ALLOC_MAX/32, 24 MiB) stored** *(no API)*: a rewrite with
  nothing shared that needs more than 31/32 of ALLOC_MAX after that point is newly refused; no repro of that
  case was run (L4). [low; L4]

*Foundations, transforms, Lie and complex*

- **`cx_abs(±Inf + i NaN)` is NaN** *(no API)*: by `cx_abs`'s NaN-in/NaN-out contract, where C99's cabs and
  IEEE's hypot give +Inf. Since 3.5.0 that value is the modulus of `cx_inv(0)`, `cx_div(1, 0)` and
  `cx_powf(0, n < 0)`, and `tests/edge_cases.tcyr` pins |1/0| = NaN (L7). The target is +Inf; it changes
  `cx_abs`'s documented answer in place: *Decisions owed*, 3.5.0's addition. [low; L7]
- **`hquat_slerp` of a nearly identical pair at large |t|** *(no API)*: for d ≥ 1 with |a − b′| > 0 the
  normalised-lerp branch serves θ below about 2^-26, and its error grows like (|t| θ)³: far outside [0, 1] it
  approaches a tangent direction, not the slerp (L6, code read, not measured). [low; L6]
- **`screen_to_world_ray` has no [0, 1] / reverse-Z depth convention** (API): it is
  `world_to_screen_checked`'s pair, which took a depth-range parameter in 3.5.0; a reverse-Z ray needs near at
  z = 1 and the infinite far plane at z = 0 (L6). [low; L6]
- **`t3d_compose_checked`'s cosine is not certified where b's scale components span more than about 2^1000**
  *(no API)*: stated in its doc (L6). [low; L6]
- **`_hquat_rotate_wide`'s doubling pins the less accurate bits** *(no API)*: 3.5.0 kept it and pinned the
  input where it moves a bit (the [3.4.2] row's second option), y 0x8027C8532976B140 doubled, …B150 undoubled,
  exact …B153 (a turn about y keeps y): 19 ulps against 3. Over three seeds the forms differ on 299 of 137,381,
  293 of 137,047 and 299 of 137,215 wide-path inputs; on seed 1 the undoubled form was closer on 265, the
  doubled on 29, with 8 ties; both are within the 22 u|v| contract (L6). Removing it needs the trust test's
  subnormal-rounding bound, derived for n ≥ 1, re-derived for |q|² in [1/4, 1). [low; L6]

*Code that the third-instance rule now covers*

- **One public quiet NaN**: eight private enum members hold 0x7FF8000000000000 (`_CX_QNAN`, `_LIE_QNAN`,
  `_LEX_QNAN`, `_DG_QNAN`, `_HV2_NAN`, `_HV3_NAN`, `_HV4_NAN`, `_QUAT_NAN`), and the literal appears in 20 of
  `src/`'s files (read on the merged tree). A public constant in `f64_util.cyr` is new public API (L7). [low]
- **One ldexp helper**: 10 private copies in 9 modules (`_col_si_ldexp`, `_geo_jet_ldexp`, `_dg_ldexp`,
  `_m4_ldexp`, `_m3_ldexp`, `_hquat_ldexp`, `_trs_ldexp`, `_cga_ldexp`, `_geo_ldexp`, `_geo_root_ldexp`),
  and `_trs_mul_err` / `_trs_add_err` copy vec3's error-free transforms (L6's reviewer; L6 left it for after
  the merge, since the copies span six lanes' files). [low]
- **The `var` aliases of constants that are enum members since 3.5.0**: `_GA_F64_POS_INF` (68 occurrences),
  `_SP_F64_POS_INF`, `_AD_F64_TINY` and `_GA_F64_TINY` are `var`s initialised from `F64_POS_INF` / `F64_TINY`,
  each reference a load and a relocation; `_GA_F64_TINY` has no reader (L6 reported the aliases; counts read
  on the merged tree). [low; L6]

*Instruments and tests*

- **The bench harness's resident peak is about 13 GB** in any overcommit mode: `tests/hisab.bcyr` maps 50
  grain chunks and never frees them, resident 12,968,428 KB, mode-2 commit with the CLI 13,369,756 KB (3.5.0's
  re-measure; 3.4.2's 40 chunks, 10,326,400 and 10,748,308 KB, L8). A
  smaller runner swaps or kills Bench. `alloc_reset` between rows moves benchmark numbers and needs its own
  before/after. [low; L8]
- **`check-constants.sh`'s expression arm reads only one shape**: a hex f64 in an expression whose trailing
  comment is, whole, one value. The 121 expression lines with more than one value in the comment, the 8 with
  two payloads and the uncommented expression literals are counted, not read; of a one-bit plant at each of
  the tree's 1,987 payloads, 179 of 285 commented one-payload expression sites and 0 of 1,293 others were
  caught. R4-SYM N2's 15 sites were never recorded, so the plant sweep stands in for them (L8). [low; L8]
- **Guards no constructed input reaches** *(no API)*: diffgeo's eu allowance `_DG_EUV` (mutant M7, 2^-1069 →
  2^-1074, survives: any constructible fixture errs by a few 2^-1075, and the 17 p d²(d − 1) factor makes the
  two constants behave alike), `cx_ln`'s renormalisation fallback below |hi| = 2^-94 (X9), and the capsule's
  tiny-copy hoist `if (tiny == 1) txf = _geo_ldexp(…)` (L1's H01; 252 + 3,000 near-axis tiny rays never reach
  it) are kept because the bound needs them; W7's first pass also reported S46 (the dd DA halved), not
  re-reported by its second. Find the inputs or prove them unreachable. Five `_ga_rim_ring` guards have no
  test either (dn < kr; the kr clamp; tmin < tmax; gr > 0; dn > dl): each needs a white-box input or an
  equivalence record (W3's second review). [low; W7, L1, W3]
- **Mutants that survive the suites** *(no API)*: MQ, `ad_div`'s d/db formed as −q·(1/b), two roundings, in
  place of −q/b, survives `modules_d`, `edge_cases` and `hisab`: no assertion pins the one-rounding d/db on
  ordinary inputs; 3.4.1's group pins only the old (1/b)² form (W6's second fix pass). In W2's capsule block,
  rows d, j, m, n, o, p, q and t kill their mutants only through bit changes within the 4-ulp contract, not a
  wrong answer (W2's second fix pass): a later in-contract repair can move them. [low; W6, W2]
- **Pinned values made less accurate within contract**: `tests/modules_c.tcyr`'s sphere 'y' pin
  0x3FEFFFFFFFFFFFFF (0.105 ulp, correctly rounded) → 0x3FF0000000000000 (0.895 ulp), its message weakened
  from "t rounded" to "t within an ulp" (W1); `ppar 1-7` 0x3FEF14E3D6347903 → …905, 0.56 → 1.44 ulp (W2);
  CG's 3-D quadratic x0 from 4/7 correctly rounded to 4/7 + 4 ulps, and x⁴ + x² 4.2e-10 → 1.9e-9 (L5's
  reviewer; on 2,500 quadratic CG runs c exactly 264 → 259, and L5 found the first trial's 1/4 curvature not
  the cause). Settle for each whether the old bits were luck. [low; W1, W2, L5]
- **One mutant's verdict is unverified**: wave 1's mutant B on the bdf2 "second sin step" fixture was killed
  by evaluation count only (151 against 155); W5's slope moved the count, and the assertion now pins 155, but
  the mutant was not re-run (W5's first pass). [low; W5]
- **What consumers on old pins get is not written down**: W8 classified every suite failure under 6.6.3 (112:
  ganita 1.2.5's SVD 22, the pin's `f64_sin` / `f64_cos` 30, `f64_pow` 22, `ganita_f64_atan2` special cases
  10, `f64_sinh` / `f64_cosh` 23, the INS5 instrument 4, `_geo_cross_exact`'s +0 1) and 6.6.10 (59: SVD 22,
  atan2 10, sinh/cosh 23, INS5 4); 6.6.4–6.6.7 match 6.6.3. Record it in `dependency-watch.md` or § Consumers.
  pravash's `scripts/probes/p1_memory.tcyr` discards `spatial_hash_insert` / `spatial_hash_clear` results and
  warns under 3.5.0's `#must_use` (L2). [low; W8, L2]

### **[3.7.0]** — the abaco bridge (additive; the pin is the maintainer's to move)

Scheduled proactively by the maintainer on 2026-10-03, so abaco can build its Rust-era plan
(`solve x^2 - 2 = 0`, interval-wrapped results) on hisab without having to ask. It adds public
names, so it needs a minor. Its prerequisite, `D074`, shipped in 3.4.0: `expr_eval_checked`
refuses an unbound name, which `expr_eval` read as 0.

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
    `expr_simplify(expr_diff(e, var_name))` under `num_newton`'s contract. ⚠ `expr_diff`'s power
    rule for an exponent that is not a CONST node gives NaN at x ≤ 0 (3.4.1, SYM-F1).
  - `sym_solve_bracket(e, var_name, vars, x0, a, b, tol, max_iter, out)`: Newton, then bisection on
    [a, b] when Newton errs or leaves the bracket; refuses a non-finite f(a)/f(b) or equal signs, and
    returns `Err(NO_CONVERGENCE)` on exhaustion, as `num_bisection` does since 3.4.0.
  - `sym_eval_interval(e, ivars, out)`: interval evaluation through `ivl_*`; needs new public
    `ivl_cos`, `ivl_ln` and `ivl_pow` that are sound for every base sign (integer exponents by
    parity, negative exponents through `ivl_div`'s unbounded rule, non-integer exponents and ln only
    on lo > 0).
- **Do not port Rust's defects**: its Pow corner rule is unsound across 0 ([-2, 1]^4 gave [1, 16];
  the range is [0, 16]), and a free variable read as NaN (hisab's `expr_eval_checked` refuses it).
- **Implementation notes, measured 2026-10-03**: routing through `num_newton` with the evaluator as
  capturing closures built inside the hisab fn solves x^2 - 2 from 1.5 to 0x3FF6A09E667F3BCD,
  bit-exact under cycc 6.6.3 and 6.6.14. Keep those closures inside a fn (the top-level `fncallN`
  crash) and never `return` a `: stack` value from them (`D082`): both are fixed in cyrius 6.6.16,
  but abaco pins 6.6.12. abaco 2.4.12's bundle and
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
3. **aarch64.** `f64v_dot` is fused (`fmla`) on aarch64 and mul+add on x86. So `hvec4_dot` and
   `hquat_dot`, and through them `hquat_slerp`, give different bits on the two targets; the 3.4.0
   audit's reading of the 6.7.9 emitters adds `hquat_length` and the quaternion normalize and
   inverse (through `hquat_length_sq`) and finds `hvec3_dot` the same on both (F-S2). CLAUDE.md's
   Language rule names only `f64v_axpy`/`f64v_fmadd`. Update the rule? An aarch64 test run is
   demand-gated below. `D120`
4. **CI consumer-pin matrix?** The public-surface and consumer-build gates run only under the
   toolchain pin (6.7.18). The multi-pin build is a manual release step, which is how 3.3.0's
   `_SYM_EPS` leak and a 3.4.0 `duplicate variable` refusal, both under 6.6.3 only, were caught.
   Automate it? `D110`
   **Extended (3.5.0).** CI now runs `check-miscompile.sh` on the bundle under the floor pin 6.6.3
   and the manifest pin (L8), and every suite compiles under 6.6.3–6.6.7 (W8). A suite step under
   the floor pin needs an expected-failure baseline: under 6.6.3, 112 rows (ganita 1.2.5's SVD 22,
   the pin's `f64_sin` / `f64_cos` 30, `f64_pow` 22, `ganita_f64_atan2` special cases 10,
   `f64_sinh` / `f64_cosh` 23, the INS5 instrument 4, and one hisab row, `_geo_cross_exact`'s +0,
   carried); under 6.6.10, 59 (SVD 22, atan2 10, sinh/cosh 23, INS5 4); 6.6.4–6.6.7 match 6.6.3
   (W8). The multi-pin consumer build stayed a manual release step: the lanes probed their own
   bundles piecemeal, and the release ran the final bundle under every pin from 6.6.2 to 6.7.18
   (*Current*).
5. **Missing local tags.** The CHANGELOG has releases 3.2.0, 2.11.1, 2.9.3, 2.2.2 and 2.2.1 with
   no local git tag. The remote was not checked. `D164`
6. **Three [3.3.5] rows landed differently from their text** (CHANGELOG 3.3.5 has the evidence):
   - `num_bisection` refuses a NaN f(a) / f(b) but still bisects ±Inf ones (the row said
     "non-finite"; refusing ±Inf broke four correct answers, e.g. ln on [0, 2] → exactly 1).
   - `calc_adaptive_simpson`'s floored acceptance is kept: every stricter rule measured either kept
     the aliased repro wrong or walked other integrands past 200M evaluations. A tripwire pins the
     wrong 1.1 the aliased integrand returns.
   - Overflowing quadrature now takes the walk its in-range twin takes: calls that returned +Inf
     after 5 evaluations could run past 2·10⁹ evaluations (> 60 s), the `D070` class. Since 3.4.0
     every walk ends at the 2^25-evaluations-per-pass backstop (#14); this input was not re-measured.
   Accept, or schedule a different outcome.
7. **`ivl_contains` / `ivl_overlaps` on a NaN endpoint** answer 1 since 3.3.7, which [NaN, NaN]
   already answered (cannot exclude); the answer was chosen with 3.3.7's uniform NaN rule, not by the
   maintainer. IEEE 1788's NaI semantics would answer 0, for [NaN, NaN] too. No hisab module, example,
   harness or consumer calls either predicate. Keep 1, or move both to 0?
8. **The DST / DCT Bluestein cap (memory policy).** Lengths that are not powers of two stay direct
   past 2^20. A Bluestein reduction holds 32m + 16·len bytes, at most 80 MiB under the cap;
   `num_dct(2^20 + 1)` through Bluestein would hold 144 MiB (8 MiB direct) for about 7,700× fewer
   operations, and the allocator's own bound would allow 5 GiB per call, never freed. Where should
   the ceiling sit? If it moves, the private enum member `_NUMX_DFT_MAX` (`_NumxDft`, since 3.5.0) and
   `modules_b.tcyr`'s three `_numx_bluestein_capped` assertions move with it; the tests lower the cap
   through the seams `_numx_capped_at`, `_numx_dct_at`, `_numx_idct_at` and `_numx_dst_at`, so there is
   no assignment to restore.
9. **The box-corner contact normal.** 3.3.7 adopted [3.3.6]'s proposal, "the closest-approach
   direction at the reported t" (`time_of_impact` and `gjk_epa_3d` on corners). Confirm it as the
   contract, or name another.
10. **`_kd_select_median`'s tie placement** is unspecified (mutant m0185 changes which of two equal
    keys goes left). Should the kd-tree promise a tie-break?
11. **Two upstream candidates from the 3.4.0 audits — decided 2026-10-31.** The maintainer approved
    filing both. Checked against each upstream's contract first:
    - **ganita: filed** — `ganita_mat_eigen_sym`'s absolute Jacobi stop (see *Toolchain*). 3.4.1
      repaired `eigen_symmetric` with hisab's own Jacobi cleanup, not by routing to `eigen_qr`, whose
      own 1e-12 deflation has the same defect (the [3.6.0] row). (AU2-G3-F3)
    - **cyrius: not filed** — `f64v_dot` starts its accumulator at +0 on x86 (`xorpd`) and aarch64
      (`eor`) alike, the same convention as reference BLAS `ddot`, so an all-(−0) dot product is
      +0 by definition, not a compiler defect. Whether `hvec3_dot` / `hvec4_dot` / `hquat_dot`
      should give the natural evaluation order's −0 is #16 below. (F-S1 = AU2-G7-F10)
12. **Three questions the audits left to the maintainer** (3.5.0 answered each in code; #18 holds the
    answers, and #12 closes when #18 does):
    - **(a) The symbolic simplifier's constant folding.** `sym_const_eq` folds within an absolute
      1e-15 "for the O(1) coefficients" (also `migration-3.3.md:92-94`), so `h·x`, `1e-16·1e300`
      and 1e-20·1e30 fold to 0. CLAUDE.md calls an absolute threshold on a quantity that carries
      scale a defect. Is that sentence the contract, or does folding become exact or relative?
      (SYM-F2, AU2-G6-F2/F3)
    - **(b) An isolated vertex: boundary or interior?** `halfedge_is_boundary` answers Ok(0),
      "interior", for a vertex no triangle uses; OpenMesh's `is_boundary(VertexHandle)` and CGAL's
      `Surface_mesh::is_border(v)` answer true. Undocumented and untested. (AU2-G5-F8)
    - **(c) `ev100_to_luminance`'s factor 12.5/π.** It gives 0.4974 at EV 0 where Lagarde /
      Filament give 0.125, while `ev100_to_exposure` follows Frostbite. Which convention?
      (SPC-F5)
13. **The 2026-08-11 audit's workflow ledger.** Its full result (52 findings, 24 skeptic verdicts,
    21 survivors) survives only in that run's session store, outside the repository; AU3 matched it
    24 of 24 and used it to enumerate the 28 never-verified findings, which three documents had
    called "not enumerated anywhere". The session store is not durable. Commit an extract, as was
    done for the epsilon census JSON? (AU3)
14. **The adaptive-Simpson backstop.** `calc_adaptive_simpson_checked` ends a walk its noise floor
    cannot stop after 2^25 evaluations per pass, the second pass kept; the integrator chose 2^25 in
    3.4.0. The worst cases measured take ~1.7–3.5 s: 5x⁴ − 3x² + x − 1/4 at 1e-20, 33,556,523
    evaluations in 1.7 s (Err either way; 3.3.7 never ended it), and A·sin(84.5x) + ε on [0, 10]
    at 1e-14, 34,055,987 in 3.5 s. 266 of the reviewer's 6,000 calls pass 4,000,000 evaluations
    (3.3.7: 383). A smaller backstop ends these sooner and turns more slow walks into Err. Keep
    2^25?
15. **The 268 masked allocation checks.** "Revert any part that moves no number" and "check every
    allocation" pull opposite ways there: 268 of 470 checks survive every suite and exhaustion point
    because an instrument's refusal is sticky and production's mmap refusal is not. 3.4.1 keeps them and
    records why in `scripts/mutation-test.sh`. Whether CLAUDE.md says so is open.
16. **What the 3.4.1 integrator settled that the rule leaves to the maintainer.** Two acceptances (the
    collision costs against 3.3.7; `f64v_dot`'s +0, whose −0 was built, measured at +5.4 to +10.6% per
    call and reverted), two rows moved to [3.5.0] (`world_to_screen_checked`'s depth, `lyapunov_max` past
    what an f64 QR can hold), and two exact paths kept though they moved no number on their pass's
    tests (`sdf_capsule`'s cross product, which a later pass found scenes for, and `cga_norm`'s exact norm²).
17. **Two stdlib notes, not filed.** `alloc_init` prints and exits when a process's first chunk is
    refused (`lib/alloc.cyr`), so that one allocation has no refusal channel for hisab to carry.
    `lib/args.cyr` keeps `_args_base` across calls and `lib/io.cyr` keeps `_env_blk` and the
    `_io_contain_*` values, so a consumer that uses them and `alloc_reset()` is exposed; read from the
    source, not run, and no `src/` code calls them. Report upstream?
18. **What 3.5.0 did to public names, for the maintainer to confirm** (answers #12; the rule is *How to read
    this file*'s "a public name whose meaning changes is retired, not redefined"). The lanes applied the rule
    three ways and asked for the call where they were unsure. #12's three questions are answered in code:
    (a) folding is exact, and `sym_const_eq` is retired (L4); (b) an isolated vertex is a boundary, under the
    same name (L2); (c) L = 2^(EV − 3), K = 12.5, with the old names retired (L6). #12 closes when this does.
    - **Retired** (no longer in `src/` on the merged tree): `sym_match`, `sym_match_bindings` → their
      `_checked` forms, `sym_const_eq` → `f64_eq` (L4); `num_mobius` → `num_mobius_checked` (L5);
      `t2d_apply` → `t2d_transform_point`; `t2d_compose`, `t3d_compose`, `tensor_get`, `tensor_set`,
      `tensor_get2`, `tensor_set2`, `m4_look_at_rh`, `hquat_from_axis_angle`, `hvec3_angle` → their `_checked`
      forms; `ev100_to_luminance`, `luminance_to_ev100` → `ev100_to_avg_luminance`, `avg_luminance_to_ev100`
      (L6). `world_to_screen_checked` kept its name and gained a fifth parameter, `depth_range` (L6; the row
      called it a signature change, and arity is enforced on every pin from 6.6.3).
    - **Kept beside a new `_checked` form, the lane asking whether to retire them.** Each answers as in 3.4.2
      and its doc points to the `_checked` form; retiring means migrating the call sites:
      - `geo_triangle_unit_normal`, `geo_segment_direction` and the scalar jet accessors
        `geo_jet_sphere_dradius`, `geo_jet_plane_ddistance`, `geo_jet_capsule_dradius` (L1). Call sites: the
        first in abuse 9, modules 18, modules_c 49, `tests/hisab.bcyr` 6; the second in abuse 8, edge_cases 1,
        modules 10, modules_c 37; the jets in modules 7 / 3 / 9 and modules_c 3 / 2 / 1;
      - `gjk_intersect_3d`, `mpr_intersect`, `gjk_epa_3d`, `mpr_penetration`, `time_of_impact` (L2, W3): 541
        call sites across the suites, the bench and the examples. They keep 3.3.5's meaning, 0 on a refused
        allocation, so W3 reads the rule as not requiring retirement; deprecating them is the open choice;
      - `ad_val`, `ad_grad_of` (L4): 166 suite call sites, `tests/hisab.bcyr` 2, `examples/basic_math.cyr` 1;
        kept as accessors with a documented precondition (`ad_ok(t, i) == 1`).
    - **Answers changed in place under the same name, which the lanes treated as repairs and asked to have
      confirmed** (CHANGELOG [Unreleased] carries every other changed answer):
      - the AABB and OBB slabs: a hit past DBL_MAX from finite input is +Inf with its face, where it was the miss
        0 (L1, citing 3.3.4's D034 NaN repair as precedent; four `tests/modules.tcyr` pin sets moved);
      - `geo_ray_plane` with finite n and d and an infinite origin or d0: +Inf, the miss or NaN by exact signs,
        where 3.4.2 gave NaN through Inf/Inf; 2,561 of 60,000 non-finite records (L1, "for the maintainer");
      - `halfedge_is_boundary`: an isolated vertex, and a bowtie whose second fan is open, answer Ok(1), the
        twin reading (L2, "confirm that reading"); `halfedge_from_triangles` records a twinless outgoing
        half-edge as a vertex's edge where it has one;
      - zero axes: `cga_rotor` (L2) and `su2_from_quat`, `su2_from_axis_angle`, `so3_from_axis_angle`,
        `lorentz_boost`, `lorentz_rotate` (L7) answer NaN in the parts that need the axis, where they answered
        the identity; L2 took L7's convention so that the two move together if it is overruled;
      - the rest of L7's list: `cx_div` / `cx_inv` over an exact zero answer C99's infinity; `cx_powf` at a
        zero base gives 0⁰ = 1 and +Inf + i NaN for n < 0; `gamma_spatial` / `gell_mann` out of range answer 0
        (a 0 pointer where a zero matrix was); `su3_structure_constant`, `ricci_scalar`, `sectional_curvature`
        and `killing_residual` on invalid input answer NaN; `se3_new` / `lorentz_is_valid` on a 0 handle
        answer 0. L7's reviewer cited the 2.11.5 rule; the AU2 §2b row said "their pins move with them";
      - `dual_pow`, `ad_pow` (a non-integer power of a moving base at ±0, and x^0 of a NaN-valued base, are
        misses) and `ad_grad` (the cut pass) (W6, citing 3.4.1 SYM4's misses under the same names).
    - **Two that wait on carried work**: if the ±λ tie-break ships, `eigen_symmetric` and `eigen_qr`'s |vals|
      stop being monotone inside a cluster, so retire both names or rule that the order inside a cluster is not
      part of their meaning (L3, W4); and the carried `cx_abs(±Inf + i NaN)` = +Inf changes a documented
      answer of an existing name (L7).

19. **Costs 3.5.0 shipped for a guarantee, which the lanes put to the maintainer** (extends #16, which holds
    3.4.1's two acceptances). Each cost as measured, with what it buys. All are in the tree; the call is to
    accept each cost or to take the cheaper form named beside it, where there is one.
    - **The certified implicit ODE step** (L5, W5). Against 3.4.2: `ode_bdf2_step` 254 → 5,504 ns, backward
      Euler on y′ = −y at dt 0.1 68 → 1,439 ns (L5; five interleaved rounds, taskset, load 3.5–3.9); W5's
      slope then added, against the wave-1 tree, +5.2% / +6.1% on backward Euler and +4.0% / +3.7% on
      `ode_bdf2_step` (W5's second fix pass: 11 interleaved rounds, two runs, load 1.8–4.0), took Robertson's
      step 4,445 → 4,056 ns, and made an answer equal to the predictor, which 3.4.2 returned unread, cost 2 to
      4 evaluations more (38 → 180 ns on a BDF-2 step). A certified answer pays the 63 samples and the slope's
      2. Buys: on a 6,000-step sin set 3.4.2 returned
      3,322 Oks on another root and 54 past a fold, L5's tree none on another root (L5). W5's argument that no
      cheaper form keeps the guarantee: g′(z) ≤ 0 can only refuse; a sufficient test needs a bound on f_y
      between samples, which a black-box f does not give; fewer samples widen the gap (3.4.1's third review:
      32 even samples left 2 of 194 wrong, 64 none). The row's two necessary checks (the move against
      sign(c f(s)); g′(z) ≤ 0) reject 1,655 of its 2,106 wrong Oks and certify nothing; not re-measured on
      this tree.
    - **The vec/quat 0-handle guards** (W8). Medians of 16 pinned interleaved pairs against the wave-1 tree
      (load 5.5–6.9, noisy): `hvec2_dot` 8.70 → 9.31 ns (+7%), `hvec3_dot` 7.56 → 8.43 (+12%), `hvec4_dot`
      5.73 → 6.95 (+21%), `hquat_dot` 6.07 → 6.88 (+13%; W8's reviewer, two 12-pair runs: +23.4% and +22.5%),
      `hvec3_distance` +9%, `hvec3_add_into` +10%; the lengths through dot (W8's reviewer): `hvec3_length`
      +6.2% / +5.9%, `hquat_length` +8.6% / +11.0%, `hvec4_length` +3.0% / +0.3%. That is more than the
      +5.4–8% (vec3) and +10.6% (vec4) 3.4.1 declined for dot's −0 (#16). Buys: 50 public functions no longer
      read address 0 on a 0 handle, the designed answer of every refused allocation (rc 139 through 3.4.2).
      The cheapest guard spelling is already shipped (two `if`s: 7.3 ns against 6.6 unguarded and 7.9 with
      `||`).
    - **`eigen_power`'s sound certificate** (W4, its decision (a)). Against 3.4.2 (W4's second fix pass; 3
      interleaved rounds, one cpu, load 4.6–6.3): 1.4–2.1× where the f64 bound certifies; on positive
      matrices scaled by 2^±300, through the wide squarings, 2.6–11× at n = 8, 18–26× at n = 16, 41–47× at
      n = 40 and 67–71× at n = 64 (5.6 → 393 ms; the first pass's "2.0–3.3×" was against the wave-1 tree);
      1.26× to 5.7× where the double-doubles take an answer back, and 5.3–15× where they refuse it (n = 64:
      6.0 → 88 ms). The cheaper wide bound (fold the four term types into two) is carried, not built. Right
      answers 3.4.2 certified unsoundly are refused (20 of 897 strongly non-normal, 40 of
      52 in the cancellation family; recovering them is carried work). Buys: no wrong Ok on the measured
      families; [[1, a, a, 0], [c, 0, 0, 0], [−c, 0, 0, 0], [0, 0, h, 3/4]] at e = 30..590 was Ok(3/4) with
      ρ = 1, and the cancellation family's 167 short and 33 non-eigenvalue Oks are NO_CONVERGENCE.
    - **`cmat_inverse` and `matrix_condition_number` on a lost pivot** (W4, its decision (c)). A coupled
      complex block at n = 16 / 32 / 64 takes 37 ms / 354 ms / 2.9 s where 0 was answered in 0.19 / 0.88 /
      6.3 ms, about a minute extrapolated at the n = 181 cap (single noisy runs); `matrix_condition_number`'s
      certified answer costs 15–25× 3.4.2's wrong −1 (6.2 ms / 55 ms / 0.48 s at n = 24 / 48 / 96, against
      0.42 / 2.5 / 19 ms), and its lost-pivot 2×2 4.4 → 5.6 µs. Buys: a certified inverse and a correctly
      rounded condition number where 3.4.2 answered "singular". The speedups not built are carried.
    - **The symbolic memo walks and the rewrite meter** (L4's note 3, then W6). On 2^19 terms 2·sin(x)
      (nothing shared, past 2^20 operators): evaluate 42 ms (3.4.2) → 108, simplify 78 → 171, differentiate
      127 → 262; wave 1 had 226 / 334 / 419. `expr_eval_deep1000` +6% from the per-operator counter (L4; it
      asked for a merged-tree A/B). The rewrite scratch region's meter is +6.1% to +17.8% for one deep walk at
      depths 1,600–8,000 (L4); one random rewrite needs 34,040 bytes against 3.4.2's 30,968 and is refused at
      an ALLOC_MAX in that window, 1 such refusal among about 1,260 compared calls, while 27 calls 3.4.2 refused
      now answer (L4's reviewer). Buys: simplify(diff(chain 4,000)) 0.84 → 0.13 s, left-nested integrate at
      20,000 terms 5.85 s → 51 ms, x doubled 24 times 2.46 → 0.17 s, and bounded storage (L4).
    - **The autodiff cut pass's trigger** (W6). A tape whose 3,000 nodes all descend from a miss, none cut,
      95.5 → 103.3 µs (+8% against the pre-W6 tree); the trigger test alone +5% (64.0 → 67.4 µs, 1,500 marked
      nodes; W6's second fix pass); unmarked tapes unchanged (rev16 6,428 → 6,441 ns).
      Buys: 25, 28 and 32 NaN gradients per 200,000 random programs read forward mode's finite answer.
    - **Accuracy repairs whose cost falls on rare inputs** (L1, L6, L7, W1, W7), each against 3.4.2 unless
      named: `sectional_curvature`'s exact tier 14× at dim 3 to 61× at dim 16 on dense tensors, and about 150×
      (0.28–0.29 s) on dense dim-16 operands spanning 2^-1074 to 1, where 3.4.2 was off by up to 3,270 ulps;
      `geo_ray_sphere` at spheres whose r² and |o − c|² flush, against the wave-1 tree, near-tangent rays of
      generic direction 876 → 3,905 ns and the widest exponent spread 825 → 6,661 ns (45% wrong before);
      `geo_triangle_normal` / `_area` 4.6× / 4.8× everywhere (what `geo_triangle_unit_normal` has paid since
      3.4.1); `m4_look_at_rh_checked` 3.3×, once a frame. No cheaper certified form was measured for any of
      them.
    - **Costs on ordinary inputs that the release A/B found and no lane raised** (CHANGELOG 3.5.0: 3.4.2
      against 3.5.0 on the 392 common rows, five interleaved rounds on one core): `ray_capsule_far_axis`
      1.43×, `t3d_compose_checked` 1.41×, `jet_plane` 1.30×, `opt_cg_quad3` 1.23×, the symbolic walks on
      ordinary trees 1.07–1.13×, reverse-mode autodiff where nothing is cut 1.05–1.11×, `tonemap_reinhard`
      1.13×, `cx_mul` 1.11×, `m3_normal_matrix_cof_wide` 1.06–1.085×. None was attributed to its change by
      a measurement; each needs an A/B of the change that caused it before a cheaper form can be sought.
    - **`minkowski_interval`'s 1.62–1.71× floor** (W7). Accept about 1.65× as this codegen's floor, or keep the
      carried row for a codegen change; no sound filter W7 measured beats it (the carried row has the figures).

---

## Toolchain, tracked upstream

⚠ **Nothing is filed upstream without the maintainer's approval.** Once approved:
- cyrius defects go to `cyrius/docs/development/issues/`, and ganita's to
  `ganita/docs/development/issues/`;
- hisab keeps a record of its exposure in `issues/`, and closes it in `issues/archived/` with a
  paired measurement on the bump that fixes it. The upstream agents never edit hisab.

**Pin: cyrius 6.7.18** (ganita 1.2.15, sakshi 2.5.10) since 3.4.1. Nothing has been filed since the
ganita QR record below; the two candidates the 3.4.0 audits found are *Decisions owed* #11.

🔴 **Open upstream (ganita), filed 2026-10-31**: `ganita_mat_eigen_sym` stops its Jacobi sweep at
an absolute 1e-12·max|a_ij|, so a pair coupled weakly against max|a| is never rotated:
[[1, 1e-13], [1e-13, 1]] returns (1, 1) after 0 rotations (exact 1 ± 1e-13). The repro exits 2 on
ganita 1.2.15. hisab's record is `issues/2026-10-31-ganita-eigen-sym-absolute-stop.md`;
`eigen_symmetric` repairs it with its own cleanup since 3.4.1.

🔴 **Open upstream (ganita), filed 2026-10-31**: `ganita_mat_qr` and `ganita_mat_least_squares`
answer wrongly with status 0 once a column's norm passes about DBL_MAX/2. Q = I and R = A (every
entry finite) for the all-equal 2×2 at 1.531–1.657·2^1022, non-finite factors above it, and
least_squares (+∞, −∞) for a system whose solution is about (1, −1). The repro still exits 6 on
ganita 1.2.15, both the 6.7.9 fold and cyrius 6.7.18's. hisab's record is
`issues/2026-10-31-ganita-qr-householder-overflow-near-dbl-max.md`. **hisab has no caller since
3.4.0**: `lyapunov_max`, the only one, runs its own scaled Householder QR (`_lext_ly_qr`), and hisab
does not call `ganita_mat_least_squares`. The record's exposure paragraph still describes the 3.3.7
caller. Close the record with a paired run on the bump that fixes
it.
- **Not filed**: the σ₁ = DBL_MAX that 3.3.7 reported for [[h, h], [h, h]], h = 2^1023. ganita's σ₁
  is 1 ulp low at every scale (0x3FFF…FF for h = 1), within its documented accuracy. hisab's
  wrappers refuse it.

✅ **Closed filings** (each archived in `issues/archived/` with its paired run):
- cyrius: `fncallN` on a capturing closure at top level, and `D082`, a closure's
  `return <: stack call>` booked against the enclosing fn; both fixed in 6.6.16 and closed in 3.3.7.
  `src/autodiff.cyr`'s recipe notes stay while any consumer pins below 6.6.16, which every direct
  consumer does.
- ganita: `ganita_mat_svd` accepting non-finite input and not converging on tiny columns, fixed in
  1.2.12 and closed in 3.3.7; `_lext_mat_all_finite` stays for consumers whose ganita is 1.2.11 or
  older. `atan2`'s signed zero and NaN, fixed in 1.2.11 and closed in 3.3.4. `D140`, `D132`

---

## 3.x.x — demand-gated (unversioned)

Each row names its driver. A driver arriving gets the row a pinned version above.

**Per-step consumers: the parts of 3.4.2's filings it did not take.** The driver is pravash, unless a row
says otherwise. Each part was optional in its filing.
- **A radius walk over occupied cells only** (pravash M6). `spatial_hash_query_radius` still probes
  every cell of the block, bounded by the flat scan once the block passes the entry count. pravash's
  stand-in sorts its cells and binary-searches the block: 19 µs on its 625-particle cloud, against
  Rust 1.4.0's 27 µs.
- **A chain cursor** (`spatial_hash_cell_first` / `spatial_hash_entry_next`), the issue's second form.
  It was not needed once `spatial_hash_query_cell_into` shipped.
- **`num_dst_direct` in 1.4.0's operation order**, so pravash's DST rows can convert with no parity
  change. The issue calls it a nice-to-have.
- **The scope over ganita's matrices.** Eight public functions still take global heap under a scope
  (`src/heap.cyr`): `inertia_box`, `inertia_sphere`, `mat_new_guarded`, `svd_compute_checked`,
  `svd_truncated`, `eigen_qr`, `svd_golub_kahan` and `opt_levenberg_marquardt`. The driver is a
  per-step consumer that needs one of them on its step path.

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
  x86_64 today. See *Decisions owed* #3. `D120`
- **Thread safety of the process-global tables**: Perlin, the einsum scratch, the CGA table, simplex
  gradients, Delaunay scratch and the EPA counter. **Driver**: a threaded consumer. `K005`
- **Benchmark row isolation.** All rows share one never-freeing arena, and adding a row once moved
  another by −32% to −44%. Isolation needs an allocator mark/restore that the stdlib lacks. `D113`

**Rust-era features never ported.** The Rust 1.4.0 line had all of these. The trigger for each is
a consumer asking.
- Monte Carlo integration
- Atkin and segmented sieves (`num_sieve` caps at 10M)
- spline arc length, de Casteljau split
- sparse Cholesky/LU, SOR (a new fn: `solve_pgs_checked` takes no ω)
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

**Fifteen repos consume `dist/hisab.cyr` today, SHA-locked**, read from each `cyrius.cyml` on
2026-11-14. Every one is on hisab **3.4.1** and cyrius **6.7.18**; impetus, prakash and pravash take
it as `scope = "test"`:

| hisab tag | cyrius pin | consumers |
|---|---|---|
| **3.4.1** | **6.7.18** | attn11, dhvani, garjan, ghurni, goonj, hoosh, impetus (test), kimiya, naad, nidhi, prakash (test), prani, pravash (test), svara, ushma |

Since the 2026-11-01 reading every direct consumer moved to 3.4.1 and 6.7.18. hoosh, impetus, kimiya,
pravash and ushma are new. svara's `calc_monotone_cubic` call is now `calc_monotone_cubic_checked`
(`svara/src/prosody.cyr:129`), so no consumer's own source calls a name 3.4.0 retired.

The 3.4.2 work is these consumers' filings (issues, archived 2026-11-14):
- **impetus** (H9) and **pravash**: the allocation scope and the `_into` value operations;
- **pravash**: the DST plan (L6), `num_cg` (P1, L5) and the spatial hash into caller storage (P2, L4);
- **ushma** (M0 probe 10, and pravash's resolve): the sakshi package warning.

**Three more repos carry a copy transitively.** `cyrius deps` vendors it through another dependency,
and each compiles it under its own pin (read 2026-11-14 from each `lib/hisab.cyr` header):

| repo | hisab | via | cyrius pin |
|---|---|---|---|
| jalwa | 2.11.2 | dhvani | 6.6.3 |
| shabda | 3.4.1 | svara | 6.7.18 |
| shabdakosh | 3.4.1 | svara | 6.7.18 |

ranga no longer carries a copy. jalwa's is still the 2.11.2 that dhvani shipped before its move.
`ls ~/Repos/*/lib/hisab.cyr` lists all eighteen; the `[deps.hisab]` grep finds only the fifteen direct
ones.

⛔ **No hisab ≥ 3.1.0 compiles under cyrius 6.6.2** (goonj and attn11 pinned it until they moved to
6.7.18). cycc 6.6.2 refuses `public struct` + `#derive` (`#derive(...) applies to a struct or an enum`),
and the 3.5.0 bundle is refused the same way (measured 2026-11-14). Measured from a
dir pinned to 6.6.2: the 3.1.0, 3.2.1, 3.2.2 and 3.3.0 bundles are refused, while the 3.0.1 bundle
compiles. Under 6.6.3, 6.6.6, 6.6.10 and 6.6.12 the 3.3.0 bundle compiles and a consumer-shaped
program runs correctly. `D146`

⚠ **The pin matters to every consumer for four more reasons**, because a consumer compiles the
bundle under its OWN cycc:
- `m3_mul_vec3` keeps its hoisted z tail, because the natural form is silently wrong below 6.6.5.
  Every direct consumer pins 6.7.18 since 2026-11-14, but jalwa, transitive through dhvani, pins
  6.6.3 and compiles whatever bundle dhvani carries: 3.4.1 at its next resolve.
  (`_cga_build_null_tbl`'s `if`-guard form went in 3.3.4: it guarded against a defect fixed in
  6.6.3, below every pin that can compile a 3.x bundle.)
- A zero produced by negation is −0 from **6.6.8** and +0 below. (3.4.0's exact matrix tiers set
  their zero signs through the sign bit, the same on every pin; the m4 inverse's tier-1/2 formula
  still negates.)
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
- The ten live pins were re-read on 2026-10-31. `D162`

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
| `expr_eval_checked(tree, vars)` (`expr_eval` through 3.3.7) — evaluate an expression TREE | -- | symbolic.cyr (23 fn at 3.3.1); ⚠ domain changed 2.11.2: `(-2)^3` was NaN for hisab's whole history, is -8 since | none |
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

⚠ `eval("sin(pi/4)")` being abaco's while hisab exposes `expr_eval_checked` is not a contradiction:
`src/symbolic.cyr` takes a **tree**, not a string. hisab has no tokenizer at all.
