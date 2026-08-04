# Benchmarks

Latest: **2026-08-04T03:44:50Z** — commit `565e278`

Tracking: `e1f8f4c` (baseline) → `5331f05` (mid) → `565e278` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 81435.0 ns **-57%** | 88209.0 ns **-53%** |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 81365.0 ns **-79%** | 289562.0 ns **-25%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 108673.0 ns +14% | 211759.0 ns +123% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 148064.0 ns +65% | 257156.0 ns +186% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 56781.0 ns **-47%** | 203588.0 ns +89% |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 88140.0 ns | 208197.0 ns +142% |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 82134.0 ns +47% | 161753.0 ns +189% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 73333.0 ns +18% | 97638.0 ns +57% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 209105.0 ns +295% | 407733.0 ns +669% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 89537.0 ns +63% | 204565.0 ns +272% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 81086.0 ns **-43%** | 94915.0 ns **-34%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 52521.0 ns +95% | 44349.0 ns +64% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 70121.0 ns +15% | 129835.0 ns +113% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 29612.0 ns +29% | 32406.0 ns +41% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 429873.0 ns +760% | 87511.0 ns +75% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 629340.0 ns +1867% | 49937.0 ns +56% |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 42673.0 ns +71% | 34711.0 ns +39% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 42743.0 ns +78% | 32895.0 ns +37% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 253384.0 ns +476% | 64813.0 ns +47% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 97848.0 ns +58% | 209874.0 ns +239% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 33804.0 ns +61% | 30032.0 ns +43% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 26958.0 ns +50% | 26680.0 ns +48% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 20603.0 ns | 47632.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 22420.0 ns | 28635.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 188990.0 ns | 208546.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 104343.0 ns | 146178.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | — | 2040.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | — | 1050.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`565e278`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | — | 16266.0 µs |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
