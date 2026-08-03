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
| mat_new (stdlib) | Integer overflow in `16 + rows*cols*8` | ✅ **RESOLVED 2026-08-03 (v2.6.11)** — upstream fixed in **ganita 1.0.4**: `ganita_mat_new` now returns 0 for non-positive dims and for `rows > GANITA_MAT_MAX_ELEMS / cols` (33_554_430, the ALLOC_MAX-derived ceiling), with null propagated through `ganita_mat_identity` / `ganita_mat_from_array`. **Note the exposure was worse than previously recorded**: the repo's vendored `lib/ganita.cyr` was stale at **1.0.3** while the 6.4.69 pin already shipped 1.0.4, so hisab built against the unguarded copy for the whole 2.6.10 cycle. Severity confirmed by measurement, not inspection — on 1.0.3 `mat_new(-5, 3)` **SIGSEGV (exit 139)**; on 1.0.4 it returns null (same compiler, only `ganita.cyr` swapped). hisab **keeps `mat_new_guarded`** (2.5.3) as the stricter policy cap (16M elems vs upstream 33.5M) for untrusted dims; internal `mat_new` calls use dims from already-allocated matrices (mitigated); raw-dim `cmat_new` is guarded. Prior history: 2026-07-21 byte-identical 6.4.66 → 6.4.69; 2026-07-17 bytes changed, body unchanged; 2026-06-30 byte-identical on 6.3.11; 2026-06-15 moved from `matrix.cyr` to `ganita_mat_new` |
| convex_hull_2d | Monotone-chain pop underflow / index | Fixed in 2.4.0 (sort + `f64_le`/`f64_ge`); `vec_get` traps on OOB rather than corrupting |
| triangulate_polygon | Ear-clip non-termination | `n*n` iteration cap + "no ear → bail" |
| delaunay_2d | Bad-triangle / cocircular degeneracy | Super-triangle + strict in-circle; collinear → empty (no trap) |
| halfedge_is_boundary | One-ring walk non-termination | 1000-step guard |
| mpr_intersect/penetration | False positive on separated shapes; non-convergence | Fixed in 2.4.4 (origin-containment early-out); `_COL_MAX_ITER = 64` |
| sequential_impulse | Zero/unbounded impulse | Fixed in 2.4.5 (sign + accumulate-into-velocity); impulse clamped ≥ 0, converges |
| num_newton/bisection | Non-convergence | max_iter bound; returns ERR_NO_CONVERGENCE |
| num_modpow | Intermediate multiplication overflow | _num_mulmod (Russian peasant) avoids overflow |
| sectional_curvature | Division by a degenerate plane area (`⟨u,u⟩⟨v,v⟩−⟨u,v⟩² ≈ 0`) | Zero guard (`< EPSILON → 0`); pinned by the degenerate-plane test |
| weyl_tensor | Division by `(n−2)` for `n < 3` | Early `if (dim < 3) return zero` — the formula is undefined / identically zero there |
| wedge_2_1 / wedge_3_1 / hodge_star_2form_4d | Fixed 4D reduced-basis layout assumed | Documented 4D contract; caller must pass 4D forms (6-/4-component) |
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
- **2026-05-30**: 2.6.x closeout (v2.6.5) — P(-1)/security review of the new diffgeo functions (sectional/Weyl/Jacobi contractions, parallel transport, higher-form wedges). Posture solid (bounded loops, guarded divisions, dims inherit the `dim ≤ 16` cap); no fix needed. Grew `math.md §2` (differential geometry). See [docs/audit/2026-05-30.md](../audit/2026-05-30.md).
- **2026-06-15**: Cyrius 6.0.14 → 6.2.11 toolchain bump (v2.6.6). No new attack surface (no shell/FFI/syscall change). Stdlib reorg: transcendentals + matrix/linalg → `ganita`; `f64_le`/`f64_ge` adopted from stdlib `math` (NaN-correct, supersedes hisab's negation-based locals — a strict robustness improvement). Re-verified the `mat_new` overflow item: unchanged and still unguarded upstream (now `ganita_mat_new`), `mat_new_guarded` mitigation intact. 3 of 5 tracked toolchain bugs fixed on the new pin (archived). 957/957 tests, all gates green.
- **2026-06-30**: Cyrius 6.2.11 → 6.3.11 toolchain bump (v2.6.7) + sakshi 2.1.0 → 2.4.2. No new attack surface (no shell/FFI/syscall change; no library source change). `ganita.cyr` byte-identical on the new pin → `mat_new` overflow item unchanged, `mat_new_guarded` mitigation intact. Vendored `lib/result.cyr` picked up the 6.3.11 `_die` agnos-portability fix: the prior bare `syscall(60, 1)` no-op'd on agnos (undefined syscall number), so unwrap-on-`Err` **failed open** and continued with garbage; now a target-guarded abort — a robustness improvement on agnos, no change on the Linux x86_64 build path. Tracked-issue re-verify: for-empty-clauses still open on 6.3.11; no new fixes. 957/957 tests, all gates green.
- **2026-07-17**: Cyrius 6.3.11 → 6.4.66 toolchain bump (v2.6.9) + sakshi 2.4.2 → 2.4.6. No new attack surface (no shell/FFI/syscall change; no library source change). `ganita.cyr` changed bytes on the new pin but `ganita_mat_new`'s body is unchanged (still `16 + rows*cols*8`, unguarded) → `mat_new` overflow item unchanged, `mat_new_guarded` mitigation intact. The much larger 6.4.66 stdlib snapshot (new `async`/`tls`/`sandhi`/`net`/`http`/`regex` modules) is **not** pulled into hisab's build — the declared `[deps] stdlib` subset is unchanged, so no new upstream surface enters the bundle. Fixed a pre-existing `tests/modules.tcyr` compile failure (cycc identifier-lexer bug, not security-relevant); filed `issues/2026-07-17-cyrius-interval-ident-lex.md`. Tracked-issue re-verify: for-empty-clauses still open on 6.4.66. 957/957 tests, all gates green.
- **2026-08-03**: Cyrius 6.4.69 → **6.5.6** toolchain bump (v2.6.11) + sakshi 2.4.6 → **2.4.7**. A minor jump across 24 releases; no executable library change (the only `src/` edit is the `mat_new_guarded` doc comment). **This bump closes a real, previously-understated exposure.** The audit-history entries above (2026-06-15 through 2026-07-21) all recorded `mat_new` as "unguarded upstream, mitigated by `mat_new_guarded`" — correct about upstream, but they compared the *toolchain snapshots* to each other and never byte-checked the **vendored** `lib/ganita.cyr` against the pin. It was stale at **1.0.3** while 6.4.69 already shipped **1.0.4** *with* the CWE-190 guard, so hisab spent the 2.6.10 cycle building against a copy that was not merely unguarded but crash-prone: measured on this bump, `mat_new(-5, 3)` **SIGSEGV (exit 139)** under 1.0.3 and returns null under 1.0.4, with the compiler held constant and only `ganita.cyr` swapped. Re-vendoring fixes it; `mat_new_guarded` is retained as the stricter 16M-element cap. **Process change:** the vendoring check is now `lib/*.cyr` byte-compared against `~/.cyrius/versions/<pin>/lib/` (which surfaced this), not toolchain-vs-toolchain. Two further **defensive** fixes, both toolchain-driven: all four `.tcyr` harnesses exited with `assert_summary()`'s raw failure count, so exactly 256/512/768 failures truncated to 0 in the 8-bit wait status and **scored PASS** — demonstrated with a 256-failure probe, then clamped (this was a CI-gate integrity hole, not a library vulnerability); and the `sys_exit_group` epilogue replaced `syscall(SYS_EXIT, …)`, which terminates only the calling thread (no impact on single-threaded hisab, correctness-by-construction going forward). `cyrius fuzz` now discovers `tests/*.fcyr` — the harness had never actually executed under `cyrius fuzz` and passes on first real run. Remaining stdlib delta (`io`, `vec` +`vec_sort_by`/`vec_select_nth`, two syscall peers) adds no surface hisab links. Tracked-issue re-verify on 6.5.6 (minimal repros): interval-ident-lex **and** for-empty-clauses both still live; cli-arg-clobber not re-tested (destructive); no new fixes. **961/961** tests (edge_cases 163 → 167: +4 assertions pinning the upstream `mat_new` contract, mutation-proven — reverting `lib/ganita.cyr` to 1.0.3 crashes the suite at exit 139 instead of passing), fuzz 1/1, all gates green.
- **2026-07-21**: Cyrius 6.4.66 → 6.4.69 toolchain bump (v2.6.10); sakshi unchanged at 2.4.6. No new attack surface (no shell/FFI/syscall change; no library source change). `ganita.cyr` byte-identical on the new pin → `mat_new` overflow item unchanged, `mat_new_guarded` mitigation intact. The 3-file stdlib delta is all upstream **hardening**, net-positive for hisab's posture: `fmt` (hex high-bit rendering + a `fmt_float_buf` non-finite guard that stops garbage output on Inf/NaN — a robustness improvement on the `symbolic` render path, byte-identical for finite inputs), `math` (float-parse exponent saturation — closes an upstream O(exp) algorithmic-complexity DoS on `"1e100000000"`-style input; hisab does not parse untrusted floats via that path, so no direct exposure, but the hardening is inherited), and an agnos-only `sys_reboot` widening (not on hisab's Linux x86_64 build path). Tracked-issue re-verify on 6.4.69 (minimal repros): interval-ident-lex **and** for-empty-clauses both still live; no new fixes. 957/957 tests, all gates green.
