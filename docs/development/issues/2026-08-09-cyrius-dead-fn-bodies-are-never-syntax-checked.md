# 2026-08-09 — cyrius: a syntax error inside an uncalled function is accepted by every gate (discriminator: an underscore in the function's name)

**Component:** `cyrius` / `cycc` — the dead-function skip in the frontend, and every gate built on it
(`build`, `check`, `vet`, `lint`).
**Toolchain seen:** cyrius **6.5.16**. Not bisected against earlier pins.
**Severity:** **Medium-High as a gate hole, Low as a live risk for hisab today.** Nothing is
mis-compiled; the problem is that a whole category of source is never checked, so a defect can sit
in the tree indefinitely and every gate reports green.
**Hisab impact:** **3 of 842** distinct `fn` names in `src/` contain no underscore — `main`,
`einsum`, `minkowski` — and all three are called (`einsum` 40 references across `src/` + `tests/`,
`minkowski` 10, both asserted in `tests/abuse.tcyr`), so nothing is hiding in hisab right now. The
exposure is forward-looking: any future no-underscore function that loses its last caller silently
stops being syntax-checked, and the naming convention that protects hisab is an accident rather
than a decision.

## Symptom

A function that nothing calls, and whose name contains **no underscore**, has its body skipped
entirely. Arbitrary garbage inside it compiles green.

```
# src/g.cyr
fn gg() { var x = ; this is not cyrius at all ]] ((
    return 0; }
fn main() { return 0; }
var r = main();
sys_exit_group(r);
```

Measured on 6.5.16 (`[deps] stdlib = ["syscalls"]`):

```
cyrius build src/g.cyr build/g              exit=0   OK
cyrius lint src/g.cyr                       exit=0   0 warnings
cyrius vet src/g.cyr                        exit=0
cyrius check --with-deps src/g.cyr          exit=0
```

Rename `gg` → `g_g`, changing **nothing else**, and the same file is rejected:

```
cyrius build src/g.cyr build/g              exit=1   error:<source>:3:20:
cyrius check --with-deps src/g.cyr          exit=1   error:<source>:3:20:
cyrius lint src/g.cyr                       exit=0   0 warnings      <- lint never catches it either way
```

Both conditions are required. With `gg` **called** from `main`, the error is reported normally
(`exit=1`), so it is the combination *uncalled* **and** *no underscore in the name*.

Confirmed across six names, holding everything else fixed:

| name | has `_` | `cyrius build` |
|---|---|---|
| `gg` | no | **exit 0** |
| `victim` | no | **exit 0** |
| `abcdefghij` | no | **exit 0** |
| `broken_fn` | yes | exit 1 |
| `victim_x` | yes | exit 1 |
| `abcde_ghij` | yes | exit 1 |

The underscore is doing real work here, which is what makes this worth filing rather than
shrugging at as "dead code is not compiled". A rule keyed on a character in an identifier is
almost certainly not the intended rule.

## Why it matters even though nothing is mis-compiled

1. **`cyrius lint` returns 0 on a file that does not parse**, in both the underscore and the
   no-underscore case. A gate whose whole job is static analysis reports "0 warnings" on garbage.
2. The skip is invisible. `cyrius build` prints `OK` and its usual `note: N unreachable fns`; nothing
   distinguishes "N functions were compiled and found unreachable" from "N functions were never
   parsed".
3. It interacts badly with the ordinary lifecycle of a private helper: a function is written, used,
   later has its last caller removed during a refactor, and from that moment its contents stop being
   checked — with no diagnostic at the moment the coverage is lost.

## Suggested upstream fix

Parse every function body regardless of reachability, and let dead-code elimination act on the
*compiled* result — reachability is a codegen concern, not a parsing one. If skipping is retained
for compile speed, the skip must not be keyed on the identifier's spelling, and `cyrius lint` at
minimum should always do a full parse, since analysing unreachable code is precisely what a linter
is for.

## Notes on measurement

- ⚠ **One reading in the first pass contradicted the others, and the cause was the harness, not the
  compiler.** A loop that renamed the function with `sed -i` left the file in its *underscore* state,
  so a later "same file" run was not the same file. Re-tested with the name restored, all four
  capture forms agree — `>/dev/null 2>&1`, redirect to a file, command substitution, and plain
  `>/dev/null` all return **exit 0** for the no-underscore case. Recorded because the intermediate
  theory ("shell redirection perturbs the exit code here") was written down before it was checked,
  and it was wrong.
- Diagnostic line numbers for the file named on the command line are offset by (stdlib dep count
  + 1), because they refer to the concatenated translation unit — hence `<source>:3:20` for an error
  on line 1. Errors inside `include`d files carry the true filename and line. Not part of this
  defect, but it will confuse anyone reproducing it.

**Upstream:** filed 2026-08-09 at
`cyrius/docs/development/issues/hisab-dead-fn-bodies-never-syntax-checked.md` (Severity Medium,
unpinned on the 6.5.x line), with a standalone repro at
`cyrius/docs/development/issues/repros/2026-08-09-dead-fn-body-not-parsed.cyr`. The repro was
re-run from the filed file itself and reproduces all three states: as filed exit 0 on
build/lint/vet/check, exit 1 after renaming `gg` → `g_g`, exit 1 after calling `gg` from `main`.
Root cause is left as an open question there — `CYRIUS_DCE_VERBOSE=1` lists both variants as
`dead:` but only the underscore one contributes bytes, which is flagged as speculation for the
Cyrius side to confirm.
**Status:** 🟡 **PARTIALLY FIXED in cyrius 6.5.17, verified 2026-08-10.** Upstream's entry claims
*"a syntax error in an uncalled fn was accepted by every gate (hisab, Medium)"* — but **two gates
still accept it**. Re-run from the filed repro on 6.5.17:

| gate | 6.5.16 | 6.5.17 |
|---|---|---|
| `cyrius build` | exit 0 ❌ | **exit 1** ✅ |
| `cyrius check --with-deps` | exit 0 ❌ | **exit 1** ✅ |
| `cyrius lint` | exit 0 ❌ | **exit 0** ❌ still |
| `cyrius vet` | exit 0 ❌ | **exit 0** ❌ still |

The load-bearing half is repaired: a file that does not parse can no longer be **built** or
**checked**, so nothing broken ships. What remains is that `lint` and `vet` still report success on
a file that does not parse — and for `lint` that is the sharper edge of the two, since analysing
code that never runs is a large part of what a linter is for. Whether that is in scope or by design
(lint/vet may deliberately not do a full parse) is upstream's call; it is recorded here as measured
rather than assumed.

hisab needs no workaround either way — its build and check gates are the ones that now catch it.
