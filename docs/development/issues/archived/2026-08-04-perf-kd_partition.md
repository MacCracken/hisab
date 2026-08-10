> **CLOSED 2026-08-09 (2.9.3).** The repair SHIPPED — in 2.7.1, as a balance guard rather than this
> record's unconditional quickselect (the spec's variant measured +114% on `kdtree_build_4k`; the
> hybrid was chosen instead). "NOT APPLIED" in this header and in `doc-health.md` was wrong for three
> releases. What was genuinely open was the evidence around it, and 2.9.3 closed both halves:
>
> - **No regression assertion.** The two tests standing next to the guard — `kdtree_build != 0` and
>   the split-plane invariant — both hold on a degenerate one-point-per-level spine, so **deleting the
>   guard broke nothing that was being checked** (measured: the whole suite stays green with the
>   condition replaced by `if (1 == 1)`). Depth is the property the guard provides, so depth is now
>   asserted, on an **octave-spaced** fixture. Mutation-proven. ⚠ The pre-existing geometric fixture
>   does NOT exercise it — ratio 0.99998 gives an axis range of only [0.85, 1], so the value-midpoint
>   lands near the count-median and the guard never fires. Asserting on that fixture would have
>   re-created the same hole; that is recorded inline beside it.
> - **The BENCH section — now shipped.** `kdtree_build_octave_512`: **564.8 µs with the guard,
>   3.397 ms without, 6.0×.** This record asked for the degenerate class in 2.7.0 and it never landed,
>   so the guard was unmeasured on the only input class it exists for.
>
> ⚠ The 574 ms headline in this file **still has not reproduced** — CHANGELOG.md:1301-1307 already
> says so, and nothing here restates it as fact. The guard is justified by the depth bound it
> provably enforces and now by the 6.0× above, not by that figure.

<!-- 2.7.0-I performance record. Analysed and adversarially reviewed, both agents running
real code. Each optimisation is CONFIRMED as a real complexity win, but each was also shown
to CHANGE RESULTS or to regress on some input class -- see the CHALLENGE section before
applying. NOT APPLIED. The bicgstab hoist, the only result-safe one, shipped in 2.7.0-I. -->

# kd_partition
verdict=CONFIRMED

## MEASURED
All numbers from `cyrius bench` on this machine, cycc 6.5.6. Fixture = geometric cloud p_i = (2^-i, 2^-i, 2^-i), timing `kdtree_build`.

GROWTH RATIO (the evidence). Shipped `_kd_partition`, 3 iters:
  n=512    2.942 ms
  n=1024  11.210 ms   (3.81x per doubling)
  n=2048  34.642 ms   (3.09x)
  n=4096  79.633 ms   (2.30x)
  n=8192 173.627 ms   (2.18x)
  n=12500 273.028 ms
  n=25000 569.118 ms  (2.08x)
  n=50000  1.141 s    (2.01x)
Patched (value-midpoint fast path + 1/8 balance guard + exact-median fallback), same fixture:
  n=512   509.8 us
  n=1024   1.221 ms   (2.40x)
  n=2048   2.108 ms   (1.73x)
  n=4096   3.078 ms   (1.46x)
  n=8192   4.833 ms   (1.57x)
  n=12500  6.366 ms
  n=25000 11.564 ms   (1.82x)
  n=50000 22.331 ms   (1.93x)
Speedup: 5.8x at n=512, 35.9x at n=8192, 51.1x at n=50,000.

READ THE RATIOS CAREFULLY -- they say something the finding got slightly wrong. Shipped is 3.8x/3.1x per doubling up to n~2048 (genuinely QUADRATIC), then decays to 2.0x. That is not noise: with x_i = 2^-i the values underflow to 0 after i~1074, so the halving chain is exhausted at ~1075 levels. Cost is O(n * min(n, L)) with L = number of representable range-halvings (<= ~1075 per f64 axis). So: true n^2 while n <= L, then linear with a ~1000x constant. The predicted count 2*1075*50000 = 1.08e8 element visits at ~10 ns matches the measured 1.141 s.

