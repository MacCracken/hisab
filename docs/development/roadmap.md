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

## Current — v3.4.1

**Status**:
- Suite **14152** across seven harnesses: hisab 1350, foundation 1094, modules 3002, modules_b 1843,
  modules_c 2055, edge_cases 2044, abuse 2764.
- Constant gate **324/324** (22 skipped), read exactly, hex enum members included (population
  floor 346).
- Public surface **703** declarations (846 gate probes; all 1489 non-public probes refused),
  enforced since 3.3.0 by the bundle's `private` marker.
- **375** benchmarks.
- **35** math modules in `[lib]`, plus the `src/visibility.cyr` marker. Bundle `dist/hisab.cyr`
  70,654 lines, 3,660,265 B; its sidecar names 7 leaves.
- Measurement provenance: **8** unmarked claim lines (detector false positives), the baseline; CI runs `--ratchet` on every push since 3.3.6.
- Every public function has a comment block (653 of 653); `check-doc-blocks.sh --ratchet` fails on one without.
- Toolchain **6.7.18**, sakshi **2.5.10**, ganita **1.2.15**.
- All gates green. Per-release detail is in
  [`CHANGELOG.md`](../../CHANGELOG.md).

Three releases broke the API, and each has a consumer guide:
- **3.0.0** moved to `Result<T, E>` with no deprecation window
  ([`../guides/migration-3.0.md`](../guides/migration-3.0.md)). **2.24.0 is the supported 2.x line.**
- **3.3.0** made the bundle private and removed the accidental public names
  ([`../guides/migration-3.3.md`](../guides/migration-3.3.md)).
- **3.4.0** retired 27 names for Result-returning replacements, the error channels for
  the fabricated-answer class ([`../guides/migration-3.4.md`](../guides/migration-3.4.md)).

⚠ **hisab ≥ 3.1.0 requires cyrius ≥ 6.6.3** (`public struct` + `#derive`). Measured again on the
3.4.1 bundle from pinned dirs:
- 6.6.2 refuses it with the known `#derive` error.
- Under 6.6.3, 6.6.6, 6.6.9, 6.6.10, 6.6.12, 6.6.13, 6.6.14, 6.6.20, 6.7.9 and 6.7.18 a cross-module
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
- ⚠ **The sidecar names 7 leaves since 3.3.7** (`cyrius distlib` ≥ 6.6.18 writes a compile-verified
  `# Requires` block and `dist/hisab.deps`: math, alloc, vec, ganita, result, str, fmt; the same seven
  on the 3.4.1 bundle). A consumer that received `assert`, `bench`, `args`, `io`, `tagged`,
  `fnptr`, `callback`, `syscalls` or `string` through hisab's sidecar must declare it.

## How to read this file

- ⭐ **Every open item sits under a pinned version heading or in an unversioned section.** A pinned
  version looks like `**[3.5.0]**`. *Demand-gated (3.x.x)* and *Parked* are the unversioned
  sections: they hold work with no driver yet. Nothing moves out of them without a consumer
  asking, and when one does, the item gets a pinned version here first.
- **The maintainer sets every release's number and contents.** The pins below are the plan of
  record, and they move only when the maintainer moves them.
- **Breaking changes are pinned inside 3.x like any other item.** 3.0.0, 3.3.0 and 3.4.0 broke the
  API. A break ships with a **Breaking** CHANGELOG section and a migration guide. A public name
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

### **[3.5.0]** — constants and conventions (breaking)

- **23 `public var` constants that any consumer can write**: `GJK_MAX_ITER` and `EPA_MAX_ITER`
  (EPA sizes its buffer from the latter), `EPSILON_F64`, `F64_TINY`, `F64_POS_INF`, the
  `GEO_JET_*` / `GEO_CAP_*` / `GEO_FACE_*` tags, `AD_NONE`/`AD_FULL`, `HALFEDGE_NONE` and
  `CGA_NUM_BLADES`/`CGA_MV_SIZE`. Probed: `GJK_MAX_ITER = 0` set from a consumer takes effect.
  Convert them to enums, per CLAUDE.md's "enums for constants". `check-constants.sh` reads hex
  members of multi-line enums since 3.3.7, so the verified constants stay in its population. The
  `_NUMX_DFT_MAX` test in `modules_b.tcyr` lowers that cap by assignment; it needs a seam (the cap as
  an argument) before the cap can become an enum member. `K003`
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

**Fabricated answers and channels the [3.4.0] row missed (found by the 3.4.0 audits).** Each needs
an error channel, a representation change or a convention the maintainer sets. The IDs are in
[`audit/2026-10-31-v3.4.0-full.md`](../audit/2026-10-31-v3.4.0-full.md), which holds the repros.
- **`t3d_compose` / `t2d_compose` disagree with applying the two transforms in turn under
  non-uniform scale**: a rotation followed by a non-uniform scale needs a shear, which a TRS triple
  cannot hold. a = (pos (0.3, −0.2), rotation 0.7, scale (1, 1)), b = (pos (0.5, 0.1), rotation 0,
  scale (2, 0.5)), p = (1, 0): applied in turn, (40050997F61C9B96, 3FD49D6E694619B8), the oracle's
  bits; through the composite, y = 3FF49D6E694619B8, four times too large. 50 random t3d pairs
  worst 2.84 (uniform scale 3.6e-15); `m4_mul` of the matrices is right. Refuse, or change the
  representation; documenting the precondition alone would fit a patch. [medium; F-X2]
- **`tensor_get` / `tensor_set` never range-check the multi-index** (code read): shape [2, 2],
  `tensor_set(t, [2, 0], 999.0)` overwrote the next tensor's rank, `[−1, 0]` overwrote t's own
  shape[0]. `tensor_get` returns an f64, so every rejection value is also a legal answer: a
  Result-returning accessor, or a documented precondition like `cmat_get`'s "RAW ACCESSORS --
  PRECONDITION, NOT GUARD". tensor.cyr states neither; the threat model's `tensor_new` row omits the
  index path. On no row since 2026-08-11 confirmed it. [high (2026-08-11); AU3-F12 = AU2-G3-F11]
- **`expr_simplify` / `sym_match` / `sym_rewrite` fold constants within an absolute 1e-15** of 0
  or 1: const(1e-20)·const(1e30) → 0, exact 1e10; k_B = 3B30B0E6D55E647C times x → 0 (the eV/K value
  is kept); `expr_simplify(expr_diff(1e-20·x²))` → 0 at every x. `sym_const_eq` documents the 1e-15
  "for the O(1) coefficients"; the three functions' own docs do not. *Decisions owed* #12(a).
  [medium; SYM-F2, AU2-G6-F2/F3 (DOCUMENTED)]
