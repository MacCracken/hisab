# Dependency Watch

Tracked dependency version constraints and upgrade paths.

## Cyrius Toolchain

**Status:** Pinned to **5.7.7** via `cyrius.cyml [package].cyrius` (legacy `.cyrius-toolchain` removed; CI/release grep the manifest directly).

**Note:** Cyrius stdlib provides `lib/linalg.cyr` with LU, Cholesky, QR, SVD, eigendecomposition. This is a critical dependency — hisab's `linalg_ext.cyr` wraps these functions.

**Upstream notes (5.x line):**
- 5.0+: `lib/matrix.cyr` overflow class addressed; SVD precision improvements landed.
- 5.5.x: structured-deps protocol, `cyrius distlib` multi-profile, `#deprecated("...")` attribute, `cyrius vet`/`cyrius lint`/`cyrius fmt` matured.
- 5.7.x: cyrius-ts JSX AST (not consumed here), fixup-table cap 262K → 1M, `cyrius build` atomic-output (failed compile no longer destroys an existing binary).

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
