---
name: Hisab Documentation Health
description: Living state of doc currency in the hisab repo — fresh / stale / read-through / dated / tracked-issue, refreshed as docs are touched
type: state
---

# Documentation Health — hisab

> **Last refresh**: 2026-05-29 (v2.5.2). Scaffolded at v2.4.6 during the post-2.4.x
> doc sweep; the **2.5.x** CGA arc opened the same day — 2.5.0 (contraction) +
> 2.5.1 (dual) + 2.5.2 (projection/rejection) shipped (suite 901 → 925; only the
> 2.5.3 `mat_new` guard closeout remains), with README / CLAUDE / testing / roadmap
> current-state counts kept synced per-patch through the arc. The v2.4.6
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

## At a glance — inventory (last reviewed 2026-05-29, v2.5.2)

**~23 markdown files** across the repo. Bucket counts:

| Bucket | Count | What it means |
|---|---|---|
| ✅ **Fresh / current** | ~12 | README, CHANGELOG, CLAUDE.md, CONTRIBUTING, SECURITY, `architecture/overview.md`, `guides/testing.md`, the four `development/` docs (roadmap, threat-model, dependency-watch, port-audit), and `benchmarks.md` — all current to v2.4.6. |
| 🟡 **Stale — refresh in place** | 0 | `benchmarks.md` was stale; re-ran `bench-history.sh` this pass. None outstanding. |
| 🟠 **Read-through outstanding** | 0 | CONTRIBUTING refreshed; `tool-issues.md` deleted; `linalg-proposal` archived. Cleared. |
| 🔵 **Evergreen** | 1 | `CODE_OF_CONDUCT.md` — Contributor Covenant; re-read only on policy change. |
| 📅 **Dated artifact — supersede, don't edit** | 5 | `audit/2026-04-15.md`, `audit/2026-05-29.md`, `benchmarks-rust-v-cyrius.md` (v2.2.0), `port-audit.md` (2026-04-15 + addendum), `development/archive/cyrius-linalg-proposal.md` (shipped). |
| 🐞 **Tracked toolchain issues (live on 6.0.14)** | 5 | `development/issues/*` — all five cyrius/cbt bugs **re-verified still reproducing** on the pinned 6.0.14 this pass (see Tier 6). |

Numbers approximate; rolls up from the per-tier tables below.

