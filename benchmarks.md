# Benchmarks

Latest: **2026-03-28T00:20:52Z** — commit `72ba090`

Tracking: `9463cf0` (baseline) → `ad559f8` (optimized) → `72ba090` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.36 ns **-9%** | 6.81 ns |
| `transform2d_apply_point` | 12.18 ns | 6.67 ns **-45%** | 6.77 ns **-44%** |
| `transform3d_to_matrix` | 11.20 ns | 5.54 ns **-51%** | 6.07 ns **-46%** |
| `transform3d_apply_point` | 13.71 ns | 6.46 ns **-53%** | 6.45 ns **-53%** |
| `projection_perspective` | 13.99 ns | 23.47 ns +68% | 12.91 ns **-8%** |
| `projection_orthographic` | 4.29 ns | 4.47 ns +4% | 4.48 ns +5% |
| `lerp_f32` | 1.05 ns | 1.10 ns +4% | 1.08 ns |
| `lerp_vec3` | 2.71 ns | 2.69 ns | 2.80 ns +3% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.96 ns **-46%** | 2.96 ns **-46%** |
| `ray_plane_hit` | 2.10 ns | 2.13 ns | 2.18 ns +4% |
| `ray_aabb_hit` | 4.52 ns | 4.93 ns +9% | 5.25 ns +16% |
| `ray_sphere_miss` | 3.40 ns | 2.28 ns **-33%** | 2.17 ns **-36%** |
| `aabb_contains` | 3.05 ns | 3.11 ns | 3.12 ns |
| `sphere_contains` | 2.31 ns | 2.35 ns | 2.38 ns +3% |
| `aabb_merge` | 4.13 ns | 4.20 ns | 4.30 ns +4% |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.17 ns +5% | 1.17 ns +5% |
| `integral_simpson_100` | 78.23 ns | 79.67 ns | 82.82 ns +6% |
| `integral_simpson_1000` | 770.1 ns | 755.4 ns | 813.0 ns +6% |
| `integral_trapezoidal_100` | 78.23 ns | 72.64 ns **-7%** | 76.55 ns |
| `integral_trapezoidal_1000` | 753.4 ns | 764.0 ns | 773.7 ns |
| `bezier_quadratic` | 1.49 ns | 1.53 ns | 1.53 ns +3% |
| `bezier_cubic` | 2.36 ns | 2.56 ns +8% | 2.52 ns +7% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.35 ns **-4%** | 6.60 ns |
| `bisection_sqrt2` | 113.2 ns | 108.2 ns **-4%** | 111.8 ns |
| `gaussian_3x3` | 83.64 ns | 77.86 ns **-7%** | 80.61 ns **-4%** |
| `gaussian_4x4` | 121.1 ns | 107.4 ns **-11%** | 110.0 ns **-9%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 193.8 ns | 203.9 ns |
| `aabb_contains_100` | — | 127.9 ns | 134.8 ns |
| `transform3d_batch_100` | — | 390.5 ns | 383.0 ns |
| `simpson_sin_10000` | — | 81923.0 ns | 85514.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.64 ns | 7.97 ns |
| `aabb_aabb_overlap` | — | 2.34 ns | 2.38 ns |
| `sphere_sphere_overlap` | — | 1.90 ns | 1.89 ns |
| `frustum_contains_point` | — | 4.72 ns | 4.75 ns |
| `frustum_contains_aabb` | — | 4.43 ns | 4.58 ns |
| `slerp` | — | 20.26 ns | 21.18 ns |
| `transform3d_lerp` | — | 24.69 ns | 25.29 ns |
| `closest_on_aabb` | — | 2.69 ns | 2.72 ns |
| `segment_closest_point` | — | 3.05 ns | 3.21 ns |
| `plane_plane_intersection` | — | 7.43 ns | 7.95 ns |
| `triangle_unit_normal` | — | 5.14 ns | 7.81 ns |
| `line_closest_point` | — | 2.14 ns | 2.78 ns |
| `closest_on_sphere` | — | 5.17 ns | 5.96 ns |
| `inverse_matrix` | — | 19.34 ns | 22.67 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 2.97 ns | 7.02 ns |
| `de_casteljau_split` | — | 6.29 ns | 7.95 ns |
| `catmull_rom` | — | 2.54 ns | 4.40 ns |
| `bspline_cubic` | — | 22.39 ns | 16.38 ns |
| `gauss_legendre_5` | — | 3.31 ns | 4.82 ns |
| `gauss_legendre_10_panels` | — | 396.2 ns | 442.6 ns |
| `arc_length_100` | — | 564.5 ns | 651.7 ns |
| `ease_in_out` | — | 0.56 ns | 1.14 ns |
| `ease_in_out_smooth` | — | 0.78 ns | 1.58 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 106.4 ns | 169.4 ns |
| `lu_solve_3x3` | — | 31.57 ns | 50.09 ns |
| `cholesky_3x3` | — | 61.57 ns | 115.4 ns |
| `cholesky_solve_3x3` | — | 36.65 ns | 65.02 ns |
| `qr_decompose_3col` | — | 123.2 ns | 248.9 ns |
| `least_squares_linear_6pt` | — | 181.8 ns | 352.2 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 413.0 ns | 612.2 ns |
| `fft_64` | — | 606.4 ns | 1033.3 ns |
| `fft_1024` | — | 15082.0 ns | 22934.0 ns |
| `fft_ifft_256` | — | 6143.9 ns | 10046.0 ns |
| `dst_64` | — | — | 71565.0 ns |
| `dct_64` | — | — | 73331.0 ns |
| `dst_idst_256` | — | — | 2425.2 µs |
| `dct_idct_256` | — | — | 2458.2 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | 2291.6 ns | 3077.3 ns |
| `rk4_exp_1000_steps` | — | 23262.0 ns | 29721.0 ns |
| `rk4_oscillator_1000` | — | 22506.0 ns | 32453.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 17774.0 ns |
| `bvh_ray_query_100` | — | — | 142.7 ns |
| `bvh_build_1000` | — | — | 205200.0 ns |
| `kdtree_build_1000` | — | — | 233410.0 ns |
| `kdtree_nearest_1000` | — | — | 569.9 ns |
| `kdtree_radius_1000` | — | — | 3256.5 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 132670.0 ns |
| `quadtree_query_1000` | — | — | 921.8 ns |
| `octree_insert_1000` | — | — | 172620.0 ns |
| `octree_query_1000` | — | — | 2105.0 ns |
| `spatial_hash_insert_1000` | — | — | 67865.0 ns |
| `spatial_hash_query_cell` | — | — | 36.64 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 4419.7 ns |
| `gjk_intersect` | — | — | 35.68 ns |
| `gjk_no_intersect` | — | — | 31.31 ns |
| `gjk_epa_penetration` | — | — | 197.5 ns |

## v06

| Benchmark | Baseline (`9463cf0`) | Mid (`ad559f8`) | Current (`72ba090`) |
|-----------|------|------|------|
| `svd_3x3` | — | — | 1326.1 ns |
| `svd_5x5` | — | — | 150430.0 ns |
| `matrix_inverse_3x3` | — | — | 526.2 ns |
| `pseudo_inverse_3x2` | — | — | 1531.3 ns |
| `csr_spmv_100x100` | — | — | 441.9 ns |
| `svd_4x2_tall` | — | — | 1085.8 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
