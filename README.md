# Ganit

> **Ganit** (Sanskrit: गणित — mathematics) — higher mathematics library for AGNOS

Higher math for the AGNOS ecosystem. Provides typed mathematical operations — linear algebra, geometry, calculus, and numerical methods — built on [glam](https://crates.io/crates/glam). Used by [impetus](https://github.com/MacCracken/impetus) (physics), [kiran](https://github.com/MacCracken/kiran) (game engine), and [joshua](https://github.com/MacCracken/joshua) (simulation).

For expression evaluation and unit conversion, see [abaco](https://github.com/MacCracken/abaco).

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `transforms` | yes | 2D/3D affine transforms, projections, slerp, lerp, glam re-exports |
| `geo` | yes | Primitives, intersections, BVH, k-d tree, quadtree, octree, spatial hash, GJK/EPA |
| `calc` | yes | Differentiation, integration, Bezier 2D/3D, splines, easing, Gauss-Legendre |
| `num` | yes | Root finding, LU/Cholesky/QR, eigenvalues, FFT, RK4 ODE solver |
| `ai` | no | Daimon/hoosh AI client (network deps) |
| `logging` | no | Structured logging via `GANIT_LOG` env var |
| `full` | — | Enables all features |

## Modules

| Module | Description |
|--------|-------------|
| `transforms` | `Transform2D`, `Transform3D`, projections, slerp, lerp, handedness conversion |
| `geo` | `Ray`, `Plane`, `Aabb`, `Sphere`, `Triangle`, `Line`, `Segment`, `Frustum`, `Bvh`, `KdTree`, `Quadtree`, `Octree`, `SpatialHash`, `ConvexPolygon`, GJK/EPA collision |
| `calc` | `derivative`, integration (trapezoidal, Simpson, Gauss-Legendre), Bezier 2D/3D, Catmull-Rom, B-spline, de Casteljau, arc-length, easing functions |
| `num` | `newton_raphson`, `bisection`, `gaussian_elimination`, LU/Cholesky/QR, `least_squares_poly`, `eigenvalue_power`, `Complex`, FFT/IFFT, `rk4` ODE solver |
| `ai` | `DaimonClient` for AGNOS daimon/hoosh integration |

## Quick Start

```toml
[dependencies]
ganit = "0.22"
```

```rust
use ganit::{Vec3, Quat, Transform3D, Ray, Sphere};
use ganit::geo::{ray_sphere, gjk_intersect};
use ganit::calc::integral_simpson;
use ganit::num::{newton_raphson, fft, Complex};

// 3D transform
let t = Transform3D::new(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY, Vec3::ONE);
let world_pos = t.apply_to_point(Vec3::ZERO);

// Ray-sphere intersection
let ray = Ray::new(Vec3::ZERO, Vec3::Z);
let sphere = Sphere::new(Vec3::new(0.0, 0.0, 5.0), 1.0);
let hit = ray_sphere(&ray, &sphere); // Some(4.0)

// Numerical integration
let area = integral_simpson(|x| x * x, 0.0, 1.0, 100); // ~0.3333

// Root finding
let sqrt2 = newton_raphson(|x| x * x - 2.0, |x| 2.0 * x, 1.0, 1e-10, 100).unwrap();
```

## Building

```sh
cargo build
cargo test --all-features   # 360 tests
make bench                  # 82 criterion benchmarks with history tracking
```

## Roadmap

See [docs/development/roadmap.md](docs/development/roadmap.md) for the full plan through V2.0.

## License

GPL-3.0 — see [LICENSE](LICENSE).
