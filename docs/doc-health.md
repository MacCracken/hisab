---
name: Hisab Documentation Health
description: Living state of doc currency in the hisab repo — fresh / stale / read-through / dated / tracked-issue, refreshed as docs are touched
type: state
---

# Documentation Health — hisab

> **Last refresh**: 2026-07-21 (v2.6.10) — Cyrius 6.4.66 → **6.4.69** toolchain
> bump (a clean 3-patch bump; sakshi unchanged at **2.4.6**, already the latest tag). No
> library source change — `dist/hisab.cyr` byte-identical bar the version header. Synced the
> pin across README / CLAUDE / CONTRIBUTING / overview / roadmap / dependency-watch /
> cyrius.cyml; added the CHANGELOG 2.6.10 entry, the roadmap 2.6.10 release-history row, and
> the threat-model 2026-07-21 audit row; smoke version string 2.6.9 → 2.6.10; regenerated
> `dist/hisab.cyr` (header-only diff). Re-vendored `lib/` to 6.4.69 (all 27 declared-subset
> files byte-match; transitive `result`/`atomic` already identical — no hand-refresh); the
> stdlib delta was **3 files** (`fmt` hex/non-finite-float rendering — linked by `symbolic`;
> `math` float-parse DoS hardening; an agnos-only `sys_reboot` widening). `cyrius.lock` 30
> deps, `deps --verify` 30/30; suite **957/957** across 4 suites, all gates green. Re-verified
> the 3 open tracked toolchain issues on 6.4.69 (minimal repros): interval-ident-lex **still
> live**, for-empty-clauses **still live**, cli-arg-clobber not re-tested (destructive) — no
> new fixes.
>
> **Prior refresh**: 2026-07-17 (v2.6.9) — Cyrius 6.3.11 → **6.4.66** toolchain
> bump + sakshi 2.4.2 → **2.4.6**. Infrastructure + a test-only fix. Synced the pin +
> dep versions across README / CLAUDE / CONTRIBUTING / roadmap / overview /
> dependency-watch / cyrius.cyml; added the CHANGELOG 2.6.9 entry, the roadmap 2.6.9 **and
> the missing 2.6.8** release-history rows, and the threat-model 2026-07-17 audit row.
> Re-vendored `lib/` to 6.4.66 (all 27 declared-subset files byte-match; transitive
> `result`/`atomic` already identical — no hand-refresh this bump); smoke version string
> 2.6.7 → 2.6.9. **Fixed** the pre-existing `tests/modules.tcyr` compile failure (cycc
> reserved SIMD intrinsic names — `iv_add`/`iv_sub`/`iv_mul` unusable as vars;
> renamed `iv_sum`/`iv_diff`/`iv_prod`), restoring 312/312 → **957/957** across 4 suites;
> filed `issues/2026-07-17-cyrius-interval-ident-lex.md`. Re-verified the (now 3) open
> tracked toolchain issues on 6.4.66: for-empty-clauses **still live**, interval-ident-lex
> **new/worked-around**, cli-arg-clobber not re-tested (destructive).
>
> **Prior refresh**: 2026-06-30 (v2.6.7) — Cyrius 6.2.11 → **6.3.11** toolchain
> bump + sakshi 2.1.0 → **2.4.2**. Infrastructure-only (no library source change).
> Synced the pin + dep versions across README / CLAUDE / CONTRIBUTING / roadmap /
> overview / dependency-watch; added the CHANGELOG 2.6.7 entry and the
> threat-model 2026-06-30 audit row. Vendored `lib/` re-synced to 6.3.11 (every
> stdlib file byte-matches the toolchain; `lib/result.cyr` picked up the `_die`
> agnos-portability fix); smoke version string 2.3.3 → 2.6.7. Re-verified the 2
> open tracked toolchain issues: **both still live** (for-empty-clauses confirmed
> on 6.3.11; CLI-clobber not re-tested, destructive) — no new fixes; the 3 fixed
> at the 6.2.11 bump stay archived. Prior refresh: 2026-06-15 (v2.6.6 — 6.0.14 →
> 6.2.11, ganita reorg, 3 of 5 issues fixed → archived);
> 2026-05-30 (v2.6.5). Scaffolded at v2.4.6 during the post-2.4.x
> doc sweep; the **2.5.x** CGA arc then ran (2.5.0–2.5.3 + 2.5.4 closeout with the
> new `architecture/math.md` catalogue), then the **2.6.x** diffgeo arc — 2.6.0
> (sectional) + 2.6.1 (Weyl) + 2.6.2 (transport) + 2.6.3 (deviation) + 2.6.4
> (higher forms) + **2.6.5** closeout (P(-1)/security audit + `math.md §2`).
> Suite 901 → 957. README / CLAUDE / testing / roadmap counts synced per-patch
> through the arcs. The v2.4.6
> verify-and-cleanup pass: re-ran `bench-history.sh`
> (benchmarks.md fresh again — **26** benchmarks at commit `b1165f9`); deleted
> the ad-hoc `development/tool-issues.md` catalog (file real bugs in `issues/`,
> not a random catalog); archived the **shipped** `cyrius-linalg-proposal.md` to
> `development/archive/`; fixed CONTRIBUTING currency (6.0.14, `src/` not `lib/`,
> +distlib gate); and **verified all five `issues/` filings still reproduce on
> 6.0.14** (none stale-fixed). | **Refresh cadence**: opportunistic — when a doc
> is touched, update its row + re-anchor the header date. Not periodic.
>
> **Scope**: this repo only (`hisab`) — the whole `docs/` tree plus root files
> (README, CHANGELOG, CLAUDE.md, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT,
> VERSION). The cyrius toolchain and first-party deps (sakshi) have their own
> doc-health ledgers in their own repos.
>
> **Convention**: adapted from `cyrius/docs/doc-health.md`. hisab's tree is ~23
> markdown files (vs cyrius's ~105), so the tier structure here is leaner.

This is a **ledger**, not a one-time audit. Rewrite-in-place as docs change.

---

## At a glance — inventory (last reviewed 2026-07-17, v2.6.9)

**~23 markdown files** across the repo. Bucket counts:

| Bucket | Count | What it means |
|---|---|---|
| ✅ **Fresh / current** | ~12 | README, CHANGELOG, CLAUDE.md, CONTRIBUTING, SECURITY, `architecture/overview.md`, `guides/testing.md`, the four `development/` docs (roadmap, threat-model, dependency-watch, port-audit), and `benchmarks.md` — all current to v2.4.6. |
| 🟡 **Stale — refresh in place** | 0 | `benchmarks.md` was stale; re-ran `bench-history.sh` this pass. None outstanding. |
| 🟠 **Read-through outstanding** | 0 | CONTRIBUTING refreshed; `tool-issues.md` deleted; `linalg-proposal` archived. Cleared. |
| 🔵 **Evergreen** | 1 | `CODE_OF_CONDUCT.md` — Contributor Covenant; re-read only on policy change. |
| 📅 **Dated artifact — supersede, don't edit** | 7 | `audit/2026-04-15.md`, `audit/2026-05-29.md`, `audit/2026-05-29-cga-arc-closeout.md`, `audit/2026-05-30.md`, `benchmarks-rust-v-cyrius.md` (v2.2.0), `port-audit.md` (2026-04-15 + addendum), `development/archive/cyrius-linalg-proposal.md` (shipped). |
| 🐞 **Tracked toolchain issues (live on 6.4.69)** | 3 | `development/issues/*` — the 6.2.11 bump fixed 3 (archived). 3 now live, re-confirmed on 6.4.69 (v2.6.10, minimal repros): for-empty-clauses (verified); interval-ident-lex (verified — `iv_add`/`iv_sub`/`iv_mul` are reserved cycc SIMD intrinsics, unusable as var names, worked around by rename); CLI-clobber (not re-tested, destructive). See Tier 6. |

Numbers approximate; rolls up from the per-tier tables below.

**Why now**: the doc tree had drifted (README said v2.2.2, overview said "v1.4.0 /
27 lib files") until the 2026-05-29 sweep — there was no surface tracking
*aggregate* currency. This file is that surface. The same-day verify pass closed
the scaffold's open items rather than letting them linger.

---

## Tier 1 — Structural (root)

| File | Last touched | Status | Action |
|---|---|---|---|
| `README.md` | 2026-07-21 | ✅ Fresh | v2.6.10: toolchain → 6.4.69, version → 2.6.10, hisab tag → 2.6.10. (Prior v2.6.9: sakshi 2.4.2 → 2.4.6.) |
| `CHANGELOG.md` | 2026-07-21 | ✅ Fresh | **Source of truth per CLAUDE.md.** +2.6.10 entry (toolchain 6.4.66 → 6.4.69, clean 3-patch bump; sakshi unchanged, **not** breaking). Refreshed every release. |
| `CLAUDE.md` | 2026-07-21 | ✅ Fresh | v2.6.10: toolchain/pin → 6.4.69, status line + CI-pin note. (Prior v2.6.7: pin → 6.3.11; v2.6.6: stdlib dep list → `math`+`ganita`.) |
| `VERSION` | 2026-06-30 | ✅ Fresh | Single source of truth (`2.6.7`). |
| `CONTRIBUTING.md` | 2026-07-21 | ✅ Fresh | v2.6.10: Cyrius pin → 6.4.69. (Prior v2.6.7: 6.3.11; 6.2.11; +fmt/distlib gates.) |
| `SECURITY.md` | 2026-06-30 | ✅ Fresh | v2.6.7: Supported Versions → hisab **2.6.x** (current) / 2.0–2.5 best-effort (the "(current)" label had been stale at 2.4.x since 2.5.0). (Prior v2.4.6: +MPR/collision rows, CWE-190/`mat_new` note, supply-chain.) Mirrors `threat-model.md`. |
| `CODE_OF_CONDUCT.md` | 2026-03-22 | 🔵 Evergreen | Contributor Covenant. Re-read only on policy change. |

---

## Tier 2 — Architecture (`docs/architecture/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `overview.md` | 2026-07-21 | ✅ Fresh | v2.6.10: header → v2.6.10 / cycc 6.4.69. (Prior v2.6.6: dependency-stack row → `math`+`ganita`, ganita subsumes matrix/linalg.) |
| `math.md` | 2026-05-30 | ✅ Fresh | Equation catalogue. §1 CGA (v2.5.4); **§2 differential geometry added in the v2.6.5 closeout** (curvature conventions, sectional/Weyl/Jacobi, transport, exterior algebra + references); §3 catalogue index. Earns the CLAUDE.md "math reference" slot. |

---

## Tier 3 — Operational / Development (`docs/development/`)

> `roadmap.md` is the forward-work surface (rotates every release). The rest rotate per-need.

| File | Last touched | Status | Action |
|---|---|---|---|
| `roadmap.md` | 2026-07-21 | ✅ Fresh | **Rotates every release.** v2.6.10: header toolchain → 6.4.69, Current → v2.6.10 (clean 3-patch bump), +2.6.10 Release-History row. Forward: 2.7.0 (rendering/GPU/reverse-AD), 3.0.0 (`Result<T,E>`), + Parked. |
| `threat-model.md` | 2026-07-21 | ✅ Fresh | v2.6.10: `mat_new` overflow re-verified on 6.4.69 (`ganita.cyr` byte-identical 6.4.66→6.4.69, still unguarded); +2026-07-21 audit-history entry (3-file stdlib hardening delta — `fmt`/`math`/agnos — no new surface). (Prior: 2026-07-17 row for v2.6.9; 2026-06-30 for v2.6.7.) |
| `dependency-watch.md` | 2026-07-21 | ✅ Fresh | Cyrius toolchain version-watch. Pin → 6.4.69 (+6.4.67–69 stdlib delta: `fmt`/`math` hardening, agnos `sys_reboot`); sakshi 2.4.6 (unchanged, latest). Watching: 5.7.11 (RISC-V). |
| `port-audit.md` | 2026-05-29 | 📅 Dated + addendum | 2026-04-15 Rust→Cyrius parity snapshot, preserved; 2026-05-29 status addendum records nearly all "P0 gaps" now ported. Don't rewrite the body. |

> Removed this pass: `tool-issues.md` (deleted — ad-hoc catalog; real bugs live in `issues/`), `cyrius-linalg-proposal.md` (→ `archive/`, shipped).

---

## Tier 4 — Guides (`docs/guides/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `testing.md` | 2026-05-29 | ✅ Fresh | v2.4.6 sweep: suite counts → 119/307/312/163 (901 total), **26** benchmarks (+SIMD-batch row), +collision/invariant coverage. |
| `usage.md` | — | ⚪ Not yet earned | "When earned" patterns/examples guide (CLAUDE.md). README Quick Start covers basics today; promote if onboarding needs more. |

---

## Tier 5 — Audits (`docs/audit/`)

Periodic audit reports; per-audit timestamped. **Don't refresh in place — supersede with a new dated doc.**

| File | Date | Status |
|---|---|---|
| `2026-04-15.md` | 2026-04-15 | 📅 Dated artifact — P(-1) audit (31 issues, 25 fixed) |
| `2026-05-29.md` | 2026-05-29 | 📅 Dated artifact — security/hardening audit closing the 2.4.x arc (no new vuln) |
| `2026-05-29-cga-arc-closeout.md` | 2026-05-29 | 📅 Dated artifact — 2.5.x closeout (v2.5.4): P(-1)/security review of the CGA operators + `mat_new_guarded` (posture solid) + the math.md deliverable |
| `2026-05-30.md` | 2026-05-30 | 📅 Dated artifact — 2.6.x closeout (v2.6.5): P(-1)/security review of the diffgeo curvature/transport/form functions (posture solid) + the math.md §2 deliverable |

Next periodic security audit: per CLAUDE.md, before a major release or after significant surface change. Natural next boundary is the 3.0.0 (`Result<T,E>`) cut.

---

## Tier 6 — Tracked issues (`docs/development/issues/`)

Toolchain/CLI bug filings observed from hisab's vantage. Filed against cyrius
**5.7.x**; 3 of 5 were fixed at the 6.2.11 bump and moved to `issues/archived/`.
The **2 still live** were **re-verified 2026-06-30 against the pinned 6.3.11 —
both still reproduce** (no new fixes). hisab carries workarounds for each open
one; the bugs belong upstream in cyrius.

**Open (`docs/development/issues/`)** — re-verified 2026-06-30 on 6.3.11 (prior: 2026-06-15 on 6.2.11):

| File | Filed | Status |
|---|---|---|
| `2026-04-26-cyrius-cli-arg-clobbers-source.md` | 2026-04-26 | 🐞 **Live (presumed)** — not re-tested on 6.3.11 (destructive: overwrites source). Workaround: `CYRIUS_VERBOSE=1` env, never an unknown flag before a subcommand. |
| `2026-04-26-cyrius-for-empty-clauses.md` | 2026-04-26 | 🐞 **Live** — re-confirmed on 6.3.11: `for (; cond;)` → parser `unexpected ';'`; `for (init; cond;)` → `expected '=', got '{'`. Workaround: `while` loops in collision_core/mesh. |

**Archived — fixed on 6.2.11 (`docs/development/issues/archived/`)**:

| File | Filed | Resolution (verified 2026-06-15) |
|---|---|---|
| `2026-04-26-cbt-modules-substring-false-positive.md` | 2026-04-26 | ✅ Fixed — a `[build]` comment with the token `modules` + a `[lib] modules` array no longer mis-binds; build compiles only the source. |
| `2026-04-26-cc5-18-arg-fn-scrambles-params.md` | 2026-04-26 | ✅ Fixed — `f18(1..18)` of params {1,2,7,8,11,12,18} → exit 59 (exact); SysV scramble gone. `src/calc.cyr`'s 10-arg split left in place (optional revert). |
| `2026-04-26-cyrius-lint-rc-as-warning-count.md` | 2026-04-26 | ✅ Fixed — 2-warning file now exits 0 while still printing `  warn …` lines; CI's `^\s*warn ` stdout grep remains the load-bearing gate. |

---

## Tier 7 — Benchmarks

| File | Last touched | Status | Action |
|---|---|---|---|
| `benchmarks.md` | 2026-05-29 | ✅ Fresh | **Re-run this pass** via `scripts/bench-history.sh` — now reflects commit `b1165f9` (v2.4.6), 26 benchmarks; appended a `bench-history.csv` row. Cadence: every release closeout with perf-relevant change. |
| `benchmarks-rust-v-cyrius.md` | 2026-04-15 | 📅 Dated artifact | Rust vs Cyrius comparison, bounded to **v2.2.0**. Supersede only if a port-parity question resurfaces. |

---

## Tier 8 — Archive (`docs/development/archive/`)

Frozen-by-design. Shipped proposals / superseded docs land here (don't edit; re-open is a `git mv` back).

| File | Status |
|---|---|
| `cyrius-linalg-proposal.md` | 📦 Shipped — `linalg` is now a cyrius stdlib module (shipped cyrius 4.10.2/4.10.3 per the doc's own header). Archived 2026-05-29. |

---

## Refresh procedure

When docs are touched:

1. Find the affected row in the relevant tier table.
2. Update **Last touched** to the new date.
3. Update **Status** if the bucket changed.
4. Update **Action** if the next step changed.
5. If a doc moved or was archived/deleted, update its row (and the removed-this-pass note).
6. Re-anchor the **Last refresh** date in the header.

When the bucket counts at the top drift by more than ~2 in any cell, refresh the at-a-glance table. Cadence is **opportunistic**, not periodic.

---

## What this file is NOT

- Not a CHANGELOG (which records what shipped, not what's stale).
- Not a TODO list (forward work lives in [`development/roadmap.md`](development/roadmap.md)).
- Not a per-doc review log (this is the ledger of where each doc stands, not the reasoning behind each).
- Not a substitute for the audit reports in `docs/audit/` (point-in-time security/correctness snapshots).

---

## Forward doc-policy commitments

Scheduled doc decisions, surfaced so they aren't forgotten when the trigger arrives.

| # | Commitment | Trigger | Notes |
|---|---|---|---|
| 1 | **Roadmap + Release History rotate every release** — completed arcs move out of active-work into the Release History table at closeout. | Every release | Per CLAUDE.md Work Loop §10–11. Last done in the v2.4.6 sweep. |
| 2 | **Re-run `bench-history.sh` at release closeout** — keep `benchmarks.md` + `bench-history.csv` current; "numbers don't lie." | Every release with perf-relevant change | ✅ Done 2026-05-29 (was stale at `8a08c99`; now `b1165f9`). |
| 3 | **Re-verify tracked `issues/` filings at each toolchain bump** — re-test on the new pin; move resolved ones to `issues/archived/`. | Each `cyrius.cyml` pin bump | ✅ Done 2026-06-30 on 6.3.11 — both open issues still live (for-empty-clauses confirmed; CLI-clobber not re-tested), no new fixes. Prior: 2026-06-15 on 6.2.11 (3 of 5 fixed and archived); 2026-05-29 on 6.0.14 (all 5 live). |
| 4 | **Periodic security audit** — full source scan before a major release or after significant surface change; supersede with a new dated `docs/audit/` doc. | Before 3.0.0; on significant change | Last: 2026-04-15 + 2026-05-29. |

---

*Initial scaffold: 2026-05-29 (v2.4.6), adapted from `cyrius/docs/doc-health.md`, immediately after the post-2.4.x documentation sweep. Same-day verify-and-cleanup pass: bench re-run, `tool-issues.md` retired, linalg proposal archived, CONTRIBUTING currency fixed, all 5 toolchain issues re-verified live on 6.0.14. Refresh in place when docs are touched.*
