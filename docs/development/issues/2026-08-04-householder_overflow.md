<!-- 2.7.0-C verification record. Produced by an independent verify agent and an
adversarial challenge agent, both of which RAN the code rather than reasoning about it.
Verdict: CONFIRMED, challenge upheld. The fix below is specified but NOT YET APPLIED.
Apply it the same way the other 2.7.0-C fixes landed: patch src/, add the assertions,
then mutation-prove by reverting the file and confirming the new assertions fail. -->

# householder_overflow

SITE: src/linalg_precision.cyr:31
VERDICT: CONFIRMED

## CURRENT CODE
fn _lp_vec_norm(arr, start, len) {
    var sum = f64_from(0);
    var i = 0;
    while (i < len) {
        var v = load64(arr + (start + i) * 8);
        sum = f64_add(sum, f64_mul(v, v));
        i = i + 1;
    }
    return f64_sqrt(sum);
}

## MINIMAL FIX
Three new helpers next to the existing `_lp_*` internals (insert after `_lp_vec_norm`, before `_lp_vec_new`):

# Largest |entry| of a stdlib matrix (0 for the zero matrix).
fn _lp_mat_max_abs(A) {
    var total = mat_rows(A) * mat_cols(A);
    var mx = f64_from(0);
    var i = 0;
    while (i < total) {
        var a = f64_abs(load64(A + 16 + i * 8));
        if (f64_gt(a, mx) == 1) { mx = a; }
        i = i + 1;
    }
    return mx;
}

# Fresh copy of A with every entry divided by s (s > 0). Divide, never multiply
# by 1/s: a subnormal s makes 1/s overflow to +Inf.
fn _lp_mat_div_scalar(A, s) {
    var B = mat_copy(A);
    if (B == 0) { return 0; }
    var total = mat_rows(A) * mat_cols(A);
    var i = 0;
    while (i < total) {
        store64(B + 16 + i * 8, f64_div(load64(B + 16 + i * 8), s));
        i = i + 1;
    }
    return B;
}

# Largest |re| or |im| over a complex matrix (0 for the zero matrix).
fn _lp_cmat_max_abs(A) {
    var rows = cmat_rows(A);
    var cols = cmat_cols(A);
    var mx = f64_from(0);
    var i = 0;
    while (i < rows) {
        var j = 0;
        while (j < cols) {
            var z = cmat_get(A, i, j);
            var a = f64_abs(HComplex_re(z));
            if (f64_gt(a, mx) == 1) { mx = a; }
            a = f64_abs(HComplex_im(z));
            if (f64_gt(a, mx) == 1) { mx = a; }
            j = j + 1;
        }
        i = i + 1;
    }
    return mx;
}

--- svd_golub_kahan: insert after `var max_iter = 30 * n * n;`, and change the
--- _lp_bidiagonalize argument from A to As:

    # Balance to unit max magnitude before decomposing; undo it on the singular
    # values at the end. EVERY step downstream squares entries -- the Householder
    # norms in _lp_bidiagonalize and the Wilkinson shift in _lp_bidiag_qr, which
    # forms the entries of B^T*B explicitly -- and |x| > 1.34e154 squares to +Inf
    # while |x| < 1.49e-154 squares to 0. Unbalanced, a matrix of 1e200 entries
    # (representable, with equally representable singular values) gave +Inf, then
    # 2/Inf = 0 and 0*Inf = NaN, and this returned HSB_ERR_NO_CONVERGENCE with U
    # all NaN; a matrix of 1e-200 entries was worse -- every reflector was skipped
    # and it returned HSB_ERR_NONE with S set to the raw diagonal of A (42.6% off).
    # Singular values are homogeneous of degree 1, sv(c*A) = c*sv(A) for c > 0,
    # and U and Vt are invariant, so the balance costs one rounding.
    var sc = _lp_mat_max_abs(A);
    if (f64_gt(sc, f64_from(0)) == 0) { sc = F64_ONE; }
    var As = _lp_mat_div_scalar(A, sc);
    if (As == 0) { return HSB_ERR_ALLOC; }

    var err = _lp_bidiagonalize(As, m, n, diag, superdiag, out_U, out_Vt);

--- svd_golub_kahan: insert immediately before `# Copy singular values to output`:

    # Undo the balancing: sv(c*A) = c*sv(A), U and Vt are invariant.
    i = 0;
    while (i < n) {
        store64(diag + i * 8, f64_mul(sc, load64(diag + i * 8)));
        i = i + 1;
    }

