# Security Policy

## Scope

Hisab is a pure mathematics library written in Cyrius providing linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, interval arithmetic, and tensor operations. The core library performs no I/O.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Allocation overflow | Integer overflow in `rows * cols * 8` could cause undersized allocation | Overflow guards on tensor, complex matrix, diffgeo allocations; dimension caps |
| Numerical stability | Catastrophic cancellation, overflow | IEEE 754 f64 throughout; documented precision limits |
| Matrix decompositions | Division by near-zero pivot | Partial pivoting with EPSILON_F64 threshold checks |
| Iterative solvers | Non-convergence on adversarial input | max_iter bounds; returns ERR_NO_CONVERGENCE |
| FFT | Invalid input length | Requires power-of-2 |
| Integration | Zero step count | Returns ERR_ZERO_STEPS |
| GJK/EPA | Non-convergence on degenerate shapes | 64-iteration hard limit |
| Sieve of Eratosthenes | Unbounded allocation | Capped at 10M elements |
| Division by zero | NaN/Inf propagation through complex, autodiff, transforms | Zero guards on cx_div, cx_inv, dual_div, dual_sqrt, dual_ln, f64_fmod, world_to_screen, linearize_depth_reverse_z |
| Modular arithmetic | Overflow in multiplication for large moduli | Russian peasant _num_mulmod avoids overflow |
| Symbolic eval | Process abort on undefined variable | Returns 0 with warning (no longer aborts) |
| Perlin noise | Global mutable state for permutation table | Single-threaded only; documented |

## Known Limitations

- `num_modpow` is NOT constant-time. Do not use for cryptographic applications.
- Jacobi eigensolver is O(n^5) worst case. Not suitable for n > 50.
- SVD via A^T*A squares the condition number. See roadmap for Golub-Kahan replacement.
- PCG32 uses signed arithmetic with masking. Verified safe but not cryptographically secure.
- `m4_get`/`m4_set` do not bounds-check col/row arguments. Caller must validate.

## Supported Versions

| Version | Supported |
|---------|-----------|
| Cyrius 1.4.x | Yes |
| Rust 1.4.x (rust-old/) | Archive only |

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
