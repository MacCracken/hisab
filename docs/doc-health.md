---
name: Hisab Documentation Health
description: Living state of doc currency in the hisab repo — fresh / stale / read-through / dated / tracked-issue, refreshed as docs are touched
type: state
---

# Documentation Health — hisab

> **Last refresh**: 2026-09-13 — **v3.1.0 RELEASED: the `pub fn` half.** 729 declarations carry
> `public` (a documented no-op — binaries byte-identical to 3.0.1), and `scripts/check-public-surface.sh`
> flips all 35 modules `private` in a scratch copy on every CI run: five claims, four mutants killed.
> ⛔ Its first draft checked the flipped BUNDLE and was vacuous (one file → `private` inside it can
> never fail); that fact halves the 4.0.0 flip — a consumer already sees exactly the 729 items, the
> whole remaining cost is the suites: **64 names / 387 sites**, measured. ⛔ Two more upstream holes
> filed with self-proving repros (`&_private` runs from another file; `public enum` leaks onto the
> next declaration — `_ad_pow` is the gate's allowlisted known leak, inverting on the fix). ⛔ Two
> gates went blind to `public` (constants: green 143/143 over 160, now floored; result-migration:
> broken both ways AND never wired into CI since 3.0.0). A 7-reviewer / 3-refuter pass (53 findings,
> 0 refuted) found `#must_use` on the wrong function since 2.17.0, three benchmark rows discarding a
> `Result`, and a test passing 24-byte `HVec3` into 72-byte `Mat3` readers — all repaired, mutation-
> proven. Suites **4214**. The 24 cross-module dispositions are worked and on the roadmap.
>
> **Previous refresh**: 2026-09-13 — **v3.0.1 RELEASED: cycc 6.6.3 — both 2026-09-11 filings closed,
> and the pin's snapshot was not the pin.** Toolchain 6.6.2 → **6.6.3**, sakshi → **2.5.2**, ganita →
> **1.2.5** (each a one-header-line refold), suites and CLI **byte-identical** under both compilers,
> 4202/4202, lock 31/31 now sorted. ⭐ Nested-`continue` and `#derive`+`public` are fixed and **closed
> in this repo's own ledger** (`issues/archived/2026-09-11-cyrius-*`), each verified as a pair from
> dirs pinned to each version — the cyrius agent never edits hisab, so an upstream fix stays "blocked"
> here until hisab closes it. Roadmap's public/private row: BLOCKED → **UNBLOCKED**. ⛔ The installed
> "6.6.2" snapshot held 6.6.3's stdlib and a bare `cyrius build` silently re-vendored + re-locked
> ganita under the unchanged pin — **both halves filed upstream with a self-proving repro**; the
> vendoring check now compares against the cyrius **tag**, never the install dir. No performance
> change, as a fact: byte-identical binaries, bench median +0.68%, 0/78 past 10%.
>
> **Previous refresh**: 2026-09-11 — **v3.0.0 RELEASED: `Result<T,E>`, and two roadmap premises that
> measurement refuted.** 48 functions, 182 `Ok`/`Err` returns, 18 `?` sites, 533 call sites.
> ⛔ The premise it was planned on was **false** — `?` on a plain i64 fn fails at compile time, it
> does not silently SIGSEGV. ⛔ And the hazard that mattered was never in the row: a `Result` in
> **argument** position degrades silently to its tag, and `Ok` tag = 0 = `HSB_ERR_NONE`, so **98
> test lines would have gone vacuous while still green**. Driven by a grep with a mutation-proven
> gate. ⛔ **Deprecation window declined** on the migration's own evidence. ⛔ **Public/private
> surface blocked upstream and reverted** — the boundary holds, but `#derive` and `public` cannot
> be combined on 6.6.2 (1,436 errors from 18 modules); filed upstream.
>
> **New**: `docs/guides/migration-3.0.md` (the four call shapes, the `?` rules, the
> argument-position trap, and why there is no deprecation window) and
> `scripts/check-result-migration.sh`.
>
> ⭐ **The 2.x roadmap is closed and 3.0.0 has shipped.** One item remains open in the whole file,
> and it was blocked on a toolchain fix rather than on work — unblocked as of 3.0.1.
>
> **Previous refresh**: 2026-09-10 — **v2.20.0 RELEASED: the four repairs 2.19.0 measured, and what
> measuring them again showed.** Each of the four filings was wrong about the size, the direction, or
> the safety of the obvious fix — so two became repairs and two became characterisations. Above 2^63
> BOTH symbolic renderers returned one wrong constant for every input; a conformal point is never null
> in BOTH tails, with no arithmetic fix possible; the subnormal-SVD claim was false in both halves (270
> of 416 rows report success, 57 right); and a CHANGELOG sentence had a divergence direction backwards
> for thirteen releases, in the one place no gate could reach. ⛔ **Adding a benchmark changed a
> different benchmark by 31%** — unchecked, a 44% phantom speedup in an untouched module.
> ⚠ Four of my own instruments were wrong before the thing measured was.
>
> **Previous refresh**: 2026-09-10 — **v2.19.0 RELEASED: the 3.0.0 prep, and five gates that could not
> fail.** `#must_use` is a COMPILER diagnostic, not a cyrlint one, so 44 annotations matched CI's
> `^  warn ` grep **zero times** until a step was wired. Two error-code mutants survived **all 3940
> assertions** — one making `HSB_ERR_ALLOC` collide with `HSB_ERR_INVALID_TRANSFORM` — because every
> assertion compares a return against a code BY NAME, so the value cancels out. The release shipped its
> own broken provenance marker, caught by a gate wired PR-only in a repository with **0 pull requests
> against 303 workflow runs**. And the version gate compared committed text to committed text and never
> ran the binary. ⛔ **Four of my own instruments were wrong before the thing measured was**, including
> both replacement version guards, wrong in OPPOSITE directions: one failed on its own tree because the
> comment explaining the change quotes the retired literal, the other was vacuous because it matched
> that same comment. **A gate that cannot tell code from a comment quoting code fires on every honest
> explanation of itself.**
>
> **Previous refresh**: 2026-09-10 — **2.19.0 bites 3-4 (unreleased): the release's own defect, and a
> gate that had never run.** The enum bite shipped a `[measured: ...]` marker wrapped across two lines
> — malformed to the single-line regex — and the gate that caught it, `check-measurements.sh`, is wired
> `if: github.event_name == 'pull_request'` in a repository with **0 pull requests and 303 workflow
> runs, the last 100 all `push`** (checked against the GitHub API). ⛔ Its `--diff` mode was also blind
> to its own dominant case: a path failure was filed against the PARAGRAPH's first line, so a bad
> marker appended to an EXISTING block was intersected away — and it got the gate GREENER, because the
> fabricated marker SATISFIED the real claim beside it. Both fixed; broken markers tree-wide **3 -> 0**.
> ⚠ **This ledger's own header was two releases stale** (v2.16.0) while four of its rows already read
> v2.18.0, and bite 3 touched four docs without refreshing a row — the per-touch rule, not per-release.
>
> **Previous refresh**: 2026-09-10 — **v2.16.0, the small-angle series — and the defect the guard was
> hiding.** `su2_log` and `so3_log` recovered the angle with `acos` of a value that rounds to
> EXACTLY 1.0 below theta ~ 1.5e-8, so the whole rotation was lost: measured, exactly 0 from 2^-28
> down. The guard was not the defect; it was what made the defect look like a design choice.
> ⭐ `atan2` recovers it bit-exactly and the round-trip floor moves **2^-26 -> 2^-537**.
> ⭐ The coefficients split three ways: one needs no series at all (a half-angle identity), two need
> Taylor forms below a MEASURED crossover of 0.1. ⛔ A mutant that would not die found a third
> defect — both log maps squared before summing, flushing the norm to zero below ~2^-511.
>
> **Previous refresh**: 2026-09-10 — **v2.15.0, the epsilon tier closed: 73 of the remaining 74.**
> 81 mutants installed, **73 killed**; 8 survivors and 1 deferral recorded rather than tidied away.
> ⭐ The census's CLASS held, but the REPAIR is not one rule: an unbounded numerator needs DBL_MIN,
> a bounded one needs an exact zero, a convergence test divides nothing and becomes relative,
> sparsity is structural, and a sort comparator with a tolerance is not an order at all.
> ⛔ `cx_div` — named in `complex.cyr`'s own comment since 2.6.14 — proves no threshold can work:
> over 401 decades, removing the guard ENTIRELY still fails 89, because `br²+bi²` is destroyed
> before any comparison runs. Smith's algorithm fails 0. ⛔ Six of my own fixtures were wrong before
> the code was, and a reachability probe returned the right answer **by accident** — the real
> finding being that the existing assertions were weak enough for a constant to pass them.
>
> **Previous refresh**: 2026-09-09 — **v2.14.0, the epsilon release: the tier was "~20 sites" and it is
> 97.** A census of every `EPSILON_F64` comparison guard in `src/` — **136 guards in 25 modules**,
> each classification independently re-derived by a second agent told to refute it, which disagreed
> on **5 of 102**. ⚠ The "~20" came from the 2026-08-11 audit's *confirmed* table, which lists
> instances someone reproduced, not a class — the ganita-migration mistake (8 estimated, 536 actual)
> a second time. **23 repaired, 58 mutants all killed, 74 enumerated and open.** ⛔ 2.10.2's own
> repair was incomplete: `geo_triangle_unit_normal` guards a **degree-four** quantity with a
> degree-one threshold, and the 10-decade sweep that certified it bottomed out at 1e-9. Sweep depths
> are now **derived** — 154 decades for `hvec3_angle`, 80 for `_sc_sweep`. ⭐ Three operations need
> three different exact guards (`1/sqrt(x)`, `1/x`, and an overflow-only test where `abuse.tcyr` pins
> a **NaN in, NaN out** contract). ⛔ And the mutation harness read a compiler *note* as a test result
> and reported four survivors as kills. **Check the probe before believing the probe.**
>
> **Previous refresh**: 2026-09-09 — **v2.13.0, the suite release: the suite could not see an error of
> 0.9.** On the 2.12.0 tree `hvec2_add` could return `a + b + 0.9` and **all 3572 assertions passed**.
> `f64_to` truncates, so `assert_eq(f64_to(v), 3)` accepts every v in [3, 4); **842 of 3105 assertion
> sites (27.1%)** compared floats that way and now **16 (0.5%)** do. `foundation.tcyr` 89% -> **0%**.
> ⭐ The migration was **self-verifying** — a bit-exact compare fails at any site that was not already
> exact — so 809 conversions became a search, and the four that failed were four different real
> defects that had been passing for the life of the file. Named tolerance helpers added to all five
> suites. ⚠ Two mutation counts were nearly published wrong when the harness's mutation silently
> failed to apply and returned `0 failing`. **Prove the mutant is installed before believing it.**
>
> **Previous refresh**: 2026-09-09 — **roadmap verification sweep: 18 of 39 items did not survive
> contact with the tree.** Every open roadmap item was handed to an adversarial verifier told to try
> to prove it already done — **21 genuinely open, 15 stale premise, 3 finished**. Report:
> [`audit/2026-09-09-roadmap-verification.md`](audit/2026-09-09-roadmap-verification.md).
> ⛔ **The gate blocking the entire release train was unexecutable**: 2.12.0 was gated on "re-run the
> 28 unverified audit findings", and **the 28 were never enumerated** — the report records them only
> as a count. Retired; the release train is rewritten around a **safety release** (abort + allocation
> tiers) because every epsilon repair is a sub-1.0 threshold change that 843 truncating assertions
> cannot see.
> ⛔ **`sequential_impulse` writes ~16 KB past a 64-byte buffer and returns rc = 0** — reproduced
> with a canary; the abuse suite's own canary block for that function misses it because all three
> `n` values it tries are in bounds.
> ⚠ **Two contracts quoted as protection do not protect**: the struct-layout "32 assertions trip a
> gate" are all `sizeof > 0` / `% 8 == 0` and pass identically on both sides of the change they were
> written to catch; and the roadmap's own header said toolchain 6.5.33 fourteen lines above a line
> saying 6.6.2.
> ⭐ **cycc 6.6.2 already ships and enforces `private`/`pub`**, so four v3.0.0 "decisions" are moot —
> and **ten repos consume hisab today** while the roadmap said none were live.
>
> **Previous refresh**: 2026-09-09 — **v2.11.5, cycc 6.6.2: the filed wrong-code bug is repaired.**
> Toolchain 6.6.1 -> **6.6.2**, no hisab behaviour change, suite **3538**, bundle **905,803 B /
> 23,405 lines**, lock 30 -> **31**. hisab's own reproducer exits 0 on 6.6.2 and 139 on 6.6.1, and
> the three suites that used to SIGSEGV pass with the `m4_mul_vec4` hoist REMOVED — kept now only
> for consistency with `m3_mul_vec3`, with its comment corrected so it no longer claims to be
> load-bearing.
> ⛔ **The filing was wrong about its scope twice** — filed as derive-specific and as a 6.5.71
> regression, it was neither; all 21 SIMD handlers were affected, reachable since 6.0.70.
> **A first-bad-version is evidence about visibility, not origin.**
> ⚠ **And `src/vec4.cyr` had documented a real rule since 2.3.1 with the wrong cause attached** —
> top-level SIMD crashes because the handlers use FRAME slots, not because of SSE stack alignment.
> Both corrected. Open filings unchanged at **1**.
>
> **Previous refresh**: 2026-09-09 — **v2.11.4, off the deprecated ganita aliases.** All **536** call
> sites over 20 of ganita 1.2.4's 53 deprecated `mat_*`/`f64_*` names moved to the `ganita_*`
> spellings, `dist/hisab.cyr` included. Suite **3532**, byte-identical output across the change.
> ⚠ **2.11.3 estimated this at 8 sites** — scoped from ganita's changelog paragraph rather than its
> surface, wrong by 67x. **Scope a migration from the dependency's surface, never its release notes.**
> ⭐ The aliases were real `call`/`ret` pairs (`#inline` needs <= 2 params; `ganita_mat_get`/`_set`
> take 3 and 4): `svd_golub_kahan_12` **-27.85%**, `eigen_qr_12` **-27.67%**, against a same-binary
> noise floor measured in the same session (median 2.10%, worst 9.13%).
> ⚠ **Two assertions had gone VACUOUS** — they compare through a tolerance and a round, so when
> upstream repaired the defect they pinned they kept passing while their stated property evaporated.
> **An assertion written against a defect cannot fail when the defect is repaired.** Open filings
> **2 -> 3**: `_ad_pow`'s rationale has now been invalidated twice and no test exercises the one
> regime it still wins.
>
> **Previous refresh**: 2026-09-09 — **v2.11.3, the toolchain catch-up that hit a wrong-code
> regression.** cyrius **6.5.33 → 6.6.1** (crossing a minor), sakshi 2.4.11 → **2.5.1**, ganita
> 1.1.4 → **1.2.4**. Suite 3526 → **3532**, constant gate **159/159**, bundle **899,733 B /
> 23,331 lines**, 72 benchmarks, open issues **5 → 2**.
> ⛔ **The bump miscompiles hisab and the bug is upstream's**: a `#derive(accessors)` getter passed
> to an `f64v_*` intrinsic leaves the intrinsic's destination stack slot unwritten, so three of five
> suites SIGSEGV'd. Bisected to cycc **6.5.71** (6.5.70 clean) and **filed in the cyrius repo**, not
> here — that is where the language agent reads them. Worked around by a load-bearing accessor hoist
> in `src/mat4.cyr`, mutation-proven both ways.
> ⚠ **Two tests were failing because upstream FIXED the defects they had pinned** — `f64_pow(-2,4)`
> 15 → bit-exact 16, and `cx_exp(-inf)` NaN → exactly +0. A suite that pins a dependency's bug as
> expected behaviour fails on the repair, not on a regression.
> ⚠ **And three numbers in this repo were wrong about what they even measured**: `cyrius.cyml`'s
> ceiling comment named `_SRC_CAP` when the limit that gates a consumer build is the *expanded
> source* cap (8 MB → 24 MB) with a *token* cap (1,048,576 → 4,194,304) that can bind first; the EPA
> filing's "+19.4%" was priced against a baseline that moved 125.6 → 78.5 us; and **41 of 72
> benchmarks report a `net` below the timer floor they subtract**, so `ray_aabb`'s −55.9% this
> release is evidence of nothing. **A constant derived from a dependency is a measurement** — third
> consecutive release saying so.
>
> **Previous refresh**: 2026-08-21 — **v2.11.2, the toolchain catch-up.** cyrius **6.5.18 → 6.5.33**
> (fifteen releases), sakshi 2.4.10 → **2.4.11**, ganita 1.0.4 → **1.1.4**. Suite 3514 → **3526**,
> constant gate **159/159**, coverage **640/644** over 36/36, bundle **898,472 B / 23,311 lines**.
> ⚠ **Every finding was a stale number, and this file is the ledger that exists to catch them** —
> `cyrius.cyml` and `CLAUDE.md` both documented a **1 MB `input_buf`** that has been **16 MB since
> 6.5.22**, `tests/hisab.bcyr` cited a ~240 ns clock cost upstream retired as 11-14x wrong, and
> `.github/workflows/ci.yml` printed a remediation command that **truncates the file to 0 bytes**
> under the 6.5.28 `fmt` semantics. None was caught by a gate; all three were caught by re-deriving
> a figure against its source. **A constant derived from a dependency is a measurement.**
>
> **Previous refresh**: 2026-08-11 — **v2.11.1, the audit release.** A full P(-1) sweep of the 2.11.0
> tree: 6 dimensions, 115 checks, **52 findings reproduced**, 21 CONFIRMED by an independent skeptic,
> 2 REFUTED, 1 already known. Suite 3507 → **3514**, constant gate 158 → **159**, coverage
> **640/644**, bundle **895,768 B / 23,291 lines**. Report:
> [`audit/2026-08-11-v2.11.0-full.md`](audit/2026-08-11-v2.11.0-full.md).
>
> ⚠ **28 of the 52 were never verified, and that is recorded in the report rather than buried.** The
> verify phase was capped at 24 by the harness, not by judgement; 2 of the 24 checked came back
> REFUTED, so an unverified finding is roughly 1-in-12 wrong. They are **reproduced, not proven**.
> That cap is the audit's own biggest process defect.
> ⛔ **The roadmap USED to gate every downstream tier on re-running them; that gate was RETIRED
> 2026-09-09.** The 28 are **not enumerated anywhere** — the report records them only as a count —
> so the task had no input, and the tiers it blocked draw from the *confirmed* set, which is disjoint
> from them. See [`audit/2026-09-09-roadmap-verification.md`](audit/2026-09-09-roadmap-verification.md).
>
> **24 of the 52 are one defect.** A guard compares a quantity against a threshold that is wrong for
> it, and the function then **fabricates a plausible answer**: `eigen_qr` wrong eigenvalues with
> `rc = 0`, `cqr_decompose` an R that is not upper triangular with `rc = 0`, `hvec3_angle` 0 rad
> ("parallel") for exactly perpendicular vectors, `m3`/`m4_inverse` the identity. **This is the ninth
> through thirty-second instance of a class first written down in `complex.cyr:58` in 2.6.14 — and
> `cx_div` is *in that comment* and still fabricating zero.** Writing a lesson beside the code that
> taught it does not fix the code and does not reach the other thirty-four modules; only a grep does.
>
> Four repairs executed here, each mutation-proven: `hquat_inverse`/`hquat_normalize` (squared
> magnitude vs unsquared `EPSILON_F64` → DBL_MIN), three caller-sized allocations in `num_ext`
> (reproduced as **SIGSEGV**, now `HSB_ERR_ALLOC`), `ad_tape_new` (a non-zero handle with a null body
> — 2.11.0's own code), and `optimize.cyr` + `threat-model.md`'s `ALLOC_MAX`-derived ceiling
> (5792 → **16384**).
>
> ⚠ **A stale comment in one file nearly buried a real defect in another.** The first
> `num_tridiag_solve` probe was sized from `optimize.cyr`'s own `ALLOC_MAX = 256 MiB`, landed
> *exactly* on the limit, allocated successfully, and the finding looked refuted — cyrius 6.4.51
> raised it to 2 GiB and `lib/alloc.cyr:169` says so outright. **A constant derived from a dependency
> is a measurement**: it goes stale silently and nothing in the build says so.
>
> ⚠ **The quaternion sweep's own first pass killed 1 of 3 mutants.** Written over 12 decades, it
> bottomed out *above* both thresholds a repair might plausibly have chosen; 20 decades kills all
> three. Same failure mode as 2.10.1's, and it recurred despite being written down. A second trap in
> the same fixture was avoided deliberately: asserting **unit length** does not discriminate, because
> the fabricated identity *is* unit — the assertion checks **direction**.
>
> **The audit's third critical is this repo's own test suite**, and it explains the rest: **836 of
> the 3510 assertions the audit's own scan counted (23.8%) compare through `f64_to`, which TRUNCATES**, so any error below 1.0 is
> invisible to a quarter of the suite; **38.4% of value-changing single-operator mutants survive all
> five suites**; 29 public functions are covered only by `assert_neq(f(...), 0)`. That is the measured
> reason 3507 assertions and 99% coverage saw none of the 52 findings.
>
> **Previous refresh**: 2026-08-11 — **documentation cleanup pass**, no source change.
>
> **`roadmap.md` 642 → 289 lines.** Every per-release narrative section was deleted: 2.6.12–2.6.15,
> 2.7.0, 2.9.0–2.9.3, 2.10.0–2.10.2 and 2.11.0 were each restating what `CHANGELOG.md` already
> holds and what the Release History table already indexes. What replaced them is a single **Open
> items** section — everything still owed, in one place, each entry carrying *why* it has not been
> done, because "deferred with a reason" and "forgotten" are indistinguishable once the reason is
> lost. The standing lessons of the arc were kept and consolidated into `Current`, since those are
> the part of a release record that does not belong in a changelog.
>
> ⚠ **Deleting sections surfaced stale claims that had been invisible inside them.** Two open
> checkboxes had already shipped (the aabb/obb/capsule jets, in 2.10.1). The `vec_sort_by` entry
> said it "unblocks the moment that is fixed upstream" — it *was*, in 6.5.17, re-verified here on
> 6.5.18; that entry has now been wrong twice in opposite directions, both times by not running
> anything. The toolchain paragraph claimed "all four remaining open filings are cyrius items":
> there are **three**, and one of them is hisab's own. And it said CI "tolerates exactly that
> signature", a tolerance removed in 2.10.0.
>
> **`README.md` was stale in 14 places** — version, toolchain pin, bundle size, CLI size, three test
> counts, the benchmark count twice, the consumer-facing `tag`, and both the Geometry and Autodiff
> module rows (which still described 2.10.0's three jets and forward-mode-only autodiff). All
> re-measured against the tree rather than carried forward.
>
> ⚠ **`geo_diff` was missing from `architecture/overview.md` entirely** — absent from the module map
> *and* from the dependency table, since it shipped in 2.10.0. Its row was established the way that
> table's own contract requires, **by building**: `error` `geo` `quat` `vec3`, with each of the four
> shown load-bearing by dropping it and watching the build fail. ⚠ `cyrius check` is **not** a
> minimality oracle here — it tolerated undefined functions in unreachable code (770 of them), so
> the candidate sets had to be exercised by a `main` that actually calls the jets.
>
> **20 broken issue links repointed** at `archived/`, across `CHANGELOG.md`, `doc-health.md`,
> `dependency-watch.md`, `threat-model.md` and an audit report — every one caused by a filing being
> archived after the link was written. Five remaining references resolve to the *cyrius* repo and
> are correctly prefixed as such. `CONTRIBUTING.md` lost a warning block telling contributors to
> expect a `distlib` failure that 6.5.17 fixed. `SECURITY.md`'s division-by-zero row now records
> that **the presence of a guard is not the same as a correct one** — four of the autodiff guards it
> cited were fabricating answers until 2.11.0.
>
> **Earlier**: 2026-08-11 — **v2.11.0, reverse-mode autodiff.** Tape-based: one sweep for an
> n-input gradient where forward-mode duals need n passes, measured at **11.2×** on 16 inputs.
> Suite 3469 → **3507**, benchmarks 70 → **72**, constant gate 157 → **158**, coverage **640/644**.
>
> **Fourth release running, the feature's own oracle found defects in the code it was going to be
> checked against.** Reverse mode is validated *against* forward mode, so `autodiff.cyr` was swept
> before a line of tape code existed. Five defects: `1/1e-13` returned `(0,0)` where the truth is
> 1e13; **`ln(−5)` returned a NaN value beside a CONFIDENT −0.2 derivative** — worse than either
> alone, because a caller checking only the gradient sees nothing wrong; `sqrt(−4)` was `(NaN,NaN)`
> because the guard ran *after* the sqrt; `d/dx x³ at −2` was NaN because `f64_pow` **was**
> `exp(n·ln(base))` and rejected negative bases. ⚠ That last was the stdlib's **stated**
> implementation — read before filing, unlike 2.10.2's near-miss — so the defect was hisab
> inheriting it undocumented.
>
> ⚠ **UPDATED 2.11.2: that stdlib fact is no longer true.** ganita 1.1.4 (cyrius 6.5.33) fixed
> `f64_pow` upstream — zero base, zero exponent and negative-base-with-integral-exponent are
> special-cased now, so `f64_pow(-2,3)` is a number and `f64_pow(0,0)` is 1. The paragraph above
> is kept as the record of what 2.11.0 faced, in past tense. hisab's own repeated-multiplication
> path survives on **precision**: the upstream fix takes its magnitude from `exp(n·ln|base|)`, so
> `f64_pow(-2,4)` truncates to **15** where hisab's gives exactly **16**.
>
> ⚠ **THE FINITE-DIFFERENCE SWEEP ALONE FOUND NONE OF THEM**, and this is the release's standing
> lesson. At every input where a guard fires, the perturbed **scalar** is NaN too, so the sample is
> skipped and the guard is never asked. A 13-op sweep reported clean while skipping exactly the rows
> that mattered. **A guard has to be interrogated directly; differencing a function cannot reach the
> branch that refuses to evaluate it.** The suite now carries both, plus a *control* pinning that
> `(−2)^0.5` must still be NaN — without it, "fixed the NaN" and "replaced one fabrication with
> another" are indistinguishable.
>
> **Three things this release got wrong and caught by measuring:**
> * **An assertion demanded more than the contract promises.** The first end-to-end optimizer test
>   asserted the minimiser to 1e-8 and failed 4 of 8 — gradient descent stops at `‖g‖ < 1e-6` and
>   therefore lands 5e-7 away, *exactly as promised*. The tolerance is now derived from the
>   contract. The other three solvers hid it by converging far tighter.
> * **Two mutants survived the first test pass**, both real gaps: the `ids` indirection was untested
>   because the fixture's variables happened to *be* nodes 0 and 1, and `ad_grad_into`'s error check
>   was untested because nothing called it with a bad root.
> * The FD sweep's first draft reported **two mismatches that were the fixture's fault** — an
>   absolute `h = 1e-6` straddles `abs`'s kink and `1/x`'s pole at `x = 1e-8`.
>
> **And a clean result, recorded because it counts too**: the solvers' own contract check —
> `‖grad‖ < tol` at `out_x` on a 0 return — came back with **0 violations across 16 runs**, the
> fifth release that check has been run and the first to find nothing.
>
> **Previous refresh**: 2026-08-11 — **v2.10.2, the primal defects the jets were sitting on.** All
> three that 2.10.1 filed, plus the OBB rotation partial it deferred (12 → **15**). Suite 3453 →
> **3469**, constant gate 156 → **157**, coverage **618/621**.
>
> **One rule settled four thresholds**, and it is the release's whole content: *guard exactly what
> makes the DIVISION fail and nothing more* — `_GEO_F64_TINY` = DBL_MIN. **A threshold that needs a
> scale chosen for it is the defect.** Every guard repaired here compared a geometric quantity
> against a tolerance picked for a different quantity: a box spanning [0,1] hit at **x = 1.4** by a
> **unit** direction, a capsule hit point **a full radius inside the solid**, a **fabricated**
> normal for a well-formed triangle.
>
> ⚠ **Three things this release got wrong first, all caught by measuring rather than by review:**
> * An `is_inf` tail check written as a **function** cost 5 ns of a 90 ns routine (+5.6%). Inlined
>   at its four sites: +1.1%. *A helper is not free in a hot loop.*
> * The first working cap repair materialised the hit point per root — `geo_ray_at` + `hvec3_sub`,
>   both of which **allocate**, four times per query under an allocator that never frees — at
>   **+92%**. The test is algebraically scalar; both dot products hoist.
> * **A toolchain defect was nearly filed that does not exist.** `var buf[N]` passed as bare `buf`
>   SIGSEGVs immediately because a stack array is a *value* and its address is `&buf`. A minimal
>   repro was written and characterised before `lib/fmt.cyr` was read and found using `&buf`
>   throughout. **The stdlib is the specification** — read it before filing against it.
>
> ⚠ **Two fixtures were caught by COUNTERS, not by failing.** The slab surface sweep's first draft
> was **all misses** (72 checks asserting nothing about a hit point); the rotation sweep's first
> draft **missed on 3 of 4 configurations**, leaving three comparisons. Both passed. A run-counter
> beside every sweep is now the cheapest thing in this repo that has repeatedly earned its keep.
>
> **And the suite caught the maintainer overturning a considered contract decision in passing**: a
> finiteness guard rejects NaN too, and `abuse.tcyr` asserts NaN *propagation* with a written
> reason. Narrowed to ±Inf. *A test that encodes a decision is the decision's only defence.*
>
> **Previous refresh**: 2026-08-10 — **v2.10.1, the branchy primitives.** Jets for aabb, obb and
> capsule, so all six `geo_ray_*` are differentiable. Each primal split onto a face/branch-reporting
> variant with the plain entry point a one-line wrapper — the value path unchanged **by
> construction**. Suite 3398 → **3443**, benchmarks 64 → **70**, constant gate 155 → **156**,
> coverage 599/604 → **617/620**, toolchain 6.5.17 → **6.5.18** (compiler-only, all 30 vendored
> files byte-identical between the pins; its `cyrius fmt` string-literal fix leaves hisab's 44
> sources still fmt-clean, checked rather than assumed).
>
> **For the THIRD release running, the design check found defects in shipped code before any
> feature code was written.** Asking "what does the primal return when there is no face?" produced
> two: `geo_ray_aabb` and `geo_ray_obb` returning **+Inf** for a degenerate direction, and
> `geo_ray_sphere`/`_capsule` comparing a **squared** length against the unsquared `EPSILON_F64` —
> the sphere reporting a MISS for any |d| < 1e-6 where the answer is finite and exact, the capsule
> silently returning a **cap** hit instead of the cylinder hit, wrong by **0.942%**.
>
> ⚠ **The sphere's guard was introduced by 2.10.0's own repair.** A fix that adds a guard adds a
> threshold, and a threshold is a new thing that can be wrong. 2.10.0's homogeneity sweep — written
> in that same commit, for that same property — could not have caught it: its direction scales are
> 2, 0.5 and 7, all of order 1, none within five orders of magnitude of the threshold. **Ask of a
> new guard what its own new assertion cannot reach.**
>
> **Three things this release got right by refusing the easy version:**
> * The capsule seam assertion **failed first** at dz = 1e-7 against a 1e-8 tolerance. Rather than
>   loosen the tolerance, the gap was measured at eleven offsets: it falls one decade per decade,
>   which is C1. The assertion is now a **rate** check, which cannot be satisfied by loosening
>   anything.
> * A **surviving mutant** was treated as evidence about the code, not the test: `cyl_t1 <= cyl_t2`
>   always, so a branch update on that path was dead. Removed, and confirmed on 932 randomised
>   quadratics.
> * A mutant that **failed to apply** printed the unmutated pass count, which reads exactly like a
>   kill. Caught by asserting the patch applied. **Verify the mutation applied, not only that the
>   restore did.**
>
> ⚠ **Two benchmark fixtures were found lying rather than failing**, both while building the
> jet/primal ratio table: `ray_aabb` fires along (0,0,1), so two of its three slabs are skipped and
> it has always measured the cheapest path the routine has; and `ray_capsule` lands on a **cap**
> while `jet_capsule` lands on the **barrel**, so dividing one by the other would have reported the
> jet at 2.6x when it is 1.38x. Both axis-aligned rows kept for CSV continuity, a diagonal row
> added beside each. **A ratio is only meaningful between rows at the same fixture.**
>
> **Previous refresh**: 2026-08-10 — **v2.10.0, differentiable geometry.** The first feature line since
> 2.8.x: `src/geo_diff.cyr`, jets for plane/sphere/triangle returning the full gradient from one
> evaluation as a post-pass on the shipped primal. 34 → **35** `[lib]` modules, suite 3376 → **3398**,
> benchmarks 60 → **64**, toolchain 6.5.16 → **6.5.17** (which fixed all three defects hisab filed
> upstream — the closure SIGSEGV and the `distlib` false positive fully, the dead-fn gate hole only
> partially: `lint` and `vet` still exit 0 on a file that does not parse, measured and fed back).
>
> **The release led with a defect in shipped code, found before any feature code was written.**
> Running 2.10.0's own design check — Euler's relation `d·∂t/∂d = −t` — against the EXISTING
> primitives showed `geo_ray_sphere` was not degree −1 in the ray direction: it returned a `t` whose
> hit point sat **3.74 from the centre of a unit sphere**, silently, with `geo_ray_capsule`
> inheriting it through its end caps. Four of six siblings honoured the contract; two did not. The
> same pass found **17 of 60 benchmarks measuring `clock_gettime` rather than the operation** —
> `ray_sphere` read 1,466 ns and is 79 ns — which flattened a true 3.6× triangle/sphere ratio into
> 1.19× and is the only reason the repair's +17.6% cost was visible at all.
>
> ⚠ **AND THEN A THIRD DEFECT, IN 2.10.1'S OWN NEW WORK, FOUND BY A CHECK COMMISSIONED AGAINST THE
> FINISHED CODE.** An independent derivation of all three jets plus an adversarial probe of the
> shipped primitives was run after the release was cut. **Every formula agreed term for term — 37
> partials, 0 of 24,237 finite-difference comparisons failing** — and its value was entirely in the
> **enumeration**: the tie flag saw edges and corners, because those are the two degeneracies the
> design document named, and was structurally blind to a **zero-width slab** and a **tangential
> clip**. A flat box returned the whole gradient in the wrong slot with `tie = 0`, on half of all
> configurations. **A test written by the author of the code inherits the author's list of cases.
> Ask what is missing from the list, not whether the entries on it pass.**
>
> The same probe found **three defects in the PRIMAL**, all re-run on this tree before being
> written down, all left OPEN for 2.10.2 because each changes what a hot four-consumer function
> returns: a **scale-free slab test** that returns a hit at x = 1.4 for a box spanning [0,1] with a
> **unit** direction; `geo_ray_capsule`'s `cyl_missed` fallback returning a point a full radius
> inside the solid; and — worst for this release's own credibility — **two rows of its own
> squared-epsilon disposition table sorted wrong**, because they were sorted by reading the code
> rather than running it. `geo_triangle_unit_normal` was called "a degeneracy policy" and returns a
> **fabricated** normal for a well-formed triangle. *A filing is not evidence until something has
> re-run it, and a disposition is a filing.*
>
> **Standing lesson, and it is the sharpest form of the arc's:** a correctness check written for a
> feature that does not exist yet is worth running against the code that does. Three of this
> release's findings came from that single move.
>
> ⚠ **A fixture-discrimination failure happened three times this release** and is now worth treating
> as a standing question rather than an occasional slip: the k-d balance guard's fixture never fired
> the guard; the `g1` harness asserted order-consistency, which a uniform sign flip satisfies; and
> the sphere jet's fixture used radius 1, where `dt/dr = r/(g·d)` is indistinguishable from
> `1/(g·d)`. Each was caught by mutation, not by review. **Ask of every new fixture: what mutant
> does this actually kill?**
>
> **Previous refresh**: 2026-08-10 — **v2.9.3, the open filings.** Five of the six internal filings in
> `development/issues/` closed; the sixth re-diagnosed and correctly open. Suite 3351 → **3376**,
> benchmarks 55 → **58**, constant gate 153 → **155**.
>
> **The filings were the thing that needed auditing.** Two of the six argued for repairs that
> measurement refuted, and both were implemented in full before being rejected: `incircle-precision`
> implies an exact determinant of *pre-differenced* operands, which fixes nothing because the two
> operands have already rounded to the same double before any exact arithmetic runs; and
> `epa-certificate` proposes dropping EPA's seed test, which certifies every box but makes **four
> exact-tangency pairs report a nonzero depth for a touch whose true depth is 0**. A third claim —
> that EPA's certified exit "is never taken" — was false because it only ever measured one of the two
> public entry points: `gjk_epa_3d` falls back 12 of 12, `mpr_penetration` **0 of 12**.
>
> Two real defects came out of it, both silent: `delaunay_2d` **dropped input points** on any set
> mixing scales (fixed with an adaptive exact `orient2d` on raw coordinates, 300/300 against exact
> rationals where the float winding scores 0/300), and `_col_dl_incircle` answered differently
> depending on **vertex order**. Plus three guards policing nothing — the k-d balance guard had
> neither a regression assertion nor a benchmark on its own input class (now 6.0×), the ear prune's
> only benchmark was its best case (reflex-heavy is 3.6×), and `_col_point_in_tri` documented itself
> as strict-interior while being boundary-inclusive.
>
> **Standing lesson, and it is 2.9.2's pointed one step further back:** a gate is not evidence until
> something has aimed it at its own output — and *a filing is not evidence until something has re-run
> it*. Three of this release's six were wrong in the record, not in the code.
>
> **Previous refresh**: 2026-08-09 — **v2.9.2, toolchain release.** Pin 6.5.9 → **6.5.16** (seven
> releases), sakshi 2.4.8 → **2.4.10**, no library source change. **The doc work was the larger half
> of this release.** A five-dimension sweep of the tree, each dimension adversarially verified,
> found **63 stale documentary claims across 15 files**; the verifier reproduced **59** and refuted
> 4 (three of which had already been fixed mid-sweep, one a misreading of which tracker a filing
> lived in). CLAUDE.md, README.md, CONTRIBUTING.md, SECURITY.md, `architecture/overview.md`,
> `guides/testing.md`, `development/dependency-watch.md` and `development/threat-model.md` had all
> been last synced at **v2.6.15 / cyrius 6.5.6 / sakshi 2.4.7** and had missed nine releases.
>
> **Three gates were green while checking nothing**, and none was found by a test failing:
> `scripts/bench-history.sh` recorded each benchmark's **max** rather than its average in every row
> it has ever written (44 of 55 in the latest run) — the CSV CLAUDE.md designates as the proof for
> every performance claim was built on the noisiest statistic the harness reports, swinging −93% to
> +742% between runs where the average is stable to a median of 3.5%; CI's version-consistency gate
> was an unanchored `grep -q "$VERSION" CHANGELOG.md` whose regex dots matched `32,942,104 B` in an
> unrelated benchmark line, so it would have passed a release with no CHANGELOG section at all; and
> `src/main.cyr`'s hardcoded CLI version string was checked by nothing. All three are now gated.
> **Standing lesson, and it is the 2.6.x lesson pointed at the tooling instead of the code:** a gate
> is not evidence until something has aimed it at its own output.
>
> **Previous refresh**: 2026-08-04 — **v2.8.1, audit release.** Six-dimension sweep of the v2.8.0 tree,
> each dimension adversarially verified by reproduction: **42 confirmed, 0 refuted** (4 critical,
> 17 high, 16 medium, 5 low) — `audit/2026-08-04-v2.8.0-full.md`, scheduled across 2.8.2–2.8.5.
> **The headline is a defect in what 2.8.0 shipped**: `delaunay_2d` is silently incomplete on 15–37%
> of ordinary uniform-random input, from a 10x super-triangle that predates the rewrite — and the
> obvious fix (raise the multiplier) breaks clustered input, because it is coupled to the
> already-filed non-adaptive in-circle predicate. Standing lesson recorded: **1801 assertions and
> 97% coverage caught none of these**, because coverage counts whether a function is *referenced*,
> not whether its contract is *checked*. The sharpest illustration is in the report — every ODE test
> integrand is autonomous, so the DOPRI45 abscissa row is certified by nothing.
>
> **Previous refresh**: 2026-08-04 — **v2.7.0 RELEASED.** Suite 1127 → **1605**, coverage 59% → **71%**,
> benchmarks 28 → **35**, constant gate 110 → **141** + a duplicate-global check. 35 of the
> re-audit's 41 findings fixed; the 6 open ones each carry a written reason and a filed spec.
> `lib/hisab.cyr` **deleted** and `cyrius.lock` regenerated (30 verified) — it was a tracked 591 KB
> copy of hisab's own distlib inside the vendored-deps directory. New: a verified module dependency
> manifest in `architecture/overview.md`, and `audit/2026-08-04-2.7.0-closeout.md` superseding the
> re-audit. **Three corrections to my own analysis are recorded in that close-out** — a grep that
> assumed one space before `=`, a probe that generated coordinates by magnitude class instead of
> reconstructing the caller's geometry, and a scan that counted comments as code. Each contradicted
> a finding; each was wrong. The standing rule: **gates beat greps.**
>
> **Previous refresh**: 2026-08-04 — **re-audit pass** (`audit/2026-08-04.md`). Doc-health work:
> every Tier 1–7 row re-dated to the v2.6.15 state; at-a-glance buckets recounted (Fresh 13,
> read-through **2**, dated artifacts **9**); forward-commitments rewritten with **two new
> entries** (#6 run the constant gate after any constant change, #7 state an audit's SCOPE
> alongside its verdict). Real drift fixed alongside the ledger: CONTRIBUTING and CLAUDE.md did
> not list `check-constants.sh` among the gates, and CONTRIBUTING still led with the manual fuzz
> build. **Correction:** the fifth pass below recorded the 2026-08-03 audit as discharged in full
> — it was not; six of the 70 findings were never scheduled. See `audit/2026-08-04.md` §2.
>
> **2.7.0-A/B repair batches (same day).** `CHANGELOG.md` and `roadmap.md` re-dated to
> **2026-08-04**; `audit/2026-08-04.md` gained a **Disposition → Closed so far** table, per its own
> commitment that the finding list is the unit of disposition. Ten findings closed (6 carried over,
> 3 high, 1 medium) plus one found while fixing; suite **1127 → 1181**. One new item was filed that
> no audit raised: `lib/hisab.cyr`, a tracked 591 KB copy of hisab's own distlib inside the
> vendored-deps directory, referenced and regenerated by nothing.
>
> **Standard-of-evidence note, recorded because it is a precedent:** every fix in these batches was
> mutation-proven except the BVH one, and the CHANGELOG, roadmap and audit all say so explicitly
> rather than letting a passing suite imply proof. Both forms of `_bvh_build_rec` return the same
> tree — the defect is a *cost*, so it is guarded by a benchmark and the assertions guard only the
> result. **An unprovable-by-assertion fix should be labelled as such, not quietly grouped with the
> proven ones.**
>
> **2.7.0-C (same day).** Four of seven medium findings closed; suite **1181 → 1217**. The three
> unapplied ones are filed as dated records under `development/issues/` rather than left in a
> chat log — each carries its verification, its adversarial challenge, and an exact fix.
> `CHANGELOG.md` and `roadmap.md` re-dated again. **Method note worth keeping:** every finding was
> verified *and then adversarially challenged* by an independent reviewer, both running real code.
> The challenges paid for themselves twice — one proved the proposed `levi_civita` assertions all
> pass a wrong-sign mutant, the other found a live SIGSEGV in `wedge_1_1` that the finding never
> mentioned. **A proposed test is not evidence until something has tried to defeat it.**
> **2.7.0-D (same day).** Two more closed — the unscaled Householder (the most consequential fix of
> the line: `svd_golub_kahan` was correct only in roughly 1 .. 1e23, failing **40/40 at scale
> 1e-10**, i.e. SI metres) and the symbolic render carry. Suite **1217 → 1234**. One finding
> remains, `svd_golub_kahan` on rank-deficient input, still filed under `development/issues/`;
> it was **re-checked after balancing landed** rather than assumed fixed, and it is not.
> The v2.6.11 record follows.
>
> **v2.6.11**: Cyrius 6.4.69 → **6.5.6** toolchain bump (a
> **minor** jump across 24 releases) + sakshi 2.4.6 → **2.4.7**. No executable library change —
> the `dist/hisab.cyr` diff is the version header plus one `mat_new_guarded` doc comment, zero
> code lines (16,878 → 16,885). Synced the pin/version across README /
> CLAUDE / CONTRIBUTING / overview / roadmap / dependency-watch / threat-model / SECURITY /
> cyrius.cyml; added the CHANGELOG 2.6.11 entry, the roadmap 2.6.11 release-history row, and the
> threat-model 2026-08-03 audit row; smoke version string 2.6.10 → 2.6.11; regenerated
> `dist/hisab.cyr` (header-only diff). **This bump was not routine.** Re-vendoring surfaced that
> `lib/ganita.cyr` had been stale at **1.0.3** while the *previous* 6.4.69 pin already shipped
> **1.0.4** — so hisab built the whole 2.6.10 cycle against a `mat_new` that **segfaults** on a
> negative dimension (measured: exit 139 on 1.0.3, null on 1.0.4, compiler held constant). That
> closes the roadmap's tracked "stdlib `mat_new` overflow guard" item (open since 2.5.3), flips
> the threat-model row from *open* to *resolved*, and corrects SECURITY.md. **Process change:**
> vendoring is now byte-checked `lib/` **against the pin's snapshot**, not previous-pin against
> new-pin — the latter is what hid this for three releases. Two further defensive fixes: all four
> `.tcyr` harnesses exited with `assert_summary()`'s raw failure count, so exactly 256/512/768
> failures scored **PASS** (demonstrated, then clamped); and the `sys_exit_group` epilogue
> replaced `syscall(SYS_EXIT, …)` repo-wide. `cyrius fuzz` now walks `tests/`, so
> `tests/hisab.fcyr` ran for the first time (1/1). Suite **961/961** (edge_cases 163 → 167: +4
> assertions pinning the upstream `mat_new` contract, mutation-proven). `cyrius.lock` 30 deps,
> `deps --verify` 30/30. Re-verified the 3 open tracked toolchain issues on 6.5.6: interval-ident-lex
> **still live**, for-empty-clauses **still live**, cli-arg-clobber not re-tested (destructive).
>
> **Fifth pass (2026-08-03, v2.6.15).** *(Header corrected 2026-08-04 — this pass claimed the
> audit was discharged in full; it was not. See the sixth pass above.)* P3 (performance) and P4
> (documentation) closed, and the last two never-tested modules (`num_ext`, `symbolic_ext`)
> brought in: **all 34 shipped modules are now included by a test suite**, where eight were not
> even compiled by `cyrius test` when the audit opened. Both O(n^2) hot paths were benchmarked
> BEFORE the rewrite and both rows are in `bench-history.csv`: halfedge twin pairing
> 190.1 ms -> **1.2 ms**, convex-hull pre-sort 22.1 ms -> **2.1 ms**, each verified
> output-identical to the old code. Six documentation drifts corrected across `overview.md`,
> `math.md`, `optimize.cyr`, `num_ext.cyr`, `symbolic.cyr`, `diffgeo.cyr` and
> `collision_core.cyr` — including a module map that credited BVH to the wrong file and a
> `sign` parameter documented three contradictory ways. Suite 1093 -> **1127**; benchmarks
> 26 -> **28**; coverage 57% -> **59%**, files 34/35. **Next: a re-audit** to confirm the
> repairs (doc-health forward-commitment #4), then the 2.7.0 items.
>
> **Fourth pass (2026-08-03, v2.6.14):** audit **P1 — the memory-safety tier — is closed**.
> Capped-constructor null propagation (`cmat_*`), solver dimension caps + `HSB_ERR_ALLOC`,
> B-spline negative indexing, the adaptive-Simpson work bound, `kdtree_build`'s O(n) recursion on
> coincident points (SIGSEGV at 60k), the Armijo NaN acceptance, the complex 1e-6-vs-1e-12
> tolerance, `FLOAT_RENDER_BUF`, and four fBm NaN returns. **Three of the four defect groups
> crashed the process pre-fix.** `calc_ext` and `noise_simplex` brought into the suites, leaving
> only `num_ext` and `symbolic_ext`. Suite 1063 → **1093**; coverage 54% → **57%**, files 32/35.
> Counts synced across README / CLAUDE / testing / overview / roadmap / cyrius.cyml; CHANGELOG
> 2.6.14 + Release-History row added. **Still open:** audit P3 (two O(n²) hot paths, LM/L-BFGS
> per-iteration allocations) and P4 (10 doc drifts).
>
> **Third pass (2026-08-03, v2.6.12 + v2.6.13 — the audit-repair pair):** P0 of the
> 2026-08-03 audit is **fully closed**. 2.6.12 re-derived 47 hand-encoded constants across seven
> tables and added `scripts/check-constants.sh` as a CI gate (110/110); 2.6.13 fixed
> `svd_golub_kahan` (U transposed), `eigen_qr` (never converged for n >= 3), the SU(2)/SE(3)
> half-vs-full-angle split, `einsum` (mis-parsed its own documented examples, then segfaulted)
> and three interval enclosure-soundness violations. Suite **961 -> 1063**, every fix
> mutation-proven; `cyrius coverage` 50% -> **54%**; 4 of the 8 never-tested modules brought into
> the suites (`einsum`, `lie_ext`, `mat3`, `linalg_precision`). Synced version/counts across
> README / CLAUDE / testing / overview / roadmap / cyrius.cyml, added CHANGELOG 2.6.12 + 2.6.13
> and both Release-History rows. **Still open:** audit P1/P3/P4, and `calc_ext`, `noise_simplex`,
> `num_ext`, `symbolic_ext` remain included by no suite.
>
> **Same-day second pass (2026-08-03, post-2.6.11):** full P(-1) audit sweep of the released tree
> → `audit/2026-08-03.md` (**70 verified findings, 2 critical**), and a roadmap restructure that
> opens **2.6.12** as a repair release. Two ledger corrections fall out of it: the 2026-05-29 and
> 2026-05-30 closeouts' "posture solid" verdicts were **scope-limited** to each arc's new
> functions and did not reach the 8 modules no suite includes (noted in Tier 5, originals left
> per the dated-artifact rule); and forward-commitment #4 is now discharged. The Parked list
> gained a rule — shipped items are **removed** with a retirement note, not left struck through.
>
> **Prior refresh**: 2026-07-21 (v2.6.10) — Cyrius 6.4.66 → **6.4.69** toolchain
> bump (a clean 3-patch bump; sakshi unchanged at **2.4.6**, already the latest tag). No
> library source change — `dist/hisab.cyr` byte-identical bar the version header. Synced the
> pin across README / CLAUDE / CONTRIBUTING / overview / roadmap / dependency-watch /
> cyrius.cyml; added the CHANGELOG 2.6.10 entry, the roadmap 2.6.10 release-history row, and
> the threat-model 2026-07-21 audit row; smoke version string 2.6.9 → 2.6.10; regenerated
> `dist/hisab.cyr` (header-only diff). Re-vendored `lib/` to 6.4.69 (all 27 declared-subset
> files byte-match; transitive `result`/`atomic` already identical — no hand-refresh); the
> stdlib delta was **3 files** (`fmt` hex/non-finite-float rendering — linked by `symbolic`;
> `math` float-parse DoS hardening; an agnos-only `sys_reboot` widening). `cyrius.lock` 30
> deps, `deps --verify` 30/30; suite **957/957** across 4 suites, all gates green. Re-verified
> the 3 open tracked toolchain issues on 6.4.69 (minimal repros): interval-ident-lex **still
> live**, for-empty-clauses **still live**, cli-arg-clobber not re-tested (destructive) — no
> new fixes.
>
> **Prior refresh**: 2026-07-17 (v2.6.9) — Cyrius 6.3.11 → **6.4.66** toolchain
> bump + sakshi 2.4.2 → **2.4.6**. Infrastructure + a test-only fix. Synced the pin +
> dep versions across README / CLAUDE / CONTRIBUTING / roadmap / overview /
> dependency-watch / cyrius.cyml; added the CHANGELOG 2.6.9 entry, the roadmap 2.6.9 **and
> the missing 2.6.8** release-history rows, and the threat-model 2026-07-17 audit row.
> Re-vendored `lib/` to 6.4.66 (all 27 declared-subset files byte-match; transitive
> `result`/`atomic` already identical — no hand-refresh this bump); smoke version string
> 2.6.7 → 2.6.9. **Fixed** the pre-existing `tests/modules.tcyr` compile failure (cycc
> reserved SIMD intrinsic names — `iv_add`/`iv_sub`/`iv_mul` unusable as vars;
> renamed `iv_sum`/`iv_diff`/`iv_prod`), restoring 312/312 → **957/957** across 4 suites;
> filed `issues/archived/2026-07-17-cyrius-interval-ident-lex.md`. Re-verified the (now 3) open
> tracked toolchain issues on 6.4.66: for-empty-clauses **still live**, interval-ident-lex
> **new/worked-around**, cli-arg-clobber not re-tested (destructive).
>
> **Prior refresh**: 2026-06-30 (v2.6.7) — Cyrius 6.2.11 → **6.3.11** toolchain
> bump + sakshi 2.1.0 → **2.4.2**. Infrastructure-only (no library source change).
> Synced the pin + dep versions across README / CLAUDE / CONTRIBUTING / roadmap /
> overview / dependency-watch; added the CHANGELOG 2.6.7 entry and the
> threat-model 2026-06-30 audit row. Vendored `lib/` re-synced to 6.3.11 (every
> stdlib file byte-matches the toolchain; `lib/result.cyr` picked up the `_die`
> agnos-portability fix); smoke version string 2.3.3 → 2.6.7. Re-verified the 2
> open tracked toolchain issues: **both still live** (for-empty-clauses confirmed
> on 6.3.11; CLI-clobber not re-tested, destructive) — no new fixes; the 3 fixed
> at the 6.2.11 bump stay archived. Prior refresh: 2026-06-15 (v2.6.6 — 6.0.14 →
> 6.2.11, ganita reorg, 3 of 5 issues fixed → archived);
> 2026-05-30 (v2.6.5). Scaffolded at v2.4.6 during the post-2.4.x
> doc sweep; the **2.5.x** CGA arc then ran (2.5.0–2.5.3 + 2.5.4 closeout with the
> new `architecture/math.md` catalogue), then the **2.6.x** diffgeo arc — 2.6.0
> (sectional) + 2.6.1 (Weyl) + 2.6.2 (transport) + 2.6.3 (deviation) + 2.6.4
> (higher forms) + **2.6.5** closeout (P(-1)/security audit + `math.md §2`).
> Suite 901 → 957. README / CLAUDE / testing / roadmap counts synced per-patch
> through the arcs. The v2.4.6
> verify-and-cleanup pass: re-ran `bench-history.sh`
> (benchmarks.md fresh again — **26** benchmarks at commit `b1165f9`); deleted
> the ad-hoc `development/tool-issues.md` catalog (file real bugs in `issues/`,
> not a random catalog); archived the **shipped** `cyrius-linalg-proposal.md` to
> `development/archive/`; fixed CONTRIBUTING currency (6.0.14, `src/` not `lib/`,
> +distlib gate); and **verified all five `issues/` filings still reproduce on
> 6.0.14** (none stale-fixed). | **Refresh cadence**: opportunistic — when a doc
> is touched, update its row + re-anchor the header date. Not periodic.
>
> **Scope**: this repo only (`hisab`) — the whole `docs/` tree plus root files
> (README, CHANGELOG, CLAUDE.md, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT,
> VERSION). The cyrius toolchain and first-party deps (sakshi) have their own
> doc-health ledgers in their own repos.
>
> **Convention**: adapted from `cyrius/docs/doc-health.md`. hisab's tree is 38
> tracked markdown files (vs cyrius's ~105), so the tier structure here is leaner.

This is a **ledger**, not a one-time audit. Rewrite-in-place as docs change.

---

## At a glance — inventory (last reviewed 2026-08-11, v2.11.1)

**52 tracked markdown files** (`git ls-files '*.md' | wc -l`), counted rather than estimated —
re-counted at v2.11.1 (was 38 at v2.9.3; the arc added nine audit/issue filings and archived
fourteen). Bucket counts:

| Bucket | Count | What it means |
|---|---|---|
| ✅ **Fresh / current** | 14 | README, CHANGELOG, CLAUDE.md, CONTRIBUTING, SECURITY, VERSION, `architecture/overview.md` + `math.md`, `guides/testing.md`, `development/` roadmap + threat-model + dependency-watch + `port-audit.md`, and `benchmarks.md`. **All re-synced to v2.9.2 / cyrius 6.5.16 / sakshi 2.4.10 on 2026-08-09** — see the note below on how far they had drifted. |
| 🟡 **Stale — refresh in place** | 0 | None outstanding. `benchmarks.md` is regenerated by `scripts/bench-history.sh`; the 2.9.2 run is the **first correct one** (the parser recorded each benchmark's max, not its average, in every row before it). |
| 🟠 **Read-through outstanding** | 0 | `SECURITY.md` and `development/threat-model.md` both carry the 2.7.0 → 2.9.x closures. |
| 🔵 **Evergreen** | 1 | `CODE_OF_CONDUCT.md` — Contributor Covenant; re-read only on policy change. |
| 📅 **Dated artifact — supersede, don't edit** | 13 | `audit/` (9 reports — the ninth is the 2026-08-11 v2.11.0 sweep), `benchmarks-rust-v-cyrius.md` (v2.2.0), `development/archive/cyrius-linalg-proposal.md` (shipped), plus the two `2026-08-04-incircle-repro*.tcyr` fixtures, **moved to `issues/archived/` in 2.11.3** after being re-run (0 disagreements of 40 and of 30) — they had been sitting in the OPEN directory implying live defects long after both parent filings were archived. |
| 🐞 **Open filings** | 0 | `development/issues/*` — none open here as of 3.0.1 (31 archived; the two 2026-09-11 upstream filings now have hisab-side closure records). **Two NEW cyrius filings from the 3.0.1 bump are open upstream**, with a self-proving repro: `2026-09-13-hisab-refresh-only-overwrites-released-snapshot.md`, `2026-09-13-hisab-deps-relocks-silently-under-unchanged-pin.md`. Earlier: **both hisab's own as of 2.11.3.** ⚠ **Cyrius bugs now live in the cyrius repo** (`cyrius/docs/development/issues/`), which is where the language agent reads them; 2.11.3 filed the derive-accessor miscompile there with a self-validating repro, and **closed the two upstream filings that used to sit here by re-testing them on 6.6.1** — `cli-arg-clobbers-source` is now guarded fail-closed (`refusing to write build output over a .cyr source file`, exit 1) and `dead-fn-bodies` is rejected by `lint` at last, both archived. ⭐ **2.11.4 closed both of the filings it was carrying rather than deferring them** — `_ad_pow` was REPAIRED (the `base > 0` delegation was discarding up to 174 ULP, and widening the loop exposed a fabricated 1.0 behind a bound checked after a saturating `f64_to`) and `bench-net-below-timer-floor` was REFUTED by measurement (its ratio compared a per-op net against a per-clock-pair floor; the tier it dismissed is the quieter one, and it had been suppressing real −54% speedups). The one that remains is `epa-certificate` (**re-diagnosed in 2.9.3**: its central claim was false and both candidate repairs are measured and rejected; what is left is the **seed-upgrade trade**, whose price is unknown until both halves are re-measured; the sphere-family non-convergence was CLOSED 2026-09-09 as not-a-defect (a polytope expansion cannot certify a smooth surface)). `distlib`'s self-check was **fixed upstream in 6.5.17** and archived; `for-empty-clauses` was archived on discovering upstream had closed it **WON'T-FIX by design**. ⚠ **26 archived filings sit beside the one**, plus a README. The 2026-08-11 audit's backlog is tracked in `roadmap.md`'s Open items rather than as 52 separate files. ⛔ **The "re-run the 28 unverified findings first" gate was RETIRED 2026-09-09**: the 28 are **not enumerated anywhere** — the report records them only as a count — and the tiers they claimed to gate draw from the *confirmed* set, which is disjoint from them. See [`audit/2026-09-09-roadmap-verification.md`](audit/2026-09-09-roadmap-verification.md). |

Numbers roll up from the per-tier tables below.

> ⚠ **How far this ledger had drifted, recorded because it is the ledger's own failure.** Between
> v2.6.15 and v2.9.1 the Tier 1–4 rows below went on reading `✅ Fresh` while the files they
> described missed **nine releases and four toolchain pins**. Every headline number in them was
> wrong by a large factor — the bundle was cited at "~578 KB / 17,286 lines" against a measured
> 805,479 B / 21,368; the suite at "1127 across 4 suites" against 3351 across five; benchmarks at 28
> against 55; the constant gate at 110/110 against 153/153; coverage at 59% against 99%. A currency
> ledger that is itself stale is worse than none, because it converts "nobody checked" into
> "somebody checked and it was fine". The 2.9.2 sweep re-derived every number from the tree rather
> than from the previous row.

**Why now**: the doc tree had drifted (README said v2.2.2, overview said "v1.4.0 /
27 lib files") until the 2026-05-29 sweep — there was no surface tracking
*aggregate* currency. This file is that surface. The same-day verify pass closed
the scaffold's open items rather than letting them linger.

---

## Tier 1 — Structural (root)

| File | Last touched | Status | Action |
|---|---|---|---|
| `README.md` | 2026-09-13 | ✅ Fresh | **v3.1.0**: Stats → 3.1.0 / 4214 / 249 `#must_use` / ~26,400 lines; Security cell leads with the declared surface and the gate; open-upstream count → 4. Earlier, **v3.0.1**: Stats → 3.0.1 / cycc 6.6.3 / sakshi 2.5.2, consumer snippet pins 6.6.3 + tag 3.0.1, Security cell's upstream count → 4 open (two new filings from this bump). Earlier, **2026-09-11 DOCUMENTATION SWEEP.** ⛔ **The Security cell had accreted to 12,724 bytes — a per-release arc narrative living inside one table cell, duplicating `CHANGELOG.md` where it belongs. Now 730 bytes** of durable posture (no FFI/libc/third-party, 248 `#must_use`, 0 open in-tree filings / 29 archived, 2 open upstream, audit-trail pointer). The 3.0.0 breaking notice moved out of that cell into its own section above **Modules**, where an upgrading consumer will actually see it. ⛔ **The Quick Start examples still showed the PRE-3.0.0 idiom** — `calc_integral_simpson(...)` and `num_newton(...)` with their returns discarded, which is now a `#must_use` warning and, in argument position, the silent-tag trap the release exists to remove; both rewritten to bind both halves. ⚠ **The Building block's test counts were a release stale in every row** (454/379/2041/233/775 + 72 benchmarks -> 550/413/2086/239/914 + 78, plus a fuzz line). ⚠ The consumer manifest snippet omitted `"result"` from `[deps] stdlib` and said **15** sidecar leaves where `dist/hisab.deps` names **16**. Earlier, **v3.0.0 RELEASED**: BREAKING banner, migration-guide link, version -> 3.0.0, tests -> **4202**. Earlier, **v2.24.0 RELEASED**: version/tag -> 2.24.0, benchmarks -> **78**, library -> 26,345 lines. Earlier, **v2.23.0 RELEASED**: version/tag -> 2.23.0, tests -> **4039**, library -> 26,243 lines. Earlier, **v2.22.1 RELEASED**: version/tag -> 2.22.1, tests -> **4018**, library -> 26,116 lines. Earlier, **v2.22.0 RELEASED**: version/tag -> 2.22.0, tests -> **4014**, library -> 26,004 lines, and the Security row leads with the Householder gate. Earlier, **v2.21.0 RELEASED**: stats re-measured against the tree — tests **3937 -> 3991**, library **25,127 -> 25,830 lines**, benchmarks **72 -> 74** — and the Security row now leads with the null basis. ⛔ Its filing-count sentence said **"1 open filing, 26 archived, plus a README"** and all three were wrong: there are **0 open and 28 archived**, and the README it named does not exist. Earlier, **v2.18.0**: version/tag -> 2.18.0, tests -> 3937 (hisab 474, modules 2041), library -> 25,127 lines, and the Security row leads with the SVD factors closing in one line, the correction to 2.17.0's NaN-holed figure, and the EPA trade declined. Earlier, **v2.17.0**: version/tag -> 2.17.0, tests -> 3920 (hisab 463, foundation 413, modules 2035), library -> 24,969 lines, bundle -> ~1.0 MB, and the Security row leads with the norm tier — 39 sites, 73 mutants, and the fact that three of its repairs describe defects 2.10.2 / 2.11.1 / 2.14.0 each record as already fixed. Earlier, **v2.16.0**: version/tag -> 2.16.0, tests -> 3807, modules 1965, library -> 24,266 lines, bundle -> ~950 KB, Security row gains the acos-cliff finding. Earlier, **v2.15.0**: version/tag -> 2.15.0, tests -> 3789 (hisab 454, foundation 379, modules 1947), library -> 24,172 lines, bundle -> ~944 KB, and the Security row records the census being CLOSED. Earlier, **v2.14.0**: version/tag -> 2.14.0, tests -> 3657 (foundation 367, modules 1865), library -> 23,738 lines, bundle -> ~914 KB, and the Security row gained the epsilon census. Earlier, **v2.13.0**: version/tag -> 2.13.0, tests -> 3574. Earlier, **Verification sweep**: repaired a dangling sentence fragment left by an earlier splice, corrected the archived-filings count, and linked the roadmap verification report. Earlier, **v2.11.5**: version/tag -> 2.11.5, toolchain -> **6.6.2**, library line count re-measured, and the Building section's stale per-suite counts (1775/739) corrected to 1797/741 — they had been carried since 2.11.0. Earlier, **v2.11.4**: version/tag -> 2.11.4, open filings 2 -> 3. Earlier, **v2.11.3**: version/tag → 2.11.3, tests 3526 → **3532**, toolchain → **6.6.1**, sakshi → **2.5.1**, and the open-filings sentence rewritten — it had enumerated three items under a count of five, and it now records that cyrius bugs are filed in the cyrius repo rather than here. Earlier, **v2.11.2**: version/tag → 2.11.2, tests 3514 → **3526**, toolchain → **6.5.33**, sakshi → **2.4.11**. Earlier, **v2.11.1**: version/tag → 2.11.1, tests → **3514** (foundation 351, abuse 739), library → 23,182 lines, bundle → ~875 KB. Earlier, **Cleanup**: stale in **14 places** — version, toolchain, bundle and CLI sizes, three test counts, benchmarks twice, the consumer `tag`, and the Geometry and Autodiff module rows. All re-measured against the tree. Earlier, **v2.11.0**: library line count re-measured (23,104). Earlier, **v2.10.2**: library line count re-measured (22,707). Earlier, **v2.10.1**: library line count re-measured (22,462), Geometry row now covers all six jets. Earlier, **v2.10.0**: version/tag → 2.10.0, module table gains **`geo_diff`** and the Geometry row now names differentiable ray/surface intersection, library → 35 modules / 21,855 lines. Earlier, **v2.9.3**: version/tag → 2.9.3, tests → **3376**, benchmarks → **58**, library → 21,498 lines, bundle → ~798 KB. Earlier, **v2.9.2 — nine releases of drift closed in one pass.** Every stat re-derived from the tree: version/tag → **2.9.2**, toolchain → **6.5.16**, sakshi → **2.4.10**, tests → **3351 across five suites** (the fifth, `abuse.tcyr`, had been missing since 2.9.0), benchmarks → **55**, bundle → **805,479 B / 21,368 lines**, ELF → **220,064 B**, src → **21,262 lines**. It had been carrying the v2.6.15 figures — `~578 KB`, `1127`, `28`, `6.5.6`, `2.4.7` — since 2026-08-03. |
| `CHANGELOG.md` | 2026-09-13 | ✅ Fresh | **v3.1.0 RELEASED**: `## [3.1.0] - 2026-09-13 — the `pub fn` half: 729 declarations, a gate whose first draft proved nothing, and two more upstream holes`. Earlier, **v3.0.1 RELEASED**: `## [3.0.1] - 2026-09-13 — cycc 6.6.3: both 2026-09-11 filings closed, and the pin's snapshot was not the pin`; the pending 2026-09-11 docs-sweep bullets are folded under it as dated sub-headers. Earlier, **v3.0.0 RELEASED**: `## [3.0.0] - 2026-09-11 — Result<T,E>, and two roadmap premises that measurement refuted`. Earlier, **v2.24.0 RELEASED**: `## [2.24.0] - 2026-09-11 — the two CGA deferrals, and a cost the plan for them did not predict`. Earlier, **v2.23.0 RELEASED**: `## [2.23.0] - 2026-09-11 — the sites no fixture reached, and a defect 2.22.0 introduced there`. Earlier, **v2.22.1 RELEASED**: `## [2.22.1] - 2026-09-11 — the subnormal residue 2.22.0 filed, and why it needed BOTH halves`. Earlier, **v2.22.0 RELEASED**: `## [2.22.0] - 2026-09-11 — the Householder gate: a silent wrong answer at ordinary conditioning, in two public entry points`. Earlier, **v2.21.0 RELEASED, header rewritten**: `## [2.21.0] - 2026-09-11 — the CGA null basis: a point's nullity error becomes ulp-level and scale-free`. ⛔ The section was rewritten end to end after the pre-tag audit: "exactly null" became the residual result (median |P·P|/q **1.0 -> ~4.5e-17, flat**), the Neumaier removal became a **removal AND restoration** with the fabricated-zero witness as the reason, the `point*point` and k-sweep figures were re-measured on both trees, the compiler filing's symptom was corrected, and a corrupted sentence fragment was removed. Earlier, **v2.21.0 RELEASED**: `## [2.21.0] - 2026-09-10 — the CGA null basis: a point is exactly null across the normal range`. Earlier, **2.21.0 bite 5 (unreleased)**: the CGA basis flipped to {e1,e2,e3,n0,ninf} — a point is now EXACTLY null across the normal range (0 of 6800, against 99.4% on the shipped 2.20.0 tree), distance recovery 46x -> 1.9e-15 and flat, translator norm exact across all 1001 binades, and 2.1-2.2x faster. Earlier, **2.21.0 bite 4 (unreleased)**: the null-basis table built and matching its derivation bit for bit — and building it surfaced a cycc 6.6.2 **wrong-code bug** (nested `continue` exits the outer loop), filed upstream with a self-proving repro. Earlier, **2.21.0 bite 3 (unreleased)**: the null-table implementation contract pinned (FNV 0xF4A98C5706D5CF5B in BLADE-INDEX space) plus the defining identities — after a first spot-check misread e3^2 as n0^2 because the derivation works in bitmask space. Earlier, **2.21.0 bite 2 (unreleased)**: the CGA product widened to two term slots, output bit-identical (checksum shown to discriminate, and the new slot shown to be live). Measured cost +3.1% dense / +6.4% sparse — the sparser operand pays more. Earlier, **2.21.0 bite 1 (unreleased)**: the CGA null-basis change SCOPED by a committed derivation — max 2 terms, all coefficients ±1 (no new rounding), 0 round-trip mismatches, and 0 of 25,500 samples non-null against 99.4% on the shipped 2.20.0 tree. Written as a script because 2.20.0's audit found that release's figures were not re-derivable. Earlier, **v2.20.0 RELEASED**: `## [2.20.0] - 2026-09-10`, plus two in-place CORRECTIONS to 2.7.1 entries whose divergence direction had been backwards for thirteen releases. Earlier, **2.20.0 bite 5 (unreleased)**: `ad_grad`'s O(m^2) Jacobian trap documented and benchmarked as a PAIR — the ratio is m-dependent (2x at m=256, 12x at m=2048), so the original "6.8x" was one point on a curve. No code change; the fix is the usage pattern. Earlier, **2.20.0 bite 4 (unreleased)**: two 2.7.1 CHANGELOG passages carried the triangulate_polygon divergence direction BACKWARDS for thirteen releases — measured old 9 (COMPLETE) -> new 6 (PARTIAL) by building both trees; corrected in place and moved into the source. Earlier, **2.20.0 bite 3 (unreleased)**: the subnormal-SVD claim retracted — 270 of 416 rows report SUCCESS and only 57 are right (213 confident wrong answers, worst 9.78x); the old sweep used only upper-triangular blocks. Repair filed, fixture family widened as its acceptance test. Earlier, **2.20.0 bite 2 (unreleased)**: CGA point nullity characterised — a WRONG ANSWER (distance recovery off by a median 46x/49x, symmetric tails), BOTH tails not just small, no arithmetic fix possible, null-basis repair filed. Earlier, **2.20.0 bite 1 (unreleased)**: exact integer rendering above 2^63 — both public renderers had returned the same wrong constant for every input, with a double minus for negatives, invisible to all 3955 assertions. Earlier, **post-2.19.0 (unreleased)**: the `Discarded #must_use results` CI step was red from the commit that added it and failed with NO output — a `bash -e` abort on an unguarded `out=$(cyrius check ...)` for the examples arm, plus a step name eaten by a YAML comment. **The gate had only ever been proven on one of its two arms.** Earlier, **v2.19.0 RELEASED**: `## [2.19.0] - 2026-09-10 — the 3.0.0 prep, and five gates that could not fail`, with entries for all five bites (the EPA polish budget and the `#must_use` tier had narrative but no structured entries until the cut). Earlier, **2.19.0 bite 5 (unreleased)**: `CYRIUS_PKG_VERSION` adopted (output byte-identical), the two live stale-interpolation sentences corrected, and both replacement CI source-guards recorded as having been wrong in OPPOSITE directions on their first draft. Earlier, **2.19.0 bites 3-4 (unreleased)**: the enum conversion, the error-code identity/distinctness block, the duplicate-symbol gate, and the provenance-marker repairs — including the marker **this release itself shipped broken**, and the `--diff` blindness behind it. Earlier, **v2.18.0**: the SVD factors — the one-line fix to an "open-ended, no known fix" item that 2.15.0 had itself created, the correction to 2.17.0's headline figure and the NaN hole both it and its probe shared, the degree-four Wilkinson shift, the wrong causal story I filed for the deflation repair and reverted, the EPA trade declined on a measurement that inverts its premise, and a measured +6.8%/+6.5% stated rather than hidden behind its own spread. Earlier, **v2.17.0**: the norm tier — the two-site row against a 51-site grep and a 19-defect census, the confident-success-at-x0 shape the class takes in a solver, the three repairs that describe defects earlier releases record as fixed, the repair that moved no number and so pointed at a second defect, the two sites this release itself missed on its first pass, and the six of my own fixtures that were wrong before the code was. Earlier, **v2.16.0**: the small-angle series — the acos cliff the guard was hiding, the three-way split of the coefficients, the measured crossover, the norm underflow a mutant found, and a benchmark run recorded as unusable in either direction. Earlier, **v2.15.0**: the epsilon tier closed — 73 of 74 repaired with the four distinctions that decide the repair (unbounded vs bounded numerator, convergence tests, structural sparsity, a comparator that is not an order), `cx_div`'s no-threshold-can-work result, the one deferral and why it is a formula change, and six of my own fixtures that were wrong before the code was. Earlier, **v2.14.0**: the epsilon release — the "~20 sites" estimate against a 97-defect census, the 23 repairs with their three different exact guard forms, 2.10.2's incomplete repair and the too-shallow sweep that certified it, the EPA sign a test had pinned, and the harness that reported four survivors as kills. Earlier, **v2.13.0**: the suite release — 842 truncating sites -> 16, the four defects the conversion surfaced, the tolerance helpers, and the before/after mutation table. Earlier, **v2.11.5**: cycc 6.6.2 — the filed SIMD miscompile repaired and hisab's filing corrected on scope and severity, the top-level-SIMD landmine retired, the `tagged` silent-redefinition class recorded as not-applicable, and no performance claim with the two apparent regressions shown to be single-run outliers. Earlier, **v2.11.4**: the deprecated-alias migration — the 67x scope miss, the byte-identical-output oracle, the -27.85%/-27.67% stated against a noise floor measured in the same session, the two vacuous assertions, and `_ad_pow`'s second collapse. Earlier, **v2.11.3**: the toolchain catch-up that hit a wrong-code regression — the `f64v_*` destination-slot miscompile bisected to cycc 6.5.71 and filed upstream, the two tests that had pinned upstream defects as expected behaviour, the two ceilings the manifest comment had misnamed, the accessor-inlining speedups stated only from the 23 benchmarks that can carry a claim, and the issue triage that took the open queue 5 → 2. Earlier, **v2.11.2**: the toolchain catch-up — the `f64_pow` domain change that moved a hisab public API, the benchmark instrument change and its `regime` column, the 38-file reformat, and the retired 1 MB bundle ceiling. Earlier, **v2.11.1**: the audit release — four repairs, the 52/21/2/28 tally stated with its unverified tier named rather than folded in, and a *What the audit found and this release does NOT fix* section so the four fixed sites are not read as the whole finding. Earlier, **Cleanup**: no content change; 11 issue links repointed at `archived/`. Earlier, **v2.11.0**: +2.11.0 — the tape, the optimizer pairing that needed no API change, the 11.2x measurement with its shortfall explained, and the five forward-mode defects the FD sweep could not reach. Earlier, **v2.10.2**: +2.10.2 — the three primal defects, the four thresholds one rule settled, the rotation partial, and the two repairs that cost far more before being measured. Earlier, **v2.10.1**: +2.10.1 — the branchy primitives, the two defects the design check found first, the C1 seam convergence table, and the jet/primal ratio table with both fixture faults stated. Earlier, **v2.10.0**: +2.10.0 — differentiable geometry, led by the `geo_ray_sphere`/`geo_ray_capsule` homogeneity defect the feature's own check found in shipped code, plus the 17-of-60 benchmark-overhead fix and the 6.5.17 bump closing all three upstream filings. Earlier, **+2.9.3.** Earlier: **Source of truth per CLAUDE.md.** +2.9.2 — the toolchain bump plus the three non-gating gates, with the alloc bullet **corrected in place**: it first claimed hisab does not reach `alloc_via`, which is false (`str_from`/`vec_new`/`vec_push` delegate through `default_alloc()`, 152 call sites in `src/`). Earlier: +2.6.12/13/14/15 entries — the audit-repair arc — plus the **[Unreleased] 2.7.0** section: the correction notice, 2.7.0-A (6 carried-over findings + the `beta`→`eta` rename), 2.7.0-B (3 high + the `cmat` null medium), a **Breaking** entry for `geo_ray_plane`, and a Performance table for `bvh_degenerate_4k` (2.722 s → 15.5 ms). 2.6.12 and 2.6.14 carry **### Security** sections. None breaking; no API or signature change across the arc, so consumers only rebuild. |
| `CLAUDE.md` | 2026-09-13 | ✅ Fresh | **v3.1.0**: status line → 3.1.0 / 4214 / modules 2098 / bundle 1,115,949 B; 3.1.0 paragraph appended; `scripts/` layout and CI gate lists gain `check-public-surface.sh` and the (finally wired) `check-result-migration.sh`. Earlier, **v3.0.1**: toolchain/dep lines → 6.6.3 / sakshi 2.5.2, status line → 3.0.1, bundle ratio 4.2% → **4.38%**, 3.0.1 paragraph appended to the status narrative, and the Layout note on `lib/` now says to byte-check against the cyrius TAG. Earlier, **2026-09-11 DOCUMENTATION SWEEP.** Doc-structure line for `roadmap.md` rewritten — it still promised "completed items, backlog, future features, v1.0 criteria" and the file no longer holds completed items. ⚠ Three figures were a release stale and were re-measured rather than copied: the distlib bundle (1,098,772 B / 26,411 lines -> **1,103,166 B / 26,457**, re-derived after this sweep's own comment edits) and the `dist/hisab.deps` leaf count (**15 -> 16**; `result` joined at 3.0.0), in both the Status paragraph and the Layout block. `cyrius.cyml`'s own bundle-weight comment was stale by two releases and is corrected too. Earlier, **v3.0.0 RELEASED**: status -> 3.0.0, counts (**4202** suites, src 26,378 lines). Earlier, **v2.24.0 RELEASED**: status -> 2.24.0, counts (src 26,345 lines, bundle 1,098,772 B, **78** benchmarks). Earlier, **v2.23.0 RELEASED**: status -> 2.23.0, counts re-measured (**4039** suites, src 26,243 lines, bundle 1,093,623 B / 26,309 lines). Earlier, **v2.22.1 RELEASED**: status -> 2.22.1, counts re-measured (**4018** suites, src 26,116 lines, bundle 1,085,673 B / 26,182 lines). Earlier, **v2.22.0 RELEASED**: status -> 2.22.0, counts re-measured (**4014** suites, src 26,004 lines, bundle 1,079,074 B / 26,070 lines); arc paragraph added for the Householder gate. Earlier, **v2.21.0 RELEASED**: counts re-measured (**3991** suites — modules 2072, src **25,830** lines, bundle **1,067,075 B / 25,896** lines) and the arc paragraph rewritten around the residual result, the compensation round trip, the 2.7 ms first call, and the corrected compiler filing. Earlier, **v2.21.0 RELEASED**: status -> 2.21.0, counts re-measured (3989 suites, 159/159 constants, src 25,737 lines, bundle 1,061,046 B / 25,803 lines, 74 benchmarks); arc paragraph extended with the null-basis result and the cycc wrong-code bug. Earlier, **v2.20.0 RELEASED**: status -> 2.20.0, counts re-measured (3983 across 5 suites, 158/158 constants, src 25,431 lines, bundle 1,043,774 B / 25,497 lines, **74** benchmarks, coverage 640/644 over 36/36); arc paragraph extended with the four re-measured filings and the benchmark-perturbation finding. Earlier, **v2.19.0 RELEASED**: status -> 2.19.0, counts re-measured (3955 across 5 suites, 158/158 constants, src 25,241 lines, bundle 1,032,113 B / 25,307 lines, 72 benchmarks, coverage 640/644 over 36/36); arc paragraph extended with the five-gates story and the four wrong instruments. Earlier, **2.19.0 bite 3 (unreleased)**: the "enums for constants" principle **named the wrong mechanism** and is corrected — `var_table` is 615 on both sides; the saving is relocations (`fixup_table` 3092 -> 2933, `code_size` -704 B). ⚠ Status counts went stale at bite ONE (claims 3937 / modules 2041; measured 3955 / 2044) and are left for the version cut. Earlier, **v2.18.0**: status -> 2.18.0, counts re-measured (3937 across 5 suites, 158/158 constants, src 25,127 lines, bundle 1,028,192 B / 25,213 lines); arc paragraph extended with the guard-pair lesson, the NaN-comparison lesson, the moves-no-number rule applied twice, my own wrong causal story, and the EPA decline. Earlier, **v2.17.0**: status -> 2.17.0, counts re-measured (3920 across 5 suites, **158**/158 constants, src 24,969 lines, bundle 1,017,386 B / 25,055 lines) and the Layout bundle figure corrected — it had been carrying 920,023 B since 2.13.0. Arc paragraph extended with the norm tier: the grep-vs-list miss, the solver shape, the three already-recorded-as-fixed defects, the moves-no-number rule, and the structural reason the suite was blind (two assertions that pinned the defect as a requirement, and a sweep that bracketed thresholds while stopping 142 decades above the arithmetic floor). Earlier, **v2.16.0**: status -> 2.16.0, counts re-measured (3807 across 5 suites, **156**/156 constants, src 24,266 lines, bundle 973,713 B); arc paragraph extended with the acos cliff, the coefficient split, and the mutant that found the norm underflow. Earlier, **v2.15.0**: status -> 2.15.0, counts re-measured (3789 across 5 suites, **155**/155 constants, src 24,172 lines, bundle 967,153 B / 24,258 lines); arc paragraph extended with the closure, the repair-is-not-one-rule distinctions, the accidental-pass reachability probe, and the vacuous-uniform-sweep lesson. Earlier, **v2.14.0**: status -> 2.14.0, counts re-measured (3657 across 5 suites, **154**/154 constants after the `F64_POS_INF` extraction, src 23,738 lines, bundle 936,150 B / 23,824 lines); arc paragraph extended with the census, the derived-sweep-depth rule, the three-guards-for-three-operations distinction, and the fourth consecutive wrong-instrument finding. Earlier, **v2.13.0**: status -> 2.13.0, counts re-measured; arc paragraph extended with the 0.9-invisible measurement and the self-verifying-migration lesson. Earlier, **v2.11.5**: status -> 2.11.5 / cycc **6.6.2**, pin and bundle figures re-measured; arc paragraph extended with the visibility-vs-origin lesson and the retire-don't-redefine rule. Earlier, **v2.11.4**: status -> 2.11.4, src 23,275 lines, bundle **902,898 B / 23,361 lines**; arc paragraph extended with the alias migration, the vacuous-assertion class and the `_ad_pow` finding. Earlier, **v2.11.3**: status → 2.11.3 / cycc **6.6.1**, counts re-measured (3532 across 5 suites, 159/159 constants, bundle **899,733 B / 23,331 lines**), and the arc paragraph extended with the miscompile, the two upstream-fixed expectations, and the two caps that replaced the retired `input_buf` figure. Earlier, **v2.11.2**: status → 2.11.2 / cycc **6.5.33**, counts re-measured (3526 across 5 suites, 159/159 constants, 640/644 coverage, 23,225 module lines, bundle **898,472 B / 23,311 lines = 5.4% of the now-16 MB `input_buf`**), the SIMD-reserved-name claim re-verified on 6.5.33 with a discriminating control, and the arc paragraph extended with 2.11.2 — including that my own instrument was wrong three times before the thing measured was. Earlier, **v2.11.1**: status → 2.11.1, counts re-measured (3514 across 5 suites, 159/159 constants, 640/644 coverage, 23,182 module lines, bundle 895,768 B / 23,291 lines), arc paragraph extended with the audit. Earlier, **v2.11.0**: status → 2.11.0, counts re-measured (3507 across 5 suites, 158/158 constants, 640/644 coverage, bundle 890,623 B / 23,213 lines); arc paragraph extended with the guards-need-direct-interrogation lesson. Earlier, **v2.10.2**: status → 2.10.2, counts re-measured (3469 across 5 suites, 157/157 constants, 618/621 coverage, bundle 874,397 B / 22,816 lines = 83.4% of `input_buf`); arc paragraph extended with 2.10.2 including the stdlib-is-the-specification lesson. Earlier, **v2.10.1**: status → 2.10.1 / cycc **6.5.18**, every count re-measured (3443 across 5 suites, 70 benchmarks, 156/156 constants, 617/620 coverage, bundle 860,061 B / 22,571 lines = 82.0% of `input_buf`); arc paragraph extended with 2.10.1 including the guard-introduced-by-a-guard lesson. Earlier, **v2.10.0**: status → 2.10.0 / cycc **6.5.17**, every count re-measured rather than carried (3398 across 5 suites, 64 benchmarks, 155/155 constants, 599/604 coverage over 36/36 files, bundle **832,590 B / 21,964 lines** = 79.4% of `input_buf`, ELF 224,160 B, src 35 modules / 21,855 lines); the distlib gate rewritten — the 6.5.14–6.5.16 self-check tolerance is **removed**, that filing archived; the arc paragraph extended with 2.10.0. Earlier, **v2.9.3**: status → 2.9.3, counts re-measured (3376/5 suites, 58 benchmarks, 155/155 constants, bundle 817,332 B / 21,604 lines = 78.0% of the 1 MB `input_buf`), and the arc paragraph gained what 2.9.3 found — that two of six filings argued for repairs measurement refuted. Earlier, **v2.9.2**: Status paragraph rewritten from v2.6.15 to the 2.7.0–2.9.2 arc; toolchain → 6.5.16; every count re-measured (3351/5 suites, 55 benchmarks, 153/153 constants, coverage 99%, bundle 805,479 B). Layout gained `tests/abuse.tcyr` and the two new scripts; CI gate list gained the measurement job and the `check --with-deps` bundle step. Also **corrected `cyrius fmt --check <file>` → `cyrius fmt <file> --check`** — the documented order prints a usage error and exits 1; CI always had it right. |
| `VERSION` | 2026-09-13 | ✅ Fresh | **v3.1.0 RELEASED**: -> `3.1.0`. Earlier, **v3.0.1 RELEASED**: -> `3.0.1`. Earlier, **v3.0.0 RELEASED**: -> `3.0.0`. Earlier, **v2.24.0 RELEASED**: -> `2.24.0`. Earlier, **v2.23.0 RELEASED**: -> `2.23.0`. Earlier, **v2.22.1 RELEASED**: -> `2.22.1`. Earlier, **v2.22.0 RELEASED**: -> `2.22.0`; `version-bump.sh` again reported "no rewrite needed" for src/main.cyr. Earlier, **v2.21.0 RELEASED**: unchanged at `2.21.0`; release date moved 2026-09-10 -> **2026-09-11** across CHANGELOG, roadmap and doc-health when the tag slipped a day. Earlier, **v2.21.0 RELEASED**: -> `2.21.0`; `version-bump.sh` again reported "no rewrite needed" for src/main.cyr. Earlier, **v2.20.0 RELEASED**: -> `2.20.0`. `scripts/version-bump.sh` again reported "no rewrite needed" for `src/main.cyr`, which is 2.19.0's `CYRIUS_PKG_VERSION` adoption doing its job. ⚠ Its printed hint uses `date -u` and would have dated this release 2026-09-11; every prior entry uses the LOCAL date, so 2026-09-10 was used and 11 markers written this session were normalised to match. Earlier, **v2.19.0 RELEASED**: -> `2.19.0`. ⭐ **There are now THREE version sites, not four, and one of them is gone for good**: `src/main.cyr` prints `CYRIUS_PKG_VERSION` instead of a literal, so the manifest (`${file:VERSION}`), the CHANGELOG header and `dist/hisab.cyr`'s bundle header are all that remain to keep in step. `scripts/version-bump.sh` reported "no rewrite needed" on this bump, which is the change working. Earlier, **v2.16.0**: -> `2.16.0`; all four sites re-verified. Earlier, **v2.15.0**: -> `2.15.0`; all four sites re-verified. Earlier, **v2.14.0**: -> `2.14.0`; all four sites re-verified. Earlier, **v2.13.0**: -> `2.13.0`; all four sites re-verified. Earlier, **v2.11.5**: -> `2.11.5`; all four sites re-verified. Earlier, **v2.11.4**: -> `2.11.4`; all four sites re-verified. Earlier, **v2.11.3**: → `2.11.3` via `scripts/version-bump.sh`; all four sites re-verified (manifest `${file:VERSION}`, anchored CHANGELOG header, `src/main.cyr`'s printed string, `dist/hisab.cyr`'s bundle header). Earlier, **v2.11.1**: → `2.11.1`; `src/main.cyr`'s printed string and `dist/hisab.cyr`'s bundle header re-stamped with it (the two sites `${file:VERSION}` cannot reach). Earlier, **v2.11.0**: → `2.11.0`. Earlier, **v2.10.2**: → `2.10.2`; all four sites re-verified. Earlier, **v2.10.1**: → `2.10.1`; all four sites re-verified. Earlier, **v2.10.0**: → `2.10.0` via `scripts/version-bump.sh`; all four sites re-verified agreeing (manifest `${file:VERSION}`, CHANGELOG anchored header, `src/main.cyr` string, `dist/hisab.cyr` bundle header). Earlier, Single source of truth (`2.9.3`). **The two sites it cannot reach are now gated AND automated** — `scripts/version-bump.sh` rewrote `src/main.cyr` on this bump, which is the gate added at 2.9.2 doing its job. **Two sites it cannot reach are now gated instead of trusted**: `src/main.cyr`'s printed string — the parenthetical here read *"(Cyrius has no build-time interpolation)"*, true on 2026-08-09 and false from cyrius 6.5.21 (2026-08-13). ⭐ 2.19.0 adopted `CYRIUS_PKG_VERSION` after re-probing on 6.6.2, so `src/main.cyr` is **no longer one of the two sites**: it prints the symbol, resolved from `[package].version` = `${file:VERSION}`, leaving the bundle header as the only place VERSION is still copied and `dist/hisab.cyr`'s `# Version:` header. `scripts/version-bump.sh` rewrites the first; CI asserts both. |
| `CONTRIBUTING.md` | 2026-09-13 | ✅ Fresh | **v3.0.1**: toolchain line → **6.6.3** as of 3.0.1. Earlier, **v2.11.2**: suite 3514 → **3526**, toolchain line → **6.5.33** as of 2.11.2. Earlier, **v2.11.1**: suite count → 3514, toolchain sentence → 6.5.18 as of 2.11.1. Earlier, **Cleanup**: removed a warning block telling contributors to expect a `distlib` exit-1 that 6.5.17 fixed; prerequisite line re-dated to 2.11.0. Earlier, **v2.11.0**: suite count → **3507**. Earlier, **v2.10.2**: suite count → **3469**. Earlier, **v2.10.1**: suite count 3398 → **3443**, prerequisite pin → 6.5.18. Earlier, **v2.10.0**: suite count 3351 → **3398**. Earlier, **v2.9.3**: pin note re-dated. Earlier, v2.9.2: prerequisite pin 6.5.6 → **6.5.16**; both test-suite lists gained `tests/abuse.tcyr`; local gate recipe gained `check-measurements.sh` and the note that `cyrius distlib` exits 1 on a correct bundle under ≥ 6.5.14. |
| `SECURITY.md` | 2026-08-11 | ✅ Fresh | **v2.11.1**: the division-by-zero row now says that the guards it names are *present*, not *proven correct* — the 2026-08-11 audit found the same class at ~24 further sites and **`cx_div`, cited in that very row, is among them**, still returning exactly zero below \|z\| = 1e-12, verified live in this release. Earlier, **Cleanup**: division-by-zero row now records that **the presence of a guard is not the same as a correct one** — four of the autodiff guards it cited were fabricating answers until 2.11.0. Earlier, **Debt discharged.** +7 attack-surface rows organised by *defect class* rather than by module, so the table teaches the pattern: capped-constructor null stores, OOB read, work bounds that do not bound work, recursion depth on degenerate geometry, negative indexing, NaN-read-as-passed, and collision-query completeness. Design Principles gained the mutation-proof standard and the constant gate. Supported-versions table opened a 2.7.x row. Mirrors `threat-model.md`. |
| `CODE_OF_CONDUCT.md` | 2026-03-22 | 🔵 Evergreen | Contributor Covenant. Re-read only on policy change. |

---

## Tier 2 — Architecture (`docs/architecture/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `overview.md` | 2026-09-13 | ✅ Fresh | **v3.0.1**: compiler line → cycc **6.6.3**. Earlier, **v2.11.2**: compiler line → cycc **6.5.33**. Earlier, **v2.11.1**: version → 2.11.1, library line count re-measured (23,182). Earlier, **Cleanup**: ⚠ **`geo_diff` was missing from both the module map and the dependency table** since 2.10.0. Its row (`error` `geo` `quat` `vec3`) was established BY BUILDING, each dep shown load-bearing by removal; `cyrius check` is not a minimality oracle because it tolerates undefined functions in unreachable code. Earlier, **v2.11.0**: header → v2.11.0 / 23,104 lines. Earlier, **v2.10.2**: header → v2.10.2 / 22,707 lines. Earlier, **v2.10.1**: header → v2.10.1 / cycc 6.5.18 / 22,462 lines. Earlier, **v2.10.0**: header → v2.10.0, 35 math modules, 21,855 lines. Earlier, **v2.9.3**: header → v2.9.3 / cycc 6.5.16 / 21,498 lines (the exact-`orient2d` predicate). Earlier, v2.9.2: Earlier, v2.6.15: module map corrected — **BVH moved to `geo_advanced` (it was credited to `spatial`)**, `f64_le`/`f64_ge` removed from `f64_util`, CG relabelled Polak-Ribière+, error constants → `HSB_ERR_*`. |
| `math.md` | 2026-09-11 | ✅ Fresh | **v2.21.0**: ⛔ **§1 was the canonical CGA reference and still documented the RETIRED ep/em basis in full** — metric, blade-index table, and a bit-XOR product rule that the null basis makes false, since `n0*ninf` is a scalar AND a bivector. Rewritten to the null basis {e1,e2,e3,n0,ninf}: new metric and blade table, the change of basis, a new §1.0 on why the basis moved (with the measured residual table), the product recast as a derived TABLE with its FNV contract and the two-index-space hazard, the pseudoscalar note re-derived, and the embedding section given the null-basis forms for point/sphere/plane/translator. Earlier, Equation catalogue. §1 CGA (v2.5.4), §2 differential geometry (v2.6.5). v2.6.15: `hodge_star_2form_4d`'s `sign` corrected — it is an overall multiplier on the Lorentzian dual, **not** a Euclidean/Lorentzian selector. |

---

## Tier 3 — Operational / Development (`docs/development/`)

> `roadmap.md` is the forward-work surface (rotates every release). The rest rotate per-need.

| File | Last touched | Status | Action |
|---|---|---|---|
| `roadmap.md` | 2026-09-13 | ✅ Fresh | **v3.1.0**: public/private row → *The `private` flip — [4.0.0]* with the measured suite reach (64/387), the bundle-collapses-boundaries finding, the two upstream holes, a 12-row disposition table for all 24 cross-module helpers and five naming candidates; new row *Public API reached by no test — [3.2.0]*; the four other rows targeted 3.1.0 retargeted to 3.2.0; *Toolchain, tracked upstream* table corrected (it still listed the two 2026-09-11 filings as open after 3.0.1 closed them) and now lists the four 2026-09-13 filings. Earlier, **v3.0.1**: header toolchain 6.6.3, Current block → v3.0.1 (6.6.3 / sakshi 2.5.2 / ganita 1.2.5), *Public / private function surface* ⛔ BLOCKED UPSTREAM → ⭐ UNBLOCKED with the paired verification, consumer row → 3.0.x and names why the pin matters to a consumer. Earlier, **2026-09-11 DOCUMENTATION SWEEP — the file is now FUTURE-FACING ONLY, 1308 -> 297 lines.** Removed: 38 completed `[x]` items, 19 struck-through release-train rows, the 70-row **Release History** table, the arc-narrative sections (standing lesson, retired-gate rationale, Documentary) and every closed toolchain row. Kept and re-verified: Scope, the v3.0.0 status snapshot, the single BLOCKED public/private item (re-scoped — `pub fn` half to a 3.x, the breaking `private` flip to 4.0.0), the open Optional/Parked entries, Consumers and the Abaco boundary. ⭐ **Four new open items were extracted from prose that was already in the file but not listed as work**: the struct-layout contract's gate that cannot fail, the Abaco table frozen at the 2.2.0 surface, getting a live consumer onto 3.0.0, and the `lerp` SIMD hybrid that was parked under `cross` and was never gated on the same thing. ⛔ **Every line-number citation carried forward was re-derived and ALL SIX had drifted** — the six ordering routines now read `collision_core.cyr:530`, `spatial.cyr:114`, `num_ext.cyr:309`, `linalg_ext.cyr:1071`, `linalg_precision.cyr:1219` and `:1753`. ⚠ The per-version record now lives ONLY in `CHANGELOG.md`; nothing else referenced the removed table except `CLAUDE.md`'s doc-structure line and this ledger's commitment #1, both corrected. Earlier, **v3.0.0 RELEASED**: three 3.0.0 items closed; the fourth recorded as **BLOCKED upstream** with its measurements. One open item left in the file. Earlier, **v2.24.0 RELEASED**: release-history row; **both CGA deferrals closed**, leaving only the four 3.0.0 items open. Earlier, **v2.23.0 RELEASED**: release-history row; the "three gates repaired on the SHAPE" item **closed as PROVEN** (all three load-bearing, and looking also found a regression 2.22.0 had introduced plus an unrepaired fourth reflector); both CGA deferrals updated with measured, patch-ready plans. Earlier, **v2.22.1 RELEASED**: release-history row; the 2.22.0 residue row **CLOSED** — its own premise refuted (one global scalar IS sufficient; f64 spans 2045 binades against the ~1076 needed) — and the defect archived in `issues/archived/`. Earlier, **v2.22.0 RELEASED**: train row + release-history row added; the **subnormal-SVD backlog item CLOSED** — with its own proposed repair recorded as REFUTED by measurement (it converts 84 loud rows into silent wrong answers) — and **two residues filed as new open items**: the deep-subnormal 2x2 fabricated zero, and the three Householder gates repaired on the SHAPE with no fixture reaching them. Earlier, **v2.21.0 RELEASED**: the 2.21.0 backlog row CLOSED (and its translator half marked NOT repaired, since both trees fail identically), the shipped row and release-history row rewritten around the residual result and the compensation round trip, the Current header re-synced to **3991**, and **two deferrals filed with measurements rather than guesses**: cutting the null-table build's remaining four 32-slot passes per pair, and adding a CGA benchmark row — there is none among the 74, which is how a 2.7 ms first call stayed invisible. Earlier, **v2.21.0 RELEASED**: train row struck, Release History row added; 2.22.0 (subnormal-SVD block scaling) is next with its acceptance test already in the suite. Earlier, **v2.20.0 RELEASED**: train row struck, Release History row added, **all five 2.20.0 items discharged (0 open)**, and two follow-on releases filed (2.21.0 the CGA null basis, 2.22.0 subnormal-SVD block scaling) each with an acceptance test already in the suite. Earlier, **post-2.19.0 audit (unreleased)**: swept for orphans before 3.0.0 and found **two items still tagged [2.17.0] that were already FIXED** — the exp-side scaled norms (2.17.0; round-trip floor re-measured **2^-537 -> 2^-1022**, probe checked against 2.16.0's lie.cyr to prove it discriminates) and `cga_norm` (2.18.0; **0 NaN, 0 fabricated zeros, 51 finite** over subnormal coefficients, where the row predicted NaN). Both checkboxes were simply never ticked. Earlier, **v2.19.0 RELEASED**: train row struck, a Release History row added, **every 2.19.0 open item discharged (0 remain)**, and a 2.20.0 row filed for the four items the decision fan-out proved are REPAIRS rather than decisions — plus a fifth found in passing (`ad_grad` is O(m^2) per Jacobian). Earlier, **2.19.0 bite 5 (unreleased)**: the `CYRIUS_PKG_VERSION` item closed — and the row's own "both still assert" was FALSE (`ci.yml` has the sentence zero times); the live claims were in two files it never named. Earlier, **2.19.0 bites 3-4 (unreleased)**: the negative-enum item closed (2 mutants had survived all 3940 assertions, one making `HSB_ERR_ALLOC` collide with `HSB_ERR_INVALID_TRANSFORM`), and `check-measurements.sh` kept + repaired + wired to push — **0 pull requests in the repo's history, so the PR-only job had never run once**. Earlier, **v2.18.0 RELEASED** — `Current` -> v2.18.0, the release-train row struck, a Release History row added, and every 2.18.0 open item discharged: the SVD factors, the deflation tests, `_lp_pow2_floor`, the Wilkinson shift, `cga_norm` and the EPA seed trade. Three NEW items filed from what this release measured but did not fix. Earlier, **v2.17.0 RELEASED** — `Current` -> v2.17.0 with every figure re-measured, the release-train row struck through, and ⛔ **three missing Release History rows restored**: 2.14.0, 2.15.0 and 2.16.0 had never been added, so CLAUDE.md's forward commitment #1 ("Roadmap + Release History rotate every release") had silently lapsed for three releases. All four rows are now present. Earlier, **2026-09-10 (2.17.0 closeout) — the norm tier is marked COMPLETE, 39 of 39 sites**, with each bite's measured before/after inline, six mutation survivors recorded with their reasons, and four NEW items filed from what the tier measured but did not fix: the SVD deflation threshold that costs a singular value 29.3% from block ratio 2^-41, `_lp_pow2_floor` skipping balancing for subnormal matrices, the SVD Wilkinson shift forming B^T*B explicitly, and `cga_norm` on versors/points being catastrophic cancellation rather than this class. Earlier, **2026-09-10 (later) — the 2.17.0 norm tier re-sized from its own grep, and one existing row REFUTED.** The tier row claimed two sites; the tree-wide grep found 51 `f64_sqrt` sites in 16 files and a 70-agent census confirmed 19 defects in 9 modules — the fifth release running that this class was wider than its list, caught by the row's own instruction to grep before sizing. ⛔ The **2.18.0 row's claim that "the singular VALUES stay correct" is struck with a measurement**: `svd_golub_kahan` returns a singular value **29.3% low for every block ratio at or below 2^-41**, silently. Progress, the 13 remaining sites, one deferral with its three independent measurements, and one refuted site are all recorded inline. Earlier, **2026-09-10 — every open item is now pinned to a VERSION.** The file had two views that could disagree — a release train and an unversioned open-items list — so "what is next" needed a cross-read. The train is now the authority on order, every open item carries `**[x.y.z]**`, and a reading rule at the head of Open items says so. ⛔ Found while doing it: the `Current` heading was **three releases stale** (v2.13.0 at a v2.16.0 tree), FOUR struck history items were rendering as open checkboxes and inflating the backlog, and THREE items were done but still listed open — one of them dischargeable by re-reading rather than repairing, because it asked for a null check on an allocation RESULT where a cap on the INPUT was already stronger. Also re-counted `#must_use`'s surface: 167 sites, not the 162 the row claimed. Earlier, **v2.13.0**: `Current` -> v2.13.0, suite tier struck as SHIPPED, release row added. Earlier, **Verification sweep**: release train rewritten around a safety release; the unexecutable 28-findings gate RETIRED; ~18 stale rows struck through **with their proofs rather than deleted**; the epsilon tier re-sized (~20 claimed, **123** guards by the mechanical grep it prescribes), the alloc tier corrected 3 -> 2, the abort tier's three unnamed `collision_core` sites named, and the Consumers section rewritten — it said none were live while **ten repos consume the bundle**. Earlier, **v2.11.5**: `Current` -> v2.11.5 / 6.6.2, release row added. Earlier, **v2.11.4**: `Current` -> v2.11.4, release row added, the migration item struck through with the 67x scope miss recorded on it, and a third open filing (`ad-pow-rationale-collapsed-twice`). Earlier, **v2.11.3**: `Current` → v2.11.3 with every figure re-measured, release row added, the open-filings table rebuilt around **cyrius bugs going to the cyrius repo**, a **closed-by-this-bump** table for the two upstream filings retired by re-testing, and an **Opened by the 2.11.3 bump** tier (benchmark resolution, and the ganita deprecated-alias migration — the one backlog item with someone else's deadline on it). Earlier, **v2.11.2**: `Current` → v2.11.2, release row added, and a new **capabilities-filed-not-taken** table (negative enums, `CYRIUS_PKG_VERSION`, a bench `load` column, the `_sym_render_f64` duplicate) so a later reader sees what was examined and deliberately deferred. Earlier, **v2.11.1**: `Current` → v2.11.1 with every figure re-measured; toolchain line at the head was **stale at 6.5.17** while the manifest pinned 6.5.18; the four-release *check the shipped code first* table gains a fifth row; a new **2026-08-11 audit backlog** under Open items carries the epsilon / allocation / abort / suite tiers, each gated on re-running the 28 unverified findings; the release train gains 2.12.0; `23 archived filings` corrected from `20`. Earlier, **Cleanup**: 642 → 289 lines. Per-release narrative sections deleted (they restated the CHANGELOG); replaced by a single **Open items** section. Two stale open checkboxes removed (shipped in 2.10.1), the `vec_sort_by` block corrected (its upstream blocker was fixed in 6.5.17), and the toolchain paragraph's "four open filings, all cyrius" corrected to **three, one of them hisab's own**. Earlier, **v2.11.0**: **v2.11.0 RELEASED** — Current re-stated, release-train row struck through with 3.0.0 promoted, the 2.11.0 section retired to a delivered one carrying the tape design and the no-`ad_minimize` decision, history row added. Earlier, **v2.10.2**: **v2.10.2 RELEASED** — Current re-stated, release-train row struck through with 2.11.0 promoted, the 2.10.2 section retired to a delivered one, history row added. The demand-gated tie-flag question is **decided: leave it a boolean**, recorded rather than done. Earlier, **v2.10.1**: **v2.10.1 RELEASED** — Current re-stated, release-train row struck through, the 2.10.1 section retired to a delivered one carrying the measured cost table, and a new **2.10.2** section (the three deferred EPA squared-epsilon sites, `dt/d(rotation)` for the OBB jet, and the demand-gated question of whether the tie flag should name WHICH faces tied). ⚠ The 2.10.1 plan's own numbers were wrong and are corrected in place: it claimed 12 `f64_max`/`f64_min` sites at drifted line numbers; the real count is **16**. Earlier, **v2.10.0**: **v2.10.0 RELEASED** — Current re-stated at 2.10.0 / 6.5.17, release-train row struck through, the 2.10.0 planning section retired to a delivered one, and **a new `2.10.1 — the branchy primitives` section** written (the `_core` split of `geo_ray_aabb`/`_obb`/`_capsule` at the 12 enumerated `f64_max`/`f64_min` sites, their jets, per-primitive benchmarks, and the autodiff-vector-layer question deferred to 2.11.0 where the reverse-mode tape forces it). History row added — and the 2.9.3 row's **Files column corrected 58 → 35**, which had been the benchmark count in a module-count column, with the rows re-sorted descending. Earlier, **v2.9.3 RELEASED** — Current re-stated, release-train row struck through, planning section retired to a released one, history row added. Earlier: **Rotates every release.** 2.7.0 sections *Carried over* and *High* marked **CLOSED** with evidence inline; the `cmat` null-propagation medium ticked; a new *Found during 2.7.0-B* section holds the `lib/hisab.cyr` item. 26 items still open under 2.7.0. v2.6.15: Current → v2.6.15; the **whole 2.6.12 audit section retired** (all 32 items closed) to a one-paragraph record + Release-History rows — 327 → 228 lines, **zero completed items left inline**; 2.7.0 re-scoped as **re-audit & refactor**; new **2.7.x feature line** holding the original 2.7.0 items plus those moved out of 2.6.x. |
| `threat-model.md` | 2026-08-11 | ✅ Fresh | **2.19.0 bite 5 (unreleased)**: the 2026-08-09 entry asserted *"Cyrius has no build-time interpolation"* as the REASON the CLI string was hand-edited — **true for four days**, falsified by cyrius 6.5.21 on 2026-08-13, and it stood four more weeks. Corrected in place; the literal is gone. Earlier, **v2.11.1**: its `ALLOC_MAX`-derived matrix ceiling was **stale** — 5792, derived from the 256 MiB limit cyrius 6.4.51 raised to 2 GiB; corrected to **16384**. The policy cap `_OPT_MAX_DIM = 4096` sits below both the old and the new ceiling, so nothing was ever unsafe — which is exactly why the drift survived four toolchain bumps without anything failing. Earlier, **Debt discharged.** Two new sub-tables — *Memory-safety tier — closed in v2.6.14* (6 rows) and *Closed in v2.7.0* (8 rows) — plus three audit-history entries (the 2026-08-03 sweep, the 2026-08-04 re-audit, the 2.7.0-A/B batches). The re-audit entry records **both** process failures verbatim, since the failure mode is the transferable part: scheduling from a digest instead of the finding list, and a coverage-reporting gate whose own coverage was never checked. ⚠️ Also **corrected a row my own change falsified** — `geo_ray_plane` no longer returns −1. **2026-08-05:** the `sequential_impulse` row was split — it credited 2.4.5 with a fix and then covered a *second* defect class it never mentioned. It now carries one row per failure (normal impulse, incl. the 2.8.3 restitution repair it had absorbed silently) plus a new row for the friction impulse that was **identically 0 at every mu** until post-2.8.4. |
| `dependency-watch.md` | 2026-09-13 | ✅ Fresh | **v3.0.1**: status → 6.6.3; a 6.6.3 block (both filings closed with the pair table, the mutated-snapshot + silent-relock finding and its two upstream filings, the not-exposed list, byte-identical binaries). Earlier, **Verification sweep**: the `vec_sort_by` deferral corrected — "exactly one hand-rolled sort" is wrong by 6x (six ordering routines in five files), so the wait-for-the-third-instance gate is discharged. Earlier, **v2.11.5**: pin **6.6.1 -> 6.6.2** — the SIMD destination-slot repair verified from the consumer side, hisab's filing corrected on scope and severity, the second (top-level) SIMD fix recorded with the wrong-cause correction to `vec4.cyr`, and the 6.6.0 `tag()`/`is_tag()` silent redefinition recorded as checked-not-exposed. Earlier, **v2.11.4**: the deprecated-alias row rewritten — **its own estimate was wrong by 67x** (8 claimed, 536 real) because it was scoped from ganita's changelog paragraph instead of its 53-entry deprecation table, and its claim that hisab "calls `ganita_mat_*` already" was false: every `ganita_*` occurrence in the tree was a comment. Earlier, **v2.11.3**: pin **6.5.33 → 6.6.1**, ganita 1.1.4 → **1.2.4**, sakshi 2.4.11 → **2.5.1**, all 30 vendored files byte-checked against the **6.6.1 snapshot itself**. Records the wrong-code regression and that the `src/mat4.cyr` hoist must not be removed; the `f64_pow` binary-exponentiation change and the `math.cyr` infinity guard, both of which moved a hisab result; and the **deprecated `f64_acos`/`f64_atan2` aliases** hisab still calls 8 times. Earlier, **v2.11.2**: pin **6.5.18 → 6.5.33** — the largest gap this file records, and **not** compiler-only: ganita 1.0.4 → 1.1.4, plus `fmt`/`assert`/`bench` and three syscalls variants. ⚠ The tree arrived **mid-sync from ≈6.5.19**, so ganita was three releases stale behind a green `deps --verify` — the third time that shortcut has hidden this exact module. sakshi 2.4.10 → 2.4.11 recorded alongside. Earlier, **v2.10.1**: pin **6.5.17 → 6.5.18**, compiler-only with a zero stdlib delta — all 30 vendored files byte-identical between the two pins, so `lib sync` was a no-op. Its `cyrius fmt` multi-line-string fix leaves hisab's 44 sources fmt-clean. The open dead-fn filing re-run from its own repro: **unchanged**, `lint` and `vet` still exit 0 on a file that does not parse. Earlier, **v2.10.0**: pin **6.5.16 → 6.5.17** with a full entry: compiler-only, **zero stdlib delta** (all 29 vendored files + `sakshi.cyr` byte-match the 6.5.17 snapshot AND are identical between the two pins, so `lib sync` was a no-op), and all three hisab-filed defects re-run from their own repros — closure SIGSEGV and `distlib` false positive **fixed**, dead-fn syntax check **partial** (`build`/`check` reject, `lint`/`vet` still exit 0). The `vec_sort_by` deferral rewritten: its measured blocker is retired, only the API mismatch and the third-instance rule survive. Earlier, Cyrius toolchain version-watch. Pin **6.5.16**, sakshi **2.4.10**; the 6.5.10–6.5.16 delta recorded (10 changed vendored files + transitive `result.cyr`), including the measured ~4 ns/`alloc_via` from 6.5.10's accessor inlining and why no macro-level win is claimed. It had gone three pins without an entry — 6.5.8 and 6.5.9 were never recorded here at all. |
| `port-audit.md` | 2026-05-29 | 📅 Dated + addendum | 2026-04-15 Rust→Cyrius parity snapshot, preserved; 2026-05-29 status addendum records nearly all "P0 gaps" now ported. Don't rewrite the body. |

> Removed this pass: `tool-issues.md` (deleted — ad-hoc catalog; real bugs live in `issues/`), `cyrius-linalg-proposal.md` (→ `archive/`, shipped).

---

## Tier 4 — Guides (`docs/guides/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `testing.md` | 2026-09-13 | ✅ Fresh | **v3.1.0**: modules 2086 → **2098** (the BCH on so(3) fixture), total 4202 → **4214**. Earlier, **v3.0.0**: suite **4039 -> 4202** as error-path assertions split into tag + payload checks. Earlier, **v2.23.0**: suite **4018 -> 4039**; 21 assertions pinning three previously-unreachable sites, each verified against every released tree. Earlier, **v2.22.1**: suite **4014 -> 4018**; the four 2.20.0 defect pins inverted into correctness assertions. Earlier, **v2.22.0**: suite **3991 -> 4014** (hisab 497 -> 512, abuse 776 -> 784). ⭐ The CI gate added in 2.21.0 caught the row/total drift automatically — the first time it has done work. Earlier, **v2.21.0**: suite -> **3991**. ⛔ **Three of the five per-suite rows had been stale for releases and the table could not announce it** — they summed to **3574** against a published total of **3989** (`foundation` 351 for 413, `modules` 1798 for 2072, `hisab` 416 for 497), because the total was updated each release and the rows were not. ⭐ CI now asserts each row against the LIVE suite **and** that the rows sum to the total, verified fail-closed against a stale row, a total that does not sum, and a deleted row. Earlier, **v2.13.0**: suite -> **3574**; a new **compare floats bit-exactly** section replacing the truncation idiom, with the self-verifying-migration rule and the prove-the-mutant-is-installed rule. Earlier, **v2.11.4**: suite unchanged at **3532**; one `modules.tcyr` assertion tightened from a `LOOSE_TOL_M` tolerance to a bit-exact compare, and one `abuse.tcyr` message stripped of a discrimination claim that ganita 1.2.4 made false. Earlier, **v2.11.3**: suite → **3532** (modules 1787 → **1791**, abuse 739 → **741**) — six assertions added where upstream fixes invalidated pinned expectations, including a discriminating `exp(+inf)` line so the newly-correct `exp(-inf) = +0` cannot be confused with the fabricated-zero defect class. Earlier, **v2.11.2**: suite → **3526** (modules 1775 → **1787**: 12 assertions pinning the ganita 1.1.4 `f64_pow` domain, 10 of which fail against 1.0.4 — the 2 that do not are the controls). Earlier, **v2.11.1**: suite → **3514** (foundation 351, abuse 739). ⚠ Its own headline is now known to be generous: the audit measured **836 of the 3510 assertions its own scan counted (23.8%) comparing through the truncating `f64_to`**, and 38.4% of value-changing mutants surviving all five suites. Earlier, **v2.11.0**: suite → **3507** (modules 1775, abuse 734), benchmarks 70 → **72** with an Autodiff row that states the same-work rule: the forward row computes all 16 partials, because timing one dual pass against a full sweep would flatter reverse mode 16x for free. Earlier, **v2.10.2**: suite → **3469** (modules 1739), and a note that 2.10.2's four threshold repairs are all asserted by SCALE COVARIANCE — a single-scale fixture cannot see a bad threshold however many samples it takes, and every other geometry fixture in the suite is single-scale. Earlier, **v2.10.1**: suite → **3443** (modules 1697 → 1713), benchmarks 64 → **70**, and the Differentiable-geometry row now states the same-fixture rule and both fixture faults that made it necessary. Earlier, **v2.10.0**: suite → **3398** (modules 1621 → 1668) in both the recipe and the table, benchmarks 55 → **64** with the new `geo_diff` / tensor / hull / kd rows, and a `bench_batch` warning added to the measurement paragraph — `bench()` cannot see anything under ~1 µs, which is how 17 rows came to be ~95% instrumentation. Earlier, **v2.9.2**: suite counts → 416/349/1621/233/**732** (**3351** total across **five** suites — `abuse.tcyr` was absent from both the recipe and the table), benchmarks 28 → **55**. **Still owes the test-quality revision** carried since 2.7.0. |
| `usage.md` | — | ⚪ Not yet earned (does not exist; CLAUDE.md's Documentation Structure was corrected in 2.9.2 to stop implying it does) | "When earned" patterns/examples guide (CLAUDE.md). README Quick Start covers basics today; promote if onboarding needs more. |

---

## Tier 5 — Audits (`docs/audit/`)

Periodic audit reports; per-audit timestamped. **Don't refresh in place — supersede with a new dated doc.**

| File | Date | Status |
|---|---|---|
| `2026-04-15.md` | 2026-04-15 | 📅 Dated artifact — P(-1) audit (31 issues, 25 fixed) |
| `2026-05-29.md` | 2026-05-29 | 📅 Dated artifact — security/hardening audit closing the 2.4.x arc (no new vuln) |
| `2026-05-29-cga-arc-closeout.md` | 2026-05-29 | 📅 Dated artifact — 2.5.x closeout (v2.5.4): P(-1)/security review of the CGA operators + `mat_new_guarded` (posture solid) + the math.md deliverable |
| `2026-05-30.md` | 2026-05-30 | 📅 Dated artifact — 2.6.x closeout (v2.6.5): P(-1)/security review of the diffgeo curvature/transport/form functions (posture solid) + the math.md §2 deliverable |
| `2026-08-04.md` | 2026-08-04 | 📅 Dated artifact — **re-audit of the repaired v2.6.15 tree, scoping 2.7.0.** 6 dimensions; **42 confirmed / 8 refuted** (4 high, 21 medium, 17 low; no criticals). **Regression verification CLEAN** — every 2.6.12–2.6.15 repair holds under execution, verified against mathematical invariants with pre-fix files re-run as controls. **Corrects the 2026-08-03 disposition record**: six of the original 70 findings were never scheduled, because work was driven from the roadmap digest rather than the finding list. Also found the constant gate silently skipping 35 of 145 declarations. Findings concentrate in `geo_advanced.cyr` (largest module, zero coverage on GJK/EPA/BVH/TOI, uncited by the first sweep) and in test quality (12 findings). |
| `2026-08-03.md` | 2026-08-03 | 📅 Dated artifact — **full P(-1) sweep of the v2.6.11 tree.** 70 findings confirmed / 7 refuted (2 critical, 23 high, 23 medium, 22 low; 64 sites in 27 files). **Disposition corrected 2026-08-04**: the four P-tiers were closed across 2.6.12–2.6.15, but **six findings were never scheduled** and remain open — they never appeared in the roadmap digest the work was driven from. See `2026-08-04.md` §2. Headline: 7 mis-transcribed hex-constant tables, 4 of them published tableaux falsified by their own invariants. Root cause identified as coverage: 8 modules included by no suite, holding 35 of 65 source defects. Explicitly corrects the "posture solid" conclusion of the two 2026-05-29/30 closeouts. |
| `2026-08-04-v2.8.0-full.md` | 2026-08-04 | 📅 Dated artifact — **full sweep of the v2.8.0 tree, scoping 2.8.x.** 6 dimensions, adversarially verified by reproduction; **42 confirmed / 0 refuted** (4 critical, 17 high, 16 medium, 5 low). **Amended in place 2026-08-05** — the one exception to the supersede-don't-edit rule on this tier, because 2.9.0 exit criterion 5 requires a per-finding disposition and the table shipped without a Disposition column, so the standing rule the doc itself states could not be satisfied by anyone. Now carries that column, verified against the tree rather than the CHANGELOG. All 4 criticals, all 17 highs and all 16 mediums were discharged by 2.8.2 / 2.8.3 / 2.8.4. **Low tier CLOSED 2026-08-05**: the 5 lows had never been scheduled and were worked as one pass, so the original 42 now stand at **42 FIXED, 0 OPEN** — with two sub-claims inside the allocation-inside-loop row that did not survive re-measurement and are recorded as such (`src/tensor.cyr:217` **REFUTED**, already hoisted; the row's fourth site **UNRESOLVED**, truncated to `src/geo_` in `70e7c8b` as well as HEAD). Two of the five lows were understated by their tier and the cells say so: the uncapped `max_depth` was a measured **SIGSEGV**, and the sibling-insert return convention is a **breaking** API change. Tracked separately: the friction half of the split `sequential_impulse` row. **Addendum row FIXED 2.9.1 (amended in place again 2026-08-06), and re-measured from scratch first** — its "26 of 3,012" had never been independently reproduced and a second sweep had found zero, so the row now carries an 11,490-evaluation sweep classified exactly in `Fraction`: **50 of the 7,540 exact-support evaluations**, all tangencies, reducing to 6 configurations of which 2 (a point on a solid box CORNER, answered differently in the two operand orders) were in neither prior report. That leaves the whole report at **0 OPEN**. One of its own `clean` notes ("RAY / OVERLAP SIBLINGS AGREE") is retracted there, and the retraction now records *why* it was a blind spot rather than an error: every real divergence needs a zero-volume operand. |
| `2026-08-11-v2.11.0-full.md` | 2026-08-11 | 📅 Dated artifact — **full P(-1) sweep of the v2.11.0 tree, scoping 2.11.1 and beyond.** 6 dimensions, 115 checks; **52 findings reproduced by their author**, 24 sent to an independent skeptic instructed to *refute*, **21 CONFIRMED / 2 REFUTED / 1 ALREADY_KNOWN** (7 critical, 16 high, 22 medium, 7 low across all 52). ⚠ **28 were never sent to a skeptic** — the verify phase was capped at 24 by the harness, not by judgement, and the report says so in its own second paragraph rather than presenting 52 as findings: with 2 of 24 refuted, an unverified finding is ~1-in-12 wrong, and the roadmap gates every downstream tier on re-running them. **24 of the 52 share one root cause** — a guard on the wrong quantity, followed by a fabricated plausible answer — the ninth through thirty-second instance of the class recorded in `complex.cyr:58` in 2.6.14, whose own named example `cx_div` is still defective. Four repairs executed in 2.11.1 (`hquat_inverse`/`hquat_normalize`, three `num_ext` allocations, `ad_tape_new`, the `ALLOC_MAX` ceiling); the epsilon / allocation / abort / suite tiers are carried as roadmap Open items. The report also audits **this repo's own suite**: 836 of the 3510 assertions its own scan counted compare through the truncating `f64_to` and 38.4% of value-changing mutants survive — the measured reason 3507 assertions saw none of it. **Not reached**: `symbolic*`, `lie*`, `spatial`, `color`, `transforms`, `noise_simplex`, `einsum`, `tensor` beyond bounds, the SIMD paths, every cross-module interaction, and anything performance-shaped. |

> **Note on the two "posture solid" closeouts above.** Left at their shipped wording per the
> dated-artifact rule, but read them with the 2026-08-03 correction in hand: neither reached the
> never-included modules, so "solid" described the arc's new code, not the library.

Next periodic security audit: **due now** — all 70 findings of `2026-08-03.md` were closed across 2.6.12–2.6.15, and forward-commitment #4 calls for a re-audit confirming the repairs (and re-running the sweep against a tree where, for the first time, every module is under test). After that, the 3.0.0 (`Result<T,E>`) cut is the next natural boundary.

---

## Tier 6 — Tracked issues (`docs/development/issues/`)

Toolchain/CLI bug filings observed from hisab's vantage. Filed against cyrius
**5.7.x** and **6.4.x**; 3 of the original 5 were fixed at the 6.2.11 bump and moved to
`issues/archived/`, and one new filing (interval-ident-lex) was added at 2.6.9 — so **3 are
open**. All three were **re-verified 2026-08-03 against the pinned 6.5.6**: the two testable
ones still reproduce, and the 6.4 → 6.5 minor fixed neither. hisab carries workarounds for each
open one; the bugs belong upstream in cyrius.

**Toolchain filings** — re-verified 2026-08-10 on **6.5.17** from each filing's own minimal repro
(prior: 2026-08-09 on 6.5.16; 2026-08-03 on 6.5.6; 2026-07-21 on 6.4.69; 2026-07-17 on 6.4.66;
2026-06-30 on 6.3.11). ⚠ Rows carrying an `archived/` prefix have SHIPPED and live in
`docs/development/issues/archived/`; only the unprefixed ones are still in `issues/`:

| File | Filed | Status |
|---|---|---|
| `archived/2026-07-17-cyrius-interval-ident-lex.md` | 2026-08-10 | 🟢 **CLOSED, archived 2026-08-09.** The *diagnostic* was the defect and it is fixed — all 67 reserved names emit the new wording on 6.5.16 and 6.5.17. ⚠ **The reservation itself is BY DESIGN and still stands**: `iv_add`/`iv_sub`/`iv_mul` remain unusable as variable names, so `modules.tcyr`'s `iv_sum`/`iv_diff`/`iv_prod` rename is load-bearing and must not be reverted. Original text: 🐞 Live — re-confirmed on 6.5.6 by minimal repro: `var iv_add = 1;` → `expected identifier, got unknown`. `iv_add`/`iv_sub`/`iv_mul` are reserved cycc SIMD intrinsic names and cannot be used as variables. Workaround: `tests/modules.tcyr` renamed them `iv_sum`/`iv_diff`/`iv_prod` (guarded by a `NOTE:` in the test). *(This row was missing from the table through 2.6.9/2.6.10 even though the at-a-glance bucket counted 3.)* |
| `archived/2026-04-26-cyrius-cli-arg-clobbers-source.md` | 2026-08-09 | 🐞 **Live (presumed)** — **deliberately not re-tested at the 2.9.2 bump either**, and now recorded as *unverified* rather than assumed-still-live. The reproducer works by getting cyrius to treat a real source path as the output slot, so running it here truncates a `src/*.cyr` to 0 bytes. Only safe observation: `cyrius -v` with no path arguments still exits 0 and prints usage rather than rejecting the unknown flag, so the strict-flag check that would fix it is absent. Suggestive, not proof. Workaround: `CYRIUS_VERBOSE=1`, never an unknown flag before a subcommand. |
| `archived/2026-04-26-cyrius-for-empty-clauses.md` | 2026-08-09 | 🐞 **Live on 6.5.16, but half-fixed.** Both forms still reject (`for (; c; s)` → `unexpected ';'`; `for (i; c;)` → `expected '=', got '{'`). What the filing actually complained about — the error landing far down the file, findable only by bisection — is **gone**: errors now carry the true `file:line` with a caret on the token, and both forms report in the same run. ⚠ Reproduce in an `include`d module: line numbers for the *top-level* source are offset by (stdlib dep count + 1) because they index the concatenated translation unit. Workaround unchanged: `while` loops in collision_core/mesh. |
| `archived/2026-08-09-cyrius-dead-fn-bodies-are-never-syntax-checked.md` | 2026-08-10 | 🐞 **Live on 6.5.17, PARTIALLY fixed.** `cyrius build` and `cyrius check --with-deps` now both reject the repro (exit 1) — the half that matters, since nothing unparseable can be built or shipped. But **`cyrius lint` and `cyrius vet` still exit 0** on a file that does not parse, so the release note's "accepted by every gate" is not discharged. Measured on the filed repro at the 6.5.17 bump and fed back upstream; left OPEN rather than closed. |
| `archived/2026-09-11-cyrius-nested-continue-binds-to-wrong-loop.md` | 2026-09-13 | 🟢 **FIXED in 6.6.3, verified as a pair on the 3.0.1 bump.** hisab's reproducer exit 1 → 0; upstream gate 2/8 → 8/8; `_cga_build_null_tbl` in its natural `continue` form (inner `continue` firing 1024×) wrong on 6.6.2 (FNV mismatch, 1024/1024 bad coefficients) and exact on 6.6.3. Written at closure — hisab had filed only upstream in 2.21.0. The `if`-guard form stays for consumers on older pins; its comment no longer claims to be load-bearing. |
| `archived/2026-09-11-cyrius-derive-cannot-combine-with-public.md` | 2026-09-13 | 🟢 **FIXED in 6.6.3, verified on hisab's own idiom in both directions.** 6.6.2 rejects `#derive(accessors) public struct`; 6.6.3 builds it, derived getters AND setter reachable cross-file (exit 0), file-private helper still refused with no binary. Written at closure — 3.0.0 had filed only upstream. Consequence: roadmap's public/private row UNBLOCKED, work unchanged (3.1.0 / 4.0.0). |
| `archived/2026-08-10-cyrius-capturing-closure-across-fn-boundary.md` | 2026-08-10 | 🟢 **FIXED in 6.5.17.** Re-run from the filed repro: case-3 returns **42** (was SIGSEGV 139) and the `fncall1` variant returns 42 too, so both dispatch paths are repaired. ⚠ Its consequence is recorded in `dependency-watch.md`, not just here — this was the *measured* half of the `vec_sort_by` deferral, and it is retired. |
| `archived/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md` | 2026-08-10 | 🟢 **FIXED in 6.5.17.** `cyrius distlib` exits 0 on hisab's real bundle and on all three arms of the minimal reproducer (stdlib function / global var / enum constant). **The CI and release tolerance is removed** — both workflows run a bare `cyrius distlib` again and a non-zero exit is a real failure once more; the history is kept inline in `ci.yml` because the shape recurs. `cyrius check --with-deps` kept independently: it is strictly stronger than the self-check ever was. |
| `archived/2026-08-10-ray-sphere-capsule-assume-unit-direction.md` | 2026-08-10 | 🟢 **FIXED (2.10.0)** — hisab's own, not a toolchain bug, listed here because 2.10.0 turned on it. `geo_ray_sphere` dropped `a = d·d` from both discriminant and division, so a non-unit direction returned a `t` whose hit point was **3.74 from the centre of a unit sphere**, silently; `geo_ray_capsule` inherited it through its end caps only — its own cylinder quadratic was already correct, so one root cause took the homogeneity sweep 5 bad → 0. Cost **+17.6%** on `ray_sphere`, interleaved A/B, accepted. |
| `archived/2026-08-10-aabb-obb-degenerate-direction.md` | 2026-08-10 | 🟢 **FIXED (2.10.1)** — hisab's own. `geo_ray_aabb` and `geo_ray_obb` returned **+Inf** for a zero-length direction with the origin INSIDE the volume: every slab skipped as parallel, nothing narrowed the interval, and the tail handed back the sentinel `t_max` was initialised with. `geo_ray_at(ray, +Inf)` is NaN in every component. ⚠ The origin position is load-bearing — from OUTSIDE the parallel branch's own bounds check fires first and the contract is honoured by accident, so a sweep firing only from outside reproduces nothing. Fixed by counting contributing slabs, deliberately not by a `d.d < EPSILON` guard (that test is the next row's defect). Cost not measurable. Third instance of the sibling-asymmetry class, after 2.9.1's mpr/gjk and 2.10.0's homogeneity pair. |
| `archived/2026-08-10-squared-epsilon-guards-in-geo-ray.md` | 2026-08-10 | 🟢 **FIXED (2.10.1)** — hisab's own, and **one half of it was introduced by 2.10.0's repair**. `geo_ray_sphere` and `geo_ray_capsule` compared a SQUARED length against `EPSILON_F64`, an unsquared tolerance, making the effective threshold 1e-6 rather than 1e-12: the sphere returned a MISS for any \|d\| < 1e-6 (t = 4e7 on a unit sphere at (0,0,5) with d = (0,0,1e-7) — finite and exact, and aabb/triangle/plane all returned it), the capsule silently returned a CAP hit instead of the cylinder hit, wrong by **0.942%** constant at every scale below the threshold — the tell that it is a BRANCH SWITCH, not precision loss. Fixed with `_GEO_F64_EPS_SQ` (1e-24); all six exact to k = 1e-11. Second instance of a class `complex.cyr:58` has carried since 2.6.14 **with the fix idiom already in that file** — a lesson recorded in one module is not applied across the tree until something greps for it. The filing carries a disposition table for all nine such sites in `src/`: 2 fixed, 3 EPA ones deferred to 2.10.2 with a reason, 4 left as documented degeneracy policies. |
| `archived/2026-08-10-slab-parallel-test-is-scale-free.md` | 2026-08-11 | 🟢 **FIXED (2.10.2)** — hisab's own, CRITICAL. `\|d_k\| < EPSILON_F64` deleted a slab, which is wrong whenever the ray travels far enough for a tiny component to matter: box [0,1]^3, origin (0.5, 0.5, -1e12), direction (9e-13, 0, 1) with **\|d\| = 1.0 exactly** returned t = 1e12 whose hit point sits at **x = 1.4**, and the same ray from inside a tall box gave an exit **18x too large**. `geo_ray_new` normalizes and did not protect. Also covered: aabb and obb disagreeing at exactly EPSILON_F64 (0 vs 4.5e12), and +Inf from entirely finite inputs. Fixed at `_GEO_F64_TINY` = DBL_MIN — guard exactly what makes the division fail. ⚠ The first tail guard tested FINITENESS and broke `abuse.tcyr`'s deliberate NaN-propagation contract; narrowed to ±Inf. |
| `archived/2026-08-10-capsule-cyl-missed-tags-a-sphere-root-as-a-cap.md` | 2026-08-11 | 🟢 **FIXED (2.10.2)** — hisab's own. `geo_ray_sphere` returns only the FIRST non-negative root, so a cap root rejected by the half-space test meant the other was never considered: an axial ray from inside returned t = 4 whose hit point is **on the axis, a full radius inside the solid**, and 32 of 600 random interior origins (5.3%) lost their exit entirely. Fixed by splitting the quadratic onto `_geo_sphere_roots` (both roots, ONE copy — `geo_ray_sphere` is now the selection on top) and deleting the `cyl_missed` fallback outright. The assertion is a SURFACE-RESIDUAL sweep, not a t check: 4.0 is a plausible number any loose t assertion would accept. ⚠ It needs `geo_segment_distance_to_point` trustworthy, which is why that repair was sequenced first. |
| `archived/2026-08-11-forward-mode-dual-guards.md` | 2026-08-11 | 🟢 **FIXED (2.11.0)** — hisab's own, five defects in `src/autodiff.cyr`, found by sweeping forward mode BEFORE writing the reverse-mode tape that would be validated against it. `1/1e-13` returned (0,0) where the truth is 1e13; **`ln(-5)` returned a NaN value beside a CONFIDENT -0.2 derivative**, worse than either alone; `sqrt(-4)` was (NaN,NaN) because the guard ran AFTER the sqrt; `d/dx x^1 at 0` and `d/dx x^3 at -2` were NaN because `f64_pow` is documented as exp(n*ln(base)) and rejects non-positive bases. ⚠ **The finite-difference sweep alone found NONE of them** — at every input where a guard fires the perturbed SCALAR is NaN too, so the sample is skipped and the guard never asked. Guards need direct interrogation. A control assertion pins that (-2)^0.5 must STILL be NaN, or 'fixed the NaN' and 'replaced one fabrication with another' are indistinguishable. |
| `archived/2026-08-11-quat-inverse-squared-epsilon.md` | 2026-08-11 | 🟢 **FIXED (2.11.1)** — hisab's own. `hquat_inverse` compared a **squared** magnitude against the unsquared `EPSILON_F64`, so the guard fired at `\|q\| < 1e-6` — six orders coarser than intended — and returned the **identity**, a valid quaternion that passes every sanity check a caller might apply. `q · q⁻¹ − I` measured 0.999999900 at s = 1e-7; a quaternion of magnitude 1.4e-7 has an inverse of magnitude 7e6, finite and exactly representable. `hquat_normalize` took the same rule. Both now guard on **DBL_MIN**. Seventh instance of the class. Constant gate 158 → 159. ⚠ Carries its own failure: the first 12-decade sweep killed 1 of 3 mutants, because both thresholds a repair might plausibly have chosen sit *below* where it looked. |
| `archived/2026-08-11-caller-sized-allocs-in-num_ext.md` | 2026-08-11 | 🟢 **FIXED (2.11.1)** — hisab's own. `num_fft_2d`, `num_ifft_2d` and `num_tridiag_solve` sized a workspace from a caller-supplied dimension and stored through `alloc`'s documented 0 return. **Reproduced as SIGSEGV (exit 139)** from public entry points; now `HSB_ERR_ALLOC`. No dimension cap added — an FFT of a genuinely large signal is a legitimate request, and the honest failure is *I could not allocate that*, not *I refuse sizes above an arbitrary line*. ⚠ **The first reproduction attempt failed** because the probe was sized from `optimize.cyr`'s stale `ALLOC_MAX = 256 MiB` comment and landed exactly on the real 2 GiB limit; a constant derived from a dependency is a measurement. Mutation: removing the guards **crashes the suite process**. |
| `archived/2026-08-11-ad-tape-new-null-nodes.md` | 2026-08-11 | 🟢 **FIXED (2.11.1)** — hisab's own, and **2.11.0's own code, one release old**. `ad_tape_new` returned a non-zero handle whose node array was `alloc`'s 0, so every subsequent record stored through null. It now returns 0, which is what every capped constructor in the tree already promised. Found by the same mechanical sweep as the `num_ext` sites — reading found the candidates in minutes, **only running separated the two real ones from the eleven that already had guards**. |

**hisab's OWN defect and perf filings also live in `issues/`** — they are not toolchain bugs and are
not counted in the 3 above. A reader counting files in that directory will see more than this tier's
table lists; this is the index for the rest. Disposition is carried in each file's own Status line
and in the CHANGELOG entry that closed it.

| File | Filed | Status |
|---|---|---|
|  `archived/2026-08-05-two-ghost-tie-branch-sign-rule-inconsistent.md` | 2026-08-05 | 🟢 **FIXED 2.9.1** — `_col_dl_ic_g2`'s M² tie branch multiplied by the winding twice, inverting the in-circle answer on every anti-cyclic ghost pair (0 wrong cyclic / 267,358 of 267,358 anti-cyclic, exact-circumcircle oracle). Latent since 2.8.2: `cr` is `+1` on every path `delaunay_2d` takes, so no reachable behaviour changed. 9 new assertions, mutation-proven both ways. The filing's open question (3) — *should this be callable with a CW triangle?* — is answered in a **Resolution** section with measurements, including the evidence that rejected the alternative repair. |
| `archived/2026-08-06-ghost-incircle-g1-g3-assume-ccw.md` | 2026-08-09 | 🟢 **CLOSED (2.9.3)** — fallout from the above. **`_col_dl_ic_g3` fixed**: the all-ghost answer is the constant `1` (re-derived independently, not inherited — equal `\|u_k\|²` put the apex at the ghosts' circumcenter, and the M⁴ coefficient shares its winding factor with the ghost triangle's orientation, so the two cancel), body is `return 1;` with **no arguments**, asserted over both windings and mutation-proved. The file records one correction to its own reasoning: "every finite point is inside a circle through three points at infinity" is too general — unconstrained direction triples answer `0` ~36% of the time. **`g1`'s M¹ tie-break also fixed in 2.9.1** (betweenness). **`g1`'s missing winding factor CLOSED in 2.9.3** — `g1` now forms `sign((b - a) x u_k)`, falling back to `sign((b - a) x (A - a))` when the edge is parallel to `u_k` (that case is NOT degenerate: the triangle keeps constant area as the ghost recedes, and the apex sets the side — the first draft got this wrong and the 2.9.1 reproducer caught it). Validated 4606/4606 against an ORDER-FREE exact oracle, 1110 of those parallel-edge. The Python-transcription evidence is replaced by `_dlg1_run` (1800 probes, pre-fix score 846 of 1500) plus four correctness assertions — order-consistency alone was too weak, a sign-flip mutant survived the grid. **`_col_dl_incircle` is order-free at every arity**; filing archived. |
| `archived/2026-08-04-incircle-precision.md` | 2026-08-09 | 🟢 **FIXED 2.9.3** — and the 2.9.1 "recommended for closure" below was WRONG. `delaunay_2d` silently dropped input points on any set mixing scales (5 of 6 points used, 4 triangles where the exact hull says 6; 8 of the first 20 seeds). The 0-of-1,528,820 sweep that justified closure drew all four fixtures from a single origin-anchored box, so it had no intra-set dynamic range and could not reach this; and `_dl_violations` calls `_col_in_circumcircle` as its own oracle, so the suite was blind by construction. Fixed with an adaptive exact `orient2d` on RAW coordinates (300/300 vs exact rationals; the float winding scores 0/300). ⚠ The obvious fix — exact determinant of **pre-differenced** operands — was implemented first and changed nothing, because the differencing collapses two points into one before any exact arithmetic runs. Prior text retained — Its CORRECTION section reasoned from a `10 × max(dx, dy)` super-triangle that **2.8.2 deleted**; the 0-of-180 was measured against geometry three minor versions gone. Re-measured over every point quadruple of the suite's own fixtures — a strict superset of `delaunay_2d`'s call set now that ghosts have no coordinates: **0 wrong of 1,528,820**. Predicate is still non-robust *in isolation* (36 of 400 at the original's magnitude range); nothing can hand it that input. |
| `archived/2026-08-04-perf-delaunay_2d.md` | 2026-08-09 | 🟢 **CLOSED (2.9.3) — SUPERSEDED.** It measured the O(n²) per-point full rescan, which the **2.8.0 adjacency rewrite** deleted; insertion is now walk + flood-fill and there is no rescan to remove. Archived rather than deleted for its CHALLENGE section. **Not** the reason `delaunay_2d` is fast, and must not be cited as one. |
| `archived/2026-08-04-perf-kd_partition.md` | 2026-08-09 | 🟢 **CLOSED (2.9.3).** Mis-filed as "NOT APPLIED": the repair shipped in **2.7.1** as a balance guard (the spec's unconditional quickselect measured +114%). 2.9.3 closed both open halves — the guard had **no regression assertion** (`kdtree_build != 0` and the split-plane invariant both hold on a degenerate spine, so deleting it broke nothing checked), now a depth assertion on an **octave-spaced** fixture, mutation-proven; and the BENCH section shipped: `kdtree_build_octave_512` **564.8 µs guarded vs 3.397 ms unguarded, 6.0×**. ⚠ The pre-existing geometric fixture does NOT exercise the guard (ratio 0.99998 → range [0.85, 1]); recorded inline so it is not mistaken for its test. ⚠ The filing's 574 ms headline still has not reproduced and is not restated as fact. Prior text — The lumped row above was wrong on all three counts for this one: the repair **did** land (2.7.1, as a balance guard rather than the spec's unconditional quickselect), so "NOT APPLIED" was false. What was genuinely missing is that the guard had **no regression assertion** — `kdtree_build != 0` and the split-plane invariant both hold on a degenerate spine, so deleting the guard broke nothing that was checked. Closed in 2.9.3 with a depth assertion on an **octave-spaced** fixture, mutation-proven. ⚠ The pre-existing geometric fixture (ratio 0.99998) does **not** exercise it — its axis range is only [0.85, 1] so the guard never fires; that is recorded inline so the weak fixture is not mistaken for the guard's test. |
| `archived/2026-08-04-perf-triangulate_polygon.md` | 2026-08-09 | 🟢 **CLOSED (2.9.3).** Was mis-filed as "NOT APPLIED": the reflex-only ear prune shipped in **2.7.1**. 2.9.3 closed the rest — the small-n regression is now MEASURED (A/B vs the pre-prune body from `97bed43^`: n=6 **+15%**, n=600 **−82%**, non-overlapping, <2% spread) and disclosed beside the −84% headline; the missing reflex-heavy benchmark shipped (`triangulate_comb_600` **5.718 ms** vs the convex case's **1.593 ms**, 3.6× — the only prior guard was the prune's own BEST case); and `_col_point_in_tri`'s false "(2D, strict interior)" comment is corrected with five assertions. ⚠ Two items carried forward and named in the filing: a backwards divergence-direction claim in the CHANGELOG, and shadowed locals at `collision_core.cyr:740,742`. Prior text — Also mis-filed as "NOT APPLIED": the reflex-only ear prune shipped in **2.7.1** (`triangulate_600gon` 9.93 → 1.59 ms). Open items are documentary — a small-n regression below break-even that the −84% headline does not disclose, and a missing reflex-heavy benchmark. 2.9.3 fixed the one code-adjacent item: `_col_point_in_tri`'s doc comment claimed "(2D, strict interior)" while the predicate is **boundary-inclusive**, which is what ear clipping actually needs; five assertions now pin the closed contract. |

**Archived — fixed on 6.2.11 (`docs/development/issues/archived/`)**:

| File | Filed | Resolution (verified 2026-06-15) |
|---|---|---|
| `2026-04-26-cbt-modules-substring-false-positive.md` | 2026-04-26 | ✅ Fixed — a `[build]` comment with the token `modules` + a `[lib] modules` array no longer mis-binds; build compiles only the source. |
| `2026-04-26-cc5-18-arg-fn-scrambles-params.md` | 2026-04-26 | ✅ Fixed — `f18(1..18)` of params {1,2,7,8,11,12,18} → exit 59 (exact); SysV scramble gone. `src/calc.cyr`'s 10-arg split left in place (optional revert). |
| `2026-04-26-cyrius-lint-rc-as-warning-count.md` | 2026-04-26 | ✅ Fixed — 2-warning file now exits 0 while still printing `  warn …` lines; CI's `^\s*warn ` stdout grep remains the load-bearing gate. |

---

## Tier 7 — Benchmarks

| File | Last touched | Status | Action |
|---|---|---|---|
| `benchmarks.md` | 2026-09-11 | ✅ Fresh | **v2.24.0**: four CGA rows added (74 -> 78), registered last; control over 26 existing rows median +0.05%, 0 past 10%. Earlier, **v2.20.0**: 72 -> **74** rows. ⛔ The two new `jac_rev` rows had to be moved to register LAST: under a bump allocator that never frees they shifted the heap and made `kdtree_radius_4k` read 484 ns instead of 696 ns — a 31% phantom change in an untouched module. Earlier, **v2.11.2**: **re-baselined, and the baseline is not comparable to any earlier row.** 6.5.19's bench instrument moved 44 of 72 rows >10% with none of it a speedup; rows now carry `regime`/`floor_ns` and the trend filter refuses to mix regimes, so the table is a 1-point trend by design until more `net` runs accumulate. Earlier, **Re-run each release of the arc** via `scripts/bench-history.sh` — now **28** benchmarks (+`convex_hull_2d_2k`, `halfedge_2k_tris`, added *before* the 2.6.15 rewrites so the wins are provable). Both before/after rows are in `bench-history.csv`. |
| `benchmarks-rust-v-cyrius.md` | 2026-04-15 | 📅 Dated artifact | Rust vs Cyrius comparison, bounded to **v2.2.0**. Supersede only if a port-parity question resurfaces. |

---

## Tier 8 — Archive (`docs/development/archive/`)

Frozen-by-design. Shipped proposals / superseded docs land here (don't edit; re-open is a `git mv` back).

| File | Status |
|---|---|
| `cyrius-linalg-proposal.md` | 📦 Shipped — `linalg` is now a cyrius stdlib module (shipped cyrius 4.10.2/4.10.3 per the doc's own header). Archived 2026-05-29. |

---

## Refresh procedure

When docs are touched:

1. Find the affected row in the relevant tier table.
2. Update **Last touched** to the new date.
3. Update **Status** if the bucket changed.
4. Update **Action** if the next step changed.
5. If a doc moved or was archived/deleted, update its row (and the removed-this-pass note).
6. Re-anchor the **Last refresh** date in the header.

When the bucket counts at the top drift by more than ~2 in any cell, refresh the at-a-glance table. Cadence is **opportunistic**, not periodic.

---

## What this file is NOT

- Not a CHANGELOG (which records what shipped, not what's stale).
- Not a TODO list (forward work lives in [`development/roadmap.md`](development/roadmap.md)).
- Not a per-doc review log (this is the ledger of where each doc stands, not the reasoning behind each).
- Not a substitute for the audit reports in `docs/audit/` (point-in-time security/correctness snapshots).

---

## Forward doc-policy commitments

Scheduled doc decisions, surfaced so they aren't forgotten when the trigger arrives.

| # | Commitment | Trigger | Notes |
|---|---|---|---|
| 1 | **Roadmap rotation — completed items are REMOVED, not struck through and not archived in-file** (changed 2026-09-11). `roadmap.md` is future-facing only | Every release | Per CLAUDE.md Work Loop §10-11. ⛔ **The old form of this commitment is retired**: it moved completed arcs into a **Release History table at the foot of the roadmap**, and that table is gone — it was a third copy of `CHANGELOG.md`, and maintaining three copies is how the `triangulate_polygon` direction stayed backwards for thirteen releases in the one copy no gate could reach. **One record, in `CHANGELOG.md`.** ⚠ The failure mode this rotation exists to prevent is unchanged and still real: at 1308 lines the roadmap was ~95% history, and the 2026-09-09 sweep found **18 of 39** of its open items stale — a backlog nobody can see the end of stops being read. Rotate at every closeout. |
| 2 | **Re-run `bench-history.sh` at release closeout** — keep `benchmarks.md` + `bench-history.csv` current; "numbers don't lie." | Every release with perf-relevant change | ✅ Done at every release of the 2.6.12–2.6.15 arc. **28** benchmarks (26 + the two collision hot paths, added *before* the 2.6.15 rewrites so the wins were provable). Both before/after rows are in `bench-history.csv` |
| 3 | **Re-verify tracked `issues/` filings at each toolchain bump** — re-test on the new pin; move resolved ones to `issues/archived/`. | Each `cyrius.cyml` pin bump | ✅ Done 2026-09-13 on 6.6.3 — both hisab-filed upstream issues FIXED and closed HERE (records written into `issues/archived/`), because the cyrius agent never edits this repo. Prior: 2026-08-03 on 6.5.6 — all 3 open issues still live (interval-ident-lex + for-empty-clauses by minimal repro; CLI-clobber not re-tested, destructive); the 6.4 → 6.5 minor fixed none. Prior: 6.4.69, 6.4.66, 6.3.11, 6.2.11 |
| 4 | **Periodic security audit** — full source scan before a major release or after significant surface change; supersede with a new dated `docs/audit/` doc. | Before 3.0.0; on significant change | ✅ 2026-08-03 → `audit/2026-08-03.md` (70 findings). ✅ **Re-audit done 2026-08-04 → `audit/2026-08-04.md`** (42 confirmed; regression-verification clean). ⏭ 2.7.0 must give **every numbered finding** an explicit disposition — the 2026-08-03 cycle closed on tier summaries and left six findings unscheduled. Also owes the SECURITY.md / threat-model.md memory-safety entries. |
| 5 | **Byte-check `lib/*.cyr` against the cyrius TAG (`git -C ~/Repos/cyrius show "<pin>:lib/<f>"`) at every pin bump** — never previous-pin against new-pin, and ⛔ **never against `~/.cyrius/versions/<pin>/lib/` either** (corrected 2026-09-13: the installed 6.6.2 dir held 6.6.3's stdlib; the install dir is mutable, the tag is not). | Each `cyrius.cyml` pin bump | Added at v2.6.11; reference changed to the tag at v3.0.1. The toolchain-vs-toolchain comparison used through 2.6.10 is blind to vendoring drift and hid a stale `ganita` 1.0.3 for three releases. `cyrius lib sync`, then `cmp` every file (excluding `sakshi.cyr`) against the pin; any difference is a defect, not a diff |
| 6 | **Run `./scripts/check-constants.sh` after touching any hand-encoded f64 constant** — it decodes every hex literal in `src/` and compares it to the value in its own comment. | Any constant change; runs in CI | **New at v2.6.12.** Seven constant tables shipped not encoding their documented values, including four published tableaux each falsified by its own invariant. 110/110 verified. The class is mechanical, so the guard is too. |
| 7 | **State an audit's SCOPE alongside its verdict** — say what was *not* examined. | Every audit / closeout | **New at v2.6.15, reinforced 2026-08-04.** The 2.5.4 and 2.6.5 closeouts concluded "posture solid" while scoped only to their own arc's new functions; the 2026-08-03 sweep then found 70 defects in the modules they never reached. The 2026-08-03 cycle then made the mirror-image error — declaring itself discharged on the strength of a tier summary rather than its finding list. **State scope, and close on the finding list.** |

---

*Initial scaffold: 2026-05-29 (v2.4.6), adapted from `cyrius/docs/doc-health.md`, immediately after the post-2.4.x documentation sweep. Same-day verify-and-cleanup pass: bench re-run, `tool-issues.md` retired, linalg proposal archived, CONTRIBUTING currency fixed, all 5 toolchain issues re-verified live on 6.0.14. Refresh in place when docs are touched.*

| `migration-3.0.md` | 2026-09-11 | ✅ Fresh | **NEW in v3.0.0**: the consumer-facing migration guide. Leads with the argument-position trap, because that is the one that is silent. |