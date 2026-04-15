# Architecture Overview

> Cyrius port of hisab v1.4.0 — 27 lib files, 11,769 lines

## Module Map

```
hisab (Cyrius)
├── Foundation types
│   ├── error.cyr          — Error codes (ERR_*), epsilon constants
│   ├── f64_util.cyr       — f64_tan, f64_fmod, f64_copysign, f64_approx_eq
│   ├── vec2.cyr           — HVec2: 2D f64 vector (heap-allocated)
│   ├── vec3.cyr           — HVec3: 3D f64 vector with cross, reflect, min/max
│   ├── vec4.cyr           — HVec4: 4D f64 vector, Vec3 conversion
│   ├── quat.cyr           — HQuat: quaternion with slerp, rotation, axis-angle
│   ├── mat3.cyr           — 3x3 matrix: mul, inverse, determinant, from_quat
│   └── mat4.cyr           — 4x4 matrix: inverse, SRT, projections, look-at
│
├── Transforms
│   ├── transforms.cyr     — Transform2D/3D, compose, Euler, screen projection, lerp
│   └── color.cyr          — sRGB/HSV/HSL, Porter-Duff (8 ops), tone mapping, SH L2, EV
│
├── Geometry
│   ├── geo.cyr            — 9 primitives, 6 ray tests, closest-point queries
│   └── geo_advanced.cyr   — GJK/EPA 3D, BVH, SDF+CSG, swept AABB, TOI, CGA 5D
│
├── Calculus
│   ├── calc.cyr           — Derivative, Simpson/Gauss-Legendre, Bezier, easing, Perlin 2D
│   └── calc_ext.cyr       — Gradient/Jacobian/Hessian, adaptive Simpson, B-spline, NURBS,
│                             Hermite TCB, monotone cubic, 3D Perlin noise
│
├── Numerical
│   ├── num.cyr            — Newton/bisection, FFT/IFFT, RK4, PCG32, primes, sieve, Kahan sum
│   ├── ode.cyr            — DOPRI45, backward Euler, BDF-2..5, SDE, symplectic, Verlet, Yoshida
│   ├── optimize.cyr       — Gradient descent, CG (Polak-Ribiere+), BFGS, L-BFGS, LM
│   ├── linalg_ext.cyr     — CSR sparse, GMRES, BiCGSTAB, PGS, SVD, eigen, Lyapunov, inertia
│   └── num_ext.cyr        — Extended GCD, totient, Mobius, factorize, CRT, DST/DCT, 2D-FFT,
│                             Halton/Sobol, tridiagonal solver
│
├── Physics
│   ├── complex.cyr        — Complex numbers + matrices, Pauli, Dirac gamma, matrix exp
│   ├── lie.cyr            — U(1), SU(2), SU(3) Gell-Mann, SO(3,1) Lorentz
│   └── diffgeo.cyr        — Christoffel→Einstein, geodesic RK4, Killing, exterior algebra
│
├── Symbolic
│   ├── symbolic.cyr       — Expr tree, evaluate, differentiate, simplify, to_str
│   └── symbolic_ext.cyr   — Symbolic integration, LaTeX rendering, pattern matching + rewrite
│
└── Other
    ├── autodiff.cyr       — Dual numbers (forward-mode AD)
    ├── interval.cyr       — Interval arithmetic
    └── tensor.cyr         — N-D dense tensor, Kronecker/Minkowski/Levi-Civita, contraction
```

## Dependencies

| Dependency | Source | Purpose |
|-----------|--------|---------|
| sakshi | External (git) | Structured logging |
| alloc, string, fmt, vec, str | Cyrius stdlib | Core data structures |
| math, matrix, linalg | Cyrius stdlib | f64 ops, dense matrix, decompositions |
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
│   BVH   │  (bvh_build, bvh_query_ray/aabb)
└────┬────┘
     ▼
Candidate pairs
     │
     ▼
┌──────────┐
│ GJK/EPA  │  (gjk_intersect_3d, gjk_epa_3d)
└────┬─────┘
     ▼
Penetration { normal, depth }
     │
     ▼
┌─────────────────┐
│ solve_pgs()     │  (projected Gauss-Seidel)
└─────────────────┘
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
