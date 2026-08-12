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

Latest: **2026-08-12T01:48:32Z** — commit `5d77b5f`

Tracking: `891d83e` (baseline) → `a5ef9b3` (mid) → `5d77b5f` (current)

## vec3_add

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `vec3_add` | 1447.0 ns | 35.00 ns **-98%** | 36.00 ns **-98%** |

## vec3_cross

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `vec3_cross` | 1463.0 ns | 72.00 ns **-95%** | 72.00 ns **-95%** |

## vec3_normalize

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `vec3_normalize` | 1457.0 ns | 51.00 ns **-96%** | 49.00 ns **-97%** |

## vec3_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 1881.0 ns | 2051.0 ns | 2034.0 ns |

## vec4_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 1695.0 ns | 301.0 ns **-82%** | 295.0 ns **-83%** |

## m4_mul_x16

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `m4_mul_x16` | 4123.0 ns | 4428.0 ns | 4621.0 ns +12% |

## m4_transform_x64

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `m4_transform_x64` | 5804.0 ns | 6005.0 ns | 6373.0 ns |

## quat_mul

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `quat_mul` | 1461.0 ns | 1469.0 ns | 1473.0 ns |

## quat_slerp

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `quat_slerp` | 1456.0 ns | 273.0 ns **-81%** | 278.0 ns **-81%** |

## quat_rotate_vec3

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 1465.0 ns | 73.00 ns **-95%** | 75.00 ns **-95%** |

## m4_mul

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `m4_mul` | 1614.0 ns | 179.0 ns **-89%** | 193.0 ns **-88%** |

## m4_inverse

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `m4_inverse` | 1602.0 ns | 298.0 ns **-81%** | 302.0 ns **-81%** |

## m4_transform_point

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `m4_transform_point` | 1581.0 ns | 162.0 ns **-90%** | 165.0 ns **-90%** |

## t3d_compose

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `t3d_compose` | 1493.0 ns | 277.0 ns **-81%** | 279.0 ns **-81%** |

## ray_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `ray_sphere` | 1466.0 ns | 99.00 ns **-93%** | 102.0 ns **-93%** |

## ray_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `ray_aabb` | 1474.0 ns | 86.00 ns **-94%** | 94.00 ns **-94%** |

## ray_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `ray_triangle` | 1554.0 ns | 300.0 ns **-81%** | 295.0 ns **-81%** |

## srgb_to_linear

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `srgb_to_linear` | 1438.0 ns | 88.00 ns **-94%** | 89.00 ns **-94%** |

## tonemap_reinhard

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 1444.0 ns | 39.00 ns **-97%** | 39.00 ns **-97%** |

## calc_derivative

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `calc_derivative` | 1431.0 ns | 97.00 ns **-93%** | 96.00 ns **-93%** |

## calc_integral_simpson

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 6812.0 ns | 6728.0 ns | 6819.0 ns |

## num_gcd

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `num_gcd` | 1439.0 ns | 28.00 ns **-98%** | 27.00 ns **-98%** |

## num_is_prime

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `num_is_prime` | 21902.0 ns | 22104.0 ns | 21569.0 ns |

## cx_mul

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `cx_mul` | 1431.0 ns | 1462.0 ns | 1459.0 ns |

## ease_in_out

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `ease_in_out` | 1420.0 ns | 1413.0 ns | 1407.0 ns |

## perlin_2d

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `perlin_2d` | 1424.0 ns | 1458.0 ns | 1461.0 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 2032.0 µs | 2043.0 µs | 2043.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 1095.0 µs | 1124.0 µs | 1136.0 µs |

## bvh_degenerate_4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3795.0 µs | 3853.0 µs | 3816.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1893.0 µs | 1920.0 µs | 1968.0 µs |

## kdtree_build_4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2687.0 µs | 2684.0 µs | 2670.0 µs |

## kdtree_radius_4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1478.0 ns | 1887.0 ns +28% | 1988.0 ns +35% |

## delaunay_2d_400

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1784.0 µs | 1812.0 µs | 1821.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 450034.0 ns | 460230.0 ns | 464934.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1703.0 µs | 1723.0 µs | 1771.0 µs |

## svd_golub_kahan_12

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 172046.0 ns | 177375.0 ns | 178364.0 ns |

## eigen_qr_12

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `eigen_qr_12` | 133868.0 ns | 134999.0 ns | 133892.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 133657.0 ns | 134169.0 ns | 135815.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 25760.0 ns | 25553.0 ns | 25507.0 ns |

## gjk_epa_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 25930.0 ns | 25580.0 ns | 24976.0 ns |

## gjk_epa_spheres

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 619640.0 ns | 616500.0 ns | 645702.0 ns |

## gjk_epa_sphere_box

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 124718.0 ns | 126886.0 ns | 128385.0 ns |

## gjk_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 840.0 ns | 801.0 ns | 816.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 1182.0 ns | 1159.0 ns | 1166.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1989.0 ns | 1961.0 ns | 1966.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 2262.0 ns | 2231.0 ns | 2264.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1790.0 ns | 1744.0 ns | 1759.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 6340.0 ns | 6299.0 ns | 6200.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 6519.0 ns | 6460.0 ns | 6143.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 6348.0 µs | 6457.0 µs | 6326.0 µs |

## spatial_hash_query_2k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 543560.0 ns | 563842.0 ns | 616754.0 ns +13% |

## num_dct_1024

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `num_dct_1024` | 293969.0 ns | 298892.0 ns | 288709.0 ns |

## num_dst_1024

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `num_dst_1024` | 7095.0 µs | 7100.0 µs | 6829.0 µs |

## num_dct_1023

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `num_dct_1023` | 1723.0 µs | 1727.0 µs | 1695.0 µs |

## num_dst_1023

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `num_dst_1023` | 475025.0 ns | 482589.0 ns | 473495.0 ns |

## kdtree_build_octave_512

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | — | 604968.0 ns | 602904.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | — | 2683.0 ns | 2679.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | — | 1974.0 ns | 1982.0 ns |

## triangulate_comb_600

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `triangulate_comb_600` | — | 6136.0 µs | 6163.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `triangulate_hex_6` | — | 3125.0 ns | 3134.0 ns |

## jet_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `jet_sphere` | — | — | 361.0 ns |

## jet_plane

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `jet_plane` | — | — | 239.0 ns |

## jet_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `jet_triangle` | — | — | 998.0 ns |

## jet_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `jet_aabb` | — | — | 364.0 ns |

## jet_obb

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `jet_obb` | — | — | 1038.0 ns |

## jet_capsule

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `jet_capsule` | — | — | 1269.0 ns |

## grad_fwd_16

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `grad_fwd_16` | — | — | 66522.0 ns |

## grad_rev_16

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `grad_rev_16` | — | — | 6151.0 ns |

## ray_obb

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `ray_obb` | — | — | 504.0 ns |

## ray_aabb_diag

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `ray_aabb_diag` | — | — | 120.0 ns |

## ray_capsule_diag

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `ray_capsule_diag` | — | — | 883.0 ns |

## ray_capsule

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`5d77b5f`) |
|-----------|------|------|------|
| `ray_capsule` | — | — | 641.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
