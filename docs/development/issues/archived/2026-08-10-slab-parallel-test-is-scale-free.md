# The slab parallel test uses an absolute epsilon, so a unit direction can produce a phantom hit

**Filed**: 2026-08-10 (during 2.10.1, by a commissioned adversarial probe)
**Status**: 🟢 **FIXED in 2.10.2** — `src/geo.cyr`, both slab routines
**Severity**: Critical — a positive `t` returned for a ray that misses, with a hit point far off
the surface, reachable through `geo_ray_new` with a **unit-length** direction
**Affects**: `geo_ray_aabb` / `geo_ray_aabb_face`, `geo_ray_obb` / `geo_ray_obb_face`, and
therefore `geo_jet_aabb` and `geo_jet_obb`

## Summary

Both slab routines skip an axis whose direction component fails an **absolute** threshold —
`f64_lt(f64_abs(dx), EPSILON_F64)` for the AABB, `f64_gt(f64_abs(f0), EPSILON_F64)` for the OBB,
with `EPSILON_F64 = 1e-12`. Skipping means "treat this slab as parallel and only bounds-check the
origin against it."

That is wrong whenever the ray travels far enough for a tiny direction component to matter. Over
`t ~ 1e12`, a component of `9e-13` moves the coordinate by ~0.9 — most of a unit box. The slab that
would have rejected the ray is deleted, and the routine returns a confident hit.

**The threshold is scale-free; the consequence is not.** What matters is `|d_k| * t_range` against
the box extent, and none of those appear in the test.

## Measurement — reproduced independently on the 2.10.1 tree

```
box   = geo_aabb_new((0,0,0), (1,1,1))
ray   = origin (0.5, 0.5, -1e12), direction (9e-13, 0, 1)
|d|   = 1.000000            <- an ordinary unit direction

geo_ray_aabb        -> t = 1000000000000.0
geo_ray_at(ray, t)  -> x = 1.400000        <- the box spans x in [0, 1]
geo_ray_aabb_face   -> face = 4 (min-z), tie = 0
geo_jet_aabb        -> hit = 1, a full gradient on a face the ray never touched
```

The commissioned probe reported two further symptoms of the same root cause, both re-run here:

* **Wrong finite `t` from inside.** Box `(0,0,0)-(1,1,1e13)`, origin `(0.5,0.5,0.5)`,
  `d (9e-13, 0, 1)`: returns `t = 1e13` where the true exit is `5.5556e11` — **18× too large**,
  hit point 8.5 outside on x. Strictly worse than the phantom hit: a plausible finite positive `t`
  on a genuine hit, so no "did we get a hit" sanity check catches it.
* **The OBB agrees.** Centre `(0,0,1e13)`, `he (1,1,1)`, `geo_ray_new((0,0,0), (5e-13,0,1))` —
  which **normalizes**, and still returns a hit whose local x is 5.0 for a half-extent of 1.

## Why this is not the 2.10.1 degenerate-direction repair

2.10.1 added `if (slabs == 0) { return 0; }`, which closes the case where **every** slab is
skipped. Its header says the counter "changes behaviour on exactly the path that produced `+Inf`
and nowhere else" — that claim is true of the counter and **irrelevant here**. The per-component
absolute test that deletes a *single* slab predates the counter and is untouched by it. This is a
different defect that happens to live next door.

## Related, same file, filed together because a fix should consider them as a set

* **AABB and OBB disagree at exactly `EPSILON_F64`.** The AABB takes the parallel branch on
  `f64_lt(|dx|, EPS)` so exact equality falls through to the **slab** branch; the OBB takes the
  slab branch on `f64_gt(|f|, EPS)` so exact equality falls through to the **parallel** branch. The
  same box expressed both ways, one ray with `d_x` equal to the literal `EPSILON_F64` bit pattern:
  `geo_ray_obb` returns `t = 4.5e12` (hit point 3.5 off the face), `geo_ray_aabb` returns 0.
* **`+Inf` from finite inputs, unrelated to the zero-direction path.** Box centre `(0,0,1e300)`,
  `d (0,0,2e-12)` — non-zero and above the guard — returns `+Inf`. Over a 20,000-config fuzz the
  probe recorded 305 non-finite returns, **230 of them with every input finite** (1.15%).
* **NaN laundering.** With `d = (NaN, 0, 1)` on `[0,1]³` the AABB returns `t = 1.0` — a
  normal-looking positive `t` with a NaN hit point — because `f64_max` erases a NaN `t_min` when a
  later slab wins (`f64_gt(NaN, x)` is 0). Whether NaN propagates or is laundered into a plausible
  number depends on which axis carries it.
