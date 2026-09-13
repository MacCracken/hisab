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

⛔ **TWO UPSTREAM HOLES MEAN THE BOUNDARY IS NOT YET ENFORCEABLE, and both are filed with
self-proving repros** (`cyrius/docs/development/issues/2026-09-13-hisab-*`):
- **`&_private_fn` from another file compiles and RUNS** (exit 42 via `callptr` and `fncall1`) while
  the direct call is refused — the visibility check covers calls and var reads, not address-of.
  Until it is fixed, `private` documents intent and does not enforce it; flipping before the fix
  ships a boundary a consumer crosses in one token.
- **A `public enum` leaks `public` onto the next top-level declaration** (6.6.2 and 6.6.3). hisab
  has two `_` items in that position — `_ad_pow` after `AdPowLimit` (allowlisted in the gate, which
  FAILS the day upstream fixes it) and `_SYM_EPS` after `ExprTag` (masked by its own marker).
  Six other declarations follow public enums and are API anyway.

⭐ **The 24 cross-module `_` items now carry `public` plus a marker comment, and their 4.0.0
dispositions were worked by a 7-reviewer / 3-refuter pass on 2026-09-13 (53 findings, 0 refuted).
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

⚠ **Five non-underscore names the review judged implementation details, now committed as API by
the naming convention** (decide before the flip — an underscore rename breaks nobody, 0 consumer
references measured): `EPSILON_F32` (error.cyr — its own comment says unused; retire it),
`F64_1E_NEG30` and `F64_NINE` (calc_ext), `F64_THREE`..`F64_FIFTEEN` (calc), `F64_SIX_DG`
(diffgeo — a copy of `F64_SIX` whose suffix only dodges the collision), `RenderLayout` (symbolic —
scratch-buffer sizes). And `public struct GeoJet` + derive exports raw slot accessors and nine
setters the module header says are reachable ONLY through the typed accessors: amend the header or
wrap.

**Order, unchanged:** consumer-call gate (done: `check-public-surface.sh` claim 2) → the
dispositions above → the two upstream holes fixed and re-verified by the gate's known-leak
inversion → flip `private` last.

### The struct-layout contract has no gate — **[3.2.0]**

The contract is real and documented: construct via the documented constructor, read via the
accessors, size arrays with `sizeof(T)`; never a hardcoded byte count, never a hand-computed offset.

⛔ **But "32 assertions … make any such change trip a gate" is FALSE.** All 32 are `sizeof(T) > 0`
or `sizeof(T) % 8 == 0`. `ColContact` going 64 → 72 bytes passes both, **identically, on both sides
of the change they were written to catch**. There is not one `assert_eq(sizeof(T), <n>)` anywhere in
the tree, and six public structs carry no assertion at all — including **`HVec3`, the type live
consumers touch most** (67 `hvec3_new` sites across 15 files in `src/`). The commit that declared the contract "now enforced"
enforced nothing, and it has been quoted as protection for five releases. **A gate that cannot fail
is not a gate.**

⭐ The repair is mechanical: an exact `assert_eq(sizeof(T), <n>)` per public struct, plus the six
that have nothing. ⚠ The 3.1.0 review added a site to the list: `hvec2_new` itself constructs
`HVec2` with a hardcoded byte count and hand-computed offsets (`alloc(16)`, `store64(v + 8, y)`),
the exact shape the contract forbids, in the module every consumer touches first. Verify it fires before trusting it — install a one-field change and watch the
assertion fail.

### Public API reached by no test — **[3.2.0]**

