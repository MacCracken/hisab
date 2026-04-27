# 2026-04-26 — cyrius parser: `for (init; cond; step)` requires all three clauses non-empty

**Component:** `cyrius` parser (`src/frontend/parse_ctrl.cyr` `PARSE_FOR`)
**Toolchain seen:** cyrius 5.7.10 (parser shape unchanged for several minor releases)
**Severity:** Misleading parse errors at the wrong line — easy to chase the wrong file
**Hisab impact:** `lib/collision_core.cyr` (3 sites) and `lib/collision_mesh.cyr` (1 site) had loops in this shape from the original 2.2.0 port. Files were never in the build chain pre-2.2.2, so the syntax errors were dormant.
**Hisab workaround:** Converted to `while`. See `lib/collision_core.cyr` lines ~376, 404, 498 and `lib/collision_mesh.cyr` line ~204 (current state) and the CHANGELOG 2.2.2 "Fixed" entry.
**Status:** Open — possibly intentional (Cyrius prefers `for ident in start..end` / `for ident in collection`).

## Symptom

Cyrius accepts the C-style `for (init; cond; step) { body }` form when **all three clauses are non-empty** (verified in `cyrius/tests/tcyr/core.tcyr:46`). When any clause is empty:

- `for (; cond; step)` (empty init) — parser reports `unexpected ';'` at a line *further down the file*, often inside an unrelated function body.
- `for (init; cond;)` (empty step) — parser reports `expected '=', got '{'` on the *next* `fn` declaration, even if that fn is hundreds of lines away.

The error line is misleading: bisecting the file is the only way to find the actual offending loop.

## Self-contained reproducer

```cyrius
include "lib/syscalls.cyr"
include "lib/alloc.cyr"

fn empty_init() {
    var i = 0;
    for (; i < 3; i = i + 1) {
        i = i + 1;
    }
    return 0;
}

fn empty_step() {
    for (var i = 0; i < 3;) {
        i = i + 1;
    }
    return 0;
}

fn main() {
    return 0;
}

var r = main();
syscall(60, r);
```

Each of `empty_init` and `empty_step` triggers a parse error individually (build fails on the first one, comment it out to see the second).

## Working forms (for comparison)

```cyrius
fn full_for() {
    for (var i = 0; i < 3; i = i + 1) { ... }   # OK — all three clauses present
    return 0;
}

fn cyrius_native_range() {
    for i in 0..3 { ... }                       # OK — preferred Cyrius form
    return 0;
}
```

## Where to look in cc5

`src/frontend/parse_ctrl.cyr` `PARSE_FOR(S)` (~L243). The fn dispatches on whether the first token after `for` is an identifier (ranged-for) or a `(` (C-style). The C-style branch presumably calls a sub-parser that requires three semicolon-separated clauses; treating the empty-clause case as "syntax error at the next valid token" rather than emitting "for clause N is empty".

## Verification once a fix lands

The reproducer above should compile cleanly OR error with a sensible "for clause must not be empty" message pointing at the actual loop line.

## Hisab follow-up after upstream fix

If the upstream decision is to **support empty clauses** in C-style `for`, no follow-up needed — hisab's `while` rewrites are equivalent and don't need to revert.

If the upstream decision is to **reject empty clauses with a clear error**, no follow-up needed — hisab is already on the working `while` form.

Either way, this file gets removed from `docs/development/issues/` once the error UX improves.
