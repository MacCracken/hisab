<!-- 2.7.0-C verification record. Produced by an independent verify agent and an
adversarial challenge agent, both of which RAN the code rather than reasoning about it.
Verdict: CONFIRMED, challenge upheld. The fix below is specified but NOT YET APPLIED.
Apply it the same way the other 2.7.0-C fixes landed: patch src/, add the assertions,
then mutation-prove by reverting the file and confirming the new assertions fail. -->

# svd_rank_deficient

SITE: src/linalg_precision.cyr:390
VERDICT: CONFIRMED

## CURRENT CODE
            # Find p_lo: smallest index in the unreduced block
            var p_lo = p_hi - 1;
            while (p_lo > 0 &&
            f64_gt(f64_abs(load64(superdiag + (p_lo - 1) * 8)), EPSILON_F64) == 1) {
                p_lo = p_lo - 1;
            }

            # ... (lines 391-424: Wilkinson shift, unchanged) ...

            # Golub-Kahan implicit QR step (chase the bulge)
            # Initial: apply Givens to first column of B^T*B - mu*I
            var y = f64_sub(f64_mul(load64(diag + p_lo * 8),
            load64(diag + p_lo * 8)), mu);
            var z = f64_mul(load64(diag + p_lo * 8),
            load64(superdiag + p_lo * 8));

## MINIMAL FIX
Two edits to src/linalg_precision.cyr. Verified: `cyrius lint` 0 warnings, no line over 100 chars, no new hex f64 literals (check-constants.sh unaffected).

EDIT 1 — insert this helper immediately before line 328 (`# Step 2: Implicit QR iteration on bidiagonal matrix for SVD.`):

