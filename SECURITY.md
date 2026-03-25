# Security Policy

## Scope

Hisab is a pure mathematics library providing linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, interval arithmetic, and tensor operations for Rust. The core library performs no I/O and contains no `unsafe` code.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Numerical stability | Catastrophic cancellation, overflow | IEEE 754 f32/f64; documented precision limits |
| Matrix decompositions | Division by near-zero pivot | Partial pivoting with threshold checks; returns `Err(SingularPivot)` |
| Iterative solvers | Non-convergence on adversarial input | max_iter bounds; returns `Err(NoConvergence)` |
| FFT | Invalid input length | Returns `Err(InvalidInput)` for non-power-of-2 |
| Integration functions | Zero step count | Returns `Err(ZeroSteps)` |
| Spatial structures | Unbounded tree depth | Configurable `max_depth`; prevents stack overflow |
| GJK/EPA | Non-convergence on degenerate shapes | Configurable iteration limits (`GJK_MAX_ITERATIONS`, `EPA_MAX_ITERATIONS`) |
| Geometric constructors | Invalid input (zero-length normals, negative radius) | All return `Result`; no panics |
| Symbolic expressions | Deep recursion on pathological trees | Consumer responsibility; practical depth well within stack limits |
| Serde deserialization | Crafted JSON | Enum validation via serde derive |
| AI client (opt-in) | Network I/O to daimon/hoosh | Feature-gated; not compiled by default |
| Dependencies | Supply chain compromise | cargo-deny, cargo-audit in CI; minimal core deps |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.0.x | Yes |
| 0.22.x–0.28.x | Security fixes only |
| < 0.22 | No |

## Reporting

- Contact: **security@agnos.dev**
- Do not open public issues for security vulnerabilities
- 48-hour acknowledgement SLA
- 90-day coordinated disclosure

## Design Principles

- Zero `unsafe` code
- No `unwrap()` or `panic!()` in library code — all errors via `Result`
- All public types are `Send + Sync` (compile-time verified)
- No network I/O in core library (AI client is opt-in via feature flag)
- Minimal dependency surface (core depends only on glam, serde, thiserror, tracing)
- Spatial structures have configurable depth limits to prevent resource exhaustion
