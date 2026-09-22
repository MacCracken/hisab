# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.6.6**. Stdlib `ganita` (6.2.x math umbrella) provides dense
> decompositions + transcendentals.

⭐ **This file is future-facing only.** Nothing below has shipped. The record of what *has* is:

| record | lives in |
|---|---|
| per-release changes, with the numbers | [`CHANGELOG.md`](../../CHANGELOG.md) |
| per-defect filings, in-tree and shipped | [`issues/`](issues/) and [`issues/archived/`](issues/archived/) |
| toolchain defects | `cyrius/docs/development/issues/` — **not this repo** |
| the evidence behind a closed item | [`audit/`](../audit/) |
| test counts and suite composition | [`../guides/testing.md`](../guides/testing.md) |

⚠ **Completed items are removed rather than struck through** (changed 2026-09-11 — this file had
grown to 1308 lines of which ~95% was history, and a backlog nobody can see the end of is not a
backlog). If you need to know why an item was closed, or that it was closed *differently from how it
was scoped*, the CHANGELOG entry for its release says so — that is where the arc narrative lives.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current — v3.2.1

Suite **4429** across five harnesses (hisab 585, foundation 429, modules 2251, edge_cases 260,
abuse 904), constant gate **159/159**, **80** benchmarks, **35** `[lib]` modules, toolchain
**6.6.6**, sakshi **2.5.2**, ganita **1.2.6**. All gates green; per-release detail is in
[`CHANGELOG.md`](../../CHANGELOG.md). **2.24.0 is the supported 2.x line** — 3.0.0's `Result<T, E>`
migration is breaking, has no deprecation window, and
[`../guides/migration-3.0.md`](../guides/migration-3.0.md) is the consumer-facing guide. ⚠ **hisab
≥ 3.1.0 requires cyrius ≥ 6.6.3** (`public struct` + `#derive`; measured 3.2.1: the 3.0.1 bundle
compiles under 6.6.2, the 3.1.0 bundle does not).

## How to read this file

⭐ **Every open item carries its target version in bold brackets** — `**[4.0.0]**` — or sits in a
section that is deliberately unversioned. *Optional, demand-gated* and *Parked / deferred* hold work
with **no driver yet**; nothing moves out of them without a consumer asking, and when one does it
gets a version here first.

⚠ **An item's measurement is part of the item.** Rows here carry the number that scoped them,
because a row without one is how this file repeatedly got the size of a class wrong — seven
consecutive releases where a row named one or two sites and a tree-wide grep found five, six, or
fifty-one. **Grep for the shape before sizing anything below.**

⛔ **An open row is not a verified one.** When the open items were last handed to a verifier told to
*prove each already done* (2026-09-09), 15 of 39 rested on a premise that had since become false.
Check the tree before believing a row in either direction — including a row that says something is
blocked.

---

## Open items

### The `private` flip — **[4.0.0]**

The `pub fn` half shipped in 3.1.0 (729 `public` declarations, `scripts/check-public-surface.sh`
proving the surface complete and exact under a full `private` flip on every CI run). What remains
is the flip itself, and **everything below is measured, not estimated**.

