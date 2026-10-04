# 2026-10-03 — cyrius: a closure's `return <: stack call>` is booked against the ENCLOSING fn (roadmap D082)

**Component:** cyrius / cycc frontend, `src/frontend/parse.cyr`. `_pair_prescan` (line 104) and
`_warn_mixed_pair_returns` (line 257) skip nested bodies on the `fn` token only (lines 158 and 295),
so a closure literal's body is scanned as the enclosing fn's.
**Toolchain seen:** cyrius **6.6.12** when hisab found it (3.3.1 work, 2026-09-30), and **6.6.14** at
filing. The repro fails on every installed pin from 6.6.0 to 6.6.14.
**Severity:** Medium upstream: correct code is refused, and the error's suggested fix reads an unset
`rdx`. For hisab it is nil since 3.3.1 (see *Exposure*).
**Status:** 🟡 **OPEN UPSTREAM.** This was roadmap *Decisions owed* #4 (`D082`). The maintainer
approved filing on 2026-10-03. It is filed as
`cyrius/docs/development/issues/2026-10-03-hisab-closure-stack-return-blamed-on-enclosing-fn.md`, with
the self-proving repro
`cyrius/docs/development/issues/repros/2026-10-03-hisab-closure-stack-return-blamed-on-enclosing-fn.sh <pin>`.
hisab wrote both files and committed neither. Committing them is cyrius's own process.

## The defect

```cyrius
fn h(x) { return Ok(x); }
fn mk(b) { var g = |x| { return h(x + b); }; return g; }   # mk returns ONE value
fn main() { var g = mk(41); return 0; }                    # error: "bind both"
```

The closure's `return h(...)` flags `mk` as pair-returning. Then:
- `mk`'s own `return g;` draws "`mk` returns a `: stack` pair on another path but a SINGLE value
  here", a warning aimed at the wrong fn;
- `var g = mk(41)` inside a fn is a hard error ("a `: stack` enum returns two values — bind both");
- at top level the same bind draws only the warning, because cycc's single-variable refusal runs
  only inside fns (`parse_decl.cyr:4064`);
- the flag also propagates through a forwarder, `fn fwd(b) { return mk(b); }`;
- following the hint, `var t, v = mk(41)`, compiles with the warning only. `v` is an unset `rdx`,
  and `fncall1(v, 1)` exits 139.

The prescan's own header says closures must be skipped, for exactly that `rdx` reason. The skip
keys on `fn`, and a closure literal has no `fn` token.

## Measured, 2026-10-03

The repro script builds each case in a throwaway project pinned to the version under test, and
prints every build's `compiler: …/versions/<v>/bin/cycc` line.

| pin | exit | D1: helper bound in a fn | D2: helper bound at top level | D3: closure inline in a fn | controls C1–C3 |
|---|---|---|---|---|---|
| each of 6.6.0 – 6.6.14 | **3** | error + warning | warning | warning | pass |

Exit 0 means fixed. The controls are a real mixed return (must warn), a real lossy bind (must be
refused) and the workaround form (must build clean). Exit 99 means a control broke. Both verdict
paths were exercised. With the D rows rewritten to the workaround form, the script exits 0 on 6.6.14.
With C1's plain return made `Err(0)`, it exits 99. A user-declared `enum R: stack { A(v); B(e); }`
gives the same diagnostics on every pin, so the cause is not `Result`.

The recipe forms, built against the 3.3.4 bundle with the closure made by a helper fn, gave the same
result on all four pins probed (6.6.3, 6.6.12, 6.6.13 and 6.6.14):

| recipe form | helper's result bound inside a fn | bound at top level |
|---|---|---|
| through 3.3.0, `return ad_grad_into(...)` | **build fails** (error + warning) | warning; `opt_lbfgs` exits 0 |
| 3.3.1 onward: bind both, `return 0` | clean; `opt_lbfgs` exits 0 | clean; exits 0 |

## Exposure

**hisab: nil since 3.3.1.** hisab builds one closure, `tests/modules.tcyr`'s
`_ot_make_closure_grad`. Its body binds both halves of `ad_grad_into` and returns 0. The recipe in
`src/autodiff.cyr` is written the same way. `tests/modules.tcyr` binds that helper's result inside
a fn (`_ot_drive_closure_recipe`), which is the placement where a regression to the old recipe
fails to compile. A consumer who copied the 3.3.0 recipe hits the error wherever the closure
sits: built by a helper fn, or built inline in fn A, a single-variable bind of that fn's result
inside another fn is refused. Probe I1 on 6.6.14:
`fn run(b) { var g = |x| { return h(x + b); }; return 0; } fn main() { var r = run(41); return r; }`
gives the error and a warning on `run`.

## What hisab does

- The comments in `src/autodiff.cyr` and `tests/modules.tcyr` that described this as the
  "6.6.3 to 6.6.12" range now say 6.6.14 and name this record.
- No source change: the shipped recipe form is the workaround.
- **Closes when** a cyrius release makes the repro exit 0 and hisab's pin has crossed it. Rerun
  the repro on both pins as a pair, then move this file to `archived/`. The recipe stays in the
  bind-both form while any consumer pins below the fix.
