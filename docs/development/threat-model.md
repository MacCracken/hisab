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
| mat_new (stdlib) | Integer overflow in `16 + rows*cols*8` | **Upstream bug** — still unguarded in pinned 6.0.14 (fix tracked for when the cyrius pin moves). hisab provides **`mat_new_guarded`** (2.5.3) as the safe constructor for untrusted dims (cap 16M elems); internal `mat_new` calls use dims from already-allocated matrices (mitigated); raw-dim `cmat_new` is guarded |
| convex_hull_2d | Monotone-chain pop underflow / index | Fixed in 2.4.0 (sort + `f64_le`/`f64_ge`); `vec_get` traps on OOB rather than corrupting |
| triangulate_polygon | Ear-clip non-termination | `n*n` iteration cap + "no ear → bail" |
| delaunay_2d | Bad-triangle / cocircular degeneracy | Super-triangle + strict in-circle; collinear → empty (no trap) |
| halfedge_is_boundary | One-ring walk non-termination | 1000-step guard |
| mpr_intersect/penetration | False positive on separated shapes; non-convergence | Fixed in 2.4.4 (origin-containment early-out); `_COL_MAX_ITER = 64` |
| sequential_impulse | Zero/unbounded impulse | Fixed in 2.4.5 (sign + accumulate-into-velocity); impulse clamped ≥ 0, converges |
| num_newton/bisection | Non-convergence | max_iter bound; returns ERR_NO_CONVERGENCE |
| num_modpow | Intermediate multiplication overflow | _num_mulmod (Russian peasant) avoids overflow |
| cga_blade_inverse | Division by zero on a null blade (`⟨B~B⟩₀ ≈ 0`) | Zero guard returns the zero multivector; pinned by the project-onto-null-blade test |
| cga_pseudoscalar_inv | Division by `⟨I~I⟩₀` | Structurally `−1` for the unit pseudoscalar (no external input); cannot be zero |
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

## Supply Chain

- No third-party runtime dependencies — only the cyrius stdlib and first-party
  **sakshi**. No FFI, no libc. Third-party-CVE attack surface is zero.
- Integrity enforced by the SHA-locked `cyrius.lock`: `cyrius deps --verify` →
  60 verified / 0 failed; `cyrius vet` → 0 untrusted (verified 2026-05-29).
- No CVEs/advisories exist for the Cyrius/cycc toolchain (niche sovereign
  project, not indexed in public CVE databases).

## Audit History

- **2026-04-15**: P(-1) audit — 31 issues found, 25 fixed. See [docs/audit/2026-04-15.md](../audit/2026-04-15.md).
- **2026-05-29**: P(-1) hardening (v2.4.6) — security/CVE/supply-chain review closing the 2.4.x collision arc. No new vulnerability; 6 allocation-guard regression tests added; `mat_new` upstream item reconfirmed. See [docs/audit/2026-05-29.md](../audit/2026-05-29.md).
- **2026-05-29**: 2.5.x closeout (v2.5.4) — P(-1)/security review of the CGA operators + `mat_new_guarded`. Posture solid (fixed-size allocs, bounded loops, guarded/structural divisions); no fix needed. Earned `architecture/math.md` (equation catalogue). See [docs/audit/2026-05-29-cga-arc-closeout.md](../audit/2026-05-29-cga-arc-closeout.md).
