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

## Current — v3.3.6

**Status**:
- Suite **8561** across six harnesses: hisab 791, foundation 686, modules 2197, modules_b 2293,
  edge_cases 695, abuse 1899. (`modules_b.tcyr` is the second half of the per-module suite, split
  in 3.3.4 to stay under cyrlint/cyrfmt's 1028 KB input cap; it is 908 KB at 3.3.6, see [3.3.7].)
- Constant gate **209/209** (3 skipped).
- Public surface **703** declarations (846 gate probes; all 802 non-public probes refused),
  enforced since 3.3.0 by the bundle's `private` marker.
- **145** benchmarks.
- **35** math modules in `[lib]`, plus the `src/visibility.cyr` marker.
- Measurement provenance: **8** unmarked claim lines (detector false positives), the baseline;
  CI runs `--ratchet` on every push since 3.3.6.
- Toolchain **6.6.14**, sakshi **2.5.6**, ganita **1.2.11**.
- All gates green. Per-release detail is in [`CHANGELOG.md`](../../CHANGELOG.md).

Two releases broke the API, and each has a consumer guide:
- **3.0.0** moved to `Result<T, E>` with no deprecation window
  ([`../guides/migration-3.0.md`](../guides/migration-3.0.md)). **2.24.0 is the supported 2.x line.**
- **3.3.0** made the bundle private and removed the accidental public names
  ([`../guides/migration-3.3.md`](../guides/migration-3.3.md)).

⚠ **hisab ≥ 3.1.0 requires cyrius ≥ 6.6.3** (`public struct` + `#derive`). Measured again on the
3.3.6 bundle from pinned dirs:
- 6.6.2 refuses it with the known `#derive` error.
- Under 6.6.3, 6.6.6, 6.6.9, 6.6.10, 6.6.12, 6.6.13 and 6.6.14 a cross-module consumer program
  (one call per 3.3.5 and 3.3.6 repair: the hostile-size refusals, `eigen_symmetric`'s
  validation, quadrature on subnormal widths, `num_bisection`'s endpoint root, BFGS at 2^-520,
  `eigen_power` / `eigen_qr` at the edges, a shallow GJK overlap, a subnormal time of impact, the
  slab, OBB, sphere and transform repairs, and 3.3.5's set) builds and runs; every 3.3.6 line
  gives its expected value on every pin, and the outputs are identical on every pin except
  `ivl_sin` below 6.6.9 ([−1, 1]) and `cx_powf(·, ±Inf)` on 6.6.3 / 6.6.6 (NaN, their ganita
  `pow`). The 3.3.4 run also
  found the perspective focal term below 6.6.9 (x87 `sin`/`cos`). On 3.3.0, all 511
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

### **[3.3.7]** — performance, with before/after; the 3.3.6 findings

**Carried from [3.3.6] (rows not finished there).** Both were attempted in 3.3.6 and the variants
measured; neither landed.
- **Box-corner contact normals** (`time_of_impact` on a quaternion box swept onto a corner, and
  `gjk_epa_3d` on a static corner overlap): needs a contract for what a corner normal should be.
  3.3.6's proposal: the normal is the closest-approach direction at the reported t. Re-measured on
  the 3.3.6 tree: an oblique sweep onto 2 × 2,000 corners is off by up to 1.94e-5 (gap 4.9e-10 at
  t; p99 5.5e-7, identical to 3.3.5), a head-on sweep by up to 2.65e-7, and `gjk_epa_3d` on 1e-9
  static corner overlaps by up to 2.35e-5 (p99 1.25e-5; `mpr_penetration` ≤ 4.5e-8). Worst sweep:
  box c (BFD8463D5220ABF8, BFE03710941A16AA, BFDFEE6BB7BEA914), h (3FE9239026A1A6D0,
  3FF601A8FA8A8A26, 3FF1BB8EF1294A53), q (BFE42E23120AB4CD, 3FD861CD1E099C3D, BFE16B0345C23B8B,
  BFD9ABC7F99E7341); sphere c (C006AF8D81D92237, 3FD4B6DD6742CA98, 40097A5107745416), r
  3FECBE3DB985B912, vel_b (3FD4EC69052694A8, 3FC0EC9E0F2236C0, BFF795D7F4AA6262), max_t 10. A
  variant that fixes corners (2.27e-5 → 9.0e-7) turned 18 OBB-edge and 3 capsule-side normals right
  → wrong at 1e-6 and cost +6.5% to +13.9% support calls, so it was not applied.
- **EPA/MPR depth on a box inside a ball, second local minima**: every face normal is a local
  minimum of the dilated box's support (h_M(n) = R + Σ hᵢ|uᵢ·n|), and the wrong answers stop at
  the second-thinnest face. Re-measured on the 3.3.6 tree (identical on 3.3.5): with h and R
  log-uniform in [0.01, 10], 8–9 (`gjk_epa_3d`) and 6–7 (`mpr_penetration`) of 1,000 concentric
  pairs; with R / h_min in [48, 520], 44–51 and 35–58 of 1,000. Repro: box c (BFE11889B3A631E6,
  3FD9DA07E939DA64, BFDAAD9503B36EF8), h (3FAFAF24B4AE3B09, 3FB308FBA5F9E2C3, 401C34E4BDE10586),
  sphere at the same centre, R 4029AA28FE4F486F: exact 12.894227117761449, `gjk_epa_3d`
  12.906699824523578 (R + h_mid). A variant took the second-face answers 15 → 6 (EPA) and 14 → 7
  (MPR) at +34.5% support calls on that family and +5.7% to +13.6% elsewhere, so it was not
  applied; 3.3.5's 1–3 extra polish starts cost +148% to +348%.

**3.3.6's costs to buy back.**
- **Quadrature's power-of-two frame on every call**: `calc_integral_gauss5` +20% (49 → 59 ns),
  trapez +11%, simpson +7%, adaptive +9.5%, `num_bisection` +10% (CHANGELOG 3.3.6 *Performance*).
  An ordinary-width fast path that keeps the bits is the target.
- **`gjk_intersect_3d` on a shallow separated pair** +28%: the enclosure proof is tried and fails.
  A cheaper refutation before the proof is the target.
- **The ray repairs' remaining costs** (same-binary A/B against 3.3.5, CHANGELOG 3.3.6
  *Performance*): `ray_sphere` +8% on the ordinary fixture, a near-surface origin 3.4× (the exact
  c), an origin on an AABB face +57% and on an OBB face +10% (the underflow checks), a ±DBL_MAX world
  box +11%, `hquat_rotate_vec3` +10% (42 → 46 ns: the overflow test spills the hot locals; cycc
  keeps five in registers). The `_GEO_*` constants are `var` globals, a memory load per reference
  (CLAUDE.md's enum rule).
- **`modules_b.tcyr` is 908 KB of cyrlint/cyrfmt's 1028 KB input cap** (791 KB at 3.3.5): plan the
  next split before a release pushes it over, as 3.3.4's did for `modules.tcyr`.

**Found during 3.3.6, not repaired (not on the [3.3.6] rows).** Each was found by a 3.3.6 lane or
its reviewers and reproduced by an independent skeptic on the 3.3.6 tree and on the 3.3.5 tag (all of
them answer the same on 3.3.5 unless noted), under cycc 6.6.14. Review and repair each here.

*Linear algebra*
- **`svd_golub_kahan` answers on the overflow line `eigen_qr` now refuses**: [[h, h], [h, h]] with
  h = 2^1023 (and its negation) returns Ok σ = (DBL_MAX, 0), and [[h, h, 0], [h, h, 0], [0, 0, 1]]
  Ok (DBL_MAX, 1, 0), where σ1 = 2^1024 overflows (both trees). 3.3.6's `eigen_qr` returns
  Err(NO_CONVERGENCE) on the same matrices (3.3.5: Ok `0x7FEFFFFFFFFFFFFE`), so the pair now
  disagrees. h = 2^1022 is right in both.
- **`eigen_qr` 251 eps·|λ|max off near the overflow line** (both trees, identical bytes; documented
  worst 8.4): 4×4 row-major 190d06ccb7341272 761b9ecd25c64fe3 0 f61104ae3d73c48f /
  761b9ecd25c64fe3 7d58a78f89e5db7b 0 2617e3bae5272a41 / 0 0 ffdffffffffffffc ffdffffffffffffb /
  f61104ae3d73c48f 2617e3bae5272a41 ffdffffffffffffb ffdffffffffffffc returns Ok {0xFFEFFFFFFFFFFFFC,
  0x7D54B80000000000, 0x7D2F680000000000, 0x190D06CCB7341272}; exact {−1.797693134862315e308,
  −9.9792e291, 1.37106e229, 6.2984336284766114e295}: 5.293e295 against 6.298e295 and 1.003e295
  against 1.37e229.
- **`eigen_power` accepts a non-dominant answer both trace floors pass**: [[1e-3, 0, −1e20],
  [0, 1.3, 0], [0, 0, −1]] (0x3F50624DD2F1A9FC, 0xC415AF1D78B58C40, 0x3FF4CCCCCCCCCCCD,
  0xBFF0000000000000) returns Ok(0xBFF0000000000001) where λ1 = 1.3 (both trees); |tr A|/n = 0.10
  and √(|tr A²|/n) = 0.95 sit below 1, a |tr A⁴|/n floor (1.065) would catch it, at O(n³) per answer
  (also at 1e168 scale and n = 5). The lane's families: 8 + 34 more caught by tr A³ / tr A⁴ floors;
  complex-dominant (30 / 15), non-normal (455 / 4,000) and larger-only (766 / 4,000) remain.

*Numerics*
- **`num_bisection` fabricates a root from an underflowing sign test**: f ≡ 0x16A8000000000000
  (≈2^-661, no root) on [0, 1] with tol = 2^-1074 returns Ok(1.0) (both trees; f ≡ 1 is refused).
  f(lo)·f(hi) underflows to 0 and reads as a sign change.
- **`calc_integral_gauss5` weights each sample before summing**, so subnormal samples round away:
  f ≡ 2^-1074 on [0, 2] gives 2^-1074 (truth 2^-1073; simpson right). Both trees.
- **`opt_conjugate_gradient` fails where 3.3.6's BFGS now converges**: on the BFGS scale fixture
  (f = (x0 − k)² + 1e5(x1 − 2k)² from (−k/2, 5k/2), tol k·1e-8) it returns Err(NO_CONVERGENCE) at
  k = 2^-486, 2^-500, 2^-600, 2^496, 2^500 and Ok at 1, 2^-485, 2^495 (both trees): an honest
  error, but the same correction-pair rescale would carry over.
- **`calc_bspline` / `calc_nurbs` dereference a failed de Boor allocation**: with ALLOC_MAX = 24,
  degree 3, 4 points, knots 0..7, t = 3.5, `_f64arr_alloc(4)` returns 0 and both exit 139 (both
  trees); `_f64arr_alloc`'s "every caller checks" comment is false. In practice it needs
  n_pts > 2^28 or an exhausted heap.
- **DST / DCT past the 2^20 Bluestein cap stay quadratic for lengths that are not powers of two**
  (`num_dct(2^20 + 1)` and `num_dst` from 524,288 take the direct path; the comment at
  `num_ext.cyr` says so). Lifting the cap is a memory-policy choice: Bluestein's buffers grow with
  the next power of two above 2n.
- **`num_fft`'s twiddle recurrence** loses accuracy with n: at 2^20 (uniform ±0.5 complex input,
  against an accurate-twiddle radix-2 reference) max|ΔX| / max|X| = 1.30e-11, rms 9.1e-12 (both
  trees).
- **The interval module's NaN rule is not uniform**: `ivl_abs([NaN, 1])` gives [0, 1] (the NaN
  dropped) while `ivl_abs([1, NaN])` gives [NaN, NaN]; `ivl_exp([NaN, 1])` and `ivl_sqrt([NaN, 1])`
  give [NaN, hi]; `ivl_sin` and `ivl_mul` give [NaN, NaN] (3.3.6 made `ivl_sin` so).

