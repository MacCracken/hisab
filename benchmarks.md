# Benchmarks

> **Measurement changes** — read before comparing across a date.
> * **2026-08-21** (hisab 2.11.2, cyrius 6.5.18 → 6.5.33): `lib/bench.cyr` now
>   MEASURES one clock read on the host and subtracts it from every sample, and
>   `bench_run` sizes its own batches instead of wrapping a clock pair around every
>   iteration. **No hisab source changed.** Measured floor on this host: ~1,349 ns.
>   `ease_in_out` 1,407 ns → 7 ns, `cx_mul` 1,459 → 37, `quat_mul` 1,473 → 67,
>   `perlin_2d` 1,461 → 82 — those four were 94–100% instrument. The old numbers
>   also FLATTENED them: four operations spanning **149x** in reality were reported
>   within **1.26x** of each other, so a real regression had room to hide. 44 of 72
>   rows moved more than 10%; **none of it is a speedup**. Rows carry `regime=net`
>   from this date and the trend filter refuses to mix them with `raw` ones.
> * **2026-08-10**: 17 sub-microsecond benchmarks moved from `bench()` to
>   `bench_batch()`. `bench_run` wraps a `clock_gettime` PAIR around every call
>   (~240 ns, documented in `lib/bench.cyr`), so those rows were **~95% clock
>   overhead**: `ray_sphere` read 1,466 ns and is 79 ns; `vec3_add` read 1,457 ns
>   and is 36 ns. It also FLATTENED them — triangle/sphere measured 1.19x when the
>   true ratio is 3.6x — so a real regression could hide inside the overhead.
>   Their drop at this date is an artefact of the fix, not a speedup.
> * **2026-08-09**: `estimate_ns` changed from each benchmark's MAX to its AVG.
>   The `stat` column records which; rows with no value there are `max`.

Latest: **2026-08-21T17:57:26Z** — commit `a09d228`

## vec3_add

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `vec3_add` | 23.00 ns |

## vec3_cross

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `vec3_cross` | 63.00 ns |

## vec3_normalize

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `vec3_normalize` | 40.00 ns |

## vec3_dot_x64

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `vec3_dot_x64` | 593.0 ns |

## vec4_dot_x64

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `vec4_dot_x64` | 325.0 ns |

## m4_mul_x16

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `m4_mul_x16` | 2170.0 ns |

## m4_transform_x64

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `m4_transform_x64` | 3263.0 ns |

## quat_mul

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `quat_mul` | 63.00 ns |

## quat_slerp

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `quat_slerp` | 257.0 ns |

## quat_rotate_vec3

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `quat_rotate_vec3` | 63.00 ns |

## m4_mul

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `m4_mul` | 132.0 ns |

## m4_inverse

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `m4_inverse` | 259.0 ns |

## m4_transform_point

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `m4_transform_point` | 122.0 ns |

## t3d_compose

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `t3d_compose` | 217.0 ns |

## jet_sphere

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `jet_sphere` | 271.0 ns |

## jet_plane

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `jet_plane` | 173.0 ns |

## jet_triangle

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `jet_triangle` | 772.0 ns |

## jet_aabb

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `jet_aabb` | 291.0 ns |

## jet_obb

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `jet_obb` | 819.0 ns |

## jet_capsule

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `jet_capsule` | 991.0 ns |

## grad_fwd_16

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `grad_fwd_16` | 58180.0 ns |

## grad_rev_16

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `grad_rev_16` | 5856.0 ns |

## ray_obb

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `ray_obb` | 443.0 ns |

## ray_aabb_diag

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `ray_aabb_diag` | 115.0 ns |

## ray_capsule_diag

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `ray_capsule_diag` | 740.0 ns |

## ray_capsule

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `ray_capsule` | 535.0 ns |

## ray_sphere

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `ray_sphere` | 90.00 ns |

## ray_aabb

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `ray_aabb` | 93.00 ns |

## ray_triangle

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `ray_triangle` | 246.0 ns |

## srgb_to_linear

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `srgb_to_linear` | 82.00 ns |

## tonemap_reinhard

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `tonemap_reinhard` | 30.00 ns |

## calc_derivative

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `calc_derivative` | 94.00 ns |

## calc_integral_simpson

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `calc_integral_simpson` | 5164.0 ns |

## num_gcd

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `num_gcd` | 25.00 ns |

## num_is_prime

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `num_is_prime` | 19829.0 ns |

## cx_mul

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `cx_mul` | 35.00 ns |

## ease_in_out

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `ease_in_out` | 7.00 ns |

## perlin_2d

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `perlin_2d` | 79.00 ns |

## convex_hull_2d_2k

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `convex_hull_2d_2k` | 1909.0 µs |

## halfedge_2k_tris

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `halfedge_2k_tris` | 827300.0 ns |

## bvh_degenerate_4k

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `bvh_degenerate_4k` | 3304.0 µs |

## bvh_query_ray_200x4k

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs |

## kdtree_build_4k

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `kdtree_build_4k` | 2371.0 µs |

## kdtree_build_octave_512

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `kdtree_build_octave_512` | 548166.0 ns |

## einsum_matmul_2x2

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `einsum_matmul_2x2` | 1102.0 ns |

## einsum_trace_2x2

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `einsum_trace_2x2` | 463.0 ns |

## kdtree_radius_4k

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `kdtree_radius_4k` | 1035.0 ns |

## delaunay_2d_400

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `delaunay_2d_400` | 1583.0 µs |

## delaunay_2d_circle_150

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `delaunay_2d_circle_150` | 383609.0 ns |

## triangulate_600gon

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `triangulate_600gon` | 1616.0 µs |

## triangulate_comb_600

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `triangulate_comb_600` | 5737.0 µs |

## triangulate_hex_6

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `triangulate_hex_6` | 1463.0 ns |

## svd_golub_kahan_12

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `svd_golub_kahan_12` | 158752.0 ns |

## eigen_qr_12

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `eigen_qr_12` | 119270.0 ns |

## gjk_epa_3d_cyl_box

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns |

## mpr_penetration_boxes

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `mpr_penetration_boxes` | 17377.0 ns |

## gjk_epa_boxes

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `gjk_epa_boxes` | 17450.0 ns |

## gjk_epa_spheres

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `gjk_epa_spheres` | 536604.0 ns |

## gjk_epa_sphere_box

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `gjk_epa_sphere_box` | 111531.0 ns |

## gjk_intersect_box_miss

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `gjk_intersect_box_miss` | 659.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `gjk_intersect_sph_miss` | 905.0 ns |

## gjk_intersect_box_hit

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `gjk_intersect_box_hit` | 1608.0 ns |

## gjk_intersect_tangent

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `gjk_intersect_tangent` | 1840.0 ns |

## mpr_intersect_box_miss

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `mpr_intersect_box_miss` | 1372.0 ns |

## mpr_intersect_box_hit

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `mpr_intersect_box_hit` | 4898.0 ns |

## mpr_intersect_tangent

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `mpr_intersect_tangent` | 4997.0 ns |

## bvh_scatter_4k

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `bvh_scatter_4k` | 5806.0 µs |

## spatial_hash_query_2k

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `spatial_hash_query_2k` | 428040.0 ns |

## num_dct_1024

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `num_dct_1024` | 293904.0 ns |

## num_dst_1024

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `num_dst_1024` | 6615.0 µs |

## num_dct_1023

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `num_dct_1023` | 1565.0 µs |

## num_dst_1023

| Benchmark | Current (`a09d228`) |
|-----------|------|
| `num_dst_1023` | 448728.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