- **`halfedge_is_boundary` calls an isolated vertex interior**: one triangle (0, 1, 2) with n_verts
  4 → vertex 3 is Ok(0), which a caller cannot tell from a real interior vertex; OpenMesh and CGAL
  answer boundary. *Decisions owed* #12(b). [medium; AU2-G5-F8]
- **`ev100_to_luminance(0)` = 3FDFD4BBAB8B494C (0.4974)**, where Lagarde / Filament (K = 12.5,
  S = 100) give 2^(EV − 3) = 0.125, a ratio of exactly 12.5/π; `ev100_to_exposure` follows
  Frostbite. No doc block. *Decisions owed* #12(c). [medium, a disputed convention; SPC-F5]
- **`cx_powf(0, n)` answers 0 + 0i for n < 0, NaN and −Inf**: n = −1 → (0, 0) where C99 gives
  +Inf, and `ganita_f64_pow(0, −1)` itself returns +Inf on this pin, so the guard comment's premise
  is false. The same guard line as `D019`'s 0⁰ above, and it lands with it; the n < 0 half needs no
  convention. [medium; AU2-G6-F4]
- **`m4_look_at_rh` returns a rank-deficient view for eye = target or up ∥ forward** (no guard, and
  `hvec3_normalize(0)` is the zero vector): eye = target = (1, 2, 3) → a rotation block of zeros;
  eye 0, target (0, 5, 0), up (0, 1, 0) → rows s and u zero. The view basis is undefined, and no
  matrix answers it: an error channel, `D088`'s shape. [medium; AU2-G7-F6]
- **`hvec3_normalize(0)` returns the zero vector** (vec2 and vec4 likewise): AU2 lists it among the
  distinguishable sentinels (|v| = 0, not 1), lane FB among the fabricated answers, and
  `m4_look_at_rh` above is where it becomes one. Keep it as a documented sentinel, or give it a
  `_checked` form as `hquat_normalize` got in 3.4.0. [FB F4]
- **`screen_to_world_ray` returns rc 0 with a NaN direction and an infinite origin when inv_vp's w
  row is 0** (a point at infinity, no ray). FB proposes a `_checked` form; the function already
  returns an integer code, which 3.4.1's near = far repair (AU2-G7-F8) uses. [low; FB F3]
- **`sym_match` / `sym_match_bindings` answer 0, "no match", for a refused binding allocation**
  (code read: a ⚠ 3.4.0 note says so); `sym_match_checked` needs its callers in modules.tcyr and
  modules_b.tcyr moved. `delaunay_2d` / `triangulate_polygon` likewise answer the empty list, a
  legitimate "no triangles", for a refused working vec (REV-AL N4; not re-verified). [low; AL]
- **Documented in their own comments, the same class** (AU2 §2b, unchanged in 3.4.0): `cx_div(a,
  0)` / `cx_inv(0)` → 0 + 0i (NaN/0 and Inf/0 too); a zero axis gives the identity in `cga_rotor`,
  `su2_from_quat`, `su2_from_axis_angle`, `so3_from_axis_angle`, `lorentz_boost` and
  `lorentz_rotate`; `geo_segment_direction(zero)` → (1, 0, 0); `geo_triangle_unit_normal(degenerate)`
  → (0, 1, 0); `geo_ray_triangle` with a NaN direction or vertex → miss; the scalar jet accessors →
  +0 on a kind mismatch; `gamma_spatial` / `gell_mann` out of range → the zero matrix;
  `su3_structure_constant`, `ad_val` and `ad_grad_of` on an invalid index → 0; `ricci_scalar`,
  `sectional_curvature` and `killing_residual` on invalid input → 0.0 ("designed 0");
  `hvec3_angle(0, b)` → 0; `num_mobius(n ≤ 0)` → 0; `num_halton(base < 2)` → 0.0; 12
  allocation-failure exits in geo_advanced answer the miss. Their pins move with them
  (`modules_c.tcyr`'s `cga_rotor(axis 0)` and `geo_triangle_unit_normal` pins, diffgeo's designed 0
  in abuse and modules).

#### Carried from the 3.4.1 close

Every row here was found or left by 3.4.1 and read on its final tree. None needs a public API change unless it says so; rows marked *(no API)* sit here because this is the next pinned section, and where they ship is the maintainer's. The repros are inline, because the scratch that held them is gone.

**Accepted at 3.4.1 with evidence, and not repaired.**

Neither acceptance is the maintainer's: *Decisions owed* #16.

- **Collision costs against 3.3.7, not bought back** *(no API)*: with the canonical order in
  place, `gjk_intersect_box_hit` 1.69 → 1.93 µs, `gjk_intersect_tangent` 1.81 → 2.40,
  `gjk_epa_face_1e9` 17.4 → 28.3, `gjk_intersect_small_edge_1e11` 14.8 → 41.4,
  `mpr_intersect_band_1e6` 4.7 → 9.7, `time_of_impact_sph_hit` 5.3 → 8.1. On the small-edge row,
  20 of the 46 Minkowski supports are the round-8 portal on a pair GJK goes on to settle itself
  (3.3.7: 23 in all); the rest of the time is the exact orientations. The portal round is part of
  the verdict contract: moving it decides 1,328 of 20,636 in-band answers another way, so nothing
  was removed. Re-measure against 3.3.7 before sizing any repair: the rows moved again with the
  rim and key work. [cost; CO3 item 5]
- **`f64v_dot` returns +0 where the IEEE sum is −0** *(no API)*:
  `hvec3_dot((0, 0, 0), (BFF0000000000000 ×3))` → 0, exact 8000000000000000; also `hvec4_dot`,
  `hquat_dot`. BLAS ddot's convention, and now documented on all three. Restoring −0 was built,
  measured at +5.4–8% (vec3) and +10.6% (vec4) per call, +8–11% on the bench's dot rows, and
  reverted. Code read: `f64v_dot` is still called. [low; AU2-G7-F10 = F-S1]

**Wrong answers on ordinary input.**

