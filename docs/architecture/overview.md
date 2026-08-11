# Architecture Overview

> Equation reference: see [`math.md`](math.md) (CGA operators + a catalogue index
> of the library's other formula families).
>
> hisab v2.10.2 — 35 math modules in `src/`, 22,707 lines of Cyrius (`lib/` is
> vendored stdlib + first-party deps only). Compiled by cycc 6.5.18.

## Module Map

```
hisab (Cyrius)
├── Foundation types
│   ├── error.cyr          — Error codes (HSB_ERR_*), epsilon constants
│   ├── f64_util.cyr       — f64_tan/fmod/copysign/approx_eq (f64_le/f64_ge come from stdlib `math`)
│   ├── vec2.cyr           — HVec2: 2D f64 vector (heap-allocated)
│   ├── vec3.cyr           — HVec3: 3D f64 vector with cross, reflect, min/max (SIMD f64v)
│   ├── vec4.cyr           — HVec4: 4D f64 vector, Vec3 conversion (SIMD f64v)
│   ├── quat.cyr           — HQuat: quaternion with slerp, rotation, axis-angle
│   ├── mat3.cyr           — 3x3 matrix: mul, inverse, determinant, from_quat
│   └── mat4.cyr           — 4x4 matrix: inverse, SRT, projections, look-at
│
├── Transforms
│   ├── transforms.cyr     — Transform2D/3D, compose, Euler, screen projection, lerp
│   └── color.cyr          — sRGB/HSV/HSL/Oklab, Porter-Duff (8 ops), tone mapping, SH L2, EV
│
├── Geometry
│   ├── geo.cyr            — 9 primitives, 6 ray tests, closest-point queries
│   ├── geo_advanced.cyr   — GJK/EPA 3D, SDF+CSG, swept AABB, TOI, CGA 5D, BVH
│   └── spatial.cyr        — k-d tree, octree, quadtree, spatial hash (BVH lives in geo_advanced)
│
├── Collision
│   ├── collision_core.cyr — MPR/XenoCollide narrowphase, sequential-impulse solver,
│   │                         convex hull 2D (monotone chain), triangulation (ear clipping)
│   └── collision_mesh.cyr — Delaunay (Bowyer-Watson), half-edge mesh, island detection (union-find)
│
├── Calculus
│   ├── calc.cyr           — Derivative, Simpson/Gauss-Legendre, Bezier, easing, Perlin 2D
│   ├── calc_ext.cyr       — Gradient/Jacobian/Hessian, adaptive Simpson, B-spline, NURBS,
│   │                         Hermite TCB, monotone cubic, 3D Perlin noise
│   └── noise_simplex.cyr  — Simplex noise (2D/3D)
│
├── Numerical
│   ├── num.cyr            — Newton/bisection, FFT/IFFT, RK4, PCG32, primes, sieve, Kahan sum
│   ├── ode.cyr            — DOPRI45, backward Euler, BDF-2..5, SDE, symplectic, Verlet, Yoshida
│   ├── optimize.cyr       — Gradient descent, CG (Polak-Ribiere+), BFGS, L-BFGS, LM
│   ├── linalg_ext.cyr     — CSR sparse, GMRES, BiCGSTAB, PGS/LCP, SVD, eigen, Lyapunov, inertia
│   ├── linalg_precision.cyr — Compensated / high-precision linear algebra
│   └── num_ext.cyr        — Extended GCD, totient, Mobius, factorize, CRT, DST/DCT, 2D-FFT,
│                             Halton/Sobol, tridiagonal solver
│
├── Physics
│   ├── complex.cyr        — Complex numbers + matrices, Pauli, Dirac gamma, matrix exp
│   ├── lie.cyr            — U(1), SU(2), SU(3) Gell-Mann, SO(3,1) Lorentz
│   ├── lie_ext.cyr        — SE(3)/SO(3), adjoint, exp/log maps, BCH
│   └── diffgeo.cyr        — Christoffel→Einstein, geodesic RK4, Killing, exterior algebra
│
├── Symbolic
│   ├── symbolic.cyr       — Expr tree, evaluate, differentiate, simplify, to_str
│   └── symbolic_ext.cyr   — Symbolic integration, LaTeX rendering, pattern matching + rewrite
│
└── Other
    ├── autodiff.cyr       — Dual numbers (forward-mode AD)
    ├── interval.cyr       — Interval arithmetic
    ├── tensor.cyr         — N-D dense tensor, Kronecker/Minkowski/Levi-Civita
    └── einsum.cyr         — Einstein-summation contraction (bounded reused arena)
```

## Dependencies

| Dependency | Source | Purpose |
|-----------|--------|---------|
| sakshi | External (git) | Structured logging |
| alloc, string, fmt, vec, str | Cyrius stdlib | Core data structures |
| math, ganita | Cyrius stdlib | `math`: inclusive cmp, clamp/lerp/min/max/sign, polyfills. `ganita` (6.2.x umbrella): transcendentals + dense matrix + decompositions (subsumes the former `matrix`/`linalg`) |
| tagged, fnptr, callback | Cyrius stdlib | Option/Result types, function pointers, closure-like callback patterns |
| syscalls, io, args | Cyrius stdlib | System interface |
| assert, bench | Cyrius stdlib | Testing, benchmarking |

## Module dependency manifest

Cyrius modules in `src/` carry **no `include` lines** — they are self-contained text that the
bundler concatenates. That makes à-la-carte consumption possible but leaves the required set
implicit, which is why the README's à-la-carte example shipped broken until 2.7.0-H (it named
`lib/` paths and omitted two modules the example needed without ever mentioning them).

