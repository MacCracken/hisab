# Migrating to hisab 3.5.0

> 3.5.0 is the **constants and conventions** release. It does four things a consumer can see:
>
> - **16 public functions are retired** for replacements that refuse where the old name
>   answered with a plausible value. The old names are removed, not redefined, so a call to
>   one does not build.
> - **One signature changes**: `world_to_screen_checked` takes a fifth parameter, the projection's
>   NDC depth range.
> - **23 `public var` constants become enum members** (K003), with the same names and values.
>   Reading them compiles as before; assigning to one no longer compiles.
> - **`spatial_hash_insert` and `spatial_hash_clear` are `#must_use`.**
>
> Many functions keep their names and answer differently, nearly all of them where they were
> wrong. They are listed under [Answers that change](#answers-that-change).

## TL;DR

```cyrius
# 3.4
var a  = hvec3_angle(u, v);                          # 0 for a zero vector: reads "parallel"
var s  = world_to_screen_checked(p, mvp, w, h);      # depth assumed GL's [-1, 1] NDC z
var mu = num_mobius(n);                              # 0 for n <= 0, and for every squareful n

# 3.5
var at, a = hvec3_angle_checked(u, v);               # Err(DIVISION_BY_ZERO) for a zero vector
var st, s = world_to_screen_checked(p, mvp, w, h, NDC_Z_MINUS_ONE_TO_ONE);
var mt, mu = num_mobius_checked(n);                  # Err(INVALID_INPUT) for n <= 0
```

Every replacement is `#must_use`. The rule from [migration-3.0.md](migration-3.0.md) and
[migration-3.4.md](migration-3.4.md) still holds: **a `Result` in argument position passes
only its tag** (0 for Ok, 1 for Err). Bind both halves (`var t, v = f(...)`) before using the
value. `scripts/check-result-migration.sh` enforces this inside hisab. A consumer's compiler
does not check it for you.

## Does this affect me?

Search your sources for the names below. Of the 15 repositories that declared `[deps.hisab]` on
2026-11-14, **none calls a retired name or the old `world_to_screen_checked` arity, and none
assigns one of the 23 constants**. pravash's `scripts/probes/p1_memory.tcyr` discards
`spatial_hash_insert` / `spatial_hash_clear` results, so it now gets a `#must_use` warning.
[measured: grep of every `~/Repos/*` with `[deps.hisab]`, 2026-11-14]

## Retired names

| 3.4 name | 3.5 replacement | What the old name did wrong |
|---|---|---|
| `t2d_apply(t, p)` | `t2d_transform_point(t, p)` → HVec2 | returned an HVec3 (x, y, 1), read back through HVec2 accessors (`D090`) |
| `t2d_compose(a, b)` | `t2d_compose_checked(a, b)` → Result | a rotation then a non-uniform scale is a shear, which no TRS triple holds; the answer was 4× off (`F-X2`) |
| `t3d_compose(a, b)` | `t3d_compose_checked(a, b)` → Result | the same; also refuses a zero quaternion |
| `tensor_get` / `tensor_set` / `tensor_get2` / `tensor_set2` | `tensor_get_checked` / `tensor_set_checked` / `tensor_get2_checked` / `tensor_set2_checked` → Result | no range check: `tensor_set(t, [2, 0], …)` on shape [2, 2] overwrote the next tensor (`AU3-F12`, rated high) |
| `ev100_to_luminance` / `luminance_to_ev100` | `ev100_to_avg_luminance` / `avg_luminance_to_ev100` | a 12.5/π factor with no source; the new pair is Lagarde/Filament's L = 2^(EV − 3) (K = 12.5, S = 100) (`SPC-F5`) |
| `m4_look_at_rh(eye, target, up)` | `m4_look_at_rh_checked(eye, target, up)` → Result | a rank-deficient matrix for eye = target or up ∥ forward (`AU2-G7-F6`) |
| `hquat_from_axis_angle(axis, angle)` | `hquat_from_axis_angle_checked(axis, angle)` → Result | multiplied the axis as given: (2, 0, 0) at π/2 rotated by 2.21 rad; a zero axis gave the identity (`MAT F4`) |
| `hvec3_angle(a, b)` | `hvec3_angle_checked(a, b)` → Result | 0 for a zero vector, the answer that means "parallel" |
| `sym_match(pat, e)` | `sym_match_checked(pat, e)` → Result: Ok(1) / Ok(0) | 0, "no match", for a refused allocation and for a 0 node |
| `sym_match_bindings(pat, e)` | `sym_match_bindings_checked(pat, e)` → Result: Ok(vec) / Ok(0) | the same |
| `sym_const_eq(a, b)` | `f64_eq(a, b)` | an absolute 1e-15 tolerance; folding and matching are exact now |
| `num_mobius(n)` | `num_mobius_checked(n)` → Result | 0 for n ≤ 0, which is also μ of every squareful n |

### The call shapes

```cyrius
# A value in the payload: bind both halves, test the tag.
var qt, q = hquat_from_axis_angle_checked(axis, angle);
if (is_err_result(qt) == 1) { … }     # q holds HSB_ERR_DIVISION_BY_ZERO (zero axis), _INVALID_INPUT, _ALLOC

# A store: Ok(0), and nothing is written on a refusal.
var wt, we = tensor_set_checked(t, idx, val);

# A boolean in the payload: Ok(0) is a definite "no", not a failure.
var mt, hit = sym_match_checked(pat, e);
if (is_err_result(mt) == 0 && hit == 1) { … }
```

- `t2d_compose_checked` / `t3d_compose_checked` answer `Err(HSB_ERR_INVALID_TRANSFORM)` for a
  shear, which no translation-rotation-scale triple can hold. Compose the matrices instead (`m4_mul`).
- `hvec3_angle_checked` keeps `hvec3_angle`'s bits on every finite non-zero pair.
- `hquat_from_axis_angle_checked` normalises the axis. For a unit axis the answer is 3.4's bits
  for 3.4's arguments. At angle ±0 every axis, the zero vector included, gives the identity.
- `t2d_transform_point` gives the x and y bits `t2d_apply` gave. Read the result as an HVec2, not
  an HVec3.

## `world_to_screen_checked` takes the depth range

```cyrius
# 3.4
var st, s = world_to_screen_checked(p, mvp, vw, vh);
# 3.5
var st, s = world_to_screen_checked(p, mvp, vw, vh, NDC_Z_MINUS_ONE_TO_ONE);   # 3.4's depth
var rt, r = world_to_screen_checked(p, mvp_rz, vw, vh, NDC_Z_ZERO_TO_ONE);     # reverse-Z, D3D, Vulkan
```

`NdcDepthRange` is a new public enum. `NDC_Z_MINUS_ONE_TO_ONE` keeps 3.4's depth (nz/2 + 1/2,
for `m4_perspective_rh` and `m4_ortho_rh`). `NDC_Z_ZERO_TO_ONE` returns nz exactly. Use it for
`m4_perspective_reverse_z`, whose depth `linearize_depth_reverse_z` reads. Any other value is
`Err(HSB_ERR_INVALID_INPUT)`. Every old call site fails to build until it names its range.

