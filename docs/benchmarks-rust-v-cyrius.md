# Benchmarks: Rust vs Cyrius

> hisab v2.2.0 benchmark comparison.
>
> - **Rust**: criterion v0.5, release mode. Final run from bench-history.csv (2026-03-31, commit 745870c). f32 via glam SIMD.
> - **Cyrius**: cc3 4.10.3, bench.cyr. Run 2026-04-15. f64 via SSE2/x87, heap-allocated types.
> - **Platform**: x86_64 Linux

## Head-to-Head

| Operation | Rust (ns) | Cyrius (ns) | Ratio | Notes |
|-----------|-----------|-------------|-------|-------|
| **Transforms** | | | | |
| slerp | 21.1 | 680 | 32x | f32 SIMD vs f64 trig+heap |
| mat4 inverse | 20.1 | 745 | 37x | f32 SIMD vs f64 Cramer |
| t3d_compose | -- | 661 | -- | f64 only |
| ease_in_out | 0.60 | 403 | 672x | f32 inline vs f64 fn call |
| **Geometry** | | | | |
| ray_sphere | 2.9 | 492 | 170x | f32 SIMD vs f64 |
| ray_aabb | 5.3 | 475 | 90x | f32 slab vs f64 slab |
| ray_triangle | 8.0 | 698 | 87x | f32 vs f64 Moller-Trumbore |
| gjk_intersect | 28.9 | -- | -- | f32 only |
| gjk_epa | 155.7 | -- | -- | f32 only |
| **Calculus** | | | | |
| derivative | 1.2 | 459 | 383x | fncall overhead |
| simpson_100 | 142.1 | 5,000 | 35x | fncall1 per sample |
| gauss_legendre_5 | 3.9 | -- | -- | |
| **Numerical** | | | | |
| gcd | -- | 433 | -- | integer |
| is_prime(1M+3) | -- | 1,000 | -- | Miller-Rabin |
| cx_mul | -- | 426 | -- | f64 |
| **Color** | | | | |
| srgb_to_linear | -- | 484 | -- | f64 pow |
| tonemap_reinhard | -- | 421 | -- | f64 |

## Full Rust Benchmark Set (90 benchmarks)

### Core transforms

| Benchmark | Rust (ns) |
|-----------|-----------|
| transform3d_to_matrix | 5.79 |
| transform3d_apply_point | 6.42 |
| projection_perspective | 12.88 |
| projection_orthographic | 4.49 |
| lerp_f32 | 1.07 |
| lerp_vec3 | 2.79 |

### Core geometry

| Benchmark | Rust (ns) |
|-----------|-----------|
| ray_sphere_hit | 2.94 |
| ray_sphere_miss | 2.92 |
| ray_plane_hit | 2.17 |
| ray_aabb_hit | 5.27 |
| aabb_contains | 3.14 |
| sphere_contains | 2.39 |
| aabb_merge | 4.31 |

### Calculus

| Benchmark | Rust (ns) |
|-----------|-----------|
| derivative_x_squared | 1.17 |
| integral_simpson_100 | 142.1 |
| integral_simpson_1000 | 1,366.9 |
| integral_trapezoidal_100 | 75.5 |
| integral_trapezoidal_1000 | 773.9 |
| bezier_quadratic | 1.53 |
| bezier_cubic | 2.55 |

### Numerical

| Benchmark | Rust (ns) |
|-----------|-----------|
| newton_sqrt2 | 6.62 |
| bisection_sqrt2 | 111.9 |
| gaussian_3x3 | 81.0 |
| gaussian_4x4 | 109.1 |

### v02 -- Geo + transforms extended

| Benchmark | Rust (ns) |
|-----------|-----------|
| ray_triangle | 8.04 |
| aabb_aabb_overlap | 2.37 |
| sphere_sphere_overlap | 1.90 |
| frustum_contains_point | 4.77 |
| frustum_contains_aabb | 4.60 |
| slerp | 21.09 |
| transform3d_lerp | 25.10 |
| closest_on_aabb | 2.73 |
| segment_closest_point | 3.14 |
| plane_plane_intersection | 7.62 |
| triangle_unit_normal | 5.45 |
| line_closest_point | 2.26 |
| closest_on_sphere | 5.35 |
| inverse_matrix | 20.10 |

### v03 -- Curves + integration

| Benchmark | Rust (ns) |
|-----------|-----------|
| bezier_cubic_3d | 2.95 |
| de_casteljau_split | 6.19 |
| catmull_rom | 2.71 |
| bspline_cubic | 15.65 |
| gauss_legendre_5 | 3.86 |
| gauss_legendre_10_panels | 429.3 |
| arc_length_100 | 643.2 |
| ease_in_out | 0.60 |
| ease_in_out_smooth | 0.84 |

### v04a -- Linear algebra

| Benchmark | Rust (ns) |
|-----------|-----------|
| lu_decompose_3x3 | 107.4 |
| lu_solve_3x3 | 33.5 |
| cholesky_3x3 | 69.3 |
| cholesky_solve_3x3 | 39.6 |
| qr_decompose_3col | 134.2 |
| least_squares_linear_6pt | 186.0 |

### v04b -- Spectral + eigen

