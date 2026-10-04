# 2026-10-03 — cyrius: `fncallN` at true top level SIGSEGVs on a capturing closure that a fn built and returned

**Component:** cyrius / cycc. The top-level `fncallN` route (`src/frontend/parse_expr.cyr:1547`, the
`_cur_fn_ix >= 0` gate on the v6.5.17 closure-aware lowering) falls through to `lib/fnptr.cyr`'s
plain `fncallN` asm.
**Toolchain seen:** cyrius **6.6.14** (hisab's pin from 3.3.4). The repro fails on every installed pin
from 6.6.0 to 6.6.14.
**Severity:** Medium for a consumer, nil in-tree (see *Exposure*). It compiles clean and SIGSEGVs at
run time, with no diagnostic.
**Status:** 🟡 **OPEN UPSTREAM.** Filed 2026-10-03 with the maintainer's approval, as
`cyrius/docs/development/issues/2026-10-03-hisab-toplevel-fncall-capturing-closure-segv.md`. Its
self-proving repro is
`cyrius/docs/development/issues/repros/2026-10-03-hisab-toplevel-fncall-capturing-closure-segv.cyr`.
hisab wrote both files and committed neither. Committing them is cyrius's own process.

## The defect

A fn builds a capturing closure and returns it. Calling it with `fncallN` **outside every fn body**
jumps to the closure value, which is its heap env pointer with bit 63 set, and faults:

```cyrius
fn mk(b) { return |x| b + x; }
var r = fncall1(mk(41), 1);      # top level: exit 139. Inside any fn: 42
sys_exit_group(r);
```

cycc lowers `fncallN` to its closure-aware dispatch only inside a fn, because the lowering spills
the callee to a frame slot and top-level code has no frame. At top level `fncallN` stays the
library call, whose asm is `call rax` on the raw value. The comment on that gate says nothing else
can reach it ("a closure literal needs a frame too"). An escaped closure was built in a frame and is
called outside one. `callptr` at top level is refused at compile time, and `fncallN` is not.

This is not the 6.5.17 defect regressing
(`archived/2026-08-10-cyrius-capturing-closure-across-fn-boundary.md`). It is the one route that fix
exempted on purpose. Its class matches 6.6.2's bare SIMD intrinsic at top level
(`dependency-watch.md`, 6.6.2 entry): top-level code has no frame. 6.6.2 resolved that one with a
compile-time refusal.

## Measured, 2026-10-03

Each pin was run from a scratch dir whose `cyrius.cyml` pins it. `cyrius build -v` printed
`compiler: …/versions/<v>/bin/cycc`, and every vendored `lib/*.cyr` byte-matched that cyrius tag.

| the filed repro | exit | its controls-only variant |
|---|---|---|
| 6.6.0 – 6.6.14, each pin, x86_64 | **139** | 0 |
| 6.6.14 `--aarch64` under `qemu-aarch64` | **139** | 0 |

The repro exits 0 once fixed. Its controls are the same value called through a one-line wrapper
fn, and a non-capturing closure called at top level. Both return 42 on every pin. No 6.5.x is
installed, so the first bad version is unknown. "At or below 6.6.0" is evidence about visibility,
not origin.

## Exposure

**hisab: nil in-tree.** The repository contains:
- 96 `fncallN`/`callptr` sites in `src/`, all inside fn bodies;
- 1 in `tests/modules.tcyr` (`_ot_drive`, inside a fn);
- none at top level in `src/`, `tests/` or `examples/`.

These counts come from a brace-depth scan with comments and strings stripped, which finds the
repro's top-level call at depth 0 as a positive control. The plain grep agrees at 97 lines.
hisab's only closure (built by `_ot_make_closure_grad`) is called only inside fns: the `opt_*` solvers and `_ot_drive`.

**Consumers: only through their own code.** The autodiff recipe's gradient closure was built by a
helper fn and run against the 3.3.4 bundle on 6.6.3, 6.6.12, 6.6.13 and 6.6.14. All four pins gave
the same result:

| use | exit |
|---|---|
| passed to `opt_lbfgs` at top level, where the solver calls it inside itself | 0, converges to (1, 2) |
| passed to `opt_lbfgs` inside a fn | 0 |
| the caller's own `fncall2(grad, x, g)` at top level | **139** |
| the caller's own `fncall2(grad, x, g)` inside a fn | 0, gradient (4, 6) at (3, 5) |

## What hisab does

- `src/autodiff.cyr`'s recipe note says: a caller that invokes such a closure itself must do it
  from inside a fn. Handing it to an `opt_*` solver from top level is fine.
- No source change, because no hisab path reaches the defect.
- **Closes when** a cyrius release makes the repro exit 0 and hisab's pin has crossed it. Rerun
  the repro on both pins as a pair, keep the recipe note while any consumer pins below the fix,
  and move this file to `archived/`.
