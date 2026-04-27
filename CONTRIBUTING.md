# Contributing to Hisab

Thank you for your interest in contributing to Hisab.

## Development Workflow

1. Fork and clone the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run the check suite (see below)
5. Open a pull request

## Prerequisites

- [Cyrius](https://github.com/MacCracken/cyrius) 5.7.10+ (`cyrius.cyml [package].cyrius` pins the exact version)
- The build tool resolves stdlib + first-party deps automatically via `cyrius.cyml` (run `cyrius deps`)

## Checking Your Work

```bash
# Build
cyrius build src/main.cyr build/hisab

# Run all tests
cyrius test tests/hisab.tcyr
cyrius test tests/foundation.tcyr
cyrius test tests/modules.tcyr
cyrius test tests/edge_cases.tcyr

# Run benchmarks
cyrius bench tests/hisab.bcyr

# Run fuzz self-test
cyrius build tests/hisab.fcyr build/hisab_fuzz && build/hisab_fuzz

# Lint
cyrius lint src/main.cyr

# Vet dependencies
cyrius vet src/main.cyr
```

## Adding a Module

1. Create `lib/module_name.cyr` with header comment (usage, requires)
2. Add `include "lib/module_name.cyr"` to `src/main.cyr`
3. Add tests to the appropriate `.tcyr` file or create a new one
4. Update README module table
5. Update `docs/architecture/overview.md`

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
- Define all helper functions **before** `alloc_init()` to avoid compiler issues
- Target: test every public function with at least one happy-path and one edge case

## Commits

- Use conventional-style messages
- One logical change per commit

## License

By contributing, you agree that your contributions will be licensed under GPL-3.0.
