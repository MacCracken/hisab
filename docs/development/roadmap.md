# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.7.9**. Stdlib `ganita` (6.2.x math umbrella) provides dense
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

## Current — v3.4.0

**Status**:
- Suite **10593** across seven harnesses: hisab 1010, foundation 856, modules 2642, modules_b 1425,
  modules_c 1329, edge_cases 1000, abuse 2331.
- Constant gate **258/258** (4 skipped), read exactly, hex enum members included (population
  floor 214).
- Public surface **703** declarations (846 gate probes; all 1053 non-public probes refused),
  enforced since 3.3.0 by the bundle's `private` marker.
- **209** benchmarks.
- **35** math modules in `[lib]`, plus the `src/visibility.cyr` marker. Bundle `dist/hisab.cyr`
  52,561 lines, 2,640,509 B; its sidecar names 7 leaves.
- Measurement provenance: **8** unmarked claim lines (detector false positives), the baseline; CI runs `--ratchet` on every push since 3.3.6.
- Toolchain **6.7.9**, sakshi **2.5.8**, ganita **1.2.15**.
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
3.4.0 bundle from pinned dirs:
- 6.6.2 refuses it with the known `#derive` error.
- Under 6.6.3, 6.6.6, 6.6.9, 6.6.10, 6.6.12, 6.6.13, 6.6.14, 6.6.20 and 6.7.9 a cross-module
  consumer program (a cross product, inverses of a scaled identity, a singular and two zero-sign
  matrices, `m3_normal_matrix_cof`, a non-unit rotation, `calc_adaptive_simpson_checked`,
  `num_bisection`, `f64_fmod(1, 0)`, `num_pollard_rho_checked`, `lyapunov_max`, `eigen_power`,
  `svd_compute_checked`, `cx_powf(2, Inf)`, `num_fft`) builds two ways, with the manifest's 16
  stdlib leaves and with the sidecar's 7 plus `syscalls`, `string` and `io`, with no warning, and
  runs. The output is bit-identical on every pin except `cx_powf(2, Inf)` on 6.6.3 / 6.6.6.
- Earlier runs found `cx_powf(·, ±Inf)` NaN on 6.6.3 / 6.6.6 (their ganita `pow`), and `ivl_sin`
  ([−1, 1]) and the perspective focal term (x87 `sin`/`cos`) different below 6.6.9.
- A 3.4.0 merge build was refused under 6.6.3 alone (`duplicate variable` in `_sym_rewrite_single`,
  repaired before release): the class *Decisions owed* #4 is about.
- ⚠ On 6.6.3 a private fn is still reachable through `&name`. That is upstream, fixed in 6.6.4;
  see *Decisions owed*.
- ⚠ **The sidecar names 7 leaves since 3.3.7** (`cyrius distlib` ≥ 6.6.18 writes a compile-verified
  `# Requires` block and `dist/hisab.deps`: math, alloc, vec, ganita, result, str, fmt; the same seven
  on the 3.4.0 bundle). A consumer that received `assert`, `bench`, `args`, `io`, `tagged`,
  `fnptr`, `callback`, `syscalls` or `string` through hisab's sidecar must declare it.

## How to read this file

- ⭐ **Every open item sits under a pinned version heading or in an unversioned section.** A pinned
  version looks like `**[3.4.1]**`. *Demand-gated (3.x.x)* and *Parked* are the unversioned
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
hisab's work; the [3.4.1] rows state their own verification. Premises still go stale: check the tree
before believing a row in either direction, including a row that says something is blocked.

---

## Pinned — 3.x patches (no public API change)

### **[3.4.1]** — the 3.4.0 findings

**Found during 3.4.0, not repaired (outside the rows 3.4.0 shipped).** Three sources, each verified
differently:
- **The audits**, `D102` (AU1, AU2) and `D103` (AU3), run on the 3.3.7 source under cycc 6.7.9
  (ganita 1.2.15). Their record holds every repro and oracle; the bullets cite its IDs. AU2's sweep
  sent its findings to independent skeptics: 95 verified (15 of them narrowed), none refuted, and
  `AU2-X1` checked by the lead only. AU1's discharge of the 2026-08-11 "did NOT reach" list re-ran 26
  of its 38 findings in a second tree; 8 were found and checked by the lane alone, and the four its
  sub-auditors ran once are marked *single run*. AU3 re-ran every 2026-08-11 repro. Their
  dispositions were read from the merged 3.4.0 source.
- **The 3.4.0 lanes** (AL allocation sites, LA linalg, NU1 calc/ode/optimize, NU2
  num/autodiff/symbolic, FB mat/quat/transforms, RT rays, CO collision): found by the lane and
  reproduced on its own trees against exact oracles. Not sent to a skeptic.
- **The two review rounds of each lane** (REV-*, then R2A–R2E): reproduced by the reviewer on the
  merged tree against its own oracle. Not re-run by a second party.

Duplicates are merged. For lane and review items, "code read" means the 3.4.0 source was read for
this list and still has the code; "not re-verified" means it was not. Their severities are assigned
here on the audits' ladder; their reporters gave none. The lanes rewrote many of these modules after
the audits ran: re-run each repro on the 3.4.0 tree before sizing its repair. Review and repair each
here.

*Costs to buy back* (same-boot A/Bs; each names its baseline, and most were not taken against 3.3.7)
- **`ray_sphere_near_surface` +4.8% over 3.3.7** (227.5 → 238.5 ns, RT's A/B on the 3.3.7 source
  under 6.7.9): the exact-c and grazing-hb gates. Moving them into helpers took the cost from +8 to
  +4.5 ns (focused); the 3.3.6/3.3.7 full-routine path (~170 ns over `ray_sphere`) is unchanged.
  [RT]
- **Collision.** Against 3.3.7, in CO's first A/B and not re-measured since: `gjk_intersect_box_hit`
  +8.9%, `gjk_intersect_tangent` +16.3%. Against earlier 3.4.0 trees only: `gjk_epa_face_1e9` +37%
  from CO's last round (23.4 → 31.9 µs; it read 18.9 µs on 3.3.7 in the first A/B),
  `gjk_intersect_small_edge_1e11` +38% (26.8 → 36.9 µs, a 3.4.0 row), the gate family's support
  calls +14%, cylinder-rim pairs that need the tilted ring 1,300–1,800 support calls, and the slowest
  query timed, `time_of_impact` at t = 0 on an in-band capsule/sphere pair at 1e3, 107 → 723 µs
  (6.8× the first 3.4.0 collision tree). Measure each against 3.3.7 in one run first. [CO]
- **`eigen_power`'s certificate on non-symmetric matrices**: where the first pass over A does not
  certify the answer, a non-symmetric A pays the Gelfand squarings, +147% / +250% / +528% at
  n = 30 / 60 / 100 on a strongly non-normal matrix (an exactly symmetric A takes two Cholesky
  factorizations instead: +4% to +56% at n = 100). The 3×3 and 12×12 bench rows are within spread.
  [LA]
- **`m4_inverse_checked` on view-projections past 2^10** takes the double-double tier: 563 ns
  against 3.3.7's 252 (2.2×), with 2-ulp answers where 3.3.7 was hundreds of ulp off. Exactly
  singular input is decided exactly on tier 3 (3.4 µs m4, 1.2 µs m3, where 3.3.7 returned the
  identity in 139 / 85 ns), and tier 3 on entries spanning 2^-1074..2^1023 takes 34.8 µs (m4) /
  15.3 µs (m3), which its comment does not say. `m4_transform_x64` read +6.4% in a full bench and
  −1% focused: one same-binary check owed. [FB, R2C]
