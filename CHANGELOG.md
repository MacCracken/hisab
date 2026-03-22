# Changelog

## 0.1.0 (2026-03-22)

Initial release.

### Modules

- **transforms**: Vec2/Vec3/Vec4/Mat3/Mat4/Quat re-exports from glam, `Transform2D`, `Transform3D`, orthographic and perspective projections, `lerp_f32`, `lerp_vec3`.
- **geo**: `Ray`, `Plane`, `Aabb`, `Sphere` primitives with `ray_plane`, `ray_sphere` (quadratic formula), and `ray_aabb` (slab method) intersection tests.
- **calc**: Central-difference `derivative`, `integral_trapezoidal`, `integral_simpson`, linear interpolation, quadratic and cubic Bezier curves.
- **num**: `newton_raphson` and `bisection` root finding, `gaussian_elimination` with partial pivoting.
- **ai**: `DaimonClient` for AGNOS daimon/hoosh AI integration (feature-gated).

### Infrastructure

- Flat `src/` module structure with feature flags (`transforms`, `geo`, `calc`, `num`, `ai`, `logging`).
- Unified `GanitError` with `#[non_exhaustive]`.
- 130+ unit tests across all modules.
- CI pipeline: fmt, clippy, test, security audit, supply chain, MSRV 1.89, coverage.
- Release pipeline: multi-platform build, crates.io publish, GitHub Release.
- Full documentation: README, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, codecov.
