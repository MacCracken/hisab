# Security Policy

## Scope

Hisab is a pure mathematics library written in Cyrius providing linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, interval arithmetic, and tensor operations. The core library performs no I/O.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Allocation overflow | Integer overflow in `rows * cols * 8` could cause undersized allocation (CWE-190) | Overflow guards on tensor, complex-matrix, diffgeo allocations; dimension caps — pinned by regression tests (2026-05-29 audit). Stdlib `mat_new` gained its own upstream guard in **ganita 1.0.4** (cyrius 6.5.6 pin, hisab **2.6.11**) — rejects non-positive dims and element counts over 33,554,430, returning null — and that contract is now pinned by regression tests. Before 2.6.11 the vendored copy was stale at 1.0.3, where `mat_new(-5, 3)` segfaulted; hisab's own usage was mitigated throughout (dims taken from already-allocated matrices), and `mat_new_guarded` is retained as the stricter 16M-element entry point for untrusted dimensions |
| Numerical stability | Catastrophic cancellation, overflow | IEEE 754 f64 throughout; documented precision limits |
| Matrix decompositions | Division by near-zero pivot | Partial pivoting with EPSILON_F64 threshold checks |
| Iterative solvers | Non-convergence on adversarial input | max_iter bounds; returns ERR_NO_CONVERGENCE |
| FFT | Invalid input length | Requires power-of-2 |
| Integration | Zero step count | Returns ERR_ZERO_STEPS |
| GJK/EPA, MPR narrowphase | Non-convergence on degenerate shapes | 64-iteration hard limit (`_COL_MAX_ITER`) |
| Collision algorithms | OOB index / non-termination (convex hull, triangulation, Delaunay, half-edge) | All index access via `vec_get` (traps on OOB, no silent corruption); ear-clip `n*n` cap, half-edge one-ring 1000-step guard. Audited + covered in the 2.4.x arc |
| Sieve of Eratosthenes | Unbounded allocation | Capped at 10M elements |
| Division by zero | NaN/Inf propagation through complex, autodiff, transforms | Zero guards on cx_div, cx_inv, dual_div, dual_sqrt, dual_ln, f64_fmod, world_to_screen, linearize_depth_reverse_z |
| Modular arithmetic | Overflow in multiplication for large moduli | Russian peasant _num_mulmod avoids overflow |
| Symbolic eval | Process abort on undefined variable | Returns 0 with warning (no longer aborts) |
| Perlin noise | Global mutable state for permutation table | Single-threaded only; documented |
| Capped constructors | A designed `0` (null) return from a dimension cap being **stored through** by the caller (CWE-476) | Closed in two passes: **2.6.14** for `cmat_new`'s four callers and the BFGS/L-BFGS working-memory allocations (`_OPT_MAX_DIM = 4096`, new `HSB_ERR_ALLOC`); **2.7.0** for `cmat_add`/`sub`/`adjoint`/`scale`/`trace`, which `cmat_commutator` feeds directly. Pre-fix mutants exit **139 (SIGSEGV)** |
| Complex matrix multiply | Out-of-bounds read past `b`'s buffer when `cols(a) != rows(b)` (CWE-125) | Conformability check added 2.7.0 |
| Unbounded / non-terminating loops | Work bounds that do not actually bound work | `cmat_exp`'s scaling loop hung forever on a non-finite norm (`+Inf * 0.5 == +Inf`) — capped 2.7.0. Adaptive Simpson capped recursion *depth* but permitted 2^50 integrand evaluations, which a NaN integrand reached exactly — bounded 2.6.14 |
| Recursion depth on degenerate geometry | Coincident points/boxes collapsing a spatial split to one element per level | `kdtree_build` **SIGSEGV measured at 60,000** coincident points — fixed 2.6.14 (positional split). `_bvh_build_rec` was the unfixed sibling: not a crash but **O(n²)** build cost, a resource-exhaustion vector — fixed 2.7.0 (median split), **2.722 s → 15.5 ms** at n=4,000 |
| Spline evaluation | Negative indexing of the control-point array from an unvalidated `degree` | Fixed 2.6.14 — `degree` and control-point-count validation |
| NaN acceptance | A NaN comparing false against every ordered test being read as "passed the check" | `_opt_armijo` accepted a NaN objective as a valid step and propagated it into `x` for three solvers — fixed 2.6.14 (requires `f_new == f_new` first). `geo_ray_capsule` returned NaN for an axis-parallel ray — fixed 2.7.0 |
| Collision query completeness | A query that silently under-reports is worse than one that errors | `time_of_impact` capped its search at 0.64 units of relative travel regardless of `max_t` — a **false negative**. Fixed 2.7.0 with a step budget spanning the full horizon |

## Known Limitations

- `num_modpow` is NOT constant-time. Do not use for cryptographic applications.
- Jacobi eigensolver is O(n^5) worst case. Not suitable for n > 50.
- SVD via A^T*A squares the condition number. See roadmap for Golub-Kahan replacement.
- PCG32 uses signed arithmetic with masking. Verified safe but not cryptographically secure.
- `m4_get`/`m4_set` do not bounds-check col/row arguments. Caller must validate.

## Supported Versions

| Version | Supported |
|---------|-----------|
| hisab 2.7.x (current) | Yes |
| hisab 2.6.x | Yes |
| hisab 2.0–2.5 | Best-effort |
| Rust 1.x | Available via pre-2.0 git tags, unsupported |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- No `syscall(60, ...)` (process abort) in library code — errors via return codes
- All public functions document their error conditions
- Allocation sizes are guarded against overflow where inputs are user-controlled
- Bump allocator (no free) — no use-after-free, no double-free
- No network I/O in core library
- Single external dependency (sakshi for structured logging)
- P(-1) audit completed 2026-04-15 — see docs/audit/2026-04-15.md
- Full sweep 2026-08-03 (70 findings, 2 critical) — repaired across 2.6.12–2.6.15; re-audited
  2026-08-04 with **zero regressions**. See docs/audit/ and docs/development/threat-model.md
- Fixes are **mutation-proven**: a repair lands only once its source file has been reverted and the
  new assertions confirmed to fail. Where a defect is a cost rather than a wrong answer, that is
  stated explicitly and a benchmark guards it instead — a passing suite is not allowed to imply a
  proof it did not provide
- Hand-encoded IEEE-754 constants are verified against their own comments by
  `scripts/check-constants.sh` (a CI gate since 2.6.12, after seven mis-transcribed tables shipped)
