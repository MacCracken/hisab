<!-- 2.7.0-G test-quality record. Produced by a derive agent and an adversarial reviewer,
both of which RAN the code. All six repairs: not vacuous, compile, expected values verified
independently, verdict upheld. The SURVIVES lines are mutants the NEW assertions still do
not catch -- each is a precisely-identified next round of work, not a rejection of the
repair. Apply the replacement, then mutation-prove by reverting the source under test. -->

# simpson_tolerance
status=REPAIRED verified_passes=True

## WHY CURRENT CANNOT FAIL
Grep over src/ tests/ examples/ confirms calc_integral_simpson (src/calc.cyr:54) has exactly four accuracy call sites in the whole tree, none tolerance-based.

A) /home/macro/Repos/hisab/tests/modules.tcyr:383-402 (verbatim):
383  test_group("calc: simpson integration");
385  # int(x^2, 0, 1) = 1/3
386  var c_int_out = alloc(8);
387  var c_rc = calc_integral_simpson(&_tc_x2, 0, F64_ONE, 100, c_int_out);
388  assert_eq(c_rc, 0, "simpson ok");
389  var c_int_v = load64(c_int_out);
390  var c_int_3 = f64_to(f64_round(f64_mul(c_int_v, f64_from(3))));
391  assert_eq(c_int_3, 1, "simpson x^2 = 1/3");
393  # int(const 7, 0, 1) = 7
394  var c_const_out = alloc(8);
395  calc_integral_simpson(&_tc_const, 0, F64_ONE, 100, c_const_out);
396  assert_eq(f64_to(load64(c_const_out)), 7, "simpson const=7");
398  # int(x, 0, 2) = 2: linear function
400  var c_lin_out = alloc(8);
401  calc_integral_simpson(&_tc_linear, 0, F64_TWO, 100, c_lin_out);
402  assert_eq(f64_to(f64_round(load64(c_lin_out))), 2, "simpson linear");

B) /home/macro/Repos/hisab/tests/hisab.tcyr:348-356 (verbatim, same defect):
351  var rc = calc_integral_simpson(&_test_x2, 0, F64_ONE, 100, int_out);
352  assert_eq(rc, 0, "simpson ok");
354  var int_val = load64(int_out);
355  var int_x3 = f64_to(f64_round(f64_mul(int_val, f64_from(3))));
356  assert_eq(int_x3, 1, "int x^2 = 1/3");

WHY EACH CANNOT FAIL (bands computed from the quantiser, not guessed):
- L388 / hisab L352 "simpson ok": return-code only. calc_integral_simpson returns HSB_ERR_NONE for every steps != 0 after the even-rounding at src/calc.cyr:56; steps=100 can only return 0. It reads none of the 8 bytes it just wrote. Unfalsifiable for this input.
- L390-391 (and hisab L355-356): f64_round then f64_to. round(3I)==1 holds for 3I in [0.5, 1.5), i.e. I in [0.1666666666666667, 0.5). True value 1/3. Accepted relative error: -50.0% to +50.0%. A Simpson that returned 0.1667 or 0.4999 scores PASS.
- L396: f64_to TRUNCATES toward zero, so trunc(I)==7 holds for I in [7.0, 8.0). Accepted: 0% to +14.29%. (Asymmetric: 6.9999999 fails, 7.99 passes — the only thing this pins is "not low".)
- L402: round(I)==2 holds for I in [1.5, 2.5). Accepted: -25% to +25%.
Composite consequence, verified by simulation: an implementation whose every result is scaled by a uniform factor anywhere in [1.00, 1.1428) — i.e. up to +14.28% wrong on every integral it computes — leaves all three modules.tcyr assertions GREEN (checked at 1.01, 1.05, 1.10, 1.14: all pass). Deleting 

## REPLACEMENT
### PATCH 1 of 2 — tests/modules.tcyr

(1a) Add three helpers to the pre-alloc_init() helper block, immediately after
`fn _tc_linear(x) { return x; }` (currently line 48; the file requires all helpers
above `alloc_init();`):

