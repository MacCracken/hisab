# geo_ray_capsule's cyl_missed fallback returns a point INSIDE the solid

**Filed**: 2026-08-10 (during 2.10.1, by a commissioned adversarial probe)
**Status**: 🐞 **OPEN** — scheduled for **2.10.2**. Not fixed in 2.10.1.
**Severity**: High — a wrong finite `t`, not a miss and not a NaN
**Affects**: `geo_ray_capsule` / `geo_ray_capsule_branch`, and therefore `geo_jet_capsule`

## Summary

When the infinite-cylinder test yields nothing — the ray is parallel to the axis, or misses the
cylinder — `geo_ray_capsule` falls back to the two cap **spheres** and returns the smaller root:

```
if (cyl_missed == 1) {
    var t1 = geo_ray_sphere(ray, sphere_at_start);
    var t2 = geo_ray_sphere(ray, sphere_at_end);
    if (t1 == 0) { ... return t2; }
    ...
}
```

**No hemisphere test is applied on this path.** The main path filters each cap hit by the
half-space that makes it belong to that cap (`dot_ab <= 0` for start, `>= 0` for end); the fallback
does not. So it can return the entry point of a cap *sphere* that lies **inside** the capsule.

## Measurement — reproduced independently on the 2.10.1 tree

```
capsule = geo_capsule_new((0,0,0), (10,0,0), r = 1)
ray     = origin (5,0,0), direction (1,0,0)     <- along the axis, starting inside

geo_ray_capsule            -> t = 4.000000
geo_ray_at(ray, 4)         -> (9, 0, 0)
distance from that point to the segment -> 0.000000        against r = 1.000000
```

The returned point is **on the axis**, a full radius inside the solid. The true exit is `t = 6`,
giving `(11,0,0)` on the end cap. `geo_ray_capsule_branch` reports `GEO_CAP_END_CAP`, and
`geo_jet_capsule` returns a confident jet — `geo_ray_sphere` correctly returned the *entry* root of
the end-cap sphere at `t = 4`, which is a real point on that sphere; it is simply not on the
capsule.

An axis-aligned capsule with an axis-aligned ray is not an exotic configuration.

## Second symptom, same function, different guard

The probe also found that **interior origins lose the exit through a cap**: over 600 random
inside-origin configurations, 32 (**5.3%**) returned the miss sentinel where a brute-force march
finds a real exit, with dropped exits ranging `t = 1.09` to `t = 10.0`. Example:
`capsule (0,0,0)-(10,0,0) r = 1`, origin `(5,0,0)`, direction `normalize(1, 0.001, 0)` returns
exactly 0 against a marched exit of `5.999985`.

Every sibling — `geo_ray_sphere`, `geo_ray_aabb`, `geo_ray_obb`, and this function's own
cylinder-wall branch — returns the exit parameter when the origin is inside. This branch does not.
Filed together because a repair of the cap acceptance logic has to answer both.

## Why it is not fixed in 2.10.1

It changes what the primal returns on inputs where it currently returns something, on a function
with a benchmark and four consumers, and the right acceptance rule for the fallback path needs
deciding rather than guessing — in particular whether an interior origin should report the exit
(matching every sibling) or continue to report a miss. That is a contract decision, and 2.10.1's
scope was the jets.

⚠ **`geo_jet_capsule` inherits it.** The jet reads the branch tag from the primal and trusts it, so
on the case above it hands back a gradient for a point a full radius inside the solid. The
commissioned probe's independent design recomputed `u` from the hit point and cross-checked
`| |g| - r |` against the radius, which catches it; the branch-tag design does not. ⚠ **The grazing
guard does not help here** — for a near-parallel ray `|den| ≈ r·|d_perp|` sits far above
`EPSILON_F64` while the point is off-surface, so a surface-residual check is the load-bearing one.

**Whether the jet should carry a surface-residual check is the open design question**, and it is
not free: it is a second correctness criterion on the hot path of a routine whose whole design is
"trust the primal, add no second copy". The alternative — fix the primal — is what this filing
schedules.

## Provenance

Found by an adversarial probe commissioned against 2.10.1's finished code and re-run on this tree
before being written down.
