# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.6.2**. Stdlib `ganita` (6.2.x math umbrella) provides dense decompositions + transcendentals.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current — v2.21.0

Suite **3991** across five harnesses (hisab 497, foundation 413, modules 2072, edge_cases 233,
abuse 776), constant gate **159/159**, **74** benchmarks, **35** `[lib]` modules, toolchain
**6.6.2**, sakshi **2.5.1**, ganita **1.2.4**, and **zero** deprecated-alias call sites. All gates green:
`lint` 0 warnings and `fmt <file> --check` 0 drift across all 44 sources, `vet` 2 deps / 0 untrusted
/ 0 missing, `deps --verify` 31/31, `fuzz` 1/0, `coverage` 640/644 (99%) functions over 36/36 files,
distlib in sync.

**Per-release detail lives in `CHANGELOG.md`**, and the one-line-per-version record is the Release
History table at the foot of this file. What belongs *here* is the part that generalises.

> ⭐ **Verified 2026-09-09 against the tree, and 18 of 39 items did not survive it.** Every open item
> in this file was handed to an independent verifier told to *try to prove it already done*: 21 came
> back genuinely open, **15 rested on a premise that is now false**, and 3 were finished. The stale
> ones are struck through below with the proof that killed them, rather than deleted — several were
> false in a way that would have sent someone to do the **wrong work**, and that is worth keeping
> visible. Evidence base: [`audit/2026-09-09-roadmap-verification.md`](../audit/2026-09-09-roadmap-verification.md).
>
> ⚠ **This file had drifted into the exact failure it narrates.** Its own header said toolchain
> 6.5.33 while line 18 said 6.6.2 — contradicting itself fourteen lines apart. Corrected here, and
> the remaining counter drift is listed in the verification report rather than trusted.

### The standing lesson of the 2.7.0–2.11.2 arc

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
| 2.11.1 | *(no feature — a full audit instead)* a mechanical grep for the class the last four releases kept repairing | the same guard defect at ~24 more sites, `cx_div` among them — the module whose own comment records the lesson |

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
* **A lesson written beside the code that taught it does not reach the other thirty-four modules.**
  2.11.1's audit found the 2.6.14 squared-epsilon class at ~24 further sites, including `cx_div` —
  which is *named in the comment recording the lesson* and still fabricates zero. Only a grep
  applies a rule tree-wide; a review applies it where someone happened to look.

⚠ **Four separate measurements stated in committed text did not reproduce** during this arc, each
caught by an adversarial reader rather than a gate, and two had already propagated into three or four
files. `scripts/check-measurements.sh` closes that class and is enabled, PR-only and scoped to the
claims a branch *adds*. **Kept** — the open question is now only whether it should run on push as well as on PRs; see **Open items** below.

## Release train

What each release **delivers to a consumer**. Released versions are not listed here — their record
is the CHANGELOG and the Release History table at the foot of this file. Defects are not roadmap
items: they are tracked in `issues/` and discharged as a **precondition** of the release they gate.

| Release | Deliverable | Gated on |
|---|---|---|
| ~~**2.12.0 — the safety release**~~ | ✅ **SHIPPED.** The abort tier (4 entry points) + the allocation tier — which was **4 sites, not the 2 this row estimated**. ⭐ Two were found by refusing to trust the list: one by a **mutant that did not die**, one by grepping for the shape after the first was repaired. | — |
| ~~**2.13.0 — the suite release**~~ | ✅ **SHIPPED.** 842 truncating assertion sites -> **16**; `foundation.tcyr` 89% -> **0%**. ⭐ The migration was self-verifying, and the four sites that failed on conversion were four different real defects. Named tolerance helpers added to all five suites. | 2.12.0 |
| ~~**2.14.0 — the epsilon release**~~ | ✅ **SHIPPED, PARTIALLY.** ⛔ **This row said "~20 sites"; the census found 136 guards and 97 confirmed defects in 23 modules** — the estimate came from the 2026-08-11 audit's *confirmed* table, which is a list of instances someone reproduced, not a census of the class. **23 confirmed sites repaired, 58 mutants killed, +83 assertions.** ⭐ Three repairs were NOT on the census list and came from grepping for the shape. **74 remain, enumerated with evidence.** | 2.13.0 |
| ~~**2.15.0 — the epsilon tier, remainder**~~ | ✅ **SHIPPED.** **73 of the 74 repaired**, 81 mutants installed and **73 killed**; 8 survivors documented with their reasons rather than tidied away. ⛔ **1 DEFERRED — `su2_log`, and it is a formula change, not a threshold**: it divides by θ² and θ³, and at θ ≤ 2.2e-162 both `θ*θ` and `1−cos θ` are exactly 0, so lowering the guard would make the coefficient `0/0 = NaN`. ⭐ **Three siblings of it were never on the census list** (`so3_log`, `se3_exp`, `se3_log`) and were found by grepping for the shape. | 2.14.0 |
| ~~**2.16.0 — the small-angle series**~~ | ✅ **SHIPPED.** All four maps repaired; 7 mutants, 5 killed, 2 documented equivalences that exist BECAUSE the repairs made each other redundant. ⛔ **The log maps had a larger defect the guard was hiding**: `acos` of a value that rounds to exactly 1.0 below θ ≈ 1.5e-8, so the whole rotation was lost — measured, exactly 0 from 2^-28 down. `atan2` recovers it bit-exactly. ⭐ **Round-trip floor 2^-26 → 2^-537, 511 decades.** | 2.15.0 |
| ~~**2.17.0 — the norm tier**~~ | ✅ **SHIPPED. 39 of 39 sites repaired, 73 mutants: 67 killed, 6 recorded with reasons.** ⛔ **This row said the tier was `su2_exp` and `so3_from_axis_angle`; the tree-wide grep found 51 `f64_sqrt` sites in 16 files and a 70-agent census confirmed 19 more defects across 9 modules** — the fourth release running where the class was wider than its list, and the row's own warning to "grep for the shape before sizing this" was the thing that caught it. ⛔ **The regression sweep found what the release itself had missed**: `hquat_inverse` reads `hquat_length_sq` directly, so repairing `hquat_length` did nothing for it — **1018 of 2041 binades wrong, 507 the fabricated identity** — and a 2.17.0 comment asserting it "routes through" the repaired norm was simply false. ⛔ **`cga_rotor`'s 2.14.0 repair note describes a defect that was still live**, 500 binades lower: 991 of 2041 axis scales still returned the identity rotor. ⛔ **In the solvers the class does not return a wrong number — it returns a CONFIDENT SUCCESS AT THE STARTING POINT**: 446 of 1001 objective scales for conjugate gradient and 461 of 1001 for Levenberg-Marquardt returned `HSB_ERR_NONE` with `x` untouched, and `cmat_inverse` called **498 of 1010** perfectly invertible matrices singular. ⭐ **Every confirmed site is closed**, and four items it MEASURED but did not fix are filed on 2.18.0 rather than folded in. | 2.16.0 |
| ~~**2.18.0 — the SVD factors**~~ | ✅ **SHIPPED. The "no known fix" item is CLOSED, and the fix was one line.** `‖A − U S Vt‖_F` degraded because `_lp_bidiagonalize` **applies** a left Householder reflector when `vtv >= F64_TINY` and **replayed it into U** only when `vtv > EPSILON_F64`: every reflector in between went into `B` and not into `U`, so `S` was exact, `U` and `Vt` were each perfectly orthogonal, and `U*S*Vt` simply was not `A`. ⛔ **2.15.0 CREATED that band** by repairing one guard of the pair and leaving its twin forty lines away — a guard repaired on one side of a pair is a new defect, not half a repair. Measured: the small block came back with the WRONG SIGN (residual exactly 2x the block) for **19 ratios, 2^-21..2^-39**, and **288 of 999** on the 2.16.0 tree; now **0 of 999**. ⭐ The SVD's Wilkinson shift also formed degree-FOUR quantities (`tr*tr`, `a11*a22`, `a12*a12`), setting the routine's floor at the FOURTH root of the subnormal floor, 2^-268.5 — measured to the binade. Rewritten in the degree-two closed form and computed on power-of-two-scaled operands: **256 correct ratios (2.16.0) → 268 (2.17.0) → 999 (2.18.0), with 474 silent wrong answers and 731 loud failures both going to 0.** ⭐ `_lp_pow2_floor` now balances subnormal matrices, so `eigen_qr` is exact to **2^-1070**. ⚠ **Original row follows.** ⛔ Found in 2.15.0 and NOT closed by it: `‖A − U S Vt‖_F` is exactly 0 for block ratios 1e-2…1e-5, then **4.17e-7 at 1e-6**, decaying proportionally to c. 4.17e-7 reproduces the census's own figure for a half-repaired bidiagonalisation. ⛔ **AND THIS ROW'S CLAIM THAT "the singular VALUES stay correct" IS REFUTED, measured 2026-09-10 during the 2.17.0 census.** `svd_golub_kahan([[1,0],[0,c],[0,c]])` has singular values {1, c·√2} and returns **{1, c} — 29.3% low — for every block ratio at or below 2^-41**, with `rc = HSB_ERR_NONE`. 2^-41 is ~1e-12, so this is an EPSILON-class deflation threshold on a sub-block, not the norm class: balancing makes the thresholds relative to the LARGEST entry, which says nothing about a small block. ⚠ **Open-ended: a known defect with no known fix**, unlike 2.17.0. Also carries the EPA seed-upgrade trade, which is blocked on re-measurement. | 2.17.0 |
| ~~**2.19.0 — the 3.0.0 prep**~~ | ✅ **SHIPPED. All three parts delivered — and every one of them needed a gate that could not have failed.** 44 `#must_use` annotations (**not the 167 this row said**: that is the count of `return HSB_ERR` STATEMENTS, and 4 of the 48 candidate functions can only ever return `HSB_ERR_NONE`), the 12 codes as `enum HsbError`, and the decisions owed measured by an 8-way fan-out with adversarial verification. ⛔ **`#must_use` is a compiler diagnostic, not a lint one** — `^  warn ` matched it zero times, so the annotations were decorative until a step was wired; it then found `examples/basic_math.cyr` discarding two fallible returns. ⛔ **Two error-code mutants survived all 3940 assertions**, one making `HSB_ERR_ALLOC` collide with `HSB_ERR_INVALID_TRANSFORM` — every assertion compares by NAME, so the value cancels out. ⛔ **The release shipped its own broken provenance marker**, caught by a gate wired PR-only in a repository with **0 pull requests**. ⛔ **The version gate never ran the binary.** ⚠ Four items proved to be REPAIRS rather than decisions and are scheduled on 2.20.0 rather than folded in; a per-benchmark noise band was measured **strictly worse** than the shipped global ±10% and is declined. Suites **3937 → 3955**. | 2.18.0 |
| ~~**2.20.0 — the four repairs 2.19.0 measured**~~ | ✅ **SHIPPED. All five items closed — two repaired, three characterised, and every one of them had a FILING that was wrong about the size, the direction, or the safety of the obvious fix.** Exact integer rendering above 2^63 (both renderers, not the one the row named); `cga_point` nullity proved a wrong answer in BOTH tails with no arithmetic fix possible; the subnormal-SVD claim retracted (270 of 416 report success, 57 right) with the fixture family widened as the repair's acceptance test; the `triangulate_polygon` direction corrected after thirteen releases; and `ad_grad`'s O(m^2) trap documented with its ratio shown to be m-dependent rather than the constant it was filed as. ⛔ **Adding a benchmark changed another benchmark by 31%** and would have produced a phantom 44% speedup. Suites **3955 -> 3983**. Two follow-on releases filed. | 2.19.0 |
| ~~**2.21.0 — the CGA null basis**~~ | ✅ **SHIPPED. A conformal point's nullity error is now ULP-LEVEL AND SCALE-FREE.** Median `|P.P| / q` over 2000 full-mantissa points per binade, against the tree that actually shipped as 2.20.0: **1.0 -> 4.48e-17 at 2^-30**, 1.05e-16 -> 4.70e-17 at 2^0, **1.0 -> 4.33e-17 at 2^+30** — a relative error of 1.0 is TOTAL LOSS, so both tails go from losing the value outright to sub-ulp, and the floor is FLAT rather than a U. Distance recovery tracks it: median **46.4x wrong at 2^-30 and 48.7x at 2^+30 -> 1.76e-15 and 1.93e-15**, centre 3.8e-15 -> 1.87e-15. ⭐ The repair was the BASIS: 2.20.0 proved no arithmetic fix existed, because at x = 2^-30 the correctly-rounded `ep` IS -1/2. ⛔ **"EXACTLY NULL" WAS THE HEADLINE UNTIL A PRE-TAG AUDIT PRICED IT.** Bit-exact nullity is reachable only by dropping 2.18.0's Neumaier compensation, which makes the norm repeat `cga_point`'s own rounding so the two cancel — and then `1 + a*e1 + a*n0 + (a/2)*ninf` returns a scalar part of **0 where the answer is 1, for 6 of 11 magnitudes, all at a >= 2^30**. **An exact zero obtained by making the same rounding error twice is an artifact, not accuracy.** Reverted; bit-exact nullity reads **6761 of 6800 non-null -> 5194**, and the claim is the residual, not a hard zero. ⛔ **NO ASSERTION IN 3989 COULD SEE THAT CHANGE** — green with the compensation present AND absent — so a witness was written (**3989 -> 3991**, verified to fail without it). ⛔ `_cga_scalar_of_geo` also had to sum ALL pairs (diagonal-only left points non-null 6800 of 6800 — WORSE): of the 32 pairs producing blade 0, only **16 are i == j**. ⛔ **THE FIRST CGA CALL COST 2.7 ms AND EVERY BENCHMARK WAS BLIND TO IT** because they all warm up — surfaced because two probes of the same operation disagreed by 2.1x and the difference was a warm-up loop; repaired to **0.72 ms** (2.20.0: 11 us), break-even ~430 products, table byte-identical because the FNV contract already pinned it. ⛔ Building it found a **cycc 6.6.2 wrong-code bug** — `continue` at two nesting levels binds to the wrong loop — filed upstream, and **the first version of that filing described its own symptom wrongly** (it said the outer loop is terminated; instrumented, it runs its full trip count). ⭐ The gate came BEFORE the code: 1024 entries derived in CI, contract FNV-1a `0xF4A98C5706D5CF5B`. ⭐ The 2.20.0 acceptance pins INVERTED as designed. ⛔ **THE AUDIT ALSO CAUGHT FOUR WRONG FIGURES AND TWO GATES THAT DID NOT GATE**, most from measuring "before" against a MODEL rather than the shipped tree: the nullity baseline (24.8% -> really 99.4%), a translator claim that was simply false (both trees fail 485 of 2046 binades, first at 2^539 — identical), "exact at EVERY magnitude" (57 binades fail), and a `point*point` figure that was right by accident. ⚠ **Speed is SCOPED**: **2.22x on point*point** and up to **10.93x SLOWER at k = 32** occupied blades, because `_cga_scalar_of_geo` is now quadratic in occupancy; crossover at k ~ 8. | 2.20.0 |
| **2.22.0 — subnormal SVD: scale the active block** | Scale the active bidiagonal block into the normal range before each QR step and unscale after; singular values scale linearly and a power-of-two factor makes it exact. ⛔ **Do not ship the 2.19.0 candidate as written** — it was validated only on upper-triangular blocks and on a general 2x2 converts LOUD failures into gross SILENT wrong answers. Acceptance test exists: 270 of 416 rows currently report success with only 57 right. | 2.20.0 |
| **3.0.0** | `Result<T,E>` API — breaking. ⚠ Re-scope before planning: it is the **v6.6.0 value form**, not the boxed form this file was written against. Carries the public/private function surface, which is breaking for the same reason. | 2.19.0 |

### ⛔ Why the epsilon tier is no longer first, and why its old gate is gone

**The old gate was unexecutable.** 2.12.0 used to be gated on "the 28 unverified audit findings re-run
first — this gates every item below it". **The 28 were never enumerated.** `audit/2026-08-11-v2.11.0-full.md`
records them only as a count (`7 critical, 16 high, 22 medium, 7 low` across all 52); its only
per-finding tables are explicitly the *confirmed* and *fixed* sets. There is no ledger in `docs/`,
none in git history (the report was committed whole and never revised), none in scratch. The 2
REFUTED are also unidentified, so a re-run cannot even subtract the known-bad ones.

⭐ **And it would not have bound anything even if it could be run.** The epsilon-tier sites come from
the table headed "**Confirmed** instances" — the 21 skeptic-confirmed set, which is *disjoint* from
the 28 unverified. The alloc and abort sites were reproduced from a cold start on HEAD **with
discriminating controls**, which is strictly stronger evidence than the skeptic pass the gate
demanded. The suite tier is not a `src/` change at all. **A gate that blocks four tiers it has no
evidential relationship to is not caution, it is a deadlock.** Retired.

**The new ordering is forced by a measurement, not a preference.** Every epsilon repair is a change
to a threshold below 1.0. **843 of 3247 assertion sites compare through `f64_to`, which truncates** —
`assert_eq(f64_to(1.9999), f64_to(1.0))` **passes**, so an absolute error of 0.9999 is invisible to
them. Repairing thresholds first would mean landing sub-1.0 behaviour changes into a suite that
structurally cannot see them. The abort and alloc tiers do not have this problem: their failure mode
is a signal (a SIGSEGV, an abort, a clobbered canary), not a digit, so they are provable *through*
the truncation and can go first.

⚠ **What replaces the retired gate**: either commission a **fresh sweep of the 2.11.5+ tree** — the
better buy, since the findings are five releases stale and the 2026-08-11 audit never reached
`symbolic*`, `lie*`, `spatial`, `color`, `noise_simplex`, `einsum`, `tensor`, the SIMD paths or any
cross-module interaction — or verify **per-site inside the epsilon tier**, where the prescribed
scale-covariance assertion *is* the verification and the sites sit at known `file:line`.

⚠ **One reconciliation is owed before 2.14.0**: 21 CONFIRMED − 4 repaired in 2.11.1 = **17 confirmed
findings** that the tiers below never account for *by count* (the epsilon tier is sized "~20 sites",
mixing confirmed with unverified). A confirmed finding can fall between buckets and nothing would
notice. Reconcile the tier lists against the audit's confirmed table, by row, once.

---

## Open items

⭐ **HOW TO READ THIS FILE.** The **Release train** table above is the authority on ORDER and on what
ships when. This section is the EVIDENCE behind those rows, and **every open item carries its version
in bold brackets** — `**[2.17.0]**` — so the two views cannot drift apart. If an item here has no
version tag, that is a bug in this file, not an item without a home.

⚠ **A struck item under a `> Original text` quote is HISTORY, not work.** Four such items read as open
checkboxes until 2026-09-10 and made the backlog look larger than it was.

⚠ **Two sections are deliberately unversioned and that is not a gap**: *Optional, demand-gated* and
*Parked / deferred* hold work with **no driver yet**. Nothing moves out of them without a consumer
asking; when one does, it gets a version here first.

