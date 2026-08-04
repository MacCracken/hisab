# Benchmarks

Latest: **2026-08-04T22:04:01Z** — commit `97bed43`

Tracking: `e1f8f4c` (baseline) → `151f9d9` (mid) → `97bed43` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 100712.0 ns **-47%** | 127111.0 ns **-33%** |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 161403.0 ns **-58%** | 141429.0 ns **-63%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 108813.0 ns +15% | 45676.0 ns **-52%** |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 63695.0 ns **-29%** | 138285.0 ns +54% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 84089.0 ns **-22%** | 55314.0 ns **-49%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 74311.0 ns **-14%** | 110070.0 ns +28% |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 96940.0 ns +73% | 124039.0 ns +121% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 93029.0 ns +50% | 134374.0 ns +117% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 80457.0 ns +52% | 132000.0 ns +149% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 81504.0 ns +48% | 149251.0 ns +171% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 90584.0 ns **-37%** | 139752.0 ns |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 46584.0 ns +73% | 51473.0 ns +91% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 101898.0 ns +67% | 139263.0 ns +128% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 40997.0 ns +78% | 56223.0 ns +144% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 86813.0 ns +74% | 60971.0 ns +22% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 29054.0 ns **-9%** | 27867.0 ns **-13%** |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 45816.0 ns +83% | 32407.0 ns +30% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 47771.0 ns +99% | 29053.0 ns +21% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 291517.0 ns +563% | 87092.0 ns +98% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 66140.0 ns +7% | 197022.0 ns +218% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 61740.0 ns +194% | 28635.0 ns +36% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 32896.0 ns +83% | 30032.0 ns +67% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 39950.0 ns | 19067.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 47632.0 ns | 25213.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 102107.0 ns | 206241.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 120336.0 ns | 97778.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | 22068.0 µs | 2025.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | 190125.0 µs | 1063.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | — | 16057.0 µs |

## kdtree_build_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `kdtree_build_4k:` | — | — | 2677.0 µs |

## kdtree_radius_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `kdtree_radius_4k:` | — | — | 4400.0 ns |

## delaunay_2d_400:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `delaunay_2d_400:` | — | — | 14908.0 µs |

## triangulate_600gon:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `triangulate_600gon:` | — | — | 1698.0 µs |

## svd_golub_kahan_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `svd_golub_kahan_12:` | — | — | 183822.0 ns |

## eigen_qr_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`97bed43`) |
|-----------|------|------|------|
| `eigen_qr_12:` | — | — | 138915.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
