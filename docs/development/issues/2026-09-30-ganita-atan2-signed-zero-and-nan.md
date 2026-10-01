# 2026-09-30 — ganita `ganita_f64_atan2`: the sign of a zero y or x is ignored, and a NaN y at x = ±0 returns −π/2

**Component:** cyrius stdlib `lib/ganita.cyr` → `ganita_f64_atan2`. The source lives in the ganita
repo at `src/math_advanced.cyr`, and cyrius folds it into its stdlib.
**Toolchain seen:** cyrius **6.6.12** (ganita 1.2.9). The function is byte-identical from ganita 1.2.4
through 1.2.10.
**Severity:** Medium. On the negative real axis `cx_arg` returns the wrong branch, and for a NaN
imaginary part at zero real part it returns a fabricated angle.
**Status:** 🔴 **OPEN — filed upstream 2026-09-30** as
`ganita/docs/development/issues/2026-09-30-f64-atan2-signed-zero-and-nan.md`. A self-proving repro
sits beside it under `repros/` and exits **5** on ganita 1.2.10: four signed-zero rows and one NaN
row of the C99 F.10.1.4 table. This file is hisab's record of its own exposure. Archive it when a
pin crosses the fix.

## How hisab found it

cyrius 6.6.8 made x86's `f64_neg` an IEEE sign flip. Before that it computed `0.0 − x`, which gives
+0. So from 6.6.8, `cx_conj(7 + 0i)` has imaginary part **−0.0**, as C99 G.6 specifies. The 3.2.2
bump found this as a failing `conj real.im=0` assertion.

The −0 does not reach the branch cut. `cx_arg` is `ganita_f64_atan2(im, re)`, and ganita picks the
quadrant with IEEE comparisons, which treat −0 and +0 as equal:

| hisab call | atan2 call | C99 | returned |
|---|---|---|---|
| `cx_arg(cx_conj(cx_new(-1, 0)))` | `atan2(-0, -1)` | −π | **+π** |
| `cx_arg(cx_new(0, NaN))` | `atan2(NaN, +0)` | NaN | **−π/2** |

`cx_sqrt` and `cx_ln` take their angle from `cx_arg`, so they land on the wrong side of their cuts
in the same way.

The two Lie-group calls are not exposed to the signed-zero rows, because their `y` is a scaled norm
(`_lie_norm3`) and a norm is never −0. `su2_log` (`lie.cyr`) also returns before calling unless
`y > 0`. `so3_log` (`lie_ext.cyr`) returns early only when `cos θ > 0`. It could therefore reach
the NaN row, but only with a NaN rotation matrix whose `cos θ` term is exactly ±0.

## Nothing changed for hisab in 3.2.2

`atan2(-0, -1)` returned +π under cycc 6.6.6 as well. The difference is that no hisab path could
produce the −0 before. hisab's branch-cut answers are the same before and after the bump.

## What hisab does meanwhile

There is no source workaround. `cx_arg` is one call to the stdlib, and the repair belongs there.
`tests/edge_cases.tcyr` pins both rows of the table above as a **tripwire**. Each assertion is
labelled `KNOWN DEFECT (ganita atan2)`, and both **fail when ganita repairs atan2**. At that point,
flip them to −π and NaN and archive this record. That is the pattern this repo used for the
`public enum` leak and the 2.20.0 acceptance pins.
