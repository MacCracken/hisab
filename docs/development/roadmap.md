# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.6.12**. Stdlib `ganita` (6.2.x math umbrella) provides dense
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

## Current — v3.3.0

**Status**:
- Suite **4510** across five harnesses: hisab 585, foundation 429, modules 2319, edge_cases 266,
  abuse 911.
- Constant gate **163/163**.
- Public surface **704** declarations (847 gate probes), enforced since 3.3.0 by the bundle's
  `private` marker.
- **80** benchmarks.
- **35** math modules in `[lib]`, plus the `src/visibility.cyr` marker.
- Toolchain **6.6.12**, sakshi **2.5.6**, ganita **1.2.9**.
- All gates green. Per-release detail is in [`CHANGELOG.md`](../../CHANGELOG.md).

Two releases broke the API, and each has a consumer guide:
- **3.0.0** moved to `Result<T, E>` with no deprecation window
  ([`../guides/migration-3.0.md`](../guides/migration-3.0.md)). **2.24.0 is the supported 2.x line.**
- **3.3.0** made the bundle private and removed the accidental public names
  ([`../guides/migration-3.3.md`](../guides/migration-3.3.md)).

⚠ **hisab ≥ 3.1.0 requires cyrius ≥ 6.6.3** (`public struct` + `#derive`). Measured again on the
3.3.0 bundle from pinned dirs:
- 6.6.2 refuses it with the known `#derive` error.
- Under 6.6.3, 6.6.6, 6.6.10 and 6.6.12 a consumer-shaped program runs correctly, and all 511
  non-public probes are refused.
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

### **[3.3.1]** — the caller's process, and gates that cannot fail

**Inputs that end the caller's process.**
- **`delaunay_2d`, `detect_islands`**: `n > vec_len` aborts with "vec: index out of bounds", and a
  null vec is SIGSEGV (exit 139) in both. Their siblings (`convex_hull_2d`, `triangulate_polygon`,
  `sequential_impulse`) got the guard in 2.12.0; these two did not. `D056`
- **`calc_gradient` / `_jacobian` / `_hessian` / `_partial_derivative`, `opt_levenberg_marquardt`**:
  `alloc(n * 8)` has no derived bound. `calc_gradient(n = 3e8)` exits 139, and a negative `n`
  returns `Ok` silently, because the guards test `n == 0` only. LM has no round-trip guard on
  `nr * np * 8`. The 2.12.0 guard shape already exists in `opt_conjugate_gradient`. `D073`
- **`sectional_curvature`, `geodesic_deviation`**: no null or `_DG_MAX_DIM` check, so
  `(0, buf, buf, 4)` exits 139. `christoffel_symbols` and `riemann_tensor` hard-code `16` instead
  of the constant. Six diffgeo entry points have no abuse coverage. `D091`
- **`num_fft_2d` / `num_ifft_2d`**: the 2.11.1 `HSB_ERR_ALLOC` repair is unasserted. Its guard sits
  after the row loop, which is ~1.4 s per call to reach. Move the guard ahead of the loop, then
  assert both. `D028`

**Gates that cannot fail.**
- **CI security scan**: 3 of its 6 patterns (`syscall\(\s*59`, `syscall\(\s*57`,
  `\bsys_system\s*\(`) are invalid basic regexes. grep exits 2 into `/dev/null`, so the step never
  fails, and a planted `syscall(59, …)` passes. Switch to `grep -E` and add a planted-defect
  self-check. The step comment also says 34 modules; there are 35. `K001`
- **Lock gate**: `cyrius deps --verify` runs only if `cyrius.lock` exists, and the bare
  `cyrius deps` step before it recreates the lock from `lib/`. Deleting `cyrius.lock` turns the
  gate green silently, in both CI and release. `D111`
- **`check-constants.sh`** has three problems:
  - Its duplicate-global arm matches 0 of 221 declarations: `^var [A-Z]` misses `public var` and
    every `_` global. `D108`
  - `POPULATION_FLOOR` is 160 against a live 164. `D109`
  - The 8-ulp slack for comment expressions hides `_SRGB_INV_2_4 = 0x3FDAAAAAAAAAAAB0`, which is
    5 ulp above the correctly rounded `1/2.4` (`…AAAB`). `5/12` and `1.0/2.4` agree in f64, so the
    slack has no reason. Fix the constant and tighten the slack. `tests/modules.tcyr:7515` also
    calls the endpoints exact, three lines above saying they are not. `D097`
