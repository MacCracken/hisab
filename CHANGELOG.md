# Changelog

## [Unreleased]

## [2.8.2] - 2026-08-04 — delaunay completeness via symbolic ghost vertices. **Two critical repairs rejected.**

One of the four criticals fixed; two were implemented, adversarially verified, and **rejected for
regressing other input classes**. Suite 1801 → **1818**.

### Fixed — delaunay_2d is complete

The audit proved no super-triangle multiplier wins both classes: 10x leaves holes on 15–37% of
uniform-random input, and raising it breaks tight clusters. The multiplier *is* the coupling to the
non-adaptive in-circle predicate.

**So the super-triangle is gone.** Vertices n, n+1, n+2 are now **symbolic ghosts** — the limit of
`A + M·u_k` as `M → ∞`, with `u = (1,2), (-2,1), (1,-2)` and apex `points[0]`. Nothing is stored for
a ghost. Every predicate touching one is decided by the sign of the leading nonzero coefficient of
its determinant expanded as a polynomial in `M`, i.e. its sign for all sufficiently large finite
`M`. A build evaluates finitely many predicates, so a single `M` satisfies all of them at once —
the predicate set is realisable by an actual configuration, which is what keeps the walk and the
flood fill consistent.

The equal-norm requirement on the direction vectors is load-bearing and was found by measurement,
not by reasoning: with `|u_k|²` all equal to 5 the two-ghost tie coefficient collapses to a plain
distance comparison against the apex. The obvious set `(-1,-1), (1,-1), (0,1)` was tried first and
its tie rule **failed 1,429 of 35,968 exact trials**.

Verified against exact rational arithmetic — every coordinate rescaled from its bit pattern to exact
integers, with a completeness certificate (soundness + exact hull-area partition + manifoldness +
Euler count). Over 769 point sets in 22 classes: **534/769 → 729/769 fully correct, 0 regressions,
195 improvements.**

| class | before | after |
|---|---|---|
| uniform-unit n=160 | 11/40 | **40/40** |
| uniform-unit n=500 | 0/5 | **5/5** |
| cluster span 1e-14 | 24/40 | **40/40** |
| cluster span 1e-20 | 0/40 | **40/40** |
| thin strip, aspect 1e6 | 0/40 | **40/40** |

The last two rows matter beyond this finding: the 1e-20 cluster and the aspect-1e6 strip were
*separately filed* defects (746 unsound triangles and 173 dropped points; 2,701 triangles short and
1,478 dropped points). Removing the super-triangle fixed them as a side effect, because they were
the same coupling seen from the other end.

Also verified independently of the implementation: **57,600 exact-rational comparisons** of every
ghost rule against the true determinant at M = 1e5 … 1e34, including all four degenerate branches
deliberately constructed. Zero failures. `delaunay_2d_400` 3.56 → 3.45 ms.

**This supersedes the adaptive-predicate work item.** There is no multiplier left to unblock.

### Rejected — two critical repairs that regressed

Both were reported FIXED by their implementer, both removed the defect they targeted, and both were
**rejected on independent verification**. Recorded because the rejections are the useful part.

- **`gjk_epa_3d` O(k²).** The repair works: face evaluations 8,576 → 812 at the shipped budget,
  4.36 ms → 0.70 ms, arena 1.23 MB → 99.6 KB, and all three metrics go quadratic → linear. But its
  flood-fill visible-set search **under-reports when rounding splits the visible region into
  disconnected components** — 239 of 47,134 cases — and every divergence coincides with one. A
  deterministic reproducer (cylinder vs rotated box) returns a depth **40.7× too small** where the
  shipped code is correct. The verifier also found the CHANGELOG, roadmap and source comment it
  shipped all claimed "0 regressions", which its own 104,000-configuration sweep contradicts.
- **`mpr_penetration`.** Rejected on the same grounds — removes the defect, regresses elsewhere.

Both are back on the roadmap with the verifiers' specific remedies, not as blank retries.

### What the rejections say about the method

Three of the last four performance or correctness rewrites in this repo were reported working and
were not. The pattern is consistent: an optimisation that replaces a full scan with a smarter search
inherits an invariant the full scan never needed, and the new invariant fails on degenerate input.
The full scan is slow precisely because it assumes nothing.

## [2.8.1] - 2026-08-04 — full audit of the v2.8.0 tree. **42 findings, 4 critical.**

An audit release: no library code changes. Six dimensions — memory-safety, numerical,
api-consistency, performance, test-quality, and a regression check that the 2.6.12–2.8.0 repairs
still hold — each swept by an independent agent and then **adversarially verified** by a second
that reproduced every finding with its own harness before it entered the report.

**42 confirmed, 0 refuted** (4 critical, 17 high, 16 medium, 5 low). Full report at
[`docs/audit/2026-08-04-v2.8.0-full.md`](docs/audit/2026-08-04-v2.8.0-full.md); the work is
scheduled across 2.8.2–2.8.5 in the roadmap.

The zero-refutation rate is not a compliment to the finders. Verifiers were required to reproduce
by **execution**, so speculative findings died inside the verify step rather than arriving as
refutations. Two findings were downgraded on verification and one materially corrected.

### The headline is a defect in what 2.8.0 shipped

`delaunay_2d` was rewritten in 2.8.0 with triangle adjacency and verified across 298 fixtures. It
is still **incomplete on ordinary uniform-random input** — 15–37% of point sets come back missing at
least one triangle, silently, no error signal. At n = 2000, eight triangles short.

The cause is not the rewrite. `d_max = max(dx, dy) * 10` builds a super-triangle only 10x the
bounding box, and Bowyer-Watson reduces to the true Delaunay triangulation only once no super-vertex
lies inside any real circumcircle. At 10x that fails, the real triangle is never created, and
compaction drops the super-vertex triangle covering the gap. Proven in exact rational arithmetic:
across 40 point sets, the triangles whose circumcircle encloses a super-vertex matched the observed
missing set **40 of 40**.

**And the obvious fix is wrong.** Raising the multiplier fixes uniform data and *breaks* clustered
data: at 1e9, a 1e-8-span cluster loses 20 triangles and gains 9 spurious overlapping ones, where
the current 10x is exactly correct. The multiplier is a genuine trade-off against the non-adaptive
in-circle determinant's conditioning — the two defects are **coupled**, precisely as
`issues/2026-08-04-incircle-precision.md` warned when it said not to fix that one by rescaling. 1e6
is a defensible interim; 1e9 must not be used; the durable fix is an adaptive predicate.

Three of the four criticals are in the collision narrowphase: `mpr_penetration` returns a wrong and
frequently negative penetration depth (65 of 81 box/box configurations disagree with the exact MTV,
27 with the wrong sign), and `gjk_epa_3d` is O(k²) in EPA iterations — 8,576 face evaluations and
1.24 MB of never-freed arena per call — while also returning wrong depths for sphere-vs-box.

### What this says about the last five releases

Every one of these 42 lived in code that 1801 assertions and 97% function coverage did not catch.
Coverage counts whether a function is *referenced*, not whether its contract is *checked* — the
report labels it "a floor, not a correctness proof" for exactly this reason. The test-quality
dimension found the sharpest example: **every ODE test integrand is autonomous**, so the entire
DOPRI45 abscissa row is certified by nothing.

### Not regressions

The regression dimension re-verified the 2.6.12–2.8.0 repairs by execution and found them all
holding: half-edge twin pairing, BVH degenerate split, interval enclosures (48,000 checks per
operator), FFT/IFFT round-trip and input preservation, `f64_fmod`/`f64_copysign` over 3,000 random
pairs, the SU(2)/SO(3)/SE(3) angle conventions over 600 axis/angle pairs, `geo_ray_plane`/
`geo_ray_capsule` over 2,000 rays, and the k-d tree split plane. One apparent SE(3) failure was the
verifier's own test bug and was refuted rather than reported.

## [2.8.0] - 2026-08-04 — delaunay_2d adjacency rewrite; it was **wrong**, not just slow

`delaunay_2d` now carries triangle adjacency and inserts by **walk + flood-fill Bowyer-Watson**,
replacing the flat-vec full rescan. Two previous attempts to speed it up failed (2.7.1: 2.6x slower
on cocircular; 2.8.0-attempt: broke the triangulation) — both are written up in
`docs/development/issues/2026-08-04-perf-delaunay_2d.md`, and both failed because they optimised a
representation that could not support the operation.

### The headline is a correctness defect, not the speedup

The premise behind the earlier attempts — "cocircular input is inherently expensive, the bad set
really is most of the triangulation" — was **wrong**, and I had accepted it. The shipped code was
not producing a triangulation at all on that class. At 150 points on a circle it returned **1651
triangles where 148 is correct**, with **206 edges shared by three or more triangles**, **9 input
points dropped**, and a total area **244x the convex hull**. It was overlapping soup.

The full rescan flagged far-away triangles through cancellation in the non-adaptive in-circle
determinant, so Bowyer-Watson kept creating spurious triangles. A flood fill cannot reach them, so
the cavity stays small — which is why the rewrite is both correct *and* faster here. The earlier
attempts were adding per-triangle work on top of the garbage.

**Nothing in the suite asserted any of this.** The eight existing `delaunay_2d` assertions all used
fixtures where the shipped code happened to be right.

| Benchmark | Before | After | |
|---|---|---|---|
| `delaunay_2d_400` (uniform) | 14.54 ms | **1.66 ms** | **−88.6%** |
| **`delaunay_2d_circle_150`** (cocircular) | **600 ms** | **455 µs** | **~1300x** |
| n = 1600 uniform | 222 ms | 7.9 ms | −96.5% |

Growth per doubling went from **3.9x (O(n²))** to **2.1x (O(n log n))** — the exponent changed, not
the constant. Arena use per call fell too: uniform n=1600 2090 → 1632 KiB; cocircular n=300
20118 → 329 KiB, a 61x reduction under an allocator that never frees.

`delaunay_2d_circle_150` is now a permanent benchmark. Its absence is exactly why the 2.7.1
regression would not have been caught.

### Method

Three independent implementations were built in isolated worktrees and each was checked by a
separate adversarial verifier that wrote its **own** differential harness — the issue file's
standing instruction after attempt 2 wasted a cycle on suite-level counts that localise nothing.
All three passed. The merged one (attempt C) was verified across **298 fixtures**: identical
canonical triangle sets everywhere the shipped output was valid, and on the 55 divergent
cocircular/near-cocircular fixtures the new output satisfies count = 2n−2−h, area = hull area,
zero non-manifold edges, zero dropped points, while the shipped output satisfies none of them.
**There is no input where the shipped code is valid and the rewrite is not.**

### Behaviour change

On exact/near-cocircular convex input the output set changes completely (circle n=60: 472 triangles
→ 58). That is a change from an invalid overlapping non-triangulation to a valid one, but a consumer
mapping triangle indices to per-index attributes will read a different mesh.

*Evidence:* five new assertions pin count, point-usage and manifoldness. Against the shipped code
they fail with 472-vs-58 triangles, 2 dropped points and 48 over-shared edges. Suite → **1801**.

### Coverage

`cyrius coverage` **94% → 97%** (570/587), all 35 files referenced. +31 assertions across the calc,
num_ext, optimize, lie_ext and linalg_ext tails. Four more of my own type errors were caught by the
gates — `pcg32_new` arity, `lorentz_adjoint` taking a Mat4 generator rather than a 4-vector, and
`bch_2nd_order` taking complex matrices rather than HVec3 (that one **SIGSEGV**'d, found by
bisecting the crashing suite).

## [2.7.3] - 2026-08-04 — coverage **80% → 94%**, all 35 files referenced

Target was 90%; the result is **94%** (555/587 functions). **Files referenced 34/35 → 35/35** — the
last one was `main.cyr`, previously excluded by design as the CLI smoke binary. Suite
**1677 → 1765**.

+88 assertions across the remaining untested surface, every expected value a closed form:

- **autodiff** — `dual_div`/`dual_pow`/`dual_abs` checked on *both* the value and the derivative
  (`d/dx x³ = 12` at x=2, `d/dx |x| = −1` at x=−5).
- **tensor** — `get`/`set`/`scale`/`contract`, including the rank-2 trace.
- **interval** — `ivl_exp` enclosure ordered and anchored at `e⁰ = 1`.
- **calc_ext** — partial derivative, gradient, Jacobian and Hessian of `x²+y²` against their exact
  values (`∇ = (6,8)`, `H = 2I`), and adaptive Simpson of `x²` to 1/3.
- **CGA** — the algebra identities rather than existence: `x ∧ x = 0`, `clone` preserving blades,
  `sub` of identical multivectors having zero norm, `scale` scaling.
- **symbolic_ext** — the whole `pat_*` constructor/accessor family plus `sym_rule`/`sym_rewrite`.
- **transforms / mat3 / vec3 / lie** — `t2d`/`t3d` accessors and composition, `m3` arithmetic,
  `hvec3_project`/`reject`/`angle`, `lorentz_boost`/`rotate` validated as genuine Lorentz
  transforms.

### Four more of my own errors, all caught by gates

Same pattern as 2.7.2 and worth the same note — every one was a *calling-convention* mistake that a
looser test would have hidden:

- **`num_crt` takes vecs, not raw `alloc()` buffers** — it reads them with `vec_get`. Raw buffers
  **SIGSEGV**; that is how the assertion was found, by bisecting a crashing suite down to one call.
- **`dual_pow`'s exponent is a plain f64, not an integer** — a raw `3` is the subnormal 1.5e-323, so
  `2³` silently returned 1.
- **`t2d_new` takes `(pos, rotation, scale)` with HVec2 pos/scale**, not five scalars; and
  **`hquat_from_euler` takes an explicit order argument**.
- **A `v3a` name collision.** `foundation.tcyr` already had that global; `cyrius lint` reported
  *"global var init refs 'v3a' declared at line 1043 (silent zero at init)"* — my later declaration
  would have silently zeroed the earlier uses. Renamed the whole block behind a `c73_` prefix. This
  is the flat-namespace hazard that 2.7.0-H added a gate for, hitting a **test** file this time.

`f64_pow` goes through exp/ln, so `2³` is 7.999… — that assertion uses a tolerance, not `f64_to`
equality, and the comment says why.

## [2.7.2] - 2026-08-04 — **coverage target met: 80%**

`cyrius coverage` **76% → 80%** (474/587 functions), the target set at the start of the 2.6.x audit
arc and carried through eleven releases. Suite **1643 → 1677**.

+34 assertions closing the last two large holes:

- **`lie_ext` SO(3)/SE(3)** — group identities, which are exact and need no tolerance: `R·R⁻¹ = I`
  on all nine entries, `|log(R)|` equal to the rotation angle, `(g·g⁻¹)` fixing an arbitrary point,
  `se3_identity` fixing every point, `se3_to_mat4`/`from_mat4` round-tripping, and BCH reducing to
  plain addition for parallel (commuting) arguments.
- **`num_ext` number theory** — exact integer answers: `extended_gcd(240,46) = 2` *and* its Bézout
  identity, `modinv(3,11) = 4`, `φ(36) = 12`, `σ₀(28) = 6` / `σ₁(28) = 56` (28 is perfect), CRT
  `(2 mod 3, 3 mod 5, 2 mod 7) → 23 mod 105`, `84 = 2·2·3·7` as four factors, Pollard-rho returning
  a real divisor of 8051, and Halton `(1, base 2) = ½` exactly.

### Three of my own errors, caught by the gates rather than by review

Worth recording because all three were *type* mistakes that a passing suite would have hidden:

- **`so3_from_quat` takes an SU(2) element, not an `HQuat`** — it calls `su2_to_quat` internally.
  Passing an `HQuat` silently reinterprets it and yields a **double-angle** rotation: `R[0][0]` came
  back `−1` (cos 180°) where `0` (cos 90°) was meant. This looks exactly like the half-vs-full-angle
  defect repaired in 2.6.13, and I initially took it for a recurrence of it. It is not — the library
  is correct and the call was wrong.
- **`se3_new` takes an SU(2) rotation, not a `Mat3`** — same family, same symptom.
- **`num_crt` takes vecs, not raw `alloc()` buffers** — it reads them with `vec_get`. Passing raw
  buffers **SIGSEGVs**, which is how that assertion was first written and how it was found.

None is a library defect. All three are recorded because "the test crashed" is the cheap outcome;
the expensive one is a test that passes against the wrong types and certifies nothing.

### Still open — carried to 2.8.x

`delaunay_2d` needs a data-structure change, not an optimisation (see 2.7.1 for the measurements
that rejected the x-sweep), and `_col_in_circumcircle`'s adaptive predicate remains latent at
0/180 under real geometry.

## [2.7.1] - 2026-08-04 — performance landings, coverage, and a rejected optimisation

Suite **1607 → 1643**, `cyrius coverage` **71% → 76%** (448/587 functions).

### Performance

| Benchmark | Before | After | Change |
|---|---|---|---|
| `triangulate_600gon` | 9.93 ms | **1.59 ms** | **−84%** |
| `kdtree_build_4k` | 2.27 ms | 2.54 ms | +12% (see below) |

**`triangulate_polygon`** now skips convex vertices when testing ear containment — only a *reflex*
vertex can lie inside a candidate ear. Unblocked by the 2.7.0-K contract documentation: it diverges
only on self-intersecting input, which ear clipping does not define, and the divergence direction is
old-partial → new-complete.

**`_kd_partition`** gained a balance guard rather than the unconditional quickselect the spec
proposed. That cost **+114%** on the common path (2.27 → 4.85 ms), so selection is now a *fallback*
taken only when the cheap midpoint split leaves a child below a quarter of its parent. Depth is
bounded at log₍₄∕₃₎(n) for +12%.

### `delaunay_2d` — implemented, measured, and REJECTED

The x-sweep-with-retirement rewrite was applied in full and measured on both input classes:

| Input | Shipped | x-sweep | |
|---|---|---|---|
| uniform scatter, n=400 | 15.09 ms | **5.37 ms** | −64% |
| **cocircular, n=150** | **615 ms** | **1616 ms** | **2.6× SLOWER** |

A pay-for-itself guard was then added — disable retirement once it has demonstrably failed to
retire anything — and **it did not help**: still 1.615 s. The cost is not the retirement scan. On
cocircular input Bowyer-Watson creates O(n²) triangles and each pays a `_col_circum_max_x` (with a
`sqrt`) **at creation**, before any switch can know whether retirement will pay.

So the optimisation is **reverted**, not shipped behind a flag. `delaunay_2d` stays O(n²).
Fixing it properly needs a different data structure — triangle adjacency for a walk-and-flood-fill
Bowyer-Watson — not a bolt-on to the flat vec. That is 2.8.x work, and the measurements above are
the specification for it.

### Coverage

+36 assertions closing the largest remaining holes: complex scalar ops (`cx_ln`, `cx_sqrt`,
`cx_sin`/`cos`, `cx_from_polar`, Pauli/Dirac matrices), geometry (`geo_line_*`, `geo_segment_*`,
`geo_obb_*`, barycentric, closest-point-on-triangle), and the symbolic accessors. Every expected
value is a closed form.

## [2.7.0] - 2026-08-04 — re-audit, repair, and the defects the repairs uncovered

Closes the 2026-08-04 re-audit. **35 of its 41 numbered findings are fixed**; the six that are not
each carry a written reason and a filed spec. Suite **1127 → 1605** (+42%), `cyrius coverage`
**59% → 71%**, benchmarks 28 → 35, constant gate 110 → 141 verified plus a new duplicate-global
check.

**Every fix is mutation-proven** — the source file is reverted and the new assertions confirmed to
fail — except two that are labelled rather than glossed: the BVH split (a cost, not a wrong answer,
so a benchmark guards it) and the incircle investigation (not fixed at all).

#### Performance — two of the three withheld optimisations now landed (2.7.0-O)

They were withheld in 2.7.0-I because each regressed on some input class. Both regressions are
addressed rather than accepted.

**`triangulate_polygon` reflex-only ear pruning.** Only a *reflex* vertex can lie inside a candidate
ear, so the containment scan skips convex ones.

| Benchmark | Before | After | Change |
|---|---|---|---|
| `triangulate_600gon` | **9.93 ms** | **1.59 ms** | **−84%** |

The blocker was that output length changes on *self-intersecting* input. That is now settled rather
than tolerated: 2.7.0-K documented the real contract (`== 3*(n-2)` complete, `< 3*(n-2)` partial),
and ear clipping is only defined for simple polygons anyway. The divergence direction is
**old-partial → new-complete** — the new code succeeds where the old bailed.

One of my own 2.7.0-K assertions failed on this, and it was the *assertion* that was wrong: it
pinned `< 9` for a self-intersecting pentagon, i.e. it asserted that this particular input bails.
That is pinning an implementation detail, not the contract. Re-pinned to `<= 9`.

**`_kd_partition` balance guard.** Unconditional quickselect fixed the distribution sensitivity but
cost **kdtree_build_4k 2.27 ms → 4.85 ms (+114%)** — far worse than the +28–30% the review
predicted, and the wrong trade for a case most inputs never hit. Made it a *fallback* instead: the
cheap value-midpoint split runs first, and exact median selection is taken only when the midpoint
leaves a child below a quarter of its parent. That bounds depth at log₍₄∕₃₎(n) — ~24 levels at
n = 10⁶ — for **+12%** on the common path.

| Benchmark | Midpoint only | Unconditional select | **Hybrid (shipped)** |
|---|---|---|---|
| `kdtree_build_4k` | 2.27 ms | 4.85 ms | **2.54 ms** |

**What I did not verify:** the finding's headline case — 574 ms vs 38 ms at n = 50,000 on
geometrically-spaced coordinates — **did not reproduce**. My probe used a ratio so close to 1 that
it degenerated into the coincident case the `lo == hi` path already handled, and hybrid and
midpoint-only timed identically (0.34 s). So the balance guard is justified by the depth bound it
provably enforces and by the plane invariant holding under geometric spacing (now asserted), **not**
by a measured win on that input. The 574 ms figure remains unconfirmed.

`delaunay_2d` stays withheld: its rewrite is 1.9–3.0x *slower* on cocircular input and changes the
triangulation of a unit square, and neither is addressed by a guard.

### The headline is not the audit's findings — it is what looking for them turned up

Four defects in this release were found by *checking that a repair preserved behaviour*, not by any
audit:

- **`kdtree_within_radius` returned the wrong count on 274 of 400 queries.** `split_val` was read
  from `items[split]` — some right-side value — while the partition was made at `mid_val`. 3,412
  points sat on the wrong side of an ancestor's pruning plane. Shipped for three releases; the
  2.6.14 kd-tree assertions pass happily on such a tree.
- **`solve_gmres` overshot its documented matvec budget without bound** — it converted a budget
  into an outer-cycle count while spending `1 + m` per cycle.
- **`triangulate_polygon` documented a return length it does not always deliver.**
- **Two findings had never been scheduled at all** — surfaced only by assigning a disposition to
  every numbered finding rather than reasoning from the tier summaries. That is the same failure
  mode this re-audit was written to correct, recurring inside the release correcting it.

### Corrections to this document, made before release

Recorded because the corrections matter more than the claims they replace:

- The incircle finding was filed **red** on the claim that `delaunay_2d`'s super-triangle makes
  every early insertion a mixed-magnitude test. The super-triangle is at **10x** the extent;
  re-measured with the real geometry, **0 of 180** cases disagree with exact arithmetic. Downgraded
  to latent.
- The duplicate-globals finding was first dismissed as overstated on the strength of a `grep` whose
  pattern assumed one space before `=`. All four globals really were duplicated. The gate found
  what the grep missed.
- The private-symbol coupling was reported as five pairs including a cycle. It is **six pairs and
  acyclic**; the two "extra" edges were comment-only mentions.

### Deliberately not applied

