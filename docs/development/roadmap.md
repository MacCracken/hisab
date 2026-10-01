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

## Current — v3.3.0

Suite **4510** across five harnesses (hisab 585, foundation 429, modules 2319, edge_cases 266,
abuse 911), constant gate **163/163**, public surface **704** declarations (847 gate probes),
enforced since 3.3.0 by the bundle's `private` marker. **80** benchmarks, **35** math modules in `[lib]` plus the
`src/visibility.cyr` marker, toolchain **6.6.12**, sakshi **2.5.6**, ganita **1.2.9**. All gates
green; per-release detail is in [`CHANGELOG.md`](../../CHANGELOG.md). Two releases broke the API,
and each has a consumer guide:
- **3.0.0** moved to `Result<T, E>` with no deprecation window
  ([`../guides/migration-3.0.md`](../guides/migration-3.0.md)). **2.24.0 is the supported 2.x line.**
- **3.3.0** made the bundle private and removed the accidental public names
  ([`../guides/migration-3.3.md`](../guides/migration-3.3.md)).

⚠ **hisab ≥ 3.1.0 requires cyrius ≥ 6.6.3** (`public struct` + `#derive`). Measured again on the
3.3.0 bundle from pinned dirs: 6.6.2 refuses it with the known `#derive` error. Under 6.6.3, 6.6.6,
6.6.10 and 6.6.12 a consumer-shaped program runs correctly, and all 511 non-public probes are
refused. ⚠ On 6.6.3 a private fn is still reachable through `&name` (upstream, fixed in 6.6.4).

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

No versioned item is open. 3.3.0 shipped the `private` flip; its record is in `CHANGELOG.md`.

---

## Toolchain, tracked upstream

⚠ **Cyrius bugs are filed in the CYRIUS repo** — `cyrius/docs/development/issues/` is where the
language agent reads them — **and closed in THIS repo too when a bump fixes them**, because the
cyrius agent never edits hisab (the pattern: a hisab-side record in `issues/archived/` carrying the
paired measurement, written on the bump that closes it).

**No hisab-filed cyrius item is open.** The 2026-09-14 register-picker filing closed in 3.2.1 (fixed
in cycc 6.6.5; `m3_mul_vec3` keeps its hoisted z tail because eight of the ten live consumers still
pin below the fix — 6.6.2 or 6.6.3 — and the comment says so). The 2026-09-21 bench filing (`min`
printed above `avg`) closed in 3.2.2: fixed in 6.6.9, verified as a pair (repro exit 1 on 6.6.6, 0 on
6.6.9 and 6.6.12; hisab rows with `min > avg` 13/320 → 0/320), record in `issues/archived/`.

⛔ **ONE upstream item is OPEN, in ganita** — filed 2026-09-30 as
`ganita/docs/development/issues/2026-09-30-f64-atan2-signed-zero-and-nan.md` (+ a repro checking 12
rows of the C99 F.10.1.4 table by bit pattern; exits 5 on ganita 1.2.10), recorded here as
`issues/2026-09-30-ganita-atan2-signed-zero-and-nan.md`. `ganita_f64_atan2` picks its quadrant with
IEEE compares, so `atan2(-0, -1)` is +π (C99: −π), `atan2(±0, -0)` is 0 (C99: ±π), and
`atan2(NaN, ±0)` is −π/2. It became reachable here in 3.2.2, when cyrius 6.6.8 made `cx_conj` carry
a −0 into `cx_arg`. It is the unfinished half of ganita's own 2026-09-07 filing, which is marked
resolved. **When it closes**: the two `KNOWN DEFECT (ganita atan2)` tripwires in
`tests/edge_cases.tcyr` fail — flip them to −π and NaN, and archive the record.

**`bench_run` auto-batching (6.5.19)** — already in force. The **39 `bench_batch()` call sites are
deliberately unchanged**: they buy a FIXED window rather than escape the timer floor, which is still
worth having when comparing two runs at identical batch sizes. Re-evaluate only if a reason appears.

---

## Optional, demand-gated

