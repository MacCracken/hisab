# `_col_in_circumcircle` loses the sign on mixed-magnitude inputs

**Status:** CONFIRMED as a property of the predicate in isolation. **Not fixed, and 2.9.1 recommends
leaving it** — see **Re-measurement (2.9.1)** at the bottom, which also retires a stale premise in
the CORRECTION section below.

## What was claimed

The 2.7.0-I performance review reported that all 23,741 triangle retirements flagged by a candidate
Delaunay sweep predicate were re-checked in exact rational arithmetic and **zero** were unsound —
i.e. the divergences came from the *shipped* `_col_in_circumcircle` being wrong, not from the
optimisation. It attributed this to "near-coincident clusters".

## What is actually true — the claim needed narrowing

Measured against exact rational arithmetic (`fractions.Fraction`, every operand converted exactly
from its f64 bit pattern), comparing the shipped predicate's 0/1 answer:

| Configuration | Disagreements |
|---|---|
| Uniformly small near-coincident clusters, scale 1e-11 … 1e-6, plus unit-scale controls | **0 of 30** |
| **Mixed magnitude** — one vertex at 1e3 … 1e7 (super-triangle scale) with two near-coincident vertices at 1e-10 … 1e-7 | **3 of 40** |

So "near-coincident clusters" is not the trigger on its own. The predicate translates all points to
the query point first, which is the well-conditioned formulation and handles a uniformly small
cluster correctly. What breaks it is the **magnitude spread**: `a_sq` for the far vertex is ~1e14
while the terms it is differenced against are ~1e-14, so the determinant is a difference of
quantities separated by ~28 orders of magnitude and every significant bit of the result is lost.

This configuration is not exotic in this codebase — `delaunay_2d` starts from a super-triangle whose
vertices are deliberately far outside the data, so **every early insertion is a mixed-magnitude
test**. Any input with a tight cluster hits it.

## Consequence

`f64_gt(det, 0)` on a determinant whose true magnitude is below its own rounding error returns an
arbitrary answer, so Bowyer-Watson can accept or reject a triangle inconsistently within a single
insertion. That produces a triangulation that is not Delaunay, and can produce a non-triangulation.

## Repro

`2026-08-04-incircle-repro.tcyr` in this directory — generated cases, each compared against the
exact rational result. Regenerate with the script in the 2.7.0-J session, or extend the seeds.

## Remedy

