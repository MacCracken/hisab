# Changelog

## [Unreleased]

### Documentation
- Repo-wide documentation sweep to v2.4.6 state: pruned the roadmap to
  forward-facing / deferred work only (the completed 2.3.x and 2.4.x arcs now
  live in Release History); refreshed README (added the **Collision** module row
  + spatial/einsum/lie_ext/linalg_precision/noise_simplex; version, counts,
  binary size); completed the `architecture/overview.md` module map (all 34
  modules + a Collision section) and updated the collision data-flow (MPR +
  sequential-impulse); synced test/benchmark counts (901 assertions / 26
  benchmarks) in `testing.md`; added collision + supply-chain entries to
  `SECURITY.md`; added a 2026-05-29 status addendum to the dated `port-audit.md`;
  refreshed `dependency-watch.md` and the CLAUDE.md status line.
- Added `docs/doc-health.md` — a living doc-currency ledger (fresh/stale/dated/
  tracked-issue per file), adapted from `cyrius/docs/doc-health.md`.
- Re-ran `scripts/bench-history.sh` (benchmarks.md was stale at an old commit) —
  refreshed to v2.4.6, 26 benchmarks.
- Removed `docs/development/tool-issues.md` (ad-hoc catalog; real bugs are filed
  individually under `docs/development/issues/`). Archived the shipped
  `cyrius-linalg-proposal.md` to `docs/development/archive/`. Fixed CONTRIBUTING
  currency (Cyrius pin 6.0.14, `src/` not `lib/`, +fmt/distlib gates).
- Re-verified the five `docs/development/issues/` toolchain filings against the
  pinned 6.0.14 — all still reproduce (none stale-fixed).

## [2.4.6] - 2026-05-29 — Security & hardening audit; 2.4.x closeout

P(-1) hardening step closing the 2.4.x arc: a memory-safety / allocation-overflow
review of the whole library, supply-chain integrity, dependency CVE posture
(known + web research), and a security review of the six collision algorithms
brought into the build chain across 2.4.0–2.4.5. **No new vulnerability found.**
Suite 895 → **901**. Full report: `docs/audit/2026-05-29.md`.

### Added
- **`tests/edge_cases.tcyr`** — 6 allocation-overflow-guard regression assertions
  (CWE-190): `cmat_new` rejects huge / non-positive dims and allocates valid ones;
  `tensor_new` rejects rank > 8 and overflow-prone shapes and allocates valid ones.
  Pins the documented guards so removing one fails the build.

### Security
- **Audit verdict — posture solid.** No shell/exec/FFI/libc surface (no CWE-78).
  hisab's own dimension allocators all guard the multiply before allocating
  (`tensor_new`, `cmat_new`, christoffel/riemann, num_sieve). All iterative paths
  are capped. Collision index access goes through `vec_get`, which traps on OOB
  rather than corrupting memory.
- **Supply chain**: `cyrius deps --verify` → 60 verified / 0 failed;
  `cyrius vet` → 0 untrusted. No third-party deps (cyrius stdlib + first-party
  sakshi only) → zero third-party-CVE surface. No CVEs exist for the cyrius/cycc
  toolchain.
- **Open (upstream)**: stdlib `mat_new` integer overflow (`16 + rows*cols*8`,
  unguarded) is still present in the pinned 6.0.14 snapshot. hisab's usage is
  mitigated (dims come from already-allocated matrices; raw-dim `cmat_new` is
  guarded). Fix belongs in cyrius stdlib — tracked as roadmap 2.5.0.

### Changed
- **`docs/development/threat-model.md`** — added the six 2.4.x collision algorithms
  to the attack-surface table; refreshed the `mat_new` note for 6.0.14 + roadmap
  2.5.0; added a Supply Chain section and a 2026-05-29 audit-history entry.

## [2.4.5] - 2026-05-29 — Collision arc: contact solver fixed; arc complete (2.4.x)

Final patch of the collision-correctness arc, and the third with a real bug.
`sequential_impulse` produced **all-zero impulses** — it did nothing.
`solve_pgs` (the general PGS/LCP solver in `linalg_ext`) was already correct but
barely tested. Suite 888 → **895**. This closes the 2.4.x arc.

### Fixed
- **`sequential_impulse` normal impulse** (`collision_core.cyr`): the impulse was
  computed as `(1+e)·v_n / inv_mass` with `v_n = −pen` — i.e. **negative** for a
  penetrating contact (the leading `−` the comment specified was missing), so
  `max(0, accumulate)` clamped it to 0 on every iteration. Separately, the loop
  re-added the full bias each iteration without folding the accumulated impulse
  back into the velocity, so it could never converge. Rewritten as proper
  sequential impulse: `v_eff = v_n + λ·inv_mass`, `Δλ = −(1+e)·v_eff / inv_mass`,
  accumulate-and-clamp. The total impulse now converges to `(1+e)·pen / inv_mass`
  (e.g. 0.25 for pen=0.5, inv_mass=2) and is iteration-independent.

### Added
- **`tests/modules.tcyr`** — 4 `sequential_impulse` assertions: resting contact
  (λ=0.25, the case that was 0), single-iteration convergence (idempotence),
  dynamic-vs-static (λ=1.0), and both-static (skipped → 0).
- **`tests/hisab.tcyr`** — strengthened the `solve_pgs` test from a lone `x1 > 0`
  sanity check to exact-solution assertions (`x ≈ [1/11, 7/11]` for
  `[[4,1],[1,3]]x=[1,2]`) plus a **bounds-active** LCP (`diag(1,1)·x=[5,5]` clamped
  to `hi=[2,2]` → `[2,2]`). solve_pgs was audited correct — no change.

### Notes — 2.4.x arc complete
- All six collision algorithms audited and covered. Genuine bugs found + fixed in
  three: the hull insertion sort + undefined `f64_le`/`f64_ge` (2.4.0), the MPR
  origin-containment early-out (2.4.4), and this impulse-sign/convergence fix
  (2.4.5). `triangulate_polygon`, `delaunay_2d`, the half-edge mesh, and
  `solve_pgs` were already correct — they just lacked coverage, which they now have.

## [2.4.4] - 2026-05-28 — Collision arc: MPR narrowphase fixed (2.4.x arc)

Fifth patch of the collision-correctness arc, and the second with a real bug
(after 2.4.0's hull). `mpr_intersect` / `mpr_penetration` (Minkowski Portal
Refinement, 3D) returned a **false positive on every separated pair** — they
never detected non-overlap. Fixed; suite 878 → **888**.

