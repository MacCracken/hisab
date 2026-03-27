# Architecture Overview

## Module Map

```
hisab
├── transforms          — Transform2D/3D, projections, slerp, lerp, handedness  [default]
│   ├── core            — Transform2D/3D, projections (ortho, perspective, reverse-Z),
│   │                     lerp, inverse_lerp, remap, flip_handedness_z, compose
│   ├── quat            — quat_from_euler, quat_to_euler, quat_look_at, look_at_rh
│   ├── screen          — world_to_screen, screen_to_world_ray
│   ├── color           — sRGB, HSV, HSL, Oklab, premultiplied alpha, Porter-Duff (10 ops),
│   │                     saturation/hue matrices, tone mapping (Reinhard, ACES), depth linearization
│   ├── dualquat        — DualQuat rigid body transforms, blend skinning
│   ├── decompose       — CSS mat4 decompose/recompose (DecomposedTransform)
│   └── sh              — spherical harmonics L0–L2 (eval, project, evaluate)
│
├── geo                 — Primitives, intersections, spatial structures, collision  [default]
│   ├── primitives      — Ray, Plane, Aabb, Sphere, OBB, Capsule, Frustum, Rect
│   ├── intersection    — Triangle, Line, Segment, ray-*, plane-plane, AABB-AABB, sphere-sphere
│   ├── closest         — closest_point_on_{ray,plane,sphere,aabb,triangle},
│   │                     barycentric_coords, segment_segment_closest, compute_tangent
│   ├── spatial         — Bvh, KdTree, Quadtree, Octree, SpatialHash
│   ├── collision       — convex_hull_2d, GJK/EPA 2D+3D, MPR/XenoCollide,
│   │                     swept_aabb, time_of_impact, sequential_impulse (with friction),
│   │                     sweep_and_prune broadphase
│   ├── sdf             — sdf_sphere/box/capsule, CSG (union/intersection/subtraction/smooth),
│   │                     triangulate_polygon (ear-clipping)
│   ├── decompose       — convex_decompose (PCA splitting), TriMesh
│   └── delaunay        — delaunay_2d (Bowyer-Watson), voronoi_2d (dual)
│
├── calc                — Differentiation, integration, curves, splines, easing  [default]
│   ├── core            — derivative, integral_{trapezoidal,simpson}, bezier 2D/3D
│   ├── integration     — gauss_legendre, adaptive_simpson, monte_carlo
│   ├── splines         — catmull_rom, bspline_eval, nurbs_eval, hermite_tcb,
│   │                     monotone_cubic, de_casteljau_split, arc-length parameterization
│   ├── easing          — ease_{in,out,in_out}{,_cubic,_smooth}, spring_step, cubic_bezier_ease
│   ├── noise           — perlin_2d, perlin_3d, fbm_2d
│   └── multivar        — partial_derivative, gradient, jacobian, hessian
│
├── num                 — Root finding, decompositions, spectral, ODE, optimization  [default]
│   ├── roots           — newton_raphson, bisection, gaussian_elimination
│   ├── linalg          — lu, cholesky, qr (in-place variants), matrix ops, rank, condition
│   ├── eigen           — eigen_symmetric (Jacobi), EigenDecomposition
│   ├── svd             — svd (one-sided Jacobi), truncated_svd, pseudo_inverse
│   ├── complex         — Complex arithmetic, serde
│   ├── fft             — fft, ifft, fft_2d, ifft_2d, dst, idst, dct, idct
│   ├── ode             — rk4, dopri45, backward_euler, bdf2, bdf(3-5),
│   │                     euler_maruyama, milstein (SDE),
│   │                     symplectic_euler, verlet, leapfrog, yoshida4
│   ├── optimize        — gradient_descent, conjugate_gradient, bfgs, levenberg_marquardt
│   ├── solvers         — projected_gauss_seidel (+SOR), gmres, bicgstab
│   ├── sparse          — CsrMatrix (spmv, spmvt, add, transpose, get),
│   │                     sparse_cholesky_solve, sparse_lu_solve
│   ├── rng             — Pcg32, halton, halton_2d, sobol
│   ├── summation       — kahan_sum, neumaier_sum
│   ├── inertia         — inertia_sphere, inertia_box, inertia_mesh
│   └── stability       — lyapunov_max
│
├── autodiff            — Automatic differentiation                              [feature: autodiff]
│   ├── forward         — Dual (val/deriv), var(), constant(), transcendentals
│   └── reverse         — Tape, Var, tape.backward(), reverse_gradient()
│
├── interval            — Interval arithmetic                                    [feature: interval]
│                         Interval type, arithmetic ops, contains, overlaps, hull, sqr, sqrt
│
├── symbolic            — Symbolic algebra                                       [feature: symbolic]
│                         Expr enum (Const/Var/Add/Mul/Pow/Neg/Sin/Cos/Exp/Ln),
│                         evaluate, differentiate, simplify, substitute
│
├── tensor              — N-dimensional tensor                                   [feature: tensor]
│                         Tensor (zeros, ones, get/set, reshape, add/sub/scale, matmul, transpose)
│
├── parallel            — Parallel batch operations                              [feature: parallel]
│                         par_transform_points, par_ray_aabb_batch, par_matrix_vector_multiply
│
├── ai                  — AGNOS AI client                                        [feature: ai]
│                         DaimonClient (register, heartbeat, hoosh_query)
│
├── logging             — Structured logging                                     [feature: logging]
│                         init_logging() via tracing-subscriber + HISAB_LOG env
│
└── error               — HisabError, DaimonError
```

