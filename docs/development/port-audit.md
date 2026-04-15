# Cyrius Port Audit — 2026-04-15

> Audit of hisab v1.4.0 Rust → Cyrius port completeness.

## Summary

| Metric | Rust | Cyrius | Status |
|--------|------|--------|--------|
| Library source | 33,612 lines | 6,674 lines | 20 lib files |
| Test assertions | 1,155 | 661+ (4 test files) | Edge cases in progress |
| Benchmarks | 14 groups (criterion) | 22 benches (bench.cyr) | + comparison doc |
| Fuzz harnesses | 0 | 5 targets | New for Cyrius |
| Binary | ~800KB dynamic | 269KB static | 3x smaller |
| Dependencies | 9 crates | 1 (sakshi) | |

## Module Coverage

| Module | Cyrius file | Lines | Status | Notes |
|--------|-------------|-------|--------|-------|
| error | error.cyr | 27 | Complete | |
| f64 utilities | f64_util.cyr | 27 | Complete | tan, fmod, copysign |
| Vec2 | vec2.cyr | 74 | Complete | |
| Vec3 | vec3.cyr | 125 | Complete | |
| Vec4 | vec4.cyr | 83 | Complete | |
| Quat | quat.cyr | 154 | Complete | |
| Mat4 | mat4.cyr | 320 | Complete | inverse, SRT, projections, look-at |
| Transforms | transforms.cyr | 192 | Complete | T2D/T3D, Euler, screen, interpolation |
| Color | color.cyr | 226 | Complete | sRGB, Porter-Duff, tone mapping, SH, EV |
| Geo primitives | geo.cyr | 689 | Complete | 9 types, 6 ray tests, closest-point |
| Geo advanced | geo_advanced.cyr | 1,158 | Complete | GJK/EPA, BVH, SDF/CSG, TOI, CGA 5D |
| Calculus | calc.cyr | 442 | Core done | Integration, Bezier, easing, Perlin |
| Numerical | num.cyr | 566 | Core done | Newton, bisection, FFT, RK4, PCG32, primes |
| Complex | complex.cyr | 436 | Complete | Numbers, matrices, Pauli, Dirac, mat_exp |
| Lie groups | lie.cyr | 545 | Complete | U(1), SU(2), SU(3), SO(3,1) |
| Diff geometry | diffgeo.cyr | 592 | Complete | Christoffel→Einstein, geodesics, exterior |
| Symbolic | symbolic.cyr | 600 | Core done | Expr tree, eval, diff, simplify, to_str |
| Autodiff | autodiff.cyr | 99 | Forward done | Dual numbers |
| Interval | interval.cyr | 116 | Complete | |
| Tensor | tensor.cyr | 203 | Complete | Dense N-D, Kronecker, Minkowski, Levi-Civita |

## Ported but not in audit agent's list

These were flagged as missing but ARE in the port:
- Screen transforms (world_to_screen, screen_to_world_ray) → transforms.cyr
- Spherical harmonics L2 → color.cyr
- SDF primitives + CSG → geo_advanced.cyr
- BVH (build, query_ray, query_aabb) → geo_advanced.cyr
- Swept AABB + time_of_impact → geo_advanced.cyr
- CGA 5D multivectors → geo_advanced.cyr

## Genuine P0 gaps (not yet ported)

### Numerical methods
- ODE beyond RK4: DOPRI45, backward Euler, BDF-2 through BDF-5
- Stochastic: Euler-Maruyama, Milstein
- Symplectic: Euler, Verlet, leapfrog, Yoshida 4th
- Optimization: gradient descent, conjugate gradient, BFGS, L-BFGS, Levenberg-Marquardt
- Sparse: CsrMatrix, sparse Cholesky/LU, GMRES, BiCGSTAB, PGS+SOR
- Decomposition: SVD, eigendecomposition (awaiting linalg.cyr in Cyrius 4.10.3)
- Transforms: DST, DCT, 2D-FFT
- Advanced number theory: Atkin sieve, segmented sieve, Pollard rho, extended GCD, totient, Mobius, CRT, continued fractions

### Calculus
- Splines: B-spline, NURBS, Hermite TCB, monotone cubic, de Casteljau, arc-length
- Integration: adaptive Simpson, Monte Carlo
- Multivariate: partial derivative, gradient, Jacobian, Hessian

### Geometry
- Collision: MPR/XenoCollide, sequential impulse solver, convex decomposition
- Spatial: k-d tree, quadtree, octree, spatial hash (BVH is done)
- Mesh: Delaunay/Voronoi, half-edge mesh, convex hull 2D, polygon triangulation
- Island detection

### Other
- Reverse-mode autodiff (Tape)
- Symbolic: integration, LaTeX, pattern matching, bridge
- Dual quaternions
- Mat4 decompose/recompose
- Parallel module (requires Cyrius threading)
- AI module (requires Cyrius HTTP)
- Logging module (sakshi covers this)

## Infrastructure gaps

| Item | Status | Action needed |
|------|--------|---------------|
| README.md | Rust-centric | Update with Cyrius quick-start, feature matrix |
| CONTRIBUTING.md | Rust-centric | Update with Cyrius workflow |
| SECURITY.md | Rust-centric | Update for Cyrius |
| CHANGELOG.md | No port entry | Add Cyrius 1.4.0 port entry |
| docs/architecture | Rust-centric | Mark ported modules |
| docs/roadmap | Rust-only | Add Cyrius section |
| CI workflows | cyrius port created | Integrate test runner |
| bench-history.csv | Not started | Create Cyrius version |

## Consumer impact

| Consumer | Can use Cyrius port now? | Blocking items |
|----------|------------------------|----------------|
| **kiran** (engine) | Mostly | Spatial structures beyond BVH |
| **aethersafha** (compositor) | Yes | — |
| **svara** (vocal synthesis) | Yes | FFT/complex all done |
| **abaco** (expression eval) | Mostly | Symbolic integration, LaTeX |
| **hisab-mimamsa** (physics) | Mostly | SVD/eigen when linalg.cyr ships |
| **kana** (quantum) | Mostly | SVD/eigen when linalg.cyr ships |
| **impetus** (physics) | Partially | Needs sparse solvers, impulse solver |
| **joshua** (simulation) | Partially | Needs DOPRI45, BDF, optimization |
