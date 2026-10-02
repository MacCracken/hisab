# 2026-09-13 — cyrius: `cyrius build` / `cyrius deps` silently RE-LOCK a stdlib file that changed under an unchanged pin

**Component:** `cyrius` — the dependency resolver (`cbt/deps.cyr`) that `cyrius build` runs
implicitly and `cyrius deps` runs explicitly.
**Toolchain seen:** cyrius **6.6.2 and 6.6.3** (every 6.x release upstream checked).
**Severity:** **High** — the lockfile exists to detect a pinned dependency whose content moved, and
instead it was rewritten to agree with it. No diagnostic named the file or either hash, the
consumer's tracked `lib/<file>` was overwritten, and `cyrius deps --verify` then reported the new
content as verified.
**Hisab impact:** found on the 3.0.1 bump (6.6.2 → 6.6.3). A `cyrius build` in hisab under the
**unchanged** 6.6.2 pin, nothing edited, printed what a no-op prints (`1 deps resolved` /
`cyrius.lock: 31 deps locked, 1 commit-pinned`) and had rewritten tracked `lib/ganita.cyr` from
1.2.4 to 1.2.5 and moved its lock hash `d4aaa7da…` → `fae5a807…`. Only `git status` noticed. The
content came from the installed 6.6.2 snapshot, which no longer was 6.6.2's — the companion record
`2026-09-13-cyrius-refresh-only-overwrites-released-snapshot.md`.
**Status:** ✅ **CLOSED — FIXED UPSTREAM IN cyrius 6.6.4.** The lock carries a `cyrius\t<pin>`
trailer; a stdlib leaf whose snapshot hash disagrees with the locked one under an unchanged pin is
refused by name, with both hashes, and nothing is vendored or re-locked; `cyrius deps --relock` is
the explicit accept. Taken by hisab 3.1.1 (2026-09-14); hisab's `cyrius.lock` has ended with the
trailer since (`cyrius	6.6.12` today). Filed upstream 2026-09-13; archived there as
`cyrius/docs/development/issues/archived/2026-09-13-hisab-deps-relocks-silently-under-unchanged-pin.md`,
repro at `cyrius/docs/development/issues/repros/2026-09-13-hisab-deps-relocks-silently-under-unchanged-pin.sh`.
Its closure is recorded in CHANGELOG.md [3.0.1] (the finding) and [3.1.1] (the fix), and in the
6.6.3 and 6.6.4 entries of `docs/development/dependency-watch.md`. hisab kept no record of its own
until 3.3.3 (audit `D139`).

## Closure — the paired measurement, run 2026-10-01 for this record

The upstream repro script, unmodified, run as `PIN=<v> sh <script>` with `TMPDIR` pointed at a
scratch dir so its throwaway `CYRIUS_HOME` (a copy of `~/.cyrius/versions/<v>`) lands there; nothing
under `~/.cyrius` is written. It writes a lock, prepends one comment line to the throwaway
snapshot's `lib/math.cyr` with the pin unchanged, runs a plain `cyrius build`, and compares the lock
entry before and after. Its exit code is the verdict (1 = bug).

| | PIN=6.6.3 | PIN=6.6.4 |
|---|---|---|
| script exit | **1** — `BUG: lock re-written silently under an unchanged pin (verify PASSES on the mutated content)` | **0** — `OK: lock untouched under an unchanged pin` |
| `lib/math.cyr` lock hash, before → after | `a7650109…cd74e2` → `e3e18611…c53c243b` | `a7650109…cd74e2` → `a7650109…cd74e2` |
| the build's output | `1 deps resolved`, `cyrius.lock: 9 deps locked, 1 commit-pinned`, `OK (124968 bytes)` | `error: lib/math.cyr: cyrius.lock and the pinned stdlib snapshot DISAGREE under an unchanged pin 6.6.4`, both hashes, `refusing to vendor or re-lock this leaf`, `1 deps resolved, 1 errors` |

The 6.6.3 before-hash matches the one in the upstream filing, and `lib/math.cyr` is byte-identical
in the 6.6.3 and 6.6.4 snapshots (the same `a7650109…` on both sides), so the pair differs only in
the resolver.

## What hisab does now

- The lock's pin trailer makes this a hard error on hisab's own pin.
- The vendoring rule from 3.0.1 stays: on a bump, byte-compare every `lib/*.cyr` against the cyrius
  **tag** (`git -C ~/Repos/cyrius show "<pin>:lib/<f>"`), never against
  `~/.cyrius/versions/<pin>/lib`, and treat any `git status` change to `lib/` or `cyrius.lock` after
  a bare `cyrius build` as a defect to investigate (CLAUDE.md, *Toolchain*).
