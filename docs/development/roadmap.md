# Roadmap

> **Hisab** (Arabic: حساب — calculation, reckoning) — higher mathematics library for the AGNOS ecosystem.
> Basic expression evaluation and unit conversion lives in [abaco](https://github.com/MacCracken/abaco).

## Scope

Hisab owns **typed mathematical operations** — the programmatic math that engines, physics, rendering, and simulation need. It does NOT own:

- **Expression parsing, user-typed math** → abaco (eval engine)
- **Unit conversion** → abaco (unit registry)
- **Physics simulation** → impetus (wraps rapier, uses hisab types)
- **Game engine** → kiran (uses hisab for transforms/projections)

## Consumers

| Consumer | What it uses from hisab |
|----------|----------------------|
| **impetus** | Vectors, quaternions, transforms, spatial geometry, 3D GJK/EPA broadphase+narrowphase |
| **kiran** | Projections, transforms, frustum culling, camera math, OBB/Capsule ray tests |
| **joshua** | ODE solvers (RK4, DOPRI45), simulation math, deterministic replay |
| **aethersafha** | Projection matrices, transform composition/interpolation for compositor |
| **abaco** | Symbolic algebra (Expr), interval arithmetic for verified evaluation |

## Versioning

Post-1.0: standard semver.

---

## V1.0.0 — Stable Release

### Modules

| Module | Feature | Description |
|--------|---------|-------------|
| **transforms** | `transforms` (default) | Transform2D/3D, projections, lerp, slerp, glam re-exports |
| **geo** | `geo` (default) | Primitives, intersections, BVH, KdTree, Quadtree, Octree, SpatialHash, 2D+3D GJK/EPA, OBB, Capsule |
| **calc** | `calc` (default) | Differentiation, integration (Simpson, Gauss-Legendre, adaptive, Monte Carlo), Bezier, splines, easing, gradient/jacobian/hessian |
| **num** | `num` (default) | Root finding, LU/Cholesky/QR/SVD, FFT/DST/DCT, optimization (GD, CG, BFGS, L-BFGS, LM), ODE (RK4, DOPRI45), sparse matrices |
| **autodiff** | `autodiff` | Forward-mode automatic differentiation (dual numbers) |
| **interval** | `interval` | Interval arithmetic for verified numerics |
| **symbolic** | `symbolic` | Expression tree with evaluation, differentiation, simplification |
| **tensor** | `tensor` | N-dimensional dense tensor type |
| **parallel** | `parallel` | Rayon-powered parallel batch operations |
| **ai** | `ai` | Daimon/hoosh AI client |
| **logging** | `logging` | Structured logging via tracing-subscriber |

### Stats

- 617 tests (574 unit + 34 integration + 9 doc)
- 12 modules, 13 feature flags
- Zero clippy warnings, cargo audit clean, cargo deny clean
- Consumer smoke tests for impetus, kiran, joshua, aethersafha, abaco

---

## Post-1.0

| Item | Priority |
|------|----------|
| GPU compute kernels via wgpu (shared with ranga) | Medium |
| Complete doctests on all public functions | Low |
| L-BFGS memory-efficient history (VecDeque) | Low |

## Boundary with Abaco

| Feature | abaco | hisab |
|---------|-------|-------|
| `eval("sin(pi/4)")` | abaco parses and evaluates | — |
| `Vec3::cross(a, b)` | — | transforms |
| `ray.intersect(sphere)` | — | geo |
| `integral(f, 0, 1)` | — | calc |
| `newton_raphson(f, df, x0)` | — | num |
| `eval("solve x^2 - 2 = 0")` | abaco parses | num solves |

Hisab should never depend on abaco. Abaco may optionally depend on hisab (num, symbolic) for solver/algebra features.
