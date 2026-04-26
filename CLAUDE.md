# Hisab — Claude Code Instructions

## Project Identity

**Hisab** (Arabic: calculation/mathematics) — Higher math: linear algebra,
geometry, calculus, numerical methods, spatial structures, Lie groups,
differential geometry, symbolic algebra.

- **Type**: Cyrius library + CLI (math toolkit)
- **License**: GPL-3.0-only
- **Language**: Cyrius (sovereign systems language, compiled by cc5)
- **Toolchain**: Cyrius 5.7.8 (`cyrius.cyml: cyrius = "5.7.8"`)
- **Version**: SemVer, version file at `VERSION` (manifest pulls via `${file:VERSION}`)
- **Status**: 2.2.2 — actually compiles under cc5 5.7.8 (CLI smoke binary builds; library validated via tests). Distlib bundle deferred until upstream cc5 5.7.9 raises input_buf to 1 MB.

## Consumers

impetus (physics), kiran (engine), joshua (simulation), aethersafha (compositor)

## Quick Start

```bash
cyrius deps                              # resolve deps into lib/
cyrius build src/main.cyr build/hisab    # build CLI
cyrius test tests/hisab.tcyr             # run a test suite
cyrius bench tests/hisab.bcyr            # run benchmarks
```

## Dependencies

- **Cyrius stdlib** — `syscalls`, `string`, `alloc`, `str`, `fmt`, `vec`,
  `io`, `args`, `assert`, `math`, `matrix`, `linalg`, `tagged`, `fnptr`,
  `bench`, `callback` (ships with Cyrius >= 5.7.8)
- **sakshi** 2.1.0 — structured logging (first-party)

No external deps. No FFI. No libc. All first-party, pinned in
`cyrius.cyml` and SHA-locked in `cyrius.lock`.

## Layout

```
src/main.cyr         — CLI smoke binary (prints version, exits — does NOT
                       include the library; library coverage is in tests/)
lib/                 — vendored deps (managed by `cyrius deps`) + math modules
                       (consumers pull project modules directly via
                       [deps.hisab] modules = ["lib/<file>.cyr", ...])
examples/            — small demos (basic_math.cyr)
tests/
  hisab.tcyr         — primary assertion suite
  foundation.tcyr    — vec/quat/mat foundations
  modules.tcyr       — per-module coverage
  edge_cases.tcyr    — degenerate inputs, boundary values
  hisab.bcyr         — benchmark harness
  hisab.fcyr         — fuzz harness
docs/
  architecture/      — module map, math reference
  development/       — roadmap, threat model, dep watch, port audit
  guides/            — usage, testing
  audit/             — dated audit reports
scripts/
  bench-history.sh   — append benchmark CSV row
  version-bump.sh    — bump VERSION + CHANGELOG header
cyrius.cyml          — package manifest (toolchain pin, [deps])
cyrius.lock          — SHA256 lockfile (after first `cyrius deps`)
VERSION              — single source of truth for version
```

## Development Process

### P(-1): Scaffold Hardening (before any new features)

0. Read roadmap, CHANGELOG, and open issues — know what was intended before auditing what was built
1. Cleanliness sweep: `cyrius lint`, `cyrius fmt --check`, `cyrius vet src/main.cyr`
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
2. Cleanliness check: `cyrius lint`, `cyrius fmt --check`, `cyrius vet src/main.cyr`
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
- **Programs call `main()` at top level**: `var exit_code = main(); syscall(60, exit_code);`
- **`cyrius build` handles everything** — never shell out to `cc5` directly

## CI / Release

- **Toolchain pin**: `cyrius = "5.7.8"` in `cyrius.cyml`. CI and release both grep
  the manifest; no hardcoded versions in YAML
- **Tag filter**: release triggers on `tags: ['v?[0-9]+.[0-9]+.[0-9]+']` (with or without `v` prefix)
- **Version-verify gate**: release asserts `VERSION == git tag` before building
  (cyrius.cyml auto-syncs via `${file:VERSION}`)
- **Lint gate**: `cyrius lint` per source — warnings are errors
- **Fmt gate**: `cyrius fmt --check` per source — drift fails the build
- **Vet gate**: `cyrius vet src/main.cyr`
- **Lock gate**: `cyrius deps --verify` against committed `cyrius.lock` (when present)
- **Test/Fuzz/Bench gates**: every `tests/*.tcyr`, `tests/*.fcyr`, `tests/*.bcyr` runs
- **Distlib**: deferred — `[lib] modules = []` is intentional for 2.2.2.
  The flat 33-file bundle is 544 KB and exceeds cc5 5.7.8's 512 KB
  input_buf. cc5 5.7.9 raises input_buf to 1 MB (per the 5.7.8 release
  header) — restore the full `[lib]` list (see git history pre-2.2.2)
  when we bump the toolchain pin to 5.7.9.
- **Concurrency**: CI uses `cancel-in-progress: true` keyed on workflow + ref

## DO NOT

- **Do not commit or push** — the user handles all git operations (commit, push, tag)
- **NEVER use `gh` CLI** — use `curl` to GitHub API only
- Do not add external dependencies — first-party only (sakshi + cyrius stdlib)
- Do not skip benchmarks before claiming performance improvements
- Do not skip fuzz verification before claiming a parser works
- Do not hardcode toolchain versions in CI YAML — read `cyrius.cyml`
- Do not commit `build/` — it's regenerated per-build
- Do not re-vendor stdlib or first-party deps into `src/` — `cyrius deps` manages `lib/`
- Do not shell out to `cc5` directly — always go through `cyrius <subcommand>`
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

docs/ (required):
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
  guides/
    usage.md         — patterns, philosophy, code examples
    testing.md       — test count, coverage, testing patterns
  audit/
    YYYY-MM-DD.md    — dated audit reports (security, perf, correctness)
  benchmarks/
    results.md       — latest numbers
    history.csv      — regression baseline
```

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
