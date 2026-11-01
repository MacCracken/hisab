# Benchmarks

> **Measurement changes** — read before comparing across a date.
> * **2026-09-30** (hisab 3.2.2, cyrius 6.6.6 → 6.6.12): `lib/bench.cyr` 6.6.9 decides
>   `min`/`max` for the ROW by op count rather than per window, which ends the caveat in
>   the next entry: rows under the resolution bar now print `min = max = avg`, and the
>   per-op line reports `E of W windows eligible`. **The `avg` this table trends did not
>   move**: three binaries interleaved ×4 on a quiet box (6.6.6 compiler + 6.6.6 harness /
>   6.6.12 compiler + 6.6.6 harness / 6.6.12 + 6.6.12) put the harness half at median
>   **+0.00%**, and the whole bump at median +0.25% over 80 rows. Rows with `min > avg`
>   went 13/320 → **0/320**. Two rows moved for real, and the cause is not the
>   instrument: `srgb_to_linear` **+96%** (ganita 1.2.8's fdlibm `pow` — within 1 ulp,
>   ~2× the cost) and `quat_slerp` **−29%** (6.6.9's software `sin`/`cos`, faster than
>   x87 `fsin` at these arguments). `regime` stays `net`. ⚠ The 2026-09-30 rows were
>   taken on **hpet**: the kernel marked the TSC unstable at boot (frequency skew), so
>   `floor_ns` is ≈ 1,328 against ≈ 337 for the 2026-09-22 rows. The same-boot A/B/C
>   above is the comparison; the trend columns straddle two clocksources.
> * **2026-09-21** (hisab 3.2.1, cyrius 6.6.4 → 6.6.6): `lib/bench.cyr` 6.6.5 rewrote
>   16 of its 25 functions — `min`/`max` only from windows that clear a resolution bar
>   (100 × clock read + tick), the raw total netted at read time instead of per-window
>   clamps, the floor re-calibrated at report, ns rounded half-up instead of truncated.
>   **The `avg` this table trends did not move**: three binaries interleaved ×4 on a quiet
>   box (6.6.4 compiler + 6.6.4 harness / 6.6.6 compiler + 6.6.4 harness / 6.6.6 + 6.6.6)
>   put the instrument half at median **+0.00%** (mean +0.31%) and the compiler half at
>   median −0.62% over 80 rows, 1 row past 10% in each and that row (`cx_mul`) inside its
>   own 14.5% same-binary spread — so `regime` stays `net` on a measurement, not on the
>   floor line's presence. ⚠ `min_ns`/`max_ns` are NOT comparable across this date for
>   the seven sub-40 ns rows whose fixed 2000-op window sits under the bar (`vec3_add`,
>   `vec3_cross`, `vec3_normalize`, `vec3_lerp`, `vec2_lerp`, `tonemap_reinhard`,
>   `num_gcd`): under 6.6.5+ only a perturbed window resolves there, so the printed `min`
>   is the minimum OF THE SLOW WINDOWS and sits above `avg` (`vec3_add: 16ns avg
>   (min=39ns …)`). Filed upstream with a self-proving repro
>   (`docs/development/issues/archived/2026-09-21-cyrius-bench-min-above-mean-*.md`).
>   ⚠ Separately, the rows dated 2026-09-14 and earlier were taken on a previous BOOT
>   (kernel 7.2.3, `floor_ns` ≈ 1,340); the box rebooted 2026-09-18 (7.2.6, tsc,
>   `floor_ns` ≈ 337) and the geometry/collision family reads 15–37% slower while the
>   dense kernels read 7–11% faster on the SAME 6.6.4 binary — host state, not code.
>   `floor_ns` is the marker; compare only within one boot.
> * **2026-08-21** (hisab 2.11.2, cyrius 6.5.18 → 6.5.33): `lib/bench.cyr` now
>   MEASURES one clock read on the host and subtracts it from every sample, and
>   `bench_run` sizes its own batches instead of wrapping a clock pair around every
>   iteration. **No hisab source changed.** The floor is re-measured every run and
>   recorded per row in `floor_ns` (~1,340–1,350 ns on this host; upstream measured a
>   230x spread across its four gate hosts, and it moves between reboots, so it is
>   not a constant and is not written down as one).
>   `ease_in_out` 1,407 ns → 7 ns, `cx_mul` 1,459 → 37, `quat_mul` 1,473 → 67,
>   `perlin_2d` 1,461 → 82 — those four were 94–100% instrument. The old numbers
>   also FLATTENED them: four operations spanning **149x** in reality were reported
>   within **1.26x** of each other, so a real regression had room to hide. 44 of 72
>   rows moved more than 10%; **none of it is a speedup**. Rows carry `regime=net`
>   from this date and the trend filter refuses to mix them with `raw` ones.
>   ⚠ `min_ns`/`max_ns` also changed meaning for anything `bench_run` chose to batch:
>   they are now per-CHUNK averages, not per-iteration extremes. That is the honest
>   reading — a single sub-floor iteration has no measurable duration — but it means
>   the spread narrows for reasons unrelated to the code. `estimate_ns` (the avg) is
>   unaffected and remains the column the trend is built on.
> * **2026-08-10**: 17 sub-microsecond benchmarks moved from `bench()` to
>   `bench_batch()`. `bench_run` wraps a `clock_gettime` PAIR around every call
>   (~240 ns, documented in `lib/bench.cyr`), so those rows were **~95% clock
>   overhead**: `ray_sphere` read 1,466 ns and is 79 ns; `vec3_add` read 1,457 ns
>   and is 36 ns. It also FLATTENED them — triangle/sphere measured 1.19x when the
>   true ratio is 3.6x — so a real regression could hide inside the overhead.
>   Their drop at this date is an artefact of the fix, not a speedup.
> * **2026-08-09**: `estimate_ns` changed from each benchmark's MAX to its AVG.
>   The `stat` column records which; rows with no value there are `max`.