- **`geo_ray_plane` where n·d cancels above DBL_MIN** *(no API)* (code read: `denom` is a plain
  `hvec3_dot`, and only below `_GEO_F64_TINY` does it reach the exact path): n = (1, 3, 7),
  d = (0.1, −0.2, 0.07142857142857142), o = (1, 2, 3), dist 0.5 → t = 438B800000000000; exact
  n·d = −5.55e-17, t = 4.95e17 = 439B800000000000 (Fractions), a factor of 2 at ordinary scale.
  On a cancelling family 822 t wrong, 55 hit → miss, 52 miss → hit of 6,000, as on 3.4.0. Suggested
  repair: where |denom| < 2^-48 Σ|n_i d_i|, form it with `_geo_plane_dot_fast`. The numerator is
  the other half: d0 − n·o is a float, so a subnormal numerator over a normal n·d keeps a few bits
  (6.8e-5 off; 91 + 253 of 4,000 mixed-scale rows), and n·d from huge setter-built n and d is NaN
  for a finite t. [medium; R2-GEO S8, RV-GEO N6, GEO F3]
- **Implicit ODE steps: Newton's own Ok is not checked to be the continuation root** (code read:
  the certificate runs only on the continuation path, and changing phase 1 changes results 3.4.0
  returned). By a 60-digit oracle, 2,106 of 17,489 phase-1 Oks are off the branch (1,147 BDF-2 and
  931 backward Euler on another root, 28 with no branch): Robertson by backward Euler at
  max_newton 50 is Ok at −7.54e-5, a negative concentration; Van der Pol μ = 100 Ok at +1.7954. Two
  cheap necessary checks at Newton's answer (z on the wrong side of s for sign(c f(s)); g′(z) ≤ 0)
  reject 1,655 of them; `_ode_on_branch` would read them at 63 evaluations. [medium; NUM3 F1, NUM4
  F4]
- **A false miss near a cylinder's rim** *(no API)* (stated in `_epa_rf_wide`'s block): all five
  entries, both orders, as on 3.4.0. Cylinder at 0, axis (0.48078214424880666, 0.8734700569649025,
  −0.07680227442758222), R 0.9529488800401018, hz 1.2999836106123528, ball (−1.1050884276369208,
  −0.9510023066043286, −0.7024436344911975), r 0.008045943372955005: depth 2.4151620860440782e-11, 1 − ρ/R 1.27e-9, answered a miss. 8 of 10,000
  answers, worst 505 eps. An enclosure there needs points on the rim within about 5e-5 rad of the
  centre's azimuth. [medium; F-CO7-2]
- **`_geo_sphere_roots_scaled` has no near-tangent certificate** *(no API)*: near-tangent rays reach it
  past the limits `geo_ray_sphere` documents (r, |o − c| or |d| from 2^510, |d| max(r, |o − c|) from
  2^512: 18% of 142,748 such rays past 16 ulp or with a wrong verdict, 3.4.0 19%, 0 right → wrong) and
  on spheres of radius under 2^-484: 252 of 4,000 in that bin (3.4.0 2,434, 3.3.7 2,445; 0 right → wrong). Exact miss,
  reads a hit: o A21C6E35AF4DF918 221ECA546967D073 A23791F84853FB67, d BBB71B5E73419454
  3BB61D1D3E53D795 BB706CADEFB7FFB1, c A238D3BA2C213A0A 2238A7C8F4011EDC A2385B7DDE23332D, r
  20D779409AF0DECD. Routing them to `_geo_sphere_acc` takes the bin to 47–60 and moves 2 pins: it
  wants the certificate inside the scaled solve. A near-tangent sphere with a r² near DBL_MIN reads
  up to 11.1 ulp, the same class. [low; GEO6, R2-GEO N10]
- **The public capsule on spread exponents** *(no API)* (not re-verified; the flag `uh` is still at
  `geo.cyr`): 49 of 2,500 wrong (one seed) where `_geo_cap_xsolve` alone gets 0, some NaN from
  finite input; 0 right → wrong against 3.4.0 and 3.3.7 on two seeds. Of an earlier 65, 27 had
  end − start overflowing, which one line may route; the other 38 need a look. Family: every
  coordinate of a capsule ray drawn across the exponent range. [low; R2-GEO S6]
- **Infinite-end capsules have no near-tangent repair** *(no API)* (code read:
  `_geo_capsule_ray_inf`): 62 of 1,600 near-tangent rays with one end infinite, judged against the
  limit (3.4.0 166), 1 right → wrong. [low; R2-GEO S7]
- **Ordinary capsules: t past 4 ulp on about 1 ray in 2,000, at most 9.21 ulp** *(no API)*: 15 of
  30,000 on one seed, 13 (max 6.63) on another, 12 of 12,000 mixed. 11 of 15 are barrel near roots
  c/q with c losing exactly one bit (|o_perp|²/|c| in 2..4). Moving the barrel's c threshold from
  two bits to one takes 30,000 from 11 to 2 with 0 right → wrong, and sends origins 1.15–1.41 radii
  off the axis down the slow path: +4.0% mean on ordinary capsules, +7.1% mixed, past the 1.05
  limit, so it was not taken. [low; R2-GEO S2]
- **Rays near a long capsule's axis stay 11–19 of 3,000 past 4 ulp per family, at most 9.14** *(no
  API)* (3.4.0 1,695 on the first). 5 of 6,000 rows went from about 3 ulp to 4.2–4.9, the plain d_perp's rounding having
  cancelled the rest. Refining from 1/4 of the axis instead of 1/8 is four times the rays for 2–3
  of 3,000. [low; GEO6]
- **`hquat_slerp` on an identical pair at huge t** *(no API)* (code read: quat.cyr's comment
  concedes it): a = b = (0.6, 0.8, 0, 0): t = 2^53 gives (0.7071, 0.7071, 0, 0), a plausible unit
  quaternion that is not a; t = 2^54 and 2^55 give NaN. The answer is a. Repro:
  `hquat_slerp(hquat_new(0x3FE3333333333333, 0x3FE999999999999A, 0, 0), <same>,
  0x4340000000000000)`. [low; MAT4, R2-MAT N6]
- **`opt_conjugate_gradient` keeps the flat-tail Ok** *(no API)* (code read: the Ok gate in
  `_opt_wolfe_search` runs only for `qn == 1`): 1e12 + 1 − exp(−x²) from 1.5 returns Ok at x = 194,
  and 1e12 + 1 − exp(−(x − 2^41)²) at z = 162, as on 3.4.0; the minimum is at 0. Its double
  well is fine (999 of 1,000 at a minimum). [medium; NUM4 F1]
- **`opt_conjugate_gradient` refuses scaled objectives** *(no API)* (code read: `a0 = F64_ONE` at
  iter 0, so the first search still starts from a unit multiple of g; not re-measured since the
  L-BFGS half was repaired): c((x0 − 1.3)² + 10(x1 − 2.7)²) from (0, 0), tol c·1e-10,
  c = 2^-1000..2^1000. [medium; NUM F1]

