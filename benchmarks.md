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

Latest: **2026-10-12T03:26:41Z** — commit `8265681`, 3.3.6 uncommitted

Tracking: `a09d228` (baseline) → `d740afc` (mid) → `8265681`, 3.3.6 uncommitted (current)

## vec3_add

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `vec3_add` | 23.00 ns | 17.00 ns **-26%** | 17.00 ns **-26%** |

## vec3_cross

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `vec3_cross` | 63.00 ns | 29.00 ns **-54%** | 29.00 ns **-54%** |

## vec3_normalize

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `vec3_normalize` | 40.00 ns | 37.00 ns | 37.00 ns |

## vec3_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `vec3_dot_x64` | 593.0 ns | 395.0 ns **-33%** | 423.0 ns **-29%** |

## vec4_dot_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `vec4_dot_x64` | 325.0 ns | 307.0 ns | 343.0 ns |

## m4_mul_x16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `m4_mul_x16` | 2170.0 ns | 1972.0 ns | 1982.0 ns |

## m4_transform_x64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `m4_transform_x64` | 3263.0 ns | 2806.0 ns **-14%** | 2744.0 ns **-16%** |

## quat_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `quat_mul` | 63.00 ns | 39.00 ns **-38%** | 40.00 ns **-37%** |

## quat_slerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `quat_slerp` | 257.0 ns | 249.0 ns | 176.0 ns **-32%** |

## quat_rotate_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `quat_rotate_vec3` | 63.00 ns | 40.00 ns **-37%** | 45.00 ns **-29%** |

## m4_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `m4_mul` | 132.0 ns | 122.0 ns | 127.0 ns |

## m4_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `m4_inverse` | 259.0 ns | 252.0 ns | 260.0 ns |

## m4_transform_point

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `m4_transform_point` | 122.0 ns | 96.00 ns **-21%** | 95.00 ns **-22%** |

## t3d_compose

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `t3d_compose` | 217.0 ns | 152.0 ns **-30%** | 160.0 ns **-26%** |

## jet_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `jet_sphere` | 271.0 ns | 198.0 ns **-27%** | 206.0 ns **-24%** |

## jet_plane

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `jet_plane` | 173.0 ns | 102.0 ns **-41%** | 100.0 ns **-42%** |

## jet_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `jet_triangle` | 772.0 ns | 480.0 ns **-38%** | 497.0 ns **-36%** |

## jet_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `jet_aabb` | 291.0 ns | 184.0 ns **-37%** | 199.0 ns **-32%** |

## jet_obb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `jet_obb` | 819.0 ns | 556.0 ns **-32%** | 586.0 ns **-28%** |

## jet_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `jet_capsule` | 991.0 ns | 712.0 ns **-28%** | 756.0 ns **-24%** |

## grad_fwd_16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `grad_fwd_16` | 58180.0 ns | 45581.0 ns **-22%** | 45391.0 ns **-22%** |

## grad_rev_16

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `grad_rev_16` | 5856.0 ns | 4634.0 ns **-21%** | 4616.0 ns **-21%** |

## ray_obb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ray_obb` | 443.0 ns | 290.0 ns **-35%** | 315.0 ns **-29%** |

## ray_aabb_diag

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ray_aabb_diag` | 115.0 ns | 66.00 ns **-43%** | 72.00 ns **-37%** |

## ray_capsule_diag

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ray_capsule_diag` | 740.0 ns | 525.0 ns **-29%** | 566.0 ns **-24%** |

## ray_capsule

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ray_capsule` | 535.0 ns | 394.0 ns **-26%** | 421.0 ns **-21%** |

## ray_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ray_sphere` | 90.00 ns | 59.00 ns **-34%** | 64.00 ns **-29%** |

## ray_aabb

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ray_aabb` | 93.00 ns | 42.00 ns **-55%** | 52.00 ns **-44%** |

## ray_triangle

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ray_triangle` | 246.0 ns | 145.0 ns **-41%** | 152.0 ns **-38%** |

## srgb_to_linear

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `srgb_to_linear` | 82.00 ns | 99.00 ns +21% | 202.0 ns +146% |