⛔ **AND A CHECKED ITEM IS NOT A VERIFIED ONE.** The 2026-09-09 sweep found **18 of 39** items stale,
and a re-read on 2026-09-10 found three more that were done but still listed open — including one
discharged by RE-READING rather than repairing, because it had asked for a null check on an allocation
RESULT where a cap on the INPUT was already the stronger guarantee. Check the tree before believing a
row in either direction.


Everything still owed, in one place. Each carries why it has not been done, because "deferred with a
reason" and "forgotten" are indistinguishable once the reason is lost.

### The 2026-08-11 audit backlog

The full P(-1) sweep of the 2.11.0 tree is
[`audit/2026-08-11-v2.11.0-full.md`](../audit/2026-08-11-v2.11.0-full.md): 6 dimensions, 115 checks,
**52 findings reproduced**, 21 CONFIRMED by an independent skeptic, 2 REFUTED, 1 already known. 2.11.1
executed four repairs; the rest is here, **reordered 2026-09-09** — see the release train for why the
epsilon tier is no longer first.

  > **Original text, kept for the record:** ~~**Re-verify the 28 findings that were never sent to a skeptic.** … **This gates every item
      below it.**~~ ⛔ **GATE RETIRED 2026-09-09 — it could not be executed and it bound nothing.**
      The 28 are **not enumerated anywhere**: the audit report records them only as a count, and its
      only per-finding tables are explicitly the *confirmed* and *fixed* sets. No ledger in `docs/`,
      none in git history, none in scratch; the 2 REFUTED are unidentified too, so a re-run cannot
      even subtract the known-bad. ⭐ And the tiers it claimed to gate draw from the *confirmed* set,
      which is **disjoint** from the 28. Replaced by a choice, recorded in the release train: a fresh
      sweep of the current tree, or per-site verification inside the epsilon tier where the
      scale-covariance assertion is itself the proof. **Keeping it would have blocked the audit's own
      highest-value item behind a task with no input.**
- [x] ✅ **The allocation tier — SHIPPED IN 2.12.0, and it was FOUR sites, not the 2 this item
      predicted.** `detect_islands`, `solve_gmres`'s Hessenberg, `solve_gmres`'s `_lext_copy` and
      `solve_bicgstab`'s `_lext_copy`. ⭐ The last two were not on any list: one surfaced because a
      **mutation test failed to kill** (the guard's stated overflow justification was unreachable —
      because an EARLIER unchecked alloc failed first), the other by grepping for the shape rather
      than assuming the tier was complete.

  ⚠ **A mechanical sweep for the whole class now exists and the number is bigger than anyone
  wrote down: 90 unchecked parameter-sized allocations tree-wide, of which 9 are PUBLIC entry
  points.** Two are repaired here; **seven remain** and are listed below rather than folded in
  silently. Original text: ⚠ The third was `halfedge_from_triangles`,
  which is **already owned by the abort tier below** and needs a *different* repair (its
  `vec_push` aborts rather than returning 0). Both real sites reproduced as SIGSEGV on HEAD with
  a discriminating control: `solve_gmres`'s Hessenberg (`linalg_ext.cyr:390` — quadratic in
  `restart`; m=16385 asks for 2,147,762,880 B, just over `ALLOC_MAX`, and the same n with
  `restart = 2` exits 0) and `detect_islands` (`collision_mesh.cyr:1263/1264/1338`). Mechanical,
  low risk, identical repair to the three `num_ext` sites closed in 2.11.1. **One API decision:**
  `detect_islands` returns a vec, not an rc, so it has no channel to report a failed allocation
  through — decide before repairing. **Scheduled: 2.12.0.**
- [x] ✅ **The abort tier — SHIPPED IN 2.12.0.** All four repaired, each reproduced with a
      discriminating control and mutation-proven. ⛔ One of the four was WORSE than an abort: The roadmap never named the three `collision_core` sites; they are named now, all
      reproduced live on HEAD. `convex_hull_2d` (`collision_core.cyr:488`), `triangulate_polygon`
      (`:653`) and `sequential_impulse` (`:262`) each index a vec with the **caller's `n`** and never
      compare it against `vec_len`; `halfedge_from_triangles` (`collision_mesh.cyr:1029`) hits
      `VEC_CAP_MAX` and calls `_vec_die`. A library must not end its caller's process.

  ⛔ **`sequential_impulse` does not even abort — it returns success after writing out of
  bounds.** Its zeroing loop runs `for zi < n` writing 16 bytes per contact **before the first
  `vec_get`**, so with `iterations = 0` it never touches the vec at all. Reproduced: an **empty**
  contacts vec, `n = 1000`, a 64-byte `out_impulses` and a 64-byte canary → **all 8 canary words
  clobbered, rc = 0, no diagnostic**. That is ~16 KB written past a 64-byte buffer with a clean
  exit — CWE-787 with no crash signal.
  ⚠ **The abuse suite has a canary block for this exact function** (`abuse.tcyr:1139-1146`) and it
  misses this: it tries `n` = 4, 0 and −2, every one of which is **in bounds** for the buffer it
  allocates. A canary only proves what its `n` reaches.
  **Repairs are O(1) `vec_len` comparisons — but the zeroing loop must be bounded too, not just
  the `vec_get` loop.** **Scheduled: 2.12.0.**
- [x] ✅ **The remaining 7 public entry points — DISCHARGED, and by RE-READING rather than by
      repairing.** Verified 2026-09-10: every one of the seven is bounded by a DIMENSION CAP
      small enough that `alloc` cannot reach `ALLOC_MAX` — `tensor_new` rank <= 8 (64 B),
      `geodesic_state_new` / `geodesic_rk4` / `parallel_transport` dim <= `_DG_MAX_DIM` = 16
      (<= 256 B), `opt_bfgs` / `opt_lbfgs` n <= `_OPT_MAX_DIM` = 4096, and
      `opt_conjugate_gradient` carries the 2.12.0 round-trip check AND null-checks all five
      allocations. ⚠ **The row asked the wrong question**: it looked for a null check on the
      RESULT, and a cap on the INPUT is the stronger form of the same guarantee. The class was
      real when filed; the sweep that produced this list did not re-read the caps already
      present. Original text follows.
  > **Original text, kept for the record:** ~~**The remaining 7 public entry points with an unchecked parameter-sized allocation.**~~
      ⭐ **This item exists because 2.12.0 stopped trusting the tier's list and ran the grep.** The
      class is: `alloc(<caller parameter> * 8)` with no check that the result is non-zero. Sweep
      result — **90 sites tree-wide, 9 of them PUBLIC entry points**; 2 repaired in 2.12.0, these 7
      remain:

  | entry point | allocation |
  |---|---|
  | `tensor_new` (`tensor.cyr`) | `alloc(rank * 8)` |
  | `geodesic_state_new` (`diffgeo.cyr`) | `alloc(dim * 2 * 8)` |
  | `geodesic_rk4` (`diffgeo.cyr`) | `alloc(dim * 8)` |
  | `parallel_transport` (`diffgeo.cyr`) | `alloc(dim * 8)` |
  | `opt_conjugate_gradient` (`optimize.cyr`) | `alloc(n * 8)` |
  | `opt_bfgs` (`optimize.cyr`) | `alloc(n * 8)` |
  | `opt_lbfgs` (`optimize.cyr`) | `alloc(n * 8)` |

  ⭐ **THE TRIAGE WARNING BELOW WAS RIGHT, AND FOLLOWING IT SAVED SIX UNNECESSARY GUARDS.**
  `tensor_new` (`rank > 8`), `geodesic_state_new` / `geodesic_rk4` / `parallel_transport`
  (`_DG_MAX_DIM = 16`), `opt_bfgs` (three layers) and `opt_lbfgs` (both `n` and `m`) are each already
  capped **ahead of the allocation**, by bounds up to 8.4 million times tighter than an
  `ALLOC_MAX`-derived one would be. A guard sitting behind an existing stricter guard is dead code,
  not safety.
  ⛔ **One real defect, and it was not the class the sweep was looking for**: `opt_conjugate_gradient`
  and its neighbour `opt_gradient_descent` let `n * 8` wrap to a small **positive** value, so `alloc`
  SUCCEEDS and the existing null check is defeated. Repaired in 2.12.0 with a round-trip test rather
  than a cap — a cap was measured and rejected because it refuses a conforming caller at n = 1e7.

  Original text:
  ⚠ **Triage before repairing — several are probably FALSE POSITIVES.** Some of these functions
  already cap their dimension with a `_MAX` enum, which would make the allocation unreachable;
  the sweep's heuristic cannot see that. **Do not add a guard to a site that cannot fail** — this
  repo's roadmap has been damaged more by claims that did not survive the tree than by missing
  work. Each site needs: reproduce with a small buffer and a large claimed dimension (⚠ not a
  probe that allocates the dimension itself and crashes in the PROBE — that error was made twice
  during 2.12.0), a discriminating control, then a bound **derived** from `ALLOC_MAX` or the
  function's own existing cap.
  **Scheduled: 2.12.1**, since it is the same mechanical repair as the tier just shipped.

- [x] ✅ **The suite tier — SHIPPED IN 2.13.0, and the justification was measured, not argued.**
      Re-counted at HEAD: **843 of 3247 assertion sites (26.0%)** compare through `f64_to`, which
      **truncates** — the audit's 836 has grown, not shrunk. ⚠ The often-quoted "23.8%" divided a
      *static* numerator by a *dynamic* denominator (3510 executed assertions); against static sites
      it is 26%.
      ⭐ **The load-bearing claim is provable by execution, not prose**: `assert_eq(f64_to(1.9999),
      f64_to(1.0))` **passes**, so an absolute error of 0.9999 is invisible.
      ⚠ It is not evenly spread, and that changes how to attack it — `foundation.tcyr` is **89%**,
      `edge_cases` 49%, `hisab` 33%, `modules` 19%, `abuse` 12%. This is far more a single-file
      problem than a tree-wide one. Only **21 of 843** scale their comparison first, and **three of
      five suites define no tolerance helper at all** (`hisab`, `modules`, `abuse`) — so the campaign
      is *add the helper, then convert*, not *convert*.
      Also in this tier: 38.4% of value-changing single-operator mutants survive all five suites, and
      29 public functions are covered only by `assert_neq(f(...), 0)`.
      ⚠ **Fresh evidence it is still biting**: 2.11.4 found **two assertions that had gone silently
      vacuous** — they compared through a tolerance and a round, so when upstream repaired the defect
      they pinned, they kept passing while the property they claimed to test evaporated.

- [x] ✅ **The epsilon tier — CLOSED across 2.14.0, 2.15.0 and 2.16.0.** The grep was run: a
      census of **136 guards in 25 modules** found **97 confirmed defects in 23**, against a row
      that said "~20 sites". **96 repaired** (23 + 73), **1 deferred and then closed** by
      2.16.0's small-angle series. Dispositions per site in
      [`docs/audit/2026-09-09-epsilon-open.json`](../audit/2026-09-09-epsilon-open.json).
      Original text follows.
  > **Original text, kept for the record:** ~~**The epsilon tier — the count is wrong, the grep it prescribes has never been run, and it
      is LAST rather than first. Scheduled: 2.14.0.**
      ⚠ "~20 sites" came from a *review*. The mechanical grep the item itself asks for returns
      **123 `f64_(lt|gt|le|ge)(…, EPSILON_F64)` comparison guards across 24 of 35 modules**
      (`geo_advanced` 22, `linalg_precision` 17, `lie` 12). Not all are defects — **nobody has
      triaged which guarded quantities actually scale**, and that triage is the first task, not the
      repair. ⚠ The DBL_MIN rule 2.10.2 settled reaches **three of thirty-five modules** (geo, autodiff, quat)
      under three different names — propagated once, by 2.11.1, and never given a shared constant. ⛔ `cx_div` is named in `complex.cyr:58`'s own lesson comment **and
      still fabricates zero**, which is the whole point of the standing lesson above. Named sites: `eigen_qr`, `cqr_decompose`,
      `solve_bicgstab`, `solve_gmres`, `m3_inverse`/`m4_inverse`, `cmat_inverse`, `svd_golub_kahan`,
      `cx_div`/`cx_inv`/`cx_powf`, `hvec3_angle`, `hvec2/3/4_normalize`, `cga_blade_inverse`,
      `m4_transform_point`, `calc_bspline`/`calc_nurbs`, `calc_monotone_cubic`, `num_newton`,
      `num_tridiag_solve`, `geo_barycentric_coords`, `geo_ray_triangle`, `sectional_curvature`,
      `hisab_inverse_lerp`/`hisab_remap`. Each gets a **scale-covariance** assertion bracketing every
      threshold a repair might plausibly have chosen. ⚠ **Not one commit** — each site is a behaviour
      change on a documented entry point, and this tree's history says these land in mutation-proven
      bites.

⚠ ~~The audit was correctness-shaped and did **not** reach `symbolic*`, `lie*`, `spatial`, `color`,
`transforms`, `noise_simplex`, `einsum`, `tensor` … or any cross-module interaction — and nothing in
it audited **performance regressions** or the **benchmark harness**.~~ **Mostly stale, and it
contradicts its own source.** `transforms` is listed as not-reached while **line 58 of the same
report files a finding against `hisab_inverse_lerp`** (`src/transforms.cyr:119`). Ten of the eleven
named modules were audited eight days earlier by the 2026-08-03/08-04 sweeps, whose §5 is titled
"Recorded so the next audit does not re-plough it". The benchmark-harness clause was discharged by
2.11.2, 2.11.4 and 2.11.5.
⭐ **TWO clauses survive, and an earlier draft of this correction silently dropped one of them.**
The ellipsis in the struck quote above elided `the SIMD f64v_* paths`, which is still a real gap —
the 2026-08-03 sweep reached only `hvec3_*`, never the 27 `f64v_*` call sites across mat4/vec4/quat/
mat3/vec3. ⚠ **Deleting a clause while striking a list is how a gap stops being tracked**, which is
the same failure the strike was correcting.
The other survivor is **"any cross-module interaction"**. ⚠ An earlier draft said no audit report
"has ever contained that string" — false: both full sweeps name it, in their own *did NOT reach*
sections. It has been declared out of scope twice, which is a stronger claim than never mentioned. Those two are where the next sweep starts.

### The norm tier [2.17.0]

- [x] ✅ **DONE IN 2.17.0 — and this checkbox was simply never ticked.** Re-verified 2026-09-10
      on the shipped tree: `su2_exp` and `so3_from_axis_angle` both route through `_lie_norm3`,
      which divides by the largest component before squaring. **The round-trip floor measured
      2^-537 -> 2^-1022 — the entire normal range.** ⚠ The probe was checked against the defect
      before being believed: built with 2.16.0's `lie.cyr` (which has no `_lie_norm3`) and
      everything else current, it reports exactly **2^-537**, the figure this row states — so it
      discriminates rather than always printing the same number.
      [measured: su2_log(su2_exp((0,0,2^-e))) bit-exact sweep, e = 1..1073, both trees, 2026-09-10]
      **[2.17.0]** **Scaled norms on the exp side.** ⛔ Found in 2.16.0 **by a mutant that would not
      die** — a fixture built to reach `se3_log`'s coefficient never did, and chasing why turned up
      the norm. `su2_exp` and `so3_from_axis_angle` compute `sqrt(x² + y² + z²)` directly, so a
      component below ~2^-511 makes the SQUARE underflow to zero and the whole rotation collapses.
      Measured: the log/exp round-trip is exact to **2^-537** and fails at **2^-538**, and that floor
      is now on the exp side — 2.16.0 moved the log side from 2^-26 to 2^-537, and this is what
      stops it going further.
      ⭐ **The fix is known and proven twice**: divide through by the largest component before
      squaring, exactly as `cx_div` (2.15.0) and the two log maps (2.16.0) now do.
      ⚠ **Grep for the shape before sizing this.** Every `sqrt(sum of squares)` in the tree is a
      candidate — `hvec3_length`'s own floor was measured at 2^-537 in 2.14.0 and is pinned by two
      assertions in `foundation.tcyr` — and the last three releases each found this class wider than
      its list said (2.12.0: 3 sites became 4; 2.14.0: "~20" became 97; 2.16.0: 4 became 4 plus two
      norms). Size it from the grep, not from this paragraph.

  ⭐ **THE GREP WAS RUN, 2026-09-10, AND THE PARAGRAPH ABOVE WAS WRONG BY 19 SITES.** 51 `f64_sqrt`
  sites in 16 files; a 10-agent census with 60 adversarial verifiers (three lenses each —
  reachability, dimension, arithmetic) confirmed **19 DEFECT_NORM** and refuted 1. **Fifth release
  running that this class was wider than its own list.** Progress and remainder below.

- [x] ✅ **DONE — vec2/3/4, quat length, complex, lie/lie_ext (12 sites, 15 mutants).**
- [x] ✅ **DONE — the foundation and geometry group (7 sites, 16 mutants, all killed).**
      `hquat_inverse` **1018/2041 → 0**, `hquat_normalize` 48/48 subnormal magnitudes → 0,
      `m3_frobenius` **992/2041 → 0**, `geo_segment_direction` **994/2041 → 0**,
      `geo_triangle_unit_normal` 877/1401 → 378 (residue is `hvec3_cross` itself, upstream of the
      norm), `cga_plane` **995/2041 → 0**, `cga_rotor` **995/2041 → 0** with 991 fabricated
      identities gone.
      ⛔ **Two of those were sites THIS RELEASE had already missed once**, found by a regression
      sweep asking "what did the repair miss" rather than by the census: `hquat_inverse` reads
      `hquat_length_sq` directly, and a 2.17.0 comment claiming it "routes through" the repaired
      norm was false. **A comment asserting a call graph is a claim like any other.**

- [x] ✅ **DONE IN 2.18.0 — the deferral's two conditions were both met, and this checkbox was
      never ticked.** This row required "a scale construction that works for subnormals and a
      `f64_div` rescale"; the shipped `cga_norm` has both — `_ga_pow2_floor` falls through to a
      mantissa-bit walk when the exponent field is 0, and the rescale divides rather than
      multiplying by a reciprocal, so the `0 * Inf` that produced the predicted NaN cannot arise.
      **Measured on 51 subnormal max coefficients (2^-1023 .. 2^-1073): 0 NaN, 0 fabricated zeros,
      51 finite positive results.** The row predicted NaN for exactly this input.
      ⚠ The line number in the heading below is stale — `cga_norm` is at `geo_advanced.cyr:2557`.
      [measured: subnormal-coefficient sweep through cga_norm, 2026-09-10]
      **[2.17.0]** **`cga_norm` (`geo_advanced.cyr:2438`) — confirmed defect, fix DEFERRED with
      reasons.** `sqrt(|cga_norm_sq(mv)|)` over a 32-slot multivector, so there is no pair of legs
      to scale. All three verifiers independently measured the obvious power-of-two rescale
      returning **NaN** for a subnormal max coefficient (`mx & 0x7FF0…` is 0 for a subnormal, the
      reciprocal is +Inf, and `cga_mv_scale` turns every structurally-zero slot into `0 * Inf`),
      where the shipped code returns a wrong `0`. **A fabricated NaN is not an improvement on a
      fabricated 0.** Needs a scale construction that works for subnormals and a `f64_div` rescale.

