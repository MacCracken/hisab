# 2026-09-11 — cyrius: `continue` in a nested loop binds one level too far out (silent wrong code)

**Component:** `cyrius` / `cycc` — loop lowering, the `continue` patch table.
**Toolchain seen:** cyrius **6.6.2**. Bisect not attempted at filing; upstream traced it to the
patch array's design, so it is older than any pin hisab has used.
**Severity:** **Critical** — wrong code with no diagnostic and no crash. The program runs to
completion and reports success with a wrong result.
**Hisab impact:** `_cga_build_null_tbl` in `src/geo_advanced.cyr` — the natural sparse-skip form
of its 32×32 pair loop produced an **all-zero 1024-entry table** while every helper was correct in
isolation. Found in 2.21.0 while building the CGA null basis; shipped since then with an `if`-guard
workaround and the FNV contract gate that caught it.

## Summary

When a loop and a loop nested inside it BOTH contain a `continue`, and the outer one appears
lexically FIRST, both bind one level too far out: the inner `continue` jumps to the OUTER loop's
latch (abandoning the inner loop AND the rest of the outer body) and the outer `continue` becomes a
no-op. Strictly lexical — moving the outer `continue` below the nested loop makes the same program
correct.

⚠ hisab's first filing described its own symptom wrongly ("the inner `continue` exits the OUTER
loop"). Instrumenting the loop refuted that: the outer body is entered all 3 times while the inner
runs 3 of 9, and the outer `continue` half was missing from the filing entirely. **A filing is a
measurement like any other.**

## Root cause (upstream)

`0x18F8A0` is ONE flat 8-entry patch array shared by every nesting level and `0x18F898` is its
next-free index. Every loop reset that index to 1 on entry, so a nested loop began writing at index 0
again and **overwrote the enclosing loop's recorded jump**. Fixed by starting each loop at the
enclosing loop's next-free index so the ranges are disjoint. Upstream found a second half not in the
filing: `PARSE_WHILE` never wrote the index, which doubles as a mode flag, so a `while` nested in a
`for` inherited for-mode and sent its `continue` to the **for's step**. ⚠ The `max 8` `continue`
limit is now total across a nest, not per loop.

## Reproduction

Filed with `cyrius/docs/development/issues/repros/2026-09-11-nested-continue-binds-to-wrong-loop.cyr`,
which **proves itself**: exit 0 when fixed, 1 while the bug is present, across ten loop shapes plus
three counters. Upstream's gate is `tests/tcyr/lang/nested_continue_binds.tcyr` (8 assertions).

**Upstream:** filed 2026-09-11 at
`cyrius/docs/development/issues/2026-09-11-nested-continue-binds-to-wrong-loop.md`; archived there
on the 6.6.3 release.

**Status:** 🟢 **FIXED in cyrius 6.6.3, verified 2026-09-13 on the 3.0.1 bump — as a pair, not
from the changelog.**

| probe | 6.6.2 | 6.6.3 |
|---|---|---|
| hisab's filed reproducer (same source) | exit **1** | exit **0** |
| upstream gate `nested_continue_binds.tcyr` | **2 passed, 6 failed** | 8 passed, 0 failed |
| `_cga_build_null_tbl` rewritten into its natural `continue` form, inner `continue` firing **1024** times per build | FNV mismatch, **1024 / 1024** coefficients flagged by `_CGA_NULL_COEF_BAD` | contract FNV exact, **2088 / 2088** (2086 + 2 probe assertions) |

⚠ **The first run of that pair read 8/8 on BOTH toolchains and was the instrument, not the
compiler.** `~/.cyrius/versions/6.6.2/bin/cyrius` reports itself as 6.6.2 from `version` but its
subcommands dispatch to the **cwd's manifest pin** — run from the cyrius repo (pinned 6.6.3) it
compiled with 6.6.3. A paired probe has to run from a directory whose manifest pins the version under
test. **Check the probe before believing the probe.**

**Consequence for hisab:** the `if`-guard form in `_cga_build_null_tbl` is **no longer
load-bearing** on the pinned toolchain, and its comment no longer claims to be. It is **kept**, for a
stated reason: `dist/hisab.cyr` is compiled by each consumer under its own pin, no live consumer had
crossed 6.6.2 at the time of writing, and the `continue` form fails **silently** below 6.6.3 — the
guard form is correct under every cycc. Same disposition as `m4_mul_vec4`'s hoist in 2.11.5.
