# Changelog

## [Unreleased]

## [3.1.1] - 2026-09-14 — cycc 6.6.4: all four 2026-09-13 filings repaired upstream, the gate inverted on its own

Toolchain **6.6.3 → 6.6.4**. sakshi **2.5.2** and ganita **1.2.5** are already the latest tags
(checked against the GitHub API, not assumed). No library source change; the five suites print
**byte-identical** output, **4214/4214**.

⭐ **6.6.4 repairs all four defects hisab filed on 2026-09-13**, and the one hisab could see from its
own tree announced itself: `scripts/check-public-surface.sh` was built to FAIL the day the
`public enum` leak was fixed, and on the pin bump — before any source changed — claim 3 reported
`_ad_pow` **REFUSED** (456/457 → **457/457** non-public probes refused). The `KNOWN_LEAKS` allowlist
is now empty. The other three (`&_private_fn` reachable via address-of, `deps` re-locking silently
under an unchanged pin, `install.sh --refresh-only` writing a released slot) are closed upstream
with gates; hisab's consequences are recorded below and in the roadmap, and the vendoring rule
(byte-compare `lib/` against the cyrius **tag**, never the install dir) stays — it is what found two
of them.

### Changed
- **cyrius.cyml** — `cyrius = "6.6.4"`.
- **lib/** — six files move with the 6.6.4 tag, all in the syscall/io layer: `io.cyr` (+22/−13:
  per-target `O_NOFOLLOW`/`O_EXCL` bridging on agnos, `xflock` collapsed onto `SYS_FLOCK`) and the
  five `syscalls_*` peers (per-target `O_DIRECT`/`O_LARGEFILE`/`O_DIRECTORY`/`O_NOFOLLOW`,
  `SYS_FLOCK`). All **30/30** stdlib files byte-match the 6.6.4 tag; `sakshi.cyr` byte-matches 2.5.2.
- **cyrius.lock** — six hashes moved with those files, and the lock now ends with the 6.6.4
  `cyrius	6.6.4` trailer (the pin-keyed guard against silent re-locking that hisab's filing asked
  for). `deps --verify` 31/31.
- **scripts/check-public-surface.sh** — `KNOWN_LEAKS=""`; the `&name` and known-leak comment blocks
  record the 6.6.4 closure. The probes stay CALLS: a consumer writes a call, and a gate that only
  worked from 6.6.4 up would be blind on every pin below it.
- **docs/development/roadmap.md** — *Toolchain, tracked upstream*: the four-row open table → closed
  in 6.6.4; the *Public / private function surface* row's two upstream holes → fixed, so the 4.0.0
  `private` flip is gated only on hisab's own dispositions now.
- **docs/development/dependency-watch.md** — 6.6.4 block.
- **docs/development/issues/archived/2026-04-26-cyrius-cli-arg-clobbers-source.md** — the one archived
  record still marked *Open* is CLOSED: the data loss has been guarded upstream since cyrius
  **6.0.36** (`cbt/commands.cyr` refuses to write build output over a `.cyr` path — exit 1, source
  intact, measured on 6.6.4 in a scratch dir). hisab carried "Open" through the entire 6.x line. The
  misparse (an unknown flag before the subcommand shifts positionals) is still present and stays
  recorded as such. `docs/development/issues/` has **0** open filings; all 33 archived records now
  carry a closed status.
- **dist/hisab.cyr** — regenerated; the diff is the version header alone (1,115,949 B, 4.43% of the
  24 MiB expanded-source cap; both caps re-checked unchanged in source between the 6.6.3 and 6.6.4
  tags — `lex.cyr` moved only for the 64 KB string-literal fix).

### Not exposed — checked rather than assumed
- The 64 KB string-literal fix: hisab's largest literal is far below it.
- `#deprecated` now firing on tail calls: hisab declares no `#deprecated` fn and every build/check
  is warning-free except one pre-existing line (below).
- `public impl` now marks no method: hisab has no `impl` blocks.
- The eight newly-checked visibility paths (`&fn`, `s.method()`, struct-returning receives): no
  in-tree consumer crosses a module boundary through any of them — the public-surface gate's 890
  reachability probes and 457 refusal probes are unchanged.
- Binaries are **not** byte-identical this time (+32 B on every suite and the CLI; `abuse` +4,128 =
  one 4 KiB alignment crossing + 32). The six `lib/` files that moved are compiled into every
  binary, so this is expected and not claimed as anything. Suite OUTPUT is byte-identical.
- ⚠ Pre-existing, not new: `cyrius check --with-deps tests/foundation.tcyr` prints
  `673:69: comparison mixes f64 and integer operands` on 6.6.3 AND 6.6.4 — a raw `!=` between a
  user fn call (return type untracked → tagged integer) and an f64 intrinsic result. The comparison
  is the intended bit-exact one. The 3.0.1 baseline grep (`OK|error`) hid it; CI's `check` step
  scopes to `dist/` + `examples/` and greps `must_use`, so it is not a gate failure. Left as is.

### Performance
**No change is claimed, and this run cannot support one either way.** The 3.1.1 rows in
`bench-history.csv` were recorded with the box at load 2.8 → 3.1 (a `qemu-system-x86` and two `cycc`
processes from an unrelated cyrius session each at 100% CPU): median **+9.84%**, mean +9.99%,
**34 of 78** rows past 10%, every row UP including the flat numeric kernels that no toolchain change
can touch — the same uniform shift 2.16.0 and 2.19.0 recorded under load. Treat these rows as a
contaminated sample, not a baseline; the next quiet-box run should be compared against the
2026-09-13 3.1.0 rows, not these.

## [3.1.0] - 2026-09-13 — the `pub fn` half: 729 declarations, a gate whose first draft proved nothing, and two more upstream holes

**Non-breaking.** Every non-underscore top-level fn / struct / enum / var in `src/` now carries
`public` — **729 declarations: 660 fn, 22 struct, 39 var, 8 enum** — plus the **24** underscore-named
helpers another module reaches, each with a marker comment naming its consumers. No module is
`private`, so the annotation is the documented no-op: `foundation` and `edge_cases` compile
**byte-identical** to the 3.0.1 build, bundle `code_size` **768,312 B** both sides. Suites **4214**
(modules 2086 → 2098). cycc 6.6.3 (the `#derive` + `public` fix 3.0.0 was blocked on).

⭐ **THE SURFACE IS NOW PROVEN, NOT DECLARED.** `scripts/check-public-surface.sh` flips all 35 modules
`private` in a scratch copy on every CI run and establishes five claims, fail-closed: (0) naming
completeness, (1) per-file internal completeness, (2) a generated consumer that CALLS every public fn
(arity read from the declaration), every derived accessor, every public var and enum member — **890
probes, all reachable**; (3) exactness — every non-public item refused (**456 of 457**, see below);
(4) `examples/*.cyr` clean under the flip. **Four mutants killed**: `_perm` unmarked → claim 1 (42
sites); `hvec3_new` unmarked → claims 0, 1 (54) and 4 (20); a `_` helper placed after a `public enum`
→ claim 3 (unexpected leak); a spacer before `_ad_pow` → claim 3 (stale allowlist).

⛔ **THE GATE'S FIRST DRAFT WAS VACUOUS, AND A SURVIVING MUTANT SAID SO.** Claim 1 checked the flipped
**bundle** — and `dist/hisab.cyr` is ONE file, `private` is per-file, so every "cross-module" call
inside it is an in-file call that can never be refused. Dropping `public` from `_perm` left it green.
Claim 1 now includes each module as its own file, the way the suites do. ⭐ **That vacuity is also a
finding about the flip itself**: a flipped bundle's 35 `private` lines face only a consumer, for whom
the surface is already exactly the 729 items — **the consumer-facing half of 4.0.0 costs nothing
further; the entire remaining cost is hisab's own suites.** Measured: **64 distinct `_` names, 387
sites** — `hisab.tcyr` 14/4, `modules.tcyr` 372/59, `hisab.bcyr` 1 — where the roadmap row had said
"52 functions, 235 sites" and counted neither globals (8) nor the benchmark harness.

⛔ **TWO UPSTREAM HOLES, BOTH FILED WITH SELF-PROVING REPROS** (`cyrius/docs/development/issues/2026-09-13-hisab-*`):
1. **`&_private_fn` from another file compiles and RUNS** — exit **42** through `callptr` and
   `fncall1` — while the direct call is refused. The check covers calls and var reads, not
   address-of. Found because the gate's obvious design ("take `&name` of every public fn") had a
   negative control that would not fail; the probes are calls for that reason.
2. **A `public enum` leaks `public` onto the next top-level declaration** (6.6.2 and 6.6.3; fn or
   var; one-line or multi-line; a non-public enum, `public var`/`fn`/`struct` do not). Found because
   exactness read **456 of 457**: the one accepted was `_ad_pow`, the declaration after
   `public enum AdPowLimit`. `_SYM_EPS` after `ExprTag` is the second hisab item in that position,
   masked by its own marker. `_ad_pow` is a **known leak** in the gate keyed to the filing, and the
   gate **fails the day upstream fixes it** (mutation-proven with a spacer declaration).

⛔ **TWO GATES WENT BLIND TO THE ANNOTATION AND ONE OF THEM HAD NEVER RUN.**
`check-constants.sh` matched `^var` and so reported a **green 143/143** over a population of 160 —
17 constants silently dropped out of the gate the moment they became `public var`, the third
recorded instance of this gate reporting a confident N/N over a shrunk population. It accepts the
prefix now and carries a **population floor** (160) that fails on any shrink, verified to fire on the
regressed regex. `check-result-migration.sh` failed in BOTH directions on `public fn` — 202 false
positives AND 29 of 48 migrated functions missed — and **it had never been wired into CI** since
3.0.0 shipped it mutation-proven; both regexes widened, reads 48 / 0 on the 3.0.1 tree and on this
one, and it runs in CI now with its `--selftest`.

⛔ **THE REVIEW FOUND DEFECTS THE ANNOTATION MERELY UNCOVERED.** A 7-reviewer / 3-refuter pass
over the real diff (53 findings, 0 refuted, 3 dissenting votes in 159):
- `#must_use` in `vec2`/`vec3`/`vec4` had sat on the private `_hvecN_len_scaled` helper **since
  2.17.0** inserted it between the directive and `hvecN_length`; the public function carried none.
  Moved. `ad_grad_into` is fallible (`?` plus a tail-returned `Result`) and had none. Added — which
  surfaced that **the three reverse-mode benchmark rows discarded `ad_grad`/`ad_grad_into`'s
  `Result`**, the exact rows whose silent tape overflow was 2.20.0's pre-tag finding. They now count
  failures and the harness exits 1: with the tape shrunk back to the 2.20.0 size the row reads
  **333 µs against 1,114 µs** and reports FATAL.
- **`tests/modules.tcyr` passed 24-byte `HVec3` values into `bch_so3_2nd_order`/`_3rd_order`, which
  read 72-byte `Mat3` operands** — a 48-byte over-read per operand that "passed" on the bump
  allocator's next bytes, then asserted `HVec3_x` of a matrix. Rewritten with `hat(u)` operands and
  closed forms: commuting → exactly `hat(1.5 e_x)`, bit-exact; non-commuting `a = 1/2, b = 1` →
  `Z2 = hat(a, b, ab/2)` exact and `Z3 = hat(11/24, 47/48, 1/4)` to `ULP_TOL_M`. **Two mutants die
  on exactly the assertions that encode the mutated coefficient** (commutator ×1: `z = 1/4` fails;
  1/12 → 1/6: the 11/24 and 47/48 rows fail). The comment calling it "the vector one" was false.
- Doc defects on the newly public surface: five module headers (`num`, `num_ext`, `ode`, `optimize`,
  `linalg_precision`) still stated the pre-3.0.0 integer error-code contract; `ode_symplectic_euler`'s
  doc block was a design monologue with four contradictory return conventions (the code has always
  returned a 16-byte `[q, p]` block); `einsum`'s header said 8 labels a–h against a 26-label
  constant since 2.6.12; `collision_mesh.cyr` had no file header at all since the 2.2.2 split;
  `_lie_norm3`'s doc block opened with `su2_from_quat`'s orphaned line.

⭐ **The 4.0.0 decision list is now worked, not owed.** The roadmap row carries a per-item
disposition for all 24 cross-module helpers, each grounded in the callee body and caller sites, and
several are not among the three options the row had offered: three families should simply **retire
the reach** (`_GEO_F64_POS_INF` is an alias of the public `F64_POS_INF`; `_noise_fade` is
bit-for-bit `ease_in_out_smooth`; `_COL_F64_ZERO/_ONE/_NEG_ONE` are literals), `_COL_SENTINEL`
should be **promoted and relocated** (it is the half-edge data contract, never used by
collision_core), the `_epa_*` quartet wants a section-level **merge**, the SU(2) accessors want the
**operation** promoted (`su2_rotate_vec3`) rather than the raw allocator that exists precisely because
every constructor normalises, and symbolic's five reaches collapse to **two extracted functions**.
Five non-underscore names the review judged implementation details (`EPSILON_F32` — its own comment
says unused; `F64_1E_NEG30`, `F64_NINE`, `F64_THREE..F64_FIFTEEN`, `F64_SIX_DG`, `RenderLayout`) are
listed for an underscore rename before the flip. And **11 public items are reached by no test**,
including `ad_neg`/`ad_cos`/`ad_ln`, `csr_new`, `bch_3rd_order`, `hodge_star_2form_4d` and all seven
`GEO_JET_*` tags — a 3.2.0 row.

⚠ **Instruments that were wrong first, this release**: a zsh `for m in $MODS` that never split the
list (both "0 violations" numbers it produced were meaningless — re-run under bash); a nested `EOF`
that ended a heredoc early; `PIPESTATUS` in zsh; a repro verdict grep that matched the routine
"deps locked" line; a struct-literal probe that SIGSEGV'd on the control too (a value passed where an
accessor wants a pointer — not a compiler bug, and not filed); and a `LOOSE_TOL_M` I called "not
1e-8" in a comment when it is exactly that — corrected before it shipped.

### Added
- `public` on 729 top-level declarations across all 35 `[lib]` modules; marker comments on the 24
  cross-module `_` helpers naming their consumers and the 4.0.0 decision owed.
- **scripts/check-public-surface.sh** — the five-claim flip gate above; CI step *Public surface is
  complete and exact*. Its header records why the probes are calls and why claim 1 is per-file.
- CI step *Result migration has no vacuous call sites* — `check-result-migration.sh` +
  `--selftest`, never wired since 3.0.0.
- `tests/modules.tcyr` — BCH on so(3) fixture with `hat`/`vee` helpers, skew-symmetry check, and
  closed-form expectations (+12 assertions); the over-reading fixture it replaces is described in
  place.
- `tests/hisab.bcyr` — `_B_AD_FAIL` counter and a FATAL exit for the three reverse-mode rows.

### Changed
- `scripts/check-constants.sh` — accepts `public var`; **population floor** 160.
- `scripts/check-result-migration.sh` — both declaration regexes accept `public fn`.
- `#must_use` moved onto `hvec2_length` / `hvec3_length` / `hvec4_length`; added to `ad_grad_into`.
- Module headers of `num`, `num_ext`, `ode`, `optimize`, `linalg_precision` state the `Result` contract;
  `ode_dopri45`'s doc says why it carries no `#must_use`; `ode_symplectic_euler`, `einsum`,
  `collision_mesh` (new file header), `lie.cyr` doc blocks corrected as above.
- Roadmap: the public/private row is now *The `private` flip — [4.0.0]* with the measurements and
  dispositions; the other rows that had been targeted at 3.1.0 (struct-layout gate, Abaco table,
  `lerp` SIMD hybrid, `vec_sort_by`) move to **3.2.0**; new row *Public API reached by no test*; the
  *Toolchain, tracked upstream* table — which still listed the two 2026-09-11 filings as open after
  3.0.1 closed them — now lists the six open 2026-09-13 filings and what each costs.

### Performance
No change, by construction: the library binaries are byte-identical. One bench run as a
same-binary control against the two 3.0.1 runs: median **−0.51%**, 0 of 78 rows past 10%.


## [3.0.1] - 2026-09-13 — cycc 6.6.3: both 2026-09-11 filings closed, and the pin's snapshot was not the pin

Toolchain **6.6.2 → 6.6.3**, sakshi **2.5.1 → 2.5.2**, ganita **1.2.4 → 1.2.5**. No library source
behaviour change: the suites and the CLI compile **byte-identical** under both compilers (`cmp` clean
on `foundation` 442,040 B, `edge_cases` 586,464 B, `build/hisab` 257,176 B), suite output is
byte-identical, **4202/4202**, every gate green, lock 31/31 and now written in sorted order.

⭐ **6.6.3 repairs both defects hisab filed on 2026-09-11, and each is closed HERE, not just upstream**
— the cyrius agent never edits this repo, so a filing it fixed stays "blocked" in hisab's roadmap and
"do not tidy" in hisab's source until hisab closes it. Both closures are recorded as
`docs/development/issues/archived/2026-09-11-cyrius-*.md` with the paired measurement, and both were
verified as a **pair from directories pinned to each version**, not read off the changelog:

| filing | 6.6.2 | 6.6.3 |
|---|---|---|
| nested `continue` binds one level out — hisab's reproducer | exit **1** | exit **0** |
| same — upstream gate `nested_continue_binds.tcyr` | **2 / 8** | 8 / 8 |
| same — `_cga_build_null_tbl` in its natural `continue` form, inner `continue` firing **1024×** | FNV mismatch, **1024 / 1024** coefficients flagged | contract table exact, 2088 / 2088 |
| `#derive` + `public` — hisab's idiom (`alloc(sizeof(P))` + derived setters), consumer reads getters and calls a setter cross-file | **compile error** | builds, exit **0** |
| same — consumer calling the file-private helper | (masked by the error) | refused, `'_hidden' is private to its file`, no binary |

⚠ **The first run of the `continue` pair read 8/8 on BOTH toolchains.** `~/.cyrius/versions/6.6.2/bin/cyrius`
re-execs to the **cwd's manifest pin** (its documented contract; inside the cyrius repo it uses
`./build/cycc`), so run from the cyrius checkout it compiled with 6.6.3. From a scratch dir pinned
6.6.2 it read 2/8. **Check the probe before believing the probe** — `cyrius build -v` prints the
resolved compiler and that line is the measurement.

⛔ **THE INSTALLED "6.6.2" SNAPSHOT WAS NOT 6.6.2, AND A BARE `cyrius build` SILENTLY MOVED A
VENDORED DEP ACROSS THE PIN.** hisab's docs said ganita 1.2.4 under 6.6.2; the committed
`lib/ganita.cyr` agreed; the cyrius **tag** 6.6.2 agrees. But `~/.cyrius/versions/6.6.2/lib` held
6.6.3's twelve refolded stdlibs, byte-identical to `versions/6.6.3/lib` (`ganita.cyr` mtime
2026-09-12 08:49, two days after 6.6.2 shipped) — cyrius's dev loop, `install.sh --refresh-only`,
writes the repo's in-progress `lib/` into `versions/$(cat VERSION)/lib`, and VERSION still named the
released 6.6.2 during 6.6.3 development. The baseline build for this release, run under the
UNCHANGED 6.6.2 pin, then rewrote `lib/ganita.cyr` 1.2.4 → 1.2.5 and re-locked its hash
`d4aaa7da…` → `fae5a807…` while printing exactly what a no-op prints; `git status` was the only
thing that noticed. ⭐ **Both halves are filed upstream with a self-proving repro** —
`cyrius/docs/development/issues/2026-09-13-hisab-refresh-only-overwrites-released-snapshot.md` and
`…/2026-09-13-hisab-deps-relocks-silently-under-unchanged-pin.md` (repro script: one comment line
changed in a throwaway pinned snapshot → `cyrius build` re-vendors it, re-locks it, and
`deps --verify` reports the mutation **verified**; exit 1 on 6.6.2 and 6.6.3). ⚠ **The vendoring
check this repo has relied on since 2.6.11 — "byte-compare `lib/` against the pin's own snapshot" —
was comparing against a mutable directory.** It now compares against `git show <pin>:lib/<file>` in
the cyrius repo: 30/30 stdlib files match the 6.6.3 tag, `sakshi.cyr` matches its 2.5.2 tag.

⚠ **Two candidate filings were checked and NOT filed, because both were my probe.** A struct
**value** passed to a derived accessor that takes a pointer SIGSEGVs (`P_x(&p)` = 3, `P_x(p)` = 139,
on the control too) — the language is untyped there and hisab's own idiom never does it; and the
wrapper's pin re-exec is its contract, not a defect. A wrong filing costs the agent the same
investigation twice.

### Changed
- **cyrius.cyml** — `cyrius = "6.6.3"`; `[deps.sakshi] tag = "2.5.2"`.
- **lib/** — `ganita.cyr` 1.2.4 → 1.2.5, `sakshi.cyr` 2.5.1 → 2.5.2: each diff is **one header
  line**, which is why the compiled output is byte-identical. All 30 stdlib files byte-match the
  cyrius 6.6.3 **tag**; sakshi byte-matches its 2.5.2 tag.
- **cyrius.lock** — 31 entries, now in sorted order (6.6.3's `_deps_lock_dir` fix); 3 entries moved
  order-insensitively (sakshi commit + hash, ganita hash). `deps --verify` 31/31.
- **src/geo_advanced.cyr** — `_cga_build_null_tbl`'s `if`-guard comment no longer claims the
  workaround is mandatory. The guard form is **kept**, for a stated reason: consumers compile
  `dist/hisab.cyr` under their **own** pins, none had crossed 6.6.2, and the `continue` form is
  correct only from 6.6.3 and fails **silently** below it. Same disposition as `m4_mul_vec4` in 2.11.5.
- **docs/development/roadmap.md** — *Public / private function surface* row: ⛔ BLOCKED UPSTREAM →
  ⭐ UNBLOCKED (cycc 6.6.3, verified 2026-09-13); target versions unchanged (`pub fn` 3.1.0,
  `private` flip 4.0.0), and the row's own landmine (a consumer-call gate must exist before the flip)
  still applies. Consumer row now names the reason the pin matters to a consumer.
- **docs/development/issues/archived/** — two new hisab-side records for the closed upstream
  filings (`2026-09-11-cyrius-nested-continue-binds-to-wrong-loop.md`,
  `2026-09-11-cyrius-derive-cannot-combine-with-public.md`).
- **docs/development/dependency-watch.md** — 6.6.3 block.
- `bench-history.csv` / `benchmarks.md` — four rows per benchmark (2 × 6.6.2, 2 × 6.6.3) on the
  3.0.0 tree, which had no baseline of its own (the last rows were from mid-migration `a6362ce`).

### Not exposed — checked rather than assumed
- 6.6.3's headline `#inline`-disarms-`#derive` fix: hisab has **zero** `#inline` directives (the one
  grep hit is inside a comment), which is why the bundle compiled on 6.6.2 despite `mat4.cyr`
  preceding eight deriving modules in `[lib]` order.
- `CYRIUS_DCE` PT_LOAD fix: hisab's scripts and CI never set it.
- `cyrius distlib` OOM fix: hisab's 35-module bundle always completed; regenerated bundle differs
  from 3.0.0's by the version header only.
- The expanded-source (24 MiB, `op > 25165824`) and token (`tc >= 4194304`) caps are **unchanged in
  source** between the tags — `lex.cyr` untouched, `lex_pp.cyr`'s 53 inserted lines are the `public `
  prefix probe. Bundle 1,103,166 B = **4.38%** of the byte cap (CLAUDE.md said ~4.2%, cyrius.cyml
  ~4.1%; both corrected).

### Performance
**No change is claimed, and for once it is a fact rather than a measurement**: the binaries are
byte-identical, so the four bench runs on a quiet box (load 0.3–0.7) are a same-binary control —
median **+0.68%**, mean +0.84%, **0 of 78** rows past 10%, no row past 2× its own run-to-run spread.
The largest movers (`spatial_hash_query_2k` −8.0%, `bvh_query_ray_200x4k` −6.3%) sit inside their own
6.6.2-side spread (7.7%, 15.1%); `ease_in_out` +8.3% is 6 → 6.5 ns quantisation.

### Changed — documentation sweep, 2026-09-11 (shipped in this release)
- **Documentation sweep, 2026-09-11 — no behaviour change; all 4202 assertions pass on both sides.**
- `docs/development/roadmap.md` is **future-facing only**, **1308 → 297 lines**. Removed: 38
  completed `[x]` items, 19 struck-through release-train rows, the 70-row **Release History** table,
  the arc-narrative sections and every closed toolchain row. **The per-version record now lives only
  in this file** — the removed table was a third copy of it, and maintaining three copies is how the
  `triangulate_polygon` direction stayed backwards for thirteen releases in the one copy no gate
  could reach. ⭐ **Four open items were extracted from prose that was already in the file but was
  not listed as work**: the struct-layout contract whose 32 assertions are all `sizeof(T) > 0` and so
  cannot fail, the Abaco boundary table frozen at the 2.2.0 surface, getting a live consumer onto
  3.0.0, and the `hvec3_lerp` SIMD hybrid parked under `cross` despite never being gated on lane
  shuffles (**25 ns → 19–20 ns, bit-identical**). The 3.0.0 public/private item is re-scoped: the
  `pub fn` half is non-breaking and belongs in a 3.x, the `private` flip is breaking and moves to
  4.0.0, and both stay blocked on the upstream `#derive`/`public` defect.
- ⛔ **Every line-number citation carried forward was re-derived and ALL SIX HAD DRIFTED.** The six
  hand-rolled ordering routines are `collision_core.cyr:530`, `spatial.cyr:114`, `num_ext.cyr:309`,
  `linalg_ext.cyr:1071`, `linalg_precision.cyr:1219` and `:1753`; the roadmap had been pointing at
  `collision_core.cyr:466`, `linalg_ext.cyr:905`, `linalg_precision.cyr:816` and `:1265`, **none of
  which is an ordering routine today**. A seventh drifted citation was found in the source itself —
  `linalg_ext.cyr` cited `linalg_precision.cyr:1334` for `eigen_qr`'s sort, which is now a comment
  about wrong-sign eigenvalues ~420 lines away — and is replaced by a citation **by block name**,
  since a number cannot be kept true by a gate that does not exist.
- `README.md`: the **Security** cell had accreted to **12,724 bytes** of per-release arc narrative
  inside one table cell and is now **730 bytes** of durable posture. The 3.0.0 breaking notice moved
  out of it into its own section above **Modules**.

### Fixed — documentation sweep, 2026-09-11
- ⛔ **README's Quick Start still showed the PRE-3.0.0 calling idiom** — `calc_integral_simpson(...)`
  and `num_newton(...)` with their returns discarded. That is now a `#must_use` warning, and the same
  shape in *argument* position is the silent-tag trap this release exists to remove, so the README
  was teaching the one mistake the migration guide leads with. Both rewritten to bind both halves.
- ⛔ **19 doc comments still stated the pre-3.0.0 contract on functions the migration changed** —
  `# Returns: 0 on success, HSB_ERR_X …` above functions that now return `Ok(0)` / `Err(...)`
  (`num_fft`, `num_ifft`, `num_dct`/`idct`, `num_dst`/`idst`, `num_fft_2d`/`ifft_2d`,
  `num_tridiag_solve`, `num_crt`, `_numx_dft`, `num_newton`, `num_bisection`,
  `calc_integral_trapez`/`_simpson`, `svd_truncated`, `eigen_power`, `lyapunov_max`, `ad_grad`).
  `src/error.cyr`'s module header said the same thing for the whole code set. **A doc comment stating
  a retired contract is worse than none: it reads as current and no gate can see it.** ⚠ Classified
  by whether the function body actually returns `Ok`/`Err` rather than by grep alone — `solve_pgs`
  and `svd_compute` still return a plain `0` and their comments are correct, so they were left.
- `README.md` Building block: every test count was a release stale (454/379/2041/233/775 + 72
  benchmarks → **550/413/2086/239/914 + 78**, plus the fuzz line). The consumer manifest snippet
  omitted `"result"` from `[deps] stdlib` and claimed **15** sidecar leaves where `dist/hisab.deps`
  names **16**. `CLAUDE.md` and `cyrius.cyml` carried a stale bundle weight, re-derived to
  **1,103,166 B / 26,457 lines** after this sweep's own comment edits, and `CLAUDE.md`'s
  doc-structure line still described the roadmap as holding "completed items".

## [3.0.0] - 2026-09-11 — Result<T,E>, and two roadmap premises that measurement refuted

**Breaking.** The integer-error-code convention is replaced by the Cyrius stdlib `Result<T, E>`.
**48 functions change signature**; nothing else about their behaviour moves. Suites **4202**.
Migration guide: `docs/guides/migration-3.0.md`.

⛔ **THE PREMISE THIS RELEASE WAS PLANNED ON WAS FALSE.** The roadmap said `?` on a plain i64 fn
"compiles clean and SIGSEGVs with no diagnostic" and concluded there was **no incremental path**.
Measured on cycc 6.6.2, it fails at **compile time**: the `?` line warns with the fix named, the
call site is a hard error (*"a `: stack` enum returns two values — bind both"*), and no binary is
emitted. Migrating a function together with its callers is safe, because a half-migrated **binding**
cannot ship.

⛔ **BUT THE HAZARD THAT MATTERED IS ONE THE ROADMAP NEVER MENTIONED, AND IT IS SILENT.** A `Result`
in **argument** position does not error — `rdx` never reaches a parameter, so the callee receives
the **tag**. Measured: `Ok` tag = **0**, `Err` tag = **1**, `HSB_ERR_NONE` = **0**. So after
migration `assert_eq(f(...), HSB_ERR_NONE, …)` becomes `assert_eq(0, 0)` and **keeps passing while
testing nothing**, while error-path asserts fail loudly. **98 test lines** would have gone vacuous
without a word.
⚠ `lib/result.cyr`'s own header says the diagnostic "is the migration tool: every stale site fails
at its own line instead of miscompiling." True for **binding** sites, **false for argument sites** —
which is where a library's tests overwhelmingly live. ⭐ So the migration was driven by a **grep**,
not by build errors: `scripts/check-result-migration.sh`, mutation-proven before the first function
moved, and green at 0 bad call sites at the end with its `--selftest` still passing.

⭐ **`?` REPLACED THE MANUAL CHECK CHAINS — 18 sites.** ⚠ It needs a **binding**: it cannot be used
on a reassignment (`err = f(...)?`), which is why four sites took a second name. A tail call needs
nothing at all — `return g(...);` already forwards both halves, verified.

⛔ **THE DEPRECATION WINDOW IS DECLINED, ON THIS MIGRATION'S OWN EVIDENCE.** A dual API (`f` beside
`f_r`) would leave every existing `assert_eq(f(...), HSB_ERR_NONE)` site **quietly passing against
the old function forever** — the exact failure this release exists to remove. **2.24.0 stands as
the supported 2.x line.**

⛔ **AND THE PUBLIC/PRIVATE SURFACE IS BLOCKED UPSTREAM, REVERTED RATHER THAN HALF-APPLIED.**
`private`/`public` works and **the boundary holds** — verified end to end: a consumer including
`dist/hisab.cyr` can call the `public` API and **cannot** reach a file-private helper (build fails,
`'_noise_fade' is private to its file`, exit 1, no binary). That check mattered, because cyrius's
own changelog records `private` silently doing nothing for twelve releases while the guide promised
it worked. ⛔ It cannot ship because **`#derive(...)` and `public` cannot be combined on 6.6.2** —
a hard error with **no `private` anywhere in the file**, against a control differing by exactly one
keyword. Without `public` on the struct its generated accessors stay file-private, and privatising
hisab produced **1,436 errors of the form `'HVec3_x' is private to its file`** from the 18 modules
that derive accessors. Filed upstream as `2026-09-11-derive-cannot-combine-with-public.md`,
cross-referenced to the existing `#inline`-disarms-`#derive` filing, which produces the identical
diagnostic and may share a root cause.
⭐ The sizing survives and is better than the roadmap's: **939 functions, 296 underscore-prefixed,
148 functions AND 25 globals crossing a module boundary, 31 of them underscore-named** — the row
counted 17 and did not count globals at all.

⚠ **FIVE OF MY OWN INSTRUMENTS WERE WRONG BEFORE THE CODE WAS.** The source migrator computed
function spans **once** and then mutated the file, so every later span drifted — it swept in
**three functions that were never fallible** (`num_halton`, `num_sobol`, `_numx_use_fft`), caught by
diffing what is migrated against what was fallible at HEAD, and fixed by processing back to front.
The call-site converter could not tell a **status** function from a **value** one, so
`assert_eq(f(...), 0)` silently dropped the value check on **22 assertions**; it now classifies by
whether every `Ok` payload is a literal `0` (34 status-style, 2 value-style). Its comma splitter
ignored string literals and broke every message containing a comma. The gate matched function names
**inside message strings** (12 false positives of 13) and flagged correct **tail delegations**. And
one compile check used `cyrius build src/main.cyr`, which **does not include the library** — the
suites failed immediately after.

### Changed
- **BREAKING** — 48 functions across 12 modules return `Result<T, E>`: `Ok(0)` for status
  functions, `Ok(value)` for the two that return an answer (`num_modpow`,
  `halfedge_is_boundary`), `Err(HSB_ERR_*)` throughout. Out-parameters are unchanged.
- **cyrius.cyml** — `result` added to `[deps] stdlib`; `dist/hisab.deps` grows 15 → 16 leaves.

### Added
- **scripts/check-result-migration.sh** — guards the one failure mode the compiler cannot see.
  Verified by `--selftest` and against a real one-function slice.
- **docs/guides/migration-3.0.md** — the four call shapes, the `?` rules, the argument-position
  trap, and why there is no deprecation window.


## [2.24.0] - 2026-09-11 — the two CGA deferrals, and a cost the plan for them did not predict

2.23.0 costed both remaining CGA items and left them as implementation. This is that implementation —
and measuring it end to end found a trade the costing had not: **the build cut is not free at steady
state.** Suites unchanged at **4039**; benchmarks **74 → 78**.

⭐ **THE FIRST CGA CALL: 687 µs → 258 µs (−62.4%).** Three cuts, all to code that runs exactly once:
memoise `_cga_geo_bits` (a pure function of two 5-bit masks, called ~16k times, each walking 5 bits with
a popcount); memoise `_cga_bits_blade` (a **linear search** over 32 blades, called up to twice per pair);
and replace four full 32-slot passes per pair — clear `mid`, clear `res`, scan `mid`, scan `res` — with
occupancy lists, since **at most 16 slots of 32 are ever touched** and the measured maximum is 2.
⭐ **The table is byte-identical**, and that was checkable only because 2.23.0 had already closed the
FNV contract's magnitude blind spot: `0xF4A98C5706D5CF5B` and `_CGA_NULL_COEF_BAD == 0` both hold.

⛔ **AND IT COSTS 1.87% ON THE STEADY-STATE PRODUCT, WHICH NOTHING IN THE PLAN ANTICIPATED.** The
builder runs **once**; the steady path executes identical machine code either way. It is nonetheless
real and reproducible: 1176 → 1198 ns, slower in 9 of 11 interleaved pairs over a 40,000-iteration loop
with a 2.6% spread. **So the change saves 429 µs once and costs 22 ns per product — a net win below
~19,500 products and a net loss above it.** That crossover is the number a consumer needs, and the
costing that preceded this release did not have it because it measured build time in isolation.

⚠ **Three explanations were ruled out by measurement before settling on code layout.** It is **not**
heap displacement: a dummy allocation of exactly the 9,472 B the memo tables add reproduces **0.00%**,
placed either before everything **or** between the table build and the loop — and the second placement
is the one that matters, because the first shifts the table and the loop's allocations together and
preserves their relative offset. It is **not** run order: reversing which binary runs first gives
+3.63% against +3.81%. What does reproduce a share of it is **unrelated code growth** — a never-called
function inserted before the builder costs +0.91% on its own.
⭐ Acting on that: extracting the two memo loops into their own functions **halves the penalty, from
+4.33% to +2.04%**, with the first-call saving unchanged. That is why they are helpers rather than
inline, and the comment there says so — it is a measured choice, not tidiness.

⛔ **THE LITERAL 1024-ENTRY TABLE IS STILL DECLINED, ON NUMBERS.** It is the only option that truly
closes the regression (11.9 µs, against 2.20.0's 11 µs) and it passes every gate — but it costs +748
lines, +1.45% bundle and +2.93% consumer `code_size`, and it gives up the *built by integer arithmetic,
exact by construction* property the derivation gate rests on. At 258 µs paid once, that is not a trade
worth making yet.

⭐ **AND CGA HAS BENCHMARK ROWS AT ALL FOR THE FIRST TIME**, which is the actual fix for how a 2.7 ms
first call hid for three releases: there was no row, so every CGA figure in that arc lived in a
throwaway probe and none was ever checked again.
- `cga_first_product_cold` — **n = 1**, because a benchmark that warms up *cannot see the build cost*,
  which is the whole reason the 2.7 ms hid. `bench_run` clamps its pilot chunk to n, so one shot really
  is one shot. Reads **203.9 µs**.
- ⚠ **It carries an entry guard, and the guard is mutation-proven.** A cold row that is not cold reads
  ~400× better and would look like a spectacular improvement rather than a broken measurement;
  pre-warming the table in a copy makes the run print `FATAL: CGA table already built` and **exit 1**.
- `cga_point_product` (1.20 µs), `cga_norm_sq_k5` (1.29 µs) and `cga_norm_sq_k32` (27.7 µs) — **both
  sides of the k ≈ 8 crossover**, because `_cga_scalar_of_geo` is quadratic in occupancy and a single
  row reports the opposite sign depending on which side it sits.
- ⚠ An `alloc_used()` bound in a `.tcyr` was considered and **rejected**: it is deterministic, but
  2.21.0's regression was CPU work rather than allocation, so it would not have caught the thing the
  row exists for — and a wall-clock ceiling in a test is a flaky-test generator on a shared runner.

⚠ **Registered LAST, and the control says it worked.** 2.20.0 measured that adding two rows moved an
untouched module's row by −32% to −44% through heap displacement under a never-freeing allocator.
Across 26 existing rows: **median +0.05%, mean +0.07%, 0 past 10%**, largest mover +0.83%.

### Changed
- **geo_advanced** — `_cga_build_null_tbl` memoises `_cga_geo_bits` and `_cga_bits_blade` and iterates
  occupancy lists instead of four 32-slot passes per pair. ⚠ `mseen`/`rseen` are load-bearing, not
  decoration: an accumulating slot can return to zero and be written again, so testing `== 0` to decide
  whether to append would append it twice and **double-count it in the scan**. ⚠ `rt` is kept sorted on
  insert because the packing loop depends on ascending bitmask order — nothing in the suite can
  currently discriminate a wrong order, so the sorted insert is what makes that ordering a guarantee
  rather than a coincidence, and at ≤ 2 terms it costs one comparison. ⚠ The `if`-guard /
  no-`continue` block is kept verbatim: the nest is still three loops deep and still relies on the
  cycc 6.6.2 workaround.

### Added
- **tests/hisab.bcyr** — four CGA rows (74 → 78), registered last, with a mutation-proven cold guard.


## [2.23.0] - 2026-09-11 — the sites no fixture reached, and a defect 2.22.0 introduced there

2.22.0 repaired three Householder gates **on the shape**, because repairing one side of a pair is how
2.18.0's U-replay defect was created — and the roadmap carried that as unproven in both directions.
This release went and looked. All three repairs were **load-bearing**; one of them also **introduced a
regression**; and the fourth reflector had never been repaired at all. Suites **4018 → 4039**.

⭐ **THE DISCRIMINATION LADDER IS THE RESULT.** The new assertions, run against every released tree:

| tree | new assertions failing |
|---|---|
| 2.21.0 | **12** |
| 2.22.0 | 7 |
| 2.22.1 | 6 |
| **2.23.0** | **0** |

Each release's repairs were real; each left something behind; and none of it was pinned until now.

⛔ **THE FOURTH REFLECTOR WAS NEVER NORMALISED.** 2.22.1 gave `v` unit length in the three REAL
reflectors so `vtv` is 1 by construction and cannot underflow. `cqr_decompose` forms `vhv` the same
naive way and was left — three of four. For `diag(1) ⊕ c·[[3,1],[4,2]]` the k=1 reflector's `v` is
`(8c, 4c)`, so `vhv = 80c²`, which drops below `DBL_MIN` at **c = 2^-515**: measured, e=514 correct and
e=515 wrong, and `80·2^-1028 > DBL_MIN > 80·2^-1030` predicts exactly that. Blocks with `vhv = 468c²`
and `0.3125c²` first fail at e=516 and e=511, **as their coefficients dictate** — causal isolation, not
correlation.

⛔ **AND THE FAILURE IS INVISIBLE TO BOTH RESIDUAL TESTS A CALLER WOULD RUN.** With the reflector
skipped, `R == A` and `Q == I`, so `max|QR − A|` and `max|QᴴQ − I|` are **exactly 0** on the wrong
answer. The only visible symptom is that **R is not upper triangular** — the one thing a QR
decomposition is for. Measured: `R21` comes back *equal to the input entry* at every scale.

⛔ **2.22.0's PHASE REPAIR INTRODUCED A DEFECT, AND IT TRADED 1 ROW RIGHT FOR 38 WRONG.** That release
replaced an absolute gate with `cx_div(x1, |x1|)` — which is only as good as `cx_abs(x1)`, and for a
subnormal `x1` that modulus has been quantised onto the subnormal grid, so the quotient is **not a unit
complex number**. The Householder needs `|phase|` exactly 1 and degrades in proportion to `||phase| − 1|`,
which reaches **+0.414** at the floor. 2.21.0's `phase = 1` fallback was **accidentally exact** in that
band. Repaired by scaling `x1` to O(1) first; the phase is scale-invariant, so nothing above the
subnormal band moves. ⚠ **A fixture built from a Pythagorean triple cannot see this**: `x1 = (3,4)·t` is
clean at every `t` because `|x1| = 5t` is exactly representable. The acceptance test uses `(1,1)`.

⛔ **THE RIGHT REFLECTOR WAS SILENTLY WRONG AND IS NOW PINNED IN BOTH ORIENTATIONS.** `A = diag(c,c,c,1)`
with `A[0][2] = t` puts a small off-diagonal where only the right reflector can see it; the trailing
block `[[c,t],[0,c]]` has singular values `c·(√(1+(t/2c)²) ± t/2c)`, so the spectrum has a relative
spread of exactly `t/c` **in closed form**. With `c = 2^-20`, `t = 2^-41`: 2.21.0 returns all three
small singular values **identical** — the entire spread erased, 2.384e-07 relative error — against
**5.06e-17** now. ⚠ `k=0` and `k=1` are separate call sites reached by different matrices, so both are
pinned; and a third fixture (`c = 2^-500`, `t = 2^-515`) sits where `norm_r` is normal but `vtv_r` is
**subnormal**, which the gate repair alone cannot reach and only 2.22.1's normalisation fixes.
⚠ **`prod(S) == |det A|` is analytically blind here** and must not be used: the defect replaces
`{c(1+d), c, c(1−d)}` with `{c, c, c}`, and `(1+d)(1−d)` differs from 1 only at second order — measured
1.24e-15 while a singular value is 2.4e-7 wrong. The discriminator is the **spread**.

⛔ **AND A GATE THAT DOES NOT GATE, PROVEN WITH A ONE-TOKEN MUTANT.** `_cga_build_null_tbl` packs only
each coefficient's **sign** and blade index, so the null-product table is a function of sign and
zero-pattern alone — any positive rescaling produces a **byte-identical** table. Changing
`co * cn * wgt` to `co * cn * wgt * 3` makes all 1024 coefficients ±3, the FNV stays bit-exact at
`0xF4A98C5706D5CF5B`, and **all 4018 assertions pass**. The "every coefficient is exactly ±1" property —
the invariant that *licenses* packing only the sign — was asserted only in the Python derivation, never
on the Cyrius side, so the source's "verified two ways" transferred only the sign projection.
`_CGA_NULL_COEF_BAD` now counts it during the build; verified to fail the mutant (`got 1024, expected 0`)
**while the FNV still passes**.

### Changed
- **linalg_precision** — `cqr_decompose` normalises `v` before forming `vhv`, completing the set of four.
- **linalg_precision** — `cqr_decompose` forms its phase from an `x1` scaled to O(1).
- **linalg_precision** — `cqr_decompose` zeroes `R`'s strictly-lower triangle explicitly. ⚠ The
  reflectors annihilate it analytically but in floating point, and through 2.22.1 that happened to
  cancel to a bit-exact 0 **on the fixtures anyone had run**; normalising `v` adds one rounding and it
  no longer does (measured `|R21|` ≈ 7e-16 *relative to the block*, on 5 of 9 decades). Upper
  triangularity is the defining property of QR and the thing the tests assert, so it should be true
  rather than nearly true. ⚠ Reconstruction verified unharmed: `max|QᴴQ − I|` = 2.22e-16 and
  `max|QR − A|` tracks the block magnitude at every scale including 2^-1000.

### Added
- **tests** — 21 assertions (**4018 → 4039**) pinning the three previously-unreachable sites, each
  verified against every released tree (the ladder above). Includes `_CGA_NULL_COEF_BAD`, mutation-proven.

### Fixed
- **roadmap** — the "three gates repaired on the SHAPE, no fixture reaches them" item is **closed as
  PROVEN, not as speculation**. All three were load-bearing. ⚠ The existing `cqr` block-ratio sweep
  stops at 1e-10 while its gate fires below 1e-12 and its `vhv` underflows below 2^-515 — **three
  decades above one defect and ~145 above the other**, and green throughout. *Bracketing the thresholds
  is not reaching the floor*, which this project last learned in 2.17.0 and has now learned again.


## [2.22.1] - 2026-09-11 — the subnormal residue 2.22.0 filed, and why it needed BOTH halves

2.22.0 shipped with a known defect filed in `issues/` rather than repaired: `svd_golub_kahan`
returning `HSB_ERR_NONE` with a **fabricated ZERO** smallest singular value. This closes it.
Classified against a 200-digit oracle over 3 fixtures × 52 subnormal scales:

| | CORRECT | LOUD | SILENT |
|---|---|---|---|
| 2.22.0 | 23 | 62 | 71 |
| **2.22.1** | **151** | **0** | **5** |

**Zero `CORRECT → anything` regressions and zero `LOUD → SILENT`.** Suites **4014 → 4018**.

⛔ **THE BALANCE SCALE WAS CHOSEN FROM ONE END, AND THAT IS WHERE THE VALUE WENT.**
`_lp_pow2_floor(max|A|)` puts the largest entry in [1,2) and says nothing about the smallest, so a
matrix spanning more than 1022 binades has its small block **subnormal after the divide** — and a
power-of-two divide is exact only while the quotient stays NORMAL, because below `DBL_MIN` the grid
is absolute rather than relative. Measured on the 2.20.0 acceptance fixture at c = 2^-1073: the entry
is 18 units of 2^-1074, `pow2_floor(9)` is 8, `18/8 = 2.25` **rounds to 2**, and `2 × 8 = 16` —
**11% destroyed before `_lp_bidiagonalize` was ever called.** The scale is now chosen from **both
ends**: halve it (scale the matrix *up*) until the smallest non-zero entry is normal, stopping if the
largest would overflow. Still a power of two, so still exact; when every entry is already normal the
loop does not execute and behaviour is unchanged.

⛔ **AND THAT REPAIR ALONE WOULD HAVE BEEN WORSE THAN NOTHING.** With only the balance fixed, the
sweep reads CORRECT 102 / SILENT 54 — but **30 loud rows became silent wrong answers**, the precise
trade the 2.19.0 refutation named and the reason this release could not ship in halves. The remaining
cause was one level down: `vtv` is computed as a naive sum of squares, and at the *bottom of the
normal range* (entries ≈ 2^-1022) every square underflows to zero, so 2.15.0's `F64_TINY` guard
correctly refuses to divide — **and the consequence is exactly the silent drop the absolute gate used
to cause**. The reflector is skipped and the extraction discards the mass below the diagonal.
⭐ `H = I − 2vvᵀ/vᵀv` is identically `I − 2uuᵀ` for `u = v/‖v‖`, and `‖v‖` comes from the scale-free
`_lp_vec_norm` 2.22.0 repaired — so normalising `v` makes `vtv` **1 by construction** and there is
nothing left to underflow. All three reflectors (both bidiagonal, and the tridiagonal one reached by
`eigen_qr`) get it, because repairing one of a pair is how 2.18.0's U-replay defect was created.

⛔ **MY OWN FILING WAS WRONG, AND MEASUREMENT REFUTED IT WITHIN THE HOUR.** The issue said
*"compute the dynamic range … if it exceeds f64's — which it does for these fixtures — then no global
scalar can work."* **False.** f64's normal range spans **2045 binades** and these fixtures need
**~1076**; a single global scalar is sufficient, and the defect was *which* scalar, not how many.
⚠ A second draft then guarded the backoff with `sc > 1`, which does nothing at all for exactly the
inputs it was written for — dividing cannot lift a subnormal input into the normal range — and
measured, it turned a **correct** row at 2^-1065 into `HSB_ERR_NO_CONVERGENCE`. **A filing is a
measurement like any other**, and this one was wrong twice before it was right.

⭐ **THE FOUR 2.20.0 DEFECT PINS INVERTED, AND ONE OF THEM IS THE SUBTLEST IN THE FILE.** They
asserted the fabricated zero, the 10.8× value, the loud arm and the `94 of 156` count as *tracked
defects*; all four now assert the truth. The subtle one: the general 2×2 at c = 2^-1074 used to fail
loudly and now **succeeds returning (6, 0)** — and succeeding is *correct*, because the true singular
values there are 6.1623 and **0.16238 units** of 2^-1074, and 0.16238 of a unit is below the smallest
representable number, so the correctly-rounded smaller value **is exactly zero**. ⚠ It is asserted
with the *larger* value beside it, because "returns 0" is also what the fabricated-zero class
produces — the discriminator is that a fabricating routine gets neither right.

### Changed
- **linalg_precision** — `_lp_balance_scale` replaces `_lp_pow2_floor(_lp_mat_max_abs(A))` in
  `svd_golub_kahan` and `eigen_qr`. Two-ended, power-of-two, and a no-op when every entry is normal.
- **linalg_precision** — all three Householder reflectors normalise `v` before forming `vtv`, so
  `vtv` is 1 by construction and cannot underflow.

### Fixed
- **issues/** — `2026-09-11-svd-balancing-divide-lossy-for-subnormal-blocks.md` **closed and moved to
  `issues/archived/`**, with its own wrong root-cause framing corrected in the archived copy rather
  than quietly deleted. The 11%-loss measurement in it was right; the "no global scalar can work"
  conclusion drawn from it was not.
- **tests** — 4 assertions added (**4014 → 4018**) and the four 2.20.0 defect pins rewritten to assert
  truth. Verified against the 2.21.0 tree: **17 assertions fail there and all pass here.**

### Performance
- **A measured cost, stated rather than hidden**: against the 2.21.0 released tree (i.e. 2.22.0 and
  2.22.1 combined), `svd_golub_kahan_12` **+4.99%** (113.93 → 119.61 µs) and `eigen_qr_12` **+3.56%**
  (85.51 → 88.55 µs), against an **untouched control of +0.40% median over 24 rows, 0 past 10%**
  (largest untouched mover 2.65%), on a quiet box (load 0.30). The cost is real and outside the
  control band: an O(n²) finiteness scan, an O(n²) min-magnitude scan, and one division per component
  per reflector. **~5% to remove a class of silent wrong answers is the trade, and it is stated.**


## [2.22.0] - 2026-09-11 — the Householder gate: a silent wrong answer at ordinary conditioning, in two public entry points

The roadmap carried this as **"subnormal SVD"** for three releases. It is not a subnormal defect, and
that is the release. Suites **3991 → 4014**.

⛔ **`_lp_bidiagonalize` GATED ITS HOUSEHOLDER REFLECTORS ON AN ABSOLUTE 1e-12, AND WHEN THE GATE FIRED
IT SILENTLY DISCARDED THE MASS IT DECLINED TO REFLECT.** `norm_l`/`norm_r` are **lengths** — degree ONE
in the caller's units — tested against `EPSILON_F64`. Below that the reflector was skipped, while the
extraction below it still read only the bidiagonal band, so the entries that reflector would have zeroed
were **dropped** and the routine returned the factorisation of a **different matrix** with
`rc = HSB_ERR_NONE`.

⭐ **THE BOUNDARY IS A BLOCK RATIO OF 2^-40 — A CONDITION NUMBER OF ~1e12, NOT A SUBNORMAL.** That is an
ordinary ill-conditioned matrix, **982 binades above** where the roadmap placed the defect. Measured
oracle-free, on `A = blockdiag(B, c·B)` with `B = [[3,4],[2,3]]` (det B = 1), where
`prod(singular values)` must equal `|det A| = c²` and needs no reference implementation:

| block ratio | 2.21.0 `prod(S)/det` | 2.22.0 |
|---|---|---|
| 2^-38 | 1.000000 | 1.000000 |
| 2^-39 | 1.000000 | 1.000000 |
| **2^-40** | **9.000000** | 1.000000 |
| 2^-400 | 9.000000 | 1.000000 |

The smallest singular value came back **9.89× too large**, with `rc = HSB_ERR_NONE`, from 2^-40 all the
way down. Over a 60-scale normal-range sweep: **25 of 60 silently wrong → 0**. The predicted crossing is
`0.9014·c > 1e-12` after balancing, i.e. 2^-39.71 — and the measured boundary is exactly e = 39 → 40, so
the mechanism is confirmed quantitatively rather than plausibly.

⛔ **AND THE SYMMETRIC TWIN HAS THE SAME GATE, WHICH IS WHY THIS RELEASE COVERS TWO PUBLIC ENTRY
POINTS.** `_lp_tridiagonalize` carries the identical absolute test, so `eigen_qr` returned the spectrum
of a different matrix. On `A = [1] ⊕ c·B3` with `B3 = [[1,1,2],[1,4,1],[2,1,1]]` (exact eigenvalues
**−1, 2, 5**; the leading 1 pins `max|A|` so balancing is the identity and nothing is subnormal): correct
at 2^-41, and from **2^-42** down it returns `λ/c = {4.5615528, 1, 0.4384472}` — **the smallest
eigenvalue comes back with the wrong sign**. ⭐ Those three numbers are exactly `(5±√17)/2` and `1`, the
spectrum of `tridiag((c,4c,c),(c,c))` — *the matrix you get by dropping `W[3,1] = 2c`*. The mechanism is
pinned by the values themselves, independently of the symptom.

⛔ **A SECOND, SEPARABLE DEFECT: `_lp_bidiag_qr`'s ACTIVE-BLOCK SEARCH FROZE THE BLOCK AT 2×2.** `p_lo`
walked left while the superdiagonal above it was still coupling — and tested that coupling against an
absolute `EPSILON_F64`, while the two deflation loops fifty lines above ask the same question
**relatively**, and the symmetric twin `_lp_tridiag_qr` has used the relative form since 2.7.0. The SVD
half of that pair was never converted. ⚠ **A 2×2 SUB-BLOCK CANNOT EXPOSE IT**, which is why the entire
2.20.0 acceptance family missed it: for a 2×2 the only superdiagonal `p_lo` examines is the exactly-zero
coupling *between* blocks, so `p_lo` is right by accident. It takes a **3×3 or larger** block, whose own
interior superdiagonal is small, to reach it. ⭐ **The two defects are perfectly complementary** — the
reflector repair leaves the 3×3 subnormal case byte-identical, and this repo's rule says a repair that
moves no number is pointing at a second defect.

⛔ **BOTH ENTRY POINTS RETURNED SUCCESS WITH A NaN PAYLOAD.** `svd_golub_kahan([[+Inf,0],[0,1]])`
returned `HSB_ERR_NONE` with **both singular values NaN**; `eigen_qr` on the same matrix did the same
with its eigenvalues. −Inf and NaN behave identically. ⭐ The guard is a single *is not finite* test,
because **every f64 comparison involving NaN returns 0** — so `f64_lt(|x|, +Inf) == 0` catches Inf and
NaN together, and splitting it in two is what invites someone to drop the NaN arm.

⛔ **THE ROADMAP'S PROPOSED REPAIR WAS AT THE WRONG PIPELINE STAGE, AND THE 2.19.0 REFUTATION THAT
BLOCKED IT FOR THREE RELEASES IS TRUE.** The row proposed scaling the active block *inside*
`_lp_bidiag_qr`. Measured: that converts **84 loud rows into gross silent wrong answers**, 83 of them on
general 2×2 blocks — and it cannot recover a reflector that bidiagonalisation never applied, because the
mass is gone before the QR sweep runs. **The refutation was right and the row it blocked was wrong; what
was missing was the measurement that says which.**

⚠ **A SEVENTH CONSECUTIVE RELEASE WHERE THE ROW WAS WRONG ABOUT SCOPE, AND THIS TIME THE GREP FOUND FIVE
SITES IN THREE FUNCTIONS UNDER THREE PUBLIC ENTRY POINTS** — `:270` and `:341` (`_lp_bidiagonalize` →
`svd_golub_kahan`), `:1119` (`_lp_tridiagonalize` → `eigen_qr`), `:1671` and `:1676` (`cqr_decompose`).
The row named **none** of them; it named only `_lp_bidiag_qr`. Two were measured end-to-end and are
confirmed defects; the other three are repaired **on the shape**, and that is stated rather than implied.
⭐ Repairing one side of a pair is how 2.18.0's U-replay defect was created, so `:341` went with `:270`
even though only `:270` was measured.

### Changed
- **linalg_precision** — the five Householder gates test `norm > 0` instead of `norm > EPSILON_F64`.
  ⭐ **Guard what actually fails.** The only operation at risk is `2/vtv`, and it already has its own
  degree-TWO `F64_TINY` test a few lines below (2.15.0) — which the absolute outer gate had made
  **unreachable**. What is left for the outer gate to ask is *"is there anything to reflect?"*, an exact
  zero question with no scale to choose. **A threshold that needs a scale chosen for it is the defect.**
- **linalg_precision** — `_lp_vec_norm` scales by the largest component before squaring. ⚠ **This is the
  one site 2.17.0's norm tier missed**: that release repaired 39 of 51 `sqrt(sum of squares)` sites, and
  this one — which serves all three real Householder reflectors — kept the naive form. It is the *floor*
  of the gate repair, not a separate nicety: with the gates now asking `norm > 0`, a norm that flushes to
  zero for a non-zero vector would reinstate the exact skip they were repaired to stop.
  ⭐ Fast path, not unconditional scaling: the naive sum announces its own failure (subnormal, zero or
  +Inf exactly when it cannot be trusted), so it is taken when normal and rescaled only when not.
- **linalg_precision** — `cqr_decompose`'s inlined complex norm gets the same scaling, and its phase test
  `abs_x1 > EPSILON_F64` becomes `> 0`. ⚠ The phase fallback was `1`, so a small-but-good `x1` got the
  **wrong phase rather than no phase** — a fabricated value, not a refusal.
- **linalg_precision** — `_lp_bidiag_qr`'s `p_lo` search uses the relative `_lp_negligible`, matching its
  own sibling deflation loops and the symmetric twin.
- **linalg_precision** — `svd_golub_kahan` and `eigen_qr` reject a non-finite entry with
  `HSB_ERR_INVALID_INPUT`, matching the `m < n` precedent already in the file: *rejecting is the honest
  contract*, since the caller's `out_S`/`out_vals` have nowhere to put "undefined".

### Removed
- **linalg_precision** — a dead 20-line block in `_lp_bidiag_qr` computing `q` via a "force exit trick"
  and an "undo the forced exit trick", after which `q` was **never read again** (verified by grepping
  every `q` in the function; the comment below it begins *"Recalculate: … Simpler approach"*). ⚠ It also
  held the file's **second** absolute `EPSILON_F64` on a superdiagonal, so a threshold census found two
  sites here and one of them could never fire. Output is bit-identical across the deletion.

### Fixed
- **tests** — 23 assertions added (**3991 → 4014**), and **every one was run against the 2.21.0 tree to
  prove it discriminates**: 9 of the new SVD/eigen assertions and 6 of the new non-finite assertions fail
  there and all pass here, while the controls pass on both. ⚠ **The first draft of the eigen assertions
  demanded bit equality and all four failed by exactly one ULP** — the QR iteration converges to within
  rounding; it is not scale-equivariant the way the SVD's power-of-two block is. The fixture was wrong,
  not the code, and the bound is now a few ulp with the **sign** asserted separately, because a tolerance
  alone would pass a repair that merely shrank the error.

### Fixed (docs)
- **linalg_precision** — a comment asserted *"the power-of-two divisor makes both the divide and the
  un-scale exact."* **It is false for a subnormal entry, and the claim was load-bearing**: it is why the
  balancing was never suspected as a source of loss. A power of two is exact only while the quotient
  stays NORMAL; below `DBL_MIN` the grid is absolute, so dividing walks off it. Measured on the 2.20.0
  acceptance fixture: the entry is 18 units of 2^-1074, `pow2_floor(9)` is 8, `18/8 = 2.25` rounds to
  **2** units, and `2 × 8 = 16 ≠ 18` — **11% destroyed before `_lp_bidiagonalize` is called**. Corrected
  in place with the measurement beside it. **A comment asserting a numerical property is a claim like any
  other.**
- **issues/** — filed `2026-09-11-svd-balancing-divide-lossy-for-subnormal-blocks.md`. ⚠ It was first
  written into the roadmap's "Decisions owed" and called *filed*, which it was not: `CLAUDE.md` says
  defects are tracked in `issues/` and discharged as a precondition of the release they gate, and
  `issues/` held nothing but `archived/`. The roadmap row is now reduced to the scaling **decision** the
  repair needs and points at the issue. **A roadmap row is not a filing.**

### Performance
- **A measured cost, stated rather than hidden behind its own spread**: `svd_golub_kahan_12` **+2.98%**
  (113.85 → 117.24 µs) and `eigen_qr_12` **+0.75%** (86.17 → 86.82 µs), against an **untouched control of
  +0.29% median over 23 rows, 0 past 10%**. Small, in the direction the added finiteness scan and the
  norm's two extra branches predict, and inside that row's own historical spread.
  ⚠ **The first draft of this claim said "no detectable cost" and was written before it was measured.**
  ⚠ The first attempt to measure it was also wrong: a `cd` into the reference tree persisted across the
  next command, so both "before" and "after" numbers came from the **same** tree. **Check the probe
  before believing the probe** — the sixth release running that this was needed.


## [2.21.0] - 2026-09-11 — the CGA null basis: a point's nullity error becomes ulp-level and scale-free

2.20.0 proved `cga_point` returned a wrong answer with **no arithmetic fix available** — at x = 2^-30
the correctly-rounded `ep` **is** exactly −1/2, so the information is gone at construction — and filed
the basis change as its own release. This is it. Suites **3983 → 3991**.

⭐ **THE REPAIR IS THE BASIS, NOT THE ARITHMETIC, AND THE RESULT IS A SCALE-FREE ERROR FLOOR.** In the
ep/em basis a point stores `(q−1)/2` and `(q+1)/2`, so `q` is the sum *and* difference of two
nearly-equal numbers and is destroyed at both ends. In the null basis `P = p + (q/2)·ninf + n0`, `q`
lives in **one** coefficient and the norm reconstructs it directly instead of recovering it from a
cancellation. Measured against **the tree that actually shipped as 2.20.0**, 2000 random full-mantissa
points per binade — median `|P·P| / q`:

| binade | 2.20.0 | 2.21.0 |
|---|---|---|
| 2^-30 | **1.0** | 4.48e-17 |
| 2^0 | 1.05e-16 | 4.70e-17 |
| 2^+30 | **1.0** | 4.33e-17 |

**A relative error of 1.0 means the entire value is lost**, so both tails went from total loss to
sub-ulp, and the result is **flat** rather than a U centred on |x| ≈ 1. Distance recovery
`P₁·P₂ = −d²/2` tracks it: **median 46.4× wrong at 2^-30 and 48.7× at 2^+30 → 1.76e-15 and 1.93e-15**,
with the centre improving too (**3.8e-15 → 1.87e-15**).

⛔ **"EXACTLY NULL" WAS THE HEADLINE OF THIS RELEASE UNTIL A PRE-TAG AUDIT SHOWED WHAT IT COST.**
Bit-exact nullity *is* reachable, and this release reached it: removing 2.18.0's Neumaier compensation
from `_cga_scalar_of_geo` makes the norm repeat `cga_point`'s own rounding of `q`, so the two cancel and
**0 of 6800 points are non-null**. That has been **reverted**. Without the compensation,
`M = 1 + a·e1 + a·n0 + (a/2)·ninf` — whose scalar part is exactly 1 — reads **0 for 6 of 11 magnitudes,
every one at a ≥ 2^30**: a fabricated zero, which is the defect class this repo has been repairing since
2.6.14, and a worse outcome than a sub-ulp residual. **An exact zero obtained by making the same
rounding error twice is an artifact, not accuracy.** With the compensation restored, the witness is
right at all 11 magnitudes, distance recovery is *better* (1.76e-15 against 1.85e-15), and bit-exact
nullity reads **5194 of 6800 non-null** against 6761 on the shipped tree. The claim this release makes
is therefore the **residual magnitude** above, which does not depend on the cancellation being exact.
⚠ I also tried compensating `cga_point`'s `q` to match instead; it moved 5194 → 4988 and changed no
median, so it was reverted. **Unmeasured complexity is not a fix.**

⛔ **AND NO ASSERTION IN 3989 COULD SEE THAT CHANGE AT ALL.** Removing the compensation and restoring it
both give a fully green suite — the fabricated zero is on a path nothing exercised. Two assertions now
pin the witness (**3989 → 3991**), verified to fail with the compensation absent (2071 passed / 1 failed)
and pass with it present. **A repair no test can distinguish from its own regression is not covered, it
is unobserved.**

⛔ **AN AUDIT BEFORE TAGGING FOUND FOUR WRONG FIGURES IN THE FIRST DRAFT OF THESE NOTES, TWO GATES THAT
DID NOT GATE, AND A FILING THAT DESCRIBED ITS OWN BUG WRONGLY — ALL OF IT MINE.** Most of the bad
numbers came from one mistake: measuring the "before" against a **model** rather than against the tree
that shipped.
- The nullity baseline read **24.8%**; that is `derive-cga-null-table.sh`'s Python *model* of ep/em
  arithmetic. The shipped 2.20.0 tree measures **99.4% (6761 of 6800)**. The result was better than
  claimed and the published number was still wrong.
- A translator claim — *"exactly 1 for all 1001 binades; it failed 526 before"* — was **simply false**.
  Measured on both trees: **485 of 2046 binades fail, first at 2^539 — identical.** The basis change did
  nothing for the translator.
- *"Exactly null at every magnitude"* overstated the range even before the compensation was restored:
  **57 binades fail** — 27 where `q` is subnormal and `fl(q/2)` loses the bottom bit that doubling
  cannot return, 30 above 2^511 where `q` overflows and the norm is NaN. Below 2^-539 nullity "returns"
  only because `q` has flushed to zero: the conformal part is gone, not correct.
- **`point*point` was published as 5306 → 2394 µs and re-measured at 5328 → 2404 µs — right by
  accident.** The first probe that re-checked it read **5135 µs on the new tree**, apparently refuting
  the speedup; the difference was a warm-up loop, and chasing it found a real cost nobody had measured
  (below). The figure stands; the reason it stood was not the one I would have given.
⛔ **And two gates were green while checking nothing.** `derive-cga-null-table.sh`'s claim 6 *printed*
the table's FNV and never compared it, and claim 4's null arm tested `q + 2·(q/2)·(−1) != 0`, which is
**identically false for every normal q** — a tautology reporting "0 non-null". Both exited 0 whatever
the table said, while the CI step advertised *"fail-closed (verified: breaking any claim exits 1)"*.
Both now compare, and each is verified to exit 1 when broken. Claim 4 is additionally labelled, in its
own output, as a **model** whose ep/em figure must never be quoted as the library's — because that is
exactly the confusion that produced the bad baseline above.

⛔ **THE FIRST CGA CALL COST 2.7 ms, AND EVERY BENCHMARK WAS BLIND TO IT BECAUSE THEY ALL WARM UP.**
The null table is built lazily at first use, and the first version of the builder called
`_cga_expand_null`/`_cga_contract_orth` **inside** the 32×32 pair loop — 1024 evaluations for 32
distinct answers — then scanned all 32×32 coefficient slots per pair when **at most 4×4 are ever
occupied**. First `cga_geometric_product`: **2.68–3.00 ms against 11 µs on 2.20.0, a ~250× one-time
regression.** It surfaced only because two probes of the same operation disagreed by 2.1× and the
difference was a warm-up loop. Repaired by hoisting both tables and iterating occupancy lists:
**0.72 ms**, still 65× the old first call. So the table pays for itself past **~430 products** — ~709 µs
extra once, ~1.65 µs saved per call after (steady state **1.25 µs against 2.90 µs**). ⭐ **The rewrite
was safe to make at tag time precisely because the FNV contract already existed**: the table is
byte-identical across it, and `0xF4A98C5706D5CF5B` is asserted in the suite.

⛔ **AND THE SPEEDUP IS SCOPED, NOT GLOBAL — IT IS A 10.9× REGRESSION FOR DENSE MULTIVECTORS.**
`_cga_scalar_of_geo` went from 32 diagonal terms to k² all-pairs terms, so it is **quadratic in
occupancy** where it was flat. `cga_norm_sq` over 2000 calls by occupied-blade count k, 2.20.0 vs
2.21.0 (µs, median of 3): k=4 **5025 / 1960** (2.56× faster), k=5 5068 / 2597 (1.95×), k=8 5331 / 5121
(parity), k=12 5031 / 9854 (**1.96× slower**), k=16 5017 / 15744 (3.14×), k=24 4971 / 31878 (6.41×),
k=32 **4996 / 54592 (10.93× slower)**. The crossover is at **k ≈ 8**. Every conformal primitive sits
below it — point 5, sphere 5, translator 4, rotor 2 — so the sparse win is what consumers see
(`point*point` **5328 → 2404 µs, 2.22×**); a dense general multivector is not.

⛔ **BUILDING IT FOUND A COMPILER BUG, AND MY FIRST FILING DESCRIBED IT WRONGLY.** It was filed as
*"a firing inner `continue` exits the OUTER loop"*. **Instrumenting the loop refuted that**: the outer
loop is not terminated and runs its full trip count. What actually happens on cycc 6.6.2, when a loop
and a loop nested inside it both contain a `continue` **and the outer one appears lexically first**, is
that both bind one level too far out — the inner `continue` jumps to the **outer** latch (abandoning the
inner loop *and* the rest of the outer body), and the **outer `continue` becomes a no-op**. That second
half was missing from the filing entirely, and it fails in the opposite direction: a `continue` that
silently does nothing produces *more* work, so it cannot announce itself. ⚠ The decisive case has an
outer `continue` that **never fires** — `if (c == 99) { continue; }` for c in 0..2 — yet its presence
breaks the inner one, which rules out a misreading of the semantics; and moving it *below* the nested
loop makes the same program correct, which isolates the trigger to lexical order. The natural sparse-skip
idiom for a 32×32 table is both broken shapes at once and produced an **all-zero 1024-entry table while
reporting success**. Filed upstream as `2026-09-11-nested-continue-binds-to-wrong-loop.md` with a repro
that **proves itself** (exit 0 when correct, 1 while present) across all ten shapes plus three
instrumentation counters. hisab works around it with if-guards and a comment saying not to tidy them back.

⭐ **THE GATE CAME BEFORE THE CODE.** `scripts/derive-cga-null-table.sh` derives all 1024 entries from
first principles in CI — at most 2 terms per product, every coefficient exactly ±1 (so the product adds
no rounding), agreement with the orthonormal table under change of basis, the defining identities
`n0² = 0`, `ninf² = 0`, `n0·ninf = −1`, and the contract **FNV-1a `0xF4A98C5706D5CF5B`** in
blade-index space. The Cyrius table reproduces it exactly and a test asserts so. ⚠ **The index space is
the hazard and it caught me**: the derivation runs in *bitmask* space and the implementation is called
with *blade indices*, and my first spot-check read `n0² = +1` when it was really `e3² = +1` in the other
numbering. **A table that is right in the wrong space is exactly the plausible-but-wrong artefact the
gate exists to stop.**

**No performance change is claimed from the basis flip in the tracked benchmarks, and the one large
mover has its own control.** Against the previous run: **median −2.01%, and exactly one row past 10%** —
`jac_rev_shared_256` at **+238%** (337–348 µs → 1175 µs). That is the tape-overflow fix from 2.20.0's
cut finally being measured: the row had been sized `N + 4*M + 8` against `N + M*N` pushed, so it was
reporting success while doing a fraction of the work. **`jac_rev_pertape_256` is flat across the same
boundary** (184.8 → 183.6 µs) because its tape was correctly sized all along — if the machine or the
basis change had slowed anything, both rows would have moved. The shared/per-tape ratio now reads
**6.4× at m = 256**, matching the corrected standalone measurement.
⚠ **The CGA figures do not appear in the trend table at all**, because there is no CGA row among the
74 benchmarks — which is how the 2.7 ms first call stayed invisible. They were measured directly, and
adding a CGA row is on the roadmap.

⭐ **AND THE 2.20.0 ACCEPTANCE PINS INVERTED, EXACTLY AS DESIGNED.** That release pinned the failures at
2^-30 and 2^+30 as KNOWN WRONG so a repair could not land quietly; this one made them correct and the
suite failed until they were rewritten. **A pinned defect is a tripwire for its own fix**, which is
worth more than a comment saying the fix is pending.

### Changed
- **geo_advanced — the CGA basis is now the NULL basis {e1,e2,e3,n0,ninf}.** Blade 4 is `n0` and 5 is
  `ninf`; they were `ep` and `em`. ⭐ **The repair was the basis, not the arithmetic.** 2.20.0 measured
  this as a wrong answer with *no arithmetic fix available*: in the ep/em basis the coefficients are
  `(q−1)/2` and `(q+1)/2`, so `q` is stored as the sum *and* difference of two nearly-equal numbers and
  is destroyed at both ends — at x = 2^-30 the correctly-rounded `ep` **is** exactly −1/2. In the null
  basis `P = p + (q/2)·ninf + n0`, `q` lives in **one** coefficient, so the norm reconstructs it
  directly instead of recovering it from a cancellation.
  - Median `|P·P| / q` over 2000 full-mantissa points per binade: **1.0 → 4.48e-17 at 2^-30**,
    1.05e-16 → 4.70e-17 at 2^0, **1.0 → 4.33e-17 at 2^+30**. A relative error of 1.0 is total loss, so
    both tails go from losing the value outright to sub-ulp, and the floor is flat rather than a U.
  - Distance recovery `P₁·P₂ = −d²/2`: **median 46.4× wrong at 2^-30 and 48.7× at 2^+30 → 1.76e-15 and
    1.93e-15**, centre 3.8e-15 → 1.87e-15.
  - Bit-exact nullity improves but is **not** claimed as exact: **6761 of 6800 non-null → 5194**. See
    the compensation entry below for why chasing 0 was reverted.
  - ⛔ **A translator claim in the first draft of these notes was FALSE and an audit deleted it.** It
    read *"`cga_norm(cga_translator(t,0,0))` is now exactly 1 for all 1001 binades; it failed 526
    before"*. Measured against the shipped 2.20.0 tree: **both trees fail 485 of 2046 binades over the
    full normal range, first at 2^539 — identical.** The basis change did nothing for the translator,
    and the "526" came from the same non-shipping configuration as the bad nullity baseline.
  - ⭐ **And it is faster for sparse operands, which is what CGA primitives are:** `point*point`
    **5328 → 2404 µs (2.22×)**, `cga_norm_sq(point)` **5068 → 2597 µs (1.95×)**, because a table lookup
    replaces the inline bit-manipulation product and the zero-skip means only a point's 5 occupied slots
    reach the inner loop. ⛔ Past **k ≈ 8** occupied blades it inverts — see the k-sweep above, up to
    **10.93× slower at k = 32**.
- **geo_advanced** — `_cga_scalar_of_geo` sums **all pairs**, not the diagonal. In an orthonormal basis
  the scalar part of `a*b` comes only from pairs (i, i); in the null basis `n0*ninf` has one too.
  ⛔ With the basis flipped and this loop still diagonal-only, points were non-null for **6800 of
  6800** — the `−q` that cancels `+q` lives exactly in the pair it was skipping.
- **geo_advanced** — 2.18.0's Neumaier compensation in `_cga_scalar_of_geo` was **removed and then
  restored**, and the round trip is the finding. Removing it buys bit-exact nullity (5194 of 6800
  non-null → 0) by making the norm repeat `cga_point`'s own rounding of `q` so the two cancel. ⛔ It
  also fabricates zeros: `M = 1 + a·e1 + a·n0 + (a/2)·ninf` has scalar part exactly 1 and read **0 for
  6 of 11 magnitudes, all at a ≥ 2^30**. **An exact zero obtained by making the same rounding error
  twice is an artifact, not accuracy** — and distance recovery was *worse* without the compensation
  (1.85e-15 against 1.76e-15), so the trade bought nothing but the headline. Restored, and the witness
  is now asserted.
- **geo_advanced** — the null-table builder hoists `_cga_expand_null`/`_cga_contract_orth` out of the
  pair loop and iterates occupancy lists instead of all 32 slots. **First CGA call 2.7 ms → 0.72 ms**
  (2.20.0: 11 µs). The table is byte-identical — pinned by the FNV contract, which is what made the
  change safe at tag time.

### Fixed
- **tests** — the two 2.20.0 acceptance pins have **inverted, exactly as designed**. They asserted the
  failures at 2^-30 and 2^+30 as KNOWN WRONG so a repair could not land quietly; the basis change made
  them correct and the suite failed until they were rewritten. **A pinned defect is a tripwire for its
  own fix**, which is worth more than a comment saying the fix is pending.
- **upstream (cyrius)** — filed `2026-09-11-nested-continue-binds-to-wrong-loop.md`: on cycc 6.6.2,
  when a loop and a loop nested inside it **both** contain a `continue` and the outer one appears
  lexically **before** the nested loop, both bind one level too far out — the inner `continue` jumps to
  the **outer** loop's latch, and the **outer `continue` becomes a no-op**.
  ⛔ **Silent wrong code, no diagnostic.** The natural sparse-skip idiom for a 32×32 table is both
  shapes at once and produced an **all-zero 1024-entry table** while reporting success. Nothing failed;
  the values were simply absent.
  ⚠ **The first version of this filing had the symptom wrong and was corrected before it was acted on.**
  It said the inner `continue` "exits the OUTER loop"; instrumented, the outer loop is **not**
  terminated — its body is entered all 3 times while the inner body runs 3 of 9 — and the outer
  `continue` breaking too was missing from the filing entirely. **A filing is a measurement like any
  other.**
  ⚠ **The decisive case has an outer `continue` that never fires**: `if (c == 99) { continue; }` for
  `c` in 0..2 cannot execute, yet its mere presence breaks the inner one — and moving it *below* the
  nested loop makes the same program correct, which isolates the trigger to lexical order.
  ⚠ The repro **proves itself**: exit 0 if the compiler is correct, exit 1 while the bug is present,
  across ten shapes plus three instrumentation counters, so a failing run also shows which shapes are
  correct on that build. **Not bisected** — only 6.6.0–6.6.2 are installed and hisab pins 6.6.2, so
  establishing a first-bad-version would mean repinning; stated rather than guessed, and this project's
  own history records a first-bad-version as evidence about *visibility* rather than *origin*.
  ⭐ hisab's table builder is written with `if`-guards and carries a comment saying **not to tidy them
  back into `continue`** until the issue closes.

### Added
- **tests/modules.tcyr** — a **compensated-scalar witness**: `M = 1 + a·e1 + a·n0 + (a/2)·ninf` has
  scalar part exactly 1, and `M*M`'s scalar part is asserted across 11 magnitudes. ⛔ It exists because
  **removing the Neumaier compensation passed all 3989 assertions** while returning 0 where the answer
  is 1 for 6 of those magnitudes. Verified discriminating: 2071 passed / 1 failed without the
  compensation, 2072 / 0 with it. Suites **3989 → 3991**.
- **geo_advanced** — the **null-basis product table**, built once at first use by **integer arithmetic
  only** (no f64 touches it, so it is exact by construction rather than by tolerance) via the change of
  basis `n0 = (em − ep)/2`, `ninf = ep + em`. It **reproduces the derivation's contract exactly**:
  FNV-1a `0xF4A98C5706D5CF5B` over all 1024 blade-index entries, with `n0² = 0`, `ninf² = 0`,
  `e1² = +1`, and `n0*ninf` filling **both** term slots (scalar + bivector). Two independent
  implementations of the same algebra agreeing bit for bit is the point — either alone could be
  confidently wrong.
- **scripts/derive-cga-null-table.sh** + a CI gate — 2.21.0's basis change rests on claims about the
  null-basis product table, and this **derives all of them from first principles** rather than asserting
  them. ⚠ It exists as a script because 2.20.0's audit found several of that release's headline figures
  could not be re-derived from the tree: the probes were never committed. ~200 ms, deterministic, and
  **each of its 6 claims is verified to exit 1 when broken** — which two of them were not, see above.
  - **At most 2 terms** per basis-blade product in the null basis — 128 of 1024 pairs vanish, 768 give
    a single blade, 128 give two.
  - **Every table coefficient is exactly ±1.** No halves survive the change of basis, so the
    null-basis product introduces **no new rounding** — the property that makes this change safe.
  - **The null table agrees with the shipped orthonormal one** under the change of basis, 0 mismatches
    over all 1024 pairs. The algebra is unchanged; only its coordinates move.
  - **A nullity MODEL**, labelled as such in its own output: it models ep/em and null arithmetic in
    Python and must never be quoted as the library's behaviour. That confusion produced this release's
    worst published figure.
  - **The defining null-basis identities**: `n0² = 0`, `ninf² = 0`, `n0·ninf = −1`, the symmetric sum
    `n0*ninf + ninf*n0 = −2` (the bivector cancels), and `e1² = +1`.
  - **The table emitted in BLADE-INDEX space**, which is `_cga_geo_blades`'s actual calling convention,
    with **FNV-1a `0xF4A98C5706D5CF5B`** over all 1024 entries as the exact contract the Cyrius table
    must reproduce. ⛔ **That claim exists because I got the index space wrong first** — blade index 4
    is bitmask 8, and my first spot-check read `(4,4) → +1·scalar` as `n0² = +1` when it was `e3² = +1`
    in the other numbering.
  ⚠ **My first ad-hoc derivation of the term distribution was wrong** — it printed 256/448/320 against
  the script's 128/768/128. The script carries a round-trip cross-check that the ad-hoc version had no
  equivalent of. **A derivation without a cross-check is a guess that compiles.**

### Changed
- **geo_advanced** — `_cga_geo_blades` now returns **two term slots** instead of one, and all five call
  sites accumulate both. **Output was bit-identical at that step**: a folded checksum over 300 random
  multivector pairs through every product, contraction, sandwich, dual, reverse and norm, plus 60
  magnitudes of every constructor, read `ff49e7071c7f4fc` before and after.
  ⚠ **And the checksum was shown to detect a change before being trusted** — moving the metric from
  `em` to `ep` gives `24f3bb7be87992d4`, corrupting the blade mask gives `b817903c0089f0cf`.
  ⚠ **The new slot was shown to be live, not dead code that merely compiles**: populating slot 1 with a
  synthetic second term moves the checksum to `bdf2efa0662f3b30` and removing it restores exactly. A
  widening whose new path is never taken would have passed every test here.
  ⚠ **This costs something and the figure is measured**: iterating the second slot unconditionally is
  **+4.5% on a dense product and +7.9% on `point*point`**; guarding it on `sign1 != 0` brings that to
  **+3.1% and +6.4%** against the one-term original, three runs each with <1% spread. ⚠ The *sparser*
  operand pays **more**, because loop overhead is a larger share of the work when fewer pairs reach the
  arithmetic — the opposite of the intuition that sparse inputs are cheap.


## [2.20.0] - 2026-09-10 — the four repairs 2.19.0 measured, and what measuring them again showed

2.19.0's decision fan-out proved four filed items were **repairs, not decisions**. Two were repaired
and two turned out to be characterisations — because measuring them a second time showed the filing
was wrong about the size, the direction, or the safety of the obvious fix. Suites **3955 → 3983**.

⛔ **ABOVE 2^63 BOTH SYMBOLIC RENDERERS RETURNED ONE WRONG CONSTANT FOR EVERY INPUT.** `f64_to`
saturates, so `expr_to_str` and `sym_to_latex` each answered **-9223372036854775808** for 1e19, 1e20 and
2^100 alike — a *negative* number for a *positive* input — and because the LaTeX path writes the sign
separately, `sym_to_latex(-1e19)` was the malformed `{--9223372036854775808.000000}`. The filing called
this "30 diverging values" and framed the *deletion* as the risk; it is the entire finite band above
2^63 and the shipped code was what was wrong. ⛔ **The row named one function; the grep found two** —
`expr_to_str` carries the same saturation with no magnitude guard at all, the sixth release running
this class was wider than its filing. Repaired by exact decimal expansion with **no f64 arithmetic in
the result**, verified against Python's arbitrary-precision `int(float)`: **10,961 renderings, 0
mismatches**.

⛔ **A CONFORMAL POINT IS NEVER EXACTLY NULL, AND THAT IS A WRONG ANSWER, NOT A REPRESENTATION
NICETY.** Distance recovery — `P₁·P₂ = −d²/2`, what CGA points are *for* — has median **46.4× at 2^-30 and 48.7× at 2^+30** — symmetric tails, against a median of exactly 0 at 2^0. ⚠ A MEDIAN, never a maximum: `rel ~ 2ab/(a−b)²` diverges as the pair closes, so the max is set by the nearest pair drawn (5.6e7 / 2.6e8 here). An earlier draft quoted those maxima as "~2000× and ~32000×" and read a 16× tail asymmetry into them; there is none. ⛔ The filing
said "small coordinates"; it is **both tails**, and the small-only reading was an artifact of a
power-of-two sweep, since dyadic `x` makes `x²` exact. ⭐ No arithmetic fix exists and that is proven:
at x = 2^-30 the correctly-rounded `(q−1)/2` **is** exactly −1/2. The remedy is a basis change, filed
as its own release.

⛔ **THE SUBNORMAL-SVD CLAIM WAS FALSE IN BOTH HALVES AND HAD BEEN ASSERTING SOMETHING REASSURING SINCE
2.18.0.** It read *"the remaining 28 are the GRID, NOT A DEFECT ... the routine reports
`HSB_ERR_NO_CONVERGENCE`, which is the honest answer."* Over 8 fixtures × 52 subnormal scales against a
120-digit oracle built from the **actual rounded entries**: **270 of 416 rows return `HSB_ERR_NONE` and
only 57 are right** — 213 confident wrong answers, worst **9.78×**, one smallest singular value
returned as exactly **zero**. ⛔ The old sweep could not have seen it: every fixture was
upper-triangular. A **general** 2×2 is **1 correct / 86 loud / 121 silent**. ⚠ No repair shipped: the
obvious fix was refuted on scope in 2.19.0, and this row's own instruction was *"widen the fixture
family before touching the code."* That family now exists as the repair's acceptance test.

⛔ **AND A CHANGELOG SENTENCE HAD THE `triangulate_polygon` DIVERGENCE BACKWARDS, TWICE, FOR THIRTEEN
RELEASES.** Built the **2.7.0 tree and HEAD side by side on cyrius 6.6.2**: the reproducer gives **old 9
(COMPLETE) → new 6 (PARTIAL)**. The new code *bails where the old succeeded*. The decision is unchanged;
the sentence was wrong, and it lived **only in the CHANGELOG, where no gate could reach it**. It is now
in the source beside the code and pinned by an **exact**-length assertion — the old `<= 9` form is
satisfied by the complete answer too, which is how a backwards claim survived thirteen releases.

⭐ **`ad_grad`'S JACOBIAN TRAP IS O(m²), AND THE "6.8×" IN ITS FILING WAS ONE POINT ON A CURVE.** The
sweep clears and scans the full tape every call, so a shared-tape driver **quadruples** per doubling of
m (461 → 1240 → 4667 → 18398 µs) while the per-residual reset **doubles** (215 → 391 → 775 → 1474). The
ratio is 2× at m = 256 and 12× at m = 2048 and grows without bound, so it must never be quoted as a
constant. No code change: the scan is O(root) on its own, so halving the clear leaves it quadratic.

⛔ **AND ADDING A BENCHMARK CHANGED A DIFFERENT BENCHMARK BY 31%.** The two new `jac_rev` rows build
tapes under a bump allocator that **never frees**, and when they registered 19th/20th they shifted the
heap for everything after: `kdtree_radius_4k` read **484 ns with them and 696 ns without**, against 706
ns in the previous release. Without checking, 2.20.0 would have claimed a **44% speedup in a module it
never touched**. They register last now, and `kdtree_radius_4k` reads 709 ns against its 706 ns
baseline. ⚠ Registering last confines the effect; it cannot remove it, and anything added after that
line inherits it.

⚠ **Four of my own instruments were wrong before the thing measured was.** A `1 << (73 - e)` scale
factor reached a shift of 71 that x86 **masks to 7**, so a 416-row sweep silently swept duplicates; a
CGA fixture used *dyadic* values — the exact artifact it was written to document — and built them with
the **subnormal** shift form at a normal exponent, testing a value 288 decades off and yielding a NaN
that `f64_gt(NaN, 1)` read as "not wrong"; a provenance marker claimed agreement over "2098 binades", a
figure carried from an unrelated sweep before anything ran (it is 1024); and a rename was called inert
on a checksum I had not shown could detect a change — two positive controls later, it could.

⛔ **THE RELEASE WAS AUDITED BEFORE TAGGING, BY FIVE INDEPENDENT RE-MEASUREMENTS, AND IT FOUND A DEFECT
THIS RELEASE HAD SHIPPED PLUS SIX WRONG FIGURES IN ITS OWN NOTES.** ⛔ **The two benchmarks 2.20.0 adds
overflowed their own tape**: `_b_jac_shared` sized it `N + 4*M + 8` = 1040 while the loop pushes
`N + M*N` = 2056 nodes, so `_ad_push` returned `AD_FULL` for half the residuals and the row measured
**half the work while reporting success** — a benchmark on its own truncated case. The scratch probe had
the same bug, so the whole O(m²) table was ~2.6× low on the shared side. Both fixed; the probe now
asserts the push did not overflow. ⛔ **A hand-encoded constant was mis-transcribed and the gate could
not see it**: `src/calc_ext.cyr`'s `F64_1E_NEG30` decodes to 9.99999999979426e-31 against a documented
1e-30, and because its comment sat on the line *above* the declaration, `check-constants.sh`'s regex
matched neither the verified nor the skipped path — it reported a confident **158/158 over a population
of 159**. That is the same shape as the 2026-08-04 finding the gate's own header records. The value is
corrected, the gate now falls back to a preceding comment line, and it is **verified to fire on the
previously-blind shape**: 159/159 clean, exit 1 with the offender named when a wrong value is planted
either way round. ⚠ The numeric consequence was nil — it is a zero-secant threshold — but it is exactly
the class the gate exists to catch. ⛔ **And the CGA magnitudes in this release were maxima presented as
the effect.** `rel ~ 2ab/(a−b)²` diverges as the pair closes, so "~2000× and ~32000×" were the nearest
pairs two 300-sample draws happened to contain, and the 16× "tail asymmetry" read into them does not
exist: over 2000 pairs per binade the **medians are 46.4× and 48.7×**, symmetric, against exactly 0 at
2^0. Also corrected: the SVD factor (28 / 2.5964 = **10.8×**, not the 9.3× obtained by rounding the
truth to 3 first), the kdtree perturbation figures (quoted from ad-hoc runs; the CSV records **480 and
396 ns perturbed against 668 ns after the reorder**), the jac_rev first-run pair (**337.3/184.8 µs** per
the record), and the claim that `ad_grad` has "zero in-tree callers" — it has five.
⚠ **Several 2.20.0 figures are not reproducible in-tree** and are flagged rather than quietly kept: the
10,961-rendering oracle sweep, the 416-row SVD classification, the CGA band sweeps and the `ad_grad`
scaling table all come from probes that were never committed. The in-tree assertions pin specific cases
from each, but a future reader cannot re-derive the aggregate figures without rebuilding the harness.

**No performance change is claimed, and the control is what says so.** Against the 2.19.0 release run,
2.20.0 is **median +2.73%, mean +2.81%, zero rows past 10%** — quieter than running the identical
binary twice (median +2.82%, **13** rows past 10%). The board drifted uniformly under a load of 1.34.
⚠ Two runs recorded at `d740afc` carry the benchmark-ordering perturbation described above and should
not be compared against.


### Changed
- **autodiff** — `ad_grad`'s cost and the Jacobian trap it creates are now documented where a caller
  will see them. The sweep is O(tape length) **twice** — it clears every adjoint, then scans from
  `root` down to index 0, both over the full tape rather than the part `root` reaches. So the obvious
  multi-residual Jacobian driver (build all m residuals on one shared tape, sweep per residual) is
  **O(m²) by construction**. The per-residual `ad_tape_reset` form is O(m) and needs **no new API**.
  Measured at n = 8, load ~1.0: shared **quadruples** per doubling of m (1214 → 4860 → 17779 → 67232 µs
  at m = 256/512/1024/2048), per-tape **doubles** (227 → 387 → 748 → 1425 µs).
  ⛔ **The first version of this table was measured on a TRUNCATED TAPE and read ~2.6× low on the shared
  side** (461/1240/4667/18398 µs): the probe sized its tape `n + 4*m + 8` while the loop pushes
  `n + m*n` nodes, so `_ad_push` returned `AD_FULL` for half the residuals and the driver measured half
  the work while reporting success. The two benchmark rows this release adds shipped with the same bug;
  both are fixed and the probe now asserts the push did not overflow.
  ⚠ **The ratio is not a constant and must not be quoted as one** — 5× at m = 256, 47× at m = 2048,
  growing without bound. An earlier note recorded this as "6.8× faster", which is one point on a curve;
  the finding is the *scaling*, not the point.
  ⚠ No code change: halving the clear would not help, because the scan is O(root) on its own, so the
  driver stays quadratic. Making it sub-linear needs a reachability sweep instead of a linear scan —
  an algorithm change, not a tuning knob. And the full clear is load-bearing as written, since
  `ad_grad_of(t, i)` above `root` must read 0 and has no way to know where the last sweep began.
  ⚠ **Correction to an earlier draft of this entry**: it said `ad_grad` has "zero in-tree callers".
  It has five — `ad_grad_into` at `src/autodiff.cyr:631` (a public entry point documented as *"the form
  an optimizer's gradient closure wants"*) plus four test sites. What is true is the narrower claim:
  **no other library module calls it**, and the 2.19.0 fan-out found no `dual_*`/`ad_*` call site in any
  consuming repo. So nothing is hitting the quadratic path today — it is a trap set for the first caller
  who builds a Jacobian by pairing reverse mode with `opt_levenberg_marquardt`, its documented purpose.

### Fixed
- **collision_core** — a local named `pn` in `triangulate_polygon`'s neighbour-refresh block shadowed
  the `pn` bound by the winding loop **at the top of the same function**, where it is an `HVec2`
  *pointer* against a ring *index* here. Renamed to `prev_slot`/`next_slot`. ⚠ Inertness is checked,
  and so is the check: a folded checksum over every index tuple on a 3×3 grid for n=4 and n=5 (65,610
  calls) is bit-identical across the rename, **and the same checksum moves** when the refresh is
  disabled or the prev flag is forced — so it can see a change. `prev_slot = i - 2` does *not* move it:
  an equivalence on this family, not a blind probe. ⚠ The filing also claimed `nn` shadowed something;
  it does not — the winding loop uses `ni`, so `pn` was the only true shadow.
- **CHANGELOG (2.7.1 entries)** — the `triangulate_polygon` prune's divergence direction was recorded
  **backwards, twice, for thirteen releases**: *"old-partial → new-complete — the new code succeeds
  where the old bailed."* Measured by building the **2.7.0 tree and the current tree side by side on
  cyrius 6.6.2** and running both on `(0,0),(1,2),(1,1),(0,2),(2,1)`: **old returns 9 indices
  (COMPLETE), new returns 6 (PARTIAL)** — old `[0 1 2 4 0 2 4 2 3]`, new `[4 0 1 1 2 3]`. The new code
  *bails where the old succeeded*. ⚠ The decision is unchanged — the divergence is confined to
  self-intersecting input, which ear clipping does not define, and a partial return is inside the
  documented `< 3*(n-2)` contract. What was wrong was the sentence, and it lived **only in the
  CHANGELOG, where no gate could reach it**. The contract and the measured direction are now stated in
  `collision_core.cyr` beside the code, and pinned by an assertion.

### Changed
- **linalg_precision / tests** — the standing claim about subnormal SVD block ratios was **false in
  both halves**, and it had been asserting something reassuring since 2.18.0. It read: *"the remaining
  28 are the GRID, NOT A DEFECT ... the routine reports `HSB_ERR_NO_CONVERGENCE`, which is the honest
  answer to an unanswerable question."*
  ⛔ **(1) It is a defect.** Scaling the active block into the normal range before the QR sweep answers
  ratios the shipped tree gets wrong, so the subnormal grid was never the binding constraint — the
  binding constraint is that the sweep does its arithmetic *on* that grid.
  ⛔ **(2) It mostly does not report `NO_CONVERGENCE`.** Measured over 8 fixtures × 52 subnormal scales
  = 416 rows, against singular values computed from the **actual rounded f64 entries** at 120 decimal
  digits: **270 rows return `HSB_ERR_NONE` and only 57 of those are right.** 213 are reported successes
  with singular values wrong by more than 2 ulp — worst **9.78×**, including a smallest singular value
  returned as **exactly zero** where the truth is 4 units of 2^-1074. 146 fail loudly.
  ⛔ **(3) The old sweep could not have seen it**: every one of its fixtures is an **upper-triangular**
  2×2 block. Split by family — upper is 56 correct / 60 loud / 92 silent; a **general** 2×2 with a
  non-zero lower-left entry is **1 correct / 86 loud / 121 silent**. The routine is essentially never
  right there.
  ⚠ **No repair is shipped here, deliberately.** The obvious candidate — scale the active block, step,
  unscale — was refuted on scope by an adversarial verifier in 2.19.0 (validated only on
  upper-triangular blocks, and on a general 2×2 it converts loud failures into silent wrong answers),
  and the roadmap's own instruction was *"widen the fixture family before touching the code."* That
  family now exists and is in the suite as the repair's acceptance test. The repair is an algorithm
  change inside `_lp_bidiag_qr` and is filed as its own release.

### Changed
- **geo_advanced** — `cga_point`'s nullity is now a **measured, tested property with a stated band**
  rather than an open question. ⛔ **It is a wrong answer, not a representation nicety.** The canonical
  use of a null conformal point is distance recovery, `P₁·P₂ = −d²/2`; measured over random
  full-mantissa coordinates the relative error in the recovered `d²` has median **46.4× at 2^-30 and 48.7× at 2^+30** — symmetric tails, against a median of exactly 0 at 2^0. ⚠ A MEDIAN, never a maximum: `rel ~ 2ab/(a−b)²` diverges as the pair closes, so the max is set by the nearest pair drawn (5.6e7 / 2.6e8 here). An earlier draft quoted those maxima as "~2000× and ~32000×" and read a 16× tail asymmetry into them; there is none. Clean bands, each
  0 of 400 samples past its tolerance: **1e-9 → 2^-7..2^3, 1e-6 → 2^-11..2^10, 1e-4 → 2^-15..2^13**.
  A band quoted without its tolerance means nothing, so all three are recorded.
  ⛔ **The filing said "small coordinates, below ~2^-26.5". It is BOTH tails**, and the small-only
  reading was an artifact of a power-of-two sweep — dyadic `x` makes `x²` exact, which is the only
  reason the failure looked banded. `|P·P|/q` is a clean U centred on |x| ≈ 1: 2^-51 at 2^0, and
  **1.0 — total loss — at 2^-30 and at 2^30 alike**.
  ⭐ **No arithmetic fix exists, and that is proven rather than assumed.** At x = 2^-30, q = 3·2^-60,
  so (q−1)/2 = −1/2 + 1.5·2^-60 — more than a full ulp below the rounding boundary of 1/2, so **−1/2
  IS the correctly-rounded value** (asserted bit-exactly). The ep/em basis stores q as the sum and
  difference of two nearly-equal numbers; the remedy is the null basis (n0/ninf), where q lives in one
  coefficient and `P·P` cancels the same computed q against itself. That is a change to the 32-slot
  blade product tables and is filed on the roadmap rather than folded in here.

### Fixed
- **symbolic / symbolic_ext** — above 2^63 **both public renderers returned the same wrong constant for
  every input**, and all 3955 assertions passed. `f64_to` SATURATES, so `f64_to(f64_floor(mag))` is
  `i64::MIN` for any |val| ≥ 2^63: `expr_to_str` and `sym_to_latex` each answered
  **-9223372036854775808** for 1e19, 1e20 and 2^100 alike — a *negative* number for a *positive* input.
  ⚠ And because the LaTeX path writes the sign separately, negatives came out malformed:
  `sym_to_latex(-1e19)` was `{--9223372036854775808.000000}`, a **double minus**.
  ⭐ Repaired with an exact decimal expansion — every f64 at |val| ≥ 2^52 is an integer, so there is no
  fractional information up there and no rounding to do; the renderer seeds a little-endian digit array
  with the 53-bit mantissa and doubles it k times, with **no f64 arithmetic touching the result**.
  Verified against an independent exact oracle (Python's arbitrary-precision `int(float)`): the 961
  binades 2^63..2^1023 plus 10,000 signed random mantissas, **10,961 renderings, 0 mismatches**.
  ⛔ **The earlier filing was wrong about the size and the shape of this.** It recorded "30 diverging
  values" and framed the current output as well-formed and the *deletion* as the risk. It is the entire
  finite band above 2^63, the shipped code is what was wrong, and the negative case was already
  malformed before anything was deleted.
- **symbolic_ext** — `_latex_fmt_const` capped its integer path at 1e15, and **the cap was the bug**.
  The function's own contract, one line above it, is *"Integer values render without decimals"* — yet
  every integral value from 1e15 up fell through to the 6-decimal float path and rendered
  `1000000000000000.000000`, while `expr_to_str` rendered `1000000000000000`. The two public renderers
  disagreed across that whole band. The cap was never a design choice: it guarded against `f64_to`
  saturating and sat ~3 binades below where saturation starts, so it matched neither the contract nor
  the hazard. Now bounded by the real hazard and exact beyond it — measured, the two agree on
  **1024 of 1024 integral binades**, 2^0..2^1023.

### Added
- **tests/hisab.bcyr** — `jac_rev_shared_256` and `jac_rev_pertape_256` (**72 → 74 benchmarks**), so
  the quadratic/linear gap is tracked rather than rediscovered. ⚠ They must be read as a **pair** and
  never as a constant speedup, for the reason above. First recorded run (`bench-history.csv`, d740afc): **337.3 µs against 184.8 µs**. ⚠ An earlier
  draft quoted 328.6/180.0 from an ad-hoc run that appears in no row of the record.
- **tests/modules.tcyr** — 3 assertions pinning the self-intersecting divergence (**3980 → 3983**).
  ⚠ The length is asserted **exactly** (`== 6`), not `<= 9`: the old `<= 9` form is satisfied by the
  complete answer too, so it could not tell the two directions apart — which is how a backwards claim
  survived thirteen releases.
- **tests/hisab.tcyr** — 7 assertions pinning the true subnormal-SVD state (**3973 → 3980**): a
  general-2×2 fixture family, the fabricated-zero case, the 9.3×-too-large case, the surviving loud
  arm, and a 156-pair sweep whose sample count is asserted alongside its result. These are a
  **characterisation of a tracked defect, not a contract** — they exist so the repair has an acceptance
  test and so any movement announces itself.
  ⚠ My own sweep was wrong twice before the measurement stood: the scale factor was built as
  `1 << (73 - e)`, which reaches a shift of 71 that x86 **masks to 7**, so the sweep silently
  duplicated scales; and the first oracle computed truth from the *ideal* integers rather than the
  rounded f64 entries the routine actually sees. The second error turned out not to change the counts,
  but it was wrong for a reason that could have.
- **tests/modules.tcyr** — 7 assertions pinning CGA point nullity (**3966 → 3973**): the bit-exact
  representational limit, a 22-binade distance-recovery sweep with its pair count asserted, and — so a
  future repair has to announce itself — the **known failures at 2^-30 and 2^+30 pinned as failures**,
  both tails.
  ⛔ **The first draft of this block repeated the exact mistake it documents, twice.** It used *dyadic*
  fixtures (the artifact that produced the original "small coordinates" reading), and built them with
  the **subnormal** form `1 << (1074 + e)` at e = −30 — a shift of 1044, masked by the hardware to 20 —
  so it silently tested a value 288 decades from the one intended. That produced a NaN relative error,
  and `f64_gt(NaN, 1)` is 0, so the "known wrong" pin read as "not wrong" and failed. A second draft
  then asserted the 1e-6 band against `_SYM_EPS` (2^-50) and failed 22 of 22. **The band is real; the
  fixture was wrong three different ways first.** 3 mutants on `cga_point` killed, no-op control
  survived.
- **tests/modules.tcyr** — 11 assertions pinning exact integer rendering (**3955 → 3966**): 2^63 itself,
  1e19, 2^100 at 31 digits, the 1e15 contract case, and a 1024-binade agreement sweep between the two
  renderers whose **pair count is asserted too**, because a loop that ran zero times would report zero
  disagreements. ⚠ The negative case is asserted on its exact string, not its magnitude — the defect
  there was a doubled sign, which a magnitude check could not see. 6 mutants installed, 6 killed, with
  a no-op control that correctly **survived**, proving the harness discriminates.


### Changed
- **roadmap / issues** — swept for orphaned work before 3.0.0. **Two items still tagged `[2.17.0]`
  were already fixed and their checkboxes never ticked**, both re-verified on the shipped tree rather
  than closed on inspection:
  - *Scaled norms on the exp side* — `su2_exp` and `so3_from_axis_angle` route through `_lie_norm3`
    since 2.17.0. The round-trip floor measures **2^-537 → 2^-1022**, the whole normal range.
    ⚠ The probe was checked against the defect first: rebuilt with 2.16.0's `lie.cyr` and everything
    else current it reports exactly **2^-537**, the figure the row states — so it discriminates
    rather than always printing the same number.
  - *`cga_norm` deferred with reasons* — 2.18.0 met both stated conditions (a scale construction that
    works for subnormals, and an `f64_div` rescale rather than a reciprocal multiply). Measured over
    51 subnormal max coefficients (2^-1023..2^-1073): **0 NaN, 0 fabricated zeros, 51 finite
    positive**, where the row predicted NaN. ⚠ Its cited line was stale (`:2438` → `:2557`).
- **issues/** — the last open filing, `2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md`,
  is closed and archived; **28 of 28 archived, none open.** It had recorded neither 2.18.0's decline
  of the seed trade (the "upgrade" measures **36,000× less accurate**) nor 2.19.0's repair of the
  thing it was actually circling — the polish budget, `mpr_penetration` **1.65e-05 → 6.3e-16**.
  Every substantive claim in that filing has now been refuted by measurement, three times over.

### Fixed
- **CI** — the `Discarded #must_use results` step was **red from the commit that added it** (2.19.0's
  `#must_use` bite) and stayed red through the 2.19.0 tag, failing with **no output whatsoever**: no grep
  hit, no `::error`, no summary line. Two independent bugs, both measured:
  - `cyrius check <file>` **without** `--with-deps` parses standalone with no manifest deps in scope, so
    `examples/basic_math.cyr` — which includes `src/` modules reading the stdlib global `F64_ONE` —
    failed with `undefined variable 'F64_ONE'`. GitHub Actions runs `bash -e`, so the `out=$(...)`
    **assignment inherited that non-zero status and aborted the step** before a line of its own logic
    ran. `--with-deps` compiles the example the way a user does and **still detects a planted discard**
    (`warning:<source>:74:26: #must_use result of 'calc_integral_simpson' is discarded`) — verified, not
    assumed, because swapping a hard failure for a vacuous pass would have been the worse bug.
  - The step **name was unquoted**, so YAML read ` #must_use results` as a comment and the step appeared
    in the UI as plain `Discarded`.
  ⛔ **The gate was only ever proven on one of its two arms.** 2.19.0 verified it by planting a discard
  in `src/ode.cyr`, which reaches it through the bundle; the `examples/` arm was never exercised and the
  YAML was never executed under `bash -e`. **Proving that a gate fires is not the same as running it the
  way CI runs it.** All four arms are now verified — clean 0, example discard 1, bundle discard 1
  (`src/num_ext.cyr`, caught at `dist/hisab.cyr:15048`), restored 0 — and all 24 `run:` steps in
  `ci.yml` plus all 12 in `release.yml` were executed locally under `bash -e`: **21 of 24 and 12 of 12
  green**, the only failures being two that need the Actions environment (the toolchain installer and
  `${{ github.base_ref }}`).
- **CI** — the `Duplicate global/enum symbols` step carried the identical unguarded-assignment shape and
  would have aborted just as silently had the bundle ever failed to compile. Both steps now capture the
  exit code and **report a compile failure with its output** instead of vanishing.
  ⚠ `release.yml` audited for both bugs and is clean; the 2.19.0 tag's release run was unaffected.
- **CI / release** — the Fuzz step built every harness to `build/$name`, and `tests/hisab.fcyr` yields
  the name `hisab`, so it **overwrote `build/hisab`** — the CLI binary the ELF and printed-version gates
  check. Harmless today only because those three steps run before Fuzz; a reorder, or any later step
  reading `build/hisab`, would have silently inspected the fuzz harness. Now `build/fuzz-$name`, in both
  workflows. Found while repairing the step above, not by anything failing.
  ⚠ No library source, `dist/hisab.cyr`, or consumer-visible behaviour changes here.


## [2.19.0] - 2026-09-10 — the 3.0.0 prep, and five gates that could not fail

The roadmap scoped this as non-breaking groundwork for `Result<T,E>`: `#must_use` on the fallible
surface, negative enum values for the 12 `HSB_ERR_*` codes, and the decisions owed. All of that shipped.
**But every piece of it arrived the same way — the annotation, the enum, the version string — each
needing a gate, and each gate turning out to be one that could not have failed.** Suites **3937 → 3955**.

⛔ **`#must_use` IS A COMPILER DIAGNOSTIC, NOT A LINT ONE.** A file that discards a `#must_use` result
gives `cyrius lint` *"0 warnings"* and matches CI's `^  warn ` grep **zero times**, while `cyrius build`
prints `warning: #must_use result of 'f' is discarded`. Annotating 44 functions without wiring a step
would have been decorative. ⚠ The row said 167 sites; it is **44 functions** — 167 is the number of
`return HSB_ERR` *statements*, and 4 of the 48 functions that can return one can only ever return
`HSB_ERR_NONE`, so annotating them would train readers to ignore the annotation. The new gate,
**verified to fire before being trusted**, found `examples/basic_math.cyr` discarding two fallible
returns.

⛔ **THE SUITE COULD NOT SEE A WRONG ERROR CODE AT ALL.** Converting the 12 codes to an enum meant
re-typing twelve values by hand, so the conversion was mutation-tested — and **two mutants survived all
3940 assertions**: `HSB_ERR_SINGULAR_MATRIX -2 → -22`, and `HSB_ERR_ALLOC -11 → -1`, which makes ALLOC
**collide with `HSB_ERR_INVALID_TRANSFORM`** — two distinct failures reported under one code. The cause
is structural rather than an oversight in any one test: **every existing assertion compares a return
against a code by NAME, so both sides move together and the value cancels out.** 15 assertions now name
the number and check all 66 pairs for distinctness; 6 of 6 mutants killed. ⚠ A duplicate enum member
*is* diagnosed at file:line where a duplicate `var` global is completely silent — but
`cyrius check --with-deps` prints that warning and still **exits 0**, and `cyrius lint` never emits it,
so that safety benefit was wired to nothing until a gate was added.

⛔ **AND THE RELEASE SHIPPED ITS OWN BROKEN PROVENANCE MARKER, CAUGHT BY THE GATE THAT HAS NEVER RUN.**
The enum bite wrote a `[measured: ...]` wrapped across two comment lines — malformed to the single-line
regex. The gate that catches it, `check-measurements.sh`, is wired
`if: github.event_name == 'pull_request'`, and this repository has **0 pull requests in its entire
history against 303 workflow runs, the last 100 all `event: push`** (checked against the GitHub API).
**It has executed zero times.** ⛔ Its `--diff` mode was *also* blind to its own dominant case: a
path-resolution failure was filed against the PARAGRAPH's first line while the malformed arm filed the
marker's own line, so a bad marker appended to an EXISTING block was intersected away as "not added" —
and it made the gate **greener**, because the fabricated marker SATISFIED the real claim beside it.
Both fixed; broken markers tree-wide **3 → 0**, now gated on push.

⛔ **THE VERSION GATE COMPARED COMMITTED TEXT TO COMMITTED TEXT AND NEVER RAN THE BINARY.**
`src/main.cyr` now prints `CYRIUS_PKG_VERSION`, so the one site `${file:VERSION}` could not reach holds
no second copy — output byte-identical, binary 257,176 B both ways. Measured with `VERSION` bumped: the
old hardcoded form rebuilds and still prints the *previous* version, which is the 2.9.1 → 2.9.2 defect
reproduced. ⚠ **The roadmap's claim that two files "still assert" the stale *"Cyrius has no build-time
string interpolation"* was FALSE** — `ci.yml` contains it zero times; both had been excised sloppily,
splicing the correction into the middle of a sentence, while the two live claims sat in files the row
never named. The claim was true for **four days**: written 2026-08-09, falsified by cyrius 6.5.21 on
2026-08-13, and it stood four more weeks.

⛔ **AND FOUR OF MY OWN INSTRUMENTS WERE WRONG BEFORE THE THING MEASURED WAS.** (1) A `git show
"$c:tests/x"` loop returned 0 for every commit because **zsh reads `:t` as a path modifier even inside
double quotes**, silently truncating the ref. (2) An included-file probe returned `2.17.0` — neither the
real version nor the control — because it ran with `cd` into a scratchpad holding a **stale
`cyrius.cyml` from an earlier session**. (3) A `$?` captured `head` after a pipe rather than the script,
reading a gate's exit 1 as 0 — the third time this repo has recorded that exact shape. (4) **Both
replacement version guards were wrong on their first draft, in opposite directions**: the "no hardcoded
literal" guard **failed on its own tree**, because the comment explaining the change quotes the retired
literal, and the "symbol still used" guard was **vacuous**, matching that same comment's mentions of the
symbol so that deleting the call still exited 0. **A gate that cannot tell code from a comment quoting
code fires on every honest explanation of itself.**

⭐ **THE DECISIONS OWED WERE MEASURED RATHER THAN ARGUED, AND FOUR OF THE ROADMAP'S OWN PREMISES DID NOT
SURVIVE IT.** A per-benchmark noise band is **strictly worse** than the shipped global ±10% (to match its
false-positive rate the band must widen to a 11.1% median, 55.0% max, and it then flags 67 code-changed
cells where the global filter flags 96) — the row that called it "the distinct and more useful item" is
refuted by the only data that exists to build one from. `dual_*` stays declined. The remaining four are
**repairs, not decisions**, and are scheduled rather than folded in: `_sym_render_f64` diverges across
the whole finite `|val| ≥ 2^63` band rather than at 30 values, and the *shipped* tree already renders
`sym_to_latex(-1e19)` as `{--9223372036854775808.000000}`, a double minus; `cga_point` is non-null at
**500/500 trials in every binade from 2^-20 to 2^20** with full-mantissa coordinates, so the recorded
"small coordinates" framing was an artifact of a dyadic sweep making `x²` exact.

**No performance change is claimed, and the control is what settles it.** Against the immediately preceding run 2.19.0 looked like **11 rows improving 10-23%** — every one of them in the mat4 / quat / transforms / jet family, which this release never touched and which contains **0 `HSB_ERR_` references**, so the enum conversion cannot have altered their codegen. The previous baseline had been recorded **twice, 53 seconds apart, from the same binary**, and that pair moves the very same rows by **+31.6%, +26.5%, +26.3%, +25.6%, +21.1%** — the same magnitudes in the opposite direction. The second of the pair was simply a contaminated sample. Measured against the other two baselines instead, 2.19.0 is **median +0.00% (mean -0.29%, 2 rows >10%)** and **median +0.43% (mean +0.62%, 0 rows >10%)**, while the same-binary control is **median +2.82%, mean +5.09%, 13 rows >10%**. The only runtime change in the release is the EPA polish budget 64 -> 128 (`mpr_penetration` worst **1.65e-05 -> 6.3e-16**), measured in isolation at no detectable cost with controls moving as much as the EPA rows. ⚠ **A single adjacent run is not a baseline** — comparing against one would have licensed claiming five wins this release did not earn.


### Changed
- **error** — the 12 `HSB_ERR_*` codes are an `enum HsbError`, not 12 `var` globals. Names and values
  are identical, so every reference is unchanged — **567 on non-comment lines** across `src/` (182)
  and `tests/` (385), 738 counting comments. ⚠ The roadmap row scoping this said 562; that figure is
  not re-used here, it is re-counted, because a number carried forward rather than measured is how
  this project has been wrong before. All twelve values were re-read through a probe and compared
  against the old `(0 - N)` forms — **0 mismatches**.
- **CLAUDE.md** — the "enums for constants" principle **named the wrong mechanism** and is corrected.
  It said "zero `gvar_toks` cost vs. `var` globals"; measured with `CYRIUS_STATS=1` over the bundle,
  `var_table` is **615 on both sides** — cycc's own diagnostic counts "globals+enums+arrays", so an
  enum member occupies an entry exactly as a global does. The saving is relocations, not table slots:
  `fixup_table` **3092 → 2933 (−159)**, `code_size` **748,024 → 747,320 B (−704)** across 170
  reference sites. Right rule, wrong reason.
- **src/main.cyr** — the CLI prints `CYRIUS_PKG_VERSION` instead of a hardcoded `"hisab 2.18.0"`.
  `cyrius build` declares that symbol from `[package].version`, i.e. from `${file:VERSION}`, so the
  version site `${file:VERSION}` could not previously reach no longer holds a second copy — the one
  the 2.9.1 → 2.9.2 bump silently missed, leaving the CLI reporting 2.9.1. Output is **byte-identical**
  (`hisab 2.18.0\n`, 13 bytes) and the binary is **257,176 B both ways**.
  ⚠ Re-probed on 6.6.2 rather than taken from the roadmap's "re-verified" note: it resolves from the
  entry file **and** from an included file, and a negative control confirms the value *tracks* — with
  `VERSION` set to `9.9.9-probe+z` the built binary printed exactly that.
  ⛔ **Constraint**: the symbol resolves from the **building** package's manifest, so it must never
  enter a `[lib]` module — a consumer compiling `dist/hisab.cyr` would get *their* version. Safe here
  (`src/main.cyr` is `[build] src`, not `[lib]`; 0 occurrences in the bundle) and now gated.
- **scripts/version-bump.sh** — the `sed` that rewrote main.cyr's literal is removed; there is no second
  copy to keep in step. It now verifies instead, and would otherwise have warned misleadingly on every
  future bump. Its stale "Cyrius has no build-time string interpolation" note is replaced with what is
  still true: `${file:VERSION}` is a *manifest-side* expansion and cannot reach a `.cyr` string literal.
- **docs/development/threat-model.md**, **docs/doc-health.md** — the two *live* assertions that "Cyrius
  has no build-time interpolation". ⚠ The roadmap said `version-bump.sh:21` and `ci.yml:285` "both still
  assert" it; measured, `ci.yml` contains the sentence **zero** times and `version-bump.sh` only inside a
  note saying it is false. Both had already been excised — sloppily, splicing the correction into the
  middle of a sentence — while the real claims sat in two files the roadmap never named. The claim was
  true for **four days**: written 2026-08-09, falsified by cyrius 6.5.21 on 2026-08-13, and it stood a
  further four weeks.

### Fixed
- **geo_advanced** — `mpr_penetration` was wrong by up to **1.65e-05** on ordinary overlapping spheres
  where `gjk_epa_3d` is 5.57e-16, and the cause was the **polish budget**, not either mechanism the
  roadmap had guessed. `_EPA_POLISH_ITER` 64 → 128: 38 of those rounds are mandatory halvings from
  0.25 rad down to `EPSILON_F64`, leaving only 26 for moves, and the portal seed starts further out so
  it was the one running out. At 128 the two entry points agree to the last bits (**6.3e-16**); 256
  changes nothing further. ⚠ Rare, not systematic — 7 of 4000 pairs exceeded 1e-10. ⚠ Both hypotheses
  originally filed were refuted by measurement: swapping which axis carries the large offset leaves the
  error identical, and both entry points polish on every case.
- **examples/basic_math.cyr** — `calc_integral_simpson` and `num_newton` return values were discarded.
  Found by the new `#must_use` gate, not by review. Output is byte-identical.
- **provenance markers** — 3 broken `[measured: ...]` markers, now **0**. One was introduced by this
  release's own enum bite: a marker wrapped across two comment lines, which the single-line `MARKER`
  regex reports as malformed. The other two (`src/collision_core.cyr`, `tests/modules.tcyr`) named
  `issues/...` paths while the script resolves **from the repo root**, so both needed the
  `docs/development/` prefix — a diagnosis the roadmap had already corrected once, after an earlier
  draft proposed a fix that would have left both markers broken.
- **scripts/check-measurements.sh** — `--diff` mode was **blind to its own dominant case**. A
  path-resolution failure was recorded against `p.lines[0][0]`, the PARAGRAPH's first line, while the
  malformed-marker arm recorded the marker's own line; `--diff` keeps a finding only when its
  `(path, line)` is in the set of lines the branch added, so a bad marker appended to an **existing**
  comment block was intersected away. Measured with one marker in two placements: appended to the
  block at `src/vec3.cyr:126`, `--diff HEAD` exited **0** and printed *"Every measurement claim added
  on this branch names its source"*; the identical marker as its own new paragraph exited **1**.
  ⚠ It was worse than silent — the fabricated marker **satisfied the real claim beside it**, so
  tree-wide unmarked fell by one and the gate got *greener* for the bad edit. Now records the marker's
  line; both placements fail, and a clean tree still exits 0.

### Added
- **`#must_use` on the fallible surface** — 44 annotations across 12 modules (`num_ext` 14,
  `linalg_precision` 5, `num` 5, `optimize` 5, `linalg_ext` 4, `calc_ext` 3, and 8 more); the tree goes
  **200 → 244**. ⚠ **The count is 44 functions, not the roadmap's 167.** 167 is the number of
  `return HSB_ERR` *statements*; `#must_use` goes on functions, of which 48 can return one — and **4 of
  those can only ever return `HSB_ERR_NONE`** (`_lp_bidiagonalize`, `_lp_tridiagonalize`,
  `num_halton_2d`, `ode_dopri45`), so annotating them would train readers to ignore the annotation.
- **CI** — a `Discarded #must_use results` gate. ⛔ Without it the annotations were decorative: a file
  that discards one gives `cyrius lint` *"0 warnings"* and matches the `^  warn ` grep **zero times**,
  while `cyrius build` prints the warning. Covers the bundle (all 35 modules with the stdlib resolved,
  so every intra-library call site) and `examples/`, deliberately **not** `tests/` — calling a fallible
  function and asserting on its out-parameter is a legitimate testing idiom and 67 sites do exactly
  that. Verified to fire: a deliberate discard in `src/ode.cyr` takes it 0 → 1, removing it returns 0.
- **tests/hisab.tcyr** — 15 assertions pinning each error code's exact **value** and all **66 pairs**
  for distinctness, plus that every failure code is negative. Written because a mutation sweep found
  the suite could not see a wrong one: `HSB_ERR_SINGULAR_MATRIX -2 → -22` **survived all 3940
  assertions**, and so did `HSB_ERR_ALLOC -11 → -1`, which makes ALLOC collide with
  `HSB_ERR_INVALID_TRANSFORM` — two distinct failures under one code. Every existing assertion
  compares a return against a code *by name*, so both sides move together and the value cancels out.
  6 mutants now installed, 6 killed. Suites **3940 → 3955**.
- **CI** — a `Verify printed version` step: it runs `./build/hisab` and compares the output to
  `VERSION`. **The first gate in either workflow that executes the binary** — every version check
  before it compared one piece of committed text against another. Measured with `VERSION` bumped to
  2.19.0: the old hardcoded file rebuilds and still prints `hisab 2.18.0` (the 2.9.1 → 2.9.2 defect,
  reproduced), the adopted form prints `hisab 2.19.0`.
  ⛔ **Both replacement source-guards were wrong on their first draft, in opposite directions, and only
  running every arm found it.** The "no hardcoded literal" guard **failed on its own tree**, because the
  comment explaining the change quotes the retired `println("hisab 2.18.0")`. The "symbol still used"
  guard was **vacuous** the other way — it matched that same comment's mentions of `CYRIUS_PKG_VERSION`,
  so deleting the call and printing a bare `hisab` still exited 0. Both now strip comment lines; all
  four arms verified (clean 0, literal-back 1, symbol-deleted 1, restored 0). **A gate that cannot tell
  code from a comment quoting code fails on every honest explanation of itself.**
- **CI** — a `Broken provenance markers` gate, running on **push**, not only on pull requests.
  ⛔ **This is the half that matters, and the reason is that the existing job has never run.** The
  `measurements` job is wired `if: github.event_name == 'pull_request'`; the repository has **0 pull
  requests in its entire history** and **303 workflow runs, the last 100 all `event: push`** (checked
  against the GitHub API, not inferred), so `--diff` — including the blindness fixed above — has
  executed exactly **zero** times. The marker this release shipped broken was caught by running the
  script by hand. Broken markers are gated tree-wide because they are wrong regardless of the 284
  unmarked paragraphs of pre-existing debt that keep the full scan from gating on its exit code; the
  step greps, since that exit code cannot distinguish the two failures. Verified to fire on **both**
  shapes — a missing-file path on an existing paragraph, and a two-line malformed marker — and to
  recover.
- **CI** — a `Duplicate global/enum symbols` gate. The conversion's stated safety benefit was real but
  **wired to nothing**: a duplicate `var` global is completely silent (last definition wins, the wrong
  value ships), while a duplicate enum member does warn at file:line — but `cyrius check --with-deps`
  prints that warning and still **exits 0**, and `cyrius lint` never emits it, so CI's `^  warn ` grep
  matched zero times. Verified end to end by adding a real second `HSB_ERR_ALLOC = -3`: consumer-build
  exit 0, lint clean, only the new test block failed. The gate is verified to fire and to recover.

## [2.18.0] - 2026-09-10 — the SVD factors, and the release that began by correcting the last one

2.15.0 filed `‖A − U S Vt‖_F` as **"open-ended: a known defect with no known fix"** and it stood on the
roadmap for three releases. **The fix is one line, and 2.15.0 had created the defect itself.**
Suites **3920 → 3937**.

⛔ **BUT THE FIRST THING THIS RELEASE DID WAS DISCOVER THAT ONE OF 2.17.0's HEADLINE FIGURES WAS
WRONG.** That release reported `svd_golub_kahan` going from "472 of 999 ratios lost, all silent" to
"0", which reads as *the routine now answers correctly there*. It did not: below the floor it returns
`S = (0,0,0,0)`, the block-ratio check computes `0/0 = NaN`, and **every f64 comparison involving NaN
returns 0** — so the relative test silently passed every one of them. The assertion and the probe that
produced the figure shared the same hole, which is exactly why they agreed. **A relative comparison is
not a correctness test until it rejects NaN.** Re-measured with a NaN arm, and then closed:

| | correct | loud `rc != NONE` | silent wrong |
|---|---|---|---|
| 2.16.0 | 256 (to 2^-257) | 269 | **474** |
| 2.17.0 | 268 (to 2^-269) | 731 | 0 |
| **2.18.0** | **999 (to 2^-1000)** | **0** | **0** |

⛔ **THE FACTOR DEFECT WAS A PAIR OF GUARDS THAT DISAGREED.** `_lp_bidiagonalize` **applies** a left
Householder reflector and records it when `vtv >= F64_TINY`, and **replayed it into `U`** only when
`vtv > EPSILON_F64`. Every reflector in between went into `B` and not into `U` — so `S` was exact,
`U` and `Vt` were each perfectly orthogonal, and `U*S*Vt` simply was not `A`: the small block came
back with the **wrong sign**, residual exactly twice the block, for 19 ratios and **288 of 999** on
the 2.16.0 tree. **2.15.0 created that band** by repairing one guard of the pair and leaving its twin
forty lines away. **A guard repaired on one side of a pair is a new defect, not half a repair.**

⭐ **A repair that moves no number is pointing at a second defect.** Repairing the Givens radius in
`_lp_tridiag_qr` changed nothing measurable — 463 of 998 block ratios lost before and after — because
the loss was the Wilkinson shift forming `e2*e2`, which flushes, collapsing `mu` to `d2`; and an
unshifted step on a symmetric block with equal diagonals is a **fixed point**. ⚠ The 2026-09-09 census
had classified that line **SAFE_UNREACHABLE** — correctly about the `f64_sqrt`, wrongly about the
function.

⛔ **AND THE EPA SEED TRADE, CARRIED SINCE 2.9.3, IS DECLINED ON A MEASUREMENT THAT INVERTS ITS OWN
PREMISE.** Its price was never the cost — it is the **accuracy**. Against the exact closed form
`ra + rb − |c1 − c2|`, `gjk_epa_3d` is **1.41e-16 mean / 5.57e-16 worst**; with the strictness
upgrade **1.70e-14 / 2.03e-11**, thirty-six thousand times worse. ⭐ The mechanism was already written
in the source: `_epa_polish`'s own comment says the depth is `min` over probed directions of `h_M(n)`
and every `h_M(n) >= ` the true MTV, so the polish "can only lower an upper bound" — **certifying is
an early-out that skips it**. The filing assumed certifying was the better state; it is the worse one.

### Fixed

- **linalg_precision** — the `U` replay guard now matches the guard that stored the reflector.
  `‖A − U S Vt‖_F / ‖A‖_F`: **288 of 999 ratios past 1e-14 → 0**, worst back to machine epsilon.
- **linalg_precision** — the SVD Wilkinson shift no longer forms degree-FOUR quantities. `tr*tr`,
  `a11*a22` and `a12*a12` put the floor at the **fourth** root of the subnormal floor, 2^-268.5, and
  the measured boundary was 2^-269 exactly. The degree-two closed form plus power-of-two scaling of
  its six operands takes it to **999 of 999**. ⚠ The rearrangement alone was not shippable: it
  reached 512 ratios and opened a five-ratio **silent wrong** band. Trading 5 loud failures for 5
  silent wrong answers is not an improvement.
- **linalg_precision** — `_lp_pow2_floor` returned `F64_ONE` ("do not balance") for every subnormal,
  so the one input that most needs balancing never got it. `eigen_qr` is now exact to **2^-1070**.
- **linalg_precision** — an exactly zero off-diagonal was never deflated: `f64_lt(|off|, eps*(|a|+|b|))`
  is `|off| < 0` once the product flushes, which is false for **every** `|off|` including zero.
  **37 of 72 subnormal block ratios failing → 28.** ⛔ I filed this as "the product flushes, so divide
  instead" and **that causal story was wrong** — the product form WITH the zero arm gives the identical
  28, and the division's mutant survives every fixture. The zero arm is the whole repair; the division
  was reverted rather than shipped as a rewrite with nothing behind it.
- **geo_advanced** — `cga_norm_sq` built a whole 32-slot multivector to read one number out of it (the
  scalar part comes from exactly the 32 **diagonal** blade pairs — verified) and its accumulation order
  destroyed that number: a translator is a unit versor, `1 + t²/4 − t²/4`, and once `t²/4` passes 2^53
  the leading 1 is rounded away. **993 of 2041 scales wrong, first failure at t = 2^28 → 2^538.**
  ⭐ Neumaier, not Kahan: Kahan folds its correction into the next *addend*, where with `sum = 1` and
  addend `2^54` a correction of 1 is below the ulp and is lost.

### Changed

- **geo_advanced** — ⚠ **behaviour change on `cga_norm` for conformal points.** The old accumulation
  returned exactly 0 for 1010 of 1101 positions, but that 0 was **rounding luck**: `cga_point` adds
  `x²/2` to `±0.5`, and for small `x` that term rounds away, so the stored multivector really does
  have `norm_sq = x²`. Exact zeros drop to 53. Judged against the point's own largest coefficient —
  the right scale — nullity barely moves (**1010 → 970** within 1e-14) and the worst relative defect
  is **identical either way, exactly 2^-26 at x = 2^-27**.

### Performance

- ⚠ **A measured cost, stated rather than hidden behind its own spread.** `svd_golub_kahan_12`
  **+6.8%** and `eigen_qr_12` **+6.5%**, consistent across two runs — the mechanism is the shift's
  new max-scan and six divides per sweep. Both sit inside their own historical spread (~41%), so the
  trend filter will not flag them, but the direction is real and it is the price of 256 → 999 correct
  block ratios. ⭐ The control holds: untouched rows moved **+0.30%** median across 72 benchmarks, and
  `triangulate_600gon`'s +15.4% was a single-run outlier (1,544k → 1,782k → 1,595k ns).

## [2.17.0] - 2026-09-10 — the norm tier, and the class that was five times wider than its list

Every `sqrt(x² + y² + …)` in the tree squares before summing, so a component below **2^-537** has a
square below the smallest subnormal and flushes to zero, one above **2^511** overflows, and — the
dangerous third — a **subnormal sum** returns an answer that is finite, plausible and wrong. **39
sites repaired across 16 modules, 73 mutants installed, 67 killed and 6 recorded with their
reasons.** Suites **3807 → 3920**.

⛔ **The roadmap row that scoped this named two sites. The tree-wide grep found 51 `f64_sqrt` sites
in 16 files**, and a 10-agent census with 60 adversarial verifiers (three lenses each — reachability,
dimension, arithmetic) confirmed 19 more defects in 9 modules and refuted one. **Fifth release
running that this class was wider than its own list** — and the row's own instruction to *"grep for
the shape before sizing this"* is what caught it.

⛔ **In a solver the class does not return a wrong number. It returns a CONFIDENT SUCCESS AT THE
STARTING POINT.** Every convergence test in `optimize.cyr` and `linalg_ext.cyr` is `norm < tol`, so a
flushed norm fires on the first iteration and the routine hands back `HSB_ERR_NONE` with `out_x`
still equal to `x0`: **446 of 1001** objective scales for conjugate gradient and **461 of 1001**
residual scales for Levenberg-Marquardt did exactly that. Both are 0 now.

⛔ **Three repairs describe defects that earlier releases record as already fixed.** `cga_rotor`'s
2.14.0 note says *"an absolute 1e-12 turned a real rotation into the identity rotor"* — and the
identity rotor was still returned, 500 binades lower, **991 times in 2041**. `hquat_inverse`'s 2.11.1
note says the same about quaternions, and it still fabricated the identity at **507 scales**.
`geo_triangle_unit_normal`'s 2.10.2 note says the fabricated `(0,1,0)` was repaired; it still came
back for **877 of 1401** leg scales. **Moving a threshold does not help when the quantity it tests
has already lost the value.**

⭐ **A repair that moves no number is pointing at a second defect.** Repairing the Givens radius in
`_lp_tridiag_qr` changed nothing measurable — 463 of 998 block ratios lost before and after. The loss
was the Wilkinson shift forming `e2*e2`, which flushes, collapsing `mu` to `d2`; and an unshifted
step on a symmetric block with equal diagonals is a **fixed point**, so `eigen_qr` burned its whole
budget and returned `HSB_ERR_NO_CONVERGENCE`, losing the *large* block's eigenvalues too. ⚠ The
2026-09-09 census had classified that line **SAFE_UNREACHABLE**, correctly about the `f64_sqrt` and
wrongly about the function.

⛔ **The suite could not see any of this, and the reason is structural.** `hvec3_length`'s own floor
was pinned by two assertions that **asserted the defect as a requirement** — `assert_eq(hvec3_length(2^-538), 0)`
— and the 2.11.1 quaternion sweep bracketed every *threshold* a repair might have chosen while
stopping 142 decades above the *arithmetic* floor. **Bracketing the candidate thresholds is not the
same as reaching the floor.**

⚠ **No performance change is claimed, and the control says so rather than a hedge.** An orphaned
`python3` (PPID 1, stdin a deleted temp file) had held a full core for 12.5 hours and was killed
before the benchmark run, so the whole board moved: **72 of 72 rows, median −6.8%.** The control
separates that from the repairs — **guard-touched rows moved −6.49% median against −6.83% for
untouched, i.e. the UNTOUCHED MOVED MORE** — and every one of the 6 rows past ±10% sits inside its
own historical spread (`vec3_add` −36.0% against a 52.9% spread; `bvh_query_ray_200x4k` −12.4%
against 89.4%). **Check what the machine was doing before believing the delta.**

### Fixed

- **vec2 / vec3 / vec4 / quat** — `hvecN_length` and `hquat_length` rewritten as a fast path plus a
  scaled rescue. The naive sum announces its own failure (subnormal, zero or +Inf), so the direct
  form is kept wherever it is exact; unconditional scaling measured **+119%**, the guarded form
  **+12%**. `_hvec2_len_scaled` delegates to `ganita_f64_hypot`, measured **bit-identical on 6294
  comparisons**.
- **quat** — `hquat_inverse` (**1018 of 2041 binades**, 507 the fabricated identity) and
  `hquat_normalize` (48 of 48 subnormal magnitudes → the identity). Both were **missed by this
  release's own first pass**, and a comment it wrote claiming `hquat_inverse` "routes through" the
  repaired norm was false — it reads `hquat_length_sq` directly.
- **complex** — `cx_abs` (**994 of 2041**), which `cx_ln`, `cx_sqrt` and `cx_powf` are all built on,
  so one repair fixed four public functions. `cx_powf` now guards the modulus, not its square;
  `cx_is_zero` stopped squaring **both** the value and the caller's tolerance.
- **mat3** — `m3_frobenius` (**992 of 2041**), guarded by exactly one assertion, at scale 1.
- **geo** — `geo_segment_direction` (**994 of 2041**) and `geo_triangle_unit_normal` (**877 of
  1401**). The residual band is now `hvec3_cross` itself, upstream of the norm, and the comment says
  so rather than claiming the class closed.
- **geo_advanced** — `cga_plane` and `cga_rotor` (**995 of 2041** each) and `cga_norm` (**992 of
  2041**). `cga_norm` rescales by a **power of two**, which is load-bearing: with an arbitrary
  divisor a null point's norm comes back non-zero at every extreme scale.
- **optimize** — `_opt_norm` plus the convergence tests in `opt_conjugate_gradient` and
  `opt_levenberg_marquardt`, via a sum-aware helper so the two sites that already hold the sum do not
  recompute it.
- **linalg_ext** — `_lext_norm` (`solve_gmres` **461 of 1001**), the GMRES Givens radius (**463 of
  1001** on an isolating fixture with `_lext_norm` already repaired), and `cmat_inverse`, which
  called **498 of 1010** perfectly invertible matrices singular. Three lines had to move together:
  the one-line fix takes 489 of 2001 to 488.
- **linalg_precision** — both arms of `_lp_split_zero_diag`, both bidiagonal bulge rotations, the
  tridiagonal rotation, and the symmetric Wilkinson shift. `svd_golub_kahan` lost the small block for
  **472 of 999** ratios, **all of them silent**; `eigen_qr` for **463 of 998**. ⚠ **The
  `svd_golub_kahan` half of this line is corrected in [Unreleased]**: the true figure is 474 silent
  wrong answers going to 0, and the band they occupied is now a loud `HSB_ERR_NO_CONVERGENCE` rather
  than a correct answer. The `eigen_qr` figure is unaffected — it was verified without the NaN hole.
- **calc_ext** — `calc_monotone_cubic`'s Fritsch-Carlson projection overflowed at hypot ≥ 2^512,
  flattening the segment: **488 of 941** secant ratios wrong by 20%, and still monotone, so the
  function's own oracle passed.

### Changed

- **error.cyr** — `_QUAT_F64_TINY` **retired, not renamed**. Both of its uses turned out to be the
  wrong test once the norms below them stopped flushing: DBL_MIN is one binade too strict for a
  reciprocal.
- **tests** — the 2.14.0 `hvec3_length` assertions that pinned the defect are replaced; the 2.11.1
  quaternion sweep, the 2.15.0 CGA sweep (400 → 2041, one-sided → two) and the log/exp round-trip are
  deepened to the arithmetic floor rather than to a chosen decade.

## [2.16.0] - 2026-09-10 — the small-angle series, and the defect the guard was hiding

2.15.0 deferred four maps because **lowering their guard alone would have made them worse**: `se3_exp`
and `se3_log` divide by θ² and θ³, and at θ ≤ 2.2e-162 both `θ*θ` and `1 − cos θ` are exactly 0, so a
DBL_MIN guard turns the coefficient into `0/0 = NaN`. The coefficients had to become computable at
small θ first. That is this release. Suites **3789 → 3807**.

⛔ **And the log maps turned out to have a much larger defect the guard was hiding.** Both recovered
the angle with `acos` — `su2_log` as `acos(w)` with `w = cos(θ/2)`, `so3_log` as `acos((tr−1)/2)`.
Below θ ≈ 1.5e-8 those arguments round to **exactly 1.0**, `acos` returns 0, and the entire rotation
is gone. Not degraded — gone. Measured on the 2.15.0 tree: `su2_log(su2_exp((0,0,t)))` recovers `t`
down to 2^-24 and returns **exactly 0 from 2^-28 down**.

⭐ **The round-trip floor moves from 2^-26 to 2^-537 — 511 decades recovered.**

### Fixed

- **lie / lie_ext** — `su2_log` and `so3_log` take the angle from `atan2` instead of `acos`. The
  information lives in the SINE, which keeps full relative precision near 0 where the cosine keeps
  none: measured bit-exact to 2^-300. It also handles θ > π correctly, which `acos` could not
  represent at all.
- **lie_ext** — `se3_exp`'s `c1 = (1−cos θ)/θ²` needs no series and no θ² at all: the half-angle
  identity `1 − cos t = 2 sin²(t/2)` turns it into `(sin h / h)² / 2`, with no subtraction of nearly
  equal numbers anywhere.
- **lie_ext** — `se3_exp`'s `c2 = (θ − sin θ)/θ³` and `se3_log`'s `c = (1 − (θ/2)/tan(θ/2))/θ²`
  become Taylor series below θ = 0.1. ⚠ What that buys, measured: at θ = 1e-7 the closed `c2` reads
  **0.172** against a true 1/6, and at 1e-8 it is **exactly 0**; `c` reads 0.0777 against a true
  1/12, and 0 by 1e-8.
- ⭐ **`su2_log` and `so3_log` also took their norms as a naive sum of squares**, which flushes to
  zero for components below ~2^-511 — so the `atan2` repair alone would still have lost the rotation,
  485 decades later than `acos` did but just as completely. Both now scale by the largest component
  first, the same technique 2.15.0 gave `cx_div`. **Found only because a mutant would not die.**

### Changed

- **lie_ext** — `_LIE_SERIES_CUT` = 0.1, and the crossover is **measured, not chosen**: below it the
  closed forms cancel catastrophically, above it the truncated series drifts (1.6e-7 relative at
  θ = 1). At 0.1 the two agree to 2e-14, which is the closed form's own cancellation error, so the
  series is the accurate one there. Constant gate 155 → **156**.
- All four guards are now the exact-zero test the rest of the tree uses.

### Notes on process

- **7 mutants, 5 killed, 2 documented equivalences** — and both equivalences exist *because* the
  repairs made each other redundant. `F64_TINY` on `se3_exp`'s guard was a **trap** in 2.15.0
  (NaN over [DBL_MIN, 2.2e-162]); with c1 and c2 rewritten there is no θ² left to underflow, so the
  guard choice stopped mattering. And `se3_log`'s series is unreachable through `su2_log`, whose own
  floor is now 2^-537 where θ² is still non-zero — it is kept for symmetry and for callers who
  assemble an SE(3) directly.
- ⚠ **Two of my own bounds were wrong and measurement corrected both.** I asserted `c1` is exactly
  1/2 below 2^-20; the θ²/24 correction is under the ULP only from 2^-24.4, so decades 20–24 failed
  against *correct* output. And a fixture meant to reach `se3_log`'s coefficient never did —
  `su2_exp`'s own guard returns the identity first, so `su2_log` gives a zero ω and `se3_log` returns
  early.
- ⚠ **The floor is now on the other side and is recorded, not hidden**: `su2_exp` and
  `so3_from_axis_angle` still take their norms as a naive sum of squares, so they collapse the
  rotation at 2^-538 before the log ever sees it. Same class, on the roadmap.
- **No performance change is claimed, and the run cannot support one either way.** The box was under
  load 1.7–2.3 throughout (an unrelated process pegged at 99.9% CPU), and **every row moved up
  ~7.4%, the untouched ones MORE than the touched** — +7.49% median untouched against +5.26%
  touched. That is the control doing its job: there is no regression attributable to this change,
  and no speedup may be read out of it. Fourteen rows sit outside their own historical spread, all
  in the same direction, which is what a busy machine looks like rather than what a code change
  looks like.


## [2.15.0] - 2026-09-10 — the epsilon tier, closed: 73 of the remaining 74

2.14.0 repaired 23 of the 97 confirmed epsilon defects and enumerated the rest. **This release
closes the other 74 — 73 repaired, every one mutation-proven, and 1 deferred with its reason
recorded.** 81 mutants installed, **73 killed**, 8 surviving and documented rather than hidden.
Suites **3657 → 3789**.

⭐ **The census's central claim held up: the class is one guard comparing a DIMENSIONED quantity
against an absolute 1e-12.** But the *repair* is not one rule, and this release is mostly a record
of the distinctions that turned out to matter:

- **An unbounded numerator needs DBL_MIN; a bounded one needs an exact zero.** `2/vtv` and
  `e2sq/denom` overflow for a subnormal denominator, so they take `F64_TINY`. `y/r` and `x/r` are
  bounded by 1 because `r` is the norm of `(y,z)`, so only an exact zero can fail and a DBL_MIN
  floor still fabricates an identity rotation.
- **A convergence test divides nothing**, so representability is the wrong question entirely —
  `solve_pgs`, `eigen_power` and `ode_dopri45_trajectory` take relative forms.
- **Sparsity is structural**, not a tolerance: `csr_from_dense` decided which entries *exist* by
  magnitude, so the same matrix in millimetres and metres became different matrices.
- **A sort comparator with a tolerance is not an order.** `_col_xy_greater`'s epsilon-equality is
  not transitive, so the relation was not a total order and the hull built on it inherits that.
- **Convexity is a sign, not a magnitude** — four `collision_core` tests asked whether twice the
  signed area exceeded 1e-12, producing a plausible *diagnosis* rather than a plausible number.

### Fixed

- **complex** — ⛔ `cx_div` is the defect `complex.cyr` has named in its own comment **since
  2.6.14**, and **no threshold fixes it**: over a 401-decade sweep 1e-12 fails 241 decades, the
  shipped 1e-24 fails 235, DBL_MIN fails 93, and *removing the guard entirely* still fails 89 —
  `br² + bi²` overflows above 1e154 and underflows below 1e-154, so the quantity is destroyed
  before any comparison runs. **Smith's algorithm fails 0.** Judging the repair against 89 rather
  than 0 would have licensed leaving 89 decades broken. `cx_inv` takes the same treatment.
- **linalg_precision** (9) and **linalg_ext** (12) — SVD, eigen, complex QR, GMRES, PGS, power
  iteration, sparse CSR. ⛔ `solve_gmres`'s back-substitution stored the **undivided numerator** as
  a solution component when it skipped the divide; at `hii = 0` the true `y_i` is unbounded, so
  that is not an approximation of anything.
- **geo_advanced** (18) — the CGA products, the BVH slab tests, `time_of_impact`,
  `cga_blade_inverse`, `cga_plane`, `cga_rotor` and EPA's degenerate-normal path.
- **collision_core** (6), **optimize** (4), **num_ext** (3), **mat3**/**mat4** (3), **lie** (5),
  **lie_ext** (2), **transforms** (2), **color** (2), **num**, **f64_util**, **diffgeo**, **ode**.

### Changed

- **error** — `F64_POS_INF` gains a fifth consumer; `_GA_F64_TINY` added as an alias.
- **geo_advanced** — `_CGA_NULL_TOL` = 8 × DBL_EPSILON, a **measured** bound: over 3000
  pseudo-random conformal points per scale the null residual is 2.13e-16 at coordinate scale 1 and
  1.11e-16 from 1e2 to 1e8, i.e. genuinely relative. ⚠ Reusing `EPSILON_F64` there is ~4500× looser
  and discards resolvable geometry — a radius-10 sphere at x = 1e4 has `nsq = 100` exactly, 90×
  above the floor, and would be called null. Constant gate 154 → **155**.

### Deferred

- **`su2_log`**, with **`so3_log`, `se3_exp` and `se3_log`** — ⛔ **a threshold repair there would
  introduce a NEW defect.** These divide by `theta²` and `theta³`, and at `theta <= 2.2e-162` both
  `theta*theta` and `1 - cos(theta)` are exactly 0, so the coefficient becomes `0/0 = NaN`. They
  need a small-angle series. ⭐ Three of the four were **never on the census list** and were found
  by grepping for the shape.

### Notes on process

- ⛔ **Six of my own fixtures were wrong before the code was**, each caught by measuring rather than
  by reasoning: a tridiagonal sub-diagonal offset by one (1500 of 2000 checks failed against
  *correct* code); an optimizer fixture that scaled the **objective**, which is not "the same
  problem in different units" — the gradient scales but the distance to the minimum does not, so at
  2^-60 the step is below the ULP of `x` and it never moves at any iteration budget; hand-written
  singular values wrong in the fourth digit; a sweep starting where the two blocks' singular values
  **interleave**; three "distinct" hull points that were **colinear**; and `cga_plane` fed a fixed
  `d` when `d` scales with the normal.
- ⛔ **A reachability probe returned the right answer by accident.** Replacing
  `_epa_degenerate_normal` with a constant `+z` passed everything, which reads as "unreachable". It
  is reachable — `+z` is simply *correct* for the coplanar-boxes case. The real finding is sharper:
  the existing assertions check only `x = 0`, `y = 0` and unit length, so **a constant would have
  passed them**. The assertions were weak, not the coverage.
- ⭐ **The census warned me off a whole class of fixture and was right.** A uniform units change is
  *vacuous* for `svd_golub_kahan` — it balances via `pow2_floor`, so `c*A` is bit-exactly invariant
  on the defective tree, and all eight mutants survived my first sweep. The discriminating variable
  is the **ratio between sub-blocks**.
- ⚠ **Eight mutants survive and are recorded with their reasons**, not tidied away: three
  tridiagonal guards behind `eigen_qr`'s balancing (where the census also failed to construct a
  reachable case and said so), a BFGS bare-sign test a convex quadratic cannot generate, an L-BFGS
  rate effect, `_col_ring_is_convex`'s conservative-only epsilon, `f64_fmod`'s `f64_eq(y,0)`
  (IEEE makes `-0.0 == 0.0`, so it is a true equivalence — **an earlier draft of that comment
  claimed otherwise and was wrong**), and an EPA edge-selection loop equivalent for planar hulls.
- ⚠ **One finding beyond the guards, recorded not papered over**: `‖A − U S Vt‖_F` is exactly 0 for
  block ratios 1e-2…1e-5, then 4.17e-7 at 1e-6, decaying proportionally to `c`. The singular
  *values* stay correct — it is `U` and `Vt` that stop reconstructing the small block. Nine
  threshold repairs do not close it; 4.17e-7 matches the census's own figure for a half-repaired
  bidiagonalisation. On the roadmap.
- **No performance change is claimed**: guard-touched rows moved **+0.25%** median against
  **−0.52%** untouched, on a quiet box (load 0.30), with **no row outside its own historical
  spread**.


## [2.14.0] - 2026-09-09 — the epsilon release: guards moved onto the quantity that actually fails

The roadmap sized this tier at **"~20 sites"**. A census of every `EPSILON_F64` comparison guard in
`src/` found **136 guards in 25 modules**, of which **97 are confirmed defects across 23 modules** —
independently re-derived by a second agent instructed to refute the first. ⚠ **The estimate came
from a summary rather than from the code**, exactly as 2.11.3 sized the ganita migration at 8 call
sites when it was 536. **23 confirmed sites are repaired here, every one mutation-proven; the other
74 are enumerated with their evidence in [`docs/audit/2026-09-09-epsilon-census.md`](docs/audit/2026-09-09-epsilon-census.md).**

The class is one guard comparing a **dimensioned** quantity against an **absolute** 1e-12 and then
**fabricating a plausible answer**: a ray test returning 0, which this module defines as *miss*; a
closest-point returning a point that really is on the sphere but on the +x axis whatever the caller
asked; barycentric weights of (1/3, 1/3, 1/3), which sum to 1 and lie inside the triangle. None is an
Inf or a NaN a caller could test for.

### Fixed

- **vec3** — `hvec3_angle` guarded `|a|·|b|`, a length *squared*, against 1e-12. Two **exactly
  perpendicular** vectors returned π/2 at lengths 1, 1e-3 and 1e-6 and **0 rad at 1e-18** — and 0 is
  the value meaning *parallel*, so it reported the opposite of the truth. → `F64_TINY`.
- **calc_ext** — five guards on knot spans, key times and grid intervals. `calc_bspline` and
  `calc_nurbs` returned the **last control point**; `calc_hermite_tcb` returned the last key's value
  for a query on key 1 of 3; `calc_monotone_cubic` returned 0.0, which is simultaneously a legal
  interpolated value and its documented error return. ⚠ Two are **not** threshold swaps:
  `calc_hermite_tcb`'s end-key clamp protects `h10·m0` from `0·Inf`, not a division, so it becomes
  exact equality (deletion is mutation-refuted twice); and `calc_monotone_cubic`'s comment said "not
  sorted" while the code tested "smaller than 1e-12" — it becomes `f64_gt(hi, 0) == 0`, a **not-greater**
  test, because `f64_lt(NaN, x)` is 0 and the old shape let a NaN abscissa through.
- **geo** — six public entry points: `geo_ray_plane`, `geo_ray_sphere`, `geo_ray_triangle`,
  `geo_ray_capsule`, `geo_closest_point_on_sphere`, `geo_barycentric_coords`.
- **geo** — ⛔ **and four guards 2.10.2 already "repaired", where it fixed the threshold and left the
  exponent wrong.** `geo_triangle_unit_normal` guards `len_sq`, a cross product **squared** and so
  degree *four* in the caller's units, against `_GEO_F64_TINY`, a threshold for a degree-*one*
  quantity — it still returned a fabricated (0,1,0) for a well-formed right triangle from legs of
  1e-77 down. **2.10.2 proved that repair with a 10-decade sweep that bottomed out at 1e-9**, so the
  sweep could only ever show the old threshold was wrong, never that the new one was right.
- **geo_diff** — all six ray/surface jets guarded `den` against 1e-12 while the thing that fails is
  `1/den` on the very next line, reporting "grazing, non-differentiable" for ordinary geometry in
  small units. ⭐ **`geo_jet_plane` was not on the census list**; it was found by grepping for the
  shape, and it mattered most, because its primal `geo_ray_plane` is repaired in this same release —
  leaving it would have left the jet rejecting rays its own primal accepts.
- **vec2 / vec3 / vec4** — `hvecN_normalize` returned the **zero vector** for any input shorter than
  1e-12: not an error value, a wrong answer with no length. ⭐ `hvec3_normalize` was not on the census
  list either, and it is the one feeding `geo_ray_new`, EPA and every unit-direction path in the tree.
- **collision (via `hvec3_normalize`)** — ⛔ **a test was pinning the fabrication.** `gjk_epa_3d` on
  an exactly tangent sphere/box asserted a contact normal of **−x**; repairing normalize flipped it to
  **+x**. +x is right, settled by **continuity**: the same pair *overlapping* (box centre 1.5 and 1.75,
  which never reach EPA's degenerate path) returns +x with depths 0.5 and 0.25, and tangency is the
  limit of that family. The old sign was produced by the zeroing, not by the geometry. The continuity
  itself is now asserted so the sign cannot drift back silently.

### Changed

- **error** — `F64_POS_INF` extracted (**fifth** instance): `_COL_`/`_SP_`/`_GA_`/`_GEO_F64_POS_INF`
  are now aliases. ⚠ `interval.cyr` keeps its own literal deliberately — it is one of the four
  standalone modules and does not include `error`. Constant gate 159 → 154 literals.
- **error** — `F64_TINY` extracted (third instance); `_GEO_`/`_AD_`/`_QUAT_F64_TINY` are aliases.
- ⭐ **Where the operation is a RECIPROCAL, the guard is now the reciprocal itself** —
  `f64_lt(|1/x|, F64_POS_INF)` is 1 for every finite reciprocal and 0 for both +Inf and NaN, so one
  comparison rejects x = 0, any x too small to invert, and a NaN, with **no scale chosen for it**.
  ⚠ `F64_TINY` is *not* equivalent there and is one binade too strict: `1/2^-1023 = 2^1023` is finite.
  ⚠ And it is **wrong for `1/sqrt(x)`**, where `f64_gt(x, 0) == 0` is right, because sqrt of any
  non-zero double is normal. Three different operations, three different exact guards.

### Added

- **tests** — 83 assertions, all scale-covariance or boundary. Suites **3574 → 3657**.
- ⭐ **Sweep depths are now DERIVED, not chosen.** `hvec3_angle` runs **154 decades** because that is
  where `denom` first goes subnormal; `_sc_sweep` went **10 → 80** because at decade 80 the quartic
  `len_sq` goes subnormal and precision dies regardless of any threshold; the normalize sweep runs 530.
  ⚠ A 20-decade version of the angle sweep let a **1e-100** threshold survive, and decade-stepping
  left a **2.65-decade blind spot** just above DBL_MIN, so there is a sub-decade probe and a `2^-511`
  fixture that squares to *exactly* DBL_MIN, pinning the threshold to the last ULP.

### Notes on process

- **58 mutants installed, all killed**, restore verified byte-identical after every cycle.
- ⛔ **My own mutation harness was wrong before the code was**: it read the last line of output, which
  for five mutants was a compiler *note*, and printed "killed" for all eleven. With a correct
  extractor **four had survived** — including deletion of a guard, and three repairs the sweep could
  not discriminate at all. **Check the probe before believing the probe**, for the fourth release running.
- ⚠ **A `<` guard that also catches NaN broke a documented contract.** `tests/abuse.tcyr` pins
  `hvec3_normalize` as **NaN in, NaN out**; the natural finite-reciprocal test is false for NaN too and
  silently zeroed it. The suite caught it. The guard tests for **overflow only**.
- ⚠ **One mutant survives on purpose and the reason is asserted, not asserted-away.** `F64_TINY` in
  the normalize family is an **equivalent mutant**: `hvecN_length` squares before the sqrt, so the
  smallest non-zero length it can produce is `2^-537` — ~146 decades *above* DBL_MIN — and the two
  guards coincide on the reachable domain. That floor is now pinned by two assertions, so if the
  function is ever rewritten in a hypot style they stop being interchangeable and the tests say so.
- ⚠ **The census's own first run lost 30 of 41 agents to a session limit** and returned 11 results
  that read exactly like a complete answer. The resumed run completed 121 agents with 0 errors.


## [2.13.0] - 2026-09-09 — the suite release: the suite could not see an error of 0.9

The audit's self-declared **highest-value item**, and the number that justifies it was measured, not
argued: on the 2.12.0 tree, `hvec2_add` could return `a + b + 0.9` — a core public function, wrong by
nearly a whole unit — and **all 3572 assertions still passed. Zero failures across all five suites.**

`assert_eq(f64_to(v), 3)` accepts every `v` in **[3, 4)**, because `f64_to` truncates. **842 of 3105
assertion sites (27.1%)** compared floats that way. They now number **16 (0.5%)**, and every survivor
is a deliberate rounding, scaling or truncation test.

| suite | before | after |
|---|---:|---:|
| `foundation.tcyr` | **89.0%** | **0.0%** |
| `edge_cases.tcyr` | 49.5% | 0.9% |
| `hisab.tcyr` | 32.9% | 0.5% |
| `modules.tcyr` | 19.3% | 0.6% |
| `abuse.tcyr` | 11.1% | 0.4% |
| **total** | **842 (27.1%)** | **16 (0.5%)** |

### Changed — 827 assertions converted, and the conversion verified itself

`assert_eq(f64_to(X), N)` became `assert_eq(X, f64_from(N))` — a bit-exact compare. ⭐ **The migration
is self-verifying: any site whose value was not already exact FAILS on conversion**, which is what
turns a mechanical rewrite into a search. 809 converted in the first pass and **all but four passed
immediately** — every one of those values had been bit-exact all along, and the truncation was pure
slack.

A second pass took the 18 sites that spelled a tolerance as *scaled* truncation —
`f64_to(f64_mul(x, f64_from(1000))) == 90`, i.e. a ±0.001 window — onto named tolerance helpers.

### Fixed — four assertions that were wrong, and had been passing anyway

Each failure the conversion produced was a different kind of defect:

- ⛔ **`srgb_to_linear(-1)` was asserted to be `0`. It is `-0.07739938080495357`.** The claim the
  assertion was *making* — that −1 stays in the linear segment rather than reaching a negative `pow`
  — is correct, and that value is exactly **−1/12.92**. It had asserted the wrong number for the life
  of the file, and truncation absorbed it. Now pinned to the value the branch computes.
- ⛔ **`sh_evaluate_l2` with a zero direction was asserted to be `0`. It is `-0.033296773478641906`
  — and its message was wrong too.** It said "sums to the band-0 term only"; measured, a zero
  direction leaves **two** bands nonzero: `Y00 = 0.2820947917738781` (= 1/(2√π)) and
  `Y20 = -0.31539156525252` (= 0.315392·(3z²−1) at z = 0). The maths is right; the claim about it was
  not. Now asserted as the **property** — exactly two nonzero bands, and their sum — so it fails if a
  third band ever leaks in.
- ⛔ **The PGS solver was checked against `0.090` and `0.636` while the line above it and its own
  messages both say the answer is `1/11` and `7/11`** — 0.090909… and 0.636363…. The assertion sat a
  tenth of a percent from the value it claimed to test, and a ±0.001 truncation window absorbed the
  gap. Now compared against the exact rationals.
- ⚠ **An antiparallel cross product returns NEGATIVE zero**, `0x8000000000000000`. `f64_to` mapped
  ±0 to the same integer and hid the distinction. −0.0 is the correct IEEE result and compares equal
  to +0 under `f64_eq`, so this one is a notation fix rather than a defect — asserted through
  `f64_eq`, which is what *"is zero"* means for a signed zero.
- ⭐ **One truncation was deliberate** — a `ColContact` assertion whose own message said so. Kept, and
  strengthened: the stored value is now pinned bit-exactly at 0.5 **and** its truncation to 0 is
  asserted. Before, only the second half was checked, so the first was free to drift.

### Added — a named tolerance helper in all five suites

Three had none. ⚠ `abuse.tcyr` had no named tolerance **constant** at all: its only approximate
compare spelled the bound as a raw hex literal at the call site, which is precisely how a tolerance
drifts unnoticed between sites. It now has `_AB_TOL` + `_ab_f64_eq`; `hisab` and `modules` gained
`assert_f64_eq` over their existing `LOOSE_TOL_*`. Each carries the measurement above, so the next
person to reach for a tolerance sees why the file distinguishes *exact* from *approximate*.

### Measured — what the suite can now see

Every core operation perturbed by the same amount, before and after. ⚠ **Two of these numbers were
nearly reported wrong**: the first harness's mutation silently failed to apply and the run came back
`0 failing`, which reads exactly like *"the suite is blind"*. The harness now refuses to report a
number unless it can `grep` a marker proving the mutant is installed.

| mutation | before | after |
|---|---:|---:|
| `hvec2_add` **+0.9** | **0** | 4 |
| `hvec2_add` +1e-6 | **0** | 4 |
| `m4_get` +1e-6 | 29 | **84** |
| `hvec3_cross.x` +1e-6 | 76 | **88** |
| `hvec3_dot` +1e-6 | — | 34 |
| `hquat_dot` +1e-6 | — | 15 |

⚠ **One ulp is still invisible on `hvec2_add`** (4 assertions exercise it, none tighter than an exact
compare of a value an ulp of error moves off). That is a *coverage* limit, not a truncation one, and
it is stated rather than left for someone to discover.

## [2.12.0] - 2026-09-09 — the safety release: eight entry points that ended, or corrupted, the caller

The first release off the rewritten release train. **Eight public entry points could be made to end
the caller's process — and one of them did something worse.** Every repair is reproduced on HEAD with
a discriminating control and mutation-proven. Suite **3538 → 3572**.

⚠ **The tier said "3 remaining" allocation sites and named three of the four abort sites. It is
eight**, and the last two were found by refusing to trust the list: one by a **mutant that did not
die**, one by grepping for the shape after the first was repaired.

### Fixed — the abort tier: a library must not end its caller's process

The stdlib's `vec_get` is bounds-checked and calls `_vec_die()` → `syscall(60, 1)`. Three entry
points indexed a vec with the **caller's `n`** without ever comparing it to `vec_len`, so passing an
`n` larger than the data did not return an error — it **exited the process**, printing
`vec: index out of bounds`.

| entry point | reproduced | control |
|---|---|---|
| `convex_hull_2d` (`collision_core.cyr`) | 3 points, `n = 100` → **exit 1** | same call, `n = 3` → exit 0 |
| `triangulate_polygon` (`collision_core.cyr`) | 3 points, `n = 100` → **exit 1** | same call, `n = 3` → exit 0 |
| `sequential_impulse` (`collision_core.cyr`) | see below | — |
| `halfedge_from_triangles` (`collision_mesh.cyr`) | `n_verts = VEC_CAP_MAX + 1` → **exit 1**, `vec: capacity overflow` | `n_verts = 3` → exit 0 |

⛔ **`sequential_impulse` did not abort — it returned SUCCESS after writing out of bounds.** Its
zeroing loop runs `for zi < n` at 16 bytes per contact **before the first `vec_get`**, so with
`iterations = 0` the vec is never touched at all. Reproduced: an **empty** contacts vec, `n = 1000`,
a 64-byte `out_impulses` and a 64-byte canary → **all 8 canary words clobbered, rc = 0, no
diagnostic**. ~16 KB past a 64-byte buffer with a clean exit. CWE-787, silent.

⚠ **The abuse suite already had a canary block for this exact function and could never have caught
it.** It tries `n` = 4, 0 and −2 against a 4-contact vec and a 4-contact buffer — **every one of them
in bounds**. A canary only proves what its `n` reaches.

⚠ **The zeroing loop is the one that had to be bounded, not the solver loop.** Mutation-proven both
ways: removing the guard restores the silent clobber (8/8 canary words), and the *tempting half-fix*
— guarding only the `vec_get` loop — leaves the silent write fully intact.

`halfedge_from_triangles` is bounded by **`VEC_CAP_MAX` itself**, read from the stdlib rather than
transcribed, so it moves when the stdlib moves.

### Fixed — the allocation tier: four sites, not two

`alloc` returns its designed **0** for `size <= 0` and `size > ALLOC_MAX` (2 GiB, re-derived from
`lib/alloc.cyr`). Four entry points stored through that 0.

| entry point | reproduced | note |
|---|---|---|
| `detect_islands` (`collision_mesh.cyr`) | `n_bodies = 268,435,457` → **139** | ⚠ **two different doors**: past the cap, *and* `n_bodies * 8` **overflowing to negative**, which `alloc` rejects via `size <= 0` — the same designed 0 by another route |
| `solve_gmres` — Hessenberg (`linalg_ext.cyr`) | `n = restart = 16385` → **139** | quadratic in `m`; the bound is exact — m = 16383 accepted (2,147,352,576 B fits), m = 16384 rejected |
| `solve_gmres` — `_lext_copy` | `n = 268,435,457` → **139** | ⭐ **found by a mutant that did not die** |
| `solve_bicgstab` — `_lext_copy` | `n = ALLOC_MAX/8 + 1` → **139** | ⭐ **found by grepping for the shape** after the above |

⭐ **The seventh site was found because a mutation test failed to kill.** The Hessenberg guard was
justified on `(m + 1) * m * 8` overflowing; swapping it for the overflowing multiply form left the
suite **green**. That said the overflow was unreachable — and it is, because `solve_gmres`'s *first
line* is `_lext_copy(x0, n)`, an unchecked `alloc(n * 8)` that fails first on any `n` large enough to
overflow it. **The surviving mutant was pointing at a second defect, not at a redundant guard.** The
comment that justified the guard has been corrected to say so.

⭐ **The eighth was found by grepping rather than assuming the list was complete.** `solve_bicgstab`
opens with the identical line. A mechanical sweep for the whole class — an `alloc` sized by a caller
parameter with no `== 0` check — finds **90 sites tree-wide, of which 9 are public entry points**.
Seven of those nine are still open and are recorded on the roadmap rather than folded in silently.

⚠ **Every bound is derived, never chosen**: `VEC_CAP_MAX`, `ALLOC_MAX / 8`, and `(ALLOC_MAX / 8) / m`
written as a **division** so the guard is not computable only when it is unnecessary. Rejection uses
each function's **existing** channel — an empty vec where `tests/abuse.tcyr` reads `vec_len`, a
designed 0 where it reads `assert_neq(..., 0)`, `HSB_ERR_INVALID_INPUT` where there is an rc.
⛔ `solve_gmres` returning `x` — the untouched initial guess — was the tempting alternative and is
exactly the fabricated-plausible-answer class the 2026-08-11 audit filed against that very function.

### Fixed — my own `sequential_impulse` repair, which introduced a regression

⛔ **The first version of that guard turned a harmless no-op into a SIGSEGV**, and an adversarial
re-check caught it before release. It read:

```
if (contacts == 0) { return HSB_ERR_INVALID_INPUT; }
if (n > vec_len(contacts)) { return HSB_ERR_INVALID_INPUT; }
```

`vec_len(v)` is `load64(v + 8)` with **no null or validity check**. On the unguarded function
`contacts` was dereferenced only inside the solver loop, so an `n <= 0` call never touched it — null,
stale or wild, harmless. Reading `vec_len` before establishing `n > 0` moves that dereference onto a
path that never had one, and **a non-null bad handle is not null**, so the null check does not cover
it. Measured: `sequential_impulse(0x1000, 0, 0, out)` — the natural *"no contacts this frame"* call
with a stale handle — returned 0 untouched before and **SIGSEGV'd** after.

Corrected by one line, ordered first: `if (n <= 0) { return HSB_ERR_NONE; }`. Both loops are already
no-ops there, so the early return is bit-identical in memory effect and rc for **every** handle
value — which is what makes the change a pure addition. The incumbent `n <= 0` → success was never
documented; it is now in the function's `Returns:` block.

⚠ **The first version of the TEST was weak too, and the same re-check proved it**: a `>` → `!=`
mutant survived all 3560 assertions, because the pre-existing canary lines call `n = 0` and `n = -2`
but read only the **canary**, never the rc — and the rc is the half that discriminates. Six
assertions added. All four mutants now die: `!=` (rejects a legal partial solve), `>=` (rejects the
exact-fit boundary), dropping the `n <= 0` line (**SIGSEGVs the suite**), and dropping the bound.

### Fixed — `opt_conjugate_gradient` and `opt_gradient_descent`: `n * 8` wraps positive

⛔ **A wrap that defeats a null check.** For `n >= 2^61+1` the product `n * 8` is a small **positive**
number — n = 2^61+1 gives exactly **8** — so `alloc(8)` SUCCEEDS, every handle is non-zero, the
existing `alloc == 0` check passes, and `_opt_copy` then walks 2^61 slots through an 8-byte buffer.
Measured **exit 139 even with 1 GiB caller buffers**, so the fault is the library's own allocation,
not a short `x0`. ⚠ This is the **opposite sign** of the wrap already asserted for
`opt_levenberg_marquardt`, whose product wraps *non-positive* and is caught by `alloc`'s `size <= 0`
gate — the harness had reasoned about the wrap only in its negative form.

The guard is a **round-trip test**, `if (n != (n * 8) / 8)`, not a dimension cap:

⛔ **A `n > _OPT_MAX_DIM` cap was measured and REJECTED — it removes a working capability.** That
cap's own comment scopes it to solvers allocating an `n × n` matrix; these are O(n) solvers, and a
**conforming caller at n = 10,000,000 returns normally today** and would start being refused. The
round-trip tests the multiply the function actually performs: no constant, nothing to go stale when
`ALLOC_MAX` moves — which is the failure this very file already records (an `ALLOC_MAX`-derived 5792
that had to become 16384 when 6.4.51 moved it). The discriminating assertion is `n = 1e9`, which does
**not** wrap and must still return `HSB_ERR_ALLOC`; the rejected cap fails that line.

### Notes — the alloc-tier sweep: 6 of 7 candidates were false positives

The mechanical grep flagged 9 public entry points. Each of the 7 unrepaired ones was handed to an
agent told that **a false positive is a valuable result and must not be manufactured**. Six came back
already guarded, each with the cap quoted at `file:line`:

| entry point | existing cap |
|---|---|
| `tensor_new` | `rank < 0 \|\| rank > 8` — first statement, before either alloc |
| `geodesic_state_new` | `_DG_MAX_DIM = 16`, first statement |
| `geodesic_rk4` | same cap, ahead of all fourteen `alloc(dim * 8)` |
| `parallel_transport` | same cap, two lines above the alloc |
| `opt_bfgs` | three layers, all ahead of the first alloc |
| `opt_lbfgs` | a four-line block bounding both `n` and `m` |

⭐ **So the roadmap's "triage before repairing — several are probably false positives" warning was
right, and following it saved six unnecessary guards** behind caps up to 8.4 million times tighter.
Dead code behind an existing stricter check is not safety; it is the next reader's confusion.

### Added

**25 assertions** in `tests/abuse.tcyr`, every group mutation-proven — including the half-fix mutant
for `sequential_impulse` and the overflowing-multiply mutant for `solve_gmres`. Boundaries are
asserted on **both** sides where one exists, because an off-by-one there is the difference between
rejecting a valid solve and accepting a SIGSEGV.

⚠ **One assertion is deliberately absent and says so in place.** `detect_islands` at exactly
`ALLOC_MAX / 8` *is* accepted — verified once, out of tree — but with 0 contacts it correctly builds
**268,435,456 single-body islands**, which is minutes and gigabytes. (My first version of that
assertion expected 0 and failed with `got 268435456`: the assertion was wrong, not the code.)

### Performance

**The guards are free: guard-touched benchmarks moved +0.08% against a 0.77% noise floor.**

⚠ **That number took two attempts, and the first one was worthless.** The initial run reported a
median **+4.43%** across all 72 benchmarks with 69 of 72 moving positive — which no guard could
cause. The control said so immediately: guard-**touched** rows +3.92%, guard-**untouched** rows
**+4.45%**, i.e. the untouched rows moved *more*. Three agent workflows were saturating the box
(load average 1.97).

Re-run on a quiet machine (load 0.33), two runs before and two after:

| | loaded box | quiet box |
|---|---:|---:|
| median move, all 72 | +4.43% | **−0.61%** |
| same-binary noise (median) | 2–9% | **0.77%** |
| guard-touched rows | +3.92% | **+0.08%** |
| guard-untouched rows | +4.45% | −0.67% |

⭐ **The lesson is the instrument, not the result.** A `vec_len` compare and a division are free, as
expected — but the first measurement could not have shown that, and the only thing that distinguished
"my change is slow" from "the box is busy" was carrying a control group through both runs. **Check
what the machine was doing before believing the delta.**

No individual row is claimed: the five that moved beyond 3× their own noise are all *faster*, all
sub-nanosecond-quantised vector rows the guards never touch (`ease_in_out` 7→6 ns, `vec3_add` 17→16).

### Changed — roadmap verified against the tree; 18 of 39 items did not survive it

No source change. Every open item in `docs/development/roadmap.md` was handed to an independent
verifier instructed to **try to prove it already done**: **21 genuinely open, 15 stale premise, 3
finished**. Stale rows are struck through **with the proof that killed them** rather than deleted —
several were false in a way that would have sent someone to do the *wrong work*. Evidence base:
[`docs/audit/2026-09-09-roadmap-verification.md`](docs/audit/2026-09-09-roadmap-verification.md).

⛔ **The gate blocking the entire release train could not be executed.** 2.12.0 was gated on "re-run
the 28 unverified audit findings first — this gates every item below it". **The 28 were never
enumerated.** The audit report records them only as a count; its only per-finding tables are
explicitly the *confirmed* and *fixed* sets, and no ledger exists in `docs/`, in git history, or in
scratch. ⭐ **And it bound nothing even in principle** — the tiers it blocked draw from the
*confirmed* set, which is **disjoint** from the 28. Retired.

**The release train is rewritten around a safety release**, and the ordering is forced by a
measurement rather than a preference:

| Was | Now |
|---|---|
| 2.12.0 = epsilon tier, gated on the 28 | **2.12.0 = safety release** (abort + allocation tiers), gated on nothing |
| — | **2.13.0 = suite tier** (the `f64_to` truncation) |
| — | **2.14.0 = epsilon tier**, gated on 2.13.0 |

Every epsilon repair is a change to a threshold **below 1.0**, and **843 of 3247 assertion sites
compare through `f64_to`, which truncates** — `assert_eq(f64_to(1.9999), f64_to(1.0))` **passes**.
Landing sub-1.0 behaviour changes into a suite that structurally cannot see them is the wrong order.
The abort and alloc tiers do not have that problem: their failure mode is a signal, not a digit.

### Fixed — 41 defects in the corrections themselves, found by auditing the edit

The roadmap corrections were handed to four independent adversarial lenses (missed-stale, broken
references, internal contradictions, and **over-reach** — did a strike delete real work?). **41
findings survived refutation, 9 of them high**, and most were mine.

⛔ **Three of my own edits rendered as CODE BLOCKS, hiding what they said.** A continuation paragraph
indented 6 spaces after a blank line inside a list item is an indented code block in GFM — lazy
continuation (no blank line) is fine, a new paragraph is not. **83 lines were swallowed**, including
the `sequential_impulse` CWE-787 finding in the file's self-declared highest-priority item, and the
entire six-pairs table in the public/private item. All dedented; a detector now confirms 0 remain.

⛔ **Unescaped `|` used as absolute-value notation split table rows.** GFM parses table pipes *before*
inline code spans, so `` `|d_k|` `` inside backticks still splits the row and every cell past it is
dropped. Fixed across `roadmap.md` and `doc-health.md`; all tables now have consistent cell counts.

⛔ **I claimed the audit backlog was "reordered" and left it in the old order.** The list still read
epsilon-first while the release train I had just written says epsilon is *last*. Moved.

⛔ **A strike silently deleted a tracked gap.** My ellipsis in the "what this audit did NOT reach"
quote elided `the SIMD f64v_* paths`, which is still real — the 2026-08-03 sweep reached only
`hvec3_*`, never the 27 `f64v_*` call sites. Restored. **Deleting a clause while striking a list is
how a gap stops being tracked**, which is the failure the strike was correcting.

⛔ **"18 of 33" was arithmetic I got wrong and propagated to five files.** 21 open + 15 stale + 3 done
= **39**.

⚠ **Two corrections were themselves wrong.** I wrote that the escape-hatch decision was "nothing to
decide" — false: privacy is per *file*, so each of the six `X`→`X_ext` pairs still forces a choice
between promoting the helper, merging the files, or leaving `X` public. And I misdiagnosed the two
broken provenance markers as citing moved paths; the real cause is that `check-measurements.sh`
resolves from the repo root while hisab writes markers relative to `docs/development/` — **my stated
fix would have left both broken.**

⚠ **A citation I "corrected" was still wrong.** The `vec_sort_by` remediation pointed at
`CHANGELOG.md:4111`; my replacement said `:4175`; the actual line is **`:4275`**. Verified by grep
this time. A remediation instruction whose citations have drifted is worse than none.

Also corrected: three EPA benchmarks → **four** (the refuter measured the omitted `gjk_epa_3d_cyl_box`
and confirmed the conclusion holds 4 of 4); a keep/delete contradiction between the standing lesson
and its own Open item; a duplicated backlog row carrying **opposite dispositions**; `defer`'s "8 uses,
all file descriptors" → 6 fds and 2 unlinks; and the constant-gate (156→159), lock (30→31), bundle,
vendored-file and current-pin counts across `CLAUDE.md`, `dependency-watch.md` and `doc-health.md`.

⭐ **README and CLAUDE.md still listed the six Rust repos as consumers** — the exact error the roadmap
sweep had flagged and that neither file had been updated for. Both now name the ten SHA-locked
consumers, with the Rust repos marked as awaiting a port.

### Fixed — documentation claims that were false

- ⛔ **`sequential_impulse` writes past its output buffer and returns success.** Its zeroing loop runs
  `for zi < n` at 16 bytes per contact **before the first `vec_get`**, with no `vec_len` comparison.
  Reproduced: an **empty** contacts vec, `n = 1000`, a 64-byte `out_impulses` and a 64-byte canary →
  **all 8 canary words clobbered, rc = 0, no diagnostic**. ~16 KB past a 64-byte buffer with a clean
  exit. ⚠ **The abuse suite has a canary block for this exact function** and misses it — its three
  `n` values (4, 0, −2) are all in bounds. Scheduled 2.12.0; the roadmap had never named the three
  `collision_core` sites at all.
- ⛔ **The struct-layout contract cannot fail.** "32 assertions across 16 public structs make any such
  change trip a gate" — all 32 are `sizeof(T) > 0` or `sizeof(T) % 8 == 0`, and `ColContact` going
  64 → 72 bytes passes both *identically on both sides of the change they were written to catch*.
  There is no `assert_eq(sizeof(T), <n>)` anywhere, and six public structs carry none — including
  **`HVec3`**, the type live consumers touch most. Quoted as protection for five releases.
- ⛔ **"None are live yet — all four come online later" is wrong in both directions.** **Ten repos
  consume `dist/hisab.cyr` today**, SHA-locked (svara pins tag 2.11.2 and calls `num_fft`), while
  impetus/kiran/joshua/hisab-mimamsa/kana have **no `cyrius.cyml` on any branch** — they are Rust
  repos needing a port. ⚠ **No live consumer has built 2.11.3+**, i.e. none has crossed the
  6.5.33 → 6.6.2 bump or the 536-site ganita migration.
- ⭐ **Four v3.0.0 "decisions" were already answered by the toolchain.** cycc 6.6.2 ships and
  **enforces** file-level `private` + per-fn `pub`. The bundle question was answered *wrong* here: a
  real `distlib` bundle from `private` modules exported only its `pub fn`s. ⚠ Landmine recorded:
  `check --with-deps` stays **green** on a bundle no consumer can call.
- **The epsilon tier is mis-sized**: "~20 sites" came from a review; the mechanical grep the item
  itself prescribes returns **123 guards across 24 of 35 modules**, untriaged. The allocation tier is
  **2, not 3** (the third is owned by the abort tier and needs a different repair).
- **EPA sphere-family non-convergence is not a defect.** Holding the seed and varying only curvature:
  smooth sphere **12** handoffs → inscribed icosahedron **4** → box **0**. A polytope expansion cannot
  certify a smooth surface at 1e-10 within 64 iterations. Nothing to repair.
- Header drift corrected: the roadmap's own header said toolchain **6.5.33** fourteen lines above a
  line saying **6.6.2**; `deps --verify` 30/30 → **31/31**; bundle figures in `cyrius.cyml` re-derived
  (**905,803 B / 23,405 lines**); archived-filing counts corrected; and the "Cyrius has no build-time
  string interpolation" claim in `scripts/version-bump.sh` and `ci.yml` — **false since 6.5.21** —
  fixed. ⚠ A dangling sentence fragment left in `README.md` by an earlier edit was also repaired.

## [2.11.5] - 2026-09-09 — cycc 6.6.2: the wrong-code bug is fixed, and the filing was wrong about its scope

Toolchain **6.6.1 → 6.6.2**. No hisab source behaviour change; suite **3538/3538**, every gate green,
lock 30 → **31** (the new `lib/boxed.cyr`, pulled transitively by `tagged`). No dependency of hisab's
moved: ganita stays **1.2.4**, sakshi **2.5.1**.

### Fixed — the SIMD destination-slot miscompile filed in 2.11.3 is repaired upstream

The reproducer hisab filed now exits **0** on 6.6.2 and **139** on 6.6.1, same binary. hisab's three
affected suites pass with the consumer-side hoist **removed** (416 / 351 / 741, rc=0 each), so
`m4_mul_vec4`'s hoist is **no longer load-bearing** — it is kept only because `m3_mul_vec3` has always
been written that way, and its comment now says so. ⭐ **A comment claiming a workaround is mandatory,
left standing after it stops being mandatory, costs the next reader the same investigation twice.**

⛔ **hisab's filing was right about the symptom and wrong about the scope, in two ways that would
each have shaped a narrower fix.**

1. **Neither derive-specific nor a 6.5.71 regression** — it was filed as both. All **21**
   `f64v_*`/`f32v_*`/`f32v8_*`/`f64v256_*`/`iv_*` handlers took `var vbase = GFLC(S)` without raising
   `GFLC` until every argument was parsed, so any argument allocating a frame local bound it over the
   intrinsic's own destination. Reachable via `callptr` since **6.0.70** and `#inline` since
   **6.5.63**; 6.5.71 merely removed the real `call` that had been forcing the spill by accident.
   ⚠ **The bisect found the release that EXPOSED the bug and I reported it as the release that
   caused it.** A first-bad-version is evidence about *visibility*, not about *origin* — and the
   filing's own Root-cause section said "older than 6.5.71" while its header contradicted it.
2. **The severity was understated.** The filing called the silent out-of-bounds write the unlucky
   case. Upstream measured both modes deterministically: with the register picker on it faults; with
   `CYRIUS_REGALLOC_PICKER_CAP=0` it **exits 0 and writes the result into the argument object**.

### Fixed — a landmine hisab has documented since 2.3.1, and documented for the wrong reason

Found upstream *while writing the test* for the above: a bare SIMD intrinsic at **top level**
compiled clean and SIGSEGV'd on every release checked, because these handlers stash operands in
**frame** slots and top-level code has no frame. 6.6.2 refuses with
`SIMD intrinsics must be called inside a function`.

⚠ **`src/vec4.cyr` has carried this rule since 2.3.1 with the wrong cause attached** — it blamed a
misaligned stack for SSE. Corrected. Verified from the consumer side, identical source both ways:
6.6.1 builds `OK` then exits **139**; 6.6.2 rejects at compile time.

### Performance

**No change is claimed — 6.6.2 is performance-neutral for hisab**, which is the interesting result
given it rewrote the operand handling of all 21 SIMD intrinsics.

⚠ **Two rows looked like regressions and are not, and the check that settled it is per-benchmark
rather than global.** `vec4_dot_x64` read +22.85% and `vec3_dot_x64` +14.01% against a *global* p90
noise band. Their raw runs say otherwise:

| benchmark | 6.6.1 runs | 6.6.2 runs |
|---|---|---|
| `vec4_dot_x64` | 295, 287 ns | 286, **429** ns |
| `vec3_dot_x64` | 386, 385 ns | 381, **498** ns |

The *first* 6.6.2 run matches 6.6.1 to within 1%; the second spiked. A single outlier in one run, not
a regression — and every other SIMD row (`vec3_add`, `vec3_cross`, `quat_mul`, `m4_mul_x16`) is flat.
⭐ **A global noise band is the wrong instrument when one benchmark is 50x noisier than the median**;
comparing each benchmark's move against *its own* run-to-run spread is what separated these.

`ease_in_out` "6 → 7 ns" is integer-nanosecond quantisation on a 7 ns value, not a measurement.

### Notes

- **hisab is not exposed to 6.6.2's headline `tagged` repair.** 6.6.0 deleted `tagged_new`/`payload`
  and **silently redefined `tag()` and `is_tag()` at unchanged arity** — the one class a consumer
  build cannot catch. Checked rather than assumed: hisab calls **none** of
  `tagged_new`/`tag`/`payload`/`is_tag`/`is_some`/`is_none`/`is_ok`/`is_err`, so it passed through
  6.6.0 and 6.6.1 unaffected. ⭐ **The rule that release establishes is worth carrying here: a name
  whose meaning changed must be RETIRED, not redefined.**
- The enum ≥ 2^62 Critical still does not reach hisab — re-scanned after the bump, **938** enum
  constants across `src/`, `lib/` and `dist/`, none at or above the threshold.
- All 31 vendored files byte-match the **6.6.2 snapshot itself**, not old-pin vs new-pin.
- Nothing else in 6.6.2 touches hisab's surface (`map_u64` sentinels, `object;` libc exports,
  `--allow-undef`, foreign-src builds, lexer token numbering).

## [2.11.4] - 2026-09-09 — off the deprecated ganita aliases, the pow repair, and two of my own instruments caught wrong

ganita 1.2.4 marks the bare `mat_*` and `f64_*` spellings **deprecated aliases, migration window
only**. All hisab call sites now use the `ganita_*` names. Suite **3532/3532**, unchanged; every
gate green; **zero** deprecated call sites left in `dist/hisab.cyr`, so consumers get a clean bundle.

⚠ **2.11.3 estimated this at 8 call sites. It is 536.** That estimate came from grepping only the
transcendentals named in ganita's changelog paragraph; the deprecation block is **53 aliases**, and
hisab was on **20** of them — including `f64_pow` (36 sites) and the entire `mat_*` surface
(`mat_set` 265, `mat_new` 121, `mat_get` 59). **A scope estimate taken from the release notes rather
than from the code was off by 67x.**

### Changed

- **536 call sites across 20 alias names and 17 files migrated to the `ganita_*` spellings.**
  Verified three ways rather than by inspection: the migrator is string-literal aware (it renames
  assertion messages, which name the function under test, but never comment prose); a re-scan finds
  **0 residual** call sites; and every one of the 20 targets was checked to be a **real definition**
  rather than a further forwarding layer (the two one-line ones, `ganita_mat_rows`/`_cols`, are
  genuine `load64` bodies).

  ⭐ **The 5 suites produce BYTE-IDENTICAL output before and after** — 3532 assertions, captured and
  diffed, not merely re-counted. The aliases are literal one-line forwarders, so semantic
  equivalence is a property of the change rather than a hope, and the diff confirms it.

  8 lines crossed the 120-char lint limit under the 7-character rename; 5 were fixed by collapsing
  alignment padding and 3 by splitting, so `lint` stays at 0 warnings.

### Performance

The aliases were **real function calls, not free**: cycc's `#inline` requires **2 or fewer
parameters**, and `ganita_mat_get` takes 3, `ganita_mat_set` takes 4. Every matrix access in
`linalg_precision.cyr`'s SVD and eigen inner loops (37 `mat_get` + 31 `mat_set`) was paying a
`call`/`ret` pair for nothing.

| benchmark | 2.11.3 | 2.11.4 | change |
|---|---:|---:|---|
| `svd_golub_kahan_12` | 158,905 ns | 114,653 ns | **−27.85%** |
| `eigen_qr_12` | 119,704 ns | 86,578 ns | **−27.67%** |

⚠ **Both figures are the mean of two post-migration runs, against a noise floor measured on this
host in this session** — two back-to-back runs of the *identical* binary, spreading by a **median
1.79%**, p90 **5.30%** and a **worst 9.13%** across all 72 benchmarks. −28% is ~3x the worst noise and lands on
exactly the two benchmarks the mechanism predicts.

**No other movement is claimed.** Every other benchmark drifted inside that band, and none of them
contains a matrix call, so there is no mechanism for a change either way.

### Fixed

- **Two assertions had gone vacuous, and neither could ever have failed to tell us.** 2.11.3 caught
  the two tests that *failed* when ganita 1.2.4 fixed what they had pinned. These two compare
  through a **tolerance** and a **round**, so when the defect they were written against was repaired
  they kept passing while the property they claimed to test evaporated:
  - `tests/abuse.tcyr` asserted `dual_pow: (-2)^3 -> -8 exactly` with the message
    *"truncation-discriminating: the stdlib path gives -7"*. The stdlib path now returns
    `0xC020000000000000` — exactly −8, truncating to −8 — so the pair **no longer discriminates
    anything**. False claim removed, the check kept.
  - `tests/modules.tcyr` said *"f64_pow goes through exp/ln, so 2^3 is 7.999… — tolerance, not
    equality"* and used `LOOSE_TOL_M`. `dual_pow(2,3)` is now `0x4020000000000000`, **exactly 8**;
    the tolerance was hiding the improvement *and* admitting any future regression up to
    `LOOSE_TOL_M`. Now asserted as bits.

  ⭐ **An assertion written against a defect cannot fail when the defect is repaired — it just stops
  testing anything.** The failing kind announces itself; this kind does not, and only re-reading the
  rationale finds it.

### Fixed — `_ad_pow` delegated away its own accuracy, and hid a fabricated 1.0 behind it

`_ad_pow` opened with `if (f64_gt(base, 0) == 1) { return ganita_f64_pow(base, n); }` — every
**positive** base went straight to the stdlib and its repeated-multiplication path never ran.

⚠ **That line also made the evidence lie.** Swept over 9 bases, `_ad_pow` tied the stdlib on every
positive base at every exponent, which reads as "this function is redundant". It was the stdlib
being **compared with itself**. Running the loop on positive bases too beats the stdlib on **44 of
48** of them, ties 2, and loses 2 by at most 3 ULP — because ganita 1.2.4 uses binary exponentiation
inside `|n| <= 1024`, and squaring amplifies relative error geometrically where repeated
multiplication accumulates it additively:

| | repeated multiplication | ganita 1.2.4 |
|---|---:|---:|
| `0.9^1024` | **7 ULP** | 137 ULP |
| `1.001^1024` | **10 ULP** | 174 ULP |
| `1.001^512` | **5 ULP** | 104 ULP |
| `(-0.999)^1101` — past the ±1024 window | 3 ULP | **1 ULP** |

The delegation is gone. **One rule replaces it and the bound is ganita's own**, so nothing here needs
a scale chosen for it: an integral exponent with `|n| <= 1024` goes through the product, everything
else defers. Across the 126-row sweep, **60 rows improved, 6 lost <= 3 ULP**, and against the stdlib
`_ad_pow` now wins 93 / ties 26 / loses 7.

⛔ **And widening the loop exposed a fabricated answer that had been live for negative bases all
along.** `f64_to` **SATURATES**: `f64_to(1e300)` is `i64::MIN`, and negating `i64::MIN` in two's
complement gives `i64::MIN` back — so a bound checked *after* the truncation reads false, the loop
`i < k` never runs, `acc` stays 1, and `_ad_pow(-2, 1e300)` returned a plausible **1.0** where the
answer is **+Inf**. `1e300` is integral, so the round check does not stop it. The bound is now tested
in **f64 space, before `f64_to`** — the order is the guard. Same class as the fabricated plausible
answer catalogued in `complex.cyr:58`.

**6 assertions added, both groups mutation-proven**: restoring the `base > 0` delegation fails the
accuracy assertion; moving the bound back after `f64_to` fails all four fabrication assertions with
`got 4607182418800017408` — `0x3FF0000000000000`, the fabricated 1.0 itself. ⚠ **Until now nothing in
3532 assertions exercised the regime this function exists for**, which is why deleting it looked
defensible.

### Fixed — my own benchmark filing was the thing that needed refuting

2.11.3 filed *"41 of 72 benchmarks report a `net` below the timer floor they subtract"* and used it
to suppress most of that release's measured result. **The ratio was the wrong instrument.** It
compares a **per-operation** net against the cost of **one clock pair**; since 6.5.19 `bench_run`
sizes each batch as `want = (fl * 100) / per`, i.e. so the clock's error is **1% of the timed
window**. A 7 ns op runs ~19,000 times inside one window — `net` at 0.01x the floor is that design
working.

Tested on the property that actually matters, two back-to-back runs of the identical binary:

| tier | n | median | p90 | worst |
|---|---:|---:|---:|---:|
| `net` >= 10x floor — *"trustworthy"* | 23 | 2.10% | 5.95% | 9.13% |
| `net` < 10x floor — *"cannot support a claim"* | 49 | **1.43%** | **4.74%** | **7.32%** |

**The tier I dismissed is the quieter one.** The filing is archived REFUTED, and 2.11.3's entry is
corrected in place: `ray_aabb` **−54.8%**, `vec3_cross` **−54.0%**, `ray_triangle` **−44.3%**,
`jet_plane` **−40.5%**, `quat_mul` **−39.7%**, `ray_sphere` **−36.7%** — each 15–25x its own noise.
cycc 6.5.71's accessor inlining sped up the whole vector/quaternion/ray surface, not nine benchmarks.

⭐ **Being conservative is not the same as being correct.** A wrong instrument suppresses real results
as readily as it invents false ones, and this one did exactly that for a whole release. The guard
that needs no threshold: **re-run the identical binary and measure the spread.**

### Notes

- ⚠ **`_ad_pow`'s reason to exist has now collapsed twice, and the comment beside it said so about
  the first collapse while being wrong about the second.** Its DOMAIN rationale died in 2.11.2
  (ganita 1.1.4); 2.11.2 rewrote it onto PRECISION; ganita 1.2.4's binary exponentiation killed that
  too — **665 of 665 comparisons now agree bit for bit**, and `(-2)^4` reads 16 through both paths.
  ⭐ **A third ground survives and it is the opposite of the second**: squaring amplifies relative
  error, so binary exponentiation loses accuracy exactly where it wins speed. Against a 60-digit
  reference, `(-0.999)^1000` is **4 ulp** from truth through `_ad_pow` and **58 ulp** through ganita
  — while past ganita's ±1024 window the ranking inverts (3 ulp vs 1 ulp). The comment now states
  that measured position instead of a rationale that stopped being true two releases ago; **it was settled by widening the loop rather than deleting it** — see the repair above. The
  regime it wins is now pinned by an assertion, which nothing in 3532 had covered.
- **Comments were NOT renamed wholesale, and the split is deliberate.** 21 present-tense API
  descriptions (`caller allocates via ganita_mat_new`, `the matvec reads ganita_mat_get(A, i, j)`)
  were corrected so a reader can grep from the comment to the call. The ~45 remaining bare mentions
  are **history** — they describe what a past ganita version did, under the name it had then, and
  renaming them would falsify the record. `src/f64_util.cyr` now states that convention in one place
  so the mixture is legible rather than looking like a missed rename.
- `mat_new_guarded` is **hisab's own** wrapper and is untouched; the migrator anchors on `name(`, so
  it could not be caught by the `mat_new` rename.

## [2.11.3] - 2026-09-09 — the toolchain catch-up that hit a wrong-code regression

cyrius **6.5.33 → 6.6.1** (forty-odd releases, crossing a minor), sakshi 2.4.11 → **2.5.1**, and
vendored with the toolchain **ganita 1.1.4 → 1.2.4**. Suite **3526 → 3532**, all passing; constant
gate 159/159, lint/fmt/vet clean, fuzz clean, lock 30/30, consumer-build gate OK.

**This bump did not land quietly: the new toolchain miscompiles hisab.** Three of five suites
SIGSEGV'd on the first run under 6.6.1, and the cause was a cycc wrong-code regression, not
anything in this repo. The other two findings are the usual shape for a catch-up release — two
tests that had pinned an *upstream defect* as expected behaviour, and a manifest constant that was
wrong about which limit it even described.

### Fixed

- **`m4_mul_vec4` SIGSEGV'd on every call under cycc >= 6.5.71** — a `#derive(accessors)` getter
  passed directly as an `f64v_*` intrinsic argument makes the intrinsic's inline expansion read its
  **destination pointer from a stack slot that nothing ever writes**, then issue a 128-bit packed
  store through it. `movupd %xmm0,(%rdx,%rsi,8)` with `%rdx = 12`, loaded from `-0x50(%rbp)`, a slot
  read once and written zero times in the function — while the `src` and `scalar` slots *are*
  written. Fixed here by hoisting the four accessor reads into locals, which is what the sibling
  `m3_mul_vec3` has always done; mat4 was the lone inconsistency. The hoist is load-bearing and
  carries a comment saying so. **Filed upstream in the cyrius repo**, where the language agent can see it:
  `cyrius/docs/development/issues/2026-09-09-hisab-derive-accessor-simd-dst-slot.md`, with a
  self-validating reproducer beside it in `repros/` (exit 0 on 6.5.70, exit 139 on 6.6.1).

  ⚠ **Bisected to 6.5.71, and the trigger is narrower than "inlining".** 6.5.70 is clean; 6.5.71,
  6.5.72, 6.5.73, 6.6.0 and 6.6.1 are not. Three functions of identical shape in one file
  discriminate it: a plain `fn` accessor compiles correctly, a raw `load64` compiles correctly, and
  only the **derived getter** fails. 6.5.71 is the release that made derive accessors reach the
  inline-replay path (`callq` 7 → 3); before it the accessor was a real call, and the call
  incidentally forced the spill the expansion depends on. **The intrinsic bug is older than 6.5.71 —
  that release only stopped hiding it.**

  ⚠ **It is a wrong-code bug, not merely a crash.** The store goes through whatever the
  uninitialised slot holds; it faults only because that value happens to be unmapped. Mutation-proven
  both ways: reverting the hoist returns all three suites to rc=139, restoring it returns 3532/3532,
  and the restore was confirmed by grep rather than assumed.

### Changed

- **Two tests updated because upstream FIXED the defects they had pinned.** Both suites were
  asserting a dependency's bug as expected behaviour, so the upstream repair is what failed them.
  - `f64_pow(-2,4)` truncated to **15**; it is now bit-exact **16**. ganita 1.2.4 replaced
    `exp(n·ln|base|)` with **binary exponentiation** for integral exponents (|exp| <= 1024), in its
    own words because `pow(7,2)` returned `48.99999999999999296` and `pow(10,15)` was up to 47 ulps
    out. Verified as bits, not through the truncating `f64_to`: `f64_pow(-2,4)` =
    `0x4030000000000000`, `(-2)^3` = `-8.0`, `2^10` = `1024.0`, `(-2)^5` = `-32.0`.
    ⚠ **The rationale that comment carried is now void**: `_ad_pow` no longer wins on precision — the
    two paths agree bit for bit — so the comment now says `_ad_pow` stays because autodiff needs the
    value and the derivative from one call. Four assertions added, including the agreement check.
  - `cx_exp(-inf + 0i)` returned **NaN**; it now returns exactly **+0**, which is the correct answer.
    cyrius 6.6.1 gave `lib/math.cyr`'s exp polyfill its missing infinity guard (`exp(+inf) = +inf`,
    `exp(-inf) = +0`); previously the range reduction computed `inf - inf` and the `2^n` bit-pack read
    a saturated `f64_to(inf)`, so it returned an infinity of the wrong sign and `0 * inf` came out NaN.
    ⚠ **The replacement assertion had to be made discriminating**, because the value it now expects
    (0) is exactly what the fabricated-guard defect class produces — the class recorded in
    `complex.cyr:58`, which `cx_div` is still an instance of two assertions above it. `exp(+inf)` is
    now asserted beside it: a blanket "any infinity → 0" guard would satisfy the `-inf` lines and fail
    the `+inf` line.

- **`cyrius.cyml`'s bundle-weight comment described a limit that does not gate this path.** It read
  "5.4% of cycc 6.5.33's 16 MB `input_buf`" (and before 2.11.2, "76.8% of a 1 MB `input_buf`"). Both
  figures describe `_SRC_CAP`, the raw read buffer — but what actually rejects a consumer's build is
  the **expanded-source** cap, and a **token** cap can bind before either:

  | ceiling         | 6.5.33    | 6.6.1     |
  |-----------------|-----------|-----------|
  | expanded source | 8 MB      | **24 MB** |
  | token count     | 1,048,576 | **4,194,304** |

  Measured as a **pair**, which is the point: a generated 9,002,640 B source is *rejected* by 6.5.33
  with `expanded source exceeds 8MB` and compiles *clean* on 6.6.1. A one-sided "it compiled" — the
  shortcut the retired 16 MB figure rested on — proves only that the cap exceeds that one file.
  ⭐ **And the token cap bit first**: an 8,202,798 B source under 6.5.33 died on `token limit
  exceeded: needed 1048577, cap is 1048576` without ever reaching the byte check, so a bytes-only
  headroom claim is the wrong instrument. Third consecutive release in which one of these numbers
  moved.

### Performance

Geometry and collision workloads got materially faster, and the cause is upstream: 6.5.71 inlines
`#derive(accessors)` getters, which hisab uses on every vector and quaternion component.

| benchmark | 2.11.2 | 2.11.3 | change |
|---|---:|---:|---|
| `bvh_query_ray_200x4k` | 1,769,000 ns | 945,832 ns | **−46.5%** |
| `bvh_degenerate_4k` | 3,304,000 ns | 2,300,000 ns | **−30.4%** |
| `gjk_epa_sphere_box` | 111,531 ns | 78,477 ns | **−29.6%** |
| `gjk_epa_3d_cyl_box` | 109,327 ns | 79,609 ns | **−27.2%** |
| `gjk_epa_spheres` | 536,604 ns | 405,228 ns | **−24.5%** |
| `grad_fwd_16` | 58,180 ns | 43,980 ns | **−24.4%** |
| `bvh_scatter_4k` | 5,806,000 ns | 4,666,000 ns | **−19.6%** |
| `spatial_hash_query_2k` | 428,040 ns | 366,498 ns | −14.4% |
| `delaunay_2d_400` | 1,583,000 ns | 1,388,000 ns | −12.3% |

⛔ **CORRECTED IN PLACE 2026-09-09 (2.11.4): the restriction below was wrong and it suppressed most
of this release's actual result.** The paragraph limited every claim to 23 of 72 benchmarks on the
grounds that the rest had a `net` below the timer floor. That ratio compares a **per-operation** net
against the cost of **one clock pair**, and since 6.5.19 `bench_run` batches so the clock's error is
1% of each window — the two numbers are not comparable. Measured on the property that actually
matters, two back-to-back runs of the identical binary, the dismissed tier is the **quieter** one
(median 1.43% vs 2.10%). The suppressed rows are real: `ray_aabb` **−54.8%**, `vec3_cross`
**−54.0%**, `ray_triangle` **−44.3%**, `jet_plane` **−40.5%**, `quat_mul` **−39.7%**, `ray_sphere`
**−36.7%**, each 15–25x its own measured noise. **cycc 6.5.71's accessor inlining sped up the whole
vector/quaternion/ray surface, not nine benchmarks.** See
[`issues/archived/2026-09-09-bench-net-below-timer-floor.md`](docs/development/issues/archived/2026-09-09-bench-net-below-timer-floor.md).

The original paragraph, left for the record:

⚠ **These are drawn only from the 23 benchmarks whose `net` is at least 10x the subtracted timer
floor.** Same instrument as 2.11.2 (`regime=net`, floor 1343 → 1329 ns), so the comparison is
like-for-like — and the flat rows *inside the trustworthy tier* are the control proving the
instrument did not shift: `svd_golub_kahan_12` +0.1%, `eigen_qr_12` +0.4%, `num_dct_1023` +1.0%,
`num_is_prime` −0.2%. Those are the numeric kernels that work on raw offsets rather than accessors,
which is exactly where an accessor-inlining win should *not* appear. No regression anywhere in the
tier; the worst movement is +1.0%.

**No claim is made from the other 49 rows** — see below.

### Notes

- ⛔ **RETRACTED 2026-09-09 (2.11.4).** This note claimed 41 of 72 benchmarks report a `net` below
  the floor they subtract and therefore measure nothing. **The ratio was the wrong instrument** — a
  per-op net against a per-clock-pair floor — and the filing it raised is now archived REFUTED. The
  dismissed tier is the quieter of the two on a same-binary re-run, and `ray_aabb` / `vec3_cross`
  are real −54% speedups rather than "not evidence of anything".
- ⚠ **The enum Critical the 6.6.1 launcher warns about does not reach hisab, and that was checked
  rather than assumed.** 6.5.33 and earlier mis-read enum constants >= 2^62 as −1; hisab declares
  constants as enums by policy, and every hex f64 bit pattern for a value >= 2.0 exceeds that
  threshold, so the exposure looked plausible. A scan of all **936 enum constants** across `src/`,
  `lib/` and `dist/` found **none** at or above 2^62 — the seven enums in `src/` top out at 128.
- **ganita 1.2.4 marks `f64_acos`, `f64_atan2` and their siblings deprecated** ("migration window
  only") in favour of `ganita_f64_*`. hisab uses `f64_acos` 5x and `f64_atan2` 3x. They still
  resolve, so nothing is changed here; recorded in `dependency-watch.md` as an actionable item
  rather than folded silently into a toolchain release.
- **sakshi 2.4.11 → 2.5.1 changes no public surface** — diffing the exported `fn` list shows only
  private `_sk_*` helpers moving (four dead functions removed, one added). The 6.6.1 stdlib ships a
  `sakshi.cyr` that is **byte-identical** to the 2.5.1 git dep (same SHA256, 76,277 B), so the
  vendored copy and the pinned dep agree rather than shadowing each other.
- **Issue triage: the open queue went 5 → 2, and two upstream bugs were closed by re-testing them.**
  ⚠ **Cyrius bugs now go to the cyrius repo** (`cyrius/docs/development/issues/`), which is where the
  language agent reads them — not into hisab's tree. The derive-accessor miscompile was filed there
  with a self-validating reproducer in `repros/` that exits **0 on 6.5.70** and **139 on 6.6.1**, so
  it proves its own bisect rather than asserting it.
  - `2026-04-26-cyrius-cli-arg-clobbers-source.md` — 🟢 **archived, fixed upstream.** Carried for
    four months as "deliberately never re-tested" because the reproducer destroys a source file. It
    is now guarded: `error: refusing to write build output over a .cyr source file`, exit **1**,
    source byte-identical across three argument shapes, verified in a throwaway package rather than
    in this tree. ⚠ The exit code needed a second look — `cyrius … | tail` reports 0, because `$?`
    after a pipe is *tail's* status. Same probe defect this repo recorded in 2.11.2, hit again.
  - `2026-08-09-cyrius-dead-fn-bodies-are-never-syntax-checked.md` — 🟢 **archived, fixed upstream.**
    The underscore discriminator is gone and **`lint` — the specific half this was left open for —
    now reports the real diagnostic** instead of 0 warnings. ⚠ **The control was run**, because
    "everything fails" and "the bug is fixed" look identical from the exit codes alone: a *clean*
    uncalled no-underscore function still passes `build`/`lint`/`vet`. `vet` still exits 0 on
    unparseable input and that is deliberately **not** claimed as fixed — it is the dependency
    auditor, not a syntax gate.
  - Two orphaned `incircle` reproducer fixtures were sitting in the **open** issues directory after
    both parent filings had been archived, implying live defects. Re-run before moving rather than
    moved on the assumption that an archived parent means a fixed child: both report **0**
    disagreements against the exact reference (of 40 and of 30 cases). ⚠ Their own output is
    misleading — `println(fmt_int(n))` prints the count *and* `fmt_int`'s return, so 0 renders as
    `00`; noted beside them so the next reader does not misread a nonzero count.
  - `2026-08-06-epa-certificate-…` stays open — a genuine design question whose two proposed repairs
    are already measured and rejected. ⚠ **But its cost figure went stale under this bump**: the
    "+19.4%" trade was priced against a `gjk_epa_sphere_box` baseline of 125.6 us, which now reads
    **78.5 us**. Annotated rather than rescaled — both the baseline and the added work moved for the
    same reason (accessor inlining) and neither was measured after the move, so the honest answer is
    that the percentage is currently **unknown**.
- **CI gains a toolchain-verify assertion** for the 6.6.x layout, and all 30 `lib/` files were
  byte-checked against the **6.6.1 snapshot itself**, not old-pin vs new-pin — the shortcut that hid
  a stale ganita for three releases.

## [2.11.2] - 2026-08-21 — the toolchain catch-up, and the three stale constants it found

Fifteen toolchain releases in one bump — cyrius **6.5.18 → 6.5.33** — plus sakshi 2.4.10 → 2.4.11
and, vendored with the toolchain, **ganita 1.0.4 → 1.1.4**. No feature work. Suite **3514 → 3526**,
constant gate 159/159, coverage 640/644 (99%) over 36/36 files, fuzz clean.

**Every finding in this release is a number that was written down once and then trusted.** Three of
them, in three different files, all the same shape as 2.11.1's `ALLOC_MAX` finding: a constant
derived from a dependency, correct when written, silently false afterwards. None was found by a
failing test — all 3514 assertions passed on **both** sides of the bump.

### Changed — toolchain, deps, and the vendored stdlib

- **`cyrius.cyml`**: `cyrius = "6.5.18"` → `"6.5.33"`, sakshi `2.4.10` → `2.4.11`.
- **`lib/` re-vendored from the 6.5.33 snapshot** — all 29 stdlib files now byte-identical to
  `~/.cyrius/versions/6.5.33/lib`, verified file by file rather than assumed.

  ⚠ **The tree arrived mid-sync and the half that was missing was the half that mattered.** `lib/`
  had been synced against **≈6.5.19**, not the target pin: `alloc`, `assert`, `atomic`, `bench` and
  `syscalls_windows` were current, while **`ganita.cyr` (1.0.4, −202 lines)**, `fmt.cyr` and three
  more syscalls variants were behind. Comparing old-pin against new-pin would have shown a tidy
  diff and missed it; only comparing against **the pin's own snapshot** finds it. That is the
  third time ganita specifically has been caught stale this way.

- **Code-only stdlib delta, measured rather than eyeballed.** A brace-aware extractor that strips
  comments puts the whole 6.5.18 → 6.5.33 stdlib change at **four functions**: `ganita_f64_pow`
  (below), `fmt_float_buf` (dropped the carry when a fraction rounded up to a full unit — `3 - 1e-7`
  printed `2.1000000`), `assert_eq` (its two numbers went to fd 1 while the rest of the message went
  to fd 2, so any harness capturing the streams separately saw them orphaned), and `bench.cyr`
  (below). ⚠ A first pass keyed on `fn` alone reported **7 changed ganita bodies**; six of those were
  the *trailing comment block* being attributed to the preceding function. The instrument was wrong
  before the measurement was.

### Fixed — `f64_pow`'s domain moved under us, and it changed a hisab public API

ganita 1.1.4 fixed `ganita_f64_pow` upstream: zero base, zero exponent, and **negative base with an
integral exponent** are special-cased instead of falling into `exp(n*ln(base))` and returning NaN.

⭐ **`expr_eval` was sitting directly on it.** `src/symbolic.cyr`'s `EXPR_POW` branch calls `f64_pow`
raw, so evaluating a negative base at an integer power returned **NaN for hisab's entire history**
and returns a number now. `f64_pow(0,0)` was NaN and is 1; `f64_pow(0,-1)` was NaN and is +Inf.
**No test covered any of it**, which is exactly why all 3514 assertions passed either side.

- **12 assertions added** pinning the new domain as a contract, so a silent revert upstream cannot
  reintroduce a NaN nobody is watching for. ⚠ **Discrimination measured, not asserted**: the group
  was re-run against a checkout still holding ganita 1.0.4 and **10 of the 12 fail there**. The two
  that pass on both sides are the two controls — `(-2)^0.5` still NaN, `_ad_pow(-2,4)` still exactly
  16. A group where every line flipped would mean the controls were not controls.

- **`_ad_pow` stays, and its stated reason was rewritten because the old one is now false.** It
  existed to escape a *domain* restriction that no longer exists. It survives on **precision**: the
  upstream fix takes its magnitude from `exp(n*ln|base|)` and applies the sign, so it inherits the
  transcendental round-trip. Measured — `f64_pow(-2,3)` is `0xC01FFFFFFFFFFFFE`
  (−7.99999999999999982) against `0xC020000000000000`, exactly −8 — and since `f64_to` **truncates**,
  the two paths disagree by a whole integer: `(-2)^4` reads **15** through the stdlib and **16**
  through hisab.

- Stale claims corrected where they had become false rather than historical: `src/autodiff.cyr`'s
  `_ad_pow` header, two assertion messages in `tests/modules.tcyr` that asserted something about the
  stdlib the stdlib had stopped doing, the note block in `tests/abuse.tcyr` (moved to past tense),
  and `docs/doc-health.md`. Historical narrative — the roadmap's per-release rows, prior CHANGELOG
  entries — was **deliberately left alone**: a figure stamped "as of 2.11.0" is a record, not a claim.

### Fixed — the benchmark instrument changed, and nothing in the harness could have said so

cyrius 6.5.19 taught `lib/bench.cyr` to **calibrate one clock read on the host and subtract it from
every sample**, and taught `bench_run` to **size its own batches** instead of wrapping a clock pair
around every iteration. Both rewrite the number without touching the code being measured.

⭐ **44 of 72 rows moved more than 10% and not one of them is a speedup.** Measured on this host
(floor ~1,343 ns), same source, zero code changes: `ease_in_out` **1,407 ns → 7 ns**, `cx_mul`
1,459 → 37, `quat_mul` 1,473 → 67, `perlin_2d` 1,461 → 82. Those four were **94–100% instrument**.

⚠ **And the old numbers FLATTENED them.** Four operations spanning **149x** in reality were reported
within **1.26x** of each other, because the floor dominated all four. A real regression had room to
hide inside that — the same failure 2.10.0 recorded when triangle/sphere measured 1.19x against a
true 3.6x. The 2.10.0 sweep moved 17 benchmarks off the floor; **these four were still on it**, and
it took the toolchain fixing the instrument to surface them.

- **`bench-history.csv` gains `regime` and `floor_ns`, and the trend filter now enforces them.**
  Filtering on `stat` alone was not enough: rows either side of this boundary are all `avg`, so
  every one of those 44 artefacts would have printed as an improvement. **This is the third
  measurement-method change to move every row** (2.9.2 max→avg, 2.10.0 bench→bench_batch, now the
  timer floor), and the first two were each answered with a paragraph of prose the trend table could
  not read. CLAUDE.md says to wait for the third instance before extracting an abstraction — so the
  marker is now a column, and the run reports the 1,734 rows it excluded rather than quietly
  dropping them.

  ⚠ **`regime` is DERIVED, not declared** — read from whether the harness printed its own measured
  floor — so it cannot go stale the way the constant it replaces did. ⚠ **What it cannot do, said
  plainly:** it separates "floor subtracted" from "floor not subtracted". A future instrument change
  that kept printing the line would not flip it. This narrows the hole; it does not close it.

- `tests/hisab.bcyr`'s `bench_batch` header cited "~240 ns of overhead per op" from `lib/bench.cyr`.
  Upstream **retired** that figure: a clock read is 15–32 ns on macOS arm64 and ~3,550 ns on aarch64
  Linux — a **230x spread** — so no single number belongs in a comment. Replaced with a pointer to
  `bench_clock_overhead_ns()`.

### Fixed — CI installed the toolchain by hand, and it broke at 6.5.33

Both workflows fetched the release tarball and laid it out **flat** as
`~/.cyrius/{bin,lib}`. cyrius resolves the stdlib snapshot from
`~/.cyrius/versions/<pin>/lib`, so `cyrius deps` failed with *"pins version 6.5.33 but it is not
installed at /home/runner/.cyrius/versions/6.5.33/lib"*. Both now pipe the pin into the **upstream
installer** (`scripts/install.sh`), matching patra. The pin stays the single source of truth — no
version is hardcoded in the YAML.

⚠ **Only one of the three defects in that block was visible.** The layout was the one that failed;
the other two had been silently true for as long as the block existed:

- **No integrity check at all.** It was a bare `curl -sLO` on a release tarball, untarred. The
  installer **requires** the published `.sha256` and fails closed when it is missing, unverifiable
  or mismatched (CVE-21), and verifies an **Ed25519 signature** over `SHA256SUMS` where one is
  published (CVE-13). Both were confirmed live: an end-to-end install into a scratch `CYRIUS_HOME`
  printed `checksum verified` and `signature verified (Ed25519)`. This repo ships a `SECURITY.md`
  and a security job and was installing an unverified binary toolchain — on the **release**
  workflow too, which builds the artifact that ships.
- **The step could not fail.** All three copies ended in `2>/dev/null || true`, so an empty `bin/`
  or a missing `lib/` "installed successfully" and surfaced later as an unrelated error. The
  installer errors loudly on a tarball missing either. **A step that cannot fail is not a step** —
  the same shape as 2.9.2's three non-gating gates.

- **New step: `Verify toolchain matches the pin`**, in both workflows. The original failure showed
  up two steps downstream as a path error from `cyrius deps`. It is now asserted where it is
  diagnosable: `cyrius version` must equal the manifest pin, and
  `~/.cyrius/versions/<pin>/lib` must exist. Verified against a real install in a scratch
  `CYRIUS_HOME` — 102 files present, both assertions pass — with the machine's own toolchain
  untouched.

### Changed — 38 files reformatted (`cyrius fmt` fixed its own output)

cyrius 6.5.28 fixed `cyrfmt`, which had never tracked parentheses: every line was indented at
`brace_depth * 4`, so a continuation line inside an unclosed `(` came out at the enclosing
statement's indent and `--check` then rejected the formatter's own output, **exiting 1 in complete
silence**. Canonical is now 2 spaces per open-paren level.

- **38 of 44 source files failed `fmt --check`** on arrival and are reformatted. ⚠ Proven
  **leading-indentation-only**: stripping leading whitespace line-for-line leaves both versions
  byte-identical, and no file changed line count. Longest line is now **119** of the 120-char lint
  limit — tight enough to be worth knowing.
- ⚠ **`cyrius fmt` REWRITES IN PLACE as of 6.5.28** (it was stdout-only). Anything shaped like
  `cyrius fmt f.cyr > f.new` must move to `--dry`. Nothing in this repo did.

### Fixed — the bundle-size ceiling this project has designed around does not exist any more

`cyrius.cyml` documented the bundle as "~786 KB / 21,368 lines — **76.8% of cycc 6.5.16's 1 MB
input_buf** (243,097 B headroom)", and CLAUDE.md carried the same ceiling at 85.4%.

⭐ **cyrius 6.5.22 relocated `input_buf` from `0x00000` to `0x4D9D000` and raised `_SRC_CAP` from
1 MB to 16,777,216 B** — the 1 MB cap had been refusing sigil (1,084,265 B), mabda (1,259,999 B) and
drishti (1,403,806 B) outright. The real figures are **898,472 B / 23,311 lines = 5.4%**, with
**15,878,744 B** of headroom.

⚠ **Verified, not read off a comment**: a 1,162,472 B source — 111% of the retired cap — compiles
clean through `cyrius check --with-deps`. The bundle-size pressure that shaped `[lib]` is gone.
Also corrected there: the module count, which said 34 while listing 35.

### Verification

- Suite **3526/3526** across 5 files (hisab 416, foundation 351, modules **1787**, edge_cases 233,
  abuse 739) · fuzz 1/1 · coverage **640/644 (99%)** over 36/36 files.
- `cyrius lint` 44/44 files, 0 warnings · `cyrius fmt --check` 44/44 clean · `cyrius vet` clean.
- `scripts/check-constants.sh` **159/159**, 1 skipped · `check-measurements.sh --selftest` recall
  21/22, 0 of 15 decoys flagged.
- `cyrius deps --verify` **30/30** · `cyrius distlib --check` no drift ·
  `cyrius check --with-deps dist/hisab.cyr` ok · build → 224 KB static x86_64 ELF.
- ⚠ **`bench-history.csv`'s pre-2026-08-21 rows are not comparable to this run's** and the trend
  table no longer pretends otherwise. **No performance claim is made in 2.11.2.**

## [2.11.1] - 2026-08-11 — the audit release: one rule, and the four sites that broke it

A full P(-1) sweep of the v2.11.0 tree — six dimensions, 115 checks, every finding handed to an
independent skeptic instructed to **refute** it. Suite **3507 → 3514**, constant gate 158 → **159**.
No feature work; every change here is a repair or a correction.

The full report is `docs/audit/2026-08-11-v2.11.0-full.md`. It found **52 reproduced findings, 21
confirmed**, and they are overwhelmingly **one root cause**: a guard compares a quantity against a
threshold that is wrong for it, then **fabricates a plausible answer** instead of reporting. This
release fixes the four with the shortest path to a caller and schedules the rest.

⚠ **28 of the 52 were reproduced but never verified** — the harness capped the verify phase at 24.
Two of the 24 that *were* checked came back REFUTED, so roughly one in twelve unverified findings is
wrong. They are unproven, not pending, and none was acted on. That cap is recorded in the report as
the audit's own biggest process defect.

### Fixed

- **`hquat_inverse` fabricated the identity for any quaternion shorter than 1e-6.** `len_sq` is a
  **squared** magnitude compared against the **unsquared** `EPSILON_F64`. Measured, `|q · q⁻¹ − I|`
  jumps from 0 to **0.9999999** between `|q| = 1e-6` and `1e-7`: the returned "inverse" is the
  identity, so the product is `q` itself. A quaternion of magnitude 1.4e-7 has an inverse of
  magnitude 7e6 — finite, exactly representable, silently replaced by a rotation of zero.
  `hquat_normalize` took the same rule. Both now guard at `_QUAT_F64_TINY` = DBL_MIN.

  **Ninth instance of a class first written down in `complex.cyr:58` in 2.6.14.**
  → `issues/archived/2026-08-11-quat-inverse-squared-epsilon.md`

- **Three caller-sized allocations in `num_ext.cyr` stored through `alloc`'s 0 return** —
  `num_fft_2d`, `num_ifft_2d`, `num_tridiag_solve`. Reproduced as **SIGSEGV (exit 139)** from public
  entry points; `num_fft_2d` requires only that its dimensions be powers of two, with no upper
  bound. All three now return `HSB_ERR_ALLOC`. Removing the guard does not fail an assertion — it
  **crashes the suite process**. → `issues/archived/2026-08-11-caller-sized-allocs-in-num_ext.md`

- **`ad_tape_new` returned a live handle whose node array was null** — ⚠ **2.11.0's own code, hours
  old.** When `cap * 40` exceeds `ALLOC_MAX` the node allocation returns 0, but the handle was
  already allocated, so a caller's `if (t != 0)` passed and the first `ad_var` SIGSEGVed inside
  `_ad_push`, whose only bound is `n >= cap` — which `n = 0` satisfies. Construction is the only
  place that can catch it, and 0 is the function's own documented reject value.

  2.11.0 wrote `AD_FULL` for exactly this class three functions away, in the hot path, and missed it
  at construction. **Handling a failure mode in the hot path is not handling it at construction.**
  → `issues/archived/2026-08-11-ad-tape-new-null-nodes.md`

- **`optimize.cyr` derived its dimension ceiling from a constant that moved.** Its comment read
  `ALLOC_MAX (0x10000000 = 256 MB) … hard ceiling at n = 5792`. `ALLOC_MAX` has been `0x80000000`
  = **2 GiB** since cyrius 6.4.51 — the real ceiling is **16384** — and `linalg_ext.cyr:789` had
  already been updated while this file and `threat-model.md` had not. The policy cap of 4096 sits
  below both, so nothing was ever unsafe, which is exactly why it survived four toolchain bumps.

  ⚠ **That stale constant nearly buried the `num_ext` defect above.** The first reproduction probe
  was sized against 256 MiB, landed *exactly* on the limit, allocated successfully, and the finding
  looked refuted. **A constant derived from a dependency is a measurement, and it goes stale
  silently.**

### Verification

- **All four repairs mutation-proven.** The `num_ext` mutant crashes the suite process; the
  `ad_tape_new` mutant fails it; the quaternion repair kills three.
- ⚠ **The quaternion sweep's first draft killed only one of three mutants.** Written over 12
  decades, it bottomed out at `|q| ~ 1e-11` — **above both thresholds a repair might plausibly have
  chosen instead** (`EPSILON_F64` at 1e-12, `_GEO_F64_EPS_SQ` at 1e-12 on the magnitude). Extended
  to 20 decades, all three die. *A sweep must bracket every candidate threshold, not just the chosen
  one* — the same failure 2.10.1 recorded, recurring despite being written down.
- ⚠ A second discrimination trap avoided deliberately: asserting only that `hquat_normalize` returns
  something of **unit length** does not discriminate, because the fabricated identity *is* unit. The
  assertion checks the **direction**.
- An independent probe, written from scratch after the scratchpad was lost, re-checks all four
  repairs **without going through the suite** — including that `num_tridiag_solve` **still solves an
  ordinary 4×4**, since a guard that rejected everything would pass the failure test and be useless.

### What the audit found and this release does NOT fix

Scheduled, with reasons, in the report and on the roadmap:

- **~20 more sites of the same epsilon class**, including `eigen_qr` returning wrong eigenvalues with
  `rc = 0`, `cqr_decompose` returning a non-triangular `R` with `rc = 0`, `solve_bicgstab` returning
  the initial guess with no error channel at all, and `m3_inverse`/`m4_inverse` returning the
  **identity** for invertible matrices. Each is a behaviour change on a documented entry point and
  lands as its own mutation-proven bite, not one commit.
- **Three more unchecked allocations** and four entry points that **abort the process**.
- **The suite itself**: 836 of 3510 assertions (23.8%) compare through `f64_to`, which **truncates**
  — 3510 being the audit's own static count of assertion sites, against the 3507 the harness reported
  at run time; the two are counted differently and neither is wrong —
  so any error below 1.0 is invisible to a quarter of the suite; 38.4% of value-changing mutants
  survive all five suites. That is why 3507 assertions and 99% coverage saw none of this, and
  repairing it strengthens 836 existing assertions at once.

⚠ **`cx_div` is named in `complex.cyr:58`'s own 2.6.14 comment recording this exact lesson, and is
still fabricating zero below 1e-12** — verified live in this release's own re-check. Writing a
lesson beside the code that taught it neither fixes that code nor reaches the other thirty-four
modules. Only a grep does.

## [2.11.0] - 2026-08-11 — reverse-mode autodiff, and the five forward-mode defects it found first

Tape-based reverse mode: one sweep produces the gradient of an n-input scalar where forward-mode
duals need n passes. Paired with `optimize.cyr`'s solvers, which have always required a
caller-supplied gradient. Suite **3469 → 3507**, benchmarks 70 → **72**, constant gate 157 →
**158**, coverage **640/644**. All gates green.

**And for the fourth release running, the feature's own oracle found defects in shipped code
first.** Reverse mode is validated *against* forward mode — that is the standard and the cheapest
check available — so `src/autodiff.cyr` was swept before a line of tape code was written. Five
defects, every one of which would have been inherited and then *confirmed* by the two agreeing.

### Fixed

- **Five defects in forward-mode autodiff, all behind guards no finite difference can reach.**

  | input | returned | truth |
  |---|--:|--:|
  | `1 / 1e-13` | **(0, 0)** | (1e13, −1e26) |
  | `ln(−5)` | **(NaN, −0.2)** | miss sentinel |
  | `ln(1e-13)` | **(0, 0)** | (−29.9336, 1e13) |
  | `sqrt(−4)` | **(NaN, NaN)** | miss sentinel |
  | `d/dx x¹ at 0` | **NaN** | 1 |
  | `d/dx x³ at −2` | **NaN** | 12 |

  `ln(−5) → (NaN, −0.2)` is the worst of them: a NaN value paired with a **confident finite
  derivative**, so a caller checking only the gradient — the entire point of an autodiff module —
  sees nothing wrong.

  Causes, one per guard. `dual_div` guarded at `EPSILON_F64` where the division only fails below
  DBL_MIN — the same class 2.10.2 repaired four times in `geo.cyr`, and repaired the same way.
  `dual_ln` tested `f64_abs(v)`, which is not the domain of `ln`, so negatives sailed past it.
  `dual_sqrt` computed `s = sqrt(v)` *before* its guard. `dual_pow` called `f64_pow`, documented in
  `lib/ganita.cyr` as `exp(n·ln(base))` and therefore NaN for every non-positive base — ⚠ **that is
  the stdlib's stated implementation, not a hidden defect**; the defect was hisab inheriting the
  restriction undocumented, on a function whose commonest use is polynomials. Integer exponents now
  go through repeated multiplication.

  ⚠ **THE FINITE-DIFFERENCE SWEEP ALONE FOUND NOTHING.** A 13-op sweep was written first and
  reported clean, for a structural reason: **at every input where a guard fires, the perturbed
  scalar is NaN too**, so the sample is skipped and the guard is never asked. It skipped exactly the
  rows that mattered. *A guard has to be interrogated directly; differencing a function cannot reach
  the branch that refuses to evaluate it.* The suite carries both, plus a **control** pinning that
  `(−2)^0.5` must still be NaN — without it, "fixed the NaN" and "replaced one fabrication with
  another" are indistinguishable.
  → `issues/archived/2026-08-11-forward-mode-dual-guards.md`

### Added

- **Tape-based reverse mode** — `ad_tape_new`, `ad_var`, `ad_const`, twelve operations mirroring the
  duals, `ad_grad` and `ad_grad_of`. One node per operation records its value, both local partials
  and both input indices. Because every node's inputs have **strictly smaller indices**, a single
  descending loop is a valid reverse topological order — no sort, no visited set, no recursion.

  `AD_FULL` is returned by every op on overflow and **propagates**: an op consuming a bad index is
  itself bad, so a caller checking only the final index is still safe. `ad_tape_reset` reuses one
  tape across many gradients, because the bump allocator never frees and a tape per iteration is a
  leak rather than a style choice.

  It lives in `src/autodiff.cyr` rather than a 36th module, deliberately: reverse mode is verified
  against forward mode, and keeping them together makes that pairing structural instead of a
  convention someone has to remember.

- **`ad_grad_write` / `ad_grad_into`** — the pairing with `optimize.cyr`. ⚠ **No API change was
  needed**: `opt_*` take `fncall2(grad, x, out)`, and a capturing closure holding the tape matches
  that exactly. That this works at all is recent and was **re-run rather than taken from the release
  note** — a capturing closure called through a function boundary SIGSEGVed on 6.5.16, the same
  defect that still blocks `vec_sort_by`; re-verified on 6.5.18.

  ⚠ **No `ad_minimize` convenience is shipped, and that is a decision.** It would put a call to
  `optimize.cyr` inside `autodiff.cyr` — a module dependency buying a caller four lines, in exchange
  for a `[lib]` ordering constraint and a module that can no longer be read on its own.

### Performance

- **`grad_fwd_16` 63.7 µs → `grad_rev_16` 5.70 µs — 11.2×** for the full gradient of a 16-input
  scalar. A second recorded run gives 68.8 µs / 6.18 µs = **11.1×**, so the ratio is stable across
  runs rather than a single reading; both rows are in `bench-history.csv`. The forward row computes **all 16 partials**; timing one dual pass against a full reverse
  sweep would have flattered reverse mode 16× for free.

  ⚠ Not the theoretical 16×, and the shortfall is stated rather than smoothed over: reverse mode
  pays to **record** the tape, and part of the remaining gap is **allocation, not arithmetic** —
  every `dual_*` op heap-allocates a 16-byte `Dual` under an allocator that never frees, while the
  reverse row calls `ad_tape_reset` and writes into memory it already owns. The ratio is a property
  of the two *implementations*, not of forward versus reverse in the abstract.

### Verification

- **Reverse mode checked against BOTH oracles, which are not interchangeable.** Reverse and forward
  share the domain guards, so their agreement is partly tautological; only central differences are
  independent of both. On a four-input objective exercising every op, worst disagreement **2.02e-10**
  against each. Two inputs feed **more than one consumer** — a chain where every node has one
  consumer never exercises the reverse *accumulation*, which is the only thing the sweep does that
  forward mode does not.
- **The solvers were checked against their own contract before any integration was written, and
  this time it found nothing.** All four return `HSB_ERR_NONE` only from
  `if (f64_lt(gnorm, tol) == 1)`; 16 runs across two objectives and both gradient sources honoured
  it. Recorded because a clean result from that check is worth as much as a dirty one — it is the
  fifth release it has been run and the first to come back empty.
- **Fourteen mutants, fourteen kills** — including the two structural ones on the sweep: clearing
  the adjoint array only up to `root` (a second gradient from a shorter root then reads the previous
  sweep's values), and sweeping ascending instead of descending.

  ⚠ **Two of them survived the first test pass**, and both were real gaps. Replacing
  `ad_grad_of(t, load64(ids + k*8))` with `ad_grad_of(t, k)` survived because the fixture's
  variables happened to *be* nodes 0 and 1 — the whole purpose of `ids` is that they need not be.
  And dropping `ad_grad_into`'s error check survived because nothing called it with a bad root. New
  fixtures put two constants first and reverse the `ids`, and call it with `AD_FULL` asserting the
  caller's buffer is untouched.

  ⚠ **And one failure was the assertion's fault, not the code's.** The first end-to-end optimizer
  test asserted the minimiser to `LOOSE_TOL_M` (1e-8) and failed 4 of 8: gradient descent stops at
  `‖g‖ < 1e-6` and therefore lands 5e-7 away, **exactly as promised**. The tolerance is now
  *derived* from the contract — `grad = 2(x − x*)`, so `‖x − x*‖ < tol/2`. The other three solvers
  hid it by converging far tighter.

## [2.10.2] - 2026-08-11 — the primal defects the jets were sitting on

2.10.1 shipped jets for all six ray primitives and filed three defects **in the primal** that the
jets inherit by design. This release closes all three, plus the OBB rotation derivative 2.10.1
deferred. Suite **3453 → 3469**, constant gate 156 → **157**, `geo_jet_obb` 12 → **15 partials**.
All gates green.

**One rule settled four thresholds.** Every guard repaired here was comparing a geometric quantity
against a tolerance chosen for a different quantity. The rule the release converged on is *guard
exactly what makes the DIVISION fail, and nothing more* — `_GEO_F64_TINY` = DBL_MIN, the smallest
magnitude whose reciprocal is finite. It needs no scale chosen for it, which is precisely why the
old thresholds were wrong.

### Fixed

- **The slab parallel test was scale-free, and returned hits that are not there.** `|d_k| <
  EPSILON_F64` (1e-12) deleted that slab; over `t ~ 1e12` a component of `9e-13` moves the
  coordinate by ~0.9, most of a unit box, so the slab that would have **rejected** the ray was
  deleted and a confident hit came back.

  | | before | after |
  |---|--:|--:|
  | phantom hit, `\|d\| = 1.0` exactly | `t = 1e12`, hit point **x = 1.4** for a box spanning [0,1] | **0** |
  | interior exit | `t = 1e13`, hit x = 9.5 | `5.5556e11`, hit x = **1.0** |
  | aabb vs obb at exactly `EPSILON_F64` | `0` vs **`4.5e12`** | `0` vs `0` |
  | `+Inf` from entirely finite inputs | **`+Inf`** | **0** |

  `geo_ray_new` normalizes and did **not** protect — the phantom-hit direction is unit to 16
  digits. The permanent assertion is a sweep over direction-component *magnitude*, 18 decades × 2
  drift factors × 2 primitives, checking every reported hit point lies on its box.

  ⚠ **The suite caught this repair overturning a considered contract decision.** The first tail
  guard tested *finiteness*, which rejects NaN too — and `abuse.tcyr` asserts *"a NaN ray yields
  NaN, not a fabricated hit distance"*, which 2.9.0 wrote with a reason: the miss sentinel is 0, so
  turning NaN into 0 silently reports "no hit" while NaN is unmistakable at the call site. Narrowed
  to `±Inf`. The remaining NaN item — a NaN in *one* axis erased by `f64_max` — is module-wide and
  left open. → `issues/archived/2026-08-10-slab-parallel-test-is-scale-free.md`

- **`geo_ray_capsule` returned a point a full radius INSIDE the solid.** `geo_ray_sphere` returns
  the *first* non-negative root, and that is all the cap branches ever saw — so a root rejected by
  the half-space test meant the other root was never considered.

  | | before | after |
  |---|--:|--:|
  | axial ray from inside, `t` | 4.0 | **6.0** |
  | its surface residual `\|dist − r\|` | **1.0** (on the axis) | **0.0** |
  | 600 random interior origins, exits dropped | 32 (5.3%) | **0** |

  The quadratic is split onto `_geo_sphere_roots`, which writes **both** roots; `geo_ray_sphere` is
  now the root *selection* on top of it, so there is one copy and they cannot drift. The separate
  `cyl_missed` fallback is **deleted** — caps are evaluated on every path, filtered by half-space.
  The assertion is a **surface-residual** sweep, not a `t` check: the broken code returned 4.0, a
  plausible number any loose `t` assertion would accept.
  → `issues/archived/2026-08-10-capsule-cyl-missed-tags-a-sphere-root-as-a-cap.md`

- **Four more squared-vs-unsquared guards, two of which 2.10.1 sorted WRONG by reading rather than
  running.** Scale covariance — the same scene in different units must give the same answer:

  | | broke at | now |
  |---|--:|--:|
  | `geo_triangle_unit_normal` | **s = 1e-4** (its `len_sq` is the squared cross product, `s⁴`) | exact to 1e-9 |
  | `geo_segment_direction` | 1e-7 | exact to 1e-9 |
  | `geo_segment_distance_to_point` | 1e-7 | exact to 1e-9 |
  | `geo_ray_capsule` degenerate | 1e-7 | exact to 1e-9 |

  `geo_triangle_unit_normal` returned a **fabricated** `(0,1,0)` for a well-formed right triangle
  where the truth is `(0,0,1)`, with no error signal. ⚠ `geo_segment_closest_point` was repaired
  **first, deliberately**: `geo_segment_distance_to_point` builds on it and is what the capsule
  sweep above uses as its **oracle** — it was silently misleading the probe that found these
  defects. ⚠ An x-aligned test segment does **not** discriminate for `geo_segment_direction`: its
  fallback *is* the right answer there. The sweep uses a diagonal.

### Added

- **`geo_jet_obb_drotation` — dt/d(rotation), 12 → 15 partials.** 2.10.1 deferred it and its stated
  objection was right: a **free** perturbation of an axis leaves the rotation group, so a finite
  difference across it measures nothing. The answer is to perturb *within* the group. For a
  world-frame angular vector `w`, `a_k → a_k + w × a_k`, and differentiating the face equation with
  the scalar triple product gives

  ```
  dt/dw = [a_k × (c − p)] / f_k
  ```

  One cross product. It is a **world-frame angular** gradient, three components, not a
  four-component quaternion one: that is the shape an optimiser wants — a step along it is a valid
  rotation by construction — and it is what the derivation produces without a chain rule. Verified
  against central differences of the shipped primal by pre-multiplying the quaternion by
  `from_axis_angle(e_i, ±h)`: **36 comparisons, worst 1.9e-10**. Three mutants killed.

### Performance

- Interleaved A/B, three pairs each:

  | | 2.10.2 | 2.10.1 | |
  |---|--:|--:|---|
  | `ray_capsule` | 627 ns | 500 ns | **+25.3%** |
  | `ray_capsule_diag` | 858 ns | 932 ns | **−7.9%** — *faster* |
  | `ray_sphere` | 101 ns | 96 ns | +5.5% (the two-root split) |
  | `ray_aabb_diag` | 118 ns | 115 ns | +2.9% |
  | `ray_aabb` | 93 ns | 92 ns | +1.1%, inside noise |
  | `ray_obb` | 494 ns | 497 ns | −0.6% |

  The `ray_capsule` fixture is the axial one that used to take the wrong-but-cheap fallback, so the
  comparison is not like-for-like: the fast path it replaces returned a point inside the solid.
  `ray_capsule_diag` is *faster* because the scalar half-space test replaces two point
  materialisations.

  ⚠ **Two versions of this release cost far more before being measured.** The `is_inf` tail check
  written as a **function** cost 5 ns of a 90 ns routine (+5.6% on `ray_aabb`); inlined at its four
  sites it is +1.1%. And the first working cap repair materialised the hit point per root —
  `geo_ray_at` + `hvec3_sub`, both of which **allocate**, four times per query under an allocator
  that never frees — at **+92%** on `ray_capsule`. The half-space test is algebraically scalar:
  `(p − cap_pt)·ab == (o − cap_pt)·ab + t·(d·ab)`, so both dot products hoist.

### Verification

- **Suite 3453 → 3469**, 0 failures. Constant gate 156 → **157**.
- **13 mutants written, 13 killed** across the four repairs and the new partial.
- ⚠ **Two fixture faults caught by counters rather than by failures.** The slab surface sweep's
  first draft was **all misses** — with a reach of `1/tiny` the drift is exactly one box-width, so
  72 checks asserted nothing about a hit point; the `_SF_HITS` counter said so. And the rotation
  sweep's first draft used hand-picked directions and **3 of 4 configurations missed**, leaving
  three comparisons; aiming at an off-centre target gives 24.
- ⚠ **A toolchain defect was nearly filed that does not exist.** `var rt[16]` passed as bare `rt`
  SIGSEGVs on the first call — a stack array is a *value*, and its address is `&rt`. It fails so
  totally that a minimal repro was written and characterised before `lib/fmt.cyr` was read and
  found using `&buf` throughout. **The stdlib is the specification.**

### Deferred, with reasons

- **The three EPA squared-epsilon sites** (`geo_advanced.cyr:1056/1060/1067`). Same class, but
  tightening them is a narrowphase behaviour change on a routine with a measured cost and a live
  open filing (`epa-certificate`); it needs its own before/after rather than riding along.
- **NaN in one axis laundered into a plausible `t`.** Real, and module-wide —
  `geo_ray_sphere`, `_plane` and `_triangle` behave identically — so it is a contract question for
  all six, not an AABB defect.
- **Whether the tie flag should name *which* faces tied** rather than being a boolean. A caller
  doing subgradient descent might prefer both faces' gradients to none. **No consumer has asked**;
  left as a boolean and recorded.

## [2.10.1] - 2026-08-10 — the branchy primitives, and two more defects the design check found first

The other half of 2.10.0's scope: jets for the three primitives whose answer comes from a
**branch** — aabb, obb, capsule — so all six `geo_ray_*` are now differentiable. Each primal gains
a face/branch-reporting variant and becomes a one-line wrapper over it, so **the value path is
unchanged by construction**: the `f64_max`/`f64_min` calls are byte-for-byte what they were and
every added line sits behind `if (out != 0)`. Suite **3398 → 3453**, benchmarks **64 → 70**,
constant gate 155 → **156**, toolchain 6.5.17 → **6.5.18**. All gates green.

**And for the third release running, the design check found defects in shipped code before any
feature code was written.** The jets need to know which face the slab method selected; asking
*"what does the primal return when there is no face?"* produced two:

- `geo_ray_aabb` and `geo_ray_obb` returned **+Infinity** for a degenerate direction with the
  origin inside the volume. Four of six siblings honoured the contract; two did not — the same
  4-vs-2 asymmetry, on the same six functions, as 2.10.0's homogeneity finding.
- `geo_ray_sphere` and `geo_ray_capsule` compared a **squared** length against `EPSILON_F64`, an
  unsquared tolerance. The sphere reported a **miss for any |d| < 1e-6** where the answer is finite
  and exactly representable; the capsule silently returned a **cap** hit instead of the cylinder
  hit, wrong by **0.942%**. ⚠ **The sphere's guard was introduced by 2.10.0's own repair** — and
  2.10.0's homogeneity sweep could not have caught it, because its direction scales are 2, 0.5 and
  7, all of order 1, none within five orders of magnitude of the threshold the same commit added.

### Added

- **`geo_ray_aabb_face` / `geo_ray_obb_face` / `geo_ray_capsule_branch`** — the split. Each takes
  an out-pointer and reports which face or branch produced `t`; the plain entry point passes 0.
  Public rather than the roadmap's private `_core`, because a face query is independently useful
  (a caller wanting the surface normal at the hit) and because a private symbol reached across
  modules is worse than a documented one.

  **Face encoding**, published in `geo.cyr`: `2 * axis + plane`, plane 0 = min side, 1 = max side;
  `GEO_FACE_NONE` (−1) when none; `geo_face_axis` / `geo_face_is_max` to decode. Out block is 16
  bytes: `out[0]` the face, `out[1]` a **tie flag** set when the selected extremum was achieved by
  more than one slab — an exact edge or corner hit.

  **Capsule branches**: `GEO_CAP_CYLINDER` / `START_CAP` / `END_CAP` / `SPHERE` (the degenerate
  zero-length case). `out[1]` is unused there — a capsule has no edge.

- **`geo_jet_aabb`** — 12 partials. Once the face is known the slab method reduces to a single
  plane, `t = (b_k − o_k)/d_k`, so every gradient has support on axis `k` alone:
  `dt/do` and `dt/dd` carry `−1/d_k` and `−t/d_k`; `dt/dmin` carries `1/d_k` on a min face and the
  zero vector otherwise, `dt/dmax` the converse.

- **`geo_jet_obb`** — 12 partials, the same reduction in the box's own frame with
  `t = (e_k + sg·he_k)/f_k`. ⚠ **`dt/d(rotation)` is deferred with a stated reason, not omitted**:
  the axes come from a unit quaternion, so a free perturbation of an axis leaves the rotation
  group and a finite difference across it measures nothing a caller can use. Same objection that
  made `dt/dd` a raw partial with `geo_jet_dt_dd_unit` offered separately.

- **`geo_jet_capsule`** — 13 partials, three branches. Caps reduce to `geo_jet_sphere` on that
  endpoint with the other endpoint's gradient identically zero. The cylinder comes from
  `F = |q|² − r²` with `q` the component of `p − a` perpendicular to `ab`, giving
  `dt/da = (1−s)q/den` and `dt/db = s·q/den` — the `ds·ab` term drops out of `q·dq` because
  `q · ab = 0` by construction.

- **`geo_jet_face`** and the typed accessors `geo_jet_aabb_dmin/_dmax`,
  `geo_jet_obb_dcenter/_dhalf_extents`, `geo_jet_capsule_dstart/_dend/_dradius`. `GeoJet` grew a
  `face` slot, 64 → **72 bytes**, appended so no existing offset moves.

- **Six benchmarks**: `jet_aabb`, `jet_obb`, `jet_capsule`, plus the primal rows they need —
  `ray_obb` (the **only** ray primitive that had no benchmark, and the most expensive of the six),
  `ray_aabb_diag` and `ray_capsule_diag`.

### Fixed

- **`geo_ray_aabb` / `geo_ray_obb` returned +Infinity for a degenerate ray direction.** With every
  slab skipped as parallel, nothing narrows the interval and the tail handed back the `+Inf` it
  initialised `t_max` with — from functions whose contract is stated three times in `geo.cyr` as
  *"0 on miss, positive f64 t on hit"*. `geo_ray_at(ray, +Inf)` on a zero direction is **NaN in
  every component**, so a consumer got NaN geometry with nothing raised.

  | primitive | origin INSIDE | origin OUTSIDE | |
  |---|--:|--:|---|
  | `geo_ray_aabb` | **inf** | 0 | ❌ |
  | `geo_ray_obb` | **inf** | 0 | ❌ |
  | the other four | 0 | 0 | ✅ |

  ⚠ **The origin position is load-bearing**: from outside, the parallel branch's own bounds check
  fires first and the contract is honoured by accident. A sweep firing only from outside — the
  natural way to write one — would have reproduced nothing.

  Fixed by counting contributing slabs, deliberately **not** by a `d·d < EPSILON_F64` guard up
  front: that test is the very defect below. **Cost: not measurable** — interleaved A/B, three runs
  each, every pair overlaps.
  → `issues/archived/2026-08-10-aabb-obb-degenerate-direction.md`

- **`geo_ray_sphere` and `geo_ray_capsule` tested a SQUARED length against an unsquared epsilon.**
  Homogeneity sweep, relative error, `999` = a real hit turned into a miss:

  | k | sphere | plane | triangle | aabb | obb | capsule |
  |---|--:|--:|--:|--:|--:|--:|
  | 1e-6 | 0 | 0 | 0 | 0 | 0 | **0.00942** |
  | 1e-7 | **999** | 0 | 0 | 0 | 0 | **0.00942** |
  | 1e-9 | **999** | 0 | 0 | 0 | 0 | **0.00942** |

  A unit sphere at `(0,0,5)` with `d = (0,0,1e-7)` has `t = 4e7`; the sphere returned **0**. The
  capsule's constant 0.942% at every scale below the threshold is the tell that it is a **branch
  switch**, not precision loss — below 1e-6 the cylinder test is rejected and the answer comes from
  the end-cap spheres. Fixed with `_GEO_F64_EPS_SQ` (1e-24); all six now exact to **k = 1e-11**.

  Second instance of a class `complex.cyr:58` has carried since **2.6.14** (*"mixing a squared
  quantity with an unsquared tolerance"*), with the fix idiom already in that file. **A lesson
  recorded in one module is not a lesson applied across the tree until something greps for it** —
  the filing carries a disposition table for all nine such sites in `src/`: two fixed, three EPA
  ones deferred to 2.10.2 with a reason, four left as documented degeneracy policies.
  → `issues/archived/2026-08-10-squared-epsilon-guards-in-geo-ray.md`

- **A dead branch update in `geo_ray_capsule`, found by a mutant that SURVIVED.** Nine mutants were
  written for the capsule jet; eight died. The ninth removed a branch assignment on the `cyl_t2`
  path and changed nothing — because `cyl_t1 ≤ cyl_t2` always (`a > 0`, `√disc ≥ 0`), so reaching
  that arm means `cyl_t1` already owns the answer and already set the branch. The line was
  unreachable-as-a-difference. Removed, invariant documented, and confirmed by measurement as well
  as on paper: **932 randomised cylinder quadratics, 0 with `t1 > t2`.** A surviving mutant is
  evidence about the code, not only about the test.

- **The jet tie flag was blind to two of the four ways a slab tie arises — found by an independent
  derivation commissioned against 2.10.1's own finished code.** The draft compared near against
  near and far against far, which catches an edge (two slabs achieving `t_min`) and a corner
  (three). It could not see:

  - **A zero-width slab** (`min_k == max_k`, or `he_k == 0`). Then `t1 == t2`, the primal's swap
    never runs — `f64_gt` is false on equality — and the face is labelled the **min slot
    unconditionally**, which is right only when `d_k > 0`. Measured, flat box, `d_x = -1`:

    | | `dt/dmin_x` | `dt/dmax_x` |
    |---|--:|--:|
    | jet said | **−1.0** | 0.0 |
    | one-sided truth | 0.0 | **−1.0** |

    The entire nonzero gradient in the wrong slot, `tie = 0` asserting differentiability, on
    **half** of all flat-box configurations — the mirror ray was right, so a fixture testing one
    direction passes on the broken code. Both directions are now asserted.
  - **A tangential clip** (`t_min == t_max`, different slabs owning the two ends). The hit sits on
    the *boundary* of the hit set, where an arbitrarily small sideways perturbation makes it a miss.

  Both are now covered by one line at the tail of each function: `if (f64_eq(t_min, t_max) == 1)`.
  ⚠ **A separate per-slab guard was written for the zero-width case and then DELETED as dead code**
  — a zero-width slab pins `t_min` and `t_max` to the same value, so it cannot fire without the
  tail line firing too. Established by mutation, not argument: removing the per-slab guard changed
  nothing; removing the tail line fails 5 assertions across both primitives.

  ⚠ **The first fixture written for the tangential clip was not tangential.** `o (-1, 2, 0.5)`,
  `d (1, -1, 0)` gives `t_min = 1, t_max = 2` — an ordinary edge hit, already covered. It passed,
  which is exactly the failure mode: a fixture that tests something already tested and reads as
  though it tested something new.
  → `issues/archived/2026-08-10-jet-tie-blind-to-degenerate-slabs.md`

### Changed

- **Toolchain pin `6.5.17` → `6.5.18`.** Compiler-only: **all 30 vendored files are byte-identical
  between the two pins**, so `lib sync` was a no-op and `deps --verify` stayed 30/30. Its headline
  fix is a `cyrius fmt` bug that corrupted multi-line string literals (upstream measured it
  rewriting 1,239 lines of cyrius's own `src/main.cyr`); hisab's 44 sources are **still fmt-clean
  under the repaired formatter**, checked rather than assumed.

  ⚠ The open **dead-function-bodies** filing was re-run on 6.5.18 from its own repro and is
  **unchanged**: `build` and `check --with-deps` reject (exit 1), **`lint` and `vet` still exit 0**
  on a file that does not parse. Still partial, still open.

- **`geo_ray_capsule`'s cylinder rejection threshold** is now `_GEO_F64_EPS_SQ`, not `EPSILON_F64`
  — see *Fixed* above. A near-axial ray now gets its barrel hit rather than a cap hit.

### Known — inherited by the jets, scheduled for 2.10.2

The jets are a post-pass on the shipped primal by design, so they are exactly as correct as it is.
The same commissioned probe found three defects **in the primal**, all re-run on this tree before
being written down, all left open because each changes what a hot, four-consumer function returns
and none belongs in a release scoped to the jets:

- **The slab parallel test is scale-free.** `|d_k| < EPSILON_F64` deletes that slab, which is wrong
  whenever the ray travels far enough for a tiny component to matter. Box `[0,1]³`, origin
  `(0.5, 0.5, -1e12)`, direction `(9e-13, 0, 1)` with **`|d| = 1.0`** returns `t = 1e12` and a hit
  point at **x = 1.4**. `geo_ray_new` normalizes and does not protect. `geo_jet_aabb` returns
  `tie = 0` and a full gradient on it. Also covers an 18×-too-large exit `t` from inside, an
  AABB/OBB disagreement at exactly `EPSILON_F64`, `+Inf` from finite inputs, NaN laundered into a
  plausible `t`, and negative half-extents making one OBB both solid and empty.
  → `issues/archived/2026-08-10-slab-parallel-test-is-scale-free.md`
- **`geo_ray_capsule`'s `cyl_missed` fallback applies no hemisphere test**, so it can return a cap
  *sphere*'s entry root that lies inside the capsule. Axial ray from inside: `t = 4`, hit point
  distance to the segment **0.0** against `r = 1`. Plus 5.3% of random interior origins losing a
  cap exit entirely. → `issues/archived/2026-08-10-capsule-cyl-missed-tags-a-sphere-root-as-a-cap.md`
- ⚠ **Two rows of this release's own squared-epsilon disposition table were wrong**, and are
  corrected in place. They were sorted by reading the code, not running it.
  `geo_triangle_unit_normal` was called "a degeneracy policy"; it returns a **fabricated**
  `(0, 1, 0)` for a well-formed right triangle with 1e-7 legs where the truth is `(0, 0, 1)`. The
  capsule's degenerate guard was called immaterial; it is **25.6%** wrong on a 1e-7-long capsule.
  *A filing is not evidence until something has re-run it — and a disposition is a filing.*

### Performance

- **`jet_obb` 1.083 µs → 872 ns (−19.5%)**, by rotating one unit vector instead of calling
  `geo_obb_axes` (which rotates all three and allocates a 24-byte array). Bit-identical result —
  it is the same `hquat_rotate_vec3` call the primal makes. Takes the jet from 2.2× its primal to
  **1.8×**.

- **Cost of the split, measured per primitive** (interleaved A/B, three runs each,
  `bench_batch(2000, 200)`):

  | | with the split | pre-split | |
  |---|--:|--:|---|
  | `ray_aabb` (axis-aligned) | 91.7 ns | 86.7 ns | **+5.7%** |
  | `ray_aabb` (diagonal) | 113.7 ns | 107.7 ns | **+5.6%** |
  | `ray_obb` | 464 ns | 460 ns | +0.9%, inside noise |
  | `ray_capsule` | 968.7 ns | 977 ns | inside noise |

  The AABB pays for it: it is the cheapest of the three, so the wrapper call and one i64 compare
  per slab are visible. **Stated as real rather than dismissed as noise** — it sits near the
  release's 3.5% median noise floor, but A was higher in 3 of 3 interleaved pairs.

- **What a gradient costs**, each jet against its own primal at the same fixture:

  | | primal | jet | ratio | partials |
  |---|--:|--:|--:|--:|
  | sphere | 97 ns | 355 ns | 3.7× | 7 |
  | aabb | 116 ns | 354 ns | **3.1×** | 12 |
  | triangle | 295 ns | 992 ns | 3.4× | 15 |
  | obb | 499 ns | 868 ns | **1.7×** | 12 |
  | capsule | 965 ns | 1.328 µs | **1.38×** | 13 |
  | plane | — | 240 ns | — | 7 |

  The branchy three are *cheaper* relative to their primals than the smooth three, because the
  primal work they reuse is larger. Forward-mode duals would need 12–13 evaluations for these.

  ⚠ **Two fixture faults were found while building this table, and both are the kind that make a
  benchmark lie rather than fail.** `ray_aabb`'s shipped fixture fires along `(0,0,1)`, so **two of
  its three slabs are skipped as parallel** — it has always measured the cheapest path the routine
  has (the ear-prune mistake of 2.9.3, in a different function). And `ray_capsule` fires down the
  capsule axis onto a **cap** while `jet_capsule` fires diagonally onto the **barrel**: dividing
  one by the other would have reported the jet at 2.6× when it is 1.38×. Both axis-aligned rows are
  **kept** so the CSV history stays comparable, and a diagonal row added beside each. It is the
  pair that is honest, not either row alone.

### Verification

- **Suite 3398 → 3453**, 0 failures. Every jet is checked against **central differences of the
  shipped primal**, so it is verified against the function it claims to differentiate.
- **Twenty-five mutants written, twenty-four killed**, the survivor being the dead-code finding
  above. Sign flips, missing `t` factors, swapped min/max and endpoint weights, wrong axis, wrong
  cap endpoint, dropped tie checks, exit paths reporting the entry face.
- **Fixture discrimination is asserted, not assumed.** `_JA_FACES == 0x3F` and `_JO_FACES == 0x3F`
  prove the box sweeps hit all six faces; `_JC_BRANCH == 0x7` proves the capsule sweep hit the
  barrel **and both caps** — the caps being where 2.10.0's defect lived. The OBB sweep uses a
  **rotated** box, because under the identity quaternion the local axes are the world axes and an
  axis mix-up is invisible.
- **The capsule seam is C¹, and that is measured rather than argued.** The first seam assertion
  used `dz = 1e-7` against a 1e-8 tolerance and **failed** — correctly: the true gap there is
  6.7e-8. Rather than loosen the tolerance, the gap was measured at eleven offsets:

  | dz | 1e-4 | 1e-6 | 1e-8 | 1e-10 | 1e-13 |
  |---|--:|--:|--:|--:|--:|
  | gradient gap | 1.7e-4 | 1.7e-6 | 1.7e-8 | 1.7e-10 | below print precision |

  One decade per decade — linear convergence, therefore C¹. The assertion is now a **rate** check
  (a 100× smaller offset must give a ≥50× smaller gap), which cannot be satisfied by loosening a
  tolerance and which a merely-C⁰ surface fails. A box edge fails it, which is why the box jets
  return the miss sentinel there and the capsule does not.
- **Edges and corners** return the miss sentinel from the box jets, with the primal asserted to
  still return `t = 1` so the test proves the jet is refusing a *real* hit. Those fixtures use the
  identity rotation deliberately — an **exact** floating-point tie needs axis-aligned arithmetic;
  a rotated box ties only approximately, which is a near-edge hit and *is* differentiable.
- **Wrapper equivalence** asserted bit-for-bit for all three primitives.
- **An independent derivation and adversarial probe were commissioned against the finished code**,
  and this is what they returned. **All 37 partials agree term for term** with the shipped
  formulas, and **0 of 24,237 finite-difference comparisons failed** — with the oracle's teeth
  demonstrated separately by corrupting each analytic value and confirming every corruption is
  caught. The value was **entirely in the enumeration**: the formulas were right, and the list of
  degenerate cases was two short (above). A test written by the author of the code inherits the
  author's list of cases. **Ask what is missing from the list, not whether the entries on it pass.**
- **Two 2.10.0 entry points had no caller anywhere** — `geo_jet_plane_ddistance` and
  `geo_jet_dpoint`, 2 of the 5 functions in that release's 599/604 coverage gap. Both now have
  real finite-difference checks (`geo_jet_dpoint` against the motion of the hit **point**, on a
  sphere rather than a plane so the `dt` term is load-bearing), plus a decode check on
  `geo_face_axis`/`geo_face_is_max`. Coverage **599/604 (99%) → 617/620 (99%)** over 36/36 files.
  Reference coverage is a floor, not a correctness proof — but a public entry point with no caller
  has not cleared even the floor.

## [2.10.0] - 2026-08-10 — differentiable geometry, and the primal defect it found first

The roadmap's first genuine feature line since 2.8.x: **autodiff through ray–surface intersection**,
so consumers can optimise *through* geometry rather than around it. `src/geo_diff.cyr` ships jets for
plane, sphere and triangle — the full gradient from one evaluation, allocation-free, as a post-pass
on the shipped primal. Suite **3376 → 3398**, benchmarks **60 → 64**, 34 → **35** `[lib]` modules,
toolchain 6.5.16 → **6.5.17**. All gates green.

**Read the order of events before the list.** No autodiff code was written until the design's own
correctness check had been run against the *existing* primitives — and it failed. `geo_ray_sphere`
was not degree −1 homogeneous in the ray direction, so it returned a `t` whose hit point was not on
the sphere, and `geo_ray_capsule` inherited it. **That is a silent wrong answer in shipped geometry,
found by a check written to validate a feature that did not exist yet.** The jets could not have
been correct on top of it: the analytic `dt/dd` is derived from an implicit-function argument that
assumes exactly the homogeneity the primal broke.

The same pass found the benchmark harness measuring `clock_gettime` rather than geometry on 17 of 60
rows — which is the only reason the repair's +17.6% cost was visible at all.

### Added

- **`src/geo_diff.cyr` — differentiable ray–surface intersection (2.10.0, first bite).**
  `geo_jet_sphere` and `geo_jet_plane` return the hit parameter **and its full gradient** from one
  evaluation: `dt/d(origin)`, `dt/d(direction)`, and the shape parameters (`dt/d(center)`,
  `dt/d(radius)`; `dt/d(normal)`, `dt/d(distance)`). Plus `geo_jet_dpoint`, the chain rule through
  `geo_ray_at` for the hit *point* rather than the parameter.

  **Not dual numbers, and that was a measured decision.** `Dual` is a heap struct and every
  `dual_*` op allocates 16 bytes under an allocator that never frees, so threading duals through a
  96 ns intersection would leak dozens of objects per call; and forward mode carries one seed per
  evaluation, so a sphere's 7 parameters would need 7 passes. The implicit function theorem gives
  the whole gradient in closed form instead — `t` is the root of `F(o + t·d; θ) = 0`, so
  `dt/do = −g/(g·d)`, `dt/dθ = −(∂F/∂θ)/(g·d)`. Every entry point is a **post-pass on the shipped
  primal**: it calls the same `geo_ray_*` a non-differentiating caller would, so there is no second
  copy of any intersection algorithm that could drift from the one under test.

  **Cost, measured**: `jet_sphere` **336 ns** against `ray_sphere`'s **95 ns** — **3.5×** the primal
  for all seven partials, versus the seven full evaluations forward-mode duals would need.
  `jet_plane` is 159 ns. Both are new rows in `bench-history.csv`.

  **Verification** — the oracle is central differences of the *shipped primal itself*, so the jet is
  checked against the function it claims to differentiate rather than a reimplementation that could
  share a mistake with it. 16 ray/sphere configurations × 10 partials. **Five mutants caught**: a
  sign flip on `dt/do`, `dt/dd` missing its `t` factor, a `dt/dcenter` sign flip, `dt/dradius`
  dropping its radius factor, and a typed accessor that stops checking `kind`.

  ⚠ **The radius mutant initially SURVIVED**, because the fixture used radius 1 — where
  `dt/dr = r/(g·d)` is indistinguishable from `1/(g·d)`. Two radii, neither 1, now make the factor
  load-bearing. That is the third time this release a fixture looked like it was testing something
  it was not; it is recorded inline beside the fixture.

  Two conventions stated rather than assumed: `dt/dd` is the **raw** partial (well-posed only
  because the homogeneity repair below landed first), with `geo_jet_dt_dd_unit` offered for a
  unit-constrained caller rather than projecting silently — projecting would make `d·(dt/dd)` read 0
  when the true value is `−t`. And the shape-derivative slots are reached **only** through typed
  per-primitive accessors that check a `kind` tag, so reading a sphere's radius slot off a plane jet
  returns the error sentinel instead of that plane's distance derivative.

- **`geo_jet_triangle` — 15 partials from one intersection.** `dt/d(origin)`, `dt/d(direction)` and
  all three vertex gradients. The surface is the triangle's plane, so the barycentric bounds decide
  only *whether* there is a hit, not where — they contribute nothing to the interior derivative, and
  on the boundary `t` is not differentiable at all, which the primal's miss return already expresses.
  Both `n` and the constant term depend on the vertices, so with `w = p − v0` and `den = n·d`:
  `dt/dv0 = −[(e1−e2)×w − n]/den`, `dt/dv1 = −[e2×w]/den`, `dt/dv2 = −[w×e1]/den`. Verified against
  FD over 382 accepted configurations, worst residual **9.6e-10**. **Four mutants caught**: swapping
  `dv1`/`dv2`, dropping `dv0`'s `−n` term, reversing a cross product, and writing `e2−e1` for `e1−e2`.

  ⚠ The translation identity `dt/dv0 + dt/dv1 + dt/dv2 == −dt/do` **holds for any shared error in
  those three expressions** — the cross terms cancel algebraically to `n/den` — so it is asserted as
  a smoke test with that stated inline, and finite differences remain the real oracle. Recorded
  because it is exactly the kind of invariant that reads as strong verification and is not.

  **This completes 2.10.0's scoped smooth seam.** Costs, all `bench_batch`:

  | | primal | jet | ratio | partials |
  |---|--:|--:|--:|--:|
  | sphere | 93 ns | **335 ns** | 3.6× | 7 |
  | triangle | 280 ns | **934 ns** | 3.3× | 15 |
  | plane | — | **227 ns** | — | 7 |

  The triangle is the case that justifies the whole approach: forward-mode duals would need **15
  evaluations** for 15 partials — ~4,200 ns before counting a single allocation — against 934 ns for
  one jet. **4.5× faster and allocation-free**, which is the measured form of the argument the design
  made on paper.

  `aabb`/`obb`/`capsule` remain deferred: all three select their hit through `f64_max`/`f64_min` at
  12 sites and need a `_core(ray, shape, out)` split of shipped `geo.cyr` before a jet can hook the
  selected branch — not worth risking under 3398 assertions here.

### Fixed

- **`geo_ray_sphere` returned a `t` whose hit point was not on the sphere, whenever the ray
  direction was not unit-length — and `geo_ray_capsule` inherited it.** Four of the six `geo_ray_*`
  siblings honour the contract they share (*"the hit point is `o + t·d`"*, which forces `t` to be
  degree −1 homogeneous in `d`); two did not. Measured by scaling a unit direction by `k` and
  comparing against `t/k`:

  | primitive | `t(2d)` | expected `t/2` | |
  |---|--:|--:|---|
  | `geo_ray_sphere` | 2.2872117 | 3.8301270 | ❌ |
  | `geo_ray_capsule` | 2.1539078 | 3.7177546 | ❌ |
  | `plane` / `triangle` / `aabb` / `obb` | — | — | ✅ all four |

  Not merely a different number: for a **unit** sphere at `(2,4,6)` and `d = (1,2,3)` it returned
  `t = 1.0`, putting `geo_ray_at(ray, t)` **3.741657387 from the centre**, silently. Root cause —
  the reduced half-`b` quadratic drops `a = d·d` from both the discriminant and the division, which
  is valid only for a unit direction. `geo_ray_new` normalizes so the intended path was safe, but
  `GeoRay` is `#derive(accessors)`, `GeoRay_set_direction` is public, and no unit-length
  precondition was documented anywhere. Same sibling-asymmetry class 2.9.1 repaired in
  `mpr_intersect` vs `gjk_intersect_3d`, and resolved the same way rather than by documenting it.

  ⚠ **One root cause, not two.** `geo_ray_capsule`'s own cylinder quadratic was *already* correct
  (it carries `a` and divides by `2a`); it failed only because its end-cap branches call the sphere.
  Repairing the sphere alone took the sweep 5 bad → 0.

  Fix: carry `a` explicitly through discriminant and roots, plus a degenerate-direction guard.
  **Cost, measured** — interleaved A/B, three runs each: `ray_sphere` **81.7 → 96.3 ns, +17.6%**,
  non-overlapping, <4% spread; `ray_triangle` unchanged as control. Accepted: 14.6 ns is the price
  of the primitive agreeing with its own contract. ⚠ **Only measurable because the benchmark harness
  was fixed first** — on the old one this sat inside 1,460 ns of clock overhead.

  Found by running 2.10.0's own design check (Euler's relation `d·∂t/∂d = −t`) against the
  **shipped** primitives before writing any autodiff code. It blocks the jets: the analytic `dt/dd`
  is derived from an implicit-function argument that assumes exactly the homogeneity these two broke.
  → `issues/archived/2026-08-10-ray-sphere-capsule-assume-unit-direction.md`

- **17 of 60 benchmarks were measuring `clock_gettime`, not the operation.** `bench_run` wraps a
  `clock_gettime` PAIR around **every call** — ~240 ns, documented in `lib/bench.cyr` — and
  `bench_batch` exists precisely to amortise that over a round. Only 7 benchmarks used it. The other
  53 included 17 sub-microsecond operations whose recorded numbers were **~95% overhead**:

  | benchmark | recorded via `bench()` | true, via `bench_batch()` |
  |---|--:|--:|
  | `ray_sphere` | 1,466 ns | **79 ns** |
  | `ray_aabb` | 1,454 ns | **82 ns** |
  | `ray_triangle` | 1,741 ns | **286 ns** |
  | `vec3_add` | 1,457 ns | **36 ns** |
  | `num_gcd` | 1,445 ns | **27 ns** |

  ⚠ **The damage is not the inflation, it is the FLATTENING.** All 17 recorded between 1,445 and
  1,765 ns — a 1.2× spread across operations whose true costs span **27 ns to 287 ns, over 10×**.
  `ray_triangle` vs `ray_sphere` measured **1.19×** when the true ratio is **3.6×**. A real
  regression of several hundred ns in any of them would have been invisible inside the overhead, and
  the tell was in the CSV all along: `min_ns` pinned at 419–489 ns on every one of the 17, the clock
  floor rather than anything about the code.

  All 17 moved to `bench_batch` (2,000 calls per clock pair, 200 rounds). **Their drop at this date
  is an artefact of the fix, not a speedup**, and `benchmarks.md` now carries a dated
  *Measurement changes* header saying so — emitted by the generator, so it cannot be lost on the
  next regeneration.

  This is the **fourth** gate in this family: `bench-history.sh` recording MAX, the version gate
  matching a byte count, the CLI version string checked by nothing, and now the benchmark harness
  measuring its own instrumentation. Found while scoping 2.10.0's benchmark plan — no honest
  performance claim about ray intersection was possible until it was fixed.

### Changed

- **Toolchain pin `6.5.16` → `6.5.17`.** Upstream fixes **all three** defects hisab filed during the
  2.9.2/2.9.3 work. Each was re-run from its own filed repro rather than taken from the release note:

  - **Capturing closure SIGSEGV across a function boundary — FIXED.** The case-3 program returns
    **42** (was SIGSEGV 139), and the `fncall1` variant returns 42 too, so both dispatch paths are
    repaired. **This unblocks `vec_sort_by`**: it was parked first on a false premise ("Cyrius has no
    closures") and then correctly on this defect, and a capturing comparator now solves the
    API-shape objection — it can close over the `points` vector instead of reaching it through a
    file-scope global.
  - **`distlib` rejecting correct bundles — FIXED.** `cyrius distlib` on hisab's real bundle exits
    **0**, and the three-arm minimal reproducer passes on all of stdlib function / global var / enum
    constant. **hisab's CI and release workaround is removed** — both workflows run a bare
    `cyrius distlib` again and a non-zero exit is a real failure once more. The separate
    `cyrius check --with-deps dist/hisab.cyr` step stays; it was worth having independently.
  - ⚠ **Dead-function bodies never syntax-checked — PARTIALLY fixed.** `cyrius build` and
    `cyrius check --with-deps` now both reject the repro (exit 1), which is the half that matters:
    nothing unparseable can be built or shipped. But **`cyrius lint` and `cyrius vet` still exit 0**
    on a file that does not parse, so the release note's "accepted by every gate" is not fully
    discharged. Measured, fed back to the upstream filing, and left open here rather than closed.

  **No vendored stdlib change** — all 30 files are byte-identical between 6.5.16 and 6.5.17 for
  hisab's declared subset, so this is a compiler-only bump. Suite holds at **3376**, all gates green.

## [2.9.3] - 2026-08-10 — the open filings, and the two whose own proposed fixes were wrong

Repair of everything left open in `docs/development/issues/`. **Five of the six internal filings are
closed**; the sixth is re-diagnosed and correctly open. Suite **3351 → 3376**, benchmarks
**55 → 58**, constant gate 153 → **155**, toolchain and deps unchanged from 2.9.2. All gates green.

**Read the shape of this release before the list.** It was scoped as "work the filings", and the
filings turned out to be the thing that needed auditing. **Two of the six argued for repairs that
measurement refuted** — `incircle-precision`'s implied fix (an exact determinant of *pre-differenced*
operands, which is exact arithmetic on information that is already gone) and `epa-certificate`'s
stated fix (drop the seed test, which produces wrong depths at exact tangency). Both were implemented
in full before being rejected. A third, `epa-certificate` again, rested on a measurement that had
only ever looked at one of the two entry points it was about. **That is the 2.6.x lesson pointed at
the record instead of the code**: a filing is a claim, and a claim nothing has re-run is not
evidence.

Two defects of consequence came out of it — `delaunay_2d` silently dropping input points on any set
that mixes scales, and `_col_dl_incircle` answering differently depending on vertex order — plus
three guards that were policing nothing.

### Fixed

- **`delaunay_2d` silently dropped input points on any set that mixes scales** — a wrong result from
  the public entry point, not the "latent, no reachable path" the filing had recorded. The winding
  term of `_col_in_circumcircle` was a plain float 2x2 determinant; with one vertex at ~6e5 and five
  points inside ~5e-17 of the origin it loses its sign entirely. Measured fixture: 5 of 6 input
  points used, **4 triangles where the exact rational hull says 6**. Failing seeds are dense — 8 of
  the first 20 on a 3,999-seed sweep. Absolute magnitude is not the trigger (translating a whole set
  to 1e9 is fine, Sterbenz handles it); **ratio within one set** is.

  Root cause → fix: `_col_orient2d_sign`, an adaptive exact `orient2d`. Fast path is the float
  determinant, returned whenever it clears `(3 + 16ε)ε · (|l| + |r|)` — every ordinary call. Exact
  path is Shewchuk's `orient2dexact` shape: six products of the **raw** coordinates, three
  `Two_Two_Diff` expansions, two `fast_expansion_sum_zeroelim` merges, sign read off the most
  significant component.

  ⚠ **The obvious version of this fix does not work, and it was implemented first.** Making the
  determinant exact while leaving the operands **pre-differenced** changes nothing: the information
  is gone before the exact arithmetic starts. With `b`, `c` at ~1e-17 and `a` at ~4e5, `f64_sub(bx, ax)`
  and `f64_sub(cx, ax)` round to the **same double**, so two distinct points become one and the exact
  determinant of *that* question is a faithful `0` — against a true `-5.457401e-11`. The exactness has
  to extend back **through** the subtraction, which is why this expands into six raw products instead
  of differencing first. That first attempt was discarded, not shipped.

  Validated against exact rationals on 300 random mixed-scale triples: **300/300 correct**, where the
  shipped float winding scores **0/300**. Three mutants caught. ⚠ **One survives and is recorded**:
  feeding the function already-translated operands passes the whole suite, so nothing here would catch
  a future change moving it back — two searches for a distinguishing case (2M mixed-magnitude, 3M
  near-degenerate at 1e15..1e17) found none, because a uniform shift by `p` moves all three vertices on
  the same lattice and the roundings cancel.

  **No cost measurable**: `delaunay_2d_400` −3.5%, `delaunay_2d_circle_150` −5.9%,
  `triangulate_600gon` −6.4%, median −3.1% across 55 benchmarks, **0 above the +10% noise floor**.
  The negative signs are noise; no improvement is claimed.

- **`_col_dl_incircle` was CCW-only, because `_col_dl_ic_g1` never formed the triple's winding.**
  "Is p inside the circumcircle of these three points" is a question about a **set**, so an answer
  that depends on the order the vertices arrive in is wrong however the caller happens to store
  them — the same defect class as the `mpr_intersect` operand-order asymmetry repaired in 2.9.1.
  `_col_dl_ic_g2` already divided its winding out and `_col_dl_ic_g3` became the constant `1` in
  2.9.1; `g1` was the last of the three still reading the CCW storage convention instead of the
  geometry. Root cause → fix: `g1` now computes `w = sign((b − a) × u_k)` and returns
  `sign((a−p) × (b−p)) · w > 0` (`collision_mesh.cyr:255`).

  ⚠ **The obvious form of that fix is wrong on the parallel edge, and the first draft shipped it.**
  `w` is 0 when the edge is parallel to `u_k`, which reads as degenerate but is not: `G_k = A + M·u_k`,
  so such a triangle has area `|(b − a) × (A − a)| / 2` — **constant in M**. The third vertex
  recedes parallel to line(a, b) at a fixed offset and the limiting circle is the half-plane on the
  **apex's** side, so `w` falls back to `sign((b − a) × (A − a))` with `A = points[0]`, the apex
  convention `_col_dl_ic_g2` already uses. Only when both vanish are the points collinear for every
  M, and `0` there is a convention (order-free) rather than a derived answer. The draft was caught
  by the 2.9.1 finding-2 reproducer, which *is* a parallel-edge case.

  Validated against an **order-free** exact oracle — circumcentre in `Fraction`, `|p − O|²` vs `r²`,
  ghosts at `A + 10³⁰·u`, a formulation that takes no orientation argument and so cannot inherit a
  convention from the code under test: **4606 of 4606 correct**, including **1110 parallel-edge
  probes**. (An earlier oracle attempt used the in-circle determinant's raw sign and reported 2219
  failures — that oracle was itself order-dependent, i.e. it was measuring the thing under test.)

  **No reachable behaviour changed**: `delaunay_2d` stores every triangle CCW, so the factor is `+1`
  on every call it makes. Suite **3351 → 3359**; `delaunay_2d_400` −2.9%, `halfedge_2k_tris` −4.6%,
  median across all 55 benchmarks −1.4% — **no cost measurable above the noise floor**, and none
  claimed. Closes `issues/archived/2026-08-06-ghost-incircle-g1-g3-assume-ccw.md`, now archived.

### Added

- **`einsum` had no benchmark at all, and `tests/hisab.bcyr` did not even include the module.** It is
  the library's **only arena consumer** — `arena_new`, `arena_reset` and 11 `arena_alloc` calls, all
  in `src/einsum.cyr` — so no benchmark in the suite touched the arena path. The gap surfaced at the
  2.9.2 toolchain bump: measuring hisab's exposure to cyrius 6.5.10's allocator change needed a
  purpose-built micro-benchmark, because no existing row would have shown a regression there.
  `einsum_matmul_2x2` (**2.53 µs**) and `einsum_trace_2x2` (**1.89 µs**) close it; the notation `Str`s
  are built once at setup so the rows measure `einsum` rather than `str_from`. Benchmarks 58 → **60**.

- **A benchmark for the input class the k-d balance guard exists for.** `kdtree_build_4k` is a
  uniform scatter — the guard never fires on it, so its cost and its absence were both invisible.
  `kdtree_build_octave_512` (every coordinate halves, so the value-midpoint sits at ~0.5 with exactly
  one point above it) makes the guard load-bearing, and now measures what it is worth:
  **564.8 µs with the guard, 3.397 ms without** — **6.0×**. The filing's BENCH section asked for this
  class in 2.7.0 and it never shipped; the guard has been unmeasured on its own use case since.

- **Two benchmark classes the `triangulate_polygon` prune had never been measured on.** The only
  guard was `triangulate_600gon`, a polygon of 600 points on a **circle** — strictly convex, i.e. the
  reflex-only ear prune's *best* case, where every vertex is convex and the containment scan is
  skipped every time. The filing that proposed the prune asked for a reflex-heavy guard; it never
  shipped, so a −84% headline has been policed by its own best case for three releases. Now:
  `triangulate_comb_600` (alternating radius, so half the vertices are reflex) at **5.718 ms**
  against the convex case's **1.593 ms** — **3.6× the cost**, on the same n. And
  `triangulate_hex_6`, the size class every shipped call site actually uses.

  **The small-n regression is real, and it is measured here rather than inherited.** A/B against the
  pre-prune body recovered from git (`97bed43^`), same machine, same toolchain, three runs each:

  | case | pre-prune | with prune | |
  |---|--:|--:|---|
  | `tp_hex6` (n = 6) | 2.556 / 2.596 / 2.577 µs | 2.956 / 2.976 / 2.963 µs | **+15%** |
  | `tp_600gon` (n = 600) | 9.556 ms | 1.691 ms | **−82%** |

  The distributions do not overlap and the spread within each is under 2%, so the +15% is signal.
  The prune allocates an n-entry `blocked[]` array and refreshes neighbour convexity per ear, which
  the naive scan does not; below break-even that setup cannot amortise. **The −84% figure in the
  2.7.1 entry stands** (it reproduces at −82% on 6.5.16) — what was missing was the other half of the
  sentence, and both classes are now in `bench-history.csv` so neither can move unobserved.

- **`_dlg1_run` / `_DLG1_BAD`** (`tests/modules.tcyr`) — 1800 probes over 4 edges × 25 query points
  × 3 ghost directions × 6 orderings, asserting every ordering of a one-ghost triple agrees. The
  pre-2.9.3 body scores **846 of 1500** ordering comparisons. This replaces the filing's Python
  transcription with a first-party assertion, which the filing itself said was missing.

  ⚠ **Order-consistency alone was not enough, and a mutant proved it**: negating the apex fallback's
  sign flips every ordering together, so the grid stayed green. Four correctness assertions were
  added, pinning two apexes on **opposite sides** of a `u_0`-parallel edge to opposite answers with
  values from the exact oracle. Four mutants are now caught — dropping the winding, treating the
  parallel edge as degenerate, flipping the apex fallback, and never consulting `u_k` (27 failures).

- **The `_kd_partition` balance guard had no regression assertion.** The two tests standing next to
  it — `kdtree_build != 0` and the split-plane invariant — both hold perfectly well on a degenerate
  one-point-per-level spine, so **deleting the guard broke nothing that was being checked**
  (measured: the whole suite stays green with the guard replaced by `if (1 == 1)`). Depth is the
  property the guard provides, so depth is now what is asserted.

  ⚠ **The obvious fixture does not exercise it, and asserting on that one would have re-created the
  same hole one line down.** The existing geometric fixture has ratio 0.99998, so over 8000 points
  the axis range is only `[0.85, 1]` and the value-midpoint lands near the count-median anyway — the
  guard never fires. What degenerates the split is a range spanning many *orders of magnitude*: with
  `v = 2^-i` the midpoint is ~0.5, exactly one point sits above it, and the build peels off a single
  point per level. New octave-spaced fixture (512 points, all three coordinates decaying, because
  the axis cycles 0,1,2 and one degenerate axis is diluted by two healthy ones); `log_(4/3)(512)` is
  about 22, unguarded it builds a ~512-deep spine. Mutation-proven.

- **`_col_point_in_tri` documented itself as "(2D, strict interior)" and is boundary-INCLUSIVE.** A
  point on an edge makes that edge's signed area exactly 0, which sets neither `has_neg` nor
  `has_pos`, so "all signs agree" holds and it returns 1; vertices likewise. **The code is right** —
  ear clipping must not cut an ear when another vertex lies *on* its boundary, only when it lies
  strictly outside — so only the contract was mis-stated. Comment corrected and the closed contract
  pinned by five assertions (`collision_core.cyr:587`).

### Changed

- **The EPA certificate filing's central claim is false, and both of its candidate repairs are now
  measured and rejected.** `2026-08-06-epa-certificate-tests-the-seed-not-the-polytope.md` argued
  that EPA's certified exit "is never taken", that `_epa_polish` is therefore the primary path, and
  that this explains why six EPA repairs were unmutatable in 2.9.0. **It only ever measured one of
  the two public entry points.** On the filing's own 12 box fixtures: `gjk_epa_3d` falls back 12 of
  12, `mpr_penetration` falls back **0 of 12** and certifies every one. The exit is live; the real
  defect is a **certification asymmetry**, and its cause is the seed — `_epa_seed_portal` tries a
  strictness upgrade (`geo_advanced.cyr:757-766`) and `_epa_seed_gjk` does not.

  - ⛔ **The filing's Proposed fix — drop the seed test, keep `lo > 0` — produces wrong depths.**
    Boxes go 12 → 0 fallbacks as predicted, but **four exact-tangency pairs then report a nonzero
    depth for a touch whose true depth is 0** (`_ag_baddepth` 0 → 4 in the 110-pair agreement
    sweep). At exact tangency `lo` can be marginally positive from rounding; the seed test was what
    caught it. **The seed test is load-bearing.**
  - **The cheaper alternative works but is not obviously worth it.** Giving `_epa_seed_gjk` the
    portal's upgrade passes every assertion, including every exact-MTV depth on both entry points,
    and takes boxes 12 → 6 — at **+19.4% on `gjk_epa_sphere_box`** (150.4 µs vs 125.6 µs, three runs
    each, non-overlapping, against a ~7% noise floor). Spheres pay the extra support evaluation and
    still fall back, which points at a separate root cause in that family. **No returned answer
    changes either way**, so this buys internal consistency and mutatability, not correctness.

  **The code is unchanged**, the filing is re-diagnosed and re-titled, and its severity is downgraded
  Medium → Low. The suite's assertions are rewritten to state the asymmetry — the `mpr_penetration`
  half was never measured before — rather than the "unreachable path" reading.

- **`2026-08-04-incircle-precision.md` was closed as "leave it" in 2.9.1 on a premise that does not
  hold.** The 0-of-1,528,820 sweep behind that call was real, but every one of its four fixtures
  draws from a **single origin-anchored box**, so the family has no *intra-set* dynamic range and
  structurally could not reach this. Filing re-opened, fixed (see **Fixed** above), and archived.

  **The suite's own oracle was blind to it by construction**: `_dl_violations` decides
  circumcircle-emptiness by calling `_col_in_circumcircle`, i.e. the code under test. Only structural
  oracles (`_dl_used`, `_dl_max_edge_share`) can see this class, and the two repro `.tcyr` files in
  `issues/` are not under `tests/`, so CI has never run them.

- **Three "NOT APPLIED, by decision" performance records were wrong on all three counts**, in both
  `doc-health.md` and `roadmap.md`. `_kd_partition` and `triangulate_polygon` **shipped in 2.7.1**
  (a balance guard rather than the spec's unconditional quickselect; reflex-only ear pruning at
  9.93 → 1.59 ms), and `delaunay_2d`'s record was **superseded by the 2.8.0 adjacency rewrite**,
  which deleted the O(n²) per-point rescan it proposed removing — so it is archived, with a note
  that it is **not** the reason `delaunay_2d` is fast and must not be cited as one. The lumped
  three-filing ledger row is split into three accurate ones. What was genuinely open in that group
  was never the code: it was the missing guard assertion and the false predicate contract above,
  plus two documentary items carried forward (the −84% headline does not disclose a small-n
  regression below the prune's break-even, and no reflex-heavy benchmark guards the prune).

- **Toolchain filings reconciled with the upstream tracker, which nobody had checked.**
  `2026-04-26-cyrius-for-empty-clauses` is **archived**: upstream closed it **WON'T-FIX, by design**
  on 2026-06-02 (cycc v6.0.36) — Cyrius requires all three `for` clauses — so hisab had carried it as
  "Open" for five releases and re-verified it six times against a documented language rule. The
  `while` rewrites in `collision_core`/`collision_mesh` are permanent, not provisional.
  `2026-04-26-cyrius-cli-arg-clobbers-source` now records that its upstream copy sits in `archived/`
  **without** a resolution note, a fix version, or an index row — i.e. swept, not fixed — so its
  location must not be read as evidence the data-loss bug is gone.
- **The two 2.9.2 toolchain defects are filed upstream**, in that repo's required format:
  `cyrius/docs/development/issues/hisab-distlib-selfcheck-rejects-stdlib-globals.md` (High) and
  `hisab-dead-fn-bodies-never-syntax-checked.md` (Medium), the latter with a standalone repro at
  `issues/repros/2026-08-09-dead-fn-body-not-parsed.cyr`, re-run from the filed file itself.

## [2.9.2] - 2026-08-09 — the toolchain bump, and three gates that were not gating

Maintenance release: toolchain pin **6.5.9 → 6.5.16** (seven releases) and sakshi
**2.4.8 → 2.4.10**. **No library source change** — all 34 modules compile clean, the `dist/hisab.cyr`
diff is the version header and nothing else, and the suite holds at **3351** across five harnesses
(hisab 416, foundation 349, modules 1621, edge_cases 233, abuse 732). Constant gate **153/153**,
`deps --verify` **30/30**, lint 0 warnings and `fmt --check` clean across all 43 sources, vet 2 deps
/ 0 untrusted / 0 missing, fuzz 1/0, `cyrius coverage` **588/591 fns (99%)** over 35/35 files.

**The bump itself was uneventful. What it surfaced was not.** Crossing 6.5.14 turned `cyrius
distlib` red on a correct bundle, and chasing that led into the benchmark harness, where the CSV
that CLAUDE.md designates as the proof for every performance claim turned out to be recording the
single worst iteration of each benchmark. Three gates in this repo were green while checking
nothing, and none of the three was found by a test failing.

### Fixed

- **`scripts/bench-history.sh` recorded the MAX, not the average — every row it has ever written.**
  Its parser took "the last `<num><unit>` in the line"; `cyrius bench` emits
  `name: <avg> avg (min=<min> max=<max>) [N iters]`, so the last token is `max=`. The header comment
  above it described a `<median> ... (per iter)` format that **does not exist**, which is how the
  heuristic looked reasonable. Measured: **44 of 55** rows in the most recent run track max rather
  than avg.

  **Why it matters is a noise argument, not a naming one.** Three back-to-back runs of the *same
  binary* put the `avg` field's spread at a median of **3.5%** (worst 6.9%; **0 of 55** benchmarks
  above 20%) — so the harness is precise enough to detect real regressions. The max field is a
  single worst sample and is not: consecutive max-based CSV rows swung **−93% to +742%**, and in one
  run `srgb_to_linear` reported **avg 1,438 ns against max 48,959 ns — a 34× ratio inside one
  measurement**. A regression baseline built on that cannot distinguish a 2× slowdown from a
  scheduler hiccup.

  Root cause → fix: the parser now **anchors on the harness's named fields** instead of scavenging
  tokens, and records `avg` in `estimate_ns` while also carrying `min_ns` / `max_ns` / `iters`. A
  line that does not match the expected shape is now a **hard error** rather than a silent skip, so
  an upstream format change cannot quietly halve the recorded benchmark count again. A new `stat`
  column names which statistic each row holds; pre-2.9.2 rows have none and are `max`. Nothing was
  deleted — but `benchmarks.md` now builds its trend **only from rows sharing the newest run's
  `stat`**, because comparing an avg against a max produces hundreds of percent of pure artefact.
  The trend's significance threshold also moved **±3% → ±10%**: ±3% sits *below* the measured noise
  floor, so roughly half the table lit up on a rebuild that changed nothing.

  ⚠ **No performance claim in this release.** The one cross-run comparison available (2.9.1's
  recorded run vs this one) confounds three variables at once — different commit, different
  toolchain, and different statistic — so it is not quoted as a result. 2.9.2 establishes the first
  *correct* baseline; the next release is the first that can honestly compare against it.

- **CI's version-consistency gate passed vacuously.** `.github/workflows/ci.yml` ran
  `grep -q "$VERSION" CHANGELOG.md` — unanchored, and with the dots as regex wildcards. With
  `VERSION=2.9.2` that pattern matched **`32,942,104 B`** in an unrelated benchmark line at
  `CHANGELOG.md:498`. The gate would have gone green on a release whose CHANGELOG section was never
  written. Now `grep -qF -- "## [$VERSION]"`, anchored to the Keep a Changelog header. This is the
  same failure mode the `fetch-depth: 0` comment in the measurements job already warns about, one
  job further down the same file.

- **`src/main.cyr` hardcodes the version the CLI prints, and nothing checked it.** `VERSION` is the
  single source of truth and `cyrius.cyml` pulls it via `${file:VERSION}`, but Cyrius has no
  build-time string interpolation, so `println("hisab 2.9.1")` is a manual edit on every bump —
  which `scripts/version-bump.sh` did not make and no gate compared. Fixed three ways: the string is
  updated, `version-bump.sh` now rewrites it (and reminds the caller to re-run `cyrius distlib`), and
  CI asserts both it **and** the `# Version:` header inside `dist/hisab.cyr` against `VERSION`.

### Changed

- **Toolchain pin `6.5.9` → `6.5.16`; sakshi `2.4.8` → `2.4.10`.** Re-vendored via `cyrius lib sync`
  — all 27 declared-subset stdlib files byte-match the 6.5.16 snapshot. The transitive
  `lib/result.cyr` is not in the declared subset and `lib sync` does not touch it; it was
  hand-refreshed (its only delta is two doc-comment paths, from 6.5.11's test-suite subfolder
  reorg). `lib/atomic.cyr` was already identical. **All 30 vendored files now byte-match
  `~/.cyrius/versions/6.5.16/lib/`**; `cyrius.lock` 30 deps (1 commit-pinned), verify 30/30.

  ⚠ **A caveat on the standing vendoring rule.** 2.6.11 established "byte-compare `lib/` against the
  *pin's own* snapshot, not previous-pin vs new-pin". That rule assumes the snapshot directories are
  immutable, and on this machine they are not: `~/.cyrius/versions/6.5.9/lib/alloc.cyr` is
  byte-identical to 6.5.16's and carries `⚡ v6.5.10` annotations, i.e. the 6.5.9 tree was refreshed
  in place on 2026-08-07, after 2.9.1 shipped. The rule still holds for the *current* pin, which is
  the one that matters; but a "6.5.N snapshot" on disk is not evidence of what 6.5.N contained.

- **Vendored stdlib delta — 10 files** (plus the transitive `result.cyr`):
  - **`alloc.cyr`** — 6.5.10 inlined the two accessor loads in `alloc_via` / `realloc_via` /
    `free_via` / `reset_via` (`fncall2(allocator_alloc_fn(a), allocator_state(a), size)` →
    `fncall2(load64(a), load64(a + 32), size)`) and removed the `_arena_alloc` / `_arena_reset`
    shims, pointing `allocator_new` at `&arena_alloc` / `&arena_reset` directly.

    ⚠ **The obvious reading of this — "hisab grepped clean for `alloc_via`, so it is unaffected" —
    is wrong, and was written down here before it was checked.** hisab's own source names no `_a`
    symbol, but it does not have to: `str_from(cstr)` is `str_from_a(default_alloc(), cstr)`
    (`lib/str.cyr:35-37`) and `vec_new()` is `vec_new_a(default_alloc())` (`lib/vec.cyr:47-49`),
    and both `_a` forms go straight through `alloc_via`. `src/` has **152 call sites** across those
    wrappers (`str_from` 17, `vec_new` 44, `vec_push` 91), so **`alloc_via` runs constantly** —
    demonstrated by trapping it, which aborts 4 of the 5 suites. The inlining is therefore live for
    hisab. The *shim removal* is genuinely dead here: nothing outside `lib/` references
    `_arena_alloc` / `_arena_reset` / `allocator_new`, and the offsets the inlining bakes in
    (+0/+8/+16/+24 fn ptrs, +32 state) match `allocator_new`'s stores in the same file.

    Measured on hisab's own call shapes, interleaved 5× before/after with the compiler held at
    6.5.16 and only the lib tree swapped: `str_from` (1 `alloc_via`) **53/53/55/56/58 ns →
    50/51/51/52/52 ns**, `vec_new` (2 `alloc_via`) **103/103/104/105/107 ns → 97/98/98/99/100 ns**
    — non-overlapping distributions, roughly **4 ns per `alloc_via`**. Same direction as cyrius's
    own 15.1 ns / 5.1 ns figure, smaller in magnitude, and measured here rather than inherited.
    **No macro-level win is claimed**: a first pass suggested ~5% on the allocation-heavy
    benchmarks and three repeats reversed the sign. At 4 ns against benchmarks in the microsecond
    range, it is below this harness's noise floor.

    The lib delta on its own is **behaviourally inert**: same compiler, only the lib tree swapped,
    both trees produce byte-identical **3351/3351** across all five suites.
  - **`syscalls_linux_common.cyr`, `syscalls_aarch64_linux.cyr`, `syscalls_macos.cyr`,
    `syscalls_x86_64_agnos.cyr`** — the 6.5.15/6.5.16 macOS work (per-OS signal/errno constant
    splits, Mach-O syscall routes, sysctl-backed `uname`/`sysinfo`). Non-Linux-x86_64 variants,
    vendored for snapshot parity; no impact on hisab's build target.
  - **`io.cyr`, `args.cyr`, `fnptr.cyr`, `tagged.cyr`, `result.cyr`** — minor upstream changes; the
    calls hisab links behave identically, which 3351/3351 confirms.
  - **`sakshi.cyr`** — 2.4.10, byte-identical to what cyrius 6.5.16 folds into its own `lib/`.
    2.4.10's one behaviour change (`sakshi_log_kv` passes fields unflattened to an `SK_OUT_HOOK`
    subscriber) reaches nothing here: hisab declares sakshi but **calls it from no file in `src/`,
    `tests/`, or `examples/`**, and registers no emit hook.

  ⚠ The pin crossed **6.5.13, whose upstream CHANGELOG section is an empty header**, though it did
  ship a distinct `cycc` binary and one changed stdlib file (`syscalls_x86_64_agnos.cyr` — agnos
  only, never compiled by hisab). Recorded because the audit trail has a hole in it, not because
  anything is suspected.

- **`dist/hisab.deps` is now tracked** (removed from `.gitignore`, added to CI's drift check). A
  consumer's `cyrius deps` reads the `.deps` sidecar sitting next to a fold and auto-resolves the
  stdlib leaves it names (`cbt/deps.cyr:1494-1519`), so ignoring it meant impetus / kiran / joshua /
  aethersafha had to hand-declare all 15 of hisab's leaves and got no error when they under-declared.
  It was also worth nothing before **6.5.10**: the sidecar was built by scanning bundled sources for
  literal `include "lib/X.cyr"` lines, and hisab's `[lib]` modules are self-contained, so it reported
  **2** leaves. 6.5.10 unions in the declared `[deps] stdlib` — hisab goes **2 → 15**.

### Security

- **`cyrius distlib` fails its own bundle self-check on a correct bundle, and hisab's CI and release
  gates were both red because of it.** cyrius 6.5.14 repaired a self-check that had never once run
  (it compiled to `/dev/null`, whose `.tmp.<pid>` sibling cannot be created, so it failed on the
  write and printed a reassuring note). The repaired check compiles the bundle **alone** under
  `_cc_allow_undef = 1` — and that flag is read in exactly two places in cycc, both in the *fixup*
  stage, both gating `reachable undefined function(s)`. An unresolved **name** dies far earlier, in
  the frontend: `parse_expr.cyr:585-593` prints `undefined variable '…'` and calls
  `syscall(SYS_EXIT, 1)` immediately. So the suppression cannot reach it **by construction** — this
  is not an oversight about `var`s, it is a premise that does not hold for any name-resolved symbol.
  Confirmed with three one-line `[lib]` modules differing only in what they reference: a stdlib
  **function** exits 0, a stdlib **global var** (`F64_ONE`) exits 1, a stdlib **enum constant**
  (`STDOUT_FD`) exits 1.

  hisab's bundle reads `F64_ONE` at `dist/hisab.cyr:111` — the ordinary way to write `1.0` in a
  language with no float literals. Both `.github/workflows/ci.yml` and `release.yml` ran a bare
  `cyrius distlib` under `bash -e`, so **both aborted before reaching the drift check that would
  have passed.** Sandbox-tested across this machine's 69 repos carrying a `[lib]` block: **44 fail**
  once pinned to 6.5.16, of which 3 are pinned ≥ 6.5.14 today (hisab and majra broken; sigil passes
  only because its bundle happens to touch no consumer-supplied global).

  Both workflows now tolerate a non-zero `distlib` rc **only** when it carries that exact signature,
  and fail on any other distlib failure. `distlib` is write-then-verify — the bundle is written ~330
  lines before the check and is not unlinked on failure — so regeneration is unaffected; proven by
  the 2.9.1 → 2.9.2 header bump landing correctly on a run that exited 1. The coverage the broken
  self-check was meant to give is replaced by a **stronger** step, `cyrius check --with-deps
  dist/hisab.cyr`, which compiles the bundle with the stdlib actually in scope — the way a consumer
  builds it — with no category of error suppressed.
  → [`issues/archived/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md`](docs/development/issues/archived/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md)

## [2.9.1] - 2026-08-06 — the deferred tier: four latent predicate defects, and a fourth false measurement

Everything 2.9.0 found and consciously left. Suite **3341 → 3351** across five harnesses, toolchain
6.5.8 → **6.5.9**, all gates green.

**The theme is that four of these were found by deriving an oracle rather than by a failing test,
and a fifth was found by the gate built to catch exactly it.** None was reachable from a shipped
call path except one — `_col_dl_ic_g1`, which was wrong on ordinary CCW input.

### Fixed

- **`collision_core` — `mpr_intersect` and `mpr_penetration` were still running the pre-2.9.0
  containment test**, so both disagreed with their identically-documented `gjk_*` siblings on exact
  tangency. `mpr_intersect`'s fallback `_epa_seed_gjk != 0` is `_gjk_core_3d != 0` — *literally the
  `gjk_intersect_3d` body from before 2.9.0 taught it tangency*. 2.9.0 repaired one call site of
  that logic and left the other two. Root cause → fix: `mpr_intersect` delegates its no-portal path
  to `gjk_intersect_3d` (`collision_core.cyr:73`), and `mpr_penetration` runs the same
  `_epa_touch_probe` certificate `gjk_epa_3d` runs at `geo_advanced.cyr:1237`.

  **The 2026-08-05 audit addendum was re-measured from scratch before it was fixed**, because its
  "26 of 3,012" had never been independently reproduced and a second sweep had found *zero*
  disagreements. New sweep: 1,149 fixtures × 5 dyadic scales (2^-20 … 2^20) × both operand orders
  = **11,490 pair evaluations**, classified exactly with `Fraction` from the f64 bit patterns the
  support functions receive; the classifier was validated by first reproducing all 55 of the
  suite's own independently-derived classifications, 0 disagreements. The honest count is **50 of
  7,540** — the subset with exact support functions (boxes, parallelepipeds, zero-radius spheres).
  The other 3,950 involve a ball of radius r > 0, whose support point carries ~2 ulp of r (measured
  7.1e-15 at r = 19 against a 3.6e-15 coordinate ulp), so a 1-ulp tangency there is unresolvable by
  any algorithm on that support function. On the exact half: `gjk_intersect_3d` wrong **0** times,
  `mpr_intersect` wrong **50** — every one an exact tangency reported as a miss, every one
  `mpr = 0`/`gjk = 1`, **0 false positives** for either.

  Both prior reports were right about different things. The 50 reduce to **6 configurations**; 4 are
  coincident degenerate self-pairs, but **2 are not and had not been reported**: a point resting
  exactly on the **corner of a solid unit box**. Those two were also an **operand-order asymmetry** —
  `mpr_intersect(box, corner point) = 0` but `mpr_intersect(corner point, box) = 1`. **160 tangency
  evaluations between two shapes that both have volume: 0 disagreements**, which is precisely why an
  axis-aligned face/edge/vertex sweep saw none of it. That asymmetry is what ruled out "keep the
  strict reading and document it": a symmetric predicate that depends on argument order is broken
  under any tangency contract. After: **0 wrong, 0 asymmetric, 0 false positives** on all 7,540.

- **`mpr_penetration` left both out-params unwritten on every `0` return** — the NULL-`HVec3` hazard
  `gjk_epa_3d` fixed in 2.8.3 and this sibling never got. A caller passing fresh `alloc()` memory
  (which is zeroed) held a null pointer that faults on the first accessor. Demonstrated rather than
  assumed: against the pre-2.9.1 body the new tangency group does not merely fail, it **aborts the
  suite** on exactly that read. Every path now writes a defined placeholder normal and depth 0.

- **`collision_mesh` — `_col_dl_ic_g3` returned the WINDING of the ghost triple instead of the
  in-circle answer** (modules **+10**). The all-ghost branch computed `orient(u_a, u_b, u_c) > 0`,
  so a clockwise presentation answered "outside" for *every point in the plane*. Measured against an
  exact-rational circumcircle oracle: **243 of 243** CW-presented triples wrong, **0 of 243** cyclic.

  Root cause → fix: the answer is the **constant `1`**, so the body is `return 1;` and the predicate
  now takes **no arguments** (`collision_mesh.cyr:383`; the entry point's `g == 3` branch loses its
  three index arguments at `:401`). Two independent derivations, both confirmed in exact rationals
  rather than inherited from the filed analysis. *Geometric*: all three `|u_k|²` are equal (5), so
  the apex `A` is exactly the circumcenter of the three ghosts and their circumcircle is the circle
  of radius `M√5` about `A`, which swallows any finite `p` once `M` is large enough. *Algebraic*:
  the M⁴ coefficient is `|u|²·orient(u_a, u_b, u_c)` — the query point cancels out of the leading
  term — while the ghost triangle's own orientation is `M²·orient(u_a, u_b, u_c)`, so the winding
  factor **cancels between them** instead of having to be divided out the way `_col_dl_ic_g2`
  divides it out with `cr`. `|u|²·orient(u) = 60 ≠ 0`, so the leading coefficient never vanishes and
  there is no tie-break branch to get wrong.

  The equal-norm premise is load-bearing and the header now says so: for three directions chosen
  *without* that constraint the answer is **not** constant — a sweep of random direction triples
  answers `0` about **28%** of the time (27.6/27.9/28.0% uniform in [-1,1]^2, 27.1/26.9/27.4% in the unit disc, 20,000 triples per seed in exact rationals — the rate is distribution-dependent, so the distribution is named with it). The previous header already claimed "every finite point is
  inside"; the body had always contradicted its own documentation.

  **No behaviour change on any reachable path** — `delaunay_2d` stores every triangle CCW and
  `_col_dl_incircle`'s three rotations are even permutations, so only the cyclic half was ever
  reached. Latent since 2.8.2, the same latency class as the `_col_dl_ic_g2` tie branch above.
  Mutation-proved: reinstating the old winding logic at the entry point (leaving the helper's arity
  intact) fails exactly the 4 anti-cyclic assertions with the grid counter at **900 of 1800** —
  all three anti-cyclic orderings of all 300 configurations, cyclic half untouched; inverting the
  constant fails all 9 behavioural assertions.
  (`docs/development/issues/archived/2026-08-06-ghost-incircle-g1-g3-assume-ccw.md`)

- **`collision_mesh` — `_col_dl_ic_g2`'s two-ghost tie branch applied the winding twice**, inverting
  the in-circle answer for every anti-cyclic ghost pair. The M² coefficient of the determinant is
  `(u_j × u_k)·(|R − A|² − |A − p|²)`, so it *already* carries one factor of the winding; the branch
  multiplied by `cr` again. With `cr² = 1` that is the identity when `cr = +1` and a sign inversion
  when `cr = −1`, so the same function used two different sign conventions in its two branches.
  Root cause → fix: `if (s2 * cr > 0)` → `if (s2 > 0)` at `collision_mesh.cyr:320`; the M³ branch at
  `:298` was correct and is unchanged.

  Re-verified against an oracle sharing no reasoning with the original derivation — the **exact
  circumcircle of the three actual points** (`Fraction` circumcenter, exact distance-vs-radius),
  a question that takes no orientation argument. Over 343,734 decided trials per orientation:
  shipped 2.9.0 was 0 wrong cyclic and **24,156 wrong anti-cyclic**; fixed is **0 and 0**. On the
  tie branch alone the split is 0 of 267,358 against **267,358 of 267,358**.

  **No behaviour change on any reachable path** — `_col_dl_incircle` only ever rotates a CCW-stored
  triangle and rotation is an even permutation, so `cr` is `+1` throughout `delaunay_2d` and the two
  expressions coincide there. Latent since 2.8.2.
  (`docs/development/issues/archived/2026-08-05-two-ghost-tie-branch-sign-rule-inconsistent.md`)

### Added

- **A `mpr_penetration` arena bound that can actually see a per-face-per-iteration leak** (modules
  **+4**). The existing bound was absolute and taken on a 24-support-call box pair: baseline
  22,776 B against 25,600, where the 80 B/face/iteration injection reaches only 25,656 — it cleared
  the bound by **56 B, 0.22%**. A thin bound is not repaired by a tighter number; it is repaired by
  a fixture with more iterations, because the leak is quadratic in the iteration count while the
  honest cost is linear. The new fixture is the **same sphere pair the `gjk_epa_3d` arena row
  already uses**, so the two budgets are directly comparable, and it is normalised per support call:
  **167,904 B over 572 calls = 293 B/call** (the sibling: 296 over 552), byte-identical across five
  separate processes *and* five repeat calls in one process. Under injection it reads **893 B/call**,
  a 3.05× jump matching the sibling's 296 → 917. Bound 400 → 36% headroom, and the injection
  overshoots by **123%**. Detection margin **0.22% → 123%**. The old absolute bound is kept: it
  still guards the polytope regime the sphere pair does not exercise.

- **23 assertions covering both `mpr_*` entry points** (modules **+23**). `mpr_intersect` and
  `mpr_penetration` now ride the existing agreement sweep — three new aggregates, against which the
  pre-repair bodies score **9 / 2 / 13** on its 110 evaluations — plus a named group for all six
  reproducers in both operand orders, with ±1-ulp negative controls on either side of the corner
  tangency so the repair is pinned from both directions and cannot manufacture a contact. The audit
  addendum's standing complaint that "no assertion anywhere compares `mpr_intersect` against either"
  sibling is discharged. Reverting `collision_core.cyr` fails 9 of them and aborts the run.

- **`mpr_intersect` benchmark rows** (`mpr_intersect_box_miss` / `_box_hit` / `_tangent`). It had no
  benchmark at all — the same blind spot that let 2.9.0 repair one sibling and leave this one, with
  neither a test nor a number covering the difference.

- **9 assertions pinning the anti-cyclic half of the two-ghost in-circle predicate** (modules
  **+9**). `_dlg_run()` became `_dlg_run(anti)`, running its 450-probe grid over the three
  **anti-cyclic** ghost pairs as well as the three cyclic ones — the half no `delaunay_2d` fixture
  in any suite reaches, which is why a 100%-wrong branch stayed green. New `_dlg_perm_bad` drives
  the public entry point `_col_dl_incircle` with all six orderings of `(R, G_j, G_k)` and asserts
  the answers agree: "inside the circumcircle" is a question about a *set*, so an order-dependent
  answer is a defect by definition.

  Mutation-proven both ways. Reverting the fix fails 3 assertions (678 / 224 / 672); dropping
  `* cr` from the M³ branch — the "assert the precondition and drop the factor" alternative — also
  fails 3 (639 / 213 / 639), which is the evidence that rejected it.

### Performance

- `mpr_intersect` separated-pair path: **1.452 → 1.698 µs (+17%)** for box/box and **1.975 → 2.360
  (+19%)** for sphere/sphere; hit and tangency paths unchanged (5.610 → 5.614, 6.307 → 6.252).
  Same-binary A/B of the old and new bodies, 2000-call batches, min of 20 rounds, median of min over
  5 runs. This is a deliberate regression on the broadphase miss path, and it is the **cheaper** of
  the two repairs that score identically correct: appending `_epa_touch_probe` unconditionally costs
  **+42% / +41% / +44%** on the same three fixtures. Delegating to `gjk_intersect_3d` inherits
  2.9.0's gate — one extra support evaluation on a separated pair, with the probe running only on a
  genuinely unresolved GJK exit. The in-suite `mpr_intersect_box_miss` row prints min = 1.611 µs; the
  A/B harness reads higher on both terms because it never calls `alloc_reset`, the same discrepancy
  already on record for `gjk_intersect_box_miss` (807 ns in-suite against 766 quoted).

### Changed

- **`_col_dl_ic_g2` now documents that it is order-free and that `_col_dl_incircle` is not.** Its
  siblings `_col_dl_ic_g1` and `_col_dl_ic_g3` drop the winding factor entirely and are wrong for a
  clockwise triple (measured: **462,846 of 463,086** and **243 of 243**), so the `cr` handling in
  `g2` must not be read as a claim about the entry point. Filed, not fixed, along with a second and
  orientation-*independent* defect found in `g1`'s M¹ tie-break — it answers "outside" for a point
  strictly between `a` and `b` whenever the edge is parallel to `u_k`, 228 of 463,086 probes on
  correctly CCW-stored input. (`docs/development/issues/archived/2026-08-06-ghost-incircle-g1-g3-assume-ccw.md`)
- **`2026-08-04-incircle-precision.md` re-measured and recommended for closure.** Its correction
  section reasoned from a super-triangle at `10 × max(dx, dy)` that 2.8.2 deleted — there is no
  super-triangle on this tree, and `_col_in_circumcircle`'s single caller
  (`_col_dl_incircle:353`, the `g == 0` branch) sees only real input points. Re-measured against
  exact integer arithmetic over every point quadruple of the suite's own delaunay fixtures, a strict
  superset of the calls `delaunay_2d` can make: **0 wrong of 1,528,820**, versus the 0-of-180 it
  replaces. The predicate remains non-robust in isolation (**36 of 400** at the original's magnitude
  range, up to 198 of 400 at extreme spread) — what changed is that nothing can hand it that input.

## [2.9.0] - 2026-08-05 — a narrowphase a physics engine can build on

All five exit criteria met. Suite **1818 → 3294** across five harnesses (a fifth was added:
`tests/abuse.tcyr`), constant gate 153/153, toolchain 6.5.6 → **6.5.8**, sakshi 2.4.7 → **2.4.8**.

**Read the shape of this release before the list.** It was scoped as "make the narrowphase
trustworthy" — and that shipped early, in 2.8.3. What 2.9.0 actually delivers is the tail that only
became visible once there were mechanisms to see it: an abuse harness that found 11 defects on
public entry points, a disposition column that caught a finding marked FIXED when half of it was
not, and a mutation sweep that found the property behind a CRITICAL had zero assertions. Every one
of those was cheap to find and had been invisible because nothing looked.

### Added

- **`tests/abuse.tcyr` — an abuse harness** (exit criterion 2). 732 assertions over negative
  indices, zero/huge dimensions, non-conformable operands, the designed-0 return, degenerate
  geometry, and canary checks. It surfaced **11 real defects on public entry points**, held in a
  known-defect register rather than deleted.
- **A Disposition column on the 2026-08-04 audit** (exit criterion 5). The table stated the rule
  "the finding list is the unit of disposition" and provided nowhere to record one. Now
  **36 FIXED / 6 OPEN / 0 REFUTED** of 42, plus an addendum row.
- **A struct-layout contract**, set deliberately while nothing depends on it. 32 assertions across
  16 public structs. Construct via the constructor, read via accessors, size with `sizeof(T)` —
  never a hardcoded byte count, never a hand-computed offset. `ColContact` grew 64 → 72 bytes this
  release and nothing would have noticed; now a layout change trips a gate and forces an entry here.

### Fixed

- **`cqr_decompose` overran its output buffer while returning `HSB_ERR_NONE`** — a success code on
  top of the overrun, 2 of 8 canary slots clobbered. The algorithm was proved general for
  rectangular input first, so what is rejected is the mis-sized out-buffer, not the shape.
- **`gjk_intersect_3d` missed 134 genuine interior overlaps.** Measured against an exact-rational
  reference over 4,386 evaluations, scales 2⁻³⁰–2³⁰: missed overlaps 134 → 0, sibling disagreement
  with `gjk_epa_3d` 390 → 0, tangencies certified 221/612 → 477/612, regressions 0. Costs +55%
  median on the no-hit path (the honest figure; a first measurement said +44% and was deflated by
  arena growth in its own harness).
- **The friction impulse was identically zero at every μ and every iteration count.** `old_fric`
  was loaded from the output slot the function had itself zeroed, clamped, and stored back — no
  update between load and clamp. `friction` was a silently ignored constructor parameter. It
  survived because every solver fixture uses friction = 0, so the branch was never entered.
- **Crash tier:** `eigen_power`, `lyapunov_max`, `eigen_qr` (null handles and unbounded `n`),
  `einsum` (its own output fed back in), `kdtree_build` and `bvh_build` (`count == 0` half-guards),
  `halfedge_adjacent_faces`/`halfedge_is_boundary`, `num_modpow` (SIGFPE at modulus 0 — and three
  silent WRONG ANSWERS found alongside it, where a negative base collapsed the accumulator).
- **The never-worked low tier**, 5 of 5 — never deferred with a reason, simply never picked up, and
  invisible until there was a column to record it in. Includes a return-convention split where
  three sibling inserters used two conventions (**Breaking**, below).

### Changed

- **Breaking:** `quadtree_insert` and `octree_insert` move from `1`-on-success to the project's
  `0 / HSB_ERR_NONE` convention, matching `spatial_hash_insert` and the rest of the library. Nine
  test expectations updated; every caller in `src/`, `tests/` and `examples/` audited.
- **Breaking (narrow):** `so3_from_mat3` returns 0 for a matrix that is not a rotation.

### Verification

- **54 mutations on the integrated tree** (exit criterion 4) — a different claim from each of eight
  merge tracks proving its own in isolation. 45 caught. Of the 11 that broke nothing, 5 were real
  coverage gaps and are closed; 6 are referred to 2.9.1 as a design question rather than papered
  over. **The delaunay ghost-direction invariant — the property the 2.8.2 critical was rebuilt
  around — had zero assertions**, and its own documented failing set passed every suite.
- Two measurements stated in committed comments were found not to reproduce and were corrected
  (`eigen_qr`'s justification; `cqr_decompose`'s arena figures, wrong by ~2× across four files).
  Both were caught by an adversarial reader, neither by a gate. `scripts/check-measurements.sh`
  exists but is **not adoptable** and is deferred to 2.9.1 — it misses 8 of 10 realistically-phrased
  claims, and a gate that flags noise while missing real claims is worse than none.


### Breaking

- **`quadtree_insert` and `octree_insert` now return `HSB_ERR_NONE` (0) on success and
  `HSB_ERR_OUT_OF_RANGE` (−3) for a point dropped as out of bounds.** They returned `1` and `0`.
  The 2026-08-04 audit's api-consistency row recorded three sibling insert routines carrying two
  conventions: `spatial_hash_insert` has always returned 0 on success, matching the project rule
  that 0 / `HSB_ERR_NONE` is success and a negative `HSB_ERR_*` is failure, while its two siblings
  inverted it — so `if (insert(…) == 0)` meant "stored" for one and "dropped" for the other two.
  `spatial_hash_insert` was the one already correct, so the outliers moved onto it.

  **Migration.** A caller that ignores the return is unaffected — that is every call site in `src/`
  (there are none) and every one in `tests/` that was not asserting on the value.
  - `if (quadtree_insert(qt, x, y, d) == 1) { … }`  →  `if (quadtree_insert(qt, x, y, d) == HSB_ERR_NONE) { … }`
  - `if (octree_insert(ot, p, d) == 0) { /* dropped */ }`  →  `if (octree_insert(ot, p, d) < 0) { /* dropped */ }`

  The failure value is now negative and distinguishable from success, which the old encoding could
  not be: "dropped" was the same integer as the project's success value. `spatial_hash_insert` is
  unchanged in behaviour; its return contract, previously undocumented, is now stated (it has no
  bounds to fall outside of, so it has no rejection path and always returns `HSB_ERR_NONE`).
  Guarded by `tests/abuse.tcyr`, *"spatial: the three sibling inserts share ONE return convention"*,
  which compares the three against **each other** rather than against hardcoded numbers.

### Fixed

- **collision_core — `sequential_impulse` computed an identically-zero friction impulse.** The
  second half of the 2026-08-04 audit row whose restitution half shipped in 2.8.3 (the block is
  byte-identical to 2.8.1, md5 `02696836a11a8e4898d94c2bac6bd60c`). The friction branch loaded the
  output slot it had itself zeroed at entry, clamped that zero against `mu * lambda_n`, and stored
  it back — there was no update between the load and the clamp, and `|0| > cone` is never true for
  a non-negative cone. Measured before the repair: **0 of 320 sampled (mu, penetration, iteration)
  configurations produced a nonzero friction impulse**, so `friction` was a silently ignored
  constructor parameter at every mu and every iteration count. It survived because no assertion in
  `tests/` mentioned friction and every solver fixture used `friction = 0`, so the branch was never
  entered. The tangential constraint is now solved against the contact's relative sliding speed
  with the same effective-velocity convention the normal path uses, giving the closed form
  `lambda_t = clamp(-v_t / inv_mass, -mu*lambda_n, +mu*lambda_n)` — accumulate **then** clamp, so
  the bound is on the TOTAL tangential impulse and a saturated contact stays saturated instead of
  gaining another `mu*lambda_n` per sweep. Like the 2.8.3 restitution repair it converges in one
  sweep and is iteration-count independent. Reverting it fails 12 of the new assertions; deleting
  just the Coulomb clamp fails 10; flipping the sign fails 12.
- **geo_advanced — `gjk_intersect_3d` contradicted its own sibling `gjk_epa_3d`.** 2.8.3 taught
  `gjk_epa_3d` to certify an exact tangency as a depth-0 contact and deliberately left the cheap
  overlap test alone, so the two public entry points disagreed. Swept over 1,506 configurations
  classified exactly in rationals (dyadic coordinates, both operand orders, five scales from 2^-20
  to 2^20): **141 of 3,012 pair evaluations disagreed**, every one of them `epa=1`/`intersect=0`.
  66 were not tangencies at all but pairs with a genuine interior overlap — all at scale 2^-20, all
  in the families whose support function normalises, because `hvec3_normalize`'s 1e-12 floor is
  absolute and hands back the shape's centre once GJK's search direction collapses, which
  manufactures a separating certificate out of a point that is not a support point. After the
  repair: **0 of 3,012 disagree**, and **0 false positives** across 2,214 exactly-separated
  evaluations (sphere/sphere 0, box/box 0, sphere/box 0).
- **time_of_impact** — its `t = 0` overlap test is `gjk_intersect_3d`, so a pair exactly touching at
  `t = 0` now reports contact at `t = 0` with a unit normal, instead of falling through to the
  zero-speed early-out and returning "no collision".
- **tests/modules.tcyr** — the group titled *"gjk_epa_3d: agrees with gjk_intersect_3d"* asserted
  that agreement only on overlapping fixtures, so it stayed green while its own stated contract was
  false. It now sweeps 55 fixtures x 2 operand orders through both entry points and asserts the
  aggregate (agreement, false positives, missed overlaps, tangency recall, depth-at-a-touch).
  Reverting the repair fails 7 of the new assertions.
- **The crash tier — six entries of `tests/abuse.tcyr`'s KNOWN DEFECT REGISTER, all now live
  assertions.** Each was a public entry point that ended the process or corrupted the heap on an
  input its own sibling already validated, so none could be executed inside a suite that has to
  exit 0. Every reproducer below was re-measured before the repair, not inherited from the register.
  - **linalg_precision — `cqr_decompose` returned `HSB_ERR_NONE` on top of a heap overrun.** The
    headline. Nothing checked that `out_R` was the documented `m x n`, and `cmat_set` takes its
    stride from `out_R`'s **own** column count, so an `m x m` `out_R` — what a caller sizing for a
    square decomposition allocates — took a store past the end for every `j >= m`. Measured on a
    2x3 A filled 1..6 into a 2x2 `out_R`: **returns 0 (`HSB_ERR_NONE`) with 2 of 8 guard slots
    clobbered**, the 16 bytes immediately after `out_R`'s data buffer; the square 2x2 control over
    the same code path clobbers 0 of 8. `out_Q` (`m x m`) and `out_R` (`m x n`) are now both
    checked, and all three handles are null-checked before any dimension is read — separately a
    SIGSEGV, since `cmat_rows(0)` is `load64(0)` and `cmat_new` returns its designed 0 above
    `_CMAT_MAX_ELEMS` (`cqr_decompose(cmat_new(100000, 100000), …)` exited 139). The **shape is
    rejected, not coerced** — there is nowhere to put an `m x n` R inside an `m x m` allocation —
    but the **rectangular case is handled, not rejected**: Householder QR is general in m and n,
    and with `out_R` sized correctly the 2x3 decomposition is measurably right (`Q*R == A`,
    `Q^H Q == I`, R upper trapezoidal, all to < 1e-8; same on a tall 4x2), so the guard rejects a
    caller error and not a legitimate shape.
  - **linalg_precision — `eigen_qr` had none of the four guards its own file-mate already had.**
    `svd_golub_kahan` (`:753`) validates null A / null outputs / non-positive dimension / shape;
    `eigen_qr` guarded `n == 0` and nothing else. `eigen_qr(0, 3, …)`, `n = -3`, a null `out_vecs`
    and a null `out_vals` **on a matrix that converges** all exited 139. It now uses
    `svd_golub_kahan`'s idiom verbatim, plus `A` must be exactly `n x n` and `out_vecs` at least
    `n x n`.
  - **linalg_ext — `eigen_power` and `lyapunov_max` did not bound `n` at all.** `alloc(n * 8)`
    with whatever arrived, then stored through: `n = -3` (negative size) and `n = 1e9` (8 GB, over
    `ALLOC_MAX`, so `alloc` returns its designed 0) both exited 139, as did `lyapunov_max` at
    `n = 0` and `n = -2` where `mat_new`'s designed 0 went straight into `mat_mul`. Both now
    null-check their handles, reject `n <= 0`, and require `n` to equal A's own extent; working
    allocations are checked for the designed 0 and return `HSB_ERR_ALLOC`. A 3x4 A with `n = 3` is
    **rejected rather than answered**: it does not read out of bounds, but it silently returned
    `HSB_ERR_NONE` carrying the result for A's leading 3x3 block, which is not the matrix passed.
  - **einsum — its own output fed back in killed the process.** `HTensor_rank(load64(tensors + oi * 8))`
    with no `t == 0` check, while `tensor_add` / `tensor_scale` / `tensor_contract` all guard
    exactly this and name einsum as the producer of the 0. `einsum` returns 0 on every error path
    and `tensor_new` returns 0 above its caps, so composing two einsum calls is the ordinary route
    to a null operand: `store64(tv, 0); einsum(str_from("ij->ji"), tv, 1)` exited 139. `notation`,
    `tensors` and every operand handle are now checked.
  - **spatial — `kdtree_build`'s `count == 0` was a half-guard.** A negative count reached
    `_kd_build_rec(items, points, 0, -1)`, missed the `count == 1` leaf case, and let
    `_kd_select_median` `vec_get` an out-of-range slot — vec's own bounds check aborted the process
    (`"vec: index out of bounds"`, exit 1). Now `count <= 0` and a null point array both return the
    documented empty-tree 0; the function returns a node **handle**, so there is no negative
    `HSB_ERR_*` to hand back through a pointer.
- **The crash tier, part two — the last four register entries.** The register is now empty of
  `[OPEN]`: three were repaired and the fourth was re-measured and reclassified. Every signal below
  was measured with a standalone probe (raw exit code) before `src/` was touched.
  - **geo_advanced — `bvh_build` carried `kdtree_build`'s half-guard one cycle longer.** The two
    siblings therefore disagreed about the same malformed input: one returned 0, the other ended
    the process. A negative count reached `_bvh_build_rec(items, 0, -1)`, whose
    `count = end - start` is −1, so it missed the `count == 1` leaf case and folded bounds over an
    empty vec — `bvh_build(boxes, -1)` aborted with `"vec: index out of bounds"`, exit 1, and
    `bvh_build(0, 2)` (a null array, not in the register) exited 139. `kdtree_build`'s guard is now
    copied **verbatim in shape**, and three assertions pin `bvh_build`'s answer to `kdtree_build`'s
    on identical input so the pair cannot drift again. 0 is the correct rejection value and not a
    dodge: the header at `geo_advanced.cyr:1424` already documents the return as a node **handle**
    whose empty value is 0.
  - **collision_mesh — `halfedge_adjacent_faces` and `halfedge_is_boundary` took an unbounded index
    into `vec_get`,** while `halfedge_from_triangles` — the function that produces the very face and
    vertex ids they are called with — validates `n_tris`, `n_verts` and every vertex id. Six
    measured signals: `(mesh, -1)` `"vec: index < 0"` exit 1, `(mesh, 99)` `"vec: index out of
    bounds"` exit 1, and `(0, 0)` exit 139, for each of the two. **The two rejection values differ
    on purpose.** `halfedge_adjacent_faces` returns a vec **handle**, so its error value is 0 — and
    it must be 0 rather than an empty vec, because an isolated face legitimately has no neighbours
    and answers with a non-null **empty** vec. `halfedge_is_boundary` is a **predicate** with no
    spare handle value: 0 would assert "interior" about a vertex that does not exist, so it returns
    `HSB_ERR_OUT_OF_RANGE` / `HSB_ERR_INVALID_INPUT`, which sit outside `{0, 1}` and leave both
    `== 1` and `== 0` callers intact.
  - **num — `num_modpow` guarded `modulus == 1` but not `modulus == 0`,** so the next line,
    `base % modulus`, was an integer divide by zero: `num_modpow(2, 10, 0)` raised **SIGFPE, exit
    136**. It now returns `HSB_ERR_DIVISION_BY_ZERO` — `x mod 0` is undefined in the mathematics
    too, and `num.cyr:51` already uses that code for exactly this. Measuring it turned up three
    **non-crash** siblings in the same signature: the header said "all arguments are non-negative
    integers" and nothing enforced it, so — measured against a verbatim copy of the pre-fix body —
    `(2, 10, -3)` returned **1**, `(2, -1, 5)` returned **1**, and `(-2, 3, 5)` returned **0** where
    the true residue is 2 (a negative base reaches `_num_mulmod` as its `b`, whose own
    `while (b > 0)` never runs, collapsing the accumulator). All three now return
    `HSB_ERR_INVALID_INPUT`, which also makes the return space unambiguous: every legal call answers
    in `[0, modulus)`, so a negative result is an error code.
- **tests/abuse.tcyr** — **the KNOWN DEFECT REGISTER is discharged.** It opened this cycle at ten
  entries, every one of them an input that could not be executed inside a suite that has to exit 0.
  Nine are now repaired and live assertions — including the canary check that catches the
  `cqr_decompose` overrun as a **test failure** (`got 2, expected 0`) rather than as luck — and the
  tenth, `cmat_get` / `cmat_set`, is re-measured and reclassified `[BY DESIGN]` (see Changed). No
  `[OPEN]` entry remains. Two register reproducers were **wrong as written** and are corrected in
  place rather than trusted: `eigen_qr(mat_new(3, 3), 3, 50, 0, mat_new(3, 3))` exits 0, not 139,
  because an all-zero A returns from the tridiagonal QR before it ever writes an eigenvalue (the
  null is only dereferenced on an A that converges, e.g. `diag(2, 3, 4)`); and
  `cmat_get(cmat_new(2, 2), -1, 0)` exits 0 returning `0+0i`, not 139. The suite ends this pass at
  **732 assertions, 0 failed**.
- **spatial — `quadtree_new` / `octree_new` stored the caller's `max_depth` uncapped, and it was the
  only bound on the insert recursion.** Coincident points always route into the same child, so past
  the node capacity every insert drives a split chain the *full* depth. Measured on this machine
  (`ulimit -s` 8192, 64 coincident points): the quadtree exits 0 at `max_depth` 23,750 and
  **SIGSEGVs (139) at 23,906**; the octree exits 0 at 20,000 and SIGSEGVs at 30,000. The arena — a
  bump allocator that never frees — grew 1,216 B per quadtree level and 6,144 B per octree level,
  i.e. **24.3 MB and 123 MB at `max_depth` 20,000**. Both constructors now clamp to
  `[0, _SP_MAX_TREE_DEPTH]` with `_SP_MAX_TREE_DEPTH = 1075`. The cap is measured, not guessed: in a
  unit root box a split separates two coordinates only while the cell is wider than the gap between
  them, and the deepest depth at which that is still possible is 53 for two coordinates 1 ULP apart
  at 0.5 and **1074 for 0 against the smallest positive denormal** — so 1075 keeps every separation
  f64 can express and truncates only the case where no depth would ever have helped. At the cap the
  pathological chain costs a bounded **1,311,304 B (quadtree) / 6,606,992 B (octree)**, measured, and
  `max_depth` 1e6 and 2^31−1 now allocate byte-identically to the cap. Capping degrades nothing to
  incorrect: an over-full leaf is still answered by linear scan. A negative `max_depth` already meant
  "never split" and is clamped to 0, which is the same behaviour written down. Removing the clamp
  makes `cyrius test tests/modules.tcyr` exit **139**.
- **geo — `geo_ray_plane`'s doc block stated the opposite convention to the code**, on the line
  directly above the 2.7.0 note that corrects it: *"Returns t >= 0 on hit, -1 on miss (negative
  sentinel, not 0, to distinguish t=0 hits)"*. The **code is right** — `src/geo.cyr:5` and the
  ray-section header both promise 0 on a miss, and every sibling `geo_ray_*` returns 0 — so the doc
  line was replaced, not the code. While pinning it, the one thing the stale line gestured at turned
  out to be real in a way nobody had recorded: a `t = 0` grazing hit returns **−0.0**
  (`0x8000000000000000`) when `denom < 0` and **+0.0** when `denom > 0`, the latter byte-identical to
  a miss. Both are `f64`-equal to zero, so no `f64_lt` / `f64_gt` / `f64_eq` test separates a grazing
  hit from a miss either way; the raw bits must not be read as a discriminator. That is now stated in
  the doc and pinned by ten assertions in `tests/modules.tcyr`. Reverting the code to the `-1`
  sentinel fails 6 of them.
- **linalg_ext — `eigen_symmetric`'s doc named a `max_iter` parameter that does not exist**, directly
  above a four-parameter signature that has never had one (*"HSB_ERR_NO_CONVERGENCE if max_iter
  exceeded"*). The budget belongs to `ganita_mat_eigen_sym` (`lib/ganita.cyr:890`,
  `100 * n * n`) and is not caller-settable. Corrected.

### Changed

- **`num_modpow` rejects negative `base`, `exp` or `modulus` with `HSB_ERR_INVALID_INPUT` instead of
  silently returning a wrong number** (see above). No in-tree caller is affected — the only one,
  `_num_miller_rabin_test`, passes `a % n` with `n > 3` odd — and no assertion covered the negative
  cases before this. Consumers relying on the old values were relying on undefined behaviour of a
  documented-non-negative contract.
- **`cmat_get` / `cmat_set` are confirmed unguarded raw accessors — reviewed and deliberately left
  that way**, with the reasoning recorded in `src/complex.cyr` so the next reader does not "fix"
  it. Unlike every other register entry these are not half-guarded public entry points but
  accessors with a stated precondition, and no rejection value beats the precondition:
  `cmat_get`'s success value is a **handle** from `cx_new`, so returning 0 merely moves the SIGSEGV
  to the caller's `HComplex_re(0)`, and returning `cx_zero()` is bit-identical to a legitimate
  `0+0i` element — a wrong answer wearing a valid answer's clothes. They also mirror stdlib
  `ganita_mat_get` / `ganita_mat_set` (`lib/ganita.cyr:53`, `:59`), the same unchecked stride
  computation; and they are the inner loop — with bounds tests temporarily added to both, a 120×120
  complex matmul went **276 ms → 294 ms (+6.5%)**, same `c[0][0]` checksum, for a check every
  legitimate caller has already done. The surface a caller can actually reach **is** validated one
  level up: every public `cmat_*` entry point rejects `cmat_new`'s designed 0 before an accessor
  sees it, and all 97 in-tree accessor call sites (39 `cmat_get`, 58 `cmat_set`) are inside those
  routines. **The register's reproducer did not reproduce**: `cmat_get(cmat_new(2, 2), -1, 0)` was
  recorded as exit 139, and re-measured in isolation it exits 0 returning `0+0i`, because
  −1 × 2 × 16 = −32 bytes lands inside the 24-byte header the bump allocator handed out just ahead
  of the data buffer. What does fault is a far index (±1e8) or a null handle, both exit 139, both
  precondition violations.
- **`eigen_qr` on a non-square A returns `HSB_ERR_INVALID_INPUT` (−10) instead of
  `HSB_ERR_NO_CONVERGENCE` (−8).** The abuse assertion covering this was **tightened, not
  weakened**: a `NO_CONVERGENCE` that depends on `max_iter` is a fragile contract, and at
  `max_iter = 10000` the same 3x4-with-`n = 3` call returned `HSB_ERR_NONE` carrying the spectrum of
  the leading 3x3 block — a real number for a matrix nobody asked about. Callers relying on the old
  code for a wide or tall A should pass the square matrix they mean.
- **`eigen_symmetric` now returns eigenvalues sorted by descending |λ|, with the eigenvector columns
  permuted alongside — the order `eigen_qr` has always used and neither routine documented.** The
  2026-08-04 audit's api-consistency row recorded that the two carry the same documented contract and
  returned different orders. Measured on the 2.8.4 tree: for symmetric `A = [[2,0,0],[0,3,1],[0,1,3]]`
  (spectrum {4, 2, 2}) `eigen_symmetric` gave **2, 4, 2** and `eigen_qr` gave **4, 2, 2**; for
  `diag(1,5,3)`, **1, 5, 3** against **5, 3, 1**. `eigen_symmetric` delegates to ganita's Jacobi
  sweep, whose output order is whatever the final diagonal happens to be, while `eigen_qr` has sorted
  since 2.6.13 — and had assertions pinning that order (`tests/abuse.tcyr`, `tests/hisab.tcyr`), so
  the sibling was moved onto it rather than the reverse. **`eigen_qr` is unchanged.** Both doc blocks
  now state the order. A caller that read `eigen_symmetric`'s raw Jacobi order gets a different
  permutation; there was no documented order to rely on, and the (λ, v) pairing is preserved because
  the columns move with the values. Sorting by |λ| rather than by λ is deliberate and pinned:
  `diag(-5,1,3)` returns −5, 3, 1. Guarded by `tests/hisab.tcyr`, *"eigen_symmetric / eigen_qr: one
  documented eigenvalue order"*, which compares the two **against each other** on four matrices;
  removing the sort fails **13** assertions (`cyrius test tests/hisab.tcyr` → 403 passed, 13 failed).

### Performance

- `gjk_intersect_3d` no-hit path pays for the repair with **one extra support evaluation**, not with
  the full touch probe: the probe runs only when GJK's exit certificate does not survive re-reading
  along a unit direction, which over the 3,012-pair sweep was 187 calls (6.2%). Batch-timed against
  the old body in the same binary (2,000 calls/round, min of 20 rounds, median of 5 runs):
  box/box 531 → 766 ns/call (+44%), sphere/sphere 901 → 1353 (+50%), sphere/box 685 → 1039 (+52%).
  The overlap path is untouched (2145 → 2197, +2.4%) and a near-tangent but separated pair — the
  resting-contact regime — pays only the same one call (1e-9 gap 3293 → 3649). **Running the touch
  probe unconditionally instead was built and measured: 531 → 1066, 901 → 1790, 685 → 1424, a
  doubling of the module's hottest entry point.** An exact tangency now costs 9955 ns against the
  4000 it used to spend reaching the wrong answer.
- **The residual allocation-inside-loop sites are hoisted.** The 2026-08-04 audit named four; three
  were real and are fixed, the fourth is discharged by measurement (below). This is a bump allocator
  that never frees, so an allocation inside a loop is arena growth proportional to the iteration
  count on **every** call. All figures are `alloc_used()` deltas around a single call.
  - `src/spatial.cyr` — `var plane_out = alloc(8);` per `_kd_build_rec` call. One 8-byte slot is now
    owned by `kdtree_build` and threaded through the recursion; sharing it is safe because
    `split_val` is read on the next line, before either child call can overwrite it.
    **`kdtree_build` on 4,096 random points: 425,832 B → 393,080 B (−32,752 B, −7.7%)** — exactly
    8 B × 4,094 internal nodes.
  - `src/tensor.cyr:189` — `var full_idx = alloc(rank * 8);` inside `tensor_contract`'s trace loop.
    Only slots `ii` and `jj` change per iteration and both are overwritten, so one buffer zeroed once
    is identical. **200×200 full trace: 3,256 B → 72 B (−3,184 B, −97.8%).**
  - `src/linalg_precision.cyr` — `var v = alloc(len * 16);` inside `cqr_decompose`'s reflector loop,
    costing `16·m·(m+1)/2` bytes per call. Now one `alloc(m * 16)` before the loop; `len = m − k`, so
    the first pass needs the largest buffer and every later pass fits inside it.
    **`cqr_decompose` on a 64×64: 32,942,104 B → 32,909,848 B (−32,256 B, −0.098%).** The bulk of that
    call is `cx_new`'s 16 B per complex temporary in the O(m²n) reflector application — a separate
    problem this hoist does not claim to solve, and the reason no arena assertion tight enough to
    isolate the reflector scratch is worth writing (it would be a 0.7% window that any unrelated
    change to `complex.cyr` would break). The buffer-reuse hazard is guarded instead by `Q*R == A`,
    `Q^H Q == I` and R-upper-triangular assertions on a 4×4, where the buffer is rewritten three
    times after its first use.
  - **The audit's second `tensor.cyr` site is REFUTED by measurement.** The disposition column
    claimed `full_idx` was "still inside **both** contraction loops"; the one before the `out_idx`
    loop was already hoisted. A 12⁴ rank-4 contraction cost **1,272 B per call both before and after**
    this cycle's edit — there was no per-iteration allocation there to remove. Pinned as such.
  - **The audit's fourth site is unidentifiable and stays that way.** Its Site cell is truncated to
    `src/geo_` in the audit table's own text, and it is truncated identically in the commit that
    first introduced the file (`70e7c8b`), so there is no earlier revision to recover it from. A
    brace-depth scan of every file in `src/` finds **no** allocation inside a loop in `geo.cyr` or
    `geo_advanced.cyr` today, consistent with the disposition's own note that the 2.8.1 sites there
    were removed by the 2.8.3 EPA and `time_of_impact` rewrites. Reported as unresolved rather than
    guessed at.
  - The same scan surfaced **six sites the audit did not name**, all in `solve_gmres`'s restart loop
    (`src/linalg_ext.cyr:386, 390, 395, 401, 402, 488` — `v_basis`, `h`, `g`, `cs`, `sn`, `y`). They
    are **not** fixed in this pass and are not claimed to be; recorded here so the next cycle has
    them.

### Added

- **collision_core — `ColContact.tangent_vel` and `contact_new_fric`.** `friction` had no input to
  act on: the reduced model derives the entire relative velocity from `penetration` **along the
  normal**, so the tangential component was structurally zero and no correct friction solver could
  produce anything but zero from it. `ColContact` gains a ninth field (offset 64) carrying the
  signed relative sliding speed of body A w.r.t. body B along a caller-chosen unit tangent, and
  `contact_new_fric` is the 9-argument constructor that sets it. The struct grows 64 → 72 bytes;
  offsets 0–56 and the 16-bytes-per-contact `out_impulses` layout are unchanged. **`contact_new`
  keeps its 8-argument arity** and sets `tangent_vel = 0`, so every existing caller and every
  consumer is source-compatible and bit-for-bit unaffected — no tangential motion means no friction
  impulse, which is why `mu = 0` and every pre-existing solver fixture do not move by a single bit.
- **tests/modules.tcyr** — 31 assertions in a new group *"sequential_impulse friction (Coulomb
  cone)"*, the first in the repo to mention friction at all. Sticking (`lambda_t = -v_t/inv_mass`,
  tangential velocity driven to **exactly** 0), sliding (clamp binding at `|lambda_t| = mu*lambda_n`
  with `v_t` reduced 1.0 → 0.75, same sign, never reversed), sign opposition on both sides,
  iteration independence at 1/8/9 sweeps for both regimes, the cone tracking `lambda_n` through
  restitution, the three no-normal-load cases (`pen = 0`, separating, both-static) collapsing the
  cone to exactly `+0`, a wide cone reproducing the unclamped solution, per-slot independence in a
  2-contact batch, and the `mu = 0` / `contact_new` bit-identity regression guards. Every expected
  value is the closed form evaluated by hand on binary-exact fixtures (`inv_mass = 2`, `pen = 0.5`,
  `lambda_n = 0.25`, cone `0.125`), so they are bit-equality assertions, not tolerance bands.
- **tests/hisab.bcyr** — `gjk_intersect_box_miss` / `gjk_intersect_sph_miss` /
  `gjk_intersect_box_hit` / `gjk_intersect_tangent`. The most-called narrowphase entry point had no
  benchmark at all, which is the condition under which the cost above could have doubled unnoticed.
  Batch-timed (`bench_batch`), because `bench_run`'s per-call clock pair is ~240 ns against a
  ~530 ns operation.

## [2.8.4] - 2026-08-05 — the DCT/DST dispatch heuristic, and what the 23× ratio actually was

**This release does not make `num_dst` faster at n = 1024.** It stays at ~6.7 ms, and
`num_dct_1024` stays at ~276 us. That 23× ratio prompted the investigation and turned out not to
be a defect at all; the defect the investigation found is somewhere else and is small. Both are
below, in that order, because the second is the one that ships code.

### The 23× was two effects, one real and one a benchmark artifact — no code change

Measured by forcing both paths at ten sizes (`cyrius bench`, 10 iters, no concurrent load):

| | n = 1023 | n = 1024 |
|---|---|---|
| `num_dct` | 1.623 ms (Bluestein) | **276.5 us** (radix-2) |
| `num_dst` | **460.8 us** (radix-2) | 6.728 ms (Bluestein) |

A DCT-II of n points takes a length-**n** DFT; a DST-I of n points takes a length-**2(n+1)** one.
So the two transforms are fast at *opposite* sizes, and n = 1024 is simultaneously the DCT's best
case and the DST's worst: 2·(1024+1) = 2050 = 2·5²·41 is not a power of two, so `_numx_dft` falls
to Bluestein with m = 8192 — three length-8192 FFTs against the DCT's single length-1024 one.

Decomposing the ratio exactly: DCT@1024 276.5 us → DST@1023 460.8 us is **1.6×**, the irreducible
cost of DST-I needing a twice-as-long DFT; DST@1023 → DST@1024 is a further **14.6×**, purely the
size. The parity penalty reproduces at every adjacent pair — 15.2× / 14.4× / 14.7× / 14.1× / 14.0×
at 255-256 / 511-512 / 1023-1024 / 2047-2048 / 4095-4096.

That is a property of the transform, not a bug, so nothing was rewritten. What was wrong was the
**reporting**: the suite benchmarked both transforms at the single size that flatters one and
punishes the other, and printed the two rows next to each other. `num_dct_1023` and `num_dst_1023`
are now benchmarked **alongside** — not instead of — the 1024 pair, so the cliff is visible in the
suite's own output instead of being a bare ratio waiting to be misread.

### Fixed

- **num_ext — `_numx_use_fft` dispatched to the slower path at five sizes.** The gate charged every
  caller `2*len` for post-processing. That is right for DCT-II and its inverse, which compute a cos
  and a sin twiddle per output bin, and wrong for DST-I, whose entire post-pass is a negate and a
  halve — so DST-I was billed for transcendentals it never evaluates, its reduction branch priced
  at 17.4–18.0 ns per abstract unit against the direct path's 22.0–23.7, and n = 7 went to the
  direct loop when the FFT is faster. Separately, measured Bluestein cost per unit is not flat
  (26.2 ns/unit at m = 512 falling to 19.9 at m = 32768, as the setup amortises), so the flat model
  under-charged small m — the band the DCT mispicks sit in. `post` is now a per-caller parameter
  (0 for `num_dst`, 2 for `num_dct` / `num_idct`) and Bluestein carries a 10% surcharge below
  m = 128. Measured effect, both paths batch-timed at min of 10 rounds × 1000–2000 calls:

  | size | was | now | gain |
  |---|---|---|---|
  | `num_dst` n = 7 | 2189 ns (direct) | 1738 ns (FFT) | **−20.6%** |
  | `num_dct` n = 26 | 32500 ns (FFT) | 28939 ns (direct) | **−11.0%** |
  | `num_dct` n = 38 | 66141 ns (FFT) | 60713 ns (direct) | **−8.2%** |
  | `num_dct` n = 39 | 66118 ns (FFT) | 64430 ns (direct) | **−2.6%** |

  Every other n from 2 to 20000, for all three callers, keeps the pick it had — enumerated before
  landing, not sampled. The total saving is single-digit microseconds per call at four small sizes;
  this is a correctness fix to a decision function with a provably bounded blast radius, not a
  throughput win. The 1/10 surcharge and the m <= 128 bound are a **fit to two measured bands**,
  not a derivation: widening to m <= 256 drags n = 58/59/60 across a band that is currently picked
  correctly, and raising it to 1/8 flips n = 40, where the FFT is measurably faster. `num_dct`
  n = 27 remains a **3% mispick** — a fit boundary, recorded rather than papered over. The gate no
  longer errs by more than 3%; it is not exact.

### Added

- **Assertions on the dispatch decision itself.** `_numx_use_fft` is a decision function, and
  nothing asserted what it decides — only that both paths compute the right answer, which stays
  true whichever one runs. A mispick was therefore invisible: it cost time and changed no result.
  Eight assertions now pin the four corrected decisions, the two bound-defining sizes (n = 40 and
  the n = 27 residual), and the two headline sizes that must not move.
- `num_dct_1023` / `num_dst_1023` benchmarks, beside the existing 1024 pair.

## [2.8.3] - 2026-08-05 — the eight-track audit-repair merge; the narrowphase now returns the exact MTV

Discharges the bulk of the 2026-08-04 audit (42 findings). Eight parallel repair tracks were
integrated into one tree; suite **1818 → 2263** across four harnesses, constant gate 147 → **153**,
all gates green (`lint` 0 warnings, `fmt --check` clean, `vet` 2 deps / 0 untrusted, `fuzz` 1/0).

**Two tracks solved the same problems differently and only one of each could land.** `geo_advanced.cyr`
was edited by five tracks. The EPA work was taken from the track that rewrote the algorithm for
*correctness* (its rewrite removed the per-face allocation as a side effect, so nothing was lost by
dropping the track that only fixed the allocation); `time_of_impact` was taken from the track that
replaced the sampler outright; the BVH work was disjoint and merged unchanged. Every other track's
fixes are in the tree — verified file by file, not assumed.

**Known, deliberately not fixed here:** `gjk_epa_3d` and `gjk_intersect_3d` now disagree on exact
tangency and on coincident degenerate points (4 of 90 swept pairs). `gjk_epa_3d` is the correct one
— two touching convex sets do intersect — so `gjk_intersect_3d` is what needs the repair, and it is
a hot public entry point that consumers use for broadphase. It gets its own cycle rather than the
tail of this one. The test group at `tests/modules.tcyr` titled "agrees with `gjk_intersect_3d`"
exercises that agreement only on overlapping fixtures, so the suite does **not** currently police it.

### Fixed

- **geo_advanced — the narrowphase returns the exact MTV.** `mpr_penetration` kept a portal whose
  `v0` is the INTERIOR point, so the plane it refined against passed through the inside of the
  Minkowski difference: its distance from the origin is not a depth, and can be zero or negative.
  Its convergence test was unsatisfiable for a body with interior, so it also burned all 64
  iterations every call. `mpr_intersect` had false positives from the same root cause. EPA's
  expansion loop had a fixed point instead of a limit, kept coplanar faces while removing their
  neighbours, and used an ABSOLUTE convergence test (`< 1e-12`, unreachable at scene scale). GJK's
  own degeneracy tests were absolute on a SQUARED length. Measured against an **exact** reference —
  never a previous hisab output: 15-axis OBB SAT in `python3` `Fractions` for box/box, closed forms
  for sphere/sphere and sphere/box, independent Nelder-Mead minimisation of the support function
  elsewhere — over 862 configurations in 35 shape classes:

  | function | agreement before | agreement after |
  |---|---|---|
  | `mpr_penetration` | 66 / 455 (216 wrong sign) | **862 / 862** |
  | `gjk_epa_3d` | 415 / 455 | **862 / 862** |
  | `mpr_intersect` vs exact overlap | 9 false positives / 862 | **0 / 862** |

  Worst relative error after: **1.6e-9**.
- **geo_advanced — `time_of_impact` missed real collisions.** It advertised conservative advancement
  but was a fixed-step **sampler**: it walked `t` in equal steps asking a *boolean* overlap test, and
  a boolean carries no information about separation, so the step could only come from the horizon
  while the overlap window in `t` is only ~`2R/|v_rel|` wide. Replaced with real conservative
  advancement over a new GJK **distance** sub-algorithm (Voronoi-region simplex closest point).
- **geo_advanced — `gjk_epa_3d` contradicted its own sibling and left both out-params unwritten.**
  `alloc()` hands back zeroed pages, so a path that returned without writing published the NULL
  `HVec3` into the caller's slot and the first accessor faulted. Every return path now writes both;
  on a `0` return the normal is a documented placeholder, not a contact normal. `time_of_impact`
  had the same defect on two paths.
- **geo_advanced** — `gjk_epa_3d` reported "no contact" for shapes in EXACT TANGENCY. Two convex
  sets touching at a point put the origin ON the boundary of their Minkowski difference, and GJK's
  containment test is strict, so a unit sphere at the origin against a unit-half box centred at
  x = 2 came back 0 instead of a depth-0 contact with the line of centres as its normal. Two causes,
  both now addressed on the path where GJK has already answered "no":
  - the strict containment test itself, and
  - a **manufactured separating certificate**: GJK's search direction is unnormalised and its
    magnitude collapses as the simplex closes on a boundary origin (5.9e-15 by the fifth iteration
    on this fixture). A support function written the obvious way — `centre + r * hvec3_normalize(dir)`
    — then returns the shape's *centre*, because `hvec3_normalize` returns the zero vector below
    `EPSILON_F64`. The point that comes back is not a support point, and its negative dot with the
    direction reads as a separating plane.

  New `_epa_touch_probe` re-asks the question with **unit** search directions and answers with a
  certificate pair, both read off an EXACT ZERO and never off a tolerance: `h_M(n) < 0` proves
  strict separation (the same certificate `_gjk_core_3d` exits on), `h_M(n) == 0` at the converged
  minimiser proves the origin is on the boundary, i.e. a touch of depth exactly 0. `_gjk_core_3d`
  is unchanged. Verified over a 1,480-configuration sweep (sphere/box both operand orders,
  box/box face, edge and vertex tangency, sphere/sphere; ±x, ±y, ±z; scales 1e-6, 1, 1e3 and a
  non-dyadic 7/3; gaps stepped by decades to 1e-18 and by 1 to 12 ulps): **190/190 exact tangencies
  now report a depth-0 contact (was 114/190) and 0 of 1,290 strictly separated pairs report contact
  (unchanged)**. Every case whose verdict did not change is bit-identical, normal and depth.

- **geo_advanced — a manufactured probe direction certified a touch it had not proved.**
  `hvec3_length_sq` SQUARES, so for two zero-volume shapes separated by less than
  `sqrt(DBL_MIN) ~ 1.5e-154` the search direction underflows to exactly 0; the degenerate-direction
  repair then substitutes a cardinal axis, whose support value is exactly 0, and that read as a
  supporting plane through the origin. 64 genuinely separated point-vs-point pairs reported contact.
  A manufactured direction is now settled against the geometry by probing the six cardinal axes
  (exact unit vectors, immune to the underflow); rejecting it outright was tried first and cost the
  legitimate coincident-shape case, which is a real depth-0 touch. Both are now pinned by assertions.
- **geo_advanced — `bvh_query_ray` silently discarded whole subtrees.** `geo_ray_aabb` returns the
  hit parameter, and a `t = 0` hit — the ray origin inside the box — was being read as a miss.
- **collision_core — `sequential_impulse` never converged for restitution >= 1.** The `(1 + e)`
  factor multiplied the *effective* velocity, already carrying the accumulated impulse, so
  restitution was re-applied every sweep. At `e = 1` the update is a period-2 orbit: an **even**
  `iterations` returned zero impulse and an **odd** one twice the right answer, so the result
  depended on the parity of a convergence parameter. Bit-identical at `e = 0`, the only case the
  previous assertions covered.
- **collision_mesh — `halfedge_from_triangles` aborted the process** instead of returning its
  documented 0.
- **calc_ext — `calc_monotone_cubic` was not monotone.** Fritsch-Carlson has two parts and only the
  `α² + β² <= 9` circle was implemented; the circle test squares its arguments, so it is blind to a
  tangent pointing *against* its own secant. Scored against exact-rational extrema of the Hermite
  cubic: **58,647 of 100,000** random sets containing an extremum left their own `[y_k, y_k+1]` box,
  overshooting by up to **57.6**. After: **0** on the same 100,000, and bit-identical on 60,000
  strictly monotone sets. The function previously had **no assertions at all**.
- **linalg_precision — `svd_golub_kahan` accepted wide matrices.** The `m >= n` precondition was
  documented and enforced nowhere, so a wide matrix ran the whole decomposition and returned
  `HSB_ERR_NONE` with numbers that are not its singular values — structurally impossible, since
  `out_U` is m×n and a rank-m matrix cannot carry n orthonormal columns. Measured against the exact
  identity `Σσᵢ² == ‖A‖_F²`, which needs no reference implementation: **200 of 200** wide matrices
  violated it silently; 0 of 200 tall ones did. Now rejects `m < n` and non-positive dimensions, and
  guards NULL handles.
- **interval — `ivl_div` returned a finite `[-1e9, +1e9]` for a zero-straddling divisor**, which is
  not an enclosure of the true result.
- **einsum — a repeated OUTPUT label was accepted** and returned a confidently wrong tensor.
- **spatial / geo_advanced — an integration defect that no textual conflict revealed.** Two tracks
  changed `spatial_hash` in ways that merged cleanly and crashed: one replaced the per-bucket vecs
  with an intrusive chain (i64 heads, `-1` empty, entries in one flat vec with a `next` link), while
  the other added a full-scan query that read those slots as vecs. `vec_len` on a chain head is a
  wild pointer. The scan now walks the flat entry list, which under the new layout needs no bucket
  traversal at all. Its crossover constant — derived from the old FIXED 1024 buckets — was likewise
  replaced by a direct cost comparison; left alone it would have full-scanned 4,194,304 buckets to
  avoid 1,331 probes.
- **Memory-safety and bounds guards** across `calc_ext`, `complex`, `linalg_ext`, `num_ext`,
  `tensor` and `spatial`: NULL-handle checks on the capped constructors, `tensor_contract` index
  validation against the rank, `calc_partial_derivative` bounding BOTH ends of its index, and
  `num_halton` rejecting bases below 2 (it previously raised SIGFPE at base 0 and never returned at
  base 1).

### Performance

- **spatial** — `spatial_hash_*` bucket count now GROWS with the data instead of sitting at a fixed
  1024, so the average chain stops being O(n) in a large scene.
- **geo_advanced** — `bvh_build` no longer recomputes centroids per level or allocates per node.
- **num_ext** — `num_dct` / `num_idct` / `num_dst` / `num_idst` gained O(n log n) reductions,
  replacing the O(n²) transforms.
- `gjk_epa_3d` on the NO-CONTACT path costs the probe: 0.65 us → 1.24 us per call and 344 → 656
  arena bytes per call (box@2 vs box@9, 16,000 calls, median of 9 runs). Contact paths are
  untouched. No benchmark in `tests/hisab.bcyr` covers a separated pair, so this number comes from
  a standalone harness rather than `bench-history.csv`.

  The three algorithmic wins above are **not** quoted with before/after numbers. Each track measured
  in its own worktree under parallel load, and those runs disagree by up to 40× on code neither of
  them touched (`vec3_add` 79,550 vs 3,205,000 ns for the identical function), so no worktree figure
  is admissible. Re-measurement on the integrated tree is owed before any of them is claimed.

### Added

- **The test-quality tier of the 2026-08-04 audit (7 findings)** — assertions that passed against
  broken code, now replaced by ones that discriminate: `calc_hessian`'s 4-point mixed-partial cross
  stencil, `calc_catmull_rom` (tested only at t = 0 and t = 1 on collinear equally-spaced points,
  where the spline is pinned by construction), `calc_adaptive_simpson` (whose only integrand was x²,
  which one unrefined Simpson panel integrates exactly), `ode_bdf` (order 4 with `f == 0` and a
  constant history — a fixed point of any consistent method), `se3_adjoint` (passed a 24-byte
  `HVec3` where a 48-byte twist is required), `so3_from_mat3`, and `delaunay_2d` winding.
- `se3_twist_new` / `se3_twist_omega` / `se3_twist_v` — the exported way to build and read a twist.
- New assertions pinning: EPA out-param contract on every return path, exact tangency in both
  operand orders, coplanar flat boxes, crossing segments, coincident zero-volume shapes, the
  1e-165 separation that must NOT read as contact, and `spatial_hash_query_radius` being bounded by
  the grid rather than by the caller's radius.

### Changed

- **Breaking (narrow):** `so3_from_mat3` now returns 0 for a matrix that is not a rotation. It
  documented "checks determinant ~ +1 and orthogonality" and performed neither, so callers that
  relied on it silently accepting a non-rotation will now see the documented failure return.

## [2.8.2] - 2026-08-04 — delaunay completeness via symbolic ghost vertices. **Two critical repairs rejected.**

One of the four criticals fixed; two were implemented, adversarially verified, and **rejected for
regressing other input classes**. Suite 1801 → **1818**.

### Fixed — delaunay_2d is complete

The audit proved no super-triangle multiplier wins both classes: 10x leaves holes on 15–37% of
uniform-random input, and raising it breaks tight clusters. The multiplier *is* the coupling to the
non-adaptive in-circle predicate.

**So the super-triangle is gone.** Vertices n, n+1, n+2 are now **symbolic ghosts** — the limit of
`A + M·u_k` as `M → ∞`, with `u = (1,2), (-2,1), (1,-2)` and apex `points[0]`. Nothing is stored for
a ghost. Every predicate touching one is decided by the sign of the leading nonzero coefficient of
its determinant expanded as a polynomial in `M`, i.e. its sign for all sufficiently large finite
`M`. A build evaluates finitely many predicates, so a single `M` satisfies all of them at once —
the predicate set is realisable by an actual configuration, which is what keeps the walk and the
flood fill consistent.

The equal-norm requirement on the direction vectors is load-bearing and was found by measurement,
not by reasoning: with `|u_k|²` all equal to 5 the two-ghost tie coefficient collapses to a plain
distance comparison against the apex. The obvious set `(-1,-1), (1,-1), (0,1)` was tried first and
its tie rule **failed 1,429 of 35,968 exact trials**.

Verified against exact rational arithmetic — every coordinate rescaled from its bit pattern to exact
integers, with a completeness certificate (soundness + exact hull-area partition + manifoldness +
Euler count). Over 769 point sets in 22 classes: **534/769 → 729/769 fully correct, 0 regressions,
195 improvements.**

| class | before | after |
|---|---|---|
| uniform-unit n=160 | 11/40 | **40/40** |
| uniform-unit n=500 | 0/5 | **5/5** |
| cluster span 1e-14 | 24/40 | **40/40** |
| cluster span 1e-20 | 0/40 | **40/40** |
| thin strip, aspect 1e6 | 0/40 | **40/40** |

The last two rows matter beyond this finding: the 1e-20 cluster and the aspect-1e6 strip were
*separately filed* defects (746 unsound triangles and 173 dropped points; 2,701 triangles short and
1,478 dropped points). Removing the super-triangle fixed them as a side effect, because they were
the same coupling seen from the other end.

Also verified independently of the implementation: **57,600 exact-rational comparisons** of every
ghost rule against the true determinant at M = 1e5 … 1e34, including all four degenerate branches
deliberately constructed. Zero failures. `delaunay_2d_400` 3.56 → 3.45 ms.

**This supersedes the adaptive-predicate work item.** There is no multiplier left to unblock.

### Rejected — two critical repairs that regressed

Both were reported FIXED by their implementer, both removed the defect they targeted, and both were
**rejected on independent verification**. Recorded because the rejections are the useful part.

- **`gjk_epa_3d` O(k²).** The repair works: face evaluations 8,576 → 812 at the shipped budget,
  4.36 ms → 0.70 ms, arena 1.23 MB → 99.6 KB, and all three metrics go quadratic → linear. But its
  flood-fill visible-set search **under-reports when rounding splits the visible region into
  disconnected components** — 239 of 47,134 cases — and every divergence coincides with one. A
  deterministic reproducer (cylinder vs rotated box) returns a depth **40.7× too small** where the
  shipped code is correct. The verifier also found the CHANGELOG, roadmap and source comment it
  shipped all claimed "0 regressions", which its own 104,000-configuration sweep contradicts.
- **`mpr_penetration`.** Rejected on the same grounds — removes the defect, regresses elsewhere.

Both are back on the roadmap with the verifiers' specific remedies, not as blank retries.

### What the rejections say about the method

Three of the last four performance or correctness rewrites in this repo were reported working and
were not. The pattern is consistent: an optimisation that replaces a full scan with a smarter search
inherits an invariant the full scan never needed, and the new invariant fails on degenerate input.
The full scan is slow precisely because it assumes nothing.

## [2.8.1] - 2026-08-04 — full audit of the v2.8.0 tree. **42 findings, 4 critical.**

An audit release: no library code changes. Six dimensions — memory-safety, numerical,
api-consistency, performance, test-quality, and a regression check that the 2.6.12–2.8.0 repairs
still hold — each swept by an independent agent and then **adversarially verified** by a second
that reproduced every finding with its own harness before it entered the report.

**42 confirmed, 0 refuted** (4 critical, 17 high, 16 medium, 5 low). Full report at
[`docs/audit/2026-08-04-v2.8.0-full.md`](docs/audit/2026-08-04-v2.8.0-full.md); the work is
scheduled across 2.8.2–2.8.5 in the roadmap.

The zero-refutation rate is not a compliment to the finders. Verifiers were required to reproduce
by **execution**, so speculative findings died inside the verify step rather than arriving as
refutations. Two findings were downgraded on verification and one materially corrected.

### The headline is a defect in what 2.8.0 shipped

`delaunay_2d` was rewritten in 2.8.0 with triangle adjacency and verified across 298 fixtures. It
is still **incomplete on ordinary uniform-random input** — 15–37% of point sets come back missing at
least one triangle, silently, no error signal. At n = 2000, eight triangles short.

The cause is not the rewrite. `d_max = max(dx, dy) * 10` builds a super-triangle only 10x the
bounding box, and Bowyer-Watson reduces to the true Delaunay triangulation only once no super-vertex
lies inside any real circumcircle. At 10x that fails, the real triangle is never created, and
compaction drops the super-vertex triangle covering the gap. Proven in exact rational arithmetic:
across 40 point sets, the triangles whose circumcircle encloses a super-vertex matched the observed
missing set **40 of 40**.

**And the obvious fix is wrong.** Raising the multiplier fixes uniform data and *breaks* clustered
data: at 1e9, a 1e-8-span cluster loses 20 triangles and gains 9 spurious overlapping ones, where
the current 10x is exactly correct. The multiplier is a genuine trade-off against the non-adaptive
in-circle determinant's conditioning — the two defects are **coupled**, precisely as
`issues/archived/2026-08-04-incircle-precision.md` warned when it said not to fix that one by rescaling. 1e6
is a defensible interim; 1e9 must not be used; the durable fix is an adaptive predicate.

Three of the four criticals are in the collision narrowphase: `mpr_penetration` returns a wrong and
frequently negative penetration depth (65 of 81 box/box configurations disagree with the exact MTV,
27 with the wrong sign), and `gjk_epa_3d` is O(k²) in EPA iterations — 8,576 face evaluations and
1.24 MB of never-freed arena per call — while also returning wrong depths for sphere-vs-box.

### What this says about the last five releases

Every one of these 42 lived in code that 1801 assertions and 97% function coverage did not catch.
Coverage counts whether a function is *referenced*, not whether its contract is *checked* — the
report labels it "a floor, not a correctness proof" for exactly this reason. The test-quality
dimension found the sharpest example: **every ODE test integrand is autonomous**, so the entire
DOPRI45 abscissa row is certified by nothing.

### Not regressions

The regression dimension re-verified the 2.6.12–2.8.0 repairs by execution and found them all
holding: half-edge twin pairing, BVH degenerate split, interval enclosures (48,000 checks per
operator), FFT/IFFT round-trip and input preservation, `f64_fmod`/`f64_copysign` over 3,000 random
pairs, the SU(2)/SO(3)/SE(3) angle conventions over 600 axis/angle pairs, `geo_ray_plane`/
`geo_ray_capsule` over 2,000 rays, and the k-d tree split plane. One apparent SE(3) failure was the
verifier's own test bug and was refuted rather than reported.

## [2.8.0] - 2026-08-04 — delaunay_2d adjacency rewrite; it was **wrong**, not just slow

`delaunay_2d` now carries triangle adjacency and inserts by **walk + flood-fill Bowyer-Watson**,
replacing the flat-vec full rescan. Two previous attempts to speed it up failed (2.7.1: 2.6x slower
on cocircular; 2.8.0-attempt: broke the triangulation) — both are written up in
`docs/development/issues/archived/2026-08-04-perf-delaunay_2d.md`, and both failed because they optimised a
representation that could not support the operation.

### The headline is a correctness defect, not the speedup

The premise behind the earlier attempts — "cocircular input is inherently expensive, the bad set
really is most of the triangulation" — was **wrong**, and I had accepted it. The shipped code was
not producing a triangulation at all on that class. At 150 points on a circle it returned **1651
triangles where 148 is correct**, with **206 edges shared by three or more triangles**, **9 input
points dropped**, and a total area **244x the convex hull**. It was overlapping soup.

The full rescan flagged far-away triangles through cancellation in the non-adaptive in-circle
determinant, so Bowyer-Watson kept creating spurious triangles. A flood fill cannot reach them, so
the cavity stays small — which is why the rewrite is both correct *and* faster here. The earlier
attempts were adding per-triangle work on top of the garbage.

**Nothing in the suite asserted any of this.** The eight existing `delaunay_2d` assertions all used
fixtures where the shipped code happened to be right.

| Benchmark | Before | After | |
|---|---|---|---|
| `delaunay_2d_400` (uniform) | 14.54 ms | **1.66 ms** | **−88.6%** |
| **`delaunay_2d_circle_150`** (cocircular) | **600 ms** | **455 µs** | **~1300x** |
| n = 1600 uniform | 222 ms | 7.9 ms | −96.5% |

Growth per doubling went from **3.9x (O(n²))** to **2.1x (O(n log n))** — the exponent changed, not
the constant. Arena use per call fell too: uniform n=1600 2090 → 1632 KiB; cocircular n=300
20118 → 329 KiB, a 61x reduction under an allocator that never frees.

`delaunay_2d_circle_150` is now a permanent benchmark. Its absence is exactly why the 2.7.1
regression would not have been caught.

### Method

Three independent implementations were built in isolated worktrees and each was checked by a
separate adversarial verifier that wrote its **own** differential harness — the issue file's
standing instruction after attempt 2 wasted a cycle on suite-level counts that localise nothing.
All three passed. The merged one (attempt C) was verified across **298 fixtures**: identical
canonical triangle sets everywhere the shipped output was valid, and on the 55 divergent
cocircular/near-cocircular fixtures the new output satisfies count = 2n−2−h, area = hull area,
zero non-manifold edges, zero dropped points, while the shipped output satisfies none of them.
**There is no input where the shipped code is valid and the rewrite is not.**

### Behaviour change

On exact/near-cocircular convex input the output set changes completely (circle n=60: 472 triangles
→ 58). That is a change from an invalid overlapping non-triangulation to a valid one, but a consumer
mapping triangle indices to per-index attributes will read a different mesh.

*Evidence:* five new assertions pin count, point-usage and manifoldness. Against the shipped code
they fail with 472-vs-58 triangles, 2 dropped points and 48 over-shared edges. Suite → **1801**.

### Coverage

`cyrius coverage` **94% → 97%** (570/587), all 35 files referenced. +31 assertions across the calc,
num_ext, optimize, lie_ext and linalg_ext tails. Four more of my own type errors were caught by the
gates — `pcg32_new` arity, `lorentz_adjoint` taking a Mat4 generator rather than a 4-vector, and
`bch_2nd_order` taking complex matrices rather than HVec3 (that one **SIGSEGV**'d, found by
bisecting the crashing suite).

## [2.7.3] - 2026-08-04 — coverage **80% → 94%**, all 35 files referenced

Target was 90%; the result is **94%** (555/587 functions). **Files referenced 34/35 → 35/35** — the
last one was `main.cyr`, previously excluded by design as the CLI smoke binary. Suite
**1677 → 1765**.

+88 assertions across the remaining untested surface, every expected value a closed form:

- **autodiff** — `dual_div`/`dual_pow`/`dual_abs` checked on *both* the value and the derivative
  (`d/dx x³ = 12` at x=2, `d/dx |x| = −1` at x=−5).
- **tensor** — `get`/`set`/`scale`/`contract`, including the rank-2 trace.
- **interval** — `ivl_exp` enclosure ordered and anchored at `e⁰ = 1`.
- **calc_ext** — partial derivative, gradient, Jacobian and Hessian of `x²+y²` against their exact
  values (`∇ = (6,8)`, `H = 2I`), and adaptive Simpson of `x²` to 1/3.
- **CGA** — the algebra identities rather than existence: `x ∧ x = 0`, `clone` preserving blades,
  `sub` of identical multivectors having zero norm, `scale` scaling.
- **symbolic_ext** — the whole `pat_*` constructor/accessor family plus `sym_rule`/`sym_rewrite`.
- **transforms / mat3 / vec3 / lie** — `t2d`/`t3d` accessors and composition, `m3` arithmetic,
  `hvec3_project`/`reject`/`angle`, `lorentz_boost`/`rotate` validated as genuine Lorentz
  transforms.

### Four more of my own errors, all caught by gates

Same pattern as 2.7.2 and worth the same note — every one was a *calling-convention* mistake that a
looser test would have hidden:

- **`num_crt` takes vecs, not raw `alloc()` buffers** — it reads them with `vec_get`. Raw buffers
  **SIGSEGV**; that is how the assertion was found, by bisecting a crashing suite down to one call.
- **`dual_pow`'s exponent is a plain f64, not an integer** — a raw `3` is the subnormal 1.5e-323, so
  `2³` silently returned 1.
- **`t2d_new` takes `(pos, rotation, scale)` with HVec2 pos/scale**, not five scalars; and
  **`hquat_from_euler` takes an explicit order argument**.
- **A `v3a` name collision.** `foundation.tcyr` already had that global; `cyrius lint` reported
  *"global var init refs 'v3a' declared at line 1043 (silent zero at init)"* — my later declaration
  would have silently zeroed the earlier uses. Renamed the whole block behind a `c73_` prefix. This
  is the flat-namespace hazard that 2.7.0-H added a gate for, hitting a **test** file this time.

`f64_pow` goes through exp/ln, so `2³` is 7.999… — that assertion uses a tolerance, not `f64_to`
equality, and the comment says why.

## [2.7.2] - 2026-08-04 — **coverage target met: 80%**

`cyrius coverage` **76% → 80%** (474/587 functions), the target set at the start of the 2.6.x audit
arc and carried through eleven releases. Suite **1643 → 1677**.

+34 assertions closing the last two large holes:

- **`lie_ext` SO(3)/SE(3)** — group identities, which are exact and need no tolerance: `R·R⁻¹ = I`
  on all nine entries, `|log(R)|` equal to the rotation angle, `(g·g⁻¹)` fixing an arbitrary point,
  `se3_identity` fixing every point, `se3_to_mat4`/`from_mat4` round-tripping, and BCH reducing to
  plain addition for parallel (commuting) arguments.
- **`num_ext` number theory** — exact integer answers: `extended_gcd(240,46) = 2` *and* its Bézout
  identity, `modinv(3,11) = 4`, `φ(36) = 12`, `σ₀(28) = 6` / `σ₁(28) = 56` (28 is perfect), CRT
  `(2 mod 3, 3 mod 5, 2 mod 7) → 23 mod 105`, `84 = 2·2·3·7` as four factors, Pollard-rho returning
  a real divisor of 8051, and Halton `(1, base 2) = ½` exactly.

### Three of my own errors, caught by the gates rather than by review

Worth recording because all three were *type* mistakes that a passing suite would have hidden:

- **`so3_from_quat` takes an SU(2) element, not an `HQuat`** — it calls `su2_to_quat` internally.
  Passing an `HQuat` silently reinterprets it and yields a **double-angle** rotation: `R[0][0]` came
  back `−1` (cos 180°) where `0` (cos 90°) was meant. This looks exactly like the half-vs-full-angle
  defect repaired in 2.6.13, and I initially took it for a recurrence of it. It is not — the library
  is correct and the call was wrong.
- **`se3_new` takes an SU(2) rotation, not a `Mat3`** — same family, same symptom.
- **`num_crt` takes vecs, not raw `alloc()` buffers** — it reads them with `vec_get`. Passing raw
  buffers **SIGSEGVs**, which is how that assertion was first written and how it was found.

None is a library defect. All three are recorded because "the test crashed" is the cheap outcome;
the expensive one is a test that passes against the wrong types and certifies nothing.

### Still open — carried to 2.8.x

`delaunay_2d` needs a data-structure change, not an optimisation (see 2.7.1 for the measurements
that rejected the x-sweep), and `_col_in_circumcircle`'s adaptive predicate remains latent at
0/180 under real geometry.

## [2.7.1] - 2026-08-04 — performance landings, coverage, and a rejected optimisation

Suite **1607 → 1643**, `cyrius coverage` **71% → 76%** (448/587 functions).

### Performance

| Benchmark | Before | After | Change |
|---|---|---|---|
| `triangulate_600gon` | 9.93 ms | **1.59 ms** | **−84%** |
| `kdtree_build_4k` | 2.27 ms | 2.54 ms | +12% (see below) |

**`triangulate_polygon`** now skips convex vertices when testing ear containment — only a *reflex*
vertex can lie inside a candidate ear. Unblocked by the 2.7.0-K contract documentation: it diverges
only on self-intersecting input, which ear clipping does not define, and the divergence direction is
old-partial → new-complete.

> ⛔ **CORRECTION (2.20.0): the direction above is BACKWARDS.** Measured by building the 2.7.0 tree
> and the current tree side by side on cyrius 6.6.2 and running both on
> `(0,0),(1,2),(1,1),(0,2),(2,1)`: **old returns 9 indices — COMPLETE — and new returns 6 — PARTIAL**
> (old `[0 1 2 4 0 2 4 2 3]`, new `[4 0 1 1 2 3]`). The new code *bails where the old succeeded*, not
> the reverse. The claim is stated backwards here and again ~50 lines below; the *same release*
> already had it right elsewhere, so this was a transcription error, not a measurement error.

**`_kd_partition`** gained a balance guard rather than the unconditional quickselect the spec
proposed. That cost **+114%** on the common path (2.27 → 4.85 ms), so selection is now a *fallback*
taken only when the cheap midpoint split leaves a child below a quarter of its parent. Depth is
bounded at log₍₄∕₃₎(n) for +12%.

### `delaunay_2d` — implemented, measured, and REJECTED

The x-sweep-with-retirement rewrite was applied in full and measured on both input classes:

| Input | Shipped | x-sweep | |
|---|---|---|---|
| uniform scatter, n=400 | 15.09 ms | **5.37 ms** | −64% |
| **cocircular, n=150** | **615 ms** | **1616 ms** | **2.6× SLOWER** |

A pay-for-itself guard was then added — disable retirement once it has demonstrably failed to
retire anything — and **it did not help**: still 1.615 s. The cost is not the retirement scan. On
cocircular input Bowyer-Watson creates O(n²) triangles and each pays a `_col_circum_max_x` (with a
`sqrt`) **at creation**, before any switch can know whether retirement will pay.

So the optimisation is **reverted**, not shipped behind a flag. `delaunay_2d` stays O(n²).
Fixing it properly needs a different data structure — triangle adjacency for a walk-and-flood-fill
Bowyer-Watson — not a bolt-on to the flat vec. That is 2.8.x work, and the measurements above are
the specification for it.

### Coverage

+36 assertions closing the largest remaining holes: complex scalar ops (`cx_ln`, `cx_sqrt`,
`cx_sin`/`cos`, `cx_from_polar`, Pauli/Dirac matrices), geometry (`geo_line_*`, `geo_segment_*`,
`geo_obb_*`, barycentric, closest-point-on-triangle), and the symbolic accessors. Every expected
value is a closed form.

## [2.7.0] - 2026-08-04 — re-audit, repair, and the defects the repairs uncovered

Closes the 2026-08-04 re-audit. **35 of its 41 numbered findings are fixed**; the six that are not
each carry a written reason and a filed spec. Suite **1127 → 1605** (+42%), `cyrius coverage`
**59% → 71%**, benchmarks 28 → 35, constant gate 110 → 141 verified plus a new duplicate-global
check.

**Every fix is mutation-proven** — the source file is reverted and the new assertions confirmed to
fail — except two that are labelled rather than glossed: the BVH split (a cost, not a wrong answer,
so a benchmark guards it) and the incircle investigation (not fixed at all).

#### Performance — two of the three withheld optimisations now landed (2.7.0-O)

They were withheld in 2.7.0-I because each regressed on some input class. Both regressions are
addressed rather than accepted.

**`triangulate_polygon` reflex-only ear pruning.** Only a *reflex* vertex can lie inside a candidate
ear, so the containment scan skips convex ones.

| Benchmark | Before | After | Change |
|---|---|---|---|
| `triangulate_600gon` | **9.93 ms** | **1.59 ms** | **−84%** |

The blocker was that output length changes on *self-intersecting* input. That is now settled rather
than tolerated: 2.7.0-K documented the real contract (`== 3*(n-2)` complete, `< 3*(n-2)` partial),
and ear clipping is only defined for simple polygons anyway. The divergence direction is
**old-partial → new-complete** — the new code succeeds where the old bailed.

> ⛔ **CORRECTION (2.20.0): backwards.** Re-measured by building both trees on 6.6.2:
> `(0,0),(1,2),(1,1),(0,2),(2,1)` gives **old 9 (COMPLETE) → new 6 (PARTIAL)**. The new code bails
> where the old succeeded. ⚠ This does not change the *decision* — the divergence is confined to
> self-intersecting input, which ear clipping does not define, and a partial return is inside the
> documented contract (`< 3*(n-2)`). What was wrong is the sentence, twice, for thirteen releases.

One of my own 2.7.0-K assertions failed on this, and it was the *assertion* that was wrong: it
pinned `< 9` for a self-intersecting pentagon, i.e. it asserted that this particular input bails.
That is pinning an implementation detail, not the contract. Re-pinned to `<= 9`.

**`_kd_partition` balance guard.** Unconditional quickselect fixed the distribution sensitivity but
cost **kdtree_build_4k 2.27 ms → 4.85 ms (+114%)** — far worse than the +28–30% the review
predicted, and the wrong trade for a case most inputs never hit. Made it a *fallback* instead: the
cheap value-midpoint split runs first, and exact median selection is taken only when the midpoint
leaves a child below a quarter of its parent. That bounds depth at log₍₄∕₃₎(n) — ~24 levels at
n = 10⁶ — for **+12%** on the common path.

| Benchmark | Midpoint only | Unconditional select | **Hybrid (shipped)** |
|---|---|---|---|
| `kdtree_build_4k` | 2.27 ms | 4.85 ms | **2.54 ms** |

**What I did not verify:** the finding's headline case — 574 ms vs 38 ms at n = 50,000 on
geometrically-spaced coordinates — **did not reproduce**. My probe used a ratio so close to 1 that
it degenerated into the coincident case the `lo == hi` path already handled, and hybrid and
midpoint-only timed identically (0.34 s). So the balance guard is justified by the depth bound it
provably enforces and by the plane invariant holding under geometric spacing (now asserted), **not**
by a measured win on that input. The 574 ms figure remains unconfirmed.

`delaunay_2d` stays withheld: its rewrite is 1.9–3.0x *slower* on cocircular input and changes the
triangulation of a unit square, and neither is addressed by a guard.

### The headline is not the audit's findings — it is what looking for them turned up

Four defects in this release were found by *checking that a repair preserved behaviour*, not by any
audit:

- **`kdtree_within_radius` returned the wrong count on 274 of 400 queries.** `split_val` was read
  from `items[split]` — some right-side value — while the partition was made at `mid_val`. 3,412
  points sat on the wrong side of an ancestor's pruning plane. Shipped for three releases; the
  2.6.14 kd-tree assertions pass happily on such a tree.
- **`solve_gmres` overshot its documented matvec budget without bound** — it converted a budget
  into an outer-cycle count while spending `1 + m` per cycle.
- **`triangulate_polygon` documented a return length it does not always deliver.**
- **Two findings had never been scheduled at all** — surfaced only by assigning a disposition to
  every numbered finding rather than reasoning from the tier summaries. That is the same failure
  mode this re-audit was written to correct, recurring inside the release correcting it.

### Corrections to this document, made before release

Recorded because the corrections matter more than the claims they replace:

- The incircle finding was filed **red** on the claim that `delaunay_2d`'s super-triangle makes
  every early insertion a mixed-magnitude test. The super-triangle is at **10x** the extent;
  re-measured with the real geometry, **0 of 180** cases disagree with exact arithmetic. Downgraded
  to latent.
- The duplicate-globals finding was first dismissed as overstated on the strength of a `grep` whose
  pattern assumed one space before `=`. All four globals really were duplicated. The gate found
  what the grep missed.
- The private-symbol coupling was reported as five pairs including a cycle. It is **six pairs and
  acyclic**; the two "extra" edges were comment-only mentions.

### Deliberately not applied

Three performance optimisations are confirmed real complexity wins and are **withheld**, each with
its measurement and review filed under `docs/development/issues/`:

- `delaunay_2d` x-sweep — **−96% at n=6,400**, but 1.9–3.0x *slower* on cocircular input and it
  changes the triangulation of a unit square.
- `_kd_partition` median selection — **+28–30%** on duplicate-heavy skewed data.
- `triangulate_polygon` reflex-only pruning — sound on ~33,000 simple polygons, but output length
  changes on self-intersecting input.

A faster wrong answer is worse than a slow right one.

### CORRECTION — the 2026-08-03 audit was **not** fully discharged
The 2.6.15 entry below claims it was. That claim is **wrong**, and the 2026-08-04 re-audit
([`docs/audit/2026-08-04.md`](docs/audit/2026-08-04.md)) found it. All four P-tiers *were* closed,
but six of the original 70 findings never appeared in those tiers and so were never scheduled:
`geo_ray_capsule`'s NaN miss, `cmat_exp`'s non-terminating scaling loop, `geo.cyr`'s ray-miss
contract, `f64_fmod`'s floor-vs-trunc, `f64_copysign`'s negative zero, and `lorentz_is_valid`'s
absolute tolerance. The repair work was scheduled from the roadmap's P0–P4 *summary bullets* — a
deduplicated digest — rather than from the audit's finding list, and "all tiers closed" was then
read as "all findings closed". **An audit's finding list is the unit of disposition.**

### 2.7.0 — re-audit & refactor (in progress)
The re-audit itself came back **clean on regression**: every 2.6.12–2.6.15 repair holds under
execution. It confirmed 42 new findings (4 high, 21 medium, 17 low; no criticals) — see
[`docs/development/roadmap.md`](docs/development/roadmap.md) §2.7.0. Two were fixed immediately
because they are gate infrastructure: `scripts/check-constants.sh` was silently skipping **35 of
145** constants (its regex rejected `_` digit separators) while printing "110/110 verified", and
the constant that exposed (`symbolic_ext.cyr:213`, encoding ~1e16 against a documented 1e15).

#### Fixed — the six carried-over findings (2.7.0-A) *(all mutation-proven)*

Every fix below was verified by reverting its source file and confirming the new assertions fail;
the failing output is the evidence, not the passing output. Suite 1127 → **1154**.

- **`f64_fmod` computed a floored modulo, not `fmod`.** It used `f64_floor` on the quotient where
  IEEE-754 `fmod` truncates. The two agree only when the quotient is non-negative, so the result
  carried the sign of the **divisor** instead of the **dividend**: `fmod(-7, 3)` returned `+2`
  where C returns `-1`, and `fmod(7, -3)` returned `-2` where C returns `+1`. Now `f64_trunc`.
- **`f64_copysign` could not see, or produce, a negative zero.** It branched on `f64_lt(y, 0)` — an
  *ordered* comparison — but `-0.0` compares equal to `+0.0`, so `copysign(3, -0.0)` returned `+3`,
  and `f64_neg` of a positive zero meant the function could never return `-0.0` at all. Replaced
  with the bit splice IEEE-754 actually specifies: `(x & ~signmask) | (y & signmask)`.
- **`geo_ray_capsule` returned NaN for any ray parallel to the capsule axis.** In that
  configuration `d_perp` is exactly the zero vector, so `a == 0`, `b == 0`, and the discriminant is
  exactly `+0.0` — which does *not* satisfy the `disc < 0` test guarding the cap-sphere fallback.
  Execution fell through to the quadratic and evaluated `0.5 / 0 = +Inf`, so a function documented
  to return `0` on a miss returned a NaN that then propagated into caller arithmetic. The parallel
  case now joins the cap-sphere path, which is the whole answer for it.
- **`geo_ray_plane` returned `-1` on a miss, against its own documented contract.** `geo.cyr` states
  "returns 0 on miss" in both the file header and the ray-section header, and every sibling
  intersection routine does exactly that. The raw i64 `-1` is `0xFFFFFFFFFFFFFFFF` — a **negative
  NaN** as an f64 — detectable only by integer comparison and silently poisoning any float
  arithmetic done on it. **Behaviour change:** see *Breaking* below.
- **`cmat_exp` never terminated on a non-finite Frobenius norm.** The scaling-squaring loop halves
  the norm until it drops below 1, but `+Inf * 0.5` is still `+Inf` and still `> 1` — so a single
  overflowing entry anywhere in the matrix hung the process forever. Verified: the pre-fix suite
  runs 1 second with the fix and does not finish in 110 seconds without it. Bounded at
  `_CMAT_EXP_MAX_SCALING = 64`, which is not a heuristic — `exp` overflows f64 above 709, so no
  larger scaling parameter can describe a representable result.
- **`lorentz_is_valid` rejected valid boosts.** It checked `Λᵀ η Λ = η` against a *fixed absolute*
  `1e-12`, but the entries of that product are sums of products of two matrix entries, so their
  rounding error scales with **cosh²(η)**. At rapidity 10, `cosh² ≈ 1.2e8` and one ulp is already
  `1.5e-8` — four orders of magnitude past the tolerance demanded. Now scaled by the matrix
  magnitude, leaving ~4500× headroom over the true rounding bound while still rejecting a
  genuinely non-Lorentzian matrix (whose defect is O(scale²), not O(1e-12)) — both directions are
  asserted.

#### Fixed — medium correctness & safety (2.7.0-C, 4 of 7)

Each finding was **independently verified against the source and then adversarially challenged**
before it was applied — 7 confirmed, 0 refuted, and every reviewer ran real code rather than
reasoning about it. The challenges were not a formality: one showed that the proposed
`levi_civita` assertions all pass a wrong-sign mutant, and another found a live SIGSEGV in
`wedge_1_1` that the finding never mentioned. Both corrections are in the fixes below.
Suite 1181 → **1217**.

- **`detect_islands` checked only the *upper* bound of a contact's body indices.** A negative
  index sailed through `ba < n_bodies` and `_col_uf_find` subscripted `parent[]` *before* the
  array. Two distinct failure modes, both measured: `parent[-1]` aliases the union-find header's
  `rank` **pointer**, so `parent + rank_ptr * 8` is a wild address (**SIGSEGV, exit 139**); small
  negatives stay inside the heap and corrupt *silently* — with `n_bodies = 4` and `body_a = -3`
  the out-of-bounds read returned root 0, fusing unrelated bodies 0 and 1, so the function
  answered **3 islands where the correct answer is 4**. Skipping an out-of-range endpoint is the
  decision the line already made for out-of-range-*high* indices; it now covers both directions.
- **`num_ifft` corrupted the caller's buffer on its error path.** Validation lived only in
  `num_fft`, which `num_ifft` calls *after* running its in-place conjugate pass — so a rejected
  length returned `HSB_ERR_INVALID_INPUT` with every imaginary part already negated. The caller
  got a failure code **and** a silently conjugated input. `num_fft` rejects the same `n` leaving
  `data` bit-identical; the inverse now offers the same guarantee.
- **`levi_civita_3` / `levi_civita_4` returned raw i64 where their siblings return f64 bit
  patterns.** Not a cosmetic type mismatch: raw `(0 - 1)` is `0xFFFF_FFFF_FFFF_FFFF`, a **negative
  NaN**, so a contraction `c_i = Σ_jk ε_ijk a_j b_k` poisoned every component — *including the ones
  that are exactly zero*, since `NaN * 0.0 = NaN`. Raw `1` is 4.94e-324, not 1.0. Eight existing
  in-tree assertions encoded the old integer contract and were migrated with the fix.
- **diffgeo's downstream contractions dereferenced the documented 0 and sized allocations from an
  unbounded dim** (CWE-190 → CWE-476). `christoffel_symbols` and `riemann_tensor` have capped at
  16 and returned 0 since they were written; `ricci_tensor`, `ricci_scalar`, `einstein_tensor`,
  `weyl_tensor`, `geodesic_state_new`, `geodesic_rk4`, `parallel_transport` and `killing_residual`
  did neither. Added a shared `_DG_MAX_DIM`, null guards on every operand, and an allocation check
  at **all ten** `alloc` sites — including the two in `christoffel_symbols`/`riemann_tensor` that
  the finding left asymmetric. Also fixes **`wedge_1_1`**, which was not in the finding at all:
  `n2 = dim*(dim-1)/2` at `dim = 100000` asks for 40 GB, so `alloc` returned 0 and the store wrote
  to address 0.

**Evidence.** Reverting `collision_mesh.cyr` yields 3 failures **plus SIGSEGV**; `num.cyr` 4
failures; `tensor.cyr` 12 (and a second, sign-only mutant that keeps the correct f64 encoding
still fails 5 — the case the original assertions missed); `diffgeo.cyr` **SIGSEGVs before printing
a single line**. Every restore was verified byte-identical afterwards, not assumed.

#### Fixed — unscaled Householder / unbalanced decompositions (2.7.0-D)

**The most consequential fix of the 2.7.0 line so far**, and the finding materially understated it.
Every step of these decompositions squares matrix entries — the Householder norms in
`_lp_bidiagonalize`/`_lp_tridiagonalize`, and the Wilkinson shifts, which form entries of `BᵀB`
explicitly. `|x| > 1.34e154` squares to `+Inf` and `|x| < 1.49e-154` squares to `0`, so the usable
band was only about **1 .. 1e23**. Measured across 440 random matrices, `svd_golub_kahan` failed
**40/40 at scale 1e-10** on a relative reconstruction check — that is SI metres, so the physics
consumer was in the failure region, not at some exotic endpoint.

Failure modes differed by direction, and both were bad:
- **Overflow (1e200):** `||v||² = +Inf`, then `2/Inf = 0` and `0*Inf = NaN` — `svd_golub_kahan`
  returned `HSB_ERR_NO_CONVERGENCE` with `U` **all NaN**; `cqr_decompose` was worse, because
  `cx_abs(x1) = Inf` made the `v^H v` guard false so **every reflector was skipped** and it
  returned `HSB_ERR_NONE` with `Q = I` and `R = A` — R not even upper triangular.
- **Underflow (1e-200):** every reflector skipped, `HSB_ERR_NONE` returned, and `S` set to the
  **raw diagonal of A** — 42.6% off, with no error code.

`svd_golub_kahan`, `eigen_qr` and `cqr_decompose` now balance to unit max magnitude before
decomposing and rescale on the way out (`sv(cA) = c·sv(A)`, `eig(cA) = c·eig(A)`; U/Vt/Q are
invariant). Balancing also converts the **absolute** `EPSILON_F64` deflation thresholds into
relative ones, which is what repairs the small-magnitude half.

The divisor is rounded **down to a power of two**, so the divide and the un-scale are exact and
every unit-scale result is **bit-identical to what shipped before** — verified: all four suites
pass unchanged. Without that refinement the shipped `S[0]` would have drifted by 1 ulp for no
reason. Doc comments for the new `HSB_ERR_ALLOC` return were updated in the same edit.

*Evidence:* reverting `linalg_precision.cyr` fails **9 of 11** new assertions. The other two are
deliberately vacuous and labelled as such — they document that the pre-fix failure was *silent*.

- **`expr_to_str` / `sym_to_latex` dropped the rounding carry.** stdlib `fmt_float_buf` zero-pads
  only under `if (pad > 0)` with `pad = decimals - flen`. A fraction that rounds *up* to exactly
  `10^decimals` needs `decimals+1` digits, so `pad` goes negative, the pad branch is skipped, and
  the overflowed digit is emitted **after** the `.` with nothing carrying into the whole part:
  `0.9999999` rendered as **"0.1000000"** and `2.9999999` as **"2.1000000"**. `lib/` is regenerated
  by `cyrius lib sync`, so the fix could not live there — hisab now carries `_sym_render_f64`, the
  stdlib algorithm plus the two lines that fold the overflow into `whole`. Deliberately *not* fixed
  by pre-rounding the value: `val * 10^6` is inexact above ~1e15 and would perturb the
  large-magnitude integral values `_latex_fmt_const` routes down the same branch. 5 of 6 new
  assertions fail on revert; the sixth is a control proving non-carrying values are untouched.

#### Fixed — rank-deficient SVD (2.7.0-E). **All 7 medium findings now closed.**

A zero entry on the **diagonal** of the active bidiagonal block makes the implicit-shift step a
no-op: `_lp_bidiag_qr` builds its first rotation from `(d[p_lo]² − mu, d[p_lo]·f[p_lo])`, and when
`d[p_lo]` is zero that pair is `(−mu, 0)`, whose rotation is a bare sign flip. The sweep reproduces
the block unchanged and spins until `max_iter`, so `svd_golub_kahan` returned
`HSB_ERR_NO_CONVERGENCE` with `out_S` **never written** — the caller's zero-filled buffer was all
it got back. The rank-1 `[[1,2,3],[2,4,6],[3,6,9]]` bidiagonalizes to `d = (−3.7417, 0, 0)`,
`f = (13.4907, 0)`, and one sweep drives it to `d = (~1e-16, 0, 0)`, `f = (−14, 0)`, which then
repeats forever.

Added `_lp_split_zero_diag`: a chase of exact Givens rotations that removes the lone superdiagonal
entry, splitting the block and deflating a singular value of exactly 0. This is the zero-diagonal
case of **Golub & Van Loan Alg. 8.6.2**, omitted from the original transcription. The rank-1
matrix above now returns `HSB_ERR_NONE` with `S = (14, 0, 0)` — the exact singular values.

Two claims in the proposal were corrected by review before landing, and the corrections are in the
source comments: the defect is **typical of** rank-deficient input, not universal (a 6×6 rank-3
counterexample converges on the unpatched module, because bidiagonalization can leave an
exactly-zero *superdiagonal* that the outer deflation loop splits first — measured 104/240 random
rank-deficient failures, but 60/60 once columns were forced dependent); and the split's `tol` uses
`EPSILON_F64` = 1e-12 rather than machine epsilon, a resolution floor probed at 1e-9/1e-11/1e-13
and found to cause no regression — at 1e-13 the pre-fix code already returned NO_CONVERGENCE with
`S` all zero, so the fix is strictly better.

*Evidence:* reverting `linalg_precision.cyr` fails 7 of 11 new assertions. The `s == 0` assertions
are **not** standalone and are labelled as such in the test — pre-fix, `out_S` is never written, so
a zero-filled buffer satisfies them trivially; they are meaningful only bundled with the
return-code and reconstruction assertions beside them.

#### Test quality — `num_rk4` had **zero effective coverage** (2.7.0-G)

`num_rk4`'s only assertion in the entire tree was `dy/dt = 1, y(0) = 0, dt = 1 → y(1) = 1`, checked
with `f64_to(erx) == 1`. It could not fail, for two independent reasons:

1. **The test problem carries no order information.** With `f(t,y) = 1` every stage of every
   explicit RK method returns 1, so Euler, midpoint, RK3, stages evaluated at the wrong `t`, and
   `k3` built from `k1` all return exactly 1.0. Order 1 versus order 4 is invisible.
2. **The comparison is a truncation, not a tolerance.** `f64_to` truncates toward zero, so the
   assertion accepts anything in `[1.0, 2.0)` — up to +100% relative error, roughly 1e15× looser
   than the method's own round-off floor.

`num_rk4` could have been replaced by `return f64_add(y, f64_mul(dt, f(t,y)));` — plain Euler — and
the whole 1250-assertion tree would have stayed green. **Confirmed by doing exactly that.**

Replaced with 16 assertions built on references derived independently of the implementation: the
stability function `R(z) = 1 + z + z²/2 + z³/6 + z⁴/24` that any 4-stage order-4 method must
satisfy (`R(1) = 65/24`, `R(1/2) = 211/128`, `R(−1) = 3/8`, all exact); Simpson-exactness on
`y' = 4t³`; the closed form `y(t) = exp(t²/2)`; and multi-step values computed in **exact rational
arithmetic** (`fractions.Fraction`, no floating point in the reference) against `e` and `√e` at 60
digits. Every hex constant produced by `struct.pack('>d')`, none typed by hand.

The strongest of these is an **observed order-of-convergence** check — `err(h)/err(h/2)` must land
in (14, 17). Measured 15.190 and 15.589 for RK4; RK3 gives 7.61, RK2 3.82, Euler 1.90.

Adversarial review then found the block still blind to `dt ≤ 0` — every case used a forward step —
so a sign defect in `half_dt` survived. Two backward-integration assertions were added, and both
kill it.

*Evidence:* against an Euler stub, **14 of the 16 new assertions fail**. The previous single
assertion passed. Suite 1250 → **1266**.

#### Test quality — `calc_integral_simpson` was untested for `a != 0` (2.7.0-G)

The existing assertions quantised through `f64_to`/`f64_round` and certified almost nothing:
`round(3I) == 1` accepts `I` in `[1/6, 1/2)` — ±50% on 1/3; `trunc(I) == 7` accepts `[7, 8)`. No
tolerance-based Simpson assertion existed anywhere in the tree.

Replaced with 20 assertions that are **exact where the rule is exact**. Simpson is exact for every
polynomial of degree ≤ 3, and on a dyadic grid (`n = 64` gives `h = 2^-6` or `2^-5`) every node,
node value, weighted term and partial sum is an exactly representable dyadic — the only rounding in
the whole computation is the closing divide by 3. So the degree-0..3 answers are asserted as **bit
patterns**, not tolerances. Degree 4 is where the rule stops being exact, and by exactly how much:
the fourth derivative is constant for a quartic, so Euler–Maclaurin is an identity rather than an
estimate and the returned value must be `1/5 + 2h^4/15` — provably *not* 0.2. Plus a 4th-order
convergence check against the exact-rational rule value for the integral of `1/(1+x)`:
`e(32)/e(64)` must land in (15.9, 16.05).

Adversarial review then found the whole block still blind to `a != 0` — **every case started at
zero**, so two independent a-drop mutants survived: `h = (b-a)/n -> b/n` and `xi = a + i*h -> i*h`.
Added the integrals of `x^2` and `x^3` over `[1,3]` (still dyadic, still bit-exact). Both mutants
now fail.

#### Test quality — `ode_verlet` / `ode_symplectic_euler` had zero numeric coverage (2.7.0-G)

Both were asserted **only** with `!= 0` on the returned block. `alloc()` never returns 0 on
success, so neither assertion could fail for any input: two integrators, no coverage at all.

Both maps are linear on the SHO, so both have an exact closed form, derived from the textbook
definitions independently of `src/ode.cyr`. With `theta = 2*asin(h/2)`, Verlet gives
`q_n = cos(n*theta)` and `v_n = -cos(theta/2)*sin(n*theta)`. Each conserves a quadratic **shadow
Hamiltonian** exactly, which yields the property that makes a symplectic method worth having —
*bounded* energy error forever: `|E_n - 1/2| <= h^2/8` for Verlet, `<= h/(2(2-h))` for symplectic
Euler. Both bounds are asserted over 4000 steps (t = 125, ~19.9 periods), **and so is their
attainment** — a lower bound too, since a frozen no-op integrator would sit at `E = 1/2` exactly
and sail through the upper bound alone. Explicit Euler violates the symplectic-Euler bound by
~3000x.

Review found three further holes, all now closed: neither integrator was ever handed a force other
than `a(q) = -q`, so **hardcoding the SHO and ignoring the function pointer passed every
assertion**; the result was always consumed before the next call, so a shared static scratch buffer
would have been invisible; and `alloc(8)` where 16 bytes are stored overran undetected. Added
constant-acceleration cases (integrated exactly by both methods, so bit-exact), guard allocations,
and a re-read of the first result after 4001 later calls.

*Evidence:* each of the three previously-surviving mutants now fails — hardcoded `accel_fn` 2
failures, hardcoded `dp_dq` 2, `alloc(8)` 1. Suite 1266 → **1298**.

#### Test quality — Gell-Mann tautology and six existence-only assertions (2.7.0-G cont.)

**`gell_mann`.** The "hermitian" assertion compared `Re(l1[0][0])` against
`Re(adjoint(l1)[0][0])` with a tolerance of **1.0**. `cx_conj` copies the real part unchanged, so
that is `Re(x) - Re(x) = 0 < 1` — an identity for *every* complex matrix ever built. The traceless
check used a tolerance of 1.0 as well.

Replaced with the three properties that actually characterise the SU(3) basis — elementwise
Hermiticity (deliberately *not* via `cmat_adjoint`, which is what made the original vacuous),
tracelessness, and `tr(la lb) = 2 delta_ab` across all 64 pairs. Adversarial review then showed
those three are invariant under **any real orthogonal mixing of lambda_4..lambda_7**, so a sign
flip on l5 or l7, a swap of l4 and l6, or a 45-degree rotation of (l4, l6) all still passed. Added
convention pins for each of those four generators, plus the algebra closure
`[la, lb] = 2i sum_c f_abc lc` over all 64 pairs — which is also the **first coverage
`su3_structure_constant` has ever had**; `grep -rn "su3_" tests/` previously returned nothing, so
the table and the matrices had never been checked against each other.

**Six existence-only assertions.** `cga_point`, `opt_gradient_descent`, `eigen_power`,
`hvec4_length`, `num_dst` and `sym_to_latex` all had their existence asserted and their value never
checked. Each now pins a closed-form result: the CGA null-basis coefficients are *forced* by the
two defining properties of the embedding (`P.P = 0`, `P.e_inf = -1`), so they are derivable without
reading the source; DST-I is pinned with delta inputs that read off one column of the kernel,
because a round-trip alone is satisfied by `-S` and by any other symmetric involution.

Review found three residual holes and all three are now closed — each verified by applying the
mutant to source and watching the suite fail:

- `eigen_power` used only **symmetric** test matrices, so a transposed matvec (power iteration on
  `A^T`, i.e. the *left* eigenvector) was invisible. Added `[[1,2],[3,4]]`, whose right and left
  dominant eigenvectors differ.
- Both `eigen_power` matrices had a **positive** dominant eigenvalue and a positive eigenvector, so
  losing the sign of the pivot was invisible. Added `[[-3,1],[1,-2]]`.
- `sym_to_latex` rendered MUL and NEG only in `paren == 0` position, so dropping a
  `\left(...\right)` group was invisible.

*Evidence:* 1353 -> **1430**. The three mutants above fail 1, 3 and 1 assertions respectively; a
lambda_5 or lambda_7 sign flip fails 2 each; stubbing `su3_structure_constant` to 0 fails 1.

#### Test quality — 21 assertions whose tolerance was a literal 1.0 (2.7.0-G cont.)

The last and largest of the six. **15 static assertion sites** (6 in `hisab.tcyr`, 9 in
`modules.tcyr`) plus 8 loop iterations — 22 at runtime — compared a quantity whose closed form is
exactly 0 or 1 against `f64_from(1)`. A tolerance of 1.0 around an expected 1.0 admits everything
in `[0, 2]`. `foundation.tcyr` additionally declared `var TOL = f64_from(1);` — dead, unreferenced,
and a trap waiting to be picked up; removed after verifying nothing uses it.

Concrete examples of what those bounds accepted:
- `"rot90.x near 0"` / `"rot90.y near 1"` were satisfied by **every** rotation angle, since
  `|cos t| < 1` and `|1 - sin t| < 1` for all `t` in `(0, pi)`.
- `"srgb roundtrip"` was satisfied by every value `linear_to_srgb` can return.
- A single `[0][0]` entry is not an inverse test at all — `m4_inverse` returning its input
  unchanged passed it.

Replaced with named tolerances (`ULP_TOL_*` = 1e-15, ~4.5x the measured floor of a few ulp) and
closed-form expected values. Measured residuals: `rot90.x` 2.22e-16, `rot45.x` 1.11e-16,
srgb round-trip 1.67e-16, Euler 1.22e-16 — every one within 1 ulp.

The adversarial review is again the more interesting half. It confirmed the census was complete
(`\bTOL\b` across `tests/` returns exactly the one dead site) and that the patch **kills 11
mutants** — including a quaternion full-angle error, `lorentz_boost_x` reading velocity as
rapidity, and `m4_inverse` dropping its translation column, caught only by the new 16-entry sweep.
But it also found **seven survivors**, each now closed:

- **`cx_exp` returning the conjugate.** `|e^(i*pi) + 1| == 0` is blind to conjugation, because pi
  is a fixed point of `z -> conj(z)`. Pinned at `e^(i*pi/2)`, where the two differ.
- **`cmat_adjoint` stubbed to a plain copy.** This is the one that matters most: it would make the
  new 8-generator Hermiticity sweep compare `A` with `A` — vacuous again, the exact failure mode
  being repaired. Pinned on a non-Hermitian matrix.
- `minkowski_interval` dropping the z² term (the test 4-vector was `(5,3,0,0)`), `linear_to_srgb`
  using the decode-side breakpoint, an `su2_compose` cross-term sign flip, a Gell-Mann λ₄/λ₅ swap,
  and `m4_inverse` transposing its upper-left 3×3.

*Evidence:* 1430 -> **1523**. Re-running the survivors: conjugated `cx_exp` fails 1 in each of two
suites, the `cmat_adjoint` stub fails 3, the dropped z² fails 1.

**All six test-quality findings are now closed.**


`docs/development/issues/2026-08-04-tq-*.md` (the unit-tolerance sweep -- 21 assertions whose tolerance is a literal 1.0). All six were adversarially reviewed and all six stand
— but the reviewers also catalogued **~35 mutants the new assertions still would not catch**, which
is the next round of work rather than a rejection. The most alarming: `calc_integral_simpson`
survives `h = (b − a)/steps` → `h = b/steps` **and** `xi = a + i·h` → `xi = i·h`, i.e. it is
effectively untested for `a ≠ 0`; `ode_verlet`/`ode_symplectic_euler` never actually invoke the
caller's function pointer (hardcoding the SHO survives every assertion), and survive `alloc(8)`
where 16 bytes are stored; `eigen_power` survives a transposed matvec because both test matrices
are symmetric; and the Gell-Mann basis leaves the **sign of λ₄..λ₇ unconstrained** — pinning it
needs the SU(3) structure constants.

#### Added — quadtree, octree and spatial hash had **zero** coverage (2.7.0-N)

Three whole data structures, **11 public functions**, untested through 2.7.0: `quadtree_new` /
`_insert` / `_query`, `octree_new` / `_insert` / `_query`, and `spatial_hash_new` / `_insert` /
`_query_cell` / `_query_radius` / `_clear`.

That is exactly the shape of the k-d tree defect closed in 2.7.0-J — a spatial structure in
`spatial.cyr` whose queries nobody checked, which then returned wrong answers for three releases.
`spatial.cyr` stood at **3 of 14 functions** referenced.

Each is now checked against a counted expectation rather than for non-null:

- **quadtree** — a 20×20 grid over `[0,100]²`; the box `[10,30]²` must find exactly **25** of 400,
  the whole domain exactly **400**, and a disjoint box exactly **0**.
- **octree** — an 8×8×8 grid over `[0,80]³`; `[0,20]³` must find exactly **27** of 512, the whole
  domain **512**.
- **spatial hash** — `query_cell` must find exactly the **10** points sharing cell (0,0,0). The
  radius query is documented as a *broadphase*, so it is asserted as a **superset** of the true
  answer — it may over-report but must never miss — with a companion assertion that the true-hit
  count is non-zero, so the superset test cannot pass vacuously. `clear()` must actually empty it.

All twelve passed on the first run, so the expected counts were right independently of the
implementation.

`cyrius coverage` **63% → 65%** (587 functions, 382 referenced); it was 59% at the start of the
2.7.0 line. Suite 1533 → **1545**.

#### Added — a verified module dependency manifest (2.7.0-M)

`src/` modules carry **no `include` lines** — they are self-contained text the bundler
concatenates. That makes à-la-carte consumption possible but leaves the required set implicit,
which is exactly why the README's example shipped broken until 2.7.0-H.

The open item asked for a decision between three options — ship a manifest, promote the private
symbols, or drop the à-la-carte claim. **Measuring the graph settled it**, and corrected the
finding on the way:

- Six module pairs reach into another module's `_`-prefixed internals, not five. Two more pairs
  *appeared* to, including a `symbolic` ↔ `symbolic_ext` cycle — both were **comment-only
  mentions**, which a naive grep counts and a comment-stripping scan does not.
- The real graph is **acyclic**. Four modules are fully standalone (`error`, `interval`,
  `symbolic`, `tensor`); the deepest is `lie_ext` at 9.
- **Every** private edge runs from a derived module to its own base — `calc_ext`→`calc`,
  `num_ext`→`num`, `lie_ext`→`lie`, `symbolic_ext`→`symbolic`, `collision_mesh`→`collision_core`,
  `noise_simplex`→`calc`. That is the layering the module *names* already declare.

So this was never tangled coupling requiring symbol promotion; it was an undocumented layering.
`docs/architecture/overview.md` now carries the per-module include sets, **derived from source and
verified by compiling each of the 34 modules against exactly its listed set — 0 undefined symbols
for all 34**, plus a negative control (`geo` without `quat` correctly reports 1). The sets are the
transitive closure, so they are what you must include rather than what a module names directly.

The README's à-la-carte comment now points at the table instead of restating a partial list.

#### Added — benchmarks for the hot public paths that had none (2.7.0-L)

Every function repaired in 2.7.0-I/J/K, and every target of a *withheld* performance patch, was
invisible to `bench-history.csv` — so a regression in any of them could not be seen. That is the
gap that let a 3,412-violation k-d tree and an O(n²) Delaunay both sit unnoticed. 29 → **35**
benchmarks.

| Benchmark | Baseline |
|---|---|
| `kdtree_build_4k` | 2.27 ms |
| `kdtree_radius_4k` | 7.1 µs |
| `delaunay_2d_400` | 15.2 ms |
| `triangulate_600gon` | 9.9 ms |
| `svd_golub_kahan_12` | 181 µs |
| `eigen_qr_12` | 140 µs |

These are **baselines, not wins** — nothing here got faster. Their value is that the three withheld
optimisations (`delaunay_2d`, `_kd_partition`, `triangulate_polygon`) now have a committed
before-number to be measured against when they are revisited, rather than being re-derived from
scratch by whoever picks them up. `kdtree_radius_4k` also guards the 2.7.0-J repair on the cost
side: the corrected split plane prunes *more*, so a future regression that re-broke the invariant
would likely show up as a speed change too.

`tests/hisab.bcyr` gained `spatial.cyr`, `linalg_ext.cyr` and `linalg_precision.cyr` includes —
none of the three had ever been benchmarked at all.

#### Fixed — the two findings that were never scheduled (2.7.0-K)

Both were surfaced by the disposition sweep, not by any tier heading — they had sat in the audit
since 2026-08-04 without ever entering the work-list.

**`solve_gmres` overshot its documented matvec budget without bound.** `max_iter` is documented as
"maximum total matrix-vector products", but the code converted it into an *outer-cycle* count —
`outer_limit = trunc(max_iter / m) + 1` — while each cycle spends `1 + m` products. True cost:
`(trunc(max_iter/m) + 1) * (m + 1)`, which is neither `max_iter` nor a bounded multiple of it. The
audit said 2.2x; the reviewer said unbounded, and **the reviewer was right** — the ratio grows with
`restart / max_iter`, so no fixed safety factor fixes it. Measured on a 3×3 system: budget 1 with
restart 100 spent **4** products.

`A_fn` is a callback precisely because it may be arbitrarily expensive, so the budget is a resource
limit, not a hint. Now counted, with a mid-cycle break — the Krylov space built so far is still
valid, so the least-squares solve uses the completed steps and returns the best answer within
budget. Measured after: 4, 1, 3 products against budgets 10, 1, 3.

**`triangulate_polygon` documented a return length it does not always deliver.** The header stated
`length = 3*(n-2)` unconditionally, but `if (ear_found == 0) { return result; }` returns early on
degenerate or self-intersecting input with no error signal — a caller could not distinguish a
complete triangulation from a truncated one. The information was always present; it was simply
undocumented.

Documented the real contract (`== 3*(n-2)` complete, `< 3*(n-2)` partial, `0` no ear on the first
pass) with the detection test spelled out, and pinned it with assertions so it is a *tested*
contract rather than an accident. Ear clipping is only defined for simple polygons, so a short
return on self-intersecting input is the honest answer, not a bug to paper over.

*Evidence:* reverting `linalg_ext.cyr` fails both budget assertions. Suite 1526 → **1533**.

#### Fixed — the k-d tree's split plane did not separate its own subtrees (2.7.0-J)

**The most serious defect found in the 2.7.0 line.** `kdtree_within_radius` returned the wrong
count on **274 of 400** queries over a 1,500-point scatter — a spatial query silently under-reporting
hits, with no error signal.

`_kd_build_rec` set each node's `split_val` from `items[split]` — *some* right-side value — while
the partition was actually made at `mid_val`. Those are not the same number. Every right-side point
whose axis value fell below `items[split]`'s therefore sat on the wrong side of its own node's
pruning plane, and `_kd_nearest_rec` / `_kd_radius_rec` pruned subtrees that still contained valid
hits. Measured on the pre-fix tree: **3,412 points on the wrong side of an ancestor's plane.**

A second, independent defect in the same function: the "ensure at least one item on each side"
clamp moved an item across the partition *after* the fact, producing a left subtree containing a
point above the plane. Its justifying comment claimed the midpoint split "can ONLY degenerate when
lo == hi" — **that reasoning does not hold in floating point**, because `(lo + hi) * 0.5` can round
down to exactly `lo`, and then `lo < mid_val` is false for the very point that attains `lo`. The
clamp now retries with the plane at `hi`, which keeps both sides non-empty *and* value-sound.

`split == end` is genuinely unreachable when `lo < hi` (`mid_val <= hi`, so the hi-attaining point
never compares below it), so that clamp is gone rather than kept as dead defensive code.

The 2.6.14 coincident-point fix is untouched and still asserted — for `lo == hi` the plane is the
common value, which separates correctly whichever way the positional split falls.

*Evidence:* against the shipped code the two new assertions fail with **3,412** plane violations and
**137 of 200** wrong query counts. Both were absent before: the 2.6.14 kdtree assertions pass
happily on a 3,412-violation tree, which is why this survived three releases. The invariant is now
checked structurally on **every node**, not just through behaviour. Suite 1523 → **1526**.

**How it was found.** Not by an audit. It surfaced because an adversarial reviewer, checking whether
a *performance* patch preserved results, built a brute-force oracle — and the shipped code failed it.
The perf patch that prompted this was **not applied** (see 2.7.0-I); the defect it exposed was.

#### Investigated — `_col_in_circumcircle` sign loss: confirmed, and the claim narrowed (2.7.0-J)

The 2.7.0-I review reported the shipped in-circle predicate as wrong on "near-coincident clusters".
Checked against exact rational arithmetic (`fractions.Fraction`, every operand converted exactly
from its f64 bit pattern):

| Configuration | Disagreements with exact |
|---|---|
| Uniformly small clusters, scale 1e-11 … 1e-6, plus unit-scale controls | **0 of 30** |
| **Mixed magnitude** — one vertex at 1e3 … 1e7 with two near-coincident vertices at 1e-10 … 1e-7 | **3 of 40** |

**Near-coincidence is not the trigger.** The predicate translates every point to the query point
first, which is the well-conditioned formulation, and it handles a uniformly small cluster
correctly. What breaks it is the *magnitude spread*: `a_sq` for the far vertex is ~1e14 while the
terms it is differenced against are ~1e-14, so the determinant is a difference across ~28 orders of
magnitude and every significant bit is lost.

**CORRECTED before release.** The first draft of this entry claimed the configuration is common
here, because `delaunay_2d` "begins from a super-triangle placed far outside the data, so every
early insertion is a mixed-magnitude test". That is false: the super-triangle sits at
`max(dx, dy) * 10` — **10x** the extent. Re-measured with that actual geometry, across cluster
spreads from 1e3 to 1e13: **0 wrong of 180**. The 3-of-40 failures came from a synthetic
configuration with absolute coordinates from 1e7 down to 1e-10, and the failure tracks the
*absolute* magnitude of the far vertex (it enters the determinant squared) rather than the ratio.

So the predicate is genuinely non-robust, but **no reachable path through `delaunay_2d` is known**.
Severity is latent, not red. The lesson is in the method: the first probe generated coordinates by
magnitude class instead of reconstructing the caller's real geometry.

**Not fixed.** The remedy is an adaptive exact predicate (Shewchuk's `incircle`: floating-point
determinant with a forward error bound, exact expansion only when |det| falls below it). Rescaling
cannot help — the spread is inherent to the super-triangle construction. Filed with its repro at
`docs/development/issues/archived/2026-08-04-incircle-precision.md` rather than attempted here, because a
half-done robust predicate is worse than a documented sharp edge.

#### Performance — `solve_bicgstab` leaked a work vector per iteration (2.7.0-I)

`s` was allocated **inside** the iteration loop. `alloc()` is a bump allocator that never frees, so
every pass abandoned another `n*8` bytes — the same hot-loop-allocation class 2.6.15 cleared out of
LM and L-BFGS. Hoisted alongside `v` and `p`.

| Measure | Before | After | Change |
|---|---|---|---|
| Arena, n=1024, max_iter=128 | **3,194,880 B** | **2,154,496 B** | **−32.6%** |

Asymptotically `8n(6 + 3·max_iter)` → `8n(7 + 2·max_iter)`. Results are **bit-identical** —
verified across `n` ∈ {0, 1, 3, 7, 64, 65, 400, 512, 1024}, convergent and non-convergent, because
all `n` slots are overwritten before any read.

The finding's "superlinear" framing was **refuted**: the time cost is linear. This is a leak, not a
complexity defect, and saying so is the point — the CSV would not have shown a time win.

#### Two shipped correctness defects found while checking that optimisations preserve results

Neither is a performance issue. Both came from adversarial reviewers building independent oracles
to verify that a proposed optimisation did not change results — and finding the **shipped** code
was already wrong. They outrank the perf work that exposed them, and are filed accordingly.

- **`kdtree_within_radius` returns the wrong count on most queries.** Measured on a 1,500-point
  scatter with 400 radius-60 queries: **222 of 400 wrong**. The 2.6.14 kdtree assertions did not
  detect a tree with 676 pruning-plane violations. A spatial query returning wrong answers.
- **`_col_in_circumcircle` is less accurate than a plain circumcentre test.** All 23,741 retirements
  flagged by a candidate sweep predicate were re-checked in **exact rational arithmetic**: zero were
  geometrically unsound. The divergences came from the *shipped* in-circle determinant losing
  precision on near-coincident clusters, so Delaunay output on such inputs is already suspect.

#### Three optimisations confirmed but deliberately NOT applied

Each is a real complexity win, and each was measured at multiple sizes so the *growth ratio* — not
a single timing — carries the claim. All three are filed with their reviews under
`docs/development/issues/2026-08-04-perf-*.md`.

- **`delaunay_2d`** is O(n²) confirmed: growth converges on **4.0x per doubling**, 217 ms at
  n=1,600. An x-sweep-with-retirement rewrite reaches 2.1–2.5x per doubling (**−93% at n=3,200,
  −96% at n=6,400**). Not applied: it is **1.9–3.0x slower** with up to 2.5x memory on *cocircular*
  input, and it changes the triangulation on inputs as simple as the **unit square** — both results
  valid, zero shared triangles.
- **`_kd_partition`** confirmed, but the patch costs **+28–30% on duplicate-heavy skewed data** and
  ships **zero permanent regression coverage** for the invariant it depends on.
- **`triangulate_polygon`** reflex-only pruning is **sound on simple polygons** — ~33,000 tested,
  including an exhaustive enumeration of every 4-, 5- and 6-gon on a 3×3 integer grid, **0
  divergences**. It diverges only on self-intersecting input, which is outside the documented
  contract — but **output length changes in both directions**, including cases where the shipped
  code returns EMPTY and the new one returns a complete triangulation. That is entangled with the
  never-scheduled `collision_core.cyr:511` contract finding, so the two should land together.

A faster wrong answer is worse than a slow right one; these stay open until the regressions and the
missing coverage are addressed.

#### Fixed — duplicate globals in a flat namespace, now gated (2.7.0-H)

`calc.cyr` and `calc_ext.cyr` each declared `F64_THREE`, `F64_FOUR`, `F64_SIX` and `F64_FIFTEEN`.
Cyrius has **one flat namespace with last-definition-wins**, and `calc_ext.cyr` follows `calc.cyr`
in the `[lib]` bundle order — so the values that actually shipped were chosen by *manifest order*,
not by the file being edited. Invisible for exactly as long as the two copies happened to agree.
This is the hazard 2.6.8 addressed for `ERR_*`, recurring.

Removed the four duplicates from `calc_ext.cyr`, and — more importantly — **added duplicate
detection to `scripts/check-constants.sh`**, so the class cannot come back. It reports every
top-level global declared in more than one `src/` file, and fails whether the values differ *or*
agree; two copies agreeing today is a coincidence with a deadline, not safety. The matcher is
anchored at column 0, since Cyrius indents function bodies and a leading-whitespace `var` is a
local that shadows nothing.

*Evidence:* re-adding `F64_SIX` to `calc_ext.cyr` with a different value exits 1 and names both
sites and both values; restoring exits 0. Constant coverage 145 -> 141 declarations (four fewer
because four duplicates are gone), 1523 assertions unchanged.

**A correction to my own analysis.** My first census of this finding reported it as overstated —
"only `F64_FIFTEEN` is duplicated; `F64_THREE`/`FOUR`/`SIX` are not defined in `src/` at all". That
was **wrong**. My grep pattern used a single space before `=` and these files align their
declarations with several, so it silently matched nothing. The original finding was accurate on all
four. The gate found what the grep missed, which is the argument for gates over greps.

#### Fixed — README's à-la-carte example did not compile (2.7.0-H)

It listed `include "lib/…"` paths, but `lib/` holds vendored stdlib only — the library source is in
`src/`. It was also missing `vec2.cyr` and `vec4.cyr`, which the example needs without ever naming
them (`quat`/`mat4`/`transforms` reach into `HVec4_*`). Both fixed and **verified by building and
running it**: it compiles clean and the ray-sphere hit really is `4.0`, as the comment claims.

#### Fixed — BVH block header described a different data structure (2.7.0-H)

The header over `geo_advanced.cyr`'s BVH block was wrong in five ways, each checked against the
code: it is **not** a flattened vec-of-nodes arena (every node is its own `alloc()`); nodes are
**32 bytes, not 40**; there is no `[32]` padding slot; `[8]`/`[16]` hold child **pointers**, not
indices; and `bvh_build` returns the **root node pointer**, not a `{ nodes_ptr, count }` handle.
Rewritten to describe what is actually there, with the old claims listed so the correction is
auditable.

#### Fixed — `eigen_qr` non-convergence (2.7.0-F)

Last release I said this finding was **not reproduced** and left it open rather than close it on
evidence that did not support either verdict. Reproduced now, by sweeping 150 random symmetric
4×4s across three magnitude regimes: **1 failed**. Its condition number is only **240** — a matrix
LAPACK's `dsyev` handles without comment — with eigenvalues 27.793, −2.999, 0.417, −0.116, and
`eigen_qr` returned `HSB_ERR_NO_CONVERGENCE` even at `max_iter = 1,000,000`.

The cause was the **block-boundary logic**, not the QR sweep. `_lp_tridiag_qr` located the start of
the unreduced block with two loops:

1. The first contained a **fake break** — a `var _break = 1;` and two no-op `p_lo = p_lo;`
   statements with a `# break` comment. Cyrius has a real `break`; this one does nothing, so the
   loop always ran to 0, zeroing offdiagonals across the whole matrix as a side effect.
2. The author evidently noticed it did not work and discarded its result — the next line is
   literally `# Re-find p_lo properly` — with a second loop whose test was an **absolute**
   `EPSILON_F64`, inconsistent with the **relative** `eps·(|d[i]| + |d[i−1]|)` threshold the
   deflation test three lines above uses.

On a matrix whose entries span 0.04 .. 24.6 those two disagree about what counts as negligible, so
the block bounds and the deflation test could contradict each other and the sweep stalled. Replaced
with a single loop using the same relative threshold as the deflation test.

*Evidence:* the 150-matrix sweep goes **1 failure → 0**. All six new assertions — convergence, the
four eigenvalues against `numpy.linalg.eigvalsh`, and the `A·v = λ·v` relation — fail on revert.
All four suites are unchanged by the fix, so it is not a tolerance loosening. A separate pre-existing defect surfaced
alongside the balancing work: `eigen_qr` returns `HSB_ERR_NO_CONVERGENCE` at *unit* scale for a
well-conditioned symmetric 4×4 (cond 6.85), reproduced on unpatched source.

### Changed
- **`lorentz_boost_x/y/z/…` parameter renamed `beta` → `eta`.** The bodies take `cosh`/`sinh` of the
  argument — it was always **rapidity** — while the signature said `beta` (velocity) and the doc
  comment said "rapidity eta". A consumer trusting the signature and passing a velocity would have
  silently built the boost for a much smaller speed. No behaviour change; the name now matches.

#### Fixed — the three high findings (2.7.0-B)

`geo_advanced.cyr` is the largest module (1,261 lines) and had **zero** coverage on GJK / EPA /
BVH / TOI before this release. Suite 1154 → **1181**.

- **`cmat_mul` never checked conformability and read past the end of `b`.** It ran the inner
  product over `cols(a)` while indexing `b` by that same bound, so any `b` with fewer rows than
  `a` has columns was read out of bounds — `b`'s buffer is allocated at exactly
  `rows(b)*cols(b)*16` bytes. A 2×3 times a 2×2 read two complex numbers of adjacent heap. Now
  returns 0 for non-conformable operands.
- **The designed-0 return was being dereferenced one call-link further out.** `cmat_new` and
  `cmat_mul` both return 0 by design (dimension cap, non-conformable), but `cmat_add`, `cmat_sub`,
  `cmat_adjoint`, `cmat_scale` and `cmat_trace` all called `load64` on it immediately —
  `cmat_commutator` feeds exactly that in, since it passes two `cmat_mul` results straight to
  `cmat_sub`. Both mutants **SIGSEGV**; with the guards they return 0. `cmat_add`/`cmat_sub` also
  now reject mismatched shapes, and `cmat_trace` returns 0 rather than `cx_zero()` — zero is a
  valid trace, so returning it would make a propagated failure indistinguishable from a result.
- **`time_of_impact` ignored `max_t` and missed every collision past 0.64 units.** The advancement
  loop was bounded by `GJK_MAX_ITER` — a cap on GJK's *simplex refinement*, not on time sampling —
  while stepping a fixed `0.01 / speed`. The two were conflated, so the search covered
  `64 × 0.01 = 0.64` units of relative displacement and then returned 0 no matter what horizon the
  caller asked for: a **false negative from a collision query**. The step budget is now its own
  constant and spans the whole horizon, with the tolerance step as a floor so short horizons keep
  full resolution.

### Performance

`_bvh_build_rec` fell back to a **1-vs-(n−1) split** whenever no center sorted below the split
midpoint — which is precisely what coincident or tightly clustered AABBs do, since then every
center *is* the midpoint. Because each level re-merges its whole remaining range to compute
`bounds`, that made the build **O(n²)** in time and arena, not merely O(n)-deep. It is the unfixed
sibling of the `kdtree_build` defect closed in 2.6.14. Splitting at the median restores O(n log n).

| Benchmark | Before | After | Change |
|---|---|---|---|
| `bvh_degenerate_4k` (4,000 coincident AABBs) | **2.722 s** | **15.5 ms** | **−99.4%** (~176×) |

Measured in `tests/hisab.bcyr` on both sides; the row is in `bench-history.csv`. An isolated
scaling probe confirms the complexity change rather than a constant-factor win — per doubling of
n the old form cost 4.0× and the new form 2.1×:

| n | Before | After |
|---|---|---|
| 1,000 | 174.3 ms | 3.46 ms |
| 2,000 | 677.3 ms | 7.36 ms |
| 4,000 | 2,690 ms | 15.94 ms |
| 8,000 | 10,720 ms | 35.22 ms |

Note on evidence: unlike every other fix this release, the BVH repair is **not** mutation-proven by
an assertion — 20,000 coincident boxes complete under both forms, so the defect is a cost, not a
wrong answer, and `assert_eq` cannot state it. The new assertions guard the *result* (all 20,000
boxes still reported, disjoint queries still empty, separated inputs unaffected); the benchmark
guards the cost.

### Breaking
- **`geo_ray_plane` now returns `0` on a miss instead of `-1`.** This aligns the code with the
  contract stated twice in its own module and with every sibling intersection function. No caller
  in `src/` depended on the old sentinel. **Migration:** consumers testing `== (0 - 1)` should test
  `== 0`. Consumers already doing float comparisons on the result were getting NaN semantics and
  are strictly better off.

## [2.6.15] - 2026-08-03 — P3/P4 closeout: two O(n²) hot paths, doc drift, full module coverage

Final repair release of the 2.6.12 audit. Closes the **performance** and **documentation** tiers
and brings the last two never-tested modules into the suites — so every one of the 34 modules
shipped in `dist/hisab.cyr` is now compiled and asserted by `cyrius test`. Suite 1093 → **1127**;
`cyrius coverage` 57% → **59%**, files 32/35 → **34/35** (the 35th is `main.cyr`, the CLI smoke
binary, which by design does not include the library).

### Performance — measured, not asserted

Both hot paths were benchmarked **before** the rewrite, per CLAUDE.md. Two new benchmarks were
added to `tests/hisab.bcyr` first, and both the before and after rows are in `bench-history.csv`:

| Benchmark | Before | After | Change |
|---|---|---|---|
| `halfedge_2k_tris` (2,048 triangles) | **190.1 ms** | **1.2 ms** | **−99.4%** (~156×) |
| `convex_hull_2d_2k` (2,000 points) | **22.1 ms** | **2.1 ms** | **−90.3%** (~10×) |

- **`halfedge_from_triangles` twin pairing was O(n_he²)** — the inner loop scanned every later
  half-edge and never broke after a match, ~2.25·n_tris² iterations. A directed edge `(src → dst)`
  has exactly one possible twin, `(dst → src)`, so an open-addressed hash of directed edges
  replaces the scan with one lookup. Verified structurally identical to the old implementation:
  `twin(twin(i)) == i` for every paired edge and a boundary count of exactly `4(W−1)` on a grid
  mesh, with both implementations agreeing.
- **`convex_hull_2d`'s pre-sort was an insertion sort** — the only super-linear step, since the
  monotone chain itself is O(n), and uncapped because the public entry point bounds nothing.
  Replaced with heapsort: O(n log n) worst case and O(1) extra memory. Heapsort rather than merge
  sort deliberately — merge sort's n-slot scratch would be *leaked on every call* under the
  never-freeing bump allocator. The comparison is inlined rather than passed as a comparator
  because it must dereference a separate `points` vector and Cyrius has no closures; this is
  exactly why stdlib `vec_sort_by` (element-value comparator via `fncall2`) does not fit here.
  **Hull output is byte-identical** to the old sort at n = 50, 500, 2,000 and 4,000.
- **Levenberg-Marquardt** allocated `jtj`, `rhs`, `lu` and `piv` inside its iteration loop
  (5.35 MB of unreclaimed bump scratch over 200 iterations) while `j_buf` beside them was already
  hoisted; all four are now hoisted, which is safe because each is fully overwritten at the top of
  every iteration. It also computed the **symmetric** JᵀJ twice — the loop now runs the upper
  triangle only and mirrors, which is bit-identical since the k-accumulation order is unchanged
  and `f64_mul` is commutative. A 2-parameter line fit converges to the same `(a, b)` as before.
- **L-BFGS** allocated three n-length buffers per iteration, all dead by the end of it; hoisted.

No claim is made for LM/L-BFGS: they are memory reductions, and neither has a benchmark.

### Fixed — documentation that contradicted the code
- **`docs/architecture/overview.md`** credited **BVH to `spatial.cyr`**; all `bvh_*` functions are
  in `geo_advanced.cyr` (`spatial.cyr` contains zero occurrences of "bvh"). Also listed
  `f64_le`/`f64_ge` under `f64_util.cyr`, which explicitly documents them as *removed* in favour
  of stdlib `math`; and its Design Principles still named the bare `ERR_*` constants, namespaced
  to `HSB_ERR_*` back in 2.6.8.
- **`opt_conjugate_gradient`** was documented as Fletcher-Reeves in two places; it implements
  Polak-Ribière+ with a `max(0, β)` restart.
- **`num_dst` / `num_idst`** were labelled DST **Type II**; the kernel is
  `sin(π(i+1)(k+1)/(n+1))`, which is DST-**I** (FFTW RODFT00). Documentation only — the transform
  is a correct DST-I and `num_idst` inverts it, now pinned by a round-trip test. (`num_dct` was
  checked too and is genuinely DCT-II, so it was left alone.)
- **`expr_eval`** documented "Aborts if a variable is not found"; it writes a warning to fd 2 and
  returns 0.0. A test had been *skipped* on the strength of that false claim.
- **`hodge_star_2form_4d`** described its `sign` parameter three mutually contradictory ways —
  "+1 Euclidean, −1 Lorentzian", then "baked in; parameter kept for API consistency" (it is live),
  then "1 for Lorentzian, −1 for Euclidean". It is an overall multiplier on the Lorentzian dual;
  **no value of it produces a Euclidean dual**, whose sign pattern differs per component rather
  than globally. Corrected here and in `docs/architecture/math.md`, which propagated the claim.
- **`collision_core.cyr`**'s header named `collision.cyr`, a file that has not existed since the
  2.2.2 split, and advertised Delaunay, half-edge mesh and island detection — all three of which
  live in `collision_mesh.cyr`.

### Changed — final two modules brought into the suites
`num_ext` and `symbolic_ext` were the last of the eight modules included by no suite. `num_ext`
now has DST-I round-trip, DCT-II, Möbius and tridiagonal-solve assertions; `symbolic_ext` has
integration (differentiating the integral recovers the integrand), LaTeX rendering, and pattern
matching. The `sym_match` tests initially failed because `sym_match` takes a **Pattern**
(`PAT_*`, built with `pat_*`), not an `Expr` — passing an `Expr` silently returns 0. That is now
asserted explicitly, including that `pat_any_const` matches a constant but not a variable.

### Verified
- Suite **1127/1127** (foundation 307 + hisab 205 + edge_cases 199 + modules 416), up from 1093.
- **All 34 library modules included by a suite** — previously 8 were not compiled by `cyrius test`
  at all. `cyrius coverage` 335/587 → **348/587 (59%)**; files 32/35 → **34/35**.
- Benchmarks **28** (was 26); before/after rows for both hot paths in `bench-history.csv`.
- `cyrius fuzz` 1/1; `check-constants.sh` 110/110; `lint` / `fmt --check` / `vet` clean;
  `deps --verify` 30/30; `cyrius distlib` regenerated.

## [2.6.14] - 2026-08-03 — P1 closeout: memory safety and robustness

Third repair release of the 2.6.12 audit, closing the **memory-safety tier**. Three of the four
defect groups **crashed the process** before this release — reverting `complex.cyr`, `calc_ext.cyr`
or `optimize.cyr` individually makes the new suite exit **139 (SIGSEGV)** rather than merely fail.

Two more never-tested modules (`calc_ext`, `noise_simplex`) are now in the suites, leaving only
`num_ext` and `symbolic_ext`. Suite 1063 → **1093**; `cyrius coverage` 54% → **57%**, files 29/35
→ **32/35**.

### Fixed — capped constructors' null return was stored through (CWE-476)
`cmat_new` caps at `_CMAT_MAX_ELEMS` (65536) and correctly returns 0 above it. Four callers then
wrote through that 0, converting a designed error signal into a segfault. Each is reachable from
operands that are themselves *under* the cap:
- `cmat_mul` — `ar × bc` can exceed it when the operands cannot: 1000×1 times 1×1000 asks for 1e6.
- `cmat_kronecker` — multiplies *both* dimensions; 17×17 ⊗ 17×17 is 83521. (16×16 ⊗ 16×16 is
  exactly 65536 — at the cap, and legitimately allowed; the tests pin both sides of that edge.)
- `cmat_inverse` (`linalg_ext`) — builds `[M | I]`, doubling the width, so a 250×250 input under
  the cap produces a 250×500 augmented matrix over it.
- `cmat_identity` — propagates from a caller-supplied `n`.

### Fixed — `opt_bfgs` / `opt_lbfgs` sized working memory from an unbounded dimension
`opt_bfgs` allocates an `n × n` inverse Hessian and `opt_lbfgs` an `m × (2n+1)` history ring, with
no cap on `n` or `m`: `alloc` returned 0 and the initialisation loop stored through it (CWE-190 →
CWE-476). Added `_OPT_MAX_DIM = 4096` (the hard ceiling from `ALLOC_MAX` is 5792) plus explicit
allocation checks in all four solvers, and a new `HSB_ERR_ALLOC` (`-11`) so allocation failure is
reportable rather than fatal.

### Fixed — `calc_bspline` / `calc_nurbs` indexed control points negatively
Both validated `n_pts != 0` and the knot-count relation but never `degree` itself, nor the
well-formedness requirement that a degree-*d* spline needs at least *d+1* control points. With
`degree >= n_pts` the span search left `k = degree`, the sentinel-undo turned that into
`k - n_pts`, and de Boor indexed `ctrl_pts` at a **negative** offset — an out-of-bounds read
resolving to a wild-pointer dereference instead of the documented 0-on-invalid-input return.

### Fixed — adaptive Simpson bounded depth but not work
The recursion capped *depth* at 50, but every non-converged node spawns two children, so the cap
permits 2^50 (~1e15) integrand evaluations. A NaN integrand takes exactly that path: every f64
comparison involving NaN returns 0, so `|error| < tol` is false forever. Two bounds added — an
explicit non-finite bail (NaN is the only value comparing unequal to itself), and a
stop-when-bisection-stops-progressing check, which bounds real work by f64 resolution rather than
by the counter.

### Fixed — `kdtree_build` recursed to depth O(n) on coincident points
`_kd_partition` splits on the axis **value** midpoint. That separates properly whenever
`lo < hi` — the point attaining `lo` is below the midpoint, the one attaining `hi` is not — so it
can *only* degenerate when every point shares the axis value. That case fell to the
"ensure at least one item per side" clamp, which peels exactly **one** element per level. Measured
on the pre-fix code: fine at 40,000 coincident points, **SIGSEGV at 60,000**. Now splits
positionally in exactly that case, which is sound precisely because the values are equal — the
`split_val` the caller reads is then the common value, so query pruning compares exactly. Pinned
by both a 60,000-point build and a half-coincident tree asserting an exact nearest-neighbour hit.

### Fixed — NaN accepted by the Armijo line search
`_opt_armijo`'s tie-breaker was written `f64_lt(threshold, f_new) == 0`, which is also true when
`f_new` is NaN, so a NaN objective was accepted as a valid step and propagated into `x` for
`opt_bfgs`, `opt_lbfgs` and `opt_conjugate_gradient`. Now requires `f_new` to compare equal to
itself first.

### Fixed — complex zero-divisor cutoff was 1e-6, not 1e-12
`cx_div`, `cx_inv` and `cx_powf` compared a **squared** modulus against the **unsquared**
`EPSILON_F64`, so any divisor with |z| < 1e-6 was silently flushed to zero — six orders of
magnitude more aggressive than intended. `cx_is_zero` alongside them always squared its tolerance
correctly, which is the tell. Now `norm_sq < EPSILON_F64²`; a 1e-9 divisor inverts correctly and
an exact zero is still guarded.

### Fixed — `FLOAT_RENDER_BUF` was 14 bytes short of stdlib's scratch requirement
`fmt_int_buf` renders digits backwards from `buf + 23` before shifting them down, so it needs 24
bytes at whatever pointer it is handed — and `fmt_float_buf` calls it **twice**, the second time
at an offset that can reach 22 (sign + 20 whole digits + `.`), touching offset **45**. The buffer
was 32. Raised to 64. A large-magnitude symbolic coefficient overran the allocation.

### Fixed — four fBm entry points returned NaN for `octaves <= 0`
`fbm_2d`, `fbm_3d`, `simplex_fbm_2d` and `simplex_fbm_3d` all end in an unguarded
`f64_div(sum, max_amp)`, and `max_amp` only accumulates inside the octave loop — so it is exactly
0 when `octaves <= 0` and the functions returned NaN despite documenting a roughly `[-1, 1]`
range. NaN then propagated silently through every downstream sample. All four now return 0.

### Changed
- **`src/error.cyr`** — added `HSB_ERR_ALLOC = -11`.
- **`calc_ext` and `noise_simplex` brought into `tests/modules.tcyr`** — both shipped in
  `dist/hisab.cyr` while being included by no suite, so they were not even compiled by
  `cyrius test`.

### Verified
- Suite **1093/1093** (foundation 307 + hisab 205 + edge_cases 199 + modules 382), up from 1063.
  Mutation-proven per file against pre-fix source: reverting `complex.cyr` → **SIGSEGV**,
  `calc_ext.cyr` → **SIGSEGV**, `optimize.cyr` → **SIGSEGV**, `noise_simplex.cyr` → 2 failures,
  `spatial.cyr` → **SIGSEGV** at 60k points.
- `cyrius coverage` 320/587 → **335/587 (57%)**; files 29/35 → **32/35**.
- `cyrius fuzz` 1/1; `check-constants.sh` 110/110; `lint` / `fmt --check` / `vet` clean;
  `deps --verify` 30/30; `cyrius distlib` regenerated (17,021 → 17,159 lines).
- **No performance claim.** No benchmarked path changed. The kd-tree fix alters build *shape* on
  degenerate input — from O(n²) to O(n log n) — but `kdtree_build` has no benchmark, so the
  improvement is stated as a complexity argument and a 60k-point build that previously crashed,
  not as a measured number.

## [2.6.13] - 2026-08-03 — P0 closeout: SVD, eigensolver, SE(3) and interval soundness

Second repair release of the 2.6.12 audit. Closes the **remaining five P0 findings**, every one
of them in a module that had **zero test coverage** — which is precisely why they shipped. Four
of the eight never-tested modules are now in the suites (`einsum`, `lie_ext`, `mat3`,
`linalg_precision`), `cyrius coverage` moves 50% → **54%**, and the suite grows 981 → **1063**.

No API or signature changes; consumers rebuild only. But every consumer of `svd_golub_kahan`,
`eigen_qr`, `se3_exp`/`se3_log`, `su2_exp`/`su2_log`, `einsum`, `ivl_sin`, `ivl_sqrt` or
`ivl_div` was getting wrong answers — in three cases with no error signalled at all.

### Fixed — `svd_golub_kahan` returned U transposed, so `A != U·S·Vᵀ`
`_lp_bidiagonalize` accumulated the left Householder reflectors in **forward** order, building
`H_{n-1}···H_0` where the decomposition needs `U = H_0···H_{n-1}`. Rewritten to store the
reflectors and accumulate **backward** after the sweep. `S` and `Vᵀ` were always correct — `Vᵀ`
is literally `G_{n-1}···G_0`, so forward order happens to be right there, which is exactly why
the error was one-sided and invisible. Verified by reconstructing `U·S·Vᵀ` entrywise: pre-fix,
**8 of 9 entries wrong** on both a symmetric and a non-symmetric 3×3.

### Fixed — `eigen_qr` never converged for n ≥ 3
`_lp_tridiag_qr` applied the similarity `Gᵀ·T·G` where `G = [[cs, sn], [-sn, cs]]` is the
rotation that *zeroes* the bulge (`G·[x;z] = [r;0]`). The matching similarity is `G·T·Gᵀ`; only
the signs of the cross terms differ, which is why it read as correct. The off-diagonal therefore
never decayed: measured, a plain symmetric 3×3 returned `HSB_ERR_NO_CONVERGENCE` **even at
max_iter = 100000**, and at n = 2 it paired each eigenvalue with the wrong eigenvector. The `Q`
accumulation (`Q ← Q·Gᵀ`) was already the correct companion to the *fixed* update and needed no
change — the two were inconsistent, not `Q` alone. A dead first attempt at the update, whose
stores were immediately overwritten, was removed. Now converges and matches reference
eigenvalues to 1e-8, with `A·v = λ·v` asserted for every returned pair at n = 2, 3 and 4.

### Fixed — SU(2)/SE(3) half-vs-full-angle convention split
`su2_exp` returned `cos(|ω|) + sin(|ω|)/|ω| · ω`, the generator convention denoting a rotation by
**2|ω|**, while `so3_exp`, `su2_from_axis_angle` and `su2_to_rotation_matrix` all use half-angle.
`se3_exp` built its translation block `V` from `theta = |ω|` but took `R` from `su2_exp`, so `R`
and `t` came from angles a factor of two apart — the resulting `(R, t)` was **not the exponential
of any twist**. `su2_exp` and `su2_log` moved to half-angle, restoring the double-cover identity
`su2_to_rotation_matrix(su2_exp(ω)) == so3_exp(ω)`, which is now asserted entrywise along with
`su2_log ∘ su2_exp = id` and the `se3_log ∘ se3_exp` round-trip.

### Fixed — `einsum` silently mis-computed its own documented examples
`_einsum_is_label` accepted only `a`–`h`, so `i`/`j`/`k` — used in **every** example in
`einsum.cyr`'s own header — were rejected; and the parser **skipped** unrecognised characters
rather than erroring. `einsum("ij,jk->ik", …)` therefore parsed to zero labels and returned the
*trace* (5, not 19) before **segfaulting**. Alphabet widened to `a`–`z`, and the parser now
rejects: unknown characters, specs longer than `_EINSUM_MAX_RANK`, and any notation rank that
disagrees with the tensor's actual `HTensor_rank` (the out-of-bounds shape read). `tensor_new`'s
documented 0-on-failure return is now checked at both sites. Spaces are tolerated.

### Fixed — three interval enclosure-soundness violations
The one contract an interval type must never break is that the result **contains** the true range.
- `ivl_sin` returned `[min(sin a, sin b), max(sin a, sin b)]` for any width ≤ π, which
  **under-approximates** whenever the interval straddles an extremum without an endpoint at it:
  `ivl_sin([1.4, 1.8])` excluded 1.0 although sin peaks at π/2 inside. Extrema are now located
  exactly (`ceil(k_lo) <= floor(k_hi)` on `π/2 + kπ`), which is both sound *and* tighter than the
  old `width > π → [-1,1]` shortcut: `[0.1, 3.3]` contains a maximum but no minimum and now
  yields `[sin(3.3), 1]` instead of all of `[-1, 1]`.
- `ivl_sqrt` clamped only the lower bound, so a wholly-negative interval produced `[0, NaN]` —
  and since `ivl_new` cannot order around NaN, `ivl_contains` then reported that interval as
  containing **everything**. Both ends are now clamped.
- `ivl_div`'s zero guard used a raw integer `== 0` on an f64 bit pattern, matching only `+0.0`;
  `-0.0` (`0x8000000000000000`) slipped through and divided by zero. Now uses `f64_le`/`f64_ge`.

### Changed — four more modules brought into the suites
`einsum`, `lie_ext`, `mat3` and `linalg_precision` had **no test suite including them** — they
were not even compiled by `cyrius test` — while all four shipped in `dist/hisab.cyr`. Each now
has coverage asserting its defining identities. Remaining untested: `calc_ext`, `noise_simplex`,
`num_ext`, `symbolic_ext`.

### Verified
- Suite **1063/1063** (foundation 307 + hisab 200 + edge_cases 187 + modules 369), up from 981.
  Every fix is mutation-proven against the pre-fix source: reverting `linalg_precision.cyr` fails
  10, `lie.cyr` fails 24, `interval.cyr` fails 6, `einsum.cyr` fails 2 **and then segfaults**.
- `cyrius coverage` 297/587 → **320/587 (54%)**; files referenced 25/35 → **29/35**.
- `cyrius fuzz` 1/1; `check-constants.sh` 110/110; `lint` / `fmt --check` / `vet` clean;
  `deps --verify` 30/30; `cyrius distlib` regenerated (16,897 → 17,021 lines).
- **No performance claim.** No benchmarked path changed; `_lp_bidiagonalize` now allocates an
  `m*n` scratch buffer for the reflectors, which is a memory cost, not a speed one, and SVD has
  no benchmark to measure against.

## [2.6.12] - 2026-08-03 — Audit sweep: seven mis-transcribed constant tables repaired

**A repair release, not a feature release.** A full P(-1) audit of the 2.6.11 tree — 8 parallel
dimensions over all 34 modules, every finding adversarially re-verified against running code —
produced **70 confirmed findings (2 critical, 23 high, 23 medium, 22 low)** across 64 sites.
Full report: [`docs/audit/2026-08-03.md`](docs/audit/2026-08-03.md). This release closes the
critical tier plus the two highest-value independent defects; the rest is tracked under
`[Unreleased]` and roadmap §2.6.12.

**No API change. No signature change. Consumers need only rebuild** — but every consumer of
`ode_dopri45`, `ode_bdf(order=4)`, `ode_yoshida4`, `calc_integral_gauss5`, `num_is_prime`,
`solve_bicgstab`, the sRGB conversions, the spherical-harmonic basis or `simplex_*` was getting
**numerically wrong answers** before this release.

### Fixed — seven hand-encoded constant tables did not encode their documented values

hisab stores f64 constants as raw i64 bit patterns (Cyrius has no float literals), so the
adjacent comment is the only readable statement of intent. In seven tables the bits disagreed
with the comment. **47 constants re-derived from exact rationals and closed forms**, each table
now pinned by its own mathematical invariant:

| Table | Site | Invariant | Was | Now |
|---|---|---|---|---|
| Dormand-Prince RK4(5) | `ode.cyr` | Σb = 1 | **0.635581152612211** | 1.0 |
| Gauss-Legendre 5-point | `calc.cyr` | Σw = 2 | **2.000492320194892** | 2.0 |
| BDF-4 | `ode.cyr` | Σα = 1 | **1.01** | 1.0 |
| Yoshida-4 symplectic | `ode.cyr` | 2w₁+w₀ = 1 | **1.4737899450113117** | 1.0 |

- **DOPRI45 — 23 of 30 tableau constants were wrong.** Stage rows also failed `Σⱼ a_ij = c_i`
  (row 3 summed to 0.25 against c₃ = 0.3). It was **not a consistent integrator at any order**:
  it did not merely lose accuracy, it converged to the wrong solution. One step of `y' = y` from
  `y(0) = 1` at `dt = 0.1` now matches e^0.1 to < 1e-9; before, it was wrong in the second digit.
- **BDF-4** drifted 1% per step on a constant solution. **Yoshida-4** advanced a free particle
  only 85.9% of the correct distance. **Gauss-Legendre-5** carried a 246 ppm relative error
  floor on a rule that is exact for polynomials of degree ≤ 9.
- **`color.cyr`** — both sRGB piecewise breakpoints (0.03275 shipped for the IEC 61966-2-1 value
  **0.04045**; 0.0031294 for 0.0031308) and four of five spherical-harmonic normalisations.
- **`noise_simplex.cyr`** — `_SIMPLEX_G2` encoded √3 as 1.73158 instead of 1.73205, so the
  skew/unskew pair was not an exact inverse.
- **`quat.cyr`** — the slerp/nlerp switchover threshold was 0.999375, not the documented 0.9995.

### Fixed — `num_is_prime` reported real primes as composite above 3.03e9
`_num_miller_rabin_test`'s squaring step used a raw `(x * x) % n` while `num_modpow` alongside it
already used the overflow-safe `_num_mulmod`. `x` ranges over `[0, n-1]`, so once
`n > 3037000499` (≈√2^63) the product wrapped negative and every witness failed. Measured: the
genuine primes **3037218841** and **4294967357** both returned composite. Now routed through
`_num_mulmod`; verified against an independent Miller-Rabin implementation across the band.

### Fixed — `solve_bicgstab` never iterated
`var breakdown_tol = f64_from(0x3C32725DD1D243AC);` — `f64_from` **converts** an integer to a
double, it does not reinterpret bits, so the tolerance was **4.34e18** instead of 1e-18 and the
first `|rho| < tol` check fired immediately. The solver returned the initial guess verbatim on
every call. The literal already is the bit pattern; the `f64_from` wrapper is removed. (The
comment also mis-stated the value as ~1e-30; it is 1e-18.) This was the only `f64_from(0x…)` in
`src/`.

### Added — `scripts/check-constants.sh`, a CI gate for the whole defect class
Decodes **every** hand-encoded f64 literal in `src/` and asserts it against the value in its own
comment, handling exact rationals, closed-form expressions (`1/(2 - 2^(1/3))`, `sqrt`, `pi`) and
truncated decimals, with the tolerance derived from the precision the comment itself claims. It
also cross-checks comments that state both an exact form and a decimal — which caught four
DOPRI45 comments whose decimals disagreed with their own fractions. **110/110 now verified.**
Wired into CI between the vet and distlib gates. The class is mechanical; the guard is too.

### Changed — assertions that a broken implementation could pass
Every fix landed with the assertion that would have caught it. The pre-existing tests were the
reason six releases shipped a broken integrator:
- `ode_dopri45` asserted only `1 < y < 2` for a value whose exact answer is 1.10517 — replaced
  with a value assertion plus a check that the embedded error estimate is not garbage.
- `gauss5` compared `f64_round(result)` to an integer, tolerating ±0.5 absolute (7.1% relative
  on the const-7 case) — replaced with 1e-11 tolerances and a weights-sum check.
- **BDF-4 and Yoshida-4 had no numeric assertions at all** — now pinned by the consistency
  invariants their wrong constants violated.
- The sRGB test was a **round-trip**, which is structurally blind to a wrong transfer curve (the
  same wrong breakpoint applies in both directions and the error cancels) — added an absolute
  value assertion and a breakpoint-segment check.
- `solve_bicgstab` and the `num_is_prime` overflow band had no coverage at all.

### Verified
- Suite **981/981** (foundation 307 + hisab 186 + edge_cases 173 + modules 315), up from 961:
  +20 assertions, all mutation-proven. Reverting `src/ode.cyr` alone fails 4; reverting
  `src/calc.cyr` fails 3; reverting `src/linalg_ext.cyr` fails 3; reverting `src/num.cyr` fails 4.
- `cyrius fuzz` 1/1. `check-constants.sh` 110/110. `cyrius lint` / `fmt --check` / `vet` clean;
  `deps --verify` 30/30; `cyrius distlib` regenerated (16,885 → 16,897 lines).
- Benchmarks re-run (26, commit `ee8032c`). **No performance claim** — `num_is_prime` measured
  20.3 µs against 20.7 µs before, which is run-to-run noise, and no other changed path is
  benchmarked. The `_num_mulmod` routing is strictly more work per squaring step; it was not
  measurably slower at the benchmark's input sizes, and correctness is not negotiable regardless.

## [2.6.11] - 2026-08-03 — Cyrius 6.5.6 toolchain bump + sakshi 2.4.7; stdlib `mat_new` CWE-190 guard lands

Maintenance release: toolchain pin **6.4.69 → 6.5.6** (a **minor** jump across 24 releases —
6.4.70–6.4.86 plus 6.5.0–6.5.6) and first-party dep **sakshi 2.4.6 → 2.4.7**. **No executable
math-module change** — all 34 modules compile clean on the new pin, and `dist/hisab.cyr` differs
only by its version header and one rewritten `mat_new_guarded` doc comment (verified: the bundle
diff contains **zero** non-comment lines, 16,878 → 16,885 lines). The bundle's stdlib leaf requirements are
unchanged (`syscalls` + `io`, verified identical when regenerated under *both* pins), so
consumers need no `[deps]` edits; bumping their own pin to 6.5.6 is recommended for parity.

Not a routine bump, though. Two latent defects were found and closed, and neither was in the
math: hisab had been building against a **stale vendored `ganita`** whose `mat_new` segfaults
on a negative dimension, and all four test suites could **score PASS with 256 failing
assertions**. Both are fixed and both are now pinned by tests.

### Security
- **Stdlib `mat_new` CWE-190 guard (ganita 1.0.3 → 1.0.4).** `ganita_mat_new` now rejects
  non-positive dimensions and element counts above `GANITA_MAT_MAX_ELEMS = 33_554_430` (the
  `ALLOC_MAX`-derived ceiling), returning 0; `ganita_mat_identity` and `ganita_mat_from_array`
  propagate the null. **This is a live hisab code path** — `mat_new` / `mat_mul` / `mat_lu` are
  thin `ganita_*` aliases (`lib/ganita.cyr:1329+`) used from `optimize.cyr` and `linalg_ext.cyr`.
  Measured with the compiler held constant and only `ganita.cyr` swapped:

  | vendored ganita | `mat_new(-5, 3)` |
  |---|---|
  | 1.0.3 (shipped through 2.6.10) | **SIGSEGV — exit 139** |
  | 1.0.4 (this release) | returns null, exit 0 |

  **The exposure was a vendoring miss, not a pin miss, and it was worse than the threat model
  recorded.** The repo's `lib/ganita.cyr` sat at **1.0.3 while the 6.4.69 pin already shipped
  1.0.4** — so hisab spent the entire 2.6.10 cycle on the unguarded copy. Every prior audit
  entry (2026-06-15 through 2026-07-21) called this item "unguarded upstream, mitigated" because
  the vendoring check compared *toolchain snapshot against toolchain snapshot*, which cannot see
  drift in the committed `lib/`. The check is now `lib/*.cyr` byte-compared against
  `~/.cyrius/versions/<pin>/lib/`, which is what surfaced it. Closes the roadmap's tracked
  "stdlib `mat_new` overflow guard" item (open since 2.5.3).
- **Upstream contract now pinned by tests.** `tests/edge_cases.tcyr` gained 4 assertions on
  stdlib `mat_new` itself (negative / zero / overflow-prone / valid dims). Previously only
  hisab's own `mat_new_guarded` wrapper was asserted, which is exactly why the stale vendoring
  was invisible. Mutation-proven: reverting `lib/ganita.cyr` to 1.0.3 crashes the suite with
  **exit 139** instead of passing.

### Fixed
- **Test suites could report PASS with 256 failing assertions.** All four `tests/*.tcyr` ended
  at a bare `var r = assert_summary();`, and `assert_summary()` returns the raw failure *count*.
  A POSIX wait status is 8 bits, so exactly 256 / 512 / 768 failures truncated to 0 and the
  `cyrius test` gate scored **PASS**. Demonstrated on hisab's own epilogue before fixing, then
  re-measured after:

  | epilogue | 256 failures | 1 failure | 0 failures |
  |---|---|---|---|
  | unclamped (through 2.6.10) | **exit 0 — PASS** | exit 1 | exit 0 |
  | clamped (this release) | **exit 1 — FAIL** | exit 1 | exit 0 |

  All four suites now clamp (`if (r > 0) { r = 1; }`) before exiting. This was a CI-gate
  integrity hole, not a library defect — no shipped math behaviour changes. cyrius 6.5.6 fixed
  the identical hole in its own `proj-tcyr` scaffold template.
- **`cyrius fuzz` never discovered `tests/hisab.fcyr`.** Through 6.5.5 the verb walked only
  `fuzz/`, a directory this repo does not have, so the fuzz gate had been silently exercising
  **zero** harnesses (it exits 0 when it finds nothing). 6.5.6 walks `tests/` as well; the
  harness now actually runs, and passes (**1/1**) on its first real execution.

### Changed
- **Toolchain pin `6.4.69` → `6.5.6`; sakshi `2.4.6` → `2.4.7`.** Re-vendored `lib/` via
  `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.5.6; the transitive
  `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed, as at
  6.4.66/6.4.69). `cyrius.lock` 30 deps (1 commit-pinned), `deps --verify` 30/30. Smoke version
  string `src/main.cyr` 2.6.10 → 2.6.11.
- **Vendored stdlib delta — six files.** `ganita.cyr` (the guard above); `vec.cyr` (6.5.4 added
  `vec_sort_by` introsort + `vec_select_nth` quickselect, comparator via `fncall2` — purely
  additive, **not adopted**, see below); `syscalls_linux_common.cyr` (+`sys_exit_group`);
  `syscalls_windows.cyr` / `syscalls_x86_64_agnos.cyr` (peer `sys_exit_group` definitions plus
  agnos GPU present/fill wrappers — non-Linux-x86_64, vendored for parity); and `io.cyr`
  (upstream changes that leave hisab's `println` path behaviour unchanged).
- **`sys_exit_group` epilogue.** `sys_exit` is `exit(2)` and ends only the *calling thread*.
  `src/main.cyr`, `examples/basic_math.cyr`, `tests/hisab.bcyr`, `tests/hisab.fcyr` and all four
  `tests/*.tcyr` migrated off `syscall(SYS_EXIT, r)` / `syscall(60, r)`. hisab is single-threaded
  so either form terminated correctly; the group form is correct by construction. CLAUDE.md's
  program-epilogue principle updated to match.
- **sakshi `2.4.6` → `2.4.7`** — a latent-bug fix with **no public API change**: two private
  helpers renamed (`_sk_write_int` → `_sk_buf_write_number`, `_sk_write_str` →
  `_sk_buf_write_string`) because `_int` / `_str` / `_cstr` are reserved cyrius overload-dispatch
  suffixes, so `_sk_write_int` was squatting the dispatch slot of the unrelated `_sk_write(buf,
  len)`. Both symbols are `_sk_`-private; hisab links nothing that moved. **Audited hisab for the
  same defect class**: `src/` defines exactly one function with a reserved suffix (`expr_to_str`)
  and there is no `expr_to` base for it to shadow — clean.
- **`src/linalg_ext.cyr`** — `mat_new_guarded`'s doc comment no longer claims stdlib `mat_new` is
  unguarded. The wrapper is **kept, not retired**: its 16,777,216-element cap is deliberately
  stricter than upstream's 33,554,430, so it remains the entry point for untrusted dimensions.
  `tests/hisab.tcyr`'s comment corrected to match (its over-cap assertion at 25M elements is
  precisely the band where hisab rejects and upstream accepts).

### Verified
- `cyrius build` + smoke (prints `hisab 2.6.11`), `cyrius test tests/*.tcyr` **961/961** —
  foundation 307 + hisab 175 + edge_cases **167** (+4, the new upstream `mat_new` assertions) +
  modules 312, 4 suites. `cyrius fuzz` **1/1**. Benchmarks re-run via `./scripts/bench-history.sh`
  (26 benchmarks, commit `5331f05`); **no performance claim is made** — this release changes no
  math code path, and the run-to-run spread against the June baseline is not attributable to the
  bump. Cleanliness gates green: `cyrius lint` (all `src`/`examples`/`tests` globs,
  warnings-as-errors), `cyrius fmt --check`, `cyrius vet src/main.cyr` (2 deps, 0 untrusted,
  0 missing). `cyrius distlib` regenerated `dist/hisab.cyr` (header-only diff, 2.6.10 → 2.6.11);
  `cyrius deps --verify` 30/30.
- **Consumer end-to-end** against the regenerated bundle on 6.5.6: `hvec3_length`, `hquat_length`,
  stdlib `mat_new` (valid + both guard paths) and `mat_new_guarded` — 7/7, exit 0.
- Tracked toolchain issues re-verified on 6.5.6 (minimal repros): interval-ident-lex **still
  live** (`var iv_add` → `expected identifier, got unknown`; the reserved-SIMD-name rename stays
  in `tests/modules.tcyr`); for-empty-clauses **still live** (`for (; …)` → `unexpected ';'`);
  cli-arg-clobber not re-tested (destructive). No tracked issue newly fixed by this bump.

### Not adopted (recorded, with reason)
- **`vec_sort_by` / `vec_select_nth`** (new in 6.5.4). hisab's one hand-rolled sort —
  `_col_sort_indices_by_xy` (`src/collision_core.cyr:339`, single caller at `:393`, the
  `convex_hull_2d` pre-sort) — **does not fit the API**: `vec_sort_by` passes element *values* to
  the comparator (`fncall2(cmp, a, b)`), while hisab sorts *indices* by dereferencing into a
  separate `points` vector, and Cyrius has no closures. Deferred on the API mismatch, CLAUDE.md's
  "wait for the third instance" rule (this is the first), and because swapping a hot-path sort is
  a performance change that needs its own before/after numbers rather than riding a maintenance
  bump. `vec_select_nth` has no call site in hisab today.

## [2.6.10] - 2026-07-21 — Cyrius 6.4.69 toolchain bump

Maintenance release: toolchain pin **6.4.66 → 6.4.69** (3 patch releases). **No library
source change** — all 34 math modules compile clean on the new pin; the `dist/hisab.cyr`
bundle is byte-identical apart from its version header, and the bundle's stdlib leaf
requirements are unchanged (`ganita` + `math`), so consumers need no `[deps]` edits (bumping
their own pin to 6.4.69 is recommended for parity). First-party dep **sakshi stays at 2.4.6**
— already the latest tag, so no dep change this release. The vendored stdlib picks up three
upstream 6.4.67–6.4.69 fixes; only one is on a code path hisab actually links (`fmt`, via
`fmt_float_buf` in the `symbolic` modules), while the `math` float-parse and agnos `sys_reboot`
fixes are vendored for snapshot parity but sit on paths hisab never exercises.

### Changed
- **Toolchain pin `6.4.66` → `6.4.69`.** Re-vendored `lib/` from the new pin via
  `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.4.69; the transitive
  `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed, as at
  6.4.66). `cyrius.lock` 30 deps (1 commit-pinned), `deps --verify` 30/30. Smoke version
  string `src/main.cyr` 2.6.9 → 2.6.10.
- **Vendored stdlib delta — three files, all upstream bug/hardening fixes:**
  - `fmt.cyr` — `fmt_hex` / `fmt_sprintf %x` loop guard `> 0` → `!= 0`, so high-bit-set
    values (negatives / u64 ≥ 2^63) render every digit instead of zero; and `fmt_float_buf`
    gained a non-finite guard emitting `inf` / `-inf` / `nan` instead of the garbage
    `-.00000-` token for Inf/NaN/out-of-i64-range inputs. **hisab links this** —
    `symbolic.cyr` + `symbolic_ext.cyr` render coefficients via `fmt_float_buf`; for every
    finite in-range value the output is byte-identical, so this is a pure correctness gain on
    the non-finite edge (suite stays 957/957).
  - `math.cyr` — the float-parse exponent accumulator now saturates at 340 (10^340 already
    overflows f64 to ±Inf), turning an O(exp_val) apply loop into O(1) and closing an
    algorithmic-complexity DoS (`"1e100000000"` burned ~237 ms upstream). Behaviour-preserving
    for every representable value.
  - `syscalls_x86_64_agnos.cyr` — `sys_reboot()` widened nullary → 4-arg
    `power_sys(magic1, magic2, cmd, arg)` (agnos 1.55.25 power-control). Agnos-only platform
    variant; hisab builds and tests the x86_64-linux target, so no behavioural impact —
    vendored for snapshot parity.

### Verified
- `cyrius build` + smoke (prints `hisab 2.6.10`), `cyrius test tests/*.tcyr` (**957/957** —
  foundation 307 + hisab 175 + edge_cases 163 + modules 312, 4 suites), fuzz harness clean,
  benchmarks nominal. Cleanliness gates green: `cyrius lint` (all `src`/`examples`/`tests`
  globs, warnings-as-errors), `cyrius fmt --check`, `cyrius vet src/main.cyr` (2 deps, 0
  untrusted, 0 missing). `cyrius distlib` regenerated `dist/hisab.cyr` (header-only diff,
  2.6.9 → 2.6.10); `cyrius deps --verify` 30/30.
- Tracked toolchain issues re-verified on 6.4.69 (minimal repros): interval-ident-lex
  **still live** (`var iv_add` → `expected identifier, got unknown` — the reserved-SIMD-name
  shadow is still worked around in `tests/modules.tcyr`); for-empty-clauses **still live**
  (`for (; …)` → `unexpected ';'`); cli-arg-clobber not re-tested (destructive). No tracked
  issue newly fixed by this bump.

## [2.6.9] - 2026-07-17 — Cyrius 6.4.66 toolchain bump + sakshi 2.4.6; modules.tcyr compile fix

Maintenance release: toolchain pin **6.3.11 → 6.4.66** (a full minor across 55 patch
releases) and first-party dep **sakshi 2.4.2 → 2.4.6**. **No library source change** — all
34 math modules compile clean on the new pin; the `dist/hisab.cyr` bundle is byte-identical
apart from its version header, and a consumer that includes the full bundle compiles and runs
end-to-end on 6.4.66. Not breaking — the bundle's stdlib leaf requirements are unchanged from
2.6.8 (`ganita` + `math`); consumers need no `[deps]` edits (bumping their own pin to 6.4.66
is recommended for parity).

### Changed
- **Toolchain pin `6.3.11` → `6.4.66`.** Re-vendored `lib/` from the new pin via
  `cyrius lib sync` — all 27 declared-subset stdlib files byte-match 6.4.66; the transitive
  `lib/result.cyr` + `lib/atomic.cyr` were already identical (no hand-refresh needed this
  bump). `cyrius.lock` 30 deps (1 commit-pinned), `deps --verify` 30/30. Smoke version string
  `src/main.cyr` 2.6.7 → 2.6.9 (was stale from 2.6.8).
- **sakshi `2.4.2` → `2.4.6`** (git, `dist/sakshi.cyr`). 2.4.6 pins cyrius 6.4.49 (≤ 6.4.66);
  its shipped surface links only `fnptr` + `atomic` and is unchanged, so logging behaviour is
  identical.

### Fixed
- **tests/modules.tcyr — the suite could not compile** (a pre-existing failure, unrelated to
  the toolchain bump: it reproduces on cycc 6.3.11–6.4.66). The interval-arithmetic section
  named its result vars `iv_add` / `iv_sub` / `iv_mul` — which are **reserved cycc SIMD
  intrinsic names** (packed integer-vector add/sub/mul; the lexer tokenizes them as builtins,
  see the toolchain's `lib/simd.cyr`). Using one as a variable name fails with a misleading
  `expected identifier, got unknown` at `var iv_add = ...`, which then desyncs the parser and
  took the whole file down (deterministic, 8/8). Renamed to `iv_sum` / `iv_diff` / `iv_prod`
  (`iv_div` / `iv_neg` / `iv_abs` are unaffected — there is no integer-vector divide, so
  `iv_div` was never reserved). The suite now compiles stably and runs **312/312**, restoring
  the full **957/957** across all four suites. Filed the toolchain bug at
  `docs/development/issues/archived/2026-07-17-cyrius-interval-ident-lex.md` (and upstream in the cyrius
  repo); a `NOTE:` comment in the test guards the names against a well-meaning rename-back.

### Verified
- `cyrius build` + smoke (prints `hisab 2.6.9`), `cyrius tests tests/` (**957/957**, 4 suites,
  0 failed), `cyrius bench` (all metrics nominal), `cyrius lint` / `cyrius fmt --check` /
  `cyrius vet` (clean), `cyrius distlib` (regenerated, v2.6.9, 16,878 lines) — all green on
  6.4.66. Tracked-issue re-verify on the new pin: for-empty-clauses **still open**
  (unchanged); the new interval-ident-lex issue filed and worked around.

## [2.6.8] - 2026-07-06 — collision hardening for co-compilation with the sandhi/TLS stack

Two consumer-facing hardening fixes surfaced when a consumer (prakash) opts hisab
in alongside the `sandhi` HTTP/TLS stdlib stack:

### Fixed
- **symbolic** — the two float-render scratch buffers switched from a stack array
  sized by an enum constant (`var buf[FLOAT_RENDER_BUF]`) to `alloc(FLOAT_RENDER_BUF)`.
  When co-compiled with the `tls`/`dynlib` stdlib modules (which relocate oversized
  locals to shared globals), the enum-sized stack-array form tripped the compiler's
  "array size identifier must be an enum constant" path; the heap form the compiler
  itself recommends sidesteps it. No behavioral change — same 32-byte buffer, same
  `fmt_float_buf` render.

### Changed
- **error** — bare error constants namespaced `ERR_*` → `HSB_ERR_*`
  (`HSB_ERR_NONE`, `HSB_ERR_INVALID_INPUT`, `HSB_ERR_NO_CONVERGENCE`, …). hisab's
  bare `ERR_NONE` previously imposed a last-wins global collision on every consumer
  (prakash namespaced its own errors `PK_ERR_*` to work around it); moving the
  namespace into hisab fixes it at the root. **Values are unchanged** (`HSB_ERR_NONE`
  is still 0); consumers that check numeric return values (`== 0`) are unaffected —
  only name-based references need the `HSB_` prefix.


## [2.6.7] - 2026-06-30 — Cyrius 6.3.11 toolchain bump + sakshi 2.4.2

Maintenance release: toolchain pin **6.2.11 → 6.3.11** and first-party dep
**sakshi 2.1.0 → 2.4.2**. Pure infrastructure — **no library source change**.
All 34 math modules compile clean on the new pin; the `dist/hisab.cyr` bundle is
byte-identical apart from its version header. 957/957 tests pass; every
cleanliness gate (lint/fmt/vet), the fuzz harness, and all 26 benchmarks are
green on 6.3.11. **Not breaking** — the bundle's stdlib leaf requirements are
unchanged from 2.6.6 (`ganita` + `math`); consumers need no `[deps]` edits,
though bumping their own toolchain pin to 6.3.11 is recommended for parity.

### Changed
- **`cyrius.cyml`** — toolchain pin `6.2.11` → `6.3.11`; `[deps.sakshi]` tag
  `2.1.0` → `2.4.2`.
- **Vendored stdlib** — `lib/*.cyr` re-resolved to the 6.3.11 toolchain. Note the
  6.3.x CLI split: `cyrius deps` now resolves **git deps only** (it commit-pins
  sakshi in `cyrius.lock`), while **`cyrius lib sync`** (no `--full`) vendors the
  declared `[deps] stdlib` subset — superseding the 6.2.x-era `cyrius deps`-does-
  both flow. Stdlib files that actually changed on the bump: `assert`, `bench`,
  `fnptr`, `io`, `math`, and the `syscalls` platform variants
  (`aarch64_linux`/`linux_common`/`windows`/`x86_64_agnos`/`x86_64_linux`).
  `ganita` (the math umbrella) is unchanged. Every vendored stdlib file now
  byte-matches the 6.3.11 toolchain; `cyrius.lock` is 30 deps (1 commit-pinned),
  `cyrius deps --verify` 30/30.
- **`src/main.cyr`** — CLI smoke version string `hisab 2.3.3` → `hisab 2.6.7`
  (had been stale since 2.3.3; Cyrius source can't interpolate `VERSION`, so the
  string is hand-maintained — it had drifted across the 2.4.x/2.5.x/2.6.x bumps).
- **`dist/hisab.cyr`** — regenerated via `cyrius distlib` (header now v2.6.7;
  bundle body unchanged, 16,878 lines).
- **`.gitignore`** — ignore `/dist/hisab.deps`, the leaf-requirements companion
  6.3.x `cyrius distlib` emits alongside the tracked `dist/hisab.cyr`.

### Fixed
- **`lib/result.cyr`** (vendored stdlib, transitive dep of `io`/`tagged`) — picked
  up the 6.3.11 `_die` portability fix: the unwrap-on-`Err` abort was a bare
  `syscall(60, 1)` that no-op'd on agnos (an undefined syscall number → `-1`), so
  unwrap-on-`Err` **failed open** and continued with garbage; it is now a
  target-guarded abort (`#60` on backed targets, `exit` on agnos). No effect on
  the Linux x86_64 build path, but vendoring a 6.2.11 stdlib file under a 6.3.11
  pin was wrong. `cyrius lib sync` skipped it (transitive include not in the
  declared subset); completed from the authoritative `6.3.11/lib`. `atomic.cyr`
  (transitive dep of `alloc`) was already byte-identical to 6.3.11.

### Security
- **Tracked toolchain issues re-verified on 6.3.11** (doc-health mandate at each
  pin bump). The 2 still-open filings carry forward unchanged: the C-style
  `for (;cond;)` empty-clause parse error **still reproduces** (confirmed on
  6.3.11 — workaround remains `while`); the destructive CLI unknown-flag-clobbers-
  source issue was not re-tested. No regressions; the 3 issues fixed at the
  6.2.11 bump remain archived.

## [2.6.6] - 2026-06-15 — Cyrius 6.2.11 toolchain bump + stdlib math reorg

Toolchain upgrade from **6.0.14 → 6.2.11**. The 6.2.x stdlib reorganised its
math surface: the transcendental functions moved out of `math` into a new
**`ganita`** umbrella module, and `ganita` now subsumes the `matrix` and
`linalg` modules entirely (re-exporting their full public API). The library
source needed two adjustments; all 957 tests pass and every cleanliness gate
is green on the new pin.

### Breaking
- **Consumer dependency change.** Consumers pulling `dist/hisab.cyr` must update
  their `[deps] stdlib` list: **add `"ganita"`** (the bundle now references
  `f64_acos`/`f64_pow`/`f64_atan2`/`f64_cosh`/`f64_sinh` and the hyperbolic
  inverses, which live in `ganita`) and **remove `"matrix"` and `"linalg"`**
  (subsumed by `ganita` — keeping them alongside `ganita` triggers
  duplicate-definition collisions). Keep `"math"` — it still uniquely provides
  `f64_le`/`f64_ge`/`f64_clamp`/`f64_lerp`/`f64_min`/`f64_max`/`f64_sign`/
  `f64_trunc`/`f64_parse`/`gcd`/`lcm` and the exp/ln polyfills.
  Migration: `["…", "math", "matrix", "linalg", …]` → `["…", "math", "ganita", …]`.

### Changed
- **`cyrius.cyml`** — toolchain pin `6.0.14` → `6.2.11`; `[deps] stdlib` adds
  `ganita`, drops `matrix` and `linalg` (now provided by `ganita`).
- **`src/f64_util.cyr`** — removed the local `f64_le`/`f64_ge` definitions. The
  stdlib `math` module now ships them (NaN-correct: a NaN operand yields 0 for
  all of `f64_lt`/`f64_gt`/`f64_eq`, so the inclusive forms correctly return 0),
  superseding hisab's negation-based versions. `f64_tan`/`f64_fmod`/
  `f64_copysign`/`f64_approx_eq` stay local (still absent from stdlib).
- **Vendored stdlib** — `lib/*.cyr` re-resolved to the 6.2.11 toolchain via
  `cyrius deps` (the manifest-driven resolver, per CLAUDE.md — **not** the
  full-snapshot `cyrius lib sync`, which over-vendors unused platform variants
  like `process_agnos.cyr`/`*_win`/`*_macos` and bloats the lock). Result: a
  minimal **30-file** `lib/` and a 30-entry `cyrius.lock` that matches
  `cyrius deps --verify` exactly (30/30) — fixing the CI hash-verify failure on
  the spurious `lib/process_agnos.cyr` entry that the snapshot had pulled in.
- **`dist/hisab.cyr`** — regenerated via `cyrius distlib` (header now v2.6.6).

### Fixed
- **Tracked toolchain issues re-verified on 6.2.11** (doc-health mandate at each
  pin bump): **3 of 5 now fixed upstream** —
  `cbt` `[build]`-comment `modules` substring false-positive,
  the cc5 18-arg-fn register/stack scramble (Perlin `_perm_init` corruptor),
  and `cyrius lint` returning the warning count as its exit code (now returns 0;
  the CI `^\s*warn ` stdout grep remains the load-bearing warning gate). Archived
  to `docs/development/issues/archived/`. Still open: the C-style
  `for (;cond;)` empty-clause parse error (workaround: `while`). The CLI
  unknown-flag-clobbers-source issue was not re-tested (destructive).

## [2.6.5] - 2026-05-30 — 2.6.x closeout: P(-1) / security audit + diffgeo equation reference

Closeout pass for the 2.6.x differential-geometry arc — P(-1) cleanliness, a
memory-safety / numerical review of the seven new `diffgeo.cyr` functions, and
the documentation deliverable. **No new vulnerability; no source change.**
Docs-only. The 2.6.x arc is complete.

### Added
- **`docs/architecture/math.md` §2 — Differential Geometry** — curvature
  conventions (Riemann `R^ρ_{σμν}`, the load-bearing sphere sign `R^θ_{φθφ}=+1`),
  the sectional / Weyl / Jacobi formulas, the transport ODE, and the
  exterior-algebra grading — each with its pinned identities and references
  (do Carmo; Lee; Wald; Misner-Thorne-Wheeler). The catalogue index now points at §2.
- **`docs/audit/2026-05-30.md`** — the closeout audit report.

### Security
- Reviewed the seven new diffgeo functions: bounded contraction loops; the RK4
  transport allocates fixed work arrays once per call; `weyl_tensor`'s `dim⁴`
  allocation matches its Riemann input (dim inherits `riemann_tensor`'s `≤ 16`
  cap). All divisions guarded or structurally safe — `sectional_curvature`
  degenerate-plane guard, `weyl_tensor` `n<3` guard, transport `/6`;
  `geodesic_deviation` and the wedges have no division. No new
  shell/FFI/syscall surface. Posture solid.

### Changed
- **`threat-model.md`** — recorded the sectional degenerate-plane guard, the Weyl
  `n<3` guard, and the 4D-form contract; added the 2026-05-30 audit-history entry.
  **`doc-health.md`** — math.md §2 logged + the new audit; bumped to v2.6.5.

## [2.6.4] - 2026-05-29 — Higher-order differential forms (2.6.x arc)

Fifth (last feature) patch of the 2.6.x arc. Extends the exterior algebra past
2-forms. Additive; suite 949 → **957**.

### Added
- **`wedge_2_1(omega, alpha)`** (2-form ∧ 1-form → 3-form) and **`wedge_3_1(beta,
  alpha)`** (3-form ∧ 1-form → 4-form) in `diffgeo.cyr`, in the same reduced
  strictly-increasing basis as `wedge_1_1`. Chaining `wedge_1_1 → wedge_2_1 →
  wedge_3_1` builds every basis k-form up to the 4D top form.
- **`tests/hisab.tcyr`** — 8 assertions: `e0∧e1∧e2 = (012)` and the unit 4-form
  `e0∧e1∧e2∧e3`; graded antisymmetry `e1∧e0 = −(e0∧e1)` and nilpotence `e0∧e0 = 0`;
  permutation signs (`e0∧e2∧e1 = −(012)`, `e1∧e2∧e3∧e0 = −(0123)`); and a repeated
  factor → 0.

## [2.6.3] - 2026-05-29 — Geodesic deviation / Jacobi equation (2.6.x arc)

Fourth patch of the 2.6.x arc. Adds the tidal acceleration between neighbouring
geodesics. Additive; suite 943 → **949**.

### Added
- **`geodesic_deviation(riemann, u, jac, dim)`** (`diffgeo.cyr`) — the Jacobi
  tidal term `D²J^ρ/dτ² = −R^ρ_{σμν} u^σ J^μ u^ν` (the J and the second `u` sit in
  Riemann's antisymmetric last-index pair, so the contraction is nonzero). For a
  space form it reduces to `−(|u|² J − ⟨u,J⟩ u)`.
- **`tests/hisab.tcyr`** — 6 assertions on the unit-sphere space form: a
  separation `⊥ u` gives `J'' = −J` (geodesics converge), `∥ u` gives `0`, the
  result scales with `|u|²`, flat space → `0`, and the operator is linear in `J`.

## [2.6.2] - 2026-05-29 — Parallel transport along a curve (2.6.x arc)

Third patch of the 2.6.x arc — the first that adds connection *integration*
(RK4) rather than an algebraic tensor combination. Additive; suite 939 → **943**.

### Added
- **`parallel_transport(v0, curve_vel, christoffel, dim, dt, steps)`**
  (`diffgeo.cyr`) — RK4 integration of `dV^a/dt = −Γ^a_{μν} V^μ ẋ^ν` along a
  curve with constant coordinate velocity `ẋ` (Christoffels held constant per
  step, same convention as `geodesic_rk4`). Plus the `_pt_deriv` RHS helper.
- **`tests/hisab.tcyr`** — 4 assertions: flat space (`Γ=0`) leaves the vector
  unchanged; on a **unit-sphere latitude circle at θ=π/4** (Γ^θ_φφ=−½,
  Γ^φ_θφ=1, metric `diag(1,½)`) transport along `φ` **preserves `⟨V,V⟩`**
  (metric compatibility) and genuinely rotates the vector.

## [2.6.1] - 2026-05-29 — Weyl conformal-curvature tensor (2.6.x arc)

Second patch of the 2.6.x arc. Adds the trace-free (conformally-invariant) part
of the Riemann tensor. Additive; suite 934 → **939**.

### Added
- **`weyl_tensor(riemann, ricci, scalar, metric, dim)`** + **`weyl_get`**
  (`diffgeo.cyr`) — the covariant Weyl tensor `C_{ρσμν}` assembled from the
  lowered Riemann minus the dimension-weighted Ricci/scalar trace terms
  (`1/(n−2)` and `R/((n−1)(n−2))`). Returns all-zero for `n ≤ 2` (Weyl
  undefined / identically zero there).
- **`tests/hisab.tcyr`** — 5 assertions + space-form/trace helpers: `C = 0` for a
  **3D space form** (Riemann & Ricci nonzero), `C = 0` for a **4D space form**
  (conformally flat, Riemann ≠ 0 — the defining oracle), a non-space-form 4D
  Riemann gives `C ≠ 0`, and that `C` is **trace-free** (`g^{ρμ} C_{ρσμν} = 0`).

## [2.6.0] - 2026-05-29 — Sectional curvature (2.6.x arc)

First patch of the 2.6.x arc (differential-geometry depth). Adds the sectional
curvature on top of the existing Riemann tensor. Additive; suite 929 → **934**.

### Added
- **`sectional_curvature(riemann, metric, u, v, dim)`** (`diffgeo.cyr`) —
  `K(u,v) = R(u,v,v,u) / (⟨u,u⟩⟨v,v⟩ − ⟨u,v⟩²)`, lowering the first Riemann index
  with the metric (`R_{ασμν} = g_{αρ} R^ρ_{σμν}`) and contracting over the plane.
  Guards a degenerate plane (`|denominator| < EPSILON → 0`). Positive for a
  sphere — verified against hisab's Riemann convention (`R^θ_{φθφ} = +1` at the
  equator). Plus a `_dg_inner` metric-inner-product helper.
- **`tests/hisab.tcyr`** — 5 assertions against a constant-curvature space form:
  unit-curvature → `K = 1` for any plane (axis-aligned + skew); a radius-2 sphere
  (`metric = diag(4,4)`) → `K = 1/4` (exercises the metric index-lowering); flat →
  `0`; and a degenerate plane → `0` (denominator guard).

### Documentation
- Roadmap maintenance: removed the completed 2.5.x arc section (now in Release
  History) and condensed the Current arc-history; re-organized the flat 2.6.0
  "differential geometry depth" item into a **2.6.x arc** (2.6.0 sectional
  curvature → 2.6.1 Weyl → 2.6.2 parallel transport → 2.6.3 geodesic deviation →
  2.6.4 higher-order forms → 2.6.5 closeout), grounded in the existing
  `diffgeo.cyr` primitives with known-manifold test oracles per patch.

## [2.5.4] - 2026-05-29 — 2.5.x closeout: P(-1) / security audit + equation catalogue

Closeout pass for the 2.5.x arc — P(-1) cleanliness, a memory-safety / numerical
review of the new CGA operators + `mat_new_guarded`, and the documentation
deliverable. **No new vulnerability; no source change** (the arc's additions are
bounded and guarded). Docs-only.

### Added
- **`docs/architecture/math.md`** — equation catalogue (earns the CLAUDE.md
  "mathematical reference" doc slot). Documents the conformal model / metric /
  blade layout; the product family (geometric, outer, left/right contraction with
  grade rules); reverse / norm / blade inverse; dual + pseudoscalar; projection /
  rejection — each with the pinned GA identities and literature references
  (Dorst-Fontijne-Mann, Hestenes-Sobczyk, Doran-Lasenby) — plus a catalogue index
  pointing at the rest of the library's formula material.
- **`docs/audit/2026-05-29-cga-arc-closeout.md`** — the closeout audit report.

### Security
- Reviewed the six 2.5.x functions: results use fixed 256-byte allocations,
  loops are bounded to blade indices 0..31 (no computed/unbounded indexing),
  `cga_blade_inverse` guards null blades, `cga_pseudoscalar_inv`'s divisor is the
  structural constant `⟨I~I⟩₀ = −1`, and `mat_new_guarded` caps dims (CWE-190).
  No new shell/FFI/syscall surface. Posture solid.

### Changed
- **`threat-model.md`** — CGA division guards added to the attack-surface table;
  2.5.4 audit-history entry. **`overview.md` / `README.md`** now reference
  `math.md`. **`doc-health.md`** — math.md marked earned + the new audit logged.

### Notes
- Stdlib `mat_new` overflow remains **upstream / deferred** (cyrius pin unchanged);
  `mat_new_guarded` is the mitigation. The CGA identity tests (29 assertions
  across 2.5.0–2.5.3) pin the equation material catalogued in `math.md`.

## [2.5.3] - 2026-05-29 — `mat_new` overflow guard (2.5.x arc)

Final feature patch of the 2.5.x arc (the 2.5.4 audit/doc pass closes it out). Carryover from the 2.4.6 audit (P(-1) C3): stdlib
`matrix.cyr` `mat_new(rows, cols)` computes `16 + rows*cols*8` with **no guard**
and is still unguarded in the pinned 6.0.14 (the cyrius fix is upstream/later).
This adds the hisab-side mitigation. Suite 925 → **929**; the 2.5.x arc is complete.

### Added
- **`mat_new_guarded(rows, cols)`** (`linalg_ext.cyr`) — overflow-guarded
  real-matrix constructor (CWE-190). Returns null for non-positive or
  overflow-prone dimensions (cap `_MAT_MAX_ELEMS = 16M`, keeping `rows*cols`
  well below 2⁶⁰ so `16 + rows*cols*8` can't wrap); otherwise delegates to
  stdlib `mat_new`. The guarded entry point for matrices sized from untrusted /
  external input — parity with the existing `cmat_new` guard. hisab's internal
  callers (SVD/QR, optimize) pass dims from already-allocated matrices and are
  unaffected.
- **`tests/hisab.tcyr`** — 4 CWE-190 regression assertions (huge dims, zero rows,
  over-cap 25M-element, and a valid small matrix).

### Notes
- The stdlib `mat_new` fix remains **upstream / roadmap-tracked** — re-verify when
  the cyrius pin moves; until then `mat_new_guarded` is the safe constructor and
  hisab's direct `mat_new` calls stay mitigated (dims from existing matrices).

## [2.5.2] - 2026-05-29 — CGA blade projection / rejection (2.5.x arc)

Third patch of the 2.5.x arc — the last CGA piece, composing 2.5.0's contraction
with a blade inverse. Additive; suite 915 → **925**.

### Added
- **`cga_blade_inverse(b)`** — `B⁻¹ = reverse(B) / (B · reverse(B))_scalar`, with a
  zero-norm guard (returns the zero multivector for a **null** blade, e.g. conformal
  points, instead of dividing by zero).
- **`cga_project(x, b)`** — `project(X,B) = (X ⌋ B) ⌋ B⁻¹`, the component of `X`
  lying in blade `B`. Preserves grade(X) (inner contraction lowers by grade(X),
  outer raises it back).
- **`cga_reject(x, b)`** — `X − project(X,B)`, the part of `X` orthogonal to `B`
  (grade-consistent since projection preserves grade).
- **`tests/hisab.tcyr`** — 10 assertions: `project(e1,e12) = e1` (in-plane, no e2
  leak), `project(e12,e12) = e12` (self), `project(e3,e12) = 0` / `reject(e3,e12) = e3`
  (orthogonal), `reject(e1,e12) = 0` (in-plane), idempotence, `project + reject = X`
  for `X = e1+e3`, and the null-blade guard (`project(e1, 0) = 0`, no trap).

## [2.5.1] - 2026-05-29 — CGA dual + pseudoscalar inverse (2.5.x arc)

Second patch of the 2.5.x arc. Adds Hodge-style duality on top of 2.5.0's
contraction. Additive (new public functions); suite 909 → **915**.

### Added
- **`cga_pseudoscalar()`** — the unit pseudoscalar `I = e1∧e2∧e3∧ep∧em` (grade-5
  blade `[31]`).
- **`cga_pseudoscalar_inv()`** — `I⁻¹ = reverse(I) / (I · reverse(I))_scalar`. In
  this metric (em² = −1) `I² = −1`, so `I⁻¹ = −I` — derived from the geometric
  product rather than hard-coded.
- **`cga_dual(x)`** — `x* = x · I⁻¹`, mapping a grade-k blade to grade `5−k`
  (since `I` is the top blade, this geometric product is exactly the contraction
  onto the pseudoscalar).
- **`tests/hisab.tcyr`** — 6 assertions: `I · I⁻¹ = 1`; grade flips `dual(1) = −I`
  (0→5) and `dual(I) = 1` (5→0); `dual(e1) = −e23pm` (1→4) with no residual e1;
  and the involution `dual(dual(e1)) = −e1` (pins the −x sign for this metric).

## [2.5.0] - 2026-05-29 — CGA contraction operators + doc catchup (2.5.x arc)

First patch of the 2.5.x arc (CGA depth + matrix guard). Conformal geometric
algebra already had the geometric/outer products, reverse, sandwich, and
conformal constructors; this adds the **interior-product** primitives. Additive
(new public functions, no signature changes); suite 901 → **909**. This release
also folds in the post-2.4.6 documentation catch-up (the repo-wide sweep + the
new `docs/doc-health.md` ledger), recorded below.

### Added
- **`cga_left_contraction(a, b)`** (a ⌋ b) and **`cga_right_contraction(a, b)`**
  (a ⌊ b) in `geo_advanced.cyr` — the contraction interior products. Same blade
  loop as `cga_outer_product` with a grade-difference selector: left keeps
  `grade(b) − grade(a)`, right keeps `grade(a) − grade(b)` (negative targets
  never match a non-negative blade grade, so out-of-range terms drop to zero).
- **`tests/hisab.tcyr`** — 8 CGA contraction assertions pinning the GA identities:
  `e1 ⌋ e12 = e2` (grade lowering), `e1 ⌋ e1 = 1` and `(2e1+3e2) ⌋ self = 13`
  (vector self-contraction → scalar norm²), `e1 ⌋ e23 = 0` (orthogonal), scalar
  contraction as scaling, and right contraction `e12 ⌊ e1 = −e2` with its grade
  drop. CGA went from 1 smoke assertion to 9.

### Documentation
- Repo-wide documentation sweep to v2.4.6 state: pruned the roadmap to
  forward-facing / deferred work only (the completed 2.3.x and 2.4.x arcs now
  live in Release History); refreshed README (added the **Collision** module row
  + spatial/einsum/lie_ext/linalg_precision/noise_simplex; version, counts,
  binary size); completed the `architecture/overview.md` module map (all 34
  modules + a Collision section) and updated the collision data-flow (MPR +
  sequential-impulse); synced test/benchmark counts (901 assertions / 26
  benchmarks) in `testing.md`; added collision + supply-chain entries to
  `SECURITY.md`; added a 2026-05-29 status addendum to the dated `port-audit.md`;
  refreshed `dependency-watch.md` and the CLAUDE.md status line.
- Added `docs/doc-health.md` — a living doc-currency ledger (fresh/stale/dated/
  tracked-issue per file), adapted from `cyrius/docs/doc-health.md`.
- Re-ran `scripts/bench-history.sh` (benchmarks.md was stale at an old commit) —
  refreshed to v2.4.6, 26 benchmarks.
- Removed `docs/development/tool-issues.md` (ad-hoc catalog; real bugs are filed
  individually under `docs/development/issues/`). Archived the shipped
  `cyrius-linalg-proposal.md` to `docs/development/archive/`. Fixed CONTRIBUTING
  currency (Cyrius pin 6.0.14, `src/` not `lib/`, +fmt/distlib gates).
- Re-verified the five `docs/development/issues/` toolchain filings against the
  pinned 6.0.14 — all still reproduce (none stale-fixed).
- Re-organized the roadmap's flat 2.5.0 item into a **2.5.x CGA arc** (2.5.0
  contraction → 2.5.1 dual → 2.5.2 projection/rejection → 2.5.3 `mat_new` guard
  closeout), mirroring the 2.4.x per-patch + commit-bite structure.

## [2.4.6] - 2026-05-29 — Security & hardening audit; 2.4.x closeout

P(-1) hardening step closing the 2.4.x arc: a memory-safety / allocation-overflow
review of the whole library, supply-chain integrity, dependency CVE posture
(known + web research), and a security review of the six collision algorithms
brought into the build chain across 2.4.0–2.4.5. **No new vulnerability found.**
Suite 895 → **901**. Full report: `docs/audit/2026-05-29.md`.

### Added
- **`tests/edge_cases.tcyr`** — 6 allocation-overflow-guard regression assertions
  (CWE-190): `cmat_new` rejects huge / non-positive dims and allocates valid ones;
  `tensor_new` rejects rank > 8 and overflow-prone shapes and allocates valid ones.
  Pins the documented guards so removing one fails the build.

### Security
- **Audit verdict — posture solid.** No shell/exec/FFI/libc surface (no CWE-78).
  hisab's own dimension allocators all guard the multiply before allocating
  (`tensor_new`, `cmat_new`, christoffel/riemann, num_sieve). All iterative paths
  are capped. Collision index access goes through `vec_get`, which traps on OOB
  rather than corrupting memory.
- **Supply chain**: `cyrius deps --verify` → 60 verified / 0 failed;
  `cyrius vet` → 0 untrusted. No third-party deps (cyrius stdlib + first-party
  sakshi only) → zero third-party-CVE surface. No CVEs exist for the cyrius/cycc
  toolchain.
- **Open (upstream)**: stdlib `mat_new` integer overflow (`16 + rows*cols*8`,
  unguarded) is still present in the pinned 6.0.14 snapshot. hisab's usage is
  mitigated (dims come from already-allocated matrices; raw-dim `cmat_new` is
  guarded). Fix belongs in cyrius stdlib — tracked as roadmap 2.5.0.

### Changed
- **`docs/development/threat-model.md`** — added the six 2.4.x collision algorithms
  to the attack-surface table; refreshed the `mat_new` note for 6.0.14 + roadmap
  2.5.0; added a Supply Chain section and a 2026-05-29 audit-history entry.

## [2.4.5] - 2026-05-29 — Collision arc: contact solver fixed; arc complete (2.4.x)

Final patch of the collision-correctness arc, and the third with a real bug.
`sequential_impulse` produced **all-zero impulses** — it did nothing.
`solve_pgs` (the general PGS/LCP solver in `linalg_ext`) was already correct but
barely tested. Suite 888 → **895**. This closes the 2.4.x arc.

### Fixed
- **`sequential_impulse` normal impulse** (`collision_core.cyr`): the impulse was
  computed as `(1+e)·v_n / inv_mass` with `v_n = −pen` — i.e. **negative** for a
  penetrating contact (the leading `−` the comment specified was missing), so
  `max(0, accumulate)` clamped it to 0 on every iteration. Separately, the loop
  re-added the full bias each iteration without folding the accumulated impulse
  back into the velocity, so it could never converge. Rewritten as proper
  sequential impulse: `v_eff = v_n + λ·inv_mass`, `Δλ = −(1+e)·v_eff / inv_mass`,
  accumulate-and-clamp. The total impulse now converges to `(1+e)·pen / inv_mass`
  (e.g. 0.25 for pen=0.5, inv_mass=2) and is iteration-independent.

### Added
- **`tests/modules.tcyr`** — 4 `sequential_impulse` assertions: resting contact
  (λ=0.25, the case that was 0), single-iteration convergence (idempotence),
  dynamic-vs-static (λ=1.0), and both-static (skipped → 0).
- **`tests/hisab.tcyr`** — strengthened the `solve_pgs` test from a lone `x1 > 0`
  sanity check to exact-solution assertions (`x ≈ [1/11, 7/11]` for
  `[[4,1],[1,3]]x=[1,2]`) plus a **bounds-active** LCP (`diag(1,1)·x=[5,5]` clamped
  to `hi=[2,2]` → `[2,2]`). solve_pgs was audited correct — no change.

### Notes — 2.4.x arc complete
- All six collision algorithms audited and covered. Genuine bugs found + fixed in
  three: the hull insertion sort + undefined `f64_le`/`f64_ge` (2.4.0), the MPR
  origin-containment early-out (2.4.4), and this impulse-sign/convergence fix
  (2.4.5). `triangulate_polygon`, `delaunay_2d`, the half-edge mesh, and
  `solve_pgs` were already correct — they just lacked coverage, which they now have.

## [2.4.4] - 2026-05-28 — Collision arc: MPR narrowphase fixed (2.4.x arc)

Fifth patch of the collision-correctness arc, and the second with a real bug
(after 2.4.0's hull). `mpr_intersect` / `mpr_penetration` (Minkowski Portal
Refinement, 3D) returned a **false positive on every separated pair** — they
never detected non-overlap. Fixed; suite 878 → **888**.

### Fixed
- **MPR origin-containment early-out** (`collision_core.cyr`): after building the
  first portal support point `v1` (the farthest Minkowski-difference point toward
  the origin), the code lacked the standard check that the origin is actually
  reachable. For separated shapes `v1` falls short of the origin, and — when the
  centers are colinear with it — execution dropped into the degenerate
  `(v1−v0)×(−v0) == 0` branch whose comment says "check if between them" but which
  **unconditionally returned 1 (hit)**. Off-axis separated pairs slipped through
  the refinement loop's convergence test the same way. Added
  `if dot(v1, dir) < 0 → miss` to both `mpr_intersect` (returns 0) and
  `mpr_penetration` (returns 0); once it passes, the origin is genuinely within
  the support toward it and the existing portal refinement is correct.

### Added
- **`tests/modules.tcyr`** — 10 MPR assertions + sphere/box support-function
  helpers: sphere-sphere overlap (hit) vs separated on- and off-axis (miss — the
  regression); penetration depth (`sum_radii − dist` = 1.0, plus the deeper 3.0
  case) with a ±x normal; separated → miss; and box-box overlap / separated.

### Notes
- Of the five arc patches so far, only 2.4.0 (hull sort) and 2.4.4 (MPR) carried
  genuine bugs; triangulate / delaunay / half-edge were already correct.

## [2.4.3] - 2026-05-28 — Collision arc: half-edge mesh audited (2.4.x arc)

Fourth patch of the collision-correctness arc. The roadmap flagged the half-edge
mesh (`halfedge_from_triangles` + `halfedge_adjacent_faces` + `halfedge_is_boundary`)
for a twin-pointer-wiring check — but a thorough audit found it **already correct**
on both closed and open meshes. Deliverable: coverage, no source change; suite
867 → **878**.

### Added
- **`tests/modules.tcyr`** — 11 half-edge assertions + `_he_tri` / `_he_boundary_sum`
  helpers, over four reference meshes: a **tetrahedron** (closed manifold → no
  boundary vertices, 3 neighbours/face), a **single triangle** (open → all-boundary,
  no neighbours), a **2-triangle quad** sharing a diagonal (4 boundary corners, 1
  shared edge), and a **hexagon fan** (center interior, 6-vertex boundary rim — the
  case that exercises the one-ring walk closing on an interior vertex). Plus the
  empty-input → null-mesh error path.

### Notes (audit — no change needed)
- Twin assignment pairs each directed half-edge `src→dst` with the reverse
  `dst→src` (O(n²) scan); closed meshes wire every twin, open meshes leave
  boundary half-edges at `_COL_SENTINEL`. `halfedge_is_boundary` walks the
  one-ring (`next→next→twin`) with a 1000-step guard, returning 1 on any sentinel
  twin and 0 only when the ring closes back to the start. `halfedge_adjacent_faces`
  collects the twin's face for each shared edge. All correct.

## [2.4.2] - 2026-05-28 — Collision arc: `delaunay_2d` audited (2.4.x arc)

Third patch of the collision-correctness arc. The roadmap flagged
`delaunay_2d` (Bowyer-Watson) as needing a numerical-stability pass — but a
thorough audit found it **already correct and robust**, including the cocircular
grid stress case. The deliverable is coverage that verifies the empty-circumcircle
property, plus an honest "no bug" record. No source change; suite 859 → **867**.

### Added
- **`tests/modules.tcyr`** — 8 `delaunay_2d` assertions + a `_dl_violations`
  helper that counts points strictly inside any output triangle's circumcircle
  (the Delaunay property holds iff that count is 0). Fixtures: square + interior
  (4 triangles), a **3×3 grid** (heavy cocircularity — the stability stress
  case, 8 triangles), an irregular 6-point set, a single triangle (`n = 3`), and
  degenerates (`n < 3` and all-collinear → empty, no trap). Every non-degenerate
  case has **0 circumcircle violations**.

### Notes (audit — no change needed)
- The in-circle predicate (`_col_in_circumcircle`) adjusts the determinant by the
  triangle's winding sign and tests **strict** interiority, so cocircular points
  (grid corners) are correctly not flagged — that's why the grid triangulates
  cleanly. Super-triangle setup/removal, bad-triangle cavity collection, boundary
  (non-shared) edge extraction, descending swap-remove, and CCW-normalized output
  are all correct.
- Collinear / `n < 3` inputs return an empty triangulation (all triangles
  reference super-triangle vertices and are filtered) rather than trapping.

## [2.4.1] - 2026-05-28 — Collision arc: `triangulate_polygon` audited (2.4.x arc)

Second patch of the collision-correctness arc. The roadmap flagged
`triangulate_polygon` (ear clipping) as a likely-buggy untested port — but a
thorough audit found it **already correct**. The deliverable is comprehensive
coverage that promotes it from "deliberately unexercised" to verified, plus an
honest record that no bug existed. No source change; suite 846 → **859**.

### Added
- **`tests/modules.tcyr`** — 13 `triangulate_polygon` assertions + a `_tp_area2x`
  helper. Each fixture checks the triangle count (`3*(n−2)` flat indices) and
  that the output **tiles the polygon** (|Σ 2×area| == polygon |2×area|; abs ⇒
  winding-agnostic): convex quad, concave 5-gon (reflex vertex), convex hexagon,
  U-shape (2 reflex vertices), CW-wound quad, a collinear edge vertex, and the
  `n < 3` empty case.

### Notes (audit — no change needed)
- Ear detection (reflex classification via signed area + point-in-triangle),
  winding normalization (CCW/CW), and the shrink-as-clipped index bookkeeping
  are all correct. The `n*n` iteration cap + "no ear found → bail" guard make
  degenerate inputs (e.g. duplicate vertices) terminate without trapping.
- Unlike `convex_hull_2d` (2.4.0, two real bugs), `triangulate_polygon` shares
  no code with the broken sort and never referenced the undefined `f64_le`/
  `f64_ge` — it uses only the strict `f64_lt`/`f64_gt` intrinsics, which is why
  it was sound where the hull was not.

## [2.4.0] - 2026-05-28 — Collision correctness arc: `convex_hull_2d` (2.4.x arc)

First patch of the 2.4.x collision-correctness arc. `convex_hull_2d` (Andrew's
monotone chain) carried two pre-existing port bugs and trapped on any non-trivial
input — it had never run, having sat outside the build chain until 2.2.2. Both
bugs are fixed and the algorithm is now exercised + covered. Unlike the 2.3.x
arc this **changes behavior** (a trapping function now returns correct results),
so it follows a red-fixture-first cadence.

### Fixed
- **`convex_hull_2d` insertion sort** (`collision_core.cyr`,
  `_col_sort_indices_by_xy`): the hand-rolled sort's `done`-exit path
  overwrote the insertion index with `-1` on every *mid-array* insertion, so
  `vec_set(indices, -1, key)` tripped a `vec: index < 0` trap (front insertions
  happened to work). Rewritten as a standard insertion sort — shift-greater
  then drop `key` at `sj + 1` (always `>= 0`).
- **Missing `f64_le` / `f64_ge` primitives** (`f64_util.cyr`): the monotone
  chain's pop test calls `f64_le(cross, 0)`, but `f64_le`/`f64_ge` were never
  defined (only the strict `f64_lt`/`f64_gt` exist as named-op intrinsics) —
  so once the sort was fixed and execution reached the chain, the call bound to
  an undefined symbol and trapped (SIGILL). Defined both in terms of the strict
  intrinsics (`a <= b == !(a > b)`, `a >= b == !(a < b)`). This also resolves
  the same latent undefined-symbol references at 6 call sites in `spatial.cyr`
  (point-in-region tests) that current tests didn't reach.

### Added
- **`tests/modules.tcyr`** — 13 `convex_hull_2d` assertions: the keystone
  square + interior-point fixture (4-vertex CCW hull, shoelace 2×area = +8),
  plus degeneracy coverage — empty/single point, triangle (count + CCW area),
  collinear set (→ 2 extreme endpoints, interior dropped), and a duplicated
  corner (→ still 4 vertices). None trap. Suite: 833 → **846**.

## [2.3.4] - 2026-05-28 — Layout & idiom modernization (2.3.x arc)

A **layout/idiom** patch: tie heap allocations and field writes to the typed
struct definitions so a future field addition can't silently under-allocate or
land at a stale offset. Internal-only — public API, results, and machine code
are unchanged. All 833 tests pass **bit-identical**; key benchmarks verified
**flat** (`sizeof(T)` is a compile-time constant equal to the old literal, and
a derived setter lowers to the same `store64` at the same offset, so codegen is
byte-identical — no perf claim).

### Changed
- **Constructors → `sizeof(T)` + derived setters** across all 13 `#derive(accessors)`
  struct modules: `alloc(<magic-literal>)` → `alloc(sizeof(T))` and
  `store64(p + <offset>, v)` → `T_set_field(p, v)`. This eliminates the last
  hand-computed offsets (reads already used the derived getters) and couples
  every allocation to its struct layout:
  - **vec2/vec3/vec4** (`HVec2`/`HVec3`/`HVec4`), **quat** (`HQuat`),
    **complex** (`HComplex`), **autodiff** (`Dual`), **interval** (`Interval`),
    **tensor** (`HTensor` header), **geo** (all 9: `GeoRay`/`GeoPlane`/`GeoAabb`/
    `GeoSphere`/`GeoObb`/`GeoCapsule`/`GeoTriangle`/`GeoLine`/`GeoSegment`),
    **collision_core** (`ColContact`), **collision_mesh** (`HalfEdge`/`HalfEdgeMesh`).
  - SIMD scalar tails (e.g. vec3's `store64(r + 16, …)` z-write) also moved to
    `HVec3_set_z` — packed `f64v_*` writes stay as raw-memory ops.
- **Matrix grid sizes → enum constants**: `mat3.cyr` `MAT3_BYTES = 72` and
  `mat4.cyr` `MAT4_BYTES = 128` replace the magic byte-size literal that was
  repeated across `alloc` + copy-loop bounds (couples the copy length to the
  allocation length). `m3_set`/`m4_set` (column-major grids, no named fields)
  stay computed-index — a field-struct doesn't fit a flat NxN grid.
- **Float-render buffer → enum constant**: `FLOAT_RENDER_BUF = 32`
  (`var buf[FLOAT_RENDER_BUF]`) shared by the `symbolic.cyr` / `symbolic_ext.cyr`
  float renderers, replacing two bare `var buf[32]` literals.

### Added
- **`#must_use` on the core value-returning API** (9 modules: vec2/3/4, quat,
  complex, interval, autodiff, mat3, mat4) — the compiler now warns at build
  time if a pure constructor/operation result is discarded (a guaranteed bug
  for a side-effect-free function). Status-returning setters (`m3_set`/`m4_set`)
  are deliberately **not** annotated. Verified the full 34-module build and the
  833-test suite emit **zero** "result is discarded" warnings.

### Notes (evaluated — deferred or N/A)
- **`#pure`**: deferred. Its CSE/memoization semantics interact unsafely with
  hisab's allocate-a-fresh-result convention (two "equal" calls must return
  distinct mutable pointers); the win is speculative (hot paths are already SIMD)
  with no benchmark driver. Not worth the risk for a patch.
- **Slices (`[T]`/`slice<T>`)**: deferred. The hot loops are raw-pointer +
  `load64`/`store64`, already SIMD-optimized (2.3.1). Bounds-checked slices would
  regress them; `slice_unchecked_get_W` discards the safety benefit, leaving only
  churn. No consumer demand.
- **`defer`**: N/A. The library holds no per-resource lifecycles — long-lived
  data uses the bump/arena allocator (never individually freed) and the library
  opens no file descriptors. Nothing for `defer` to clean up.

## [2.3.3] - 2026-05-28 — Safety & numerical-correctness audit (2.3.x arc)

A correctness audit of the integer/bit-math surface against the vidya gotcha
catalogue (signed shifts, static buffers, element-vs-byte sizing, overflow).
**Verdict: no bugs found — hisab was already careful.** The deliverable is a
fixed misleading comment plus regression tests that pin the load-bearing
invariants, so a future toolchain change can't silently break them. 833/833
tests pass (was 825 + 8 new invariant assertions).

### Fixed
- **num.cyr PCG comment**: the note claimed "Cyrius `>>` is arithmetic
  (sign-extending)" — it is **logical** (zero-fill), verified on 6.0.14
  (`0x8000…0 >> 1 == 0x4000…0`). The PCG RNG is correct either way (it wants
  unsigned shifts and masks to 32 bits), but the comment actively misinformed.
  Corrected.

### Added
- **`tests/edge_cases.tcyr`** — safety-invariant section (8 assertions):
  - `>>` is logical/zero-fill (the PCG + bit-reversal code depends on it).
  - `_num_mulmod` (Russian-peasant) stays overflow-safe — Fermat check
    `2^(p−1) ≡ 1 (mod p)` for prime p≈1e11, whose intermediate squares (~1e22)
    far exceed i64 (a naive `a*b` would overflow).
  - PCG32 determinism/reproducibility for a fixed seed.
  - float-render `var buf[N]` independence across calls (`str_from_buf` copies,
    so the process-static buffer is safe).

### Notes (audit findings — no change needed)
- **Static buffers**: only two `var buf[32]` (float rendering), both passed to
  `str_from_buf`, which allocates + `memcpy`s — no escape, no cross-call
  corruption despite `var buf[N]` being process-static in Cyrius.
- **Bytes-vs-elements**: no bug — element arrays consistently use `alloc(n*8)`;
  the only stack arrays are byte buffers used as bytes.
- **Signed shifts**: every `>>`/`<<` operates on non-negative or masked values
  (FFT lengths, masked RNG state, non-negative exponents, bit-index masks), so
  logical `>>` is correct throughout. No `asr()` helper needed.
- **Overflow**: no factorial/binomial kernels exist (those are abaco's);
  Pollard-rho/modpow already use `_num_mulmod` to avoid `a*b` overflow. No
  unary `~` is used anywhere (all `~` are "≈" in comments).

## [2.3.2] - 2026-05-28 — Bounded einsum scratch memory (2.3.x arc)

A **memory-footprint** patch (not a speed change): einsum's parse/contraction
scratch now lives in a reused module-global arena instead of leaking to the
never-freeing bump allocator on every call. Internal-only; all 825 tests pass
with identical results.

This patch re-scoped 2.3.2 after verification (see Notes): the roadmap's
"replace hand-rolled transcendentals" item was a no-op — hisab already uses
the stdlib `f64_sin/cos/exp/ln/abs` named-op intrinsics everywhere, with zero
hand-rolled series — and arena adoption only fit einsum.

### Changed
- **einsum** (`src/einsum.cyr`): all ~11 per-call scratch allocations (operand
  specs, label dims, contraction sets, output shape) and the hot per-element
  `label_vals`/per-product `indices` buffers now come from a module-global
  arena (`_einsum_arena`, 8 KB) that is `arena_reset` at each call's entry.
  `label_vals`/`indices` are also hoisted to reused fixed-size buffers instead
  of being re-allocated per output element/product. einsum is non-reentrant
  and single-threaded; `tensor_new` copies the shape, so no scratch escapes
  into the returned tensor (verified).

### Performance (memory, not speed)
Measured per call for a 4×4 `ab,bc->ac` contraction (bump bytes consumed,
100-call average), before = HEAD's released 2.3.1 einsum:

- **3960 → 176 bytes/call** (~**22×** less). The 176 B is the result tensor
  the caller owns; the ~3784 B of per-call scratch leak is **eliminated** —
  scratch is now a one-time 808 B arena reused across all calls.
- For a consumer making N einsum calls: `N×3960` (unbounded growth) → `N×176
  + 808`. Speed is unchanged (arena alloc ≈ bump alloc, both O(1) bumps).

### Notes
- **Transcendentals already optimal**: `f64_sin/cos/exp/ln/abs` are stdlib
  named-op intrinsics (hardware FP on x86, aarch64 polyfills). hisab uses them
  directly (`f64_util.cyr` only composes them, e.g. `f64_tan = sin/cos`). No
  hand-rolled Taylor/Newton series exist to replace. No change made.
- **Arena scoped to einsum**: symbolic/tensor allocations are escaping result
  nodes (expr trees / tensors returned to the caller), not scratch, so an
  arena-reset would corrupt results — correctly left alone.

## [2.3.1] - 2026-05-28 — SIMD hot paths (2.3.x optimize/modernize arc)

First patch of the 2.3.x arc: route vec/mat/quat hot paths through Cyrius's
packed-double `f64v_*` builtins (added with hisab named as the gap-close
consumer). Internal-only — public API and results unchanged; all 825 tests
pass with **bit-identical** output to the scalar paths.

### Performance
Measured with an amplified microbench (loop timed by two `now_ns()` calls;
the committed `bench()` harness has a ~488 ns per-iteration floor that hides
op-level timings — see Notes). Before → after, ns/op:

- **vec4 dot**: 26 → 4 ns (**~6.5×**, no alloc)
- **vec4 add**: 46 → 20 ns (**~2.3×**)
- **vec3 dot**: 20 → 9 ns (**~2.2×**, n=2 xy + scalar z)
- **vec3 add**: 35 → 22 ns (**~1.6×**)
- **m4_mul**: 716 → 158 ns (**~4.5×** — scalar path made 64+ accessor calls)
- **m4_mul_vec4**: 106 → 59 ns (**~1.8×**)
- **m3_mul**: 349 → 108 ns (**~3.2×**, over-read-safe hybrid)

### Changed
- **vec4** (`src/vec4.cyr`): `dot`/`add`/`sub`/`scale` → `f64v_*` at n=4 (HVec4
  is exactly 4 contiguous f64; even n → no over-read). `length`/`length_sq`/
  `normalize` inherit via `dot`.
- **vec3** (`src/vec3.cyr`): `dot`/`add`/`sub`/`mul`/`scale` → over-read-safe
  hybrid (`f64v_*` at n=2 on the xy pair + scalar z tail). HVec3 is 24 B = 3
  doubles, so n=3 would read one f64 past the allocation.
- **mat4** (`src/mat4.cyr`): `m4_mul` and `m4_mul_vec4` → column-major linear
  combination (`f64v_scale`+`f64v_add` over contiguous 32-B columns).
  `m4_transform_point`/`dir` inherit it.
- **mat3** (`src/mat3.cyr`): `m3_mul`, `m3_mul_vec3` → over-read-safe hybrid.
- **quat** (`src/quat.cyr`): `dot` → `f64v_dot` n=4; `normalize` scale → `f64v_scale`.
  The Hamilton product stays scalar (not elementwise).

### Added
- **`tests/hisab.bcyr`**: amplified batch benchmarks (`vec3_dot_x64`,
  `vec4_dot_x64`, `m4_mul_x16`, `m4_transform_x64`) that loop N ops per timed
  call to clear the harness floor, so the SIMD wins are regression-trackable
  in `bench-history.csv`.

### Notes
- `f64v_*` are **global codegen intrinsics** (no `[deps]`/include needed). They
  MUST be reached through a function prologue: a bare `f64v_*` call from a
  top-level `.tcyr` statement SIGSEGVs on a misaligned stack (SSE alignment).
  All hisab ops are functions, so production + tests-via-wrappers are safe.
- FP associativity: SIMD pairwise-sum vs scalar left-fold gave bit-identical
  results across all 825 assertions — no tolerance regressions.

## [2.3.0] - 2026-05-28 — Cyrius 6.0.14 toolchain + library laid out in `src/`

Toolchain modernization to **Cyrius 6.0.14** (from 5.7.10) and a structural
move of the library source into `src/`, matching the sibling math engine
abaco's "proper library" layout. No behavioral changes — all 825 tests pass
unchanged. Reviewed abaco (modern patterns), and vidya + cyrius changelogs
(language updates) to scope the upgrade; the 6.0.x range is a low-risk jump
for a pure math library (no forced source-syntax breaks).

### Changed
- **Toolchain**: `cyrius.cyml` pin `5.7.10` → `6.0.14`. The 6.0.0 `cc5`→`cycc`
  binary rename is transparent to consumers (`cyrius build` dispatches; back-
  compat symlinks ship through 6.0.x).
- **Layout**: the 34 math modules moved `lib/*.cyr` → `src/*.cyr` (`git mv`).
  `lib/` is now exclusively vendored stdlib + first-party deps managed by
  `cyrius deps`; `src/` is the library source (smoke `main.cyr` + 34 modules).
  This stops the committed `./lib/` from shadowing the toolchain's
  version-pinned stdlib snapshot. Updated `[lib] modules` paths and the
  `include "lib/…"` → `include "src/…"` references across the 7 test/bench/
  fuzz/example files that exercise the library.

### Fixed
- **sakshi dependency**: `lib/sakshi_sakshi.cyr` was a dangling symlink into a
  stale `~/.cyrius/deps/sakshi/0.9.0` cache (a 5.7.x-era vendoring artifact),
  which broke the lockfile sha step (`sha256sum: No such file`). Cleared the
  bad 2.1.0 cache, re-resolved, and restored the vendored module as a regular
  file (per the 6.0.2 file-over-symlink fix). `[deps.sakshi]` (tag 2.1.0) is
  retained; `cyrius deps --verify` is green (60 verified, 0 failed — the lock
  now hashes only genuine vendored deps under `lib/`, down from 94 once the 34
  math modules moved to `src/`).
- **lint**: collapsed 7 "multiple consecutive blank lines" warnings in
  `src/geo_advanced.cyr` (4) and `src/spatial.cyr` (3) — newly gated now that
  the modules live under `src/`.
- **lint (global-init-order)**: 6.0.14's lint (the check landed at 5.7.32,
  after the old 5.7.10 pin) flagged 5 "global var init refs … (silent zero at
  init)" warnings in `tests/hisab.tcyr` — caused by duplicate top-level var
  names reused across test sections (`t`, `sx`, `sx2`, `se`), where an early
  reference resolved to a later redeclaration. Renamed the later (symbolic +
  tensor) instances to unique names (`symx`/`symx2`/`sexpr`/`tn`). Lint, fmt,
  and vet are now clean across the full CI file set (src + examples + tests).
- **fmt**: reformatted the whole tree (30 files) under 6.0.14's `cyrfmt` — the
  6.0.x formatter differs from the 5.7.10-era one, so files untouched by this
  change still carried latent drift. fmt is whitespace-only; 825/825 unchanged.

### CI (reviewed against abaco's workflows)
- **Fixed the fmt gate**: the old gate did `diff <(cyrius fmt "$f" --check) "$f"`
  — but `--check` writes its report to *stderr* and emits nothing on stdout, so
  the diff compared an empty stream to a non-empty file and failed **every**
  file. Switched to abaco's exit-code form (`cyrius fmt "$f" --check || fail`),
  aggregating across files so each drifted file is annotated.
- **Added a Security Scan job** (mirrors abaco): greps `src/` for raw
  `execve`/`fork` syscalls, `sys_system`, writes to `/etc|/bin|/sbin`, and
  oversized stack buffers (≥64 KiB). hisab's `src/` passes clean; now that the
  math modules live in `src/`, the scan covers the full library surface.
- **Hardened the docs version gate**: now also asserts the
  `version = "${file:VERSION}"` manifest template is intact (matches abaco),
  not just that the version appears in the CHANGELOG.

### Notes
- The empty 0-byte `cyrius.lock` (a symlink/file-copy bug present ecosystem-
  wide since 5.11.8, fixed at 6.0.2) now populates correctly — 60 SHA entries.
- `dist/hisab.cyr` (~545 KB / 16,195 lines) regenerated from the new `src/`
  paths; the `# Generated by: cyrius distlib` header keeps fmt/lint skipping
  the bundle (it exceeds the 512 KiB fmt/lint input cap).

## [2.2.2] - 2026-04-26 — Make it actually build under cc5 5.7.10 + full 34-module distlib

2.2.1 captured the manifest/CI/release modernization intent but was never
tagged because `cyrius build src/main.cyr` didn't pass — Cyrius 5.7.x's
new reserved keywords + an oversized include chain (the CLI prepended all
33 project modules, exceeding cc5's 512 KB input_buf) blocked the build.
2.2.2 closes those, picks up the **5.7.10** toolchain bump for its
`input_buf` 512 KB → 1 MB expansion (and 5.7.8's `syscall arity mismatch`
fix + lockfile-default + 5.7.9's duplicate-fn warning), restores
`lib/collision_core.cyr` + `lib/collision_mesh.cyr` to the bundle (after
fixing pre-existing syntax issues those files had carried since the
original Cyrius port — they were never actually compiled before), and
ships a full **34-module `dist/hisab.cyr` (~544 KB)** verified end-to-end
against a consumer build.

### Fixed
- **`lib/num_ext.cyr`**: renamed local variable `stack` → `stk` (6 identifier sites in `_factorize_pollard_rho`). `stack` became a reserved keyword in Cyrius 5.7.x; the four mentions in comments were left intact
- **`lib/collision_core.cyr`**: 3 empty-init / empty-step `for` loops converted to `while`. Cyrius's `for (init; cond; step)` requires *all three* clauses — `for (; cont == 1;)` and `for (; sj >= 0; sj = sj - 1)` were never valid syntax. The for-with-step variant kept its step semantics by appending `sj = sj - 1` to the loop body tail. File was never in the build chain pre-2.2.2 (orphan-include-after-syscall trick masked it), so this is the first time it actually parses
- **`lib/collision_mesh.cyr`**: same migration — one `for (var ti = 0; ti < n_tris;)` (empty step) converted to a manual `var ti = 0; while (ti < n_tris) { ...; ti = ti + 1 (or stay) }` (loop conditionally advances based on whether the current element was removed). Plus renamed local `shared` → `is_shared` (4 identifier sites — `shared` is reserved in Cyrius 5.5+; one comment mention left intact)
- **`lib/calc.cyr`** `_perm_init` (Perlin noise table): refactored the 18-arg `_perm_store_block(base, off, a..p)` helper into a 10-arg `_perm_store_8(base, off, a..h)` form, called 32× instead of 16×. **cc5 5.7.10 has a codegen bug at 18+ args** that scrambles register/stack params (args 1, 2, 7-12 silently read garbage values — see [`docs/development/issues/archived/2026-04-26-cc5-18-arg-fn-scrambles-params.md`](docs/development/issues/archived/2026-04-26-cc5-18-arg-fn-scrambles-params.md) for the full reproducer; the filing was archived when 6.2.11 fixed it). Pre-fix: bench segfaulted at exit 139 inside `perlin_2d` because the permutation table got written full of garbage. Post-fix: `perlin_2d` runs through 200,000 iterations cleanly
- **`src/main.cyr`**: stripped from 30+ project-module includes down to the two stdlib includes its `fn main()` actually uses (`syscalls`, `io`). The previous form prepended every project module just to "validate the include chain" — but cc5 5.7.7's 512 KB input_buf can't fit that, and the test suites already cover include integration. Bonus: fixed three orphan `include` lines that sat *after* `syscall(SYS_EXIT, r)` (parsed but unreachable; first time any of them was scrutinized was when one tripped a parse error). CLI binary: now ~140 KB static ELF, prints the version string and exits
- **`tests/modules.tcyr`**: stripped six "multiple consecutive blank lines" lint warnings (lines 43-45, 263-264, 390 in the pre-fix file). `cyrius lint` returns the warning count as its exit code, which the prior CI loop swallowed under GHA's `set -eo pipefail` — the loop would abort on the first non-zero rc without reporting which file tripped it
- **`examples/basic_math.cyr` + `tests/{edge_cases,foundation,hisab,modules}.tcyr`**: applied `cyrius fmt` to flatten multi-line continuation-indent drift from the modern formatter
- **CI lint loop**: added `set +e` and per-file rc capture so warning-count exit codes don't abort the sweep before later files report. Adds `::error file=...::` annotations so the offending file is visible in the GHA UI

### Added
- **`dist/hisab.cyr`** — full **34-module** distlib bundle (~544 KB / 16,200 lines), regenerated from `[lib]` via `cyrius distlib`. Consumers pull it as `[deps.hisab] modules = ["dist/hisab.cyr"]` — single self-contained file, no per-module `include` choreography. Verified end-to-end: a fresh `[deps] stdlib = [...]` consumer build of an `hvec3_dot` example exits 0 and prints expected output. Fits cc5 5.7.10's 1 MB input_buf with ~480 KB headroom
- **CI distlib drift gate** in `ci.yml` — `cyrius distlib` runs on every push; CI fails if the committed `dist/hisab.cyr` differs from what the current `lib/` produces, so consumers always pull a fresh bundle that matches `lib/`
- **Release distlib regeneration** in `release.yml` — bundle is regenerated and shipped as `hisab-<TAG>.cyr` alongside the source tarball + linux binary + `SHA256SUMS`
- **`tests/modules.tcyr`** — added `collision_core` (`contact_new` + `ColContact_*` accessors) + `collision_mesh` (`detect_islands` smoke) test groups. 4 new assertions; total 253 in this suite, 825 across all four. Convex-hull / Delaunay / MPR are deliberately not exercised yet — they trip pre-existing algorithmic bugs unrelated to the syntax migration; coverage will land alongside an audit pass in a follow-up release
- **`docs/development/tool-issues.md`** — catalog of Cyrius / cc5 / cbt / cyrius-lint / cyim quirks hisab development has tripped on. Language-specific items each have a deep-dive at `docs/development/issues/<DATE>-<slug>.md` with self-contained reproducers + verification steps, so the cc5 maintainer agent can pick them up when there's a slot. Five issues catalogued at 2.2.2: 18-arg fn miscompile, `for` empty-clauses parse, CLI clobbers source on unknown flag, `cyrius lint` rc-as-warning-count, `cbt modules` substring false-positive in `[build]` comments

### Changed
- **Toolchain pin**: 5.7.7 → **5.7.10**. Picks up:
  - **5.7.8** — cc5-level fix for the noisy `lib/syscalls_x86_64_linux.cyr:358: syscall arity mismatch` warning (was firing on every build of every downstream that includes syscalls), `cyrius deps` writing `cyrius.lock` by default, `cyrius check` no longer auto-prepending manifest deps, and the new `cyrius build --no-deps` flag
  - **5.7.9** — `warning: duplicate fn '<name>' (last definition wins)` at registration time (hisab build emits zero such warnings); `json_build` cross-module collision resolved upstream via patra rename
  - **5.7.10** — `input_buf` 512 KB → **1 MB** heap-map reshuffle (+0x100000 region shift). Hisab was the load-bearing reason for this bump per the cc5 5.7.10 release header — `dist/hisab.cyr` was at 96 % of the old cap with hisab actively censoring upstream to stay under
- **`[lib]` block**: full 34-module list restored (32-module trimmed bundle from earlier 2.2.2 drafts is no longer needed). `lib/collision_core.cyr` + `lib/collision_mesh.cyr` re-included after their syntax fixes
- **`README.md` quick-start**: `[deps.hisab] modules = ["dist/hisab.cyr"]` example back; toolchain version → 5.7.10
- **`CLAUDE.md` status + layout**: reflects the CLI smoke binary + 34-module distlib bundle; CI/Release section restores the distlib drift gate

### Local verification
All gates pass locally on this branch:
- `cyrius lint` — clean across 8 files (4 tcyr, 1 bcyr, 1 fcyr, 1 example, src/main.cyr)
- `cyrius fmt --check` — no drift
- `cyrius vet src/main.cyr` — clean
- `cyrius distlib` — `dist/hisab.cyr` = 544 KB / 16,200 lines (34 modules)
- `cyrius build src/main.cyr build/hisab` — OK, 143 KB static ELF, magic verified
- `./build/hisab` — prints `hisab 2.2.2`, exit 0
- `cyrius test tests/hisab.tcyr` — 116/116
- `cyrius test tests/foundation.tcyr` — 307/307
- `cyrius test tests/modules.tcyr` — **253/253** (added 4 collision smoke assertions)
- `cyrius test tests/edge_cases.tcyr` — 149/149
- Total: **825/825 assertions, 0 failed**
- `cyrius build tests/hisab.fcyr` + 5 s timeout run — fuzz: ok
- `cyrius bench tests/hisab.bcyr` — 22 benchmarks complete
- **Consumer smoke**: a fresh `[deps.hisab] modules = ["dist/hisab.cyr"]` consumer project compiles + runs the bundle end-to-end (exits 0, prints expected output)

### Known limitations (deferred)
- `convex_hull_2d` / `triangulate_polygon` / `mpr_intersect` / `delaunay_2d` / `halfedge_from_triangles` / `sequential_impulse` — collision_core and collision_mesh now compile and link, but the algorithms themselves trip runtime bugs (e.g. `convex_hull_2d` hits a `vec: index < 0` bounds check on a 5-point input). These predate the cc5 5.7.x port — the files were added in 2.2.0 but never actually exercised because they sat outside the build chain. Audit + fixes are queued for a follow-up release; for now `tests/modules.tcyr` only smoke-tests the API surface (`contact_new` + accessors, `detect_islands`)
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
