# Three caller-sized allocations in num_ext.cyr store through alloc's 0 return

**Filed**: 2026-08-11 (P(-1) sweep at v2.11.0)
**Status**: 🟢 **FIXED in 2.11.1** — `src/num_ext.cyr`
**Severity**: High — reproduced as SIGSEGV from a public entry point
**Class**: CWE-190 → CWE-476, caller-sized allocation stored through unchecked (second instance;
the four `optimize.cyr` solvers were the first, 2.6.14)

## Summary

Three allocations size a working buffer from a **caller-supplied dimension** and then store through
the result without checking `alloc`'s documented 0 return:

| site | allocation | reached from |
|---|---|---|
| `num_fft_2d` | `alloc(rows * 16)` | public |
| `num_ifft_2d` | `alloc(rows * 16)` | public |
| `num_tridiag_solve` | `alloc(n * 8)` ×2 | public |

`num_fft_2d` requires only that `rows` and `cols` are powers of two — there is no upper bound.
`num_tridiag_solve` has no dimension guard at all before its workspace.

## Reproduced

```
num_tridiag_solve, n = 300,000,000  ->  alloc(n*8) = 2.4 GB     exit 139 (SIGSEGV)
num_fft_2d, rows = 2^28, cols = 1   ->  alloc(rows*16) = 4.3 GB exit 139 (SIGSEGV)
```

After the repair both return **`HSB_ERR_ALLOC` (−11)**.

## ⚠ The first reproduction attempt FAILED, and the reason is the interesting part

The probe was written against `ALLOC_MAX = 0x10000000` (256 MiB) — the value stated in
`src/optimize.cyr`'s own comment. At `n = 33,554,432` that makes `n * 8` **exactly** `0x10000000`,
the allocation **succeeded**, and the function returned a clean error code. The finding looked
refuted.

**`ALLOC_MAX` is `0x80000000` = 2 GiB.** cyrius 6.4.51 raised it, `lib/alloc.cyr:169` says so
outright, and `linalg_ext.cyr:789` had already been updated — while `optimize.cyr:47` and
`threat-model.md` still derived a ceiling from the old value. So a stale comment in one file nearly
buried a real defect in another, by making the probe too small.

Both stale derivations are corrected in the same release: the ceiling on an `n × n` f64 matrix is
**16384**, not 5792. The policy cap `_OPT_MAX_DIM = 4096` sits below both the old and the new
ceiling, so no behaviour was ever unsafe — which is exactly why the drift survived four toolchain
bumps without anything failing.

**A constant derived from a dependency is a measurement.** It goes stale silently when the
dependency moves, and nothing in the build will say so.

## Fix

An explicit `if (buf == 0) { return HSB_ERR_ALLOC; }` after each of the three allocations — the same
repair 2.6.14 applied to the four `optimize.cyr` solvers, and the reason `HSB_ERR_ALLOC` exists.

No dimension cap was added. Unlike `opt_bfgs`'s `n × n` Hessian, an FFT of a genuinely large signal
is a legitimate request; the honest failure is "I could not allocate that", not "I refuse sizes
above an arbitrary line".

## Verification

* `tests/abuse.tcyr` asserts `num_tridiag_solve` returns `HSB_ERR_ALLOC` at `n = 300,000,000`.
  **No memory is committed** — `alloc` refuses, which is the point.
* **Mutation: removing the two `cp`/`dp` guards CRASHES THE SUITE PROCESS** rather than failing an
  assertion. That is the strongest kill available.
* ⚠ **`num_fft_2d` and `num_ifft_2d` are deliberately NOT asserted in the suite**, and that is
  recorded rather than quietly skipped: reaching their allocation requires the row loop to run
  `rows` times first, and `rows` must exceed 2^27 to cross `ALLOC_MAX` — 268 million iterations
  before the allocation is even attempted. That is a reproducer, not a unit test. Both were
  verified by hand at `rows = 2^28`.

## How it was found

A mechanical sweep for `alloc(<caller-supplied expression>)` across all 35 modules, run *before* any
free-form review, because this defect class has produced findings before. Reading the code found the
candidates in minutes; **only running them separated the two real ones from the eleven that already
had guards.**
