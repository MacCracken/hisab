# Dependency Watch

Tracked dependency version constraints and upgrade paths.

## Cyrius Toolchain

**Status:** Pinned to **6.6.4** via `cyrius.cyml [package].cyrius` (legacy `.cyrius-toolchain` removed; CI/release grep the manifest directly).

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
- **6.6.4** (current pin, v3.1.1): **repairs all four defects hisab filed on 2026-09-13** —
  `&_private_fn` reachable via address-of (eleven resolution paths now run `_vis_check`),
  `public enum` leaking `public` onto the next declaration (hisab's public-surface gate reported
  `_ad_pow` REFUSED on the pin bump, 456/457 → 457/457; `KNOWN_LEAKS` emptied), `cyrius deps`
  re-locking a stdlib leaf silently under an unchanged pin (`cyrius.lock` now ends with a
  `cyrius\t<pin>` trailer; a moved leaf is refused, `deps --relock` accepts), and
  `install.sh --refresh-only` writing a released slot (refused from a drifted tree;
  `scripts/verify-store.sh` audits/restores). All archived upstream. Stdlib delta: six files, all in
  the syscall/io layer (`io.cyr` + five `syscalls_*` peers — per-target `O_*` flags, `SYS_FLOCK`);
  30/30 byte-match the 6.6.4 tag, sakshi 2.5.2 and ganita 1.2.5 unchanged and still the latest
  tags. Suites print byte-identical output (4214/4214); binaries +32 B each (the moved `io.cyr` is
  in every one). No `#deprecated`, no `impl`, no ≥ 64 KB literal in hisab, so the rest of 6.6.4's
  list is not exposed.
- **6.6.3** (v3.0.1): **closes both defects hisab filed on 2026-09-11, and finds two
  new ones in the toolchain's own release plumbing.** Stdlib delta is two header lines — ganita
  **1.2.4 → 1.2.5**, sakshi **2.5.1 → 2.5.2**, each a pin-only refold — so the suites and the CLI
  compile **byte-identical** under 6.6.2 and 6.6.3 (`foundation` 442,040 B, `edge_cases` 586,464 B,
  `build/hisab` 257,176 B, `cmp` clean). Suite output byte-identical too, 4202/4202. Lock is now
  written in **sorted order** (a 6.6.3 fix); 48 lines of textual churn, 3 entries order-insensitively.

  ⭐ **Nested `continue` is FIXED**, verified as a pair from dirs pinned to each version: hisab's
  filed reproducer exits **1 → 0**, the upstream gate reads **2/8 → 8/8**, and `_cga_build_null_tbl`
  rewritten into its natural `continue` form (inner `continue` firing 1024 times) gives a wrong table
  on 6.6.2 (FNV mismatch, 1024/1024 bad coefficients) and the exact contract table on 6.6.3. The
  `if`-guard form is **kept** — consumers compile the bundle under their own pins and the `continue`
  form fails silently below 6.6.3 — but its comment no longer claims to be load-bearing.
  Record: `issues/archived/2026-09-11-cyrius-nested-continue-binds-to-wrong-loop.md`.

  ⭐ **`#derive` + `public` is FIXED**, verified on hisab's own idiom in both directions: 6.6.2
  rejects it; 6.6.3 builds it, derived getters AND setter reachable cross-file (exit 0), and a
  consumer calling the file-private helper is still refused with no binary. The roadmap's
  public/private row is unblocked; the work stays scheduled (3.1.0 / 4.0.0).
  Record: `issues/archived/2026-09-11-cyrius-derive-cannot-combine-with-public.md`.

  ⛔ **THE INSTALLED "6.6.2" SNAPSHOT WAS NOT 6.6.2.** `~/.cyrius/versions/6.6.2/lib` held 6.6.3's
  twelve refolded stdlibs, byte-identical to `versions/6.6.3/lib` (ganita.cyr mtime 2026-09-12
  08:49, two days after 6.6.2 shipped), because cyrius's dev loop (`install.sh --refresh-only`)
  writes the repo's in-progress `lib/` into `versions/$(cat VERSION)/lib` and VERSION still named the
  released 6.6.2. The cyrius **tag** 6.6.2 ships ganita 1.2.4 — hisab's committed `lib/` and docs
  were right. **And a bare `cyrius build` under the unchanged 6.6.2 pin silently rewrote
  `lib/ganita.cyr` to 1.2.5 and re-locked it**, printing exactly what a no-op prints; only
  `git status` noticed. Both halves filed upstream with a self-proving repro
  (`cyrius/docs/development/issues/2026-09-13-hisab-refresh-only-overwrites-released-snapshot.md`,
  `…/2026-09-13-hisab-deps-relocks-silently-under-unchanged-pin.md`). **The vendoring check now
  compares every `lib/` file against `git show <pin>:lib/<file>` in the cyrius repo — the tag, never
  the install dir**: 30/30 match 6.6.3's tag, sakshi matches its 2.5.2 tag, `deps --verify` 31/31.

  ⚠ **Not exposed, checked rather than assumed**: the headline `#inline`-disarms-`#derive` fix —
  hisab has **zero** `#inline` directives (the one match is inside a comment), which is why the bundle
  compiled on 6.6.2 despite mat4 preceding eight deriving modules in `[lib]` order. `CYRIUS_DCE` is
  used nowhere in hisab's scripts or CI. The expanded-source (24 MiB) and token (4,194,304) caps are
  unchanged in source between the tags (`lex.cyr` untouched; `lex_pp.cyr`'s 53 inserted lines are the
  `public ` prefix probe), bundle 1,103,166 B = **4.38%** of the byte cap.

  ⚠ **No performance change is claimed, and for once that is a fact rather than a measurement**: the
  binaries are byte-identical, so the four bench runs (2 × 6.6.2, 2 × 6.6.3, quiet box, load
  0.3–0.7) are a same-binary control — median **+0.68%**, 0 of 78 rows past 10%, no row past 2× its
  own spread.