fn _tc_cubic(x) {
    var cx2 = f64_mul(x, x);
    var cx3 = f64_mul(cx2, x);
    var ct1 = f64_sub(cx3, f64_mul(F64_TWO, cx2));
    var ct2 = f64_add(ct1, f64_mul(f64_from(3), x));
    return f64_sub(ct2, f64_from(4));
}
fn _tc_x4(x) { var qx2 = f64_mul(x, x); return f64_mul(qx2, qx2); }
fn _tc_inv1p(x) { return f64_div(F64_ONE, f64_add(F64_ONE, x)); }

(1b) Replace lines 383-402 in full (anchor: from `test_group("calc: simpson
integration");` through `assert_eq(f64_to(f64_round(load64(c_lin_out))), 2, "simpson linear");`)
with:

test_group("calc: simpson integration");

# EXACT WHERE THE RULE IS EXACT, TIGHT WHERE IT IS NOT.
# The assertions here through 2.7.x quantised the result through f64_to/f64_round
# and so certified almost nothing: round(3I) == 1 accepts I in [1/6, 1/2), i.e.
# -50%/+50% on 1/3; trunc(I) == 7 accepts [7, 8) (+14.3%); round(I) == 2 accepts
# [1.5, 2.5) (+/-25%). No tolerance-based Simpson assertion existed anywhere.
#
# Simpson's rule is EXACT for every polynomial of degree <= 3, and 4th-order
# otherwise: int - S = -(b-a) h^4 f''''(xi) / 180. Each case below runs on a
# dyadic grid -- n = 64 gives h = 2^-6 on [0,1] and h = 2^-5 on [0,2] -- so every
# node i*h, every f(x_i), every weighted term (the weights 4 and 2 are powers of
# two) and every partial sum is an exactly representable dyadic well inside 2^53.
# The ONLY rounding in the whole computation is the closing divide by 3, which is
# correctly rounded. The polynomial answers are therefore bit-exact and are
# asserted as bit patterns; every constant below is the closed-form value, not a
# recorded output.
var c_int_out = alloc(8);

# degree 0: int(7, 0, 1) = 7. T = 7*(2 + 4*32 + 2*31) = 1344, T*h = 21, 21/3 = 7.
var c_rc = calc_integral_simpson(&_tc_const, 0, F64_ONE, 64, c_int_out);
assert_eq(c_rc, 0, "simpson ok");
assert_eq(load64(c_int_out), 0x401C000000000000, "simpson deg-0: int(7,0,1) is 7.0 bit-exact");

# degree 1: int(x, 0, 2) = 2. T = 192, T*h = 6, 6/3 = 2.
calc_integral_simpson(&_tc_linear, 0, F64_TWO, 64, c_int_out);
assert_eq(load64(c_int_out), 0x4000000000000000, "simpson deg-1: int(x,0,2) is 2.0 bit-exact");

# degree 2: int(x^2, 0, 1) = 1/3. T = 64 exactly, T*h = 1.0, and 1.0/3.0 is the
# correctly rounded fl(1/3) = 0x3FD5555555555555.
calc_integral_simpson(&_tc_x2, 0, F64_ONE, 64, c_int_out);
assert_eq(load64(c_int_out), 0x3FD5555555555555, "simpson deg-2: int(x^2,0,1) is fl(1/3) bit-exact");

# degree 3 -- the defining property. int(x^3 - 2x^2 + 3x - 4, 0, 2)
#           = 4 - 16/3 + 6 - 8 = -10/3. T = -320, T*h = -10, fl(-10/3).
calc_integral_simpson(&_tc_cubic, 0, F64_TWO, 64, c_int_out);
assert_eq(load64(c_int_out), 0xC00AAAAAAAAAAAAB, "simpson deg-3: int(cubic,0,2) is fl(-10/3) bit-exact");

# degree 4 -- where the rule stops being exact, and by exactly how much. f'''' is
# constant for a quartic, so Euler-Maclaurin is an identity, not an estimate:
#   S_n = int + (b-a) h^4 f''''/180 = 1/5 + 24 h^4/180 = 1/5 + 2 h^4/15.
# h = 2^-6 gives 2/(15*2^24) = 7.947285970052083e-09, so the rule must return
# 0.20000000794728598 = 0x3FC99999AAAAAAAB -- provably NOT 0.2.
calc_integral_simpson(&_tc_x4, 0, F64_ONE, 64, c_int_out);
assert_eq(load64(c_int_out), 0x3FC99999AAAAAAAB, "simpson deg-4: int(x^4,0,1) is fl(1/5 + 2h^4/15)");
var c_q_dev = f64_sub(load64(c_int_out), 0x3FC999999999999A);   # minus fl(0.2)
assert(f64_gt(c_q_dev, 0x3E410D0FA854A07E) == 1, "simpson deg-4 error term > 7.94e-9");
assert(f64_lt(c_q_dev, 0x3E41128F084CFED7) == 1, "simpson deg-4 error term < 7.95e-9");