### Fixed
- **MPR origin-containment early-out** (`collision_core.cyr`): after building the
  first portal support point `v1` (the farthest Minkowski-difference point toward
  the origin), the code lacked the standard check that the origin is actually
  reachable. For separated shapes `v1` falls short of the origin, and — when the
  centers are colinear with it — execution dropped into the degenerate
  `(v1−v0)×(−v0) == 0` branch whose comment says "check if between them" but which
  **unconditionally returned 1 (hit)**. Off-axis separated pairs slipped through
  the refinement loop's convergence test the same way. Added
  `if dot(v1, dir) < 0 → miss` to both `mpr_intersect` (returns 0) and
  `mpr_penetration` (returns 0); once it passes, the origin is genuinely within
  the support toward it and the existing portal refinement is correct.

### Added
- **`tests/modules.tcyr`** — 10 MPR assertions + sphere/box support-function
  helpers: sphere-sphere overlap (hit) vs separated on- and off-axis (miss — the
  regression); penetration depth (`sum_radii − dist` = 1.0, plus the deeper 3.0
  case) with a ±x normal; separated → miss; and box-box overlap / separated.

### Notes
- Of the five arc patches so far, only 2.4.0 (hull sort) and 2.4.4 (MPR) carried
  genuine bugs; triangulate / delaunay / half-edge were already correct.

## [2.4.3] - 2026-05-28 — Collision arc: half-edge mesh audited (2.4.x arc)

Fourth patch of the collision-correctness arc. The roadmap flagged the half-edge
mesh (`halfedge_from_triangles` + `halfedge_adjacent_faces` + `halfedge_is_boundary`)
for a twin-pointer-wiring check — but a thorough audit found it **already correct**
on both closed and open meshes. Deliverable: coverage, no source change; suite
867 → **878**.

### Added
- **`tests/modules.tcyr`** — 11 half-edge assertions + `_he_tri` / `_he_boundary_sum`
  helpers, over four reference meshes: a **tetrahedron** (closed manifold → no
  boundary vertices, 3 neighbours/face), a **single triangle** (open → all-boundary,
  no neighbours), a **2-triangle quad** sharing a diagonal (4 boundary corners, 1
  shared edge), and a **hexagon fan** (center interior, 6-vertex boundary rim — the
  case that exercises the one-ring walk closing on an interior vertex). Plus the
  empty-input → null-mesh error path.

### Notes (audit — no change needed)
- Twin assignment pairs each directed half-edge `src→dst` with the reverse
  `dst→src` (O(n²) scan); closed meshes wire every twin, open meshes leave
  boundary half-edges at `_COL_SENTINEL`. `halfedge_is_boundary` walks the
  one-ring (`next→next→twin`) with a 1000-step guard, returning 1 on any sentinel
  twin and 0 only when the ring closes back to the start. `halfedge_adjacent_faces`
  collects the twin's face for each shared edge. All correct.

## [2.4.2] - 2026-05-28 — Collision arc: `delaunay_2d` audited (2.4.x arc)

Third patch of the collision-correctness arc. The roadmap flagged
`delaunay_2d` (Bowyer-Watson) as needing a numerical-stability pass — but a
thorough audit found it **already correct and robust**, including the cocircular
grid stress case. The deliverable is coverage that verifies the empty-circumcircle
property, plus an honest "no bug" record. No source change; suite 859 → **867**.

### Added
- **`tests/modules.tcyr`** — 8 `delaunay_2d` assertions + a `_dl_violations`
  helper that counts points strictly inside any output triangle's circumcircle
  (the Delaunay property holds iff that count is 0). Fixtures: square + interior
  (4 triangles), a **3×3 grid** (heavy cocircularity — the stability stress
  case, 8 triangles), an irregular 6-point set, a single triangle (`n = 3`), and
  degenerates (`n < 3` and all-collinear → empty, no trap). Every non-degenerate
  case has **0 circumcircle violations**.

### Notes (audit — no change needed)
- The in-circle predicate (`_col_in_circumcircle`) adjusts the determinant by the
  triangle's winding sign and tests **strict** interiority, so cocircular points
  (grid corners) are correctly not flagged — that's why the grid triangulates
  cleanly. Super-triangle setup/removal, bad-triangle cavity collection, boundary
  (non-shared) edge extraction, descending swap-remove, and CCW-normalized output
  are all correct.
- Collinear / `n < 3` inputs return an empty triangulation (all triangles
  reference super-triangle vertices and are filtered) rather than trapping.

## [2.4.1] - 2026-05-28 — Collision arc: `triangulate_polygon` audited (2.4.x arc)

Second patch of the collision-correctness arc. The roadmap flagged
`triangulate_polygon` (ear clipping) as a likely-buggy untested port — but a
thorough audit found it **already correct**. The deliverable is comprehensive
coverage that promotes it from "deliberately unexercised" to verified, plus an
honest record that no bug existed. No source change; suite 846 → **859**.

### Added
- **`tests/modules.tcyr`** — 13 `triangulate_polygon` assertions + a `_tp_area2x`
  helper. Each fixture checks the triangle count (`3*(n−2)` flat indices) and
  that the output **tiles the polygon** (|Σ 2×area| == polygon |2×area|; abs ⇒
  winding-agnostic): convex quad, concave 5-gon (reflex vertex), convex hexagon,
  U-shape (2 reflex vertices), CW-wound quad, a collinear edge vertex, and the
  `n < 3` empty case.

### Notes (audit — no change needed)
- Ear detection (reflex classification via signed area + point-in-triangle),
  winding normalization (CCW/CW), and the shrink-as-clipped index bookkeeping
  are all correct. The `n*n` iteration cap + "no ear found → bail" guard make
  degenerate inputs (e.g. duplicate vertices) terminate without trapping.
- Unlike `convex_hull_2d` (2.4.0, two real bugs), `triangulate_polygon` shares
  no code with the broken sort and never referenced the undefined `f64_le`/
  `f64_ge` — it uses only the strict `f64_lt`/`f64_gt` intrinsics, which is why
  it was sound where the hull was not.

## [2.4.0] - 2026-05-28 — Collision correctness arc: `convex_hull_2d` (2.4.x arc)

First patch of the 2.4.x collision-correctness arc. `convex_hull_2d` (Andrew's
monotone chain) carried two pre-existing port bugs and trapped on any non-trivial
input — it had never run, having sat outside the build chain until 2.2.2. Both
bugs are fixed and the algorithm is now exercised + covered. Unlike the 2.3.x
arc this **changes behavior** (a trapping function now returns correct results),
so it follows a red-fixture-first cadence.

