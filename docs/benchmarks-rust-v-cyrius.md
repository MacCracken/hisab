# Benchmarks: Rust vs Cyrius

> Comparison of hisab v1.4.0 performance between the original Rust implementation
> and the Cyrius port. All times in nanoseconds unless noted.
>
> - **Rust**: criterion v0.5, `--all-features`, release mode, bench-history.csv from 2026-03-31
> - **Cyrius**: cc3 4.9.2, `bench.cyr` framework, static ELF, measured 2026-04-15
> - **Platform**: x86_64 Linux
>
> Rust uses f32 (glam) for transforms/geo. Cyrius uses f64 for everything (all math is
> f64 bit patterns via SSE2/x87). This means Cyrius does more work per operation but
> at higher precision.

## Core Operations

| Operation | Rust (ns) | Cyrius (ns) | Ratio | Notes |
|-----------|-----------|-------------|-------|-------|
| **Transforms** | | | | |
| transform2d_to_matrix | 6.6 | — | — | f32, glam SIMD |
| transform2d_apply_point | 6.6 | — | — | f32, glam SIMD |
| transform3d_to_matrix | 5.6 | — | — | f32, glam SIMD |
| transform3d_apply_point | 6.4 | — | — | f32, glam SIMD |
| t3d_compose | — | 684 | — | f64, heap-alloc |
| projection_perspective | 12.8 | — | — | f32 |
| lerp_f32 | 1.1 | — | — | f32 |
| lerp_vec3 | 2.8 | — | — | f32, SIMD |
| slerp | 21.2 | 704 | 33x | f32 SIMD vs f64 heap+trig |
| inverse_matrix (4x4) | 20.0 | 755 | 38x | f32 SIMD vs f64 Cramer |
| **Geometry** | | | | |
| ray_sphere_hit | 2.9 | 512 | 177x | f32 SIMD vs f64 heap |
| ray_plane_hit | 2.1 | — | — | f32 |
| ray_aabb_hit | 4.9 | 484 | 99x | f32 SIMD vs f64 slab |
| ray_triangle | 8.0 | 733 | 92x | f32 vs f64 Moller-Trumbore |
| aabb_aabb_overlap | 2.4 | — | — | f32, branch-free |
| sphere_sphere | 1.9 | — | — | f32 |
| closest_on_aabb | 2.8 | — | — | f32 |
| closest_on_sphere | 5.4 | — | — | f32 |
| gjk_intersect | 28.3 | — | — | f32 |
| gjk_epa_penetration | 156.2 | — | — | f32 |
| **Calculus** | | | | |
| derivative_x_squared | 1.2 | 483 | 403x | f64 both, but overhead |
| integral_simpson_100 | 142.2 | 6,000 | 42x | f64, fncall overhead |
| gauss_legendre_5 | 4.0 | — | — | |
| bezier_cubic_3d | 2.9 | — | — | f32 |
| catmull_rom | 2.7 | — | — | f32 |
| ease_in_out | 0.6 | 410 | 683x | f32 inline vs f64 fn call |
| ease_in_out_smooth | 0.8 | — | — | |
| **Numerical** | | | | |
| newton_sqrt2 | 6.7 | — | — | |
| bisection_sqrt2 | 112.5 | — | — | |
| gaussian_3x3 | 79.8 | — | — | |
| lu_decompose_3x3 | 112.0 | — | — | |
| cholesky_3x3 | 65.9 | — | — | |
| qr_decompose_3col | 135.6 | — | — | |
| eigenvalue_3x3 | 434.2 | — | — | |
| svd_3x3 | 829.7 | — | — | |
| **Number Theory** | | | | |
| gcd | — | 447 | — | integer ops |
| is_prime(1000003) | — | 1,000 | — | Miller-Rabin |
| **Complex** | | | | |
| cx_mul | — | 431 | — | f64 |
| **Color** | | | | |
| srgb_to_linear | — | 499 | — | f64 pow |
| tonemap_reinhard | — | 431 | — | f64 div |

## Analysis

### Why Cyrius is slower per-operation

The Rust version benefits from three factors the Cyrius port cannot match:

1. **SIMD (glam)** — Rust's glam uses SSE2/AVX for Vec3/Vec4/Mat4/Quat operations. A single `_mm_mul_ps` does 4 f32 multiplies in one cycle. Cyrius does scalar f64 operations.

2. **Stack allocation** — Rust's Vec3 is 12 bytes on the stack. Cyrius's HVec3 is a 24-byte heap allocation via `alloc()` + 3 `store64` calls per construction.

3. **f32 vs f64** — The Rust version uses f32 for all transforms/geo (matching GPU precision). Cyrius uses f64 for everything, which is 2x the memory bandwidth and often slower on x87/SSE2.

### Where the ratios come from

| Factor | Approximate cost |
|--------|-----------------|
| Heap alloc overhead | ~200-300ns per allocation |
| f64 vs f32 | ~1.5-2x per arithmetic op |
| No SIMD vectorization | ~2-4x for vector/matrix ops |
| Function call overhead | ~10-20ns per fn call |
| Combined (typical) | ~30-100x for simple ops |

### Where Cyrius wins

- **Binary size**: 269KB static vs ~800KB dynamic (3x smaller)
- **Build time**: instant vs seconds
- **Precision**: f64 everywhere (1e-12 tolerance vs 1e-7 for f32)
- **Dependencies**: 1 (sakshi) vs 9 crates
- **Simplicity**: 6,674 lines of readable Cyrius vs 33,612 lines of Rust

### Batch/throughput perspective

The per-operation overhead is dominated by allocation. In batch processing (e.g., transforming 10K vertices), the allocation pattern can be amortized with arena allocators. The Rust version's batch benchmarks show:

| Batch operation | Rust (ns) | Per-item (ns) |
|----------------|-----------|---------------|
| ray_sphere × 100 | 201 | 2.0 |
| aabb_contains × 100 | 135 | 1.4 |
| transform3d × 100 | 388 | 3.9 |
| simpson_sin × 10000 | 92,259 | 9.2 |

Cyrius batch operations would need arena allocation patterns to approach these numbers.

## Recommendations

1. **Hot paths**: For consumers like kiran (engine) that need sub-microsecond transforms, use Cyrius for logic/setup and hand-optimize hot inner loops
2. **Arena allocators**: Batch operations should use `arena_new/arena_alloc/arena_reset` to amortize allocation
3. **Future**: When Cyrius gets SIMD intrinsics (roadmap 5.x), the gap will narrow significantly for vectorizable operations
4. **Precision trade**: The f64 precision is a feature, not a bug — mimamsa (theoretical physics) and kana (quantum) need it