## Constants that became enum members (K003)

| Constants | Enum |
|---|---|
| `EPSILON_F64`, `F64_TINY`, `F64_POS_INF` | `HsbF64` |
| `GEO_FACE_NONE` | `GeoFace` |
| `GEO_FACE_OUT_BYTES` | `GeoFaceLayout` |
| `GEO_CAP_CYLINDER`, `GEO_CAP_START_CAP`, `GEO_CAP_END_CAP`, `GEO_CAP_SPHERE` | `GeoCapBranch` |
| `GEO_JET_MISS` … `GEO_JET_CAPSULE` (7) | `GeoJetKind` |
| `GJK_MAX_ITER`, `EPA_MAX_ITER` | `GaIterLimit` |
| `CGA_NUM_BLADES`, `CGA_MV_SIZE` | `CgaLayout` |
| `HALFEDGE_NONE` | `HalfEdgeIndex` |
| `AD_NONE`, `AD_FULL` | `AdIndex` |

Names and values are unchanged. Through 3.4.2 any consumer could assign one, and the assignment
took effect library-wide: `GJK_MAX_ITER = 0` stopped GJK, and `GEO_JET_SPHERE = 9` made the jet
accessors refuse sphere jets. Now `cannot assign to enum constant` is a compile error.

## New `_checked` forms beside names that stay

