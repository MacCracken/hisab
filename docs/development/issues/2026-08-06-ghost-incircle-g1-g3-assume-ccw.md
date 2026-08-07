# `_col_dl_ic_g1` and `_col_dl_ic_g3` drop the winding factor, so `_col_dl_incircle` is not order-free — plus a genuine M¹ tie-break defect in `g1`

**Status:** 🟡 **PARTIALLY FIXED** — `g3` (finding 1, all-ghost half) **fixed in 2.9.1**; `g1`'s
M¹ tie-break (finding 2) **fixed in 2.9.1**; `g1`'s missing winding factor (finding 1, one-ghost
half) **still OPEN and latent**. Filed 2026-08-06.

**2.9.1 — finding 1, `g3`.** Confirmed and repaired. The suggested repair below (`g3` becomes
`return 1`) was re-derived from scratch rather than inherited, and holds: the body is now
`return 1;` and the predicate takes no arguments. Two independent proofs, both checked in exact
rationals — the equal `|u_k|²` make `A` the circumcenter of the three ghosts, and the M⁴
coefficient `|u|²·orient(u)` shares its winding factor with the ghost triangle's own orientation
`M²·orient(u)`, so the two cancel. One correction to the reasoning recorded below: the claim that
"**every** finite point is inside a circle through three points at infinity, whichever way they are
listed" is **too general**. It is true here, but for three directions chosen without the equal-norm
(or positive-spanning) constraint a sweep of random triples answers `0` about **28%** of the
time — 27.6/27.9/28.0% uniform in [-1,1]², 27.1/26.9/27.4% in the unit disc, 20,000 triples per
seed in exact rationals. The rate is distribution-dependent, so the distribution is named with
it. (An earlier draft said 36% and named no distribution; it did not reproduce, and had reached
four files before a verifying pass re-ran it.)
Asserted in `tests/modules.tcyr` over both windings (1800 probes: 4 scales × 3 apexes × 25 query
points × 6 orderings), and mutation-proved in both directions.
**Placement:** unpinned.
**Discovered:** 2026-08-06, while deciding
`2026-08-05-two-ghost-tie-branch-sign-rule-inconsistent.md`. That issue asked whether
`_col_dl_ic_g2` should be callable with a clockwise triangle; answering it required measuring
whether its two siblings are, and they are not.
**Severity:** Low today (finding 1 is unreachable, finding 2 needs a codimension-2 configuration),
high if reached — both are sign inversions in a geometric predicate.
**Affects:** hisab 2.8.2 onward (the symbolic-ghost `delaunay_2d`).

## Method

Ground truth is the **exact circumcircle of the three actual points** — `Fraction` circumcenter,
exact `|p − O|²` vs `r²`, ghosts placed at `A + 10³⁰·u`. That question takes no orientation
argument, so it cannot inherit a sign convention from the code under test. `_col_dl_ic_g1` and
`_col_dl_ic_g3` were transcribed to Python operation-for-operation from the shipped source.

Note this is a **Python transcription** of the Cyrius source, not the Cyrius source itself. The
transcription is literal, but that is the standard of evidence here and it is weaker than a
first-party assertion. Neither finding is asserted against in any suite.

## Finding 1 — neither helper normalises by winding

"Is p inside the circumcircle of these three points" is a question about a **set**: the answer
cannot depend on the order the vertices arrive in. The in-circle determinant's *sign* does depend
on it, so a correct implementation multiplies by the triple's winding exactly once.
`_col_dl_ic_g2` does (`collision_mesh.cyr:298`). Its two siblings do not:

| helper | CW-presented triple | wrong | status |
|---|---|---|---|
| `_col_dl_ic_g1` (`:237`) | 463,086 | **462,846 (99.95%)** | 🟡 open |
| `_col_dl_ic_g3` (`:329`) | 243 | **243 (100%)** | ✅ fixed 2.9.1 — now `return 1`, asserted both windings |

- `g1` returns `sign((a−p) × (b−p))`, which is `sign(M² coeff)`. The stored triple is `(a, b, G_k)`
  and its limiting winding is `sign((b − a) × u_k)`; `g1` never forms that. Its own comment
  (`:231-236`) states the dependence — *"the stored triangle is counter-clockwise so that side is the
  positive one"* — so this is documented, not accidental.
- `g3` returns `orient(u_a, u_b, u_c) > 0`. The correct order-free answer is that **every** finite
  point is inside a circle through three points at infinity, whichever way they are listed, i.e.
  a constant `1`. Under a CW listing `g3` answers `0` for every point in the plane.

