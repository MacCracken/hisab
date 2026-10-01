# Architecture Overview

> Equation reference: see [`math.md`](math.md) (CGA operators + a catalogue index
> of the library's other formula families).
>
> hisab v4.0.0 — 35 math modules in `src/` plus the bundle's `visibility.cyr` marker, 26,627
> lines of Cyrius (`lib/` is vendored stdlib + first-party deps only). Compiled by cycc 6.6.12; a
> consumer needs cycc ≥ 6.6.3 (`public struct` + `#derive`, since 3.1.0).

## Module Map

`src/visibility.cyr` is not a math module: it is the single `private` line `cyrius distlib` puts
first in `dist/hisab.cyr`, which makes the bundle refuse every name not marked `public` (4.0.0).

```
hisab (Cyrius)
├── Foundation types
│   ├── error.cyr          — Error codes (HSB_ERR_*), epsilon constants
│   ├── f64_util.cyr       — f64_tan/fmod/copysign/approx_eq (f64_le/f64_ge come from stdlib `math`)
│   ├── vec2.cyr           — HVec2: 2D f64 vector (heap-allocated)
│   ├── vec3.cyr           — HVec3: 3D f64 vector with cross, reflect, min/max (SIMD f64v)
│   ├── vec4.cyr           — HVec4: 4D f64 vector, Vec3 conversion (SIMD f64v)
│   ├── quat.cyr           — HQuat: quaternion with slerp, rotation, axis-angle
│   ├── mat3.cyr           — 3x3 matrix: mul, inverse, determinant, from_quat
│   └── mat4.cyr           — 4x4 matrix: inverse, SRT, projections, look-at
│
├── Transforms
│   ├── transforms.cyr     — Transform2D/3D, compose, Euler, screen projection, lerp
│   └── color.cyr          — sRGB/HSV/HSL/Oklab, Porter-Duff (8 ops), tone mapping, SH L2, EV
│
├── Geometry
│   ├── geo.cyr            — 9 primitives, 6 ray tests, closest-point queries
│   ├── geo_advanced.cyr   — GJK/EPA 3D, MPR/XenoCollide, SDF+CSG, swept AABB, TOI, CGA 5D, BVH
│   ├── geo_diff.cyr       — Ray/surface JETS for all six primitives: t plus its full
│   │                        gradient from one evaluation, as a post-pass on the shipped primal
│   └── spatial.cyr        — k-d tree, octree, quadtree, spatial hash (BVH lives in geo_advanced)
│
├── Collision
│   ├── collision_core.cyr — sequential-impulse solver + PGS/LCP, convex hull 2D
│   │                         (monotone chain), triangulation (ear clipping)
│   └── collision_mesh.cyr — Delaunay (Bowyer-Watson), half-edge mesh, island detection (union-find)
│
├── Calculus
│   ├── calc.cyr           — Derivative, Simpson/Gauss-Legendre, Bezier, easing, Perlin 2D
│   ├── calc_ext.cyr       — Gradient/Jacobian/Hessian, adaptive Simpson, B-spline, NURBS,
│   │                         Hermite TCB, monotone cubic, 3D Perlin noise
│   └── noise_simplex.cyr  — Simplex noise (2D/3D)
│
├── Numerical
│   ├── num.cyr            — Newton/bisection, FFT/IFFT, RK4, PCG32, primes, sieve, Kahan sum
│   ├── ode.cyr            — DOPRI45, backward Euler, BDF-2..5, SDE, symplectic, Verlet, Yoshida
│   ├── optimize.cyr       — Gradient descent, CG (Polak-Ribiere+), BFGS, L-BFGS, LM
│   ├── linalg_ext.cyr     — CSR sparse, GMRES, BiCGSTAB, PGS/LCP, SVD, eigen, Lyapunov, inertia
│   ├── linalg_precision.cyr — Compensated / high-precision linear algebra
│   └── num_ext.cyr        — Extended GCD, totient, Mobius, factorize, CRT, DST/DCT, 2D-FFT,
│                             Halton/Sobol, tridiagonal solver
│
├── Physics
│   ├── complex.cyr        — Complex numbers + matrices, Pauli, Dirac gamma, matrix exp
│   ├── lie.cyr            — U(1), SU(2) incl. its adjoint action, SU(3) Gell-Mann, SO(3,1) Lorentz
│   ├── lie_ext.cyr        — SE(3)/SO(3), adjoint, exp/log maps, BCH
│   └── diffgeo.cyr        — Christoffel→Einstein, geodesic RK4, Killing, exterior algebra
│
├── Symbolic
│   ├── symbolic.cyr       — Expr tree, evaluate, differentiate, simplify, to_str
│   └── symbolic_ext.cyr   — Symbolic integration, LaTeX rendering, pattern matching + rewrite
│
└── Other
    ├── autodiff.cyr       — Forward-mode duals AND a tape-based reverse mode: one sweep
    │                        for an n-input gradient where forward needs n passes
    ├── interval.cyr       — Interval arithmetic
    ├── tensor.cyr         — N-D dense tensor, Kronecker/Minkowski/Levi-Civita
    └── einsum.cyr         — Einstein-summation contraction (bounded reused arena)
```

## Dependencies

| Dependency | Source | Purpose |
|-----------|--------|---------|
| sakshi | External (git) | Structured logging |
| alloc, string, fmt, vec, str | Cyrius stdlib | Core data structures |
| math, ganita | Cyrius stdlib | `math`: inclusive cmp, clamp/lerp/min/max/sign, polyfills. `ganita` (6.2.x umbrella): transcendentals + dense matrix + decompositions (subsumes the former `matrix`/`linalg`) |
| tagged, fnptr, callback | Cyrius stdlib | Option/Result types, function pointers, closure-like callback patterns |
| syscalls, io, args | Cyrius stdlib | System interface |
| assert, bench | Cyrius stdlib | Testing, benchmarking |

## Module dependency manifest

Cyrius modules in `src/` carry **no `include` lines** — they are self-contained text that the
bundler concatenates. That makes à-la-carte consumption possible but leaves the required set
implicit, which is why the README's à-la-carte example shipped broken until 2.7.0-H (it named
`lib/` paths and omitted two modules the example needed without ever mentioning them).

