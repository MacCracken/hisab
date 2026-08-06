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
| sequential_impulse | Zero/unbounded normal impulse | Fixed in 2.4.5 (sign + accumulate-into-velocity); impulse clamped ≥ 0, converges. Restitution re-applied per sweep (period-2 orbit at e = 1) fixed in 2.8.3 — e is a one-off bias on the target velocity |
| sequential_impulse | Friction impulse identically 0 — Coulomb cone never enforced because nothing was ever clamped | Fixed post-2.8.4. The branch clamped the zero it had just loaded from its own output slot; measured 0 of 320 configs nonzero. Now `λ_t = clamp(-v_t/inv_mass, ±mu·λ_n)`, accumulate-then-clamp so the bound is on the total; `mu = 0` and `tangent_vel = 0` paths bit-identical |
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
| geo_ray_plane | Ambiguous t=0 hit vs miss | **Changed 2.7.0** — returns **0** for a miss, matching the contract stated twice in `geo.cyr`'s own headers and every sibling intersection routine. The previous `-1` was the raw i64 `0xFFFFFFFFFFFFFFFF`, i.e. a **negative NaN** as an f64: detectable only by integer comparison, and silently poisoning any float arithmetic a consumer performed on it |
| GJK/EPA | Non-convergence on degenerate shapes | 64-iteration hard limit |
| Perlin noise | Global mutable permutation table | Single-threaded only |
| PCG32 | Signed arithmetic for unsigned ops | Verified safe: & masks discard sign extension |
| m4_get/m4_set | No bounds check | Contract: col/row in [0,3], caller must validate |
| Jacobi eigensolver | O(n^5) for large matrices | Documented: not for n > 50 |
| SVD via A^T*A | Squares condition number | Documented: Golub-Kahan planned |

### Memory-safety tier — closed in v2.6.14

Recorded here because three of these four groups **crashed the process**: reverting `complex.cyr`,
`calc_ext.cyr` or `optimize.cyr` individually makes the suite exit **139 (SIGSEGV)**.

| Module | Risk | Mitigation |
|--------|------|------------|
| `cmat_mul` / `cmat_kronecker` / `cmat_inverse` / `cmat_identity` | `cmat_new` correctly returns 0 above `_CMAT_MAX_ELEMS` (65536); four callers then **stored through that 0** (CWE-476). Reachable from operands that are themselves under the cap — 1000×1 times 1×1000 asks for 1e6; 17×17 ⊗ 17×17 is 83521; `cmat_inverse` doubles width building `[M ǀ I]` | Fixed 2.6.14 — null checks at every store site. Tests pin **both sides** of the 16×16 ⊗ 16×16 = 65536 edge, which is legitimately allowed |
| `opt_bfgs` / `opt_lbfgs` | Working memory sized from an **unbounded** caller dimension: an `n × n` inverse Hessian and an `m × (2n+1)` history ring, with `alloc` returning 0 and the init loop storing through it (CWE-190 → CWE-476) | Fixed 2.6.14 — `_OPT_MAX_DIM = 4096` (the `ALLOC_MAX` hard ceiling is 5792) + explicit allocation checks in all four solvers + a new `HSB_ERR_ALLOC` (`-11`) so allocation failure is reportable rather than fatal |
| `calc_bspline` / `calc_nurbs` | `degree` itself was never validated, nor the well-formedness rule that a degree-*d* spline needs ≥ *d+1* control points. With `degree >= n_pts` the sentinel-undo produced `k - n_pts` and de Boor indexed `ctrl_pts` at a **negative** offset — a wild-pointer read instead of the documented 0-on-invalid-input | Fixed 2.6.14 — degree and control-point-count validation |
| adaptive Simpson | Capped recursion **depth** at 50 but not **work**: each non-converged node spawns two children, permitting 2^50 (~1e15) integrand evaluations. A NaN integrand takes exactly that path, since every f64 comparison with NaN returns 0, so `ǀerrorǀ < tol` is false forever | Fixed 2.6.14 — explicit non-finite bail plus a stop-when-bisection-stops-progressing check, bounding real work by f64 resolution rather than by a counter |
| `kdtree_build` | `_kd_partition` splits on the axis **value** midpoint, which can only degenerate when every point shares that value — and that case fell to the one-element-per-level clamp, giving O(n) depth. **Measured**, not inferred: fine at 40,000 coincident points, **SIGSEGV at 60,000** | Fixed 2.6.14 — splits positionally in exactly that case, sound because the values are equal (the `split_val` the caller reads is then the common value, so query pruning still compares exactly) |
| `_opt_armijo` | Tie-breaker written `f64_lt(threshold, f_new) == 0`, which is **also true when `f_new` is NaN** — so a NaN objective was accepted as a valid step and propagated into `x` for `opt_bfgs`, `opt_lbfgs` and `opt_conjugate_gradient` | Fixed 2.6.14 — requires `f_new` to compare equal to itself first |

