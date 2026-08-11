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

Latest: **2026-08-11T15:22:17Z** — commit `6f43da3`

Tracking: `891d83e` (baseline) → `a5ef9b3` (mid) → `6f43da3` (current)

## vec3_add

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `vec3_add` | 1447.0 ns | 35.00 ns **-98%** | 38.00 ns **-97%** |

## vec3_cross

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `vec3_cross` | 1463.0 ns | 72.00 ns **-95%** | 71.00 ns **-95%** |

## vec3_normalize

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `vec3_normalize` | 1457.0 ns | 51.00 ns **-96%** | 47.00 ns **-97%** |

## vec3_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 1881.0 ns | 2051.0 ns | 2035.0 ns |

## vec4_dot_x64

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 1695.0 ns | 301.0 ns **-82%** | 310.0 ns **-82%** |

## m4_mul_x16

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `m4_mul_x16` | 4123.0 ns | 4428.0 ns | 4365.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `m4_transform_x64` | 5804.0 ns | 6005.0 ns | 6289.0 ns |

## quat_mul

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `quat_mul` | 1461.0 ns | 1469.0 ns | 1473.0 ns |

## quat_slerp

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `quat_slerp` | 1456.0 ns | 273.0 ns **-81%** | 285.0 ns **-80%** |

## quat_rotate_vec3

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 1465.0 ns | 73.00 ns **-95%** | 77.00 ns **-95%** |

## m4_mul

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `m4_mul` | 1614.0 ns | 179.0 ns **-89%** | 188.0 ns **-88%** |

## m4_inverse

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `m4_inverse` | 1602.0 ns | 298.0 ns **-81%** | 311.0 ns **-81%** |

## m4_transform_point

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `m4_transform_point` | 1581.0 ns | 162.0 ns **-90%** | 170.0 ns **-89%** |

## t3d_compose

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `t3d_compose` | 1493.0 ns | 277.0 ns **-81%** | 285.0 ns **-81%** |

## ray_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `ray_sphere` | 1466.0 ns | 99.00 ns **-93%** | 107.0 ns **-93%** |

## ray_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `ray_aabb` | 1474.0 ns | 86.00 ns **-94%** | 98.00 ns **-93%** |

## ray_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `ray_triangle` | 1554.0 ns | 300.0 ns **-81%** | 302.0 ns **-81%** |

## srgb_to_linear

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `srgb_to_linear` | 1438.0 ns | 88.00 ns **-94%** | 91.00 ns **-94%** |

## tonemap_reinhard

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 1444.0 ns | 39.00 ns **-97%** | 41.00 ns **-97%** |

## calc_derivative

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `calc_derivative` | 1431.0 ns | 97.00 ns **-93%** | 101.0 ns **-93%** |

## calc_integral_simpson

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 6812.0 ns | 6728.0 ns | 6958.0 ns |

## num_gcd

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `num_gcd` | 1439.0 ns | 28.00 ns **-98%** | 27.00 ns **-98%** |

## num_is_prime

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `num_is_prime` | 21902.0 ns | 22104.0 ns | 22143.0 ns |

## cx_mul

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `cx_mul` | 1431.0 ns | 1462.0 ns | 1461.0 ns |

## ease_in_out

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `ease_in_out` | 1420.0 ns | 1413.0 ns | 1412.0 ns |

## perlin_2d

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `perlin_2d` | 1424.0 ns | 1458.0 ns | 1439.0 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 2032.0 µs | 2043.0 µs | 2044.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 1095.0 µs | 1124.0 µs | 1130.0 µs |

## bvh_degenerate_4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3795.0 µs | 3853.0 µs | 3828.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1893.0 µs | 1920.0 µs | 1936.0 µs |

## kdtree_build_4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2687.0 µs | 2684.0 µs | 2686.0 µs |

## kdtree_radius_4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1478.0 ns | 1887.0 ns +28% | 2063.0 ns +40% |

## delaunay_2d_400

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1784.0 µs | 1812.0 µs | 1813.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 450034.0 ns | 460230.0 ns | 461534.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1703.0 µs | 1723.0 µs | 1738.0 µs |

## svd_golub_kahan_12

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 172046.0 ns | 177375.0 ns | 175556.0 ns |

## eigen_qr_12

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `eigen_qr_12` | 133868.0 ns | 134999.0 ns | 133767.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 133657.0 ns | 134169.0 ns | 134755.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 25760.0 ns | 25553.0 ns | 25418.0 ns |

## gjk_epa_boxes

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 25930.0 ns | 25580.0 ns | 25661.0 ns |

## gjk_epa_spheres

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 619640.0 ns | 616500.0 ns | 619952.0 ns |

## gjk_epa_sphere_box

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 124718.0 ns | 126886.0 ns | 127961.0 ns |

## gjk_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 840.0 ns | 801.0 ns | 816.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 1182.0 ns | 1159.0 ns | 1179.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1989.0 ns | 1961.0 ns | 1964.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 2262.0 ns | 2231.0 ns | 2236.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1790.0 ns | 1744.0 ns | 1764.0 ns |

## mpr_intersect_box_hit

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 6340.0 ns | 6299.0 ns | 6350.0 ns |

## mpr_intersect_tangent

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 6519.0 ns | 6460.0 ns | 6505.0 ns |

## bvh_scatter_4k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 6348.0 µs | 6457.0 µs | 6511.0 µs |

## spatial_hash_query_2k

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 543560.0 ns | 563842.0 ns | 637957.0 ns +17% |

## num_dct_1024

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `num_dct_1024` | 293969.0 ns | 298892.0 ns | 301658.0 ns |

## num_dst_1024

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `num_dst_1024` | 7095.0 µs | 7100.0 µs | 7205.0 µs |

## num_dct_1023

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `num_dct_1023` | 1723.0 µs | 1727.0 µs | 1755.0 µs |

## num_dst_1023

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `num_dst_1023` | 475025.0 ns | 482589.0 ns | 484858.0 ns |

## kdtree_build_octave_512

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | — | 604968.0 ns | 605174.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | — | 2683.0 ns | 2688.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | — | 1974.0 ns | 1974.0 ns |

## triangulate_comb_600

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `triangulate_comb_600` | — | 6136.0 µs | 6127.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `triangulate_hex_6` | — | 3125.0 ns | 3109.0 ns |

## jet_sphere

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `jet_sphere` | — | — | 370.0 ns |

## jet_plane

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `jet_plane` | — | — | 247.0 ns |

## jet_triangle

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `jet_triangle` | — | — | 1023.0 ns |

## jet_aabb

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `jet_aabb` | — | — | 367.0 ns |

## jet_obb

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `jet_obb` | — | — | 1054.0 ns |

## jet_capsule

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `jet_capsule` | — | — | 1298.0 ns |

## grad_fwd_16

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `grad_fwd_16` | — | — | 68811.0 ns |

## grad_rev_16

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `grad_rev_16` | — | — | 6179.0 ns |

## ray_obb

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `ray_obb` | — | — | 512.0 ns |

## ray_aabb_diag

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `ray_aabb_diag` | — | — | 123.0 ns |

## ray_capsule_diag

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `ray_capsule_diag` | — | — | 913.0 ns |

## ray_capsule

| Benchmark | Baseline (`891d83e`) | Mid (`a5ef9b3`) | Current (`6f43da3`) |
|-----------|------|------|------|
| `ray_capsule` | — | — | 669.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
