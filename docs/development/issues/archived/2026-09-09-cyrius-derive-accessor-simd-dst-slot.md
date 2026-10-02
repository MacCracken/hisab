# 2026-09-09 — cyrius: an `f64v_*` intrinsic reads its DESTINATION pointer from a frame slot nothing writes (wrong code)

**Component:** `cyrius` / `cycc` — the SIMD intrinsic handlers in `src/frontend/parse_expr.cyr`
(`f64v_*`, `f32v_*`, `f32v8_*`, `f64v256_*`, `iv_*`), which stash their operands in frame slots.
**Toolchain seen:** cyrius **6.6.1**, on hisab's 6.5.33 → 6.6.1 bump (hisab 2.11.3).
**Severity:** **Critical** — wrong code. With the register picker on (the default) the packed store
goes through a garbage pointer and usually SIGSEGVs. With the picker off it exits 0 and writes the
result into the argument object instead.
**Hisab impact:** `m4_mul_vec4` (`src/mat4.cyr`) passed `HVec4_x(v)` straight into `f64v_scale` and
SIGSEGV'd on every call; three of the five suites died at rc=139 before reaching a verdict, which
blocked the bump. 2.11.3 shipped with the getters hoisted into locals (`var vx = HVec4_x(v);`), the
form `m3_mul_vec3` already used.
**Status:** ✅ **CLOSED — FIXED UPSTREAM IN cycc 6.6.2** (one `_SIMD_RESERVE` helper shared by all
21 handlers). Taken by hisab 2.11.5 (2026-09-09). Filed upstream 2026-09-09; archived there as
`cyrius/docs/development/issues/archived/2026-09-09-hisab-derive-accessor-simd-dst-slot.md`, repro
at `cyrius/docs/development/issues/repros/2026-09-09-derive-accessor-simd-dst-slot.cyr`. Its
closure is recorded in CHANGELOG.md [2.11.5] and in the 6.6.2 entry of
`docs/development/dependency-watch.md`. hisab kept no record of its own until 3.3.3 (audit `D139`).

## What hisab filed, and what upstream found

hisab filed it as a `#derive(accessors)` defect and a 6.5.71 regression (6.5.70 clean, 6.5.71
broken). Upstream corrected the scope three ways, all accepted:

1. **Not derive-specific.** Every one of the 21 handlers took `var vbase = GFLC(S)` and did not raise
   `GFLC` until all arguments were parsed, so ANY argument that allocated a frame local was bound to
   the intrinsic's own destination slot. `#derive` getters were one route; `#inline` (6.5.63) and
   `callptr` (6.0.70) were others.
2. **Not a 6.5.71 regression.** That release put derived getters on the inline-replay path, which
   removed a real `call` that had been forcing the spill by accident. The bisect found the release
   that exposed the defect, not the one that caused it.
3. **The SIGSEGV was the lucky half.** With `CYRIUS_REGALLOC_PICKER_CAP=0` the same program runs to
   completion with a wrong answer. (Upstream later found that the picker-on crash also needed a
   register-picker classification hole, fixed in 6.6.5; that hole is hisab's 2026-09-14 record,
   `2026-09-14-cyrius-simd-dst-slot-regalloc-picker.md`, beside this one.)

## Closure — the paired measurement, re-run 2026-10-01 for this record

Each side from a scratch dir whose `cyrius.cyml` pins the version, `[deps] stdlib = ["syscalls",
"alloc", "io", "fmt", "string", "str"]`, `cyrius lib sync`, then `cyrius build -v`; the `compiler:`
line read `versions/6.6.1/bin/cycc` and `versions/6.6.2/bin/cycc` respectively.

| probe | 6.6.1 | 6.6.2 |
|---|---|---|
| upstream repro, verbatim, picker on (default) | W1/W2/W4 ok, W3 **SIGSEGV, exit 139** | all four ok, **exit 0** |
| upstream repro, verbatim, built with `CYRIUS_REGALLOC_PICKER_CAP=0` | exit **0** — ⚠ the repro checks only for the crash, so it cannot see the silent mode | exit 0 |
| repro + a value check after W3 (r = 2m = (2, 4, 6, 8), the argument v still (2, 0, 0, 0)), picker on | exit **139** | exit **0** |
| same, picker off | exit **17**: 7 of the 8 words wrong — r is (0, 0, 0, 0), never written, and v reads (2, 4, 6, 8): the result went into the argument object | exit **0** |

The value check is this record's addition, not upstream's file: the call becomes `var r3 = w3(m,
v);`, eight comparisons (r3's four words, v's four) follow it, and the program returns 10 + the
number wrong when any is wrong, or 0 when none is. v's first word is right by coincidence (2 = 2 x
1). The picker-off build differs from the picker-on build on both toolchains (`cmp` reports a
difference, for the verbatim repro and for the value-check variant), so the knob reached the
compiler.

## What hisab does now

- `m4_mul_vec4` keeps the hoist. 2.11.5 verified it is no longer load-bearing on 6.6.2 (the three
  affected suites pass with it removed: hisab 416, foundation 351, abuse 741, rc=0 each), and the
  comment on the function says it is kept for consistency with `m3_mul_vec3`, not because it is
  required.
- hisab ≥ 3.1.0 requires cyrius ≥ 6.6.3 (`public struct` + `#derive`), so every consumer of a 3.x
  bundle compiles past this fix.
- The rule this filing produced is in CLAUDE.md: **a first-bad-version is evidence about
  visibility, not origin.**
