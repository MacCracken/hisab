# Benchmarks

Latest: **2026-08-03T22:08:00Z** — commit `5331f05`

Tracking: `e1f8f4c` (baseline) → `b1165f9` (mid) → `5331f05` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 445000.0 ns +135% | 81435.0 ns **-57%** |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 141000.0 ns **-63%** | 81365.0 ns **-79%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 94000.0 ns | 108673.0 ns +14% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 59000.0 ns **-34%** | 148064.0 ns +65% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 52000.0 ns **-52%** | 56781.0 ns **-47%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 59000.0 ns **-31%** | 88140.0 ns |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 72000.0 ns +29% | 82134.0 ns +47% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 47000.0 ns **-24%** | 73333.0 ns +18% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 57000.0 ns +8% | 209105.0 ns +295% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 60000.0 ns +9% | 89537.0 ns +63% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 54000.0 ns **-62%** | 81086.0 ns **-43%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 28000.0 ns +4% | 52521.0 ns +95% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 84000.0 ns +38% | 70121.0 ns +15% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 23000.0 ns | 29612.0 ns +29% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 53000.0 ns +6% | 429873.0 ns +760% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 21000.0 ns **-34%** | 629340.0 ns +1867% |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 20000.0 ns **-20%** | 42673.0 ns +71% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 23000.0 ns **-4%** | 42743.0 ns +78% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 57000.0 ns +30% | 253384.0 ns +476% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 53000.0 ns **-15%** | 97848.0 ns +58% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 19000.0 ns **-10%** | 33804.0 ns +61% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 29000.0 ns +61% | 26958.0 ns +50% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 35000.0 ns | 20603.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 17000.0 ns | 22420.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 160000.0 ns | 188990.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`b1165f9`) | Current (`5331f05`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 107000.0 ns | 104343.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
