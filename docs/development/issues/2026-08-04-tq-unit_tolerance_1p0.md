<!-- 2.7.0-G test-quality record. Produced by a derive agent and an adversarial reviewer,
both of which RAN the code. All six repairs: not vacuous, compile, expected values verified
independently, verdict upheld. The SURVIVES lines are mutants the NEW assertions still do
not catch -- each is a precisely-identified next round of work, not a rejection of the
repair. Apply the replacement, then mutation-prove by reverting the source under test. -->

# unit_tolerance_1p0
status=REPAIRED verified_passes=True

## WHY CURRENT CANNOT FAIL
CENSUS: 15 static `f64_from(1)`-as-tolerance assert sites (6 in tests/hisab.tcyr, 9 in tests/modules.tcyr). One of them is inside an 8-iteration loop, so 22 assertions execute at runtime. Plus 1 dead tolerance variable. All quoted verbatim.

=== tests/hisab.tcyr ===

[H1] tests/hisab.tcyr:93
  assert(f64_lt(f64_abs(HVec3_x(rotated)), f64_from(1)) == 1, "rot90.x near 0");
  WHY IT CANNOT FAIL: rotated = hquat_rotate_vec3(hquat_from_rotation_z(pi/2), (1,0,0)). x is exactly cos(t) for a rotation by t about Z, so |x| <= 1 by construction for EVERY angle. |x| < 1 holds for every t except t = 0 (mod 2pi). The assertion is satisfied by every nonzero rotation angle in the library and certifies nothing about 90 degrees.

[H2] tests/hisab.tcyr:94
  assert(f64_lt(f64_abs(f64_sub(HVec3_y(rotated), F64_ONE)), f64_from(1)) == 1, "rot90.y near 1");
  WHY: y = sin(t), so |y - 1| < 1 <=> sin(t) > 0 <=> t in (0, pi). Passes for every rotation between 0 and 180 degrees.

[H3] tests/hisab.tcyr:121
  assert(f64_lt(f64_abs(f64_sub(m4_get(prod, 0, 0), F64_ONE)), f64_from(1)) == 1, "inv diag");
  WHY: prod = srt * m4_inverse(srt) where srt is a PURE TRANSLATION (scale=1, quat=identity, t=(2,3,4)). prod[0][0] = 1 exactly. Tolerance 1.0 admits [0, 2]. It also passes if m4_inverse returns the identity matrix unchanged (then prod = srt and prod[0][0] is still 1) - so it is not an inverse test at all.

[H4] tests/hisab.tcyr:150
  assert(f64_lt(f64_abs(f64_sub(back, half)), f64_from(1)) == 1, "srgb roundtrip");
  WHY: back = linear_to_srgb(srgb_to_linear(0.5)). Tolerance 1.0 admits (-0.5, 1.5) - the entire output range of linear_to_srgb for any in-gamut input. Cannot fail for any transfer curve whatsoever.

[H5] tests/hisab.tcyr:744
  assert(f64_lt(cx_abs(euler), f64_from(1)) == 1, "euler identity");
  WHY: euler = e^(i*pi) + 1. |e^(i*t) + 1| = 2|cos(t/2)|, which is < 1 for every t in (2pi/3, 4pi/3). The assertion passes for any argument within +/-60 degrees of pi. It is blind to a 60-degree phase error in cx_exp.

[H6] tests/hisab.tcyr:776
  assert(f64_lt(f64_abs(load64(u_id)), f64_from(1)) == 1, "u1 inv");
  WHY: u_id = u1_compose(u1_new(pi/4), u1_inv(u1_new(pi/4))) - the residual phase, exactly +0.0. Tolerance 1.0 accepts any residual phase up to +/-57 degrees.

=== tests/modules.tcyr ===

[M1] tests/modules.tcyr:532
  assert(f64_lt(f64_abs(load64(n_sin_out)), f64_from(1)) == 1, "newton sin root near 0");
  WHY: num_newton for sin from x0 = 0.5. |x| < 1 is satisfied by the STARTING 

## REPLACEMENT
Paste-ready. Two new named tolerances, then per-site replacements. Verified: hisab 226 -> 231 passed / 0 failed; modules 512 -> 524 passed / 0 failed, against the unmodified library.

================================================================
FILE: tests/hisab.tcyr
================================================================

--- (1) after `var LOOSE_TOL_H = 0x3E45798EE2308C3A;` (line 30), INSERT:

# ~1e-15 -- a few ulp of 1.0. For quantities whose closed form is exactly 0 or 1
# (unit-vector rotations, M*M^-1 diagonals, group-inverse round-trips, Euler's
# identity). Every one of them lands within 1 ulp on 6.5.6, so this is ~4.5x the
# measured floor. It replaces a literal f64_from(1): a tolerance of 1.0 around an
# expected 1.0 admits everything in [0, 2] and can never fail.
var ULP_TOL_H = 0x3CD203AF9EE75616;  # 1e-15

--- (2) lines 93-94, REPLACE:
assert(f64_lt(f64_abs(HVec3_x(rotated)), f64_from(1)) == 1, "rot90.x near 0");
assert(f64_lt(f64_abs(f64_sub(HVec3_y(rotated), F64_ONE)), f64_from(1)) == 1, "rot90.y near 1");
    WITH:
