# Benchmarks

Latest: **2026-05-29T16:54:40Z** — commit `b1165f9`

Tracking: `e1f8f4c` (baseline) → `8a08c99` (mid) → `b1165f9` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 207000.0 ns +10% | 445000.0 ns +135% |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 124000.0 ns **-68%** | 141000.0 ns **-63%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 124000.0 ns +31% | 94000.0 ns |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 138000.0 ns +53% | 59000.0 ns **-34%** |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 382000.0 ns +254% | 52000.0 ns **-52%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 59000.0 ns **-31%** | 59000.0 ns **-31%** |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 67000.0 ns +20% | 72000.0 ns +29% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 61000.0 ns | 47000.0 ns **-24%** |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 75000.0 ns +42% | 57000.0 ns +8% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 60000.0 ns +9% | 60000.0 ns +9% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 78000.0 ns **-45%** | 54000.0 ns **-62%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 24000.0 ns **-11%** | 28000.0 ns +4% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 94000.0 ns +54% | 84000.0 ns +38% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 33000.0 ns +43% | 23000.0 ns |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 54000.0 ns +8% | 53000.0 ns +6% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 26000.0 ns **-19%** | 21000.0 ns **-34%** |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 58000.0 ns +132% | 20000.0 ns **-20%** |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 23000.0 ns **-4%** | 23000.0 ns **-4%** |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 49000.0 ns +11% | 57000.0 ns +30% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 165000.0 ns +166% | 53000.0 ns **-15%** |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 27000.0 ns +29% | 19000.0 ns **-10%** |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 25000.0 ns +39% | 29000.0 ns +61% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 21000.0 ns | 35000.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 38000.0 ns | 17000.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 133000.0 ns | 160000.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`b1165f9`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 191000.0 ns | 107000.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
