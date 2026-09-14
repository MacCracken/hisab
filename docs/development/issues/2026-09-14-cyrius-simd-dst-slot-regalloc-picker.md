# 2026-09-14 — cyrius: register picker leaves an `f64v_*` DESTINATION slot unwritten when a derived setter's value reads the same object through the derived getter (silent wrong code)

**Component:** `cyrius` / `cycc` — the Poletto-Sarkar linear-scan register picker (v5.6.20) against
the SIMD intrinsic handlers' frame-slot operands (`_SIMD_RESERVE`, 6.6.2).
**Toolchain seen:** cyrius **6.6.4** (x86_64). **Placement: unpinned within 6.6.x** — the repro
exits 1 on **6.6.0, 6.6.1, 6.6.2, 6.6.3 and 6.6.4** alike, each run from a scratch dir pinned to
that version (`build -v` naming `versions/<v>/bin/cycc`), so it is not a 6.6.4 regression and the
6.6.2 `_SIMD_RESERVE` repair never covered it. No pin below 6.6.0 is installed on this box.
**Severity:** **Critical** — wrong code. A 128-bit packed store is issued through a frame slot no
instruction in the function writes. In a 60-line standalone program it exits 0 with a wrong answer;
inside hisab's 35-module tree the same shape SIGSEGVs, so the crash is the lucky half.
**Hisab impact:** found while repairing `m3_mul_vec3` (`src/mat3.cyr`) onto `sizeof(HVec3)` and the
derived accessors for the 3.2.0 struct-layout gate. The natural rewrite — `HVec3_set_z(r,
f64_add(HVec3_z(r), …))` between the `f64v_*` calls — died on every call through a global-operand
loop while passing a single local-operand call. Shipped instead with the z tail accumulated in a
local and stored through the setter ONCE; the comment on the function names this file.
**Status:** 🔴 **OPEN — filed upstream 2026-09-14** at
`cyrius/docs/development/issues/2026-09-14-hisab-simd-dst-slot-regalloc-picker.md` with the repro
beside it under `repros/`, unchanged. hisab ships `m3_mul_vec3` in the hoisted form until a pin
crosses the fix; archive this file when it does.

## Summary

In a function that expands `f64v_*` intrinsics, a `#derive(accessors)` SETTER whose value argument
contains the derived GETTER of the SAME object makes the register picker rewrite one intrinsic's
destination-slot STORE into a register move (`mov %rax,%r13`) while the intrinsic's inline loop
still READS the destination from the frame slot (`mov -0xe8(%rbp),%rdx`). The slot is written by
nothing, and `movupd %xmm0,(%rdx,%rsi,8)` goes through whatever it held.

Each half alone is correct, verified with the same audit: a setter fed by a raw `load64`, or a raw
`store64` fed by the getter, both write every destination slot. Only the combination fails. The
6.6.2 `_SIMD_RESERVE` repair (which reserved the intrinsic's slots against argument-allocated frame
locals) does not cover this: the slot IS reserved; it is the picker's patch pass that removes the
store to it.

`CYRIUS_REGALLOC_PICKER_CAP=0` makes the same source correct, which isolates it to the picker.

## Reproduction

`docs/development/issues/repros/2026-09-14-simd-dst-slot-regalloc-picker.cyr` — self-proving:

```
cyrius build docs/development/issues/repros/2026-09-14-simd-dst-slot-regalloc-picker.cyr /tmp/repro
/tmp/repro          # prints "1 4 3" / WRONG, exit 1      (stock 6.6.4)
CYRIUS_REGALLOC_PICKER_CAP=0 cyrius build ... /tmp/repro_np
/tmp/repro_np       # prints "4 5 3" / OK, exit 0          (control)
```

`M = I` with column 3 = (1,1,1), `v = (1,2,3)`: `M v = (4, 5, 3)`. The stock build returns
`(1, 4, 3)` — the third column's contribution to x and y is lost because the third `f64v_scale`
wrote its result through a garbage pointer instead of into `tmp`, and the following `f64v_add`
read an untouched `tmp`.

Measured on the stock binary with an objdump audit (every `movupd %xmm0,(%rdx,%rsi,8)` whose
`%rdx` was loaded from a frame slot must have a store to that slot in the same function):

```
stock 6.6.4:                 packed stores checked: 5, unwritten destination slots: 1  (-0xe8)
CYRIUS_REGALLOC_PICKER_CAP=0: packed stores checked: 5, unwritten destination slots: 0
```

The hisab-shaped probe (`m3_mul_vec3` with the accessor rewrite, called 64x with global operands)
gives the same audit result — 32 packed stores, 1 unwritten, the same `-0xe8` — and SIGSEGVs at that
`movupd` with `%rdx = 0`. Disassembly of the faulting region, stock build:

```
4329b8: mov    %r12,%rax            ; tmp (picked into r12)
4329bb: mov    %rax,%r13            ; <- was the store to the dest slot; rewritten by the picker
4329cf: mov    %rax,-0xf0(%rbp)     ; src operand: stored
4329d9: mov    %rax,-0xf8(%rbp)     ; scalar operand: stored
...
432a14: mov    -0xe8(%rbp),%rdx     ; dest operand: READ from a slot never written
432a1b: movupd %xmm0,(%rdx,%rsi,8)  ; SIGSEGV, %rdx = 0
```

## What hisab did about it

`m3_mul_vec3` accumulates the z tail in a local and stores it through `HVec3_set_z` once, after the
last intrinsic. Audit: 32 packed stores, 0 unwritten; values bit-identical to the pre-repair form
(same three f64 ops in the same order). `foundation.tcyr` now multiplies by a matrix whose third
column is NOT zero and pins all three components bit-exactly — the only prior `m3_mul_vec3` test
used the identity and read only z, so it could not have seen `(1, 4, 3)`.

⚠ **Do not "tidy" `m3_mul_vec3` back to the natural accessor form until this is fixed upstream and
hisab's pin has crossed the fix.** The comment on the function says so; this file is the reason.