# Closed form: exactly (0, 1, 0). Measured on 6.5.6: x = 2.220446e-16 (1 ulp of
# 1.0), y - 1 = 0, z = 0. The old bound of 1.0 was met by EVERY nonzero rotation
# angle -- |x| = |cos t| < 1 and |y - 1| = |1 - sin t| < 1 for all t in (0, pi).
assert(f64_lt(f64_abs(HVec3_x(rotated)), ULP_TOL_H) == 1, "rot90.x == 0 (1e-15)");
assert(f64_lt(f64_abs(f64_sub(HVec3_y(rotated), F64_ONE)), ULP_TOL_H) == 1, "rot90.y == 1 (1e-15)");
assert(f64_lt(f64_abs(HVec3_z(rotated)), ULP_TOL_H) == 1, "rot90.z == 0 (1e-15)");
# |x| <= 1 holds for every rotation of a unit vector, so the bound above only
# bites next to a case that pins the ANGLE. 45 deg about Z sends (1,0,0) to
# (sqrt(2)/2, sqrt(2)/2, 0) = (0.7071067811865476, 0.7071067811865476, 0).
var _sqrt2_2 = 0x3FE6A09E667F3BCD;  # 0.7071067811865476
var rot45 = hquat_rotate_vec3(hquat_from_rotation_z(F64_PI_4), hvec3_unit_x());
assert(f64_lt(f64_abs(f64_sub(HVec3_x(rot45), _sqrt2_2)), ULP_TOL_H) == 1, "rot45.x == sqrt(2)/2");
assert(f64_lt(f64_abs(f64_sub(HVec3_y(rot45), _sqrt2_2)), ULP_TOL_H) == 1, "rot45.y == sqrt(2)/2");

--- (3) line 121, REPLACE:
assert(f64_lt(f64_abs(f64_sub(m4_get(prod, 0, 0), F64_ONE)), f64_from(1)) == 1, "inv diag");
    WITH:
# srt is a pure translation, so M * M^-1 is I exactly -- measured residual 0 on
# every entry. The old bound of 1.0 accepted any [0][0] in [0, 2]; and a single
# diagonal entry is not an inverse test anyway (m4_inverse returning I unchanged
# leaves [0][0] == 1). Check all 16 entries.
assert(f64_lt(f64_abs(f64_sub(m4_get(prod, 0, 0), F64_ONE)), ULP_TOL_H) == 1, "inv diag [0][0] == 1");
var _mi_bad = 0;
var _mi_r = 0;
while (_mi_r < 4) {
    var _mi_c = 0;
    while (_mi_c < 4) {
        var _mi_want = 0;
        if (_mi_r == _mi_c) { _mi_want = F64_ONE; }
        if (f64_lt(f64_abs(f64_sub(m4_get(prod, _mi_c, _mi_r), _mi_want)), ULP_TOL_H) == 0) {
            _mi_bad = _mi_bad + 1;
        }
        _mi_c = _mi_c + 1;
    }
    _mi_r = _mi_r + 1;
}
assert_eq(_mi_bad, 0, "M * M^-1 == I in all 16 entries (1e-15)");

--- (4) line 150, REPLACE:
assert(f64_lt(f64_abs(f64_sub(back, half)), f64_from(1)) == 1, "srgb roundtrip");
    WITH:
# Measured round-trip residual on 6.5.6: 1.665e-16. The old bound of 1.0 was met
# by anything in (-0.5, 1.5) -- i.e. by every value linear_to_srgb can return.
assert(f64_lt(f64_abs(f64_sub(back, half)), ULP_TOL_H) == 1, "srgb roundtrip == 0.5 (1e-15)");

--- (5) after the "srgb: linear segment below breakpoint" assert (ends line 163), INSERT:
# Only the forward leg was pinned absolutely; a compensating pair of wrong
# exponents still round-trips green. Pin the inverse leg too:
# linear_to_srgb(0.2) = 1.055 * 0.2^(1/2.4) - 0.055 = 0.48452920448170694
var _srgb_lin_02 = 0x3FC999999999999A;   # 0.2
var _srgb_enc_02 = 0x3FDF0286C7CD2C49;   # 0.48452920448170694
assert(f64_lt(f64_abs(f64_sub(linear_to_srgb(_srgb_lin_02), _srgb_enc_02)), ULP_TOL_H) == 1,
"linear_to_srgb(0.2) == 0.4845292044817069 (absolute, not round-trip)");

--- (6) line 744, REPLACE:
assert(f64_lt(cx_abs(euler), f64_from(1)) == 1, "euler identity");
    WITH:
# e^(i*pi_d) = cos(pi_d) + i sin(pi_d) with pi_d the double nearest pi:
# cos(pi_d) rounds to exactly -1.0 and sin(pi_d) = 1.2246467991473532e-16, so the
# closed-form modulus is 1.2246e-16 (measured: identical). The old bound of 1.0
# passed for any argument within about +-60 deg of pi.
assert(f64_lt(cx_abs(euler), ULP_TOL_H) == 1, "euler e^(i*pi)+1 == 0 (1e-15)");

--- (7) line 776, REPLACE:
assert(f64_lt(f64_abs(load64(u_id)), f64_from(1)) == 1, "u1 inv");
    WITH:
# u1_compose(a, u1_inv(a)) is literally f64_add(theta, f64_neg(theta)); IEEE-754
# round-to-nearest makes x + (-x) exactly +0.0 for every finite x. So this is an
# exact assertion, not an approximate one. Measured bit pattern: 0x0.
# (The old bound of 1.0 passed for any residual phase up to +-57 deg.)
assert_eq(load64(u_id), 0, "u1 inv phase is exactly +0.0");

================================================================
FILE: tests/modules.tcyr
================================================================

