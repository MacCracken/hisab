# `num_dst` / `num_idst` allocate on every call, so a per-step pressure solve leaks — closed in 3.4.2

**Status:** ✅ **CLOSED in 3.4.2** (2026-11-14). Opened on hisab 3.4.1 with cyrius 6.7.18 (x86_64), 2026-11-11.
**Discovered:** 2026-11-11, by pravash's Rust → Cyrius port (M0 step 9, probes 1 and 8).
**Reporter:** pravash 2.0.0 port (conversion ledger L6; `pravash/docs/development/pravash-port-plan.md` §8).
**Related:** H9 (`2026-11-09-allocator-scoped-calls.md`) and cyrius S1, which would also contain it.
**Severity:** **Medium–high for grid fluid solvers.** Nothing fails; memory grows every step.

## Summary

A grid solver's DST pressure solve (pravash's `FluidGrid` with no-slip walls) transforms every interior row and
column, forward and back, every step. Each `num_dst` / `num_idst` call allocates its working set:

| n (interior size) | Global heap per call, never returned |
|---|---|
| 14 (a 16² grid) | 112 B |
| 62 (64²) | 10,208 B |
| 126 (128²) | 20,448 B |

A 64² solve makes about 248 such calls: **≈ 2.5 MB a step**.

## Reproduction

[`2026-11-11-dst-allocates-per-call-repro.cyr`](2026-11-11-dst-allocates-per-call-repro.cyr), from the repo root:

```sh
cyrius build docs/development/issues/archived/2026-11-11-dst-allocates-per-call-repro.cyr /tmp/dst
/tmp/dst; echo "exit=$?"
# n = 14: 112 B ... n = 62: 10208 B ... n = 126: 20448 B of global heap per num_dst / num_idst call
# exit=1
```

## Ask

A transform into **caller-provided scratch**, for example `num_dst_into(data, n, scratch, scratch_len)`, with the
scratch size a function of n that the caller can query (`num_dst_scratch_words(n)`). The result stays in place, as
now.

## Notes from the consumer (not defects)

- **The bits differ from hisab 1.4.0's on purpose.** 1.4.0's `dst` was the direct O(N²) sum, with
  `sin(π (i + 1)(k + 1) / (n + 1))` per term. 3.4.1 switches to an FFT past small n. pravash's probe 8 compared them
  through its frozen oracle: 20 of 36 small-n vectors have the same bits; none of 12 at n = 30–126 do (up to
  ≈ 67,000 ulp, on near-zero outputs). pravash needs 1.4.0's bits for parity, so it transcribes the direct form as a
  marked stand-in. It would convert to hisab's only with a re-bounded parity class for the DST rows.
- If hisab ever offers the direct form as an option (`num_dst_direct`), in 1.4.0's operation order and with a
  correctly rounded sin, pravash could convert with no parity change. That is a nice-to-have, not part of this
  request.

## pravash: the direct form wants a caller-kept plan too (2026-11-12, M3 part b)

pravash's stand-in no longer calls sin inside the sum. 1.4.0's term `sin(π (i + 1)(k + 1) / (n + 1))` depends only
on (i, k, n), so the grid keeps each transform length's n² sines in a table it owns (its "plan") and reuses it
across rows, columns and steps. The values are the same, so the bits are 1.4.0's. With correctly rounded sines the
calls were the whole cost: a 2D solve makes 4n³ of them, about 165 ns each, and about 18 µs each at the arguments
that land on fl(k·π/2) (ganita's correctly-rounded-math request has those numbers).

A `FluidGrid::step` on the DST projection, pinned, A/B interleaved, n × n grid (transform length n − 2):

| n | sin in the sum | plan, first step (builds it) | plan, later steps | Rust 1.3.0 (glibc sin) |
|---|---|---|---|---|
| 15 | 28.5 ms | 760 µs | 190 µs | 127 µs |
| 30 | 15.9 ms | 1.10 ms | 965 µs | 1.07 ms |
| 63 | 719 ms | 8.6 ms | 5.64 ms | 10.1 ms |

**Ask, extended:** let the into-scratch form take a plan the caller keeps, for example `num_dst_plan(n, plan)`
(`num_dst_plan_words(n)` words: the sines for the direct form, or the FFT's twiddles) and
`num_dst_into(data, n, plan, scratch)`. pravash would convert its plan to hisab's under the same ledger row, L6.

## Closed in 3.4.2

**The repair is the extended ask: a plan the caller keeps, and scratch the caller provides.** It is in `src/num_ext.cyr`:

| Function | What it does |
|---|---|
| `num_dst_plan_words(n)` | The plan's size in words, as `Ok(words)` |
| `num_dst_scratch_words(n)` | The scratch's size in words, as `Ok(words)` |
| `num_dst_plan(n, plan)` | Fills the plan once per transform length |
| `num_dst_into(data, n, plan, scratch)` | The transform, in place; allocates nothing |
| `num_idst_into(data, n, plan, scratch)` | The inverse, in place; allocates nothing |

The plan holds what `num_dst` computes afresh on every call that does not depend on the data:
- the direct path's n² sines;
- the power-of-two FFT's twiddles;
- Bluestein's chirp, the transform of its kernel, and the FFT's twiddles.

**The bits are `num_dst`'s and `num_idst`'s on every path.** The plan takes the same path `num_dst` takes and holds the
values `num_dst` computes. Its FFT is `num_fft`'s butterflies with the twiddles read from the table. The tests pin it at
every n = 1..300, at 511, 1023, 1500 and 2047, and in both directions.

**Not added:** `num_dst_direct` in 1.4.0's operation order, the issue's nice-to-have. The plan's direct path is
`num_dst`'s own direct sum: its sines are `sin(PI ((i + 1)(k + 1)) / (n + 1))`, the same values pravash's stand-in
tables.

**Paired run, 2026-11-14 (cyrius 6.7.18):**
- `2026-11-11-dst-allocates-per-call-repro.cyr` (moved beside this file) exits **1** on 3.4.1 and on 3.4.2 as filed:
  112, 10,208 and 20,448 B per `num_dst` / `num_idst` call, unchanged.
- `…-repro-3.4.2.cyr`, the same loop on `num_dst_into` / `num_idst_into`, exits **0**: 0 B per call at n = 14, 62 and
  126.
- `tests/modules_c.tcyr`, group "3.4.2: num_dst_into / num_idst_into", pins the bits and the 0 B.
