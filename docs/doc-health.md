---
name: Hisab Documentation Health
description: Living state of doc currency in the hisab repo — fresh / stale / read-through / dated / tracked-issue, refreshed as docs are touched
type: state
---

# Documentation Health — hisab

> **Last refresh**: 2026-08-10 — **v2.10.0, differentiable geometry.** The first feature line since
> 2.8.x: `src/geo_diff.cyr`, jets for plane/sphere/triangle returning the full gradient from one
> evaluation as a post-pass on the shipped primal. 34 → **35** `[lib]` modules, suite 3376 → **3398**,
> benchmarks 60 → **64**, toolchain 6.5.16 → **6.5.17** (which fixed all three defects hisab filed
> upstream — the closure SIGSEGV and the `distlib` false positive fully, the dead-fn gate hole only
> partially: `lint` and `vet` still exit 0 on a file that does not parse, measured and fed back).
>
> **The release led with a defect in shipped code, found before any feature code was written.**
> Running 2.10.0's own design check — Euler's relation `d·∂t/∂d = −t` — against the EXISTING
> primitives showed `geo_ray_sphere` was not degree −1 in the ray direction: it returned a `t` whose
> hit point sat **3.74 from the centre of a unit sphere**, silently, with `geo_ray_capsule`
> inheriting it through its end caps. Four of six siblings honoured the contract; two did not. The
> same pass found **17 of 60 benchmarks measuring `clock_gettime` rather than the operation** —
> `ray_sphere` read 1,466 ns and is 79 ns — which flattened a true 3.6× triangle/sphere ratio into
> 1.19× and is the only reason the repair's +17.6% cost was visible at all.
>
> **Standing lesson, and it is the sharpest form of the arc's:** a correctness check written for a
> feature that does not exist yet is worth running against the code that does. Three of this
> release's findings came from that single move.
>
> ⚠ **A fixture-discrimination failure happened three times this release** and is now worth treating
> as a standing question rather than an occasional slip: the k-d balance guard's fixture never fired
> the guard; the `g1` harness asserted order-consistency, which a uniform sign flip satisfies; and
> the sphere jet's fixture used radius 1, where `dt/dr = r/(g·d)` is indistinguishable from
> `1/(g·d)`. Each was caught by mutation, not by review. **Ask of every new fixture: what mutant
> does this actually kill?**
>
> **Previous refresh**: 2026-08-10 — **v2.9.3, the open filings.** Five of the six internal filings in
> `development/issues/` closed; the sixth re-diagnosed and correctly open. Suite 3351 → **3376**,
> benchmarks 55 → **58**, constant gate 153 → **155**.
>
> **The filings were the thing that needed auditing.** Two of the six argued for repairs that
> measurement refuted, and both were implemented in full before being rejected: `incircle-precision`
> implies an exact determinant of *pre-differenced* operands, which fixes nothing because the two
> operands have already rounded to the same double before any exact arithmetic runs; and
> `epa-certificate` proposes dropping EPA's seed test, which certifies every box but makes **four
> exact-tangency pairs report a nonzero depth for a touch whose true depth is 0**. A third claim —
> that EPA's certified exit "is never taken" — was false because it only ever measured one of the two
> public entry points: `gjk_epa_3d` falls back 12 of 12, `mpr_penetration` **0 of 12**.
>
> Two real defects came out of it, both silent: `delaunay_2d` **dropped input points** on any set
> mixing scales (fixed with an adaptive exact `orient2d` on raw coordinates, 300/300 against exact
> rationals where the float winding scores 0/300), and `_col_dl_incircle` answered differently
> depending on **vertex order**. Plus three guards policing nothing — the k-d balance guard had
> neither a regression assertion nor a benchmark on its own input class (now 6.0×), the ear prune's
> only benchmark was its best case (reflex-heavy is 3.6×), and `_col_point_in_tri` documented itself
> as strict-interior while being boundary-inclusive.
>
> **Standing lesson, and it is 2.9.2's pointed one step further back:** a gate is not evidence until
> something has aimed it at its own output — and *a filing is not evidence until something has re-run
> it*. Three of this release's six were wrong in the record, not in the code.
>
> **Previous refresh**: 2026-08-09 — **v2.9.2, toolchain release.** Pin 6.5.9 → **6.5.16** (seven
> releases), sakshi 2.4.8 → **2.4.10**, no library source change. **The doc work was the larger half
> of this release.** A five-dimension sweep of the tree, each dimension adversarially verified,
> found **63 stale documentary claims across 15 files**; the verifier reproduced **59** and refuted
> 4 (three of which had already been fixed mid-sweep, one a misreading of which tracker a filing
> lived in). CLAUDE.md, README.md, CONTRIBUTING.md, SECURITY.md, `architecture/overview.md`,
> `guides/testing.md`, `development/dependency-watch.md` and `development/threat-model.md` had all
> been last synced at **v2.6.15 / cyrius 6.5.6 / sakshi 2.4.7** and had missed nine releases.
>
> **Three gates were green while checking nothing**, and none was found by a test failing:
> `scripts/bench-history.sh` recorded each benchmark's **max** rather than its average in every row
> it has ever written (44 of 55 in the latest run) — the CSV CLAUDE.md designates as the proof for
> every performance claim was built on the noisiest statistic the harness reports, swinging −93% to
> +742% between runs where the average is stable to a median of 3.5%; CI's version-consistency gate
> was an unanchored `grep -q "$VERSION" CHANGELOG.md` whose regex dots matched `32,942,104 B` in an
> unrelated benchmark line, so it would have passed a release with no CHANGELOG section at all; and
> `src/main.cyr`'s hardcoded CLI version string was checked by nothing. All three are now gated.
> **Standing lesson, and it is the 2.6.x lesson pointed at the tooling instead of the code:** a gate
> is not evidence until something has aimed it at its own output.
>
> **Previous refresh**: 2026-08-04 — **v2.8.1, audit release.** Six-dimension sweep of the v2.8.0 tree,
> each dimension adversarially verified by reproduction: **42 confirmed, 0 refuted** (4 critical,
> 17 high, 16 medium, 5 low) — `audit/2026-08-04-v2.8.0-full.md`, scheduled across 2.8.2–2.8.5.
> **The headline is a defect in what 2.8.0 shipped**: `delaunay_2d` is silently incomplete on 15–37%
> of ordinary uniform-random input, from a 10x super-triangle that predates the rewrite — and the
> obvious fix (raise the multiplier) breaks clustered input, because it is coupled to the
> already-filed non-adaptive in-circle predicate. Standing lesson recorded: **1801 assertions and
> 97% coverage caught none of these**, because coverage counts whether a function is *referenced*,
> not whether its contract is *checked*. The sharpest illustration is in the report — every ODE test
> integrand is autonomous, so the DOPRI45 abscissa row is certified by nothing.
>
> **Previous refresh**: 2026-08-04 — **v2.7.0 RELEASED.** Suite 1127 → **1605**, coverage 59% → **71%**,
> benchmarks 28 → **35**, constant gate 110 → **141** + a duplicate-global check. 35 of the
> re-audit's 41 findings fixed; the 6 open ones each carry a written reason and a filed spec.
> `lib/hisab.cyr` **deleted** and `cyrius.lock` regenerated (30 verified) — it was a tracked 591 KB
> copy of hisab's own distlib inside the vendored-deps directory. New: a verified module dependency
> manifest in `architecture/overview.md`, and `audit/2026-08-04-2.7.0-closeout.md` superseding the
> re-audit. **Three corrections to my own analysis are recorded in that close-out** — a grep that
> assumed one space before `=`, a probe that generated coordinates by magnitude class instead of
> reconstructing the caller's geometry, and a scan that counted comments as code. Each contradicted
> a finding; each was wrong. The standing rule: **gates beat greps.**
>
> **Previous refresh**: 2026-08-04 — **re-audit pass** (`audit/2026-08-04.md`). Doc-health work:
> every Tier 1–7 row re-dated to the v2.6.15 state; at-a-glance buckets recounted (Fresh 13,
> read-through **2**, dated artifacts **9**); forward-commitments rewritten with **two new
> entries** (#6 run the constant gate after any constant change, #7 state an audit's SCOPE
> alongside its verdict). Real drift fixed alongside the ledger: CONTRIBUTING and CLAUDE.md did
> not list `check-constants.sh` among the gates, and CONTRIBUTING still led with the manual fuzz
> build. **Correction:** the fifth pass below recorded the 2026-08-03 audit as discharged in full
> — it was not; six of the 70 findings were never scheduled. See `audit/2026-08-04.md` §2.
>
> **2.7.0-A/B repair batches (same day).** `CHANGELOG.md` and `roadmap.md` re-dated to
> **2026-08-04**; `audit/2026-08-04.md` gained a **Disposition → Closed so far** table, per its own
> commitment that the finding list is the unit of disposition. Ten findings closed (6 carried over,
> 3 high, 1 medium) plus one found while fixing; suite **1127 → 1181**. One new item was filed that
> no audit raised: `lib/hisab.cyr`, a tracked 591 KB copy of hisab's own distlib inside the
> vendored-deps directory, referenced and regenerated by nothing.
>
> **Standard-of-evidence note, recorded because it is a precedent:** every fix in these batches was
> mutation-proven except the BVH one, and the CHANGELOG, roadmap and audit all say so explicitly
> rather than letting a passing suite imply proof. Both forms of `_bvh_build_rec` return the same
> tree — the defect is a *cost*, so it is guarded by a benchmark and the assertions guard only the
> result. **An unprovable-by-assertion fix should be labelled as such, not quietly grouped with the
> proven ones.**
>
> **2.7.0-C (same day).** Four of seven medium findings closed; suite **1181 → 1217**. The three
> unapplied ones are filed as dated records under `development/issues/` rather than left in a
> chat log — each carries its verification, its adversarial challenge, and an exact fix.
> `CHANGELOG.md` and `roadmap.md` re-dated again. **Method note worth keeping:** every finding was
> verified *and then adversarially challenged* by an independent reviewer, both running real code.
> The challenges paid for themselves twice — one proved the proposed `levi_civita` assertions all
> pass a wrong-sign mutant, the other found a live SIGSEGV in `wedge_1_1` that the finding never
> mentioned. **A proposed test is not evidence until something has tried to defeat it.**
> **2.7.0-D (same day).** Two more closed — the unscaled Householder (the most consequential fix of
> the line: `svd_golub_kahan` was correct only in roughly 1 .. 1e23, failing **40/40 at scale
> 1e-10**, i.e. SI metres) and the symbolic render carry. Suite **1217 → 1234**. One finding
> remains, `svd_golub_kahan` on rank-deficient input, still filed under `development/issues/`;
> it was **re-checked after balancing landed** rather than assumed fixed, and it is not.
> The v2.6.11 record follows.
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
> **Fifth pass (2026-08-03, v2.6.15).** *(Header corrected 2026-08-04 — this pass claimed the
> audit was discharged in full; it was not. See the sixth pass above.)* P3 (performance) and P4
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
> **Convention**: adapted from `cyrius/docs/doc-health.md`. hisab's tree is 38
> tracked markdown files (vs cyrius's ~105), so the tier structure here is leaner.

This is a **ledger**, not a one-time audit. Rewrite-in-place as docs change.

---

## At a glance — inventory (last reviewed 2026-08-10, v2.9.3)

**38 tracked markdown files** (`git ls-files '*.md' | wc -l`), counted rather than estimated —
the previous "~23" was an estimate that had been carried since 2026-05-29. Bucket counts:

| Bucket | Count | What it means |
|---|---|---|
| ✅ **Fresh / current** | 14 | README, CHANGELOG, CLAUDE.md, CONTRIBUTING, SECURITY, VERSION, `architecture/overview.md` + `math.md`, `guides/testing.md`, `development/` roadmap + threat-model + dependency-watch + `port-audit.md`, and `benchmarks.md`. **All re-synced to v2.9.2 / cyrius 6.5.16 / sakshi 2.4.10 on 2026-08-09** — see the note below on how far they had drifted. |
| 🟡 **Stale — refresh in place** | 0 | None outstanding. `benchmarks.md` is regenerated by `scripts/bench-history.sh`; the 2.9.2 run is the **first correct one** (the parser recorded each benchmark's max, not its average, in every row before it). |
| 🟠 **Read-through outstanding** | 0 | `SECURITY.md` and `development/threat-model.md` both carry the 2.7.0 → 2.9.x closures. |
| 🔵 **Evergreen** | 1 | `CODE_OF_CONDUCT.md` — Contributor Covenant; re-read only on policy change. |
| 📅 **Dated artifact — supersede, don't edit** | 12 | `audit/` (8 reports), `benchmarks-rust-v-cyrius.md` (v2.2.0), `development/archive/cyrius-linalg-proposal.md` (shipped), plus the two `2026-08-04-incircle-repro*.tcyr` fixtures under `issues/`. |
| 🐞 **Open filings** | 4 | `development/issues/*`, and **all four are now upstream-toolchain**, not hisab: `cli-arg-clobbers-source` (deliberately never re-tested — destructive, recorded as unverified rather than assumed), `distlib`'s self-check rejecting bundles that read a stdlib global, dead-function bodies never being syntax-checked, and `epa-certificate` — the one internal item still open, and **re-diagnosed in 2.9.3**: its central claim was false and both its candidate repairs are measured and rejected; what is left is the sphere-family non-convergence, which is independent of the certificate. `for-empty-clauses` was archived on discovering upstream had closed it **WON'T-FIX by design** five releases earlier. Two of the four are filed upstream in the cyrius repo. |

Numbers roll up from the per-tier tables below.

> ⚠ **How far this ledger had drifted, recorded because it is the ledger's own failure.** Between
> v2.6.15 and v2.9.1 the Tier 1–4 rows below went on reading `✅ Fresh` while the files they
> described missed **nine releases and four toolchain pins**. Every headline number in them was
> wrong by a large factor — the bundle was cited at "~578 KB / 17,286 lines" against a measured
> 805,479 B / 21,368; the suite at "1127 across 4 suites" against 3351 across five; benchmarks at 28
> against 55; the constant gate at 110/110 against 153/153; coverage at 59% against 99%. A currency
> ledger that is itself stale is worse than none, because it converts "nobody checked" into
> "somebody checked and it was fine". The 2.9.2 sweep re-derived every number from the tree rather
> than from the previous row.

**Why now**: the doc tree had drifted (README said v2.2.2, overview said "v1.4.0 /
27 lib files") until the 2026-05-29 sweep — there was no surface tracking
*aggregate* currency. This file is that surface. The same-day verify pass closed
the scaffold's open items rather than letting them linger.

---

## Tier 1 — Structural (root)

| File | Last touched | Status | Action |
|---|---|---|---|
| `README.md` | 2026-08-10 | ✅ Fresh | **v2.10.0**: version/tag → 2.10.0, module table gains **`geo_diff`** and the Geometry row now names differentiable ray/surface intersection, library → 35 modules / 21,855 lines. Earlier, **v2.9.3**: version/tag → 2.9.3, tests → **3376**, benchmarks → **58**, library → 21,498 lines, bundle → ~798 KB. Earlier, **v2.9.2 — nine releases of drift closed in one pass.** Every stat re-derived from the tree: version/tag → **2.9.2**, toolchain → **6.5.16**, sakshi → **2.4.10**, tests → **3351 across five suites** (the fifth, `abuse.tcyr`, had been missing since 2.9.0), benchmarks → **55**, bundle → **805,479 B / 21,368 lines**, ELF → **220,064 B**, src → **21,262 lines**. It had been carrying the v2.6.15 figures — `~578 KB`, `1127`, `28`, `6.5.6`, `2.4.7` — since 2026-08-03. |
| `CHANGELOG.md` | 2026-08-10 | ✅ Fresh | **v2.10.0**: +2.10.0 — differentiable geometry, led by the `geo_ray_sphere`/`geo_ray_capsule` homogeneity defect the feature's own check found in shipped code, plus the 17-of-60 benchmark-overhead fix and the 6.5.17 bump closing all three upstream filings. Earlier, **+2.9.3.** Earlier: **Source of truth per CLAUDE.md.** +2.9.2 — the toolchain bump plus the three non-gating gates, with the alloc bullet **corrected in place**: it first claimed hisab does not reach `alloc_via`, which is false (`str_from`/`vec_new`/`vec_push` delegate through `default_alloc()`, 152 call sites in `src/`). Earlier: +2.6.12/13/14/15 entries — the audit-repair arc — plus the **[Unreleased] 2.7.0** section: the correction notice, 2.7.0-A (6 carried-over findings + the `beta`→`eta` rename), 2.7.0-B (3 high + the `cmat` null medium), a **Breaking** entry for `geo_ray_plane`, and a Performance table for `bvh_degenerate_4k` (2.722 s → 15.5 ms). 2.6.12 and 2.6.14 carry **### Security** sections. None breaking; no API or signature change across the arc, so consumers only rebuild. |
| `CLAUDE.md` | 2026-08-10 | ✅ Fresh | **v2.10.0**: status → 2.10.0 / cycc **6.5.17**, every count re-measured rather than carried (3398 across 5 suites, 64 benchmarks, 155/155 constants, 599/604 coverage over 36/36 files, bundle **832,590 B / 21,964 lines** = 79.4% of `input_buf`, ELF 224,160 B, src 35 modules / 21,855 lines); the distlib gate rewritten — the 6.5.14–6.5.16 self-check tolerance is **removed**, that filing archived; the arc paragraph extended with 2.10.0. Earlier, **v2.9.3**: status → 2.9.3, counts re-measured (3376/5 suites, 58 benchmarks, 155/155 constants, bundle 817,332 B / 21,604 lines = 78.0% of the 1 MB `input_buf`), and the arc paragraph gained what 2.9.3 found — that two of six filings argued for repairs measurement refuted. Earlier, **v2.9.2**: Status paragraph rewritten from v2.6.15 to the 2.7.0–2.9.2 arc; toolchain → 6.5.16; every count re-measured (3351/5 suites, 55 benchmarks, 153/153 constants, coverage 99%, bundle 805,479 B). Layout gained `tests/abuse.tcyr` and the two new scripts; CI gate list gained the measurement job and the `check --with-deps` bundle step. Also **corrected `cyrius fmt --check <file>` → `cyrius fmt <file> --check`** — the documented order prints a usage error and exits 1; CI always had it right. |
| `VERSION` | 2026-08-10 | ✅ Fresh | **v2.10.0**: → `2.10.0` via `scripts/version-bump.sh`; all four sites re-verified agreeing (manifest `${file:VERSION}`, CHANGELOG anchored header, `src/main.cyr` string, `dist/hisab.cyr` bundle header). Earlier, Single source of truth (`2.9.3`). **The two sites it cannot reach are now gated AND automated** — `scripts/version-bump.sh` rewrote `src/main.cyr` on this bump, which is the gate added at 2.9.2 doing its job. **Two sites it cannot reach are now gated instead of trusted**: `src/main.cyr`'s printed string (Cyrius has no build-time interpolation) and `dist/hisab.cyr`'s `# Version:` header. `scripts/version-bump.sh` rewrites the first; CI asserts both. |
| `CONTRIBUTING.md` | 2026-08-10 | ✅ Fresh | **v2.10.0**: suite count 3351 → **3398**. Earlier, **v2.9.3**: pin note re-dated. Earlier, v2.9.2: prerequisite pin 6.5.6 → **6.5.16**; both test-suite lists gained `tests/abuse.tcyr`; local gate recipe gained `check-measurements.sh` and the note that `cyrius distlib` exits 1 on a correct bundle under ≥ 6.5.14. |
| `SECURITY.md` | 2026-08-04 | ✅ Fresh | **Debt discharged.** +7 attack-surface rows organised by *defect class* rather than by module, so the table teaches the pattern: capped-constructor null stores, OOB read, work bounds that do not bound work, recursion depth on degenerate geometry, negative indexing, NaN-read-as-passed, and collision-query completeness. Design Principles gained the mutation-proof standard and the constant gate. Supported-versions table opened a 2.7.x row. Mirrors `threat-model.md`. |
| `CODE_OF_CONDUCT.md` | 2026-03-22 | 🔵 Evergreen | Contributor Covenant. Re-read only on policy change. |

---

## Tier 2 — Architecture (`docs/architecture/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `overview.md` | 2026-08-10 | ✅ Fresh | **v2.10.0**: header → v2.10.0, 35 math modules, 21,855 lines. Earlier, **v2.9.3**: header → v2.9.3 / cycc 6.5.16 / 21,498 lines (the exact-`orient2d` predicate). Earlier, v2.9.2: Earlier, v2.6.15: module map corrected — **BVH moved to `geo_advanced` (it was credited to `spatial`)**, `f64_le`/`f64_ge` removed from `f64_util`, CG relabelled Polak-Ribière+, error constants → `HSB_ERR_*`. |
| `math.md` | 2026-08-03 | ✅ Fresh | Equation catalogue. §1 CGA (v2.5.4), §2 differential geometry (v2.6.5). v2.6.15: `hodge_star_2form_4d`'s `sign` corrected — it is an overall multiplier on the Lorentzian dual, **not** a Euclidean/Lorentzian selector. |

---

## Tier 3 — Operational / Development (`docs/development/`)

> `roadmap.md` is the forward-work surface (rotates every release). The rest rotate per-need.

| File | Last touched | Status | Action |
|---|---|---|---|
| `roadmap.md` | 2026-08-10 | ✅ Fresh | **v2.10.0**: **v2.10.0 RELEASED** — Current re-stated at 2.10.0 / 6.5.17, release-train row struck through, the 2.10.0 planning section retired to a delivered one, and **a new `2.10.1 — the branchy primitives` section** written (the `_core` split of `geo_ray_aabb`/`_obb`/`_capsule` at the 12 enumerated `f64_max`/`f64_min` sites, their jets, per-primitive benchmarks, and the autodiff-vector-layer question deferred to 2.11.0 where the reverse-mode tape forces it). History row added — and the 2.9.3 row's **Files column corrected 58 → 35**, which had been the benchmark count in a module-count column, with the rows re-sorted descending. Earlier, **v2.9.3 RELEASED** — Current re-stated, release-train row struck through, planning section retired to a released one, history row added. Earlier: **Rotates every release.** 2.7.0 sections *Carried over* and *High* marked **CLOSED** with evidence inline; the `cmat` null-propagation medium ticked; a new *Found during 2.7.0-B* section holds the `lib/hisab.cyr` item. 26 items still open under 2.7.0. v2.6.15: Current → v2.6.15; the **whole 2.6.12 audit section retired** (all 32 items closed) to a one-paragraph record + Release-History rows — 327 → 228 lines, **zero completed items left inline**; 2.7.0 re-scoped as **re-audit & refactor**; new **2.7.x feature line** holding the original 2.7.0 items plus those moved out of 2.6.x. |
| `threat-model.md` | 2026-08-04 | ✅ Fresh | **Debt discharged.** Two new sub-tables — *Memory-safety tier — closed in v2.6.14* (6 rows) and *Closed in v2.7.0* (8 rows) — plus three audit-history entries (the 2026-08-03 sweep, the 2026-08-04 re-audit, the 2.7.0-A/B batches). The re-audit entry records **both** process failures verbatim, since the failure mode is the transferable part: scheduling from a digest instead of the finding list, and a coverage-reporting gate whose own coverage was never checked. ⚠️ Also **corrected a row my own change falsified** — `geo_ray_plane` no longer returns −1. **2026-08-05:** the `sequential_impulse` row was split — it credited 2.4.5 with a fix and then covered a *second* defect class it never mentioned. It now carries one row per failure (normal impulse, incl. the 2.8.3 restitution repair it had absorbed silently) plus a new row for the friction impulse that was **identically 0 at every mu** until post-2.8.4. |
| `dependency-watch.md` | 2026-08-10 | ✅ Fresh | **v2.10.0**: pin **6.5.16 → 6.5.17** with a full entry: compiler-only, **zero stdlib delta** (all 29 vendored files + `sakshi.cyr` byte-match the 6.5.17 snapshot AND are identical between the two pins, so `lib sync` was a no-op), and all three hisab-filed defects re-run from their own repros — closure SIGSEGV and `distlib` false positive **fixed**, dead-fn syntax check **partial** (`build`/`check` reject, `lint`/`vet` still exit 0). The `vec_sort_by` deferral rewritten: its measured blocker is retired, only the API mismatch and the third-instance rule survive. Earlier, Cyrius toolchain version-watch. Pin **6.5.16**, sakshi **2.4.10**; the 6.5.10–6.5.16 delta recorded (10 changed vendored files + transitive `result.cyr`), including the measured ~4 ns/`alloc_via` from 6.5.10's accessor inlining and why no macro-level win is claimed. It had gone three pins without an entry — 6.5.8 and 6.5.9 were never recorded here at all. |
| `port-audit.md` | 2026-05-29 | 📅 Dated + addendum | 2026-04-15 Rust→Cyrius parity snapshot, preserved; 2026-05-29 status addendum records nearly all "P0 gaps" now ported. Don't rewrite the body. |

> Removed this pass: `tool-issues.md` (deleted — ad-hoc catalog; real bugs live in `issues/`), `cyrius-linalg-proposal.md` (→ `archive/`, shipped).

---

## Tier 4 — Guides (`docs/guides/`)

| File | Last touched | Status | Action |
|---|---|---|---|
| `testing.md` | 2026-08-10 | ✅ Fresh | **v2.10.0**: suite → **3398** (modules 1621 → 1668) in both the recipe and the table, benchmarks 55 → **64** with the new `geo_diff` / tensor / hull / kd rows, and a `bench_batch` warning added to the measurement paragraph — `bench()` cannot see anything under ~1 µs, which is how 17 rows came to be ~95% instrumentation. Earlier, **v2.9.2**: suite counts → 416/349/1621/233/**732** (**3351** total across **five** suites — `abuse.tcyr` was absent from both the recipe and the table), benchmarks 28 → **55**. **Still owes the test-quality revision** carried since 2.7.0. |
| `usage.md` | — | ⚪ Not yet earned (does not exist; CLAUDE.md's Documentation Structure was corrected in 2.9.2 to stop implying it does) | "When earned" patterns/examples guide (CLAUDE.md). README Quick Start covers basics today; promote if onboarding needs more. |

---

## Tier 5 — Audits (`docs/audit/`)

Periodic audit reports; per-audit timestamped. **Don't refresh in place — supersede with a new dated doc.**

| File | Date | Status |
|---|---|---|
| `2026-04-15.md` | 2026-04-15 | 📅 Dated artifact — P(-1) audit (31 issues, 25 fixed) |
| `2026-05-29.md` | 2026-05-29 | 📅 Dated artifact — security/hardening audit closing the 2.4.x arc (no new vuln) |
| `2026-05-29-cga-arc-closeout.md` | 2026-05-29 | 📅 Dated artifact — 2.5.x closeout (v2.5.4): P(-1)/security review of the CGA operators + `mat_new_guarded` (posture solid) + the math.md deliverable |
| `2026-05-30.md` | 2026-05-30 | 📅 Dated artifact — 2.6.x closeout (v2.6.5): P(-1)/security review of the diffgeo curvature/transport/form functions (posture solid) + the math.md §2 deliverable |
| `2026-08-04.md` | 2026-08-04 | 📅 Dated artifact — **re-audit of the repaired v2.6.15 tree, scoping 2.7.0.** 6 dimensions; **42 confirmed / 8 refuted** (4 high, 21 medium, 17 low; no criticals). **Regression verification CLEAN** — every 2.6.12–2.6.15 repair holds under execution, verified against mathematical invariants with pre-fix files re-run as controls. **Corrects the 2026-08-03 disposition record**: six of the original 70 findings were never scheduled, because work was driven from the roadmap digest rather than the finding list. Also found the constant gate silently skipping 35 of 145 declarations. Findings concentrate in `geo_advanced.cyr` (largest module, zero coverage on GJK/EPA/BVH/TOI, uncited by the first sweep) and in test quality (12 findings). |
| `2026-08-03.md` | 2026-08-03 | 📅 Dated artifact — **full P(-1) sweep of the v2.6.11 tree.** 70 findings confirmed / 7 refuted (2 critical, 23 high, 23 medium, 22 low; 64 sites in 27 files). **Disposition corrected 2026-08-04**: the four P-tiers were closed across 2.6.12–2.6.15, but **six findings were never scheduled** and remain open — they never appeared in the roadmap digest the work was driven from. See `2026-08-04.md` §2. Headline: 7 mis-transcribed hex-constant tables, 4 of them published tableaux falsified by their own invariants. Root cause identified as coverage: 8 modules included by no suite, holding 35 of 65 source defects. Explicitly corrects the "posture solid" conclusion of the two 2026-05-29/30 closeouts. |
| `2026-08-04-v2.8.0-full.md` | 2026-08-04 | 📅 Dated artifact — **full sweep of the v2.8.0 tree, scoping 2.8.x.** 6 dimensions, adversarially verified by reproduction; **42 confirmed / 0 refuted** (4 critical, 17 high, 16 medium, 5 low). **Amended in place 2026-08-05** — the one exception to the supersede-don't-edit rule on this tier, because 2.9.0 exit criterion 5 requires a per-finding disposition and the table shipped without a Disposition column, so the standing rule the doc itself states could not be satisfied by anyone. Now carries that column, verified against the tree rather than the CHANGELOG. All 4 criticals, all 17 highs and all 16 mediums were discharged by 2.8.2 / 2.8.3 / 2.8.4. **Low tier CLOSED 2026-08-05**: the 5 lows had never been scheduled and were worked as one pass, so the original 42 now stand at **42 FIXED, 0 OPEN** — with two sub-claims inside the allocation-inside-loop row that did not survive re-measurement and are recorded as such (`src/tensor.cyr:217` **REFUTED**, already hoisted; the row's fourth site **UNRESOLVED**, truncated to `src/geo_` in `70e7c8b` as well as HEAD). Two of the five lows were understated by their tier and the cells say so: the uncapped `max_depth` was a measured **SIGSEGV**, and the sibling-insert return convention is a **breaking** API change. Tracked separately: the friction half of the split `sequential_impulse` row. **Addendum row FIXED 2.9.1 (amended in place again 2026-08-06), and re-measured from scratch first** — its "26 of 3,012" had never been independently reproduced and a second sweep had found zero, so the row now carries an 11,490-evaluation sweep classified exactly in `Fraction`: **50 of the 7,540 exact-support evaluations**, all tangencies, reducing to 6 configurations of which 2 (a point on a solid box CORNER, answered differently in the two operand orders) were in neither prior report. That leaves the whole report at **0 OPEN**. One of its own `clean` notes ("RAY / OVERLAP SIBLINGS AGREE") is retracted there, and the retraction now records *why* it was a blind spot rather than an error: every real divergence needs a zero-volume operand. |

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

**Toolchain filings** — re-verified 2026-08-10 on **6.5.17** from each filing's own minimal repro
(prior: 2026-08-09 on 6.5.16; 2026-08-03 on 6.5.6; 2026-07-21 on 6.4.69; 2026-07-17 on 6.4.66;
2026-06-30 on 6.3.11). ⚠ Rows carrying an `archived/` prefix have SHIPPED and live in
`docs/development/issues/archived/`; only the unprefixed ones are still in `issues/`:

| File | Filed | Status |
|---|---|---|
| `archived/2026-07-17-cyrius-interval-ident-lex.md` | 2026-08-10 | 🟢 **CLOSED, archived 2026-08-09.** The *diagnostic* was the defect and it is fixed — all 67 reserved names emit the new wording on 6.5.16 and 6.5.17. ⚠ **The reservation itself is BY DESIGN and still stands**: `iv_add`/`iv_sub`/`iv_mul` remain unusable as variable names, so `modules.tcyr`'s `iv_sum`/`iv_diff`/`iv_prod` rename is load-bearing and must not be reverted. Original text: 🐞 Live — re-confirmed on 6.5.6 by minimal repro: `var iv_add = 1;` → `expected identifier, got unknown`. `iv_add`/`iv_sub`/`iv_mul` are reserved cycc SIMD intrinsic names and cannot be used as variables. Workaround: `tests/modules.tcyr` renamed them `iv_sum`/`iv_diff`/`iv_prod` (guarded by a `NOTE:` in the test). *(This row was missing from the table through 2.6.9/2.6.10 even though the at-a-glance bucket counted 3.)* |
| `2026-04-26-cyrius-cli-arg-clobbers-source.md` | 2026-08-09 | 🐞 **Live (presumed)** — **deliberately not re-tested at the 2.9.2 bump either**, and now recorded as *unverified* rather than assumed-still-live. The reproducer works by getting cyrius to treat a real source path as the output slot, so running it here truncates a `src/*.cyr` to 0 bytes. Only safe observation: `cyrius -v` with no path arguments still exits 0 and prints usage rather than rejecting the unknown flag, so the strict-flag check that would fix it is absent. Suggestive, not proof. Workaround: `CYRIUS_VERBOSE=1`, never an unknown flag before a subcommand. |
| `archived/2026-04-26-cyrius-for-empty-clauses.md` | 2026-08-09 | 🐞 **Live on 6.5.16, but half-fixed.** Both forms still reject (`for (; c; s)` → `unexpected ';'`; `for (i; c;)` → `expected '=', got '{'`). What the filing actually complained about — the error landing far down the file, findable only by bisection — is **gone**: errors now carry the true `file:line` with a caret on the token, and both forms report in the same run. ⚠ Reproduce in an `include`d module: line numbers for the *top-level* source are offset by (stdlib dep count + 1) because they index the concatenated translation unit. Workaround unchanged: `while` loops in collision_core/mesh. |
| `2026-08-09-cyrius-dead-fn-bodies-are-never-syntax-checked.md` | 2026-08-10 | 🐞 **Live on 6.5.17, PARTIALLY fixed.** `cyrius build` and `cyrius check --with-deps` now both reject the repro (exit 1) — the half that matters, since nothing unparseable can be built or shipped. But **`cyrius lint` and `cyrius vet` still exit 0** on a file that does not parse, so the release note's "accepted by every gate" is not discharged. Measured on the filed repro at the 6.5.17 bump and fed back upstream; left OPEN rather than closed. |
| `archived/2026-08-10-cyrius-capturing-closure-across-fn-boundary.md` | 2026-08-10 | 🟢 **FIXED in 6.5.17.** Re-run from the filed repro: case-3 returns **42** (was SIGSEGV 139) and the `fncall1` variant returns 42 too, so both dispatch paths are repaired. ⚠ Its consequence is recorded in `dependency-watch.md`, not just here — this was the *measured* half of the `vec_sort_by` deferral, and it is retired. |
| `archived/2026-08-09-cyrius-distlib-selfcheck-rejects-stdlib-globals.md` | 2026-08-10 | 🟢 **FIXED in 6.5.17.** `cyrius distlib` exits 0 on hisab's real bundle and on all three arms of the minimal reproducer (stdlib function / global var / enum constant). **The CI and release tolerance is removed** — both workflows run a bare `cyrius distlib` again and a non-zero exit is a real failure once more; the history is kept inline in `ci.yml` because the shape recurs. `cyrius check --with-deps` kept independently: it is strictly stronger than the self-check ever was. |
| `archived/2026-08-10-ray-sphere-capsule-assume-unit-direction.md` | 2026-08-10 | 🟢 **FIXED (2.10.0)** — hisab's own, not a toolchain bug, listed here because 2.10.0 turned on it. `geo_ray_sphere` dropped `a = d·d` from both discriminant and division, so a non-unit direction returned a `t` whose hit point was **3.74 from the centre of a unit sphere**, silently; `geo_ray_capsule` inherited it through its end caps only — its own cylinder quadratic was already correct, so one root cause took the homogeneity sweep 5 bad → 0. Cost **+17.6%** on `ray_sphere`, interleaved A/B, accepted. |

**hisab's OWN defect and perf filings also live in `issues/`** — they are not toolchain bugs and are
not counted in the 3 above. A reader counting files in that directory will see more than this tier's
table lists; this is the index for the rest. Disposition is carried in each file's own Status line
and in the CHANGELOG entry that closed it.

| File | Filed | Status |
|---|---|---|
|  `archived/2026-08-05-two-ghost-tie-branch-sign-rule-inconsistent.md` | 2026-08-05 | 🟢 **FIXED 2.9.1** — `_col_dl_ic_g2`'s M² tie branch multiplied by the winding twice, inverting the in-circle answer on every anti-cyclic ghost pair (0 wrong cyclic / 267,358 of 267,358 anti-cyclic, exact-circumcircle oracle). Latent since 2.8.2: `cr` is `+1` on every path `delaunay_2d` takes, so no reachable behaviour changed. 9 new assertions, mutation-proven both ways. The filing's open question (3) — *should this be callable with a CW triangle?* — is answered in a **Resolution** section with measurements, including the evidence that rejected the alternative repair. |
| `archived/2026-08-06-ghost-incircle-g1-g3-assume-ccw.md` | 2026-08-09 | 🟢 **CLOSED (2.9.3)** — fallout from the above. **`_col_dl_ic_g3` fixed**: the all-ghost answer is the constant `1` (re-derived independently, not inherited — equal `\|u_k\|²` put the apex at the ghosts' circumcenter, and the M⁴ coefficient shares its winding factor with the ghost triangle's orientation, so the two cancel), body is `return 1;` with **no arguments**, asserted over both windings and mutation-proved. The file records one correction to its own reasoning: "every finite point is inside a circle through three points at infinity" is too general — unconstrained direction triples answer `0` ~36% of the time. **`g1`'s M¹ tie-break also fixed in 2.9.1** (betweenness). **`g1`'s missing winding factor CLOSED in 2.9.3** — `g1` now forms `sign((b - a) x u_k)`, falling back to `sign((b - a) x (A - a))` when the edge is parallel to `u_k` (that case is NOT degenerate: the triangle keeps constant area as the ghost recedes, and the apex sets the side — the first draft got this wrong and the 2.9.1 reproducer caught it). Validated 4606/4606 against an ORDER-FREE exact oracle, 1110 of those parallel-edge. The Python-transcription evidence is replaced by `_dlg1_run` (1800 probes, pre-fix score 846 of 1500) plus four correctness assertions — order-consistency alone was too weak, a sign-flip mutant survived the grid. **`_col_dl_incircle` is order-free at every arity**; filing archived. |
| `archived/2026-08-04-incircle-precision.md` | 2026-08-09 | 🟢 **FIXED 2.9.3** — and the 2.9.1 "recommended for closure" below was WRONG. `delaunay_2d` silently dropped input points on any set mixing scales (5 of 6 points used, 4 triangles where the exact hull says 6; 8 of the first 20 seeds). The 0-of-1,528,820 sweep that justified closure drew all four fixtures from a single origin-anchored box, so it had no intra-set dynamic range and could not reach this; and `_dl_violations` calls `_col_in_circumcircle` as its own oracle, so the suite was blind by construction. Fixed with an adaptive exact `orient2d` on RAW coordinates (300/300 vs exact rationals; the float winding scores 0/300). ⚠ The obvious fix — exact determinant of **pre-differenced** operands — was implemented first and changed nothing, because the differencing collapses two points into one before any exact arithmetic runs. Prior text retained: | Its CORRECTION section reasoned from a `10 × max(dx, dy)` super-triangle that **2.8.2 deleted**; the 0-of-180 was measured against geometry three minor versions gone. Re-measured over every point quadruple of the suite's own fixtures — a strict superset of `delaunay_2d`'s call set now that ghosts have no coordinates: **0 wrong of 1,528,820**. Predicate is still non-robust *in isolation* (36 of 400 at the original's magnitude range); nothing can hand it that input. |
| `archived/2026-08-04-perf-delaunay_2d.md` | 2026-08-09 | 🟢 **CLOSED (2.9.3) — SUPERSEDED.** It measured the O(n²) per-point full rescan, which the **2.8.0 adjacency rewrite** deleted; insertion is now walk + flood-fill and there is no rescan to remove. Archived rather than deleted for its CHALLENGE section. **Not** the reason `delaunay_2d` is fast, and must not be cited as one. |
| `archived/2026-08-04-perf-kd_partition.md` | 2026-08-09 | 🟢 **CLOSED (2.9.3).** Mis-filed as "NOT APPLIED": the repair shipped in **2.7.1** as a balance guard (the spec's unconditional quickselect measured +114%). 2.9.3 closed both open halves — the guard had **no regression assertion** (`kdtree_build != 0` and the split-plane invariant both hold on a degenerate spine, so deleting it broke nothing checked), now a depth assertion on an **octave-spaced** fixture, mutation-proven; and the BENCH section shipped: `kdtree_build_octave_512` **564.8 µs guarded vs 3.397 ms unguarded, 6.0×**. ⚠ The pre-existing geometric fixture does NOT exercise the guard (ratio 0.99998 → range [0.85, 1]); recorded inline so it is not mistaken for its test. ⚠ The filing's 574 ms headline still has not reproduced and is not restated as fact. Prior text: | The lumped row above was wrong on all three counts for this one: the repair **did** land (2.7.1, as a balance guard rather than the spec's unconditional quickselect), so "NOT APPLIED" was false. What was genuinely missing is that the guard had **no regression assertion** — `kdtree_build != 0` and the split-plane invariant both hold on a degenerate spine, so deleting the guard broke nothing that was checked. Closed in 2.9.3 with a depth assertion on an **octave-spaced** fixture, mutation-proven. ⚠ The pre-existing geometric fixture (ratio 0.99998) does **not** exercise it — its axis range is only [0.85, 1] so the guard never fires; that is recorded inline so the weak fixture is not mistaken for the guard's test. |
| `archived/2026-08-04-perf-triangulate_polygon.md` | 2026-08-09 | 🟢 **CLOSED (2.9.3).** Was mis-filed as "NOT APPLIED": the reflex-only ear prune shipped in **2.7.1**. 2.9.3 closed the rest — the small-n regression is now MEASURED (A/B vs the pre-prune body from `97bed43^`: n=6 **+15%**, n=600 **−82%**, non-overlapping, <2% spread) and disclosed beside the −84% headline; the missing reflex-heavy benchmark shipped (`triangulate_comb_600` **5.718 ms** vs the convex case's **1.593 ms**, 3.6× — the only prior guard was the prune's own BEST case); and `_col_point_in_tri`'s false "(2D, strict interior)" comment is corrected with five assertions. ⚠ Two items carried forward and named in the filing: a backwards divergence-direction claim in the CHANGELOG, and shadowed locals at `collision_core.cyr:740,742`. Prior text: | Also mis-filed as "NOT APPLIED": the reflex-only ear prune shipped in **2.7.1** (`triangulate_600gon` 9.93 → 1.59 ms). Open items are documentary — a small-n regression below break-even that the −84% headline does not disclose, and a missing reflex-heavy benchmark. 2.9.3 fixed the one code-adjacent item: `_col_point_in_tri`'s doc comment claimed "(2D, strict interior)" while the predicate is **boundary-inclusive**, which is what ear clipping actually needs; five assertions now pin the closed contract. |

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
| `benchmarks.md` | 2026-08-03 | ✅ Fresh | **Re-run each release of the arc** via `scripts/bench-history.sh` — now **28** benchmarks (+`convex_hull_2d_2k`, `halfedge_2k_tris`, added *before* the 2.6.15 rewrites so the wins are provable). Both before/after rows are in `bench-history.csv`. |
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
| 1 | **Roadmap + Release History rotate every release** — completed arcs move out of active-work into the Release History table at closeout. | Every release | Per CLAUDE.md Work Loop §10–11. ✅ **Done 2026-08-03 at v2.6.15** — the entire 2.6.12 audit section (32 closed items, 165 lines) retired to a one-paragraph record plus Release-History rows; the roadmap now carries **zero completed items inline** |
| 2 | **Re-run `bench-history.sh` at release closeout** — keep `benchmarks.md` + `bench-history.csv` current; "numbers don't lie." | Every release with perf-relevant change | ✅ Done at every release of the 2.6.12–2.6.15 arc. **28** benchmarks (26 + the two collision hot paths, added *before* the 2.6.15 rewrites so the wins were provable). Both before/after rows are in `bench-history.csv` |
| 3 | **Re-verify tracked `issues/` filings at each toolchain bump** — re-test on the new pin; move resolved ones to `issues/archived/`. | Each `cyrius.cyml` pin bump | ✅ Done 2026-08-03 on 6.5.6 — all 3 open issues still live (interval-ident-lex + for-empty-clauses by minimal repro; CLI-clobber not re-tested, destructive); the 6.4 → 6.5 minor fixed none. Prior: 6.4.69, 6.4.66, 6.3.11, 6.2.11 |
| 4 | **Periodic security audit** — full source scan before a major release or after significant surface change; supersede with a new dated `docs/audit/` doc. | Before 3.0.0; on significant change | ✅ 2026-08-03 → `audit/2026-08-03.md` (70 findings). ✅ **Re-audit done 2026-08-04 → `audit/2026-08-04.md`** (42 confirmed; regression-verification clean). ⏭ 2.7.0 must give **every numbered finding** an explicit disposition — the 2026-08-03 cycle closed on tier summaries and left six findings unscheduled. Also owes the SECURITY.md / threat-model.md memory-safety entries. |
| 5 | **Byte-check `lib/*.cyr` against `~/.cyrius/versions/<pin>/lib/` at every pin bump** — compare the vendored tree to the *pin*, never previous-pin against new-pin. | Each `cyrius.cyml` pin bump | Added at v2.6.11. The toolchain-vs-toolchain comparison used through 2.6.10 is blind to vendoring drift and hid a stale `ganita` 1.0.3 for three releases. `cyrius lib sync`, then `cmp` every file (excluding `sakshi.cyr`) against the pin; any difference is a defect, not a diff |
| 6 | **Run `./scripts/check-constants.sh` after touching any hand-encoded f64 constant** — it decodes every hex literal in `src/` and compares it to the value in its own comment. | Any constant change; runs in CI | **New at v2.6.12.** Seven constant tables shipped not encoding their documented values, including four published tableaux each falsified by its own invariant. 110/110 verified. The class is mechanical, so the guard is too. |
| 7 | **State an audit's SCOPE alongside its verdict** — say what was *not* examined. | Every audit / closeout | **New at v2.6.15, reinforced 2026-08-04.** The 2.5.4 and 2.6.5 closeouts concluded "posture solid" while scoped only to their own arc's new functions; the 2026-08-03 sweep then found 70 defects in the modules they never reached. The 2026-08-03 cycle then made the mirror-image error — declaring itself discharged on the strength of a tier summary rather than its finding list. **State scope, and close on the finding list.** |

---

*Initial scaffold: 2026-05-29 (v2.4.6), adapted from `cyrius/docs/doc-health.md`, immediately after the post-2.4.x documentation sweep. Same-day verify-and-cleanup pass: bench re-run, `tool-issues.md` retired, linalg proposal archived, CONTRIBUTING currency fixed, all 5 toolchain issues re-verified live on 6.0.14. Refresh in place when docs are touched.*
