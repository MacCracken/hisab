# 2026-09-11 — cyrius: `#derive(...)` and `public` cannot be combined

**Component:** `cyrius` / `cycc` — `PP_PARSE_STRUCT_DEF` in `src/frontend/lex_pp.cyr`.
**Toolchain seen:** cyrius **6.6.2**.
**Severity:** **High** — a hard compile error with no workaround, on the exact shape every
foundation type needs. Not wrong code.
**Hisab impact:** blocked the public/private surface planned for 3.0.0. `private`/`public` itself
works and the boundary holds (verified end to end — a consumer of `dist/hisab.cyr` could not reach
a file-private helper: build fails, exit 1, no binary), but privatising hisab produced
**1,436 `'HVec3_x' is private to its file` errors** from the 18 modules that `#derive(accessors)`,
because without `public` on the struct its generated accessors stay file-private. Reverted rather
than half-applied; recorded in the 3.0.0 CHANGELOG and the roadmap row.

## Summary

```cyrius
#derive(accessors)
public struct P { x; y; }      # error: #derive(...) applies to a struct or an enum;
                               #        the following declaration is neither
```

The control differs by exactly one keyword (`struct P2 { x; y; }` builds). ⚠ Neither file contains
`private` — this is purely the parse of `public` in front of a derived declaration, not a
visibility interaction.

## Root cause (upstream)

`PP_PARSE_STRUCT_DEF` byte-compares the literal `"struct "` / `"enum "` AT the declaration
position, so `public struct P` matched neither probe. 6.6.3 probes for a `public ` prefix first, and
— because compiling was not enough — **propagates `public` onto the generated accessors**, verified
in both directions upstream (a non-public struct in a private file still yields private accessors).
Premise-checked upstream: the `#inline`-disarms-`#derive` fix (same release) does NOT close this,
which the filing had asked.

**Upstream:** filed 2026-09-11 at
`cyrius/docs/development/issues/2026-09-11-derive-cannot-combine-with-public.md`; archived there on
the 6.6.3 release with gate `tests/gates/frontend/derive_with_public.sh` (5 axes, mutation-proven).

**Status:** 🟢 **FIXED in cyrius 6.6.3, verified 2026-09-13 on the 3.0.1 bump — on hisab's own
idiom, as a pair, in both directions.** A two-file probe: a `private` library file with
`#derive(accessors) public struct P`, `public fn` constructors using `alloc(sizeof(P))` + the
derived setters (the way `hvec3_new` is written), and a file-private `_hidden()`; a consumer file
reading `P_x`/`P_y`, calling `P_set_x`, and a second consumer calling `_hidden()`.

| probe | 6.6.2 | 6.6.3 |
|---|---|---|
| control: same files, visibility keywords removed | builds, exit 0 | builds, exit 0 |
| derive + `public` consumer | **compile error** (the filing) | builds, **exit 0** — getters AND setter reachable cross-file, values correct |
| consumer calling `_hidden()` | (masked by the same compile error) | **rejected**: `'_hidden' is private to its file`, no binary |

⚠ My first two probes were wrong before the compiler was: one used per-item `private fn`, which is
not the syntax (`private` stands alone on a line; 6.6.3's diagnostic said so), and the next used a
struct literal `P { x: a, y: b }` that SIGSEGV'd — **on the control too**, which is what named it as
the probe's fault. hisab's own construction idiom is the one that runs.

**Consequence for hisab:** the roadmap's *Public / private function surface* row is **UNBLOCKED**.
The work itself — 939 functions, 296 underscore-prefixed, 148 functions and 25 globals crossing a
module boundary — stays scheduled where it was (`pub fn` half 3.1.0, `private` flip 4.0.0), and the
row's own landmine still applies: a consumer-call gate has to exist before the flip, because
`cyrius check --with-deps dist/hisab.cyr` stays green on a bundle no consumer can call.
