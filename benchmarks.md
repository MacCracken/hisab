# Benchmarks

Latest: **2026-03-28T00:58:18Z** — commit `bc09eb9`

Tracking: `9463cf0` (baseline) → `7f26a9a` (optimized) → `bc09eb9` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 5.97 ns **-15%** | 6.55 ns **-6%** |
| `transform2d_apply_point` | 12.18 ns | 6.12 ns **-50%** | 6.68 ns **-45%** |
| `transform3d_to_matrix` | 11.20 ns | 5.19 ns **-54%** | 5.79 ns **-48%** |
| `transform3d_apply_point` | 13.71 ns | 5.87 ns **-57%** | 6.42 ns **-53%** |
| `projection_perspective` | 13.99 ns | 13.98 ns | 12.88 ns **-8%** |
| `projection_orthographic` | 4.29 ns | 4.07 ns **-5%** | 4.49 ns +5% |
| `lerp_f32` | 1.05 ns | 1.01 ns **-4%** | 1.07 ns |
| `lerp_vec3` | 2.71 ns | 2.61 ns **-4%** | 2.79 ns +3% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.71 ns **-51%** | 2.94 ns **-46%** |
| `ray_plane_hit` | 2.10 ns | 1.97 ns **-6%** | 2.17 ns +4% |
| `ray_aabb_hit` | 4.52 ns | 4.74 ns +5% | 5.27 ns +17% |
| `ray_sphere_miss` | 3.40 ns | 1.91 ns **-44%** | 2.92 ns **-14%** |
| `aabb_contains` | 3.05 ns | 2.85 ns **-7%** | 3.14 ns |
| `sphere_contains` | 2.31 ns | 2.20 ns **-5%** | 2.39 ns +4% |
| `aabb_merge` | 4.13 ns | 3.79 ns **-8%** | 4.31 ns +4% |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.06 ns **-4%** | 1.17 ns +6% |
| `integral_simpson_100` | 78.23 ns | 75.83 ns **-3%** | 142.1 ns +82% |
| `integral_simpson_1000` | 770.1 ns | 749.5 ns | 1366.9 ns +78% |
| `integral_trapezoidal_100` | 78.23 ns | 68.92 ns **-12%** | 75.50 ns **-3%** |
| `integral_trapezoidal_1000` | 753.4 ns | 708.4 ns **-6%** | 773.9 ns |
| `bezier_quadratic` | 1.49 ns | 1.41 ns **-5%** | 1.53 ns |
| `bezier_cubic` | 2.36 ns | 2.29 ns | 2.55 ns +8% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.11 ns **-8%** | 6.62 ns |
| `bisection_sqrt2` | 113.2 ns | 103.1 ns **-9%** | 111.9 ns |
| `gaussian_3x3` | 83.64 ns | 79.98 ns **-4%** | 80.98 ns **-3%** |
| `gaussian_4x4` | 121.1 ns | 104.3 ns **-14%** | 109.1 ns **-10%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 185.9 ns | 200.9 ns |
| `aabb_contains_100` | — | 124.1 ns | 135.3 ns |
| `transform3d_batch_100` | — | 349.1 ns | 383.4 ns |
| `simpson_sin_10000` | — | 78076.0 ns | 92282.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.26 ns | 8.04 ns |
| `aabb_aabb_overlap` | — | 2.18 ns | 2.37 ns |
| `sphere_sphere_overlap` | — | 1.75 ns | 1.90 ns |
| `frustum_contains_point` | — | 4.39 ns | 4.77 ns |
| `frustum_contains_aabb` | — | 4.16 ns | 4.60 ns |
| `slerp` | — | 17.21 ns | 21.09 ns |
| `transform3d_lerp` | — | 20.22 ns | 25.09 ns |
| `closest_on_aabb` | — | 2.50 ns | 2.73 ns |
| `segment_closest_point` | — | 2.88 ns | 3.14 ns |
| `plane_plane_intersection` | — | 6.93 ns | 7.62 ns |
| `triangle_unit_normal` | — | 4.86 ns | 5.45 ns |
| `line_closest_point` | — | 2.17 ns | 2.26 ns |
| `closest_on_sphere` | — | 4.99 ns | 5.35 ns |
| `inverse_matrix` | — | 18.30 ns | 20.10 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.73 ns | 2.95 ns |
| `de_casteljau_split` | — | 5.65 ns | 6.19 ns |
| `catmull_rom` | — | 2.62 ns | 2.71 ns |
| `bspline_cubic` | — | 22.00 ns | 15.65 ns |
| `gauss_legendre_5` | — | 3.32 ns | 3.86 ns |
| `gauss_legendre_10_panels` | — | 399.7 ns | 429.3 ns |
| `arc_length_100` | — | 550.0 ns | 643.1 ns |
| `ease_in_out` | — | 0.55 ns | 0.60 ns |
| `ease_in_out_smooth` | — | 0.78 ns | 0.84 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 110.5 ns | 107.4 ns |
| `lu_solve_3x3` | — | 32.29 ns | 33.48 ns |
| `cholesky_3x3` | — | 62.49 ns | 69.33 ns |
| `cholesky_solve_3x3` | — | 37.68 ns | 39.58 ns |
| `qr_decompose_3col` | — | 127.3 ns | 134.2 ns |
| `least_squares_linear_6pt` | — | 177.1 ns | 186.0 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 402.1 ns | 434.6 ns |
| `fft_64` | — | 607.6 ns | 663.0 ns |
| `fft_1024` | — | 15166.0 ns | 16515.0 ns |
| `fft_ifft_256` | — | 6205.3 ns | 6827.5 ns |
| `dst_64` | — | — | 41239.0 ns |
| `dct_64` | — | — | 43031.0 ns |
| `dst_idst_256` | — | — | 1456.2 µs |
| `dct_idct_256` | — | — | 1512.8 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | 2527.8 ns | 2907.2 ns |
| `rk4_exp_1000_steps` | — | 25193.0 ns | 28612.0 ns |
| `rk4_oscillator_1000` | — | 24136.0 ns | 31551.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `bvh_build_100` | — | 10695.0 ns | 8046.5 ns |
| `bvh_ray_query_100` | — | 62.08 ns | 68.07 ns |
| `bvh_build_1000` | — | 141510.0 ns | 102210.0 ns |
| `kdtree_build_1000` | — | 139570.0 ns | 114830.0 ns |
| `kdtree_nearest_1000` | — | 265.8 ns | 267.8 ns |
| `kdtree_radius_1000` | — | 1401.1 ns | 1566.6 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 76370.0 ns |
| `quadtree_query_1000` | — | — | 416.8 ns |
| `octree_insert_1000` | — | — | 87964.0 ns |
| `octree_query_1000` | — | — | 766.9 ns |
| `spatial_hash_insert_1000` | — | — | 43978.0 ns |
| `spatial_hash_query_cell` | — | — | 24.20 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 2242.3 ns |
| `gjk_intersect` | — | — | 28.89 ns |
| `gjk_no_intersect` | — | — | 22.15 ns |
| `gjk_epa_penetration` | — | — | 155.7 ns |

## v06

| Benchmark | Baseline (`9463cf0`) | Mid (`7f26a9a`) | Current (`bc09eb9`) |
|-----------|------|------|------|
| `svd_3x3` | — | — | 761.6 ns |
| `svd_5x5` | — | — | 122720.0 ns |
| `matrix_inverse_3x3` | — | — | 270.1 ns |
| `pseudo_inverse_3x2` | — | — | 587.9 ns |
| `csr_spmv_100x100` | — | — | 237.6 ns |
| `svd_4x2_tall` | — | — | 558.3 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
