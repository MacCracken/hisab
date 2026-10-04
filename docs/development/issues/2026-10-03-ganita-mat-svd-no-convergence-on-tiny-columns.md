# 2026-10-03 — ganita `ganita_mat_svd`: −1 ("did not converge") for finite matrices that have an SVD — square matrices with a zero or repeated row, column pairs that need a very small rotation, and a rounding two-cycle

**Component:** cyrius stdlib `lib/ganita.cyr`: `ganita_mat_svd` → `_linalg_svd_impl`, the one-sided
Jacobi sweep. `ganita_mat_rank`, `ganita_mat_condition` and `ganita_mat_pseudo_inv` share it. The
source lives in the ganita repo at `src/linalg.cyr`, and cyrius folds it into its stdlib.
**Toolchain seen:** cyrius **6.6.14** (ganita 1.2.11, hisab's pin from 3.3.4). The SVD bodies are
byte-identical in every cyrius tag from 6.6.1 (ganita 1.2.4) to 6.6.14.
**Severity:** Low for hisab. ganita never returns a wrong value here; it refuses. Through 3.3.3 hisab
reported the refusal; since 3.3.4 it answers through its own SVD (see *What hisab does*). No
consumer calls the SVD wrappers.
**Status:** 🟡 **OPEN UPSTREAM.** This was roadmap [3.3.4] *Needs the maintainer's answer* (a). The
maintainer approved filing on 2026-10-03. It is filed as
`ganita/docs/development/issues/2026-10-03-mat-svd-no-convergence-on-tiny-columns.md`, with the
self-proving repro `ganita/docs/development/issues/repros/2026-10-03-mat-svd-no-convergence-on-tiny-columns.cyr`.
hisab wrote both files and committed neither; committing them is ganita's own process.

## The defect

For each column pair, ganita forms α = |w_p|², β = |w_q|² and γ = w_p · w_q as plain sums of
products of A / max|a_ij|, and rotates the pair while |γ| > 2^-52 · sqrt(α·β). Four things can keep
a pair rotating. The sweep then runs its 60 sweeps and returns −1:

1. **ζ² overflows.** The rotation is t = 1/(|ζ| + sqrt(1 + ζ²)) with ζ = (β − α)/(2γ). For
   |ζ| ≥ 2^512, ζ·ζ is +∞, so t = 0 and the rotation is the identity. Nothing changes, and the next
   sweep finds the same pair.
2. **α·β underflows.** A column whose scaled norm is below about 2^-537 has α = 0. The orthogonality
   threshold 2^-52 · sqrt(α·β) is then 0, and a rounding-level γ keeps every sweep rotating. The
   ganita repro's B1 and B2 fail this way.
3. **A null column stays parallel.** In a rank-deficient A, the rotations wear one column of W down
   toward zero. For the zero-row witness Z below, what is left of column 0 is rounding residue nearly
   parallel to the column it is paired with: |γ| / sqrt(α·β) is above 0.9999 in every sweep from 2 to
   11, so the pair always rotates. Each sweep leaves a residue 2^-52 to 2^-57 times smaller, still
   parallel. From sweep 12, α is 0 and so is the threshold, while each γ stays fixed
   (`0x9DE12376203B13D8` for the pair (0, 1)) and ζ² overflows. Both pairs are then stuck in
   mechanisms 1 and 2 at once.
4. **A two-cycle at rounding level.** For the full-rank 5×4 witness below, α ≈ 0.61 and β ≈ 0.86 are
   normal and ζ² is finite, so neither of the first two applies. From sweep 3 to 59 only the pair
   (0, 1) rotates. γ alternates between ±`0x3CA8000000000000` (1.5 · 2^-53) against a threshold of
   `0x3CA725BAACD58D30` (about 1.45 · 2^-53). Each rotation flips γ's sign but not its magnitude.

hisab's family is A = [[1, t], [t, t], [t, −t]], where ζ ≈ −1/(2t). The boundary is exact:
- t = 2^-513 returns −1;
- the next double above it, `0x1FE0000000000001`, decomposes;
- t = 2^-512 decomposes.

The exact singular values, about (1, √2·t), are representable down to t = 2^-1074.

## Measured, 2026-10-03

Each probe ran from a scratch dir whose `cyrius.cyml` pins the version and declares hisab's 16 stdlib
leaves. The vendored files byte-match the cyrius tag: 29 of 29 for 6.6.14 and 28 of 28 for 6.6.3.
The `compiler:` line read `versions/<v>/bin/cycc`. The exact values come from python3 `Fraction`
and 80-digit `Decimal`.

- **Scan**, t = m · 2^e for e = −1074 … −1 and four mantissas: 2245 of 4296 return −1. For m = 1
  that is all 562 from 2^-1074 to 2^-513, and for each full mantissa all 561 from e = −1074 to −514.
  Every one that decomposes is within 2 ulp of exact. Output is identical under 6.6.3 and 6.6.14.
- **Random 2×2 / 3×2**, 6000 matrices with exponent spreads of ±4, ±60, ±600 and ±2100 binades:
  468 return −1. That is 0, 0, 263 and 205 by spread.
- **Square matrices with a zero or repeated row.** The entries are random, with full mantissas,
  |a| < 2 and random signs. Then one row is zeroed, or the last row is set to a copy of the first.
  For n = 3, 4, 6 and 10, 499 to 500 of each 500 return −1; for n = 2, 8 and 2 of 500. The same edits
  on the tall 3×2, 5×4 and 8×5 shapes, which keep full column rank, fail 0 of 500. So do rounded outer
  products x·yᵀ at every shape. Not every rank-deficient matrix fails: [[1, 2, 3], [4, 5, 6],
  [7, 8, 9]], rank 2, decomposes.
- **Two witnesses for mechanisms 3 and 4**, with exact σ from python3 `Fraction` and `Decimal`,
  rounded once. Each returns −1 under 6.6.3 and 6.6.14:
  - **Z** = [[1.3, 2.7, 0.4], [0, 0, 0], [0.9, 1.1, 2.3]], each entry the nearest double. σ =
    (3.6560995985065308, 1.7558290707812039, 0), that is `0x400D3FB1257408C4`, `0x3FFC17E03945F102`, 0.
    [[1, 2, 3], [1, 2, 3], [4, 5, 6]] also returns −1.
  - **The 5×4**, condition number 11.12, σ = (13.166199101181938, 11.788740275814376,
    9.8679004436648459, 1.183645125577899). It returns −1 at its own scale and scaled by 2^-827,
    2^-327 and 2^173. Its row-major bits:

    ```
    bfc2cacd2bba2bfc 3ff436f97395fc68 c00807f8a62bf805 3ff337df0357c8c9
    3fefd315acc77fe7 bfa1614b40ea6cc2 c0295cebbfddbb61 bfc32af6f3470282
    bfd6269dc8658203 c025b46bf0a392b4 3fc03ff652eed9bf 0000000000000000
    3fdf9160669d391a 3fcc98b730a0d724 bfbc46376ff5e5c4 3fc92a56372a02bb
    c02551acec11860f 400130c3519e4f30 3fc6d251f22a1e9b 3fe9e1efc1aeb952
    ```

| the ganita repro | exit | wrong rows |
|---|---|---|
| cyrius 6.6.3 fold (ganita 1.2.5), include lines dropped | 6 | A1, A4, A5, A6, B1, B2 |
| cyrius 6.6.14 fold (ganita 1.2.11), include lines dropped | 6 | the same six |
| ganita repo copy, `src/` at 1.2.11, its pin 6.6.12 | 6 | the same six |
| the same copy with the filing's branch-1 sketch | 2 | B1, B2 |

**The branch-1 sketch**, tested on a copy:
- When ζ² overflows, t = γ/(β − α); every other case keeps the existing bits.
- For 2-column matrices, where a pair whose ζ² overflows can never converge, every input that
  decomposes today (7583) returns bit-identical S, U and Vt.
- With three or more columns it can change bits. A second A/B ran 4000 random matrices each of 3×3,
  4×3, 4×4 and 5×4 over the same four spreads; 12830 of them decompose today. 34 of those change U or
  Vt bits, and 4 of the 34 also change S, all at the ±2100 spread. Each S change is below 10^-200
  of σ₁, so it is normwise negligible. But one 5×4 σ₄ goes from 5.1144e13, relative error 1.9e-16,
  to 2.2478e83.
- All 2245 scan failures and 463 of the 468 random ones decompose.
- `tests/ganita.tcyr` passes 844 / 844.
- It does not reach mechanisms 3 and 4. Z, the repeated-row 3×3 and the 5×4 still return −1. So do
  118 of the 500 random 3×3 zero-row matrices and 493 of the 500 at 10×10.

The 5 random failures left are mechanism 2. Instrumented at the 60th sweep, each shows α or β
exactly 0 and a zero threshold. The filing proposes per-pair scaled columns for that mechanism, not
prototyped. That proposal does not reach mechanism 3: there the cosine itself is about 1, so a
scale-free test still rotates.

## Exposure

hisab calls `ganita_mat_svd` in two places, both in `src/linalg_ext.cyr`, and nothing in `src/`
calls `ganita_mat_rank`, `ganita_mat_condition` or `ganita_mat_pseudo_inv`:

Before the 3.3.4 fallback (*What hisab does*):
- **`svd_compute`** returned ganita's status unchanged, so this input gave **−1**. Its doc says −1
  means one of ganita's working allocations failed or the rotations did not converge in 60 sweeps.
  It also says to compare the status with 0 and nothing else: −1 is bit-identical to
  `HSB_ERR_INVALID_TRANSFORM`. `src/error.cyr` says the same.
- **`svd_truncated`** mapped any non-zero status to **`Err(HSB_ERR_NO_CONVERGENCE)`** (−8) and left
  `out_S` untouched. Through 3.3.2 it discarded the status and reported `Ok(0)`.

Measured on the 3.3.4 tree's bundle before the fallback, which is `base334`, under 6.6.14 and under
6.6.3 with identical output:

| A | `svd_compute` | `svd_truncated(k = 1)` |
|---|---|---|
| t = 2^-512 | 0, S₀ = 1 | `Ok(0)`, S₀ = 1 |
| t = 2^-513 | −1, out_S untouched | `Err(-8)` = `HSB_ERR_NO_CONVERGENCE`, out_S untouched |
| t = 2^-1074 | −1, out_S untouched | `Err(-8)`, out_S untouched |
| Z, the zero-row 3×3 | −1, out_S untouched | `Err(-8)`, out_S untouched |
| [[1, 2, 3], [1, 2, 3], [4, 5, 6]] | −1, out_S untouched | `Err(-8)`, out_S untouched |
| the 5×4 witness | −1, out_S untouched | `Err(-8)`, out_S untouched |
| [[1, 2, 3], [4, 5, 6], [7, 8, 9]], rank 2 | 0, S₀ = `0x4030D91D4D231B23` | `Ok(0)`, the same S₀ |

On the last four, ganita's own `ganita_mat_rank` (tol 1e-12), `ganita_mat_condition` and
`ganita_mat_pseudo_inv` give −1, NaN (`0xFFF8000000000000`) and null for the three that fail, and
2, −1.0 and a matrix for the rank-2 control. hisab calls none of the three.

Until the fallback, `tests/abuse.tcyr` pinned both in the group "3.3.3 G2a: svd_truncated status,
null and out-param contract": `svd_compute`'s −1 at t = 2^-513 was the tripwire, and three
`svd_truncated` rows checked the `Err`, its code and the untouched out_S. The pins now in place are
listed under *What hisab does*.

**Consumers.** The ten direct consumers pin hisab 2.11.2, 2.22.1 or 3.2.1, and none calls
`svd_compute`, `svd_truncated` or the ganita SVD family. The search covered every `.cyr` and
`.tcyr` outside `lib/`, `dist/` and `build/`; the same search finds svara's `num_fft` and amuzesh's
`ganita_mat_*` calls as positive controls. Outside the ganita repo, the only callers of the SVD
family are hisab's `src/linalg_ext.cyr` and cyrius's `tests/tcyr/math/linalg.tcyr`. A consumer that
did call it would get the refusal under its own pin: the body is the same in ganita 1.2.4 – 1.2.11.

## What hisab does

**Since 3.3.4, `svd_compute` and `svd_truncated` fall back to `svd_golub_kahan`**
(`src/linalg_precision.cyr`), hisab's own SVD, which does not route through ganita, whenever
`ganita_mat_svd` returns −1.
- **Order.** The non-finite pre-check still runs first (−2 / `Err(HSB_ERR_INVALID_INPUT)`). ganita
  runs next, and every input it answers is returned exactly as before. The fallback runs only on
  its −1, which writes nothing. `svd_compute` hands it the caller's out-params; `svd_truncated` hands
  it the same full-size temporaries ganita had, then copies the top k as before.
- **Conventions.** These match ganita's: S non-negative and descending, U m × n, Vt = Vᵀ n × n.
  There is one difference. For an exact zero singular value ganita writes a zero U column, while
  `svd_golub_kahan` writes a unit one. On the zero-row witness, U[1][2] = 1.
- **When the fallback fails too.** `svd_compute` keeps its integer codes and returns −1 (out_S
  untouched; out_U and out_Vt may hold the fallback's partial work). `svd_truncated` returns the
  fallback's `Err` as is: `HSB_ERR_NO_CONVERGENCE`, which includes a singular value above DBL_MAX,
  or `HSB_ERR_ALLOC`, with out_S untouched. No probe row below fails both. The constructed
  [[1,2,3],[1,2,3],[4,5,6]] × 2^1021 does, because its σ₁ ≈ 1.27 · 2^1024 has no double.
- **`ganita_mat_rank`, `ganita_mat_condition` and `ganita_mat_pseudo_inv`** have no fallback. hisab
  does not call them, and a consumer that calls them directly still gets ganita's refusal.

**Measured, 2026-10-03** (cycc 6.6.14, this tree's bundle; 300 random n × n per class and n =
3..10, entries with full mantissas and exponents −4..0, plus the witnesses):

| class | ganita −1 | answered after the fallback |
|---|---|---|
| one zero row | 2399 / 2400 | 2399 |
| a duplicated row | 2397 / 2400 | 2397 |
| two zero rows | 2216 / 2400 | 2216 |
| dense, rank 1, a duplicated column | 0 / 7200 | — (ganita's answer, bit-identical) |
| [[1,t],[t,t],[t,−t]] at t = 2^-513 and 2^-1074, the two 3×3 witnesses | 4 / 4 | 4 |

- **Singular values.** Against an 80-digit Decimal one-sided-Jacobi oracle, the 7012 fallback rows
  are within 7.75 ε·σ₁. ganita's own answers on the dense, rank-1 and duplicated-column rows reach
  8.36. Every fallback row's smallest σ is exactly +0, as is the truth.
- **Orthogonality.** U and Vt are orthonormal to 14.6 ε.
- **Reconstruction.** ‖A − U S Vt‖_F is looser: 4820 ε‖A‖_F at worst (1.07e-12 relative), with a
  median of 5.8 and 1856 of 7012 rows above 100 ε. ganita's answered rows reach 12.9. This is the
  order of `svd_golub_kahan`'s ~2900 ε on graded bidiagonals, roadmap [3.6.0].
- **Wide exponent spans.** A second search ran 6000 square matrices (n = 3..5) with a zero row, a
  duplicated row, or a zero row in a rank-1 x_i(1 + j), with entry exponents spread ±30 to ±1020.
  ganita returns −1 on 3608 of them, and the fallback answers all 3608.
- **Pass-through.** All 7390 rows ganita answers come back from `svd_compute` bit-identical to a
  direct `ganita_mat_svd` call. `svd_truncated(k = n)` matches `svd_compute` bit for bit on all
  14,407 rows.
- **The two 3×3 witnesses**, against their closed forms:
  - [[1,2,3],[1,2,3],[4,5,6]], with σ² = (105 ± √10593)/2 and 0, gives S = (0x4024646BB0023E7D,
    0x3FF04ECE4EBDCE2F, +0). That is 1.6 and 3.1 ulp high, and the zero exact.
  - [[1.3,2.7,0.4],[0,0,0],[0.9,1.1,2.3]] gives S = (0x400D3FB1257408C6, 0x3FFC17E03945F102, +0).
    That is 2.3 ulp high, correctly rounded, and exact.
- **The t-family.** At t = 2^-513 the fallback gives S = (1, 0x1FE6A09E667F3BCC), 1 ulp below the
  correctly rounded √2·t. At 2^-1074 it gives S = (1, 2^-1074), correctly rounded.

**Pins in `tests/abuse.tcyr`.**
- **Group "3.3.3 G2a".** `ganita_mat_svd` returns −1 at t = 2^-513; this is the tripwire. Both
  wrappers answer there, with S pinned. The t = 2^-512 pair is the control.
- **Group "3.3.4: ganita's -1 falls back to svd_golub_kahan".** It holds:
  - two `ganita_mat_svd` −1 tripwires, the duplicated-row and zero-row 3×3s;
  - each witness's S bit-exact through both wrappers, with max |A − U S Vt| < 4 ε σ₁ (measured
    2.35 and 0.82) and the zero row's unit null column;
  - the × 2^1021 refusal through both wrappers;
  - a pass-through row, [[1..9]], where `svd_compute` must return ganita's bits, which differ from
    `svd_golub_kahan`'s.

Every new assertion was mutation-proven. The mutants removed either fallback, mapped the fallback's
`Err` to success, ignored it or recoded it, always fell back, sent Vt or U to scratch, zeroed U's
null column, and wrote out_S before the rescale check. Fixture mutants that ganita answers fail
each tripwire.

## When ganita repairs it

- **The tripwires fail.** Each `ganita_mat_svd` assertion in `tests/abuse.tcyr` (t = 2^-513, and
  the two 3×3 witnesses) fails as designed once ganita answers its matrix. The wrappers then take
  ganita's answer, so the S pins beside it move. Re-derive them from ganita's output, or replace
  the input with one ganita still refuses. If only branch 1 is repaired, the repro's B1 and B2
  still return −1 and can replace the t = 2^-513 input. Once no tripwire input reaches ganita's −1,
  retire the tripwires, keep the wrapper pins (re-derived), and archive this record.
- **The return code.** If ganita moves non-convergence to −3, its ADR 0001 code, update
  `svd_compute`'s doc comment and `src/error.cyr`'s paragraph. `svd_compute` falls back only on −1
  (`rc != 0 - 1` passes every other code through), so its test must move to −3 as well, or the
  fallback stops running. `svd_truncated` falls back on any non-zero status and is unaffected.
- **Consumers keep the old answer** until their own cyrius pin carries the fixed ganita.
