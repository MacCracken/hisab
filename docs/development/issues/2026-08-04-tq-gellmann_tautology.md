<!-- 2.7.0-G test-quality record. Produced by a derive agent and an adversarial reviewer,
both of which RAN the code. All six repairs: not vacuous, compile, expected values verified
independently, verdict upheld. The SURVIVES lines are mutants the NEW assertions still do
not catch -- each is a precisely-identified next round of work, not a rejection of the
repair. Apply the replacement, then mutation-prove by reverting the source under test. -->

# gellmann_tautology
status=REPAIRED verified_passes=True

## WHY CURRENT CANNOT FAIL
FILE: /home/macro/Repos/hisab/tests/modules.tcyr

--- (a) THE TAUTOLOGY -- tests/modules.tcyr:799-805 (verbatim) ---
799: # Hermitian check: lambda_1 should be hermitian (adj = self)
800: var l_gm1 = gell_mann(0);
801: var l_gm1_adj = cmat_adjoint(l_gm1);
802: var l_gm1_00 = cmat_get(l_gm1, 0, 0);
803: var l_gm1_adj_00 = cmat_get(l_gm1_adj, 0, 0);
804: assert(f64_lt(f64_abs(f64_sub(HComplex_re(l_gm1_00), HComplex_re(l_gm1_adj_00))), f64_from(1)) == 1,
805: "gell-mann hermitian");

WHY IT CANNOT FAIL -- four independent reasons, any one of which is fatal:

1. IT IS AN ALGEBRAIC IDENTITY. cmat_adjoint (src/complex.cyr:264-282) writes
   r[j][i] = cx_conj(m[i][j]). At i == j == 0 that is r[0][0] = cx_conj(m[0][0]).
   cx_conj (src/complex.cyr:69) is `cx_new(HComplex_re(a), f64_neg(HComplex_im(a)))`
   -- it copies the real part UNCHANGED and touches only the imaginary part.
   So HComplex_re(adjoint(A)[0][0]) is bit-identical to HComplex_re(A[0][0]) for
   EVERY complex matrix ever constructed. f64_sub yields exactly +0.0, f64_abs(+0.0)
   is +0.0, and f64_lt(+0.0, 1.0) == 1. Always. Unconditionally.

2. THE ONLY COMPONENT THAT CARRIES HERMITICITY IS NEVER READ. Hermiticity is
   A[i][j] == conj(A[j][i]); the imaginary part is the whole content of the
   conjugation. The assertion reads HComplex_re only -- Im is never inspected,
   so a matrix with an imaginary diagonal (maximally non-Hermitian) passes.

3. THE TOLERANCE IS 1.0. `f64_from(1)` converts the integer 1, so the guard is
   |diff| < 1.0 ABSOLUTE on a quantity whose entries are 0 or +-1. Even if (1)
   and (2) were fixed, a 99%-wrong real part would still pass.

4. ONE ENTRY OF ONE MATRIX. Only lambda_1 (idx 0) is tested, and only entry
   (0,0) -- which in lambda_1 is 0. The assertion literally evaluates
   |0.0 - 0.0| < 1.0. The entries that hold lambda_1's content, (0,1) and (1,0),
   are never touched, and lambda_2/5/7 -- the generators where conjugation
   actually matters -- are never tested at all.

--- (b) THE COMPANION TRACELESS LOOP -- tests/modules.tcyr:790-797 (verbatim) ---
790: # All 8 Gell-Mann matrices should be traceless
791: var _gm_i = 0;
792: while (_gm_i < 8) {
793:     var l_gm = gell_mann(_gm_i);
794:     var l_gm_tr = cmat_trace(l_gm);
795:     assert(f64_lt(f64_abs(HComplex_re(l_gm_tr)), f64_from(1)) == 1, "gell-mann traceless");
796:     _gm_i = _gm_i + 1;
797: }

