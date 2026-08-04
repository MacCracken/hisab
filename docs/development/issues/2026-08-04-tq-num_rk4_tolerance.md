<!-- 2.7.0-G test-quality record. Produced by a derive agent and an adversarial reviewer,
both of which RAN the code. All six repairs: not vacuous, compile, expected values verified
independently, verdict upheld. The SURVIVES lines are mutants the NEW assertions still do
not catch -- each is a precisely-identified next round of work, not a rejection of the
repair. Apply the replacement, then mutation-prove by reverting the source under test. -->

# num_rk4_tolerance
status=REPAIRED verified_passes=True

## WHY CURRENT CANNOT FAIL
SITE (the ONLY num_rk4 call in the entire tree — `grep -rn num_rk4 src/ tests/ examples/` returns
tests/edge_cases.tcyr:682 plus src/num.cyr:283 (the definition) and one doc-comment line; ode.cyr's
DOPRI45 and diffgeo.cyr's geodesic_rk4 are different functions and nothing in src/ calls num_rk4):

  tests/edge_cases.tcyr:676-683 (verbatim)
    # ================================================================
    # Num — RK4 basic test
    # ================================================================
    test_group("edge: num rk4");

    # dy/dt = 1 (constant), y(0)=0, dt=1 → y(1) = 1
    var erx = num_rk4(&_ec_dydt_const, 0, 0, F64_ONE);
    assert_eq(f64_to(erx), 1, "rk4 const dy/dt=1 → y=1");

  helper, tests/edge_cases.tcyr:50 (verbatim)
    fn _ec_dydt_const(t, y) { return F64_ONE; }

WHY IT CANNOT FAIL — two independent reasons, either one alone is fatal:

(1) THE TEST PROBLEM CARRIES NO ORDER INFORMATION. f(t,y) = 1 is a constant, so every stage of
    every explicit RK method returns 1: k1=k2=k3=k4=1, sum=6, y = 0 + (1/6)*6 = 1. A pure Euler
    step `y + dt*k1` returns exactly 1. So does midpoint/RK2, RK3, a version with every stage
    evaluated at the wrong t, a version with the wrong a-coefficients, a version with k3 built
    from k1. The ONLY defect class this input can even in principle expose is Sum(b_i) != 1 —
    consistency — and not even that reliably, see (2). Order 1 vs order 4 is invisible.
    Confirmed numerically: Euler, RK2, RK3, k4-evaluated-at-t+h/2, k3-from-k1, and
    all-stages-at-t all return exactly 1.0 on this input.

(2) THE COMPARISON IS A TRUNCATION, NOT A TOLERANCE. f64_to truncates toward zero, so
    `f64_to(erx) == 1` is satisfied by every result in [1.0, 2.0). The exact answer is 1.0, so
    the assertion accepts up to +99.999...% relative error (the audit's "90%" is a conservative
    statement of the same band; the true band is +100%-epsilon). It rejects only y < 1.0 or
    y >= 2.0. For a 4th-order integrator on a smooth ODE the achievable accuracy is ~1e-15
    relative per step — the assertion is roughly 1e15 times looser than the method's own
    round-off floor.

Net: num_rk4 has zero effective coverage. It could be replaced by `return f64_add(y, dt);`
(Euler) and the whole 4-suite, 1127-assertion tree would stay green.

## REPLACEMENT
TWO EDITS to /home/macro/Repos/hisab/tests/edge_cases.tcyr.

--- EDIT 1: insert after line 50 (`fn _ec_dydt_const(t, y) { return F64_ONE; }`, in the
    helper block above `alloc_init();`) ---

# Helpers for the RK4 order/accuracy tests
fn _ec_dydt_y(t, y) { return y; }
fn _ec_dydt_ty(t, y) { return f64_mul(t, y); }
fn _ec_dydt_4t3(t, y) { return f64_mul(f64_from(4), f64_mul(t, f64_mul(t, t))); }

# n fixed RK4 steps of size h starting from (t0, y0). Returns y(t0 + n*h).
fn _ec_rk4_run(f, t0, y0, h, n) {
    var t = t0;
    var y = y0;
    var i = 0;
    while (i < n) {
        y = num_rk4(f, t, y, h);
        t = f64_add(t, h);
        i = i + 1;
    }
    return y;
}

--- EDIT 2: replace lines 676-683 (the whole "Num — RK4 basic test" block) with ---

# ================================================================
# Num — RK4: 4th-order accuracy and observed order of convergence
# ================================================================
# Every reference value below is derived independently of src/num.cyr:
#   * y' = y  is integrated by ANY explicit RK method as y1 = y0*R(z),
#     R(z) = sum b_i over the tableau; for classical RK4 (and only for it)
#     R(z) = 1 + z + z^2/2 + z^3/6 + z^4/24  (textbook stability function).
#   * y' = 4t^3 makes RK4 collapse to Simpson's rule, exact for cubics.
#   * y' = t*y has the closed-form solution y(t) = y0*exp(t^2/2).
# Multi-step reference values were computed in exact rational arithmetic
# (python3 fractions) and every hex literal via python3 struct.pack('>d').
test_group("edge: num rk4 accuracy");

var RK4_T13   = 0x3D3C25C268497682;  # 1e-13
var RK4_T14   = 0x3D06849B86A12B9B;  # 1e-14
var RK4_T15   = 0x3CD203AF9EE75616;  # 1e-15
var RK4_2EM6  = 0x3EC0C6F7A0B5ED8D;  # 2e-6
var RK4_E     = 0x4005BF0A8B145769;  # e       = 2.718281828459045
var RK4_SQRTE = 0x3FFA61298E1E069C;  # sqrt(e) = 1.6487212707001282

# --- 1. y' = 1, y(0)=0, dt=1 -> y(1)=1 exactly (was the only rk4 assertion;
#        it is exact for Euler too, so it is kept only as a sanity check).
var erx = num_rk4(&_ec_dydt_const, 0, 0, F64_ONE);
assert(f64_approx_eq(erx, F64_ONE, RK4_T15), "rk4 const dy/dt=1 -> y=1 (1e-15)");

# --- 2. y' = y, y(0)=1, one step dt=1: R(1) = 65/24 = 2.7083333333333333.
#        Euler gives 2.0, midpoint/RK2 2.5, RK3 8/3 = 2.6666667 -- all fail 1e-13.
var erk_r1 = num_rk4(&_ec_dydt_y, 0, F64_ONE, F64_ONE);
assert(f64_approx_eq(erk_r1, 0x4005AAAAAAAAAAAB, RK4_T13), "rk4 R(1)=65/24 (1e-13)");

# --- 3. Same map at dt=1/2: R(1/2) = 211/128 = 1.6484375 exactly.
#        RK3 gives 1.6458333 (2.6e-3 off) -- a second point pins the polynomial.
var erk_rh = num_rk4(&_ec_dydt_y, 0, F64_ONE, F64_HALF);
assert(f64_approx_eq(erk_rh, 0x3FFA600000000000, RK4_T14), "rk4 R(1/2)=211/128 (1e-14)");

# --- 4. y' = 4t^3, y(0)=0, one step dt=1 -> Simpson: (1/6)(0 + 4*0.5 + 4) = 1,
#        which is also the exact solution t^4 at t=1. Euler gives 0, midpoint 0.5,
#        and evaluating k2/k3 at t instead of t+dt/2 gives 2/3 -- all fail 1e-15.
var erk_q = num_rk4(&_ec_dydt_4t3, 0, 0, F64_ONE);
assert(f64_approx_eq(erk_q, F64_ONE, RK4_T15), "rk4 cubic quadrature = 1 (1e-15)");

# --- 5. y' = y on [0,1] with n = 8/16/32 fixed steps, y(0)=1.
#        Exact-rational RK4 references: 2.7182768444167342, 2.7182815003405851,
#        2.7182818074111932 (errors vs e: 4.984e-6, 3.281e-7, 2.105e-8).
var erk_h8  = f64_div(F64_ONE, f64_from(8));
var erk_h16 = f64_div(F64_ONE, f64_from(16));
var erk_h32 = f64_div(F64_ONE, f64_from(32));
var erk_y8  = _ec_rk4_run(&_ec_dydt_y, 0, F64_ONE, erk_h8, 8);
var erk_y16 = _ec_rk4_run(&_ec_dydt_y, 0, F64_ONE, erk_h16, 16);
var erk_y32 = _ec_rk4_run(&_ec_dydt_y, 0, F64_ONE, erk_h32, 32);
assert(f64_approx_eq(erk_y8, 0x4005BF07EE21F39E, RK4_T13), "rk4 exp n=8 (1e-13)");
assert(f64_approx_eq(erk_y16, 0x4005BF0A5F0A46EA, RK4_T13), "rk4 exp n=16 (1e-13)");
assert(f64_approx_eq(erk_y32, 0x4005BF0A8841248C, RK4_T13), "rk4 exp n=32 (1e-13)");

# --- 6. Observed order: err(h)/err(h/2) must be ~2^4 = 16.
#        Measured 15.190 and 15.589; RK3 gives 7.61, RK2 3.82, Euler 1.90.
var erk_e8  = f64_abs(f64_sub(erk_y8, RK4_E));
var erk_e16 = f64_abs(f64_sub(erk_y16, RK4_E));
var erk_e32 = f64_abs(f64_sub(erk_y32, RK4_E));
var erk_ra = f64_div(erk_e8, erk_e16);
var erk_rb = f64_div(erk_e16, erk_e32);
assert(f64_gt(erk_ra, f64_from(14)) == 1, "rk4 order ratio 8->16 > 14");
assert(f64_lt(erk_ra, f64_from(17)) == 1, "rk4 order ratio 8->16 < 17");
assert(f64_gt(erk_rb, f64_from(14)) == 1, "rk4 order ratio 16->32 > 14");
assert(f64_lt(erk_rb, f64_from(17)) == 1, "rk4 order ratio 16->32 < 17");

# --- 7. Non-autonomous, non-polynomial: y' = t*y, y(0)=1 -> y(1) = sqrt(e).
#        n=8 exact-rational RK4 reference 1.6487206103294634 (err 6.604e-7);
#        n=16 reference 1.6487212322380775 (err 3.846e-8).
#        RK3 errs 9.40e-5, RK2 3.95e-3, Euler 1.25e-1 -- all fail the 2e-6 bound.
var erk_t8  = _ec_rk4_run(&_ec_dydt_ty, 0, F64_ONE, erk_h8, 8);
var erk_t16 = _ec_rk4_run(&_ec_dydt_ty, 0, F64_ONE, erk_h16, 16);
assert(f64_approx_eq(erk_t8, 0x3FFA6128DCD9B304, RK4_T13), "rk4 t*y n=8 (1e-13)");
assert(f64_approx_eq(erk_t16, 0x3FFA612983CAEF0F, RK4_T13), "rk4 t*y n=16 (1e-13)");
assert(f64_approx_eq(erk_t8, RK4_SQRTE, RK4_2EM6), "rk4 t*y n=8 vs sqrt(e) < 2e-6");
assert(f64_approx_eq(erk_t16, RK4_SQRTE, RK4_2EM6), "rk4 t*y n=16 vs sqrt(e) < 2e-6");

## NOTES
RAN IT. Nothing in /home/macro/Repos/hisab was modified (read-only respected).

1) GREEN AGAINST THE UNMODIFIED LIBRARY. tests/edge_cases.tcyr was copied to
   /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/rk4_probe.tcyr, both edits spliced in, then:
     cyrius test /tmp/.../scratchpad/rk4_probe.tcyr   ->  213 passed, 0 failed
   Baseline `cyrius test tests/edge_cases.tcyr` -> 199 passed, 0 failed. 199 - 1 + 15 = 213, so
   all 15 new assertions ran and passed. No library defect found: num_rk4 is a correct classical
   RK4 and matches the exact-rational reference to < 1e-13 at every point tested.
   `cyrius lint` on the spliced file: 0 warnings, 0 untracked deferrals. Longest line 105 chars
   (limit 120). No identifier collisions (`erk_*`, `RK4_*`, `_ec_dydt_y/_ty/_4t3`, `_ec_rk4_run`
   are all new). `cyrius fmt --check` takes only .cyr, so it was not run on the .tcyr; the block
   follows the file's existing 4-space/brace style.

