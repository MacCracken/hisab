# 2026-09-30 — Deferment audit and roadmap re-organisation (v3.3.0 tree)

> Read-only audit of every deferment in the tree, run to re-organise `docs/development/roadmap.md`
> into pinned 3.x versions. It records the evidence behind each roadmap row. The roadmap carries the
> plan, and this file carries the register the plan cites: `D001`–`D164`, plus `K001`–`K007` from
> the completeness pass.
>
> **Scope**: `src/`, `tests/`, `scripts/`, `.github/workflows/`, `examples/`, `cyrius.cyml`, every
> document under `docs/` (including `audit/` and `issues/archived/`), `README.md`, `SECURITY.md`,
> `CONTRIBUTING.md`, `CHANGELOG.md` (all releases, 3.3.0 down to 2.2.1), the git history of the
> roadmap, the open upstream filings in `~/Repos/cyrius` and `~/Repos/ganita`, and the ten direct
> consumers' manifests. **Toolchain**: cyrius 6.6.12, ganita 1.2.9.
>
> **Nothing in the repo was edited, built or committed by the audit.** Compile probes ran only in
> scratch directories whose own `cyrius.cyml` pinned the version under test.

## Method

1. **Sweep: 20 independent finders.** Four covered `src/` by module group. Six covered the test
   harnesses, with `modules.tcyr` split three ways. The rest covered: development docs and guides,
   the audit reports, the CHANGELOG in four version ranges, the roadmap's git history, the removed
   `private`-flip disposition table, the upstream filings plus consumer pins, and the current
   roadmap's premises. A finder reported a "deferment" only if the text promises later or
   conditional work, or a pinned defect waits on a fix. Permanent design decisions that promise
   nothing were excluded.
2. **Consolidate.** 302 raw items became 164 entries after merging true duplicates. Nothing that
   was not a duplicate was dropped.
3. **Completeness critic.** A separate pass searched from different angles: a whole-tree marker
   grep, every "filed/recorded on the roadmap" claim, and the sources no finder was assigned. It
   added 7 entries (`K001`–`K007`), for **171** in total.
4. **Verify: two independent skeptics per entry, in 35 batches.**
   - Skeptic A tried to refute "still open". It looked for the work already done in the tree, a
     later release closing it, or a premise now false on 6.6.12.
   - Skeptic B tried to refute "real hisab work". It looked for a permanent decision, a refuted
     idea, history only, caller-side work, another repo's work, or upstream-only tracking. It then
     classified the semver impact.
5. **Failure count:** 92 agents, 0 errors, 0 empty results, 0 missing verdicts (171/171 A and B).
6. **Orchestrator spot checks**, each re-run by hand on the tree:
   - `K001`: `grep -n 'syscall\(\s*59'` exits 2 with "Unmatched ( or \(" under basic regex. The
     `-E` form matches a planted `syscall(59, …)`.
   - `D109`: `POPULATION_FLOOR = 160` against a live 163 verified + 1 skipped.
   - `D056`: `delaunay_2d` has no `vec_len` or null check before `vec_get`.

## What the audit found

- **Tracking before this audit: 152 of 171 entries sat on no roadmap row.** The roadmap read "No
  versioned item is open". Meanwhile at least eleven comments in `src/`, `tests/` and `docs/` said
  their item was "filed/recorded on the roadmap", pointing at rows that did not exist. Examples:
  `tests/modules.tcyr:11351` (the `calc_monotone_cubic` slope guard), `src/geo.cyr:364-367`,
  `src/geo_advanced.cyr:3218-3225`, `tests/hisab.tcyr:2699-2700` and `:3976-3978`, and
  `docs/development/dependency-watch.md:261`. Several of the 2026-09-11 roadmap rotation's
  removals dropped open work along with history (`D034`, `D036`, `D045`, `D048`, `D080`, `D102`,
  `D103`, `D159`).
- **Skeptic A**: 147 open, 11 partly done, 10 premise false, 3 done in tree.
- **Skeptic B**:
  - 119 are real hisab deferments.
  - 25 are permanent decisions, 16 history only and 4 refuted ideas.
  - 3 are upstream-only tracking, 3 another repo's work and 1 caller-side.
  - Semver of the work: 131 patch, 22 minor, 3 major, 15 n/a. hisab does not reserve breaking work
    for a new major line: breaking items are pinned inside 3.x, as 3.0.0 and 3.3.0 were.
- **Highest-severity live findings** (not deferments, found on the way):
  - `K001`: three of the six CI security-scan patterns can never fire.
  - `D056`, `D073`, `D091`: inputs that end the caller's process.
  - `D062`: 3.3.0's claim that a consumer cannot change a jet's `kind`/`t`/`ss`/`face` is false
    on every consumer pin.
  - `D082`: the documented closure recipe for the tape-backed optimizer gradient SIGSEGVs on cycc
    6.6.3 and 6.6.4, the pin of six direct consumers.
  - `D001`: `svd_golub_kahan` returns `rc = OK` with exact-zero singular values for a graded
    bidiagonal from t = 2^-40. LAPACK gets the same matrix to full relative accuracy.

## Dispositions that are not work

Each was refuted by a skeptic with evidence and needs no row.

| id | why not |
|---|---|
| D014 | Does not reproduce on 3.3.0 / 6.6.12. The reproducer was found in git history (`df5fc8f`, issue file :274). |
| D024 | `td <= mm / td` is exactly equivalent for positive integers and cannot wrap. The fallback is unreachable from tests at any affordable cost. |
| D039 | `sequential_impulse` is a documented reduced model. A velocity-driven solver is physics simulation, which the roadmap Scope gives to impetus. |
| D040 | The "5 of 2,440" figure does not reproduce. What remains is the support-precision limit, always in the safe (missed-touch) direction. |
| D043 | The 2.8.2 "back on the roadmap" was true: the row existed (`2cd3b90`) and was retired when 2.8.3 discharged the critical. |
| D055 | The archived filing's own status line supersedes the "leave it" trigger. Fixed in 2.9.3. |
| D057 | Deliberate: the accepted side of the boundary means building 268,435,456 islands. It is recorded in the test. |
| D064 | The truncated site can only be `geo_advanced.cyr`, and the 2.8.3 scan found no alloc-in-loop there. |
| D107 | A recorded precision/recall trade-off in the measurement detector, with no promise. |
| D116 | Owed only if a speedup is ever claimed, and none is. |
| D117 | Moot: `m4_mul` returned to 118–123 ns. |
| D123 | Policy since 2.20.0: `[measured: scratch harness, not reproducible in-tree]` is an accepted marker. |
| D143 | A zero-cost, gate-enforced standing constraint. Relaxing it gains nothing. |
| D145 | A standing pin-dependence contract, already in the roadmap's Consumers prose. |
| D149 | The accepted price of ≤ 1 ulp pow. No live caller pays it, and the exponent in `srgb_to_linear` is non-integral anyway. |

`K006` (doc-health ledger rows) is input to the per-release doc-health refresh, not a roadmap row.

## Register, by target

The verdict columns are skeptic A's status and skeptic B's classification / semver. An entry that
splits (for example, a patch repair now and an API change later) appears under each target and is
marked *also*.