# The n=100 shape the old assertions used. h = fl(1/100) is not dyadic, so the
# nodes and node values round -- yet x^2 still lands under one ulp of 1/3.
calc_integral_simpson(&_tc_x2, 0, F64_ONE, 100, c_int_out);
var c_int_err = f64_abs(f64_sub(load64(c_int_out), 0x3FD5555555555555));
assert(f64_lt(c_int_err, 0x3C9CD2B297D889BC) == 1, "simpson x^2 n=100 within 1e-16 of 1/3 (< 2 ulp)");

# 1000 subintervals. The Neumaier compensation in the accumulator is what holds
# these two to zero error: deleting it costs 2 ulp at n=100 and 3 ulp here, so
# this pair is also the only regression test the compensated sum has.
calc_integral_simpson(&_tc_x2, 0, F64_ONE, 1000, c_int_out);
var c_int_err_k = f64_abs(f64_sub(load64(c_int_out), 0x3FD5555555555555));
assert(f64_lt(c_int_err_k, 0x3C9CD2B297D889BC) == 1, "simpson x^2 n=1000 within 1e-16 of 1/3");

# NON-POLYNOMIAL, so the rule is genuinely inexact: int(1/(1+x), 0, 1) = ln 2.
# 1/(1+x) on a dyadic grid is one correctly rounded IEEE divide, so the rule's
# value is computable in exact rational arithmetic independently of this library:
# S_64 = 0.6931471824214548 (0x3FE62E42FFFA11FA), S_32 = 0.693147210289823.
var c_ln2_64 = alloc(8);
calc_integral_simpson(&_tc_inv1p, 0, F64_ONE, 64, c_ln2_64);
var c_s64_err = f64_abs(f64_sub(load64(c_ln2_64), 0x3FE62E42FFFA11FA));
assert(f64_lt(c_s64_err, 0x3CD203AF9EE75616) == 1, "simpson 1/(1+x) n=64 hits exact rule value to 1e-15");

# 4th-order convergence: halving h must cut the error by 16. The exact-rational
# prediction is e(32)/e(64) = 15.9708437; the trapezoid rule would give ~4, and a
# mis-weighted Simpson gives O(1). e(64) itself is pinned to the h^4 term,
# (h^4/180)*(f'''(1) - f'''(0)) = 0.03125*h^4 = 1.8626e-09.
var c_ln2_32 = alloc(8);
calc_integral_simpson(&_tc_inv1p, 0, F64_ONE, 32, c_ln2_32);
var c_e64 = f64_abs(f64_sub(load64(c_ln2_64), 0x3FE62E42FEFA39EF));   # fl(ln 2)
var c_e32 = f64_abs(f64_sub(load64(c_ln2_32), 0x3FE62E42FEFA39EF));
assert(f64_gt(c_e64, 0x3E1EEC7BD512B572) == 1, "simpson n=64 error > 1.8e-9 (h^4 term is real)");
assert(f64_lt(c_e64, 0x3E20C01868BF779E) == 1, "simpson n=64 error < 1.95e-9");
var c_ratio = f64_div(c_e32, c_e64);
assert(f64_gt(c_ratio, 0x402FCCCCCCCCCCCD) == 1, "simpson 4th order: e(32)/e(64) > 15.9");
assert(f64_lt(c_ratio, 0x40300CCCCCCCCCCD) == 1, "simpson 4th order: e(32)/e(64) < 16.05");


### PATCH 2 of 2 — tests/hisab.tcyr

Replace lines 348-356 (anchor: from `# Integrate x^2 from 0 to 1 = 1/3` through
`assert_eq(int_x3, 1, "int x^2 = 1/3");`) with:

