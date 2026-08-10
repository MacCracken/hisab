# `_col_in_circumcircle` loses the sign on mixed-magnitude inputs

**Status:** 🟢 **FIXED in 2.9.3.** It was never "a property of the predicate in isolation" — it was
a **silently wrong `delaunay_2d`** on the public entry point, and the 2.9.1 recommendation to leave
it rested on a premise that does not hold. `_col_in_circumcircle`'s winding term is now an adaptive
exact `orient2d` over the raw coordinates. This supersedes the *Disposition: leave it* section at
the bottom of this file.

⚠ **The first repair attempt was wrong and is recorded below**, because it is the attempt anyone
would make second time round: making the determinant exact **while leaving the operands
pre-differenced** changes nothing.

## Re-scoping (2.9.3) — three findings, one of them a refutation of the obvious remedy

### 1. It is reachable, and the evidence that said otherwise was vacuous

2.9.1 closed on "nothing can hand it that input", supported by a **0 wrong of 1,528,820** sweep over
quadruples of four suite fixtures. That sweep was real, but every one of those fixtures is drawn from
a **single origin-anchored box** (`_dlr_points`, `tests/modules.tcyr:126`), so the family has no
*intra-set* dynamic range and structurally could not reach this failure mode.

What breaks the predicate is **ratio within one set**, not absolute magnitude. Translating a whole
set out to 1e9 is fine — Sterbenz differencing handles it, and 40 such sets show 0 failures. Mixing
scales inside one set is not: with one vertex at ~6e5 and five points clustered inside ~5e-17 of the
origin, **`delaunay_2d` silently drops an input point**. On a 3,999-seed sweep of that shape, failing
sets are dense (seeds 1, 2, 6, 7, 9, 16, 20, … in the first twenty).

A ready-made failing fixture, so the next attempt does not have to re-derive one — exact hull h = 4,
so a correct triangulation has 2n−2−h = **6** triangles; the shipped predicate returns **4** and uses
only **5 of 6** input points:

```
vec_push(icp, hvec2_new(0x4122_9CE2_D5D6_E212, 0x4127_8907_29BF_24A1));   # ~ (6.099e5, 7.712e5)
vec_push(icp, hvec2_new(0x3C8E_24E8_BBBE_CC94, 0xBC5C_7CF2_DE23_7A70));
vec_push(icp, hvec2_new(0xBC5C_8956_4E5D_FCA0, 0x3C80_D342_FFE4_0540));
vec_push(icp, hvec2_new(0x3C88_267B_1B35_CD8E, 0x3C47_9EEC_3C48_9E00));
vec_push(icp, hvec2_new(0xBC7B_7473_90E5_40E4, 0x3C82_D0D7_239D_1858));
vec_push(icp, hvec2_new(0xBC68_8A23_88FE_A9B8, 0x3C6A_FCD4_4D14_CF88));
```

### 2. The suite's own oracle is blind to this by construction

`_dl_violations` (`tests/modules.tcyr:225`) decides whether a circumcircle is empty by calling
**`_col_in_circumcircle`** — the code under test. It cannot detect a sign flip in that predicate. Any
assertion added here must use a **structural** oracle instead: `_dl_used` (which input indices appear
in the output) and `_dl_max_edge_share` evaluate no predicate at all. The two repro files in this
directory are `.tcyr` but are **not under `tests/`**, so CI has never run them.

### 3. ⛔ An exact `orient2d` on the predicate's own arguments does NOT fix it — measured

*(This was the first attempt. The shipped fix, in §4 below, operates on raw coordinates instead.)*

The obvious repair, and the one this file's own remedy section implies, is to make the winding term
exact. **That was implemented in 2.9.3 — an adaptive Shewchuk-style `orient2d` with Dekker splitting,
TwoProduct/TwoSum/TwoDiff tails and a four-component `Two_Two_Diff` expansion — and it changed
nothing.** The fixture above still returned 4 triangles and still dropped a point. The expansion
itself was verified correct (it reproduces exact rational results on ordinary triples, and its
products satisfy `l + lt == acx*bcy` exactly).

