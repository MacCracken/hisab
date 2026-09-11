# Hisab

> **Hisab** (Arabic: حساب — calculation) — higher mathematics library for AGNOS

Higher math for the AGNOS ecosystem — linear algebra, geometry, calculus, numerical methods, automatic differentiation, symbolic algebra, and more. Written in [Cyrius](https://github.com/MacCracken/cyrius), ported from Rust.

Used by **svara**, **naad**, **goonj**, **dhvani**, **attn11**, **ghurni**, **prani**, **garjan**, **prakash** and **nidhi** — ten repos that pull `dist/hisab.cyr` SHA-locked (verified 2026-09-09).

⚠ [impetus](https://github.com/MacCracken/impetus) (physics), [kiran](https://github.com/MacCracken/kiran) (engine), [joshua](https://github.com/MacCracken/joshua) (simulation), [aethersafha](https://github.com/MacCracken/aethersafha) (compositor), [hisab-mimamsa](https://github.com/MacCracken/hisab-mimamsa) (theoretical physics) and [kana](https://github.com/MacCracken/kana) (quantum science) are **planned** — they are Rust repos awaiting a Cyrius port and have no `cyrius.cyml` on any branch.

For expression evaluation and unit conversion, see [abaco](https://github.com/MacCracken/abaco).

## Modules

| Module | Files | Description |
|--------|-------|-------------|
| **Core** | f64_util, error | f64 helpers the stdlib does not carry; the `HSB_ERR_*` code set every fallible entry point returns |
| **Foundation** | vec2, vec3, vec4, quat, mat3, mat4 | Vector/matrix/quaternion types (f64, heap-allocated; SIMD `f64v_*` hot paths) |
| **Transforms** | transforms, color | 2D/3D affine transforms, projections, slerp/lerp, Euler angles, sRGB/HSV/HSL/Oklab, Porter-Duff compositing (8 ops), tone mapping (Reinhard, ACES), SH L2, EV/exposure |
| **Geometry** | geo, geo_advanced, geo_diff, spatial | 9 primitives, 6 ray tests, closest-point queries, GJK/EPA 3D, SDF+CSG, swept-AABB/TOI, conformal geometric algebra (5D CGA); spatial structures (BVH, k-d tree, octree, quadtree, spatial hash); differentiable ray/surface intersection for **all six** primitives — plane, sphere, triangle, aabb, obb and capsule jets returning the full gradient from one evaluation |
| **Collision** | collision_core, collision_mesh | MPR/XenoCollide narrowphase, sequential-impulse contact solver, convex hull 2D (monotone chain), polygon triangulation (ear clipping), Delaunay (Bowyer-Watson), half-edge mesh, island detection (union-find) |
| **Calculus** | calc, calc_ext, noise_simplex | Differentiation, integration (Simpson, Gauss-Legendre, adaptive), Bezier/Catmull-Rom/B-spline/NURBS/Hermite TCB/monotone cubic, easing, Perlin 2D+3D, simplex noise, gradient/Jacobian/Hessian |
| **Numerical** | num, ode, optimize, linalg_ext, linalg_precision, num_ext | Root finding, FFT/DST/DCT/2D-FFT, ODE (RK4, DOPRI45, BDF, symplectic, SDE), optimization (GD, CG, BFGS, L-BFGS, LM), sparse CSR, GMRES, PGS/LCP, SVD, eigendecomposition, compensated/high-precision linalg, number theory (primes, factorize, CRT, totient, Mobius), PCG32, Halton/Sobol, tridiagonal solver |
| **Complex** | complex | Complex numbers + matrices, Pauli/Dirac gamma matrices, matrix exponential |
| **Lie groups** | lie, lie_ext | U(1), SU(2), SU(3) Gell-Mann, SO(3,1) Lorentz, SE(3)/SO(3), exponential/log maps, adjoint, BCH |
| **Diff geometry** | diffgeo | Christoffel symbols, Riemann/Ricci/Einstein tensors, geodesic solver, Killing vectors, exterior algebra |
| **Symbolic** | symbolic, symbolic_ext | Expression tree, evaluate, differentiate, simplify, symbolic integration, LaTeX rendering, pattern matching + rewrite rules |
| **Autodiff** | autodiff | Forward-mode (dual numbers) **and** tape-based reverse mode — one sweep for an n-input gradient where forward needs n passes, measured 11.2x at n = 16. Pairs with `optimize.cyr`'s solvers through a capturing closure, no API change |
| **Interval** | interval | Interval arithmetic for verified numerics |
| **Tensor** | tensor, einsum | N-dimensional dense tensor, Kronecker delta, Minkowski metric, Levi-Civita, Einstein-summation (einsum) |

## Quick Start

```toml
# cyrius.cyml
[package]
name        = "your-project"
version     = "${file:VERSION}"
language    = "cyrius"
cyrius      = "6.6.2"

[deps]
# `ganita` (Cyrius 6.2.x) provides the transcendentals (acos/asin/atan2/pow/
# sinh/cosh/tanh + inverses) AND subsumes `matrix`/`linalg` — do not list those
# alongside it or you get duplicate-definition collisions. `math` stays for the
# inclusive comparisons, clamp/lerp/min/max/sign and the polyfills.
stdlib = ["string", "fmt", "alloc", "vec", "str", "math", "ganita", "tagged", "fnptr"]

[deps.hisab]
git     = "https://github.com/MacCracken/hisab.git"
tag     = "2.22.0"
modules = ["dist/hisab.cyr"]   # ~1.0 MB self-contained bundle (all 35 modules)
# `dist/hisab.deps` is tracked as of 2.9.2 -- `cyrius deps` reads that sidecar and
# pulls in hisab's own 15 stdlib leaves, so the `stdlib` list above only has to
# name what *your* code uses.
# Or pull individual files for a smaller compilation unit. The per-module include
# sets are tabulated in docs/architecture/overview.md -- derived from the source
# and verified by compiling each of the 35 modules against exactly its listed set.
# The example below needs vec2/vec4 even though it never names them:
# modules = ["src/f64_util.cyr", "src/error.cyr", "src/vec2.cyr", "src/vec3.cyr", ...]
```

```cyrius
include "src/f64_util.cyr"
include "src/error.cyr"
include "src/vec2.cyr"
include "src/vec3.cyr"
include "src/vec4.cyr"
include "src/quat.cyr"
include "src/mat4.cyr"
include "src/transforms.cyr"
include "src/geo.cyr"
include "src/calc.cyr"
include "src/num.cyr"

alloc_init();

# 3D transform with composition
var pos = hvec3_new(f64_from(1), f64_from(2), f64_from(3));
var t = t3d_new(pos, hquat_identity(), hvec3_one());
var world = t3d_apply(t, hvec3_zero());

# Ray-sphere intersection
var ray = geo_ray_new(hvec3_zero(), hvec3_unit_z());
var sphere = geo_sphere_new(hvec3_new(0, 0, f64_from(5)), f64_from(1));
var t_hit = geo_ray_sphere(ray, sphere);  # 4.0

# Numerical integration (Simpson's rule)
fn x_squared(x) { return f64_mul(x, x); }
var result = alloc(8);
calc_integral_simpson(&x_squared, 0, F64_ONE, 100, result);
# load64(result) ≈ 0.3333

# Root finding (Newton-Raphson for sqrt(2))
fn f(x) { return f64_sub(f64_mul(x, x), F64_TWO); }
fn df(x) { return f64_mul(F64_TWO, x); }
var root = alloc(8);
num_newton(&f, &df, F64_ONE, EPSILON_F64, 100, root);
# load64(root) ≈ 1.41421356...
```

## Building

```sh
cyrius build src/main.cyr build/hisab
cyrius test tests/hisab.tcyr        # 454 smoke tests
cyrius test tests/foundation.tcyr   # 379 foundation tests
cyrius test tests/modules.tcyr      # 2041 module tests
cyrius test tests/edge_cases.tcyr   # 233 edge case tests
cyrius test tests/abuse.tcyr        # 775 abuse tests (negative indices, zero/huge
                                    #   dimensions, non-conformable operands, canaries)
cyrius bench tests/hisab.bcyr       # 72 benchmarks
```

## Architecture

See [docs/architecture/overview.md](docs/architecture/overview.md) for the full module map, data flow, and design principles, and [docs/architecture/math.md](docs/architecture/math.md) for the equation reference (CGA operators, plus a catalogue index of the other formula families).

## Stats

| Metric | Value |
|--------|-------|
| Version | 2.22.0 |
| Library | 35 modules, ~26,004 lines of Cyrius |
| Tests | 4014 assertions across 5 suites |
| Benchmarks | 74 operations |
| Fuzz targets | 5 with invariant checks |
| CLI binary | ~251 KB static ELF (`build/hisab` — version smoke test only) |
| Toolchain | Cyrius 6.6.2 |
| Dependencies | 1 (sakshi 2.5.1); no third-party, no FFI/libc |
| Security | ⭐ **2.22.0 repaired a silent wrong answer at ORDINARY conditioning in TWO public entry points, filed for three releases as "subnormal SVD" and never a subnormal defect at all.** `_lp_bidiagonalize` gated its Householder reflectors on an **absolute 1e-12 applied to a length**, and when the gate fired the reflector was skipped while the extraction still read only the bidiagonal band — so the entries it would have zeroed were **silently dropped** and the routine returned the factorisation of a *different matrix* with `rc = HSB_ERR_NONE`. The boundary is a block ratio of **2^-40 — a condition number of ~1e12, 982 binades above where the roadmap placed it**. Measured oracle-free (`prod(S)` must equal `|det A|`): **25 of 60 normal-range ratios silently wrong → 0**, smallest singular value **9.89× too large → exact**. ⛔ `_lp_tridiagonalize` carries the identical gate, so **`eigen_qr` returned the smallest eigenvalue with the WRONG SIGN** — and the values it returned are exactly the spectrum of the matrix you get by dropping the entry it skipped, pinning the mechanism by the values themselves. ⛔ **Both entry points returned SUCCESS with a NaN payload** for non-finite input. ⛔ **The repair the roadmap proposed was at the wrong pipeline stage**, and the 2.19.0 refutation that blocked this release for three versions is TRUE: it converts 84 loud rows into silent wrong answers. ⚠ Seventh consecutive release where the row was wrong about scope — the grep found 5 sites in 3 functions under 3 public entry points and the row named none of them. ⭐ **2.21.0 moved CGA to the null basis {e1,e2,e3,n0,ninf}, and the repair was the BASIS rather than the arithmetic.** 2.20.0 had proved no arithmetic fix existed — at x = 2^-30 the correctly-rounded `ep` **is** exactly −1/2 — so a conformal point's `q = |p|²` now lives in **one** coefficient instead of being split across `(q∓1)/2`. Median `|P·P| / q` over 2000 full-mantissa points per binade: **1.0 → 4.48e-17 at 2^-30** and **1.0 → 4.33e-17 at 2^+30**, where **1.0 means the whole value is lost** — both tails go from total loss to sub-ulp, and the floor is flat rather than a U. ⛔ **"Exactly null" was the headline until a pre-tag audit priced it**: bit-exact nullity is reachable only by dropping the Neumaier compensation, which makes the norm repeat the constructor's own rounding — and then `1 + a·e1 + a·n0 + (a/2)·ninf` returns a scalar part of **0 where the answer is 1**, for 6 of 11 magnitudes. **An exact zero obtained by making the same rounding error twice is an artifact, not accuracy**; it was reverted, and no assertion in 3989 could tell the two trees apart until a witness was written. ⛔ **The first CGA call cost 2.7 ms and every benchmark was blind to it** because they all warm up — found because two probes of the same operation disagreed by 2.1× and the difference was a warm-up loop; repaired to 0.72 ms, and the product table is byte-identical across the rewrite because an FNV contract already pinned it. ⚠ The speedup is scoped: **2.22× on `point*point` and up to 10.9× SLOWER at 32 occupied blades**, crossover at k ≈ 8. ⛔ Building it found a cycc 6.6.2 **wrong-code bug** — `continue` at two nesting levels binds to the wrong loop — filed upstream, and the first version of that filing **described its own symptom wrongly** until the loop was instrumented. ⭐ **2.18.0 closed the SVD factors — an item the roadmap carried for three releases as "open-ended: a known defect with no known fix". The fix is one line, and 2.15.0 had created the defect**: `_lp_bidiagonalize` applies a Householder reflector when `vtv >= F64_TINY` but replayed it into `U` only when `vtv > EPSILON_F64`, so every reflector in between went into `B` and not `U` — `S` exact, `U` and `Vt` orthogonal, `U*S*Vt` not `A`. ⛔ **It also began by correcting 2.17.0**: that release's "472 of 999 → 0" came from a probe that shared its assertion's NaN hole (`0/0` is NaN, and every f64 comparison with NaN returns 0). Block ratios answered correctly: **256 (2.16.0) → 268 (2.17.0) → 999 (2.18.0)**. ⛔ **And the EPA seed trade, carried since 2.9.3, is DECLINED on a measurement that inverts its premise** — certifying is an early-out that skips `_epa_polish`, and the polish was delivering the accuracy: the "upgrade" is 36,000x less accurate against the exact sphere formula. ⭐ **2.17.0 closed the norm tier — 39 sites across 16 modules, 73 mutants, 67 killed.** Every `sqrt(sum of squares)` in the tree squared before summing, so a component below 2^-537 flushed to zero and one above 2^511 overflowed. ⛔ **In a solver that is not a wrong number but a confident success at the starting point**: 446 of 1001 objective scales for conjugate gradient and 461 of 1001 residual scales for Levenberg-Marquardt returned `HSB_ERR_NONE` with `x` untouched, and `cmat_inverse` called 498 of 1010 perfectly invertible matrices singular. ⛔ **Three of the repairs describe defects that 2.10.2, 2.11.1 and 2.14.0 each record as already fixed** — the thresholds had moved, the fabrications had not. ⚠ Its roadmap row named two sites; the grep found 51 and the census confirmed 19 more, the fifth release running that this class was wider than its list. P(-1) audit (2026-04-15) + hardening pass (2026-05-29); [2026-08-03](docs/audit/2026-08-03.md) sweep (70 findings, 2 critical) discharged in 2.6.12–2.6.15; [2026-08-04](docs/audit/2026-08-04.md) re-audit (42 confirmed, 0 critical) discharged in 2.7.0; [2026-08-04 v2.8.0 full sweep](docs/audit/2026-08-04-v2.8.0-full.md) (42 confirmed, 4 critical) **42 FIXED / 0 OPEN** as of 2.9.1; `tests/abuse.tcyr` (2.9.0) found 11 defects on public entry points, all fixed; [2026-08-11 v2.11.0 full sweep](docs/audit/2026-08-11-v2.11.0-full.md) (**52 reproduced, 21 confirmed, 2 refuted — and 28 reproduced but never verified, recorded as such**) with four repairs shipped in 2.11.1 and the rest scheduled on the roadmap. **0 open filings** in [docs/development/issues/](docs/development/issues/) and **28 archived** beside them — the EPA seed/certificate question, re-diagnosed in 2.9.3 and DECLINED on measurement in 2.18.0, was the last of them. ⚠ This line read "1 open filing, 26 archived, plus a README" until 2.21.0 checked it against the directory: all three numbers were wrong, and the README it named does not exist. **A count in prose is a measurement like any other.** ⚠ **2.11.4 closed both of the filings it had been carrying rather than deferring them**: `_ad_pow` was **repaired** (its `base > 0` delegation was discarding up to 174 ULP, and widening the loop exposed a fabricated 1.0 hiding behind a bound checked *after* a saturating `f64_to`), and the benchmark-floor filing was **REFUTED** — the ratio it rested on compared a per-op net against a per-clock-pair floor, and on a same-binary re-run the tier it dismissed is the quieter of the two, so it had been suppressing real −54% speedups. ⚠ **Cyrius bugs are filed in the cyrius repo**, not here; the one 2.11.3 filed there is **FIXED in cycc 6.6.2 and archived** — all 21 SIMD intrinsic handlers were binding an argument's frame local over their own destination slot. ⚠ hisab filed it as derive-specific and as a 6.5.71 regression and it was **neither**: 6.5.71 only exposed it, so the bisect found the release that made it visible, not the one that caused it. 2.11.3 also **closed two** older upstream filings by re-testing them on 6.6.1 — the CLI source-clobber data-loss risk is now guarded fail-closed, and a syntax error in an uncalled function is finally rejected by `lint`. ⭐ **2.16.0 closed the last deferral — and found a larger defect the guard had been hiding**: `su2_log` and `so3_log` recovered the angle with `acos` of a value that rounds to exactly 1.0 below θ ≈ 1.5e-8, so the entire rotation was lost (measured: exactly 0 from 2^-28 down). `atan2` recovers it bit-exactly, and the round-trip floor moves **2^-26 → 2^-537**. ⭐ **2.15.0 CLOSED that census: 73 of the remaining 74 repaired, 81 mutants installed and 73 killed, with 8 survivors and 1 deferral documented rather than tidied away.** ⛔ The deferral (`su2_log`, with `so3_log`/`se3_exp`/`se3_log`) is a formula change, not a threshold: those divide by θ² and θ³, and at θ ≤ 2.2e-162 both `θ*θ` and `1−cos θ` are exactly 0, so lowering the guard would make the coefficient `0/0 = NaN`. ⭐ Three of those four were never on the census list and were found by grepping for the shape. ⭐ **2.14.0's epsilon census is the largest single measurement in this repo's history**: every `EPSILON_F64` comparison guard in `src/` classified and then independently re-derived by a second agent instructed to refute the first — **136 guards, 97 confirmed defects across 23 modules**, against a roadmap row that said **"~20 sites"**. 23 repaired (58 mutants, all killed), **74 enumerated and open** in [`docs/audit/2026-09-09-epsilon-census.md`](docs/audit/2026-09-09-epsilon-census.md). ⚠ Its own first run **lost 30 of 41 agents to a session limit** and returned 11 results that read exactly like a complete answer. ⭐ **A 2026-09-09 verification sweep of the roadmap found 18 of 39 items stale** and retired the gate that had been blocking the whole release train — [`docs/audit/2026-09-09-roadmap-verification.md`](docs/audit/2026-09-09-roadmap-verification.md) |

## License

GPL-3.0-only — see [LICENSE](LICENSE).
