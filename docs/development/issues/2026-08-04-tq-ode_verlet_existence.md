<!-- 2.7.0-G test-quality record. Produced by a derive agent and an adversarial reviewer,
both of which RAN the code. All six repairs: not vacuous, compile, expected values verified
independently, verdict upheld. The SURVIVES lines are mutants the NEW assertions still do
not catch -- each is a precisely-identified next round of work, not a rejection of the
repair. Apply the replacement, then mutation-prove by reverting the source under test. -->

# ode_verlet_existence
status=REPAIRED verified_passes=True

## WHY CURRENT CANNOT FAIL
/home/macro/Repos/hisab/tests/hisab.tcyr:476-487 — the entire current coverage of both integrators:

  476  # Verlet: simple harmonic oscillator q''=-q, start q=1,v=0
  477  # After dt=pi, q should be ≈ -1
  478  fn _ode_sho_accel(q) { return f64_neg(q); }
  479  var vr = ode_verlet(&_ode_sho_accel, F64_ONE, 0, F64_HALF);
  480  # Just check it returned a valid pointer (16 bytes: q, v)
  481  assert(vr != 0, "verlet returns result");
  482
  483  # Symplectic Euler: same SHO
  484  fn _ode_dp_dq(q) { return f64_neg(q); }
  485  fn _ode_dq_dp(p) { return p; }
  486  var se = ode_symplectic_euler(&_ode_dq_dp, &_ode_dp_dq, F64_ONE, 0, F64_HALF);
  487  assert(se != 0, "symplectic euler ok");

WHY THEY CANNOT FAIL

Line 481 and line 487 test a POINTER, never a value. Both functions end with the same three lines (/home/macro/Repos/hisab/src/ode.cyr:581-584 and :606-609):

    var result = alloc(16);
    store64(result, q_new);
    store64(result + 8, v_new);
    return result;

`result` is whatever `alloc(16)` returned, and it is returned unconditionally — there is no early-return path, no error path, no branch at all in either function. So `vr != 0` and `se != 0` are exactly the proposition "alloc(16) did not return 0".

hisab's allocator (/home/macro/Repos/hisab/lib/alloc.cyr) is a bump pointer over 256 MB anonymous mmap chunks, already initialised by `alloc_init()` at the top of the suite, with ~1400 assertions' worth of allocation still to go afterwards. For `alloc(16)` to return 0 the mmap chain would have to be exhausted or fail — at which point every one of the other 224 assertions in the file collapses too. Nothing about the integrators is being observed: `q_new` and `v_new` are stored and then never loaded. The comment on line 480 admits this outright ("Just check it returned a valid pointer").

Concretely: replace the bodies of both functions with `var result = alloc(16); return result;` (compute nothing, store nothing) and both assertions still pass. Both stored words could be NaN, zero, or the unmodified inputs. Lines 476-477 describe a dt=pi / q≈-1 test that was never written — the code passes dt=1/2 and checks nothing.

This is the same shape of hole called out five lines earlier in the same file for BDF-4 and Yoshida-4 ("had NO numeric assertions at all through 2.6.11, which is why their mis-transcribed coefficient tables shipped"). ode_verlet and ode_symplectic_euler were left in exactly that state.

## REPLACEMENT
Replace /home/macro/Repos/hisab/tests/hisab.tcyr lines 476-487 verbatim with the block below.
It keeps the three helper fns (same names, same bodies) so nothing else in the file moves,
and turns 2 vacuous assertions into 14 numeric ones. Longest line 117 chars.