2) TIGHTNESS PROVEN BY MUTATION, NOT BY ASSERTION. Written and run:
   /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/mutant_check.tcyr — 29 passed, 0 failed. It imple

## CHALLENGE  vacuous=False compiles=True values_ok=True stands=True
- SURVIVES: M7 `var half_dt = f64_mul(f64_abs(dt), _NUM_F64_HALF);` (sign-of-dt defect) — 16/16 assertions PASS. Verified by building the mutant at scratchpad/rk4w/muts/M7_abs_dt.cyr and running the proposed block against it.
- SURVIVES: M16 bogus guard `if (f64_lt(dt, 0) == 1) { return y; }` inserted at the top of num_rk4 — 16/16 assertions PASS. Same root cause: every proposed test uses dt > 0 (dt = 1, 1/2, 1/8, 1/16, 1/32) and t0 = 0, so backward/zero integration is an entirely untested regime. num_rk4's doc comment (src/num.cyr:278-282) puts no sign constraint on dt, and the consumers are physics/simulation engines that do integrate backwards.
- SURVIVES: (Not a defect, but diagnostic) M15 = the Kutta 3/8 rule — a DIFFERENT but equally valid order-4 tableau — is caught by ONLY 2 of the 15 assertions (`rk4 t*y n=8` and `rk4 t*y n=16` at 1e-13). Everything else (R(1), R(1/2), the cubic, all three exp references, all four order ratios) passes, because every 4-stage order-4 explicit RK method shares the stability function R(z) and is exact for cubics. So the identity of the classical tableau rests entirely on those two t*y assertions; if a future edit loosens or removes them, tableau-swap defects go undetected.

