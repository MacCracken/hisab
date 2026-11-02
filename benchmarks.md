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

Latest: **2026-11-02T02:39:30Z** — commit `c05bacd`, 3.4.1 uncommitted

Tracking: `a09d228` (baseline) → `5bfb8f4` (mid) → `c05bacd`, 3.4.1 uncommitted (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 16.00 ns **-30%** | 16.00 ns **-30%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 29.00 ns **-54%** | 27.00 ns **-57%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 34.00 ns **-15%** | 35.00 ns **-12%** |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 398.0 ns **-33%** | 401.0 ns **-32%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 299.0 ns | 341.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1898.0 ns **-13%** | 1954.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2775.0 ns **-15%** | 2717.0 ns **-17%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 38.00 ns **-40%** | 38.00 ns **-40%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 237.0 ns | 170.0 ns **-34%** |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 38.00 ns **-40%** | 49.00 ns **-22%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 118.0 ns **-11%** | 125.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 244.0 ns | 184.0 ns **-29%** |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 93.00 ns **-24%** | 102.0 ns **-16%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 146.0 ns **-33%** | 164.0 ns **-24%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 191.0 ns **-30%** | 211.0 ns **-22%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 99.00 ns **-43%** | 66.00 ns **-62%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 463.0 ns **-40%** | 442.0 ns **-43%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 179.0 ns **-38%** | 188.0 ns **-35%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 537.0 ns **-34%** | 552.0 ns **-33%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 687.0 ns **-31%** | 854.0 ns **-14%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 43624.0 ns **-25%** | 35343.0 ns **-39%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4470.0 ns **-24%** | 5191.0 ns **-11%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 281.0 ns **-37%** | 270.0 ns **-39%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 63.00 ns **-45%** | 68.00 ns **-41%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 509.0 ns **-31%** | 617.0 ns **-17%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 372.0 ns **-30%** | 388.0 ns **-27%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 57.00 ns **-37%** | 61.00 ns **-32%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 42.00 ns **-55%** | 51.00 ns **-45%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 142.0 ns **-42%** | 87.00 ns **-65%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 95.00 ns +16% | 189.0 ns +130% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 22.00 ns **-27%** | 22.00 ns **-27%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 91.00 ns | 93.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 4809.0 ns | 5000.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 24.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 19578.0 ns | 1763.0 ns **-91%** |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 23.00 ns **-34%** | 23.00 ns **-34%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 6.00 ns **-14%** | 8.00 ns +14% |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 75.00 ns | 81.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1634.0 µs **-14%** | 1696.0 µs **-11%** |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 712585.0 ns **-14%** | 711534.0 ns **-14%** |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2310.0 µs **-30%** | 2154.0 µs **-35%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 932282.0 ns **-47%** | 1012.0 µs **-43%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2286.0 µs | 2359.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 530767.0 ns | 542860.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 956.0 ns **-13%** | 966.0 ns **-12%** |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 422.0 ns | 388.0 ns **-16%** |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 663.0 ns **-36%** | 412.0 ns **-60%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1381.0 µs **-13%** | 1502.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 356211.0 ns | 564345.0 ns +47% |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1535.0 µs | 1554.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5219.0 µs | 2553.0 µs **-55%** |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1173.0 ns **-20%** | 1142.0 ns **-22%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 116769.0 ns **-26%** | 118599.0 ns **-25%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 86968.0 ns **-27%** | 88431.0 ns **-26%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 80414.0 ns **-26%** | 97357.0 ns **-11%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 12791.0 ns **-26%** | 12735.0 ns **-27%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13349.0 ns **-24%** | 13144.0 ns **-25%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 406268.0 ns **-24%** | 396184.0 ns **-26%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 79011.0 ns **-29%** | 88800.0 ns **-20%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 455.0 ns **-31%** | 697.0 ns |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 665.0 ns **-27%** | 961.0 ns |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1037.0 ns **-36%** | 1928.0 ns +20% |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1135.0 ns **-38%** | 2307.0 ns +25% |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 957.0 ns **-30%** | 702.0 ns **-49%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3360.0 ns **-31%** | 1879.0 ns **-62%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3479.0 ns **-30%** | 2343.0 ns **-53%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4667.0 µs **-20%** | 4827.0 µs **-17%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 377812.0 ns **-12%** | 394550.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 288765.0 ns | 273479.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6552.0 µs | 6693.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1569.0 µs | 1600.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 432875.0 ns | 438109.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | 1096.0 µs | 1379.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | 178923.0 ns | 212211.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | 192617.0 ns | 195289.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cga_point_product` | — | 1176.0 ns | 967.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | 1274.0 ns | 314.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | 28310.0 ns | 501.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 23.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 21.00 ns |

## m3_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_mul` | — | — | 89.00 ns |

## m3_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_inverse` | — | — | 85.00 ns |

## m3_mul_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_mul_vec3` | — | — | 41.00 ns |

## ivl_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ivl_mul` | — | — | 200.0 ns |

## ivl_sin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ivl_sin` | — | — | 135.0 ns |

## ivl_width

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ivl_width` | — | — | 14.00 ns |

## calc_adaptive_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_adaptive_simpson` | — | — | 7643.0 ns |

## calc_bspline_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_bspline_cubic_8` | — | — | 419.0 ns |

## calc_nurbs_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_cubic_8` | — | — | 597.0 ns |

## calc_hessian_4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_hessian_4` | — | — | 2067.0 ns |

## perlin_3d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `perlin_3d` | — | — | 188.0 ns |

## christoffel_symbols_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `christoffel_symbols_4d` | — | — | 6219.0 ns |

## riemann_tensor_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `riemann_tensor_4d` | — | — | 29325.0 ns |

## geodesic_rk4_4d_100

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `geodesic_rk4_4d_100` | — | — | 220261.0 ns |

## su2_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `su2_exp` | — | — | 100.0 ns |

## su2_log

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `su2_log` | — | — | 123.0 ns |

## lorentz_boost

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `lorentz_boost` | — | — | 293.0 ns |

## so3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `so3_exp` | — | — | 227.0 ns |

## so3_log

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `so3_log` | — | — | 149.0 ns |

## se3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `se3_exp` | — | — | 657.0 ns |

## se3_log

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `se3_log` | — | — | 659.0 ns |

## simplex_2d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `simplex_2d` | — | — | 84.00 ns |

## simplex_3d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `simplex_3d` | — | — | 153.0 ns |

## ode_dopri45_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_step` | — | — | 224.0 ns |

## ode_dopri45_trajectory

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_trajectory` | — | — | 34613.0 ns |

## ode_bdf2_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ode_bdf2_step` | — | — | 222.0 ns |

## ode_yoshida4_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ode_yoshida4_step` | — | — | 61.00 ns |

## opt_lbfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_rosenbrock` | — | — | 27495.0 ns |

## opt_bfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_rosenbrock` | — | — | 15639.0 ns |

## opt_lm_expfit_32

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `opt_lm_expfit_32` | — | — | 11935.0 ns |

## expr_eval

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `expr_eval` | — | — | 491.0 ns |

## expr_diff_simplify

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `expr_diff_simplify` | — | — | 1039.0 ns |

## sym_integrate

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sym_integrate` | — | — | 330.0 ns |

## sym_rewrite_default

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_default` | — | — | 2579.0 ns |

## csr_spmv_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `csr_spmv_256` | — | — | 3986.0 ns |

## solve_gmres_64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64` | — | — | 76722.0 ns |

## solve_bicgstab_64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `solve_bicgstab_64` | — | — | 31146.0 ns |

## eigen_symmetric_12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_symmetric_12` | — | — | 138339.0 ns |

## f64_fmod

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `f64_fmod` | — | — | 21.00 ns |

## f64_fmod_gap2097

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `f64_fmod_gap2097` | — | — | 4013.0 ns |

## num_is_prime_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_is_prime_2e63` | — | — | 397028.0 ns |

## num_pollard_rho_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_pollard_rho_2e63` | — | — | 74563.0 µs |

## num_crt_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_crt_2e63` | — | — | 964.0 ns |

## num_modinv_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_modinv_2e63` | — | — | 24.00 ns |

## num_divisor_sigma_720720

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma_720720` | — | — | 204.0 ns |

## num_divisor_sigma0_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma0_2e63` | — | — | 320540.0 ns |

## dual_pow_x3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `dual_pow_x3` | — | — | 230.0 ns |

## hvec3_angle

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `hvec3_angle` | — | — | 515.0 ns |

## perlin_2d_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `perlin_2d_2e63` | — | — | 80.00 ns |

## delaunay_2d_mixed_100

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_mixed_100` | — | — | 868638.0 ns |

## bvh_query_ray_oblique_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_oblique_200x4k` | — | — | 743790.0 ns |

## cga_blade_inverse_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_sphere` | — | — | 946.0 ns |

## cga_blade_inverse_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_tiny` | — | — | 962.0 ns |

## eigen_power_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_3x3` | — | — | 5560.0 ns |

## eigen_power_12x12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_12x12` | — | — | 13861.0 ns |

## eigen_power_2x2_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_wide` | — | — | 709.0 ns |

## mpr_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_near_tangent` | — | — | 9943.0 ns |

## gjk_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_near_tangent` | — | — | 9824.0 ns |

## gjk_intersect_box_miss_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss_2e600` | — | — | 909.0 ns |

## gjk_intersect_box_hit_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit_2e600` | — | — | 3864.0 ns |

## gjk_epa_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes_separated` | — | — | 768.0 ns |

## gjk_epa_spheres_separated

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres_separated` | — | — | 1025.0 ns |

## mpr_penetration_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes_separated` | — | — | 1384.0 ns |

## time_of_impact_sph_hit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_sph_hit` | — | — | 98904.0 ns |

## time_of_impact_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_miss` | — | — | 2108.0 ns |

## calc_integral_gauss5_x2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_integral_gauss5_x2` | — | — | 43.00 ns |

## calc_integral_trapez_x2_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_integral_trapez_x2_16` | — | — | 140.0 ns |

## num_bisection_x2m2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_bisection_x2m2` | — | — | 482.0 ns |

## ray_sphere_near_surface

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_near_surface` | — | — | 161.0 ns |

## ray_aabb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_face_origin` | — | — | 89.00 ns |

## ray_obb_face_origin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_obb_face_origin` | — | — | 296.0 ns |

## gjk_intersect_sep_capsule_1e14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sep_capsule_1e14` | — | — | 258400.0 ns |

## solve_gmres_64_m4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64_m4` | — | — | 45137.0 ns |

## cga_dense_product

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cga_dense_product` | — | — | 21614.0 ns |

## num_fft_256

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_fft_256` | — | — | 40824.0 ns |

## num_fft_4096

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_fft_4096` | — | — | 951366.0 ns |

## kdtree_nearest_4k

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `kdtree_nearest_4k` | — | — | 2146.0 ns |

## sh_evaluate_l2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sh_evaluate_l2` | — | — | 43.00 ns |

## lorentz_is_valid

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `lorentz_is_valid` | — | — | 585.0 ns |

## levi_civita_4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `levi_civita_4` | — | — | 27.00 ns |

## dual_ln

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `dual_ln` | — | — | 55.00 ns |

## dual_sqrt

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `dual_sqrt` | — | — | 22.00 ns |

## dual_abs

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `dual_abs` | — | — | 21.00 ns |

## num_bisection_stall

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_bisection_stall` | — | — | 659.0 ns |

## expr_eval_unbound

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `expr_eval_unbound` | — | — | 536.0 ns |

## num_bisection_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_bisection_wide` | — | — | 11383.0 ns |

## eigen_power_tri5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri5` | — | — | 1538.0 ns |

## eigen_power_tiny2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tiny2` | — | — | 1979.0 ns |

## eigen_power_wide3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_wide3` | — | — | 148110.0 ns |

## lyapunov_4x4_k50

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `lyapunov_4x4_k50` | — | — | 77138.0 ns |

## svd_compute_checked_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `svd_compute_checked_3x3` | — | — | 2890.0 ns |

## calc_integral_simpson_x2_16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson_x2_16` | — | — | 319.0 ns |

## ode_backward_euler_step

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ode_backward_euler_step` | — | — | 61.00 ns |

## fbm_2d_4oct

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `fbm_2d_4oct` | — | — | 349.0 ns |

## opt_cg_quad3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `opt_cg_quad3` | — | — | 1125.0 ns |

## opt_lbfgs_quad3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_quad3` | — | — | 6768.0 ns |

## gjk_epa_face_1e9

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_face_1e9` | — | — | 28595.0 ns |

## gjk_intersect_sep_edge_1e12

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sep_edge_1e12` | — | — | 136098.0 ns |

## gjk_intersect_cap_1e3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_cap_1e3` | — | — | 35375.0 ns |

## time_of_impact_box_edge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_edge` | — | — | 177435.0 ns |

## gjk_epa_small_edge_1e14

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_small_edge_1e14` | — | — | 136531.0 ns |

## gjk_intersect_small_edge_1e11

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_small_edge_1e11` | — | — | 38726.0 ns |

## gjk_intersect_box_ee_1e3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_ee_1e3` | — | — | 7327.0 ns |

## mpr_intersect_band_1e6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_band_1e6` | — | — | 8918.0 ns |

## ray_capsule_far_axis

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_axis` | — | — | 1591.0 ns |

## ray_capsule_near_barrel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_near_barrel` | — | — | 1623.0 ns |

## ray_capsule_thin_2e168

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_2e168` | — | — | 2093.0 ns |

## ray_capsule_half_infinite

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_half_infinite` | — | — | 295.0 ns |

## barycentric_colinear

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `barycentric_colinear` | — | — | 1779.0 ns |

## barycentric_thin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `barycentric_thin` | — | — | 3808.0 ns |

## m3_normal_matrix_cof

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof` | — | — | 105.0 ns |

## m3_inverse_near_singular

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_inverse_near_singular` | — | — | 232.0 ns |

## m4_inverse_singular_rank3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_singular_rank3` | — | — | 2163.0 ns |

## m4_inverse_near_singular

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_near_singular` | — | — | 551.0 ns |

## m4_transform_point_w0

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point_w0` | — | — | 111.0 ns |

## quat_rotate_vec3_q1000

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3_q1000` | — | — | 110.0 ns |

## quat_inverse_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `quat_inverse_checked` | — | — | 35.00 ns |

## quat_normalize_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `quat_normalize_checked` | — | — | 32.00 ns |

## inverse_lerp_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `inverse_lerp_checked` | — | — | 10.00 ns |

## world_to_screen_checked

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `world_to_screen_checked` | — | — | 111.0 ns |

## m4_inverse_view_projection

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_view_projection` | — | — | 558.0 ns |

## m4_inverse_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_inverse_exact` | — | — | 11205.0 ns |

## m4_transform_point_exact_w

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point_exact_w` | — | — | 1064.0 ns |

## ray_capsule_thin_far_miss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_far_miss` | — | — | 293.0 ns |

## ray_capsule_thin_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_2e600` | — | — | 13933.0 ns |

## ray_sphere_c_one_bit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_c_one_bit` | — | — | 161.0 ns |

## ray_capsule_seam_graze

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_seam_graze` | — | — | 27599.0 ns |

## jet_capsule_half_infinite

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jet_capsule_half_infinite` | — | — | 644.0 ns |

## ray_sphere_graze_close

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_graze_close` | — | — | 506.0 ns |

## quat_slerp_near

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `quat_slerp_near` | — | — | 187.0 ns |

## vec3_lerp_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec3_lerp_wide` | — | — | 53.00 ns |

## m3_normal_matrix_cof_near_rank1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_near_rank1` | — | — | 1069.0 ns |

## m3_normal_matrix_cof_rows

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_rows` | — | — | 134.0 ns |

## cubic_bezier_ease_css

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_css` | — | — | 226.0 ns |

## cubic_bezier_ease_flat

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_flat` | — | — | 144.0 ns |

## num_newton_lin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_newton_lin` | — | — | 22.00 ns |

## calc_gradient_4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_gradient_4` | — | — | 174.0 ns |

## calc_hermite_tcb_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_hermite_tcb_8` | — | — | 111.0 ns |

## opt_lm_rosenbrock_ls

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `opt_lm_rosenbrock_ls` | — | — | 16194.0 ns |

## simplex_fbm_2d_4oct

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `simplex_fbm_2d_4oct` | — | — | 394.0 ns |

## f64_approx_eq

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `f64_approx_eq` | — | — | 8.00 ns |

## num_factorize_84

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `num_factorize_84` | — | — | 141.0 ns |

## mpr_penetration_corner_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_corner_tiny` | — | — | 148020.0 ns |

## cga_norm_motor_2e536

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cga_norm_motor_2e536` | — | — | 759.0 ns |

## cga_norm_k32

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cga_norm_k32` | — | — | 1070.0 ns |

## sdf_capsule_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sdf_capsule_unit` | — | — | 154.0 ns |

## sdf_capsule_2e512

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sdf_capsule_2e512` | — | — | 172.0 ns |

## time_of_impact_cap_1e3_t0

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_cap_1e3_t0` | — | — | 186224.0 ns |

## ray_triangle_scaled_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_scaled_tiny` | — | — | 424.0 ns |

## ray_triangle_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_exact` | — | — | 1600.0 ns |

## ray_plane_subnormal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_plane_subnormal` | — | — | 211.0 ns |

## jet_capsule_scaled_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jet_capsule_scaled_tiny` | — | — | 994.0 ns |

## jet_sphere_subnormal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `jet_sphere_subnormal` | — | — | 716.0 ns |

## segment_closest_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `segment_closest_tiny` | — | — | 149.0 ns |

## sphere_sphere_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sphere_sphere_tiny` | — | — | 91.00 ns |

## eigen_power_tri30_cert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri30_cert` | — | — | 224454.0 ns |

## eigen_symmetric_30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_symmetric_30` | — | — | 2444.0 µs |

## matrix_condition_number_8

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `matrix_condition_number_8` | — | — | 13272.0 ns |

## cmat_inverse_6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cmat_inverse_6` | — | — | 12176.0 ns |

## solve_pgs_frame3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `solve_pgs_frame3` | — | — | 1596.0 ns |

## b41_sectional_curvature_4d

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41_sectional_curvature_4d` | — | — | 7634.0 ns |

## b41_so3_log_near_pi

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41_so3_log_near_pi` | — | — | 226.0 ns |

## b41_se3_exp_theta39

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41_se3_exp_theta39` | — | — | 358.0 ns |

## b41_minkowski_interval

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41_minkowski_interval` | — | — | 55.00 ns |

## b41_minkowski_interval_plain

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41_minkowski_interval_plain` | — | — | 15.00 ns |

## b41_spatial_hash_radius

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41_spatial_hash_radius` | — | — | 4221.0 ns |

## b41_linearize_depth

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41_linearize_depth` | — | — | 9.00 ns |

## b41_unpremultiply_alpha

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41_unpremultiply_alpha` | — | — | 25.00 ns |

## cx_div

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cx_div` | — | — | 32.00 ns |

## cx_div_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cx_div_wide` | — | — | 55.00 ns |

## dual_div

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `dual_div` | — | — | 26.00 ns |

## ad_div_grad

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ad_div_grad` | — | — | 136.0 ns |

## expr_to_str

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `expr_to_str` | — | — | 1259.0 ns |

## sym_to_latex

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sym_to_latex` | — | — | 983.0 ns |

## expr_eval_deep1000

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `expr_eval_deep1000` | — | — | 25638.0 ns |

## m3_normal_matrix_cof_r1sr2

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_r1sr2` | — | — | 405.0 ns |

## m3_normal_matrix_cof_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_exact` | — | — | 2882.0 ns |

## triangulate_square_1e9

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `triangulate_square_1e9` | — | — | 625.0 ns |

## triangulate_square_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `triangulate_square_2e600` | — | — | 3986.0 ns |

## delaunay_2d_square_edge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_square_edge` | — | — | 9245.0 ns |

## sequential_impulse_64x8_mixed

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_64x8_mixed` | — | — | 13216.0 ns |

## sequential_impulse_16x4_overflow

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4_overflow` | — | — | 16708.0 ns |

## ray_triangle_exact_edge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_exact_edge` | — | — | 971.0 ns |

## ray_triangle_exact_dense

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_triangle_exact_dense` | — | — | 3367.0 ns |

## triangle_unit_normal

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal` | — | — | 216.0 ns |

## triangle_unit_normal_flush

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal_flush` | — | — | 483.0 ns |

## b41l2_minkowski_gauss

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l2_minkowski_gauss` | — | — | 15.00 ns |

## b41l2_minkowski_photon

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l2_minkowski_photon` | — | — | 126.0 ns |

## b41l2_minkowski_lightlike

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l2_minkowski_lightlike` | — | — | 75.00 ns |

## b41l2_reinhard_extended

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l2_reinhard_extended` | — | — | 42.00 ns |

## b41l2_levi_civita_3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l2_levi_civita_3` | — | — | 9.00 ns |

## b41l2_spatial_hash_insert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l2_spatial_hash_insert` | — | — | 96.00 ns |

## b41l2_quadtree_insert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l2_quadtree_insert` | — | — | 293.0 ns |

## b41l2_octree_insert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l2_octree_insert` | — | — | 362.0 ns |

## cond_singular_dup30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cond_singular_dup30` | — | — | 96920.0 ns |

## cond_singular_rank1_30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cond_singular_rank1_30` | — | — | 74847.0 ns |

## cond_singular_wide30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cond_singular_wide30` | — | — | 96978.0 ns |

## cmat_inverse_singular10

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cmat_inverse_singular10` | — | — | 379965.0 ns |

## eigen_power_blocks4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_blocks4` | — | — | 2248.0 ns |

## eigen_power_2x2_cancel90

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_cancel90` | — | — | 1758.0 ns |

## co3_cga_norm_point

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co3_cga_norm_point` | — | — | 303.0 ns |

## co3_cga_inverse_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co3_cga_inverse_sphere` | — | — | 944.0 ns |

## co3_cga_project_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co3_cga_project_sphere` | — | — | 2371.0 ns |

## co3_gjk_epa_cyl_side

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co3_gjk_epa_cyl_side` | — | — | 73841.0 ns |

## co3_gjk_epa_deep_corner

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co3_gjk_epa_deep_corner` | — | — | 490914.0 ns |

## co3_toi_corner_1e6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co3_toi_corner_1e6` | — | — | 31879.0 ns |

## co3_gjk_intersect_band_pair

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co3_gjk_intersect_band_pair` | — | — | 56055.0 ns |

## sym_rewrite_lin64

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_lin64` | — | — | 3311.0 µs |

## m3_normal_matrix_cof_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_wide` | — | — | 636.0 ns |

## m3_normal_matrix_cof_wide_ovf

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_wide_ovf` | — | — | 676.0 ns |

## sequential_impulse_16x4

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4` | — | — | 1707.0 ns |

## sequential_impulse_16x1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x1` | — | — | 484.0 ns |

## sequential_impulse_16x4_subnormal_cone

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4_subnormal_cone` | — | — | 13870.0 ns |

## sequential_impulse_16x4_underflow_cone

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sequential_impulse_16x4_underflow_cone` | — | — | 11934.0 ns |

## triangle_unit_normal_certified

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal_certified` | — | — | 220.0 ns |

## triangle_unit_normal_declined

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `triangle_unit_normal_declined` | — | — | 868.0 ns |

## barycentric_colinear_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `barycentric_colinear_far` | — | — | 2517.0 ns |

## barycentric_sliver

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `barycentric_sliver` | — | — | 3286.0 ns |

## ray_plane_small_cancel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_plane_small_cancel` | — | — | 606.0 ns |

## co5_gjk_epa_cyl_cap_deep

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co5_gjk_epa_cyl_cap_deep` | — | — | 34775.0 ns |

## co5_gjk_epa_spheres_deep

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co5_gjk_epa_spheres_deep` | — | — | 456283.0 ns |

## co5_cga_norm_point_frac

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co5_cga_norm_point_frac` | — | — | 413.0 ns |

## co5_cga_norm_culprit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co5_cga_norm_culprit` | — | — | 1152.0 ns |

## co5_sdf_capsule_near40

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co5_sdf_capsule_near40` | — | — | 820.0 ns |

## co5_sdf_capsule_near56

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co5_sdf_capsule_near56` | — | — | 1156.0 ns |

## co5_sdf_capsule_thin

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co5_sdf_capsule_thin` | — | — | 796.0 ns |

## co6_gjk_epa_cyl_rim

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co6_gjk_epa_cyl_rim` | — | — | 31798.0 ns |

## co6_sequential_impulse_top

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co6_sequential_impulse_top` | — | — | 388.0 ns |

## co6_cga_norm_tier

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co6_cga_norm_tier` | — | — | 422.0 ns |

## co6_sdf_capsule_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co6_sdf_capsule_exact` | — | — | 3773.0 ns |

## cx_div_lowrange

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cx_div_lowrange` | — | — | 230.0 ns |

## cx_inv_lowrange

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cx_inv_lowrange` | — | — | 151.0 ns |

## sym_rewrite_deep400

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_deep400` | — | — | 29072.0 µs |

## sym_rewrite_refuse_1m

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_refuse_1m` | — | — | 3427.0 µs |

## ad_grad_cancel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ad_grad_cancel` | — | — | 285.0 ns |

## b41l3_minkowski_level1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_level1` | — | — | 55.00 ns |

## b41l3_minkowski_mix

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_mix` | — | — | 26.00 ns |

## b41l3_minkowski_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_exact` | — | — | 396.0 ns |

## b41l3_minkowski_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l3_minkowski_far` | — | — | 320.0 ns |

## b41l3_kdtree_nearest_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `b41l3_kdtree_nearest_2e600` | — | — | 3848.0 ns |

## cubic_bezier_ease_easein_1m2^-53

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_easein_1m2^-53` | — | — | 112.0 ns |

## cubic_bezier_ease_one_near1

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_one_near1` | — | — | 125.0 ns |

## cubic_bezier_ease_flat_half-18ulp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_flat_half-18ulp` | — | — | 142.0 ns |

## cubic_bezier_ease_easeout_1e-13

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_easeout_1e-13` | — | — | 134.0 ns |

## calc_nurbs_cubic_8_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_cubic_8_far` | — | — | 821.0 ns |

## calc_nurbs_d1_wide_gauge

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_d1_wide_gauge` | — | — | 358.0 ns |

## ode_bdf2_continuation_exp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ode_bdf2_continuation_exp` | — | — | 9272.0 ns |

## ode_backward_euler_fail_mn3

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ode_backward_euler_fail_mn3` | — | — | 1271.0 ns |

## opt_bfgs_extros10_f2^-35

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_extros10_f2^-35` | — | — | 103109.0 ns |

## opt_lbfgs_quad6_x2^-27.5

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_quad6_x2^-27.5` | — | — | 377483.0 ns |

## barycentric_ordinary

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `barycentric_ordinary` | — | — | 225.0 ns |

## barycentric_offplane

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `barycentric_offplane` | — | — | 281.0 ns |

## barycentric_declined

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `barycentric_declined` | — | — | 4391.0 ns |

## ray_plane_small_fast

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_plane_small_fast` | — | — | 226.0 ns |

## vec3_mul

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec3_mul` | — | — | 19.00 ns |

## vec4_add

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec4_add` | — | — | 19.00 ns |

## vec4_sub

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec4_sub` | — | — | 18.00 ns |

## vec4_scale

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `vec4_scale` | — | — | 18.00 ns |

## quat_dot

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `quat_dot` | — | — | 6.00 ns |

## m3_normal_matrix_cof_wide_lossy

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `m3_normal_matrix_cof_wide_lossy` | — | — | 890.0 ns |

## ray_capsule_far_thin_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_thin_exact` | — | — | 7198.0 ns |

## ray_capsule_far_certified

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_far_certified` | — | — | 6752.0 ns |

## ray_sphere_graze_comp

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_graze_comp` | — | — | 304.0 ns |

## ray_capsule_seam_exact

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_seam_exact` | — | — | 7597.0 ns |

## ray_capsule_thin_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_thin_tangent` | — | — | 5847.0 ns |

## cond_singular_gram30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cond_singular_gram30` | — | — | 409261.0 ns |

## cond_singular_lap10

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cond_singular_lap10` | — | — | 14552.0 ns |

## cond_singular_unlucky6

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cond_singular_unlucky6` | — | — | 17268.0 ns |

## eigen_power_2x2_ordinary

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_ordinary` | — | — | 686.0 ns |

## eigen_power_2x2_fullrange

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_fullrange` | — | — | 695.0 ns |

## eigen_power_2x2_tie

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_tie` | — | — | 1851.0 ns |

## eigen_power_n1_block

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_n1_block` | — | — | 4271.0 ns |

## eigen_power_tri60_cert

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `eigen_power_tri60_cert` | — | — | 861088.0 ns |

## inertia_sphere_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `inertia_sphere_unit` | — | — | 64.00 ns |

## inertia_sphere_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `inertia_sphere_wide` | — | — | 108.0 ns |

## inertia_box_unit

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `inertia_box_unit` | — | — | 79.00 ns |

## inertia_box_wide

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `inertia_box_wide` | — | — | 311.0 ns |

## ray_capsule_near_parallel

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_near_parallel` | — | — | 901.0 ns |

## ray_capsule_dd_refused

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_dd_refused` | — | — | 6329.0 ns |

## ray_sphere_k_bound

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `ray_sphere_k_bound` | — | — | 2824.0 ns |

## opt_bfgs_backoff_2^46

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_backoff_2^46` | — | — | 4769.0 ns |

## cubic_bezier_ease_ext_slope_overflow

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cubic_bezier_ease_ext_slope_overflow` | — | — | 50.00 ns |

## co7_gjk_epa_cyl_tilted

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co7_gjk_epa_cyl_tilted` | — | — | 71864.0 ns |

## co7_gjk_epa_cyl_rim_1e11

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co7_gjk_epa_cyl_rim_1e11` | — | — | 925342.0 ns |

## co7_sequential_impulse_fv

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co7_sequential_impulse_fv` | — | — | 279.0 ns |

## cond_singular_block30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cond_singular_block30` | — | — | 423702.0 ns |

## cond_singular_chain30

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cond_singular_chain30` | — | — | 354626.0 ns |

## cx_div_zero_num

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cx_div_zero_num` | — | — | 45.00 ns |

## cx_div_self

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cx_div_self` | — | — | 42.00 ns |

## cx_div_real_quot

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `cx_div_real_quot` | — | — | 41.00 ns |

## dual_div_far

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `dual_div_far` | — | — | 83.00 ns |

## sym_rewrite_noop_dag16

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_noop_dag16` | — | — | 2181.0 µs |

## sym_rewrite_noop_dag60

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_noop_dag60` | — | — | 17370.0 µs |

## co8_gjk_epa_ball_in_rim

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co8_gjk_epa_ball_in_rim` | — | — | 958279.0 ns |

## co8_gjk_epa_ball_past_rim

| Benchmark | Baseline (`a09d228`) | Mid (`5bfb8f4`) | Current (`c05bacd`, 3.4.1 uncommitted) |
|-----------|------|------|------|
| `co8_gjk_epa_ball_past_rim` | — | — | 330063.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
