# `_col_dl_ic_g2`'s tie branch multiplies by `cr` where the determinant says it should not — latent, unreachable today

**Status:** 🟡 **OPEN** — latent. Filed 2026-08-05, not fixed, not asserted against.
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

## Note on the line numbers

An earlier draft of this finding cited `:291` and `:294`. Both were wrong — `:294` is a closing
brace. The verified sites are **`:274`** and **`:292`**, confirmed by
`grep -n "s \* cr > 0\|s2 \* cr > 0" src/collision_mesh.cyr`. Recorded because a stale line
citation in an issue file is how a real finding gets dismissed as already-fixed.