### 3.3.1 (13)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D028 | num_fft_2d / num_ifft_2d HSB_ERR_ALLOC repair has no in-suite assertion | open | real-hisab-deferment / patch | tests/abuse.tcyr:3001-3006; docs/development/issues/archived/2026-08-11-caller-sized-allocs-in-num_ext.md |
| D056 | delaunay_2d and detect_islands abort the caller's process when n > vec_len | open | real-hisab-deferment / patch | src/collision_mesh.cyr:744-746; src/collision_mesh.cyr:1385-1386; src/collision_core.cyr:406-421 |
| D062 | GeoJet wrap is partial: vector slots stay writable by reference, and the _GeoJet type name stays visible to consumers | open | real-hisab-deferment / patch | src/geo_diff.cyr:83; src/geo_diff.cyr:136-139; CHANGELOG.md:97 |
| D073 | Unguarded caller-sized allocations in calc_gradient, calc_jacobian, calc_hessian, calc_partial_derivative and opt_levenberg_marquardt | open | real-hisab-deferment / patch | src/calc_ext.cyr:86-91; src/calc_ext.cyr:113-117; src/calc_ext.cyr:148-156 |
| D082 | No test compiles the documented closure form of the tape-backed optimizer gradient *(also: decision)* | open | real-hisab-deferment / patch | tests/modules.tcyr:4354-4357; src/autodiff.cyr:537; src/autodiff.cyr:549-554 |
| D091 | diffgeo: sectional_curvature and geodesic_deviation skip the module's 'shared' dim cap and null guards, with no abuse coverage | open | real-hisab-deferment / patch | src/diffgeo.cyr:22-28; src/diffgeo.cyr:73; src/diffgeo.cyr:134 |
| D097 | lerp_srgb_vec3 returns its endpoints 1 ulp low; the test documents this instead of fixing it | open | permanent-decision / patch | tests/modules.tcyr:7518-7520; src/color.cyr:15; src/color.cyr:24 |
| D108 | check-constants.sh's duplicate-global check matches zero declarations, and CI's comment on duplicate vars is stale | open | real-hisab-deferment / patch | scripts/check-constants.sh:260-300; .github/workflows/ci.yml:279-313 |
| D109 | The constant gate's POPULATION_FLOOR (160) lags the live population (164) | open | real-hisab-deferment / patch | scripts/check-constants.sh:68; scripts/check-constants.sh:328-345; docs/audit/2026-09-09-roadmap-verification.md:132 |
| D111 | CI and release lockfile verification passes when cyrius.lock is missing | open | real-hisab-deferment / patch | .github/workflows/ci.yml:77-85; .github/workflows/release.yml:68-72 |
| D122 | check-result-migration.sh cannot see a Result fn that only forwards (ad_grad_into) | open | real-hisab-deferment / patch | scripts/check-result-migration.sh:38-57; src/autodiff.cyr:586-589; tests/modules.tcyr:4445 |
| D130 | edge_cases 'complex div by zero' group has a false comment and an assertion that cannot fail | open | real-hisab-deferment / patch | tests/edge_cases.tcyr:481-491; tests/edge_cases.tcyr:891; src/complex.cyr:87-88 |
| K001 | CI security scan: 3 of its 6 patterns are invalid basic regexes, so the execve, fork and sys_system checks can never fire | open | real-hisab-deferment / patch | .github/workflows/ci.yml:553-583; .github/workflows/ci.yml:562; .github/workflows/ci.yml:570-572 |

