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

Latest: **2026-08-11T00:50:58Z** — commit `f9db51b`

Tracking: `891d83e` (baseline) → `882023d` (mid) → `f9db51b` (current)

## vec3_add

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `vec3_add` | 1447.0 ns | 1457.0 ns | 38.00 ns **-97%** |

## vec3_cross

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `vec3_cross` | 1463.0 ns | 1469.0 ns | 71.00 ns **-95%** |

## vec3_normalize

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `vec3_normalize` | 1457.0 ns | 1467.0 ns | 49.00 ns **-97%** |

## vec3_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 1881.0 ns | 2034.0 ns | 2023.0 ns |

## vec4_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 1695.0 ns | 1743.0 ns | 311.0 ns **-82%** |

## m4_mul_x16

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `m4_mul_x16` | 4123.0 ns | 4414.0 ns | 4343.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `m4_transform_x64` | 5804.0 ns | 5916.0 ns | 6045.0 ns |

## quat_mul

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `quat_mul` | 1461.0 ns | 1469.0 ns | 1465.0 ns |

## quat_slerp

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `quat_slerp` | 1456.0 ns | 1727.0 ns +19% | 274.0 ns **-81%** |

## quat_rotate_vec3

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 1465.0 ns | 1466.0 ns | 73.00 ns **-95%** |

## m4_mul

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `m4_mul` | 1614.0 ns | 1529.0 ns | 180.0 ns **-89%** |

## m4_inverse

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `m4_inverse` | 1602.0 ns | 1765.0 ns +10% | 299.0 ns **-81%** |

## m4_transform_point

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `m4_transform_point` | 1581.0 ns | 1510.0 ns | 165.0 ns **-90%** |

## t3d_compose

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `t3d_compose` | 1493.0 ns | 1745.0 ns +17% | 272.0 ns **-82%** |

## ray_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `ray_sphere` | 1466.0 ns | 1465.0 ns | 97.00 ns **-93%** |

## ray_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `ray_aabb` | 1474.0 ns | 1455.0 ns | 91.00 ns **-94%** |

## ray_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `ray_triangle` | 1554.0 ns | 1733.0 ns +12% | 293.0 ns **-81%** |

## srgb_to_linear

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `srgb_to_linear` | 1438.0 ns | 1455.0 ns | 87.00 ns **-94%** |

## tonemap_reinhard

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 1444.0 ns | 1456.0 ns | 39.00 ns **-97%** |

## calc_derivative

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `calc_derivative` | 1431.0 ns | 1456.0 ns | 97.00 ns **-93%** |

## calc_integral_simpson

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 6812.0 ns | 6939.0 ns | 6717.0 ns |

## num_gcd

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `num_gcd` | 1439.0 ns | 1445.0 ns | 26.00 ns **-98%** |

## num_is_prime

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `num_is_prime` | 21902.0 ns | 21220.0 ns | 21398.0 ns |

## cx_mul

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `cx_mul` | 1431.0 ns | 1459.0 ns | 1454.0 ns |

## ease_in_out

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `ease_in_out` | 1420.0 ns | 1361.0 ns | 1378.0 ns |

## perlin_2d

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `perlin_2d` | 1424.0 ns | 1456.0 ns | 1455.0 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 2032.0 µs | 1944.0 µs | 2002.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 1095.0 µs | 1050.0 µs | 1104.0 µs |

## bvh_degenerate_4k

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3795.0 µs | 3785.0 µs | 3688.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1893.0 µs | 1866.0 µs | 1921.0 µs |

## kdtree_build_4k

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2687.0 µs | 2532.0 µs | 2586.0 µs |

## kdtree_radius_4k

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1478.0 ns | 1915.0 ns +30% | 1975.0 ns +34% |

## delaunay_2d_400

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1784.0 µs | 1793.0 µs | 1815.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 450034.0 ns | 454247.0 ns | 463048.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1703.0 µs | 1636.0 µs | 1704.0 µs |

## svd_golub_kahan_12

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 172046.0 ns | 169396.0 ns | 173098.0 ns |

## eigen_qr_12

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `eigen_qr_12` | 133868.0 ns | 128417.0 ns | 131766.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 133657.0 ns | 127076.0 ns | 131401.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 25760.0 ns | 24735.0 ns | 24870.0 ns |

## gjk_epa_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 25930.0 ns | 24170.0 ns | 24643.0 ns |

## gjk_epa_spheres

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 619640.0 ns | 585690.0 ns | 598035.0 ns |

## gjk_epa_sphere_box

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 124718.0 ns | 121350.0 ns | 123614.0 ns |

## gjk_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 840.0 ns | 774.0 ns | 781.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 1182.0 ns | 1130.0 ns | 1126.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1989.0 ns | 1877.0 ns | 1898.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 2262.0 ns | 2151.0 ns | 2182.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1790.0 ns | 1719.0 ns | 1701.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 6340.0 ns | 6178.0 ns | 6200.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 6519.0 ns | 6194.0 ns | 6236.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 6348.0 µs | 6105.0 µs | 6310.0 µs |

## spatial_hash_query_2k

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 543560.0 ns | 553631.0 ns | 572098.0 ns |

## num_dct_1024

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `num_dct_1024` | 293969.0 ns | 291922.0 ns | 296650.0 ns |

## num_dst_1024

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `num_dst_1024` | 7095.0 µs | 6684.0 µs | 6923.0 µs |

## num_dct_1023

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `num_dct_1023` | 1723.0 µs | 1650.0 µs | 1658.0 µs |

## num_dst_1023

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `num_dst_1023` | 475025.0 ns | 462586.0 ns | 470974.0 ns |

## kdtree_build_octave_512

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | — | 565613.0 ns | 592910.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | — | 2534.0 ns | 2567.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | — | 1893.0 ns | 1910.0 ns |

## triangulate_comb_600

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `triangulate_comb_600` | — | 5783.0 µs | 6051.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `triangulate_hex_6` | — | 2972.0 ns | 3033.0 ns |

## jet_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `jet_sphere` | — | — | 355.0 ns |

## jet_plane

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `jet_plane` | — | — | 240.0 ns |

## jet_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `jet_triangle` | — | — | 992.0 ns |

## jet_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `jet_aabb` | — | — | 351.0 ns |

## jet_obb

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `jet_obb` | — | — | 864.0 ns |

## jet_capsule

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `jet_capsule` | — | — | 1327.0 ns |

## ray_obb

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `ray_obb` | — | — | 492.0 ns |

## ray_aabb_diag

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `ray_aabb_diag` | — | — | 115.0 ns |

## ray_capsule_diag

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `ray_capsule_diag` | — | — | 953.0 ns |

## ray_capsule

| Benchmark | Baseline (`891d83e`) | Mid (`882023d`) | Current (`f9db51b`) |
|-----------|------|------|------|
| `ray_capsule` | — | — | 507.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
