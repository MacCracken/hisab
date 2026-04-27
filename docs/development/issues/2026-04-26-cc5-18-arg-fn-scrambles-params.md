# 2026-04-26 — cc5 5.7.10: 18-arg fn call scrambles register/stack params

**Component:** cc5 codegen (x86_64 SysV calling convention)
**Toolchain seen:** cyrius 5.7.10
**Severity:** Silent miscompile — wrong values, no warning, downstream segfault
**Hisab impact:** `lib/calc.cyr` `_perm_init` (Perlin noise table init) corrupts the 256-byte permutation table, causing every subsequent `perlin_2d` call to crash. Surfaced as bench segfault (exit 139) after `ease_in_out`.
**Hisab workaround:** `_perm_store_block` (18 args) refactored into `_perm_store_8` (10 args) called twice as often. See `lib/calc.cyr` `_perm_init` and the helper at the bottom of section 4.
**Status:** Open upstream. Hisab unblocked.

## Symptom

A function with 18 parameters reads garbage for parameters 1, 2, and 7-12 when called. No warning at the call site. Bytes get scrambled, downstream code accesses garbage values, eventually crashes (or silently computes wrong results).

`n8`, `n10`, `n12`, `n14`, `n16` all work correctly with the same per-arg-uses test pattern. The bug appears at 18 arguments.

## Self-contained reproducer

Save as `/tmp/cc5_18arg.cyr` and build with `cyrius build` from a directory whose `cyrius.cyml [deps] stdlib` resolves the includes:

```cyrius
include "lib/syscalls.cyr"
include "lib/string.cyr"
include "lib/alloc.cyr"
include "lib/str.cyr"
include "lib/fmt.cyr"
include "lib/io.cyr"

fn show18(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r) {
    print_num(a); println(" =a (expect 100)");
    print_num(b); println(" =b (expect 200)");
    print_num(c); println(" =c (expect 300)");
    print_num(d); println(" =d (expect 400)");
    print_num(e); println(" =e (expect 500)");
    print_num(f); println(" =f (expect 600)");
    print_num(g); println(" =g (expect 700)");
    print_num(h); println(" =h (expect 800)");
    print_num(i); println(" =i (expect 900)");
    print_num(j); println(" =j (expect 1000)");
    print_num(k); println(" =k (expect 1100)");
    print_num(l); println(" =l (expect 1200)");
    print_num(m); println(" =m (expect 1300)");
    print_num(n); println(" =n (expect 1400)");
    print_num(o); println(" =o (expect 1500)");
    print_num(p); println(" =p (expect 1600)");
    print_num(q); println(" =q (expect 1700)");
    print_num(r); println(" =r (expect 1800)");
    return 0;
}

fn main() {
    alloc_init();
    show18(100, 200, 300, 400, 500, 600, 700, 800, 900,
           1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800);
    return 0;
}

var r = main();
syscall(60, r);
```

### Observed output (cc5 5.7.10, x86_64 linux)

```
0 =a (expect 100)            # rdi clobbered to 0
0 =b (expect 200)            # rsi clobbered to 0
300 =c (expect 300)
400 =d (expect 400)
500 =e (expect 500)
600 =f (expect 600)
100 =g (expect 700)          # arg 7 reads slot 1's value
200 =h (expect 800)          # arg 8 reads slot 2's value
1500 =i (expect 900)         # arg 9 reads slot 15's value
1600 =j (expect 1000)
1700 =k (expect 1100)
1800 =l (expect 1200)
1300 =m (expect 1300)
1400 =n (expect 1400)
1500 =o (expect 1500)
1600 =p (expect 1600)
1700 =q (expect 1700)
1800 =r (expect 1800)
```

Pattern: register args 1-2 zeroed, args 3-6 correct, stack args 7-12 read scrambled slots, stack args 13-18 read correctly.

## Bisecting the threshold

```cyrius
fn n8 (a,b,c,d,e,f,g,h)                                 { return a + h; }
fn n10(a,b,c,d,e,f,g,h,i,j)                             { return a + j; }
fn n12(a,b,c,d,e,f,g,h,i,j,k,l)                         { return a + l; }
fn n14(a,b,c,d,e,f,g,h,i,j,k,l,m,n)                     { return a + n; }
fn n16(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p)                 { return a + p; }
fn n18(a,b,c,d,e,f,g,h,i,j,k,l,m,n,o,p,q,r)             { return a + r; }
```

n8: 1+8=9 ✓; n10: 1+10=11 ✓; n12: 1+12=13 ✓; n14: 1+14=15 ✓; n16: 1+16=17 ✓; **n18: expected 1+18=19, got 34** ✗.

So the threshold is between 16 and 18 args. n17 not tested.

## Where to look in cc5

The x86_64 SysV ABI puts args 1-6 in `rdi/rsi/rdx/rcx/r8/r9` and the rest on the stack (last arg deepest, first stack arg at `[rbp+16]`). The cc5 implementation:

- Call site: `ECALLPOPS(S, n)` in `src/bridge.cyr` — pops `min(n, 6)` args into registers. For >6 args, somewhere the remaining args must get pushed before the call.
- Callee: `ESTORESTACKPARM(S, pidx, disp, pc)` in `src/bridge.cyr` ~L1530 — stores stack arg at `[rbp + 16 + (nstack-1-(pidx-6))*8]` to local slot `disp`.

The arg-reorder pattern in the observed output (args 7-12 read from wrong stack slots) suggests `nstack` calculation in `ESTORESTACKPARM` may be off when `nstack > 10`. `nstack = pc - 6`; for 18-arg `nstack = 12`. Earlier "7+ arg stack offset" bugs are in cc5 history (CHANGELOG search "Bug #32").

## Verification once a fix lands

The reproducer above is a one-shot pass/fail. Re-running it against a candidate-fix cc5 should print every "expect X" matching the actual value and exit 0.

A second verification: re-enable hisab's pre-fix `_perm_store_block` (18-arg) form in `lib/calc.cyr`, build the bench (`cyrius bench tests/hisab.bcyr`), and confirm `perlin_2d` runs to completion with sensible numbers. Pre-fix git revision will be tagged once the upstream fix lands so the comparison is mechanical.

## Hisab follow-up after upstream fix

1. Confirm cc5 fix shipped in some `5.7.X`.
2. Bump `cyrius.cyml [package].cyrius` to that version.
3. Optionally restore `_perm_store_16(base, off, a..p)` in `lib/calc.cyr` and revert the 32-call form back to 16 calls (cosmetic — the 32-call form works fine, just slightly noisier).
4. Remove this file from `docs/development/issues/` (or move to a `closed/` subdir).
5. Update `docs/development/tool-issues.md` to drop the link.