The finding's own numbers (574 ms vs 38 ms at n=50,000) are the right order but not what I measured: I got 1.141 s vs 22.3 ms at n=50,000. 569 ms is what my fixture costs at n=25,000. Exact absolute values depend on the geometric ratio chosen; direction and magnitude are confirmed.

NO-REGRESSION, in-process A/B/C (same binary, same cache state, 20,000-point pseudo-scatter, 8 iters, three interleaved rounds):
  shipped                         13.007 / 12.857 / 12.940 ms  -> 12.93 ms
  balance guard only (perf fix)   12.986 / 12.903 / 12.939 ms  -> 12.94 ms  (+0.1%, free)
  guard + split_val hoist          13.698 / 13.471 / 13.683 ms  -> 13.62 ms  (+5.3%)
So the PERFORMANCE fix costs nothing on well-behaved data; the entire +5.3% is the separate correctness hoist described in result_risk.

WHY NOT PURE nth-element EVERY LEVEL (what the finding proposed): measured, it is a 2.2x REGRESSION on well-behaved input -- rnd_8192 5.080 ms shipped vs 10.999 ms pure-quickselect; rnd_2048 1.126 -> 2.435 ms. Hence the hybrid.

COINCIDENT (2.6.14 guard), n=20,000 all-identical: 5.535 ms shipped vs 5.535 ms patched -- the `f64_eq(lo, hi)` early return is untouched and still carries that case.

PROPOSED FIXTURE, measured end-to-end against a patched copy of the real module:
  kdtree_geometric_20k  463.478 ms -> 10.283 ms  (-97.8%)
  kdtree_scatter_20k     14.271 ms -> 14.359 ms  (+0.6%, inside noise)

Scratch files: /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/{kdpart.bcyr,kdpart2.bcyr,kdpart3.bcyr,kdab.bcyr,kdfixture.bcyr,kdcheck.tcyr,kdverify.tcyr,spatial_patched.cyr}. `cyrius lint` on the patched module: 0 warnings. `kdverify.tcyr` (real function names, patched copy of src/spatial.cyr) passes 12/12 including the two 2.6.14 assertions copied verbatim from tests/modules.tcyr.

## ROOT CAUSE
/home/macro/Repos/hisab/src/spatial.cyr:61-110, `_kd_partition`, called once per level from `_kd_build_rec` (line 122).

The function makes two full O(end-start) passes: the min/max scan at lines 66-72 and the partition scan at lines 94-105. Both are linear and fine. The defect is that the split threshold is `mid_val = (lo + hi) / 2` -- the midpoint of the VALUE RANGE, not of the POPULATION. It guarantees only that the point attaining `lo` goes left and the point attaining `hi` goes right; it guarantees nothing about the counts.

On geometrically spaced coordinates x_i = 2^-i: lo = 0 (or the smallest), hi = 1, mid = ~0.5, and only the single point at x=1 fails `v < mid_val`. Split = n-1 vs 1. The right child is a leaf; the left child recurses on n-1 elements whose range has merely halved. Next level peels one more. So the recursion is a chain, and the two linear passes are re-run on an almost-unshrunk range at every link:

  total work = 2 * sum_{k=0..L-1} (n - k),  L = number of halvings available

L is bounded by log2(range/min_gap), i.e. <= ~1075 for f64 on one axis (2^-1075 rounds to 0). Hence O(n^2) while n <= L and O(1075*n) after -- exactly the 3.8x-then-2.0x ratio curve measured above. The clamps at lines 107-108 (`if (split == start) { split = start + 1; }`) are the same one-per-level mechanism 2.6.14 fixed for the lo == hi case, except here they are not even the trigger: `split` legitimately lands at n-1, so no clamp fires and nothing flags the degeneracy.