--- (8) after `var LOOSE_TOL_M = 0x3E45798EE2308C3A;` (line 40), INSERT:

# ~1e-15 -- a few ulp of 1.0, for quantities whose closed form is exactly 0 or 1
# (group-inverse round-trips, traceless generators, Euler's identity). Measured
# floor for all of them on 6.5.6 is <= 2.3e-16. Replaces a literal f64_from(1),
# which is a tolerance of 1.0 and admits everything in [0, 2].
var ULP_TOL_M = 0x3CD203AF9EE75616;  # 1e-15
# ~1e-13 -- for the Minkowski interval of (5,3,0,0), whose magnitude is 16 and
# whose measured boost drift is 7.105e-15 (4 ulp of 16).
var IVL_TOL_M = 0x3D3C25C268497682;  # 1e-13

--- (9) lines 531-532, REPLACE:
num_newton(&_tn_sin_f, &_tn_sin_df, F64_HALF, EPSILON_F64, 100, n_sin_out);
assert(f64_lt(f64_abs(load64(n_sin_out)), f64_from(1)) == 1, "newton sin root near 0");
    WITH:
var n_sin_rc = num_newton(&_tn_sin_f, &_tn_sin_df, F64_HALF, EPSILON_F64, 100, n_sin_out);
assert_eq(n_sin_rc, 0, "newton sin converged");
# num_newton returns the first iterate with |sin(x)| < tol = EPSILON_F64 = 1e-12.
# Near 0, |sin x| >= |x| - |x|^3/6, so the root is bounded by 1.0000000000002e-12
# -- and that bound also excludes the other roots (+-pi, ...). Measured: 1.21e-14.
# The old bound of 1.0 was satisfied by every point in (-1, 1), including x0=0.5.
assert(f64_lt(f64_abs(load64(n_sin_out)), EPSILON_F64) == 1, "newton sin root == 0 (1e-12)");

--- (10) line 692, REPLACE:
assert(f64_lt(cx_abs(c_euler), f64_from(1)) == 1, "euler e^(ipi)+1 ~ 0");
    WITH:
# cos(pi_d) rounds to exactly -1.0 and sin(pi_d) = 1.2246467991473532e-16, so the
# closed-form modulus is 1.2246e-16 (measured: identical). The old bound of 1.0
# passed for any argument within about +-60 deg of pi.
assert(f64_lt(cx_abs(c_euler), ULP_TOL_M) == 1, "euler e^(ipi)+1 == 0 (1e-15)");

--- (11) line 757, REPLACE:
assert(f64_lt(f64_abs(load64(l_u_id)), f64_from(1)) == 1, "u1 inv ~ 0");
    WITH:
# f64_add(theta, f64_neg(theta)) is exactly +0.0 in IEEE-754 round-to-nearest for
# every finite theta, so this is exact. Measured bit pattern: 0x0.
assert_eq(load64(l_u_id), 0, "u1 inv phase is exactly +0.0");

--- (12) lines 778-779, REPLACE:
# w component should be ~1
assert(f64_lt(f64_abs(f64_sub(_su2_w(l_su2_p), F64_ONE)), f64_from(1)) == 1, "su2 compose inv ~ id");
    WITH:
# Half-angle convention: su2_from_axis_angle(z_hat, pi/4) = cos(pi/8) + sin(pi/8) k.
# Pin those first -- w = 1 after compose-with-inverse and dw = 0 after a to_quat
# round-trip are both TRUE under a full-angle build, so neither check below can
# see the half-vs-full-angle split that shipped through 2.6.12.
#   cos(pi/8) = 0.9238795325112867   (a full-angle build would put 0.7071067811865476 here)
#   sin(pi/8) = 0.3826834323650898
var _cos_pi8 = 0x3FED906BCF328D46;  # 0.9238795325112867
var _sin_pi8 = 0x3FD87DE2A6AEA963;  # 0.3826834323650898
assert(f64_lt(f64_abs(f64_sub(_su2_w(l_su2_r), _cos_pi8)), ULP_TOL_M) == 1, "su2 axis-angle w == cos(pi/8)");
assert(f64_lt(f64_abs(f64_sub(_su2_z(l_su2_r), _sin_pi8)), ULP_TOL_M) == 1, "su2 axis-angle z == sin(pi/8)");
# w of q*q^-1 is w^2+x^2+y^2+z^2 = 1 exactly for a unit quaternion (measured
# residual 0). The old bound of 1.0 merely asserted w > 0.
assert(f64_lt(f64_abs(f64_sub(_su2_w(l_su2_p), F64_ONE)), ULP_TOL_M) == 1, "su2 compose inv w == 1 (1e-15)");

--- (13) lines 784-785, REPLACE:
assert(f64_lt(f64_abs(f64_sub(_su2_w(l_su2_back), _su2_w(l_su2_r))), f64_from(1)) == 1,
"su2 to_quat roundtrip");
    WITH:
# Both w's live in [-1, 1], so |dw| < 1.0 was near-unfalsifiable. The round trip
# renormalizes by a norm of exactly 1.0 here: measured dw = 0.
assert(f64_lt(f64_abs(f64_sub(_su2_w(l_su2_back), _su2_w(l_su2_r))), ULP_TOL_M) == 1,
"su2 to_quat roundtrip dw == 0 (1e-15)");