### Closed in v2.7.0

| Module | Risk | Mitigation |
|--------|------|------------|
| `cmat_mul` | Never checked `cols(a) == rows(b)` and ran the inner product over `cols(a)` while indexing `b` by that bound — an **out-of-bounds read** (CWE-125) past a buffer allocated at exactly `rows(b)*cols(b)*16` bytes. A 2×3 times a 2×2 read two complex numbers of adjacent heap | Conformability check returns 0 |
| `cmat_add` / `sub` / `adjoint` / `scale` / `trace` | Dereferenced the **designed-0** return of `cmat_new`/`cmat_mul` (CWE-476) — the 2.6.14 null-propagation fix stopped one call-link short, and `cmat_commutator` feeds exactly that in by passing two `cmat_mul` results straight to `cmat_sub`. Both pre-fix mutants **SIGSEGV** | Null + shape guards; `cmat_trace` returns 0 rather than `cx_zero()`, since zero is a valid trace |
| `cmat_exp` | Scaling loop halves the Frobenius norm until it drops below 1 — but `+Inf * 0.5` is still `+Inf`, so **a single overflowing entry hung the process forever**. Measured: 1 s with the fix, >110 s without | `_CMAT_EXP_MAX_SCALING = 64`; `exp` overflows f64 above 709, so no larger scaling parameter can describe a representable result |
| `_bvh_build_rec` | Fell back to a 1-vs-(n−1) split whenever no center sorted below the midpoint — precisely what coincident or clustered AABBs do. Because each level re-merges its whole range for `bounds`, build cost was **O(n²)** in time and arena: a resource-exhaustion vector on attacker-chosen geometry. Measured 2.722 s at n=4,000 | Median fallback restores O(n log n) — `bvh_degenerate_4k` **2.722 s → 15.5 ms**. Guarded by benchmark, not assertion: both forms return the same tree |
| `time_of_impact` | **False negative from a collision query.** The advancement loop was bounded by `GJK_MAX_ITER` — a cap on GJK's *simplex refinement*, not on time sampling — while stepping a fixed `0.01/speed`, so it covered `64 × 0.01 = 0.64` units of relative displacement and returned 0 regardless of the `max_t` the caller asked for | Dedicated `_GA_TOI_MAX_STEPS` budget spanning the full horizon, with the tolerance step as a floor |
| `geo_ray_capsule` | A ray **parallel to the capsule axis** makes `d_perp` exactly zero, so `a == 0`, `b == 0` and the discriminant is exactly `+0.0` — which does not satisfy the `disc < 0` test guarding the cap-sphere fallback. Execution reached `0.5 / 0 = +Inf` and returned NaN from a function documented to return 0 on a miss, propagating into caller arithmetic | The parallel case now joins the cap-sphere path, which is the whole answer for it |
| `f64_copysign` | Branched on `f64_lt(y, 0)`, an **ordered** comparison: `-0.0` compares equal to `+0.0`, so the sign of a negative zero was invisible and the function could never return `-0.0` at all | Replaced with the IEEE-754 sign-bit splice `(x & ~signmask) ǀ (y & signmask)` |
| `f64_fmod` | Used `f64_floor` on the quotient where IEEE-754 `fmod` truncates, so the result carried the sign of the **divisor** rather than the **dividend**: `fmod(-7, 3)` returned `+2` where C returns `-1` | `f64_trunc` |


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

