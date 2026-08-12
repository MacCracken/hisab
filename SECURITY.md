# Security Policy

## Scope

Hisab is a pure mathematics library written in Cyrius providing linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, interval arithmetic, and tensor operations. The core library performs no I/O, with one exception: `expr_eval` writes a warning to stderr on an undefined variable and on an unknown expression tag (`src/symbolic.cyr:275`, `:317`) instead of aborting — see the Symbolic eval row below. No network I/O, no filesystem access, no FFI, no libc.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Allocation overflow | Integer overflow in `rows * cols * 8` could cause undersized allocation (CWE-190) | Overflow guards on tensor, complex-matrix, diffgeo allocations; dimension caps — pinned by regression tests (2026-05-29 audit). Stdlib `mat_new` gained its own upstream guard in **ganita 1.0.4** (cyrius 6.5.6 pin, hisab **2.6.11**) — rejects non-positive dims and element counts over 33,554,430, returning null — and that contract is now pinned by regression tests. Before 2.6.11 the vendored copy was stale at 1.0.3, where `mat_new(-5, 3)` segfaulted; hisab's own usage was mitigated throughout (dims taken from already-allocated matrices), and `mat_new_guarded` is retained as the stricter 16M-element entry point for untrusted dimensions |
| Numerical stability | Catastrophic cancellation, overflow | IEEE 754 f64 throughout; documented precision limits |
| Matrix decompositions | Division by near-zero pivot | Partial pivoting with EPSILON_F64 threshold checks |
| Iterative solvers | Non-convergence on adversarial input | max_iter bounds; returns ERR_NO_CONVERGENCE |
| FFT | Invalid input length | Requires power-of-2 |
| Integration | Zero step count | Returns ERR_ZERO_STEPS |
| GJK/EPA, MPR narrowphase | Non-convergence on degenerate shapes; an iteration cap that is not a work bound because the loop allocates | 64-iteration hard limit (`_COL_MAX_ITER`). The cap alone was not enough: `mpr_penetration`'s convergence test was **unsatisfiable for a body with interior**, so it burned all 64 iterations every call, and EPA's expansion loop was O(k²) — **8,576 face evaluations and 1.24 MB of never-freed arena per call** at the 2026-08-04 audit. Both rewritten 2.8.3. A first O(k²) repair was **rejected** in 2.8.2 despite hitting 812 evaluations / 99.6 KB, because its flood-fill visible-set search under-reports when rounding splits the visible region into disconnected components (239 of 47,134 cases; a cylinder-vs-rotated-box reproducer 40.7× too small). Arena growth is now pinned per support call by `tests/modules.tcyr` rather than by an absolute bound: `mpr_penetration` **293 B/call over 572 calls** (sibling `gjk_epa_3d` 296 over 552), byte-identical across five processes, sized so an injected 80 B/face/iteration leak overshoots by **123%** where the old absolute bound cleared it by 0.22% |
| Collision algorithms | OOB index / non-termination (convex hull, triangulation, Delaunay, half-edge) | All index access via `vec_get` (traps on OOB, no silent corruption); ear-clip `n*n` cap, half-edge one-ring 1000-step guard. Audited + covered in the 2.4.x arc |
| Sieve of Eratosthenes | Unbounded allocation | Capped at 10M elements |
| Division by zero | NaN/Inf propagation through complex, autodiff, transforms | Zero guards on cx_div, cx_inv, dual_div, dual_sqrt, dual_ln, f64_fmod, world_to_screen, linearize_depth_reverse_z. ⚠ **The presence of a guard is not the same as a correct one**, and 2.11.0 found four of the autodiff guards were fabricating answers rather than rejecting bad input: `dual_div` returned (0,0) for `1/1e-13` where the truth is 1e13, `dual_ln` tested `f64_abs(v)` — not ln's domain — so `ln(-5)` came back as a NaN value beside a **confident** -0.2 derivative, and `dual_sqrt` computed the root *before* its guard so a negative input was already NaN. All repaired at DBL_MIN, on the rule *guard exactly what makes the division fail and nothing more*; the same rule settled six thresholds in `geo.cyr` across 2.10.2. See `issues/archived/2026-08-11-forward-mode-dual-guards.md`. ⚠ **2.11.1's audit found the same class at ~24 further sites and `cx_div` — cited in this very row — is among them, still returning exactly zero below |z| = 1e-12**; verified live during that release. The tier is scheduled on the roadmap, so treat every guard named in this row as *present*, not as *proven correct* |
| Modular arithmetic | Overflow in multiplication for large moduli | Russian peasant _num_mulmod avoids overflow |
| Symbolic eval | Process abort on undefined variable | Returns 0 with warning (no longer aborts) |
| Perlin noise | Global mutable state for permutation table | Single-threaded only; documented |
| Capped constructors | A designed `0` (null) return from a dimension cap being **stored through** by the caller (CWE-476) | Closed in two passes: **2.6.14** for `cmat_new`'s four callers and the BFGS/L-BFGS working-memory allocations (`_OPT_MAX_DIM = 4096`, new `HSB_ERR_ALLOC`); **2.7.0** for `cmat_add`/`sub`/`adjoint`/`scale`/`trace`, which `cmat_commutator` feeds directly. Pre-fix mutants exit **139 (SIGSEGV)** |
| Complex matrix multiply | Out-of-bounds read past `b`'s buffer when `cols(a) != rows(b)` (CWE-125) | Conformability check added 2.7.0 |
| Unbounded / non-terminating loops | Work bounds that do not actually bound work | `cmat_exp`'s scaling loop hung forever on a non-finite norm (`+Inf * 0.5 == +Inf`) — capped 2.7.0. Adaptive Simpson capped recursion *depth* but permitted 2^50 integrand evaluations, which a NaN integrand reached exactly — bounded 2.6.14 |
| Recursion depth on degenerate geometry | Coincident points/boxes collapsing a spatial split to one element per level | `kdtree_build` **SIGSEGV measured at 60,000** coincident points — fixed 2.6.14 (positional split). `_bvh_build_rec` was the unfixed sibling: not a crash but **O(n²)** build cost, a resource-exhaustion vector — fixed 2.7.0 (median split), **2.722 s → 15.5 ms** at n=4,000 |
| Spline evaluation | Negative indexing of the control-point array from an unvalidated `degree` | Fixed 2.6.14 — `degree` and control-point-count validation |
| NaN acceptance | A NaN comparing false against every ordered test being read as "passed the check" | `_opt_armijo` accepted a NaN objective as a valid step and propagated it into `x` for three solvers — fixed 2.6.14 (requires `f_new == f_new` first). `geo_ray_capsule` returned NaN for an axis-parallel ray — fixed 2.7.0 |
| Collision query completeness | A query that silently under-reports, or answers with a wrong sign, is worse than one that errors | `time_of_impact` capped its search at 0.64 units of relative travel regardless of `max_t` — a **false negative**. The 2.7.0 step budget spanning the full horizon did not close the class: 2.8.3 found the function still a fixed-step **sampler** walking `t` in equal steps against a *boolean* overlap test, and a boolean carries no information about separation, so the step could only come from the horizon while the overlap window in `t` is only ~`2R/‖v_rel‖` wide. Replaced 2.8.3 with real conservative advancement over a new GJK **distance** sub-algorithm (Voronoi-region simplex closest point) |
| Narrowphase depth as a wrong answer | A penetration depth that is wrong — frequently negative — is consumed as a physics impulse | `mpr_penetration` refined against a portal whose `v0` is the **interior** point, so its plane passed through the inside of the Minkowski difference and its distance from the origin was not a depth. `mpr_intersect` had false positives from the same root cause; EPA kept coplanar faces while removing their neighbours and used an **absolute** convergence test unreachable at scene scale; GJK's degeneracy tests were absolute on a *squared* length. Fixed 2.8.3, measured against an exact reference (15-axis OBB SAT in `Fractions`, closed forms, independent Nelder-Mead) over 862 configurations in 35 shape classes: `mpr_penetration` **66/455 (216 wrong sign) → 862/862**, `gjk_epa_3d` 415/455 → 862/862, `mpr_intersect` 9 false positives → **0**. Worst relative error after: **1.6e-9**. `gjk_intersect_3d` was the remaining sibling — 134 genuine interior overlaps missed against an exact-rational reference over 4,386 evaluations — closed 2.9.0 at 0 missed, 0 sibling disagreement, for +55% median on the no-hit path |
| Out-parameter conformance | An out-param whose dimensions are never checked, written through a stride taken from that same out-param — an out-of-bounds write **under a success return code** (CWE-787) | `cqr_decompose` documents `out_R` as `m x n` but nothing checked it, and `cmat_set` takes its stride from `out_R`'s *own* column count, so the `m x m` buffer a caller sizing for a square decomposition allocates took a store past the end for every `j >= m`. Measured on a 2x3 A filled 1..6 into a 2x2 `out_R`: **returns 0 (`HSB_ERR_NONE`) with 2 of 8 guard slots clobbered** — the 16 bytes immediately after the data buffer — where the square 2x2 control over the same code path clobbers 0 of 8. Fixed 2.8.3: `out_Q` (`m x m`) and `out_R` (`m x n`) are both checked, and all three handles are null-checked before any dimension is read (separately a SIGSEGV — `cmat_rows(0)` is `load64(0)`, and `cmat_new` returns its designed `0` above `_CMAT_MAX_ELEMS`). The **shape is rejected, not coerced**; the rectangular case is handled, not rejected. `tests/abuse.tcyr`'s canary group now reads the overrun as a test failure (`got 2, expected 0`) rather than as luck |
| Unwritten out-parameters | A return path that leaves an out-param untouched publishes the zero page into the caller's slot — `alloc()` hands back zeroed memory, so the caller holds a NULL `HVec3` that faults on its first accessor (CWE-476) | `gjk_epa_3d` and `time_of_impact` (two paths) fixed 2.8.3; the sibling `mpr_penetration` was missed and fixed **2.9.1**, on every `0` return. Demonstrated rather than assumed: against the pre-2.9.1 body the new tangency group does not merely fail, it **aborts the suite** on exactly that read. Every path now writes a defined placeholder normal and depth 0, documented as a placeholder rather than a contact normal |

