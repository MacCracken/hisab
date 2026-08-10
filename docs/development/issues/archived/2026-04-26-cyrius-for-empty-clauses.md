# 2026-04-26 — cyrius parser: `for (init; cond; step)` requires all three clauses non-empty

> **CLOSED 2026-08-09 (hisab v2.9.3 triage) — WON'T-FIX UPSTREAM, BY DESIGN.**
> The upstream filing was resolved on **2026-06-02 (cycc v6.0.36)** and archived at
> `cyrius/docs/development/issues/archived/2026-04-26-cyrius-for-empty-clauses.md`:
>
> > Cyrius requires all three `for` clauses; omitted clauses (`for (;;)`) are not supported by
> > design. Use `while` for unbounded/custom-stepped loops and the counted `for` / `for x in …`
> > forms otherwise. The convention is now documented in `docs/guides/cyrius-guide.md`
> > (control-flow section).
>
> **hisab carried this as "Open" for five releases after it had been decided.** The re-verifications
> below (6.3.11, 6.4.66, 6.4.69, 6.5.6, 6.5.9, 6.5.16) were all re-confirming a documented language
> rule, not tracking an unfixed bug — nobody had checked the upstream tracker. The one thing those
> passes did establish that is still worth having is that **the misleading-line-number half of the
> complaint was genuinely fixed**: on 6.5.16 the error lands on the true `file:line` with a caret on
> the offending token and both forms report in the same run, against the original symptom of an
> error appearing inside an unrelated function hundreds of lines away.
>
> **Consequence for hisab: permanent, not provisional.** The `while` rewrites in
> `src/collision_core.cyr` and `src/collision_mesh.cyr` are the correct idiom and must not be
> "restored" to the C-style form by a future reader who assumes this is still pending.

**Component:** `cyrius` parser (`src/frontend/parse_ctrl.cyr` `PARSE_FOR`)
**Toolchain seen:** cyrius 5.7.10 (parser shape unchanged for several minor releases)
**Severity:** Misleading parse errors at the wrong line — easy to chase the wrong file
**Hisab impact:** `lib/collision_core.cyr` (3 sites) and `lib/collision_mesh.cyr` (1 site) had loops in this shape from the original 2.2.0 port. Files were never in the build chain pre-2.2.2, so the syntax errors were dormant.
**Hisab workaround:** Converted to `while`. See `lib/collision_core.cyr` lines ~376, 404, 498 and `lib/collision_mesh.cyr` line ~204 (current state) and the CHANGELOG 2.2.2 "Fixed" entry.
**Status:** Open — possibly intentional (Cyrius prefers `for ident in start..end` / `for ident in collection`).
**Re-verified 2026-08-09 on cycc 6.5.16 (v2.9.2 bump): STILL LIVE — but HALF THE COMPLAINT IS NOW
FIXED.** Both forms still reject, unchanged in substance: `for (; i < 3; i = i + 1)` →
`unexpected ';'`, `for (var j = 0; j < 3;)` → `expected '=', got '{'`. The `while` rewrites in
`src/collision_core.cyr` / `src/collision_mesh.cyr` stay.

What HAS changed is the thing the Symptom section below actually complains about. The errors no
longer land "at a line further down the file, often inside an unrelated function body" and there is
no bisection to do — they land on the true `file:line` with a caret on the offending token, and
**both forms are reported in the same run** rather than one masking the other:

```
error:srcmod/formod.cyr:5:10: unexpected ';'
        for (; i < 3; i = i + 1) {
             ^
error:srcmod/formod.cyr:12:29: expected '=', got '{'
        for (var j = 0; j < 3;) {
                                ^
```

⚠ Reproduce this in an **`include`d module**, not in the file named on the command line. Line
numbers for the top-level source are reported against the concatenated translation unit and are
offset by (stdlib dep count + 1); errors inside included files carry the true filename and line.
Mistaking that offset for the wrong-line bug this filing describes is an easy error to make.

The remaining complaint is therefore only the rejection itself, which may well be intentional.

**Re-verified 2026-08-06 on cycc 6.5.9: STILL LIVE.** `for (; i < 3; i = i + 1) {}` reports
`error: unexpected ';'`. Carried forward unchanged.
**Re-verified:** 2026-08-03 on pinned **6.5.6** (v2.6.11 bump) — empty-init form still rejects: `for (; cond; step)` → `unexpected ';'`; unchanged across the 6.4 → 6.5 minor. Prior: 2026-07-21 on 6.4.69 (v2.6.10 bump — `for (; cond; step)` → `unexpected ';'`); 2026-07-17 on 6.4.66 (v2.6.9 — both forms: `for (; cond; step)` → `unexpected ';'`; `for (init; cond;)` → `expected '=', got '{'`); 2026-06-30 on 6.3.11; 2026-06-15 on 6.2.11 (all live).

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
