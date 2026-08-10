# 2026-08-10 — cyrius: a CAPTURING closure SIGSEGVs when passed through another function and called there

**Component:** `cyrius` / `cycc` — closure environment lowering (`|x| …` capture-by-value, guide
§Closures, v6.3.8).
**Toolchain seen:** cyrius **6.5.16**. Not bisected against earlier pins.
**Severity:** **High** — silent crash, not a diagnostic. The code compiles clean and dies at
runtime with SIGSEGV.
**Hisab impact:** none in shipped code (hisab calls no closure today), but it is the **exact shape
`vec_sort_by` needs**, and it is why that adoption stays parked. See *How this was found*.

## Summary

A capturing closure works when called in the function that built it. Pass the same closure to
another function and call it there, and the process segfaults. A **non**-capturing closure passed
the same way is fine, so the variable is capture-across-a-call-boundary.

## Reproduction

Four programs, `[deps] stdlib = ["syscalls", "io", "alloc", "math"]`, built with
`cyrius build src/t.cyr build/t` on 6.5.16. Each is otherwise identical.

```
# 1. non-capturing, passed through a fn      -> exit 42   OK
fn takes(f): i64 { return callptr(f, 1); }
fn main(): i64 { var f = |x| 41 + x; return takes(f); }

# 2. capturing, called inline                -> exit 42   OK
fn main(): i64 { var base = 41; var f = |x| base + x; return callptr(f, 1); }

# 3. capturing, passed through a fn          -> SIGSEGV (139)
fn takes(f): i64 { return callptr(f, 1); }
fn main(): i64 { var base = 41; var f = |x| base + x; return takes(f); }

# 4. same as 3 but fncall1 instead of callptr -> SIGSEGV (139)
fn takes(f): i64 { return fncall1(f, 1); }
fn main(): i64 { var base = 41; var f = |x| base + x; return takes(f); }
```

1 and 2 are the controls: each isolates one half of the failing case and both pass. 3 and 4 differ
from 2 only by crossing a function boundary, and from 1 only by capturing.

Also confirmed with a captured **pointer** (`var pts = alloc(32); var f = |i| load64(pts + i * 8);`):
called inline it returns the right element; passed through a function it segfaults.

## Root cause (speculation — flagged as such)

The guide says a capturing closure "*is*" its heap environment object `[fn_ptr, cap0, …]`, and that
`callptr` "auto-detects a captured closure and dispatches it (loads the fn pointer from the object
and passes the object itself as the hidden first argument)". The controls suggest that detection
works on the value while it is still the local the closure was assigned to, and stops working once
that value has been passed as an ordinary parameter — i.e. the callee sees the environment pointer
but dispatches it as a bare function pointer, jumping to whatever `cap0` happens to be. That the
crash is identical under `fncall1` points at the shared dispatch rather than at `callptr`.

**Not verified** — a consumer-side reading of the observable behaviour, not of the codegen.

## Proposed fix

None offered; this needs someone who knows the lowering. The useful constraint from out here is
that whatever makes `callptr` work at the definition site has to survive the value being copied
into a parameter slot — or the copy has to be rejected at compile time instead of crashing at run
time. A hard error would be far better than a segfault.

## Consumer-side workaround

Do not pass capturing closures across a function boundary. Either call them where they are built,
or hoist the captured state into a file-scope global and use a non-capturing closure.

## How this was found, and why it matters beyond the bug

hisab's own records asserted **"Cyrius has no closures"** in four places
(`dependency-watch.md`, `roadmap.md`, `src/collision_core.cyr`, and two CHANGELOG entries), and used
it as the stated reason for not adopting stdlib `vec_sort_by`. **That claim is false and had never
been run** — closures have existed since v6.3.8. The claim was inherited, repeated, and propagated.

Checking it turned up this defect. So the conclusion those files reached (don't adopt `vec_sort_by`
yet) happens to be right, but for a completely different reason than the one recorded — and it is
now recorded as a measurement with a repro instead of as a belief.

**Upstream:** filed 2026-08-10 at `cyrius/docs/development/issues/hisab-capturing-closure-segv-across-fn-boundary.md`,
with a standalone repro at `issues/repros/2026-08-10-capturing-closure-across-fn-boundary.cyr` — re-run
from the filed file itself and it reproduces (SIGSEGV, exit 139).
**Status:** 🔴 **OPEN** — live on 6.5.16.
