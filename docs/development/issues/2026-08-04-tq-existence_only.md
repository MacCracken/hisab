<!-- 2.7.0-G test-quality record. Produced by a derive agent and an adversarial reviewer,
both of which RAN the code. All six repairs: not vacuous, compile, expected values verified
independently, verdict upheld. The SURVIVES lines are mutants the NEW assertions still do
not catch -- each is a precisely-identified next round of work, not a rejection of the
repair. Apply the replacement, then mutation-prove by reverting the source under test. -->

# existence_only
status=REPAIRED verified_passes=True

## WHY CURRENT CANNOT FAIL
All six verified verbatim against the working tree (line numbers current as of this run; note another agent concurrently added +38 lines to tests/hisab.tcyr below line 675 — my three anchors there are still unique and unmoved).

=== 1. eigen_power — /home/macro/Repos/hisab/tests/hisab.tcyr:668-675 ===
    var epm = mat_new(2, 2);
    mat_set(epm, 0, 0, f64_from(2));
    mat_set(epm, 1, 1, F64_ONE);
    var ep_val = alloc(8);
    var ep_vec = alloc(16);
    var ep_rc = eigen_power(epm, 2, 1000, ep_val, ep_vec);
    assert_eq(ep_rc, 0, "eigen_power ok");
    assert_eq(f64_to(f64_round(load64(ep_val))), 2, "eigen_power val=2");
WHY IT CANNOT FAIL: (a) A = diag(2,1), and the routine seeds v = e_0, which is ALREADY the exact dominant eigenvector — A·e_0 = 2·e_0 on iteration 1, so the power loop never has to converge to anything; (b) f64_round + f64_to means any value in [1.5, 2.5) scores PASS; (c) out_vec is never read at all — eigen_power could write garbage (or nothing) to it. Demonstrated: I mutated `var tol = EPSILON_F64;` -> `var tol = f64_from(1);` in src/linalg_ext.cyr (the convergence test becomes |dlambda| < 1.0, i.e. stop after ~1 iteration) and this assertion still PASSED.

=== 2. cga_point — tests/hisab.tcyr:239-242 ===
    # CGA: point embedding round-trip
    var cga_p = cga_point(F64_ONE, F64_TWO, f64_from(3));
    # Just check it's non-null (32 components allocated)
    assert(cga_p != 0, "cga point alloc");
WHY IT CANNOT FAIL: it asserts a pointer is non-null. cga_point's first statement is `cga_mv_zero()` which is `alloc(CGA_MV_SIZE)`; the only way to fail is OOM. Zero of the 32 blade components is inspected. Demonstrated: I flipped the ep sign (`f64_sub` -> `f64_add` at src/geo_advanced.cyr:1204), which destroys the defining null property P·P = 0 — the assertion still PASSED.

=== 3. sym_to_latex — tests/modules.tcyr:1789-1792 ===
    # LaTeX rendering returns a non-empty string
    var se_tex = sym_to_latex(expr_add(se_x, expr_const(F64_ONE)));
    assert_neq(se_tex, 0, "sym_to_latex returns a Str");
    assert(str_len(se_tex) > 0, "sym_to_latex output is non-empty");
WHY IT CANNOT FAIL: sym_to_latex is `str_builder_new(); _latex_write(...); str_builder_build(sb)` — it returns a non-null Str for every input, and `_latex_write` on an ADD unconditionally emits at least " + ", so length > 0 always. Any output whatsoever passes. Demonstrated: I changed the ADD paren guard `if (paren == 1)` -> `if (paren == 9)` (src/symbolic_ext.cyr:275), removin

## REPLACEMENT
Three surgical replacements, all verified. Each block below REPLACES the quoted current text
exactly (each anchor matched exactly once in the working tree). Full spliced files are at
/tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/{hisab,foundation,modules}_spliced.tcyr

