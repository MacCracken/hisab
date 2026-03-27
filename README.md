# Hisab

> **Hisab** (Arabic: حساب — calculation) — higher mathematics library for AGNOS

Higher math for the AGNOS ecosystem. Provides typed mathematical operations — linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, and more — built on [glam](https://crates.io/crates/glam).

Used by [impetus](https://github.com/MacCracken/impetus) (physics), [kiran](https://github.com/MacCracken/kiran) (engine), [joshua](https://github.com/MacCracken/joshua) (simulation), [aethersafha](https://github.com/MacCracken/aethersafha) (compositor), [svara](https://github.com/MacCracken/svara) (vocal synthesis), and [prani](https://github.com/MacCracken/prani) (creature vocal synthesis).

For expression evaluation and unit conversion, see [abaco](https://github.com/MacCracken/abaco).

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `transforms` | yes | 2D/3D affine transforms, projections, slerp/lerp, reverse-Z, glam re-exports, HSV/HSL/Oklab color, premultiplied alpha, Porter-Duff compositing, HDR tone mapping (Reinhard, ACES), depth linearization, dual quaternions, CSS decompose, spherical harmonics |
| `geo` | yes | Primitives (Ray, Plane, AABB, Sphere, OBB, Capsule, Triangle), intersections, closest-point queries, barycentric coordinates, BVH, k-d tree, quadtree, octree, spatial hash, 2D+3D GJK/EPA, MPR/XenoCollide, SAP broadphase, CCD/TOI, sequential impulse with friction, SDFs + CSG, convex decomposition, Delaunay triangulation, Voronoi diagrams, tangent space |
| `calc` | yes | Differentiation, integration (trapezoidal, Simpson, Gauss-Legendre, adaptive, Monte Carlo), Bezier 2D/3D, Catmull-Rom, B-splines, NURBS, Hermite TCB, monotone cubic (Fritsch-Carlson), easing, spring dynamics, gradient/Jacobian/Hessian, Perlin/fBm noise |
| `num` | yes | Root finding (Newton-Raphson, bisection), LU/Cholesky/QR/SVD, FFT/DST/DCT/2D-FFT, optimization (GD, CG, BFGS, LM), ODE (RK4, DOPRI45, backward Euler, BDF-2/3/4/5), SDE (Euler-Maruyama, Milstein), symplectic integrators (Euler, Verlet, leapfrog, Yoshida 4th-order), sparse CSR (spmv, spmvt, Cholesky, LU), GMRES, PGS+SOR, BiCGSTAB, eigendecomposition, inertia tensors, Lyapunov exponents, Halton/Sobol sequences, compensated summation (Kahan, Neumaier), PCG32 RNG |
| `autodiff` | no | Forward-mode (dual numbers) and reverse-mode (tape-based) automatic differentiation |
| `interval` | no | Interval arithmetic for verified numerics |
| `symbolic` | no | Symbolic expression tree with evaluation, differentiation, simplification, substitution |
| `tensor` | no | N-dimensional dense tensor with reshape, matmul, transpose |
| `parallel` | no | Rayon-powered parallel batch operations (transform, intersection, matvec) |
| `ai` | no | Daimon/hoosh AI client (network deps) |
| `logging` | no | Structured logging via `HISAB_LOG` env var |
| `full` | — | Enables all features |

## Quick Start

```toml
[dependencies]
hisab = "1.2"
```

```rust
use hisab::{Vec3, Quat, Transform3D, Ray, Sphere};
use hisab::geo::ray_sphere;
use hisab::calc::integral_simpson;
use hisab::num::newton_raphson;

// 3D transform with composition
let a = Transform3D::new(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE);
let b = Transform3D::new(Vec3::ZERO, Quat::from_rotation_y(0.5), Vec3::splat(2.0));
let combined = a.compose(&b);
let world_pos = combined.apply_to_point(Vec3::ZERO);

// Ray-sphere intersection
let ray = Ray::new(Vec3::ZERO, Vec3::Z).unwrap();
let sphere = Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0).unwrap();
let hit = ray_sphere(&ray, &sphere); // Some(4.0)

// Numerical integration
let area = integral_simpson(|x| x * x, 0.0, 1.0, 100).unwrap(); // ~0.3333

// Root finding
let sqrt2 = newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, 1.0, 1e-10, 100).unwrap();
```

## Architecture

See [docs/architecture/overview.md](docs/architecture/overview.md) for the full module map, data flow, and design principles.

## Building

```sh
cargo build
cargo test --all-features   # 786 tests
make bench                  # criterion benchmarks with history tracking
```

## License

GPL-3.0-only — see [LICENSE](LICENSE).