### 3.3.2 (48)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D001 | SVD bidiagonal QR 'honest limit' (dqds / implicit scaled Wilkinson shift) claimed 'on the roadmap'; no row exists and the premise is likely stale *(also: 3.6.0)* | premise-false | real-hisab-deferment / minor | src/linalg_precision.cyr:1000-1007; src/linalg_precision.cyr:965-970; src/linalg_precision.cyr:906-922 |
| D002 | Stale comments still call the subnormal-SVD balancing defect (closed 2.22.1) filed or tracked, and name a refuted repair | open | history-only / patch | src/linalg_precision.cyr:1176-1192; src/linalg_precision.cyr:198-245; tests/hisab.tcyr:4080-4083 |
| D008 | Three tridiagonal guard mutants behind eigen_qr survive, recorded with stale line citations | partly-done | history-only / patch | tests/hisab.tcyr:2791-2804; src/linalg_precision.cyr:1455; src/linalg_precision.cyr:1529-1536 |
| D020 | cx_div(a, 0) and cx_inv(0) return a fabricated 0+0i that is indistinguishable from a real zero | premise-false | permanent-decision / patch | src/complex.cyr:54; src/complex.cyr:71-75; src/complex.cyr:83-91 |
| D023 | num_factorize carries a dead 'stk' stack block from an abandoned first attempt | open | real-hisab-deferment / patch | src/num_ext.cyr:295-310 |
| D025 | DST/DCT FFT-vs-direct dispatch: DCT n = 27 is a pinned 3% mispick, and the cost model was calibrated on cycc 6.5.6 | premise-false | real-hisab-deferment / patch | src/num_ext.cyr:654-697; tests/modules.tcyr:10728-10736; CHANGELOG.md:5811-5818 |
| D029 | num_is_prime doc says '0 if composite' although 0, 1 and negatives are neither prime nor composite | open | real-hisab-deferment / patch | docs/audit/2026-08-04-v2.8.0-full.md:105; src/num.cyr:569 |
| D033 | Drifted line citations and stale claims in linalg_precision, linalg_ext, complex and num_ext comments | open | real-hisab-deferment / patch | src/complex.cyr:296; src/linalg_ext.cyr:1150; src/linalg_ext.cyr:909 |
| D041 | gjk_intersect_3d no-hit cost (+62% box / +57% sphere since 2.9.0); left to a consumer question, while src, threat-model and SECURITY still quote +55% *(also: consumers)* | partly-done | caller-side / patch | docs/development/roadmap.md:208-215; src/geo_advanced.cyr:389-397; src/geo_advanced.cyr:413 |
| D046 | _cga_build_null_tbl comment says the occupancy-list cut is 'on the roadmap'; it shipped in 2.24.0 | open | history-only / patch | src/geo_advanced.cyr:2554-2571; src/geo_advanced.cyr:2626-2640; CHANGELOG.md:1530-1534 |
| D053 | Test gap: nothing discriminates the rt sorted-insert order in _cga_build_null_tbl | premise-false | refuted-idea / patch | src/geo_advanced.cyr:2634-2638; CHANGELOG.md:1588-1590 |
| D059 | The spatial-hash cell for a NaN/+-Inf coordinate depends on the consumer's pin, and a NaN point is returned near the origin | open | permanent-decision / patch | CHANGELOG.md:392-397; src/spatial.cyr:968-970; src/spatial.cyr:1071-1073 |
| D061 | triangulate_polygon's reflex-only prune costs +15% at small n, with no small-n fallback | premise-false | permanent-decision / minor | tests/hisab.bcyr:734-736; tests/hisab.bcyr:1210; CHANGELOG.md:4946 |
| D063 | _COL_F64_ONE/_NEG_ONE and eight more _COL_* globals in collision_core are dead | open | real-hisab-deferment / patch | src/collision_core.cyr:24; src/collision_core.cyr:26; src/collision_core.cyr:31 |
| D065 | Stale line references, issue paths and comments across geo, geo_diff, geo_advanced, collision_core and collision_mesh | open | real-hisab-deferment / patch | src/geo.cyr:478; src/geo.cyr:1340; src/collision_core.cyr:11 |
| D066 | Test comments cite a roadmap row and an audit figure that no longer exist (cylinder-vs-rotated-OBB reference depth) | open | history-only / patch | tests/modules.tcyr:5226-5229; tests/modules.tcyr:9070; tests/modules.tcyr:9096-9097 |
| D075 | Retire _sym_render_f64, a hand copy of stdlib fmt_float_buf whose rationale is stale; its 'filed on the roadmap' is dangling | open | real-hisab-deferment / patch | src/symbolic.cyr:133-150; src/symbolic.cyr:169; src/symbolic.cyr:703 |
| D077 | _perm_init's 10-arg _perm_store_8 workaround still says it is required, for a cc5 18-arg bug fixed in cyrius 6.2.11 | open | real-hisab-deferment / patch | src/calc.cyr:391-429; src/calc.cyr:440-441; docs/development/issues/archived/2026-04-26-cc5-18-arg-fn-scrambles-params.md:7-8 |
| D084 | Stale comments in noise_simplex, calc_ext, optimize, ode and symbolic, plus a pre-3.3.0 name in the atan2 issue record | open | real-hisab-deferment / patch | src/noise_simplex.cyr:1-5; src/noise_simplex.cyr:90; src/noise_simplex.cyr:97-98 |
| D092 | geodesic_rk4 and parallel_transport hold the Christoffel symbols constant along the path, and only parallel_transport says so *(also: demand-gated)* | open | permanent-decision / patch | src/diffgeo.cyr:530-535; src/diffgeo.cyr:664-669; docs/development/roadmap.md:205 |
| D094 | Stale 'se3_exp/se3_log deliberately left alone ... Recorded on the roadmap' (done in 2.16.0) | open | history-only / patch | tests/modules.tcyr:12979-12986; tests/modules.tcyr:12733-12752; src/lie_ext.cyr:39 |
| D095 | Dead internal helpers in lie_ext.cyr (_vec6_set, _vec6_add, _vec6_scale, _unskew3) | open | real-hisab-deferment / patch | src/lie_ext.cyr:59; src/lie_ext.cyr:61-68; src/lie_ext.cyr:70-77 |
| D098 | Stale comments in quat, lie, lie_ext, derive-cga-null-table.sh, check-public-surface.sh and ci.yml | open | real-hisab-deferment / patch | src/quat.cyr:8-11; src/lie.cyr:116-121; src/lie_ext.cyr:129-134 |
| D124 | Stale 'perlin noise tests skipped' note and helper-ordering rule in tests/modules.tcyr | open | history-only / patch | tests/modules.tcyr:40-41; tests/modules.tcyr:1259; tests/modules.tcyr:3293-3294 |
| D125 | Stale source cross-references and header claims in tests/modules.tcyr lines 1-4800 | open | history-only / patch | tests/modules.tcyr:1; tests/modules.tcyr:3; tests/modules.tcyr:47-51 |
| D126 | Stale file:line, benchmark-name and doc claims in tests/modules.tcyr lines 4800-9600 | open | real-hisab-deferment / patch | tests/modules.tcyr:6036; tests/modules.tcyr:6144; tests/modules.tcyr:6701 |
| D127 | Stale source and line cross-references in tests/hisab.tcyr comments | open | real-hisab-deferment / patch | tests/hisab.tcyr:631; tests/hisab.tcyr:1231; tests/hisab.tcyr:2542 |
| D128 | abuse.tcyr KNOWN DEFECT register (10 FIXED in 2.9.0, 1 BY DESIGN) carries stale src citations, '[Unreleased]' labels and count drift | open | real-hisab-deferment / patch | tests/abuse.tcyr:30-35; tests/abuse.tcyr:3086-3101; tests/abuse.tcyr:3103 |
| D129 | Stale comment says spatial_hash_insert 'is not migrated yet' to Result | open | permanent-decision / patch | tests/abuse.tcyr:2197-2202; src/spatial.cyr:1069-1078; CHANGELOG.md:820 |
| D131 | Stale claim in foundation.tcyr about the 10-decade _sc_sweep and hisab_inverse_lerp | open | history-only / patch | tests/foundation.tcyr:1472-1475; tests/modules.tcyr:1507-1526; src/transforms.cyr:119-129 |
| D133 | Stale 'open', 'scheduled' and 'on the roadmap' claims across dependency-watch, threat-model, doc-health, overview, CONTRIBUTING, SECURITY, math.md and two archived filings | partly-done | history-only / patch | docs/development/dependency-watch.md:132; docs/development/dependency-watch.md:286; docs/development/dependency-watch.md:293 |
| D134 | Audit ledgers still show OPEN/DEFERRED dispositions for items closed later, while claiming to be the authority | open | history-only / patch | docs/audit/2026-09-09-epsilon-open.json:474; docs/audit/2026-09-09-epsilon-open.json:899; docs/audit/2026-09-09-epsilon-census.md:56-67 |
| D135 | Archived bench-net record's header still says 'Open ... scheduled on the roadmap' | open | refuted-idea / patch | docs/development/issues/archived/2026-09-09-bench-net-below-timer-floor.md:6 |
| D136 | SECURITY 'Known Limitations' and the threat model list svd_compute via A^T*A (stale since ganita 1.2.4), Jacobi O(n^5) and unchecked m4_get/m4_set, with no disposition | open | real-hisab-deferment / patch | SECURITY.md:35-43; docs/development/threat-model.md:53-55; src/linalg_ext.cyr:950-962 |
| D138 | The Rust-vs-Cyrius comparison's 'Optimization vectors for future versions' is stale and has no roadmap row | open | permanent-decision / patch | docs/benchmarks-rust-v-cyrius.md:237-263 |
| D139 | Upstream filings closed with no hisab-side record, and a gate comment cites an archive path that never existed | open | real-hisab-deferment / patch | scripts/check-public-surface.sh:62-67; docs/development/roadmap.md:75-78 |
| D141 | m3_mul_vec3 keeps its hoisted z tail for consumers below cycc 6.6.5, and also for a measured -6.5% that never expires *(also: 3.3.7)* | open | permanent-decision / patch | src/mat3.cyr:83-117; src/mat4.cyr:106-115; tests/foundation.tcyr:1408-1427 |
| D142 | _cga_build_null_tbl's if-guard workaround: its stated reason no longer protects any consumer of the 3.x bundle *(also: decision)* | open | real-hisab-deferment / patch | src/geo_advanced.cyr:2664-2694; docs/development/roadmap.md:43-46; docs/development/roadmap.md:171-178 |
| D150 | Residual cyrius CLI misparse: an unknown flag before the subcommand still shifts positionals | premise-false | upstream-only-tracking / n/a | docs/development/issues/archived/2026-04-26-cyrius-cli-arg-clobbers-source.md:8-19; CHANGELOG.md:1148-1154; CHANGELOG.md:8539 |
| D151 | RISC-V rv64 upstream watch: a stale 'as of 6.5.18' note, with vendoring work due when it lands | open | upstream-only-tracking / patch | docs/development/dependency-watch.md:326-327 |
| D158 | Rust-era capabilities never ported and absent from the roadmap, though port-audit calls parity complete *(also: demand-gated)* | open | real-hisab-deferment / minor | docs/development/port-audit.md:6-25; docs/development/port-audit.md:80; docs/development/port-audit.md:83 |
| D160 | Parallel, AI and serialization modules are unported on premises that are now outdated (thread.cyr and http.cyr exist at 6.6.12), and the global Perlin table is single-threaded *(also: demand-gated)* | open | real-hisab-deferment / minor | docs/development/port-audit.md:101-102; docs/development/dependency-watch.md:379-382; docs/development/threat-model.md:51 |
| D161 | Planned-consumer tables promise capabilities hisab lacks: frustum (kiran), spinors (kana), complex eigen/SVD *(also: demand-gated)* | open | real-hisab-deferment / minor | docs/architecture/overview.md:254-259; docs/development/roadmap.md:198-206; docs/benchmarks-rust-v-cyrius.md:102 |
| D163 | Boundary-with-Abaco table has stale fn counts (3.1.1 surface) and caller counts | open | real-hisab-deferment / patch | docs/development/roadmap.md:219-255 |
| K002 | Every module header's 'Usage: include "lib/<m>.cyr"' line is wrong, and most 'Requires:' lines contradict overview.md's include table, which was derived by compiling each module | open | real-hisab-deferment / patch | src/vec3.cyr:2; src/mat3.cyr:2-3; src/linalg_ext.cyr:3-4 |
| K003 | 23 public constants are mutable `public var` globals, so any consumer can rewrite library-wide guards and caps; SECURITY and threat-model name a dead global as the iteration limit *(also: 3.5.0)* | open | real-hisab-deferment / minor | src/geo_advanced.cyr:60; src/geo_advanced.cyr:66; src/geo_advanced.cyr:872 |
| K005 | Process-global mutable state beyond the Perlin table is undocumented: the einsum arena, CGA table, simplex gradient table, Delaunay scratch buffer and EPA counter *(also: demand-gated)* | open | real-hisab-deferment / patch | docs/development/threat-model.md:51; SECURITY.md:23; src/einsum.cyr:26-38 |
| K007 | tests/modules.tcyr: the cx_powf sweep comment still says the guard is on \|a\|², a premise src/complex.cyr says died in 2.17.0 | open | real-hisab-deferment / patch | tests/modules.tcyr:13033-13039; src/complex.cyr:193-210; tests/modules.tcyr:13146-13163 |

