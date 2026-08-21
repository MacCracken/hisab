# Hisab

> **Hisab** (Arabic: حساب — calculation) — higher mathematics library for AGNOS

Higher math for the AGNOS ecosystem — linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, and more. Written in [Cyrius](https://github.com/MacCracken/cyrius), ported from Rust.

Used by [impetus](https://github.com/MacCracken/impetus) (physics), [kiran](https://github.com/MacCracken/kiran) (engine), [joshua](https://github.com/MacCracken/joshua) (simulation), [aethersafha](https://github.com/MacCracken/aethersafha) (compositor), [svara](https://github.com/MacCracken/svara) (vocal synthesis), [hisab-mimamsa](https://github.com/MacCracken/hisab-mimamsa) (theoretical physics), and [kana](https://github.com/MacCracken/kana) (quantum science).

For expression evaluation and unit conversion, see [abaco](https://github.com/MacCracken/abaco).

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
cyrius      = "6.5.33"

[deps]
# `ganita` (Cyrius 6.2.x) provides the transcendentals (acos/asin/atan2/pow/
# sinh/cosh/tanh + inverses) AND subsumes `matrix`/`linalg` — do not list those
# alongside it or you get duplicate-definition collisions. `math` stays for the
# inclusive comparisons, clamp/lerp/min/max/sign and the polyfills.
stdlib = ["string", "fmt", "alloc", "vec", "str", "math", "ganita", "tagged", "fnptr"]

[deps.hisab]
git     = "https://github.com/MacCracken/hisab.git"
tag     = "2.11.2"
modules = ["dist/hisab.cyr"]   # ~875 KB self-contained bundle (all 35 modules)
# `dist/hisab.deps` is tracked as of 2.9.2 -- `cyrius deps` reads that sidecar and
# pulls in hisab's own 15 stdlib leaves, so the `stdlib` list above only has to
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
fn x_squared(x) { return f64_mul(x, x); }
var result = alloc(8);
calc_integral_simpson(&x_squared, 0, F64_ONE, 100, result);
# load64(result) ≈ 0.3333

# Root finding (Newton-Raphson for sqrt(2))
fn f(x) { return f64_sub(f64_mul(x, x), F64_TWO); }
fn df(x) { return f64_mul(F64_TWO, x); }
var root = alloc(8);
num_newton(&f, &df, F64_ONE, EPSILON_F64, 100, root);
# load64(root) ≈ 1.41421356...
```

## Building

```sh
cyrius build src/main.cyr build/hisab
cyrius test tests/hisab.tcyr        # 416 smoke tests
cyrius test tests/foundation.tcyr   # 351 foundation tests
cyrius test tests/modules.tcyr      # 1775 module tests
cyrius test tests/edge_cases.tcyr   # 233 edge case tests
cyrius test tests/abuse.tcyr        # 739 abuse tests (negative indices, zero/huge
                                    #   dimensions, non-conformable operands, canaries)
cyrius bench tests/hisab.bcyr       # 72 benchmarks
```

## Architecture

See [docs/architecture/overview.md](docs/architecture/overview.md) for the full module map, data flow, and design principles, and [docs/architecture/math.md](docs/architecture/math.md) for the equation reference (CGA operators, plus a catalogue index of the other formula families).

## Stats

| Metric | Value |
|--------|-------|
| Version | 2.11.2 |
| Library | 35 modules, ~23,182 lines of Cyrius |
| Tests | 3526 assertions across 5 suites |
| Benchmarks | 72 operations |
| Fuzz targets | 5 with invariant checks |
| CLI binary | ~219 KB static ELF (`build/hisab` — version smoke test only) |
| Toolchain | Cyrius 6.5.33 |
| Dependencies | 1 (sakshi 2.4.11); no third-party, no FFI/libc |
| Security | P(-1) audit (2026-04-15) + hardening pass (2026-05-29); [2026-08-03](docs/audit/2026-08-03.md) sweep (70 findings, 2 critical) discharged in 2.6.12–2.6.15; [2026-08-04](docs/audit/2026-08-04.md) re-audit (42 confirmed, 0 critical) discharged in 2.7.0; [2026-08-04 v2.8.0 full sweep](docs/audit/2026-08-04-v2.8.0-full.md) (42 confirmed, 4 critical) **42 FIXED / 0 OPEN** as of 2.9.1; `tests/abuse.tcyr` (2.9.0) found 11 defects on public entry points, all fixed; [2026-08-11 v2.11.0 full sweep](docs/audit/2026-08-11-v2.11.0-full.md) (**52 reproduced, 21 confirmed, 2 refuted — and 28 reproduced but never verified, recorded as such**) with four repairs shipped in 2.11.1 and the rest scheduled on the roadmap. **3 open filings** in [docs/development/issues/](docs/development/issues/), 23 archived beside them: two are upstream cyrius items (one partially fixed in 6.5.17 and re-verified on 6.5.18, one deliberately never re-tested because its reproducer is destructive) and one is hisab's own — the EPA seed/certificate question, re-diagnosed in 2.9.3 |

## License

GPL-3.0-only — see [LICENSE](LICENSE).