--- (14) line 795 (inside the loop), REPLACE:
    assert(f64_lt(f64_abs(HComplex_re(l_gm_tr)), f64_from(1)) == 1, "gell-mann traceless");
    WITH:
    assert(f64_lt(f64_abs(HComplex_re(l_gm_tr)), ULP_TOL_M) == 1, "gell-mann traceless (1e-15)");

--- (15) lines 799-805, REPLACE:
# Hermitian check: lambda_1 should be hermitian (adj = self)
var l_gm1 = gell_mann(0);
var l_gm1_adj = cmat_adjoint(l_gm1);
var l_gm1_00 = cmat_get(l_gm1, 0, 0);
var l_gm1_adj_00 = cmat_get(l_gm1_adj, 0, 0);
assert(f64_lt(f64_abs(f64_sub(HComplex_re(l_gm1_00), HComplex_re(l_gm1_adj_00))), f64_from(1)) == 1,
"gell-mann hermitian");
    WITH:
# lambda_1..lambda_7 are traceless entry-by-entry (all diagonals are literal 0),
# so only lambda_8 exercises the sum -- and tracelessness alone does not pin its
# normalization: diag(a, a, -2a) is traceless for ANY a. Pin the spec values.
# 1/sqrt(3) = 0.5773502691896258, -2/sqrt(3) = -1.1547005383792517; 2*(1/sqrt(3))
# is the same double as 2/sqrt(3), so the trace is exactly +0.0 (measured).
var _inv_sqrt3 = 0x3FE279A74590331D;   # 0.5773502691896258
var _neg2_sqrt3 = 0xBFF279A74590331D;  # -1.1547005383792517
var l_gm8 = gell_mann(7);
assert(f64_lt(f64_abs(f64_sub(HComplex_re(cmat_get(l_gm8, 0, 0)), _inv_sqrt3)), ULP_TOL_M) == 1,
"gell-mann 8 [0][0] == 1/sqrt(3)");
assert(f64_lt(f64_abs(f64_sub(HComplex_re(cmat_get(l_gm8, 2, 2)), _neg2_sqrt3)), ULP_TOL_M) == 1,
"gell-mann 8 [2][2] == -2/sqrt(3)");

# Hermitian check. Comparing Re(lambda_1[0][0]) with Re(lambda_1^dagger[0][0]) is
# a tautology -- Re(conj(z)) == Re(z) for every z, and both sides are 0 here, so
# no tolerance can make it fail. Assert the actual condition instead:
# lambda^dagger == lambda in all 9 entries, real AND imaginary, for all 8
# generators. This is what catches a sign-flipped i entry in lambda_2/5/7.
var _gh_bad = 0;
var _gh_k = 0;
while (_gh_k < 8) {
    var l_gh = gell_mann(_gh_k);
    var l_gh_adj = cmat_adjoint(l_gh);
    var _gh_r = 0;
    while (_gh_r < 3) {
        var _gh_c = 0;
        while (_gh_c < 3) {
            var _gh_a = cmat_get(l_gh, _gh_r, _gh_c);
            var _gh_b = cmat_get(l_gh_adj, _gh_r, _gh_c);
            if (f64_lt(f64_abs(f64_sub(HComplex_re(_gh_a), HComplex_re(_gh_b))), ULP_TOL_M) == 0) {
                _gh_bad = _gh_bad + 1;
            }
            if (f64_lt(f64_abs(f64_sub(HComplex_im(_gh_a), HComplex_im(_gh_b))), ULP_TOL_M) == 0) {
                _gh_bad = _gh_bad + 1;
            }
            _gh_c = _gh_c + 1;
        }
        _gh_r = _gh_r + 1;
    }
    _gh_k = _gh_k + 1;
}
assert_eq(_gh_bad, 0, "all 8 gell-mann matrices are hermitian (re+im, 1e-15)");
# lambda_2 = [[0, -i], [i, 0]]: pin the off-diagonal so hermiticity is not
# satisfied vacuously by an all-real matrix.
assert_eq(f64_to(HComplex_im(cmat_get(gell_mann(1), 0, 1))), (0 - 1), "lambda_2[0][1] == -i");
assert_eq(f64_to(HComplex_im(cmat_get(gell_mann(1), 1, 0))), 1, "lambda_2[1][0] == +i");

--- (16) after `assert_eq(lorentz_is_valid(l_boost), 1, "boost is valid");` (line 820), INSERT:
# lorentz_boost_x takes RAPIDITY, not velocity: Lambda^0_0 = cosh(0.5) and
# Lambda^1_0 = sinh(0.5). If it ever silently reverts to reading the argument as
# beta, those become gamma = 1.1547005383792517 and gamma*beta = 0.5773502691896258
# -- 0.027 and 0.056 away, both far outside 1e-15. The compose-inverse and
# interval-preservation checks below hold under EITHER reading, so they cannot
# see that regression; these two can.
var _cosh_half = 0x3FF20AC1862AE8D0;  # cosh(0.5) = 1.1276259652063807
var _sinh_half = 0x3FE0ACD00FE63B97;  # sinh(0.5) = 0.5210953054937474
assert(f64_lt(f64_abs(f64_sub(m4_get(l_boost, 0, 0), _cosh_half)), ULP_TOL_M) == 1,
"boost_x(0.5)[0][0] == cosh(0.5)");
assert(f64_lt(f64_abs(f64_sub(m4_get(l_boost, 1, 0), _sinh_half)), ULP_TOL_M) == 1,
"boost_x(0.5)[1][0] == sinh(0.5)");

--- (17) lines 826-827, REPLACE:
assert(f64_lt(f64_abs(f64_sub(m4_get(l_comp, 0, 0), F64_ONE)), f64_from(1)) == 1,
"lorentz compose inv diag");
    WITH:
