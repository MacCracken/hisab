# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.5.17**. Stdlib `ganita` (6.2.x math umbrella) provides dense decompositions + transcendentals.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current — v2.11.0

Suite **3507** across five harnesses (hisab 416, foundation 349, modules 1775, edge_cases 233,
abuse 734), constant gate **158/158**, **72** benchmarks, **35** `[lib]` modules, toolchain
**6.5.18**, sakshi **2.4.10**. All gates green:
`lint` 0 warnings and `fmt <file> --check` 0 drift across all 44 sources, `vet` 2 deps / 0 untrusted
/ 0 missing, `deps --verify` 30/30, `fuzz` 1/0, `coverage` 640/644 (99%) functions over 36/36 files,
distlib in sync.

**Per-release detail lives in `CHANGELOG.md`**, and the one-line-per-version record is the Release
History table at the foot of this file. What belongs *here* is the part that generalises.

### The standing lesson of the 2.7.0–2.11.0 arc

**Every defect of consequence was cheap to find and invisible because nothing looked.** The friction
impulse had been identically zero at every mu since before the 2026-08-04 audit, surviving because
every solver fixture used friction = 0. The delaunay ghost-direction invariant — the property a
CRITICAL was rebuilt around — had zero assertions. `cqr_decompose` overran its output buffer while
returning a success code. None needed cleverness; they needed something that looked.

**2.9.2 extended that from the code to the gates.** Three were green while checking nothing, and not
one was found by a test failing: CI's version gate ran an unanchored `grep` that matched a byte
count in a benchmark line, so a release with no CHANGELOG section would have passed; `src/main.cyr`'s
CLI version string was compared against nothing; and `bench-history.sh` recorded each benchmark's
**max** rather than its average, in every row it had ever written.

**2.10.0 onward turned it into a procedure that keeps paying.** Run the feature's own correctness
check against the code that already ships, *before* writing the feature — it found a defect in four
consecutive releases:

| release | the check | what it found in shipped code |
|---|---|---|
| 2.10.0 | degree −1 homogeneity in the ray direction | `geo_ray_sphere` returning a hit point 3.74 from the centre of a unit sphere |
| 2.10.1 | "what does the primal return when there is no face?" | `geo_ray_aabb`/`_obb` returning `+Inf`; a squared-vs-unsquared epsilon **2.10.0's own repair introduced** |
| 2.10.2 | the hit point must lie on the box | a scale-free slab test hitting at x = 1.4 for a box spanning [0,1] with a **unit** direction |
| 2.11.0 | reverse mode is validated against forward mode, so sweep forward mode first | five defects in the duals, incl. `ln(−5)` returning a NaN value beside a **confident** −0.2 derivative |

Three corollaries, each learned by being caught out:

* **A test written by the author of the code inherits the author's list of cases.** An independent
  derivation commissioned against 2.10.1's finished code confirmed all 37 partials and 0 of 24,237
  FD comparisons failed — its entire value was in the **enumeration**, naming two degeneracies the
  design had not.
* **A guard cannot be reached by differencing the function it guards.** At every input where one
  fires, the perturbed scalar is NaN too, so the sample is skipped. 2.11.0's FD sweep reported clean
  while skipping exactly the five rows that mattered.
* **A threshold that needs a scale chosen for it is the defect.** One rule — *guard exactly what
  makes the division fail and nothing more* — settled six thresholds across 2.10.2 and 2.11.0.

⚠ **Four separate measurements stated in committed text did not reproduce** during this arc, each
caught by an adversarial reader rather than a gate, and two had already propagated into three or four
files. `scripts/check-measurements.sh` closes that class and is enabled, PR-only and scoped to the
claims a branch *adds*. Whether to keep it is still open — see **Open items** below.

## Release train

What each release **delivers to a consumer**. Released versions are not listed here — their record
is the CHANGELOG and the Release History table at the foot of this file. Defects are not roadmap
items: they are tracked in `issues/` and discharged as a **precondition** of the release they gate.

| Release | Deliverable | Gated on |
|---|---|---|
| **3.0.0** | `Result<T,E>` API — breaking | 2.11.0 feature-complete |

---

## Open items

Everything still owed, in one place. Each carries why it has not been done, because "deferred with a
reason" and "forgotten" are indistinguishable once the reason is lost.

### EPA — one routine, three entangled questions

All three touch `gjk_epa_*` and none should be done alone: they share a benchmark and a live filing.