The finding is right about the mechanism and the direction; it overstates the asymptotics (it is O(n * min(n, ~1075)), not unbounded O(n^2)) and its absolute numbers do not reproduce -- see `measured`.

## FIX
Replace lines 57-110 of /home/macro/Repos/hisab/src/spatial.cyr (the `--- Median-of-three partitioning ---` comment block through the end of `_kd_partition`). Everything else in the file is untouched -- `_kd_build_rec`'s `split_val = axis value of items[split]` convention is preserved, and the lo == hi positional fallback is verbatim.

# Axis value of the item sitting at position `i` of the index vec.
fn _kd_item_val(items, points, i, axis) {
    return _kd_axis_val(load64(points + vec_get(items, i) * 8), axis);
}

fn _kd_swap(items, a, b) {
    var tmp = vec_get(items, a);
    vec_set(items, a, vec_get(items, b));
    vec_set(items, b, tmp);
}

# --- Exact median selection (quickselect / nth-element) ---
# Reorders items[start..end) so that items[target] holds the median BY AXIS VALUE,
# every element before it compares <= it and every element after compares >= it.
# That is exactly the invariant _kd_nearest_rec's plane test needs, and it is
# distribution-free: O(n) expected per level instead of O(n * halvings).
#
# The middle band of the three-way (Dutch-flag) partition is load-bearing: a
# two-way split would push every element EQUAL to the pivot onto one side and
# reproduce the 2.6.14 one-element-per-level degeneracy on coincident input.
fn _kd_select_median(items, points, start, end, axis) {
    var target = start + (end - start) / 2;
    var lo = start;
    var hi = end;

    while (hi - lo > 1) {
        # Median-of-three pivot, so sorted and reverse-sorted ranges are best
        # case rather than worst case.
        var mid = lo + (hi - lo) / 2;
        var a = _kd_item_val(items, points, lo, axis);
        var b = _kd_item_val(items, points, mid, axis);
        var c = _kd_item_val(items, points, hi - 1, axis);
        var pivot = b;
        if (f64_lt(a, b) == 1) {
            if (f64_lt(b, c) == 1) { pivot = b; }
            elif (f64_lt(a, c) == 1) { pivot = c; }
            else { pivot = a; }
        } else {
            if (f64_lt(a, c) == 1) { pivot = a; }
            elif (f64_lt(b, c) == 1) { pivot = c; }
            else { pivot = b; }
        }

        # [lo, lt) < pivot | [lt, gt) == pivot | [gt, hi) > pivot.
        # NaN compares false both ways and lands in the equal band, so a NaN
        # coordinate cannot spin this loop.
        var lt = lo;
        var eq = lo;
        var gt = hi;
        while (eq < gt) {
            var v = _kd_item_val(items, points, eq, axis);
            if (f64_lt(v, pivot) == 1) {
                _kd_swap(items, lt, eq);
                lt = lt + 1;
                eq = eq + 1;
            } elif (f64_gt(v, pivot) == 1) {
                gt = gt - 1;
                _kd_swap(items, eq, gt);
            } else {
                eq = eq + 1;
            }
        }

        # The equal band is never empty (the pivot is a real element), so both
        # branches strictly shrink [lo, hi) and the loop always terminates.
        if (target < lt) { hi = lt; }
        elif (target < gt) { lo = target; hi = target + 1; }
        else { lo = gt; }
    }

    return target;
}

