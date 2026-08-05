# Benchmarks

Latest: **2026-08-05T19:16:28Z** — commit `00f7c49`

Tracking: `e1f8f4c` (baseline) → `151f9d9` (mid) → `00f7c49` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 89117.0 ns **-53%** | 162381.0 ns **-14%** |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 96800.0 ns **-75%** | 131372.0 ns **-66%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 109162.0 ns +15% | 136819.0 ns +44% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 84159.0 ns **-6%** | 84508.0 ns **-6%** |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 92121.0 ns **-15%** | 55733.0 ns **-48%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 74521.0 ns **-13%** | 115587.0 ns +34% |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 72006.0 ns +29% | 97359.0 ns +74% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 69841.0 ns +13% | 63905.0 ns +3% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 81295.0 ns +53% | 204425.0 ns +286% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 206102.0 ns +275% | 108393.0 ns +97% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 199956.0 ns +40% | 86254.0 ns **-40%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 32197.0 ns +19% | 29962.0 ns +11% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 117263.0 ns +92% | 216089.0 ns +254% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 47911.0 ns +108% | 192482.0 ns +737% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 83321.0 ns +67% | 67257.0 ns +35% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 37086.0 ns +16% | 35130.0 ns +10% |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 36318.0 ns +45% | 64882.0 ns +160% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 40647.0 ns +69% | 35270.0 ns +47% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 100012.0 ns +127% | 125086.0 ns +184% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 246400.0 ns +297% | 64812.0 ns +5% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 50914.0 ns +142% | 497759.0 ns +2270% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 31010.0 ns +72% | 25841.0 ns +44% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 27238.0 ns | 37225.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 27657.0 ns | 31987.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 174813.0 ns | 166083.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 93867.0 ns | 123060.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | 2141.0 µs | 2065.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | 1214.0 µs | 1209.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | — | 3779.0 µs |

## bvh_query_ray_200x4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k:` | — | — | 2269.0 µs |

## kdtree_build_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `kdtree_build_4k:` | — | — | 2755.0 µs |

## kdtree_radius_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `kdtree_radius_4k:` | — | — | 20114.0 ns |

## delaunay_2d_400:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `delaunay_2d_400:` | — | — | 1859.0 µs |

## delaunay_2d_circle_150:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150:` | — | — | 467029.0 ns |

## triangulate_600gon:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `triangulate_600gon:` | — | — | 1735.0 µs |

## svd_golub_kahan_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `svd_golub_kahan_12:` | — | — | 188991.0 ns |

## eigen_qr_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `eigen_qr_12:` | — | — | 147784.0 ns |

## gjk_epa_3d_cyl_box:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box:` | — | — | 154000.0 ns |

## mpr_penetration_boxes:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `mpr_penetration_boxes:` | — | — | 111048.0 ns |

## gjk_epa_boxes:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `gjk_epa_boxes:` | — | — | 41486.0 ns |

## gjk_epa_spheres:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `gjk_epa_spheres:` | — | — | 818261.0 ns |

## gjk_epa_sphere_box:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box:` | — | — | 208896.0 ns |

## bvh_scatter_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `bvh_scatter_4k:` | — | — | 6491.0 µs |

## spatial_hash_query_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `spatial_hash_query_2k:` | — | — | 988255.0 ns |

## num_dct_1024:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `num_dct_1024:` | — | — | 320642.0 ns |

## num_dst_1024:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`151f9d9`) | Current (`00f7c49`) |
|-----------|------|------|------|
| `num_dst_1024:` | — | — | 7143.0 µs |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