Three performance optimisations are confirmed real complexity wins and are **withheld**, each with
its measurement and review filed under `docs/development/issues/`:

- `delaunay_2d` x-sweep — **−96% at n=6,400**, but 1.9–3.0x *slower* on cocircular input and it
  changes the triangulation of a unit square.
- `_kd_partition` median selection — **+28–30%** on duplicate-heavy skewed data.
- `triangulate_polygon` reflex-only pruning — sound on ~33,000 simple polygons, but output length
  changes on self-intersecting input.

A faster wrong answer is worse than a slow right one.

### CORRECTION — the 2026-08-03 audit was **not** fully discharged
The 2.6.15 entry below claims it was. That claim is **wrong**, and the 2026-08-04 re-audit
([`docs/audit/2026-08-04.md`](docs/audit/2026-08-04.md)) found it. All four P-tiers *were* closed,
but six of the original 70 findings never appeared in those tiers and so were never scheduled:
`geo_ray_capsule`'s NaN miss, `cmat_exp`'s non-terminating scaling loop, `geo.cyr`'s ray-miss
contract, `f64_fmod`'s floor-vs-trunc, `f64_copysign`'s negative zero, and `lorentz_is_valid`'s
absolute tolerance. The repair work was scheduled from the roadmap's P0–P4 *summary bullets* — a
deduplicated digest — rather than from the audit's finding list, and "all tiers closed" was then
read as "all findings closed". **An audit's finding list is the unit of disposition.**

### 2.7.0 — re-audit & refactor (in progress)
The re-audit itself came back **clean on regression**: every 2.6.12–2.6.15 repair holds under
execution. It confirmed 42 new findings (4 high, 21 medium, 17 low; no criticals) — see
[`docs/development/roadmap.md`](docs/development/roadmap.md) §2.7.0. Two were fixed immediately
because they are gate infrastructure: `scripts/check-constants.sh` was silently skipping **35 of
145** constants (its regex rejected `_` digit separators) while printing "110/110 verified", and
the constant that exposed (`symbolic_ext.cyr:213`, encoding ~1e16 against a documented 1e15).

#### Fixed — the six carried-over findings (2.7.0-A) *(all mutation-proven)*

Every fix below was verified by reverting its source file and confirming the new assertions fail;
the failing output is the evidence, not the passing output. Suite 1127 → **1154**.

- **`f64_fmod` computed a floored modulo, not `fmod`.** It used `f64_floor` on the quotient where
  IEEE-754 `fmod` truncates. The two agree only when the quotient is non-negative, so the result
  carried the sign of the **divisor** instead of the **dividend**: `fmod(-7, 3)` returned `+2`
  where C returns `-1`, and `fmod(7, -3)` returned `-2` where C returns `+1`. Now `f64_trunc`.
- **`f64_copysign` could not see, or produce, a negative zero.** It branched on `f64_lt(y, 0)` — an
  *ordered* comparison — but `-0.0` compares equal to `+0.0`, so `copysign(3, -0.0)` returned `+3`,
  and `f64_neg` of a positive zero meant the function could never return `-0.0` at all. Replaced
  with the bit splice IEEE-754 actually specifies: `(x & ~signmask) | (y & signmask)`.
- **`geo_ray_capsule` returned NaN for any ray parallel to the capsule axis.** In that
  configuration `d_perp` is exactly the zero vector, so `a == 0`, `b == 0`, and the discriminant is
  exactly `+0.0` — which does *not* satisfy the `disc < 0` test guarding the cap-sphere fallback.
  Execution fell through to the quadratic and evaluated `0.5 / 0 = +Inf`, so a function documented
  to return `0` on a miss returned a NaN that then propagated into caller arithmetic. The parallel
  case now joins the cap-sphere path, which is the whole answer for it.
- **`geo_ray_plane` returned `-1` on a miss, against its own documented contract.** `geo.cyr` states
  "returns 0 on miss" in both the file header and the ray-section header, and every sibling
  intersection routine does exactly that. The raw i64 `-1` is `0xFFFFFFFFFFFFFFFF` — a **negative
  NaN** as an f64 — detectable only by integer comparison and silently poisoning any float
  arithmetic done on it. **Behaviour change:** see *Breaking* below.
- **`cmat_exp` never terminated on a non-finite Frobenius norm.** The scaling-squaring loop halves
  the norm until it drops below 1, but `+Inf * 0.5` is still `+Inf` and still `> 1` — so a single
  overflowing entry anywhere in the matrix hung the process forever. Verified: the pre-fix suite
  runs 1 second with the fix and does not finish in 110 seconds without it. Bounded at
  `_CMAT_EXP_MAX_SCALING = 64`, which is not a heuristic — `exp` overflows f64 above 709, so no
  larger scaling parameter can describe a representable result.
- **`lorentz_is_valid` rejected valid boosts.** It checked `Λᵀ η Λ = η` against a *fixed absolute*
  `1e-12`, but the entries of that product are sums of products of two matrix entries, so their
  rounding error scales with **cosh²(η)**. At rapidity 10, `cosh² ≈ 1.2e8` and one ulp is already
  `1.5e-8` — four orders of magnitude past the tolerance demanded. Now scaled by the matrix
  magnitude, leaving ~4500× headroom over the true rounding bound while still rejecting a
  genuinely non-Lorentzian matrix (whose defect is O(scale²), not O(1e-12)) — both directions are
  asserted.

#### Fixed — medium correctness & safety (2.7.0-C, 4 of 7)

Each finding was **independently verified against the source and then adversarially challenged**
before it was applied — 7 confirmed, 0 refuted, and every reviewer ran real code rather than
reasoning about it. The challenges were not a formality: one showed that the proposed
`levi_civita` assertions all pass a wrong-sign mutant, and another found a live SIGSEGV in
`wedge_1_1` that the finding never mentioned. Both corrections are in the fixes below.
Suite 1181 → **1217**.

- **`detect_islands` checked only the *upper* bound of a contact's body indices.** A negative
  index sailed through `ba < n_bodies` and `_col_uf_find` subscripted `parent[]` *before* the
  array. Two distinct failure modes, both measured: `parent[-1]` aliases the union-find header's
  `rank` **pointer**, so `parent + rank_ptr * 8` is a wild address (**SIGSEGV, exit 139**); small
  negatives stay inside the heap and corrupt *silently* — with `n_bodies = 4` and `body_a = -3`
  the out-of-bounds read returned root 0, fusing unrelated bodies 0 and 1, so the function
  answered **3 islands where the correct answer is 4**. Skipping an out-of-range endpoint is the
  decision the line already made for out-of-range-*high* indices; it now covers both directions.
- **`num_ifft` corrupted the caller's buffer on its error path.** Validation lived only in
  `num_fft`, which `num_ifft` calls *after* running its in-place conjugate pass — so a rejected
  length returned `HSB_ERR_INVALID_INPUT` with every imaginary part already negated. The caller
  got a failure code **and** a silently conjugated input. `num_fft` rejects the same `n` leaving
  `data` bit-identical; the inverse now offers the same guarantee.
- **`levi_civita_3` / `levi_civita_4` returned raw i64 where their siblings return f64 bit
  patterns.** Not a cosmetic type mismatch: raw `(0 - 1)` is `0xFFFF_FFFF_FFFF_FFFF`, a **negative
  NaN**, so a contraction `c_i = Σ_jk ε_ijk a_j b_k` poisoned every component — *including the ones
  that are exactly zero*, since `NaN * 0.0 = NaN`. Raw `1` is 4.94e-324, not 1.0. Eight existing
  in-tree assertions encoded the old integer contract and were migrated with the fix.
- **diffgeo's downstream contractions dereferenced the documented 0 and sized allocations from an
  unbounded dim** (CWE-190 → CWE-476). `christoffel_symbols` and `riemann_tensor` have capped at
  16 and returned 0 since they were written; `ricci_tensor`, `ricci_scalar`, `einstein_tensor`,
  `weyl_tensor`, `geodesic_state_new`, `geodesic_rk4`, `parallel_transport` and `killing_residual`
  did neither. Added a shared `_DG_MAX_DIM`, null guards on every operand, and an allocation check
  at **all ten** `alloc` sites — including the two in `christoffel_symbols`/`riemann_tensor` that
  the finding left asymmetric. Also fixes **`wedge_1_1`**, which was not in the finding at all:
  `n2 = dim*(dim-1)/2` at `dim = 100000` asks for 40 GB, so `alloc` returned 0 and the store wrote
  to address 0.

**Evidence.** Reverting `collision_mesh.cyr` yields 3 failures **plus SIGSEGV**; `num.cyr` 4
failures; `tensor.cyr` 12 (and a second, sign-only mutant that keeps the correct f64 encoding
still fails 5 — the case the original assertions missed); `diffgeo.cyr` **SIGSEGVs before printing
a single line**. Every restore was verified byte-identical afterwards, not assumed.

#### Fixed — unscaled Householder / unbalanced decompositions (2.7.0-D)

**The most consequential fix of the 2.7.0 line so far**, and the finding materially understated it.
Every step of these decompositions squares matrix entries — the Householder norms in
`_lp_bidiagonalize`/`_lp_tridiagonalize`, and the Wilkinson shifts, which form entries of `BᵀB`
explicitly. `|x| > 1.34e154` squares to `+Inf` and `|x| < 1.49e-154` squares to `0`, so the usable
band was only about **1 .. 1e23**. Measured across 440 random matrices, `svd_golub_kahan` failed
**40/40 at scale 1e-10** on a relative reconstruction check — that is SI metres, so the physics
consumer was in the failure region, not at some exotic endpoint.

Failure modes differed by direction, and both were bad:
- **Overflow (1e200):** `||v||² = +Inf`, then `2/Inf = 0` and `0*Inf = NaN` — `svd_golub_kahan`
  returned `HSB_ERR_NO_CONVERGENCE` with `U` **all NaN**; `cqr_decompose` was worse, because
  `cx_abs(x1) = Inf` made the `v^H v` guard false so **every reflector was skipped** and it
  returned `HSB_ERR_NONE` with `Q = I` and `R = A` — R not even upper triangular.
- **Underflow (1e-200):** every reflector skipped, `HSB_ERR_NONE` returned, and `S` set to the
  **raw diagonal of A** — 42.6% off, with no error code.

`svd_golub_kahan`, `eigen_qr` and `cqr_decompose` now balance to unit max magnitude before
decomposing and rescale on the way out (`sv(cA) = c·sv(A)`, `eig(cA) = c·eig(A)`; U/Vt/Q are
invariant). Balancing also converts the **absolute** `EPSILON_F64` deflation thresholds into
relative ones, which is what repairs the small-magnitude half.

The divisor is rounded **down to a power of two**, so the divide and the un-scale are exact and
every unit-scale result is **bit-identical to what shipped before** — verified: all four suites
pass unchanged. Without that refinement the shipped `S[0]` would have drifted by 1 ulp for no
reason. Doc comments for the new `HSB_ERR_ALLOC` return were updated in the same edit.

*Evidence:* reverting `linalg_precision.cyr` fails **9 of 11** new assertions. The other two are
deliberately vacuous and labelled as such — they document that the pre-fix failure was *silent*.

- **`expr_to_str` / `sym_to_latex` dropped the rounding carry.** stdlib `fmt_float_buf` zero-pads
  only under `if (pad > 0)` with `pad = decimals - flen`. A fraction that rounds *up* to exactly
  `10^decimals` needs `decimals+1` digits, so `pad` goes negative, the pad branch is skipped, and
  the overflowed digit is emitted **after** the `.` with nothing carrying into the whole part:
  `0.9999999` rendered as **"0.1000000"** and `2.9999999` as **"2.1000000"**. `lib/` is regenerated
  by `cyrius lib sync`, so the fix could not live there — hisab now carries `_sym_render_f64`, the
  stdlib algorithm plus the two lines that fold the overflow into `whole`. Deliberately *not* fixed
  by pre-rounding the value: `val * 10^6` is inexact above ~1e15 and would perturb the
  large-magnitude integral values `_latex_fmt_const` routes down the same branch. 5 of 6 new
  assertions fail on revert; the sixth is a control proving non-carrying values are untouched.

#### Fixed — rank-deficient SVD (2.7.0-E). **All 7 medium findings now closed.**

A zero entry on the **diagonal** of the active bidiagonal block makes the implicit-shift step a
no-op: `_lp_bidiag_qr` builds its first rotation from `(d[p_lo]² − mu, d[p_lo]·f[p_lo])`, and when
`d[p_lo]` is zero that pair is `(−mu, 0)`, whose rotation is a bare sign flip. The sweep reproduces
the block unchanged and spins until `max_iter`, so `svd_golub_kahan` returned
`HSB_ERR_NO_CONVERGENCE` with `out_S` **never written** — the caller's zero-filled buffer was all
it got back. The rank-1 `[[1,2,3],[2,4,6],[3,6,9]]` bidiagonalizes to `d = (−3.7417, 0, 0)`,
`f = (13.4907, 0)`, and one sweep drives it to `d = (~1e-16, 0, 0)`, `f = (−14, 0)`, which then
repeats forever.

Added `_lp_split_zero_diag`: a chase of exact Givens rotations that removes the lone superdiagonal
entry, splitting the block and deflating a singular value of exactly 0. This is the zero-diagonal
case of **Golub & Van Loan Alg. 8.6.2**, omitted from the original transcription. The rank-1
matrix above now returns `HSB_ERR_NONE` with `S = (14, 0, 0)` — the exact singular values.

Two claims in the proposal were corrected by review before landing, and the corrections are in the
source comments: the defect is **typical of** rank-deficient input, not universal (a 6×6 rank-3
counterexample converges on the unpatched module, because bidiagonalization can leave an
exactly-zero *superdiagonal* that the outer deflation loop splits first — measured 104/240 random
rank-deficient failures, but 60/60 once columns were forced dependent); and the split's `tol` uses
`EPSILON_F64` = 1e-12 rather than machine epsilon, a resolution floor probed at 1e-9/1e-11/1e-13
and found to cause no regression — at 1e-13 the pre-fix code already returned NO_CONVERGENCE with
`S` all zero, so the fix is strictly better.

*Evidence:* reverting `linalg_precision.cyr` fails 7 of 11 new assertions. The `s == 0` assertions
are **not** standalone and are labelled as such in the test — pre-fix, `out_S` is never written, so
a zero-filled buffer satisfies them trivially; they are meaningful only bundled with the
return-code and reconstruction assertions beside them.

#### Test quality — `num_rk4` had **zero effective coverage** (2.7.0-G)

`num_rk4`'s only assertion in the entire tree was `dy/dt = 1, y(0) = 0, dt = 1 → y(1) = 1`, checked
with `f64_to(erx) == 1`. It could not fail, for two independent reasons:

1. **The test problem carries no order information.** With `f(t,y) = 1` every stage of every
   explicit RK method returns 1, so Euler, midpoint, RK3, stages evaluated at the wrong `t`, and
   `k3` built from `k1` all return exactly 1.0. Order 1 versus order 4 is invisible.
2. **The comparison is a truncation, not a tolerance.** `f64_to` truncates toward zero, so the
   assertion accepts anything in `[1.0, 2.0)` — up to +100% relative error, roughly 1e15× looser
   than the method's own round-off floor.

`num_rk4` could have been replaced by `return f64_add(y, f64_mul(dt, f(t,y)));` — plain Euler — and
the whole 1250-assertion tree would have stayed green. **Confirmed by doing exactly that.**

Replaced with 16 assertions built on references derived independently of the implementation: the
stability function `R(z) = 1 + z + z²/2 + z³/6 + z⁴/24` that any 4-stage order-4 method must
satisfy (`R(1) = 65/24`, `R(1/2) = 211/128`, `R(−1) = 3/8`, all exact); Simpson-exactness on
`y' = 4t³`; the closed form `y(t) = exp(t²/2)`; and multi-step values computed in **exact rational
arithmetic** (`fractions.Fraction`, no floating point in the reference) against `e` and `√e` at 60
digits. Every hex constant produced by `struct.pack('>d')`, none typed by hand.

The strongest of these is an **observed order-of-convergence** check — `err(h)/err(h/2)` must land
in (14, 17). Measured 15.190 and 15.589 for RK4; RK3 gives 7.61, RK2 3.82, Euler 1.90.

Adversarial review then found the block still blind to `dt ≤ 0` — every case used a forward step —
so a sign defect in `half_dt` survived. Two backward-integration assertions were added, and both
kill it.

*Evidence:* against an Euler stub, **14 of the 16 new assertions fail**. The previous single
assertion passed. Suite 1250 → **1266**.

#### Test quality — `calc_integral_simpson` was untested for `a != 0` (2.7.0-G)

The existing assertions quantised through `f64_to`/`f64_round` and certified almost nothing:
`round(3I) == 1` accepts `I` in `[1/6, 1/2)` — ±50% on 1/3; `trunc(I) == 7` accepts `[7, 8)`. No
tolerance-based Simpson assertion existed anywhere in the tree.

Replaced with 20 assertions that are **exact where the rule is exact**. Simpson is exact for every
polynomial of degree ≤ 3, and on a dyadic grid (`n = 64` gives `h = 2^-6` or `2^-5`) every node,
node value, weighted term and partial sum is an exactly representable dyadic — the only rounding in
the whole computation is the closing divide by 3. So the degree-0..3 answers are asserted as **bit
patterns**, not tolerances. Degree 4 is where the rule stops being exact, and by exactly how much:
the fourth derivative is constant for a quartic, so Euler–Maclaurin is an identity rather than an
estimate and the returned value must be `1/5 + 2h^4/15` — provably *not* 0.2. Plus a 4th-order
convergence check against the exact-rational rule value for the integral of `1/(1+x)`:
`e(32)/e(64)` must land in (15.9, 16.05).

Adversarial review then found the whole block still blind to `a != 0` — **every case started at
zero**, so two independent a-drop mutants survived: `h = (b-a)/n -> b/n` and `xi = a + i*h -> i*h`.
Added the integrals of `x^2` and `x^3` over `[1,3]` (still dyadic, still bit-exact). Both mutants
now fail.

#### Test quality — `ode_verlet` / `ode_symplectic_euler` had zero numeric coverage (2.7.0-G)

Both were asserted **only** with `!= 0` on the returned block. `alloc()` never returns 0 on
success, so neither assertion could fail for any input: two integrators, no coverage at all.

Both maps are linear on the SHO, so both have an exact closed form, derived from the textbook
definitions independently of `src/ode.cyr`. With `theta = 2*asin(h/2)`, Verlet gives
`q_n = cos(n*theta)` and `v_n = -cos(theta/2)*sin(n*theta)`. Each conserves a quadratic **shadow
Hamiltonian** exactly, which yields the property that makes a symplectic method worth having —
*bounded* energy error forever: `|E_n - 1/2| <= h^2/8` for Verlet, `<= h/(2(2-h))` for symplectic
Euler. Both bounds are asserted over 4000 steps (t = 125, ~19.9 periods), **and so is their
attainment** — a lower bound too, since a frozen no-op integrator would sit at `E = 1/2` exactly
and sail through the upper bound alone. Explicit Euler violates the symplectic-Euler bound by
~3000x.

Review found three further holes, all now closed: neither integrator was ever handed a force other
than `a(q) = -q`, so **hardcoding the SHO and ignoring the function pointer passed every
assertion**; the result was always consumed before the next call, so a shared static scratch buffer
would have been invisible; and `alloc(8)` where 16 bytes are stored overran undetected. Added
constant-acceleration cases (integrated exactly by both methods, so bit-exact), guard allocations,
and a re-read of the first result after 4001 later calls.

*Evidence:* each of the three previously-surviving mutants now fails — hardcoded `accel_fn` 2
failures, hardcoded `dp_dq` 2, `alloc(8)` 1. Suite 1266 → **1298**.

#### Test quality — Gell-Mann tautology and six existence-only assertions (2.7.0-G cont.)

**`gell_mann`.** The "hermitian" assertion compared `Re(l1[0][0])` against
`Re(adjoint(l1)[0][0])` with a tolerance of **1.0**. `cx_conj` copies the real part unchanged, so
that is `Re(x) - Re(x) = 0 < 1` — an identity for *every* complex matrix ever built. The traceless
check used a tolerance of 1.0 as well.

Replaced with the three properties that actually characterise the SU(3) basis — elementwise
Hermiticity (deliberately *not* via `cmat_adjoint`, which is what made the original vacuous),
tracelessness, and `tr(la lb) = 2 delta_ab` across all 64 pairs. Adversarial review then showed
those three are invariant under **any real orthogonal mixing of lambda_4..lambda_7**, so a sign
flip on l5 or l7, a swap of l4 and l6, or a 45-degree rotation of (l4, l6) all still passed. Added
convention pins for each of those four generators, plus the algebra closure
`[la, lb] = 2i sum_c f_abc lc` over all 64 pairs — which is also the **first coverage
`su3_structure_constant` has ever had**; `grep -rn "su3_" tests/` previously returned nothing, so
the table and the matrices had never been checked against each other.

**Six existence-only assertions.** `cga_point`, `opt_gradient_descent`, `eigen_power`,
`hvec4_length`, `num_dst` and `sym_to_latex` all had their existence asserted and their value never
checked. Each now pins a closed-form result: the CGA null-basis coefficients are *forced* by the
two defining properties of the embedding (`P.P = 0`, `P.e_inf = -1`), so they are derivable without
reading the source; DST-I is pinned with delta inputs that read off one column of the kernel,
because a round-trip alone is satisfied by `-S` and by any other symmetric involution.

Review found three residual holes and all three are now closed — each verified by applying the
mutant to source and watching the suite fail:

- `eigen_power` used only **symmetric** test matrices, so a transposed matvec (power iteration on
  `A^T`, i.e. the *left* eigenvector) was invisible. Added `[[1,2],[3,4]]`, whose right and left
  dominant eigenvectors differ.
- Both `eigen_power` matrices had a **positive** dominant eigenvalue and a positive eigenvector, so
  losing the sign of the pivot was invisible. Added `[[-3,1],[1,-2]]`.
- `sym_to_latex` rendered MUL and NEG only in `paren == 0` position, so dropping a
  `\left(...\right)` group was invisible.

*Evidence:* 1353 -> **1430**. The three mutants above fail 1, 3 and 1 assertions respectively; a
lambda_5 or lambda_7 sign flip fails 2 each; stubbing `su3_structure_constant` to 0 fails 1.

#### Test quality — 21 assertions whose tolerance was a literal 1.0 (2.7.0-G cont.)

The last and largest of the six. **15 static assertion sites** (6 in `hisab.tcyr`, 9 in
`modules.tcyr`) plus 8 loop iterations — 22 at runtime — compared a quantity whose closed form is
exactly 0 or 1 against `f64_from(1)`. A tolerance of 1.0 around an expected 1.0 admits everything
in `[0, 2]`. `foundation.tcyr` additionally declared `var TOL = f64_from(1);` — dead, unreferenced,
and a trap waiting to be picked up; removed after verifying nothing uses it.

Concrete examples of what those bounds accepted:
- `"rot90.x near 0"` / `"rot90.y near 1"` were satisfied by **every** rotation angle, since
  `|cos t| < 1` and `|1 - sin t| < 1` for all `t` in `(0, pi)`.
- `"srgb roundtrip"` was satisfied by every value `linear_to_srgb` can return.
- A single `[0][0]` entry is not an inverse test at all — `m4_inverse` returning its input
  unchanged passed it.

Replaced with named tolerances (`ULP_TOL_*` = 1e-15, ~4.5x the measured floor of a few ulp) and
closed-form expected values. Measured residuals: `rot90.x` 2.22e-16, `rot45.x` 1.11e-16,
srgb round-trip 1.67e-16, Euler 1.22e-16 — every one within 1 ulp.