### Fixed
- **`convex_hull_2d` insertion sort** (`collision_core.cyr`,
  `_col_sort_indices_by_xy`): the hand-rolled sort's `done`-exit path
  overwrote the insertion index with `-1` on every *mid-array* insertion, so
  `vec_set(indices, -1, key)` tripped a `vec: index < 0` trap (front insertions
  happened to work). Rewritten as a standard insertion sort — shift-greater
  then drop `key` at `sj + 1` (always `>= 0`).
- **Missing `f64_le` / `f64_ge` primitives** (`f64_util.cyr`): the monotone
  chain's pop test calls `f64_le(cross, 0)`, but `f64_le`/`f64_ge` were never
  defined (only the strict `f64_lt`/`f64_gt` exist as named-op intrinsics) —
  so once the sort was fixed and execution reached the chain, the call bound to
  an undefined symbol and trapped (SIGILL). Defined both in terms of the strict
  intrinsics (`a <= b == !(a > b)`, `a >= b == !(a < b)`). This also resolves
  the same latent undefined-symbol references at 6 call sites in `spatial.cyr`
  (point-in-region tests) that current tests didn't reach.

### Added
- **`tests/modules.tcyr`** — 13 `convex_hull_2d` assertions: the keystone
  square + interior-point fixture (4-vertex CCW hull, shoelace 2×area = +8),
  plus degeneracy coverage — empty/single point, triangle (count + CCW area),
  collinear set (→ 2 extreme endpoints, interior dropped), and a duplicated
  corner (→ still 4 vertices). None trap. Suite: 833 → **846**.

## [2.3.4] - 2026-05-28 — Layout & idiom modernization (2.3.x arc)

A **layout/idiom** patch: tie heap allocations and field writes to the typed
struct definitions so a future field addition can't silently under-allocate or
land at a stale offset. Internal-only — public API, results, and machine code
are unchanged. All 833 tests pass **bit-identical**; key benchmarks verified
**flat** (`sizeof(T)` is a compile-time constant equal to the old literal, and
a derived setter lowers to the same `store64` at the same offset, so codegen is
byte-identical — no perf claim).

### Changed
- **Constructors → `sizeof(T)` + derived setters** across all 13 `#derive(accessors)`
  struct modules: `alloc(<magic-literal>)` → `alloc(sizeof(T))` and
  `store64(p + <offset>, v)` → `T_set_field(p, v)`. This eliminates the last
  hand-computed offsets (reads already used the derived getters) and couples
  every allocation to its struct layout:
  - **vec2/vec3/vec4** (`HVec2`/`HVec3`/`HVec4`), **quat** (`HQuat`),
    **complex** (`HComplex`), **autodiff** (`Dual`), **interval** (`Interval`),
    **tensor** (`HTensor` header), **geo** (all 9: `GeoRay`/`GeoPlane`/`GeoAabb`/
    `GeoSphere`/`GeoObb`/`GeoCapsule`/`GeoTriangle`/`GeoLine`/`GeoSegment`),
    **collision_core** (`ColContact`), **collision_mesh** (`HalfEdge`/`HalfEdgeMesh`).
  - SIMD scalar tails (e.g. vec3's `store64(r + 16, …)` z-write) also moved to
    `HVec3_set_z` — packed `f64v_*` writes stay as raw-memory ops.
- **Matrix grid sizes → enum constants**: `mat3.cyr` `MAT3_BYTES = 72` and
  `mat4.cyr` `MAT4_BYTES = 128` replace the magic byte-size literal that was
  repeated across `alloc` + copy-loop bounds (couples the copy length to the
  allocation length). `m3_set`/`m4_set` (column-major grids, no named fields)
  stay computed-index — a field-struct doesn't fit a flat NxN grid.
- **Float-render buffer → enum constant**: `FLOAT_RENDER_BUF = 32`
  (`var buf[FLOAT_RENDER_BUF]`) shared by the `symbolic.cyr` / `symbolic_ext.cyr`
  float renderers, replacing two bare `var buf[32]` literals.

### Added
- **`#must_use` on the core value-returning API** (9 modules: vec2/3/4, quat,
  complex, interval, autodiff, mat3, mat4) — the compiler now warns at build
  time if a pure constructor/operation result is discarded (a guaranteed bug
  for a side-effect-free function). Status-returning setters (`m3_set`/`m4_set`)
  are deliberately **not** annotated. Verified the full 34-module build and the
  833-test suite emit **zero** "result is discarded" warnings.

### Notes (evaluated — deferred or N/A)
- **`#pure`**: deferred. Its CSE/memoization semantics interact unsafely with
  hisab's allocate-a-fresh-result convention (two "equal" calls must return
  distinct mutable pointers); the win is speculative (hot paths are already SIMD)
  with no benchmark driver. Not worth the risk for a patch.
- **Slices (`[T]`/`slice<T>`)**: deferred. The hot loops are raw-pointer +
  `load64`/`store64`, already SIMD-optimized (2.3.1). Bounds-checked slices would
  regress them; `slice_unchecked_get_W` discards the safety benefit, leaving only
  churn. No consumer demand.
- **`defer`**: N/A. The library holds no per-resource lifecycles — long-lived
  data uses the bump/arena allocator (never individually freed) and the library
  opens no file descriptors. Nothing for `defer` to clean up.

## [2.3.3] - 2026-05-28 — Safety & numerical-correctness audit (2.3.x arc)

A correctness audit of the integer/bit-math surface against the vidya gotcha
catalogue (signed shifts, static buffers, element-vs-byte sizing, overflow).
**Verdict: no bugs found — hisab was already careful.** The deliverable is a
fixed misleading comment plus regression tests that pin the load-bearing
invariants, so a future toolchain change can't silently break them. 833/833
tests pass (was 825 + 8 new invariant assertions).

### Fixed
- **num.cyr PCG comment**: the note claimed "Cyrius `>>` is arithmetic
  (sign-extending)" — it is **logical** (zero-fill), verified on 6.0.14
  (`0x8000…0 >> 1 == 0x4000…0`). The PCG RNG is correct either way (it wants
  unsigned shifts and masks to 32 bits), but the comment actively misinformed.
  Corrected.

