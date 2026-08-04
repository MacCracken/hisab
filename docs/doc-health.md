---
name: Hisab Documentation Health
description: Living state of doc currency in the hisab repo — fresh / stale / read-through / dated / tracked-issue, refreshed as docs are touched
type: state
---

# Documentation Health — hisab

> **Last refresh**: 2026-08-03 (v2.6.15) — the 2026-08-03 audit is fully discharged; see
> the fifth- through third-pass blocks below for the repair sequence. The v2.6.11 record follows.
>
> **v2.6.11**: Cyrius 6.4.69 → **6.5.6** toolchain bump (a
> **minor** jump across 24 releases) + sakshi 2.4.6 → **2.4.7**. No executable library change —
> the `dist/hisab.cyr` diff is the version header plus one `mat_new_guarded` doc comment, zero
> code lines (16,878 → 16,885). Synced the pin/version across README /
> CLAUDE / CONTRIBUTING / overview / roadmap / dependency-watch / threat-model / SECURITY /
> cyrius.cyml; added the CHANGELOG 2.6.11 entry, the roadmap 2.6.11 release-history row, and the
> threat-model 2026-08-03 audit row; smoke version string 2.6.10 → 2.6.11; regenerated
> `dist/hisab.cyr` (header-only diff). **This bump was not routine.** Re-vendoring surfaced that
> `lib/ganita.cyr` had been stale at **1.0.3** while the *previous* 6.4.69 pin already shipped
> **1.0.4** — so hisab built the whole 2.6.10 cycle against a `mat_new` that **segfaults** on a
> negative dimension (measured: exit 139 on 1.0.3, null on 1.0.4, compiler held constant). That
> closes the roadmap's tracked "stdlib `mat_new` overflow guard" item (open since 2.5.3), flips
> the threat-model row from *open* to *resolved*, and corrects SECURITY.md. **Process change:**
> vendoring is now byte-checked `lib/` **against the pin's snapshot**, not previous-pin against
> new-pin — the latter is what hid this for three releases. Two further defensive fixes: all four
> `.tcyr` harnesses exited with `assert_summary()`'s raw failure count, so exactly 256/512/768
> failures scored **PASS** (demonstrated, then clamped); and the `sys_exit_group` epilogue
> replaced `syscall(SYS_EXIT, …)` repo-wide. `cyrius fuzz` now walks `tests/`, so
> `tests/hisab.fcyr` ran for the first time (1/1). Suite **961/961** (edge_cases 163 → 167: +4
> assertions pinning the upstream `mat_new` contract, mutation-proven). `cyrius.lock` 30 deps,
> `deps --verify` 30/30. Re-verified the 3 open tracked toolchain issues on 6.5.6: interval-ident-lex
> **still live**, for-empty-clauses **still live**, cli-arg-clobber not re-tested (destructive).
>
> **Fifth pass (2026-08-03, v2.6.15) — THE AUDIT IS DISCHARGED.** P3 (performance) and P4
> (documentation) closed, and the last two never-tested modules (`num_ext`, `symbolic_ext`)
> brought in: **all 34 shipped modules are now included by a test suite**, where eight were not
> even compiled by `cyrius test` when the audit opened. Both O(n^2) hot paths were benchmarked
> BEFORE the rewrite and both rows are in `bench-history.csv`: halfedge twin pairing
> 190.1 ms -> **1.2 ms**, convex-hull pre-sort 22.1 ms -> **2.1 ms**, each verified
> output-identical to the old code. Six documentation drifts corrected across `overview.md`,
> `math.md`, `optimize.cyr`, `num_ext.cyr`, `symbolic.cyr`, `diffgeo.cyr` and
> `collision_core.cyr` — including a module map that credited BVH to the wrong file and a
> `sign` parameter documented three contradictory ways. Suite 1093 -> **1127**; benchmarks
> 26 -> **28**; coverage 57% -> **59%**, files 34/35. **Next: a re-audit** to confirm the
> repairs (doc-health forward-commitment #4), then the 2.7.0 items.
>
> **Fourth pass (2026-08-03, v2.6.14):** audit **P1 — the memory-safety tier — is closed**.
> Capped-constructor null propagation (`cmat_*`), solver dimension caps + `HSB_ERR_ALLOC`,
> B-spline negative indexing, the adaptive-Simpson work bound, `kdtree_build`'s O(n) recursion on
> coincident points (SIGSEGV at 60k), the Armijo NaN acceptance, the complex 1e-6-vs-1e-12
> tolerance, `FLOAT_RENDER_BUF`, and four fBm NaN returns. **Three of the four defect groups
> crashed the process pre-fix.** `calc_ext` and `noise_simplex` brought into the suites, leaving
> only `num_ext` and `symbolic_ext`. Suite 1063 → **1093**; coverage 54% → **57%**, files 32/35.
> Counts synced across README / CLAUDE / testing / overview / roadmap / cyrius.cyml; CHANGELOG
> 2.6.14 + Release-History row added. **Still open:** audit P3 (two O(n²) hot paths, LM/L-BFGS
> per-iteration allocations) and P4 (10 doc drifts).
>
> **Third pass (2026-08-03, v2.6.12 + v2.6.13 — the audit-repair pair):** P0 of the
> 2026-08-03 audit is **fully closed**. 2.6.12 re-derived 47 hand-encoded constants across seven
> tables and added `scripts/check-constants.sh` as a CI gate (110/110); 2.6.13 fixed
> `svd_golub_kahan` (U transposed), `eigen_qr` (never converged for n >= 3), the SU(2)/SE(3)
> half-vs-full-angle split, `einsum` (mis-parsed its own documented examples, then segfaulted)
> and three interval enclosure-soundness violations. Suite **961 -> 1063**, every fix
> mutation-proven; `cyrius coverage` 50% -> **54%**; 4 of the 8 never-tested modules brought into
> the suites (`einsum`, `lie_ext`, `mat3`, `linalg_precision`). Synced version/counts across
> README / CLAUDE / testing / overview / roadmap / cyrius.cyml, added CHANGELOG 2.6.12 + 2.6.13
> and both Release-History rows. **Still open:** audit P1/P3/P4, and `calc_ext`, `noise_simplex`,
> `num_ext`, `symbolic_ext` remain included by no suite.
>
> **Same-day second pass (2026-08-03, post-2.6.11):** full P(-1) audit sweep of the released tree
> → `audit/2026-08-03.md` (**70 verified findings, 2 critical**), and a roadmap restructure that
> opens **2.6.12** as a repair release. Two ledger corrections fall out of it: the 2026-05-29 and
> 2026-05-30 closeouts' "posture solid" verdicts were **scope-limited** to each arc's new
> functions and did not reach the 8 modules no suite includes (noted in Tier 5, originals left
> per the dated-artifact rule); and forward-commitment #4 is now discharged. The Parked list
> gained a rule — shipped items are **removed** with a retirement note, not left struck through.
>
> **Prior refresh**: 2026-07-21 (v2.6.10) — Cyrius 6.4.66 → **6.4.69** toolchain
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

## At a glance — inventory (last reviewed 2026-08-03, v2.6.11)

**~23 markdown files** across the repo. Bucket counts:

| Bucket | Count | What it means |
|---|---|---|
| ✅ **Fresh / current** | ~12 | README, CHANGELOG, CLAUDE.md, CONTRIBUTING, SECURITY, `architecture/overview.md`, `guides/testing.md`, the four `development/` docs (roadmap, threat-model, dependency-watch, port-audit), and `benchmarks.md` — all current to v2.6.11. |
| 🟡 **Stale — refresh in place** | 0 | `benchmarks.md` was stale; re-ran `bench-history.sh` this pass. None outstanding. |
| 🟠 **Read-through outstanding** | 0 | CONTRIBUTING refreshed; `tool-issues.md` deleted; `linalg-proposal` archived. Cleared. |
| 🔵 **Evergreen** | 1 | `CODE_OF_CONDUCT.md` — Contributor Covenant; re-read only on policy change. |
| 📅 **Dated artifact — supersede, don't edit** | 8 | `audit/2026-04-15.md`, `audit/2026-05-29.md`, `audit/2026-05-29-cga-arc-closeout.md`, `audit/2026-05-30.md`, `benchmarks-rust-v-cyrius.md` (v2.2.0), `port-audit.md` (2026-04-15 + addendum), `development/archive/cyrius-linalg-proposal.md` (shipped). |
| 🐞 **Tracked toolchain issues (live on 6.5.6)** | 3 | `development/issues/*` — the 6.2.11 bump fixed 3 (archived). 3 now live, re-confirmed on 6.5.6 (v2.6.11, minimal repros): for-empty-clauses (verified); interval-ident-lex (verified — `iv_add`/`iv_sub`/`iv_mul` are reserved cycc SIMD intrinsics, unusable as var names, worked around by rename); CLI-clobber (not re-tested, destructive). Neither of the two verified ones was fixed by the 6.4 → 6.5 minor. See Tier 6. |

Numbers approximate; rolls up from the per-tier tables below.

**Why now**: the doc tree had drifted (README said v2.2.2, overview said "v1.4.0 /
27 lib files") until the 2026-05-29 sweep — there was no surface tracking
*aggregate* currency. This file is that surface. The same-day verify pass closed
the scaffold's open items rather than letting them linger.

---

## Tier 1 — Structural (root)

| File | Last touched | Status | Action |
|---|---|---|---|
| `README.md` | 2026-08-03 | ✅ Fresh | v2.6.11: toolchain → 6.5.6, version → 2.6.11, hisab tag → 2.6.11, sakshi 2.4.6 → **2.4.7**, tests 957 → **961**. (Prior v2.6.10: toolchain → 6.4.69, version → 2.6.10.) |
| `CHANGELOG.md` | 2026-08-03 | ✅ Fresh | **Source of truth per CLAUDE.md.** +2.6.11 entry (toolchain 6.4.69 → 6.5.6 + sakshi 2.4.6 → 2.4.7). Carries a **### Security** section — the upstream ganita 1.0.4 CWE-190 `mat_new` guard landed and the vendored copy had been stale at 1.0.3 — plus **### Fixed** for the 256-failure exit-code truncation. **Not** breaking. Refreshed every release. |
| `CLAUDE.md` | 2026-08-03 | ✅ Fresh | v2.6.11: toolchain/pin → 6.5.6, sakshi → 2.4.7, status line + CI-pin note, tests → 961; **program-epilogue principle changed** to `sys_exit_group` + harness clamp; Quick Start gained `cyrius fuzz`. (Prior v2.6.10: pin → 6.4.69.) |
| `VERSION` | 2026-08-03 | ✅ Fresh | Single source of truth (`2.6.11`). *(Row had drifted — it recorded `2.6.7` through the 2.6.8/2.6.9/2.6.10 releases.)* |
| `CONTRIBUTING.md` | 2026-08-03 | ✅ Fresh | v2.6.11: Cyrius pin → 6.5.6. (Prior v2.6.10: 6.4.69; v2.6.7: 6.3.11; +fmt/distlib gates.) |
| `SECURITY.md` | 2026-08-03 | ✅ Fresh | v2.6.11: allocation-overflow row **flipped from open to resolved** — stdlib `mat_new` gained its upstream guard in ganita 1.0.4 (cap 33,554,430, null on non-positive dims), with the note that the vendored copy was stale at 1.0.3 (where it segfaulted) and that `mat_new_guarded` is retained as the stricter 16M entry point. (Prior v2.6.7: Supported Versions → hisab **2.6.x**.) Mirrors `threat-model.md`. |
| `CODE_OF_CONDUCT.md` | 2026-03-22 | 🔵 Evergreen | Contributor Covenant. Re-read only on policy change. |

---

## Tier 2 — Architecture (`docs/architecture/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `overview.md` | 2026-08-03 | ✅ Fresh | v2.6.11: header → v2.6.11 / cycc 6.5.6. (Prior v2.6.10: → v2.6.10 / 6.4.69; v2.6.6: dependency-stack row → `math`+`ganita`, ganita subsumes matrix/linalg.) |
| `math.md` | 2026-05-30 | ✅ Fresh | Equation catalogue. §1 CGA (v2.5.4); **§2 differential geometry added in the v2.6.5 closeout** (curvature conventions, sectional/Weyl/Jacobi, transport, exterior algebra + references); §3 catalogue index. Earns the CLAUDE.md "math reference" slot. |

---

## Tier 3 — Operational / Development (`docs/development/`)

> `roadmap.md` is the forward-work surface (rotates every release). The rest rotate per-need.

| File | Last touched | Status | Action |
|---|---|---|---|
| `roadmap.md` | 2026-08-03 | ✅ Fresh | **Rotates every release.** v2.6.11: header → 6.5.6, Current → v2.6.11, assertions → 961, +2.6.11 Release-History row. **Restructured same-day for 2.6.12**: new **2.6.12 audit/hardening/repair** section scoped from the 2026-08-03 audit (P0 correctness → P4 docs, with sequencing); the resolved `mat_new` Parked item **removed** (not struck) with a one-line retirement note, per the new "shipped items leave this list" rule; 2.6.x recorded as **feature-closed with no residue**, and the three items too large for a patch (`vec_sort_by` adoption, SIMD flat-array kernels, per-module dependency manifest) **moved to 2.7.0**. Forward: 2.6.12 (repair), 2.7.0, 3.0.0, + Parked. |
| `threat-model.md` | 2026-08-03 | ✅ Fresh | v2.6.11: the `mat_new` attack-surface row **flipped open → ✅ RESOLVED** (ganita 1.0.3 → 1.0.4: `GANITA_MAT_MAX_ELEMS = 33554430` + null propagation), recording that the exposure was *worse* than prior entries stated — the vendored copy was stale at 1.0.3 and segfaulted (exit 139), which the toolchain-vs-toolchain vendoring check could not see. +2026-08-03 audit-history entry with the corrected process. (Prior: 2026-07-21 for v2.6.10; 2026-07-17 for v2.6.9.) |
| `dependency-watch.md` | 2026-08-03 | ✅ Fresh | Cyrius toolchain version-watch. Pin → **6.5.6** (+6-file stdlib delta: `ganita` CWE-190 `mat_new` guard, `syscalls*` `sys_exit_group`, `vec` `vec_sort_by`/`vec_select_nth`, `io`); sakshi 2.4.6 → **2.4.7** (private `_int`/`_str` dispatch-suffix rename; hisab audited clean). Records the `vec_sort_by` non-adoption with its concrete API-mismatch reason. Watching: 5.7.11 (RISC-V). |
| `port-audit.md` | 2026-05-29 | 📅 Dated + addendum | 2026-04-15 Rust→Cyrius parity snapshot, preserved; 2026-05-29 status addendum records nearly all "P0 gaps" now ported. Don't rewrite the body. |

> Removed this pass: `tool-issues.md` (deleted — ad-hoc catalog; real bugs live in `issues/`), `cyrius-linalg-proposal.md` (→ `archive/`, shipped).

---

## Tier 4 — Guides (`docs/guides/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `testing.md` | 2026-08-03 | ✅ Fresh | v2.6.13: suite counts → 307/200/369/**187** (**1063** total). (v2.6.11: 307/175/312/167 (**961** total, edge_cases +4 for the upstream `mat_new` contract); fuzz recipe now leads with `cyrius fuzz` (6.5.6 walks `tests/`; before that it looked only in `fuzz/`, found nothing, and still exited 0). (Prior v2.4.6 sweep: 901 total, **26** benchmarks, +collision/invariant coverage.) |
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
| `2026-08-03.md` | 2026-08-03 | 📅 Dated artifact — **full P(-1) sweep of the v2.6.11 tree, scoping 2.6.12.** 8 audit dimensions over all 34 modules, adversarially verified: **70 findings confirmed / 7 refuted** (2 critical, 23 high, 23 medium, 22 low; 64 sites in 27 files). Headline: 7 mis-transcribed hex-constant tables, 4 of them published tableaux falsified by their own invariants (DOPRI45 Σb = 0.636, Gauss-5 Σw = 2.000492, BDF-4 Σα = 1.01, Yoshida-4 = 1.474). Root cause identified as coverage: 8 modules / 5,068 lines / 104 public fns included by no suite; 35 of 65 source defects sit at ≤30% coverage. **Explicitly corrects the "posture solid" conclusion of the two 2026-05-29/30 closeouts** (they were scoped to arc-new functions only). |

> **Note on the two "posture solid" closeouts above.** Left at their shipped wording per the
> dated-artifact rule, but read them with the 2026-08-03 correction in hand: neither reached the
> never-included modules, so "solid" described the arc's new code, not the library.

Next periodic security audit: **due now** — all 70 findings of `2026-08-03.md` were closed across 2.6.12–2.6.15, and forward-commitment #4 calls for a re-audit confirming the repairs (and re-running the sweep against a tree where, for the first time, every module is under test). After that, the 3.0.0 (`Result<T,E>`) cut is the next natural boundary.

---

## Tier 6 — Tracked issues (`docs/development/issues/`)

Toolchain/CLI bug filings observed from hisab's vantage. Filed against cyrius
**5.7.x** and **6.4.x**; 3 of the original 5 were fixed at the 6.2.11 bump and moved to
`issues/archived/`, and one new filing (interval-ident-lex) was added at 2.6.9 — so **3 are
open**. All three were **re-verified 2026-08-03 against the pinned 6.5.6**: the two testable
ones still reproduce, and the 6.4 → 6.5 minor fixed neither. hisab carries workarounds for each
open one; the bugs belong upstream in cyrius.

**Open (`docs/development/issues/`)** — re-verified 2026-08-03 on 6.5.6 (prior: 2026-07-21 on 6.4.69; 2026-07-17 on 6.4.66; 2026-06-30 on 6.3.11):

| File | Filed | Status |
|---|---|---|
| `2026-07-17-cyrius-interval-ident-lex.md` | 2026-07-17 | 🐞 **Live** — re-confirmed on 6.5.6 by minimal repro: `var iv_add = 1;` → `expected identifier, got unknown`. `iv_add`/`iv_sub`/`iv_mul` are reserved cycc SIMD intrinsic names and cannot be used as variables. Workaround: `tests/modules.tcyr` renamed them `iv_sum`/`iv_diff`/`iv_prod` (guarded by a `NOTE:` in the test). *(This row was missing from the table through 2.6.9/2.6.10 even though the at-a-glance bucket counted 3.)* |
| `2026-04-26-cyrius-cli-arg-clobbers-source.md` | 2026-04-26 | 🐞 **Live (presumed)** — not re-tested on 6.5.6 (destructive: overwrites source). Workaround: `CYRIUS_VERBOSE=1` env, never an unknown flag before a subcommand. |
| `2026-04-26-cyrius-for-empty-clauses.md` | 2026-04-26 | 🐞 **Live** — re-confirmed on 6.5.6 by minimal repro: `for (; cond;)` → parser `unexpected ';'` (on 6.4.66 also `for (init; cond;)` → `expected '=', got '{'`). Workaround: `while` loops in collision_core/mesh. |

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
| `benchmarks.md` | 2026-08-03 | ✅ Fresh | **Re-run this pass** via `scripts/bench-history.sh` — now reflects commit `5331f05` (v2.6.11), 26 benchmarks; appended a `bench-history.csv` row. **No performance claim is attached to 2.6.11** — it changes no math code path, so the delta against the June baseline (`7ae75ef`) is run-to-run spread, not an effect of the bump. Cadence: every release closeout with perf-relevant change. |
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
| 1 | **Roadmap + Release History rotate every release** — completed arcs move out of active-work into the Release History table at closeout. | Every release | Per CLAUDE.md Work Loop §10–11. Last done in the v2.6.11 bump (+2.6.11 row; the `mat_new` Parked item struck as RESOLVED). |
| 2 | **Re-run `bench-history.sh` at release closeout** — keep `benchmarks.md` + `bench-history.csv` current; "numbers don't lie." | Every release with perf-relevant change | ✅ Done 2026-08-03 at v2.6.11 (was at `7ae75ef`; now `5331f05`, 26 benchmarks). No perf claim attached — 2.6.11 changes no math code path. |
| 3 | **Re-verify tracked `issues/` filings at each toolchain bump** — re-test on the new pin; move resolved ones to `issues/archived/`. | Each `cyrius.cyml` pin bump | ✅ Done 2026-08-03 on 6.5.6 — all 3 open issues still live (interval-ident-lex + for-empty-clauses confirmed by minimal repro; CLI-clobber not re-tested, destructive); the 6.4 → 6.5 minor fixed none. Prior: 2026-07-21 on 6.4.69; 2026-07-17 on 6.4.66; 2026-06-30 on 6.3.11; 2026-06-15 on 6.2.11 (3 of 5 fixed and archived). |
| 4 | **Periodic security audit** — full source scan before a major release or after significant surface change; supersede with a new dated `docs/audit/` doc. | Before 3.0.0; on significant change | ✅ **Done 2026-08-03 — `audit/2026-08-03.md`**, a full P(-1) sweep of all 34 modules: 70 verified findings (2 critical), scoping 2.6.12. Supersedes the scope, though not the text, of the 2026-05-29/30 closeouts. Also 2026-08-03 in `threat-model.md`: the vendored `ganita` staleness (dependency currency, not source). Prior: 2026-04-15, 2026-05-29. **Next: re-audit after 2.6.12 lands, to confirm the repairs.** |
| 5 | **Byte-check `lib/*.cyr` against `~/.cyrius/versions/<pin>/lib/` at every pin bump** — compare the vendored tree to the *pin*, never previous-pin against new-pin. | Each `cyrius.cyml` pin bump | **New at v2.6.11.** The toolchain-vs-toolchain comparison used through 2.6.10 is blind to vendoring drift and hid the stale `ganita` 1.0.3 for three releases. `cyrius lib sync` then `cmp` every file (excluding `sakshi.cyr`), and treat any `lib/` file that differs from the pin as a defect, not a diff. |

---

*Initial scaffold: 2026-05-29 (v2.4.6), adapted from `cyrius/docs/doc-health.md`, immediately after the post-2.4.x documentation sweep. Same-day verify-and-cleanup pass: bench re-run, `tool-issues.md` retired, linalg proposal archived, CONTRIBUTING currency fixed, all 5 toolchain issues re-verified live on 6.0.14. Refresh in place when docs are touched.*
