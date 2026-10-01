# 2026-09-09 — 41 of 72 benchmarks report a `net` value SMALLER than the timer floor they subtract

**Component:** `tests/hisab.bcyr` + `scripts/bench-history.sh` (the `regime=net` instrument).
**Severity:** Measurement — no wrong code, but **41 rows of `benchmarks.md` cannot support a
performance claim** and one of them moved −56% this release on nothing.
**Status:** 🔴 **Refuted and archived 2026-09-09 (v2.11.4)**: see the section of that name below.
No work is owed. *Original status line, kept for the record:* "Open. Filed rather than fixed in
2.11.3; scheduled on the roadmap." The 2.11.3 roadmap row ("Give the benchmark harness
resolution, not just an unbiased floor") was struck as refuted in 2.11.4. As of 2026-09-30 the
roadmap's only mention of this file was the [3.3.2] row (audit D135) that corrected this header.

## The finding

Since 6.5.19 the harness measures the `clock_gettime` floor and subtracts it from every sample, and
2.11.2 recorded that as a `regime` column. `net = raw − floor` is the right correction, but nothing
checks whether `net` is **large enough relative to `floor` to mean anything**.

Measured on this host at floor = **1329 ns**:

| benchmark          | net (ns) | net / floor |
|--------------------|---------:|------------:|
| `ease_in_out`      |        7 |    **0.01x** |
| `vec3_add`         |       17 |      0.01x |
| `tonemap_reinhard` |       22 |      0.02x |
| `cx_mul`           |       23 |      0.02x |
| `vec3_cross`       |       28 |      0.02x |
| `quat_mul`         |       38 |      0.03x |
| `ray_aabb`         |       41 |      0.03x |

- **41 of 72** benchmarks have `net` **below** the floor.
- **48 of 72** have `net` below **10x** the floor.
- [measured: bench-history.csv, run 2026-09-09T16:22:39, commit 1bc71e3]

## Why it matters

`net` is a difference of two numbers that are both ~1330–1370 ns. 2.11.2 measured this harness's own
noise on `avg` at a **median 3.5% spread, worst 6.9%**. A 3.5% drift on a 1370 ns raw sample is
**48 ns** — larger than the *entire* signal of 41 of these benchmarks. So their run-to-run movement
is dominated by host noise, and a regression inside one of them is invisible.

⚠ **This release is a live demonstration.** `ray_aabb` reads −55.9% and `vec3_cross` −55.6% between
2.11.2 and 2.11.3. Both are in the sub-floor tier, so neither number is evidence of anything. The
speedups this release *does* claim are drawn only from the 23 benchmarks whose `net` is ≥ 10x the
floor, where the same comparison is trustworthy — and the flat rows in that tier
(`svd_golub_kahan_12` +0.1%, `eigen_qr_12` +0.4%, `num_dct_1023` +1.0%) are the control that shows
the instrument itself did not shift.

## Same class as the 2.10.0 finding

2.10.0 found *"17 of 60 benchmarks measuring `clock_gettime` rather than the operation"* and moved
those to `bench_batch()`. This is the surviving remainder of that class: the floor is now
*subtracted* rather than *included*, which fixed the bias but not the **resolution**. Subtraction
cannot recover a signal that is 1% of the sample.

## Suggested fix

Amplify inside the timed region — loop the operation N times per sample so `net` lands at
≥ 10x floor — and have `bench-history.sh` **refuse to write a trend row** whose `net/floor` is below
a threshold, the way it already refuses rows that are not `stat=avg regime=net`. A gate that
silently reports a meaningless number is the shape this repo has been bitten by three times.

---

## 🔴 REFUTED AND ARCHIVED 2026-09-09 (v2.11.4) — the instrument was mine, not the harness's

**This filing is wrong, and the way it is wrong is the exact failure it accused the harness of.**

It compared each benchmark's **per-operation** `net` against `floor_ns`, which is the cost of **one
clock pair** — two quantities that are not comparable. Since 6.5.19 `bench_run` does not wrap a clock
pair around each call; `_bench_chunk_for(per, fl)` sizes every batch as `want = (fl * 100) / per`,
i.e. **so the clock's own error is 1% of the timed window**. A 7 ns operation therefore runs ~19,000
times inside one window. `net` being 0.01x the floor is that design working, not a signal drowning.

Tested rather than argued, on the property the filing actually cared about — reproducibility. Two
back-to-back runs of the **identical** binary, all 72 benchmarks, split by the very ratio this filing
used to condemn them:

| tier | n | median spread | p90 | worst |
|---|---:|---:|---:|---:|
| `net` >= 10x floor — *"trustworthy"* | 23 | 2.10% | 5.95% | 9.13% |
| `net` < 10x floor — *"cannot support a claim"* | 49 | **1.43%** | **4.74%** | **7.32%** |

**The tier this filing dismissed is the quieter of the two.** There was never a resolution problem.

### ⚠ What the bad instrument cost

2.11.3 used this filing to suppress its own result, saying `ray_aabb` −55.9% and `vec3_cross` −55.6%
were "not evidence of anything". Against measured same-binary noise they are 15–25x the noise and
entirely real:

| benchmark | 2.11.2 → 2.11.4 | same-binary noise |
|---|---:|---:|
| `ray_aabb` | **−54.8%** | 2.33% |
| `vec3_cross` | **−54.0%** | 3.33% |
| `ray_triangle` | **−44.3%** | 2.84% |
| `jet_plane` | **−40.5%** | 0.98% |
| `quat_mul` | **−39.7%** | 2.56% |
| `ray_sphere` | **−36.7%** | 1.72% |

So cycc 6.5.71's accessor inlining was a far broader win than 2.11.3 claimed, and the claim was
narrowed by a ratio that measures nothing. **Being conservative is not the same as being correct** —
a wrong instrument suppresses real findings as readily as it invents false ones, and this one did
exactly that for a whole release.

⭐ **The right guard, if one is ever wanted, is the one used here**: re-run the identical binary and
measure the spread. That needs no threshold, no ratio and no assumption about what the harness is
doing internally. Recorded rather than deleted so the reasoning is not repeated.
[measured: bench-history.csv, runs 2026-09-09; two back-to-back runs of the same binary]
