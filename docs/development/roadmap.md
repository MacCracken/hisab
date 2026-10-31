# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.6.20**. Stdlib `ganita` (6.2.x math umbrella) provides dense
> decompositions + transcendentals.

⭐ **This file is future-facing only.** Nothing below has shipped. The record of what *has* is:

| record | lives in |
|---|---|
| per-release changes, with the numbers | [`CHANGELOG.md`](../../CHANGELOG.md) |
| per-defect filings, in-tree and shipped | [`issues/`](issues/) and [`issues/archived/`](issues/archived/) |
| toolchain defects | `cyrius/docs/development/issues/` — **not this repo** |
| the evidence behind a closed item | [`audit/`](../audit/) |
| the evidence behind every open row below | [`audit/2026-09-30-deferment-audit.md`](../audit/2026-09-30-deferment-audit.md) |
| test counts and suite composition | [`../guides/testing.md`](../guides/testing.md) |

⚠ **Completed items are removed, not struck through.** This changed on 2026-09-11. The file had
grown to 1308 lines, about 95% of them history, and a backlog nobody can see the end of is not a
backlog. To learn why an item was closed, or that it closed differently from how it was scoped, read
the CHANGELOG entry for its release.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current — v3.3.7

**Status**:
- Suite **9023** across seven harnesses: hisab 827, foundation 699, modules 2396, modules_b 1148,
  modules_c 1165, edge_cases 777, abuse 2011. (3.3.7 split `modules_b.tcyr` in two; each half is
  about 450 KB of cyrlint/cyrfmt's 1028 KB input cap.)
- Constant gate **211/211** (3 skipped), read exactly since 3.3.7, hex enum members included
  (population floor 214).
- Public surface **703** declarations (846 gate probes; all 854 non-public probes refused),
  enforced since 3.3.0 by the bundle's `private` marker.
- **156** benchmarks.
- **35** math modules in `[lib]`, plus the `src/visibility.cyr` marker.
- Measurement provenance: **8** unmarked claim lines (detector false positives), the baseline;
  CI runs `--ratchet` on every push since 3.3.6.
- Toolchain **6.6.20**, sakshi **2.5.7**, ganita **1.2.14**.
- All gates green. Per-release detail is in [`CHANGELOG.md`](../../CHANGELOG.md).

Two releases broke the API, and each has a consumer guide:
- **3.0.0** moved to `Result<T, E>` with no deprecation window
  ([`../guides/migration-3.0.md`](../guides/migration-3.0.md)). **2.24.0 is the supported 2.x line.**
- **3.3.0** made the bundle private and removed the accidental public names
  ([`../guides/migration-3.3.md`](../guides/migration-3.3.md)).

⚠ **hisab ≥ 3.1.0 requires cyrius ≥ 6.6.3** (`public struct` + `#derive`). Measured again on the
3.3.7 bundle from pinned dirs:
- 6.6.2 refuses it with the known `#derive` error.
- Under 6.6.3, 6.6.6, 6.6.9, 6.6.10, 6.6.12, 6.6.13, 6.6.14 and 6.6.20 a cross-module consumer
  program (a cross product, `num_fft`, quadrature and bisection, two sphere rays, the SVD entry
  points on the duplicated-row, ×2^1021 and h = 2^1023 witnesses and under `ALLOC_MAX` 16,
  `lyapunov_max` on NaN, `eigen_symmetric`, `cx_powf(2, Inf)`, `ivl_sin`, a CGA product and norm)
  builds two ways, with the manifest's 16 stdlib leaves and with the sidecar's 7 plus `syscalls`,
  `string` and `io`, and runs. The outputs are identical on every pin except `svd_compute`'s S₀
  (ganita 1.2.14's last bit on 6.6.20, hisab's fallback below it) and `cx_powf(·, ±Inf)` on 6.6.3 /
  6.6.6 (NaN, their ganita `pow`). Earlier runs also found `ivl_sin` below 6.6.9 ([−1, 1])
  and the perspective focal term below 6.6.9 (x87 `sin`/`cos`). On 3.3.0, all 511 non-public
  probes were refused under 6.6.3, 6.6.4 and 6.6.12.
- ⚠ On 6.6.3 a private fn is still reachable through `&name`. That is upstream, fixed in 6.6.4;
  see *Decisions owed*.
- ⚠ **The sidecar names 7 leaves since 3.3.7** (`cyrius distlib` ≥ 6.6.18 writes a compile-verified
  `# Requires` block and `dist/hisab.deps`: math, alloc, vec, ganita, result, str, fmt). A consumer
  that received `assert`, `bench`, `args`, `io`, `tagged`, `fnptr`, `callback`, `syscalls` or `string`
  through hisab's sidecar must declare it.

## How to read this file

- ⭐ **Every open item sits under a pinned version heading or in an unversioned section.** A pinned
  version looks like `**[3.4.0]**`. *Demand-gated (3.x.x)* and *Parked* are the unversioned
  sections: they hold work with no driver yet. Nothing moves out of them without a consumer
  asking, and when one does, the item gets a pinned version here first.
- **The maintainer sets every release's number and contents.** The pins below are the plan of
  record, and they move only when the maintainer moves them.
- **Breaking changes are pinned inside 3.x like any other item.** 3.0.0 and 3.3.0 both broke the
  API. A break ships with a **Breaking** CHANGELOG section and a migration guide. A public name
  whose meaning changes is retired, not redefined, because consumers compile the bundle under
  their own pins.
- **`Dnnn` / `Knnn` are register IDs** in
  [`audit/2026-09-30-deferment-audit.md`](../audit/2026-09-30-deferment-audit.md). Each carries
  the sources (file:line), both skeptics' verdicts and the probes behind the numbers quoted here.

⚠ **An item's measurement is part of the item.** Rows here carry the number that scoped them.
Rows without one are how this file kept getting the size of a class wrong: for seven consecutive
releases a row named one or two sites and a tree-wide grep found five, six or fifty-one. **Grep for
the shape before sizing anything below.**

⛔ **An open row is not a verified one.** Every row below was re-verified on 2026-09-30 by two
independent skeptics, one told to prove it already done and one told to prove it is not hisab's
work. Premises still go stale: check the tree before believing a row in either direction,
including a row that says something is blocked.

---

## Pinned — 3.3.x patches (no public API change)

### **[3.3.8]** — the 3.3.7 findings, and the costs 3.3.7 did not buy back

**Found during 3.3.7, not repaired (not on the [3.3.7] rows).** Each was found by a 3.3.7 lane or
reviewer on the 3.3.7 tree under cycc 6.6.20 and reproduced by whoever reported it; "both trees"
means 3.3.6 gives the same answer. Unlike 3.3.6's list, these were not re-verified by a second
skeptic: do that before sizing a repair. Review and repair each here.

*Costs to buy back* (same-boot A/B, CHANGELOG 3.3.7 *Performance*; 3.3.5 → 3.3.6 → 3.3.7)
- **A ray origin on an AABB or OBB face**: 78 → 117 → 117 ns and 296 → 332 → 337 ns. The underflow
  check of a zero crossing is the whole cost (without it, 72 ns). An exponent filter in place of the
  divisions moved nothing (106 ns), one 9-argument check without calls reached 105 but OBB 328, and
  the check inlined with its locals reached 95 but slowed `ray_aabb_world` 51 → 61 ns and the
  ±DBL_MAX box 74 → 79 (cycc's five-register picker reassigned the slab locals). Needs a cheaper
  check shape.
- **`calc_integral_trapez` / `_simpson`** keep +6% to +9% over 3.3.5 (147 vs 135 ns; 335 vs 316):
  the per-node `if (p != 0)` and the x0 local. A build without the per-node test gave 139 / 327 ns;
  removing it needs a second copy of each loop (simpson's carries the overflow-halving logic).
- **3.3.7's ray repairs**: `ray_capsule_diag` 536 → 678 ns (+26.5%), `jet_capsule` 722 → 900
  (+24.7%), `jet_sphere` 195 → 212 (+8.7%), `jet_triangle` 474 → 503 (+6.1%); a barrel ray from far
  along the axis +57% (the compensated o_perp, ~155 ns where it fires). A near-surface sphere origin
  is 208 ns against 57 at 3.3.5 (the exact c); a continuation that skipped the full path's recompute
  reached 174 ns but slowed the ordinary `ray_sphere` 60 → 69 ns.
- **3.3.7's collision repairs**: `gjk_epa_boxes` +13.1%, `mpr_penetration_boxes` +11.9%,
  `gjk_epa_sphere_box` +8.2%, `gjk_epa_3d_cyl_box` +8.1% against 3.3.6. Faces 1e-10 and 1e-11 deep
  still run the shallow polish when its gap exceeds 1/8 of the depth: 1.25× to 1.9× the calls for an
  error that moves only 1.9 → 1.5 (1e-10) and 15 → 3.9 (1e-11) of the depth. A face normal is the
  cross product of two crease jumps; snapping to it would be exact for polytope cores.

*Linear algebra*
- **`eigen_power` answers above ρ(A) on non-normal triangular matrices** (8 of 1,000 in a
  triangular family): 3×3 rows `371B5E3E602B33DF EB649BD1AEEB8399 654A5AEE5D9850EA / 0
  B728AD3EB2747301 0 / 0 427336E7F5A31AA7 B7205AF06B3D7EB8` returns Ok(0xB728AD598B5A488C) =
  −5.5328078e-43; the eigenvalues are the diagonal, dominant −5.5327160e-43, so the answer is 1.7e-5
  relative off and larger in modulus than every eigenvalue. When every strongly connected component
  is a singleton the spectrum is known exactly, so a ceiling (an answer within tol of some a_ii)
  could refuse these; no floor can.
- **`eigen_power` with a complex dominant pair inside a reducible matrix** (7 of 8 such rows wrong):
  4×4 `B8350A7018574CEB 383E22F02D064C7F 0 0 / B83E22F02D064C7F B8350A7018574CEB 0 0 / 0 0
  3831EEDF183EEFEA BB23DB28B4E134C6 / 0 0 0 3840D712CBCE1F36` returns Ok(9.8977e-38) = a33, while
  the 2×2 block's eigenvalues are −6.18e-38 ± 8.86e-38i (modulus 1.08e-37). A 2×2 strongly connected
  block's modulus is √det when its discriminant is negative, which the Tarjan pass could supply.
- **`eigen_power` refuses any dominant |λ| below about 2.47e-312**: the relative test
  `|new − old| < tol·|new|` underflows to `< 0`. [[x]] with x = 0x00000000000ABCDE (3.5e-318) at
  max_iter 1000 and x = 0x00000000ABCDEF12 at 100,000 give Err(−8); 0x000001DDB5ED82B0 (1.01e-311)
  gives Ok(x). The doc block does not state the limit (`src/linalg_ext.cyr`, the convergence test).
- **`eigen_power`'s irreducible class** (no singleton component) stays open: a |tr A⁴|/n floor
  catches 45 of 3.3.7's 263 constructed wrong answers (44 of them the diagonal floor already
  catches) for n³ multiply-adds per answer (+67% of `eigen_power_12x12`'s matvec work).

*Numerics*
- **238 of 364 allocation sites are unchecked across 30 modules**, 63 of them sized from a variable
  (a heuristic scan: a variable assigned from `alloc` / `_f64arr_alloc` / `arena_alloc` and never
  compared with 0 in its function). The variable-sized ones need triage first: diffgeo
  `geodesic_rk4` / `parallel_transport` (dim, 20), linalg_ext CSR / GMRES / BiCGSTAB (24), tensor
  (6), collision_mesh halfedge / union-find (6), calc_ext `_calcx_mc_ext` (3, bounded against
  `ALLOC_MAX` but not checked against a refused allocation). Fixed-size sites fail only on an
  exhausted heap.
- **`_toi_outside_plane`** (`src/geo_advanced.cyr`) uses the product sign test 3.3.7 removed from
  `num_bisection`, `f64_gt(f64_mul(sp, sd), 0)`: an underflowed product reads as coplanar, which its
  comment calls the conservative side (one extra face examined). Not probed.
- **`opt_conjugate_gradient` below 2^-495**: f falls under DBL_MIN near the minimum and the Armijo
  test cannot measure a step. Needs a derivative-based line search (Wolfe, or a secant on
  g(x + αd)·d). Also: on the 1e5-conditioned fixture CG needs 300 iterations at k = 1 where BFGS
  needs 7, and PR+ with an Armijo-only search produced a non-descent direction at k = 2^504
  (iteration 1); a restart when d·g ≥ 0 is the standard guard (it would move in-band bits).
- **`calc_adaptive_simpson` returns NaN for an integral past DBL_MAX**: f = x·2^1003 on
  [−2^20, 2^21] (rule and integral 1.5·2^1043) returns NaN after 5 evaluations (3.3.6 too); the
  symmetric [−2^20, 2^20] returns the exact 0. `calc_integral_simpson` / `_trapez` return ±Inf for
  an overflowing true sum.

*Rays*
- **`_geo_sphere_roots`' fast path lets a·δ underflow**: in its `c >= ff` branch δ and a are each at
  least 2^-968, so big_d can flush to 0 and the tangent point −hb/a comes back. `S 266D80528E570290
  2640C274DD7C67D8 A63BD31CFE844670 B4CDD136A0538570 B4C1E9C95A16434E B4556284E3BE0DBB
  264C361F40E124F8 260557CC0268B800 25F2BC5A786B3D80 2653E5D7389D1D47` (o, d, centre, r): exact
  0x317F734637AA02EF, both trees 0x3182C719D9D92556 (19% off); scaled by 2^409 or 2^179 both are
  right. As a capsule (start `A6624EA215115806 A660AA8F79F6B7D3 A6557376B7C23C24`, same end and ray)
  3.3.6 answered a false barrel root 0x319009B0722EB8FC and 3.3.7 answers 0 (exact END
  0x317F734637AA02EF). Repair suggested by the review: hand off to `_geo_sphere_roots_full` when
  big_d < 2^-968, as the `c < ff` branch does for dc.
- **Thin barrels beyond 2^100 radii from their start** need more than double-double: 1,037 of 2,478
  wrong (3.3.6: 1,678). `C 35857EAD7683670C B57C831B1D037CCA 3FFE9CAF93B0BFF3 0 0 BFD3333333333333
  BFD0D5F4A1BF108C BFDC22E7A91CD28A BFDCE33C8E195248 3FD0D5F4A1BF108C 3FDC22E7A91CD28A
  3FDCE33C8E195248 357B598E926EB69D` (o, d, start, end, r; |o − start| = 2^168 r): an exact miss;
  3.3.6 0x401982924B798BAA, 3.3.7 0x4019829250689FF4 (both false hits).
- **A sphere origin on its surface with a ray grazing there** (cos(oc, d) ≤ 0.17): hb = oc·d cancels,
  4–110 ulp (79 of a 4,000-ray family). `S 487602669DB9C14E 4BE07CD947E59420 0 BFEF8DE3667D2A6A
  BFE6F8E578F5B885 BFD809F00CD795E2 4BD8BB45EBD85E30 0 0 4BE49C0F99DEF928`: exact
  0x48C7CDFEAF272978, 3.3.7 0x48C7CDFEAF27296F (8.9 ulp; 3.3.6 0, the contact).
- **A barrel origin just outside its surface**: c = |o_perp|² − r² from a rounded o_perp (it would
  need o_perp as two doubles), 11.7 ulp on both trees: `C 925EA7EEFAD6162C 925E8890FC4203E4
  124D7E0369DB754C AD2689EC800B377A 3FC85814D48BDA5F AD25E2A03313AF87 9268000000000000
  1260000000000000 0 927161BED61BED62 0 922745D1745D1746 126967B9F957588B`: exact
  0x124F5DF0E31E9F97, both 0x124F5DF0E31E9FA3. The same class gave the six verdicts of 3.3.7's
  5,000-origin surface family that went right → wrong (origins under an ulp from the surface).

*Collision*
- **Separated pairs read as depth-0 touches** (both trees, all four entry points): sphere/box-edge and
  capsule-side pairs 1e-12 of the size apart, 55 / 1,600 and 42 / 1,600 queries (5 at 1e-11) — wider
  than the documented half-ulp class. Box c BFD61257EB6AADD8 BFDB382E360A6F08 BFE1B090E0DA5D8C, h
  3FE10E5D921C4E9E 3FF067BEBE3E2F80 3FFBFA0DDE29B88B, e0 3FDD71EC3BC1CA08 3FEC6333A41698AC
  3FA2E9C4C599C230, e1 BFE25D3B836AD314 3FD0FBCC7D2B148E 3FE8CAC41890BDFE, e2 3FE5ADF93F7FFD11
  BFD82B57E862939C 3FE432AC1CEEBD8C; sphere c BFF10BAE64A09C30 3FE608C85C5CA8EF 3FD24679A7A58712, r
  3FD782FA1BDAC874: exact gap 9.9994e-13; `gjk_intersect_3d` 1, `gjk_epa_3d` depth 0. Most of them
  come from the GJK core itself, before any portal or proof: 10 of 13 such contacts on the
  sphere/box-edge family 1e-14 apart, 7 of 8 on capsule/sphere 1e-14 (3.3.5 answers the same pairs 1).
- **A small sphere on a box corner reads as a depth-0 touch** (both trees): radius 1e-6 overlapping a
  corner by 1e-9, 1–5 of 100 per mode, normal more than 0.1 rad off. Box c 3FD7489AF806C39C
  BFEC4084DC4F2D7A 3FD0995A0B8B02DC, h 3FF142450839E5A7 3FFADB6A606B0209 3FF793A4464D881C, e0
  BFE55282FC826F62 3FABA2446D402F18 BFE7CC74D74BE8D4, e1 3FE7AFBEBB0DBAE0 3FC58CD492AD23FC
  BFE4D4B22C35A3AE, e2 3FB70F46B4F3D590 BFEF7EF32CD2E9CC BFC37915E9946F70; sphere c BFD2BAF1075B81CE
  3FCD02554AC1F5F4 4003070E125C2EBB, r 3EB0C6F7A0B5ED8D: `gjk_epa_3d(sphere, box)` depth 0 (exact
  1.0e-9). 3.3.7 also spends 3.2× / 2.7× the calls on this family at 1e-9.
- **Sphere-on-box-face contacts 1e-9 deep are up to 12% too deep** (3.3.6: 22%): a face normal is a
  vertex-type minimum of h, EPA certifies at 1e-10 of the shape, and the pattern search cannot reach
  below its floor. Box c (0x3F8E166792D29900, 0x3FE3445D3F532334, 0x3FE0C0386E64E880), h
  (0x3FF2FD63699B3F08, 0x3FE7C7FBDA9ED738, 0x3FF111049041DCE8), q (0x3FD6C81C22CA2B18,
  0x3FD3F882420CFAC5, 0x3FDDD94F6700C7D0, 0xBFE7E99278B4FB98), sphere c (0xBFF29FE8714B4DF3,
  0x3F5B1DF9BD736200, 0x3FEB1E57B758C839), r 0x3FE35FA860613C3E: exact depth ~1.0000e-9, both trees
  0x3E133057A4000000 (11.7% deep). The face snap under *Costs* would make it exact.
- **`time_of_impact` normals on box edges and capsule sides** remain up to 1.7e-4 (oblique) and
  5.9e-4 (head-on) rad from the closest-approach direction (p99 4.2e-5 / 1.5e-4; capsule sides
  1.5e-4), both trees. 3.3.7 repaired corners only.
- **At coordinates ~1e3, `gjk_*` and `mpr_*` disagree on shallow overlaps below the proof margin**:
  200 cap pairs 1.2e-12 deep at offset 1e3, `gjk_*` miss 187 / 196 and `mpr_*` 18 / 17 (both trees):
  MPR keeps an unrefuted portal, GJK needs a proof the margin (1.4e-11) forbids.

*Instruments and tests*
- **`check-measurements.sh` reads a free-text marker that begins with a dotted number as a path**:
  `[measured: 3.3.7 python3 fractions oracle (scratch)]` gives "BROKEN … names a file that does not
  exist: 3.3.7"; `[measured: scratch … (3.3.7)]` passes. The path regex is `^([\w./-]*\.\w+)` in
  `check_marker`.
- **Dead code in the harnesses**: `tests/hisab.fcyr`'s `fuzz_read_int`, `fuzz_is_nan` and
  `fuzz_approx_eq`; the globals `qt_in` (modules.tcyr, quadtree group), `IVL_TOL_M` and `_rq5_init`
  (modules_b.tcyr).
- **Mutation gaps**: m0157 (`_numx_dft`'s zero-fill at `bv − 8`, an out-of-bounds write that is
  harmless mid-chunk and exits 139 when `bv` opens a fresh 256 MB chunk) is unpinned because its only
  kill reads allocator internals;
  `gjk_intersect_3d`'s new refutation at h ≤ 0 instead of h < 0 survives (h exactly 0 along the EPA
  normal never occurred in 25,200 queries; a tangency is answered by the touch probe first) and is
  not proven equivalent.
- **`delaunay_2d` may never reach `_col_dl_locate_scan`**: an exit planted at its entry never fired
  in any suite (positive control fired). 3.3.7 tests it directly; whether `delaunay_2d` can reach it
  is unproven — find an input or remove the fallback.

---

## Pinned — 3.x minors (API and observable behaviour)

### **[3.4.0]** — error channels for the fabricated-answer class (breaking)

Each function below returns a value that is also a legal answer when the operation has failed.
- **How the break ships**: each gets a Result-returning replacement, and the old name is retired,
  not redefined. A **Breaking** CHANGELOG section and `docs/guides/migration-3.4.md` go with it.
- **Prerequisite met**: `check-result-migration.sh` sees forwarders since 3.3.1 (`D122`).
- **Before the API work, two audit tasks**:
  - Run a full multi-dimension sweep. The last one covered v2.11.0 (2026-08-11), and 3.0.0, 3.1.0
    and 3.3.0 have shipped since. Its "did NOT reach" list was never discharged, including the
    SIMD `f64v_*` paths and cross-module interaction. `D102`
  - Reconcile the 2026-08-11 audit's confirmed findings against the censuses, row by row;
    `solve_bicgstab` fell between the buckets. `D103`

**The functions.**
- **`svd_compute`** returns 0 / −1 / −2 as integers that alias hisab codes. Since 3.3.7 that holds
  under every pin (ganita 1.2.12's −3 never escapes), and −1 means an allocation failure, a fallback
  that could not answer, or a σ proven past the overflow line. It becomes a Result. `D012`
- **`solve_bicgstab`** and the Krylov family should report breakdown and non-convergence instead
  of returning the current `x`. Until then the contract is "an unconverged stop returns the last
  iterate" (the maintainer's 3.3.4 decision): 941 of 43,000 runs in the 3.3.4 sweep return a finite
  wrong iterate, and the exact-zero-t stop returns an unreported half step (36 wrong of 549).
  Operand scaling lands with the error channel. `D015`
- **`calc_adaptive_simpson`, `ode_backward_euler`, `ode_bdf2`, `ode_bdf`** should report
  non-convergence. `calc_ext`'s "no Result type" reason was false from 3.0.0, and `ode.cyr`
  documented a return value the code doesn't give; 3.3.2 corrected both comments. 3.3.3 made
  `calc_adaptive_simpson` return NaN for an unreachable tolerance (`D071`); the Result form replaces
  those NaNs. Also: `calc_adaptive_simpson` still walks its depth cap on integrands with zeros or
  evaluation noise above the 3.3.4 relative floor (5x^4 − 3x^2 + x − 1/4 at tol ≤ 1e-20, Gaussian
  tails: over 3,000,000 evaluations), and `num_bisection` returns Ok on max_iter exhaustion. `D070`
- **Plausible-value fallbacks**: `hisab_inverse_lerp` (returns 0), `world_to_screen`
  (`hvec3_zero`), `calc_monotone_cubic` (0), `calc_partial_derivative`, `calc_hermite_tcb`,
  `ode_bdf`. **svara** works around `calc_monotone_cubic`'s 0 with
  `if (r == 0) { return F64_ONE; }`. The dual sentinels stay, because they are a documented miss
  contract. Also: `f64_fmod(x, ±0)` returns +0 for every x, NaN and ±Inf included (C: NaN; kept
  when 3.3.4 made the rest of `f64_fmod` exact); `fbm_2d` / `fbm_3d` / simplex fBm return +0 for
  octaves ≤ 0; `dual_ln` / `dual_sqrt` / `ad_ln` / `ad_sqrt` map NaN to the (0, 0) miss sentinel;
  `num_pollard_rho(n ≤ 1)` returns n. `D069`
- **`expr_eval`** returns +0.0 and writes to fd 2 on an undefined variable or unknown tag. That
  write is the only I/O in library code. Return NaN or a Result, and drop the write. `D074`
- **`m3_inverse` / `m4_inverse`** return the identity for singular or NaN input.
  `m3_normal_matrix` inherits this, and `m4_transform_point` returns undivided coordinates at
  w = 0. `hquat_inverse(0)` and `hquat_normalize(0)` return the identity too (left unchanged in
  3.3.4, which repaired their subnormal inputs only). `D088`
- **`lyapunov_max`** returns `Ok` with exponent 0 when `iterations ≤ 0` or the Jacobian has no
  growth at all. Its doc block does not say so. `D032`

**Contracts to write** (each re-measured in 3.3.7; what the right answer is needs a decision first).
- **`geo_barycentric_coords` on an exactly colinear triangle**: the Gram determinant is 0 and the
  centroid fallback answers (⅓, ⅓, ⅓) whatever the point. 400 full-mantissa colinear triangles with
  p on the line: 134 exact-zero determinants answer the centroid, and none of the 400 reproduces p
  (worst residual 3.32), both trees.
- **`hquat_rotate_vec3` for non-unit q** computes v + w·t + u×t (t = 2·u×v), a rotation only for
  unit q, neither q·v·q⁻¹ nor q·v·q* otherwise. Out of a 6u·S bound for subnormal v: 0 of 167 for
  |q| ≤ 2, 108 of 288 for (2, 16], 511 of 545 for (16, 1024].
- **Support functions noisier than 2^-46 of the coordinates** (the premise 3.3.6 documented): 1,200
  separated pairs through K = 2^12 / 2^16 / 2^20 ((c + K) + r·n − K): 24 / 154 / 311 contacts from
  `gjk_*`, 23 / 91 / 178 from `mpr_*`, none at gaps of 1e-10 and wider. State what a support
  function owes.
- **A capsule with an infinite end**: of 20,000 special-value capsule rays, 3 with an infinite start
  moved in 3.3.7 through the finite end cap's repaired root, e.g. `C C01806A97BEF2F00
  C022A529106B66CC FFEFFFFFFFFFFFFF BF868F7EF0F424FE 3FCA497068AC434E BFD46BD394C2B5F8
  BFF0700D7ED5A51E BFC7FF308E5DC880 7FF0000000000000 BFF3AE4A48143454 3FFB25F992C4CB5C
  3FE14CDA87A91BF8 FFEFFFFFFFFFFFFF` (o, d, start, end, r): 3.3.6 0 / END_CAP, 3.3.7 NaN / START_CAP.

### **[3.5.0]** — constants and conventions (breaking)

- **23 `public var` constants that any consumer can write**: `GJK_MAX_ITER` and `EPA_MAX_ITER`
  (EPA sizes its buffer from the latter), `EPSILON_F64`, `F64_TINY`, `F64_POS_INF`, the
  `GEO_JET_*` / `GEO_CAP_*` / `GEO_FACE_*` tags, `AD_NONE`/`AD_FULL`, `HALFEDGE_NONE` and
  `CGA_NUM_BLADES`/`CGA_MV_SIZE`. Probed: `GJK_MAX_ITER = 0` set from a consumer takes effect.
  Convert them to enums, per CLAUDE.md's "enums for constants". `check-constants.sh` reads hex
  members of multi-line enums since 3.3.7, so the verified constants stay in its population. The
  `_NUMX_DFT_MAX` test in `modules_b.tcyr` lowers that cap by assignment; it needs a seam (the cap as
  an argument) before the cap can become an enum member. `K003`
- **`cx_powf` with a zero base**: decide hisab's 0⁰. `cx_powf` gives 0, while `num_modpow` and
  `dual_pow` give 1. `D019`
- **`linearize_depth_reverse_z(0)`** returns 0, which means the eye, for the infinite far plane.
  IEEE and continuity with the subnormal neighbour both say +Inf. Three assertions pin the 0.
  `D096`
- **`t2d_apply`** takes an `HVec2` and returns `hvec3_new(x, y, 1)`, which tests read back through
  HVec2 accessors. None of this is documented (audit 2026-04-15, L4). `D090`
- **`sym_integrate`** returns 0 ("unsupported") for var-free integrands (`sin(y)`, `ln(2)`, `y^2`,
  `x^y`, `2^x`) and for `ln(x)`, though x·ln x − x can be built from existing nodes. The fix is
  additive. `D076`

### **[3.6.0]** — SVD relative accuracy

- **`eigen_qr`'s 1e-12 deflation** (`_lp_negligible`) is what puts 251 eps·|λ|max into its answer
  on the 4×4 near the overflow line (3.3.6's row; the same matrix at 2^-100 to 2^-300 gives the same
  bits scaled), and an in-range family of its shape reaches 711 eps (median 0.97, p99 53). A private
  2^-52·(|d_k| + |d_k+1|) threshold takes both to ≤ 5 eps but moves the bits of 1,909 of 6,000
  in-range rows and costs +11.7% QR sweeps; LAPACK's form moves 3,752 and makes 5 sparse rows
  NO_CONVERGENCE. Decide it with `D001`; `eigen_qr` carries a ⚠ paragraph saying so.
- **`svd_golub_kahan` refuses some fitting matrices near the overflow line**: 425 of 3,030 sweep
  rows whose σ₁ fits within 9 units of 2^-53 of the line return NO_CONVERGENCE at the 3.3.3 rescale
  (the computed σ rounds past), as on 3.3.6.

- **`svd_golub_kahan` on a graded bidiagonal `[[t,t,0],[0,t,1],[0,0,1]]`**: every singular value
  is right to 4.4e-16 relative through t = 2^-39 (1200-digit oracle). At 2^-40 it returns
  `rc = OK` with an exact-zero singular value where the truth is 2^-41.09, the middle one 19% low,
  and ‖A − USVᵀ‖_F = 9.1e-13 (~2900 ε, so not even backward stable). At every t from 2^-41 to
  2^-1022 two exact zeros come back. LAPACK gets all of these to full relative accuracy. (Through
  3.3.1 this row said "through 2^-36" and "from 2^-44"; 3.3.2 re-measured it.)
  - **Candidate causes**: the `EPSILON_F64 = 1e-12` deflation and zero-diagonal snap
    (`_lp_negligible_rel`, documented at `linalg_precision.cyr:687-691` as snapping values "where
    LAPACK's dbdsqr would keep it"), and the lack of dqds or zero-shift QR.
  - **Rules for the fix**: use a private tolerance and never redefine `EPSILON_F64`. Commit a
    graded-bidiagonal oracle sweep first, then A/B against 3.3.0. `D001`, `D009`
- **Since 3.3.4 this residual reaches `svd_compute` / `svd_truncated`**: they answer ganita's −1
  (rank-deficient square input) through `svd_golub_kahan`. On 7,012 such rows the worst
  ‖A − U S Vᵀ‖_F is 4,820 ε‖A‖_F (ganita's own answers reach 12.9), with S within 7.75 ε·σ₁.
  Re-run that probe as part of this row's A/B.
- **Commit the high-precision oracle for the subnormal-boundary rows.** `tests/hisab.tcyr:4209-4220`
  asserts only `rc`. The committed fixtures show 2 rows at 3 ulp, and a wider band reaches 14 ulp.
  `D003`
- **SVD/eigen B-side lift on first attempts** (moved from [3.3.5], whose row deferred it here):
  lifting a sub-DBL_MIN rotation on the bidiagonal too fixes 27 targeted wrong rows (up to
  1.75e13 eps) and moves 2 rows slightly (3.18 → 6.61, 1.6 → 1.72 ulp). Decide it with `D001`.
- **3.3.5's last-attempt floor is normwise only.** When no headroom is left, a failed
  `svd_golub_kahan` / `eigen_qr` gets one last attempt that zeroes couplings below DBL_MIN × the
  largest entry; on inputs 3.3.4 refused, a decoupled block below about DBL_MIN·max can lose all
  relative accuracy, sign included (an eigen_qr 5×5 returns its block's diagonal, 6.180e-101 and
  2.718e-101, where the truth is 1.1729e-100 and −2.8306e-101; block relative error median 31% svd,
  54% eigen on 300 / 200 such inputs). A reviewer's prototype — apply the floor only on a stall
  (more than 2n passes without deflation) and only to the active window — answers the same rows
  with block errors of 2e-14 / 1.8e-13, but moves one probe row to ~2^20 ε·max; re-measure it here.
  Also: with the last attempt in place the growth constants decide only the last scale tried, and
  4n returns some subnormal-grid values more accurately than 8n (a 25u eigenvalue exact at 4n, 24u
  at 8n).

### **[3.7.0]** — the abaco bridge (additive; the pin is the maintainer's to move)

Scheduled proactively by the maintainer on 2026-10-03, so abaco can build its Rust-era plan
(`solve x^2 - 2 = 0`, interval-wrapped results) on hisab without having to ask. It adds public
names, so it needs a minor. It depends on [3.4.0] `D074` (`expr_eval` reads an unbound name as 0).

- **What Rust 1.3.0 shipped** (`src/symbolic/bridge.rs`, commit `bc09eb9`, unchanged through 1.4.0):
  `solve_expr(expr, var, opts{x0 = 0, bracket, tol = 1e-12, max_iter = 100})` ran Newton on the
  expression and its simplified derivative, with a bisection fallback only when Newton erred and a
  bracket was given (that fallback returned Ok(midpoint) on exhaustion); `eval_verified(expr,
  intervals)` evaluated a tree over intervals. `ExprValue` and its converters were transport only
  and stay out of scope (port-audit).
- **Port onto hisab's expression TREE** (abaco builds the tree; hisab has no tokenizer), each
  `#must_use`, returning a Result and writing `out` only on Ok:
  - `sym_solve(e, var_name, vars, x0, tol, max_iter, out)`: refuses an unbound name, a non-finite
    x0, tol <= 0 or max_iter <= 0 before evaluating; then Newton on `e` and
    `expr_simplify(expr_diff(e, var_name))` under `num_newton`'s contract.
  - `sym_solve_bracket(e, var_name, vars, x0, a, b, tol, max_iter, out)`: Newton, then bisection on
    [a, b] when Newton errs or leaves the bracket; refuses a non-finite f(a)/f(b) or equal signs, and
    returns `Err(NO_CONVERGENCE)` on exhaustion (unlike `num_bisection`).
  - `sym_eval_interval(e, ivars, out)`: interval evaluation through `ivl_*`; needs new public
    `ivl_cos`, `ivl_ln` and `ivl_pow` that are sound for every base sign (integer exponents by
    parity, negative exponents through `ivl_div`'s unbounded rule, non-integer exponents and ln only
    on lo > 0).
- **Do not port Rust's defects**: its Pow corner rule is unsound across 0 ([-2, 1]^4 gave [1, 16];
  the range is [0, 16]), and a free variable read as NaN (hisab's `expr_eval` reads 0, `D074`).
- **Implementation notes, measured 2026-10-03**: routing through `num_newton` with the evaluator as
  capturing closures built inside the hisab fn solves x^2 - 2 from 1.5 to 0x3FF6A09E667F3BCD,
  bit-exact under cycc 6.6.3 and 6.6.14. Keep those closures inside a fn (the top-level `fncallN`
  crash) and never `return` a `: stack` value from them (`D082`): both are fixed in cyrius 6.6.16,
  but abaco pins 6.6.12. abaco 2.4.12's bundle and
  hisab's compile together under abaco's pin 6.6.12 with no duplicate symbol.
- **Tests**: Rust's cases as bit-exact fixtures (√2), a bracket case that really reaches the
  fallback (x^2 - 2 from x0 = 0 on [1, 2]), the unbound-name and NaN-endpoint refusals, exhaustion,
  and [-2, 1]^4 ⊇ [0, 16].

---

## Decisions owed — the maintainer's

Nothing in this section is scheduled until the maintainer decides it.

1. **2.24.x support.** SECURITY.md lists 2.24.x as supported, but only the 2.24.0 tag exists, and
   it carries known defects:
   - the CWE-190 `num_*` overflow above 2^62
   - the non-terminating `num_pollard_rho` and `num_divisor_sigma(2^63−1)`
   - the `_sh_within` wrap
   - the `dual_pow` subnormal 0

   No consumer pins 2.24.x; the 2.x consumers are on 2.22.1 and 2.11.2. Backport, or mark it
   best-effort with an end version. A backport must not take 3.3.4's revert of
   `_cga_build_null_tbl`'s guard form: 2.24.0 pins cyrius 6.6.2. `D148`
2. **Minimum cyrius 6.6.4?** On 6.6.3, `&_private_fn` still reaches private bundle functions
   (`&_num_mulmod` probed), and six direct and three transitive consumers pin 6.6.3. 3.3.0 kept
   the floor at 6.6.3 and documented the hole. `D144`
3. **aarch64.** `f64v_dot` is fused (`fmla`) on aarch64 and mul+add on x86. So `hvec4_dot` and
   `hquat_dot`, and through it `hquat_slerp`, give different bits on the two targets. CLAUDE.md's
   Language rule names only `f64v_axpy`/`f64v_fmadd`. Update the rule? An aarch64 test run is
   demand-gated below. `D120`
4. **CI consumer-pin matrix?** The public-surface and consumer-build gates run only under 6.6.20.
   The multi-pin build is a manual release step, which is how 3.3.0's `_SYM_EPS` leak under 6.6.3
   was caught. Automate it? `D110`
5. **Missing local tags.** The CHANGELOG has releases 3.2.0, 2.11.1, 2.9.3, 2.2.2 and 2.2.1 with
   no local git tag. The remote was not checked. `D164`
6. **Three [3.3.5] rows landed differently from their text** (CHANGELOG 3.3.5 has the evidence):
   - `num_bisection` refuses a NaN f(a) / f(b) but still bisects ±Inf ones (the row said
     "non-finite"; refusing ±Inf broke four correct answers, e.g. ln on [0, 2] → exactly 1).
   - `calc_adaptive_simpson`'s floored acceptance is kept: every stricter rule measured either kept
     the aliased repro wrong or walked other integrands past 200M evaluations. A tripwire pins the
     wrong 1.1 the aliased integrand returns.
   - Overflowing quadrature now takes the walk its in-range twin takes: calls that returned +Inf
     after 5 evaluations can run past 2·10⁹ evaluations (> 60 s), the [3.4.0] `D070` class.
   Accept, or schedule a different outcome.
7. **`ivl_contains` / `ivl_overlaps` on a NaN endpoint** answer 1 since 3.3.7, which [NaN, NaN]
   already answered (cannot exclude); the answer was chosen with 3.3.7's uniform NaN rule, not by the
   maintainer. IEEE 1788's NaI semantics would answer 0, for [NaN, NaN] too. No hisab module, example,
   harness or consumer calls either predicate. Keep 1, or move both to 0?
8. **The DST / DCT Bluestein cap (memory policy).** Lengths that are not powers of two stay direct
   past 2^20. A Bluestein reduction holds 32m + 16·len bytes, at most 80 MiB under the cap;
   `num_dct(2^20 + 1)` through Bluestein would hold 144 MiB (8 MiB direct) for about 7,700× fewer
   operations, and the allocator's own bound would allow 5 GiB per call, never freed. Where should
   the ceiling sit? If it moves, `modules_b.tcyr`'s three `_numx_bluestein_capped` assertions and its
   `_NUMX_DFT_MAX = 0x100000` restore move with it.
9. **The box-corner contact normal.** 3.3.7 adopted [3.3.6]'s proposal, "the closest-approach
   direction at the reported t" (`time_of_impact` and `gjk_epa_3d` on corners). Confirm it as the
   contract, or name another.
10. **`_kd_select_median`'s tie placement** is unspecified (mutant m0185 changes which of two equal
    keys goes left). Should the kd-tree promise a tie-break?

---

## Toolchain, tracked upstream

⚠ **Nothing is filed upstream without the maintainer's approval.** Once approved:
- cyrius defects go to `cyrius/docs/development/issues/`, and ganita's to
  `ganita/docs/development/issues/`;
- hisab keeps a record of its exposure in `issues/`, and closes it in `issues/archived/` with a
  paired measurement on the bump that fixes it. The upstream agents never edit hisab.

✅ **All four hisab-filed items closed in 3.3.7** (cyrius 6.6.20, ganita 1.2.14), each archived in
`issues/archived/` with its paired run on 6.6.14 and 6.6.20:
- cyrius: `fncallN` on a capturing closure at top level (exit 139 → 0) and `D082`, a closure's
  `return <: stack call>` booked against the enclosing fn (exit 3 → 0), both fixed in 6.6.16.
  `src/autodiff.cyr`'s recipe notes stay while any consumer pins below 6.6.16, which every direct
  consumer does.
- ganita: `ganita_mat_svd` accepting non-finite input (exit 18 → 0) and not converging on tiny
  columns (exit 9 → 0), both fixed in ganita 1.2.12 (folded as 1.2.13 in 6.6.16 and 1.2.14 in
  6.6.19). Its return codes moved (−1 allocation only, −2 contract, −3 non-convergence); 3.3.7's
  `svd_compute` / `svd_truncated` handle all three, and `_lext_mat_all_finite` stays for consumers
  whose ganita is 1.2.11 or older.

⚠ **Reported to the maintainer in 3.3.7, not filed** (filing is the maintainer's call):
- ganita 1.2.14's `ganita_mat_qr` returns status 0 with −Inf in Q and R (R00 = Q00 =
  0xFFF0000000000000) for the finite all-equal 2×2 with a = 1.5·2^1023, under cyrius 6.6.20; at
  a = 1.5·2^1022 its Householder overflows internally. hisab's `lyapunov_max` refuses both since
  3.3.7.
- ganita 1.2.14's `ganita_mat_svd` returns status 0 with σ₁ = DBL_MAX for [[h, h], [h, h]],
  h = 2^1023, whose σ₁ = 2^1024 overflows (1.2.11 returned +Inf). hisab's wrappers certify and
  refuse it since 3.3.7.

✅ **ganita's `atan2` filing closed in 3.3.4** (ganita 1.2.11, cyrius 6.6.13): the tripwires are
flipped, the D132 rows are pinned as C99 values, and the record is in
`issues/archived/2026-09-30-ganita-atan2-signed-zero-and-nan.md` with its paired run. `D140`, `D132`

---

## 3.x.x — demand-gated (unversioned)

Each row names its driver. A driver arriving gets the row a pinned version above.

**Private flip and kernels.**
- **Flip the `src/` modules `private` too, not only the bundle.**
  - 3.3.0 made `dist/hisab.cyr` private through `src/visibility.cyr`, which is every consumer's
    view. The modules themselves stay unflipped, so a caller that `include`s `src/*.cyr` one at a
    time sees no boundary. Today only hisab's own suites do that.
  - ⚠ **Measured cost on 3.3.0, reproduced 2026-09-30**: under a full per-module flip the suites
    reach **68 private names at 390 sites**: `tests/modules.tcyr` 375 / 63, `tests/hisab.tcyr`
    14 / 4, `tests/hisab.bcyr` 1 / 1.
  - These are almost all deliberate white-box tests: Delaunay predicates, spatial node layouts,
    and the instrumentation counters that mutation-proven guards read (`_CGA_NULL_TBL`,
    `_GA_EPA_POLISH_COUNT`, `_CGA_NULL_COEF_BAD`). Those counters would need a public read-only
    getter, e.g. `cga_null_table_built()`.
  - **Driver**: an à-la-carte consumer that includes `src/` directly. None does; all ten use the
    bundle. `D156`, `D157`
- **SIMD the flat-array kernels**: `_opt_dot`/`_opt_norm`/`_opt_axpy`, the L-BFGS sweeps,
  `_lext_dot`/`_lext_norm`.
  - Every test calls these solvers at n = 1–3, where a 2-wide dot cannot win, so **the driver is a
    consumer-sourced `n`**. No live consumer calls `optimize.cyr` or `linalg_ext.cyr`.
  - 2.3.1 measured 1.6–6.5× (median ~2.3×), and much of that came from removing accessor calls,
    which don't exist here.
  - The buffers are exactly `alloc(n * 8)`, and `f64v_*` over-reads on odd `n`, so each kernel
    needs a pair loop plus a scalar tail.
  - A SIMD sum reorders `_opt_norm_ss` and breaks bit-identity with the scalar path. `D018`

**Numerics and algorithms.**
- **Public arbitrary-length FFT.** The Bluestein engine exists (`_numx_dft`, capped at 2^20), but
  prakash pins `num_fft(…, 3)` as `Err`, so exposing it changes a live consumer's assertion.
  **Driver**: a caller that needs a non-power-of-two DFT. `D027`
- **L-BFGS convergence.** It takes 674 iterations on Rosenbrock where BFGS takes 36. Both use the
  same Armijo search, so the gap comes from L-BFGS's γ scaling combined with backtracking that
  never extends α past 1. Measure that before choosing a Wolfe search. **Driver**: any caller of
  `opt_*`. `D030`
- **Complex arithmetic without a heap `HComplex` per temporary.** One 64×64 `cqr_decompose` uses
  32.9 MB of arena. **Driver**: a caller of `cmat_*`/`cqr_*`. `D011`
- **The interval entire/empty tag**, after **[3.3.3]**'s clamp. `D021`
- **`sym_integrate` by parts and linear substitution.** abaco, the planned user, declares no
  `[deps.hisab]`. `D076`
- **A position-dependent connection in `geodesic_rk4` / `parallel_transport`.** `D092`
- **kdtree ninther pivot**, for a consumer feeding attacker-controlled coordinates. `D060`

**Geometry.**
- **Jet tie flag**: name which faces tied, for subgradient callers. `D036`
- **The literal 1024-entry CGA table.** Measured at 2.24.0 on 6.6.2 (re-measure on the current
  pin first):
  - **Gain**: the first call drops from 258 µs to 11.9 µs, and the ~2% steady-state layout cost
    goes away.
  - **Cost**: +748 lines, and +2.93% consumer `code_size`.
  - **Driver**: a live `cga_*` caller. `D049`

**Autodiff.**
- **`ad_grad` reachability sweep.** A shared-tape Jacobian runs 5× to 47× slower than resetting
  the tape per residual, at m = 256 to 2048. `D078`
- **`ad_minimize` convenience.** The source says "Revisit if a consumer asks". `D079`
- **Fixed-size `dual_*` vector layer.** Declined in 2.19.0: forward mode wins only below n = 8,
  and the tape is 1.43× ahead at n = 16. `D080`

**Platform and infrastructure.**
- **aarch64 bit-identity run.** Cross-build plus qemu works on 6.6.12. attn11 gates hisab to
  x86_64 today. See *Decisions owed* #3. `D120`
- **Thread safety of the process-global tables**: Perlin, the einsum arena, the CGA table, simplex
  gradients, Delaunay scratch and the EPA counter. **Driver**: a threaded consumer. `K005`
- **Benchmark row isolation.** All rows share one never-freeing arena, and adding a row once moved
  another by −32% to −44%. Isolation needs an allocator mark/restore that the stdlib lacks. `D113`

**Rust-era features never ported.** The Rust 1.4.0 line had all of these. The trigger for each is
a consumer asking.
- Monte Carlo integration
- Atkin and segmented sieves (`num_sieve` caps at 10M)
- spline arc length, de Casteljau split
- sparse Cholesky/LU, SOR (a new fn: `solve_pgs` takes no ω)
- Voronoi, Mat4 SRT decompose (scale recovery; `se3_from_mat4` already covers a rigid R + t),
  plane–plane intersection `D158`
- dual quaternions, convex decomposition, differentiable rendering
- GPU via mabda (an opt-in include, not the Rust-era soorat) `D159`
- frustum (planned by kiran); spinors and complex Hermitian eigen/SVD (planned by kana) `D161`
- the parallel module (`thread.cyr` exists at the 6.6.12 tag), and serialization. The Rust AI
  module would conflict with SECURITY's "no network I/O in core". `D160`

**Docs.**
- **`docs/guides/usage.md`**, if onboarding needs more than the README. `D137`

---

## Parked / deferred (revisit when a driver appears)

These were evaluated and consciously deferred. Each carries the measurement that parked it,
because once the reason is gone, "deferred with a reason" and "forgotten" look the same.

- **SIMD `cross`** (from 2.3.1) needs lane shuffles.
  - `f64v_shuffle`, `permute`, `blend`, `swap` and `swizzle` are all still undefined on 6.6.14
    (probed 2026-10-03; the intrinsic-name lists are identical at the 6.6.12 and 6.6.14 tags).
  - The best shuffle-free formulation measured **38 ns vs 25 ns scalar (+52%)** on 6.6.2. That
    pair has not been re-run since. `D152`
- **`#pure` annotations** (from 2.3.4). The original premise, "unsafe CSE interaction", is
  refuted: there is no CSE to be unsafe.
  - Re-probed on 6.6.14 (2026-10-03): binaries are byte-identical with and without `#pure`, and
    two allocating calls return distinct pointers.
  - ⛔ **The objection to keep**: `alloc()` carries no `#alloc`. Annotating hisab's **286**
    `alloc()` call sites `#pure` would assert a falsehood with no compiler backstop. `D153`
- **Slices (`[T]` / `slice<T>`)** (from 2.3.4).
  - Re-measured on 6.6.12: checked slice indexing costs **3.6–4.2×** a raw `load64`, and the
    unchecked accessor **3.1–3.5×**.
  - The tag's `simd.cyr` has **zero** slice-taking forms, so slices cannot cover the SIMD hot
    paths.
  - The unchecked typed-array subscript (new in 6.6.12) refuses f64 elements by design: 6.6.13's
    M2 keeps it integer-only, and 6.6.14 still refuses `var a: f64[4]; a[2]`. It is no
    alternative. The cost figures above were measured on 6.6.12. `D154`

---

## Consumers

**Ten repos consume `dist/hisab.cyr` today, SHA-locked**, read from each `cyrius.cyml` on
2026-10-31 (unchanged since 2026-09-30):

| hisab tag | consumers |
|---|---|
| **3.2.1** | prakash |
| **2.22.1** | svara, naad, goonj |
| **2.11.2** | dhvani, attn11, ghurni, prani, garjan, nidhi |

| cyrius pin | consumers |
|---|---|
| **6.6.2** | goonj, attn11 |
| **6.6.3** | svara, naad, ghurni, prani, garjan, nidhi |
| **6.6.6** | prakash |
| **6.6.10** | dhvani |

prakash moved to cyrius 6.6.6 on 2026-09-23. It was the first consumer past the `Result` break, on
2026-09-15, calling `num_fft` through the `var rt, rv = …` idiom. `svara/src/spectral.cyr:246`
calls `num_fft`.

**Four more repos carry a copy transitively.** `cyrius deps` vendors it through another dependency,
and each compiles it under its own pin (read 2026-10-31 from each `lib/hisab.cyr` header, unchanged since 2026-09-30):

| repo | hisab | via | cyrius pin |
|---|---|---|---|
| jalwa | 2.11.2 | dhvani | 6.6.3 |
| ranga | 2.11.2 | prakash | 6.6.9 |
| shabda | 2.22.1 | svara | 6.6.3 |
| shabdakosh | 2.22.1 | svara | 6.6.3 |

`ls ~/Repos/*/lib/hisab.cyr` lists all fourteen; the `[deps.hisab]` grep finds only the ten direct
ones.

⛔ **goonj and attn11 cannot take ANY hisab ≥ 3.1.0 until they move their cyrius pin.** cycc 6.6.2
refuses `public struct` + `#derive` (`#derive(...) applies to a struct or an enum`). Measured from a
dir pinned to 6.6.2: the 3.1.0, 3.2.1, 3.2.2 and 3.3.0 bundles are refused, while the 3.0.1 bundle
compiles. Under 6.6.3, 6.6.6, 6.6.10 and 6.6.12 the 3.3.0 bundle compiles and a consumer-shaped
program runs correctly. `D146`

⚠ **The pin matters to every consumer for four more reasons**, because a consumer compiles the
bundle under its OWN cycc:
- `m3_mul_vec3` keeps its hoisted z tail, because the natural form is silently wrong below 6.6.5.
  (`_cga_build_null_tbl`'s `if`-guard form went in 3.3.4: it guarded against a defect fixed in
  6.6.3, below every pin that can compile a 3.x bundle.)
- A zero produced by negation is −0 from **6.6.8** and +0 below.
- Since 3.3.4 a NaN spatial-hash coordinate goes into cell 0 on every pin (it followed `f64_to`
  before: cell 0 from 6.6.8, i64::MIN below), and a query with a NaN centre or radius returns
  nothing (3.3.3 returned every entry for a NaN radius). `D059`
- `dual_pow` is within 1 ulp from **6.6.10**. Below that it takes the older ganita's `pow`: 137
  ulp at 0.9^1024 on every pin from 6.6.3 to 6.6.9.
- From **6.6.13** (ganita 1.2.11): `cx_arg`, `cx_ln` and `cx_powf` take the
  C99 side of the branch cut for a −0 imaginary part, `atan2` of a NaN is NaN, and both-infinite
  arguments give ±π/4 or ±3π/4 (were NaN). `sinh`/`cosh` moved too, so `cx_sin`, `cx_cos` and the
  Lorentz builders change bits (hundreds of ulp near overflow and below 1; cosh is even bit for
  bit). `f64_tan` exists only from 6.6.13 (ganita's); hisab retired its own in 3.3.4.
- A consumer moving its pin to **6.6.13** or later must re-vendor `lib/math.cyr`: `f64_le`,
  `f64_ge` and `f64_trunc` became compiler builtins and reserved names, and the old file is
  refused ("reserved keyword"), loudly.
- On **6.6.0–6.6.5** a block-bodied closure in a top-level `var` is miscompiled (fixed in 6.6.6),
  so `autodiff.cyr`'s closure recipe must be built inside a fn there.

All of these were measured on the 3.2.2 bundle from pinned dirs and re-measured unchanged on
3.3.0's. The NaN cell was re-probed on the 3.3.1 bundle on 2026-09-30: an origin query returns the
NaN-inserted point under 6.6.12 and does not under 6.6.6.

⛔ **The remaining nine are still on 2.x, and the `Result` break fails silently.** A `Result` in
*argument* position degrades to its tag, and the `Ok` tag is 0, which equals `HSB_ERR_NONE`. So
`assert_eq(f(...), HSB_ERR_NONE)` **builds clean and tests nothing**. `D147`
- Hand a migrating consumer `scripts/check-result-migration.sh`, which exists for exactly that
  class and is mutation-proven.
- The reached surface is small (**34 distinct public fns in 8 modules**, per the Boundary table
  below), so each port is bounded.

⛔ **impetus, kiran, joshua, hisab-mimamsa and kana are NOT consumers.** They have no
`cyrius.cyml`; they are Rust repos needing a *port*, not a scheduling decision.
- aethersafha IS a Cyrius port now (`cyrius = "6.6.11"`), but declares no `[deps.hisab]`.
- abaco's README calls hisab a sibling, not a consumer (verified 2026-09-14).
- The ten live pins were re-read on 2026-09-30. `D162`

The table below is what these repos *would* use. It is kept because it is the planning surface,
not a statement that anything is wired up. Capabilities it names that hisab lacks are demand-gated
above.

| Planned consumer | Domain | Surface it will use |
|----------|--------|---------------------|
| **impetus** | physics | GJK/EPA, MPR, PGS, sequential-impulse, inertia, spatial |
| **kiran** | engine | projections, BVH, k-d tree, frustum (not in hisab) |
| **joshua** | simulation | DOPRI45, BDF, symplectic, optimize |
| **aethersafha** | compositor | projections, compositing, color |
| **abaco** | expression eval | symbolic integrate/LaTeX/patterns, interval |
| **hisab-mimamsa** | physics | tensors, Lie groups, diffgeo, CGA |
| **kana** | quantum | tensors, Lie groups, complex LA (no complex eigen/SVD), spinors (not in hisab) |

**Known caveat, carried forward: the no-hit path costs more since 2.9.x.**
- `gjk_intersect_3d` costs **+62% (box) / +57% (sphere)** on the no-hit path since 2.9.0, the
  price of no longer missing 134 genuine interior overlaps per 4,386 evaluations. Re-derived on
  6.6.2 by same-binary ABBA, and measured again on 6.6.12 at +62% / +58%.
- `mpr_intersect` inherits the same question: **+21% / +20%** on its miss path on 6.6.12 (the
  2.9.1 delegation to `gjk_intersect_3d`). `D041`, `D042`
- The no-hit path is the broadphase-common case. ⛔ But "a cheaper pre-filter" cannot be built
  inside the entry point. It receives only two support-function pointers, so it has **no cheaper
  information to filter with**, and hisab already ships the broadphase (`src/spatial.cyr`).
- **This is caller-side work, not a hisab roadmap item.** No live consumer calls `gjk_*` or `mpr_*`
  today.

---

## Boundary with Abaco

One row per surface, at the 3.3.1 surface: fn counts are `^public fn` in the module, `_` helpers
excluded, re-counted 2026-09-30 (identical at the 3.3.1 tag and in the 3.3.2 tree). The 3.1.1
counts this table carried until then were stale for five modules: symbolic 21 → 23, geo_diff
25 → 29, geo_advanced 41 → 43 (MPR moved in at 3.3.0), lie 30 → 32, lie_ext 26 → 25. `D163`

The last column is **measured, not assumed**: every call or `&` reference of a hisab public fn in
each consumer's own source (`lib/`, `dist/` and `build/` excluded; comments and strings stripped;
a name the consumer defines itself is not counted), re-derived on 2026-09-30 across abaco and the
ten live consumers. The names searched are the union of the current `public fn`s and every
non-`_` fn in the bundle the consumer actually vendors. Against the 2026-09-14 count, three cells
moved: naad's vec3 5/47 → 5/45 and num 5/9 → 4/8 (its only `num_rk4` is in a comment), and
prakash's num 2/7 → 2/9.

| Feature | abaco | hisab | live callers (symbols / call sites) |
|---------|-------|-------|-------------------------------------|
| `eval("sin(pi/4)")` — tokenize + parse a string | parses and evaluates | -- (no tokenizer) | -- |
| `expr_eval(tree, vars)` — evaluate an expression TREE | -- | symbolic.cyr (23 fn); ⚠ domain changed 2.11.2: `(-2)^3` was NaN for hisab's whole history, is -8 since | none |
| `sym_integrate(e, var)`, `sym_to_latex(e)`, patterns | -- | symbolic_ext.cyr (25 fn) | none |
| `ivl_*` interval arithmetic | -- | interval.cyr (15 fn) | none |
| `hvec3_cross(a, b)` and the vec family | -- | vec3.cyr | goonj 17/502, prakash 7/11, naad 5/45, dhvani 1/2 |
| `geo_ray_sphere(ray, sphere)`, BVH | -- | geo.cyr, geo_advanced.cyr | goonj 4/5 (`geo_aabb_new`, `geo_ray_new`, `bvh_build`, `bvh_query_ray`) |
| `calc_integral_simpson(&f, a, b, n, out)` -> `Result` | -- | calc.cyr, calc_ext.cyr | svara 2/11, naad 2/2, prani 1/2 (splines, `ease_in_out_smooth`) |
| `num_newton(&f, &df, x0, tol, max, out)` -> `Result`, `num_fft` | -- | num.cyr, num_ext.cyr | naad 4/8, prakash 2/9, svara 2/2, attn11 1/1 (`num_fft` in all four) |
| `dual_*` forward duals, `ad_*` reverse tape | -- | autodiff.cyr (15 dual + 23 tape fn) | none |
| `geo_jet_{sphere,plane,triangle,aabb,obb,capsule}` + partial readers | -- | geo_diff.cyr (29 fn) | none |
| `cga_*` conformal GA (null basis since 2.21.0) | -- | geo_advanced.cyr (27 of its 43 fn) | none |
| `so3_*`/`se3_*`/`bch_*`, `u1_*`/`su2_*`/`su3`/`lorentz_*` | -- | lie.cyr (32 fn), lie_ext.cyr (25 fn) | none |
| `f64_fmod` scalar helper (and `f64_tan` through 3.3.3; ganita's from cyrius 6.6.13) | -- | f64_util.cyr | `f64_tan`: goonj 1/3, naad 1/1; `f64_fmod`: garjan 1/2 |

**Hisab should never depend on abaco.** Abaco may optionally depend on hisab, and **today it does
not** (measured 2026-09-14, re-read 2026-09-30):
- abaco's `cyrius.cyml` has no `[deps.hisab]`, its 22 source files call 0 hisab symbols, and its
  own README lists hisab as a *sibling* library, "not a consumer". The "abaco" row in the
  planned-consumer table above is therefore a plan on hisab's side only.

**The reached surface across the ten live consumers is 34 distinct public fns in 8 modules**: vec3,
num, num_ext, calc, calc_ext, geo, geo_advanced, f64_util. The 2026-09-14 count said 35, because it
counted naad's `num_rk4`, which appears only in a comment.
- ghurni `include`s the bundle and calls nothing from it, and nidhi names hisab only in a comment.
- **0 of 10 reference `HSB_*` in `src/`.** prakash's `tests/wave_pattern.tcyr` asserts
  `HSB_ERR_INVALID_INPUT` as the Err payload of `num_fft(…, 3)`, which is the correct post-3.0.0
  idiom (re-read 2026-09-30).
- Eight of hisab's 35 modules have a live caller. The other 27, including everything the rows
  above mark "none", are reached only by hisab's own suites.

**No gate is owed here.** The only git dep is sakshi, and `deps --verify` already fails any
unreviewed dep.

⚠ `eval("sin(pi/4)")` being abaco's while hisab exposes `expr_eval` is not a contradiction:
`src/symbolic.cyr` takes a **tree**, not a string. hisab has no tokenizer at all.
