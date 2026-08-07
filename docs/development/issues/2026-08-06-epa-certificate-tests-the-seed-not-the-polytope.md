# EPA's certified exit is never taken — `strict` tests the SEED tetrahedron, but the lower-bound argument is about the FINAL polytope

**Status:** 🟡 **OPEN** — diagnosed and measured in 2.9.1, deliberately not fixed there.
**Placement:** wants its own cycle. The fix changes narrowphase certification behaviour on a hot
path, so it needs the 862-configuration exact-reference harness, not a patch release.
**Discovered:** 2026-08-06, closing the 2.9.0 mutation sweep's U6–U11 "coverage gap".
**Severity:** Medium. Not a wrong answer — `_epa_polish` recovers the result — but it makes a whole
code path unreachable, costs a pattern search on every overlap, and made six repairs untestable.
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