# Integrate x^2 from 0 to 1 = 1/3. TOLERANCE, not rounding: the assertion here
# through 2.7.x compared f64_to(f64_round(3*I)) against 1, which accepts any I in
# [1/6, 1/2) -- a -50%/+50% band around 1/3. Simpson is exact for degree <= 3.
fn _test_x2(x) { return f64_mul(x, x); }
var int_out = alloc(8);
var rc = calc_integral_simpson(&_test_x2, 0, F64_ONE, 100, int_out);
assert_eq(rc, 0, "simpson ok");
var int_err = f64_abs(f64_sub(load64(int_out), 0x3FD5555555555555));   # fl(1/3)
assert(f64_lt(int_err, 0x3C9CD2B297D889BC) == 1, "int x^2 = 1/3 within 1e-16 (< 2 ulp)");

# And exact on a dyadic grid: n = 64 over [0,2] gives h = 2^-5, so every node,
# node value, weighted term and partial sum is an exact dyadic and the only
# rounding is the closing divide by 3. int(x^3, 0, 2) = 4 must come back bit-exact.
fn _test_x3(x) { return f64_mul(x, f64_mul(x, x)); }
calc_integral_simpson(&_test_x3, 0, F64_TWO, 64, int_out);
assert_eq(load64(int_out), 0x4010000000000000, "simpson cubic: int(x^3,0,2) is 4.0 bit-exact");

(_test_x2 is still referenced at hisab.tcyr:359 and :368, so it must stay defined;
int_val / int_x3 have no other reader — grep confirms.)

## NOTES
RUN, not reasoned about. Both patches were spliced into copies of the real suites in the scratchpad and executed against the unmodified library:
  cyrius test /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/modules_simpson.tcyr -> 523 passed, 0 failed  (baseline tests/modules.tcyr: 512 passed; +11 net, 4 assertions replaced by 15)
  cyrius test /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/hisab_simpson.tcyr  -> 233 passed, 0 failed  (baseline tests/hisab.tcyr: 232 passed; +1)
Both re-run twice, stable. `cyrius lint` on both spliced files: 0 warnings, 0 untracked deferrals; no line exceeds 120 chars (longest is 105). NOTE: run cyrius from the repo root — the relative `include "src/*.cyr"` lines fail if cwd is the scratchpad ("cannot open include file"), which is the only compile error I saw and it was my cwd, not the code.

NO LIBRARY DEFECT FOUND. calc_integral_simpson is correct: it reproduced all five closed-form polynomial values bit-for-bit (0 ulp), landed 1 ulp from the exact-rational rule value on 1/(1+x), and converged at measured order 15.97084294 vs the predicted 15.970843745. The finding 

## CHALLENGE  vacuous=False compiles=True values_ok=True stands=True
- SURVIVES: a-drop in the step size: src/calc.cyr:57 `var h = f64_div(f64_sub(b, a), f64_from(steps));` -> `var h = f64_div(b, f64_from(steps));`. EXECUTED against the patched files: tests/modules.tcyr 523 passed / 0 failed, tests/hisab.tcyr 233/0, tests/edge_cases.tcyr 199/0, tests/foundation.tcyr 307/0. Every calc_integral_simpson call site in the whole tree (src, all four .tcyr, tests/hisab.bcyr:208, examples/basic_math.cyr:70) passes a = 0, and the proposed patch adds five new integrands without ever moving the lower bound off zero, so a mishandled lower bound is invisible. With a != 0 this mutant integrates over the wrong interval entirely.
- SURVIVES: a-drop in the node: src/calc.cyr:64 `var xi = f64_add(a, f64_mul(f64_from(i), h));` -> `var xi = f64_mul(f64_from(i), h);`. Same class, same measured result: 20/20 of the proposed assertions pass, plus edge_cases 199/0 and foundation 307/0. Both a-mutants are killed by the two-line addition in corrected_replacement (re-run confirms 18 passed / 2 failed).
- SURVIVES: odd-steps rounds DOWN instead of up: `if (steps % 2 == 1) { steps = steps + 1; }` -> `{ steps = steps - 1; }`. Survives the proposed patch (18/18 pass) because every new case uses an even n (64, 32, 100, 1000). NOTE: caught elsewhere in the tree by tests/edge_cases.tcyr:303-304 (steps=1 -> 0 -> HSB_ERR_ZERO_STEPS), so it is not a tree-level survivor. Not fixable by a polynomial assertion anyway -- Simpson is exact for degree <= 3 at ANY n, so n=63 and n=62 return identical bits (verified).
- SURVIVES: even-rounding deleted entirely: `if (steps % 2 == 1) { steps = steps + 1; }` -> removed. Survives the proposed patch (18/18 pass) for the same reason. Caught by edge_cases.tcyr:304 (`simpson 1 step const=5` fails). Would be killed inside the new block by one line: `calc_integral_simpson(&_tc_x2, 0, F64_ONE, 63, c_int_out); assert_eq(load64(c_int_out), 0x3FD5555555555555, ...)` -- verified that n=63 rounds up to 64 and returns fl(1/3) bit-exact, while the un-rounded n=63 returns 0x3FD5000581057081 (0.32812631).
- SURVIVES: NOT a survivor, but a redundancy worth noting: the two deg-4 band assertions (`c_q_dev > 7.94e-9`, `< 7.95e-9`) are strictly implied by the bit-exact assertion on the same buffer three lines above, and the two e(64) band assertions (1.8e-9 / 1.95e-9, a +/-4% window) are strictly implied by the `c_s64_err < 1e-15` pin (four orders tighter). Empirically confirmed: mutant `store64(out, f64_mul(total, f64_div(h, F64_THREE)))` shifted the deg-4 value by 1 ulp -- the bit-exact assertion fired, the bands did not. They add zero mutation-killing power. The e(32)/e(64) ratio band is NOT redundant: it is the only thing constraining S_32.

