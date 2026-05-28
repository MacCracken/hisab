# Dependency Watch

Tracked dependency version constraints and upgrade paths.

## Cyrius Toolchain

**Status:** Pinned to **6.0.14** via `cyrius.cyml [package].cyrius` (legacy `.cyrius-toolchain` removed; CI/release grep the manifest directly).

**Note:** Cyrius stdlib provides `lib/linalg.cyr` with LU, Cholesky, QR, SVD, eigendecomposition. This is a critical dependency — hisab's `linalg_ext.cyr` wraps these functions.

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
- **6.0.14** (current pin): clean build/test (825/825). Migration was manifest-only (pin bump + sakshi resolution); the 34 math modules moved `lib/`→`src/` so the committed `lib/` no longer shadows the toolchain's version-pinned stdlib snapshot.

**Watching upstream:**
- **5.7.11** — RISC-V rv64.

## Cyrius stdlib modules (23 used)

| Module | Purpose | Risk |
|--------|---------|------|
| alloc | Bump allocator | Foundation — cannot change |
| string, str | C strings, fat strings | Stable |
| fmt | Formatting | Stable |
| vec | Dynamic array | Stable |
| math | f64 extended ops (sinh, pow, atan2) | Stable |
| matrix | Dense matrix storage | **Overflow bug** (C3) |
| linalg | Decompositions (LU, QR, SVD, eigen) | New in 4.10.2 |
| tagged | Option/Result types | Stable |
| fnptr | Function pointer calls | Stable |
| syscalls, io, args | System interface | Stable |
| assert, bench | Test/benchmark framework | Stable |
| callback | Higher-order functions | Stable |

## sakshi (first-party dependency)

**Status:** `sakshi` **2.1.0** via git, modules path `dist/sakshi.cyr`

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