## tonemap_reinhard

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `tonemap_reinhard` | 30.00 ns | 23.00 ns **-23%** | 23.00 ns **-23%** |

## calc_derivative

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `calc_derivative` | 94.00 ns | 96.00 ns | 98.00 ns |

## calc_integral_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `calc_integral_simpson` | 5164.0 ns | 5085.0 ns | 5421.0 ns |

## num_gcd

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_gcd` | 25.00 ns | 27.00 ns | 25.00 ns |

## num_is_prime

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_is_prime` | 19829.0 ns | 20385.0 ns | 1850.0 ns **-91%** |

## cx_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `cx_mul` | 35.00 ns | 24.00 ns **-31%** | 24.00 ns **-31%** |

## ease_in_out

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ease_in_out` | 7.00 ns | 7.00 ns | 8.00 ns +14% |

## perlin_2d

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `perlin_2d` | 79.00 ns | 78.00 ns | 86.00 ns |

## convex_hull_2d_2k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `convex_hull_2d_2k` | 1909.0 µs | 1693.0 µs **-11%** | 1740.0 µs |

## halfedge_2k_tris

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `halfedge_2k_tris` | 827300.0 ns | 731895.0 ns **-12%** | 761918.0 ns |

## bvh_degenerate_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `bvh_degenerate_4k` | 3304.0 µs | 2362.0 µs **-29%** | 2479.0 µs **-25%** |

## bvh_query_ray_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_200x4k` | 1769.0 µs | 965079.0 ns **-45%** | 1090.0 µs **-38%** |

## kdtree_build_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_4k` | 2371.0 µs | 2378.0 µs | 2537.0 µs |

## kdtree_build_octave_512

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `kdtree_build_octave_512` | 548166.0 ns | 553853.0 ns | 566342.0 ns |

## einsum_matmul_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `einsum_matmul_2x2` | 1102.0 ns | 986.0 ns **-11%** | 995.0 ns |

## einsum_trace_2x2

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `einsum_trace_2x2` | 463.0 ns | 443.0 ns | 435.0 ns |

## kdtree_radius_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `kdtree_radius_4k` | 1035.0 ns | 668.0 ns **-35%** | 386.0 ns **-63%** |

## delaunay_2d_400

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_400` | 1583.0 µs | 1442.0 µs | 1655.0 µs |

## delaunay_2d_circle_150

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_circle_150` | 383609.0 ns | 374439.0 ns | 616334.0 ns +61% |

## triangulate_600gon

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `triangulate_600gon` | 1616.0 µs | 1596.0 µs | 1630.0 µs |

## triangulate_comb_600

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `triangulate_comb_600` | 5737.0 µs | 5530.0 µs | 5562.0 µs |

## triangulate_hex_6

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `triangulate_hex_6` | 1463.0 ns | 1205.0 ns **-18%** | 1221.0 ns **-17%** |

## svd_golub_kahan_12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `svd_golub_kahan_12` | 158752.0 ns | 119192.0 ns **-25%** | 125385.0 ns **-21%** |

## eigen_qr_12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `eigen_qr_12` | 119270.0 ns | 90068.0 ns **-24%** | 93775.0 ns **-21%** |

## gjk_epa_3d_cyl_box

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_3d_cyl_box` | 109327.0 ns | 83574.0 ns **-24%** | 86159.0 ns **-21%** |

## mpr_penetration_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes` | 17377.0 ns | 13444.0 ns **-23%** | 12269.0 ns **-29%** |

## gjk_epa_boxes

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes` | 17450.0 ns | 13661.0 ns **-22%** | 13263.0 ns **-24%** |

## gjk_epa_spheres

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres` | 536604.0 ns | 420130.0 ns **-22%** | 430216.0 ns **-20%** |

## gjk_epa_sphere_box

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_sphere_box` | 111531.0 ns | 82406.0 ns **-26%** | 84334.0 ns **-24%** |

## gjk_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss` | 659.0 ns | 464.0 ns **-30%** | 540.0 ns **-18%** |

## gjk_intersect_sph_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_sph_miss` | 905.0 ns | 680.0 ns **-25%** | 778.0 ns **-14%** |

## gjk_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit` | 1608.0 ns | 1045.0 ns **-35%** | 1660.0 ns |

