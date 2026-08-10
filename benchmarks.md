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

Latest: **2026-08-10T22:27:12Z** — commit `6cb1ba5`

Tracking: `891d83e` (baseline) → `aa9febe` (mid) → `6cb1ba5` (current)

## vec3_add

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `vec3_add` | 1447.0 ns | 1449.0 ns | 34.00 ns **-98%** |

## vec3_cross

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `vec3_cross` | 1463.0 ns | 1463.0 ns | 69.00 ns **-95%** |

## vec3_normalize

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `vec3_normalize` | 1457.0 ns | 1467.0 ns | 47.00 ns **-97%** |

## vec3_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 1881.0 ns | 1987.0 ns | 1986.0 ns |

## vec4_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 1695.0 ns | 1738.0 ns | 289.0 ns **-83%** |

## m4_mul_x16

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `m4_mul_x16` | 4123.0 ns | 4328.0 ns | 4289.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `m4_transform_x64` | 5804.0 ns | 5859.0 ns | 5918.0 ns |

## quat_mul

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `quat_mul` | 1461.0 ns | 1472.0 ns | 1468.0 ns |

## quat_slerp

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `quat_slerp` | 1456.0 ns | 1718.0 ns +18% | 271.0 ns **-81%** |

## quat_rotate_vec3

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 1465.0 ns | 1465.0 ns | 76.00 ns **-95%** |

## m4_mul

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `m4_mul` | 1614.0 ns | 1530.0 ns | 181.0 ns **-89%** |

## m4_inverse

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `m4_inverse` | 1602.0 ns | 1735.0 ns | 292.0 ns **-82%** |

## m4_transform_point

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `m4_transform_point` | 1581.0 ns | 1511.0 ns | 164.0 ns **-90%** |

## t3d_compose

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `t3d_compose` | 1493.0 ns | 1741.0 ns +17% | 268.0 ns **-82%** |

## ray_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `ray_sphere` | 1466.0 ns | 1463.0 ns | 77.00 ns **-95%** |

## ray_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `ray_aabb` | 1474.0 ns | 1453.0 ns | 83.00 ns **-94%** |

## ray_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `ray_triangle` | 1554.0 ns | 1734.0 ns +12% | 284.0 ns **-82%** |

## srgb_to_linear

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `srgb_to_linear` | 1438.0 ns | 1450.0 ns | 86.00 ns **-94%** |

## tonemap_reinhard

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 1444.0 ns | 1439.0 ns | 38.00 ns **-97%** |

## calc_derivative

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `calc_derivative` | 1431.0 ns | 1452.0 ns | 95.00 ns **-93%** |

## calc_integral_simpson

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 6812.0 ns | 6535.0 ns | 6654.0 ns |

## num_gcd

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `num_gcd` | 1439.0 ns | 1412.0 ns | 26.00 ns **-98%** |

## num_is_prime

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `num_is_prime` | 21902.0 ns | 21037.0 ns | 20963.0 ns |

## cx_mul

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `cx_mul` | 1431.0 ns | 1454.0 ns | 1458.0 ns |

## ease_in_out

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `ease_in_out` | 1420.0 ns | 1343.0 ns | 1379.0 ns |

## perlin_2d

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `perlin_2d` | 1424.0 ns | 1452.0 ns | 1459.0 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 2032.0 µs | 1973.0 µs | 1926.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 1095.0 µs | 1045.0 µs | 1078.0 µs |

## bvh_degenerate_4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3795.0 µs | 3538.0 µs | 3655.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1893.0 µs | 1776.0 µs | 1834.0 µs |

## kdtree_build_4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2687.0 µs | 2498.0 µs | 2607.0 µs |

## kdtree_radius_4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1478.0 ns | 1924.0 ns +30% | 1983.0 ns +34% |

## delaunay_2d_400

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1784.0 µs | 1723.0 µs | 1762.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 450034.0 ns | 433598.0 ns | 464491.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1703.0 µs | 1643.0 µs | 1665.0 µs |

## svd_golub_kahan_12

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 172046.0 ns | 165499.0 ns | 170849.0 ns |

## eigen_qr_12

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `eigen_qr_12` | 133868.0 ns | 122449.0 ns | 129440.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 133657.0 ns | 127487.0 ns | 127867.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 25760.0 ns | 24276.0 ns | 24360.0 ns |

## gjk_epa_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 25930.0 ns | 24180.0 ns | 24588.0 ns |

## gjk_epa_spheres

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 619640.0 ns | 583239.0 ns | 593363.0 ns |

## gjk_epa_sphere_box

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 124718.0 ns | 120301.0 ns | 124400.0 ns |

## gjk_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 840.0 ns | 782.0 ns | 774.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 1182.0 ns | 1097.0 ns | 1108.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1989.0 ns | 1878.0 ns | 1891.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 2262.0 ns | 2156.0 ns | 2138.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1790.0 ns | 1675.0 ns | 1656.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 6340.0 ns | 5939.0 ns | 6063.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 6519.0 ns | 6084.0 ns | 6287.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 6348.0 µs | 6020.0 µs | 6173.0 µs |

## spatial_hash_query_2k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 543560.0 ns | 546242.0 ns | 546382.0 ns |

## num_dct_1024

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `num_dct_1024` | 293969.0 ns | 291887.0 ns | 289401.0 ns |

## num_dst_1024

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `num_dst_1024` | 7095.0 µs | 6696.0 µs | 6918.0 µs |

## num_dct_1023

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `num_dct_1023` | 1723.0 µs | 1614.0 µs | 1677.0 µs |

## num_dst_1023

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `num_dst_1023` | 475025.0 ns | 449337.0 ns | 460750.0 ns |

## kdtree_build_octave_512

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | — | 563783.0 ns | 571162.0 ns |

## triangulate_comb_600

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `triangulate_comb_600` | — | 5767.0 µs | 5776.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `triangulate_hex_6` | — | 2950.0 ns | 3004.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | — | — | 2599.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`6cb1ba5`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | — | — | 1914.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