# ----------------------------------------------------------------
# Symplectic integrators on the SHO  q'' = -q,  start (q, v) = (1, 0).
#
# Through 2.6.15 ode_verlet and ode_symplectic_euler were asserted ONLY with
# `!= 0` on the returned 16-byte block. alloc() never returns 0 on success, so
# neither assertion could fail for any input: two integrators, zero numeric
# coverage. Replaced below with the closed form of each map plus the property
# that makes a symplectic method worth having -- bounded energy error forever.
#
# Both maps are LINEAR on the SHO, so both have an exact closed form. Derived
# from the textbook definitions, independently of src/ode.cyr:
#   velocity Verlet  : q1 = q + v*h + (h^2/2)a(q),   v1 = v + (h/2)(a(q) + a(q1))
#                      M_V = [[1-h^2/2, h], [-h(1-h^2/4), 1-h^2/2]],  det = 1
#   symplectic Euler : p1 = p + h*F(q),              q1 = q + h*p1
#                      M_S = [[1-h^2,   h], [-h,           1     ]],  det = 1
# Both have trace 2 - h^2, so with theta = 2*asin(h/2)  (i.e. cos theta = 1-h^2/2)
# and M^n = cos(n th) I + (sin(n th)/sin th) (M - cos(th) I):
#   Verlet           q_n = cos(n th)                        v_n = -cos(th/2) sin(n th)
#   symplectic Euler q_n = cos(n th) - tan(th/2) sin(n th)  p_n = -sin(n th)/cos(th/2)
# Each conserves a quadratic shadow Hamiltonian EXACTLY (the S with M^T S M = S):
#   Verlet           E_V = (1 - h^2/4) q^2/2 + v^2/2
#   symplectic Euler H~  = (q^2 + p^2 - h*q*p)/2
# Since E = (q^2+v^2)/2 = E_V + (h^2/8) q^2 and |q_n| <= 1, the TRUE energy obeys
#   Verlet           |E_n - 1/2| <= h^2/8            for all n, forever
#   symplectic Euler |E_n - 1/2| <= h/(2(2-h))       for all n, forever
# That is the property under test. A non-symplectic method of the same order
# fails it outright: explicit Euler on this problem multiplies E by (1+h^2)^n,
# which is 49.7x over the 4000 steps below.
fn _ode_sho_accel(q) { return f64_neg(q); }
fn _ode_dp_dq(q) { return f64_neg(q); }
fn _ode_dq_dp(p) { return p; }

# (1) One step at dt = 1/2. Every intermediate is a dyadic rational, so the
# results are bit-exact and asserted as such:
#   Verlet           q1 = 1 - h^2/2 = 7/8,   v1 = -h + h^3/4 = -15/32
#   symplectic Euler q1 = 1 - h^2   = 3/4,   p1 = -h         = -1/2
var _SY_7_8    = 0x3FEC000000000000;   #   0.875     = 7/8
var _SY_N15_32 = 0xBFDE000000000000;   #  -0.46875   = -15/32
var _SY_3_4    = 0x3FE8000000000000;   #   0.75      = 3/4
var _SY_NHALF  = 0xBFE0000000000000;   #  -0.5
var vr = ode_verlet(&_ode_sho_accel, F64_ONE, 0, F64_HALF);
assert_eq(load64(vr), _SY_7_8, "verlet SHO dt=1/2: q1 == 7/8 exactly");
assert_eq(load64(vr + 8), _SY_N15_32, "verlet SHO dt=1/2: v1 == -15/32 exactly");
var se = ode_symplectic_euler(&_ode_dq_dp, &_ode_dp_dq, F64_ONE, 0, F64_HALF);
assert_eq(load64(se), _SY_3_4, "sympeuler SHO dt=1/2: q1 == 3/4 exactly");
assert_eq(load64(se + 8), _SY_NHALF, "sympeuler SHO dt=1/2: p1 == -1/2 exactly");

# (2) 4000 steps at h = 1/32 -> t = 125.0, 19.89 periods. Reference values below
# were evaluated from the closed form at 60 decimal digits and rounded to f64;
# the observed error of the run is ~2e-15, so 1e-13 leaves ~50x headroom while
# still pinning the DISCRETE map (comparing against cos(t) instead would only
# support ~5e-3, since the h^2/24 frequency error is 5.1e-3 rad over t = 125).
var _SY_TOL13 = 0x3D3C25C268497682;    # 1e-13
var _SY_TOL12 = 0x3D719799812DEA11;    # 1e-12
var _SY_0P9999 = 0x3FEFFF2E48E8A71E;   # 0.9999
var _SY_0P99   = 0x3FEFAE147AE147AE;   # 0.99
var _sy_h = f64_div(F64_ONE, f64_from(32));                     # 1/32, exact
var _sy_h28 = f64_div(f64_mul(_sy_h, _sy_h), f64_from(8));      # h^2/8 = 2^-13
var _sy_i = 0;
var _sy_st = 0;
var _sy_en = 0;
var _sy_dev = 0;

