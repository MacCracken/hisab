# Benchmarks

Latest: **2026-08-05T21:42:41Z** — commit `0adde51`

Tracking: `e1f8f4c` (baseline) → `565e278` (mid) → `0adde51` (current)

## vec3_add:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `vec3_add:` | 189000.0 ns | 88209.0 ns **-53%** | 83600.0 ns **-56%** |

## vec3_cross:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `vec3_cross:` | 384000.0 ns | 289562.0 ns **-25%** | 204286.0 ns **-47%** |

## vec3_normalize:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `vec3_normalize:` | 95000.0 ns | 211759.0 ns +123% | 47631.0 ns **-50%** |

## quat_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `quat_mul:` | 90000.0 ns | 257156.0 ns +186% | 129276.0 ns +44% |

## quat_slerp:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `quat_slerp:` | 108000.0 ns | 203588.0 ns +89% | 88558.0 ns **-18%** |

## quat_rotate_vec3:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `quat_rotate_vec3:` | 86000.0 ns | 208197.0 ns +142% | 130253.0 ns +51% |

## m4_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `m4_mul:` | 56000.0 ns | 161753.0 ns +189% | 216229.0 ns +286% |

## m4_inverse:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `m4_inverse:` | 62000.0 ns | 97638.0 ns +57% | 141848.0 ns +129% |

## m4_transform_point:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `m4_transform_point:` | 53000.0 ns | 407733.0 ns +669% | 436229.0 ns +723% |

## t3d_compose:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `t3d_compose:` | 55000.0 ns | 204565.0 ns +272% | 81854.0 ns +49% |

## ray_sphere:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `ray_sphere:` | 143000.0 ns | 94915.0 ns **-34%** | 125924.0 ns **-12%** |

## ray_aabb:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `ray_aabb:` | 27000.0 ns | 44349.0 ns +64% | 63975.0 ns +137% |

## ray_triangle:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `ray_triangle:` | 61000.0 ns | 129835.0 ns +113% | 217695.0 ns +257% |

## srgb_to_linear:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `srgb_to_linear:` | 23000.0 ns | 32406.0 ns +41% | 34711.0 ns +51% |

## tonemap_reinhard:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `tonemap_reinhard:` | 50000.0 ns | 87511.0 ns +75% | 69073.0 ns +38% |

## calc_derivative:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `calc_derivative:` | 32000.0 ns | 49937.0 ns +56% | 53917.0 ns +68% |

## calc_integral_simpson:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `calc_integral_simpson:` | 25000.0 ns | 34711.0 ns +39% | 25003.0 ns |

## num_gcd:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `num_gcd:` | 24000.0 ns | 32895.0 ns +37% | 38273.0 ns +59% |

## num_is_prime:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `num_is_prime:` | 44000.0 ns | 64813.0 ns +47% | 383428.0 ns +771% |

## cx_mul:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `cx_mul:` | 62000.0 ns | 209874.0 ns +239% | 108673.0 ns +75% |

## ease_in_out:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `ease_in_out:` | 21000.0 ns | 30032.0 ns +43% | 429384.0 ns +1945% |

## perlin_2d:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `perlin_2d:` | 18000.0 ns | 26680.0 ns +48% | 31917.0 ns +77% |

## vec3_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `vec3_dot_x64:` | — | 47632.0 ns | 26750.0 ns |

## vec4_dot_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `vec4_dot_x64:` | — | 28635.0 ns | 24235.0 ns |

## m4_mul_x16:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `m4_mul_x16:` | — | 208546.0 ns | 172369.0 ns |

## m4_transform_x64:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `m4_transform_x64:` | — | 146178.0 ns | 105880.0 ns |

## convex_hull_2d_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `convex_hull_2d_2k:` | — | 2040.0 µs | 2089.0 µs |

## halfedge_2k_tris:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `halfedge_2k_tris:` | — | 1050.0 µs | 1157.0 µs |

## bvh_degenerate_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `bvh_degenerate_4k:` | — | 16266.0 µs | 3840.0 µs |

## bvh_query_ray_200x4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k:` | — | — | 2194.0 µs |

## kdtree_build_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `kdtree_build_4k:` | — | — | 2732.0 µs |

## kdtree_radius_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `kdtree_radius_4k:` | — | — | 4400.0 ns |

## delaunay_2d_400:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `delaunay_2d_400:` | — | — | 1743.0 µs |

## delaunay_2d_circle_150:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150:` | — | — | 464235.0 ns |

## triangulate_600gon:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `triangulate_600gon:` | — | — | 1692.0 µs |

## svd_golub_kahan_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `svd_golub_kahan_12:` | — | — | 184241.0 ns |

## eigen_qr_12:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `eigen_qr_12:` | — | — | 144083.0 ns |

## gjk_epa_3d_cyl_box:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box:` | — | — | 220489.0 ns |

## mpr_penetration_boxes:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `mpr_penetration_boxes:` | — | — | 37924.0 ns |

## gjk_epa_boxes:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `gjk_epa_boxes:` | — | — | 99384.0 ns |

## gjk_epa_spheres:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `gjk_epa_spheres:` | — | — | 675086.0 ns |

## gjk_epa_sphere_box:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box:` | — | — | 162241.0 ns |

## gjk_intersect_box_miss:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss:` | — | — | 782.0 ns |

## gjk_intersect_sph_miss:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss:` | — | — | 1161.0 ns |

## gjk_intersect_box_hit:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit:` | — | — | 2009.0 ns |

## gjk_intersect_tangent:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `gjk_intersect_tangent:` | — | — | 2286.0 ns |

## bvh_scatter_4k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `bvh_scatter_4k:` | — | — | 6454.0 µs |

## spatial_hash_query_2k:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `spatial_hash_query_2k:` | — | — | 920718.0 ns |

## num_dct_1024:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `num_dct_1024:` | — | — | 304299.0 ns |

## num_dst_1024:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `num_dst_1024:` | — | — | 7071.0 µs |

## num_dct_1023:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `num_dct_1023:` | — | — | 1695.0 µs |

## num_dst_1023:

| Benchmark | Baseline (`e1f8f4c`) | Mid (`565e278`) | Current (`0adde51`) |
|-----------|------|------|------|
| `num_dst_1023:` | — | — | 481137.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