This table is **derived and verified, not hand-maintained**. Each row was produced with the
compiler as the oracle: compile the module alone, map every undefined name it reports to its
defining module, add that module, and repeat until the build is clean. Then each row was checked
twice: it builds with **0 undefined names** against exactly its listed set, and dropping any ONE
listed module breaks the build. Both checks hold for all 35 rows (re-derived 2026-09-30 on 4.0.0).
Undefined *functions* count even when nothing calls them. A function is only a warning until
something reaches it, and a set that leaves one out is a trap for whoever calls it next.

⚠ **Re-deriving it found the published table wrong in five rows.** Three were already wrong in
3.2.2, which the same procedure run on that tree confirms:
- `f64_util` was listed as needing `error` and needs nothing;
- `mat3` and `mat4` were missing `quat`.

The other two were wrong for 3.2.2 and are changed by 4.0.0:
- `collision_core` was listed as `error vec3`. On 3.2.2 it really needed `error geo geo_advanced
  quat vec2 vec3`, because MPR called geo_advanced's EPA helpers.
- `collision_mesh` inherited all of that. 4.0.0 moved MPR into `geo_advanced`, so both rows are
  small now.

The sets are minimal in the sense that they contain no module that is not reached; they are the
transitive closure, so they are what you must include, not merely what the module names directly.

Consumers pulling the whole `dist/hisab.cyr` bundle can ignore this table entirely.

