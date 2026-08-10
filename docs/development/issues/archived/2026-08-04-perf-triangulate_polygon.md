> **CLOSED 2026-08-09 (2.9.3).** The optimisation SHIPPED — in 2.7.1, not "NOT APPLIED" as this
> file's header and `doc-health.md` both said for three releases. `src/collision_core.cyr:612-758` is
> byte-identical to the FIX block below. What was genuinely open was everything around it:
>
> - **CORRECTION 2 (small-n regression) — now measured and disclosed.** A/B against the pre-prune
>   body from `97bed43^`, three runs each: n = 6 goes **2.577 µs → 2.963 µs, +15%**, while n = 600
>   goes **9.556 ms → 1.691 ms, −82%**. Non-overlapping distributions, <2% spread within each. The
>   prune allocates an n-entry `blocked[]` and refreshes neighbour convexity per ear; below
>   break-even that setup cannot amortise. Recorded in the 2.9.3 CHANGELOG next to the −84% headline.
> - **The BENCH section — now shipped.** `triangulate_comb_600` (half the vertices reflex) costs
>   **5.718 ms** against the convex `triangulate_600gon`'s **1.593 ms**, 3.6× on the same n. The only
>   previous guard was the convex case, which is the prune's *best* case — a −84% headline policed by
>   the input it is fastest on. `triangulate_hex_6` covers the small-n class above.
> - **CORRECTION 4 (the false `(2D, strict interior)` comment)** — fixed in 2.9.3; the predicate is
>   boundary-INCLUSIVE, which is what ear clipping needs, so the code was right and only the contract
>   was wrong. Five assertions pin it.
>
> ⚠ **Still open, carried forward and NOT fixed here:** corrections 1 and 4-cosmetic of the CORRECTED
> section — the CHANGELOG's claim that the divergence direction is "old-partial → new-complete" is
> backwards on at least one reproducer, and the shadowed neighbour-refresh locals at
> `collision_core.cyr:740,742` still collide with the winding loop's `pn`/`nn`. Neither changes
> behaviour; both are recorded so closing this file does not bury them.

<!-- 2.7.0-I performance record. Analysed and adversarially reviewed, both agents running
real code. Each optimisation is CONFIRMED as a real complexity win, but each was also shown
to CHANGE RESULTS or to regress on some input class -- see the CHALLENGE section before
applying. NOT APPLIED. The bicgstab hoist, the only result-safe one, shipped in 2.7.0-I. -->

# triangulate_polygon
verdict=PARTIAL

## MEASURED
All numbers from `cyrius bench` on the repo toolchain, median of 3 full runs (run-to-run spread < 5%). Scratch harnesses: /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/{tri_base.bcyr,tri_ab.bcyr,tri_fixture_old.bcyr,tri_fixture_new.bcyr}.

FIXTURE A — convex n-gon (n points on the parabola y = x^2 closed by the top chord; every vertex strictly convex, so NO vertex is ever inside a candidate ear = the worst case for an unpruned sweep):

  n      BEFORE      AFTER      speedup
  250    1.763 ms    0.344 ms   5.1x
  500    7.025 ms    1.187 ms   5.9x
  1000   28.34 ms    4.45 ms    6.4x
  2000   113.2 ms    17.38 ms   6.5x

  growth per doubling BEFORE: 3.98x, 4.03x, 3.99x  -> clean O(n^2)
  growth per doubling AFTER:  3.45x, 3.75x, 3.91x  -> STILL O(n^2)

FIXTURE B — comb polygon (zigzag top chain, ~50% of vertices reflex; adversarial for the prune):

  n      BEFORE      AFTER      speedup
  251    1.876 ms    1.102 ms   1.70x
  501    7.065 ms    4.015 ms   1.76x
  1001   29.35 ms    15.99 ms   1.84x

  growth per doubling BEFORE: 3.77x, 4.15x;  AFTER: 3.64x, 3.98x  -> both O(n^2)

FLOOR PROBE (tri_skel.cyr — deliberately wrong: containment sweep deleted entirely, clips the first convex vertex with no ear validation; exists only to attribute the residue):
  n=1000: 1.158 ms   n=2000: 4.31 ms   -> 3.72x per doubling, itself quadratic.