The adversarial review is again the more interesting half. It confirmed the census was complete
(`\bTOL\b` across `tests/` returns exactly the one dead site) and that the patch **kills 11
mutants** — including a quaternion full-angle error, `lorentz_boost_x` reading velocity as
rapidity, and `m4_inverse` dropping its translation column, caught only by the new 16-entry sweep.
But it also found **seven survivors**, each now closed:

- **`cx_exp` returning the conjugate.** `|e^(i*pi) + 1| == 0` is blind to conjugation, because pi
  is a fixed point of `z -> conj(z)`. Pinned at `e^(i*pi/2)`, where the two differ.
- **`cmat_adjoint` stubbed to a plain copy.** This is the one that matters most: it would make the
  new 8-generator Hermiticity sweep compare `A` with `A` — vacuous again, the exact failure mode
  being repaired. Pinned on a non-Hermitian matrix.
- `minkowski_interval` dropping the z² term (the test 4-vector was `(5,3,0,0)`), `linear_to_srgb`
  using the decode-side breakpoint, an `su2_compose` cross-term sign flip, a Gell-Mann λ₄/λ₅ swap,
  and `m4_inverse` transposing its upper-left 3×3.

*Evidence:* 1430 -> **1523**. Re-running the survivors: conjugated `cx_exp` fails 1 in each of two
suites, the `cmat_adjoint` stub fails 3, the dropped z² fails 1.

**All six test-quality findings are now closed.**


`docs/development/issues/2026-08-04-tq-*.md` (the unit-tolerance sweep -- 21 assertions whose tolerance is a literal 1.0). All six were adversarially reviewed and all six stand
— but the reviewers also catalogued **~35 mutants the new assertions still would not catch**, which
is the next round of work rather than a rejection. The most alarming: `calc_integral_simpson`
survives `h = (b − a)/steps` → `h = b/steps` **and** `xi = a + i·h` → `xi = i·h`, i.e. it is
effectively untested for `a ≠ 0`; `ode_verlet`/`ode_symplectic_euler` never actually invoke the
caller's function pointer (hardcoding the SHO survives every assertion), and survive `alloc(8)`
where 16 bytes are stored; `eigen_power` survives a transposed matvec because both test matrices
are symmetric; and the Gell-Mann basis leaves the **sign of λ₄..λ₇ unconstrained** — pinning it
needs the SU(3) structure constants.

#### Added — quadtree, octree and spatial hash had **zero** coverage (2.7.0-N)

Three whole data structures, **11 public functions**, untested through 2.7.0: `quadtree_new` /
`_insert` / `_query`, `octree_new` / `_insert` / `_query`, and `spatial_hash_new` / `_insert` /
`_query_cell` / `_query_radius` / `_clear`.

That is exactly the shape of the k-d tree defect closed in 2.7.0-J — a spatial structure in
`spatial.cyr` whose queries nobody checked, which then returned wrong answers for three releases.
`spatial.cyr` stood at **3 of 14 functions** referenced.

Each is now checked against a counted expectation rather than for non-null:

- **quadtree** — a 20×20 grid over `[0,100]²`; the box `[10,30]²` must find exactly **25** of 400,
  the whole domain exactly **400**, and a disjoint box exactly **0**.
- **octree** — an 8×8×8 grid over `[0,80]³`; `[0,20]³` must find exactly **27** of 512, the whole
  domain **512**.
- **spatial hash** — `query_cell` must find exactly the **10** points sharing cell (0,0,0). The
  radius query is documented as a *broadphase*, so it is asserted as a **superset** of the true
  answer — it may over-report but must never miss — with a companion assertion that the true-hit
  count is non-zero, so the superset test cannot pass vacuously. `clear()` must actually empty it.

All twelve passed on the first run, so the expected counts were right independently of the
implementation.

`cyrius coverage` **63% → 65%** (587 functions, 382 referenced); it was 59% at the start of the
2.7.0 line. Suite 1533 → **1545**.

#### Added — a verified module dependency manifest (2.7.0-M)

`src/` modules carry **no `include` lines** — they are self-contained text the bundler
concatenates. That makes à-la-carte consumption possible but leaves the required set implicit,
which is exactly why the README's example shipped broken until 2.7.0-H.

The open item asked for a decision between three options — ship a manifest, promote the private
symbols, or drop the à-la-carte claim. **Measuring the graph settled it**, and corrected the
finding on the way:

- Six module pairs reach into another module's `_`-prefixed internals, not five. Two more pairs
  *appeared* to, including a `symbolic` ↔ `symbolic_ext` cycle — both were **comment-only
  mentions**, which a naive grep counts and a comment-stripping scan does not.
- The real graph is **acyclic**. Four modules are fully standalone (`error`, `interval`,
  `symbolic`, `tensor`); the deepest is `lie_ext` at 9.
- **Every** private edge runs from a derived module to its own base — `calc_ext`→`calc`,
  `num_ext`→`num`, `lie_ext`→`lie`, `symbolic_ext`→`symbolic`, `collision_mesh`→`collision_core`,
  `noise_simplex`→`calc`. That is the layering the module *names* already declare.

So this was never tangled coupling requiring symbol promotion; it was an undocumented layering.
`docs/architecture/overview.md` now carries the per-module include sets, **derived from source and
verified by compiling each of the 34 modules against exactly its listed set — 0 undefined symbols
for all 34**, plus a negative control (`geo` without `quat` correctly reports 1). The sets are the
transitive closure, so they are what you must include rather than what a module names directly.

The README's à-la-carte comment now points at the table instead of restating a partial list.

#### Added — benchmarks for the hot public paths that had none (2.7.0-L)

Every function repaired in 2.7.0-I/J/K, and every target of a *withheld* performance patch, was
invisible to `bench-history.csv` — so a regression in any of them could not be seen. That is the
gap that let a 3,412-violation k-d tree and an O(n²) Delaunay both sit unnoticed. 29 → **35**
benchmarks.

| Benchmark | Baseline |
|---|---|
| `kdtree_build_4k` | 2.27 ms |
| `kdtree_radius_4k` | 7.1 µs |
| `delaunay_2d_400` | 15.2 ms |
| `triangulate_600gon` | 9.9 ms |
| `svd_golub_kahan_12` | 181 µs |
| `eigen_qr_12` | 140 µs |

These are **baselines, not wins** — nothing here got faster. Their value is that the three withheld
optimisations (`delaunay_2d`, `_kd_partition`, `triangulate_polygon`) now have a committed
before-number to be measured against when they are revisited, rather than being re-derived from
scratch by whoever picks them up. `kdtree_radius_4k` also guards the 2.7.0-J repair on the cost
side: the corrected split plane prunes *more*, so a future regression that re-broke the invariant
would likely show up as a speed change too.

`tests/hisab.bcyr` gained `spatial.cyr`, `linalg_ext.cyr` and `linalg_precision.cyr` includes —
none of the three had ever been benchmarked at all.

#### Fixed — the two findings that were never scheduled (2.7.0-K)

Both were surfaced by the disposition sweep, not by any tier heading — they had sat in the audit
since 2026-08-04 without ever entering the work-list.

**`solve_gmres` overshot its documented matvec budget without bound.** `max_iter` is documented as
"maximum total matrix-vector products", but the code converted it into an *outer-cycle* count —
`outer_limit = trunc(max_iter / m) + 1` — while each cycle spends `1 + m` products. True cost:
`(trunc(max_iter/m) + 1) * (m + 1)`, which is neither `max_iter` nor a bounded multiple of it. The
audit said 2.2x; the reviewer said unbounded, and **the reviewer was right** — the ratio grows with
`restart / max_iter`, so no fixed safety factor fixes it. Measured on a 3×3 system: budget 1 with
restart 100 spent **4** products.

`A_fn` is a callback precisely because it may be arbitrarily expensive, so the budget is a resource
limit, not a hint. Now counted, with a mid-cycle break — the Krylov space built so far is still
valid, so the least-squares solve uses the completed steps and returns the best answer within
budget. Measured after: 4, 1, 3 products against budgets 10, 1, 3.

**`triangulate_polygon` documented a return length it does not always deliver.** The header stated
`length = 3*(n-2)` unconditionally, but `if (ear_found == 0) { return result; }` returns early on
degenerate or self-intersecting input with no error signal — a caller could not distinguish a
complete triangulation from a truncated one. The information was always present; it was simply
undocumented.

Documented the real contract (`== 3*(n-2)` complete, `< 3*(n-2)` partial, `0` no ear on the first
pass) with the detection test spelled out, and pinned it with assertions so it is a *tested*
contract rather than an accident. Ear clipping is only defined for simple polygons, so a short
return on self-intersecting input is the honest answer, not a bug to paper over.

*Evidence:* reverting `linalg_ext.cyr` fails both budget assertions. Suite 1526 → **1533**.

#### Fixed — the k-d tree's split plane did not separate its own subtrees (2.7.0-J)

**The most serious defect found in the 2.7.0 line.** `kdtree_within_radius` returned the wrong
count on **274 of 400** queries over a 1,500-point scatter — a spatial query silently under-reporting
hits, with no error signal.

`_kd_build_rec` set each node's `split_val` from `items[split]` — *some* right-side value — while
the partition was actually made at `mid_val`. Those are not the same number. Every right-side point
whose axis value fell below `items[split]`'s therefore sat on the wrong side of its own node's
pruning plane, and `_kd_nearest_rec` / `_kd_radius_rec` pruned subtrees that still contained valid
hits. Measured on the pre-fix tree: **3,412 points on the wrong side of an ancestor's plane.**

A second, independent defect in the same function: the "ensure at least one item on each side"
clamp moved an item across the partition *after* the fact, producing a left subtree containing a
point above the plane. Its justifying comment claimed the midpoint split "can ONLY degenerate when
lo == hi" — **that reasoning does not hold in floating point**, because `(lo + hi) * 0.5` can round
down to exactly `lo`, and then `lo < mid_val` is false for the very point that attains `lo`. The
clamp now retries with the plane at `hi`, which keeps both sides non-empty *and* value-sound.

`split == end` is genuinely unreachable when `lo < hi` (`mid_val <= hi`, so the hi-attaining point
never compares below it), so that clamp is gone rather than kept as dead defensive code.

The 2.6.14 coincident-point fix is untouched and still asserted — for `lo == hi` the plane is the
common value, which separates correctly whichever way the positional split falls.

*Evidence:* against the shipped code the two new assertions fail with **3,412** plane violations and
**137 of 200** wrong query counts. Both were absent before: the 2.6.14 kdtree assertions pass
happily on a 3,412-violation tree, which is why this survived three releases. The invariant is now
checked structurally on **every node**, not just through behaviour. Suite 1523 → **1526**.

**How it was found.** Not by an audit. It surfaced because an adversarial reviewer, checking whether
a *performance* patch preserved results, built a brute-force oracle — and the shipped code failed it.
The perf patch that prompted this was **not applied** (see 2.7.0-I); the defect it exposed was.

#### Investigated — `_col_in_circumcircle` sign loss: confirmed, and the claim narrowed (2.7.0-J)

The 2.7.0-I review reported the shipped in-circle predicate as wrong on "near-coincident clusters".
Checked against exact rational arithmetic (`fractions.Fraction`, every operand converted exactly
from its f64 bit pattern):

| Configuration | Disagreements with exact |
|---|---|
| Uniformly small clusters, scale 1e-11 … 1e-6, plus unit-scale controls | **0 of 30** |
| **Mixed magnitude** — one vertex at 1e3 … 1e7 with two near-coincident vertices at 1e-10 … 1e-7 | **3 of 40** |

**Near-coincidence is not the trigger.** The predicate translates every point to the query point
first, which is the well-conditioned formulation, and it handles a uniformly small cluster
correctly. What breaks it is the *magnitude spread*: `a_sq` for the far vertex is ~1e14 while the
terms it is differenced against are ~1e-14, so the determinant is a difference across ~28 orders of
magnitude and every significant bit is lost.

**CORRECTED before release.** The first draft of this entry claimed the configuration is common
here, because `delaunay_2d` "begins from a super-triangle placed far outside the data, so every
early insertion is a mixed-magnitude test". That is false: the super-triangle sits at
`max(dx, dy) * 10` — **10x** the extent. Re-measured with that actual geometry, across cluster
spreads from 1e3 to 1e13: **0 wrong of 180**. The 3-of-40 failures came from a synthetic
configuration with absolute coordinates from 1e7 down to 1e-10, and the failure tracks the
*absolute* magnitude of the far vertex (it enters the determinant squared) rather than the ratio.

So the predicate is genuinely non-robust, but **no reachable path through `delaunay_2d` is known**.
Severity is latent, not red. The lesson is in the method: the first probe generated coordinates by
magnitude class instead of reconstructing the caller's real geometry.

**Not fixed.** The remedy is an adaptive exact predicate (Shewchuk's `incircle`: floating-point
determinant with a forward error bound, exact expansion only when |det| falls below it). Rescaling
cannot help — the spread is inherent to the super-triangle construction. Filed with its repro at
`docs/development/issues/2026-08-04-incircle-precision.md` rather than attempted here, because a
half-done robust predicate is worse than a documented sharp edge.

#### Performance — `solve_bicgstab` leaked a work vector per iteration (2.7.0-I)

`s` was allocated **inside** the iteration loop. `alloc()` is a bump allocator that never frees, so
every pass abandoned another `n*8` bytes — the same hot-loop-allocation class 2.6.15 cleared out of
LM and L-BFGS. Hoisted alongside `v` and `p`.

| Measure | Before | After | Change |
|---|---|---|---|
| Arena, n=1024, max_iter=128 | **3,194,880 B** | **2,154,496 B** | **−32.6%** |

Asymptotically `8n(6 + 3·max_iter)` → `8n(7 + 2·max_iter)`. Results are **bit-identical** —
verified across `n` ∈ {0, 1, 3, 7, 64, 65, 400, 512, 1024}, convergent and non-convergent, because
all `n` slots are overwritten before any read.

The finding's "superlinear" framing was **refuted**: the time cost is linear. This is a leak, not a
complexity defect, and saying so is the point — the CSV would not have shown a time win.

#### Two shipped correctness defects found while checking that optimisations preserve results

Neither is a performance issue. Both came from adversarial reviewers building independent oracles
to verify that a proposed optimisation did not change results — and finding the **shipped** code
was already wrong. They outrank the perf work that exposed them, and are filed accordingly.

- **`kdtree_within_radius` returns the wrong count on most queries.** Measured on a 1,500-point
  scatter with 400 radius-60 queries: **222 of 400 wrong**. The 2.6.14 kdtree assertions did not
  detect a tree with 676 pruning-plane violations. A spatial query returning wrong answers.
- **`_col_in_circumcircle` is less accurate than a plain circumcentre test.** All 23,741 retirements
  flagged by a candidate sweep predicate were re-checked in **exact rational arithmetic**: zero were
  geometrically unsound. The divergences came from the *shipped* in-circle determinant losing
  precision on near-coincident clusters, so Delaunay output on such inputs is already suspect.

#### Three optimisations confirmed but deliberately NOT applied

Each is a real complexity win, and each was measured at multiple sizes so the *growth ratio* — not
a single timing — carries the claim. All three are filed with their reviews under
`docs/development/issues/2026-08-04-perf-*.md`.

- **`delaunay_2d`** is O(n²) confirmed: growth converges on **4.0x per doubling**, 217 ms at
  n=1,600. An x-sweep-with-retirement rewrite reaches 2.1–2.5x per doubling (**−93% at n=3,200,
  −96% at n=6,400**). Not applied: it is **1.9–3.0x slower** with up to 2.5x memory on *cocircular*
  input, and it changes the triangulation on inputs as simple as the **unit square** — both results
  valid, zero shared triangles.
- **`_kd_partition`** confirmed, but the patch costs **+28–30% on duplicate-heavy skewed data** and
  ships **zero permanent regression coverage** for the invariant it depends on.
- **`triangulate_polygon`** reflex-only pruning is **sound on simple polygons** — ~33,000 tested,
  including an exhaustive enumeration of every 4-, 5- and 6-gon on a 3×3 integer grid, **0
  divergences**. It diverges only on self-intersecting input, which is outside the documented
  contract — but **output length changes in both directions**, including cases where the shipped
  code returns EMPTY and the new one returns a complete triangulation. That is entangled with the
  never-scheduled `collision_core.cyr:511` contract finding, so the two should land together.

A faster wrong answer is worse than a slow right one; these stay open until the regressions and the
missing coverage are addressed.

#### Fixed — duplicate globals in a flat namespace, now gated (2.7.0-H)

`calc.cyr` and `calc_ext.cyr` each declared `F64_THREE`, `F64_FOUR`, `F64_SIX` and `F64_FIFTEEN`.
Cyrius has **one flat namespace with last-definition-wins**, and `calc_ext.cyr` follows `calc.cyr`
in the `[lib]` bundle order — so the values that actually shipped were chosen by *manifest order*,
not by the file being edited. Invisible for exactly as long as the two copies happened to agree.
This is the hazard 2.6.8 addressed for `ERR_*`, recurring.

Removed the four duplicates from `calc_ext.cyr`, and — more importantly — **added duplicate
detection to `scripts/check-constants.sh`**, so the class cannot come back. It reports every
top-level global declared in more than one `src/` file, and fails whether the values differ *or*
agree; two copies agreeing today is a coincidence with a deadline, not safety. The matcher is
anchored at column 0, since Cyrius indents function bodies and a leading-whitespace `var` is a
local that shadows nothing.

*Evidence:* re-adding `F64_SIX` to `calc_ext.cyr` with a different value exits 1 and names both
sites and both values; restoring exits 0. Constant coverage 145 -> 141 declarations (four fewer
because four duplicates are gone), 1523 assertions unchanged.

**A correction to my own analysis.** My first census of this finding reported it as overstated —
"only `F64_FIFTEEN` is duplicated; `F64_THREE`/`FOUR`/`SIX` are not defined in `src/` at all". That
was **wrong**. My grep pattern used a single space before `=` and these files align their
declarations with several, so it silently matched nothing. The original finding was accurate on all
four. The gate found what the grep missed, which is the argument for gates over greps.

#### Fixed — README's à-la-carte example did not compile (2.7.0-H)

It listed `include "lib/…"` paths, but `lib/` holds vendored stdlib only — the library source is in
`src/`. It was also missing `vec2.cyr` and `vec4.cyr`, which the example needs without ever naming
them (`quat`/`mat4`/`transforms` reach into `HVec4_*`). Both fixed and **verified by building and
running it**: it compiles clean and the ray-sphere hit really is `4.0`, as the comment claims.

#### Fixed — BVH block header described a different data structure (2.7.0-H)

The header over `geo_advanced.cyr`'s BVH block was wrong in five ways, each checked against the
code: it is **not** a flattened vec-of-nodes arena (every node is its own `alloc()`); nodes are
**32 bytes, not 40**; there is no `[32]` padding slot; `[8]`/`[16]` hold child **pointers**, not
indices; and `bvh_build` returns the **root node pointer**, not a `{ nodes_ptr, count }` handle.
Rewritten to describe what is actually there, with the old claims listed so the correction is
auditable.

#### Fixed — `eigen_qr` non-convergence (2.7.0-F)

Last release I said this finding was **not reproduced** and left it open rather than close it on
evidence that did not support either verdict. Reproduced now, by sweeping 150 random symmetric
4×4s across three magnitude regimes: **1 failed**. Its condition number is only **240** — a matrix
LAPACK's `dsyev` handles without comment — with eigenvalues 27.793, −2.999, 0.417, −0.116, and
`eigen_qr` returned `HSB_ERR_NO_CONVERGENCE` even at `max_iter = 1,000,000`.

The cause was the **block-boundary logic**, not the QR sweep. `_lp_tridiag_qr` located the start of
the unreduced block with two loops:

1. The first contained a **fake break** — a `var _break = 1;` and two no-op `p_lo = p_lo;`
   statements with a `# break` comment. Cyrius has a real `break`; this one does nothing, so the
   loop always ran to 0, zeroing offdiagonals across the whole matrix as a side effect.
2. The author evidently noticed it did not work and discarded its result — the next line is
   literally `# Re-find p_lo properly` — with a second loop whose test was an **absolute**
   `EPSILON_F64`, inconsistent with the **relative** `eps·(|d[i]| + |d[i−1]|)` threshold the
   deflation test three lines above uses.

On a matrix whose entries span 0.04 .. 24.6 those two disagree about what counts as negligible, so
the block bounds and the deflation test could contradict each other and the sweep stalled. Replaced
with a single loop using the same relative threshold as the deflation test.

*Evidence:* the 150-matrix sweep goes **1 failure → 0**. All six new assertions — convergence, the
four eigenvalues against `numpy.linalg.eigvalsh`, and the `A·v = λ·v` relation — fail on revert.
All four suites are unchanged by the fix, so it is not a tolerance loosening. A separate pre-existing defect surfaced
alongside the balancing work: `eigen_qr` returns `HSB_ERR_NO_CONVERGENCE` at *unit* scale for a
well-conditioned symmetric 4×4 (cond 6.85), reproduced on unpatched source.

### Changed
- **`lorentz_boost_x/y/z/…` parameter renamed `beta` → `eta`.** The bodies take `cosh`/`sinh` of the
  argument — it was always **rapidity** — while the signature said `beta` (velocity) and the doc
  comment said "rapidity eta". A consumer trusting the signature and passing a velocity would have
  silently built the boost for a much smaller speed. No behaviour change; the name now matches.

#### Fixed — the three high findings (2.7.0-B)

`geo_advanced.cyr` is the largest module (1,261 lines) and had **zero** coverage on GJK / EPA /
BVH / TOI before this release. Suite 1154 → **1181**.

- **`cmat_mul` never checked conformability and read past the end of `b`.** It ran the inner
  product over `cols(a)` while indexing `b` by that same bound, so any `b` with fewer rows than
  `a` has columns was read out of bounds — `b`'s buffer is allocated at exactly
  `rows(b)*cols(b)*16` bytes. A 2×3 times a 2×2 read two complex numbers of adjacent heap. Now
  returns 0 for non-conformable operands.
- **The designed-0 return was being dereferenced one call-link further out.** `cmat_new` and
  `cmat_mul` both return 0 by design (dimension cap, non-conformable), but `cmat_add`, `cmat_sub`,
  `cmat_adjoint`, `cmat_scale` and `cmat_trace` all called `load64` on it immediately —
  `cmat_commutator` feeds exactly that in, since it passes two `cmat_mul` results straight to
  `cmat_sub`. Both mutants **SIGSEGV**; with the guards they return 0. `cmat_add`/`cmat_sub` also
  now reject mismatched shapes, and `cmat_trace` returns 0 rather than `cx_zero()` — zero is a
  valid trace, so returning it would make a propagated failure indistinguishable from a result.
- **`time_of_impact` ignored `max_t` and missed every collision past 0.64 units.** The advancement
  loop was bounded by `GJK_MAX_ITER` — a cap on GJK's *simplex refinement*, not on time sampling —
  while stepping a fixed `0.01 / speed`. The two were conflated, so the search covered
  `64 × 0.01 = 0.64` units of relative displacement and then returned 0 no matter what horizon the
  caller asked for: a **false negative from a collision query**. The step budget is now its own
  constant and spans the whole horizon, with the tolerance step as a floor so short horizons keep
  full resolution.

### Performance

`_bvh_build_rec` fell back to a **1-vs-(n−1) split** whenever no center sorted below the split
midpoint — which is precisely what coincident or tightly clustered AABBs do, since then every
center *is* the midpoint. Because each level re-merges its whole remaining range to compute
`bounds`, that made the build **O(n²)** in time and arena, not merely O(n)-deep. It is the unfixed
sibling of the `kdtree_build` defect closed in 2.6.14. Splitting at the median restores O(n log n).