### 3.3.3 (10)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D010 | cqr_decompose still uses the one-sided balance scale that 2.22.1 proved lossy (unrepaired sibling) | open | real-hisab-deferment / patch | src/linalg_precision.cyr:1790; src/linalg_precision.cyr:1193; src/linalg_precision.cyr:1705 |
| D012 | error.cyr says svd_compute 'can only ever succeed', but it passes through ganita_mat_svd's -1/-2, and svd_truncated discards that return *(also: 3.4.0)* | open | real-hisab-deferment / patch | src/error.cyr:12-14; src/linalg_ext.cyr:954-962; src/linalg_ext.cyr:972-1014 |
| D013 | eigen_power returns a non-dominant eigenvalue when e_0 is an eigenvector (diag(2,3,4) gives 2) | open | real-hisab-deferment / patch | tests/abuse.tcyr:984-992; src/linalg_ext.cyr:1018; src/linalg_ext.cyr:1056-1060 |
| D015 | solve_bicgstab: absolute 1e-18 breakdown guards silently return the initial guess (2.11.0 audit finding, scheduled, never done) *(also: 3.4.0)* | open | real-hisab-deferment / patch | src/linalg_ext.cyr:796-857; src/linalg_ext.cyr:802; src/linalg_ext.cyr:808 |
| D017 | GMRES and BiCGSTAB rely on an undocumented 'A_fn returns a fresh buffer' contract (2026-04-15 M6, never closed) | open | real-hisab-deferment / patch | docs/audit/2026-04-15.md:138; src/linalg_ext.cyr:495; src/linalg_ext.cyr:607 |
| D021 | Interval type has no entire/empty tag, so ivl_add/ivl_sub on unbounded operands are unsound *(also: demand-gated)* | open | real-hisab-deferment / patch | src/interval.cyr:130-136; src/interval.cyr:148-153; src/interval.cyr:202-203 |
| D022 | num_divisor_sigma silently wraps i64 for k >= 1 on large n | open | real-hisab-deferment / patch | src/num_ext.cyr:157-159; src/num_ext.cyr:183-193; src/num_ext.cyr:207-220 |
| D068 | calc_monotone_cubic's absolute 1e-30 slope guard zeroes tangents, breaking scale invariance; the test's 'recorded on the roadmap' is dangling | open | real-hisab-deferment / patch | src/calc_ext.cyr:766-771; src/calc_ext.cyr:20-33; tests/modules.tcyr:11348-11354 |
| D071 | calc_adaptive_simpson does unbounded work for tol <= 0 (the 2026-08-03 finding was closed only for NaN) | open | real-hisab-deferment / patch | src/calc_ext.cyr:283-294; src/calc_ext.cyr:354-364; docs/audit/2026-08-03.md:135 |
| D072 | calc_integral_trapez and calc_integral_simpson accept negative step counts and return a wrong answer as Ok | open | real-hisab-deferment / patch | tests/abuse.tcyr:1252-1281; src/calc.cyr:30-33; src/calc.cyr:50 |

### 3.3.4 (12)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D034 | geo_ray_* turn a NaN in one ray axis into a plausible finite t (a module-wide contract left open) | open | real-hisab-deferment / patch | src/geo.cyr:357-367; src/geo.cyr:737; CHANGELOG.md:4268-4273 |
| D035 | geo_ray_obb: negative half-extents make one box solid or empty depending on the ray | open | real-hisab-deferment / patch | CHANGELOG.md:4548-4555; docs/development/issues/archived/2026-08-10-slab-parallel-test-is-scale-free.md:68-71; src/geo.cyr:102-108 |
| D037 | geo_triangle_unit_normal residual band: hvec3_cross over/underflows, leaving 378 of 1401 leg scales wrong | open | real-hisab-deferment / patch | src/geo.cyr:192-205; tests/modules.tcyr:13409-13433; CHANGELOG.md:2796-2798 |
| D038 | geo_barycentric_coords still fabricates (1/3,1/3,1/3) at small scale (Gram determinant flushes) | open | real-hisab-deferment / patch | src/geo.cyr:1324-1351 |
| D045 | time_of_impact / _toi_near_point is not scale-free: a false impact below 2^-537 | open | real-hisab-deferment / patch | src/geo_advanced.cyr:2133; src/geo_advanced.cyr:2138-2142; src/geo_advanced.cyr:2153 |
| D048 | cga_point forms x^2+y^2+z^2 as a naive sum of squares | open | real-hisab-deferment / patch | src/geo_advanced.cyr:3376; src/geo_advanced.cyr:3388 |
| D052 | cga_blade_inverse treats non-null spheres as null depending on position or radius, and its comment is stale | open | real-hisab-deferment / patch | src/geo_advanced.cyr:3275-3314; src/geo_advanced.cyr:22-30 |
| D058 | quadtree_insert and octree_insert accept NaN points | open | real-hisab-deferment / patch | tests/abuse.tcyr:2074-2096; src/spatial.cyr:458; src/spatial.cyr:706 |
| D085 | lie_norm3/_lie_norm4 drop a NaN, so the Lie constructors and exp/log maps return the identity for NaN input | open | real-hisab-deferment / patch | src/lie.cyr:91-98; src/lie.cyr:100-109; src/lie.cyr:262-264 |
| D086 | Lie axis guards fire on non-zero subnormal norms; their 'F64_TINY is exactly norm == 0' comment has been false since 2.17.0 | open | real-hisab-deferment / patch | src/lie.cyr:122-128; src/lie.cyr:146-149; src/lie.cyr:211-214 |
| D087 | hvec3_normalize and hquat_normalize return zero/identity for 47 of 48 non-zero subnormal inputs; the 'documented tree-wide limit' is documented nowhere | open | real-hisab-deferment / patch | tests/foundation.tcyr:745-773; src/vec3.cyr:160-161; src/quat.cyr:142-149 |
| D089 | Zero-input fallbacks in quat, lie, lorentz, diffgeo, gell_mann and geo_segment_direction return structurally valid answers with no error channel | open | permanent-decision / major | src/quat.cyr:145; src/quat.cyr:207; src/lie.cyr:116-128 |

