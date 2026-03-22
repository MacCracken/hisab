# Security Policy

## Scope

Ganit is a pure mathematics library providing linear algebra, geometry, calculus, and numerical methods for Rust. The core library performs no I/O and contains no `unsafe` code.

## Attack Surface

| Area | Risk | Mitigation |
|------|------|------------|
| Numerical stability | Catastrophic cancellation, overflow | IEEE 754 f32/f64; documented precision limits |
| Gaussian elimination | Division by near-zero pivot | Partial pivoting with 1e-12 threshold |
| Iterative solvers | Non-convergence on adversarial input | max_iter bounds; returns `Err(NoConvergence)` |
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
- All public types are `Send + Sync`
- No network I/O in core library (AI client is opt-in via feature flag)
- Minimal dependency surface (core depends only on glam, serde, thiserror, tracing)
