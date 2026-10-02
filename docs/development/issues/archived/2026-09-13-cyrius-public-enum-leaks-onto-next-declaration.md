# 2026-09-13 — cyrius: in a `private` file, a `public enum` hands its `public` to the NEXT declaration

**Component:** `cyrius` / `cycc` — the top-level visibility marker (`_TL_VIS`): `public` armed it
unconditionally and only a fn or global-var definition consumed it, so after an enum it was still
armed when the next declaration was registered.
**Toolchain seen:** cyrius **6.6.2 and 6.6.3**, identically.
**Severity:** **High, silent** — a file-private fn or var becomes reachable, callable and writable
from any other file, with no diagnostic, because of what precedes it in the source.
**Hisab impact:**
- **Found by hisab's own gate.** 3.1.0's `scripts/check-public-surface.sh` generated a consumer that
  called every one of hisab's 457 non-public top-level items against a fully `private` bundle. 456
  were refused; the one accepted was `_ad_pow`, the declaration right after `public enum
  AdPowLimit` in `src/autodiff.cyr`. The gate carried it as a `KNOWN_LEAKS` entry keyed to the
  filing, built to FAIL the day the leak stopped.
- **And it nearly shipped.** In 3.3.0's first-draft bundle `_SYM_EPS` sat directly after `public
  enum ExprTag` in `src/symbolic.cyr`. While it was itself public (through 3.2.x) that was
  invisible; 3.3.0 made it private, and under 6.6.3 a consumer could read and write it from that
  draft (510 / 511 non-public probes refused). The gate missed it because CI runs only the repo's
  pin. It was caught before the tag, in 3.3.0's pre-tag review (CHANGELOG.md [3.3.0], *Fixed —
  found by the review*), by the manual multi-pin build step (roadmap, *Decisions owed* #7).
**Status:** ✅ **CLOSED — FIXED UPSTREAM IN cycc 6.6.4** (`public` arms only for a token that can
carry visibility and is consumed otherwise). Taken by hisab 3.1.1 (2026-09-14): on the pin bump,
before any source changed, the gate reported `_ad_pow` **REFUSED** (456/457 → 457/457) and failed as
designed; `KNOWN_LEAKS` has been empty since. Filed upstream 2026-09-13; archived there as
`cyrius/docs/development/issues/archived/2026-09-13-hisab-public-enum-leaks-onto-next-declaration.md`,
repro at `cyrius/docs/development/issues/repros/2026-09-13-hisab-public-enum-leaks-onto-next-declaration.cyr`.
Its closure is recorded in CHANGELOG.md [3.1.1] and the 6.6.4 entry of
`docs/development/dependency-watch.md`. hisab kept no record of its own until 3.3.3 (audit `D139`).

## Closure — the paired measurement, run 2026-10-01 for this record

Each side from a scratch dir whose `cyrius.cyml` pins the version; `lib_min.cyr` is the library
block in the repro's header (`private`; `public enum PubOne { PO_A = 1 }`; `fn _after_pub_enum()
{ return 3; }`; `fn _second_after() { return 4; }`; `public fn api() { return PO_A; }`). The
`compiler:` line of `cyrius build -v` read `versions/6.6.3/bin/cycc` and `versions/6.6.4/bin/cycc`.

| program (a second file including `lib_min.cyr`) | 6.6.3 | 6.6.4 |
|---|---|---|
| the repro: `return _after_pub_enum();` | builds, runs, **exit 3** — the private fn ran from another file | **refused**: `'_after_pub_enum' is private to its file`, no binary |
| control: `return _second_after();` (the declaration after that) | refused, no binary | refused, no binary |
| positive control: `return api() + PO_A;` | builds, exit 2 | builds, exit 2 |

## What hisab does now

- **Consumers below 6.6.4 still have the defect**, and the 3.x bundle's documented floor is 6.6.3.
  `scripts/check-public-surface.sh` claim 5 fails any shipped bundle in which a non-public
  declaration directly follows a `public enum`, which closes the class for hisab's bundle without
  running 6.6.3 in CI. `ExprTag` sits directly above `public fn expr_tag`, and the comment above
  the enum says why that slot must stay public.
- The probes stay calls, not `&name` — see the companion record
  `2026-09-13-cyrius-private-fn-reachable-via-address-of.md`.
