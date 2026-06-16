# Benchmarks

Latest: **2026-06-16T01:23:49Z** — commit `7ae75ef`

Tracking: `e1f8f4c` (baseline) → `8a08c99` (mid) → `7ae75ef` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 207000.0 ns +10% | 202000.0 ns +7% |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 124000.0 ns **-68%** | 217000.0 ns **-43%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 124000.0 ns +31% | 137000.0 ns +44% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 138000.0 ns +53% | 76000.0 ns **-16%** |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 382000.0 ns +254% | 70000.0 ns **-35%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 59000.0 ns **-31%** | 204000.0 ns +137% |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 67000.0 ns +20% | 178000.0 ns +218% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 61000.0 ns | 68000.0 ns +10% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 75000.0 ns +42% | 87000.0 ns +64% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 60000.0 ns +9% | 171000.0 ns +211% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 78000.0 ns **-45%** | 71000.0 ns **-50%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 24000.0 ns **-11%** | 32000.0 ns +19% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 94000.0 ns +54% | 92000.0 ns +51% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 33000.0 ns +43% | 54000.0 ns +135% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 54000.0 ns +8% | 97000.0 ns +94% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 26000.0 ns **-19%** | 26000.0 ns **-19%** |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 58000.0 ns +132% | 38000.0 ns +52% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 23000.0 ns **-4%** | 36000.0 ns +50% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 49000.0 ns +11% | 487000.0 ns +1007% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 165000.0 ns +166% | 89000.0 ns +44% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 27000.0 ns +29% | 28000.0 ns +33% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 25000.0 ns +39% | 520000.0 ns +2789% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 21000.0 ns | 24000.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 38000.0 ns | 23000.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 133000.0 ns | 245000.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`7ae75ef`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 191000.0 ns | 160000.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