- **Numerics.** `calc_adaptive_simpson_checked` on x⁴ at 1e-10: 7.1 → 8.6 µs (+21%) against the
  first 3.4.0 form, from the per-failing-panel tests; not measured against 3.3.7. `opt_lbfgs`'s
  gradients against 3.3.7: Wood 38 → 118 (f 95 → 149), extended Rosenbrock 88 → 101, Beale 14–15 →
  18–19; fewer on the set's other six problems (Rosenbrock 674 → 47). `simplex_fbm_2d_checked` +4.7%
  at 4 octaves (404 → 423 ns), put down to its seventh parameter without an ablation. [NU1, R2B]
- **Rows nobody owns**: `ivl_mul` +8.0% from cyrius 6.6.20 to 6.7.9 (212 → 229 ns, medians, ranges
  disjoint); `jet_plane` +5.2% from 3.3.6 to 3.3.7 (105 → 110 ns), not attributed because the two
  pins differ. Both under concurrent load. [AU2, AU1]

*Memory safety and process kills*
- **`cmat_trace` reads past a tall matrix** (code read: it guards only m == 0 and loops to
  `cmat_rows(m)`): `cmat_new(3, 1)` holding 1, 2, 3 → re 0x4018000000000000 (6.0; the extra 2.0 is
  the next heap object); 256×255, and 65536×1 at a heap-chunk edge, exit 139. A non-square trace is
  undefined; threat-model.md claims shape guards for it. [critical; AU2-G6-F1]
- **`num_crt` aborts when `count` exceeds the vecs** (code read: it tests only count ≤ 0):
  remainders [2, 3], moduli [3, 5], count 3 → exit 1 (count 2 → Ok, 8 mod 15); a null vec → exit
  139. Five siblings have the `n > vec_len` guard. [critical; AU2-G5-F2]
- **Null handles crash the spatial structures**: `kdtree_nearest(root, 0, q)`,
  `kdtree_within_radius(root, 0, q, 1)`, `quadtree_insert` / `_query(0, …)`, `octree_insert` /
  `_query(0, …)`, `spatial_hash_insert` / `_query_radius` / `_query_cell` / `_clear(0)` and a null
  query point: 11 calls, each exit 139 (code read: `quadtree_insert` and `spatial_hash_clear` load
  through the handle first). No precondition is stated; `kdtree_build(0)` and the halfedge, Delaunay
  and diffgeo entries refuse null. 3.4.0 checked these functions' own allocations, not their
  arguments. [critical, a caller error; AU2-G5-F3]
- **`sym_rewrite`'s `max_iter = 50` bounds passes, not work** (code read): rules
  {sin(a) → sin(sin(a))} on sin(x) double the depth each pass (4096 after 12), and the recursion
  overflows the stack, exit 139. A linear-growth rule stops after 50 passes. [critical by the
  ladder, high in practice; AU2-G6-F14]
- **Every recursive symbolic walker overflows the stack on a deep tree**: a left-folded sum of
  100,000 terms through `expr_eval` under `ulimit -s 8192` exits 139 (sub-auditor depths: eval
  ≈ 65.5k, simplify ≈ 40k, diff / LaTeX / integrate ≈ 47.5k, rewrite ≈ 87k). `kdtree_build` had the
  class until 2.6.14. [medium; SYM-F3]
- **A 0 node crashes `expr_to_str`, `sym_to_latex` and `sym_integrate`** (code read: `expr_tag`
  loads through it, and neither of the first two tests it). 3.4.0 made `expr_eval_checked`,
  `expr_simplify` and `expr_diff` refuse 0, and a refused allocation now yields a 0 node, so a
  failure value reaches them; `sym_integrate` and `sym_match_bindings` document 0 as theirs. Two
  hand-built shapes the constructors no longer make also crash: `expr_diff` on an `EXPR_POW` whose
  exponent child is 0, and `expr_eval_checked` with a binding from `expr_bind(0, v)` (in `str_eq`;
  not re-verified). [low; SYM-F8 (partly repaired), NU2 F4, REV-NU2 N1]
- **`expr_to_str` uses quadratic memory**: a 20,000-node ADD spine peaked at 2.3 GB RSS;
  `sym_to_latex` on the same tree, 13 MB. [low–medium; SYM-F4, single run: verify first]
- **The stdlib's abort-on-refusal reaches through hisab.** `vec_push` and `str_builder_add*` print
  to fd 2 and exit on a refused grow, where `vec_push_a` / `str_builder_*_a` return −1 (a 3.4.0
  grep, comments included: 83 `vec_push(` sites in 7 modules, 71 `str_builder_add` in symbolic and
  symbolic_ext); `vec_get` exits on a bad index (174 sites; 177 at 3.3.7). AL's first census also
  found 285 of 992 in-tree calls reading a hisab-allocated temporary without a check (a lower bound,
  one level deep), so a composite function can still fault one frame above a constructor that now
  returns 0; AL's 83-case exhaustion sweep is clean on what it covers. Propagate or abort is a
  design decision. [census; NU2 F3, AU2, AL F4; the grep counts code read, the census not
  re-measured]

*Wrong answers on ordinary input*
- **`so3_log` returns the wrong axis for every rotation near a half turn** (code read: the
  eigenvector branch runs only when the antisymmetric part is exactly 0; otherwise ω =
  θ·asym/|asym|, whose direction is rounding noise ∝ ε/(π − θ)). n = (3FD3C6EF372FE950,
  3FE6A09E667F3BCD, BFE1E3779B97F4A8): `so3_log(so3_from_axis_angle(n, F64_PI))` → (0,
  40067ABA6D20E485, BFF67ABA6D20E485); exact ±(3FF04D072724094F, 4002A674D8FAB223,
  BFFD7D14162AF485) = π·n̂; ‖exp(log R) − R‖_F = 0.33. 100 of 100 random axes at π are off by more
  than 1e-3, 79 by more than 10%. The only test uses the z axis; `su2_log` is exact on the same
  inputs. [critical; AU2-G7-F1 = LIE-F1]
- **`cubic_bezier_ease` returns an unconverged point on a CSS-legal curve** (code read: Newton from
  t = x, unbracketed, an 8-iteration cap and a 1e-12 slope stop, neither reported).
  cubic-bezier(1, 0, 0, 1): x = 3FE028F5C28F5C29 (0.505) → 3FBAD7D99EF4BF80 (0.105), exact
  3FE51734657C739D (0.659); 24 of 201 grid points are off by up to 0.554, and the output is not
  monotone. Standard CSS curves are within 1.5e-12; the only test is x = 0. [high; AU2-G4-F1]
- **`eigen_symmetric` is accurate only to 1e-12·max|a|, with Ok** (code read: it answers through
  `ganita_mat_eigen_sym`, whose Jacobi sweeps stop at max_off ≤ 1e-12·max|a_ij|): [[1, e], [e, 1]],
  e = 3D3C25C268497682 (1e-13) → λ = (3FF0000000000000 ×2) and identity vectors; exact 1 ± e =
  3FF00000000001C2 / 3FEFFFFFFFFFFC7B (450 ulp) and (1, ±1)/√2. `eigen_qr` gets it bit-exact.
  Document the limit or route to `eigen_qr`; an upstream report is *Decisions owed* #11. [high;
  AU2-G3-F3]
- **`hquat_slerp` switches to normalised lerp above d = 0.9995**: 3.7e-7 off (1.7e9 ulp) just above
  the switch against a 60-digit slerp, ≤ 0.6 ulp just below; `t3d_lerp` inherits. The switch is
  stated in-body, its cost nowhere; θ from an atan2 form could move it to ~1 − 2^-52. Also
  `hquat_slerp(0, 0, 0.5)` returns (0, 0, 0, 0) from its acos branch where the lerp branch gives NaN
  for a zero interpolant (REV-FB N4; FB left it unchanged). [medium; AU2-G7-F2]
