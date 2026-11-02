# Migrating to hisab 3.4.0

> 3.4.0 retires **27 public functions** that answered with a plausible value where they had
> none: the identity for a singular matrix, 0 for an fBm with no octaves, the last Newton
> iterate for an implicit step that did not converge, a bare integer status that read as an
> unrelated `HSB_ERR_*`. Each is replaced: 26 by a `_checked` form that returns a `Result`,
> and `m3_normal_matrix` by `m3_normal_matrix_cof`, whose answer has a different magnitude.
> The old names are **removed, not redefined**, so a consumer that still calls one does not
> build. No other public name or parameter list changes. Several functions keep their names
> and answer differently, almost always where they were wrong. Those are listed at the end.

## TL;DR

```cyrius
# 3.3
var inv = m3_inverse(m);                          # the identity when m is singular
var y1  = ode_bdf2(&f, t, y, y_prev, dt, tol);    # the last iterate when Newton fails

# 3.4
var it, inv = m3_inverse_checked(m);              # the answer is the payload
if (is_err_result(it) == 1) { … }                 # inv holds HSB_ERR_SINGULAR_MATRIX / _INVALID_INPUT / _ALLOC
var y1: i64[1];
var bt, be = ode_bdf2_checked(&f, t, y, y_prev, dt, tol, &y1);   # the answer goes to a new last `out`
if (is_err_result(bt) == 1) { … }                 # be holds the code; y1 is untouched
var y_next = load64(&y1);
```

Every replacement is `#must_use`. `result` is already a stdlib dependency, as it has been since
3.0.0: `dist/hisab.deps` still names the same seven leaves, so `cyrius deps` needs nothing new.