- **`check-result-migration.sh`** counts a fn as migrated only if its body contains `return Ok(`
  or `Err(`. `ad_grad_into` forwards through `?` and a tail call, so the gate cannot see it, and
  its Err payload is never asserted. `D122`
- **`tests/edge_cases.tcyr:481-491`** ("complex div by zero → large result") checks only that a
  pointer is non-zero, which cannot fail. The code returns `0+0i`, asserted at `:891`. `D130`

**False safety claims shipped in 3.3.0.**
- **`_GeoJet`**: four places say a consumer can no longer change a jet's `kind`/`t`/`ss`/`face`
  or replace a slot: CHANGELOG 3.3.0, `threat-model.md:225-227`, `migration-3.3.md:40` and
  `geo_diff.cyr:133-140`. That is false on every pin from 6.6.3 to 6.6.12. Type names carry no
  visibility, so `var j: _GeoJet = …; j.kind = 77;` rewrites the field (probed).
  - Fix: replace the named struct with offset-layout `alloc`/`load64`/`store64`, which leaves no
    type name to leak. `_GeoJet` is not API, so this is a patch.
  - Otherwise, correct the four documents. `D062`
- **Tape-backed optimizer recipe**: the documented closure (`autodiff.cyr:536-548`) SIGSEGVs under
  cycc 6.6.2–6.6.4, and six direct consumers pin 6.6.3. The cause: the `opt_*` solvers call the
  gradient as a bare statement, `fncall2(grad, x, g);`, at 10 sites, and cycc lowered that form
  without closure awareness before 6.6.5.
  - Bind the result at those sites.
  - Add a test that compiles the recipe as documented.
  - Correct the header's "re-verified … no longer blocks anything".
  - The recipe also draws a cycc diagnostic on every pin; see *Decisions owed*. `D082`

### **[3.3.2]** — stale claims, dangling pointers, dead code

After this release, every comment that says "filed / recorded / on the roadmap" either points at a
row in this file or reads as history. Comments ship in the bundle, so regenerate `dist/hisab.cyr`.

**Deferral claims whose work is done but whose comment still says otherwise.**

| claim | status | id |
|---|---|---|
| subnormal SVD balancing "filed, not repaired" | closed 2.22.1 | `D002` |
| `_lp_pow2_floor` "filed on the roadmap" | closed 2.18.0 | `D004` |
| SVD bidiagonal "honest limit … on the roadmap" | real limit; point it at **[3.6.0]** | `D001` |
| the CGA occupancy-list cut "on the roadmap" | shipped 2.24.0 | `D046` |
| `cga_norm` "filed on the roadmap" | translator fixed 2.18.0, point 2.21.0 | `D047` |
| `se3_exp` / `se3_log` "deliberately left alone" | shipped 2.16.0 | `D094` |
| tridiagonal survivors | two of the three are now killed | `D008` |
| `spatial_hash_insert` "not migrated yet" | its return is deliberate | `D129` |
| Perlin "tests skipped", and the helper-ordering rule | stale | `D124` |
| `calc_ext` / `ode` "no Result type" | false since 3.0.0 | `D070` |
| `expr_eval` "no NaN constant" | false | `D074` |
| DCT "3% mispick at n = 27" | 0.3% on 6.6.12, and n = 39 flipped | `D025` |
| `cx_div` "still an instance" of the fabricated-answer class | superseded | `D020` |
| the rt sorted-insert note | stale | `D053` |
| `cx_powf`'s guard "on \|a\|²" | dead since 2.17.0 | `K007` |
| foundation's "10-decade `_sc_sweep`" | runs 80 now | `D131` |
| the cylinder-vs-OBB "roadmap 2.8.2" figure | the figure was never the audit's | `D066` |
| `m3_mul_vec3`'s "eight of ten consumers below the fix" | two of those cannot compile 3.x at all | `D141` |
| archived bench-net header "Open … scheduled on the roadmap" | stale | `D135` |

**Wrong line citations and stale facts in comments.**