# --- Partitioning ---
# Partition items[start..end) on the given axis.
# items: vec of point indices into `points`.
# Returns the split position: [start, split) is the left child, [split, end) the
# right, and the caller reads split_val from items[split].
fn _kd_partition(items, points, start, end, axis) {
    # Simple selection: find item closest to midpoint on axis
    # First pass: find min/max on axis
    var lo = _SP_F64_POS_INF;
    var hi = f64_neg(_SP_F64_POS_INF);
    for (var pi = start; pi < end; pi = pi + 1) {
        var idx = vec_get(items, pi);
        var pt = load64(points + idx * 8);
        var v = _kd_axis_val(pt, axis);
        if (f64_lt(v, lo) == 1) { lo = v; }
        if (f64_gt(v, hi) == 1) { hi = v; }
    }
    # Degenerate range: every point shares one axis value.
    #
    # Splitting positionally here is safe precisely because the values are equal:
    # split_val (read by the caller from items[split]) is then the common value, so
    # query pruning compares exactly and either subtree is a valid destination.
    # This is the 2.6.14 fix for _kd_build_rec SEGFAULTing on ~60k coincident
    # points; it stays exactly as it was.
    if (f64_eq(lo, hi) == 1) {
        return start + (end - start) / 2;
    }

    var mid_val = f64_mul(f64_add(lo, hi), F64_HALF);

    # Partition: items with val < mid_val go to front. min_r_pos tracks where the
    # SMALLEST right-hand value currently lives; the only way that element moves
    # is the swap below, which relocates whatever sits at `split` to `qi`.
    var split = start;
    var min_r_pos = 0 - 1;
    var min_r_val = 0;
    for (var qi = start; qi < end; qi = qi + 1) {
        var idx2 = vec_get(items, qi);
        var pt2 = load64(points + idx2 * 8);
        var v2 = _kd_axis_val(pt2, axis);
        if (f64_lt(v2, mid_val) == 1) {
            if (min_r_pos == split) { min_r_pos = qi; }
            # Swap items[qi] and items[split]
            var tmp = vec_get(items, split);
            vec_set(items, split, vec_get(items, qi));
            vec_set(items, qi, tmp);
            split = split + 1;
        } elif (min_r_pos < 0 || f64_lt(v2, min_r_val) == 1) {
            min_r_val = v2;
            min_r_pos = qi;
        }
    }

    # BALANCE GUARD -> EXACT MEDIAN.
    #
    # The value midpoint separates lo from hi, but it says nothing about how many
    # points land on each side. On geometrically spaced coordinates (x_i = 2^-i)
    # it peels exactly ONE point per level: the range halves, the population does
    # not. Depth then tracks log2(range / min_gap) -- up to ~1075 levels for f64,
    # each re-scanning an almost-unshrunk range twice. Measured on the shipped
    # code, geometric input: 512 -> 2.94 ms, 1024 -> 11.21 ms, 2048 -> 34.64 ms
    # (3.8x then 3.1x per doubling -- quadratic), flattening to O(1075 * n) past
    # n ~ 1024, where 50,000 points cost 1.141 s. Falling back to the exact median
    # whenever the value split is worse than 1/8 vs 7/8 bounds the depth at
    # log(8/7)(n): the same 50,000-point build is 22.3 ms (51x).
    # The 1/8 slack is what keeps well-behaved clouds on the two-pass fast path --
    # a 20,000-point scatter measured 12.93 ms before and 12.94 ms after, while
    # selecting the median at EVERY level costs 2.2x there (8,192 points:
    # 5.08 ms -> 11.00 ms).
    var count = end - start;
    var guard = count / 8;
    if (guard < 1) { guard = 1; }
    if (split - start < guard || end - split < guard) {
        return _kd_select_median(items, points, start, end, axis);
    }

    # Put the SMALLEST right-hand value at items[split]. The caller publishes
    # items[split]'s axis value as the node's splitting plane, and _kd_nearest_rec
    # skips the far subtree when d*d >= best -- sound only if every point over
    # there is at least split_val away on this axis. Any other right-hand element
    # leaves points BETWEEN mid_val and split_val on the far side, closer to the
    # query than the plane claims. Costs 5.3% on the fast path (20,000-point
    # scatter 12.94 ms -> 13.62 ms) and buys back an exact nearest-neighbour.
    if (min_r_pos > split) { _kd_swap(items, split, min_r_pos); }

    # No clamp needed: guard >= 1 already routed every empty-side case to
    # _kd_select_median, which always returns start < target < end for count >= 2.
    return split;
}