The stdlib half of the dependency set stopped being implicit in 2.9.2. `dist/hisab.deps` — the
sidecar `cyrius distlib` writes beside the bundle, which a consumer's `cyrius deps` reads to
auto-resolve stdlib leaves — is now **tracked** rather than gitignored, and names all 16 leaves the
fold needs in scope: `syscalls`, `io`, `string`, `alloc`, `str`, `fmt`, `vec`, `args`, `assert`,
`math`, `ganita`, `tagged`, `result` (since 3.0.0), `fnptr`, `bench`, `callback`. It was worth
ignoring until cyrius 6.5.10
for the same reason this section exists: the sidecar was built by scanning bundled sources for
literal `include "lib/X.cyr"` lines, and hisab's 35 `[lib]` modules have none, so it reported 2.
6.5.10 unions in the declared `[deps] stdlib`, which is what takes it from 2 to 16.

| Module | Also include |
|---|---|
| `error` | *(standalone)* |
| `f64_util` | *(standalone)* |
| `interval` | *(standalone)* |
| `symbolic` | *(standalone)* |
| `tensor` | *(standalone)* |
| `autodiff` | `error` |
| `complex` | `error` |
| `diffgeo` | `error` |
| `einsum` | `tensor` |
| `num` | `error` |
| `ode` | `error` |
| `optimize` | `error` |
| `symbolic_ext` | `symbolic` |
| `vec2` | `error` |
| `vec3` | `error` |
| `collision_core` | `error` `vec2` |
| `linalg_ext` | `complex` `error` |
| `num_ext` | `error` `num` |
| `quat` | `error` `vec3` |
| `vec4` | `error` `vec3` |
| `calc` | `error` `vec2` `vec3` |
| `collision_mesh` | `collision_core` `error` `vec2` |
| `color` | `error` `vec3` `vec4` |
| `geo` | `error` `quat` `vec3` |
| `mat3` | `error` `quat` `vec3` |
| `calc_ext` | `calc` `error` `vec2` `vec3` |
| `geo_advanced` | `error` `geo` `quat` `vec3` |
| `geo_diff` | `error` `geo` `quat` `vec3` |
| `linalg_precision` | `complex` `error` `linalg_ext` (`linalg_sort_desc` orders every S / eigenvalue vector) |
| `noise_simplex` | `calc` `error` `vec2` `vec3` |
| `spatial` | `error` `geo` `quat` `vec3` |
| `mat4` | `error` `f64_util` `quat` `vec3` `vec4` |
| `transforms` | `error` `f64_util` `mat4` `quat` `vec2` `vec3` `vec4` |
| `lie` | `complex` `error` `f64_util` `mat3` (3.2.0: `MAT3_BYTES`) `mat4` `quat` `vec3` `vec4` |
| `lie_ext` | `complex` `error` `f64_util` `lie` `mat3` `mat4` `quat` `vec3` `vec4` |

**Shape of the graph.** Five modules are fully standalone (`error`, `f64_util`, `interval`,
`symbolic`, `tensor`); the deepest is `lie_ext` at 9. There are **no cycles**, and every edge runs
from a derived module to its base, which is the layering the module *names* already declare.
**Since 4.0.0 no module reaches another module's `_` internals.** Through 3.2.x nine pairs did.
Eight of them now go through public names or plain literals:

- `calc_ext`→`calc` and `noise_simplex`→`calc`;
- `collision_mesh`→`collision_core`;
- `geo_diff`→`geo`;
- `lie_ext`→`lie`;
- `num_ext`→`num`;
- `symbolic_ext`→`symbolic`;
- `linalg_precision`→`linalg_ext`.

The ninth, `collision_core`→`geo_advanced`, ran UP the layering: MPR called EPA's helpers. The
paragraph that stood here listed seven pairs and missed both `geo_diff`→`geo` and this one, which
was removed by moving MPR into `geo_advanced` beside them. `scripts/check-public-surface.sh`
claims 0 and 1 enforce the rule: claim 0 refuses a `_` name that carries `public`, and claim 1
flips every module `private` and includes each one as its own
file.

## Design Principles

