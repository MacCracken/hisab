# A per-step neighbour search leaks through `spatial_hash_query_cell`: every query returns a fresh vec — closed in 3.4.2

**Status:** ✅ **CLOSED in 3.4.2** (2026-11-14). Opened on hisab 3.4.1 with cyrius 6.7.18 (x86_64), 2026-11-11.
**Discovered:** 2026-11-11, by pravash's Rust → Cyrius port (M0 step 9, probe 1).
**Reporter:** pravash 2.0.0 port (its request P2, conversion ledger L4; `pravash/docs/development/pravash-port-plan.md` §8).
**Related:** this file's sibling `2026-11-09-allocator-scoped-calls.md` (H9) and cyrius S1
(`docs/development/issues/2026-11-09-no-scoped-allocator-for-alloc.md`). Either would also contain this leak. This
request is narrower: a query that writes into the caller's storage needs neither.
**Severity:** **High for SPH and any particle method** that rebuilds a neighbour hash per step. Nothing fails; memory
grows without bound.

## Summary

SPH rebuilds its neighbour hash every step: clear it, insert every particle, then look up the 27 cells around each
particle (pravash 1.3.0's `SphSolver::build_neighbors`, on hisab 1.4.0's `SpatialHash::query_cell`, which returns a
borrowed slice). With hisab 3.4.1, the rebuild itself is fine: after the first step, `spatial_hash_clear` and
`spatial_hash_insert` reuse their storage and grow nothing. **Every `spatial_hash_query_cell`, though, returns a
newly allocated vec**, about 152 B each, which the bump heap never gets back:

| Per step, after the first, 1,000 particles | Global heap, never returned |
|---|---|
| `spatial_hash_clear` + 1,000 `spatial_hash_insert` | 0 B |
| 27,000 `spatial_hash_query_cell` | **4,104,000 B** |

At 10⁴ particles that is about 41 MB a step, or 2.5 GB a second at 60 Hz. pravash therefore keeps its own neighbour
grid (sorted arrays) as a marked stand-in, and will switch to hisab's when this lands.

## Reproduction

[`2026-11-11-spatial-hash-per-step-rebuild-allocates-repro.cyr`](2026-11-11-spatial-hash-per-step-rebuild-allocates-repro.cyr),
from the repo root:

```sh
cyrius build docs/development/issues/archived/2026-11-11-spatial-hash-per-step-rebuild-allocates-repro.cyr /tmp/p2
/tmp/p2; echo "exit=$?"
# per step after the first, 1,000 particles: clear + insert grew the heap by 0 B, the 27,000 query_cell calls by 4104000 B
# exit=1
```

It exits 1 while a step after the first grows the heap.

## Ask

A query that doesn't allocate. Either form works for pravash:
- **`spatial_hash_query_cell_into(sh, point, out, cap) → count`**: write the cell's data words into the caller's
  buffer (up to `cap`), and return how many the cell holds, so a caller with too small a buffer can grow it and ask
  again; or
- **a chain cursor**: `spatial_hash_cell_first(sh, point) → entry` and `spatial_hash_entry_next(sh, entry) → entry`,
  with `spatial_hash_entry_data(sh, entry)`, walking the cell in insertion order.

Both must keep the order the data comes back in: **insertion order within a cell**. SPH's sums follow the candidate
order, so it is part of the result (pravash's port-parity §4.4 shows the oracle's lists are exactly (cell offset,
insertion)).

## Notes from the consumer