################################################################################
# A. tests/hisab.tcyr:239-242 — REPLACE the 4-line cga_point block with:
################################################################################
# CGA: conformal point embedding.
#   P = x e1 + y e2 + z e3 + a ep + b em,   a = (p^2-1)/2,  b = (p^2+1)/2
# Through 2.6.15 this only asserted `cga_p != 0`, which cannot fail unless alloc
# returns null -- every component was unchecked. The two null-basis coefficients
# are FORCED by the two defining properties of the embedding (metric ep^2=+1,
# em^2=-1, e_inf = ep+em), so they are derivable without reading the source:
#   P.P     = p^2 + a^2 - b^2 = 0   (P must be a null vector)
#   P.e_inf = a - b           = -1  (P must be normalized)
# Solving: a = (p^2-1)/2, b = (p^2+1)/2. For p=(1,2,3): p^2=14, a=6.5, b=7.5.
# Both are dyadic rationals, hence exact in f64 -- assert bit equality.
var cga_p = cga_point(F64_ONE, F64_TWO, f64_from(3));
assert(cga_p != 0, "cga point alloc");
assert_eq(cga_mv_get(cga_p, 1), F64_ONE, "cga point e1 = 1");
assert_eq(cga_mv_get(cga_p, 2), F64_TWO, "cga point e2 = 2");
assert_eq(cga_mv_get(cga_p, 3), f64_from(3), "cga point e3 = 3");
assert_eq(cga_mv_get(cga_p, 4), 0x401A000000000000, "cga point ep = (14-1)/2 = 6.5");
assert_eq(cga_mv_get(cga_p, 5), 0x401E000000000000, "cga point em = (14+1)/2 = 7.5");
assert_eq(cga_mv_get(cga_p, 0), 0, "cga point scalar part = 0");
var cga_p_nz = 0;
var cga_pi = 6;
while (cga_pi < 32) {
    if (cga_mv_get(cga_p, cga_pi) != 0) { cga_p_nz = cga_p_nz + 1; }
    cga_pi = cga_pi + 1;
}
assert_eq(cga_p_nz, 0, "cga point writes only blades 1..5");
# The two derived invariants, recomputed from the returned components.
assert_eq(f64_sub(f64_add(f64_from(14), f64_mul(cga_mv_get(cga_p, 4), cga_mv_get(cga_p, 4))),
f64_mul(cga_mv_get(cga_p, 5), cga_mv_get(cga_p, 5))), 0, "cga point is null: P.P = 0");
assert_eq(f64_sub(cga_mv_get(cga_p, 4), cga_mv_get(cga_p, 5)), f64_neg(F64_ONE),
"cga point normalized: P.e_inf = -1");
# Fractional + negative coordinates: p = (0.5, -1.5, 2) -> p^2 = 6.5, a = 2.75, b = 3.75.
var cga_q = cga_point(F64_HALF, 0xBFF8000000000000, F64_TWO);
assert_eq(cga_mv_get(cga_q, 1), F64_HALF, "cga point q e1 = 0.5");
assert_eq(cga_mv_get(cga_q, 2), 0xBFF8000000000000, "cga point q e2 = -1.5");
assert_eq(cga_mv_get(cga_q, 3), F64_TWO, "cga point q e3 = 2");
assert_eq(cga_mv_get(cga_q, 4), 0x4006000000000000, "cga point q ep = (6.5-1)/2 = 2.75");
assert_eq(cga_mv_get(cga_q, 5), 0x400E000000000000, "cga point q em = (6.5+1)/2 = 3.75");

################################################################################
# B. tests/hisab.tcyr:494-506 — REPLACE the whole gradient-descent block
#    (from the "# Gradient descent: minimize f(x) = x^2" comment through the
#    `assert(f64_lt(opt_result, F64_ONE) == 1, "gd x near 0");` line) with:
################################################################################
# Gradient descent on an ANISOTROPIC quadratic with a non-zero minimizer:
#   f(x,y) = (x-3)^2 + 5(y+2)^2,   grad = (2(x-3), 10(y+2)),   x0 = (5, 3)
# Through 2.6.15 the test minimized x^2 from x0=5 with lr=0.5. That step size
# lands on the exact minimum in ONE update (x <- 5 - 0.5*10 = 0), and the only
# value assertion was |x| < 1 -- a window so wide that halving the step, or
# ignoring lr entirely, still passes; out_fmin was never read at all.
#
# Closed form. With lr, each coordinate contracts geometrically and independently:
#   x_k - 3 = (1 - 2*lr)^k * 2,   y_k + 2 = (1 - 10*lr)^k * 5
# At lr = 1/20 the factors are exactly 0.9 and 0.5.
fn _opt_f(x_ptr) {
    var dx = f64_sub(load64(x_ptr), f64_from(3));
    var dy = f64_add(load64(x_ptr + 8), F64_TWO);
    return f64_add(f64_mul(dx, dx), f64_mul(f64_from(5), f64_mul(dy, dy)));
}
fn _opt_grad(x_ptr, out) {
    var dx = f64_sub(load64(x_ptr), f64_from(3));
    var dy = f64_add(load64(x_ptr + 8), F64_TWO);
    store64(out, f64_mul(F64_TWO, dx));
    store64(out + 8, f64_mul(f64_from(10), dy));
    return 0;
}
var opt_lr = f64_div(F64_ONE, f64_from(20));          # 0.05
var opt_x0 = alloc(16);
store64(opt_x0, f64_from(5));
store64(opt_x0 + 8, f64_from(3));
var opt_out_x = alloc(16);
var opt_out_f = alloc(8);

# (a) Exactly 10 steps, tol unreachable -> pins the whole trajectory.
#   x = 3 + 2*0.9^10  = 3.6973568802
#   y = -2 + 5*0.5^10 = -1.9951171875   (exact: -2 + 5/1024)
#   f = (2*0.9^10)^2 + 5*(5/1024)^2 = 0.4864258276518282
var opt_rc = opt_gradient_descent(&_opt_f, &_opt_grad, opt_x0, 2, opt_lr, EPSILON_F64, 10,
opt_out_x, opt_out_f);
assert_eq(opt_rc, HSB_ERR_NO_CONVERGENCE, "gd 10 steps: no convergence yet");
assert(f64_approx_eq(load64(opt_out_x), 0x400D942FD810CBF6, EPSILON_F64),
"gd x_10 = 3 + 2*0.9^10 = 3.6973568802");
assert(f64_approx_eq(load64(opt_out_x + 8), 0xBFFFEC0000000000, EPSILON_F64),
"gd y_10 = -2 + 5/1024 = -1.9951171875");
assert(f64_approx_eq(load64(opt_out_f), 0x3FDF2199CB6C6FF8, EPSILON_F64),
"gd f_10 = 0.4864258276518282");
assert_eq(load64(opt_x0), f64_from(5), "gd does not clobber x0");
assert_eq(load64(opt_x0 + 8), f64_from(3), "gd does not clobber x0[1]");

