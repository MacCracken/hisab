# Benchmarks

> **Measurement changes** — read before comparing across a date.
> * **2026-09-30** (hisab 3.2.2, cyrius 6.6.6 → 6.6.12): `lib/bench.cyr` 6.6.9 decides
>   `min`/`max` for the ROW by op count rather than per window, which ends the caveat in
>   the next entry: rows under the resolution bar now print `min = max = avg`, and the
>   per-op line reports `E of W windows eligible`. **The `avg` this table trends did not
>   move**: three binaries interleaved ×4 on a quiet box (6.6.6 compiler + 6.6.6 harness /
>   6.6.12 compiler + 6.6.6 harness / 6.6.12 + 6.6.12) put the harness half at median
>   **+0.00%**, and the whole bump at median +0.25% over 80 rows. Rows with `min > avg`
>   went 13/320 → **0/320**. Two rows moved for real, and the cause is not the
>   instrument: `srgb_to_linear` **+96%** (ganita 1.2.8's fdlibm `pow` — within 1 ulp,
>   ~2× the cost) and `quat_slerp` **−29%** (6.6.9's software `sin`/`cos`, faster than
>   x87 `fsin` at these arguments). `regime` stays `net`. ⚠ The 2026-09-30 rows were
>   taken on **hpet**: the kernel marked the TSC unstable at boot (frequency skew), so
>   `floor_ns` is ≈ 1,328 against ≈ 337 for the 2026-09-22 rows. The same-boot A/B/C
>   above is the comparison; the trend columns straddle two clocksources.
> * **2026-09-21** (hisab 3.2.1, cyrius 6.6.4 → 6.6.6): `lib/bench.cyr` 6.6.5 rewrote
>   16 of its 25 functions — `min`/`max` only from windows that clear a resolution bar
>   (100 × clock read + tick), the raw total netted at read time instead of per-window
>   clamps, the floor re-calibrated at report, ns rounded half-up instead of truncated.
>   **The `avg` this table trends did not move**: three binaries interleaved ×4 on a quiet
>   box (6.6.4 compiler + 6.6.4 harness / 6.6.6 compiler + 6.6.4 harness / 6.6.6 + 6.6.6)
>   put the instrument half at median **+0.00%** (mean +0.31%) and the compiler half at
>   median −0.62% over 80 rows, 1 row past 10% in each and that row (`cx_mul`) inside its
>   own 14.5% same-binary spread — so `regime` stays `net` on a measurement, not on the
>   floor line's presence. ⚠ `min_ns`/`max_ns` are NOT comparable across this date for
>   the seven sub-40 ns rows whose fixed 2000-op window sits under the bar (`vec3_add`,
>   `vec3_cross`, `vec3_normalize`, `vec3_lerp`, `vec2_lerp`, `tonemap_reinhard`,
>   `num_gcd`): under 6.6.5+ only a perturbed window resolves there, so the printed `min`
>   is the minimum OF THE SLOW WINDOWS and sits above `avg` (`vec3_add: 16ns avg
>   (min=39ns …)`). Filed upstream with a self-proving repro
>   (`docs/development/issues/archived/2026-09-21-cyrius-bench-min-above-mean-*.md`).
>   ⚠ Separately, the rows dated 2026-09-14 and earlier were taken on a previous BOOT
>   (kernel 7.2.3, `floor_ns` ≈ 1,340); the box rebooted 2026-09-18 (7.2.6, tsc,
>   `floor_ns` ≈ 337) and the geometry/collision family reads 15–37% slower while the
>   dense kernels read 7–11% faster on the SAME 6.6.4 binary — host state, not code.
>   `floor_ns` is the marker; compare only within one boot.
> * **2026-08-21** (hisab 2.11.2, cyrius 6.5.18 → 6.5.33): `lib/bench.cyr` now
>   MEASURES one clock read on the host and subtracts it from every sample, and
>   `bench_run` sizes its own batches instead of wrapping a clock pair around every
>   iteration. **No hisab source changed.** The floor is re-measured every run and
>   recorded per row in `floor_ns` (~1,340–1,350 ns on this host; upstream measured a
>   230x spread across its four gate hosts, and it moves between reboots, so it is
>   not a constant and is not written down as one).
>   `ease_in_out` 1,407 ns → 7 ns, `cx_mul` 1,459 → 37, `quat_mul` 1,473 → 67,
>   `perlin_2d` 1,461 → 82 — those four were 94–100% instrument. The old numbers
>   also FLATTENED them: four operations spanning **149x** in reality were reported
>   within **1.26x** of each other, so a real regression had room to hide. 44 of 72
>   rows moved more than 10%; **none of it is a speedup**. Rows carry `regime=net`
>   from this date and the trend filter refuses to mix them with `raw` ones.
>   ⚠ `min_ns`/`max_ns` also changed meaning for anything `bench_run` chose to batch:
>   they are now per-CHUNK averages, not per-iteration extremes. That is the honest
>   reading — a single sub-floor iteration has no measurable duration — but it means
>   the spread narrows for reasons unrelated to the code. `estimate_ns` (the avg) is
>   unaffected and remains the column the trend is built on.
> * **2026-08-10**: 17 sub-microsecond benchmarks moved from `bench()` to
>   `bench_batch()`. `bench_run` wraps a `clock_gettime` PAIR around every call
>   (~240 ns, documented in `lib/bench.cyr`), so those rows were **~95% clock
>   overhead**: `ray_sphere` read 1,466 ns and is 79 ns; `vec3_add` read 1,457 ns
>   and is 36 ns. It also FLATTENED them — triangle/sphere measured 1.19x when the
>   true ratio is 3.6x — so a real regression could hide inside the overhead.
>   Their drop at this date is an artefact of the fix, not a speedup.
> * **2026-08-09**: `estimate_ns` changed from each benchmark's MAX to its AVG.
>   The `stat` column records which; rows with no value there are `max`.

