# Five defects in forward-mode autodiff, all behind guards no finite difference can reach

**Filed**: 2026-08-11 (during 2.11.0, before any reverse-mode code was written)
**Status**: 🟢 **FIXED in 2.11.0** — `src/autodiff.cyr`
**Severity**: High — fabricated values *and* fabricated derivatives on ordinary input
**Found by**: running the feature's own correctness check against the code that already ships

## Why this was checked first

2.11.0 is tape-based reverse mode. Reverse mode is validated **against forward mode** — that is the
standard and the cheapest oracle available. So a wrong `dual_*` would have been inherited wholesale
and then *confirmed* by the two agreeing with each other.

Fourth release running in which asking that question of shipped code found defects in it.

## The five

| site | input | returned | truth |
|---|---|--:|--:|
| `dual_div` | `1 / 1e-13` | **(0, 0)** | (1e13, −1e26) |
| `dual_ln` | `ln(−5)` | **(NaN, −0.2)** | miss sentinel |
| `dual_ln` | `ln(1e-13)` | **(0, 0)** | (−29.9336, 1e13) |
| `dual_sqrt` | `sqrt(−4)` | **(NaN, NaN)** | miss sentinel |
| `dual_pow` | `d/dx x¹ at 0` | **NaN** | 1 |
| `dual_pow` | `d/dx x³ at −2` | **NaN** | 12 |

`dual_ln(−5)` returning `(NaN, −0.2)` is the worst of them: a NaN value paired with a **confident
finite derivative**. A caller checking only the gradient — which is the entire point of an autodiff
module — sees nothing wrong.

## Root causes, one per guard

* **`dual_div`** guarded `|b| < EPSILON_F64` (1e-12) and returned `(0, 0)`. But `1/1e-13 = 1e13` is
  exactly representable; the division only fails below DBL_MIN. Same class as the four thresholds
  2.10.2 repaired in `geo.cyr`, and repaired the same way: **guard exactly what makes the division
  fail and nothing more** — `_AD_F64_TINY` = DBL_MIN.
* **`dual_ln`** tested `f64_abs(v)`, which is not the domain of `ln`. Negatives sailed past it into
  `f64_ln`, and small positives were rejected although `ln` is fine to DBL_MIN (`ln(5e-324)` = −744).
  Now `f64_gt(v, 0) != 1`, which rejects negatives, zero **and** NaN.
* **`dual_sqrt`** computed `s = f64_sqrt(v)` *before* its guard, so a negative input was already NaN
  by the time anything checked. The domain test belongs on the **input**.
* **`dual_pow`** called `f64_pow`, which `lib/ganita.cyr` documents as `exp(exp * ln(base))` — so it
  is NaN for **every** non-positive base, and for `f64_pow(0, 0)`. ⚠ **That is the stdlib's stated
  implementation, not a hidden defect**, and it is the right answer for the transcendental case.
  The defect is hisab's: `dual_pow` inherited the restriction without documenting it, on a function
  whose commonest use is polynomials. Integer exponents now go through repeated multiplication —
  exact, no domain restriction, and it agrees with IEEE-754 `pow`.

## ⚠ The finite-difference sweep alone found NOTHING

This is the part worth keeping. A 13-op × 4-point FD sweep was written first and reported **zero**
real disagreements. Every guard sat in its blind spot, for a structural reason:

**at every input where a guard fires, the perturbed scalar is NaN too** — `ln(1e-13 − h)` is NaN,
`sqrt(−4 ± h)` is NaN, `(−2 ± h)^3.5` is NaN — so the sample is *skipped* and the guard is never
asked. The sweep skipped exactly the rows that mattered and reported clean.

It also produced **two mismatches that were the fixture's fault**: with an absolute `h = 1e-6`, the
difference quotient straddles the kink of `abs` and the pole of `1/x` at `x = 1e-8`. Both analytic
values were right. `h` is now scale-relative.

**A guard has to be interrogated directly. Differencing a function cannot reach the branch that
refuses to evaluate it.** The suite carries both: the FD sweep for the chain rule, and a separate
block that calls each guard at its threshold.

## Verification

* Suite `modules.tcyr` 1739 → **1751**, whole suite **3481**, 0 failures. Constant gate 157 → **158**.
* **Five mutants, five kills**: reverting each guard in turn fails 1, 2, 2, 3 and 2 assertions.
* ⚠ **A control assertion pins that the repair did not simply widen the domain**: `(−2)^0.5` must
  **still be NaN**, because there NaN is the right answer. Without it, "fixed the NaN" and "replaced
  one fabrication with another" are indistinguishable.

## One shipped assertion changed, and why that is not a revert

`abuse.tcyr` (2.9.0) pinned `dual_pow(0, 0)` as NaN. Its note reads: *"the value comes back NaN
rather than the conventional 1: pinned here so the degenerate exponent is a known, checked
outcome."*

That is a pin on an **incidental**, and it names 1 as the conventional answer itself — unlike the
NaN-propagation pin elsewhere in the same file, which **argues** that NaN is the better contract and
which 2.10.2 correctly refused to overturn. The distinction is whether the note makes a case or
records a fact. Updated to `0^0 = 1`, with the reasoning kept beside it.