- **6.6.2** (v2.11.5): **repairs the wrong-code bug hisab filed on the 6.6.1 bump.**
  Stdlib delta is `result.cyr`, `tagged.cyr` and the NEW `lib/boxed.cyr` (pulled transitively by
  `tagged`; lock 30 -> 31). ganita stays 1.2.4, sakshi stays 2.5.1. All 31 vendored files
  byte-match the **6.6.2 snapshot itself**.

  ⭐ **The SIMD destination-slot miscompile is FIXED**, verified from the consumer side: hisab's own
  filed reproducer exits **0** on 6.6.2 and **139** on 6.6.1, same binary, and the three suites that
  used to SIGSEGV pass with `m4_mul_vec4`'s hoist REMOVED. The hoist is kept for consistency with
  `m3_mul_vec3` only, and its comment no longer claims otherwise.

  ⛔ **hisab's filing was wrong about the scope in two ways.** It was filed as derive-specific and as
  a 6.5.71 regression and was **neither** — all 21 `f64v_*`/`f32v_*`/`f32v8_*`/`f64v256_*`/`iv_*`
  handlers bound an argument's frame local over their own destination slot, reachable via `callptr`
  since 6.0.70. **A first-bad-version is evidence about visibility, not origin.** Severity was
  understated too: under `CYRIUS_REGALLOC_PICKER_CAP=0` it exits 0 and writes into the argument
  object rather than faulting.

  ⚠ **A second SIMD fix retires a landmine this repo documented since 2.3.1 with the WRONG CAUSE.**
  A bare intrinsic at top level compiled clean and SIGSEGV'd on every release — not SSE stack
  misalignment as `src/vec4.cyr` claimed, but because the handlers stash operands in FRAME slots and
  top-level code has no frame. 6.6.2 refuses at compile time. Corrected in `vec4.cyr`.

  ⛔ **NOT EXPOSED, BUT WORTH KNOWING: 6.6.0 silently redefined `tag()` and `is_tag()` at unchanged
  arity** — `tag(box)` returned the pointer, `is_tag` compared pointer to tag, both compiling and
  running clean. hisab calls neither (checked, not assumed), so it passed through 6.6.0/6.6.1
  unaffected. **The rule 6.6.2 establishes: a name whose meaning changed must be RETIRED, not
  redefined** — same-arity redefinition is the one class a consumer build structurally cannot catch.

- **6.6.1** (v2.11.3): **forty-odd releases from 6.5.33, crossing a minor.** Not
  compiler-only: **ganita 1.1.4 → 1.2.4**, plus `io.cyr`, `math.cyr`, `result.cyr`, `tagged.cyr`
  and four syscalls variants. sakshi 2.4.11 → **2.5.1** alongside it. All 30 `lib/` files are
  byte-identical to `~/.cyrius/versions/6.6.1/lib`, checked file by file against **the pin's own
  snapshot** — not old-pin vs new-pin, the shortcut that hid a stale ganita three times.

  ⛔ **THIS BUMP MISCOMPILES HISAB, AND THE REGRESSION IS UPSTREAM.** A `#derive(accessors)` getter
  passed directly as an `f64v_*` intrinsic argument makes the intrinsic read its **destination
  pointer from a stack slot nothing writes**. `m4_mul_vec4` SIGSEGV'd on every call; three of five
  suites died at rc=139. **Bisected: 6.5.70 clean, 6.5.71 broken** — the release that put derive
  accessors on the inline-replay path (`callq` 7 → 3), removing the call that had incidentally
  forced the spill. Worked around in `src/mat4.cyr` by hoisting the accessors into locals (what
  `m3_mul_vec3` always did). ⛔ **SUPERSEDED 2026-09-09**: cycc 6.6.2 repairs this upstream (see the 6.6.2 entry above). The hoist is verified non-load-bearing and is retained only for consistency with `m3_mul_vec3`. **Filed in the cyrius repo** (that is where the language agent reads them):
  `cyrius/docs/development/issues/2026-09-09-hisab-derive-accessor-simd-dst-slot.md`, with a
  self-validating repro in `repros/` — exit 0 on 6.5.70, exit 139 on 6.6.1.

  ⭐ **DEPRECATED ALIASES: MIGRATED IN 2.11.4, and the estimate in this paragraph was wrong by 67x.**
  It read "hisab uses `f64_acos` 5x and `f64_atan2` 3x" and closed with "hisab calls `ganita_mat_*`
  already". Both were false. The real figure is **536 call sites over 20 of the 53 deprecated
  names**, `mat_set` alone accounting for 265 — and every `ganita_*` occurrence in the tree at the
  time was in a **comment**, not a call. ⚠ **The estimate was taken from the names ganita's changelog
  paragraph happened to mention, not from hisab's code**; the deprecation block is a 53-entry table
  further down the same file. Scope a migration from the dependency's *surface*, never from its
  release notes. All 536 are now on the `ganita_*` spellings, `dist/hisab.cyr` included.

  - **`ganita_f64_pow` (1.2.4)** — integral exponents now go through **binary exponentiation**
    instead of `exp(n·ln|base|)`, because the transcendental round trip returned
    `pow(7,2) = 48.99999999999999296` and `pow(10,15)` up to 47 ulps out. **This changed a hisab
    public API result**: `f64_pow(-2,4)` read **15** through the truncating `f64_to` and now reads
    a bit-exact **16**. `tests/modules.tcyr` had pinned the old value, so the upstream *fix* failed
    the suite. It also removes `_ad_pow`'s precision advantage — the two paths now agree bit for bit.
  - **`lib/math.cyr` (6.6.1)** — `_f64_exp_polyfill` and its `exp2` twin gained the missing
    infinity guard (`exp(+inf) = +inf`, `exp(-inf) = +0`). Previously the range reduction computed
    `inf - inf` and the `2^n` bit-pack read a saturated `f64_to(inf)`. **Also changed a hisab
    result**: `cx_exp(-inf + 0i)` returned NaN and now returns exactly +0, which is correct;
    `tests/abuse.tcyr` had pinned the NaN.
  - **`lib/io.cyr` (6.5.36/6.5.45)** — `getenv` now reads `/proc/self/environ` to EOF into a heap
    buffer and caches it, instead of scanning a fixed 8 KB stack window on every call. Nothing in
    the vendored subset calls `getenv`, so hisab is unaffected either way.
  - ⚠ **Two compiler ceilings moved, and neither is the one this repo had written down.** Expanded
    source **8 MB → 24 MB**; token count **1,048,576 → 4,194,304**. See the corrected comment in
    `cyrius.cyml` — the retired "16 MB `input_buf`" figure described `_SRC_CAP`, which is not what
    rejects a consumer's build.