- pravash 1.3.0 hashes **f32** positions (hisab 1.4.0's `SpatialHash` takes `Vec3`). 3.4.1's hash is f64, which
  pravash will move to before its 2.0 anyway (its decision D3: f32 keys lose cells at |x| ≈ 10⁶).
- With a non-allocating query, pravash's stand-in converts to hisab's hash. Its parity rows (neighbour lists,
  class A) must then come out unchanged.

### The stand-in's numbers (pravash M4 part b, 2026-11-12)

pravash's stand-in, `PvNeighborGrid` in `src/neighbor.cyr` (ledger L4), sorts the points on (cell key, index), keeps
each cell as a run, and finds cells by binary search. It allocates nothing once its columns have grown, and its lists
equal the oracle's on every row (30 rows, class A, including large offsets and a grid kept within f32::EPSILON).
Pinned to one core, one build of a 1,000-particle grid (h 0.05, 27 probes a particle):

| Lattice | Rust 1.3.0 on hisab 1.4.0 (`HashMap`) | the stand-in |
|---|---|---|
| 2D, 40 wide (36 neighbours a particle) | 0.81–0.83 ms | 1.24–1.25 ms |
| 3D, 10³ (216 at the interior) | 0.83–0.84 ms | 1.60–1.64 ms |

A converted hash must keep the stand-in at no allocation per build, and the lists in insertion order within a cell.

### The radius query's order (pravash M6 part a, 2026-11-12)

pravash's `drag_from_particles_indexed` takes a hash the caller filled with each particle's position and index. It
sums the velocities of the candidates `query_radius` returns, within the sample radius of the body's surface, in the
order they come back. So that order is part of the result, as the neighbour lists' is.

- **hisab 1.4.0** walks the block (2⌈r/cell⌉ + 1)³ about the centre's cell, x outermost, each cell in insertion order.
  It walks every cell of the block, so an infinite radius never returns. Rust's indexed drag on a 2D cloud of 625
  particles, cell 0.05, radius 0.25, makes 1,331 lookups for 121 occupied cells and runs 18× its own brute force
  (27 µs against 1.5 µs).
- **3.4.1's** `spatial_hash_query_radius` keeps that order on its cell walk, but its `full_scan` plan (taken when the
  block has more cells than the hash has entries) returns entries in insertion order instead. Its comment says the
  multiset is identical and only the visit order differs. A sum over the result then takes different bits depending
  on the plan, and the plan depends on the radius and the hash's size.

pravash's stand-in for now is a cells-only `PvNeighborGrid` built over the particles (ledger L4, ADR 0002 #23). It
sorts its cells into (x, y, z) order once per build, and its query walks only the occupied cells of the block, in that
order, with binary searches to skip runs outside it. On the 625-particle cloud above it takes 19 µs, against Rust's
27 µs. On a 10,000-particle block, with 121 occupied cells in the block, it takes 79 µs against Rust's 35 µs. The
build is 3.4–5.1× hisab 1.4.0's `HashMap` inserts.

For the conversion, a radius query also needs **one order on every plan**: cells in (x, y, z) order, each cell's
entries in insertion order. That is the cell walk's order, so a full scan would have to sort its matches by cell. A
query that writes into caller storage (above) and visits only occupied cells would serve both pravash's SPH lists and
its drag.

## Closed in 3.4.2

**The repair is the ask's first form, and the radius query's order.**
- **`spatial_hash_query_cell_into(sh, point, out, cap)`:**
  - writes the cell's data into the caller's buffer, in insertion order (the oldest `cap` when the cell holds more);
  - returns how many the cell holds;
  - allocates nothing.
- **`spatial_hash_query_radius_into(sh, point, radius, out, cap)`** is the same for the radius query.
- **One order on every plan:** the block's cells in (x, y, z) order, x outermost, and each cell's entries in insertion
  order. `spatial_hash_query_radius`'s flat scan and its radius +Inf path now sort their matches into the cell walk's
  order (O(m log m)); through 3.4.1 they answered in insertion order. The `_into` form keeps the first `cap` in that
  order with a heap kept inside the caller's buffer.

**Not added:**
- **The chain cursor.** The issue accepted either form.
- **A walk that visits only occupied cells.** The radius query still probes every cell of the block, and the flat scan
  bounds that by the entry count.

**Paired run, 2026-11-14 (cyrius 6.7.18):**
- `2026-11-11-spatial-hash-per-step-rebuild-allocates-repro.cyr` (moved beside this file) exits **1** on 3.4.1 and on
  3.4.2 as filed: 4,104,000 B per step from `spatial_hash_query_cell`.
- `…-repro-3.4.2.cyr`, the same step on `spatial_hash_query_cell_into`, exits **0**: 0 B.
- `tests/modules_c.tcyr`, group "3.4.2: spatial hash queries into caller storage, one radius order on every plan",
  pins the 0 B, the sequences against a brute force on both plans, and every cap.