--- eigen_qr: insert after the `if (n == 1) { ... }` early return, and change the
--- _lp_tridiagonalize argument from A to As:

    # Balance to unit max magnitude first; see svd_golub_kahan for why. The
    # Wilkinson shift in _lp_tridiag_qr squares the offdiagonal (e2sq = e2*e2),
    # so this overflowed at 1e200 exactly as the Householder norm did.
    var sc = _lp_mat_max_abs(A);
    if (f64_gt(sc, f64_from(0)) == 0) { sc = F64_ONE; }
    var As = _lp_mat_div_scalar(A, sc);
    if (As == 0) { return HSB_ERR_ALLOC; }

    var err = _lp_tridiagonalize(As, n, diag, offdiag, out_vecs);

--- eigen_qr: insert immediately before `# Step 3: Copy eigenvalues, sort by descending magnitude`:

    # Undo the balancing: eig(c*A) = c*eig(A), eigenvectors invariant.
    var b = 0;
    while (b < n) {
        store64(diag + b * 8, f64_mul(sc, load64(diag + b * 8)));
        b = b + 1;
    }

--- cqr_decompose: insert after `var n = cmat_cols(A);` and fold the divide into
--- the existing copy-into-out_R loop (no extra allocation needed):

    # Balance to unit max magnitude on the way into out_R; undo it on R at the
    # end. Q is invariant under a positive rescale of A. Unbalanced, ||v||^2 = Inf
    # made cx_abs(x1) = Inf, phase = cx_div(x1, Inf) = NaN and v^H v = NaN, so the
    # `f64_gt(vhv, EPSILON_F64)` guard was false and EVERY reflector was skipped:
    # at 1e200 this returned HSB_ERR_NONE with Q = I and R = A, i.e. R not even
    # upper triangular.
    var sc = _lp_cmat_max_abs(A);
    if (f64_gt(sc, f64_from(0)) == 0) { sc = F64_ONE; }

    # Copy A into out_R
    var i = 0;
    while (i < m) {
        var j = 0;
        while (j < n) {
            var aij = cmat_get(A, i, j);
            cmat_set(out_R, i, j, cx_new(f64_div(HComplex_re(aij), sc),
            f64_div(HComplex_im(aij), sc)));
            j = j + 1;
        }
        i = i + 1;
    }

--- cqr_decompose: insert immediately before `# Q has accumulated H_k ... H_1.`:

    # Undo the balancing on R: A/sc = Q*R implies A = Q*(sc*R).
    i = 0;
    while (i < m) {
        var j = 0;
        while (j < n) {
            cmat_set(out_R, i, j, cx_scale(cmat_get(out_R, i, j), sc));
            j = j + 1;
        }
        i = i + 1;
    }

## ASSERTIONS
# Relative closeness helper -- LOOSE_TOL_H is an ABSOLUTE 1e-8 and is useless at
# 1e200 scale, so these need a relative comparison.
var _LP_RELTOL = 0x3DDB7CDFD9D7BDBB;   # 1e-10
var _LP_DBLMAX = 0x7FEFFFFFFFFFFFFF;
fn _lp_relclose(a, b) {
    if (f64_eq(a, a) == 0) { return 0; }                          # NaN
    if (f64_gt(f64_abs(a), _LP_DBLMAX) == 1) { return 0; }        # +/-Inf
    if (f64_lt(f64_abs(f64_sub(a, b)), f64_mul(_LP_RELTOL, f64_abs(b))) == 1) { return 1; }
    return 0;
}

