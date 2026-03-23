# Benchmarks

Latest: **2026-03-23T02:47:48Z** — commit `78cc9f6`

Tracking: `9463cf0` (baseline) → `0314972` (optimized) → `78cc9f6` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.09 ns **-13%** | 6.24 ns **-11%** |
| `transform2d_apply_point` | 12.18 ns | 6.14 ns **-50%** | 6.03 ns **-50%** |
| `transform3d_to_matrix` | 11.20 ns | 5.16 ns **-54%** | 5.23 ns **-53%** |
| `transform3d_apply_point` | 13.71 ns | 6.01 ns **-56%** | 5.91 ns **-57%** |
| `projection_perspective` | 13.99 ns | 22.07 ns +58% | 21.95 ns +57% |
| `projection_orthographic` | 4.29 ns | 4.04 ns **-6%** | 4.08 ns **-5%** |
| `lerp_f32` | 1.05 ns | 1.00 ns **-5%** | 1.00 ns **-4%** |
| `lerp_vec3` | 2.71 ns | 2.60 ns **-4%** | 2.67 ns |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.73 ns **-50%** | 2.74 ns **-50%** |
| `ray_plane_hit` | 2.10 ns | 2.01 ns **-4%** | 2.01 ns **-4%** |
| `ray_aabb_hit` | 4.52 ns | 4.54 ns | 4.84 ns +7% |
| `ray_sphere_miss` | 3.40 ns | 1.92 ns **-43%** | 1.96 ns **-42%** |
| `aabb_contains` | 3.05 ns | 2.81 ns **-8%** | 2.87 ns **-6%** |
| `sphere_contains` | 2.31 ns | 2.22 ns **-4%** | 2.25 ns |
| `aabb_merge` | 4.13 ns | 3.95 ns **-4%** | 3.85 ns **-7%** |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.07 ns **-4%** | 1.08 ns **-3%** |
| `integral_simpson_100` | 78.23 ns | 75.67 ns **-3%** | 76.68 ns |
| `integral_simpson_1000` | 770.1 ns | 742.1 ns **-4%** | 749.2 ns |
| `integral_trapezoidal_100` | 78.23 ns | 80.89 ns +3% | 69.90 ns **-11%** |
| `integral_trapezoidal_1000` | 753.4 ns | 714.6 ns **-5%** | 712.4 ns **-5%** |
| `bezier_quadratic` | 1.49 ns | 1.43 ns **-4%** | 1.42 ns **-4%** |
| `bezier_cubic` | 2.36 ns | 2.24 ns **-5%** | 2.41 ns |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.20 ns **-6%** | 6.10 ns **-8%** |
| `bisection_sqrt2` | 113.2 ns | 102.3 ns **-10%** | 102.9 ns **-9%** |
| `gaussian_3x3` | 83.64 ns | 74.40 ns **-11%** | 76.56 ns **-8%** |
| `gaussian_4x4` | 121.1 ns | 102.1 ns **-16%** | 103.7 ns **-14%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 188.0 ns | 187.9 ns |
| `aabb_contains_100` | — | 126.8 ns | 126.1 ns |
| `transform3d_batch_100` | — | 357.2 ns | 355.1 ns |
| `simpson_sin_10000` | — | 79271.0 ns | 78862.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.42 ns | 7.39 ns |
| `aabb_aabb_overlap` | — | 2.28 ns | 2.25 ns |
| `sphere_sphere_overlap` | — | 1.77 ns | 1.76 ns |
| `frustum_contains_point` | — | 4.45 ns | 4.44 ns |
| `frustum_contains_aabb` | — | 4.30 ns | 4.30 ns |
| `slerp` | — | 19.59 ns | 17.50 ns |
| `transform3d_lerp` | — | 20.79 ns | 20.71 ns |
| `closest_on_aabb` | — | 2.60 ns | 2.52 ns |
| `segment_closest_point` | — | 2.96 ns | 2.90 ns |
| `plane_plane_intersection` | — | 7.28 ns | 7.12 ns |
| `triangle_unit_normal` | — | 5.02 ns | 5.02 ns |
| `line_closest_point` | — | 2.10 ns | 2.11 ns |
| `closest_on_sphere` | — | 5.05 ns | 4.97 ns |
| `inverse_matrix` | — | 18.74 ns | 18.62 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.77 ns | 2.77 ns |
| `de_casteljau_split` | — | 6.01 ns | 5.74 ns |
| `catmull_rom` | — | 2.54 ns | 2.53 ns |
| `bspline_cubic` | — | 22.02 ns | 22.76 ns |
| `gauss_legendre_5` | — | 3.44 ns | 3.31 ns |
| `gauss_legendre_10_panels` | — | 404.7 ns | 400.8 ns |
| `arc_length_100` | — | 559.1 ns | 547.0 ns |
| `ease_in_out` | — | 0.56 ns | 0.55 ns |
| `ease_in_out_smooth` | — | 0.80 ns | 0.78 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 104.5 ns | 104.5 ns |
| `lu_solve_3x3` | — | 34.90 ns | 32.50 ns |
| `cholesky_3x3` | — | 64.40 ns | 62.90 ns |
| `cholesky_solve_3x3` | — | 38.98 ns | 36.94 ns |
| `qr_decompose_3col` | — | 126.7 ns | 121.6 ns |
| `least_squares_linear_6pt` | — | 185.4 ns | 177.9 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | — | 403.8 ns |
| `fft_64` | — | — | 614.1 ns |
| `fft_1024` | — | — | 15026.0 ns |
| `fft_ifft_256` | — | — | 6126.3 ns |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | — | 2339.5 ns |
| `rk4_exp_1000_steps` | — | — | 24998.0 ns |
| `rk4_oscillator_1000` | — | — | 24391.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 7530.8 ns |
| `bvh_ray_query_100` | — | — | 62.96 ns |
| `bvh_build_1000` | — | — | 93139.0 ns |
| `kdtree_build_1000` | — | — | 106980.0 ns |
| `kdtree_nearest_1000` | — | — | 252.0 ns |
| `kdtree_radius_1000` | — | — | 1461.7 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`0314972`) | Current (`78cc9f6`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 71484.0 ns |
| `quadtree_query_1000` | — | — | 391.8 ns |
| `octree_insert_1000` | — | — | 77933.0 ns |
| `octree_query_1000` | — | — | 713.2 ns |
| `spatial_hash_insert_1000` | — | — | 36261.0 ns |
| `spatial_hash_query_cell` | — | — | 21.51 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