* **Negative half-extents make the same OBB solid or empty depending on the ray.** `he = (-1,-1,-1)`
  with an oblique ray returns a `t` **byte-identical** to `he = (+1,+1,+1)` (the slab swap
  rehabilitates it into a box of size `|he|`), while an axis-parallel ray returns 0 (the parallel
  branch's `-e - he > 0` test rejects). One box, two answers.

## Why it is not fixed in 2.10.1

Every item above changes what the **primal** returns, on inputs where it currently returns
something. That is a behaviour change to the six most-used functions in the module, with a
benchmark cost to measure and four consumers downstream — not something to fold into a release
whose scope was the jets, and not something to decide without measuring what a scale-aware test
costs on the hot path (`ray_aabb` is 92 ns).

⚠ **2.10.1's jets inherit all of it**, and that is stated in the CHANGELOG rather than left
implicit: `geo_jet_aabb` and `geo_jet_obb` return `tie = 0` and a full gradient on the phantom hit
above. The jets are a post-pass on the shipped primal by design, so they are exactly as correct as
it is — which is the honest cost of that design and the reason it is written down.

## Fix — option 2, measured

The guard's only real job is to avoid a division that yields no usable reciprocal, so the
threshold is now exactly that: **`_GEO_F64_TINY` = DBL_MIN**, the smallest magnitude whose `1/x` is
finite. Every direction component that *can* be divided by now is.

```
var _GEO_F64_TINY = 0x0010_0000_0000_0000;   # 2.2250738585072014e-308
```

`geo_ray_aabb_face`: `f64_lt(f64_abs(dx), EPSILON_F64)` → `f64_lt(f64_abs(dx), _GEO_F64_TINY)`
`geo_ray_obb_face`: `f64_gt(f64_abs(f0), EPSILON_F64) == 1` →
`f64_lt(f64_abs(f0), _GEO_F64_TINY) != 1` — the **negation of the AABB's test rather than the more
natural `f64_gt` form**, so the two are textually identical and "they agree at the boundary" is
checkable by reading. A mutant swapping it to `f64_gt` survives, and that is not a coverage gap:
the two differ only at `|f|` exactly DBL_MIN, where `1/f = 4.5e307` makes the slab interval so wide
it constrains nothing — which is what the parallel branch also expresses.

Plus a tail check that the returned `t` is not `±Inf`, because an overflow can produce one from
entirely finite inputs. ⚠ **NaN is deliberately left to propagate.** The first draft used a
finiteness test, which rejects NaN too — and that broke `abuse.tcyr`'s *"a NaN ray yields NaN, not
a fabricated hit distance"*, an assertion 2.9.0 made with a written reason: the miss sentinel is 0,
so turning a NaN ray into 0 reports "no hit" for an input the caller has no business supplying,
while a NaN return is unmistakable at the call site. **The suite caught this repair overturning a
considered contract decision in passing.** The remaining NaN item — a NaN in *one* axis erased by
`f64_max` and laundered into a plausible finite `t` — is real, module-wide (sphere, plane and
triangle behave the same), and left open.

Constant gate 156 → **157 verified**.

## Verification

| symptom | before | after |
|---|--:|--:|
| phantom hit, `\|d\| = 1.0` | `t = 1e12`, hit x = **1.4** | **0** |
| interior exit | `t = 1e13`, hit x = **9.5** | `t = 5.5556e11`, hit x = **1.0** |
| aabb vs obb at exactly `EPSILON_F64` | `0` vs **`4.5e12`** | `0` vs `0` |
| `+Inf` from finite inputs | **`+Inf`** | **0** |
| five controls | — | **unchanged** |

The permanent assertion is a **sweep over direction-component magnitude**, 18 decades × 2 drift
factors × 2 primitives, checking that every reported hit point lies on its box and that the AABB
and the OBB agree. ⚠ **Its first draft was all misses** — with a reach of `1/tiny` the x drift is
exactly one box-width, so every sample exited on x and 72 checks asserted nothing about a hit
point. The `_SF_HITS` counter is what said so; a second drift factor of `1/(4·tiny)` lands inside.
*A structural invariant about hit points is vacuous over a sample of misses.*

**Three of four mutants killed**: reverting the AABB threshold (3 failures), reverting the OBB
threshold (1), removing the `±Inf` tail check (1). The fourth is the boundary-sense equivalence
above.

**Cost, interleaved A/B, three pairs**: `ray_aabb` +1.1% (inside the 3.5% noise floor),
`ray_aabb_diag` **+2.9%** (A higher in 3 of 3 — it reaches the tail check on every call),
`ray_obb` −0.6%. ⚠ Written as a **function** first, the `is_inf` test cost 5 ns of a 90 ns routine
(+5.6%); inlined at its four use sites it costs the above. Suite 3453 → **3460**.

## Fix directions considered

1. **Scale the threshold.** Compare `|d_k|` against something derived from the box extent and the
   surviving `t` interval rather than a fixed 1e-12. Needs a decision about what "the ray's scale"
   means when the caller supplies a non-unit direction.
2. **Never skip a slab.** With `|d_k| > 0` the interval `(b - o)/d_k` is finite and correct; the
   guard exists only to avoid dividing by exactly zero. Cheapest to reason about; needs measurement
   of the `+Inf`/overflow behaviour it exposes at the extremes.
3. **Validate the result.** Check the returned hit point against the box before returning. Robust,
   but it is a second correctness criterion bolted onto the first, and it costs on every call.

Option 2 is the most likely and the easiest to prove equivalent on ordinary input; it needs the
overflow cases in the list above answered first.

## Provenance

Found by an independent adversarial probe commissioned to check 2.10.1's own jet formulas — which
it confirmed term for term (37 partials, 0 of 24,237 finite-difference comparisons failing). Its
value was entirely in the cases the design had not enumerated. Every finding above was re-run on
this tree before being written down.