# Split the active bidiagonal block at a negligible DIAGONAL entry.
#
# The implicit-shift step in _lp_bidiag_qr builds its first rotation from the
# pair (d[p_lo]^2 - mu, d[p_lo]*f[p_lo]). When d[p_lo] is zero that pair is
# (-mu, 0), whose rotation is a bare sign flip: the sweep reproduces the block
# unchanged and spins until max_iter, so svd_golub_kahan returns
# HSB_ERR_NO_CONVERGENCE with out_S never written. Bidiagonalizing a rank-r
# m x n matrix leaves n - r zero diagonal entries, so EVERY rank-deficient input
# reaches that state -- the rank-1 [[1,2,3],[2,4,6],[3,6,9]] bidiagonalizes to
# d = (-3.7417, 0, 0), f = (13.4907, 0), and one sweep drives it to
# d = (~1e-16, 0, 0), f = (-14, 0), which then repeats forever.
#
# A negligible d[k] means row k (for k < p_hi) or column p_hi (for k == p_hi)
# of the block is empty apart from one superdiagonal entry, and a chase of exact
# Givens rotations removes that entry outright -- splitting the block and
# deflating a singular value of exactly 0. This is the zero-diagonal case of
# Golub & Van Loan Alg. 8.6.2, omitted from the original transcription.
#
# Returns 1 if a split was performed (caller must restart the sweep), else 0.
fn _lp_split_zero_diag(diag, superdiag, n, m, U_acc, Vt_acc, p_lo, p_hi) {
    # The zero test must be RELATIVE to the block: a bare EPSILON_F64 would call
    # every entry of a 1e-20-scaled block zero and no entry of a 1e+20 one.
    var scale = f64_from(0);
    var i = p_lo;
    while (i <= p_hi) {
        var dv = f64_abs(load64(diag + i * 8));
        if (f64_gt(dv, scale) == 1) { scale = dv; }
        if (i < p_hi) {
            var fv = f64_abs(load64(superdiag + i * 8));
            if (f64_gt(fv, scale) == 1) { scale = fv; }
        }
        i = i + 1;
    }
    var tol = f64_mul(EPSILON_F64, scale);

    var hit = (0 - 1);
    i = p_lo;
    while (i <= p_hi) {
        if (f64_lt(f64_abs(load64(diag + i * 8)), tol) == 1) {
            hit = i;
            break;
        }
        i = i + 1;
    }
    if (hit < 0) { return 0; }

    # Snap it to a hard zero first: both chases assume row `hit` (resp. column
    # p_hi) is empty, and a residual 1e-16 left there would be rotated into the
    # sub-diagonal and destroy the bidiagonal form.
    store64(diag + hit * 8, f64_from(0));

    if (hit < p_hi) {
        # Row `hit` holds only f[hit], at column hit+1. Rotate it away against
        # d[hit+1] (rows hit, hit+1); that pulls f[hit+1] into column hit+2 of
        # row `hit`, so the entry is chased one column right per rotation until
        # it falls off the end of the block.
        var carry = load64(superdiag + hit * 8);
        store64(superdiag + hit * 8, f64_from(0));
        var j = hit + 1;
        while (j <= p_hi) {
            var dj = load64(diag + j * 8);
            var r = f64_sqrt(f64_add(f64_mul(dj, dj), f64_mul(carry, carry)));
            if (f64_lt(r, tol) == 1) { break; }
            var cs = f64_div(dj, r);
            var sn = f64_div(carry, r);
            store64(diag + j * 8, r);
            if (j < p_hi) {
                var fj = load64(superdiag + j * 8);
                store64(superdiag + j * 8, f64_mul(cs, fj));
                carry = f64_mul(f64_neg(sn), fj);
            } else {
                carry = f64_from(0);
            }
            # Left rotation on rows (hit, j) of B == columns (hit, j) of U --
            # same G = [[cs, -sn], [sn, cs]] convention as the bulge chase.
            var t = 0;
            while (t < m) {
                var u0 = mat_get(U_acc, t, hit);
                var u1 = mat_get(U_acc, t, j);
                mat_set(U_acc, t, hit, f64_sub(f64_mul(cs, u0), f64_mul(sn, u1)));
                mat_set(U_acc, t, j, f64_add(f64_mul(sn, u0), f64_mul(cs, u1)));
                t = t + 1;
            }
            j = j + 1;
        }
        return 1;
    }

    # hit == p_hi: column p_hi holds only f[p_hi-1], at row p_hi-1. Rotate it
    # away against d[p_hi-1] (columns p_hi-1, p_hi); that pulls f[p_hi-2] into
    # row p_hi-2 of column p_hi, chasing the entry one row up per rotation.
    var ccarry = load64(superdiag + (p_hi - 1) * 8);
    store64(superdiag + (p_hi - 1) * 8, f64_from(0));
    var c = p_hi - 1;
    while (c >= p_lo) {
        var dc = load64(diag + c * 8);
        var r2 = f64_sqrt(f64_add(f64_mul(dc, dc), f64_mul(ccarry, ccarry)));
        if (f64_lt(r2, tol) == 1) { break; }
        var cs2 = f64_div(dc, r2);
        var sn2 = f64_div(f64_neg(ccarry), r2);
        store64(diag + c * 8, r2);
        if (c > p_lo) {
            var fc = load64(superdiag + (c - 1) * 8);
            store64(superdiag + (c - 1) * 8, f64_mul(cs2, fc));
            ccarry = f64_mul(sn2, fc);
        } else {
            ccarry = f64_from(0);
        }
        # Right rotation on columns (c, p_hi) of B == rows (c, p_hi) of Vt.
        var t2 = 0;
        while (t2 < n) {
            var v0 = mat_get(Vt_acc, c, t2);
            var v1 = mat_get(Vt_acc, p_hi, t2);
            mat_set(Vt_acc, c, t2, f64_sub(f64_mul(cs2, v0), f64_mul(sn2, v1)));
            mat_set(Vt_acc, p_hi, t2, f64_add(f64_mul(sn2, v0), f64_mul(cs2, v1)));
            t2 = t2 + 1;
        }
        c = c - 1;
    }
    return 1;
}

EDIT 2 — in _lp_bidiag_qr, insert at line 390, i.e. immediately after the `p_lo` search loop closes (line 389 `}`) and before the `# Wilkinson shift:` comment:

            # A zero diagonal entry inside the block makes the shifted step below
            # a no-op (see _lp_split_zero_diag), which is exactly what any
            # rank-deficient input produces. Split there instead and restart.
            if (_lp_split_zero_diag(diag, superdiag, n, m, U_acc, Vt_acc,
            p_lo, p_hi) == 1) {
                iter = iter + 1;
                continue;
            }