An adaptive exact predicate (Shewchuk's `incircle`): compute the floating-point determinant with a
forward error bound, and fall back to exact expansion arithmetic only when |det| is below that
bound. That is the standard fix and it keeps the fast path fast. A cheaper stopgap — treating
|det| < bound as "not inside" — makes the answer *consistent* rather than *correct*, which is
enough to stop Bowyer-Watson producing a non-triangulation but does not make the output Delaunay.

**Do not** attempt to fix this by rescaling: the magnitude spread is inherent to the super-triangle
construction, so the only sound answers are an exact predicate or a different initialisation.


---

## CORRECTION (same session, after the above was written)

The write-up above says *"`delaunay_2d` starts from a super-triangle placed far outside the data,
so **every early insertion** is a mixed-magnitude test."* **That is wrong**, and the severity here
was overstated.

`delaunay_2d` builds its super-triangle at `d_max = max(dx, dy) * 10` — only **10x** the data
extent, not "far outside" it. Re-measured against exact rational arithmetic using that actual
geometry (span 1, super-triangle 10, cluster shrunk to force the spread):

| cluster spread vs span | wrong of 30 |
|---|---|
| 1e3 | 0 |
| 1e5 | 0 |
| 1e7 | 0 |
| 1e9 | 0 |
| 1e11 | 0 |
| 1e13 | 0 |

**0 of 180.** The 3-of-40 result that opened this file used a synthetic configuration with
*absolute* coordinates spanning 1e7 down to 1e-10; the failure depends on the absolute magnitude of
the far vertex (it enters the determinant squared, as `a_sq`), not on the ratio alone. Under the
geometry `delaunay_2d` actually constructs, the dominant `a_sq * cross(b,c)` term carries the sign
and the predicate agrees with exact arithmetic.

**Revised disposition.** The predicate is genuinely non-robust — 3 of 40 synthetic cases is a real
wrong answer — but it is **not** reachable through `delaunay_2d`'s own construction at any spread
tested. It is reachable by a caller invoking `_col_in_circumcircle` directly on far-apart
coordinates, which is a private function. Severity drops from "red, Delaunay output already
suspect" to "latent, no known reachable path". The adaptive-predicate work is still the correct
remedy if this is ever exposed publicly or the super-triangle constant changes.

**Method note.** The first probe generated coordinates by magnitude class rather than by
reconstructing the caller's actual geometry. Measuring the *construct under test* rather than an
abstraction of it is what changed the answer.

---

## Re-measurement (2.9.1)

### The CORRECTION section's premise is now stale

It says *"`delaunay_2d` builds its super-triangle at `d_max = max(dx, dy) * 10`"*. **There is no
super-triangle on this tree.** 2.8.2 replaced it with the three symbolic ghosts at infinity, which
have no coordinates at all (`collision_mesh.cyr:58-119`). The 0-of-180 table was measured against
geometry that has not existed for three minor versions, so it could not be inherited and had to be
re-run.

The replacement makes the reasoning *stronger*, not weaker. `_col_in_circumcircle` now has exactly
one caller in `src/` — `_col_dl_incircle:353`, on the `g == 0` branch — which is reached only when
all three triangle vertices are **real input points**. The query point is likewise always a real
input point. So the mixed-magnitude configuration that produced 3 of 40 is no longer merely
unreached: **no vertex at super-triangle scale exists to construct it with.**

### Re-measured on this tree

Every call `delaunay_2d` can make is a quadruple of input points, so enumerating *all* quadruples
of a fixture is a strict superset of its call set. The shipped predicate was transcribed
operation-for-operation into IEEE-754 doubles and compared against exact integer arithmetic
(coordinates scaled by a common power of two, so the reference is exact, not merely higher
precision). Fixtures are the suite's own, regenerated bit-identically from the same splitmix64
seeds:

| fixture | quadruples | wrong |
|---|---|---|
| cluster-1e-20 (span 1e-20, seed 1) | 365,560 | **0** |
| thin-strip-1e6 (flat 1e-6, seed 1) | 365,560 | **0** |
| uniform-160 (span 1, seed 2; first 48 points) | 778,320 | **0** |
| uniform-20 (literal hex fixture) | 19,380 | **0** |
| **total** | **1,528,820** | **0** |

Points exactly *on* the circumcircle are excluded (the predicate is strict, so agreement there is a
convention question, not a precision one); zero degenerate collinear triples occurred.

### Disposition: leave it

**0 of 1,528,820** — four orders of magnitude more evidence than the 0-of-180 this replaces, on
geometry that actually exists. The adaptive Shewchuk predicate would add expansion arithmetic and a
forward error bound to the hottest inner loop of `delaunay_2d` to fix nothing measurable. The
remedy section above remains correct about *what* the fix would be; it is the trigger that is now
absent.

**Reopen this if** `_col_in_circumcircle` is made public, or if it acquires a caller that can pass
coordinates not drawn from a single input set. Its non-robustness **in isolation** is real and
unchanged — re-measured directly, with a far vertex and a tight cluster:

| far vertex | cluster | wrong |
|---|---|---|
| 1e3 … 1e7 (the original range) | 1e-7 … 1e-12 | **36 of 400** |
| 1e6 … 1e12 | 1e-7 … 1e-12 | 148 of 400 |
| 1e10 … 1e18 | 1e-7 … 1e-12 | 177 of 400 |
| 1e14 … 1e22 | 1e-7 … 1e-12 | 198 of 400 |

The first row is the same order as the 3-of-40 that opened this file (9% vs 7.5%). Recorded because
a first attempt at 40 samples returned **0 of 400-scaled-down-to-40** and would have read as a
refutation; it was sample size, not a real disagreement, and the sweep above is what settles it.
What changed in 2.9.1 is not the predicate — it is that the construct under test can no longer
build that input.