- **2026-08-03**: P(-1) full sweep of the v2.6.11 tree — **70 findings confirmed / 7 refuted** (2 critical, 23 high, 23 medium, 22 low; 64 sites in 27 files). See [docs/audit/2026-08-03.md](../audit/2026-08-03.md). Headline was not a memory-safety issue but a **correctness** one: seven hand-encoded IEEE-754 constant tables did not encode their documented values, four of them published numerical-method tableaux falsified by their own mathematical invariants — DOPRI45 had 23 of 30 coefficients wrong and Σb = 0.636, so it was **not a consistent integrator at any order**. Root cause identified as coverage: 8 shipped modules were included by no test suite and held 35 of 65 source defects. Repaired across **2.6.12–2.6.15**; the constant class is now a CI gate (`scripts/check-constants.sh`).
- **2026-08-04**: Re-audit of the repaired tree (v2.6.15) — see [docs/audit/2026-08-04.md](../audit/2026-08-04.md). **Regression verification CLEAN**: every 2.6.12–2.6.15 repair holds under execution, verified against mathematical invariants with the pre-fix files re-run as controls. 42 new findings confirmed / 8 refuted (4 high, 21 medium, 17 low; **no criticals**). Two corrections to the record, both process failures rather than code defects, and both recorded because the failure mode is what matters: (1) the 2026-08-03 audit was **not** fully discharged — six of the 70 findings were never scheduled, because repair work was driven from this roadmap's deduplicated P0–P4 *digest* rather than from the audit's **finding list**, and "all tiers closed" was then read as "all findings closed"; (2) `scripts/check-constants.sh` — the gate added to prevent the headline defect recurring — had a **25% blind spot**, its regex rejecting `_` digit separators so it skipped 35 of 145 declarations while printing a confident "110/110 verified". Fixing the gate immediately exposed a real mis-encoded constant. **A gate that reports its own coverage must have that coverage checked.**
- **2026-08-04**: v2.7.0-A/B repair batches. Ten findings closed (the 6 carried over, the 3 high, 1 medium) plus one found while fixing. Security-relevant closures are tabled above under *Closed in v2.7.0*: an out-of-bounds read (`cmat_mul`), a null-dereference family that **SIGSEGVs** (`cmat_add`/`sub`/`adjoint`/`scale`/`trace`), a **non-terminating loop** on non-finite input (`cmat_exp`), an **O(n²) resource-exhaustion** path on attacker-chosen geometry (`_bvh_build_rec`), and a **false negative from a collision query** (`time_of_impact`). Every fix except the BVH one is **mutation-proven** — the source file was reverted and the new assertions confirmed to fail. The exception is labelled rather than glossed: both forms of `_bvh_build_rec` return the same tree, so the defect is a *cost* and is guarded by a benchmark instead. Suite 1127 → **1181**; all gates green.

