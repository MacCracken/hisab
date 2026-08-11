# geo_ray_sphere and geo_ray_capsule test a SQUARED length against an unsquared epsilon

**Filed**: 2026-08-10 (during 2.10.1)
**Status**: 🟢 **FIXED** — two sites in 2.10.1, four more in **2.10.2** (see the corrected table below)
**Severity**: High — silent wrong output; one of the two returns a *wrong number*, not a sentinel
**Class**: squared-vs-unsquared tolerance (second instance; `complex.cyr` was the first, 2.6.14)

## Summary

Two guards in `src/geo.cyr` compare a **squared** length against `EPSILON_F64`, which is a
tolerance for **unsquared** quantities (1e-12). The effective threshold is therefore 1e-6 on the
length, six orders of magnitude coarser than intended, and both guards fire on inputs whose
answers are real, finite and exactly representable.

| site | guarded quantity | what it did wrong |
|---|---|---|
| `geo_ray_sphere` | `a = d·d` | reported a **MISS** for any \|d\| < 1e-6 |
| `geo_ray_capsule` | `a = d_perp·d_perp` | silently answered with a **CAP** hit instead of the cylinder hit |

The capsule one is the worse of the two: it does not return a sentinel a caller could test, it
returns a plausible number that is wrong by **0.942%**.

## Measurement

Degree −1 homogeneity — `t(k·d)` must equal `t(d)/k` — swept over all six `geo_ray_*` primitives
at direction scales from 1e0 down to 1e-11. Relative error; `999` marks a real hit turned into a
miss:

| k | sphere | plane | triangle | aabb | obb | capsule |
|---|--:|--:|--:|--:|--:|--:|
| 1e-5 | 0 | 0 | 0 | 0 | 0 | 0 |
| 1e-6 | 0 | 0 | 0 | 0 | 0 | **0.00942** |
| 1e-7 | **999** | 0 | 0 | 0 | 0 | **0.00942** |
| 1e-8 | **999** | 0 | 0 | 0 | 0 | **0.00942** |
| 1e-9 | **999** | 0 | 0 | 0 | 0 | **0.00942** |

Four of six exact at every scale; the same two that failed 2.10.0's homogeneity check fail this
one. A concrete case: a unit sphere at `(0,0,5)`, ray from the origin with `d = (0,0,1e-7)`. The
answer is `t = 4e7`. `geo_ray_aabb`, `geo_ray_triangle` and `geo_ray_plane` on the equivalent
geometry all return it. `geo_ray_sphere` returned **0**.

The capsule's constant 0.00942 across every scale below the threshold is the tell that it is a
**branch switch**, not a precision loss: below 1e-6 the cylinder test is rejected and the answer
comes from the end-cap spheres instead, and 0.942% is the geometric distance between those two
surfaces for that ray.

## Provenance — one is mine, from last release

* **`geo_ray_sphere`'s guard did not exist before 2.10.0.** `git log -S` puts it in `b0783ad`
  ("repair geo sphere") — the commit that fixed the unit-direction defect. **2.10.0's repair
  introduced this one.** Carrying `a` explicitly means dividing by it, so a guard was correctly
  added; the threshold was wrong.
* **`geo_ray_capsule`'s cylinder guard is older** — `3fc1959` ("part a and b for 2.7"), the 2.7.x
  parallel-axis repair. It has been returning cap hits for near-axial rays since then.

## Fix

A named constant, so the constant gate verifies it:

```
var _GEO_F64_EPS_SQ  = 0x3AF3_57C2_99A8_8EA7;  # 1e-24
```

`geo_ray_sphere`: `f64_lt(a, EPSILON_F64)` → `f64_lt(a, _GEO_F64_EPS_SQ)`
`geo_ray_capsule`: `f64_lt(f64_abs(a), EPSILON_F64)` → `f64_lt(f64_abs(a), _GEO_F64_EPS_SQ)`

Constant gate 155 → **156 verified**.

## Verification

* All six primitives now exact (relative error 0.000000) at every scale down to **1e-11**.
* **Mutation-proven independently.** Reverting the sphere guard alone → 2 bad of 18; the capsule
  alone → 2 bad; both → 4 bad. The two are independent defects with a shared root cause, not one
  propagating into the other — the sweep's ray hits the capsule's cylinder **body**, so it never
  reaches the cap branches that call the sphere.
* Suite `modules.tcyr` 1670 → **1672**, whole suite **3402**, 0 failures.

## ⚠ Why the 2.10.0 sweep could not have caught it

2.10.0 added an oracle-free homogeneity sweep for exactly this property. It scales the direction
by **k = 2, 0.5 and 7**. Every one of those is of order 1; none comes within five orders of
magnitude of the guard threshold the same release introduced.