**Range and accuracy.**

- **`lyapunov_max` past what an f64 QR can hold** (code read: stated in its doc; the row moved here
  at 3.4.1 for size, not for API): a direction Q cannot hold, [[1.7·2^-60, 0], [1.7e308,
  1.3·2^-80]] at k = 20 gives −16.94, exact finite-k −3.519 (q₀ = (a/c, 1) with a/c = 8.6e-327
  flushes); 3 of 12 dense 2×2 / 3×3 with entries 10^±150 up to 9× off; a dense exactly nilpotent A
  gives a finite rounding-level value where the finite-k answer is −Inf once every column has met
  its exact rank drop. Repair: Q and the QR in a wide m·2^e form where every column keeps its own
  exponent, and an exact rank analysis of A^t on the flag (rational arithmetic, or `_lext_det_zero`
  on the flag's leading blocks). A new QR with its own oracle. [low; R2A n3, n4, LA row 9, RV-LA N7]
- **`matrix_condition_number` / `cmat_inverse` past cond ≈ 2^53** (code read: both docs state it):
  an invertible matrix whose elimination loses a pivot to rounding answers −1 / 0, which also mean
  singular, non-square and non-finite. An exact or extended tier repairs the answer; a `Result`
  form separates the failures (API). [low; LA F2]
- **`matrix_condition_number([2147483629])` = 3FEFFFFFFFFFFFFF**, 1 ulp under the exact 1 *(no API)*:
  p·fl(1/p), and any 1×1 [x] with fl(1/x)·x < 1 does the same, where cond_F ≥ 1 always. [low; LA F3;
  not re-verified]
- **`eigen_power`, a component through a block's root** *(no API)* (code read: stated in the doc):
  where λ is a 2×2 block's root it is a double-double within about 2^-104, so a component solved
  through an eigenvalue within relative distance g carries about 2^-104/g: 5.23 and 10.02 ulps
  measured. Exact needs λ as (P + √Q)/2 and each det as U + V√Q, with integers past 2^12,000, beyond
  the 302-word scratch. [low; LA3 F5]
- **`eigen_power`'s certificate reads A through a frame that flushes entries below 2^-1074 of
  max|A|** *(no API)*: no wrong certification found. The half of the `eigen_power` row 3.4.1 did not touch. [low; LA F2,
  R2A n5; not re-verified]
- **`eigen_symmetric` / `eigen_qr` disagree on ±λ order** (API, a tie-break): they share the sort
  but not the rounding, on 640 of 796 matrices with exact ±λ pairs, identically in 3.4.0;
  [[0, 1, 0], [1, 0, 1], [0, 1, 0]] is one. A rounding-free tie-break, positive first within the
  routines' error, would need reordering one of them. [low; LA2 F3, RV-LA S1]
- **`solve_bicgstab_checked`'s true-residual confirmation** passes answers whose exact residual is
  above tol·‖b‖ at tol ≤ 1e-14 *(no API)*: 102 of 7,452, relative residual below 1e-12. Stated in
  its doc. [low; LA F5; not re-verified]
- **Autodiff: extended-exponent tangents and adjoints** *(no API)* (code read: the doc block's
  RANGE item): a tangent or adjoint can leave the doubles while the derivative is ordinary:
  ln(5e-324·cos eˣ) at −1 gives −0 forward and −Inf on the tape for −0.1323. Carrying an exponent
  apart from each tangent and adjoint, as `_ad_div_x` does for one op, removes it, at a cost on every
  op that was not measured. Related: the tape keeps the rounding of cancelling rounded contributions
  (3x − x/(3x) at 1e-300: −3.7e283 for 3; forward exact), and compensation is not one-directional
  (x → (1e308, −1e308) with adjoints 1e300 + 10 − 1e300 each: NaN, exact 0; 3.4.0's 0 was luck). [low;
  SYM3 item 5, F1, R2-SYM N3]
- **Autodiff answers with no real function behind them** *(no API)*: `dual_pow` is not a domain
  miss, so (−|x|)^1.5 at 0 reads −0, and (x^0.5)^0 at −0.5 reads 0 through pow(NaN, 0) = 1. A
  constant miss's value can be wrong, not only its derivative: 1/5e-324 is a constant miss, so
  sqrt(|x/5e-324|) reads 0 (decided sentinel, recorded). Forward-finite, tape-NaN rows stay at 2 per
  30,000 programs, all range: ln(5e-324·x/x) at 1.57. [low; SYM4 F3, F4, F5]
- **`cx_exp`, `cx_sin`, `cx_cos`, `cx_from_polar`: 0·Inf NaN parts at overflow** (code read: stated
  in their blocks): `cx_exp(710 + 0i)` = (+Inf, NaN), C99 cexp (+Inf, +0); `cx_sin(0 + 800i)` = (NaN,
  +Inf), exact (0, +Inf); `cx_cos(0 + 800i)` = (+Inf, NaN), exact (+Inf, −0); `cx_from_polar(+Inf, 0)`
  = (+Inf, NaN). Same block: `cx_mul`'s intermediate overflow, (1e300(1 + i))·(1e10(1 − i)) = (+Inf,
  NaN), exact real part past DBL_MAX and imaginary 0; `cx_ln` near |a| = 1, `cx_ln(1 + 1e-10 i).re` = 0,
  exact 5e-21. [low; SYM F2, F3, F4]
- **Quasi-Newton: an extreme dynamic range** *(no API)* (code read: `_opt_qn_first` states it): the
  first trial's parameter a0 scales as a²/c, so past 2^1024 it is not a double and the trial falls
  back to 1: 60 of 60 Err at x ×2^±600, 30 of 60 moved at f ×2^±600. Needs the search parametrised
  by a normalised direction, a path change everywhere. The approximate Wolfe allowance 2^-20|f0| also
  lets a mid-run step rise that far where Armijo is unmeasurable; the Ok gate covers the exit only.
  One Beale start of 12 fails in every version. [low; NUM3 F2, F3, NUM4 F2, R3-NUM N1]
- **`calc_nurbs` past the documented span range** *(no API)* (code read: the doc says about 2^2000):
  with weights 1e±300 and |P| to 1e300, 10 of 800 calls return the designed 0 for a finite point
  (span 2^2108..2^2619; each NaN in 3.4.0), 15 more are finite and wrong. A scale on P as well as on
  w would extend the range. 6 excursions below that range on mixed-size coordinates reach 72 units
  (degree 4). [low; R3-NUM N4]
