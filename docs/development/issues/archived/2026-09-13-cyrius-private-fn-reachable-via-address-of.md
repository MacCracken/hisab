# 2026-09-13 — cyrius: a file-private fn is reachable from another file through `&name`

**Component:** `cyrius` / `cycc` — visibility checking. A direct call and a var read ran
`_vis_check`; `&fn` was one of eight resolution paths that resolved a user identifier through
`FINDFN` without it (with `s.method()`, the retptr/pair struct receives, and four Win64-only SIMD
paths).
**Toolchain seen:** cyrius **6.6.2 and 6.6.3**; upstream: older than both, since the visibility check
had not changed shape since 6.5.38.
**Severity:** **High** — `private` is bypassable in one token. The pointer is fully callable through
`callptr` / `fncallN`, while a direct call to the same name is refused.
**Hisab impact:**
- **It invalidated the first design of hisab's reachability gate.** 3.1.0's
  `scripts/check-public-surface.sh` was to take `&name` of every public fn from a consumer file; its
  negative control, `&_private`, was supposed to be refused and was not, so `&name` proved nothing
  in either direction. The gate generates CALLS instead (arity read from the declaration,
  compile-only), for both halves, and still does.
- **It still reaches hisab's private bundle on 6.6.3**: `&_num_mulmod` compiles and runs there
  against the shipped 3.3.2 bundle (the bundle rows below; roadmap, *Decisions owed* #2, `D144`), and
  6.6.3 is the 3.x bundle's documented floor. Six direct consumers pin 6.6.3 (ghurni, garjan,
  naad, nidhi, prani, svara; read from their manifests 2026-10-01). All six are on 2.x hisab tags
  (2.11.2, 2.22.1), which predate the private bundle, so today none of them compiles a private
  bundle under 6.6.3; one that takes a 3.3.x bundle without moving its pin gets this hole. Whether
  to raise the floor to 6.6.4 is the maintainer's decision; this record does not make it.
**Status:** ✅ **CLOSED UPSTREAM — FIXED IN cycc 6.6.4** (all eight paths now check, at eleven call sites). Taken by hisab
3.1.1 (2026-09-14). Filed upstream 2026-09-13; archived there as
`cyrius/docs/development/issues/archived/2026-09-13-hisab-private-fn-reachable-via-address-of.md`,
repro at `cyrius/docs/development/issues/repros/2026-09-13-hisab-private-fn-reachable-via-address-of.cyr`.
Its closure is recorded in CHANGELOG.md [3.1.1] and the 6.6.4 entry of
`docs/development/dependency-watch.md`. hisab kept no record of its own until 3.3.3 (audit `D139`).

## Closure — the paired measurement, run 2026-10-01 for this record

Each side from a scratch dir whose `cyrius.cyml` pins the version; `lib_priv.cyr` is the library
block in the repro's header (`private`; `fn _helper(v) { return v * 2; }`; `public fn api(v) {
return _helper(v); }`). The `compiler:` line of `cyrius build -v` read `versions/6.6.3/bin/cycc` and
`versions/6.6.4/bin/cycc`.

| program (a second file including `lib_priv.cyr`) | 6.6.3 | 6.6.4 |
|---|---|---|
| the repro: `var f = &_helper; callptr(f, 21)` and `fncall1(&_helper, 21)` | builds, runs, **exit 42** — the private fn ran from another file | **refused**: `'_helper' is private to its file`, twice (one per `&`), no binary |
| control: a direct call, `return _helper(21);` | refused, no binary | refused, no binary |
| positive control: `var f = &api; return callptr(f, 21);` | builds, exit 42 | builds, exit 42 |
| hisab's own bundle (the shipped 3.3.2 `dist/hisab.cyr`): `fncall3(&_num_mulmod, 7, 9, 10)` | builds, **exit 3** (= 63 mod 10: the private helper ran) | **refused**: `'_num_mulmod' is private to its file`, no binary |
| same bundle, control: a direct `_num_mulmod(7, 9, 10)` | refused, no binary | refused, no binary |
| same bundle, positive control: `&num_modpow` (public) taken and tested non-zero | builds, exit 5 | builds, exit 5 |

## What hisab does now

- The public-surface gate's probes stay calls. A call is the shape a consumer writes, and a gate
  that only worked from 6.6.4 up would be blind on every pin below it.
- On 6.6.3 the hole remains for consumers. 3.3.0 made hisab's 6.6.3 statements say so; the
  threat model's 3.3.0 verification caveats and the roadmap's *Decisions owed* #2 carry it.
