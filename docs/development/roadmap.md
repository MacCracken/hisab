# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.5.16**. Stdlib `ganita` (6.2.x math umbrella) provides dense decompositions + transcendentals.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current — v2.9.2

Suite **3351** across five harnesses (hisab 416, foundation 349, modules 1621, edge_cases 233,
abuse 732), constant gate **153/153**, toolchain **6.5.16**, sakshi **2.4.10**. All gates green:
`lint` 0 warnings and `fmt <file> --check` 0 drift across all 43 sources, `vet` 2 deps / 0 untrusted
/ 0 missing, `deps --verify` 30/30, `fuzz` 1/0, `coverage` 588/591 functions (99%) over 35/35 files,
distlib in sync.

**2.9.0** delivered the narrowphase release — but read its section for what that actually means: the
capability shipped early in 2.8.3, and 2.9.0's real content was the tail that only became visible
once there were mechanisms to see it (an abuse harness, a finding-disposition column, a mutation
sweep on the integrated tree). **2.9.1** cleared the deferred tier: four latent predicate defects in
the ghost in-circle family, one of which was wrong on ordinary reachable input. **2.9.2** is a
maintenance release — toolchain 6.5.9 → 6.5.16, sakshi 2.4.8 → 2.4.10, **no library source change**,
the `dist/hisab.cyr` diff is the version header alone — and it is on this page for what the bump
surfaced, not for the bump.

**The standing lesson from this arc, stated once here so it is not rediscovered:** every defect of
consequence was cheap to find and invisible because nothing looked. The friction impulse had been
identically zero at every mu since before the 2026-08-04 audit, surviving because every solver
fixture used friction = 0. The delaunay ghost-direction invariant — the property a CRITICAL was
rebuilt around — had zero assertions. `cqr_decompose` overran its output buffer while returning a
success code. None needed cleverness; they needed something that looked.

**2.9.2 extended that lesson from the code to the gates.** Three of them were green while checking
nothing, and not one was found by a test failing: CI's version-consistency gate ran an unanchored
`grep -q "$VERSION" CHANGELOG.md`, and `2.9.2` matched `32,942,104 B` in a benchmark line, so a
release whose CHANGELOG section was never written would have passed; `src/main.cyr`'s hardcoded CLI
version string was compared against nothing; and `scripts/bench-history.sh` recorded the **max** of
each benchmark rather than the average, in every row it has ever written.

**FOUR separate measurements stated in committed text did not reproduce** during this arc, each
caught by an adversarial reader rather than a gate, and two had already propagated into three or
four files. `scripts/check-measurements.sh` closes that class and **is enabled** — it landed in
`ci.yml` on 2026-08-06 (`b7b229f`), pull-request-only and scoped to the claims a branch *adds*, with
a second step gating detector recall. What is not settled is whether to keep it — see 2.9.3.

## 2.6.12–2.6.15 -- Audit & repair arc  **CLOSED**

Retired from active work 2026-08-03. The full record lives in
[`../audit/2026-08-03.md`](../audit/2026-08-03.md) (all 70 findings), the CHANGELOG entries for
2.6.12–2.6.15, and the Release History table below. Kept here only as the one-paragraph summary
a future reader needs:

> A P(-1) sweep of the 2.6.11 tree — 8 parallel dimensions over all 34 modules, every finding
> adversarially re-verified — produced **70 confirmed findings (2 critical, 23 high, 23 medium,
> 22 low)** across 64 sites. All four tiers closed across four releases. The headline was
> systemic, not scattered: **seven hand-encoded IEEE-754 constant tables did not encode the
> values their own comments stated**, including four published tableaux each falsified by its own
> invariant — DOPRI45's Σb was 0.636, so it was **not a consistent integrator at any order**.
> Also closed: an SVD returning U transposed, an eigensolver that never converged for n ≥ 3,
> `einsum` mis-parsing its own documented examples then segfaulting, a `kdtree_build` that
> SIGSEGV'd at 60k coincident points, and two O(n²) hot paths (190 ms → 1.2 ms, 22 ms → 2.1 ms).
> **Every defect sat in a module with zero or near-zero coverage** — eight modules were not even
> compiled by `cyrius test`. Suite 957 → 1127; coverage → 59%; all 34 modules now under test.
> `scripts/check-constants.sh` was added as a CI gate so the constant class cannot recur.

**What the arc leaves behind as standing practice** (not work items — they are already in place):
- `scripts/check-constants.sh` runs in CI between the vet and distlib gates.
- Vendoring is byte-checked against the **pin's own snapshot**, not previous-pin vs new-pin.
- Test harnesses clamp `assert_summary()` before exiting, and use `sys_exit_group`.
- Fixes land with the assertion that would have caught them; perf claims need before/after rows
  in `bench-history.csv`.

---

## Release train

What each release **delivers to a consumer**. Defects are not roadmap items — they are tracked in
`../audit/2026-08-04-v2.8.0-full.md` and discharged as a **precondition** of the release they gate,
not as line items here. If a finding does not change what a consumer can rely on, it does not
appear on this page at all.