var _vlt_q = F64_ONE;
var _vlt_v = 0;
var _vlt_dmax = 0;
_sy_i = 0;
while (_sy_i < 4000) {
    _sy_st = ode_verlet(&_ode_sho_accel, _vlt_q, _vlt_v, _sy_h);
    _vlt_q = load64(_sy_st);
    _vlt_v = load64(_sy_st + 8);
    _sy_en = f64_mul(F64_HALF, f64_add(f64_mul(_vlt_q, _vlt_q), f64_mul(_vlt_v, _vlt_v)));
    _sy_dev = f64_abs(f64_sub(_sy_en, F64_HALF));
    if (f64_gt(_sy_dev, _vlt_dmax) == 1) { _vlt_dmax = _sy_dev; }
    _sy_i = _sy_i + 1;
}
# q_4000 = cos(4000*th) = 0.790837995393526746698654537268764  -> 0x3FE94E8B7BD4C7DA
# v_4000 = -cos(th/2) sin(4000*th) = 0.611950828092119903368758 -> 0x3FE39519E72D4E03
var _VLT_QREF = 0x3FE94E8B7BD4C7DA;
var _VLT_VREF = 0x3FE39519E72D4E03;
assert(f64_lt(f64_abs(f64_sub(_vlt_q, _VLT_QREF)), _SY_TOL13) == 1,
"verlet: q after 4000 steps == cos(n*theta) to 1e-13");
assert(f64_lt(f64_abs(f64_sub(_vlt_v, _VLT_VREF)), _SY_TOL13) == 1,
"verlet: v after 4000 steps == -cos(theta/2)sin(n*theta) to 1e-13");
# Shadow Hamiltonian E_V = (1-h^2/4)q^2/2 + v^2/2 -- conserved exactly, so it
# still equals its t=0 value (1-h^2/4)/2 = 0.4998779296875 after 4000 steps.
var _VLT_EVREF = 0x3FDFFE0000000000;   # 0.4998779296875 = (1 - (1/32)^2/4)/2
var _vlt_ev = f64_mul(F64_HALF, f64_add(
f64_mul(f64_sub(F64_ONE, f64_div(f64_mul(_sy_h, _sy_h), f64_from(4))), f64_mul(_vlt_q, _vlt_q)),
f64_mul(_vlt_v, _vlt_v)));
assert(f64_lt(f64_abs(f64_sub(_vlt_ev, _VLT_EVREF)), _SY_TOL12) == 1,
"verlet: shadow Hamiltonian (1-h^2/4)q^2/2+v^2/2 held to 1e-12 over 4000 steps");
# Bounded energy error, both sides. Upper bound h^2/8 = 1.220703125e-4 is the
# symplecticity statement; measured max deviation is 1.2207030914740e-4. The
# LOWER bound is what rejects a frozen / no-op integrator, which would sit at
# E == 1/2 exactly and sail through the upper bound alone.
assert(f64_lt(_vlt_dmax, _sy_h28) == 1,
"verlet: |E - 1/2| <= h^2/8 for all 4000 steps (energy bounded, not drifting)");
assert(f64_gt(_vlt_dmax, f64_mul(_sy_h28, _SY_0P9999)) == 1,
"verlet: the h^2/8 energy bound is actually attained (orbit is traversed)");

