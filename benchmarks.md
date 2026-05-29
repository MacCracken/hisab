# Benchmarks

Latest: **2026-05-29T00:56:12Z** — commit `8a08c99`

Tracking: `e1f8f4c` (baseline) → `8a08c99` (mid) → `8a08c99` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 121000.0 ns **-36%** | 207000.0 ns +10% |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 203000.0 ns **-47%** | 124000.0 ns **-68%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 170000.0 ns +79% | 124000.0 ns +31% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 201000.0 ns +123% | 138000.0 ns +53% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 134000.0 ns +24% | 382000.0 ns +254% |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 324000.0 ns +277% | 59000.0 ns **-31%** |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 158000.0 ns +182% | 67000.0 ns +20% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 166000.0 ns +168% | 61000.0 ns |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 536000.0 ns +911% | 75000.0 ns +42% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 437000.0 ns +695% | 60000.0 ns +9% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 89000.0 ns **-38%** | 78000.0 ns **-45%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 30000.0 ns +11% | 24000.0 ns **-11%** |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 70000.0 ns +15% | 94000.0 ns +54% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 444000.0 ns +1830% | 33000.0 ns +43% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 51000.0 ns | 54000.0 ns +8% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 149000.0 ns +366% | 26000.0 ns **-19%** |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 29000.0 ns +16% | 58000.0 ns +132% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 44000.0 ns +83% | 23000.0 ns **-4%** |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 754000.0 ns +1614% | 49000.0 ns +11% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 84000.0 ns +35% | 165000.0 ns +166% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 33000.0 ns +57% | 27000.0 ns +29% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 17000.0 ns **-6%** | 25000.0 ns +39% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | — | 21000.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | — | 38000.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | — | 133000.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`8a08c99`) | Current (`8a08c99`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | — | 191000.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
