# Benchmarks

Latest: **2026-08-04T21:50:34Z** — commit `6e93b87`

Tracking: `e1f8f4c` (baseline) → `ee8032c` (mid) → `6e93b87` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 203517.0 ns +8% | 109092.0 ns **-42%** |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 206731.0 ns **-46%** | 158540.0 ns **-59%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 215879.0 ns +127% | 96731.0 ns |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 93028.0 ns +3% | 63486.0 ns **-29%** |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 74031.0 ns **-31%** | 58667.0 ns **-46%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 91632.0 ns +7% | 72984.0 ns **-15%** |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 109790.0 ns +96% | 126692.0 ns +126% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 98756.0 ns +59% | 86324.0 ns +39% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 94844.0 ns +79% | 81505.0 ns +54% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 194439.0 ns +254% | 64813.0 ns +18% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 99873.0 ns **-30%** | 77873.0 ns **-46%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 32895.0 ns +22% | 31499.0 ns +17% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 112654.0 ns +85% | 81505.0 ns +34% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 37575.0 ns +63% | 28635.0 ns +24% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 221466.0 ns +343% | 79759.0 ns +60% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 30800.0 ns **-4%** | 390972.0 ns +1122% |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 32896.0 ns +32% | 50705.0 ns +103% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 35759.0 ns +49% | 28496.0 ns +19% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 67886.0 ns +54% | 68165.0 ns +55% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 210502.0 ns +240% | 80178.0 ns +29% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 32406.0 ns +54% | 34781.0 ns +66% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 30031.0 ns +67% | 20743.0 ns +15% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 24375.0 ns | 31988.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 24444.0 ns | 24095.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 222864.0 ns | 158819.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 100292.0 ns | 85835.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | — | 2021.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | — | 1168.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | — | 16235.0 µs |

## kdtree_build_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `kdtree_build_4k:` | — | — | 2678.0 µs |

## kdtree_radius_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `kdtree_radius_4k:` | — | — | 4400.0 ns |

## delaunay_2d_400:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `delaunay_2d_400:` | — | — | 14799.0 µs |

## triangulate_600gon:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `triangulate_600gon:` | — | — | 1689.0 µs |

## svd_golub_kahan_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `svd_golub_kahan_12:` | — | — | 182565.0 ns |

## eigen_qr_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`6e93b87`) |
|-----------|------|------|------|
| `eigen_qr_12:` | — | — | 139194.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
