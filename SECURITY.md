# Security Policy

## Scope

Ganit is a pure mathematics library providing linear algebra, geometry, calculus, numerical methods, spatial structures, and collision detection for Rust. The core library performs no I/O and contains no `unsafe` code.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Numerical stability | Catastrophic cancellation, overflow | IEEE 754 f32/f64; documented precision limits |
| Matrix decompositions | Division by near-zero pivot | Partial pivoting with threshold checks; returns `Err(SingularPivot)` |
| Iterative solvers | Non-convergence on adversarial input | max_iter bounds; returns `Err(NoConvergence)` |
| FFT | Panic on non-power-of-2 input | Documented `# Panics`; V1.0 will return `Result` |
| Integration functions | Panic on n=0 | Documented; V1.0 will return `Result` |
| Spatial structures | Unbounded tree depth | Configurable `max_depth`; prevents stack overflow |
| GJK/EPA | Non-convergence on degenerate shapes | Hardcoded 64-iteration limit; returns best estimate |
| ConvexPolygon | Empty vertex list causes panic | V1.0 will validate in constructor |
| AI client (opt-in) | Network I/O to daimon/hoosh | Feature-gated; not compiled by default |
| Dependencies | Supply chain compromise | cargo-deny, cargo-audit in CI; minimal core deps |

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x | Yes |

## Reporting a Vulnerability

Please report security issues to **security@agnos.dev**.

- You will receive acknowledgement within 48 hours
- We follow a 90-day coordinated disclosure timeline
- Please do not open public issues for security vulnerabilities

## Design Principles

- Zero `unsafe` code
- All public types are `Send + Sync` (compile-time verified)
- No network I/O in core library (AI client is opt-in via feature flag)
- Minimal dependency surface (core depends only on glam, serde, thiserror, tracing)
- Spatial structures have configurable depth limits to prevent resource exhaustion