- [x] ✅ **DONE — the solver group (6 sites, 13 mutants, 12 killed + 1 proven equivalence).**
      `optimize:88/294/788` and `linalg_ext:324/514/1219`.
      ⛔ **In an optimiser this class does not produce a wrong number, it produces a CONFIDENT
      SUCCESS AT THE STARTING POINT.** Every convergence test in both files is `norm < tol`, so a
      flushed norm fires on the first iteration and the solver returns `HSB_ERR_NONE` with `out_x`
      still equal to `x0`. Measured: **446 of 1001** objective scales for conjugate gradient and
      **461 of 1001** residual scales for Levenberg-Marquardt did exactly that. **Both are 0 now.**
      `solve_gmres` on A = 2I: **461/1001 → 0**. `cmat_inverse`: **498 of 1010 exponents returned a
      FABRICATED "singular"** for perfectly invertible matrices → **0**.
      ⚠ **`cmat_inverse` needed three lines moved together**, not one: repairing only the
      open-coded `cx_abs` at :1219 takes 489 of 2001 to 488. `max_norm` is now a MODULUS, so both
      sides of both guards are degree one and the `f64_sqrt` is deleted rather than moved.
      ⚠ **`_lext_norm` alone does not move the entry point either** — the Givens radius at :514 is
      the same class in the same function, and with `_lext_norm` repaired and :514 left alone an
      isolating fixture (n = 1, A = a·I, where both `_lext_norm` calls see exactly 1 and 0) still
      fails **463 of 1001**.
      ⛔ **Six of thirteen mutants survived the end-to-end solver sweeps** and needed the helpers
      tested by name: a Krylov solve on A = 2I converges in one iteration whatever the residual
      norm reads, so a norm 33% wrong still yields the right answer. **A consequence test cannot
      discriminate a band the consequence smooths over.**
      ⚠ **One fixture of mine was measuring itself**: scaling the objective scales the gradient but
      NOT the step, so `x + α·d` falls below the ULP of `x` and the line search stalls — the mirror
      of the trap the 2.15.0 sweep documents. The valid metric is the silent-success count, and
      that is what is asserted.

- [x] ✅ **DONE — the linalg_precision QR sweeps (6 sites, 11 mutants, 7 killed + 4 recorded).**
      `:481/:516/:669/:716/:1156` plus the Wilkinson shift at `:1130`.
      ⛔ **`svd_golub_kahan` lost the small block for 472 of 999 ratios and ALL 472 WERE SILENT**
      (`rc = HSB_ERR_NONE`); `eigen_qr` lost it for **463 of 998** and took the LARGE block's
      eigenvalues down with it via `HSB_ERR_NO_CONVERGENCE`. Both are 0 now.
      ⛔ **THE SITE THAT MATTERED ON THE EIGEN SIDE WAS NOT ON THE CENSUS LIST, and the census had
      explicitly cleared it.** `:1132` was filed SAFE_UNREACHABLE — correctly, about the `f64_sqrt`
      — but the loss is one line above, `e2sq = e2*e2`, which flushes and collapses `mu` to `d2`.
      An unshifted step on a symmetric block with equal diagonals is a **fixed point**: the
      rotation swaps two equal entries and flips a sign forever. **It surfaced because repairing
      the Givens radius 60 lines below changed nothing measurable — a repair that moves no number
      is pointing at a second defect**, which is the 2.12.0 lesson repeating.
      ⭐ The fix forms no square at all: `mu = d2 - e2*(e2/denom)` with
      `denom = sign(delta)*(|delta| + hypot(delta, e2))`, which cannot cancel.
      ⚠ **Four mutants survive and are recorded in `tests/hisab.tcyr` with their reasons** — two
      are a genuine gap (the QR sweep is self-correcting, so a 13%-wrong rotation in the subnormal
      band is absorbed within 4 ulp), one is an equivalence kept anyway, one is a coupling.
      ⚠ **`_lp_split_zero_diag` needs a RANK-DEFICIENT block**: the 999-ratio full-rank sweep could
      not reach it and both its mutants survived until a zero-diagonal fixture was added.

- [x] ✅ **DONE — the last two sites (8 mutants, 7 killed + 1 correctness equivalence).**
      **`cga_norm`** was wrong on **992 of 2041 binades** (`|k*e1|` returning 0 at 2^-600) → 0.
      ⭐ Two design constraints, both measured rather than argued: the rescale MUST be by a POWER
      OF TWO — with an arbitrary divisor the exact cancellation breaks and a null point's norm
      comes back non-zero at every extreme scale — and the gate MUST be on the INPUT RANGE, because
      `nsq == 0` is the legitimate and common answer for every conformal point, so this is the one
      site in the tier where the naive result does not announce its own failure.
      ⚠ Its power-of-two floor could not reuse `_lp_pow2_floor`: that reads the exponent field,
      which is 0 for every subnormal, so it returns 1.0 and rescales by nothing.
      **`calc_monotone_cubic`** projects onto the Fritsch-Carlson circle, so it is a norm-scaled
      projection; `alpha² + beta²` overflowed at hypot ≥ 2^512, `tau` became +0 and the segment was
      FLATTENED — **20% wrong, and still monotone**, so the function's own oracle passed.
      **488 of 941 secant ratios wrong → 0.**
      ⚠ Only the `+Inf` leg: `s > 9` is guaranteed one line above, so the subnormal half of the
      class does not exist there and an `F64_TINY` leg would be dead code.

- [x] ✅ **DONE — Neumaier compensation, and the first failure moves 2^28 -> 2^538.** `cga_norm_sq`
      is the scalar part of `mv * rev(mv)`, and the scalar part comes from exactly the 32 DIAGONAL
      blade pairs — verified: of the 1024 pairs, 32 give blade 0 and every one has `i == j`. So it
      stopped building a whole multivector to read one number out of it, and accumulates with
      NEUMAIER compensation. **993 of 2041 translator scales wrong -> 482, first failure 2^28 ->
      2^538.**
      ⭐ **Kahan is not enough and the mutant proves it**: Kahan folds its correction into the next
      ADDEND, and with `sum = 1`, addend `2^54`, the correction 1 is below the ulp of 2^54 and is
      lost. Neumaier branches on which operand is larger and keeps the correction to the end.
      ⚠ **The residue past 2^538 is inherent**: rescaling by `s` makes the scalar term `1/s`, whose
      square underflows once `s > 2^537`. A translator at 2^539 has coefficients spanning 538
      binades and their SQUARES span 2^1076 — wider than the double range, so no single scale keeps
      both ends.
      ⛔ **AND IT IS A BEHAVIOUR CHANGE ON CONFORMAL POINTS, recorded rather than buried.** The old
      accumulation returned exactly 0 for 1010 of 1101 point positions — but that 0 was ROUNDING
      LUCK: `cga_point` adds `x²/2` to `±0.5`, and for small x that term rounds away entirely, so
      the multivector it stores has `norm_sq = x²`. The compensated sum reports that truth and exact
      zeros drop to 53. ⭐ Judged against the point's own largest coefficient — the unit-weight `e_o`
      part, which is the right scale — nullity is essentially unchanged (**1010 -> 970** within
      1e-14) and the WORST relative defect is identical either way, **exactly 2^-26 at x = 2^-27**,
      where `x²/2` falls below half an ulp of 0.5. Original text follows.
      ~~**`cga_norm` on versors and points is CATASTROPHIC CANCELLATION, not this class.**~~

- [x] ✅ **CHARACTERISED, PINNED, AND THE REAL REPAIR FILED — it is a WRONG ANSWER, not a
      representation nicety, and it is BOTH tails.** Measured: `P1.P2 = -d^2/2` recovers d^2 with a
      relative error with **median 46.4x at 2^-30 and 48.7x at 2^+30** — symmetric tails, 0 at 2^0.
      ⚠ A MEDIAN: rel ~ 2ab/(a-b)^2 diverges as the pair closes, so a max is whatever the nearest
      pair drawn happened to be. An earlier draft quoted maxima as ~2000x/~32000x and read a 16x
      asymmetry into them; there is none. Samples past 1e-6 at both
      ends. Clean bands (0/400 past tolerance): 1e-9 -> 2^-7..2^3, 1e-6 -> 2^-11..2^10,
      1e-4 -> 2^-15..2^13. ⛔ **"Small coordinates" was an artifact of a POWER-OF-TWO sweep** --
      dyadic x makes x^2 exact, the only reason it looked banded; `|P.P|/q` is a U centred on |x|~1,
      reaching total loss at BOTH 2^-30 and 2^+30.
      ⭐ **No arithmetic fix exists and it is proven**: at x = 2^-30, (q-1)/2 sits more than a full
      ulp below the rounding boundary of 1/2, so **-1/2 IS the correctly-rounded ep** (asserted
      bit-exactly). The ep/em basis stores q as the sum AND difference of two nearly-equal numbers.
      ⭐ **THE REAL REPAIR IS A BASIS CHANGE, FILED SEPARATELY**: in the null basis (n0 = (em-ep)/2,
      ninf = ep+em) a point is p + (q/2)*ninf + n0, q lives in ONE coefficient, and P.P cancels the
      same computed q against itself -- exactly 0. That touches the 32-slot blade product tables, not
      `cga_point`, so it is a release of its own rather than a bite.
      ⚠ 7 assertions pin the band AND the two known failures, so a future repair must announce
      itself. ⛔ My own first fixture repeated the artifact it documents -- dyadic values, built with
      the SUBNORMAL shift form at a normal exponent (masked 1044 -> 20, testing a value 288 decades
      off), yielding a NaN that `f64_gt(NaN, 1)` read as "not wrong". 3 mutants killed, no-op
      control survived. ⛔ ORIGINAL ROW FOLLOWS.
- [x] ✅ **DONE in 2.21.0 — the CGA null basis shipped.** Median `|P.P| / q` went **1.0 -> 4.48e-17 at
      2^-30 and 1.0 -> 4.33e-17 at 2^+30**, flat rather than a U. ⚠ **NOT "exactly null", and the
      difference is the release's main finding**: bit-exact nullity needs 2.18.0's Neumaier compensation
      dropped, which fabricates a scalar `0` where the answer is `1` for 6 of 11 magnitudes. Reverted;
      the claim is the sub-ulp residual. ⛔ The translator half of this row was **NOT** repaired and the
      claim that it was is false: both trees fail 485 of 2046 binades, first at 2^539 — identical.
      ⛔ ORIGINAL ROW FOLLOWS.
      **`cga_point` cannot represent a small point as exactly null**, and that is the
      ⛔ **RE-MEASURED 2026-09-10 AND THIS ROW'S SCOPE IS WRONG.** "A small point" / "below ~2^-26.5"
      is an artifact of the power-of-two sweep that produced it — dyadic `x` makes `x^2` exact, which is
      the only reason the failure looked banded. With FULL-MANTISSA coordinates `cga_norm_sq` is non-zero
      at **500/500 trials in every binade from 2^-20 to 2^+20**, and **458/500 even for coordinates in
      [1,2)**. Same shape as the 8-vs-536 ganita estimate: the figure came from the previous probe, not
      from the class. Scheduled on 2.20.0 as a repair.
      [measured: full-mantissa binade sweep vs the original dyadic sweep, 2026-09-10]
      residue above rather than an arithmetic fault. It stores `x²/2 ± 1/2` in the ep/em basis, and
      for `x` below ~2^-26.5 the `x²/2` term is under half an ulp of 0.5 and is gone. Storing the
      `e_inf`/`e_o` coefficients directly instead would hold it — an internal basis change for the
      whole module, so it is a design decision, not a repair. `cga_norm(cga_translator(t,0,0))`
      must be exactly 1 (a translator is a unit versor, `e_inf² = 0`) and stops being so at
      **t = 2^28** — an entirely ordinary translation distance. `cga_norm(cga_point(x,0,0))` must
      be exactly 0 and stops at **x = 2^-27**. Both are the accumulation ORDER inside
      `cga_geometric_product`: `1 + t²/4 - t²/4` loses the 1 once `t²/4` passes 2^53. A
      power-of-two rescale is exact, so it cannot and does not help. ⚠ The 2026-09-09 census filed
      these two as reach chains for the norm site; measurement says they are a separate defect.
      ⭐ Related, one level up and also unfixed: `cga_point`'s own `x² + y² + z²` is a naive sum of
      squares that overflows above 2^511 before `cga_norm` ever runs.

- [x] ✅ **DONE — `_lp_pow2_floor` now balances subnormal matrices.** The exponent-mask arm returns
      `F64_ONE` — "do not balance" — for every subnormal, so the one input that most needs balancing
      never got it. With a mantissa-highest-bit arm, `eigen_qr` is exact to **2^-1070**, four binades
      off the smallest subnormal there is (22 of 1061 magnitudes failing → 0).
      ⚠ **The fixture that pinned the old boundary could not see the repair**: it sat at 2^-1038 and
      2.17.0's norm work had already reached 2^-1039, so a mutant reverting this survived until the
      assertion moved to 2^-1070. **The depth has to follow the repair.**

- [x] ✅ **DONE — the SVD Wilkinson shift no longer forms `B^T*B` in degree four.** `tr*tr`,
      `a11*a22` and `a12*a12` are degree FOUR in the bidiagonal entries, which puts the floor at the
      FOURTH root of the subnormal floor — 2^-268.5, and the measured boundary was 2^-269 exactly.
      The degree-two closed form (`mu = a22 - a12*(a12/(delta + sign(delta)*hypot(delta, a12)))`)
      plus power-of-two scaling of the six operands takes it to **999 of 999 ratios correct**.
      ⚠ The rearrangement ALONE was not shippable: it reached 512 ratios but opened a five-ratio
      SILENT WRONG band at 2^-514..2^-518. **Trading 5 loud failures for 5 silent wrong answers is
      not an improvement**, which is why the scaling is part of the same change.

- [x] ✅ **DONE — an exactly zero off-diagonal was never deflated.** All five deflation tests read
      `f64_lt(|off|, EPSILON_F64 * (|a| + |b|))`, and for a sub-block at 2^-1050 that product is
      below the smallest subnormal, i.e. exactly 0 — so the test reads `|off| < 0`, which is false
      for **every** `|off|` including zero, and a block that had fully decoupled was never
      recognised. Measured across the 72 subnormal block ratios: **37 failing → 28.** Five sites now
      share one helper.
      ⛔ **THE CAUSAL STORY I WROTE FIRST WAS WRONG AND MEASUREMENT CAUGHT IT.** I filed this as
      *"the product flushes, so divide instead"* and implemented the division. Running the PRODUCT
      form with the new zero arm gives the identical 28 of 72, and the division's mutant survives
      every fixture — it is equivalent, because on the subnormal grid there is no representable
      non-zero value below `1e-12 * scale`, so the only case the two forms could disagree on is
      exactly the one the zero arm answers. **The zero arm is the whole repair**; the division was
      reverted rather than shipped as a rewrite with nothing behind it.
      ⭐ **And the shape of the zero test is load-bearing**: `f64_eq(|off|, 0)`, not
      `f64_gt(|off|, 0) == 0`. The latter is true for NaN too — measured, with it installed a 2x2
      whose off-diagonal is entirely NaN returns `HSB_ERR_NONE` with **fabricated eigenvalues
      (3, 3)**. Same trap as `_opt_armijo`, and the same one that made 2.17.0's SVD figure wrong.

- [x] ✅ **MEASURED, THE FALSE CLAIM RETRACTED, THE FIXTURE FAMILY WIDENED — repair filed, not
      shipped.** This row's own instruction was "widen the fixture family before touching the code";
      that is done, and the widened family says the standing claim was false in BOTH halves.
      ⛔ **It is a defect**, and ⛔ **it mostly does NOT report NO_CONVERGENCE**: over 8 fixtures x 52
      subnormal scales (416 rows) against a 120-digit oracle built from the ACTUAL rounded entries,
      **270 rows return HSB_ERR_NONE and only 57 are right** — 213 confident wrong answers, worst
      **9.78x**, one of them a smallest singular value returned as exactly ZERO where the truth is 4
      units of 2^-1074. ⛔ The old sweep could not have seen it: all its fixtures are
      UPPER-TRIANGULAR. Upper is 56/60/92 (correct/loud/silent); a GENERAL 2x2 is **1/86/121**.
      ⚠ The verifier's 2.19.0 refutation of the obvious fix STANDS and is why nothing is shipped
      here. 7 assertions pin the true state as a tracked defect with an acceptance test.
      ⛔ ORIGINAL ROW FOLLOWS.
- [ ] **[filed for its own release — subnormal SVD: scale the active block]** ⚠ **The remaining 28 subnormal block ratios are the GRID, not a defect —
      ⛔ **"THE GRID, NOT A DEFECT" IS REFUTED, AND THE REPLACEMENT IS NOT READY EITHER.** Scaling the
      block into the normal range before the QR sweep answers **all 75 ratios** on the SAME grid, worst
      **2 ulp**, 600/600 across eight fixtures — so the grid was never the binding constraint; the
      binding constraint is that the sweep does its arithmetic ON the subnormal grid, where every
      rounding is +-0.5 unit of 2^-1074. ⚠ **But an adversarial verifier refuted it on SCOPE and that
      refutation stands**: all eight fixtures are UPPER-TRIANGULAR 2x2 blocks, the one family in which a
      larger upstream defect is a no-op, and on a general 2x2 the same change converts LOUD failures
      into gross SILENT wrong answers — the exact trade it was adopted to avoid. Widen the fixture
      family before touching the code.
      [measured: 6 fixtures x 75 ratios vs 80-digit Decimal truth; 79 rows rc=NONE and >2 ulp wrong]
      measured, not assumed.** At 2^-1050 with **max_iter = 100,000** the superdiagonal is stuck at
      ONE unit of 2^-1074 (the smallest non-zero value that exists) against a diagonal of 2.7e7
      units, so the achievable ratio floor is 3.7e-8 and a relative 1e-12 deflation criterion is
      unreachable by construction. `svd_golub_kahan` reports `HSB_ERR_NO_CONVERGENCE`, which is the
      honest answer to an unanswerable question. **Open only as a decision**: whether to deflate at
      the grid's own resolution and accept a ~1e-8 relative error on such a block, or keep failing
      loudly. Not a defect either way.

- [x] ~~**[2.18.0]** **`_lp_pow2_floor` skips balancing for a subnormal matrix.** It reads the
      exponent field, which is 0 for every subnormal, and returns `F64_ONE` — so an all-subnormal
      matrix runs the whole decomposition unbalanced. Measured: the 2.17.0 norm repairs move
      `eigen_qr`'s first failing magnitude from 2^-1025 to 2^-1039 and **22 of 1061 magnitudes
      still fail**; the residue is this, not a norm site. Separate defect, separate fix.