## ASSERTIONS
var A1 = mat_new(3, 3);
mat_set(A1, 0, 0, f64_from(1)); mat_set(A1, 0, 1, f64_from(2)); mat_set(A1, 0, 2, f64_from(3));
mat_set(A1, 1, 0, f64_from(2)); mat_set(A1, 1, 1, f64_from(4)); mat_set(A1, 1, 2, f64_from(6));
mat_set(A1, 2, 0, f64_from(3)); mat_set(A1, 2, 1, f64_from(6)); mat_set(A1, 2, 2, f64_from(9));
var U1 = mat_new(3, 3);
var V1 = mat_new(3, 3);
var S1 = alloc(24);
assert_eq(svd_golub_kahan(A1, U1, S1, V1), HSB_ERR_NONE, "svd rank-1 3x3: converges");
assert(f64_approx_eq(load64(S1), f64_from(14), LOOSE_TOL_H) == 1, "svd rank-1 3x3: s0 == 14");
  # WHY: OBSERVED. Current code: svd_golub_kahan returns -8 (HSB_ERR_NO_CONVERGENCE) and S1[0] = 0.0 (out_S is never written — the function returns at line 554, before the _lp_vec_copy at line 611). Fixed code: returns 0 (HSB_ERR_NONE) and S1[0] = 14.0 exactly (= ||u||^2 for u = (1,2,3)). Both assertions flip fail -> pass.

# _lp_check_svd is the reconstruction helper already defined at tests/hisab.tcyr:558
_lp_check_svd(A1, 3, 3, U1, S1, V1, "svd rank-1 3x3: A == U*S*Vt");
assert(f64_lt(f64_abs(load64(S1 + 8)), EPSILON_F64) == 1, "svd rank-1 3x3: s1 == 0");
assert(f64_lt(f64_abs(load64(S1 + 16)), EPSILON_F64) == 1, "svd rank-1 3x3: s2 == 0");
  # WHY: OBSERVED, with a caveat you must keep in mind. _lp_check_svd is fully discriminating: current code reports bad = 9 (all 9 entries of U*S*Vt differ from A, because U/S/Vt are the abandoned mid-iteration state and S is all zeros), fixed code reports bad = 0. The two `s == 0` assertions are NOT discriminating on their own — they pass under the current code too, for the wrong reason (S is all zeros be

# rectangular, rank-2: column 3 = 3 * column 1
var A5 = mat_new(4, 3);
mat_set(A5, 0, 0, f64_from(1)); mat_set(A5, 0, 1, f64_from(2)); mat_set(A5, 0, 2, f64_from(3));
mat_set(A5, 1, 0, f64_from(2)); mat_set(A5, 1, 1, f64_from(4)); mat_set(A5, 1, 2, f64_from(6));
mat_set(A5, 2, 0, f64_from(0)); mat_set(A5, 2, 1, f64_from(1)); mat_set(A5, 2, 2, f64_from(0));
mat_set(A5, 3, 0, f64_from(0)); mat_set(A5, 3, 1, f64_from(2)); mat_set(A5, 3, 2, f64_from(0));
var U5 = mat_new(4, 3);
var V5 = mat_new(3, 3);
var S5 = alloc(24);
assert_eq(svd_golub_kahan(A5, U5, S5, V5), HSB_ERR_NONE, "svd 4x3 rank-2: converges");
_lp_check_svd(A5, 4, 3, U5, S5, V5, "svd 4x3 rank-2: A == U*S*Vt");
  # WHY: OBSERVED. Current code: returns -8 and _lp_check_svd reports bad = 8 (8 of 12 entries wrong). Fixed code: returns 0 and bad = 0, with S5 = (s0, s1, 0.0). This covers the rectangular m > n path, where U is m x n and the left-rotation accumulation in the fix runs over m = 4 rows — a different code path from the square case.