### CORRECTED
The proposed patch is sound as written -- it compiles, is lint-clean, passes on unmodified src (modules 523/523, hisab 233/233), and kills every mutant I threw at it except the `a` class. Only ONE addition is required. Append these 9 lines to the end of the new modules.tcyr block, immediately after the final `assert(f64_lt(c_ratio, 0x40300CCCCCCCCCCD) == 1, "simpson 4th order: e(32)/e(64) < 16.05");`:

# NONZERO LOWER BOUND. Every calc_integral_simpson call site in the tree -- all
# four suites, the benchmark and the example -- passed a = 0, so a dropped or
# mishandled `a` (in h, or in the node x_i = a + i*h) was undetectable: both
# mutations leave all four suites at 0 failed. [1,3] at n = 64 is still dyadic
# (h = 2^-5), so these stay bit-exact by the same argument as above:
#   int(x^2, 1, 3) = (27 - 1)/3 = 26/3   (T = 832, T*h = 26, fl(26/3))
#   int(x^3 - 2x^2 + 3x - 4, 1, 3) = (81/4 - 18 + 27/2 - 12) - (1/4 - 2/3 + 3/2 - 4) = 20/3
calc_integral_simpson(&_tc_x2, F64_ONE, f64_from(3), 64, c_int_out);
assert_eq(load64(c_int_out), 0x4021555555555555, "simpson a!=0: int(x^2,1,3) is fl(26/3) bit-exact");
calc_integral_simpson(&_tc_cubic, F64_ONE, f64_from(3), 64, c_int_out);
assert_eq(load64(c_int_out), 0x401AAAAAAAAAAAAB, "simpson a!=0: int(cubic,1,3) is fl(20/3) bit-exact");

VERIFIED: both constants derived with python3 struct.pack('>d') and cross-checked against exact-rational composite Simpson (Fraction arithmetic returns exactly 26/3 and 20/3 at n=64 on [1,3]). With these two lines the harness goes 20/20 on clean src, and both `a`-mutants become 18 passed / 2 failed. No line exceeds 120 chars; cyrius lint reports 0 warnings.

OPTIONAL (covers the odd-steps contract inside the accuracy block rather than relying solely on edge_cases.tcyr:303):
# steps is documented as "rounded up to even": 63 must behave exactly like 64.
calc_integral_simpson(&_tc_x2, 0, F64_ONE, 63, c_int_out);
assert_eq(load64(c_int_out), 0x3FD5555555555555, "simpson odd steps round UP: n=63 == n=64 == fl(1/3)");

OPTIONAL (drop, as strictly implied by the assertion above them and proven to add no kill power):
the two `simpson deg-4 error term` band assertions.