- **Pure math** — no I/O in library code
- **f64 everywhere** — all math is IEEE 754 double precision. ⚠ There is no house tolerance: the 2.14.0 census found 97 of 136 `EPSILON_F64` guards wrong (an absolute 1e-12 against a quantity in the caller's units), and guards are now exact, DBL_MIN-derived, or relative — *guard exactly what makes the division fail and nothing more*
- **Heap-allocated types** — multi-field structs via `alloc(sizeof(T))` + `#derive(accessors)`; every public struct's exact `sizeof` is pinned in `tests/abuse.tcyr` (3.2.0), so a layout change fails a test rather than a consumer
- **`Result<T, E>`** (3.0.0) — the 48 fallible entry points return `Ok(0)` / `Err(HSB_ERR_*)` and carry `#must_use`; callers bind both halves and test `is_err_result`. The `HSB_ERR_*` codes (namespaced from bare `ERR_*` in 2.6.8) are the `E`; a `Result` passed straight into an argument list silently degrades to its tag
- **Out-parameters** — results written via `store64(out, value)` pointers
- **No abort** — library code never calls `syscall(60, ...)` / `sys_exit_group` (warnings only)
- **Overflow guards** — allocation sizes checked against caps for user-controlled dimensions
- **Function pointers** — callbacks via `fncall1`/`fncall2` from fnptr.cyr

## Data Flow — Collision Pipeline

```
Scene objects
      │
      ▼
┌─────────┐
│  BVH /  │  (bvh_build, bvh_query_ray/aabb; k-d tree, octree, spatial hash)
│ spatial │
└────┬────┘
     ▼
Candidate pairs
     │
     ▼
┌────────────────────┐
│  Narrowphase:      │  GJK/EPA  (gjk_intersect_3d, gjk_epa_3d)
│  GJK/EPA  or  MPR  │  MPR      (mpr_intersect, mpr_penetration — XenoCollide)
└────┬───────────────┘
     ▼
Penetration { normal, depth }  →  contact_new()
     │
     ▼
┌──────────────────────────────┐
│  Constraint solve:           │  sequential_impulse()  (accumulate-clamp impulses)
│  sequential_impulse / PGS    │  solve_pgs()           (projected Gauss-Seidel LCP)
└──────────────────────────────┘
     │
     ▼
detect_islands()  (union-find — partitions the contact graph)
```

## Data Flow — ODE Solving

```
dy/dt = f(t, y)
      │
      ├── Explicit ─── num_rk4, ode_dopri45
      ├── Implicit ─── ode_backward_euler, ode_bdf2..5
      ├── Symplectic ─ ode_symplectic_euler, ode_verlet, ode_yoshida4
      └── Stochastic ─ ode_euler_maruyama, ode_milstein
```

## Consumers

Ten repos pull `dist/hisab.cyr` SHA-locked today — prakash (tag 3.2.1, the first past the `Result`
break), svara, naad, goonj (tag 2.22.1), dhvani, attn11, ghurni, prani, garjan, nidhi (tag 2.11.2);
pins read from their manifests 2026-09-30; goonj and attn11 pin cyrius 6.6.2, below the 3.1.0+
bundle's minimum. Four more (jalwa, ranga, shabda, shabdakosh) carry a copy transitively. The
whole surface they reach is **35 public fns in 8 modules** (vec3, num, num_ext, calc, calc_ext, geo,
geo_advanced, f64_util) — the measured per-module column is in the roadmap's *Boundary with Abaco*
table. The projects below are the **planned** consumers: Rust repos awaiting a Cyrius port, with no
`cyrius.cyml` on any branch.

| Planned project | What it would use |
|---------|-------------|
| **impetus** | Transforms, GJK/EPA, PGS solver, inertia tensors, BVH |
| **kiran** | Projections, frustum, BVH, ray tests, easing |
| **joshua** | DOPRI45, BDF, symplectic, optimization, PCG32 |
| **aethersafha** | Projections, compositing, tone mapping, color |
| **abaco** | Symbolic algebra, interval arithmetic — a plan on hisab's side only: abaco's README calls hisab a sibling, not a consumer |
| **hisab-mimamsa** | Tensors, Lie groups, diffgeo, complex LA, CGA |
| **kana** | Tensors, Lie groups, complex LA, spinors |
