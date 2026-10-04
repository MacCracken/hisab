# 2026-09-30 — ganita `ganita_f64_atan2`: the sign of a zero y or x is ignored, and a NaN y at x = ±0 returns −π/2

**Component:** cyrius stdlib `lib/ganita.cyr` → `ganita_f64_atan2`. The source lives in the ganita
repo at `src/math_advanced.cyr`, and cyrius folds it into its stdlib.
**Toolchain seen:** cyrius **6.6.12** (ganita 1.2.9). The function is byte-identical from ganita 1.2.4
through 1.2.10.
**Severity:** Medium. On the negative real axis `cx_arg` returns the wrong branch, and for a NaN
imaginary part at zero real part it returns a fabricated angle.
**Status:** ✅ **CLOSED — FIXED UPSTREAM IN ganita 1.2.11** (2026-10-01), shipped in cyrius **6.6.13**
and **6.6.14** (`lib/ganita.cyr` is byte-identical in both tags). hisab took it in **3.3.4** (2026-10-03)
on the 6.6.12 → 6.6.14 bump. ganita archived the filing as
`ganita/docs/development/issues/archived/2026-09-30-f64-atan2-signed-zero-and-nan.md`, together with its
companion `2026-09-30-f64-atan2-infinite-arguments.md` (abaco's ±∞ rows). One rewrite of
`ganita_f64_atan2` closed both: a NaN operand is returned first (y first, payload kept), zeros are
decided on their sign bits, and atan2(±∞, ±∞) is ±π/4 or ±3π/4. Until 3.3.4 this record was the one
open filing (roadmap D140, `dependency-watch.md`'s 6.6.12 entry).

Filed 2026-09-30 as
`ganita/docs/development/issues/2026-09-30-f64-atan2-signed-zero-and-nan.md`. A self-proving repro
sat beside it under `repros/` and exited **5** on ganita 1.2.10: four signed-zero rows and one NaN
row of the C99 F.10.1.4 table. The sections before *Closure* are the record as it stood while open.

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
(`lie_norm3`, named `_lie_norm3` through 3.2.x) and a norm is never −0. `su2_log` (`lie.cyr`)
also returns before calling unless `y > 0`. `so3_log` (`lie_ext.cyr`) returns early only when
`cos θ > 0`. It could therefore reach the NaN row, but only with a NaN rotation matrix whose
`cos θ` term is exactly ±0.

## Nothing changed for hisab in 3.2.2

`atan2(-0, -1)` returned +π under cycc 6.6.6 as well. The difference is that no hisab path could
produce the −0 before. hisab's branch-cut answers are the same before and after the bump.

## What hisab does meanwhile

There is no source workaround. `cx_arg` is one call to the stdlib, and the repair belongs there.
`tests/edge_cases.tcyr` pins both rows of the table above as a **tripwire**. Each assertion is
labelled `KNOWN DEFECT (ganita atan2)`, and both **fail when ganita repairs atan2**. At that point,
flip them to −π and NaN and archive this record. That is the pattern this repo used for the
`public enum` leak and the 2.20.0 acceptance pins.

## Closure — the paired measurement, run 2026-10-03

Each side ran from a scratch dir whose `cyrius.cyml` pins the version and declares hisab's 16
stdlib leaves. All 29 vendored files byte-match the cyrius tag for 6.6.12 and for 6.6.14, and the
`compiler:` line read `versions/<v>/bin/cycc`.

| ganita's repro, include lines dropped so it builds against the stdlib copy of `lib/ganita.cyr` | exit | wrong rows |
|---|---|---|
| 6.6.3 (ganita 1.2.5) | 5 | atan2(−0,+0), (+0,−0), (−0,−0), (−0,−1), (NaN,+0) |
| 6.6.12 (ganita 1.2.9) | 5 | the same five |
| 6.6.13 (ganita 1.2.11) | 0 | none |
| 6.6.14 (ganita 1.2.11) | 0 | none |

| hisab call | 6.6.12 | 6.6.14 | C99 |
|---|---|---|---|
| `cx_arg(cx_conj(cx_new(-1, 0)))` | `0x400921FB54442D18` (+π) | `0xC00921FB54442D18` (−π) | −π |
| `cx_arg(cx_new(0, 0x7FF8000000000000))` | `0xBFF921FB54442D18` (−π/2) | `0x7FF8000000000000` (the input NaN) | NaN |

`tests/edge_cases.tcyr` before the flip: 383/0 on 6.6.12, and 381/2 on 6.6.14, where the two
failures are the tripwires failing as designed. After the flip and the D132 rows: 395/0 on 6.6.14.
The same file on 6.6.12 gives 381/14, so every new assertion fails against the old atan2.

**Exposure.** Seven public functions reach `ganita_f64_atan2`: `cx_arg`, `cx_ln`, `cx_sqrt`,
`cx_powf`, `su2_log`, `so3_log` and `se3_log`. On an 81-input special-value grid plus the conj rows,
65 of 519 output lines change. Every changed `cx_arg` and `cx_ln` row is now C99 carg/clog.
`cx_sqrt` takes the C99 sign on the cut, and gives +∞ ± i∞ for both-infinite input where it gave
NaN. The 25 Lie-group rows are bit-identical: `so3_log` does reach atan2(NaN, +0), but 3.3.3's
non-finite exit discards the angle.

## What hisab does now

- The two `KNOWN DEFECT (ganita atan2)` tripwires are flipped to −π and NaN. The D132 rows
  (`cx_sqrt`, `cx_ln`, `cx_powf` on conj(−x + 0i), and `cx_arg(±0, ±0)`) and the ±∞ rows are
  pinned as C99 values, bit-exact. D132 and D140 are closed.
- The `so3_log` quarter-turn NaN fixture in `tests/modules.tcyr` no longer discriminates its guard
  under hisab's pin (the eigenvector branch now scales by NaN). The guard is still killed on 6.6.14,
  by the two "Inf - Inf off the diagonal" rows, through the NaN's sign bit; the comment says so.
- **Consumers below cyrius 6.6.13 still get the old answers**, because each compiles the bundle
  against its own stdlib. All ten direct consumers pin 6.6.2–6.6.10 (ganita 1.2.4–1.2.8). From 6.6.8
  on, `cx_conj` produces the −0 that reaches the wrong branch; below 6.6.8 only an explicit −0 input
  does.
