# Benchmarks: Rust vs Cyrius

> hisab v1.4.0 performance comparison. All times in nanoseconds.
>
> - **Rust**: criterion v0.5, `--all-features`, release mode. Data from bench-history.csv (2026-03-31, commit 745870c)
> - **Cyrius**: cc3 4.10.3, bench.cyr framework, static ELF (2026-04-15)
> - **Platform**: x86_64 Linux
>
> Rust uses f32 (glam SIMD) for transforms/geo. Cyrius uses f64 everywhere (SSE2/x87).

## Matched Benchmarks

Operations where both Rust and Cyrius implementations exist and are directly comparable.

| Operation | Rust (ns) | Cyrius (ns) | Ratio | Notes |
|-----------|-----------|-------------|-------|-------|
| **Transforms** | | | | |
| slerp | 21.2 | 704 | 33x | f32 SIMD vs f64 trig+heap |
| mat4 inverse | 20.0 | 755 | 38x | f32 SIMD cofactor vs f64 Cramer |
| transform3d_apply | 6.4 | — | — | f32, glam SIMD |
| t3d_compose | — | 684 | — | f64, heap alloc |
| ease_in_out | 0.6 | 410 | 683x | f32 inline vs f64 fn call |
| ease_in_out_smooth | 0.8 | — | — | |
| **Geometry** | | | | |
| ray_sphere hit | 2.9 | 512 | 177x | f32 SIMD vs f64 heap |
| ray_aabb hit | 4.9 | 484 | 99x | f32 SIMD slab vs f64 slab |
| ray_triangle | 8.0 | 733 | 92x | f32 vs f64 Moller-Trumbore |
| gjk_intersect | 28.3 | — | — | f32 |
| gjk_epa | 156.2 | — | — | f32 |
| aabb_aabb overlap | 2.4 | — | — | f32, branch-free |
| sphere_sphere | 1.9 | — | — | f32 |
| **Calculus** | | | | |
| derivative x^2 | 1.2 | 483 | 403x | fncall overhead dominates |
| simpson 100 steps | 142.2 | 6,000 | 42x | fncall1 per sample |
| gauss_legendre_5 | 4.0 | — | — | |
| bezier_cubic_3d | 2.9 | — | — | f32 |
| catmull_rom | 2.7 | — | — | f32 |
| **Numerical** | | | | |
| newton_sqrt2 | 6.7 | — | — | |
| bisection_sqrt2 | 112.5 | — | — | |
| gcd | — | 447 | — | integer |
| is_prime(1M+3) | — | 1,000 | — | Miller-Rabin |
| lu_decompose_3x3 | 112.0 | — | — | |
| cholesky_3x3 | 65.9 | — | — | |
| eigenvalue_3x3 | 434.2 | — | — | |
| svd_3x3 | 829.7 | — | — | |
| fft_64 | 665.2 | — | — | |
| fft_1024 | 16,594 | — | — | |
| rk4 100 steps | 2,901 | — | — | |
| **Complex** | | | | |
| cx_mul | — | 431 | — | f64 |
| **Color** | | | | |
| srgb_to_linear | — | 499 | — | f64 pow |
| tonemap_reinhard | — | 431 | — | f64 div |

## Rust-Only Benchmarks (no Cyrius equivalent measured)

