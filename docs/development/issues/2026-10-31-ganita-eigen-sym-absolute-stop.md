# ganita's symmetric eigensolver is accurate only to ~1e-12·max|a| — hisab's exposure

**Status:** 🔴 **OPEN upstream (ganita)**, filed 2026-10-31 as
`ganita/docs/development/issues/2026-10-31-eigen-sym-absolute-stop.md` with a self-proving repro
(`repros/` beside it): exit 2 on ganita 1.2.15 (`src/` and the fold in cyrius 6.7.9).

**Defect.** `ganita_mat_eigen_sym` stops its Jacobi sweep when the largest off-diagonal is below
1e-12 times the largest |a_ij|, an absolute threshold, so a pair coupled weakly against max|a|
but strongly against its own diagonal is never rotated: [[1, 1e-13], [1e-13, 1]] returns (1, 1)
after 0 rotations where the eigenvalues are 1 ± 1e-13 (450 ulp), and
diag(1e6) ⊕ [[1, 1e-7], [1e-7, 1]] returns (1e6, 1, 1). Success is reported.

**hisab's exposure.** `eigen_symmetric` (`src/linalg_ext.cyr`) wraps it and returns `Ok` with
those values (audit `AU2-G3-F3`, `docs/audit/2026-10-31-v3.4.0-full.md`). hisab's own `eigen_qr`
answers both witnesses bit-exactly; routing `eigen_symmetric` to it, or to a relative-criterion
Jacobi, is the [3.4.1] row.

**Not filed: the dot product's sign of zero (`F-S1` / `AU2-G7-F10`).** `hvec3_dot`, `hvec4_dot`
and `hquat_dot` return +0 for an all-(−0) sum because cyrius's `f64v_dot` starts its accumulator
at +0 — on x86 (`xorpd`) and on aarch64 (`eor`) alike, the same convention as reference BLAS
`ddot`. That is a consistent definition, not a compiler defect; whether hisab wants the natural
evaluation order's −0 is hisab's own [3.4.1] decision.

**Close** with a paired run of the ganita repro on the bump that fixes it.
