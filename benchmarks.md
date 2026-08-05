# Benchmarks

Latest: **2026-08-05T19:49:17Z** — commit `b498a5c`

Tracking: `e1f8f4c` (baseline) → `565e278` (mid) → `b498a5c` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 88209.0 ns **-53%** | 212946.0 ns +13% |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 289562.0 ns **-25%** | 206381.0 ns **-46%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 211759.0 ns +123% | 105879.0 ns +11% |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 257156.0 ns +186% | 108813.0 ns +21% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 203588.0 ns +89% | 148832.0 ns +38% |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 208197.0 ns +142% | 207638.0 ns +141% |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 161753.0 ns +189% | 133886.0 ns +139% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 97638.0 ns +57% | 72425.0 ns +17% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 407733.0 ns +669% | 107207.0 ns +102% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 204565.0 ns +272% | 95893.0 ns +74% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 94915.0 ns **-34%** | 129766.0 ns **-9%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 44349.0 ns +64% | 31219.0 ns +16% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 129835.0 ns +113% | 219441.0 ns +260% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 32406.0 ns +41% | 24793.0 ns +8% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 87511.0 ns +75% | 74870.0 ns +50% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 49937.0 ns +56% | 37645.0 ns +18% |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 34711.0 ns +39% | 155956.0 ns +524% |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 32895.0 ns +37% | 27657.0 ns +15% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 64813.0 ns +47% | 65651.0 ns +49% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 209874.0 ns +239% | 195416.0 ns +215% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 30032.0 ns +43% | 33803.0 ns +61% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 26680.0 ns +48% | 370857.0 ns +1960% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 47632.0 ns | 34781.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 28635.0 ns | 31009.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 208546.0 ns | 219791.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 146178.0 ns | 95263.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | 2040.0 µs | 2041.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | 1050.0 µs | 1305.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | 16266.0 µs | 3783.0 µs |

## bvh_query_ray_200x4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k:` | — | — | 2154.0 µs |

## kdtree_build_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `kdtree_build_4k:` | — | — | 2639.0 µs |

## kdtree_radius_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `kdtree_radius_4k:` | — | — | 4820.0 ns |

## delaunay_2d_400:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `delaunay_2d_400:` | — | — | 1707.0 µs |

## delaunay_2d_circle_150:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150:` | — | — | 452292.0 ns |

## triangulate_600gon:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `triangulate_600gon:` | — | — | 1667.0 µs |

## svd_golub_kahan_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `svd_golub_kahan_12:` | — | — | 179911.0 ns |

## eigen_qr_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `eigen_qr_12:` | — | — | 134374.0 ns |

## gjk_epa_3d_cyl_box:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box:` | — | — | 220000.0 ns |

## mpr_penetration_boxes:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `mpr_penetration_boxes:` | — | — | 34222.0 ns |

## gjk_epa_boxes:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `gjk_epa_boxes:` | — | — | 117962.0 ns |

## gjk_epa_spheres:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `gjk_epa_spheres:` | — | — | 677950.0 ns |

## gjk_epa_sphere_box:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box:` | — | — | 190667.0 ns |

## bvh_scatter_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `bvh_scatter_4k:` | — | — | 6275.0 µs |

## spatial_hash_query_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `spatial_hash_query_2k:` | — | — | 912756.0 ns |

## num_dct_1024:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `num_dct_1024:` | — | — | 287048.0 ns |

## num_dst_1024:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `num_dst_1024:` | — | — | 6839.0 µs |

## num_dct_1023:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `num_dct_1023:` | — | — | 1675.0 µs |

## num_dst_1023:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`b498a5c`) |
|-----------|------|------|------|
| `num_dst_1023:` | — | — | 478553.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