## Known Limitations

- `num_modpow` is NOT constant-time. Do not use for cryptographic applications.
- Jacobi eigensolver is O(n^5) worst case. Not suitable for n > 50.
- `svd_compute` / `svd_truncated` wrap the stdlib `mat_svd` (A^T*A), which squares the condition
  number. The Golub-Kahan replacement has shipped since 2.1.0 — use `svd_golub_kahan`
  (`linalg_precision`) for ill-conditioned input.
- PCG32 uses signed arithmetic with masking. Verified safe but not cryptographically secure.
- `m4_get`/`m4_set` do not bounds-check col/row arguments. Caller must validate.

## Supported Versions

| Version | Supported |
|---------|-----------|
| hisab 2.9.x (current) | Yes |
| hisab 2.8.x | Yes |
| hisab 2.0–2.7 | Best-effort |
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
- Full sweep 2026-08-03 (70 findings, 2 critical) — the four P-tiers were repaired across
  2.6.12–2.6.15, but **six findings never appeared in a tier and so were never scheduled**; the
  2026-08-04 re-audit found that gap and 2.7.0 closed all six. That re-audit came back **clean on
  regression** — every 2.6.12–2.6.15 repair holds under execution — and confirmed 42 further
  findings / 8 refuted, no criticals, of which 2.7.0 fixed **35 of its 41 numbered**, each of the
  other six carrying a written reason and a filed spec. See docs/audit/2026-08-03.md,
  docs/audit/2026-08-04.md and docs/development/threat-model.md
