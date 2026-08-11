# The 2.10.1 jet tie flag was blind to two of the four ways a slab tie arises

**Filed**: 2026-08-10 (against 2.10.1's own draft, before release)
**Status**: 🟢 **FIXED in 2.10.1** — `src/geo.cyr`, `geo_ray_aabb_face` and `geo_ray_obb_face`
**Severity**: High — a confident gradient in the wrong slot, with `tie = 0` asserting the point
is differentiable
**Found by**: an independent derivation + adversarial probe commissioned **against the finished
2.10.1 code**, not by the suite written alongside it

## Summary

`geo_jet_aabb` and `geo_jet_obb` return the miss sentinel when the primal reports a tie, because
an exact edge or corner hit has a genuine kink in `t`. The draft's tie test compared **near
against near and far against far**, which catches an edge (two slabs achieving the same entry) and
a corner (three). It could not see either of the other two ways `t` stops being differentiable:

| shape | condition | draft behaviour |
|---|---|---|
| edge | two slabs achieve `t_min` | ✅ tie reported |
| corner | three slabs achieve `t_min` | ✅ tie reported |
| **zero-width slab** | `min_k == max_k`, or `he_k == 0` | ❌ **wrong-slot gradient, `tie = 0`** |
| **tangential clip** | `t_min == t_max`, different slabs | ❌ confident gradient, `tie = 0` |

## The zero-width slab — a wrong number, not just a missing sentinel

With `min_k == max_k` the two faces of that axis coincide, so `t1 == t2` and the primal's swap
never runs — `f64_gt(t1, t2)` is false on equality. The face is therefore labelled the **min slot
unconditionally**, which is correct only when `d_k > 0`.

Measured on the draft, flat box `min (0,0,0) / max (0,1,1)`, ray `o (1, 0.5, 0.5)`, `d (-1, 0, 0)`:

```
jet     dt/dmin_x = -1.0     dt/dmax_x =  0.0     tie = 0
truth   dt/dmin_x =  0.0     dt/dmax_x = -1.0     (one-sided, widening each slot outward)
```

**The entire nonzero gradient is in the wrong slot, and the slot that carries it reads zero.** The
mirror ray (`d_x = +1`) is correct — so exactly **half** of all flat-box configurations were wrong,
and a fixture testing only one direction passes on the broken code. The suite now asserts both.

Reported as a tie rather than relabelled, because `t` is genuinely not two-sided differentiable
there: widening `min` and widening `max` give different one-sided answers, and a central difference
averages them into a number (0.5) that is neither.

## The tangential clip

`t_min == t_max` with *different* slabs owning the two ends. The ray grazes: entry and exit
coincide, and the hit sits on the **boundary of the hit set**, where an arbitrarily small sideways
perturbation turns it into a miss. The per-slab tie never compares `t_min` against `t_max`, so it
is structurally blind to this.

## Fix

**One line, at the tail of both functions, covering both shapes:**

```
if (f64_eq(t_min, t_max) == 1) { tie_near = 1; tie_far = 1; }
```

⚠ **A separate per-slab `f64_eq(t1, t2)` guard was written for the zero-width case first, and then
DELETED as dead code.** A zero-width slab pins both `t_min` and `t_max` to the same value, so it
can never fire without the tail line firing too. That was established by **mutation, not by
argument**: removing the per-slab guard changed nothing in the suite or in a dedicated probe;
removing the tail line broke both fixtures. Second time this release a written-then-measured guard
turned out to be unreachable-as-a-difference — the other was `geo_ray_capsule`'s `cyl_t2` branch
update.

## Verification

Removing the single tail guard fails **5 assertions** across both primitives and all three
degeneracy shapes:

```
geo_jet_aabb on a ZERO-WIDTH slab returns the miss sentinel, either direction   (x2)
geo_jet_aabb on a TANGENTIAL clip (t_min == t_max, different slabs)
geo_jet_obb  on a ZERO half-extent returns the miss sentinel, either direction  (x2)
```

Every fixture also asserts the **primal still returns a real `t`**, so the test proves the jet is
refusing a genuine hit rather than agreeing with a miss.

⚠ **The first fixture written for the tangential clip was not tangential.** Origin `(-1, 2, 0.5)`
with `d (1, -1, 0)` gives `t_min = 1, t_max = 2` — an ordinary **edge** hit, already covered by the
per-slab tie. It passed, which is exactly the failure mode: a fixture that tests something already
tested and reads as though it tested something new. The real one is `o (-1, 1, 0.5)`, where x owns
`t_min = 1`, y owns `t_max = 1`, and neither slab ties against the running value.

## Why it matters beyond the defect

**The suite that shipped alongside 2.10.1 asserted edges and corners because those are the two
degeneracies the design document named.** It could not have found these two, because it was
written from the same understanding as the code. The independent derivation was commissioned to
check the *formulas* — all 37 partials agreed term for term, and 0 of 24,237 finite-difference
comparisons failed — and its value turned out to be entirely in the **enumeration**: it listed
degeneracies the design had not.

That is a sharper version of the arc's standing lesson. A test written by the author of the code
inherits the author's list of cases. **Ask what is missing from the list, not whether the entries
on it pass.**
