# Hisab

> **Hisab** (Arabic: حساب — calculation) — higher mathematics library for AGNOS

Higher math for the AGNOS ecosystem — linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, and more. Written in [Cyrius](https://github.com/MacCracken/cyrius), ported from Rust.

Used by [impetus](https://github.com/MacCracken/impetus) (physics), [kiran](https://github.com/MacCracken/kiran) (engine), [joshua](https://github.com/MacCracken/joshua) (simulation), [aethersafha](https://github.com/MacCracken/aethersafha) (compositor), [svara](https://github.com/MacCracken/svara) (vocal synthesis), [hisab-mimamsa](https://github.com/MacCracken/hisab-mimamsa) (theoretical physics), and [kana](https://github.com/MacCracken/kana) (quantum science).

For expression evaluation and unit conversion, see [abaco](https://github.com/MacCracken/abaco).

## Modules

| Module | Files | Description |
|--------|-------|-------------|
| **Foundation** | vec2, vec3, vec4, quat, mat3, mat4 | Vector/matrix/quaternion types (f64, heap-allocated) |
| **Transforms** | transforms, color | 2D/3D affine transforms, projections, slerp/lerp, Euler angles, sRGB/HSV/HSL/Oklab, Porter-Duff compositing (8 ops), tone mapping (Reinhard, ACES), SH L2, EV/exposure |
| **Geometry** | geo, geo_advanced | 9 primitives, 6 ray tests, closest-point queries, GJK/EPA 3D, BVH, SDF+CSG, TOI, conformal geometric algebra (5D CGA) |
| **Calculus** | calc, calc_ext | Differentiation, integration (Simpson, Gauss-Legendre, adaptive), Bezier/Catmull-Rom/B-spline/NURBS/Hermite TCB/monotone cubic, easing, Perlin 2D+3D, gradient/Jacobian/Hessian |
| **Numerical** | num, ode, optimize, linalg_ext, num_ext | Root finding, FFT/DST/DCT/2D-FFT, ODE (RK4, DOPRI45, BDF, symplectic, SDE), optimization (GD, CG, BFGS, L-BFGS, LM), sparse CSR, GMRES, PGS, SVD, eigendecomposition, number theory (primes, factorize, CRT, totient, Mobius), PCG32, Halton/Sobol, tridiagonal solver |
| **Complex** | complex | Complex numbers + matrices, Pauli/Dirac gamma matrices, matrix exponential |
| **Lie groups** | lie | U(1), SU(2), SU(3) Gell-Mann, SO(3,1) Lorentz, exponential maps |
| **Diff geometry** | diffgeo | Christoffel symbols, Riemann/Ricci/Einstein tensors, geodesic solver, Killing vectors, exterior algebra |
| **Symbolic** | symbolic, symbolic_ext | Expression tree, evaluate, differentiate, simplify, symbolic integration, LaTeX rendering, pattern matching + rewrite rules |
| **Autodiff** | autodiff | Forward-mode automatic differentiation (dual numbers) |
| **Interval** | interval | Interval arithmetic for verified numerics |
| **Tensor** | tensor | N-dimensional dense tensor, Kronecker delta, Minkowski metric, Levi-Civita |

## Quick Start

```toml
# cyrius.cyml
[package]
name        = "your-project"
version     = "${file:VERSION}"
language    = "cyrius"
cyrius      = "5.7.7"

[deps]
stdlib = ["string", "fmt", "alloc", "vec", "str", "math", "matrix", "linalg", "tagged", "fnptr"]

[deps.hisab]
git     = "https://github.com/MacCracken/hisab.git"
tag     = "2.2.1"
modules = ["dist/hisab.cyr"]
```

```cyrius
include "lib/f64_util.cyr"
include "lib/error.cyr"
include "lib/vec3.cyr"
include "lib/quat.cyr"
include "lib/mat4.cyr"
include "lib/transforms.cyr"
include "lib/geo.cyr"
include "lib/calc.cyr"
include "lib/num.cyr"

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
cyrius test tests/hisab.tcyr        # 116 smoke tests
cyrius test tests/foundation.tcyr   # 307 foundation tests
cyrius test tests/modules.tcyr      # 249 module tests
cyrius test tests/edge_cases.tcyr   # 149 edge case tests
cyrius bench tests/hisab.bcyr       # 22 benchmarks
```

## Architecture

See [docs/architecture/overview.md](docs/architecture/overview.md) for the full module map, data flow, and design principles.

## Stats

| Metric | Value |
|--------|-------|
| Version | 2.2.0 |
| Library | 33 files, 15,676 lines of Cyrius |
| Tests | 821 assertions across 4 test suites |
| Benchmarks | 22 operations |
| Fuzz targets | 5 with invariant checks |
| Binary | 511KB static ELF |
| Toolchain | Cyrius 4.10.3 |
| Dependencies | 1 (sakshi) |
| Security | P(-1) audited, 25 of 31 issues fixed |

## License

GPL-3.0-only — see [LICENSE](LICENSE).
