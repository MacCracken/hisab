# `_col_in_circumcircle` loses the sign on mixed-magnitude inputs

**Status:** CONFIRMED and characterised. **Not fixed** — the proper remedy is an adaptive exact
predicate, which deserves its own focused change.

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