- [x] ✅ **DONE in 2.18.0 — and it did NOT need the algorithm change this row predicted.** The row
      below says an implicit (dqds / Demmel-Kahan) shift is required. It is not: the Wilkinson shift
      has a degree-TWO closed form, and with the six operands scaled by a power of two first the
      floor is gone entirely — **999 of 999 block ratios correct, down to 2^-1000**. ⚠ The row was
      also wrong about where the floor was: it says "dies below 2^-537", and the measured boundary
      is **2^-269**, because `tr*tr`/`a11*a22`/`a12*a12` are degree FOUR, not two. Original text
      follows. ~~**The SVD Wilkinson shift forms B^T*B explicitly.**~~ `_lp_bidiag_qr` builds
      `a11`/`a12`/`a22` from `d*d` and `f*f`, so its shift dies below 2^-537 the same way the
      symmetric one did — but unlike the symmetric case there is no two-line algebraic
      rearrangement: it needs an implicit (scaled) shift, which is an **algorithm change**. The
      2.17.0 repair of `:669` therefore recovers bidiagonal entries in (2^-537, 2^-268.5] and
      nothing below, and its source comment says so rather than claiming the class is closed.

- [x] ✅ **REFUTED — `geo_advanced:1962` (`_toi_near_point`) is not a norm-class site.** Unanimous
      3/3: eight lines above it, `if (f64_le(vv, 0) == 1) { … return 1; }` makes the flush-to-zero
      band unreachable at that line. ⚠ Recorded rather than dropped: the verifiers note the function
      is still not scale-free for other reasons (the `f64_le(vv, 0)` escape itself reports a false
      impact below 2^-537, and the optimality test runs on the degraded `vv`). That is a separate
      finding about `time_of_impact`, not about this line.

### EPA — one routine, three entangled questions

~~All three touch `gjk_epa_*` and none should be done alone: they share a benchmark and a live filing.~~
⚠ **Refuted 2026-09-09 — they are not entangled.** `_epa_degenerate_normal` has a single caller behind
a double cold guard, and none of the **four** EPA benchmarks can enter that branch, so the squared-epsilon
sites do **not** share a benchmark with the other two. The three questions are independent and one of
them is not a defect at all.

- [x] ✅ **The three squared-epsilon sites — SHIPPED IN 2.15.0.** All three repaired and
      mutation-proven: the two `e1` tests became exact-zero, the cross-product test became the
      dimensionless sin form this file already uses at :530 and :682. ⭐ Proving them needed a
      PLANAR degenerate simplex — crossing segments give a two-vertex one, where the code takes
      its perpendicular branch and the cross test never decides. ⛔ And a reachability probe
      returned the right answer BY ACCIDENT: a constant `+z` passes every existing assertion
      there, because they check only x = 0, y = 0 and unit length. Original text follows.
  > **Original text, kept for the record:** ~~**The three squared-epsilon sites.**~~ `geo_advanced.cyr:1056/1060/1067` compare
      `hvec3_length_sq` against `EPSILON_F64` — the same squared-vs-unsquared mistake 2.10.1 and
      2.10.2 repaired at six sites in `geo.cyr`. **Deferred, not dismissed**: tightening them is a
      narrowphase behaviour change on a routine with a measured cost, so it needs its own
      before/after rather than riding along with a geometry release.
      ⚠ **The stated blocker is wrong; the two real ones are:** `geo_advanced.cyr` has **no local tiny
      constant** — a repair needs `_GA_F64_TINY = DBL_MIN` plus registration in
      `scripts/check-constants.sh` — and `grep -rn "degenerate_normal" tests/` returns **zero**, so a
      repair currently has nothing to prove itself with. Write the test first.
      The disposition table for all nine sites is in
      [`issues/archived/2026-08-10-squared-epsilon-guards-in-geo-ray.md`](issues/archived/2026-08-10-squared-epsilon-guards-in-geo-ray.md).
- [x] ⛔ **DECLINED 2026-09-10, on a measurement that inverts the filing's premise.** The trade was
      carried since 2.9.3 as a judgement call whose price was unknown. Re-measured on the 2.17.0
      tree, the price is not the cost — it is the **ACCURACY**, and the direction is the opposite of
      what the filing assumes.
      **Random overlapping spheres judged against the EXACT closed form `ra + rb - |c1 - c2|`** (no
      reference implementation, no harness):
      | | mean | worst |
      |---|---|---|
      | `gjk_epa_3d` shipped, 1200 pairs | **1.41e-16** | **5.57e-16** |
      | `gjk_epa_3d` upgraded, same pairs | 1.70e-14 | **2.03e-11** — 36,000x worse |
      | over a wider 4000-pair sweep | 3.40e-10 → 2.85e-09 | 1.32e-06 → 1.14e-05 — 8.4x worse |
      ⭐ **THE MECHANISM WAS ALREADY WRITTEN IN THE SOURCE.** `_epa_polish`'s own comment states the
      returned depth is `min` over probed directions of `h_M(n)`, every `h_M(n)` is `>=` the true
      MTV, so the polish "can only lower an upper bound" — monotone. **Certifying is an EARLY-OUT
      that skips it.** The filing's premise is that certifying is the better state; it is the worse
      one, and the polish was delivering the accuracy all along.
      ⚠ **Two of the filing's four claims are false as written**: "every assertion in the suite
      passes" (the box-fallback assertion fails, 12 → 6 — the filing's own tripwire, added after
      that sentence was written) and "it alters no returned answer" (it alters them, for the worse).
      ⭐ **A tripwire now pins it** in `tests/modules.tcyr`, so the trade cannot be taken by
      accident: the upgrade fails two assertions.
      ⚠ Cost was never needed. Original text: ⚠ **The seed-upgrade trade — DO NOT ACT ON THE NUMBER BELOW; it is void.** The "+19.4%" was
      priced against a `gjk_epa_sphere_box` baseline of **125.6 µs** which now reads **78.5 µs**
      (cycc 6.6.x accessor inlining). Its own linked filing has said since 2026-09-09: *"Do not
      rescale the old percentage — re-measure both halves."* Both the baseline and the added work
      moved for the same reason and neither was measured after the move, so **the price of this trade
      is currently unknown** and the decision cannot be taken. Re-measure first. Original text:
      Giving `_epa_seed_gjk` the strictness upgrade `_epa_seed_portal`
      already performs closes the certification asymmetry between two public entry points and makes
      six previously-unmutatable EPA repairs observable — at a measured **+19.4% on
      `gjk_epa_sphere_box`**, for a change that alters no returned answer. 2.9.3 measured it and
      declined. A judgement call, not a defect; the change is one contained edit.
- [x] ~~**Sphere-family non-convergence**~~ ⭐ **ROOT CAUSE ESTABLISHED 2026-09-09 — and it is not a
      defect, so there is nothing to repair.** Holding the seed at the strict portal seed and varying
      **only curvature** gives a monotone series: a smooth sphere hands off **12** times, a 12-vertex
      icosahedron inscribed in that same sphere **4**, a box **0**. Certification requires
      `best_d - lo <= 1e-10 * max(...)` within `EPA_MAX_ITER = 64`; on a strictly curved boundary the
      achievable relative gap at ~132 faces is ~**1.5e-2**, seven orders away. **A polytope expansion
      cannot certify a smooth surface** — the handoff is the algorithm working, not failing.
      ⚠ **And the objection quoted against it is measured on the wrong shape**: the "+55% pays and
      gains nothing" verdict cites `gjk_epa_sphere_box`, a sphere-vs-**box** pair, which certifies
      **0 of 12** under the strict seed — i.e. the benchmark carrying the objection is one that
      *gains*. Original text, kept for the record: the genuinely open half of that filing and independent of
      the certificate wording: spheres do not certify *even with a strict seed*, and they pay the
      upgrade's cost without gaining anything. **Root cause not established.** This is what the
      filing is actually about now.
      → [`issues/archived/2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md`](issues/archived/2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md)

- [x] ✅ **DONE — and the cause was the POLISH BUDGET, not either of the two things this row guessed.**
      `_epa_polish` starts at 0.25 rad and breaks when its step falls under `EPSILON_F64`, which takes
      **38 halvings**; every round that MOVES spends budget without halving, so a cap of 64 left at
      most 26 moving rounds. `mpr_penetration` reaches the refinement through the PORTAL seed, which
      starts further out, so it was the one running out — **returning while still walking, not
      because it had converged.** Budget 64 → 128 (38 mandatory halvings + 90 moving rounds):
      `mpr_penetration` worst **1.65e-05 → 6.3e-16**, the same as `gjk_epa_3d` to the last bits.
      ⚠ **NO MEASURABLE COST**: the EPA benchmark rows moved +0.1% to +2.5% and the CONTROL rows that
      never reach EPA moved +1.6% to +2.9% in the same pair of runs — the board drifted and the
      change is not separable from it. Only calls that were hitting the cap spend the extra rounds.
      ⛔ **BOTH HYPOTHESES THIS ROW RECORDED WERE REFUTED BY MEASUREMENT.**
      (a) *"the cause is likely the portal seed's `v0`, built from the +x axis"* — **refuted**:
      swapping which axis carries the large offset component leaves the error IDENTICAL
      (3.167e-04 either way), and counting outliers per orientation bucket gives 2/1/1/2/1 across
      the range. The 10^5 spread in bucket MEANS that suggested the hypothesis was one outlier
      landing in one bucket — **the mean of a heavy-tailed sample is not a correlation measure.**
      (b) *certification skipping the polish* (the 2.18.0 EPA mechanism) — **refuted**: both entry
      points take the polish on every one of these cases.
      ⭐ The gate the row asked for exists now: a 1200-pair sweep asserting both public entry points
      agree with the closed form to 1e-14.
      Original text follows. ~~⛔ **`mpr_penetration` and `gjk_epa_3d` disagree by up to 1.6e-5 on ordinary
      overlapping spheres, and NOTHING checks it.** Found while re-measuring the seed trade, and it
      is larger than the trade was. Both are public, both compute the same quantity, and against the
      exact closed form over the same 1200 pairs:
      `gjk_epa_3d` mean **1.41e-16** / worst **5.57e-16**; `mpr_penetration` mean **1.37e-08** /
      worst **1.65e-05** — roughly **10^8 times** the error, on geometry with a one-line answer.
      ⚠ **The suite cannot see it**: `_ag_mprpen` (`tests/modules.tcyr:811`) compares the two entry
      points' RETURN CODES only, and no assertion anywhere compares their DEPTHS. That is the same
      shape of gap the 2.13.0 suite release was about.
      ⭐ Cheap to gate before it is cheap to fix: an assertion that the two public entry points agree
      on depth to a stated tolerance over a randomised sphere sweep. ⚠ The cause is likely the
      portal seed's `v0`, which `_epa_seed_portal` builds from the **+x axis** specifically — the
      error concentrates where the centre offset is near-parallel or near-perpendicular to it — but
      that is an inference from where the errors cluster, not a measurement.

### Decisions owed

- [ ] **[2.21.0, deferred with a measurement rather than a guess]** **Cut the null-table build's
      remaining four 32-slot passes per pair.** The build is lazy and one-time, and 2.21.0 took it from
      **2.7 ms to 0.72 ms** by hoisting `_cga_expand_null`/`_cga_contract_orth` out of the pair loop
      and iterating occupancy lists — but 2.20.0's first CGA call was **11 us**, so this is still a
      **65x one-time regression**, and it pays for itself only past **~430 products** (~709 us extra
      once, ~1.65 us saved per call after). What remains is four full 32-slot passes per pair —
      clearing `mid` and `res`, then scanning each — i.e. 131k iterations for at most 16 occupied
      slots. Tracking touched indices would cut most of it. ⚠ The `res` scan must stay in ascending
      bitmask order (the packing depends on it), so only its CLEAR can use the list.
      ⭐ **This is safe to do whenever**: the FNV contract `0xF4A98C5706D5CF5B` is asserted in the
      suite, so any drift in the table fails the build. **The reason to defer was that 0.72 ms is
      already small in absolute terms and a tag was pending — not that the change is hard.**
      ⚠ Alternative worth pricing first: emit the 1024 entries as a literal table. That makes the
      first call free, but costs ~1024 lines in the distlib bundle and gives up the "built by integer
      arithmetic, exact by construction" property that is currently the design's argument.
      [measured: first-call probe, 3 runs each tree, 2026-09-11]

- [ ] **[2.21.0]** **There is no CGA row among the 74 tracked benchmarks, and that is how a 2.7 ms
      first call stayed invisible.** Every CGA figure in 2.21.0 — the 2.22x on `point*point`, the
      10.93x regression at k = 32, the table-build cost — was measured with a throwaway probe, so
      none of them is in `bench-history.csv` and none will be checked again by anything. ⚠ A CGA row
      has to be designed around the lazy table: a benchmark that warms up **cannot see the build
      cost**, which is exactly the hole this release fell into, so the build wants its own one-shot
      row rather than an average. ⭐ At minimum: `point*point`, `cga_norm_sq` at k = 5 and k = 32
      (the two sides of the crossover), and a cold first-call measurement.

- [x] ✅ **KEPT, REPAIRED, AND WIRED TO PUSH — and the deciding fact is that the job has never run.**
      **[2.19.0]** **`scripts/check-measurements.sh`: keep it or delete it.** The gate runs PR-only on the claims
      a branch *adds*, detector recall 21/22 with 0/15 decoys, but paragraph-level false positives
      sit at ~21% and **522** unmarked pre-existing claims mean it cannot be widened past the diff
      without a marking campaign first. It has now earned its keep several times over — it caught
      the arc's fourth non-reproducing measurement, and it has forced provenance markers onto every
      claim added since 2.9.3. The open question is whether ~21% FP is tolerable on a gate people
      must not learn to ignore.
      ⭐ **The deciding question is MOOT, and the answer is keep.** The gate is wired
      `if: github.event_name == 'pull_request'` (`ci.yml:238`) and **this repo has zero pull
      requests** — 270 workflow runs since 2026-03-22, every one `event: push`, no merge commits,
      one branch. The job has reported `skipped` in all 51 runs since it was wired, 2.11.5 included.
      **Nobody has ever been shown a false positive**, so the ~21% FP rate has never cost anything.
      ⚠ And a full-tree scan today reports **2 BROKEN PROVENANCE MARKERS** which **no other gate
      catches**, in the mode CI does not run. ⛔ **An earlier draft of this bullet misdiagnosed
      them**: it said both "cite paths that have since moved to `archived/`", but
      `tests/modules.tcyr`'s marker already names an `archived/` path and that file exists. The real
      cause is that `check-measurements.sh` resolves marker paths **from the repo root** while hisab
      writes them relative to `docs/development/` — so the missing prefix is `docs/development/`, and
      the implied fix would have left both markers broken. A wrong diagnosis on a two-line repair is
      worse than none. The real
      decision is smaller and different: **should it run on push, not only on PRs?**
      ⭐ **ANSWERED: yes, for BROKEN markers; no, for the unmarked debt.** Verified against the GitHub
      API rather than inferred: **0 pull requests in the repository's entire history**, 303 workflow
      runs, the last 100 all `event: push`. So the `measurements` job has executed **zero** times and
      every finding `--diff` can produce has been unreachable. A new `Broken provenance markers` step
      runs on push, greps the full scan (whose exit code cannot gate — it is 1 on 284 unmarked
      paragraphs of debt), and is **verified to fire on both shapes and to recover**.
      ⛔ **THE COUNT WAS 2 AND IS 3, BECAUSE THIS RELEASE ADDED ONE.** The enum bite wrote a
      `[measured: ...]` wrapped across two comment lines — malformed to the single-line regex. All
      three are fixed; the two path markers needed the `docs/development/` prefix this row had already
      worked out. Tree-wide broken markers: **0**.
      ⛔ **AND `--diff` COULD NOT SEE ITS OWN DOMINANT CASE.** A path-resolution failure was filed
      against the PARAGRAPH'S first line while the malformed arm filed the marker's own line, and
      `--diff` keeps only findings on lines the branch ADDED — so a bad marker appended to an EXISTING
      block was intersected away. Measured, one marker, two placements: appended at `src/vec3.cyr:126`
      `--diff HEAD` exits **0** and prints *"Every measurement claim added on this branch names its
      source"*; as its own new paragraph it exits **1**. ⚠ Worse than silent — the fabricated marker
      SATISFIED the real claim beside it, so tree-wide unmarked fell by one and the gate got GREENER
      for the bad edit. Fixed to record the marker's line.
      ⚠ **Still open and deliberately not gated**: `--ratchet` is red (**521 unmarked against a 417
      baseline, 15 files risen**) — pre-existing debt across 2.14.0-2.18.0, unchanged by this repair
      and needing a marking campaign, not a gate.
- [x] ✅ **DECLINED — and the decline SURVIVED both skeptics.** **Whether `dual_*` should gain a vector layer.** 2.10.0 answered "no dual vectors" for the
      **[2.19.0]** Measured rather than argued: **0 call sites of `dual_*` or `ad_*` across all 14
      hisab-consuming repos** (every one of 920 grep hits is inside a vendored `lib/hisab.cyr`), and the
      crossover is close — vector-forward wins below n=8, TIES at n=8, and the tape is 1.43x ahead at
      n=16 and 2.2x at n=32. Reverse mode already covers the regime that would motivate it.
      ⭐ **A finding fell out that is NOT about the vector layer and is worth more than it**: `ad_grad`
      clears all n+3m adjoints and walks from the root to index 0 on EVERY call, so a multi-residual
      Jacobian driver built on one shared tape is **O(m^2) by construction**. Resetting the tape per
      residual uses only shipped API and adds no source: **6.8x at n=8,m=512** (1,736,153 -> 256,043 ns),
      2.8x at n=4,m=128, 1.4x at n=3,m=32. Filed on 2.20.0.
      [measured: 3-run probe, independently reproduced by a second agent, 2026-09-10]
      *geometry*, on measured grounds (allocation per op, one seed per pass), and 2.11.0's
      reverse-mode tape removed the pressure entirely for the many-input case — one sweep instead of
      n passes. What remains is whether a *small fixed-size* vector dual is worth it for callers who
      want a Jacobian rather than a gradient. **No consumer has asked** — and now measured:
      **0 call sites of `dual_*` or `ad_*` across all 14 hisab-consuming repos.** The scalar API this
      would extend has no external users either, and the one in-tree Jacobian contract
      (`optimize.cyr:652`) is already satisfied by per-root reverse sweeps. Leave parked.

### Documentary

