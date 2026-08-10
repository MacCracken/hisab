# Benchmarks

Latest: **2026-08-10T14:39:42Z** — commit `9427003`

Tracking: `891d83e` (baseline) → `2c62720` (mid) → `9427003` (current)

## vec3_add

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `vec3_add` | 1447.0 ns | 1428.0 ns | 1407.0 ns |

## vec3_cross

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `vec3_cross` | 1463.0 ns | 1460.0 ns | 1465.0 ns |

## vec3_normalize

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `vec3_normalize` | 1457.0 ns | 1466.0 ns | 1462.0 ns |

## vec3_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 1881.0 ns | 2033.0 ns | 1954.0 ns |

## vec4_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 1695.0 ns | 1765.0 ns | 1706.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `m4_mul_x16` | 4123.0 ns | 4474.0 ns | 4216.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `m4_transform_x64` | 5804.0 ns | 6061.0 ns | 5799.0 ns |

## quat_mul

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `quat_mul` | 1461.0 ns | 1471.0 ns | 1467.0 ns |

## quat_slerp

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `quat_slerp` | 1456.0 ns | 1748.0 ns +20% | 1704.0 ns +17% |

## quat_rotate_vec3

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 1465.0 ns | 1466.0 ns | 1467.0 ns |

## m4_mul

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `m4_mul` | 1614.0 ns | 1544.0 ns | 1527.0 ns |

## m4_inverse

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `m4_inverse` | 1602.0 ns | 1759.0 ns | 1733.0 ns |

## m4_transform_point

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `m4_transform_point` | 1581.0 ns | 1531.0 ns | 1513.0 ns |

## t3d_compose

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `t3d_compose` | 1493.0 ns | 1754.0 ns +17% | 1745.0 ns +17% |

## ray_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `ray_sphere` | 1466.0 ns | 1471.0 ns | 1465.0 ns |

## ray_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `ray_aabb` | 1474.0 ns | 1457.0 ns | 1455.0 ns |

## ray_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `ray_triangle` | 1554.0 ns | 1693.0 ns | 1729.0 ns +11% |

## srgb_to_linear

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `srgb_to_linear` | 1438.0 ns | 1460.0 ns | 1453.0 ns |

## tonemap_reinhard

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 1444.0 ns | 1464.0 ns | 1420.0 ns |

## calc_derivative

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `calc_derivative` | 1431.0 ns | 1458.0 ns | 1453.0 ns |

## calc_integral_simpson

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 6812.0 ns | 6771.0 ns | 6483.0 ns |

## num_gcd

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `num_gcd` | 1439.0 ns | 1442.0 ns | 1327.0 ns |

## num_is_prime

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `num_is_prime` | 21902.0 ns | 21364.0 ns | 20613.0 ns |

## cx_mul

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `cx_mul` | 1431.0 ns | 1463.0 ns | 1453.0 ns |

## ease_in_out

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `ease_in_out` | 1420.0 ns | 1376.0 ns | 1341.0 ns |

## perlin_2d

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `perlin_2d` | 1424.0 ns | 1459.0 ns | 1455.0 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 2032.0 µs | 1941.0 µs | 1869.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 1095.0 µs | 1045.0 µs | 1004.0 µs |

## bvh_degenerate_4k

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3795.0 µs | 3576.0 µs | 3468.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1893.0 µs | 1867.0 µs | 1742.0 µs |

## kdtree_build_4k

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2687.0 µs | 2525.0 µs | 2439.0 µs |

## kdtree_radius_4k

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1478.0 ns | 1989.0 ns +35% | 1959.0 ns +33% |

## delaunay_2d_400

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1784.0 µs | 1732.0 µs | 1672.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 450034.0 ns | 451779.0 ns | 425170.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1703.0 µs | 1692.0 µs | 1584.0 µs |

## svd_golub_kahan_12

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 172046.0 ns | 169047.0 ns | 164511.0 ns |

## eigen_qr_12

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `eigen_qr_12` | 133868.0 ns | 131224.0 ns | 124300.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 133657.0 ns | 129358.0 ns | 124641.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 25760.0 ns | 24650.0 ns | 23173.0 ns **-10%** |

## gjk_epa_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 25930.0 ns | 24096.0 ns | 22939.0 ns **-12%** |

## gjk_epa_spheres

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 619640.0 ns | 598242.0 ns | 568878.0 ns |

## gjk_epa_sphere_box

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 124718.0 ns | 122995.0 ns | 118188.0 ns |

## gjk_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 840.0 ns | 780.0 ns | 756.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 1182.0 ns | 1112.0 ns | 1077.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1989.0 ns | 1883.0 ns | 1823.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 2262.0 ns | 2145.0 ns | 2072.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1790.0 ns | 1674.0 ns | 1633.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 6340.0 ns | 6095.0 ns | 5920.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 6519.0 ns | 6402.0 ns | 6074.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 6348.0 µs | 6285.0 µs | 5933.0 µs |

## spatial_hash_query_2k

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 543560.0 ns | 532581.0 ns | 510386.0 ns |

## num_dct_1024

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `num_dct_1024` | 293969.0 ns | 293494.0 ns | 275489.0 ns |

## num_dst_1024

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `num_dst_1024` | 7095.0 µs | 6928.0 µs | 6612.0 µs |

## num_dct_1023

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `num_dct_1023` | 1723.0 µs | 1642.0 µs | 1604.0 µs |

## num_dst_1023

| Benchmark | Baseline (`891d83e`) | Mid (`2c62720`) | Current (`9427003`) |
|-----------|------|------|------|
| `num_dst_1023` | 475025.0 ns | 468250.0 ns | 457285.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