- **`calc_adaptive_simpson_checked`, the parts #14 does not decide**: noise in cancelling terms has
  a jump's convergence signature and is told from one only by magnitude, which the samples
  understate near the polynomial's roots, so ending such walks needs the integrand's term magnitudes
  (API); 1/√x needs an open or extrapolating rule, and with the 50-level cap even an open rule
  reaches only about 6e-8 (a method change); an overflowing sample is right as `Err`. The backstop's
  size is *Decisions owed* #14. [medium; NUM]
- **`cubic_bezier_ease` near a double root of B_y** *(no API)*: (0.5435, 2.6677, 0.0223, −1.8860) at
  0.49503 → −1.0044821649632793e-17, exact −1.0044821649632942e-17, 20.35 units, 6 of 16,761 over 8;
  harmless in value. Crafted curves whose y1 cancels y2 cost up to 16.6× (24.5 → 406 ns at
  x = 2^-1074), and on 6 of those 36 inputs 3.4.0 was right. [low; R3-NUM N2, N3]
- **`sdf_smooth_union(Inf, Inf, 1)` = NaN**, limit +Inf *(no API)* (code read: `d2 − d1` is
  Inf − Inf). **`sdf_capsule`** with p past DBL_MAX and radius +Inf is NaN, exact −Inf (code read:
  `_toi_vmax(ap) − radius`). [low; RV-CO N4, R2-CO N8]
- **`tonemap_reinhard_extended(+Inf, w)` = NaN**, limit +Inf, as `tonemap_reinhard` *(no API)* (code
  read: documented). **`linearize_depth(1, 1e-200, 1e200)` = +Inf**, exact 1e200 (code read: near/far
  underflows and the denominator is 0 at ndc = 1). [low; LIE2 F2, RV-LIE N7]
- **The octree centre overflows for a box with both corners near DBL_MAX of one sign** *(no API)*
  (code read: (min + max)·0.5 in `_ot_center_into`; `geo.cyr`'s `_geo_mid3` does not): kept
  bit-identical to the packed form on purpose. [low; LIE2 F3]
- **`sectional_curvature`** on cancellation-heavy indefinite curvature tensors: worse than 3.4.0 in
  7 of 150 cases, better in 43 (worst 37 eps against 61 in 3-D, 37 against 31 in 4-D) *(no API)*. Its
  documented 0 for a degenerate plane is also "flat": a distinct refusal is a channel question
  (API). [low; RV-LIE N11, LIE F3; not re-verified]
- **`se3_exp` for θ ≫ 1**: accuracy about ε·θ from forming |ω| in double *(no API)*: inherent, and
  not documented in the function. **`so3_log`'s bound is first order** and takes x87's atan: a
  rigorous all-orders bound and aarch64's polyfill are not done; the sample's 2.35 sits far below
  the 7 ulp, so it can probably be tightened. [low; LIE F5, LIE4 F3]
- **`geo_triangle_normal` / `geo_triangle_area`: plain products** *(no API)* (code read: the doc says
  plain): zero below edges of about 2^-537, +Inf/NaN above about 2^512. **The family's overflow
  convention differs**: slabs answer 0 past DBL_MAX, plane, sphere, triangle and capsule +Inf; each
  documents its own, and no doc says they differ (API to unify). [low; GEO F4, F5, RV-GEO N3]
- **`geo_ray_triangle`'s float t is certified to 2^-33 relative, not to an ulp**: 7 ulps on a far
  hit (3927B14BB6B53ECC against …EC5), 2 ulps at 2^1000. Within the contract; a tighter threshold
  sends ordinary rays to the exact path. [low; G2-F2]
- **`hquat_from_axis_angle` with a non-unit axis** rotates by 2 atan2(|axis| sin(angle/2),
  cos(angle/2)), not by `angle` (API: a `_checked` form or normalising the axis). Documented. [low;
  MAT F4]
- **`halfedge_is_boundary` on a non-manifold bowtie vertex** reads only the fan of its first
  outgoing half-edge. Documented; goes with the isolated-vertex row (AU2-G5-F8). [low; CO F4;
  not re-verified]

**Memory and work bounds.**

- **Symbolic: expression sharing for every walker but `sym_rewrite`** (a design change, not a patch):
  `expr_diff` builds a DAG and every other walker walks it as a tree, so a deep chain's derivative is
  quadratic. `expr_simplify(expr_diff(e))`: an (x^1)^1… chain at depth 2k/4k/8k/16k takes
  0.21/0.84/3.27/13.4 s, a sin chain 0.13/0.52/2.05/8.11 s and 38/138/520/2,018 MB. At depth
  100,000, 519 s and 536 MB, and more than 11.7 GB for the sin chain (3.4.0 exited 139 on both).
  [medium; RV-SYM N6, SYM2 item 7]
- **`sym_integrate` is O(n²) on left-nested products** *(no API)* (documented): contains-var rescans,
  0.57 s at n = 5,000, 2.2 s at 10,000, 8.7 s at 20,000. **A no-op rewrite's temporary stack blocks are
  never reused** (36.6 MB on d/dx sin^17000(x)): one per-call scratch block would remove them. [low;
  SYM F5, SYM4]
- **`expr_to_str` / `sym_to_latex` render a tag no constructor writes as "?"** (code read:
  `symbolic.cyr`) where `expr_eval_checked` and `expr_diff` refuse it. [low; SYM F6]
- **A tape with an out-of-range hand-set edge writes past `adj`** *(no API)* (code read:
  `_ad_add_plain(adj, i1, …)` bounds i1 only below; identically on 3.4.0): `AdTape` is a public struct
  with public setters. [low; R2-SYM N9]
- **`calc_bspline` / `calc_nurbs` take bump heap per call** *(no API)* (code read: three and four
  `_f64arr_alloc(d + 1)`), the class of the multivariate differences' scratch. [low; NUM F2]
- **`ode_dopri45_trajectory` with a NaN y0** is not refused up front (code read: only tol, t0, tf, dt0):
  it returns NO_CONVERGENCE with the one initial point, no fabricated Ok; a refusal is a contract
  addition. [low; NUM F4]
- **`lorentz_is_valid(0)`, `se3_new`'s parts** have no null check (`se3_new`'s doc says so). [low; LIE
  F4; not re-verified]
- **`#must_use` on `spatial_hash_insert`, `spatial_hash_clear`** (API, decided for [3.5.0]): both can
  fail since 3.4.1 and neither carries it (code read). [low]

**API and conventions.**