- **`world_to_screen_checked`'s depth assumes [−1, 1] NDC, while the library's own
  `m4_perspective_reverse_z` produces [0, 1]**: a point 5 away reads back as 0.196 through
  `linearize_depth_reverse_z` (exact 5); 50 random reverse-Z cameras are up to 98.4% off, GL ones
  within 1.9e-14. No NDC convention is stated; a depth-range parameter would be a [3.5.0] change.
  [medium; F-X1]
- **`matrix_condition_number` calls an invertible matrix singular once cond ≳ 1e12** (ganita LU's
  pivot test): diag(3D3C25C268497682, 1) → −1, exact 1e13; Hilbert 11 and 12 → −1 (exact 5.33e14,
  1.72e16). It is +Inf for a well-conditioned matrix scaled below DBL_MIN (exact 2.26), and reads a
  non-square A as −1, which its doc does not say. [medium, the doc low; AU2-G3-F4, F5, F15]
- **`cmat_inverse` refuses exactly invertible matrices**: its relative 1e-12 pivot test refuses
  diag(1e-13, 1), and its absolute DBL_MIN floor refuses [000C000000000000] (exact
  7FD5555555555555). It answers complex Hilbert 11 and 12, which `ganita_mat_inv` refuses. [medium,
  the floor low; AU2-G3-F7, F6]
- **`expr_diff`'s power rule fails for an exponent with no x that is not a CONST node**: it takes
  f^g·(g′·ln f + g·f′/f), and ln f is NaN or −Inf for f ≤ 0. x^y at x = BFFBB303EC100C7C, y = 3 →
  NaN, exact 3x² = 4021FB800DAD62C2; at x = 0, NaN either way (exact 0); 168 of 22,526 random trees.
  [3.7.0]'s Newton on `expr_simplify(expr_diff(e))` meets it. [medium; SYM-F1]
- **`convex_hull_2d` / `triangulate_polygon` decide orientation with a rounded determinant, and
  accept non-finite sites**: P0 = (3FE0000000000077, 3FE0000000000096), P1 = (12, 12), P2 =
  (24, 24) → hull [0 2], exact orientation +93/2^51 and hull [0 1 2], which `delaunay_2d` returns; a
  square plus (NaN, 2) → an 8-index hull with repeats; a pentagon with (+Inf, 2) → a "complete"
  triangulation. `delaunay_2d` has exact predicates (3.3.5) and rejects non-finite sites (3.3.3).
  [medium; AU2-G5-F4, F5]
- **`lorentz_is_valid` accepts NaN, ±Inf, entries ≥ 2^512 and O(1) defects at high rapidity** (code
  read: a NaN residual fails `f64_gt(·, tol)`, and tol = 1e-12·(1 + scale²) is +Inf from 2^512):
  the identity with (0,0) = NaN, +Inf or 5FF199999999999A → 1; diag(1e300, 1, 1, 1) → 1;
  `lorentz_boost_x(800)`, every entry +Inf, → 1; `lorentz_boost_x(20)` with Λ²₂ = 100 → 1 (exact
  (ΛᵀηΛ)₂₂ = −10⁴). The comment's "a non-Lorentzian defect is O(scale²)" is false. [medium;
  AU2-G7-F4 = LIE-F2, LIE-F3, REV-AL N7]
- **`halfedge_is_boundary` gives up after 1000 steps and answers Ok(0), "interior"** (code read:
  `guard < 1000`): an open fan around vertex 0, triangles (0,2,3), (0,1,2), then (0,k,k+1) for
  k = 3..N → Ok(1) for N ≤ 1001 and Ok(0) from N = 1002, though half-edge 0→1 has no twin.
  `vec_len(half_edges)` bounds a manifold walk. [medium; AU2-G5-F7]
- **`sdf_capsule` is NaN for a zero-length capsule and squares |ab| naively**: p = (3, 4, 0),
  a = b = 0, r = 1 → NaN, exact 4 (`sdf_sphere` and b = (1e-170, 0, 0) both give it); a scene scaled
  by 2^512 is 24% high, NaN from 2^513 up and from 2^-538 down. A test pins the NaN (*Tests that pin
  a defect*). [medium; AU3-F23, AU2-G1-F4]
- **`sdf_smooth_union` at k = 0**: (1, 1, 0) → NaN, (1, 2, 0) → min; k = −0 returns the maximum in
  either order. The limit k → 0+ is min. A test pins the NaN. [medium; AU2-G1-F8]
- **One NaN point makes finite points unreachable in the kd-tree** (its splitting plane becomes
  NaN): eight points, one of them (NaN, 0, 0) → `kdtree_nearest((8, 2, 0))` = 3 where brute force
  gives 0 at distance 0. The quadtree and octree reject NaN since 3.3.4. [medium; SPC-F1]
- **`dual_pow` / `ad_pow` with n = 0 give a NaN derivative at a zero or tiny base** (code read:
  n·base^(n−1) = 0·Inf): `dual_pow(dual_const(0), 0)` → derivative FFF8000000000000; x⁰ ≡ 1, so
  the derivative is 0. [medium; AU2-G6-F8 = NU2 F2]
- **`su2_log(−1)` returns the zero vector**, whose `su2_exp` is +1; a vector of length 2π is right
  (the neighbour w = −1, x = 2^-1054 gives 2π·x̂). `se3_log`'s use is harmless. [medium;
  AU2-G7-F3 = LIE-F7]
- **`se3_log(se3_from_mat4(T))` is not the principal log for rotations of 120° or more** (a turn
  of 2π − θ about −ω̂, disagreeing with `so3_log`). [low; LIE-F8, single run: verify first]
- **`calc_hermite_tcb_checked` does not validate key order** (code read: it tests n < 1 and the
  end clamps only): times [0, 2, 1], values [0, 5, 10], t = 1.5 → Ok(10.0) from the end clamp,
  though t lies between keys 0 and 1. `calc_monotone_cubic_checked` refuses such a grid. [low; NU1]
- **Out-parameter aliases**: `calc_hessian(f, x, n, h, out = x)` corrupts the off-diagonals (f =
  x0²·x1 + x1³ + x0·x2 at (2, 3, 5): H[0][1] = 12, exact 4), the H[0][0] write landing in x[0]
  first; `se3_new` stores the caller's pointers, not copies. Neither is documented. [low; F-A2]
- **`spatial_hash_query_radius` misses a point 1.67e-16 inside the radius**: cell 0.1, point x =
  401ACCCCCCCCCCCC, centre 4019999999999999, r = 3FD3333333333333 → no entries. The cell index is
  floor(v·fl(1/s)), not floor(v/s) as the comment says. [low; SPC-F9]
- **`geo_closest_point_on_aabb` reads an inverted (setter-built) box unsorted** (min (2, 0, 0),
  max (1, 1, 1) → x = 2); `geo_aabb_aabb`, `geo_ray_aabb_face` and `geo_jet_aabb` sort it since
  3.3.4. [low; AU2-G2-F12]
- **`eigen_qr(diag(2, 1), 2, max_iter 0)` → Err(−8)** for a matrix that needs no sweep;
  **`wedge_1_1(α, β, 1)` returns null** for a valid input (Λ²(R¹) has no components: an empty
  array, not a failure). [low; AU2-G3-F14, AU2-G6-F11]

