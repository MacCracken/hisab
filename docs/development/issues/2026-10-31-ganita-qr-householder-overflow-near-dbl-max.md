# ganita QR and least squares answer wrongly, with status 0, near DBL_MAX — hisab's exposure

**Status:** 🔴 **OPEN upstream (ganita)**, filed 2026-10-31 as
`ganita/docs/development/issues/2026-10-31-qr-householder-overflow-near-dbl-max.md` with a
self-proving repro (`repros/` beside it): exit 6 on ganita 1.2.5, 1.2.7, 1.2.11, 1.2.14 and 1.2.15
(the fold in every cyrius from 6.6.3 through 6.7.18), output byte-identical across all of them.

**Defect.** `ganita_mat_qr` and `ganita_mat_least_squares` form the Householder vector unscaled. A
column whose norm passes about DBL_MAX/2 overflows ‖v‖ or v₀ ± ‖x‖ while every exact factor is
representable. In one band (all-equal 2×2, 1.531·2^1022 < c ≤ 1.657·2^1022) the reflector becomes
the identity and qr returns status 0 with Q = I and R = A, every entry finite. Above it, Q and R
come back non-finite. least_squares returns (+∞, −∞) for a well-conditioned system whose solution is
about (1, −1).

**hisab's exposure.** `lyapunov_max` (`src/linalg_ext.cyr`) is the only caller. Since 3.3.7 it
refuses a non-finite Z and a non-zero status, which covers the non-finite band. The finite band
passes through: R = Z, so r00 = |Z₀₀| instead of the column norm, and the exponent is wrong with
Ok. Derived from the mechanism and not yet measured through `lyapunov_max`; the repair is a
[3.3.8] row. hisab does not call `ganita_mat_least_squares`.

**Not filed: the SVD result 3.3.7 reported.** `ganita_mat_svd` returns σ₁ = DBL_MAX for
[[h, h], [h, h]], h = 2^1023, whose σ₁ is 2^1024. The same 1-ulp-low σ₁ comes back at every scale
(0x3FFFFFFFFFFFFFFF for h = 1, 0x7E7FFFFFFFFFFFFF for h = 2^1000), within the few-ulp accuracy
ganita documents. At h = 2^1023 that ulp lands on DBL_MAX. Not a defect; hisab's wrappers prove
the exact σ₁ past DBL_MAX and refuse it.

**Close** with a paired run of the ganita repro on the bump that fixes it, and the [3.3.8] row's
`lyapunov_max` test.
