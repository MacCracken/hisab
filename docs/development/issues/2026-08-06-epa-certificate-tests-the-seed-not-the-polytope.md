# EPA certification is ASYMMETRIC between `gjk_epa_3d` and `mpr_penetration` — the seed differs, not the certificate

*(Original title, retained because it is what the body argues and what 2.9.3 refuted: "EPA's certified exit is never taken — `strict` tests the SEED tetrahedron, but the lower-bound argument is about the FINAL polytope".)*

**Status:** 🟡 **OPEN — but RE-DIAGNOSED 2026-08-09 (2.9.3), and BOTH proposed repairs are now
measured and rejected.** Read the *Re-diagnosis* section before the body: this file's central claim
is wrong, and its Proposed fix produces incorrect depths.
**Placement:** still wants its own cycle, but the cycle is now about the *sphere family*, not the
certificate — see Re-diagnosis §3.
**Discovered:** 2026-08-06, closing the 2.9.0 mutation sweep's U6–U11 "coverage gap".
**Severity:** Low, downgraded from Medium. Not a wrong answer, and **not** a dead code path either
(that was the error) — it is an inconsistency in *which* path two public entry points take to reach
the same answer.

## Re-diagnosis (2.9.3)

### 1. ⛔ "The certified exit is never taken" is FALSE — it only measured one entry point

The 12/12 fallback below is real for `gjk_epa_3d`. It is **0/12 for `mpr_penetration`**, which
computes the same quantity through the same `_epa_refine`:

```
gjk_epa_3d      box polish increments:  12   (certifies none)
mpr_penetration box polish increments:   0   (certifies all 12, returns 1 on all 12)
```

So the certified path is not unreachable, `_epa_polish` is not "the primary path", and the
**"Why this explains U6–U11" section is not established** — the exit those six repairs improve does
run, on every `mpr_penetration` overlap.

The cause is the **seed**, not the certificate. `_epa_seed_portal` tries a strictness upgrade — swap
`v0` for the support point opposite the far face, keep it only if the result strictly encloses the
origin (`geo_advanced.cyr:757-766`) — and `_epa_seed_gjk` never did. The portal path therefore
arrives certifiable and the GJK path arrives one rounding step short. **The real defect is a
certification asymmetry between two public entry points.**

### 2. ⛔ The Proposed fix below produces WRONG DEPTHS — measured

"Drop the seed test and rely on `lo > 0`" was implemented. Boxes go **12 → 0** fallbacks, as this
file predicts. But **four exact-tangency pairs then report a nonzero depth for a touch whose true
depth is 0** — `_ag_baddepth` in the 110-pair agreement sweep goes **0 → 4**. `lo > 0` is therefore
*not* the property the certificate needs: at exact tangency `lo` can be marginally positive from
rounding, and the seed test was the thing catching it. **The seed test is load-bearing.** Do not
apply this file's Proposed fix.

### 3. The cheaper alternative works but costs 19%, and is not obviously worth it

Giving `_epa_seed_gjk` the portal's strictness upgrade: **every assertion in the suite passes**
(including every exact-MTV depth on both entry points) and boxes go **12 → 6** fallbacks. But it adds
one Minkowski support evaluation per seed and measures **+19.4% on `gjk_epa_sphere_box`** — 150.4 µs
against 125.6 µs, three runs each, non-overlapping, well clear of the ~7% noise floor. Spheres pay
the extra support call and *still* fall back, which points at a separate root cause in the sphere
family (they do not certify even with a strict seed).

**No returned answer changes either way** — the polish already recovers the depth, and the two entry
points agree to 1e-9. So the trade is: pay 19% on a narrowphase hot path to make an internal code
path consistent between entry points and to make six repairs mutatable. That is a judgement call
rather than a defect fix, and 2.9.3 did not take it. **The code is unchanged.**

### What is actually left