This table is **derived and verified, not hand-maintained**: every symbol referenced in code (with
comments stripped) is resolved to its defining module, closed transitively, and then each row is
confirmed by actually compiling that module against exactly its listed set — **0 undefined symbols
for all 34**, with a negative control (`geo` without `quat`) correctly reporting 1.

The sets are minimal in the sense that they contain no module that is not reached; they are the
transitive closure, so they are what you must include, not merely what the module names directly.

Consumers pulling the whole `dist/hisab.cyr` bundle can ignore this table entirely.

The stdlib half of the dependency set stopped being implicit in 2.9.2. `dist/hisab.deps` — the
sidecar `cyrius distlib` writes beside the bundle, which a consumer's `cyrius deps` reads to
auto-resolve stdlib leaves — is now **tracked** rather than gitignored, and names all 15 leaves the
fold needs in scope: `syscalls`, `io`, `string`, `alloc`, `str`, `fmt`, `vec`, `args`, `assert`,
`math`, `ganita`, `tagged`, `fnptr`, `bench`, `callback`. It was worth ignoring until cyrius 6.5.10
for the same reason this section exists: the sidecar was built by scanning bundled sources for
literal `include "lib/X.cyr"` lines, and hisab's 34 `[lib]` modules have none, so it reported 2.
6.5.10 unions in the declared `[deps] stdlib`, which is what takes it from 2 to 15.

| Module | Also include |
|---|---|
| `error` | *(standalone)* |
| `interval` | *(standalone)* |
| `symbolic` | *(standalone)* |
| `tensor` | *(standalone)* |
| `autodiff` | `error` |
| `complex` | `error` |
| `diffgeo` | `error` |
| `einsum` | `tensor` |
| `f64_util` | `error` |
| `num` | `error` |
| `ode` | `error` |
| `optimize` | `error` |
| `symbolic_ext` | `symbolic` |
| `vec2` | `error` |
| `vec3` | `error` |
| `collision_core` | `error` `vec3` |
| `linalg_ext` | `complex` `error` |
| `linalg_precision` | `complex` `error` |
| `mat3` | `error` `vec3` |
| `num_ext` | `error` `num` |
| `quat` | `error` `vec3` |
| `vec4` | `error` `vec3` |
| `calc` | `error` `vec2` `vec3` |
| `color` | `error` `vec3` `vec4` |
| `geo` | `error` `quat` `vec3` |
| `calc_ext` | `calc` `error` `vec2` `vec3` |
| `collision_mesh` | `collision_core` `error` `vec2` `vec3` |
| `geo_advanced` | `error` `geo` `quat` `vec3` |
| `mat4` | `error` `f64_util` `vec3` `vec4` |
| `noise_simplex` | `calc` `error` `vec2` `vec3` |
| `spatial` | `error` `geo` `quat` `vec3` |
| `lie` | `complex` `error` `f64_util` `mat4` `quat` `vec3` `vec4` |
| `transforms` | `error` `f64_util` `mat4` `quat` `vec2` `vec3` `vec4` |
| `lie_ext` | `complex` `error` `f64_util` `lie` `mat3` `mat4` `quat` `vec3` `vec4` |

