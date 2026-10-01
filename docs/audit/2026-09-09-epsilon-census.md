# 2026-09-09 — the epsilon census

**Scope.** Every `EPSILON_F64` comparison guard in `src/`, classified, then each
classification independently re-derived by a second agent instructed to refute it.

## What the census actually found

The roadmap sized the epsilon tier at **"~20 sites"**. It is **97 confirmed
defects across 23 modules**, from a census of **136 guards in 25 modules**.

⚠ **The estimate came from a summary, not from the code** — the same mistake
2.11.3 made when it sized the ganita migration at 8 call sites and it was 536.
The "~20" traces to the 2026-08-11 audit's *confirmed* table, which is a list of
instances someone happened to reproduce, not a census of the class.

| Verdict | Triage (136) | Independent verify (102) |
|---|---|---|
| `DEFECT_ABSOLUTE_ON_SCALING` | 89 | 87 |
| `DEFECT_SQUARED_VS_UNSQUARED` | 11 | 8 |
| `DEFECT_WRONG_QUANTITY` | 1 | 2 |
| `CORRECT_TOLERANCE` | 18 | 3 |
| `CORRECT_SCALE_FREE` | 17 | 2 |

**The verify pass disagreed with triage on 5 of 102** — three flipped
DEFECT → CORRECT (`collision_core:709`, `lie_ext:443`, `linalg_precision:590`),
two were reclassified within DEFECT. That is a ~1-in-20 error rate on triage
alone, against the ~1-in-12 that 2.11.1 measured, and it is the reason nothing
here was repaired on a single agent's say-so.

⚠ **The first run of this census lost 30 of 41 agents to a session limit** and
reported 11 results as though they were the whole picture. The resumed run
completed 121 agents with 0 errors. A partial fan-out reads exactly like a
complete one unless the failure count is checked.

## Repaired in 2.14.0 — 23 confirmed sites, plus 3 the census missed

Every one mutation-proven: 58 mutants installed, all killed, restore verified
byte-identical each time.

| Module | Sites | Entry points |
|---|---|---|
| `vec3` | 1 | `hvec3_angle` |
| `calc_ext` | 6 | `calc_bspline`, `calc_nurbs` ×2, `calc_hermite_tcb` ×2, `calc_monotone_cubic` |
| `geo` | 10 | `geo_ray_plane`, `geo_ray_sphere`, `geo_ray_triangle`, `geo_ray_capsule`, `geo_closest_point_on_sphere`, `geo_barycentric_coords`, `geo_triangle_unit_normal`, `geo_segment_direction`, `geo_segment_closest_point` |
| `geo_diff` | 6 | all six jets |
| `vec2`/`vec3`/`vec4` | 3 | `hvecN_normalize` |

⭐ **Three of those were NOT on the census list and were found by grepping for
the shape**: `geo_diff.cyr:204` (`geo_jet_plane`), `src/vec3.cyr` (`hvec3_normalize`
— the one that feeds `geo_ray_new`, EPA and every unit-direction path in the
tree), and the four `geo.cyr` squared-length guards. The same lesson as 2.12.0's
"the tier said 3 allocation sites; it was 4".

## ~~Still open — 74 confirmed defects~~ → CLOSED IN 2.15.0

⭐ **2.15.0 repaired 73 of the 74**, every one mutation-proven; **1 is deferred**
with its reason recorded. The table below is the state as this section was
written, kept because the per-module shape is the useful record; the live
disposition is the `disposition` field on each entry of the JSON beside it.

⛔ **`su2_log` is the deferral, and it is a formula change rather than a
threshold.** Lowering its guard alone would introduce a NEW defect: the log and
exp maps divide by `theta^2` and `theta^3`, and at `theta <= 2.2e-162` both
`theta*theta` and `1 - cos(theta)` are exactly 0, so the coefficient becomes
`0/0 = NaN`. It is grouped with `so3_log`, `se3_exp` and `se3_log`, which were
never on this list but have the identical shape — three more sites the census
missed and a grep found.