The sphere-family non-convergence (§3) is the open question, and it is independent of the
certificate wording. Anything done here still needs the exact-MTV bar — which is *in* the suite now
(`collision_core: narrowphase depth == exact MTV`, plus the 110-pair agreement sweep), so a future
attempt can be scored in-tree rather than against an external harness.
**Affects:** hisab 2.8.3 onward.

## Summary

`_epa_refine` sets `certified = 1` only when

```
if (f64_gt(lo, 0) == 1 && strict == 1) { certified = 1; }     src/geo_advanced.cyr:906
```

`strict` is `_epa_tetra_strict(seed)` (`:853`), evaluated **once, on the GJK seed tetrahedron**,
requiring all four face gaps to exceed `EPSILON_F64 * lmax`.

**That condition never holds in practice.** Measured over 24 overlaps across two shape families —
boxes and spheres, swept from shallow to deep — the certified exit was taken **0 times**. Every
call fell back to `_epa_polish`.

So the polish is not a fallback. It is the primary path, and the EPA refinement loop is a seed
generator for it.

## The measurement

`_GA_EPA_POLISH_COUNT` (added 2.9.1) counts handoffs. With the shipped condition:

| fixture family | calls | fell back to polish |
|---|---|---|
| box/box, 12 offsets | 12 | **12** |
| sphere/sphere, 12 radii | 12 | **12** |

Dropping **only** `strict` from the certificate — one edit, `strict == 1` removed, `lo > 0` kept:

| fixture family | calls | fell back to polish |
|---|---|---|
| box/box, 12 offsets | 12 | **0** |

So `lo > 0` is satisfied routinely and `strict` is the sole blocker. (Mutation applied and
reverted; `src/geo_advanced.cyr` md5-verified back to `176ae917fb2142a3de5aa75589574623`.)

## Why it never holds

GJK terminates as soon as its simplex **contains** the origin — which routinely leaves the origin
within rounding distance of a face. `_epa_tetra_strict` then demands a margin of
`EPSILON_F64 * lmax` on all four faces of that same simplex, and rejects it.

The deeper problem is the **object being tested**. The comment at `:903-905` states the reasoning:

> The same applies when the SEED was not strictly enclosing: the lower-bound argument assumes the
> origin is inside the polytope, so a seed that only just contains it cannot certify anything.

But the lower-bound argument is about the polytope **at the moment of certification**, not about
the seed. EPA only ever *adds* vertices, so an expanded polytope encloses the origin at least as
well as the seed did. Testing the seed's margin is therefore both the wrong object and needlessly
conservative — it condemns every run for a property of its starting state.

## Why this explains U6–U11

The 2.9.0 mutation sweep found six EPA repairs whose reversion produced **bit-identical** output,
and could not classify them as coverage gaps. This is why: the exit those repairs improve is
unreachable, so nothing downstream of it can be observed. It was not that the fallback *sometimes*
masked them — the certified path never runs at all.

## Proposed fix

Evaluate strict enclosure against the **current** polytope at the point of certification rather
than against the seed, or drop the seed test and rely on `lo > 0` (which is itself the
positive-lower-bound condition the argument needs).

**Do not land this without the exact-reference harness.** Certification changes which value is
returned — the certified EPA depth instead of the polished one — on every overlapping pair. That is
a narrowphase behaviour change and must be scored against the 862-configuration exact-MTV reference
(15-axis OBB SAT in `Fraction`, closed forms for spheres) that 2.8.3 established, with the same
zero-wrong-sign bar. It should also be benchmarked: skipping the pattern search on every overlap is
plausibly a real win, and this repo does not accept a performance claim without before/after
numbers.

## Note

2.9.1 pins the current state in `tests/modules.tcyr` — assertions that all 12 box and all 12 sphere
overlaps fall back, plus a negative control proving the counter is not unconditional. **When this
is fixed those assertions will FAIL, and that is the intended signal.** Update them then and say so
in the CHANGELOG.