- **6.5.33** (v2.11.2): **fifteen-release bump from 6.5.18** — the largest gap this
  file has recorded. Not compiler-only: **ganita 1.0.4 → 1.1.4**, plus `fmt.cyr`, `assert.cyr`,
  `bench.cyr` and three syscalls variants. sakshi 2.4.10 → **2.4.11** alongside it.

  ⚠ **THE TREE ARRIVED MID-SYNC, AND THE HALF THAT WAS MISSING WAS THE HALF THAT MATTERED.**
  `lib/` had already been vendored from **≈6.5.19**: `alloc`, `assert`, `atomic`, `bench` and
  `syscalls_windows` were current, while `ganita.cyr` (**1.0.4, −202 lines**), `fmt.cyr` and three
  more syscalls variants were still behind. A diff of old-pin against new-pin looks tidy and misses
  this entirely; only a byte-compare against **the pin's own snapshot** finds it. That is now the
  **third** time ganita specifically has been caught stale by exactly this shortcut. All 29 files
  are byte-identical to `~/.cyrius/versions/6.5.33/lib` as of this entry, checked file by file.

  **The code-only stdlib delta is four functions**, measured with a brace-aware extractor that
  strips comments: `ganita_f64_pow`, `fmt_float_buf`, `assert_eq`, and `bench.cyr`'s timing core.
  ⚠ A first pass keyed on `fn` alone reported **seven** changed ganita bodies; six were the trailing
  comment block being attributed to the preceding function.

  - **`ganita_f64_pow` (1.1.4)** — full C-pow domain: zero base, zero exponent, and negative base
    with an integral exponent are special-cased instead of returning NaN from `exp(n*ln(base))`.
    **This changed a hisab public API**: `symbolic.cyr`'s `expr_eval` calls it raw, so `(-2)^3`
    returned NaN for hisab's entire history and returns a number now. 12 assertions added; 10 of
    them fail against a 1.0.4 checkout, the 2 that do not are the controls. hisab's `_ad_pow` stays,
    on **precision** — the upstream magnitude comes from `exp(n*ln|base|)`, so `(-2)^4` reads **15**
    through the stdlib and **16** through hisab, `f64_to` being a truncation.
  - **`bench.cyr` (6.5.19)** — calibrates one clock read on the host and subtracts it from every
    sample; `bench_run` sizes its own batches. **44 of hisab's 72 rows moved >10%, none a speedup.**
    Drove the `regime`/`floor_ns` columns in `bench-history.csv`. See `benchmarks.md`.
  - **`fmt_float_buf` (6.5.30)** — emitted the integer part before rounding the fraction, so a
    carry was lost: `3 - 1e-7` printed `2.1000000`. hisab's own `_sym_render_f64` is a hand-copy of
    this routine and is now a redundant duplicate; filed on the roadmap, not fixed here.
  - **`assert_eq` (6.5.19)** — its two numbers went to fd 1 while the rest of the message went to
    fd 2, orphaning them under any harness that captures the streams separately.

  **Two toolchain behaviours changed that no stdlib diff would show:**

  - ⚠ **`cyrius fmt` REWRITES IN PLACE as of 6.5.28** (was stdout-only); `--dry` is the old
    behaviour. **This repo's CI was printing `cyrius fmt $f > tmp && mv tmp $f` as its remediation
    advice**, which now truncates the file — reproduced at `src/vec2.cyr`, **2,233 B → 0 B, rc=0**.
    Fixed in `.github/workflows/ci.yml`. The same release fixed `cyrfmt`'s missing paren tracking,
    which drifted **38 of 44** sources here at once — i.e. the destructive advice would have fired
    38 times on this very bump.
  - ⭐ **`input_buf` went 1 MB → 16 MB in 6.5.22** (`_SRC_CAP`, relocated `0x00000` → `0x4D9D000`);
    the old cap had been refusing sigil (1,084,265 B), mabda (1,259,999 B) and drishti
    (1,403,806 B) outright. `cyrius.cyml` and CLAUDE.md both still documented the 1 MB ceiling.
    Verified by compiling a **1,162,472 B** source — 111% of the retired cap — clean. The bundle is
    **5.4%** of the buffer, not 77%.

  **A constant derived from a dependency is a measurement, and it goes stale silently.** Three of
  them here, none surfaced by a failing test: all 3514 assertions passed on both sides of the bump.

