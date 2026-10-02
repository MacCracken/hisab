# 2026-09-13 — cyrius: `install.sh --refresh-only` writes the repo's in-progress `lib/` into an ALREADY-RELEASED version's snapshot

**Component:** `cyrius` — `scripts/install.sh --refresh-only`, cyrius's dev-loop refresh, which
keyed its destination on the working tree's `VERSION` and never asked whether that version was
already cut.
**Toolchain seen:** the cyrius tree at **6.6.3** (upstream: every release).
**Severity:** **High** — `~/.cyrius/versions/<v>/lib` is what a pin means, and on a box that runs
the cyrius dev loop it silently became the NEXT release's stdlib under the old release's name. From
a consumer's side the mutation is invisible.
**Hisab impact:** found on the 3.0.1 bump. hisab's docs said ganita **1.2.4** under its 6.6.2 pin,
its committed `lib/ganita.cyr` agreed, and so did the cyrius **tag** 6.6.2. But the installed
`~/.cyrius/versions/6.6.2/lib` held 6.6.3's twelve refolded stdlib files (`ganita.cyr` mtime
2026-09-12 08:49, two days after 6.6.2 shipped). Combined with the companion defect
(`2026-09-13-cyrius-deps-relocks-silently-under-unchanged-pin.md`), a bare `cyrius build` under the
unchanged pin re-vendored and re-locked ganita 1.2.5. The check hisab had used since 2.6.11,
"byte-compare `lib/` against the pin's own snapshot", was comparing against the mutated copy.
**Status:** ✅ **CLOSED — FIXED UPSTREAM IN cyrius 6.6.4.** `--refresh-only` refuses to write a
released version's slot from a tree that has drifted from its tag when the destination is live
(the slot exists, or the home is `$HOME/.cyrius`); every refresh stamps the slot with
`SOURCE_COMMIT` and `tree-matches-tag: yes|no`, and `scripts/verify-store.sh` audits and restores a
slot from its tag. Taken by hisab 3.1.1 (2026-09-14). Filed upstream 2026-09-13; archived there as
`cyrius/docs/development/issues/archived/2026-09-13-hisab-refresh-only-overwrites-released-snapshot.md`
(its repro is inline in that filing; there is no file under `repros/`). Its closure is recorded in
CHANGELOG.md [3.0.1] (the finding) and [3.1.1] (the fix), and in the 6.6.3 and 6.6.4 entries of
`docs/development/dependency-watch.md`. hisab kept no record of its own until 3.3.3 (audit `D139`).

## Closure — the paired measurement, run 2026-10-01 for this record

This is a script in the cyrius repo, not a compiler, so "pinned to a version" means the script at
each tag: a scratch clone of `~/Repos/cyrius` (read-only to the original), checked out at the tag,
and a throwaway `CYRIUS_HOME` in a scratch dir. `~/.cyrius` is only read, to seed the throwaway
slot.

⚠ **The upstream repro as written cannot tell the two versions apart.** It refreshes into an EMPTY
throwaway home, and 6.6.4 lets that through on purpose (a throwaway never held the release). Run
that way from a drifted tree, both tags exit 0 and write the drifted `lib/math.cyr` into the new
`versions/<v>/lib`; 6.6.4 only adds a stamp, `dirty` / `tree-matches-tag: no`. So the pair below
makes the destination LIVE, the defect's real shape: the throwaway home's `versions/<v>` is a copy
of the installed slot (its `lib/math.cyr` hashes `a7650109…`, the same as `git show
<v>:lib/math.cyr` at both tags), and the clone is drifted by appending one comment line to
`lib/math.cyr` (hash `fe4cba3c…`).

| `CYRIUS_HOME=<throwaway> sh scripts/install.sh --refresh-only` | tree at tag 6.6.3, drifted | tree at tag 6.6.4, drifted | control: tag 6.6.4, NOT drifted |
|---|---|---|---|
| exit | **0** | **1** | 0 |
| what it printed | `refreshed 23 bins/scripts + 110 stdlib files` | `refusing --refresh-only: VERSION=6.6.4 is a CUT RELEASE (its tag exists) and the tree has moved past it`, then the three ways forward | `refreshed 23 bins/scripts + 110 stdlib files` |
| the live slot's `lib/math.cyr` afterwards | **`fe4cba3c…` — the drifted file, under the released name** | `a7650109…`, untouched | `a7650109…` (the tag's own bytes), stamped `tree-matches-tag: yes` |

⚠ In the 6.6.3 column the slot's `SOURCE_COMMIT` still reads `tree-matches-tag: yes` afterwards: it
is the copied installed slot's stamp, and the 6.6.3 script writes none, so after the overwrite the
stamp is false.

## What hisab does now

- **The cyrius tag is the reference, never the install dir.** Every bump byte-compares `lib/*.cyr`
  against `git -C ~/Repos/cyrius show "<pin>:lib/<f>"` and `sakshi.cyr` against its tag's `dist/`
  (CLAUDE.md, *Toolchain*). The rule outlives the fix: it is what found both 2026-09-13 store
  defects, and a store can still be written by hand.
- `scripts/verify-store.sh` in the cyrius repo can audit a slot against its tag. On this box the
  6.6.3 and 6.6.4 slots both carry `SOURCE_COMMIT` stamps saying `tree-matches-tag: yes` (read
  2026-10-01).
