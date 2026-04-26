# Changelog

## [Unreleased]

## [2.2.2] - 2026-04-26 — Make it actually build under cc5 5.7.8 + distlib bundle

2.2.1 captured the manifest/CI/release modernization intent but was never
tagged because `cyrius build src/main.cyr` didn't pass — Cyrius 5.7.x's
new reserved keywords + an oversized include chain (the CLI prepended all
33 project modules, exceeding cc5's 512 KB input_buf) blocked the build.
2.2.2 closes those, picks up the 5.7.8 toolchain bump for its `syscall
arity mismatch` warning fix + lockfile-default behavior, ships a 32-module
`dist/hisab.cyr` distlib bundle (~505 KB, fits cc5 5.7.8's input_buf and
verified end-to-end against a consumer build), and gets all gates green.

### Fixed
- **`lib/num_ext.cyr`**: renamed local variable `stack` → `stk` (6 identifier sites in `_factorize_pollard_rho`). `stack` became a reserved keyword in Cyrius 5.7.x; the four mentions in comments were left intact
- **`src/main.cyr`**: stripped from 30+ project-module includes down to the two stdlib includes its `fn main()` actually uses (`syscalls`, `io`). The previous form prepended every project module just to "validate the include chain" — but cc5 5.7.7's 512 KB input_buf can't fit that, and the test suites already cover include integration. Bonus: fixed three orphan `include` lines that sat *after* `syscall(SYS_EXIT, r)` (parsed but unreachable; first time any of them was scrutinized was when one tripped a parse error). CLI binary: now ~140 KB static ELF, prints the version string and exits
- **`tests/modules.tcyr`**: stripped six "multiple consecutive blank lines" lint warnings (lines 43-45, 263-264, 390 in the pre-fix file). `cyrius lint` returns the warning count as its exit code, which the prior CI loop swallowed under GHA's `set -eo pipefail` — the loop would abort on the first non-zero rc without reporting which file tripped it
- **`examples/basic_math.cyr` + `tests/{edge_cases,foundation,hisab,modules}.tcyr`**: applied `cyrius fmt` to flatten multi-line continuation-indent drift from the modern formatter
- **CI lint loop**: added `set +e` and per-file rc capture so warning-count exit codes don't abort the sweep before later files report. Adds `::error file=...::` annotations so the offending file is visible in the GHA UI

### Added
- **`dist/hisab.cyr`** — 32-module distlib bundle (~505 KB), regenerated from `[lib]` via `cyrius distlib`. Consumers pull it as `[deps.hisab] modules = ["dist/hisab.cyr"]` — single self-contained file, no per-module `include` choreography. Verified end-to-end: a fresh `[deps] stdlib = [...]` consumer build of a `hvec3_dot` example exits 0 and prints expected output. Fits cc5 5.7.8's 512 KB input_buf with ~7 KB headroom; restoring `lib/collision_core.cyr` + `lib/collision_mesh.cyr` (~33 KB) is gated on cc5 5.7.9's input_buf bump to 1 MB
- **CI distlib drift gate** in `ci.yml` — `cyrius distlib` runs on every push and CI fails if the committed `dist/hisab.cyr` differs from what the current `lib/` produces, so consumers always pull a fresh bundle that matches `lib/`
- **Release distlib regeneration** in `release.yml` — bundle is regenerated and shipped as `hisab-<TAG>.cyr` alongside the source tarball + linux binary + `SHA256SUMS`

### Changed
- **Toolchain pin**: 5.7.7 → **5.7.8**. Picks up the cc5-level fix for the noisy `lib/syscalls_x86_64_linux.cyr:358: syscall arity mismatch` warning (was firing on every build of every downstream that includes syscalls), `cyrius deps` writing `cyrius.lock` by default, `cyrius check` no longer auto-prepending manifest deps, and the new `cyrius build --no-deps` flag
- **Distlib gates removed from CI/release** — the regenerate-and-diff step in `ci.yml` and the bundle archive step in `release.yml` are gone for 2.2.2. The release still ships the source tarball + linux binary + `SHA256SUMS`
- **`README.md` quick-start**: `[deps.hisab] modules = ["dist/hisab.cyr"]` → per-module `["lib/<file>.cyr", ...]` example; toolchain version → 5.7.7; CLI binary description corrected (it's a smoke test, not a library entry point)
- **`CLAUDE.md` status + layout**: reflects that the CLI is a smoke binary and library coverage runs through `cyrius test`; CI/Release section drops the distlib gate and adds the deferral note

### Local verification
All gates pass locally on this branch:
- `cyrius lint` — clean across 8 files (4 tcyr, 1 bcyr, 1 fcyr, 1 example, src/main.cyr)
- `cyrius fmt --check` — no drift
- `cyrius vet src/main.cyr` — clean
- `cyrius build src/main.cyr build/hisab` — OK, 143 KB static ELF, magic verified
- `./build/hisab` — prints `hisab 2.2.2`, exit 0
- `cyrius test tests/hisab.tcyr` — 116/116
- `cyrius test tests/foundation.tcyr` — 307/307
- `cyrius test tests/modules.tcyr` — 249/249
- `cyrius test tests/edge_cases.tcyr` — 149/149
- Total: **821/821 assertions, 0 failed**
- `cyrius build tests/hisab.fcyr` + 5 s timeout run — fuzz: ok
- `cyrius bench tests/hisab.bcyr` — 22 benchmarks complete

### Notes
- `lib/collision_core.cyr` and `lib/collision_mesh.cyr` are present in `lib/` but excluded from the `[lib]` distlib bundle — both have pre-existing parse issues against cc5 5.7.x (originally surfaced because the prior `src/main.cyr` had three orphan `include` lines after `syscall(SYS_EXIT)`, which the parser still walked). Combined size ~33 KB, which would push the bundle past cc5 5.7.8's 512 KB input_buf anyway. Restoration is gated on cc5 5.7.9's input_buf bump to 1 MB — see roadmap
- `lib/` still mixes vendored stdlib + project source — a yukti-style split (project source in `src/*.cyr`, `lib/` purely deps + gitignored) is a natural future restructure but out of 2.2.2's scope

## [2.2.1] - 2026-04-26 — Cyrius 5.7.7 modernization + distlib bundle

Toolchain catch-up release: spans the full 5.x line in one bite, lands the
distlib-driven distribution model, and brings CI / release / scripts into
line with first-party Cyrius project conventions (yukti / sakshi / patra).
No source changes to the math modules themselves — pure scaffold rework.

### Changed
- **Toolchain bump**: Cyrius **4.10.3 → 5.7.7**. Picks up the manifest format, `cyrius distlib` multi-profile, `cyrius.lock`, `${file:VERSION}` interpolation, `#deprecated("...")` attribute, structured-deps protocol, fixup-table cap 262K → 1M, atomic `cyrius build` output (failed compile no longer destroys an existing binary)
- **Manifest migration**: `cyrius.toml` → **`cyrius.cyml`**. Adds `language = "cyrius"`, `cyrius = "5.7.7"` toolchain pin in `[package]`, and `version = "${file:VERSION}"` interpolation so VERSION is the single source of truth
- **Toolchain pin location**: `.cyrius-toolchain` (legacy) removed — pin lives in `cyrius.cyml [package].cyrius` per Cyrius 5.5.41+ convention; CI/release grep the manifest directly
- **Dep bump**: sakshi **0.9.0 → 2.1.0** (modules path now `dist/sakshi.cyr` per the distlib convention)
- **`scripts/version-bump.sh`**: was editing `Cargo.toml` + running `cargo generate-lockfile` — now just writes `VERSION` (manifest auto-syncs via `${file:VERSION}`); validates semver shape; prints next-step hints for CHANGELOG section + tag
- **`scripts/bench-history.sh`**: was running `cargo bench --bench benchmarks` and parsing criterion output — now runs `cyrius bench tests/hisab.bcyr` and parses `lib/bench.cyr` output; tolerant unit normalization (ps/ns/µs/ms/s); 3-point trend table generation unchanged
- **`CLAUDE.md`**: full rewrite for the actual Cyrius project — replaced Rust idioms (cargo/clippy/RUSTDOCFLAGS, MSRV 1.89, `#[non_exhaustive]`) with the real toolchain (`cyrius lint/fmt/vet/build/test/bench/distlib`); refreshed layout, deps, key principles, CI gates, doc structure
- **`CONTRIBUTING.md`**: prereq updated to Cyrius 5.7.7+ via `cyrius.cyml`
- **`README.md`** quick-start: `cyrius.toml` snippet → `cyrius.cyml` with full `[package]` + first-party `[deps.hisab]` example pointing at `dist/hisab.cyr`
- **`.gitignore`**: dropped Rust-only entries (criterion, proptest, tarpaulin, cargo-vet); added `cyrius-*.tar.gz` (CI download) and `.claude/`
- **`docs/development/dependency-watch.md`**: toolchain status updated; sakshi reclassified as first-party; 5.x line notes added

### Added
- **`[lib]` modules block** in `cyrius.cyml` listing 34 hisab modules in dependency order (foundation → types → derived) — input to `cyrius distlib`
- **`dist/hisab.cyr`** distribution bundle (regenerated from `[lib]` via `cyrius distlib`). Consumers pull it with `[deps.hisab] modules = ["dist/hisab.cyr"]` — single self-contained module, no per-file `include` choreography needed
- **`.github/workflows/ci.yml`** rebuilt: lint, fmt-check, vet, **distlib drift check** (regenerates `dist/hisab.cyr` and fails if committed bundle is stale), build (with ELF magic check), test (auto-discover `tests/*.tcyr`), fuzz (auto-discover `tests/*.fcyr` with 10s timeout per harness), bench (auto-discover `tests/*.bcyr`), docs + version-consistency gate
- **`.github/workflows/release.yml`** rebuilt: CI gate, version-verify (tag matches VERSION; supports both `1.2.3` and `v1.2.3`), distlib regeneration, source tarball + `dist/hisab.cyr` + linux binary + `SHA256SUMS` upload, changelog-section extract for release body
- **`cyrius deps --verify`** lock-gate in CI (gated on `cyrius.lock` presence)

### Removed
- `.cyrius-toolchain` — superseded by `cyrius.cyml [package].cyrius`
- `cyrius.toml` — superseded by `cyrius.cyml`

### Notes
- `lib/collision_mesh.cyr` is included in the `[lib]` bundle. The 2.2.0 changelog flagged it as "exceeds cc3 1MB preprocess buffer, ships with Cyrius 5.0"; cc5 5.7.7 has lifted the relevant caps. If a downstream `cyrius build` of `dist/hisab.cyr` fails on this file, drop it from the `[lib]` list and reopen the deferral
- `lib/` still mixes vendored stdlib + project source. yukti-style separation (project source in `src/*.cyr`, `lib/` purely deps + gitignored) remains a future restructuring decision

## 2.2.0 (2026-04-15) -- Geometry & group extensions

### Added
- **lie_ext.cyr** (523 lines) -- SE(3) rigid body motions (exp/log, compose, transform), SO(3) explicit (Rodrigues, exp/log), adjoint representations (SU(2), Lorentz, SE(3)), Baker-Campbell-Hausdorff 2nd/3rd order
- **spatial.cyr** (864 lines) -- k-d tree (build, nearest, within_radius), quadtree (insert, query), octree (insert, query), spatial hash (insert, query_cell, query_radius, clear)
- **collision_core.cyr** (574 lines) -- MPR/XenoCollide (intersect + penetration), sequential impulse solver with friction, convex hull 2D (Andrew's monotone chain), polygon triangulation (ear clipping)
- **collision_mesh.cyr** (522 lines, written but deferred) -- Delaunay triangulation (Bowyer-Watson), half-edge mesh, island detection (union-find). Exceeds cc3 1MB preprocess buffer. Ships with Cyrius 5.0.

### Changed
- All .cyr files cleaned of Unicode in comments (em-dashes, Greek letters -> ASCII)

### Known issue
- Cyrius cc3 1MB preprocess buffer limit reached at ~16K lines. collision_mesh.cyr deferred to Cyrius 5.0.

## 2.1.0 (2026-04-15) -- Precision + depth

### Added
- **linalg_precision.cyr** (1,124 lines) -- Golub-Kahan SVD (full precision, replaces A^T*A), QR eigendecomposition O(n^3) (replaces Jacobi O(n^5)), complex Householder QR
- **noise_simplex.cyr** (343 lines) -- OpenSimplex2 2D+3D with fBm layering
- **einsum.cyr** (305 lines) -- Einstein summation notation parser (`"ij,jk->ik"`)

### Changed
- Audit M8 (SVD precision) resolved

### Performance
- `eigen_qr`: O(n^3) for symmetric eigenproblems (was O(n^5) with Jacobi for large n)

## 2.0.0 (2026-04-15) -- Cyrius port

**Breaking: complete rewrite from Rust to Cyrius.** New language, new API, new binary format. Rust source available via pre-2.0 git tags.

### Changed
- **Language**: Rust -> Cyrius (self-hosting systems language, static ELF binaries)
- **Types**: glam f32 SIMD types -> native f64 heap-allocated types (HVec2/3/4, HQuat, Mat3, Mat4)
- **Errors**: `Result<T, HisabError>` -> integer error codes (ERR_NONE, ERR_SINGULAR_MATRIX, etc.)
- **API**: method syntax (v.dot(w)) -> free functions (hvec3_dot(v, w))
- **Precision**: f32 (1e-7) -> f64 (1e-12) everywhere
- **Dependencies**: 9 Rust crates -> 1 Cyrius dep (sakshi)
- **Binary**: ~800KB dynamic -> 420KB static ELF

### Added -- 27 library files, 11,943 lines

**Foundation (8 files):** error, f64_util, vec2, vec3, vec4, quat, mat3, mat4
**Transforms (2 files):** transforms (T2D/T3D, Euler, screen), color (sRGB, Porter-Duff, tone mapping, SH, EV)
**Geometry (2 files):** geo (9 primitives, 6 ray tests), geo_advanced (GJK/EPA, BVH, SDF, CGA 5D)
**Calculus (2 files):** calc (integration, Bezier, easing, Perlin), calc_ext (gradient/Jacobian/Hessian, B-spline, NURBS, Hermite, monotone cubic, 3D Perlin)
**Numerical (5 files):** num (roots, FFT, RK4, PCG32, primes), ode (DOPRI45, BDF, symplectic), optimize (GD, CG, BFGS, L-BFGS, LM), linalg_ext (CSR, GMRES, BiCGSTAB, PGS, SVD, eigen, inertia), num_ext (extended GCD, totient, Mobius, factorize, CRT, DST/DCT, 2D-FFT, Halton/Sobol, tridiagonal)
**Physics (3 files):** complex (numbers + matrices, Pauli, Dirac, matrix exp), lie (U(1), SU(2), SU(3), SO(3,1)), diffgeo (Christoffel -> Einstein, geodesics, exterior algebra)
**Symbolic (2 files):** symbolic (expr tree, eval, diff, simplify), symbolic_ext (integration, LaTeX, pattern matching)
**Other (3 files):** autodiff (dual numbers), interval (arithmetic), tensor (N-D dense, contraction, physics tensors)

### Security -- P(-1) audit (31 issues found, 25 fixed)
- Allocation overflow guards (tensor, complex matrix, diffgeo dim cap, sieve cap)
- Division-by-zero guards throughout (complex, autodiff, transforms, depth)
- m4_determinant rewritten with correct cofactor formula
- tensor_contract implemented (was returning zeros)
- BDF-5 coefficients recomputed exact (IEEE 754 verified)
- Bisection midpoint overflow fix, CG upgraded to Polak-Ribiere+
- modpow overflow-safe via Russian peasant multiplication
- expr_eval warns instead of aborting process

### Testing
- 821 assertions (4 suites), 22 benchmarks, 5 fuzz targets

### Documentation
- README, CONTRIBUTING, SECURITY updated for Cyrius
- Architecture overview, testing guide, threat model, dependency watch
- P(-1) audit report, Rust vs Cyrius benchmark comparison
- Working example (examples/basic_math.cyr)

---

## Rust era (archived in pre-2.0 git tags)

### 1.4.0 (2026-03-30) -- Theoretical physics foundation
- Complex linear algebra (ComplexMatrix, Hermitian eigen, complex SVD, Pauli/Dirac, spinors, matrix exp)
- Indexed tensor algebra (Einstein summation, contraction, raising/lowering)
- Symmetric/antisymmetric/sparse tensors
- Lie groups (U(1), SU(2), SU(3), SO(3,1), exponential maps, Casimir)
- Differential geometry (Christoffel, Riemann, Ricci, Einstein, geodesics, Killing, exterior algebra)
- Conformal geometric algebra (5D CGA multivectors)

### 1.3.0 (2026-03-27) -- Number theory + symbolic
- Prime sieves, primality, factorization, modular arithmetic
- Symbolic integration, LaTeX, pattern matching, abaco bridge

### 1.2.0 (2026-03-27) -- Interpolation + color
- Inverse lerp, remap, reverse-Z, HSV/HSL/Oklab, Porter-Duff, compensated summation

### 1.1.0 (2026-03-25) -- Feature completion
- Symplectic integrators, SDFs, DualQuat, SH, stiff ODE, SDE, eigen, reverse-mode AD

### 1.0.0 -- Stable release
- Core: transforms, geo, calc, num, autodiff, interval, symbolic, tensor, parallel, ai, logging
