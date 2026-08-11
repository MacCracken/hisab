# ad_tape_new returns a live handle whose node array is null

**Filed**: 2026-08-11 (P(-1) sweep at v2.11.0)
**Status**: 🟢 **FIXED in 2.11.1** — `src/autodiff.cyr`
**Severity**: High — SIGSEGV from a public constructor
**Provenance**: ⚠ **2.11.0's own code, four hours old when the audit found it**

## Summary

`ad_tape_new(cap)` allocated its node array and adjoint array without checking either. When
`cap * 40` exceeds `ALLOC_MAX` (2 GiB, i.e. `cap > 53,687,091`) the node allocation returns its
documented 0 — but the handle itself was already allocated, so the function returned a **non-zero
handle with a null body**.

A caller's `if (t != 0)` therefore passes, and the first `ad_var` dies inside `_ad_push`, whose
only bound is `n >= cap` — which `n = 0` satisfies happily.

## Reproduced

```
ad_tape_new(60000000)  ->  140129526087680   <- non-zero: the caller's check passes
AdTape_nodes(t)        ->  0                 <- alloc refused 2,400,000,000 B
ad_var(t, 1.0)         ->  EXIT 139          <- SIGSEGV
```

Threshold bisected at `ALLOC_MAX / 40 = 53,687,091`. After the repair the same call returns **0**.

## Fix

Check all three allocations; return 0 if any fails.

Returning 0 is not an invention — the function's **first line already reserves it**
(`if (cap < 1) { return 0; }`), and it returns a handle rather than an error code, so there is no
`HSB_ERR_ALLOC` channel available. Construction is the only place that can catch this.

No dimension cap was added, for the reason the sibling `num_ext` filing states: the honest failure
is "I could not allocate that", not "I refuse sizes above an arbitrary line".

## Verification

`tests/abuse.tcyr` asserts the rejection at `cap = 60,000,000`, that `cap = 0` still returns 0, and
that an ordinary cap still yields a tape **whose node array is really allocated** — without that
last one, "return 0 always" would pass. Removing the `nodes` guard fails the suite.

## Why it matters beyond the defect

2.11.0 built this tape, verified it against forward mode **and** finite differences, killed fourteen
mutants, and benchmarked it. None of that asks *what happens when the allocation fails* — every one
of those checks starts from a tape that exists.

⚠ **And 2.11.0 wrote the guard for exactly this class, three functions away.** `_ad_push` carries a
`return AD_FULL` for the full-tape case, with a comment about being "checkable, never a silently
dropped operation". The constructor that hands out the tape got no such treatment. **Handling a
failure mode in the hot path is not the same as handling it at construction**, and the release that
was most alert to this class still missed it in its own new code.