> **Later disposition (added 2026-09-30, v3.3.2).** The deferral closed in **2.16.0**
> (`CHANGELOG.md` [2.16.0]), and `su2_log` needed no series after all. It and `so3_log` now take
> the angle from `atan2` of a scaled norm instead of `acos`, which had been losing the whole
> rotation below θ ≈ 1.5e-8. The series went to the `se3_exp` and `se3_log` coefficients below
> θ = 0.1. Every `Open` count in the table below is therefore closed: 73 sites in 2.15.0 and
> `su2_log` in 2.16.0. The JSON's `annotation_3_3_2` field records the same.

Each entry has a verified `guarded_quantity`, a `correct_threshold` and a
bracketing `scale_test`.

⭐ **The full evidence is committed beside this file** as
[`2026-09-09-epsilon-open.json`](2026-09-09-epsilon-open.json) — 74 entries, every field
as the verifier wrote it. It was extracted from the run journal at
`.claude/.../workflows/wf_1d8434cc-58d/journal.jsonl`, which lives outside the repo
and is not durable; the census cost ~12.5M subagent tokens to produce and would
have to be re-run from scratch if that journal were lost.

⚠ **Re-locate each site by its quoted guard text, not by its line number.** The
numbers are as of the 2.13.0/2.14.0 tree, several were already corrected by the
verifier against stale triage output, and 2.14.0's own repairs shifted lines in
`geo.cyr`, `calc_ext.cyr`, `geo_diff.cyr` and the three vector modules.

| Module | Open | Functions |
|---|---|---|
| `geo_advanced` | 18 | `_bvh_ray_hits_aabb`, `_epa_degenerate_normal`, `cga_blade_inverse`, `cga_geometric_product`, `cga_left_contraction`, `cga_outer_product`, `cga_plane`, `cga_right_contraction`, `cga_rotor`, `time_of_impact` |
| `linalg_ext` | 12 | `cmat_inverse`, `csr_add`, `csr_from_dense`, `eigen_power`, `lyapunov_max`, `solve_gmres`, `solve_pgs` |
| `linalg_precision` | 9 | `_lp_bidiag_qr`, `_lp_bidiagonalize`, `_lp_tridiag_qr`, `_lp_tridiagonalize`, `cqr_decompose` |
| `collision_core` | 6 | `_col_ring_is_convex`, `_col_xy_greater`, `sequential_impulse`, `triangulate_polygon` |
| `lie` | 6 | `lorentz_boost`, `lorentz_rotate`, `su2_exp`, `su2_from_axis_angle`, `su2_from_quat`, `su2_log` |
| `?` | 4 | `opt_bfgs`, `opt_conjugate_gradient`, `opt_lbfgs` |
| `complex` | 3 | `cx_div`, `cx_inv`, `cx_powf` |
| `num_ext` | 3 | `num_tridiag_solve` |
| `color` | 2 | `linearize_depth_reverse_z`, `unpremultiply_alpha` |
| `lie_ext` | 2 | `so3_exp`, `so3_from_axis_angle` |
| `mat4` | 2 | `m4_inverse`, `m4_transform_point` |
| `transforms` | 2 | `hisab_inverse_lerp`, `world_to_screen` |
| `diffgeo` | 1 | `sectional_curvature` |
| `f64_util` | 1 | `f64_fmod` |
| `mat3` | 1 | `m3_inverse` |
| `num` | 1 | `num_newton` |
| `ode` | 1 | `ode_dopri45_trajectory` |
⚠ The `?` row is `optimize` — four sites whose verifier reported the function
name without a file-qualified line, so they need re-locating before repair.

⭐ **`cx_div` is in this list.** `complex.cyr:58` has carried a comment naming
this exact defect class since 2.6.14, and `cx_div` is named *in that comment* and
is still fabricating zero. Writing the lesson beside the code did not fix the
code — which is the whole reason this census was a grep and not a re-read.