# cosh^2 - sinh^2 = 1 in closed form; measured residual 2.22e-16 (1 ulp). The old
# bound of 1.0 accepted any [0][0] in [0, 2]. Check an off-diagonal too, or a
# lorentz_inv that returned the identity would leave [0][0] = cosh(0.5) = 1.128
# and still pass.
assert(f64_lt(f64_abs(f64_sub(m4_get(l_comp, 0, 0), F64_ONE)), ULP_TOL_M) == 1,
"lorentz compose inv [0][0] == 1 (1e-15)");
assert(f64_lt(f64_abs(f64_sub(m4_get(l_comp, 1, 1), F64_ONE)), ULP_TOL_M) == 1,
"lorentz compose inv [1][1] == 1 (1e-15)");
assert(f64_lt(f64_abs(m4_get(l_comp, 0, 1)), ULP_TOL_M) == 1, "lorentz compose inv [0][1] == 0");

--- (18) line 837, REPLACE:
assert(f64_lt(l_intv_diff, f64_from(1)) == 1, "lorentz preserves interval");
    WITH:
# Pin the interval itself: 5^2 - 3^2 = 16 exactly. A (-,+,+,+) signature would
# give -16 and the before/after DIFFERENCE would still be 0, so the preservation
# check alone is blind to a global sign flip.
assert_eq(f64_to(l_intv_before), 16, "minkowski interval of (5,3,0,0) == +16");
# Boosted by rapidity 0.5: t' = 5cosh+3sinh = 7.201415742513145,
# x' = 5sinh+3cosh = 5.988354423087879, t'^2 - x'^2 = 15.999999999999993.
# Measured drift 7.105e-15 = 4 ulp of 16. The old bound of 1.0 was a 6.25%
# relative tolerance on a quantity of magnitude 16.
assert(f64_lt(l_intv_diff, IVL_TOL_M) == 1, "lorentz preserves interval (1e-13)");

================================================================
FILE: tests/foundation.tcyr
================================================================

--- (19) line 15, DELETE:
var TOL = f64_from(1);  # 1e-10 is too tight for bit patterns; we use manual checks
    (dead: never referenced. Leaving a 1.0 named TOL in scope is the trap that
     produced this whole finding class. LOOSE_TOL/TIGHT_TOL on lines 17-18 are
     the ones actually used.)

## NOTES
VERIFICATION (all run from /home/macro/Repos/hisab against the unmodified library):
  cyrius test .../scratchpad/hisab_fix.tcyr    -> 231 passed, 0 failed  (HEAD baseline: 226 passed -> +5)
  cyrius test .../scratchpad/modules_fix.tcyr  -> 524 passed, 0 failed  (HEAD baseline: 512 passed -> +12)
No library defect found. Every independently derived value matched the library bit-for-bit or to 1 ulp, so this is purely a test-quality repair.

MEASUREMENT METHOD. /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/probe1.tcyr prints each residual as f64_to(|resid| * 1e18) through a deliberately failing assert_eq (assert_eq echoes "got N"). Measured floor, in units of 1e-18:
  rot90.x 222 | rot90.y-1 0 | rot90.z 0 | rot45.x-sqrt2/2 111 | rot45.y 0
  m4 prod[0][0]-1 0 | prod[1][1]-1 0 | prod[3][0] 0
  srgb roundtrip 166 | linear_to_srgb(0.2) pin 222
  |e^ipi+1| 122 | Re+1 0 | Im 122
  u1 inv phase 0 (raw bits 0x0)
  su2 w-cos(pi/8) 0 | su2 z-sin(pi/8) 0 | compose-inv w-1 0 | roundtrip dw 0
  all 8 gell-mann traces 0 | lambda8[0][0] pin 0 | lambda8[2][2] pin 0
  boost[0][0]-cosh 0 | boost[1][0]-sinh 0 | comp[0][0]-1 222 | comp[1][1]-1 222 | comp[0][1] 0