So the assertion written to protect a property, and the guard that breaks that property, shipped
in the same commit without ever meeting. **The fixture question that `doc-health.md` now asks of
every new test — "what mutant does this actually kill?" — would have caught it: the 2.10.0 sweep
kills a formula error and cannot kill a threshold error.** The 2.10.1 sweep brackets the threshold
deliberately (1e-5 above, 1e-7 and 1e-9 below), so a fix that merely moved it a decade still fails.

## Site sweep — every squared-vs-EPSILON_F64 comparison in `src/`, with a disposition

Found by grep after the two above were confirmed, because a defect class is worth sweeping once:

| site | quantity | disposition |
|---|---|---|
| `geo.cyr` `geo_ray_sphere` | `d·d` | **FIXED 2.10.1** — returned a miss for a real hit |
| `geo.cyr` `geo_ray_capsule` cyl | `d_perp·d_perp` | **FIXED 2.10.1** — returned a wrong t |
| `geo.cyr` `geo_ray_capsule` degenerate | `ab·ab` | 🟢 **FIXED 2.10.2** — and the 2.10.1 disposition below was WRONG. The reasoning below said the two answers "differ by less than the capsule's own length". Measured, they do not: capsule `(0,0,0)-(1e-7,0,0)` with `r = 1e-8`, ray from `(5e-7,0,0)` along `-x`, returns `t = 4.9e-7` against a true `3.9e-7` — **25.6% error**. Scale covariance holds from `s = 1e0` down to `1e-6` and then collapses to a miss at `1e-7`. |
| `geo.cyr` `geo_triangle_unit_normal` | `len_sq` | 🟢 **FIXED 2.10.2** — and the 2.10.1 disposition below was WRONG. Called a "degeneracy policy" below; it is a **fabricated answer**. A right triangle with 1e-7 legs in the xy-plane returns **(0, 1, 0)** where the truth is (0, 0, 1) — a well-formed triangle, a wrong normal, no error signal. Re-measured on this tree. |
| `geo.cyr` `geo_segment_direction` | `len_sq` | 🟢 **FIXED 2.10.2** — returned a fabricated `unit_x` below 1e-6. ⚠ An x-aligned test segment does NOT discriminate: the fallback *is* the right answer there. The sweep uses a diagonal. |
| `geo.cyr` `geo_segment_closest_point` | `len_sq` | 🟢 **FIXED 2.10.2**, and it was the most load-bearing of the four because it is used as an ORACLE. `geo_segment_distance_to_point` builds on it and returns `1e-8` where the truth is 0 for a segment of length 1e-7. The commissioned probe reached for it as a reference while checking the capsule and was silently misled by it. |
| `quat.cyr:102` | `len_sq` | **Left** — same, returns identity |
| `geo_advanced.cyr:1056/1060/1067` | `hvec3_length_sq` | **DEFERRED, not dismissed.** These are EPA's degenerate-face guards. Tightening them is a narrowphase behaviour change on a routine with a measured cost and a live open filing (`epa-certificate`), and it needs its own before/after. Filed as a 2.10.2 item on the roadmap rather than folded in here. |

⚠ **THE DISPOSITION TABLE ABOVE WAS ITSELF AUDITED, AND TWO OF ITS ROWS WERE WRONG.** They were
written from reading the code — "returns `unit_y`, that's a policy" — not from running it. An
independent probe commissioned a few hours later measured both and found fabricated answers on
well-formed input. Corrected in place above, and scheduled for 2.10.2.

**All four are now repaired (2.10.2), at `_GEO_F64_TINY`** rather than
`_GEO_F64_EPS_SQ` — the same rule the slab repair landed on: *guard exactly what makes the DIVISION
fail and nothing more*. That is stricter than squaring the epsilon and it is the one threshold that
does not need a scale to be chosen for it. The permanent assertion is a **scale-covariance sweep**
— the same scene in different units must give the same answer — over 4 properties × 10 decades,
because that is the only shape of test a bad threshold cannot pass. Each of the four is
independently mutation-proven.

That is the 2.9.3 lesson recurring inside a single release: *a filing is not evidence until
something has re-run it*, and a **disposition** is a filing. Sorting nine sites into "fixed" and
"fine" by inspection produced two wrong sorts out of nine. The two rows that were checked by
measurement are the two that were fixed.

## Precedent

`src/complex.cyr:58` carries the note from **2.6.14**: *"this tested `norm_sq < EPSILON_F64`,
mixing a squared quantity with an unsquared tolerance"*, fixed there with
`f64_mul(EPSILON_F64, EPSILON_F64)` at line 129. The idiom existed, the lesson was written down,
and the geometry module never got the sweep. That is the recurring shape: **a lesson recorded in
one module is not a lesson applied across the tree until something greps for it.**