Latest: **2026-10-11T18:23:03Z** — commit `e360abb`

Tracking: `a09d228` (baseline) → `d740afc` (mid) → `e360abb` (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 17.00 ns **-26%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 29.00 ns **-54%** | 29.00 ns **-54%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 37.00 ns | 37.00 ns |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 395.0 ns **-33%** | 426.0 ns **-28%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 307.0 ns | 345.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1972.0 ns | 2134.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2806.0 ns **-14%** | 2878.0 ns **-12%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 39.00 ns **-38%** | 39.00 ns **-38%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 249.0 ns | 175.0 ns **-32%** |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 40.00 ns **-37%** | 42.00 ns **-33%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 122.0 ns | 137.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 252.0 ns | 261.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 96.00 ns **-21%** | 94.00 ns **-23%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 152.0 ns **-30%** | 156.0 ns **-28%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 198.0 ns **-27%** | 204.0 ns **-25%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 102.0 ns **-41%** | 101.0 ns **-42%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 480.0 ns **-38%** | 491.0 ns **-36%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 184.0 ns **-37%** | 195.0 ns **-33%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 556.0 ns **-32%** | 575.0 ns **-30%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 712.0 ns **-28%** | 747.0 ns **-25%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 45581.0 ns **-22%** | 45114.0 ns **-22%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4634.0 ns **-21%** | 4594.0 ns **-22%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 290.0 ns **-35%** | 297.0 ns **-33%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 66.00 ns **-43%** | 70.00 ns **-39%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 525.0 ns **-29%** | 560.0 ns **-24%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 394.0 ns **-26%** | 424.0 ns **-21%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 59.00 ns **-34%** | 63.00 ns **-30%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 42.00 ns **-55%** | 54.00 ns **-42%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 145.0 ns **-41%** | 152.0 ns **-38%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 99.00 ns +21% | 198.0 ns +141% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 23.00 ns **-23%** | 22.00 ns **-27%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 96.00 ns | 99.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 5085.0 ns | 5217.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 27.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 20385.0 ns | 1851.0 ns **-91%** |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 24.00 ns **-31%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 8.00 ns +14% |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 78.00 ns | 84.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1693.0 µs **-11%** | 1727.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 731895.0 ns **-12%** | 764984.0 ns |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2362.0 µs **-29%** | 2324.0 µs **-30%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 965079.0 ns **-45%** | 1144.0 µs **-35%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2378.0 µs | 2446.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 553853.0 ns | 563726.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 986.0 ns **-11%** | 997.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 443.0 ns | 441.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 668.0 ns **-35%** | 684.0 ns **-34%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1442.0 µs | 1655.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 374439.0 ns | 621466.0 ns +62% |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1596.0 µs | 1650.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5530.0 µs | 5594.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1205.0 ns **-18%** | 1221.0 ns **-17%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 119192.0 ns **-25%** | 125433.0 ns **-21%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 90068.0 ns **-24%** | 94312.0 ns **-21%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 83574.0 ns **-24%** | 85133.0 ns **-22%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13444.0 ns **-23%** | 12328.0 ns **-29%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13661.0 ns **-22%** | 12997.0 ns **-26%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 420130.0 ns **-22%** | 426093.0 ns **-21%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 82406.0 ns **-26%** | 83716.0 ns **-25%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 464.0 ns **-30%** | 532.0 ns **-19%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 680.0 ns **-25%** | 764.0 ns **-16%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1045.0 ns **-35%** | 1653.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1181.0 ns **-36%** | 1796.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 990.0 ns **-28%** | 1100.0 ns **-20%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3516.0 ns **-28%** | 3349.0 ns **-32%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3607.0 ns **-28%** | 4183.0 ns **-16%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4868.0 µs **-16%** | 5151.0 µs **-11%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 379012.0 ns **-11%** | 486749.0 ns +14% |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 283624.0 ns | 277918.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6670.0 µs | 6878.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1603.0 µs | 1653.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 452891.0 ns | 456286.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | 347749.0 ns | 1182.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | 185211.0 ns | 196184.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | — | 225712.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `cga_point_product` | — | — | 1203.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | — | 1309.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | — | 27939.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 22.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`e360abb`) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 18.00 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