var _LP_BIG = 0x6974E718D7D7625A;   # 1e200
var lpo_A = mat_new(3, 3);
mat_set(lpo_A, 0, 0, f64_mul(F64_ONE, _LP_BIG));
mat_set(lpo_A, 0, 1, f64_mul(F64_TWO, _LP_BIG));
mat_set(lpo_A, 0, 2, f64_mul(f64_from(3), _LP_BIG));
mat_set(lpo_A, 1, 0, f64_mul(f64_from(4), _LP_BIG));
mat_set(lpo_A, 1, 1, f64_mul(f64_from(5), _LP_BIG));
mat_set(lpo_A, 1, 2, f64_mul(f64_from(6), _LP_BIG));
mat_set(lpo_A, 2, 0, f64_mul(f64_from(7), _LP_BIG));
mat_set(lpo_A, 2, 1, f64_mul(f64_from(8), _LP_BIG));
mat_set(lpo_A, 2, 2, f64_mul(f64_from(10), _LP_BIG));
var lpo_U = mat_new(3, 3);
var lpo_Vt = mat_new(3, 3);
var lpo_S = alloc(24);
assert_eq(svd_golub_kahan(lpo_A, lpo_U, lpo_S, lpo_Vt), HSB_ERR_NONE,
"svd @1e200: converges (was HSB_ERR_NO_CONVERGENCE, U all NaN)");
# 17.412505166808593 * 1e200
assert(_lp_relclose(load64(lpo_S), 0x69B6BF80142486CE) == 1,
"svd @1e200: S[0] == 1.7412505166808593e201");
  # WHY: CURRENT: svd_golub_kahan returns -8 (HSB_ERR_NO_CONVERGENCE) -- measured -- so the first assert_eq prints "(got -8, expected 0)". Because it returns early at the `if (err != HSB_ERR_NONE) { return err; }` after _lp_bidiag_qr, out_S is NEVER written, so load64(lpo_S) is the zero-filled alloc value +0.0 and the second assertion fails too (|0 - 1.74e201| is not < 1e-10*1.74e201). FIXED: returns 0 and

var _LP_TINY = 0x16687E92154EF7AC;   # 1e-200
var lpu_A = mat_new(3, 3);
mat_set(lpu_A, 0, 0, f64_mul(F64_ONE, _LP_TINY));
mat_set(lpu_A, 0, 1, f64_mul(F64_TWO, _LP_TINY));
mat_set(lpu_A, 0, 2, f64_mul(f64_from(3), _LP_TINY));
mat_set(lpu_A, 1, 0, f64_mul(f64_from(4), _LP_TINY));
mat_set(lpu_A, 1, 1, f64_mul(f64_from(5), _LP_TINY));
mat_set(lpu_A, 1, 2, f64_mul(f64_from(6), _LP_TINY));
mat_set(lpu_A, 2, 0, f64_mul(f64_from(7), _LP_TINY));
mat_set(lpu_A, 2, 1, f64_mul(f64_from(8), _LP_TINY));
mat_set(lpu_A, 2, 2, f64_mul(f64_from(10), _LP_TINY));
var lpu_U = mat_new(3, 3);
var lpu_Vt = mat_new(3, 3);
var lpu_S = alloc(24);
assert_eq(svd_golub_kahan(lpu_A, lpu_U, lpu_S, lpu_Vt), HSB_ERR_NONE, "svd @1e-200: returns ERR_NONE");
# 17.412505166808593 * 1e-200; the underflowed path returned 1e-199 (= raw A[2][2])
assert(_lp_relclose(load64(lpu_S), 0x16AAA8257F7EC875) == 1,
"svd @1e-200: S[0] == 1.7412505166808593e-199 (was 1e-199, the raw A[2][2])");
  # WHY: This is the SILENT half and the most valuable assertion of the set. CURRENT: svd_golub_kahan returns 0 (HSB_ERR_NONE) -- so the error-code assert passes both ways and is useless on its own -- but S[0] = 1e-199 (bit pattern 0x169E9E369AA2B597), which is literally A[2][2], not a singular value. Squares underflow to 0, so `f64_gt(norm_l, EPSILON_F64)` is false and every reflector is skipped; then _lp

var lpo_E = mat_new(3, 3);
mat_set(lpo_E, 0, 0, f64_mul(f64_from(4), _LP_BIG));
mat_set(lpo_E, 0, 1, f64_mul(F64_ONE, _LP_BIG));
mat_set(lpo_E, 0, 2, f64_from(0));
mat_set(lpo_E, 1, 0, f64_mul(F64_ONE, _LP_BIG));
mat_set(lpo_E, 1, 1, f64_mul(f64_from(3), _LP_BIG));
mat_set(lpo_E, 1, 2, f64_mul(F64_ONE, _LP_BIG));
mat_set(lpo_E, 2, 0, f64_from(0));
mat_set(lpo_E, 2, 1, f64_mul(F64_ONE, _LP_BIG));
mat_set(lpo_E, 2, 2, f64_mul(F64_TWO, _LP_BIG));
var lpo_ev = alloc(24);
var lpo_EV = mat_new(3, 3);
assert_eq(eigen_qr(lpo_E, 3, 10000, lpo_ev, lpo_EV), HSB_ERR_NONE,
"eigen_qr @1e200: converges (was HSB_ERR_NO_CONVERGENCE)");
# 4.732050807568877 * 1e200
assert(_lp_relclose(load64(lpo_ev), 0x6998BA6A7000200E) == 1,
"eigen_qr @1e200: lambda0 == 4.732050807568877e200");
  # WHY: CURRENT: eigen_qr returns -8 (measured, "got -8, expected 0"). _lp_tridiagonalize produces NaN for diag[1], diag[2] and for Q (verified directly), and out_vals is never written because eigen_qr returns before `_lp_vec_copy(out_vals, diag, n)`, so load64(lpo_ev) is +0.0 -> second assertion also fails. FIXED: returns 0 with lambda0 = 4.732050807568877e200. This is the same matrix tests/hisab.tcyr al