| Benchmark | Before | After | Change |
|---|---|---|---|
| `bvh_degenerate_4k` (4,000 coincident AABBs) | **2.722 s** | **15.5 ms** | **−99.4%** (~176×) |

Measured in `tests/hisab.bcyr` on both sides; the row is in `bench-history.csv`. An isolated
scaling probe confirms the complexity change rather than a constant-factor win — per doubling of
n the old form cost 4.0× and the new form 2.1×:

| n | Before | After |
|---|---|---|
| 1,000 | 174.3 ms | 3.46 ms |
| 2,000 | 677.3 ms | 7.36 ms |
| 4,000 | 2,690 ms | 15.94 ms |
| 8,000 | 10,720 ms | 35.22 ms |

Note on evidence: unlike every other fix this release, the BVH repair is **not** mutation-proven by
an assertion — 20,000 coincident boxes complete under both forms, so the defect is a cost, not a
wrong answer, and `assert_eq` cannot state it. The new assertions guard the *result* (all 20,000
boxes still reported, disjoint queries still empty, separated inputs unaffected); the benchmark
guards the cost.

### Breaking
- **`geo_ray_plane` now returns `0` on a miss instead of `-1`.** This aligns the code with the
  contract stated twice in its own module and with every sibling intersection function. No caller
  in `src/` depended on the old sentinel. **Migration:** consumers testing `== (0 - 1)` should test
  `== 0`. Consumers already doing float comparisons on the result were getting NaN semantics and
  are strictly better off.

## [2.6.15] - 2026-08-03 — P3/P4 closeout: two O(n²) hot paths, doc drift, full module coverage

Final repair release of the 2.6.12 audit. Closes the **performance** and **documentation** tiers
and brings the last two never-tested modules into the suites — so every one of the 34 modules
shipped in `dist/hisab.cyr` is now compiled and asserted by `cyrius test`. Suite 1093 → **1127**;
`cyrius coverage` 57% → **59%**, files 32/35 → **34/35** (the 35th is `main.cyr`, the CLI smoke
binary, which by design does not include the library).

### Performance — measured, not asserted

Both hot paths were benchmarked **before** the rewrite, per CLAUDE.md. Two new benchmarks were
added to `tests/hisab.bcyr` first, and both the before and after rows are in `bench-history.csv`:

| Benchmark | Before | After | Change |
|---|---|---|---|
| `halfedge_2k_tris` (2,048 triangles) | **190.1 ms** | **1.2 ms** | **−99.4%** (~156×) |
| `convex_hull_2d_2k` (2,000 points) | **22.1 ms** | **2.1 ms** | **−90.3%** (~10×) |

- **`halfedge_from_triangles` twin pairing was O(n_he²)** — the inner loop scanned every later
  half-edge and never broke after a match, ~2.25·n_tris² iterations. A directed edge `(src → dst)`
  has exactly one possible twin, `(dst → src)`, so an open-addressed hash of directed edges
  replaces the scan with one lookup. Verified structurally identical to the old implementation:
  `twin(twin(i)) == i` for every paired edge and a boundary count of exactly `4(W−1)` on a grid
  mesh, with both implementations agreeing.
- **`convex_hull_2d`'s pre-sort was an insertion sort** — the only super-linear step, since the
  monotone chain itself is O(n), and uncapped because the public entry point bounds nothing.
  Replaced with heapsort: O(n log n) worst case and O(1) extra memory. Heapsort rather than merge
  sort deliberately — merge sort's n-slot scratch would be *leaked on every call* under the
  never-freeing bump allocator. The comparison is inlined rather than passed as a comparator
  because it must dereference a separate `points` vector and Cyrius has no closures; this is
  exactly why stdlib `vec_sort_by` (element-value comparator via `fncall2`) does not fit here.
  **Hull output is byte-identical** to the old sort at n = 50, 500, 2,000 and 4,000.
- **Levenberg-Marquardt** allocated `jtj`, `rhs`, `lu` and `piv` inside its iteration loop
  (5.35 MB of unreclaimed bump scratch over 200 iterations) while `j_buf` beside them was already
  hoisted; all four are now hoisted, which is safe because each is fully overwritten at the top of
  every iteration. It also computed the **symmetric** JᵀJ twice — the loop now runs the upper
  triangle only and mirrors, which is bit-identical since the k-accumulation order is unchanged
  and `f64_mul` is commutative. A 2-parameter line fit converges to the same `(a, b)` as before.
- **L-BFGS** allocated three n-length buffers per iteration, all dead by the end of it; hoisted.

No claim is made for LM/L-BFGS: they are memory reductions, and neither has a benchmark.

### Fixed — documentation that contradicted the code
- **`docs/architecture/overview.md`** credited **BVH to `spatial.cyr`**; all `bvh_*` functions are
  in `geo_advanced.cyr` (`spatial.cyr` contains zero occurrences of "bvh"). Also listed
  `f64_le`/`f64_ge` under `f64_util.cyr`, which explicitly documents them as *removed* in favour
  of stdlib `math`; and its Design Principles still named the bare `ERR_*` constants, namespaced
  to `HSB_ERR_*` back in 2.6.8.
- **`opt_conjugate_gradient`** was documented as Fletcher-Reeves in two places; it implements
  Polak-Ribière+ with a `max(0, β)` restart.
- **`num_dst` / `num_idst`** were labelled DST **Type II**; the kernel is
  `sin(π(i+1)(k+1)/(n+1))`, which is DST-**I** (FFTW RODFT00). Documentation only — the transform
  is a correct DST-I and `num_idst` inverts it, now pinned by a round-trip test. (`num_dct` was
  checked too and is genuinely DCT-II, so it was left alone.)
- **`expr_eval`** documented "Aborts if a variable is not found"; it writes a warning to fd 2 and
  returns 0.0. A test had been *skipped* on the strength of that false claim.
- **`hodge_star_2form_4d`** described its `sign` parameter three mutually contradictory ways —
  "+1 Euclidean, −1 Lorentzian", then "baked in; parameter kept for API consistency" (it is live),
  then "1 for Lorentzian, −1 for Euclidean". It is an overall multiplier on the Lorentzian dual;
  **no value of it produces a Euclidean dual**, whose sign pattern differs per component rather
  than globally. Corrected here and in `docs/architecture/math.md`, which propagated the claim.
- **`collision_core.cyr`**'s header named `collision.cyr`, a file that has not existed since the
  2.2.2 split, and advertised Delaunay, half-edge mesh and island detection — all three of which
  live in `collision_mesh.cyr`.

### Changed — final two modules brought into the suites
`num_ext` and `symbolic_ext` were the last of the eight modules included by no suite. `num_ext`
now has DST-I round-trip, DCT-II, Möbius and tridiagonal-solve assertions; `symbolic_ext` has
integration (differentiating the integral recovers the integrand), LaTeX rendering, and pattern
matching. The `sym_match` tests initially failed because `sym_match` takes a **Pattern**
(`PAT_*`, built with `pat_*`), not an `Expr` — passing an `Expr` silently returns 0. That is now
asserted explicitly, including that `pat_any_const` matches a constant but not a variable.

### Verified
- Suite **1127/1127** (foundation 307 + hisab 205 + edge_cases 199 + modules 416), up from 1093.
- **All 34 library modules included by a suite** — previously 8 were not compiled by `cyrius test`
  at all. `cyrius coverage` 335/587 → **348/587 (59%)**; files 32/35 → **34/35**.
- Benchmarks **28** (was 26); before/after rows for both hot paths in `bench-history.csv`.
- `cyrius fuzz` 1/1; `check-constants.sh` 110/110; `lint` / `fmt --check` / `vet` clean;
  `deps --verify` 30/30; `cyrius distlib` regenerated.

## [2.6.14] - 2026-08-03 — P1 closeout: memory safety and robustness

Third repair release of the 2.6.12 audit, closing the **memory-safety tier**. Three of the four
defect groups **crashed the process** before this release — reverting `complex.cyr`, `calc_ext.cyr`
or `optimize.cyr` individually makes the new suite exit **139 (SIGSEGV)** rather than merely fail.

Two more never-tested modules (`calc_ext`, `noise_simplex`) are now in the suites, leaving only
`num_ext` and `symbolic_ext`. Suite 1063 → **1093**; `cyrius coverage` 54% → **57%**, files 29/35
→ **32/35**.

### Fixed — capped constructors' null return was stored through (CWE-476)
`cmat_new` caps at `_CMAT_MAX_ELEMS` (65536) and correctly returns 0 above it. Four callers then
wrote through that 0, converting a designed error signal into a segfault. Each is reachable from
operands that are themselves *under* the cap:
- `cmat_mul` — `ar × bc` can exceed it when the operands cannot: 1000×1 times 1×1000 asks for 1e6.
- `cmat_kronecker` — multiplies *both* dimensions; 17×17 ⊗ 17×17 is 83521. (16×16 ⊗ 16×16 is
  exactly 65536 — at the cap, and legitimately allowed; the tests pin both sides of that edge.)
- `cmat_inverse` (`linalg_ext`) — builds `[M | I]`, doubling the width, so a 250×250 input under
  the cap produces a 250×500 augmented matrix over it.
- `cmat_identity` — propagates from a caller-supplied `n`.

### Fixed — `opt_bfgs` / `opt_lbfgs` sized working memory from an unbounded dimension
`opt_bfgs` allocates an `n × n` inverse Hessian and `opt_lbfgs` an `m × (2n+1)` history ring, with
no cap on `n` or `m`: `alloc` returned 0 and the initialisation loop stored through it (CWE-190 →
CWE-476). Added `_OPT_MAX_DIM = 4096` (the hard ceiling from `ALLOC_MAX` is 5792) plus explicit
allocation checks in all four solvers, and a new `HSB_ERR_ALLOC` (`-11`) so allocation failure is
reportable rather than fatal.

### Fixed — `calc_bspline` / `calc_nurbs` indexed control points negatively
Both validated `n_pts != 0` and the knot-count relation but never `degree` itself, nor the
well-formedness requirement that a degree-*d* spline needs at least *d+1* control points. With
`degree >= n_pts` the span search left `k = degree`, the sentinel-undo turned that into
`k - n_pts`, and de Boor indexed `ctrl_pts` at a **negative** offset — an out-of-bounds read
resolving to a wild-pointer dereference instead of the documented 0-on-invalid-input return.

### Fixed — adaptive Simpson bounded depth but not work
The recursion capped *depth* at 50, but every non-converged node spawns two children, so the cap
permits 2^50 (~1e15) integrand evaluations. A NaN integrand takes exactly that path: every f64
comparison involving NaN returns 0, so `|error| < tol` is false forever. Two bounds added — an
explicit non-finite bail (NaN is the only value comparing unequal to itself), and a
stop-when-bisection-stops-progressing check, which bounds real work by f64 resolution rather than
by the counter.

### Fixed — `kdtree_build` recursed to depth O(n) on coincident points
`_kd_partition` splits on the axis **value** midpoint. That separates properly whenever
`lo < hi` — the point attaining `lo` is below the midpoint, the one attaining `hi` is not — so it
can *only* degenerate when every point shares the axis value. That case fell to the
"ensure at least one item per side" clamp, which peels exactly **one** element per level. Measured
on the pre-fix code: fine at 40,000 coincident points, **SIGSEGV at 60,000**. Now splits
positionally in exactly that case, which is sound precisely because the values are equal — the
`split_val` the caller reads is then the common value, so query pruning compares exactly. Pinned
by both a 60,000-point build and a half-coincident tree asserting an exact nearest-neighbour hit.

### Fixed — NaN accepted by the Armijo line search
`_opt_armijo`'s tie-breaker was written `f64_lt(threshold, f_new) == 0`, which is also true when
`f_new` is NaN, so a NaN objective was accepted as a valid step and propagated into `x` for
`opt_bfgs`, `opt_lbfgs` and `opt_conjugate_gradient`. Now requires `f_new` to compare equal to
itself first.

### Fixed — complex zero-divisor cutoff was 1e-6, not 1e-12
`cx_div`, `cx_inv` and `cx_powf` compared a **squared** modulus against the **unsquared**
`EPSILON_F64`, so any divisor with |z| < 1e-6 was silently flushed to zero — six orders of
magnitude more aggressive than intended. `cx_is_zero` alongside them always squared its tolerance
correctly, which is the tell. Now `norm_sq < EPSILON_F64²`; a 1e-9 divisor inverts correctly and
an exact zero is still guarded.

### Fixed — `FLOAT_RENDER_BUF` was 14 bytes short of stdlib's scratch requirement
`fmt_int_buf` renders digits backwards from `buf + 23` before shifting them down, so it needs 24
bytes at whatever pointer it is handed — and `fmt_float_buf` calls it **twice**, the second time
at an offset that can reach 22 (sign + 20 whole digits + `.`), touching offset **45**. The buffer
was 32. Raised to 64. A large-magnitude symbolic coefficient overran the allocation.

### Fixed — four fBm entry points returned NaN for `octaves <= 0`
`fbm_2d`, `fbm_3d`, `simplex_fbm_2d` and `simplex_fbm_3d` all end in an unguarded
`f64_div(sum, max_amp)`, and `max_amp` only accumulates inside the octave loop — so it is exactly
0 when `octaves <= 0` and the functions returned NaN despite documenting a roughly `[-1, 1]`
range. NaN then propagated silently through every downstream sample. All four now return 0.

### Changed
- **`src/error.cyr`** — added `HSB_ERR_ALLOC = -11`.
- **`calc_ext` and `noise_simplex` brought into `tests/modules.tcyr`** — both shipped in
  `dist/hisab.cyr` while being included by no suite, so they were not even compiled by
  `cyrius test`.

### Verified
- Suite **1093/1093** (foundation 307 + hisab 205 + edge_cases 199 + modules 382), up from 1063.
  Mutation-proven per file against pre-fix source: reverting `complex.cyr` → **SIGSEGV**,
  `calc_ext.cyr` → **SIGSEGV**, `optimize.cyr` → **SIGSEGV**, `noise_simplex.cyr` → 2 failures,
  `spatial.cyr` → **SIGSEGV** at 60k points.
- `cyrius coverage` 320/587 → **335/587 (57%)**; files 29/35 → **32/35**.
- `cyrius fuzz` 1/1; `check-constants.sh` 110/110; `lint` / `fmt --check` / `vet` clean;
  `deps --verify` 30/30; `cyrius distlib` regenerated (17,021 → 17,159 lines).
- **No performance claim.** No benchmarked path changed. The kd-tree fix alters build *shape* on
  degenerate input — from O(n²) to O(n log n) — but `kdtree_build` has no benchmark, so the
  improvement is stated as a complexity argument and a 60k-point build that previously crashed,
  not as a measured number.

## [2.6.13] - 2026-08-03 — P0 closeout: SVD, eigensolver, SE(3) and interval soundness

Second repair release of the 2.6.12 audit. Closes the **remaining five P0 findings**, every one
of them in a module that had **zero test coverage** — which is precisely why they shipped. Four
of the eight never-tested modules are now in the suites (`einsum`, `lie_ext`, `mat3`,
`linalg_precision`), `cyrius coverage` moves 50% → **54%**, and the suite grows 981 → **1063**.

No API or signature changes; consumers rebuild only. But every consumer of `svd_golub_kahan`,
`eigen_qr`, `se3_exp`/`se3_log`, `su2_exp`/`su2_log`, `einsum`, `ivl_sin`, `ivl_sqrt` or
`ivl_div` was getting wrong answers — in three cases with no error signalled at all.

### Fixed — `svd_golub_kahan` returned U transposed, so `A != U·S·Vᵀ`
`_lp_bidiagonalize` accumulated the left Householder reflectors in **forward** order, building
`H_{n-1}···H_0` where the decomposition needs `U = H_0···H_{n-1}`. Rewritten to store the
reflectors and accumulate **backward** after the sweep. `S` and `Vᵀ` were always correct — `Vᵀ`
is literally `G_{n-1}···G_0`, so forward order happens to be right there, which is exactly why
the error was one-sided and invisible. Verified by reconstructing `U·S·Vᵀ` entrywise: pre-fix,
**8 of 9 entries wrong** on both a symmetric and a non-symmetric 3×3.

### Fixed — `eigen_qr` never converged for n ≥ 3
`_lp_tridiag_qr` applied the similarity `Gᵀ·T·G` where `G = [[cs, sn], [-sn, cs]]` is the
rotation that *zeroes* the bulge (`G·[x;z] = [r;0]`). The matching similarity is `G·T·Gᵀ`; only
the signs of the cross terms differ, which is why it read as correct. The off-diagonal therefore
never decayed: measured, a plain symmetric 3×3 returned `HSB_ERR_NO_CONVERGENCE` **even at
max_iter = 100000**, and at n = 2 it paired each eigenvalue with the wrong eigenvector. The `Q`
accumulation (`Q ← Q·Gᵀ`) was already the correct companion to the *fixed* update and needed no
change — the two were inconsistent, not `Q` alone. A dead first attempt at the update, whose
stores were immediately overwritten, was removed. Now converges and matches reference
eigenvalues to 1e-8, with `A·v = λ·v` asserted for every returned pair at n = 2, 3 and 4.

### Fixed — SU(2)/SE(3) half-vs-full-angle convention split
`su2_exp` returned `cos(|ω|) + sin(|ω|)/|ω| · ω`, the generator convention denoting a rotation by
**2|ω|**, while `so3_exp`, `su2_from_axis_angle` and `su2_to_rotation_matrix` all use half-angle.
`se3_exp` built its translation block `V` from `theta = |ω|` but took `R` from `su2_exp`, so `R`
and `t` came from angles a factor of two apart — the resulting `(R, t)` was **not the exponential
of any twist**. `su2_exp` and `su2_log` moved to half-angle, restoring the double-cover identity
`su2_to_rotation_matrix(su2_exp(ω)) == so3_exp(ω)`, which is now asserted entrywise along with
`su2_log ∘ su2_exp = id` and the `se3_log ∘ se3_exp` round-trip.

### Fixed — `einsum` silently mis-computed its own documented examples
`_einsum_is_label` accepted only `a`–`h`, so `i`/`j`/`k` — used in **every** example in
`einsum.cyr`'s own header — were rejected; and the parser **skipped** unrecognised characters
rather than erroring. `einsum("ij,jk->ik", …)` therefore parsed to zero labels and returned the
*trace* (5, not 19) before **segfaulting**. Alphabet widened to `a`–`z`, and the parser now
rejects: unknown characters, specs longer than `_EINSUM_MAX_RANK`, and any notation rank that
disagrees with the tensor's actual `HTensor_rank` (the out-of-bounds shape read). `tensor_new`'s
documented 0-on-failure return is now checked at both sites. Spaces are tolerated.

### Fixed — three interval enclosure-soundness violations
The one contract an interval type must never break is that the result **contains** the true range.
- `ivl_sin` returned `[min(sin a, sin b), max(sin a, sin b)]` for any width ≤ π, which
  **under-approximates** whenever the interval straddles an extremum without an endpoint at it:
  `ivl_sin([1.4, 1.8])` excluded 1.0 although sin peaks at π/2 inside. Extrema are now located
  exactly (`ceil(k_lo) <= floor(k_hi)` on `π/2 + kπ`), which is both sound *and* tighter than the
  old `width > π → [-1,1]` shortcut: `[0.1, 3.3]` contains a maximum but no minimum and now
  yields `[sin(3.3), 1]` instead of all of `[-1, 1]`.
- `ivl_sqrt` clamped only the lower bound, so a wholly-negative interval produced `[0, NaN]` —
  and since `ivl_new` cannot order around NaN, `ivl_contains` then reported that interval as
  containing **everything**. Both ends are now clamped.
- `ivl_div`'s zero guard used a raw integer `== 0` on an f64 bit pattern, matching only `+0.0`;
  `-0.0` (`0x8000000000000000`) slipped through and divided by zero. Now uses `f64_le`/`f64_ge`.

### Changed — four more modules brought into the suites
`einsum`, `lie_ext`, `mat3` and `linalg_precision` had **no test suite including them** — they
were not even compiled by `cyrius test` — while all four shipped in `dist/hisab.cyr`. Each now
has coverage asserting its defining identities. Remaining untested: `calc_ext`, `noise_simplex`,
`num_ext`, `symbolic_ext`.

### Verified
- Suite **1063/1063** (foundation 307 + hisab 200 + edge_cases 187 + modules 369), up from 981.
  Every fix is mutation-proven against the pre-fix source: reverting `linalg_precision.cyr` fails
  10, `lie.cyr` fails 24, `interval.cyr` fails 6, `einsum.cyr` fails 2 **and then segfaults**.
- `cyrius coverage` 297/587 → **320/587 (54%)**; files referenced 25/35 → **29/35**.
- `cyrius fuzz` 1/1; `check-constants.sh` 110/110; `lint` / `fmt --check` / `vet` clean;
  `deps --verify` 30/30; `cyrius distlib` regenerated (16,897 → 17,021 lines).
- **No performance claim.** No benchmarked path changed; `_lp_bidiagonalize` now allocates an
  `m*n` scratch buffer for the reflectors, which is a memory cost, not a speed one, and SVD has
  no benchmark to measure against.

## [2.6.12] - 2026-08-03 — Audit sweep: seven mis-transcribed constant tables repaired

**A repair release, not a feature release.** A full P(-1) audit of the 2.6.11 tree — 8 parallel
dimensions over all 34 modules, every finding adversarially re-verified against running code —
produced **70 confirmed findings (2 critical, 23 high, 23 medium, 22 low)** across 64 sites.
Full report: [`docs/audit/2026-08-03.md`](docs/audit/2026-08-03.md). This release closes the
critical tier plus the two highest-value independent defects; the rest is tracked under
`[Unreleased]` and roadmap §2.6.12.

**No API change. No signature change. Consumers need only rebuild** — but every consumer of
`ode_dopri45`, `ode_bdf(order=4)`, `ode_yoshida4`, `calc_integral_gauss5`, `num_is_prime`,
`solve_bicgstab`, the sRGB conversions, the spherical-harmonic basis or `simplex_*` was getting
**numerically wrong answers** before this release.

### Fixed — seven hand-encoded constant tables did not encode their documented values

hisab stores f64 constants as raw i64 bit patterns (Cyrius has no float literals), so the
adjacent comment is the only readable statement of intent. In seven tables the bits disagreed
with the comment. **47 constants re-derived from exact rationals and closed forms**, each table
now pinned by its own mathematical invariant:

| Table | Site | Invariant | Was | Now |
|---|---|---|---|---|
| Dormand-Prince RK4(5) | `ode.cyr` | Σb = 1 | **0.635581152612211** | 1.0 |
| Gauss-Legendre 5-point | `calc.cyr` | Σw = 2 | **2.000492320194892** | 2.0 |
| BDF-4 | `ode.cyr` | Σα = 1 | **1.01** | 1.0 |
| Yoshida-4 symplectic | `ode.cyr` | 2w₁+w₀ = 1 | **1.4737899450113117** | 1.0 |

- **DOPRI45 — 23 of 30 tableau constants were wrong.** Stage rows also failed `Σⱼ a_ij = c_i`
  (row 3 summed to 0.25 against c₃ = 0.3). It was **not a consistent integrator at any order**:
  it did not merely lose accuracy, it converged to the wrong solution. One step of `y' = y` from
  `y(0) = 1` at `dt = 0.1` now matches e^0.1 to < 1e-9; before, it was wrong in the second digit.
- **BDF-4** drifted 1% per step on a constant solution. **Yoshida-4** advanced a free particle
  only 85.9% of the correct distance. **Gauss-Legendre-5** carried a 246 ppm relative error
  floor on a rule that is exact for polynomials of degree ≤ 9.
