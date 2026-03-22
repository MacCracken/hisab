# Ganit

**AGNOS math library** (Sanskrit: *ganit* = mathematics/calculation)

A Rust workspace providing core math, geometry, calculus, numerical methods, and
AGNOS AI integration for the AGNOS operating system ecosystem.

## Crate Architecture

| Crate | Description |
|-------|-------------|
| `ganit-core` | Vectors, matrices, quaternions, 2D/3D transforms, projections (built on glam) |
| `ganit-geo` | Geometric primitives: rays, planes, AABBs, spheres, intersection tests |
| `ganit-calc` | Calculus: differentiation, integration (trapezoidal/Simpson), Bezier curves |
| `ganit-num` | Numerical methods: Newton-Raphson, bisection, Gaussian elimination |
| `ganit-ai` | AGNOS daimon/hoosh client for AI-assisted math |

## Building

```sh
cargo build --workspace
cargo test --workspace
```

## License

GPL-3.0 -- see [LICENSE](LICENSE).