### CORRECTED
The proposed replacement is strong and should be taken essentially as written (12 of 13 realistic forward-integration mutants killed, every hex constant correct, compiles/lints/fmts clean, 199 -> 213 assertions). Only one change is required: close the dt <= 0 blind spot by APPENDING the following section 8 to the end of the proposed block, immediately after the line `assert(f64_approx_eq(erk_t16, RK4_SQRTE, RK4_2EM6), "rk4 t*y n=16 vs sqrt(e) < 2e-6");`. It needs no new helpers (it reuses _ec_dydt_y and _ec_dydt_4t3 from Edit 1) and no new tolerance constants.

# --- 8. Backward integration (dt < 0). Nothing above ever uses a negative or
#        zero step, so a sign defect in dt/half_dt is invisible to 1-7.
#        y' = y, one step dt=-1: R(-1) = 1 - 1 + 1/2 - 1/6 + 1/24 = 3/8 exactly.
#        y' = 4t^3 from (t=1, y=1) with dt=-1 lands on the exact solution t^4
#        at t=0, i.e. 0 (Simpson again, and it pins t_half = t + dt/2 < t).
var erk_rn = num_rk4(&_ec_dydt_y, 0, F64_ONE, f64_neg(F64_ONE));
assert(f64_approx_eq(erk_rn, 0x3FD8000000000000, RK4_T13), "rk4 R(-1)=3/8 (1e-13)");
var erk_qb = num_rk4(&_ec_dydt_4t3, F64_ONE, F64_ONE, f64_neg(F64_ONE));
assert(f64_approx_eq(erk_qb, 0, RK4_T15), "rk4 backward cubic t=1->0 gives 0 (1e-15)");

