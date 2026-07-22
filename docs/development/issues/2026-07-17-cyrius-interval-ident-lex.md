# 2026-07-17 — cyrius: `iv_add`/`iv_sub`/`iv_mul` are reserved SIMD intrinsics, unusable as var names

**Component:** `cyrius` lexer/parser (cycc `src/frontend/lex.cyr` ~L991-997, `parse.cyr`)
**Toolchain seen:** cyrius 6.4.69 — reproduces on **6.3.11, 6.4.5, 6.4.6, 6.4.65, 6.4.66, 6.4.69**
(every version tested; re-verified 2026-07-21 on the 6.4.69 pin: `var iv_add = 1;` →
`expected identifier, got unknown`. The reserved name predates the toolchain bump — not a 6.4.x regression).
**Severity:** Medium — hard compile failure on this suite with a misleading diagnostic; a
one-line rename works around it.
**Hisab impact:** `tests/modules.tcyr` — the interval-arithmetic section named its result vars
`iv_add` / `iv_sub` / `iv_mul`. Those are reserved intrinsic tokens, so `var iv_add = ...`
fails to compile (`expected identifier, got unknown`), taking the whole 312-assertion suite
(and thus the 957-assertion run) down. **This is name-based, not interval-specific** — the
interval section is simply where hisab happened to use the names.
**Hisab workaround (shipped in 2.6.9):** renamed to `iv_sum` / `iv_diff` / `iv_prod`. Suite
compiles deterministically (0/8 fail) and runs 312/312. A `NOTE:` comment at
`tests/modules.tcyr` `test_group("interval: arithmetic")` guards the names.
**Upstream:** filed at `cyrius/docs/development/issues/2026-07-17-iv-simd-intrinsic-shadows-var-name.md`
(+ repro `.../repros/2026-07-17-iv-intrinsic-var-name.cyr`).
**Status:** Open (toolchain bug) — worked around in-tree.

## Symptom

Declaring a variable named `iv_add`, `iv_sub`, or `iv_mul` fails at compile time with
`error: expected identifier, got unknown` pointing at the (clean) name, then the parser
desyncs — following lines cascade into `expected '(', got ')'` and `undefined variable`. In a
large file the real line is buried; only bisection (or knowing the cause) finds it.
Deterministic: 8/8 fail with these names, 8/8 pass renamed. `iv_div`, `iv_neg`, `iv_abs` are
**not** affected.

## Root cause

`iv_add` / `iv_sub` / `iv_mul` (and `iv_dp8`) are **reserved SIMD intrinsics** — the lexer
tokenizes them unconditionally as packed integer-vector ops (cycc `lib/simd.cyr` calls them as
builtins for `i8v16`/`i16v8`/`i32v4`/`i64v2` add/sub/mul; there is no `fn iv_add` — it's a
compiler token, `lex.cyr` L993 `ADDTOK(S, 143, 0)`). In `var <name>` position the parser wants
an identifier, gets the intrinsic token, and falls through to the catch-all
`expected identifier, got unknown`. `iv_div` is safe because there is no integer-vector divide
instruction, so it was never reserved — which is exactly why the failure looks name-arbitrary.
**Not** a hash collision or a size/buffer cap (earlier speculation): the stat tables have huge
headroom (`CYRIUS_STATS`: identifiers 28303/262144), and a two-line program reproduces it.

## Self-contained reproducer

```cyrius
include "lib/syscalls.cyr"
var iv_add = 1;          # -> error: expected identifier, got unknown
```

`cyrius test <file>` → fails 8/8. Rename `iv_add`→`iv_sum` (etc.) → passes.

## Suggested upstream fix

Minimum: when the parser expects an identifier and sees an `iv_*` intrinsic token, emit a
clear "`iv_add` is a reserved SIMD intrinsic" diagnostic and recover without desyncing.
Better: make the intrinsics *contextual* keywords (keyword only before `(`), so the names are
usable as identifiers everywhere else.
