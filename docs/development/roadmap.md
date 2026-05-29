# Roadmap

> **Hisab** (Arabic: حساب -- calculation) -- higher mathematics library for the AGNOS ecosystem.
> Written in Cyrius. Toolchain: **6.0.14**. Stdlib `linalg.cyr` provides dense decompositions.

## Scope

Hisab owns **typed mathematical operations**. It does NOT own:
- **Expression parsing** -- abaco
- **Unit conversion** -- abaco
- **Physics simulation** -- impetus
- **Game engine** -- kiran

## Current -- v2.3.1

- **34 math modules in `src/`, 16,195 lines** (`lib/` is vendored-only)
- **825 test assertions**, 28 benchmarks (incl. amplified SIMD batches), fuzz harness
- **CLI smoke binary** ~152 KB static ELF
- **`dist/hisab.cyr` distlib bundle** ~540 KB / 16,195 lines (all **34 modules**) — fits cycc 6.0.14's 1 MB input_buf with ~480 KB headroom
- Toolchain **6.0.14**; CI fmt/lint/vet/security all green
- P(-1) audit: 26/31 fixed

### Shipped in 2.3.0–2.3.1
- **2.3.0** — toolchain 5.7.10 → **6.0.14**; library source moved `lib/*.cyr` → `src/*.cyr`; sakshi resolution repaired; tree reformatted under 6.0.x `cyrfmt`; CI aligned to abaco (fmt gate, security scan, version-template gate)
- **2.3.1** — SIMD hot paths via `f64v_*`: vec4 dot 6.5×, m4_mul 4.5×, m3_mul 3.2×, vec3 dot 2.2× (825/825 bit-identical)

---

## 2.3.x -- Optimization & Modernization Arc

Now that the toolchain is on 6.0.14, fold in the language + stdlib features
that landed across 5.8–6.0 to make hot paths faster and the code more
idiomatic — **without changing the public API**, so these ship as patches.
Principles hold: every perf claim is benchmark-gated against
`bench-history.csv`; every behavioral-adjacent change adds tests *first*;
large items get broken into small, individually-verified bites.

> Patch numbers below are thematic groupings, not hard commitments — reorder
> by leverage as benchmarks dictate. The **breaking** error-handling migration
> (`Result<T,E>` + `?`) is intentionally *not* here — see 3.0.0.