| where | id |
|---|---|
| `linalg_precision`, `linalg_ext`, `complex`, `num_ext` | `D033` |
| `geo`, `geo_diff`, `geo_advanced`, `collision_*`, including a basis still called "orthonormal" and "Actually, let's return two values" | `D065` |
| `noise_simplex` (calls itself OpenSimplex2; it is classic simplex), `calc_ext`, `optimize`, `ode`, `symbolic` | `D084` |
| `quat`, `lie`, `lie_ext`, two scripts, `ci.yml` | `D098` |
| `tests/modules.tcyr` lines 1–4800 and 4800–9600 | `D125`, `D126` |
| `tests/hisab.tcyr` | `D127` |
| the abuse KNOWN DEFECT register: 11 stale src citations, seven `[Unreleased]` labels that mean 2.9.0, and count drift | `D128` |
| all 35 module headers' `# Usage: include "lib/<m>.cyr"`, and 26 of 35 `# Requires:` lines that disagree with `overview.md`'s compiled include table | `K002` |
| `num_is_prime`'s "0 if composite" | `D029` |
| `geodesic_rk4`'s doc, which never says the connection is frozen | `D092` |
| bench comments: `jac_rev` "2×/12×" (now 5×/47×), and `triangulate_hex_6`'s "every shipped call site" (there are none) | `D078`, `D061` |

**Documents.**

| what | id |
|---|---|
| "open" / "scheduled" claims in dependency-watch, threat-model, overview, CONTRIBUTING, SECURITY and math.md | `D133` |
| audit ledgers still marking later-closed rows OPEN/DEFERRED: annotate them, don't rewrite | `D134` |
| SECURITY's "Known Limitations" and the threat model: `svd_compute` via AᵀA (one-sided Jacobi since ganita 1.2.4); "Jacobi O(n⁵)" (O(n³) typical, O(n⁴) worst); `_COL_MAX_ITER` named as the MPR limit, though it is a dead global | `D136`, `K003` |
| `gjk_intersect_3d`'s "+55%" in src, the threat model and SECURITY; 6.6.12 measures +62% box / +58% sphere | `D041` |
| the pin-dependent results list should add the spatial-hash NaN cell: cell 0 from 6.6.8, so origin queries return the NaN point | `D059` |
| process-global mutable state beyond Perlin is undocumented: einsum arena, CGA table, simplex gradients, Delaunay scratch, EPA counter | `K005` |
| rust-v-cyrius "Why 30–700× slower / Optimization vectors" | `D138` |
| five closed upstream filings have no hisab-side record, and gate comments cite archive paths that never existed | `D139` |
| the CLI-misparse archive says "still present", but it was fixed upstream in 6.6.5 | `D150` |
| the rv64 watch note says "as of 6.5.18" | `D151` |
| port-audit says "parity complete" on outdated premises (`thread.cyr` and `http.cyr` exist at the 6.6.12 tag), and the planning tables promise frustum, spinors and complex eigen/SVD | `D158`, `D160`, `D161` |
| refresh the Boundary table at the 3.3.0 surface: symbolic 23, geo_diff 29, geo_advanced 43, lie 32, lie_ext 25 fns; 34 distinct fns live, not 35 | `D163` |

**Dead and obsolete code.**
- `num_factorize`'s abandoned `stk` block allocates 152 B per call that nothing reads. `D023`
- Ten `_COL_*` globals that no code reads, plus `_COL_F64_NEG_INF`, read only by tests. They are
  hex declarations the constant gate counts, so re-derive 163/163 and the floor afterwards. `D063`
- Eight dead private helpers in four modules: `_vec6_set`, `_vec6_add`, `_vec6_scale`, `_unskew3`,
  `_gjk_simplex_set_count`, `_gjk_simplex_set`, `_lp_mat_copy_into`, `_sh_cell_size`. `D095`
- Retire `_sym_render_f64`, a hand copy of `fmt_float_buf`: 0 of 431,676 reachable inputs differ
  under 6.6.2, 6.6.3 or 6.6.12. Its "filed on the roadmap" in dependency-watch is dangling. `D075`
- `_perm_store_8`'s 10-argument workaround is for a cc5 bug fixed in 6.2.11. An 18-argument store
  was probed correct on six pins from 6.6.2 to 6.6.12. `D077`
- `_cga_build_null_tbl`'s `if`-guard form: its reason ("no consumer past 6.6.2") is void, because
  6.6.2 cannot compile any 3.x bundle. Revert it, unless a 2.24.x backport is wanted (see
  *Decisions owed*). `D142`

### **[3.3.3]** — silent wrong answers: linalg, num, calc

- **`cqr_decompose`** still balances one-ended (`_lp_pow2_floor(max|A|)`). 2.22.1 proved that
  scale lossy but replaced it only in `svd_golub_kahan` and `eigen_qr`. At lead 9, `|R22| = 0`
  where the truth is 2.4u. At lead 1000 the whole subnormal block is wiped, and `rc` is still
  `Ok`. Use `_lp_balance_scale`. `D010`
