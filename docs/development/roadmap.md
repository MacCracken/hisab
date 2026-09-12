# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.6.2**. Stdlib `ganita` (6.2.x math umbrella) provides dense
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

## Current — v3.0.0

Suite **4202** across five harnesses (hisab 550, foundation 413, modules 2086, edge_cases 239,
abuse 914), constant gate **159/159**, **78** benchmarks, **35** `[lib]` modules, toolchain
**6.6.2**, sakshi **2.5.1**, ganita **1.2.4**, and **zero** deprecated-alias call sites. All gates
green: `lint` 0 warnings and `fmt <file> --check` 0 drift across all 44 sources, `vet` 2 deps /
0 untrusted / 0 missing, `deps --verify` 31/31, `fuzz` 1/0, `coverage` 640/644 (99%) functions over
36/36 files, distlib in sync.

3.0.0 is the `Result<T, E>` migration — 48 functions, 182 `Ok`/`Err` returns, 18 `?` sites, 533 call
sites — and it is breaking. [`../guides/migration-3.0.md`](../guides/migration-3.0.md) is the
consumer-facing guide; **2.24.0 is the supported 2.x line**, and there is no deprecation window.

## How to read this file

⭐ **Every open item carries its target version in bold brackets** — `**[3.1.0]**` — or sits in a
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

### Public / private function surface — **[`pub fn` half: 3.1.0 · `private` flip: 4.0.0]** ⛔ BLOCKED UPSTREAM

hisab signals internal-vs-public by naming convention alone: a leading `_` means "internal" and
nothing enforces it. `geo_diff.cyr` reaches `geo.cyr`'s helpers across a module boundary because
nothing distinguishes API from implementation, and the 2.10.1 split had to be named
`geo_ray_aabb_face` rather than `_core` specifically so a cross-module call would not be reaching
for an underscore.

⭐ **The mechanism works and the boundary HOLDS — verified end to end, not assumed.** On a copy of
the tree, a consumer including `dist/hisab.cyr` can call the `public` API and **cannot** reach a
file-private helper: the build fails with `'_noise_fade' is private to its file`, exits 1, and emits
no binary. `distlib` passes `private` through verbatim and `dist/hisab.deps` is unaffected.
⚠ **Landmine**: `cyrius check --with-deps dist/hisab.cyr` stays **green** on a bundle no consumer
can call, because it compiles the bundle rather than calling into it. **A consumer-call gate has to
exist before the flip**, not after.

⛔ **It cannot ship because `#derive(...)` and `public` cannot be combined on cycc 6.6.2.**
`#derive(accessors)` above a `public struct` is a hard error — with NO `private` anywhere in the
file, and a control differing by exactly one keyword compiles. Without `public` on the struct its
generated accessors stay file-private, and privatising hisab produced **1,436 errors of the form
`'HVec3_x' is private to its file`** from the 18 modules that derive accessors — which are the
foundation types every other module touches. Filed upstream as
`2026-09-11-derive-cannot-combine-with-public.md`, cross-referenced to the existing
`#inline`-disarms-`#derive` filing, which produces the identical diagnostic and may share a root
cause. **The change is reverted rather than half-applied.**

**Sizing, measured rather than estimated** (a full cross-module reference scan, 2026-09-11): 939
functions, 296 underscore-prefixed, **148 functions AND 25 globals** crossing a module boundary, of
which **31 are underscore-named**. ⚠ The pre-3.0.0 version of this row counted 17 and did not count
globals at all. The tests reach **52 distinct `_` functions across 235 sites**.

**Split the work, because the halves have different blast radii:**
- **`pub fn` everywhere — non-breaking, verified a no-op, lands in a 3.x.** It does not need the
  major and should not wait for it. ⛔ Still blocked on the upstream `#derive`/`public` fix.
- **The `private` flip — breaking, so 4.0.0.** Marking a function private breaks anyone already
  calling it, which is why it was scoped onto 3.0.0 in the first place; 3.0.0 shipped without it, so
  it moves to the next major rather than into a minor.
