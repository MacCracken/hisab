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

Latest: **2026-11-15T05:32:53Z** — commit `b175825`, 3.5.0 uncommitted

Tracking: `a09d228` (baseline) → `5bfb8f4` (mid) → `b175825`, 3.5.0 uncommitted (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 20.00 ns **-13%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 27.00 ns **-57%** | 32.00 ns **-49%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 34.00 ns **-15%** | 42.00 ns |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 399.0 ns **-33%** | 508.0 ns **-14%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 296.0 ns | 482.0 ns +48% |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1919.0 ns **-12%** | 2406.0 ns +11% |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2725.0 ns **-16%** | 3037.0 ns |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 40.00 ns **-37%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 239.0 ns | 181.0 ns **-30%** |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 56.00 ns **-11%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 119.0 ns | 128.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 246.0 ns | 197.0 ns **-24%** |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 93.00 ns **-24%** | 110.0 ns |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 147.0 ns **-32%** | 252.0 ns +16% |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 192.0 ns **-29%** | 224.0 ns **-17%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 100.0 ns **-42%** | 89.00 ns **-49%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 471.0 ns **-39%** | 477.0 ns **-38%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 180.0 ns **-38%** | 206.0 ns **-29%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 539.0 ns **-34%** | 573.0 ns **-30%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 690.0 ns **-30%** | 933.0 ns |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 43644.0 ns **-25%** | 38198.0 ns **-34%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4498.0 ns **-23%** | 5616.0 ns |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 282.0 ns **-36%** | 251.0 ns **-43%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 63.00 ns **-45%** | 71.00 ns **-38%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 509.0 ns **-31%** | 654.0 ns **-12%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 376.0 ns **-30%** | 410.0 ns **-23%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 58.00 ns **-36%** | 63.00 ns **-30%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 42.00 ns **-55%** | 54.00 ns **-42%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 141.0 ns **-43%** | 92.00 ns **-63%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 96.00 ns +17% | 194.0 ns +137% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 24.00 ns **-20%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 91.00 ns | 96.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4830.0 ns | 5184.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 25.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19712.0 ns | 1838.0 ns **-91%** |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 27.00 ns **-23%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 6.00 ns **-14%** | 8.00 ns +14% |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 75.00 ns | 83.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1646.0 µs **-14%** | 1746.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 719974.0 ns **-13%** | 782670.0 ns |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2366.0 µs **-28%** | 2289.0 µs **-31%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 1084.0 µs **-39%** | 1059.0 µs **-40%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2393.0 µs | 2500.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 533836.0 ns | 568358.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `einsum_matmul_2x2` † | 1102.0 ns | 965.0 ns | 922.0 ns **-16%** |

† `einsum_matmul_2x2` moves 1,004-1,165 ns over code layouts of one source (review INS6 F2): a step under 16% is not marked here, and is not evidence on its own.

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 424.0 ns | 399.0 ns **-14%** |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 669.0 ns **-35%** | 402.0 ns **-61%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1384.0 µs **-13%** | 1620.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 361407.0 ns | 623114.0 ns +62% |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1544.0 µs | 1658.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5238.0 µs | 2767.0 µs **-52%** |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1180.0 ns **-19%** | 1232.0 ns **-16%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 118022.0 ns **-26%** | 127464.0 ns **-20%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 87540.0 ns **-27%** | 95013.0 ns **-20%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 81676.0 ns **-25%** | 107579.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 12734.0 ns **-27%** | 13978.0 ns **-20%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13088.0 ns **-25%** | 14489.0 ns **-17%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 408757.0 ns **-24%** | 434490.0 ns **-19%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 79614.0 ns **-29%** | 98093.0 ns **-12%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 457.0 ns **-31%** | 772.0 ns +17% |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 661.0 ns **-27%** | 1068.0 ns +18% |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 998.0 ns **-38%** | 2006.0 ns +25% |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1148.0 ns **-38%** | 2633.0 ns +43% |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 959.0 ns **-30%** | 982.0 ns **-28%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3401.0 ns **-31%** | 2381.0 ns **-51%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3520.0 ns **-30%** | 2541.0 ns **-49%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4681.0 µs **-19%** | 5185.0 µs **-11%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 407888.0 ns | 421961.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 275070.0 ns | 283162.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6516.0 µs | 7025.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1568.0 µs | 1699.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 437633.0 ns | 467417.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | 1105.0 µs | 1467.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | 178560.0 ns | 249305.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | 202546.0 ns | 213824.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cga_point_product` | — | 1219.0 ns | 1031.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | 1287.0 ns | 361.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | 27390.0 ns | 612.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 26.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 23.00 ns |

## m3_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_mul` | — | — | 94.00 ns |

## m3_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_inverse` | — | — | 90.00 ns |

## m3_mul_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_mul_vec3` | — | — | 42.00 ns |

## ivl_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ivl_mul` | — | — | 210.0 ns |

## ivl_sin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ivl_sin` | — | — | 151.0 ns |

## ivl_width

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ivl_width` | — | — | 15.00 ns |

## calc_adaptive_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_adaptive_simpson` | — | — | 8035.0 ns |

## calc_bspline_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_bspline_cubic_8` | — | — | 409.0 ns |

## calc_nurbs_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_cubic_8` | — | — | 590.0 ns |

## calc_hessian_4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_hessian_4` | — | — | 2167.0 ns |

## perlin_3d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `perlin_3d` | — | — | 201.0 ns |

## christoffel_symbols_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `christoffel_symbols_4d` | — | — | 6553.0 ns |

## riemann_tensor_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `riemann_tensor_4d` | — | — | 30900.0 ns |

## geodesic_rk4_4d_100

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `geodesic_rk4_4d_100` | — | — | 231605.0 ns |

## su2_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `su2_exp` | — | — | 107.0 ns |

## su2_log

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `su2_log` | — | — | 132.0 ns |

## lorentz_boost

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `lorentz_boost` | — | — | 311.0 ns |

## so3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `so3_exp` | — | — | 233.0 ns |

## so3_log

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `so3_log` | — | — | 164.0 ns |

## se3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `se3_exp` | — | — | 699.0 ns |

## se3_log

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `se3_log` | — | — | 675.0 ns |

## simplex_2d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `simplex_2d` | — | — | 89.00 ns |

## simplex_3d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `simplex_3d` | — | — | 160.0 ns |

## ode_dopri45_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_step` | — | — | 248.0 ns |

## ode_dopri45_trajectory

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_trajectory` | — | — | 36341.0 ns |

## ode_bdf2_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ode_bdf2_step` | — | — | 5228.0 ns |

## ode_yoshida4_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ode_yoshida4_step` | — | — | 65.00 ns |

## opt_lbfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_rosenbrock` | — | — | 29599.0 ns |

## opt_bfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_rosenbrock` | — | — | 16403.0 ns |

## opt_lm_expfit_32

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `opt_lm_expfit_32` | — | — | 12663.0 ns |

## expr_eval

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `expr_eval` | — | — | 539.0 ns |

## expr_diff_simplify

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `expr_diff_simplify` | — | — | 1248.0 ns |

## sym_integrate

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sym_integrate` | — | — | 393.0 ns |

## sym_rewrite_default

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_default` | — | — | 2924.0 ns |

## csr_spmv_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `csr_spmv_256` | — | — | 3888.0 ns |

## solve_gmres_64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64` | — | — | 81074.0 ns |

## solve_bicgstab_64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `solve_bicgstab_64` | — | — | 33990.0 ns |

## eigen_symmetric_12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_symmetric_12` | — | — | 140567.0 ns |

## f64_fmod

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `f64_fmod` | — | — | 21.00 ns |

## f64_fmod_gap2097

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `f64_fmod_gap2097` | — | — | 4212.0 ns |

## num_is_prime_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_is_prime_2e63` | — | — | 420353.0 ns |

## num_pollard_rho_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_pollard_rho_2e63` | — | — | 78282.0 µs |

## num_crt_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_crt_2e63` | — | — | 1008.0 ns |

## num_modinv_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_modinv_2e63` | — | — | 26.00 ns |

## num_divisor_sigma_720720

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma_720720` | — | — | 216.0 ns |

## num_divisor_sigma0_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma0_2e63` | — | — | 332377.0 ns |

## dual_pow_x3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `dual_pow_x3` | — | — | 240.0 ns |

## hvec3_angle

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `hvec3_angle` | — | — | 548.0 ns |

## perlin_2d_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `perlin_2d_2e63` | — | — | 83.00 ns |

## delaunay_2d_mixed_100

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_mixed_100` | — | — | 953775.0 ns |

## bvh_query_ray_oblique_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_oblique_200x4k` | — | — | 781793.0 ns |

## cga_blade_inverse_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_sphere` | — | — | 990.0 ns |

## cga_blade_inverse_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_tiny` | — | — | 995.0 ns |

## eigen_power_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_3x3` | — | — | 5498.0 ns |

## eigen_power_12x12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_12x12` | — | — | 14747.0 ns |

## eigen_power_2x2_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_wide` | — | — | 777.0 ns |

## mpr_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_near_tangent` | — | — | 11027.0 ns |

## gjk_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_near_tangent` | — | — | 10982.0 ns |

## gjk_intersect_box_miss_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss_2e600` | — | — | 1005.0 ns |

## gjk_intersect_box_hit_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit_2e600` | — | — | 4237.0 ns |

## gjk_epa_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes_separated` | — | — | 824.0 ns |

## gjk_epa_spheres_separated

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres_separated` | — | — | 1119.0 ns |

## mpr_penetration_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes_separated` | — | — | 1511.0 ns |

## time_of_impact_sph_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_sph_hit` | — | — | 109299.0 ns |

## time_of_impact_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_miss` | — | — | 2474.0 ns |

## calc_integral_gauss5_x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_integral_gauss5_x2` | — | — | 45.00 ns |

## calc_integral_trapez_x2_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_integral_trapez_x2_16` | — | — | 139.0 ns |

## num_bisection_x2m2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_bisection_x2m2` | — | — | 505.0 ns |

## ray_sphere_near_surface

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_near_surface` | — | — | 170.0 ns |

## ray_aabb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_face_origin` | — | — | 93.00 ns |

## ray_obb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_obb_face_origin` | — | — | 275.0 ns |

## gjk_intersect_sep_capsule_1e14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sep_capsule_1e14` | — | — | 279730.0 ns |

## solve_gmres_64_m4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64_m4` | — | — | 48702.0 ns |

## cga_dense_product

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cga_dense_product` | — | — | 23064.0 ns |

## num_fft_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_fft_256` | — | — | 43042.0 ns |

## num_fft_4096

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_fft_4096` | — | — | 1005.0 µs |

## kdtree_nearest_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `kdtree_nearest_4k` | — | — | 2268.0 ns |

## sh_evaluate_l2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sh_evaluate_l2` | — | — | 45.00 ns |

## lorentz_is_valid

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `lorentz_is_valid` | — | — | 616.0 ns |

## levi_civita_4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `levi_civita_4` | — | — | 29.00 ns |

## dual_ln

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `dual_ln` | — | — | 54.00 ns |

## dual_sqrt

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `dual_sqrt` | — | — | 26.00 ns |

## dual_abs

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `dual_abs` | — | — | 21.00 ns |

## num_bisection_stall

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_bisection_stall` | — | — | 675.0 ns |

## expr_eval_unbound

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `expr_eval_unbound` | — | — | 584.0 ns |

## num_bisection_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_bisection_wide` | — | — | 11846.0 ns |

## eigen_power_tri5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri5` | — | — | 1637.0 ns |

## eigen_power_tiny2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tiny2` | — | — | 1882.0 ns |

## eigen_power_wide3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_wide3` | — | — | 217297.0 ns |

## lyapunov_4x4_k50

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `lyapunov_4x4_k50` | — | — | 81069.0 ns |

## svd_compute_checked_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `svd_compute_checked_3x3` | — | — | 3038.0 ns |

## calc_integral_simpson_x2_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson_x2_16` | — | — | 334.0 ns |

## ode_backward_euler_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ode_backward_euler_step` | — | — | 1392.0 ns |

## fbm_2d_4oct

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `fbm_2d_4oct` | — | — | 361.0 ns |

## opt_cg_quad3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `opt_cg_quad3` | — | — | 1472.0 ns |

## opt_lbfgs_quad3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_quad3` | — | — | 6903.0 ns |

## gjk_epa_face_1e9

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_face_1e9` | — | — | 31186.0 ns |

## gjk_intersect_sep_edge_1e12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sep_edge_1e12` | — | — | 148368.0 ns |

## gjk_intersect_cap_1e3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_cap_1e3` | — | — | 41153.0 ns |

## time_of_impact_box_edge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_edge` | — | — | 195174.0 ns |

## gjk_epa_small_edge_1e14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_small_edge_1e14` | — | — | 148262.0 ns |

## gjk_intersect_small_edge_1e11

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_small_edge_1e11` | — | — | 41856.0 ns |

## gjk_intersect_box_ee_1e3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_ee_1e3` | — | — | 7769.0 ns |

## mpr_intersect_band_1e6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_band_1e6` | — | — | 9872.0 ns |

## ray_capsule_far_axis

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_axis` | — | — | 2330.0 ns |

## ray_capsule_near_barrel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_near_barrel` | — | — | 1707.0 ns |

## ray_capsule_thin_2e168

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_2e168` | — | — | 2193.0 ns |

## ray_capsule_half_infinite

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_half_infinite` | — | — | 304.0 ns |

## barycentric_colinear

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `barycentric_colinear` | — | — | 1897.0 ns |

## barycentric_thin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `barycentric_thin` | — | — | 4005.0 ns |

## m3_normal_matrix_cof

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof` | — | — | 110.0 ns |

## m3_inverse_near_singular

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_inverse_near_singular` | — | — | 248.0 ns |

## m4_inverse_singular_rank3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_singular_rank3` | — | — | 2289.0 ns |

## m4_inverse_near_singular

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_near_singular` | — | — | 591.0 ns |

## m4_transform_point_w0

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point_w0` | — | — | 122.0 ns |

## quat_rotate_vec3_q1000

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3_q1000` | — | — | 124.0 ns |

## quat_inverse_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_inverse_checked` | — | — | 37.00 ns |

## quat_normalize_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_normalize_checked` | — | — | 36.00 ns |

## inverse_lerp_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `inverse_lerp_checked` | — | — | 11.00 ns |

## world_to_screen_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `world_to_screen_checked` | — | — | 120.0 ns |

## m4_inverse_view_projection

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_view_projection` | — | — | 577.0 ns |

## m4_inverse_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_exact` | — | — | 11833.0 ns |

## m4_transform_point_exact_w

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point_exact_w` | — | — | 1137.0 ns |

## ray_capsule_thin_far_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_far_miss` | — | — | 324.0 ns |

## ray_capsule_thin_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_2e600` | — | — | 14756.0 ns |

## ray_sphere_c_one_bit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_c_one_bit` | — | — | 174.0 ns |

## ray_capsule_seam_graze

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_seam_graze` | — | — | 28846.0 ns |

## jet_capsule_half_infinite

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jet_capsule_half_infinite` | — | — | 700.0 ns |

## ray_sphere_graze_close

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_graze_close` | — | — | 522.0 ns |

## quat_slerp_near

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_slerp_near` | — | — | 208.0 ns |

## vec3_lerp_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_lerp_wide` | — | — | 58.00 ns |

## m3_normal_matrix_cof_near_rank1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_near_rank1` | — | — | 1082.0 ns |

## m3_normal_matrix_cof_rows

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_rows` | — | — | 142.0 ns |

## cubic_bezier_ease_css

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_css` | — | — | 235.0 ns |

## cubic_bezier_ease_flat

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_flat` | — | — | 153.0 ns |

## num_newton_lin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_newton_lin` | — | — | 23.00 ns |

## calc_gradient_4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_gradient_4` | — | — | 185.0 ns |

## calc_hermite_tcb_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_hermite_tcb_8` | — | — | 116.0 ns |

## opt_lm_rosenbrock_ls

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `opt_lm_rosenbrock_ls` | — | — | 17390.0 ns |

## simplex_fbm_2d_4oct

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `simplex_fbm_2d_4oct` | — | — | 405.0 ns |

## f64_approx_eq

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `f64_approx_eq` | — | — | 8.00 ns |

## num_factorize_84

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_factorize_84` | — | — | 153.0 ns |

## mpr_penetration_corner_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_corner_tiny` | — | — | 162613.0 ns |

## cga_norm_motor_2e536

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cga_norm_motor_2e536` | — | — | 805.0 ns |

## cga_norm_k32

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cga_norm_k32` | — | — | 1128.0 ns |

## sdf_capsule_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sdf_capsule_unit` | — | — | 165.0 ns |

## sdf_capsule_2e512

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sdf_capsule_2e512` | — | — | 185.0 ns |

## time_of_impact_cap_1e3_t0

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_cap_1e3_t0` | — | — | 205055.0 ns |

## ray_triangle_scaled_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_scaled_tiny` | — | — | 503.0 ns |

## ray_triangle_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_exact` | — | — | 1701.0 ns |

## ray_plane_subnormal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_plane_subnormal` | — | — | 227.0 ns |

## jet_capsule_scaled_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jet_capsule_scaled_tiny` | — | — | 1069.0 ns |

## jet_sphere_subnormal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `jet_sphere_subnormal` | — | — | 713.0 ns |

## segment_closest_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `segment_closest_tiny` | — | — | 162.0 ns |

## sphere_sphere_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sphere_sphere_tiny` | — | — | 96.00 ns |

## eigen_power_tri30_cert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri30_cert` | — | — | 385287.0 ns |

## eigen_symmetric_30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_symmetric_30` | — | — | 2536.0 µs |

## matrix_condition_number_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `matrix_condition_number_8` | — | — | 14227.0 ns |

## cmat_inverse_6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cmat_inverse_6` | — | — | 13108.0 ns |

## solve_pgs_frame3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `solve_pgs_frame3` | — | — | 1676.0 ns |

## b41_sectional_curvature_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41_sectional_curvature_4d` | — | — | 4811.0 ns |

## b41_so3_log_near_pi

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41_so3_log_near_pi` | — | — | 236.0 ns |

## b41_se3_exp_theta39

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41_se3_exp_theta39` | — | — | 387.0 ns |

## b41_minkowski_interval

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41_minkowski_interval` | — | — | 58.00 ns |

## b41_minkowski_interval_plain

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41_minkowski_interval_plain` | — | — | 16.00 ns |

## b41_spatial_hash_radius

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41_spatial_hash_radius` | — | — | 4432.0 ns |

## b41_linearize_depth

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41_linearize_depth` | — | — | 10.00 ns |

## b41_unpremultiply_alpha

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41_unpremultiply_alpha` | — | — | 27.00 ns |

## cx_div

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cx_div` | — | — | 31.00 ns |

## cx_div_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cx_div_wide` | — | — | 56.00 ns |

## dual_div

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `dual_div` | — | — | 28.00 ns |

## ad_div_grad

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ad_div_grad` | — | — | 151.0 ns |

## expr_to_str

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `expr_to_str` | — | — | 1324.0 ns |

## sym_to_latex

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sym_to_latex` | — | — | 1065.0 ns |

## expr_eval_deep1000

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `expr_eval_deep1000` | — | — | 28731.0 ns |

## m3_normal_matrix_cof_r1sr2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_r1sr2` | — | — | 434.0 ns |

## m3_normal_matrix_cof_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_exact` | — | — | 2967.0 ns |

## triangulate_square_1e9

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `triangulate_square_1e9` | — | — | 663.0 ns |

## triangulate_square_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `triangulate_square_2e600` | — | — | 4162.0 ns |

## delaunay_2d_square_edge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_square_edge` | — | — | 10132.0 ns |

## sequential_impulse_64x8_mixed

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_64x8_mixed` | — | — | 13912.0 ns |

## sequential_impulse_16x4_overflow

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4_overflow` | — | — | 17426.0 ns |

## ray_triangle_exact_edge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_exact_edge` | — | — | 1023.0 ns |

## ray_triangle_exact_dense

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_exact_dense` | — | — | 3513.0 ns |

## triangle_unit_normal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal` | — | — | 226.0 ns |

## triangle_unit_normal_flush

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal_flush` | — | — | 500.0 ns |

## b41l2_minkowski_gauss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l2_minkowski_gauss` | — | — | 16.00 ns |

## b41l2_minkowski_photon

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l2_minkowski_photon` | — | — | 132.0 ns |

## b41l2_minkowski_lightlike

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l2_minkowski_lightlike` | — | — | 79.00 ns |

## b41l2_reinhard_extended

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l2_reinhard_extended` | — | — | 46.00 ns |

## b41l2_levi_civita_3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l2_levi_civita_3` | — | — | 9.00 ns |

## b41l2_spatial_hash_insert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l2_spatial_hash_insert` | — | — | 101.0 ns |

## b41l2_quadtree_insert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l2_quadtree_insert` | — | — | 299.0 ns |

## b41l2_octree_insert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l2_octree_insert` | — | — | 393.0 ns |

## cond_singular_dup30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cond_singular_dup30` | — | — | 103284.0 ns |

## cond_singular_rank1_30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cond_singular_rank1_30` | — | — | 77256.0 ns |

## cond_singular_wide30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cond_singular_wide30` | — | — | 100808.0 ns |

## cmat_inverse_singular10

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cmat_inverse_singular10` | — | — | 394705.0 ns |

## eigen_power_blocks4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_blocks4` | — | — | 2363.0 ns |

## eigen_power_2x2_cancel90

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_cancel90` | — | — | 1892.0 ns |

## co3_cga_norm_point

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co3_cga_norm_point` | — | — | 317.0 ns |

## co3_cga_inverse_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co3_cga_inverse_sphere` | — | — | 987.0 ns |

## co3_cga_project_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co3_cga_project_sphere` | — | — | 2502.0 ns |

## co3_gjk_epa_cyl_side

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co3_gjk_epa_cyl_side` | — | — | 80329.0 ns |

## co3_gjk_epa_deep_corner

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co3_gjk_epa_deep_corner` | — | — | 536750.0 ns |

## co3_toi_corner_1e6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co3_toi_corner_1e6` | — | — | 35207.0 ns |

## co3_gjk_intersect_band_pair

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co3_gjk_intersect_band_pair` | — | — | 62897.0 ns |

## sym_rewrite_lin64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_lin64` | — | — | 3699.0 µs |

## m3_normal_matrix_cof_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_wide` | — | — | 740.0 ns |

## m3_normal_matrix_cof_wide_ovf

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_wide_ovf` | — | — | 778.0 ns |

## sequential_impulse_16x4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4` | — | — | 1802.0 ns |

## sequential_impulse_16x1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x1` | — | — | 503.0 ns |

## sequential_impulse_16x4_subnormal_cone

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4_subnormal_cone` | — | — | 14480.0 ns |

## sequential_impulse_16x4_underflow_cone

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4_underflow_cone` | — | — | 12492.0 ns |

## triangle_unit_normal_certified

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal_certified` | — | — | 227.0 ns |

## triangle_unit_normal_declined

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal_declined` | — | — | 918.0 ns |

## barycentric_colinear_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `barycentric_colinear_far` | — | — | 2643.0 ns |

## barycentric_sliver

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `barycentric_sliver` | — | — | 3451.0 ns |

## ray_plane_small_cancel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_plane_small_cancel` | — | — | 638.0 ns |

## co5_gjk_epa_cyl_cap_deep

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co5_gjk_epa_cyl_cap_deep` | — | — | 38250.0 ns |

## co5_gjk_epa_spheres_deep

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co5_gjk_epa_spheres_deep` | — | — | 496962.0 ns |

## co5_cga_norm_point_frac

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co5_cga_norm_point_frac` | — | — | 439.0 ns |

## co5_cga_norm_culprit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co5_cga_norm_culprit` | — | — | 1193.0 ns |

## co5_sdf_capsule_near40

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co5_sdf_capsule_near40` | — | — | 869.0 ns |

## co5_sdf_capsule_near56

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co5_sdf_capsule_near56` | — | — | 1220.0 ns |

## co5_sdf_capsule_thin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co5_sdf_capsule_thin` | — | — | 840.0 ns |

## co6_gjk_epa_cyl_rim

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co6_gjk_epa_cyl_rim` | — | — | 34822.0 ns |

## co6_sequential_impulse_top

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co6_sequential_impulse_top` | — | — | 411.0 ns |

## co6_cga_norm_tier

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co6_cga_norm_tier` | — | — | 440.0 ns |

## co6_sdf_capsule_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co6_sdf_capsule_exact` | — | — | 3977.0 ns |

## cx_div_lowrange

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cx_div_lowrange` | — | — | 240.0 ns |

## cx_inv_lowrange

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cx_inv_lowrange` | — | — | 159.0 ns |

## sym_rewrite_deep400

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_deep400` | — | — | 31667.0 µs |

## sym_rewrite_refuse_1m

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_refuse_1m` | — | — | 3881.0 µs |

## ad_grad_cancel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ad_grad_cancel` | — | — | 312.0 ns |

## b41l3_minkowski_level1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_level1` | — | — | 58.00 ns |

## b41l3_minkowski_mix

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_mix` | — | — | 27.00 ns |

## b41l3_minkowski_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_exact` | — | — | 425.0 ns |

## b41l3_minkowski_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_far` | — | — | 338.0 ns |

## b41l3_kdtree_nearest_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b41l3_kdtree_nearest_2e600` | — | — | 1651.0 ns |

## cubic_bezier_ease_easein_1m2^-53

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_easein_1m2^-53` | — | — | 117.0 ns |

## cubic_bezier_ease_one_near1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_one_near1` | — | — | 132.0 ns |

## cubic_bezier_ease_flat_half-18ulp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_flat_half-18ulp` | — | — | 150.0 ns |

## cubic_bezier_ease_easeout_1e-13

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_easeout_1e-13` | — | — | 143.0 ns |

## calc_nurbs_cubic_8_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_cubic_8_far` | — | — | 839.0 ns |

## calc_nurbs_d1_wide_gauge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_d1_wide_gauge` | — | — | 343.0 ns |

## ode_bdf2_continuation_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ode_bdf2_continuation_exp` | — | — | 10124.0 ns |

## ode_backward_euler_fail_mn3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ode_backward_euler_fail_mn3` | — | — | 1342.0 ns |

## opt_bfgs_extros10_f2^-35

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_extros10_f2^-35` | — | — | 106984.0 ns |

## opt_lbfgs_quad6_x2^-27.5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_quad6_x2^-27.5` | — | — | 387802.0 ns |

## barycentric_ordinary

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `barycentric_ordinary` | — | — | 236.0 ns |

## barycentric_offplane

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `barycentric_offplane` | — | — | 300.0 ns |

## barycentric_declined

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `barycentric_declined` | — | — | 4670.0 ns |

## ray_plane_small_fast

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_plane_small_fast` | — | — | 248.0 ns |

## vec3_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_mul` | — | — | 21.00 ns |

## vec4_add

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec4_add` | — | — | 21.00 ns |

## vec4_sub

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec4_sub` | — | — | 21.00 ns |

## vec4_scale

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec4_scale` | — | — | 20.00 ns |

## quat_dot

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_dot` | — | — | 7.00 ns |

## m3_normal_matrix_cof_wide_lossy

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_wide_lossy` | — | — | 1010.0 ns |

## ray_capsule_far_thin_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_thin_exact` | — | — | 7574.0 ns |

## ray_capsule_far_certified

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_certified` | — | — | 7094.0 ns |

## ray_sphere_graze_comp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_graze_comp` | — | — | 637.0 ns |

## ray_capsule_seam_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_seam_exact` | — | — | 7838.0 ns |

## ray_capsule_thin_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_tangent` | — | — | 6177.0 ns |

## cond_singular_gram30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cond_singular_gram30` | — | — | 432911.0 ns |

## cond_singular_lap10

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cond_singular_lap10` | — | — | 15745.0 ns |

## cond_singular_unlucky6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cond_singular_unlucky6` | — | — | 18269.0 ns |

## eigen_power_2x2_ordinary

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_ordinary` | — | — | 721.0 ns |

## eigen_power_2x2_fullrange

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_fullrange` | — | — | 751.0 ns |

## eigen_power_2x2_tie

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_tie` | — | — | 1941.0 ns |

## eigen_power_n1_block

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_n1_block` | — | — | 4483.0 ns |

## eigen_power_tri60_cert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri60_cert` | — | — | 1533.0 µs |

## inertia_sphere_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `inertia_sphere_unit` | — | — | 68.00 ns |

## inertia_sphere_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `inertia_sphere_wide` | — | — | 113.0 ns |

## inertia_box_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `inertia_box_unit` | — | — | 81.00 ns |

## inertia_box_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `inertia_box_wide` | — | — | 320.0 ns |

## ray_capsule_near_parallel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_near_parallel` | — | — | 979.0 ns |

## ray_capsule_dd_refused

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_dd_refused` | — | — | 6617.0 ns |

## ray_sphere_k_bound

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_k_bound` | — | — | 2975.0 ns |

## opt_bfgs_backoff_2^46

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_backoff_2^46` | — | — | 4974.0 ns |

## cubic_bezier_ease_ext_slope_overflow

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_ext_slope_overflow` | — | — | 53.00 ns |

## co7_gjk_epa_cyl_tilted

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co7_gjk_epa_cyl_tilted` | — | — | 78835.0 ns |

## co7_gjk_epa_cyl_rim_1e11

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co7_gjk_epa_cyl_rim_1e11` | — | — | 1017.0 µs |

## co7_sequential_impulse_fv

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co7_sequential_impulse_fv` | — | — | 296.0 ns |

## cond_singular_block30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cond_singular_block30` | — | — | 456794.0 ns |

## cond_singular_chain30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cond_singular_chain30` | — | — | 377402.0 ns |

## cx_div_zero_num

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cx_div_zero_num` | — | — | 47.00 ns |

## cx_div_self

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cx_div_self` | — | — | 43.00 ns |

## cx_div_real_quot

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `cx_div_real_quot` | — | — | 42.00 ns |

## dual_div_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `dual_div_far` | — | — | 88.00 ns |

## sym_rewrite_noop_dag16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_noop_dag16` | — | — | 2313.0 µs |

## sym_rewrite_noop_dag60

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_noop_dag60` | — | — | 18426.0 µs |

## co8_gjk_epa_ball_in_rim

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co8_gjk_epa_ball_in_rim` | — | — | 1045.0 µs |

## co8_gjk_epa_ball_past_rim

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `co8_gjk_epa_ball_past_rim` | — | — | 365078.0 ns |

## vec3_add_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_add_x64` | — | — | 1151.0 ns |

## vec3_add_x64_scoped

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_add_x64_scoped` | — | — | 2013.0 ns |

## vec3_add_into_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `vec3_add_into_x64` | — | — | 516.0 ns |

## quat_rotate_vec3_into_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3_into_x64` | — | — | 2825.0 ns |

## quat_rotate_vec3_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3_x64` | — | — | 3556.0 ns |

## m3_mul_vec3_into_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m3_mul_vec3_into_x64` | — | — | 1136.0 ns |

## m4_mul_into_x16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `m4_mul_into_x16` | — | — | 2900.0 ns |

## gjk_epa_spheres_scoped

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres_scoped` | — | — | 482438.0 ns |

## num_dst_14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_dst_14` | — | — | 9556.0 ns |

## num_dst_into_14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_dst_into_14` | — | — | 564.0 ns |

## num_dst_62

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_dst_62` | — | — | 158673.0 ns |

## num_dst_into_62

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_dst_into_62` | — | — | 35935.0 ns |

## num_dst_into_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_dst_into_1023` | — | — | 173729.0 ns |

## num_cg_poisson_16x16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `num_cg_poisson_16x16` | — | — | 346544.0 ns |

## spatial_hash_query_cell_into_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_query_cell_into_2k` | — | — | 296303.0 ns |

## spatial_hash_radius_scan_200

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_radius_scan_200` | — | — | 15850.0 ns |

## spatial_hash_radius_into_scan_200

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_radius_into_scan_200` | — | — | 16067.0 ns |

## b35L1_ray_plane_ordinary

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_ray_plane_ordinary` | — | — | 31.00 ns |

## b35L1_ray_plane_cancel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_ray_plane_cancel` | — | — | 484.0 ns |

## b35L1_jet_plane_cancel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_jet_plane_cancel` | — | — | 1046.0 ns |

## b35L1_triangle_normal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_triangle_normal` | — | — | 226.0 ns |

## b35L1_triangle_normal_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_triangle_normal_exact` | — | — | 490.0 ns |

## b35L1_triangle_area

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_triangle_area` | — | — | 222.0 ns |

## b35L1_unit_normal_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_unit_normal_checked` | — | — | 255.0 ns |

## b35L1_ray_aabb_far_exit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_ray_aabb_far_exit` | — | — | 250.0 ns |

## b35L1_ray_capsule_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_ray_capsule_far` | — | — | 10742.0 ns |

## b35L1_ray_sphere_bigd

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L1_ray_sphere_bigd` | — | — | 1057.0 ns |

## b35L2_kd_nearest_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_kd_nearest_2e600` | — | — | 1472.0 ns |

## b35L2_kd_nearest_unit512

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_kd_nearest_unit512` | — | — | 897.0 ns |

## b35L2_kd_radius_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_kd_radius_2e600` | — | — | 10944.0 ns |

## b35L2_kd_radius_nan_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_kd_radius_nan_4k` | — | — | 55.00 ns |

## b35L2_gjk_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_gjk_checked` | — | — | 2546.0 ns |

## b35L2_epa_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_epa_checked` | — | — | 25200.0 ns |

## b35L2_sdf_smooth_blend

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_sdf_smooth_blend` | — | — | 21.00 ns |

## b35L2_sdf_smooth_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_sdf_smooth_far` | — | — | 15.00 ns |

## b35L2_octree_512

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_octree_512` | — | — | 139233.0 ns |

## b35L2_quadtree_512

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_quadtree_512` | — | — | 117911.0 ns |

## b35L2_halfedge_grid

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_halfedge_grid` | — | — | 752453.0 ns |

## b35L2_he_boundary_64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L2_he_boundary_64` | — | — | 4217.0 ns |

## b35L3_eigen_power_chain_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L3_eigen_power_chain_wide` | — | — | 1174.0 µs |

## b35L3_eigen_power_chain_refused

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L3_eigen_power_chain_refused` | — | — | 937641.0 ns |

## b35L3_cond_lost_pivot_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L3_cond_lost_pivot_2x2` | — | — | 5213.0 ns |

## b35L3_cond_lost_pivot_7x7

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L3_cond_lost_pivot_7x7` | — | — | 213294.0 ns |

## b35L3_m3_cof_cliff_above

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L3_m3_cof_cliff_above` | — | — | 2702.0 ns |

## b35L3_m3_cof_cliff_below

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L3_m3_cof_cliff_below` | — | — | 2419.0 ns |

## b35L3_m3_cof_cliff_cancel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L3_m3_cof_cliff_cancel` | — | — | 9683.0 ns |

## b35L4_simp_diff_chain600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L4_simp_diff_chain600` | — | — | 8505.0 µs |

## b35L4_simp_diff_chain4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L4_simp_diff_chain4k` | — | — | 54829.0 µs |

## b35L4_eval_dbl40

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L4_eval_dbl40` | — | — | 36482.0 µs |

## b35L4_int_lnest5k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L4_int_lnest5k` | — | — | 34985.0 µs |

## b35L4_int_ln_free

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L4_int_ln_free` | — | — | 392.0 ns |

## b35L4_rewrite_noop_sin600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L4_rewrite_noop_sin600` | — | — | 9597.0 µs |

## b35L4_match_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L4_match_checked` | — | — | 390.0 ns |

## b35L4_ad_grad_into

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L4_ad_grad_into` | — | — | 328.0 ns |

## b35L5_be_newton_cert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L5_be_newton_cert` | — | — | 1410.0 ns |

## b35L5_be_robertson

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L5_be_robertson` | — | — | 4120.0 ns |

## b35L5_be_near_eq

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L5_be_near_eq` | — | — | 632.0 ns |

## b35L5_cg_flat_tail

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L5_cg_flat_tail` | — | — | 3201.0 ns |

## b35L5_cg_quad_1e5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L5_cg_quad_1e5` | — | — | 884.0 ns |

## b35L5_mobius_30030

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L5_mobius_30030` | — | — | 99.00 ns |

## b35L6_hvec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_hvec3_add` | — | — | 21.00 ns |

## b35L6_tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_tonemap_reinhard` | — | — | 27.00 ns |

## b35L6_linearize_depth

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_linearize_depth` | — | — | 10.00 ns |

## b35L6_ev100_avg_luminance

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_ev100_avg_luminance` | — | — | 202.0 ns |

## b35L6_t3d_compose_commuting

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_t3d_compose_commuting` | — | — | 259.0 ns |

## b35L6_t3d_compose_general

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_t3d_compose_general` | — | — | 1858.0 ns |

## b35L6_t2d_compose_commuting

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_t2d_compose_commuting` | — | — | 121.0 ns |

## b35L6_t2d_compose_general

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_t2d_compose_general` | — | — | 274.0 ns |

## b35L6_look_at_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_look_at_checked` | — | — | 938.0 ns |

## b35L6_look_at_near_parallel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_look_at_near_parallel` | — | — | 966.0 ns |

## b35L6_screen_ray

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_screen_ray` | — | — | 356.0 ns |

## b35L6_screen_ray_inf_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_screen_ray_inf_far` | — | — | 2079.0 ns |

## b35L6_world_to_screen_z01

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_world_to_screen_z01` | — | — | 125.0 ns |

## b35L6_tensor_get_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_tensor_get_checked` | — | — | 13.00 ns |

## b35L6_hvec3_angle_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_hvec3_angle_checked` | — | — | 551.0 ns |

## b35L6_axis_angle_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_axis_angle_unit` | — | — | 77.00 ns |

## b35L6_axis_angle_nonunit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_axis_angle_nonunit` | — | — | 127.0 ns |

## b35L6_slerp_identical

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_slerp_identical` | — | — | 57.00 ns |

## b35L6_screen_ray_far_oor

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L6_screen_ray_far_oor` | — | — | 7355.0 ns |

## b35L7_cx_ln_series

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L7_cx_ln_series` | — | — | 152.0 ns |

## b35L7_cx_ln_band

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L7_cx_ln_band` | — | — | 150.0 ns |

## b35L7_cx_ln_out

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L7_cx_ln_out` | — | — | 120.0 ns |

## b35L7_cx_exp_ovf

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L7_cx_exp_ovf` | — | — | 174.0 ns |

## b35L7_cx_sin_ovf

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L7_cx_sin_ovf` | — | — | 255.0 ns |

## b35L7_cx_mul_x

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L7_cx_mul_x` | — | — | 256.0 ns |

## b35L7_cx_div_pole

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L7_cx_div_pole` | — | — | 37.00 ns |

## b35L7_cx_powf_zero_neg

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L7_cx_powf_zero_neg` | — | — | 45.00 ns |

## b35L7_su2_axis_zero

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `b35L7_su2_axis_zero` | — | — | 98.00 ns |

## bw2W1_ray_sphere_r400_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W1_ray_sphere_r400_tangent` | — | — | 639.0 ns |

## bw2W1_ray_sphere_past_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W1_ray_sphere_past_tangent` | — | — | 838.0 ns |

## bw2W1_ray_sphere_past_ordinary

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W1_ray_sphere_past_ordinary` | — | — | 304.0 ns |

## bw2W1_ray_sphere_tiny_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W1_ray_sphere_tiny_tangent` | — | — | 1108.0 ns |

## bw2W1_ray_sphere_tiny_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W1_ray_sphere_tiny_miss` | — | — | 817.0 ns |

## bw2W2_cap_band

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W2_cap_band` | — | — | 874.0 ns |

## bw2W2_cap_dperp_q

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W2_cap_dperp_q` | — | — | 971.0 ns |

## bw2W2_cap_long

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W2_cap_long` | — | — | 7508.0 ns |

## bw2W2_cap_axis

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W2_cap_axis` | — | — | 1242.0 ns |

## bw2W3_rim_cap_intersect

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W3_rim_cap_intersect` | — | — | 504049.0 ns |

## bw2W3_rim_cap_epa

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W3_rim_cap_epa` | — | — | 480798.0 ns |

## bw2W3_rim_cap_apart

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W3_rim_cap_apart` | — | — | 1067.0 µs |

## bw2W3_par_concentric_epa

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W3_par_concentric_epa` | — | — | 544003.0 ns |

## bw2W3_par_deep_snap

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W3_par_deep_snap` | — | — | 196028.0 ns |

## bw2W4_cond_lost24

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W4_cond_lost24` | — | — | 5453.0 µs |

## bw2W4_cond_lost24_coupled

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W4_cond_lost24_coupled` | — | — | 5920.0 µs |

## bw2W4_cond_lost6_retry

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W4_cond_lost6_retry` | — | — | 190665.0 ns |

## bw2W4_cinv_lost4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W4_cinv_lost4` | — | — | 332497.0 ns |

## bw2W4_eigen_power_cancel30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W4_eigen_power_cancel30` | — | — | 77214.0 ns |

## bw2W4_eigen_power_nonnormal24

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W4_eigen_power_nonnormal24` | — | — | 2406.0 µs |

## bw2W4_eigen_power_dd4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W4_eigen_power_dd4` | — | — | 563950.0 ns |

## bw2W5_be_slope_refuse

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W5_be_slope_refuse` | — | — | 4137.0 ns |

## bw2W5_be_tri_cert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W5_be_tri_cert` | — | — | 8128.0 ns |

## bw2W5_be_neg_cert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W5_be_neg_cert` | — | — | 1411.0 ns |

## bw2W6_eval_memo_1m

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W6_eval_memo_1m` | — | — | 96228.0 µs |

## bw2W6_simp_memo_1m

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W6_simp_memo_1m` | — | — | 146791.0 µs |

## bw2W6_diff_memo_1m

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W6_diff_memo_1m` | — | — | 213676.0 µs |

## bw2W6_ad_grad_cut

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W6_ad_grad_cut` | — | — | 921.0 ns |

## bw2W6_ad_grad_norm_ss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W6_ad_grad_norm_ss` | — | — | 90115.0 ns |

## bw2W6_ad_grad_norm_ss1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W6_ad_grad_norm_ss1` | — | — | 181575.0 ns |

## bw2W6_ad_grad_cut_400

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W6_ad_grad_cut_400` | — | — | 236233.0 ns |

## bw2W6_dual_pow_1_5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W6_dual_pow_1_5` | — | — | 429.0 ns |

## sectional_curvature_dd_3d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sectional_curvature_dd_3d` | — | — | 6601.0 ns |

## sectional_curvature_exact_3d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sectional_curvature_exact_3d` | — | — | 13351.0 ns |

## se3_log_w_negative

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `se3_log_w_negative` | — | — | 692.0 ns |

## sectional_curvature_dd_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sectional_curvature_dd_4d` | — | — | 16618.0 ns |

## sectional_curvature_exact_dense_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `sectional_curvature_exact_dense_4d` | — | — | 81593.0 ns |

## bw2W8_vec2_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W8_vec2_dot_x64` | — | — | 490.0 ns |

## bw2W8_vec3_distance_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W8_vec3_distance_x64` | — | — | 1226.0 ns |

## bw2W8_vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W8_vec3_cross` | — | — | 31.00 ns |

## bw2W8_vec3_neg

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W8_vec3_neg` | — | — | 24.00 ns |

## bw2W8_quat_conjugate

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`b175825`, 3.5.0 uncommitted) |
|-----------|------|------|------|
| `bw2W8_quat_conjugate` | — | — | 26.00 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
