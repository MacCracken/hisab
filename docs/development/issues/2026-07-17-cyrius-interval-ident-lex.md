# 2026-07-17 — cyrius lexer: identifiers `iv_add`/`iv_sub`/`iv_mul` lex as "unknown"

**Component:** `cyrius` lexer / identifier interning (cycc `lex.cyr`)
**Toolchain seen:** cyrius 6.4.66 — **also reproduces on 6.3.11, 6.4.0, 6.4.65** (not a
6.4.x regression; pre-dates the 2.6.9 toolchain bump)
**Severity:** High for the affected suite — a whole `.tcyr` fails to **compile**, and the
diagnostic points at a clean identifier, so the real cause is invisible without bisection.
**Hisab impact:** `tests/modules.tcyr` — the interval-arithmetic section named its result
vars `iv_add` / `iv_sub` / `iv_mul`. Any compile unit that includes `src/interval.cyr` and
declares those names fails at the first one (`var iv_add = ivl_add(...)` →
`expected identifier, got unknown`), taking the entire 312-assertion suite (and thus the
whole 957-assertion run) down at compile time. Discovered during the v2.6.9 bump; the suite
had been silently un-compilable.
**Hisab workaround (shipped in 2.6.9):** renamed the three result vars to
`iv_sum` / `iv_diff` / `iv_prod`. Suite compiles deterministically (0/8 fail) and runs
312/312. A `NOTE:` comment at `tests/modules.tcyr` `test_group("interval: arithmetic")`
warns against renaming them back.
**Status:** Open (toolchain bug) — worked around in-tree. No known cycc fix.

## Symptom

In a compile unit that includes `src/interval.cyr`, the identifiers `iv_add`, `iv_sub`,
`iv_mul` are lexed as an `unknown` token when they appear as a `var` name — the parser then
reports `expected identifier, got unknown` at that line and desyncs (cascading
`expected '(', got ')'` on the following `assert_eq` lines, then `undefined variable` for
the never-bound name). The sibling names `iv_div`, `iv_neg`, `iv_abs`, `iv_a`, `iv_b` are
**not** affected. Behaviour is **deterministic** for a given source (8/8 fail with the toxic
names, 8/8 pass renamed) — it is not run-to-run flaky; it depends on the identifier
population of the unit (a lex/intern hash collision is the leading hypothesis).

Note: `iv_add` / `iv_sub` / `iv_mul` each have a one-character-longer library counterpart
(`ivl_add` / `ivl_sub` / `ivl_mul`), but `iv_div` — which also has `ivl_div` — is fine, so a
naive "collides with the `ivl_*` name" explanation does not hold; the trigger is
hash-slot-specific.

## Self-contained reproducer

```cyrius
include "src/f64_util.cyr"
include "src/error.cyr"
include "src/interval.cyr"

alloc_init();
test_group("repro");
var iv_a = ivl_new(f64_from(1), f64_from(3));
var iv_b = ivl_new(f64_from(2), f64_from(4));
var iv_add = ivl_add(iv_a, iv_b);          # <-- error: expected identifier, got unknown
assert_eq(f64_to(Interval_lo(iv_add)), 3, "lo");
```

`cyrius test <file>` → fails 6/6. Rename `iv_add`→`iv_sum` (and `iv_sub`/`iv_mul`
likewise) → passes 6/6.

## Suggested upstream fix

Audit the lexer's identifier interning / dedup hash for the `unknown`-token fallback path —
a colliding or truncated intern for these specific byte sequences appears to mis-classify the
token. Until then, the in-tree rename is the mitigation.
