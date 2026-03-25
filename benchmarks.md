# Benchmarks

Latest: **2026-03-25T05:16:24Z** — commit `0dcccc3`

Tracking: `9463cf0` (baseline) → `18bedd9` (optimized) → `0dcccc3` (current)

## transforms

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `transform2d_to_matrix` | 6.98 ns | 6.28 ns **-10%** | 7.05 ns |
| `transform2d_apply_point` | 12.18 ns | 6.26 ns **-49%** | 7.42 ns **-39%** |
| `transform3d_to_matrix` | 11.20 ns | 5.35 ns **-52%** | 5.79 ns **-48%** |
| `transform3d_apply_point` | 13.71 ns | 6.11 ns **-55%** | 7.13 ns **-48%** |
| `projection_perspective` | 13.99 ns | 25.44 ns +82% | 15.45 ns +10% |
| `projection_orthographic` | 4.29 ns | 4.23 ns | 5.12 ns +19% |
| `lerp_f32` | 1.05 ns | 1.03 ns | 1.13 ns +8% |
| `lerp_vec3` | 2.71 ns | 2.64 ns | 3.06 ns +13% |

## geo

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `ray_sphere_hit` | 5.48 ns | 2.74 ns **-50%** | 3.25 ns **-41%** |
| `ray_plane_hit` | 2.10 ns | 2.03 ns **-3%** | 2.43 ns +16% |
| `ray_aabb_hit` | 4.52 ns | 4.63 ns | 5.42 ns +20% |
| `ray_sphere_miss` | 3.40 ns | 2.04 ns **-40%** | 2.22 ns **-35%** |
| `aabb_contains` | 3.05 ns | 2.86 ns **-6%** | 3.22 ns +5% |
| `sphere_contains` | 2.31 ns | 2.23 ns **-3%** | 2.46 ns +7% |
| `aabb_merge` | 4.13 ns | 4.04 ns | 4.28 ns +3% |

## calc

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `derivative_x_squared` | 1.11 ns | 1.10 ns | 1.20 ns +8% |
| `integral_simpson_100` | 78.23 ns | 78.34 ns | 89.04 ns +14% |
| `integral_simpson_1000` | 770.1 ns | 755.7 ns | 816.6 ns +6% |
| `integral_trapezoidal_100` | 78.23 ns | 83.04 ns +6% | 81.36 ns +4% |
| `integral_trapezoidal_1000` | 753.4 ns | 724.5 ns **-4%** | 828.1 ns +10% |
| `bezier_quadratic` | 1.49 ns | 1.46 ns | 1.54 ns +3% |
| `bezier_cubic` | 2.36 ns | 2.23 ns **-6%** | 2.58 ns +9% |

## num

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `newton_sqrt2` | 6.62 ns | 6.37 ns **-4%** | 6.69 ns |
| `bisection_sqrt2` | 113.2 ns | 112.9 ns | 112.7 ns |
| `gaussian_3x3` | 83.64 ns | 79.11 ns **-5%** | 83.71 ns |
| `gaussian_4x4` | 121.1 ns | 119.9 ns | 115.2 ns **-5%** |

## batch

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `ray_sphere_100` | — | 195.2 ns | 216.7 ns |
| `aabb_contains_100` | — | 128.3 ns | 152.9 ns |
| `transform3d_batch_100` | — | 387.3 ns | 438.5 ns |
| `simpson_sin_10000` | — | 82339.0 ns | 94828.0 ns |

## v02

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `ray_triangle` | — | 7.77 ns | 8.88 ns |
| `aabb_aabb_overlap` | — | 2.30 ns | 2.67 ns |
| `sphere_sphere_overlap` | — | 1.91 ns | 1.99 ns |
| `frustum_contains_point` | — | 4.67 ns | 4.95 ns |
| `frustum_contains_aabb` | — | 4.41 ns | 5.42 ns |
| `slerp` | — | 20.29 ns | 23.53 ns |
| `transform3d_lerp` | — | 21.66 ns | 26.35 ns |
| `closest_on_aabb` | — | 2.64 ns | 2.99 ns |
| `segment_closest_point` | — | 3.12 ns | 3.17 ns |
| `plane_plane_intersection` | — | 7.68 ns | 7.82 ns |
| `triangle_unit_normal` | — | 5.31 ns | 5.44 ns |
| `line_closest_point` | — | 2.21 ns | 2.36 ns |
| `closest_on_sphere` | — | 5.21 ns | 5.40 ns |
| `inverse_matrix` | — | 19.40 ns | 20.17 ns |