- [x] ✅ **BOTH CLOSED, and the direction claim is now measured rather than inherited.** ⛔ The
      CHANGELOG recorded the prune's divergence BACKWARDS, twice, for thirteen releases. Built the
      **2.7.0 tree and HEAD side by side on cyrius 6.6.2**: `(0,0),(1,2),(1,1),(0,2),(2,1)` gives
      **old 9 (COMPLETE) -> new 6 (PARTIAL)** — old [0 1 2 4 0 2 4 2 3], new [4 0 1 1 2 3]. The new
      code BAILS WHERE THE OLD SUCCEEDED. The decision is unchanged; the sentence was wrong, and it
      lived only in the CHANGELOG **where no gate could reach it** — now stated in the source beside
      the code and pinned by an EXACT-length assertion (`== 6`, not the old `<= 9`, which the
      complete answer also satisfies and so could not discriminate).
      ⭐ The shadow is renamed `prev_slot`/`next_slot`, verified inert by a 65,610-case checksum
      **whose ability to detect a change was itself verified** (2 of 3 mutants move it; `i - 2` is an
      equivalence on this family). ⚠ The filing's `nn` shadow does not exist — the winding loop uses
      `ni`. ⚠ And the citation, which had drifted three times (740/742 -> 762/764 -> **870/872**), is
      now moot: the names are unique, so there is nothing left to cite. ⛔ ORIGINAL ROW FOLLOWS. **Two items inside the archived `triangulate_polygon` filing** — the 2.7.x CHANGELOG entries
      ⛔ **EVERY FIGURE IN THE FILING REPRODUCED INDEX-FOR-INDEX** under an independent exact-integer
      simplicity test — the n=5/3x3 sweep (15,120 cases, 1,696 diverged, 32 full->partial), the n=6
      sweep (60,480 / 11,864 / 632), and **0 divergences over all 106,888 simple polygons**. A skeptic
      could not break a single number. ⚠ It is refuted on SCOPE only: the proposed change over-reaches
      in the suite and under-reaches on the live source contract, so it must not ship as written.
      ⚠ **The citation has now drifted three times for the same two lines** — `collision_core.cyr`
      740/742 in the filing, 762/764 in the 2026-09-09 confirmation, and **870/872 actually**. This
      row's own closing line, "a citation that drifted is how a five-minute fix becomes an
      investigation", is now true of the row itself.
      state the prune's divergence direction backwards on at least one reproducer, and the
      neighbour-refresh locals shadow the winding loop's `pn`/`nn`. Neither changes behaviour; both
      were recorded rather than buried when the file was archived.
      ⭐ **Both confirmed 2026-09-09, and both citations in this bullet were wrong.** The direction
      claim reproduces on 6.6.2 — `(0,0),(1,2),(1,1),(0,2),(2,1)` gives old **9** indices (COMPLETE)
      → new **6** (PARTIAL), i.e. the CHANGELOG has it backwards, and the *same release* already
      states it correctly 630 lines apart (`CHANGELOG.md:3508`). ⚠ The shadow is at
      **`collision_core.cyr:762,764`**, not the `740,742` this bullet and the archived filing both
      cite — and the winding loop has no `nn` at all, so only `pn` is a true shadow. A citation that
      drifted is how a five-minute fix becomes an investigation.
      → [`issues/archived/2026-08-04-perf-triangulate_polygon.md`](issues/archived/2026-08-04-perf-triangulate_polygon.md)

### Toolchain, tracked upstream

**One filing is open here** (`docs/development/issues/`, **26** filings archived beside it, plus a README). ⚠ **Cyrius bugs are filed in the CYRIUS repo**, not this one — `cyrius/docs/development/issues/` is where the language agent reads them; only hisab's own items and hisab's record of a live upstream workaround belong here:

| filing | state |
|---|---|
| ~~upstream: `2026-09-09-hisab-derive-accessor-simd-dst-slot.md`~~ | ✅ **FIXED in cycc 6.6.2 and archived upstream** — the pin is 6.6.2, so this row is discharged. ⚠ The workaround hoist in `src/mat4.cyr` is **no longer load-bearing** (verified: removing it passes hisab 416 / foundation 351 / abuse 741, rc=0) and is kept only for consistency with `m3_mul_vec3`. ⛔ **This row also propagated a framing upstream REFUTED**: it was filed as derive-specific and as a 6.5.71 regression and was **neither** — all 21 SIMD handlers were affected, reachable since 6.0.70; 6.5.71 only removed the call that had been forcing the spill. **A first-bad-version is evidence about visibility, not origin.** |
| `2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md` | ⚠ **hisab's own, not a toolchain item** — the EPA tier above. |

#### Capabilities 6.5.19–6.5.33 opened, filed rather than taken in 2.11.2

A toolchain bump is not the place to adopt features, so these were verified as available and left.
Each is a real, checked capability, not a hunch — the ones that were checked and found **not** to
apply are named too, so a later reader knows they were examined rather than skipped.

| item | state |
|---|---|
| ~~**[2.19.0]** **Negative enum values** (6.5.32, `enum E { NONE = -1; }`)~~ | ✅ **SHIPPED. 12 codes converted, 6 mutants installed and 6 killed — but only after the sweep found the suite could not see a wrong value at all.** ⛔ **TWO MUTANTS SURVIVED ALL 3940 ASSERTIONS**: `HSB_ERR_SINGULAR_MATRIX -2 → -22`, and `HSB_ERR_ALLOC -11 → -1`, which makes ALLOC **collide with `HSB_ERR_INVALID_TRANSFORM`** — two distinct failures reported under one code. The cause is structural, not an oversight in any one test: every existing assertion compares a return against a code BY NAME, so both sides move together and the value cancels out. Closed by 15 assertions that name the NUMBER and check all 66 pairs for distinctness (**3940 → 3955**); the pair COUNT is asserted too, because a loop that ran zero times would report zero duplicates. ⛔ **AND THIS ROW'S OWN JUSTIFICATION NAMED THE WRONG MECHANISM — so did CLAUDE.md's principle.** Both said enums cost zero `gvar_toks` vs `var` globals. Measured with `CYRIUS_STATS=1` over the bundle: **`var_table` is 615 on BOTH sides**, and cycc's own diagnostic counts "globals+enums+arrays", so an enum member occupies an entry exactly as a global does. The real saving is RELOCATIONS — **`fixup_table` 3092 → 2933 (−159)** and **`code_size` 748,024 → 747,320 B (−704)** across the 170 non-comment reference sites, ~94% of sites losing a fixup at ~4.4 bytes each. CLAUDE.md's principle line is corrected in place. ⭐ **The row's safety claim was TRUE BUT NOT WIRED TO ANYTHING.** A duplicate `var` global is **completely silent** (last wins, the wrong value ships); a duplicate enum member does get `warning: duplicate symbol 'X' redefined with conflicting value` at file:line — but `cyrius check --with-deps` prints it and **exits 0**, and `cyrius lint` never emits it, so the CI `^  warn ` grep matches zero times. Verified end to end with a real second `HSB_ERR_ALLOC = -3`: consumer-build exit 0, lint clean, only the new test block failed. A CI step now greps for it, **verified to fire and to recover**. ⚠ The `check-constants.sh` widening this row offered as optional is moot: the gate's value regex needs a ≥12-digit hex literal and these are small decimals, so it never reached them and still does not. |
| ~~**[2.19.0]** **`CYRIUS_PKG_VERSION`** (6.5.21)~~ | ✅ **ADOPTED. `src/main.cyr` no longer carries a version literal, and the output is byte-identical** (`hisab 2.18.0\n`, 13 bytes, binary 257,176 B both ways). ⚠ **Re-probed on 6.6.2 rather than taken from this row**, which is the rule this row's own "re-verified" annotation invites you to skip: the symbol resolves from the ENTRY file and from an INCLUDED file, and a negative control confirms the VALUE TRACKS — with `VERSION` set to `9.9.9-probe+z` the built binary printed exactly that. ⛔ **MY FIRST INCLUDED-FILE PROBE RETURNED `2.17.0`** — neither the real version nor the control — because I ran it with `cd` into the scratchpad, where a **stale `cyrius.cyml` from an earlier session** was picked up instead of hisab's. Re-run from the repo root it reads 2.18.0. *Check the probe before believing the probe*, for the third time in this release. ⭐ **THE GATE SWAP IS A NET GAIN, MEASURED.** With `VERSION` bumped to 2.19.0 the old hardcoded file rebuilds and still prints `hisab 2.18.0` — **the 2.9.1 -> 2.9.2 defect reproduced** — while the adopted form prints 2.19.0. The replaced grep compared committed TEXT to committed TEXT; the new `Verify printed version` step **runs the binary**, the first gate in either workflow that does. ⛔ **BOTH REPLACEMENT SOURCE GUARDS WERE WRONG ON THEIR FIRST DRAFT, IN OPPOSITE DIRECTIONS, AND ONLY RUNNING EVERY ARM FOUND IT.** The "no hardcoded literal" guard FAILED ON ITS OWN TREE, because the comment explaining the change quotes the retired `println("hisab 2.18.0")`; the "symbol still used" guard was VACUOUS the other way, matching that same comment's five mentions of `CYRIUS_PKG_VERSION`, so deleting the call and printing a bare `hisab` still exited 0. Both now strip comment lines; all four arms verified (clean 0, literal-back 1, symbol-deleted 1, restored 0). ⛔ **AND THIS ROW'S "both still assert" WAS FALSE.** `ci.yml` contains the sentence **zero** times (`:285` is inside the `#must_use` step) and `version-bump.sh` carries it only at `:24`, inside a 2026-09-09 note saying it is false. Both had been excised — sloppily, splicing the correction into the MIDDLE of a sentence in each file, which is why they still read as assertions. The two LIVE claims were in files this row never names: `threat-model.md:147` and `doc-health.md:657`, both now corrected. ⚠ The stale claim was true for **four days**: written 2026-08-09, falsified by 6.5.21 on 2026-08-13, and it stood four more weeks. ⛔ **STANDING CONSTRAINT**: `CYRIUS_PKG_VERSION` resolves from the **building** package's manifest, so it must never enter a `[lib]` module — a consumer compiling `dist/hisab.cyr` would get THEIR version. Safe by construction here (`src/main.cyr` is `[build] src`, not `[lib]`; the string occurs 0 times in the bundle) and now gated. |
| **[no version — in force already]** **`bench_run` auto-batching** (6.5.19) | Already in force — it is what moved the 44 rows. The **36 `bench_batch()` call sites are deliberately unchanged**: they now buy a FIXED window rather than escape the floor, which is still worth having when comparing two runs at identical batch sizes. Re-evaluate only if a reason appears. |
| ~~**[2.19.0, as the noise band]** **A `load` column for `bench-history.csv`**~~ ✅ **BOTH DECLINED — the band is MEASURED STRICTLY WORSE than the filter it would replace.** At every (K, floor) tried, to match the shipped global +-10%'s false-positive rate on clean replicate pairs (4 of 288) a history-derived per-benchmark band must widen to a **median 11.1% / maximum 55.0%**, at which point it flags **67** code-changed cells where the global filter flags **96**. This row called the band "the distinct and more useful item"; the only data that exists to build one from refutes that. ⚠ A band computed from history also AUTO-WIDENS after a real regression, hiding it. ⚠ Two instrument facts found in passing and worth keeping: the CSV's quantum is **1000 ns, not 1** for the 146 of 1152 rows above 1e6 ns (`_fmt_time` prints the next-smaller unit, so those are microsecond-resolution), and `bench-history.sh` renders every delta against the **OLDEST** retained run, not the previous one. ⛔ ORIGINAL ROW FOLLOWS. | Upstream records that a **~4% move in both metrics together** is box-wide contention, not a code change. hisab records neither load nor any second metric, so it cannot currently tell the two apart. ⚠ **Nothing technical blocks it; what is missing is a RULE THAT READS IT.** Upstream ships a prose ~4% heuristic and has no load column of its own, and the guard hisab actually adopted in 2.11.5 — re-run the identical binary and compare each row to **its own** spread — already settles the cases this would. A load snapshot at run *start* does not describe load *during*. ⭐ **The distinct and more useful item is a per-benchmark noise band** recorded in `benchmarks.md`, replacing its global +-10% filter; 2.11.5 did that by hand. File that instead. |
| ~~**[2.20.0]** **`_sym_render_f64` is a hand-copied `fmt_float_buf`**~~ | ✅ **SHIPPED — and the defect was neither the duplication nor the size this row gave it.** ⛔ **Above 2^63 BOTH public renderers returned ONE wrong constant for EVERY input**: `f64_to` saturates, so `expr_to_str` and `sym_to_latex` each answered **-9223372036854775808** for 1e19, 1e20 and 2^100 alike — a NEGATIVE number for a POSITIVE input — with `sym_to_latex(-1e19)` malformed as `{--9223372036854775808.000000}`, a double minus. All 3955 assertions passed, because nothing asserted any value above 1e15. ⛔ **THE ROW FOUND A SECOND RENDERER IT NEVER NAMED.** It is about `_sym_render_f64` in `symbolic.cyr`; `expr_to_str` forty lines below carries the same saturation with no magnitude guard at all. **Grep for the shape, not the function** — sixth release running that this held. ⭐ Repaired by exact decimal expansion (mantissa seeded into a digit array, doubled k times, **no f64 arithmetic in the result**), verified against Python's arbitrary-precision `int(float)`: **10,961 renderings, 0 mismatches** over the 961 binades 2^63..2^1023 plus 10,000 signed random mantissas. ⭐ **The second half was a CONTRACT repair**: `_latex_fmt_const` documents "Integer values render without decimals" and capped that at 1e15, so integral values from 1e15 up rendered `1000000000000000.000000` while `expr_to_str` rendered `1000000000000000`. The cap guarded `f64_to` saturation and sat ~3 binades below where saturation starts — matching neither the contract nor the hazard. The two renderers now agree on **1024 of 1024 integral binades**. ⚠ **My own marker was wrong before the code was**: the first draft claimed agreement over "2098 binades", a figure carried from an unrelated sweep before anything was run; it is 1024. And the marker was written across two lines, which the provenance gate rejects — **the second time this release cycle**. 11 assertions added (3955 → 3966), 6 mutants killed, no-op control survived. ⛔ ORIGINAL ROW FOLLOWS. | ⛔ **RE-MEASURED, AND BOTH THIS ROW'S FRAMING AND ITS COUNT ARE WRONG.** The divergence set is not 30 values: it is the **entire finite |val| >= 2^63 band — 7,702 of 30,432 probed inputs (25.3%), 961 of the 2046 normal binades**. The "30" was an artifact of the earlier probe's input distribution, which this row already flags as unreproducible. ⛔ And "current = well-formed, deleted = malformed" holds for POSITIVE inputs only: on the CURRENT tree `sym_to_latex(-1e19)` renders **`{--9223372036854775808.000000}`**, a double minus. So the shipped code is defective above 2^63 today, and the decision is not keep-or-delete but **what the correct rendering above 2^63 is**. ⛔ ORIGINAL ROW FOLLOWS. | 6.5.30's carry fix is now vendored in `lib/fmt.cyr`, making hisab's private copy a redundant duplicate carrying its own now-stale rationale comments. ⛔ **"Pure debt, zero output change" is REFUTED.** Over 30,149 probed inputs, **30 diverge**, all at \|val\| >= 2^63: `sym_to_latex(1e19)` gives `-9223372036854775808.000000` today versus a malformed `-9223372036854775808.-9223372036854775808` after deletion — and `symbolic_ext.cyr:211` routes \|val\| >= 1e15 down that branch **on purpose**. ⚠ The 41,998-input equivalence run is **not reproducible from the tree** (`git log -S"41,998"` finds only the commit that filed the claim). This is a **behaviour decision now, not a cleanup**. |

#### Closed by the 2.11.3 bump — both re-tested on 6.6.1, both fixed upstream

| filing | outcome |
|---|---|
| `2026-04-26-cyrius-cli-arg-clobbers-source.md` | 🟢 **Archived.** Carried four months as "deliberately never re-tested" because the reproducer destroys a source file. 6.6.1 guards it: `error: refusing to write build output over a .cyr source file`, exit **1**, source byte-identical across three argument shapes. ⚠ The exit code needed a second look — `cyrius … \| tail` reports 0 because `$?` after a pipe is *tail's*. |
| `2026-08-09-cyrius-dead-fn-bodies-are-never-syntax-checked.md` | 🟢 **Archived.** The underscore discriminator is gone and **`lint` — the half this was left open for — now catches it**. Control run: a *clean* uncalled no-underscore fn still passes `build`/`lint`/`vet`, so the probe discriminates rather than merely reddening. `vet` still exits 0 on unparseable input, deliberately not claimed as a defect: it is the dependency auditor, not a syntax gate. |

#### Opened by the 2.11.3 bump

| item | state |
|---|---|
| ~~**Give the benchmark harness resolution**~~ | 🔴 **REFUTED in 2.11.4, not built.** The premise was mine and it was wrong: the `net/floor` ratio compares a **per-op** net against a **per-clock-pair** floor, while `bench_run` sizes every batch so the clock is 1% of the window. Measured on a same-binary re-run, the tier it condemned is the **quieter** one (median 1.43% vs 2.10%). ⚠ It had suppressed most of 2.11.3's real result — `ray_aabb` −54.8%, `vec3_cross` −54.0% — so **a wrong instrument suppresses real findings as readily as it invents false ones**. The guard that needs no threshold: re-run the identical binary and measure the spread. |
| ~~**Migrate off ganita's deprecated aliases**~~ | 🟢 **DONE in 2.11.4** — 536 call sites over 20 of the 53 deprecated names, not the **8** this row estimated. ⚠ **The estimate was scoped from ganita's changelog paragraph rather than from its surface**, and was wrong by 67x; the deprecation block is a 53-entry table further down the same file. Suites byte-identical across the change; `svd_golub_kahan_12` **−27.85%** and `eigen_qr_12` **−27.67%** because the aliases were real `call`/`ret` pairs (`#inline` needs ≤2 params; `ganita_mat_get`/`_set` take 3 and 4). |
| ~~**A `load` column for `bench-history.csv`**~~ | ⛔ **SUPERSEDED — see the capability row above, which supersedes it with the opposite disposition.** Two rows for one item, disagreeing, is exactly what this file exists to prevent. What should be filed instead is a **per-benchmark noise band** in `benchmarks.md`, replacing its global ±10% filter: distinct from a load column (one separates box contention, the other replaces the filter), and 2.11.5 and 2.12.0 both did it by hand. Original text: this release saw 40 of 72 rows move in the same direction at once, and distinguishing "box-wide contention" from a real change had to be done by hand, using the flat numeric kernels as an ad-hoc control. A recorded second metric would make that mechanical. |

---
## Optional, demand-gated

- ~~**GPU compute via soorat** (feature-gated) — no consumer has asked.~~ ⛔ **DELETE — this row is
  pre-port and cannot be executed as written.** It was authored **2026-03-27**, for *Rust-era* hisab
  (`Cargo.toml` deleted 2026-04-15; `cyrius.cyml` added 2026-04-26), and "feature-gated" names a
  Cargo `[features]` key **the Cyrius package format does not have**. soorat today is 42 `.rs` files,
  **0 `.cyr`**, no `cyrius.cyml` — depending on it needs FFI, which `CLAUDE.md` forbids outright.
  ⭐ A superseding route already exists if a driver ever appears: **mabda** is ported and folded into
  the toolchain (`cyrius/lib/mabda.cyr`, opt-in include).