WHAT THE NUMBERS SETTLE:
1. The 29 ms / 1,000-gon claim is CONFIRMED: measured 28.34 ms (and 113 ms at n=2000 vs the audit's 112 ms).
2. The "should be near O(n)" claim is REFUTED. After the prune the growth ratio is 3.9x per doubling, not 2.1x. The prune is a 6.5x CONSTANT-FACTOR win, not an asymptotic one, and the floor probe shows why: with the containment sweep costing literally zero, n=2000 still takes 4.31 ms and still doubles at 3.7x, because `vec_remove(idx, i)` is an O(remaining) memmove (lib/vec.cyr:174-184) called n-2 times, and the containment loop still has to WALK the whole ring even when every entry is skipped. Reaching near-O(n) needs the vec-backed ring replaced with a doubly-linked list plus a compacted reflex set — a rewrite, not this fix.
3. The audit's "~17x more than necessary on convex input" is REFUTED as a magnitude: measured 6.5x.
4. Attribution at n=2000 convex: 113.2 ms total, 4.3 ms irreducible skeleton, 108.9 ms removable. The prune removes 95.8 ms of that (88%); the 13.1 ms left above the floor is the ring walk itself (vec_get + flag load per j), which the prune cannot skip.

## ROOT CAUSE
The mechanism claim is correct as written. src/collision_core.cyr:610 —

    for (var j = 0; j < remaining; j = j + 1) {
        if (has_point == 0) {
            if (j != prev) { if (j != i) { if (j != next) {
                var ji = vec_get(idx, j);
                var jp = vec_get(vertices, ji);
                var inside = _col_point_in_tri(...);

This inner loop sits inside `for (var i = 0; i < remaining; ...)` (line 575), which sits inside `while (remaining > 2)` (line 570). On convex input the very first candidate (i = 0) is always a valid ear, so exactly ONE containment sweep runs per clip and it runs over all `remaining` vertices: sum over remaining = n..3 of (remaining - 3) ~= n^2/2 calls to `_col_point_in_tri`. At n=2000 that is 2.0M calls, each doing three `_col_triangle_area_2x` evaluations (6 f64_mul, 12 f64_sub) plus six f64 comparisons with NO early exit (lines 520-539 compute d1, d2 and d3 unconditionally). That is the 113 ms.

Only a vertex that is NOT strictly convex can lie inside a candidate ear of a simple polygon (Eberly, "Triangulation by Ear Clipping"; the same prune is what mapbox/earcut's `isEar` does with its `area(p.prev, p, p.next) >= 0` guard). On a convex polygon that count is zero, so every one of those 2.0M tests is provably wasted.

Two secondary wasted-work sites in the same nest, both free to fix and both semantically inert:
- line 575/576: after an ear is clipped the outer `for i` loop keeps iterating to `remaining`, doing nothing because the whole body is wrapped in `if (ear_found == 0)`. There is no `break`.
- line 610/611: same pattern for `has_point` — once a blocker is found the loop still walks the rest of the ring.

Where the finding OVERREACHES: it treats the sweep as the whole cost. It is not. `vec_remove(idx, i)` shifts the tail of the index vec on every clip, so the algorithm carries an irreducible O(n^2) memmove term regardless of the prune (floor probe: 4.31 ms at n=2000). Ear clipping over a vec-backed ring is O(n^2) by construction; the prune shrinks the constant by ~6.5x on convex input, it does not change the exponent.

## FIX
Replace `fn triangulate_polygon` (src/collision_core.cyr:541-644) with the block below, which also inserts one new helper immediately after `_col_point_in_tri` (line 539). Verified: compiles under cycc 6.5.6, `cyrius lint` 0 warnings, max line length 84 (< 120), tests/modules.tcyr 692/692 green with this patch applied.

# Convexity of the ring vertex at position `p`, using the SAME predicate and
# the SAME EPSILON_F64 threshold as the ear gate below, so "not convex here"
# is exactly "would be rejected as an ear tip there".
fn _col_ring_is_convex(vertices, idx, remaining, p, ccw) {
    var pv = p - 1;
    if (pv < 0) { pv = remaining - 1; }
    var nx = p + 1;
    if (nx >= remaining) { nx = 0; }
    var ap = vec_get(vertices, vec_get(idx, pv));
    var bp = vec_get(vertices, vec_get(idx, p));
    var cp = vec_get(vertices, vec_get(idx, nx));
    var cross = _col_triangle_area_2x(HVec2_x(ap), HVec2_y(ap),
    HVec2_x(bp), HVec2_y(bp),
    HVec2_x(cp), HVec2_y(cp));
    if (ccw == 1) {
        if (f64_gt(cross, EPSILON_F64) == 1) { return 1; }
        return 0;
    }
    if (f64_lt(cross, f64_neg(EPSILON_F64)) == 1) { return 1; }
    return 0;
}

fn triangulate_polygon(vertices, n) {
    var result = vec_new();
    if (n < 3) { return result; }

    # Build index list (will shrink as ears are clipped)
    var idx = vec_new();
    for (var i = 0; i < n; i = i + 1) {
        vec_push(idx, i);
    }

    # Determine winding: sum of signed areas
    var total_area = _COL_F64_ZERO;
    for (var i = 0; i < n; i = i + 1) {
        var ni = i + 1;
        if (ni >= n) { ni = 0; }
        var pi = vec_get(vertices, i);
        var pn = vec_get(vertices, ni);
        total_area = f64_add(total_area,
        f64_sub(f64_mul(HVec2_x(pi), HVec2_y(pn)),
        f64_mul(HVec2_x(pn), HVec2_y(pi))));
    }
    # ccw = 1 if total_area > 0 (standard CCW winding)
    var ccw = 1;
    if (f64_lt(total_area, _COL_F64_ZERO) == 1) { ccw = 0; }

    # Blocker cache, keyed by ORIGINAL vertex index (never shifts as idx
    # shrinks). blocked[v] = 1 iff v is NOT strictly convex -- i.e. reflex or
    # collinear. Only such a vertex can sit inside a candidate ear of a simple
    # polygon, so every 0 here is a point-in-triangle test we can skip. Built
    # in O(n); a clip changes the ring only at the two neighbours of the
    # removed vertex, so each clip refreshes exactly 2 entries.
    var blocked = alloc(n * 8);
    if (blocked == 0) { return result; }
    for (var i = 0; i < n; i = i + 1) {
        store64(blocked + i * 8, 1 - _col_ring_is_convex(vertices, idx, n, i, ccw));
    }

    var remaining = n;
    var max_iters = n * n;  # Safety bound
    var iter_count = 0;

    while (remaining > 2) {
        iter_count = iter_count + 1;
        if (iter_count > max_iters) { return result; }  # Degenerate -- bail

        var ear_found = 0;
        for (var i = 0; i < remaining; i = i + 1) {
            var prev = i - 1;
            if (prev < 0) { prev = remaining - 1; }
            var next = i + 1;
            if (next >= remaining) { next = 0; }

            var ai = vec_get(idx, prev);
            var bi = vec_get(idx, i);
            var ci = vec_get(idx, next);

            var ap = vec_get(vertices, ai);
            var bp = vec_get(vertices, bi);
            var cp = vec_get(vertices, ci);

            var ax = HVec2_x(ap);
            var ay = HVec2_y(ap);
            var bx = HVec2_x(bp);
            var by = HVec2_y(bp);
            var cx = HVec2_x(cp);
            var cy = HVec2_y(cp);

            var cross = _col_triangle_area_2x(ax, ay, bx, by, cx, cy);

            # Check convexity: for CCW polygon, ear has positive cross
            var convex = 0;
            if (ccw == 1) {
                if (f64_gt(cross, EPSILON_F64) == 1) { convex = 1; }
            } else {
                if (f64_lt(cross, f64_neg(EPSILON_F64)) == 1) { convex = 1; }
            }

            if (convex == 1) {
                # Check no other BLOCKING vertex inside this triangle
                var has_point = 0;
                for (var j = 0; j < remaining; j = j + 1) {
                    if (j != prev) {
                        if (j != i) {
                            if (j != next) {
                                var ji = vec_get(idx, j);
                                if (load64(blocked + ji * 8) == 1) {
                                    var jp = vec_get(vertices, ji);
                                    var inside = _col_point_in_tri(
                                    HVec2_x(jp), HVec2_y(jp),
                                    ax, ay, bx, by, cx, cy);
                                    if (inside == 1) { has_point = 1; break; }
                                }
                            }
                        }
                    }
                }

                if (has_point == 0) {
                    # Clip this ear
                    vec_push(result, ai);
                    vec_push(result, bi);
                    vec_push(result, ci);
                    vec_remove(idx, i);
                    remaining = remaining - 1;
                    ear_found = 1;
                    # Removing position i leaves the old prev at i-1 and pulls
                    # the old next down into slot i; only those two vertices
                    # changed neighbours, so only those two flags are stale.
                    if (remaining > 2) {
                        var pn = i - 1;
                        if (pn < 0) { pn = remaining - 1; }
                        var nn = i;
                        if (nn >= remaining) { nn = 0; }
                        store64(blocked + vec_get(idx, pn) * 8,
                        1 - _col_ring_is_convex(vertices, idx, remaining, pn, ccw));
                        store64(blocked + vec_get(idx, nn) * 8,
                        1 - _col_ring_is_convex(vertices, idx, remaining, nn, ccw));
                    }
                    break;
                }
            }
        }

        # If no ear found this pass, polygon may be degenerate
        if (ear_found == 0) { return result; }
    }
    return result;
}

Three changes, no more: (1) the `blocked` cache and the `load64(blocked + ji * 8) == 1` gate on the containment test; (2) `break` after clipping an ear and after finding a blocker — both loops already made their remaining iterations no-ops via the `if (ear_found == 0)` / `if (has_point == 0)` body guards, which are now dropped since the break subsumes them; (3) an `if (blocked == 0)` guard matching the 2.6.14 alloc-null tier (src/spatial.cyr:261 idiom).

NOTE: dist/hisab.cyr embeds a copy of collision_core.cyr — `cyrius distlib` must be re-run or the distlib CI gate fails on drift.

## GAIN
Convex / near-convex input: -84% at n=1000 (27.4 ms -> 4.45 ms) and -85% at n=2000 (113 ms -> 17.4 ms), i.e. 6.4-6.5x. Reflex-heavy comb (~50% reflex): -45% at n=1001 (29.3 ms -> 16.0 ms), i.e. 1.8x. No input class regresses — the added work is one O(n) classification pass plus 2 cross products per clip, which is strictly less than the 1 point-in-triangle test per skipped vertex it buys back. Asymptotics are UNCHANGED: 3.9x per doubling after vs 4.0x before, still O(n^2). This is a constant-factor fix and must be described as one in the CHANGELOG — do not claim a complexity improvement.

## RESULT RISK (author)
NOT "none". Checked, and there is one real behaviour change — confined to input that was already outside the algorithm's contract.

SIMPLE POLYGONS: identical output, index-for-index. Verified by differential harnesses that run both versions and compare vec_len plus every element (/tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/{tri_diff.tcyr,tri_rand.tcyr,tri_probe.tcyr}, 19 assertions, all green):
 - 300 random simple polygons, n = 5..40, generated by 2-opt uncrossing and then VERIFIED simple by an O(n^2) proper-crossing count (0 crossings) before comparison — 0 divergences.
 - 60 x-monotone random simple polygons (reflex-rich, crossing count asserted 0) — 0 divergences.
 - convex parabola n = 3..60, both windings; combs m = 1..30, both windings.
 - square with k = 1..12 exactly-collinear vertices per side, both windings.
 - duplicated corner x0..x7; 24 fully coincident vertices (no ear exists — both take the `ear_found == 0` bail and return the same partial vec).
 - sliver vertices displaced by 1e-6 down to 1e-16 so the cross product straddles EPSILON_F64, both windings.
 - all 7 shipped tests/modules.tcyr cases (convex quad, concave 5-gon, hexagon, U-shape, CW quad, collinear edge vertex, n<3).
 - tests/modules.tcyr run end-to-end with the patched module substituted: 692/692, same as the unpatched baseline.

SELF-INTERSECTING POLYGONS: results DO change. The rectilinear-spiral family diverges in 14 of 14 cases (7 spirals x 2 windings). tri_probe.tcyr measures 1..7 properly-crossing edge pairs in each of those, i.e. every diverging case is non-simple — the reflex-only theorem is only valid for simple polygons, so the prune is not sound there. Both versions still return exactly 3*(n-2) indices; they pick different, equally arbitrary overlapping triangles of a polygon that has no valid triangulation. Neither answer is "right", but it is a visible behavioural change: any consumer that snapshots the index list for a self-intersecting input will see a diff. This is the honest cost of the fix and belongs in the CHANGELOG.

COLLINEAR / TIE HANDLING — deliberately preserved. `_col_ring_is_convex` uses the SAME predicate and the SAME EPSILON_F64 threshold as the ear gate, so "blocked" is exactly the complement of "would be accepted as an ear tip". Collinear vertices (cross == 0) are therefore classified as blockers and are still containment-tested. This is what keeps the modules.tcyr `tp_l` collinear-edge case byte-identical: `_col_point_in_tri` reports boundary points as inside (its "strict interior" comment is wrong — a point with one d == 0 and the others same-signed returns 1), so a collinear vertex on the ear boundary still vetoes the ear exactly as before. Classifying collinear as convex-and-skippable WOULD have changed that case; it was checked and rejected.

NO FLOATING-POINT CHANGE. The prune only elides calls to `_col_point_in_tri`; every cross product still evaluated is the identical expression with identical operand order. No new tolerance, no reassociation, no NaN-path change.

THE TWO `break`s ARE INERT. In the current code, once `ear_found = 1` the outer loop's whole body is skipped by `if (ear_found == 0)`, and once `has_point = 1` the inner body is skipped by `if (has_point == 0)`. The remaining iterations were already no-ops, so breaking out is observationally identical (and the `iter_count`/`max_iters` bail is on the WHILE loop, untouched).

MEMORY. One extra `alloc(n * 8)` per call from the bump arena, never freed. At n = 2000 that is 16 KB on top of the ~164 KB the call's two vecs already burn (idx grows to cap 2048 and result to cap 8192, and vec doubling abandons every old buffer) — about +10%. It is a per-CALL allocation, not per-iteration, so it does not create the hot-loop-leak defect class. The `if (blocked == 0) { return result; }` guard means an arena exhaustion now returns an empty vec instead of storing through a null base.

UNCHANGED DEFECTS. The doc comment at src/collision_core.cyr:511 ("length = 3*(n-2)") is still wrong on the `ear_found == 0` bail path — that is the separate LOW api-consistency finding and this patch neither fixes nor worsens it.

## BENCH
Add to tests/hisab.bcyr. No new `include` lines needed — src/vec2.cyr and src/collision_core.cyr are already in the file's include list. Verified end-to-end: 27.4 ms / 27.8 ms before, 4.45 ms / 16.1 ms after; total wall time of both benches post-fix is ~125 ms.

# --- with the other fixture globals near the top of the file -----------
var _b_tri_convex = 0;
var _b_tri_comb = 0;
var _b_tri_comb_n = 0;

# --- with the other collision benchmark fns ----------------------------

# Worst-case ear-clip input: n points on the parabola y = x^2, closed by the
# top chord. Parabola points are in strictly convex position, so NO vertex is
# ever inside a candidate ear and the containment sweep never terminates
# early -- the case the 2026-08-04 audit measured at 29 ms for n = 1,000.
fn _b_build_convex_ngon(n) {
    var v = vec_new();
    var i = 0;
    while (i < n) {
        var x = f64_from(i);
        vec_push(v, hvec2_new(x, f64_mul(x, x)));
        i = i + 1;
    }
    return v;
}

# Adversarial input for the convexity prune: a comb whose top chain zigzags
# between y = 2 and y = 3, so roughly half the vertices are reflex and only
# half the containment tests can be skipped. Guards against "optimised the
# convex case, regressed the concave one". n = 2*m + 3.
fn _b_build_comb(m) {
    var v = vec_new();
    vec_push(v, hvec2_new(0, 0));
    vec_push(v, hvec2_new(f64_from(2 * m), 0));
    var i = 2 * m;
    while (i >= 0) {
        var y = 2;
        if (i - (i / 2) * 2 == 1) { y = 3; }
        vec_push(v, hvec2_new(f64_from(i), f64_from(y)));
        i = i - 1;
    }
    return v;
}

fn bench_triangulate_convex() {
    var t = triangulate_polygon(_b_tri_convex, 1000);
    return 0;
}

fn bench_triangulate_comb() {
    var t = triangulate_polygon(_b_tri_comb, _b_tri_comb_n);
    return 0;
}

# --- in main(), with the other fixture setup ---------------------------
    _b_tri_convex = _b_build_convex_ngon(1000);
    _b_tri_comb = _b_build_comb(499);
    _b_tri_comb_n = vec_len(_b_tri_comb);

# --- in main(), in the "Collision hot paths" bench block ---------------
    # Ear clipping. Two shapes, because the convexity prune helps them by very
    # different factors: all-convex is the best case (6.4x), a half-reflex comb
    # is the worst (1.8x). Iteration counts are low because each call is
    # milliseconds; both stay O(n^2), so do not raise n without re-timing.
    bench("triangulate_convex_1k", &bench_triangulate_convex, 10);
    bench("triangulate_comb_1k",   &bench_triangulate_comb,    5);

Expected bench-history.csv rows:
  triangulate_convex_1k: 27.4 ms -> 4.45 ms (-84%)
  triangulate_comb_1k:   27.8 ms -> 16.1 ms (-42%)

## CHALLENGE changes_results=True compiles=True repro=True stands=True
- METHOD NOTE (how I checked): I built a bit-exact Python port of both versions (every op is +,-,* on IEEE doubles, so the port is exact) and validated it against the real toolchain -- /tmp/claude-1000/-home-macro-Repos-hisab/02f8a39a-a230-4034-83fe-7d1f605e314f/scratchpad/adv/{port.py,classify.py} vs a Cyrius dump harness. Real cycc 6.5.6 reproduced the port's output index-for-index on all 7 probe cases, including all 4 divergent ones. All divergence claims below were then re-confirmed by running the ACTUAL patched src/collision_core.cyr (scratchpad/rev_26154), not just my model.
- SOUNDNESS ON SIMPLE POLYGONS: CONFIRMED, and far more strongly than they showed. ~33,000 simple/collinear polygons, 0 divergences. That includes an EXHAUSTIVE enumeration of every 4-, 5- and 6-gon on a 3x3 integer grid (597,051 vertex tuples enumerated; 6,848 classified simple or collinear) -> 0 divergences; 1,153 uncrossed random polygons replayed at six scales 1e-7 .. 1e8 to probe the absolute EPSILON_F64=1e-12 threshold -> 0; 600 guaranteed-simple star / x-monotone polygons at n = 50..500 -> 0 (this specifically rules out the incremental `blocked` cache going stale over many clips, which was my main structural suspicion); 3,829 collinear edge-splits -> 0; 3,829 adjacent duplicate vertices (zero-length edges) -> 0. The reflex-only prune, the 2-neighbour cache refresh (I hand-verified the ring-position arithmetic for i=0, i=R-1 and the interior case), and the two `break`s are all correct.
- ERROR 1 -- THE DIVERGING CLASS IS MIS-SCOPED. They wrote 'SELF-INTERSECTING POLYGONS: results DO change' and justified it with 'tri_probe.tcyr measures 1..7 properly-crossing edge pairs in each of those, i.e. every diverging case is non-simple', and they verified their 300 random polygons 'simple by an O(n^2) proper-crossing count (0 crossings)'. A proper-crossing count cannot detect a polygon that self-TOUCHES or pinches. Self-touching polygons with ZERO proper crossings diverge 45% of the time (29,330 of 65,294 sampled). Taking a verified-simple polygon and making it revisit one point elsewhere in the ring ('pinch') diverges 60.7% of the time (2,325 of 3,829). Every one of those scores 0 crossings on their probe and would have been counted as 'simple'. Minimal reproducer, ALL FIVE VERTICES DISTINCT, zero proper crossings, confirmed on real cycc 6.5.6: (0,0),(0,2),(1,1),(1,0),(2,2) -> old [0,1,2, 0,2,3], new [4,0,1, 1,2,3]. Their conclusion ('non-simple input only') is still true, but the evidence they offered for it does not establish it, and the CHANGELOG wording they propose would tell consumers the wrong thing to look for.
- ERROR 2 -- 'Both versions still return exactly 3*(n-2) indices' IS FALSE. Output LENGTH changes, in both directions, and length change is the DOMINANT divergence mode (13,914 of 14,838 divergences in one 60k-trial run). old EMPTY -> new COMPLETE: (0,0),(0,2),(1,1),(0,2),(2,1) gives old = [] (0 indices), new = 9 indices. old COMPLETE -> new PARTIAL, the damaging direction: (0,0),(1,2),(1,1),(0,2),(2,1) gives old = [0,1,2, 4,0,2, 4,2,3] (the full 3*(n-2)), new = [4,0,1, 1,2,3] (truncated). Both confirmed on real cycc 6.5.6. Rates over 120k random 4..11-gons: touch class 6 oldFULL->newPARTIAL and 514 oldPART->newFULL of 65,294; crossing class 801 and 55 of 44,902. This is materially worse than 'they pick different, equally arbitrary overlapping triangles' -- a consumer can go from a complete triangulation to a truncated one on unchanged input, and src/collision_core.cyr:511 still documents 'length = 3*(n-2)'.
- ERROR 3 -- THE WIN IS NEGATIVE AT REALISTIC SIZES, AND THEY NEVER MEASURED THERE. Their smallest fixture is n=250. I A/B'd both versions in one binary, 60,000 iterations each, two independent passes that agree to ~1%: n=4 2.05->2.39us (-17%), n=5 2.31->2.69us (-17%), n=6 2.60->3.02us (-16%), n=8 3.52->3.87us (-10%), n=10 4.62->4.68us (break-even), n=12 5.80->5.41us (+7%), n=16 (+25%), n=32 (+97%), n=48 (+148%). Break-even is n ~= 10-11; below it the O(n) cache build plus two _col_ring_is_convex refreshes per clip cost more than the skipped point-in-triangle tests save. The repo's own triangulate_polygon tests are n=4, 5, 5, 6, 4, 5 -- every one sits in the regression band, as do typical physics/rendering polygons for the listed consumers (impetus, kiran, joshua, aethersafha). The proposal should state this, not just the 6.5x headline.
- MINOR -- the patch declares `var pn` twice inside `triangulate_polygon`: once as the winding-sum point (`var pn = vec_get(vertices, ni);`) and again as the prev-neighbour ring position (`var pn = i - 1;`). It compiles and is harmless (disjoint code paths, and I confirmed the real patched source is output-identical to a renamed model on all 7 probe cases), but reusing a name for a point pointer and then for a loop index in one 100-line function is exactly the kind of thing the next reader misparses. Rename the second to `pn_pos`/`nn_pos`.
- CONFIRMED AS STATED, no correction needed: the 29 ms / 1,000-gon claim (I measure 27.5 ms; 110.6 ms at n=2000); the refutation of 'should be near O(n)' (my AFTER growth per doubling is 3.45x, 3.81x, 3.82x -- still cleanly quadratic, so the 6.5x is a constant factor and vec_remove's memmove plus the ring walk are the residue); the refutation of the audit's '~17x' magnitude (I measure 6.5x at n=2000, 5.2x at n=250); the mechanism and line references; the claim that the two `break`s are observationally inert (the guarded bodies are pure no-ops -- and my Cyrius harness confirms the guard-based repo version and the break-based version agree even on all four DIVERGENT inputs); the collinear/tie preservation via the shared EPSILON_F64 predicate; 'no floating-point change'; and the distlib regeneration note (dist/hisab.cyr does bundle this function).
- CLEANLINESS GATES ALL PASS on the patched copy: `cyrius lint src/collision_core.cyr` 0 warnings, `cyrius fmt --check` clean, `cyrius vet src/main.cyr` OK, max line 84 chars. All four suites green and byte-identical to an unpatched baseline copy: hisab 306/306, foundation 310/310, modules 692/692, edge_cases 215/215 (1523 total, same both ways). `_col_ring_is_convex` collides with nothing; no reserved SIMD identifiers used; `alloc(n*8)` cannot reach ALLOC_MAX (2 GiB, i.e. n > 268M). I wrote nothing to /home/macro/Repos/hisab -- `git diff --stat src/collision_core.cyr` is empty (the two modified docs/ files predate my work and are not mine).

### CORRECTED
SHIP THE CODE AS WRITTEN -- the algorithm is correct and I could not break it on any in-contract input. Three corrections, none to the hot loop:

1. FIX THE RISK DISCLOSURE (required -- this is what goes in the CHANGELOG). Replace "self-intersecting polygons" with "polygons that are not simple, INCLUDING ones that merely self-touch or pinch with no edge crossing at all", and delete "Both versions still return exactly 3*(n-2) indices" -- it is false. Correct wording:

  ### Changed
  - **collision_core** — `triangulate_polygon` now tests only non-convex (reflex or
    collinear) vertices for point-in-triangle containment when validating an ear.
    On a simple polygon this is provably equivalent (only a non-convex vertex can
    lie inside a candidate ear) and output is index-for-index identical — verified
    against ~33,000 simple polygons, including an exhaustive enumeration of every
    4-, 5- and 6-gon on a 3x3 integer grid.
    **Breaking for non-simple input.** Ear clipping never had a defined result for a
    polygon that self-intersects, self-touches or repeats a vertex, and for those
    inputs both the chosen triangles AND the number of triangles returned may now
    differ. A call that previously returned the full 3*(n-2) indices may now return
    fewer, and vice versa. Example: (0,0),(1,2),(1,1),(0,2),(2,1) returned 9 indices
    and now returns 6. Note that a self-touching polygon has zero edge crossings, so
    a crossing test is NOT sufficient to tell whether your input is affected.

2. ADD A REGRESSION TEST FOR THE SMALL-n COST, or gate it. The patch is 16-17% SLOWER for n <= 8 and does not break even until n ~= 10-11, which is where the repo's own tests and most consumer polygons live. Either state that explicitly in the CHANGELOG next to the 6.5x, or restore the old path for small inputs with a sentinel (keeps one predictable branch, no duplicated loop):

    var blocked = 0;
    if (n >= 12) {
        blocked = alloc(n * 8);
        if (blocked == 0) { return result; }
        for (var i = 0; i < n; i = i + 1) {
            store64(blocked + i * 8, 1 - _col_ring_is_convex(vertices, idx, n, i, ccw));
        }
    }
    ...
    var skip = 0;
    if (blocked != 0) { if (load64(blocked + ji * 8) == 0) { skip = 1; } }
    if (skip == 0) { ...existing point-in-tri test... }
    ...
    if (blocked != 0) { if (remaining > 2) { ...existing 2-entry refresh... } }

  Trade-off to decide deliberately: this makes the degenerate-input divergence
  n-dependent (n<12 keeps the old answers, n>=12 does not), which is arguably worse
  for reproducibility than one uniform rule. If you prefer uniform behaviour, drop
  the gate and just document the sub-12 cost.

3. FIX src/collision_core.cyr:511 IN THIS CHANGE, not later. The comment promises
   "length = 3*(n-2)". This patch adds new ways for that to be wrong, so deferring it
   to the separate LOW api-consistency finding is no longer appropriate — it is now
   part of this change's contract:
     # Returns: vec of index triples [i0, i1, i2, ...] (flat). Length is 3*(n-2) for
     # a simple polygon; a degenerate or non-simple polygon bails early and returns a
     # SHORTER partial result, so callers must read vec_len rather than assume 3*(n-2).

4. Rename the shadowed `var pn` in the neighbour-refresh block to `pn_pos` (and `nn` to
   `nn_pos`) so it does not collide visually with the winding loop's `var pn` point.

Do NOT try to preserve old output on non-simple input — it is not achievable with this
prune (the minimal counterexample has all-distinct vertices and zero crossings), and
detecting non-simplicity is the O(n^2) cost the optimisation exists to remove.