*Fabricated answers* — a legal-looking value with a success status, in functions the [3.4.0] rows
did not name; each can answer or refuse through the return it already has:
- **`sdf_box` returns a finite distance for a NaN point, centre or extent** (stdlib `f64_max` drops
  NaN): point (NaN, 0, 0) → BFE5555555555555 (−0.667, "inside"); `sdf_sphere` and `sdf_capsule`
  return NaN. **The SDF booleans drop a NaN first operand**: `sdf_union(NaN, 1)` = 1, `(1, NaN)` =
  NaN, `sdf_intersection` / `sdf_subtraction` likewise, which launders `sdf_capsule(a == b)`'s NaN
  by operand order. [medium; AU2-G1-F5, F6]
- **`ode_dopri45_trajectory` returns Ok for a run that did not integrate**: tol < 0, NaN or −Inf
  accepts every step (y′ = −10y, tol −1e-8 → y(1) = 4030A0FA309355A0, 16.63; exact e^-10); a NaN
  error estimate is accepted (y′ = −√y, dt0 1.9 → a NaN trajectory); tf < t0 → Ok with one point,
  pinned by a test. `calc_adaptive_simpson_checked` refuses tol ≤ 0 or NaN up front. [medium;
  AU2-G4-F3, F4, F5]
- **`killing_residual` drops a NaN residual and reports 0.0, "exactly Killing"**: dim 2, ∂ξ =
  [[0, NaN], [3, 0]] → 0 ([[0, 1], [3, 0]] → 4, right). [medium; AU2-G6-F6]
- **`cga_project` / `cga_reject` turn `cga_blade_inverse`'s "null" refusal into an answer**: x =
  `cga_point(1, 2, 3)`, b a unit sphere at 19998112 (`cga_norm_sq(b)` = 1 exactly) → project = the
  zero multivector, reject = x; exact e1 = C46B18EDA5209F17 (Fraction, and the guard-free
  (x⌋b)⌋b). The inverse's resolution limit is documented (`D052`); its silent propagation is not.
  [medium; AU2-G1-F1]
- **`unpremultiply_alpha` zeroes a subnormal or NaN alpha** (below 2^-1024): (1, 0, 0, 1)·2^-1074
  → (0, 0, 0, 0), exact (3FF0000000000000, 0, 0, 0000000000000001); a NaN alpha → (0, 0, 0, 0). The
  ⛔ comment describes the 1e-12 version; the repair only moved the threshold. **`composite_plus`
  maps NaN to 1** (stdlib `f64_min(NaN, 1)` returns 1). [medium; AU2-G7-F5 = SPC-F3, AU2-G7-F7 =
  SPC-F4]
- **`sequential_impulse` answers 0 with Ok when the inverse masses sum below DBL_MIN**: pen
  0000000300000000 with inverse masses 0000000400000000 ×2, 4 iterations → normal impulse 0, exact
  λ = pen/(ima + imb) = 3/8, which the ×2^1000 twin answers; pen 2^-100 with inverse masses
  2^-1027 → 0, exact 2^926. [medium; AU2-G5-F10]
- **`num_fft` / `num_ifft` return Ok for a negative n** (code read: `n <= 1 → Ok(0)` runs before
  the power-of-two test), while `num_dst` / `_idst` / `_dct` / `_idct(buf, −4)` and the 2-D FFTs
  answer Err(−10), and both doc blocks promise INVALID_INPUT for a non-power of two. n = 0 and n = 1
  as no-ops are defensible; a negative length is not. Pinned since 2.9.0. [low; AU3-F25 =
  AU2-G4-F13]
- **`num_factorize(n ≤ 0)` returns the empty vec**, which is 1's factorisation (code read: `n <= 1`
  returns it; the precondition says n > 0): `num_factorize(0)` and `(−12)` → length 0. [low; NU2
  F1]
- **`levi_civita_3` / `minkowski` return ±1.0 out of range** (`levi_civita_3(0, 1, 3)` = −1.0),
  pinned by a test comment only; `levi_civita_4` is a consistent extension. [low; AU2-G3-F12]
- **Spatial extents answer from invalid input**: `spatial_hash_new(−0.0)` or `(−1)` is accepted,
  and every radius query then returns nothing while `spatial_hash_query_cell` finds the point; a
  negative radius is |r| in `kdtree_within_radius` and empty in `spatial_hash_query_radius`; the
  quadtree does not normalise swapped corners (insert → Err(−3), a swapped query → 0 points), the
  octree does. [low; AU2-G5-F11, F13, F14 = SPC-F11]
- **`sym_rewrite` silently skips a rule whose template names an unbound wildcard** (`pat_var("x") →
  pat_wildcard("b")` on x + x leaves it unchanged), **and one non-finite constant blocks its
  fixpoint**: `sym_const_eq(inf, inf)` = 0, so the first rule reads as a change for all 50 passes
  and `(x + neg(x)) + const(+Inf)` never becomes `0 + inf`. [low; AU2-G6-F15, SYM-F5]
- **`screen_to_world_ray` returns a zero direction with rc 0 when near = far** (inv_vp = I with
  (2,2) = 0: origin (−0.75, 0.833, 0), direction (0, 0, 0)); no ray exists. It returns an integer
  code, so the refusal needs no new channel; its w = 0 case is a [3.5.0] row. [low; AU2-G7-F8]
- **`hquat_from_euler` treats an order outside `EulerOrder` (7, −1) as ZYX**, bit for bit. [low;
  AU2-G7-F9]
- **A zero normal or direction builds a plane or line that answers every query**:
  `geo_plane_from_point_normal(p, 0)` gives distance 0 for every point, `geo_line_new(o, 0)` returns
  o for every query; `geo_jet_dpoint` on a miss jet returns (0, 0, 0), which its doc does not
  mention. 3.4.0 refuses a null handle, not a zero vector. [low; AU2-G2-F15]

*Range and accuracy: rays, geometry and collision*
- **`geo_ray_triangle` is right only for scenes 2^-341..2^341** (a degree-3 numerator, never
  pre-scaled as its 3.3.4 siblings were; 3.4.0 kept its bits): a scene at 2^-359 misses, 2^-358 is
  6.3% high, 2^342 gives +Inf and 2^512 NaN; 4,213 of 6,276 scaled rows wrong (exact rational
  Möller–Trumbore). A triangle with legs of 2^-513 hit squarely is a miss: v0 = (9FE0000000000000,
  9FE0000000000000, 0), v1 = (1FE0000000000000, 9FE0000000000000, 0), v2 = (0, 1FE0000000000000,
  0), o = (1FC0000000000000, 1FC8000000000000, C014000000000000), d = (0, 0, 1) → 0, exact 5.
  `geo_jet_triangle` inherits. [medium; AU2-G2-F1, AL F1]
- **`geo_jet_capsule` answers a miss jet for barrel hits once |ab|² under- or overflows**: hit jets
  only for scenes 2^-536..2^512 (a unit scene × 2^-550: primal t = the Decimal barrel root, jet a
  miss); the skeptic also found wrong hit jets, dt/do off by 1e-11 to 3.3×, at 2^-538..2^-518 and
  at 2^512. [medium; AU2-G2-F8]
- **`geo_ray_capsule_branch` rejects a cap hit when d·(end − start) underflows**: 3.3.6 added the
  overflow half only; it needs a setter-built |d| ≲ 2^-590 (exact start-cap t 49FFBC7CA92F338B,
  answer 0 / branch −1). [medium; AU2-G2-F14]