- **6.5.18** (v2.10.1): single-release bump from 6.5.17. **Compiler-only, ZERO stdlib delta** — all 29 vendored `.cyr` files plus `sakshi.cyr` are byte-identical between the two pins as well as byte-matching the 6.5.18 snapshot, so `cyrius lib sync` was a no-op and `deps --verify` stayed 30/30.

  Its headline fix is a **`cyrius fmt` bug that corrupted multi-line string literals** — a continuation line was given the enclosing statement's indentation, putting spaces *inside* the string; upstream measured it rewriting **1,239 lines of cyrius's own `src/main.cyr`**. hisab's exposure was checked rather than assumed: all **44 sources are still `fmt --check` clean** under the repaired formatter, so no hisab file had been silently reformatted by the broken one.

  ⚠ **The open dead-function-bodies filing was re-run on 6.5.18 from its own repro and is UNCHANGED**: `cyrius build` and `cyrius check --with-deps` reject a file that does not parse (exit 1), but **`cyrius lint` and `cyrius vet` still exit 0**. Still partial, still open, and now verified across two consecutive releases rather than assumed to have been swept up.

- **6.5.17** (v2.10.0): single-release bump from 6.5.16. **Compiler-only — zero stdlib delta**: all 29 vendored `.cyr` files plus `sakshi.cyr` byte-match the 6.5.17 snapshot AND are byte-identical between 6.5.16 and 6.5.17 for hisab's declared subset, so `cyrius lib sync` was a no-op and `deps --verify` stayed 30/30. Suite unchanged at 3376 across the bump itself (3398 after 2.10.0's own additions), all gates green.

  **This release fixes all three defects hisab filed upstream during the 2.9.2/2.9.3 work.** Each was re-verified by re-running its own filed repro on the new pin rather than read off the release note:
  - **Capturing closure SIGSEGV across a function boundary — FIXED.** The filed case-3 program returns **42** (was SIGSEGV 139) and the `fncall1` variant returns 42 too, so both dispatch paths are repaired. Archived. ⚠ **This retires the measured half of the `vec_sort_by` deferral below** — a capturing comparator now works and can close over the `points` vector, so the "shared mutable global" objection is gone. What survives is the plain API mismatch (`fncall2(cmp, elem_a, elem_b)` passes values, hisab sorts *indices*) plus CLAUDE.md's wait-for-the-third-instance rule; still one call site.
  - **`cyrius distlib` rejecting correct bundles — FIXED.** `cyrius distlib` on hisab's real bundle exits **0**, and the three-arm minimal reproducer passes on stdlib function / global var / enum constant alike. **The CI and release workaround is removed** — both workflows run a bare `cyrius distlib` again and a non-zero exit is a real failure once more. The independent `cyrius check --with-deps dist/hisab.cyr` step was kept; it is strictly stronger than the self-check ever was.
  - ⚠ **Dead-function bodies never syntax-checked — PARTIALLY fixed, still open.** `cyrius build` and `cyrius check --with-deps` now both reject the repro (exit 1), which is the half that matters: nothing unparseable can be built or shipped. But **`cyrius lint` and `cyrius vet` still exit 0** on a file that does not parse, so the release note's "accepted by every gate" is not discharged. Measured, fed back to the upstream filing, left open.

