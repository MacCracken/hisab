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

Latest: **2026-09-14T15:28:50Z** — commit `52e0f55`

Tracking: `a09d228` (baseline) → `5857854` (mid) → `52e0f55` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 18.00 ns **-22%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 28.00 ns **-56%** | 30.00 ns **-52%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 35.00 ns **-12%** | 39.00 ns |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 380.0 ns **-36%** | 436.0 ns **-26%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 297.0 ns | 322.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1909.0 ns **-12%** | 2066.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2717.0 ns **-17%** | 2951.0 ns |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 41.00 ns **-35%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 240.0 ns | 264.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 42.00 ns **-33%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 118.0 ns **-11%** | 127.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 244.0 ns | 266.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 95.00 ns **-22%** | 102.0 ns **-16%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 151.0 ns **-30%** | 161.0 ns **-26%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 190.0 ns **-30%** | 208.0 ns **-23%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 100.0 ns **-42%** | 108.0 ns **-38%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 467.0 ns **-40%** | 508.0 ns **-34%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 178.0 ns **-39%** | 195.0 ns **-33%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 541.0 ns **-34%** | 590.0 ns **-28%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 691.0 ns **-30%** | 760.0 ns **-23%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 43830.0 ns **-25%** | 48413.0 ns **-17%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4505.0 ns **-23%** | 4985.0 ns **-15%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 282.0 ns **-36%** | 313.0 ns **-29%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 66.00 ns **-43%** | 70.00 ns **-39%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 511.0 ns **-31%** | 567.0 ns **-23%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 376.0 ns **-30%** | 415.0 ns **-22%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 58.00 ns **-36%** | 63.00 ns **-30%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 41.00 ns **-56%** | 47.00 ns **-49%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 141.0 ns **-43%** | 153.0 ns **-38%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 96.00 ns +17% | 104.0 ns +27% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 24.00 ns **-20%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 91.00 ns | 100.0 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4822.0 ns | 5278.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 28.00 ns +12% |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19690.0 ns | 21655.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 25.00 ns **-29%** | 26.00 ns **-26%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 74.00 ns | 82.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1645.0 µs **-14%** | 1812.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 730157.0 ns **-12%** | 802171.0 ns |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2297.0 µs **-30%** | 2531.0 µs **-23%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 935245.0 ns **-47%** | 1031.0 µs **-42%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2292.0 µs | 2545.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 529838.0 ns | 584085.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 955.0 ns **-13%** | 1059.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 418.0 ns | 465.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 669.0 ns **-35%** | 845.0 ns **-18%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1385.0 µs **-13%** | 1526.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 355401.0 ns | 393824.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1544.0 µs | 1694.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5259.0 µs | 5792.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1184.0 ns **-19%** | 1292.0 ns **-12%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 111422.0 ns **-30%** | 130724.0 ns **-18%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 84593.0 ns **-29%** | 96642.0 ns **-19%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 80638.0 ns **-26%** | 89378.0 ns **-18%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 12845.0 ns **-26%** | 14026.0 ns **-19%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 12905.0 ns **-26%** | 14216.0 ns **-19%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 408642.0 ns **-24%** | 451143.0 ns **-16%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 79652.0 ns **-29%** | 88642.0 ns **-21%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 459.0 ns **-30%** | 499.0 ns **-24%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 660.0 ns **-27%** | 729.0 ns **-19%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 998.0 ns **-38%** | 1102.0 ns **-31%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1147.0 ns **-38%** | 1256.0 ns **-32%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 957.0 ns **-30%** | 1055.0 ns **-23%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3413.0 ns **-30%** | 3828.0 ns **-22%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3482.0 ns **-30%** | 3840.0 ns **-23%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4728.0 µs **-19%** | 5218.0 µs **-10%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 361108.0 ns **-16%** | 396259.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 270984.0 ns | 302130.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6510.0 µs | 7190.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1571.0 µs | 1730.0 µs +11% |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 436871.0 ns | 482244.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | — | 1219.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | — | 197524.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | — | 213684.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `cga_point_product` | — | — | 1290.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | — | 1409.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`5857854`) | Current (`52e0f55`) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | — | 29772.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