- **Adopt `vec_sort_by` / `vec_select_nth`** (cyrius 6.5.4) — consolidation onto stdlib, not a fix.
  ⛔ **"exactly one hand-rolled sort" is wrong by 6x — there are SIX ordering routines**, in five
  files: `collision_core.cyr:466`, `spatial.cyr:114`, `num_ext.cyr:309`, `linalg_ext.cyr:905`,
  `linalg_precision.cyr:816` and `:1265`. Three are near-identical descending-magnitude selection
  sorts whose own comments concede they have already disagreed once.
  ⭐ **So the wait-for-the-third-instance gate is DISCHARGED** — this is the sixth instance, not the
  first. ⚠ The same wrong count survives at `CHANGELOG.md:4275`, beside the already-refuted
  "Cyrius has no closures" at `:4279` (and again at `:3848`); `dependency-watch.md:184` was
  corrected 2026-09-09. ⚠ **Every line number in the original of this sentence was stale — and so
  were the ones that first replaced them.** A remediation instruction whose citations have drifted
  sends the next reader to the wrong line, which is worse than giving no citation at all.
  ⚠ **This entry has now been wrong twice, in opposite directions, and both times by not running
  anything.** It first read "and Cyrius has no closures" — false since v6.3.8, propagated to four
  files. It was then corrected to a *measured* block: on 6.5.16 a capturing closure SIGSEGVed when
  passed through a function and called there, which is exactly this shape. **That was fixed in
  6.5.17 and re-verified on 6.5.18** (42 both directly and across the boundary), so the closure
  block is gone too.
  What actually remains is the plain API mismatch: `vec_sort_by` invokes its comparator as
  `fncall2(cmp, elem_a, elem_b)` — element *values* — whereas hisab sorts *indices* by dereferencing
  each into a separate `points` vector. A capturing comparator can now close over `points`, so this
  is doable — re-verified on 6.6.2, a capturing comparator works across a fn boundary. ⚠ The
  deferral reason recorded here ("this is the first instance") is **false**; what remains is only
  that 2.6.15 already fixed the *complexity* of the two hot sorts, so this is consolidation for
  consistency, not for speed. **Unpark it: the gate it was waiting on has been met three times over.**

- **SIMD the flat-array kernels** — `_opt_dot`/`_opt_norm`/`_opt_axpy`, the L-BFGS sweeps,
  `_lext_dot`/`_lext_norm`. ⚠ **Correct the citation: 2.3.1 measured 1.6–6.5x (median ~2.3x), not
  "5–8x"** — and much of that win was accessor-call elimination, which does not exist here.
  ⚠ **The stated gate is not the blocker.** Benchmarks are cheap (72 working labels in the harness).
  The real blocker is that **nobody knows the `n`**: every test calls these solvers at n = 1, 2, 3,
  where a 2-wide dot cannot win, so authoring a benchmark today would reproduce the exact failure
  mode `CLAUDE.md` records twice this arc — measuring the instrument instead of the operation.
  **Needs a consumer-sourced `n` first**, and ten consumers are now live to ask.
  ⚠ Secondary: these buffers are exactly `alloc(n*8)` and `f64v_*` **over-reads on odd `n`**, so each
  kernel needs the pair+scalar-tail hybrid, not a one-line swap.

---

## 3.0.0 -- Error-handling migration (breaking)

The integer-error-code convention (`src/error.cyr`: functions return 0 / a
negative `ERR_*` code) predates the stdlib `Result<T,E>` (`lib/result.cyr`,
v5.8.28) and `?` propagation (v5.8.29). Migrating is a library-wide signature
change — breaking for consumers (impetus, kiran, joshua, …) — so it lands as
a major, with a migration guide, not a 2.x patch.

> ⛔ **RE-SCOPE BEFORE PLANNING: this section was written against a `Result` that no longer exists.**
> It cites the v5.8.28 **boxed** form. `lib/result.cyr` at the current pin is the **v6.6.0 value
> form**: `var r = f();` is a hard compile error, `payload()` is gone, and every call site becomes
> `var t, v = f()`. ⚠ And there is **no incremental path** — `?` on a plain i64 fn **compiles clean
> and SIGSEGVs** (exit 139 where 6 is correct) with no diagnostic.
>
> ⭐ **A free win is available at 2.x instead, and should be taken first**: `#must_use` exists (199
> uses in the tree) and is on **zero** of the nine heaviest fallible modules — `num_ext` 61 `HSB_ERR`
> returns / 0 `#must_use`, `linalg_precision` 48/0, `optimize` 31/0. That is most of the value of
> `Result` for none of the breakage. ⚠ Caveat: it is a *compiler* warning, not a lint warning, so
> CI's `^  warn ` grep will not gate it — wire that before relying on it.

- [ ] **[3.0.0]** Wrap fallible returns in `Result<T,E>` (keep `ERR_*` codes as the `E` payload) — **value form**
- [ ] **[3.0.0]** Adopt `?` to replace manual `-1`-return + check chains
- [x] ✅ **DONE — 44 annotations, and a GATE, because without one it was decorative.**
      ⚠ **The count is 44 functions, not 167.** 167 is the number of `return HSB_ERR` STATEMENTS;
      `#must_use` goes on functions, of which 48 can return one — and **4 of those can only ever
      return `HSB_ERR_NONE`** (`_lp_bidiagonalize`, `_lp_tridiagonalize`, `num_halton_2d`,
      `ode_dopri45`), so annotating them would train readers to ignore the annotation. 44 annotated,
      200 → 244 in the tree.
      ⛔ **THE ROW'S CAVEAT WAS RIGHT AND IS NOW MEASURED**: `#must_use` is a COMPILER diagnostic, not
      a cyrlint one. A file that discards one gives `cyrius lint` *"0 warnings"* and matches the CI
      `^  warn ` grep **zero times**, while `cyrius build` prints
      `warning: #must_use result of 'f' is discarded`. A new CI step greps for it.
      ⭐ **The gate was verified to FIRE before being trusted**: a deliberate discard installed in
      `src/ode.cyr` takes it 0 → 1, and removing it returns 0. It covers the bundle (all 35 modules
      with the stdlib resolved, so every intra-library call site) and `examples/`, and deliberately
      NOT `tests/` — calling a fallible function and asserting on its out-parameter is a legitimate
      testing idiom, and 67 sites there do exactly that.
      ⛔ **It found a real defect on its first run**: `examples/basic_math.cyr` called
      `calc_integral_simpson` and `num_newton` and read their out-parameters **without checking
      either return**, so a failed call would have printed a confident wrong number. **An example is
      copied more often than it is read.** Repaired, output unchanged.
- [ ] **[3.0.0]** Migration guide + deprecation window for the old integer-code API
- [ ] **[3.0.0]** **Public / private function surface.** hisab currently signals intent by naming convention
      alone — a leading `_` means "internal" and nothing enforces it. Two consequences already
      visible in the tree: `geo_diff.cyr` reaches `geo.cyr`'s helpers across a module boundary
      because nothing distinguishes "public API" from "implementation detail", and the 2.10.1 split
      had to be named `geo_ray_aabb_face` rather than `_core` specifically so a cross-module call
      would not be reaching for an underscore. A major is the right place to draw that line, because
      marking a function private is a **breaking change for anyone already calling it** — the same
      reason the `Result<T,E>` migration lands here.

  **Sized by measurement, not estimated: 269 `_`-prefixed functions across `src/`, of which 17
  are called from a different module**, in six pairs:

  | defined in | called from | count |
  |---|---|--:|
  | `calc` | `calc_ext` | 3 |
  | `calc` | `noise_simplex` | 2 |
  | `geo_advanced` | `collision_core` | 4 |
  | `lie` | `lie_ext` | 4 |
  | `num` | `num_ext` | 2 |
  | `symbolic` | `symbolic_ext` | 2 |

  So a blanket "`_` means private" would break six real call paths, every one of them a
  `X` → `X_ext` pair where the split is an artefact of file size rather than of API design.
  ~~Decide: the visibility marker; whether those six pairs get a shared-internal escape hatch or
  the helper is promoted to public; and whether the distlib bundle needs the distinction at all.~~

  ⭐ **ALL THREE DECISIONS ARE ALREADY ANSWERED — cycc 6.6.2 ships and ENFORCES visibility.**
  Verified on the pin: a file whose first line is a bare `private`, containing `fn _secret` and
  `pub fn public_api`, gives `error: '_secret' is private to its file` for a cross-file call to
  the former and `ok` for the latter. A per-item `private` is refused with its own diagnostic.
  The stdlib already uses it at scale (322 `pub fn` in `yukti.cyr`; `regex.cyr` is file-private).

  - **The marker**: nothing to decide. It is file-level `private` + per-fn `pub`.
  - **The escape hatch**: ⚠ **still a real decision — an earlier draft of this correction wrote
    "nothing to decide", and that was wrong.** Privacy is per FILE, so marking `X` private breaks
    every cross-module call into it, and the bundle does **not** paper over it (next bullet). Each
    of the six `X` → `X_ext` pairs forces a choice: mark the helper `pub` — promoting an
    implementation detail to public API — or merge the pair into one file, or leave `X` public.
    Three options, six sites. It is no longer a *marker* decision, but it is still a decision.
  - **The bundle**: ⛔ **this file answered it WRONG.** A real `cyrius distlib` bundle built from
    `private`-marked modules exported **only** its `pub fn`s and rejected `_a_helper` from a
    consumer. So concatenation is *not* a free pass — it preserves the distinction.
    ⚠ **Landmine**: `distlib` passes `private` through verbatim into the single 23,405-line
    bundle, and `cyrius check --with-deps dist/hisab.cyr` stays **green** on a bundle **no
    consumer can call**. The existing gate cannot see this failure.

  ⚠ **Re-scope before doing it — this is now a mechanical pass, not a decision, and it is 3x
  bigger than sized above.** Privacy is per-file, so marking one module private breaks **24
  symbols against one suite, 21 of them non-underscore**; the tests reach **52 distinct `_`
  functions across 235 sites**. Safe order: **`pub fn` everywhere first** (verified a no-op and
  non-breaking — can land in a 2.x), add a consumer-call gate, then flip `private` last.
  **The `pub fn` half does not need 3.0.0 and should not wait for it.**

---

## Parked / deferred (revisit when a driver appears)

Evaluated during earlier arcs and consciously deferred — recorded so they
aren't silently lost (full rationale in the CHANGELOG). Items that have since **shipped** are
removed from this list rather than struck through; their record lives in the CHANGELOG and the
Release History table below.
- **SIMD `cross`** (from 2.3.1) — needs lane shuffles; `f64v_shuffle`/`permute`/`blend`/`swap` are all
  undefined on 6.6.2 (probed), so still correctly parked. ⭐ Now with a number instead of an assertion:
  the best shuffle-free formulation measures **38 ns vs 25 ns scalar (+52%)**.
  ⚠ **`lerp` was never gated on shuffles at all and should be unparked** — measured on 6.6.2 with the
  existing n=2-pair + scalar-tail hybrid: **25 ns → 19-20 ns, bit-identical results, zero shuffles**.
  ⚠ The "full rationale in the CHANGELOG" this list cites **does not exist**: 0 hits for shuffle/lane.
- **`#pure` annotations** (from 2.3.4) — ~~unsafe CSE interaction with hisab's allocate-a-fresh-result
  convention~~. ⚠ **Premise refuted on 6.6.2: there is no CSE to be unsafe.** Three identical `#pure`
  calls emit `calls: 3`; two `#pure` allocating calls return distinct pointers; and the binaries are
  **byte-identical** (same sha256) with and without the annotation. `#pure`'s entire effect in cycc is
  two warnings. ⛔ **Stronger objection than the one recorded**: `alloc()` carries no `#alloc`, so
  annotating hisab's 267 allocation sites `#pure` would assert a falsehood with no compiler backstop.
  Stays parked — for the right reason now.
- **Slices (`[T]` / `slice<T>`)** (from 2.3.4) — correctly parked, and now measured rather than
  predicted: checked slice indexing is **3.8x** raw `load64`, and the "unchecked escape hatch" is
  still **3.2x** — it discards the safety *and* keeps 85% of the cost. the toolchain's `~/.cyrius/versions/<pin>/lib/simd.cyr` has **zero**
  slice-taking forms, so slices provably cannot cover the SIMD hot paths at all.
- ~~**`defer`** (from 2.3.4) — N/A under the bump/arena model.~~ ✅ **Decided in 2.3.4 and re-derived
  2026-09-09: 0 `_free`/`_destroy`/`_close`-shaped fns in `src/`, 0 `sys_open`/`sys_close`, 339
  `alloc(` with zero frees.** `defer` works fine in the stdlib (8 uses — 6 closing a file descriptor, 2 unlinking a temp file); hisab
  has no call site to attach one to. **Not a deferral — an answered question. Delete this row.**

*Retired 2026-08-03:* **stdlib `mat_new` overflow guard** (parked from 2.5.3) — shipped in
**2.6.11** via ganita 1.0.4 at the 6.5.6 pin. `mat_new_guarded` is retained as the stricter 16M
entry point, so nothing further is owed.

---

## Consumers

⛔ **~~None are live yet — all four come online later.~~ THIS IS WRONG IN BOTH DIRECTIONS, and it is
the most consequential stale claim in this file.**

**Ten repos consume `dist/hisab.cyr` today, SHA-locked**: svara, naad, goonj, dhvani, attn11, ghurni,
prani, garjan, prakash, nidhi. svara's `cyrius.lock` pins hisab commit `1bc71e3` (tag **2.11.2**) and
`svara/src/spectral.cyr:246` calls `num_fft`. Meanwhile **impetus, kiran, joshua, hisab-mimamsa and
kana have no `cyrius.cyml` on any branch** — they are Rust repos needing a *port*, not a scheduling
decision. ~~`README.md:7` is wrong the opposite way, listing the Rust repos as users.~~ ✅ Corrected 2026-09-09.

⚠ **No live consumer has built 2.11.3 or later.** All ten sit at 2.11.1/2.11.2, behind the
6.5.33 → 6.6.2 toolchain bump **and** the 536-site ganita alias migration.
`cyrius check --with-deps dist/hisab.cyr` proves the bundle compiles against **this** manifest's
ganita — not against theirs. **Highest-value action in this section: get one live consumer onto
2.11.5.**

**The struct-layout contract** — construct via the documented constructor, read via the accessors,
size arrays with `sizeof(T)`; never a hardcoded byte count, never a hand-computed offset.
⛔ **But "32 assertions … make any such change trip a gate" is FALSE.** All 32 are `sizeof(T) > 0` or
`sizeof(T) % 8 == 0`. `ColContact` going 64 → 72 bytes passes both, **identically, on both sides of
the change they were written to catch**. There is not one `assert_eq(sizeof(T), <n>)` anywhere in the
tree, and six public structs carry no assertion at all — including **`HVec3`, the type live consumers
touch most** (54 `hvec3_new` sites). The commit that declared the contract "now enforced" enforced
nothing. **A gate that cannot fail is not a gate**, and this one has been quoted as protection for
five releases.

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

**Known caveat, and first integration has already happened (see above):** `gjk_intersect_3d` costs
~+55% on the no-hit path since 2.9.0 — the price of it no longer missing 134 genuine interior
overlaps per 4,386 evaluations. The no-hit path is the broadphase-common case, so if a consumer finds
that cost unacceptable the answer is a cheaper pre-filter, not reverting the correctness fix.
⚠ **Re-derived on 6.6.2 post-ganita, same-binary ABBA: box +62%, sphere +57% — this file understates
it.** ⛔ But "a cheaper pre-filter" cannot be built where this row implies: the entry point receives
only two support-function pointers, so there is **no cheaper information inside it to filter with**,
and hisab already ships the broadphase (`src/spatial.cyr`). **This is caller-side work, not a hisab
roadmap item** — and with ten live consumers, someone can now actually be asked whether it bites.

---

## Release History

