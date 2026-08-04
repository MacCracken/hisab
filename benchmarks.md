# Benchmarks

Latest: **2026-08-04T18:45:50Z** — commit `978b414`

Tracking: `e1f8f4c` (baseline) → `5331f05` (mid) → `978b414` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 81435.0 ns **-57%** | 41416.0 ns **-78%** |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 81365.0 ns **-79%** | 165105.0 ns **-57%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 108673.0 ns +14% | 44489.0 ns **-53%** |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 148064.0 ns +65% | 91073.0 ns |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 56781.0 ns **-47%** | 66279.0 ns **-39%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 88140.0 ns | 137447.0 ns +60% |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 82134.0 ns +47% | 65860.0 ns +18% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 73333.0 ns +18% | 68654.0 ns +11% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 209105.0 ns +295% | 65301.0 ns +23% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 89537.0 ns +63% | 72355.0 ns +32% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 81086.0 ns **-43%** | 131092.0 ns **-8%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 52521.0 ns +95% | 28565.0 ns +6% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 70121.0 ns +15% | 94495.0 ns +55% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 29612.0 ns +29% | 33943.0 ns +48% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 429873.0 ns +760% | 103784.0 ns +108% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 629340.0 ns +1867% | 23536.0 ns **-26%** |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 42673.0 ns +71% | 26540.0 ns +6% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 42743.0 ns +78% | 30241.0 ns +26% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 253384.0 ns +476% | 72425.0 ns +65% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 97848.0 ns +58% | 146527.0 ns +136% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 33804.0 ns +61% | 26610.0 ns +27% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 26958.0 ns +50% | 24375.0 ns +35% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 20603.0 ns | 21022.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 22420.0 ns | 35689.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 188990.0 ns | 125644.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 104343.0 ns | 97778.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | — | 2009.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | — | 1135.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | — | 16072.0 µs |

## kdtree_build_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `kdtree_build_4k:` | — | — | 2271.0 µs |

## kdtree_radius_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `kdtree_radius_4k:` | — | — | 7123.0 ns |

## delaunay_2d_400:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `delaunay_2d_400:` | — | — | 15153.0 µs |

## triangulate_600gon:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `triangulate_600gon:` | — | — | 9933.0 µs |

## svd_golub_kahan_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `svd_golub_kahan_12:` | — | — | 181378.0 ns |

## eigen_qr_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`5331f05`) | Current (`978b414`) |
|-----------|------|------|------|
| `eigen_qr_12:` | — | — | 139822.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
