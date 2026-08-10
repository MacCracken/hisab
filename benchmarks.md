# Benchmarks

Latest: **2026-08-10T15:23:14Z** — commit `882023d`

Tracking: `891d83e` (baseline) → `aa9febe` (mid) → `882023d` (current)

## vec3_add

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `vec3_add` | 1447.0 ns | 1384.0 ns | 1457.0 ns |

## vec3_cross

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `vec3_cross` | 1463.0 ns | 1463.0 ns | 1469.0 ns |

## vec3_normalize

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `vec3_normalize` | 1457.0 ns | 1459.0 ns | 1467.0 ns |

## vec3_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 1881.0 ns | 1948.0 ns | 2034.0 ns |

## vec4_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 1695.0 ns | 1691.0 ns | 1743.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `m4_mul_x16` | 4123.0 ns | 4218.0 ns | 4414.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `m4_transform_x64` | 5804.0 ns | 5735.0 ns | 5916.0 ns |

## quat_mul

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `quat_mul` | 1461.0 ns | 1467.0 ns | 1469.0 ns |

## quat_slerp

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `quat_slerp` | 1456.0 ns | 1709.0 ns +17% | 1727.0 ns +19% |

## quat_rotate_vec3

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 1465.0 ns | 1465.0 ns | 1466.0 ns |

## m4_mul

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `m4_mul` | 1614.0 ns | 1522.0 ns | 1529.0 ns |

## m4_inverse

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `m4_inverse` | 1602.0 ns | 1730.0 ns | 1765.0 ns +10% |

## m4_transform_point

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `m4_transform_point` | 1581.0 ns | 1505.0 ns | 1510.0 ns |

## t3d_compose

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `t3d_compose` | 1493.0 ns | 1741.0 ns +17% | 1745.0 ns +17% |

## ray_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `ray_sphere` | 1466.0 ns | 1464.0 ns | 1465.0 ns |

## ray_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `ray_aabb` | 1474.0 ns | 1444.0 ns | 1455.0 ns |

## ray_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `ray_triangle` | 1554.0 ns | 1729.0 ns +11% | 1733.0 ns +12% |

## srgb_to_linear

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `srgb_to_linear` | 1438.0 ns | 1449.0 ns | 1455.0 ns |

## tonemap_reinhard

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 1444.0 ns | 1432.0 ns | 1456.0 ns |

## calc_derivative

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `calc_derivative` | 1431.0 ns | 1455.0 ns | 1456.0 ns |

## calc_integral_simpson

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 6812.0 ns | 6611.0 ns | 6939.0 ns |

## num_gcd

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `num_gcd` | 1439.0 ns | 1363.0 ns | 1445.0 ns |

## num_is_prime

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `num_is_prime` | 21902.0 ns | 20505.0 ns | 21220.0 ns |

## cx_mul

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `cx_mul` | 1431.0 ns | 1453.0 ns | 1459.0 ns |

## ease_in_out

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `ease_in_out` | 1420.0 ns | 1339.0 ns | 1361.0 ns |

## perlin_2d

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `perlin_2d` | 1424.0 ns | 1454.0 ns | 1456.0 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 2032.0 µs | 1878.0 µs | 1944.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 1095.0 µs | 995185.0 ns | 1050.0 µs |

## bvh_degenerate_4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3795.0 µs | 3502.0 µs | 3785.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1893.0 µs | 1792.0 µs | 1866.0 µs |

## kdtree_build_4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2687.0 µs | 2482.0 µs | 2532.0 µs |

## kdtree_radius_4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1478.0 ns | 1902.0 ns +29% | 1915.0 ns +30% |

## delaunay_2d_400

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1784.0 µs | 1676.0 µs | 1793.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 450034.0 ns | 452129.0 ns | 454247.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1703.0 µs | 1593.0 µs | 1636.0 µs |

## svd_golub_kahan_12

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 172046.0 ns | 165523.0 ns | 169396.0 ns |

## eigen_qr_12

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `eigen_qr_12` | 133868.0 ns | 126252.0 ns | 128417.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 133657.0 ns | 127379.0 ns | 127076.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 25760.0 ns | 23187.0 ns | 24735.0 ns |

## gjk_epa_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 25930.0 ns | 23221.0 ns **-10%** | 24170.0 ns |

## gjk_epa_spheres

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 619640.0 ns | 585022.0 ns | 585690.0 ns |

## gjk_epa_sphere_box

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 124718.0 ns | 120175.0 ns | 121350.0 ns |

## gjk_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 840.0 ns | 802.0 ns | 774.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 1182.0 ns | 1135.0 ns | 1130.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1989.0 ns | 1857.0 ns | 1877.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 2262.0 ns | 2125.0 ns | 2151.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1790.0 ns | 1644.0 ns | 1719.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 6340.0 ns | 6069.0 ns | 6178.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 6519.0 ns | 6209.0 ns | 6194.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 6348.0 µs | 5951.0 µs | 6105.0 µs |

## spatial_hash_query_2k

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 543560.0 ns | 523502.0 ns | 553631.0 ns |

## num_dct_1024

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `num_dct_1024` | 293969.0 ns | 286377.0 ns | 291922.0 ns |

## num_dst_1024

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `num_dst_1024` | 7095.0 µs | 6617.0 µs | 6684.0 µs |

## num_dct_1023

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `num_dct_1023` | 1723.0 µs | 1595.0 µs | 1650.0 µs |

## num_dst_1023

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `num_dst_1023` | 475025.0 ns | 448562.0 ns | 462586.0 ns |

## triangulate_comb_600

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `triangulate_comb_600` | — | 5718.0 µs | 5783.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `triangulate_hex_6` | — | 2906.0 ns | 2972.0 ns |

## kdtree_build_octave_512

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | — | — | 565613.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | — | — | 2534.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`aa9febe`) | Current (`882023d`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | — | — | 1893.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
