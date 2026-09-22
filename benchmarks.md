# Benchmarks

> **Measurement changes** — read before comparing across a date.
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
>   (`docs/development/issues/2026-09-21-cyrius-bench-min-above-mean-*.md`).
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

Latest: **2026-09-22T07:08:15Z** — commit `379ac5a`

Tracking: `a09d228` (baseline) → `8924440` (mid) → `379ac5a` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 17.00 ns **-26%** | 15.00 ns **-35%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 30.00 ns **-52%** | 27.00 ns **-57%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 36.00 ns | 34.00 ns **-15%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 401.0 ns **-32%** | 382.0 ns **-36%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 317.0 ns | 291.0 ns **-10%** |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 2246.0 ns | 1879.0 ns **-13%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 3240.0 ns | 2661.0 ns **-18%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 46.00 ns **-27%** | 36.00 ns **-43%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 268.0 ns | 236.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 48.00 ns **-24%** | 38.00 ns **-40%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 147.0 ns +11% | 118.0 ns **-11%** |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 321.0 ns +24% | 237.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 109.0 ns **-11%** | 91.00 ns **-25%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 191.0 ns **-12%** | 145.0 ns **-33%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 226.0 ns **-17%** | 186.0 ns **-31%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 119.0 ns **-31%** | 98.00 ns **-43%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 517.0 ns **-33%** | 460.0 ns **-40%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 185.0 ns **-36%** | 176.0 ns **-40%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 558.0 ns **-32%** | 536.0 ns **-35%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 712.0 ns **-28%** | 680.0 ns **-31%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 45170.0 ns **-22%** | 44864.0 ns **-23%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4684.0 ns **-20%** | 4421.0 ns **-25%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 283.0 ns **-36%** | 343.0 ns **-23%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 64.00 ns **-44%** | 62.00 ns **-46%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 525.0 ns **-29%** | 642.0 ns **-13%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 390.0 ns **-27%** | 459.0 ns **-14%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 59.00 ns **-34%** | 64.00 ns **-29%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 43.00 ns **-54%** | 42.00 ns **-55%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 145.0 ns **-41%** | 180.0 ns **-27%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 98.00 ns +20% | 95.00 ns +16% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 23.00 ns **-23%** | 28.00 ns |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 93.00 ns | 89.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4999.0 ns | 4667.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 24.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 20298.0 ns | 18949.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 24.00 ns **-31%** | 27.00 ns **-23%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 6.00 ns **-14%** |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 76.00 ns | 73.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1675.0 µs **-12%** | 1649.0 µs **-14%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 734924.0 ns **-11%** | 844495.0 ns |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2361.0 µs **-29%** | 2572.0 µs **-22%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 987275.0 ns **-44%** | 979415.0 ns **-45%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2362.0 µs | 2453.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 543241.0 ns | 546114.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 979.0 ns **-11%** | 954.0 ns **-13%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 439.0 ns | 428.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 663.0 ns **-36%** | 471.0 ns **-54%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1447.0 µs | 1542.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 372065.0 ns | 401658.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1595.0 µs | 1580.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5387.0 µs | 5274.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1207.0 ns **-17%** | 1275.0 ns **-13%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 118962.0 ns **-25%** | 121071.0 ns **-24%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 90107.0 ns **-24%** | 87921.0 ns **-26%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 82876.0 ns **-24%** | 97593.0 ns **-11%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13877.0 ns **-20%** | 19199.0 ns +10% |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13651.0 ns **-22%** | 18893.0 ns |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 424455.0 ns **-21%** | 461604.0 ns **-14%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 82473.0 ns **-26%** | 94999.0 ns **-15%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 463.0 ns **-30%** | 585.0 ns **-11%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 678.0 ns **-25%** | 886.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1031.0 ns **-36%** | 1305.0 ns **-19%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1164.0 ns **-37%** | 1462.0 ns **-21%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 979.0 ns **-29%** | 1276.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3451.0 ns **-30%** | 4434.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3561.0 ns **-29%** | 4608.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4853.0 µs **-16%** | 4939.0 µs **-15%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 381486.0 ns **-11%** | 469433.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 282521.0 ns | 268170.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6633.0 µs | 6525.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1661.0 µs | 1575.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 466001.0 ns | 438112.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | — | 1123.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | — | 178508.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | — | 207642.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `cga_point_product` | — | — | 1567.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | — | 1577.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | — | 38879.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 29.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`379ac5a`) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 22.00 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