### 3.3.5 (19)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D004 | tests/hisab.tcyr still calls _lp_pow2_floor subnormal balancing open (fixed 2.18.0), and a mutant-survivor argument rests on that false premise | open | real-hisab-deferment / patch | tests/hisab.tcyr:3972-3978; tests/hisab.tcyr:3992-3996; tests/hisab.tcyr:4404-4411 |
| D005 | SVD 4x4 block-ratio sweep asserts reconstruction only for lp_r < 4, under a dangling 'recorded on the roadmap' for a defect fixed in 2.18.0 | open | real-hisab-deferment / patch | tests/hisab.tcyr:2693-2713; CHANGELOG.md:2977-2981; CHANGELOG.md:2645-2672 |
| D006 | 2.15.0 SVD and eigen block-ratio sweep depths are sized to floors that 2.18.0 removed | open | real-hisab-deferment / patch | tests/hisab.tcyr:2562-2569; tests/hisab.tcyr:2582; tests/hisab.tcyr:2633-2646 |
| D007 | Bulge-chase Givens trigger narrowing (< F64_TINY -> == 0) survives as a self-described genuine gap | partly-done | real-hisab-deferment / patch | tests/hisab.tcyr:4389-4402; src/linalg_precision.cyr:1010; src/linalg_precision.cyr:1069 |
| D026 | Bluestein chirp-angle reduction (j*j mod 2N) has no assertion guarding it | open | real-hisab-deferment / patch | src/num_ext.cyr:450-461; src/num_ext.cyr:473-479 |
| D031 | BFGS bare-sign-test and L-BFGS gamma-guard mutants survive, because no fixture reaches what either 2.15.0 repair targets | open | real-hisab-deferment / patch | tests/modules.tcyr:12512-12526; src/optimize.cyr:513-527; src/optimize.cyr:688-706 |
| D047 | cga_norm 'filed on the roadmap' block and the 2^-26 point-nullity test bound both describe the pre-2.21.0 ep/em basis | open | real-hisab-deferment / patch | src/geo_advanced.cyr:3218-3225; src/geo_advanced.cyr:3133-3136; src/geo_advanced.cyr:3374-3389 |
| D054 | Test gap: no assertion tells raw from translated operands in _col_orient2d_sign | open | real-hisab-deferment / patch | src/collision_mesh.cyr:179-193; docs/development/issues/archived/2026-08-04-incircle-precision.md:257-264; CHANGELOG.md:4881-4885 |
| D067 | Stale 'separate open finding' on gjk_epa_3d box/box depth, which 2.8.3 closed | open | real-hisab-deferment / patch | tests/modules.tcyr:9624-9630; tests/modules.tcyr:5066-5115; CHANGELOG.md:5851-5867 |
| D081 | ad_grad_write is the one public fn no test calls (coverage 653/654), and its n < 0 guard is never reached | open | real-hisab-deferment / patch | src/autodiff.cyr:571-577; tests/modules.tcyr:4435; CHANGELOG.md:30-31 |
| D083 | dual_pow's derivative is still asserted through LOOSE_TOL_M although it is bit-exact | partly-done | real-hisab-deferment / patch | tests/modules.tcyr:8279-8287; src/autodiff.cyr:181 |
| D093 | so3_exp scale sweep stops at 2^-529, citing a norm limitation that 2.17.0 removed | open | real-hisab-deferment / patch | tests/modules.tcyr:12929-12948; tests/modules.tcyr:12759-12768; src/lie_ext.cyr:217-224 |
| D099 | Residual f64_to(f64_round(...)) assertions across all five suites, missed by the 2.13.0 migration | open | real-hisab-deferment / patch | tests/modules.tcyr:1129; tests/modules.tcyr:3318; tests/modules.tcyr:4699 |
| D100 | 20 public fns are covered only by a pointer check; m3_normal_matrix, SH bands 1-8 and the symbolic rewriter are never value-asserted | open | real-hisab-deferment / patch | docs/audit/2026-08-11-v2.11.0-full.md:84-88; docs/audit/2026-08-04-2.7.0-closeout.md:63; tests/foundation.tcyr:1430 |
| D104 | Two tree-wide sweeps the 2.11.0/2.11.4 lessons imply were never run: vacuous assertions and direct guard interrogation | partly-done | real-hisab-deferment / patch | CHANGELOG.md:3585-3600; CHANGELOG.md:4159-4166; docs/audit/2026-09-09-roadmap-verification.md:128-129 |
| D105 | Recorded blind spot: an additive 1-ulp error in hvec2_add is invisible to the suites | open | real-hisab-deferment / patch | CHANGELOG.md:3166-3168 |
| D118 | The fuzz harness is a fixed self-test, and the einsum notation parser is not fuzzed | open | real-hisab-deferment / patch | tests/hisab.fcyr:4-12; tests/hisab.fcyr:295; tests/hisab.fcyr:333 |
| D121 | A compiler warning in tests/foundation.tcyr has been left since 3.1.1, and CI's check step never compiles the suites | open | permanent-decision / patch | tests/foundation.tcyr:821; CHANGELOG.md:1170-1174; CHANGELOG.md:794-796 |
| D132 | ganita atan2 exposure beyond the two tripwires is unpinned: cx_sqrt, cx_ln and arg(-0, +-0) | open | real-hisab-deferment / patch | docs/development/issues/2026-09-30-ganita-atan2-signed-zero-and-nan.md:29-30; docs/development/roadmap.md:89-94; tests/edge_cases.tcyr:512-525 |

### 3.3.6 (9)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D044 | No benchmark covers the gjk_epa_3d / mpr_penetration no-contact (separated-pair) path | open | real-hisab-deferment / patch | CHANGELOG.md:5958-5961; tests/hisab.bcyr:497-516; tests/hisab.bcyr:1359-1377 |
| D101 | No mutation harness in the repo; the 38.4% mutant-survival figure has never been re-measured | open | real-hisab-deferment / patch | docs/audit/2026-08-11-v2.11.0-full.md:84; docs/audit/2026-09-09-roadmap-verification.md:117; CHANGELOG.md:4112 |
| D106 | Measurement-provenance debt: 517 unmarked claims, --ratchet red against a 417 baseline, and the per-claim gate never runs *(also: decision)* | open | real-hisab-deferment / patch | scripts/check-measurements.sh:86-116; scripts/measurement-baseline.txt:1-29; .github/workflows/ci.yml:332-337 |
| D112 | bench-history's 'regime' column cannot detect an instrument change that keeps printing the floor line | open | real-hisab-deferment / patch | scripts/bench-history.sh:41-73; CHANGELOG.md:3947-3950 |
| D114 | Benchmark coverage: 11 of 35 modules have no row, so the LM/L-BFGS hoists and the calc_ext costs go unmeasured | open | real-hisab-deferment / patch | tests/hisab.bcyr:4; tests/hisab.bcyr:443-450; tests/hisab.bcyr:1340-1400 |
| D115 | No benchmark rows for the paths whose 3.x costs were measured only in scratch harnesses | open | real-hisab-deferment / patch | CHANGELOG.md:145-149; CHANGELOG.md:295-300; CHANGELOG.md:431-440 |
| D119 | Threat model claims a standing destination-slot (miscompile) audit of suite binaries; it was a one-time objdump check, with no script or CI gate | open | real-hisab-deferment / patch | docs/development/threat-model.md:179-181; CHANGELOG.md:891-893; CONTRIBUTING.md:93 |
| D155 | bench_batch fixed-window rows: the roadmap's counts are stale (40 sites, not 39; 80 labels, not 78), and a reason to re-evaluate may have appeared | open | real-hisab-deferment / patch | docs/development/roadmap.md:96-98; docs/development/roadmap.md:117; tests/hisab.bcyr:52-60 |
| K004 | bench-history.csv has no load column (left open by the 2026-09-09 verification, then dropped from the roadmap with no disposition), and its commit column labels each run with the previous commit | open | real-hisab-deferment / patch | scripts/bench-history.sh:16; scripts/bench-history.sh:170; bench-history.csv:1 |