- **The escape hatch is still a decision, not a marker.** Privacy is per FILE, so each of the six
  `X` → `X_ext` pairs forces a choice: mark the helper `pub` (promoting an implementation detail to
  public API), merge the pair into one file, or leave `X` public. Three options, six sites.

Safe order, once unblocked: **`pub fn` everywhere first**, then the consumer-call gate, then flip
`private` last.

### The struct-layout contract has no gate — **[3.1.0]**

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
that have nothing. Verify it fires before trusting it — install a one-field change and watch the
assertion fail.

### The Boundary-with-Abaco table lags the surface it describes — **[3.1.0]**

⚠ **It is frozen at the 2.2.0 surface.** It has no row for `expr_eval` — whose **domain changed** in
2.11.2 (`(-2)^3` returned NaN for hisab's entire history and returns a number now), and whose named
consumer is abaco — and none for autodiff (forward duals + the reverse tape), the six `geo_diff`
jets, CGA, or Lie. **A boundary table that lags the surface is how a consumer learns the boundary
from a compile error instead.** The table itself is at the foot of this file.

### Get one live consumer onto 3.0.0 — **[unversioned — external]**

⚠ **No live consumer has built 2.11.3 or later.** All ten sit at 2.11.1/2.11.2, behind the
6.5.33 → 6.6.2 toolchain bump, the 536-site ganita alias migration, **and now the `Result<T, E>`
break**. `cyrius check --with-deps dist/hisab.cyr` proves the bundle compiles against **this**
manifest's ganita — not against theirs, and not against a real call graph.

⛔ **The `Result` break makes this urgent rather than merely overdue**, because of the failure mode
the migration guide leads with: a `Result` in *argument* position does not error, it degrades to its
tag, and `Ok` tag = 0 = `HSB_ERR_NONE`. A consumer whose checks look like
`assert_eq(f(...), HSB_ERR_NONE)` will **build clean and test nothing**. Hand them
`scripts/check-result-migration.sh`, which exists for exactly that class and is mutation-proven.
**Highest-value action in this section.**

### `hvec3_lerp` / `hvec2_lerp` — unpark the SIMD hybrid — **[3.1.0]**

⚠ **This was parked under "SIMD `cross`" and was never gated on the same thing.** `cross` needs lane
shuffles; `lerp` does not. Measured on 6.6.2 with the existing n=2-pair + scalar-tail hybrid the
other `hvec3_*` arithmetic already uses: **25 ns → 19–20 ns, bit-identical results, zero shuffles.**
⚠ Same over-read rule as every other `f64v_*` path here — the pair plus a scalar tail, never n=3.

### Consolidate onto stdlib `vec_sort_by` / `vec_select_nth` — **[3.1.0]**

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
language agent reads them. Only hisab's own items, and hisab's record of a live upstream workaround,
belong in this repo's `issues/`. **Two hisab-filed toolchain items are open:**

| filing (upstream) | what it costs hisab today |
|---|---|
| `2026-09-11-derive-cannot-combine-with-public.md` | **Blocks the public/private item above outright.** `#derive(accessors)` + `public struct` is a hard error; 1,436 privacy errors from the 18 deriving modules if applied anyway. |
| `2026-09-11-nested-continue-binds-to-wrong-loop.md` | When a loop and a loop nested inside it both contain a `continue` and the outer appears lexically FIRST, both bind one level too far out: the inner jumps to the OUTER latch, and the outer `continue` becomes a **no-op**. The natural sparse-skip idiom is both broken shapes at once and produced an **all-zero 1024-entry CGA table while reporting success**, every helper correct in isolation. ⚠ **`src/geo_advanced.cyr` is written around this** — moving a `continue` below a nested loop is what makes the same program correct. Do not "tidy" those loops until this closes. |

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
`dist/hisab.cyr` and `src/`. The only git dep is sakshi 2.5.1, and `deps --verify` already fails any
unreviewed dep — **no new gate is owed here.** ⚠ The apparent contradiction between
`eval("sin(pi/4)")` being abaco's while hisab exposes `expr_eval` is not one: `src/symbolic.cyr:324`
takes a **tree**, not a string. hisab has no tokenizer at all.

⚠ **This table is stale and refreshing it is an open item above.**