## GAIN
Geometric / heavy-tailed coordinate distributions, `kdtree_build`: 45x at n=20,000 (463.478 ms -> 10.283 ms, -97.8%) and 51x at n=50,000 (1.141 s -> 22.331 ms, -98.0%). Growth per doubling drops from 3.8x/3.1x (quadratic regime) to 1.5-1.9x (n log n). Well-behaved point clouds: unchanged, +0.1% for the performance change alone; +5.3% if the split_val correctness hoist ships with it (20,000-point scatter, 12.93 -> 12.94 -> 13.62 ms). All-coincident input: unchanged (5.535 -> 5.535 ms). Bonus that is not a timing: nearest-neighbour queries stop returning non-nearest points (237/400 wrong -> 0/400 on a 1,500-point scatter).

## RESULT RISK (author)
NOT "none". Two distinct effects, one of them large -- read both.

1. THE SHIPPED TREE IS ALREADY WRONG, AND THIS PATCH CHANGES ITS ANSWERS. While verifying I found that the shipped `split_val` is not a sound pruning plane. `_kd_partition` puts {v < mid_val} left and {v >= mid_val} right, but `_kd_build_rec` reads `split_val` from `items[split]` -- an ARBITRARY right-hand element, whose value can exceed the right side's minimum. `_kd_nearest_rec` then skips the far subtree when `d*d >= best`, which is only valid if everything over there is >= split_val away. Reproduced by hand and in code, 3 points, no tree-shape subtlety:
   points (0,0,0), (100,0,0), (51,0,0); query (49,0,0)
   brute force -> index 2 (distance 2). Shipped kdtree_nearest -> index 0 (distance 49).
Over a 1,500-point pseudo-scatter, the shipped tree returned a NON-nearest point on 237 of 400 queries; on a 600-point geometric cloud, 179 of 600. The patch returns 0 of 400 and 0 of 600 wrong (harness: scratchpad/kdcheck.tcyr, kdverify.tcyr). The `min_r_pos` hoist is what fixes it; it is +5.3% on the fast path (measured, isolated). If you want the pure performance change with byte-identical query answers, drop the `min_r_pos`/`min_r_val` lines and the hoist -- that variant measured 12.94 ms vs 12.93 ms shipped on 20,000 scatter points, i.e. free -- but you would be shipping a knowingly unsound plane. My recommendation is to keep the hoist and land it as a Fixed entry, not just Performance.

2. TIE-BREAKING AND TREE SHAPE. Balanced-input trees are unchanged in shape whenever the value split is inside 1/8..7/8 and the min-of-right element already sat at `split`. Otherwise the tree shape differs, so when two points are EQUIDISTANT from a query, which index `kdtree_nearest` returns can change. Distances are identical; the index is not guaranteed stable. `kdtree_within_radius` is set-valued and therefore unaffected in content (verified equal to brute force count); its output ORDER can change.

Degenerate cases I actually checked (all pass, scratchpad/kdverify.tcyr, 12/12 against the patched real module):
  - all-coincident n=60,000: builds, no stack overflow (2.6.14 guard, verbatim from tests/modules.tcyr). The `f64_eq(lo, hi)` early return is byte-identical and still the path taken; timing unchanged (5.535 ms vs 5.535 ms at n=20,000).
  - half-coincident n=4,000 + exact nearest hit: passes (the other 2.6.14 guard).
  - n=0 -> null tree; n=1; n=2; empty ranges cannot occur (guard >= 1 routes split == start and split == end to the median selector, which returns start < target < end for count >= 2, so the old clamps are provably dead and were removed).
  - collinear, quadratically spaced (500 points on the x axis, y=z=0), 200 queries: exact.
  - sorted-ascending diagonal (3,000 points), 300 queries: exact -- median-of-three makes sorted input best case, not worst.
  - NaN coordinates: `f64_lt` and `f64_gt` are both false for NaN, so a NaN lands in the equal band and cannot spin the selection loop. Ordering with NaN is meaningless either way, but the shipped code was worse here: all-NaN makes mid_val NaN, no element compares <, split == start, the old clamp peels one per level -> O(n^2) and O(n) recursion depth. Not separately timed.

