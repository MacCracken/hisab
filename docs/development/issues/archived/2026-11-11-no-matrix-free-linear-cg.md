# No matrix-free linear conjugate gradient: 1.4.0's `num::conjugate_gradient` has no 3.x counterpart — closed in 3.4.2

**Status:** ✅ **CLOSED in 3.4.2** (2026-11-14). A feature request, checked against hisab 3.4.1 with cyrius 6.7.18, 2026-11-11.
**Discovered:** 2026-11-11, by pravash's Rust → Cyrius port (M0 step 4 and the language review, plan §1.1).
**Reporter:** pravash 2.0.0 port (its request P1, conversion ledger L5; `pravash/docs/development/pravash-port-plan.md` §8).
**Severity:** **Medium.** No consumer breaks, but a grid fluid solver has no hisab CG to call, so pravash transcribes
1.4.0's.

## Summary

hisab 1.4.0 had `num::conjugate_gradient(a_mul, b, x0, tol, max_iter)`: a **linear** CG for a symmetric positive
definite system, where the operator is a closure (`a_mul(x) → A·x`), so no matrix is ever formed. pravash's
`FluidGrid` pressure projection calls it on the 5-point Poisson operator every step (`use_cg`), with a relative
tolerance (1e-6·‖b‖, floor 1e-14).

hisab 3.4.1 has no counterpart:
- `opt_conjugate_gradient(f, grad, x0, n, tol, max_iter, out_x, out_fmin)` is the **nonlinear** optimiser (it
  minimises f);
- `solve_bicgstab_checked`, `solve_gmres_checked` and `solve_pgs_checked` take a **CSR matrix**. Assembling the
  Poisson matrix every step costs memory and time the matrix-free form doesn't, and BiCGSTAB isn't CG's iteration,
  so its bits and iteration counts differ from the oracle's.

## Ask

A matrix-free linear CG, for example:

```cyrius
# Solves A·x = b for a symmetric positive definite A given as an operator: apply(ctx, x, out) writes A·x into out.
# x0 is the initial guess and is overwritten with the solution. Stops when ‖r‖ ≤ tol (absolute; the caller scales
# it by ‖b‖) or after max_iter iterations. Returns the iteration count, or Err.
public fn num_cg(apply, ctx, b, x, n, tol, max_iter, scratch) { ... }
```

- **The operator as a function pointer plus a context word** (a closure works the same way through `fncall`), so a
  stencil is applied with no matrix.
- **Caller-provided scratch** (3·n words: r, p, A·p), so a per-step solve allocates nothing (see this repo's H9 issue).
- **The iteration count returned**: pravash compares it (class A) against the oracle's.

For bit parity with 1.4.0, pravash will transcribe 1.4.0's loop as a marked stand-in. The operation order matters:
1.4.0 computes α = (r·r)/(p·Ap), updates x and r, then β = (r'·r')/(r·r). If hisab's version keeps that order,
pravash converts to it with no parity change.

## Closed in 3.4.2

**The repair is the ask's signature:** `num_cg(apply, ctx, b, x, n, tol, max_iter, scratch)` in `src/num_ext.cyr`.
- **The operator:** `fncall3(apply, ctx, v, out)` writes `A v` into `out`.
- **The tolerance:** absolute, ‖r‖ < tol.
- **The scratch:** 3n words, so nothing is allocated.
- **The returns:** `Ok(iterations)` when it converges. `Err(HSB_ERR_NO_CONVERGENCE)` after `max_iter` iterations or at
  a breakdown, with `x` holding the last iterate.

**1.4.0's iteration, in 1.4.0's order:**
- α = (r·r)/(p·Ap); then x and r are updated; then β = (r'·r')/(r·r).
- Each dot product is folded in index order from −0, as Rust's `f64` sum folds.
- It runs in a frame scaled by a power of two set from max|b − A x0|. That scaling is exact, so x, the iteration count
  and every test are 1.4.0's own, bit for bit, wherever 1.4.0's values stay normal and its guard does not fire.
  `tests/modules_c.tcyr`, group "3.4.2: num_cg", pins this against a transcription of 1.4.0's loop (pravash's
  stand-in), on 16 × 16 and 8 × 8 Poisson problems.

**Where it differs from 1.4.0, each on purpose:**
- **1.4.0's absolute guard is gone.** 1.4.0 stopped when |p·Ap| < 1e-12 and returned that iterate as `Ok`, which made
  the answer depend on the problem's scale. A = 2⁻³⁰·L, b = 2⁻³⁰·c stops early under 1.4.0. `num_cg` returns the
  unscaled problem's x bit for bit, so a pravash row where 1.4.0's guard fired converges further.
- **The only breakdown is the exact one:** p·Ap not above 0, or not finite.
- **`max_iter` and a breakdown are reported:** both return `Err(NO_CONVERGENCE)`. 1.4.0 returned `Ok` for both, and
  went on iterating after a negative p·Ap.
