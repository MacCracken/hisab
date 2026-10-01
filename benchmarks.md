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

Latest: **2026-10-01T07:49:56Z** — commit `6d38643`

Tracking: `a09d228` (baseline) → `d740afc` (mid) → `6d38643` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 17.00 ns **-26%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 29.00 ns **-54%** | 27.00 ns **-57%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 36.00 ns | 34.00 ns **-15%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 392.0 ns **-34%** | 404.0 ns **-32%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 305.0 ns | 328.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1960.0 ns | 1891.0 ns **-13%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2789.0 ns **-15%** | 2779.0 ns **-15%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 39.00 ns **-38%** | 38.00 ns **-40%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 249.0 ns | 170.0 ns **-34%** |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 39.00 ns **-38%** | 37.00 ns **-41%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 121.0 ns | 122.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 252.0 ns | 252.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 95.00 ns **-22%** | 95.00 ns **-22%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 152.0 ns **-30%** | 148.0 ns **-32%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 197.0 ns **-27%** | 195.0 ns **-28%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 103.0 ns **-40%** | 101.0 ns **-42%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 482.0 ns **-38%** | 465.0 ns **-40%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 183.0 ns **-37%** | 184.0 ns **-37%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 554.0 ns **-32%** | 549.0 ns **-33%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 715.0 ns **-28%** | 697.0 ns **-30%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 45416.0 ns **-22%** | 42949.0 ns **-26%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4639.0 ns **-21%** | 4406.0 ns **-25%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 291.0 ns **-34%** | 281.0 ns **-37%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 65.00 ns **-43%** | 64.00 ns **-44%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 526.0 ns **-29%** | 528.0 ns **-29%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 385.0 ns **-28%** | 378.0 ns **-29%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 60.00 ns **-33%** | 57.00 ns **-37%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 42.00 ns **-55%** | 45.00 ns **-52%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 144.0 ns **-41%** | 144.0 ns **-41%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 98.00 ns +20% | 187.0 ns +128% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 23.00 ns **-23%** | 21.00 ns **-30%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 95.00 ns | 94.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 5046.0 ns | 4698.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 26.00 ns | 24.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 20310.0 ns | 1752.0 ns **-91%** |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 22.00 ns **-37%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 74.00 ns | 73.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1701.0 µs **-11%** | 1651.0 µs **-14%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 741191.0 ns **-10%** | 743165.0 ns **-10%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2350.0 µs **-29%** | 2337.0 µs **-29%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 959771.0 ns **-46%** | 939144.0 ns **-47%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2385.0 µs | 2336.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 551129.0 ns | 534573.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 988.0 ns **-10%** | 938.0 ns **-15%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 441.0 ns | 423.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 396.0 ns **-62%** | 679.0 ns **-34%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1474.0 µs | 1429.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 380725.0 ns | 361129.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1636.0 µs | 1561.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5387.0 µs | 5273.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1195.0 ns **-18%** | 1164.0 ns **-20%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 119433.0 ns **-25%** | 117544.0 ns **-26%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 93529.0 ns **-22%** | 87872.0 ns **-26%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 83461.0 ns **-24%** | 81436.0 ns **-26%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13438.0 ns **-23%** | 12669.0 ns **-27%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13500.0 ns **-23%** | 12756.0 ns **-27%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 419283.0 ns **-22%** | 406367.0 ns **-24%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 81536.0 ns **-27%** | 79820.0 ns **-28%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 464.0 ns **-30%** | 462.0 ns **-30%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 684.0 ns **-24%** | 662.0 ns **-27%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1027.0 ns **-36%** | 1015.0 ns **-37%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1169.0 ns **-36%** | 1155.0 ns **-37%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 982.0 ns **-28%** | 966.0 ns **-30%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3505.0 ns **-28%** | 3407.0 ns **-30%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3557.0 ns **-29%** | 3621.0 ns **-28%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4817.0 µs **-17%** | 5019.0 µs **-14%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 396152.0 ns | 382048.0 ns **-11%** |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 281794.0 ns | 280107.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6633.0 µs | 6552.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1621.0 µs | 1581.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 445600.0 ns | 433038.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | 341167.0 ns | 1110.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | 185817.0 ns | 183791.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | — | 198219.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `cga_point_product` | — | — | 1144.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | — | 1237.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | — | 26339.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 22.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`6d38643`) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 17.00 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