- **`svd_truncated`** ignores `ganita_mat_svd`'s −1/−2, reports `Ok(0)`, and never null-checks
  `A`. `error.cyr` says `svd_compute` "can only ever succeed", but it returns −2 on a 2×3 input,
  which aliases `HSB_ERR_SINGULAR_MATRIX`. Propagate the failure as `Err` and fix the doc. The
  `svd_compute` signature itself is **[3.4.0]**. `D012`
- **`eigen_power`** seeds `e_0`, so any matrix with `A·e_0 = λ·e_0` returns `A[0][0]`:
  `diag(2,3,4)` and `[[2,0,0],[0,3,1],[0,1,3]]` both give 2. Change the seed and re-derive
  `abuse.tcyr:991`. `D013`
- **`solve_bicgstab`**'s breakdown guards compare degree-2 quantities against an absolute 1e-18.
  At operator scale ≤ 1e-5 it returns `x0`; just above that, a wrong partial iterate. Make the
  guards exact-zero or relative. The 2026-09-09 census missed them because the threshold is a hex
  literal, not `EPSILON_F64`. The error channel is **[3.4.0]**. `D015`
- **GMRES / BiCGSTAB `A_fn` aliasing**: GMRES modifies and keeps the vector `A_fn` returns, and
  BiCGSTAB keeps `v` across the next call. Give them an `A_fn` that reuses one buffer and GMRES
  burns its whole 60-matvec budget (4 with fresh buffers), while BiCGSTAB needs 29 matvecs (6 with
  fresh buffers) and stops at a residual of ~1.1e-10 without saying so. Copy into
  hisab-owned storage; this merges with the **[3.3.7]** restart-loop hoist. It is audit M6 from
  2026-04-15. `D017`
- **`ivl_*`**: round-to-nearest overflow produces a degenerate `[+Inf, +Inf]`, which then poisons
  add/sub with NaN. The LIMIT comment blames `Inf − Inf` from `ivl_div` instead, but 0 of 32
  add/sub pairs of its outputs give NaN. Clamp the inner endpoint and rewrite the comment. The
  entire/empty tag stays demand-gated. `D021`
- **`num_divisor_sigma`** wraps i64 silently for k ≥ 1: `(3·2^61, 1)` returns −4, and even
  `sigma_4(720720)` (n < 2^20) wraps. Two of the three measured wraps come out positive, so a sign
  check cannot catch them. Return the designed 0 on overflow, as 3.3.0 did for `num_crt`. `D022`
- **`calc_monotone_cubic`**'s flat-segment test applies an absolute 1e-30 to a slope. Tangents go
  to zero past an x-scale of 2^99.7, and 40 of 140 y-scales are wrong (0 of 140 with an
  exact-zero test). The repair has two parts, because an exact-zero test alone regresses to NaN.
  Extend sweep H past 2^100. **svara calls this function.** `D068`
- **`calc_adaptive_simpson`** walks the full 2^50-leaf tree when `tol` is ≤ 0, NaN, or halves to
  zero: `tol = 0` exceeded 2·10⁷ evaluations, where 1e-10 takes 257. Reject unreachable
  tolerances with a NaN return; the Result form is **[3.4.0]**. `D071`
- **`calc_integral_trapez` / `_simpson`** accept negative step counts and return an endpoint-only
  estimate as `Ok`, which `abuse.tcyr:1272-1281` pins as accepted. Return `Err` and flip the pins.
  `D072`

### **[3.3.4]** — silent wrong answers: geometry, CGA, spatial, Lie, vectors

- **`geo_ray_aabb[_face]` / `geo_ray_obb[_face]`**: `f64_max`/`f64_min` erase a NaN in one slab
  and launder it into a plausible `t`. `origin.x = NaN` returns `t = 1.0`, bit-identical to the
  clean ray. Sphere, plane and capsule already propagate NaN. This was deferred in 2.10.2, and its
  row was lost in 2.11.0. `D034`
- **`geo_ray_obb` with negative half-extents**: whether the box is solid or empty depends on the
  ray. An axial ray misses at he = −1 and hits at +1, and mixed signs differ by axis. Take `|he|`
  in the parallel branches, or document the precondition. `D035`
