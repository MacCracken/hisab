# `_col_dl_ic_g2`'s tie branch multiplies by `cr` where the determinant says it should not — latent, unreachable today

**Status:** 🟢 **FIXED in 2.9.1** — `src/collision_mesh.cyr:320` now reads `if (s2 > 0)`. Nine new
assertions in `tests/modules.tcyr` cover the anti-cyclic half; both are mutation-proven below.
See **Resolution** at the bottom, which also answers question (3) with measurements.
**Placement:** unpinned. Not a 2.9.0 blocker: unreachable on every path that exists today.
**Discovered:** 2026-08-05, while deriving an exact-rational oracle to close the 2.9.0 mutation
sweep's U4 coverage gap. Found by the derivation, not by a failing test — nothing in the suite
reaches it.
**Severity:** Low **today**, high **if reached**. It is a sign inversion in a geometric predicate,
so when it fires it returns the exact opposite answer.
**Affects:** hisab 2.8.2 onward (the symbolic-ghost `delaunay_2d` rewrite).

## Summary

`_col_dl_ic_g2` decides the two-ghost in-circle test from the leading nonzero coefficient of the
determinant expanded in the ghost magnitude M. It has two branches:

- `src/collision_mesh.cyr:274` — the **M³** branch: `if (s * cr > 0) { return 1; }`
- `src/collision_mesh.cyr:292` — the **M² tie** branch: `if (s2 * cr > 0) { return 1; }`

The M³ branch is correct for both orientations. The tie branch is not: the determinant's M²
coefficient already carries the orientation, so multiplying by `cr` applies it **twice**.

With `cr² = 1`, `sign(M² coeff) = cr * s2`, so the correct rule is `s2 > 0` — the two forms agree
only when `cr = +1`. The same function therefore uses two different sign conventions in its two
branches.

## Why it does not fire today

`_col_dl_incircle` only ever presents ghost pairs taken from a **CCW-stored** triangle, which
forces `cr = +1`, where the two rules coincide. That is also why the U4 probe grid added in this
cycle is restricted to the three cyclic pairs.

Measured against the true determinant in exact rational arithmetic (Python `Fraction`), on the
shipped ghost constants:

| pair orientation | tie-only trials | mismatches |
|---|---|---|
| cyclic (`cr = +1`, the only reachable case) | 8,737 | **0** |
| anti-cyclic (`cr = −1`) | 8,737 | **8,737 (100%)** |

So the branch is wrong for exactly half the input space, and today's callers never supply that
half. A future change that stores a triangle clockwise, or reuses this predicate outside
`_col_dl_incircle`, reaches it immediately — and it does not fail loudly, it returns the opposite
in-circle answer, which surfaces as a silently non-Delaunay mesh.

Corroboration from the same cycle: mutating the ghost constants to a clockwise-equivalent set
(`u1 ↔ u2`) makes the new U4 harness's `MISMATCH` counter jump to 224.

## Proposed fix

Drop the `* cr` from the tie branch only, leaving `src/collision_mesh.cyr:274` alone:

```
if (s2 > 0) { return 1; }
```

## Why it is filed rather than fixed

The task that found it was contract-bound to leave `src/` byte-identical, and a sign flip in a
geometric predicate is not a change to make as a side effect of a test-coverage pass. It wants its
own cycle with:

1. the fix,
2. an assertion driving the predicate with an anti-cyclic pair, which no current fixture does, and
3. a decision on whether `_col_dl_ic_g2` should be callable with a CW triangle at all — if the
   CCW precondition is meant to be permanent, the honest repair may be to **assert** `cr == 1`
   rather than to handle `cr == -1` correctly.

Question (3) is the real one. Fixing the arithmetic without deciding it just moves the latent
assumption somewhere less visible.

---

## Resolution (2.9.1)

### The finding reproduces — re-derived from scratch, not inherited

Re-verified with an oracle that shares no reasoning with the derivation above: the ground truth is
the **exact circumcircle of the three actual points** (`Fraction` circumcenter, exact
distance-vs-radius comparison, ghosts placed at `A + 10³⁰·u`). That question takes no orientation
argument at all, so it cannot inherit a sign convention.

Coefficient identities, 400 random `(A, R, p)` × all 6 pairs — **0 disagreements each**:

- `c₃ = |u|² · (r × (u_j − u_k))`
- `c₂ = (u_j × u_k) · (|R − A|² − |A − p|²)`

Whole predicate, 7×7 integer grid of `A`, `R`, `p`, 343,734 decided trials per orientation:

| variant | cyclic wrong | anti-cyclic wrong |
|---|---|---|
| shipped 2.9.0 | 0 | **24,156** |
| drop `* cr` from `:292` only | 0 | **0** |
| drop `* cr` from **both** branches | 0 | **319,578** |

Restricting to the tie branch alone (295,245 trials/orientation, 267,358 decided): shipped is
**0** wrong cyclic and **267,358 of 267,358** wrong anti-cyclic. The 0 / 100% split in the table at
the top of this file is confirmed on an independently built and much larger grid.

The third row is the load-bearing one: **`* cr` at `:274` must stay.** It is not redundant
decoration, and removing it costs 319,578 wrong answers on the anti-cyclic half.

### Answer to question (3): yes for this helper, no for the entry point

`_col_dl_ic_g2` **should** be callable with either ordering, because "p is inside the circumcircle
of these three points" is a question about a *set* — the answer cannot depend on vertex order. The
determinant's sign does depend on order, and `cr` is exactly the factor that divides that
dependence out. So `cr` at `:274` is not "handling a clockwise triangle"; it is what makes the
function compute the geometric predicate it advertises. The tie branch simply applied that
correction twice.

Choosing the opposite — assert `cr == 1` and drop the factor — was rejected on evidence: it deletes
correct code (row 3 above) to buy an assertion, and it would make `_col_dl_ic_g2` a fourth distinct
flavour of "trusts its caller's winding" in a file whose stated house style for
should-be-unreachable states is the opposite (`collision_mesh.cyr:362-367` and `:529-532` both
refuse to trust an invariant and take a defined path instead).

**But the entry point is a different question, and the answer there is no.** Measured the same way,
with the shipped sources transcribed literally:

| helper | CW-presented triple | wrong |
|---|---|---|
| `_col_dl_ic_g1` | 463,086 | **462,846 (99.95%)** |
| `_col_dl_ic_g3` | 243 | **243 (100%)** |

Both drop the winding factor entirely, so **`_col_dl_incircle` is not CW-safe and this fix does not
make it so.** That is now stated in the source comment above `_col_dl_ic_g2` so no future reader
infers otherwise, and filed as
`2026-08-06-ghost-incircle-g1-g3-assume-ccw.md` — which also records a *second*, orientation-
independent defect the same sweep turned up in `_col_dl_ic_g1`'s M¹ tie-break.

### Assertions added (`tests/modules.tcyr`)

`_dlg_run()` became `_dlg_run(anti)`; the existing cyclic block is unchanged and now calls
`_dlg_run(0)`. Added: the same 450-probe grid over the three **anti-cyclic** pairs, plus
`_dlg_perm_bad`, which drives the **public entry point** `_col_dl_incircle` with all six orderings
of `(R, G_j, G_k)` and counts answers that disagree with the first. Modules **+9**.

### Mutation proof

Two mutations, applied and reverted one at a time; `src/collision_mesh.cyr` md5-verified back to
`2d8eb32a4570fb2ab16dcdcf146f118f` after each.

| mutation | result |
|---|---|
| revert the fix — `:320` → `if (s2 * cr > 0)` | **3 FAIL** — perm cyclic 678, oracle anti-cyclic 224, perm anti-cyclic 672 |
| drop `* cr` from the M³ branch `:298` | **3 FAIL** — perm cyclic 639, oracle anti-cyclic 213, perm anti-cyclic 639 |

The anti-cyclic oracle mismatch of **224** under the first mutation independently reproduces the
`MISMATCH → 224` corroboration recorded above from the clockwise-equivalent constant swap.

The second mutation is the evidence that closed the decision: it is precisely the "assert the
precondition and drop the factor" repair, and the suite now rejects it.

### Behaviour on reachable paths

Unchanged, by construction — `cr` is `+1` on every path `delaunay_2d` takes, where `s2 * cr > 0`
and `s2 > 0` are the same expression. All five suites green, 0 failed: abuse 732, hisab 416,
edge_cases 233, foundation 349, modules 1576 (this change contributes +9; a concurrent 2.9.1 task
added the other +3 to the same file). Constant gate 153/153.

## Note on the line numbers

An earlier draft of this finding cited `:291` and `:294`. Both were wrong — `:294` is a closing
brace. The verified sites are **`:274`** and **`:292`**, confirmed by
`grep -n "s \* cr > 0\|s2 \* cr > 0" src/collision_mesh.cyr`. Recorded because a stale line
citation in an issue file is how a real finding gets dismissed as already-fixed.
