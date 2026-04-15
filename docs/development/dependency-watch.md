# Dependency Watch

Tracked dependency version constraints and upgrade paths.

## Cyrius Toolchain

**Status:** Pinned to 4.10.3 via `.cyrius-toolchain`

**Note:** Cyrius 4.10.3 provides `lib/linalg.cyr` (957 lines) with LU, Cholesky, QR, SVD, eigendecomposition. This is a critical dependency — hisab's `linalg_ext.cyr` wraps these functions.

**Upstream issue:** `lib/matrix.cyr` has integer overflow in `mat_new(rows, cols)` allocation size calculation. Tracked for cyrius 5.0.1.

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

## sakshi (external dependency)

**Status:** `sakshi` 0.9.0 via git

**Purpose:** Structured logging (timestamps, levels, categories)

**Risk:** Low. Logging is write-only — no data flows back. If sakshi breaks, hisab still compiles (just without logging).

## Rust-era dependencies (archived in rust-old/)

These are no longer used but documented for reference:
- `glam` 0.29 — replaced by hisab's own vec/mat/quat types
- `serde` 1.0 — no serialization in Cyrius port (yet)
- `thiserror` 2.0 — replaced by ERR_* integer codes
- `tracing` 0.1 — replaced by sakshi
- `reqwest` 0.12, `tokio` 1.0 — AI module not ported
- `rayon` 1.0 — parallel module not ported
- `criterion` 0.5 — replaced by bench.cyr