- **`geo_triangle_unit_normal`**: `hvec3_cross` is formed unscaled. Legs ≤ 2^-512 return the
  fabricated `(0,1,0)` and legs ≥ 2^512 return NaN: 378 of 1401 leg scales are wrong. Pre-scale
  the edges by a power of two. `D037`
- **`geo_barycentric_coords`**: the Gram determinant is degree four and flushes. Legs ≤ 2^-256
  return the centroid weights and legs ≥ 2^256 return NaN: 890 of 1401 scales are wrong, none of
  it documented. The comment's description of the guard is stale. `D038`
- **`time_of_impact`**: `f64_le(vv, 0)` reports a false impact from about 2^-540 down (2^-538 is
  still correct), and from 2^-560 every
  motion reports impact at t = 0. Repair the escape and also the optimality and lower-bound
  arithmetic on the flushed `vv`; fixing the escape alone turns false impacts into missed ones.
  `D045`
- **`cga_point`** forms `x² + y² + z²` naively. Only |p| ∈ [2^512, 2^512.5) is recoverable, because
  q/2 overflows above that. Form q/2 scaled. `D048`
- **`cga_blade_inverse`**: the null test `|nsq| ≤ 1.78e-15·Σb²` is neither translation- nor
  scale-covariant. A unit sphere at distance ≥ ~6.9e3, and origin spheres with r < 4.2e-8 or
  r > 4.7e7, get the zero multivector as their inverse, and `cga_project` inherits it. Re-derive
  the test in the null basis. `D052`
- **`quadtree_insert` / `octree_insert`** store a NaN point as inside, because ordered compares are
  false for NaN. In-bounds queries then return it. It is pinned as a wrong answer at
  `abuse.tcyr:2080` and `:2096`. `D058`
- **`lie_norm3` / `_lie_norm4`** drop NaN in every component but z, because stdlib
  `f64_max` returns `b` when `a` is NaN. So `su2_exp`, `so3_exp`, `se3_exp` and `su2_from_quat`
  return the identity for NaN input. Five more `f64_gt(x, 0) == 0` guards are NaN-true. `D085`
- **Lie axis guards** (`su2_*`, `so3_from_axis_angle`, `so3_exp`, `lorentz_boost`/`_rotate`) fire
  on non-zero subnormal norms and return the identity across 52 binades. Their "F64_TINY is
  exactly norm == 0" comment has been false since 2.17.0. Divide directly, as `hquat_inverse`
  does; the reciprocal-overflow route recovers only 1 of the 52. `D086`
- **`hvec3_normalize` / `hquat_normalize`** return zero or the identity for 47 of 48 non-zero
  subnormal inputs, and `geo_segment_direction` for 44 of 48. The "documented tree-wide limit" is
  documented nowhere, and the test's companion claim is false. Same direct-division repair.
  `D087`
- **`hvec3_angle`** returns 0 rad for non-zero perpendicular vectors whenever |a|·|b| < DBL_MIN,
  which covers 511 of 1022 normal binades. `foundation.tcyr:1503-1507` and `:1541-1542` pin it as
  "genuinely degenerate", but Cauchy-Schwarz bounds the numerator, so it is not. `D089`

### **[3.3.5]** — the suites: assertions that cannot see

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
- **ganita atan2 exposure beyond the two tripwires**: pin these now as KNOWN DEFECT tripwires, so
  the upstream fix cannot change them silently. `D132`

  | call | today | C99 |
  |---|---|---|
  | `cx_sqrt(conj(-4+0i)).im` | +2 | −2 |
  | `cx_ln(conj(-1+0i)).im` | +π | −π |
  | `cx_powf(conj(-4+0i), 0.5).im` | +2 | −2 |
  | `cx_arg(-0±0i)` | +0 | ±π |
  | `cx_arg(+0-0i)` | +0 | −0 |

- **Fuzz**: `hisab.fcyr` runs 7 fixed buffers, because `cyrius fuzz` supplies no input. Add a
  seeded generator and an einsum-notation target; einsum's is hisab's only string parser. Also,
  `fuzz_quat_rotate` accepts a NaN `rel`. `D118`
- **The one compiler warning in the suites** ("comparison mixes f64 and integer operands",
  `foundation.tcyr:821`) has been left since 3.1.1. `D121`

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
- **Measurement provenance**: 517 claim lines in 27 files are unmarked. `--ratchet` is red against
  the 417 baseline, with 15 files up, and CI runs `--diff` only on pull requests. The fix needs a
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
- **Prerequisite**: `check-result-migration.sh` must see forwarders first (`D122`, **[3.3.1]**).
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
  of returning the current `x`. `D015`