- **`world_to_screen_checked`'s depth range** (API): `depth = nz/2 + 1/2` assumes [−1, 1] NDC; a
  [0, 1] projection (`m4_perspective_reverse_z`) reads nz from `m4_transform_point_checked`. The
  convention is documented since 3.4.1. A depth-range parameter is a signature change. [medium;
  F-X1]

**Costs a later pass could buy back.**

- **A block-triangular split before the elimination** *(no API)*: a singular block under wide
  entries stays 11–56× 3.4.0 because one elimination alone is 16× at n = 100. Dulmage–Mendelsohn
  would decide the block alone, n/2 × n/2, about 8× cheaper on these, and needs no new bound. Not
  prototyped. [cost; F-LA4-1]
- **The exact test's ceiling at small n** *(no API)*: `_lext_dz_divp` costs wa divisions a row a
  digit, so entries past 2^1000 pay at small n (96 µs at n = 10 against 27 at 2^60): a count of the
  limbs in use closes it. Rank n − 1 with 24- to 50-bit entries has medians touching 10× at n = 10
  (10.7 at worst); m unlucky primes cost m eliminations, up to 7.7× (8.43 at most). [cost; F-LA4-2,
  F-LA4-3, R3-LA N6]
- **`m3_normal_matrix_cof`'s cliff at 2^997 − 2^970** *(no API)* (code read: `_M3_WENT`): one ulp
  either side of the threshold takes 1.79–1.83 µs and 10.2–10.4 µs on one near-rank-1 matrix, 5.7×
  for one ulp; 132 of 901 matrices on the threshold are declined this way. Accepting the first form's
  13u certificate there removes it, at the price of bits. An input spanning 1,400 binades stays
  11.2× 3.4.0, which was 8e7 u off on it. [cost; R3-MAT N3, MAT4]
- **Exact capsule solve on spread exponents** *(no API)*: 213 µs median, 1.73 ms worst directly; the
  public capsule at 1.57 ms, 204× 3.3.7's 7.7 µs (exotic input, and 3.3.7 is wrong there). A sign
  filter before the exact a²c1 − b²c2 in `_geo_xc_sgn2`, or a smaller t3, would cut it. Stack 64–72
  KB (`_geo_cap_xsolve` holds 59,712 B). [cost; R2-GEO S1, S9]
- **`barycentric_thin` above the first 3.4.0 form**: 3.9–4.2 µs against 2.6 (3.4.0 itself 12.8), the
  residue of the barycentric cost row. A filter must correctly round degree-four sums: nine compensated cross components
  at about 61 ns each and a tie certificate. **Declined slivers**, 22×, want `lo` exact. [cost; GEO5]
- **kd-tree queries far outside [2^-500, 2^500]** pay scaled comparisons on every leaf (nearest at
  2^600: 4.1 µs against 1.2): a per-query scale restores the float path. **A finite radius around a
  NaN or infinite query walks the tree** (13.5–20.6 µs at 4,096 points): an early return. **`se3_log`
  allocates an HQuat per call** (code read: its ⚠ COST note): testing θ > π after `su2_log` keeps w ≥ 0
  elements allocation-free. [cost; LIE3 F2, LIE4 F2, RV-LIE N2]
- **`minkowski_interval`'s ordinary-vector floor is 1.8×**; the 1–1.3× target is unreachable with a
  sound filter in this codegen (the cheapest sound form about 1.6×, with a worse 28% accurate share).
  R = 2^-49 or 2^-48 cuts the accurate share to 8.6% / 4.1% but not the 1.8×. [cost; LIE3 F1]

**Simplifications, each needing its own proof.**

- **`_geo_barrel_cert` looks redundant** (code read: still called at `geo.cyr`): with
  `_geo_lows_hold` and the double-double gate in, a variant without it changes no verdict, label or
  wrong count on 13 families. On ordinary rays 2^21..2^40 radii away it gives 0.55 of the previous
  mean where the tree gives 0.75 (1,023 / 1,410 / 1,869 ns; p90 1,624 / 1,785 / 5,696). It is the 10×
  on ordinary rays past 2^20 radii. **`_geo_barrel_entry_ok` changes no answer** (946 of 54,000 rows
  take bin = 0, none changes): it only keeps some rays out of the cap skip. **`_geo_dperp_fine`** reuses
  `_geo_perp_dd`, six exact products where three would do. **`f64_eq(tc, 0)`** in the far path can
  go. [low; GEO6, R2-GEO S3]
- **`_kd_plane_past`'s factor 1 − 2^-50 moves no answer** (code read: 0x3FEFFFFFFFFFFFF8):
  `_kd_scmp` already demands a 2^-47 margin against about 8u. One line. [low; LIE4 F1, R2-LIE N2]

**Instruments and tests.**

- **`check-miscompile.sh` in CI** (code read: its header says "Not wired into CI"): 3.4.0 shipped
  the octree miscompile with this script already in the tree (since 3.3.6), and CI does not run it.
  CI must install the consumer floor pin too. *Decisions owed* #4 is the wider matrix. [medium; INTEGRATOR_TODO]