Both reference values are exact and independently derived: 3/8 is dyadic (0x3FD8000000000000 verified via python3 struct; exact-rational RK4 returns exactly Fraction(3,8)), and the backward cubic is exactly 0 in double precision too (the -1/6 * 6 product rounds to exactly -1.0 by round-half-to-even). Measured on unmodified src both pass with zero error; they kill M7 and M16 (both assertions fail on each), and also independently kill M1_euler and M3_k4_at_thalf.

Verification of the whole corrected block, run in a full working copy at
/tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/rk4w :
  cyrius test tests/edge_cases.tcyr  -> 215 passed, 0 failed (baseline 199)
  cyrius lint tests/edge_cases.tcyr  -> 0 warnings, 0 untracked deferrals
  cyrius fmt tests/edge_cases.tcyr --check -> rc 0
  no line > 120 chars; no f64_from on a hex literal; no integer compare on an f64;
  no `else if`; no negative literals (f64_neg used); no reserved iv_* identifiers.

ADDITIONAL NON-BLOCKING NOTES (do not require a change, but the claims should be stated accurately):

1. FOUR ASSERTIONS ARE LOGICALLY DEAD. Section 6's order-ratio band is fully implied by section 5: if erk_y8/y16/y32 each sit within 1e-13 (absolute) of references whose errors vs e are 4.984e-6 / 3.281e-7 / 2.105e-8, the ratios are pinned to 15.1898 +/- 3e-7 and 15.5892 +/- 5e-6. The ratios cannot fail unless section 5 already failed. Empirically: across 13 mutants, section 6 produced ZERO unique kills, and the two "< 17" assertions never fired against any mutant at all. Section 7's two "vs sqrt(e) < 2e-6" assertions are likewise implied by the two 1e-13 t*y reference assertions. Keeping them is fine as executable documentation of the order claim, but the comment "Observed order: err(h)/err(h/2) must be ~2^4 = 16" oversells them — the order is pinned by the pointwise references, not by the ratios.