- **`calc_adaptive_simpson`, `ode_backward_euler`, `ode_bdf2`, `ode_bdf`** should report
  non-convergence. The "no Result type" reason has been false since 3.0.0, and `ode.cyr:338`
  documents a return value the code doesn't give. `D070`
- **Plausible-value fallbacks**: `hisab_inverse_lerp` (returns 0), `world_to_screen`
  (`hvec3_zero`), `calc_monotone_cubic` (0), `calc_partial_derivative`, `calc_hermite_tcb`,
  `ode_bdf`. **svara** works around `calc_monotone_cubic`'s 0 with
  `if (r == 0) { return F64_ONE; }`. The dual sentinels stay, because they are a documented miss
  contract. `D069`
- **`expr_eval`** returns +0.0 and writes to fd 2 on an undefined variable or unknown tag. That
  write is the only I/O in library code. Return NaN or a Result, and drop the write. `D074`
- **`m3_inverse` / `m4_inverse`** return the identity for singular or NaN input.
  `m3_normal_matrix` inherits this, and `m4_transform_point` returns undivided coordinates at
  w = 0. `D088`
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

- **`svd_golub_kahan` on a graded bidiagonal `[[t,t,0],[0,t,1],[0,0,1]]`**: accurate through
  t = 2^-36. At 2^-40 it returns `rc = OK` with an exact-zero singular value where the truth is
  2^-41.09, and ‖A − USVᵀ‖_F = 9.1e-13 (~2900 ε, so not even backward stable). From 2^-44 down,
  two exact zeros come back. LAPACK gets all of these to full relative accuracy.
  - **Candidate causes**: the `EPSILON_F64 = 1e-12` deflation and zero-diagonal snap
    (`_lp_negligible_rel`, documented at `linalg_precision.cyr:687-691` as snapping values "where
    LAPACK's dbdsqr would keep it"), and the lack of dqds or zero-shift QR.
  - **Rules for the fix**: use a private tolerance and never redefine `EPSILON_F64`. Commit a
    graded-bidiagonal oracle sweep first, then A/B against 3.3.0. `D001`, `D009`
- **Commit the high-precision oracle for the subnormal-boundary rows.** `tests/hisab.tcyr:4209-4220`
  asserts only `rc`. The committed fixtures show 2 rows at 3 ulp, and a wider band reaches 14 ulp.
  `D003`

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
   best-effort with an end version. `D142`'s guard revert matters only if you backport. `D148`
2. **Minimum cyrius 6.6.4?** On 6.6.3, `&_private_fn` still reaches private bundle functions
   (`&_num_mulmod` probed), and six direct and three transitive consumers pin 6.6.3. 3.3.0 kept
   the floor at 6.6.3 and documented the hole. `D144`
3. **Measurement debt.** Either mark the ~100 claims added since the baseline, or run
   `--update-baseline`, which forgives them. Then decide whether `--ratchet` runs on push.
   `D106`
4. **A possible upstream defect, not filed.** The documented `ad_grad_into` closure recipe draws a
   cycc diagnostic on every pin from 6.6.3 to 6.6.12: a warning inline ("returns a `: stack` pair
   on another path") and an error when built inside a helper fn. The likely cause is the
   diagnostic walking into closure bodies. File it in `~/Repos/cyrius` or not? `D082`
5. **aarch64.** `f64v_dot` is fused (`fmla`) on aarch64 and mul+add on x86. So `hvec4_dot` and
   `hquat_dot`, and through it `hquat_slerp`, give different bits on the two targets. CLAUDE.md's
   Language rule names only `f64v_axpy`/`f64v_fmadd`. Update the rule? An aarch64 test run is
   demand-gated below. `D120`
6. **CLAUDE.md's Consumers bullet** cites `_cga_build_null_tbl`'s guard form as a kept workaround.
   It changes when `D142` lands.
7. **CI consumer-pin matrix?** The public-surface and consumer-build gates run only under 6.6.12.
   The multi-pin build is a manual release step, which is how 3.3.0's `_SYM_EPS` leak under 6.6.3
   was caught. Automate it? `D110`
8. **Missing local tags.** The CHANGELOG has releases 3.2.0, 2.11.1, 2.9.3, 2.2.2 and 2.2.1 with
   no local git tag. The remote was not checked. `D164`

---

## Toolchain, tracked upstream

⚠ **Nothing is filed upstream without the maintainer's approval.** Once approved:
- cyrius defects go to `cyrius/docs/development/issues/`, and ganita's to
  `ganita/docs/development/issues/`;
- hisab keeps a record of its exposure in `issues/`, and closes it in `issues/archived/` with a
  paired measurement on the bump that fixes it. The upstream agents never edit hisab.

**No hisab-filed cyrius item is open.**

⛔ **ONE upstream item is OPEN, in ganita.** It is filed as
`ganita/docs/development/issues/2026-09-30-f64-atan2-signed-zero-and-nan.md`, committed in ganita
as `e49368e`, with a repro that checks 12 rows of the C99 F.10.1.4 table by bit pattern. It is
recorded here as `issues/2026-09-30-ganita-atan2-signed-zero-and-nan.md`.
- **The defect**: `ganita_f64_atan2` picks its quadrant with IEEE compares. So `atan2(-0, -1)` is
  +π (C99: −π), `atan2(±0, -0)` is 0 (C99: ±π), and `atan2(NaN, ±0)` is −π/2. It is the
  unfinished half of ganita's own 2026-09-07 filing, which is marked resolved.
- **Status**: still unfixed in ganita 1.2.10, and cyrius 6.6.12 ships 1.2.9.
- **When it closes**:
  - the two `KNOWN DEFECT (ganita atan2)` tripwires in `tests/edge_cases.tcyr` fail; flip them to
    −π and NaN;
  - flip the tripwires **[3.3.5]** adds for `cx_sqrt`, `cx_ln`, `cx_powf` and `cx_arg(±0, ±0)`;
  - archive the record with a paired run. `D140`, `D132`

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
  x86_64 today. See *Decisions owed* #5. `D120`
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
- Voronoi, Mat4 decompose, plane–plane intersection `D158`
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
  - `f64v_shuffle`, `permute`, `blend`, `swap` and `swizzle` are all still undefined on 6.6.12
    (probed 2026-09-30).
  - The best shuffle-free formulation measured **38 ns vs 25 ns scalar (+52%)** on 6.6.2. That
    pair has not been re-run since. `D152`
- **`#pure` annotations** (from 2.3.4). The original premise, "unsafe CSE interaction", is
  refuted: there is no CSE to be unsafe.
  - Re-probed on 6.6.12: binaries are byte-identical with and without `#pure`, and two allocating
    calls return distinct pointers.
  - ⛔ **The objection to keep**: `alloc()` carries no `#alloc`. Annotating hisab's **286**
    `alloc()` call sites `#pure` would assert a falsehood with no compiler backstop. `D153`
- **Slices (`[T]` / `slice<T>`)** (from 2.3.4).
  - Re-measured on 6.6.12: checked slice indexing costs **3.6–4.2×** a raw `load64`, and the
    unchecked accessor **3.1–3.5×**.
  - The tag's `simd.cyr` has **zero** slice-taking forms, so slices cannot cover the SIMD hot
    paths.
  - 6.6.12's new unchecked typed-array subscript refuses f64 elements, so it is no alternative.
    `D154`

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

⚠ **The pin matters to every consumer for three more reasons**, because a consumer compiles the
bundle under its OWN cycc:
- `m3_mul_vec3` keeps its hoisted z tail, because the natural form is silently wrong below 6.6.5.
  `_cga_build_null_tbl`'s `if`-guard form (wrong below 6.6.3) protects no 3.x consumer, since
  6.6.2 cannot compile the bundle; its revert is **[3.3.2]**.
- A zero produced by negation is −0 from **6.6.8** and +0 below. A NaN spatial-hash coordinate
  lands in cell 0 from 6.6.8 (**[3.3.2]** documents this).
- `dual_pow` is within 1 ulp from **6.6.10**. Below that it takes the older ganita's `pow`: 137
  ulp at 0.9^1024 on every pin from 6.6.3 to 6.6.9.

All of these were measured on the 3.2.2 bundle from pinned dirs and re-measured unchanged on
3.3.0's.

⛔ **The remaining nine are still on 2.x, and the `Result` break fails silently.** A `Result` in
*argument* position degrades to its tag, and the `Ok` tag is 0, which equals `HSB_ERR_NONE`. So
`assert_eq(f(...), HSB_ERR_NONE)` **builds clean and tests nothing**. `D147`
- Hand a migrating consumer `scripts/check-result-migration.sh`, which exists for exactly that
  class and is mutation-proven.
- The reached surface is small (**35 distinct public fns in 8 modules**, per the Boundary table;
  the 2026-09-30 audit counts 34, and **[3.3.2]** refreshes it), so each port is bounded.

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

One row per surface, at the 3.1.1 surface: fn counts are `^public fn` in the module, `_` helpers
excluded. **[3.3.2]** refreshes the counts against 3.3.0 (`D163`).

The last column is **measured, not assumed**: every call of a hisab public fn in each consumer's own
source (`lib/`, `dist/` and `build/` excluded), grepped on 2026-09-14 across abaco and the ten live
consumers.

| Feature | abaco | hisab | live callers (symbols / call sites) |
|---------|-------|-------|-------------------------------------|
| `eval("sin(pi/4)")` — tokenize + parse a string | parses and evaluates | -- (no tokenizer) | -- |
| `expr_eval(tree, vars)` — evaluate an expression TREE | -- | symbolic.cyr (21 fn); ⚠ domain changed 2.11.2: `(-2)^3` was NaN for hisab's whole history, is -8 since | none |
| `sym_integrate(e, var)`, `sym_to_latex(e)`, patterns | -- | symbolic_ext.cyr (25 fn) | none |
| `ivl_*` interval arithmetic | -- | interval.cyr (15 fn) | none |
| `hvec3_cross(a, b)` and the vec family | -- | vec3.cyr | goonj 17/502, prakash 7/11, naad 5/47, dhvani 1/2 |
| `geo_ray_sphere(ray, sphere)`, BVH | -- | geo.cyr, geo_advanced.cyr | goonj 4/5 (`geo_aabb_new`, `geo_ray_new`, `bvh_build`, `bvh_query_ray`) |
| `calc_integral_simpson(&f, a, b, n, out)` -> `Result` | -- | calc.cyr, calc_ext.cyr | svara 2/11, naad 2/2, prani 1/2 (splines, `ease_in_out_smooth`) |
| `num_newton(&f, &df, x0, tol, max, out)` -> `Result`, `num_fft` | -- | num.cyr, num_ext.cyr | naad 5/9, prakash 2/7, svara 2/2, attn11 1/1 (`num_fft` in all four) |
| `dual_*` forward duals, `ad_*` reverse tape | -- | autodiff.cyr (15 dual + 23 tape fn) | none |
| `geo_jet_{sphere,plane,triangle,aabb,obb,capsule}` + partial readers | -- | geo_diff.cyr (25 fn) | none |
| `cga_*` conformal GA (null basis since 2.21.0) | -- | geo_advanced.cyr (27 of its 41 fn) | none |
| `so3_*`/`se3_*`/`bch_*`, `u1_*`/`su2_*`/`su3`/`lorentz_*` | -- | lie.cyr (30 fn), lie_ext.cyr (26 fn) | none |
| `f64_tan`, `f64_fmod` scalar helpers | -- | f64_util.cyr | goonj 1/3, naad 1/1, garjan 1/2 |

**Hisab should never depend on abaco.** Abaco may optionally depend on hisab, and **today it does
not** (measured 2026-09-14):
- abaco's `cyrius.cyml` has no `[deps.hisab]`, its 22 source files call 0 hisab symbols, and its
  own README lists hisab as a *sibling* library, "not a consumer". The "abaco" row in the
  planned-consumer table above is therefore a plan on hisab's side only.

**The reached surface across the ten live consumers is 35 distinct public fns in 8 modules**: vec3,
num, num_ext, calc, calc_ext, geo, geo_advanced, f64_util.
- ghurni `include`s the bundle and calls nothing from it, and nidhi names hisab only in a comment.
- **0 of 10 reference `HSB_*` in `src/`.** prakash's `tests/wave_pattern.tcyr` asserts
  `HSB_ERR_INVALID_INPUT` as the Err payload of `num_fft(…, 3)`, which is the correct post-3.0.0
  idiom (re-read 2026-09-21).
- Eight of hisab's 35 modules have a live caller. The other 27, including everything the rows
  above mark "none", are reached only by hisab's own suites.

**No gate is owed here.** The only git dep is sakshi, and `deps --verify` already fails any
unreviewed dep.

⚠ `eval("sin(pi/4)")` being abaco's while hisab exposes `expr_eval` is not a contradiction:
`src/symbolic.cyr` takes a **tree**, not a string. hisab has no tokenizer at all.