**Why now**: the doc tree had drifted (README said v2.2.2, overview said "v1.4.0 /
27 lib files") until the 2026-05-29 sweep — there was no surface tracking
*aggregate* currency. This file is that surface. The same-day verify pass closed
the scaffold's open items rather than letting them linger.

---

## Tier 1 — Structural (root)

| File | Last touched | Status | Action |
|---|---|---|---|
| `README.md` | 2026-05-29 | ✅ Fresh | v2.4.6 sweep: +Collision module row + spatial/einsum/lie_ext/linalg_precision/noise_simplex; version→2.4.6, counts→901/34, 26 benches, binary→~152 KB. |
| `CHANGELOG.md` | 2026-05-29 | ✅ Fresh | **Source of truth per CLAUDE.md.** Through v2.4.6 + an `[Unreleased]` Documentation entry. Refreshed every release. |
| `CLAUDE.md` | 2026-05-29 | ✅ Fresh | Process + principles + identity. Status line → 2.4.6 / 16,446-line bundle / 901 tests; +doc-health.md pointer in the docs-structure list. |
| `VERSION` | 2026-05-29 | ✅ Fresh | Single source of truth (`2.4.6`); bumped via `scripts/version-bump.sh`. |
| `CONTRIBUTING.md` | 2026-05-29 | ✅ Fresh | **Refreshed this pass**: Cyrius pin 5.7.10 → 6.0.14; "Adding a Module" corrected (`src/` not `lib/`, self-contained sources, `[lib] modules` + `cyrius distlib`); +fmt/distlib gates in the check suite. |
| `SECURITY.md` | 2026-05-29 | ✅ Fresh | v2.4.6 sweep: +MPR/collision rows, CWE-190/`mat_new` note, supply-chain, supported-versions → hisab 2.4.x. Mirrors `threat-model.md`. |
| `CODE_OF_CONDUCT.md` | 2026-03-22 | 🔵 Evergreen | Contributor Covenant. Re-read only on policy change. |

---

## Tier 2 — Architecture (`docs/architecture/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `overview.md` | 2026-05-29 | ✅ Fresh | v2.4.6 sweep: header → v2.4.6 / 34 modules; module map completed (+Collision section); collision data-flow shows MPR + sequential-impulse. |
| `math.md` | — | ⚪ Not yet earned | "When applicable" math reference (CLAUDE.md). Create when a non-obvious derivation needs a home (e.g. MPR portal refinement, in-circle predicate). |

---

## Tier 3 — Operational / Development (`docs/development/`)

> `roadmap.md` is the forward-work surface (rotates every release). The rest rotate per-need.

| File | Last touched | Status | Action |
|---|---|---|---|
| `roadmap.md` | 2026-05-29 | ✅ Fresh | **Rotates every release.** Pruned to forward-facing only (completed arcs → Release History). Open: 2.5.x arc (CGA depth + `mat_new` guard), 2.6.0 (diffgeo), 2.7.0 (rendering/GPU/reverse-AD), 3.0.0 (`Result<T,E>`), + Parked. |
| `threat-model.md` | 2026-05-29 | ✅ Fresh | v2.4.6 sweep: +six collision algorithms, `mat_new` note → 6.0.14, +Supply Chain, +2026-05-29 audit-history entry. |
| `dependency-watch.md` | 2026-05-29 | ✅ Fresh | Cyrius toolchain version-watch. Build/test marker → 901/901 @ v2.4.6. Watching: 5.7.11 (RISC-V). |
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

Next periodic security audit: per CLAUDE.md, before a major release or after significant surface change. Natural next boundary is the 3.0.0 (`Result<T,E>`) cut.

---

## Tier 6 — Tracked issues (`docs/development/issues/`)

Toolchain/CLI bug filings observed from hisab's vantage. Filed against cyrius
**5.7.x**; **re-verified 2026-05-29 against the pinned 6.0.14 — all five still
reproduce** (none stale-fixed). hisab carries workarounds for each; the bugs
belong upstream in cyrius. No `issues/archived/` yet (nothing resolved to move).

| File | Filed | Status (verified 2026-05-29 on 6.0.14) |
|---|---|---|
| `2026-04-26-cyrius-cli-arg-clobbers-source.md` | 2026-04-26 | 🐞 **Live (destructive)** — `cyrius -v build src out` overwrote the source file with the 155 KB binary. Workaround: `CYRIUS_VERBOSE=1` env, never an unknown flag. |
| `2026-04-26-cc5-18-arg-fn-scrambles-params.md` | 2026-04-26 | 🐞 **Live** — 18-arg fn sum returned 180 vs expected 171 (params scrambled). Workaround: keep fns ≤ ~10 args (hisab's widest is `contact_new`, 8). |
| `2026-04-26-cyrius-lint-rc-as-warning-count.md` | 2026-04-26 | 🐞 **Live** — 2-warning file → exit code 2 (rc = warning count). Workaround: CI uses `set +e` + per-file rc capture. |
| `2026-04-26-cyrius-for-empty-clauses.md` | 2026-04-26 | 🐞 **Live** — `for (; cond;)` → parser `unexpected ';'`. Workaround: `while` loops in collision_core/mesh. |
| `2026-04-26-cbt-modules-substring-false-positive.md` | 2026-04-26 | 🐞 Open — manifest-parser `modules` substring grab. Not isolation-tested this pass; workaround (banner + split-token comments) in `cyrius.cyml`. |

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
| 3 | **Re-verify tracked `issues/` filings at each toolchain bump** — re-test on the new pin; `git mv` resolved ones to `issues/archived/`. | Each `cyrius.cyml` pin bump | ✅ Done 2026-05-29 on 6.0.14 — all 5 still live, none moved. |
| 4 | **Periodic security audit** — full source scan before a major release or after significant surface change; supersede with a new dated `docs/audit/` doc. | Before 3.0.0; on significant change | Last: 2026-04-15 + 2026-05-29. |

---

*Initial scaffold: 2026-05-29 (v2.4.6), adapted from `cyrius/docs/doc-health.md`, immediately after the post-2.4.x documentation sweep. Same-day verify-and-cleanup pass: bench re-run, `tool-issues.md` retired, linalg proposal archived, CONTRIBUTING currency fixed, all 5 toolchain issues re-verified live on 6.0.14. Refresh in place when docs are touched.*