## gjk_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_tangent` | 1840.0 ns | 1181.0 ns **-36%** | 1796.0 ns |

## mpr_intersect_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_miss` | 1372.0 ns | 990.0 ns **-28%** | 1096.0 ns **-20%** |

## mpr_intersect_box_hit

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_box_hit` | 4898.0 ns | 3516.0 ns **-28%** | 3334.0 ns **-32%** |

## mpr_intersect_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_tangent` | 4997.0 ns | 3607.0 ns **-28%** | 4187.0 ns **-16%** |

## bvh_scatter_4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `bvh_scatter_4k` | 5806.0 µs | 4868.0 µs **-16%** | 5136.0 µs **-12%** |

## spatial_hash_query_2k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `spatial_hash_query_2k` | 428040.0 ns | 379012.0 ns **-11%** | 422796.0 ns |

## num_dct_1024

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_dct_1024` | 293904.0 ns | 283624.0 ns | 274960.0 ns |

## num_dst_1024

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_dst_1024` | 6615.0 µs | 6670.0 µs | 6911.0 µs |

## num_dct_1023

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_dct_1023` | 1565.0 µs | 1603.0 µs | 1658.0 µs |

## num_dst_1023

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_dst_1023` | 448728.0 ns | 452891.0 ns | 455807.0 ns |

## jac_rev_shared_256

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `jac_rev_shared_256` | — | 347749.0 ns | 1169.0 µs |

## jac_rev_pertape_256

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `jac_rev_pertape_256` | — | 185211.0 ns | 190504.0 ns |

## cga_first_product_cold

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `cga_first_product_cold` | — | — | 205566.0 ns |

## cga_point_product

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `cga_point_product` | — | — | 1207.0 ns |

## cga_norm_sq_k5

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k5` | — | — | 1317.0 ns |

## cga_norm_sq_k32

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `cga_norm_sq_k32` | — | — | 29180.0 ns |

## vec3_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `vec3_lerp` | — | — | 22.00 ns |

## vec2_lerp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `vec2_lerp` | — | — | 18.00 ns |

## m3_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `m3_mul` | — | — | 93.00 ns |

## m3_inverse

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `m3_inverse` | — | — | 184.0 ns |

## m3_mul_vec3

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `m3_mul_vec3` | — | — | 41.00 ns |

## ivl_mul

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ivl_mul` | — | — | 208.0 ns |

## ivl_sin

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ivl_sin` | — | — | 139.0 ns |

## ivl_width

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ivl_width` | — | — | 15.00 ns |

## calc_adaptive_simpson

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `calc_adaptive_simpson` | — | — | 7763.0 ns |

## calc_bspline_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `calc_bspline_cubic_8` | — | — | 436.0 ns |

## calc_nurbs_cubic_8

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `calc_nurbs_cubic_8` | — | — | 558.0 ns |

## calc_hessian_4

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `calc_hessian_4` | — | — | 2111.0 ns |

## perlin_3d

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `perlin_3d` | — | — | 198.0 ns |

## christoffel_symbols_4d

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `christoffel_symbols_4d` | — | — | 6536.0 ns |

## riemann_tensor_4d

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `riemann_tensor_4d` | — | — | 30894.0 ns |

## geodesic_rk4_4d_100

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `geodesic_rk4_4d_100` | — | — | 233133.0 ns |

## su2_exp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `su2_exp` | — | — | 107.0 ns |

## su2_log

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `su2_log` | — | — | 131.0 ns |

## lorentz_boost

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `lorentz_boost` | — | — | 305.0 ns |

## so3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `so3_exp` | — | — | 239.0 ns |

## so3_log

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `so3_log` | — | — | 164.0 ns |

## se3_exp

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `se3_exp` | — | — | 682.0 ns |

## se3_log

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `se3_log` | — | — | 663.0 ns |

## simplex_2d

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `simplex_2d` | — | — | 89.00 ns |

## simplex_3d

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `simplex_3d` | — | — | 161.0 ns |

## ode_dopri45_step

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_step` | — | — | 247.0 ns |

## ode_dopri45_trajectory

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ode_dopri45_trajectory` | — | — | 36173.0 ns |