# (b) Run to convergence with tol = 1e-6 on ||grad||. The stopping index is
#   determined in closed form: sqrt(4*(2*0.9^k)^2 + 100*(5*0.5^k)^2) < 1e-6
#   first holds at k = 145, giving x = 3 + 2*0.9^145 = 3.0000004636538438.
var opt_tol6 = 0x3EB0C6F7A0B5ED8D;                    # 1e-6
var opt_out_x2 = alloc(16);
var opt_out_f2 = alloc(8);
var opt_rc2 = opt_gradient_descent(&_opt_f, &_opt_grad, opt_x0, 2, opt_lr, opt_tol6, 1000,
opt_out_x2, opt_out_f2);
assert_eq(opt_rc2, HSB_ERR_NONE, "gd converged");
assert(f64_approx_eq(load64(opt_out_x2), 0x400800003E3B0657, EPSILON_F64),
"gd x* = 3 + 2*0.9^145 = 3.0000004636538438");
assert_eq(load64(opt_out_x2 + 8), 0xC000000000000000, "gd y* = -2 exactly");
assert(f64_lt(load64(opt_out_f2), 0x3DA5FD7FE1796495) == 1, "gd f* < 1e-11");
assert_eq(load64(opt_out_f2), _opt_f(opt_out_x2), "gd out_fmin == f(out_x)");
# Documented postcondition: the returned point satisfies ||grad|| < tol.
var opt_g = alloc(16);
_opt_grad(opt_out_x2, opt_g);
assert(f64_lt(f64_sqrt(f64_add(f64_mul(load64(opt_g), load64(opt_g)),
f64_mul(load64(opt_g + 8), load64(opt_g + 8)))), opt_tol6) == 1, "gd ||grad(x*)|| < tol");

################################################################################
# C. tests/hisab.tcyr:668-675 — REPLACE the 8-line eigen_power block with:
#    (the preceding comment "# Eigen power: dominant eigenvalue of [[2,0],[0,1]] = 2"
#     is now wrong and should go too)
################################################################################
# Through 2.6.15: A = diag(2,1) with `round(val) == 2`, i.e. anything in [1.5,2.5)
# passed -- and e_0 is ALREADY the exact eigenvector of diag(2,1), so the iteration
# never had to do anything. The eigenvector output was never read.
#
# A = [[2,1],[1,3]]. char poly lambda^2 - 5 lambda + 5 = 0  =>  lambda = (5 +- sqrt5)/2.
#   lambda_1 = (5 + sqrt5)/2 = 3.618033988749895   (= phi + 2)
# From (2 - lambda_1) x + y = 0, y = (lambda_1 - 2) x = phi x, so the eigenvector
# scaled to max-|component| = 1 (this routine's normalization) is
#   v = (1/phi, 1) = (0.6180339887498949, 1).
# Convergence ratio |lambda_2/lambda_1| = 0.381966, so 1000 iterations is ample.
var epm = mat_new(2, 2);
mat_set(epm, 0, 0, F64_TWO);   mat_set(epm, 0, 1, F64_ONE);
mat_set(epm, 1, 0, F64_ONE);   mat_set(epm, 1, 1, f64_from(3));
var ep_val = alloc(8);
var ep_vec = alloc(16);
var ep_rc = eigen_power(epm, 2, 1000, ep_val, ep_vec);
assert_eq(ep_rc, 0, "eigen_power ok");
var EP_TOL = 0x3DA5FD7FE1796495;   # 1e-11 (measured error is 2.5e-13; tol=EPSILON_F64
                                   # is the routine's own |dlambda| stopping test)
assert(f64_approx_eq(load64(ep_val), 0x400CF1BBCDCBFA54, EP_TOL),
"eigen_power lambda = (5+sqrt5)/2 = 3.618033988749895");
assert(f64_approx_eq(load64(ep_vec), 0x3FE3C6EF372FE94F, EP_TOL),
"eigen_power v[0] = 1/phi = 0.6180339887498949");
assert_eq(load64(ep_vec + 8), F64_ONE, "eigen_power v[1] = 1 (max component, exact)");
# Residual ||A v - lambda v||_inf, recomputed from the two outputs.
assert(f64_lt(f64_abs(f64_sub(f64_add(f64_mul(F64_TWO, load64(ep_vec)), load64(ep_vec + 8)),
f64_mul(load64(ep_val), load64(ep_vec)))), EP_TOL) == 1, "eigen_power residual row 0");
assert(f64_lt(f64_abs(f64_sub(f64_add(load64(ep_vec), f64_mul(f64_from(3), load64(ep_vec + 8))),
f64_mul(load64(ep_val), load64(ep_vec + 8)))), EP_TOL) == 1, "eigen_power residual row 1");

