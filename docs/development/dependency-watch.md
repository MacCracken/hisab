# Dependency Watch

Tracked dependency version constraints and upgrade paths.

## Cyrius Toolchain

**Status:** Pinned to **6.5.6** via `cyrius.cyml [package].cyrius` (legacy `.cyrius-toolchain` removed; CI/release grep the manifest directly).

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
- **6.5.6** (current pin, v2.6.11): bump from 6.4.69 — a **minor** jump across 24 releases (6.4.70–6.4.86 + 6.5.0–6.5.6). **No executable library change** — all 34 modules compile clean; the `dist/hisab.cyr` diff is the version header plus one rewritten `mat_new_guarded` doc comment and contains zero non-comment lines (16,878 → 16,885); a consumer including the full bundle compiles + runs end-to-end (7/7 assertions, exit 0). Re-vendored via `cyrius lib sync` — all 27 declared-subset files byte-match 6.5.6; transitive `lib/result.cyr` + `lib/atomic.cyr` already identical (no hand-refresh). `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. sakshi **2.4.6 → 2.4.7** (latest tag; cyrius 6.5.6 folds the same 2.4.7 into its own `lib/`). Stdlib delta was **six files**:
  - **`ganita.cyr` 1.0.3 → 1.0.4 — the important one, and it was a *vendoring* miss, not a pin miss.** The repo's copy was stale at 1.0.3 while the **6.4.69 pin already shipped 1.0.4**, so this delta is partly catch-up rather than new upstream work. 1.0.4 adds the CWE-190 guard to `ganita_mat_new` (`GANITA_MAT_MAX_ELEMS = 33_554_430`, derived from `alloc`'s 256 MB `ALLOC_MAX`; returns 0 for non-positive dims and when `rows > MAX / cols`) plus null propagation through `ganita_mat_identity` / `ganita_mat_from_array`. **hisab links this directly** — `mat_new`/`mat_mul`/`mat_lu` are thin `ganita_*` aliases (`lib/ganita.cyr:1329+`) used across `optimize.cyr` and `linalg_ext.cyr`. Measured, compiler held constant and only `ganita.cyr` swapped: `mat_new(-5, 3)` → **SIGSEGV (exit 139)** on 1.0.3, → null on 1.0.4. Closes the roadmap's tracked "stdlib `mat_new` overflow guard" item. **Lesson:** byte-compare `lib/` against `~/.cyrius/versions/<pin>/lib/`, not the previous pin against the new one — the latter cannot see vendoring drift.
  - **`vec.cyr`** — +`vec_sort_by` (introsort: median-of-3 quicksort, Hoare partition, insertion cutoff 16, heapsort at `2*log2(n)` depth) and +`vec_select_nth` (Hoare quickselect), both comparator-driven via `fncall2`, from 6.5.4. Purely additive; hisab's existing `vec_*` calls are unchanged. Not adopted this release — see the note under *Adoption candidates* below.
  - **`syscalls_linux_common.cyr`** — +`sys_exit_group(code)`, the `exit_group(2)` counterpart to the per-thread `sys_exit`. **Adopted**: `src/main.cyr`, `examples/basic_math.cyr` and all six harnesses now use it.
  - **`syscalls_windows.cyr` / `syscalls_x86_64_agnos.cyr`** — the peer definitions of `sys_exit_group` (Windows routes to `SYS_EXIT` because `ExitProcess` already ends every thread; agnos defines no `exit_group`), plus agnos GPU `present`/`fill` wrappers from 6.4.70. Non-Linux-x86_64 variants; vendored for snapshot parity, no impact on hisab's build target.
  - **`io.cyr`** — minor upstream changes; hisab links `io` (via `println` in the smoke binary and harnesses) and behaviour for those calls is unchanged; 961/961 confirms.

  Tracked-issue re-verify on the new pin (minimal repros): interval-ident-lex **still live** (`var iv_add` → `expected identifier, got unknown`); for-empty-clauses **still live** (`for (; …)` → `unexpected ';'`); cli-arg-clobber not re-tested (destructive). No new fixes.

  *Adoption candidates (deliberately deferred, with a concrete blocker):* hisab has exactly **one** hand-rolled sort — `_col_sort_indices_by_xy` (`src/collision_core.cyr:339`), an O(n²) insertion sort with a single caller at `:393` (the `convex_hull_2d` monotone-chain pre-sort). It is a plausible `vec_sort_by` target on paper, but **it does not fit the API**: `vec_sort_by` invokes the comparator as `fncall2(cmp, elem_a, elem_b)` — element *values* only (`lib/vec.cyr:346`) — whereas hisab sorts *indices* by dereferencing each into a separate `points` vector. Cyrius has no closures, so the comparator could only reach `points` through a file-scope global, trading a self-contained 30-line function for shared mutable state. Deferred on three grounds: the API mismatch above; CLAUDE.md's "wait for the third instance" rule (this is the first); and the fact that any swap is a hot-path perf change that would need before/after benchmark numbers, which a maintenance bump should not be smuggling in. Revisit if a second/third keyed-sort site appears, or if upstream adds a context-carrying comparator. `vec_select_nth` has no call site in hisab today.
- **6.4.69** (v2.6.10): clean 3-patch bump from 6.4.66. **No library source change** — all 34 modules compile clean; `dist/hisab.cyr` byte-identical apart from the version header. Re-vendored via `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.4.69; the transitive `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed). `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. Stdlib delta was **three files**, all upstream bug/hardening fixes: `fmt.cyr` (`fmt_hex`/`%x` `> 0`→`!= 0` so high-bit-set values render every digit; `fmt_float_buf` non-finite guard → `inf`/`-inf`/`nan` instead of a garbage `-.00000-` token — **hisab links this** via `symbolic`/`symbolic_ext`, byte-identical for finite in-range values so tests stay 957/957), `math.cyr` (float-parse exponent saturates at 340, O(exp)→O(1), closing a `"1e100000000"` ~237 ms algorithmic-complexity DoS; behaviour-preserving), and `syscalls_x86_64_agnos.cyr` (`sys_reboot()` widened nullary→4-arg `power_sys` for agnos 1.55.25 — agnos-only variant, no impact on hisab's x86_64-linux target). sakshi unchanged at **2.4.6** (already the latest tag). Tracked-issue re-verify on the new pin (minimal repros): interval-ident-lex **still live** (`var iv_add` → `expected identifier, got unknown`); for-empty-clauses **still live** (`for (; …)` → `unexpected ';'`); cli-arg-clobber not re-tested (destructive). No new fixes.
- **6.4.66** (v2.6.9): bump from 6.3.11 (a full minor across 55 patch releases). **No library source change** — all 34 modules compile clean; `dist/hisab.cyr` byte-identical apart from the version header; a consumer including the full bundle compiles + runs end-to-end. Re-vendored via `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.4.66; the transitive `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed this bump, unlike 6.3.11). `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. The 6.4.66 stdlib snapshot is much larger than 6.3.11's (new `async`/`tls`/`sandhi`/`regex`/`http`/… modules) but the declared subset hisab pulls is unchanged. **Fixed** a pre-existing `tests/modules.tcyr` compile failure surfaced (not caused) by this bump: its interval result vars `iv_add`/`iv_sub`/`iv_mul` collide with reserved cycc SIMD intrinsic names, unusable as variables (`expected identifier, got unknown`; reproduces on 6.3.11–6.4.66) — renamed to `iv_sum`/`iv_diff`/`iv_prod`, suite restored to 312/312, full run 957/957. New tracked issue: `issues/2026-07-17-cyrius-interval-ident-lex.md`. Tracked-issue re-verify on the new pin: for-empty-clauses **still open** (unchanged rejections on 6.4.66).
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

**Status:** `sakshi` **2.4.7** via git, modules path `dist/sakshi.cyr` (bumped 2.4.6 → 2.4.7 in v2.6.11; commit-pinned in `cyrius.lock`). Its shipped surface still links only `fnptr` + `atomic`. 2.4.7 is a **latent-bug fix with no public API change**: two private helpers were renamed — `_sk_write_int` → `_sk_buf_write_number` and `_sk_write_str` → `_sk_buf_write_string` — because `_int` / `_str` / `_cstr` are **reserved cyrius overload-dispatch suffixes**. The compiler rewrites a call `X(a, …)` into `X_int` / `X_str` when arg 1 matches that type and the sibling exists, so `_sk_write_int` was squatting the dispatch slot of the unrelated `_sk_write(buf, len)` flush function; any `_sk_write(someInt, …)` would have been silently redirected into a 4-parameter function with the missing args bound to garbage. Latent, not live, because every real call site passes a buffer address first. Both renamed symbols are `_sk_`-private, so hisab links nothing that moved.

> **Checked in hisab as a result** (2026-08-03): the same defect class does not exist here. `src/` defines exactly one function carrying a reserved dispatch suffix — `expr_to_str` (`symbolic`) — and there is **no** `expr_to` base function for it to shadow, so no dispatch slot is claimed. Re-run on any new `*_int` / `*_str` / `*_cstr` definition: the rule is that a suffix-named function must be the same-arity type variant of its base, or not carry the suffix at all.

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
