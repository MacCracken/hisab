# 2026-04-26 — cbt manifest parser substring-matches `modules` inside `[build]` comments

**Component:** `cbt/deps.cyr` `_read_build_modules` (~L540)
**Toolchain seen:** cyrius 5.7.7 → 5.7.10
**Severity:** Silent miscompile — every `[lib]` module gets auto-prepended to every build, blowing past `input_buf` and surfacing pre-existing parse errors in modules not meant to be in the build chain. Misleading because the error blames a module the user didn't include.
**Hisab impact:** Spent significant time hunting a phantom `lib/collision_core.cyr:223 unexpected struct` error before realizing that `cbt` was prepending the entire `[lib]` list to `cyrius build src/main.cyr`. Comment containing the literal token `modules` was the trigger.
**Hisab workaround:** A banner comment at the top of `[build]` in `cyrius.cyml` warns about this and uses split-token (`m`+`odules`) to dodge the same trap. See `cyrius.cyml`.
**Status:** Open upstream. Same false-positive class as the `stdlib` substring bug already guarded against in `cbt/deps.cyr` ~L170 (added v5.5.26 when patra hit it).

## Symptom

A comment inside the `[build]` section that contains the literal token `modules` causes `cbt` to grab the next TOML array it sees as `[build].modules` — even if that array belongs to a downstream section like `[lib]`.

Result: every file in `[lib]` gets auto-prepended to every `cyrius build`, including modules the project's own `src/main.cyr` doesn't `include`. Errors then get blamed on those auto-prepended modules.

## Self-contained reproducer

```toml
# /tmp/cbt_repro/cyrius.cyml
[package]
name = "repro"
version = "0.0.1"
language = "cyrius"
cyrius = "5.7.10"

[build]
src = "src/main.cyr"
output = "build/repro"

# Documentation: pull this dep via [deps.foo] modules = [...]
# ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
#                                 the literal token `modules`
#                                 above is the bug trigger

[lib]
modules = ["lib/intentionally_broken.cyr"]   # auto-prepended to every build

[deps]
stdlib = ["syscalls"]
```

```cyrius
// /tmp/cbt_repro/lib/intentionally_broken.cyr — never included by main.cyr
this is not valid cyrius code at all
```

```cyrius
// /tmp/cbt_repro/src/main.cyr
include "lib/syscalls.cyr"
fn main() { return 0; }
var r = main();
syscall(60, r);
```

```bash
cd /tmp/cbt_repro && cyrius build src/main.cyr build/repro 2>&1 | head -3
# Expected: build OK (main.cyr is well-formed and doesn't touch the broken file)
# Actual:   error in lib/intentionally_broken.cyr (auto-prepended from [lib])
```

Removing the word "modules" from the comment makes the build pass.

## Where to look

`cbt/deps.cyr` `_read_build_modules` walks `[build]` body line-by-line:

```cyr
while (bi < n) {
    if (load8(buf + bi) == 91 && load8(buf + bi + 1) != 91) { break; }   # next section
    if (memeq(buf + bi, "modules", 7) == 1) {
        _build_modules = _parse_toml_str_array(buf, n, bi);
        break;
    }
    while (bi < n && load8(buf + bi) != 10) { bi = bi + 1; }
    bi = bi + 1;
}
```

The check `if (memeq(buf + bi, "modules", 7) == 1)` runs even when `bi` points at a comment line starting with `#`. The `stdlib` key has an explicit comment-line guard (~L186-200 of the same file) that walks back to the line start and checks for `#` before treating the substring as a key — that guard needs to be applied to the `modules` key too.

## Verification once a fix lands

The reproducer above should build cleanly with the literal token `modules` present in a `[build]`-section comment.

## Hisab follow-up after upstream fix

1. Drop the `DO NOT write the word m+odules in this comment block` banner from `cyrius.cyml`.
2. Reword the `[build]` section comment to its natural form.
3. Remove this file from `docs/development/issues/`.

No source changes needed beyond manifest cleanup.