| Release | Deliverable | Gated on |
|---|---|---|
| ~~2.9.0~~ | A narrowphase a physics engine can build on | **RELEASED 2026-08-05** |
| ~~2.9.1~~ | The deferred tier — latent predicate defects | **RELEASED 2026-08-06** |
| ~~2.9.2~~ | The toolchain bump, and three gates that were not gating | **RELEASED 2026-08-09** |
| **2.9.3** | EPA certification + the measurement-gate decision | 2.9.2 |
| **2.10.0** | Differentiable geometry (autodiff through intersections) | 2.9.x |
| **2.11.0** | Reverse-mode autodiff (tape-based) | 2.10.0 |
| **3.0.0** | `Result<T,E>` API — breaking | 2.11.0 feature-complete |

---

## 2.9.0 — a narrowphase a physics engine can build on  **RELEASED 2026-08-05**

All five exit criteria met. Full record in the CHANGELOG and
[`../audit/2026-08-04-v2.8.0-full.md`](../audit/2026-08-04-v2.8.0-full.md), which now carries a
Disposition column (36 FIXED / 6 OPEN / 0 REFUTED of 42, plus an addendum row).

> Scoped as "make the narrowphase trustworthy"; that shipped in 2.8.3 (exact MTV, 862/862 against a
> reference in exact rationals, worst relative error 1.6e-9). What 2.9.0 actually delivered was the
> tail: `tests/abuse.tcyr` (732 assertions, **11 real defects on public entry points**, headed by
> `cqr_decompose` overrunning `out_R` while returning `HSB_ERR_NONE`); the disposition column, which
> on its first pass caught `sequential_impulse` marked FIXED when only its restitution half was —
> the friction impulse was identically zero at every mu; and a 54-mutation sweep on the INTEGRATED
> tree, which found the delaunay ghost-direction invariant had no assertions at all. Suite
> 1818 → 3294. The entire low tier of the audit, 5 of 5, turned out never to have been scheduled.

## 2.9.1 — the deferred tier  **RELEASED 2026-08-06**

Everything 2.9.0 found and consciously left. Suite 3294 → **3351**, toolchain 6.5.8 → 6.5.9.

> Four latent defects in the ghost in-circle family, all found by deriving an exact-rational oracle
> rather than by a failing test: `_col_dl_ic_g2`'s tie branch applied winding twice; `_col_dl_ic_g1`'s
> M¹ tie-break vanished whenever the edge was parallel to `u_k` and could not separate "between a
> and b" from "beyond b" — **the only one wrong on reachable CCW input**, 228 of 463,086 — replaced
> by a plain betweenness test; `_col_dl_ic_g3` reduced to the constant `1` with two independent
> proofs. `mpr_intersect`/`mpr_penetration` were re-measured *before* being touched, and the
> re-measurement changed the answer: they were still running the pre-2.9.0 containment test, so
> 2.9.0 had repaired one call site of that logic and left two. `_col_in_circumcircle` re-measured
> at 0 wrong of 1,528,820 and deliberately left alone.

## 2.9.2 — the toolchain bump, and three gates that were not gating  **RELEASED 2026-08-09**

Toolchain 6.5.9 → **6.5.16** (seven releases) and sakshi 2.4.8 → **2.4.10**. No library source
change — the `dist/hisab.cyr` diff is the version header alone and the suite holds at 3351. Ten
vendored stdlib files moved (`alloc`, `args`, `fnptr`, `io`, `sakshi`, `tagged`, four `syscalls_*`
platform variants) plus the transitive `result.cyr`; all 30 byte-match the 6.5.16 snapshot,
`deps --verify` 30/30.

> The bump was uneventful; what it surfaced was not. Crossing **6.5.14** turned `cyrius distlib` red
> on a correct bundle — the new self-check compiles the bundle alone under `_cc_allow_undef`, which
> suppresses undefined *functions* and cannot reach an undefined *name* because those hard-exit in
> the frontend, and hisab's bundle legitimately reads the stdlib constant `F64_ONE`. Both `ci.yml`
> and `release.yml` were RED; they now tolerate exactly that signature and add
> `cyrius check --with-deps dist/hisab.cyr`, which is the stronger check the broken one was reaching
> for ([filed](issues/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md)). Chasing it
> reached the benchmark harness, where `scripts/bench-history.sh` took "the last `<num><unit>` in the
> line" from a format that ends in `max=`, so **44 of 55** rows — every row it has ever written —
> track the max rather than the average. The parser now anchors on the harness's named fields and the
> CSV carries `stat`/`avg_ns`/`min_ns`/`max_ns`/`iters`; `benchmarks.md` compares only rows sharing
> the newest run's `stat`, at ±10% rather than ±3%, because ±3% sits below the measured noise floor
> (same binary, three back-to-back runs: avg spread median **3.5%**, worst 6.9%, 0 of 55 above 20% —
> against −93%..+742% swings on the max-based rows). **No performance claim is made in 2.9.2**; it
> establishes the first correct baseline, and the next release is the first that can compare against
> it. Also landed: `version-bump.sh` now rewrites `src/main.cyr`'s version string and CI asserts it
> and the bundle header against `VERSION`, and `dist/hisab.deps` is tracked so a consumer's
> `cyrius deps` auto-resolves hisab's 15 stdlib leaves.