These are additions. The plain names keep their 3.4 answers.

- **collision**: `gjk_intersect_3d_checked`, `mpr_intersect_checked`, `gjk_epa_3d_checked`,
  `mpr_penetration_checked` and `time_of_impact_checked` return `Err(HSB_ERR_ALLOC)` for a
  refused allocation. The plain forms answer 0 there, which also means "apart".
- **geo**: `geo_triangle_unit_normal_checked` and `geo_segment_direction_checked` return
  `Err(HSB_ERR_DEGENERATE)` where the plain forms answer (0, 1, 0) or (1, 0, 0).
  `geo_jet_sphere_dradius_checked`, `geo_jet_plane_ddistance_checked` and
  `geo_jet_capsule_dradius_checked` refuse a kind mismatch or a miss jet, where the plain forms
  answer +0.
- **vectors**: `hvec2_normalize_checked`, `hvec3_normalize_checked` and `hvec4_normalize_checked`
  return `Err(HSB_ERR_DIVISION_BY_ZERO)` for the zero vector, where `hvecN_normalize` keeps
  returning it as a documented sentinel.
- **mat4**: `m4_transform_point_frexp_checked(m, v, out_e)` returns a projected point as
  significands and a power of two, for points past the double range.
- **linalg**: `matrix_condition_number_checked(A)` reports singular, invalid and refused inputs
  apart. The plain form answers -1 for all three. `cmat_inverse_checked(m)` returns
  `Err(HSB_ERR_INVALID_INPUT)` (null, non-square, a NaN or infinite part),
  `Err(HSB_ERR_SINGULAR_MATRIX)` (decided exactly), `Err(HSB_ERR_OUT_OF_RANGE)` (an entry past
  DBL_MAX) or `Err(HSB_ERR_ALLOC)`; `cmat_inverse` answers 0 for every one of those.
- **autodiff**: `ad_val_checked(t, i)` and `ad_grad_of_checked(t, i)` refuse an index that is not a
  node. The plain forms answer 0.

## Answers that change

All of these are cases where 3.4.2's answer was wrong, undefined or a stand-in. The CHANGELOG has
the measured counts for each.

**Geometry (`geo`, `geo_diff`)**
- `geo_ray_aabb[_face]` and `geo_ray_obb[_face]`: a hit past DBL_MAX from finite input is +Inf with
  its face, as the plane, sphere, triangle and capsule already answered. It was the miss 0.
- `geo_ray_triangle`: a NaN direction or vertex returns that NaN, not the miss 0.
- `geo_ray_plane`: the verdict is exact for finite input. A NaN origin or distance is NaN on every
  path. On a near-parallel ray, the t and hit/miss verdict now come from correctly rounded sums.
- `geo_triangle_normal` is the exact cross product rounded once per component, and
  `geo_triangle_area` is half its length. Values move on ordinary triangles, by an ulp or so.
- `geo_ray_sphere`: past the documented limits and at spheres whose r² and |o − c|² flush, the
  verdict and t are certified. Wrong answers there (a miss read as a hit at 1e30, t = 0 for a
  direction past 2^511) are right now; a few correct answers move by an ulp or two inside the
  4-ulp contract. One known class stays: a ray tangent from an origin under 2^-537 short of its
  tangent point reads t = 0.
