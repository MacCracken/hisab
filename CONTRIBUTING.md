# Contributing to Hisab

Thank you for your interest in contributing to Hisab.

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run the check suite (see below)
5. Open a pull request

## Prerequisites

- [Cyrius](https://github.com/MacCracken/cyrius), at whatever version `cyrius.cyml [package].cyrius` pins — 6.5.18 as of 2.10.0. CI greps the manifest rather than carrying a literal; match the manifest and don't hardcode a version elsewhere
- The build tool resolves stdlib + first-party deps automatically via `cyrius.cyml` (run `cyrius deps`)

## Checking Your Work

```bash
# Build
cyrius build src/main.cyr build/hisab

# Run all five suites (3507 assertions). CI runs every tests/*.tcyr — running
# fewer than five locally skips a whole surface, not a handful of cases.
cyrius test tests/hisab.tcyr
cyrius test tests/foundation.tcyr
cyrius test tests/modules.tcyr
cyrius test tests/edge_cases.tcyr
cyrius test tests/abuse.tcyr

# Run benchmarks
cyrius bench tests/hisab.bcyr

# Run fuzz self-test (cyrius >= 6.5.6 walks tests/; older toolchains need the manual build)
cyrius fuzz

# Lint + format check (CI runs these per file across src/ AND tests/; warnings are errors)
cyrius lint src/main.cyr
cyrius fmt src/main.cyr --check

# Verify hand-encoded f64 constants against their own comments (CI gate since 2.6.12,
# after seven mis-transcribed constant tables shipped). Run after touching any constant.
./scripts/check-constants.sh

# Every measurement-shaped claim your branch ADDS to a comment must name where the
# number came from (`[measured: …]`). CI gates this on pull requests only, over the
# diff against the base branch — pass the branch you forked from.
./scripts/check-measurements.sh --diff origin/main

# Vet include dependencies
cyrius vet src/main.cyr

# Regenerate the distlib bundle (CI fails on drift; required after any src/ or [lib] change).
# ⚠ On cyrius >= 6.5.14 this exits 1 with "does not compile" + "undefined variable" even when
# the bundle is good: distlib's new self-check compiles the bundle ALONE, and `_cc_allow_undef`
# downgrades undefined FUNCTIONS but not undefined global VARIABLES — hisab's bundle reads
# stdlib constants (F64_ONE) a consumer supplies via `[deps].stdlib`. The bundle is written
# before that check runs, so regeneration and drift detection are unaffected; CI tolerates
# exactly that signature and nothing else. See
# docs/development/issues/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md
cyrius distlib

# What that self-check should have been: compile the bundle with the stdlib in scope, the
# way a consumer actually builds it
cyrius check --with-deps dist/hisab.cyr
```

## Adding a Module

Library source lives in `src/` (`lib/` is vendored stdlib + deps only — never add project source there).

1. Create `src/module_name.cyr` with a header comment (purpose, requires). Source files are self-contained — no `include` lines; stdlib + first-party deps resolve via `cyrius.cyml`.
2. Add it to the `[lib] modules` list in `cyrius.cyml` (this is what the distlib bundle pulls in)
3. Add tests to the appropriate `.tcyr` file or create a new one
4. Update the README module table, `docs/architecture/overview.md`, and `docs/doc-health.md`
5. Run `cyrius distlib` to regenerate `dist/hisab.cyr` **and** `dist/hisab.deps` (the stdlib-leaf sidecar a consumer's `cyrius deps` reads — tracked since 2.9.2), and commit both; CI checks each for drift

## Code Style

- All f64 values stored as IEEE 754 bit patterns (use `f64_from()` / `f64_to()`)
- Heap-allocate multi-field structs: `var v = alloc(N); store64(v, x); return v;`
- Use `#derive(accessors)` for struct field access
- Prefix private helpers with underscore: `fn _my_helper()`
- Comment f64 hex constants with their decimal value
- Use `elif` not `else if`
- No negative literals: use `(0 - N)` or `f64_neg(x)`
- Error codes: return `ERR_NONE` (0) on success, negative `ERR_*` on failure
- Results via out-parameter pointers: `store64(out, result_value)`

## Testing

- Smoke tests in `tests/hisab.tcyr` (integration across modules)
- Foundation tests in `tests/foundation.tcyr` (Vec/Quat/Mat exhaustive)
- Module tests in `tests/modules.tcyr` (per-module coverage)
- Edge cases in `tests/edge_cases.tcyr` (boundary conditions, error paths)
- Abuse tests in `tests/abuse.tcyr` (hostile input: negative indices and counts, zero/huge dimensions, non-conformable operands, the designed-0 handle every capped constructor returns, degenerate geometry, and canary blocks allocated after an out-buffer so a write past the end fails a test instead of passing by luck). Not about numerical answers — those belong in the suites above
- Define all helper functions **before** `alloc_init()` to avoid compiler issues
- Target: test every public function with at least one happy-path and one edge case

## Commits

- Use conventional-style messages
- One logical change per commit

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