var lpc_A = cmat_new(2, 2);
cmat_set(lpc_A, 0, 0, cx_new(f64_mul(f64_from(3), _LP_BIG), f64_from(0)));
cmat_set(lpc_A, 0, 1, cx_new(f64_mul(F64_ONE, _LP_BIG), f64_from(0)));
cmat_set(lpc_A, 1, 0, cx_new(f64_mul(f64_from(4), _LP_BIG), f64_from(0)));
cmat_set(lpc_A, 1, 1, cx_new(f64_mul(F64_TWO, _LP_BIG), f64_from(0)));
var lpc_Q = cmat_new(2, 2);
var lpc_R = cmat_new(2, 2);
assert_eq(cqr_decompose(lpc_A, lpc_Q, lpc_R), HSB_ERR_NONE, "cqr @1e200: returns ERR_NONE");
# |R[0,0]| must be ||col0|| = 5e200
assert(_lp_relclose(f64_abs(HComplex_re(cmat_get(lpc_R, 0, 0))), 0x699A20DF0DCD3AF1) == 1,
"cqr @1e200: |R[0,0]| == 5e200 (was 3e200 -- reflectors skipped, R == A)");
# R must be upper triangular
assert(f64_lt(f64_abs(HComplex_re(cmat_get(lpc_R, 1, 0))), 0x6761F48EAF234AD4) == 1,
"cqr @1e200: |R[1,0]| < 1e190 (was 4e200 -- R not upper triangular)");
  # WHY: CURRENT: cqr_decompose returns 0 (HSB_ERR_NONE) with Q = I and R = A verbatim -- dumped and confirmed: R[0,0]=3e200, R[0,1]=1e200, R[1,0]=4e200, R[1,1]=2e200, Q[0,0]=1, Q[1,0]=0. The error-code assert passes both ways (deliberately -- it documents that the failure is silent), but |R[0,0]| is 3e200 not 5e200 and |R[1,0]| is 4e200 not ~0, so both value assertions fail. FIXED: |R[0,0]| = 5e200 and R[

## RISKS
1. NEW RETURN VALUE. `svd_golub_kahan` and `eigen_qr` can now return HSB_ERR_ALLOC (-11) when `mat_copy` fails, which their header comments do not list. The guard is deliberate — 2.6.14 established that storing through a null `mat_*` return is the memory-safety bug class this repo just finished discharging, and `_lp_bidiagonalize`'s existing bare `var W = mat_copy(A);` (line 100) is already an unguarded instance. Update the two header comment blocks (lines 84 and 627) or the docs will drift again. If you would rather not widen the contract, drop the `if (As == 0) { return HSB_ERR_ALLOC; }` lines — but then `_lp_mat_div_scalar` must not be reached with a null B.

2. EXTRA ALLOCATION. `_lp_mat_div_scalar` allocates one more m*n matrix per call to svd_golub_kahan / eigen_qr, on top of the `mat_copy(A)` already done inside `_lp_bidiagonalize` / `_lp_tridiagonalize` — peak memory for those two entry points goes from 1 to 2 copies of A. Cost is O(mn) against an O(mn^2) decomposition. There is no bench for svd/eigen/cqr in tests/hisab.bcyr (checked), so no benchmark gate is affected, but if you add one, take the baseline before landing this. `cqr_decompose` allocates nothing extra — the divide folds into the copy-into-out_R loop it already ran.

3. WHAT THIS DOES NOT FIX. `_lp_vec_norm` (line 31/36) and the three `vtv` accumulators are still textually unscaled. They are now unreachable with out-of-range magnitudes because every public entry point balances first — but a direct caller