**Consequence.** `_col_dl_incircle` is CCW-only, and the 2.9.1 fix to `_col_dl_ic_g2` does not
change that — it makes one of three helpers order-free. The source comment above `_col_dl_ic_g2`
now says so explicitly so the `cr` handling there is not read as a claim about the entry point.

**Not reachable today.** `delaunay_2d` stores every triangle CCW: the seed ghost triangle is listed
`(g0, g1, g2)`, each fan triangle inherits CCW from a CCW cavity boundary edge, and
`_col_dl_incircle`'s three rotations are even permutations. `tests/modules.tcyr` already asserts
the emitted end of that chain (`_dl_non_ccw == 0` across eight fixtures) and the constant end
(`_col_ghost_cross` is `+1` on exactly the cyclic pairs).

## Finding 2 — `g1`'s M¹ tie-break is wrong when the ghost edge is parallel to `u_k`

This one is **not** about orientation. Restricted to **correctly CCW-stored** triples, `g1` still
disagrees with the exact circumcircle on **228 of 463,086** probes. Every one of them:

- is decided by the **M¹ branch** (`collision_mesh.cyr:247-258`),
- answers `0` where the truth is `1`, and
- has `(a − p) · (b − p) < 0`, i.e. **p lies strictly between a and b** — exactly the case the
  branch's own comment claims it returns `1` for: *"a point on the segment is inside the limiting
  circle"*.

**Root cause.** With `C = (a−p) × (b−p) = 0` the branch computes
`t = u_x·d1 + u_y·d2`, which is the M¹ coefficient — correct. But that coefficient factors as

```
t = |b−p|² · (u × (a−p))  −  |a−p|² · (u × (b−p))
```

so `t` vanishes identically whenever `(a−p)` and `(b−p)` are both parallel to `u_k` — that is,
whenever **p is on line(a, b) and line(a, b) is parallel to `u_k`**. The branch then falls through
to `return 0` ("not inside"), but the M⁰ term decides, and it says *inside*. The comment attributes
`t == 0` solely to "p coinciding with a or b, i.e. a duplicate point", which is one cause of `t == 0`
and not the only one.

Smallest reproducer found, with `k = 0`, `u₀ = (1, 2)`:

```
a = (-2, 0)   b = (-4, -4)   p = (-3, -2)
a - p = (1, 2) = u₀        b - p = (-1, -2) = -u₀
C = 0,  t = 1·20 + 2·(-10) = 0   ->  g1 returns 0
truth: p is the midpoint of segment ab, hence strictly inside.  Expected 1.
```

**Reachability.** Needs a hull edge exactly parallel to one of the three fixed rational directions
`(1, 2)`, `(−2, 1)`, `(1, −2)` **and** the query point exactly on that line — codimension 2, and
`u_k`'s directions are deliberately not axis-aligned, which is most of why it has not been seen.
Integer or lattice-structured input is where to look: the reproducer above is all small integers.
**Not measured against real `delaunay_2d` output** — that is the first thing to do before assigning
severity, and it was out of scope for the 2.9.1 task that found it.

## Suggested repair

Take them separately; they have different risk.

1. **Finding 2 first** — it is the one that can be wrong on correctly-formed input. When `t == 0`,
   fall through to the M⁰ coefficient (`q_x·d1 + q_y·d2 + |q|²·C`, with `q = A − p`) instead of
   returning `0`. Needs the apex `A`, which `g1` does not currently take.
2. **Finding 1** — if `_col_dl_incircle` is to become order-free, `g1` multiplies by
   `sign((b − a) × u_k)` (one extra cross product, cold path only if hoisted) and `g3` becomes
   `return 1`. If it is *not* to become order-free, then say so at the entry point rather than in
   two of three helper comments, and note that `_col_dl_ic_g2` is order-free anyway.

   **`g3` half done in 2.9.1** — it is `return 1` and is now order-free unconditionally, at no cost,
   so it no longer constrains the decision. Only `g1` is left, and it is the half that carries an
   actual cost (the extra cross product), so the "is the entry point order-free?" question is now
   purely about `g1`.

Whichever way finding 1 goes, the assertion already exists in the shape needed: `_dlg_perm_bad` in
`tests/modules.tcyr` counts entry-point answers that disagree across all six vertex orderings, and
is asserted `== 0` for the two-ghost path.

**2.9.1 update.** The all-ghost path now has its own harness — `_dlg3_bad` / `_dlg3_run` in
`tests/modules.tcyr` — asserted green over both windings (1800 probes). It does not reuse
`_dlg_perm_bad` because the all-ghost answer is a known constant rather than merely
order-*consistent*: asserting `== 1` outright is strictly stronger than asserting that the six
orderings agree, which a uniformly wrong constant would also satisfy. The **one-ghost** path
remains the one that would fail if pointed at today, and it is what is left of this issue.
