# Benchmarks

Latest: **2026-08-05T06:13:48Z** — commit `70e7c8b`

Tracking: `e1f8f4c` (baseline) → `151f9d9` (mid) → `70e7c8b` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 89117.0 ns **-53%** | 241931.0 ns +28% |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 96800.0 ns **-75%** | 251848.0 ns **-34%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 109162.0 ns +15% | 237390.0 ns +150% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 84159.0 ns **-6%** | 132209.0 ns +47% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 92121.0 ns **-15%** | 114261.0 ns +6% |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 74521.0 ns **-13%** | 137169.0 ns +59% |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 72006.0 ns +29% | 137448.0 ns +145% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 69841.0 ns +13% | 125156.0 ns +102% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 81295.0 ns +53% | 138215.0 ns +161% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 206102.0 ns +275% | 127879.0 ns +133% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 199956.0 ns +40% | 124527.0 ns **-13%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 32197.0 ns +19% | 21442.0 ns **-21%** |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 117263.0 ns +92% | 122153.0 ns +100% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 47911.0 ns +108% | 24724.0 ns +7% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 83321.0 ns +67% | 131022.0 ns +162% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 37086.0 ns +16% | 24584.0 ns **-23%** |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 36318.0 ns +45% | 28076.0 ns +12% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 40647.0 ns +69% | 23257.0 ns **-3%** |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 100012.0 ns +127% | 60063.0 ns +37% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 246400.0 ns +297% | 123200.0 ns +99% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 50914.0 ns +142% | 21931.0 ns +4% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 31010.0 ns +72% | 13759.0 ns **-24%** |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 27238.0 ns | 24863.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 27657.0 ns | 20185.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 174813.0 ns | 195905.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 93867.0 ns | 204285.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | 2141.0 µs | 4157.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | 1214.0 µs | 2009.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | — | 30693.0 µs |

## kdtree_build_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `kdtree_build_4k:` | — | — | 5158.0 µs |

## kdtree_radius_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `kdtree_radius_4k:` | — | — | 12432.0 ns |

## delaunay_2d_400:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `delaunay_2d_400:` | — | — | 3537.0 µs |

## delaunay_2d_circle_150:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150:` | — | — | 896622.0 ns |

## triangulate_600gon:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `triangulate_600gon:` | — | — | 3840.0 µs |

## svd_golub_kahan_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `svd_golub_kahan_12:` | — | — | 367644.0 ns |

## eigen_qr_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`70e7c8b`) |
|-----------|------|------|------|
| `eigen_qr_12:` | — | — | 281390.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
