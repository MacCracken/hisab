# Benchmarks

Latest: **2026-08-04T23:44:49Z** — commit `b018c61`

Tracking: `e1f8f4c` (baseline) → `151f9d9` (mid) → `b018c61` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 100712.0 ns **-47%** | 197092.0 ns +4% |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 161403.0 ns **-58%** | 205613.0 ns **-46%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 108813.0 ns +15% | 207848.0 ns +119% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 63695.0 ns **-29%** | 97778.0 ns +9% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 84089.0 ns **-22%** | 68095.0 ns **-37%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 74311.0 ns **-14%** | 78641.0 ns **-9%** |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 96940.0 ns +73% | 83670.0 ns +49% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 93029.0 ns +50% | 80248.0 ns +29% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 80457.0 ns +52% | 74031.0 ns +40% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 81504.0 ns +48% | 88070.0 ns +60% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 90584.0 ns **-37%** | 73823.0 ns **-48%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 46584.0 ns +73% | 38204.0 ns +41% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 101898.0 ns +67% | 142895.0 ns +134% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 40997.0 ns +78% | 27587.0 ns +20% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 86813.0 ns +74% | 117822.0 ns +136% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 29054.0 ns **-9%** | 23117.0 ns **-28%** |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 45816.0 ns +83% | 79898.0 ns +220% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 47771.0 ns +99% | 31499.0 ns +31% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 291517.0 ns +563% | 77384.0 ns +76% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 66140.0 ns +7% | 61460.0 ns |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 61740.0 ns +194% | 46375.0 ns +121% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 32896.0 ns +83% | 16692.0 ns **-7%** |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 39950.0 ns | 23816.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 47632.0 ns | 30590.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 102107.0 ns | 211828.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 120336.0 ns | 117822.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | 22068.0 µs | 1896.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | 190125.0 µs | 1029.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | — | 15588.0 µs |

## kdtree_build_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `kdtree_build_4k:` | — | — | 2584.0 µs |

## kdtree_radius_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `kdtree_radius_4k:` | — | — | 10336.0 ns |

## delaunay_2d_400:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `delaunay_2d_400:` | — | — | 1672.0 µs |

## delaunay_2d_circle_150:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150:` | — | — | 466330.0 ns |

## triangulate_600gon:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `triangulate_600gon:` | — | — | 1686.0 µs |

## svd_golub_kahan_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `svd_golub_kahan_12:` | — | — | 172229.0 ns |

## eigen_qr_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`b018c61`) |
|-----------|------|------|------|
| `eigen_qr_12:` | — | — | 127810.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