- Full sweep of the v2.8.0 tree 2026-08-04 — six dimensions, each swept by an independent agent and
  then **adversarially verified by reproduction** before a finding entered the report:
  **42 confirmed, 0 refuted** (4 critical, 17 high, 16 medium, 5 low). Three of the four criticals
  were in the collision narrowphase. Discharged across 2.8.2–2.8.4, with the low tier — never
  scheduled, worked as one pass — closed 2026-08-05 and the addendum row 2.9.1; the report carries a
  per-finding Disposition column and stands at **42 FIXED, 0 OPEN**. Every one of the 42 lived in
  code that 1801 assertions and 97% function coverage did not catch: coverage counts whether a
  function is *referenced*, not whether its contract is *checked*.
  See docs/audit/2026-08-04-v2.8.0-full.md
- **An audit's finding list is the unit of disposition**, not its tier summaries — closing on a
  deduplicated digest is exactly how the six unscheduled 2026-08-03 findings stayed invisible for
  four releases. Every dated report in docs/audit/ now records a disposition per finding
- Public entry points are exercised under **abuse**, not only under use: `tests/abuse.tcyr` (added
  2.9.0, **732 assertions**) drives negative indices, zero and huge dimensions, non-conformable
  operands, the designed-`0` return, degenerate geometry and heap canaries. It surfaced **11 real
  defects on public entry points**, held in a known-defect register rather than deleted. The register
  is discharged: nine repaired into live assertions, the tenth re-measured and reclassified
  `[BY DESIGN]`, and two of its own reproducers corrected in place rather than trusted
- Fixes are **mutation-proven**: a repair lands only once its source file has been reverted and the
  new assertions confirmed to fail. Where a defect is a cost rather than a wrong answer, that is
  stated explicitly and a benchmark guards it instead — a passing suite is not allowed to imply a
  proof it did not provide
- Hand-encoded IEEE-754 constants are verified against their own comments by
  `scripts/check-constants.sh` (a CI gate since 2.6.12, after seven mis-transcribed tables shipped).
  The gate was itself audited: until 2.7.0 its regex rejected `_` digit separators and it silently
  skipped **35 of 145** declarations while printing "110/110 verified" — one of the skipped constants
  encoded ~1e16 against a documented 1e15. Now **153/153 verified, 1 skipped**
- **A green gate is not evidence that it gated.** The release version check was
  `grep -q "$VERSION" CHANGELOG.md` — an unanchored regex, which `2.9.2` satisfied by matching
  `32,942,104 B` in an unrelated line, so a release could ship with no CHANGELOG entry at all.
  Anchored to `grep -qF -- "## [$VERSION]"` in 2.9.2, and CI now additionally asserts
  `src/main.cyr`'s printed version string and `dist/hisab.cyr`'s header against `VERSION`