# 3x3 with a degenerate trailing spectrum: A = 3I + J (J = all ones).
# J has eigenvalues (3,0,0), so A has (6,3,3): lambda_1 = 6, v = (1,1,1).
var ep3 = mat_new(3, 3);
for (var ep3i = 0; ep3i < 3; ep3i = ep3i + 1) {
    for (var ep3j = 0; ep3j < 3; ep3j = ep3j + 1) {
        if (ep3i == ep3j) { mat_set(ep3, ep3i, ep3j, f64_from(4)); }
        else { mat_set(ep3, ep3i, ep3j, F64_ONE); }
    }
}
var ep3_val = alloc(8);
var ep3_vec = alloc(24);
assert_eq(eigen_power(ep3, 3, 1000, ep3_val, ep3_vec), 0, "eigen_power 3x3 ok");
assert(f64_approx_eq(load64(ep3_val), f64_from(6), EP_TOL), "eigen_power 3x3 lambda = 6");
assert(f64_approx_eq(load64(ep3_vec), F64_ONE, EP_TOL), "eigen_power 3x3 v[0] = 1");
assert(f64_approx_eq(load64(ep3_vec + 8), F64_ONE, EP_TOL), "eigen_power 3x3 v[1] = 1");
assert(f64_approx_eq(load64(ep3_vec + 16), F64_ONE, EP_TOL), "eigen_power 3x3 v[2] = 1");

################################################################################
# D. tests/foundation.tcyr:450-451 — APPEND after the existing "v4 length unit"
#    line (keep it; it is harmless, just insufficient):
################################################################################
# The line above cannot fail for any length() that is right on a unit basis
# vector -- sqrt(1) == 1 == 1^2, and f64_to TRUNCATES, so anything in [1,2)
# scores. The normalize round-trip below is also blind to a constant scale
# factor: length(v/length(v)) cancels it. Pin the real number instead.
#   |(1,2,3,4)| = sqrt(1+4+9+16) = sqrt(30) = 5.477225575051661
# 30 is exact in f64 and f64_sqrt is correctly rounded, so this is bit-exact.
assert_eq(hvec4_length(v4a), 0x4015E8ADD236A58F, "v4 length (1,2,3,4) = sqrt(30)");
# Negative and fractional components: (0.5,-1.5,2,-3)
#   |v|^2 = 0.25+2.25+4+9 = 15.5,  |v| = sqrt(15.5) = 3.9370039370059056
var v4m = hvec4_new(F64_HALF, 0xBFF8000000000000, F64_TWO, 0xC008000000000000);
assert_eq(hvec4_length_sq(v4m), 0x402F000000000000, "v4 length_sq mixed = 15.5");
assert_eq(hvec4_length(v4m), 0x400F7EFBEB8D4F12, "v4 length mixed = sqrt(15.5)");

################################################################################
# E. tests/modules.tcyr:1736 — APPEND immediately after the
#    `assert_eq(num_dst(ne_x, 0), HSB_ERR_INVALID_INPUT, ...)` line:
################################################################################
# The round-trip above is necessary but NOT sufficient: DST-I is symmetric with
# S^2 = ((n+1)/2) I, and so is -S, and so is every other symmetric involution
# (DCT-I included). A global sign flip, or the wrong kernel entirely, round-trips
# perfectly. Pin the kernel itself with delta inputs, which read off one column:
#   x = e_j  =>  X[k] = sin(PI (j+1)(k+1) / (n+1))
# With n = 7 the angles are exact multiples of PI/8, so the column is a closed
# form: sin(PI/8)=0.3826834323650898, sin(PI/4)=0.7071067811865476,
#       sin(3PI/8)=0.9238795325112867, sin(PI/2)=1.
var nd = alloc(7 * 8);
var ndi = 0;
while (ndi < 7) { store64(nd + ndi * 8, 0); ndi = ndi + 1; }
store64(nd, F64_ONE);                      # x = e_0
assert_eq(num_dst(nd, 7), HSB_ERR_NONE, "num_dst(e_0) ok");
assert(f64_approx_eq(load64(nd + 0), 0x3FD87DE2A6AEA963, EPSILON_F64), "DST-I e0 k0 sin(PI/8)");
assert(f64_approx_eq(load64(nd + 8), 0x3FE6A09E667F3BCC, EPSILON_F64), "DST-I e0 k1 sin(PI/4)");
assert(f64_approx_eq(load64(nd + 16), 0x3FED906BCF328D46, EPSILON_F64), "DST-I e0 k2 sin(3PI/8)");
assert(f64_approx_eq(load64(nd + 24), F64_ONE, EPSILON_F64), "DST-I e0 k3 sin(PI/2)=1");
assert(f64_approx_eq(load64(nd + 32), 0x3FED906BCF328D46, EPSILON_F64), "DST-I e0 k4 sin(5PI/8)");
assert(f64_approx_eq(load64(nd + 40), 0x3FE6A09E667F3BCC, EPSILON_F64), "DST-I e0 k5 sin(3PI/4)");
assert(f64_approx_eq(load64(nd + 48), 0x3FD87DE2A6AEA963, EPSILON_F64), "DST-I e0 k6 sin(7PI/8)");
# x = e_1 -> X[k] = sin(PI (k+1)/4): the signed, zero-crossing column. This is
# what catches a sign flip or a cosine kernel; column 0 above is palindromic.
var nd2 = alloc(7 * 8);
ndi = 0;
while (ndi < 7) { store64(nd2 + ndi * 8, 0); ndi = ndi + 1; }
store64(nd2 + 8, F64_ONE);
assert_eq(num_dst(nd2, 7), HSB_ERR_NONE, "num_dst(e_1) ok");
assert(f64_approx_eq(load64(nd2 + 0), 0x3FE6A09E667F3BCC, EPSILON_F64), "DST-I e1 k0 = +sqrt2/2");
assert(f64_approx_eq(load64(nd2 + 8), F64_ONE, EPSILON_F64), "DST-I e1 k1 = +1");
assert(f64_approx_eq(load64(nd2 + 16), 0x3FE6A09E667F3BCC, EPSILON_F64), "DST-I e1 k2 = +sqrt2/2");
assert(f64_approx_eq(load64(nd2 + 24), 0, EPSILON_F64), "DST-I e1 k3 = sin(PI) = 0");
assert(f64_approx_eq(load64(nd2 + 32), 0xBFE6A09E667F3BCC, EPSILON_F64), "DST-I e1 k4 = -sqrt2/2");
assert(f64_approx_eq(load64(nd2 + 40), f64_neg(F64_ONE), EPSILON_F64), "DST-I e1 k5 = -1");
assert(f64_approx_eq(load64(nd2 + 48), 0xBFE6A09E667F3BCC, EPSILON_F64), "DST-I e1 k6 = -sqrt2/2");

