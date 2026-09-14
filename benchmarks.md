# Benchmarks

> **Measurement changes** — read before comparing across a date.
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

Latest: **2026-09-14T16:33:36Z** — commit `50d2589`

Tracking: `a09d228` (baseline) → `8924440` (mid) → `50d2589` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 17.00 ns **-26%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 28.00 ns **-56%** | 29.00 ns **-54%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 35.00 ns **-12%** | 35.00 ns **-12%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 381.0 ns **-36%** | 393.0 ns **-34%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 298.0 ns | 299.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1905.0 ns **-12%** | 2061.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2722.0 ns **-17%** | 2883.0 ns **-12%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 40.00 ns **-37%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 238.0 ns | 258.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 41.00 ns **-35%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 117.0 ns **-11%** | 134.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 244.0 ns | 261.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 93.00 ns **-24%** | 95.00 ns **-22%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 151.0 ns **-30%** | 148.0 ns **-32%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 191.0 ns **-30%** | 195.0 ns **-28%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 99.00 ns **-43%** | 98.00 ns **-43%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 476.0 ns **-38%** | 468.0 ns **-39%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 180.0 ns **-38%** | 182.0 ns **-37%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 548.0 ns **-33%** | 542.0 ns **-34%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 694.0 ns **-30%** | 694.0 ns **-30%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 43742.0 ns **-25%** | 44204.0 ns **-24%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4505.0 ns **-23%** | 4501.0 ns **-23%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 282.0 ns **-36%** | 284.0 ns **-36%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 64.00 ns **-44%** | 64.00 ns **-44%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 513.0 ns **-31%** | 520.0 ns **-30%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 379.0 ns **-29%** | 400.0 ns **-25%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 57.00 ns **-37%** | 61.00 ns **-32%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 41.00 ns **-56%** | 46.00 ns **-51%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 141.0 ns **-43%** | 149.0 ns **-39%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 96.00 ns +17% | 101.0 ns +23% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 23.00 ns **-23%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 91.00 ns | 94.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4821.0 ns | 4978.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 27.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19691.0 ns | 19588.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 6.00 ns **-14%** | 6.00 ns **-14%** |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 73.00 ns | 73.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1636.0 µs **-14%** | 1658.0 µs **-13%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 743902.0 ns **-10%** | 728173.0 ns **-12%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2294.0 µs **-31%** | 2290.0 µs **-31%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 981522.0 ns **-45%** | 932072.0 ns **-47%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2357.0 µs | 2362.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 541819.0 ns | 537091.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 945.0 ns **-14%** | 975.0 ns **-12%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 427.0 ns | 422.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 713.0 ns **-31%** | 661.0 ns **-36%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1385.0 µs **-13%** | 1401.0 µs **-11%** |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 357356.0 ns | 367571.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1782.0 µs +10% | 1571.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5579.0 µs | 5344.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1231.0 ns **-16%** | 1180.0 ns **-19%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 116831.0 ns **-26%** | 117757.0 ns **-26%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 88340.0 ns **-26%** | 90113.0 ns **-24%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 81777.0 ns **-25%** | 80903.0 ns **-26%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13369.0 ns **-23%** | 12809.0 ns **-26%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13475.0 ns **-23%** | 13298.0 ns **-24%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 415621.0 ns **-23%** | 409467.0 ns **-24%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 80381.0 ns **-28%** | 79538.0 ns **-29%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 462.0 ns **-30%** | 454.0 ns **-31%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 664.0 ns **-27%** | 665.0 ns **-27%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1028.0 ns **-36%** | 996.0 ns **-38%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1180.0 ns **-36%** | 1155.0 ns **-37%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 965.0 ns **-30%** | 959.0 ns **-30%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3391.0 ns **-31%** | 3377.0 ns **-31%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3577.0 ns **-28%** | 3497.0 ns **-30%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4728.0 µs **-19%** | 4735.0 µs **-18%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 413210.0 ns | 367852.0 ns **-14%** |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 281705.0 ns | 284281.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6567.0 µs | 6579.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1585.0 µs | 1621.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 447627.0 ns | 456642.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | — | 1138.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | — | 182965.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | — | 199038.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `cga_point_product` | — | — | 1167.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | — | 1283.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | — | 27206.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 22.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`8924440`) | Current (`50d2589`) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 18.00 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
