# Benchmarks

Latest: **2026-03-27T21:09:08Z** — commit `0f426af`

Tracking: `9463cf0` (baseline) → `ad559f8` (optimized) → `0f426af` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.36 ns **-9%** | 6.77 ns **-3%** |
| `transform2d_apply_point` | 12.18 ns | 6.67 ns **-45%** | 7.71 ns **-37%** |
| `transform3d_to_matrix` | 11.20 ns | 5.54 ns **-51%** | 6.31 ns **-44%** |
| `transform3d_apply_point` | 13.71 ns | 6.46 ns **-53%** | 7.22 ns **-47%** |
| `projection_perspective` | 13.99 ns | 23.47 ns +68% | 14.73 ns +5% |
| `projection_orthographic` | 4.29 ns | 4.47 ns +4% | 4.95 ns +15% |
| `lerp_f32` | 1.05 ns | 1.10 ns +4% | 1.11 ns +5% |
| `lerp_vec3` | 2.71 ns | 2.69 ns | 3.00 ns +11% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.96 ns **-46%** | 3.44 ns **-37%** |
| `ray_plane_hit` | 2.10 ns | 2.13 ns | 2.31 ns +10% |
| `ray_aabb_hit` | 4.52 ns | 4.93 ns +9% | 5.18 ns +15% |
| `ray_sphere_miss` | 3.40 ns | 2.28 ns **-33%** | 2.42 ns **-29%** |
| `aabb_contains` | 3.05 ns | 3.11 ns | 3.36 ns +10% |
| `sphere_contains` | 2.31 ns | 2.35 ns | 2.58 ns +12% |
| `aabb_merge` | 4.13 ns | 4.20 ns | 4.51 ns +9% |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.17 ns +5% | 1.23 ns +11% |
| `integral_simpson_100` | 78.23 ns | 79.67 ns | 86.45 ns +11% |
| `integral_simpson_1000` | 770.1 ns | 755.4 ns | 822.5 ns +7% |
| `integral_trapezoidal_100` | 78.23 ns | 72.64 ns **-7%** | 76.31 ns |
| `integral_trapezoidal_1000` | 753.4 ns | 764.0 ns | 805.0 ns +7% |
| `bezier_quadratic` | 1.49 ns | 1.53 ns | 1.64 ns +10% |
| `bezier_cubic` | 2.36 ns | 2.56 ns +8% | 3.94 ns +67% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.35 ns **-4%** | 10.87 ns +64% |
| `bisection_sqrt2` | 113.2 ns | 108.2 ns **-4%** | 128.2 ns +13% |
| `gaussian_3x3` | 83.64 ns | 77.86 ns **-7%** | 127.0 ns +52% |
| `gaussian_4x4` | 121.1 ns | 107.4 ns **-11%** | 119.7 ns |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 193.8 ns | 228.0 ns |
| `aabb_contains_100` | — | 127.9 ns | 150.9 ns |
| `transform3d_batch_100` | — | 390.5 ns | 425.8 ns |
| `simpson_sin_10000` | — | 81923.0 ns | 92186.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.64 ns | 8.82 ns |
| `aabb_aabb_overlap` | — | 2.34 ns | 2.61 ns |
| `sphere_sphere_overlap` | — | 1.90 ns | 2.12 ns |
| `frustum_contains_point` | — | 4.72 ns | 5.37 ns |
| `frustum_contains_aabb` | — | 4.43 ns | 4.90 ns |
| `slerp` | — | 20.26 ns | 21.31 ns |
| `transform3d_lerp` | — | 24.69 ns | 25.41 ns |
| `closest_on_aabb` | — | 2.69 ns | 2.92 ns |
| `segment_closest_point` | — | 3.05 ns | 3.58 ns |
| `plane_plane_intersection` | — | 7.43 ns | 7.89 ns |
| `triangle_unit_normal` | — | 5.14 ns | 5.64 ns |
| `line_closest_point` | — | 2.14 ns | 2.36 ns |
| `closest_on_sphere` | — | 5.17 ns | 5.47 ns |
| `inverse_matrix` | — | 19.34 ns | 20.48 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.97 ns | 3.28 ns |
| `de_casteljau_split` | — | 6.29 ns | 6.55 ns |
| `catmull_rom` | — | 2.54 ns | 2.76 ns |
| `bspline_cubic` | — | 22.39 ns | 16.26 ns |
| `gauss_legendre_5` | — | 3.31 ns | 4.15 ns |
| `gauss_legendre_10_panels` | — | 396.2 ns | 462.8 ns |
| `arc_length_100` | — | 564.5 ns | 642.8 ns |
| `ease_in_out` | — | 0.56 ns | 0.60 ns |
| `ease_in_out_smooth` | — | 0.78 ns | 0.87 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 106.4 ns | 123.2 ns |
| `lu_solve_3x3` | — | 31.57 ns | 35.44 ns |
| `cholesky_3x3` | — | 61.57 ns | 68.01 ns |
| `cholesky_solve_3x3` | — | 36.65 ns | 40.68 ns |
| `qr_decompose_3col` | — | 123.2 ns | 139.9 ns |
| `least_squares_linear_6pt` | — | 181.8 ns | 191.8 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 413.0 ns | 467.8 ns |
| `fft_64` | — | 606.4 ns | 703.2 ns |
| `fft_1024` | — | 15082.0 ns | 16971.0 ns |
| `fft_ifft_256` | — | 6143.9 ns | 7038.2 ns |
| `dst_64` | — | — | 41426.0 ns |
| `dct_64` | — | — | 43191.0 ns |
| `dst_idst_256` | — | — | 1534.2 µs |
| `dct_idct_256` | — | — | 1569.9 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | 2291.6 ns | 2601.4 ns |
| `rk4_exp_1000_steps` | — | 23262.0 ns | 25702.0 ns |
| `rk4_oscillator_1000` | — | 22506.0 ns | 28806.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 8277.5 ns |
| `bvh_ray_query_100` | — | — | 76.73 ns |
| `bvh_build_1000` | — | — | 111290.0 ns |
| `kdtree_build_1000` | — | — | 126890.0 ns |
| `kdtree_nearest_1000` | — | — | 305.8 ns |
| `kdtree_radius_1000` | — | — | 1802.8 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 83456.0 ns |
| `quadtree_query_1000` | — | — | 449.6 ns |
| `octree_insert_1000` | — | — | 85306.0 ns |
| `octree_query_1000` | — | — | 822.1 ns |
| `spatial_hash_insert_1000` | — | — | 39035.0 ns |
| `spatial_hash_query_cell` | — | — | 25.02 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 2080.8 ns |
| `gjk_intersect` | — | — | 31.50 ns |
| `gjk_no_intersect` | — | — | 25.95 ns |
| `gjk_epa_penetration` | — | — | 170.4 ns |

## v06

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`0f426af`) |
|-----------|------|------|------|
| `svd_3x3` | — | — | 939.2 ns |
| `svd_5x5` | — | — | 132630.0 ns |
| `matrix_inverse_3x3` | — | — | 276.7 ns |
| `pseudo_inverse_3x2` | — | — | 698.7 ns |
| `csr_spmv_100x100` | — | — | 243.5 ns |
| `svd_4x2_tall` | — | — | 531.7 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