- **Naive squares at the ends of the range**: `geo_segment_closest_point` / `_distance_to_point`
  (a segment end queried at 2^-540 → the start, distance ≠ 0; a midpoint at 2^600 → NaN; 700 of
  1,628 wrong, all at ≤ 2^-519 or ≥ 2^511), `geo_sphere_sphere` (separated spheres read as touching
  below 2^-538 and above 2^512; 427 of 1,746, every one a false contact), `inertia_sphere` /
  `inertia_box` (+Inf for a representable 6DC70FB664EDE14B; a subnormal r² 6.2% low), and the
  kd-tree's squared distances (two points near 2^512 and query 0 → −1, "empty"; 133 of 200 nearest
  answers wrong at 2^-545; `kdtree_within_radius` includes a point 3r away past 2^516 and below
  2^-536). [medium; AU2-G2-F2, F3, AU2-G3-F9, AU2-G5-F6 = SPC-F2]
- **`geo_ray_plane`'s |n·d| < DBL_MIN guard refuses finite hits**: plane z = 0, origin (0, 0,
  8173C0CA428C59FB), d = (1, 0, 00001E06522C8B91) → 0, exact 41C50D793FFAFE38 (7.06e8).
  `geo_jet_plane` inherits. [medium; AU2-G2-F4]
- **`geo_triangle_unit_normal`'s pre-scale flushes small components**: v1 = (4C73C0CA428C59FB, 0,
  0), v2 = (0, 017E06522C8B909A, 0176A09E667F3BCD) → the fallback (0, 1, 0); exact (0,
  BFE3425F773FB196, 3FE98E315CDBC591), which `hvec3_normalize(geo_triangle_normal)` returns.
  [medium; AU2-G2-F5]
- **Sphere and capsule jets lose gradient accuracy on subnormal scenes**: t exact, dt/do 2.9e-3
  relative off; 7 of 1,857 jets beyond 1e-12, all at scenes ≤ 2^-1030. [medium; AU2-G2-F9]
- **`geo_aabb_center` / `geo_segment_midpoint` / `geo_triangle_centroid` overflow in the top
  binade**: min 7FE8000000000000 ×3, max 7FEC000000000000 ×3 → +Inf ×3, exact 7FEA000000000000 ×3.
  [medium; AU2-G2-F7]
- **`time_of_impact` misses a collision when |vel_b − vel_a| overflows from finite velocities**
  (`_toi_core` still forms rel_vel / speed: u0 = v/Inf = 0, then Inf/Inf): spheres r = 1 at 0 and
  (10, 10, 0), vel_b = (FFE72409614C1E6A ×2, 0) → a miss, exact t = 0027BEDC8CC94029. [medium;
  AU2-G1-F3]
- **`cga_norm`'s range gate rescales by the largest coefficient, including null-basis blades that
  form no term**: `cga_translator(2^539, 0, 0)` → 0 while `cga_norm_sq` = 1; a motor at t =
  1.7346·2^524 1.9e-9 low (3.1% at 2^536); 1·e1n0 + 2^-600·e23 → 0 (exact 2^-600); an infinite
  n0 / ninf coefficient gives +Inf where every finite multiple gives 0. The test's "inherent" is
  false since 2.21.0's null basis. [medium, the +Inf exit low; AU2-G1-F2, F7]
- **Jets and tags at non-finite results**: `geo_jet_plane` returns hit jets with ±Inf partials
  (dt/dd_z, dt/dn_x = +Inf for a true 2^1200) where the sphere, capsule and triangle jets answer
  their miss jet; `geo_ray_capsule_branch` tags a NaN t from a NaN ray component or radius with a
  capsule branch (END_CAP at 3.3.7, CYLINDER at 3.4.0) where the FACE ENCODING header says
  GEO_FACE_NONE; r = −0 answers as +0. [low; AU2-G2-F10, F11, R2E n6]
- **Rays RT's rounds left** (measured by lane RT's final round against base 3.3.7 and an
  intermediate 3.4.0 tree, exact Fraction oracle; not re-verified by a third review):
  - thin capsules answered through `_geo_capsule_far` can get the wrong hit/miss verdict, not only
    the wrong label (60 of 69 wrong answers in one adversarial "aimed" family). A repair was built
    and works (78 → 28 wrong, no new right → wrong) but costs a median 36 µs per ray, up to ~900×
    3.3.7's path; withdrawn. Needs a fixed-size certified verdict. [medium; REV-RT S-c, R2E S-c]
  - the exact thin-barrel solve, extended to 2^1000 radii in 3.4.0, takes > 10× 3.3.7's time on
    the thin-barrel families (aimed / span / T1: 384 / 696 / 1,081 of 2,000 rays). [medium; cost]
  - right → wrong against 3.3.7 that remain, all narrow: a needle ray met along the axis (1);
    labels at a seam with g = 0, decided by a float side test (4 graze / 3 cross; the exact side
    test measured 17–48× slower); origins a few 2^-1074 units off the axis at gx 1070 (6); label
    ties with t exact on span rays (1 each). [low; RT final round]
  - `geo_barycentric_coords`' exact two-word thin and colinear paths cost `barycentric_thin`
    +350% (2.6 → 11.9 µs) and `barycentric_colinear` +196% against the first 3.4.0 form; ordinary
    triangles are unchanged. [cost]
  - far-path capsule labels at a seam (`_geo_capsule_far`): the copy carries ~2^-106·|o − start|,
    so a grazing ray within that of a seam can take either label while t is right; 1 right → wrong
    in 79,000 sweep rays. Run `_geo_barrel_seam_x` on the original inputs when the copy's answer is
    a barrel or cap root. [low; RT review round, finding 1]
  - the near-tangent barrel class (rays tangent to within 2^-1..2^-50 of r, 1–20 radii out): 1,149
    of 4,000 wrong on 3.3.7 and on RT's first tree; ordinary rays with c partly cancelled, 4–6 ulp;
    capsule rays parallel to the axis up to rounding near the barrel, 36–45 of ~170 wrong, up to
    4e7 ulp; spheres with origins 2^30 radii away and near-tangent rays, 216 of 3,000 wrong.
    `_geo_barrel_xact` answers such barrels exactly at ~26 µs. [medium; RT F1, F2, REV-RT n5]
  - a needle capsule (|u|/r ~ 2^200) met along its axis enters within 2^-200 of the end cap's
    plane, the float side test drops it and the far cap's exit comes back (every tree); exactly
    tangent touches at a seam circle read the miss (2 rays). [low; R2E n4, n5]
  - barycentric weights above the 2^-16 thin switch carry the Gram form's conditioning: triangles
    2^-6 from colinear, 58 of 300 within 64 ulps (worst residual 8.9e-12). [low; RT F4]
- **Operand order still decides some in-band collision pairs**: the support contract states one
  verdict per *ordered* pair. Pairs unanimous in each order that flip when A and B swap, all inside
  the 4-eps band: 443 / 338 / 410 / 670 on the sw11 / sw12 / big31 / big32 sweeps (3.3.7: 273 /
  260 / 301 / 443). A canonical order removed every flip but put the slower order's cost on both,
  up to 15.3× the first 3.4.0 collision tree. [low, inside the contract; CO's last round, R2D R4]
- **Sphere-on-cylinder depths up to 6,725 eps** (4.3e-5 relative; the normal tilted 2.2e-10 along
  the crease) from `gjk_epa_3d` / `mpr_penetration` on a z-cylinder's side and cap: the snap's
  closed forms cover polytope and segment cores only. The first 3.4.0 collision tree reached 9,900
  eps. [medium; R2D N3; CO's last round left the cylinder families' answers unchanged]
- **Deep tiny spheres at box corners and edges** (depth ~ r, centre inside the box): up to 1.7%
  (1.1e6 eps) off, the same worst case on every tree; `mpr_penetration` moved one pair of 2,160
  0.97% further off when it took `gjk_epa_3d`'s path. [medium; R2D N4; not re-verified after CO's
  last round]