⛔ **THE BUNDLE COLLAPSES MODULE BOUNDARIES, WHICH SPLITS THE FLIP INTO TWO DIFFERENT PROBLEMS.**
`dist/hisab.cyr` is ONE file and `private` is per-file, so inside the bundle every "cross-module"
call is an in-file call: a flipped bundle faces only a CONSUMER, and for a consumer the surface is
already exactly the 729 public items (the gate's claims 2–3 prove it in both directions). The
per-file boundary exists only where modules are included as separate files — `tests/*.tcyr`,
`tests/hisab.bcyr`, `examples/*.cyr`, and any consumer that includes `src/` directly. **So the
consumer-facing half of the flip costs nothing further; the whole remaining cost is hisab's own
suites.** ⚠ Consumers compile the bundle under their OWN pins, and the boundary is enforced by cycc
only from **6.6.4** (below it `&_private_fn` and a `public enum`'s neighbour leak) — one more
reason this is a 4.0.0 item.

⛔ **What the flip breaks today, measured by compiling the suites against a fully-`private` tree**:
**64 distinct `_` names, 387 sites** — `tests/hisab.tcyr` 14 sites / 4 names, `tests/modules.tcyr`
372 / 59, `tests/hisab.bcyr` 1 (`_CGA_NULL_TBL`, the 2.24.0 cold-row guard); `foundation`,
`edge_cases`, `abuse` and the fuzz harness are already flip-clean. 56 are fns, 8 are globals
(`_SP_MAX_TREE_DEPTH`, `_SP_F64_POS_INF`, `_SH_ENTRY_SIZE`, `_GA_EPA_POLISH_COUNT`,
`_CGA_NULL_TBL`, `_CGA_NULL_COEF_BAD`, `_COL_F64_NEG_INF`, `_GEO_F64_THIRD`). Top by sites:
`_f64arr_set` 125, `_f64arr_alloc` 27, `_col_dl_incircle` 24, `_f64arr_get` 15, `_col_ghost_ux`/`_uy`
15 each, `_col_dl_ic_g1` 13, `_GA_EPA_POLISH_COUNT` 12. Three families: white-box layout probes
(spatial's 17 `_kd_node_*`/`_qt_node_*`/`_ot_node_*`/`_sh_*` accessors, `_bvh_node_*`), the
instrumentation counters mutation-proven guards depend on (`_CGA_NULL_TBL`, `_GA_EPA_POLISH_COUNT`,
`_CGA_NULL_COEF_BAD` — these need a public read-only getter, e.g. `cga_null_table_built()`, or
the guards die with the flip), and the `_f64arr_*` scratch helpers. Each needs a decision:
rewrite the test against the public API, or promote as a documented inspector.

⭐ **The 25 cross-module `_` items carry `public` plus a marker comment, and each has a 4.0.0
disposition** (worked by a 7-reviewer / 3-refuter pass on 2026-09-13, 0 refuted; `_lext_sort_desc`
added in 3.2.0). Each is grounded in the callee body and the caller sites:

| item(s) | defined in → reached from | disposition |
|---|---|---|
| `_GEO_F64_POS_INF` | geo → geo_diff | **retire the reach**: it is a pure alias of the public `F64_POS_INF`; point geo_diff's 6 reads at that and drop the marker |
| `_noise_fade` | calc → calc_ext | **retire the reach**: bit-for-bit identical to the public `ease_in_out_smooth`; replace 5 sites, delete the helper (perlin's bit-exact tests are the acceptance test) |
| `_COL_F64_ZERO` / `_ONE` / `_NEG_ONE` | collision_core → collision_mesh | **retire the reach**: literals; use `0`, `F64_ONE`, a local `-1.0` at the 18 sites (same bit patterns, arithmetic unchanged) |
| `_COL_SENTINEL` | collision_core → collision_mesh (+ tests) | **promote AND relocate**: it is the half-edge data contract (`twin` of a boundary edge, `vertex_edge` of an isolated vertex), never used by collision_core itself → `public var HALFEDGE_NONE` in collision_mesh |
| `_epa_seed_gjk` / `_epa_seed_portal` / `_epa_refine` / `_epa_touch_probe` | geo_advanced → collision_core | **merge at section level**: move the MPR section (~30 code lines) next to `gjk_epa_3d`; NOT promote — `out4` and the 0/1/2 seed codes are pipeline internals |
| `_perm_init` / `_perm` | calc → calc_ext, noise_simplex | **promote-rename** `noise_perm_init` / `noise_perm` (Perlin's reference 512-entry table; three algorithm families in three files share it, so neither merge nor leave-open resolves it) |
| `_num_is_pow2` | num → num_ext | **promote-rename** `num_is_pow2`: the predicate for the precondition every FFT entry point documents |
| `_num_mulmod` | num → num_ext | **promote-rename** `num_mulmod`, and DECIDE the precondition: its own doc says `num_modpow` is the only entry point and enforces `a >= 0`; `num_pollard_rho` and `num_crt` reach it directly and `num_crt` does not |
| `_su2_alloc` / `_su2_x` / `_su2_y` / `_su2_z` | lie → lie_ext | **promote the OPERATION, not the accessors**: `su2_rotate_vec3(g, v)` with the existing sandwich arithmetic (byte-identical results); the raw allocator exists precisely because every public constructor normalises |
| `_lie_norm3` | lie → lie_ext | **promote-rename** `lie_norm3` (scale-safe component norm; `hvec3_length` is not a drop-in — it takes an HVec3 and moves results by an ulp in the normal band) |
| `_SYM_2_POW_63` / `_sym_int_exact_buf` / `_sym_render_f64` (+ `RenderLayout`) | symbolic → symbolic_ext | **extract one function**: `sym_const_to_str(val)` holding `expr_to_str`'s EXPR_CONST branch; `_latex_fmt_const` calls it, and the 2.20.0 "both renderers agree at every magnitude" property becomes true by construction |
| `_SYM_EPS` / `_sym_is_zero` | symbolic → symbolic_ext | **promote as** `sym_const_eq(a, b)` with the 1e-15 absolute tolerance stated; `_SYM_EPS` stays private |
| `_lext_sort_desc` (3.2.0) | linalg_ext → linalg_precision | **promote-rename** `linalg_sort_desc(vals, n, by_abs, col_mat, col_rows, row_mat, row_cols)`: it is the ONE ordering every eigen/SVD entry point returns through, its tie rule (first max, strict `>`, swap) is pinned by 8 assertions and observable to every consumer, and a private twin in each file is exactly what 3.2.0 consolidated away |

⚠ **Five non-underscore names the review judged implementation details, now committed as API by
the naming convention** (decide before the flip — an underscore rename breaks nobody, 0 consumer
references measured): `EPSILON_F32` (error.cyr — its own comment says unused; retire it),
`F64_1E_NEG30` and `F64_NINE` (calc_ext), `F64_THREE`..`F64_FIFTEEN` (calc), `F64_SIX_DG`
(diffgeo — a copy of `F64_SIX` whose suffix only dodges the collision), `RenderLayout` (symbolic —
scratch-buffer sizes). And `public struct GeoJet` + derive exports raw slot accessors and nine
setters the module header says are reachable ONLY through the typed accessors: amend the header or
wrap.

**Order:** the dispositions above → rewrite or promote the 64 `_` names the suites reach → flip
`private` last. The consumer-call gate already exists (`check-public-surface.sh` claim 2).

---

## Toolchain, tracked upstream

⚠ **Cyrius bugs are filed in the CYRIUS repo** — `cyrius/docs/development/issues/` is where the
language agent reads them — **and closed in THIS repo too when a bump fixes them**, because the
cyrius agent never edits hisab (the pattern: a hisab-side record in `issues/archived/` carrying the
paired measurement, written on the bump that closes it).

⭐ **The 2026-09-14 register-picker filing is CLOSED** — fixed upstream in cycc 6.6.5, hisab's pin
crossed it in 3.2.1 with the paired measurement in
`issues/archived/2026-09-14-cyrius-simd-dst-slot-regalloc-picker.md`. `m3_mul_vec3` keeps its hoisted
z tail on its merits (every live consumer pins 6.6.2–6.6.4, below the fix); the comment says so.

⛔ **ONE hisab-filed toolchain item is OPEN** — filed 2026-09-21 as
`cyrius/docs/development/issues/2026-09-21-hisab-bench-min-above-mean-below-resolution-bar.md` (+
self-proving repro: exit 0 on 6.6.4, 1 on 6.6.5/6.6.6), recorded here as
`issues/2026-09-21-cyrius-bench-min-above-mean-below-resolution-bar.md`. `lib/bench.cyr` 6.6.5 lets a
window into `min`/`max` only if it clears a 100 × (clock read + tick) bar; a fixed 2000-op window for
a sub-40 ns op sits under it and resolves only when perturbed, so the printed `min` is the minimum of
the SLOW windows and sits above `avg` (`vec3_add: 16ns avg (min=39ns …)`, 24 of 320 rows, 7
benchmarks). **No hisab number is wrong** — the CSV records `avg`, measured unmoved (median +0.00%) —
but `min_ns`/`max_ns` for those seven rows are not comparable across 2026-09-21. No source change:
widening `bench_batch` is a benchmark-shape change (2.20.0). When it closes: archive the record, and
re-check whether `bench_run_batch`'s fixed windows still need the caveat in `benchmarks.md`.

**`bench_run` auto-batching (6.5.19)** — already in force. The **39 `bench_batch()` call sites are
deliberately unchanged**: they buy a FIXED window rather than escape the timer floor, which is still
worth having when comparing two runs at identical batch sizes. Re-evaluate only if a reason appears.

---

## Optional, demand-gated

- **SIMD the flat-array kernels** — `_opt_dot`/`_opt_norm`/`_opt_axpy`, the L-BFGS sweeps,
  `_lext_dot`/`_lext_norm`. ⚠ **The stated gate is not the blocker.** Benchmarks are cheap (78
  working labels in the harness). The real blocker is that **nobody knows the `n`**: every test calls
  these solvers at n = 1, 2, 3, where a 2-wide dot cannot win, so authoring a benchmark today would
  reproduce the exact failure mode this project has recorded twice — measuring the instrument
  instead of the operation. **Needs a consumer-sourced `n` first**, and ten consumers are now live
  to ask.
  ⚠ **Correct the citation before quoting it**: 2.3.1 measured 1.6–6.5x (median ~2.3x), not the
  "5–8x" this row used to claim — and much of that win was accessor-call elimination, which does not
  exist here.
  ⚠ Secondary: these buffers are exactly `alloc(n*8)` and `f64v_*` **over-reads on odd `n`**, so
  each kernel needs the pair+scalar-tail hybrid, not a one-line swap.

---

## Parked / deferred (revisit when a driver appears)

Evaluated during earlier arcs and consciously deferred — recorded so they are not silently lost, and
**each with the measurement that parked it**, because "deferred with a reason" and "forgotten" are
indistinguishable once the reason is gone.

- **SIMD `cross`** (from 2.3.1) — needs lane shuffles; `f64v_shuffle`/`permute`/`blend`/`swap` are
  all undefined on 6.6.2 (probed), so still correctly parked. ⭐ Now with a number instead of an
  assertion: the best shuffle-free formulation measures **38 ns vs 25 ns scalar (+52%)**. (`lerp`
  needs no shuffle and is not gated on this.)
- **`#pure` annotations** (from 2.3.4) — parked, but **not for the reason originally recorded**.
  ⛔ The old premise ("unsafe CSE interaction with the allocate-a-fresh-result convention") is
  refuted on 6.6.2: *there is no CSE to be unsafe*. Three identical `#pure` calls emit `calls: 3`,
  two `#pure` allocating calls return distinct pointers, and the binaries are **byte-identical**
  (same sha256) with and without the annotation. `#pure`'s entire effect in cycc is two warnings.
  ⛔ **The stronger objection is the one to keep**: `alloc()` carries no `#alloc`, so annotating
  hisab's 310 `alloc()` sites `#pure` would assert a falsehood with no compiler backstop.
- **Slices (`[T]` / `slice<T>`)** (from 2.3.4) — parked, and now measured rather than predicted:
  checked slice indexing is **3.8x** raw `load64`, and the "unchecked escape hatch" is still
  **3.2x** — it discards the safety *and* keeps 85% of the cost. The toolchain's
  `~/.cyrius/versions/<pin>/lib/simd.cyr` has **zero** slice-taking forms, so slices provably cannot
  cover the SIMD hot paths at all.

---

## Consumers

**Ten repos consume `dist/hisab.cyr` today, SHA-locked** — read from each `cyrius.cyml` on
2026-09-21: **prakash** on hisab **3.1.1** (cyrius 6.6.4; pinned 2026-09-15, `num_fft` through the
`var rt, rv = …` idiom — the first live consumer past the `Result` break, which closed the roadmap
item that said none had); svara, naad, goonj on **2.22.1**; dhvani, attn11, ghurni, prani, garjan,
nidhi on **2.11.2**. Their cyrius pins are **6.6.3** (svara, naad, dhvani, ghurni, prani, garjan,
nidhi), **6.6.4** (prakash) and **6.6.2** (goonj, attn11). `svara/src/spectral.cyr:246` calls `num_fft`.

⛔ **goonj and attn11 cannot take ANY hisab ≥ 3.1.0 until they move their cyrius pin**: `public
struct` + `#derive` is refused by cycc 6.6.2 (`#derive(...) applies to a struct or an enum`), measured
on the 3.1.0 and 3.2.1 bundles from a dir pinned to 6.6.2, while the 3.0.1 bundle compiles there.
Under 6.6.3 and 6.6.4 the 3.2.1 bundle compiles and a consumer-shaped program runs correctly. ⚠ The
pin matters to every consumer for a second reason: `_cga_build_null_tbl` keeps its `if`-guard form and
`m3_mul_vec3` its hoisted z tail because a consumer compiles the bundle under its OWN cycc, and the
natural forms are silently wrong below 6.6.3 and 6.6.5 respectively.

⛔ **The remaining nine are still on 2.x, and the `Result` break's failure mode is silent**: a
`Result` in *argument* position degrades to its tag, and `Ok` tag = 0 = `HSB_ERR_NONE`, so
`assert_eq(f(...), HSB_ERR_NONE)` **builds clean and tests nothing**. Hand a migrating consumer
`scripts/check-result-migration.sh`, which exists for exactly that class and is mutation-proven. The
reached surface is small — **35 distinct public fns in 8 modules** (Boundary table) — so each port is
bounded.

⛔ **impetus, kiran, joshua, aethersafha, hisab-mimamsa and kana are NOT consumers** — they have no
`cyrius.cyml` on any branch. They are Rust repos needing a *port*, not a scheduling decision
(verified 2026-09-09, and again 2026-09-14 for abaco: its README calls hisab a sibling, not a
consumer; the ten live pins re-read 2026-09-21). The table below is what they *would* use, kept because it is the planning
surface; it is not a statement that anything is wired up.

| Planned consumer | Domain | Surface it will use |
|----------|--------|---------------------|
| **impetus** | physics | GJK/EPA, MPR, PGS, sequential-impulse, inertia, spatial |
| **kiran** | engine | projections, BVH, k-d tree, frustum |
| **joshua** | simulation | DOPRI45, BDF, symplectic, optimize |
| **aethersafha** | compositor | projections, compositing, color |
| **abaco** | expression eval | symbolic integrate/LaTeX/patterns, interval |
| **hisab-mimamsa** | physics | tensors, Lie groups, diffgeo, CGA |
| **kana** | quantum | tensors, Lie groups, complex LA, spinors |

**Known caveat, carried forward:** `gjk_intersect_3d` costs **+62% (box) / +57% (sphere)** on the
no-hit path since 2.9.0 — the price of it no longer missing 134 genuine interior overlaps per 4,386
evaluations. Re-derived on 6.6.2 post-ganita by same-binary ABBA, so the older "~+55%" understated
it. The no-hit path is the broadphase-common case. ⛔ But "a cheaper pre-filter" cannot be built
inside the entry point, which receives only two support-function pointers and so has **no cheaper
information to filter with** — and hisab already ships the broadphase (`src/spatial.cyr`). **This is
caller-side work, not a hisab roadmap item**, and with ten live consumers someone can now be asked
whether it actually bites.

---

## Boundary with Abaco

One row per surface, at the 3.1.1 surface (fn counts are `^public fn` in the module, `_` helpers
excluded). The last column is **measured, not assumed**: every call of a hisab public fn in each
consumer's own source (`lib/`, `dist/`, `build/` excluded), grepped 2026-09-14 across abaco and the
ten live consumers.

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

Hisab should never depend on abaco. Abaco may optionally depend on hisab — and **today it does not**:
abaco's `cyrius.cyml` has no `[deps.hisab]`, its 22 source files call 0 hisab symbols, and its own
README lists hisab as a *sibling* library, "not a consumer" (measured 2026-09-14). The "abaco"
row in the planned-consumer table above is therefore a plan on hisab's side only. Across the ten
live consumers the whole reached surface is **35 distinct public fns in 8 modules**
(vec3, num, num_ext, calc, calc_ext, geo, geo_advanced, f64_util); ghurni `include`s the bundle
and calls nothing from it, nidhi names hisab only in a comment, and **0 of 10 reference `HSB_*` in
`src/`** — prakash's `tests/wave_pattern.tcyr` asserts `HSB_ERR_INVALID_INPUT` as the Err payload of
`num_fft(…, 3)`, which is the correct post-3.0.0 idiom (re-read 2026-09-21).
Eight of hisab's 35 modules have a live caller; the 27 others — everything the rows above mark
"none" included — are reached only by hisab's own suites.
The only git dep is sakshi, and `deps --verify` already fails any unreviewed dep — **no gate is owed
here.** ⚠ `eval("sin(pi/4)")` being abaco's while hisab exposes `expr_eval` is not a contradiction:
`src/symbolic.cyr` takes a **tree**, not a string. hisab has no tokenizer at all.