## 2.9.3 — what 2.9.1 and 2.9.2 left, and why

Three items, each with a stated reason for not being done sooner rather than a shrug. They stay in
the 2.9.x line rather than folding into 2.10.0: certification changes the value returned on every
overlapping pair, and 2.10.0 differentiates *through* those same intersection routines, so it wants
a settled narrowphase underneath it rather than a concurrent one.

- [ ] **The EPA certified exit is never taken.** Measured 0 of 24 overlaps across boxes and spheres;
      `strict` is the sole blocker (removing it alone takes the fallback count 12 → 0). It tests the
      **GJK seed**, but the lower-bound argument is about the polytope **at certification time**, and
      EPA only ever adds vertices. This also explains why six EPA repairs were unmutatable in 2.9.0 —
      the exit they improve is unreachable. **Not fixed in 2.9.1 deliberately:** certification changes
      which value is returned on every overlapping pair, so it needs the 862-configuration exact-MTV
      harness and a benchmark, not a patch release.
      → [`issues/2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md`](issues/2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md)
- [ ] **Keep `scripts/check-measurements.sh`, or delete it.** The gate itself is no longer the open
      question: it landed in `ci.yml` on 2026-08-06 (`b7b229f`) as a **pull-request-only** job that
      checks the claims a branch *adds* (`--diff origin/<base>`, with `fetch-depth: 0` so an
      unresolvable base fails rather than passing vacuously), plus a `--selftest` step gating
      detector recall — currently **21/22** on the planted corpus, **0/15** decoys falsely flagged.
      What is unsettled is whether it earns its place. False positives are ~21% paragraph-level, and
      a full-tree scan still reports **522** unmarked pre-existing claims across 26 files (287
      paragraphs), so it cannot be widened past the diff without a marking campaign first. It has
      earned its keep once — it caught the fourth non-reproducing measurement of the arc. The
      decision is whether ~21% FP is tolerable on a gate people must not learn to ignore.
- [x] **`_col_dl_ic_g1`/`g3` are no longer CCW-only — `_col_dl_incircle` is order-free.** This item
      previously read *"remain CCW-only at the entry point, by design. Documented, not a defect."*
      That framing did not survive contact: `g3` became the constant `1` in 2.9.1 at no cost, which
      left the whole question resting on `g1` alone, and `g1`'s share turned out to be **one cross
      product on the ghost path**. Landed in 2.9.3 with a 1800-probe order-freeness harness and four
      correctness assertions; four mutants caught; no cost measurable above the noise floor. A
      symmetric predicate whose answer depends on argument order is the same defect class 2.9.1
      rejected "keep the strict reading and document it" for in `mpr_intersect`.
      → [`issues/2026-08-06-ghost-incircle-g1-g3-assume-ccw.md`](issues/2026-08-06-ghost-incircle-g1-g3-assume-ccw.md)

**Performance, demand-gated** — ~~`delaunay_2d`, `_kd_partition`, `triangulate_polygon`~~. **This
line was wrong on all three, and 2.9.3 closed it.** It described them as filed-but-unapplied
optimisations; in fact `_kd_partition` and `triangulate_polygon` **shipped in 2.7.1** (a balance
guard, and reflex-only ear pruning at 9.93 → 1.59 ms) and `delaunay_2d`'s record was **superseded by
the 2.8.0 adjacency rewrite**, which deleted the O(n²) rescan it proposed removing. What was
genuinely open was never the code:

- the `_kd_partition` balance guard had **no regression assertion** — `kdtree_build != 0` and the
  split-plane invariant both hold on a degenerate spine, so deleting the guard broke nothing that
  was checked. Closed in 2.9.3, mutation-proven, on an octave-spaced fixture. ⚠ The pre-existing
  geometric fixture does **not** exercise it (ratio 0.99998 → axis range [0.85, 1], the guard never
  fires); that is recorded inline so it is not mistaken for the guard's test.
- `_col_point_in_tri` documented itself as "(2D, strict interior)" while being **boundary-inclusive**
  — which is what ear clipping needs, so the code was right and only the contract was mis-stated.
  Closed in 2.9.3 with five assertions.

**Also closed in 2.9.3**: `2026-08-04-incircle-precision.md`, which had been recorded as
"leave it" since 2.9.1. It was not a property of the predicate in isolation — `delaunay_2d` silently
dropped input points on any set mixing scales. Fixed with an adaptive exact `orient2d` on raw
coordinates; the record of what the 2.9.1 closure got wrong, and of the repair attempt that did not
work, is in the archived filing.

**Still open from that group** (documentary, carried forward): the −84% `triangulate_600gon`
headline does not disclose a small-n regression below the prune's break-even, and there is no
reflex-heavy benchmark guarding the prune.

- [ ] **`einsum` is the only arena consumer in the library and has no benchmark at all.**
      `src/einsum.cyr` holds every `arena_*` call in `src/` — `arena_new` at `:41`, `arena_reset` at
      `:42`, and 11 `arena_alloc` calls — and `tests/hisab.bcyr` does not include the module, so none
      of the 55 benchmarks touches the arena path. Found while separating the 6.5.10 allocator change
      from the compiler change at the 2.9.2 bump: measuring hisab's exposure to it needed a
      purpose-built micro-benchmark because nothing in the suite would have shown a regression there.
      Cheap to close and worth closing before any allocator-adjacent change lands, per the standing
      rule that a perf change needs a number to move.

