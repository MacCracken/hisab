# Benchmarks

Latest: **2026-03-23T03:38:53Z** — commit `6a58334`

Tracking: `9463cf0` (baseline) → `ee84541` (optimized) → `6a58334` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.18 ns **-11%** | 6.22 ns **-11%** |
| `transform2d_apply_point` | 12.18 ns | 6.19 ns **-49%** | 6.17 ns **-49%** |
| `transform3d_to_matrix` | 11.20 ns | 5.30 ns **-53%** | 5.42 ns **-52%** |
| `transform3d_apply_point` | 13.71 ns | 6.08 ns **-56%** | 6.06 ns **-56%** |
| `projection_perspective` | 13.99 ns | 22.04 ns +58% | 22.37 ns +60% |
| `projection_orthographic` | 4.29 ns | 4.09 ns **-5%** | 4.09 ns **-5%** |
| `lerp_f32` | 1.05 ns | 1.02 ns **-3%** | 1.02 ns |
| `lerp_vec3` | 2.71 ns | 2.69 ns | 2.62 ns **-3%** |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.75 ns **-50%** | 2.70 ns **-51%** |
| `ray_plane_hit` | 2.10 ns | 2.04 ns | 2.01 ns **-4%** |
| `ray_aabb_hit` | 4.52 ns | 4.85 ns +7% | 4.85 ns +7% |
| `ray_sphere_miss` | 3.40 ns | 1.95 ns **-42%** | 1.96 ns **-42%** |
| `aabb_contains` | 3.05 ns | 2.86 ns **-6%** | 2.87 ns **-6%** |
| `sphere_contains` | 2.31 ns | 2.26 ns | 2.23 ns **-3%** |
| `aabb_merge` | 4.13 ns | 3.90 ns **-6%** | 4.02 ns |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.13 ns | 1.09 ns |
| `integral_simpson_100` | 78.23 ns | 82.10 ns +5% | 77.02 ns |
| `integral_simpson_1000` | 770.1 ns | 811.4 ns +5% | 756.5 ns |
| `integral_trapezoidal_100` | 78.23 ns | 75.95 ns | 69.95 ns **-11%** |
| `integral_trapezoidal_1000` | 753.4 ns | 779.8 ns +3% | 739.8 ns |
| `bezier_quadratic` | 1.49 ns | 1.61 ns +8% | 1.44 ns |
| `bezier_cubic` | 2.36 ns | 2.49 ns +5% | 2.21 ns **-6%** |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.97 ns +5% | 6.05 ns **-9%** |
| `bisection_sqrt2` | 113.2 ns | 114.0 ns | 102.2 ns **-10%** |
| `gaussian_3x3` | 83.64 ns | 84.77 ns | 76.78 ns **-8%** |
| `gaussian_4x4` | 121.1 ns | 115.0 ns **-5%** | 103.7 ns **-14%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 211.6 ns | 186.1 ns |
| `aabb_contains_100` | — | 141.1 ns | 125.4 ns |
| `transform3d_batch_100` | — | 407.5 ns | 352.0 ns |
| `simpson_sin_10000` | — | 87211.0 ns | 78349.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `ray_triangle` | — | 8.22 ns | 7.47 ns |
| `aabb_aabb_overlap` | — | 2.30 ns | 2.33 ns |
| `sphere_sphere_overlap` | — | 1.92 ns | 1.80 ns |
| `frustum_contains_point` | — | 4.79 ns | 4.53 ns |
| `frustum_contains_aabb` | — | 4.45 ns | 4.33 ns |
| `slerp` | — | 20.43 ns | 18.70 ns |
| `transform3d_lerp` | — | 25.29 ns | 20.91 ns |
| `closest_on_aabb` | — | 2.69 ns | 2.63 ns |
| `segment_closest_point` | — | 3.10 ns | 2.98 ns |
| `plane_plane_intersection` | — | 7.52 ns | 7.13 ns |
| `triangle_unit_normal` | — | 5.14 ns | 5.03 ns |
| `line_closest_point` | — | 2.16 ns | 2.15 ns |
| `closest_on_sphere` | — | 5.19 ns | 5.06 ns |
| `inverse_matrix` | — | 19.35 ns | 19.05 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.98 ns | 2.82 ns |
| `de_casteljau_split` | — | 6.16 ns | 6.01 ns |
| `catmull_rom` | — | 2.62 ns | 2.55 ns |
| `bspline_cubic` | — | 23.30 ns | 22.83 ns |
| `gauss_legendre_5` | — | 3.62 ns | 3.41 ns |
| `gauss_legendre_10_panels` | — | 440.4 ns | 440.5 ns |
| `arc_length_100` | — | 602.1 ns | 617.5 ns |
| `ease_in_out` | — | 0.59 ns | 0.66 ns |
| `ease_in_out_smooth` | — | 0.82 ns | 0.81 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 119.1 ns | 106.8 ns |
| `lu_solve_3x3` | — | 33.80 ns | 35.12 ns |
| `cholesky_3x3` | — | 78.11 ns | 65.89 ns |
| `cholesky_solve_3x3` | — | 38.32 ns | 39.17 ns |
| `qr_decompose_3col` | — | 127.9 ns | 129.0 ns |
| `least_squares_linear_6pt` | — | 182.2 ns | 191.0 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | — | 436.8 ns |
| `fft_64` | — | — | 630.0 ns |
| `fft_1024` | — | — | 15688.0 ns |
| `fft_ifft_256` | — | — | 6405.8 ns |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | — | 2543.6 ns |
| `rk4_exp_1000_steps` | — | — | 23818.0 ns |
| `rk4_oscillator_1000` | — | — | 23320.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 7776.1 ns |
| `bvh_ray_query_100` | — | — | 62.80 ns |
| `bvh_build_1000` | — | — | 96706.0 ns |
| `kdtree_build_1000` | — | — | 108620.0 ns |
| `kdtree_nearest_1000` | — | — | 251.7 ns |
| `kdtree_radius_1000` | — | — | 1544.2 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 72310.0 ns |
| `quadtree_query_1000` | — | — | 420.1 ns |
| `octree_insert_1000` | — | — | 81383.0 ns |
| `octree_query_1000` | — | — | 712.3 ns |
| `spatial_hash_insert_1000` | — | — | 37031.0 ns |
| `spatial_hash_query_cell` | — | — | 23.36 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`ee84541`) | Current (`6a58334`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 1907.8 ns |
| `gjk_intersect` | — | — | 36.16 ns |
| `gjk_no_intersect` | — | — | 26.36 ns |
| `gjk_epa_penetration` | — | — | 322.2 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
