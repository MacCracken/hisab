# Migrating to hisab 3.0.0

> 3.0.0 replaces the integer-error-code convention with the Cyrius stdlib
> `Result<T, E>`. **48 functions change signature.** Nothing else about their
> behaviour changes: the same computations, the same out-parameters, the same
> `HSB_ERR_*` taxonomy — only the channel the error arrives on.

## TL;DR

```cyrius
# 2.x
var rc = calc_integral_simpson(&f, a, b, steps, out);
if (rc != HSB_ERR_NONE) { … }

# 3.0
var tag, val = calc_integral_simpson(&f, a, b, steps, out);
if (is_err_result(tag) == 1) { … }        # val holds the HSB_ERR_* code
```

Add `"result"` to your `[deps] stdlib` list. `cyrius deps` picks it up from
`dist/hisab.deps` automatically, which now names 16 leaves rather than 15.

## ⛔ The one thing that will bite you

**A `Result` in argument position does not error — it silently degrades to its tag.**

`rdx` never reaches a parameter, so a one-argument callee receives only the tag.
Measured on cycc 6.6.2: `Ok` tag = **0**, `Err` tag = **1**, and `HSB_ERR_NONE` = **0**.

That means this **keeps compiling and keeps passing while testing nothing**:

```cyrius
assert_eq(calc_integral_simpson(&f, a, b, steps, out), HSB_ERR_NONE, "ok");   # ⛔ assert_eq(0, 0)
```

whereas an error-path check fails loudly, because the tag `1` never equals a negative code:

```cyrius
assert_eq(calc_integral_simpson(&f, a, b, 0, out), HSB_ERR_ZERO_STEPS, "…");  # fails: 1 != -12
```

**So the failure mode is asymmetric, and the silent half is the success path.** In hisab's own
suites, 98 lines compared a fallible call against `HSB_ERR_NONE` and would have gone vacuous
without a word.

⚠ `lib/result.cyr`'s header says the value form's diagnostic "is the migration tool: every stale
site fails at its own line instead of miscompiling." That is true for **binding** sites — `var r =
f();` is a hard error naming the fix — and **false for argument sites**. Do not rely on the
compiler alone.

⭐ **Drive your migration with a grep, not with build errors.** For every hisab function you call,
find every call site and convert it. `scripts/check-result-migration.sh` in this repo does exactly
that and is available to copy.

## The four call shapes

| 2.x | 3.0 |
|---|---|
| `var rc = f(…); if (rc != HSB_ERR_NONE) {` | `var t, v = f(…); if (is_err_result(t) == 1) {` |
| `assert_eq(f(…), HSB_ERR_NONE, m)` | `var t, v = f(…); assert_eq(is_ok(t), 1, m)` |
| `assert_eq(f(…), HSB_ERR_X, m)` | `var t, v = f(…); assert_eq(v, HSB_ERR_X, m)` |
| `f(…);` (discarding) | `var t, v = f(…);` |

For a function that returns a **value** rather than a status — `num_modpow`,
`halfedge_is_boundary` — the old return becomes the **payload**, not the tag:

```cyrius
var t, v = num_modpow(2, 10, 1000);
assert_eq(is_ok(t), 1, "…");
assert_eq(v, 24, "…");                 # v is the answer; t is only ok/err
```

## Forwarding chains become `?`

```cyrius
# 2.x
var err = num_fft(data, n);
if (err != 0) { return err; }

# 3.0
var err = num_fft(data, n)?;
```

⚠ `?` requires a **binding**; it cannot be used on a reassignment (`err = f(…)?`). Introduce a new
name. And a tail call needs nothing at all — `return g(…);` already forwards both halves.

## Deprecation window

There is none, and that is deliberate.

A dual API (`f` returning a code beside `f_r` returning a `Result`) was considered and rejected:
it doubles the surface, and — because of the silent argument-position degradation above — it would
leave every existing `assert_eq(f(…), HSB_ERR_NONE)` call site **quietly passing against the old
function forever**, which is the exact failure this release exists to remove. A hard break at a
major, with this guide, is the honest version.

**2.24.0 remains the supported 2.x line.** It is the last release before the signature change, and
every 2.x fix through 2.24.0 is in it.

## What did NOT change

- Out-parameters. Every function that wrote its result through a pointer still does.
- The `HSB_ERR_*` codes and their meanings — they are now the `E` payload.
- Anything non-fallible. 891 of hisab's 939 functions are untouched.
- `#must_use`, which still applies and still has its own CI gate.