## CHALLENGE  vacuous=False compiles=True values_ok=True stands=True
- SURVIVES: src/complex.cyr cx_exp -- negate the imaginary output (return the CONJUGATE): `f64_neg(f64_mul(r, f64_sin(HComplex_im(a))))`. pi is a fixed point of conjugation, so |e^(i*pi)+1| is bit-identical and BOTH tightened Euler asserts ([H5] hisab:744, [M2] modules:692) still pass at 1e-15. VERIFIED SURVIVES ALL FOUR SUITES: hisab_new 237/0, modules_new 524/0, foundation_new 307/0, edge_cases 199/0. No tolerance can fix this -- it needs a pin off the real axis (e.g. cx_exp(i*pi/2).im == +1).
- SURVIVES: src/color.cyr linear_to_srgb -- use the encode-side breakpoint by mistake: `if (f64_lt(c, _SRGB_THRESHOLD_TO_LINEAR) == 1)` (0.04045) instead of _SRGB_THRESHOLD_TO_SRGB (0.0031308). This is exactly the 2.6.11-class mis-transcription the finding cites. Both the newly tightened round-trip (lin = 0.214) and the NEW absolute pin linear_to_srgb(0.2) sit in the power segment, so neither sees it. VERIFIED SURVIVES ALL FOUR SUITES. The repair added a below-breakpoint probe for srgb_to_linear in an earlier pass but the new inverse-leg pin has no counterpart.
- SURVIVES: src/lie.cyr su2_compose -- sign flip in the z cross term: `f64_sub(f64_mul(ax, by), f64_mul(ay, bx))` -> `f64_add(...)`. w of q*q^-1 is w^2+x^2+y^2+z^2, an inner product untouched by the vector-part formula, so [M4] 'su2 compose inv w == 1 (1e-15)' passes; [M5] only compares w's; the new cos(pi/8)/sin(pi/8) pins test su2_from_axis_angle, not su2_compose. VERIFIED SURVIVES ALL FOUR SUITES. Every su2 assertion in the repo looks only at w.
- SURVIVES: src/lie.cyr gell_mann -- swap the lambda_4 and lambda_5 bodies (an index-dispatch shift: idx==3 returns the i-valued matrix, idx==4 the real one). All 8 generators stay hermitian AND traceless, and the repair only pins entries of lambda_2 and lambda_8. VERIFIED SURVIVES ALL FOUR SUITES. Hermiticity + tracelessness are invariant under any permutation of the generators.
- SURVIVES: src/lie.cyr minkowski_interval -- drop the z^2 term (or the y^2 term): `f64_sub(f64_mul(t,t), f64_add(f64_mul(x,x), f64_mul(y,y)))`. The test 4-vector is (5,3,0,0), so both the new exact '== +16' pin and the new 1e-13 drift bound pass unchanged. VERIFIED SURVIVES ALL FOUR SUITES. The repair fixed the tolerance but left the degenerate input.
- SURVIVES: src/complex.cyr cmat_adjoint -- stub it to a plain copy: `cmat_set(r, i, j, cmat_get(m, i, j))` (no transpose, no conjugate). The new 8-generator, 9-entry, re+im hermitian sweep then compares A with A and is vacuous again -- the exact failure mode [M7] set out to fix. VERIFIED SURVIVES ALL FOUR SUITES. The only other adjoint coverage is 'adj(I) trace=3' (modules:737-739), which an identity function satisfies. (The weaker mutant -- transpose without conjugation -- IS caught by the new sweep.)
- SURVIVES: src/mat4.cyr m4_inverse -- transpose the returned upper-left 3x3 (row/column-major slip). `srt` in [H3] is a PURE TRANSLATION, so that block is the identity and the transpose is a no-op: the new 16-entry sweep passes. VERIFIED: survives hisab_new 237/0, modules_new 524/0, edge_cases 199/0; caught only by foundation.tcyr's rotation-inverse test. Repo-level escape: no. Site-level blindness: yes -- the sweep is a real improvement (it catches a dropped translation column, see below) but the input is still degenerate.
- SURVIVES: src/quat.cyr hquat_rotate_vec3 -- sign flip in the tz cross term (`f64_sub` -> `f64_add`). A Z-axis rotation applied to unit_x has qx = qy = vy = vz = 0, so BOTH the new rot90 pins and the new rot45 angle pin pass. VERIFIED: survives hisab_new 237/0 and modules_new 524/0; caught by foundation.tcyr. Same story as above: the added 45-degree case pins the ANGLE but not the rotation formula, because it reuses the same sparse axis/vector pair.
- SURVIVES: src/num.cyr num_newton -- ignore the `tol` argument and hardcode 1e-6 (the 2.6.12 solve_bicgstab no-op-tolerance defect class). Newton on sin converges cubically and overshoots (0.5 -> -4.63e-2 -> 3.31e-5 -> 1.21e-14), so the returned root is still 1.21e-14 and the new |x| < 1e-12 bound passes. VERIFIED SURVIVES modules_new 524/0. Weak mutant (the returned value is still correct), but worth recording: the new bound pins the ROOT, not tol-honoring, and it is self-referential -- EPSILON_F64 is both the tol passed in and the assertion bound, so a wrong global epsilon moves both together.

### CORRECTED
KEEP THE PROPOSED PATCH AS WRITTEN -- it is genuinely strong and should land. Then ADD the block below, which closes the seven surviving gaps.

WHAT I VERIFIED ABOUT THEIR PATCH (all on a scratchpad copy of the repo at /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/adv2):
* Census is complete and accurate. 15 static f64_from(1)-as-tolerance assert sites (6 hisab, 9 modules) + 8 loop iterations = 22 runtime; every quote is verbatim; `\bTOL\b` in tests/ returns exactly foundation.tcyr:15 (dead). No other f64_from(N) tolerance sites exist.
* Applies cleanly: every anchor string is unique in the CURRENT working tree.
* Compiles and passes: hisab 232 -> 237 (0 failed), modules 512 -> 524 (0 failed), foundation 307 -> 307 (0 failed). NOTE: their "hisab 226 -> 231" does not reproduce -- the delta (+5) is right, the absolute numbers are not, because tests/hisab.tcyr has uncommitted concurrent edits from another repair. Not a defect in the patch.
* Every hex constant decodes to its comment (python3 struct, 13/13), and every derivation re-derives: sqrt(2)/2, linear_to_srgb(0.2)=0.48452920448170694, cos/sin(pi/8), 1/sqrt(3) with 2*(1/sqrt3)==2/sqrt3 True, cosh/sinh(0.5), t'=7.201415742513145 / x'=5.988354423087879 / drift 7.105427357601002e-15.
* Measured residuals match their claims EXACTLY (probe printing resid*1e18): rot90.x 222, rot45.x 111, rot45.y 0, srgb rt 166, srgb enc(0.2) 222, euler 122, lorentz comp[0][0] 222, interval drift 7105. Headroom 4.5x (1e-15) and 14x (1e-13).
* Cyrius-correct: no f64_from on a hex literal, no integer compare on an f64 pattern, no 'else if', no negative literals, no reserved iv_* names, 0 lines >120, `cyrius lint` 0 warnings on all three files.
* It has real teeth. 11 mutants CAUGHT, each by an assertion the patch adds: quat full-angle (4 fails); su2_from_axis_angle full-angle -- their headline claim, CONFIRMED; lorentz_boost_x reading velocity instead of rapidity; lorentz_inv returning identity; lambda_8 normalized as diag(.5,.5,-1); lambda_2 and lambda_5 imaginary sign flips (hermitian sweep); minkowski signature flip (the +16 pin); su2_to_quat w/z swap; cmat_adjoint dropping conjugation; num_newton premature exit (the rc check); and m4_inverse dropping the translation column -- caught ONLY by the new 16-entry sweep, which therefore earns its keep.

