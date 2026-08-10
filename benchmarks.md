# Benchmarks

Latest: **2026-08-10T14:54:18Z** — commit `aa9febe`

Tracking: `891d83e` (baseline) → `9427003` (mid) → `aa9febe` (current)

## vec3_add

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `vec3_add` | 1447.0 ns | 1407.0 ns | 1449.0 ns |

## vec3_cross

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `vec3_cross` | 1463.0 ns | 1465.0 ns | 1463.0 ns |

## vec3_normalize

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `vec3_normalize` | 1457.0 ns | 1462.0 ns | 1467.0 ns |

## vec3_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 1881.0 ns | 1954.0 ns | 1987.0 ns |

## vec4_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 1695.0 ns | 1706.0 ns | 1738.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `m4_mul_x16` | 4123.0 ns | 4216.0 ns | 4328.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `m4_transform_x64` | 5804.0 ns | 5799.0 ns | 5859.0 ns |

## quat_mul

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `quat_mul` | 1461.0 ns | 1467.0 ns | 1472.0 ns |

## quat_slerp

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `quat_slerp` | 1456.0 ns | 1704.0 ns +17% | 1718.0 ns +18% |

## quat_rotate_vec3

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 1465.0 ns | 1467.0 ns | 1465.0 ns |

## m4_mul

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `m4_mul` | 1614.0 ns | 1527.0 ns | 1530.0 ns |

## m4_inverse

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `m4_inverse` | 1602.0 ns | 1733.0 ns | 1735.0 ns |

## m4_transform_point

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `m4_transform_point` | 1581.0 ns | 1513.0 ns | 1511.0 ns |

## t3d_compose

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `t3d_compose` | 1493.0 ns | 1745.0 ns +17% | 1741.0 ns +17% |

## ray_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `ray_sphere` | 1466.0 ns | 1465.0 ns | 1463.0 ns |

## ray_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `ray_aabb` | 1474.0 ns | 1455.0 ns | 1453.0 ns |

## ray_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `ray_triangle` | 1554.0 ns | 1729.0 ns +11% | 1734.0 ns +12% |

## srgb_to_linear

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `srgb_to_linear` | 1438.0 ns | 1453.0 ns | 1450.0 ns |

## tonemap_reinhard

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 1444.0 ns | 1420.0 ns | 1439.0 ns |

## calc_derivative

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `calc_derivative` | 1431.0 ns | 1453.0 ns | 1452.0 ns |

## calc_integral_simpson

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 6812.0 ns | 6483.0 ns | 6535.0 ns |

## num_gcd

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `num_gcd` | 1439.0 ns | 1327.0 ns | 1412.0 ns |

## num_is_prime

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `num_is_prime` | 21902.0 ns | 20613.0 ns | 21037.0 ns |

## cx_mul

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `cx_mul` | 1431.0 ns | 1453.0 ns | 1454.0 ns |

## ease_in_out

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `ease_in_out` | 1420.0 ns | 1341.0 ns | 1343.0 ns |

## perlin_2d

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `perlin_2d` | 1424.0 ns | 1455.0 ns | 1452.0 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 2032.0 µs | 1869.0 µs | 1973.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 1095.0 µs | 1004.0 µs | 1045.0 µs |

## bvh_degenerate_4k

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3795.0 µs | 3468.0 µs | 3538.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1893.0 µs | 1742.0 µs | 1776.0 µs |

## kdtree_build_4k

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2687.0 µs | 2439.0 µs | 2498.0 µs |

## kdtree_radius_4k

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1478.0 ns | 1959.0 ns +33% | 1924.0 ns +30% |

## delaunay_2d_400

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1784.0 µs | 1672.0 µs | 1723.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 450034.0 ns | 425170.0 ns | 433598.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1703.0 µs | 1584.0 µs | 1643.0 µs |

## svd_golub_kahan_12

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 172046.0 ns | 164511.0 ns | 165499.0 ns |

## eigen_qr_12

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `eigen_qr_12` | 133868.0 ns | 124300.0 ns | 122449.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 133657.0 ns | 124641.0 ns | 127487.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 25760.0 ns | 23173.0 ns **-10%** | 24276.0 ns |

## gjk_epa_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 25930.0 ns | 22939.0 ns **-12%** | 24180.0 ns |

## gjk_epa_spheres

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 619640.0 ns | 568878.0 ns | 583239.0 ns |

## gjk_epa_sphere_box

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 124718.0 ns | 118188.0 ns | 120301.0 ns |

## gjk_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 840.0 ns | 756.0 ns | 782.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 1182.0 ns | 1077.0 ns | 1097.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1989.0 ns | 1823.0 ns | 1878.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 2262.0 ns | 2072.0 ns | 2156.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1790.0 ns | 1633.0 ns | 1675.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 6340.0 ns | 5920.0 ns | 5939.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 6519.0 ns | 6074.0 ns | 6084.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 6348.0 µs | 5933.0 µs | 6020.0 µs |

## spatial_hash_query_2k

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 543560.0 ns | 510386.0 ns | 546242.0 ns |

## num_dct_1024

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `num_dct_1024` | 293969.0 ns | 275489.0 ns | 291887.0 ns |

## num_dst_1024

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `num_dst_1024` | 7095.0 µs | 6612.0 µs | 6696.0 µs |

## num_dct_1023

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `num_dct_1023` | 1723.0 µs | 1604.0 µs | 1614.0 µs |

## num_dst_1023

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `num_dst_1023` | 475025.0 ns | 457285.0 ns | 449337.0 ns |

## kdtree_build_octave_512

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | — | — | 563783.0 ns |

## triangulate_comb_600

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `triangulate_comb_600` | — | — | 5767.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`891d83e`) | Mid (`9427003`) | Current (`aa9febe`) |
|-----------|------|------|------|
| `triangulate_hex_6` | — | — | 2950.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