- **`color.cyr`** — both sRGB piecewise breakpoints (0.03275 shipped for the IEC 61966-2-1 value
  **0.04045**; 0.0031294 for 0.0031308) and four of five spherical-harmonic normalisations.
- **`noise_simplex.cyr`** — `_SIMPLEX_G2` encoded √3 as 1.73158 instead of 1.73205, so the
  skew/unskew pair was not an exact inverse.
- **`quat.cyr`** — the slerp/nlerp switchover threshold was 0.999375, not the documented 0.9995.

### Fixed — `num_is_prime` reported real primes as composite above 3.03e9
`_num_miller_rabin_test`'s squaring step used a raw `(x * x) % n` while `num_modpow` alongside it
already used the overflow-safe `_num_mulmod`. `x` ranges over `[0, n-1]`, so once
`n > 3037000499` (≈√2^63) the product wrapped negative and every witness failed. Measured: the
genuine primes **3037218841** and **4294967357** both returned composite. Now routed through
`_num_mulmod`; verified against an independent Miller-Rabin implementation across the band.

### Fixed — `solve_bicgstab` never iterated
`var breakdown_tol = f64_from(0x3C32725DD1D243AC);` — `f64_from` **converts** an integer to a
double, it does not reinterpret bits, so the tolerance was **4.34e18** instead of 1e-18 and the
first `|rho| < tol` check fired immediately. The solver returned the initial guess verbatim on
every call. The literal already is the bit pattern; the `f64_from` wrapper is removed. (The
comment also mis-stated the value as ~1e-30; it is 1e-18.) This was the only `f64_from(0x…)` in
`src/`.

### Added — `scripts/check-constants.sh`, a CI gate for the whole defect class
Decodes **every** hand-encoded f64 literal in `src/` and asserts it against the value in its own
comment, handling exact rationals, closed-form expressions (`1/(2 - 2^(1/3))`, `sqrt`, `pi`) and
truncated decimals, with the tolerance derived from the precision the comment itself claims. It
also cross-checks comments that state both an exact form and a decimal — which caught four
DOPRI45 comments whose decimals disagreed with their own fractions. **110/110 now verified.**
Wired into CI between the vet and distlib gates. The class is mechanical; the guard is too.

### Changed — assertions that a broken implementation could pass
Every fix landed with the assertion that would have caught it. The pre-existing tests were the
reason six releases shipped a broken integrator:
- `ode_dopri45` asserted only `1 < y < 2` for a value whose exact answer is 1.10517 — replaced
  with a value assertion plus a check that the embedded error estimate is not garbage.
- `gauss5` compared `f64_round(result)` to an integer, tolerating ±0.5 absolute (7.1% relative
  on the const-7 case) — replaced with 1e-11 tolerances and a weights-sum check.
- **BDF-4 and Yoshida-4 had no numeric assertions at all** — now pinned by the consistency
  invariants their wrong constants violated.
- The sRGB test was a **round-trip**, which is structurally blind to a wrong transfer curve (the
  same wrong breakpoint applies in both directions and the error cancels) — added an absolute
  value assertion and a breakpoint-segment check.
- `solve_bicgstab` and the `num_is_prime` overflow band had no coverage at all.

### Verified
- Suite **981/981** (foundation 307 + hisab 186 + edge_cases 173 + modules 315), up from 961:
  +20 assertions, all mutation-proven. Reverting `src/ode.cyr` alone fails 4; reverting
  `src/calc.cyr` fails 3; reverting `src/linalg_ext.cyr` fails 3; reverting `src/num.cyr` fails 4.
- `cyrius fuzz` 1/1. `check-constants.sh` 110/110. `cyrius lint` / `fmt --check` / `vet` clean;
  `deps --verify` 30/30; `cyrius distlib` regenerated (16,885 → 16,897 lines).
- Benchmarks re-run (26, commit `ee8032c`). **No performance claim** — `num_is_prime` measured
  20.3 µs against 20.7 µs before, which is run-to-run noise, and no other changed path is
  benchmarked. The `_num_mulmod` routing is strictly more work per squaring step; it was not
  measurably slower at the benchmark's input sizes, and correctness is not negotiable regardless.

## [2.6.11] - 2026-08-03 — Cyrius 6.5.6 toolchain bump + sakshi 2.4.7; stdlib `mat_new` CWE-190 guard lands

Maintenance release: toolchain pin **6.4.69 → 6.5.6** (a **minor** jump across 24 releases —
6.4.70–6.4.86 plus 6.5.0–6.5.6) and first-party dep **sakshi 2.4.6 → 2.4.7**. **No executable
math-module change** — all 34 modules compile clean on the new pin, and `dist/hisab.cyr` differs
only by its version header and one rewritten `mat_new_guarded` doc comment (verified: the bundle
diff contains **zero** non-comment lines, 16,878 → 16,885 lines). The bundle's stdlib leaf requirements are
unchanged (`syscalls` + `io`, verified identical when regenerated under *both* pins), so
consumers need no `[deps]` edits; bumping their own pin to 6.5.6 is recommended for parity.

Not a routine bump, though. Two latent defects were found and closed, and neither was in the
math: hisab had been building against a **stale vendored `ganita`** whose `mat_new` segfaults
on a negative dimension, and all four test suites could **score PASS with 256 failing
assertions**. Both are fixed and both are now pinned by tests.

### Security
- **Stdlib `mat_new` CWE-190 guard (ganita 1.0.3 → 1.0.4).** `ganita_mat_new` now rejects
  non-positive dimensions and element counts above `GANITA_MAT_MAX_ELEMS = 33_554_430` (the
  `ALLOC_MAX`-derived ceiling), returning 0; `ganita_mat_identity` and `ganita_mat_from_array`
  propagate the null. **This is a live hisab code path** — `mat_new` / `mat_mul` / `mat_lu` are
  thin `ganita_*` aliases (`lib/ganita.cyr:1329+`) used from `optimize.cyr` and `linalg_ext.cyr`.
  Measured with the compiler held constant and only `ganita.cyr` swapped:

  | vendored ganita | `mat_new(-5, 3)` |
  |---|---|
  | 1.0.3 (shipped through 2.6.10) | **SIGSEGV — exit 139** |
  | 1.0.4 (this release) | returns null, exit 0 |

  **The exposure was a vendoring miss, not a pin miss, and it was worse than the threat model
  recorded.** The repo's `lib/ganita.cyr` sat at **1.0.3 while the 6.4.69 pin already shipped
  1.0.4** — so hisab spent the entire 2.6.10 cycle on the unguarded copy. Every prior audit
  entry (2026-06-15 through 2026-07-21) called this item "unguarded upstream, mitigated" because
  the vendoring check compared *toolchain snapshot against toolchain snapshot*, which cannot see
  drift in the committed `lib/`. The check is now `lib/*.cyr` byte-compared against
  `~/.cyrius/versions/<pin>/lib/`, which is what surfaced it. Closes the roadmap's tracked
  "stdlib `mat_new` overflow guard" item (open since 2.5.3).
- **Upstream contract now pinned by tests.** `tests/edge_cases.tcyr` gained 4 assertions on
  stdlib `mat_new` itself (negative / zero / overflow-prone / valid dims). Previously only
  hisab's own `mat_new_guarded` wrapper was asserted, which is exactly why the stale vendoring
  was invisible. Mutation-proven: reverting `lib/ganita.cyr` to 1.0.3 crashes the suite with
  **exit 139** instead of passing.

### Fixed
- **Test suites could report PASS with 256 failing assertions.** All four `tests/*.tcyr` ended
  at a bare `var r = assert_summary();`, and `assert_summary()` returns the raw failure *count*.
  A POSIX wait status is 8 bits, so exactly 256 / 512 / 768 failures truncated to 0 and the
  `cyrius test` gate scored **PASS**. Demonstrated on hisab's own epilogue before fixing, then
  re-measured after:

  | epilogue | 256 failures | 1 failure | 0 failures |
  |---|---|---|---|
  | unclamped (through 2.6.10) | **exit 0 — PASS** | exit 1 | exit 0 |
  | clamped (this release) | **exit 1 — FAIL** | exit 1 | exit 0 |

  All four suites now clamp (`if (r > 0) { r = 1; }`) before exiting. This was a CI-gate
  integrity hole, not a library defect — no shipped math behaviour changes. cyrius 6.5.6 fixed
  the identical hole in its own `proj-tcyr` scaffold template.
- **`cyrius fuzz` never discovered `tests/hisab.fcyr`.** Through 6.5.5 the verb walked only
  `fuzz/`, a directory this repo does not have, so the fuzz gate had been silently exercising
  **zero** harnesses (it exits 0 when it finds nothing). 6.5.6 walks `tests/` as well; the
  harness now actually runs, and passes (**1/1**) on its first real execution.

### Changed
- **Toolchain pin `6.4.69` → `6.5.6`; sakshi `2.4.6` → `2.4.7`.** Re-vendored `lib/` via
  `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.5.6; the transitive
  `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed, as at
  6.4.66/6.4.69). `cyrius.lock` 30 deps (1 commit-pinned), `deps --verify` 30/30. Smoke version
  string `src/main.cyr` 2.6.10 → 2.6.11.
- **Vendored stdlib delta — six files.** `ganita.cyr` (the guard above); `vec.cyr` (6.5.4 added
  `vec_sort_by` introsort + `vec_select_nth` quickselect, comparator via `fncall2` — purely
  additive, **not adopted**, see below); `syscalls_linux_common.cyr` (+`sys_exit_group`);
  `syscalls_windows.cyr` / `syscalls_x86_64_agnos.cyr` (peer `sys_exit_group` definitions plus
  agnos GPU present/fill wrappers — non-Linux-x86_64, vendored for parity); and `io.cyr`
  (upstream changes that leave hisab's `println` path behaviour unchanged).
- **`sys_exit_group` epilogue.** `sys_exit` is `exit(2)` and ends only the *calling thread*.
  `src/main.cyr`, `examples/basic_math.cyr`, `tests/hisab.bcyr`, `tests/hisab.fcyr` and all four
  `tests/*.tcyr` migrated off `syscall(SYS_EXIT, r)` / `syscall(60, r)`. hisab is single-threaded
  so either form terminated correctly; the group form is correct by construction. CLAUDE.md's
  program-epilogue principle updated to match.
- **sakshi `2.4.6` → `2.4.7`** — a latent-bug fix with **no public API change**: two private
  helpers renamed (`_sk_write_int` → `_sk_buf_write_number`, `_sk_write_str` →
  `_sk_buf_write_string`) because `_int` / `_str` / `_cstr` are reserved cyrius overload-dispatch
  suffixes, so `_sk_write_int` was squatting the dispatch slot of the unrelated `_sk_write(buf,
  len)`. Both symbols are `_sk_`-private; hisab links nothing that moved. **Audited hisab for the
  same defect class**: `src/` defines exactly one function with a reserved suffix (`expr_to_str`)
  and there is no `expr_to` base for it to shadow — clean.
- **`src/linalg_ext.cyr`** — `mat_new_guarded`'s doc comment no longer claims stdlib `mat_new` is
  unguarded. The wrapper is **kept, not retired**: its 16,777,216-element cap is deliberately
  stricter than upstream's 33,554,430, so it remains the entry point for untrusted dimensions.
  `tests/hisab.tcyr`'s comment corrected to match (its over-cap assertion at 25M elements is
  precisely the band where hisab rejects and upstream accepts).

### Verified
- `cyrius build` + smoke (prints `hisab 2.6.11`), `cyrius test tests/*.tcyr` **961/961** —
  foundation 307 + hisab 175 + edge_cases **167** (+4, the new upstream `mat_new` assertions) +
  modules 312, 4 suites. `cyrius fuzz` **1/1**. Benchmarks re-run via `./scripts/bench-history.sh`
  (26 benchmarks, commit `5331f05`); **no performance claim is made** — this release changes no
  math code path, and the run-to-run spread against the June baseline is not attributable to the
  bump. Cleanliness gates green: `cyrius lint` (all `src`/`examples`/`tests` globs,
  warnings-as-errors), `cyrius fmt --check`, `cyrius vet src/main.cyr` (2 deps, 0 untrusted,
  0 missing). `cyrius distlib` regenerated `dist/hisab.cyr` (header-only diff, 2.6.10 → 2.6.11);
  `cyrius deps --verify` 30/30.
- **Consumer end-to-end** against the regenerated bundle on 6.5.6: `hvec3_length`, `hquat_length`,
  stdlib `mat_new` (valid + both guard paths) and `mat_new_guarded` — 7/7, exit 0.
- Tracked toolchain issues re-verified on 6.5.6 (minimal repros): interval-ident-lex **still
  live** (`var iv_add` → `expected identifier, got unknown`; the reserved-SIMD-name rename stays
  in `tests/modules.tcyr`); for-empty-clauses **still live** (`for (; …)` → `unexpected ';'`);
  cli-arg-clobber not re-tested (destructive). No tracked issue newly fixed by this bump.

### Not adopted (recorded, with reason)
- **`vec_sort_by` / `vec_select_nth`** (new in 6.5.4). hisab's one hand-rolled sort —
  `_col_sort_indices_by_xy` (`src/collision_core.cyr:339`, single caller at `:393`, the
  `convex_hull_2d` pre-sort) — **does not fit the API**: `vec_sort_by` passes element *values* to
  the comparator (`fncall2(cmp, a, b)`), while hisab sorts *indices* by dereferencing into a
  separate `points` vector, and Cyrius has no closures. Deferred on the API mismatch, CLAUDE.md's
  "wait for the third instance" rule (this is the first), and because swapping a hot-path sort is
  a performance change that needs its own before/after numbers rather than riding a maintenance
  bump. `vec_select_nth` has no call site in hisab today.

## [2.6.10] - 2026-07-21 — Cyrius 6.4.69 toolchain bump

Maintenance release: toolchain pin **6.4.66 → 6.4.69** (3 patch releases). **No library
source change** — all 34 math modules compile clean on the new pin; the `dist/hisab.cyr`
bundle is byte-identical apart from its version header, and the bundle's stdlib leaf
requirements are unchanged (`ganita` + `math`), so consumers need no `[deps]` edits (bumping
their own pin to 6.4.69 is recommended for parity). First-party dep **sakshi stays at 2.4.6**
— already the latest tag, so no dep change this release. The vendored stdlib picks up three
upstream 6.4.67–6.4.69 fixes; only one is on a code path hisab actually links (`fmt`, via
`fmt_float_buf` in the `symbolic` modules), while the `math` float-parse and agnos `sys_reboot`
fixes are vendored for snapshot parity but sit on paths hisab never exercises.

### Changed
- **Toolchain pin `6.4.66` → `6.4.69`.** Re-vendored `lib/` from the new pin via
  `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.4.69; the transitive
  `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed, as at
  6.4.66). `cyrius.lock` 30 deps (1 commit-pinned), `deps --verify` 30/30. Smoke version
  string `src/main.cyr` 2.6.9 → 2.6.10.
- **Vendored stdlib delta — three files, all upstream bug/hardening fixes:**
  - `fmt.cyr` — `fmt_hex` / `fmt_sprintf %x` loop guard `> 0` → `!= 0`, so high-bit-set
    values (negatives / u64 ≥ 2^63) render every digit instead of zero; and `fmt_float_buf`
    gained a non-finite guard emitting `inf` / `-inf` / `nan` instead of the garbage
    `-.00000-` token for Inf/NaN/out-of-i64-range inputs. **hisab links this** —
    `symbolic.cyr` + `symbolic_ext.cyr` render coefficients via `fmt_float_buf`; for every
    finite in-range value the output is byte-identical, so this is a pure correctness gain on
    the non-finite edge (suite stays 957/957).
  - `math.cyr` — the float-parse exponent accumulator now saturates at 340 (10^340 already
    overflows f64 to ±Inf), turning an O(exp_val) apply loop into O(1) and closing an
    algorithmic-complexity DoS (`"1e100000000"` burned ~237 ms upstream). Behaviour-preserving
    for every representable value.
  - `syscalls_x86_64_agnos.cyr` — `sys_reboot()` widened nullary → 4-arg
    `power_sys(magic1, magic2, cmd, arg)` (agnos 1.55.25 power-control). Agnos-only platform
    variant; hisab builds and tests the x86_64-linux target, so no behavioural impact —
    vendored for snapshot parity.

### Verified
- `cyrius build` + smoke (prints `hisab 2.6.10`), `cyrius test tests/*.tcyr` (**957/957** —
  foundation 307 + hisab 175 + edge_cases 163 + modules 312, 4 suites), fuzz harness clean,
  benchmarks nominal. Cleanliness gates green: `cyrius lint` (all `src`/`examples`/`tests`
  globs, warnings-as-errors), `cyrius fmt --check`, `cyrius vet src/main.cyr` (2 deps, 0
  untrusted, 0 missing). `cyrius distlib` regenerated `dist/hisab.cyr` (header-only diff,
  2.6.9 → 2.6.10); `cyrius deps --verify` 30/30.
- Tracked toolchain issues re-verified on 6.4.69 (minimal repros): interval-ident-lex
  **still live** (`var iv_add` → `expected identifier, got unknown` — the reserved-SIMD-name
  shadow is still worked around in `tests/modules.tcyr`); for-empty-clauses **still live**
  (`for (; …)` → `unexpected ';'`); cli-arg-clobber not re-tested (destructive). No tracked
  issue newly fixed by this bump.

## [2.6.9] - 2026-07-17 — Cyrius 6.4.66 toolchain bump + sakshi 2.4.6; modules.tcyr compile fix

Maintenance release: toolchain pin **6.3.11 → 6.4.66** (a full minor across 55 patch
releases) and first-party dep **sakshi 2.4.2 → 2.4.6**. **No library source change** — all
34 math modules compile clean on the new pin; the `dist/hisab.cyr` bundle is byte-identical
apart from its version header, and a consumer that includes the full bundle compiles and runs
end-to-end on 6.4.66. Not breaking — the bundle's stdlib leaf requirements are unchanged from
2.6.8 (`ganita` + `math`); consumers need no `[deps]` edits (bumping their own pin to 6.4.66
is recommended for parity).

### Changed
- **Toolchain pin `6.3.11` → `6.4.66`.** Re-vendored `lib/` from the new pin via
  `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.4.66; the transitive
  `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed this
  bump). `cyrius.lock` 30 deps (1 commit-pinned), `deps --verify` 30/30. Smoke version string
  `src/main.cyr` 2.6.7 → 2.6.9 (was stale from 2.6.8).
- **sakshi `2.4.2` → `2.4.6`** (git, `dist/sakshi.cyr`). 2.4.6 pins cyrius 6.4.49 (≤ 6.4.66);
  its shipped surface links only `fnptr` + `atomic` and is unchanged, so logging behaviour is
  identical.