- **Flip the `src/` modules `private` too, not only the bundle.** 3.3.0 made `dist/hisab.cyr`
  private through `src/visibility.cyr`, which is every consumer's view. The modules themselves stay
  unflipped, so a caller that `include`s `src/*.cyr` one at a time — today only hisab's own suites —
  sees no boundary. `check-public-surface.sh` claim 1 already proves no module reaches another's
  internals. ⚠ **Measured cost on 3.3.0**: under a full per-module flip the suites reach **68 private
  names at 390 sites** — `tests/modules.tcyr` 375 / 63, `tests/hisab.tcyr` 14 / 4, `tests/hisab.bcyr`
  1 / 1. Top by sites: `_f64arr_set` 125, `_f64arr_alloc` 27, `_col_dl_incircle` 24, `_f64arr_get`
  15, `_col_ghost_ux`/`_uy` 15 each. They are almost all deliberate white-box tests: Delaunay
  predicates, spatial node layouts, and the instrumentation counters mutation-proven guards read.
  Each would have to be promoted to public API or rewritten through public behaviour. **Driver: an
  à-la-carte consumer that includes `src/` directly.** None does today.

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
2026-09-30: **prakash** on hisab **3.2.1** (cyrius 6.6.6 since 2026-09-23; first past the `Result`
break on 2026-09-15, `num_fft` through the `var rt, rv = …` idiom); svara, naad, goonj on **2.22.1**;
dhvani, attn11, ghurni, prani, garjan, nidhi on **2.11.2**. Their cyrius pins are **6.6.3** (svara,
naad, ghurni, prani, garjan, nidhi), **6.6.6** (prakash), **6.6.10** (dhvani) and **6.6.2** (goonj,
attn11). `svara/src/spectral.cyr:246` calls `num_fft`.

**Four more repos carry a copy transitively**, vendored by `cyrius deps` through another dependency
and compiled under their own pins (read 2026-09-30 from each `lib/hisab.cyr` header): jalwa (2.11.2,
via dhvani, cyrius 6.6.3), ranga (2.11.2, via prakash, 6.6.9), shabda and shabdakosh (2.22.1, via
svara, 6.6.3). `ls ~/Repos/*/lib/hisab.cyr` lists all fourteen; the `[deps.hisab]` grep finds only
the ten direct ones.

⛔ **goonj and attn11 cannot take ANY hisab ≥ 3.1.0 until they move their cyrius pin**: `public
struct` + `#derive` is refused by cycc 6.6.2 (`#derive(...) applies to a struct or an enum`), measured
on the 3.1.0, 3.2.1, 3.2.2 and 3.3.0 bundles from a dir pinned to 6.6.2, while the 3.0.1 bundle
compiles there. Under 6.6.3, 6.6.6, 6.6.10 and 6.6.12 the 3.3.0 bundle compiles and a
consumer-shaped program runs correctly. ⚠ The pin matters to every consumer for three more reasons, because a consumer
compiles the bundle under its OWN cycc:
- `_cga_build_null_tbl` keeps its `if`-guard form and `m3_mul_vec3` its hoisted z tail, since the
  natural forms are silently wrong below 6.6.3 and 6.6.5 respectively.
- A zero produced by negation is −0 from **6.6.8** and +0 below.
- `dual_pow` is within 1 ulp from **6.6.10**. Below that it takes the older ganita's `pow`, 137 ulp
  at 0.9^1024 on 6.6.3 and 6.6.6. All of these were measured on the 3.2.2 bundle from pinned dirs,
  and re-measured unchanged on 3.3.0's.

⛔ **The remaining nine are still on 2.x, and the `Result` break's failure mode is silent**: a
`Result` in *argument* position degrades to its tag, and `Ok` tag = 0 = `HSB_ERR_NONE`, so
`assert_eq(f(...), HSB_ERR_NONE)` **builds clean and tests nothing**. Hand a migrating consumer
`scripts/check-result-migration.sh`, which exists for exactly that class and is mutation-proven. The
reached surface is small — **35 distinct public fns in 8 modules** (Boundary table) — so each port is
bounded.

⛔ **impetus, kiran, joshua, hisab-mimamsa and kana are NOT consumers** — they have no `cyrius.cyml`.
They are Rust repos needing a *port*, not a scheduling decision. aethersafha IS a Cyrius port now
(`cyrius = "6.6.11"`) but declares no `[deps.hisab]`
(verified 2026-09-09, and again 2026-09-14 for abaco: its README calls hisab a sibling, not a
consumer; the ten live pins re-read 2026-09-30). The table below is what they *would* use, kept because it is the planning
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
