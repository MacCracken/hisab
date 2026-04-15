# Threat Model

## Trust Boundaries

Hisab operates at the **library boundary**. It trusts the calling application to:
- Provide valid numeric inputs (not NaN/Infinity unless documented)
- Provide valid pointers from `alloc()` (not arbitrary addresses)
- Respect dimension constraints (matrix sizes, tensor ranks)

Hisab does NOT trust:
- Input sizes — validates/caps dimensions for matrix, tensor, diffgeo operations
- Input magnitudes — uses EPSILON_F64 thresholds for near-zero detection
- Convergence — all iterative algorithms have max_iter bounds

## Attack Surface

| Module | Risk | Mitigation |
|--------|------|------------|
| tensor_new | Integer overflow in `total * 8` | Overflow guard, max 1M elements, rank cap 8 |
| cmat_new | Integer overflow in `rows * cols * 16` | Overflow guard, max 64K elements |
| christoffel/riemann | `dim^3` / `dim^4` overflow | Dim capped at 16 |
| num_sieve | Unbounded allocation | Capped at 10M |
| mat_new (stdlib) | Integer overflow in `rows * cols * 8` | **Upstream bug** — tracked for cyrius 5.0.1 |
| num_newton/bisection | Non-convergence | max_iter bound; returns ERR_NO_CONVERGENCE |
| num_modpow | Intermediate multiplication overflow | _num_mulmod (Russian peasant) avoids overflow |
| cx_div, cx_inv | Division by zero | Zero guard returns cx_zero() |
| dual_div/ln/sqrt | Division by zero | Zero guard returns dual_new(0,0) |
| world_to_screen | Perspective divide by w=0 | Returns hvec3_zero() |
| linearize_depth_reverse_z | Division by ndc=0 | Returns 0 |
| f64_fmod | Division by y=0 | Returns 0 |
| f64_tan | cos(x)=0 at PI/2 | Returns IEEE 754 Inf (documented) |
| expr_eval | Undefined variable | Returns 0 with stderr warning (no longer aborts) |
| geo_ray_plane | Ambiguous t=0 hit vs miss | Returns -1 for miss (not 0) |
| GJK/EPA | Non-convergence on degenerate shapes | 64-iteration hard limit |
| Perlin noise | Global mutable permutation table | Single-threaded only |
| PCG32 | Signed arithmetic for unsigned ops | Verified safe: & masks discard sign extension |
| m4_get/m4_set | No bounds check | Contract: col/row in [0,3], caller must validate |
| Jacobi eigensolver | O(n^5) for large matrices | Documented: not for n > 50 |
| SVD via A^T*A | Squares condition number | Documented: Golub-Kahan planned |

## Known Non-Cryptographic Functions

- `num_modpow` — NOT constant-time. Do not use for cryptographic key operations.
- `pcg32` — fast PRNG, not cryptographically secure. Use for simulation/testing only.

## Numerical Precision

All math uses f64 (IEEE 754 double precision, ~15 significant digits).

| Constant | Value | Hex | Verified |
|----------|-------|-----|----------|
| EPSILON_F64 | 1e-12 | 0x3D719799812DEA11 | Yes |
| F64_PI | π | 0x400921FB54442D18 | Yes (stdlib) |
| F64_E | e | 0x4005BF0A8B145769 | Yes (stdlib) |

BDF-5 coefficients (300/137, etc.) were recomputed exact and verified via IEEE 754 encoding during the 2026-04-15 audit.

## Audit History

- **2026-04-15**: P(-1) audit — 31 issues found, 25 fixed. See [docs/audit/2026-04-15.md](../audit/2026-04-15.md).