| Version | Date | Lines | Files | Highlights |
|---------|------|-------|-------|-----------|
| 2.21.0 | 2026-09-11 | 25,830 | 36 | **The CGA null basis: a conformal point's nullity error becomes ulp-level and scale-free.** 2.20.0 proved no ARITHMETIC fix existed — at x = 2^-30 the correctly-rounded `ep` IS -1/2 — so this changed the BASIS. `q` now lives in one coefficient, so the norm recovers it by subtraction instead of from a cancellation. Median `|P.P| / q`: **1.0 -> 4.48e-17 at 2^-30** and **1.0 -> 4.33e-17 at 2^+30** (1.0 = the whole value lost), FLAT rather than a U; distance recovery **median 46.4x/48.7x wrong -> 1.76e-15/1.93e-15**. ⛔ **"Exactly null" was the headline until a pre-tag audit priced it**: it needs 2.18.0's Neumaier compensation DROPPED, which makes the norm repeat the constructor's rounding — and then `1 + a*e1 + a*n0 + (a/2)*ninf` reads a scalar part of **0 where the answer is 1**, 6 of 11 magnitudes. **An exact zero from making the same rounding error twice is an artifact.** Reverted; **no assertion in 3989 could tell the two trees apart** until a witness was written. ⛔ `_cga_scalar_of_geo` had to sum ALL pairs (diagonal-only left 6800 of 6800 non-null — WORSE); only 16 of the 32 scalar-producing pairs are diagonal. ⛔ **The first CGA call cost 2.7 ms and every benchmark was blind to it** (they all warm up) — found because two probes disagreed by 2.1x; repaired to 0.72 ms, table byte-identical under the FNV contract. ⚠ **2.22x on point*point, up to 10.93x SLOWER at k = 32.** ⛔ Building it found a **cycc 6.6.2 wrong-code bug** — `continue` at two nesting levels binds to the wrong loop — an all-zero table reported as success, filed upstream with a self-proving repro; ⚠ **my first filing described the symptom wrongly** and was corrected by instrumenting the loop. ⭐ The gate came BEFORE the code: 1024 entries derived in CI, FNV-1a `0xF4A98C5706D5CF5B`. ⚠ The index space caught me: a first spot-check read `n0²=+1` when it was `e3²=+1` in the other numbering. ⭐ The 2.20.0 acceptance pins INVERTED as designed. Suites **3983 -> 3991**. |
| 2.20.0 | 2026-09-10 | 25,469 | 36 | **The four repairs 2.19.0 measured — and measuring them again showed the FILING was wrong each time.** ⛔ **Above 2^63 both symbolic renderers returned ONE wrong constant for EVERY input**: `f64_to` saturates, so `expr_to_str` and `sym_to_latex` each answered **-9223372036854775808** for 1e19, 1e20 and 2^100 alike — a NEGATIVE number for a POSITIVE input, with a DOUBLE MINUS for negatives. The filing said "30 diverging values" and framed the DELETION as the risk. ⚠ It named one function; the grep found two. Repaired exactly (**10,961 renderings vs an exact oracle, 0 mismatches**). ⛔ **A conformal point is never exactly null and that is a WRONG ANSWER** — distance recovery off by a **median 46x at 2^-30 and 49x at 2^+30** — symmetric tails, and a MEDIAN because the max diverges as the pair closes; an earlier draft's ~2000x/~32000x were maxima read as an asymmetry, the "small coordinates" reading being an artifact of a power-of-two sweep. No arithmetic fix exists (at 2^-30 the correctly-rounded ep IS -1/2); basis change filed. ⛔ **The subnormal-SVD claim was false in both halves**: **270 of 416 rows report success and only 57 are right**, worst 9.78x, one value returned as exactly ZERO — and the old sweep used only upper-triangular fixtures, where a GENERAL 2x2 is 1 correct / 86 loud / 121 silent. Repair filed, fixture family widened as its acceptance test. ⛔ **A CHANGELOG sentence had the `triangulate_polygon` direction BACKWARDS, twice, for thirteen releases** — 2.7.0 vs HEAD built side by side gives **old 9 (COMPLETE) -> new 6 (PARTIAL)** — and it lived only in the CHANGELOG where no gate could reach it. ⭐ `ad_grad` is **O(m^2)** and its "6.8x" was one point on a curve (2x at m=256, 12x at m=2048). ⛔ **Adding a benchmark changed a different benchmark by 31%**: the new `jac_rev` rows shifted the heap under a never-freeing allocator and `kdtree_radius_4k` read 484 ns with them against 696 ns without — unchecked, a **44% phantom speedup in an untouched module**. ⚠ Four of my own instruments were wrong first (a shift masked 71->7, a dyadic CGA fixture built with the subnormal form, a marker figure carried from an unrelated sweep, and an unvalidated checksum). ⛔ **AUDITED BEFORE TAGGING — five independent re-measurements found a defect this release had SHIPPED and six wrong figures in its own notes.** The two new benchmarks **overflowed their own tape** (1040 sized, 2056 pushed) and measured half the work while reporting success, and the probe behind the O(m^2) table had the same bug, so it read ~2.6x low. A hand-encoded constant was **mis-transcribed and invisible to the constant gate** (comment above the declaration, not trailing) — a confident 158/158 over a population of 159; value fixed, gate widened, verified to fire. The CGA magnitudes were MAXIMA of a diverging quantity presented as the effect; the medians are 46.4x / 48.7x and the tails are SYMMETRIC. Also corrected: the SVD factor (10.8x, not 9.3x from a rounded truth), the kdtree figures (to the CSV record), the jac first-run pair, and "zero in-tree callers" (it has five). **No performance change claimed**: median **+2.73%, 0 rows >10%**, against a same-binary control of +2.82% with **13**. |
| 2.19.0 | 2026-09-10 | 25,241 | 36 | **The 3.0.0 prep, and five gates that could not fail.** `#must_use` on the fallible surface, the 12 `HSB_ERR_*` codes as an enum, and the decisions owed — and every piece needed a gate that turned out to be unable to fail. ⛔ **`#must_use` is a COMPILER diagnostic, not a cyrlint one**: a discarded result gives `cyrius lint` "0 warnings" and matches CI's `^  warn ` grep **zero times**, so 44 annotations without a step were decorative; the gate found `examples/basic_math.cyr` discarding two fallible returns. ⚠ 167 was the count of `return HSB_ERR` STATEMENTS — it is **44 functions**, 4 candidates excluded because they can only return `HSB_ERR_NONE`. ⛔ **The suite could not see a wrong error code at all**: two mutants survived all 3940 assertions, one making `HSB_ERR_ALLOC` COLLIDE with `HSB_ERR_INVALID_TRANSFORM`, because **every assertion compares a return against a code BY NAME so the value cancels out**; 15 assertions now name the number and check all 66 pairs. ⚠ The principle motivating the enum named the wrong mechanism — `var_table` is **615 on both sides**; the saving is relocations (`fixup_table` 3092 → 2933, `code_size` −704 B). ⛔ **The release shipped its own broken provenance marker and the gate that caught it has never run**: `check-measurements.sh` is PR-only in a repo with **0 pull requests against 303 workflow runs**, and its `--diff` mode was blind to its own dominant case — a path failure filed against the PARAGRAPH's first line, intersected away, making the gate **GREENER** because the fabricated marker SATISFIED the real claim beside it. ⛔ **The version gate compared text to text and never ran the binary**; `src/main.cyr` prints `CYRIUS_PKG_VERSION` (byte-identical output) and the new step runs `build/hisab`. ⚠ The roadmap's "both still assert" was FALSE — `ci.yml` had the sentence zero times; the live claims were in two files it never named, stale for four weeks after being true for four days. ⛔ **Four of my own instruments were wrong first**: zsh reading `:t` as a path modifier inside double quotes, a stale scratchpad `cyrius.cyml`, a `$?` capturing `head`, and **both replacement guards wrong in OPPOSITE directions** — one failing on its own tree because the comment quotes the retired literal, one vacuous because it matched that same comment. ⭐ The decisions owed were measured by an 8-way fan-out with adversarial verification and **every refutation landed on SCOPE, not facts**: a per-benchmark noise band is strictly worse than the global ±10%; `dual_*` declined; four remaining items are repairs, not decisions, and are scheduled. No performance change claimed, and **the control is what settles it**: against the immediately preceding run 2.19.0 looked like 11 rows improving 10-23%, all in the mat4/quat/jet family this release never touched (**0 `HSB_ERR_` references**, so the enum cannot have altered their codegen) — but that baseline was recorded **twice from the same binary 53 seconds apart**, and the pair moves those same rows **+31.6% / +26.5% / +26.3% / +25.6%**, i.e. the same magnitudes the other way. Against the other two baselines 2.19.0 is **median +0.00%** and **median +0.43% (0 rows >10%)**, while the same-binary control is median +2.82% with **13 rows >10%**. ⚠ **A single adjacent run is not a baseline.** The only runtime change is EPA polish 64 → 128 (`mpr_penetration` **1.65e-05 → 6.3e-16**). |
| 2.18.0 | 2026-09-10 | 25,127 | 36 | **The SVD factors — an item carried three releases as "no known fix", closed in one line, and the line was 2.15.0's own.** `_lp_bidiagonalize` APPLIES a left Householder reflector when `vtv >= F64_TINY` and REPLAYED it into `U` only when `vtv > EPSILON_F64`; every reflector in between went into `B` and not `U`, so `S` was exact, `U` and `Vt` were each perfectly orthogonal, and `U*S*Vt` was not `A` — the small block came back with the WRONG SIGN, residual exactly twice the block, **288 of 999 ratios**. **A guard repaired on one side of a pair is a new defect, not half a repair.** ⛔ **The release opened by finding one of 2.17.0's own headline figures wrong**: "472 of 999 -> 0" came from a probe that shared its assertion's NaN hole (`0/0` is NaN; every f64 comparison with NaN returns 0), so both agreed while both were blind. Block ratios answered correctly **256 -> 268 -> 999**, silent wrong 474 -> 0, loud failures 731 -> 0. ⭐ **A repair that moves no number is pointing at a second defect**: repairing `_lp_tridiag_qr`'s Givens radius changed nothing (463 of 998 before and after) because the loss was the Wilkinson shift forming `e2*e2`, and an unshifted step on a symmetric block with equal diagonals is a FIXED POINT — a line the census had filed SAFE_UNREACHABLE, right about the sqrt and wrong about the function. The SVD's own shift formed DEGREE-FOUR quantities, floor at the fourth root of the subnormal floor (2^-268.5, measured to the binade). `_lp_pow2_floor` now balances subnormals, so `eigen_qr` is exact to **2^-1070**. ⛔ **And I filed a wrong causal story of my own**, caught by measuring which half did the work: the deflation repair was filed as "the product flushes, so divide", and the PRODUCT form with the new zero arm gives the identical 28 of 72 — the zero arm was the whole repair and the division was reverted rather than shipped unproven. ⛔ **The EPA seed trade, carried since 2.9.3, is DECLINED**: its price was never the cost but the ACCURACY, and certifying is an early-out that skips `_epa_polish` — the "upgrade" is **36,000x** less accurate against the exact sphere formula. ⭐ `cga_norm_sq` got Neumaier compensation (not Kahan, whose correction is lost below the addend's ulp): translator failure **t = 2^28 -> 2^538**. ⚠ **A measured cost, stated rather than hidden behind its spread**: svd +6.8%, eigen +6.5% across two runs, both inside their ~41% history; untouched rows +0.30% median. 3937 |
| 2.17.0 | 2026-09-10 | 24,969 | 36 | **The norm tier: the row said two sites, the grep found 51.** Every `sqrt(x² + y² + …)` in the tree squares before summing, so a component below **2^-537** flushes to zero, one above **2^511** overflows, and a **subnormal sum** returns a finite, plausible, wrong answer. A 10-agent census with 60 adversarial verifiers (reachability / dimension / arithmetic) confirmed 19 more defects in 9 modules and refuted one; **39 sites repaired, 73 mutants, 67 killed and 6 recorded with reasons.** ⛔ **In a solver the class does not return a wrong number — it returns a CONFIDENT SUCCESS AT THE STARTING POINT.** Every convergence test is `norm < tol`, so a flushed norm fires on iteration one and the routine hands back `HSB_ERR_NONE` with `out_x` still equal to `x0`: **446 of 1001** objective scales for conjugate gradient, **461 of 1001** residual scales for Levenberg-Marquardt, and `cmat_inverse` calling **498 of 1010** perfectly invertible matrices singular. `svd_golub_kahan` turned **474 of 999** silent wrong answers into loud ones (⚠ **corrected 2026-09-10** — the original "472 → 0" was measured by a probe that shared the assertion's NaN hole; the routine now FAILS LOUDLY there rather than answering, and the correct range moved only 2^-257 → 2^-269). ⛔ **Three of the repairs describe defects that 2.10.2, 2.11.1 and 2.14.0 each record as ALREADY FIXED** — `cga_rotor` still returned the identity rotor 991 times in 2041, `hquat_inverse` 507 times, `geo_triangle_unit_normal` the fabricated (0,1,0) for 877 of 1401 leg scales. **Moving a threshold does not help when the quantity it tests has already lost the value.** ⭐ **A repair that moves no number is pointing at a second defect**: repairing `_lp_tridiag_qr`'s Givens radius changed nothing — 463 of 998 before and after — because the loss was the Wilkinson shift forming `e2*e2`, and an unshifted step on a symmetric block with equal diagonals is a **fixed point**; the census had filed that line SAFE_UNREACHABLE, right about the sqrt and wrong about the function. ⛔ **Two sites were missed by this release's own first pass** and found by a regression sweep asking what the repair missed — `hquat_inverse` reads `hquat_length_sq` directly, and a comment 2.17.0 itself wrote saying it *routes through* the repaired norm was false. **A comment asserting a call graph is a claim like any other.** ⛔ **The suite was blind for a structural reason**: `hvec3_length`'s floor was pinned by two assertions that **asserted the defect as a requirement**, and the 2.11.1 quaternion sweep bracketed every candidate *threshold* while stopping 142 decades above the *arithmetic* floor. ⚠ **Six of my own fixtures were wrong before the code was** — an axis-aligned segment where the fabricated fallback IS the right answer, a repaired path added while the old guard still fired first (**the order of the guard is the guard**), a **vacuous** assertion at 1e-12 where the mutant is 2 ulp off, an objective-scaled optimiser fixture measuring its own step collapse, a full-rank SVD sweep that cannot enter `_lp_split_zero_diag` at all, and six of thirteen solver mutants surviving the end-to-end sweeps because **a consequence test cannot discriminate a band the consequence smooths over**. **No performance claim**: the box was quieted mid-session (an orphaned `python3` on a full core for 12.5 h), moving all 72 rows −6.8% median — and the control settles it, untouched rows −6.83% against touched −6.49%. 3920 |
| 2.16.0 | 2026-09-10 | 24,266 | 36 | **The small-angle series, and the defect the guard was hiding.** 2.15.0 deferred four maps because lowering their guard alone makes them worse: `se3_exp`/`se3_log` divide by θ² and θ³, and at θ ≤ 2.2e-162 both `θ*θ` and `1 − cos θ` are exactly 0, so DBL_MIN turns the coefficient into `0/0 = NaN`. ⛔ **The log maps had a far larger defect the guard was hiding**: both recovered the angle with `acos` of a value that rounds to **exactly 1.0** below θ ≈ 1.5e-8, so the entire rotation was gone — measured, exactly 0 from 2^-28 down. `atan2` recovers it bit-exactly and also represents θ > π, which `acos` never could. ⭐ **Round-trip floor 2^-26 → 2^-537, 511 decades.** ⭐ The coefficients split three ways: `c1` needs no series at all (the half-angle identity makes it `(sin h / h)²/2`), `c2` and `se3_log`'s `c` become Taylor below θ = 0.1 — a **measured** crossover, not a chosen one. ⛔ **A mutant that would not die found a third defect**: both log maps took their norms as a naive sum of squares. 3807 |
| 2.15.0 | 2026-09-10 | 24,172 | 36 | **The epsilon tier closed: 73 of 74, 81 mutants, 73 killed.** ⭐ **The class held but the REPAIR is not one rule**, and the distinctions are the release: an unbounded numerator needs DBL_MIN while a bounded one needs an exact zero; a convergence test divides nothing so representability is the wrong question; sparsity is structural; a comparator with a tolerance is not an order, because epsilon-equality is not transitive. ⛔ **`cx_div` proves no threshold can be right**: over 401 decades 1e-12 fails 241, the shipped 1e-24 fails 235, DBL_MIN fails 93, and REMOVING THE GUARD ENTIRELY still fails 89 — Smith's fails 0. **Judging a repair against 89 rather than 0 would have licensed leaving 89 decades broken.** ⛔ **Six of my own fixtures were wrong before the code was**, every one caught by measuring. ⭐ **The census warned me off an entire class of fixture and was right**: a uniform units change is VACUOUS for `svd_golub_kahan`, which balances via `pow2_floor`. 3789 |
| 2.14.0 | 2026-09-09 | 23,738 | 36 | **The epsilon release: the row said ~20 sites, the census found 97.** Every `EPSILON_F64` comparison guard in `src/` classified and then independently re-derived by a second agent told to refute the first — **136 guards, 97 confirmed defects across 23 modules**. 23 repaired, 58 mutants all killed, **74 enumerated and left open with evidence**. ⛔ **2.10.2's own repair was incomplete and its proof could not have shown otherwise**: `geo_triangle_unit_normal` guarded a degree-FOUR quantity with a degree-ONE threshold and still fabricated (0,1,0) from legs of 1e-77 down, while the 10-decade sweep that certified it bottomed out at 1e-9. **A sweep can only show the OLD threshold was wrong; showing the new one is right needs a depth derived from the arithmetic.** ⭐ **Three operations need three DIFFERENT exact guards** and confusing them is measurable. ⛔ **My own mutation harness was wrong before the code was, for the fourth release running** — it read a compiler *note* as a result and printed "killed" for all eleven; four had SURVIVED. 3657 |
| 2.13.0 | 2026-09-09 | 23,528 | 36 | **The suite release: the suite could not see an error of 0.9.** The audit's self-declared highest-value item, justified by measurement rather than prose — on the 2.12.0 tree `hvec2_add` could return `a + b + 0.9`, a core public function wrong by nearly a whole unit, and **all 3572 assertions still passed, zero failures across five suites**. `assert_eq(f64_to(v), 3)` accepts every v in [3,4) because `f64_to` TRUNCATES; **842 of 3105 sites (27.1%)** compared floats that way and now **16 (0.5%)** do, every survivor a deliberate rounding or scaling test. `foundation.tcyr` went **89% -> 0%**. ⭐ **The migration verified itself** — a bit-exact compare FAILS at any site whose value was not already exact, so a mechanical rewrite became a search: 809 converted and **all but four passed at once**, meaning the truncation had been pure slack. ⛔ **Those four were four different defects, each passing only because of truncation**: `srgb_to_linear(-1)` asserted as 0, actually **-1/12.92**; `sh_evaluate_l2` on a zero direction asserted as 0, actually **-0.0333**, and its message was wrong too — a zero direction leaves **two** bands nonzero (Y00 and Y20), not one; the PGS solver checked against 0.090/0.636 where the line above it says **1/11 and 7/11**; and an antiparallel cross product returning **negative zero**, which `f64_to` mapped to the same integer as +0. A fifth truncation was deliberate and is now pinned on both sides. Named tolerance helpers added to all five suites — `abuse.tcyr` had no named constant at all, spelling its bound as a raw hex literal at the call site. **Measured strengthening**: `m4_get + 1e-6` 29 -> **84** failing, `hvec3_cross.x + 1e-6` 76 -> **88**. ⚠ Two of those numbers were nearly reported wrong — the first harness's mutation silently failed to apply and returned `0 failing`, which reads exactly like "the suite is blind"; it now refuses to report unless it can grep a marker proving the mutant is installed. **Prove the mutant is installed before believing the count.** 3574 |
| 2.11.5 | 2026-09-09 | 23,319 | 36 | **cycc 6.6.2: the wrong-code bug is fixed, and the filing was wrong about its scope.** Toolchain 6.6.1 -> 6.6.2, no hisab behaviour change, suite 3538/3538, lock 30 -> 31 (new `lib/boxed.cyr` via `tagged`). hisab's filed reproducer now exits **0** on 6.6.2 and **139** on 6.6.1, and the three suites that used to SIGSEGV pass with `m4_mul_vec4`'s hoist REMOVED — so the workaround is retained for consistency with `m3_mul_vec3` only, and its comment no longer claims to be load-bearing. ⛔ **The filing was wrong twice**: it was filed as derive-specific and as a 6.5.71 regression and was NEITHER — all **21** SIMD handlers took `var vbase = GFLC(S)` without raising GFLC until every argument was parsed, so any argument allocating a frame local bound it over the destination; reachable via `callptr` since 6.0.70 and `#inline` since 6.5.63. **A first-bad-version is evidence about VISIBILITY, not ORIGIN** — and the filing's own root-cause section said so while its header contradicted it. Severity was understated too: with `CYRIUS_REGALLOC_PICKER_CAP=0` it exits 0 and writes into the ARGUMENT OBJECT. ⚠ **A second fix retires a landmine this repo documented since 2.3.1 with the WRONG CAUSE attached** — a bare SIMD intrinsic at top level compiled clean and SIGSEGV'd on every release, not from SSE stack misalignment as `vec4.cyr` claimed but because the handlers stash operands in FRAME slots and top-level code has no frame; 6.6.2 refuses at compile time, verified 6.6.1 OK-then-139 against 6.6.2 compile-error. ⚠ 6.6.2 also shows the one class a consumer build cannot catch — 6.6.0 **silently redefined `tag()`/`is_tag()` at unchanged arity** — checked rather than assumed: hisab calls neither. **A name whose meaning changed must be RETIRED, not redefined.** **No performance change claimed**: two rows read as regressions (+22.85%, +14.01%) and were single-run outliers — the FIRST 6.6.2 run matched 6.6.1 within 1% — settled by comparing each benchmark against ITS OWN spread rather than a global band that one 50x-noisier row had widened. 3538 |
| 2.11.4 | 2026-09-09 | 23,275 | 36 | **Off the deprecated ganita aliases, and the 28% that was hiding behind them.** ganita 1.2.4 marks the bare `mat_*`/`f64_*` spellings deprecated (migration window only); all hisab call sites now use the `ganita_*` names, `dist/hisab.cyr` included. ⚠ **2.11.3 estimated this at 8 call sites. It is 536** — the estimate was scoped from the names ganita's changelog paragraph happened to mention rather than from its surface, where the deprecation block is a **53-entry table**; hisab was on 20 of them, `mat_set` alone 265. **Scope a migration from the dependency's surface, never from its release notes.** The 5 suites produce **byte-identical output** across the change — captured and diffed, not re-counted — which is the right oracle because the aliases are literal one-line forwarders. ⭐ **They were not free**: cycc's `#inline` requires <= 2 parameters and `ganita_mat_get`/`_set` take 3 and 4, so all 68 matrix accesses in `linalg_precision.cyr`'s SVD and eigen inner loops paid a `call`/`ret` for nothing — `svd_golub_kahan_12` **-27.85%**, `eigen_qr_12` **-27.67%**, against a same-binary noise floor measured in the same session (median **2.10%**, worst **9.13%**), i.e. ~3x the worst noise and landing on exactly the two benchmarks the mechanism predicts; the other 21 trustworthy rows drifted +0.07..+6.99% inside that band with zero matrix calls between them, so **no other movement is claimed**. ⚠ **Two assertions had gone VACUOUS**: 2.11.3 fixed the two tests that FAILED when upstream repaired what they had pinned, but these compare through a tolerance and a round, so they kept passing while the property evaporated — the stdlib `(-2)^3` truncates to -8 now, so a pair labelled "truncation-discriminating" discriminates nothing. **An assertion written against a defect cannot fail when the defect is repaired; it just stops testing anything.** ⚠ And **`_ad_pow`'s reason to exist collapsed a second time** — domain died in 2.11.2, precision here (665/665 comparisons now bit-identical) — leaving a third ground that is the OPPOSITE of the second: squaring amplifies relative error, so `(-0.999)^1000` is **4 ulp** from truth through repeated multiplication and **58** through binary exponentiation, inverting past ganita's +-1024 window. Filed rather than decided, recording that no test exercises the regime it still wins. 21 present-tense API comments corrected; ~45 historical mentions deliberately left, with the convention stated once in `src/f64_util.cyr`. 3532 |
| 2.11.3 | 2026-09-09 | 23,245 | 36 | **The toolchain catch-up that hit a wrong-code regression.** cyrius **6.5.33 -> 6.6.1** (forty-odd releases, crossing a minor), sakshi 2.4.11 -> **2.5.1**, ganita 1.1.4 -> **1.2.4**. ⛔ **The new toolchain miscompiles hisab, and the bug is upstream's, not this repo's**: three of five suites SIGSEGV'd on the first run. A `#derive(accessors)` getter passed directly as an `f64v_*` intrinsic argument makes the intrinsic read its **destination pointer from a stack slot nothing ever writes** — `movupd %xmm0,(%rdx,%rsi,8)` with `%rdx = 12`, loaded from `-0x50(%rbp)`, a slot read once and written zero times, while the src and scalar slots ARE written. **Bisected: 6.5.70 clean, 6.5.71 broken** — the release that put derive accessors on the inline-replay path (`callq` 7 -> 3), removing the call that had incidentally forced the spill, so **the intrinsic bug is older than 6.5.71 and that release only stopped hiding it**. Three functions of identical shape in ONE file discriminate it: a plain fn accessor is fine, a raw `load64` is fine, only the derived getter fails — so it is not 'inlining' in general. Fixed by hoisting the accessors into locals, **which `m3_mul_vec3` has always done**; mat4 was the lone inconsistency. Mutation-proven both ways, restore confirmed by grep. ⚠ **Wrong-code, not merely a crash** — it faults only because the garbage slot happens to be unmapped. Filed upstream in the cyrius repo. **(2) Two tests updated because upstream FIXED the defects they had pinned**: ganita 1.2.4 replaced `exp(n*ln\|base\|)` with binary exponentiation for integral exponents (upstream saw `pow(7,2) = 48.99999999999999296`), so `f64_pow(-2,4)` went from **15** to a bit-exact **16** and `_ad_pow`'s precision rationale is void — the two now agree bit for bit; and 6.6.1 gave `math.cyr`'s exp polyfill its missing infinity guard, so `cx_exp(-inf)` went from NaN to exactly **+0**, which is correct. ⚠ **The replacement had to be made discriminating**, because 0 is also what the fabricated-guard class produces — `exp(+inf)` is now asserted beside it. **(3) The manifest's ceiling comment named a limit that does not gate this path**: expanded source **8 MB -> 24 MB**, tokens **1,048,576 -> 4,194,304**, and the retired '16 MB input_buf' described `_SRC_CAP` instead. Measured as a PAIR — a 9,002,640 B source is rejected by 6.5.33 and clean on 6.6.1 — because a one-sided 'it compiled' proves only that the cap exceeds one file. ⭐ **And the token cap bit first.** **Performance**: geometry/collision up sharply from the same accessor inlining — bvh_query_ray_200x4k **-46.5%**, bvh_degenerate_4k -30.4%, gjk_epa_sphere_box -29.6%, grad_fwd_16 -24.4% — claimed ONLY from the 23 benchmarks whose net is >= 10x the timer floor, with the flat numeric kernels (svd +0.1%, eigen_qr +0.4%) as the control proving the instrument did not shift. ⚠ **41 of 72 benchmarks report a net BELOW the floor they subtract** (worst 0.01x), so this release's `ray_aabb` -55.9% is evidence of nothing; filed, not fixed. ⚠ **The enum Critical the launcher warns about was checked, not assumed**: 0 of 936 enum constants tree-wide reach 2^62. 3532 |
| 2.11.2 | 2026-08-21 | 23,225 | 36 | **The toolchain catch-up, and the three stale constants it found.** cyrius **6.5.18 → 6.5.33** (fifteen releases), sakshi 2.4.10 → 2.4.11, ganita 1.0.4 → 1.1.4. No feature work. **Every finding is a number written down once and then trusted** — the same shape as 2.11.1's ALLOC_MAX, three times over, and **none was found by a failing test**: all 3514 assertions passed on BOTH sides of the bump. ⚠ **The tree arrived mid-sync and the missing half was the half that mattered**: `lib/` had been vendored from ≈6.5.19, so alloc/assert/atomic/bench were current while **ganita (1.0.4, −202 lines)**, fmt and three syscalls variants were behind — old-pin-vs-new-pin shows a tidy diff and misses it; only comparing against **the pin's own snapshot** finds it, the third time ganita specifically has been caught this way. A brace-aware, comment-stripping extractor puts the whole stdlib delta at **four functions** (a first pass keyed on `fn` alone said seven — six were the trailing comment block attributed to the preceding function; **the instrument was wrong before the measurement was**, three times today). **(1) `f64_pow`'s domain moved under us**: ganita 1.1.4 fixed negative-base-integral-exponent upstream, and `symbolic.cyr`'s `expr_eval` sits directly on it — `(-2)^3` returned **NaN for hisab's entire history** and returns a number now; `f64_pow(0,0)` was NaN, is 1. 12 assertions pin the new domain, and **discrimination was measured, not asserted**: re-run against a checkout still on 1.0.4, **10 of the 12 fail** and the 2 that pass are the 2 controls. `_ad_pow` survives on **precision** rather than domain — the upstream fix takes its magnitude from exp(n·ln\|base\|), so `(-2)^4` reads **15** through the stdlib and **16** through hisab, because `f64_to` truncates. **(2) The benchmark instrument changed**: 6.5.19 measures the clock floor and subtracts it, and `bench_run` sizes its own batches — **44 of 72 rows moved >10% and not one is a speedup** (`ease_in_out` 1,407 ns → **7 ns**, 100% instrument). ⚠ And the old numbers **FLATTENED** them: four operations spanning **149x** in reality were reported within **1.26x**, so a regression had room to hide — 2.10.0 moved 17 benchmarks off that floor and **these four were still on it**. `bench-history.csv` gains **`regime`/`floor_ns`** and the trend filter enforces them: `stat` alone would have admitted all 44. **Third instance of a measurement-method change moving every row** (2.9.2, 2.10.0, now), and the first two were answered with prose the trend table could not read — so per CLAUDE.md's third-instance rule it is now a column, and it is **derived** from whether the harness printed its own floor, not declared. **(3) The bundle ceiling does not exist**: 6.5.22 raised `_SRC_CAP` **1 MB → 16 MB** (it had been refusing sigil, mabda and drishti outright), so the bundle is **5.4%** of input_buf, not 77% — verified by compiling a 1,162,472 B source, 111% of the retired cap. ⭐ **And a CI annotation was handing developers a command that destroyed the file**: `cyrius fmt $f > tmp && mv tmp $f` was correct until 6.5.28 made fmt rewrite in place and print nothing — reproduced, src/vec2.cyr **2,233 B → 0 B, rc=0** — and it only ever printed for an already-drifting file, of which the same 6.5.28 fmt fix produced **38 at once**. Also: 38 files reformatted (proven leading-indentation-only, no line-count change), and three `syscall` write lengths in `examples/basic_math.cyr` over-read by one byte each, **printing a NUL into the shipped example's output**. 3526 |
| 2.11.1 | 2026-08-11 | 23,182 | 36 | **The audit release: one rule, and the four sites that broke it.** A full P(-1) sweep of the 2.11.0 tree — 6 dimensions, 115 checks, **52 findings reproduced**, 21 CONFIRMED by an independent skeptic, 2 REFUTED, 1 already known, and **28 never verified because the harness capped the verify phase at 24** (recorded as the audit's own biggest process defect, not buried: 2 of the 24 that WERE checked came back refuted, so an unverified finding is ~1-in-12 wrong and none may be acted on until re-run). **24 of the 52 are the same defect** — a guard comparing a quantity against a threshold that is wrong for it, then FABRICATING a plausible answer: `eigen_qr` wrong eigenvalues with rc = 0, `cqr_decompose` an R that is not upper triangular with rc = 0, `hvec3_angle` 0 rad ('parallel') for exactly perpendicular vectors, `m3/m4_inverse` the identity. This is the ninth through thirty-second instance of a class first written down in `complex.cyr:58` in 2.6.14 — and `cx_div` is IN THAT COMMENT and still fabricating zero. **Writing a lesson beside the code that taught it does not fix the code and does not reach the other thirty-four modules; only a grep does.** Four repairs executed: `hquat_inverse`/`hquat_normalize` compared a SQUARED magnitude against the unsquared EPSILON_F64, returning the identity for any \|q\| < 1e-6 where the inverse is finite and exact; three caller-sized allocations in `num_ext` stored through `alloc`'s 0 return (**reproduced as SIGSEGV**, now HSB_ERR_ALLOC); `ad_tape_new` handed back a non-zero handle with a null body — 2.11.0's own code; and `optimize.cyr`'s ALLOC_MAX-derived ceiling was re-derived 5792 → 16384. ⚠ **A stale comment nearly buried a real defect**: the first `num_tridiag_solve` probe was sized from `optimize.cyr`'s own `ALLOC_MAX = 256 MiB`, landed EXACTLY on the limit, allocated successfully, and the finding looked refuted — cyrius 6.4.51 raised it to 2 GiB. **A constant derived from a dependency is a measurement.** ⚠ The quaternion sweep's first 12 decades killed only 1 of 3 mutants (both rejected thresholds sit BELOW where it looked; 20 decades kills all three), and asserting unit length would not have discriminated at all — the fabricated identity IS unit, so the assertion checks DIRECTION. The audit's third critical is the suite itself: **836 of the 3510 assertions its own scan counted (23.8%) compare through `f64_to`, which TRUNCATES**, and 38.4% of value-changing mutants survive all five suites — a measured reason why 3507 assertions and 99% coverage saw none of the 52. 3514 |
| 2.11.0 | 2026-08-11 | 23,104 | 36 | **Reverse-mode autodiff, and the five forward-mode defects it found first.** Tape-based: one node per op recording both local partials and both input indices, and because inputs always have strictly smaller indices a single descending loop is a valid reverse order — no sort, no visited set. **grad_fwd_16 63.7 us -> grad_rev_16 5.70 us, 11.2x** for a 16-input gradient (not the theoretical 16x: reverse pays to RECORD the tape, and part of the rest is dual_* heap-allocating under an allocator that never frees). The pairing with optimize.cyr needed NO API change — a capturing closure holding the tape matches fncall2(grad, x, out), re-verified on 6.5.18 because that shape SIGSEGVed on 6.5.16. **Reverse mode is validated AGAINST forward mode, so autodiff.cyr was swept before a line of tape code existed**, and five defects came out: 1/1e-13 returned (0,0) where the truth is 1e13; ln(-5) returned a NaN value beside a CONFIDENT -0.2 derivative; sqrt(-4) was (NaN,NaN) because the guard ran after the sqrt; d/dx x^3 at -2 was NaN because f64_pow is exp(n*ln(base)) and rejects negative bases — the stdlib's STATED implementation, so the defect was hisab inheriting it undocumented. ⚠ **The finite-difference sweep alone found NONE of them**: at every input where a guard fires the perturbed scalar is NaN too, so the sample is skipped and the guard never asked. A guard has to be interrogated DIRECTLY. 14 mutants, 14 kills — two survived the first pass (the fixture's variables happened to BE nodes 0 and 1, so the ids indirection was untested) and one failure was the ASSERTION's fault, demanding 1e-8 where gradient descent only promises \|\|g\|\| < 1e-6. The solvers' own contract check came back CLEAN, the first time in five releases. 3507 |
| 2.10.2 | 2026-08-11 | 22,707 | 36 | **The primal defects the jets were sitting on.** All three 2.10.1 filed, plus the OBB rotation partial it deferred. **One rule settled four thresholds** — guard exactly what makes the DIVISION fail and nothing more (`_GEO_F64_TINY` = DBL_MIN); a threshold that needs a scale chosen for it IS the defect. The slab parallel test was scale-free: a box spanning [0,1] with a **unit** direction (9e-13, 0, 1) returned a hit at **x = 1.4**, and the same ray from inside a tall box gave an exit **18x too large**; `geo_ray_new` normalizes and did not protect. `geo_ray_capsule` returned a point a **full radius inside the solid** (t = 4 where the exit is 6, hit point ON THE AXIS) because `geo_ray_sphere` hands back only the FIRST root, so a cap root rejected by the half-space test meant the other was never considered — 32 of 600 interior origins also lost their exit entirely. Fixed by splitting the quadratic onto `_geo_sphere_roots` (both roots, one copy) and deleting the `cyl_missed` fallback outright. `geo_triangle_unit_normal` returned a **fabricated** (0,1,0) for a well-formed right triangle with 1e-4 legs. `dt/d(rotation) = [a_k x (c - p)]/f_k`, derived by perturbing WITHIN the rotation group rather than freely, verified over 36 FD comparisons at worst 1.9e-10 — a world-frame ANGULAR gradient, so a step along it is a valid rotation by construction. ⚠ Two repairs cost far more before being measured (an is_inf HELPER +5.6% on a 90 ns routine; materialising a hit point per cap root +92%), two fixtures were caught by COUNTERS rather than by failing, and a toolchain defect was nearly filed that does not exist — `var buf[N]` needs `&buf`. 13 mutants, 13 killed. 3469 |
| 2.10.1 | 2026-08-10 | 22,529 | 36 | **The branchy primitives, and two more defects the design check found first.** Jets for aabb, obb and capsule — all six `geo_ray_*` are now differentiable. Each primal split onto a face/branch-reporting variant with the plain entry point a one-line wrapper, so the value path is unchanged BY CONSTRUCTION: the `f64_max`/`f64_min` calls are byte-for-byte what they were and every added line sits behind `if (out != 0)`. aabb 3.1x its primal for 12 partials, obb 1.7x for 12, capsule 1.38x for 13 — the branchy three are CHEAPER relative to their primals than the smooth three, because the primal work they reuse is larger. **Before any feature code**, asking "what does the primal return when there is no face?" found `geo_ray_aabb` and `geo_ray_obb` returning **+Inf** for a degenerate direction (4 of 6 siblings honoured the contract; the same 4-vs-2 shape as 2.10.0) and `geo_ray_sphere`/`_capsule` comparing a **squared** length against the unsquared `EPSILON_F64` — the sphere reporting a MISS for any \|d\| < 1e-6 where t = 4e7 is exact, the capsule silently returning a CAP hit instead of the cylinder hit, wrong by 0.942%. ⚠ **The sphere's guard was introduced by 2.10.0's own repair**, and 2.10.0's homogeneity sweep could not have caught it: its scales are 2, 0.5 and 7, none within five orders of magnitude of the threshold the same commit added. A THIRD came from an independent derivation commissioned against the finished code — the tie flag saw edges and corners but not a zero-width slab or a tangential clip, so a flat box returned the whole gradient in the WRONG SLOT with tie = 0 on half of all configurations; the derivation confirmed all 37 partials (0 of 24,237 FD comparisons failing) and its value was entirely in the ENUMERATION. Three defects in the PRIMAL are filed OPEN for 2.10.2, including a scale-free slab test that returns a hit at x = 1.4 for a box spanning [0,1] with a UNIT direction. The capsule seam is proven C1 by a CONVERGENCE-RATE assertion after a single-offset one failed on a fixture fault. 25 mutants written, 24 killed — the survivor proved a branch update was dead code (cyl_t1 <= cyl_t2 always; 932 randomised quadratics, 0 violations). jet_obb 1.083 us -> 872 ns (-19.5%). Toolchain 6.5.17 -> 6.5.18, zero stdlib delta. 3453 |
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
✅ **Verified 2026-09-09**: 0 hits for `abaco` in `cyrius.cyml`, `cyrius.lock`, `dist/hisab.deps`,
`dist/hisab.cyr` and `src/`. The only git dep is sakshi 2.5.1, and `deps --verify` already fails any
unreviewed dep — **no new gate is owed here.** ⚠ The apparent contradiction between `eval("sin(pi/4)")`
being abaco's while hisab exposes `expr_eval` is not one: `src/symbolic.cyr:254` takes a **tree**, not
a string. hisab has no tokenizer at all.

⚠ **This table is frozen at the 2.2.0 surface.** It has no row for `expr_eval` — whose **domain
changed** in 2.11.2 (`(-2)^3` returned NaN for hisab's entire history and returns a number now), and
whose named consumer is abaco — and none for autodiff (forward duals + the reverse tape), the six
`geo_diff` jets, CGA, or Lie. A boundary table that lags the surface it describes is how a consumer
learns the boundary from a compile error instead.