### 3.3.7 (4)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D016 | solve_gmres allocates six work buffers inside its restart loop, so the bump arena grows per restart cycle | open | real-hisab-deferment / patch | src/linalg_ext.cyr:563; src/linalg_ext.cyr:567; src/linalg_ext.cyr:581 |
| D050 | _cga_scalar_of_geo is quadratic in occupancy: dense cga_norm_sq is 10.93x slower | open | real-hisab-deferment / patch | src/geo_advanced.cyr:3364-3372; src/geo_advanced.cyr:3138-3172; CHANGELOG.md:1570-1573 |
| D051 | _cga_geo_blades allocates 32 B per term pair in every CGA product | open | real-hisab-deferment / patch | src/geo_advanced.cyr:2799-2815; src/geo_advanced.cyr:2904; src/geo_advanced.cyr:3147 |
| D141 | m3_mul_vec3 keeps its hoisted z tail for consumers below cycc 6.6.5, and also for a measured -6.5% that never expires *(also: 3.3.2)* | open | permanent-decision / patch | src/mat3.cyr:83-117; src/mat4.cyr:106-115; tests/foundation.tcyr:1408-1427 |

### 3.4.0 (9)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D012 | error.cyr says svd_compute 'can only ever succeed', but it passes through ganita_mat_svd's -1/-2, and svd_truncated discards that return *(also: 3.3.3)* | open | real-hisab-deferment / patch | src/error.cyr:12-14; src/linalg_ext.cyr:954-962; src/linalg_ext.cyr:972-1014 |
| D015 | solve_bicgstab: absolute 1e-18 breakdown guards silently return the initial guess (2.11.0 audit finding, scheduled, never done) *(also: 3.3.3)* | open | real-hisab-deferment / patch | src/linalg_ext.cyr:796-857; src/linalg_ext.cyr:802; src/linalg_ext.cyr:808 |
| D032 | lyapunov_max with iterations <= 0 returns Ok with exponent 0, unlike eigen_qr and opt_* | open | permanent-decision / patch | tests/abuse.tcyr:1028-1030 |
| D069 | Plausible-value fallbacks in transforms, calc_ext, ode and autodiff, acknowledged in comments but tracked nowhere (one live consumer affected) | open | real-hisab-deferment / minor | src/transforms.cyr:121-128; src/transforms.cyr:176-181; src/calc_ext.cyr:74 |
| D070 | calc_adaptive_simpson and the implicit ODE steppers cannot report non-convergence, on the stale premise that Cyrius has 'no Result type' | open | real-hisab-deferment / minor | src/calc_ext.cyr:283-287; src/calc_ext.cyr:326-332; src/calc_ext.cyr:342-365 |
| D074 | expr_eval returns a fabricated 0.0 and writes to stderr on an undefined variable or unknown tag; the NaN deferral premise is stale and the test is skipped | open | real-hisab-deferment / minor | src/symbolic.cyr:345-349; src/symbolic.cyr:369-379; src/symbolic.cyr:417-419 |
| D088 | m3_inverse/m4_inverse return a fabricated identity for singular or NaN input (2.15.0 fixed only the threshold) | open | permanent-decision / major | src/mat3.cyr:156-167; src/mat3.cyr:228-230; src/mat4.cyr:150-159 |
| D102 | No full audit sweep since the v2.11.0 tree; its 'did NOT reach' list, including the SIMD f64v_* paths and cross-module interaction, was never discharged | partly-done | real-hisab-deferment / patch | docs/audit/2026-08-11-v2.11.0-full.md:125-131; docs/audit/2026-09-09-roadmap-verification.md:59; docs/audit/2026-09-09-roadmap-verification.md:116 |
| D103 | Reconcile, row by row, the 17 confirmed-but-unrepaired findings from the 2026-08-11 audit | open | real-hisab-deferment / patch | docs/audit/2026-09-09-roadmap-verification.md:119; docs/audit/2026-09-09-roadmap-verification.md:178 |

### 3.5.0 (5)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D019 | cx_powf zero base: the 0^0 convention and the 0^-n pole are deferred, so cx_powf disagrees with num_modpow and dual_pow | open | real-hisab-deferment / minor | src/complex.cyr:203-210; tests/abuse.tcyr:2723-2728; tests/abuse.tcyr:2782 |
| D076 | sym_integrate gaps: ln(x), products, substitution and non-bare powers *(also: demand-gated)* | open | real-hisab-deferment / minor | src/symbolic_ext.cyr:138-139; src/symbolic_ext.cyr:146-165; src/symbolic_ext.cyr:168-190 |
| D090 | 2026-04-15 audit items L4/L6 never closed: t2d_apply returns an HVec3, and gamma_spatial answers an invalid index with a zero matrix | partly-done | real-hisab-deferment / patch | docs/audit/2026-04-15.md:18-26; docs/audit/2026-04-15.md:160-168; docs/audit/2026-04-15.md:201 |
| D096 | linearize_depth_reverse_z(0) returns 0, which its own comment defines as the eye, for the infinite far plane | open | real-hisab-deferment / minor | src/color.cyr:159-171; tests/modules.tcyr:7496; tests/modules.tcyr:12561-12562 |
| K003 | 23 public constants are mutable `public var` globals, so any consumer can rewrite library-wide guards and caps; SECURITY and threat-model name a dead global as the iteration limit *(also: 3.3.2)* | open | real-hisab-deferment / minor | src/geo_advanced.cyr:60; src/geo_advanced.cyr:66; src/geo_advanced.cyr:872 |

### 3.6.0 (3)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D001 | SVD bidiagonal QR 'honest limit' (dqds / implicit scaled Wilkinson shift) claimed 'on the roadmap'; no row exists and the premise is likely stale *(also: 3.3.2)* | premise-false | real-hisab-deferment / minor | src/linalg_precision.cyr:1000-1007; src/linalg_precision.cyr:965-970; src/linalg_precision.cyr:906-922 |
| D003 | svd_golub_kahan returns 5 silent wrong answers at the normal/subnormal boundary (c ~ 2^-1023), and the suite counts them as SUCCESS | open | history-only / patch | tests/hisab.tcyr:4209-4220; tests/hisab.tcyr:4072-4083; CHANGELOG.md:1686-1694 |
| D009 | SVD and eigen deflation use EPSILON_F64 = 1e-12 rather than machine epsilon, snapping small singular values to exact 0 | open | permanent-decision / minor | src/linalg_precision.cyr:687-691; src/linalg_precision.cyr:151-161 |

### upstream (1)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D140 | ganita atan2 signed-zero/NaN defect: flip the two KNOWN DEFECT tripwires and archive the record when upstream fixes it | open | real-hisab-deferment / patch | docs/development/roadmap.md:86-94; tests/edge_cases.tcyr:511-525; src/complex.cyr:141 |

