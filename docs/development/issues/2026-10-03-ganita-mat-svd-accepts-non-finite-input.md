# 2026-10-03 — ganita `ganita_mat_svd`: a NaN or infinite entry is accepted and reported as success (an all-NaN matrix has singular values (0, 0))

**Component:** cyrius stdlib `lib/ganita.cyr`: `ganita_mat_svd` → `_linalg_svd_impl`, together with
`ganita_mat_rank`, `ganita_mat_condition` and `ganita_mat_pseudo_inv`, which share it. The source
lives in the ganita repo at `src/linalg.cyr`, and cyrius folds it into its stdlib.
**Toolchain seen:** cyrius **6.6.14** (ganita 1.2.11, hisab's pin from 3.3.4). `ganita_mat_svd`,
`_linalg_svd_impl`, `_linalg_mat_scale`, `_linalg_norm2`, `ganita_mat_rank` and `ganita_mat_condition`
are byte-identical in every cyrius tag from 6.6.1 (ganita 1.2.4) to 6.6.14. `ganita_mat_pseudo_inv`
changed at ganita 1.2.6 (cyrius 6.6.5), and the repro's output is the same on all 14 folds.
**Severity:** Medium upstream, because it is a fabricated success. Nil for hisab ≥ 3.3.3, which
refuses a non-finite A before calling ganita.
**Status:** 🟡 **OPEN UPSTREAM.** This was roadmap [3.3.4] *Needs the maintainer's answer* (a). Filing
was approved by the maintainer on 2026-10-03. It is filed as
`ganita/docs/development/issues/2026-10-03-mat-svd-accepts-non-finite-input.md`, with the
self-proving repro `ganita/docs/development/issues/repros/2026-10-03-mat-svd-accepts-non-finite-input.cyr`.
hisab wrote both files and committed neither; committing them is ganita's own process.

## The defect

`_linalg_svd_impl` never asks whether A is finite, and three steps then drop a NaN quietly:
- **The scale.** It is an ordered max that a NaN never wins, so an all-NaN A has scale 0. It takes
  the matrix-of-zeros branch: status 0, every σ = 0, U = 0, V = I.
- **The sweep.** A NaN in α, β or γ makes the orthogonality test false, so the pair counts as
  orthogonal.
- **The norms.** σ_j is `_linalg_norm2` of column j, another ordered max, which returns 0 for a
  column whose only non-zero entries are NaN.

Measured on ganita 1.2.11 (cyrius 6.6.14), with identical output under 6.6.3 (ganita 1.2.5):

| A (2×2) | `ganita_mat_svd` | rank (tol 1e-12) | condition | pseudo_inv [0][0] |
|---|---|---|---|---|
| all NaN | **0**, S = **(0, 0)** | **0** | **−1.0** | **0** |
| [[NaN, 0], [0, 2]] | **0**, S = **(2, 0)** | **1** | **−1.0** | **0** |
| [[+Inf, 0], [0, 2]] | **0**, S = (NaN, NaN) | **0** | NaN | **0** |
| [[3, NaN], [0, 2]] | **0**, S = (3, NaN) | **1** | NaN | NaN |
| [[3, 0], [0, NaN]] | **0**, S = **(3, 0)** | **1** | **−1.0** | **1/3** |
| diag(3, 2), control | 0, S = (3, 2) | 2 | 1.5 | 1/3 |

The second and fifth rows were not in hisab's 3.3.3 measurement. In both the NaN entry vanishes, and
every answer is finite and plausible. The ganita repro counts 18 wrong checks across five non-finite
inputs and exits **18**, with the same output each way:
- from every cyrius fold from 6.6.1 to 6.6.14, with the include lines dropped, each built from a dir
  pinned to that version;
- from a copy of the ganita repo at 1.2.11, under its own pin, 6.6.12.

The filing's sketch, tested on that copy, is one finiteness scan at the top of `_linalg_svd_impl`,
with `rank` passing the status through. With it the repro exits **0** and `tests/ganita.tcyr` passes
844 / 844. Partial scans are caught:
- a mutant that scans only `[0][0]` leaves 7 checks wrong;
- mutants that scan `rows`, `cols` or `rows * cols − 1` entries leave 4 each, all on the
  [[3, 0], [0, NaN]] row. Before that row was added the repro had 14 checks, and those three mutants
  exited 0.

## Exposure

hisab calls `ganita_mat_svd` in two places, both in `src/linalg_ext.cyr`, and nothing in `src/`
calls `ganita_mat_rank`, `ganita_mat_condition` or `ganita_mat_pseudo_inv`. Since 3.3.3 both callers
scan A with `_lext_mat_all_finite` before calling ganita. It uses one `f64_lt(|x|, +Inf) == 0` test,
which catches ±Inf and NaN together:

- **`svd_compute`** returns **−2**, ganita's own code for an argument it cannot decompose, and leaves
  out_U, out_S and out_Vt untouched.
- **`svd_truncated`** returns **`Err(HSB_ERR_INVALID_INPUT)`** (−10), also before allocating its
  temporaries.

Measured on the 3.3.4 tree's bundle, which is `base334`, under 6.6.14 and under 6.6.3 with identical
output. All-NaN, [[NaN, 0], [0, 2]], [[+Inf, 0], [0, 2]] and [[3, 0], [0, NaN]] each give
`svd_compute` −2 and `svd_truncated` `Err(-10)`, and out_S keeps its 7.0 stamp in both. diag(3, 2)
gives 0 and `Ok(0)` with S₀ = 3.

The suites pin it:
- `tests/abuse.tcyr`, group "3.3.3 W3: svd_compute refuses a non-finite A". All-NaN,
  [[3, NaN], [0, 2]], [[±Inf, 0], [0, 2]], a 3×2 with NaN at [2][1] and a finite wide 2×3 (ganita's
  own −2) each return −2 and leave the 7.0 stamps in out_S, out_U and out_Vt untouched. The
  diag(3, 2) control asserts status 0 and S = (3, 2) bit-exact;
- `tests/abuse.tcyr`, the "A NON-FINITE ENTRY" rows of the G2a group, for `svd_truncated`.

These pin hisab's own guard, so they do not move when ganita is repaired. **Consumers** pin hisab
2.11.2, 2.22.1 or 3.2.1, all before the guard. None of the ten calls `svd_compute`, `svd_truncated`
or the ganita SVD family; the search is the one in the companion record, with its positive control.

## What hisab does

It keeps the guard. It is hisab's contract for both functions, and consumers compile the bundle
against their own pins (cyrius 6.6.2–6.6.10, ganita 1.2.4–1.2.8), which will not carry a fix. Its
comments (`src/linalg_ext.cyr`, above `_lext_mat_all_finite` and `svd_compute`) describe ganita's
behaviour as measured on 3.3.2; the second and fifth table rows above add cases they do not list.

## When ganita repairs it

- No hisab assertion fails, because the guard answers first.
- Re-run the ganita repro from a dir pinned to the new cyrius, and record the paired before and after
  here.
- Rewrite the two comments to say the guard stays for consumers on older pins.
- Archive this record.

The companion filing is `2026-10-03-ganita-mat-svd-no-convergence-on-tiny-columns.md`.
