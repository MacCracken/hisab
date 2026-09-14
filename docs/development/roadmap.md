# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.6.3**. Stdlib `ganita` (6.2.x math umbrella) provides dense
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

## Current — v3.1.0

Suite **4214** across five harnesses (hisab 550, foundation 413, modules 2098, edge_cases 239,
abuse 914), constant gate **159/159**, **78** benchmarks, **35** `[lib]` modules, toolchain
**6.6.3**, sakshi **2.5.2**, ganita **1.2.5**, and **zero** deprecated-alias call sites. All gates
green: `lint` 0 warnings and `fmt <file> --check` 0 drift across all 44 sources, `vet` 2 deps /
0 untrusted / 0 missing, `deps --verify` 31/31, `fuzz` 1/0, `coverage` 640/644 (99%) functions over
36/36 files, distlib in sync.

3.1.0 is the `pub fn` half of the public/private surface: 729 declarations annotated, byte-identical
binaries, and `scripts/check-public-surface.sh` proving the surface complete and exact under a full
`private` flip on every CI run. 3.0.1 was the toolchain catch-up onto 6.6.3. 3.0.0 is the `Result<T, E>`
migration — 48 functions, 182 `Ok`/`Err` returns, 18 `?` sites, 533 call
sites — and it is breaking. [`../guides/migration-3.0.md`](../guides/migration-3.0.md) is the
consumer-facing guide; **2.24.0 is the supported 2.x line**, and there is no deprecation window.

## How to read this file

⭐ **Every open item carries its target version in bold brackets** — `**[3.2.0]**` — or sits in a
section that is deliberately unversioned. *Optional, demand-gated* and *Parked / deferred* hold work
with **no driver yet**; nothing moves out of them without a consumer asking, and when one does it
gets a version here first.

⚠ **An item's measurement is part of the item.** Rows here carry the number that scoped them,
because a row without one is how this file repeatedly got the size of a class wrong — seven
consecutive releases where a row named one or two sites and a tree-wide grep found five, six, or
fifty-one. **Grep for the shape before sizing anything below.**

⛔ **An open row is not a verified one.** The 2026-09-09 sweep handed every open item to a verifier
told to *prove it already done*: 21 came back genuinely open, **15 rested on a premise that had
become false**, and 3 were finished. Check the tree before believing a row in either direction —
including a row that says something is blocked.

---

## Open items

### The `private` flip — **[4.0.0]** (the `pub fn` half shipped in 3.1.0)

hisab's API is now DECLARED rather than implied: as of 3.1.0 every non-underscore top-level
fn / struct / enum / var in `src/` carries `public` (729 declarations — 660 fn, 22 struct, 39 var,
8 enum), and no module is `private`, so the annotation is the documented no-op — the suites compile
**byte-identical** with and without it. `scripts/check-public-surface.sh` flips all 35 modules
`private` in a scratch copy on every CI run and proves the surface complete and exact (five claims,
four mutants killed). What remains is the flip itself, and **everything below is measured, not
estimated** — this row used to say "52 `_` functions across 235 sites" and counted neither globals
nor the benchmark harness.

⛔ **THE BUNDLE COLLAPSES MODULE BOUNDARIES, WHICH SPLITS THE FLIP INTO TWO DIFFERENT PROBLEMS.**
`dist/hisab.cyr` is ONE file and `private` is per-file, so inside the bundle every "cross-module"
call is an in-file call: the 35 `private` lines a flipped bundle would carry face only a CONSUMER,
and for a consumer the surface is already exactly the 729 public items (the gate's claims 2–3
prove it in both directions). The per-file boundary exists only where modules are included as
separate files — `tests/*.tcyr`, `tests/hisab.bcyr`, `examples/*.cyr`, and any consumer that
includes `src/` directly. **So the consumer-facing half of the flip costs nothing further; the
whole remaining cost is hisab's own suites.** (The gate's first draft checked the flipped bundle
and stayed green with `_perm` unmarked — it was proving nothing. Claim 1 now includes each module
as its own file.)

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

⭐ **The two upstream holes that made the boundary unenforceable are FIXED in cyrius 6.6.4**
(hisab pinned it in 3.1.1): `&_private_fn` from another file is refused like the direct call, and a
`public enum` no longer leaks `public` onto the next declaration — the gate's `_ad_pow` allowlist
inverted on the pin bump exactly as designed and is now empty. ⚠ Consumers compile `dist/hisab.cyr`
under their OWN pins; the flip is only enforced for a consumer at ≥ 6.6.4, which is one more reason
it is a 4.0.0 item.

⭐ **The 25 cross-module `_` items now carry `public` plus a marker comment** (24 at 3.1.0; 3.2.0's
sort consolidation added `_lext_sort_desc`), **and their 4.0.0 dispositions were worked by a
7-reviewer / 3-refuter pass on 2026-09-13 (53 findings, 0 refuted), plus one row for the newcomer.
The recommendations, each grounded in the callee body and the caller sites:**

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

**Order, unchanged:** consumer-call gate (done: `check-public-surface.sh` claim 2) → the
dispositions above → flip `private` last (the upstream holes are fixed and the gate's known-leak
inversion has already fired).

### Get one live consumer onto 3.0.x — **[unversioned — external]**

⚠ **No live consumer has built 2.11.3 or later.** All ten sit at 2.11.1/2.11.2, behind the
6.5.33 → 6.6.3 toolchain bump, the 536-site ganita alias migration, **and now the `Result<T, E>`
break**. ⚠ 3.0.1 adds a reason the pin matters to THEM: `_cga_build_null_tbl` keeps its
`if`-guard form precisely because a consumer compiles `dist/hisab.cyr` under its OWN cycc, and
the natural `continue` form is silently wrong below 6.6.3. `cyrius check --with-deps dist/hisab.cyr` proves the bundle compiles against **this**
manifest's ganita — not against theirs, and not against a real call graph.