**Toolchain**, tracked upstream, all revisited at the 6.5.16 bump (2026-08-09):
`cyrius-for-empty-clauses` (`for (; c; s)` → `unexpected ';'`) is **still live** in substance — both
forms still reject — though the misleading-line-number half of the complaint is fixed, so the `while`
rewrites stay. `cyrius-cli-arg-clobbers-source` was **deliberately not re-tested**: its reproducer
truncates a real `src/*.cyr` to 0 bytes by construction, so it is recorded as unverified rather than
assumed-still-live. `cyrius-interval-ident-lex` is **closed and archived** — 67 of 67 reserved names
emit the new wording, none the old — while the reservation itself stands by design, so the
`iv_sum`/`iv_diff`/`iv_prod` rename in `tests/modules.tcyr` remains load-bearing. Two new filings
this release: [`distlib`'s self-check rejecting stdlib globals](issues/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md),
worked around in both workflows, and
[a syntax error inside an uncalled function passing every gate](issues/2026-08-09-cyrius-dead-fn-bodies-are-never-syntax-checked.md).

## 2.10.0 — differentiable geometry

Autodiff through ray–surface intersection, so consumers can optimise *through* geometry rather than
around it. The forward-mode dual infrastructure already ships (`autodiff.cyr`); this is the geometry
integration and the chain-rule coverage for `geo_ray_*`.

Driver: aethersafha (compositor) and the differentiable-rendering use case. Blocked on 2.9.0 only
because it consumes the same intersection routines.

## 2.11.0 — reverse-mode autodiff

Tape-based reverse mode for gradients of many-input scalar objectives, where forward-mode's
per-input cost is the wrong shape. Pairs with `optimize.cyr`'s solvers, which currently require a
caller-supplied gradient.

## Optional, demand-gated

- **GPU compute via soorat** (feature-gated) — no consumer has asked; parked until one does.
- **Adopt `vec_sort_by` / `vec_select_nth`** (cyrius 6.5.4) — consolidation onto stdlib, not a fix.
  Blocked on an API mismatch: the comparator receives element *values*, hisab sorts *indices* by
  dereferencing a separate vector, and Cyrius has no closures. 2.6.15 already fixed the *complexity*
  of both hot sorts, so this is now optional rather than load-bearing.
- **SIMD the flat-array kernels** — `_opt_dot`/`_opt_norm`/`_opt_axpy`, the L-BFGS sweeps,
  `_lext_dot`/`_lext_norm`. 5–8× on comparable kernels in 2.3.1. **Needs a benchmark per kernel
  first** — none is benchmarked, so there is nothing to prove a win against.

---

## 3.0.0 -- Error-handling migration (breaking)

The integer-error-code convention (`src/error.cyr`: functions return 0 / a
negative `ERR_*` code) predates the stdlib `Result<T,E>` (`lib/result.cyr`,
v5.8.28) and `?` propagation (v5.8.29). Migrating is a library-wide signature
change — breaking for consumers (impetus, kiran, joshua, …) — so it lands as
a major, with a migration guide, not a 2.x patch.

- [ ] Wrap fallible returns in `Result<T,E>` (keep `ERR_*` codes as the `E` payload)
- [ ] Adopt `?` to replace manual `-1`-return + check chains
- [ ] Migration guide + deprecation window for the old integer-code API

---

## Parked / deferred (revisit when a driver appears)

Evaluated during earlier arcs and consciously deferred — recorded so they
aren't silently lost (full rationale in the CHANGELOG). Items that have since **shipped** are
removed from this list rather than struck through; their record lives in the CHANGELOG and the
Release History table below.
- **SIMD `cross` / `lerp`** (from 2.3.1) — need lane shuffles, not a clean `f64v_*` fit. Revisit if Cyrius adds packed shuffles.
- **`#pure` annotations** (from 2.3.4) — unsafe CSE interaction with hisab's allocate-a-fresh-result convention; speculative perf, no driver.
- **Slices (`[T]` / `slice<T>`)** (from 2.3.4) — would regress the proven raw-pointer SIMD hot paths; `slice_unchecked_get_W` discards the safety benefit.
- **`defer`** (from 2.3.4) — N/A under the bump/arena model (no per-resource lifecycle to clean up).

*Retired 2026-08-03:* **stdlib `mat_new` overflow guard** (parked from 2.5.3) — shipped in
**2.6.11** via ganita 1.0.4 at the 6.5.6 pin. `mat_new_guarded` is retained as the stricter 16M
entry point, so nothing further is owed.

---

## 2.7.0 -- Re-audit & refactor  **RELEASED 2026-08-04**

Closed the 2026-08-04 re-audit: 35 of 41 findings fixed, per-finding disposition in
[`../audit/2026-08-04.md`](../audit/2026-08-04.md). Suite 1127 → 1605. Its two carried items are
both discharged — `delaunay_2d` was rewritten in 2.8.0/2.8.2 and `_col_in_circumcircle` was
re-measured in 2.9.1 at 0 wrong of 1,528,820 and left deliberately.

> Four defects were found by *checking that a repair preserved behaviour*, not by any audit — the
> largest being `kdtree_within_radius` returning wrong counts on 274 of 400 queries after three
> releases. Two findings had never been scheduled at all. Three confirmed performance wins were
> **withheld** because each regressed on some input class.

## Consumers

None are live yet — all four come online later, which is why 2.9.0 set the struct-layout contract
while nothing depends on it. **The contract: construct via the documented constructor, read via the
accessors, size arrays with `sizeof(T)`. Never a hardcoded byte count, never a hand-computed
offset.** Layout is an implementation detail and does move — `ColContact` went 64 → 72 bytes in
2.9.0 — and 32 assertions across 16 public structs now make any such change trip a gate.

| Consumer | Domain | Surface it will use |
|----------|--------|---------------------|
| **impetus** | physics | GJK/EPA, MPR, PGS, sequential-impulse, inertia, spatial |
| **kiran** | engine | projections, BVH, k-d tree, frustum |
| **joshua** | simulation | DOPRI45, BDF, symplectic, optimize |
| **aethersafha** | compositor | projections, compositing, color |
| **abaco** | expression eval | symbolic integrate/LaTeX/patterns, interval |
| **svara** | vocal synthesis | complex, FFT, easing |
| **hisab-mimamsa** | physics | tensors, Lie groups, diffgeo, CGA |
| **kana** | quantum | tensors, Lie groups, complex LA, spinors |

**Known caveat to carry into first integration:** `gjk_intersect_3d` costs ~+55% on the no-hit path
since 2.9.0 — the price of it no longer missing 134 genuine interior overlaps per 4,386 evaluations.
The no-hit path is the broadphase-common case, so if a consumer finds that cost unacceptable the
answer is a cheaper pre-filter, not reverting the correctness fix.

---

## Release History

| Version | Date | Lines | Files | Highlights |
|---------|------|-------|-------|-----------|
| 2.9.2 | 2026-08-09 | 21,262 | 35 | **The toolchain bump, and three gates that were not gating.** Toolchain 6.5.9 → **6.5.16** (seven releases) + sakshi 2.4.8 → **2.4.10**. No library source change — the bundle diff is the version header alone; all 30 vendored files byte-match the 6.5.16 snapshot, `deps --verify` 30/30. `scripts/bench-history.sh` recorded the **max** of each benchmark, not the average, in **44 of 55** rows and in every row it has ever written — the parser scavenged the last `<num><unit>` from a line that ends in `max=`. Re-anchored on the harness's named fields, CSV gains `stat`/`avg_ns`/`min_ns`/`max_ns`/`iters`, and `benchmarks.md` compares only same-`stat` rows at ±10% — the measured noise floor is a 3.5% median avg spread over three back-to-back runs of one binary, against −93%..+742% swings on the max rows. **No performance claim in this release**: it is the first correct baseline. CI's version gate was an unanchored `grep` — `2.9.2` matched `32,942,104 B` — and `src/main.cyr`'s hardcoded CLI string was checked by nothing; both now assert against `VERSION`. cyrius 6.5.14's `distlib` self-check rejects any bundle reading a stdlib constant (hisab's reads `F64_ONE`), so `ci.yml` and `release.yml` were both RED; now tolerated by exact signature plus `cyrius check --with-deps`. `dist/hisab.deps` tracked, 15 stdlib leaves. 3351 |
| 2.9.1 | 2026-08-06 | 21,262 | 35 | **The deferred tier.** Four latent ghost in-circle defects, all found by deriving an exact-rational oracle rather than by a failing test. `_col_dl_ic_g1`'s M^1 tie-break was the only one wrong on REACHABLE CCW input (228 of 463,086) — it vanished whenever the edge was parallel to `u_k` and could not separate "between a and b" from "beyond b"; replaced by a betweenness test. `_col_dl_ic_g2`'s tie branch applied winding twice; `_col_dl_ic_g3` reduced to the constant 1 with two independent proofs. `mpr_intersect`/`mpr_penetration` re-measured BEFORE being touched, which changed the answer — they were still running the pre-2.9.0 containment test. A FOURTH non-reproducing measurement (the "36%" claim, actually ~28% and distribution-dependent) corrected across four files. Toolchain 6.5.8 → 6.5.9. 3351 |
| 2.9.0 | 2026-08-05 | 21,094 | 35 | **A narrowphase a physics engine can build on.** All five exit criteria met. Scoped as the narrowphase repair, which had already shipped in 2.8.3 — the real content was the tail that mechanisms exposed: `tests/abuse.tcyr` found **11 defects on public entry points** (`cqr_decompose` overran `out_R` while returning `HSB_ERR_NONE`); a disposition column caught `sequential_impulse` marked FIXED when the friction impulse was identically zero at every mu; a 54-mutation sweep on the integrated tree found the delaunay ghost-direction invariant had **zero** assertions. `gjk_intersect_3d` stopped missing 134 genuine interior overlaps (+55% no-hit cost, stated). Struct-layout contract set. The audit's entire low tier, 5 of 5, had never been scheduled. 1818 → 3294 |
| 2.8.4 | 2026-08-05 | 20,778 | 35 | DCT/DST dispatch heuristic. The 23x `num_dst_1024` gap was **not a defect**: 1.6x is DST-I's irreducible 2(n+1) DFT and 14.6x is that n = 1024 makes that extension non-power-of-two. Sibling sizes added to the bench so the parity cliff is visible. The real defect was `_numx_use_fft` billing DST-I for post-processing transcendentals it never evaluates — four sizes dispatched to the slower path. |
| 2.8.3 | 2026-08-05 | 20,437 | 35 | **The eight-track audit-repair merge.** Exact MTV (`mpr_penetration` 66/455 with 216 wrong signs → 862/862 against a reference in exact rationals, worst rel. error 1.6e-9); `time_of_impact` fixed-step sampler → real conservative advancement; `gjk_epa_3d` out-param contract on every return path. Two tracks solved the same problems incompatibly and only one of each could land. Toolchain 6.5.6 → 6.5.8. 1818 → 2263 |
| 2.6.15 | 2026-08-03 | 17,100 | 34 | **P3 + P4 closeout — the 2026-08-03 audit is fully discharged.** Both O(n^2) hot paths rewritten, benchmarked before and after (rows in `bench-history.csv`): `halfedge_from_triangles` twin pairing 190.1 ms -> **1.2 ms** (-99.4%) via an open-addressed hash of directed edges, and `convex_hull_2d`'s insertion-sort pre-pass 22.1 ms -> **2.1 ms** (-90.3%) via heapsort (chosen over merge sort because its scratch would leak per call under the bump allocator; the comparator is inlined because Cyrius has no closures). Both verified output-identical to the old code. Levenberg-Marquardt's 4 loop-invariant buffers hoisted and its symmetric J^T J computed once; L-BFGS's 3 per-iteration buffers hoisted. Six documentation drifts corrected (BVH mis-attributed to `spatial.cyr`, Fletcher-Reeves vs Polak-Ribiere+, DST-II vs DST-I, `expr_eval` "aborts", `hodge_star_2form_4d`'s three contradictory `sign` docs, `collision_core`'s header naming a deleted file). **`num_ext` + `symbolic_ext` brought in — all 34 modules are now covered by a suite**; coverage 57% -> **59%**, files 34/35. 1127 |
| 2.6.14 | 2026-08-03 | 17,000 | 34 | **P1 closeout — the memory-safety tier.** Three of the four defect groups **crashed the process** pre-fix: reverting `complex.cyr`, `calc_ext.cyr` or `optimize.cyr` individually makes the suite exit 139. Capped constructors (`cmat_mul`, `cmat_kronecker`, `cmat_identity`, `cmat_inverse`) stored through `cmat_new`'s documented 0-on-failure return — reachable from operands *under* the cap, since `cmat_mul` of 1000×1 by 1×1000 asks for 1e6 and `cmat_inverse` doubles the width. `opt_bfgs`/`opt_lbfgs` sized `n×n` / `m×(2n+1)` buffers from an unbounded dimension (added `_OPT_MAX_DIM`, alloc checks, `HSB_ERR_ALLOC`). `calc_bspline`/`calc_nurbs` indexed control points **negatively** when `n_pts <= degree`. Adaptive Simpson bounded depth but not work (2^50 evaluations on a NaN integrand). `kdtree_build` recursed O(n) deep on coincident points — fine at 40k, **SIGSEGV at 60k**. `_opt_armijo` accepted NaN. `cx_div`/`cx_inv`/`cx_powf` used a 1e-6 cutoff instead of 1e-12 (squared vs unsquared tolerance). `FLOAT_RENDER_BUF` was 32 against a scratch reach of 45. Four fBm entry points returned NaN for `octaves <= 0`. **+`calc_ext`, `noise_simplex` into the suites**; coverage 54% → **57%**. 1093 |
| 2.6.13 | 2026-08-03 | 16,900 | 34 | **P0 closeout** of the 2026-08-03 audit — every remaining critical/high correctness defect, all in modules with **zero test coverage**. `svd_golub_kahan` composed the left Householder reflectors forward, returning U **transposed** so `A != U·S·Vᵀ` (8 of 9 entries wrong); now accumulated backward. `eigen_qr` applied `Gᵀ T G` instead of `G T Gᵀ` — the transpose of the rotation that zeroes the bulge — so it **never converged for n ≥ 3** (NO_CONVERGENCE on a symmetric 3×3 at 100k iters) and paired wrong eigenvectors at n = 2. `su2_exp`/`su2_log` moved to half-angle, restoring `su2_to_rotation_matrix(su2_exp(ω)) == so3_exp(ω)` and fixing `se3_exp`, which had built R and t from angles a factor of 2 apart. `einsum` accepted only labels `a`–`h`, so **every example in its own header** was silently mis-parsed — it returned the trace (5, not 19) then segfaulted; alphabet widened to `a`–`z` with real validation. Three interval enclosure-soundness violations fixed (`ivl_sin` under-approximated across extrema, `ivl_sqrt` returned `[0,NaN]` which `ivl_contains` treated as universal, `ivl_div` missed `-0.0`). **4 of the 8 never-tested modules brought into the suites** (`einsum`, `lie_ext`, `mat3`, `linalg_precision`); coverage 50% → **54%**. 1063 |
| 2.6.12 | 2026-08-03 | 16,600 | 34 | **Audit sweep — repair release.** Full P(-1) audit of all 34 modules found **70 verified defects (2 critical)**; see `audit/2026-08-03.md`. Closed the critical tier: **seven hand-encoded constant tables** did not encode their documented values — **47 constants re-derived** from exact rationals. DOPRI45 had 23 of 30 tableau constants wrong (Σb = 0.636, not 1) and was **not a consistent integrator at any order**; BDF-4 drifted 1%/step; Yoshida-4 moved a free particle 85.9% of the correct distance; Gauss-5 carried a 246 ppm error floor; plus sRGB breakpoints, spherical harmonics, `_SIMPLEX_G2`, the slerp threshold. Also fixed `num_is_prime` (i64 overflow reported real primes composite above 3.03e9) and `solve_bicgstab` (`f64_from` on a bit pattern made the tolerance 4.34e18 — it never iterated). Added **`scripts/check-constants.sh`**, a CI gate verifying all 110 hex f64 literals against their comments. Replaced the loose assertions that let it all ship (dopri45 asserted only `1 < y < 2`). 981 |
| 2.6.11 | 2026-08-03 | 16,600 | 34 | Toolchain 6.4.69 → **6.5.6** (a **minor** jump across 24 releases) + sakshi 2.4.6 → **2.4.7**. No executable library change — the bundle diff is the version header plus one `mat_new_guarded` doc comment, zero code lines. **Security:** the vendored `lib/ganita.cyr` was stale at **1.0.3** (the 6.4.69 pin already shipped 1.0.4); re-vendoring closes the tracked CWE-190 in stdlib `mat_new` — `mat_new(-5, 3)` **segfaulted** on 1.0.3, returns null on 1.0.4 (measured, same compiler, only ganita swapped). **Fixed:** all four `.tcyr` harnesses exited with `assert_summary()`'s raw failure count, so exactly 256/512/768 failures truncated to 0 and scored PASS — now clamped. Adopted the 6.5.6 `sys_exit_group` epilogue. `cyrius fuzz` now discovers `tests/*.fcyr` (1 passed — previously never run). Smoke string 2.6.10 → 2.6.11. Tracked issues re-verified still-live (interval-ident-lex, for-empty-clauses). +4 assertions pinning the upstream `mat_new` contract. 961 |
| 2.6.10 | 2026-07-21 | 16,600 | 34 | Toolchain 6.4.66 → **6.4.69** (clean 3-patch bump; sakshi unchanged at 2.4.6, already latest). No library source change — bundle byte-identical bar the header. Vendored stdlib picks up three upstream fixes: `fmt` hex-high-bit + `fmt_float_buf` non-finite guard (linked by `symbolic`; byte-identical for finite values), `math` float-parse DoS hardening, agnos-only `sys_reboot` widening. Smoke string 2.6.9 → 2.6.10. Tracked issues re-verified still-live (interval-ident-lex, for-empty-clauses); no new fixes. 957 |
| 2.6.9 | 2026-07-17 | 16,600 | 34 | Toolchain 6.3.11 → **6.4.66** + sakshi 2.4.2 → **2.4.6**. Infrastructure + test-only fix — no library source change; bundle byte-identical bar the header. Fixed a pre-existing `tests/modules.tcyr` compile failure (`iv_add`/`iv_sub`/`iv_mul` collide with reserved cycc SIMD intrinsic names; renamed `iv_sum`/`iv_diff`/`iv_prod`), restoring the suite to 312/312. Smoke string 2.6.7 → 2.6.9. New interval-ident-lex issue filed; for-empty-clauses still open. 957 |
| 2.6.8 | 2026-07-06 | 16,600 | 34 | Collision hardening for co-compilation with the sandhi/TLS stack: `symbolic` float-render scratch moved `var buf[N]` → `alloc(N)` (dodges the "array size must be enum constant" path under `tls`/`dynlib` co-compile); bare error constants namespaced `ERR_*` → `HSB_ERR_*` (values unchanged) to stop a last-wins global collision on consumers. 957 |
| 2.6.7 | 2026-06-30 | 16,600 | 34 | Toolchain 6.2.11 → **6.3.11** + sakshi 2.1.0 → **2.4.2**. Infrastructure-only — no library source change; bundle byte-identical bar the header. `lib/result.cyr` `_die` agnos-portability fix; smoke version string 2.3.3 → 2.6.7. for-empty-clauses still open on 6.3.11 (no new fixes). 957 |
| 2.6.6 | 2026-06-15 | 16,600 | 34 | Toolchain 6.0.14 → **6.2.11**. Stdlib math reorg: transcendentals + matrix/linalg → new `ganita` umbrella; `math` gains NaN-correct `f64_le`/`f64_ge` (dropped local copies). `[deps]`: +ganita −matrix −linalg. 3 of 5 tracked toolchain bugs fixed (archived). 957 |
| 2.6.5 | 2026-05-30 | 16,600 | 34 | Diffgeo arc COMPLETE — P(-1)/security audit (posture solid) + `math.md §2` differential-geometry reference. Docs-only, 957 |
| 2.6.4 | 2026-05-29 | 16,600 | 34 | Diffgeo arc — higher-order forms (`wedge_2_1`/`wedge_3_1`); 8 wedge antisymmetry/grading assertions. 957 |
| 2.6.3 | 2026-05-29 | 16,580 | 34 | Diffgeo arc — geodesic deviation / Jacobi (`geodesic_deviation`); 6 sphere/flat/linearity assertions. 949 |
| 2.6.2 | 2026-05-29 | 16,560 | 34 | Diffgeo arc — parallel transport (`parallel_transport`, RK4); 4 flat/sphere length-preservation assertions. 943 |
| 2.6.1 | 2026-05-29 | 16,540 | 34 | Diffgeo arc — Weyl conformal-curvature tensor (`weyl_tensor`); 5 space-form/trace-free assertions. 939 |
| 2.6.0 | 2026-05-29 | 16,520 | 34 | Diffgeo arc — sectional curvature (`sectional_curvature` from Riemann); 5 space-form/sphere assertions. 934 |
| 2.5.4 | 2026-05-29 | 16,500 | 34 | CGA arc closeout — P(-1)/security audit (posture solid) + `architecture/math.md` equation catalogue. Docs-only, 929 |
| 2.5.3 | 2026-05-29 | 16,500 | 34 | CGA arc — `mat_new_guarded` (CWE-190 real-matrix guard); 4 assertions. 929 |
| 2.5.2 | 2026-05-29 | 16,490 | 34 | CGA arc — blade projection/rejection (`cga_project`/`cga_reject` + blade inverse); 10 assertions. 925 |
| 2.5.1 | 2026-05-29 | 16,480 | 34 | CGA arc — dual + pseudoscalar inverse (`cga_pseudoscalar`/`cga_dual`); 6 GA-identity assertions. 915 |
| 2.5.0 | 2026-05-29 | 16,470 | 34 | CGA arc — contraction operators (`cga_left_contraction`/`cga_right_contraction`); 8 GA-identity assertions. 909 |
| 2.4.6 | 2026-05-29 | 16,460 | 34 | Security/hardening audit — posture solid, no new vuln; 6 alloc-guard tests + threat-model refresh. 901 |
| 2.4.5 | 2026-05-29 | 16,460 | 34 | Collision arc COMPLETE — contact solver fixed (impulse was always 0); solve_pgs verified; 7 assertions. 895 |
| 2.4.4 | 2026-05-28 | 16,460 | 34 | Collision arc — MPR narrowphase fixed (separated pairs were false +ve); 10 assertions. 888 |
| 2.4.3 | 2026-05-28 | 16,450 | 34 | Collision arc — half-edge mesh audited (no bug; twin/boundary wiring correct); 11 assertions. 878 |
| 2.4.2 | 2026-05-28 | 16,450 | 34 | Collision arc — `delaunay_2d` audited (no bug; cocircular-robust); 8 empty-circumcircle assertions. 867 |
| 2.4.1 | 2026-05-28 | 16,450 | 34 | Collision arc — `triangulate_polygon` audited (no bug); 13 tiling/count assertions added. 859 |
| 2.4.0 | 2026-05-28 | 16,450 | 34 | Collision arc — `convex_hull_2d` fixed (broken insertion sort + undefined `f64_le`/`f64_ge`); 13 assertions added. 846 |
| 2.3.4 | 2026-05-28 | 16,424 | 34 | Layout/idiom modernization — `alloc(sizeof(T))`+derived setters (13 modules), enum-const grid/buffer sizes, `#must_use` on core API. Codegen-identical, 833/833 |
| 2.3.3 | 2026-05-28 | 16,195 | 34 | Safety/numerical audit — no bugs; fixed wrong `>>` comment + 8 invariant tests. 833/833 |
| 2.3.2 | 2026-05-28 | 16,195 | 34 | Bounded einsum scratch via reused arena — 3960 → 176 B/call (~22×). Memory-only, 825/825 |
| 2.3.1 | 2026-05-28 | 16,195 | 34 | SIMD hot paths (`f64v_*`) for vec/mat/quat — vec4 dot 6.5×, m4_mul 4.5×, m3_mul 3.2×. Bit-identical, 825/825 |
| 2.3.0 | 2026-05-28 | 16,195 | 34 | Cyrius 6.0.14 toolchain; library source moved to `src/`; sakshi resolution repaired; CI aligned to abaco (fmt/security/version gates). No behavioral change |
| 2.2.0 | 2026-04-15 | 15,676 | 33 | SE(3), SO(3), adjoint, BCH, spatial structures, MPR, impulse solver, simplex noise, einsum, Golub-Kahan SVD |
| 2.1.0 | 2026-04-15 | 13,715 | 30 | Golub-Kahan SVD, QR eigen, complex QR, simplex noise, einsum |
| 2.0.0 | 2026-04-15 | 11,943 | 27 | Cyrius port from Rust. P(-1) audit. |
| Rust 1.4.0 | 2026-03-30 | 33,612 | 65 | Final Rust release. Available via pre-2.0 git tags. |

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
