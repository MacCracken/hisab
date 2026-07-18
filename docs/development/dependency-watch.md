# Dependency Watch

Tracked dependency version constraints and upgrade paths.

## Cyrius Toolchain

**Status:** Pinned to **6.4.66** via `cyrius.cyml [package].cyrius` (legacy `.cyrius-toolchain` removed; CI/release grep the manifest directly).

**Note:** Cyrius stdlib provides dense LU, Cholesky, QR, SVD, eigendecomposition. As of 6.2.x these live in the new **`ganita`** umbrella module (which re-exports the former `matrix`/`linalg` API in full and also hosts the transcendentals). This is a critical dependency — hisab's `linalg_ext.cyr` wraps these functions. The `[deps] stdlib` list pulls `ganita` (not `matrix`/`linalg` — listing those alongside `ganita` collides).

**Upstream notes (5.x line):**
- 5.0+: `lib/matrix.cyr` overflow class addressed; SVD precision improvements landed.
- 5.5.x: structured-deps protocol, `cyrius distlib` multi-profile, `#deprecated("...")` attribute, `cyrius vet`/`cyrius lint`/`cyrius fmt` matured.
- 5.7.x: cyrius-ts JSX AST (not consumed here), fixup-table cap 262K → 1M, `cyrius build` atomic-output (failed compile no longer destroys an existing binary).
- 5.7.8: `syscall arity mismatch` warning fixed at the cc5 level; `cyrius deps` writes `cyrius.lock` by default; `cyrius check` no longer auto-prepends manifest deps; `cyrius build --no-deps` flag added.
- 5.7.9: `warning: duplicate fn '<name>' (last definition wins)` at registration time. Hisab build emits zero such warnings. `json_build` cross-module collision resolved upstream via patra rename.
- **5.7.10**: `input_buf` **512 KB → 1 MB** heap-map reshuffle (+0x100000 region shift across 95 distinct heap addresses). Hisab was the load-bearing reason — `dist/hisab.cyr` was at 96 % of the old cap. Unblocked the full 34-module bundle.
- 5.8.65: sakshi (+ patra/sigil/vani/yukti/sankoch) folded byte-identical into the compiler stdlib — `[deps.<name>]` git blocks no longer required for them.
- **6.0.0**: `cc5`→`cycc` / `cyrc`→`cybs` binary rename (transparent to consumers; `cyrius build` dispatches, back-compat symlinks ship through 6.0.x). No source-syntax breaks for a pure math library.
- 6.0.2: lockfile/vendoring fix — `cyrius deps` now hashes all `.cyr` under `lib/` and writes a real lock (the empty 0-byte `cyrius.lock` bug present since 5.11.8); vendored deps are regular file-copies, not the dangling symlinks that broke CI.
- **6.0.14**: clean build/test (901/901 as of v2.4.6). Migration was manifest-only (pin bump + sakshi resolution); the 34 math modules moved `lib/`→`src/` so the committed `lib/` no longer shadows the toolchain's version-pinned stdlib snapshot.
- **6.2.11** (v2.6.6): stdlib math reorg. The transcendentals (`f64_acos`/`f64_asin`/`f64_atan2`/`f64_pow`/`f64_sinh`/`f64_cosh`/`f64_tanh` + hyperbolic inverses) moved out of `math` into the new **`ganita`** module, which also subsumes `matrix`/`linalg` (re-exports their full API). `math` now ships NaN-correct `f64_le`/`f64_ge` (hisab dropped its local copies). `[deps] stdlib`: `+ganita`, `−matrix`, `−linalg`. Clean build, 957/957 tests, all gates green. Tracked-issue re-verify: **3 of 5 fixed** (modules-substring, 18-arg-fn scramble, lint rc-as-count → all archived); for-empty-clauses still open. Vendored `lib/` re-resolved via `cyrius deps` (30 files — **not** the full-snapshot `cyrius lib sync`, which over-vendors unused platform variants and breaks `deps --verify` on a spurious `process_agnos.cyr` entry); `cyrius.lock` 30 deps, verify 30/30.
- **6.4.66** (current pin, v2.6.9): bump from 6.3.11 (a full minor across 55 patch releases). **No library source change** — all 34 modules compile clean; `dist/hisab.cyr` byte-identical apart from the version header; a consumer including the full bundle compiles + runs end-to-end. Re-vendored via `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.4.66; the transitive `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed this bump, unlike 6.3.11). `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. The 6.4.66 stdlib snapshot is much larger than 6.3.11's (new `async`/`tls`/`sandhi`/`regex`/`http`/… modules) but the declared subset hisab pulls is unchanged. **Fixed** a pre-existing `tests/modules.tcyr` compile failure surfaced (not caused) by this bump: a cycc identifier-lexer bug where `iv_add`/`iv_sub`/`iv_mul` lex as `unknown` once `interval.cyr` is in the unit (reproduces on 6.3.11 too) — renamed to `iv_sum`/`iv_diff`/`iv_prod`, suite restored to 312/312, full run 957/957. New tracked issue: `issues/2026-07-17-cyrius-interval-ident-lex.md`. Tracked-issue re-verify on the new pin: for-empty-clauses **still open** (unchanged rejections on 6.4.66).
- **6.3.11** (v2.6.7): infrastructure-only bump from 6.2.11. **No library source change** — all 34 modules compile clean; `dist/hisab.cyr` byte-identical apart from the version header. Stdlib delta touched `assert`/`bench`/`fnptr`/`io`/`math` + the `syscalls` platform variants (`ganita` unchanged); `lib/result.cyr` (transitive dep of `io`/`tagged`) picked up the 6.3.11 `_die` agnos-portability fix (was a bare `syscall(60,1)` that no-op'd → failed-open on agnos; now target-guarded). 6.3.x CLI split: **`cyrius deps`** resolves git deps only (commit-pins sakshi in the lock), **`cyrius lib sync`** (no `--full`) vendors the declared stdlib subset — superseding the 6.2.x `cyrius deps`-does-both flow. Every vendored stdlib file byte-matches 6.3.11; `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. 957/957 tests, all gates green. Tracked-issue re-verify: for-empty-clauses **still open** on 6.3.11; no new fixes (3 prior fixes stay archived).