################################################################################
# F. tests/modules.tcyr:1789-1792 — REPLACE the comment line
#    "# LaTeX rendering returns a non-empty string" with the 4 comment lines
#    below, KEEP the three existing lines, and APPEND the nine assertions:
################################################################################
# LaTeX rendering. `!= 0` plus `len > 0` passes for ANY non-empty output --
# including one that drops every \\left(...\\right) group. Pin the strings.
# Integral-valued constants render as bare integers (_latex_fmt_const), a
# convention already pinned by the carry test at the end of this file.
var se_tex = sym_to_latex(expr_add(se_x, expr_const(F64_ONE)));
assert_neq(se_tex, 0, "sym_to_latex returns a Str");
assert(str_len(se_tex) > 0, "sym_to_latex output is non-empty");
assert_eq(str_eq_cstr(se_tex, "x + 1"), 1, "latex: x + 1");
assert_eq(str_eq_cstr(sym_to_latex(expr_add(se_x, expr_neg(expr_const(F64_TWO)))), "x - 2"), 1,
"latex: a + neg(b) folds to a - b");
assert_eq(str_eq_cstr(sym_to_latex(expr_mul(expr_const(F64_TWO), expr_const(f64_from(3)))),
"2 \\cdot 3"), 1, "latex: number*number needs \\cdot");
assert_eq(str_eq_cstr(sym_to_latex(expr_mul(expr_const(F64_TWO),
expr_add(se_x, expr_const(F64_ONE)))), "2 \\left(x + 1\\right)"), 1,
"latex: juxtaposition + parenthesized sum");
assert_eq(str_eq_cstr(sym_to_latex(expr_pow(se_x, expr_const(f64_neg(F64_ONE)))),
"\\frac{1}{x}"), 1, "latex: x^(-1) -> \\frac{1}{x}");
assert_eq(str_eq_cstr(sym_to_latex(expr_pow(se_x, expr_const(F64_HALF))), "\\sqrt{x}"), 1,
"latex: x^(1/2) -> \\sqrt{x}");
assert_eq(str_eq_cstr(sym_to_latex(expr_const(f64_neg(F64_TWO))), "{-2}"), 1,
"latex: bare negative constant is braced");
assert_eq(str_eq_cstr(sym_to_latex(expr_var(str_from("theta"))), "\\mathrm{theta}"), 1,
"latex: multi-char identifier gets \\mathrm");
assert_eq(str_eq_cstr(sym_to_latex(expr_add(expr_mul(expr_const(f64_from(3)),
expr_pow(se_x, expr_const(F64_TWO))), expr_neg(expr_sin(se_x)))),
"3 x^{2} - \\sin\\left(x\\right)"), 1, "latex: 3x^2 - sin(x)");

## NOTES
NO LIBRARY DEFECT FOUND. All six functions compute the correct values; the finding is purely
test-quality. Every replacement assertion passed on the FIRST run against the unmodified library
(35/35 in the first scratch probe), which is itself evidence the expected values were derived
independently and correctly rather than reverse-engineered.

WHAT I ACTUALLY RAN (all from /home/macro/Repos/hisab, library untouched):
  1. scratchpad/probe.tcyr       — first pass, 35 passed / 0 failed
  2. scratchpad/probe2.tcyr      — tightened + extended, 64 passed / 0 failed
  3. scratchpad/hisab_spliced.tcyr      265 passed / 0 failed   (baseline tests/hisab.tcyr:      232)
     scratchpad/foundation_spliced.tcyr 310 passed / 0 failed   (baseline tests/foundation.tcyr: 307)
     scratchpad/modules_spliced.tcyr    537 passed / 0 failed   (baseline tests/modules.tcyr:    512)
     => +61 assertions, 0 regressions. 1127 -> 1188 across the four suites.
  4. cyrius lint on all three spliced files: "0 warnings, 0 untracked deferrals" each.
     (cyrius fmt --check refuses .tcyr — "Usage: cyrfmt --check <file.cyr>" — so the fmt gate
     does not apply to test files.)

