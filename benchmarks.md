# Benchmarks

Latest: **2026-08-04T19:08:49Z** — commit `26429cc`

Tracking: `e1f8f4c` (baseline) → `ee8032c` (mid) → `26429cc` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 203517.0 ns +8% | 171111.0 ns **-9%** |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 206731.0 ns **-46%** | 109650.0 ns **-71%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 215879.0 ns +127% | 366667.0 ns +286% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 93028.0 ns +3% | 98196.0 ns +9% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 74031.0 ns **-31%** | 64743.0 ns **-40%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 91632.0 ns +7% | 79968.0 ns **-7%** |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 109790.0 ns +96% | 83949.0 ns +50% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 98756.0 ns +59% | 51613.0 ns **-17%** |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 94844.0 ns +79% | 70679.0 ns +33% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 194439.0 ns +254% | 76406.0 ns +39% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 99873.0 ns **-30%** | 86743.0 ns **-39%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 32895.0 ns +22% | 36667.0 ns +36% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 112654.0 ns +85% | 108673.0 ns +78% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 37575.0 ns +63% | 26191.0 ns +14% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 221466.0 ns +343% | 382311.0 ns +665% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 30800.0 ns **-4%** | 37086.0 ns +16% |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 32896.0 ns +32% | 41137.0 ns +65% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 35759.0 ns +49% | 31079.0 ns +29% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 67886.0 ns +54% | 61530.0 ns +40% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 210502.0 ns +240% | 61810.0 ns |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 32406.0 ns +54% | 34292.0 ns +63% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 30031.0 ns +67% | 29124.0 ns +62% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 24375.0 ns | 50984.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 24444.0 ns | 23606.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 222864.0 ns | 93797.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 100292.0 ns | 189480.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | — | 1965.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | — | 1031.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | — | 15575.0 µs |

## kdtree_build_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `kdtree_build_4k:` | — | — | 2214.0 µs |

## kdtree_radius_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `kdtree_radius_4k:` | — | — | 6635.0 ns |

## delaunay_2d_400:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `delaunay_2d_400:` | — | — | 14746.0 µs |

## triangulate_600gon:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `triangulate_600gon:` | — | — | 9741.0 µs |

## svd_golub_kahan_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `svd_golub_kahan_12:` | — | — | 179143.0 ns |

## eigen_qr_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`ee8032c`) | Current (`26429cc`) |
|-----------|------|------|------|
| `eigen_qr_12:` | — | — | 133536.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