### Fixed
- **tests/modules.tcyr — the suite could not compile** (a pre-existing failure, unrelated to
  the toolchain bump: it reproduces on cycc 6.3.11–6.4.66). The interval-arithmetic section
  named its result vars `iv_add` / `iv_sub` / `iv_mul` — which are **reserved cycc SIMD
  intrinsic names** (packed integer-vector add/sub/mul; the lexer tokenizes them as builtins,
  see the toolchain's `lib/simd.cyr`). Using one as a variable name fails with a misleading
  `expected identifier, got unknown` at `var iv_add = ...`, which then desyncs the parser and
  took the whole file down (deterministic, 8/8). Renamed to `iv_sum` / `iv_diff` / `iv_prod`
  (`iv_div` / `iv_neg` / `iv_abs` are unaffected — there is no integer-vector divide, so
  `iv_div` was never reserved). The suite now compiles stably and runs **312/312**, restoring
  the full **957/957** across all four suites. Filed the toolchain bug at
  `docs/development/issues/2026-07-17-cyrius-interval-ident-lex.md` (and upstream in the cyrius
  repo); a `NOTE:` comment in the test guards the names against a well-meaning rename-back.

### Verified
- `cyrius build` + smoke (prints `hisab 2.6.9`), `cyrius tests tests/` (**957/957**, 4 suites,
  0 failed), `cyrius bench` (all metrics nominal), `cyrius lint` / `cyrius fmt --check` /
  `cyrius vet` (clean), `cyrius distlib` (regenerated, v2.6.9, 16,878 lines) — all green on
  6.4.66. Tracked-issue re-verify on the new pin: for-empty-clauses **still open**
  (unchanged); the new interval-ident-lex issue filed and worked around.

## [2.6.8] - 2026-07-06 — collision hardening for co-compilation with the sandhi/TLS stack

Two consumer-facing hardening fixes surfaced when a consumer (prakash) opts hisab
in alongside the `sandhi` HTTP/TLS stdlib stack:

### Fixed
- **symbolic** — the two float-render scratch buffers switched from a stack array
  sized by an enum constant (`var buf[FLOAT_RENDER_BUF]`) to `alloc(FLOAT_RENDER_BUF)`.
  When co-compiled with the `tls`/`dynlib` stdlib modules (which relocate oversized
  locals to shared globals), the enum-sized stack-array form tripped the compiler's
  "array size identifier must be an enum constant" path; the heap form the compiler
  itself recommends sidesteps it. No behavioral change — same 32-byte buffer, same
  `fmt_float_buf` render.

### Changed
- **error** — bare error constants namespaced `ERR_*` → `HSB_ERR_*`
  (`HSB_ERR_NONE`, `HSB_ERR_INVALID_INPUT`, `HSB_ERR_NO_CONVERGENCE`, …). hisab's
  bare `ERR_NONE` previously imposed a last-wins global collision on every consumer
  (prakash namespaced its own errors `PK_ERR_*` to work around it); moving the
  namespace into hisab fixes it at the root. **Values are unchanged** (`HSB_ERR_NONE`
  is still 0); consumers that check numeric return values (`== 0`) are unaffected —
  only name-based references need the `HSB_` prefix.


## [2.6.7] - 2026-06-30 — Cyrius 6.3.11 toolchain bump + sakshi 2.4.2

Maintenance release: toolchain pin **6.2.11 → 6.3.11** and first-party dep
**sakshi 2.1.0 → 2.4.2**. Pure infrastructure — **no library source change**.
All 34 math modules compile clean on the new pin; the `dist/hisab.cyr` bundle is
byte-identical apart from its version header. 957/957 tests pass; every
cleanliness gate (lint/fmt/vet), the fuzz harness, and all 26 benchmarks are
green on 6.3.11. **Not breaking** — the bundle's stdlib leaf requirements are
unchanged from 2.6.6 (`ganita` + `math`); consumers need no `[deps]` edits,
though bumping their own toolchain pin to 6.3.11 is recommended for parity.

### Changed
- **`cyrius.cyml`** — toolchain pin `6.2.11` → `6.3.11`; `[deps.sakshi]` tag
  `2.1.0` → `2.4.2`.
- **Vendored stdlib** — `lib/*.cyr` re-resolved to the 6.3.11 toolchain. Note the
  6.3.x CLI split: `cyrius deps` now resolves **git deps only** (it commit-pins
  sakshi in `cyrius.lock`), while **`cyrius lib sync`** (no `--full`) vendors the
  declared `[deps] stdlib` subset — superseding the 6.2.x-era `cyrius deps`-does-
  both flow. Stdlib files that actually changed on the bump: `assert`, `bench`,
  `fnptr`, `io`, `math`, and the `syscalls` platform variants
  (`aarch64_linux`/`linux_common`/`windows`/`x86_64_agnos`/`x86_64_linux`).
  `ganita` (the math umbrella) is unchanged. Every vendored stdlib file now
  byte-matches the 6.3.11 toolchain; `cyrius.lock` is 30 deps (1 commit-pinned),
  `cyrius deps --verify` 30/30.
- **`src/main.cyr`** — CLI smoke version string `hisab 2.3.3` → `hisab 2.6.7`
  (had been stale since 2.3.3; Cyrius source can't interpolate `VERSION`, so the
  string is hand-maintained — it had drifted across the 2.4.x/2.5.x/2.6.x bumps).
- **`dist/hisab.cyr`** — regenerated via `cyrius distlib` (header now v2.6.7;
  bundle body unchanged, 16,878 lines).
- **`.gitignore`** — ignore `/dist/hisab.deps`, the leaf-requirements companion
  6.3.x `cyrius distlib` emits alongside the tracked `dist/hisab.cyr`.

### Fixed
- **`lib/result.cyr`** (vendored stdlib, transitive dep of `io`/`tagged`) — picked
  up the 6.3.11 `_die` portability fix: the unwrap-on-`Err` abort was a bare
  `syscall(60, 1)` that no-op'd on agnos (an undefined syscall number → `-1`), so
  unwrap-on-`Err` **failed open** and continued with garbage; it is now a
  target-guarded abort (`#60` on backed targets, `exit` on agnos). No effect on
  the Linux x86_64 build path, but vendoring a 6.2.11 stdlib file under a 6.3.11
  pin was wrong. `cyrius lib sync` skipped it (transitive include not in the
  declared subset); completed from the authoritative `6.3.11/lib`. `atomic.cyr`
  (transitive dep of `alloc`) was already byte-identical to 6.3.11.

### Security
- **Tracked toolchain issues re-verified on 6.3.11** (doc-health mandate at each
  pin bump). The 2 still-open filings carry forward unchanged: the C-style
  `for (;cond;)` empty-clause parse error **still reproduces** (confirmed on
  6.3.11 — workaround remains `while`); the destructive CLI unknown-flag-clobbers-
  source issue was not re-tested. No regressions; the 3 issues fixed at the
  6.2.11 bump remain archived.

## [2.6.6] - 2026-06-15 — Cyrius 6.2.11 toolchain bump + stdlib math reorg

Toolchain upgrade from **6.0.14 → 6.2.11**. The 6.2.x stdlib reorganised its
math surface: the transcendental functions moved out of `math` into a new
**`ganita`** umbrella module, and `ganita` now subsumes the `matrix` and
`linalg` modules entirely (re-exporting their full public API). The library
source needed two adjustments; all 957 tests pass and every cleanliness gate
is green on the new pin.

### Breaking
- **Consumer dependency change.** Consumers pulling `dist/hisab.cyr` must update
  their `[deps] stdlib` list: **add `"ganita"`** (the bundle now references
  `f64_acos`/`f64_pow`/`f64_atan2`/`f64_cosh`/`f64_sinh` and the hyperbolic
  inverses, which live in `ganita`) and **remove `"matrix"` and `"linalg"`**
  (subsumed by `ganita` — keeping them alongside `ganita` triggers
  duplicate-definition collisions). Keep `"math"` — it still uniquely provides
  `f64_le`/`f64_ge`/`f64_clamp`/`f64_lerp`/`f64_min`/`f64_max`/`f64_sign`/
  `f64_trunc`/`f64_parse`/`gcd`/`lcm` and the exp/ln polyfills.
  Migration: `["…", "math", "matrix", "linalg", …]` → `["…", "math", "ganita", …]`.

### Changed
- **`cyrius.cyml`** — toolchain pin `6.0.14` → `6.2.11`; `[deps] stdlib` adds
  `ganita`, drops `matrix` and `linalg` (now provided by `ganita`).
- **`src/f64_util.cyr`** — removed the local `f64_le`/`f64_ge` definitions. The
  stdlib `math` module now ships them (NaN-correct: a NaN operand yields 0 for
  all of `f64_lt`/`f64_gt`/`f64_eq`, so the inclusive forms correctly return 0),
  superseding hisab's negation-based versions. `f64_tan`/`f64_fmod`/
  `f64_copysign`/`f64_approx_eq` stay local (still absent from stdlib).
- **Vendored stdlib** — `lib/*.cyr` re-resolved to the 6.2.11 toolchain via
  `cyrius deps` (the manifest-driven resolver, per CLAUDE.md — **not** the
  full-snapshot `cyrius lib sync`, which over-vendors unused platform variants
  like `process_agnos.cyr`/`*_win`/`*_macos` and bloats the lock). Result: a
  minimal **30-file** `lib/` and a 30-entry `cyrius.lock` that matches
  `cyrius deps --verify` exactly (30/30) — fixing the CI hash-verify failure on
  the spurious `lib/process_agnos.cyr` entry that the snapshot had pulled in.
- **`dist/hisab.cyr`** — regenerated via `cyrius distlib` (header now v2.6.6).

### Fixed
- **Tracked toolchain issues re-verified on 6.2.11** (doc-health mandate at each
  pin bump): **3 of 5 now fixed upstream** —
  `cbt` `[build]`-comment `modules` substring false-positive,
  the cc5 18-arg-fn register/stack scramble (Perlin `_perm_init` corruptor),
  and `cyrius lint` returning the warning count as its exit code (now returns 0;
  the CI `^\s*warn ` stdout grep remains the load-bearing warning gate). Archived
  to `docs/development/issues/archived/`. Still open: the C-style
  `for (;cond;)` empty-clause parse error (workaround: `while`). The CLI
  unknown-flag-clobbers-source issue was not re-tested (destructive).

## [2.6.5] - 2026-05-30 — 2.6.x closeout: P(-1) / security audit + diffgeo equation reference

Closeout pass for the 2.6.x differential-geometry arc — P(-1) cleanliness, a
memory-safety / numerical review of the seven new `diffgeo.cyr` functions, and
the documentation deliverable. **No new vulnerability; no source change.**
Docs-only. The 2.6.x arc is complete.

### Added
- **`docs/architecture/math.md` §2 — Differential Geometry** — curvature
  conventions (Riemann `R^ρ_{σμν}`, the load-bearing sphere sign `R^θ_{φθφ}=+1`),
  the sectional / Weyl / Jacobi formulas, the transport ODE, and the
  exterior-algebra grading — each with its pinned identities and references
  (do Carmo; Lee; Wald; Misner-Thorne-Wheeler). The catalogue index now points at §2.
- **`docs/audit/2026-05-30.md`** — the closeout audit report.

### Security
- Reviewed the seven new diffgeo functions: bounded contraction loops; the RK4
  transport allocates fixed work arrays once per call; `weyl_tensor`'s `dim⁴`
  allocation matches its Riemann input (dim inherits `riemann_tensor`'s `≤ 16`
  cap). All divisions guarded or structurally safe — `sectional_curvature`
  degenerate-plane guard, `weyl_tensor` `n<3` guard, transport `/6`;
  `geodesic_deviation` and the wedges have no division. No new
  shell/FFI/syscall surface. Posture solid.

### Changed
- **`threat-model.md`** — recorded the sectional degenerate-plane guard, the Weyl
  `n<3` guard, and the 4D-form contract; added the 2026-05-30 audit-history entry.
  **`doc-health.md`** — math.md §2 logged + the new audit; bumped to v2.6.5.

## [2.6.4] - 2026-05-29 — Higher-order differential forms (2.6.x arc)

Fifth (last feature) patch of the 2.6.x arc. Extends the exterior algebra past
2-forms. Additive; suite 949 → **957**.

### Added
- **`wedge_2_1(omega, alpha)`** (2-form ∧ 1-form → 3-form) and **`wedge_3_1(beta,
  alpha)`** (3-form ∧ 1-form → 4-form) in `diffgeo.cyr`, in the same reduced
  strictly-increasing basis as `wedge_1_1`. Chaining `wedge_1_1 → wedge_2_1 →
  wedge_3_1` builds every basis k-form up to the 4D top form.
- **`tests/hisab.tcyr`** — 8 assertions: `e0∧e1∧e2 = (012)` and the unit 4-form
  `e0∧e1∧e2∧e3`; graded antisymmetry `e1∧e0 = −(e0∧e1)` and nilpotence `e0∧e0 = 0`;
  permutation signs (`e0∧e2∧e1 = −(012)`, `e1∧e2∧e3∧e0 = −(0123)`); and a repeated
  factor → 0.

## [2.6.3] - 2026-05-29 — Geodesic deviation / Jacobi equation (2.6.x arc)

Fourth patch of the 2.6.x arc. Adds the tidal acceleration between neighbouring
geodesics. Additive; suite 943 → **949**.

### Added
- **`geodesic_deviation(riemann, u, jac, dim)`** (`diffgeo.cyr`) — the Jacobi
  tidal term `D²J^ρ/dτ² = −R^ρ_{σμν} u^σ J^μ u^ν` (the J and the second `u` sit in
  Riemann's antisymmetric last-index pair, so the contraction is nonzero). For a
  space form it reduces to `−(|u|² J − ⟨u,J⟩ u)`.
- **`tests/hisab.tcyr`** — 6 assertions on the unit-sphere space form: a
  separation `⊥ u` gives `J'' = −J` (geodesics converge), `∥ u` gives `0`, the
  result scales with `|u|²`, flat space → `0`, and the operator is linear in `J`.

## [2.6.2] - 2026-05-29 — Parallel transport along a curve (2.6.x arc)

Third patch of the 2.6.x arc — the first that adds connection *integration*
(RK4) rather than an algebraic tensor combination. Additive; suite 939 → **943**.

### Added
- **`parallel_transport(v0, curve_vel, christoffel, dim, dt, steps)`**
  (`diffgeo.cyr`) — RK4 integration of `dV^a/dt = −Γ^a_{μν} V^μ ẋ^ν` along a
  curve with constant coordinate velocity `ẋ` (Christoffels held constant per
  step, same convention as `geodesic_rk4`). Plus the `_pt_deriv` RHS helper.
- **`tests/hisab.tcyr`** — 4 assertions: flat space (`Γ=0`) leaves the vector
  unchanged; on a **unit-sphere latitude circle at θ=π/4** (Γ^θ_φφ=−½,
  Γ^φ_θφ=1, metric `diag(1,½)`) transport along `φ` **preserves `⟨V,V⟩`**
  (metric compatibility) and genuinely rotates the vector.

## [2.6.1] - 2026-05-29 — Weyl conformal-curvature tensor (2.6.x arc)

Second patch of the 2.6.x arc. Adds the trace-free (conformally-invariant) part
of the Riemann tensor. Additive; suite 934 → **939**.

### Added
- **`weyl_tensor(riemann, ricci, scalar, metric, dim)`** + **`weyl_get`**
  (`diffgeo.cyr`) — the covariant Weyl tensor `C_{ρσμν}` assembled from the
  lowered Riemann minus the dimension-weighted Ricci/scalar trace terms
  (`1/(n−2)` and `R/((n−1)(n−2))`). Returns all-zero for `n ≤ 2` (Weyl
  undefined / identically zero there).
- **`tests/hisab.tcyr`** — 5 assertions + space-form/trace helpers: `C = 0` for a
  **3D space form** (Riemann & Ricci nonzero), `C = 0` for a **4D space form**
  (conformally flat, Riemann ≠ 0 — the defining oracle), a non-space-form 4D
  Riemann gives `C ≠ 0`, and that `C` is **trace-free** (`g^{ρμ} C_{ρσμν} = 0`).

## [2.6.0] - 2026-05-29 — Sectional curvature (2.6.x arc)

First patch of the 2.6.x arc (differential-geometry depth). Adds the sectional
curvature on top of the existing Riemann tensor. Additive; suite 929 → **934**.

### Added
- **`sectional_curvature(riemann, metric, u, v, dim)`** (`diffgeo.cyr`) —
  `K(u,v) = R(u,v,v,u) / (⟨u,u⟩⟨v,v⟩ − ⟨u,v⟩²)`, lowering the first Riemann index
  with the metric (`R_{ασμν} = g_{αρ} R^ρ_{σμν}`) and contracting over the plane.
  Guards a degenerate plane (`|denominator| < EPSILON → 0`). Positive for a
  sphere — verified against hisab's Riemann convention (`R^θ_{φθφ} = +1` at the
  equator). Plus a `_dg_inner` metric-inner-product helper.
- **`tests/hisab.tcyr`** — 5 assertions against a constant-curvature space form:
  unit-curvature → `K = 1` for any plane (axis-aligned + skew); a radius-2 sphere
  (`metric = diag(4,4)`) → `K = 1/4` (exercises the metric index-lowering); flat →
  `0`; and a degenerate plane → `0` (denominator guard).

### Documentation
- Roadmap maintenance: removed the completed 2.5.x arc section (now in Release
  History) and condensed the Current arc-history; re-organized the flat 2.6.0
  "differential geometry depth" item into a **2.6.x arc** (2.6.0 sectional
  curvature → 2.6.1 Weyl → 2.6.2 parallel transport → 2.6.3 geodesic deviation →
  2.6.4 higher-order forms → 2.6.5 closeout), grounded in the existing
  `diffgeo.cyr` primitives with known-manifold test oracles per patch.

## [2.5.4] - 2026-05-29 — 2.5.x closeout: P(-1) / security audit + equation catalogue

Closeout pass for the 2.5.x arc — P(-1) cleanliness, a memory-safety / numerical
review of the new CGA operators + `mat_new_guarded`, and the documentation
deliverable. **No new vulnerability; no source change** (the arc's additions are
bounded and guarded). Docs-only.

### Added
- **`docs/architecture/math.md`** — equation catalogue (earns the CLAUDE.md
  "mathematical reference" doc slot). Documents the conformal model / metric /
  blade layout; the product family (geometric, outer, left/right contraction with
  grade rules); reverse / norm / blade inverse; dual + pseudoscalar; projection /
  rejection — each with the pinned GA identities and literature references
  (Dorst-Fontijne-Mann, Hestenes-Sobczyk, Doran-Lasenby) — plus a catalogue index
  pointing at the rest of the library's formula material.
- **`docs/audit/2026-05-29-cga-arc-closeout.md`** — the closeout audit report.

### Security
- Reviewed the six 2.5.x functions: results use fixed 256-byte allocations,
  loops are bounded to blade indices 0..31 (no computed/unbounded indexing),
  `cga_blade_inverse` guards null blades, `cga_pseudoscalar_inv`'s divisor is the
  structural constant `⟨I~I⟩₀ = −1`, and `mat_new_guarded` caps dims (CWE-190).
  No new shell/FFI/syscall surface. Posture solid.

### Changed
- **`threat-model.md`** — CGA division guards added to the attack-surface table;
  2.5.4 audit-history entry. **`overview.md` / `README.md`** now reference
  `math.md`. **`doc-health.md`** — math.md marked earned + the new audit logged.

### Notes
- Stdlib `mat_new` overflow remains **upstream / deferred** (cyrius pin unchanged);
  `mat_new_guarded` is the mitigation. The CGA identity tests (29 assertions
  across 2.5.0–2.5.3) pin the equation material catalogued in `math.md`.

## [2.5.3] - 2026-05-29 — `mat_new` overflow guard (2.5.x arc)

Final feature patch of the 2.5.x arc (the 2.5.4 audit/doc pass closes it out). Carryover from the 2.4.6 audit (P(-1) C3): stdlib
`matrix.cyr` `mat_new(rows, cols)` computes `16 + rows*cols*8` with **no guard**
and is still unguarded in the pinned 6.0.14 (the cyrius fix is upstream/later).
This adds the hisab-side mitigation. Suite 925 → **929**; the 2.5.x arc is complete.

### Added
- **`mat_new_guarded(rows, cols)`** (`linalg_ext.cyr`) — overflow-guarded
  real-matrix constructor (CWE-190). Returns null for non-positive or
  overflow-prone dimensions (cap `_MAT_MAX_ELEMS = 16M`, keeping `rows*cols`
  well below 2⁶⁰ so `16 + rows*cols*8` can't wrap); otherwise delegates to
  stdlib `mat_new`. The guarded entry point for matrices sized from untrusted /
  external input — parity with the existing `cmat_new` guard. hisab's internal
  callers (SVD/QR, optimize) pass dims from already-allocated matrices and are
  unaffected.
- **`tests/hisab.tcyr`** — 4 CWE-190 regression assertions (huge dims, zero rows,
  over-cap 25M-element, and a valid small matrix).

### Notes
- The stdlib `mat_new` fix remains **upstream / roadmap-tracked** — re-verify when
  the cyrius pin moves; until then `mat_new_guarded` is the safe constructor and
  hisab's direct `mat_new` calls stay mitigated (dims from existing matrices).

## [2.5.2] - 2026-05-29 — CGA blade projection / rejection (2.5.x arc)

Third patch of the 2.5.x arc — the last CGA piece, composing 2.5.0's contraction
with a blade inverse. Additive; suite 915 → **925**.

### Added
- **`cga_blade_inverse(b)`** — `B⁻¹ = reverse(B) / (B · reverse(B))_scalar`, with a
  zero-norm guard (returns the zero multivector for a **null** blade, e.g. conformal
  points, instead of dividing by zero).
- **`cga_project(x, b)`** — `project(X,B) = (X ⌋ B) ⌋ B⁻¹`, the component of `X`
  lying in blade `B`. Preserves grade(X) (inner contraction lowers by grade(X),
  outer raises it back).
- **`cga_reject(x, b)`** — `X − project(X,B)`, the part of `X` orthogonal to `B`
  (grade-consistent since projection preserves grade).
- **`tests/hisab.tcyr`** — 10 assertions: `project(e1,e12) = e1` (in-plane, no e2
  leak), `project(e12,e12) = e12` (self), `project(e3,e12) = 0` / `reject(e3,e12) = e3`
  (orthogonal), `reject(e1,e12) = 0` (in-plane), idempotence, `project + reject = X`
  for `X = e1+e3`, and the null-blade guard (`project(e1, 0) = 0`, no trap).

## [2.5.1] - 2026-05-29 — CGA dual + pseudoscalar inverse (2.5.x arc)

Second patch of the 2.5.x arc. Adds Hodge-style duality on top of 2.5.0's
contraction. Additive (new public functions); suite 909 → **915**.

### Added
- **`cga_pseudoscalar()`** — the unit pseudoscalar `I = e1∧e2∧e3∧ep∧em` (grade-5
  blade `[31]`).
- **`cga_pseudoscalar_inv()`** — `I⁻¹ = reverse(I) / (I · reverse(I))_scalar`. In
  this metric (em² = −1) `I² = −1`, so `I⁻¹ = −I` — derived from the geometric
  product rather than hard-coded.
- **`cga_dual(x)`** — `x* = x · I⁻¹`, mapping a grade-k blade to grade `5−k`
  (since `I` is the top blade, this geometric product is exactly the contraction
  onto the pseudoscalar).
- **`tests/hisab.tcyr`** — 6 assertions: `I · I⁻¹ = 1`; grade flips `dual(1) = −I`
  (0→5) and `dual(I) = 1` (5→0); `dual(e1) = −e23pm` (1→4) with no residual e1;
  and the involution `dual(dual(e1)) = −e1` (pins the −x sign for this metric).

## [2.5.0] - 2026-05-29 — CGA contraction operators + doc catchup (2.5.x arc)

First patch of the 2.5.x arc (CGA depth + matrix guard). Conformal geometric
algebra already had the geometric/outer products, reverse, sandwich, and
conformal constructors; this adds the **interior-product** primitives. Additive
(new public functions, no signature changes); suite 901 → **909**. This release
also folds in the post-2.4.6 documentation catch-up (the repo-wide sweep + the
new `docs/doc-health.md` ledger), recorded below.

### Added
- **`cga_left_contraction(a, b)`** (a ⌋ b) and **`cga_right_contraction(a, b)`**
  (a ⌊ b) in `geo_advanced.cyr` — the contraction interior products. Same blade
  loop as `cga_outer_product` with a grade-difference selector: left keeps
  `grade(b) − grade(a)`, right keeps `grade(a) − grade(b)` (negative targets
  never match a non-negative blade grade, so out-of-range terms drop to zero).
- **`tests/hisab.tcyr`** — 8 CGA contraction assertions pinning the GA identities:
  `e1 ⌋ e12 = e2` (grade lowering), `e1 ⌋ e1 = 1` and `(2e1+3e2) ⌋ self = 13`
  (vector self-contraction → scalar norm²), `e1 ⌋ e23 = 0` (orthogonal), scalar
  contraction as scaling, and right contraction `e12 ⌊ e1 = −e2` with its grade
  drop. CGA went from 1 smoke assertion to 9.

### Documentation
- Repo-wide documentation sweep to v2.4.6 state: pruned the roadmap to
  forward-facing / deferred work only (the completed 2.3.x and 2.4.x arcs now
  live in Release History); refreshed README (added the **Collision** module row
  + spatial/einsum/lie_ext/linalg_precision/noise_simplex; version, counts,
  binary size); completed the `architecture/overview.md` module map (all 34
  modules + a Collision section) and updated the collision data-flow (MPR +
  sequential-impulse); synced test/benchmark counts (901 assertions / 26
  benchmarks) in `testing.md`; added collision + supply-chain entries to
  `SECURITY.md`; added a 2026-05-29 status addendum to the dated `port-audit.md`;
  refreshed `dependency-watch.md` and the CLAUDE.md status line.
- Added `docs/doc-health.md` — a living doc-currency ledger (fresh/stale/dated/
  tracked-issue per file), adapted from `cyrius/docs/doc-health.md`.
- Re-ran `scripts/bench-history.sh` (benchmarks.md was stale at an old commit) —
  refreshed to v2.4.6, 26 benchmarks.
- Removed `docs/development/tool-issues.md` (ad-hoc catalog; real bugs are filed
  individually under `docs/development/issues/`). Archived the shipped
  `cyrius-linalg-proposal.md` to `docs/development/archive/`. Fixed CONTRIBUTING
  currency (Cyrius pin 6.0.14, `src/` not `lib/`, +fmt/distlib gates).
- Re-verified the five `docs/development/issues/` toolchain filings against the
  pinned 6.0.14 — all still reproduce (none stale-fixed).
- Re-organized the roadmap's flat 2.5.0 item into a **2.5.x CGA arc** (2.5.0
  contraction → 2.5.1 dual → 2.5.2 projection/rejection → 2.5.3 `mat_new` guard
  closeout), mirroring the 2.4.x per-patch + commit-bite structure.

## [2.4.6] - 2026-05-29 — Security & hardening audit; 2.4.x closeout

P(-1) hardening step closing the 2.4.x arc: a memory-safety / allocation-overflow
review of the whole library, supply-chain integrity, dependency CVE posture
(known + web research), and a security review of the six collision algorithms
brought into the build chain across 2.4.0–2.4.5. **No new vulnerability found.**
Suite 895 → **901**. Full report: `docs/audit/2026-05-29.md`.

### Added
- **`tests/edge_cases.tcyr`** — 6 allocation-overflow-guard regression assertions
  (CWE-190): `cmat_new` rejects huge / non-positive dims and allocates valid ones;
  `tensor_new` rejects rank > 8 and overflow-prone shapes and allocates valid ones.
  Pins the documented guards so removing one fails the build.

### Security
- **Audit verdict — posture solid.** No shell/exec/FFI/libc surface (no CWE-78).
  hisab's own dimension allocators all guard the multiply before allocating
  (`tensor_new`, `cmat_new`, christoffel/riemann, num_sieve). All iterative paths
  are capped. Collision index access goes through `vec_get`, which traps on OOB
  rather than corrupting memory.
- **Supply chain**: `cyrius deps --verify` → 60 verified / 0 failed;
  `cyrius vet` → 0 untrusted. No third-party deps (cyrius stdlib + first-party
  sakshi only) → zero third-party-CVE surface. No CVEs exist for the cyrius/cycc
  toolchain.
- **Open (upstream)**: stdlib `mat_new` integer overflow (`16 + rows*cols*8`,
  unguarded) is still present in the pinned 6.0.14 snapshot. hisab's usage is
  mitigated (dims come from already-allocated matrices; raw-dim `cmat_new` is
  guarded). Fix belongs in cyrius stdlib — tracked as roadmap 2.5.0.

### Changed
- **`docs/development/threat-model.md`** — added the six 2.4.x collision algorithms
  to the attack-surface table; refreshed the `mat_new` note for 6.0.14 + roadmap
  2.5.0; added a Supply Chain section and a 2026-05-29 audit-history entry.

## [2.4.5] - 2026-05-29 — Collision arc: contact solver fixed; arc complete (2.4.x)

Final patch of the collision-correctness arc, and the third with a real bug.
`sequential_impulse` produced **all-zero impulses** — it did nothing.
`solve_pgs` (the general PGS/LCP solver in `linalg_ext`) was already correct but
barely tested. Suite 888 → **895**. This closes the 2.4.x arc.

### Fixed
- **`sequential_impulse` normal impulse** (`collision_core.cyr`): the impulse was
  computed as `(1+e)·v_n / inv_mass` with `v_n = −pen` — i.e. **negative** for a
  penetrating contact (the leading `−` the comment specified was missing), so
  `max(0, accumulate)` clamped it to 0 on every iteration. Separately, the loop
  re-added the full bias each iteration without folding the accumulated impulse
  back into the velocity, so it could never converge. Rewritten as proper
  sequential impulse: `v_eff = v_n + λ·inv_mass`, `Δλ = −(1+e)·v_eff / inv_mass`,
  accumulate-and-clamp. The total impulse now converges to `(1+e)·pen / inv_mass`
  (e.g. 0.25 for pen=0.5, inv_mass=2) and is iteration-independent.

### Added
- **`tests/modules.tcyr`** — 4 `sequential_impulse` assertions: resting contact
  (λ=0.25, the case that was 0), single-iteration convergence (idempotence),
  dynamic-vs-static (λ=1.0), and both-static (skipped → 0).
- **`tests/hisab.tcyr`** — strengthened the `solve_pgs` test from a lone `x1 > 0`
  sanity check to exact-solution assertions (`x ≈ [1/11, 7/11]` for
  `[[4,1],[1,3]]x=[1,2]`) plus a **bounds-active** LCP (`diag(1,1)·x=[5,5]` clamped
  to `hi=[2,2]` → `[2,2]`). solve_pgs was audited correct — no change.

### Notes — 2.4.x arc complete
- All six collision algorithms audited and covered. Genuine bugs found + fixed in
  three: the hull insertion sort + undefined `f64_le`/`f64_ge` (2.4.0), the MPR
  origin-containment early-out (2.4.4), and this impulse-sign/convergence fix
  (2.4.5). `triangulate_polygon`, `delaunay_2d`, the half-edge mesh, and
  `solve_pgs` were already correct — they just lacked coverage, which they now have.

## [2.4.4] - 2026-05-28 — Collision arc: MPR narrowphase fixed (2.4.x arc)

Fifth patch of the collision-correctness arc, and the second with a real bug
(after 2.4.0's hull). `mpr_intersect` / `mpr_penetration` (Minkowski Portal
Refinement, 3D) returned a **false positive on every separated pair** — they
never detected non-overlap. Fixed; suite 878 → **888**.

### Fixed
- **MPR origin-containment early-out** (`collision_core.cyr`): after building the
  first portal support point `v1` (the farthest Minkowski-difference point toward
  the origin), the code lacked the standard check that the origin is actually
  reachable. For separated shapes `v1` falls short of the origin, and — when the
  centers are colinear with it — execution dropped into the degenerate
  `(v1−v0)×(−v0) == 0` branch whose comment says "check if between them" but which
  **unconditionally returned 1 (hit)**. Off-axis separated pairs slipped through
  the refinement loop's convergence test the same way. Added
  `if dot(v1, dir) < 0 → miss` to both `mpr_intersect` (returns 0) and
  `mpr_penetration` (returns 0); once it passes, the origin is genuinely within
  the support toward it and the existing portal refinement is correct.

### Added
- **`tests/modules.tcyr`** — 10 MPR assertions + sphere/box support-function
  helpers: sphere-sphere overlap (hit) vs separated on- and off-axis (miss — the
  regression); penetration depth (`sum_radii − dist` = 1.0, plus the deeper 3.0
  case) with a ±x normal; separated → miss; and box-box overlap / separated.

### Notes
- Of the five arc patches so far, only 2.4.0 (hull sort) and 2.4.4 (MPR) carried
  genuine bugs; triangulate / delaunay / half-edge were already correct.

## [2.4.3] - 2026-05-28 — Collision arc: half-edge mesh audited (2.4.x arc)

Fourth patch of the collision-correctness arc. The roadmap flagged the half-edge
mesh (`halfedge_from_triangles` + `halfedge_adjacent_faces` + `halfedge_is_boundary`)
for a twin-pointer-wiring check — but a thorough audit found it **already correct**
on both closed and open meshes. Deliverable: coverage, no source change; suite
867 → **878**.

### Added
- **`tests/modules.tcyr`** — 11 half-edge assertions + `_he_tri` / `_he_boundary_sum`
  helpers, over four reference meshes: a **tetrahedron** (closed manifold → no
  boundary vertices, 3 neighbours/face), a **single triangle** (open → all-boundary,
  no neighbours), a **2-triangle quad** sharing a diagonal (4 boundary corners, 1
  shared edge), and a **hexagon fan** (center interior, 6-vertex boundary rim — the
  case that exercises the one-ring walk closing on an interior vertex). Plus the
  empty-input → null-mesh error path.

### Notes (audit — no change needed)
- Twin assignment pairs each directed half-edge `src→dst` with the reverse
  `dst→src` (O(n²) scan); closed meshes wire every twin, open meshes leave
  boundary half-edges at `_COL_SENTINEL`. `halfedge_is_boundary` walks the
  one-ring (`next→next→twin`) with a 1000-step guard, returning 1 on any sentinel
  twin and 0 only when the ring closes back to the start. `halfedge_adjacent_faces`
  collects the twin's face for each shared edge. All correct.

## [2.4.2] - 2026-05-28 — Collision arc: `delaunay_2d` audited (2.4.x arc)

Third patch of the collision-correctness arc. The roadmap flagged
`delaunay_2d` (Bowyer-Watson) as needing a numerical-stability pass — but a
thorough audit found it **already correct and robust**, including the cocircular
grid stress case. The deliverable is coverage that verifies the empty-circumcircle
property, plus an honest "no bug" record. No source change; suite 859 → **867**.

### Added
- **`tests/modules.tcyr`** — 8 `delaunay_2d` assertions + a `_dl_violations`
  helper that counts points strictly inside any output triangle's circumcircle
  (the Delaunay property holds iff that count is 0). Fixtures: square + interior
  (4 triangles), a **3×3 grid** (heavy cocircularity — the stability stress
  case, 8 triangles), an irregular 6-point set, a single triangle (`n = 3`), and
  degenerates (`n < 3` and all-collinear → empty, no trap). Every non-degenerate
  case has **0 circumcircle violations**.

### Notes (audit — no change needed)
- The in-circle predicate (`_col_in_circumcircle`) adjusts the determinant by the
  triangle's winding sign and tests **strict** interiority, so cocircular points
  (grid corners) are correctly not flagged — that's why the grid triangulates
  cleanly. Super-triangle setup/removal, bad-triangle cavity collection, boundary
  (non-shared) edge extraction, descending swap-remove, and CCW-normalized output
  are all correct.
- Collinear / `n < 3` inputs return an empty triangulation (all triangles
  reference super-triangle vertices and are filtered) rather than trapping.

## [2.4.1] - 2026-05-28 — Collision arc: `triangulate_polygon` audited (2.4.x arc)

Second patch of the collision-correctness arc. The roadmap flagged
`triangulate_polygon` (ear clipping) as a likely-buggy untested port — but a
thorough audit found it **already correct**. The deliverable is comprehensive
coverage that promotes it from "deliberately unexercised" to verified, plus an
honest record that no bug existed. No source change; suite 846 → **859**.

### Added
- **`tests/modules.tcyr`** — 13 `triangulate_polygon` assertions + a `_tp_area2x`
  helper. Each fixture checks the triangle count (`3*(n−2)` flat indices) and
  that the output **tiles the polygon** (|Σ 2×area| == polygon |2×area|; abs ⇒
  winding-agnostic): convex quad, concave 5-gon (reflex vertex), convex hexagon,
  U-shape (2 reflex vertices), CW-wound quad, a collinear edge vertex, and the
  `n < 3` empty case.

### Notes (audit — no change needed)
- Ear detection (reflex classification via signed area + point-in-triangle),
  winding normalization (CCW/CW), and the shrink-as-clipped index bookkeeping
  are all correct. The `n*n` iteration cap + "no ear found → bail" guard make
  degenerate inputs (e.g. duplicate vertices) terminate without trapping.
- Unlike `convex_hull_2d` (2.4.0, two real bugs), `triangulate_polygon` shares
  no code with the broken sort and never referenced the undefined `f64_le`/
  `f64_ge` — it uses only the strict `f64_lt`/`f64_gt` intrinsics, which is why
  it was sound where the hull was not.

## [2.4.0] - 2026-05-28 — Collision correctness arc: `convex_hull_2d` (2.4.x arc)

First patch of the 2.4.x collision-correctness arc. `convex_hull_2d` (Andrew's
monotone chain) carried two pre-existing port bugs and trapped on any non-trivial
input — it had never run, having sat outside the build chain until 2.2.2. Both
bugs are fixed and the algorithm is now exercised + covered. Unlike the 2.3.x
arc this **changes behavior** (a trapping function now returns correct results),
so it follows a red-fixture-first cadence.

### Fixed
- **`convex_hull_2d` insertion sort** (`collision_core.cyr`,
  `_col_sort_indices_by_xy`): the hand-rolled sort's `done`-exit path
  overwrote the insertion index with `-1` on every *mid-array* insertion, so
  `vec_set(indices, -1, key)` tripped a `vec: index < 0` trap (front insertions
  happened to work). Rewritten as a standard insertion sort — shift-greater
  then drop `key` at `sj + 1` (always `>= 0`).
- **Missing `f64_le` / `f64_ge` primitives** (`f64_util.cyr`): the monotone
  chain's pop test calls `f64_le(cross, 0)`, but `f64_le`/`f64_ge` were never
  defined (only the strict `f64_lt`/`f64_gt` exist as named-op intrinsics) —
  so once the sort was fixed and execution reached the chain, the call bound to
  an undefined symbol and trapped (SIGILL). Defined both in terms of the strict
  intrinsics (`a <= b == !(a > b)`, `a >= b == !(a < b)`). This also resolves
  the same latent undefined-symbol references at 6 call sites in `spatial.cyr`
  (point-in-region tests) that current tests didn't reach.

### Added
- **`tests/modules.tcyr`** — 13 `convex_hull_2d` assertions: the keystone
  square + interior-point fixture (4-vertex CCW hull, shoelace 2×area = +8),
  plus degeneracy coverage — empty/single point, triangle (count + CCW area),
  collinear set (→ 2 extreme endpoints, interior dropped), and a duplicated
  corner (→ still 4 vertices). None trap. Suite: 833 → **846**.

## [2.3.4] - 2026-05-28 — Layout & idiom modernization (2.3.x arc)

A **layout/idiom** patch: tie heap allocations and field writes to the typed
struct definitions so a future field addition can't silently under-allocate or
land at a stale offset. Internal-only — public API, results, and machine code
are unchanged. All 833 tests pass **bit-identical**; key benchmarks verified
**flat** (`sizeof(T)` is a compile-time constant equal to the old literal, and
a derived setter lowers to the same `store64` at the same offset, so codegen is
byte-identical — no perf claim).

### Changed
- **Constructors → `sizeof(T)` + derived setters** across all 13 `#derive(accessors)`
  struct modules: `alloc(<magic-literal>)` → `alloc(sizeof(T))` and
  `store64(p + <offset>, v)` → `T_set_field(p, v)`. This eliminates the last
  hand-computed offsets (reads already used the derived getters) and couples
  every allocation to its struct layout:
  - **vec2/vec3/vec4** (`HVec2`/`HVec3`/`HVec4`), **quat** (`HQuat`),
    **complex** (`HComplex`), **autodiff** (`Dual`), **interval** (`Interval`),
    **tensor** (`HTensor` header), **geo** (all 9: `GeoRay`/`GeoPlane`/`GeoAabb`/
    `GeoSphere`/`GeoObb`/`GeoCapsule`/`GeoTriangle`/`GeoLine`/`GeoSegment`),
    **collision_core** (`ColContact`), **collision_mesh** (`HalfEdge`/`HalfEdgeMesh`).
  - SIMD scalar tails (e.g. vec3's `store64(r + 16, …)` z-write) also moved to
    `HVec3_set_z` — packed `f64v_*` writes stay as raw-memory ops.
- **Matrix grid sizes → enum constants**: `mat3.cyr` `MAT3_BYTES = 72` and
  `mat4.cyr` `MAT4_BYTES = 128` replace the magic byte-size literal that was
  repeated across `alloc` + copy-loop bounds (couples the copy length to the
  allocation length). `m3_set`/`m4_set` (column-major grids, no named fields)
  stay computed-index — a field-struct doesn't fit a flat NxN grid.
- **Float-render buffer → enum constant**: `FLOAT_RENDER_BUF = 32`
  (`var buf[FLOAT_RENDER_BUF]`) shared by the `symbolic.cyr` / `symbolic_ext.cyr`
  float renderers, replacing two bare `var buf[32]` literals.

### Added
- **`#must_use` on the core value-returning API** (9 modules: vec2/3/4, quat,
  complex, interval, autodiff, mat3, mat4) — the compiler now warns at build
  time if a pure constructor/operation result is discarded (a guaranteed bug
  for a side-effect-free function). Status-returning setters (`m3_set`/`m4_set`)
  are deliberately **not** annotated. Verified the full 34-module build and the
  833-test suite emit **zero** "result is discarded" warnings.

### Notes (evaluated — deferred or N/A)
- **`#pure`**: deferred. Its CSE/memoization semantics interact unsafely with
  hisab's allocate-a-fresh-result convention (two "equal" calls must return
  distinct mutable pointers); the win is speculative (hot paths are already SIMD)
  with no benchmark driver. Not worth the risk for a patch.
- **Slices (`[T]`/`slice<T>`)**: deferred. The hot loops are raw-pointer +
  `load64`/`store64`, already SIMD-optimized (2.3.1). Bounds-checked slices would
  regress them; `slice_unchecked_get_W` discards the safety benefit, leaving only
  churn. No consumer demand.
- **`defer`**: N/A. The library holds no per-resource lifecycles — long-lived
  data uses the bump/arena allocator (never individually freed) and the library
  opens no file descriptors. Nothing for `defer` to clean up.

## [2.3.3] - 2026-05-28 — Safety & numerical-correctness audit (2.3.x arc)

A correctness audit of the integer/bit-math surface against the vidya gotcha
catalogue (signed shifts, static buffers, element-vs-byte sizing, overflow).
**Verdict: no bugs found — hisab was already careful.** The deliverable is a
fixed misleading comment plus regression tests that pin the load-bearing
invariants, so a future toolchain change can't silently break them. 833/833
tests pass (was 825 + 8 new invariant assertions).

### Fixed
- **num.cyr PCG comment**: the note claimed "Cyrius `>>` is arithmetic
  (sign-extending)" — it is **logical** (zero-fill), verified on 6.0.14
  (`0x8000…0 >> 1 == 0x4000…0`). The PCG RNG is correct either way (it wants
  unsigned shifts and masks to 32 bits), but the comment actively misinformed.
  Corrected.

### Added
- **`tests/edge_cases.tcyr`** — safety-invariant section (8 assertions):
  - `>>` is logical/zero-fill (the PCG + bit-reversal code depends on it).
  - `_num_mulmod` (Russian-peasant) stays overflow-safe — Fermat check
    `2^(p−1) ≡ 1 (mod p)` for prime p≈1e11, whose intermediate squares (~1e22)
    far exceed i64 (a naive `a*b` would overflow).
  - PCG32 determinism/reproducibility for a fixed seed.
  - float-render `var buf[N]` independence across calls (`str_from_buf` copies,
    so the process-static buffer is safe).

### Notes (audit findings — no change needed)
- **Static buffers**: only two `var buf[32]` (float rendering), both passed to
  `str_from_buf`, which allocates + `memcpy`s — no escape, no cross-call
  corruption despite `var buf[N]` being process-static in Cyrius.
- **Bytes-vs-elements**: no bug — element arrays consistently use `alloc(n*8)`;
  the only stack arrays are byte buffers used as bytes.
- **Signed shifts**: every `>>`/`<<` operates on non-negative or masked values
  (FFT lengths, masked RNG state, non-negative exponents, bit-index masks), so
  logical `>>` is correct throughout. No `asr()` helper needed.
- **Overflow**: no factorial/binomial kernels exist (those are abaco's);
  Pollard-rho/modpow already use `_num_mulmod` to avoid `a*b` overflow. No
  unary `~` is used anywhere (all `~` are "≈" in comments).

## [2.3.2] - 2026-05-28 — Bounded einsum scratch memory (2.3.x arc)

A **memory-footprint** patch (not a speed change): einsum's parse/contraction
scratch now lives in a reused module-global arena instead of leaking to the
never-freeing bump allocator on every call. Internal-only; all 825 tests pass
with identical results.

This patch re-scoped 2.3.2 after verification (see Notes): the roadmap's
"replace hand-rolled transcendentals" item was a no-op — hisab already uses
the stdlib `f64_sin/cos/exp/ln/abs` named-op intrinsics everywhere, with zero
hand-rolled series — and arena adoption only fit einsum.

### Changed
- **einsum** (`src/einsum.cyr`): all ~11 per-call scratch allocations (operand
  specs, label dims, contraction sets, output shape) and the hot per-element
  `label_vals`/per-product `indices` buffers now come from a module-global
  arena (`_einsum_arena`, 8 KB) that is `arena_reset` at each call's entry.
  `label_vals`/`indices` are also hoisted to reused fixed-size buffers instead
  of being re-allocated per output element/product. einsum is non-reentrant
  and single-threaded; `tensor_new` copies the shape, so no scratch escapes
  into the returned tensor (verified).

### Performance (memory, not speed)
Measured per call for a 4×4 `ab,bc->ac` contraction (bump bytes consumed,
100-call average), before = HEAD's released 2.3.1 einsum:

- **3960 → 176 bytes/call** (~**22×** less). The 176 B is the result tensor
  the caller owns; the ~3784 B of per-call scratch leak is **eliminated** —
  scratch is now a one-time 808 B arena reused across all calls.
- For a consumer making N einsum calls: `N×3960` (unbounded growth) → `N×176
  + 808`. Speed is unchanged (arena alloc ≈ bump alloc, both O(1) bumps).

### Notes
- **Transcendentals already optimal**: `f64_sin/cos/exp/ln/abs` are stdlib
  named-op intrinsics (hardware FP on x86, aarch64 polyfills). hisab uses them
  directly (`f64_util.cyr` only composes them, e.g. `f64_tan = sin/cos`). No
  hand-rolled Taylor/Newton series exist to replace. No change made.
- **Arena scoped to einsum**: symbolic/tensor allocations are escaping result
  nodes (expr trees / tensors returned to the caller), not scratch, so an
  arena-reset would corrupt results — correctly left alone.

## [2.3.1] - 2026-05-28 — SIMD hot paths (2.3.x optimize/modernize arc)

First patch of the 2.3.x arc: route vec/mat/quat hot paths through Cyrius's
packed-double `f64v_*` builtins (added with hisab named as the gap-close
consumer). Internal-only — public API and results unchanged; all 825 tests
pass with **bit-identical** output to the scalar paths.

### Performance
Measured with an amplified microbench (loop timed by two `now_ns()` calls;
the committed `bench()` harness has a ~488 ns per-iteration floor that hides
op-level timings — see Notes). Before → after, ns/op:

- **vec4 dot**: 26 → 4 ns (**~6.5×**, no alloc)
- **vec4 add**: 46 → 20 ns (**~2.3×**)
- **vec3 dot**: 20 → 9 ns (**~2.2×**, n=2 xy + scalar z)
- **vec3 add**: 35 → 22 ns (**~1.6×**)
- **m4_mul**: 716 → 158 ns (**~4.5×** — scalar path made 64+ accessor calls)
- **m4_mul_vec4**: 106 → 59 ns (**~1.8×**)
- **m3_mul**: 349 → 108 ns (**~3.2×**, over-read-safe hybrid)

### Changed
- **vec4** (`src/vec4.cyr`): `dot`/`add`/`sub`/`scale` → `f64v_*` at n=4 (HVec4
  is exactly 4 contiguous f64; even n → no over-read). `length`/`length_sq`/
  `normalize` inherit via `dot`.
- **vec3** (`src/vec3.cyr`): `dot`/`add`/`sub`/`mul`/`scale` → over-read-safe
  hybrid (`f64v_*` at n=2 on the xy pair + scalar z tail). HVec3 is 24 B = 3
  doubles, so n=3 would read one f64 past the allocation.
- **mat4** (`src/mat4.cyr`): `m4_mul` and `m4_mul_vec4` → column-major linear
  combination (`f64v_scale`+`f64v_add` over contiguous 32-B columns).
  `m4_transform_point`/`dir` inherit it.
- **mat3** (`src/mat3.cyr`): `m3_mul`, `m3_mul_vec3` → over-read-safe hybrid.
- **quat** (`src/quat.cyr`): `dot` → `f64v_dot` n=4; `normalize` scale → `f64v_scale`.
  The Hamilton product stays scalar (not elementwise).

### Added
- **`tests/hisab.bcyr`**: amplified batch benchmarks (`vec3_dot_x64`,
  `vec4_dot_x64`, `m4_mul_x16`, `m4_transform_x64`) that loop N ops per timed
  call to clear the harness floor, so the SIMD wins are regression-trackable
  in `bench-history.csv`.

### Notes
- `f64v_*` are **global codegen intrinsics** (no `[deps]`/include needed). They
  MUST be reached through a function prologue: a bare `f64v_*` call from a
  top-level `.tcyr` statement SIGSEGVs on a misaligned stack (SSE alignment).
  All hisab ops are functions, so production + tests-via-wrappers are safe.
- FP associativity: SIMD pairwise-sum vs scalar left-fold gave bit-identical
  results across all 825 assertions — no tolerance regressions.

## [2.3.0] - 2026-05-28 — Cyrius 6.0.14 toolchain + library laid out in `src/`

Toolchain modernization to **Cyrius 6.0.14** (from 5.7.10) and a structural
move of the library source into `src/`, matching the sibling math engine
abaco's "proper library" layout. No behavioral changes — all 825 tests pass
unchanged. Reviewed abaco (modern patterns), and vidya + cyrius changelogs
(language updates) to scope the upgrade; the 6.0.x range is a low-risk jump
for a pure math library (no forced source-syntax breaks).

### Changed
- **Toolchain**: `cyrius.cyml` pin `5.7.10` → `6.0.14`. The 6.0.0 `cc5`→`cycc`
  binary rename is transparent to consumers (`cyrius build` dispatches; back-
  compat symlinks ship through 6.0.x).
- **Layout**: the 34 math modules moved `lib/*.cyr` → `src/*.cyr` (`git mv`).
  `lib/` is now exclusively vendored stdlib + first-party deps managed by
  `cyrius deps`; `src/` is the library source (smoke `main.cyr` + 34 modules).
  This stops the committed `./lib/` from shadowing the toolchain's
  version-pinned stdlib snapshot. Updated `[lib] modules` paths and the
  `include "lib/…"` → `include "src/…"` references across the 7 test/bench/
  fuzz/example files that exercise the library.

### Fixed
- **sakshi dependency**: `lib/sakshi_sakshi.cyr` was a dangling symlink into a
  stale `~/.cyrius/deps/sakshi/0.9.0` cache (a 5.7.x-era vendoring artifact),
  which broke the lockfile sha step (`sha256sum: No such file`). Cleared the
  bad 2.1.0 cache, re-resolved, and restored the vendored module as a regular
  file (per the 6.0.2 file-over-symlink fix). `[deps.sakshi]` (tag 2.1.0) is
  retained; `cyrius deps --verify` is green (60 verified, 0 failed — the lock
  now hashes only genuine vendored deps under `lib/`, down from 94 once the 34
  math modules moved to `src/`).
- **lint**: collapsed 7 "multiple consecutive blank lines" warnings in
  `src/geo_advanced.cyr` (4) and `src/spatial.cyr` (3) — newly gated now that
  the modules live under `src/`.
- **lint (global-init-order)**: 6.0.14's lint (the check landed at 5.7.32,
  after the old 5.7.10 pin) flagged 5 "global var init refs … (silent zero at
  init)" warnings in `tests/hisab.tcyr` — caused by duplicate top-level var
  names reused across test sections (`t`, `sx`, `sx2`, `se`), where an early
  reference resolved to a later redeclaration. Renamed the later (symbolic +
  tensor) instances to unique names (`symx`/`symx2`/`sexpr`/`tn`). Lint, fmt,
  and vet are now clean across the full CI file set (src + examples + tests).
