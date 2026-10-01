# 2026-09-21 — cyrius `lib/bench.cyr` 6.6.5+: a fixed-batch window below the resolution bar resolves only when something slow happens in it, so the printed `min=`/`max=` are the extremes of the SLOW windows and `min` sits ABOVE `avg`

**Component:** cyrius stdlib `lib/bench.cyr` — the 6.6.5 resolution rule (`_bench_record`:
a window enters `min`/`max` only if `net >= 100 × (clock read + tick)`), the residue of the
`min-minus-mean-floor` repair.
**Toolchain seen:** cyrius **6.6.6** (and 6.6.5), x86_64, hpet/tsc box (floor 336–341 ns, tick
330–340 ns → bar ≈ 68 µs). 6.6.4 and below do not have this shape.
**Severity:** Medium — a printed statistic that cannot be what it says (a minimum above the mean of
the same sample), with `bench_min_resolved()` answering 1. **No hisab answer is wrong**: hisab's
CSV records `avg`, and `avg` is unaffected (below).
**Status:** ✅ **CLOSED 2026-09-30 (hisab 3.2.2, cycc 6.6.6 → 6.6.12) — FIXED UPSTREAM IN cycc
6.6.9**, bite 6: min/max are now decided for the ROW by op count (`min_k × mean` against the bar),
never per window by its own duration, and `bench_run` books its pilot and growth chunks with the next
sized chunk, so `min <= avg` holds on every row. cyrius archived the filing as
`cyrius/docs/development/issues/archived/2026-09-21-hisab-bench-min-above-mean-below-resolution-bar.md`.
Verified here as a pair rather than read off the changelog — see **Closed** at the end.

## What hisab sees

`tests/hisab.bcyr` registers 39 rows through `bench_batch(name, fn, 2000, 200)` — a **fixed** 2000-op
window. For a sub-40 ns op that window nets 30–60 µs, **below** the 68 µs bar on this box, so a
plain window can never resolve; only a window that contained something slow (interrupt, page fault)
crosses the bar, and the row's `min=`/`max=` are then the extremes of those windows only:

```
vec3_add: 16ns avg (min=39ns max=43ns) [400000 iters]      # 6.6.6 instrument
vec3_add: 16ns avg (min=15ns max=44ns) [400000 iters]      # 6.6.4 instrument, same compiler
```

Measured with three binaries interleaved ×4 on a quiet box (2026-09-21): **A** 6.6.4 compiler +
6.6.4 `bench.cyr`, **B** 6.6.6 compiler + 6.6.4 `bench.cyr`, **C** 6.6.6 compiler + 6.6.6
`bench.cyr` (the shipped 3.2.1 harness):

| binary | rows with `min > avg·1.02` | which |
|---|---|---|
| A | 0 / 320 | — |
| B | 0 / 320 | — |
| C | **24 / 320** | `vec3_add` (4/4 runs, min 2.4–2.6× avg), `vec3_cross` (4/4), `vec3_lerp` (4/4), `vec2_lerp` (4/4), `tonemap_reinhard` (4/4), `num_gcd` (2/4), `vec3_normalize` (2/4) |

Every affected row is one whose 2000-op window sits under the bar; `quat_mul` (36 ns × 2000 = 72 µs)
and everything slower is consistent (`min/avg` 0.92–1.00). `avg` between B and C: median **+0.00%**,
mean +0.31% over 80 rows — the statistic hisab records did not move.

## Consequence for hisab, and the workaround

- `bench-history.csv` rows written under 6.6.5+ carry a `min_ns`/`max_ns` for those seven rows that
  is the minimum/maximum **of perturbed windows**; `estimate_ns`/`avg_ns` (what `benchmarks.md`
  trends) is unaffected. Nothing is rewritten: the columns are supplementary and the rows are marked
  by their `floor_ns` (≈ 337 on this boot) and date.
- No source change. Widening `bench_batch` to clear the bar would change every row's window and is
  a **benchmark-shape change** (2.20.0: registering two rows shifted a third by 31%); the fix belongs
  in the instrument, and the proposed fix (resolve a row only when the *typical* window clears the
  bar; size `bench_run` chunks with margin) is in the upstream filing.

## Closed — the paired measurement (2026-09-30)

Both halves were measured from scratch dirs pinned to each version, with `cyrius build -v` naming
`versions/<v>/bin/cycc`. Each dir's vendored `lib/bench.cyr` byte-matches its cyrius TAG, and the
installed `versions/{6.6.6,6.6.9,6.6.12}/lib` slots match their tags 104/104.

| probe | 6.6.6 | 6.6.9 | 6.6.12 |
|---|---|---|---|
| upstream self-proving repro (`repros/2026-09-21-hisab-bench-min-above-mean-below-resolution-bar.cyr`) | exit **1**, "MIN ABOVE MEAN" | exit **0** | exit **0** |

On hisab's own suite, three binaries were run interleaved ×4 on a quiet box (max load 1.38).
**A** is the 6.6.6 compiler with the 6.6.6 `bench.cyr`, **B** the 6.6.12 compiler with the 6.6.6
`bench.cyr`, and **C** the 6.6.12 compiler with the 6.6.12 `bench.cyr`:

| binary | rows with `min > avg` |
|---|---|
| A | 13 / 320 |
| B | 8 / 320 |
| C | **0 / 320** |

The statistic hisab records did not move. C against B, the harness change alone, is median
**+0.00%** on `avg` over 80 rows, with 1 row past 10%. So the CSV's `regime` stays `net`, and the
`min_ns`/`max_ns` caveat in `benchmarks.md` applies only to rows written under 6.6.5–6.6.8.
Upstream says rows under the resolution bar now print `min = max = avg`, and the new harness prints
`E of W windows eligible` on each per-op line. `scripts/bench-history.sh` parses the row line, which
is unchanged.