- **`time_of_impact`'s corner normal at r = 1e-6**: one corner sweep keeps 4.3e-8 rad (the other 39
  a median of 1.4e-10); the radius table in `time_of_impact`'s doc states it. The CO2 M2 pair's
  depth costs 580 support calls (3.3.7: 276): EPA's GJK seed is a sliver and the polish runs. [low;
  CO2 F3, F4; not re-verified after CO's last round]

*Range and accuracy: linear algebra and numerics*
- **`hisab_lerp(−DBL_MAX, DBL_MAX, 0.5)` = +Inf** (code read: a + (b − a)·t; exact 0); `hvec2_lerp`
  / `hvec3_lerp` the same, and `t3d_lerp` inherits. 3.4.0 repaired the inverse
  (`hisab_inverse_lerp_checked`); the exact value is finite, so no channel is needed. [medium;
  AU2-G7-F15 (the rest), FB F2]
- **`eigen_power` answers above ρ on an irreducible 2×2 whose eigenvector sits on the subnormal
  grid** (|a01| near DBL_MAX, a10 a few subnormal units): [[3F4FFFFFFF800000, 7FE0000000000000],
  [0000000000000001, 3F40000000000000]] → Ok(3F50000000200000), exact 2^-10 (4.7e-10 off);
  [[3EA504B73F812A3C, 7FE28392F4312526], [13·2^-1074, 3EA1BA1B90F3D9A9]] → Ok(3EA69AF208C555CB),
  exact 3EA69232FBAA0889 (1.5e-3 off). 19 of 60 such matrices are Ok more than 1e-9 off, 17 of them
  above ρ. The Gelfand certificate cannot see an answer above ρ. [medium; LA review R1; stands after
  the certificate, per the lane]
- **`eigen_power` refuses some computable answers**: a matrix whose strongly connected components
  all have ≤ 2 vertices has a known spectrum, but one the iteration misses is NO_CONVERGENCE
  (3.3.7's [[1e-3, 0, −1e20], [0, 1.3, 0], [0, 0, −1]] plus a 2-cycle vertex: 1.3); the certificate
  reads A through a frame that flushes entries below 2^-1074 of max|A| (no wrong certification
  found). [low; LA F2, R2A n5; not re-verified]
- **The Krylov solvers' orthogonal-t bound's n factor refuses 10 of 40,000 injected-t runs** that
  converge without it. [low; LA review R3; not re-verified]
- **`lyapunov_max` where f64 cannot follow the method**: Q cannot hold the direction of
  [[1.7·2^-60, 0], [1.7e308, 1.3·2^-80]] (k = 20: −16.94, exact finite-k −3.519); 3 of 12 dense
  10^±150 matrices are up to 9× off; a dense exactly nilpotent A gives a finite rounding-level
  value, not −Inf. Each is stated in its doc. [low; R2A n3, n4, LA]
- **`solve_pgs_checked` floors its tolerance at DBL_MIN**: [[4,1],[1,3]], b = (1, 2)·2^-1030 →
  (0000040000000000, 0000095555555555), exact (000001745D1745D1, 00000A2E8BA2E8BA), 175% off after
  one sweep. [medium; AU2-G3-F8]
- **`calc_adaptive_simpson_checked` where the noise floor cannot see the noise**: noise in
  cancelling terms ends at the 2^25-evaluation backstop (5x⁴ − 3x² + x − 1/4 at 1e-20: 33,556,523
  evaluations, 1.7 s, Err with 1/4 exactly), as does argument-rounding noise (A·sin(84.5x) + ε at
  1e-14: 34,055,987, 3.5 s); 266 of the reviewer's 6,000 calls pass 4,000,000 evaluations (3.3.7:
  383). Ok rests on Richardson estimates, not a bound; the doc lists the measured exceptions. An
  integrand infinite at an endpoint (1/√x on [0, 1]) is NO_CONVERGENCE where an open rule would give
  2, and a sample that overflows (x·2^1003 on [−(2^21 − 2^-32), 2^21], integral ≈ 2^992) is Err
  with NaN. The backstop's size is *Decisions owed* #14. [medium; NU1, R2B, REV-NU1]
- **`opt_bfgs` keeps `_opt_armijo`**, whose 40-halving fallback takes an untested α = 2^-40 that
  can step uphill (CG did at curvature 2^40: f 8.2e13 → 2.9e16, NaN by iteration 10, before 3.4.0
  moved CG and L-BFGS to a strong-Wolfe search); scale refusals follow, and BFGS returns
  NO_CONVERGENCE 0.7 off on 1 + 2^-80|x − 0.3|², which CG and L-BFGS now solve. [medium;
  AU2-G4-F7 (partly repaired), REV-NU1 n6]
- **`opt_levenberg_marquardt` refuses residuals scaled ≤ 2^-34 or ≥ 2^510** (an absolute λ0 = 1e-3
  and a strict-decrease accept): r_i = c·(p0 + p1·i − (1.3 + 2.7·i)), c = 2^-40 → Err, x = (0, 0);
  closed form (1.3, 2.7) at every c. Right at 271 of 1,046 scales, never Ok and wrong. [medium;
  AU2-G4-F6]
- **`num_newton` refuses a subnormal derivative**: f = c(x − 1.5), c = 0008000000000000, x0 = 1 →
  Err(−4); one step gives exactly 1.5. Its comment still reads as the old 1e-12 defect. [medium;
  AU2-G4-F9]
- **`calc_bspline` / `calc_nurbs` treat a subnormal knot width or weight as zero**: knots ×2^-1064
  → the last control point, exact (3FFC5FFDD5155555, 3FFE6591AAAAAAAB, 3FD94584A92AAAAB); weights
  ×2^-1023 → 0. 3.4.0 repaired `calc_hermite_tcb_checked`'s instance. [medium; AU2-G4-F10 (partly
  repaired)]
- **`calc_gradient` / `calc_jacobian` / `calc_hessian` overflow 1/(2h) and 1/(4h²), with Ok**:
  x + y at 0, h = 2^-1074 → (+Inf, +Inf), exact (1, 1), which `calc_partial_derivative_checked`
  gives; the Hessian's off-diagonal is +Inf for h ≤ 2^-513. They, `calc_partial_derivative_checked`
  and `calc_monotone_cubic_checked` also take scratch from the bump heap on every call (n·8 to
  n·16 bytes), so a hot loop grows the heap without bound. [medium, the heap use low; AU2-G4-F11,
  NU1]
- **The ODE `_checked` Newton**: cos t − exp(k·y) from rest is NO_CONVERGENCE at k = 2.5e13 (the
  first secant is 5e11× too steep); the singular margin refuses 34 Prothero–Robinson steps with
  |1 − cλ| in (2^-26, 2^-23) that Newton could solve (3.3.7 solved none); two Van der Pol (μ = 1000)
  BDF-2 steps cycle where 3.3.7 converged by luck. [low; NU1, R2B n4, n5; not re-verified]
- **`num_bisection` keeps evaluating after the bracket closes**: 147 wasted calls in the stall row,
  about 950 at max_iter 1000; an in-loop exit would return the same value. [low, an optimisation;
  REV-NU2 N2]
- **Forward and reverse automatic differentiation disagree in 124 of 1,440 special-value cases**
  (R2A's sweep; 232 before 3.4.0's last round, none of the 124 a domain miss or a NaN value): 74 are
  a NaN tangent through a zero partial (`ad_grad` skips a +0 adjoint, code read: `if (a != 0)`, so
  it never forms 0·NaN), 39 an infinite value times a zero tangent or Inf/Inf in a forward formula
  (Inf·2: forward NaN, the tape 2), 11 the sign of a zero (adjoints accumulate from +0). The skip
  was kept by decision; the threat model states the three classes. [low–medium; R2A SF2, NU2]