# 5x4 rank-3 (col 3 = 2 * col 1) -- exercises an interior zero diagonal, and
# checks that the fix keeps U and Vt orthogonal rather than merely reconstructing.
var A7 = mat_new(5, 4);
mat_set(A7, 0, 0, f64_from(1)); mat_set(A7, 0, 1, f64_from(5)); mat_set(A7, 0, 2, f64_from(2));  mat_set(A7, 0, 3, f64_from(2));
mat_set(A7, 1, 0, f64_from(3)); mat_set(A7, 1, 1, f64_from(1)); mat_set(A7, 1, 2, f64_from(6));  mat_set(A7, 1, 3, f64_from(6));
mat_set(A7, 2, 0, f64_from(7)); mat_set(A7, 2, 1, f64_from(2)); mat_set(A7, 2, 2, f64_from(14)); mat_set(A7, 2, 3, f64_from(4));
mat_set(A7, 3, 0, f64_from(0)); mat_set(A7, 3, 1, f64_from(4)); mat_set(A7, 3, 2, f64_from(0));  mat_set(A7, 3, 3, f64_from(9));
mat_set(A7, 4, 0, f64_from(2)); mat_set(A7, 4, 1, f64_from(2)); mat_set(A7, 4, 2, f64_from(4));  mat_set(A7, 4, 3, f64_from(1));
var U7 = mat_new(5, 4);
var V7 = mat_new(4, 4);
var S7 = alloc(32);
assert_eq(svd_golub_kahan(A7, U7, S7, V7), HSB_ERR_NONE, "svd 5x4 rank-3: converges");
assert(f64_lt(f64_abs(load64(S7 + 24)), EPSILON_F64) == 1, "svd 5x4 rank-3: s3 == 0");
_lp_check_svd(A7, 5, 4, U7, S7, V7, "svd 5x4 rank-3: A == U*S*Vt");
  # WHY: OBSERVED. Current code: returns -8, _lp_check_svd reports bad = 18 (18 of 20 entries wrong). Fixed code: returns 0, bad = 0, S7[3] = 0.0. I additionally checked U^T U == I (4x4) and Vt Vt^T == I (4x4) to LOOSE_TOL_H on this case under the fix — both exact, confirming the added Givens chases are genuine orthogonal transforms and not just a way to force the return code to 0.

## RISKS
MEASURED REGRESSION RESULT: the full in-tree suite `tests/hisab.tcyr` scores 205 passed / 0 failed both before and after the patch (I ran it against a scratchpad copy of the module with the fix applied). The existing full-rank fixture at tests/hisab.tcyr:620-632 is unaffected. `cyrius lint` on the patched module: 0 warnings, 0 untracked deferrals. No line exceeds 100 chars (120 limit). No new hex f64 literals, so `scripts/check-constants.sh` (110/110) is untouched. `tests/hisab.bcyr` has no svd/linalg_precision coverage, so there is no benchmark to move.

What the fix could still break:

1. ILL-CONDITIONED FULL-RANK INPUT. `tol = EPSILON_F64 * max|block entry|` with EPSILON_F64 = 1e-12, which is ~4500x coarser than machine eps (2.2e-16). A genuinely full-rank block with condition number worse than 1e12 will now have its smallest singular value snapped to exactly 0 instead of being resolved to ~1e-13 * scale. Previously such a matrix would either resolve it or (more likely) hit NO_CONVERGENCE. This is the standard trade and it matches the module's existing convention — the deflation test at line 340 and the p_hi reduction at line 377 already use EPSILON_F64 — but it is a behaviour change for near-singular input, and it is the one thing to reconsider if a consumer (impetus/kiran) depends on tiny-but-nonzero singular values. Tightening to a hard-coded 2.2e-16 relative would preserve more of them at the cost of more sweeps.

