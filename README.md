# Hisab

> **Hisab** (Arabic: حساب — calculation) — higher mathematics library for AGNOS

Higher math for the AGNOS ecosystem. Provides typed mathematical operations — linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, and more — built on [glam](https://crates.io/crates/glam).

Used by [impetus](https://github.com/MacCracken/impetus) (physics), [kiran](https://github.com/MacCracken/kiran) (engine), [joshua](https://github.com/MacCracken/joshua) (simulation), and [aethersafha](https://github.com/MacCracken/aethersafha) (compositor).

For expression evaluation and unit conversion, see [abaco](https://github.com/MacCracken/abaco).

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `transforms` | yes | 2D/3D affine transforms, projections, slerp, lerp, glam re-exports |
| `geo` | yes | Primitives, intersections, BVH, k-d tree, quadtree, octree, spatial hash, 2D+3D GJK/EPA, OBB, Capsule |
| `calc` | yes | Differentiation, integration (Simpson, Gauss-Legendre, adaptive, Monte Carlo), Bezier, splines, easing, gradient/jacobian/hessian |
| `num` | yes | Root finding, LU/Cholesky/QR/SVD, FFT/DST/DCT, optimization (GD, CG, BFGS, L-BFGS, LM), ODE (RK4, DOPRI45), sparse matrices |
| `autodiff` | no | Forward-mode automatic differentiation (dual numbers) |
| `interval` | no | Interval arithmetic for verified numerics |
| `symbolic` | no | Symbolic expression tree with evaluation, differentiation, simplification |
| `tensor` | no | N-dimensional dense tensor type |
| `parallel` | no | Rayon-powered parallel batch operations |
| `ai` | no | Daimon/hoosh AI client (network deps) |
| `logging` | no | Structured logging via `HISAB_LOG` env var |
| `full` | — | Enables all features |

## Quick Start

```toml
[dependencies]
hisab = "1"
```

```rust
use hisab::{Vec3, Quat, Transform3D, Ray, Sphere};
use hisab::geo::ray_sphere;
use hisab::calc::integral_simpson;
use hisab::num::newton_raphson;

// 3D transform
let t = Transform3D::new(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE);
let world_pos = t.apply_to_point(Vec3::ZERO);

// Ray-sphere intersection
let ray = Ray::new(Vec3::ZERO, Vec3::Z).unwrap();
let sphere = Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0).unwrap();
let hit = ray_sphere(&ray, &sphere); // Some(4.0)

// Numerical integration
let area = integral_simpson(|x| x * x, 0.0, 1.0, 100).unwrap(); // ~0.3333

// Root finding
let sqrt2 = newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, 1.0, 1e-10, 100).unwrap();
```

## Building

```sh
cargo build
cargo test --all-features   # 617 tests
make bench                  # criterion benchmarks with history tracking
```

## License

GPL-3.0-only — see [LICENSE](LICENSE).
