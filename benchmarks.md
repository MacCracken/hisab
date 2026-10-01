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

Latest: **2026-10-01T06:13:58Z** — commit `cf4517e`

Tracking: `a09d228` (baseline) → `d740afc` (mid) → `cf4517e` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 17.00 ns **-26%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 30.00 ns **-52%** | 27.00 ns **-57%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 38.00 ns | 34.00 ns **-15%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 390.0 ns **-34%** | 402.0 ns **-32%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 306.0 ns | 327.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1944.0 ns **-10%** | 1902.0 ns **-12%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2770.0 ns **-15%** | 2813.0 ns **-14%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 39.00 ns **-38%** | 38.00 ns **-40%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 246.0 ns | 169.0 ns **-34%** |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 39.00 ns **-38%** | 37.00 ns **-41%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 120.0 ns | 122.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 250.0 ns | 249.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 100.0 ns **-18%** | 96.00 ns **-21%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 155.0 ns **-29%** | 149.0 ns **-31%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 195.0 ns **-28%** | 191.0 ns **-30%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 105.0 ns **-39%** | 99.00 ns **-43%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 497.0 ns **-36%** | 466.0 ns **-40%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 190.0 ns **-35%** | 180.0 ns **-38%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 561.0 ns **-32%** | 544.0 ns **-34%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 729.0 ns **-26%** | 698.0 ns **-30%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 45098.0 ns **-22%** | 43167.0 ns **-26%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4618.0 ns **-21%** | 4389.0 ns **-25%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 290.0 ns **-35%** | 282.0 ns **-36%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 65.00 ns **-43%** | 65.00 ns **-43%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 535.0 ns **-28%** | 517.0 ns **-30%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 392.0 ns **-27%** | 382.0 ns **-29%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 60.00 ns **-33%** | 58.00 ns **-36%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 42.00 ns **-55%** | 46.00 ns **-51%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 143.0 ns **-42%** | 144.0 ns **-41%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 99.00 ns +21% | 188.0 ns +129% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 23.00 ns **-23%** | 21.00 ns **-30%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 94.00 ns | 95.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 5019.0 ns | 4726.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 26.00 ns | 24.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 20261.0 ns | 1772.0 ns **-91%** |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 78.00 ns | 74.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1694.0 µs **-11%** | 1657.0 µs **-13%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 744796.0 ns | 727999.0 ns **-12%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2338.0 µs **-29%** | 2298.0 µs **-30%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 960428.0 ns **-46%** | 990227.0 ns **-44%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2374.0 µs | 2327.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 549914.0 ns | 545549.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 1007.0 ns | 941.0 ns **-15%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 442.0 ns | 411.0 ns **-11%** |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 480.0 ns **-54%** | 667.0 ns **-36%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1470.0 µs | 1388.0 µs **-12%** |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 372717.0 ns | 359431.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1598.0 µs | 1549.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5452.0 µs | 5264.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1202.0 ns **-18%** | 1168.0 ns **-20%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 118745.0 ns **-25%** | 117440.0 ns **-26%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 92775.0 ns **-22%** | 90795.0 ns **-24%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 84317.0 ns **-23%** | 82192.0 ns **-25%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13520.0 ns **-22%** | 12628.0 ns **-27%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13618.0 ns **-22%** | 13054.0 ns **-25%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 418915.0 ns **-22%** | 407957.0 ns **-24%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 82248.0 ns **-26%** | 79943.0 ns **-28%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 467.0 ns **-29%** | 459.0 ns **-30%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 687.0 ns **-24%** | 673.0 ns **-26%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1047.0 ns **-35%** | 1061.0 ns **-34%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1235.0 ns **-33%** | 1162.0 ns **-37%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 997.0 ns **-27%** | 979.0 ns **-29%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3509.0 ns **-28%** | 3510.0 ns **-28%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3715.0 ns **-26%** | 3548.0 ns **-29%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4801.0 µs **-17%** | 4676.0 µs **-19%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 392520.0 ns | 439416.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 280321.0 ns | 278130.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6678.0 µs | 6566.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1626.0 µs | 1565.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 446096.0 ns | 435399.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | 337329.0 ns | 1115.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | 184793.0 ns | 181361.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | — | 195081.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `cga_point_product` | — | — | 1148.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | — | 1241.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | — | 27731.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 21.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`cf4517e`) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 17.00 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