**Shape of the graph.** Four modules are fully standalone (`error`, `interval`, `symbolic`,
`tensor`); the deepest is `lie_ext` at 9. There are **no cycles**. Six module pairs reach into
another module's `_`-prefixed internals — `calc_ext`→`calc`, `collision_mesh`→`collision_core`,
`lie_ext`→`lie`, `noise_simplex`→`calc`, `num_ext`→`num`, `symbolic_ext`→`symbolic` — and every one
of those runs from a derived module to its own base, which is the layering the module *names*
already declare. That is a documentation gap rather than tangled coupling, and this table is the
documentation.

## Design Principles

- **Pure math** — no I/O in library code
- **f64 everywhere** — all math is IEEE 754 double precision (1e-12 tolerance)
- **Heap-allocated types** — multi-field structs via `alloc()` + `#derive(accessors)`
- **Error codes** — functions return `HSB_ERR_NONE` (0) on success, negative `HSB_ERR_*` on failure (namespaced from bare `ERR_*` in 2.6.8 to avoid consumer collisions)
- **Out-parameters** — results written via `store64(out, value)` pointers
- **No abort** — library code never calls `syscall(60, ...)` / `sys_exit_group` (warnings only)
- **Overflow guards** — allocation sizes checked against caps for user-controlled dimensions
- **Function pointers** — callbacks via `fncall1`/`fncall2` from fnptr.cyr

## Data Flow — Collision Pipeline

```
Scene objects
      │
      ▼
┌─────────┐
│  BVH /  │  (bvh_build, bvh_query_ray/aabb; k-d tree, octree, spatial hash)
│ spatial │
└────┬────┘
     ▼
Candidate pairs
     │
     ▼
┌────────────────────┐
│  Narrowphase:      │  GJK/EPA  (gjk_intersect_3d, gjk_epa_3d)
│  GJK/EPA  or  MPR  │  MPR      (mpr_intersect, mpr_penetration — XenoCollide)
└────┬───────────────┘
     ▼
Penetration { normal, depth }  →  contact_new()
     │
     ▼
┌──────────────────────────────┐
│  Constraint solve:           │  sequential_impulse()  (accumulate-clamp impulses)
│  sequential_impulse / PGS    │  solve_pgs()           (projected Gauss-Seidel LCP)
└──────────────────────────────┘
     │
     ▼
detect_islands()  (union-find — partitions the contact graph)
```

## Data Flow — ODE Solving

```
dy/dt = f(t, y)
      │
      ├── Explicit ─── num_rk4, ode_dopri45
      ├── Implicit ─── ode_backward_euler, ode_bdf2..5
      ├── Symplectic ─ ode_symplectic_euler, ode_verlet, ode_yoshida4
      └── Stochastic ─ ode_euler_maruyama, ode_milstein
```

## Consumers

| Project | What it uses |
|---------|-------------|
| **impetus** | Transforms, GJK/EPA, PGS solver, inertia tensors, BVH |
| **kiran** | Projections, frustum, BVH, ray tests, easing |
| **joshua** | DOPRI45, BDF, symplectic, optimization, PCG32 |
| **aethersafha** | Projections, compositing, tone mapping, color |
| **abaco** | Symbolic algebra, interval arithmetic |
| **svara** | Complex, FFT, easing |
| **hisab-mimamsa** | Tensors, Lie groups, diffgeo, complex LA, CGA |
| **kana** | Tensors, Lie groups, complex LA, spinors |
