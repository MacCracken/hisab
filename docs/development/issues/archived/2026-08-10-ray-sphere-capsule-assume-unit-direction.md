# `geo_ray_sphere` and `geo_ray_capsule` silently return a `t` that is not on the surface when the ray direction is not unit-length — the other four primitives handle it

**Status:** 🟢 **FIXED 2026-08-10 (2.10.0 first bite).** Found while scoping 2.10.0.

## Resolution

**One root cause, not two.** `geo_ray_capsule`'s own cylinder quadratic was already correct — it
carries `a = dot(d_perp, d_perp)` and divides by `2a` (`geo.cyr:465, 493`). It failed the sweep only
because its **end-cap branches call `geo_ray_sphere`**. Repairing the sphere alone took the sweep
from 5 bad to 0, which confirms the single cause rather than assuming it.

Fix: option 1 as recommended — `geo_ray_sphere` carries `a = d·d` explicitly through both the
discriminant and the roots, plus a degenerate-direction guard (`a < EPSILON_F64 → miss`) since
carrying `a` means dividing by it.

**Cost, measured** — interleaved A/B, three runs each, `bench_batch` timing: `ray_sphere`
**81.7 ns → 96.3 ns, +17.6%**, distributions non-overlapping with <4% spread; `ray_triangle`
unchanged as a control (287/291/294 vs 285/288/288). ⚠ The single-run CSV pair reads +28.6%; the
interleaved figure is the one to quote. The cost is real and accepted: this is a silent-wrong-answer
repair, and 14.6 ns is the price of the primitive agreeing with its own documented contract.

⚠ **This was only measurable because the benchmark harness was fixed first.** On the pre-2026-08-10
harness `ray_sphere` read 1,466 ns of which ~95% was `clock_gettime`, so a 14.6 ns change would have
been invisible — the same harness that flattened a 3.6× triangle/sphere ratio into 1.19×.

**Assertions** (`tests/modules.tcyr`, group *"2.10.0 geo_ray_*: t is degree -1 homogeneous"*): an
oracle-free sweep of all 6 primitives × 3 direction scales (`_HG_BAD == 0`, 18 probes) — it compares
each routine against itself at two scales, so it needs no reference and cannot inherit a convention
from the code under test; plus a direct on-surface check that `geo_ray_at(t)` lands on the sphere for
a non-unit ray, and zero-direction guards for sphere and capsule. **Three mutants caught**: the
pre-fix reduced form (5 bad + off-surface), `a` in the discriminant but not the roots (4 bad), and
dropping the degenerate guard (returns a huge negative instead of the miss sentinel).

⚠ **The degenerate guard survived the first mutation round** — nothing asserted it, so it could have
been deleted freely. The zero-direction assertions were added in response, which is the second time
this release that order-consistency-style coverage turned out to be weaker than it looked.

`geo_ray_capsule` also gained its first benchmark (**520 ns**); it had none, which would have left
the indirect change to it unmeasured.
**Severity:** **Medium.** Silent wrong output, not a crash or an error return. Unreachable through
`geo_ray_new` alone, reachable through the public derived setter `GeoRay_set_direction`.
**Affects:** all versions with the current `geo.cyr` (the half-`b` sphere form predates 2.9.x).

## The defect

`geo_ray_*` is six sibling routines with one documented contract — *"intersect this ray with this
shape, return the parameter `t`, 0 for a miss"* — where the hit point is `geo_ray_at(ray, t)` =
`o + t·d`. That contract makes `t` **degree −1 homogeneous in `d`**: scale the direction by `k` and
the same geometric point is reached at `t/k`.

**Four of the six honour that. Two do not.** Measured by scaling a unit direction by `k = 2` and
comparing the returned `t` against `t_unit / k`:

| primitive | `t(2d)` | expected `t/2` | |
|---|--:|--:|---|
| `geo_ray_sphere` | 2.2872117 | 3.8301270 | ❌ **wrong** |
| `geo_ray_plane` | 4.3301270 | 4.3301270 | ✅ |
| `geo_ray_triangle` | 4.3301270 | 4.3301270 | ✅ |
| `geo_ray_aabb` | 3.4641016 | 3.4641016 | ✅ |
| `geo_ray_capsule` | 2.1539078 | 3.7177546 | ❌ **wrong** |
| `geo_ray_obb` | 3.4641016 | 3.4641016 | ✅ |

The two that fail are exactly the two that solve a **quadratic** and assume its leading coefficient
`a = d·d` equals 1.

## Why it is worse than "returns a different number"

The returned `t` does not identify a point on the surface at all. Unit sphere at
`c = (2,4,6)`, ray from the origin:

```
UNIT direction:              shipped t = 6.483314774   |o + t·d − c| = 1.000000000   ✅ on the sphere
NON-UNIT d = (1,2,3):        shipped t = 1.000000000   |o + t·d − c| = 3.741657387   ❌
    (exact root is 1.732738758)
NON-UNIT d = (0.5,1,1.5):    shipped t = 2.125657913   |o + t·d − c| = 3.506572958   ❌
    (exact root is 3.465477516)
```

So a caller doing the documented thing — `var t = geo_ray_sphere(r, s); var p = geo_ray_at(r, t);` —
gets a point **3.74 units from the centre of a radius-1 sphere**, with no error signalled.

## Root cause

`src/geo.cyr:256-272`. The routine uses the *half-b* quadratic form:

```
var half_b = hvec3_dot(oc, GeoRay_direction(ray));
var c = f64_sub(hvec3_dot(oc, oc), f64_mul(r, r));
var discriminant = f64_sub(f64_mul(half_b, half_b), c);   # <- a == 1 assumed
...
var t1 = f64_sub(f64_neg(half_b), sqrt_d);                # <- no /a
```

That is the standard reduced form for a **unit** direction: with `a = d·d`, the roots are
`(−half_b ± sqrt(half_b² − a·c))/a`. Both the discriminant and the division silently drop `a`.
`geo_ray_capsule` (`:446`) has the same shape in its cylinder-body branch.

## Is it reachable?

`geo_ray_new` (`:27`) normalizes, so the intended construction path is safe, and it is the only
constructor in `src/`. **But `GeoRay` is `#derive(accessors)`, so `GeoRay_set_direction` is public
API**, and nothing in the struct, the constructor, or the routine comments states a unit-length
precondition. A consumer that stores a ray and rescales its direction — a perfectly ordinary thing
to do when the direction carries a speed or a step size — gets silently wrong intersections from two
of the six primitives and correct ones from the other four.

## Why this surfaced now, and why 2.10.0 cannot proceed without deciding it

2.10.0 is differentiable geometry. The analytic derivative `dt/dd` is derived from the implicit
function theorem on `F(o + t·d; θ) = 0`, which **assumes the homogeneity above**. Euler's relation
`d·(∂t/∂d) = −t` is a direct consequence, and it is what first flagged this — by finite difference
against the shipped primal:

```
plane      d·(dt/dd) = −6.236095645   −t = −6.236095645   holds exactly
sphere     d·(dt/dd) = −48.516685269  −t = −6.483314774   MISMATCH
```

If the primal is not homogeneous, an analytic `dt/dd` and the shipped `t` describe different
functions. So the jets cannot be built on `geo_ray_sphere`/`geo_ray_capsule` as they stand.

⚠ **A harness note, recorded because it nearly produced a false finding.** The same check on
`geo_ray_triangle` first reported a mismatch with a relative error of 267,260 — an artefact, not a
defect. The perturbed direction moved the ray off the triangle, `geo_ray_triangle` correctly returned
`0` for a miss, and the finite difference divided that step by `2h`, giving `t/(2×10⁻⁶) ≈ 1.87e6`
— which is exactly the number that appeared. Any FD probe over these routines must reject samples
where either evaluation returns the miss sentinel.

## Options

1. **Use the full quadratic** — compute `a = hvec3_dot(d, d)` and divide. One extra dot product and
   one divide on each of two routines; makes all six siblings consistent and makes the 2.10.0 jets
   well-posed. `ray_sphere` currently measures **79 ns** (`bench_batch`), so the cost is measurable
   and must be benchmarked, not asserted.
2. **Normalize defensively inside the two routines** — costs a `sqrt`, and silently changes the
   meaning of `t` (it would become distance rather than the ray parameter), which would break the
   `geo_ray_at` round trip. **Not recommended.**
3. **Document the precondition and enforce it** — state unit-length in the `GeoRay` contract, and
   have the two routines detect `|d·d − 1| > eps` and return the error sentinel. Honest, cheaper than
   (1), but makes two primitives reject input the other four accept.

Option 1 is the one consistent with how this project resolved the equivalent sibling asymmetry in
2.9.1 (`mpr_intersect` vs `gjk_intersect_3d`, where "keep the strict reading and document it" was
explicitly rejected because a contract that depends on how you got there is not a contract).

**Whichever is chosen it needs an assertion**: the homogeneity sweep above, over all six primitives
and several `k`, is the natural gate — it is oracle-free (it compares the routine against itself at
two scales) and it catches all three options' failure modes.