- **`dual_div`'s derivative and `ad_div`'s ∂/∂b go through b²**: `dual_div(var b, const b)` →
  derivative 0 at b = 1e200 (exact 1/b), +Inf at 1.785e-200, 2.4e9 ulp off at 1EF6E0C2B9F0C1D5;
  `ad_div`'s value is a·(1/b), two roundings (1 ulp off the correctly rounded `dual_div`). [medium,
  the value low; AU2-G6-F7, F13]
- **`cx_div` / `cx_inv` return 0 when Smith's denominator overflows near DBL_MAX**:
  `cx_div(2^1023 + 0i, 7FE8F0A3D70A3D71 + 7FE724C3FA1D5B7Ai)` → (0, −0), exact 0.3447 − 0.3199i;
  `cx_inv(DBL_MAX(1 + i))` → (0, −0), exact (0002000000000000, 8002000000000000); scaled by 2^-100,
  exact. The in-body "the only input that still fails is b = 0 exactly" is false. Dividing by
  +Inf + Inf·i gives NaN where C99 G.5.1 gives 0. `cqr_decompose` calls `cx_div`. [medium, the
  infinite divisor low; AU2-X1 (lead only), AU2-G6-F16]
- **`sectional_curvature` answers 0 ("flat") for planes 1e-6 rad apart** (sin²θ < 1e-12; exact 1)
  **and 0 or NaN past 2^±256** (an absolute DBL_MIN backstop on a degree-4 quantity): u = s·e1,
  v = s·e2 on the unit-curvature form → 0 at s = 2^-256, NaN at 2^256, exact 1; a metric
  1A74CCCCCCCCCCCD·I → 0. The epsilon census's #15 reads REPAIRED. [medium; AU2-G6-F5a, F5b =
  AU3-R2]
- **The unsimplified derivative gives NaN from 0·Inf**: d/dx(x^const 0) at 0 and d/dx(2·exp(x)) at
  710 (simplified: 0 and +Inf). [low; SYM-F6]

*Range and accuracy: Lie groups, matrices, color and noise*
- **`se3_exp` at the ends of the |ω| range**: ω = (2^-1074, 0, 0) → t = NaN ×3 (θ/2 rounds to 0,
  and sinc is 0/0; exact t = v); |ω| = 1.7e103 drops the K² term (t = v, exact (n̂·v)n̂). The test
  checks only 2^-1035. [medium, the K² term low; LIE-F4, LIE-F9]
- **`1 − cos θ` and `cosh η − 1` by subtraction** in `so3_from_axis_angle`, `so3_exp`,
  `lorentz_rotate` and `lorentz_boost`: 8.3e-8 relative at θ = 1e-5, 0 at 1e-8 (exact
  3C752709B8F6B63D); the boost's Λ¹₂ is 0 at η = 1.2e-8 (exact 2.28e-17).
  `su2_to_rotation_matrix(su2_exp)` stays within 2 ulp, and `se3_exp` already uses the half-angle
  form. [medium; AU2-G7-F13 = LIE-F6]
- **`minkowski_interval` squares before subtracting**: (2^520, 2^520(1 − 2^-53), 0, 0) → NaN,
  exact 2^988; (5FF0B1C2D3E4F506, 5FEF1A2B3C4D5E6F, 0, 0) → +Inf, exact 7FC26EC00FAA4C34; a
  lightlike vector near 2^664 → NaN, exact 0. [medium; AU2-G7-F14 = LIE-F5]
- **The 3.4.0 matrix inverses' remaining accuracy gaps**: tiers 1 and 2's derived bound has no
  subnormal term, so an entry beside det ≈ 2^1000 can carry one subnormal step more than it states;
  `m3_normal_matrix_cof` has no accuracy certificate for near-rank-1 M (53 of 1,000 above 2^-10
  normwise, worst 0.27; 1 of 40,000 transformed normals points to the wrong side), and its rescue
  cannot hold a cofactor matrix spanning more than ~2^2098 (both documented); the m4 tier-1/2
  formula's zero signs still come from `f64_neg`, so they follow the consumer's pin (code read: the
  exact tiers flip the sign bit since 3.4.0). [low; R2C n2, n6, REV-FB N2, N3; the bounds not
  re-verified]
- **`gell_mann(7)` (λ₈) is 1 ulp off**: 3FE279A74590331D / BFF279A74590331D, correctly rounded
  …331C (1/√3 formed as `f64_div(1, f64_sqrt(3))`). [low; LIE-F11]
- **`f64v_dot` returns +0 where the IEEE sum is −0** (x86's accumulator starts at +0):
  `hvec3_dot((0, 0, 0), (BFF0000000000000 ×3))` → 0, exact 8000000000000000; also `hvec4_dot` and
  `hquat_dot` (206 of 6,000 records for vec3, 101 for vec4 / quat). The two divisions by a dot in
  geo_diff refuse a non-finite reciprocal. A hisab-side form, or the upstream report of *Decisions
  owed* #11. [low; AU2-G7-F10 = F-S1]
- **`f64_approx_eq(+Inf, +Inf, 1)` → 0** (Inf − Inf = NaN). [low; AU2-G7-F11]
- **sRGB breakpoints take the power branch** (the stored thresholds round below 0.04045 / 0.0031308,
  and the test is `<` where IEC 61966-2-1 has `≤`): `srgb_to_linear(3FA4B5DCC63F1412)` →
  3F69A5C75C81E0EB, standard 3F69A5C61C57A062. **Tone maps overflow where the answer is
  representable**: `tonemap_aces` is NaN from x ≈ 1.3e154, `tonemap_reinhard_extended(1e200, 1)`
  +Inf. **`linearize_depth`** fails on an infinite far plane ((0.5, 0.01, +Inf) → NaN, exact 0.02)
  and overflows near·far ((0.5, 1e200, 1.002e201) → +Inf). [low; AU2-G7-F12 = SPC-F6, AU2-G7-F16 =
  SPC-F7, SPC-F8]
- **fBm leaves its documented "approximately [−1, 1]" for negative persistence** (code read: both
  `fbm_2d_checked` and `simplex_fbm_2d_checked` state it): `simplex_fbm_2d(404F6D245415BABD,
  4047EBB17CCF4E41, 2, 1, 1, −0.5)` → 2.897; persistence BFEFFFFFFFFFFFFF → −9.5e15. The Perlin
  forms share the normalisation (not probed). [low; SPC-F10, AU2-G6-F9 (partly repaired)]

*Tests that pin a defect* (each is re-derived when its finding is repaired; 3.4.0 line numbers)
- `tests/abuse.tcyr:6261-6263` asserts `sdf_capsule(a == b)` and `sdf_smooth_union(0, 0, 0)` are
  NaN ("not a fabricated distance"; the exact distance is 4); `:7205` asserts
  `ode_dopri45_trajectory` is Ok for tf < t0; `:4378-4379` asserts `num_fft(…, −4)` is a no-op;
  `:726-733` pins `levi_civita_3` / `minkowski` out of range. [AU3-F23, AU2-G1-F8, G4-F5, AU3-F25,
  AU2-G3-F12]
- `tests/modules_c.tcyr:4943` calls the `cga_norm` residue past 2^538 "inherent"; `:5936` calls a
  scaled-origin `geo_ray_triangle` failure "not the library's"; `tests/hisab.tcyr:3035` says
  `sectional_curvature`'s limit "becomes f64's" (dividing u and v by their largest components is
  exact). [AU2-G1-F2, G2-F1, G6-F5b]