### Added
- **`tests/edge_cases.tcyr`** — safety-invariant section (8 assertions):
  - `>>` is logical/zero-fill (the PCG + bit-reversal code depends on it).
  - `_num_mulmod` (Russian-peasant) stays overflow-safe — Fermat check
    `2^(p−1) ≡ 1 (mod p)` for prime p≈1e11, whose intermediate squares (~1e22)
    far exceed i64 (a naive `a*b` would overflow).
  - PCG32 determinism/reproducibility for a fixed seed.
  - float-render `var buf[N]` independence across calls (`str_from_buf` copies,
    so the process-static buffer is safe).

### Notes (audit findings — no change needed)
- **Static buffers**: only two `var buf[32]` (float rendering), both passed to
  `str_from_buf`, which allocates + `memcpy`s — no escape, no cross-call
  corruption despite `var buf[N]` being process-static in Cyrius.
- **Bytes-vs-elements**: no bug — element arrays consistently use `alloc(n*8)`;
  the only stack arrays are byte buffers used as bytes.
- **Signed shifts**: every `>>`/`<<` operates on non-negative or masked values
  (FFT lengths, masked RNG state, non-negative exponents, bit-index masks), so
  logical `>>` is correct throughout. No `asr()` helper needed.
- **Overflow**: no factorial/binomial kernels exist (those are abaco's);
  Pollard-rho/modpow already use `_num_mulmod` to avoid `a*b` overflow. No
  unary `~` is used anywhere (all `~` are "≈" in comments).

## [2.3.2] - 2026-05-28 — Bounded einsum scratch memory (2.3.x arc)

A **memory-footprint** patch (not a speed change): einsum's parse/contraction
scratch now lives in a reused module-global arena instead of leaking to the
never-freeing bump allocator on every call. Internal-only; all 825 tests pass
with identical results.

This patch re-scoped 2.3.2 after verification (see Notes): the roadmap's
"replace hand-rolled transcendentals" item was a no-op — hisab already uses
the stdlib `f64_sin/cos/exp/ln/abs` named-op intrinsics everywhere, with zero
hand-rolled series — and arena adoption only fit einsum.

### Changed
- **einsum** (`src/einsum.cyr`): all ~11 per-call scratch allocations (operand
  specs, label dims, contraction sets, output shape) and the hot per-element
  `label_vals`/per-product `indices` buffers now come from a module-global
  arena (`_einsum_arena`, 8 KB) that is `arena_reset` at each call's entry.
  `label_vals`/`indices` are also hoisted to reused fixed-size buffers instead
  of being re-allocated per output element/product. einsum is non-reentrant
  and single-threaded; `tensor_new` copies the shape, so no scratch escapes
  into the returned tensor (verified).

### Performance (memory, not speed)
Measured per call for a 4×4 `ab,bc->ac` contraction (bump bytes consumed,
100-call average), before = HEAD's released 2.3.1 einsum:

- **3960 → 176 bytes/call** (~**22×** less). The 176 B is the result tensor
  the caller owns; the ~3784 B of per-call scratch leak is **eliminated** —
  scratch is now a one-time 808 B arena reused across all calls.
- For a consumer making N einsum calls: `N×3960` (unbounded growth) → `N×176
  + 808`. Speed is unchanged (arena alloc ≈ bump alloc, both O(1) bumps).

### Notes
- **Transcendentals already optimal**: `f64_sin/cos/exp/ln/abs` are stdlib
  named-op intrinsics (hardware FP on x86, aarch64 polyfills). hisab uses them
  directly (`f64_util.cyr` only composes them, e.g. `f64_tan = sin/cos`). No
  hand-rolled Taylor/Newton series exist to replace. No change made.
- **Arena scoped to einsum**: symbolic/tensor allocations are escaping result
  nodes (expr trees / tensors returned to the caller), not scratch, so an
  arena-reset would corrupt results — correctly left alone.

## [2.3.1] - 2026-05-28 — SIMD hot paths (2.3.x optimize/modernize arc)

First patch of the 2.3.x arc: route vec/mat/quat hot paths through Cyrius's
packed-double `f64v_*` builtins (added with hisab named as the gap-close
consumer). Internal-only — public API and results unchanged; all 825 tests
pass with **bit-identical** output to the scalar paths.

### Performance
Measured with an amplified microbench (loop timed by two `now_ns()` calls;
the committed `bench()` harness has a ~488 ns per-iteration floor that hides
op-level timings — see Notes). Before → after, ns/op:

- **vec4 dot**: 26 → 4 ns (**~6.5×**, no alloc)
- **vec4 add**: 46 → 20 ns (**~2.3×**)
- **vec3 dot**: 20 → 9 ns (**~2.2×**, n=2 xy + scalar z)
- **vec3 add**: 35 → 22 ns (**~1.6×**)
- **m4_mul**: 716 → 158 ns (**~4.5×** — scalar path made 64+ accessor calls)
- **m4_mul_vec4**: 106 → 59 ns (**~1.8×**)
- **m3_mul**: 349 → 108 ns (**~3.2×**, over-read-safe hybrid)

### Changed
- **vec4** (`src/vec4.cyr`): `dot`/`add`/`sub`/`scale` → `f64v_*` at n=4 (HVec4
  is exactly 4 contiguous f64; even n → no over-read). `length`/`length_sq`/
  `normalize` inherit via `dot`.
- **vec3** (`src/vec3.cyr`): `dot`/`add`/`sub`/`mul`/`scale` → over-read-safe
  hybrid (`f64v_*` at n=2 on the xy pair + scalar z tail). HVec3 is 24 B = 3
  doubles, so n=3 would read one f64 past the allocation.
- **mat4** (`src/mat4.cyr`): `m4_mul` and `m4_mul_vec4` → column-major linear
  combination (`f64v_scale`+`f64v_add` over contiguous 32-B columns).
  `m4_transform_point`/`dir` inherit it.
- **mat3** (`src/mat3.cyr`): `m3_mul`, `m3_mul_vec3` → over-read-safe hybrid.
- **quat** (`src/quat.cyr`): `dot` → `f64v_dot` n=4; `normalize` scale → `f64v_scale`.
  The Hamilton product stays scalar (not elementwise).

### Added
- **`tests/hisab.bcyr`**: amplified batch benchmarks (`vec3_dot_x64`,
  `vec4_dot_x64`, `m4_mul_x16`, `m4_transform_x64`) that loop N ops per timed
  call to clear the harness floor, so the SIMD wins are regression-trackable
  in `bench-history.csv`.

### Notes
- `f64v_*` are **global codegen intrinsics** (no `[deps]`/include needed). They
  MUST be reached through a function prologue: a bare `f64v_*` call from a
  top-level `.tcyr` statement SIGSEGVs on a misaligned stack (SSE alignment).
  All hisab ops are functions, so production + tests-via-wrappers are safe.
- FP associativity: SIMD pairwise-sum vs scalar left-fold gave bit-identical
  results across all 825 assertions — no tolerance regressions.

## [2.3.0] - 2026-05-28 — Cyrius 6.0.14 toolchain + library laid out in `src/`

Toolchain modernization to **Cyrius 6.0.14** (from 5.7.10) and a structural
move of the library source into `src/`, matching the sibling math engine
abaco's "proper library" layout. No behavioral changes — all 825 tests pass
unchanged. Reviewed abaco (modern patterns), and vidya + cyrius changelogs
(language updates) to scope the upgrade; the 6.0.x range is a low-risk jump
for a pure math library (no forced source-syntax breaks).

### Changed
- **Toolchain**: `cyrius.cyml` pin `5.7.10` → `6.0.14`. The 6.0.0 `cc5`→`cycc`
  binary rename is transparent to consumers (`cyrius build` dispatches; back-
  compat symlinks ship through 6.0.x).
- **Layout**: the 34 math modules moved `lib/*.cyr` → `src/*.cyr` (`git mv`).
  `lib/` is now exclusively vendored stdlib + first-party deps managed by
  `cyrius deps`; `src/` is the library source (smoke `main.cyr` + 34 modules).
  This stops the committed `./lib/` from shadowing the toolchain's
  version-pinned stdlib snapshot. Updated `[lib] modules` paths and the
  `include "lib/…"` → `include "src/…"` references across the 7 test/bench/
  fuzz/example files that exercise the library.

### Fixed
- **sakshi dependency**: `lib/sakshi_sakshi.cyr` was a dangling symlink into a
  stale `~/.cyrius/deps/sakshi/0.9.0` cache (a 5.7.x-era vendoring artifact),
  which broke the lockfile sha step (`sha256sum: No such file`). Cleared the
  bad 2.1.0 cache, re-resolved, and restored the vendored module as a regular
  file (per the 6.0.2 file-over-symlink fix). `[deps.sakshi]` (tag 2.1.0) is
  retained; `cyrius deps --verify` is green (60 verified, 0 failed — the lock
  now hashes only genuine vendored deps under `lib/`, down from 94 once the 34
  math modules moved to `src/`).
- **lint**: collapsed 7 "multiple consecutive blank lines" warnings in
  `src/geo_advanced.cyr` (4) and `src/spatial.cyr` (3) — newly gated now that
  the modules live under `src/`.
- **lint (global-init-order)**: 6.0.14's lint (the check landed at 5.7.32,
  after the old 5.7.10 pin) flagged 5 "global var init refs … (silent zero at
  init)" warnings in `tests/hisab.tcyr` — caused by duplicate top-level var
  names reused across test sections (`t`, `sx`, `sx2`, `se`), where an early
  reference resolved to a later redeclaration. Renamed the later (symbolic +
  tensor) instances to unique names (`symx`/`symx2`/`sexpr`/`tn`). Lint, fmt,
  and vet are now clean across the full CI file set (src + examples + tests).
