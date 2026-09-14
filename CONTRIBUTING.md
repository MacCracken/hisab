# Contributing to Hisab

Thank you for your interest in contributing to Hisab.

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run the check suite (see below)
5. Open a pull request

## Prerequisites

- [Cyrius](https://github.com/MacCracken/cyrius), at whatever version `cyrius.cyml [package].cyrius` pins — 6.6.4 as of 3.1.1. CI greps the manifest rather than carrying a literal; match the manifest and don't hardcode a version elsewhere
- The build tool resolves stdlib + first-party deps automatically via `cyrius.cyml` (run `cyrius deps`)

## Checking Your Work

```bash
# Build
cyrius build src/main.cyr build/hisab

# Run all five suites (4429 assertions). CI runs every tests/*.tcyr — running
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

# The public surface is declared, not implied (3.1.0): every non-underscore top-level
# declaration carries `public`, and this flips every module `private` in a scratch copy
# to prove the surface complete and exact. Adding a public fn without the keyword fails
# claim 0; a cross-module `_` helper without the marker comment fails claim 1.
./scripts/check-public-surface.sh

# No Result-returning call may sit in ARGUMENT position (it degrades to its tag, silently)
./scripts/check-result-migration.sh

# Regenerate the distlib bundle (CI fails on drift; required after any src/ or [lib] change,
# and on ANY version bump — the bundle header is stamped from VERSION).
# A non-zero exit here is a real failure. ⚠ It was not always: cyrius 6.5.14–6.5.16 ran a bundle
# self-check that compiled the bundle ALONE, which rejected any bundle reading a stdlib global
# (hisab's reads F64_ONE), so both workflows tolerated exactly that signature for three toolchain
# releases. Fixed upstream in 6.5.17; the tolerance is gone. See
# docs/development/issues/archived/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md
cyrius distlib

# Compile the bundle with the stdlib in scope, the way a consumer actually builds it —
# strictly stronger than the self-check ever was, and kept independently of it
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
- Heap-allocate structs through their declared layout: `var v = alloc(sizeof(T)); T_set_x(v, x); return v;` — never a hardcoded byte count or a hand-computed offset for a type that has a `struct` (every public struct's `sizeof` is pinned exactly in `tests/abuse.tcyr` since 3.2.0). Manual `alloc(N)` + `store64` layouts are for types with no struct declaration only
- Use `#derive(accessors)` for struct field access. ⚠ In a function that also expands `f64v_*` intrinsics, never read a derived getter of an object INSIDE the value argument of a derived setter on that same object — cycc 6.6.0–6.6.4 miscompiles that shape (open filing; see `m3_mul_vec3`)
- Prefix private helpers with underscore: `fn _my_helper()`. Public API carries `public` (`public fn`, `public struct`, `public var`, `public enum`); a `_` helper another module reaches also carries `public` plus the `# public: cross-module helper reached from …` marker comment, and owes a 4.0.0 disposition on the roadmap
- Comment f64 hex constants with their decimal value
- Use `elif` not `else if`
- No negative literals: use `(0 - N)` or `f64_neg(x)`
- Fallible functions return `Result<T, E>`: `return Ok(0);` on success, `return Err(HSB_ERR_*);` on failure, with `#must_use` on the declaration. Callers bind both halves — `var t, v = f(...)` — and test `is_err_result(t)`; never pass such a call straight into another function's argument list (it compiles and hands over the tag)
- Values via out-parameter pointers: `store64(out, result_value)`

## Testing

- Smoke tests in `tests/hisab.tcyr` (integration across modules)
- Foundation tests in `tests/foundation.tcyr` (Vec/Quat/Mat exhaustive)
- Module tests in `tests/modules.tcyr` (per-module coverage)
- Edge cases in `tests/edge_cases.tcyr` (boundary conditions, error paths)
- Abuse tests in `tests/abuse.tcyr` (hostile input: negative indices and counts, zero/huge dimensions, non-conformable operands, the designed-0 handle every capped constructor returns, degenerate geometry, and canary blocks allocated after an out-buffer so a write past the end fails a test instead of passing by luck). Not about numerical answers — those belong in the suites above
- Define all helper functions **before** `alloc_init()` to avoid compiler issues
- Compare floats bit-exactly (`assert_eq(x, f64_from(3), …)`) unless you can say why not; a relative comparison must reject NaN. Never through `f64_to`, which truncates
- Every new assertion is shown to discriminate: install a one-token mutant, watch it fail, restore, prove the restore
- Target: test every public function with at least one happy-path and one edge case — and the property, not a degenerate fixture (the identity, radius 1, `a = 0`) that a wrong implementation also satisfies

## Commits

- Use conventional-style messages
- One logical change per commit

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