### 2.3.1 — SIMD hot paths (keystone) ✅ shipped
Cyrius's `f64v_*` packed-double builtins (v5.10.16) were added with hisab
named as the gap-close consumer — the single biggest perf lever available.
- [x] vec3/vec4 dot · add · sub · scale via `f64v_dot`/`f64v_add`/`f64v_sub`/`f64v_scale` (vec4 n=4; vec3 n=2+scalar-z). `length`/`normalize` inherit via `dot`
- [x] mat3/mat4 multiply + vec transform via column-major linear combination (`f64v_scale`+`f64v_add`)
- [x] quat `dot`/`normalize` via packed ops (Hamilton product stays scalar — not elementwise)
- [x] **Guarded the odd-`n` over-read** — vec3/mat3 (24-B = 3 doubles) use n=2 + scalar tail; vec4/mat4/quat use n=4 (even, safe)
- [x] Correctness verified against the 825-assertion suite (bit-identical to scalar) + amplified equivalence probes
- [x] Benchmarked before/after; amplified batch benches added to `bench-history.csv` (single-op benches sit below the harness's ~488 ns floor)
- Wins: vec4 dot 6.5×, m4_mul 4.5×, m3_mul 3.2×, vec3 dot 2.2× (see CHANGELOG 2.3.1)
- Deferred: cross/lerp (need shuffles, not a clean f64v fit); `f64v2`/`f64v4` value-form (pointer-form via heap vecs was sufficient)

### 2.3.2 — Scalar ops + scratch allocators
- [ ] Replace hand-rolled scalar transcendentals with `f64_sin/cos/exp/ln`; use `f64_abs` (v5.11.37–.40, peephole-optimized to 2 instructions)
- [ ] Adopt stdlib `arena_*` (v5.11.14: `arena_new/alloc/reset/free`) for per-call scratch in tensor / einsum / symbolic instead of bump-and-leak — `arena_reset` for per-frame reuse
- [ ] Benchmark allocator-heavy paths (einsum contraction, symbolic simplify)

### 2.3.3 — Safety & numerical-correctness audit (mined from vidya gotchas)
- [ ] Audit every `var buf[N]` in a helper that returns a buffer-backed handle — these are **process-static** and silently corrupt across calls; switch to per-call `alloc()` (compiler now warns on static data > 4K)
- [ ] Audit `var name[N]` sizing — `[N]` is **bytes, not elements**; vector/matrix lane buffers need `[N*8]`
- [ ] Review signed bit-math: `>>` is logical (zero-fill), there is no unary `~` (use `x ^ -1`); add an `asr()` helper where arithmetic shift is intended
- [ ] Apply overflow-explicit operators (`+%`/`+|`/`+?` from `lib/overflow.cyr`) in numerically-careful kernels (factorial, binomial, Pollard-rho factorization) for intent + checked-panic safety
- [ ] Property/fuzz coverage for the fixed-point and signed paths

### 2.3.4 — Layout & idiom modernization
- [ ] Replace duplicated literal buffer sizes with enum-const array sizes (v5.10.48: `var buf[ENUM_CONST]` now parses)
- [ ] Where manual `load64`/`store64` + named-offset blocks exist, evaluate typed structs + `#derive(accessors)` + `sizeof(T)` (keeps accessor fn names stable, kills hand-computed offsets)
- [ ] Use slices (`[T]` / `slice<T>`) for bounds-checked array views; `slice_unchecked_get_W` in proven-hot inner loops
- [ ] `defer` for alloc/fd cleanup in the few resource-holding paths
- [ ] Annotate pure computational fns with `#must_use` / `#pure` (warn-only; hardens the API surface, documents purity)

---

## 2.4.0 -- Collision module audit (algorithmic correctness)

`collision_core.cyr` and `collision_mesh.cyr` compile and link, but the
algorithms carry pre-existing bugs from the 2.2.0 port — they were added
then but never exercised (they sat outside the build chain via the old
orphan-include-after-syscall trick). 2.2.2 only smoke-tests the API surface
(`contact_new` + `ColContact_*` accessors, `detect_islands`); the heavy
algorithms need a correctness pass.

- [ ] **`convex_hull_2d`** — `vec: index < 0` runtime bounds check on a 5-point input (square + interior point). Insertion-sort + monotone-chain logic needs review
- [ ] **`triangulate_polygon`** (ear clipping) — likely similar boundary issues; not exercised yet
- [ ] **`mpr_intersect`** + **`mpr_penetration`** — XenoCollide / Minkowski Portal Refinement in 3D; correctness against known test fixtures (sphere-sphere, OBB-OBB)
- [ ] **`sequential_impulse`** + **`solve_pgs`** — projected Gauss-Seidel solver for contact constraints; verify convergence + restitution behavior
- [ ] **`delaunay_2d`** (Bowyer-Watson) — needs a numerical-stability pass alongside basic correctness fixtures
- [ ] **`halfedge_from_triangles`** + `halfedge_adjacent_faces` + `halfedge_is_boundary` — half-edge mesh accessors; check twin-pointer wiring
- [ ] Add coverage to `tests/modules.tcyr` as each algorithm is fixed

---

## 2.5.0 -- CGA + matrix overflow guards

- [ ] CGA left/right contraction operators
- [ ] CGA dual operation, blade projection/rejection
- [ ] Confirm `matrix.cyr` `mat_new(rows, cols)` overflow is fixed upstream (was C3 in P(-1) audit; expected fixed by cyrius 5.x/6.x — verify with a regression test)

---

## 2.6.0 -- Differential geometry depth

- [ ] Parallel transport of vector fields along curves
- [ ] Sectional curvature computation
- [ ] Geodesic deviation equation
- [ ] Weyl tensor (conformal curvature)
- [ ] Higher-order differential forms (3-forms, 4-forms)

---

## 2.7.0 -- Rendering, GPU, reverse-mode AD

- [ ] Differentiable rendering math (autodiff through ray-surface intersections)
- [ ] Reverse-mode autodiff (Tape-based)
- [ ] GPU compute via soorat (feature-gated)

---

## 3.0.0 -- Error-handling migration (breaking)

The integer-error-code convention (`lib/error.cyr`: functions return 0 / a
negative `ERR_*` code) predates the stdlib `Result<T,E>` (`lib/result.cyr`,
v5.8.28) and `?` propagation (v5.8.29). Migrating is a library-wide signature
change — breaking for consumers (impetus, kiran, joshua, …) — so it lands as
a major, with a migration guide, not a 2.3.x patch.

- [ ] Wrap fallible returns in `Result<T,E>` (keep `ERR_*` codes as the `E` payload)
- [ ] Adopt `?` to replace manual `-1`-return + check chains
- [ ] Migration guide + deprecation window for the old integer-code API

---

## Consumers

| Consumer | Status |
|----------|--------|
| **impetus** (physics) | Usable -- GJK/EPA, PGS, inertia, spatial |
| **kiran** (engine) | Usable -- projections, BVH, k-d tree, frustum |
| **joshua** (simulation) | Usable -- DOPRI45, BDF, symplectic, optimize |
| **aethersafha** (compositor) | Usable -- projections, compositing, color |
| **abaco** (expression eval) | Usable -- symbolic integrate/LaTeX/patterns, interval |
| **svara** (vocal synthesis) | Usable -- complex, FFT, easing |
| **hisab-mimamsa** (physics) | Usable -- tensors, Lie groups, diffgeo, CGA |
| **kana** (quantum) | Usable -- tensors, Lie groups, complex LA, spinors |

---

## Release History

| Version | Date | Lines | Files | Highlights |
|---------|------|-------|-------|-----------|
| 2.3.1 | 2026-05-28 | 16,195 | 34 | SIMD hot paths (`f64v_*`) for vec/mat/quat — vec4 dot 6.5×, m4_mul 4.5×, m3_mul 3.2×. Bit-identical, 825/825 |
| 2.3.0 | 2026-05-28 | 16,195 | 34 | Cyrius 6.0.14 toolchain; library source moved to `src/`; sakshi resolution repaired; CI aligned to abaco (fmt/security/version gates). No behavioral change |
| 2.2.0 | 2026-04-15 | 15,676 | 33 | SE(3), SO(3), adjoint, BCH, spatial structures, MPR, impulse solver, simplex noise, einsum, Golub-Kahan SVD |
| 2.1.0 | 2026-04-15 | 13,715 | 30 | Golub-Kahan SVD, QR eigen, complex QR, simplex noise, einsum |
| 2.0.0 | 2026-04-15 | 11,943 | 27 | Cyrius port from Rust. P(-1) audit. |
| Rust 1.4.0 | 2026-03-30 | 33,612 | 65 | Final Rust release. Available via pre-2.0 git tags. |

---

## Boundary with Abaco

| Feature | abaco | hisab |
|---------|-------|-------|
| `eval("sin(pi/4)")` | parses and evaluates | -- |
| `hvec3_cross(a, b)` | -- | vec3.cyr |
| `geo_ray_sphere(ray, sphere)` | -- | geo.cyr |
| `calc_integral_simpson(&f, a, b, n, out)` | -- | calc.cyr |
| `num_newton(&f, &df, x0, tol, max, out)` | -- | num.cyr |
| `sym_integrate(expr, var)` | -- | symbolic_ext.cyr |
| `sym_to_latex(expr)` | -- | symbolic_ext.cyr |

Hisab should never depend on abaco. Abaco may optionally depend on hisab.