- **fmt**: reformatted the whole tree (30 files) under 6.0.14's `cyrfmt` — the
  6.0.x formatter differs from the 5.7.10-era one, so files untouched by this
  change still carried latent drift. fmt is whitespace-only; 825/825 unchanged.

### CI (reviewed against abaco's workflows)
- **Fixed the fmt gate**: the old gate did `diff <(cyrius fmt "$f" --check) "$f"`
  — but `--check` writes its report to *stderr* and emits nothing on stdout, so
  the diff compared an empty stream to a non-empty file and failed **every**
  file. Switched to abaco's exit-code form (`cyrius fmt "$f" --check || fail`),
  aggregating across files so each drifted file is annotated.
- **Added a Security Scan job** (mirrors abaco): greps `src/` for raw
  `execve`/`fork` syscalls, `sys_system`, writes to `/etc|/bin|/sbin`, and
  oversized stack buffers (≥64 KiB). hisab's `src/` passes clean; now that the
  math modules live in `src/`, the scan covers the full library surface.
- **Hardened the docs version gate**: now also asserts the
  `version = "${file:VERSION}"` manifest template is intact (matches abaco),
  not just that the version appears in the CHANGELOG.

### Notes
- The empty 0-byte `cyrius.lock` (a symlink/file-copy bug present ecosystem-
  wide since 5.11.8, fixed at 6.0.2) now populates correctly — 60 SHA entries.
- `dist/hisab.cyr` (~545 KB / 16,195 lines) regenerated from the new `src/`
  paths; the `# Generated by: cyrius distlib` header keeps fmt/lint skipping
  the bundle (it exceeds the 512 KiB fmt/lint input cap).

## [2.2.2] - 2026-04-26 — Make it actually build under cc5 5.7.10 + full 34-module distlib