- **2026-04-15**: P(-1) audit — 31 issues found, 25 fixed. See [docs/audit/2026-04-15.md](../audit/2026-04-15.md).
- **2026-05-29**: P(-1) hardening (v2.4.6) — security/CVE/supply-chain review closing the 2.4.x collision arc. No new vulnerability; 6 allocation-guard regression tests added; `mat_new` upstream item reconfirmed. See [docs/audit/2026-05-29.md](../audit/2026-05-29.md).
- **2026-05-29**: 2.5.x closeout (v2.5.4) — P(-1)/security review of the CGA operators + `mat_new_guarded`. Posture solid (fixed-size allocs, bounded loops, guarded/structural divisions); no fix needed. Earned `architecture/math.md` (equation catalogue). See [docs/audit/2026-05-29-cga-arc-closeout.md](../audit/2026-05-29-cga-arc-closeout.md).
- **2026-05-30**: 2.6.x closeout (v2.6.5) — P(-1)/security review of the new diffgeo functions (sectional/Weyl/Jacobi contractions, parallel transport, higher-form wedges). Posture solid (bounded loops, guarded divisions, dims inherit the `dim ≤ 16` cap); no fix needed. Grew `math.md §2` (differential geometry). See [docs/audit/2026-05-30.md](../audit/2026-05-30.md).
- **2026-06-15**: Cyrius 6.0.14 → 6.2.11 toolchain bump (v2.6.6). No new attack surface (no shell/FFI/syscall change). Stdlib reorg: transcendentals + matrix/linalg → `ganita`; `f64_le`/`f64_ge` adopted from stdlib `math` (NaN-correct, supersedes hisab's negation-based locals — a strict robustness improvement). Re-verified the `mat_new` overflow item: unchanged and still unguarded upstream (now `ganita_mat_new`), `mat_new_guarded` mitigation intact. 3 of 5 tracked toolchain bugs fixed on the new pin (archived). 957/957 tests, all gates green.
- **2026-06-30**: Cyrius 6.2.11 → 6.3.11 toolchain bump (v2.6.7) + sakshi 2.1.0 → 2.4.2. No new attack surface (no shell/FFI/syscall change; no library source change). `ganita.cyr` byte-identical on the new pin → `mat_new` overflow item unchanged, `mat_new_guarded` mitigation intact. Vendored `lib/result.cyr` picked up the 6.3.11 `_die` agnos-portability fix: the prior bare `syscall(60, 1)` no-op'd on agnos (undefined syscall number), so unwrap-on-`Err` **failed open** and continued with garbage; now a target-guarded abort — a robustness improvement on agnos, no change on the Linux x86_64 build path. Tracked-issue re-verify: for-empty-clauses still open on 6.3.11; no new fixes. 957/957 tests, all gates green.
- **2026-07-17**: Cyrius 6.3.11 → 6.4.66 toolchain bump (v2.6.9) + sakshi 2.4.2 → 2.4.6. No new attack surface (no shell/FFI/syscall change; no library source change). `ganita.cyr` changed bytes on the new pin but `ganita_mat_new`'s body is unchanged (still `16 + rows*cols*8`, unguarded) → `mat_new` overflow item unchanged, `mat_new_guarded` mitigation intact. The much larger 6.4.66 stdlib snapshot (new `async`/`tls`/`sandhi`/`net`/`http`/`regex` modules) is **not** pulled into hisab's build — the declared `[deps] stdlib` subset is unchanged, so no new upstream surface enters the bundle. Fixed a pre-existing `tests/modules.tcyr` compile failure (cycc identifier-lexer bug, not security-relevant); filed `issues/2026-07-17-cyrius-interval-ident-lex.md`. Tracked-issue re-verify: for-empty-clauses still open on 6.4.66. 957/957 tests, all gates green.
- **2026-08-03**: Cyrius 6.4.69 → **6.5.6** toolchain bump (v2.6.11) + sakshi 2.4.6 → **2.4.7**. A minor jump across 24 releases; no executable library change (the only `src/` edit is the `mat_new_guarded` doc comment). **This bump closes a real, previously-understated exposure.** The audit-history entries above (2026-06-15 through 2026-07-21) all recorded `mat_new` as "unguarded upstream, mitigated by `mat_new_guarded`" — correct about upstream, but they compared the *toolchain snapshots* to each other and never byte-checked the **vendored** `lib/ganita.cyr` against the pin. It was stale at **1.0.3** while 6.4.69 already shipped **1.0.4** *with* the CWE-190 guard, so hisab spent the 2.6.10 cycle building against a copy that was not merely unguarded but crash-prone: measured on this bump, `mat_new(-5, 3)` **SIGSEGV (exit 139)** under 1.0.3 and returns null under 1.0.4, with the compiler held constant and only `ganita.cyr` swapped. Re-vendoring fixes it; `mat_new_guarded` is retained as the stricter 16M-element cap. **Process change:** the vendoring check is now `lib/*.cyr` byte-compared against `~/.cyrius/versions/<pin>/lib/` (which surfaced this), not toolchain-vs-toolchain. Two further **defensive** fixes, both toolchain-driven: all four `.tcyr` harnesses exited with `assert_summary()`'s raw failure count, so exactly 256/512/768 failures truncated to 0 in the 8-bit wait status and **scored PASS** — demonstrated with a 256-failure probe, then clamped (this was a CI-gate integrity hole, not a library vulnerability); and the `sys_exit_group` epilogue replaced `syscall(SYS_EXIT, …)`, which terminates only the calling thread (no impact on single-threaded hisab, correctness-by-construction going forward). `cyrius fuzz` now discovers `tests/*.fcyr` — the harness had never actually executed under `cyrius fuzz` and passes on first real run. Remaining stdlib delta (`io`, `vec` +`vec_sort_by`/`vec_select_nth`, two syscall peers) adds no surface hisab links. Tracked-issue re-verify on 6.5.6 (minimal repros): interval-ident-lex **and** for-empty-clauses both still live; cli-arg-clobber not re-tested (destructive); no new fixes. **961/961** tests (edge_cases 163 → 167: +4 assertions pinning the upstream `mat_new` contract, mutation-proven — reverting `lib/ganita.cyr` to 1.0.3 crashes the suite at exit 139 instead of passing), fuzz 1/1, all gates green.
- **2026-07-21**: Cyrius 6.4.66 → 6.4.69 toolchain bump (v2.6.10); sakshi unchanged at 2.4.6. No new attack surface (no shell/FFI/syscall change; no library source change). `ganita.cyr` byte-identical on the new pin → `mat_new` overflow item unchanged, `mat_new_guarded` mitigation intact. The 3-file stdlib delta is all upstream **hardening**, net-positive for hisab's posture: `fmt` (hex high-bit rendering + a `fmt_float_buf` non-finite guard that stops garbage output on Inf/NaN — a robustness improvement on the `symbolic` render path, byte-identical for finite inputs), `math` (float-parse exponent saturation — closes an upstream O(exp) algorithmic-complexity DoS on `"1e100000000"`-style input; hisab does not parse untrusted floats via that path, so no direct exposure, but the hardening is inherited), and an agnos-only `sys_reboot` widening (not on hisab's Linux x86_64 build path). Tracked-issue re-verify on 6.4.69 (minimal repros): interval-ident-lex **and** for-empty-clauses both still live; no new fixes. 957/957 tests, all gates green.