Residual risk I did NOT eliminate: `_kd_select_median` is median-of-three quickselect, so it is O(n) EXPECTED, not worst-case. A deliberately constructed median-of-three killer permutation could push one level back to O(n^2). The three-way band means duplicates cannot do it, and the fallback only runs on already-lopsided ranges, but if you want a hard bound the drop-in is a ninther pivot for ranges >= 128 (BSD qsort style, ~15 lines) or full median-of-medians (hard O(n), ~2-3x the constant). I did not measure either.

## BENCH
Add to /home/macro/Repos/hisab/tests/hisab.bcyr. NOTE: src/spatial.cyr is NOT currently in that file's include list -- add it after `include "src/geo_advanced.cyr"`:

include "src/spatial.cyr"

Fixture globals, alongside the existing `_b_bvh_boxes` / `_b_bvh_n` block near line 55:

var _b_kd_geo = 0;
var _b_kd_rnd = 0;
var _b_kd_n = 0;

Bench fns, next to bench_bvh_degenerate:

# Worst-case k-d input: geometrically spaced coordinates. The VALUE midpoint
# lands between the largest point and everything else, so the split is 1-vs-(n-1)
# and the two linear passes re-run on an almost-unshrunk range for as many levels
# as f64 has halvings (~1075). Pre-fix this cost 463 ms; the balance guard that
# falls back to an exact median makes it 10 ms.
fn bench_kdtree_geometric() {
    var t = kdtree_build(_b_kd_geo, _b_kd_n);
    return 0;
}

# Same size, well-behaved scatter: the guard must NOT push this off the two-pass
# fast path. Selecting the median at every level instead would be a 2.2x
# regression here, which is why the value-midpoint split is kept.
fn bench_kdtree_scatter() {
    var t = kdtree_build(_b_kd_rnd, _b_kd_n);
    return 0;
}

Setup, in main() after the BVH fixture:

    # 20,000 points, two distributions from one loop. Sized to the bump arena:
    # kdtree_build allocates 2n-1 40-byte nodes per call and never frees, so
    # 3 iters x 2 fixtures is ~4.8 MB.
    _b_kd_n = 20000;
    _b_kd_geo = alloc(_b_kd_n * 8);
    _b_kd_rnd = alloc(_b_kd_n * 8);
    var kv = F64_ONE;
    var ki = 0;
    while (ki < _b_kd_n) {
        store64(_b_kd_geo + ki * 8, hvec3_new(kv, kv, kv));
        kv = f64_mul(kv, F64_HALF);
        var kx = (ki * 2654435761) % 10007;
        var ky = (ki * 40503) % 10009;
        var kz = (ki * 22695477) % 10037;
        store64(_b_kd_rnd + ki * 8, hvec3_new(f64_from(kx), f64_from(ky), f64_from(kz)));
        ki = ki + 1;
    }

Registration, next to the bvh_degenerate_4k line:

    # 3 iterations, not 20: pre-fix a single geometric build was ~460 ms.
    bench("kdtree_geometric_20k", &bench_kdtree_geometric, 3);
    bench("kdtree_scatter_20k",   &bench_kdtree_scatter,   3);

Measured with this exact fixture (patched copy of src/spatial.cyr vs shipped):
  kdtree_geometric_20k  463.478 ms -> 10.283 ms  (-97.8%)
  kdtree_scatter_20k     14.271 ms -> 14.359 ms  (+0.6%)
Total added CI cost after the fix: ~74 ms.