2.2.1 captured the manifest/CI/release modernization intent but was never
tagged because `cyrius build src/main.cyr` didn't pass — Cyrius 5.7.x's
new reserved keywords + an oversized include chain (the CLI prepended all
33 project modules, exceeding cc5's 512 KB input_buf) blocked the build.
2.2.2 closes those, picks up the **5.7.10** toolchain bump for its
`input_buf` 512 KB → 1 MB expansion (and 5.7.8's `syscall arity mismatch`
fix + lockfile-default + 5.7.9's duplicate-fn warning), restores
`lib/collision_core.cyr` + `lib/collision_mesh.cyr` to the bundle (after
fixing pre-existing syntax issues those files had carried since the
original Cyrius port — they were never actually compiled before), and
ships a full **34-module `dist/hisab.cyr` (~544 KB)** verified end-to-end
against a consumer build.

### Fixed
- **`lib/num_ext.cyr`**: renamed local variable `stack` → `stk` (6 identifier sites in `_factorize_pollard_rho`). `stack` became a reserved keyword in Cyrius 5.7.x; the four mentions in comments were left intact
- **`lib/collision_core.cyr`**: 3 empty-init / empty-step `for` loops converted to `while`. Cyrius's `for (init; cond; step)` requires *all three* clauses — `for (; cont == 1;)` and `for (; sj >= 0; sj = sj - 1)` were never valid syntax. The for-with-step variant kept its step semantics by appending `sj = sj - 1` to the loop body tail. File was never in the build chain pre-2.2.2 (orphan-include-after-syscall trick masked it), so this is the first time it actually parses
- **`lib/collision_mesh.cyr`**: same migration — one `for (var ti = 0; ti < n_tris;)` (empty step) converted to a manual `var ti = 0; while (ti < n_tris) { ...; ti = ti + 1 (or stay) }` (loop conditionally advances based on whether the current element was removed). Plus renamed local `shared` → `is_shared` (4 identifier sites — `shared` is reserved in Cyrius 5.5+; one comment mention left intact)
- **`lib/calc.cyr`** `_perm_init` (Perlin noise table): refactored the 18-arg `_perm_store_block(base, off, a..p)` helper into a 10-arg `_perm_store_8(base, off, a..h)` form, called 32× instead of 16×. **cc5 5.7.10 has a codegen bug at 18+ args** that scrambles register/stack params (args 1, 2, 7-12 silently read garbage values — see [`docs/development/issues/2026-04-26-cc5-18-arg-fn-scrambles-params.md`](docs/development/issues/2026-04-26-cc5-18-arg-fn-scrambles-params.md) for the full reproducer). Pre-fix: bench segfaulted at exit 139 inside `perlin_2d` because the permutation table got written full of garbage. Post-fix: `perlin_2d` runs through 200,000 iterations cleanly
- **`src/main.cyr`**: stripped from 30+ project-module includes down to the two stdlib includes its `fn main()` actually uses (`syscalls`, `io`). The previous form prepended every project module just to "validate the include chain" — but cc5 5.7.7's 512 KB input_buf can't fit that, and the test suites already cover include integration. Bonus: fixed three orphan `include` lines that sat *after* `syscall(SYS_EXIT, r)` (parsed but unreachable; first time any of them was scrutinized was when one tripped a parse error). CLI binary: now ~140 KB static ELF, prints the version string and exits
- **`tests/modules.tcyr`**: stripped six "multiple consecutive blank lines" lint warnings (lines 43-45, 263-264, 390 in the pre-fix file). `cyrius lint` returns the warning count as its exit code, which the prior CI loop swallowed under GHA's `set -eo pipefail` — the loop would abort on the first non-zero rc without reporting which file tripped it
- **`examples/basic_math.cyr` + `tests/{edge_cases,foundation,hisab,modules}.tcyr`**: applied `cyrius fmt` to flatten multi-line continuation-indent drift from the modern formatter
- **CI lint loop**: added `set +e` and per-file rc capture so warning-count exit codes don't abort the sweep before later files report. Adds `::error file=...::` annotations so the offending file is visible in the GHA UI

### Added
- **`dist/hisab.cyr`** — full **34-module** distlib bundle (~544 KB / 16,200 lines), regenerated from `[lib]` via `cyrius distlib`. Consumers pull it as `[deps.hisab] modules = ["dist/hisab.cyr"]` — single self-contained file, no per-module `include` choreography. Verified end-to-end: a fresh `[deps] stdlib = [...]` consumer build of an `hvec3_dot` example exits 0 and prints expected output. Fits cc5 5.7.10's 1 MB input_buf with ~480 KB headroom
- **CI distlib drift gate** in `ci.yml` — `cyrius distlib` runs on every push; CI fails if the committed `dist/hisab.cyr` differs from what the current `lib/` produces, so consumers always pull a fresh bundle that matches `lib/`
- **Release distlib regeneration** in `release.yml` — bundle is regenerated and shipped as `hisab-<TAG>.cyr` alongside the source tarball + linux binary + `SHA256SUMS`
- **`tests/modules.tcyr`** — added `collision_core` (`contact_new` + `ColContact_*` accessors) + `collision_mesh` (`detect_islands` smoke) test groups. 4 new assertions; total 253 in this suite, 825 across all four. Convex-hull / Delaunay / MPR are deliberately not exercised yet — they trip pre-existing algorithmic bugs unrelated to the syntax migration; coverage will land alongside an audit pass in a follow-up release
- **`docs/development/tool-issues.md`** — catalog of Cyrius / cc5 / cbt / cyrius-lint / cyim quirks hisab development has tripped on. Language-specific items each have a deep-dive at `docs/development/issues/<DATE>-<slug>.md` with self-contained reproducers + verification steps, so the cc5 maintainer agent can pick them up when there's a slot. Five issues catalogued at 2.2.2: 18-arg fn miscompile, `for` empty-clauses parse, CLI clobbers source on unknown flag, `cyrius lint` rc-as-warning-count, `cbt modules` substring false-positive in `[build]` comments

### Changed
- **Toolchain pin**: 5.7.7 → **5.7.10**. Picks up:
  - **5.7.8** — cc5-level fix for the noisy `lib/syscalls_x86_64_linux.cyr:358: syscall arity mismatch` warning (was firing on every build of every downstream that includes syscalls), `cyrius deps` writing `cyrius.lock` by default, `cyrius check` no longer auto-prepending manifest deps, and the new `cyrius build --no-deps` flag
  - **5.7.9** — `warning: duplicate fn '<name>' (last definition wins)` at registration time (hisab build emits zero such warnings); `json_build` cross-module collision resolved upstream via patra rename
  - **5.7.10** — `input_buf` 512 KB → **1 MB** heap-map reshuffle (+0x100000 region shift). Hisab was the load-bearing reason for this bump per the cc5 5.7.10 release header — `dist/hisab.cyr` was at 96 % of the old cap with hisab actively censoring upstream to stay under
- **`[lib]` block**: full 34-module list restored (32-module trimmed bundle from earlier 2.2.2 drafts is no longer needed). `lib/collision_core.cyr` + `lib/collision_mesh.cyr` re-included after their syntax fixes
- **`README.md` quick-start**: `[deps.hisab] modules = ["dist/hisab.cyr"]` example back; toolchain version → 5.7.10
- **`CLAUDE.md` status + layout**: reflects the CLI smoke binary + 34-module distlib bundle; CI/Release section restores the distlib drift gate

### Local verification
All gates pass locally on this branch:
- `cyrius lint` — clean across 8 files (4 tcyr, 1 bcyr, 1 fcyr, 1 example, src/main.cyr)
- `cyrius fmt --check` — no drift
- `cyrius vet src/main.cyr` — clean
- `cyrius distlib` — `dist/hisab.cyr` = 544 KB / 16,200 lines (34 modules)
- `cyrius build src/main.cyr build/hisab` — OK, 143 KB static ELF, magic verified
- `./build/hisab` — prints `hisab 2.2.2`, exit 0
- `cyrius test tests/hisab.tcyr` — 116/116
- `cyrius test tests/foundation.tcyr` — 307/307
- `cyrius test tests/modules.tcyr` — **253/253** (added 4 collision smoke assertions)
- `cyrius test tests/edge_cases.tcyr` — 149/149
- Total: **825/825 assertions, 0 failed**
- `cyrius build tests/hisab.fcyr` + 5 s timeout run — fuzz: ok
- `cyrius bench tests/hisab.bcyr` — 22 benchmarks complete
- **Consumer smoke**: a fresh `[deps.hisab] modules = ["dist/hisab.cyr"]` consumer project compiles + runs the bundle end-to-end (exits 0, prints expected output)

### Known limitations (deferred)
- `convex_hull_2d` / `triangulate_polygon` / `mpr_intersect` / `delaunay_2d` / `halfedge_from_triangles` / `sequential_impulse` — collision_core and collision_mesh now compile and link, but the algorithms themselves trip runtime bugs (e.g. `convex_hull_2d` hits a `vec: index < 0` bounds check on a 5-point input). These predate the cc5 5.7.x port — the files were added in 2.2.0 but never actually exercised because they sat outside the build chain. Audit + fixes are queued for a follow-up release; for now `tests/modules.tcyr` only smoke-tests the API surface (`contact_new` + accessors, `detect_islands`)
- `lib/` still mixes vendored stdlib + project source — a yukti-style split (project source in `src/*.cyr`, `lib/` purely deps + gitignored) is a natural future restructure but out of 2.2.2's scope

## [2.2.1] - 2026-04-26 — Cyrius 5.7.7 modernization + distlib bundle

Toolchain catch-up release: spans the full 5.x line in one bite, lands the
distlib-driven distribution model, and brings CI / release / scripts into
line with first-party Cyrius project conventions (yukti / sakshi / patra).
No source changes to the math modules themselves — pure scaffold rework.

### Changed
- **Toolchain bump**: Cyrius **4.10.3 → 5.7.7**. Picks up the manifest format, `cyrius distlib` multi-profile, `cyrius.lock`, `${file:VERSION}` interpolation, `#deprecated("...")` attribute, structured-deps protocol, fixup-table cap 262K → 1M, atomic `cyrius build` output (failed compile no longer destroys an existing binary)
- **Manifest migration**: `cyrius.toml` → **`cyrius.cyml`**. Adds `language = "cyrius"`, `cyrius = "5.7.7"` toolchain pin in `[package]`, and `version = "${file:VERSION}"` interpolation so VERSION is the single source of truth
- **Toolchain pin location**: `.cyrius-toolchain` (legacy) removed — pin lives in `cyrius.cyml [package].cyrius` per Cyrius 5.5.41+ convention; CI/release grep the manifest directly
- **Dep bump**: sakshi **0.9.0 → 2.1.0** (modules path now `dist/sakshi.cyr` per the distlib convention)
- **`scripts/version-bump.sh`**: was editing `Cargo.toml` + running `cargo generate-lockfile` — now just writes `VERSION` (manifest auto-syncs via `${file:VERSION}`); validates semver shape; prints next-step hints for CHANGELOG section + tag
- **`scripts/bench-history.sh`**: was running `cargo bench --bench benchmarks` and parsing criterion output — now runs `cyrius bench tests/hisab.bcyr` and parses `lib/bench.cyr` output; tolerant unit normalization (ps/ns/µs/ms/s); 3-point trend table generation unchanged
- **`CLAUDE.md`**: full rewrite for the actual Cyrius project — replaced Rust idioms (cargo/clippy/RUSTDOCFLAGS, MSRV 1.89, `#[non_exhaustive]`) with the real toolchain (`cyrius lint/fmt/vet/build/test/bench/distlib`); refreshed layout, deps, key principles, CI gates, doc structure
- **`CONTRIBUTING.md`**: prereq updated to Cyrius 5.7.7+ via `cyrius.cyml`
- **`README.md`** quick-start: `cyrius.toml` snippet → `cyrius.cyml` with full `[package]` + first-party `[deps.hisab]` example pointing at `dist/hisab.cyr`
- **`.gitignore`**: dropped Rust-only entries (criterion, proptest, tarpaulin, cargo-vet); added `cyrius-*.tar.gz` (CI download) and `.claude/`
- **`docs/development/dependency-watch.md`**: toolchain status updated; sakshi reclassified as first-party; 5.x line notes added

### Added
- **`[lib]` modules block** in `cyrius.cyml` listing 34 hisab modules in dependency order (foundation → types → derived) — input to `cyrius distlib`
- **`dist/hisab.cyr`** distribution bundle (regenerated from `[lib]` via `cyrius distlib`). Consumers pull it with `[deps.hisab] modules = ["dist/hisab.cyr"]` — single self-contained module, no per-file `include` choreography needed
- **`.github/workflows/ci.yml`** rebuilt: lint, fmt-check, vet, **distlib drift check** (regenerates `dist/hisab.cyr` and fails if committed bundle is stale), build (with ELF magic check), test (auto-discover `tests/*.tcyr`), fuzz (auto-discover `tests/*.fcyr` with 10s timeout per harness), bench (auto-discover `tests/*.bcyr`), docs + version-consistency gate
- **`.github/workflows/release.yml`** rebuilt: CI gate, version-verify (tag matches VERSION; supports both `1.2.3` and `v1.2.3`), distlib regeneration, source tarball + `dist/hisab.cyr` + linux binary + `SHA256SUMS` upload, changelog-section extract for release body
- **`cyrius deps --verify`** lock-gate in CI (gated on `cyrius.lock` presence)

### Removed
- `.cyrius-toolchain` — superseded by `cyrius.cyml [package].cyrius`
- `cyrius.toml` — superseded by `cyrius.cyml`

### Notes
- `lib/collision_mesh.cyr` is included in the `[lib]` bundle. The 2.2.0 changelog flagged it as "exceeds cc3 1MB preprocess buffer, ships with Cyrius 5.0"; cc5 5.7.7 has lifted the relevant caps. If a downstream `cyrius build` of `dist/hisab.cyr` fails on this file, drop it from the `[lib]` list and reopen the deferral
- `lib/` still mixes vendored stdlib + project source. yukti-style separation (project source in `src/*.cyr`, `lib/` purely deps + gitignored) remains a future restructuring decision

## 2.2.0 (2026-04-15) -- Geometry & group extensions

### Added
- **lie_ext.cyr** (523 lines) -- SE(3) rigid body motions (exp/log, compose, transform), SO(3) explicit (Rodrigues, exp/log), adjoint representations (SU(2), Lorentz, SE(3)), Baker-Campbell-Hausdorff 2nd/3rd order
- **spatial.cyr** (864 lines) -- k-d tree (build, nearest, within_radius), quadtree (insert, query), octree (insert, query), spatial hash (insert, query_cell, query_radius, clear)
- **collision_core.cyr** (574 lines) -- MPR/XenoCollide (intersect + penetration), sequential impulse solver with friction, convex hull 2D (Andrew's monotone chain), polygon triangulation (ear clipping)
- **collision_mesh.cyr** (522 lines, written but deferred) -- Delaunay triangulation (Bowyer-Watson), half-edge mesh, island detection (union-find). Exceeds cc3 1MB preprocess buffer. Ships with Cyrius 5.0.

### Changed
- All .cyr files cleaned of Unicode in comments (em-dashes, Greek letters -> ASCII)

### Known issue
- Cyrius cc3 1MB preprocess buffer limit reached at ~16K lines. collision_mesh.cyr deferred to Cyrius 5.0.

## 2.1.0 (2026-04-15) -- Precision + depth

### Added
- **linalg_precision.cyr** (1,124 lines) -- Golub-Kahan SVD (full precision, replaces A^T*A), QR eigendecomposition O(n^3) (replaces Jacobi O(n^5)), complex Householder QR
- **noise_simplex.cyr** (343 lines) -- OpenSimplex2 2D+3D with fBm layering
- **einsum.cyr** (305 lines) -- Einstein summation notation parser (`"ij,jk->ik"`)

### Changed
- Audit M8 (SVD precision) resolved

### Performance
- `eigen_qr`: O(n^3) for symmetric eigenproblems (was O(n^5) with Jacobi for large n)

## 2.0.0 (2026-04-15) -- Cyrius port

**Breaking: complete rewrite from Rust to Cyrius.** New language, new API, new binary format. Rust source available via pre-2.0 git tags.

### Changed
- **Language**: Rust -> Cyrius (self-hosting systems language, static ELF binaries)
- **Types**: glam f32 SIMD types -> native f64 heap-allocated types (HVec2/3/4, HQuat, Mat3, Mat4)
- **Errors**: `Result<T, HisabError>` -> integer error codes (ERR_NONE, ERR_SINGULAR_MATRIX, etc.)
- **API**: method syntax (v.dot(w)) -> free functions (hvec3_dot(v, w))
- **Precision**: f32 (1e-7) -> f64 (1e-12) everywhere
- **Dependencies**: 9 Rust crates -> 1 Cyrius dep (sakshi)
- **Binary**: ~800KB dynamic -> 420KB static ELF

### Added -- 27 library files, 11,943 lines

**Foundation (8 files):** error, f64_util, vec2, vec3, vec4, quat, mat3, mat4
**Transforms (2 files):** transforms (T2D/T3D, Euler, screen), color (sRGB, Porter-Duff, tone mapping, SH, EV)
**Geometry (2 files):** geo (9 primitives, 6 ray tests), geo_advanced (GJK/EPA, BVH, SDF, CGA 5D)
**Calculus (2 files):** calc (integration, Bezier, easing, Perlin), calc_ext (gradient/Jacobian/Hessian, B-spline, NURBS, Hermite, monotone cubic, 3D Perlin)
**Numerical (5 files):** num (roots, FFT, RK4, PCG32, primes), ode (DOPRI45, BDF, symplectic), optimize (GD, CG, BFGS, L-BFGS, LM), linalg_ext (CSR, GMRES, BiCGSTAB, PGS, SVD, eigen, inertia), num_ext (extended GCD, totient, Mobius, factorize, CRT, DST/DCT, 2D-FFT, Halton/Sobol, tridiagonal)
**Physics (3 files):** complex (numbers + matrices, Pauli, Dirac, matrix exp), lie (U(1), SU(2), SU(3), SO(3,1)), diffgeo (Christoffel -> Einstein, geodesics, exterior algebra)
**Symbolic (2 files):** symbolic (expr tree, eval, diff, simplify), symbolic_ext (integration, LaTeX, pattern matching)
**Other (3 files):** autodiff (dual numbers), interval (arithmetic), tensor (N-D dense, contraction, physics tensors)

### Security -- P(-1) audit (31 issues found, 25 fixed)
- Allocation overflow guards (tensor, complex matrix, diffgeo dim cap, sieve cap)
- Division-by-zero guards throughout (complex, autodiff, transforms, depth)
- m4_determinant rewritten with correct cofactor formula
- tensor_contract implemented (was returning zeros)
- BDF-5 coefficients recomputed exact (IEEE 754 verified)
- Bisection midpoint overflow fix, CG upgraded to Polak-Ribiere+
- modpow overflow-safe via Russian peasant multiplication
- expr_eval warns instead of aborting process

### Testing
- 821 assertions (4 suites), 22 benchmarks, 5 fuzz targets

### Documentation
- README, CONTRIBUTING, SECURITY updated for Cyrius
- Architecture overview, testing guide, threat model, dependency watch
- P(-1) audit report, Rust vs Cyrius benchmark comparison
- Working example (examples/basic_math.cyr)

---

## Rust era (archived in pre-2.0 git tags)

### 1.4.0 (2026-03-30) -- Theoretical physics foundation
- Complex linear algebra (ComplexMatrix, Hermitian eigen, complex SVD, Pauli/Dirac, spinors, matrix exp)
- Indexed tensor algebra (Einstein summation, contraction, raising/lowering)
- Symmetric/antisymmetric/sparse tensors
- Lie groups (U(1), SU(2), SU(3), SO(3,1), exponential maps, Casimir)
- Differential geometry (Christoffel, Riemann, Ricci, Einstein, geodesics, Killing, exterior algebra)
- Conformal geometric algebra (5D CGA multivectors)

### 1.3.0 (2026-03-27) -- Number theory + symbolic
- Prime sieves, primality, factorization, modular arithmetic
- Symbolic integration, LaTeX, pattern matching, abaco bridge

### 1.2.0 (2026-03-27) -- Interpolation + color
- Inverse lerp, remap, reverse-Z, HSV/HSL/Oklab, Porter-Duff, compensated summation

### 1.1.0 (2026-03-25) -- Feature completion
- Symplectic integrators, SDFs, DualQuat, SH, stiff ODE, SDE, eigen, reverse-mode AD

### 1.0.0 -- Stable release
- Core: transforms, geo, calc, num, autodiff, interval, symbolic, tensor, parallel, ai, logging
