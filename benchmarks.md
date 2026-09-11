# Benchmarks

> **Measurement changes** — read before comparing across a date.
> * **2026-08-21** (hisab 2.11.2, cyrius 6.5.18 → 6.5.33): `lib/bench.cyr` now
>   MEASURES one clock read on the host and subtracts it from every sample, and
>   `bench_run` sizes its own batches instead of wrapping a clock pair around every
>   iteration. **No hisab source changed.** The floor is re-measured every run and
>   recorded per row in `floor_ns` (~1,340–1,350 ns on this host; upstream measured a
>   230x spread across its four gate hosts, and it moves between reboots, so it is
>   not a constant and is not written down as one).
>   `ease_in_out` 1,407 ns → 7 ns, `cx_mul` 1,459 → 37, `quat_mul` 1,473 → 67,
>   `perlin_2d` 1,461 → 82 — those four were 94–100% instrument. The old numbers
>   also FLATTENED them: four operations spanning **149x** in reality were reported
>   within **1.26x** of each other, so a real regression had room to hide. 44 of 72
>   rows moved more than 10%; **none of it is a speedup**. Rows carry `regime=net`
>   from this date and the trend filter refuses to mix them with `raw` ones.
>   ⚠ `min_ns`/`max_ns` also changed meaning for anything `bench_run` chose to batch:
>   they are now per-CHUNK averages, not per-iteration extremes. That is the honest
>   reading — a single sub-floor iteration has no measurable duration — but it means
>   the spread narrows for reasons unrelated to the code. `estimate_ns` (the avg) is
>   unaffected and remains the column the trend is built on.
> * **2026-08-10**: 17 sub-microsecond benchmarks moved from `bench()` to
>   `bench_batch()`. `bench_run` wraps a `clock_gettime` PAIR around every call
>   (~240 ns, documented in `lib/bench.cyr`), so those rows were **~95% clock
>   overhead**: `ray_sphere` read 1,466 ns and is 79 ns; `vec3_add` read 1,457 ns
>   and is 36 ns. It also FLATTENED them — triangle/sphere measured 1.19x when the
>   true ratio is 3.6x — so a real regression could hide inside the overhead.
>   Their drop at this date is an artefact of the fix, not a speedup.
> * **2026-08-09**: `estimate_ns` changed from each benchmark's MAX to its AVG.
>   The `stat` column records which; rows with no value there are `max`.

Latest: **2026-09-11T05:57:09Z** — commit `a6362ce`

Tracking: `a09d228` (baseline) → `e241e7a` (mid) → `a6362ce` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 18.00 ns **-22%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 28.00 ns **-56%** | 27.00 ns **-57%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 33.00 ns **-18%** | 34.00 ns **-15%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 393.0 ns **-34%** | 381.0 ns **-36%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 295.0 ns | 297.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1949.0 ns **-10%** | 1904.0 ns **-12%** |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2788.0 ns **-15%** | 2731.0 ns **-16%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 38.00 ns **-40%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 239.0 ns | 239.0 ns |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 38.00 ns **-40%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 120.0 ns | 118.0 ns **-11%** |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 244.0 ns | 244.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 92.00 ns **-25%** | 94.00 ns **-23%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 148.0 ns **-32%** | 147.0 ns **-32%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 190.0 ns **-30%** | 193.0 ns **-29%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 100.0 ns **-42%** | 99.00 ns **-43%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 465.0 ns **-40%** | 471.0 ns **-39%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 181.0 ns **-38%** | 180.0 ns **-38%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 534.0 ns **-35%** | 546.0 ns **-33%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 707.0 ns **-29%** | 694.0 ns **-30%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 43696.0 ns **-25%** | 44037.0 ns **-24%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4486.0 ns **-23%** | 4526.0 ns **-23%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 278.0 ns **-37%** | 280.0 ns **-37%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 64.00 ns **-44%** | 64.00 ns **-44%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 524.0 ns **-29%** | 516.0 ns **-30%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 394.0 ns **-26%** | 382.0 ns **-29%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 57.00 ns **-37%** | 58.00 ns **-36%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 42.00 ns **-55%** | 41.00 ns **-56%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 142.0 ns **-42%** | 141.0 ns **-43%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 96.00 ns +17% | 96.00 ns +17% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 22.00 ns **-27%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 91.00 ns | 91.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4844.0 ns | 4849.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19661.0 ns | 19974.0 ns |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 7.00 ns |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 75.00 ns | 75.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1681.0 µs **-12%** | 1707.0 µs **-11%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 713099.0 ns **-14%** | 734800.0 ns **-11%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2302.0 µs **-30%** | 2344.0 µs **-29%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 955134.0 ns **-46%** | 955724.0 ns **-46%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2328.0 µs | 2352.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 534480.0 ns | 542429.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 987.0 ns **-10%** | 998.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 445.0 ns | 448.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 717.0 ns **-31%** | 665.0 ns **-36%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1424.0 µs **-10%** | 1429.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 370320.0 ns | 371141.0 ns |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1596.0 µs | 1627.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5257.0 µs | 5416.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1183.0 ns **-19%** | 1211.0 ns **-17%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 111350.0 ns **-30%** | 120909.0 ns **-24%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 86955.0 ns **-27%** | 91086.0 ns **-24%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 80206.0 ns **-27%** | 86456.0 ns **-21%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 12771.0 ns **-27%** | 13781.0 ns **-21%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 12706.0 ns **-27%** | 13681.0 ns **-22%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 405717.0 ns **-24%** | 423561.0 ns **-21%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 80361.0 ns **-28%** | 81371.0 ns **-27%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 458.0 ns **-31%** | 463.0 ns **-30%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 659.0 ns **-27%** | 671.0 ns **-26%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1013.0 ns **-37%** | 1024.0 ns **-36%** |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1163.0 ns **-37%** | 1188.0 ns **-35%** |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 964.0 ns **-30%** | 978.0 ns **-29%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3399.0 ns **-31%** | 3419.0 ns **-30%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3484.0 ns **-30%** | 3544.0 ns **-29%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4706.0 µs **-19%** | 4770.0 µs **-18%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 363230.0 ns **-15%** | 380609.0 ns **-11%** |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 282347.0 ns | 283207.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6495.0 µs | 6568.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1579.0 µs | 1583.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 446208.0 ns | 446901.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | — | 1175.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`e241e7a`) | Current (`a6362ce`) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | — | 183629.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