- **6.5.16** (v2.9.2): bump from 6.5.9 across **seven releases** (6.5.10–6.5.16). **No library source change** — all 34 modules compile clean; the `dist/hisab.cyr` diff is the version header and nothing else. Re-vendored via `cyrius lib sync` — all 27 declared-subset files byte-match the 6.5.16 snapshot; the transitive `lib/result.cyr` is outside the declared subset and `lib sync` does not touch it, so it was hand-refreshed (its only delta is two doc-comment paths, from 6.5.11's test-suite subfolder reorg), and `lib/atomic.cyr` was already identical. All **30** vendored files byte-match `~/.cyrius/versions/6.5.16/lib/`; `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. sakshi **2.4.8 → 2.4.10** (see the sakshi section below). Stdlib delta was **10 files** plus the transitive `result.cyr`:
  - **`alloc.cyr`** — 6.5.10 inlined the two accessor loads in `alloc_via` / `realloc_via` / `free_via` / `reset_via` (`fncall2(allocator_alloc_fn(a), allocator_state(a), size)` → `fncall2(load64(a), load64(a + 32), size)`) and removed the `_arena_alloc` / `_arena_reset` shims, pointing `allocator_new` at `&arena_alloc` / `&arena_reset` directly. **hisab never takes that path** — **zero** calls to `alloc_via` / `allocator_new` / the removed shims anywhere in `src/`, and the four functions hisab does call are **byte-identical** before and after. Cyrius's own "15.1 ns of which 5.1 ns was those two calls" is *its* measurement of a path hisab does not enter, and is not restated as a hisab number.
  - **`syscalls_linux_common.cyr` / `syscalls_aarch64_linux.cyr` / `syscalls_macos.cyr` / `syscalls_x86_64_agnos.cyr`** — the 6.5.15/6.5.16 macOS work (per-OS signal/errno constant splits, Mach-O syscall routes, sysctl-backed `uname`/`sysinfo`). Non-Linux-x86_64 variants, vendored for snapshot parity; no impact on hisab's build target.
  - **`io.cyr` / `args.cyr` / `fnptr.cyr` / `tagged.cyr` / `result.cyr`** — minor upstream changes; the calls hisab links behave identically, which 3351/3351 confirms.
  - **`sakshi.cyr`** — 2.4.10, byte-identical to what cyrius 6.5.16 folds into its own `lib/`.

  ⚠ **A caveat on the standing vendoring rule.** 2.6.11 established "byte-compare `lib/` against the *pin's own* snapshot, not previous-pin vs new-pin". That rule assumes the snapshot directories are immutable, and on this machine they are not: `~/.cyrius/versions/6.5.9/lib/alloc.cyr` is byte-identical to 6.5.16's and carries `⚡ v6.5.10` annotations — the 6.5.9 tree was refreshed in place on 2026-08-07, after v2.9.1 shipped. The rule still holds for the *current* pin, which is the one that matters; but a "6.5.N snapshot" on disk is not evidence of what 6.5.N contained.

  ⚠ The pin crossed **6.5.13, whose upstream CHANGELOG section is an empty header**, though it did ship a distinct `cycc` binary and one changed stdlib file (`syscalls_x86_64_agnos.cyr` — agnos only, never compiled by hisab). Recorded because the audit trail has a hole in it, not because anything is suspected.

  **Two toolchain-facing consequences landed in this bump, neither of them a source change:**
  - **`cyrius distlib` fails its own bundle self-check on a correct bundle**, and both `ci.yml` and `release.yml` were red because of it. 6.5.14 repaired a self-check that had never once run; the repaired check compiles the bundle **alone** under `_cc_allow_undef`, which is read only in the *fixup* stage and only gates `reachable undefined function(s)`. An unresolved **name** dies far earlier in the frontend (`parse_expr.cyr:585-593` → `undefined variable '…'`, immediate exit), so the suppression cannot reach it by construction. hisab's bundle reads `F64_ONE` **294 times**, first at `dist/hisab.cyr:108` (`hvec2_one`) — it is the ordinary way to write `1.0` in a language with no float literals, so this is not a removable dependency. Both workflows now tolerate a non-zero `distlib` rc **only** with that exact signature, and add `cyrius check --with-deps dist/hisab.cyr`, which compiles the bundle with the stdlib actually in scope. → [`issues/archived/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md`](issues/archived/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md)
  - **`dist/hisab.deps` is now tracked** (removed from `.gitignore`, added to CI's drift check), so a consumer's `cyrius deps` auto-resolves hisab's stdlib leaves instead of the four consumers hand-declaring them. It was worth nothing before **6.5.10**: the sidecar was built by scanning bundled sources for literal `include "lib/X.cyr"` lines, and hisab's `[lib]` modules are self-contained, so it reported **2** leaves. 6.5.10 unions in the declared `[deps] stdlib` — hisab goes **2 → 15**.

  Tracked-issue re-verification against 6.5.16 is carried separately and is **not** recorded here; the newest verdicts in this file remain the 6.5.6 ones below.
- **6.5.9** (v2.9.1): single-release bump from 6.5.8. **No library source change.** Stdlib delta was **two files**. **`alloc.cyr`** grew an **arena exhaustion policy** — the header goes 24 → 56 bytes (`{ base, ptr, end, policy, first_chunk, cur_chunk, chunk_size }`, `base`/`end` mirroring the current chunk so the hot path stays two loads) with `ARENA_FULL_NULL` / `_GROW` / `_SPILL` / `_ABORT`. Filed upstream by agnosai (2026-08-06): the primitives already returned 0 on exhaustion, but a `Str` of 0 is indistinguishable from a valid one and there is no error channel through the `_a` families, so the 0 flowed on and faulted several layers away. **hisab links the arena** — `src/einsum.cyr` is its only user (11 `arena_alloc`, one each `arena_new` / `arena_reset`) — but the default stays `ARENA_FULL_NULL`, so every existing arena behaves exactly as before. The second file is `syscalls_x86_64_agnos.cyr` (agnos-only, never compiled here).
- **6.5.8** (v2.9.0): bump from 6.5.6, alongside sakshi 2.4.7 → 2.4.8. **No library source change.** Stdlib delta was **11 files**, and one of them is a correctness fix hisab links directly: **`fmt.cyr`** (`fmt_int`, `fmt_int_fd`, `efmt_int`, `fmt_byte`) and **`string.cyr`** (`print_num`) rendered `i64::MIN` as a bare `"-"` — `n = 0 - n` is a no-op at the most negative two's-complement value, so `n` stayed negative, both the `n == 0` guard and the `n > 0` loop were skipped, and the sign byte was emitted with no digits after it. Fixed by keeping the sign as a *flag*, never negating `n`, and negating each extracted digit under `while (n != 0)` — the same shape `fmt_hex` had carried since 6.4.69, which the decimal siblings were never brought along for. **hisab reaches this**: `src/symbolic.cyr` calls `fmt_int` at 3 sites. **`alloc.cyr`** carries 6.5.7's fix for the default `Allocator` vtable being built inside the bump arena — `alloc_reset()` scrubbed it and the next `vec_new()` loaded a null fn pointer out of the dead vtable and jumped to 0; the vendored file credits the filing to hisab, 2026-08-05, but hisab calls neither `allocator_new` nor `alloc_reset` in `src/`, so nothing here was exposed. **`io.cyr`** gained the directory/symlink family (`xmkdir`, `xmkdir_p`, `xsymlink`, `xreadlink`, `xlink`) and **`syscalls.cyr`** gained `signal_default` — purely additive, no hisab call site. The remaining six are the `syscalls_*` platform variants (`sys_chdir`, `sys_fchownat`, and 6.5.7's Darwin `AT_*` divergence fix, which had left `xrmdir` broken on macOS-arm64 since 6.5.2).
- **6.5.6** (v2.6.11): bump from 6.4.69 — a **minor** jump across 24 releases (6.4.70–6.4.86 + 6.5.0–6.5.6). **No executable library change** — all 34 modules compile clean; the `dist/hisab.cyr` diff is the version header plus one rewritten `mat_new_guarded` doc comment and contains zero non-comment lines (16,878 → 16,885); a consumer including the full bundle compiles + runs end-to-end (7/7 assertions, exit 0). Re-vendored via `cyrius lib sync` — all 27 declared-subset files byte-match 6.5.6; transitive `lib/result.cyr` + `lib/atomic.cyr` already identical (no hand-refresh). `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. sakshi **2.4.6 → 2.4.7** (latest tag; cyrius 6.5.6 folds the same 2.4.7 into its own `lib/`). Stdlib delta was **six files**:
  - **`ganita.cyr` 1.0.3 → 1.0.4 — the important one, and it was a *vendoring* miss, not a pin miss.** The repo's copy was stale at 1.0.3 while the **6.4.69 pin already shipped 1.0.4**, so this delta is partly catch-up rather than new upstream work. 1.0.4 adds the CWE-190 guard to `ganita_mat_new` (`GANITA_MAT_MAX_ELEMS = 33_554_430`, derived from `alloc`'s 256 MB `ALLOC_MAX`; returns 0 for non-positive dims and when `rows > MAX / cols`) plus null propagation through `ganita_mat_identity` / `ganita_mat_from_array`. **hisab links this directly** — `mat_new`/`mat_mul`/`mat_lu` are thin `ganita_*` aliases (`lib/ganita.cyr:1329+`) used across `optimize.cyr` and `linalg_ext.cyr`. Measured, compiler held constant and only `ganita.cyr` swapped: `mat_new(-5, 3)` → **SIGSEGV (exit 139)** on 1.0.3, → null on 1.0.4. Closes the roadmap's tracked "stdlib `mat_new` overflow guard" item. **Lesson:** byte-compare `lib/` against `~/.cyrius/versions/<pin>/lib/`, not the previous pin against the new one — the latter cannot see vendoring drift.
  - **`vec.cyr`** — +`vec_sort_by` (introsort: median-of-3 quicksort, Hoare partition, insertion cutoff 16, heapsort at `2*log2(n)` depth) and +`vec_select_nth` (Hoare quickselect), both comparator-driven via `fncall2`, from 6.5.4. Purely additive; hisab's existing `vec_*` calls are unchanged. Not adopted this release — see the note under *Adoption candidates* below.
  - **`syscalls_linux_common.cyr`** — +`sys_exit_group(code)`, the `exit_group(2)` counterpart to the per-thread `sys_exit`. **Adopted**: `src/main.cyr`, `examples/basic_math.cyr` and all six harnesses now use it.
  - **`syscalls_windows.cyr` / `syscalls_x86_64_agnos.cyr`** — the peer definitions of `sys_exit_group` (Windows routes to `SYS_EXIT` because `ExitProcess` already ends every thread; agnos defines no `exit_group`), plus agnos GPU `present`/`fill` wrappers from 6.4.70. Non-Linux-x86_64 variants; vendored for snapshot parity, no impact on hisab's build target.
  - **`io.cyr`** — minor upstream changes; hisab links `io` (via `println` in the smoke binary and harnesses) and behaviour for those calls is unchanged; 961/961 confirms.

  Tracked-issue re-verify on the new pin (minimal repros): interval-ident-lex **still live** (`var iv_add` → `expected identifier, got unknown`); for-empty-clauses **still live** (`for (; …)` → `unexpected ';'`); cli-arg-clobber not re-tested (destructive). No new fixes.

  *Adoption candidates:* ⭐ **DECIDED 3.2.0 (2026-09-14), per routine.** `vec_sort_by` ADOPTED for one of the six: `num_factorize`'s prime-factor sort (integer VALUES, so equal keys are indistinguishable and every correct sort yields the same vec). The three linalg descending sorts are CONSOLIDATED onto one hisab helper, `_lext_sort_desc` (`src/linalg_ext.cyr`), not onto the stdlib: equal eigenvalues / singular values carry distinct vectors, so tie order is observable, and introsort permutes ties differently from the selection sort the suites and consumers have always seen (pinned: `diag(2,2,4)` gives columns `(e3, e2, e1)`, a stable sort gives `(e3, e1, e2)`). `_col_sort_indices_by_xy` (heapsort) and `_kd_select_median` (three-way quickselect) are LEFT: the first because coincident points are equal keys with distinct indices and a probe showed the two algorithms disagree on 8 of 8 slots at n = 8 and 35 of 40 at n = 40, changing the hull's returned indices; the second because `vec_select_nth` partitions a whole vec while the k-d build works on sub-ranges of one shared vec (an allocation per node under a never-freeing allocator) and its two-way Hoare partition places tied axis values by a different rule. The closure objection is gone for good — a capturing comparator over `points` ran through `vec_sort_by` on 6.6.4 in that probe. `vec_select_nth` still has no call site. Original text follows. ⛔ **CORRECTED 2026-09-09 — "exactly one hand-rolled sort" is wrong by 6x. There are SIX ordering routines** in five files (`collision_core.cyr:466`, `spatial.cyr:114`, `num_ext.cyr:309`, `linalg_ext.cyr:905`, `linalg_precision.cyr:816`, `:1265`), three of them near-identical descending-magnitude selection sorts whose own comments concede they have already disagreed once — so **CLAUDE.md's wait-for-the-third-instance gate is DISCHARGED**, and the deferral reason "this is the first" was false. A capturing comparator across a fn boundary is re-verified working on 6.6.2. Original text follows. hisab has exactly **one** hand-rolled sort — `_col_sort_indices_by_xy` (`src/collision_core.cyr:339`), an O(n²) insertion sort with a single caller at `:393` (the `convex_hull_2d` monotone-chain pre-sort). It is a plausible `vec_sort_by` target on paper, but **it does not fit the API**: `vec_sort_by` invokes the comparator as `fncall2(cmp, elem_a, elem_b)` — element *values* only (`lib/vec.cyr:346`) — whereas hisab sorts *indices* by dereferencing each into a separate `points` vector. ⚠ **This rationale used to continue "and Cyrius has no closures" — that is FALSE**, and was repeated across four files without being run: Cyrius has capturing closures (`|x| base + x`, captured by value, guide §Closures, since v6.3.8). The deferral survives for a *measured* reason instead: on 6.5.16 a capturing closure **SIGSEGVed when passed through another function and called there**, which is precisely the `vec_sort_by` shape. Isolated to that single variable (non-capturing across a boundary is fine; capturing inline is fine; capturing across the boundary dies under both `callptr` and `fncall1`) and filed at `issues/archived/2026-08-10-cyrius-capturing-closure-across-fn-boundary.md` and **fixed in 6.5.17** (see that entry above) — so this particular blocker is retired and a capturing comparator would work today. Deferred on two remaining grounds: the API mismatch above; CLAUDE.md's "wait for the third instance" rule (this is the first); and the fact that any swap is a hot-path perf change that would need before/after benchmark numbers, which a maintenance bump should not be smuggling in. Revisit if a second/third keyed-sort site appears, or if upstream adds a context-carrying comparator. `vec_select_nth` has no call site in hisab today.
- **6.4.69** (v2.6.10): clean 3-patch bump from 6.4.66. **No library source change** — all 34 modules compile clean; `dist/hisab.cyr` byte-identical apart from the version header. Re-vendored via `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.4.69; the transitive `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed). `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. Stdlib delta was **three files**, all upstream bug/hardening fixes: `fmt.cyr` (`fmt_hex`/`%x` `> 0`→`!= 0` so high-bit-set values render every digit; `fmt_float_buf` non-finite guard → `inf`/`-inf`/`nan` instead of a garbage `-.00000-` token — **hisab links this** via `symbolic`/`symbolic_ext`, byte-identical for finite in-range values so tests stay 957/957), `math.cyr` (float-parse exponent saturates at 340, O(exp)→O(1), closing a `"1e100000000"` ~237 ms algorithmic-complexity DoS; behaviour-preserving), and `syscalls_x86_64_agnos.cyr` (`sys_reboot()` widened nullary→4-arg `power_sys` for agnos 1.55.25 — agnos-only variant, no impact on hisab's x86_64-linux target). sakshi unchanged at **2.4.6** (already the latest tag). Tracked-issue re-verify on the new pin (minimal repros): interval-ident-lex **still live** (`var iv_add` → `expected identifier, got unknown`); for-empty-clauses **still live** (`for (; …)` → `unexpected ';'`); cli-arg-clobber not re-tested (destructive). No new fixes.
- **6.4.66** (v2.6.9): bump from 6.3.11 (a full minor across 55 patch releases). **No library source change** — all 34 modules compile clean; `dist/hisab.cyr` byte-identical apart from the version header; a consumer including the full bundle compiles + runs end-to-end. Re-vendored via `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.4.66; the transitive `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed this bump, unlike 6.3.11). `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. The 6.4.66 stdlib snapshot is much larger than 6.3.11's (new `async`/`tls`/`sandhi`/`regex`/`http`/… modules) but the declared subset hisab pulls is unchanged. **Fixed** a pre-existing `tests/modules.tcyr` compile failure surfaced (not caused) by this bump: its interval result vars `iv_add`/`iv_sub`/`iv_mul` collide with reserved cycc SIMD intrinsic names, unusable as variables (`expected identifier, got unknown`; reproduces on 6.3.11–6.4.66) — renamed to `iv_sum`/`iv_diff`/`iv_prod`, suite restored to 312/312, full run 957/957. New tracked issue: `issues/archived/2026-07-17-cyrius-interval-ident-lex.md`. Tracked-issue re-verify on the new pin: for-empty-clauses **still open** (unchanged rejections on 6.4.66).
- **6.3.11** (v2.6.7): infrastructure-only bump from 6.2.11. **No library source change** — all 34 modules compile clean; `dist/hisab.cyr` byte-identical apart from the version header. Stdlib delta touched `assert`/`bench`/`fnptr`/`io`/`math` + the `syscalls` platform variants (`ganita` unchanged); `lib/result.cyr` (transitive dep of `io`/`tagged`) picked up the 6.3.11 `_die` agnos-portability fix (was a bare `syscall(60,1)` that no-op'd → failed-open on agnos; now target-guarded). 6.3.x CLI split: **`cyrius deps`** resolves git deps only (commit-pins sakshi in the lock), **`cyrius lib sync`** (no `--full`) vendors the declared stdlib subset — superseding the 6.2.x `cyrius deps`-does-both flow. Every vendored stdlib file byte-matches 6.3.11; `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30. 957/957 tests, all gates green. Tracked-issue re-verify: for-empty-clauses **still open** on 6.3.11; no new fixes (3 prior fixes stay archived).

**Watching upstream:**
- **RISC-V rv64** — the 4th platform peer. This entry read "5.7.11" for four minors; it has slipped repeatedly since and is now re-homed to **v6.7.x / v6.8.x** (upstream `docs/development/roadmap_6.md:176`, theme set 2026-07-07). Not landed as of 6.5.18, and the cyrius CHANGELOG carries no rv64 mention newer than its 6.2.0 section. Watched, not blocking: hisab is pure math with no target-specific code, so the only expected surface is another `syscalls_*` platform variant vendored for snapshot parity.

## Cyrius stdlib modules (15 declared, 31 vendored)

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

The rows above are exactly the 15 names in `cyrius.cyml [deps] stdlib` (three rows group two or three names each). They expand to **27** files on disk — `syscalls` to 7 platform variants, `alloc` and `args` to 4 each — and `lib/` holds **30**: those 27 plus the transitive `result.cyr` / `atomic.cyr` and the vendored `sakshi.cyr`. `cyrius lib sync` vendors the 27; the other three are not in the declared subset and it does not touch them.

## sakshi (first-party dependency)

**Status:** `sakshi` **2.5.1** via git, modules path `dist/sakshi.cyr` (bumped 2.4.11 → 2.5.1 in v2.11.3; commit-pinned in `cyrius.lock`, and byte-identical to what cyrius 6.6.1 folds into its own `lib/` — same SHA256, 76,277 B, so the vendored copy and the pinned dep agree rather than one shadowing the other). Its shipped surface still links only `fnptr` + `atomic`.

**2.5.0 / 2.5.1 change no public surface.** Diffing the exported `fn` list across the bump shows
only private helpers moving: `_sk_bin_read_hdr`, `_sk_bin_write_hdr`, `_sk_memset` and `_sk_ring_put`
removed (2.5.0's "four dead functions"), `_sk_aring_dropped_count` added. Every `sakshi_*` entry
point is unchanged, so the bump is safe for hisab by inspection as well as by test. The releases
themselves are audit-and-fix work — a P(-1) sweep (50 findings, 14 refuted, 12 repaired), a Windows
PE clock that emitted timestamp 0 and could stall 10 ms per log call, an oversized ring event that
overwrote its own header, and AGNOS timestamps frozen for a whole program run.

**Nothing in hisab calls sakshi.** It is declared, vendored and version-tracked, but there is no `sakshi_*` call site in `src/`, `tests/` or `examples/`, and no emit hook is registered — so every release below is recorded for the dependency trail, not because a behaviour reached this repo.

- **2.4.11** (v2.11.2) — a single upstream commit (`515a57f`, "repairing defect in span") touching `src/span.cyr` and its tests; the change is confined to the span stack. **No public API change and no effect reachable from hisab** — see the standing note above: hisab declares, vendors and version-tracks sakshi but has no `sakshi_*` call site in `src/`, `tests/` or `examples/`. Vendored, commit-pinned, `deps --verify` 30/30.
- **2.4.10** (v2.9.2) — `sakshi_log_kv` composed `msg + ' ' + key + '=' + value` into a 256-byte scratch **before** calling `_sk_emit`, so an `SK_OUT_HOOK` subscriber received one flat string with no way back to the pieces (`"deploy failed reason=oom"` was indistinguishable from a message that merely contains that text). The message is now passed unflattened with a fields block in the hook's sixth argument (`count` at offset 0, then 32-byte key-ptr/key-len/val-ptr/val-len records); `level` discriminates, and the block **lives on the caller's stack** — a subscriber keeping a field must copy the bytes. ⚠ **Behaviour change for existing hook subscribers**, scoped to `sakshi_log_kv` + `SK_OUT_HOOK`; stderr, file, ring, atomic ring and UDP still receive the composed text byte for byte. Also: `sakshi_log_kv` truncated at 256 bytes in silence and now returns the number of bytes that did not fit (0 = everything emitted, so existing callers that ignore the return are unaffected).
- **2.4.9** — committed upstream but **never tagged**; folded into 2.4.10 rather than released, because its benchmark numbers had been measured against a stale toolchain (the repo's own pin read 6.5.0 while the installed compiler was 6.5.15).
- **2.4.8** (v2.9.0) — the upstream half of cyrius 6.5.8's `i64::MIN` sweep: `_sk_fmt_int` emitted a bare `"-"` for the most negative i64 (`n = 0 - n` is a no-op there), same defect and same fix as `lib/fmt.cyr`.
- **2.4.7** (v2.6.11) — a **latent-bug fix with no public API change**: two private helpers were renamed — `_sk_write_int` → `_sk_buf_write_number` and `_sk_write_str` → `_sk_buf_write_string` — because `_int` / `_str` / `_cstr` are **reserved cyrius overload-dispatch suffixes**. The compiler rewrites a call `X(a, …)` into `X_int` / `X_str` when arg 1 matches that type and the sibling exists, so `_sk_write_int` was squatting the dispatch slot of the unrelated `_sk_write(buf, len)` flush function; any `_sk_write(someInt, …)` would have been silently redirected into a 4-parameter function with the missing args bound to garbage. Latent, not live, because every real call site passes a buffer address first. Both renamed symbols are `_sk_`-private, so hisab links nothing that moved.

  > **Checked in hisab as a result** (2026-08-03, re-confirmed 2026-08-09): the same defect class does not exist here. `src/` defines exactly one function carrying a reserved dispatch suffix — `expr_to_str` (`symbolic`) — and there is **no** `expr_to` base function for it to shadow, so no dispatch slot is claimed. Re-run on any new `*_int` / `*_str` / `*_cstr` definition: the rule is that a suffix-named function must be the same-arity type variant of its base, or not carry the suffix at all.

**Purpose:** Structured logging (timestamps, levels, categories)

**Risk:** Low, and currently zero at runtime. Logging is write-only — no data flows back — and with no call site in this repo a sakshi behaviour change cannot reach hisab at all; the exposure is limited to the dep resolving, compiling and staying lock-verified. That will stop being true the moment the first `sakshi_*` call lands, so this section is kept current against the tag rather than against usage.

## Rust-era dependencies (archived in pre-2.0 git tags)

These are no longer used but documented for reference:
- `glam` 0.29 — replaced by hisab's own vec/mat/quat types
- `serde` 1.0 — no serialization in Cyrius port (yet)
- `thiserror` 2.0 — replaced by ERR_* integer codes
- `tracing` 0.1 — replaced by sakshi
- `reqwest` 0.12, `tokio` 1.0 — AI module not ported
- `rayon` 1.0 — parallel module not ported
- `criterion` 0.5 — replaced by bench.cyr