## CHALLENGE
fix_correct=True assertions_discriminate=True stands=True

- NO QUOTE DRIFT, NO MECHANISM DRIFT. I reproduced every single number they reported, byte for byte. `_lp_vec_norm` at /home/macro/Repos/hisab/src/linalg_precision.cyr:31-40 matches their quote verbatim. All cited squaring sites verified: 36, 163, 224, 672, 1059, 1085, 396-411. Only drift found: `e2sq = f64_mul(e2, e2)` is at line 825, not 826. Immaterial.
- MEASURED BASELINE (unpatched src, probes under scratchpad): `_lp_vec_norm([1e200,1e200])` returns bits 0x7FF0000000000000 = +Inf; `_lp_vec_norm([1e-200,1e-200])` returns exactly +0.0. `svd_golub_kahan(1e200*[[1,2,3],[4,5,6],[7,8,10]])` returns -8, out_S never written. `svd_golub_kahan(1e-200*same)` returns 0 (ERR_NONE) with S[0]=0x169E9E369AA2B597=1e-199 and S[1]=5e-200 -- literally A[2][2] and A[1][1], the raw diagonal, silently. `eigen_qr(1e200*[[4,1,0],[1,3,1],[0,1,2]])` returns -8. `cqr_decompose(1e200*[[3,1],[4,2]])` returns 0 with R[0,0]=3e200 (should be 5e200) and R[1,0]=4e200 (should be 0) -- R is not upper triangular. All six claims CONFIRMED.
- FIX COMPILES AND PASSES EVERY GATE. Applied verbatim to a scratch copy: `cyrius lint src/linalg_precision.cyr` = 0 warnings; `cyrius fmt src/linalg_precision.cyr --check` rc=0; `scripts/check-constants.sh` 145/145; `cyrius vet src/main.cyr` clean; all four suites unchanged at 205/307/470/199 = 1181/1181; `cyrius bench tests/hisab.bcyr` completes. No line exceeds 110 chars. No `f64_from` on a hex literal, no integer compare on an f64, no `else if`, no negative literals, no reserved `iv_*` names. Every identifier exists: `mat_rows`/`mat_cols`/`mat_copy` (lib/ganita.cyr:1349-1363, data at offset +16 confirmed by ganita_mat_new), `cmat_rows`/`cmat_cols`/`cmat_get` (src/complex.cyr:178-188), `cx_scale`/`cx_new`/`HComplex_re`/`HComplex_im`, `HSB_ERR_ALLOC` (src/error.cyr:22 = -11). No name collision for `_lp_mat_max_abs`/`_lp_mat_div_scalar`/`_lp_cmat_max_abs`.
- ASSERTIONS: 7 of 9 discriminate. Ran the four blocks verbatim in both trees: CURRENT = 2 passed / 7 failed; FIXED = 9 passed / 0 failed. The two that pass both ways are assertion-2's `svd @1e-200: returns ERR_NONE` and assertion-4's `cqr @1e200: returns ERR_NONE` -- both VACUOUS, but the author explicitly flags each as deliberately documenting that the failure is silent, so this is disclosure, not padding. Every hex constant in the assertions decodes correctly: 0x3DDB7CDFD9D7BDBB=1e-10, 0x6974E718D7D7625A=1e200, 0x16687E92154EF7AC=1e-200, 0x69B6BF80142486CE=1.7412505166808593e201, 0x16AAA8257F7EC875=1.7412505166808593e-199, 0x6998BA6A7000200E=4.732050807568877e200, 0x699A20DF0DCD3AF1=5e200, 0x6761F48EAF234AD4=1e190. numpy confirms sv(A)=17.412505166808593, eigvalsh=4.732050807568877, ||col0 [3,4]||=5.
- DOCUMENTED CONTRACT CHANGE, NOT CALLED OUT BY THE AUTHOR -- must be fixed before landing. The patch makes `svd_golub_kahan` and `eigen_qr` able to return HSB_ERR_ALLOC (-11), which contradicts three doc comments the patch leaves untouched: src/linalg_precision.cyr:15 ('All functions use out-params and return error codes (HSB_ERR_NONE / HSB_ERR_NO_CONVERGENCE)'), :84 and :539 ('Returns: HSB_ERR_NONE on success, HSB_ERR_NO_CONVERGENCE if QR iteration fails'), :627 and :933 (same for eigen_qr). Given this repo just shipped a release fixing six doc drifts, shipping a seventh in the same file would be poor form.
- THE FINDING'S HEADLINE REMEDY IS NOT WHAT THE PATCH DOES, and the patch is right. `_lp_vec_norm` at line 31 -- the site the finding names -- is left completely untouched. The author already measured why (scaling only inside _lp_vec_norm gets 11/18; scaling all three real Householder columns gets 16/18; the Wilkinson shifts at 396-411 and 825 overflow independently). This is correct, but the finding title, the site line number, and any CHANGELOG entry should describe the actual remedy (LAPACK dlascl-style balancing at the public entry points), not 'scale the column by its max-abs'. `_lp_vec_norm`, `_lp_bidiagonalize`, `_lp_tridiagonalize`, `_lp_bidiag_qr`, `_lp_tridiag_qr` all remain individually overflow-prone; they have no callers outside the module today, but Cyrius has ONE flat global namespace, so a consumer of dist/hisab.cyr can call them directly. Worth a one-line caveat in the hea
- 1-ULP UNIT-SCALE DRIFT, avoidable. With the max-abs divisor as written, `svd_golub_kahan` on the existing unit-scale test matrix changes S[0] from 17.412505166808593 to 17.412505166808597 (bits ...522 -> ...523). Harmless against LOOSE_TOL_H (absolute 1e-8) and all suites still pass, but it is a gratuitous change to a shipped numeric result. Rounding the divisor down to a power of two makes both the divide and the un-scale EXACT: I measured the unit-scale output back to bits ...522, BIT-IDENTICAL to today, with all 9 proposed assertions and all 1181 suite assertions still passing. See corrected_fix.
- dist/hisab.cyr MUST BE REGENERATED. The patch changes src/, and dist/hisab.cyr is the 34-module bundle CI diffs for drift (`cyrius distlib`). The distlib-version-coupling note in memory applies. Also note lib/hisab.cyr and dist/hisab.cyr are currently NOT byte-identical in the repo -- pre-existing, unrelated, but worth a glance.
- TEST-SIDE NAMESPACE HYGIENE. `fn _lp_relclose` is defined in a .tcyr using the module's own `_lp_` internal prefix. Cyrius is one flat namespace with last-definition-wins and the test includes src first, so a future `_lp_relclose` added to src/linalg_precision.cyr would be SILENTLY overridden by the test's version -- exactly the class of trap this repo already hit with the iv_add/iv_sub/iv_mul lexer issue. Rename to something like `lp_test_relclose`. Separately, assertion blocks 2/3/4 depend on `_lp_relclose`, `_LP_BIG` and `_LP_TINY` declared in blocks 1 and 2, so they must be inserted in order in a single file.
- PERF/MEMORY: `_lp_mat_div_scalar` adds one `mat_copy(A)` per svd/eigen call on top of the one `_lp_bidiagonalize`/`_lp_tridiagonalize` already do, plus an O(mn) max-abs scan -- negligible against O(n^3), and the bump allocator never frees so it is 2x on one allocation, not a new leak class. But there are ZERO svd/eigen/cqr benchmarks in tests/hisab.bcyr, so nothing would catch a regression here. Optional follow-up: pass the pre-scaled W straight into the bidiagonalize/tridiagonalize to avoid the double copy.
- THE FINDING UNDER-STATES THE BLAST RADIUS -- this is not an exotic-input curiosity. I ran 40 random 4x4 matrices per scale across 1e-200 .. 1e200 with a 1e-10 RELATIVE reconstruction check (|A - U*S*Vt|_max / |A|_max). CURRENT svd_golub_kahan fails 40/40 at scale 1e-10, 40/40 at 1e-23, 40/40 at 1e-100/1e-200/1e100/1e200, and 15/40 at 1e-4 -- it is only correct in the band 1 .. 1e23. CURRENT eigen_qr fails 40/40 at 1e-10 and 26/40 at 1e-4. Root cause of the small-magnitude half is the ABSOLUTE EPSILON_F64 = 1e-12 deflation/skip thresholds, which the balancing incidentally converts into relative ones. FIXED: 11/11 scales clean for both (440 matrices each). A physics consumer (impetus) working in SI metres at 1e-10 gets silently wrong singular values TODAY. That materially raises the severity.
- SEPARATE PRE-EXISTING DEFECT UNCOVERED (not caused by this fix, worth its own finding): `eigen_qr` returns HSB_ERR_NO_CONVERGENCE at UNIT scale for a well-conditioned symmetric 4x4 (numpy cond = 6.85, eigenvalues -1.877, -0.278, 0.274, 1.387), even at max_iter = 1,000,000. Reproduced identically on the UNPATCHED repo src. Bit pattern of the matrix (row-major): 0xBFEAC083126E978C, 0x3FEDE353F7CED917, 0xBFEF8D4FDF3B645A, 0xBFE4BC6A7EF9DB23 / 0x3FEDE353F7CED917, 0x3FD8C49BA5E353F8, 0xBFCF1A9FBE76C8B3, 0x3FC6E978D4FDF3B6 / 0xBFEF8D4FDF3B645A, 0xBFCF1A9FBE76C8B3, 0x3F95810624DD2F1B, 0x3FC0E5604189374C / 0xBFE4BC6A7EF9DB23, 0x3FC6E978D4FDF3B6, 0x3FC0E5604189374C, 0xBFB0E5604189374C. Repro file: /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/preexist.tcyr

