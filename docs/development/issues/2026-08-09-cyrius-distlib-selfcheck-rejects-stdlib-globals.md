# 2026-08-09 — cyrius: `distlib`'s bundle self-check rejects a bundle that reads a stdlib global CONSTANT

**Component:** `cyrius` build tool — `cbt/commands.cyr` (the `distlib` bundle self-check added in
6.5.14), and whatever consumes `_cc_allow_undef` in cycc.
**Toolchain seen:** cyrius **6.5.16**. Introduced in **6.5.14** ("Fixed — `distlib`'s bundle
self-check had never once run"). Not present on 6.5.9 or earlier, where the self-check compiled to
`/dev/null` and therefore never executed.
**Severity:** **High for any project whose `[lib]` bundle reads a stdlib `var`** — `cyrius distlib`
exits **1** on a correct bundle, so it fails CI and release as a gate even though the artifact it
produces is fine. Low as a correctness risk: nothing is mis-generated.
**Hisab impact:** `cyrius distlib` exits 1 with

```
error:hisab.cyr:111:42: undefined variable 'F64_ONE' (missing include or enum?)
error: distlib: the generated bundle does not compile: dist/hisab.cyr
      (undefined fns are already downgraded — this is a REAL defect
       in the bundle, not the consumer's missing stdlib).
```

`dist/hisab.cyr:111` is `fn hvec2_one() { return hvec2_new(F64_ONE, F64_ONE); }`, from
`src/vec2.cyr`. `F64_ONE` is defined at `lib/math.cyr:15` and reaches a consumer through
`[deps] stdlib = [… "math" …]`. The error's parenthetical is exactly backwards for this case: it
**is** the missing stdlib, and the bundle has no defect.

## Root cause

`cbt/commands.cyr` (~L3117-3145) verifies a freshly written bundle by generating a one-line entry
next to it —

```
include "hisab.cyr"
```

— and compiling **that alone**, with no stdlib in scope, under `_cc_allow_undef = 1`. Its own
comment states the intent:

> Undefined fns are the ONE expected failure (a bundle ships no stdlib), so
> downgrade exactly that and let everything else stay fatal.

A bundle ships no stdlib **`var`s** either — but the deeper point is that the flag could not have
covered them even if someone had wanted it to. **The two symbol kinds fail in different compiler
stages, and `--allow-undef` only exists in the later one.**

`_cc_allow_undef` becomes a single `--allow-undef` argv token (`cbt/build.cyr:659-661`), and inside
cycc `_allow_undef` is read in exactly two places — `src/backend/x86/fixup.cyr:725` and
`src/backend/aarch64/fixup.cyr:559` — each gating a `undef_count` whose own message reads
`reachable undefined function(s) (pass --allow-undef to downgrade)`. That is the **fixup stage**, at
the end of the backend.

An unresolved *name*, by contrast, dies in the **frontend**, before any backend runs
(`src/frontend/parse_expr.cyr:585-593`):

```
var idx = FINDVAR(S, noff);
if (idx < 0) {
    ... ": undefined variable '" ... "' (missing include or enum?)\n"
    syscall(SYS_EXIT, 1);
}
```

`syscall(SYS_EXIT, 1)` — an immediate process exit. There is no path by which `--allow-undef` can
influence it. So this is not "vars were forgotten"; the self-check's premise (*the only expected
failure is an undefined symbol, and one flag suppresses it*) is unreachable for every symbol kind
resolved by name.

**Which means the defect class is wider than global variables.** Function *calls* are the only thing
that survives, because only they are deferred to fixup. Measured on 6.5.16 with three `[lib]`
modules identical but for one expression:

| `[lib]` module body | `cyrius distlib` |
|---|---|
| `fn c_fn(s) { return strlen(s); }` — stdlib **function** | **exit 0** (`warning: undefined function 'strlen'`) |
| `fn c_var() { return F64_ONE; }` — stdlib **global var** | **exit 1** (`undefined variable 'F64_ONE'`) |
| `fn c_enum() { return STDOUT_FD; }` — stdlib **enum constant** | **exit 1** (`undefined variable 'STDOUT_FD'`) |

A repo with no float constants at all is still hit, via `STDOUT_FD`, `SYS_WRITE`, or any other enum
member. `F64_ONE` is simply the most universal instance: it is how every Cyrius project writes the
literal `1.0`, because the language has no float literals.

## Minimal repro

Two projects, identical except for the single expression in the `[lib]` module. Both compile fine
for a real consumer; only one satisfies `distlib`.

```
# cyrius.cyml (both)
[package]
name = "repro"
version = "0.0.1"
language = "cyrius"
cyrius = "6.5.16"
[build]
src = "src/onlyfn.cyr"
output = "build/repro"
[lib]
modules = ["src/onlyfn.cyr"]     # or ["src/onlyvar.cyr"]
[deps]
stdlib = ["syscalls", "math"]
```

```
# src/onlyfn.cyr  — references a stdlib FUNCTION
fn probe_fn(a, b) { return f64_add(a, b); }

# src/onlyvar.cyr — references a stdlib GLOBAL VAR
fn probe_var() { return F64_ONE; }
```

Observed on 6.5.16:

```
$ cyrius distlib          # [lib] = onlyfn.cyr
dist/repro.deps: 2 stdlib leaf requirements
dist/repro.cyr: 2 lines (vunknown)                        <- exit 0

$ cyrius distlib          # [lib] = onlyvar.cyr
error:repro.cyr:9:32: undefined variable 'F64_ONE' (missing include or enum?)
error: distlib: the generated bundle does not compile: dist/repro.cyr
      (undefined fns are already downgraded — this is a REAL defect
       in the bundle, not the consumer's missing stdlib).      <- exit 1
```

## What is and is not affected

Established by running each, not by reading the code:

| Path | Result on hisab @ 6.5.16 | Note |
|---|---|---|
| `cyrius distlib` (write path) | **exit 1** | but the bundle **is written first** — see below |
| `cyrius distlib --check` (drift path) | **passes** (`current: dist/hisab.cyr`) | never compiles the bundle |
| `cyrius check --with-deps dist/hisab.cyr` | **`ok`** | compiles it with the stdlib resolved |
| `cyrius build` / `test` / `bench` / `fuzz` | unaffected | they do not go through the bundle |

**The bundle is written before the self-check runs**, which is what makes this survivable. Proven
rather than assumed: bumping `VERSION` 2.9.1 → 2.9.2 and running `cyrius distlib` exits 1 *and*
leaves `dist/hisab.cyr` correctly regenerated — `git diff` on it is exactly the one-line header
change `# Version: 2.9.1` → `# Version: 2.9.2`, with no other hunk.

## Blast radius beyond hisab

Sandbox-tested across every one of this machine's Cyrius repos carrying a `[lib]` block (each
repo's manifest + `[lib]` modules copied to a scratch directory, pin rewritten to 6.5.16, `cyrius
distlib` run — nothing written back into any source repo):

- **69** repos declare a `[lib]` bundle. **44** exit 1 once pinned to 6.5.16; **42** of those name an
  `undefined variable`.
- Only **3** are pinned at ≥ 6.5.14 today, so only they are broken right now: **hisab** (6.5.16,
  fails on `F64_ONE`), **majra** (6.5.14, fails on `CLOCK_MONOTONIC`), and **sigil** (6.5.14, passes
  — its bundle happens to reference no consumer-supplied global, which is very likely why upstream
  shipped 6.5.14 without noticing).
- The remaining ~41 are latent and fire on their next toolchain bump.

That split is the argument for prioritising an upstream fix, and it is also the practical advice:
a repo with a `[lib]` bundle should expect this the moment it crosses 6.5.14.

## Hisab's response (shipped in 2.9.2)

`.github/workflows/ci.yml` and `release.yml` both ran a bare `cyrius distlib`, so **both were red
on 6.5.16**. They now:

1. capture `distlib`'s output and rc, and tolerate a non-zero rc **only** when it carries this
   exact signature (`does not compile` *and* `undefined variable`). Any other distlib failure still
   fails the job, so a genuinely broken bundle is not waved through;
2. keep the drift check (`git diff --quiet dist/hisab.cyr`) unchanged — that is what the gate was
   always for, and it never depended on the self-check;
3. add `cyrius check --with-deps dist/hisab.cyr` as a separate step.

Point 3 is the important one: it is **strictly stronger** than the self-check it replaces. The
self-check compiles the bundle with every stdlib symbol undefined and then has to pretend a whole
category of undefined symbol is expected — which is precisely the hole this issue is about. Passing
the manifest's stdlib in scope compiles the bundle the way a consumer actually builds it, with no
category of error suppressed. What is lost: nothing hisab was getting, since the self-check has been
failing on hisab from the moment it started running.

## Suggested upstream fix

**Preferred: compile the self-check bundle with the project's own `[deps]` stdlib in scope** — i.e.
give the generated entry the same treatment `cyrius check --with-deps` gets. That tests the bundle
in the scope a consumer actually builds it in, needs no suppression flag at all, and is immune to
the whole class rather than to one symbol kind. The manifest is already parsed at this point, and
`distlib` has just written the `.deps` sidecar naming exactly those leaves.

The alternative — extending suppression to names — is a poor second: it means adding an
`_allow_undef`-equivalent gate to the frontend's `FINDVAR` failure path *and* to every other
name-resolution exit, and it would still be verifying the bundle in a scope no consumer uses.

A second, smaller point: the two `_info` lines asserting "this is a REAL defect in the bundle, not
the consumer's missing stdlib" should not be printed unconditionally, since the one case they
exclude is the case that fires here.

**Upstream:** filed 2026-08-09 at
`cyrius/docs/development/issues/hisab-distlib-selfcheck-rejects-stdlib-globals.md`
(Severity High, unpinned on the 6.5.x line), carrying the root-cause reading, the three-way
fn/var/enum reproducer, the 6.5.9 / 6.5.13 / 6.5.14 / 6.5.16 bisection, and the 69-repo blast
radius. Preferred fix proposed there: compile the self-check bundle with the project's own
`[deps]` stdlib in scope — the `cyrius check --with-deps` shape — rather than extending
suppression to name resolution.
**Status:** 🔴 **OPEN** — live on 6.5.16, worked around in hisab's CI as above.