### decision (8)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D082 | No test compiles the documented closure form of the tape-backed optimizer gradient *(also: 3.3.1)* | open | real-hisab-deferment / patch | tests/modules.tcyr:4354-4357; src/autodiff.cyr:537; src/autodiff.cyr:549-554 |
| D106 | Measurement-provenance debt: 517 unmarked claims, --ratchet red against a 417 baseline, and the per-claim gate never runs *(also: 3.3.6)* | open | real-hisab-deferment / patch | scripts/check-measurements.sh:86-116; scripts/measurement-baseline.txt:1-29; .github/workflows/ci.yml:332-337 |
| D110 | The public-surface and consumer-build gates run only under hisab's own pin; the consumer-pin matrix is manual | open | permanent-decision / patch | scripts/check-public-surface.sh:58-67; scripts/check-public-surface.sh:332-337; .github/workflows/ci.yml:51-55 |
| D120 | aarch64 FMA divergence of f64v_axpy/f64v_fmadd is guarded only by a comment, and nothing runs on aarch64 *(also: demand-gated)* | open | real-hisab-deferment / patch | CHANGELOG.md:1006-1010; src/vec3.cyr:175-177; src/vec2.cyr:140 |
| D142 | _cga_build_null_tbl's if-guard workaround: its stated reason no longer protects any consumer of the 3.x bundle *(also: 3.3.2)* | open | real-hisab-deferment / patch | src/geo_advanced.cyr:2664-2694; docs/development/roadmap.md:43-46; docs/development/roadmap.md:171-178 |
| D144 | &_private_fn still reaches private bundle functions on cycc 6.6.3, the pin of six direct consumers; no decision recorded on raising hisab's floor to 6.6.4 | partly-done | upstream-only-tracking / n/a | docs/development/roadmap.md:46; README.md:17; CHANGELOG.md:18-22 |
| D148 | The 'supported' 2.24.x line has no end-of-support version and no backport of the 3.2.2/3.3.0 correctness fixes, including CWE-190 | open | real-hisab-deferment / patch | SECURITY.md:45-51; docs/development/roadmap.md:39; docs/guides/migration-3.0.md:112-113 |
| D164 | Roadmap Current section (v3.3.0 status block): numbers verified; local tag 3.2.0 missing | done-in-tree | history-only / n/a | docs/development/roadmap.md:30-46; docs/guides/testing.md:7-11; CHANGELOG.md:871 |

### demand-gated (23)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D011 | Complex-matrix routines allocate a fresh HComplex per element op: about 32.9 MB of arena per 64x64 cqr_decompose | open | real-hisab-deferment / patch | src/linalg_precision.cyr:1823-1832; src/complex.cyr:12-18; src/complex.cyr:341-372 |
| D018 | SIMD the flat-array optimizer and Krylov kernels (_opt_dot/_opt_norm/_opt_axpy, L-BFGS sweeps, _lext_dot/_lext_norm) | open | real-hisab-deferment / patch | docs/development/roadmap.md:116-127; src/optimize.cyr:74-171; src/optimize.cyr:647-746 |
| D021 | Interval type has no entire/empty tag, so ivl_add/ivl_sub on unbounded operands are unsound *(also: 3.3.3)* | open | real-hisab-deferment / patch | src/interval.cyr:130-136; src/interval.cyr:148-153; src/interval.cyr:202-203 |
| D027 | Public arbitrary-length FFT: a Bluestein engine exists internally, but num_fft stays radix-2 only | partly-done | real-hisab-deferment / minor | docs/audit/2026-04-15.md:178-180; src/num_ext.cyr:436-439; src/num.cyr:170 |
| D030 | L-BFGS uses an Armijo-only line search, measured at about 19x the BFGS iteration count on Rosenbrock | open | real-hisab-deferment / minor | tests/hisab.tcyr:1196-1202; tests/hisab.tcyr:1217-1218; src/optimize.cyr:587 |
| D036 | Jet tie flag is a boolean; naming which faces tied is deferred until a consumer asks | open | real-hisab-deferment / minor | src/geo.cyr:327-335; src/geo_diff.cyr:333-338; CHANGELOG.md:4374-4376 |
| D049 | CGA null table: the literal 1024-entry table is declined 'yet', and the memo rewrite carries about a 2% steady-state layout cost | open | real-hisab-deferment / patch | src/geo_advanced.cyr:2554-2565; src/geo_advanced.cyr:2611-2622; CHANGELOG.md:1538-1561 |
| D060 | kdtree_build median selection is median-of-three quickselect: O(n) expected, not worst case, on adversarial coordinates | open | real-hisab-deferment / patch | src/spatial.cyr:105-132; docs/development/threat-model.md:68; docs/development/issues/archived/2026-08-04-perf-kd_partition.md:345 |
| D076 | sym_integrate gaps: ln(x), products, substitution and non-bare powers *(also: 3.5.0)* | open | real-hisab-deferment / minor | src/symbolic_ext.cyr:138-139; src/symbolic_ext.cyr:146-165; src/symbolic_ext.cyr:168-190 |
| D078 | ad_grad scans the whole tape, so a shared-tape Jacobian is O(m^2); the reachability sweep was left undone | open | real-hisab-deferment / patch | src/autodiff.cyr:461-491; tests/hisab.bcyr:314-321; CHANGELOG.md:2196-2200 |
| D079 | No ad_minimize convenience: 'Revisit if a consumer asks', with no roadmap row | open | real-hisab-deferment / minor | src/autodiff.cyr:558-562; CHANGELOG.md:4190 |
| D080 | Fixed-size dual_* vector layer: declined in 2.19.0 ('leave parked'), then dropped from the roadmap with no parked row | open | real-hisab-deferment / minor | CHANGELOG.md:2517-2521; docs/audit/2026-09-09-roadmap-verification.md:108; docs/audit/2026-09-09-roadmap-verification.md:377-386 |
| D092 | geodesic_rk4 and parallel_transport hold the Christoffel symbols constant along the path, and only parallel_transport says so *(also: 3.3.2)* | open | permanent-decision / patch | src/diffgeo.cyr:530-535; src/diffgeo.cyr:664-669; docs/development/roadmap.md:205 |
| D113 | Bench rows share one never-freeing arena, so row order is load-bearing | open | real-hisab-deferment / patch | tests/hisab.bcyr:1398-1441; docs/guides/testing.md:57 |
| D120 | aarch64 FMA divergence of f64v_axpy/f64v_fmadd is guarded only by a comment, and nothing runs on aarch64 *(also: decision)* | open | real-hisab-deferment / patch | CHANGELOG.md:1006-1010; src/vec3.cyr:175-177; src/vec2.cyr:140 |
| D137 | usage.md guide is 'not yet earned'; promote it if onboarding needs more | open | real-hisab-deferment / patch | docs/doc-health.md:831 |
| D156 | Flip the src/ modules private too, not only the bundle (68 private names reached at 390 sites) | open | real-hisab-deferment / minor | docs/development/roadmap.md:104-114; src/visibility.cyr:9-13; scripts/check-public-surface.sh:52-53 |
| D157 | No public read-only getter for the instrumentation counters (the cga_null_table_built() idea); needed only if src/ is flipped | open | real-hisab-deferment / minor | tests/hisab.bcyr:1431; tests/modules.tcyr:10239; tests/modules.tcyr:8575 |
| D158 | Rust-era capabilities never ported and absent from the roadmap, though port-audit calls parity complete *(also: 3.3.2)* | open | real-hisab-deferment / minor | docs/development/port-audit.md:6-25; docs/development/port-audit.md:80; docs/development/port-audit.md:83 |
| D159 | Port-audit's never-scheduled features (dual quaternions, convex decomposition, differentiable rendering, GPU via mabda) have no parked or demand-gated row | open | real-hisab-deferment / minor | docs/development/port-audit.md:18-25; docs/development/port-audit.md:91; docs/development/port-audit.md:99 |
| D160 | Parallel, AI and serialization modules are unported on premises that are now outdated (thread.cyr and http.cyr exist at 6.6.12), and the global Perlin table is single-threaded *(also: 3.3.2)* | open | real-hisab-deferment / minor | docs/development/port-audit.md:101-102; docs/development/dependency-watch.md:379-382; docs/development/threat-model.md:51 |
| D161 | Planned-consumer tables promise capabilities hisab lacks: frustum (kiran), spinors (kana), complex eigen/SVD *(also: 3.3.2)* | open | real-hisab-deferment / minor | docs/architecture/overview.md:254-259; docs/development/roadmap.md:198-206; docs/benchmarks-rust-v-cyrius.md:102 |
| K005 | Process-global mutable state beyond the Perlin table is undocumented: the einsum arena, CGA table, simplex gradient table, Delaunay scratch buffer and EPA counter *(also: 3.3.2)* | open | real-hisab-deferment / patch | docs/development/threat-model.md:51; SECURITY.md:23; src/einsum.cyr:26-38 |

