# Ganit

> **Ganit** (Sanskrit: गणित — mathematics) — higher mathematics library for AGNOS

Higher math for the AGNOS ecosystem. Provides typed mathematical operations — linear algebra, geometry, calculus, and numerical methods — built on [glam](https://crates.io/crates/glam). Used by [impetus](https://github.com/MacCracken/impetus) (physics), [kiran](https://github.com/MacCracken/kiran) (game engine), and [joshua](https://github.com/MacCracken/joshua) (simulation).

For expression evaluation and unit conversion, see [abaco](https://github.com/MacCracken/abaco).

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `transforms` | yes | 2D/3D affine transforms, projections, lerp, glam re-exports |
| `geo` | yes | Rays, planes, AABBs, spheres, ray intersection tests |
| `calc` | yes | Differentiation, integration (trapezoidal/Simpson), Bezier curves |
| `num` | yes | Newton-Raphson, bisection, Gaussian elimination |
| `ai` | no | Daimon/hoosh AI client (network deps) |
| `logging` | no | Structured logging via `GANIT_LOG` env var |
| `full` | — | Enables all features |

## Modules

| Module | Description |
|--------|-------------|
| `transforms` | `Transform2D`, `Transform3D`, orthographic/perspective projections, lerp |
| `geo` | `Ray`, `Plane`, `Aabb`, `Sphere`, `ray_plane`, `ray_sphere`, `ray_aabb` |
| `calc` | `derivative`, `integral_trapezoidal`, `integral_simpson`, `bezier_quadratic`, `bezier_cubic` |
| `num` | `newton_raphson`, `bisection`, `gaussian_elimination` |
| `ai` | `DaimonClient` for AGNOS daimon/hoosh integration |

## Quick Start

```toml
[dependencies]
ganit = "0.1"
```

```rust
use ganit::{Vec3, Quat, Transform3D, Ray, Sphere};
use ganit::geo::ray_sphere;
use ganit::calc::integral_simpson;
use ganit::num::newton_raphson;

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
cargo test --all-features   # 130+ tests
```

## License

GPL-3.0 — see [LICENSE](LICENSE).