| Benchmark | Rust (ns) |
|-----------|-----------|
| eigenvalue_3x3 | 434.6 |
| fft_64 | 663.0 |
| fft_1024 | 16,515 |
| fft_ifft_256 | 6,828 |
| dst_64 | 41,239 |
| dct_64 | 43,031 |

### v04c -- ODE

| Benchmark | Rust (ns) |
|-----------|-----------|
| rk4_exp_100_steps | 2,907 |
| rk4_exp_1000_steps | 28,612 |
| rk4_oscillator_1000 | 31,551 |

### v05a -- Spatial (BVH, k-d tree)

| Benchmark | Rust (ns) |
|-----------|-----------|
| bvh_build_100 | 8,047 |
| bvh_ray_query_100 | 68.1 |
| bvh_build_1000 | 102,210 |
| kdtree_build_1000 | 114,830 |
| kdtree_nearest_1000 | 267.8 |
| kdtree_radius_1000 | 1,567 |

### v05b -- Spatial (quadtree, octree, hash)

| Benchmark | Rust (ns) |
|-----------|-----------|
| quadtree_insert_1000 | 76,370 |
| quadtree_query_1000 | 416.8 |
| octree_insert_1000 | 87,964 |
| octree_query_1000 | 766.9 |
| spatial_hash_insert_1000 | 43,978 |
| spatial_hash_query_cell | 24.2 |

### v05c -- Collision

| Benchmark | Rust (ns) |
|-----------|-----------|
| convex_hull_100 | 2,242 |
| gjk_intersect | 28.9 |
| gjk_no_intersect | 22.2 |
| gjk_epa_penetration | 155.7 |

### v06 -- Advanced linalg

| Benchmark | Rust (ns) |
|-----------|-----------|
| svd_3x3 | 761.6 |
| svd_5x5 | 122,720 |
| matrix_inverse_3x3 | 270.1 |
| pseudo_inverse_3x2 | 587.9 |
| csr_spmv_100x100 | 237.6 |
| svd_4x2_tall | 558.3 |

### Batch

| Benchmark | Rust (ns) | Per-item |
|-----------|-----------|----------|
| ray_sphere x 100 | 200.9 | 2.0 |
| aabb_contains x 100 | 135.3 | 1.4 |
| transform3d x 100 | 383.4 | 3.8 |
| simpson_sin x 10000 | 92,282 | 9.2 |

## Full Cyrius Benchmark Set (21 benchmarks, 2026-04-15)

| Benchmark | Avg (ns) | Min (ns) | Iterations |
|-----------|---------|---------|------------|
| vec3_add | 434 | 400 | 1,000,000 |
| vec3_cross | 447 | 410 | 1,000,000 |
| vec3_normalize | 458 | 420 | 1,000,000 |
| quat_mul | 474 | 420 | 1,000,000 |
| quat_slerp | 680 | 641 | 500,000 |
| quat_rotate_vec3 | 457 | 420 | 1,000,000 |
| m4_mul | 1,000 | 1,000 | 500,000 |
| m4_inverse | 745 | 641 | 200,000 |
| m4_transform_point | 581 | 510 | 1,000,000 |
| t3d_compose | 661 | 561 | 500,000 |
| ray_sphere | 492 | 460 | 1,000,000 |
| ray_aabb | 475 | 450 | 1,000,000 |
| ray_triangle | 698 | 611 | 1,000,000 |
| srgb_to_linear | 484 | 460 | 1,000,000 |
| tonemap_reinhard | 421 | 390 | 1,000,000 |
| calc_derivative | 459 | 440 | 500,000 |
| calc_simpson_100 | 5,000 | 5,000 | 100,000 |
| num_gcd | 433 | 410 | 1,000,000 |
| num_is_prime | 1,000 | 1,000 | 500,000 |
| cx_mul | 426 | 400 | 1,000,000 |
| ease_in_out | 403 | 380 | 1,000,000 |

## Analysis

### Why Cyrius is 30-700x slower per-operation

| Factor | Cost | When |
|--------|------|------|
| Heap allocation | ~200-400ns per alloc+store | Every Vec3/Quat/Mat4 |
| f64 vs f32 | ~1.5-2x | All math |
| No SIMD | ~2-4x | Vector/matrix ops |
| fncall overhead | ~10-20ns | Integration, root finding |
| Combined typical | 30-100x | Simple vector/matrix ops |
| Combined worst | 400-700x | Trivial ops (ease, lerp) |

### Where Cyrius wins

| Metric | Rust | Cyrius |
|--------|------|--------|
| Binary | ~800KB dynamic | 511KB static |
| Build | seconds | instant |
| Precision | f32 (1e-7) | f64 (1e-12) |
| Dependencies | 9 crates | 1 (sakshi) |
| Source | 33,612 lines | 15,676 lines |

### Optimization vectors for future versions

1. **Arena allocation** -- amortize alloc across batch ops
2. **Stack structs** -- Cyrius single-field structs are stack-allocated
3. **SIMD** -- Cyrius 5.x roadmap; would close gap 2-4x
4. **Inline expansion** -- `#regalloc` + DCE already help; future inlining would help trivial wrappers