NITS (optional, none block landing):
1. hisab item (3): the sweep reads `m4_get(prod, _mi_c, _mi_r)` while the loop vars are named row/col. mat4 is column-major with m4_get(m, col, row), and the expected pattern is symmetric, so it is harmless -- but rename to _mi_col/_mi_row so it does not read as a transposition bug.
2. hisab item (3): the standalone [0][0] assert is fully subsumed by the sweep.
3. modules item (18): `assert_eq(f64_to(l_intv_before), 16)` truncates toward zero, so it accepts [16, 17). Same for the lambda_2 pins, which accept (-2, -1]. Use an f64_abs/f64_sub band if you want them exact.

=== ADD THIS (verified: 22 new assertions, 0 failed against unmodified src; lint clean; 0 lines >120; spliced into both patched suites -> hisab 242/0, modules 546/0, no name collisions). Each block kills one surviving mutant, confirmed by re-running the mutant against it. ===

--- (A) modules.tcyr AND hisab.tcyr, right after each euler assert -- uses ULP_TOL_M / ULP_TOL_H respectively:
# e^(i*pi)+1 == 0 is BLIND to conjugation: pi is a fixed point of z -> conj(z), so
# negating the imaginary output of cx_exp leaves this bit-identical (verified: such
# a mutant passes all four suites). Pin a point where the two differ.
# e^(i*pi/2) = 6.123233995736766e-17 + 1i
var _e_ih = cx_exp(cx_new(0, F64_PI_2));
assert(f64_lt(f64_abs(HComplex_re(_e_ih)), ULP_TOL_M) == 1, "cx_exp(i*pi/2).re == 0");
assert(f64_lt(f64_abs(f64_sub(HComplex_im(_e_ih), F64_ONE)), ULP_TOL_M) == 1,
"cx_exp(i*pi/2).im == +1 (not -1)");

--- (B) hisab.tcyr, after the new linear_to_srgb(0.2) pin:
# The 0.2 pin is in the POWER segment, so it does not pin the encode-side
# breakpoint. Using 0.04045 (the decode threshold) here instead of 0.0031308
# passes every existing colour assertion -- verified against all four suites.
# linear_to_srgb(0.004) = 1.055*0.004^(1/2.4) - 0.055 = 0.05070871397734796,
# vs 12.92*0.004 = 0.05168 if the linear branch is wrongly taken.
var _lts_in_hi = 0x3F70624DD2F1A9FC;   # 0.004
var _lts_hi = 0x3FA9F67E184F52AB;      # 0.05070871397734796
assert(f64_lt(f64_abs(f64_sub(linear_to_srgb(_lts_in_hi), _lts_hi)), ULP_TOL_H) == 1,
"linear_to_srgb(0.004) uses the power segment");
var _lts_in_lo = 0x3F60624DD2F1A9FC;   # 0.002
var _lts_lo = 0x3F9A75CD0BB6ED68;      # 0.025840000000000002 = 12.92 * 0.002
assert(f64_lt(f64_abs(f64_sub(linear_to_srgb(_lts_in_lo), _lts_lo)), ULP_TOL_H) == 1,
"linear_to_srgb(0.002) uses the linear segment");

--- (C) modules.tcyr, in the SU(2) section after the compose-inverse asserts:
# w of q*q^-1 is an inner product, so EVERY su2 assertion in the repo (w == 1, the
# to_quat dw, the axis-angle pins) survives a sign flip in su2_compose's x/y/z
# cross terms -- verified. Pin all four components of a product of two
# non-parallel rotations.
var _c07 = 0x3FE6666666666666;  # 0.7
var _c11 = 0x3FF199999999999A;  # 1.1
var _sa = su2_from_axis_angle(hvec3_new(F64_ONE, F64_TWO, f64_from(3)), _c07);
var _sb = su2_from_axis_angle(hvec3_new(f64_neg(F64_TWO), F64_ONE, F64_HALF), _c11);
var _sp = su2_compose(_sa, _sb);
var _sp_w = 0x3FE89F9435EE839F;   # 0.7694798520425258
var _sp_x = 0xBFD91ACF6D1B1AAA;   # -0.392261368321139
var _sp_y = 0x3FCE094E17426F47;   # 0.23465896735876354
var _sp_z = 0x3FDC8C331EC15140;   # 0.44605710986549596  (a z-sign flip gives 0.2788)
assert(f64_lt(f64_abs(f64_sub(_su2_w(_sp), _sp_w)), TOL_1EM12) == 1, "su2_compose generic w");
assert(f64_lt(f64_abs(f64_sub(_su2_x(_sp), _sp_x)), TOL_1EM12) == 1, "su2_compose generic x");
assert(f64_lt(f64_abs(f64_sub(_su2_y(_sp), _sp_y)), TOL_1EM12) == 1, "su2_compose generic y");
assert(f64_lt(f64_abs(f64_sub(_su2_z(_sp), _sp_z)), TOL_1EM12) == 1, "su2_compose generic z");
# where: var TOL_1EM12 = 0x3D719799812DEA11;  # 1e-12 (== EPSILON_F64)