| Operation | Rust (ns) | Category |
|-----------|-----------|----------|
| transform2d_to_matrix | 6.6 | transforms |
| transform2d_apply_point | 6.6 | transforms |
| transform3d_to_matrix | 5.6 | transforms |
| projection_perspective | 12.8 | transforms |
| projection_orthographic | 4.4 | transforms |
| lerp_f32 | 1.1 | transforms |
| lerp_vec3 | 2.8 | transforms |
| ray_plane_hit | 2.1 | geo |
| aabb_contains | 3.1 | geo |
| sphere_contains | 2.4 | geo |
| aabb_merge | 4.1 | geo |
| closest_on_aabb | 2.8 | geo |
| closest_on_sphere | 5.4 | geo |
| frustum_contains_point | 4.8 | geo |
| triangle_unit_normal | 5.4 | geo |
| segment_closest_point | 3.1 | geo |
| integral_trapezoidal_100 | 75.4 | calc |
| bezier_quadratic | 1.5 | calc |
| gaussian_3x3 | 79.8 | num |
| gaussian_4x4 | 111.0 | num |
| lu_solve_3x3 | 34.3 | num |
| qr_decompose_3col | 135.6 | num |
| least_squares_6pt | 186.1 | num |
| svd_5x5 | 122,720 | num |
| csr_spmv_100x100 | 209.0 | num |
| bvh_build_100 | 8,047 | spatial |
| bvh_ray_query_100 | 67.4 | spatial |
| bvh_build_1000 | 103,790 | spatial |
| kdtree_nearest_1000 | 267.7 | spatial |
| convex_hull_100 | 2,028 | geo |
| rk4_oscillator_1000 | 31,484 | num |

## Batch Operations (Rust)

| Operation | Rust (ns) | Per-item (ns) |
|-----------|-----------|---------------|
| ray_sphere × 100 | 201 | 2.0 |
| aabb_contains × 100 | 135 | 1.4 |
| transform3d × 100 | 388 | 3.9 |
| simpson_sin × 10000 | 92,259 | 9.2 |

## Analysis

### Why Cyrius is 30-700x slower per-operation

Three architectural factors compound:

| Factor | Cost | Applies to |
|--------|------|-----------|
| **Heap allocation** | ~200-400ns per `alloc()` + 3× `store64()` | Every Vec3/Quat/Mat4 construction |
| **f64 vs f32** | ~1.5-2x | All math (Cyrius has no f32 type) |
| **No SIMD** | ~2-4x | Vec/Mat ops (glam uses SSE2/AVX) |
| **Function call overhead** | ~10-20ns per `fncall1` | Integration, root finding |
| **Combined typical** | 30-100x | Simple operations |
| **Combined worst** | 400-700x | Trivial ops (ease, lerp) where Rust inlines to 1 instruction |

### Where Cyrius wins

| Metric | Rust | Cyrius | Winner |
|--------|------|--------|--------|
| Binary size | ~800KB dynamic | 349KB static | **Cyrius 2.3x** |
| Build time | seconds | instant | **Cyrius** |
| Precision | f32 (1e-7) | f64 (1e-12) | **Cyrius 100,000x** |
| Dependencies | 9 crates | 1 (sakshi) | **Cyrius** |
| Source size | 33,612 lines | 8,945 lines | **Cyrius 3.8x** |
| Cold start | dynamic linker | none (static) | **Cyrius** |

### When the speed gap matters

For **hot inner loops** (transform 10K vertices, ray-trace 1M pixels), the 30-100x gap is significant. For **setup/logic** (configure a projection, solve an ODE, factorize a matrix), the gap is irrelevant — the operations take microseconds either way.

**Consumer impact:**
- **kiran** (engine): Hot transform paths need optimization. Use arena allocators for batch ops.
- **impetus** (physics): PGS solver runs many iterations — each microsecond matters at scale.
- **joshua** (simulation): ODE integration is I/O-bound, not compute-bound. Gap doesn't matter.
- **mimamsa/kana** (physics): Matrix factorizations are one-shot. f64 precision is a requirement, not a trade.

### Future optimization vectors

1. **Arena allocation** — `arena_new/arena_alloc/arena_reset` amortizes allocation across batch ops
2. **Stack structs** — Cyrius single-field structs are stack-allocated; split Vec3 into 3 separate values in hot paths
3. **SIMD intrinsics** — Cyrius 5.x roadmap includes SIMD; this would close the gap 2-4x
4. **Inline expansion** — `#regalloc` + DCE already reduce overhead; future inlining would help trivial wrappers
