# hquat_inverse fabricates the identity for any quaternion shorter than 1e-6

**Filed**: 2026-08-11 (P(-1) sweep at v2.11.0)
**Status**: 🟢 **FIXED in 2.11.1** — `src/quat.cyr`
**Severity**: High — silent wrong output; `q · q⁻¹` stops being the identity with no error signal
**Class**: squared-vs-unsquared tolerance — **seventh instance**

## Summary

```
fn hquat_inverse(q) {
    var len_sq = hquat_length_sq(q);
    if (f64_lt(f64_abs(len_sq), EPSILON_F64) == 1) { return hquat_identity(); }
```

`len_sq` is a **squared** magnitude compared against `EPSILON_F64` (1e-12), an **unsquared**
tolerance. The effective threshold is therefore `|q| < 1e-6` — six orders of magnitude coarser than
intended — and inside it the function returns the **identity**, which is a valid quaternion and
passes every sanity check a caller might apply.

## Reproduced

The defining property of an inverse, swept by scale. `q = (s, 0, 0, s)`, magnitude `s√2`, never
zero and always invertible:

| s | `\|q · q⁻¹ − I\|` | |
|---|--:|---|
| 1e-5 | 0.0 | ok |
| 1e-6 | 0.0 | ok |
| **1e-7** | **0.999999900** | ❌ |
| **1e-8** | **0.999999990** | ❌ |
| **1e-9** | **0.999999999** | ❌ |

The error is ~1.0 rather than small because the returned "inverse" is the identity, so the product
is `q` itself — a near-zero quaternion, one unit away from the identity it should be.

A quaternion of magnitude 1.4e-7 has an inverse of magnitude 7e6: finite, exactly representable, and
silently replaced by a rotation of zero.

`hquat_normalize` took the same rule. Its guard was on the **unsquared** length so the units were at
least right, but it too fabricated the identity where the division is well-defined far below 1e-12.

## Fix

```
var _QUAT_F64_TINY = 0x0010_0000_0000_0000;  # 2.2250738585072014e-308
```

Both guards now test against DBL_MIN — *guard exactly what makes the division fail and nothing
more*. That is the rule that settled six thresholds across `geo.cyr` in 2.10.1/2.10.2 and four in
`autodiff.cyr` in 2.11.0, and it is the only one that needs no scale chosen for it.

Constant gate 158 → **159**.

## The class, seven instances deep

| release | site |
|---|---|
| 2.6.14 | `cx_div` / `cx_inv` / `cx_powf` — recorded the lesson in `complex.cyr:58` |
| 2.10.1 | `geo_ray_sphere`, `geo_ray_capsule` (cylinder) |
| 2.10.2 | `geo_ray_capsule` (degenerate), `geo_triangle_unit_normal`, `geo_segment_direction`, `geo_segment_closest_point` |
| 2.11.0 | `dual_div`, `dual_ln`, `dual_sqrt` |
| **2.11.1** | **`hquat_inverse`, `hquat_normalize`** |

The lesson has been written down since 2.6.14. It keeps recurring because **writing a lesson in one
module does not apply it to the other thirty-four** — only a grep does, and the sweep that finally
reached `quat.cyr` was mechanical, not a review.

## ⚠ My own first sweep failed to kill two of three mutants

The scale sweep was written over 12 decades, bottoming out at `|q| ~ 1e-11`. Two mutants survived:
restoring `EPSILON_F64` (which fires at 1e-12 on the unsquared length) and substituting
`_GEO_F64_EPS_SQ` (which fires at 1e-12 on the magnitude). **Both thresholds sit below where the
sweep looked.**

Extended to 20 decades, all three mutants die.

**A sweep must bracket every threshold a repair might plausibly have chosen, not just the one it
did.** This is the same failure mode as 2.10.1's — where 2.10.0's homogeneity sweep used direction
scales of 2, 0.5 and 7 and could not reach a guard the same commit had introduced — and it happened
again here despite that being written down.

⚠ A second discrimination trap in the same fixture, avoided deliberately: asserting only that
`hquat_normalize` returns something of **unit length** does not discriminate, because the fabricated
identity *is* unit. The assertion checks the **direction** — for `q = (s,0,0,s)` the normalized x
component is `1/√2` and the identity's is 0.
