# Architecture Overview

> Equation reference: see [`math.md`](math.md) (CGA operators + a catalogue index
> of the library's other formula families).
>
> hisab v2.6.9 — 34 math modules in `src/`, ~16,900 lines of Cyrius (`lib/` is
> vendored stdlib + first-party deps only). Compiled by cycc 6.4.66.

## Module Map

```
hisab (Cyrius)
├── Foundation types
│   ├── error.cyr          — Error codes (ERR_*), epsilon constants
│   ├── f64_util.cyr       — f64_tan/fmod/copysign/approx_eq, f64_le/f64_ge (non-strict cmp)
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
│   ├── geo_advanced.cyr   — GJK/EPA 3D, SDF+CSG, swept AABB, TOI, CGA 5D
│   └── spatial.cyr        — BVH, k-d tree, octree, quadtree, spatial hash
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
| tagged, fnptr | Cyrius stdlib | Option/Result types, function pointers |
| syscalls, io, args | Cyrius stdlib | System interface |
| assert, bench | Cyrius stdlib | Testing, benchmarking |

## Design Principles

- **Pure math** — no I/O in library code
- **f64 everywhere** — all math is IEEE 754 double precision (1e-12 tolerance)
- **Heap-allocated types** — multi-field structs via `alloc()` + `#derive(accessors)`
- **Error codes** — functions return `ERR_NONE` (0) on success, negative `ERR_*` on failure
- **Out-parameters** — results written via `store64(out, value)` pointers
- **No abort** — library code never calls `syscall(60, ...)` (warnings only)
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