var _seu_q = F64_ONE;
var _seu_p = 0;
var _seu_dmax = 0;
_sy_i = 0;
while (_sy_i < 4000) {
    _sy_st = ode_symplectic_euler(&_ode_dq_dp, &_ode_dp_dq, _seu_q, _seu_p, _sy_h);
    _seu_q = load64(_sy_st);
    _seu_p = load64(_sy_st + 8);
    _sy_en = f64_mul(F64_HALF, f64_add(f64_mul(_seu_q, _seu_q), f64_mul(_seu_p, _seu_p)));
    _sy_dev = f64_abs(f64_sub(_sy_en, F64_HALF));
    if (f64_gt(_sy_dev, _seu_dmax) == 1) { _seu_dmax = _sy_dev; }
    _sy_i = _sy_i + 1;
}
# q_4000 = cos(n th) - tan(th/2) sin(n th) = 0.800402062059679536397214 -> 0x3FE99CE4C906502B
# p_4000 = -sin(n th)/cos(th/2)            = 0.612100266633778540707798 -> 0x3FE396534C621425
var _SEU_QREF = 0x3FE99CE4C906502B;
var _SEU_PREF = 0x3FE396534C621425;
assert(f64_lt(f64_abs(f64_sub(_seu_q, _SEU_QREF)), _SY_TOL13) == 1,
"sympeuler: q after 4000 steps == cos(n th) - tan(th/2)sin(n th) to 1e-13");
assert(f64_lt(f64_abs(f64_sub(_seu_p, _SEU_PREF)), _SY_TOL13) == 1,
"sympeuler: p after 4000 steps == -sin(n th)/cos(th/2) to 1e-13");
# H~ = (q^2 + p^2 - h*q*p)/2 is conserved exactly; it started at 1/2.
var _seu_h = f64_mul(F64_HALF, f64_sub(
f64_add(f64_mul(_seu_q, _seu_q), f64_mul(_seu_p, _seu_p)),
f64_mul(_sy_h, f64_mul(_seu_q, _seu_p))));
assert(f64_lt(f64_abs(f64_sub(_seu_h, F64_HALF)), _SY_TOL12) == 1,
"sympeuler: shadow Hamiltonian (q^2+p^2-h*q*p)/2 == 1/2 to 1e-12 over 4000 steps");
# |E - 1/2| <= h/(2(2-h)) = 7.936507936507936e-3; measured max 7.936507535744e-3.
# Explicit Euler would reach 24.4 here, i.e. this bound is ~3000x violated by the
# obvious non-symplectic alternative.
var _seu_bnd = f64_div(_sy_h, f64_mul(F64_TWO, f64_sub(F64_TWO, _sy_h)));
assert(f64_lt(_seu_dmax, _seu_bnd) == 1,
"sympeuler: |E - 1/2| <= h/(2(2-h)) for all 4000 steps (energy bounded, not drifting)");
assert(f64_gt(_seu_dmax, f64_mul(_seu_bnd, _SY_0P99)) == 1,
"sympeuler: the h/(2(2-h)) energy bound is actually attained (orbit is traversed)");

## NOTES
NO LIBRARY DEFECT FOUND. ode_verlet and ode_symplectic_euler are both correct — they match
their closed forms to 2e-15 over 4000 steps and conserve their shadow Hamiltonians to 2.4e-15.
This is a pure test-quality repair; the library was not touched.

VERIFICATION ACTUALLY RUN (repo left byte-identical; `git status --porcelain` empty at the end)
  Scratch copy with the block spliced in:
    /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/hisab_ode_repair.tcyr
  cyrius test <that file>          -> 238 passed, 0 failed
  cyrius test tests/hisab.tcyr     -> 226 passed, 0 failed   (baseline; 226 - 2 + 14 = 238)
  cyrius lint <that file>          -> 0 warnings, 0 untracked deferrals
  Longest line in the new block: 117 chars (limit 120).
  (`cyrius fmt --check` only accepts *.cyr — it rejects any .tcyr, including the current
   tests/hisab.tcyr, so it is not a gate this file can be run through.)