WHY IT IS NEAR-VACUOUS: the exact value is 0, but the tolerance is f64_from(1)
= 1.0 absolute (8 executions of 

## REPLACEMENT
Two splices into /home/macro/Repos/hisab/tests/modules.tcyr.

=========================== SPLICE 1 of 2 ===========================
Insert immediately BEFORE `alloc_init();` (tests/modules.tcyr:149) -- the file
requires all helper fns to be defined before alloc_init().

# Elementwise Hermiticity test: A[i][j] == conj(A[j][i]) for EVERY (i,j),
# checked on the raw components. Deliberately does NOT go through
# cmat_adjoint: that writes r[j][i] = cx_conj(m[i][j]) and cx_conj copies the
# real part unchanged, so "Re(A[0][0]) == Re(adjoint(A)[0][0])" is an identity
# for every complex matrix ever built. Returns 1 (Hermitian) or 0.
# At i == j the Im test becomes Im + Im == 0, i.e. the diagonal must be real.
# f64_eq is IEEE equality, so +0.0 and -0.0 compare equal (cx_conj turns the
# +0.0 imaginary parts of the real generators into -0.0).
fn _gm_is_herm(m) {
    var n = cmat_rows(m);
    var i = 0;
    while (i < n) {
        var j = 0;
        while (j < n) {
            var a = cmat_get(m, i, j);
            var b = cmat_get(m, j, i);
            if (f64_eq(HComplex_re(a), HComplex_re(b)) == 0) { return 0; }
            if (f64_eq(f64_add(HComplex_im(a), HComplex_im(b)), 0) == 0) { return 0; }
            j = j + 1;
        }
        i = i + 1;
    }
    return 1;
}

=========================== SPLICE 2 of 2 ===========================
Replace tests/modules.tcyr lines 790-805 (everything after
`test_group("lie: Gell-Mann");` at line 788, up to and including the
"gell-mann hermitian" assertion) with:

# The three properties that CHARACTERISE the SU(3) Gell-Mann basis:
#   (1) lambda_a = lambda_a^H          (Hermitian, checked elementwise)
#   (2) tr lambda_a = 0                (traceless)
#   (3) tr(lambda_a lambda_b) = 2 delta_ab   (orthogonal, normalised to 2)
# Every expected value below is closed form. The ONLY floating-point residual
# anywhere in this block is tr(l8 l8): l8's entries are fl(1/fl(sqrt 3)), so the
# trace is 6*fl(x*x) = 2.0000000000000004 — exactly one ulp at 2.0
# (4.4408920985006262e-16) above the exact 2. Everything else is bit-exact:
# the other seven generators have entries in {0, +-1, +-i}, and l8's trace
# cancels exactly because fl(2/s) == 2*fl(1/s) for s = fl(sqrt 3).
var GM_TR_TOL = 0x3CD203AF9EE75616;   # 1e-15  = 2.25 ulp at 2.0
var GM_L8_TOL = 0x3CBCD2B297D889BC;   # 4e-16  = 3.6 ulp at 0.577, 1.8 ulp at 1.155
# Correctly-rounded doubles of 1/sqrt(3) and 2/sqrt(3), derived independently at
# 60 decimal digits (NOT read off the library, which rounds twice and lands
# 1 ulp high at 0x3FE279A74590331D — inside GM_L8_TOL).
var GM_L8_A  = 0x3FE279A74590331C;    #  1/sqrt(3) =  0.5773502691896257645091
var GM_L8_B2 = 0xBFF279A74590331C;    # -2/sqrt(3) = -1.1547005383792515290183

# --- (0) the checker must be able to FAIL --------------------------------
# The assertion this replaces compared Re(A[0][0]) against Re(adjoint(A)[0][0]),
# which is bit-identical for every complex matrix. These three controls prove
# _gm_is_herm actually discriminates.
var l_gm_nh1 = cmat_new(3, 3);
cmat_set(l_gm_nh1, 0, 1, cx_one());                 # 1 at (0,1), 0 at (1,0)
assert_eq(_gm_is_herm(l_gm_nh1), 0, "gm herm control: rejects non-symmetric real part");
var l_gm_nh2 = cmat_new(3, 3);
cmat_set(l_gm_nh2, 0, 0, cx_i());                   # imaginary entry on the diagonal
assert_eq(_gm_is_herm(l_gm_nh2), 0, "gm herm control: rejects imaginary diagonal");
var l_gm_nh3 = cmat_new(3, 3);                      # lambda_2 with the conjugation dropped
cmat_set(l_gm_nh3, 0, 1, cx_i());
cmat_set(l_gm_nh3, 1, 0, cx_i());
assert_eq(_gm_is_herm(l_gm_nh3), 0, "gm herm control: rejects unconjugated off-diagonal");

# --- (1) Hermiticity + (2) tracelessness, all 8 generators ---------------
var _gm_a = 0;
while (_gm_a < 8) {
    var l_gma = gell_mann(_gm_a);
    assert_eq(_gm_is_herm(l_gma), 1, "gell-mann hermitian (all 9 entries, re and im)");
    # cmat_adjoint must reproduce the matrix exactly — all 9 entries, both
    # components. Off-diagonal entries make this a real transpose+conjugate test.
    var l_gma_h = cmat_adjoint(l_gma);
    var l_gm_adj_bad = 0;
    var _gm_i = 0;
    while (_gm_i < 3) {
        var _gm_j = 0;
        while (_gm_j < 3) {
            var l_gm_u = cmat_get(l_gma, _gm_i, _gm_j);
            var l_gm_v = cmat_get(l_gma_h, _gm_i, _gm_j);
            if (f64_eq(HComplex_re(l_gm_u), HComplex_re(l_gm_v)) == 0) {
                l_gm_adj_bad = l_gm_adj_bad + 1;
            }
            if (f64_eq(HComplex_im(l_gm_u), HComplex_im(l_gm_v)) == 0) {
                l_gm_adj_bad = l_gm_adj_bad + 1;
            }
            _gm_j = _gm_j + 1;
        }
        _gm_i = _gm_i + 1;
    }
    assert_eq(l_gm_adj_bad, 0, "gell-mann adjoint == self, entrywise");
    # Traceless: exactly 0, both components (not "within 1.0").
    var l_gm_tr = cmat_trace(l_gma);
    assert_eq(f64_eq(HComplex_re(l_gm_tr), 0), 1, "gell-mann traceless (re exactly 0)");
    assert_eq(f64_eq(HComplex_im(l_gm_tr), 0), 1, "gell-mann traceless (im exactly 0)");
    _gm_a = _gm_a + 1;
}

# --- (3) orthogonality: tr(l_a l_b) = 2 delta_ab over all 64 pairs -------
var _gm_p = 0;
var l_gm_off_bad = 0;
var l_gm_diag_bad = 0;
var l_gm_im_bad = 0;
while (_gm_p < 8) {
    var l_gm_pa = gell_mann(_gm_p);
    var _gm_q = 0;
    while (_gm_q < 8) {
        var l_gm_t = cmat_trace(cmat_mul(l_gm_pa, gell_mann(_gm_q)));
        if (f64_eq(HComplex_im(l_gm_t), 0) == 0) { l_gm_im_bad = l_gm_im_bad + 1; }
        if (_gm_p == _gm_q) {
            if (f64_lt(f64_abs(f64_sub(HComplex_re(l_gm_t), F64_TWO)), GM_TR_TOL) == 0) {
                l_gm_diag_bad = l_gm_diag_bad + 1;
            }
        } else {
            if (f64_eq(HComplex_re(l_gm_t), 0) == 0) { l_gm_off_bad = l_gm_off_bad + 1; }
        }
        _gm_q = _gm_q + 1;
    }
    _gm_p = _gm_p + 1;
}
assert_eq(l_gm_off_bad, 0, "gell-mann tr(la lb) exactly 0 for a != b (56 pairs)");
assert_eq(l_gm_diag_bad, 0, "gell-mann tr(la la) = 2 within 1e-15 (8 pairs)");
assert_eq(l_gm_im_bad, 0, "gell-mann tr(la lb) exactly real (64 pairs)");

# Controls proving the Gram sweep measures normalisation, not a constant 2:
# tr((2 l1)^2) = 4 * tr(l1^2) = 8, and tr((l1+l4)^2) = 2 + 2*tr(l1 l4) + 2 = 4.
var l_gm_2l1 = cmat_scale(gell_mann(0), cx_real(F64_TWO));
var l_gm_ct1 = cmat_trace(cmat_mul(l_gm_2l1, l_gm_2l1));
assert_eq(f64_eq(HComplex_re(l_gm_ct1), f64_from(8)), 1, "gm gram control: tr((2 l1)^2) = 8");
var l_gm_s14 = cmat_add(gell_mann(0), gell_mann(3));
var l_gm_ct2 = cmat_trace(cmat_mul(l_gm_s14, l_gm_s14));
assert_eq(f64_eq(HComplex_re(l_gm_ct2), f64_from(4)), 1, "gm gram control: tr((l1+l4)^2) = 4");

# --- convention pin ------------------------------------------------------
# Hermiticity + tracelessness + tr(la lb) = 2 delta_ab are invariant under any
# unitary change of basis, so they do not by themselves pin the Gell-Mann
# convention. These do:
#   lambda_1 = [[0,1,0],[1,0,0],[0,0,0]]
#   lambda_3 = diag(1, -1, 0)
#   lambda_8 = diag(1, 1, -2)/sqrt(3)
#   [lambda_1, lambda_2] = 2i lambda_3   (structure constant f_123 = +1)
var l_gm_l1 = gell_mann(0);
assert_eq(f64_eq(HComplex_re(cmat_get(l_gm_l1, 0, 1)), F64_ONE), 1, "l1(0,1) = 1");
assert_eq(f64_eq(HComplex_re(cmat_get(l_gm_l1, 0, 0)), 0), 1, "l1(0,0) = 0");
var l_gm_l3 = gell_mann(2);
assert_eq(f64_eq(HComplex_re(cmat_get(l_gm_l3, 0, 0)), F64_ONE), 1, "l3(0,0) = +1");
assert_eq(f64_eq(HComplex_re(cmat_get(l_gm_l3, 1, 1)), f64_neg(F64_ONE)), 1, "l3(1,1) = -1");
assert_eq(f64_eq(HComplex_re(cmat_get(l_gm_l3, 2, 2)), 0), 1, "l3(2,2) = 0");
var l_gm_l8 = gell_mann(7);
var l_gm_e00 = HComplex_re(cmat_get(l_gm_l8, 0, 0));
var l_gm_e11 = HComplex_re(cmat_get(l_gm_l8, 1, 1));
var l_gm_e22 = HComplex_re(cmat_get(l_gm_l8, 2, 2));
assert_eq(f64_lt(f64_abs(f64_sub(l_gm_e00, GM_L8_A)), GM_L8_TOL), 1, "l8(0,0) = 1/sqrt(3)");
assert_eq(f64_eq(l_gm_e11, l_gm_e00), 1, "l8(1,1) == l8(0,0)");
assert_eq(f64_lt(f64_abs(f64_sub(l_gm_e22, GM_L8_B2)), GM_L8_TOL), 1, "l8(2,2) = -2/sqrt(3)");
assert_eq(f64_eq(HComplex_re(cmat_get(l_gm_l8, 0, 1)), 0), 1, "l8(0,1) = 0");
var l_gm_c12 = cmat_commutator(gell_mann(0), gell_mann(1));
var l_gm_k00 = cmat_get(l_gm_c12, 0, 0);
var l_gm_k11 = cmat_get(l_gm_c12, 1, 1);
assert_eq(f64_eq(HComplex_re(l_gm_k00), 0), 1, "[l1,l2](0,0) is pure imaginary");
assert_eq(f64_eq(HComplex_im(l_gm_k00), F64_TWO), 1, "[l1,l2] = 2i l3: (0,0) im = +2");
assert_eq(f64_eq(HComplex_im(l_gm_k11), f64_neg(F64_TWO)), 1, "[l1,l2] = 2i l3: (1,1) im = -2");
assert_eq(f64_eq(HComplex_im(cmat_get(l_gm_c12, 2, 2)), 0), 1, "[l1,l2] = 2i l3: (2,2) im = 0");
assert_eq(f64_eq(HComplex_im(cmat_get(l_gm_c12, 0, 1)), 0), 1, "[l1,l2] = 2i l3: (0,1) im = 0");

## NOTES
RUN EVIDENCE (verified_passes = true; nothing in /home/macro/Repos/hisab was modified -- `git diff --stat -- src/lie.cyr tests/modules.tcyr` is empty).

  Baseline, unmodified suite:
    cyrius test /home/macro/Repos/hisab/tests/modules.tcyr
      -> 512 passed, 0 failed (512 total)

  Spliced copy (the exact code above, in-place in a full copy of modules.tcyr):
    cyrius test .../scratchpad/gm_modules.tcyr
      -> 557 passed, 0 failed (557 total), exit 0
    Net +45: 9 old assertions removed (8 traceless + 1 tautology), 54 added.
    awk length>120: no violations.  cyrius lint: 0 warnings, 0 deferrals.

  Isolated copy (same block, minimal include set): .../scratchpad/gm_min.tcyr
      -> 57 passed, 0 failed.

TIGHTNESS -- MUTATION TESTED (5 mutants of a scratch copy of src/lie.cyr, included
via CYRIUS_ALLOW_ABSOLUTE_INCLUDES=1; src/ never touched). Old block vs new block:

  A  lambda_2 conjugation dropped ([[0,i],[i,0]])       OLD: pass   NEW: 7 failures
       (herm elementwise, adjoint entrywise, tr(l2 l2) = -2, Im(tr) != 0, commutator)
  B  lambda_8 normalised by 1/3 instead of 1/sqrt(3)    OLD: pass   NEW: 3 failures
       (tr(l8 l8) = 0.667, l8(0,0) pin, l8(2,2) pin)
  C

## CHALLENGE  vacuous=False compiles=True values_ok=True stands=True
- SURVIVES: M1_l5_signflip -- src/lie.cyr gell_mann idx==4: swap i_neg/i_pos so lambda_5 = [[0,0,+i],[0,0,0],[0,0,-i... i.e. -lambda_5]]. Still Hermitian, traceless, tr(l5 l5)=2, orthogonal to all others. Proposed block: 54 passed, 0 failed. Breaks f_458, f_156, f_345 -- su3_structure_constant would disagree with the matrices.
- SURVIVES: M3_l7_signflip -- src/lie.cyr gell_mann idx==6: swap i_neg/i_pos so lambda_7 -> -lambda_7. Proposed block: 54 passed, 0 failed. Same class: sign of any of lambda_4..lambda_7 is unconstrained.
- SURVIVES: M2_l4_l6_swapped -- src/lie.cyr gell_mann: idx==3 returns lambda_6's entries (1,2)/(2,1) and idx==5 returns lambda_4's entries (0,2)/(2,0). A copy-paste error in the dispatch chain. The 8-element set is unchanged so the whole Gram sweep, Hermiticity sweep and trace sweep are blind; the gram control tr((l1+gell_mann(3))^2)=4 also passes because tr(l1 l6)=0 just like tr(l1 l4)=0. Proposed block: 54 passed, 0 failed.
- SURVIVES: M12_l4l6_rotated_45deg -- src/lie.cyr gell_mann: lambda_4 := (lambda_4+lambda_6)/sqrt(2), lambda_6 := (lambda_4-lambda_6)/sqrt(2). A real orthogonal rotation inside the {l4,l5,l6,l7} subspace. Hermitian, traceless, tr(la lb)=2 delta_ab all still hold exactly, and the convention pins never touch l4..l7. Proposed block: 54 passed, 0 failed. This is the general form of the gap: ANY O(4) mixing of lambda_4..lambda_7 is invisible.

### CORRECTED
APPEND the following to the end of SPLICE 2 (immediately after the last
`assert_eq(... "[l1,l2] = 2i l3: (0,1) im = 0");` line, and before the original
blank line + `# ----------------------------------------------------------------`
separator that precedes `test_group("lie: Lorentz");`). Do NOT end the pasted
block with a blank line -- original line 806 is already blank and `cyrius lint`
treats consecutive blank lines as a warning, which CI treats as an error.

Everything else in SPLICE 1 and SPLICE 2 is correct as proposed and should be
applied unchanged. Verified: with this appended, the block scores 64 passed /
0 failed against unmodified src, and catches all 14 mutants I built (the 4 that
previously survived plus 2 new structure-constant-table mutants). Longest added
line is 98 chars. `cyrius lint` clean.

# --- (4) convention pin for lambda_4..lambda_7 ---------------------------
# Properties (1)-(3) are invariant under ANY real orthogonal mixing of
# lambda_4..lambda_7 (the 4-dim subspace orthogonal to l1,l2,l3,l8), and the
# pins above touch only l1, l2, l3, l8 -- so a sign flip on l5 or l7, a swap of
# l4 and l6, or a 45-degree rotation of (l4,l6) passes everything above.
# Each pin below plus Hermiticity plus tr(la la) = 2 determines the generator
# uniquely: two off-diagonal entries of modulus 1 already exhaust the norm.
assert_eq(f64_eq(HComplex_re(cmat_get(gell_mann(3), 0, 2)), F64_ONE), 1, "l4(0,2) = +1");
assert_eq(f64_eq(HComplex_im(cmat_get(gell_mann(3), 0, 2)), 0), 1, "l4(0,2) im = 0");
assert_eq(f64_eq(HComplex_re(cmat_get(gell_mann(3), 1, 2)), 0), 1, "l4(1,2) = 0");
assert_eq(f64_eq(HComplex_im(cmat_get(gell_mann(4), 0, 2)), f64_neg(F64_ONE)), 1, "l5(0,2) = -i");
assert_eq(f64_eq(HComplex_re(cmat_get(gell_mann(4), 0, 2)), 0), 1, "l5(0,2) re = 0");
assert_eq(f64_eq(HComplex_re(cmat_get(gell_mann(5), 1, 2)), F64_ONE), 1, "l6(1,2) = +1");
assert_eq(f64_eq(HComplex_re(cmat_get(gell_mann(5), 0, 2)), 0), 1, "l6(0,2) = 0");
assert_eq(f64_eq(HComplex_im(cmat_get(gell_mann(6), 1, 2)), f64_neg(F64_ONE)), 1, "l7(1,2) = -i");
assert_eq(f64_eq(HComplex_re(cmat_get(gell_mann(6), 1, 2)), 0), 1, "l7(1,2) re = 0");

# --- (5) algebra closure: [l_a, l_b] = 2i sum_c f_abc l_c ----------------
# The identity that actually ties the matrices to the algebra, over all 64
# pairs and all 9 entries. Also the first coverage su3_structure_constant
# (src/lie.cyr:310) has ever had -- grep -rn "su3_" tests/ returns nothing
# today, so the table and the matrices have never been checked against each
# other. Exact in double: 2 * (sqrt3/2) * (1/sqrt3) == 1.0 and
# 2 * (sqrt3/2) * (-2/sqrt3) == -2.0 bit for bit, so the tolerance is slack.
var l_gm_sc_bad = 0;
var _gm_u = 0;
while (_gm_u < 8) {
    var _gm_v = 0;
    while (_gm_v < 8) {
        var l_gm_lhs = cmat_commutator(gell_mann(_gm_u), gell_mann(_gm_v));
        var l_gm_rhs = cmat_new(3, 3);
        var _gm_c = 0;
        while (_gm_c < 8) {
            var l_gm_f = su3_structure_constant(_gm_u + 1, _gm_v + 1, _gm_c + 1);
            if (f64_eq(l_gm_f, 0) == 0) {
                var l_gm_w = cx_new(0, f64_mul(F64_TWO, l_gm_f));
                l_gm_rhs = cmat_add(l_gm_rhs, cmat_scale(gell_mann(_gm_c), l_gm_w));
            }
            _gm_c = _gm_c + 1;
        }
        var _gm_r = 0;
        while (_gm_r < 3) {
            var _gm_s = 0;
            while (_gm_s < 3) {
                var l_gm_lv = cmat_get(l_gm_lhs, _gm_r, _gm_s);
                var l_gm_rv = cmat_get(l_gm_rhs, _gm_r, _gm_s);
                var l_gm_dr = f64_abs(f64_sub(HComplex_re(l_gm_lv), HComplex_re(l_gm_rv)));
                var l_gm_di = f64_abs(f64_sub(HComplex_im(l_gm_lv), HComplex_im(l_gm_rv)));
                if (f64_lt(l_gm_dr, GM_TR_TOL) == 0) { l_gm_sc_bad = l_gm_sc_bad + 1; }
                if (f64_lt(l_gm_di, GM_TR_TOL) == 0) { l_gm_sc_bad = l_gm_sc_bad + 1; }
                _gm_s = _gm_s + 1;
            }
            _gm_r = _gm_r + 1;
        }
        _gm_v = _gm_v + 1;
    }
    _gm_u = _gm_u + 1;
}
assert_eq(l_gm_sc_bad, 0, "gell-mann [la,lb] = 2i f_abc lc (64 pairs, 9 entries, re+im)");