- `geo_ray_capsule` / `geo_ray_capsule_branch`: a ray within 2^-48 of a long axis, a capsule longer
  than 2^511.5 and a projection past DBL_MAX are solved exactly; ordinary rays whose barrel term
  cancels move by an ulp or so toward the exact t. A ray along an ordinary capsule's axis keeps its
  bits.

**Collision and meshes**
- `halfedge_is_boundary` answers Ok(1) for an isolated vertex, as OpenMesh and CGAL do. It also
  answers Ok(1) for a vertex with a twinless half-edge on any of its fans.
- `cga_rotor`, `su2_from_axis_angle`, `so3_from_axis_angle`, `lorentz_boost`, `lorentz_rotate`:
  a zero axis at a non-zero angle gives NaN where the axis matters. It was the identity.
  Angle 0 still gives the identity.
- `sdf_smooth_union(Inf, Inf, k)` is +Inf, not NaN. At the clamped ends a finite answer can now be
  min(d1, d2)'s -0.
- `gjk_intersect_3d`, `mpr_intersect`, `gjk_epa_3d`, `mpr_penetration`, `time_of_impact`: a ball over
  a cylinder's or cone's cap close to its rim is a contact, where it was a miss. `gjk_epa_3d`'s deep
  contacts can take a face's closed form, so a depth can move by a fraction of an eps (it can now sit
  as far below the exact depth as your support function's own error allows).

**Complex numbers**
- `cx_div(a, 0)` and `cx_inv(0)` give C99's infinities, `(1+0i)/0 = +Inf + i NaN`. They were
  0 + 0i.
- `cx_powf(0, 0)` is 1. `cx_powf(0, n < 0)` is +Inf + i NaN. Both were 0.
- `cx_exp`, `cx_sin`, `cx_cos` and `cx_from_polar` past overflow keep the finite parts finite.
  `cx_mul` with finite operands no longer makes Inf − Inf NaN parts. `cx_ln` near |a| = 1 is
  within 1 ulp, where it was off by up to 2.4e21 ulp.

**Lie groups, differential geometry**
- `su2_from_quat(0)`: NaN, was the identity.
- `su3_structure_constant`: NaN on an invalid index, was 0.
- `gamma_spatial` / `gell_mann`: 0 out of range, was a zero matrix.
- `ricci_scalar`, `sectional_curvature` and `killing_residual`: NaN on invalid input, was 0.0.
- `sectional_curvature` is within 2^-46 of the exact K of its inputs, however much cancels; it was
  off by up to 3,270 ulps on indefinite metrics, and a near-null plane whose den rounded to 0 read 0,
  "flat" (0.5 one binade before). Such a plane now gets its curvature. Inputs its plain path
  certifies keep their bits; others change.
- `lorentz_is_valid(0)` and `se3_new` with a 0 part return 0, where they faulted.

**Transforms, colour, quaternions**
- `linearize_depth_reverse_z(±0, near)` is +Inf, the far plane at infinity. It was 0, the eye.
- `screen_to_world_ray` returns `HSB_ERR_DIVISION_BY_ZERO`, `_INVALID_INPUT` or `_OUT_OF_RANGE`
  where it returned rc 0 with a NaN direction.
- `hquat_slerp` of an identical pair is `a` for every finite t. It was NaN from t = 2^54.
- Tone maps give their limits at ±Inf.
- A 0 handle passed to a vec2, vec3, vec4 or quat function no longer faults. A handle result is the
  designed 0 (nothing is allocated); a double result (`_dot`, `_length_sq`, `_length`,
  `_distance`) is NaN, 0x7FF8000000000000; `hquat_normalize_checked`, `hquat_inverse_checked` and
  `hvec3_angle_checked` return `Err(HSB_ERR_INVALID_INPUT)`; an `_into` form writes NaN into every
  component of `out` and returns 0. A caller that tested a dot or a length against 0 to detect a
  refused allocation upstream gets NaN instead: test the handle.

**Symbolic, autodiff**
- `expr_simplify` folds exactly. `1e-20 * 1e30` was folded to 0, and `k_B * x` to 0. Two
  constants fold as the evaluator computes them, so 0 · +Inf is NaN and −(+0) is −0.