2. COMMENT DRIFT (this repo's documented recurring defect class). The section-2 comment says "R(1) = 65/24 = 2.7083333333333333"; the literal 0x4005AAAAAAAAAAAB decodes to 2.7083333333333335. Change the comment digit (...335, not ...333). scripts/check-constants.sh only scans src/ so CI will not catch it, which is exactly why it is worth fixing by hand.

3. GARBLED MATH IN A COMMENT: "y1 = y0*R(z), R(z) = sum b_i over the tableau" is wrong — R(z) = 1 + z*b^T (I - zA)^-1 * 1, and sum b_i = 1 is merely the consistency condition. Suggested rewrite: "R(z) = 1 + z*b^T(I - zA)^{-1}e; for any 4-stage order-4 explicit method this is 1 + z + z^2/2 + z^3/6 + z^4/24."

4. TOLERANCE HEADROOM IS AMPLE, NOT FRAGILE. Measured actual-vs-reference on unmodified src: R(1) 1 ulp (4.44e-16), R(1/2) 0 ulp, cubic 0 ulp, const 0 ulp, exp n=8 1 ulp, n=16 1 ulp, n=32 0 ulp, t*y n=8 2 ulp, n=16 1 ulp. Against a 1e-13 absolute bound on values ~1.6-2.7 that is 225x-450x headroom, while being ~1e7 times tighter than the nearest wrong-order result. Measured order ratios 15.189765 and 15.589166, comfortably inside (14, 17).

5. THE FINDING ITSELF IS CONFIRMED EMPIRICALLY, not just by argument. I kept the old assertion `assert_eq(f64_to(num_rk4(&_ec_dydt_const, 0, 0, F64_ONE)), 1, ...)` in the mutant harness. It PASSED under all 13 mutants including the pure-Euler replacement `return f64_add(y, f64_mul(dt, k1));`, M8 which drops the +y entirely, M5 which uses weights (1,1,2,2)/6, and M12 which swaps the (t, y) argument order at every fncall2. Zero effective coverage, exactly as claimed.