TIGHTNESS — MUTATION TESTED, NOT ASSERTED
Six mutants of src/ode.cyr, each spliced into a scratch tree (originals untouched; run with
CYRIUS_ALLOW_ABSOLUTE_INCLUDES=1 since cyrius rejects absolute include paths by default):

  A  Verlet drops the (a + a_

## CHALLENGE  vacuous=False compiles=True values_ok=True stands=True
- SURVIVES: ode_verlet: alloc(8) instead of alloc(16) -- the 16-byte result block is under-allocated by half and store64(result+8, v_new) writes 8 bytes past it. Under the bump allocator the value still reads back correctly on the next line, so all 14 proposed assertions pass. (Killed by the addendum's guard-alloc.)
- SURVIVES: ode_symplectic_euler: alloc(8) instead of alloc(16) -- identical heap overrun, identical survival. (Killed by the addendum's guard-alloc.)
- SURVIVES: ode_verlet: return a module-level shared scratch buffer (var _ODE_VERLET_SCRATCH = alloc(16)) instead of a fresh alloc per call. Every result aliases every other -- a caller holding two step results silently gets one. The proposed block always consumes each 16-byte block before the next call, so all 14 assertions pass. (Killed by re-reading vr/se after the 4000-step loops.)
- SURVIVES: ode_verlet: replace `var a = fncall1(accel_fn, q);` with `var a = f64_neg(q);` -- i.e. ignore the caller's function pointer entirely and hardcode the SHO. All 14 assertions pass, because the block only ever passes a(q) = -q. The tests never establish that ode_verlet integrates the force it is handed.
- SURVIVES: ode_verlet: replace `var a_new = fncall1(accel_fn, q_new);` with `var a_new = f64_neg(q_new);` -- same hardcoding on the second (corrector) evaluation. All 14 pass.
- SURVIVES: ode_symplectic_euler: replace `var force = fncall1(dp_dq, q);` with `var force = f64_neg(q);` -- the dp_dq pointer is never actually invoked. All 14 pass.
- SURVIVES: ode_symplectic_euler: replace `var vel = fncall1(dq_dp, p_new);` with `var vel = p_new;` -- the dq_dp pointer is never actually invoked (the block only ever passes the identity as dq_dp, so a hardcoded identity is indistinguishable). All 14 pass.
- SURVIVES: (equivalent mutant, not a defect) ode_verlet: reassociate the position update to f64_mul(f64_mul(half_dt, dt), a). Survives because it is algebraically and bit-identical on these inputs -- listed only for completeness.

### CORRECTED
# ----------------------------------------------------------------
# Symplectic integrators on the SHO  q'' = -q,  start (q, v) = (1, 0).
#
# Through 2.6.15 ode_verlet and ode_symplectic_euler were asserted ONLY with
# `!= 0` on the returned 16-byte block. alloc() never returns 0 on success, so
# neither assertion could fail for any input: two integrators, zero numeric
# coverage. Replaced below with the closed form of each map plus the property
# that makes a symplectic method worth having -- bounded energy error forever.
#
# Both maps are LINEAR on the SHO, so both have an exact closed form. Derived
# from the textbook definitions, independently of src/ode.cyr:
#   velocity Verlet  : q1 = q + v*h + (h^2/2)a(q),   v1 = v + (h/2)(a(q) + a(q1))
#                      M_V = [[1-h^2/2, h], [-h(1-h^2/4), 1-h^2/2]],  det = 1
#   symplectic Euler : p1 = p + h*F(q),              q1 = q + h*p1
#                      M_S = [[1-h^2,   h], [-h,           1     ]],  det = 1
# Both have trace 2 - h^2, so with theta = 2*asin(h/2)  (i.e. cos theta = 1-h^2/2)
# and M^n = cos(n th) I + (sin(n th)/sin th) (M - cos(th) I):
#   Verlet           q_n = cos(n th)                        v_n = -cos(th/2) sin(n th)
#   symplectic Euler q_n = cos(n th) - tan(th/2) sin(n th)  p_n = -sin(n th)/cos(th/2)
# Each conserves a quadratic shadow Hamiltonian EXACTLY (the S with M^T S M = S):
#   Verlet           E_V = (1 - h^2/4) q^2/2 + v^2/2
#   symplectic Euler H~  = (q^2 + p^2 - h*q*p)/2
# Since E = (q^2+v^2)/2 = E_V + (h^2/8) q^2 and |q_n| <= 1, the TRUE energy obeys
#   Verlet           |E_n - 1/2| <= h^2/8            for all n, forever
#   symplectic Euler |E_n - 1/2| <= h/(2(2-h))       for all n, forever
# That is the property under test. A non-symplectic method of the same order
# fails it outright: explicit Euler on this problem multiplies E by (1+h^2)^n,
# which is 49.7x over the 4000 steps below.
fn _ode_sho_accel(q) { return f64_neg(q); }
fn _ode_dp_dq(q) { return f64_neg(q); }
fn _ode_dq_dp(p) { return p; }

# (1) One step at dt = 1/2. Every intermediate is a dyadic rational, so the
# results are bit-exact and asserted as such:
#   Verlet           q1 = 1 - h^2/2 = 7/8,   v1 = -h + h^3/4 = -15/32
#   symplectic Euler q1 = 1 - h^2   = 3/4,   p1 = -h         = -1/2
var _SY_7_8    = 0x3FEC000000000000;   #   0.875     = 7/8
var _SY_N15_32 = 0xBFDE000000000000;   #  -0.46875   = -15/32
var _SY_3_4    = 0x3FE8000000000000;   #   0.75      = 3/4
var _SY_NHALF  = 0xBFE0000000000000;   #  -0.5
var vr = ode_verlet(&_ode_sho_accel, F64_ONE, 0, F64_HALF);
assert_eq(load64(vr), _SY_7_8, "verlet SHO dt=1/2: q1 == 7/8 exactly");
assert_eq(load64(vr + 8), _SY_N15_32, "verlet SHO dt=1/2: v1 == -15/32 exactly");
var se = ode_symplectic_euler(&_ode_dq_dp, &_ode_dp_dq, F64_ONE, 0, F64_HALF);
assert_eq(load64(se), _SY_3_4, "sympeuler SHO dt=1/2: q1 == 3/4 exactly");
assert_eq(load64(se + 8), _SY_NHALF, "sympeuler SHO dt=1/2: p1 == -1/2 exactly");

# (2) 4000 steps at h = 1/32 -> t = 125.0, 19.89 periods. Reference values below
# are M^4000 applied to (1, 0) in exact rational arithmetic, rounded to f64;
# the observed error of the run is ~2e-15, so 1e-13 leaves ~50x headroom while
# still pinning the DISCRETE map (comparing against cos(t) instead would only
# support ~5e-3, since the h^2/24 frequency error is 5.1e-3 rad over t = 125).
var _SY_TOL13 = 0x3D3C25C268497682;    # 1e-13
var _SY_TOL12 = 0x3D719799812DEA11;    # 1e-12
var _SY_0P9999 = 0x3FEFFF2E48E8A71E;   # 0.9999
var _SY_0P99   = 0x3FEFAE147AE147AE;   # 0.99
var _sy_h = f64_div(F64_ONE, f64_from(32));                     # 1/32, exact
var _sy_h28 = f64_div(f64_mul(_sy_h, _sy_h), f64_from(8));      # h^2/8 = 2^-13
var _sy_i = 0;
var _sy_st = 0;
var _sy_en = 0;
var _sy_dev = 0;

var _vlt_q = F64_ONE;
var _vlt_v = 0;
var _vlt_dmax = 0;
_sy_i = 0;
while (_sy_i < 4000) {
    _sy_st = ode_verlet(&_ode_sho_accel, _vlt_q, _vlt_v, _sy_h);
    _vlt_q = load64(_sy_st);
    _vlt_v = load64(_sy_st + 8);
    _sy_en = f64_mul(F64_HALF, f64_add(f64_mul(_vlt_q, _vlt_q), f64_mul(_vlt_v, _vlt_v)));
    _sy_dev = f64_abs(f64_sub(_sy_en, F64_HALF));
    if (f64_gt(_sy_dev, _vlt_dmax) == 1) { _vlt_dmax = _sy_dev; }
    _sy_i = _sy_i + 1;
}
# q_4000 = cos(4000*th) = 0.790837995393526746698654537268764  -> 0x3FE94E8B7BD4C7DA
# v_4000 = -cos(th/2) sin(4000*th) = 0.611950828092119903368758 -> 0x3FE39519E72D4E03
var _VLT_QREF = 0x3FE94E8B7BD4C7DA;
var _VLT_VREF = 0x3FE39519E72D4E03;
assert(f64_lt(f64_abs(f64_sub(_vlt_q, _VLT_QREF)), _SY_TOL13) == 1,
"verlet: q after 4000 steps == cos(n*theta) to 1e-13");
assert(f64_lt(f64_abs(f64_sub(_vlt_v, _VLT_VREF)), _SY_TOL13) == 1,
"verlet: v after 4000 steps == -cos(theta/2)sin(n*theta) to 1e-13");
# Shadow Hamiltonian E_V = (1-h^2/4)q^2/2 + v^2/2 -- conserved exactly, so it
# still equals its t=0 value (1-h^2/4)/2 = 0.4998779296875 after 4000 steps.
var _VLT_EVREF = 0x3FDFFE0000000000;   # 0.4998779296875 = (1 - (1/32)^2/4)/2
var _vlt_ev = f64_mul(F64_HALF, f64_add(
f64_mul(f64_sub(F64_ONE, f64_div(f64_mul(_sy_h, _sy_h), f64_from(4))), f64_mul(_vlt_q, _vlt_q)),
f64_mul(_vlt_v, _vlt_v)));
assert(f64_lt(f64_abs(f64_sub(_vlt_ev, _VLT_EVREF)), _SY_TOL12) == 1,
"verlet: shadow Hamiltonian (1-h^2/4)q^2/2+v^2/2 held to 1e-12 over 4000 steps");
# Bounded energy error, both sides. Upper bound h^2/8 = 1.220703125e-4 is the
# symplecticity statement; measured max deviation is 1.2207030914740e-4. The
# LOWER bound is what rejects a frozen / no-op integrator, which would sit at
# E == 1/2 exactly and sail through the upper bound alone.
assert(f64_lt(_vlt_dmax, _sy_h28) == 1,
"verlet: |E - 1/2| <= h^2/8 for all 4000 steps (energy bounded, not drifting)");
assert(f64_gt(_vlt_dmax, f64_mul(_sy_h28, _SY_0P9999)) == 1,
"verlet: the h^2/8 energy bound is actually attained (orbit is traversed)");

var _seu_q = F64_ONE;
var _seu_p = 0;
var _seu_dmax = 0;
_sy_i = 0;
while (_sy_i < 4000) {
    _sy_st = ode_symplectic_euler(&_ode_dq_dp, &_ode_dp_dq, _seu_q, _seu_p, _sy_h);
    _seu_q = load64(_sy_st);
    _seu_p = load64(_sy_st + 8);
    _sy_en = f64_mul(F64_HALF, f64_add(f64_mul(_seu_q, _seu_q), f64_mul(_seu_p, _seu_p)));
    _sy_dev = f64_abs(f64_sub(_sy_en, F64_HALF));
    if (f64_gt(_sy_dev, _seu_dmax) == 1) { _seu_dmax = _sy_dev; }
    _sy_i = _sy_i + 1;
}
# q_4000 = cos(n th) - tan(th/2) sin(n th) = 0.800402062059679536397214 -> 0x3FE99CE4C906502B
# p_4000 = -sin(n th)/cos(th/2)            = 0.612100266633778540707798 -> 0x3FE396534C621425
var _SEU_QREF = 0x3FE99CE4C906502B;
var _SEU_PREF = 0x3FE396534C621425;
assert(f64_lt(f64_abs(f64_sub(_seu_q, _SEU_QREF)), _SY_TOL13) == 1,
"sympeuler: q after 4000 steps == cos(n th) - tan(th/2)sin(n th) to 1e-13");
assert(f64_lt(f64_abs(f64_sub(_seu_p, _SEU_PREF)), _SY_TOL13) == 1,
"sympeuler: p after 4000 steps == -sin(n th)/cos(th/2) to 1e-13");
# H~ = (q^2 + p^2 - h*q*p)/2 is conserved exactly; it started at 1/2.
var _seu_h = f64_mul(F64_HALF, f64_sub(
f64_add(f64_mul(_seu_q, _seu_q), f64_mul(_seu_p, _seu_p)),
f64_mul(_sy_h, f64_mul(_seu_q, _seu_p))));
assert(f64_lt(f64_abs(f64_sub(_seu_h, F64_HALF)), _SY_TOL12) == 1,
"sympeuler: shadow Hamiltonian (q^2+p^2-h*q*p)/2 == 1/2 to 1e-12 over 4000 steps");
# |E - 1/2| <= h/(2(2-h)) = 7.936507936507936e-3; measured max 7.936507535744e-3.
# Explicit Euler would reach 24.4 here, i.e. this bound is ~3000x violated by the
# obvious non-symplectic alternative.
var _seu_bnd = f64_div(_sy_h, f64_mul(F64_TWO, f64_sub(F64_TWO, _sy_h)));
assert(f64_lt(_seu_dmax, _seu_bnd) == 1,
"sympeuler: |E - 1/2| <= h/(2(2-h)) for all 4000 steps (energy bounded, not drifting)");
assert(f64_gt(_seu_dmax, f64_mul(_seu_bnd, _SY_0P99)) == 1,
"sympeuler: the h/(2(2-h)) energy bound is actually attained (orbit is traversed)");

# (3) Three holes a single-force, read-immediately test still leaves open:
#   * neither integrator is ever handed a force OTHER than a(q) = -q, so an
#     implementation that ignores accel_fn / dp_dq / dq_dp and hardcodes the
#     SHO passes every assertion above;
#   * the 16-byte result is always consumed before the next call, so a shared
#     static scratch buffer instead of a fresh alloc is invisible;
#   * alloc(8) instead of alloc(16) overruns the block by 8 bytes, invisible too
#     (the guard alloc below lands exactly on the overrun under a bump heap).
# Constant acceleration is integrated EXACTLY by both methods, so all of this is
# dyadic and asserted bit-exact.
fn _ode_grav2(q) { return f64_neg(F64_TWO); }     # a(q) = -2,  ignores q
fn _ode_grav1(q) { return f64_neg(F64_ONE); }     # F(q) = -1,  ignores q
fn _ode_vel2(p) { return f64_mul(F64_TWO, p); }   # dq/dp = 2p  (mass 1/2)
var _SY_1_4 = 0x3FD0000000000000;                 # 0.25
# Verlet, a = -2 constant, (q, v) = (0, 1), h = 1/2:
#   q1 = 0 + 1*(1/2) + (1/4)(1/2)(-2) = 1/4       v1 = 1 + (1/4)(-2 + -2) = 0
var _gv = ode_verlet(&_ode_grav2, 0, F64_ONE, F64_HALF);
var _gv_guard = alloc(8);
store64(_gv_guard, F64_ONE);
assert_eq(load64(_gv), _SY_1_4, "verlet a=-2: q1 == 1/4 exactly (accel_fn is really called)");
assert_eq(load64(_gv + 8), 0, "verlet a=-2: v1 == 0 exactly (result block is a full 16 bytes)");
# Symplectic Euler, F = -1 constant, dq/dp = 2p, (q, p) = (0, 1), h = 1/2:
#   p1 = 1 + (1/2)(-1) = 1/2                      q1 = 0 + (1/2)(2*(1/2)) = 1/2
var _gs = ode_symplectic_euler(&_ode_vel2, &_ode_grav1, 0, F64_ONE, F64_HALF);
var _gs_guard = alloc(8);
store64(_gs_guard, F64_ONE);
assert_eq(load64(_gs), F64_HALF, "sympeuler F=-1,dq/dp=2p: q1 == 1/2 exactly (both fn ptrs used)");
assert_eq(load64(_gs + 8), F64_HALF, "sympeuler F=-1,dq/dp=2p: p1 == 1/2 exactly (16-byte block)");
# A shared static scratch buffer inside either integrator would have been
# overwritten by the 4000-step loops above; a fresh alloc per call survives.
assert_eq(load64(vr), _SY_7_8, "verlet: first result still intact after 4001 later calls");
assert_eq(load64(se + 8), _SY_NHALF, "sympeuler: first result still intact after later calls");