## Feature Flags

| Flag | Dependencies | Description |
|------|-------------|-------------|
| `transforms` | (default) | Core transforms, projections, color, interpolation |
| `geo` | (default) | All geometry: primitives, spatial, collision, Delaunay/Voronoi |
| `calc` | (default) | Calculus, curves, splines (including NURBS), easing, noise |
| `num` | (default) | Numerical methods, decompositions, FFT, ODE/SDE, optimization, sparse |
| `autodiff` | — | Forward-mode (dual numbers) + reverse-mode (tape) AD |
| `interval` | — | Interval arithmetic with bound propagation |
| `symbolic` | — | Expression tree algebra (evaluate, differentiate, simplify) |
| `tensor` | — | N-dimensional dense tensor operations |
| `parallel` | `rayon` | Parallel batch operations for transform/intersection/matvec |
| `ai` | `reqwest`, `tokio`, `serde_json` | AGNOS daimon/hoosh AI client |
| `logging` | `tracing-subscriber` | Structured logging via `HISAB_LOG` env |
| `full` | all above | Everything |

All default features are pure computation — no I/O, no async, no network.

## Design Principles

- **Pure math** — no I/O in default features; AI client is opt-in
- **Zero unsafe** — no `unsafe` blocks anywhere
- **Thread-safe** — all public types are `Send + Sync` (compile-time verified)
- **Built on glam** — leverages glam's SIMD-optimized Vec3/Mat4/Quat
- **Feature-gated** — heavy deps (reqwest, tokio, rayon) only compiled when needed
- **`#[non_exhaustive]`** — on all public enums for forward compatibility
- **`#[must_use]`** — on all pure functions
- **`#[inline]`** — on hot-path functions (intersection tests, transforms, easing)
- **`write!` over `format!`** — avoid temporary allocations
- **Result over panic** — no `unwrap()`, `panic!()`, or `assert!()` in library code

## Data Flow — Collision Pipeline

```
Scene objects
      │
      ▼
┌─────────────┐     ┌──────────────┐     ┌─────────────┐
│  SpatialHash │ or  │     BVH      │ or  │   Octree    │
│  (broadphase)│     │  (broadphase)│     │ (broadphase)│
└──────┬──────┘     └──────┬───────┘     └──────┬──────┘
       │                   │                    │
       └───────────────────┼────────────────────┘
                           ▼
                  Candidate pairs (indices)
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
     ┌────────────┐ ┌────────────┐ ┌────────────┐
     │ gjk_*()    │ │  mpr_*()   │ │ sphere/aabb│
     │ (GJK/EPA)  │ │  (MPR)     │ │ (fast path)│
     └─────┬──────┘ └─────┬──────┘ └─────┬──────┘
           │               │              │
           └───────────────┼──────────────┘
                           ▼
              Penetration { normal, depth }
                           │
                           ▼
              ┌──────────────────────┐
              │ sequential_impulse() │
              │ (normal + friction)  │
              └──────────────────────┘
```

## Data Flow — ODE Solving

```
dy/dt = f(t, y)
      │
      ├── Explicit ──────────────────────────┐
      │   rk4()         — 4th-order, fixed   │
      │   dopri45()     — 4(5) adaptive      │
      │                                      │
      ├── Implicit (stiff) ──────────────────┤
      │   backward_euler() — 1st-order       │
      │   bdf2()           — 2nd-order       │
      │   bdf(order=3..5)  — higher-order    │
      │                                      │
      ├── Symplectic (Hamiltonian) ──────────┤
      │   symplectic_euler() — 1st-order     │
      │   verlet()           — 2nd-order     │
      │   yoshida4()         — 4th-order     │
      │                                      │
      └── Stochastic ───────────────────────┘
          euler_maruyama() — strong order 0.5
          milstein()       — strong order 1.0
```

## Consumers

| Project | What it uses |
|---------|-------------|
| **impetus** | Transforms, spatial structures (BVH broadphase), GJK/EPA/MPR collision, sequential impulse, inertia tensors |
| **kiran** | Projections (incl. reverse-Z), frustum culling, camera transforms, OBB/Capsule rays, easing, Delaunay, tangent space |
| **joshua** | ODE solvers (RK4, DOPRI45, BDF), symplectic integrators (Yoshida), SDE, Lyapunov, deterministic replay (PCG32, monotone cubic) |
| **aethersafha** | Projection matrices, transform composition, Porter-Duff compositing, tone mapping, color spaces, depth linearization |
| **abaco** | Symbolic algebra (Expr), interval arithmetic for verified evaluation |
| **svara** | Complex, FFT, easing functions (vocal synthesis) |
| **prani** | Easing functions (creature vocal synthesis, via svara) |