The 3.1.0 surface scan (comments and string literals stripped) found **7 public functions
referenced by nothing** in `src/`, `tests/`, `examples/` or `dist/` beyond their own declaration:
`ad_neg`, `ad_cos`, `ad_ln` (the tape-mode twins of tested `dual_*` — `ad_ln`'s domain guard has
never been asked), `csr_new` (the only CSR constructor that bypasses `csr_from_dense`'s
`|v| > 1e-12` drop), `bch_3rd_order`, and `hodge_star_2form_4d` (whose sign table was rewritten in
2.6.15 after three contradictory descriptions and is pinned by **no assertion**); `ad_grad_write`
was on the list and is not (reached intra-module by `ad_grad_into`). The review added constants and
tags read by nothing: the seven `GEO_JET_*` kind tags (no test reads `GeoJet_kind`), all six
`EulerOrder` members (the only `hquat_from_euler` test passes a literal), `Mat3Layout`/`Mat4Layout`
(two in-tree sites hand-size the arrays instead), `CGA_NUM_BLADES`, and `hisab_is_err` (0 callers
in src, examples, or any of the ten consumers). ⚠ The 3.1.0 review also found the one BCH test that
did exist was passing 24-byte `HVec3` values into `Mat3` readers (a 48-byte over-read that
"passed" on the bump allocator's next bytes) — repaired in 3.1.0 with closed-form fixtures; the
untested twins may hide the same shape. **"99% function coverage" counts a function reached by
anything, including a test that reads garbage.**

### The Boundary-with-Abaco table lags the surface it describes — **[3.2.0]**

⚠ **It is frozen at the 2.2.0 surface.** It has no row for `expr_eval` — whose **domain changed** in
2.11.2 (`(-2)^3` returned NaN for hisab's entire history and returns a number now), and whose named
consumer is abaco — and none for autodiff (forward duals + the reverse tape), the six `geo_diff`
jets, CGA, or Lie. **A boundary table that lags the surface is how a consumer learns the boundary
from a compile error instead.** The table itself is at the foot of this file.

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

### `hvec3_lerp` / `hvec2_lerp` — unpark the SIMD hybrid — **[3.2.0]**

⚠ **This was parked under "SIMD `cross`" and was never gated on the same thing.** `cross` needs lane
shuffles; `lerp` does not. Measured on 6.6.2 with the existing n=2-pair + scalar-tail hybrid the
other `hvec3_*` arithmetic already uses: **25 ns → 19–20 ns, bit-identical results, zero shuffles.**
⚠ Same over-read rule as every other `f64v_*` path here — the pair plus a scalar tail, never n=3.

### Consolidate onto stdlib `vec_sort_by` / `vec_select_nth` — **[3.2.0]**

Consolidation for consistency, **not for speed** — 2.6.15 already fixed the complexity of the two
hot sorts.

⭐ **The wait-for-the-third-instance gate is DISCHARGED and was discharged twice over.** The claim
that hisab has "exactly one hand-rolled sort" is wrong by 6x: there are **SIX ordering routines in
five files**, re-derived against the 3.0.0 tree on 2026-09-11 — `collision_core.cyr:530`
(`_col_sort_indices_by_xy`, heapsort), `spatial.cyr:114` (`_kd_select_median`, three-way
quickselect), `num_ext.cyr:309` (insertion sort, prime factors), `linalg_ext.cyr:1071`
(`eigen_symmetric`), `linalg_precision.cyr:1219` (SVD singular values) and
`linalg_precision.cyr:1753` (`eigen_qr` step 3). **The last three are near-identical
descending-magnitude selection sorts** whose own comments concede they have already disagreed
once.

⚠ **This entry has been wrong twice, in opposite directions, both times by not running anything.**
It first read "and Cyrius has no closures" — false since v6.3.8, propagated to four files. It was
then corrected to a *measured* block: on 6.5.16 a capturing closure SIGSEGVed when passed through a
function and called there. **That was fixed in 6.5.17, re-verified on 6.5.18, and re-verified again
on 6.6.2**, so the closure block is gone too.

What actually remains is the plain API mismatch: `vec_sort_by` invokes its comparator as
`fncall2(cmp, elem_a, elem_b)` — element *values* — whereas hisab sorts *indices* by dereferencing
each into a separate `points` vector. A capturing comparator can now close over `points`, so this is
doable. ⚠ **Re-derive every line number above before acting on them.** All six had drifted — the
citations this row carried before 2026-09-11 pointed at `collision_core.cyr:466`,
`linalg_ext.cyr:905`, `linalg_precision.cyr:816` and `:1265`, none of which is an ordering routine
today — and a remediation instruction whose citations have drifted sends the next reader to the
wrong line, which is worse than giving no citation at all.

---

## Toolchain, tracked upstream

⚠ **Cyrius bugs are filed in the CYRIUS repo** — `cyrius/docs/development/issues/` is where the
language agent reads them — **and closed in THIS repo too when a bump fixes them**, because the
cyrius agent never edits hisab (both 2026-09-11 filings were fixed in 6.6.3 and closed here in
3.0.1: `issues/archived/2026-09-11-cyrius-*`). **Four hisab-filed toolchain items are open:**

| filing (upstream, `2026-09-13-hisab-…`) | what it costs hisab today |
|---|---|
| `private-fn-reachable-via-address-of.md` | **The 4.0.0 boundary is bypassable**: `&_helper` from another file compiles and runs. The public-surface gate uses calls, never `&name`, for exactly this reason. |
| `public-enum-leaks-onto-next-declaration.md` | `_ad_pow` and `_SYM_EPS` are public regardless of their markers; `_ad_pow` is allowlisted in the gate and the gate FAILS when upstream fixes it (remove the entry then). |
| `refresh-only-overwrites-released-snapshot.md` | `~/.cyrius/versions/<pin>/lib` is not evidence of what a pin ships; the vendoring check compares against the cyrius TAG. |
| `deps-relocks-silently-under-unchanged-pin.md` | A bare `cyrius build` can re-vendor and re-lock a stdlib file with no diagnostic; treat any `git status` change to `lib/` or `cyrius.lock` after a build as a defect to investigate. |

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
  ⚠ `lerp` was never gated on this at all and is **unparked above**.
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
✅ **Verified 2026-09-09**: 0 hits for `abaco` in `cyrius.cyml`, `cyrius.lock`, `dist/hisab.deps`,
`dist/hisab.cyr` and `src/`. The only git dep is sakshi (2.5.2 as of 3.0.1), and `deps --verify` already fails any
unreviewed dep — **no new gate is owed here.** ⚠ The apparent contradiction between
`eval("sin(pi/4)")` being abaco's while hisab exposes `expr_eval` is not one: `src/symbolic.cyr:324`
takes a **tree**, not a string. hisab has no tokenizer at all.

⚠ **This table is stale and refreshing it is an open item above.**