## CHALLENGE changes_results=True compiles=True repro=True stands=True
- UNDER-REPORTED RESULT CHANGE: kdtree_within_radius. The reviewer wrote that it is 'set-valued and therefore unaffected in content'. That is wrong for the SHIPPED code and understates the patch. On a 1,500-point scatter with 400 radius-60 queries I measured shipped: 222/400 queries return the WRONG COUNT, 63 total hits returned where brute force expects 380, bad_member=0 (i.e. everything returned is valid, so the shipped result is a silent SUBSET -- it drops ~83% of the in-radius points). Patched: count_mismatch=0, total_hits=380, exact. Randomized sweep (1,200 queries): shipped 234 count mismatches, patched 0. Same root cause as nearest (_kd_radius_rec prunes the far subtree on the same unsound split_val). This is a consumer-visible behaviour change for impetus/kiran/joshua/aethersafha and needs its own Fixed entry, not a footnote about output ORDER.
- REGRESSION CLAIM IS UNDER-SCOPED. 'The PERFORMANCE fix costs nothing on well-behaved data' was established on ONE 20,000-point uniform scatter. On duplicate-heavy skewed distributions the balance guard fires and the cost is real: kd_skew16_50k 24.509 -> 31.365 ms (+28%), kd_two_cluster_50k 22.502 -> 29.238 ms (+30%), both n=50,000, 3 iters. I isolated it with their own guard-only build (spatial_nohoist.cyr): 31.486 ms and 29.447 ms -- i.e. the entire +28/+30% is the GUARD (median fallback firing), not the hoist. Uniform scatter total cost is +3-5% (scatter_2048 1.092->1.150, scatter_8192 4.986->5.243, scatter_20000 13.519->13.966). The trade is still clearly worth it (7 ms worst observed cost vs 1.11 s saved on geometric), but 'free on well-behaved data' should not go in the CHANGELOG unqualified.
- HOIST-COST DECOMPOSITION DOES NOT REPRODUCE CLEANLY. They report guard-only +0.1% and hoist +5.3%. Across separate binaries I get guard-only already +0.7%/+2.9%/+2.7% at 2048/8192/20000 scatter, with the hoist adding the rest to +5.3%/+5.2%/+3.3%. Their interleaved in-process A/B/C is the better instrument for that split, so I am not disputing the number -- but the published claim should be the TOTAL (+3-5% on uniform scatter), since guard and hoist are landing together.
- ZERO PERMANENT REGRESSION COVERAGE. All four suites (modules 692, hisab 306, foundation 310, edge_cases 215) pass identically under shipped AND patched. The 2.6.14 kdtree assertions in tests/modules.tcyr:2421-2450 do not detect a 676-violation tree. The pruning-plane invariant must land as a permanent assertion, not only as scratchpad/kdverify.tcyr. The cheapest high-value test is the structural one, not query sampling: walk the built tree and assert max(axis val in left subtree) <= split_val <= min(axis val in right subtree) at every split node -- it catches 676/1999 bad nodes on a plain 2,000-point scatter, where random query sampling only catches ~58% of queries.
- BENCH FIXTURE CANNOT BE ADDED AS PROPOSED. tests/hisab.bcyr does not include src/spatial.cyr at all (its include list stops at geo_advanced.cyr) and contains no spatial benchmarks. The proposed kdtree_geometric_20k / kdtree_scatter_20k fixtures require adding that include line first.
- DISTLIB GATE WILL FAIL WITHOUT REGEN. dist/hisab.cyr embeds _kd_partition (line 4788). I ran `cyrius distlib` on the patched tree: it succeeds and the bundle grows 15,747 -> 15,851 non-blank lines. CI's distlib drift gate fails unless dist/hisab.cyr is regenerated in the same commit.
- SOURCE-COMMENT DRIFT. The comment baked into spatial_patched.cyr cites 'a 16,384-point scatter measured 10.65 ms before and 10.97 ms after', while the proposal text for the same comment cites 'a 20,000-point scatter 12.93 ms before and 12.94 ms after'. Two different fixtures for the same claim. Given this repo's history with numbers baked into source comments (the 2.6.12 constant-table arc), pick one fixture, make it the one in bench-history.csv, and cite that.
- RESIDUAL RISK CONFIRMED, NOT ELIMINATED. _kd_select_median is median-of-three quickselect: O(n) EXPECTED, not worst-case, exactly as disclosed. I could not construct a killer within budget, and every pathological distribution I threw at it stayed flat at n=50,000 (reverse-sorted 32.5 ms, sorted-ascending 31.8, organ-pipe 32.4, sawtooth, 1/16-skew 31.4, two-cluster 29.2, geometric 22.6, coincident 5.6, mixed NaN+inf and all-NaN at n=5,000 both build). The three-way band does provably block duplicate-driven degeneracy. But the hard bound is still not there -- if a consumer feeds attacker-controlled coordinates, take their ninther suggestion.