- **A duplicate `fn` in `tests/hisab.bcyr`, or a repeated top-level `var` in a suite, builds with no
  diagnostic** (code read: CI's duplicate gate checks `dist/hisab.cyr`): `cyrius build` and
  `check --with-deps` said nothing on a planted `_c41_s0`. A grep gate over `tests/` would do. [low;
  MAT4, INS4]
- **Zero signs on consumer pins below cycc 6.6.8** *(no API)* (code read: `f64_neg` in `hvec2_neg`,
  `hvec3_neg`, `hvec4_neg`, `hquat_conjugate`, and `m4_flip_handedness_z`, `m4_ortho_rh`,
  `m4_perspective_rh`, `hquat_slerp`'s b negation, `m4_look_at_rh`): `f64_neg(+0)` is +0 there. Documented;
  a sign-bit flip costs about 1 ns each. `complex`, `autodiff`, `symbolic`, `collision_core` and
  `m4_inverse_checked` were repaired in 3.4.1. [low; MAT F1]
- **Survivors with no killing row**: three in `geo.cyr` that the d_perp repair moved past their rows
  (the |f| term of ed, `_geo_barrel_entry_ok`, the copy's piece against the certificate's), and three
  more (the sign test at gx < 20; n's exact form from 2^-14 rather than 2^-6; the near root's target
  at 2^-48), each with its argument in the report: margin, or a path no generator reaches. They are
  arguments, not proofs. Near-parallel rays are the family: generators must solve for r from the exact
  line distance. [low; GEO6 S3, R2-GEO N12]
- **The `np9` arena label is stale** (code read: `tests/modules.tcyr:6726`, `:6749` say 52,272 B):
  the fixture measures 62,720 B against the 65,536 B bound, 4% of headroom. Re-measure at
  integration; the label moves with it. [low; CO7]
- **The suites under consumer pins**: `abuse` and `modules_b` do not compile under 6.6.3, on 3.4.0
  either (`expected '=', got '['`; `_r5ga_sup_cap`). The others run with failures that are not
  examined except `edge_cases`' 33: foundation 3, hisab 16, modules 46, modules_c 4. Consumers on 6.6.3
  and 6.6.10 get those values today (arg/atan2 9, powf(±Inf) 13, ivl_sin 10, sin(150x) 1). [low;
  NUM4 item 8, F3, R2-SYM N8]
- **Strict overcommit is unverified**: the m0157 group's "a refused reservation is loud" was measured
  under `ulimit -v` only (8 assertions fail); mode 2 needs root, and CI's `vm.overcommit_memory` is
  unchecked. [low; INS]
- **Bench setup checks that cannot tell the path apart**: `b41_minkowski_interval` checks "> 0", its
  `_plain` 14.75, which both forms give; `so3_log_near_pi` checks |ω| ≈ θ, which the antisymmetric path
  also gives. [low; RV-LIE N5; not re-verified]
- **Three `measurement` gaps left**: top-level files are not in `TREE_DIRS`, markers are checked only
  on paragraphs that hold a claim, and dropping `in` from FILE_EXT survives the selftest. [low; INS,
  RV-INS N2; not re-verified]

**Rays**
- **A capsule hit with t past DBL_MAX can read the miss** *(no API)*: 3 of 216,561 capsule rays in the
  seam family, t 2^1108..2^1285, same on 3.4.0 and on every tree since. Input `1 5f9b49d262e1c6df
  dfb524d82fc50b2b dfaf6d3b22120ef2 0f3050074235d302 8f10bdd9fcaba0a0 8effba06520a6780 5f693bdbf04dcd5e
  dfae9ae8a57227d2 5f6277873d06c7ca 5f792ff99e3cda7c dfb54f37a2ae41a0 5f70a4a4a5890d7f
  5fb2802cd828b9da` (the GEO7 driver's row: kind, then the ray, the ends and r as hex doubles). [low; GEO7 F2]
- **`_geo_sphere_roots_full` forms big_d again and does not retest it** *(no API)*: past the limits,
  |d| r near 2^512 and |oc| / r past 2^41, the re-formed offset's a delta overflows, sq is +Inf outside
  the graze range too, and the hit reads the miss: 24 of 6,000 on that family, 24 on 3.4.0. `0
  c334ac4181898235 c33d789fcddc4080 4336dd4d5ab19bd7 5fc4b46d24fa8a47 5fcd8445a33f19c4 dfc6e656bba18f7c
  c017c19ee3b2e9a0 4016a8e3bb00a470 c00d3f5e370b2872 4008a9a02004b682`: exact t 2.68e-138, reads 0. A
  retest there, to the scaled copy, looks like one line. Not tried. [low; GEO7 F3]
- **Past the limit, on origins at the surface, a second class** *(no API)*: t 4 to 300 ulp off where the
  scaled copy is right (`|oc| 2^32.2, r 2^32.2, |d| 2^500.7`, 311 ulp), 6 of the wrong rows at |d| 2^480
  and 2^500. Mechanism not traced; hb² overflowing the near-surface gates is the first suspect. [low; GEO7 F4]
- **`_geo_sph_comp` refuses every ray with r |oc| past 2^512** *(no API)*: its sqrt(ff ococ) is +Inf. No
  wrong answer, `_geo_sphere_acc` answers, but the near-tangent row at r 2^400 (330 ns → 1.5 µs) is
  the cost. A change here moves ed's bits on ordinary rays. [low; GEO7 F5]

**Collision**
- **A ball by a cone's rim** *(no API)*: a cone's side has no great circle of normals, so neither the
  fit nor the section's close reads it. Centre inside, margins 1e-10..1e-2 of the size: 512 of 2,047,
  base and side tied 1,046 of 1,978, worst 1.4e9 eps; 29 of those 6,000 scenes are right on 3.4.0, its
  order's luck, none on the tree before the ball-by-the-rim repair. Taking the no-ball test on the
  absolute least curvature mends 175 more and lifts the 9-point cylinder/box fixture to 66,512 B, past its
  65,536 B pin. Family: ball against a cone at any orientation, centre within those margins of the rim. [medium; F-CO8-1]
- **Asking the core's section wherever the core ran** *(no API)*: mends the one scene of 66,000 balls by
  a cylinder's rim that is right on 3.4.0 and wrong now (cap and side within 16 eps, 15.8 eps), and costs
  every polytope core 19–24% (861 → 1,027 and 553 → 685 support calls). A gate on an open certificate does
  not separate them. [low; CO8]
- **Two sides of near-axial cylinders** at 6.0 and 6.97 eps, the same before the repair. Family:
  balls by the rim of cylinders whose axis lies near a coordinate axis. [low; F-CO8-3]
- **A concentric-box pin 29 ulp above its closed form** in `tests/modules.tcyr`: 0x400FBCBD8FFF3B11
  against r + h_min = 0x400FBCBD8FFF3AF4, 0.026 of 1e-12. Inside 4 eps; the group's comment says exact
  to 11 digits only. [low; F-CO8-2]
- **`_ga_near_ring` reads a nested query's `_GA_NEAR_V`** *(no API)*: `_ga_inside_proven` clears it on
  entry, then runs supports. Where `_toi_near_point` returns 1 with `_GA_NEAR_TETRA` 0 (the exact-origin
  exit) it returns 0 without writing it, so a query run inside a support function leaves a value there.
  Not the alloc_reset class: the memory is live. Poisoning `_GA_NEAR_V` at every `_ga_canon` SIGSEGVs
  modules_b in group "collision: finite shapes near DBL_MAX (3.3.5)"; unpoisoned, a stale non-zero value
  reached that exit 0 times, so no answer is known to move. Fix: `_GA_NEAR_V = 0` on the `w == 0` return. [low; INS6 F1]

**Refusal and contracts**
- **Six entries have no failure channel but a plausible answer** (API): the five collision entries and
  `sym_match` answer "apart" or "no match" on a refused allocation. 0 is the channel the collision
  entries have had since 3.3.5, pinned by `tests/modules_b.tcyr`; each entry needs one line for a
  distinct code, and five modules_b assertions move. [medium; INS5 F1]
- **A 0 handle passed in still faults** *(no API)*: `geo_closest_point_on_plane(
  geo_plane_from_point_normal(zero, zero), p)` (a zero normal gives 0), `ode_bdf_checked(f, 0, 0, 2, h,
  tol, out)`, `hvec3_add(0, v)`, each rc 139. These are inputs, not temporaries, so the refusal sweep
  does not cover them. [low; INS5 F3]
- **`expr_eval_checked` answers `INVALID_INPUT` for a refused allocation** *(no API)*: `_sym_eval` has
  one `bad` flag for that and for an undefined variable, so the channel holds but the code is wrong:
  102 runs of the refusal sweep. [low; INS7]
- **Dead code in `geo_triangle_unit_normal`**: the halving needs a non-finite vertex and a top-binade
  vertex at once, which its guards exclude. 5 mutants survive there. Delete it or find the case. [low; INS5 F4]
- **Four refusal mutants unproven by the suite** *(no API)*: 3 + 1 deep capsule paths with `out` in
  `geo_advanced.cyr` that the abuse fixtures do not reach, and `_geo_cap_side_exact` /
  `_geo_cap_seam_proj` are reached through no public entry of the six suites marked (the exact route
  answers first; their stack forms have only direct white-box rows). [low; INS5 survivors, F5]
- **`_opt_ls_peak` does not ask a step the search falls back on** (`st[2]`) *(no API)*: no run on the
  3.4.1 sets reached Ok that way at a maximum. [low; NUM5]
- **The ODE certificate's close pairs** *(no API)*: on a triangle wave at tol 1e-1 to 1e-12, 18 of 12,116
  continuation answers, all at tol 1e-3 or 0.1, sit on the next root of a pair closer than the gap between
  samples (1/48 of the move) or inside |g| < tol. They were `Err` on 3.4.0. Using the answer's residual
  for the band cost right answers. [low; NUM5]
- **`eigen_power`: Ok on a wrong eigenvalue, because underflow reads as nilpotence** *(no API)*:
  `_lext_gf_loop`'s `if (rs == 0) { return 1; }` takes an underflow of the squarings for a proof. n = 9,
  block upper-bidiagonal, diagonal blocks l/4 [[2,1,1],[1,2,1],[1,1,2]] for l = 1.9, 1.5, 2.0 (eigenvalues
  l, l/4, l/4), the two super-diagonal blocks all 2^1000: rho(A) = 2, Ok(1.9000000000059638,
  3FFE66666666CF51). 20 of 564 chains (2–5 blocks, 2^100–2^1020), lowest coupling 2^320, same bits on 3.4.0.
  `return 0` there gives 0 wrong and 381 of 564 `Err`, many of them right answers, so that line alone
  is not the repair. `_lext_gelfand`'s comment is false here. [medium; R4-CERT N1]

**Autodiff and symbolic**
- **A moving miss's NaN on the tape reaches an input it should not, on a narrow shape** *(no API)*: over
  20,000 programs of 3 to 5 inputs, 21,631 of the 21,720 inputs a miss does not reach are right; 4 read
  NaN where forward is right, because two factors that both descend from a miss, at least one zero, meet
  an infinite slope. The tape has no per-seed notion, and dropping that edge strips the second factor's
  miss of its NaN. Visible, not a finite wrong answer. Stated in `autodiff.cyr`'s header. [low; SYM5]
- **`_pat_instantiate` walks a shared template as a tree** *(no API)*: a rule that fires with a template
  doubled 40 times builds nodes until the meter refuses, 0 after about 2 GiB. Bounded, and 3.4.0 fails
  the same way; `sym_rule` no longer hangs on it, but such a rule cannot answer. [low; SYM5]
- **The first attempt's storage can refuse a call the memo would answer** *(no API)*: {y → sin^L(z)} on y
  doubled 60: L = 1 answers at 29.4 MB, L = 100 at 1.40 GB, L = 300 refuses with 2 GiB allocated before
  the memo walk starts (3.4.0: 0 at both, 8 GB). The table's 129 N visits hold while it loses nothing;
  past its limit the bound is `_SX_MEMO_LOST`, 2^28 overwrites. [low; R4-SYM N1, SYM5]

**Instruments**
- **`einsum_matmul_2x2` swings with code layout**: 1,004–1,165 ns over layouts of one source. A step
  under about 10% in that row is not evidence on its own. [low; INS6 F2]
- **`check-constants.sh` does not see a function-local constant** (`sc = 0x3BF0…;   # 2^-64`, or a local
  with no comment): 15 of 350 plants passed there (`calc.cyr:1314`, `geo_advanced.cyr:7472` among them). [low; R4-SYM N2]

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
   toolchain pin (6.7.9). The multi-pin build is a manual release step, which is how 3.3.0's
   `_SYM_EPS` leak and a 3.4.0 `duplicate variable` refusal, both under 6.6.3 only, were caught.
   Automate it? `D110`
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
   the ceiling sit? If it moves, `modules_b.tcyr`'s three `_numx_bluestein_capped` assertions and its
   `_NUMX_DFT_MAX = 0x100000` restore move with it.
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
12. **Three questions the audits left to the maintainer** (the rows are in [3.5.0]):
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

---

## Toolchain, tracked upstream

⚠ **Nothing is filed upstream without the maintainer's approval.** Once approved:
- cyrius defects go to `cyrius/docs/development/issues/`, and ganita's to
  `ganita/docs/development/issues/`;
- hisab keeps a record of its exposure in `issues/`, and closes it in `issues/archived/` with a
  paired measurement on the bump that fixes it. The upstream agents never edit hisab.

**Pin: cyrius 6.7.9** (ganita 1.2.15, sakshi 2.5.8) since 3.4.0. Nothing has been filed since the
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

**Ten repos consume `dist/hisab.cyr` today, SHA-locked**, read from each `cyrius.cyml` on
2026-11-01 (unchanged since 2026-09-30):

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

⚠ **`svara/src/prosody.cyr:115` calls `calc_monotone_cubic`, which 3.4.0 retired**, and maps its
0 to 1.0 (`if (r == 0) { return F64_ONE; }`). It migrates to `calc_monotone_cubic_checked`, mapping
the Err instead, when it moves past 3.3.x
([`../guides/migration-3.4.md`](../guides/migration-3.4.md)). It is the only call of a name 3.4.0
retired in any consumer's own source (re-read 2026-10-31).

**Four more repos carry a copy transitively.** `cyrius deps` vendors it through another dependency,
and each compiles it under its own pin (read 2026-10-31 from each `lib/hisab.cyr` header, unchanged since 2026-09-30):

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