⚠ **Read [Toolchain](#toolchain) before you move `[deps.hisab] tag`.** hisab itself now pins
cyrius 6.7.9. The consumer-pin results for the 3.4.0 bundle are still to be filled in there.

## ⛔ The one thing that will bite you

**A `Result` in argument position does not error: the callee receives only the tag.**

This is the rule from [migration-3.0.md](migration-3.0.md), and it is worse here. Thirteen of
the replacements carry the answer itself in the payload: a matrix, a vector, a quaternion, a
scalar. Nest one, as the retired names often were (`m3_normal_matrix` itself was
`m3_transpose(m3_inverse(m))`), and the payload never arrives. The callee gets **0** for `Ok`
and **1** for `Err`:

- **A scalar argument sees +0.0 on success and 2^-1074 on failure.** Bits 1 are the smallest
  subnormal, a legal number.
- **A handle argument sees address 0 or 1.** A function that refuses a 0 handle returns its own
  designed 0, so the failure travels on silently. The rest read address 0 or 1 and fault.

Measured on cycc 6.7.9 against a bundle regenerated from the 3.4.0 tree (scratch probes,
2026-10-31):

```cyrius
var a = f64_from(2); var b = f64_from(4); var x = f64_from(3);   # the answer is 0.5
var s = f64_mul(hisab_inverse_lerp_checked(a, b, x), F64_ONE);   # ⛔ +0.0 (bits 0), not 0.5
var e = f64_mul(hisab_inverse_lerp_checked(a, a, a), F64_ONE);   # ⛔ bits 1 (2^-1074), not an error
var r = hquat_rotate_vec3(hquat_normalize_checked(q), v);        # ⛔ exit 139
var lt, lv = hisab_inverse_lerp_checked(a, b, x);                # right: lt = 0 (Ok), lv = 0.5
```

**None of the three ⛔ lines draws a diagnostic.** The build is clean, and `#must_use` stays
silent, because a value in argument position counts as used.

A comparison against 0 is then true for every success, whatever the answer was:
`assert_eq(expr_eval_checked(e, vars), 0, "…")` passes for an `e` that evaluates to 5.

The **binding** site is safe. `var inv = m3_inverse_checked(m);`, a single-variable bind of a
`Result`, is a hard compile error that names the fix (documented in `lib/result.cyr`). So a
mechanical rename of `var x = f(…);` lines fails loudly, and a rename of nested calls compiles.

⭐ **Drive the migration with a grep, not with build errors.** The build finds every reachable
call of a retired name. It cannot find a `_checked` call you have put in argument position. For
each replacement you adopt, find every call site and give it a binding:
`var t, v = f(…);`, or `var v = f(…)?;` inside a function that itself returns a `Result`.
`scripts/check-result-migration.sh` in this repo does exactly that for hisab's own tree. To use
it on yours, copy it and add your vendored `lib/hisab.cyr` to the glob it reads definitions from
(it reads `src/*.cyr`).

## Does this affect me?

On 2026-10-31 we searched the sources of the 14 repos that carry a hisab bundle (every
`.cyr`, `.tcyr`, `.bcyr` and `.fcyr` outside `lib/`, `dist/`, `build/` and `rust-old/`,
comment lines excluded). **One** calls a retired name: svara, `src/prosody.cyr:115`, `calc_monotone_cubic`.
Its migration is the worked example under [calc](#calc--calc_ext--noise_simplex).

Thirteen of those 14 are still on hisab 2.x (2.11.2 or 2.22.1), and one is on 3.2.1. Moving to
3.4.0 also means [migration-3.0.md](migration-3.0.md) (`Result` for 48 functions) and
[migration-3.3.md](migration-3.3.md) (the private surface). Read them first.

You are affected if you:

- call any of the 27 names below;
- nest `m3_inverse(…)`, `hquat_normalize(…)` or a similar call inside another call (see above);
- pass a non-unit quaternion to a rotation, rely on `f64_fmod(x, 0) == 0`, call `num_bisection`
  with a small `max_iter`, or pin any value listed under
  [Behaviour that changed](#behaviour-that-changed-under-an-unchanged-name).

## The four call shapes

| retired (3.3.7 return) | replacement | the answer arrives | Err codes |
|---|---|---|---|
| `svd_compute(A, out_U, out_S, out_Vt)` → 0 / −1 / −2 | `svd_compute_checked(A, out_U, out_S, out_Vt)` | the out-params, as before | INVALID_INPUT, ALLOC, NO_CONVERGENCE |
| `solve_gmres(A_fn, b, x0, n, tol, max_iter, restart)` → x | `solve_gmres_checked(…)`, same parameters, **tol relative** | payload: x | INVALID_INPUT, ALLOC, NO_CONVERGENCE |
| `solve_bicgstab(A_fn, b, x0, n, tol, max_iter)` → x | `solve_bicgstab_checked(…)`, same parameters, **tol relative** | payload: x | INVALID_INPUT, ALLOC, NO_CONVERGENCE |
| `solve_pgs(A, b, x, n, lo, hi, max_iter)` → 0 | `solve_pgs_checked(…)`, same parameters | `x`, in place, as before | INVALID_INPUT, SINGULAR_PIVOT, NO_CONVERGENCE, ALLOC |
| `calc_adaptive_simpson(f, a, b, tol)` → f64 | `calc_adaptive_simpson_checked(f, a, b, tol, out)` | `out` (on NO_CONVERGENCE too) | INVALID_INPUT, NO_CONVERGENCE |
| `calc_partial_derivative(f, x, n, index, h)` → f64 | `calc_partial_derivative_checked(f, x, n, index, h, out)` | `out` | INVALID_INPUT, ALLOC |
| `calc_hermite_tcb(times, values, n, tension, continuity, bias, t)` → f64 | `calc_hermite_tcb_checked(…, t, out)` | `out` | INVALID_INPUT |
| `calc_monotone_cubic(xs, ys, n, x)` → f64 | `calc_monotone_cubic_checked(xs, ys, n, x, out)` | `out` | OUT_OF_RANGE, INVALID_INPUT, ALLOC |
| `fbm_2d(x, y, octaves, freq, amp, persistence)` → f64 | `fbm_2d_checked(…, persistence, out)` | `out` | INVALID_INPUT, DIVISION_BY_ZERO |
| `fbm_3d(x, y, z, octaves, freq, amp, persistence)` → f64 | `fbm_3d_checked(…, persistence, out)` | `out` | INVALID_INPUT, DIVISION_BY_ZERO |
| `simplex_fbm_2d(x, y, octaves, freq, amp, persistence)` → f64 | `simplex_fbm_2d_checked(…, persistence, out)` | `out` | INVALID_INPUT, DIVISION_BY_ZERO |
| `simplex_fbm_3d(x, y, z, octaves, freq, amp, persistence)` → f64 | `simplex_fbm_3d_checked(…, persistence, out)` | `out` | INVALID_INPUT, DIVISION_BY_ZERO |
| `ode_backward_euler(f, t, y, dt, tol, max_newton)` → f64 | `ode_backward_euler_checked(…, max_newton, out)` | `out` | INVALID_INPUT, NO_CONVERGENCE, SINGULAR_MATRIX |
| `ode_bdf2(f, t, y, y_prev, dt, tol)` → f64 | `ode_bdf2_checked(…, tol, out)` | `out` | INVALID_INPUT, NO_CONVERGENCE, SINGULAR_MATRIX |
| `ode_bdf(f, t, history, order, dt, tol)` → f64 | `ode_bdf_checked(…, tol, out)` | `out` | INVALID_INPUT, NO_CONVERGENCE, SINGULAR_MATRIX |
| `num_pollard_rho(n)` → i64 | `num_pollard_rho_checked(n)` | payload | INVALID_INPUT, NO_CONVERGENCE |
| `expr_eval(e, vars)` → f64 | `expr_eval_checked(e, vars)` | payload | INVALID_INPUT |
| `m3_inverse(m)` → Mat3 | `m3_inverse_checked(m)` | payload | SINGULAR_MATRIX, INVALID_INPUT, ALLOC |
| `m4_inverse(m)` → Mat4 | `m4_inverse_checked(m)` | payload | SINGULAR_MATRIX, INVALID_INPUT, ALLOC |
| `t3d_inverse_matrix(t)` → Mat4 | `t3d_inverse_matrix_checked(t)` | payload | SINGULAR_MATRIX, INVALID_INPUT, ALLOC |
| `m4_transform_point(m, v)` → Vec3 | `m4_transform_point_checked(m, v)` | payload | DIVISION_BY_ZERO, INVALID_INPUT, ALLOC |
| `world_to_screen(point, mvp, vw, vh)` → Vec3 | `world_to_screen_checked(point, mvp, vw, vh)` | payload | DIVISION_BY_ZERO, INVALID_INPUT, ALLOC |
| `hquat_inverse(q)` → Quat | `hquat_inverse_checked(q)` | payload | DIVISION_BY_ZERO, INVALID_INPUT, ALLOC |
| `hquat_normalize(q)` → Quat | `hquat_normalize_checked(q)` | payload | DIVISION_BY_ZERO, INVALID_INPUT, ALLOC |
| `hisab_inverse_lerp(a, b, value)` → f64 | `hisab_inverse_lerp_checked(a, b, value)` | payload | DIVISION_BY_ZERO, INVALID_INPUT |
| `hisab_remap(value, in_lo, in_hi, out_lo, out_hi)` → f64 | `hisab_remap_checked(…)` | payload | DIVISION_BY_ZERO, INVALID_INPUT |
| `m3_normal_matrix(m)` → M^-T | `m3_normal_matrix_cof(m)` → sign(det M)·cof(M) | the return value (no `Result`) | none (0 on a refused allocation) |

That is four shapes:

| shape | how to call it |
|---|---|
| **payload** (13): the old return value is now the `Ok` payload | `var t, v = f(…); if (is_err_result(t) == 1) { /* v is the code */ }` |
| **`out`** (11): a new last parameter receives the value, and the `Result` is `Ok(0)` / `Err(code)` | `var slot: i64[1]; var t, e = f(…, &slot); … load64(&slot)` |
| **status** (2): `svd_compute`, `solve_pgs`: same out-params, the integer status becomes a `Result` | `var t, e = f(…); if (is_err_result(t) == 1) { … }` |
| **renamed** (1): `m3_normal_matrix_cof`, cannot fail | `var n = m3_normal_matrix_cof(m);`, then renormalise what it transforms |

⚠ **`out` is not null-checked.** Pass 8 writable bytes: `var slot: i64[1]` with `&slot`, or
`alloc(8)`. On every `Err`, `out` is left untouched, except `calc_adaptive_simpson_checked`'s
NO_CONVERGENCE, which writes its best estimate there. Read `out` only after testing the tag.

## linalg_ext

### `svd_compute` → `svd_compute_checked`

`svd_compute(A, out_U, out_S, out_Vt)` returned ganita's status vocabulary as a bare integer:
0, −1 or −2. Those values are bit-identical to `HSB_ERR_INVALID_TRANSFORM` and
`HSB_ERR_SINGULAR_MATRIX`, so a caller comparing against the hisab codes read a wide matrix as
"singular". −1 covered an allocation failure, a fallback that did not converge, and a singular
value proven past DBL_MAX, with nothing to tell them apart.

`svd_compute_checked(A, out_U, out_S, out_Vt)` takes the same arguments and returns `Ok(0)`, with
the decomposition in the out-params as before. The errors:

- **`Err(HSB_ERR_INVALID_INPUT)`** (was −2), refused before anything is allocated or written:
  - a null `A`, `out_U`, `out_S` or `out_Vt`;
  - m ≤ 0 or n ≤ 0;
  - a wide `A` (m < n);
  - an `out_U` that is not exactly m × n, or an `out_Vt` that is not exactly n × n;
  - a NaN or infinite entry in `A`.
- **`Err(HSB_ERR_ALLOC)`** (was −1): its n-slot scratch cannot be allocated, or ganita gives no
  answer and the `svd_golub_kahan` fallback cannot allocate either.
- **`Err(HSB_ERR_NO_CONVERGENCE)`** (was −1): ganita gives no answer and the fallback does not
  converge, which includes a singular value above DBL_MAX. Or ganita's answer has a singular
  value that its own singular pair proves past the rounding line.

On any `Err`, `out_S` is untouched. On ALLOC and NO_CONVERGENCE, `out_U` and `out_Vt` may hold
partial work.

```cyrius
# 3.3
var rc = svd_compute(A, U, S, Vt);
if (rc != 0) { … }                          # -1 or -2, ganita's codes

# 3.4
var st, se = svd_compute_checked(A, U, S, Vt);
if (is_err_result(st) == 1) { … }           # se: HSB_ERR_INVALID_INPUT / _ALLOC / _NO_CONVERGENCE
```

### `solve_gmres` / `solve_bicgstab` → `solve_gmres_checked` / `solve_bicgstab_checked`

The retired solvers returned the current iterate `x` on **every** exit, with no report. That
included the end of the budget, every breakdown, GMRES's Givens estimate taken on trust, and,
for BiCGSTAB, an all-NaN `x` or a half step. Their `tol` was **absolute** (‖r‖ < tol). A
refused allocation returned a 0 pointer, and n ≤ 0 crashed.

```
solve_gmres_checked(A_fn, b, x0, n, tol, max_iter, restart)  → Result
solve_bicgstab_checked(A_fn, b, x0, n, tol, max_iter)        → Result
```

The parameters are unchanged in name and order, but **`tol` is relative now**. `Ok(x)` comes
back, `x` a fresh flat array of n f64, only when the **true residual** b − A x, formed with
`A_fn`, has ‖b − A x‖₂ ≤ tol·‖b‖₂. For `b = 0`, x = 0 comes back at once, with no matvec. `tol`
must be finite and ≥ 0. `tol = 0` accepts only an exactly zero residual.

The errors (the payload is then the code, **not a vector**):

- **`Err(HSB_ERR_INVALID_INPUT)`**, before any work: a null `A_fn`, `b` or `x0`; n ≤ 0;
  max_iter ≤ 0; a `tol` that is negative, NaN or +Inf; a NaN or infinite entry in `b` or `x0`.
- **`Err(HSB_ERR_ALLOC)`**: n past `ALLOC_MAX / 8`; GMRES's (m + 1)·m-word Hessenberg past
  `ALLOC_MAX`; any working buffer refused.
- **`Err(HSB_ERR_NO_CONVERGENCE)`** in each of these cases:
  - the budget ran out;
  - a breakdown. For GMRES, a restart cycle that leaves x bit for bit unchanged. For BiCGSTAB,
    r̂·r, r̂·Ap or ω exactly 0, or a t = A s numerically orthogonal to s;
  - a residual that stops being finite;
  - a probe `A_fn(b)` that is not finite even at b·2^-600;
  - a solution past DBL_MAX.

**Budgets.** For GMRES, `max_iter` still counts matvecs. The probe that fixes the solver's
scaling counts as one, and an `x0` of 0 spends none on b − A x0. Confirming a Givens estimate
costs one matvec more than 3.3.7 spent. For BiCGSTAB, `max_iter` counts iterations (two matvecs
each). The probe, the initial residual and each confirmation are not counted against it.
`restart` is still clamped to [1, n].

```cyrius
# 3.3: absolute tol, x whatever happened
var x = solve_gmres(&mv, b, x0, n, tol_abs, 200, 16);

# 3.4: relative tol; x only on Ok
var gt, gx = solve_gmres_checked(&mv, b, x0, n, f64_div(tol_abs, bnorm), 200, 16);   # bnorm = ||b||_2
if (is_err_result(gt) == 1) { … }           # gx is HSB_ERR_*, not a vector
```

Dividing by ‖b‖₂ reproduces 3.3.7's stopping threshold. Because acceptance is now on the true
residual, a run can still stop on a different pass. A system scaled by powers of two,
(2^i A, 2^j b, 2^(j−i) x0), now gives the scaled answer bit for bit wherever it is representable.

### `solve_pgs` → `solve_pgs_checked`

`solve_pgs(A, b, x, n, lo, hi, max_iter)` returned **0 for every outcome**: an unconverged stop,
a zero-diagonal row skipped with the caller's guess left in `x_i`, and non-finite input that
reached `x` or was clamped away. It also mishandled an `x` passed as `b`: it overwrote `b` while
still reading it, and returned 0 with roughly (−3.7e-8, 2.4e-8, 6.7e-8) where the solution is
(1, 2, 4). n ≤ 0 or max_iter ≤ 0 returned 0 with nothing done.

`solve_pgs_checked(A, b, x, n, lo, hi, max_iter)` takes the same parameters, and the solution is
written into `x`, as before. It returns `Ok(0)` when a sweep moves no x_i by more than 1e-12 of
max|x| (DBL_MIN when x is 0). Every x_i is then finite. The errors:

- **`Err(HSB_ERR_INVALID_INPUT)`**, before `x` is written:
  - a null `A`, `b`, `x`, `lo` or `hi`;
  - n ≤ 0 or max_iter ≤ 0;
  - an `A` smaller than n × n;
  - a NaN or infinite entry in A's n × n block, in `b` or in `x`;
  - a NaN bound, lo_i = +Inf, hi_i = −Inf, or lo_i > hi_i.
- **`Err(HSB_ERR_SINGULAR_PIVOT)`**, before `x` is written: a diagonal entry that is exactly 0,
  of either sign. It used to be skipped silently.
- **`Err(HSB_ERR_NO_CONVERGENCE)`**: `max_iter` sweeps ended without the stopping test. `x`
  holds the last sweep, which is not a solution.
- **`Err(HSB_ERR_ALLOC)`**: the private copies of `b`, `lo` and `hi` cannot be allocated.

`x` may now share storage with `b`, `lo` or `hi`. It must not overlap `A`.

```cyrius
# 3.3
solve_pgs(A, b, x, n, lo, hi, 100);         # 0 whatever happened

# 3.4
var pt, pe = solve_pgs_checked(A, b, x, n, lo, hi, 100);
if (is_err_result(pt) == 1) { … }           # NO_CONVERGENCE: x is the last sweep, not a solution
```

## calc / calc_ext / noise_simplex

All eight replacements take the value through a new last parameter, `out`, and return `Ok(0)`
or `Err(code)`.

### `calc_adaptive_simpson` → `calc_adaptive_simpson_checked`

`calc_adaptive_simpson(f, a, b, tol)` returned the estimate with no status. It returned NaN,
evaluating nothing, for tol ≤ 0 or NaN, and NaN for a tolerance that halved to zero part-way
down. A walk that could not meet its tolerance kept going: 5x⁴ − 3x² + x − 1/4 at 1e-20 passed
3,000,000 evaluations.

`calc_adaptive_simpson_checked(f, a, b, tol, out)` returns:

- **`Ok(0)`**, the integral in `out`, when E, the sum of the error estimates of the panels the
  result is built from, plus half an ulp of the double reported, is at most `tol`. An integral
  that is ±Inf from finite samples (one past the range) is Ok too. A zero-width interval is
  `Ok`, +0, without calling `f`.
- **`Err(HSB_ERR_INVALID_INPUT)`** for tol ≤ 0 or NaN. This is checked before `f` is called, and
  `out` is untouched.
- **`Err(HSB_ERR_NO_CONVERGENCE)`**, with the **best estimate in `out`**, in each of these cases:
  - E plus that half ulp passes `tol`. This includes any `tol` below half an ulp of the result,
    as QUADPACK's ier = 2.
  - Some panel could not be refined to its share of `tol`, because of:
    - the depth cap of 50 levels;
    - a width too narrow to bisect;
    - a tolerance that halves to zero;
    - the backstop of 2^25 evaluations per pass.
  - A sample was infinite or NaN (a NaN result included).

⚠ Ok rests on Richardson estimates, not on a bound. The function's doc block lists the measured
integrand classes where an Ok answer is off by more than `tol`.

```cyrius
# 3.3
var r = calc_adaptive_simpson(&f, a, b, tol);        # NaN for tol <= 0; no status

# 3.4
var r: i64[1];
var at, ae = calc_adaptive_simpson_checked(&f, a, b, tol, &r);
if (is_err_result(at) == 1) {
    # ae == HSB_ERR_NO_CONVERGENCE: load64(&r) is the best estimate, not certified
    # ae == HSB_ERR_INVALID_INPUT:  r untouched
}
```

### `calc_partial_derivative` → `calc_partial_derivative_checked`

`calc_partial_derivative(f, x, n, index, h)` returned 0 for n ≤ 0, for an index outside [0, n),
and when its scratch could not be allocated. 0 is the derivative along every constant direction.

`calc_partial_derivative_checked(f, x, n, index, h, out)` returns `Ok(0)` with the derivative in
`out`. It returns **`Err(HSB_ERR_INVALID_INPUT)`** for n ≤ 0 or an index outside [0, n), and
**`Err(HSB_ERR_ALLOC)`** when its two n-element scratch arrays cannot be allocated. On both,
`out` is untouched and `f` is not called.

```cyrius
# 3.3
var d = calc_partial_derivative(&f, x, n, i, h);
# 3.4
var d: i64[1];
var dt, de = calc_partial_derivative_checked(&f, x, n, i, h, &d);
```

### `calc_hermite_tcb` → `calc_hermite_tcb_checked`

`calc_hermite_tcb(times, values, n, tension, continuity, bias, t)` returned 0 for n < 2.

`calc_hermite_tcb_checked(times, values, n, tension, continuity, bias, t, out)` returns `Ok(0)`
with the value. One key (n = 1) is the constant `values[0]`, as both end clamps give it. It
returns **`Err(HSB_ERR_INVALID_INPUT)`** for n < 1, with `out` untouched. Key times are still
not checked for order. Keys spaced below DBL_MIN are now interpolated: 3.3.7 returned the
segment's first key there.

```cyrius
# 3.3
var v = calc_hermite_tcb(times, vals, n, ten, con, bias, tq);
# 3.4
var v: i64[1];
var ht, he = calc_hermite_tcb_checked(times, vals, n, ten, con, bias, tq, &v);
```

### `calc_monotone_cubic` → `calc_monotone_cubic_checked`

`calc_monotone_cubic(xs, ys, n, x)` returned 0, a legal y, in each of these cases:

- n < 2;
- x outside [xs[0], xs[n−1]];
- abscissae that are not strictly increasing;
- scratch that could not be allocated.

`calc_monotone_cubic_checked(xs, ys, n, x, out)` returns `Ok(0)` with the value. A NaN query or
ordinate still gives NaN, as Ok. One point (n = 1) is `ys[0]` at x = xs[0]. The errors, each
with `out` untouched:

- **`Err(HSB_ERR_OUT_OF_RANGE)`**: x outside [xs[0], xs[n−1]] (for n = 1, any x ≠ xs[0]).
- **`Err(HSB_ERR_INVALID_INPUT)`**: n < 1, or abscissae that are not strictly increasing. A NaN
  abscissa or a repeated infinite one counts as not increasing.
- **`Err(HSB_ERR_ALLOC)`**: its scratch arrays cannot be allocated.

svara, `src/prosody.cyr:115`, the one consumer call site found:

```cyrius
# 3.3
var r = calc_monotone_cubic(xs, ys, n, tc);
if (r == 0) { return F64_ONE; }                       # 0 meant "refused" -- and also a real 0
return r;

# 3.4
var r: i64[1];
var mt, me = calc_monotone_cubic_checked(xs, ys, n, tc, &r);
if (is_err_result(mt) == 1) { return F64_ONE; }       # OUT_OF_RANGE / INVALID_INPUT / ALLOC
return load64(&r);                                    # a genuine 0 now comes back as 0
```

Note the last line. Through 3.3.7 that code turned an interpolant that was really 0 at `tc` into
1.0.

### `fbm_2d`, `fbm_3d`, `simplex_fbm_2d`, `simplex_fbm_3d` → `…_checked`

```
fbm_2d_checked(x, y, octaves, freq, amp, persistence, out)            # calc
fbm_3d_checked(x, y, z, octaves, freq, amp, persistence, out)         # calc_ext
simplex_fbm_2d_checked(x, y, octaves, freq, amp, persistence, out)    # noise_simplex
simplex_fbm_3d_checked(x, y, z, octaves, freq, amp, persistence, out) # noise_simplex
```

All four retired forms returned 0, a legal noise value, whatever the coordinates, in three
cases: octaves ≤ 0, amp = 0, and weights that sum to 0.

The replacements return `Ok(0)` with the octaves' weighted mean Σaᵢnᵢ / Σaᵢ, aᵢ = amp·pⁱ, in
`out`. The errors, each with `out` untouched:

- **`Err(HSB_ERR_INVALID_INPUT)`**: octaves ≤ 0. There is no octave, so there is no mean.
- **`Err(HSB_ERR_DIVISION_BY_ZERO)`**: the weights sum to 0 (persistence = −1 with an even
  octave count).

**amp = 0 is not an error.** The mean does not depend on amp, so amp = 0 returns the value
amp = 1 gives. So does an amp whose weights leave the range: ±Inf, a subnormal amp, or weights
that overflow. A NaN amp still gives NaN. Every other amp keeps its 3.3.7 bits. The simplex
forms still return NaN, as Ok, where an octave's scaled coordinates pass simplex noise's reach.

```cyrius
# 3.3
var v = fbm_2d(x, y, oct, freq, amp, pers);            # 0 when refused
# 3.4
var v: i64[1];
var ft, fe = fbm_2d_checked(x, y, oct, freq, amp, pers, &v);
if (is_err_result(ft) == 1) { … }                      # fe: HSB_ERR_INVALID_INPUT / _DIVISION_BY_ZERO
```

## ode

```
ode_backward_euler_checked(f, t, y, dt, tol, max_newton, out)
ode_bdf2_checked(f, t, y, y_prev, dt, tol, out)
ode_bdf_checked(f, t, history, order, dt, tol, out)
```

The retired steppers returned the **last Newton iterate** whenever Newton failed, and never
tested the residual of the iterate their last update produced. Three examples:

- **`ode_backward_euler`** on y′ = −y, dt = 1, max_newton 0 returned the predictor 0, where the
  step's value is 0.5.
- On y′ = y with dt = 1, a step that does not exist, it returned 773,325.
- **`ode_bdf`** returned 0 for an order outside 2..5 or a history shorter than the order.

Each replacement returns `Ok(0)`, y_{n+1} in `out`, when an iterate's residual |g(z)| is below
`tol`. The predictor is tested first, and so is the iterate the last update made. The errors,
each with `out` untouched:

- **`Err(HSB_ERR_INVALID_INPUT)`**, before `f` is called, for:
  - tol ≤ 0 or NaN;
  - `ode_backward_euler_checked`: max_newton < 0;
  - `ode_bdf_checked`: an order outside 2..5, or a history shorter than the order.
- **`Err(HSB_ERR_NO_CONVERGENCE)`**: the updates ran out. That is `max_newton` updates for
  backward Euler and 50 for BDF. An `f` that returns NaN ends here.
- **`Err(HSB_ERR_SINGULAR_MATRIX)`**: the Newton slope g′(z) is not resolved from 0
  (|g′| ≤ 2^-24·(1 + |c·f_y|)), so the step's equation is singular. y′ = y at dt = 1 is one.
  3.3.7 returned an iterate there.

`max_newton = 0` now means "accept the predictor or report". It used to return the predictor
regardless. The Newton slope's difference step now scales with the state (2^-26·|z|), so `Ok`
values can differ from 3.3.7's in the last bits.

```cyrius
# 3.3
var y1 = ode_backward_euler(&f, t, y, dt, tol, 20);
var y2 = ode_bdf(&f, t, hist, order, dt, tol);         # 0 for a bad order

# 3.4
var y1: i64[1];
var et, ee = ode_backward_euler_checked(&f, t, y, dt, tol, 20, &y1);
if (is_err_result(et) == 1) { … }       # ee: NO_CONVERGENCE / SINGULAR_MATRIX / INVALID_INPUT
var y2: i64[1];
var kt, ke = ode_bdf_checked(&f, t, hist, order, dt, tol, &y2);
```

## num_ext

### `num_pollard_rho` → `num_pollard_rho_checked`

`num_pollard_rho(n)` returned `n` for every n ≤ 1. n is the answer that means "n is prime", so
1, 0 and every negative n read as primes. It also returned n for a composite it could not split.

`num_pollard_rho_checked(n)` returns:

- **`Ok(d)`** with 1 < d < n and d | n, for a composite n;
- **`Ok(n)`** exactly when n is prime;
- **`Err(HSB_ERR_INVALID_INPUT)`** for n ≤ 1 (i64 min included);
- **`Err(HSB_ERR_NO_CONVERGENCE)`** if every rho sequence fails to split a composite. No odd
  composite measured reaches this.

For n > 1 the payload equals 3.3.7's return bit for bit.

```cyrius
# 3.3
var d = num_pollard_rho(n);
# 3.4
var rt, d = num_pollard_rho_checked(n);
if (is_err_result(rt) == 1) { … }       # n <= 1
```

## symbolic

### `expr_eval` → `expr_eval_checked`

`expr_eval(e, vars)` returned +0.0 for an unbound variable or an unknown tag, so x + 1 with x
unbound evaluated to 1.0. It also wrote a warning to **fd 2**, the only I/O hisab's library code
did. A 0 node was read at address 0.

`expr_eval_checked(e, vars)` returns **`Ok(value)`**, the same bits `expr_eval` gave every bound
tree. It returns **`Err(HSB_ERR_INVALID_INPUT)`** in three cases:

- a variable with no binding in `vars`;
- a 0 node (the failure handle every `expr_*` constructor now returns);
- a tag no constructor writes.

It writes nothing to any fd. A `vars` of 0 is read as no bindings, and a 0 pair is skipped.
`symbolic.cyr` now needs `error.cyr` ahead of it, which the bundle's order already provides.

```cyrius
# 3.3
var v = expr_eval(tree, vars);          # 0.0 and a stderr line on a miss
# 3.4
var xt, v = expr_eval_checked(tree, vars);
if (is_err_result(xt) == 1) { … }       # v is HSB_ERR_INVALID_INPUT, not a value
```

## mat3 / mat4 / quat / transforms

### `m3_inverse` / `m4_inverse` → `m3_inverse_checked` / `m4_inverse_checked`

The retired inverses returned the **identity** for a singular or NaN matrix, with no error
channel. Their reciprocal test also misfired on scale:

- `m3_inverse`: 2^-400·I got the identity, and 2^400·I the zero matrix;
- `m4_inverse`: 2^-300·I got the identity, and 2^300·I a signed-zero matrix.

`m3_inverse_checked(m)` and `m4_inverse_checked(m)` return **`Ok(M⁻¹)`**. An entry whose exact
value lies past DBL_MAX comes back ±Inf, not an error. The errors:

- **`Err(HSB_ERR_SINGULAR_MATRIX)`** exactly when det M = 0, with the entries taken as the exact
  rationals they are. This is decided over the whole double range, never by a threshold.
- **`Err(HSB_ERR_INVALID_INPUT)`**: a NaN or infinite entry.
- **`Err(HSB_ERR_ALLOC)`**: the result cannot be allocated.

Where the cofactor formula is certified, the answer is 3.3.7's arithmetic bit for bit. Elsewhere
a double-double or an exact integer tier answers.

```cyrius
# 3.3
var inv = m4_inverse(m);                 # the identity when m is singular
# 3.4
var it, inv = m4_inverse_checked(m);
if (is_err_result(it) == 1) { … }        # inv: HSB_ERR_SINGULAR_MATRIX / _INVALID_INPUT / _ALLOC
```

### `t3d_inverse_matrix` → `t3d_inverse_matrix_checked`

`t3d_inverse_matrix(t)` was `m4_inverse(t3d_to_matrix(t))`. It inherited the identity for a zero
scale and for a NaN or infinite component, and it read a zero quaternion as no rotation.

`t3d_inverse_matrix_checked(t)` returns `m4_inverse_checked`'s `Result`. The errors:

- **`Err(HSB_ERR_SINGULAR_MATRIX)`**: a zero scale component.
- **`Err(HSB_ERR_INVALID_INPUT)`**: a zero rotation quaternion, or a NaN or infinite component.
  A zero q has no rotation, so `t3d_to_matrix` now gives it a NaN block.
- **`Err(HSB_ERR_ALLOC)`**.

```cyrius
# 3.3
var mi = t3d_inverse_matrix(t);
# 3.4
var tt, mi = t3d_inverse_matrix_checked(t);
```

### `m4_transform_point` / `world_to_screen` → `…_checked`

The retired forms failed at the w = 0 point at infinity, and wherever the clip vector overflowed
from finite input:

| input | `m4_transform_point` | `world_to_screen` |
|---|---|---|
| w = ±0 or NaN | the **undivided** clip coordinates | **(0, 0, 0)**, a legal screen position |
| w = ±Inf (1/w = 0) | 0 | the screen centre |
| a clip x that overflowed beside a finite w | ±Inf for a finite quotient | — |

`m4_transform_point_checked(m, v)` returns **`Ok(point)`**, M·(v, 1) divided by its w.
`world_to_screen_checked(point, mvp, vw, vh)` returns **`Ok((screen_x, screen_y, depth))`**,
with normalised device coordinates equal to `m4_transform_point_checked`'s bit for bit. In both,
w is decided **exactly**: every non-zero w divides, however small, and a rounded w of 0
decides nothing. The errors:

- **`Err(HSB_ERR_DIVISION_BY_ZERO)`**: the exact w is 0.
- **`Err(HSB_ERR_INVALID_INPUT)`**: a NaN or infinite entry in the matrix or the point.
- **`Err(HSB_ERR_ALLOC)`**.

Every answer the certificate passes keeps 3.3.7's bits.

```cyrius
# 3.3
var p = m4_transform_point(m, v);
var s = world_to_screen(pt, mvp, vw, vh);
# 3.4
var pt1, p = m4_transform_point_checked(m, v);
if (is_err_result(pt1) == 1) { … }       # p == HSB_ERR_DIVISION_BY_ZERO: the point is at infinity
var st1, s = world_to_screen_checked(pt, mvp, vw, vh);
```

### `hquat_inverse` / `hquat_normalize` → `…_checked`

Both retired forms returned the **identity** for q = 0, and NaN components for a NaN or infinite
component.

`hquat_inverse_checked(q)` returns `Ok(q⁻¹)` and `hquat_normalize_checked(q)` returns
`Ok(q / |q|)`. Every finite non-zero q gets the bits the retired function gave it. The errors:

- **`Err(HSB_ERR_DIVISION_BY_ZERO)`**: |q| = 0.
- **`Err(HSB_ERR_INVALID_INPUT)`**: a NaN or infinite component.
- **`Err(HSB_ERR_ALLOC)`**.

```cyrius
# 3.3
var u = hquat_normalize(q);
var r = hquat_rotate_vec3(hquat_normalize(q), v);     # nested: fine in 3.3
# 3.4
var nt, u = hquat_normalize_checked(q);
if (is_err_result(nt) == 1) { … }
var r = hquat_rotate_vec3(u, v);                      # never hquat_rotate_vec3(hquat_normalize_checked(q), v)
```

Since 3.4.0, `hquat_rotate_vec3` rotates by q/|q| itself (see below), so the normalisation is
often unnecessary.

### `hisab_inverse_lerp` / `hisab_remap` → `…_checked`

`hisab_inverse_lerp(a, b, value)` returned **0** in two cases where 0 was not the answer:

- for a = b, where 0 is also the right answer for value = a;
- for every subnormal width, where the division has an exact answer.

A width or offset that overflowed gave 0 or ±Inf for a finite t. `hisab_remap` inherited all of
this, and for in_lo = in_hi it returned out_lo.

`hisab_inverse_lerp_checked(a, b, value)` returns **`Ok(t)`**, t = (value − a) / (b − a), as f64
bits. Subnormal widths divide exactly, and overflowing ones divide halved copies. The errors:

- **`Err(HSB_ERR_DIVISION_BY_ZERO)`**: a = b. Either no t or every t solves it.
- **`Err(HSB_ERR_INVALID_INPUT)`**: a NaN or infinite `a`, `b` or `value`.

`hisab_remap_checked(value, in_lo, in_hi, out_lo, out_hi)` returns `Ok(x)` with the same errors,
taken over (in_lo, in_hi, value). There is one exception: value = in_lo with out_lo = out_hi is
determined, and comes back `Ok(out_lo)`.

```cyrius
# 3.3
var x = hisab_inverse_lerp(a, b, v);     # 0 for a == b
# 3.4
var lt, x = hisab_inverse_lerp_checked(a, b, v);
if (is_err_result(lt) == 1) { … }        # x: HSB_ERR_DIVISION_BY_ZERO / _INVALID_INPUT
# or, to take 0 on any refusal, as 3.3 did for a == b:
var x2 = result_unwrap_or(lt, x, 0);
```

### `m3_normal_matrix` → `m3_normal_matrix_cof`

`m3_normal_matrix(m)` was `m3_transpose(m3_inverse(m))`, M⁻ᵀ, and it failed in two ways:

- It inherited the inverse's **identity** for every singular m. diag(1, 1, 0) kept normals along
  x, where its image, the plane z = 0, has the normal z alone.
- M⁻ᵀ overflows where m is merely small along one axis: diag(1, 1, 2^-1074) sends (0, 0, 1) to
  (0, 0, +Inf).

`m3_normal_matrix_cof(m)` returns **sign(det M)·cof(M)**, the cofactor matrix with its
orientation fixed. That is |det M|·M⁻ᵀ wherever M is invertible: **the same directions, a
different magnitude**. That magnitude change is why the name is retired rather than redefined.
It exists for every M:

- For a singular M that flattens a surface onto a plane, it sends normals to that plane's
  normal (with det M = 0 the sign is +1).
- For rank 1 or 0, it is the zero matrix (no normal exists).
- A NaN or infinite entry propagates.
- Where an entry overflows or the largest falls below 2^-969, it is the cofactor matrix of
  M·2^-e: the same directions, times a power of two.

It cannot fail, so it returns a plain matrix: the designed 0 on a refused allocation.

```cyrius
# 3.3
var nm = m3_normal_matrix(m);                 # M^-T (the identity for a singular m)
# 3.4
var nm = m3_normal_matrix_cof(m);             # |det M| M^-T: the same directions
var n2 = hvec3_normalize(m3_mul_vec3(nm, n)); # renormalise every normal it transforms
```

## No compatibility aliases

There is no deprecation window. A retired name kept with its new meaning would compile silently
in a consumer that expects the old one, and consumers compile the bundle under their own pins.
An alias kept with the old meaning would keep the fabricated answers this release exists to
remove. So the 27 names are gone, and a reachable call to one is a build error (see
[Toolchain](#toolchain)). This is a minor release with breaking changes by design: hisab keeps
API breaks in the 3.x line.

## Behaviour that changed under an unchanged name

Each row below was checked against the 3.4.0 source (doc block or body). "3.3.7" is the tagged
release.

### Numerics

| function | 3.3.7 | 3.4.0 |
|---|---|---|
| `num_bisection` | max_iter exhausted: `Ok(midpoint)`, met or not (x² − 2 on [0, 2], max_iter 5: Ok(1.40625) at tol 1e-12); max_iter 0: Ok(1.0), untested | `Err(HSB_ERR_NO_CONVERGENCE)`, `out` untouched, unless the final bracket is narrower than tol or its ends are adjacent doubles (Ok, the bits 3.3.7 gave) |
| `num_bisection` | a NaN `a` or `b` reached f, and a step f could return Ok(NaN) | `Err(HSB_ERR_INVALID_INPUT)` before f is called |
| `num_bisection` | a bracket wider than DBL_MAX: Ok(±Inf) (outside [a, b]); an infinite end: NaN midpoint, refused | solved: the midpoint is the sum of the halves. x − c on [−DBL_MAX, DBL_MAX] returns c exactly; [−Inf, 4] and [−Inf, +Inf] find their roots. Brackets narrower than DBL_MAX keep their bits |
| `f64_fmod(x, ±0)` | +0 for every x (NaN and ±Inf included) | NaN: x's NaN quieted, else 0xFFF8000000000000 (glibc's bits). A caller that tested `== 0` to detect y = 0 must test y |
| `dual_ln`, `dual_sqrt`, `ad_ln`, `ad_sqrt` at a NaN | the (0, 0) domain-miss sentinel | NaN value and derivative, payload kept, a signalling NaN quieted. v ≤ 0 (−0, −Inf) keeps (0, 0) |
| `dual_abs`, `ad_abs` | NaN: derivative +a′ / partial +1; value kept its sign bit (\|−0\| = −0) | NaN derivative (a′·v); value `f64_abs(v)`: \|−0\| = +0, a NaN's sign cleared |
| `dual_div` over \|b\| < DBL_MIN (and the `dual_ln` / `dual_sqrt` misses) | a NaN numerator, or a NaN derivative on either side: (0, 0) | a NaN numerator: NaN value and derivative, quieted. A numeric numerator keeps (0, 0) — a domain miss is the constant 0, so its derivative is exactly 0 whatever tangent arrives, in forward and reverse mode alike |
| `ad_div` / `ad_mul` with a NaN operand value | `ad_div`: a NaN `a` over \|b\| < DBL_MIN was the (0, 0) miss; `ad_mul` / `ad_div` gave the other operand's partial (x·2 at x = NaN had gradient 2) | NaN value and every partial NaN, as forward mode gives |
| `dual_new` and every dual op | a refused 16-byte allocation was stored through (exit 139) | `dual_new` returns 0, and each dual op returns what it returns. A 0 passed in as an operand is still read |

### Symbolic

| function | 3.3.7 | 3.4.0 |
|---|---|---|
| `expr_diff` | an unknown tag differentiated to `expr_const(0)` (x + ⟨tag 77⟩ → (1 + 0)); a 0 node read address 0 | 0, the failure handle `sym_integrate` already used. Well-formed trees differentiate as before |
| the ten `expr_*` constructors, `expr_bind` | a 0 child built a node with a null child; a refused allocation was stored through | 0 for a 0 child (and for `expr_var` a 0 name) and for a refused allocation, so a failure reaches the root |
| `expr_simplify` | read the tag of a 0 from a child | passes the 0 up |
| `sym_const_to_str` | stored through a refused buffer | 0 |
| `sym_rewrite` | read a refused node | the designed 0 |
| `sym_match`, `sym_match_bindings` | crashed on a refused binding | 0 ("no match") for a refused binding as well. Documented: there is no other value to report it with |
| ⚠ `expr_to_str` | — | **unchanged**: it still reads a 0 node. Test the handle from `expr_diff` / a constructor before rendering it |

### Linear algebra

| function | 3.3.7 | 3.4.0 |
|---|---|---|
| `eigen_power`: triangular or permuted-triangular A | iterated; a non-normal one could converge above ρ(A) (the roadmap 3 × 3: 1.7e-5 off) | read off the diagonal without iterating: exact λ, eigenvector by substitution; `max_iter` does not bound it |
| `eigen_power`: dominant diagonal values tied ± / equal | iterated | ± tie: `NO_CONVERGENCE` at once. Equal values: answered exactly ([[2,1],[0,2]] → 2; 3.3.7: NO_CONVERGENCE) |
| `eigen_power`: a complex dominant pair in a two-vertex component | Ok with a smaller real eigenvalue | `NO_CONVERGENCE` |
| `eigen_power`: dominant \|λ\| below about 2.47e-312 | `NO_CONVERGENCE` | answered, in an exactly scaled frame |
| `eigen_power`: a non-normal A | Ok on the iteration's own tests | the answer must also pass a Gelfand bound, ρ(A) ≤ 1.001\|λ\|, or it is `NO_CONVERGENCE` |
| `eigen_power`: max_iter ≤ 0 | `NO_CONVERGENCE` | `INVALID_INPUT` |
| `lyapunov_max` | column 0 only: every upper-triangular A returned ln\|a00\| (diag(1, 2) → 0 for ln 2); nilpotent A → 0 | the largest of the n column averages; nilpotent → −Inf |
| `lyapunov_max`: iterations ≤ 0 | `Ok(0)` | `INVALID_INPUT` |
| `lyapunov_max`: A near DBL_MAX, a reflector below 1e-12, subnormal entries | ganita's QR: ln a for ln 2a, or NO_CONVERGENCE; 0 for [[1,1,0],[0,0,3e13],[0,1e-13,0.5]]; digits lost | hisab's own scaled Householder QR. Every finite A is answered; [[2,1],[1,2]] at k = 1000 moved from 126 to 15 ulp of the exact value |
| `svd_golub_kahan` | out_U / out_Vt shapes unchecked: an undersized one was written past, then Err | `INVALID_INPUT` unless exactly m × n / n × n, before anything is written |
| `csr_from_dense`, `csr_spmv`, `csr_transpose`, `csr_add` | a null handle or refused allocation: exit 139 | 0. Caller-sized counts are bounded before sizing |
| `matrix_condition_number(0)` | exit 139 | −1.0, its documented failure value |

### Calculus and optimisation

| function | 3.3.7 | 3.4.0 |
|---|---|---|
| `opt_conjugate_gradient`, `opt_lbfgs` | Armijo halving from α = 1 | a strong-Wolfe line search on the slope g(x + αd)·d. **Different iterates and evaluation counts.** Answers move toward the minimiser; problems that returned NO_CONVERGENCE after 30,000 iterations converge (Σ(i+1)(xᵢ−1)² + x₀x₁: CG 7 gradients, L-BFGS 11). Some cost more: L-BFGS on Wood from (−3, −1, −3, −1), 149 f / 118 g against 95 / 38. `opt_bfgs` is unchanged |
| `ode_symplectic_euler`, `ode_verlet`, `ode_leapfrog`, `ode_yoshida4` | stored through a refused 16-byte block | 0 |
| `ode_dopri45_trajectory` | two heap cells per call (16 bytes kept) | stack locals; same answers |
| Perlin and simplex noise | first use allocated its tables and stored through a refusal | static tables; same values |

### Geometry and transforms

| function | 3.3.7 | 3.4.0 |
|---|---|---|
| `hquat_rotate_vec3` | a rotation only for unit q; for q = (0.154, 3.32, 3.86, −15.2), (1, 1, −1) → (167, −175, 144); q = 0 → v unchanged | rotation by q/\|q\| for every finite non-zero q; q = 0 → NaN. A q whose \|q\|² is exactly 1.0 keeps its bits |
| `m3_from_quat`, `m4_from_srt`, `t3d_to_matrix` | the unit formula for every q; q = 0 → the identity rotation | R(q/\|q\|); q = 0 or non-finite → NaN block. \|q\|² = 1.0 keeps its bits |
| `t3d_apply`, `t3d_compose`, OBB rays and jets | followed the old rotation | follow the new one (they call `hquat_rotate_vec3`) |
| `hquat_slerp`, `t3d_lerp` | a zero interpolant in the lerp branch → the identity | a zero or non-finite interpolant → a NaN quaternion (it has no direction); 0 on a refused allocation |
| `screen_to_world_ray` | returned 0 (it stored through a refused allocation) | `HSB_ERR_ALLOC` as a plain integer, out-params untouched |
| `geo_barycentric_coords` | the centroid (⅓, ⅓, ⅓) whenever the Gram form read under DBL_MIN, whatever the point | exactly colinear or coincident vertices: the least-squares weights of minimum norm (sum 1, reproduce p's projection onto the line; ⅓ each when coincident). Other thin triangles (Gram form under 2^-16·d00·d11): weights from exact cross products |
| `geo_ray_capsule`, `geo_ray_capsule_branch`: r < 0 | \|r\| used, or NaN | the miss 0, `GEO_FACE_NONE` |
| … r = +Inf | +Inf, sometimes on the wrong piece | +Inf on the piece the exit lies on as r grows |
| … an end with one infinite component | undefined | the half-infinite capsule (both ends infinite on one axis and equal off it: the infinite cylinder); two infinite components or Inf − Inf: the miss; an infinite ray against an infinite end: NaN. A NaN radius or coordinate is still NaN out (3.3.3's contract) |
| `geo_ray_sphere`, `geo_ray_capsule` accuracy | wrong roots where a·δ underflowed, for origins on or near the surface or barrel (up to 7.3e5 ulp), and for thin barrels 2^48–2^400 radii away (the roadmap's barrel 2^168 radii away: a hit at 6.4 for an exact miss) | those classes are solved with exact expansions where the coefficients cancel, so their answers move; see the 3.4.0 notes in `geo.cyr` |
| `gjk_intersect_3d`, `gjk_epa_3d`, `mpr_intersect`, `mpr_penetration`, `time_of_impact` at t = 0 | separated pairs ~1e-12 apart reported as depth-0 contacts; GJK and MPR could disagree | **the support contract** (eps = 2^-46·M): a pair separated by more than 4 eps is never a contact. Inside the band, gjk_* and mpr_* give one verdict per pair. mpr_* answer alone only where their portal proves the contact, so in-band they pay GJK's path (lane CO2's A/B: 84.8× for two spheres 1e-8 deep at 1e6). Depths and normals on corners, edges and capsule sides snap to closed forms, so pinned values move |
| `cga_*` constructors and products | stored through a refused allocation | 0 for a refused allocation or a 0 handle passed in; `cga_norm` / `cga_norm_sq` NaN |
| `bvh_build` | stored through a refusal | 0, the handle it returns for no boxes |

### Allocation refusals, everywhere

Through 3.3.7 most of hisab's allocation sites stored through `alloc`'s designed 0 once the heap
refused a request: exit 139. In 3.4.0 every function answers its own documented failure value
instead:

- the designed **0** handle, for a constructor or a function that returns a handle;
- the **empty vec**, for `convex_hull_2d`, `triangulate_polygon`, `delaunay_2d` and
  `detect_islands`;
- **`Err(HSB_ERR_ALLOC)`**, for a `Result`.

The composite builders (the jets, the SE(3) poses, the rule lists) form every part before the
whole, so none hands back an object with a 0 inside it. The containers keep every point:

- A quadtree or octree split that cannot get its children leaves the leaf whole.
- A spatial hash that cannot grow keeps its table.
- A k-d tree that cannot get a node is refused whole.

There are two documented exceptions:

- `sym_match` and `sym_match_bindings` answer 0, "no match", for a refused binding too (above).
- einsum's arena requests are left unchecked because they fit the arena by construction.
  `einsum` itself returns 0 when the arena cannot be created.

  **Since 3.4.1** einsum has no arena: its scratch is static, nothing but the result is asked of the
  heap, and the call survives a caller's `alloc_reset()`. A limit too small for the old 8,208-byte
  arena now answers. The same release takes the refusal path to every public entry: a refused
  allocation no longer faults or exits, and the five collision entries answer their designed 0.

`halfedge_from_triangles` has two new guards, both checked before anything is built:

- a twin table past `ALLOC_MAX / 8` slots returns 0;
- `n_tris > vec_len / 3` returns 0, written as a division. A count whose ×3 wrapped used to
  exit 1.

⚠ **The stdlib's own growth paths are not covered.** `vec_push` and `str_builder_add` still
abort the process on a refused grow, by the stdlib's policy, and hisab still grows through them
in places: `num_sieve`, `num_factorize` and `expr_to_str` among others.

## What did NOT change

- **The `HSB_ERR_*` codes**: the same twelve names and values. No new code was added.
- **Every other public name and parameter list.** We diffed 3.3.7's bundle against one
  regenerated from the 3.4.0 tree. Both carry 653 public functions and 703 public declarations.
  The sets differ by exactly the 27 renames, and the parameter lists of the other 626 functions
  are identical.
- **The stdlib sidecar.** `dist/hisab.deps` names the same seven leaves: `math`, `alloc`, `vec`,
  `ganita`, `result`, `str`, `fmt`.
- **Out-parameters** of `svd_compute_checked` and `solve_pgs_checked`, which keep their
  predecessors' parameters.
- **`#must_use`.** All 27 replacements carry it. Discarding one draws
  `warning: #must_use result of '…' is discarded`.

## Toolchain

**hisab's own pin moves from cyrius 6.6.20 (3.3.7) to 6.7.9** (`cyrius.cyml`). A consumer still
compiles `dist/hisab.cyr` under its **own** cycc, so what matters is the pin each consumer
holds. The 14 repos carrying a hisab bundle pin cyrius 6.6.2 (attn11, goonj), 6.6.3 (garjan,
ghurni, jalwa, naad, nidhi, prani, shabda, shabdakosh, svara), 6.6.6 (prakash), 6.6.9 (ranga)
and 6.6.10 (dhvani), read from their `cyrius.cyml` on 2026-10-31. The floor documented since
3.1.0 is **cyrius ≥ 6.6.3**: 6.6.2 refuses `public struct` + `#derive(accessors)`. The 3.4.0
bundle still meets that floor:

**Measured on the release bundle** (2026-10-31): a consumer program — an inverse of a scaled
identity, a singular and two zero-sign inverses, `m3_normal_matrix_cof`, a non-unit rotation,
`calc_adaptive_simpson_checked`, `num_bisection`, `f64_fmod(1, 0)`, `num_pollard_rho_checked`,
`lyapunov_max`, `eigen_power`, `svd_compute_checked`, `cx_powf(2, Inf)` and `num_fft` — was built
two ways (the manifest's 16 stdlib leaves, and the sidecar's 7 plus `syscalls`, `string` and
`io`) from scratch dirs pinned to 6.6.2, 6.6.3, 6.6.6, 6.6.9, 6.6.10, 6.6.12, 6.6.13, 6.6.14,
6.6.20 and 6.7.9.

- **6.6.2 refuses** the bundle with the known `#derive(...) applies to a struct or an enum`
  error, and nothing new.
- **Every other pin** builds both ways with no warning or `duplicate` line, and runs. The output
  is bit-identical across all of them except `cx_powf(2, Inf)` on 6.6.3 and 6.6.6 (NaN; the
  known pair since 3.3.5).
- Two 3.4.0 repairs exist because of older pins: a `var` repeated in sibling `elif` blocks of
  `_sym_rewrite_single` was refused by 6.6.3 (`duplicate variable`; declared once since), and the
  exact matrix paths flip a sign with the sign bit rather than `f64_neg`, which gave +0 for +0
  below cycc 6.6.8 (so a consumer on 6.6.3 or 6.6.6 got +0 where IEEE gives −0).

**What a retired name does at build time** (measured on 6.7.9 against the regenerated bundle):

```
warning: undefined function 'm3_inverse' (reachable call site)
error: refusing to emit binary with 1 reachable undefined function(s) (pass --allow-undef to downgrade)
```

A call in a function nothing reaches is **only the warning**, and `--allow-undef` turns the
error into a warning too. So the build is not a complete search: use the grep in the checklist.
(The 3.3 guide records the same refusal for `f64_tan` under 6.6.3–6.6.12.)

**The sidecar** was checked by running `cyrius distlib` on a scratch copy of the 3.4.0 tree
(2026-10-31): `sidecar: 7 leaf(s), compile-verified`, the same seven as 3.3.7. `result` has been
on that list since 3.0.0, and `cyrius deps` reads it from `dist/hisab.deps`.

## Checklist

1. **Find every retired name**, with a grep rather than the build. Bash, from your repo root:

   ```bash
   grep -rnE '(^|[^A-Za-z0-9_])(svd_compute|solve_gmres|solve_bicgstab|solve_pgs|calc_adaptive_simpson|ode_backward_euler|ode_bdf2|ode_bdf|calc_monotone_cubic|calc_partial_derivative|calc_hermite_tcb|fbm_2d|fbm_3d|simplex_fbm_2d|simplex_fbm_3d|num_pollard_rho|expr_eval|m3_inverse|m4_inverse|t3d_inverse_matrix|m4_transform_point|world_to_screen|hquat_inverse|hquat_normalize|hisab_inverse_lerp|hisab_remap|m3_normal_matrix)[[:space:]]*\(' \
     --include='*.cyr' --include='*.tcyr' --include='*.bcyr' --include='*.fcyr' \
     --exclude-dir=lib --exclude-dir=dist --exclude-dir=build .
   ```

   The pattern does not match the `_checked` / `_cof` names, or `simplex_fbm_2d` for `fbm_2d`.
   Comment lines that quote a call do match: skip them. Prove it fires on your tree first, with
   one name you know you call. (Under zsh, `grep` may be a ugrep wrapper; run the line under
   `bash -c`.) On svara it prints exactly `./src/prosody.cyr:115`.
2. **Bind both halves at every replacement call**: `var t, v = f(…);`, or `var v = f(…)?;`
   inside a `Result`-returning function. **Never nest one in another call.** Then grep for each
   `_checked` name you adopted and check that no call sits inside another call's parentheses.
3. **For the `out` forms**, pass 8 writable bytes (`var slot: i64[1]` with `&slot`), and read
   it only after `is_ok(t)`. `calc_adaptive_simpson_checked` also writes it on NO_CONVERGENCE.
4. **Krylov**: pass `tol / ‖b‖₂` for 3.3.7's threshold, and leave a matvec or two of headroom in
   GMRES's `max_iter`. On Err the payload is a code, not `x`.
5. **`m3_normal_matrix_cof`**: renormalise every normal it transforms.
6. **`result` in `[deps] stdlib`**: it is there already if you call any 3.x `Result` function,
   and `cyrius deps` adds it from `dist/hisab.deps` either way.
7. **Re-pin tests that pinned old answers.** Pins that change:
   - an identity from an inverse;
   - a 0 from an fBm, an interpolator or `hisab_inverse_lerp`;
   - a NaN from adaptive Simpson;
   - a last iterate from an implicit ODE step;
   - `f64_fmod(x, 0) == 0`;
   - `num_bisection` Ok on an exhausted `max_iter`;
   - `lyapunov_max` 0 or ln|a00|;
   - `eigen_power` on triangular matrices (now exact) and on tiny ones (now answered, where
     3.3.7 gave NO_CONVERGENCE);
   - rotations by a q whose |q|² is not exactly 1;
   - centroid barycentrics;
   - a negative capsule radius;
   - collision depths, normals and support-call counts near the noise band;
   - CG / L-BFGS iterates and evaluation counts.

   Settle each new value independently (a closed form, an exact oracle) before re-pinning: do not
   copy whatever the new build prints.
8. **Check 0 handles** from `expr_diff`, the `expr_*` constructors and the CGA / geometry
   constructors before passing them on. `expr_to_str` still reads a 0.
9. **Build under your own pin** and read the warnings: `undefined function` (a retired name you
   missed) and `#must_use result of '…' is discarded` (a replacement whose `Result` you dropped).
   The first is an error only for a reachable call. The second is only a warning.