- **fmt**: reformatted the whole tree (30 files) under 6.0.14's `cyrfmt` — the
  6.0.x formatter differs from the 5.7.10-era one, so files untouched by this
  change still carried latent drift. fmt is whitespace-only; 825/825 unchanged.

### CI (reviewed against abaco's workflows)
- **Fixed the fmt gate**: the old gate did `diff <(cyrius fmt "$f" --check) "$f"`
  — but `--check` writes its report to *stderr* and emits nothing on stdout, so
  the diff compared an empty stream to a non-empty file and failed **every**
  file. Switched to abaco's exit-code form (`cyrius fmt "$f" --check || fail`),
  aggregating across files so each drifted file is annotated.
- **Added a Security Scan job** (mirrors abaco): greps `src/` for raw
  `execve`/`fork` syscalls, `sys_system`, writes to `/etc|/bin|/sbin`, and
  oversized stack buffers (≥64 KiB). hisab's `src/` passes clean; now that the
  math modules live in `src/`, the scan covers the full library surface.
- **Hardened the docs version gate**: now also asserts the
  `version = "${file:VERSION}"` manifest template is intact (matches abaco),
  not just that the version appears in the CHANGELOG.

### Notes
- The empty 0-byte `cyrius.lock` (a symlink/file-copy bug present ecosystem-
  wide since 5.11.8, fixed at 6.0.2) now populates correctly — 60 SHA entries.