**Watching upstream:**
- **5.7.11** — RISC-V rv64.

## Cyrius stdlib modules (22 used)

| Module | Purpose | Risk |
|--------|---------|------|
| alloc | Bump allocator | Foundation — cannot change |
| string, str | C strings, fat strings | Stable |
| fmt | Formatting | Stable |
| vec | Dynamic array | Stable |
| math | f64 inclusive cmp (`f64_le`/`f64_ge`), clamp/lerp/min/max/sign/trunc, exp/ln polyfills, gcd/lcm | Stable |
| ganita | 6.2.x math umbrella: transcendentals (sinh, pow, atan2, …) + dense matrix storage + decompositions (LU, QR, SVD, eigen). Subsumes the former `matrix`/`linalg` | New in 6.2.x — replaces `matrix`+`linalg` |
| tagged | Option/Result types | Stable |
| fnptr | Function pointer calls | Stable |
| syscalls, io, args | System interface | Stable |
| assert, bench | Test/benchmark framework | Stable |
| callback | Higher-order functions | Stable |

## sakshi (first-party dependency)

**Status:** `sakshi` **2.4.6** via git, modules path `dist/sakshi.cyr` (bumped 2.4.2 → 2.4.6 in v2.6.9; commit-pinned in `cyrius.lock`). 2.4.6 pins cyrius 6.4.49 (≤ our 6.4.66); its shipped surface links only `fnptr` + `atomic` and is unchanged, so behaviour is identical.

**Purpose:** Structured logging (timestamps, levels, categories)

**Risk:** Low. Logging is write-only — no data flows back. If sakshi breaks, hisab still compiles (just without logging).

## Rust-era dependencies (archived in pre-2.0 git tags)

These are no longer used but documented for reference:
- `glam` 0.29 — replaced by hisab's own vec/mat/quat types
- `serde` 1.0 — no serialization in Cyrius port (yet)
- `thiserror` 2.0 — replaced by ERR_* integer codes
- `tracing` 0.1 — replaced by sakshi
- `reqwest` 0.12, `tokio` 1.0 — AI module not ported
- `rayon` 1.0 — parallel module not ported
- `criterion` 0.5 — replaced by bench.cyr
