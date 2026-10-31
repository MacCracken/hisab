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

Latest: **2026-10-31T11:36:06Z** — commit `c86c561`, 3.3.7 uncommitted

Tracking: `a09d228` (baseline) → `a6362ce` (mid) → `c86c561`, 3.3.7 uncommitted (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 27.00 ns **-57%** | 27.00 ns **-57%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 34.00 ns **-15%** | 34.00 ns **-15%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 381.0 ns **-36%** | 418.0 ns **-30%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 297.0 ns | 325.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1904.0 ns **-12%** | 1870.0 ns **-14%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2731.0 ns **-16%** | 2624.0 ns **-20%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 37.00 ns **-41%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 239.0 ns | 166.0 ns **-35%** |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 39.00 ns **-38%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 118.0 ns **-11%** | 119.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 244.0 ns | 247.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 94.00 ns **-23%** | 90.00 ns **-26%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 147.0 ns **-32%** | 155.0 ns **-29%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 193.0 ns **-29%** | 210.0 ns **-23%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 99.00 ns **-43%** | 99.00 ns **-43%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 471.0 ns **-39%** | 499.0 ns **-35%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 180.0 ns **-38%** | 187.0 ns **-36%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 546.0 ns **-33%** | 557.0 ns **-32%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 694.0 ns **-30%** | 899.0 ns |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 44037.0 ns **-24%** | 43253.0 ns **-26%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4526.0 ns **-23%** | 4368.0 ns **-25%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 280.0 ns **-37%** | 296.0 ns **-33%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 64.00 ns **-44%** | 70.00 ns **-39%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 516.0 ns **-30%** | 676.0 ns |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 382.0 ns **-29%** | 409.0 ns **-24%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 58.00 ns **-36%** | 60.00 ns **-33%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 41.00 ns **-56%** | 50.00 ns **-46%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 141.0 ns **-43%** | 143.0 ns **-42%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 96.00 ns +17% | 186.0 ns +127% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 21.00 ns **-30%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 91.00 ns | 92.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4849.0 ns | 5119.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 24.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19974.0 ns | 1744.0 ns **-91%** |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 22.00 ns **-37%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 75.00 ns | 79.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1707.0 µs **-11%** | 1616.0 µs **-15%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 734800.0 ns **-11%** | 730454.0 ns **-12%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2344.0 µs **-29%** | 2261.0 µs **-32%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 955724.0 ns **-46%** | 1110.0 µs **-37%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2352.0 µs | 2317.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 542429.0 ns | 531518.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 998.0 ns | 945.0 ns **-14%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 448.0 ns | 412.0 ns **-11%** |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 665.0 ns **-36%** | 375.0 ns **-64%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1429.0 µs | 1568.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 371141.0 ns | 576940.0 ns +50% |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1627.0 µs | 1554.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5416.0 µs | 5250.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1211.0 ns **-17%** | 1157.0 ns **-21%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 120909.0 ns **-24%** | 117433.0 ns **-26%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 91086.0 ns **-24%** | 90334.0 ns **-24%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 86456.0 ns **-21%** | 87721.0 ns **-20%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13781.0 ns **-21%** | 13081.0 ns **-25%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13681.0 ns **-22%** | 14120.0 ns **-19%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 423561.0 ns **-21%** | 404193.0 ns **-25%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 81371.0 ns **-27%** | 85574.0 ns **-23%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 463.0 ns **-30%** | 512.0 ns **-22%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 671.0 ns **-26%** | 731.0 ns **-19%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1024.0 ns **-36%** | 1555.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1188.0 ns **-35%** | 1708.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 978.0 ns **-29%** | 1052.0 ns **-23%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3419.0 ns **-30%** | 3169.0 ns **-35%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3544.0 ns **-29%** | 3954.0 ns **-21%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4770.0 µs **-18%** | 4804.0 µs **-17%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 380609.0 ns **-11%** | 460313.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 283207.0 ns | 262982.0 ns **-11%** |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6568.0 µs | 6592.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1583.0 µs | 1590.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 446901.0 ns | 435699.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | 1175.0 µs | 1106.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | 183629.0 ns | 179660.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | — | 192915.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `cga_point_product` | — | — | 924.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | — | 329.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | — | 493.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 21.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 17.00 ns |

## m3_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `m3_mul` | — | — | 87.00 ns |

## m3_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `m3_inverse` | — | — | 172.0 ns |

## m3_mul_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `m3_mul_vec3` | — | — | 41.00 ns |

## ivl_mul

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ivl_mul` | — | — | 196.0 ns |

## ivl_sin

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ivl_sin` | — | — | 132.0 ns |

## ivl_width

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ivl_width` | — | — | 14.00 ns |

## calc_adaptive_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `calc_adaptive_simpson` | — | — | 6134.0 ns |

## calc_bspline_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `calc_bspline_cubic_8` | — | — | 411.0 ns |

## calc_nurbs_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_cubic_8` | — | — | 531.0 ns |

## calc_hessian_4

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `calc_hessian_4` | — | — | 1990.0 ns |

## perlin_3d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `perlin_3d` | — | — | 184.0 ns |

## christoffel_symbols_4d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `christoffel_symbols_4d` | — | — | 6157.0 ns |

## riemann_tensor_4d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `riemann_tensor_4d` | — | — | 29113.0 ns |

## geodesic_rk4_4d_100

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `geodesic_rk4_4d_100` | — | — | 219890.0 ns |

## su2_exp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `su2_exp` | — | — | 100.0 ns |

## su2_log

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `su2_log` | — | — | 124.0 ns |

## lorentz_boost

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `lorentz_boost` | — | — | 284.0 ns |

## so3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `so3_exp` | — | — | 228.0 ns |

## so3_log

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `so3_log` | — | — | 152.0 ns |

## se3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `se3_exp` | — | — | 646.0 ns |

## se3_log

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `se3_log` | — | — | 624.0 ns |

## simplex_2d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `simplex_2d` | — | — | 84.00 ns |

## simplex_3d

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `simplex_3d` | — | — | 153.0 ns |

## ode_dopri45_step

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_step` | — | — | 231.0 ns |

## ode_dopri45_trajectory

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_trajectory` | — | — | 33894.0 ns |

## ode_bdf2_step

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ode_bdf2_step` | — | — | 448.0 ns |

## ode_yoshida4_step

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ode_yoshida4_step` | — | — | 61.00 ns |

## opt_lbfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_rosenbrock` | — | — | 246861.0 ns |

## opt_bfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_rosenbrock` | — | — | 9585.0 ns |

## opt_lm_expfit_32

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `opt_lm_expfit_32` | — | — | 10295.0 ns |

## expr_eval

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `expr_eval` | — | — | 501.0 ns |

## expr_diff_simplify

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `expr_diff_simplify` | — | — | 1123.0 ns |

## sym_integrate

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `sym_integrate` | — | — | 338.0 ns |

## sym_rewrite_default

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_default` | — | — | 6193.0 ns |

## csr_spmv_256

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `csr_spmv_256` | — | — | 3895.0 ns |

## solve_gmres_64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64` | — | — | 71418.0 ns |

## solve_bicgstab_64

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `solve_bicgstab_64` | — | — | 25404.0 ns |

## eigen_symmetric_12

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `eigen_symmetric_12` | — | — | 277450.0 ns |

## f64_fmod

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `f64_fmod` | — | — | 20.00 ns |

## f64_fmod_gap2097

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `f64_fmod_gap2097` | — | — | 4011.0 ns |

## num_is_prime_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_is_prime_2e63` | — | — | 395640.0 ns |

## num_pollard_rho_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_pollard_rho_2e63` | — | — | 74154.0 µs |

## num_crt_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_crt_2e63` | — | — | 1014.0 ns |

## num_modinv_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_modinv_2e63` | — | — | 42.00 ns |

## num_divisor_sigma_720720

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma_720720` | — | — | 390.0 ns |

## num_divisor_sigma0_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma0_2e63` | — | — | 318006.0 ns |

## dual_pow_x3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `dual_pow_x3` | — | — | 219.0 ns |

## hvec3_angle

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `hvec3_angle` | — | — | 513.0 ns |

## perlin_2d_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `perlin_2d_2e63` | — | — | 78.00 ns |

## delaunay_2d_mixed_100

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_mixed_100` | — | — | 904992.0 ns |

## bvh_query_ray_oblique_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_oblique_200x4k` | — | — | 737281.0 ns |

## cga_blade_inverse_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_sphere` | — | — | 740.0 ns |

## cga_blade_inverse_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_tiny` | — | — | 1100.0 ns |

## eigen_power_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `eigen_power_3x3` | — | — | 6431.0 ns |

## eigen_power_12x12

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `eigen_power_12x12` | — | — | 21184.0 ns |

## eigen_power_2x2_wide

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_wide` | — | — | 12187.0 ns |

## mpr_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_near_tangent` | — | — | 223621.0 ns |

## gjk_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_near_tangent` | — | — | 809103.0 ns |

## gjk_intersect_box_miss_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss_2e600` | — | — | 733.0 ns |

## gjk_intersect_box_hit_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit_2e600` | — | — | 3441.0 ns |

## gjk_epa_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes_separated` | — | — | 743.0 ns |

## gjk_epa_spheres_separated

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres_separated` | — | — | 1033.0 ns |

## mpr_penetration_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes_separated` | — | — | 1264.0 ns |

## time_of_impact_sph_hit

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_sph_hit` | — | — | 92509.0 ns |

## time_of_impact_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_miss` | — | — | 1896.0 ns |

## calc_integral_gauss5_x2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `calc_integral_gauss5_x2` | — | — | 42.00 ns |

## calc_integral_trapez_x2_16

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `calc_integral_trapez_x2_16` | — | — | 145.0 ns |

## num_bisection_x2m2

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_bisection_x2m2` | — | — | 461.0 ns |

## ray_sphere_near_surface

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_near_surface` | — | — | 205.0 ns |

## ray_aabb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_face_origin` | — | — | 115.0 ns |

## ray_obb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `ray_obb_face_origin` | — | — | 331.0 ns |

## gjk_intersect_sep_capsule_1e14

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sep_capsule_1e14` | — | — | 144059.0 ns |

## solve_gmres_64_m4

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64_m4` | — | — | 39335.0 ns |

## cga_dense_product

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `cga_dense_product` | — | — | 22332.0 ns |

## num_fft_256

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_fft_256` | — | — | 40618.0 ns |

## num_fft_4096

| Benchmark | Baseline (`a09d228`) | Mid (`a6362ce`) | Current (`c86c561`, 3.3.7 uncommitted) |
|-----------|------|------|------|
| `num_fft_4096` | — | — | 946350.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
