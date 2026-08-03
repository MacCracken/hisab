# Benchmarks

Latest: **2026-08-03T23:43:55Z** — commit `ee8032c`

Tracking: `e1f8f4c` (baseline) → `b1165f9` (mid) → `ee8032c` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 445000.0 ns +135% | 203517.0 ns +8% |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 141000.0 ns **-63%** | 206731.0 ns **-46%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 94000.0 ns | 215879.0 ns +127% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 59000.0 ns **-34%** | 93028.0 ns +3% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 52000.0 ns **-52%** | 74031.0 ns **-31%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 59000.0 ns **-31%** | 91632.0 ns +7% |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 72000.0 ns +29% | 109790.0 ns +96% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 47000.0 ns **-24%** | 98756.0 ns +59% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 57000.0 ns +8% | 94844.0 ns +79% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 60000.0 ns +9% | 194439.0 ns +254% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 54000.0 ns **-62%** | 99873.0 ns **-30%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 28000.0 ns +4% | 32895.0 ns +22% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 84000.0 ns +38% | 112654.0 ns +85% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 23000.0 ns | 37575.0 ns +63% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 53000.0 ns +6% | 221466.0 ns +343% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 21000.0 ns **-34%** | 30800.0 ns **-4%** |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 20000.0 ns **-20%** | 32896.0 ns +32% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 23000.0 ns **-4%** | 35759.0 ns +49% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 57000.0 ns +30% | 67886.0 ns +54% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 53000.0 ns **-15%** | 210502.0 ns +240% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 19000.0 ns **-10%** | 32406.0 ns +54% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 29000.0 ns +61% | 30031.0 ns +67% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 35000.0 ns | 24375.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 17000.0 ns | 24444.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 160000.0 ns | 222864.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`ee8032c`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 107000.0 ns | 100292.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
