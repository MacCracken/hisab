# Cyrius Port Audit — 2026-04-15

> Audit of hisab v1.4.0 Rust → Cyrius port completeness.

> **Status update (2026-05-29, v2.4.6):** this is the original 2026-04-15
> snapshot, preserved as-is. Since then nearly all of the "Genuine P0 gaps"
> below have been ported — the library is now **34 modules / ~16,460 lines**.
> Done since: full ODE suite (DOPRI45, BDF-2..5, symplectic, SDE), optimization
> (GD/CG/BFGS/L-BFGS/LM), sparse + GMRES/BiCGSTAB/PGS, SVD/eigen, DST/DCT/2D-FFT,
> advanced number theory, all splines + adaptive Simpson + gradient/Jacobian/
> Hessian, simplex noise, spatial (k-d tree/quadtree/octree/spatial hash), the
> **full collision arc** (MPR, sequential-impulse, convex hull, triangulation,
> Delaunay, half-edge, island detection — audited + fixed in 2.4.x), SE(3)/SO(3)
> + adjoint + BCH (lie_ext), einsum, and symbolic integration/LaTeX/pattern
> matching. Of the items that paragraph called open, all but two have since
> shipped: reverse-mode (tape) autodiff (2.11.0), differentiable geometry (the
> six `geo_jet_*` primitives, 2.10.0–2.10.2), deeper differential geometry
> (2.6.x), and the `Result<T,E>` migration (3.0.0). **Status 2026-09-14 (v3.2.0):
> 35 modules / 26,567 lines, 4429 assertions, 80 benchmarks.** Dual quaternions,
> convex *decomposition*, differentiable *rendering* and GPU were never
> scheduled and are not on the roadmap (no consumer has asked; none of the ten
> live consumers reaches beyond 35 public fns in 8 modules) — a consumer asking
> is what would put them there. The port is
> complete as a parity question — this file is the historical record of the
> 2026-04-15 snapshot and is not maintained further.

> **Status 2026-09-30 (v3.3.2): the parity claim above was wrong, and is
> withdrawn.** A grep of `src/` at the 3.3.1 tag finds no implementation of these
> Rust 1.4.0 capabilities. Each is named in the 2026-04-15 gap list below, in the
> Rust benchmark set (`docs/benchmarks-rust-v-cyrius.md`), or in the Rust 1.4.0
> CHANGELOG entry:
>
> - Monte Carlo integration
> - Atkin and segmented sieves: `num_sieve` is Eratosthenes, and returns an empty
>   vec above 10M
> - spline arc length, and the de Casteljau split
> - sparse Cholesky/LU, and SOR: `solve_pgs` takes no relaxation factor
> - Voronoi diagrams: the `src/` hits for "Voronoi" are closest-point region tests
> - Mat4 decompose: `m4_from_srt` composes; `se3_from_mat4` (lie_ext) recovers
>   rotation + translation from a rigid 4x4, but nothing recovers scale (no SRT
>   decompose)
> - plane–plane intersection
> - frustum tests (planned by kiran)
> - spinors, and complex Hermitian eigen and complex SVD (planned by kana). The
>   complex surface is `cx_*`, the `cmat_*` set (including `cmat_exp` and
>   `cmat_inverse`), Pauli/Dirac/Gell-Mann and `cqr_decompose`. So the Module
>   Coverage table's *Complete* for Complex overstated it against Rust 1.4.0. The
>   Consumer impact table's "SVD/eigen when linalg.cyr ships" is met for the real
>   solvers, which shipped, and not for the complex forms.
>
> All of these are **not ported, and demand-gated on the roadmap** (§ 3.x.x —
> demand-gated, *Rust-era features never ported*: audit D158 and D161). The trigger
> for each is a consumer asking. The four named in the 2026-09-14 status above
> (dual quaternions, convex decomposition, differentiable rendering, GPU) are now
> listed there as well (D159).
>
> The parallel and AI modules' premises in the gap list are outdated: the cyrius
> 6.6.12 tag ships `lib/thread.cyr` and `lib/http.cyr`. The parallel module is on
> the same demand-gated list (D160). The AI module would conflict with
> SECURITY.md's "No network I/O in core library" (D160); it is not on the roadmap,
> and whether to drop it for good is the maintainer's decision. Serialization,
> which the Rust crate had through serde, is on the demand-gated list. The 6.6.12
> stdlib has no JSON or serde module, but it does ship `lib/protobuf.cyr`. The
> symbolic "bridge" (Rust 1.3.0's abaco bridge) was not ported either. It has no
> roadmap row and no recorded disposition, so dropping it is left to the
> maintainer. What argues against porting it: the roadmap's *Scope* assigns
> expression parsing to abaco, and *Boundary with Abaco* says hisab should never
> depend on abaco.

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
- Parallel module (requires Cyrius threading) *(outdated: the cyrius 6.6.12 tag ships
  `lib/thread.cyr`; see the 2026-09-30 status)*
- AI module (requires Cyrius HTTP) *(outdated: the cyrius 6.6.12 tag ships `lib/http.cyr`; the
  module is dropped, since it would conflict with SECURITY.md's "No network I/O in core library")*
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