### parked (3)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D152 | Parked SIMD cross product: the premise still holds on 6.6.12, but the row cites 6.6.2 and its lerp aside is history | open | real-hisab-deferment / patch | docs/development/roadmap.md:137-140; src/vec3.cyr:92-99; docs/audit/2026-09-09-roadmap-verification.md:53 |
| D153 | Parked #pure annotations: the premise holds on 6.6.12; only the '310 alloc() sites' count is stale (286 after stripping comments) | open | refuted-idea / patch | docs/development/roadmap.md:141-147; docs/audit/2026-09-09-roadmap-verification.md:52; docs/audit/2026-09-09-roadmap-verification.md:819-820 |
| D154 | Parked slices ([T] / slice<T>): re-measured on 6.6.12, and the park still holds | open | real-hisab-deferment / major | docs/development/roadmap.md:148-152; docs/audit/2026-09-09-roadmap-verification.md:108; docs/audit/2026-09-09-roadmap-verification.md:466-486 |

### consumers (5)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D041 | gjk_intersect_3d no-hit cost (+62% box / +57% sphere since 2.9.0); left to a consumer question, while src, threat-model and SECURITY still quote +55% *(also: 3.3.2)* | partly-done | caller-side / patch | docs/development/roadmap.md:208-215; src/geo_advanced.cyr:389-397; src/geo_advanced.cyr:413 |
| D042 | mpr_intersect no-hit path costs +17% (box) / +19% (sphere) since 2.9.1, and the roadmap caveat does not name it | open | permanent-decision / n/a | tests/hisab.bcyr:546-562; src/geo_advanced.cyr:1404; src/geo_advanced.cyr:1415 |
| D146 | goonj and attn11 pin cyrius 6.6.2 and cannot take any hisab >= 3.1.0 until they move their pin | open | other-repo / n/a | docs/development/roadmap.md:43-46; docs/development/roadmap.md:171-175; README.md:7 |
| D147 | Nine of ten direct consumers are still on hisab 2.x, behind the silent Result break (3.0) and the private flip (3.3) | open | other-repo / n/a | docs/development/roadmap.md:158-169; docs/development/roadmap.md:184-189; docs/development/roadmap.md:246-252 |
| D162 | Planned-consumer table: Rust repos that need a port, and aethersafha, a Cyrius port that declares no hisab dependency 'yet' | open | other-repo / n/a | docs/development/roadmap.md:191-206; README.md:9; docs/architecture/overview.md:248 |

### release-refresh (1)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| K006 | doc-health ledger rows D133 does not list: a summary still 'synced to v2.9.2', no rows for the three 2026-09-09 audit files, a CLOSED filing still marked 'Live'; plus one stale threat-model row | open | real-hisab-deferment / patch | docs/doc-health.md:760-767; docs/doc-health.md:839-848; docs/doc-health.md:879 |

### not-work (15)

| id | item | still open? (A) | real hisab work? (B) | sources |
|---|---|---|---|---|
| D014 | eigen_qr unit-scale NO_CONVERGENCE on a cond-6.85 symmetric 4x4 (2.7.0) has no explicit closure | done-in-tree | history-only / n/a | CHANGELOG.md:7052-7054; CHANGELOG.md:7027-7051; docs/audit/2026-08-04-v2.8.0-full.md:154 |
| D024 | num_factorize's trial-division fallback was repaired on its shape only, and no test reaches it | open | permanent-decision / patch | src/num_ext.cyr:326-345; src/num_ext.cyr:232-261; CHANGELOG.md:242-243 |
| D039 | sequential_impulse uses penetration as a velocity proxy; a full solver that takes caller-supplied relative velocities is not built | premise-false | permanent-decision / minor | src/collision_core.cyr:180-183; src/collision_core.cyr:59-64 |
| D040 | GJK exit-certificate gate can miss a genuine touch at large scale (5 of 2,440 at 2^20) | premise-false | permanent-decision / patch | src/geo_advanced.cyr:327-337 |
| D043 | EPA visible-set search is still a full scan per iteration; the O(k^2) repair 2.8.2 sent 'back on the roadmap' is not there | premise-false | permanent-decision / patch | src/geo_advanced.cyr:471-476; src/geo_advanced.cyr:897-898; CHANGELOG.md:6042-6051 |
| D055 | Delaunay in-circle determinant stays non-adaptive; its reopen trigger is recorded only in an archived filing | open | permanent-decision / patch | src/collision_mesh.cyr:239-247; src/collision_mesh.cyr:829-840; docs/development/issues/archived/2026-08-04-incircle-precision.md:208-224 |
| D057 | detect_islands: the accepted side of the ALLOC_MAX/8 boundary is not asserted | open | permanent-decision / n/a | tests/abuse.tcyr:1921 |
| D064 | The fourth site of the 2.8.1 audit's allocation-inside-loop row is truncated to 'src/geo_' and was never resolved | done-in-tree | history-only / n/a | docs/audit/2026-08-04-v2.8.0-full.md:194-198; docs/audit/2026-08-04-v2.8.0-full.md:240-243; CHANGELOG.md:5719-5725 |
| D107 | check-measurements detector: a known recall miss left open, and a conditional plan to drop the 'residual' shape | open | refuted-idea / n/a | scripts/check-measurements.sh:491-495; scripts/check-measurements.sh:124-128 |
| D116 | Before/after numbers are still owed for the 2.8.3 algorithmic speedups (spatial hash growth, BVH centroid cache, O(n log n) DCT/DST) | open | history-only / n/a | docs/audit/2026-08-04-v2.8.0-full.md:243-248; CHANGELOG.md:5759; CHANGELOG.md:5953-5966 |
| D117 | Benchmark movers never attributed: m4_mul +6.7%/+13.6% (3.2.0) and vec4_dot_x64 about +10% (3.2.2) | partly-done | history-only / n/a | CHANGELOG.md:1068-1075; CHANGELOG.md:597-601; docs/development/dependency-watch.md:57-60 |
| D123 | Headline figures from 2.20.0-2.24.0 rest on oracle and probe harnesses that were never committed | open | permanent-decision / patch | CHANGELOG.md:2240-2243; src/symbolic.cyr:65; src/linalg_precision.cyr:220 |
| D143 | Bundle-shape constraints kept only for consumers on cycc 6.6.2-6.6.4: ExprTag placement (gate claim 5) and visibility.cyr first in [lib] | open | permanent-decision / n/a | src/symbolic.cyr:199-206; src/visibility.cyr:5-7; scripts/check-public-surface.sh:46-53 |
| D145 | Pin-dependent results for consumers: dual_pow is <= 1 ulp only from 6.6.10, and negation yields -0 only from 6.6.8 | premise-false | permanent-decision / n/a | src/autodiff.cyr:164-167; docs/development/roadmap.md:175-182; docs/development/roadmap.md:236 |
| D149 | Accepted ganita pow slowdown: srgb_to_linear +96%, integral pow 28 -> 180 ns, dual_pow x^3 ~11 -> ~180 ns | open | permanent-decision / patch | docs/development/dependency-watch.md:57-60; CHANGELOG.md:431-438; CHANGELOG.md:597-601 |
