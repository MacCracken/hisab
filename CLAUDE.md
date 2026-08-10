# Hisab — Claude Code Instructions

## Project Identity

**Hisab** (Arabic: calculation/mathematics) — Higher math: linear algebra,
geometry, calculus, numerical methods, spatial structures, Lie groups,
differential geometry, symbolic algebra.

- **Type**: Cyrius library + CLI (math toolkit)
- **License**: GPL-3.0-only
- **Language**: Cyrius (sovereign systems language, compiled by cycc)
- **Toolchain**: Cyrius 6.5.16 (`cyrius.cyml: cyrius = "6.5.16"`)
- **Version**: SemVer, version file at `VERSION` (manifest pulls via `${file:VERSION}`)
- **Status**: 2.9.3 — compiles cleanly under cycc 6.5.16. Library source lives in `src/` (smoke `main.cyr` + 34 math modules, 21,498 lines); `lib/` is vendored stdlib + deps only. CLI smoke binary builds (220,064 B static x86_64 ELF); the **full 34-module distlib bundle** (817,332 B / 21,604 lines — 78.0% of cycc's 1 MB `input_buf`, 231,244 B of headroom) ships at `dist/hisab.cyr`, and since 2.9.2 the `dist/hisab.deps` leaf sidecar (15 stdlib leaves) is tracked beside it so a consumer's `cyrius deps` auto-resolves them instead of the consumer hand-declaring the list. Library validated via tests (**3376/3376** across 5 suites — hisab 416, foundation 349, modules 1646, edge_cases 233, abuse 732), 60 benchmarks, and `scripts/check-constants.sh` (155/155 verified, 1 skipped). `cyrius coverage` **588/591 functions (99%)** over 35/35 files; the 80% target set at the start of the 2.6.x audit was met in 2.7.2. **2.7.0–2.9.2 are the arc after the 2.6.12–2.6.15 audit-repair arc** — that earlier record lives in `CHANGELOG.md` and `docs/development/roadmap.md`, and is not restated here. 2.7.0 closed the 2026-08-04 re-audit (35 of its 41 findings, every fix mutation-proven) and found the constant gate *itself* had been silently skipping 35 of 145 declarations while printing "110/110". 2.8.0 rewrote `delaunay_2d` onto triangle adjacency after establishing the shipped code was **wrong, not just slow**: 150 points on a circle returned 1651 triangles where 148 is correct, 206 edges shared by three or more triangles, 9 input points dropped. 2.8.1 audited that tree (42 findings, 4 critical, 0 refuted — `docs/audit/2026-08-04-v2.8.0-full.md`); 2.8.3 integrated eight parallel repair tracks and made the narrowphase return the exact MTV. 2.9.0 added `tests/abuse.tcyr` — 732 assertions that surfaced 11 defects on public entry points — plus a finding-disposition column and a struct-layout contract; 2.9.1 cleared the deferred tier of four latent ghost in-circle predicate defects, one of them (`_col_dl_ic_g1`) wrong on ordinary CCW input. 2.9.2 is a toolchain release — pin 6.5.9 → 6.5.16, sakshi 2.4.8 → 2.4.10, **no library source change** (the `dist/hisab.cyr` diff is the version header alone) — whose content is **three gates that were green while checking nothing**: `scripts/bench-history.sh` recorded each benchmark's MAX rather than its average in every row it ever wrote (44 of 55 in the latest run), CI's version gate was an unanchored `grep -q "$VERSION" CHANGELOG.md` that matched `32,942,104 B` in an unrelated benchmark line, and `cyrius distlib`'s 6.5.14 bundle self-check rejects any bundle reading a stdlib global. **No performance claim is made in 2.9.2**; it establishes the first *correct* benchmark baseline (measured noise on the `avg` field: median 3.5% spread over three back-to-back runs of the same binary, worst 6.9%, 0 of 55 above 20%). 2.9.3 worked the open filings and found **the filings themselves were the thing needing audit**: two of six argued for repairs that measurement refuted — an exact determinant of *pre-differenced* operands fixes nothing (the information is gone before it runs), and dropping EPA's seed test produces wrong depths at exact tangency — and a third rested on a measurement that had only looked at one of the two entry points it was about. Both real defects it did fix were silent: `delaunay_2d` DROPPED input points on any set mixing scales, and `_col_dl_incircle` answered differently by vertex order. **The standing lesson from the whole arc**: every defect of consequence was cheap to find and invisible because nothing looked — 2.6.x's defects all sat in modules with zero or near-zero coverage, and 2.9.x's sat behind gates nobody had aimed at their own output. The 6.4.66 bump (2.6.9) renamed `modules.tcyr`'s interval result vars `iv_add`/`iv_sub`/`iv_mul` → `iv_sum`/`iv_diff`/`iv_prod` because the originals collide with reserved cycc SIMD intrinsic names. **That reservation is by design and still stands on 6.5.16 — the rename is load-bearing and must not be reverted**; what was a defect was the misleading `expected identifier, got unknown` diagnostic, and it is fixed (all 67 reserved names emit the new wording on 6.5.16; the issue closed and archived 2026-08-09). See `docs/development/issues/archived/2026-07-17-cyrius-interval-ident-lex.md`.

## Consumers

impetus (physics), kiran (engine), joshua (simulation), aethersafha (compositor)

## Quick Start

```bash
cyrius deps                              # resolve GIT deps into lib/ (sakshi)
cyrius lib sync                          # vendor the declared stdlib subset into lib/
cyrius build src/main.cyr build/hisab    # build CLI
cyrius test tests/hisab.tcyr             # run a test suite
cyrius bench tests/hisab.bcyr            # run benchmarks
cyrius fuzz                              # run fuzz harnesses (walks tests/, needs >= 6.5.6)
```

## Dependencies

- **Cyrius stdlib** — `syscalls`, `string`, `alloc`, `str`, `fmt`, `vec`,
  `io`, `args`, `assert`, `math`, `ganita`, `tagged`, `fnptr`,
  `bench`, `callback` (ships with Cyrius >= 6.2.11). `ganita` is the 6.2.x
  math umbrella — transcendentals (acos/asin/atan2/pow/sinh/cosh/tanh +
  inverses) plus the full `matrix`/`linalg` API; do **not** also list `matrix`
  or `linalg` (duplicate-definition collisions). `math` stays for the inclusive
  comparisons, clamp/lerp/min/max/sign and the exp/ln polyfills.
- **sakshi** 2.4.10 — structured logging (first-party)

No external deps. No FFI. No libc. All first-party, pinned in
`cyrius.cyml` and SHA-locked in `cyrius.lock`.

## Layout

```
src/main.cyr         — CLI smoke binary (prints version, exits — does NOT
                       include the library; library coverage is in tests/)
src/*.cyr            — the 34 math modules (the library source). Self-contained
                       (no `include` lines); stdlib resolves via [deps] stdlib
lib/                 — vendored stdlib + first-party deps ONLY — no project
                       source here. Since the 6.3.x CLI split, `cyrius deps`
                       resolves GIT deps (sakshi) and `cyrius lib sync` vendors
                       the declared stdlib subset; byte-check the result against
                       the PIN's own snapshot, never old-pin vs new-pin
dist/hisab.cyr       — full 34-module distlib bundle (805,479 B / 21,368 lines),
                       regenerated via `cyrius distlib`. Consumers pull this
                       single file via [deps.hisab] modules = ["dist/hisab.cyr"]
dist/hisab.deps      — leaf sidecar naming the 15 stdlib leaves the bundle
                       needs in scope; written by `cyrius distlib`, read by a
                       consumer's `cyrius deps`. TRACKED since 2.9.2
examples/            — small demos (basic_math.cyr)
tests/
  hisab.tcyr         — primary assertion suite
  foundation.tcyr    — vec/quat/mat foundations
  modules.tcyr       — per-module coverage
  edge_cases.tcyr    — degenerate inputs, boundary values
  abuse.tcyr         — negative indices, zero/huge dims, non-conformable
                       operands, the designed-0 return, canary checks
  hisab.bcyr         — benchmark harness
  hisab.fcyr         — fuzz harness
docs/
  architecture/      — module map, math reference
  development/       — roadmap, threat model, dep watch, port audit, issues
  guides/            — testing
  audit/             — dated audit reports
scripts/
  bench-history.sh   — run tests/hisab.bcyr, append CSV rows, rebuild
                       benchmarks.md
  check-constants.sh — decode + verify every hand-encoded hex f64 in src/
  check-measurements.sh — every measurement-shaped claim in a comment carries
                       a [measured: …] provenance marker
  measurement-baseline.txt — per-file unmarked-claim counts (ratchet floor)
  version-bump.sh    — bump VERSION + CHANGELOG header + src/main.cyr's
                       printed version string
benchmarks.md        — rendered benchmark trend table (repo root; generated)
bench-history.csv    — benchmark regression history (repo root)
cyrius.cyml          — package manifest (toolchain pin, [deps])
cyrius.lock          — SHA256 lockfile (after first `cyrius deps`)
VERSION              — single source of truth for version
```

## Development Process

### P(-1): Scaffold Hardening (before any new features)

0. Read roadmap, CHANGELOG, and open issues — know what was intended before auditing what was built
1. Cleanliness sweep: `cyrius lint <file>`, `cyrius fmt <file> --check`, `cyrius vet src/main.cyr`
2. Test + benchmark sweep of existing code: `cyrius test tests/*.tcyr`, fuzz harnesses pass
3. Baseline benchmarks: `./scripts/bench-history.sh`
4. Internal deep review — gaps, optimizations, security, logging/errors, docs
5. External research — domain completeness, missing capabilities, best practices, world-class accuracy
6. Cleanliness re-check — must be clean after review
7. Additional tests/benchmarks from findings
8. Post-review benchmarks — prove the wins
9. Repeat if heavy

### Work Loop (continuous)

1. Work phase — new features, roadmap items, bug fixes
2. Cleanliness check: `cyrius lint <file>`, `cyrius fmt <file> --check`, `cyrius vet src/main.cyr`
3. Test + benchmark additions for new code
4. Run benchmarks: `./scripts/bench-history.sh`
5. Internal review — performance, memory, security, throughput, correctness
6. Cleanliness re-check — must be clean after audit
7. Deeper tests/benchmarks from audit observations
8. Run benchmarks again — prove the wins
9. If audit heavy → return to step 5
10. Documentation — update CHANGELOG, roadmap, docs
11. Version check — VERSION matches CHANGELOG header (cyrius.cyml auto-syncs via `${file:VERSION}`)
12. Return to step 1

### Task Sizing

- **Low/Medium effort**: batch freely — multiple items per work loop cycle
- **Large effort**: small bites only — break into sub-tasks, verify each before moving to the next
- **If unsure**: treat it as large

### Refactoring

- Refactor when the code tells you to — duplication, unclear boundaries, performance bottlenecks
- Never refactor speculatively. Wait for the third instance before extracting an abstraction
- Refactoring is part of the work loop, not a separate phase. If a review reveals structural issues, refactor before moving on
- Every refactor passes the same cleanliness + benchmark gates as new code

## Key Principles

- **Numbers don't lie.** Never claim a performance improvement without before/after benchmark numbers. The CSV history is the proof
- **Tests + benchmarks are the way.** Aim for 80%+ coverage
- **Own the stack.** First-party deps only — sakshi for logging, cyrius stdlib for everything else
- **No magic.** Every operation is measurable, auditable, traceable
- **Direct syscalls** — no libc wrappers
- **Manual struct layout** when needed — `alloc()` + `load64`/`store64` with named offset constants
- **Bump allocator** for long-lived data; **freelist** for individually-lifecycled data
- **str_builder** for formatting — avoid temporary allocations
- **Enums for constants** — zero `gvar_toks` cost vs. `var` globals
- **Source files only need project includes** — stdlib + first-party deps auto-resolve from `cyrius.cyml`
- **Every buffer is a contract**: `var buf[N]` = N bytes
- **Programs call `main()` at top level**: `var exit_code = main(); sys_exit_group(exit_code);`
  — `sys_exit_group`, never `syscall(60, …)`/`syscall(SYS_EXIT, …)`: SYS_EXIT is exit(2) and
  ends only the *calling thread*, so a program that ever spawns one hangs. Test harnesses must
  also **clamp** first (`if (r > 0) { r = 1; }`) — `assert_summary()` returns the raw failure
  count and a wait status is 8 bits, so exactly 256/512/768 failures would otherwise exit 0
  and score PASS
- **`cyrius build` handles everything** — never shell out to `cycc` directly

## CI / Release

- **Toolchain pin**: `cyrius = "6.5.16"` in `cyrius.cyml`. CI and release both grep
  the manifest; no hardcoded versions in YAML
- **Tag filter**: release triggers on `tags: ['v?[0-9]+.[0-9]+.[0-9]+']` (with or without `v` prefix)
- **Version-verify gate**: release asserts `VERSION == git tag` before building
  (cyrius.cyml auto-syncs via `${file:VERSION}`). CI's docs job additionally asserts the
  manifest still pulls `${file:VERSION}`, that `CHANGELOG.md` carries a literal
  `## [$VERSION]` header — `grep -qF --`, anchored, after the old unanchored
  `grep -q "$VERSION" CHANGELOG.md` matched `32,942,104 B` in an unrelated line and
  passed vacuously — plus the two version sites `${file:VERSION}` cannot reach:
  `src/main.cyr`'s printed string and `dist/hisab.cyr`'s bundle header
- **Lint gate**: `cyrius lint <file>` per source — warnings are errors. Since 6.2.11 lint
  exits 0 even with warnings, so the `^  warn ` grep over stdout is the load-bearing half
- **Fmt gate**: `cyrius fmt <file> --check` per source — drift fails the build.
  The flag goes AFTER the path; `cyrius fmt --check <file>` is a usage error. Check mode
  signals via exit code and writes nothing to stdout — never diff its stdout against the file
- **Vet gate**: `cyrius vet src/main.cyr`
- **Constant gate**: `./scripts/check-constants.sh` — decodes every hand-encoded hex f64 literal
  in `src/` and asserts it against the value in its own comment (153/153 verified, 1 skipped).
  Added 2.6.12 after seven mis-transcribed constant tables shipped; run it after touching ANY
  constant. Its own regex rejected `_` digit separators until the 2026-08-04 re-audit, silently
  skipping 35 of 145 declarations while reporting "110/110"
- **Measurement gate**: `./scripts/check-measurements.sh --diff origin/<base>` — pull requests
  only — every measurement-shaped claim a branch ADDS to a comment must carry a
  `[measured: …]` provenance marker, plus `--selftest` for detector recall. It cannot check the
  number, only that a later reader can re-derive it; a full-tree scan is pre-existing debt
  ratcheted by `scripts/measurement-baseline.txt`
- **Lock gate**: `cyrius deps --verify` against committed `cyrius.lock` (30/30)
- **Distlib gate**: `cyrius distlib` regenerates `dist/hisab.cyr` **and** `dist/hisab.deps`;
  CI fails on drift in either (run `cyrius distlib` locally before committing changes to the
  `src/*.cyr` modules named in `[lib]`, to `[lib]` itself, or on ANY version bump — the bundle
  header is stamped from `VERSION`). From 6.5.14 `distlib` runs a bundle self-check that
  compiles the bundle ALONE under `_cc_allow_undef`, which downgrades undefined *functions* but
  not undefined global variables — hisab's bundle legitimately reads `F64_ONE`, so it exits 1 on
  a good bundle. The bundle is still written first, so drift detection is unaffected; CI tolerates
  exactly that signature and nothing else. See
  `docs/development/issues/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md`
- **Consumer-build gate**: `cyrius check --with-deps dist/hisab.cyr` — compiles the bundle with
  the manifest's stdlib in scope, i.e. the way a consumer builds it. Strictly stronger than the
  broken self-check above
- **Build/ELF gates**: `cyrius build src/main.cyr build/hisab`, then the magic bytes are checked
- **Test/Fuzz/Bench gates**: every `tests/*.tcyr`, `tests/*.fcyr`, `tests/*.bcyr` runs
- **Docs + security jobs**: required root files present, version consistency (above), and a scan
  for dangerous patterns
- **Concurrency**: CI uses `cancel-in-progress: true` keyed on workflow + ref

## DO NOT

- **Do not commit or push** — the user handles all git operations (commit, push, tag)
- **NEVER use `gh` CLI** — use `curl` to GitHub API only
- Do not add external dependencies — first-party only (sakshi + cyrius stdlib)
- Do not skip benchmarks before claiming performance improvements
- Do not skip fuzz verification before claiming a parser works
- Do not hardcode toolchain versions in CI YAML — read `cyrius.cyml`
- Do not commit `build/` — it's regenerated per-build
- Do not re-vendor stdlib or first-party deps into `src/` — `cyrius deps` (git deps) and
  `cyrius lib sync` (declared stdlib subset) manage `lib/`. Sync the declared subset, never a
  full snapshot — that over-vendors unused platform variants and breaks `deps --verify`
- Do not shell out to `cycc` directly — always go through `cyrius <subcommand>`
- Do not use `sys_system()` with unsanitized input — command injection risk

## Documentation Structure

```
Root files (required):
  README.md          — quick start, features, dependency stack, consumers, license
  CHANGELOG.md       — per-version changes (Added/Changed/Fixed/Removed)
  CLAUDE.md          — this file (development process, principles, DO NOTs)
  CONTRIBUTING.md    — fork, branch, cleanliness, PR workflow
  SECURITY.md        — supported versions, scope, reporting
  CODE_OF_CONDUCT.md — Contributor Covenant
  LICENSE            — GPL-3.0-only
  VERSION            — current semver
  cyrius.cyml        — package manifest (toolchain pin, deps, build)
  benchmarks.md      — rendered benchmark trend table (regenerated by
                       scripts/bench-history.sh; NOT hand-edited)
  bench-history.csv  — benchmark regression history, one row per run per
                       benchmark. The `stat` column names which statistic
                       `estimate_ns` holds; rows written before 2.9.2 have no
                       value there and are `max`

docs/ (required):
  doc-health.md      — living doc-currency ledger (fresh/stale/read-through/dated
                       per file); refresh the affected row whenever a doc is touched
  architecture/
    overview.md      — module map, data flow, consumers, dependency stack
    math.md          — (when applicable) mathematical reference for algorithms/formulas
  development/
    roadmap.md       — completed items, backlog, future features (demand-gated), v1.0 criteria

docs/ (when earned — not scaffolded empty):
  adr/
    NNN-title.md     — architectural decision records (when non-obvious choices are made)
  development/
    threat-model.md  — attack surface, mitigations (when security-relevant)
    dependency-watch.md — deps to monitor for updates / CVEs
    port-audit.md    — Rust-era → Cyrius parity audit
    issues/          — one file per real bug (in-tree or upstream toolchain);
                       shipped ones move to issues/archived/
  guides/
    usage.md         — patterns, philosophy, code examples (NOT YET EARNED —
                       only testing.md exists; see doc-health.md)
    testing.md       — test count, coverage, testing patterns
  audit/
    YYYY-MM-DD.md    — dated audit reports (security, perf, correctness)
```

Benchmarks live at the repo root (`benchmarks.md`, `bench-history.csv`), not under
`docs/` — there is no `docs/benchmarks/`. Do not create a second benchmark surface.

ADR format:

```
# NNN — Title
## Status: Accepted/Superseded
## Context: Why this decision was needed
## Decision: What we chose
## Consequences: Trade-offs, what changes
```

## CHANGELOG Format

Follow [Keep a Changelog](https://keepachangelog.com/):

```markdown
# Changelog

## [Unreleased]
### Added — new features
### Changed — changes to existing features
### Fixed — bug fixes
### Removed — removed features
### Security — vulnerability fixes
### Performance — benchmark-proven improvements (include numbers)

## [X.Y.Z] - YYYY-MM-DD
### Added
- **module_name** — what was added and why
### Changed
- item: old behavior → new behavior
### Fixed
- issue description (root cause → fix)
### Performance
- benchmark_name: before → after (−XX%)
```

Rules:
- Every PR/commit that changes behavior gets a CHANGELOG entry
- Performance claims MUST include benchmark numbers
- Breaking changes get a **Breaking** section with migration guide
- Group by module when multiple changes in one release
- Link to ADR if a change was driven by an architectural decision