2. `continue` INSIDE THE OUTER `while`. The added `cont

## CHALLENGE
fix_correct=True assertions_discriminate=True stands=True

- MINOR / undisclosed vacuity: assertion set #4's `assert(f64_lt(f64_abs(load64(S7 + 24)), EPSILON_F64) == 1, "svd 5x4 rank-3: s3 == 0")` is VACUOUS on the current code, for exactly the reason the author correctly flagged for assertion set #2 -- svd_golub_kahan bails at src/linalg_precision.cyr:554 before `_lp_vec_copy(out_S, diag, n)`, so out_S is never written and the caller's zero-filled alloc trivially satisfies it. I confirmed this empirically: on the unpatched module the 5x4 rank-3 case reports only two failures (`converges` got -8, reconstruction got 18) -- the `s3 == 0` assertion passes. The author disclosed this caveat for set #2 but not for set #4. It is harmless as written because it is bundled with the strongly discriminating return-code and _lp_check_svd assertions, but the same 'never standalone' caveat must be repeated on set #4.
- MINOR / factual overstatement in the proposed helper's doc comment: 'Bidiagonalizing a rank-r m x n matrix leaves n - r zero diagonal entries, so EVERY rank-deficient input reaches that state' is FALSE as stated. Counterexample I ran: a 6x6 rank-3 matrix (rows 2-5 duplicated), true S = (73.7897, 23.8116, 4.9083, 0, 0, 0) -- THREE zero singular values -- converges correctly on the UNPATCHED module, with A == U*S*Vt, U^T U == I and Vt Vt^T == I all clean. Bidiagonalization can leave an exactly-zero SUPERdiagonal entry, which lets the outer deflation loop (src/linalg_precision.cyr:376-379) split the block before any zero diagonal ever sits inside an active unreduced block. The defect is dominant but not universal: in my 240-matrix stress sweep the unpatched code failed 104/240, and in a 60-trial forced-dependent-column sweep it failed 60/60. Reword to 'typically' / 'any input whose bidiagon
- NOTE / inherited resolution floor, not a regression: the helper's `tol = f64_mul(EPSILON_F64, scale)` uses EPSILON_F64 = 1e-12 (src/error.cyr:26), NOT machine epsilon 2.22e-16, so the split truncates any singular value below 1e-12 * ||B_block||_inf to exactly 0. LAPACK's dbdsqr uses machine eps here. I probed the boundary directly with [[1,1],[0,eps]] (full rank, sigma_min ~ 0.7071*eps) at eps = 1e-9, 1e-11, 1e-13 and found NO observable regression: at 1e-9 and 1e-11 baseline and fixed agree exactly (s1 = 7.07106e-7 / 7.071e-12, s0*s1 = |det|), and at 1e-13 the BASELINE already returns -8 with all-zero S while the fix returns 0 with s1 = 0 -- strictly better. The 1e-12 choice is also consistent with the pre-existing deflation threshold at line 336-343. Flag it in the comment; it is not a blocker.
- ADJACENT PRE-EXISTING DEFECT the fix neither causes nor addresses -- do NOT conflate the two, and do NOT add a scaled-input assertion to the suite expecting this fix to make it pass: svd_golub_kahan fails on any matrix whose entries are around 1e-8 or smaller, regardless of rank, because _lp_bidiagonalize gates its Householder reflectors on ABSOLUTE thresholds `f64_gt(norm_l, EPSILON_F64)` / `f64_gt(vtv, EPSILON_F64)` at src/linalg_precision.cyr:151, 166, 213, 226 with EPSILON_F64 = 1e-12. Proof it is rank-independent: the FULL-RANK in-tree fixture [[1,2,3],[4,5,6],[7,8,10]] scaled by 1e-8 returns -8 with 9/9 reconstruction failures on the UNPATCHED module. This is the only case in my whole battery that still fails under the fix (1 of 199 in the scaled sweep), and it fails identically without it.

### CORRECTED FIX
No functional change required -- the fix as written compiles, lints clean, and is mathematically correct. Two comment-only corrections:

1. In the _lp_split_zero_diag header comment, replace "so EVERY rank-deficient input reaches that state" with "so any input whose bidiagonal form leaves a zero diagonal inside an unreduced block reaches that state -- which is most rank-deficient input, though a bidiagonalization that happens to produce an exactly-zero superdiagonal can split the block first and avoid it (a 6x6 rank-3 matrix with duplicated rows does)."

2. Append to the "The zero test must be RELATIVE to the block" comment: "EPSILON_F64 is 1e-12, not machine epsilon, matching the deflation threshold above; singular values below 1e-12 * ||block||_inf are therefore truncated to exactly 0 rather than resolved."

3. In the test assertions, carry the set-#2 vacuity caveat onto set #4's `s3 == 0` as well -- keep it only alongside the `converges` and `_lp_check_svd` assertions.

Everything I checked on the code itself passes:
- QUOTE FIDELITY: the quoted p_lo loop matches src/linalg_precision.cyr:384-389 verbatim, and the quoted y/z block matches lines 425-430 verbatim. No drift. The EDIT 2 insertion point (line 390, blank line between the p_lo loop's closing `}` at 389 and `# Wilkinson shift:` at 391) is correct, as is the EDIT 1 point (line 328, `# Step 2: Implicit QR iteration...`).
- MECHANISM: confirmed to the digit. Calling _lp_bidiagonalize(A1, 3, 3, ...) directly yields diag = (-3.7416, 0, 0) and superdiag = (13.4907, 0), exactly as claimed. The blocking superdiagonal is 13.49, so the originally reported site (the line-340 deflation threshold EPSILON_F64 * (|d_i| + |d_{i+1}|)) genuinely cannot fix it -- the attribution correction from line 340 to _lp_bidiag_qr:384-430 is right.
- COMPILES on cycc 6.5.6: verified by actually building and running the patched module in five separate .tcyr files. `cyrius lint` -> 0 warnings, 0 untracked deferrals. `cyrius fmt <file> --check` (the exact CI invocation from .github/workflows/ci.yml:85) -> CLEAN. Max line length in the patched file 96 chars.
- CYRIUS TRAPS: `break` and `continue` ARE supported (precedent at src/num_ext.cyr:271/282/285 and src/optimize.cyr:743) -- the legacy "we need a break here -- use a flag" hack still sitting at src/linalg_precision.cyr:352-365 is just stale style, not a language limit. No `else if`; `elif` unused but correct where I used it in tests. No negative literals (uses `(0 - 1)`). No f64_from on a hex literal. Every float comparison goes through f64_lt/f64_gt == 1, never integer < or ==. No iv_add/iv_sub/iv_mul identifiers. Zero new hex f64 constants, so scripts/check-constants.sh is unaffected. `_lp_split_zero_diag` collides with nothing in src/, lib/ or tests/. All referenced identifiers exist (mat_get/mat_set via ganita, f64_abs/sqrt/div/neg/from, EPSILON_F64 at src/error.cyr:26, load64/store64).
- CONTRACT: the header at src/linalg_precision.cyr:74-89 and the fn doc at 540-546 both say "HSB_ERR_NONE on success, HSB_ERR_NO_CONVERGENCE if QR iteration fails". Making a rank-deficient input succeed moves it into the documented success path -- no doc edit needed, but CLAUDE.md requires a CHANGELOG "Fixed" entry.
- NO REGRESSION: all four in-tree suites give byte-identical pass counts with the patched module -- hisab 205/205, foundation 307/307, modules 470/470, edge_cases 199/199 (1181 total, both ways).
- THE FIX IS REAL MATH, NOT A RETURN-CODE HACK: 240-matrix stress (m 2..8, n 2..m, 0-3 forced dependent columns) asserting A == U*S*Vt AND U^T U == I AND Vt Vt^T == I AND S descending AND S >= 0: baseline 104 NO_CONVERGENCE / 136 fully correct; fixed 0 NO_CONVERGENCE / 240 fully correct. A separate 60-trial forced-rank-deficient sweep: baseline 0/60 converged, fixed 60/60 with reconstruction and U orthonormality clean. Zero cases where the fix made a previously-working input worse.
- The author's reported numbers reproduce exactly: rank-1 3x3 -> -8 with bad = 9; 4x3 rank-2 -> -8 with bad = 8; 5x4 rank-3 -> -8 with bad = 18; all four named controls still pass.