- `tests/modules_c.tcyr:6782-6783`'s `_r3_qc` pins a z component 4.65e15 ulp from the exact
  1.62e-89 (normwise 2^-1300·|v|, documented, but componentwise wrong). [REV-FB N5]
- The optimizers' scale-refusal comments (`optimize.cyr:136-144` and two `modules_c.tcyr` blocks,
  3.3.7's lines) excuse LM's refusals with reasoning that does not apply to it. [AU2-G4-F6, F7]
- **Coverage holes behind the findings**: `cubic_bezier_ease` is asserted only at x = 0
  (`modules_b.tcyr:2147`); `so3_log` at π only on coordinate axes; `se3_exp`'s small |ω| only at
  2^-1035; `num_newton`'s scale sweep stops at 2^-299; the 2.17.0 optimizer scale sweeps counted
  false successes, never refusals; `t2d` / `t3d` composition only with unrotated or uniform-scale
  transforms. [AU2, AU1]
- **Two mutants from the 2026-08-11 audit still survive all seven suites**: swapping ny and nz in
  `lorentz_boost` (every off-axis test compares `lorentz_boost` with itself; `lorentz_boost((0, 1,
  0), 0.5)`'s column 0 is (3FF20AC1862AE8D0, 0, 3FE0ACD00FE63B97, 0)), and `vec_push(results, pd)` →
  `pt` in `octree_query` (every assertion on it is a count). [medium (2026-08-11); AU3-F36, F37]

*Instruments*
- **`check-result-migration.sh` misses three forms**, each compiling clean on 6.7.9 and reducing a
  Result to its tag (`rf(x)` = `Ok(x + 100)`): another call's `)?` later on the line exempts the
  line (`two(rf(3), rf(5)?)` → 105); `return f(…) <op> …` passes as a tail return (`return rf(3) ==
  0;` → 1); a Result function passed by pointer (`fncall1(&rf, 3)` → 0). The tree has 0 offences
  (764 call sites at 3.3.7); the stdlib's 8 Result functions are outside the gate's set. [low;
  F-R1]
- **`check-measurements.sh` still passes two marker shapes silently**: one that opens with "(" or a
  non-version word before a missing path (`(tests/no_such.tcyr)`, `6.6.x-pinned run,
  tests/no_such.tcyr`), and extensions outside FILE_EXT (`foo.c`, `main.rs`, `Makefile.am`). No
  marker in the tree has either shape with a missing in-tree file. [low; R2C n5]
- **Tests that depend on the allocator's internals**: the m0157 group (last in `tests/hisab.tcyr`)
  depends on `lib/alloc.cyr`'s chunk grain and bump policy, on `_numx_dft`'s buffer order and on an
  unmapped word below a fresh mapping, each self-checked; it reserves 512 MB of address space,
  charged in full under strict overcommit (not measured; CI's setting not checked). AL's
  exhausted-heap group in `tests/abuse.tcyr` sets four `lib/alloc.cyr` internals, and each of its k
  is a hand sum of `vec_new`'s layout: a stdlib that changes the initial capacity leaves most
  assertions passing without reaching the check they name. Take each k from `alloc_used()` deltas.
  [low; NU2, REV-NU2 SF3, REV-AL N5]
- **AL's mutation campaign recorded 268 checks as masked** (no input reaches them past an earlier
  refusal in the same function). CLAUDE.md's "revert any part that moves no number" and checking
  every allocation pull opposite ways there; a CLAUDE.md sentence either way is the maintainer's.
  [low; AL F6]
- **258 of 653 public functions have no comment block** (3.3.7: complex 38, transforms 23, color
  22, geo 21, vec3 18, autodiff 16, geo_diff 16, …); several findings above are "no doc block, so no
  contract". The bench harness reaches 178 of 653 public functions; 3.3.7's SIMD functions with no
  row were `hvec3_mul`, `hvec4_add` / `_sub` / `_scale`, `hquat_dot`, `hquat_normalize`,
  `hquat_inverse` and `world_to_screen`. [low; AU2, AU1]

*Docs*
- **`geo_diff.cyr`'s jet_obb comment is stale**: "jet_obb 1.083 us -> 872 ns, -19.5% … 1.8x for all
  twelve partials [measured: bench-history.csv jet_obb]"; the row it names reads 864 ns, the
  configuration predates 2.10.2's rotation partial (the block says 15 partials), and 3.3.7 reads
  557 / 296 = 1.88x. `check-measurements.sh` passes it because the marker resolves. [medium
  (2026-08-11); AU3-F49]
- **`tests/abuse.tcyr:4745`'s message says "degenerate -> zero vector, not NaN"**;
  `geo_triangle_unit_normal` returns (0, 1, 0) by documented design. Only the message is wrong.
  [cosmetic; AU3-F30]
- **Six `geo.cyr` comments cite an "open roadmap row" on the non-finite contract that does not
  exist.** **`lorentz_rotate`'s Rodrigues comment has the wrong sign**; the code is right. [low;
  AU2-G2-F13, LIE-F10 (single run)]
- **Doc lines owed**: `_SH_MAX_SPAN_CELLS`'s comment misstates where ceil leaves i64 (it fits to
  ~9.2e18); `tensor_contract`'s doc says rank N − 2 (a rank-2 input returns rank 1, shape [1]);
  `num_tridiag_solve`'s says it fails "if pivot is zero" (it refuses a pivot below DBL_MIN on
  purpose: a tiny pivot under a normal right-hand side overflows); `sequential_impulse` drops
  friction for μ ≤ 1e-12 and returns zero impulses for iterations ≤ 0. [low; AU2-G5-F15, G3-F13,
  G5-F9, G5-F12]
- **Conventions stated nowhere**: `hquat_from_euler`'s (EULER_XYZ is Rx·Ry·Rz on column vectors:
  extrinsic Z-Y-X, intrinsic X-Y-Z); `num_sieve(limit > 10^7)` returns an empty vec, also the answer
  for limit < 2; `cga_mv_get` / `cga_mv_set`'s 0..31 index range. [low; AU1, AU2]
- **aarch64 fuses the second product per lane of `hvec4_dot` / `hquat_dot`** (`fmla v2.2d`), and
  through `hquat_length_sq` of `hquat_length` and the quaternion normalize, inverse and slerp;
  `hvec3_dot` is the same on both targets. CLAUDE.md's Language rule names only `f64v_axpy` /
  `f64v_fmadd`; extending it is *Decisions owed* #3. [low; F-S2, code reading at the 6.7.9 tag]

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
  returns an integer code, which [3.4.1]'s near = far row (AU2-G7-F8) uses. [low; FB F3]
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
    rule for an exponent that is not a CONST node gives NaN at x ≤ 0 ([3.4.1], SYM-F1).
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
    - **ganita: filed** — `ganita_mat_eigen_sym`'s absolute Jacobi stop (see *Toolchain*). Whether
      hisab routes `eigen_symmetric` to `eigen_qr` meanwhile is its [3.4.1] row. (AU2-G3-F3)
    - **cyrius: not filed** — `f64v_dot` starts its accumulator at +0 on x86 (`xorpd`) and aarch64
      (`eor`) alike, the same convention as reference BLAS `ddot`, so an all-(−0) dot product is
      +0 by definition, not a compiler defect. Whether `hvec3_dot` / `hvec4_dot` / `hquat_dot`
      should give the natural evaluation order's −0 is hisab's own [3.4.1] row. (F-S1 = AU2-G7-F10)
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
`eigen_symmetric` is exposed ([3.4.1]).

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
- **Thread safety of the process-global tables**: Perlin, the einsum arena, the CGA table, simplex
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
2026-10-31 (unchanged since 2026-09-30):

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
