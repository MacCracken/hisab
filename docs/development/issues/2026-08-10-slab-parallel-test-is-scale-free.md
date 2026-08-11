# The slab parallel test uses an absolute epsilon, so a unit direction can produce a phantom hit

**Filed**: 2026-08-10 (during 2.10.1, by a commissioned adversarial probe)
**Status**: 🐞 **OPEN** — scheduled for **2.10.2**. Not fixed in 2.10.1.
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

## Fix directions, none chosen yet

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