- `dist/hisab.cyr` (~545 KB / 16,195 lines) regenerated from the new `src/`
  paths; the `# Generated by: cyrius distlib` header keeps fmt/lint skipping
  the bundle (it exceeds the 512 KiB fmt/lint input cap).

## [2.2.2] - 2026-04-26 — Make it actually build under cc5 5.7.10 + full 34-module distlib

2.2.1 captured the manifest/CI/release modernization intent but was never
tagged because `cyrius build src/main.cyr` didn't pass — Cyrius 5.7.x's
new reserved keywords + an oversized include chain (the CLI prepended all
33 project modules, exceeding cc5's 512 KB input_buf) blocked the build.
2.2.2 closes those, picks up the **5.7.10** toolchain bump for its
`input_buf` 512 KB → 1 MB expansion (and 5.7.8's `syscall arity mismatch`
fix + lockfile-default + 5.7.9's duplicate-fn warning), restores
`lib/collision_core.cyr` + `lib/collision_mesh.cyr` to the bundle (after
fixing pre-existing syntax issues those files had carried since the
original Cyrius port — they were never actually compiled before), and
ships a full **34-module `dist/hisab.cyr` (~544 KB)** verified end-to-end
against a consumer build.

### Fixed
- **`lib/num_ext.cyr`**: renamed local variable `stack` → `stk` (6 identifier sites in `_factorize_pollard_rho`). `stack` became a reserved keyword in Cyrius 5.7.x; the four mentions in comments were left intact
- **`lib/collision_core.cyr`**: 3 empty-init / empty-step `for` loops converted to `while`. Cyrius's `for (init; cond; step)` requires *all three* clauses — `for (; cont == 1;)` and `for (; sj >= 0; sj = sj - 1)` were never valid syntax. The for-with-step variant kept its step semantics by appending `sj = sj - 1` to the loop body tail. File was never in the build chain pre-2.2.2 (orphan-include-after-syscall trick masked it), so this is the first time it actually parses
- **`lib/collision_mesh.cyr`**: same migration — one `for (var ti = 0; ti < n_tris;)` (empty step) converted to a manual `var ti = 0; while (ti < n_tris) { ...; ti = ti + 1 (or stay) }` (loop conditionally advances based on whether the current element was removed). Plus renamed local `shared` → `is_shared` (4 identifier sites — `shared` is reserved in Cyrius 5.5+; one comment mention left intact)
- **`lib/calc.cyr`** `_perm_init` (Perlin noise table): refactored the 18-arg `_perm_store_block(base, off, a..p)` helper into a 10-arg `_perm_store_8(base, off, a..h)` form, called 32× instead of 16×. **cc5 5.7.10 has a codegen bug at 18+ args** that scrambles register/stack params (args 1, 2, 7-12 silently read garbage values — see [`docs/development/issues/2026-04-26-cc5-18-arg-fn-scrambles-params.md`](docs/development/issues/2026-04-26-cc5-18-arg-fn-scrambles-params.md) for the full reproducer). Pre-fix: bench segfaulted at exit 139 inside `perlin_2d` because the permutation table got written full of garbage. Post-fix: `perlin_2d` runs through 200,000 iterations cleanly
- **`src/main.cyr`**: stripped from 30+ project-module includes down to the two stdlib includes its `fn main()` actually uses (`syscalls`, `io`). The previous form prepended every project module just to "validate the include chain" — but cc5 5.7.7's 512 KB input_buf can't fit that, and the test suites already cover include integration. Bonus: fixed three orphan `include` lines that sat *after* `syscall(SYS_EXIT, r)` (parsed but unreachable; first time any of them was scrutinized was when one tripped a parse error). CLI binary: now ~140 KB static ELF, prints the version string and exits
- **`tests/modules.tcyr`**: stripped six "multiple consecutive blank lines" lint warnings (lines 43-45, 263-264, 390 in the pre-fix file). `cyrius lint` returns the warning count as its exit code, which the prior CI loop swallowed under GHA's `set -eo pipefail` — the loop would abort on the first non-zero rc without reporting which file tripped it
- **`examples/basic_math.cyr` + `tests/{edge_cases,foundation,hisab,modules}.tcyr`**: applied `cyrius fmt` to flatten multi-line continuation-indent drift from the modern formatter
- **CI lint loop**: added `set +e` and per-file rc capture so warning-count exit codes don't abort the sweep before later files report. Adds `::error file=...::` annotations so the offending file is visible in the GHA UI

### Added
- **`dist/hisab.cyr`** — full **34-module** distlib bundle (~544 KB / 16,200 lines), regenerated from `[lib]` via `cyrius distlib`. Consumers pull it as `[deps.hisab] modules = ["dist/hisab.cyr"]` — single self-contained file, no per-module `include` choreography. Verified end-to-end: a fresh `[deps] stdlib = [...]` consumer build of an `hvec3_dot` example exits 0 and prints expected output. Fits cc5 5.7.10's 1 MB input_buf with ~480 KB headroom
- **CI distlib drift gate** in `ci.yml` — `cyrius distlib` runs on every push; CI fails if the committed `dist/hisab.cyr` differs from what the current `lib/` produces, so consumers always pull a fresh bundle that matches `lib/`
- **Release distlib regeneration** in `release.yml` — bundle is regenerated and shipped as `hisab-<TAG>.cyr` alongside the source tarball + linux binary + `SHA256SUMS`
- **`tests/modules.tcyr`** — added `collision_core` (`contact_new` + `ColContact_*` accessors) + `collision_mesh` (`detect_islands` smoke) test groups. 4 new assertions; total 253 in this suite, 825 across all four. Convex-hull / Delaunay / MPR are deliberately not exercised yet — they trip pre-existing algorithmic bugs unrelated to the syntax migration; coverage will land alongside an audit pass in a follow-up release
- **`docs/development/tool-issues.md`** — catalog of Cyrius / cc5 / cbt / cyrius-lint / cyim quirks hisab development has tripped on. Language-specific items each have a deep-dive at `docs/development/issues/<DATE>-<slug>.md` with self-contained reproducers + verification steps, so the cc5 maintainer agent can pick them up when there's a slot. Five issues catalogued at 2.2.2: 18-arg fn miscompile, `for` empty-clauses parse, CLI clobbers source on unknown flag, `cyrius lint` rc-as-warning-count, `cbt modules` substring false-positive in `[build]` comments

### Changed
- **Toolchain pin**: 5.7.7 → **5.7.10**. Picks up:
  - **5.7.8** — cc5-level fix for the noisy `lib/syscalls_x86_64_linux.cyr:358: syscall arity mismatch` warning (was firing on every build of every downstream that includes syscalls), `cyrius deps` writing `cyrius.lock` by default, `cyrius check` no longer auto-prepending manifest deps, and the new `cyrius build --no-deps` flag
  - **5.7.9** — `warning: duplicate fn '<name>' (last definition wins)` at registration time (hisab build emits zero such warnings); `json_build` cross-module collision resolved upstream via patra rename
  - **5.7.10** — `input_buf` 512 KB → **1 MB** heap-map reshuffle (+0x100000 region shift). Hisab was the load-bearing reason for this bump per the cc5 5.7.10 release header — `dist/hisab.cyr` was at 96 % of the old cap with hisab actively censoring upstream to stay under
- **`[lib]` block**: full 34-module list restored (32-module trimmed bundle from earlier 2.2.2 drafts is no longer needed). `lib/collision_core.cyr` + `lib/collision_mesh.cyr` re-included after their syntax fixes
- **`README.md` quick-start**: `[deps.hisab] modules = ["dist/hisab.cyr"]` example back; toolchain version → 5.7.10
- **`CLAUDE.md` status + layout**: reflects the CLI smoke binary + 34-module distlib bundle; CI/Release section restores the distlib drift gate

### Local verification
All gates pass locally on this branch:
- `cyrius lint` — clean across 8 files (4 tcyr, 1 bcyr, 1 fcyr, 1 example, src/main.cyr)
- `cyrius fmt --check` — no drift
- `cyrius vet src/main.cyr` — clean
- `cyrius distlib` — `dist/hisab.cyr` = 544 KB / 16,200 lines (34 modules)
- `cyrius build src/main.cyr build/hisab` — OK, 143 KB static ELF, magic verified
- `./build/hisab` — prints `hisab 2.2.2`, exit 0
- `cyrius test tests/hisab.tcyr` — 116/116
- `cyrius test tests/foundation.tcyr` — 307/307
- `cyrius test tests/modules.tcyr` — **253/253** (added 4 collision smoke assertions)
- `cyrius test tests/edge_cases.tcyr` — 149/149
- Total: **825/825 assertions, 0 failed**
- `cyrius build tests/hisab.fcyr` + 5 s timeout run — fuzz: ok
- `cyrius bench tests/hisab.bcyr` — 22 benchmarks complete
- **Consumer smoke**: a fresh `[deps.hisab] modules = ["dist/hisab.cyr"]` consumer project compiles + runs the bundle end-to-end (exits 0, prints expected output)

### Known limitations (deferred)
- `convex_hull_2d` / `triangulate_polygon` / `mpr_intersect` / `delaunay_2d` / `halfedge_from_triangles` / `sequential_impulse` — collision_core and collision_mesh now compile and link, but the algorithms themselves trip runtime bugs (e.g. `convex_hull_2d` hits a `vec: index < 0` bounds check on a 5-point input). These predate the cc5 5.7.x port — the files were added in 2.2.0 but never actually exercised because they sat outside the build chain. Audit + fixes are queued for a follow-up release; for now `tests/modules.tcyr` only smoke-tests the API surface (`contact_new` + accessors, `detect_islands`)
- `lib/` still mixes vendored stdlib + project source — a yukti-style split (project source in `src/*.cyr`, `lib/` purely deps + gitignored) is a natural future restructure but out of 2.2.2's scope

## [2.2.1] - 2026-04-26 — Cyrius 5.7.7 modernization + distlib bundle

Toolchain catch-up release: spans the full 5.x line in one bite, lands the
distlib-driven distribution model, and brings CI / release / scripts into
line with first-party Cyrius project conventions (yukti / sakshi / patra).
No source changes to the math modules themselves — pure scaffold rework.

### Changed
- **Toolchain bump**: Cyrius **4.10.3 → 5.7.7**. Picks up the manifest format, `cyrius distlib` multi-profile, `cyrius.lock`, `${file:VERSION}` interpolation, `#deprecated("...")` attribute, structured-deps protocol, fixup-table cap 262K → 1M, atomic `cyrius build` output (failed compile no longer destroys an existing binary)
- **Manifest migration**: `cyrius.toml` → **`cyrius.cyml`**. Adds `language = "cyrius"`, `cyrius = "5.7.7"` toolchain pin in `[package]`, and `version = "${file:VERSION}"` interpolation so VERSION is the single source of truth
- **Toolchain pin location**: `.cyrius-toolchain` (legacy) removed — pin lives in `cyrius.cyml [package].cyrius` per Cyrius 5.5.41+ convention; CI/release grep the manifest directly
- **Dep bump**: sakshi **0.9.0 → 2.1.0** (modules path now `dist/sakshi.cyr` per the distlib convention)
- **`scripts/version-bump.sh`**: was editing `Cargo.toml` + running `cargo generate-lockfile` — now just writes `VERSION` (manifest auto-syncs via `${file:VERSION}`); validates semver shape; prints next-step hints for CHANGELOG section + tag
- **`scripts/bench-history.sh`**: was running `cargo bench --bench benchmarks` and parsing criterion output — now runs `cyrius bench tests/hisab.bcyr` and parses `lib/bench.cyr` output; tolerant unit normalization (ps/ns/µs/ms/s); 3-point trend table generation unchanged
- **`CLAUDE.md`**: full rewrite for the actual Cyrius project — replaced Rust idioms (cargo/clippy/RUSTDOCFLAGS, MSRV 1.89, `#[non_exhaustive]`) with the real toolchain (`cyrius lint/fmt/vet/build/test/bench/distlib`); refreshed layout, deps, key principles, CI gates, doc structure
- **`CONTRIBUTING.md`**: prereq updated to Cyrius 5.7.7+ via `cyrius.cyml`
- **`README.md`** quick-start: `cyrius.toml` snippet → `cyrius.cyml` with full `[package]` + first-party `[deps.hisab]` example pointing at `dist/hisab.cyr`
- **`.gitignore`**: dropped Rust-only entries (criterion, proptest, tarpaulin, cargo-vet); added `cyrius-*.tar.gz` (CI download) and `.claude/`
- **`docs/development/dependency-watch.md`**: toolchain status updated; sakshi reclassified as first-party; 5.x line notes added

### Added
- **`[lib]` modules block** in `cyrius.cyml` listing 34 hisab modules in dependency order (foundation → types → derived) — input to `cyrius distlib`
- **`dist/hisab.cyr`** distribution bundle (regenerated from `[lib]` via `cyrius distlib`). Consumers pull it with `[deps.hisab] modules = ["dist/hisab.cyr"]` — single self-contained module, no per-file `include` choreography needed
- **`.github/workflows/ci.yml`** rebuilt: lint, fmt-check, vet, **distlib drift check** (regenerates `dist/hisab.cyr` and fails if committed bundle is stale), build (with ELF magic check), test (auto-discover `tests/*.tcyr`), fuzz (auto-discover `tests/*.fcyr` with 10s timeout per harness), bench (auto-discover `tests/*.bcyr`), docs + version-consistency gate
- **`.github/workflows/release.yml`** rebuilt: CI gate, version-verify (tag matches VERSION; supports both `1.2.3` and `v1.2.3`), distlib regeneration, source tarball + `dist/hisab.cyr` + linux binary + `SHA256SUMS` upload, changelog-section extract for release body
- **`cyrius deps --verify`** lock-gate in CI (gated on `cyrius.lock` presence)

### Removed
- `.cyrius-toolchain` — superseded by `cyrius.cyml [package].cyrius`
- `cyrius.toml` — superseded by `cyrius.cyml`

### Notes
- `lib/collision_mesh.cyr` is included in the `[lib]` bundle. The 2.2.0 changelog flagged it as "exceeds cc3 1MB preprocess buffer, ships with Cyrius 5.0"; cc5 5.7.7 has lifted the relevant caps. If a downstream `cyrius build` of `dist/hisab.cyr` fails on this file, drop it from the `[lib]` list and reopen the deferral
- `lib/` still mixes vendored stdlib + project source. yukti-style separation (project source in `src/*.cyr`, `lib/` purely deps + gitignored) remains a future restructuring decision

## 2.2.0 (2026-04-15) -- Geometry & group extensions

### Added
- **lie_ext.cyr** (523 lines) -- SE(3) rigid body motions (exp/log, compose, transform), SO(3) explicit (Rodrigues, exp/log), adjoint representations (SU(2), Lorentz, SE(3)), Baker-Campbell-Hausdorff 2nd/3rd order
- **spatial.cyr** (864 lines) -- k-d tree (build, nearest, within_radius), quadtree (insert, query), octree (insert, query), spatial hash (insert, query_cell, query_radius, clear)
- **collision_core.cyr** (574 lines) -- MPR/XenoCollide (intersect + penetration), sequential impulse solver with friction, convex hull 2D (Andrew's monotone chain), polygon triangulation (ear clipping)
- **collision_mesh.cyr** (522 lines, written but deferred) -- Delaunay triangulation (Bowyer-Watson), half-edge mesh, island detection (union-find). Exceeds cc3 1MB preprocess buffer. Ships with Cyrius 5.0.

### Changed
- All .cyr files cleaned of Unicode in comments (em-dashes, Greek letters -> ASCII)

### Known issue
- Cyrius cc3 1MB preprocess buffer limit reached at ~16K lines. collision_mesh.cyr deferred to Cyrius 5.0.

## 2.1.0 (2026-04-15) -- Precision + depth

### Added
- **linalg_precision.cyr** (1,124 lines) -- Golub-Kahan SVD (full precision, replaces A^T*A), QR eigendecomposition O(n^3) (replaces Jacobi O(n^5)), complex Householder QR
- **noise_simplex.cyr** (343 lines) -- OpenSimplex2 2D+3D with fBm layering
- **einsum.cyr** (305 lines) -- Einstein summation notation parser (`"ij,jk->ik"`)

### Changed
- Audit M8 (SVD precision) resolved

### Performance
- `eigen_qr`: O(n^3) for symmetric eigenproblems (was O(n^5) with Jacobi for large n)

## 2.0.0 (2026-04-15) -- Cyrius port

**Breaking: complete rewrite from Rust to Cyrius.** New language, new API, new binary format. Rust source available via pre-2.0 git tags.

### Changed
- **Language**: Rust -> Cyrius (self-hosting systems language, static ELF binaries)
- **Types**: glam f32 SIMD types -> native f64 heap-allocated types (HVec2/3/4, HQuat, Mat3, Mat4)
- **Errors**: `Result<T, HisabError>` -> integer error codes (ERR_NONE, ERR_SINGULAR_MATRIX, etc.)
- **API**: method syntax (v.dot(w)) -> free functions (hvec3_dot(v, w))
- **Precision**: f32 (1e-7) -> f64 (1e-12) everywhere
- **Dependencies**: 9 Rust crates -> 1 Cyrius dep (sakshi)
- **Binary**: ~800KB dynamic -> 420KB static ELF

### Added -- 27 library files, 11,943 lines

**Foundation (8 files):** error, f64_util, vec2, vec3, vec4, quat, mat3, mat4
**Transforms (2 files):** transforms (T2D/T3D, Euler, screen), color (sRGB, Porter-Duff, tone mapping, SH, EV)
**Geometry (2 files):** geo (9 primitives, 6 ray tests), geo_advanced (GJK/EPA, BVH, SDF, CGA 5D)
**Calculus (2 files):** calc (integration, Bezier, easing, Perlin), calc_ext (gradient/Jacobian/Hessian, B-spline, NURBS, Hermite, monotone cubic, 3D Perlin)
**Numerical (5 files):** num (roots, FFT, RK4, PCG32, primes), ode (DOPRI45, BDF, symplectic), optimize (GD, CG, BFGS, L-BFGS, LM), linalg_ext (CSR, GMRES, BiCGSTAB, PGS, SVD, eigen, inertia), num_ext (extended GCD, totient, Mobius, factorize, CRT, DST/DCT, 2D-FFT, Halton/Sobol, tridiagonal)
**Physics (3 files):** complex (numbers + matrices, Pauli, Dirac, matrix exp), lie (U(1), SU(2), SU(3), SO(3,1)), diffgeo (Christoffel -> Einstein, geodesics, exterior algebra)
**Symbolic (2 files):** symbolic (expr tree, eval, diff, simplify), symbolic_ext (integration, LaTeX, pattern matching)
**Other (3 files):** autodiff (dual numbers), interval (arithmetic), tensor (N-D dense, contraction, physics tensors)

### Security -- P(-1) audit (31 issues found, 25 fixed)
- Allocation overflow guards (tensor, complex matrix, diffgeo dim cap, sieve cap)
- Division-by-zero guards throughout (complex, autodiff, transforms, depth)
- m4_determinant rewritten with correct cofactor formula
- tensor_contract implemented (was returning zeros)
- BDF-5 coefficients recomputed exact (IEEE 754 verified)
- Bisection midpoint overflow fix, CG upgraded to Polak-Ribiere+
- modpow overflow-safe via Russian peasant multiplication
- expr_eval warns instead of aborting process

### Testing
- 821 assertions (4 suites), 22 benchmarks, 5 fuzz targets

### Documentation
- README, CONTRIBUTING, SECURITY updated for Cyrius
- Architecture overview, testing guide, threat model, dependency watch
- P(-1) audit report, Rust vs Cyrius benchmark comparison
- Working example (examples/basic_math.cyr)

---

## Rust era (archived in pre-2.0 git tags)

### 1.4.0 (2026-03-30) -- Theoretical physics foundation
- Complex linear algebra (ComplexMatrix, Hermitian eigen, complex SVD, Pauli/Dirac, spinors, matrix exp)
- Indexed tensor algebra (Einstein summation, contraction, raising/lowering)
- Symmetric/antisymmetric/sparse tensors
- Lie groups (U(1), SU(2), SU(3), SO(3,1), exponential maps, Casimir)
- Differential geometry (Christoffel, Riemann, Ricci, Einstein, geodesics, Killing, exterior algebra)
- Conformal geometric algebra (5D CGA multivectors)

### 1.3.0 (2026-03-27) -- Number theory + symbolic
- Prime sieves, primality, factorization, modular arithmetic
- Symbolic integration, LaTeX, pattern matching, abaco bridge

### 1.2.0 (2026-03-27) -- Interpolation + color
- Inverse lerp, remap, reverse-Z, HSV/HSL/Oklab, Porter-Duff, compensated summation

### 1.1.0 (2026-03-25) -- Feature completion
- Symplectic integrators, SDFs, DualQuat, SH, stiff ODE, SDE, eigen, reverse-mode AD

### 1.0.0 -- Stable release
- Core: transforms, geo, calc, num, autodiff, interval, symbolic, tensor, parallel, ai, logging