### CORRECTED
No correction to the CODE is needed -- I verified the patch line by line and by construction, and it holds up:

1. `_kd_select_median` termination is provable, not just empirical. The pivot is always an element of [lo,hi), so it lands in the equal band, so lt < gt strictly. `target < lt -> hi = lt` shrinks because lt < gt <= hi; `target >= gt -> lo = gt` shrinks because gt > lt >= lo; the middle branch collapses to a single slot. NaN can only widen the equal band (both f64_lt and f64_gt are false), never empty it, so NaN cannot spin the loop -- confirmed empirically with all-NaN n=5,000 and mixed NaN/inf n=5,000.
2. The `min_r_pos` tracking is correct. Right-hand elements occupy exactly [split, qi), so min_r_pos is always >= split or -1; the ONLY element the Lomuto swap relocates is the one at `split`, and `if (min_r_pos == split) { min_r_pos = qi; }` fires before the swap, which is the right order. min_r_val needs no update because it is the same element. min_r_pos == -1 at the hoist is impossible: the guard already routed end-split == 0 to the median selector.
3. Removing the two clamps is provably safe, and the clamps were themselves a correctness bug. Every path returns start < split < end for count >= 2 (degenerate path: start + count/2; median path: same; fast path: guarded by guard >= 1 on both sides), and count <= 1 never reaches _kd_partition because _kd_build_rec returns a leaf at count == 1. Note the old `if (split == end) { split = end - 1; }` clamp is what produces the LEFT-side violation I measured on geom2000 (left=1) -- it drags a left-partition element into the right partition, making split_val smaller than left's max.
4. Depth is bounded. The 1/8 guard gives log_{8/7}(n) + O(1); measured max depth on 2,000 geometric points drops from 548 (shipped) to 21 (patched). That also shrinks _kd_nearest_rec's per-query recursion depth by the same factor, which the write-up never claims credit for.

What I would CHANGE about the landing, not the code:

a) Land it as Fixed + Performance with TWO Fixed entries: kdtree_nearest returning non-nearest points, AND kdtree_within_radius silently returning a subset (63 of 380 expected hits on my fixture). Flag the radius change for consumers explicitly -- a consumer that tuned a radius against the broken behaviour will suddenly get ~6x more hits.
b) Add the structural invariant test to tests/modules.tcyr (see problems[3]) -- it is a ~25-line recursive walk and it is a far stronger gate than query sampling.
c) Add `include "src/spatial.cyr"` to tests/hisab.bcyr before adding the two bench fixtures.
d) Regenerate dist/hisab.cyr in the same commit.
e) Qualify the perf claim: 50.2x on the geometric fixture at n=50,000 (1.136 s -> 22.630 ms, my numbers), 35.6x at n=8,192, 5.2x at n=512; cost +3-5% on uniform scatter and up to +30% on duplicate-heavy skewed clouds (kd_skew16_50k 24.5 -> 31.4 ms).
f) Optional hardening, unmeasured: ninther pivot for ranges >= 128 in _kd_select_median if the worst-case bound matters to a consumer.