- [ ] **The three squared-epsilon sites.** `geo_advanced.cyr:1056/1060/1067` compare
      `hvec3_length_sq` against `EPSILON_F64` — the same squared-vs-unsquared mistake 2.10.1 and
      2.10.2 repaired at six sites in `geo.cyr`. **Deferred, not dismissed**: tightening them is a
      narrowphase behaviour change on a routine with a measured cost, so it needs its own
      before/after rather than riding along with a geometry release. The disposition table for all
      nine sites is in
      [`issues/archived/2026-08-10-squared-epsilon-guards-in-geo-ray.md`](issues/archived/2026-08-10-squared-epsilon-guards-in-geo-ray.md).
- [ ] **The seed-upgrade trade.** Giving `_epa_seed_gjk` the strictness upgrade `_epa_seed_portal`
      already performs closes the certification asymmetry between two public entry points and makes
      six previously-unmutatable EPA repairs observable — at a measured **+19.4% on
      `gjk_epa_sphere_box`**, for a change that alters no returned answer. 2.9.3 measured it and
      declined. A judgement call, not a defect; the change is one contained edit.
- [ ] **Sphere-family non-convergence** — the genuinely open half of that filing and independent of
      the certificate wording: spheres do not certify *even with a strict seed*, and they pay the
      upgrade's cost without gaining anything. **Root cause not established.** This is what the
      filing is actually about now.
      → [`issues/2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md`](issues/2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md)

### Decisions owed

- [ ] **`scripts/check-measurements.sh`: keep it or delete it.** The gate runs PR-only on the claims
      a branch *adds*, detector recall 21/22 with 0/15 decoys, but paragraph-level false positives
      sit at ~21% and **522** unmarked pre-existing claims mean it cannot be widened past the diff
      without a marking campaign first. It has now earned its keep several times over — it caught
      the arc's fourth non-reproducing measurement, and it has forced provenance markers onto every
      claim added since 2.9.3. The open question is whether ~21% FP is tolerable on a gate people
      must not learn to ignore.
- [ ] **Whether `dual_*` should gain a vector layer.** 2.10.0 answered "no dual vectors" for the
      *geometry*, on measured grounds (allocation per op, one seed per pass), and 2.11.0's
      reverse-mode tape removed the pressure entirely for the many-input case — one sweep instead of
      n passes. What remains is whether a *small fixed-size* vector dual is worth it for callers who
      want a Jacobian rather than a gradient. **No consumer has asked.**

### Documentary

- [ ] **Two items inside the archived `triangulate_polygon` filing** — the 2.7.x CHANGELOG entries
      state the prune's divergence direction backwards on at least one reproducer, and the
      neighbour-refresh locals at `src/collision_core.cyr:740,742` shadow the winding loop's
      `pn`/`nn`. Neither changes behaviour; both were recorded rather than buried when the file was
      archived.
      → [`issues/archived/2026-08-04-perf-triangulate_polygon.md`](issues/archived/2026-08-04-perf-triangulate_polygon.md)

### Toolchain, tracked upstream

**Three filings are open** (`docs/development/issues/`, 20 archived beside them):

| filing | state |
|---|---|
| `2026-04-26-cyrius-cli-arg-clobbers-source.md` | cyrius. **Deliberately never re-tested** — the reproducer destroys a source file. Presumed live. |
| `2026-08-09-cyrius-dead-fn-bodies-are-never-syntax-checked.md` | cyrius. **Partially fixed in 6.5.17**, re-run on 6.5.18 and unchanged: `build` and `check --with-deps` reject a file that does not parse, but **`lint` and `vet` still exit 0**. |
| `2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md` | ⚠ **hisab's own, not a toolchain item** — the EPA tier above. |

---
## Optional, demand-gated

