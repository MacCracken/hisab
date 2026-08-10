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

Latest: **2026-08-10T22:53:39Z** — commit `b0783ad`

Tracking: `891d83e` (baseline) → `c59e1fc` (mid) → `b0783ad` (current)

## vec3_add

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `vec3_add` | 1447.0 ns | 1461.0 ns | 36.00 ns **-98%** |

## vec3_cross

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `vec3_cross` | 1463.0 ns | 1465.0 ns | 69.00 ns **-95%** |

## vec3_normalize

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `vec3_normalize` | 1457.0 ns | 1466.0 ns | 48.00 ns **-97%** |

## vec3_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 1881.0 ns | 1994.0 ns | 1990.0 ns |

## vec4_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 1695.0 ns | 1738.0 ns | 309.0 ns **-82%** |

## m4_mul_x16

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `m4_mul_x16` | 4123.0 ns | 4421.0 ns | 4315.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `m4_transform_x64` | 5804.0 ns | 6105.0 ns | 6017.0 ns |

## quat_mul

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `quat_mul` | 1461.0 ns | 1468.0 ns | 1468.0 ns |

## quat_slerp

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `quat_slerp` | 1456.0 ns | 1759.0 ns +21% | 270.0 ns **-81%** |

## quat_rotate_vec3

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 1465.0 ns | 1467.0 ns | 72.00 ns **-95%** |

## m4_mul

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `m4_mul` | 1614.0 ns | 1535.0 ns | 177.0 ns **-89%** |

## m4_inverse

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `m4_inverse` | 1602.0 ns | 1738.0 ns | 297.0 ns **-81%** |

## m4_transform_point

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `m4_transform_point` | 1581.0 ns | 1515.0 ns | 165.0 ns **-90%** |

## t3d_compose

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `t3d_compose` | 1493.0 ns | 1744.0 ns +17% | 272.0 ns **-82%** |

## ray_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `ray_sphere` | 1466.0 ns | 1463.0 ns | 95.00 ns **-94%** |

## ray_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `ray_aabb` | 1474.0 ns | 1459.0 ns | 83.00 ns **-94%** |

## ray_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `ray_triangle` | 1554.0 ns | 1735.0 ns +12% | 285.0 ns **-82%** |

## srgb_to_linear

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `srgb_to_linear` | 1438.0 ns | 1456.0 ns | 85.00 ns **-94%** |

## tonemap_reinhard

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 1444.0 ns | 1462.0 ns | 38.00 ns **-97%** |

## calc_derivative

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `calc_derivative` | 1431.0 ns | 1457.0 ns | 94.00 ns **-93%** |

## calc_integral_simpson

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 6812.0 ns | 6695.0 ns | 6625.0 ns |

## num_gcd

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `num_gcd` | 1439.0 ns | 1430.0 ns | 26.00 ns **-98%** |

## num_is_prime

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `num_is_prime` | 21902.0 ns | 21189.0 ns | 20842.0 ns |

## cx_mul

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `cx_mul` | 1431.0 ns | 1458.0 ns | 1450.0 ns |

## ease_in_out

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `ease_in_out` | 1420.0 ns | 1358.0 ns | 1351.0 ns |

## perlin_2d

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `perlin_2d` | 1424.0 ns | 1457.0 ns | 1457.0 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 2032.0 µs | 1949.0 µs | 1993.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 1095.0 µs | 1064.0 µs | 1069.0 µs |

## bvh_degenerate_4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3795.0 µs | 3538.0 µs | 3606.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1893.0 µs | 1807.0 µs | 1832.0 µs |

## kdtree_build_4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2687.0 µs | 2537.0 µs | 2609.0 µs |

## kdtree_radius_4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1478.0 ns | 1971.0 ns +33% | 1996.0 ns +35% |

## delaunay_2d_400

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1784.0 µs | 1733.0 µs | 1745.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 450034.0 ns | 482836.0 ns | 467959.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1703.0 µs | 1636.0 µs | 1666.0 µs |

## svd_golub_kahan_12

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 172046.0 ns | 169267.0 ns | 170978.0 ns |

## eigen_qr_12

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `eigen_qr_12` | 133868.0 ns | 128088.0 ns | 128640.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 133657.0 ns | 127348.0 ns | 128984.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 25760.0 ns | 24390.0 ns | 24188.0 ns |

## gjk_epa_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 25930.0 ns | 24961.0 ns | 25831.0 ns |

## gjk_epa_spheres

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 619640.0 ns | 605960.0 ns | 596625.0 ns |

## gjk_epa_sphere_box

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 124718.0 ns | 142910.0 ns +15% | 122809.0 ns |

## gjk_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 840.0 ns | 778.0 ns | 764.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 1182.0 ns | 1120.0 ns | 1101.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1989.0 ns | 1857.0 ns | 1898.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 2262.0 ns | 2100.0 ns | 2100.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1790.0 ns | 1659.0 ns | 1642.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 6340.0 ns | 6017.0 ns | 5943.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 6519.0 ns | 6148.0 ns | 6128.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 6348.0 µs | 6098.0 µs | 6126.0 µs |

## spatial_hash_query_2k

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 543560.0 ns | 571469.0 ns | 541898.0 ns |

## num_dct_1024

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `num_dct_1024` | 293969.0 ns | 291950.0 ns | 287515.0 ns |

## num_dst_1024

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `num_dst_1024` | 7095.0 µs | 6735.0 µs | 6707.0 µs |

## num_dct_1023

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `num_dct_1023` | 1723.0 µs | 1631.0 µs | 1624.0 µs |

## num_dst_1023

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `num_dst_1023` | 475025.0 ns | 461588.0 ns | 460694.0 ns |

## kdtree_build_octave_512

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | — | 567443.0 ns | 569674.0 ns |

## triangulate_comb_600

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `triangulate_comb_600` | — | 5717.0 µs | 6010.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `triangulate_hex_6` | — | 2977.0 ns | 3059.0 ns |

## jet_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `jet_sphere` | — | — | 336.0 ns |

## jet_plane

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `jet_plane` | — | — | 159.0 ns |

## ray_capsule

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `ray_capsule` | — | — | 500.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | — | — | 2568.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`c59e1fc`) | Current (`b0783ad`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | — | — | 1927.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
