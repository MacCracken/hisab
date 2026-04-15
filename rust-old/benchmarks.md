# Benchmarks

Latest: **2026-03-31T04:33:35Z** — commit `745870c`

Tracking: `9463cf0` (baseline) → `fc9aa33` (optimized) → `745870c` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.12 ns **-12%** | 6.60 ns **-5%** |
| `transform2d_apply_point` | 12.18 ns | 6.09 ns **-50%** | 6.56 ns **-46%** |
| `transform3d_to_matrix` | 11.20 ns | 5.18 ns **-54%** | 5.60 ns **-50%** |
| `transform3d_apply_point` | 13.71 ns | 5.92 ns **-57%** | 6.43 ns **-53%** |
| `projection_perspective` | 13.99 ns | 21.54 ns +54% | 12.84 ns **-8%** |
| `projection_orthographic` | 4.29 ns | 4.01 ns **-6%** | 4.45 ns +4% |
| `lerp_f32` | 1.05 ns | 0.99 ns **-6%** | 1.07 ns |
| `lerp_vec3` | 2.71 ns | 2.59 ns **-5%** | 2.81 ns +4% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.70 ns **-51%** | 2.93 ns **-47%** |
| `ray_plane_hit` | 2.10 ns | 1.97 ns **-6%** | 2.13 ns |
| `ray_aabb_hit` | 4.52 ns | 4.54 ns | 4.90 ns +9% |
| `ray_sphere_miss` | 3.40 ns | 2.00 ns **-41%** | 2.18 ns **-36%** |
| `aabb_contains` | 3.05 ns | 2.84 ns **-7%** | 3.06 ns |
| `sphere_contains` | 2.31 ns | 2.22 ns **-4%** | 2.40 ns +4% |
| `aabb_merge` | 4.13 ns | 4.00 ns **-3%** | 4.15 ns |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.09 ns | 1.17 ns +5% |
| `integral_simpson_100` | 78.23 ns | 76.63 ns | 142.2 ns +82% |
| `integral_simpson_1000` | 770.1 ns | 757.9 ns | 1365.3 ns +77% |
| `integral_trapezoidal_100` | 78.23 ns | 80.02 ns | 75.37 ns **-4%** |
| `integral_trapezoidal_1000` | 753.4 ns | 716.0 ns **-5%** | 773.7 ns |
| `bezier_quadratic` | 1.49 ns | 1.43 ns **-4%** | 1.54 ns +4% |
| `bezier_cubic` | 2.36 ns | 2.22 ns **-6%** | 2.56 ns +9% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.12 ns **-7%** | 6.73 ns |
| `bisection_sqrt2` | 113.2 ns | 104.0 ns **-8%** | 112.5 ns |
| `gaussian_3x3` | 83.64 ns | 74.93 ns **-10%** | 79.75 ns **-5%** |
| `gaussian_4x4` | 121.1 ns | 104.2 ns **-14%** | 111.0 ns **-8%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 186.7 ns | 201.0 ns |
| `aabb_contains_100` | — | 125.0 ns | 135.2 ns |
| `transform3d_batch_100` | — | 358.1 ns | 387.8 ns |
| `simpson_sin_10000` | — | 79820.0 ns | 92259.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.38 ns | 7.97 ns |
| `aabb_aabb_overlap` | — | 2.23 ns | 2.38 ns |
| `sphere_sphere_overlap` | — | 1.76 ns | 1.91 ns |
| `frustum_contains_point` | — | 4.44 ns | 4.75 ns |
| `frustum_contains_aabb` | — | 4.25 ns | 4.50 ns |
| `slerp` | — | 19.22 ns | 21.20 ns |
| `transform3d_lerp` | — | 24.06 ns | 25.27 ns |
| `closest_on_aabb` | — | 2.52 ns | 2.75 ns |
| `segment_closest_point` | — | 2.91 ns | 3.13 ns |
| `plane_plane_intersection` | — | 7.04 ns | 7.71 ns |
| `triangle_unit_normal` | — | 4.91 ns | 5.41 ns |
| `line_closest_point` | — | 2.12 ns | 2.23 ns |
| `closest_on_sphere` | — | 4.87 ns | 5.36 ns |
| `inverse_matrix` | — | 18.24 ns | 20.03 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.70 ns | 2.95 ns |
| `de_casteljau_split` | — | 5.66 ns | 6.17 ns |
| `catmull_rom` | — | 2.50 ns | 2.70 ns |
| `bspline_cubic` | — | 21.66 ns | 16.03 ns |
| `gauss_legendre_5` | — | 3.30 ns | 4.00 ns |
| `gauss_legendre_10_panels` | — | 399.6 ns | 429.2 ns |
| `arc_length_100` | — | 552.8 ns | 636.6 ns |
| `ease_in_out` | — | 0.56 ns | 0.60 ns |
| `ease_in_out_smooth` | — | 0.78 ns | 0.84 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 99.78 ns | 112.0 ns |
| `lu_solve_3x3` | — | 32.36 ns | 34.28 ns |
| `cholesky_3x3` | — | 62.69 ns | 65.87 ns |
| `cholesky_solve_3x3` | — | 36.21 ns | 40.38 ns |
| `qr_decompose_3col` | — | 123.1 ns | 135.6 ns |
| `least_squares_linear_6pt` | — | 173.5 ns | 186.1 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 399.3 ns | 434.2 ns |
| `fft_64` | — | 612.9 ns | 665.1 ns |
| `fft_1024` | — | 15168.0 ns | 16594.0 ns |
| `fft_ifft_256` | — | 6149.7 ns | 6787.9 ns |
| `dst_64` | — | — | 41142.0 ns |
| `dct_64` | — | — | 42815.0 ns |
| `dst_idst_256` | — | — | 1461.9 µs |
| `dct_idct_256` | — | — | 1491.4 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | 2333.1 ns | 2900.9 ns |
| `rk4_exp_1000_steps` | — | 25720.0 ns | 28617.0 ns |
| `rk4_oscillator_1000` | — | 26020.0 ns | 31484.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `bvh_build_100` | — | 7322.8 ns | 8046.7 ns |
| `bvh_ray_query_100` | — | 62.64 ns | 67.42 ns |
| `bvh_build_1000` | — | 93431.0 ns | 103790.0 ns |
| `kdtree_build_1000` | — | 104080.0 ns | 117630.0 ns |
| `kdtree_nearest_1000` | — | 246.4 ns | 267.7 ns |
| `kdtree_radius_1000` | — | 1427.0 ns | 1574.1 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 75843.0 ns |
| `quadtree_query_1000` | — | — | 422.1 ns |
| `octree_insert_1000` | — | — | 84933.0 ns |
| `octree_query_1000` | — | — | 747.3 ns |
| `spatial_hash_insert_1000` | — | — | 43065.0 ns |
| `spatial_hash_query_cell` | — | — | 23.20 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 2027.7 ns |
| `gjk_intersect` | — | — | 28.30 ns |
| `gjk_no_intersect` | — | — | 23.07 ns |
| `gjk_epa_penetration` | — | — | 156.2 ns |

## v06

| Benchmark | Baseline (`9463cf0`) | Mid (`fc9aa33`) | Current (`745870c`) |
|-----------|------|------|------|
| `svd_3x3` | — | — | 829.7 ns |
| `svd_5x5` | — | — | 122720.0 ns |
| `matrix_inverse_3x3` | — | — | 242.2 ns |
| `pseudo_inverse_3x2` | — | — | 693.0 ns |
| `csr_spmv_100x100` | — | — | 209.0 ns |
| `svd_4x2_tall` | — | — | 552.1 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
