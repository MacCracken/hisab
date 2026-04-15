# Proposal: `lib/linalg.cyr` for Cyrius stdlib

> Upstream target: cyrius stdlib
> Motivation: hisab port to Cyrius
> Status: **shipped** — cyrius 4.10.2 (Tier 1 + Tier 3, 659 lines, 34 assertions) + 4.10.3 (Tier 2, +298 lines, 51 total assertions)

## Problem

`lib/matrix.cyr` (178 lines) provides the storage layer — `{ rows, cols, data[] }` layout, get/set, add/sub/mul/transpose. That's clean and complete for what it is.

The hisab port needs dense linear algebra (LU, QR, Cholesky, SVD, eigendecomposition, solvers) which doesn't exist in the Cyrius stdlib. All of it is pure math on top of the existing matrix layout using `f64_*` builtins — no compiler changes needed.

## Design: separate file, not expanding matrix.cyr

Follows the stdlib layering convention:

| Low-level | High-level |
|-----------|------------|
| `string.cyr` (raw C strings) | `str.cyr` (fat Str type) |
| `hashmap.cyr` (basic) | `hashmap_fast.cyr` (Swiss table) |
| f64 builtins (compiler) | `math.cyr` (sinh, pow, atan2) |
| **`matrix.cyr`** (storage + basic ops) | **`linalg.cyr`** (decompositions, solvers) |

## Dependencies

```cyrius
include "lib/alloc.cyr"
include "lib/math.cyr"
include "lib/matrix.cyr"
include "lib/linalg.cyr"
```

## Proposed API

### Tier 1 — Linear solvers (~285 lines)

Unblocks hisab `num/linalg.rs`, `num/roots.rs`.

| Function | Description |
|----------|-------------|
| `mat_lu(m)` | LU decomposition with partial pivoting. Returns `(lu_matrix, pivot_vec)` — packed L\U in one matrix, pivot as separate i64 array |
| `mat_lu_solve(lu, piv, b)` | Solve Ax=b from LU factors. `b` is flat f64 array of length n |
| `mat_det(m)` | Determinant via LU (product of diagonal x pivot sign) |
| `mat_inv(m)` | Inverse via LU solve on identity columns |
| `mat_cholesky(m)` | Cholesky factorization for symmetric positive-definite. Returns lower-triangular L where A = LL^T |
| `mat_cholesky_solve(l, b)` | Forward/backward substitution from Cholesky factor |
| `mat_qr(m)` | Householder QR decomposition. Returns `(q_matrix, r_matrix)` |
| `mat_gaussian_elim(aug)` | Gaussian elimination on augmented matrix. Solves in-place |
| `mat_trace(m)` | Sum of diagonal elements |

### Tier 2 — Advanced decomposition (~285 lines)

Unblocks hisab `num/svd.rs`, `num/eigen.rs`, `num/optimize.rs`.

| Function | Description |
|----------|-------------|
| `mat_svd(m)` | SVD via Golub-Kahan bidiagonalization. Returns `(u, sigma_vec, vt)` |
| `mat_eigen_sym(m)` | Eigendecomposition for real symmetric matrices (Jacobi rotation). Returns `(eigenvalues, eigenvectors)` |
| `mat_pseudo_inv(m)` | Moore-Penrose pseudoinverse via SVD |
| `mat_rank(m, tol)` | Numerical rank via SVD singular value count above tolerance |
| `mat_condition(m)` | Condition number (sigma_max / sigma_min) |
| `mat_least_squares(a, b)` | Least-squares solve via QR |

### Tier 3 — Utilities (~130 lines, added to matrix.cyr itself)

Small helpers that belong in the storage layer.

| Function | Description |
|----------|-------------|
| `mat_copy(m)` | Deep copy |
| `mat_neg(m)` | Negate all elements |
| `mat_row(m, r)` | Extract row as flat f64 array |
| `mat_col(m, c)` | Extract column as flat f64 array |
| `mat_set_row(m, r, data)` | Overwrite entire row |
| `mat_set_col(m, c, data)` | Overwrite entire column |
| `mat_submatrix(m, r0, c0, r1, c1)` | Extract submatrix as new matrix |
| `mat_frobenius(m)` | Frobenius norm |
| `mat_max_norm(m)` | Infinity norm (max absolute row sum) |
| `mat_is_symmetric(m, tol)` | Check symmetry within tolerance |
| `mat_eq(a, b, tol)` | Element-wise equality within tolerance |

## Hisab module mapping

| hisab Rust file | Lines | What it needs from linalg.cyr |
|-----------------|-------|-------------------------------|
| `num/linalg.rs` | 722 | LU, Cholesky, QR, determinant, trace, multiply, inverse, pseudo-inverse |
| `num/roots.rs` | 166 | gaussian_elimination |
| `num/svd.rs` | 330 | SVD, truncated SVD |
| `num/eigen.rs` | 282 | Power iteration, symmetric eigendecomposition |
| `num/optimize.rs` | 497 | mat_mul, mat_inv (via LU) for BFGS/L-BFGS |
| `num/solvers.rs` | 380 | mat_mul for iterative solvers (PGS, GMRES, BiCGSTAB) |
| `num/sparse.rs` | 198 | CSR format — separate concern, not in this proposal |
| `num/complex_linalg.rs` | 1430 | Complex variant — separate concern, future `complex_linalg.cyr` |

## Test plan

- New `tests/tcyr/linalg.tcyr` covering all Tier 1 + Tier 2 functions
- Extended `tests/tcyr/matrix.tcyr` for Tier 3 additions
- Benchmarks: `tests/bcyr/linalg.bcyr` for decomposition performance on 4x4, 16x16, 64x64 matrices

## Estimate

~800 lines of new Cyrius code. Deliverable incrementally — Tier 1 alone unblocks the bulk of hisab's `num` module port.

## Compiler builtins available (no changes needed)

f64_from, f64_to, f64_add, f64_sub, f64_mul, f64_div, f64_neg, f64_eq, f64_lt, f64_gt, f64_abs, f64_sqrt, f64_sin, f64_cos, f64_exp, f64_ln, f64_atan, f64_floor, f64_ceil, f64_round, f64_exp2

Plus `lib/math.cyr`: f64_pow, f64_min, f64_max, f64_clamp, f64_asin, f64_acos, f64_atan2
