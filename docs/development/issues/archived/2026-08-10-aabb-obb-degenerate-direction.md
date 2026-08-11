# geo_ray_aabb / geo_ray_obb return +Inf for a degenerate ray direction

**Filed**: 2026-08-10 (during 2.10.1 design)
**Status**: 🟢 **FIXED in 2.10.1** — `src/geo.cyr`, both functions
**Severity**: High — silent wrong output on a public entry point
**Class**: sibling asymmetry on a shared contract (third instance; see *Precedent*)

## Summary

A ray whose direction has all three components below `EPSILON_F64` (1e-12), fired from an
origin **inside** the volume, makes `geo_ray_aabb` and `geo_ray_obb` return **+Infinity**.

The module contract is stated three times in `src/geo.cyr` — file header line 5, ray-section
header line 217, and each function's own comment:

> intersection functions return 0 on miss, positive f64 t on hit

`+Inf` is neither. `geo_ray_at(ray, +Inf)` on a zero direction is `0 * Inf` = **NaN in every
component**, so a consumer receives NaN geometry with nothing raised anywhere.

## Measurement

Sweep over all six `geo_ray_*` primitives with `direction = (0, 0, 0)`, run before any 2.10.1
feature code was written:

| primitive | origin INSIDE | origin OUTSIDE | |
|---|--:|--:|---|
| `geo_ray_aabb` | **inf** | 0 | ❌ |
| `geo_ray_obb` | **inf** | 0 | ❌ |
| `geo_ray_sphere` | 0 | 0 | ✅ |
| `geo_ray_plane` | 0 | 0 | ✅ |
| `geo_ray_triangle` | 0 | 0 | ✅ |
| `geo_ray_capsule` | 0 | 0 | ✅ |

Four of six honoured the contract; two did not. Reproducer:
`scratchpad/probe/degen.cyr` (recorded in `tests/modules.tcyr` as a permanent sweep).

⚠ **The origin position is load-bearing.** From outside, the parallel-slab branch's own
`if (f64_lt(o_k, bmin_k)) { return 0; }` fires first and the contract is honoured by accident.
A sweep that only fired rays from outside — the natural way to write one — would have
reproduced nothing.

## Root cause

Both functions skip a slab whose direction component is below `EPSILON_F64`, treating it as
parallel. With **all three** skipped, nothing ever narrows the interval: `t_min` stays
`-Inf` and `t_max` stays `+Inf`. The tail then reads

```
if (f64_lt(t_min, 0) != 1) { return t_min; }   # -Inf IS < 0, so skipped
if (f64_lt(t_max, 0) != 1) { return t_max; }   # +Inf is not < 0 -> returns +Inf
return 0;
```

and hands back the sentinel it initialised with.

Reachable exactly as the 2.10.0 defect was: `geo_ray_new` normalizes, so the constructor path
never produces it, but `GeoRay` is `#derive(accessors)` and **`GeoRay_set_direction` is public**
with no documented unit-length precondition.

## Fix

Count the slabs that actually contributed and return the miss sentinel when the count is zero.

```
var slabs = 0;
...
    slabs = slabs + 1;      # inside each non-parallel branch, all three axes
...
    if (slabs == 0) { return 0; }
    if (f64_lt(t_min, 0) != 1) { return t_min; }
```

⚠ **Why a counter and not a `d.d < EPSILON_F64` guard up front** — which is what
`geo_ray_sphere` uses, and would have been the obvious symmetry. That test compares a **squared**
length against an epsilon chosen for unsquared quantities, so it rejects legitimate directions:
`(0, 0, 1e-7)` has `|d|² = 1e-14 < 1e-12` and would be turned into a miss, even though its z slab
is not parallel (`1e-7 > 1e-12`) and its hit is real, finite and correct. The counter changes
behaviour on exactly the path that produced `+Inf` and on no other input — which is what makes the
3398 pre-existing assertions a valid gate on the repair.

⚠ **That squared-vs-unsquared mismatch is a defect in `geo_ray_sphere`'s own guard**, not just an
argument against copying it: `2.6.14` fixed the identical mistake in `cx_div`/`cx_inv`/`cx_powf`
("a 1e-6 cutoff instead of 1e-12 — squared vs unsquared tolerance"). It is filed separately as
`2026-08-10-squared-epsilon-guards-in-geo-ray.md` rather than folded in here, because it
changes an answer this issue does not.

## Verification

* **Mutation-proven, both guards independently.** Removing the `aabb` guard alone, the `obb`
  guard alone, or both, each fails
  `tests/modules.tcyr` → *"every geo_ray_* returns 0 for a zero-length direction"*.
  Suite 1668 → 1670 in `modules.tcyr`, 3398 → 3400 overall, 0 failures.
* **Cost: not measurable.** Interleaved A/B, three runs each, `bench_batch(2000, 200)`:

  | | with guard | pre-fix |
  |---|--:|--:|
  | `aabb_axis` | 87 / 88 / 88 ns | 89 / 90 / 89 ns |
  | `aabb_diag` | 107 / 112 / 114 ns | 105 / 113 / 108 ns |
  | `obb_axis` | 450 / 456 / 465 ns | 450 / 458 / 460 ns |
  | `obb_diag` | 476 / 483 / 490 ns | 479 / 495 / 495 ns |

  Every pair overlaps; the repair is an i64 increment per slab and one compare. **No
  performance claim is made in either direction** — the ranges do not separate at this
  release's measured 3.5% median noise floor.

## Why it shipped

The pre-2.10.1 suite tested the zero direction on **`geo_ray_sphere` and `geo_ray_capsule`
only** — the two that 2.10.0 had just repaired. The other four were never asked the question.
Fixing a defect in two siblings and asserting the fix on those two siblings is how the same
defect stays alive in the other four.

The 2.10.1 repair is therefore a **sweep over all six**, with a `_DD_RUN` counter that fails if
a primitive is ever silently dropped from it — the same shape as 2.10.0's homogeneity sweep,
for the same reason.

## Precedent

Third instance of the same class, all found by asking one sibling's question of the others:

| release | pair | asymmetry |
|---|---|---|
| 2.9.1 | `mpr_intersect` vs `gjk_intersect_3d` | one was still running the pre-2.9.0 containment test |
| 2.10.0 | `geo_ray_sphere`/`_capsule` vs the other four | degree −1 homogeneity in the direction |
| 2.10.1 | `geo_ray_aabb`/`_obb` vs the other four | the degenerate-direction return value |

## How it was found

By 2.10.1's own design requirement, before any feature code existed. A jet needs to know **which
face** the slab method selected; on a degenerate direction there is no face, so the question
*"what does the primal return there?"* had to be asked — and the answer for two of six was
`+Inf`.

That is the second consecutive release where running the feature's correctness requirement
against the **shipped** code found a defect before the feature was written. It is recorded in
`docs/doc-health.md` as a standing move, not a coincidence.
