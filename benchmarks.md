# Benchmarks

Latest: **2026-08-04T01:06:51Z** — commit `151f9d9`

Tracking: `e1f8f4c` (baseline) → `7ae75ef` (mid) → `151f9d9` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 202000.0 ns +7% | 89117.0 ns **-53%** |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 217000.0 ns **-43%** | 96800.0 ns **-75%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 137000.0 ns +44% | 109162.0 ns +15% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 76000.0 ns **-16%** | 84159.0 ns **-6%** |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 70000.0 ns **-35%** | 92121.0 ns **-15%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 204000.0 ns +137% | 74521.0 ns **-13%** |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 178000.0 ns +218% | 72006.0 ns +29% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 68000.0 ns +10% | 69841.0 ns +13% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 87000.0 ns +64% | 81295.0 ns +53% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 171000.0 ns +211% | 206102.0 ns +275% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 71000.0 ns **-50%** | 199956.0 ns +40% |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 32000.0 ns +19% | 32197.0 ns +19% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 92000.0 ns +51% | 117263.0 ns +92% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 54000.0 ns +135% | 47911.0 ns +108% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 97000.0 ns +94% | 83321.0 ns +67% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 26000.0 ns **-19%** | 37086.0 ns +16% |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 38000.0 ns +52% | 36318.0 ns +45% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 36000.0 ns +50% | 40647.0 ns +69% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 487000.0 ns +1007% | 100012.0 ns +127% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 89000.0 ns +44% | 246400.0 ns +297% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 28000.0 ns +33% | 50914.0 ns +142% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 520000.0 ns +2789% | 31010.0 ns +72% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 24000.0 ns | 27238.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 23000.0 ns | 27657.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 245000.0 ns | 174813.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 160000.0 ns | 93867.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | — | 2141.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`7ae75ef`) | Current (`151f9d9`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | — | 1214.0 µs |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
