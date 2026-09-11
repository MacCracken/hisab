# `svd_golub_kahan`'s balancing divide is lossy for a subnormal sub-block

**Status:** ✅ **RESOLVED in 2.22.1.** Classified against a 200-digit oracle over 3 fixtures x 52
subnormal scales: CORRECT **23 -> 151**, LOUD **62 -> 0**, SILENT **71 -> 5**, with zero
`CORRECT -> anything` regressions and zero `LOUD -> SILENT`.
⛔ **THIS FILING'S OWN ROOT-CAUSE CONCLUSION WAS WRONG AND IS CORRECTED INLINE BELOW RATHER THAN
DELETED.** It said "no global scalar can work"; measurement refuted that within the hour. It also
named only half the cause. A filing that was wrong is evidence about how these get written.
**Discovered:** 2026-09-11, during the 2.22.0 Householder-gate census.
**Severity:** High — **silent wrong answer**. `svd_golub_kahan` returns `HSB_ERR_NONE` with a
smallest singular value of exactly **ZERO** for a matrix that is demonstrably non-singular.
**Blocks:** nothing shipped; it is a pre-existing defect that 2.22.0 measured but did not widen.

## Symptom

For a block-diagonal `A = blockdiag(B, c·B)` with `B = [[9,1],[0,2]]` and `c = 2^-1073`, on the
2.22.0 tree:

```
  c=2^-1073  rc=0                       <- HSB_ERR_NONE, i.e. "success"
    S[0] = 9.2195444572928871           (correct)
    S[1] = 1.9523195857244057           (correct)
    S[2] = 16 units of 2^-1074          truth: 18 units
    S[3] = 0                            truth: 4 units   <- FABRICATED ZERO
```

Truth is from a 120-digit `Decimal` oracle built from the **actual rounded entries**, and the
matrix is block-diagonal, so the exact singular values are `{s(B)} ∪ {c·s(B)}` — no reference
implementation is involved.

⚠ This is the row `tests/hisab.tcyr` pins as `_sq_ok == 94` and as *"…and its smallest singular
value is a fabricated ZERO (truth: 4 units)"*. It is a **tracked** defect with an acceptance test,
not an unknown one.

## Root cause — measured, not inferred

The entry-point balancing at `src/linalg_precision.cyr:~986` divides every entry by
`_lp_pow2_floor(_lp_mat_max_abs(A))`, a power of two chosen from the **largest** entry — which is
set by the NORMAL block. The small block is subnormal, and **a power-of-two divide is exact only
while the quotient stays normal.** Below `DBL_MIN` the representable grid is absolute (integer
multiples of 2^-1074), not relative, so dividing walks off it.

Round-trip probe on the failing fixture:

| quantity | value |
|---|---|
| small entry `9c` | **18** units of 2^-1074 |
| balance scale `pow2_floor(9)` | 8 |
| after divide | 18/8 = 2.25 → **rounds to 2** units |
| multiplied back | 2 × 8 = **16** ≠ 18 |

**11% of the value is destroyed before `_lp_bidiagonalize` is ever called**, and nothing
downstream can return it.

⛔ **A comment in the shipped source asserted the opposite, and that is why nobody looked here.**
It read *"the power-of-two divisor makes both the divide and the un-scale exact."* Corrected in
2.22.0 in place, with the measurement beside it. **A comment asserting a numerical property is a
claim like any other.**

## Why 2.22.0's repairs do not touch it

2.22.0 repaired two real defects in the same pipeline — the absolute `EPSILON_F64` Householder
gates (`_lp_bidiagonalize:270/:341`, `_lp_tridiagonalize:1119`) and `_lp_bidiag_qr`'s absolute
`p_lo` active-block search. **Both leave this row byte-identical**, verified rather than assumed.
That is consistent: the loss happens upstream of both.

⭐ This is the repo's own rule working as intended — *a repair that moves no number is pointing at
a second defect.*

## What a fix has to establish FIRST

⛔ **Do not reach for "scale the active block inside `_lp_bidiag_qr`."** That was the roadmap's
proposal for three releases and 2.22.0 measured it: it converts **84 loud rows into gross silent
wrong answers**, 83 of them on general 2×2 blocks. The 2.19.0 adversarial refutation of it is
TRUE.

The question to answer with numbers, before touching code:

1. ~~Compute the dynamic range … if it exceeds f64's range — which it does for these fixtures —
   then **no global scalar can work**.~~
   ⛔ **CORRECTION (2.22.1): FALSE, and refuted by the very measurement this step asked for.**
   f64's normal range spans **2045 binades**; these fixtures need **~1076**. A single global scalar
   is entirely sufficient. The defect was **which** scalar: `pow2_floor(max|A|)` normalises the
   LARGE end and says nothing about the small one. The fix chooses it from **both** ends — halve it
   (scale the matrix *up*) until the smallest non-zero entry is normal, stopping if the largest
   would overflow. ⚠ It must be allowed past 1 into `sc < 1`: dividing cannot lift a subnormal
   input into the normal range, and a draft that guarded `sc > 1` did nothing for exactly the
   inputs it was written for — measured, it turned a **correct** row at 2^-1065 into
   `HSB_ERR_NO_CONVERGENCE`.
2. If per-block: where does the block boundary come from, and what happens to a matrix whose
   scales are a continuum rather than two clean blocks? The fixture family is block-diagonal, and
   a class sized only on its fixtures is how 2.20.0's acceptance family missed the `p_lo` defect
   (all its blocks were 2×2, where `p_lo` is correct by accident).
3. If refusal: `HSB_ERR_INVALID_INPUT` vs `HSB_ERR_NO_CONVERGENCE` — the latter is honest about
   *this* matrix; the former is a contract statement. 2.22.0 chose refusal for non-finite input on
   exactly this reasoning, so there is a precedent to match.

## Reproduce

`cyrius build <probe>.cyr <out>` from the repo root, where the probe builds the fixture above and
prints `rc` plus the four singular-value bit patterns. The balancing round-trip needs no SVD at
all:

```
small = f64_mul(f64_from(9), 1 << 1);      # 9 * 2^-1073 == 18 units of 2^-1074
sc    = _lp_pow2_floor(f64_from(9));       # == 8
back  = f64_mul(f64_div(small, sc), sc);   # == 16, not 18
```

## ⛔ CORRECTION: the balancing divide was only HALF the cause

Repairing the balance alone gives CORRECT 102 / SILENT 54 — but converts **30 LOUD rows into SILENT
wrong answers**, the exact trade the 2.19.0 refutation named. The other half is one level down:
`vtv` is a naive sum of squares, and at the **bottom of the normal range** (entries around 2^-1022)
every square underflows to zero, so 2.15.0's `F64_TINY` guard correctly refuses to divide — and the
consequence is the same silent drop the absolute gate used to cause.

⭐ `H = I - 2vv'/v'v` is identically `I - 2uu'` for `u = v/||v||`, so normalising `v` makes `vtv`
**1 by construction**. Applied to all three reflectors. **Either half alone is worse than neither.**

## Related

- `docs/development/roadmap.md` — "Decisions owed" carries the sizing question as a scheduled item
  and points here for the defect itself.
- The sibling coverage gap (three Householder gates repaired on SHAPE with no fixture reaching
  them: `_lp_bidiagonalize:341`, `cqr_decompose:1671`/`:1676`) is a **coverage** item, not a bug,
  and stays on the roadmap.