⛔ **The `Result` break makes this urgent rather than merely overdue**, because of the failure mode
the migration guide leads with: a `Result` in *argument* position does not error, it degrades to its
tag, and `Ok` tag = 0 = `HSB_ERR_NONE`. A consumer whose checks look like
`assert_eq(f(...), HSB_ERR_NONE)` will **build clean and test nothing**. Hand them
`scripts/check-result-migration.sh`, which exists for exactly that class and is mutation-proven.
**Highest-value action in this section.**

---

## Toolchain, tracked upstream

⚠ **Cyrius bugs are filed in the CYRIUS repo** — `cyrius/docs/development/issues/` is where the
language agent reads them — **and closed in THIS repo too when a bump fixes them**, because the
cyrius agent never edits hisab (both 2026-09-11 filings were fixed in 6.6.3 and closed here in
3.0.1: `issues/archived/2026-09-11-cyrius-*`).

⛔ **ONE hisab-filed toolchain item is OPEN, filed in the cyrius repo on 2026-09-14** (`cyrius/docs/development/issues/2026-09-14-hisab-simd-dst-slot-regalloc-picker.md` + repro).
`issues/2026-09-14-cyrius-simd-dst-slot-regalloc-picker.md` (cycc **6.6.4**, wrong code): a derived
SETTER whose value argument reads the same object through the derived GETTER, in a function that also
expands `f64v_*` intrinsics, leaves one intrinsic's DESTINATION frame slot unwritten — the register
picker rewrites the slot store into a register move and the inline loop still reads the slot. Silent
`(1, 4, 3)` for `(4, 5, 3)` standalone; SIGSEGV inside hisab. The self-proving repro under
`issues/repros/` (exit 1 stock, exit 0 with `CYRIUS_REGALLOC_PICKER_CAP=0`) is wrong on **6.6.0 through
6.6.4** from dirs pinned to each, so it is not a 6.6.4 regression. Until it is fixed and the pin
has crossed the fix, `m3_mul_vec3` keeps its z tail in a local and stores it through the setter
ONCE (comment on the function); do not tidy it back to the natural form. When it closes: archive the
filing here, and the hisab-side `m3_mul_vec3` form may be re-measured (the hoisted form is −5.4% on
its own, so there may be nothing to change).

The four **2026-09-13 filings were repaired in cyrius 6.6.4 and closed here in 3.1.1** (2026-09-14).
What each still means for hisab:

| filing (upstream, `2026-09-13-hisab-…`, archived there) | after 6.6.4 |
|---|---|
| `private-fn-reachable-via-address-of.md` | Fixed: `&fn`, `s.method()` and six more resolution paths now run `_vis_check`. The public-surface gate keeps CALL probes (a consumer writes calls; a gate valid only from 6.6.4 up would be blind below it). |
| `public-enum-leaks-onto-next-declaration.md` | Fixed. The gate's known-leak inversion fired on the pin bump (`_ad_pow` refused, 457/457); `KNOWN_LEAKS` is empty. |
| `refresh-only-overwrites-released-snapshot.md` | Fixed upstream (`install.sh --refresh-only` refuses a released slot; `verify-store.sh`). hisab STILL byte-compares `lib/` against the cyrius TAG, never the install dir — the rule cost nothing and found this. |
| `deps-relocks-silently-under-unchanged-pin.md` | Fixed: `cyrius.lock` carries a `cyrius\t<pin>` trailer and a stdlib leaf whose snapshot hash moves under an unchanged pin is REFUSED (`deps --relock` accepts). Any `git status` change to `lib/` or `cyrius.lock` after a build is still worth reading. |

**`bench_run` auto-batching (6.5.19)** — already in force; it is what moved 44 benchmark rows when
the instrument changed. The **39 `bench_batch()` call sites are deliberately unchanged**: they now
buy a FIXED window rather than escape the timer floor, which is still worth having when comparing
two runs at identical batch sizes. Re-evaluate only if a reason appears.

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
  assertion: the best shuffle-free formulation measures **38 ns vs 25 ns scalar (+52%)**.
  ⚠ `lerp` was never gated on this at all; it shipped on the shuffle-free n=2 hybrid in 3.2.0.
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

**Ten repos consume `dist/hisab.cyr` today, SHA-locked**: svara, naad, goonj, dhvani, attn11,
ghurni, prani, garjan, prakash, nidhi. svara's `cyrius.lock` pins hisab commit `1bc71e3`
(tag **2.11.2**) and `svara/src/spectral.cyr:246` calls `num_fft`.

⛔ **impetus, kiran, joshua, aethersafha, hisab-mimamsa and kana are NOT consumers** — they have no
`cyrius.cyml` on any branch. They are Rust repos needing a *port*, not a scheduling decision
(verified 2026-09-09). The table below is what they *would* use, kept because it is the planning
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
and calls nothing from it, nidhi names hisab only in a comment, and **0 of 10 reference `HSB_*`**.
Eight of hisab's 35 modules have a live caller; the 27 others — everything the rows above mark
"none" included — are reached only by hisab's own suites.
✅ **Verified 2026-09-09**: 0 hits for `abaco` in `cyrius.cyml`, `cyrius.lock`, `dist/hisab.deps`,
`dist/hisab.cyr` and `src/`. The only git dep is sakshi (2.5.2 as of 3.0.1), and `deps --verify` already fails any
unreviewed dep — **no new gate is owed here.** ⚠ The apparent contradiction between
`eval("sin(pi/4)")` being abaco's while hisab exposes `expr_eval` is not one: `src/symbolic.cyr:339`
takes a **tree**, not a string. hisab has no tokenizer at all.