- **GPU compute via soorat** (feature-gated) — no consumer has asked; parked until one does.
- **Adopt `vec_sort_by` / `vec_select_nth`** (cyrius 6.5.4) — consolidation onto stdlib, not a fix.
  hisab has exactly **one** hand-rolled sort, `_col_sort_indices_by_xy` (`src/collision_core.cyr`),
  with a single caller.
  ⚠ **This entry has now been wrong twice, in opposite directions, and both times by not running
  anything.** It first read "and Cyrius has no closures" — false since v6.3.8, propagated to four
  files. It was then corrected to a *measured* block: on 6.5.16 a capturing closure SIGSEGVed when
  passed through a function and called there, which is exactly this shape. **That was fixed in
  6.5.17 and re-verified on 6.5.18** (42 both directly and across the boundary), so the closure
  block is gone too.
  What actually remains is the plain API mismatch: `vec_sort_by` invokes its comparator as
  `fncall2(cmp, elem_a, elem_b)` — element *values* — whereas hisab sorts *indices* by dereferencing
  each into a separate `points` vector. A capturing comparator can now close over `points`, so this
  is doable; it is deferred on CLAUDE.md's wait-for-the-third-instance rule (this is the first) and
  because 2.6.15 already fixed the *complexity* of both hot sorts. Optional, not load-bearing.

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
| 2.11.0 | 2026-08-11 | 23,104 | 36 | **Reverse-mode autodiff, and the five forward-mode defects it found first.** Tape-based: one node per op recording both local partials and both input indices, and because inputs always have strictly smaller indices a single descending loop is a valid reverse order — no sort, no visited set. **grad_fwd_16 63.7 us -> grad_rev_16 5.70 us, 11.2x** for a 16-input gradient (not the theoretical 16x: reverse pays to RECORD the tape, and part of the rest is dual_* heap-allocating under an allocator that never frees). The pairing with optimize.cyr needed NO API change — a capturing closure holding the tape matches fncall2(grad, x, out), re-verified on 6.5.18 because that shape SIGSEGVed on 6.5.16. **Reverse mode is validated AGAINST forward mode, so autodiff.cyr was swept before a line of tape code existed**, and five defects came out: 1/1e-13 returned (0,0) where the truth is 1e13; ln(-5) returned a NaN value beside a CONFIDENT -0.2 derivative; sqrt(-4) was (NaN,NaN) because the guard ran after the sqrt; d/dx x^3 at -2 was NaN because f64_pow is exp(n*ln(base)) and rejects negative bases — the stdlib's STATED implementation, so the defect was hisab inheriting it undocumented. ⚠ **The finite-difference sweep alone found NONE of them**: at every input where a guard fires the perturbed scalar is NaN too, so the sample is skipped and the guard never asked. A guard has to be interrogated DIRECTLY. 14 mutants, 14 kills — two survived the first pass (the fixture's variables happened to BE nodes 0 and 1, so the ids indirection was untested) and one failure was the ASSERTION's fault, demanding 1e-8 where gradient descent only promises ||g|| < 1e-6. The solvers' own contract check came back CLEAN, the first time in five releases. 3507 |
| 2.10.2 | 2026-08-11 | 22,707 | 36 | **The primal defects the jets were sitting on.** All three 2.10.1 filed, plus the OBB rotation partial it deferred. **One rule settled four thresholds** — guard exactly what makes the DIVISION fail and nothing more (`_GEO_F64_TINY` = DBL_MIN); a threshold that needs a scale chosen for it IS the defect. The slab parallel test was scale-free: a box spanning [0,1] with a **unit** direction (9e-13, 0, 1) returned a hit at **x = 1.4**, and the same ray from inside a tall box gave an exit **18x too large**; `geo_ray_new` normalizes and did not protect. `geo_ray_capsule` returned a point a **full radius inside the solid** (t = 4 where the exit is 6, hit point ON THE AXIS) because `geo_ray_sphere` hands back only the FIRST root, so a cap root rejected by the half-space test meant the other was never considered — 32 of 600 interior origins also lost their exit entirely. Fixed by splitting the quadratic onto `_geo_sphere_roots` (both roots, one copy) and deleting the `cyl_missed` fallback outright. `geo_triangle_unit_normal` returned a **fabricated** (0,1,0) for a well-formed right triangle with 1e-4 legs. `dt/d(rotation) = [a_k x (c - p)]/f_k`, derived by perturbing WITHIN the rotation group rather than freely, verified over 36 FD comparisons at worst 1.9e-10 — a world-frame ANGULAR gradient, so a step along it is a valid rotation by construction. ⚠ Two repairs cost far more before being measured (an is_inf HELPER +5.6% on a 90 ns routine; materialising a hit point per cap root +92%), two fixtures were caught by COUNTERS rather than by failing, and a toolchain defect was nearly filed that does not exist — `var buf[N]` needs `&buf`. 13 mutants, 13 killed. 3469 |
| 2.10.1 | 2026-08-10 | 22,529 | 36 | **The branchy primitives, and two more defects the design check found first.** Jets for aabb, obb and capsule — all six `geo_ray_*` are now differentiable. Each primal split onto a face/branch-reporting variant with the plain entry point a one-line wrapper, so the value path is unchanged BY CONSTRUCTION: the `f64_max`/`f64_min` calls are byte-for-byte what they were and every added line sits behind `if (out != 0)`. aabb 3.1x its primal for 12 partials, obb 1.7x for 12, capsule 1.38x for 13 — the branchy three are CHEAPER relative to their primals than the smooth three, because the primal work they reuse is larger. **Before any feature code**, asking "what does the primal return when there is no face?" found `geo_ray_aabb` and `geo_ray_obb` returning **+Inf** for a degenerate direction (4 of 6 siblings honoured the contract; the same 4-vs-2 shape as 2.10.0) and `geo_ray_sphere`/`_capsule` comparing a **squared** length against the unsquared `EPSILON_F64` — the sphere reporting a MISS for any |d| < 1e-6 where t = 4e7 is exact, the capsule silently returning a CAP hit instead of the cylinder hit, wrong by 0.942%. ⚠ **The sphere's guard was introduced by 2.10.0's own repair**, and 2.10.0's homogeneity sweep could not have caught it: its scales are 2, 0.5 and 7, none within five orders of magnitude of the threshold the same commit added. A THIRD came from an independent derivation commissioned against the finished code — the tie flag saw edges and corners but not a zero-width slab or a tangential clip, so a flat box returned the whole gradient in the WRONG SLOT with tie = 0 on half of all configurations; the derivation confirmed all 37 partials (0 of 24,237 FD comparisons failing) and its value was entirely in the ENUMERATION. Three defects in the PRIMAL are filed OPEN for 2.10.2, including a scale-free slab test that returns a hit at x = 1.4 for a box spanning [0,1] with a UNIT direction. The capsule seam is proven C1 by a CONVERGENCE-RATE assertion after a single-offset one failed on a fixture fault. 25 mutants written, 24 killed — the survivor proved a branch update was dead code (cyl_t1 <= cyl_t2 always; 932 randomised quadratics, 0 violations). jet_obb 1.083 us -> 872 ns (-19.5%). Toolchain 6.5.17 -> 6.5.18, zero stdlib delta. 3453 |
| 2.10.0 | 2026-08-10 | 21,855 | 36 | **Differentiable geometry — and the primal defect it found first.** `src/geo_diff.cyr`: jets for plane, sphere and triangle returning the full gradient from ONE evaluation, allocation-free, as a post-pass on the shipped primal so no intersection algorithm is duplicated. Sphere 7 partials at 335 ns vs its 93 ns primal (3.6x); triangle 15 partials at 934 ns vs 280 ns (3.3x) — against the ~4,200 ns forward-mode duals would need for the same 15. **Before any autodiff was written, the design's own homogeneity check was run against the EXISTING primitives and failed**: `geo_ray_sphere` was not degree -1 in the ray direction, returning a t whose hit point sat 3.74 from the centre of a unit sphere, with `geo_ray_capsule` inheriting it through its end caps. Silent wrong output in shipped geometry, found by a check for a feature that did not exist yet. The same pass found 17 of 60 benchmarks measuring `clock_gettime` rather than the operation (ray_sphere read 1,466 ns, is 79 ns) — the fourth non-gating gate of the arc, and the only reason the repair's +17.6% was visible. Nine mutants caught across the two jets. Toolchain 6.5.16 -> 6.5.17, which fixed all three defects hisab filed upstream. 3398 |
| 2.9.3 | 2026-08-10 | 21,498 | 35 | **The open filings, and the two whose own proposed fixes were wrong.** Five of six internal filings closed. Two real defects: `delaunay_2d` silently DROPPED input points on any set mixing scales (one vertex at ~6e5 with five inside ~5e-17 of the origin returned 4 triangles where the exact hull says 6, and used 5 of 6 points) — fixed with an adaptive exact `orient2d` on RAW coordinates, 300/300 against exact rationals where the float winding scores 0/300; and `_col_dl_incircle` answered differently depending on VERTEX ORDER, `g1` being the last helper reading the CCW storage convention instead of forming the winding. Three guards were policing nothing: the k-d balance guard had no regression assertion (deleting it broke nothing checked) and no benchmark on its own input class (now 564.8 us guarded vs 3.397 ms unguarded, 6.0x); the ear prune's only benchmark was its BEST case (reflex-heavy is 3.6x, and the small-n regression is +15% measured against the pre-prune body); and `_col_point_in_tri` documented itself as strict-interior while being boundary-inclusive. **TWO FILINGS ARGUED FOR REPAIRS THAT MEASUREMENT REFUTED** — an exact determinant of pre-differenced operands fixes nothing (the information is gone before it runs), and dropping EPA's seed test produces wrong depths at exact tangency (`_ag_baddepth` 0 -> 4). Both were implemented in full before being rejected. The EPA filing's central claim was also false: it measured one of two entry points. 3376 |
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