The reason is that **the information is already gone before the exact arithmetic starts.**
`_col_in_circumcircle` translates to the query point and then differences in *floating point* —
`acx = f64_sub(bx, ax)` — and with `b`, `c` at ~1e-17 and `a` at ~4e5 **both differences round to the
same double**:

```
(b-a).x float = -390661.50221406674
(c-a).x float = -390661.50221406674     <- identical; b and c are now the same point
exact determinant of those float differences  =  0            <- faithfully zero
exact determinant of the ORIGINAL coordinates = -5.457401e-11 <- the real answer
```

So an exact predicate fed pre-differenced operands correctly reports 0 for a configuration that is
not degenerate. **The exactness has to extend back through the subtraction**: the fix is Shewchuk's
`orient2dexact` shape — six TwoProducts of the **raw** coordinates combined by expansion sums, never
rounding a difference — not an exact determinant of rounded differences. That is a materially larger
routine than the one attempted, and it belongs in its own bite with its own benchmark, since it sits
in `delaunay_2d`'s inner loop.

That first attempt was **discarded rather than shipped** — it added cost and a page of numerical code
while fixing none of the measured failures — and replaced by §4.

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

### 4. The shipped fix (2.9.3)

`_col_orient2d_sign` — adaptive, in `src/collision_mesh.cyr`:

- **fast path**: the float determinant, returned whenever `|det|` clears
  `(3 + 16ε)ε · (|l| + |r|)`. That is every ordinary call, so the cost lands only where a sign error
  is actually possible.
- **exact path**: `_col_o2d_exact_raw`, Shewchuk's `orient2dexact` shape — the determinant expanded
  into **six products of the raw coordinates**, three `Two_Two_Diff` four-component expansions, and
  two `fast_expansion_sum_zeroelim` merges. No difference is ever rounded, which is precisely what
  §3 got wrong. The sign is read off the most significant component of the final expansion;
  components are non-overlapping, so no summation and therefore no rounding is involved.

**Validation.** 300 random mixed-scale triples (one vertex at ~1e6, two clustered inside ~5e-17),
cross-checked against exact rationals: **300/300 correct**. The shipped float winding scores
**0/300** on the same set. Plus the ordinary CCW / CW / exactly-collinear cases, and the sign-loss
triple where the float determinant is exactly `0.0` and the truth is `-5.457401e-11`.

**Assertions.** `tests/modules.tcyr`, group *"2.9.3 delaunay_2d on WIDE INTRA-SET DYNAMIC RANGE"*:
the fixture above (structural oracles only — `_dl_used`, `_dl_max_edge_share` — never
`_dl_violations`), plus four unit assertions on `_col_orient2d_sign`. **Three mutants caught**: the
pre-2.9.3 float winding (fixture drops to 5 points / 4 triangles), and the filter forced always-taken
(same, plus the unit assertion).

⚠ **One mutant SURVIVES, and it is recorded rather than hidden.** Feeding `_col_orient2d_sign` the
already-translated operands passes the entire suite, because on this fixture the query translation is
exact and the expansion recovers the sign either way. Raw coordinates are still correct on principle —
one fewer rounding step, and what Shewchuk's orient2d takes — but **no assertion here distinguishes
them**, so a future change moving that call back onto translated operands would not be caught. Two
searches for a distinguishing case (2,000,000 mixed-magnitude triples, 3,000,000 near-degenerate ones
at 1e15..1e17) found none: a uniform shift by `p` moves all three vertices on the same lattice, so the
roundings largely cancel out of the determinant.

**Cost: none measurable.** Same machine, avg statistic, against the 2.9.2 baseline —
`delaunay_2d_400` −3.5%, `delaunay_2d_circle_150` −5.9%, `halfedge_2k_tris` −3.9%,
`convex_hull_2d_2k` −3.7%, `triangulate_600gon` −6.4%; median across all 55 benchmarks −3.1%, with
**0 benchmarks above the +10% noise floor**. The negative signs are noise, not a win, and no
improvement is claimed.