--- (D) modules.tcyr, in the Gell-Mann section:
# Hermitian + traceless is invariant under PERMUTING the generators: swapping the
# lambda_4 and lambda_5 bodies passes all four suites. Pin each one's identity.
assert_eq(f64_to(HComplex_re(cmat_get(gell_mann(3), 0, 2))), 1, "lambda_4[0][2] == +1 (real)");
assert_eq(f64_to(HComplex_im(cmat_get(gell_mann(3), 0, 2))), 0, "lambda_4[0][2] has no i");
assert_eq(f64_to(HComplex_im(cmat_get(gell_mann(4), 0, 2))), (0 - 1), "lambda_5[0][2] == -i");
assert_eq(f64_to(HComplex_re(cmat_get(gell_mann(4), 0, 2))), 0, "lambda_5[0][2] is pure imaginary");
assert_eq(f64_to(HComplex_re(cmat_get(gell_mann(5), 1, 2))), 1, "lambda_6[1][2] == +1 (real)");
assert_eq(f64_to(HComplex_im(cmat_get(gell_mann(6), 1, 2))), (0 - 1), "lambda_7[1][2] == -i");
assert_eq(f64_to(HComplex_re(cmat_get(gell_mann(2), 0, 0))), 1, "lambda_3[0][0] == +1");
assert_eq(f64_to(HComplex_re(cmat_get(gell_mann(2), 1, 1))), (0 - 1), "lambda_3[1][1] == -1");

--- (E) modules.tcyr, in the complex-matrix section (THIS IS WHAT KEEPS THE NEW HERMITIAN SWEEP NON-VACUOUS):
# If cmat_adjoint were a plain copy, the 8-generator hermitian sweep is A == A
# again -- verified: such a stub passes all four suites, because 'adj(I) trace==3'
# is satisfied by the identity function. Pin adjoint on a NON-hermitian matrix.
var _adj_m = cmat_new(2, 2);
cmat_set(_adj_m, 0, 1, cx_new(F64_TWO, f64_from(3)));
var _adj_r = cmat_adjoint(_adj_m);
assert_eq(f64_to(HComplex_re(cmat_get(_adj_r, 1, 0))), 2, "adj[1][0].re == 2 (transposed)");
assert_eq(f64_to(HComplex_im(cmat_get(_adj_r, 1, 0))), (0 - 3), "adj[1][0].im == -3 (conjugated)");
assert_eq(f64_to(HComplex_re(cmat_get(_adj_r, 0, 1))), 0, "adj[0][1] == 0 (not a copy)");

--- (F) modules.tcyr, in the Lorentz section after the interval asserts:
# (5,3,0,0) leaves y and z at zero, so dropping either spatial term is invisible
# to both the +16 pin and the 1e-13 drift bound -- verified across all four suites.
var _mv = alloc(32);
store64(_mv, f64_from(5)); store64(_mv + 8, f64_from(3));
store64(_mv + 16, f64_from(4)); store64(_mv + 24, F64_TWO);
# 25 - 9 - 16 - 4 = -4
assert_eq(f64_to(minkowski_interval(_mv)), (0 - 4), "interval(5,3,4,2) == -4");
var _mo = lorentz_transform_4vec(l_boost, _mv);
assert(f64_lt(f64_abs(f64_sub(minkowski_interval(_mo), minkowski_interval(_mv))), TOL_1EM12) == 1,
"boost preserves interval(5,3,4,2)");

--- (G) hisab.tcyr, after the 16-entry sweep:
# The sweep's input is a pure translation, whose 3x3 block is I -- transposing it
# is a no-op, so a row/column-major slip in m4_inverse still passes (verified;
# only foundation.tcyr catches it). Repeat with a non-symmetric rotation block.
var _rq = hquat_from_rotation_z(F64_PI_4);
var _rm = m4_from_srt(hvec3_one(), _rq, hvec3_new(F64_TWO, f64_from(3), f64_from(4)));
var _rp = m4_mul(_rm, m4_inverse(_rm));
var _rbad = 0;
var _rr = 0;
while (_rr < 4) {
    var _rc = 0;
    while (_rc < 4) {
        var _rwant = 0;
        if (_rr == _rc) { _rwant = F64_ONE; }
        if (f64_lt(f64_abs(f64_sub(m4_get(_rp, _rc, _rr), _rwant)), TOL_1EM14) == 0) {
            _rbad = _rbad + 1;
        }
        _rc = _rc + 1;
    }
    _rr = _rr + 1;
}
assert_eq(_rbad, 0, "rot+trans M * M^-1 == I in all 16 entries (1e-14)");
# where: var TOL_1EM14 = 0x3D06849B86A12B9B;  # 1e-14

Mutant re-run against these additions: D_cxexp_conj 1 fail, C_srgb_threshold 1 fail, E_su2_z_sign 1 fail, F_gm_swap45 4 fails, G_mink_drop_z 1 fail, S_adjoint_noop 3 fails, B_m4inv_transpose 1 fail. All seven survivors killed; 0 failures against unmodified src.