### CORRECTED FIX
The fix is sound and lands as written. Two deltas required, one optional-but-free.

DELTA 1 (REQUIRED -- doc contract). The patch introduces HSB_ERR_ALLOC as a new return of svd_golub_kahan and eigen_qr. Update the four doc comments the patch leaves stale in src/linalg_precision.cyr:

  line 15:  # All functions use out-params and return error codes
            # (HSB_ERR_NONE / HSB_ERR_NO_CONVERGENCE / HSB_ERR_ALLOC).

  lines 84 and 539 (svd_golub_kahan):
            # Returns: HSB_ERR_NONE on success, HSB_ERR_NO_CONVERGENCE if QR
            #          iteration fails, HSB_ERR_ALLOC if the balancing copy fails.
            # A is balanced to unit max magnitude before decomposing and the
            # singular values are rescaled on the way out; U and Vt are
            # invariant under a positive rescale of A.

  lines 627 and 933 (eigen_qr): same two additions, "eigenvalues"/"eigenvectors"
  instead of "singular values"/"U and Vt".

  cqr_decompose (line 1000): note that R is computed balanced and rescaled, and
  that Q is invariant.

Also add one line to the header saying the _lp_* internals are NOT individually
overflow-safe and must only be reached through the balanced public entry points.

DELTA 2 (STRONGLY RECOMMENDED -- makes the balance exact, zero ULP drift).
Round the divisor down to a power of two. Dividing and re-multiplying by 2^e is
exact, so unit-scale results come back BIT-IDENTICAL to what ships today
(measured: S[0] bits 4625594402168430522 both before and after, vs
...523 with the plain max-abs divisor). Three-line change, applied identically
at all three sites right after the existing zero guard:

    var sc = _lp_mat_max_abs(A);
    if (f64_gt(sc, f64_from(0)) == 0) { sc = F64_ONE; }
    # Round DOWN to a power of two: dividing and re-multiplying by 2^e is exact,
    # so balancing costs zero rounding and unit-scale results are unchanged.
    sc = sc & 0x7FF0000000000000;
    if (sc == 0) { sc = F64_ONE; }   # subnormal max: exponent field is 0

(for cqr use _lp_cmat_max_abs). Verified on a scratch tree: lint 0 warnings,
fmt --check rc=0, check-constants 145/145, all four suites 1181/1181, the nine
proposed assertions 9/9, and my cross-scale probes 11/11 for both svd and eigen.

DELTA 3 (test hygiene). Rename `_lp_relclose` to `lp_test_relclose` (or any
non-`_lp_` name) so the test cannot silently shadow a future src symbol under
last-definition-wins, and keep assertion blocks 1-4 in that order in one file
since 2/3/4 consume `_lp_relclose`, `_LP_BIG` and `_LP_TINY` from 1/2.

FOLLOW-UPS (separate work items, not blockers):
 - regenerate dist/hisab.cyr (`cyrius distlib`) -- CI distlib gate.
 - add svd/eigen/cqr benchmarks to tests/hisab.bcyr; there are none today.
 - file the eigen_qr unit-scale non-convergence (cond 6.85 symmetric 4x4,
   matrix bits in problems[11]) as its own finding.
 - consider widening the assertion set: the strongest evidence is not the
   1e+/-200 endpoints but scale 1e-10, where current svd fails 40/40 random
   4x4 on a 1e-10 relative reconstruction check.