## v03

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `bezier_cubic_3d` | — | 3.16 ns | 2.96 ns |
| `de_casteljau_split` | — | 6.25 ns | 6.20 ns |
| `catmull_rom` | — | 2.59 ns | 2.93 ns |
| `bspline_cubic` | — | 22.82 ns | 18.28 ns |
| `gauss_legendre_5` | — | 3.89 ns | 4.18 ns |
| `gauss_legendre_10_panels` | — | 432.5 ns | 475.5 ns |
| `arc_length_100` | — | 598.8 ns | 654.1 ns |
| `ease_in_out` | — | 0.60 ns | 0.62 ns |
| `ease_in_out_smooth` | — | 0.86 ns | 0.87 ns |

## v04a

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `lu_decompose_3x3` | — | 111.3 ns | 118.7 ns |
| `lu_solve_3x3` | — | 33.13 ns | 35.83 ns |
| `cholesky_3x3` | — | 63.25 ns | 70.88 ns |
| `cholesky_solve_3x3` | — | 38.80 ns | 41.94 ns |
| `qr_decompose_3col` | — | 126.4 ns | 155.2 ns |
| `least_squares_linear_6pt` | — | 196.8 ns | 221.2 ns |

## v04b

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `eigenvalue_3x3` | — | 441.6 ns | 542.4 ns |
| `fft_64` | — | 672.0 ns | 783.3 ns |
| `fft_1024` | — | 16354.0 ns | 18881.0 ns |
| `fft_ifft_256` | — | 6747.1 ns | 7403.3 ns |
| `dst_64` | — | — | 48909.0 ns |
| `dct_64` | — | — | 55637.0 ns |
| `dst_idst_256` | — | — | 1652.1 µs |
| `dct_idct_256` | — | — | 1783.9 µs |

## v04c

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `rk4_exp_100_steps` | — | — | 2792.5 ns |
| `rk4_exp_1000_steps` | — | — | 27094.0 ns |
| `rk4_oscillator_1000` | — | — | 29114.0 ns |

## v05a

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `bvh_build_100` | — | — | 9034.1 ns |
| `bvh_ray_query_100` | — | — | 70.16 ns |
| `bvh_build_1000` | — | — | 107450.0 ns |
| `kdtree_build_1000` | — | — | 120880.0 ns |
| `kdtree_nearest_1000` | — | — | 274.9 ns |
| `kdtree_radius_1000` | — | — | 1600.3 ns |

## v05b

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `quadtree_insert_1000` | — | — | 76504.0 ns |
| `quadtree_query_1000` | — | — | 436.9 ns |
| `octree_insert_1000` | — | — | 84815.0 ns |
| `octree_query_1000` | — | — | 753.8 ns |
| `spatial_hash_insert_1000` | — | — | 39893.0 ns |
| `spatial_hash_query_cell` | — | — | 23.85 ns |

## v05c

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `convex_hull_100` | — | — | 2032.9 ns |
| `gjk_intersect` | — | — | 27.19 ns |
| `gjk_no_intersect` | — | — | 23.05 ns |
| `gjk_epa_penetration` | — | — | 260.6 ns |

## v06

| Benchmark | Baseline (`9463cf0`) | Mid (`18bedd9`) | Current (`0dcccc3`) |
|-----------|------|------|------|
| `svd_3x3` | — | — | 868.9 ns |
| `svd_5x5` | — | — | 124170.0 ns |
| `matrix_inverse_3x3` | — | — | 242.3 ns |
| `pseudo_inverse_3x2` | — | — | 720.2 ns |
| `csr_spmv_100x100` | — | — | 239.7 ns |
| `svd_4x2_tall` | — | — | 562.1 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
