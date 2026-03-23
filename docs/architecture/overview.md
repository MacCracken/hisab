# Architecture Overview

## Module Map

```
ganit
├── transforms      — Transform2D/3D, projections, slerp, lerp, handedness
├── geo             — Primitives, intersections, spatial structures, collision
│   ├── primitives  — Ray, Plane, Aabb, Sphere, Triangle, Line, Segment, Rect
│   ├── intersect   — ray_plane, ray_sphere, ray_aabb, ray_triangle, overlaps
│   ├── closest     — closest_point_on_ray/plane/sphere/aabb
│   ├── frustum     — Frustum (6-plane extraction from VP matrix)
│   ├── spatial     — Bvh, KdTree, Quadtree, Octree, SpatialHash
│   └── collision   — convex_hull_2d, ConvexPolygon, GJK, EPA
├── calc            — Differentiation, integration, curves, splines, easing
│   ├── deriv       — derivative (central difference)
│   ├── integrate   — trapezoidal, Simpson, Gauss-Legendre (5-point composite)
│   ├── bezier      — quadratic/cubic 2D+3D, de Casteljau split
│   ├── spline      — catmull_rom, bspline_eval (de Boor)
│   ├── arc_length  — bezier_cubic_3d_arc_length, param_at_length
│   └── easing      — ease_in/out/in_out (quadratic, cubic, quintic)
├── num             — Root finding, decompositions, spectral, ODE
│   ├── roots       — newton_raphson, bisection
│   ├── solve       — gaussian_elimination
│   ├── decompose   — lu, cholesky, qr, least_squares_poly
│   ├── spectral    — eigenvalue_power, Complex, fft, ifft
│   └── ode         — rk4, rk4_trajectory
├── ai              — DaimonClient, hoosh queries                [feature: ai]
├── logging         — GANIT_LOG tracing init                     [feature: logging]
└── error           — GanitError, DaimonError
```

## Feature Flags

| Flag | Dependencies | Description |
|------|-------------|-------------|
| `transforms` | (default) | Core transforms, projections, interpolation |
| `geo` | (default) | All geometry: primitives, spatial, collision |
| `calc` | (default) | Calculus, curves, splines, easing |
| `num` | (default) | Numerical methods, decompositions, FFT, ODE |
| `ai` | `reqwest`, `tokio`, `serde_json` | AGNOS daimon/hoosh AI client |
| `logging` | `tracing-subscriber` | Structured logging via `GANIT_LOG` env |
| `full` | all above | Everything |

All default features are pure computation — no I/O, no async, no network.

## Design Principles

- **Pure math** — no I/O in default features; AI client is opt-in
- **Zero unsafe** — no `unsafe` blocks anywhere
- **Thread-safe** — all public types are `Send + Sync` (compile-time verified)
- **Built on glam** — leverages glam's SIMD-optimized Vec3/Mat4/Quat
- **Feature-gated** — heavy deps (reqwest, tokio) only compiled when needed
- **`#[non_exhaustive]`** — on all public enums for forward compatibility
- **`#[inline]`** — on all hot-path functions (intersection tests, transforms, easing)

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
                           ▼
                ┌──────────────────┐
                │  gjk_intersect() │  ← narrowphase
                └────────┬─────────┘
                         │ (if overlapping)
                         ▼
                ┌──────────────────┐
                │ epa_penetration()│  ← penetration depth
                └────────┬─────────┘
                         │
                         ▼
                   Penetration { normal, depth }
```

## Data Flow — ODE Solving

```
dy/dt = f(t, y)
      │
      ▼
┌─────────────────┐
│  rk4(f, t0, y0, │
│     t_end, n)    │
└────────┬────────┘
         │  (n steps, reuses scratch buffer)
         ▼
    y(t_end): Vec<f64>

┌──────────────────────┐
│  rk4_trajectory(...)  │  ← same + stores all intermediate states
└────────┬─────────────┘
         ▼
   Vec<(t, Vec<f64>)>
```

## Consumers

| Project | What it uses |
|---------|-------------|
| **impetus** | Transforms, spatial structures (BVH broadphase), GJK/EPA collision |
| **kiran** | Projections, frustum culling, camera transforms, easing |
| **joshua** | RK4 ODE solver, deterministic simulation math |
| **aethersafha** | Projection matrices, transform composition |
| **abaco** | Future: ganit num for expression-level solve commands |