*Instruments and tests*
- **`bench-history.sh` drops a row whose name has a character outside `[A-Za-z0-9_]`, silently**:
  a replayed 80-row log with one row renamed `vec3_cross_2e-600:` records 79 and exits 0 (the
  unparsed-line check uses the same name shape). Latent: no current row has such a name.
- **Mutation survivors that are real gaps** (from 3.3.6's seeded 200-mutant sample, seed 336):
  - `src/mat3.cyr:230` `m3_set(m, 1, 2, …)` → `(m, 1, 3, …)` survives all six suites: no
    assertion reads `m3_from_quat`'s (1, 2) entry.
  - `src/collision_mesh.cyr:153` `load64(f + fi * 8)` → `fi / 8` survives: it flips
    `_col_orient2d_sign` on near-collinear triples past the filter and certificate (36,281 of
    271,152 in a model; Cyrius repro 0xBFE1149A64F3AF6C 0xBFE11FD6E2A04070 0xBFE1F2BEFD4A0E3C
    0xBFB4AE9B8BB03850 0xBFE0939B9532AE0E 0xBFE9F1C0ABFB283A: +1, mutant −1, exact +1).
  - `src/linalg_ext.cyr:1929` (`_lext_grows`'s rescale loop from 1, not 0) survives: A =
    [[s, −3s, 0], [−3s, s, 0], [ps, ps, rs]] with s = 0x7FCCE93BBB40654B, ps = 0x7FE5845C58277F0D,
    rs = 0x7FDC768B2082CF59 gives `eigen_power` 0x7FECE93BBB405EAA, the mutant 0x7FECE93BBB4061FA,
    truth 4s = 0x7FECE93BBB40654B (15 of 6,000 in that family differ).
  - Of the sample's other 41 survivors, 3 are recorded equivalent (m0036, m0048, m0081) and 38
    are untriaged, among them one at `src/geo.cyr:1842` of the pre-lane-C tree
    (`scripts/mutation-test.sh`, seed 336).
- **`check-constants.sh` accepts a decimal comment within 4 ulp**: planting +1, +3 and +4 ulp on
  `_SH_Y20` (comment `0.31539156525252000603`) passes; +5 fails. Expression comments are
  evaluated in doubles. A comment carrying the correctly rounded digits should pin the last bit.
- **69 dead test helpers**: 36 in `modules.tcyr`, 32 in `modules_b.tcyr` and `_ab_f64_eq` in
  `abuse.tcyr` are defined and never referenced in their file (6 were dead before the 3.3.4 split).
- **A test assigns the private `_NUMX_DFT_MAX`** (`modules_b.tcyr`, set to 16 and restored with a
  literal), a [3.5.0] `K003` enum conversion would break it at compile time.

*Collision*
- **Shallow capsule/sphere depth stops at EPA's certificate**: on 600 capsule/sphere overlaps
  ~1.2e-12 deep (exact rc + rs − dist), `mpr_penetration`'s depth is a median 1.00× but up to 490×
  the exact one (97 more than 2×, 6 more than 64×); `gjk_epa_3d` up to 61× (25 more than 2×). The
  absolute overshoot stays ≤ ~7e-10, the `1e-10·max(|d|, |p|)` stopping rule. Worst pair: capsule
  c (BFD0FBAD4706B994, BFEB1D8981B21794, BFB888D305897130), a (BFEF931CE85D7103,
  BFE39028303E5122, BFEA53A55271639C), r 3FEE121D8D91B466; sphere c (3FF2EF3E6B7D075E,
  BFFDDB253F4663AC, BFF1976CD6009E13), r 3FF188C48375A7E4: exact 1.1995e-12, mpr_penetration
  5.8716e-10 (both trees). Sphere/sphere at 1e-12: 64 / 44 of ~990 `gjk_epa_3d` contacts
  overestimate by > 1e-14·(ra + rb) (worst 2.26e-10 on 1.38e-12, a 3.3.5 contact too; of the
  contacts 3.3.6 newly proves, 47 of 901 / 27 of 912, worst 9.25e-12 on 2.39e-12).
- **Shallow end-cap overlaps are missed by every entry point, and verdicts depend on operand
  order**: of 200 cap/sphere overlaps 1.2e-12 deep, `mpr_*` miss 66 on both trees and `gjk_*` 67
  (3.3.5: 160); at 1e-13 `mpr_penetration` misses 85 of 300. Sample: capsule c (3FDE58E76BCB5D0C,
  3FDA1288C2129D30, 3FE2C4E197B5003A), a (3FF9B7BC0A658B2F, BFE25D1A6A71A46A, 3FE6F26E1A3A0A16),
  r 3FF158071624A44D; sphere c (400955BE53E03EE1, 3FF30FB2A16B4BD4, 3FF935B51250D8BB), r
  3FE59D2FCA996935 (exact 1.19997e-12). The worst pair above answers 0 from both `gjk_*` as
  (capsule, sphere) and a contact of depth 4.52e-11 as (sphere, capsule). `gjk_*` still miss 113
  of the 600 side and cap overlaps (3.3.5: 297) and answer depth 0 on 11 more.
- **Separated pairs within half an ulp of the largest coordinate read as contacts** (both trees,
  all four entry points; the documented within-1-ulp class): on ~30,000 separated sphere/box pairs
  near (1000, −2000, 500) the widest gap with a false contact is 5.68e-17 relative (~0.5 ulp of
  2000), every face-pair one being a pair whose rounded support points touch; none from 1.5e-16 to
  1e-10. Not MPR-specific (GJK gives the same verdict on 97%).
- **Collisions at 2^-1030**: on 2,000 sphere pairs within ±3 subnormal steps of contact, `gjk_*`
  are wrong on ~1,003 (every overlap missed) and `mpr_*` on 102, on both trees.
- **Out-of-contract support functions** (the premise 3.3.6 documented): with a sphere support
  routed through 2^16 ((c + K) + r·n − K), `gjk_*` turn 99 of 960 separated near-tangent pairs
  (gaps 1e-9 to 1e-13) into contacts 3.3.5 answered as misses (3.3.5 already had 4 such
  contacts, `mpr_*` 47 on both trees); none at gaps of 1e-10 and wider.

*Rays, triangles and transforms*
- **A capsule shorter than ~2^-537 is not solved as a capsule**: |ab|² underflows. Capsule
  (0,0,0)–(2^-600,0,0), r = 2^-602, origin (2^-601, 2^-600, 0) along −y: exact barrel hit
  0x1A68000000000000 (3·2^-602); 3.3.6 answers a miss, 3.3.5 2^-600 (a flushed tangent root of
  the start sphere, which the ray also misses). On 4,000 capsules with |ab| from 2^-1000 to
  2^-539, 3.3.6 is wrong on every barrel and end-cap hit (380 + 87) and 14 start-cap hits; 3.3.5
  on 2,115; none right → wrong.
- **The barrel's classic quadratic loses accuracy with distance along the axis**: at a fixed
  perpendicular offset of 3r the median error grows as ~0.2·|o − a|/r ulp (19 ulp at 1e2, 2.2e3 at
  1e4, 2e7 at 1e8; both trees). o = (0x40B33F8C8241857E, 0x40C11DCB5B12F347, 0x40C5E584F5870178),
  d = (0x3FEE0C4B13CD9C32, 0xBFDC585CFBBBDF75, 0xBFB373E859BE4F37), a = (0xBFE4115D01803530,
  0xC01042355B1B6BD0, 0x4011477561717E7E), b = (0x40CB8F220269B35E, 0x40D87EC08203A8B0,
  0x40DF5299636F48FC), r = 1: exact 0x3FFEBDA633ED69E0, both trees 0x3FFEBDA633ED565F (4,993 ulp).
  At ordinary scale (5,000 barrel hits per family, bit-identical on both trees): ends in [−5, 5]³,
  origins in [−10, 10]³: median 4.8 ulp, p99 ~100, max ~1,600, 55% past 4 ulp; ends in [−1, 1]³,
  origins in [−4, 4]³: median 1.86, p99 17.7, max 585, 21%.
- **Rays from an origin within an ulp of a centre's coordinates read a miss as a hit, or the contact
  at t = 0** (sphere and capsule caps; o − c rounds the origin onto the centre or the surface):
  centre (1.5·2^600, 1.25·2^600, 0), r = 1, origin (0, 2^500, 0), raw direction = the centre gives
  t = 1.0 (exact: a miss, the centre 2.5e150 from the line; 3.3.5 NaN); centre ~2^100 with origin
  (0, 2^40, 0) gives t = 1 on both trees. o − c rounds to −c. The capsule's caps share the class
  since 3.3.6 answers a capsule contact as the sphere does (64 of the 3.3.6 review's edge family,
  where o − cap rounds the origin onto a cap sphere one ulp inside it).
- **The sphere and capsule jets refuse hits when the radius is below an ulp of the hit
  coordinates**: g = p − c rounds to zero and reads as grazing. Unit sphere at (3.5e252, 0, 0),
  origin 0 along +x: primal 0x745E8D87446FDE22 (right), jet a miss (true dt/do = (−1, 0, 0),
  dt/dr = −1; 3.3.5 primal NaN); a unit sphere at 1e17 refuses on both trees; capsule
  (0.05·DBL_MAX, 0, 0)–(0.06·DBL_MAX, 0, 0), r = 1, from 0 along +x: start-cap hit
  0x7FA9999999999999, jet a miss. (A power-of-two scale sweep refuses none on 3.3.6; 3.3.5 refused
  3,339.)
- **`geo_jet_triangle`'s vertex gradients lose ~eps·|o − v0|/size**: on a 1e-6 triangle at
  distance 1, dt/dv0..2 have relative error median 3.6e-10, max 1.4e-5 (dt/do ~1e-16); they are
  noise from triangle sizes ~1e-12 (tail) to 1e-14 (typical). Both trees.
- **`screen_to_world_ray` has the reciprocal defect 3.3.6 repaired in `world_to_screen`**: with
  inv_vp = 2^-1074·I or 2^-1024·I on 800×600, the centre gives origin (NaN, NaN, −Inf) and direction
  NaN, the corners ±Inf / NaN (truth: origin (nx, ny, −1), direction (0, 0, 1)); 2^-1022·I is right.
- **`geo_closest_point_on_sphere` is NaN when point − centre overflows**: point (DBL_MAX, 0, 0),
  centre (−DBL_MAX, 0, 0), r = 1 gives (NaN, 0, 0) (truth (−DBL_MAX, 0, 0)); point
  (DBL_MAX, DBL_MAX, 0) gives (NaN, 0, 0) (truth (−DBL_MAX, 0.447, 0)). Both trees.
- **`geo_barycentric_coords` on an exactly colinear triangle**: the Gram determinant is 0 and the
  centroid fallback answers (⅓, ⅓, ⅓) whatever the point; with a rounded determinant the weights do
  not reproduce p (242 of 400 full-mantissa colinear triangles, p on the line; both trees). Needs a
  contract for a degenerate triangle (a [3.4.0] fabricated-answer question).
- **`hquat_rotate_vec3` for non-unit q**: it computes v + w·t + u×t (t = 2·u×v), which is a rotation
  only for unit q, and is neither q·v·q⁻¹ nor q·v·q* otherwise; its accuracy against that formula
  degrades with |q| for subnormal v (out of a 6u·S bound: 0 of 95 for |q| ≤ 2, 126 of 282 for
  (2, 16], 509 of 623 for (16, 1024]; both trees). What a non-unit q should mean is a [3.4.0]
  contract question.

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
3. **aarch64.** `f64v_dot` is fused (`fmla`) on aarch64 and mul+add on x86. So `hvec4_dot` and
   `hquat_dot`, and through it `hquat_slerp`, give different bits on the two targets. CLAUDE.md's
   Language rule names only `f64v_axpy`/`f64v_fmadd`. Update the rule? An aarch64 test run is
   demand-gated below. `D120`
4. **CI consumer-pin matrix?** The public-surface and consumer-build gates run only under 6.6.14.
   The multi-pin build is a manual release step, which is how 3.3.0's `_SYM_EPS` leak under 6.6.3
   was caught. Automate it? `D110`
5. **Missing local tags.** The CHANGELOG has releases 3.2.0, 2.11.1, 2.9.3, 2.2.2 and 2.2.1 with
   no local git tag. The remote was not checked. `D164`
6. **Three [3.3.5] rows landed differently from their text** (CHANGELOG 3.3.5 has the evidence):
   - `num_bisection` refuses a NaN f(a) / f(b) but still bisects ±Inf ones (the row said
     "non-finite"; refusing ±Inf broke four correct answers, e.g. ln on [0, 2] → exactly 1).
   - `calc_adaptive_simpson`'s floored acceptance is kept: every stricter rule measured either kept
     the aliased repro wrong or walked other integrands past 200M evaluations. A tripwire pins the
     wrong 1.1 the aliased integrand returns.
   - Overflowing quadrature now takes the walk its in-range twin takes: calls that returned +Inf
     after 5 evaluations can run past 2·10⁹ evaluations (> 60 s), the [3.4.0] `D070` class.
   Accept, or schedule a different outcome.

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
  x86_64 today. See *Decisions owed* #3. `D120`
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
2026-10-11 (unchanged since 2026-09-30):

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
and each compiles it under its own pin (read 2026-10-11 from each `lib/hisab.cyr` header, unchanged since 2026-09-30):

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
