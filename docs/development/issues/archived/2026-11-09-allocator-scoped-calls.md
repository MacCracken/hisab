# A per-step consumer can't contain what hisab's calls allocate: every returned value is on the global bump heap — closed in 3.4.2

**Status:** ✅ **CLOSED in 3.4.2** (2026-11-14), repaired in hisab: cyrius S1 is still open at 6.7.18. Opened 3.4.1, verified with cyrius 6.7.18 (x86_64), 2026-11-09.
**Discovered:** 2026-11-09, by impetus's Rust → Cyrius port (M0 step 9, probe 1), on a physics step.
**Reporter:** impetus 2.0.0 port (its request H9, port plan §8).
**Related:** cyrius `docs/development/issues/2026-11-09-no-scoped-allocator-for-alloc.md` (S1, open). If S1 lands,
this issue closes as superseded with no hisab change.
**Severity:** **High for any long-running consumer that calls hisab per frame, step or request.** Nothing fails;
memory grows without bound.

## Summary

hisab's convention is that every `HVec3`, `HQuat` or matrix result, and every internal working set, is a fresh
`alloc` (574 `alloc` sites in the 3.4.1 bundle). `alloc` is cyrius's bump allocator: it has no free, and nothing
lets a caller route it to an arena. A library consumer can't reclaim it either: `alloc_reset` also frees the
consumer's own live blocks (S1's repro shows this).

So a consumer that calls hisab once per step leaks every step, for the life of the process. Measured by the repro
below (the same numbers as `impetus/docs/development/port-parity.md` §4.1):

| Call | Global heap per call, never returned |
|---|---|
| `hvec3_add`, `hvec3_cross`, `hquat_rotate_vec3` | 24 B |
| `m3_mul_vec3` | 48 B |
| `gjk_epa_3d`, two overlapping unit spheres | 105,920 B |
| `detect_islands`, 100 bodies, 50 contacts | 10,936 B |
| `bvh_build`, 100 boxes | 20,792 B |

A 3D scene with 100 touching pairs leaks about 10 MB per step through `gjk_epa_3d` alone, or 600 MB a second at
60 Hz. impetus therefore keeps hisab off its step path and out of its queries (impetus ADR 0007, amendment), and owns
a stopgap copy of the 3D math and GJK/EPA (impetus ADR 0012). Every per-frame consumer is in the same position:
kiran, soorat and impetus.

## Reproduction

[`2026-11-09-allocator-scoped-calls-repro.cyr`](2026-11-09-allocator-scoped-calls-repro.cyr), from the repo root:

```sh
cyrius build docs/development/issues/archived/2026-11-09-allocator-scoped-calls-repro.cyr /tmp/h9
/tmp/h9; echo "exit=$?"
# hvec3_add: 24 bytes of global heap per call, never returned
# hvec3_cross: 24 bytes of global heap per call, never returned
# hquat_rotate_vec3: 24 bytes of global heap per call, never returned
# m3_mul_vec3: 48 bytes of global heap per call, never returned
# gjk_epa_3d (two spheres): 105920 bytes of global heap per call, never returned
# detect_islands (100 bodies, 50 contacts): 10936 bytes of global heap per call, never returned
# bvh_build (100 boxes): 20792 bytes of global heap per call, never returned
# 7 of 7 calls leak
# exit=1
```

It exits 1 while any measured call grows the global heap. Once a fix lands, switch its calls to the scoped form and
it exits 0.

## Proposed fix

**Preferred: nothing in hisab.** S1 (an allocation scope that `alloc` honours) contains all 574 sites at once, with no
API change. This issue exists in case S1 is declined or delayed.

**Otherwise, allocator-scoped variants of the calls a step makes**, in two shapes:

- **Value operations take an output** (impetus's H1): `hvec3_add_into(out, a, b)`, `hvec3_cross_into`,
  `hquat_rotate_vec3_into`, `m3_mul_vec3_into`, and the quaternion and matrix products. They allocate nothing, and
  they are also the fast path that probe 2 measured (impetus `port-parity.md` §4.2).
- **Working-set operations take an allocator** (`Allocator`, dispatched through `alloc_via`), or a caller-owned
  scratch that hisab grows and reuses: `gjk_epa_3d_a(al, ..)`, `bvh_build_a(al, ..)`, `detect_islands_a(al, ..)`.
  - Every internal allocation goes to `al`: the simplex, the EPA polytope and its face and edge lists, the BVH
    nodes, the island arrays.
  - The support-callback contract needs an allocation-free form too, such as writing the support point to an
    out-parameter. Today's callbacks return a fresh `HVec3`, so a scoped GJK would still leak through them.
  - The consumer then resets its step arena once per step, which reclaims exactly that step's hisab memory.

The existing functions stay as they are, so no current consumer changes.

## Consumer-side workaround

impetus keeps world storage on `freelist` and per-step scratch on a growable arena reset every step; both are
measured flat. Its 3D vector, quaternion and matrix math is a by-value transcription of glam 0.29.3, and its GJK/EPA
is its own Rust port (impetus ADR 0012). When this or S1 lands, impetus replaces each of those paths with the hisab
call, one change at a time, and supersedes ADR 0012.

## Close

Close with a run of the repro that exits 0, on the scoped calls or under S1's scope.

## Another per-step consumer: pravash (2026-11-11)

pravash's Rust → Cyrius port measured its own step paths the same way (its `port-parity.md` §4.1, probe 1;
conversion ledger L3 and L8):

| Call | Global heap, never returned |
|---|---|
| `hvec3_add` (glam's DVec3 arithmetic, all over SPH) | 24 B per call |
| `spatial_hash_query_cell`, 27 per particle per step | **4,104,000 B per step for 1,000 particles**: filed narrowly as `2026-11-11-spatial-hash-per-step-rebuild-allocates.md` |
| `num_dst` / `num_idst`, n = 62 | 10,208 B per call: filed as `2026-11-11-dst-allocates-per-call.md` |
| an `ad_tape` gradient after `ad_tape_reset` | **0 B**: the tape already reuses its storage |

pravash, like impetus, keeps hisab off its step path until this or S1 lands: freelist and a per-step arena are flat
for it too. Scoped calls would let it retire three marked stand-ins at once: its vector math, its neighbour grid and
its DST.

## Closed in 3.4.2

**The repair is a scope in hisab: S1's shape, for hisab's own allocations.** `hisab_alloc_scope(al)` (`src/heap.cyr`)
routes every allocation hisab's code makes to the caller's `Allocator` until the next call, and returns the scope it
replaced. Every `alloc` site in `src/` became `hisab_alloc`, every vec `hisab_vec_new` / `vec_push_a(hisab_allocator(), …)`,
and every Str the symbolic modules build takes the scope's allocator too. Memory from a scope is zeroed, as `alloc`'s is.
`scripts/check-heap-globals.sh` now refuses a new direct call of a global-heap stdlib function in `src/`.

- **What it does not reach:** eight public functions call ganita's matrix routines, which allocate inside ganita:
  `inertia_box`, `inertia_sphere`, `mat_new_guarded`, `svd_compute_checked`, `svd_truncated`, `eigen_qr`,
  `svd_golub_kahan`, `opt_levenberg_marquardt`. A sweep of every one of `tests/hisab.bcyr`'s 375 calls under a scope:
  364 take 0 B of global heap, 9 reach those eight, and 2 allocate in the benchmark itself.
- **The value operations take an output (impetus's H1):** `hvec3_add_into`, `hvec3_sub_into`, `hvec3_scale_into`,
  `hvec3_cross_into`, `hquat_mul_into`, `hquat_rotate_vec3_into`, `m3_mul_vec3_into`, `m3_mul_into`, `m4_mul_into`.
  Each gives its allocating form's bits, allocates nothing, and accepts an operand as `out`.
- **Not done as proposed, and why:**
  - There are no per-call `gjk_epa_3d_a` / `bvh_build_a` / `detect_islands_a`. The scope covers those three and every
    other call, with no new form per function.
  - There is no allocation-free support-callback contract. Under a scope the support function's own `hvec3_*` calls are
    scoped, and memory it allocates itself is the consumer's.
- **The contract a consumer reads (`src/heap.cyr`):** a reset of the scope's allocator invalidates every value hisab
  returned under it, and every structure hisab grew under it. Build long-lived structures (a hash, a tree, a tape)
  outside a scope that is reset. There is one scope per process.

**Paired run, 2026-11-14 (cyrius 6.7.18):**
- `2026-11-09-allocator-scoped-calls-repro.cyr` (this file's repro, moved beside it) exits **1** on 3.4.1, and still
  exits 1 on 3.4.2 as filed: unscoped calls take global heap as they did.
- `2026-11-09-allocator-scoped-calls-repro-3.4.2.cyr`, the same seven calls each run as a scoped step with the arena
  reset after it, exits **0**: 0 B of global heap per step after the first, for all seven.
- `tests/modules_c.tcyr`, group "3.4.2: the allocation scope", pins it in the suite: the step's answers are bit for
  bit the global heap's, and it takes 0 B a step.
