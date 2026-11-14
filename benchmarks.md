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

Latest: **2026-11-14T18:07:25Z** — commit `d6703c5`, 3.4.2 uncommitted

Tracking: `a09d228` (baseline) → `5bfb8f4` (mid) → `d6703c5`, 3.4.2 uncommitted (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 19.00 ns **-17%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 29.00 ns **-54%** | 30.00 ns **-52%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 34.00 ns **-15%** | 38.00 ns |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 398.0 ns **-33%** | 423.0 ns **-29%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 299.0 ns | 359.0 ns +10% |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1898.0 ns **-13%** | 2047.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2775.0 ns **-15%** | 2937.0 ns |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 41.00 ns **-35%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 237.0 ns | 183.0 ns **-29%** |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 57.00 ns |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 118.0 ns **-11%** | 129.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 244.0 ns | 201.0 ns **-22%** |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 93.00 ns **-24%** | 110.0 ns |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 146.0 ns **-33%** | 184.0 ns **-15%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 191.0 ns **-30%** | 231.0 ns **-15%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 99.00 ns **-43%** | 70.00 ns **-60%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 463.0 ns **-40%** | 477.0 ns **-38%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 179.0 ns **-38%** | 209.0 ns **-28%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 537.0 ns **-34%** | 576.0 ns **-30%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 687.0 ns **-31%** | 904.0 ns |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 43624.0 ns **-25%** | 38586.0 ns **-34%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4470.0 ns **-24%** | 5465.0 ns |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 281.0 ns **-37%** | 249.0 ns **-44%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 63.00 ns **-45%** | 72.00 ns **-37%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 509.0 ns **-31%** | 659.0 ns **-11%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 372.0 ns **-30%** | 414.0 ns **-23%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 57.00 ns **-37%** | 65.00 ns **-28%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 42.00 ns **-55%** | 53.00 ns **-43%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 142.0 ns **-42%** | 91.00 ns **-63%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 95.00 ns +16% | 197.0 ns +140% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 24.00 ns **-20%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 91.00 ns | 97.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4809.0 ns | 5254.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 24.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19578.0 ns | 1835.0 ns **-91%** |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 25.00 ns **-29%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 6.00 ns **-14%** | 8.00 ns +14% |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 75.00 ns | 84.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1634.0 µs **-14%** | 1739.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 712585.0 ns **-14%** | 744719.0 ns |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2310.0 µs **-30%** | 2283.0 µs **-31%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 932282.0 ns **-47%** | 1072.0 µs **-39%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2286.0 µs | 2495.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 530767.0 ns | 568680.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 956.0 ns **-13%** | 945.0 ns **-14%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 422.0 ns | 390.0 ns **-16%** |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 663.0 ns **-36%** | 381.0 ns **-63%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1381.0 µs **-13%** | 1613.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 356211.0 ns | 633569.0 ns +65% |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1535.0 µs | 1621.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5219.0 µs | 2687.0 µs **-53%** |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1173.0 ns **-20%** | 1226.0 ns **-16%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 116769.0 ns **-26%** | 126837.0 ns **-20%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 86968.0 ns **-27%** | 96159.0 ns **-19%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 80414.0 ns **-26%** | 104773.0 ns |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 12791.0 ns **-26%** | 13816.0 ns **-20%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13349.0 ns **-24%** | 14164.0 ns **-19%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 406268.0 ns **-24%** | 417932.0 ns **-22%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 79011.0 ns **-29%** | 94058.0 ns **-16%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 455.0 ns **-31%** | 740.0 ns +12% |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 665.0 ns **-27%** | 1027.0 ns +13% |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1037.0 ns **-36%** | 1989.0 ns +24% |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1135.0 ns **-38%** | 2424.0 ns +32% |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 957.0 ns **-30%** | 757.0 ns **-45%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3360.0 ns **-31%** | 2007.0 ns **-59%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3479.0 ns **-30%** | 2419.0 ns **-52%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4667.0 µs **-20%** | 5031.0 µs **-13%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 377812.0 ns **-12%** | 426852.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 288765.0 ns | 283602.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6552.0 µs | 6944.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1569.0 µs | 1666.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 432875.0 ns | 473417.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | 1096.0 µs | 1426.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | 178923.0 ns | 223206.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | 192617.0 ns | 204125.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cga_point_product` | — | 1176.0 ns | 1006.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | 1274.0 ns | 335.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | 28310.0 ns | 541.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 26.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 22.00 ns |

## m3_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_mul` | — | — | 96.00 ns |

## m3_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_inverse` | — | — | 89.00 ns |

## m3_mul_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_mul_vec3` | — | — | 43.00 ns |

## ivl_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ivl_mul` | — | — | 206.0 ns |

## ivl_sin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ivl_sin` | — | — | 141.0 ns |

## ivl_width

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ivl_width` | — | — | 15.00 ns |

## calc_adaptive_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_adaptive_simpson` | — | — | 7939.0 ns |

## calc_bspline_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_bspline_cubic_8` | — | — | 442.0 ns |

## calc_nurbs_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_cubic_8` | — | — | 626.0 ns |

## calc_hessian_4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_hessian_4` | — | — | 2137.0 ns |

## perlin_3d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `perlin_3d` | — | — | 202.0 ns |

## christoffel_symbols_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `christoffel_symbols_4d` | — | — | 6442.0 ns |

## riemann_tensor_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `riemann_tensor_4d` | — | — | 31003.0 ns |

## geodesic_rk4_4d_100

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `geodesic_rk4_4d_100` | — | — | 231323.0 ns |

## su2_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `su2_exp` | — | — | 108.0 ns |

## su2_log

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `su2_log` | — | — | 132.0 ns |

## lorentz_boost

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `lorentz_boost` | — | — | 309.0 ns |

## so3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `so3_exp` | — | — | 243.0 ns |

## so3_log

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `so3_log` | — | — | 166.0 ns |

## se3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `se3_exp` | — | — | 724.0 ns |

## se3_log

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `se3_log` | — | — | 725.0 ns |

## simplex_2d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `simplex_2d` | — | — | 89.00 ns |

## simplex_3d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `simplex_3d` | — | — | 159.0 ns |

## ode_dopri45_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_step` | — | — | 245.0 ns |

## ode_dopri45_trajectory

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_trajectory` | — | — | 36454.0 ns |

## ode_bdf2_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ode_bdf2_step` | — | — | 233.0 ns |

## ode_yoshida4_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ode_yoshida4_step` | — | — | 65.00 ns |

## opt_lbfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_rosenbrock` | — | — | 28726.0 ns |

## opt_bfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_rosenbrock` | — | — | 16392.0 ns |

## opt_lm_expfit_32

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `opt_lm_expfit_32` | — | — | 12611.0 ns |

## expr_eval

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `expr_eval` | — | — | 518.0 ns |

## expr_diff_simplify

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `expr_diff_simplify` | — | — | 1108.0 ns |

## sym_integrate

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sym_integrate` | — | — | 353.0 ns |

## sym_rewrite_default

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_default` | — | — | 2655.0 ns |

## csr_spmv_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `csr_spmv_256` | — | — | 3843.0 ns |

## solve_gmres_64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64` | — | — | 81083.0 ns |

## solve_bicgstab_64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `solve_bicgstab_64` | — | — | 34858.0 ns |

## eigen_symmetric_12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_symmetric_12` | — | — | 141052.0 ns |

## f64_fmod

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `f64_fmod` | — | — | 22.00 ns |

## f64_fmod_gap2097

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `f64_fmod_gap2097` | — | — | 4202.0 ns |

## num_is_prime_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_is_prime_2e63` | — | — | 415444.0 ns |

## num_pollard_rho_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_pollard_rho_2e63` | — | — | 77892.0 µs |

## num_crt_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_crt_2e63` | — | — | 1017.0 ns |

## num_modinv_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_modinv_2e63` | — | — | 26.00 ns |

## num_divisor_sigma_720720

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma_720720` | — | — | 216.0 ns |

## num_divisor_sigma0_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma0_2e63` | — | — | 336238.0 ns |

## dual_pow_x3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `dual_pow_x3` | — | — | 235.0 ns |

## hvec3_angle

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `hvec3_angle` | — | — | 541.0 ns |

## perlin_2d_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `perlin_2d_2e63` | — | — | 83.00 ns |

## delaunay_2d_mixed_100

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_mixed_100` | — | — | 925256.0 ns |

## bvh_query_ray_oblique_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_oblique_200x4k` | — | — | 787553.0 ns |

## cga_blade_inverse_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_sphere` | — | — | 1022.0 ns |

## cga_blade_inverse_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_tiny` | — | — | 1053.0 ns |

## eigen_power_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_3x3` | — | — | 6127.0 ns |

## eigen_power_12x12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_12x12` | — | — | 14874.0 ns |

## eigen_power_2x2_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_wide` | — | — | 1062.0 ns |

## mpr_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_near_tangent` | — | — | 10696.0 ns |

## gjk_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_near_tangent` | — | — | 10828.0 ns |

## gjk_intersect_box_miss_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss_2e600` | — | — | 984.0 ns |

## gjk_intersect_box_hit_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit_2e600` | — | — | 4114.0 ns |

## gjk_epa_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes_separated` | — | — | 817.0 ns |

## gjk_epa_spheres_separated

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres_separated` | — | — | 1117.0 ns |

## mpr_penetration_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes_separated` | — | — | 1505.0 ns |

## time_of_impact_sph_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_sph_hit` | — | — | 106180.0 ns |

## time_of_impact_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_miss` | — | — | 2330.0 ns |

## calc_integral_gauss5_x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_integral_gauss5_x2` | — | — | 45.00 ns |

## calc_integral_trapez_x2_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_integral_trapez_x2_16` | — | — | 145.0 ns |

## num_bisection_x2m2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_bisection_x2m2` | — | — | 493.0 ns |

## ray_sphere_near_surface

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_near_surface` | — | — | 170.0 ns |

## ray_aabb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_face_origin` | — | — | 93.00 ns |

## ray_obb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_obb_face_origin` | — | — | 272.0 ns |

## gjk_intersect_sep_capsule_1e14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sep_capsule_1e14` | — | — | 277179.0 ns |

## solve_gmres_64_m4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64_m4` | — | — | 48605.0 ns |

## cga_dense_product

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cga_dense_product` | — | — | 22387.0 ns |

## num_fft_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_fft_256` | — | — | 42815.0 ns |

## num_fft_4096

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_fft_4096` | — | — | 996926.0 ns |

## kdtree_nearest_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `kdtree_nearest_4k` | — | — | 2232.0 ns |

## sh_evaluate_l2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sh_evaluate_l2` | — | — | 45.00 ns |

## lorentz_is_valid

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `lorentz_is_valid` | — | — | 614.0 ns |

## levi_civita_4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `levi_civita_4` | — | — | 33.00 ns |

## dual_ln

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `dual_ln` | — | — | 55.00 ns |

## dual_sqrt

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `dual_sqrt` | — | — | 26.00 ns |

## dual_abs

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `dual_abs` | — | — | 21.00 ns |

## num_bisection_stall

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_bisection_stall` | — | — | 676.0 ns |

## expr_eval_unbound

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `expr_eval_unbound` | — | — | 562.0 ns |

## num_bisection_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_bisection_wide` | — | — | 11948.0 ns |

## eigen_power_tri5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri5` | — | — | 1758.0 ns |

## eigen_power_tiny2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tiny2` | — | — | 1861.0 ns |

## eigen_power_wide3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_wide3` | — | — | 152908.0 ns |

## lyapunov_4x4_k50

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `lyapunov_4x4_k50` | — | — | 81833.0 ns |

## svd_compute_checked_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `svd_compute_checked_3x3` | — | — | 3044.0 ns |

## calc_integral_simpson_x2_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson_x2_16` | — | — | 334.0 ns |

## ode_backward_euler_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ode_backward_euler_step` | — | — | 66.00 ns |

## fbm_2d_4oct

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `fbm_2d_4oct` | — | — | 361.0 ns |

## opt_cg_quad3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `opt_cg_quad3` | — | — | 1185.0 ns |

## opt_lbfgs_quad3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_quad3` | — | — | 6870.0 ns |

## gjk_epa_face_1e9

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_face_1e9` | — | — | 30693.0 ns |

## gjk_intersect_sep_edge_1e12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sep_edge_1e12` | — | — | 146750.0 ns |

## gjk_intersect_cap_1e3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_cap_1e3` | — | — | 40039.0 ns |

## time_of_impact_box_edge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_edge` | — | — | 192432.0 ns |

## gjk_epa_small_edge_1e14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_small_edge_1e14` | — | — | 146496.0 ns |

## gjk_intersect_small_edge_1e11

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_small_edge_1e11` | — | — | 41560.0 ns |

## gjk_intersect_box_ee_1e3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_ee_1e3` | — | — | 7823.0 ns |

## mpr_intersect_band_1e6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_band_1e6` | — | — | 9797.0 ns |

## ray_capsule_far_axis

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_axis` | — | — | 1669.0 ns |

## ray_capsule_near_barrel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_near_barrel` | — | — | 1692.0 ns |

## ray_capsule_thin_2e168

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_2e168` | — | — | 2167.0 ns |

## ray_capsule_half_infinite

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_half_infinite` | — | — | 312.0 ns |

## barycentric_colinear

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `barycentric_colinear` | — | — | 1860.0 ns |

## barycentric_thin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `barycentric_thin` | — | — | 3976.0 ns |

## m3_normal_matrix_cof

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof` | — | — | 109.0 ns |

## m3_inverse_near_singular

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_inverse_near_singular` | — | — | 241.0 ns |

## m4_inverse_singular_rank3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_singular_rank3` | — | — | 2287.0 ns |

## m4_inverse_near_singular

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_near_singular` | — | — | 585.0 ns |

## m4_transform_point_w0

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point_w0` | — | — | 118.0 ns |

## quat_rotate_vec3_q1000

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3_q1000` | — | — | 122.0 ns |

## quat_inverse_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_inverse_checked` | — | — | 37.00 ns |

## quat_normalize_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_normalize_checked` | — | — | 34.00 ns |

## inverse_lerp_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `inverse_lerp_checked` | — | — | 11.00 ns |

## world_to_screen_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `world_to_screen_checked` | — | — | 120.0 ns |

## m4_inverse_view_projection

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_view_projection` | — | — | 592.0 ns |

## m4_inverse_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_exact` | — | — | 11775.0 ns |

## m4_transform_point_exact_w

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point_exact_w` | — | — | 1111.0 ns |

## ray_capsule_thin_far_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_far_miss` | — | — | 306.0 ns |

## ray_capsule_thin_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_2e600` | — | — | 14574.0 ns |

## ray_sphere_c_one_bit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_c_one_bit` | — | — | 168.0 ns |

## ray_capsule_seam_graze

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_seam_graze` | — | — | 28855.0 ns |

## jet_capsule_half_infinite

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jet_capsule_half_infinite` | — | — | 692.0 ns |

## ray_sphere_graze_close

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_graze_close` | — | — | 520.0 ns |

## quat_slerp_near

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_slerp_near` | — | — | 208.0 ns |

## vec3_lerp_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_lerp_wide` | — | — | 56.00 ns |

## m3_normal_matrix_cof_near_rank1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_near_rank1` | — | — | 1110.0 ns |

## m3_normal_matrix_cof_rows

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_rows` | — | — | 139.0 ns |

## cubic_bezier_ease_css

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_css` | — | — | 236.0 ns |

## cubic_bezier_ease_flat

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_flat` | — | — | 153.0 ns |

## num_newton_lin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_newton_lin` | — | — | 23.00 ns |

## calc_gradient_4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_gradient_4` | — | — | 182.0 ns |

## calc_hermite_tcb_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_hermite_tcb_8` | — | — | 117.0 ns |

## opt_lm_rosenbrock_ls

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `opt_lm_rosenbrock_ls` | — | — | 18036.0 ns |

## simplex_fbm_2d_4oct

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `simplex_fbm_2d_4oct` | — | — | 407.0 ns |

## f64_approx_eq

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `f64_approx_eq` | — | — | 8.00 ns |

## num_factorize_84

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_factorize_84` | — | — | 162.0 ns |

## mpr_penetration_corner_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_corner_tiny` | — | — | 159202.0 ns |

## cga_norm_motor_2e536

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cga_norm_motor_2e536` | — | — | 820.0 ns |

## cga_norm_k32

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cga_norm_k32` | — | — | 1130.0 ns |

## sdf_capsule_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sdf_capsule_unit` | — | — | 163.0 ns |

## sdf_capsule_2e512

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sdf_capsule_2e512` | — | — | 182.0 ns |

## time_of_impact_cap_1e3_t0

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_cap_1e3_t0` | — | — | 206110.0 ns |

## ray_triangle_scaled_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_scaled_tiny` | — | — | 446.0 ns |

## ray_triangle_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_exact` | — | — | 1726.0 ns |

## ray_plane_subnormal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_plane_subnormal` | — | — | 220.0 ns |

## jet_capsule_scaled_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jet_capsule_scaled_tiny` | — | — | 1048.0 ns |

## jet_sphere_subnormal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `jet_sphere_subnormal` | — | — | 737.0 ns |

## segment_closest_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `segment_closest_tiny` | — | — | 166.0 ns |

## sphere_sphere_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sphere_sphere_tiny` | — | — | 97.00 ns |

## eigen_power_tri30_cert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri30_cert` | — | — | 232674.0 ns |

## eigen_symmetric_30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_symmetric_30` | — | — | 2559.0 µs |

## matrix_condition_number_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `matrix_condition_number_8` | — | — | 14219.0 ns |

## cmat_inverse_6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cmat_inverse_6` | — | — | 12856.0 ns |

## solve_pgs_frame3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `solve_pgs_frame3` | — | — | 1688.0 ns |

## b41_sectional_curvature_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41_sectional_curvature_4d` | — | — | 7987.0 ns |

## b41_so3_log_near_pi

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41_so3_log_near_pi` | — | — | 240.0 ns |

## b41_se3_exp_theta39

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41_se3_exp_theta39` | — | — | 378.0 ns |

## b41_minkowski_interval

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41_minkowski_interval` | — | — | 58.00 ns |

## b41_minkowski_interval_plain

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41_minkowski_interval_plain` | — | — | 16.00 ns |

## b41_spatial_hash_radius

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41_spatial_hash_radius` | — | — | 4429.0 ns |

## b41_linearize_depth

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41_linearize_depth` | — | — | 10.00 ns |

## b41_unpremultiply_alpha

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41_unpremultiply_alpha` | — | — | 41.00 ns |

## cx_div

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cx_div` | — | — | 40.00 ns |

## cx_div_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cx_div_wide` | — | — | 58.00 ns |

## dual_div

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `dual_div` | — | — | 27.00 ns |

## ad_div_grad

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ad_div_grad` | — | — | 140.0 ns |

## expr_to_str

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `expr_to_str` | — | — | 1316.0 ns |

## sym_to_latex

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sym_to_latex` | — | — | 1051.0 ns |

## expr_eval_deep1000

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `expr_eval_deep1000` | — | — | 26762.0 ns |

## m3_normal_matrix_cof_r1sr2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_r1sr2` | — | — | 428.0 ns |

## m3_normal_matrix_cof_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_exact` | — | — | 2997.0 ns |

## triangulate_square_1e9

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `triangulate_square_1e9` | — | — | 669.0 ns |

## triangulate_square_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `triangulate_square_2e600` | — | — | 4209.0 ns |

## delaunay_2d_square_edge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_square_edge` | — | — | 10085.0 ns |

## sequential_impulse_64x8_mixed

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_64x8_mixed` | — | — | 14062.0 ns |

## sequential_impulse_16x4_overflow

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4_overflow` | — | — | 17474.0 ns |

## ray_triangle_exact_edge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_exact_edge` | — | — | 1014.0 ns |

## ray_triangle_exact_dense

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_exact_dense` | — | — | 3544.0 ns |

## triangle_unit_normal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal` | — | — | 228.0 ns |

## triangle_unit_normal_flush

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal_flush` | — | — | 515.0 ns |

## b41l2_minkowski_gauss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l2_minkowski_gauss` | — | — | 16.00 ns |

## b41l2_minkowski_photon

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l2_minkowski_photon` | — | — | 132.0 ns |

## b41l2_minkowski_lightlike

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l2_minkowski_lightlike` | — | — | 79.00 ns |

## b41l2_reinhard_extended

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l2_reinhard_extended` | — | — | 44.00 ns |

## b41l2_levi_civita_3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l2_levi_civita_3` | — | — | 9.00 ns |

## b41l2_spatial_hash_insert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l2_spatial_hash_insert` | — | — | 103.0 ns |

## b41l2_quadtree_insert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l2_quadtree_insert` | — | — | 294.0 ns |

## b41l2_octree_insert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l2_octree_insert` | — | — | 364.0 ns |

## cond_singular_dup30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cond_singular_dup30` | — | — | 101127.0 ns |

## cond_singular_rank1_30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cond_singular_rank1_30` | — | — | 78616.0 ns |

## cond_singular_wide30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cond_singular_wide30` | — | — | 101290.0 ns |

## cmat_inverse_singular10

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cmat_inverse_singular10` | — | — | 393019.0 ns |

## eigen_power_blocks4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_blocks4` | — | — | 2375.0 ns |

## eigen_power_2x2_cancel90

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_cancel90` | — | — | 1868.0 ns |

## co3_cga_norm_point

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co3_cga_norm_point` | — | — | 314.0 ns |

## co3_cga_inverse_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co3_cga_inverse_sphere` | — | — | 1007.0 ns |

## co3_cga_project_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co3_cga_project_sphere` | — | — | 2506.0 ns |

## co3_gjk_epa_cyl_side

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co3_gjk_epa_cyl_side` | — | — | 77864.0 ns |

## co3_gjk_epa_deep_corner

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co3_gjk_epa_deep_corner` | — | — | 517977.0 ns |

## co3_toi_corner_1e6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co3_toi_corner_1e6` | — | — | 34858.0 ns |

## co3_gjk_intersect_band_pair

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co3_gjk_intersect_band_pair` | — | — | 61494.0 ns |

## sym_rewrite_lin64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_lin64` | — | — | 3554.0 µs |

## m3_normal_matrix_cof_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_wide` | — | — | 667.0 ns |

## m3_normal_matrix_cof_wide_ovf

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_wide_ovf` | — | — | 711.0 ns |

## sequential_impulse_16x4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4` | — | — | 1815.0 ns |

## sequential_impulse_16x1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x1` | — | — | 513.0 ns |

## sequential_impulse_16x4_subnormal_cone

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4_subnormal_cone` | — | — | 14586.0 ns |

## sequential_impulse_16x4_underflow_cone

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4_underflow_cone` | — | — | 12415.0 ns |

## triangle_unit_normal_certified

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal_certified` | — | — | 226.0 ns |

## triangle_unit_normal_declined

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal_declined` | — | — | 954.0 ns |

## barycentric_colinear_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `barycentric_colinear_far` | — | — | 2648.0 ns |

## barycentric_sliver

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `barycentric_sliver` | — | — | 3433.0 ns |

## ray_plane_small_cancel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_plane_small_cancel` | — | — | 628.0 ns |

## co5_gjk_epa_cyl_cap_deep

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co5_gjk_epa_cyl_cap_deep` | — | — | 37054.0 ns |

## co5_gjk_epa_spheres_deep

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co5_gjk_epa_spheres_deep` | — | — | 480847.0 ns |

## co5_cga_norm_point_frac

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co5_cga_norm_point_frac` | — | — | 431.0 ns |

## co5_cga_norm_culprit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co5_cga_norm_culprit` | — | — | 1211.0 ns |

## co5_sdf_capsule_near40

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co5_sdf_capsule_near40` | — | — | 860.0 ns |

## co5_sdf_capsule_near56

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co5_sdf_capsule_near56` | — | — | 1228.0 ns |

## co5_sdf_capsule_thin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co5_sdf_capsule_thin` | — | — | 834.0 ns |

## co6_gjk_epa_cyl_rim

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co6_gjk_epa_cyl_rim` | — | — | 33986.0 ns |

## co6_sequential_impulse_top

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co6_sequential_impulse_top` | — | — | 404.0 ns |

## co6_cga_norm_tier

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co6_cga_norm_tier` | — | — | 430.0 ns |

## co6_sdf_capsule_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co6_sdf_capsule_exact` | — | — | 3980.0 ns |

## cx_div_lowrange

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cx_div_lowrange` | — | — | 239.0 ns |

## cx_inv_lowrange

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cx_inv_lowrange` | — | — | 158.0 ns |

## sym_rewrite_deep400

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_deep400` | — | — | 30800.0 µs |

## sym_rewrite_refuse_1m

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_refuse_1m` | — | — | 3566.0 µs |

## ad_grad_cancel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ad_grad_cancel` | — | — | 296.0 ns |

## b41l3_minkowski_level1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_level1` | — | — | 58.00 ns |

## b41l3_minkowski_mix

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_mix` | — | — | 27.00 ns |

## b41l3_minkowski_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_exact` | — | — | 420.0 ns |

## b41l3_minkowski_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_far` | — | — | 325.0 ns |

## b41l3_kdtree_nearest_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `b41l3_kdtree_nearest_2e600` | — | — | 3963.0 ns |

## cubic_bezier_ease_easein_1m2^-53

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_easein_1m2^-53` | — | — | 118.0 ns |

## cubic_bezier_ease_one_near1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_one_near1` | — | — | 131.0 ns |

## cubic_bezier_ease_flat_half-18ulp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_flat_half-18ulp` | — | — | 150.0 ns |

## cubic_bezier_ease_easeout_1e-13

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_easeout_1e-13` | — | — | 142.0 ns |

## calc_nurbs_cubic_8_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_cubic_8_far` | — | — | 886.0 ns |

## calc_nurbs_d1_wide_gauge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_d1_wide_gauge` | — | — | 391.0 ns |

## ode_bdf2_continuation_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ode_bdf2_continuation_exp` | — | — | 9830.0 ns |

## ode_backward_euler_fail_mn3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ode_backward_euler_fail_mn3` | — | — | 1333.0 ns |

## opt_bfgs_extros10_f2^-35

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_extros10_f2^-35` | — | — | 106513.0 ns |

## opt_lbfgs_quad6_x2^-27.5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_quad6_x2^-27.5` | — | — | 391209.0 ns |

## barycentric_ordinary

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `barycentric_ordinary` | — | — | 233.0 ns |

## barycentric_offplane

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `barycentric_offplane` | — | — | 295.0 ns |

## barycentric_declined

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `barycentric_declined` | — | — | 4581.0 ns |

## ray_plane_small_fast

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_plane_small_fast` | — | — | 236.0 ns |

## vec3_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_mul` | — | — | 20.00 ns |

## vec4_add

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec4_add` | — | — | 20.00 ns |

## vec4_sub

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec4_sub` | — | — | 20.00 ns |

## vec4_scale

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec4_scale` | — | — | 19.00 ns |

## quat_dot

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_dot` | — | — | 6.00 ns |

## m3_normal_matrix_cof_wide_lossy

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_wide_lossy` | — | — | 918.0 ns |

## ray_capsule_far_thin_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_thin_exact` | — | — | 7543.0 ns |

## ray_capsule_far_certified

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_certified` | — | — | 7064.0 ns |

## ray_sphere_graze_comp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_graze_comp` | — | — | 319.0 ns |

## ray_capsule_seam_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_seam_exact` | — | — | 7732.0 ns |

## ray_capsule_thin_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_tangent` | — | — | 6126.0 ns |

## cond_singular_gram30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cond_singular_gram30` | — | — | 427478.0 ns |

## cond_singular_lap10

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cond_singular_lap10` | — | — | 15286.0 ns |

## cond_singular_unlucky6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cond_singular_unlucky6` | — | — | 18015.0 ns |

## eigen_power_2x2_ordinary

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_ordinary` | — | — | 723.0 ns |

## eigen_power_2x2_fullrange

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_fullrange` | — | — | 753.0 ns |

## eigen_power_2x2_tie

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_tie` | — | — | 1942.0 ns |

## eigen_power_n1_block

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_n1_block` | — | — | 4440.0 ns |

## eigen_power_tri60_cert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri60_cert` | — | — | 902019.0 ns |

## inertia_sphere_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `inertia_sphere_unit` | — | — | 67.00 ns |

## inertia_sphere_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `inertia_sphere_wide` | — | — | 112.0 ns |

## inertia_box_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `inertia_box_unit` | — | — | 80.00 ns |

## inertia_box_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `inertia_box_wide` | — | — | 320.0 ns |

## ray_capsule_near_parallel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_near_parallel` | — | — | 939.0 ns |

## ray_capsule_dd_refused

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_dd_refused` | — | — | 6577.0 ns |

## ray_sphere_k_bound

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_k_bound` | — | — | 2953.0 ns |

## opt_bfgs_backoff_2^46

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_backoff_2^46` | — | — | 4969.0 ns |

## cubic_bezier_ease_ext_slope_overflow

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_ext_slope_overflow` | — | — | 52.00 ns |

## co7_gjk_epa_cyl_tilted

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co7_gjk_epa_cyl_tilted` | — | — | 76937.0 ns |

## co7_gjk_epa_cyl_rim_1e11

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co7_gjk_epa_cyl_rim_1e11` | — | — | 989047.0 ns |

## co7_sequential_impulse_fv

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co7_sequential_impulse_fv` | — | — | 293.0 ns |

## cond_singular_block30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cond_singular_block30` | — | — | 447804.0 ns |

## cond_singular_chain30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cond_singular_chain30` | — | — | 372100.0 ns |

## cx_div_zero_num

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cx_div_zero_num` | — | — | 48.00 ns |

## cx_div_self

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cx_div_self` | — | — | 45.00 ns |

## cx_div_real_quot

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `cx_div_real_quot` | — | — | 44.00 ns |

## dual_div_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `dual_div_far` | — | — | 87.00 ns |

## sym_rewrite_noop_dag16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_noop_dag16` | — | — | 2251.0 µs |

## sym_rewrite_noop_dag60

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_noop_dag60` | — | — | 17840.0 µs |

## co8_gjk_epa_ball_in_rim

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co8_gjk_epa_ball_in_rim` | — | — | 1038.0 µs |

## co8_gjk_epa_ball_past_rim

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `co8_gjk_epa_ball_past_rim` | — | — | 348088.0 ns |

## vec3_add_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_add_x64` | — | — | 1086.0 ns |

## vec3_add_x64_scoped

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_add_x64_scoped` | — | — | 1991.0 ns |

## vec3_add_into_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `vec3_add_into_x64` | — | — | 468.0 ns |

## quat_rotate_vec3_into_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3_into_x64` | — | — | 2739.0 ns |

## quat_rotate_vec3_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3_x64` | — | — | 3439.0 ns |

## m3_mul_vec3_into_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m3_mul_vec3_into_x64` | — | — | 1135.0 ns |

## m4_mul_into_x16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `m4_mul_into_x16` | — | — | 2874.0 ns |

## gjk_epa_spheres_scoped

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres_scoped` | — | — | 464966.0 ns |

## num_dst_14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_dst_14` | — | — | 9469.0 ns |

## num_dst_into_14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_dst_into_14` | — | — | 561.0 ns |

## num_dst_62

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_dst_62` | — | — | 155545.0 ns |

## num_dst_into_62

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_dst_into_62` | — | — | 35111.0 ns |

## num_dst_into_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_dst_into_1023` | — | — | 170680.0 ns |

## num_cg_poisson_16x16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `num_cg_poisson_16x16` | — | — | 340628.0 ns |

## spatial_hash_query_cell_into_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_query_cell_into_2k` | — | — | 295452.0 ns |

## spatial_hash_radius_scan_200

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_radius_scan_200` | — | — | 15645.0 ns |

## spatial_hash_radius_into_scan_200

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`d6703c5`, 3.4.2 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_radius_into_scan_200` | — | — | 15681.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