Latest: **2026-11-01T01:37:16Z** — commit `b6e8e1a`, 3.4.0 uncommitted

Tracking: `a09d228` (baseline) → `a6362ce` (mid) → `b6e8e1a`, 3.4.0 uncommitted (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 27.00 ns **-57%** | 28.00 ns **-56%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 34.00 ns **-15%** | 36.00 ns |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 381.0 ns **-36%** | 423.0 ns **-29%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 297.0 ns | 343.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1904.0 ns **-12%** | 2037.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2731.0 ns **-16%** | 2833.0 ns **-13%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 40.00 ns **-37%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 239.0 ns | 177.0 ns **-31%** |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 51.00 ns **-19%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 118.0 ns **-11%** | 128.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 244.0 ns | 196.0 ns **-24%** |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 94.00 ns **-23%** | 104.0 ns **-15%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 147.0 ns **-32%** | 173.0 ns **-20%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 193.0 ns **-29%** | 221.0 ns **-18%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 99.00 ns **-43%** | 112.0 ns **-35%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 471.0 ns **-39%** | 461.0 ns **-40%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 180.0 ns **-38%** | 209.0 ns **-28%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 546.0 ns **-33%** | 571.0 ns **-30%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 694.0 ns **-30%** | 841.0 ns **-15%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 44037.0 ns **-24%** | 44773.0 ns **-23%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4526.0 ns **-23%** | 4514.0 ns **-23%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 280.0 ns **-37%** | 281.0 ns **-37%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 64.00 ns **-44%** | 70.00 ns **-39%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 516.0 ns **-30%** | 584.0 ns **-21%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 382.0 ns **-29%** | 375.0 ns **-30%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 58.00 ns **-36%** | 62.00 ns **-31%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 41.00 ns **-56%** | 52.00 ns **-44%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 141.0 ns **-43%** | 104.0 ns **-58%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 96.00 ns +17% | 199.0 ns +143% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 23.00 ns **-23%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 91.00 ns | 96.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4849.0 ns | 5113.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19974.0 ns | 1797.0 ns **-91%** |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 75.00 ns | 81.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1707.0 µs **-11%** | 1654.0 µs **-13%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 734800.0 ns **-11%** | 737946.0 ns **-11%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2344.0 µs **-29%** | 2177.0 µs **-34%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 955724.0 ns **-46%** | 1054.0 µs **-40%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2352.0 µs | 2404.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 542429.0 ns | 552235.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 998.0 ns | 978.0 ns **-11%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 448.0 ns | 433.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 665.0 ns **-36%** | 352.0 ns **-66%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1429.0 µs | 1589.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 371141.0 ns | 612391.0 ns +60% |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1627.0 µs | 1586.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5416.0 µs | 5381.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1211.0 ns **-17%** | 1180.0 ns **-19%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 120909.0 ns **-24%** | 127432.0 ns **-20%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 91086.0 ns **-24%** | 94324.0 ns **-21%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 86456.0 ns **-21%** | 77772.0 ns **-29%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13781.0 ns **-21%** | 13056.0 ns **-25%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13681.0 ns **-22%** | 13548.0 ns **-22%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 423561.0 ns **-21%** | 397301.0 ns **-26%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 81371.0 ns **-27%** | 80622.0 ns **-28%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 463.0 ns **-30%** | 557.0 ns **-15%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 671.0 ns **-26%** | 803.0 ns **-11%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1024.0 ns **-36%** | 1801.0 ns +12% |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1188.0 ns **-35%** | 2228.0 ns +21% |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 978.0 ns **-29%** | 556.0 ns **-59%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3419.0 ns **-30%** | 1768.0 ns **-64%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3544.0 ns **-29%** | 2219.0 ns **-56%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4770.0 µs **-18%** | 4968.0 µs **-14%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 380609.0 ns **-11%** | 416701.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 283207.0 ns | 277416.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6568.0 µs | 6770.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1583.0 µs | 1657.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 446901.0 ns | 457495.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | 1175.0 µs | 1135.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | 183629.0 ns | 188026.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | — | 199256.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `cga_point_product` | — | — | 980.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | — | 343.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | — | 529.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 22.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 17.00 ns |

## m3_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m3_mul` | — | — | 88.00 ns |

## m3_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m3_inverse` | — | — | 84.00 ns |

## m3_mul_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m3_mul_vec3` | — | — | 41.00 ns |

## ivl_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ivl_mul` | — | — | 198.0 ns |

## ivl_sin

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ivl_sin` | — | — | 138.0 ns |

## ivl_width

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ivl_width` | — | — | 15.00 ns |

## calc_adaptive_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `calc_adaptive_simpson` | — | — | 8147.0 ns |

## calc_bspline_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `calc_bspline_cubic_8` | — | — | 426.0 ns |

## calc_nurbs_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_cubic_8` | — | — | 546.0 ns |

## calc_hessian_4

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `calc_hessian_4` | — | — | 2051.0 ns |

## perlin_3d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `perlin_3d` | — | — | 194.0 ns |

## christoffel_symbols_4d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `christoffel_symbols_4d` | — | — | 6448.0 ns |

## riemann_tensor_4d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `riemann_tensor_4d` | — | — | 29796.0 ns |

## geodesic_rk4_4d_100

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `geodesic_rk4_4d_100` | — | — | 225695.0 ns |

## su2_exp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `su2_exp` | — | — | 104.0 ns |

## su2_log

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `su2_log` | — | — | 131.0 ns |

## lorentz_boost

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `lorentz_boost` | — | — | 294.0 ns |

## so3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `so3_exp` | — | — | 237.0 ns |

## so3_log

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `so3_log` | — | — | 162.0 ns |

## se3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `se3_exp` | — | — | 675.0 ns |

## se3_log

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `se3_log` | — | — | 652.0 ns |

## simplex_2d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `simplex_2d` | — | — | 88.00 ns |

## simplex_3d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `simplex_3d` | — | — | 155.0 ns |

## ode_dopri45_step

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_step` | — | — | 240.0 ns |

## ode_dopri45_trajectory

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_trajectory` | — | — | 35277.0 ns |

## ode_bdf2_step

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ode_bdf2_step` | — | — | 238.0 ns |

## ode_yoshida4_step

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ode_yoshida4_step` | — | — | 63.00 ns |

## opt_lbfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_rosenbrock` | — | — | 26629.0 ns |

## opt_bfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_rosenbrock` | — | — | 9947.0 ns |

## opt_lm_expfit_32

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `opt_lm_expfit_32` | — | — | 10978.0 ns |

## expr_eval

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `expr_eval` | — | — | 542.0 ns |

## expr_diff_simplify

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `expr_diff_simplify` | — | — | 1267.0 ns |

## sym_integrate

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `sym_integrate` | — | — | 373.0 ns |

## sym_rewrite_default

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_default` | — | — | 6590.0 ns |

## csr_spmv_256

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `csr_spmv_256` | — | — | 4161.0 ns |

## solve_gmres_64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64` | — | — | 80552.0 ns |

## solve_bicgstab_64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `solve_bicgstab_64` | — | — | 31428.0 ns |

## eigen_symmetric_12

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `eigen_symmetric_12` | — | — | 287112.0 ns |

## f64_fmod

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `f64_fmod` | — | — | 21.00 ns |

## f64_fmod_gap2097

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `f64_fmod_gap2097` | — | — | 4137.0 ns |

## num_is_prime_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_is_prime_2e63` | — | — | 414284.0 ns |

## num_pollard_rho_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_pollard_rho_2e63` | — | — | 76767.0 µs |

## num_crt_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_crt_2e63` | — | — | 972.0 ns |

## num_modinv_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_modinv_2e63` | — | — | 25.00 ns |

## num_divisor_sigma_720720

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma_720720` | — | — | 401.0 ns |

## num_divisor_sigma0_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma0_2e63` | — | — | 325222.0 ns |

## dual_pow_x3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `dual_pow_x3` | — | — | 230.0 ns |

## hvec3_angle

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `hvec3_angle` | — | — | 535.0 ns |

## perlin_2d_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `perlin_2d_2e63` | — | — | 81.00 ns |

## delaunay_2d_mixed_100

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_mixed_100` | — | — | 905499.0 ns |

## bvh_query_ray_oblique_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_oblique_200x4k` | — | — | 779320.0 ns |

## cga_blade_inverse_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_sphere` | — | — | 788.0 ns |

## cga_blade_inverse_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_tiny` | — | — | 1160.0 ns |

## eigen_power_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_3x3` | — | — | 7451.0 ns |

## eigen_power_12x12

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_12x12` | — | — | 25354.0 ns |

## eigen_power_2x2_wide

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_wide` | — | — | 5977.0 ns |

## mpr_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_near_tangent` | — | — | 10124.0 ns |

## gjk_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_near_tangent` | — | — | 10035.0 ns |

## gjk_intersect_box_miss_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss_2e600` | — | — | 790.0 ns |

## gjk_intersect_box_hit_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit_2e600` | — | — | 3792.0 ns |

## gjk_epa_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes_separated` | — | — | 610.0 ns |

## gjk_epa_spheres_separated

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres_separated` | — | — | 839.0 ns |

## mpr_penetration_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes_separated` | — | — | 1221.0 ns |

## time_of_impact_sph_hit

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_sph_hit` | — | — | 98610.0 ns |

## time_of_impact_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_miss` | — | — | 1993.0 ns |

## calc_integral_gauss5_x2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `calc_integral_gauss5_x2` | — | — | 44.00 ns |

## calc_integral_trapez_x2_16

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `calc_integral_trapez_x2_16` | — | — | 141.0 ns |

## num_bisection_x2m2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_bisection_x2m2` | — | — | 480.0 ns |

## ray_sphere_near_surface

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_near_surface` | — | — | 227.0 ns |

## ray_aabb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_face_origin` | — | — | 92.00 ns |

## ray_obb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_obb_face_origin` | — | — | 307.0 ns |

## gjk_intersect_sep_capsule_1e14

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sep_capsule_1e14` | — | — | 129536.0 ns |

## solve_gmres_64_m4

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64_m4` | — | — | 46794.0 ns |

## cga_dense_product

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `cga_dense_product` | — | — | 22979.0 ns |

## num_fft_256

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_fft_256` | — | — | 41863.0 ns |

## num_fft_4096

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_fft_4096` | — | — | 974106.0 ns |

## kdtree_nearest_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `kdtree_nearest_4k` | — | — | 2143.0 ns |

## sh_evaluate_l2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `sh_evaluate_l2` | — | — | 45.00 ns |

## lorentz_is_valid

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `lorentz_is_valid` | — | — | 785.0 ns |

## levi_civita_4

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `levi_civita_4` | — | — | 28.00 ns |

## dual_ln

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `dual_ln` | — | — | 52.00 ns |

## dual_sqrt

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `dual_sqrt` | — | — | 22.00 ns |

## dual_abs

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `dual_abs` | — | — | 21.00 ns |

## num_bisection_stall

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_bisection_stall` | — | — | 2291.0 ns |

## expr_eval_unbound

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `expr_eval_unbound` | — | — | 581.0 ns |

## num_bisection_wide

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `num_bisection_wide` | — | — | 11352.0 ns |

## eigen_power_tri5

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri5` | — | — | 1598.0 ns |

## eigen_power_tiny2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tiny2` | — | — | 5028.0 ns |

## eigen_power_wide3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_wide3` | — | — | 155772.0 ns |

## lyapunov_4x4_k50

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `lyapunov_4x4_k50` | — | — | 80505.0 ns |

## svd_compute_checked_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `svd_compute_checked_3x3` | — | — | 2960.0 ns |

## calc_integral_simpson_x2_16

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson_x2_16` | — | — | 332.0 ns |

## ode_backward_euler_step

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ode_backward_euler_step` | — | — | 58.00 ns |

## fbm_2d_4oct

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `fbm_2d_4oct` | — | — | 361.0 ns |

## opt_cg_quad3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `opt_cg_quad3` | — | — | 1195.0 ns |

## opt_lbfgs_quad3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_quad3` | — | — | 6276.0 ns |

## gjk_epa_face_1e9

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_face_1e9` | — | — | 34974.0 ns |

## gjk_intersect_sep_edge_1e12

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sep_edge_1e12` | — | — | 23590.0 ns |

## gjk_intersect_cap_1e3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_cap_1e3` | — | — | 31938.0 ns |

## time_of_impact_box_edge

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_edge` | — | — | 180061.0 ns |

## gjk_epa_small_edge_1e14

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_small_edge_1e14` | — | — | 177053.0 ns |

## gjk_intersect_small_edge_1e11

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_small_edge_1e11` | — | — | 39444.0 ns |

## gjk_intersect_box_ee_1e3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_ee_1e3` | — | — | 7191.0 ns |

## mpr_intersect_band_1e6

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_band_1e6` | — | — | 9380.0 ns |

## ray_capsule_far_axis

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_axis` | — | — | 712.0 ns |

## ray_capsule_near_barrel

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_near_barrel` | — | — | 2194.0 ns |

## ray_capsule_thin_2e168

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_2e168` | — | — | 7132.0 ns |

## ray_capsule_half_infinite

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_half_infinite` | — | — | 290.0 ns |

## barycentric_colinear

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `barycentric_colinear` | — | — | 5678.0 ns |

## barycentric_thin

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `barycentric_thin` | — | — | 11924.0 ns |

## m3_normal_matrix_cof

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof` | — | — | 88.00 ns |

## m3_inverse_near_singular

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m3_inverse_near_singular` | — | — | 231.0 ns |

## m4_inverse_singular_rank3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_singular_rank3` | — | — | 2243.0 ns |

## m4_inverse_near_singular

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_near_singular` | — | — | 558.0 ns |

## m4_transform_point_w0

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point_w0` | — | — | 112.0 ns |

## quat_rotate_vec3_q1000

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3_q1000` | — | — | 112.0 ns |

## quat_inverse_checked

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `quat_inverse_checked` | — | — | 35.00 ns |

## quat_normalize_checked

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `quat_normalize_checked` | — | — | 33.00 ns |

## inverse_lerp_checked

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `inverse_lerp_checked` | — | — | 11.00 ns |

## world_to_screen_checked

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `world_to_screen_checked` | — | — | 110.0 ns |

## m4_inverse_view_projection

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_view_projection` | — | — | 562.0 ns |

## m4_inverse_exact

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_exact` | — | — | 11373.0 ns |

## m4_transform_point_exact_w

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point_exact_w` | — | — | 1069.0 ns |

## ray_capsule_thin_far_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_far_miss` | — | — | 450.0 ns |

## ray_capsule_thin_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_2e600` | — | — | 12333.0 ns |

## ray_sphere_c_one_bit

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_c_one_bit` | — | — | 222.0 ns |

## ray_capsule_seam_graze

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_seam_graze` | — | — | 749123.0 ns |

## jet_capsule_half_infinite

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `jet_capsule_half_infinite` | — | — | 668.0 ns |

## ray_sphere_graze_close

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`b6e8e1a`, 3.4.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_graze_close` | — | — | 519.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