- `sym_integrate` handles ln(x), integrands free of x, x^g and a^x, where it answered 0
  ("unsupported") (`D076`).
- `expr_eval_checked` reports a refused allocation as `HSB_ERR_ALLOC`. It was `_INVALID_INPUT`.
- `ad_grad` refuses a hand-set tape it cannot read, writing NaN to every adjoint. It used to write
  past the adjoint buffer.
- `dual_pow` and `ad_pow` of a moving base at ±0 with an exponent that is not an integer have the
  derivative NaN, a domain miss, as `sqrt` at 0 has. x^0.5 at 0 was +Inf; x^1.5 and (x²)^1.5 at 0
  were 0, and (−|x|)^1.5 at 0 was −0. The value is unchanged. Where a model needs the one-sided
  derivative at 0, keep the base off 0 or write that derivative by hand. A constant zero no longer
  annihilates such a power: 0·x^0.5 and (x − x)·x^0.5 at 0 were 0 and are NaN now, in both modes.
- x^0 of a moving base whose value is NaN, such as (x^0.5)^0 at x < 0, has the derivative NaN. It was
  0; the value 1 is unchanged.
- `ad_grad` gives a finite derivative where it gave NaN and forward mode a number: a zero from a
  domain miss, or a constant zero, beside a partner with an infinite slope along an input the zero
  does not move with. sqrt(y)·(sqrt(z) + 1/x) at y = z = 0, x = 1e−200 has d/dx = 0 (was NaN). This
  applies from a root below 4,096. The answer is the tape's own sweep with those edges cut, not
  forward mode's, so a NaN can become ±Inf or a finite value carrying the tape's rounding (−2^942 for
  3 along x when 3x − x/(3x) at 1e−300 is added). A hand-set node may carry i2 = −4, the cut-node
  record: both sweeps read it as one edge and no second, and it is cut only beside a ±0 partial.

**ODE, optimisation, numbers**
- `ode_backward_euler_checked`, `ode_bdf2_checked` and `ode_bdf_checked` certify Newton's answer on
  the solution branch. Robertson's y2 is +3.9077e-5, where it was −7.54e-5 (a negative
  concentration). The certificate also reads g's slope at the answer, so the second root of a close
  pair is refused. An answer equal to the step's start, which was returned unread, is read along the
  branch's first move. ⚠ This costs about 20× per converging implicit step; see the CHANGELOG.
- `ode_bdf_checked` refuses a 0 history (it faulted).
- `ode_dopri45_trajectory` refuses a NaN or infinite y0.
- `opt_conjugate_gradient` takes 3.4.1's quasi-Newton repairs. More runs converge, in far fewer
  gradients.
- `num_halton(i, base < 2)` is NaN, not 0.0.
- `matrix_condition_number` gives cond_F, correctly rounded, for an invertible matrix whose
  elimination loses a pivot, where it answered -1 (singular). A 1×1 [x ≠ 0] is exactly 1.
- `cmat_inverse` answers such a matrix with a certified inverse, where it answered 0. A caller that
  read 0 as "singular or ill-conditioned" now gets an inverse; only a singular matrix gives 0.
- `eigen_power` answers `Err(HSB_ERR_NO_CONVERGENCE)` where its certificate cannot bound the powers'
  rounding. That covers wrong answers 3.4.2 gave as Ok, and also some right ones: strongly
  non-normal matrices with a subdominant ratio of 0.9 or more (about 2% of the families measured),
  and powers bounded only through a cancellation such as 1 + ac − ac with ac past 2^8. Every Ok it
  gives has 3.4.2's bits.

## Toolchain

hisab still pins cyrius **6.7.18**. The bundle's floor is unchanged: **hisab ≥ 3.1.0 needs
cyrius ≥ 6.6.3**. The roadmap's *Current* section records the 3.5.0 bundle's run under every
consumer pin.
