# Benchmarks

> **Measurement changes** — read before comparing across a date.
> * **2026-08-10**: 17 sub-microsecond benchmarks moved from `bench()` to
>   `bench_batch()`. `bench_run` wraps a `clock_gettime` PAIR around every call
>   (~240 ns, documented in `lib/bench.cyr`), so those rows were **~95% clock
>   overhead**: `ray_sphere` read 1,466 ns and is 79 ns; `vec3_add` read 1,457 ns
>   and is 36 ns. It also FLATTENED them — triangle/sphere measured 1.19x when the
>   true ratio is 3.6x — so a real regression could hide inside the overhead.
>   Their drop at this date is an artefact of the fix, not a speedup.
> * **2026-08-09**: `estimate_ns` changed from each benchmark's MAX to its AVG.
>   The `stat` column records which; rows with no value there are `max`.

Latest: **2026-08-10T22:46:07Z** — commit `a5ef9b3`

Tracking: `891d83e` (baseline) → `c59e1fc` (mid) → `a5ef9b3` (current)

## vec3_add

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `vec3_add` | 1447.0 ns | 1461.0 ns | 36.00 ns **-98%** |

## vec3_cross

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `vec3_cross` | 1463.0 ns | 1465.0 ns | 72.00 ns **-95%** |

## vec3_normalize

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `vec3_normalize` | 1457.0 ns | 1466.0 ns | 51.00 ns **-96%** |

## vec3_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 1881.0 ns | 1994.0 ns | 2070.0 ns +10% |

## vec4_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 1695.0 ns | 1738.0 ns | 307.0 ns **-82%** |

## m4_mul_x16

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `m4_mul_x16` | 4123.0 ns | 4421.0 ns | 4504.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `m4_transform_x64` | 5804.0 ns | 6105.0 ns | 6212.0 ns |

## quat_mul

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `quat_mul` | 1461.0 ns | 1468.0 ns | 1470.0 ns |

## quat_slerp

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `quat_slerp` | 1456.0 ns | 1759.0 ns +21% | 289.0 ns **-80%** |

## quat_rotate_vec3

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 1465.0 ns | 1467.0 ns | 77.00 ns **-95%** |

## m4_mul

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `m4_mul` | 1614.0 ns | 1535.0 ns | 188.0 ns **-88%** |

## m4_inverse

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `m4_inverse` | 1602.0 ns | 1738.0 ns | 309.0 ns **-81%** |

## m4_transform_point

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `m4_transform_point` | 1581.0 ns | 1515.0 ns | 171.0 ns **-89%** |

## t3d_compose

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `t3d_compose` | 1493.0 ns | 1744.0 ns +17% | 288.0 ns **-81%** |

## ray_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `ray_sphere` | 1466.0 ns | 1463.0 ns | 101.0 ns **-93%** |

## ray_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `ray_aabb` | 1474.0 ns | 1459.0 ns | 89.00 ns **-94%** |

## ray_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `ray_triangle` | 1554.0 ns | 1735.0 ns +12% | 303.0 ns **-81%** |

## srgb_to_linear

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `srgb_to_linear` | 1438.0 ns | 1456.0 ns | 91.00 ns **-94%** |

## tonemap_reinhard

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 1444.0 ns | 1462.0 ns | 41.00 ns **-97%** |

## calc_derivative

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `calc_derivative` | 1431.0 ns | 1457.0 ns | 102.0 ns **-93%** |

## calc_integral_simpson

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 6812.0 ns | 6695.0 ns | 6947.0 ns |

## num_gcd

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `num_gcd` | 1439.0 ns | 1430.0 ns | 27.00 ns **-98%** |

## num_is_prime

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `num_is_prime` | 21902.0 ns | 21189.0 ns | 22080.0 ns |

## cx_mul

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `cx_mul` | 1431.0 ns | 1458.0 ns | 1461.0 ns |

## ease_in_out

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `ease_in_out` | 1420.0 ns | 1358.0 ns | 1420.0 ns |

## perlin_2d

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `perlin_2d` | 1424.0 ns | 1457.0 ns | 1459.0 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 2032.0 µs | 1949.0 µs | 2049.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 1095.0 µs | 1064.0 µs | 1130.0 µs |

## bvh_degenerate_4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3795.0 µs | 3538.0 µs | 3841.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1893.0 µs | 1807.0 µs | 1939.0 µs |

## kdtree_build_4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2687.0 µs | 2537.0 µs | 2712.0 µs |

## kdtree_radius_4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1478.0 ns | 1971.0 ns +33% | 1938.0 ns +31% |

## delaunay_2d_400

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1784.0 µs | 1733.0 µs | 1809.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 450034.0 ns | 482836.0 ns | 463629.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1703.0 µs | 1636.0 µs | 1717.0 µs |

## svd_golub_kahan_12

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 172046.0 ns | 169267.0 ns | 178262.0 ns |

## eigen_qr_12

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `eigen_qr_12` | 133868.0 ns | 128088.0 ns | 134710.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 133657.0 ns | 127348.0 ns | 135008.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 25760.0 ns | 24390.0 ns | 25252.0 ns |

## gjk_epa_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 25930.0 ns | 24961.0 ns | 25456.0 ns |

## gjk_epa_spheres

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 619640.0 ns | 605960.0 ns | 616128.0 ns |

## gjk_epa_sphere_box

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 124718.0 ns | 142910.0 ns +15% | 126317.0 ns |

## gjk_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 840.0 ns | 778.0 ns | 796.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 1182.0 ns | 1120.0 ns | 1163.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1989.0 ns | 1857.0 ns | 1901.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 2262.0 ns | 2100.0 ns | 2158.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1790.0 ns | 1659.0 ns | 1674.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 6340.0 ns | 6017.0 ns | 6225.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 6519.0 ns | 6148.0 ns | 6341.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 6348.0 µs | 6098.0 µs | 6554.0 µs |

## spatial_hash_query_2k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 543560.0 ns | 571469.0 ns | 565588.0 ns |

## num_dct_1024

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `num_dct_1024` | 293969.0 ns | 291950.0 ns | 297572.0 ns |

## num_dst_1024

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `num_dst_1024` | 7095.0 µs | 6735.0 µs | 7160.0 µs |

## num_dct_1023

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `num_dct_1023` | 1723.0 µs | 1631.0 µs | 1735.0 µs |

## num_dst_1023

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `num_dst_1023` | 475025.0 ns | 461588.0 ns | 484118.0 ns |

## kdtree_build_octave_512

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | — | 567443.0 ns | 598833.0 ns |

## triangulate_comb_600

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `triangulate_comb_600` | — | 5717.0 µs | 6076.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `triangulate_hex_6` | — | 2977.0 ns | 3106.0 ns |

## ray_capsule

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `ray_capsule` | — | — | 526.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | — | — | 2557.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`a5ef9b3`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | — | — | 1968.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
