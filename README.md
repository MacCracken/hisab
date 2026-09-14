# Hisab

> **Hisab** (Arabic: حساب — calculation) — higher mathematics library for AGNOS

Higher math for the AGNOS ecosystem — linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, and more. Written in [Cyrius](https://github.com/MacCracken/cyrius), ported from Rust.

Used by **svara**, **naad**, **goonj**, **dhvani**, **attn11**, **ghurni**, **prani**, **garjan**, **prakash** and **nidhi** — ten repos that pull `dist/hisab.cyr` SHA-locked (verified 2026-09-09).

⚠ [impetus](https://github.com/MacCracken/impetus) (physics), [kiran](https://github.com/MacCracken/kiran) (engine), [joshua](https://github.com/MacCracken/joshua) (simulation), [aethersafha](https://github.com/MacCracken/aethersafha) (compositor), [hisab-mimamsa](https://github.com/MacCracken/hisab-mimamsa) (theoretical physics) and [kana](https://github.com/MacCracken/kana) (quantum science) are **planned** — they are Rust repos awaiting a Cyrius port and have no `cyrius.cyml` on any branch.

For expression evaluation and unit conversion, see [abaco](https://github.com/MacCracken/abaco).

## ⛔ 3.0.0 is breaking

48 functions move from integer `HSB_ERR_*` codes to the stdlib `Result<T, E>`. Read
**[docs/guides/migration-3.0.md](docs/guides/migration-3.0.md)** before upgrading.

**The one thing that will bite you**: a `Result` in *argument* position does not error — it silently
degrades to its tag, and `Ok` tag = 0 = `HSB_ERR_NONE`. An existing `assert_eq(f(...), HSB_ERR_NONE)`
therefore **keeps passing while testing nothing**; 98 lines in hisab's own suites would have gone
vacuous this way. Drive your migration with a grep, not with build errors —
`scripts/check-result-migration.sh` is there to copy.

There is **no deprecation window**, on this migration's own evidence: a dual API would leave exactly
those call sites quietly passing against the old function forever, which is the failure this release
removes. **2.24.0 is the supported 2.x line.**

## Modules

| Module | Files | Description |
|--------|-------|-------------|
| **Core** | f64_util, error | f64 helpers the stdlib does not carry; the `HSB_ERR_*` code set every fallible entry point returns |
| **Foundation** | vec2, vec3, vec4, quat, mat3, mat4 | Vector/matrix/quaternion types (f64, heap-allocated; SIMD `f64v_*` hot paths) |
| **Transforms** | transforms, color | 2D/3D affine transforms, projections, slerp/lerp, Euler angles, sRGB/HSV/HSL/Oklab, Porter-Duff compositing (8 ops), tone mapping (Reinhard, ACES), SH L2, EV/exposure |
| **Geometry** | geo, geo_advanced, geo_diff, spatial | 9 primitives, 6 ray tests, closest-point queries, GJK/EPA 3D, SDF+CSG, swept-AABB/TOI, conformal geometric algebra (5D CGA); spatial structures (BVH, k-d tree, octree, quadtree, spatial hash); differentiable ray/surface intersection for **all six** primitives — plane, sphere, triangle, aabb, obb and capsule jets returning the full gradient from one evaluation |
| **Collision** | collision_core, collision_mesh | MPR/XenoCollide narrowphase, sequential-impulse contact solver, convex hull 2D (monotone chain), polygon triangulation (ear clipping), Delaunay (Bowyer-Watson), half-edge mesh, island detection (union-find) |
| **Calculus** | calc, calc_ext, noise_simplex | Differentiation, integration (Simpson, Gauss-Legendre, adaptive), Bezier/Catmull-Rom/B-spline/NURBS/Hermite TCB/monotone cubic, easing, Perlin 2D+3D, simplex noise, gradient/Jacobian/Hessian |
| **Numerical** | num, ode, optimize, linalg_ext, linalg_precision, num_ext | Root finding, FFT/DST/DCT/2D-FFT, ODE (RK4, DOPRI45, BDF, symplectic, SDE), optimization (GD, CG, BFGS, L-BFGS, LM), sparse CSR, GMRES, PGS/LCP, SVD, eigendecomposition, compensated/high-precision linalg, number theory (primes, factorize, CRT, totient, Mobius), PCG32, Halton/Sobol, tridiagonal solver |
| **Complex** | complex | Complex numbers + matrices, Pauli/Dirac gamma matrices, matrix exponential |
| **Lie groups** | lie, lie_ext | U(1), SU(2), SU(3) Gell-Mann, SO(3,1) Lorentz, SE(3)/SO(3), exponential/log maps, adjoint, BCH |
| **Diff geometry** | diffgeo | Christoffel symbols, Riemann/Ricci/Einstein tensors, geodesic solver, Killing vectors, exterior algebra |
| **Symbolic** | symbolic, symbolic_ext | Expression tree, evaluate, differentiate, simplify, symbolic integration, LaTeX rendering, pattern matching + rewrite rules |
| **Autodiff** | autodiff | Forward-mode (dual numbers) **and** tape-based reverse mode — one sweep for an n-input gradient where forward needs n passes, measured 11.2x at n = 16. Pairs with `optimize.cyr`'s solvers through a capturing closure, no API change |
| **Interval** | interval | Interval arithmetic for verified numerics |
| **Tensor** | tensor, einsum | N-dimensional dense tensor, Kronecker delta, Minkowski metric, Levi-Civita, Einstein-summation (einsum) |

## Quick Start

```toml
# cyrius.cyml
[package]
name        = "your-project"
version     = "${file:VERSION}"
language    = "cyrius"
cyrius      = "6.6.4"

[deps]
# `ganita` (Cyrius 6.2.x) provides the transcendentals (acos/asin/atan2/pow/
# sinh/cosh/tanh + inverses) AND subsumes `matrix`/`linalg` — do not list those
# alongside it or you get duplicate-definition collisions. `math` stays for the
# inclusive comparisons, clamp/lerp/min/max/sign and the polyfills.
# `result` is REQUIRED as of 3.0.0 — every fallible hisab entry point returns
# Result<T, E>, so your own code needs Ok/Err/is_err_result in scope.
stdlib = ["string", "fmt", "alloc", "vec", "str", "math", "ganita", "tagged", "result", "fnptr"]

[deps.hisab]
git     = "https://github.com/MacCracken/hisab.git"
tag     = "3.0.1"
modules = ["dist/hisab.cyr"]   # ~1.1 MB self-contained bundle (all 35 modules)
# `dist/hisab.deps` is tracked as of 2.9.2 -- `cyrius deps` reads that sidecar and
# pulls in hisab's own 16 stdlib leaves, so the `stdlib` list above only has to
# name what *your* code uses.
# Or pull individual files for a smaller compilation unit. The per-module include
# sets are tabulated in docs/architecture/overview.md -- derived from the source
# and verified by compiling each of the 35 modules against exactly its listed set.
# The example below needs vec2/vec4 even though it never names them:
# modules = ["src/f64_util.cyr", "src/error.cyr", "src/vec2.cyr", "src/vec3.cyr", ...]
```

```cyrius
include "src/f64_util.cyr"
include "src/error.cyr"
include "src/vec2.cyr"
include "src/vec3.cyr"
include "src/vec4.cyr"
include "src/quat.cyr"
include "src/mat4.cyr"
include "src/transforms.cyr"
include "src/geo.cyr"
include "src/calc.cyr"
include "src/num.cyr"

alloc_init();

# 3D transform with composition
var pos = hvec3_new(f64_from(1), f64_from(2), f64_from(3));
var t = t3d_new(pos, hquat_identity(), hvec3_one());
var world = t3d_apply(t, hvec3_zero());

# Ray-sphere intersection
var ray = geo_ray_new(hvec3_zero(), hvec3_unit_z());
var sphere = geo_sphere_new(hvec3_new(0, 0, f64_from(5)), f64_from(1));
var t_hit = geo_ray_sphere(ray, sphere);  # 4.0

# Numerical integration (Simpson's rule)
# Fallible entry points return Result<T, E>: bind BOTH halves, never `var r = f()`.
fn x_squared(x) { return f64_mul(x, x); }
var result = alloc(8);
var int_t, int_v = calc_integral_simpson(&x_squared, 0, F64_ONE, 100, result);
if (is_err_result(int_t) == 1) { return 1; }
# load64(result) ≈ 0.3333

# Root finding (Newton-Raphson for sqrt(2))
fn f(x) { return f64_sub(f64_mul(x, x), F64_TWO); }
fn df(x) { return f64_mul(F64_TWO, x); }
var root = alloc(8);
var root_t, root_v = num_newton(&f, &df, F64_ONE, EPSILON_F64, 100, root);
if (is_err_result(root_t) == 1) { return 1; }
# load64(root) ≈ 1.41421356...
```

⚠ **`var r = calc_integral_simpson(...)` is a hard compile error** and discarding the return trips
`#must_use`. Passing a call like this straight into another function's argument list is worse — it
compiles, and the callee silently receives the **tag**. See
[docs/guides/migration-3.0.md](docs/guides/migration-3.0.md).

## Building

```sh
cyrius build src/main.cyr build/hisab
cyrius test tests/hisab.tcyr        # 550 cross-module integration assertions
cyrius test tests/foundation.tcyr   # 413 vec/quat/mat foundation assertions
cyrius test tests/modules.tcyr      # 2086 per-module assertions
cyrius test tests/edge_cases.tcyr   # 239 degenerate-input assertions
cyrius test tests/abuse.tcyr        # 914 hostile-input assertions (negative indices,
                                    #   zero/huge dimensions, non-conformable operands, canaries)
cyrius bench tests/hisab.bcyr       # 78 benchmarks
cyrius fuzz                         # 5 fuzz targets with invariant checks
```

## Architecture

See [docs/architecture/overview.md](docs/architecture/overview.md) for the full module map, data flow, and design principles, and [docs/architecture/math.md](docs/architecture/math.md) for the equation reference (CGA operators, plus a catalogue index of the other formula families).

## Stats

| Metric | Value |
|--------|-------|
| Version | 3.1.0 |
| Library | 35 modules, ~26,400 lines of Cyrius |
| Tests | 4214 assertions across 5 suites |
| Benchmarks | 78 operations |
| Fuzz targets | 5 with invariant checks |
| CLI binary | ~251 KB static ELF (`build/hisab` — version smoke test only) |
| Toolchain | Cyrius 6.6.4 |
| Dependencies | 1 (sakshi 2.5.2); no third-party, no FFI/libc |
| Security | No FFI, no libc, no third-party code — one first-party dependency. Every fallible entry point returns `Result<T, E>`; 249 `#must_use` annotations in `src/`, gated in CI because it is a *compiler* diagnostic a lint grep cannot see. The allocation and abort surfaces were swept in 2.12.0 and the guards are derived, not chosen. **The public API is declared, not implied** (3.1.0): every non-underscore top-level declaration carries `public`, and `scripts/check-public-surface.sh` flips every module `private` in a scratch copy on each CI run to prove the surface complete and exact — with calls, never `&name` (a private fn was reachable through address-of on cycc 6.6.2/6.6.3; fixed in 6.6.4, and the gate keeps call probes because consumers build under their own pins). **0 open filings** in [docs/development/issues/](docs/development/issues/) (31 archived); **0** hisab-filed toolchain defects open upstream in [cyrius](https://github.com/MacCracken/cyrius) — all four 2026-09-13 filings were repaired in 6.6.4 and are archived there. Dated reports in [docs/audit/](docs/audit/) — the largest is the 2026-08-11 P(-1) sweep (52 reproduced, 21 confirmed, 2 refuted, **28 reproduced but never verified and recorded as such**). |

## License

GPL-3.0-only — see [LICENSE](LICENSE).