MUTATION EVIDENCE (the part that m

## CHALLENGE  vacuous=False compiles=True values_ok=True stands=True
- SURVIVES: eigen_power TRANSPOSED MATVEC (src/linalg_ext.cyr:769 `mat_get(A, i, j)` -> `mat_get(A, j, i)`): SURVIVES. Both proposed matrices ([[2,1],[1,3]] and 3I+J) are SYMMETRIC, so power iteration on A^T is indistinguishable. Verified: 265 passed / 0 failed on the spliced hisab.tcyr. This is the repo's own demonstrated defect class (2.6.13: `svd_golub_kahan returned U transposed`). Detection needs a non-symmetric A whose seed e_0 is not already an eigenvector, e.g. A = [[1,2],[3,4]]: right dominant eigenvector (0.4574271077563381, 1) vs left (0.6861406616345072, 1) -- note A = [[2,1],[0,3]] does NOT work, e_0 is an exact eigenvector of the SUBdominant value there and the clean routine legitimately r
- SURVIVES: eigen_power EIGENVALUE LOSES ITS SIGN (src/linalg_ext.cyr:782 `max_val = wi;` -> `max_val = f64_abs(wi);`): SURVIVES. Both proposed matrices have a positive dominant eigenvalue and an all-positive eigenvector, so the sign of the max-|.| pivot is never exercised. Verified: 265 passed / 0 failed. Detection needs a negative dominant eigenvalue, e.g. B = [[-3,1],[1,-2]], lambda = -(5+sqrt5)/2 = -3.618033988749895, v = (1, -1/phi).
- SURVIVES: opt_gradient_descent CONVERGENCE NORM IS L1 NOT L2 (src/optimize.cyr:58-60 `_opt_norm` rewritten as sum|g_i| instead of sqrt(sum g_i^2)): SURVIVES. In case (b) the y coordinate contracts by 0.5/step while x contracts by 0.9, so at the stopping index |g_y| ~ 1e-42 and |g_x| ~ 9e-7 -- 35 orders apart, and ||g||_1 == ||g||_2 bit-for-bit. Case (a)'s tol is unreachable, so it constrains nothing either. The test's own `gd ||grad(x*)|| < tol` postcondition recomputes L2 on the test side and passes for the same reason. Verified: 265 passed / 0 failed. The very anisotropy that makes (a) strong is what blinds (b) to the norm's form.
- SURVIVES: sym_to_latex MUL PAREN GUARD REMOVED (src/symbolic_ext.cyr:292 and :303 `if (paren == 1)` -> `if (paren == 9)`): SURVIVES. This is the IDENTICAL mutation the proposer demonstrated on the ADD arm (line 275), just moved to the MUL arm -- and it is not caught, because no proposed assertion ever puts a MUL in paren==1 position. `sym_to_latex(expr_mul(x, expr_mul(y, z)))` should render `x \left(y z\right)`; the mutant emits `x y z`. Verified: 537 passed / 0 failed.
- SURVIVES: sym_to_latex EXPR_NEG ARM EMITS '+' (src/symbolic_ext.cyr:338 `str_builder_add_cstr(sb, "-")` -> `"+"`): SURVIVES. Every minus sign in the nine proposed strings comes from ADD's a+neg(b) fold (line 279), which never enters the EXPR_NEG branch. The branch is dead code as far as the new assertions are concerned. `sym_to_latex(expr_mul(expr_neg(x), y))` = `-x y` clean, `+x y` mutated. Verified: 537 passed / 0 failed.
- SURVIVES: sym_to_latex NEG CHILD PAREN ARGUMENT DROPPED (src/symbolic_ext.cyr:339 `_latex_write(sb, expr_child(e), 1)` -> `..., 0`): SURVIVES. `-(x+y) z` renders as `-x + y z` under the mutant -- a mathematically WRONG string. Verified: 537 passed / 0 failed.
- SURVIVES: sym_to_latex SUBTRACTION-FOLD CHILD PAREN DROPPED (src/symbolic_ext.cyr:281 `_latex_write(sb, expr_child(rhs), 1)` -> `..., 0`): SURVIVES, and this is the most serious of the four. `expr_add(x, expr_neg(expr_add(y, z)))` must render `x - \left(y + z\right)`; the mutant emits `x - y + z`, which is a DIFFERENT EXPRESSION. The proposed `3 x^{2} - \sin\left(x\right)` case does not catch it because \sin brings its own parens. Verified: 537 passed / 0 failed.
- SURVIVES: eigen_power RETURNS THE STALE EIGENVALUE (src/linalg_ext.cyr:802 `store64(out_val, new_eigenvalue)` -> `store64(out_val, eigenvalue)`): SURVIVES, but this one I judge NOT a real defect -- at the return point the two differ by less than the routine's own stopping tol (1e-12) and EP_TOL is 1e-11. Listed for completeness only; do not widen the test for it.
- SURVIVES: cga_mv_zero STOPS ZEROING (src/geo_advanced.cyr:953 loop bound 32 -> 0): SURVIVES, because alloc() hands back zeroed pages. The `cga point writes only blades 1..5` loop is therefore vacuous with respect to a non-zeroing CONSTRUCTOR -- but it does have teeth for cga_point itself: an added stray `cga_mv_set(mv, 6, x)` IS caught. Out of scope for this finding; noted so nobody over-credits that assertion.

### CORRECTED
The proposed repair is genuinely strong and should land AS-IS; the following are three ADDITIVE blocks that close the residual holes. All three verified: 277/277 (hisab), 541/541 (modules), 310/310 (foundation) on clean src; `cyrius lint` 0 warnings on all three files; no line > 120 chars. Full hardened files: /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/hisab_hardened.tcyr and .../modules_hardened.tcyr

=== A1. tests/hisab.tcyr — APPEND immediately after
    `assert(f64_approx_eq(load64(ep3_vec + 16), F64_ONE, EP_TOL), "eigen_power 3x3 v[2] = 1");`
    (kills the transpose and sign-loss mutants) ===

# GAP 1: both matrices above are SYMMETRIC, so mat_get(A, i, j) -> mat_get(A, j, i)
# (power iteration on A^T, i.e. the LEFT eigenvector) is invisible to them.
# A = [[1,2],[3,4]]: lambda_1 = (5+sqrt33)/2 = 5.372281323269014.
#   right dominant eigenvector (max-component normalized) = (0.4574271077563381, 1)
#   LEFT  dominant eigenvector (what a transposed matvec yields) = (0.6861406616345072, 1)
var epn = mat_new(2, 2);
mat_set(epn, 0, 0, F64_ONE);       mat_set(epn, 0, 1, F64_TWO);
mat_set(epn, 1, 0, f64_from(3));   mat_set(epn, 1, 1, f64_from(4));
var epn_val = alloc(8);
var epn_vec = alloc(16);
assert_eq(eigen_power(epn, 2, 1000, epn_val, epn_vec), 0, "eigen_power nonsym ok");
assert(f64_approx_eq(load64(epn_val), 0x40157D3750B16878, EP_TOL),
"eigen_power nonsym lambda = (5+sqrt33)/2 = 5.372281323269014");
assert(f64_approx_eq(load64(epn_vec), 0x3FDD467C59078280, EP_TOL),
"eigen_power nonsym v[0] = 0.4574271077563381 (NOT the left eigenvector 0.6861406616)");
assert(f64_approx_eq(load64(epn_vec + 8), F64_ONE, EP_TOL), "eigen_power nonsym v[1] = 1");

# GAP 2: both matrices above have a POSITIVE dominant eigenvalue and a positive
# eigenvector, so `max_val = wi` -> `max_val = f64_abs(wi)` is invisible.
# B = [[-3,1],[1,-2]]: lambda_1 = -(5+sqrt5)/2 = -3.618033988749895, v = (1, -1/phi).
var epb = mat_new(2, 2);
mat_set(epb, 0, 0, f64_neg(f64_from(3)));   mat_set(epb, 0, 1, F64_ONE);
mat_set(epb, 1, 0, F64_ONE);                mat_set(epb, 1, 1, f64_neg(F64_TWO));
var epb_val = alloc(8);
var epb_vec = alloc(16);
assert_eq(eigen_power(epb, 2, 1000, epb_val, epb_vec), 0, "eigen_power negdom ok");
assert(f64_approx_eq(load64(epb_val), 0xC00CF1BBCDCBFA54, EP_TOL),
"eigen_power negdom lambda = -3.618033988749895 (sign must survive)");
assert(f64_approx_eq(load64(epb_vec), F64_ONE, EP_TOL), "eigen_power negdom v[0] = 1");
assert(f64_approx_eq(load64(epb_vec + 8), 0xBFE3C6EF372FE94F, EP_TOL),
"eigen_power negdom v[1] = -1/phi");

=== A2. tests/hisab.tcyr — APPEND immediately after the proposal's last gd line
    `f64_mul(load64(opt_g + 8), load64(opt_g + 8)))), opt_tol6) == 1, "gd ||grad(x*)|| < tol");`
    (kills the L1-vs-L2 norm mutant) ===

# GAP: in (b) the y-coordinate contracts by 0.5 per step while x contracts by 0.9,
# so at the stopping index |g_y| ~ 1e-42 and |g_x| ~ 9e-7 -- the two are 35 orders
# apart and ||g||_1 == ||g||_2 bit-for-bit. The FORM of _opt_norm is therefore
# unconstrained by (a) or (b). Add a third coordinate that contracts at the SAME
# rate as x, so the two norms differ by ~sqrt(2) at the stop and the iteration
# index moves: f(x,y,z) = (x-3)^2 + 5(y+2)^2 + (z-1)^2, x0 = (5,3,4), lr = 1/20.
#   L2 stops at k = 150 -> x = 3 + 2*0.9^150 = 3.000000273782958
#   L1 would stop at k = 153 -> x = 3.0000001995877765  (7.4e-8 apart)
fn _opt_f3(x_ptr) {
    var ax = f64_sub(load64(x_ptr), f64_from(3));
    var ay = f64_add(load64(x_ptr + 8), F64_TWO);
    var az = f64_sub(load64(x_ptr + 16), F64_ONE);
    return f64_add(f64_add(f64_mul(ax, ax), f64_mul(f64_from(5), f64_mul(ay, ay))),
    f64_mul(az, az));
}
fn _opt_grad3(x_ptr, out) {
    store64(out, f64_mul(F64_TWO, f64_sub(load64(x_ptr), f64_from(3))));
    store64(out + 8, f64_mul(f64_from(10), f64_add(load64(x_ptr + 8), F64_TWO)));
    store64(out + 16, f64_mul(F64_TWO, f64_sub(load64(x_ptr + 16), F64_ONE)));
    return 0;
}
var opt3_x0 = alloc(24);
store64(opt3_x0, f64_from(5));
store64(opt3_x0 + 8, f64_from(3));
store64(opt3_x0 + 16, f64_from(4));
var opt3_x = alloc(24);
var opt3_f = alloc(8);
assert_eq(opt_gradient_descent(&_opt_f3, &_opt_grad3, opt3_x0, 3, opt_lr, opt_tol6, 1000,
opt3_x, opt3_f), HSB_ERR_NONE, "gd 3d converged");
assert(f64_approx_eq(load64(opt3_x), 0x4008000024BF1C5E, EPSILON_F64),
"gd 3d x* = 3 + 2*0.9^150 (L2 stop at k=150, an L1 norm would stop at 153)");
assert(f64_approx_eq(load64(opt3_x + 16), 0x3FF000006E3D551B, EPSILON_F64),
"gd 3d z* = 1 + 3*0.9^150");
assert_eq(load64(opt3_x + 8), 0xC000000000000000, "gd 3d y* = -2 exactly");

=== A3. tests/modules.tcyr — APPEND immediately after the proposal's last latex line
    `"3 x^{2} - \\sin\\left(x\\right)"), 1, "latex: 3x^2 - sin(x)");`
    (kills all four surviving latex mutants) ===

# GAP: every assertion above renders MUL and NEG only in paren==0 position, so
# the MUL \left(...\right) guard and the whole EXPR_NEG branch are unreached.
# `if (paren == 1)` -> `if (paren == 9)` in the MUL arm, "-" -> "+" in the NEG
# arm, and dropping the paren=1 argument on either child all survive otherwise.
var se_y = expr_var(str_from("y"));
var se_z = expr_var(str_from("z"));
assert_eq(str_eq_cstr(sym_to_latex(expr_mul(se_x, expr_mul(se_y, se_z))),
"x \\left(y z\\right)"), 1, "latex: a MUL operand of a MUL is parenthesized");
assert_eq(str_eq_cstr(sym_to_latex(expr_mul(expr_neg(se_x), se_y)), "-x y"), 1,
"latex: the EXPR_NEG arm itself emits a minus");
assert_eq(str_eq_cstr(sym_to_latex(expr_mul(expr_neg(expr_add(se_x, se_y)), se_z)),
"-\\left(x + y\\right) z"), 1, "latex: NEG of a sum keeps its parens");
assert_eq(str_eq_cstr(sym_to_latex(expr_add(se_x, expr_neg(expr_add(se_y, se_z)))),
"x - \\left(y + z\\right)"), 1, "latex: a - (b + c), not the wrong a - b + c");

=== EVIDENCE ===
Mutation results on the AS-PROPOSED suites (32 valid mutants, 24 caught):
  eigen_power  CAUGHT: loose stopping tol (the original demo), max-search i<n-1, matvec j<n-1.
               SURVIVED: transpose, abs-eigenvalue, stale-eigenvalue.
  cga_point    CAUGHT: ep sign flip (demo), p^2 drops z, ep/em swap, stray write to blade 6.
               SURVIVED: cga_mv_zero non-zeroing (out of scope; alloc returns zeroed pages).
  gd           CAUGHT: half step (demo), iter<=max_iter, out_fmin at x0, single-coordinate
               update, tol x10.  SURVIVED: _opt_norm L1.
  hvec4_length CAUGHT: 1.5 scale (demo), w term dropped, L1 norm.  SURVIVED: none.
               (`f64v_dot(v,v,3)` is NOT a valid mutant -- the SIMD helper over-reads to 4 lanes.)
  num_dst      CAUGHT: sign flip (demo), (i+1)->i, n+1->n, cos kernel, k<n-1.  SURVIVED: none.
               This block is the strongest of the six.
  sym_to_latex CAUGHT: ADD paren guard (demo), \cdot suppressed, \mathrm threshold.
               SURVIVED: MUL paren guard, NEG arm sign, NEG child paren, sub-fold child paren.
After applying A1+A2+A3 all seven meaningful survivors are CAUGHT, and every original
demo mutant remains caught.

Constants: all 22 hex f64 literals re-derived with python3 struct and confirmed byte-exact
(sqrt(30), sqrt(15.5), (5+sqrt5)/2, 1/phi, 6.5/7.5/2.75/3.75, 1e-6, 1e-11, the DST-I
sin(k*PI/8) column, and the GD trajectory). The GD closed form and an independent FP replay
of the documented spec agree to 1 ulp (x_10, y_10, f_10 exact; x* differs by 1 ulp = 4.4e-16,
far inside EPSILON_F64). Three cosmetic COMMENT drifts, no assertion impact: the eigen v[0]
comment says 0.6180339887498949 but 0x3FE3C6EF372FE94F is ...948; the gd f_10 comment says
0.4864258276518282 but 0x3FDF2199CB6C6FF8 is ...807 (the constant matches the FP replay, the
comment matches the closed form); the DST comment says sin(PI/4)=0.7071067811865476 but
0x3FE6A09E667F3BCC is ...475.

Cyrius correctness: compiles and passes on cycc 6.5.6 (265/265 hisab, 310/310 foundation,
537/537 modules as proposed). No `f64_from(0x...)`, no integer comparison on an f64 bit
pattern, no `else if` (plain `else` is fine and compiles), no bare negative literals, no
reserved iv_add/iv_sub/iv_mul identifiers, no line over 120 chars, `cyrius lint` reports
0 warnings on all three files. `f64_approx_eq` returns 1/0 so bare `assert(f64_approx_eq(...))`
is valid. The redefined `_opt_f`/`_opt_grad` are not referenced by any other test in the file.