## ode_bdf2_step

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ode_bdf2_step` | — | — | 464.0 ns |

## ode_yoshida4_step

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `ode_yoshida4_step` | — | — | 64.00 ns |

## opt_lbfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `opt_lbfgs_rosenbrock` | — | — | 255719.0 ns |

## opt_bfgs_rosenbrock

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `opt_bfgs_rosenbrock` | — | — | 9807.0 ns |

## opt_lm_expfit_32

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `opt_lm_expfit_32` | — | — | 10873.0 ns |

## expr_eval

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `expr_eval` | — | — | 541.0 ns |

## expr_diff_simplify

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `expr_diff_simplify` | — | — | 1187.0 ns |

## sym_integrate

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `sym_integrate` | — | — | 356.0 ns |

## sym_rewrite_default

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `sym_rewrite_default` | — | — | 6718.0 ns |

## csr_spmv_256

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `csr_spmv_256` | — | — | 4612.0 ns |

## solve_gmres_64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `solve_gmres_64` | — | — | 76407.0 ns |

## solve_bicgstab_64

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `solve_bicgstab_64` | — | — | 26949.0 ns |

## eigen_symmetric_12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `eigen_symmetric_12` | — | — | 295589.0 ns |

## f64_fmod

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `f64_fmod` | — | — | 21.00 ns |

## f64_fmod_gap2097

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `f64_fmod_gap2097` | — | — | 4236.0 ns |

## num_is_prime_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_is_prime_2e63` | — | — | 418614.0 ns |

## num_pollard_rho_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_pollard_rho_2e63` | — | — | 78629.0 µs |

## num_crt_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_crt_2e63` | — | — | 1059.0 ns |

## num_modinv_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_modinv_2e63` | — | — | 44.00 ns |

## num_divisor_sigma_720720

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma_720720` | — | — | 410.0 ns |

## num_divisor_sigma0_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `num_divisor_sigma0_2e63` | — | — | 335109.0 ns |

## dual_pow_x3

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `dual_pow_x3` | — | — | 235.0 ns |

## hvec3_angle

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `hvec3_angle` | — | — | 555.0 ns |

## perlin_2d_2e63

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `perlin_2d_2e63` | — | — | 84.00 ns |

## delaunay_2d_mixed_100

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `delaunay_2d_mixed_100` | — | — | 934135.0 ns |

## bvh_query_ray_oblique_200x4k

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `bvh_query_ray_oblique_200x4k` | — | — | 767499.0 ns |

## cga_blade_inverse_sphere

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_sphere` | — | — | 1777.0 ns |

## cga_blade_inverse_tiny

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `cga_blade_inverse_tiny` | — | — | 2183.0 ns |

## eigen_power_3x3

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `eigen_power_3x3` | — | — | 7308.0 ns |

## eigen_power_12x12

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `eigen_power_12x12` | — | — | 22466.0 ns |

## eigen_power_2x2_wide

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `eigen_power_2x2_wide` | — | — | 13093.0 ns |

## mpr_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `mpr_intersect_near_tangent` | — | — | 236597.0 ns |

## gjk_intersect_near_tangent

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_near_tangent` | — | — | 856496.0 ns |

## gjk_intersect_box_miss_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_miss_2e600` | — | — | 767.0 ns |

## gjk_intersect_box_hit_2e600

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_intersect_box_hit_2e600` | — | — | 3654.0 ns |

## gjk_epa_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_boxes_separated` | — | — | 776.0 ns |

## gjk_epa_spheres_separated

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `gjk_epa_spheres_separated` | — | — | 1076.0 ns |

## mpr_penetration_boxes_separated

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `mpr_penetration_boxes_separated` | — | — | 1323.0 ns |

## time_of_impact_sph_hit

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_sph_hit` | — | — | 96981.0 ns |

## time_of_impact_box_miss

| Benchmark | Baseline (`a09d228`) | Mid (`d740afc`) | Current (`8265681`, 3.3.6 uncommitted) |
|-----------|------|------|------|
| `time_of_impact_box_miss` | — | — | 1961.0 ns |

---

Generated by `./scripts/bench-history.sh`. History in `bench-history.csv`.
